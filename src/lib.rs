//! Static device registration. All model and connection bindings are frozen before IO.
use instrument_api::{DriverError, DriverFactory, Endpoint, ErrorKind, Result};
use instrument_scpi::{Connector, InventoryBinding, Profile, ScpiFactory};
use std::{collections::BTreeMap, sync::Arc};
include!("../generated/profiles.rs");
pub use generated_profiles::profiles;
/// A complete inventory for one endpoint under its canonical driver identity.
/// Endpoint schemes are ASCII case-insensitive; addresses are exact.
#[derive(Clone, Debug)]
pub struct ConnectionBinding {
    pub driver_id: String,
    pub endpoint: Endpoint,
    pub inventory: InventoryBinding,
}
fn validate_bindings(
    profiles: &[Profile],
    bindings: &BTreeMap<String, InventoryBinding>,
    connections: &[ConnectionBinding],
) -> Result<()> {
    if bindings
        .keys()
        .chain(connections.iter().map(|c| &c.driver_id))
        .any(|id| !profiles.iter().any(|p| p.id() == id))
    {
        return Err(DriverError::new(
            ErrorKind::DriverContractViolation,
            "Unknown driver ID in inventory bindings",
        ));
    }
    Ok(())
}
fn bound(profile: &Profile, binding: &InventoryBinding) -> Result<Profile> {
    let steps = profile
        .plans()
        .find(|p| p.operation == instrument_api::catalog::CatalogOperation::Shutdown)
        .map(|p| p.steps.clone())
        .unwrap_or_default();
    bind_profile(profile, binding, Some(steps))
}
fn routes(
    profile: &Profile,
    connections: &[ConnectionBinding],
) -> Result<Vec<(Endpoint, Profile)>> {
    connections
        .iter()
        .filter(|c| c.driver_id == profile.id())
        .map(|c| Ok((c.endpoint.clone(), bound(profile, &c.inventory)?)))
        .collect()
}
pub fn sync_factories(
    connector: Arc<dyn Connector>,
    bindings: &BTreeMap<String, InventoryBinding>,
) -> Result<Vec<Arc<dyn DriverFactory>>> {
    sync_factories_for_connections(connector, bindings, &[])
}
pub fn sync_factories_for_connections(
    connector: Arc<dyn Connector>,
    bindings: &BTreeMap<String, InventoryBinding>,
    connections: &[ConnectionBinding],
) -> Result<Vec<Arc<dyn DriverFactory>>> {
    let sources = profiles()?;
    validate_bindings(&sources, bindings, connections)?;
    sources
        .into_iter()
        .zip(generated_profiles::sync_factories(connector.clone())?)
        .map(|(source, factory)| {
            let default = match bindings.get(source.id()) {
                Some(binding) => bound(&source, binding)?,
                None => source.clone(),
            };
            Ok(Arc::new(
                ScpiFactory::new(default, "scpi", connector.clone())
                    .with_rules(factory.descriptor().rules)
                    .with_endpoint_profiles(routes(&source, connections)?)?,
            ) as Arc<dyn DriverFactory>)
        })
        .collect()
}
#[cfg(feature = "async")]
pub fn native_factories(
    connector: Arc<dyn instrument_scpi::AsyncConnector>,
    bindings: &BTreeMap<String, InventoryBinding>,
) -> Result<Vec<Arc<dyn DriverFactory>>> {
    native_factories_for_connections(connector, bindings, &[])
}
#[cfg(feature = "async")]
pub fn native_factories_for_connections(
    connector: Arc<dyn instrument_scpi::AsyncConnector>,
    bindings: &BTreeMap<String, InventoryBinding>,
    connections: &[ConnectionBinding],
) -> Result<Vec<Arc<dyn DriverFactory>>> {
    let sources = profiles()?;
    validate_bindings(&sources, bindings, connections)?;
    sources
        .into_iter()
        .zip(generated_profiles::native_factories(connector.clone())?)
        .map(|(source, factory)| {
            let default = match bindings.get(source.id()) {
                Some(binding) => bound(&source, binding)?,
                None => source.clone(),
            };
            Ok(Arc::new(
                instrument_scpi::AsyncScpiFactory::new(default, "scpi", connector.clone())
                    .with_rules(factory.descriptor().rules)
                    .with_endpoint_profiles(routes(&source, connections)?)?,
            ) as Arc<dyn DriverFactory>)
        })
        .collect()
}
#[cfg(feature = "async")]
pub fn combined_factories(
    connector: Arc<dyn Connector>,
    native: Arc<dyn instrument_scpi::AsyncConnector>,
    bindings: &BTreeMap<String, InventoryBinding>,
) -> Result<Vec<Arc<dyn DriverFactory>>> {
    combined_factories_for_connections(connector, native, bindings, &[])
}
#[cfg(feature = "async")]
pub fn combined_factories_for_connections(
    connector: Arc<dyn Connector>,
    native: Arc<dyn instrument_scpi::AsyncConnector>,
    bindings: &BTreeMap<String, InventoryBinding>,
    connections: &[ConnectionBinding],
) -> Result<Vec<Arc<dyn DriverFactory>>> {
    let sources = profiles()?;
    validate_bindings(&sources, bindings, connections)?;
    sources
        .into_iter()
        .zip(generated_profiles::sync_factories(connector.clone())?)
        .map(|(source, factory)| {
            let default = match bindings.get(source.id()) {
                Some(binding) => bound(&source, binding)?,
                None => source.clone(),
            };
            Ok(Arc::new(
                ScpiFactory::new(default, "scpi", connector.clone())
                    .with_rules(factory.descriptor().rules)
                    .with_native_connector(native.clone())
                    .with_endpoint_profiles(routes(&source, connections)?)?,
            ) as Arc<dyn DriverFactory>)
        })
        .collect()
}

/// Rebind device-owned channel tokens with the supported physical addresses.
/// The CHAN wire spelling belongs to these external profiles, never shared core.
pub fn bind_profile(
    profile: &instrument_scpi::Profile,
    binding: &InventoryBinding,
    shutdown: Option<Vec<instrument_scpi::Step>>,
) -> Result<instrument_scpi::Profile> {
    let mut plans: Vec<_> = profile.plans().cloned().collect();
    if let Some(channels) = &binding.channels {
        for plan in &mut plans {
            if plan.operation
                == instrument_api::catalog::CatalogOperation::OscilloscopeSetTriggerSource
            {
                let mapping = plan.tokens.entry("source".into()).or_default();
                mapping.retain(|key, _| key == "EXT" || key == "LINE");
                for channel in channels {
                    let token = format!("CHAN{channel}");
                    mapping.insert(token.clone(), token);
                }
            }
        }
    }
    let profile = instrument_scpi::Profile::new(
        profile.id(),
        profile.channel_ids().map(|v| v.to_vec()),
        profile.output_channel_ids().map(|v| v.to_vec()),
        plans,
    )?;
    profile.bind_inventory(binding, shutdown)
}
