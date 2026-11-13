use instrument::{api::catalog::*, api::*, DeviceCatalog};
use instrument_runtime::*;
use instrument_testkit::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
fn factories(io: Arc<Script>) -> Result<Vec<Arc<dyn DriverFactory>>> {
    let mut all = Vec::new();
    all.extend(instrument_driver_keysight::sync_factories(
        io.clone(),
        &BTreeMap::new(),
    )?);
    all.extend(instrument_driver_keithley::sync_factories(
        io.clone(),
        &BTreeMap::new(),
    )?);
    all.extend(instrument_driver_rigol::sync_factories(
        io.clone(),
        &BTreeMap::new(),
    )?);
    all.extend(instrument_driver_siglent::sync_factories(
        io.clone(),
        &BTreeMap::new(),
    )?);
    all.extend(instrument_driver_tektronix::sync_factories(
        io.clone(),
        &BTreeMap::new(),
    )?);
    all.extend(keysight::composed_additions(&io, false)?);
    all.extend(tektronix::composed_additions(&io, false)?);
    Ok(all)
}
#[cfg(feature = "async")]
fn combined(io: Arc<Script>) -> Result<Vec<Arc<dyn DriverFactory>>> {
    let mut all = Vec::new();
    all.extend(instrument_driver_keysight::combined_factories(
        io.clone(),
        io.clone(),
        &BTreeMap::new(),
    )?);
    all.extend(instrument_driver_keithley::combined_factories(
        io.clone(),
        io.clone(),
        &BTreeMap::new(),
    )?);
    all.extend(instrument_driver_rigol::combined_factories(
        io.clone(),
        io.clone(),
        &BTreeMap::new(),
    )?);
    all.extend(instrument_driver_siglent::combined_factories(
        io.clone(),
        io.clone(),
        &BTreeMap::new(),
    )?);
    all.extend(instrument_driver_tektronix::combined_factories(
        io.clone(),
        io.clone(),
        &BTreeMap::new(),
    )?);
    all.extend(keysight::composed_additions(&io, true)?);
    all.extend(tektronix::composed_additions(&io, true)?);
    Ok(all)
}
fn candidate(identity: Identity) -> Candidate {
    Candidate {
        endpoint: Endpoint {
            scheme: "scpi".into(),
            address: "fixture".into(),
        },
        identity,
    }
}
struct Alias(DriverDescriptor);
impl DriverFactory for Alias {
    fn descriptor(&self) -> DriverDescriptor {
        self.0.clone()
    }
}
fn matching() -> Result<()> {
    let io = Arc::new(Script::new([]));
    let all = factories(io.clone())?;
    let registry = DriverRegistry::new(all.clone())?;
    let mut hints = 0;
    let mut usb = 0;
    for f in &all {
        let d = f.descriptor();
        for rule in d.rules {
            let id = match rule {
                MatchRule::ExactIdentity {
                    scheme,
                    manufacturer,
                    model,
                } => {
                    hints += 1;
                    (
                        scheme,
                        Identity {
                            manufacturer: Some(manufacturer),
                            model: Some(model),
                            ..Identity::default()
                        },
                    )
                }
                MatchRule::ExactProduct {
                    scheme,
                    vendor_id,
                    product_id,
                } => {
                    usb += 1;
                    (
                        scheme,
                        Identity {
                            vendor_id: Some(vendor_id),
                            product_id: Some(product_id),
                            ..Identity::default()
                        },
                    )
                }
                _ => panic!("Unexpected source hint"),
            };
            assert_eq!(
                registry
                    .select(
                        &Candidate {
                            endpoint: Endpoint {
                                scheme: id.0,
                                address: "fixture".into()
                            },
                            identity: id.1
                        },
                        &Selection::default()
                    )?
                    .driver_id,
                d.id
            );
        }
    }
    assert!(hints >= 47);
    println!("PASS actual registration evidence: {hints} identity rules, {usb} product rules; explicit device factories are checked below");
    for id in [
        "keysight.34401a",
        "keysight.33120a",
        "keysight.53132a",
        "keysight.hp3585b",
        "keysight.34980a",
        "keysight.hp5334a",
        "keysight.hp5334b",
        "keysight.hp3497a",
        "keysight.hp8920a",
        "keysight.hp8920b",
        "keysight.hp8921a",
        "tektronix.tds1002",
    ] {
        assert!(
            all.iter().any(|f| f.descriptor().id == id),
            "Missing composed registration {id}"
        );
    }
    // Synthetic metadata-only USB rule: no hardware identity claim.
    let mut usb_alias = all[0].descriptor();
    usb_alias.id = "synthetic.usb".into();
    usb_alias.rules = vec![MatchRule::ExactProduct {
        scheme: "scpi".into(),
        vendor_id: 0x1234,
        product_id: 0x5678,
    }];
    let registry = DriverRegistry::new(
        all.iter()
            .cloned()
            .chain([Arc::new(Alias(usb_alias)) as Arc<dyn DriverFactory>]),
    )?;
    let same = Identity {
        vendor_id: Some(0x1234),
        product_id: Some(0x5678),
        ..Identity::default()
    };
    assert_eq!(
        registry
            .select(&candidate(same.clone()), &Selection::default())?
            .driver_id,
        "synthetic.usb"
    );
    let conflict = Identity {
        manufacturer: Some("Keithley Instruments".into()),
        model: Some("2602B".into()),
        ..same
    };
    let conflict = candidate(conflict);
    assert_eq!(
        registry
            .select(&conflict, &Selection::default())
            .unwrap_err()
            .kind,
        ErrorKind::AmbiguousDriver
    );
    let preferred = Selection {
        preferences: [("keithley.2602b".into(), 7)].into_iter().collect(),
        ..Selection::default()
    };
    let evidence = registry.select(&conflict, &preferred)?;
    assert_eq!(evidence.driver_id, "keithley.2602b");
    assert!(!evidence.explicit);
    let forced = Selection {
        driver_id: Some("synthetic.usb".into()),
        ..preferred
    };
    let evidence = registry.select(&conflict, &forced)?;
    assert_eq!(evidence.driver_id, "synthetic.usb");
    assert!(evidence.explicit);
    assert_eq!(
        registry
            .select(
                &conflict,
                &Selection {
                    driver_id: Some("synthetic.not-registered".into()),
                    ..Selection::default()
                }
            )
            .unwrap_err()
            .kind,
        ErrorKind::DriverNotFound
    );
    let mut a = all[0].descriptor();
    a.id = "synthetic.equal-a".into();
    let mut b = a.clone();
    b.id = "synthetic.equal-b".into();
    let c = match &a.rules[0] {
        MatchRule::ExactIdentity {
            manufacturer,
            model,
            ..
        } => candidate(Identity {
            manufacturer: Some(manufacturer.clone()),
            model: Some(model.clone()),
            ..Identity::default()
        }),
        _ => panic!("Expected model"),
    };
    assert_eq!(
        DriverRegistry::new([
            Arc::new(Alias(a)) as Arc<dyn DriverFactory>,
            Arc::new(Alias(b))
        ])?
        .select(&c, &Selection::default())
        .unwrap_err()
        .kind,
        ErrorKind::AmbiguousDriver
    );
    assert_eq!(io.opens(), 0);
    Ok(())
}
fn script() -> Script {
    Script::new([
        ScriptStep::Query(":MEAS:VOLT:DC?".into(), Ok("1.25".into())),
        ScriptStep::Query(":MEAS:VOLT? 1".into(), Ok("2.5".into())),
        ScriptStep::Close(Ok(())),
    ])
}
fn smu() -> Candidate {
    candidate(Identity {
        manufacturer: Some("Keithley Instruments".into()),
        model: Some("2602B".into()),
        ..Identity::default()
    })
}
fn required() -> BTreeSet<Operation> {
    [
        CatalogOperation::DmmMeasureVoltageDc,
        CatalogOperation::DcPowerSupplyReadVoltage,
    ]
    .into_iter()
    .map(Operation::Catalog)
    .collect()
}
fn synchronous() -> Result<()> {
    let io = Arc::new(script());
    let c = smu();
    let catalog = DeviceCatalog::new(
        DriverRegistry::new(factories(io.clone())?)?,
        vec![c.clone()],
    )?;
    let h = catalog.device(&c.endpoint)?.open(
        &Selection::default(),
        &required(),
        ConnectionOwnership::Owned,
    )?;
    assert_eq!(Dmm(&h).measure_voltage_dc(None)?.value, 1.25);
    assert_eq!(DcPowerSupply(&h).read_voltage(1)?.value, 2.5);
    h.close(false)?;
    io.assert_complete()?;
    assert_eq!(io.opens(), 1);
    assert_eq!(io.closes(), 1);
    Ok(())
}
#[cfg(feature = "async")]
async fn asynchronous() -> Result<()> {
    for mode in 0..3 {
        let io = Arc::new(script());
        let c = smu();
        let catalog =
            DeviceCatalog::new(DriverRegistry::new(combined(io.clone())?)?, vec![c.clone()])?;
        let d = catalog.device(&c.endpoint)?;
        if mode == 0 {
            let h = d
                .open_native(
                    &Selection::default(),
                    &required(),
                    ConnectionOwnership::Owned,
                )
                .await?;
            assert_eq!(AsyncDmm(&h).measure_voltage_dc(None).await?.value, 1.25);
            assert_eq!(AsyncDcPowerSupply(&h).read_voltage(1).await?.value, 2.5);
            h.close(false).await?;
        } else {
            let h = d
                .open_worker(
                    &Selection::default(),
                    &required(),
                    ConnectionOwnership::Owned,
                )
                .await?;
            if mode == 1 {
                assert_eq!(AsyncDmm(&h).measure_voltage_dc(None).await?.value, 1.25);
                assert_eq!(AsyncDcPowerSupply(&h).read_voltage(1).await?.value, 2.5);
            } else {
                assert_eq!(Dmm(&h).measure_voltage_dc(None)?.value, 1.25);
                assert_eq!(DcPowerSupply(&h).read_voltage(1)?.value, 2.5);
            }
            h.close(false).await?;
            io.assert_thread_affinity()?;
        }
        io.assert_complete()?;
        assert_eq!(io.opens(), 1);
        assert_eq!(io.closes(), 1);
    }
    Ok(())
}
mod keithley;
mod keysight;
mod rigol;
mod siglent;
mod tektronix;
fn run() -> Result<()> {
    matching()?;
    synchronous()?;
    #[cfg(feature = "async")]
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(asynchronous())?;
    keysight::package_proof_main()?;
    keithley::package_proof_main()?;
    rigol::package_proof_main()?;
    siglent::package_proof_main()?;
    tektronix::package_proof_main()?;
    println!("PASS all-five static application:current registered models and synthetic USB, combined/conflicting evidence, equal-rank ambiguity, one2602B backend, independent all-operation cases");
    Ok(())
}

fn main() -> Result<()> {
    std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap()
}
