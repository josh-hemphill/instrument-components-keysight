using System.Collections.Frozen;
using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Drivers.Keysight;

internal sealed record MessageStep(string Message, Func<string, CatalogReply?>? Decode = null);
internal interface IMessageEngine
{
    IReadOnlySet<Operation> Operations { get; }
    IReadOnlyList<MessageStep> Prepare(CatalogRequest request);
    void Begin(CatalogRequest request) { }
    void Commit(CatalogRequest request) { }
}
internal static class NativeMessages
{
    internal static DriverException Error(DriverErrorKind kind, string message) => new(kind, message);
    internal static (string, string) Key(Endpoint endpoint) => (string.Concat(endpoint.Scheme.Trim().Select(c => c is >= 'A' and <= 'Z' ? (char)(c + 32) : c)), endpoint.Address);
    internal static void ValidateEndpoint(Endpoint endpoint)
    {
        if (!endpoint.Scheme.Trim().Equals("hpib", StringComparison.OrdinalIgnoreCase) || string.IsNullOrWhiteSpace(endpoint.Address))
            throw Error(DriverErrorKind.InvalidArgument, "Nonempty hpib endpoint required");
    }
    internal static void ValidateOpen(DriverOpenContext context, IReadOnlySet<Operation> supported)
    {
        ValidateEndpoint(context.Candidate.Endpoint);
        if (!Enum.IsDefined(context.Ownership)) throw Error(DriverErrorKind.InvalidArgument, "Unknown ownership");
        if (context.RequiredOperations is { } required && !required.IsSubsetOf(supported))
            throw Error(DriverErrorKind.UnsupportedOperation, "Required operation unavailable on selected endpoint");
    }
    internal static void CheckBody(string body)
    {
        if (body.Length is 0 or > 256 || body.Any(c => !char.IsAscii(c) || char.IsControl(c)))
            throw Error(DriverErrorKind.Parse, "Expected bounded complete ASCII scalar body without control characters");
    }
    internal static DriverDescriptor Descriptor(string id, string model, IReadOnlySet<Operation> supported) => new(id, DriverContract.Major,
        new[] { "HP", "Hewlett-Packard" }.Select(m => new ExactIdentityRule("hpib", m, model)), supported);
    internal static IReadOnlySet<ExecutionMode> Modes(bool native) => (native
        ? new[] { ExecutionMode.Sync, ExecutionMode.WorkerAsync, ExecutionMode.NativeAsync }
        : new[] { ExecutionMode.Sync, ExecutionMode.WorkerAsync }).ToFrozenSet();
}
internal sealed class MessageDevice(IMessageEngine engine, IMessageIo io, ConnectionOwnership ownership) : IDeviceBackend, ICatalogBackend
{
    private bool _closed, _healthy = true;
    public IReadOnlySet<Operation> Operations => engine.Operations;
    public ICatalogBackend Catalog => this;
    public CatalogReply Invoke(CatalogRequest request)
    {
        if (_closed) throw NativeMessages.Error(DriverErrorKind.SessionClosed, "Resource closed");
        if (!_healthy) throw NativeMessages.Error(DriverErrorKind.SessionNeedsRecovery, "External resynchronization and reopen required");
        request.Validate();
        var steps = engine.Prepare(request);
        engine.Begin(request);
        CatalogReply reply = new CatalogReply.Empty();
        foreach (var step in steps)
        {
            _healthy = false;
            if (step.Decode is null) io.Write(step.Message);
            else
            {
                var body = io.Query(step.Message);
                NativeMessages.CheckBody(body);
                reply = step.Decode(body) ?? reply;
            }
        }
        engine.Commit(request);
        _healthy = true;
        return reply;
    }
    public void Dispose() { if (_closed) return; _closed = true; if (ownership == ConnectionOwnership.Owned) io.Dispose(); }
}
internal sealed class AsyncMessageDevice(IMessageEngine engine, IAsyncMessageIo io, ConnectionOwnership ownership) : IAsyncDeviceBackend, IAsyncCatalogBackend
{
    private bool _closed, _healthy = true;
    public IReadOnlySet<Operation> Operations => engine.Operations;
    public IAsyncCatalogBackend Catalog => this;
    public async ValueTask<CatalogReply> InvokeAsync(CatalogRequest request, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        if (_closed) throw NativeMessages.Error(DriverErrorKind.SessionClosed, "Resource closed");
        if (!_healthy) throw NativeMessages.Error(DriverErrorKind.SessionNeedsRecovery, "External resynchronization and reopen required");
        request.Validate();
        var steps = engine.Prepare(request);
        engine.Begin(request);
        CatalogReply reply = new CatalogReply.Empty();
        foreach (var step in steps)
        {
            _healthy = false;
            if (step.Decode is null) await io.WriteAsync(step.Message, cancellationToken).ConfigureAwait(false);
            else
            {
                var body = await io.QueryAsync(step.Message, cancellationToken).ConfigureAwait(false);
                NativeMessages.CheckBody(body);
                reply = step.Decode(body) ?? reply;
            }
        }
        engine.Commit(request);
        _healthy = true;
        return reply;
    }
    public void Dispose() { if (_closed) return; _closed = true; if (ownership == ConnectionOwnership.Owned) io.Dispose(); }
}
