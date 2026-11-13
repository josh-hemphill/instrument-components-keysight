using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Drivers.Keysight;
using InstrumentComponents.Runtime;
using InstrumentComponents.TestKit;

internal static class HpAdmissionProof
{
    private static void Require(bool value, string message) { if (!value) throw new InvalidOperationException(message); }
    private static void Reject(Action action, DriverErrorKind kind = DriverErrorKind.InvalidArgument)
    {
        try { action(); } catch (DriverException e) { Require(e.Kind == kind, $"Expected {kind}, got {e.Kind}"); return; }
        throw new InvalidOperationException("Expected " + kind);
    }
    private static async Task RejectAsync(Func<Task> action, DriverErrorKind kind)
    {
        try { await action(); } catch (DriverException e) { Require(e.Kind == kind, $"Expected {kind}, got {e.Kind}"); return; }
        throw new InvalidOperationException("Expected " + kind);
    }
    private static HpRadioConfiguration Config(HpRadioModel model) => new(model, model switch { HpRadioModel.Hp8920A => "A.18.00", HpRadioModel.Hp8920B => "B.06.00", _ => "A.15.00" }, HpRadioAttenuator.Electronic, HpRadioInputPower.Standard, HpRadioSourcePort.RfInOut, HpRadioReceiverInput.RfInOut, HpRadioCommissioning.Confirmed);
    internal static async Task Run()
    {
        var idle = new Script([]); var input = new Hp3497Input(7,0,0,Hp3497Module.Option010,Hp3497Compensation.None); var baseline = new Hp3497Configuration(true,true,[input]);
        Hp3497Factory Daq(Hp3497Configuration c) => new([new(new("hpib","fixture"),c)],idle);
        Reject(() => Daq(baseline with { Inputs = [] }));
        Reject(() => Daq(baseline with { Inputs = [input,input with { Local=1 }] })); // isolated public ID duplicate
        Reject(() => Daq(baseline with { Inputs = [input,input with { Channel=8 }] })); // isolated native duplicate
        Reject(() => Daq(baseline with { Inputs = [input,new(8,0,1,Hp3497Module.Option020,Hp3497Compensation.Software)] }));
        foreach(var mapping in new[] { input with { Local=20 },input with { Compensation=Hp3497Compensation.Software },input with { Compensation=Hp3497Compensation.Unknown },input with { Module=Hp3497Module.Option020,Compensation=Hp3497Compensation.Hardware },input with { Module=Hp3497Module.Option020,Compensation=Hp3497Compensation.Unknown } }) Reject(() => Daq(baseline with { Inputs=[mapping] }));
        Reject(() => new Hp3497Factory([],idle));
        Reject(() => new Hp3497Factory(Enumerable.Range(0,4097).Select(i=>new Hp3497Binding(new("hpib","route"+i),baseline)),idle));
        _ = new Hp3497Factory(Enumerable.Range(0,4096).Select(i=>new Hp3497Binding(new("hpib","route"+i),baseline)),idle);
        Require(!Daq(baseline).ExecutionModes.Contains(ExecutionMode.NativeAsync) && idle.Opens==0,"DAQ native-mode truthfulness and no rejected acquisition");
        var script = new Script([new("query","VT4AE0VC0VS0VF1VN1VW0SO1VR5AI0","+1.25000E+0"),new("close")]);
        var caller = new List<Hp3497Input>{input}; var frozen = new Hp3497Factory([new(new("hpib","fixture"),baseline with { Inputs=caller })],script); caller.Clear();
        var handle = new DriverRegistry([frozen]).Open(new(new("hpib","fixture"),new()),new(DriverId:"keysight.hp3497a"));
        Require(new DataAcquisition(handle).MeasureVoltageDc(7,null).Value==1.25,"Valid channel survives caller mutation");handle.Close();script.AssertComplete();
        foreach(var model in Enum.GetValues<HpRadioModel>())
        {
            idle=new Script([]);var valid=Config(model);
            Reject(()=>new HpRadioFactory(model,[],idle));
            Reject(()=>new HpRadioFactory(model,Enumerable.Range(0,4097).Select(i=>new HpRadioBinding(new("hpib","route"+i),valid)),idle));
            _ = new HpRadioFactory(model,Enumerable.Range(0,4096).Select(i=>new HpRadioBinding(new("hpib","route"+i),valid)),idle);
            foreach(var c in new[]{valid with{SourcePort=HpRadioSourcePort.Unknown},valid with{ReceiverInput=HpRadioReceiverInput.Unknown},valid with{InputPower=HpRadioInputPower.Unknown}}) Reject(()=>new HpRadioFactory(model,[new(new("hpib","fixture"),c)],idle));
            Require(!new HpRadioFactory(model,[new(new("hpib","fixture"),valid)],idle).ExecutionModes.Contains(ExecutionMode.NativeAsync) && idle.Opens==0,"RF native-mode truthfulness and no rejected acquisition");
            script=new Script([new("write","DISP RFG"),new("write","RFG:AMPL:STAT OFF"),new("query","RFG:AMPL:STAT?","0"),new("close")]);
            var factory=new HpRadioFactory(model,[new(new("hpib","fixture"),valid)],script);var candidate=new DeviceCandidate(new("hpib","fixture"),new());var selection=new DriverSelection(DriverId:factory.DriverId);
            handle=new DriverRegistry([factory]).Open(candidate,selection);
            foreach(var v in new[]{double.NaN,double.PositiveInfinity,double.NegativeInfinity}) foreach(var pair in new[]{(v,-30d),(30_000_000d,v)})
            {
                Reject(()=>new RfSignalGenerator(handle).Configure(pair.Item1,pair.Item2));Require(script.Events.Count==0,"RF nonfinite rejection emits no IO");
            }
            new RfSignalGenerator(handle).OutputEnable(false);handle.Close();script.AssertComplete();
            script=new Script([]);factory=new HpRadioFactory(model,[new(new("hpib","fixture"),valid),new(new("hpib","ant"),valid with{ReceiverInput=HpRadioReceiverInput.Antenna})],script,script);
            var registry=new DriverRegistry([factory]);
            foreach(var route in new[]{("ant",true,DriverErrorKind.UnsupportedOperation),("missing",false,DriverErrorKind.InvalidArgument),("FIXTURE",false,DriverErrorKind.InvalidArgument)})
            {
                candidate=new(new("hpib",route.Item1),new());var required=route.Item2?new HashSet<Operation>{Operation.CatalogRfReceiverMeasurePower}:new HashSet<Operation>();
                await RejectAsync(async()=>{await registry.OpenNativeAsync(candidate,selection,required);},route.Item3);
                await RejectAsync(async()=>{await registry.OpenWorkerAsync(candidate,selection,required);},route.Item3);
                Require(script.Opens==0,"Selected RF route rejects before native/worker acquisition");
            }
            script.AssertComplete();
        }
        script=new Script([]);var daq=new Hp3497Factory([new(new("hpib","fixture"),baseline)],script,script);var daqRegistry=new DriverRegistry([daq]);
        await RejectAsync(async()=>{await daqRegistry.OpenNativeAsync(new(new("hpib","missing"),new()),new(DriverId:"keysight.hp3497a"));},DriverErrorKind.InvalidArgument);
        await RejectAsync(async()=>{await daqRegistry.OpenWorkerAsync(new(new("hpib","missing"),new()),new(DriverId:"keysight.hp3497a"));},DriverErrorKind.InvalidArgument);
        Require(script.Opens==0,"DAQ selected route rejects before native/worker acquisition");script.AssertComplete();
        Console.WriteLine("PASS HP isolated inventory invariants,4097 unique route bounds,native-mode absence,valid inventory after mutation,RF nonfinite usable follow-up and native/worker selected-route admission");
    }
}
