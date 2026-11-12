use instrument_api::{catalog::*, *};
use instrument_driver_keysight as driver;
use instrument_runtime::*;
use instrument_scpi::{InventoryBinding, Output};
use instrument_testkit::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
include!("../../../generated/cases.rs");
fn bindings() -> Result<BTreeMap<String, InventoryBinding>> {
    Ok(driver::profiles()?
        .into_iter()
        .map(|p| {
            let psu = p.operations().contains(&Operation::Catalog(
                CatalogOperation::DcPowerSupplySetVoltage,
            ));
            let fgen = p.operations().contains(&Operation::Catalog(
                CatalogOperation::FunctionGeneratorOutputEnable,
            ));
            let sparse = p.operations().contains(&Operation::Catalog(
                CatalogOperation::OscilloscopeSetTriggerSource,
            )) || p
                .operations()
                .contains(&Operation::Catalog(CatalogOperation::SwitchCloseRoute));
            let mut outputs = Vec::new();
            if psu {
                outputs.extend([Output::DcPowerSupply(1), Output::DcPowerSupply(3)]);
            }
            if fgen {
                outputs.push(Output::FunctionGenerator);
            }
            (
                p.id().to_owned(),
                InventoryBinding {
                    channels: if psu || sparse {
                        Some(vec![1, 3])
                    } else {
                        p.channel_ids().map(|v| v.to_vec())
                    },
                    outputs: Some(outputs),
                },
            )
        })
        .collect())
}
fn candidate() -> Candidate {
    Candidate {
        endpoint: Endpoint {
            scheme: "scpi".into(),
            address: "fixture".into(),
        },
        identity: Identity::default(),
    }
}
fn selection(id: &str) -> Selection {
    Selection {
        driver_id: Some(id.split('|').next().unwrap().into()),
        ..Selection::default()
    }
}
fn binding_for(case: &CatalogCase) -> Result<BTreeMap<String, InventoryBinding>> {
    if case.id.split('|').nth(1) == Some("U") {
        Ok(BTreeMap::new())
    } else {
        bindings()
    }
}
fn connections_for(case: &CatalogCase) -> Result<Vec<driver::ConnectionBinding>> {
    Ok(binding_for(case)?
        .into_iter()
        .map(|(driver_id, inventory)| driver::ConnectionBinding {
            driver_id,
            endpoint: candidate().endpoint,
            inventory,
        })
        .collect())
}
fn check(actual: Result<CatalogReply>, case: &CatalogCase, last: bool) {
    if last {
        if let Some(error) = case.expected_error {
            assert_eq!(actual.unwrap_err().kind, error, "{}", case.id);
        } else {
            assert_eq!(
                actual.unwrap(),
                case.expected.clone().unwrap(),
                "{}",
                case.id
            );
        }
    } else {
        actual.unwrap();
    }
}
fn identities() -> Result<()> {
    let script = Script::new([]);
    let registry = DriverRegistry::new(driver::sync_factories(
        Arc::new(script.clone()),
        &BTreeMap::new(),
    )?)?;
    for factory in driver::sync_factories(Arc::new(script.clone()), &BTreeMap::new())? {
        let descriptor = factory.descriptor();
        for rule in &descriptor.rules {
            let identity = match rule {
                MatchRule::ExactIdentity {
                    manufacturer,
                    model,
                    ..
                } => Identity {
                    manufacturer: Some(manufacturer.clone()),
                    model: Some(model.clone()),
                    ..Identity::default()
                },
                MatchRule::ExactProduct {
                    vendor_id,
                    product_id,
                    ..
                } => Identity {
                    vendor_id: Some(*vendor_id),
                    product_id: Some(*product_id),
                    ..Identity::default()
                },
                _ => panic!("Expected exact migrated hint"),
            };
            let mut c = candidate();
            c.identity = identity;
            assert_eq!(
                registry.select(&c, &Selection::default())?.driver_id,
                descriptor.id
            );
        }
    }
    assert_eq!(script.opens(), 0);
    Ok(())
}
fn synchronous() -> Result<()> {
    identities()?;
    for case in generated_cases::cases() {
        let script = Script::new(case.script.clone());
        let registry = DriverRegistry::new(driver::sync_factories_for_connections(
            Arc::new(script.clone()),
            &BTreeMap::new(),
            &connections_for(&case)?,
        )?)?;
        let h = registry.open(
            &candidate(),
            &selection(case.id),
            &BTreeSet::new(),
            ExecutionMode::Sync,
            ConnectionOwnership::Owned,
        )?;
        for (i, request) in case.calls.iter().enumerate() {
            check(h.invoke(request.clone()), &case, i + 1 == case.calls.len());
        }
        h.close(false)?;
        assert_eq!(
            h.invoke(CatalogRequest::QueryIdentity).unwrap_err().kind,
            ErrorKind::SessionClosed,
            "{} closed",
            case.id
        );
        script.assert_complete()?;
        script.assert_thread_affinity()?;
        assert_eq!(script.opens(), 1);
        assert_eq!(script.closes(), 1);
    }
    Ok(())
}
#[cfg(feature = "async")]
async fn asynchronous() -> Result<()> {
    for mode in 0..3 {
        for case in generated_cases::cases() {
            let script = Script::new(case.script.clone());
            let factories = if mode == 0 {
                driver::native_factories_for_connections(
                    Arc::new(script.clone()),
                    &BTreeMap::new(),
                    &connections_for(&case)?,
                )?
            } else {
                driver::sync_factories_for_connections(
                    Arc::new(script.clone()),
                    &BTreeMap::new(),
                    &connections_for(&case)?,
                )?
            };
            let registry = DriverRegistry::new(factories)?;
            let native = if mode == 0 {
                Some(
                    registry
                        .open_native(
                            &candidate(),
                            &selection(case.id),
                            &BTreeSet::new(),
                            ConnectionOwnership::Owned,
                        )
                        .await?,
                )
            } else {
                None
            };
            let worker = if mode > 0 {
                Some(
                    registry
                        .open_worker(
                            &candidate(),
                            &selection(case.id),
                            &BTreeSet::new(),
                            ConnectionOwnership::Owned,
                        )
                        .await?,
                )
            } else {
                None
            };
            for (i, request) in case.calls.iter().enumerate() {
                let actual = if let Some(h) = &native {
                    h.invoke_async(request.clone()).await
                } else if mode == 1 {
                    worker.as_ref().unwrap().invoke_async(request.clone()).await
                } else {
                    worker.as_ref().unwrap().invoke(request.clone())
                };
                check(actual, &case, i + 1 == case.calls.len());
            }
            if let Some(h) = native {
                h.close(false).await?;
                assert_eq!(
                    h.invoke_async(CatalogRequest::QueryIdentity)
                        .await
                        .unwrap_err()
                        .kind,
                    ErrorKind::SessionClosed
                );
            } else {
                let h = worker.unwrap();
                h.close(false).await?;
                assert_eq!(
                    h.invoke(CatalogRequest::QueryIdentity).unwrap_err().kind,
                    ErrorKind::SessionClosed
                );
                script.assert_thread_affinity()?;
            }
            script.assert_complete()?;
            assert_eq!(script.opens(), 1);
            assert_eq!(script.closes(), 1);
        }
    }
    Ok(())
}
fn main() -> Result<()> {
    synchronous()?;
    #[cfg(feature = "async")]
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(asynchronous())?;
    println!("PASS Keysight package: {} independent cases; exact traffic, values, units, errors and resource lifetime",generated_cases::cases().len());
    Ok(())
}
