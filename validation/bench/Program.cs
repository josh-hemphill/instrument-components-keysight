using InstrumentComponents.Errors;
using InstrumentComponents.Transport;
using System.Diagnostics;
using System.Security.Cryptography;
using System.Text.Json;
using System.Text.Json.Serialization;
using InstrumentComponents.Address;
using InstrumentComponents.Connect;
using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Runtime;
using InstrumentComponents.Scpi;
using InstrumentComponents.Session;
using InstrumentComponents.Visa;
using CatalogIdentity=InstrumentComponents.Drivers.Catalog.DeviceIdentity;
return Bench.Run(args);
internal static class Bench{
 internal static IDriverFactory[] Factories(IScpiConnector io)=>[
  ..InstrumentComponents.Drivers.Keysight.Drivers.Factories(io),..InstrumentComponents.Drivers.Keithley.Drivers.Factories(io),..InstrumentComponents.Drivers.Rigol.Drivers.Factories(io),..InstrumentComponents.Drivers.Siglent.Drivers.Factories(io),..InstrumentComponents.Drivers.Tektronix.Drivers.Factories(io)];
 private static string? Arg(string[] args,string name){var i=Array.IndexOf(args,name);return i>=0&&i+1<args.Length?args[i+1]:null;}
 internal static int Run(string[] args){
  if(args.Contains("--self-test"))return SelfTest();
  if(args.Length==0||args.Contains("--list")){
   foreach(var f in Factories(new NoIo()))Console.WriteLine(f.Descriptor.Id+": "+string.Join(",",f.Descriptor.Operations));return 0;
  }
  var configPath=Arg(args,"--config")??throw new ArgumentException("Provide a reviewed --config file; --list never accesses hardware");
  var text=File.ReadAllText(configPath);var config=JsonSerializer.Deserialize(text,BenchJson.Default.BenchConfig)??throw new ArgumentException("Empty configuration");
  var requests=config.Steps.Select(Request).ToArray();foreach(var request in requests)request.Validate();Validate(config);
  var candidate=new DeviceCandidate(new("scpi",config.Resource),new());var selection=new DriverSelection(DriverId:config.DriverId);
  var factories=Factories(new NoIo());var passive=new DriverRegistry(factories);var selected=passive.Select(candidate,selection);
  var operations=factories.Single(f=>f.Descriptor.Id==selected.DriverId).Descriptor.Operations;
  if(requests.Any(r=>!operations.Contains(r.Operation.Runtime())))throw new ArgumentException("Selected source profile lacks a requested operation");
  if(args.Contains("--validate-config")){Console.WriteLine("PASS configuration and static driver selection; no IO");return 0;}
  if(!args.Contains("--hardware")||!config.ApprovedForHardware)throw new ArgumentException("Physical execution requires --hardware and ApprovedForHardware in the reviewed configuration");
  if(Environment.GetEnvironmentVariable("INSTRUMENT_RESOURCE") is {} explicitResource&&explicitResource!=config.Resource)throw new ArgumentException("Operator resource differs from reviewed configuration");
  if(string.IsNullOrWhiteSpace(config.ExpectedManufacturer)||string.IsNullOrWhiteSpace(config.ExpectedModel))throw new ArgumentException("Expected manufacturer and model are required before hardware");
  if(config.Resource.Contains('<')||string.IsNullOrWhiteSpace(config.Operator)||string.IsNullOrWhiteSpace(config.Wiring)||string.IsNullOrWhiteSpace(config.Reference)||string.IsNullOrWhiteSpace(config.VisaRuntimeVersion))throw new ArgumentException("Complete operator, wiring, reference and VISA runtime evidence before hardware");
  var record=Arg(args,"--record")??throw new ArgumentException("Provide --record evidence output");
  if(File.Exists(record))throw new ArgumentException("Use a new evidence filename; preserve earlier records");
  var evidence=new BenchEvidence{StartedUtc=DateTimeOffset.UtcNow,ConfigSha256=Hash(configPath),Config=config,Runtime=System.Runtime.InteropServices.RuntimeInformation.FrameworkDescription,Platform=System.Runtime.InteropServices.RuntimeInformation.OSDescription,ApplicationSha256=Hash(typeof(Bench).Assembly.Location),PackageLockSha256=File.Exists("validation/bench/packages.lock.json")?Hash("validation/bench/packages.lock.json"):throw new ArgumentException("Run locked restore and retain validation/bench/packages.lock.json")};
  foreach(var assembly in new[]{typeof(InstrumentComponents.Drivers.Keysight.Drivers).Assembly,typeof(InstrumentComponents.Drivers.Keithley.Drivers).Assembly,typeof(InstrumentComponents.Drivers.Rigol.Drivers).Assembly,typeof(InstrumentComponents.Drivers.Siglent.Drivers).Assembly,typeof(InstrumentComponents.Drivers.Tektronix.Drivers).Assembly,typeof(DriverContract).Assembly,typeof(ScpiSession).Assembly,typeof(DriverRegistry).Assembly,typeof(ScpiFactory).Assembly,typeof(VisaSessionOpener).Assembly})evidence.AssemblyHashes.Add(assembly.GetName().Name!,Hash(assembly.Location));
  var io=new BenchConnector(config.TimeoutMs,evidence.Traffic);DeviceHandle? handle=null;var watch=Stopwatch.StartNew();
  try{
   var registry=new DriverRegistry(Factories(io));var required=requests.Select(r=>r.Operation.Runtime()).Append(Operation.CatalogQueryIdentity).ToHashSet();
   handle=registry.Open(candidate,selection,required);evidence.Identity=handle.Invoke(new QueryIdentityRequest()).IntoIdentity();
   if(!StringComparer.OrdinalIgnoreCase.Equals(evidence.Identity.Manufacturer.Trim(),config.ExpectedManufacturer.Trim())||!StringComparer.OrdinalIgnoreCase.Equals(evidence.Identity.Model.Trim(),config.ExpectedModel.Trim())||(config.ExpectedSerial is {Length:>0} serial&&evidence.Identity.Serial!=serial))throw new DriverException(DriverErrorKind.Device,"Physical identity differs from reviewed configuration; no measurement issued");
   for(var i=0;i<requests.Length;i++)evidence.Results.Add(Evaluate(config.Steps[i],handle.Invoke(requests[i])));
  }catch(Exception error){evidence.Failure=error.GetType().Name+": "+error.Message;}
  finally{
   try{handle?.Close();}catch(Exception error){evidence.Failure=(evidence.Failure??"")+"; close: "+error.Message;}
   evidence.DurationMs=watch.ElapsedMilliseconds;evidence.Passed=evidence.Failure is null&&evidence.Results.Count==requests.Length&&evidence.Results.All(r=>r.Passed);
   Directory.CreateDirectory(Path.GetDirectoryName(Path.GetFullPath(record))!);File.WriteAllText(record,JsonSerializer.Serialize(evidence,BenchJson.Default.BenchEvidence));
  }
  Console.WriteLine(evidence.Passed?"PASS physical bench record":"FAIL physical bench record; inspect evidence");return evidence.Passed?0:1;
 }
 private static void Validate(BenchConfig c){
  if(c.Steps.Count==0||string.IsNullOrWhiteSpace(c.DriverId)||string.IsNullOrWhiteSpace(c.Resource)||c.TimeoutMs is <100 or >120000)throw new ArgumentException("Invalid bench configuration");
  foreach(var s in c.Steps){
   if(s.Operation=="PowerMeterConfigureMeasurement"){if(s.PowerUnit is not "Watt" and not "Dbm")throw new ArgumentException("Explicit power configuration unit required");}
   else if(s.Operation=="SwitchIsClosed"){if(s.ExpectedBoolean is null)throw new ArgumentException("Switch query requires ExpectedBoolean");}
   else if(!Enum.TryParse<Unit>(s.ExpectedUnit,out var physicalUnit)||!Enum.IsDefined(physicalUnit)||physicalUnit.ToString()!=s.ExpectedUnit||physicalUnit is Unit.Unknown or Unit.Temperature or Unit.ConfiguredDmm or Unit.ConfiguredPower||s.ExpectedBoolean is not null||!double.IsFinite(s.Minimum)||!double.IsFinite(s.Maximum)||s.Minimum>s.Maximum||s.ExpectedUnit is "" or "Unknown")throw new ArgumentException("Scalar acceptance needs finite ordered bounds and a known expected unit");
  }
 }
 private static CatalogRequest Request(BenchStep s)=>s.Operation switch{
  "DmmMeasureVoltageDc"=>new DmmMeasureVoltageDcRequest(null),"DcPowerSupplyReadVoltage"=>new DcPowerSupplyReadVoltageRequest(s.Channel),"FunctionGeneratorReadFrequency"=>new FunctionGeneratorReadFrequencyRequest(),"OscilloscopeMeasureVpp"=>new OscilloscopeMeasureVppRequest(s.Channel),"SwitchIsClosed"=>new SwitchIsClosedRequest(s.Channel,s.OtherChannel),"CounterMeasureFrequency"=>new CounterMeasureFrequencyRequest(),"PowerMeterRead"=>new PowerMeterReadRequest(),"PowerMeterConfigureMeasurement"=>new PowerMeterConfigureMeasurementRequest(Enum.Parse<PowerUnit>(s.PowerUnit),true,true,null,null),"SpectrumAnalyzerMarkerX"=>new SpectrumAnalyzerMarkerXRequest(),"SpectrumAnalyzerMarkerY"=>new SpectrumAnalyzerMarkerYRequest(),_=>throw new ArgumentException("Unsupported bench operation; extend and review explicit request mapping first")};
 private static BenchResult Evaluate(BenchStep step,CatalogReply reply){
  if(reply is CatalogReply.Empty){reply.IntoUnit();return new(){Operation=step.Operation,Passed=true};}
  if(reply is CatalogReply.State boolean)return new(){Operation=step.Operation,Boolean=boolean.Value,Passed=boolean.Value==step.ExpectedBoolean};
  var q=reply.IntoQuantity();return new(){Operation=step.Operation,Value=q.Value,Unit=q.Unit.ToString(),Status=q.Status.ToString(),Passed=q.Status==MeasurementStatus.Valid&&q.Unit.ToString()==step.ExpectedUnit&&double.IsFinite(q.Value)&&q.Value>=step.Minimum&&q.Value<=step.Maximum};
 }
 private static string Hash(string file)=>Convert.ToHexString(SHA256.HashData(File.ReadAllBytes(file))).ToLowerInvariant();
 private static int SelfTest(){
  CheckBoundedConnector();
  foreach(var unit in new[]{"999","1","ConfiguredDmm","ConfiguredPower","Unknown","Temperature","volt"}){
   try{Validate(new(){DriverId="fixture",Resource="fixture",Steps=[new(){Operation="DmmMeasureVoltageDc",ExpectedUnit=unit,Minimum=0,Maximum=2}]});throw new InvalidOperationException("Invalid physical unit accepted");}catch(ArgumentException){}
  }
  var io=new SelfConnector();var registry=new DriverRegistry(Factories(io));var candidate=new DeviceCandidate(new("scpi","self-test"),new());using var h=registry.Open(candidate,new(DriverId:"keysight.34410a"));
  var identity=h.Invoke(new QueryIdentityRequest()).IntoIdentity();var step=new BenchStep{Operation="DmmMeasureVoltageDc",ExpectedUnit="Volt",Minimum=1.2,Maximum=1.3};Validate(new(){DriverId="keysight.34410a",Resource="fixture",Steps=[step]});
  if(identity.Model!="34410A"||!Evaluate(step,h.Invoke(Request(step))).Passed)throw new InvalidOperationException("Bench evaluator");h.Close();if(io.Calls!=3)throw new InvalidOperationException("Unexpected bench traffic");Console.WriteLine("PASS bench executable self-test; no VISA session opened");return 0;}
 private static void CheckBoundedConnector(){
  var opener=new BoundedOpener();var connector=new BenchConnector(1000,[],opener);
  using var io=connector.Open(new(new(new("scpi","TCPIP0::192.0.2.1::inst0::INSTR"),new()),ConnectionOwnership.Owned));
  try{io.Query("READ?");throw new InvalidOperationException("Oversize bench response accepted");}
  catch(DriverException error)when(error.Kind==DriverErrorKind.Parse&&error.InnerException is ParseException){}
  if(opener.Options?.MaxResponseBytes!=ScpiNumbers.MaxResponseBytes||opener.Wire.Reads!=1)throw new InvalidOperationException("Bench message bound missing or oversized body read");
 }
 private sealed class BoundedOpener:ISessionOpener{
  internal readonly BoundedWire Wire=new();internal ConnectOptions? Options;
  public ITransport Open(ResourceAddress address,ConnectOptions options){Options=options;return Wire;}
 }
 private sealed class BoundedWire:TransportBase{
  internal int Reads;
  public override void Write(ReadOnlySpan<byte> data){}
  public override int Read(Span<byte> buffer){if(++Reads>1)throw new InvalidOperationException("Oversize body must not be read");var header="#71048577"u8;header.CopyTo(buffer);return header.Length;}
  public override void Clear(){}public override void SetReadTimeout(TimeSpan timeout){}
 }
 private sealed class NoIo:IScpiConnector{public IMessageIo Open(DriverOpenContext context)=>throw new InvalidOperationException("Validation must not open IO");}
 private sealed class SelfConnector:IScpiConnector,IMessageIo{
  internal int Calls;public IMessageIo Open(DriverOpenContext context)=>this;public void Write(string message)=>throw new InvalidOperationException("Unexpected write");
  public string Query(string message)=>++Calls switch{1 when message=="*IDN?"=>"Keysight Technologies,34410A,self-test,fixture",2 when message==":MEAS:VOLT:DC?"=>"1.25",_=>throw new InvalidOperationException("Unexpected query")};public void Dispose()=>Calls++;
 }
}
internal sealed class BenchConnector(int timeoutMs,List<BenchTraffic> traffic,ISessionOpener? opener=null):IScpiConnector{
 public IMessageIo Open(DriverOpenContext context){try{var timeout=TimeSpan.FromMilliseconds(timeoutMs);var options=new ConnectOptions{MaxResponseBytes=ScpiNumbers.MaxResponseBytes,OpenTimeout=timeout,ReadTimeout=timeout,WriteTimeout=timeout,PerOpTimeout=timeout,ResetOnConnect=false,Retries=0,ReconnectOnFailure=false};var transport=(opener??new VisaSessionOpener()).Open(ResourceAddress.Parse(context.Candidate.Endpoint.Address),options);return new Message(new ScpiSession(transport,options),traffic);}catch(InstrumentException error){throw Translate(error);}}
 private static DriverException Translate(InstrumentException error){
  var kind=error is InvalidAddressException?DriverErrorKind.InvalidArgument:error is ParseException?DriverErrorKind.Parse:DriverErrorKind.Transport;
  for(Exception? current=error;current is not null;current=current.InnerException)if(current is InstrumentTimeoutException or TimeoutException)kind=DriverErrorKind.Timeout;
  return new(kind,error.Message,error);
 }
 private sealed class Message(ScpiSession session,List<BenchTraffic> traffic):IMessageIo{
  public void Write(string message){traffic.Add(new(DateTimeOffset.UtcNow,"write",message,null));try{session.Write(message);}catch(InstrumentException error){throw Translate(error);}}
  public string Query(string message){traffic.Add(new(DateTimeOffset.UtcNow,"query",message,null));try{var result=session.Query(message);traffic.Add(new(DateTimeOffset.UtcNow,"reply",message,result));return result;}catch(InstrumentException error){throw Translate(error);}}
  public void Dispose()=>session.Dispose();
 }
}
internal sealed class BenchConfig{public bool ApprovedForHardware{get;set;}public string DriverId{get;set;}="";public string Resource{get;set;}="";public int TimeoutMs{get;set;}=5000;public string Operator{get;set;}="";public string Wiring{get;set;}="";public string Reference{get;set;}="";public string VisaRuntimeVersion{get;set;}="";public string ExpectedManufacturer{get;set;}="";public string ExpectedModel{get;set;}="";public string? ExpectedSerial{get;set;}public List<BenchStep> Steps{get;set;}=[];}
internal sealed class BenchStep{public string Operation{get;set;}="";public uint Channel{get;set;}=1;public uint OtherChannel{get;set;}=2;public string ExpectedUnit{get;set;}="";public string PowerUnit{get;set;}="";public double Minimum{get;set;}public double Maximum{get;set;}public bool? ExpectedBoolean{get;set;}}
internal sealed class BenchResult{public string Operation{get;set;}="";public double? Value{get;set;}public string? Unit{get;set;}public string? Status{get;set;}public bool? Boolean{get;set;}public bool Passed{get;set;}}
internal sealed record BenchTraffic(DateTimeOffset Timestamp,string Kind,string Command,string? Reply);
internal sealed class BenchEvidence{public DateTimeOffset StartedUtc{get;set;}public string ConfigSha256{get;set;}="";public BenchConfig Config{get;set;}=new();public string Runtime{get;set;}="";public string Platform{get;set;}="";public string ApplicationSha256{get;set;}="";public string PackageLockSha256{get;set;}="";public Dictionary<string,string> AssemblyHashes{get;set;}=[];public CatalogIdentity? Identity{get;set;}public List<BenchTraffic> Traffic{get;set;}=[];public List<BenchResult> Results{get;set;}=[];public string? Failure{get;set;}public long DurationMs{get;set;}public bool Passed{get;set;}}
[JsonSourceGenerationOptions(WriteIndented=true)]
[JsonSerializable(typeof(BenchConfig))][JsonSerializable(typeof(BenchEvidence))]
internal partial class BenchJson:JsonSerializerContext{}
