//! HP3497A Option001 scalar DVM; explicit module inventory, no discovery.
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
    Option010,
    Option020,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compensation {
    Unknown,
    None,
    Software,
    Hardware,
}
#[derive(Clone, Debug)]
pub struct Input {
    pub channel: u32,
    pub slot: u8,
    pub local: u8,
    pub module: Module,
    pub compensation: Compensation,
}
#[derive(Clone, Debug)]
pub struct Configuration {
    pub option001: bool,
    pub normal_dvm_routing_confirmed: bool,
    pub inputs: Vec<Input>,
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
impl Factory {
    pub fn new(bindings: Vec<Binding>, connector: Arc<dyn Connector>) -> Result<Self> {
        if bindings.is_empty() || bindings.len() > 4096 {
            return Err(error(
                ErrorKind::InvalidArgument,
                "Bind 1..4096 endpoints explicitly",
            ));
        }
        let mut routes = BTreeMap::new();
        for binding in bindings {
            tx::validate_endpoint(&binding.endpoint)?;
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
        tx::validate_endpoint(&context.candidate.endpoint)?;
        let core = self
            .routes
            .get(&tx::endpoint_key(&context.candidate.endpoint))
            .ok_or_else(|| {
                error(
                    ErrorKind::InvalidArgument,
                    "Endpoint has no commissioned inventory",
                )
            })?;
        if !context.required.is_subset(&operations()) {
            return Err(error(
                ErrorKind::UnsupportedOperation,
                "Required operation unavailable",
            ));
        }
        Ok(core.clone())
    }
}
fn operations() -> BTreeSet<Operation> {
    BTreeSet::from([Operation::Catalog(
        CatalogOperation::DataAcquisitionMeasureVoltageDc,
    )])
}
impl DriverFactory for Factory {
    fn descriptor(&self) -> DriverDescriptor {
        tx::descriptor("keysight.hp3497a", "3497A", operations())
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
#[derive(Clone)]
struct Core {
    inputs: BTreeMap<u32, u8>,
}
impl Core {
    fn new(config: Configuration) -> Result<Self> {
        let bad = || {
            error(
                ErrorKind::InvalidArgument,
                "Require Option001, normal DVM routing, and explicit qualified module channels",
            )
        };
        if !config.option001
            || !config.normal_dvm_routing_confirmed
            || config.inputs.is_empty()
            || config.inputs.len() > 100
        {
            return Err(bad());
        }
        let mut inputs = BTreeMap::new();
        let mut native = BTreeSet::new();
        let mut slots = BTreeMap::new();
        for input in config.inputs {
            if input.channel == 0
                || input.slot > 4
                || !matches!(
                    (input.module, input.compensation, input.local),
                    (Module::Option010, Compensation::None, 0..=19)
                        | (Module::Option020, Compensation::Software, 0..=18)
                )
            {
                return Err(bad());
            }
            let number = 20 * input.slot + input.local;
            if inputs.insert(input.channel, number).is_some() || !native.insert(number) {
                return Err(bad());
            }
            if let Some(previous) = slots.insert(input.slot, (input.module, input.compensation)) {
                if previous != (input.module, input.compensation) {
                    return Err(bad());
                }
            }
        }
        Ok(Self { inputs })
    }
}
impl Engine for Core {
    type Check = ();
    fn operations(&self) -> BTreeSet<Operation> {
        operations()
    }
    fn prepare(&self, request: &CatalogRequest) -> Result<Vec<Step<()>>> {
        let CatalogRequest::DataAcquisitionMeasureVoltageDc { channel, range } = request else {
            return Err(error(
                ErrorKind::UnsupportedOperation,
                "Only scalar DC acquisition supported",
            ));
        };
        let native = self
            .inputs
            .get(channel)
            .ok_or_else(|| error(ErrorKind::InvalidArgument, "Unbound logical channel"))?;
        let range = match range {
            Some(v) if *v == 0.1 => 1,
            Some(v) if *v == 1.0 => 2,
            Some(v) if *v == 10.0 => 3,
            Some(v) if *v == 100.0 => 4,
            None => 5,
            _ => {
                return Err(error(
                    ErrorKind::InvalidArgument,
                    "Range must be 0.1, 1, 10, 100 volts or auto",
                ))
            }
        };
        Ok(vec![Step::Query(
            format!("VT4AE0VC0VS0VF1VN1VW0SO1VR{range}AI{native}"),
            (),
        )])
    }
    fn decode(&self, _: (), body: &str) -> Result<Option<CatalogReply>> {
        let bad = || {
            error(
                ErrorKind::Parse,
                "Expected eleven-byte HP3497A voltage record",
            )
        };
        let b = body.as_bytes();
        if b.len() != 11
            || !matches!(b[0], b'+' | b'-')
            || !b[1].is_ascii_digit()
            || b[2] != b'.'
            || !b[3..8].iter().all(u8::is_ascii_digit)
            || b[8] != b'E'
            || !matches!(b[9], b'+' | b'-')
            || !b[10].is_ascii_digit()
        {
            return Err(bad());
        }
        let (value, status) = if body == "+9.00000E+9" {
            (0.0, MeasurementStatus::Overload)
        } else {
            let value: f64 = body.parse().map_err(|_| bad())?;
            if !value.is_finite() || value.abs() > 120.0 {
                return Err(bad());
            }
            (value, MeasurementStatus::Valid)
        };
        Ok(Some(CatalogReply::Quantity(Quantity {
            value,
            unit: Unit::Volt,
            status,
        })))
    }
}
