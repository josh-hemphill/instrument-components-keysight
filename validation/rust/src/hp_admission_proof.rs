//! Isolated admission invariants identified in independent Council source review.
use instrument_api::{catalog::*, *};
use instrument_driver_keysight::{hp3497 as daq, hp_radio as rf};
use instrument_runtime::*;
use instrument_testkit::*;
use std::{collections::BTreeSet, sync::Arc};
fn endpoint(address: &str) -> Endpoint {
    Endpoint {
        scheme: "hpib".into(),
        address: address.into(),
    }
}
fn daq_config() -> daq::Configuration {
    daq::Configuration {
        option001: true,
        normal_dvm_routing_confirmed: true,
        inputs: vec![daq::Input {
            channel: 7,
            slot: 0,
            local: 0,
            module: daq::Module::Option010,
            compensation: daq::Compensation::None,
        }],
    }
}
fn radio_config(model: rf::Model) -> rf::Configuration {
    rf::Configuration {
        model,
        firmware: match model {
            rf::Model::Hp8920A => "A.18.00",
            rf::Model::Hp8920B => "B.06.00",
            rf::Model::Hp8921A => "A.15.00",
        }
        .into(),
        attenuator: rf::Attenuator::Electronic,
        input_power: rf::InputPower::Standard,
        source_port: rf::SourcePort::RfInOut,
        receiver_input: rf::ReceiverInput::RfInOut,
        commissioning: rf::Commissioning::Confirmed,
    }
}
fn rejected<T>(result: Result<T>, kind: ErrorKind) {
    assert!(matches!(result,Err(e) if e.kind==kind));
}
pub fn synchronous() -> Result<()> {
    let empty = Script::new([]);
    let base = daq_config();
    let make = |configuration| {
        daq::Factory::new(
            vec![daq::Binding {
                endpoint: endpoint("fixture"),
                configuration,
            }],
            Arc::new(empty.clone()),
        )
    };
    let mut c = base.clone();
    c.inputs.clear();
    rejected(make(c), ErrorKind::InvalidArgument);
    // Each duplicate invariant is isolated: the other field is distinct.
    let mut c = base.clone();
    let mut extra = c.inputs[0].clone();
    extra.local = 1;
    c.inputs.push(extra);
    rejected(make(c), ErrorKind::InvalidArgument);
    let mut c = base.clone();
    let mut extra = c.inputs[0].clone();
    extra.channel = 8;
    c.inputs.push(extra);
    rejected(make(c), ErrorKind::InvalidArgument);
    // Each mapping is individually valid, but the two modules conflict in slot0.
    let mut c = base.clone();
    c.inputs.push(daq::Input {
        channel: 8,
        slot: 0,
        local: 1,
        module: daq::Module::Option020,
        compensation: daq::Compensation::Software,
    });
    rejected(make(c), ErrorKind::InvalidArgument);
    for (module, compensation, local) in [
        (daq::Module::Option010, daq::Compensation::None, 20),
        (daq::Module::Option010, daq::Compensation::Software, 0),
        (daq::Module::Option010, daq::Compensation::Unknown, 0),
        (daq::Module::Option020, daq::Compensation::Hardware, 0),
        (daq::Module::Option020, daq::Compensation::Unknown, 0),
    ] {
        let mut c = base.clone();
        c.inputs[0].module = module;
        c.inputs[0].compensation = compensation;
        c.inputs[0].local = local;
        rejected(make(c), ErrorKind::InvalidArgument);
    }
    rejected(
        daq::Factory::new(vec![], Arc::new(empty.clone())),
        ErrorKind::InvalidArgument,
    );
    let routes = (0..4097)
        .map(|i| daq::Binding {
            endpoint: endpoint(&format!("route{i}")),
            configuration: base.clone(),
        })
        .collect();
    let routes: Vec<daq::Binding> = routes;
    daq::Factory::new(routes[..4096].to_vec(), Arc::new(empty.clone()))?;
    rejected(
        daq::Factory::new(routes, Arc::new(empty.clone())),
        ErrorKind::InvalidArgument,
    );
    assert!(!make(base.clone())?
        .execution_modes()
        .contains(&ExecutionMode::NativeAsync));
    assert_eq!(empty.opens(), 0);
    // Rust caller retains a clone which is mutated after factory construction.
    let script = Script::new([
        ScriptStep::Query(
            "VT4AE0VC0VS0VF1VN1VW0SO1VR5AI0".into(),
            Ok("+1.25000E+0".into()),
        ),
        ScriptStep::Close(Ok(())),
    ]);
    let mut caller = base;
    let factory = daq::Factory::new(
        vec![daq::Binding {
            endpoint: endpoint("fixture"),
            configuration: caller.clone(),
        }],
        Arc::new(script.clone()),
    )?;
    caller.inputs.clear();
    let registry = DriverRegistry::new(vec![Arc::new(factory) as Arc<dyn DriverFactory>])?;
    let handle = registry.open(
        &Candidate {
            endpoint: endpoint("fixture"),
            identity: Identity::default(),
        },
        &Selection {
            driver_id: Some("keysight.hp3497a".into()),
            ..Default::default()
        },
        &BTreeSet::new(),
        ExecutionMode::Sync,
        ConnectionOwnership::Owned,
    )?;
    assert_eq!(
        DataAcquisition(&handle).measure_voltage_dc(7, None)?.value,
        1.25
    );
    handle.close(false)?;
    script.assert_complete()?;
    for model in [rf::Model::Hp8920A, rf::Model::Hp8920B, rf::Model::Hp8921A] {
        let empty = Script::new([]);
        let valid = radio_config(model);
        rejected(
            rf::Factory::new(model, vec![], Arc::new(empty.clone())),
            ErrorKind::InvalidArgument,
        );
        let routes = (0..4097)
            .map(|i| rf::Binding {
                endpoint: endpoint(&format!("route{i}")),
                configuration: valid.clone(),
            })
            .collect();
        let routes: Vec<rf::Binding> = routes;
        rf::Factory::new(model, routes[..4096].to_vec(), Arc::new(empty.clone()))?;
        rejected(
            rf::Factory::new(model, routes, Arc::new(empty.clone())),
            ErrorKind::InvalidArgument,
        );
        for field in 0..3 {
            let mut c = valid.clone();
            match field {
                0 => c.source_port = rf::SourcePort::Unknown,
                1 => c.receiver_input = rf::ReceiverInput::Unknown,
                _ => c.input_power = rf::InputPower::Unknown,
            };
            rejected(
                rf::Factory::new(
                    model,
                    vec![rf::Binding {
                        endpoint: endpoint("fixture"),
                        configuration: c,
                    }],
                    Arc::new(empty.clone()),
                ),
                ErrorKind::InvalidArgument,
            );
        }
        let f = rf::Factory::new(
            model,
            vec![rf::Binding {
                endpoint: endpoint("fixture"),
                configuration: valid.clone(),
            }],
            Arc::new(empty.clone()),
        )?;
        assert!(!f.execution_modes().contains(&ExecutionMode::NativeAsync));
        assert_eq!(empty.opens(), 0);
        let script = Script::new([
            ScriptStep::Write("DISP RFG".into()),
            ScriptStep::Write("RFG:AMPL:STAT OFF".into()),
            ScriptStep::Query("RFG:AMPL:STAT?".into(), Ok("0".into())),
            ScriptStep::Close(Ok(())),
        ]);
        let f = rf::Factory::new(
            model,
            vec![rf::Binding {
                endpoint: endpoint("fixture"),
                configuration: valid,
            }],
            Arc::new(script.clone()),
        )?;
        let registry = DriverRegistry::new(vec![Arc::new(f) as Arc<dyn DriverFactory>])?;
        let handle = registry.open(
            &Candidate {
                endpoint: endpoint("fixture"),
                identity: Identity::default(),
            },
            &Selection {
                driver_id: Some(model.driver_id().into()),
                ..Default::default()
            },
            &BTreeSet::new(),
            ExecutionMode::Sync,
            ConnectionOwnership::Owned,
        )?;
        for v in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            for (frequency_hz, level_dbm) in [(v, -30.0), (30_000_000.0, v)] {
                rejected(
                    RfSignalGenerator(&handle).configure(frequency_hz, level_dbm),
                    ErrorKind::InvalidArgument,
                );
                assert!(script.events().is_empty());
            }
        }
        // A successful typed operation proves pre-IO rejection did not quarantine.
        RfSignalGenerator(&handle).output_enable(false)?;
        handle.close(false)?;
        script.assert_complete()?;
    }
    println!("PASS HP isolated inventory invariants,4097 unique route bounds, native-mode absence, valid inventory after caller mutation and RF nonfinite pre-IO usable follow-up");
    Ok(())
}
#[cfg(feature = "async")]
pub async fn asynchronous() -> Result<()> {
    for model in [rf::Model::Hp8920A, rf::Model::Hp8920B, rf::Model::Hp8921A] {
        let script = Script::new([]);
        let valid = radio_config(model);
        let mut ant = valid.clone();
        ant.receiver_input = rf::ReceiverInput::Antenna;
        let f = rf::Factory::new(
            model,
            vec![
                rf::Binding {
                    endpoint: endpoint("fixture"),
                    configuration: valid,
                },
                rf::Binding {
                    endpoint: endpoint("ant"),
                    configuration: ant,
                },
            ],
            Arc::new(script.clone()),
        )?
        .with_native_connector(Arc::new(script.clone()));
        let registry = DriverRegistry::new(vec![Arc::new(f) as Arc<dyn DriverFactory>])?;
        let selection = Selection {
            driver_id: Some(model.driver_id().into()),
            ..Default::default()
        };
        let power = BTreeSet::from([Operation::Catalog(CatalogOperation::RfReceiverMeasurePower)]);
        for (address, required, kind) in [
            ("ant", power, ErrorKind::UnsupportedOperation),
            ("missing", BTreeSet::new(), ErrorKind::InvalidArgument),
            ("FIXTURE", BTreeSet::new(), ErrorKind::InvalidArgument),
        ] {
            let candidate = Candidate {
                endpoint: endpoint(address),
                identity: Identity::default(),
            };
            rejected(
                registry
                    .open_native(
                        &candidate,
                        &selection,
                        &required,
                        ConnectionOwnership::Owned,
                    )
                    .await,
                kind,
            );
            rejected(
                registry
                    .open_worker(
                        &candidate,
                        &selection,
                        &required,
                        ConnectionOwnership::Owned,
                    )
                    .await,
                kind,
            );
            assert_eq!(script.opens(), 0);
        }
        script.assert_complete()?;
    }
    let script = Script::new([]);
    let f = daq::Factory::new(
        vec![daq::Binding {
            endpoint: endpoint("fixture"),
            configuration: daq_config(),
        }],
        Arc::new(script.clone()),
    )?
    .with_native_connector(Arc::new(script.clone()));
    let registry = DriverRegistry::new(vec![Arc::new(f) as Arc<dyn DriverFactory>])?;
    let candidate = Candidate {
        endpoint: endpoint("missing"),
        identity: Identity::default(),
    };
    let selection = Selection {
        driver_id: Some("keysight.hp3497a".into()),
        ..Default::default()
    };
    rejected(
        registry
            .open_native(
                &candidate,
                &selection,
                &BTreeSet::new(),
                ConnectionOwnership::Owned,
            )
            .await,
        ErrorKind::InvalidArgument,
    );
    rejected(
        registry
            .open_worker(
                &candidate,
                &selection,
                &BTreeSet::new(),
                ConnectionOwnership::Owned,
            )
            .await,
        ErrorKind::InvalidArgument,
    );
    assert_eq!(script.opens(), 0);
    script.assert_complete()?;
    println!("PASS HP selected-route admission before native/worker acquisition; no unbound or ANT capability fallback");
    Ok(())
}
