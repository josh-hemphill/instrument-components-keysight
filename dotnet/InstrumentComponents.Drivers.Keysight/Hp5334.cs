using System.Collections.Frozen;
using System.Globalization;
using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Drivers.Keysight;

public enum Hp5334Model { Hp5334A, Hp5334B }
/// <summary>Application commissioning evidence, never inferred through probing.</summary>
public enum Hp5334Commissioning { Unknown, Ciil, NativeInternalGatingConfirmed }

/// <summary>Native HP counter factory. Open sends no commands. See spec/legacy/HP5334A.md and HP5334B.md.</summary>
public sealed class Hp5334Factory : IDriverFactory
{
    private readonly Hp5334Model _model;
    private readonly IScpiConnector _connector;
    private readonly IAsyncScpiConnector? _native;
    internal static readonly FrozenSet<Operation> Supported = new[] {
        Operation.CatalogCounterSelectChannel, Operation.CatalogCounterMeasureFrequency,
        Operation.CatalogCounterMeasurePeriod, Operation.CatalogCounterSetGateTime,
        Operation.CatalogReset, Operation.Reset
    }.ToFrozenSet();

    public Hp5334Factory(Hp5334Model model, Hp5334Commissioning commissioning,
        IScpiConnector connector, IAsyncScpiConnector? nativeConnector = null)
    {
        if (!Enum.IsDefined(model) || commissioning != Hp5334Commissioning.NativeInternalGatingConfirmed)
            throw Invalid("Confirm native language, internal gating and exclusive REMOTE commissioning before registration");
        ArgumentNullException.ThrowIfNull(connector);
        _model = model;
        _connector = connector;
        _native = nativeConnector;
    }

    public string Model => _model == Hp5334Model.Hp5334A ? "5334A" : "5334B";
    public string DriverId => _model == Hp5334Model.Hp5334A ? "keysight.hp5334a" : "keysight.hp5334b";
    public DriverDescriptor Descriptor => new(DriverId, DriverContract.Major,
        new[] { "HP", "Hewlett-Packard" }.Select(m => new ExactIdentityRule("hpib", m, Model)), Supported);
    public IReadOnlySet<ExecutionMode> ExecutionModes => (_native is null
        ? new[] { ExecutionMode.Sync, ExecutionMode.WorkerAsync }
        : new[] { ExecutionMode.Sync, ExecutionMode.WorkerAsync, ExecutionMode.NativeAsync }).ToFrozenSet();

    private static DriverException Invalid(string message) => new(DriverErrorKind.InvalidArgument, message);
    private static void Validate(DriverOpenContext context)
    {
        if (!context.Candidate.Endpoint.Scheme.Trim().Equals("hpib", StringComparison.OrdinalIgnoreCase)
            || string.IsNullOrWhiteSpace(context.Candidate.Endpoint.Address))
            throw Invalid("HP counter requires a nonempty hpib endpoint");
        if (context.RequiredOperations is { } required && !required.IsSubsetOf(Supported))
            throw new DriverException(DriverErrorKind.UnsupportedOperation, "Required operation unavailable for HP counter");
        if (!Enum.IsDefined(context.Ownership)) throw Invalid("Unknown connection ownership");
    }

    public IDeviceBackend Open(DriverOpenContext context)
    {
        Validate(context);
        return new Device(_connector.Open(context), context.Ownership);
    }

    public async ValueTask<IAsyncDeviceBackend> OpenNativeAsync(DriverOpenContext context, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        Validate(context);
        if (_native is null) throw new DriverException(DriverErrorKind.UnsupportedExecutionMode, "Native connector not configured");
        var io = await _native.OpenAsync(context, cancellationToken).ConfigureAwait(false);
        return new AsyncDevice(io, context.Ownership);
    }

    private enum ActionKind { Local, Write, Query }
    private enum UpdateKind { None, Channel, Gate, Reset }
    private sealed record Prepared(ActionKind Action, string Message = "", Unit Unit = Unit.Unknown,
        UpdateKind Update = UpdateKind.None, uint Channel = 0, double Gate = 0);

    private sealed class Core
    {
        private uint _channel = 1;
        private double? _gate;
        public bool Healthy { get; set; } = true;
        private static string GateText(double value) => value.ToString("0.00000000000E+00", CultureInfo.InvariantCulture);

        public Prepared Prepare(CatalogRequest request)
        {
            request.Validate();
            if (!Healthy) throw new DriverException(DriverErrorKind.SessionNeedsRecovery, "HP counter requires external resynchronization and reopen");
            switch (request)
            {
                case CounterSelectChannelRequest select:
                    if (select.Channel is not (1 or 2)) throw Invalid("Only channels 1=A and 2=B are configured");
                    return new(ActionKind.Local, Update: UpdateKind.Channel, Channel: select.Channel);
                case CounterSetGateTimeRequest gate:
                    if (gate.Seconds is < 0.001 or > 99.999) throw Invalid("Gate time must be 0.001 through 99.999 seconds");
                    return new(ActionKind.Write, "GA" + GateText(gate.Seconds), Update: UpdateKind.Gate, Gate: gate.Seconds);
                case CounterMeasureFrequencyRequest:
                case CounterMeasurePeriodRequest:
                    var period = request is CounterMeasurePeriodRequest;
                    if (period && _channel != 1) throw new DriverException(DriverErrorKind.UnsupportedOperation, "Period is supported on channel A only");
                    var function = period ? 4 : _channel;
                    var explicitGate = _gate is { } value ? "GA" + GateText(value) + "," : "";
                    return new(ActionKind.Query, $"FN{function},HS0,MD1,WA1,{explicitGate}RE", period ? Unit.Second : Unit.Hertz);
                case ResetRequest:
                    return new(ActionKind.Write, "IN", Update: UpdateKind.Reset);
                default:
                    throw new DriverException(DriverErrorKind.UnsupportedOperation, "Operation not implemented by HP counter");
            }
        }

