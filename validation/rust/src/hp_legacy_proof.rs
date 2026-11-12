//! Procedural inventory, capability, lifetime and quarantine checks beyond wire cases.
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
fn config(model: rf::Model) -> rf::Configuration {
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
fn radio(model: rf::Model, config: rf::Configuration, script: &Script) -> Result<rf::Factory> {
    rf::Factory::new(
        model,
        vec![rf::Binding {
            endpoint: endpoint("fixture"),
            configuration: config,
        }],
        Arc::new(script.clone()),
    )
}
fn invalid<T>(result: Result<T>) {
    assert!(matches!(result, Err(e) if e.kind == ErrorKind::InvalidArgument));
}
pub fn synchronous() -> Result<()> {
    for model in [rf::Model::Hp8920A, rf::Model::Hp8920B, rf::Model::Hp8921A] {
        let empty = Script::new([]);
        let valid = config(model);
        for firmware in [
            "", "a.99.00", "A.9.00", "A.99.00 ", "A.+9.00", "A.99.0X", "C.99.00", "A.00.00",
            "B.00.00",
        ] {
            let mut c = valid.clone();
            c.firmware = firmware.into();
            invalid(radio(model, c, &empty));
        }
        let mut later = valid.clone();
        later.firmware = if model == rf::Model::Hp8920B {
            "B.99.99"
        } else {
            "A.99.99"
        }
        .into();
        radio(model, later, &empty)?;
        let mut unknown = valid.clone();
        unknown.commissioning = rf::Commissioning::Unknown;
        invalid(radio(model, unknown, &empty));
        let mut unknown = valid.clone();
        unknown.attenuator = rf::Attenuator::Unknown;
        invalid(radio(model, unknown, &empty));
        let mut wrong = valid.clone();
        wrong.model = if model == rf::Model::Hp8920A {
            rf::Model::Hp8920B
        } else {
            rf::Model::Hp8920A
        };
        invalid(radio(model, wrong, &empty));
        for option in [
            rf::InputPower::Unknown,
            rf::InputPower::Option006,
            rf::InputPower::Option007,
            rf::InputPower::Option008,
            rf::InputPower::Option016,
        ] {
            let mut c = valid.clone();
            c.input_power = option;
            let allowed = match model {
                rf::Model::Hp8920A => matches!(
                    option,
                    rf::InputPower::Option007
                        | rf::InputPower::Option008
                        | rf::InputPower::Option016
                ),
                rf::Model::Hp8920B => matches!(
                    option,
                    rf::InputPower::Option006
                        | rf::InputPower::Option007
                        | rf::InputPower::Option016
                ),
                rf::Model::Hp8921A => false,
            };
            if allowed {
                radio(model, c, &empty)?;
            } else {
                invalid(radio(model, c, &empty));
            }
        }
        if model == rf::Model::Hp8920B {
            for option in [rf::InputPower::Option006, rf::InputPower::Option007] {
                let mut c = valid.clone();
                c.attenuator = rf::Attenuator::Mechanical055;
                c.input_power = option;
                invalid(radio(model, c, &empty));
            }
        }
        let duplicate = rf::Binding {
            endpoint: endpoint("fixture"),
            configuration: valid.clone(),
        };
        invalid(rf::Factory::new(
            model,
            vec![duplicate.clone(), duplicate],
            Arc::new(empty.clone()),
        ));
        let mut ant = valid.clone();
        ant.receiver_input = rf::ReceiverInput::Antenna;
        let routes = vec![
            rf::Binding {
                endpoint: endpoint("fixture"),
                configuration: valid.clone(),
            },
            rf::Binding {
                endpoint: endpoint("ant"),
                configuration: ant,
            },
        ];
        let factory = rf::Factory::new(model, routes, Arc::new(empty.clone()))?;
        let registry = DriverRegistry::new(vec![Arc::new(factory) as Arc<dyn DriverFactory>])?;
        let required =
            BTreeSet::from([Operation::Catalog(CatalogOperation::RfReceiverMeasurePower)]);
        let ant_candidate = Candidate {
            endpoint: endpoint("ant"),
            identity: Identity::default(),
        };
        let selection = Selection {
            driver_id: Some(model.driver_id().into()),
            ..Default::default()
        };
        assert!(
            matches!(registry.open(&ant_candidate,&selection,&required,ExecutionMode::Sync,ConnectionOwnership::Owned),Err(e) if e.kind == ErrorKind::UnsupportedOperation)
        );
        let missing = Candidate {
            endpoint: endpoint("not-bound"),
            identity: Identity::default(),
        };
        invalid(registry.open(
            &missing,
            &selection,
            &BTreeSet::new(),
            ExecutionMode::Sync,
            ConnectionOwnership::Owned,
        ));
        assert_eq!(empty.opens(), 0);
        let candidate = Candidate {
            endpoint: endpoint("fixture"),
            identity: Identity {
                manufacturer: Some("Hewlett-Packard".into()),
                model: Some(model.name().into()),
                ..Default::default()
            },
        };
        assert_eq!(
            registry
                .select(&candidate, &Selection::default())?
                .driver_id,
            model.driver_id()
        );
        let handle = registry.open(
            &candidate,
            &selection,
            &BTreeSet::new(),
            ExecutionMode::Sync,
            ConnectionOwnership::Borrowed,
        )?;
        handle.close(false)?;
        handle.close(false)?;
        assert_eq!(empty.closes(), 0);
        empty.assert_complete()?;
    }
    let empty = Script::new([]);
    let base = daq::Configuration {
        option001: true,
        normal_dvm_routing_confirmed: true,
        inputs: vec![daq::Input {
            channel: 7,
            slot: 0,
            local: 0,
            module: daq::Module::Option010,
            compensation: daq::Compensation::None,
        }],
    };
    let make = |configuration| {
        daq::Factory::new(
            vec![daq::Binding {
                endpoint: endpoint("fixture"),
                configuration,
            }],
            Arc::new(empty.clone()),
        )
    };
    for (module, compensation, local) in [
        (daq::Module::Unknown, daq::Compensation::None, 0),
        (daq::Module::Option020, daq::Compensation::Software, 19),
        (daq::Module::Option010, daq::Compensation::Hardware, 0),
        (daq::Module::Option020, daq::Compensation::None, 0),
    ] {
        let mut c = base.clone();
        c.inputs[0].module = module;
        c.inputs[0].compensation = compensation;
        c.inputs[0].local = local;
        invalid(make(c));
    }
    let mut c = base.clone();
    c.option001 = false;
    invalid(make(c));
    let mut c = base.clone();
    c.normal_dvm_routing_confirmed = false;
    invalid(make(c));
    let mut c = base.clone();
    c.inputs.push(c.inputs[0].clone());
    invalid(make(c));
    let mut c = base.clone();
    c.inputs[0].slot = 5;
    invalid(make(c));
    let mut c = base.clone();
    c.inputs[0].channel = 0;
    invalid(make(c));
    assert_eq!(empty.opens(), 0);
    for id in [
        "keysight.hp3497a|D|proof",
        "keysight.hp8920a|R:E:Standard:RfInOut:RfInOut|proof",
        "keysight.hp8920b|R:E:Standard:RfInOut:RfInOut|proof",
        "keysight.hp8921a|R:E:Standard:RfInOut:RfInOut|proof",
    ] {
        let daq = id.contains("3497");
        for kind in [ErrorKind::Timeout, ErrorKind::Transport, ErrorKind::Parse] {
            let first = if daq {
                ScriptStep::Query(
                    "VT4AE0VC0VS0VF1VN1VW0SO1VR5AI0".into(),
                    if kind == ErrorKind::Parse {
                        Ok("bad".into())
                    } else {
                        Err(DriverError::new(kind, "injected"))
                    },
                )
            } else if kind == ErrorKind::Parse {
                // Parse occurs at first verification; no CONFIG command may follow.
                ScriptStep::Write("DISP RFG".into())
            } else {
                ScriptStep::WriteFailure("DISP RFG".into(), DriverError::new(kind, "injected"))
            };
            let mut steps = vec![first];
            if !daq && kind == ErrorKind::Parse {
                steps.extend([
                    ScriptStep::Write("RFG:AMPL:STAT OFF".into()),
                    ScriptStep::Query("RFG:AMPL:STAT?".into(), Ok("bad".into())),
                ]);
            }
            steps.push(ScriptStep::Close(Ok(())));
            let script = Script::new(steps);
            let factory = super::hp_fixture::factory(id, &script, false)?.unwrap();
            let registry = DriverRegistry::new(vec![factory])?;
            let candidate = Candidate {
                endpoint: endpoint("fixture"),
                identity: Identity::default(),
            };
            let selection = Selection {
                driver_id: Some(id.split('|').next().unwrap().into()),
                ..Default::default()
            };
            let handle = registry.open(
                &candidate,
                &selection,
                &BTreeSet::new(),
                ExecutionMode::Sync,
                ConnectionOwnership::Owned,
            )?;
            let request = if daq {
                CatalogRequest::DataAcquisitionMeasureVoltageDc {
                    channel: 7,
                    range: None,
                }
            } else {
                CatalogRequest::RfReceiverMeasureFrequency
            };
            assert_eq!(handle.invoke(request.clone()).unwrap_err().kind, kind);
            assert_eq!(
                handle.invoke(request).unwrap_err().kind,
                ErrorKind::SessionNeedsRecovery
            );
            assert_eq!(
                handle.recover().unwrap_err().kind,
                ErrorKind::SessionNeedsRecovery
            );
            handle.close(false)?;
            handle.close(false)?;
            script.assert_complete()?;
            assert_eq!(script.closes(), 1);
        }
    }
    println!("PASS HP3497A/8920A/8920B/8921A procedural inventory, firmware, route capabilities, ownership and fail-fast quarantine");
    Ok(())
}
