using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Runtime;
using InstrumentComponents.Scpi;
using InstrumentComponents.TestKit;
using InstrumentComponents.Drivers.Keysight;
using Vendor=InstrumentComponents.Drivers.Keysight.Drivers;
using Identity=InstrumentComponents.Drivers.DeviceIdentity;
await Proof.Run();
internal static class Proof {
    private static Dictionary<string,InventoryBinding> Bindings()=>Vendor.Profiles().ToDictionary(p=>p.Id,p=>{
        var psu=p.Operations.Contains(Operation.CatalogDcPowerSupplySetVoltage);var fgen=p.Operations.Contains(Operation.CatalogFunctionGeneratorOutputEnable);var outputs=new List<Output>();
        if(psu){outputs.Add(new(OutputKind.DcPowerSupply,1));outputs.Add(new(OutputKind.DcPowerSupply,3));}if(fgen)outputs.Add(new(OutputKind.FunctionGenerator));
        var sparse=p.Operations.Contains(Operation.CatalogOscilloscopeSetTriggerSource)||p.Operations.Contains(Operation.CatalogSwitchCloseRoute);
        return new InventoryBinding(psu||sparse?[1,3]:p.ChannelIds,outputs);
    });
    private static DeviceCandidate Candidate=>new(new("scpi","fixture"),new());
    public static async Task Run(){
        await HpCounterProof.Run();
        HpLegacyProof.Run();
        await HpAsyncProof.Run();
        await HpAdmissionProof.Run();
        var empty=new Script([]);var registration=Vendor.Factories(empty);var passive=new DriverRegistry(registration);
        foreach(var factory in registration)foreach(var rule in factory.Descriptor.Rules){
            var identity=rule switch {ExactIdentityRule r=>new Identity(r.Manufacturer,r.Model),ExactProductRule r=>new Identity(VendorId:r.VendorId,ProductId:r.ProductId),_=>throw new InvalidOperationException("Expected exact migrated hint")};
            if(passive.Select(Candidate with{Identity=identity}).DriverId!=factory.Descriptor.Id)throw new InvalidOperationException("Identity ambiguity");
        }
        if(empty.Opens!=0)throw new InvalidOperationException("Passive selection opened IO");
        for(var mode=0;mode<4;mode++)foreach(var test in GeneratedDrivers.GeneratedCases.Create()){
            var script=new Script(test.Script);var bindings=test.Id.Split('|')[1]=="U"?null:Bindings();
            var connections=bindings?.Select(p=>new InstrumentComponents.Drivers.Keysight.ConnectionBinding(p.Key,Candidate.Endpoint,p.Value)).ToArray();
            var hpModel=test.Id.StartsWith("keysight.hp5334a|",StringComparison.Ordinal)?Hp5334Model.Hp5334A:test.Id.StartsWith("keysight.hp5334b|",StringComparison.Ordinal)?Hp5334Model.Hp5334B:(Hp5334Model?)null;
            var candidate=test.Id.StartsWith("keysight.hp",StringComparison.Ordinal)?Candidate with{Endpoint=new("hpib","fixture")}:Candidate;
            var hpFactory=HpFixture.Factory(test.Id,script,mode==1);
            var factories=hpFactory is not null?new[]{hpFactory}:hpModel is {} model?new IDriverFactory[]{new Hp5334Factory(model,Hp5334Commissioning.NativeInternalGatingConfirmed,script,mode==1?script:null)}:mode==1?Vendor.NativeFactories(script,connections:connections):Vendor.Factories(script,connections:connections);
            var registry=new DriverRegistry(factories);var selection=new DriverSelection(DriverId:test.Id.Split('|')[0]);
            var sync=mode==0?registry.Open(candidate,selection):null;var native=mode==1?await registry.OpenNativeAsync(candidate,selection):null;var worker=mode>=2?await registry.OpenWorkerAsync(candidate,selection):null;
            for(var i=0;i<test.Calls.Count;i++){
                CatalogReply? actual=null;DriverException? failure=null;
                try{actual=mode switch{0=>sync!.Invoke(test.Calls[i]),1=>await native!.InvokeAsync(test.Calls[i]),2=>await worker!.InvokeAsync(test.Calls[i]),_=>worker!.Invoke(test.Calls[i])};}catch(DriverException error){failure=error;}
                if(i==test.Calls.Count-1 && test.ExpectedError is {} expected){if(failure?.Kind!=expected)throw new InvalidOperationException(test.Id+": wrong error "+failure?.Kind);}
                else{if(failure is not null)throw failure;if(i==test.Calls.Count-1 && !Equal(actual!,test.Expected!))throw new InvalidOperationException(test.Id+": wrong value/unit/status");}
            }
            if(sync is not null)sync.Close();else if(native is not null)await native.CloseAsync();else{await worker!.CloseAsync();script.AssertThreadAffinity();}
            DriverException? closed=null;try{if(mode==0)sync!.Invoke(new QueryIdentityRequest());else if(mode==1)await native!.InvokeAsync(new QueryIdentityRequest());else worker!.Invoke(new QueryIdentityRequest());}catch(DriverException error){closed=error;}
            if(closed?.Kind!=DriverErrorKind.SessionClosed)throw new InvalidOperationException(test.Id+": closed handle accepted operation");
            script.AssertComplete();if(script.Opens!=1 || script.Closes!=1)throw new InvalidOperationException("Resource lifetime");
        }
        Console.WriteLine($"PASS Keysight conformance: {GeneratedDrivers.GeneratedCases.Create().Count} independent cases x 4 frontends; exact traffic, values, units, errors and lifetime");
    }
    private static bool Equal(CatalogReply actual,CatalogReply expected)=>(actual,expected) switch{(CatalogReply.Trace a,CatalogReply.Trace b)=>a.Value.Unit==b.Value.Unit && a.Value.SampleIntervalS==b.Value.SampleIntervalS && a.Value.Values.SequenceEqual(b.Value.Values),_=>actual==expected};
}
