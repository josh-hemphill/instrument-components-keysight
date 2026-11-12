using System.Collections.Frozen;
using System.Globalization;
using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Drivers.Keysight;

public enum HpRadioModel { Hp8920A, Hp8920B, Hp8921A }
public enum HpRadioAttenuator { Unknown, Electronic, Mechanical055 }
public enum HpRadioInputPower { Unknown, Standard, Option006, Option007, Option008, Option016 }
public enum HpRadioSourcePort { Unknown, RfInOut, DuplexOut }
public enum HpRadioReceiverInput { Unknown, RfInOut, Antenna }
/// <summary>Confirms all model-spec prerequisites: native GPIB, initial Manual RX/TX,
/// independent frequency display, inhibited external PTT, exclusive access,
/// qualified carrier, and bounded finite complete-message IO.</summary>
public enum HpRadioCommissioning { Unknown, Confirmed }
public sealed record HpRadioConfiguration(HpRadioModel Model, string Firmware, HpRadioAttenuator Attenuator,
    HpRadioInputPower InputPower, HpRadioSourcePort SourcePort, HpRadioReceiverInput ReceiverInput, HpRadioCommissioning Commissioning);
public sealed record HpRadioBinding(Endpoint Endpoint, HpRadioConfiguration Configuration);

/// <summary>Static, explicitly commissioned HP scalar RF routes. See spec/legacy/HP892*.md.</summary>
public sealed class HpRadioFactory : IDriverFactory
{
    private readonly HpRadioModel _model;
    private readonly FrozenDictionary<(string, string), HpRadioConfiguration> _routes;
    private readonly IScpiConnector _connector;
    private readonly IAsyncScpiConnector? _native;
    private static DriverException Invalid() => NativeMessages.Error(DriverErrorKind.InvalidArgument, "Unqualified model, firmware, inventory, endpoint or RF setting");
    public HpRadioFactory(HpRadioModel model, IEnumerable<HpRadioBinding> bindings, IScpiConnector connector, IAsyncScpiConnector? nativeConnector = null)
    {
        ArgumentNullException.ThrowIfNull(bindings); ArgumentNullException.ThrowIfNull(connector);
        if (!Enum.IsDefined(model)) throw Invalid();
        var routes = new Dictionary<(string, string), HpRadioConfiguration>();
        foreach (var binding in bindings)
        {
            NativeMessages.ValidateEndpoint(binding.Endpoint); ValidateConfiguration(model, binding.Configuration);
            if (routes.Count == 4096 || !routes.TryAdd(NativeMessages.Key(binding.Endpoint), binding.Configuration with { })) throw Invalid();
        }
        if (routes.Count == 0) throw Invalid();
        _model = model; _routes = routes.ToFrozenDictionary(); _connector = connector; _native = nativeConnector;
    }
    public string Model => _model switch { HpRadioModel.Hp8920A => "8920A", HpRadioModel.Hp8920B => "8920B", _ => "8921A" };
    public string DriverId => "keysight.hp" + Model.ToLowerInvariant();
    public DriverDescriptor Descriptor => NativeMessages.Descriptor(DriverId, Model, _routes.Values.SelectMany(Operations).ToFrozenSet());
    public IReadOnlySet<ExecutionMode> ExecutionModes => NativeMessages.Modes(_native is not null);
    private static FrozenSet<Operation> Operations(HpRadioConfiguration config)
    {
        var result = new HashSet<Operation> { Operation.CatalogRfSignalGeneratorConfigure, Operation.CatalogRfSignalGeneratorOutputEnable,
            Operation.CatalogRfSignalGeneratorOutputStateQuery, Operation.CatalogRfReceiverMeasureFrequency };
        if (config.ReceiverInput == HpRadioReceiverInput.RfInOut) result.Add(Operation.CatalogRfReceiverMeasurePower);
        return result.ToFrozenSet();
    }
    private static void ValidateConfiguration(HpRadioModel model, HpRadioConfiguration config)
    {
        if (config.Model != model || config.Commissioning != HpRadioCommissioning.Confirmed
            || !Enum.IsDefined(config.Attenuator) || config.Attenuator == HpRadioAttenuator.Unknown
            || !Enum.IsDefined(config.SourcePort) || config.SourcePort == HpRadioSourcePort.Unknown
            || !Enum.IsDefined(config.ReceiverInput) || config.ReceiverInput == HpRadioReceiverInput.Unknown) throw Invalid();
        var allowed = model switch
        {
            HpRadioModel.Hp8920A => config.InputPower is HpRadioInputPower.Standard or HpRadioInputPower.Option007 or HpRadioInputPower.Option008 or HpRadioInputPower.Option016,
            HpRadioModel.Hp8920B => config.InputPower is HpRadioInputPower.Standard or HpRadioInputPower.Option006 or HpRadioInputPower.Option007 or HpRadioInputPower.Option016
                && !(config.Attenuator == HpRadioAttenuator.Mechanical055 && config.InputPower is HpRadioInputPower.Option006 or HpRadioInputPower.Option007),
            _ => config.InputPower == HpRadioInputPower.Standard
        };
        var family = model == HpRadioModel.Hp8920B ? 'B' : 'A'; var minimum = model switch { HpRadioModel.Hp8920A => 18, HpRadioModel.Hp8920B => 6, _ => 15 };
        var f = config.Firmware;
        if (!allowed || f is null || f.Length != 7 || f[0] != family || f[1] != '.' || f[4] != '.'
            || !char.IsAsciiDigit(f[2]) || !char.IsAsciiDigit(f[3]) || !char.IsAsciiDigit(f[5]) || !char.IsAsciiDigit(f[6]) || (f[2] - '0') * 10 + f[3] - '0' < minimum) throw Invalid();
    }
    private Core Select(DriverOpenContext context)
    {
        NativeMessages.ValidateEndpoint(context.Candidate.Endpoint);
        if (!_routes.TryGetValue(NativeMessages.Key(context.Candidate.Endpoint), out var config)) throw Invalid();
        NativeMessages.ValidateOpen(context, Operations(config)); return new Core(config);
    }
    public IDeviceBackend Open(DriverOpenContext context) { var core = Select(context); return new MessageDevice(core, _connector.Open(context), context.Ownership); }
    public async ValueTask<IAsyncDeviceBackend> OpenNativeAsync(DriverOpenContext context, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested(); var core = Select(context);
        if (_native is null) throw NativeMessages.Error(DriverErrorKind.UnsupportedExecutionMode, "Native connector not configured");
        var io = await _native.OpenAsync(context, cancellationToken).ConfigureAwait(false); return new AsyncMessageDevice(core, io, context.Ownership);
    }
    private static (double Min, double Max) SourceFrequency(HpRadioConfiguration c) => (c.Attenuator == HpRadioAttenuator.Electronic ? 30_000_000 : 250_000, 1_000_000_000);
    private static (double Min, double Max) SourceLevel(HpRadioConfiguration c)
    {
        var electronic = c.Attenuator == HpRadioAttenuator.Electronic;
        if (c.SourcePort == HpRadioSourcePort.DuplexOut) return (-127, electronic ? c.Model == HpRadioModel.Hp8920B ? 3.5 : 5 : 7);
        var max = c.InputPower switch { HpRadioInputPower.Standard => electronic ? -20.5 : -19,
            HpRadioInputPower.Option006 => -12.5, HpRadioInputPower.Option007 => electronic ? -6.5 : -5,
            HpRadioInputPower.Option008 => electronic ? -10.5 : -9, HpRadioInputPower.Option016 => electronic || c.Model == HpRadioModel.Hp8920B ? -22.5 : -21, _ => throw Invalid() };
        return (-137, max);
    }
    private static (double Min, double Max) ReceiverFrequency(HpRadioConfiguration c) => (c.Attenuator == HpRadioAttenuator.Mechanical055 ? 400_000 : c.Model == HpRadioModel.Hp8920A ? 30_000_000 : 10_000_000, 1_000_000_000);
    // Frozen nearest-binary64 constants, never recompute platform log10.
    private static (double Min, double Max) ReceiverPower(HpRadioConfiguration c) => c.InputPower switch
    {
        HpRadioInputPower.Standard => (0, 47.78151250383644), HpRadioInputPower.Option006 => (-13.010299956639813, 40),
        HpRadioInputPower.Option007 => (-13.979400086720377, 33.802112417116064), HpRadioInputPower.Option008 => (-10, 37.78151250383644),
        HpRadioInputPower.Option016 => (2.041199826559248, 50), _ => throw Invalid()
    };
    private static string SourceToken(HpRadioConfiguration c) => c.SourcePort == HpRadioSourcePort.RfInOut ? "RF Out" : "Dupl";
    private static string ReceiverToken(HpRadioConfiguration c) => c.ReceiverInput == HpRadioReceiverInput.RfInOut ? "RF In" : "Ant";
    private static DriverException Bad() => NativeMessages.Error(DriverErrorKind.Parse, "Malformed or implausible RF response");
    private static DriverException Mismatch() => NativeMessages.Error(DriverErrorKind.Device, "Instrument readback does not match requested setting");

