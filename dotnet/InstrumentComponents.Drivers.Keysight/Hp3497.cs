using System.Collections.Frozen;
using System.Globalization;
using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Drivers.Keysight;

public enum Hp3497Module { Unknown, Option010, Option020 }
public enum Hp3497Compensation { Unknown, None, Software, Hardware }
public sealed record Hp3497Input(uint Channel, byte Slot, byte Local, Hp3497Module Module, Hp3497Compensation Compensation);
public sealed record Hp3497Configuration(bool Option001, bool NormalDvmRoutingConfirmed, IReadOnlyList<Hp3497Input> Inputs);
public sealed record Hp3497Binding(Endpoint Endpoint, Hp3497Configuration Configuration);

/// <summary>Explicit HP3497A Option001 inventory. No probing; see spec/legacy/HP3497A.md.</summary>
public sealed class Hp3497Factory : IDriverFactory
{
    private static readonly FrozenSet<Operation> Supported = new[] { Operation.CatalogDataAcquisitionMeasureVoltageDc }.ToFrozenSet();
    private readonly FrozenDictionary<(string, string), FrozenDictionary<uint, byte>> _routes;
    private readonly IScpiConnector _connector;
    private readonly IAsyncScpiConnector? _native;
    public Hp3497Factory(IEnumerable<Hp3497Binding> bindings, IScpiConnector connector, IAsyncScpiConnector? nativeConnector = null)
    {
        ArgumentNullException.ThrowIfNull(bindings); ArgumentNullException.ThrowIfNull(connector);
        var routes = new Dictionary<(string, string), FrozenDictionary<uint, byte>>();
        foreach (var binding in bindings)
        {
            NativeMessages.ValidateEndpoint(binding.Endpoint);
            if (routes.Count == 4096 || !routes.TryAdd(NativeMessages.Key(binding.Endpoint), Freeze(binding.Configuration))) throw Invalid();
        }
        if (routes.Count == 0) throw Invalid();
        _routes = routes.ToFrozenDictionary(); _connector = connector; _native = nativeConnector;
    }
    private static DriverException Invalid() => NativeMessages.Error(DriverErrorKind.InvalidArgument, "Require explicit Option001 normal-DVM module inventory and unique endpoints/channels");
    private static FrozenDictionary<uint, byte> Freeze(Hp3497Configuration configuration)
    {
        if (!configuration.Option001 || !configuration.NormalDvmRoutingConfirmed || configuration.Inputs.Count is 0 or > 100) throw Invalid();
        var inputs = new Dictionary<uint, byte>(); var native = new HashSet<byte>(); var slots = new Dictionary<byte, (Hp3497Module, Hp3497Compensation)>();
        foreach (var input in configuration.Inputs)
        {
            if (input.Channel == 0 || input.Slot > 4 || (input.Module, input.Compensation, input.Local) is not
                ((Hp3497Module.Option010, Hp3497Compensation.None, <= 19) or (Hp3497Module.Option020, Hp3497Compensation.Software, <= 18))) throw Invalid();
            var number = (byte)(20 * input.Slot + input.Local);
            if (!inputs.TryAdd(input.Channel, number) || !native.Add(number)) throw Invalid();
            if (slots.TryGetValue(input.Slot, out var previous) && previous != (input.Module, input.Compensation)) throw Invalid();
            slots[input.Slot] = (input.Module, input.Compensation);
        }
        return inputs.ToFrozenDictionary();
    }
    public DriverDescriptor Descriptor => NativeMessages.Descriptor("keysight.hp3497a", "3497A", Supported);
    public IReadOnlySet<ExecutionMode> ExecutionModes => NativeMessages.Modes(_native is not null);
    private Core Select(DriverOpenContext context)
    {
        NativeMessages.ValidateOpen(context, Supported);
        if (!_routes.TryGetValue(NativeMessages.Key(context.Candidate.Endpoint), out var inputs)) throw Invalid();
        return new Core(inputs);
    }
    public IDeviceBackend Open(DriverOpenContext context) { var core = Select(context); return new MessageDevice(core, _connector.Open(context), context.Ownership); }
    public async ValueTask<IAsyncDeviceBackend> OpenNativeAsync(DriverOpenContext context, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested(); var core = Select(context);
        if (_native is null) throw NativeMessages.Error(DriverErrorKind.UnsupportedExecutionMode, "Native connector not configured");
        var io = await _native.OpenAsync(context, cancellationToken).ConfigureAwait(false);
        return new AsyncMessageDevice(core, io, context.Ownership);
    }
    private sealed class Core(FrozenDictionary<uint, byte> inputs) : IMessageEngine
    {
        public IReadOnlySet<Operation> Operations => Supported;
        public IReadOnlyList<MessageStep> Prepare(CatalogRequest request)
        {
            if (request is not DataAcquisitionMeasureVoltageDcRequest measure) throw NativeMessages.Error(DriverErrorKind.UnsupportedOperation, "Only scalar DC acquisition supported");
            if (!inputs.TryGetValue(measure.Channel, out var native)) throw Invalid();
            var range = measure.Range switch { 0.1 => 1, 1 => 2, 10 => 3, 100 => 4, null => 5, _ => throw Invalid() };
            return new[] { new MessageStep($"VT4AE0VC0VS0VF1VN1VW0SO1VR{range}AI{native}", Parse) };
        }
        private static CatalogReply Parse(string body)
        {
            static DriverException Bad() => NativeMessages.Error(DriverErrorKind.Parse, "Expected eleven-byte HP3497A voltage record");
            if (body.Length != 11 || body[0] is not ('+' or '-') || !char.IsAsciiDigit(body[1]) || body[2] != '.' || !body.AsSpan(3, 5).ToArray().All(char.IsAsciiDigit) || body[8] != 'E' || body[9] is not ('+' or '-') || !char.IsAsciiDigit(body[10])) throw Bad();
            if (body == "+9.00000E+9") return new CatalogReply.Numeric(new(0, Unit.Volt, MeasurementStatus.Overload));
            if (!double.TryParse(body, NumberStyles.AllowLeadingSign | NumberStyles.AllowDecimalPoint | NumberStyles.AllowExponent, CultureInfo.InvariantCulture, out var value) || !double.IsFinite(value) || Math.Abs(value) > 120) throw Bad();
            return new CatalogReply.Numeric(new(value, Unit.Volt));
        }
    }
}
