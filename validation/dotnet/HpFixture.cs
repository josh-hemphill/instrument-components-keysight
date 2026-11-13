using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Keysight;
using InstrumentComponents.TestKit;

internal static class HpFixture
{
    internal static IDriverFactory? Factory(string id, Script script, bool native)
    {
        var endpoint = new Endpoint("hpib", "fixture");
        if (id.StartsWith("keysight.hp3497a|", StringComparison.Ordinal)) return new Hp3497Factory([
            new(endpoint, new(true, true, [new(7,0,0,Hp3497Module.Option010,Hp3497Compensation.None), new(13,0,19,Hp3497Module.Option010,Hp3497Compensation.None),
                new(29,1,18,Hp3497Module.Option020,Hp3497Compensation.Software), new(61,4,19,Hp3497Module.Option010,Hp3497Compensation.None)]))], script, native ? script : null);
        var model = id.Split('|')[0] switch { "keysight.hp8920a" => HpRadioModel.Hp8920A, "keysight.hp8920b" => HpRadioModel.Hp8920B, "keysight.hp8921a" => HpRadioModel.Hp8921A, _ => (HpRadioModel?)null };
        if (model is null) return null;
        var f = id.Split('|')[1].Split(':');
        var config = new HpRadioConfiguration(model.Value, model switch { HpRadioModel.Hp8920A => "A.18.00", HpRadioModel.Hp8920B => "B.06.00", _ => "A.15.00" },
            f[1] switch { "E" => HpRadioAttenuator.Electronic, "M" => HpRadioAttenuator.Mechanical055, _ => throw new InvalidOperationException("Fixture attenuator") },
            f[2] switch { "Standard" => HpRadioInputPower.Standard, "Option006" => HpRadioInputPower.Option006, "Option007" => HpRadioInputPower.Option007, "Option008" => HpRadioInputPower.Option008, "Option016" => HpRadioInputPower.Option016, _ => throw new InvalidOperationException("Fixture option") },
            f[3] switch { "RfInOut" => HpRadioSourcePort.RfInOut, "DuplexOut" => HpRadioSourcePort.DuplexOut, _ => throw new InvalidOperationException("Fixture source") },
            f[4] switch { "RfInOut" => HpRadioReceiverInput.RfInOut, "Antenna" => HpRadioReceiverInput.Antenna, _ => throw new InvalidOperationException("Fixture receiver") }, HpRadioCommissioning.Confirmed);
        return new HpRadioFactory(model.Value, [new(endpoint, config)], script, native ? script : null);
    }
}
