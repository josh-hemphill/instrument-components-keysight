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
    ..InstrumentComponents.Drivers.Tektronix.Drivers.Factories(io,nativeConnector:io)];
 private static DeviceCandidate Candidate(Identity identity)=>new(new("scpi","fixture"),identity);
 internal static async Task Run(){
    var empty=new Script([]);var factories=Factories(empty);var registry=new DriverRegistry(factories);var hints=0;var usb=0;
    foreach(var f in factories)foreach(var rule in f.Descriptor.Rules){Identity identity;
        if(rule is ExactIdentityRule idn){hints++;identity=new(idn.Manufacturer,idn.Model);}else if(rule is ExactProductRule product){usb++;identity=new(VendorId:product.VendorId,ProductId:product.ProductId);}else throw new InvalidOperationException("Unexpected hint");
        Require(registry.Select(Candidate(identity)).DriverId==f.Descriptor.Id,"Identity selection");
    }
    Require(hints==47&&usb==1,"Hint coverage");
    var same=new Identity("Keysight Technologies","34401A",VendorId:0x0957,ProductId:0x0607);
    Require(registry.Select(Candidate(same)).DriverId=="keysight.34401a","Combined identity");
    ExpectAmbiguous(()=>registry.Select(Candidate(same with{Manufacturer="Keithley Instruments",Model="2602B"})));
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
    Console.WriteLine("PASS all-five static Native AOT app:47IDN+USB, combined/conflicting hints, equal rank,2602B multiview and four frontends");
 }
 private sealed class Alias(DriverDescriptor descriptor):IDriverFactory{public DriverDescriptor Descriptor=>descriptor;}
 private static void Require(bool value,string message){if(!value)throw new InvalidOperationException(message);}
 private static void ExpectAmbiguous(Action call){try{call();throw new InvalidOperationException("Expected ambiguity");}catch(DriverException e)when(e.Kind==DriverErrorKind.AmbiguousDriver){}}
}
