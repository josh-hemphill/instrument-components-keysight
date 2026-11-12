using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Drivers.Keysight;
using InstrumentComponents.Runtime;
using InstrumentComponents.TestKit;

internal static class HpLegacyProof
{
    private static void Require(bool value, string message) { if (!value) throw new InvalidOperationException(message); }
    private static void Error(Action action, DriverErrorKind kind = DriverErrorKind.InvalidArgument)
    {
        try { action(); } catch (DriverException e) { Require(e.Kind == kind, $"Expected {kind}, got {e.Kind}"); return; }
        throw new InvalidOperationException("Expected " + kind);
    }
    private static HpRadioConfiguration Config(HpRadioModel model) => new(model, model switch { HpRadioModel.Hp8920A => "A.18.00", HpRadioModel.Hp8920B => "B.06.00", _ => "A.15.00" }, HpRadioAttenuator.Electronic, HpRadioInputPower.Standard, HpRadioSourcePort.RfInOut, HpRadioReceiverInput.RfInOut, HpRadioCommissioning.Confirmed);
    internal static void Run()
    {
        foreach (var model in Enum.GetValues<HpRadioModel>())
        {
            var empty = new Script([]); var valid = Config(model);
            HpRadioFactory Make(HpRadioConfiguration c) => new(model, [new(new("hpib", "fixture"), c)], empty);
            foreach (var f in new[] { "", "a.99.00", "A.9.00", "A.99.00 ", "A.+9.00", "A.99.0X", "C.99.00", "A.00.00", "B.00.00" }) Error(() => Make(valid with { Firmware = f }));
            Make(valid with { Firmware = model == HpRadioModel.Hp8920B ? "B.99.99" : "A.99.99" });
            Error(() => Make(valid with { Commissioning = HpRadioCommissioning.Unknown }));
            Error(() => Make(valid with { Attenuator = HpRadioAttenuator.Unknown }));
            Error(() => Make(valid with { SourcePort = (HpRadioSourcePort)99 }));
            Error(() => Make(valid with { ReceiverInput = (HpRadioReceiverInput)99 }));
            Error(() => Make(valid with { Model = model == HpRadioModel.Hp8920A ? HpRadioModel.Hp8920B : HpRadioModel.Hp8920A }));
            foreach (var option in new[] { HpRadioInputPower.Unknown, HpRadioInputPower.Option006, HpRadioInputPower.Option007, HpRadioInputPower.Option008, HpRadioInputPower.Option016 })
            {
                var allowed = model switch { HpRadioModel.Hp8920A => option is HpRadioInputPower.Option007 or HpRadioInputPower.Option008 or HpRadioInputPower.Option016,
                    HpRadioModel.Hp8920B => option is HpRadioInputPower.Option006 or HpRadioInputPower.Option007 or HpRadioInputPower.Option016, _ => false };
                if (allowed) Make(valid with { InputPower = option }); else Error(() => Make(valid with { InputPower = option }));
            }
            if (model == HpRadioModel.Hp8920B) foreach (var option in new[] { HpRadioInputPower.Option006, HpRadioInputPower.Option007 }) Error(() => Make(valid with { Attenuator = HpRadioAttenuator.Mechanical055, InputPower = option }));
            var duplicate = new HpRadioBinding(new("hpib", "fixture"), valid);
            Error(() => new HpRadioFactory(model, [duplicate, duplicate], empty));
            var list = new List<HpRadioBinding> { duplicate, new(new("hpib", "ant"), valid with { ReceiverInput = HpRadioReceiverInput.Antenna }) };
            var factory = new HpRadioFactory(model, list, empty); list.Clear();
            var registry = new DriverRegistry([factory]); var selection = new DriverSelection(DriverId: factory.DriverId);
            Error(() => registry.Open(new(new("hpib", "ant"), new()), selection, required: new HashSet<Operation> { Operation.CatalogRfReceiverMeasurePower }), DriverErrorKind.UnsupportedOperation);
            Error(() => registry.Open(new(new("hpib", "not-bound"), new()), selection));
            Require(empty.Opens == 0, "RF pre-open route and capability checks");
            var candidate = new DeviceCandidate(new("hpib", "fixture"), new("Hewlett-Packard", factory.Model));
            Require(registry.Select(candidate).DriverId == factory.DriverId, "Passive own-model identity");
            var borrowed = registry.Open(candidate, selection, ownership: ConnectionOwnership.Borrowed);
            borrowed.Close(); borrowed.Close(); borrowed.Dispose(); Require(empty.Closes == 0, "RF borrowed IO preserved"); empty.AssertComplete();
        }
        var idle = new Script([]);
        var inputs = new List<Hp3497Input> { new(7, 0, 0, Hp3497Module.Option010, Hp3497Compensation.None) };
        var baseline = new Hp3497Configuration(true, true, inputs);
        Hp3497Factory Daq(Hp3497Configuration c) => new([new(new("hpib", "fixture"), c)], idle);
        foreach (var input in new[] { new Hp3497Input(7, 0, 0, Hp3497Module.Unknown, Hp3497Compensation.None), new(7, 0, 19, Hp3497Module.Option020, Hp3497Compensation.Software),
            new(7, 0, 0, Hp3497Module.Option010, Hp3497Compensation.Hardware), new(7, 0, 0, Hp3497Module.Option020, Hp3497Compensation.None), new(7, 5, 0, Hp3497Module.Option010, Hp3497Compensation.None), new(0, 0, 0, Hp3497Module.Option010, Hp3497Compensation.None) }) Error(() => Daq(baseline with { Inputs = [input] }));
        Error(() => Daq(baseline with { Option001 = false })); Error(() => Daq(baseline with { NormalDvmRoutingConfirmed = false })); Error(() => Daq(baseline with { Inputs = [inputs[0], inputs[0]] }));
        var frozen = Daq(baseline); inputs.Clear();
        using (var borrowed = new DriverRegistry([frozen]).Open(new(new("hpib", "fixture"), new()), new(DriverId: "keysight.hp3497a"), ownership: ConnectionOwnership.Borrowed))
            Error(() => borrowed.Invoke(new DataAcquisitionMeasureVoltageDcRequest(99, null)));
        Require(idle.Opens == 1 && idle.Closes == 0, "DAQ borrowed resource preserved"); idle.AssertComplete();
        foreach (var id in new[] { "keysight.hp3497a|D|proof", "keysight.hp8920a|R:E:Standard:RfInOut:RfInOut|proof", "keysight.hp8920b|R:E:Standard:RfInOut:RfInOut|proof", "keysight.hp8921a|R:E:Standard:RfInOut:RfInOut|proof" })
        {
            var daq = id.Contains("3497", StringComparison.Ordinal);
            foreach (var failure in new[] { DriverErrorKind.Timeout, DriverErrorKind.Transport, DriverErrorKind.Parse, DriverErrorKind.Device })
            {
                if (daq && failure == DriverErrorKind.Device) continue;
                var steps = new List<ScriptStep>();
                if (daq) steps.Add(new("query", "VT4AE0VC0VS0VF1VN1VW0SO1VR5AI0", failure == DriverErrorKind.Parse ? "bad" : "", failure == DriverErrorKind.Parse ? null : failure));
                else if (failure is DriverErrorKind.Parse or DriverErrorKind.Device) steps.AddRange([new("write", "DISP RFG"), new("write", "RFG:AMPL:STAT OFF"), new("query", "RFG:AMPL:STAT?", failure == DriverErrorKind.Device ? "1" : "bad")]);
                else steps.Add(new("write", "DISP RFG", Failure: failure));
                steps.Add(new("close")); var script = new Script(steps); var factory = HpFixture.Factory(id, script, false)!;
                var handle = new DriverRegistry([factory]).Open(new(new("hpib", "fixture"), new()), new(DriverId: id.Split('|')[0]));
                CatalogRequest request = daq ? new DataAcquisitionMeasureVoltageDcRequest(7, null) : new RfReceiverMeasureFrequencyRequest();
                Error(() => handle.Invoke(request), failure); Error(() => handle.Invoke(request), DriverErrorKind.SessionNeedsRecovery); Error(handle.Recover, DriverErrorKind.SessionNeedsRecovery);
                handle.Close(); handle.Close(); script.AssertComplete(); Require(script.Closes == 1, "Owned fail-fast close once");
            }
        }
        Console.WriteLine("PASS HP3497A/8920A/8920B/8921A procedural inventory, firmware, route capabilities, immutable configuration, ownership and fail-fast quarantine");
    }
}
