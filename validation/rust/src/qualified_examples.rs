//! Compiled hardware-free composition examples. Scripts are synthetic; the
//! application replaces Script with its commissioned complete-message connector.
use instrument_api::{catalog::*, *};
use instrument_driver_keysight::{hp34980 as m, hp3585 as a, qualified_scpi as q};
use instrument_runtime::*;
use instrument_testkit::*;
use std::{collections::BTreeSet, sync::Arc};
fn endpoint(scheme: &str) -> Endpoint {
    Endpoint {
        scheme: scheme.into(),
        address: "fixture".into(),
    }
}
fn open(factory: Arc<dyn DriverFactory>, scheme: &str) -> Result<DeviceHandle> {
    let selection = Selection {
        driver_id: Some(factory.descriptor().id),
        ..Selection::default()
    };
    DriverRegistry::new(vec![factory])?.open(
        &Candidate {
            endpoint: endpoint(scheme),
            identity: Identity::default(),
        },
        &selection,
        &BTreeSet::new(),
        ExecutionMode::Sync,
        ConnectionOwnership::Owned,
    )
}
pub fn run() -> Result<()> {
    let script = Script::new([
        ScriptStep::Query(
            ":MEAS:VOLT:DC? DEF,DEF".into(),
            Ok("+1.25000000E+00".into()),
        ),
        ScriptStep::Close(Ok(())),
    ]);
    let factory = q::Factory::new(
        q::Model::Agilent34401A,
        vec![q::Binding {
            endpoint: endpoint("scpi"),
            commissioning: q::Commissioning::GpibScpiConfirmed,
        }],
        Arc::new(script.clone()),
    )?;
    let handle = open(Arc::new(factory), "scpi")?;
    assert_eq!(Dmm(&handle).measure_voltage_dc(None)?.value, 1.25);
    handle.close(false)?;
    script.assert_complete()?;

    // Reading a frequency setting makes no promise of electrical output-off.
    // Changes require ContinuousSine50OhmVppOutputChangesAuthorized commissioning.
    let script = Script::new([
        ScriptStep::Query(":FUNC:SHAP?".into(), Ok("SIN".into())),
        ScriptStep::Query(":OUTP:LOAD?".into(), Ok("+5.000000E+01".into())),
        ScriptStep::Query(":VOLT:UNIT?".into(), Ok("VPP".into())),
        ScriptStep::Query(":AM:STAT?".into(), Ok("0".into())),
        ScriptStep::Query(":FM:STAT?".into(), Ok("0".into())),
        ScriptStep::Query(":BM:STAT?".into(), Ok("0".into())),
        ScriptStep::Query(":FSK:STAT?".into(), Ok("0".into())),
        ScriptStep::Query(":SWE:STAT?".into(), Ok("0".into())),
        ScriptStep::Query(":FREQ?".into(), Ok("+1.000000E+03".into())),
        ScriptStep::Close(Ok(())),
    ]);
    let factory = q::Factory::new(
        q::Model::Agilent33120A,
        vec![q::Binding {
            endpoint: endpoint("scpi"),
            commissioning: q::Commissioning::ContinuousSine50OhmVppOutputChangesAuthorized,
        }],
        Arc::new(script.clone()),
    )?;
    let handle = open(Arc::new(factory), "scpi")?;
    assert_eq!(FunctionGenerator(&handle).read_frequency()?.value, 1000.0);
    handle.close(false)?;
    script.assert_complete()?;

    let script = Script::new([
        ScriptStep::Write(":CONF:FREQ DEF,DEF,(@1)".into()),
        ScriptStep::Write(":SENS:FREQ:ARM:STAR:SOUR IMM".into()),
        ScriptStep::Write(":SENS:FREQ:ARM:STOP:SOUR TIM".into()),
        ScriptStep::Write(":SENS:FREQ:ARM:STOP:TIM 0.100".into()),
        ScriptStep::Write(":FORM ASC".into()),
        ScriptStep::Query(":READ?".into(), Ok("+1.25000000000E+06".into())),
        ScriptStep::Close(Ok(())),
    ]);
    let factory = q::Factory::new(
        q::Model::Agilent53132A,
        vec![q::Binding {
            endpoint: endpoint("scpi"),
            commissioning: q::Commissioning::GpibScpiConfirmed,
        }],
        Arc::new(script.clone()),
    )?;
    let handle = open(Arc::new(factory), "scpi")?;
    Counter(&handle).select_channel(1)?;
    Counter(&handle).set_gate_time(0.100)?;
    assert_eq!(Counter(&handle).measure_frequency()?.value, 1_250_000.0);
    handle.close(false)?;
    script.assert_complete()?;

    // All modules share one parent resource. Opening a FormC relay selects NC;
    // it does not isolate its common terminal. No reset/open-all is implicit.
    let script = Script::new([
        ScriptStep::Write(":ROUT:CLOS (@1001)".into()),
        ScriptStep::Query(":ROUT:CLOS? (@2001)".into(), Ok("0".into())),
        ScriptStep::Close(Ok(())),
    ]);
    let factory = m::Factory::new(
        vec![m::Binding {
            endpoint: endpoint("scpi"),
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
                        channel: 101,
                        slot: 2,
                        local: 1,
                        module: m::Module::Hp34922A,
                    },
                ],
                ports: vec![m::DigitalPort {
                    port: 13,
                    slot: 3,
                    local: 104,
                    direction: m::Direction::Output,
                }],
            },
        }],
        Arc::new(script.clone()),
    )?;
    let handle = open(Arc::new(factory), "scpi")?;
    Switch(&handle).close_channel(7)?;
    assert!(!Switch(&handle).channel_is_closed(101)?);
    // DigitalIo(&handle).write_byte(13, 0)? explicitly changes the output after
    // verifying the module's commissioned byte/electrical state.
    handle.close(false)?;
    script.assert_complete()?;

    // 3585B requires qualified serial3008A00860+, R12/+30dBm50ohm/liveA/dBm.
    // MarkerPeak selects the stored trace; it does not sweep a fresh full trace.
    let script = Script::new([
        ScriptStep::Write("CF1000000HZ".into()),
        ScriptStep::Close(Ok(())),
    ]);
    let factory = a::Factory::new(
        vec![a::Binding {
            endpoint: endpoint("hpib"),
            commissioning: a::Commissioning::QualifiedNativeScalarConfirmed,
        }],
        Arc::new(script.clone()),
    )?;
    let handle = open(Arc::new(factory), "hpib")?;
    SpectrumAnalyzer(&handle).set_center_frequency(1_000_000.0)?;
    handle.close(false)?;
    script.assert_complete()?;
    println!("PASS compiled Agilent composition examples: five factories, shared module parent, typed views");
    Ok(())
}