        public CatalogReply Finish(Prepared prepared, string? response)
        {
            CatalogReply result = prepared.Action == ActionKind.Query
                ? new CatalogReply.Numeric(Parse(response ?? "", prepared.Unit)) : new CatalogReply.Empty();
            switch (prepared.Update)
            {
                case UpdateKind.Channel: _channel = prepared.Channel; break;
                case UpdateKind.Gate: _gate = prepared.Gate; break;
                case UpdateKind.Reset: _channel = 1; _gate = null; break;
            }
            Healthy = true;
            return result;
        }
    }

    private static Quantity Parse(string body, Unit unit)
    {
        static DriverException Bad() => new(DriverErrorKind.Parse, "Invalid HP5334 native ASCII measurement record");
        if (body == "O+9.99999999999E+99") return new(0, unit, MeasurementStatus.Overload);
        var expected = unit == Unit.Hertz ? 'F' : 'S';
        if (body.Length is < 2 or > 19 || body[0] != expected || body.Any(c => !char.IsAscii(c) || char.IsControl(c))) throw Bad();
        var field = body.AsSpan(1);
        var number = field.TrimStart(' ');
        if (field[0] == ' ' && number.SequenceEqual("0E+00")) return new(0, unit);
        if (body.Length != 19 || number.Length < 6 || number[0] is not ('+' or '-') || !char.IsAsciiDigit(number[1])) throw Bad();
        var exponent = number.Length - 4;
        if (number[exponent] != 'E' || number[exponent + 1] is not ('+' or '-')
            || !Digits(number[(exponent + 2)..])) throw Bad();
        if (exponent != 2 && (exponent < 4 || number[2] != '.' || !Digits(number[3..exponent]))) throw Bad();
        if (!double.TryParse(number, NumberStyles.AllowLeadingSign | NumberStyles.AllowDecimalPoint | NumberStyles.AllowExponent,
            CultureInfo.InvariantCulture, out var value) || !double.IsFinite(value)) throw Bad();
        return new(value, unit);
    }

    private static bool Digits(ReadOnlySpan<char> value)
    {
        foreach (var c in value) if (!char.IsAsciiDigit(c)) return false;
        return true;
    }

    private sealed class Device(IMessageIo io, ConnectionOwnership ownership) : IDeviceBackend, ICatalogBackend, IDeviceReset
    {
        private readonly Core _core = new();
        private bool _closed;
        public IReadOnlySet<Operation> Operations => Supported;
        public ICatalogBackend Catalog => this;
        public IDeviceReset Reset => this;
        public CatalogReply Invoke(CatalogRequest request)
        {
            if (_closed) throw new DriverException(DriverErrorKind.SessionClosed, "HP counter resource closed");
            var prepared = _core.Prepare(request);
            string? response = null;
            if (prepared.Action != ActionKind.Local)
            {
                _core.Healthy = false;
                if (prepared.Action == ActionKind.Write) io.Write(prepared.Message);
                else response = io.Query(prepared.Message);
            }
            return _core.Finish(prepared, response);
        }
        void IDeviceReset.Reset() => Invoke(new ResetRequest()).IntoUnit();
        public void Dispose() { if (_closed) return; _closed = true; if (ownership == ConnectionOwnership.Owned) io.Dispose(); }
    }

    private sealed class AsyncDevice(IAsyncMessageIo io, ConnectionOwnership ownership) : IAsyncDeviceBackend, IAsyncCatalogBackend, IAsyncReset
    {
        private readonly Core _core = new();
        private bool _closed;
        public IReadOnlySet<Operation> Operations => Supported;
        public IAsyncCatalogBackend Catalog => this;
        public IAsyncReset Reset => this;
        public async ValueTask<CatalogReply> InvokeAsync(CatalogRequest request, CancellationToken cancellationToken = default)
        {
            cancellationToken.ThrowIfCancellationRequested();
            if (_closed) throw new DriverException(DriverErrorKind.SessionClosed, "HP counter resource closed");
            var prepared = _core.Prepare(request);
            string? response = null;
            if (prepared.Action != ActionKind.Local)
            {
                _core.Healthy = false;
                if (prepared.Action == ActionKind.Write) await io.WriteAsync(prepared.Message, cancellationToken).ConfigureAwait(false);
                else response = await io.QueryAsync(prepared.Message, cancellationToken).ConfigureAwait(false);
            }
            return _core.Finish(prepared, response);
        }
        async ValueTask IAsyncReset.ResetAsync(CancellationToken cancellationToken) => (await InvokeAsync(new ResetRequest(), cancellationToken).ConfigureAwait(false)).IntoUnit();
        public void Dispose() { if (_closed) return; _closed = true; if (ownership == ConnectionOwnership.Owned) io.Dispose(); }
    }
}
