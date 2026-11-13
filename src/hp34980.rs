//! Mainframe-owned channel switching and commissioned static byte IO.
use crate::message_backend::{self as tx, error, Engine, Step};
use instrument_api::{catalog::*, *};
use instrument_scpi::Connector;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Module {
    Unknown,
    Hp34937A,
    Hp34922A,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Unknown,
    Input,
    Output,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Commissioning {
    Unknown,
    NoAutonomousActionsConfirmed,
}
#[derive(Clone, Debug)]
pub struct SwitchChannel {
    pub channel: u32,
    pub slot: u8,
    pub local: u16,
    pub module: Module,
}
#[derive(Clone, Debug)]
pub struct DigitalPort {
    pub port: u32,
    pub slot: u8,
    pub local: u16,
    pub direction: Direction,
}
#[derive(Clone, Debug)]
pub struct Configuration {
    pub commissioning: Commissioning,
    pub switches: Vec<SwitchChannel>,
    pub ports: Vec<DigitalPort>,
}
#[derive(Clone, Debug)]
pub struct Binding {
    pub endpoint: Endpoint,
    pub configuration: Configuration,
}
#[derive(Clone)]
pub struct Factory {
    routes: BTreeMap<(String, String), Core>,
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
            "Nonempty commissioned scpi endpoint required",
        ));
    }
    Ok(())
}
impl Factory {
    pub fn new(bindings: Vec<Binding>, connector: Arc<dyn Connector>) -> Result<Self> {
        if bindings.is_empty() || bindings.len() > 4096 {
            return Err(error(ErrorKind::InvalidArgument, "Bind1..4096 mainframes"));
        }
        let mut routes = BTreeMap::new();
        for binding in bindings {
            endpoint(&binding.endpoint)?;
            let core = Core::new(binding.configuration)?;
            if routes
                .insert(tx::endpoint_key(&binding.endpoint), core)
                .is_some()
            {
                return Err(error(ErrorKind::InvalidArgument, "Duplicate endpoint"));
            }
        }
        Ok(Self {
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
        let core = self
            .routes
            .get(&tx::endpoint_key(&context.candidate.endpoint))
            .ok_or_else(|| error(ErrorKind::InvalidArgument, "Unbound mainframe"))?;
        if !context.required.is_subset(&core.operations()) {
            return Err(error(
                ErrorKind::UnsupportedOperation,
                "Required operation unavailable on selected mainframe",
            ));
        }
        Ok(core.clone())
    }
}
impl DriverFactory for Factory {
    fn descriptor(&self) -> DriverDescriptor {
        DriverDescriptor {
            id: "keysight.34980a".into(),
            contract_major: CONTRACT_MAJOR,
            rules: ["Agilent Technologies", "Keysight Technologies"]
                .into_iter()
                .map(|m| MatchRule::ExactIdentity {
                    scheme: "scpi".into(),
                    manufacturer: m.into(),
                    model: "34980A".into(),
                })
                .collect(),
            operations: self.routes.values().flat_map(Engine::operations).collect(),
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
#[derive(Clone)]
struct Core {
    switches: BTreeMap<u32, u16>,
    ports: BTreeMap<u32, (u16, Direction)>,
}
impl Core {
    fn new(config: Configuration) -> Result<Self> {
        let bad = || {
            error(
                ErrorKind::InvalidArgument,
                "Require commissioned valid, unique slot/channel inventory",
            )
        };
        if config.commissioning != Commissioning::NoAutonomousActionsConfirmed
            || config.switches.len() > 560
            || config.ports.len() > 64
            || config.switches.is_empty() && config.ports.is_empty()
        {
            return Err(bad());
        }
        let mut core = Self {
            switches: BTreeMap::new(),
            ports: BTreeMap::new(),
        };
        let mut slots = BTreeMap::new();
        let mut physical = BTreeSet::new();
        for entry in config.switches {
            if entry.channel == 0
                || !(1..=8).contains(&entry.slot)
                || !match entry.module {
                    Module::Hp34937A => (1..=32).contains(&entry.local),
                    Module::Hp34922A => (1..=70).contains(&entry.local),
                    _ => false,
                }
            {
                return Err(bad());
            }
            let kind = if entry.module == Module::Hp34937A {
                1
            } else {
                2
            };
            if slots
                .insert(entry.slot, kind)
                .is_some_and(|old| old != kind)
            {
                return Err(bad());
            }
            let native = 1000 * u16::from(entry.slot) + entry.local;
            if !physical.insert(native) || core.switches.insert(entry.channel, native).is_some() {
                return Err(bad());
            }
        }
        for entry in config.ports {
            if entry.port == 0
                || !(1..=8).contains(&entry.slot)
                || !matches!(entry.local,101..=104|201..=204)
                || !matches!(entry.direction, Direction::Input | Direction::Output)
            {
                return Err(bad());
            }
            if slots.insert(entry.slot, 3).is_some_and(|old| old != 3) {
                return Err(bad());
            }
            let native = 1000 * u16::from(entry.slot) + entry.local;
            if !physical.insert(native)
                || core
                    .ports
                    .insert(entry.port, (native, entry.direction))
                    .is_some()
            {
                return Err(bad());
            }
        }
        Ok(core)
    }
}
enum Check {
    State,
    Width,
    Direction(Direction),
    Polarity,
    Handshake,
    Disabled,
    Threshold,
    Drive,
    Level,
    Byte,
}
fn query(command: &str, address: u16, check: Check) -> Step<Check> {
    Step::Query(format!("{command} (@{address})"), check)
}
impl Engine for Core {
    type Check = Check;
    fn operations(&self) -> BTreeSet<Operation> {
        let mut values = BTreeSet::new();
        if !self.switches.is_empty() {
            for op in [
                CatalogOperation::SwitchCloseChannel,
                CatalogOperation::SwitchOpenChannel,
                CatalogOperation::SwitchChannelIsClosed,
            ] {
                values.insert(Operation::Catalog(op));
            }
        }
        for (_, direction) in self.ports.values() {
            values.insert(Operation::Catalog(if *direction == Direction::Input {
                CatalogOperation::DigitalIoReadByte
            } else {
                CatalogOperation::DigitalIoWriteByte
            }));
        }
        values
    }
    fn prepare(&self, request: &CatalogRequest) -> Result<Vec<Step<Check>>> {
        use CatalogRequest::*;
        match request {
            SwitchCloseChannel { channel }
            | SwitchOpenChannel { channel }
            | SwitchChannelIsClosed { channel } => {
                let address = self
                    .switches
                    .get(channel)
                    .ok_or_else(|| error(ErrorKind::InvalidArgument, "Unbound switch channel"))?;
                Ok(vec![match request {
                    SwitchCloseChannel { .. } => Step::Write(format!(":ROUT:CLOS (@{address})")),
                    SwitchOpenChannel { .. } => Step::Write(format!(":ROUT:OPEN (@{address})")),
                    _ => query(":ROUT:CLOS?", *address, Check::State),
                }])
            }
            DigitalIoReadByte { port } | DigitalIoWriteByte { port, .. } => {
                let (address, direction) = *self
                    .ports
                    .get(port)
                    .ok_or_else(|| error(ErrorKind::InvalidArgument, "Unbound digital port"))?;
                let read = matches!(request, DigitalIoReadByte { .. });
                if read != (direction == Direction::Input) {
                    return Err(error(
                        ErrorKind::InvalidArgument,
                        "Operation unavailable for this frozen port direction",
                    ));
                }
                if let DigitalIoWriteByte { value, .. } = request {
                    if *value > 255 {
                        return Err(error(ErrorKind::InvalidArgument, "Byte must be0..255"));
                    }
                }
                let bank = address / 1000 * 1000 + if address % 1000 < 200 { 101 } else { 201 };
                let mut steps: Vec<_> = (bank..bank + 4)
                    .map(|a| query(":CONF:DIG:WIDT?", a, Check::Width))
                    .collect();
                steps.extend([
                    query(":CONF:DIG:DIR?", address, Check::Direction(direction)),
                    query(":CONF:DIG:POL?", address, Check::Polarity),
                    query(":CONF:DIG:HAND:STAT?", bank, Check::Handshake),
                    query(":SENS:DIG:MEM:ENAB?", bank, Check::Disabled),
                    query(":SOUR:DIG:MEM:ENAB?", bank, Check::Disabled),
                ]);
                if read {
                    steps.extend([
                        query(":SENS:DIG:THR?", address, Check::Threshold),
                        Step::Query(
                            format!(":SENS:DIG:DATA:BYTE? DEC,(@{address})"),
                            Check::Byte,
                        ),
                    ]);
                } else {
                    steps.extend([
                        query(":SOUR:DIG:DRIV?", address, Check::Drive),
                        query(":SOUR:DIG:LEV?", address, Check::Level),
                    ]);
                    if let DigitalIoWriteByte { value, .. } = request {
                        steps.push(Step::Write(format!(
                            ":SOUR:DIG:DATA:BYTE {value},(@{address})"
                        )));
                    }
                }
                Ok(steps)
            }
            _ => Err(error(
                ErrorKind::UnsupportedOperation,
                "Operation outside mainframe subset",
            )),
        }
    }
    fn decode(&self, check: Check, body: &str) -> Result<Option<CatalogReply>> {
        let parse = || error(ErrorKind::Parse, "Unexpected mainframe response");
        let device = || {
            error(
                ErrorKind::Device,
                "Mainframe state violates commissioned byte policy",
            )
        };
        let token = |expected: &str, known: &[&str]| {
            if body == expected {
                Ok(None)
            } else if known.contains(&body) {
                Err(device())
            } else {
                Err(parse())
            }
        };
        match check {
            Check::State => match body {
                "0" => Ok(Some(CatalogReply::Boolean(false))),
                "1" => Ok(Some(CatalogReply::Boolean(true))),
                _ => Err(parse()),
            },
            Check::Width => token("1", &["2", "4"]),
            Check::Direction(direction) => token(
                if direction == Direction::Input {
                    "INP"
                } else {
                    "OUTP"
                },
                &["INP", "OUTP"],
            ),
            Check::Polarity => token("NORM", &["INV"]),
            Check::Handshake => token("HIMP", &["OFF", "ON"]),
            Check::Disabled => token("0", &["1"]),
            Check::Drive => token("ACT", &["OCOL"]),
            Check::Threshold | Check::Level => {
                let value = crate::qualified_scpi::decimal(body)?;
                if value.abs() >= 1e30 {
                    return Err(parse());
                }
                if value
                    != if matches!(check, Check::Threshold) {
                        2.5
                    } else {
                        3.3
                    }
                {
                    return Err(device());
                }
                Ok(None)
            }
            Check::Byte => {
                if !body.bytes().all(|b| b.is_ascii_digit()) {
                    return Err(parse());
                }
                let value = body
                    .parse::<u32>()
                    .ok()
                    .filter(|v| *v <= 255)
                    .ok_or_else(parse)?;
                Ok(Some(CatalogReply::Count(value)))
            }
        }
    }
}
