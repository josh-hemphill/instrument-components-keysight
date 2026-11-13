//! Explicitly commissioned, own-model scalar subsets. No discovery or implicit reset.
use crate::message_backend::{self as tx, error, Engine, Step};
use instrument_api::{catalog::*, *};
use instrument_scpi::Connector;
use std::{collections::BTreeSet, sync::Arc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Model {
    Agilent34401A,
    Agilent33120A,
    Agilent53132A,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Commissioning {
    Unknown,
    GpibScpiConfirmed,
    ContinuousSine50OhmVppOutputChangesAuthorized,
}
#[derive(Clone, Debug)]
pub struct Binding {
    pub endpoint: Endpoint,
    pub commissioning: Commissioning,
}
#[derive(Clone)]
pub struct Factory {
    model: Model,
    routes: BTreeSet<(String, String)>,
    connector: Arc<dyn Connector>,
    #[cfg(feature = "async")]
    native: Option<Arc<dyn instrument_scpi::AsyncConnector>>,
}
fn endpoint(endpoint: &Endpoint) -> Result<()> {
    if !endpoint.scheme.is_ascii()
        || !endpoint.scheme.trim().eq_ignore_ascii_case("scpi")
        || endpoint.address.trim().is_empty()
    {
        return Err(error(
            ErrorKind::InvalidArgument,
            "Commissioned nonempty scpi endpoint required",
        ));
    }
    Ok(())
}
impl Model {
    fn identity(self) -> (&'static str, &'static str) {
        match self {
            Self::Agilent34401A => ("keysight.34401a", "34401A"),
            Self::Agilent33120A => ("keysight.33120a", "33120A"),
            Self::Agilent53132A => ("keysight.53132a", "53132A"),
        }
    }
    fn operations(self) -> BTreeSet<Operation> {
        use CatalogOperation::*;
        let values: &[CatalogOperation] = match self {
            Self::Agilent34401A => &[
                DmmMeasureVoltageDc,
                DmmMeasureVoltageAc,
                DmmMeasureCurrentDc,
                DmmMeasureCurrentAc,
                DmmMeasureResistance,
                DmmMeasureResistance2Wire,
                DmmMeasureResistance4Wire,
            ],
            Self::Agilent33120A => &[
                FunctionGeneratorSetFrequency,
                FunctionGeneratorSetAmplitude,
                FunctionGeneratorSetOffset,
                FunctionGeneratorReadFrequency,
            ],
            Self::Agilent53132A => &[
                CounterSelectChannel,
                CounterSetGateTime,
                CounterMeasureFrequency,
                CounterMeasurePeriod,
            ],
        };
        values.iter().copied().map(Operation::Catalog).collect()
    }
}
impl Factory {
    pub fn new(
        model: Model,
        bindings: Vec<Binding>,
        connector: Arc<dyn Connector>,
    ) -> Result<Self> {
        if bindings.is_empty() || bindings.len() > 4096 {
            return Err(error(ErrorKind::InvalidArgument, "Bind 1..4096 endpoints"));
        }
        let expected = if model == Model::Agilent33120A {
            Commissioning::ContinuousSine50OhmVppOutputChangesAuthorized
        } else {
            Commissioning::GpibScpiConfirmed
        };
        let mut routes = BTreeSet::new();
        for binding in bindings {
            endpoint(&binding.endpoint)?;
            if binding.commissioning != expected
                || !routes.insert(tx::endpoint_key(&binding.endpoint))
            {
                return Err(error(
                    ErrorKind::InvalidArgument,
                    "Explicit matching commissioning and unique endpoints required",
                ));
            }
        }
        Ok(Self {
            model,
            routes,
            connector,
            #[cfg(feature = "async")]
            native: None,
        })
    }
    #[cfg(feature = "async")]
    pub fn with_native_connector(
        mut self,
        connector: Arc<dyn instrument_scpi::AsyncConnector>,
    ) -> Self {
        self.native = Some(connector);
        self
    }
    fn selected(&self, context: &OpenContext<'_>) -> Result<Core> {
        endpoint(&context.candidate.endpoint)?;
        if !self
            .routes
            .contains(&tx::endpoint_key(&context.candidate.endpoint))
        {
            return Err(error(
                ErrorKind::InvalidArgument,
                "Unbound commissioned endpoint",
            ));
        }
        if !context.required.is_subset(&self.model.operations()) {
            return Err(error(
                ErrorKind::UnsupportedOperation,
                "Required operation unavailable",
            ));
        }
        Ok(Core {
            model: self.model,
            channel: 1,
            gate_ms: 100,
        })
    }
}
impl DriverFactory for Factory {
    fn descriptor(&self) -> DriverDescriptor {
        let (id, model) = self.model.identity();
        let aliases: &[&str] = if self.model == Model::Agilent33120A {
            &["HP", "HEWLETT-PACKARD", "Agilent Technologies"]
        } else {
            &[
                "HP",
                "HEWLETT-PACKARD",
                "Agilent Technologies",
                "Keysight Technologies",
            ]
        };
        DriverDescriptor {
            id: id.into(),
            contract_major: CONTRACT_MAJOR,
            rules: aliases
                .iter()
                .map(|m| MatchRule::ExactIdentity {
                    scheme: "scpi".into(),
                    manufacturer: (*m).into(),
                    model: model.into(),
                })
                .collect(),
            operations: self.model.operations(),
        }
    }
    fn execution_modes(&self) -> BTreeSet<ExecutionMode> {
        #[cfg(feature = "async")]
        let native = self.native.is_some();
        #[cfg(not(feature = "async"))]
        let native = false;
        tx::modes(native)
    }
    fn open(&self, context: &OpenContext<'_>) -> Result<Box<dyn DeviceBackend>> {
        let core = self.selected(context)?;
        Ok(Box::new(tx::Device::new(
            core,
            self.connector.open(context)?,
            context.ownership,
        )))
    }
    #[cfg(feature = "async")]
    fn open_native<'a>(
        &'a self,
        context: &'a OpenContext<'_>,
    ) -> DriverFuture<'a, Box<dyn AsyncDeviceBackend>> {
        Box::pin(async move {
            let core = self.selected(context)?;
            let native = self.native.as_ref().ok_or_else(|| {
                error(
                    ErrorKind::UnsupportedExecutionMode,
                    "Native connector not configured",
                )
            })?;
            Ok(Box::new(tx::AsyncDevice::new(
                core,
                native.open(context).await?,
                context.ownership,
            )) as Box<dyn AsyncDeviceBackend>)
        })
    }
}

