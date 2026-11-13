using System.Collections.Frozen;
using System.Globalization;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Scpi;
namespace InstrumentComponents.Drivers.Keysight;
public enum Hp3585Commissioning { Unknown, QualifiedNativeScalarConfirmed }
public sealed record Hp3585Binding(Endpoint Endpoint,Hp3585Commissioning Commissioning);
/// <summary>Native scalar subset; stored peak selection and selected-point validity are distinct.</summary>
public sealed class Hp3585Factory:IDriverFactory
{
    private readonly FrozenSet<(string,string)> _routes;
    private readonly IScpiConnector _connector;
    private readonly IAsyncScpiConnector? _native;
    private static DriverException Error(DriverErrorKind kind,string message)=>new(kind,message);
    private static readonly IReadOnlySet<Operation> Supported=new[]{Operation.CatalogSpectrumAnalyzerSetCenterFrequency,Operation.CatalogSpectrumAnalyzerSetSpan,Operation.CatalogSpectrumAnalyzerSetRbw,Operation.CatalogSpectrumAnalyzerSetVbw,Operation.CatalogSpectrumAnalyzerSetRefLevel,Operation.CatalogSpectrumAnalyzerMarkerPeak,Operation.CatalogSpectrumAnalyzerMarkerX,Operation.CatalogSpectrumAnalyzerMarkerY}.ToFrozenSet();
    private static void EndpointValid(Endpoint endpoint)
    {
        if(endpoint is null||string.IsNullOrWhiteSpace(endpoint.Scheme)||endpoint.Scheme.Any(c=>!char.IsAscii(c)))throw Error(DriverErrorKind.InvalidArgument,"Nonempty hpib endpoint required");
        NativeMessages.ValidateEndpoint(endpoint);
    }
    public Hp3585Factory(IEnumerable<Hp3585Binding> bindings,IScpiConnector connector,IAsyncScpiConnector? nativeConnector=null)
    {
        ArgumentNullException.ThrowIfNull(bindings);ArgumentNullException.ThrowIfNull(connector);
        var copy=bindings.Take(4097).ToArray();if(copy.Length is 0 or >4096)throw Error(DriverErrorKind.InvalidArgument,"Bind1..4096 qualified native endpoints");
        var routes=new HashSet<(string,string)>();
        foreach(var binding in copy)
        {
            if(binding is null)throw Error(DriverErrorKind.InvalidArgument,"Null binding");EndpointValid(binding.Endpoint);
            if(binding.Commissioning!=Hp3585Commissioning.QualifiedNativeScalarConfirmed||!routes.Add(NativeMessages.Key(binding.Endpoint)))throw Error(DriverErrorKind.InvalidArgument,"Explicit qualification and unique endpoints required");
        }
        _routes=routes.ToFrozenSet();_connector=connector;_native=nativeConnector;
    }
    public DriverDescriptor Descriptor=>new("keysight.hp3585b",DriverContract.Major,new[]{"HP","HEWLETT-PACKARD","Agilent Technologies"}.Select(m=>new ExactIdentityRule("hpib",m,"3585B")),Supported);
    public IReadOnlySet<ExecutionMode> ExecutionModes=>NativeMessages.Modes(_native is not null);
    private void Selected(DriverOpenContext context)
    {
        EndpointValid(context.Candidate.Endpoint);
        if(!Enum.IsDefined(context.Ownership))throw Error(DriverErrorKind.InvalidArgument,"Unknown ownership");
        if(!_routes.Contains(NativeMessages.Key(context.Candidate.Endpoint)))throw Error(DriverErrorKind.InvalidArgument,"Unbound qualified endpoint");
        if(context.RequiredOperations is{} required&&!required.IsSubsetOf(Supported))throw Error(DriverErrorKind.UnsupportedOperation,"Required operation outside native scalar subset");
    }
    public IDeviceBackend Open(DriverOpenContext context){Selected(context);return new MessageDevice(new Core(),_connector.Open(context),context.Ownership);}
    public async ValueTask<IAsyncDeviceBackend> OpenNativeAsync(DriverOpenContext context,CancellationToken cancellationToken=default)
    {
        cancellationToken.ThrowIfCancellationRequested();Selected(context);var native=_native??throw Error(DriverErrorKind.UnsupportedExecutionMode,"Native connector not configured");
        return new AsyncMessageDevice(new Core(),await native.OpenAsync(context,cancellationToken).ConfigureAwait(false),context.Ownership);
    }
    private static uint IntegerHz(double value)
    {
        if(!double.IsFinite(value)||value<1||value>40100000||value!=Math.Truncate(value))throw Error(DriverErrorKind.InvalidArgument,"Require integer1..40100000Hz");return(uint)value;
    }
    private static int Tenths(double value)
    {
        if(double.IsFinite(value)&&value>=-70&&value<=40){var n=(int)Math.Round(value*10,MidpointRounding.AwayFromZero);if(value==n/10.0)return n;}
        throw Error(DriverErrorKind.InvalidArgument,"Reference requires integer tenths in -70..40dBm");
    }
    private static double Field(string text,int integers,int fractions)
    {
        var point=1+integers;
        if(text.Length!=point+1+fractions||text[0] is not('+' or '-')||text[point]!='.'||text.AsSpan(point+1).IndexOfAnyExceptInRange('0','9')>=0)throw Error(DriverErrorKind.Parse,"Invalid native fixed field");
        var digits=text[1..point].TrimStart(' ');if(digits.Length==0||digits.Any(c=>!char.IsAsciiDigit(c)))throw Error(DriverErrorKind.Parse,"Invalid native leading-space field");
        return QualifiedScpiFactory.Decimal(text[..1]+digits+"."+text[(point+1)..]);
    }
    private sealed class Core:IMessageEngine
    {
        public IReadOnlySet<Operation> Operations=>Supported;
        public IReadOnlyList<MessageStep> Prepare(CatalogRequest request)
        {
            string command;
            switch(request)
            {
                case SpectrumAnalyzerSetCenterFrequencyRequest p:command="CF"+IntegerHz(p.Hz).ToString(CultureInfo.InvariantCulture)+"HZ";break;
                case SpectrumAnalyzerSetSpanRequest p:command="FS"+IntegerHz(p.Hz).ToString(CultureInfo.InvariantCulture)+"HZ";break;
                case SpectrumAnalyzerSetRbwRequest p:command=Bandwidth(p.Hz,false);break;
                case SpectrumAnalyzerSetVbwRequest p:command=Bandwidth(p.Hz,true);break;
                case SpectrumAnalyzerSetRefLevelRequest p:
                    var n=Tenths(p.Dbm);command="RL"+(n<0?"-":"")+(Math.Abs(n)/10).ToString(CultureInfo.InvariantCulture)+"."+(Math.Abs(n)%10).ToString(CultureInfo.InvariantCulture)+"DM";break;
                case SpectrumAnalyzerMarkerPeakRequest:return[new("CL"),new("SP")];
                case SpectrumAnalyzerMarkerXRequest:return[new("CL"),new("D2,T6",b=>Decode(b,Unit.Hertz))];
                case SpectrumAnalyzerMarkerYRequest:return[new("CL"),new("D2,T6",b=>Decode(b,Unit.Dbm))];
                default:throw Error(DriverErrorKind.UnsupportedOperation,"Operation outside native scalar subset");
            }
            return[new(command)];
        }
        private static string Bandwidth(double hz,bool video)
        {
            var value=IntegerHz(hz);if(!new uint[]{3,10,30,100,300,1000,3000,10000,30000}.Contains(value)&&!(video&&value==1))throw Error(DriverErrorKind.InvalidArgument,"Unsupported discrete bandwidth");
            return(video?"VB":"RB")+value.ToString(CultureInfo.InvariantCulture)+"HZ";
        }
        private static CatalogReply Decode(string body,Unit unit)
        {
            if(body.Length!=24||body.Any(c=>!char.IsAscii(c))||body[12]!=','||body[21..]!="E-0")throw Error(DriverErrorKind.Parse,"Expected24-character native D2 record");
            var frequency=Field(body[..12],9,1);var amplitude=Field(body[13..21],4,2);
            if(frequency<0||frequency>40100000)throw Error(DriverErrorKind.Parse,"Frequency outside native record band");
            return new CatalogReply.Numeric(new(unit==Unit.Hertz?frequency:amplitude,unit,MeasurementStatus.Valid));
        }
    }
}
