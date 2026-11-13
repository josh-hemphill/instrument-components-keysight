use instrument_api::*;
use instrument_driver_keysight::qualified_scpi as q;
use instrument_scpi::Connector;
#[cfg(feature = "async")]
pub trait FixtureHost: Connector + instrument_scpi::AsyncConnector + Clone + 'static {}
#[cfg(feature = "async")]
impl<T: Connector + instrument_scpi::AsyncConnector + Clone + 'static> FixtureHost for T {}
#[cfg(not(feature = "async"))]
pub trait FixtureHost: Connector + Clone + 'static {}
#[cfg(not(feature = "async"))]
impl<T: Connector + Clone + 'static> FixtureHost for T {}
use std::sync::Arc;
pub fn factory<H: FixtureHost>(
    id: &str,
    script: &H,
    native: bool,
) -> Result<Option<Arc<dyn DriverFactory>>> {
    if id.split('|').next() == Some("keysight.hp3585b") {
        use instrument_driver_keysight::hp3585 as a;
        let factory = a::Factory::new(
            vec![a::Binding {
                endpoint: Endpoint {
                    scheme: "hpib".into(),
                    address: "fixture".into(),
                },
                commissioning: a::Commissioning::QualifiedNativeScalarConfirmed,
            }],
            Arc::new(script.clone()),
        )?;
        #[cfg(feature = "async")]
        let factory = if native {
            factory.with_native_connector(Arc::new(script.clone()))
        } else {
            factory
        };
        return Ok(Some(Arc::new(factory)));
    }
    if id.split('|').next() == Some("keysight.34980a") {
        use instrument_driver_keysight::hp34980 as m;
        let factory = m::Factory::new(
            vec![m::Binding {
                endpoint: Endpoint {
                    scheme: "scpi".into(),
                    address: "fixture".into(),
                },
                configuration: m::Configuration {
                    commissioning: m::Commissioning::NoAutonomousActionsConfirmed,
                    switches: vec![
                        m::SwitchChannel {
                            channel: 7,
                            slot: 1,
                            local: 1,
                            module: m::Module::Hp34937A,
                        },
                        m::SwitchChannel {
                            channel: 13,
                            slot: 1,
                            local: 28,
                            module: m::Module::Hp34937A,
                        },
                        m::SwitchChannel {
                            channel: 29,
                            slot: 1,
                            local: 29,
                            module: m::Module::Hp34937A,
                        },
                        m::SwitchChannel {
                            channel: 61,
                            slot: 1,
                            local: 32,
                            module: m::Module::Hp34937A,
                        },
                        m::SwitchChannel {
                            channel: 101,
                            slot: 2,
                            local: 1,
                            module: m::Module::Hp34922A,
                        },
                        m::SwitchChannel {
                            channel: 135,
                            slot: 2,
                            local: 35,
                            module: m::Module::Hp34922A,
                        },
                        m::SwitchChannel {
                            channel: 136,
                            slot: 2,
                            local: 36,
                            module: m::Module::Hp34922A,
                        },
                        m::SwitchChannel {
                            channel: 170,
                            slot: 2,
                            local: 70,
                            module: m::Module::Hp34922A,
                        },
                    ],
                    ports: vec![
                        m::DigitalPort {
                            port: 7,
                            slot: 3,
                            local: 101,
                            direction: m::Direction::Input,
                        },
                        m::DigitalPort {
                            port: 13,
                            slot: 3,
                            local: 104,
                            direction: m::Direction::Output,
                        },
                        m::DigitalPort {
                            port: 29,
                            slot: 3,
                            local: 201,
                            direction: m::Direction::Input,
                        },
                        m::DigitalPort {
                            port: 61,
                            slot: 3,
                            local: 204,
                            direction: m::Direction::Output,
                        },
                    ],
                },
            }],
            Arc::new(script.clone()),
        )?;
        #[cfg(feature = "async")]
        let factory = if native {
            factory.with_native_connector(Arc::new(script.clone()))
        } else {
            factory
        };
        return Ok(Some(Arc::new(factory)));
    }
    let model = match id.split('|').next().unwrap() {
        "keysight.34401a" => q::Model::Agilent34401A,
        "keysight.33120a" => q::Model::Agilent33120A,
        "keysight.53132a" => q::Model::Agilent53132A,
        _ => return Ok(None),
    };
    let commissioning = if model == q::Model::Agilent33120A {
        q::Commissioning::ContinuousSine50OhmVppOutputChangesAuthorized
    } else {
        q::Commissioning::GpibScpiConfirmed
    };
    let factory = q::Factory::new(
        model,
        vec![q::Binding {
            endpoint: Endpoint {
                scheme: "scpi".into(),
                address: "fixture".into(),
            },
            commissioning,
        }],
        Arc::new(script.clone()),
    )?;
    #[cfg(feature = "async")]
    let factory = if native {
        factory.with_native_connector(Arc::new(script.clone()))
    } else {
        factory
    };
    #[cfg(not(feature = "async"))]
    let _ = native;
    Ok(Some(Arc::new(factory)))
}
