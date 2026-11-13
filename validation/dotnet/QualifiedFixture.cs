using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Keysight;
using InstrumentComponents.TestKit;
using InstrumentComponents.Scpi;
internal static class QualifiedFixture
{
    internal static IDriverFactory? Factory(string id,Script script,bool native) => Factory(id,(IScpiConnector)script,native?script:null);
    internal static IDriverFactory? Factory(string id,IScpiConnector script,IAsyncScpiConnector? native)
    {
        if(id.Split('|')[0]=="keysight.hp3585b")return new Hp3585Factory([new(new("hpib","fixture"),Hp3585Commissioning.QualifiedNativeScalarConfirmed)],script,native);
        if(id.Split('|')[0]=="keysight.34980a")return new Hp34980Factory([new(new("scpi","fixture"),new(Hp34980Commissioning.NoAutonomousActionsConfirmed,[new(7,1,1,Hp34980Module.Hp34937A),
new(13,1,28,Hp34980Module.Hp34937A),
new(29,1,29,Hp34980Module.Hp34937A),
new(61,1,32,Hp34980Module.Hp34937A),
new(101,2,1,Hp34980Module.Hp34922A),
new(135,2,35,Hp34980Module.Hp34922A),
new(136,2,36,Hp34980Module.Hp34922A),
new(170,2,70,Hp34980Module.Hp34922A)],[new(7,3,101,Hp34980Direction.Input),
new(13,3,104,Hp34980Direction.Output),
new(29,3,201,Hp34980Direction.Input),
new(61,3,204,Hp34980Direction.Output)]))],script,native);
        var model=id.Split('|')[0] switch {"keysight.34401a"=>QualifiedScpiModel.Agilent34401A,"keysight.33120a"=>QualifiedScpiModel.Agilent33120A,"keysight.53132a"=>QualifiedScpiModel.Agilent53132A,_=>(QualifiedScpiModel?)null};
        if(model is null)return null;
        var commissioning=model==QualifiedScpiModel.Agilent33120A?QualifiedScpiCommissioning.ContinuousSine50OhmVppOutputChangesAuthorized:QualifiedScpiCommissioning.GpibScpiConfirmed;
        return new QualifiedScpiFactory(model.Value,[new(new("scpi","fixture"),commissioning)],script,native);
    }
}
