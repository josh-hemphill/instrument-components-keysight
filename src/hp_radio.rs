//! Explicitly commissioned HP8920A/B and HP8921A scalar RF transactions.
use crate::message_backend::{self as tx, error, Engine, Step};
use instrument_api::{catalog::*, *};
use instrument_scpi::Connector;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Model {
    Hp8920A,
    Hp8920B,
    Hp8921A,
}
impl Model {
    pub fn name(self) -> &'static str {
        match self {
            Self::Hp8920A => "8920A",
            Self::Hp8920B => "8920B",
            Self::Hp8921A => "8921A",
        }
    }
    pub fn driver_id(self) -> &'static str {
        match self {
            Self::Hp8920A => "keysight.hp8920a",
            Self::Hp8920B => "keysight.hp8920b",
            Self::Hp8921A => "keysight.hp8921a",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attenuator {
    Unknown,
    Electronic,
    Mechanical055,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputPower {
    Unknown,
    Standard,
    Option006,
    Option007,
    Option008,
    Option016,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourcePort {
    Unknown,
    RfInOut,
    DuplexOut,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReceiverInput {
    Unknown,
    RfInOut,
    Antenna,
}
/// Confirms every prerequisite enumerated in the model specification: native GPIB,
/// initial Manual RX/TX, independent frequency display, inhibited external PTT,
/// exclusive settings access, qualified carrier, and bounded finite complete IO.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Commissioning {
    Unknown,
    Confirmed,
}
#[derive(Clone, Debug)]
pub struct Configuration {
    pub model: Model,
    pub firmware: String,
    pub attenuator: Attenuator,
    pub input_power: InputPower,
    pub source_port: SourcePort,
    pub receiver_input: ReceiverInput,
    pub commissioning: Commissioning,
}
#[derive(Clone, Debug)]
pub struct Binding {
    pub endpoint: Endpoint,
    pub configuration: Configuration,
}
#[derive(Clone)]
pub struct Factory {
    model: Model,
    routes: BTreeMap<(String, String), Configuration>,
    connector: Arc<dyn Connector>,
    #[cfg(feature = "async")]
    native: Option<Arc<dyn instrument_scpi::AsyncConnector>>,
}
fn invalid() -> DriverError {
    error(
        ErrorKind::InvalidArgument,
        "Unqualified model, firmware, inventory, endpoint or RF setting",
    )
}
impl Configuration {
    fn validate(&self, model: Model) -> Result<()> {
        use InputPower::*;
        if self.model != model
            || self.commissioning != Commissioning::Confirmed
            || self.attenuator == Attenuator::Unknown
            || self.source_port == SourcePort::Unknown
            || self.receiver_input == ReceiverInput::Unknown
        {
            return Err(invalid());
        }
        let allowed = match model {
            Model::Hp8920A => matches!(
                self.input_power,
                Standard | Option007 | Option008 | Option016
            ),
            Model::Hp8920B => {
                matches!(
                    self.input_power,
                    Standard | Option006 | Option007 | Option016
                ) && !(self.attenuator == Attenuator::Mechanical055
                    && matches!(self.input_power, Option006 | Option007))
            }
            Model::Hp8921A => self.input_power == Standard,
        };
        let (family, minimum) = match model {
            Model::Hp8920A => (b'A', 18),
            Model::Hp8920B => (b'B', 6),
            Model::Hp8921A => (b'A', 15),
        };
        let b = self.firmware.as_bytes();
        if !allowed
            || b.len() != 7
            || b[0] != family
            || b[1] != b'.'
            || b[4] != b'.'
            || ![b[2], b[3], b[5], b[6]].iter().all(u8::is_ascii_digit)
            || (b[2] - b'0') * 10 + b[3] - b'0' < minimum
        {
            return Err(invalid());
        }
        Ok(())
    }
    fn operations(&self) -> BTreeSet<Operation> {
        use CatalogOperation::*;
        let mut ops = BTreeSet::from(
            [
                RfSignalGeneratorConfigure,
                RfSignalGeneratorOutputEnable,
                RfSignalGeneratorOutputStateQuery,
                RfReceiverMeasureFrequency,
            ]
            .map(Operation::Catalog),
        );
        if self.receiver_input == ReceiverInput::RfInOut {
            ops.insert(Operation::Catalog(RfReceiverMeasurePower));
        }
        ops
    }
    fn source_frequency(&self) -> (f64, f64) {
        (
            if self.attenuator == Attenuator::Electronic {
                30_000_000.0
            } else {
                250_000.0
            },
            1_000_000_000.0,
        )
    }
    fn source_level(&self) -> (f64, f64) {
        let electronic = self.attenuator == Attenuator::Electronic;
        if self.source_port == SourcePort::DuplexOut {
            return (
                -127.0,
                if electronic {
                    if self.model == Model::Hp8920B {
                        3.5
                    } else {
                        5.0
                    }
                } else {
                    7.0
                },
            );
        }
        let max = match self.input_power {
            InputPower::Standard => {
                if electronic {
                    -20.5
                } else {
                    -19.0
                }
            }
            InputPower::Option006 => -12.5,
            InputPower::Option007 => {
                if electronic {
                    -6.5
                } else {
                    -5.0
                }
            }
            InputPower::Option008 => {
                if electronic {
                    -10.5
                } else {
                    -9.0
                }
            }
            InputPower::Option016 => {
                if electronic || self.model == Model::Hp8920B {
                    -22.5
                } else {
                    -21.0
                }
            }
            InputPower::Unknown => unreachable!(),
        };
        (-137.0, max)
    }
    fn receiver_frequency(&self) -> (f64, f64) {
        (
            if self.attenuator == Attenuator::Mechanical055 {
                400_000.0
            } else if self.model == Model::Hp8920A {
                30_000_000.0
            } else {
                10_000_000.0
            },
            1_000_000_000.0,
        )
    }
    fn receiver_power(&self) -> (f64, f64) {
        // Frozen nearest-binary64 constants: spec/acceptance/power-bound-constants.json.
        match self.input_power {
            InputPower::Standard => (0.0, 47.78151250383644),
            InputPower::Option006 => (-13.010299956639813, 40.0),
            InputPower::Option007 => (-13.979400086720377, 33.802112417116064),
            InputPower::Option008 => (-10.0, 37.78151250383644),
            InputPower::Option016 => (2.041199826559248, 50.0),
            InputPower::Unknown => unreachable!(),
        }
    }
    fn source_token(&self) -> &'static str {
        if self.source_port == SourcePort::RfInOut {
            "RF Out"
        } else {
            "Dupl"
        }
    }
    fn receiver_token(&self) -> &'static str {
        if self.receiver_input == ReceiverInput::RfInOut {
            "RF In"
        } else {
            "Ant"
        }
    }
}
impl Factory {
    pub fn new(
        model: Model,
        bindings: Vec<Binding>,
        connector: Arc<dyn Connector>,
    ) -> Result<Self> {
        if bindings.is_empty() || bindings.len() > 4096 {
            return Err(invalid());
        }
        let mut routes = BTreeMap::new();
        for binding in bindings {
            tx::validate_endpoint(&binding.endpoint)?;
            binding.configuration.validate(model)?;
            if routes
                .insert(tx::endpoint_key(&binding.endpoint), binding.configuration)
                .is_some()
            {
                return Err(invalid());
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
        tx::validate_endpoint(&context.candidate.endpoint)?;
        let config = self
            .routes
            .get(&tx::endpoint_key(&context.candidate.endpoint))
            .ok_or_else(invalid)?;
        if !context.required.is_subset(&config.operations()) {
            return Err(error(
                ErrorKind::UnsupportedOperation,
                "Required operation unavailable on selected RF route",
            ));
        }
        Ok(Core {
            config: config.clone(),
            configured: false,
            previous: false,
        })
    }
}
impl DriverFactory for Factory {
    fn descriptor(&self) -> DriverDescriptor {
        tx::descriptor(
            self.model.driver_id(),
            self.model.name(),
            self.routes
                .values()
                .flat_map(Configuration::operations)
                .collect(),
        )
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
            let connector = self.native.as_ref().ok_or_else(|| {
                error(
                    ErrorKind::UnsupportedExecutionMode,
                    "Native connector not configured",
                )
            })?;
            Ok(Box::new(tx::AsyncDevice::new(
                core,
                connector.open(context).await?,
                context.ownership,
            )) as Box<dyn AsyncDeviceBackend>)
        })
    }
}

fn parse_error() -> DriverError {
    error(ErrorKind::Parse, "Malformed or implausible RF response")
}
fn mismatch() -> DriverError {
    error(
        ErrorKind::Device,
        "Instrument readback does not match requested setting",
    )
}
/// Normalized exact decimal representation, bounded by the complete-body limit.
struct Decimal {
    negative: bool,
    digits: Vec<u8>,
    point: i32,
}
impl Decimal {
    fn parse(text: &str) -> Result<Self> {
        let bytes = text.as_bytes();
        let mut i = 0;
        let mut negative = false;
        if matches!(bytes.first(), Some(b'+' | b'-')) {
            negative = bytes[0] == b'-';
            i += 1;
        }
        let mut digits = Vec::new();
        let mut integer = 0;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            digits.push(bytes[i]);
            integer += 1;
            i += 1;
        }
        if bytes.get(i) == Some(&b'.') {
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                digits.push(bytes[i]);
                i += 1;
            }
        }
        if digits.is_empty() {
            return Err(parse_error());
        }
        let mut exponent: i32 = 0;
        if matches!(bytes.get(i), Some(b'E' | b'e')) {
            i += 1;
            let mut sign = 1;
            if matches!(bytes.get(i), Some(b'+' | b'-')) {
                if bytes[i] == b'-' {
                    sign = -1;
                }
                i += 1;
            }
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                exponent = (exponent * 10 + i32::from(bytes[i] - b'0')).min(10_000);
                i += 1;
            }
            if i == start {
                return Err(parse_error());
            }
            exponent *= sign;
        }
        if i != bytes.len() {
            return Err(parse_error());
        }
        let leading = digits.iter().take_while(|b| **b == b'0').count();
        digits.drain(..leading);
        if digits.is_empty() {
            return Ok(Self {
                negative: false,
                digits: vec![b'0'],
                point: 1,
            });
        }
        while digits.last() == Some(&b'0') {
            digits.pop();
        }
        Ok(Self {
            negative,
            digits,
            point: integer - leading as i32 + exponent,
        })
    }
    fn cmp(&self, other: &Self) -> Ordering {
        let zero = self.digits == b"0";
        let other_zero = other.digits == b"0";
        if zero || other_zero {
            return if zero && other_zero {
                Ordering::Equal
            } else if zero {
                if other.negative {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else if self.negative {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }
        if self.negative != other.negative {
            return if self.negative {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }
        let mut cmp = self.point.cmp(&other.point);
        if cmp == Ordering::Equal {
            for i in 0..self.digits.len().max(other.digits.len()) {
                cmp = self
                    .digits
                    .get(i)
                    .unwrap_or(&b'0')
                    .cmp(other.digits.get(i).unwrap_or(&b'0'));
                if cmp != Ordering::Equal {
                    break;
                }
            }
        }
        if self.negative {
            cmp.reverse()
        } else {
            cmp
        }
    }
}
fn number(body: &str) -> Result<(f64, Decimal)> {
    let body = body.trim_matches(' ');
    let decimal = Decimal::parse(body)?;
    let value: f64 = body.parse().map_err(|_| parse_error())?;
    if !value.is_finite() {
        return Err(parse_error());
    }
    Ok((value, decimal))
}
fn hundredths(value: i64) -> Decimal {
    Decimal::parse(&format!(
        "{}{}.{:02}",
        if value < 0 { "-" } else { "" },
        value.abs() / 100,
        value.abs() % 100
    ))
    .expect("bounded integer formatter")
}
fn tenths(value: i64) -> String {
    format!(
        "{}{}{}",
        if value < 0 { "-" } else { "" },
        value.abs() / 10,
        if value % 10 == 0 {
            String::new()
        } else {
            format!(".{}", value.abs() % 10)
        }
    )
}
enum Check {
    State(Option<bool>),
    Token(&'static [&'static str], &'static str, Quoting),
    Setting {
        bounds: (f64, f64),
        expected100: i64,
        tolerance100: i64,
    },
    Measurement((f64, f64), Unit),
}
enum Quoting {
    Required,
    Bare,
    Either,
}
struct Core {
    config: Configuration,
    configured: bool,
    previous: bool,
}
fn write(steps: &mut Vec<Step<Check>>, text: impl Into<String>) {
    steps.push(Step::Write(text.into()));
}
fn query(steps: &mut Vec<Step<Check>>, text: impl Into<String>, check: Check) {
    steps.push(Step::Query(text.into(), check));
}
fn off(steps: &mut Vec<Step<Check>>) {
    write(steps, "DISP RFG");
    write(steps, "RFG:AMPL:STAT OFF");
    query(steps, "RFG:AMPL:STAT?", Check::State(Some(false)));
}
fn prefix(steps: &mut Vec<Step<Check>>) {
    off(steps);
    write(steps, "DISP CONF");
    write(steps, "CONF:ARTS 'Manual'");
    write(steps, "CONF:OMOD 'Off'");
    write(steps, "CONF:OFL:MODE 'Off'");
    query(
        steps,
        "CONF:ARTS?",
        Check::Token(&["MANUAL", "AUTO"], "MANUAL", Quoting::Required),
    );
    for text in ["CONF:OMOD?", "CONF:OFL:MODE?"] {
        query(
            steps,
            text,
            Check::Token(&["OFF", "ON"], "OFF", Quoting::Required),
        );
    }
}
impl Engine for Core {
    type Check = Check;
    fn operations(&self) -> BTreeSet<Operation> {
        self.config.operations()
    }
    fn begin(&mut self, _: &CatalogRequest) {
        self.previous = self.configured;
        self.configured = false;
    }
    fn commit(&mut self, request: &CatalogRequest) {
        self.configured = match request {
            CatalogRequest::RfSignalGeneratorConfigure { .. } => true,
            CatalogRequest::RfReceiverMeasureFrequency | CatalogRequest::RfReceiverMeasurePower => {
                false
            }
            _ => self.previous,
        };
    }
    fn prepare(&self, request: &CatalogRequest) -> Result<Vec<Step<Check>>> {
        let mut steps = Vec::new();
        match request {
            CatalogRequest::RfSignalGeneratorConfigure {
                frequency_hz,
                level_dbm,
            } => {
                let (frequency, level) = (*frequency_hz, *level_dbm);
                let fb = self.config.source_frequency();
                let lb = self.config.source_level();
                if !frequency.is_finite()
                    || !level.is_finite()
                    || frequency < fb.0
                    || frequency > fb.1
                    || frequency.fract() != 0.0
                    || level < lb.0
                    || level > lb.1
                {
                    return Err(invalid());
                }
                let decimal = Decimal::parse(&level.to_string())?;
                if decimal.digits.len() as i32 - decimal.point > 1 {
                    return Err(invalid());
                }
                let level10 = (level * 10.0).round() as i64;
                prefix(&mut steps);
                off(&mut steps);
                write(&mut steps, "RFG:ATT 'Off'");
                query(
                    &mut steps,
                    "RFG:ATT?",
                    Check::Token(&["OFF", "ON"], "OFF", Quoting::Required),
                );
                write(
                    &mut steps,
                    format!("RFG:OUTP '{}'", self.config.source_token()),
                );
                write(&mut steps, format!("RFG:FREQ {:.0} HZ", frequency));
                write(&mut steps, format!("RFG:AMPL {} DBM", tenths(level10)));
                write(&mut steps, "RFG:AMPL:UNIT DBM");
                query(
                    &mut steps,
                    "RFG:AMPL:UNIT?",
                    Check::Token(&["DBM", "W", "V", "DBUV"], "DBM", Quoting::Either),
                );
                write(&mut steps, "RFG:AMPL:STAT OFF");
                query(&mut steps, "RFG:AMPL:STAT?", Check::State(Some(false)));
                query(
                    &mut steps,
                    "RFG:OUTP?",
                    Check::Token(
                        &["RF OUT", "DUPL"],
                        if self.config.source_port == SourcePort::RfInOut {
                            "RF OUT"
                        } else {
                            "DUPL"
                        },
                        Quoting::Required,
                    ),
                );
                query(
                    &mut steps,
                    "RFG:FREQ?",
                    Check::Setting {
                        bounds: fb,
                        expected100: frequency as i64 * 100,
                        tolerance100: 50,
                    },
                );
                query(
                    &mut steps,
                    "RFG:AMPL?",
                    Check::Setting {
                        bounds: lb,
                        expected100: level10 * 10,
                        tolerance100: 5,
                    },
                );
            }
            CatalogRequest::RfSignalGeneratorOutputEnable { enabled } => {
                if *enabled && !self.configured {
                    return Err(invalid());
                }
                write(&mut steps, "DISP RFG");
                write(
                    &mut steps,
                    format!("RFG:AMPL:STAT {}", if *enabled { "ON" } else { "OFF" }),
                );
                query(&mut steps, "RFG:AMPL:STAT?", Check::State(Some(*enabled)));
            }
            CatalogRequest::RfSignalGeneratorOutputStateQuery => {
                write(&mut steps, "DISP RFG");
                query(&mut steps, "RFG:AMPL:STAT?", Check::State(None));
            }
            CatalogRequest::RfReceiverMeasureFrequency | CatalogRequest::RfReceiverMeasurePower => {
                let power = matches!(request, CatalogRequest::RfReceiverMeasurePower);
                if power && self.config.receiver_input != ReceiverInput::RfInOut {
                    return Err(error(
                        ErrorKind::UnsupportedOperation,
                        "Power unavailable on ANT",
                    ));
                }
                prefix(&mut steps);
                off(&mut steps);
                write(&mut steps, "DISP RFAN");
                write(
                    &mut steps,
                    format!("RFAN:INP '{}'", self.config.receiver_token()),
                );
                write(&mut steps, "RFAN:TMOD 'Auto'");
                query(
                    &mut steps,
                    "RFAN:INP?",
                    Check::Token(
                        &["RF IN", "ANT"],
                        if self.config.receiver_input == ReceiverInput::RfInOut {
                            "RF IN"
                        } else {
                            "ANT"
                        },
                        Quoting::Required,
                    ),
                );
                query(
                    &mut steps,
                    "RFAN:TMOD?",
                    Check::Token(&["AUTO", "MANUAL"], "AUTO", Quoting::Required),
                );
                let p = if power {
                    "MEAS:RFR:POW"
                } else {
                    "MEAS:RFR:FREQ:ABS"
                };
                if power {
                    write(&mut steps, format!("{p}:UNIT DBM"));
                    query(
                        &mut steps,
                        format!("{p}:UNIT?"),
                        Check::Token(&["DBM", "W"], "DBM", Quoting::Either),
                    );
                }
                for suffix in ["AVER:STAT", "REF:STAT"] {
                    write(&mut steps, format!("{p}:{suffix} OFF"));
                }
                for suffix in ["AVER:STAT", "REF:STAT"] {
                    query(
                        &mut steps,
                        format!("{p}:{suffix}?"),
                        Check::State(Some(false)),
                    );
                }
                write(&mut steps, format!("{p}:STAT ON"));
                query(&mut steps, format!("{p}:STAT?"), Check::State(Some(true)));
                write(&mut steps, "TRIG:MODE:SETT FULL");
                write(&mut steps, "TRIG:MODE:RETR SING");
                query(
                    &mut steps,
                    "TRIG:MODE:SETT?",
                    Check::Token(&["FULL", "FAST"], "FULL", Quoting::Bare),
                );
                query(
                    &mut steps,
                    "TRIG:MODE:RETR?",
                    Check::Token(&["SING", "REP"], "SING", Quoting::Bare),
                );
                write(&mut steps, "TRIG:IMM");
                query(
                    &mut steps,
                    format!("{p}?"),
                    Check::Measurement(
                        if power {
                            self.config.receiver_power()
                        } else {
                            self.config.receiver_frequency()
                        },
                        if power { Unit::Dbm } else { Unit::Hertz },
                    ),
                );
            }
            _ => {
                return Err(error(
                    ErrorKind::UnsupportedOperation,
                    "Operation unavailable for RF test set",
                ))
            }
        }
        Ok(steps)
    }
    fn decode(&self, check: Check, body: &str) -> Result<Option<CatalogReply>> {
        let body = body.trim_matches(' ');
        match check {
            Check::State(expected) => {
                let actual = match body {
                    "0" => false,
                    "1" => true,
                    _ => return Err(parse_error()),
                };
                if let Some(value) = expected {
                    if value != actual {
                        return Err(mismatch());
                    }
                    Ok(None)
                } else {
                    Ok(Some(CatalogReply::Boolean(actual)))
                }
            }
            Check::Token(domain, expected, quoting) => {
                let quoted = matches!(body.as_bytes().first(), Some(b'\'' | b'"'));
                if matches!(quoting, Quoting::Required) && !quoted
                    || matches!(quoting, Quoting::Bare) && quoted
                {
                    return Err(parse_error());
                }
                let token = if quoted {
                    if body.len() < 2 || body.as_bytes().first() != body.as_bytes().last() {
                        return Err(parse_error());
                    }
                    &body[1..body.len() - 1]
                } else {
                    body
                };
                let token = token.to_ascii_uppercase();
                if !domain.contains(&token.as_str()) {
                    return Err(parse_error());
                }
                if token != expected {
                    return Err(mismatch());
                }
                Ok(None)
            }
            Check::Setting {
                bounds,
                expected100,
                tolerance100,
            } => {
                let (value, decimal) = number(body)?;
                if value < bounds.0 || value > bounds.1 {
                    return Err(parse_error());
                }
                if decimal.cmp(&hundredths(expected100 - tolerance100)) == Ordering::Less
                    || decimal.cmp(&hundredths(expected100 + tolerance100)) == Ordering::Greater
                {
                    return Err(mismatch());
                }
                Ok(None)
            }
            Check::Measurement(bounds, unit) => {
                let (value, _) = number(body)?;
                if value < bounds.0 || value > bounds.1 {
                    return Err(parse_error());
                }
                Ok(Some(CatalogReply::Quantity(Quantity {
                    value,
                    unit,
                    status: MeasurementStatus::Valid,
                })))
            }
        }
    }
}
