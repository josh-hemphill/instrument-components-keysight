//! Admission, healthy rejection and quarantine proofs for the actual new factories.
use crate::{generated_cases, qualified_fixture};
use instrument_api::{catalog::*, *};
use instrument_driver_keysight::{hp34980 as m, hp3585 as a, qualified_scpi as q};
use instrument_runtime::*;
use instrument_testkit::*;
use std::{collections::BTreeSet, sync::Arc};
const IDS: [&str; 5] = [
    "keysight.34401a",
    "keysight.33120a",
    "keysight.53132a",
    "keysight.hp3585b",
    "keysight.34980a",
];
fn candidate(id: &str) -> Candidate {
    Candidate {
        endpoint: Endpoint {
            scheme: if id == IDS[3] { "hpib" } else { "scpi" }.into(),
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
fn is_new(id: &str) -> bool {
    IDS.contains(&id.split('|').next().unwrap())
}
fn good(id: &str) -> CatalogCase {
    generated_cases::cases()
        .into_iter()
        .find(|c| c.id.split('|').next() == Some(id) && c.expected.is_some())
        .unwrap()
}
fn factory(
    index: usize,
    ends: Vec<Endpoint>,
    confirmed: bool,
    script: &Script,
) -> Result<Arc<dyn DriverFactory>> {
    Ok(match index {
        0..=2 => {
            let model = [
                q::Model::Agilent34401A,
                q::Model::Agilent33120A,
                q::Model::Agilent53132A,
            ][index];
            let commissioning = if !confirmed {
                q::Commissioning::Unknown
            } else if index == 1 {
                q::Commissioning::ContinuousSine50OhmVppOutputChangesAuthorized
            } else {
                q::Commissioning::GpibScpiConfirmed
            };
            Arc::new(q::Factory::new(
                model,
                ends.into_iter()
                    .map(|endpoint| q::Binding {
                        endpoint,
                        commissioning,
                    })
                    .collect(),
                Arc::new(script.clone()),
            )?)
        }
        3 => Arc::new(a::Factory::new(
            ends.into_iter()
                .map(|endpoint| a::Binding {
                    endpoint,
                    commissioning: if confirmed {
                        a::Commissioning::QualifiedNativeScalarConfirmed
                    } else {
                        a::Commissioning::Unknown
                    },
                })
                .collect(),
            Arc::new(script.clone()),
        )?),
        _ => Arc::new(m::Factory::new(
            ends.into_iter()
                .map(|endpoint| m::Binding {
                    endpoint,
                    configuration: m::Configuration {
                        commissioning: if confirmed {
                            m::Commissioning::NoAutonomousActionsConfirmed
                        } else {
                            m::Commissioning::Unknown
                        },
                        switches: vec![m::SwitchChannel {
                            channel: 7,
                            slot: 1,
                            local: 1,
                            module: m::Module::Hp34937A,
                        }],
                        ports: vec![],
                    },
                })
                .collect(),
            Arc::new(script.clone()),
        )?),
    })
}
fn invalid(result: Result<Arc<dyn DriverFactory>>) {
    assert!(matches!(result,Err(e)if e.kind==ErrorKind::InvalidArgument));
}
fn admission() -> Result<()> {
    for (index, id) in IDS.into_iter().enumerate() {
        let script = Script::new([]);
        let endpoint = candidate(id).endpoint;
        invalid(factory(index, vec![endpoint.clone()], false, &script));
        invalid(factory(index, vec![], true, &script));
        for scheme in [
            "wrong".to_owned(),
            format!("\u{a0}{}", endpoint.scheme),
            format!("{}\u{a0}", endpoint.scheme),
            format!("\u{2003}{}\u{2003}", endpoint.scheme),
        ] {
            invalid(factory(
                index,
                vec![Endpoint {
                    scheme,
                    ..endpoint.clone()
                }],
                true,
                &script,
            ));
        }
        invalid(factory(
            index,
            vec![Endpoint {
                address: " \t".into(),
                ..endpoint.clone()
            }],
            true,
            &script,
        ));
        invalid(factory(
            index,
            vec![
                endpoint.clone(),
                Endpoint {
                    scheme: format!(" \t{} ", endpoint.scheme.to_ascii_uppercase()),
                    ..endpoint.clone()
                },
            ],
            true,
            &script,
        ));
        let endpoints: Vec<_> = (0..4096)
            .map(|i| Endpoint {
                address: format!("route{i}"),
                ..endpoint.clone()
            })
            .collect();
        assert!(factory(index, endpoints.clone(), true, &script).is_ok());
        let mut too_many = endpoints;
        too_many.push(endpoint.clone());
        invalid(factory(index, too_many, true, &script));
        let f = factory(index, vec![endpoint.clone()], true, &script)?;
        assert!(!f.execution_modes().contains(&ExecutionMode::NativeAsync));
        let registry = DriverRegistry::new(vec![f.clone()])?;
        for rule in f.descriptor().rules {
            let MatchRule::ExactIdentity {
                scheme,
                manufacturer,
                model,
            } = rule
            else {
                panic!("Unexpected non-exact identity")
            };
            let mut c = candidate(id);
            c.endpoint.scheme = scheme;
            c.identity.manufacturer = Some(manufacturer);
            c.identity.model = Some(model);
            assert_eq!(registry.select(&c, &Selection::default())?.driver_id, id);
        }
        let mut unbound = candidate(id);
        unbound.endpoint.address = "unbound".into();
        assert!(
            matches!(registry.open(&unbound,&selection(id),&BTreeSet::new(),ExecutionMode::Sync,ConnectionOwnership::Owned),Err(e)if e.kind==ErrorKind::InvalidArgument)
        );
        let required = BTreeSet::from([Operation::Catalog(CatalogOperation::Shutdown)]);
        assert!(
            matches!(registry.open(&candidate(id),&selection(id),&required,ExecutionMode::Sync,ConnectionOwnership::Owned),Err(e)if e.kind==ErrorKind::UnsupportedOperation)
        );
        assert_eq!(script.opens(), 0);
        assert!(script.events().is_empty());
    }
    Ok(())
}
fn bad_requests(id: &str) -> Vec<CatalogRequest> {
    let mut result = vec![];
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        match id {
            "keysight.34401a" => {
                for request in [
                    CatalogRequest::DmmMeasureVoltageDc { range: Some(value) },
                    CatalogRequest::DmmMeasureVoltageAc { range: Some(value) },
                    CatalogRequest::DmmMeasureCurrentDc { range: Some(value) },
                    CatalogRequest::DmmMeasureCurrentAc { range: Some(value) },
                    CatalogRequest::DmmMeasureResistance { range: Some(value) },
                    CatalogRequest::DmmMeasureResistance2Wire { range: Some(value) },
                    CatalogRequest::DmmMeasureResistance4Wire { range: Some(value) },
                ] {
                    result.push(request)
                }
            }
            "keysight.33120a" => result.extend([
                CatalogRequest::FunctionGeneratorSetFrequency { hz: value },
                CatalogRequest::FunctionGeneratorSetAmplitude { vpp: value },
                CatalogRequest::FunctionGeneratorSetOffset { volts: value },
            ]),
            "keysight.53132a" => result.push(CatalogRequest::CounterSetGateTime { seconds: value }),
            "keysight.hp3585b" => result.extend([
                CatalogRequest::SpectrumAnalyzerSetCenterFrequency { hz: value },
                CatalogRequest::SpectrumAnalyzerSetSpan { hz: value },
                CatalogRequest::SpectrumAnalyzerSetRbw { hz: value },
                CatalogRequest::SpectrumAnalyzerSetVbw { hz: value },
                CatalogRequest::SpectrumAnalyzerSetRefLevel { dbm: value },
            ]),
            _ => {}
        }
    }
    if id == "keysight.34980a" {
        result.extend([
            CatalogRequest::DigitalIoWriteByte {
                port: 13,
                value: 256,
            },
            CatalogRequest::SwitchCloseChannel { channel: 0 },
            CatalogRequest::DigitalIoReadByte { port: 13 },
        ]);
    }
    result
}
fn invoke_case(handle: &DeviceHandle, case: &CatalogCase) -> Result<()> {
    for (i, request) in case.calls.iter().enumerate() {
        let value = handle.invoke(request.clone());
        if i + 1 == case.calls.len() {
            assert_eq!(value?, case.expected.clone().unwrap(), "{}", case.id);
        } else {
            value?;
        }
    }
    Ok(())
}
pub fn synchronous() -> Result<()> {
    admission()?;
    inventory()?;
    frozen_inventory_and_counter_intent()?;
    generator_failure_positions()?;
    // New-device integration beyond the golden corpus: invalid inputs must leave
    // the actual handle usable, not merely return the right initial error.
    for id in IDS {
        let case = good(id);
        let script = Script::new(case.script.clone());
        let f = qualified_fixture::factory(id, &script, false)?.unwrap();
        let registry = DriverRegistry::new(vec![f])?;
        let handle = registry.open(
            &candidate(id),
            &selection(id),
            &BTreeSet::new(),
            ExecutionMode::Sync,
            ConnectionOwnership::Owned,
        )?;
        let events = script.events().len();
        for request in bad_requests(id) {
            assert_eq!(
                handle.invoke(request).unwrap_err().kind,
                ErrorKind::InvalidArgument
            );
            assert_eq!(script.events().len(), events);
        }
        invoke_case(&handle, &case)?;
        handle.close(false)?;
        script.assert_complete()?;
        assert_eq!(script.closes(), 1);
    }
    let normals: std::collections::BTreeMap<_, _> =
        IDS.into_iter().map(|id| (id, good(id))).collect();
    // Apply follow-up health assertions to every independent new-model error case.
    for case in generated_cases::cases()
        .into_iter()
        .filter(|c| is_new(c.id) && c.expected_error.is_some())
    {
        let id = case.id.split('|').next().unwrap();
        let normal = &normals[id];
        let had_io = case
            .script
            .iter()
            .any(|s| !matches!(s, ScriptStep::Close(_)));
        let mut steps: Vec<_> = case
            .script
            .iter()
            .filter(|s| !matches!(s, ScriptStep::Close(_)))
            .cloned()
            .collect();
        if !had_io {
            steps.extend(normal.script.clone());
        } else {
            steps.push(ScriptStep::Close(Ok(())));
        }
        let script = Script::new(steps);
        let f = qualified_fixture::factory(id, &script, false)?.unwrap();
        let registry = DriverRegistry::new(vec![f])?;
        let handle = registry.open(
            &candidate(id),
            &selection(id),
            &BTreeSet::new(),
            ExecutionMode::Sync,
            ConnectionOwnership::Owned,
        )?;
        for (i, request) in case.calls.iter().enumerate() {
            let value = handle.invoke(request.clone());
            if i + 1 == case.calls.len() {
                assert_eq!(
                    value.unwrap_err().kind,
                    case.expected_error.unwrap(),
                    "{}",
                    case.id
                );
            } else {
                value?;
            }
        }
        if had_io {
            let events = script.events().len();
            let followup = if id == "keysight.34980a" {
                CatalogRequest::SwitchCloseChannel { channel: 7 }
            } else {
                normal.calls[0].clone()
            };
            assert_eq!(
                handle.invoke(followup).unwrap_err().kind,
                ErrorKind::SessionNeedsRecovery,
                "{}",
                case.id
            );
            assert_eq!(script.events().len(), events);
        } else {
            if id == "keysight.53132a" {
                handle.invoke(CatalogRequest::CounterSelectChannel { channel: 1 })?;
                handle.invoke(CatalogRequest::CounterSetGateTime { seconds: 0.1 })?;
            }
            invoke_case(&handle, normal)?;
        }
        handle.close(false)?;
        script.assert_complete()?;
        assert_eq!(script.closes(), 1);
    }
    for id in IDS {
        let case = good(id);
        let script = Script::new(
            case.script
                .iter()
                .filter(|s| !matches!(s, ScriptStep::Close(_)))
                .cloned(),
        );
        let f = qualified_fixture::factory(id, &script, false)?.unwrap();
        let registry = DriverRegistry::new(vec![f])?;
        let handle = registry.open(
            &candidate(id),
            &selection(id),
            &BTreeSet::new(),
            ExecutionMode::Sync,
            ConnectionOwnership::Borrowed,
        )?;
        invoke_case(&handle, &case)?;
        handle.close(false)?;
        handle.close(false)?;
        assert_eq!(
            handle.invoke(case.calls[0].clone()).unwrap_err().kind,
            ErrorKind::SessionClosed
        );
        script.assert_complete()?;
        assert_eq!(script.closes(), 0);
    }
    println!("PASS new Agilent factories admission/ASCII schemes,4096 bounds, nonfinite pre-IO usable follow-up, all golden error quarantine/health follow-ups and borrowed/owned lifetime");
    Ok(())
}

// Extend the independent successful amplitude/offset transcripts with host
// failures at every position, including counterpart reads and the final write.
fn generator_failure_positions() -> Result<()> {
    let cases = generated_cases::cases();
    for operation in [
        CatalogOperation::FunctionGeneratorSetAmplitude,
        CatalogOperation::FunctionGeneratorSetOffset,
    ] {
        let case = cases
            .iter()
            .find(|c| {
                c.id.starts_with("keysight.33120a|")
                    && c.expected.is_some()
                    && c.calls.len() == 1
                    && c.calls[0].operation() == operation
            })
            .unwrap();
        for position in 0..case.script.len() - 1 {
            for kind in [ErrorKind::Transport, ErrorKind::Timeout] {
                let error = DriverError::new(kind, "injected host failure");
                let failure = match &case.script[position] {
                    ScriptStep::Write(message) => ScriptStep::WriteFailure(message.clone(), error),
                    ScriptStep::Query(message, _) => ScriptStep::Query(message.clone(), Err(error)),
                    _ => panic!("non-IO transaction position"),
                };
                let mut steps = case.script[..position].to_vec();
                steps.extend([failure, ScriptStep::Close(Ok(()))]);
                let script = Script::new(steps);
                let factory = qualified_fixture::factory(IDS[1], &script, false)?.unwrap();
                let handle = DriverRegistry::new(vec![factory])?.open(
                    &candidate(IDS[1]),
                    &selection(IDS[1]),
                    &BTreeSet::new(),
                    ExecutionMode::Sync,
                    ConnectionOwnership::Owned,
                )?;
                assert_eq!(handle.invoke(case.calls[0].clone()).unwrap_err().kind, kind);
                let events = script.events().len();
                assert_eq!(
                    FunctionGenerator(&handle)
                        .read_frequency()
                        .unwrap_err()
                        .kind,
                    ErrorKind::SessionNeedsRecovery
                );
                assert_eq!(script.events().len(), events);
                handle.close(false)?;
                script.assert_complete()?;
                assert_eq!(script.closes(), 1);
            }
        }
    }
    Ok(())
}
fn frozen_inventory_and_counter_intent() -> Result<()> {
    let cases: std::collections::BTreeMap<_, _> = generated_cases::cases()
        .into_iter()
        .map(|c| (c.id, c))
        .collect();
    let reading = &cases["keysight.34980a|E|34950a-read-7-0"];
    let script = Script::new(reading.script.clone());
    let mut switches = vec![m::SwitchChannel {
        channel: 7,
        slot: 1,
        local: 1,
        module: m::Module::Hp34937A,
    }];
    let mut ports = vec![m::DigitalPort {
        port: 7,
        slot: 3,
        local: 101,
        direction: m::Direction::Input,
    }];
    let mut bindings = vec![m::Binding {
        endpoint: candidate(IDS[4]).endpoint,
        configuration: m::Configuration {
            commissioning: m::Commissioning::NoAutonomousActionsConfirmed,
            switches: switches.clone(),
            ports: ports.clone(),
        },
    }];
    let factory = m::Factory::new(bindings.clone(), Arc::new(script.clone()))?;
    switches.clear();
    ports[0].direction = m::Direction::Output;
    bindings.clear();
    let registry = DriverRegistry::new(vec![Arc::new(factory) as Arc<dyn DriverFactory>])?;
    let handle = registry.open(
        &candidate(IDS[4]),
        &selection(IDS[4]),
        &BTreeSet::new(),
        ExecutionMode::Sync,
        ConnectionOwnership::Owned,
    )?;
    invoke_case(&handle, reading)?;
    handle.close(false)?;
    script.assert_complete()?;
    let normal = &cases["keysight.53132a|E|Frequency-channel-1"];
    let intended = &cases["keysight.53132a|E|switch-intent-and-repeat"];
    let mut steps = normal.script.clone();
    steps.extend(intended.script.clone());
    let script = Script::new(steps);
    let factory = qualified_fixture::factory(IDS[2], &script, false)?.unwrap();
    let registry = DriverRegistry::new(vec![factory])?;
    let first = registry.open(
        &candidate(IDS[2]),
        &selection(IDS[2]),
        &BTreeSet::new(),
        ExecutionMode::Sync,
        ConnectionOwnership::Owned,
    )?;
    first.invoke(intended.calls[0].clone())?;
    first.invoke(intended.calls[1].clone())?;
    for request in [
        CatalogRequest::CounterSelectChannel { channel: 3 },
        CatalogRequest::CounterSetGateTime { seconds: f64::NAN },
        CatalogRequest::CounterSetGateTime {
            seconds: 0.010000000000000002,
        },
    ] {
        assert_eq!(
            first.invoke(request).unwrap_err().kind,
            ErrorKind::InvalidArgument
        );
    }
    let second = registry.open(
        &candidate(IDS[2]),
        &selection(IDS[2]),
        &BTreeSet::new(),
        ExecutionMode::Sync,
        ConnectionOwnership::Owned,
    )?;
    invoke_case(&second, normal)?;
    second.close(false)?;
    for request in &intended.calls[2..] {
        first.invoke(request.clone())?;
    }
    first.close(false)?;
    script.assert_complete()?;
    assert_eq!(script.opens(), 2);
    assert_eq!(script.closes(), 2);
    Ok(())
}

#[cfg(feature = "async")]
pub async fn selected_route_admission() -> Result<()> {
    let script = Script::new([]);
    let config = |switches, ports| m::Configuration {
        commissioning: m::Commissioning::NoAutonomousActionsConfirmed,
        switches,
        ports,
    };
    let endpoints = [
        (
            "relay",
            config(
                vec![m::SwitchChannel {
                    channel: 7,
                    slot: 1,
                    local: 1,
                    module: m::Module::Hp34937A,
                }],
                vec![],
            ),
        ),
        (
            "input",
            config(
                vec![],
                vec![m::DigitalPort {
                    port: 7,
                    slot: 3,
                    local: 101,
                    direction: m::Direction::Input,
                }],
            ),
        ),
        (
            "output",
            config(
                vec![],
                vec![m::DigitalPort {
                    port: 13,
                    slot: 3,
                    local: 104,
                    direction: m::Direction::Output,
                }],
            ),
        ),
    ];
    let factory = m::Factory::new(
        endpoints
            .into_iter()
            .map(|(address, configuration)| m::Binding {
                endpoint: Endpoint {
                    scheme: "scpi".into(),
                    address: address.into(),
                },
                configuration,
            })
            .collect(),
        Arc::new(script.clone()),
    )?
    .with_native_connector(Arc::new(script.clone()));
    let registry = DriverRegistry::new(vec![Arc::new(factory) as Arc<dyn DriverFactory>])?;
    for (address, operation) in [
        ("relay", CatalogOperation::DigitalIoReadByte),
        ("relay", CatalogOperation::DigitalIoWriteByte),
        ("input", CatalogOperation::DigitalIoWriteByte),
        ("input", CatalogOperation::SwitchCloseChannel),
        ("output", CatalogOperation::DigitalIoReadByte),
        ("output", CatalogOperation::SwitchCloseChannel),
    ] {
        let mut c = candidate(IDS[4]);
        c.endpoint.address = address.into();
        let required = BTreeSet::from([Operation::Catalog(operation)]);
        assert!(
            matches!(registry.open(&c,&selection(IDS[4]),&required,ExecutionMode::Sync,ConnectionOwnership::Owned),Err(e)if e.kind==ErrorKind::UnsupportedOperation)
        );
        assert!(
            matches!(registry.open_native(&c,&selection(IDS[4]),&required,ConnectionOwnership::Owned).await,Err(e)if e.kind==ErrorKind::UnsupportedOperation)
        );
        assert!(
            matches!(registry.open_worker(&c,&selection(IDS[4]),&required,ConnectionOwnership::Owned).await,Err(e)if e.kind==ErrorKind::UnsupportedOperation)
        );
    }
    for id in IDS {
        let f = qualified_fixture::factory(id, &script, false)?.unwrap();
        let registry = DriverRegistry::new(vec![f])?;
        assert!(
            matches!(registry.open_native(&candidate(id),&selection(id),&BTreeSet::new(),ConnectionOwnership::Owned).await,Err(e)if e.kind==ErrorKind::UnsupportedExecutionMode)
        );
    }
    assert_eq!(script.opens(), 0);
    assert!(script.events().is_empty());
    println!("PASS selected mainframe endpoint capabilities before sync/native/worker acquisition; native absence; immutable inventory and isolated counter intent");
    Ok(())
}
fn inventory() -> Result<()> {
    let script = Script::new([]);
    let build = |switches, ports, commissioning| {
        m::Factory::new(
            vec![m::Binding {
                endpoint: candidate(IDS[4]).endpoint,
                configuration: m::Configuration {
                    commissioning,
                    switches,
                    ports,
                },
            }],
            Arc::new(script.clone()),
        )
    };
    let relay = m::SwitchChannel {
        channel: 7,
        slot: 1,
        local: 1,
        module: m::Module::Hp34937A,
    };
    let port = m::DigitalPort {
        port: 7,
        slot: 3,
        local: 101,
        direction: m::Direction::Input,
    };
    for slot in 1..=8 {
        for (module, max) in [(m::Module::Hp34937A, 32), (m::Module::Hp34922A, 70)] {
            for local in 1..=max {
                assert!(build(
                    vec![m::SwitchChannel {
                        slot,
                        local,
                        module,
                        ..relay.clone()
                    }],
                    vec![],
                    m::Commissioning::NoAutonomousActionsConfirmed
                )
                .is_ok());
            }
        }
        for local in [101, 102, 103, 104, 201, 202, 203, 204] {
            for direction in [m::Direction::Input, m::Direction::Output] {
                assert!(build(
                    vec![],
                    vec![m::DigitalPort {
                        slot,
                        local,
                        direction,
                        ..port.clone()
                    }],
                    m::Commissioning::NoAutonomousActionsConfirmed
                )
                .is_ok());
            }
        }
    }
    for bad in [
        m::SwitchChannel {
            slot: 0,
            ..relay.clone()
        },
        m::SwitchChannel {
            slot: 9,
            ..relay.clone()
        },
        m::SwitchChannel {
            channel: 0,
            ..relay.clone()
        },
        m::SwitchChannel {
            local: 0,
            ..relay.clone()
        },
        m::SwitchChannel {
            local: 33,
            ..relay.clone()
        },
        m::SwitchChannel {
            module: m::Module::Unknown,
            ..relay.clone()
        },
        m::SwitchChannel {
            module: m::Module::Hp34922A,
            local: 71,
            ..relay.clone()
        },
    ] {
        assert!(
            matches!(build(vec![bad],vec![],m::Commissioning::NoAutonomousActionsConfirmed),Err(e)if e.kind==ErrorKind::InvalidArgument)
        );
    }
    for bad in [
        m::DigitalPort {
            slot: 0,
            ..port.clone()
        },
        m::DigitalPort {
            slot: 9,
            ..port.clone()
        },
        m::DigitalPort {
            port: 0,
            ..port.clone()
        },
        m::DigitalPort {
            local: 100,
            ..port.clone()
        },
        m::DigitalPort {
            local: 105,
            ..port.clone()
        },
        m::DigitalPort {
            local: 200,
            ..port.clone()
        },
        m::DigitalPort {
            local: 205,
            ..port.clone()
        },
        m::DigitalPort {
            direction: m::Direction::Unknown,
            ..port.clone()
        },
    ] {
        assert!(build(
            vec![],
            vec![bad],
            m::Commissioning::NoAutonomousActionsConfirmed
        )
        .is_err());
    }
    for switches in [
        vec![
            relay.clone(),
            m::SwitchChannel {
                channel: 7,
                local: 2,
                ..relay.clone()
            },
        ],
        vec![
            relay.clone(),
            m::SwitchChannel {
                channel: 13,
                ..relay.clone()
            },
        ],
        vec![
            relay.clone(),
            m::SwitchChannel {
                channel: 13,
                local: 2,
                module: m::Module::Hp34922A,
                ..relay.clone()
            },
        ],
    ] {
        assert!(build(
            switches,
            vec![],
            m::Commissioning::NoAutonomousActionsConfirmed
        )
        .is_err());
    }
    for ports in [
        vec![
            port.clone(),
            m::DigitalPort {
                port: 7,
                local: 102,
                ..port.clone()
            },
        ],
        vec![
            port.clone(),
            m::DigitalPort {
                port: 13,
                ..port.clone()
            },
        ],
    ] {
        assert!(build(
            vec![],
            ports,
            m::Commissioning::NoAutonomousActionsConfirmed
        )
        .is_err());
    }
    assert!(build(
        vec![relay.clone()],
        vec![m::DigitalPort { slot: 1, ..port }],
        m::Commissioning::NoAutonomousActionsConfirmed
    )
    .is_err());
    assert!(build(vec![relay], vec![], m::Commissioning::Unknown).is_err());
    assert!(build(
        vec![],
        vec![],
        m::Commissioning::NoAutonomousActionsConfirmed
    )
    .is_err());
    assert_eq!(script.opens(), 0);
    Ok(())
}