    private sealed record ExactDecimal(bool Negative, string Digits, int Point) : IComparable<ExactDecimal>
    {
        public static ExactDecimal Parse(string text)
        {
            var i = 0; var negative = false;
            if (text.Length > 0 && text[0] is '+' or '-') { negative = text[0] == '-'; i++; }
            var digits = new System.Text.StringBuilder(); var integer = 0;
            while (i < text.Length && char.IsAsciiDigit(text[i])) { digits.Append(text[i++]); integer++; }
            if (i < text.Length && text[i] == '.') { i++; while (i < text.Length && char.IsAsciiDigit(text[i])) digits.Append(text[i++]); }
            if (digits.Length == 0) throw Bad();
            var exponent = 0;
            if (i < text.Length && text[i] is 'E' or 'e')
            {
                i++; var sign = 1;
                if (i < text.Length && text[i] is '+' or '-') { if (text[i] == '-') sign = -1; i++; }
                var start = i; while (i < text.Length && char.IsAsciiDigit(text[i])) exponent = Math.Min(10_000, exponent * 10 + text[i++] - '0');
                if (i == start) throw Bad(); exponent *= sign;
            }
            if (i != text.Length) throw Bad();
            var raw = digits.ToString(); var leading = 0; while (leading < raw.Length && raw[leading] == '0') leading++;
            if (leading == raw.Length) return new(false, "0", 1);
            return new(negative, raw[leading..].TrimEnd('0'), integer - leading + exponent);
        }
        public int CompareTo(ExactDecimal? other)
        {
            ArgumentNullException.ThrowIfNull(other);
            var zero = Digits == "0"; var otherZero = other.Digits == "0";
            if (zero || otherZero) return zero && otherZero ? 0 : zero ? other.Negative ? 1 : -1 : Negative ? -1 : 1;
            if (Negative != other.Negative) return Negative ? -1 : 1;
            var cmp = Point.CompareTo(other.Point);
            if (cmp == 0) for (var i = 0; i < Math.Max(Digits.Length, other.Digits.Length); i++) { cmp = (i < Digits.Length ? Digits[i] : '0').CompareTo(i < other.Digits.Length ? other.Digits[i] : '0'); if (cmp != 0) break; }
            return Negative ? -cmp : cmp;
        }
    }
    private static (double Value, ExactDecimal Decimal) Number(string body)
    {
        body = body.Trim(' '); var exact = ExactDecimal.Parse(body);
        if (!double.TryParse(body, NumberStyles.AllowLeadingSign | NumberStyles.AllowDecimalPoint | NumberStyles.AllowExponent, CultureInfo.InvariantCulture, out var value) || !double.IsFinite(value)) throw Bad();
        return (value, exact);
    }
    private static ExactDecimal Hundredths(long value) => ExactDecimal.Parse((value < 0 ? "-" : "") + (Math.Abs(value) / 100).ToString(CultureInfo.InvariantCulture) + "." + (Math.Abs(value) % 100).ToString("00", CultureInfo.InvariantCulture));
    private static string Tenths(long value) => (value < 0 ? "-" : "") + (Math.Abs(value) / 10).ToString(CultureInfo.InvariantCulture) + (value % 10 == 0 ? "" : "." + (Math.Abs(value) % 10).ToString(CultureInfo.InvariantCulture));
    private enum Quoting { Required, Bare, Either }
    private static Func<string, CatalogReply?> State(bool? expected) => body =>
    {
        var actual = body.Trim(' ') switch { "0" => false, "1" => true, _ => throw Bad() };
        if (expected is { } value) { if (actual != value) throw Mismatch(); return null; }
        return new CatalogReply.State(actual);
    };
    private static Func<string, CatalogReply?> Token(string[] domain, string expected, Quoting quoting) => body =>
    {
        body = body.Trim(' '); var quoted = body.Length > 0 && body[0] is '\'' or '"';
        if (quoting == Quoting.Required && !quoted || quoting == Quoting.Bare && quoted) throw Bad();
        if (quoted) { if (body.Length < 2 || body[0] != body[^1]) throw Bad(); body = body[1..^1]; }
        var token = body.ToUpperInvariant(); if (!domain.Contains(token)) throw Bad(); if (token != expected) throw Mismatch(); return null;
    };
    private static Func<string, CatalogReply?> Setting((double Min, double Max) bounds, long expected100, long tolerance100) => body =>
    {
        var (value, exact) = Number(body); if (value < bounds.Min || value > bounds.Max) throw Bad();
        if (exact.CompareTo(Hundredths(expected100 - tolerance100)) < 0 || exact.CompareTo(Hundredths(expected100 + tolerance100)) > 0) throw Mismatch(); return null;
    };
    private static Func<string, CatalogReply?> Measurement((double Min, double Max) bounds, Unit unit) => body =>
    {
        var (value, _) = Number(body); if (value < bounds.Min || value > bounds.Max) throw Bad(); return new CatalogReply.Numeric(new(value, unit));
    };
    private static void W(List<MessageStep> steps, string text) => steps.Add(new(text));
    private static void Q(List<MessageStep> steps, string text, Func<string, CatalogReply?> decode) => steps.Add(new(text, decode));
    private static void Off(List<MessageStep> steps) { W(steps, "DISP RFG"); W(steps, "RFG:AMPL:STAT OFF"); Q(steps, "RFG:AMPL:STAT?", State(false)); }
    private static void Prefix(List<MessageStep> steps)
    {
        Off(steps); W(steps, "DISP CONF"); W(steps, "CONF:ARTS 'Manual'"); W(steps, "CONF:OMOD 'Off'"); W(steps, "CONF:OFL:MODE 'Off'");
        Q(steps, "CONF:ARTS?", Token(["MANUAL", "AUTO"], "MANUAL", Quoting.Required));
        foreach (var text in new[] { "CONF:OMOD?", "CONF:OFL:MODE?" }) Q(steps, text, Token(["OFF", "ON"], "OFF", Quoting.Required));
    }
    private sealed class Core(HpRadioConfiguration config) : IMessageEngine
    {
        private bool _configured, _previous;
        public IReadOnlySet<Operation> Operations => HpRadioFactory.Operations(config);
        public void Begin(CatalogRequest request) { _previous = _configured; _configured = false; }
        public void Commit(CatalogRequest request) => _configured = request switch { RfSignalGeneratorConfigureRequest => true, RfReceiverMeasureFrequencyRequest or RfReceiverMeasurePowerRequest => false, _ => _previous };
        public IReadOnlyList<MessageStep> Prepare(CatalogRequest request)
        {
            var steps = new List<MessageStep>();
            switch (request)
            {
                case RfSignalGeneratorConfigureRequest c:
                    var fb = SourceFrequency(config); var lb = SourceLevel(config);
                    if (!double.IsFinite(c.FrequencyHz) || !double.IsFinite(c.LevelDbm) || c.FrequencyHz < fb.Min || c.FrequencyHz > fb.Max || c.FrequencyHz % 1 != 0 || c.LevelDbm < lb.Min || c.LevelDbm > lb.Max) throw Invalid();
                    var exact = ExactDecimal.Parse(c.LevelDbm.ToString("R", CultureInfo.InvariantCulture));
                    if (exact.Digits.Length - exact.Point > 1) throw Invalid();
                    var level10 = (long)Math.Round(c.LevelDbm * 10);
                    Prefix(steps); Off(steps); W(steps, "RFG:ATT 'Off'"); Q(steps, "RFG:ATT?", Token(["OFF", "ON"], "OFF", Quoting.Required));
                    W(steps, "RFG:OUTP '" + SourceToken(config) + "'"); W(steps, "RFG:FREQ " + c.FrequencyHz.ToString("F0", CultureInfo.InvariantCulture) + " HZ"); W(steps, "RFG:AMPL " + Tenths(level10) + " DBM");
                    W(steps, "RFG:AMPL:UNIT DBM"); Q(steps, "RFG:AMPL:UNIT?", Token(["DBM", "W", "V", "DBUV"], "DBM", Quoting.Either));
                    W(steps, "RFG:AMPL:STAT OFF"); Q(steps, "RFG:AMPL:STAT?", State(false));
                    Q(steps, "RFG:OUTP?", Token(["RF OUT", "DUPL"], SourceToken(config).ToUpperInvariant(), Quoting.Required));
                    Q(steps, "RFG:FREQ?", Setting(fb, (long)c.FrequencyHz * 100, 50)); Q(steps, "RFG:AMPL?", Setting(lb, level10 * 10, 5)); break;
                case RfSignalGeneratorOutputEnableRequest e:
                    if (e.Enabled && !_configured) throw Invalid();
                    W(steps, "DISP RFG"); W(steps, "RFG:AMPL:STAT " + (e.Enabled ? "ON" : "OFF")); Q(steps, "RFG:AMPL:STAT?", State(e.Enabled)); break;
                case RfSignalGeneratorOutputStateQueryRequest:
                    W(steps, "DISP RFG"); Q(steps, "RFG:AMPL:STAT?", State(null)); break;
                case RfReceiverMeasureFrequencyRequest:
                case RfReceiverMeasurePowerRequest:
                    var power = request is RfReceiverMeasurePowerRequest;
                    if (power && config.ReceiverInput != HpRadioReceiverInput.RfInOut) throw NativeMessages.Error(DriverErrorKind.UnsupportedOperation, "Power unavailable on ANT");
                    Prefix(steps); Off(steps); W(steps, "DISP RFAN"); W(steps, "RFAN:INP '" + ReceiverToken(config) + "'"); W(steps, "RFAN:TMOD 'Auto'");
                    Q(steps, "RFAN:INP?", Token(["RF IN", "ANT"], ReceiverToken(config).ToUpperInvariant(), Quoting.Required)); Q(steps, "RFAN:TMOD?", Token(["AUTO", "MANUAL"], "AUTO", Quoting.Required));
                    var p = power ? "MEAS:RFR:POW" : "MEAS:RFR:FREQ:ABS";
                    if (power) { W(steps, p + ":UNIT DBM"); Q(steps, p + ":UNIT?", Token(["DBM", "W"], "DBM", Quoting.Either)); }
                    foreach (var suffix in new[] { "AVER:STAT", "REF:STAT" }) W(steps, p + ":" + suffix + " OFF");
                    foreach (var suffix in new[] { "AVER:STAT", "REF:STAT" }) Q(steps, p + ":" + suffix + "?", State(false));
                    W(steps, p + ":STAT ON"); Q(steps, p + ":STAT?", State(true));
                    W(steps, "TRIG:MODE:SETT FULL"); W(steps, "TRIG:MODE:RETR SING");
                    Q(steps, "TRIG:MODE:SETT?", Token(["FULL", "FAST"], "FULL", Quoting.Bare)); Q(steps, "TRIG:MODE:RETR?", Token(["SING", "REP"], "SING", Quoting.Bare));
                    W(steps, "TRIG:IMM"); Q(steps, p + "?", Measurement(power ? ReceiverPower(config) : ReceiverFrequency(config), power ? Unit.Dbm : Unit.Hertz)); break;
                default: throw NativeMessages.Error(DriverErrorKind.UnsupportedOperation, "Operation unavailable for RF test set");
            }
            return steps;
        }
    }
}
