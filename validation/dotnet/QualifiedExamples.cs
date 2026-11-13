// Compiled hardware-free composition examples. Replace Script with an
// application-supplied, commissioned complete-message connector on a bench.
using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Drivers.Keysight;
using InstrumentComponents.Runtime;
using InstrumentComponents.TestKit;
internal static class QualifiedExamples
{
    private static Endpoint Endpoint(string scheme="scpi")=>new(scheme,"fixture");
    private static DeviceHandle Open(IDriverFactory factory,string scheme="scpi")=>new DriverRegistry([factory]).Open(new(Endpoint(scheme),new()),new(DriverId:factory.Descriptor.Id));
    private static void Require(bool value){if(!value)throw new InvalidOperationException("Composition example result");}
    internal static void Run()
    {
        var script=new Script([ScriptStep.Query(":MEAS:VOLT:DC? DEF,DEF","+1.25000000E+00"),ScriptStep.Close()]);
        var factory=new QualifiedScpiFactory(QualifiedScpiModel.Agilent34401A,
            [new(Endpoint(),QualifiedScpiCommissioning.GpibScpiConfirmed)],script);
        using(var handle=Open(factory)) {Require(new Dmm(handle).MeasureVoltageDc(null).Value==1.25);handle.Close();}
        script.AssertComplete();

        // The 33120A has no output-off relay; setting changes are always live.
        script=new([ScriptStep.Query(":FUNC:SHAP?","SIN"),ScriptStep.Query(":OUTP:LOAD?","+5.000000E+01"),ScriptStep.Query(":VOLT:UNIT?","VPP"),
            ScriptStep.Query(":AM:STAT?","0"),ScriptStep.Query(":FM:STAT?","0"),ScriptStep.Query(":BM:STAT?","0"),ScriptStep.Query(":FSK:STAT?","0"),
            ScriptStep.Query(":SWE:STAT?","0"),ScriptStep.Query(":FREQ?","+1.000000E+03"),ScriptStep.Close()]);
        factory=new(QualifiedScpiModel.Agilent33120A,
            [new(Endpoint(),QualifiedScpiCommissioning.ContinuousSine50OhmVppOutputChangesAuthorized)],script);
        using(var handle=Open(factory)){Require(new FunctionGenerator(handle).ReadFrequency().Value==1000);handle.Close();}
        script.AssertComplete();

        script=new([ScriptStep.Write(":CONF:FREQ DEF,DEF,(@1)"),ScriptStep.Write(":SENS:FREQ:ARM:STAR:SOUR IMM"),
            ScriptStep.Write(":SENS:FREQ:ARM:STOP:SOUR TIM"),ScriptStep.Write(":SENS:FREQ:ARM:STOP:TIM 0.100"),
            ScriptStep.Write(":FORM ASC"),ScriptStep.Query(":READ?","+1.25000000000E+06"),ScriptStep.Close()]);
        factory=new(QualifiedScpiModel.Agilent53132A,[new(Endpoint(),QualifiedScpiCommissioning.GpibScpiConfirmed)],script);
        using(var handle=Open(factory))
        {
            var counter=new Counter(handle);counter.SelectChannel(1);counter.SetGateTime(.100);
            Require(counter.MeasureFrequency().Value==1250000);handle.Close();
        }
        script.AssertComplete();

        // One parent handle owns every module view. FormC open selects NC.
        script=new([ScriptStep.Write(":ROUT:CLOS (@1001)"),ScriptStep.Query(":ROUT:CLOS? (@2001)","0"),ScriptStep.Close()]);
        var mainframe=new Hp34980Factory([new(Endpoint(),new(Hp34980Commissioning.NoAutonomousActionsConfirmed,
            [new(7,1,1,Hp34980Module.Hp34937A),new(101,2,1,Hp34980Module.Hp34922A)],
            [new(13,3,104,Hp34980Direction.Output)]))],script);
        using(var handle=Open(mainframe))
        {
            var switches=new Switch(handle);switches.CloseChannel(7);Require(!switches.ChannelIsClosed(101));
            // new DigitalIo(handle).WriteByte(13,0) explicitly changes an output
            // after verifying the commissioned byte/electrical module state.
            handle.Close();
        }
        script.AssertComplete();

        // Qualified serial3008A00860+, native R12/+30dBm50ohm/liveA/dBm.
        // MarkerPeak selects stored data; it does not acquire a full new sweep.
        script=new([ScriptStep.Write("CF1000000HZ"),ScriptStep.Close()]);
        var analyzer=new Hp3585Factory([new(Endpoint("hpib"),Hp3585Commissioning.QualifiedNativeScalarConfirmed)],script);
        using(var handle=Open(analyzer,"hpib")){new SpectrumAnalyzer(handle).SetCenterFrequency(1000000);handle.Close();}
        script.AssertComplete();
        Console.WriteLine("PASS compiled Agilent composition examples: five factories, shared module parent, typed views");
    }
}
