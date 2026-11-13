//! Native HP 5334A/B counters. See spec/legacy/HP5334A.md and HP5334B.md.
//! The application commissions exclusive REMOTE/native/internal-gating operation.
use instrument_api::{catalog::*, *};
use instrument_scpi::{Connector, MessageIo};
use std::{collections::BTreeSet, sync::Arc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Model {
    Hp5334A,
    Hp5334B,
}
impl Model {
    pub fn name(self) -> &'static str {
        match self {
            Self::Hp5334A => "5334A",
            Self::Hp5334B => "5334B",
        }
    }
    pub fn driver_id(self) -> &'static str {
        match self {
            Self::Hp5334A => "keysight.hp5334a",
            Self::Hp5334B => "keysight.hp5334b",
        }
    }
}
/// Commissioning evidence supplied by the application, never inferred by probing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Commissioning {
    Unknown,
    Ciil,
    NativeInternalGatingConfirmed,
}

fn error(kind: ErrorKind, message: &str) -> DriverError {
    DriverError::new(kind, message)
}
fn operations() -> BTreeSet<Operation> {
    [
        CatalogOperation::CounterSelectChannel,
        CatalogOperation::CounterMeasureFrequency,
        CatalogOperation::CounterMeasurePeriod,
        CatalogOperation::CounterSetGateTime,
        CatalogOperation::Reset,
    ]
    .into_iter()
    .map(Operation::Catalog)
    .chain([Operation::Reset])
    .collect()
}
fn descriptor(model: Model) -> DriverDescriptor {
    DriverDescriptor {
        id: model.driver_id().into(),
        contract_major: CONTRACT_MAJOR,
        rules: ["HP", "Hewlett-Packard"]
            .into_iter()
            .map(|manufacturer| MatchRule::ExactIdentity {
                scheme: "hpib".into(),
                manufacturer: manufacturer.into(),
                model: model.name().into(),
            })
            .collect(),
        operations: operations(),
    }
}
fn validate(model: Model, context: &OpenContext<'_>) -> Result<()> {
    if !context
        .candidate
        .endpoint
        .scheme
        .trim()
        .eq_ignore_ascii_case("hpib")
        || context.candidate.endpoint.address.trim().is_empty()
    {
        return Err(error(
            ErrorKind::InvalidArgument,
            "HP counter requires a nonempty hpib endpoint",
        ));
    }
    if !context.required.is_subset(&descriptor(model).operations) {
        return Err(error(
            ErrorKind::UnsupportedOperation,
            "required operation unavailable for HP counter",
        ));
    }
    Ok(())
}

