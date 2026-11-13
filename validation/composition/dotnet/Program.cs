using InstrumentComponents.Catalog;
using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Runtime;
using InstrumentComponents.TestKit;
using Identity=InstrumentComponents.Drivers.DeviceIdentity;
await Composition.Run();
await CompositionProof.Keysight.Proof.Run();await CompositionProof.Keithley.Proof.Run();await CompositionProof.Rigol.Proof.Run();await CompositionProof.Siglent.Proof.Run();await CompositionProof.Tektronix.Proof.Run();
internal static class Composition{
 private static IDriverFactory[] Factories(Script io)=>[
    ..InstrumentComponents.Drivers.Keysight.Drivers.Factories(io,nativeConnector:io),
    ..InstrumentComponents.Drivers.Keithley.Drivers.Factories(io,nativeConnector:io),
    ..InstrumentComponents.Drivers.Rigol.Drivers.Factories(io,nativeConnector:io),
    ..InstrumentComponents.Drivers.Siglent.Drivers.Factories(io,nativeConnector:io),
    ..InstrumentComponents.Drivers.Tektronix.Drivers.Factories(io,nativeConnector:io),
    ..CompositionProof.Keysight.ComposedRegistration.Factories(io),
    CompositionProof.Tektronix.TdsFixture.Factory(io,true)];
 private static DeviceCandidate Candidate(Identity identity)=>new(new("scpi","fixture"),identity);
 internal static async Task Run(){
    var empty=new Script([]);var factories=Factories(empty);var registry=new DriverRegistry(factories);var hints=0;var usb=0;
    foreach(var f in factories)foreach(var rule in f.Descriptor.Rules){Identity identity;
        if(rule is ExactIdentityRule idn){hints++;identity=new(idn.Manufacturer,idn.Model);}else if(rule is ExactProductRule product){usb++;identity=new(VendorId:product.VendorId,ProductId:product.ProductId);}else throw new InvalidOperationException("Unexpected hint");
        Require(registry.Select(new DeviceCandidate(new(rule.Scheme,"fixture"),identity)).DriverId==f.Descriptor.Id,"Identity selection");
    }
    Require(hints>=47,"Identity coverage");
    Console.WriteLine($"PASS actual registration evidence: {hints} identity rules, {usb} product rules; explicit device factories checked below");
    foreach(var id in new[]{"keysight.34401a","keysight.33120a","keysight.53132a","keysight.hp3585b","keysight.34980a","keysight.hp5334a","keysight.hp5334b","keysight.hp3497a","keysight.hp8920a","keysight.hp8920b","keysight.hp8921a","tektronix.tds1002"})
        Require(factories.Any(f=>f.Descriptor.Id==id),"Missing composed registration "+id);
    // Synthetic metadata-only identity rule, with no hardware USB mapping claim.
    var template=factories[0].Descriptor;
    registry=new DriverRegistry([..factories,new Alias(new("synthetic.usb",template.ContractMajor,[new ExactProductRule("scpi",0x1234,0x5678)],template.Operations))]);
    var same=new Identity(null,null,VendorId:0x1234,ProductId:0x5678);
    Require(registry.Select(Candidate(same)).DriverId=="synthetic.usb","Synthetic USB identity");
    var conflict=Candidate(same with{Manufacturer="Keithley Instruments",Model="2602B"});
    ExpectAmbiguous(()=>registry.Select(conflict));
    var preferences=new Dictionary<string,int>{{"keithley.2602b",7}};
    var preferred=registry.Select(conflict,new(Preferences:preferences));Require(preferred.DriverId=="keithley.2602b"&&!preferred.Explicit,"Composed preference");
    var forced=registry.Select(conflict,new(DriverId:"synthetic.usb",Preferences:preferences));Require(forced.DriverId=="synthetic.usb"&&forced.Explicit,"Explicit override must take precedence over preferences");
    try{registry.Select(conflict,new(DriverId:"synthetic.not-registered"));throw new InvalidOperationException("Unknown explicit driver accepted");}catch(DriverException error)when(error.Kind==DriverErrorKind.DriverNotFound){}
    var original=factories[0].Descriptor;var rule0=(ExactIdentityRule)original.Rules[0];
    var duplicate=new DriverRegistry([new Alias(new("synthetic.equal-a",original.ContractMajor,original.Rules,original.Operations)),new Alias(new("synthetic.equal-b",original.ContractMajor,original.Rules,original.Operations))]);
    ExpectAmbiguous(()=>duplicate.Select(Candidate(new(rule0.Manufacturer,rule0.Model))));Require(empty.Opens==0,"Passive matching used IO");
    for(var mode=0;mode<4;mode++){
        var script=new Script([ScriptStep.Query(":MEAS:VOLT:DC?","1.25"),ScriptStep.Query(":MEAS:VOLT? 1","2.5"),ScriptStep.Close()]);
        var candidate=Candidate(new("Keithley Instruments","2602B"));var catalog=new DeviceCatalog(new DriverRegistry(Factories(script)),[candidate]);var device=catalog.Device(candidate.Endpoint);
        var required=new HashSet<Operation>{Operation.CatalogDmmMeasureVoltageDc,Operation.CatalogDcPowerSupplyReadVoltage};
        if(mode==0){using var h=device.Open(required:required);Require(new Dmm(h).MeasureVoltageDc(null).Value==1.25&&new DcPowerSupply(h).ReadVoltage(1).Value==2.5,"Multiview values");h.Close();}
        else if(mode==1){var h=await device.OpenNativeAsync(required:required);Require((await new AsyncDmm(h).MeasureVoltageDcAsync(null)).Value==1.25&&(await new AsyncDcPowerSupply(h).ReadVoltageAsync(1)).Value==2.5,"Native multiview");await h.CloseAsync();}
        else{var h=await device.OpenWorkerAsync(required:required);if(mode==2)Require((await new AsyncDmm(h).MeasureVoltageDcAsync(null)).Value==1.25&&(await new AsyncDcPowerSupply(h).ReadVoltageAsync(1)).Value==2.5,"Worker multiview");else Require(new Dmm(h).MeasureVoltageDc(null).Value==1.25&&new DcPowerSupply(h).ReadVoltage(1).Value==2.5,"Blocking worker multiview");await h.CloseAsync();script.AssertThreadAffinity();}
        script.AssertComplete();Require(script.Opens==1&&script.Closes==1,"Multifunction backend lifetime");
    }
    Console.WriteLine("PASS all-five static Native AOT app:current models and synthetic USB, combined/conflicting hints, equal rank,2602B multiview and four frontends");
 }
 private sealed class Alias(DriverDescriptor descriptor):IDriverFactory{public DriverDescriptor Descriptor=>descriptor;}
 private static void Require(bool value,string message){if(!value)throw new InvalidOperationException(message);}
 private static void ExpectAmbiguous(Action call){try{call();throw new InvalidOperationException("Expected ambiguity");}catch(DriverException e)when(e.Kind==DriverErrorKind.AmbiguousDriver){}}
}
