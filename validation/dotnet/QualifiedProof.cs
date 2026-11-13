using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Drivers.Keysight;
using InstrumentComponents.Runtime;
using InstrumentComponents.TestKit;

internal static class QualifiedProof
{
    private static readonly string[] Ids=["keysight.34401a","keysight.33120a","keysight.53132a","keysight.hp3585b","keysight.34980a"];
    private static DeviceCandidate Candidate(string id)=>new(new(id==Ids[3]?"hpib":"scpi","fixture"),new());
    private static DriverSelection Selection(string id)=>new(DriverId:id.Split('|')[0]);
    private static void Require(bool value,string message){if(!value)throw new InvalidOperationException(message);}
    private static void Error(DriverErrorKind kind,Action action)
    {
        try{action();throw new InvalidOperationException("Missing expected "+kind);}
        catch(DriverException error){Require(error.Kind==kind,"Expected "+kind+", actual "+error.Kind);}
    }
    private static CatalogCase Good(string id)=>GeneratedDrivers.GeneratedCases.Create().First(c=>c.Id.Split('|')[0]==id&&c.Expected is not null);
    private static IDriverFactory Registration(int index,IEnumerable<Endpoint> endpoints,bool confirmed,Script script)
    {
        if(index<3)
        {
            var model=new[]{QualifiedScpiModel.Agilent34401A,QualifiedScpiModel.Agilent33120A,QualifiedScpiModel.Agilent53132A}[index];
            var commissioning=!confirmed?QualifiedScpiCommissioning.Unknown:index==1?QualifiedScpiCommissioning.ContinuousSine50OhmVppOutputChangesAuthorized:QualifiedScpiCommissioning.GpibScpiConfirmed;
            return new QualifiedScpiFactory(model,endpoints.Select(e=>new QualifiedScpiBinding(e,commissioning)),script);
        }
        if(index==3)return new Hp3585Factory(endpoints.Select(e=>new Hp3585Binding(e,confirmed?Hp3585Commissioning.QualifiedNativeScalarConfirmed:Hp3585Commissioning.Unknown)),script);
        return new Hp34980Factory(endpoints.Select(e=>new Hp34980Binding(e,new(confirmed?Hp34980Commissioning.NoAutonomousActionsConfirmed:Hp34980Commissioning.Unknown,[new(7,1,1,Hp34980Module.Hp34937A)],[]))),script);
    }
    private static void Admission()
    {
        for(var index=0;index<Ids.Length;index++)
        {
            var id=Ids[index];var script=new Script([]);var endpoint=Candidate(id).Endpoint;
            Error(DriverErrorKind.InvalidArgument,()=>Registration(index,[endpoint],false,script));
            Error(DriverErrorKind.InvalidArgument,()=>Registration(index,[],true,script));
            foreach(var scheme in new[]{"wrong","\u00a0"+endpoint.Scheme,endpoint.Scheme+"\u00a0","\u2003"+endpoint.Scheme+"\u2003"})Error(DriverErrorKind.InvalidArgument,()=>Registration(index,[endpoint with{Scheme=scheme}],true,script));
            Error(DriverErrorKind.InvalidArgument,()=>Registration(index,[endpoint with{Address=" \t"}],true,script));
            Error(DriverErrorKind.InvalidArgument,()=>Registration(index,[endpoint,endpoint with{Scheme=" \t"+endpoint.Scheme.ToUpperInvariant()+" "}],true,script));
            var endpoints=Enumerable.Range(0,4096).Select(i=>endpoint with{Address="route"+i}).ToArray();Registration(index,endpoints,true,script);
            Error(DriverErrorKind.InvalidArgument,()=>Registration(index,endpoints.Append(endpoint),true,script));
            var factory=Registration(index,[endpoint],true,script);var registry=new DriverRegistry([factory]);
            Require(!factory.ExecutionModes.Contains(ExecutionMode.NativeAsync),"Absent native mode advertised");
            foreach(var rule in factory.Descriptor.Rules)
            {
                var exact=rule as ExactIdentityRule??throw new InvalidOperationException("Unexpected non-exact identity");
                Require(registry.Select(new(endpoint with{Scheme=exact.Scheme},new(exact.Manufacturer,exact.Model))).DriverId==id,"Passive exact model/alias");
            }
            Error(DriverErrorKind.InvalidArgument,()=>registry.Open(Candidate(id) with{Endpoint=endpoint with{Address="unbound"}},Selection(id)));
            Error(DriverErrorKind.UnsupportedOperation,()=>registry.Open(Candidate(id),Selection(id),new HashSet<Operation>{Operation.CatalogShutdown}));
            Require(script.Opens==0&&script.Events.Count==0,"Rejected admission performed IO");
        }
    }
    private static IEnumerable<CatalogRequest> BadRequests(string id)
    {
        foreach(var value in new[]{double.NaN,double.PositiveInfinity,double.NegativeInfinity})
        {
            if(id==Ids[0])foreach(var request in new CatalogRequest[]{new DmmMeasureVoltageDcRequest(value),new DmmMeasureVoltageAcRequest(value),new DmmMeasureCurrentDcRequest(value),new DmmMeasureCurrentAcRequest(value),new DmmMeasureResistanceRequest(value),new DmmMeasureResistance2WireRequest(value),new DmmMeasureResistance4WireRequest(value)})yield return request;
            if(id==Ids[1])foreach(var request in new CatalogRequest[]{new FunctionGeneratorSetFrequencyRequest(value),new FunctionGeneratorSetAmplitudeRequest(value),new FunctionGeneratorSetOffsetRequest(value)})yield return request;
            if(id==Ids[2])yield return new CounterSetGateTimeRequest(value);
            if(id==Ids[3])foreach(var request in new CatalogRequest[]{new SpectrumAnalyzerSetCenterFrequencyRequest(value),new SpectrumAnalyzerSetSpanRequest(value),new SpectrumAnalyzerSetRbwRequest(value),new SpectrumAnalyzerSetVbwRequest(value),new SpectrumAnalyzerSetRefLevelRequest(value)})yield return request;
        }
        if(id==Ids[4]){yield return new DigitalIoWriteByteRequest(13,256);yield return new SwitchCloseChannelRequest(0);yield return new DigitalIoReadByteRequest(13);}
    }
    private static void InvokeCase(DeviceHandle handle,CatalogCase test)
    {
        for(var i=0;i<test.Calls.Count;i++){var actual=handle.Invoke(test.Calls[i]);if(i+1==test.Calls.Count)Require(Equals(actual,test.Expected),test.Id+" follow-up result");}
    }
    internal static void Run()
    {
        Admission();Inventory();FrozenInventoryAndCounterIntent();GeneratorFailurePositions();
        foreach(var id in Ids)
        {
            var test=Good(id);var script=new Script(test.Script);var factory=QualifiedFixture.Factory(id,script,false)!;
            var handle=new DriverRegistry([factory]).Open(Candidate(id),Selection(id));var events=script.Events.Count;
            foreach(var request in BadRequests(id)){Error(DriverErrorKind.InvalidArgument,()=>handle.Invoke(request));Require(script.Events.Count==events,"Structural rejection accessed IO");}
            InvokeCase(handle,test);handle.Close();script.AssertComplete();Require(script.Closes==1,"Owned lifetime");
        }
        var normals=Ids.ToDictionary(id=>id,Good);
        foreach(var test in GeneratedDrivers.GeneratedCases.Create().Where(c=>Ids.Contains(c.Id.Split('|')[0])&&c.ExpectedError is not null))
        {
            var id=test.Id.Split('|')[0];var normal=normals[id];var hadIo=test.Script.Any(s=>s.Kind!="close");
            var steps=test.Script.Where(s=>s.Kind!="close").Concat(hadIo?[new ScriptStep("close")]:normal.Script).ToArray();
            var script=new Script(steps);var factory=QualifiedFixture.Factory(id,script,false)!;var handle=new DriverRegistry([factory]).Open(Candidate(id),Selection(id));
            for(var i=0;i<test.Calls.Count;i++){if(i+1==test.Calls.Count)Error(test.ExpectedError!.Value,()=>handle.Invoke(test.Calls[i]));else handle.Invoke(test.Calls[i]);}
            if(hadIo)
            {
                var events=script.Events.Count;var followup=id==Ids[4]?new SwitchCloseChannelRequest(7):normal.Calls[0];
                Error(DriverErrorKind.SessionNeedsRecovery,()=>handle.Invoke(followup));Require(script.Events.Count==events,"Quarantine accessed IO");
            }
            else
            {
                if(id==Ids[2]){handle.Invoke(new CounterSelectChannelRequest(1));handle.Invoke(new CounterSetGateTimeRequest(.1));}
                InvokeCase(handle,normal);
            }
            handle.Close();script.AssertComplete();Require(script.Closes==1,"Failed owned lifetime");
        }
        foreach(var id in Ids)
        {
            var test=Good(id);var script=new Script(test.Script.Where(s=>s.Kind!="close"));var factory=QualifiedFixture.Factory(id,script,false)!;
            var handle=new DriverRegistry([factory]).Open(Candidate(id),Selection(id),ownership:ConnectionOwnership.Borrowed);
            InvokeCase(handle,test);handle.Close();handle.Close();Error(DriverErrorKind.SessionClosed,()=>handle.Invoke(test.Calls[0]));script.AssertComplete();Require(script.Closes==0,"Borrowed IO closed");
        }
        Console.WriteLine("PASS new Agilent factories admission/ASCII schemes,4096 bounds, nonfinite pre-IO usable follow-up, all golden error quarantine/health follow-ups and borrowed/owned lifetime");
    }
    private static void GeneratorFailurePositions()
    {
        var cases=GeneratedDrivers.GeneratedCases.Create();
        foreach(var operation in new[]{CatalogOperation.FunctionGeneratorSetAmplitude,CatalogOperation.FunctionGeneratorSetOffset})
        {
            var test=cases.First(c=>c.Id.StartsWith("keysight.33120a|",StringComparison.Ordinal)&&c.Expected is not null&&c.Calls.Count==1&&c.Calls[0].Operation==operation);
            for(var position=0;position<test.Script.Count-1;position++)foreach(var kind in new[]{DriverErrorKind.Transport,DriverErrorKind.Timeout})
            {
                var steps=test.Script.Take(position).Append(test.Script[position] with{Failure=kind}).Append(ScriptStep.Close());
                var script=new Script(steps);var factory=QualifiedFixture.Factory(Ids[1],script,false)!;
                var handle=new DriverRegistry([factory]).Open(Candidate(Ids[1]),Selection(Ids[1]));
                Error(kind,()=>handle.Invoke(test.Calls[0]));var events=script.Events.Count;
                Error(DriverErrorKind.SessionNeedsRecovery,()=>new FunctionGenerator(handle).ReadFrequency());
                Require(script.Events.Count==events,"Generator failure retried IO");handle.Close();script.AssertComplete();Require(script.Closes==1,"Generator failure lifetime");
            }
        }
    }
    private static void FrozenInventoryAndCounterIntent()
    {
        var cases=GeneratedDrivers.GeneratedCases.Create().ToDictionary(c=>c.Id,StringComparer.Ordinal);
        var reading=cases["keysight.34980a|E|34950a-read-7-0"];var script=new Script(reading.Script);
        var switches=new List<Hp34980SwitchChannel>{new(7,1,1,Hp34980Module.Hp34937A)};
        var ports=new List<Hp34980DigitalPort>{new(7,3,101,Hp34980Direction.Input)};
        var bindings=new List<Hp34980Binding>{new(Candidate(Ids[4]).Endpoint,new(Hp34980Commissioning.NoAutonomousActionsConfirmed,switches,ports))};
        var factory=new Hp34980Factory(bindings,script);switches.Clear();ports[0]=ports[0] with{Direction=Hp34980Direction.Output};bindings.Clear();
        var handle=new DriverRegistry([factory]).Open(Candidate(Ids[4]),Selection(Ids[4]));InvokeCase(handle,reading);handle.Close();script.AssertComplete();
        foreach(var index in new[]{0,1,2,3})
        {
            var id=Ids[index];var normal=Good(id);var host=new Script(normal.Script);IDriverFactory frozen;
            if(index<3)
            {
                var commissioning=index==1?QualifiedScpiCommissioning.ContinuousSine50OhmVppOutputChangesAuthorized:QualifiedScpiCommissioning.GpibScpiConfirmed;
                var mutable=new List<QualifiedScpiBinding>{new(Candidate(id).Endpoint,commissioning)};
                frozen=new QualifiedScpiFactory(new[]{QualifiedScpiModel.Agilent34401A,QualifiedScpiModel.Agilent33120A,QualifiedScpiModel.Agilent53132A}[index],mutable,host);mutable.Clear();
                Error(DriverErrorKind.InvalidArgument,()=>new QualifiedScpiFactory((QualifiedScpiModel)99,[new(Candidate(id).Endpoint,commissioning)],host));
                Error(DriverErrorKind.InvalidArgument,()=>new QualifiedScpiFactory(new[]{QualifiedScpiModel.Agilent34401A,QualifiedScpiModel.Agilent33120A,QualifiedScpiModel.Agilent53132A}[index],[new(Candidate(id).Endpoint,(QualifiedScpiCommissioning)99)],host));
            }
            else
            {
                var mutable=new List<Hp3585Binding>{new(Candidate(id).Endpoint,Hp3585Commissioning.QualifiedNativeScalarConfirmed)};frozen=new Hp3585Factory(mutable,host);mutable.Clear();
                Error(DriverErrorKind.InvalidArgument,()=>new Hp3585Factory([new(Candidate(id).Endpoint,(Hp3585Commissioning)99)],host));
            }
            Error(DriverErrorKind.InvalidArgument,()=>frozen.Open(new(Candidate(id),(ConnectionOwnership)99)));
            Require(host.Opens==0,"Invalid ownership acquired a resource");var opened=new DriverRegistry([frozen]).Open(Candidate(id),Selection(id));InvokeCase(opened,normal);opened.Close();host.AssertComplete();
        }
        var standard=cases["keysight.53132a|E|Frequency-channel-1"];var intended=cases["keysight.53132a|E|switch-intent-and-repeat"];
        var both=new Script(standard.Script.Concat(intended.Script));var counter=QualifiedFixture.Factory(Ids[2],both,false)!;var registry=new DriverRegistry([counter]);
        var first=registry.Open(Candidate(Ids[2]),Selection(Ids[2]));first.Invoke(intended.Calls[0]);first.Invoke(intended.Calls[1]);
        foreach(var invalid in new CatalogRequest[]{new CounterSelectChannelRequest(3),new CounterSetGateTimeRequest(double.NaN),new CounterSetGateTimeRequest(.010000000000000002)})Error(DriverErrorKind.InvalidArgument,()=>first.Invoke(invalid));
        var second=registry.Open(Candidate(Ids[2]),Selection(Ids[2]));InvokeCase(second,standard);second.Close();
        foreach(var request in intended.Calls.Skip(2))first.Invoke(request);first.Close();both.AssertComplete();Require(both.Opens==2&&both.Closes==2,"Independent counter handles");
    }
    internal static async Task SelectedRouteAdmission()
    {
        var script=new Script([]);
        var factory=new Hp34980Factory([
            new(new("scpi","relay"),new(Hp34980Commissioning.NoAutonomousActionsConfirmed,[new(7,1,1,Hp34980Module.Hp34937A)],[])),
            new(new("scpi","input"),new(Hp34980Commissioning.NoAutonomousActionsConfirmed,[],[new(7,3,101,Hp34980Direction.Input)])),
            new(new("scpi","output"),new(Hp34980Commissioning.NoAutonomousActionsConfirmed,[],[new(13,3,104,Hp34980Direction.Output)]))],script,script);
        var registry=new DriverRegistry([factory]);
        foreach(var(address,operation)in new[]{("relay",Operation.CatalogDigitalIoReadByte),("relay",Operation.CatalogDigitalIoWriteByte),("input",Operation.CatalogDigitalIoWriteByte),("input",Operation.CatalogSwitchCloseChannel),("output",Operation.CatalogDigitalIoReadByte),("output",Operation.CatalogSwitchCloseChannel)})
        {
            var candidate=new DeviceCandidate(new("scpi",address),new());var required=new[]{operation};
            Error(DriverErrorKind.UnsupportedOperation,()=>registry.Open(candidate,Selection(Ids[4]),required));
            try{await registry.OpenNativeAsync(candidate,Selection(Ids[4]),required);throw new InvalidOperationException("Required endpoint capability accepted");}catch(DriverException error){Require(error.Kind==DriverErrorKind.UnsupportedOperation,"Native selected-route error");}
            try{await registry.OpenWorkerAsync(candidate,Selection(Ids[4]),required);throw new InvalidOperationException("Required endpoint capability accepted");}catch(DriverException error){Require(error.Kind==DriverErrorKind.UnsupportedOperation,"Worker selected-route error");}
        }
        Error(DriverErrorKind.InvalidArgument,()=>factory.Open(new(new(new("scpi","relay"),new()),(ConnectionOwnership)99)));
        foreach(var id in Ids)
        {
            var f=QualifiedFixture.Factory(id,script,false)!;var registered=new DriverRegistry([f]);
            try{await registered.OpenNativeAsync(Candidate(id),Selection(id));throw new InvalidOperationException("Absent native mode accepted");}catch(DriverException error){Require(error.Kind==DriverErrorKind.UnsupportedExecutionMode,"Absent native mode error");}
        }
        Require(script.Opens==0&&script.Events.Count==0,"Selected route acquired a resource");
        Console.WriteLine("PASS selected mainframe endpoint capabilities before sync/native/worker acquisition; native absence; immutable inventory and isolated counter intent");
    }
    private static void Inventory()
    {
        var script=new Script([]);var relay=new Hp34980SwitchChannel(7,1,1,Hp34980Module.Hp34937A);var port=new Hp34980DigitalPort(7,3,101,Hp34980Direction.Input);
        Hp34980Factory Build(IReadOnlyList<Hp34980SwitchChannel> switches,IReadOnlyList<Hp34980DigitalPort> ports,Hp34980Commissioning commission=Hp34980Commissioning.NoAutonomousActionsConfirmed)=>new([new(Candidate(Ids[4]).Endpoint,new(commission,switches,ports))],script);
        for(byte slot=1;slot<=8;slot++)
        {
            foreach(var(module,max)in new[]{(Hp34980Module.Hp34937A,32),(Hp34980Module.Hp34922A,70)})for(ushort local=1;local<=max;local++)Build([relay with{Slot=slot,Local=local,Module=module}],[]);
            foreach(ushort local in new ushort[]{101,102,103,104,201,202,203,204})foreach(var direction in new[]{Hp34980Direction.Input,Hp34980Direction.Output})Build([],[port with{Slot=slot,Local=local,Direction=direction}]);
        }
        foreach(var bad in new[]{relay with{Slot=0},relay with{Slot=9},relay with{Channel=0},relay with{Local=0},relay with{Local=33},relay with{Module=Hp34980Module.Unknown},relay with{Module=(Hp34980Module)99},relay with{Module=Hp34980Module.Hp34922A,Local=71}})Error(DriverErrorKind.InvalidArgument,()=>Build([bad],[]));
        foreach(var bad in new[]{port with{Slot=0},port with{Slot=9},port with{Port=0},port with{Local=100},port with{Local=105},port with{Local=200},port with{Local=205},port with{Direction=Hp34980Direction.Unknown},port with{Direction=(Hp34980Direction)99}})Error(DriverErrorKind.InvalidArgument,()=>Build([],[bad]));
        foreach(var entries in new[]{new[]{relay,relay with{Channel=7,Local=2}},new[]{relay,relay with{Channel=13}},new[]{relay,relay with{Channel=13,Local=2,Module=Hp34980Module.Hp34922A}}})Error(DriverErrorKind.InvalidArgument,()=>Build(entries,[]));
        foreach(var entries in new[]{new[]{port,port with{Port=7,Local=102}},new[]{port,port with{Port=13}}})Error(DriverErrorKind.InvalidArgument,()=>Build([],entries));
        Error(DriverErrorKind.InvalidArgument,()=>Build([relay],[port with{Slot=1}]));Error(DriverErrorKind.InvalidArgument,()=>Build([relay],[],Hp34980Commissioning.Unknown));Error(DriverErrorKind.InvalidArgument,()=>Build([relay],[],(Hp34980Commissioning)99));Error(DriverErrorKind.InvalidArgument,()=>Build([],[]));Require(script.Opens==0,"Inventory validation accessed IO");
    }
}
