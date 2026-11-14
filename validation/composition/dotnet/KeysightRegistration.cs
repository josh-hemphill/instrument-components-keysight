using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Keysight;
using InstrumentComponents.TestKit;
namespace CompositionProof.Keysight;
// Test-only explicit composition of the independently commissioned fixtures.
internal static class ComposedRegistration
{
    internal static IEnumerable<IDriverFactory> Factories(Script io)
    {
        var required=new HashSet<string>{"keysight.34401a","keysight.33120a","keysight.53132a","keysight.hp3585b","keysight.34980a","keysight.hp5334a","keysight.hp5334b","keysight.hp3497a","keysight.hp8920a","keysight.hp8920b","keysight.hp8921a"};
        foreach(var test in GeneratedKeysight.GeneratedCases.Create())
        {
            var id=test.Id.Split('|')[0];if(!required.Remove(id))continue;
            var factory=QualifiedFixture.Factory(test.Id,io,true)??HpFixture.Factory(test.Id,io,true);
            if(factory is not null){yield return factory;continue;}
            var model=id=="keysight.hp5334a"?Hp5334Model.Hp5334A:Hp5334Model.Hp5334B;
            yield return new Hp5334Factory(model,Hp5334Commissioning.NativeInternalGatingConfirmed,io,io);
        }
        if(required.Count!=0)throw new InvalidOperationException("Missing commissioned fixtures: "+string.Join(",",required));
    }
}