/// Statically register this factory in the neutral runtime. Open sends no commands.
#[derive(Clone)]
pub struct Factory {
    model: Model,
    connector: Arc<dyn Connector>,
    #[cfg(feature = "async")]
    native: Option<Arc<dyn instrument_scpi::AsyncConnector>>,
}
impl Factory {
    pub fn new(
        model: Model,
        commissioning: Commissioning,
        connector: Arc<dyn Connector>,
    ) -> Result<Self> {
        if commissioning != Commissioning::NativeInternalGatingConfirmed {
            return Err(error(ErrorKind::InvalidArgument, "confirm native language, internal gating and exclusive REMOTE commissioning before registration"));
        }
        Ok(Self {
            model,
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
}
impl DriverFactory for Factory {
    fn descriptor(&self) -> DriverDescriptor {
        descriptor(self.model)
    }
    fn execution_modes(&self) -> BTreeSet<ExecutionMode> {
        let modes = [ExecutionMode::Sync];
        #[cfg(feature = "async")]
        {
            let mut modes: BTreeSet<_> = modes
                .into_iter()
                .chain([ExecutionMode::WorkerAsync])
                .collect();
            if self.native.is_some() {
                modes.insert(ExecutionMode::NativeAsync);
            }
            modes
        }
        #[cfg(not(feature = "async"))]
        {
            modes.into_iter().collect()
        }
    }
    fn open(&self, context: &OpenContext<'_>) -> Result<Box<dyn DeviceBackend>> {
        validate(self.model, context)?;
        Ok(Box::new(Device {
            core: Core::default(),
            io: self.connector.open(context)?,
            ownership: context.ownership,
            closed: false,
        }))
    }
    #[cfg(feature = "async")]
    fn open_native<'a>(
        &'a self,
        context: &'a OpenContext<'_>,
    ) -> DriverFuture<'a, Box<dyn AsyncDeviceBackend>> {
        Box::pin(async move {
            validate(self.model, context)?;
            let connector = self.native.as_ref().ok_or_else(|| {
                error(
                    ErrorKind::UnsupportedExecutionMode,
                    "native connector not configured",
                )
            })?;
            let io = connector.open(context).await?;
            Ok(Box::new(AsyncDevice {
                core: Core::default(),
                io,
                ownership: context.ownership,
                closed: false,
            }) as Box<dyn AsyncDeviceBackend>)
        })
    }
}

enum Action {
    Local,
    Write(String),
    Query(String, Unit),
}
enum Update {
    None,
    Channel(u32),
    Gate(f64),
    Reset,
}
struct Prepared {
    action: Action,
    update: Update,
}
struct Core {
    channel: u32,
    gate: Option<f64>,
    healthy: bool,
}
impl Default for Core {
    fn default() -> Self {
        Self {
            channel: 1,
            gate: None,
            healthy: true,
        }
    }
}
fn gate_text(seconds: f64) -> String {
    let raw = format!("{seconds:.11E}");
    let (mantissa, exponent) = raw.split_once('E').expect("scientific formatter");
    format!(
        "{mantissa}E{:+03}",
        exponent.parse::<i32>().expect("scientific exponent")
    )
}
impl Core {
    fn prepare(&self, request: &CatalogRequest) -> Result<Prepared> {
        request.validate()?;
        if !self.healthy {
            return Err(error(
                ErrorKind::SessionNeedsRecovery,
                "HP counter requires external resynchronization and reopen",
            ));
        }
        let (action, update) = match request {
            CatalogRequest::CounterSelectChannel { channel } => {
                if !matches!(channel, 1 | 2) {
                    return Err(error(
                        ErrorKind::InvalidArgument,
                        "only channels 1=A and 2=B are configured",
                    ));
                }
                (Action::Local, Update::Channel(*channel))
            }
            CatalogRequest::CounterSetGateTime { seconds } => {
                if !(0.001..=99.999).contains(seconds) {
                    return Err(error(
                        ErrorKind::InvalidArgument,
                        "gate time must be 0.001 through 99.999 seconds",
                    ));
                }
                (
                    Action::Write(format!("GA{}", gate_text(*seconds))),
                    Update::Gate(*seconds),
                )
            }
            CatalogRequest::CounterMeasureFrequency | CatalogRequest::CounterMeasurePeriod => {
                let period = matches!(request, CatalogRequest::CounterMeasurePeriod);
                if period && self.channel != 1 {
                    return Err(error(
                        ErrorKind::UnsupportedOperation,
                        "period is supported on channel A only",
                    ));
                }
                let function = if period { 4 } else { self.channel };
                let gate = self
                    .gate
                    .map(|v| format!("GA{},", gate_text(v)))
                    .unwrap_or_default();
                (
                    Action::Query(
                        format!("FN{function},HS0,MD1,WA1,{gate}RE"),
                        if period { Unit::Second } else { Unit::Hertz },
                    ),
                    Update::None,
                )
            }
            CatalogRequest::Reset => (Action::Write("IN".into()), Update::Reset),
            _ => {
                return Err(error(
                    ErrorKind::UnsupportedOperation,
                    "operation not implemented by HP counter",
                ))
            }
        };
        Ok(Prepared { action, update })
    }
    fn finish(&mut self, prepared: Prepared, response: Option<String>) -> Result<CatalogReply> {
        let reply = match prepared.action {
            Action::Query(_, unit) => {
                CatalogReply::Quantity(parse(response.as_deref().unwrap_or_default(), unit)?)
            }
            _ => CatalogReply::Unit,
        };
        match prepared.update {
            Update::None => {}
            Update::Channel(c) => self.channel = c,
            Update::Gate(v) => self.gate = Some(v),
            Update::Reset => {
                self.channel = 1;
                self.gate = None;
            }
        }
        self.healthy = true;
        Ok(reply)
    }
}
fn parse(body: &str, unit: Unit) -> Result<Quantity> {
    let bad = || {
        error(
            ErrorKind::Parse,
            "invalid HP5334 native ASCII measurement record",
        )
    };
    if body == "O+9.99999999999E+99" {
        return Ok(Quantity {
            value: 0.0,
            unit,
            status: MeasurementStatus::Overload,
        });
    }
    let expected = if unit == Unit::Hertz { b'F' } else { b'S' };
    let b = body.as_bytes();
    if b.len() > 19
        || b.first() != Some(&expected)
        || !b.iter().all(|c| c.is_ascii() && !c.is_ascii_control())
    {
        return Err(bad());
    }
    let field = &body[1..];
    let number = field.trim_start_matches(' ');
    if field.starts_with(' ') && number == "0E+00" {
        return Ok(Quantity {
            value: 0.0,
            unit,
            status: MeasurementStatus::Valid,
        });
    }
    if b.len() != 19 {
        return Err(bad());
    }
    let n = number.as_bytes();
    if n.len() < 6 || !matches!(n[0], b'+' | b'-') || !n[1].is_ascii_digit() {
        return Err(bad());
    }
    let exponent = n.len() - 4;
    if n[exponent] != b'E'
        || !matches!(n[exponent + 1], b'+' | b'-')
        || !n[exponent + 2..].iter().all(u8::is_ascii_digit)
    {
        return Err(bad());
    }
    if exponent != 2
        && (exponent < 4 || n[2] != b'.' || !n[3..exponent].iter().all(u8::is_ascii_digit))
    {
        return Err(bad());
    }
    let value: f64 = number.parse().map_err(|_| bad())?;
    if !value.is_finite() {
        return Err(bad());
    }
    Ok(Quantity {
        value,
        unit,
        status: MeasurementStatus::Valid,
    })
}

struct Device {
    core: Core,
    io: Box<dyn MessageIo>,
    ownership: ConnectionOwnership,
    closed: bool,
}
impl Device {
    fn release(&mut self) -> Result<()> {
        if self.closed {
            return Ok(());
        }
        self.closed = true;
        if self.ownership == ConnectionOwnership::Owned {
            self.io.close()
        } else {
            Ok(())
        }
    }
}
impl Drop for Device {
    fn drop(&mut self) {
        let _ = self.release();
    }
}
impl CatalogBackend for Device {
    fn invoke(&mut self, request: CatalogRequest) -> Result<CatalogReply> {
        if self.closed {
            return Err(error(
                ErrorKind::SessionClosed,
                "HP counter resource closed",
            ));
        }
        let prepared = self.core.prepare(&request)?;
        let response = match &prepared.action {
            Action::Local => None,
            Action::Write(command) => {
                self.core.healthy = false;
                self.io.write(command)?;
                None
            }
            Action::Query(command, _) => {
                self.core.healthy = false;
                Some(self.io.query(command)?)
            }
        };
        self.core.finish(prepared, response)
    }
}
impl Reset for Device {
    fn reset(&mut self) -> Result<()> {
        self.invoke(CatalogRequest::Reset)?.into_unit()
    }
}
impl DeviceBackend for Device {
    fn operations(&self) -> BTreeSet<Operation> {
        operations()
    }
    fn catalog(&mut self) -> Option<&mut dyn CatalogBackend> {
        Some(self)
    }
    fn reset(&mut self) -> Option<&mut dyn Reset> {
        Some(self)
    }
    fn close(&mut self) -> Result<()> {
        self.release()
    }
}

#[cfg(feature = "async")]
struct AsyncDevice {
    core: Core,
    io: Box<dyn instrument_scpi::AsyncMessageIo>,
    ownership: ConnectionOwnership,
    closed: bool,
}
#[cfg(feature = "async")]
impl AsyncDevice {
    fn release(&mut self) -> Result<()> {
        if self.closed {
            return Ok(());
        }
        self.closed = true;
        if self.ownership == ConnectionOwnership::Owned {
            self.io.close()
        } else {
            Ok(())
        }
    }
}
#[cfg(feature = "async")]
impl Drop for AsyncDevice {
    fn drop(&mut self) {
        let _ = self.release();
    }
}
#[cfg(feature = "async")]
impl AsyncCatalogBackend for AsyncDevice {
    fn invoke_async(&mut self, request: CatalogRequest) -> DriverFuture<'_, CatalogReply> {
        Box::pin(async move {
            if self.closed {
                return Err(error(
                    ErrorKind::SessionClosed,
                    "HP counter resource closed",
                ));
            }
            let prepared = self.core.prepare(&request)?;
            let response = match &prepared.action {
                Action::Local => None,
                Action::Write(command) => {
                    self.core.healthy = false;
                    self.io.write(command).await?;
                    None
                }
                Action::Query(command, _) => {
                    self.core.healthy = false;
                    Some(self.io.query(command).await?)
                }
            };
            self.core.finish(prepared, response)
        })
    }
}
#[cfg(feature = "async")]
impl AsyncReset for AsyncDevice {
    fn reset(&mut self) -> DriverFuture<'_, ()> {
        Box::pin(async { self.invoke_async(CatalogRequest::Reset).await?.into_unit() })
    }
}
#[cfg(feature = "async")]
impl AsyncDeviceBackend for AsyncDevice {
    fn operations(&self) -> BTreeSet<Operation> {
        operations()
    }
    fn catalog(&mut self) -> Option<&mut dyn AsyncCatalogBackend> {
        Some(self)
    }
    fn reset(&mut self) -> Option<&mut dyn AsyncReset> {
        Some(self)
    }
    fn close(&mut self) -> Result<()> {
        self.release()
    }
}
