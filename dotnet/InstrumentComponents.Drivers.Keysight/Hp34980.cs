using System.Collections.Frozen;
using System.Globalization;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Drivers.Keysight;
public enum Hp34980Module { Unknown, Hp34937A, Hp34922A }
public enum Hp34980Direction { Unknown, Input, Output }
public enum Hp34980Commissioning { Unknown, NoAutonomousActionsConfirmed }
public sealed record Hp34980SwitchChannel(uint Channel,byte Slot,ushort Local,Hp34980Module Module);
public sealed record Hp34980DigitalPort(uint Port,byte Slot,ushort Local,Hp34980Direction Direction);
public sealed record Hp34980Configuration(Hp34980Commissioning Commissioning,IReadOnlyList<Hp34980SwitchChannel> Switches,IReadOnlyList<Hp34980DigitalPort> Ports);
public sealed record Hp34980Binding(Endpoint Endpoint,Hp34980Configuration Configuration);

/// <summary>One physical parent resource and lease for all admitted module views.</summary>
public sealed class Hp34980Factory:IDriverFactory
{
    private readonly FrozenDictionary<(string,string),Core> _routes;
    private readonly IScpiConnector _connector;
    private readonly IAsyncScpiConnector? _native;
    private static DriverException Error(DriverErrorKind kind,string message)=>new(kind,message);
    private static void EndpointValid(Endpoint endpoint)
    {
        if(endpoint is null||string.IsNullOrWhiteSpace(endpoint.Scheme)||!endpoint.Scheme.Trim().Equals("scpi",StringComparison.OrdinalIgnoreCase)||endpoint.Scheme.Any(c=>!char.IsAscii(c))||string.IsNullOrWhiteSpace(endpoint.Address))throw Error(DriverErrorKind.InvalidArgument,"Nonempty commissioned scpi endpoint required");
    }
    public Hp34980Factory(IEnumerable<Hp34980Binding> bindings,IScpiConnector connector,IAsyncScpiConnector? nativeConnector=null)
    {
        ArgumentNullException.ThrowIfNull(bindings);ArgumentNullException.ThrowIfNull(connector);
        var copy=bindings.Take(4097).ToArray();if(copy.Length is 0 or >4096)throw Error(DriverErrorKind.InvalidArgument,"Bind1..4096 mainframes");
        var routes=new Dictionary<(string,string),Core>();
        foreach(var binding in copy)
        {
            if(binding is null)throw Error(DriverErrorKind.InvalidArgument,"Null binding");
            EndpointValid(binding.Endpoint);var core=new Core(binding.Configuration);
            if(!routes.TryAdd(NativeMessages.Key(binding.Endpoint),core))throw Error(DriverErrorKind.InvalidArgument,"Duplicate endpoint");
        }
        _routes=routes.ToFrozenDictionary();_connector=connector;_native=nativeConnector;
    }
    public DriverDescriptor Descriptor=>new("keysight.34980a",DriverContract.Major,new[]{"Agilent Technologies","Keysight Technologies"}.Select(m=>new ExactIdentityRule("scpi",m,"34980A")),_routes.Values.SelectMany(c=>c.Operations));
    public IReadOnlySet<ExecutionMode> ExecutionModes=>NativeMessages.Modes(_native is not null);
    private Core Selected(DriverOpenContext context)
    {
        EndpointValid(context.Candidate.Endpoint);
        if(!Enum.IsDefined(context.Ownership))throw Error(DriverErrorKind.InvalidArgument,"Unknown ownership");
        if(!_routes.TryGetValue(NativeMessages.Key(context.Candidate.Endpoint),out var core))throw Error(DriverErrorKind.InvalidArgument,"Unbound mainframe");
        if(context.RequiredOperations is{} required&&!required.IsSubsetOf(core.Operations))throw Error(DriverErrorKind.UnsupportedOperation,"Required operation unavailable on selected mainframe");
        return core;
    }
    public IDeviceBackend Open(DriverOpenContext context)
    {var core=Selected(context);return new MessageDevice(core,_connector.Open(context),context.Ownership);}
    public async ValueTask<IAsyncDeviceBackend> OpenNativeAsync(DriverOpenContext context,CancellationToken cancellationToken=default)
    {
        cancellationToken.ThrowIfCancellationRequested();var core=Selected(context);var native=_native??throw Error(DriverErrorKind.UnsupportedExecutionMode,"Native connector not configured");
        return new AsyncMessageDevice(core,await native.OpenAsync(context,cancellationToken).ConfigureAwait(false),context.Ownership);
    }
    // This engine is immutable; health and lifetime belong to each opened MessageDevice.
    private sealed class Core:IMessageEngine
    {
        private readonly FrozenDictionary<uint,ushort> _switches;
        private readonly FrozenDictionary<uint,(ushort Address,Hp34980Direction Direction)> _ports;
        public IReadOnlySet<Operation> Operations{get;}
        private static DriverException Bad()=>Error(DriverErrorKind.InvalidArgument,"Require commissioned valid, unique slot/channel inventory");
        public Core(Hp34980Configuration config)
        {
            if(config is null||config.Commissioning!=Hp34980Commissioning.NoAutonomousActionsConfirmed||config.Switches is null||config.Ports is null)throw Bad();
            var switches=config.Switches.Take(561).ToArray();var ports=config.Ports.Take(65).ToArray();
            if(switches.Length>560||ports.Length>64||switches.Length+ports.Length==0)throw Bad();
            var paths=new Dictionary<uint,ushort>();var digital=new Dictionary<uint,(ushort,Hp34980Direction)>();var physical=new HashSet<ushort>();var slots=new Dictionary<byte,int>();
            void Slot(byte slot,int kind){if(slots.TryGetValue(slot,out var old)&&old!=kind)throw Bad();slots[slot]=kind;}
            foreach(var channel in switches)
            {
                if(channel is null||channel.Channel==0||channel.Slot is <1 or >8||!(channel.Module switch{Hp34980Module.Hp34937A=>channel.Local is >=1 and <=32,Hp34980Module.Hp34922A=>channel.Local is >=1 and <=70,_=>false}))throw Bad();
                Slot(channel.Slot,channel.Module==Hp34980Module.Hp34937A?1:2);
                var address=(ushort)(1000*channel.Slot+channel.Local);
                if(!physical.Add(address)||!paths.TryAdd(channel.Channel,address))throw Bad();
            }
            foreach(var port in ports)
            {
                if(port is null||port.Port==0||port.Slot is <1 or >8||port.Local is not(>=101 and <=104 or >=201 and <=204)||port.Direction is not(Hp34980Direction.Input or Hp34980Direction.Output))throw Bad();
                Slot(port.Slot,3);var address=(ushort)(1000*port.Slot+port.Local);
                if(!physical.Add(address)||!digital.TryAdd(port.Port,(address,port.Direction)))throw Bad();
            }
            _switches=paths.ToFrozenDictionary();_ports=digital.ToFrozenDictionary();var ops=new HashSet<Operation>();
            if(paths.Count>0)ops.UnionWith([Operation.CatalogSwitchCloseChannel,Operation.CatalogSwitchOpenChannel,Operation.CatalogSwitchChannelIsClosed]);
            foreach(var(_,direction)in digital.Values)ops.Add(direction==Hp34980Direction.Input?Operation.CatalogDigitalIoReadByte:Operation.CatalogDigitalIoWriteByte);
            Operations=ops.ToFrozenSet();
        }
        private static DriverException Parse()=>Error(DriverErrorKind.Parse,"Unexpected mainframe response");
        private static DriverException Device()=>Error(DriverErrorKind.Device,"Mainframe state violates commissioned byte policy");
        private static CatalogReply? Token(string body,string expected,params string[] known)
        {if(body==expected)return null;if(known.Contains(body,StringComparer.Ordinal))throw Device();throw Parse();}
        private static MessageStep Query(string command,ushort address,Func<string,CatalogReply?> decode)=>new(command+" (@"+address.ToString(CultureInfo.InvariantCulture)+")",decode);
        public IReadOnlyList<MessageStep> Prepare(CatalogRequest request)
        {
            var channel=request switch{SwitchCloseChannelRequest p=>p.Channel,SwitchOpenChannelRequest p=>p.Channel,SwitchChannelIsClosedRequest p=>p.Channel,_=>0u};
            if(request is SwitchCloseChannelRequest or SwitchOpenChannelRequest or SwitchChannelIsClosedRequest)
            {
                if(!_switches.TryGetValue(channel,out var address))throw Error(DriverErrorKind.InvalidArgument,"Unbound switch channel");
                var text=address.ToString(CultureInfo.InvariantCulture);
                return[request switch{SwitchCloseChannelRequest=>new MessageStep(":ROUT:CLOS (@"+text+")"),SwitchOpenChannelRequest=>new MessageStep(":ROUT:OPEN (@"+text+")"),_=>Query(":ROUT:CLOS?",address,b=>b switch{"0"=>new CatalogReply.State(false),"1"=>new CatalogReply.State(true),_=>throw Parse()})}];
            }
            if(request is not(DigitalIoReadByteRequest or DigitalIoWriteByteRequest))throw Error(DriverErrorKind.UnsupportedOperation,"Operation outside mainframe subset");
            var read=request is DigitalIoReadByteRequest;var port=request is DigitalIoReadByteRequest input?input.Port:((DigitalIoWriteByteRequest)request).Port;
            if(!_ports.TryGetValue(port,out var selected))throw Error(DriverErrorKind.InvalidArgument,"Unbound digital port");
            if(read!=(selected.Direction==Hp34980Direction.Input))throw Error(DriverErrorKind.InvalidArgument,"Operation unavailable for this frozen port direction");
            if(request is DigitalIoWriteByteRequest output&&output.Value>255)throw Error(DriverErrorKind.InvalidArgument,"Byte must be0..255");
            var a=selected.Address;var b=(ushort)(a/1000*1000+(a%1000<200?101:201));var steps=new List<MessageStep>();
            for(ushort p=b;p<b+4;p++)steps.Add(Query(":CONF:DIG:WIDT?",p,s=>Token(s,"1","2","4")));
            steps.AddRange([
                Query(":CONF:DIG:DIR?",a,s=>Token(s,read?"INP":"OUTP","INP","OUTP")),
                Query(":CONF:DIG:POL?",a,s=>Token(s,"NORM","INV")),
                Query(":CONF:DIG:HAND:STAT?",b,s=>Token(s,"HIMP","OFF","ON")),
                Query(":SENS:DIG:MEM:ENAB?",b,s=>Token(s,"0","1")),
                Query(":SOUR:DIG:MEM:ENAB?",b,s=>Token(s,"0","1"))]);
            if(read)
            {
                steps.Add(Query(":SENS:DIG:THR?",a,s=>{if(QualifiedScpiFactory.Bounded(s)!=2.5)throw Device();return null;}));
                steps.Add(new(":SENS:DIG:DATA:BYTE? DEC,(@"+a.ToString(CultureInfo.InvariantCulture)+")",s=>{
                    if(s.Any(c=>!char.IsAsciiDigit(c))||!uint.TryParse(s,NumberStyles.None,CultureInfo.InvariantCulture,out var value)||value>255)throw Parse();
                    return new CatalogReply.Count(value);
                }));
            }
            else
            {
                steps.Add(Query(":SOUR:DIG:DRIV?",a,s=>Token(s,"ACT","OCOL")));
                steps.Add(Query(":SOUR:DIG:LEV?",a,s=>{if(QualifiedScpiFactory.Bounded(s)!=3.3)throw Device();return null;}));
                var value=((DigitalIoWriteByteRequest)request).Value;
                steps.Add(new(":SOUR:DIG:DATA:BYTE "+value.ToString(CultureInfo.InvariantCulture)+",(@"+a.ToString(CultureInfo.InvariantCulture)+")"));
            }
            return steps;
        }
    }
}
