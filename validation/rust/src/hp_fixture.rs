//! Only test metadata selects these independently specified commissioning records.
use instrument_api::*;
use instrument_driver_keysight::{hp3497 as daq, hp_radio as rf};
use instrument_testkit::Script;
use std::sync::Arc;
pub fn factory(id: &str, script: &Script, native: bool) -> Result<Option<Arc<dyn DriverFactory>>> {
    let endpoint = Endpoint {
        scheme: "hpib".into(),
        address: "fixture".into(),
    };
    if id.starts_with("keysight.hp3497a|") {
        let inputs = [
            (7, 0, 0, daq::Module::Option010, daq::Compensation::None),
            (13, 0, 19, daq::Module::Option010, daq::Compensation::None),
            (
                29,
                1,
                18,
                daq::Module::Option020,
                daq::Compensation::Software,
            ),
            (61, 4, 19, daq::Module::Option010, daq::Compensation::None),
        ]
        .map(|(channel, slot, local, module, compensation)| daq::Input {
            channel,
            slot,
            local,
            module,
            compensation,
        })
        .to_vec();
        let factory = daq::Factory::new(
            vec![daq::Binding {
                endpoint,
                configuration: daq::Configuration {
                    option001: true,
                    normal_dvm_routing_confirmed: true,
                    inputs,
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
        #[cfg(not(feature = "async"))]
        let _ = native;
        return Ok(Some(Arc::new(factory)));
    }
    let model = match id.split('|').next().unwrap() {
        "keysight.hp8920a" => rf::Model::Hp8920A,
        "keysight.hp8920b" => rf::Model::Hp8920B,
        "keysight.hp8921a" => rf::Model::Hp8921A,
        _ => return Ok(None),
    };
    let fields: Vec<_> = id
        .split('|')
        .nth(1)
        .expect("route fixture")
        .split(':')
        .collect();
    let config = rf::Configuration {
        model,
        firmware: match model {
            rf::Model::Hp8920A => "A.18.00",
            rf::Model::Hp8920B => "B.06.00",
            rf::Model::Hp8921A => "A.15.00",
        }
        .into(),
        attenuator: match fields[1] {
            "E" => rf::Attenuator::Electronic,
            "M" => rf::Attenuator::Mechanical055,
            _ => panic!("fixture attenuator"),
        },
        input_power: match fields[2] {
            "Standard" => rf::InputPower::Standard,
            "Option006" => rf::InputPower::Option006,
            "Option007" => rf::InputPower::Option007,
            "Option008" => rf::InputPower::Option008,
            "Option016" => rf::InputPower::Option016,
            _ => panic!("fixture option"),
        },
        source_port: match fields[3] {
            "RfInOut" => rf::SourcePort::RfInOut,
            "DuplexOut" => rf::SourcePort::DuplexOut,
            _ => panic!("fixture source"),
        },
        receiver_input: match fields[4] {
            "RfInOut" => rf::ReceiverInput::RfInOut,
            "Antenna" => rf::ReceiverInput::Antenna,
            _ => panic!("fixture receiver"),
        },
        commissioning: rf::Commissioning::Confirmed,
    };
    let factory = rf::Factory::new(
        model,
        vec![rf::Binding {
            endpoint,
            configuration: config,
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
