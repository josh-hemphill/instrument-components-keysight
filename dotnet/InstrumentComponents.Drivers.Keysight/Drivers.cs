using InstrumentComponents.Scpi;
namespace InstrumentComponents.Drivers.Keysight;
/// <summary>Inventory for one exact endpoint, preserving its canonical driver identity.</summary>
public sealed record ConnectionBinding(string DriverId,Endpoint Endpoint,InventoryBinding Inventory);
/// <summary>Static registration. Model and connection bindings are frozen before IO.</summary>
public static class Drivers
{
    public static IReadOnlyList<ScpiProfile> Profiles()=>GeneratedDrivers.GeneratedProfiles.Create();
    private static void Validate(IReadOnlyList<ScpiProfile> profiles,IReadOnlyDictionary<string,InventoryBinding>? bindings,IReadOnlyList<ConnectionBinding> connections)
    {
        var ids=profiles.Select(p=>p.Id).ToHashSet(StringComparer.Ordinal);
        if((bindings?.Keys??[]).Concat(connections.Select(c=>c.DriverId)).Any(id=>!ids.Contains(id)))
            throw new DriverException(DriverErrorKind.DriverContractViolation,"Unknown driver ID in inventory bindings");
    }
    private static ScpiProfile Bound(ScpiProfile source,InventoryBinding binding)=>BindProfile(source,binding,source.OperationPlans.FirstOrDefault(p=>p.Operation==Catalog.CatalogOperation.Shutdown)?.Steps??[]);
    private static KeyValuePair<Endpoint,ScpiProfile>[] Routes(ScpiProfile source,IReadOnlyList<ConnectionBinding> connections)=>connections.Where(c=>c.DriverId==source.Id).Select(c=>new KeyValuePair<Endpoint,ScpiProfile>(c.Endpoint,Bound(source,c.Inventory))).ToArray();
    public static IReadOnlyList<IDriverFactory> Factories(IScpiConnector connector,IReadOnlyDictionary<string,InventoryBinding>? bindings=null,IAsyncScpiConnector? nativeConnector=null,IEnumerable<ConnectionBinding>? connections=null)
    {
        var profiles=Profiles();var frozen=connections?.ToArray()??[];Validate(profiles,bindings,frozen);
        var descriptors=GeneratedDrivers.GeneratedProfiles.SyncFactories(connector);
        return profiles.Select((source,i)=>{
            var bound=bindings is not null&&bindings.TryGetValue(source.Id,out var binding)?Bound(source,binding):source;
            return (IDriverFactory)new ScpiFactory(bound,"scpi",connector,descriptors[i].Descriptor.Rules,nativeConnector,Routes(source,frozen));
        }).ToArray();
    }
    public static IReadOnlyList<IDriverFactory> NativeFactories(IAsyncScpiConnector connector,IReadOnlyDictionary<string,InventoryBinding>? bindings=null,IEnumerable<ConnectionBinding>? connections=null)
    {
        var profiles=Profiles();var frozen=connections?.ToArray()??[];Validate(profiles,bindings,frozen);
        var descriptors=GeneratedDrivers.GeneratedProfiles.NativeFactories(connector);
        return profiles.Select((source,i)=>{
            var bound=bindings is not null&&bindings.TryGetValue(source.Id,out var binding)?Bound(source,binding):source;
            return (IDriverFactory)new AsyncScpiFactory(bound,"scpi",connector,descriptors[i].Descriptor.Rules,Routes(source,frozen));
        }).ToArray();
    }
    public static ScpiProfile BindProfile(ScpiProfile profile,InventoryBinding binding,IReadOnlyList<Step>? shutdownSteps)
    {
        var plans=profile.OperationPlans.Select(plan=>{
            if(plan.Operation!=Catalog.CatalogOperation.OscilloscopeSetTriggerSource || binding.Channels is null)return plan;
            var tokens=plan.Tokens!.ToDictionary(p=>p.Key,p=>p.Value);
            var mapping=tokens["source"].Where(p=>p.Key is "EXT" or "LINE").ToDictionary(p=>p.Key,p=>p.Value);
            foreach(var channel in binding.Channels){var token="CHAN"+channel.ToString(System.Globalization.CultureInfo.InvariantCulture);mapping[token]=token;}
            tokens["source"]=mapping;return plan with{Tokens=tokens};
        }).ToArray();
        var source=new ScpiProfile(profile.Id,profile.ChannelIds,profile.OutputChannelIds,plans);
        return source.BindInventory(binding,shutdownSteps);
    }
}