// Same standard E11 algorithm qualified by the native HP counter's midpoint vectors.
pub(crate) fn real12(value: f64) -> String {
    let raw = format!("{value:.11E}");
    let (m, e) = raw.split_once('E').expect("scientific formatter");
    format!("{m}E{:+03}", e.parse::<i32>().expect("scientific exponent"))
}
pub(crate) fn decimal(body: &str) -> Result<f64> {
    let bytes = body.as_bytes();
    let mut i = 0;
    if matches!(bytes.first(), Some(b'+') | Some(b'-')) {
        i += 1;
    }
    let start = i;
    while bytes.get(i).is_some_and(u8::is_ascii_digit) {
        i += 1;
    }
    if i == start {
        return Err(error(ErrorKind::Parse, "Expected decimal digits"));
    }
    if bytes.get(i) == Some(&b'.') {
        i += 1;
        let start = i;
        while bytes.get(i).is_some_and(u8::is_ascii_digit) {
            i += 1;
        }
        if i == start {
            return Err(error(ErrorKind::Parse, "Fraction digits required"));
        }
    }
    if matches!(bytes.get(i), Some(b'E') | Some(b'e')) {
        i += 1;
        if matches!(bytes.get(i), Some(b'+') | Some(b'-')) {
            i += 1;
        }
        let start = i;
        while bytes.get(i).is_some_and(u8::is_ascii_digit) {
            i += 1;
        }
        if i == start {
            return Err(error(ErrorKind::Parse, "Exponent digits required"));
        }
    }
    if i != bytes.len() {
        return Err(error(ErrorKind::Parse, "Unexpected scalar payload"));
    }
    body.parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
        .ok_or_else(|| error(ErrorKind::Parse, "Nonfinite scalar"))
}
fn bounded(body: &str) -> Result<f64> {
    let value = decimal(body)?;
    if value.abs() >= 1e30 {
        return Err(error(ErrorKind::Parse, "Implausible scalar"));
    }
    Ok(value)
}
fn quantity(value: f64, unit: Unit) -> Option<CatalogReply> {
    Some(CatalogReply::Quantity(Quantity {
        value,
        unit,
        status: MeasurementStatus::Valid,
    }))
}
fn within(value: f64, min: f64, max: f64) -> bool {
    value.is_finite() && (min..=max).contains(&value)
}
fn pair(vpp: f64, offset: f64) -> bool {
    offset.abs() + vpp / 2.0 <= 5.0 && offset.abs() <= 2.0 * vpp
}
fn gate_ms(seconds: f64) -> Result<u32> {
    if within(seconds, 0.001, 1000.0) {
        let value = (seconds * 1000.0).round() as u32;
        if seconds == f64::from(value) / 1000.0 {
            return Ok(value);
        }
    }
    Err(error(
        ErrorKind::InvalidArgument,
        "Gate requires 1..1000000 integer milliseconds",
    ))
}
struct Core {
    model: Model,
    channel: u32,
    gate_ms: u32,
}
enum Check {
    Dmm(Unit),
    Counter(Unit),
    Shape,
    Load,
    Vpp,
    Disabled,
    Frequency,
    OffsetForAmplitude(f64, f64),
    AmplitudeForOffset(f64, f64),
}
fn guards() -> Vec<Step<Check>> {
    let mut steps = vec![
        Step::Query(":FUNC:SHAP?".into(), Check::Shape),
        Step::Query(":OUTP:LOAD?".into(), Check::Load),
        Step::Query(":VOLT:UNIT?".into(), Check::Vpp),
    ];
    for command in [
        ":AM:STAT?",
        ":FM:STAT?",
        ":BM:STAT?",
        ":FSK:STAT?",
        ":SWE:STAT?",
    ] {
        steps.push(Step::Query(command.into(), Check::Disabled));
    }
    steps
}
impl Engine for Core {
    type Check = Check;
    fn operations(&self) -> BTreeSet<Operation> {
        self.model.operations()
    }
    fn prepare(&self, request: &CatalogRequest) -> Result<Vec<Step<Check>>> {
        if !self
            .operations()
            .contains(&Operation::Catalog(request.operation()))
        {
            return Err(error(
                ErrorKind::UnsupportedOperation,
                "Operation outside qualified subset",
            ));
        }
        use CatalogRequest::*;
        if self.model == Model::Agilent34401A {
            let (root, range, max, unit) = match request {
                DmmMeasureVoltageDc { range } => (":MEAS:VOLT:DC?", range, 1000.0, Unit::Volt),
                DmmMeasureVoltageAc { range } => (":MEAS:VOLT:AC?", range, 750.0, Unit::Volt),
                DmmMeasureCurrentDc { range } => (":MEAS:CURR:DC?", range, 3.0, Unit::Ampere),
                DmmMeasureCurrentAc { range } => (":MEAS:CURR:AC?", range, 3.0, Unit::Ampere),
                DmmMeasureResistance { range } | DmmMeasureResistance2Wire { range } => {
                    (":MEAS:RES?", range, 1e8, Unit::Ohm)
                }
                DmmMeasureResistance4Wire { range } => (":MEAS:FRES?", range, 1e8, Unit::Ohm),
                _ => unreachable!(),
            };
            if range.is_some_and(|v| !within(v, f64::from_bits(1), max)) {
                return Err(error(
                    ErrorKind::InvalidArgument,
                    "Range outside qualified software bound",
                ));
            }
            let value = range.map(real12).unwrap_or_else(|| "DEF".into());
            return Ok(vec![Step::Query(
                format!("{root} {value},DEF"),
                Check::Dmm(unit),
            )]);
        }
        if self.model == Model::Agilent53132A {
            let (function, unit) = match request {
                CounterSelectChannel { channel } => {
                    if !matches!(channel, 1 | 2) {
                        return Err(error(ErrorKind::InvalidArgument, "Only channels1/2"));
                    }
                    return Ok(vec![]);
                }
                CounterSetGateTime { seconds } => {
                    gate_ms(*seconds)?;
                    return Ok(vec![]);
                }
                CounterMeasureFrequency => ("FREQ", Unit::Hertz),
                CounterMeasurePeriod => ("PER", Unit::Second),
                _ => unreachable!(),
            };
            return Ok(vec![
                Step::Write(format!(":CONF:{function} DEF,DEF,(@{})", self.channel)),
                Step::Write(":SENS:FREQ:ARM:STAR:SOUR IMM".into()),
                Step::Write(":SENS:FREQ:ARM:STOP:SOUR TIM".into()),
                Step::Write(format!(
                    ":SENS:FREQ:ARM:STOP:TIM {}.{:03}",
                    self.gate_ms / 1000,
                    self.gate_ms % 1000
                )),
                Step::Write(":FORM ASC".into()),
                Step::Query(":READ?".into(), Check::Counter(unit)),
            ]);
        }
        // Validate structural requests before even the read-only guard prefix.
        let tail = match request {
            FunctionGeneratorSetFrequency { hz } => {
                if !within(*hz, 0.0001, 15e6) {
                    return Err(error(
                        ErrorKind::InvalidArgument,
                        "Frequency outside qualified sine range",
                    ));
                }
                vec![Step::Write(format!(":FREQ {}", real12(*hz)))]
            }
            FunctionGeneratorSetAmplitude { vpp } => {
                if !within(*vpp, 0.05, 10.0) {
                    return Err(error(
                        ErrorKind::InvalidArgument,
                        "Amplitude outside50Ohm Vpp range",
                    ));
                }
                let text = real12(*vpp);
                let rounded = decimal(&text)?;
                vec![
                    Step::Query(
                        ":VOLT:OFFS?".into(),
                        Check::OffsetForAmplitude(*vpp, rounded),
                    ),
                    Step::Write(format!(":VOLT {text}")),
                ]
            }
            FunctionGeneratorSetOffset { volts } => {
                if !within(*volts, -4.0, 4.0) {
                    return Err(error(
                        ErrorKind::InvalidArgument,
                        "Offset outside coupled envelope",
                    ));
                }
                let text = real12(*volts);
                let rounded = decimal(&text)?;
                vec![
                    Step::Query(":VOLT?".into(), Check::AmplitudeForOffset(*volts, rounded)),
                    Step::Write(format!(":VOLT:OFFS {text}")),
                ]
            }
            FunctionGeneratorReadFrequency => vec![Step::Query(":FREQ?".into(), Check::Frequency)],
            _ => unreachable!(),
        };
        let mut steps = guards();
        steps.extend(tail);
        Ok(steps)
    }
    fn decode(&self, check: Check, body: &str) -> Result<Option<CatalogReply>> {
        let device = || {
            error(
                ErrorKind::Device,
                "Device setting/result outside commissioned contract",
            )
        };
        let parse = || error(ErrorKind::Parse, "Unexpected scalar grammar");
        match check {
            Check::Dmm(unit) => {
                let b = body.as_bytes();
                if b.len() != 15
                    || !matches!(b[0], b'+' | b'-')
                    || !b[1].is_ascii_digit()
                    || b[2] != b'.'
                    || !b[3..11].iter().all(u8::is_ascii_digit)
                    || b[11] != b'E'
                    || !matches!(b[12], b'+' | b'-')
                    || !b[13..15].iter().all(u8::is_ascii_digit)
                {
                    return Err(parse());
                }
                if body == "+9.90000000E+37" {
                    return Ok(Some(CatalogReply::Quantity(Quantity {
                        value: 0.0,
                        unit,
                        status: MeasurementStatus::Overload,
                    })));
                }
                Ok(quantity(bounded(body)?, unit))
            }
            Check::Counter(unit) => {
                let split = body.find(['E', 'e']).ok_or_else(parse)?;
                let mantissa = &body[..split];
                let exponent = body[split + 1..].trim_start_matches(['+', '-']);
                if exponent.is_empty()
                    || exponent.len() > 3
                    || !exponent.bytes().all(|b| b.is_ascii_digit())
                {
                    return Err(parse());
                }
                let digits: Vec<_> = mantissa.bytes().filter(u8::is_ascii_digit).collect();
                let significant = digits
                    .iter()
                    .position(|b| *b != b'0')
                    .map(|i| digits.len() - i)
                    .unwrap_or(1);
                if significant > 15 {
                    return Err(parse());
                }
                let value = decimal(body)?;
                if value <= 0.0 || value == 9.91e37 {
                    return Err(device());
                }
                Ok(quantity(value, unit))
            }
            Check::Shape => {
                if body == "SIN" {
                    Ok(None)
                } else if ["SQU", "TRI", "RAMP", "NOIS", "DC", "USER"].contains(&body) {
                    Err(device())
                } else {
                    Err(parse())
                }
            }
            Check::Vpp => {
                if body == "VPP" {
                    Ok(None)
                } else if ["VRMS", "DBM"].contains(&body) {
                    Err(device())
                } else {
                    Err(parse())
                }
            }
            Check::Disabled => match body {
                "0" => Ok(None),
                "1" => Err(device()),
                _ => Err(parse()),
            },
            Check::Load => {
                let value = decimal(body)?;
                if value == 9.9e37 {
                    return Err(device());
                }
                if value.abs() >= 1e30 {
                    return Err(parse());
                }
                if value <= 0.0 {
                    return Err(parse());
                }
                if value != 50.0 {
                    return Err(device());
                }
                Ok(None)
            }
            Check::Frequency => {
                let value = bounded(body)?;
                if !within(value, 0.0001, 15e6) {
                    return Err(device());
                }
                Ok(quantity(value, Unit::Hertz))
            }
            Check::OffsetForAmplitude(original, rounded) => {
                let offset = bounded(body)?;
                if !within(offset, -4.0, 4.0) {
                    return Err(device());
                }
                if !pair(original, offset) || !pair(rounded, offset) {
                    return Err(error(
                        ErrorKind::InvalidArgument,
                        "Requested amplitude conflicts with current offset",
                    ));
                }
                Ok(None)
            }
            Check::AmplitudeForOffset(original, rounded) => {
                let amplitude = bounded(body)?;
                if !within(amplitude, 0.05, 10.0) {
                    return Err(device());
                }
                if !pair(amplitude, original) || !pair(amplitude, rounded) {
                    return Err(error(
                        ErrorKind::InvalidArgument,
                        "Requested offset conflicts with current amplitude",
                    ));
                }
                Ok(None)
            }
        }
    }
    fn commit(&mut self, request: &CatalogRequest) {
        if self.model == Model::Agilent53132A {
            match request {
                CatalogRequest::CounterSelectChannel { channel } => self.channel = *channel,
                CatalogRequest::CounterSetGateTime { seconds } => {
                    self.gate_ms = gate_ms(*seconds).expect("validated gate")
                }
                _ => {}
            }
        }
    }
}
