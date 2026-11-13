using System.Collections.Frozen;
using System.Globalization;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Drivers.Keysight;

public enum QualifiedScpiModel { Agilent34401A, Agilent33120A, Agilent53132A }
public enum QualifiedScpiCommissioning { Unknown, GpibScpiConfirmed, ContinuousSine50OhmVppOutputChangesAuthorized }
public sealed record QualifiedScpiBinding(Endpoint Endpoint, QualifiedScpiCommissioning Commissioning);

/// <summary>Explicit GPIB commissioning; no probing, reset or output changes during open.</summary>
public sealed class QualifiedScpiFactory : IDriverFactory
{
    private readonly QualifiedScpiModel _model;
    private readonly FrozenSet<(string,string)> _routes;
    private readonly IScpiConnector _connector;
    private readonly IAsyncScpiConnector? _native;
    private static DriverException Error(DriverErrorKind kind,string message)=>new(kind,message);
    private static void EndpointValid(Endpoint endpoint)
    {
        if (endpoint is null || string.IsNullOrWhiteSpace(endpoint.Scheme) || !endpoint.Scheme.Trim().Equals("scpi",StringComparison.OrdinalIgnoreCase)
            || endpoint.Scheme.Any(c=>!char.IsAscii(c)) || string.IsNullOrWhiteSpace(endpoint.Address))
            throw Error(DriverErrorKind.InvalidArgument,"Commissioned nonempty scpi endpoint required");
    }
    public QualifiedScpiFactory(QualifiedScpiModel model,IEnumerable<QualifiedScpiBinding> bindings,IScpiConnector connector,IAsyncScpiConnector? nativeConnector=null)
    {
        ArgumentNullException.ThrowIfNull(bindings);ArgumentNullException.ThrowIfNull(connector);
        if(!Enum.IsDefined(model))throw Error(DriverErrorKind.InvalidArgument,"Unknown model");
        var frozen=bindings.Take(4097).ToArray();
        if(frozen.Length is 0 or >4096)throw Error(DriverErrorKind.InvalidArgument,"Bind1..4096 endpoints");
        var expected=model==QualifiedScpiModel.Agilent33120A?QualifiedScpiCommissioning.ContinuousSine50OhmVppOutputChangesAuthorized:QualifiedScpiCommissioning.GpibScpiConfirmed;
        var routes=new HashSet<(string,string)>();
        foreach(var binding in frozen)
        {
            if(binding is null)throw Error(DriverErrorKind.InvalidArgument,"Null binding");
            EndpointValid(binding.Endpoint);
            if(binding.Commissioning!=expected || !routes.Add(NativeMessages.Key(binding.Endpoint)))
                throw Error(DriverErrorKind.InvalidArgument,"Explicit matching commissioning and unique endpoints required");
        }
        _model=model;_routes=routes.ToFrozenSet();_connector=connector;_native=nativeConnector;
    }
    private static IReadOnlySet<Operation> Operations(QualifiedScpiModel model)=>(model switch
    {
        QualifiedScpiModel.Agilent34401A=>new[]{Operation.CatalogDmmMeasureVoltageDc,Operation.CatalogDmmMeasureVoltageAc,Operation.CatalogDmmMeasureCurrentDc,Operation.CatalogDmmMeasureCurrentAc,Operation.CatalogDmmMeasureResistance,Operation.CatalogDmmMeasureResistance2Wire,Operation.CatalogDmmMeasureResistance4Wire},
        QualifiedScpiModel.Agilent33120A=>new[]{Operation.CatalogFunctionGeneratorSetFrequency,Operation.CatalogFunctionGeneratorSetAmplitude,Operation.CatalogFunctionGeneratorSetOffset,Operation.CatalogFunctionGeneratorReadFrequency},
        _=>new[]{Operation.CatalogCounterSelectChannel,Operation.CatalogCounterSetGateTime,Operation.CatalogCounterMeasureFrequency,Operation.CatalogCounterMeasurePeriod},
    }).ToFrozenSet();
    public DriverDescriptor Descriptor
    {
        get
        {
            var model=_model switch{QualifiedScpiModel.Agilent34401A=>"34401A",QualifiedScpiModel.Agilent33120A=>"33120A",_=>"53132A"};
            var aliases=_model==QualifiedScpiModel.Agilent33120A?new[]{"HP","HEWLETT-PACKARD","Agilent Technologies"}:new[]{"HP","HEWLETT-PACKARD","Agilent Technologies","Keysight Technologies"};
            return new("keysight."+model.ToLowerInvariant(),DriverContract.Major,aliases.Select(m=>new ExactIdentityRule("scpi",m,model)),Operations(_model));
        }
    }
    public IReadOnlySet<ExecutionMode> ExecutionModes=>NativeMessages.Modes(_native is not null);
    private Core Selected(DriverOpenContext context)
    {
        EndpointValid(context.Candidate.Endpoint);
        if(!Enum.IsDefined(context.Ownership))throw Error(DriverErrorKind.InvalidArgument,"Unknown ownership");
        if(!_routes.Contains(NativeMessages.Key(context.Candidate.Endpoint)))throw Error(DriverErrorKind.InvalidArgument,"Unbound commissioned endpoint");
        if(context.RequiredOperations is{} required&&!required.IsSubsetOf(Operations(_model)))throw Error(DriverErrorKind.UnsupportedOperation,"Required operation unavailable");
        return new(_model);
    }
    public IDeviceBackend Open(DriverOpenContext context)
    {
        var core=Selected(context);return new MessageDevice(core,_connector.Open(context),context.Ownership);
    }
    public async ValueTask<IAsyncDeviceBackend> OpenNativeAsync(DriverOpenContext context,CancellationToken cancellationToken=default)
    {
        cancellationToken.ThrowIfCancellationRequested();var core=Selected(context);
        var connector=_native??throw Error(DriverErrorKind.UnsupportedExecutionMode,"Native connector not configured");
        return new AsyncMessageDevice(core,await connector.OpenAsync(context,cancellationToken).ConfigureAwait(false),context.Ownership);
    }
    internal static string Real12(double value)
    {
        // Standard E11, matching the previously qualified native-counter algorithm.
        var text=value.ToString("E11",CultureInfo.InvariantCulture);var marker=text.IndexOf('E');
        var exponent=int.Parse(text.AsSpan(marker+1),CultureInfo.InvariantCulture);
        return text[..(marker+1)]+(exponent<0?"-":"+")+Math.Abs(exponent).ToString("00",CultureInfo.InvariantCulture);
    }
    internal static double Decimal(string body)
    {
        var i=0;if(body.Length>0&&body[0] is '+' or '-')i++;
        var start=i;while(i<body.Length&&char.IsAsciiDigit(body[i]))i++;
        if(i==start)throw Error(DriverErrorKind.Parse,"Expected decimal digits");
        if(i<body.Length&&body[i]=='.')
        {
            i++;start=i;while(i<body.Length&&char.IsAsciiDigit(body[i]))i++;
            if(i==start)throw Error(DriverErrorKind.Parse,"Fraction digits required");
        }
        if(i<body.Length&&body[i] is 'E' or 'e')
        {
            i++;if(i<body.Length&&body[i] is '+' or '-')i++;
            start=i;while(i<body.Length&&char.IsAsciiDigit(body[i]))i++;
            if(i==start)throw Error(DriverErrorKind.Parse,"Exponent digits required");
        }
        if(i!=body.Length||!double.TryParse(body,NumberStyles.AllowLeadingSign|NumberStyles.AllowDecimalPoint|NumberStyles.AllowExponent,CultureInfo.InvariantCulture,out var value)||!double.IsFinite(value))
            throw Error(DriverErrorKind.Parse,"Unexpected or nonfinite scalar");
        return value;
    }
    internal static double Bounded(string body)
    {
        var value=Decimal(body);if(Math.Abs(value)>=1e30)throw Error(DriverErrorKind.Parse,"Implausible scalar");return value;
    }
    private static bool Within(double value,double min,double max)=>double.IsFinite(value)&&value>=min&&value<=max;
    private static bool Pair(double vpp,double offset)=>Math.Abs(offset)+vpp/2<=5&&Math.Abs(offset)<=2*vpp;
    private static uint GateMilliseconds(double seconds)
    {
        if(Within(seconds,.001,1000))
        {
            var milliseconds=(uint)Math.Round(seconds*1000,MidpointRounding.AwayFromZero);
            if(seconds==milliseconds/1000.0)return milliseconds;
        }
        throw Error(DriverErrorKind.InvalidArgument,"Gate requires1..1000000 integer milliseconds");
    }
    private sealed class Core(QualifiedScpiModel model):IMessageEngine
    {
        private uint _channel=1,_gateMilliseconds=100;
        public IReadOnlySet<Operation> Operations=>QualifiedScpiFactory.Operations(model);
        private static DriverException Parse()=>Error(DriverErrorKind.Parse,"Unexpected scalar grammar");
        private static DriverException Device()=>Error(DriverErrorKind.Device,"Device setting/result outside commissioned contract");
        private static CatalogReply Numeric(double value,Unit unit)=>new CatalogReply.Numeric(new(value,unit,MeasurementStatus.Valid));
        private static CatalogReply? Token(string body,string expected,params string[] known)
        {
            if(body==expected)return null;if(known.Contains(body,StringComparer.Ordinal))throw Device();throw Parse();
        }
        private static List<MessageStep> Guards()=>[
            new(":FUNC:SHAP?",b=>Token(b,"SIN","SQU","TRI","RAMP","NOIS","DC","USER")),
            new(":OUTP:LOAD?",b=>{var v=Decimal(b);if(v==9.9e37)throw Device();if(Math.Abs(v)>=1e30||v<=0)throw Parse();if(v!=50)throw Device();return null;}),
            new(":VOLT:UNIT?",b=>Token(b,"VPP","VRMS","DBM")),
            ..new[]{":AM:STAT?",":FM:STAT?",":BM:STAT?",":FSK:STAT?",":SWE:STAT?"}.Select(s=>new MessageStep(s,b=>Token(b,"0","1")))];
        public IReadOnlyList<MessageStep> Prepare(CatalogRequest request)
        {
            if(!Operations.Contains(request.Operation.Runtime()))throw Error(DriverErrorKind.UnsupportedOperation,"Operation outside qualified subset");
            if(model==QualifiedScpiModel.Agilent34401A)
            {
                var(root,range,max,unit)=request switch
                {
                    DmmMeasureVoltageDcRequest p=>(":MEAS:VOLT:DC?",p.Range,1000.0,Unit.Volt),DmmMeasureVoltageAcRequest p=>(":MEAS:VOLT:AC?",p.Range,750.0,Unit.Volt),
                    DmmMeasureCurrentDcRequest p=>(":MEAS:CURR:DC?",p.Range,3.0,Unit.Ampere),DmmMeasureCurrentAcRequest p=>(":MEAS:CURR:AC?",p.Range,3.0,Unit.Ampere),
                    DmmMeasureResistanceRequest p=>(":MEAS:RES?",p.Range,1e8,Unit.Ohm),DmmMeasureResistance2WireRequest p=>(":MEAS:RES?",p.Range,1e8,Unit.Ohm),DmmMeasureResistance4WireRequest p=>(":MEAS:FRES?",p.Range,1e8,Unit.Ohm),
                    _=>throw Error(DriverErrorKind.UnsupportedOperation,"Unsupported DMM method")
                };
                if(range is{} r&&!Within(r,double.Epsilon,max))throw Error(DriverErrorKind.InvalidArgument,"Range outside qualified software bound");
                return[new(root+" "+(range is{} value?Real12(value):"DEF")+",DEF",b=>Dmm(b,unit))];
            }
            if(model==QualifiedScpiModel.Agilent53132A)
            {
                if(request is CounterSelectChannelRequest channel){if(channel.Channel is not(1 or 2))throw Error(DriverErrorKind.InvalidArgument,"Only channels1/2");return[];}
                if(request is CounterSetGateTimeRequest gate){GateMilliseconds(gate.Seconds);return[];}
                var(function,unit)=request is CounterMeasureFrequencyRequest?("FREQ",Unit.Hertz):("PER",Unit.Second);
                var channelText=_channel.ToString(CultureInfo.InvariantCulture);
                var gateText=(_gateMilliseconds/1000).ToString(CultureInfo.InvariantCulture)+"."+(_gateMilliseconds%1000).ToString("000",CultureInfo.InvariantCulture);
                return[new(":CONF:"+function+" DEF,DEF,(@"+channelText+")"),new(":SENS:FREQ:ARM:STAR:SOUR IMM"),new(":SENS:FREQ:ARM:STOP:SOUR TIM"),new(":SENS:FREQ:ARM:STOP:TIM "+gateText),new(":FORM ASC"),new(":READ?",b=>Counter(b,unit))];
            }
            MessageStep[] tail;
            switch(request)
            {
                case FunctionGeneratorSetFrequencyRequest p:
                    if(!Within(p.Hz,.0001,15e6))throw Error(DriverErrorKind.InvalidArgument,"Frequency outside qualified sine range");
                    tail=[new(":FREQ "+Real12(p.Hz))];break;
                case FunctionGeneratorSetAmplitudeRequest p:
                    if(!Within(p.Vpp,.05,10))throw Error(DriverErrorKind.InvalidArgument,"Amplitude outside50Ohm Vpp range");
                    var amplitude=Real12(p.Vpp);var roundedAmplitude=Decimal(amplitude);
                    tail=[new(":VOLT:OFFS?",b=>{var offset=Bounded(b);if(!Within(offset,-4,4))throw Device();if(!Pair(p.Vpp,offset)||!Pair(roundedAmplitude,offset))throw Error(DriverErrorKind.InvalidArgument,"Requested amplitude conflicts with current offset");return null;}),new(":VOLT "+amplitude)];break;
                case FunctionGeneratorSetOffsetRequest p:
                    if(!Within(p.Volts,-4,4))throw Error(DriverErrorKind.InvalidArgument,"Offset outside coupled envelope");
                    var offsetText=Real12(p.Volts);var roundedOffset=Decimal(offsetText);
                    tail=[new(":VOLT?",b=>{var vpp=Bounded(b);if(!Within(vpp,.05,10))throw Device();if(!Pair(vpp,p.Volts)||!Pair(vpp,roundedOffset))throw Error(DriverErrorKind.InvalidArgument,"Requested offset conflicts with current amplitude");return null;}),new(":VOLT:OFFS "+offsetText)];break;
                default:tail=[new(":FREQ?",b=>{var value=Bounded(b);if(!Within(value,.0001,15e6))throw Device();return Numeric(value,Unit.Hertz);})];break;
            }
            var steps=Guards();steps.AddRange(tail);return steps;
        }
        public void Commit(CatalogRequest request)
        {
            if(model!=QualifiedScpiModel.Agilent53132A)return;
            if(request is CounterSelectChannelRequest channel)_channel=channel.Channel;
            if(request is CounterSetGateTimeRequest gate)_gateMilliseconds=GateMilliseconds(gate.Seconds);
        }
        private static CatalogReply Dmm(string body,Unit unit)
        {
            if(body.Length!=15||body[0] is not('+' or '-')||!char.IsAsciiDigit(body[1])||body[2]!='.'||body.AsSpan(3,8).IndexOfAnyExceptInRange('0','9')>=0||body[11]!='E'||body[12] is not('+' or '-')||body.AsSpan(13,2).IndexOfAnyExceptInRange('0','9')>=0)throw Parse();
            if(body=="+9.90000000E+37")return new CatalogReply.Numeric(new(0,unit,MeasurementStatus.Overload));
            return Numeric(Bounded(body),unit);
        }
        private static CatalogReply Counter(string body,Unit unit)
        {
            var split=body.IndexOfAny(['E','e']);if(split<0)throw Parse();
            var exponent=body[(split+1)..].TrimStart('+','-');
            if(exponent.Length is <1 or >3||exponent.Any(c=>!char.IsAsciiDigit(c)))throw Parse();
            var digits=new string(body[..split].Where(char.IsAsciiDigit).ToArray()).TrimStart('0');
            if(digits.Length>15)throw Parse();
            var value=Decimal(body);if(value<=0||value==9.91e37)throw Device();return Numeric(value,unit);
        }
    }
}
