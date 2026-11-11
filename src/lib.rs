//! Explicit device package registration; no implicit IO or runtime plugin loading.
use instrument_api::{DriverFactory, Result};
use instrument_scpi::{Connector, InventoryBinding, ScpiFactory};
use std::{collections::BTreeMap, sync::Arc};
include!("../generated/profiles.rs");
pub use generated_profiles::profiles;
/// Bind complete physical inventories before opening. An absent binding preserves source hints.
pub fn sync_factories(
    connector: Arc<dyn Connector>,
    bindings: &BTreeMap<String, InventoryBinding>,
) -> Result<Vec<Arc<dyn DriverFactory>>> {
    profiles()?
        .into_iter()
        .zip(generated_profiles::sync_factories(connector.clone())?)
        .map(|(profile, factory)| {
            let descriptor = factory.descriptor();
            let steps = profile
                .plans()
                .find(|p| p.operation == instrument_api::catalog::CatalogOperation::Shutdown)
                .map(|p| p.steps.clone())
                .unwrap_or_default();
            let profile = match bindings.get(profile.id()) {
                Some(binding) => profile.bind_inventory(binding, Some(steps))?,
                None => profile,
            };
            Ok(Arc::new(
                ScpiFactory::new(profile, "scpi", connector.clone()).with_rules(descriptor.rules),
            ) as Arc<dyn DriverFactory>)
        })
        .collect()
}
#[cfg(feature = "async")]
pub fn native_factories(
    connector: Arc<dyn instrument_scpi::AsyncConnector>,
    bindings: &BTreeMap<String, InventoryBinding>,
) -> Result<Vec<Arc<dyn DriverFactory>>> {
    profiles()?
        .into_iter()
        .zip(generated_profiles::native_factories(connector.clone())?)
        .map(|(profile, factory)| {
            let steps = profile
                .plans()
                .find(|p| p.operation == instrument_api::catalog::CatalogOperation::Shutdown)
                .map(|p| p.steps.clone())
                .unwrap_or_default();
            let profile = match bindings.get(profile.id()) {
                Some(binding) => profile.bind_inventory(binding, Some(steps))?,
                None => profile,
            };
            Ok(Arc::new(
                instrument_scpi::AsyncScpiFactory::new(profile, "scpi", connector.clone())
                    .with_rules(factory.descriptor().rules),
            ) as Arc<dyn DriverFactory>)
        })
        .collect()
}
