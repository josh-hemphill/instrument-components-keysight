using InstrumentComponents.Scpi;
namespace InstrumentComponents.Drivers.Keysight;
/// <summary>Static registration. Bindings are frozen and validated before IO opens.</summary>
public static class Drivers
{
    public static IReadOnlyList<ScpiProfile> Profiles()=>GeneratedDrivers.GeneratedProfiles.Create();
    public static IReadOnlyList<IDriverFactory> Factories(IScpiConnector connector,IReadOnlyDictionary<string,InventoryBinding>? bindings=null,IAsyncScpiConnector? nativeConnector=null)
    {
        var profiles=Profiles();var descriptors=GeneratedDrivers.GeneratedProfiles.SyncFactories(connector);
        return profiles.Select((profile,i)=>{
            var steps=profile.OperationPlans.FirstOrDefault(p=>p.Operation==Catalog.CatalogOperation.Shutdown)?.Steps ?? [];
            var bound=bindings is not null && bindings.TryGetValue(profile.Id,out var binding)?profile.BindInventory(binding,steps):profile;
            return (IDriverFactory)new ScpiFactory(bound,"scpi",connector,descriptors[i].Descriptor.Rules,nativeConnector);
        }).ToArray();
    }
    public static IReadOnlyList<IDriverFactory> NativeFactories(IAsyncScpiConnector connector,IReadOnlyDictionary<string,InventoryBinding>? bindings=null)
    {
        var profiles=Profiles();var descriptors=GeneratedDrivers.GeneratedProfiles.NativeFactories(connector);
        return profiles.Select((profile,i)=>{
            var steps=profile.OperationPlans.FirstOrDefault(p=>p.Operation==Catalog.CatalogOperation.Shutdown)?.Steps ?? [];
            var bound=bindings is not null && bindings.TryGetValue(profile.Id,out var binding)?profile.BindInventory(binding,steps):profile;
            return (IDriverFactory)new AsyncScpiFactory(bound,"scpi",connector,descriptors[i].Descriptor.Rules);
        }).ToArray();
    }
}
