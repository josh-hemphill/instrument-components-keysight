//! Native 3585B scalar subset; SP selects stored data, T6 refreshes only its selected point.
use crate::message_backend::{self as tx, error, Engine, Step};
use instrument_api::{catalog::*, *};
use instrument_scpi::Connector;
use std::{collections::BTreeSet, sync::Arc};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Commissioning {
    Unknown,
    QualifiedNativeScalarConfirmed,
}
#[derive(Clone, Debug)]
pub struct Binding {
    pub endpoint: Endpoint,
    pub commissioning: Commissioning,
}
#[derive(Clone)]
pub struct Factory {
    routes: BTreeSet<(String, String)>,
    connector: Arc<dyn Connector>,
    #[cfg(feature = "async")]
    native: Option<Arc<dyn instrument_scpi::AsyncConnector>>,
}
fn operations() -> BTreeSet<Operation> {
    use CatalogOperation::*;
    [
        SpectrumAnalyzerSetCenterFrequency,
        SpectrumAnalyzerSetSpan,
        SpectrumAnalyzerSetRbw,
        SpectrumAnalyzerSetVbw,
        SpectrumAnalyzerSetRefLevel,
        SpectrumAnalyzerMarkerPeak,
        SpectrumAnalyzerMarkerX,
        SpectrumAnalyzerMarkerY,
    ]
    .into_iter()
    .map(Operation::Catalog)
    .collect()
}
fn endpoint(endpoint: &Endpoint) -> Result<()> {
    if !endpoint.scheme.is_ascii() {
        return Err(error(
            ErrorKind::InvalidArgument,
            "ASCII hpib scheme required",
        ));
    }
    tx::validate_endpoint(endpoint)
}
impl Factory {
    pub fn new(bindings: Vec<Binding>, connector: Arc<dyn Connector>) -> Result<Self> {
        if bindings.is_empty() || bindings.len() > 4096 {
            return Err(error(
                ErrorKind::InvalidArgument,
                "Bind1..4096 qualified native endpoints",
            ));
        }
        let mut routes = BTreeSet::new();
        for binding in bindings {
            endpoint(&binding.endpoint)?;
            if binding.commissioning != Commissioning::QualifiedNativeScalarConfirmed
                || !routes.insert(tx::endpoint_key(&binding.endpoint))
            {
                return Err(error(
                    ErrorKind::InvalidArgument,
                    "Explicit qualification and unique endpoints required",
                ));
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
        if !self
            .routes
            .contains(&tx::endpoint_key(&context.candidate.endpoint))
        {
            return Err(error(
                ErrorKind::InvalidArgument,
                "Unbound qualified endpoint",
            ));
        }
        if !context.required.is_subset(&operations()) {
            return Err(error(
                ErrorKind::UnsupportedOperation,
                "Required operation outside native scalar subset",
            ));
        }
        Ok(Core)
    }
}
impl DriverFactory for Factory {
    fn descriptor(&self) -> DriverDescriptor {
        DriverDescriptor {
            id: "keysight.hp3585b".into(),
            contract_major: CONTRACT_MAJOR,
            rules: ["HP", "HEWLETT-PACKARD", "Agilent Technologies"]
                .into_iter()
                .map(|m| MatchRule::ExactIdentity {
                    scheme: "hpib".into(),
                    manufacturer: m.into(),
                    model: "3585B".into(),
                })
                .collect(),
            operations: operations(),
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
struct Core;
fn integer_hz(value: f64) -> Result<u32> {
    if value.is_finite() && (1.0..=40_100_000.0).contains(&value) && value.fract() == 0.0 {
        Ok(value as u32)
    } else {
        Err(error(
            ErrorKind::InvalidArgument,
            "Require integer1..40100000Hz",
        ))
    }
}
fn tenths(value: f64) -> Result<i32> {
    if value.is_finite() && (-70.0..=40.0).contains(&value) {
        let n = (value * 10.0).round() as i32;
        if value == f64::from(n) / 10.0 {
            return Ok(n);
        }
    }
    Err(error(
        ErrorKind::InvalidArgument,
        "Reference requires integer tenths in -70..40dBm",
    ))
}
fn field(text: &str, integers: usize, fractions: usize) -> Result<f64> {
    let parse = || error(ErrorKind::Parse, "Invalid native fixed field");
    let b = text.as_bytes();
    let point = 1 + integers;
    if b.len() != point + 1 + fractions
        || !matches!(b[0], b'+' | b'-')
        || b[point] != b'.'
        || !b[point + 1..].iter().all(u8::is_ascii_digit)
    {
        return Err(parse());
    }
    let digits = text[1..point].trim_start_matches(' ');
    if digits.is_empty() || !digits.bytes().all(|c| c.is_ascii_digit()) {
        return Err(parse());
    }
    format!("{}{}.{}", &text[..1], digits, &text[point + 1..])
        .parse::<f64>()
        .map_err(|_| parse())
}
impl Engine for Core {
    type Check = Unit;
    fn operations(&self) -> BTreeSet<Operation> {
        operations()
    }
    fn prepare(&self, request: &CatalogRequest) -> Result<Vec<Step<Unit>>> {
        use CatalogRequest::*;
        let command = match request {
            SpectrumAnalyzerSetCenterFrequency { hz } => format!("CF{}HZ", integer_hz(*hz)?),
            SpectrumAnalyzerSetSpan { hz } => format!("FS{}HZ", integer_hz(*hz)?),
            SpectrumAnalyzerSetRbw { hz } | SpectrumAnalyzerSetVbw { hz } => {
                let value = integer_hz(*hz)?;
                let vb = matches!(request, SpectrumAnalyzerSetVbw { .. });
                if ![3, 10, 30, 100, 300, 1000, 3000, 10000, 30000].contains(&value)
                    && !(vb && value == 1)
                {
                    return Err(error(
                        ErrorKind::InvalidArgument,
                        "Unsupported discrete bandwidth",
                    ));
                }
                format!("{}{}HZ", if vb { "VB" } else { "RB" }, value)
            }
            SpectrumAnalyzerSetRefLevel { dbm } => {
                let n = tenths(*dbm)?;
                format!(
                    "RL{}{}.{}DM",
                    if n < 0 { "-" } else { "" },
                    n.abs() / 10,
                    n.abs() % 10
                )
            }
            SpectrumAnalyzerMarkerPeak => {
                return Ok(vec![Step::Write("CL".into()), Step::Write("SP".into())])
            }
            SpectrumAnalyzerMarkerX | SpectrumAnalyzerMarkerY => {
                return Ok(vec![
                    Step::Write("CL".into()),
                    Step::Query(
                        "D2,T6".into(),
                        if matches!(request, SpectrumAnalyzerMarkerX) {
                            Unit::Hertz
                        } else {
                            Unit::Dbm
                        },
                    ),
                ])
            }
            _ => {
                return Err(error(
                    ErrorKind::UnsupportedOperation,
                    "Operation outside native scalar subset",
                ))
            }
        };
        Ok(vec![Step::Write(command)])
    }
    fn decode(&self, unit: Unit, body: &str) -> Result<Option<CatalogReply>> {
        let b = body.as_bytes();
        let parse = || {
            error(
                ErrorKind::Parse,
                "Expected24-character native D2 frequency/amplitude record",
            )
        };
        if b.len() != 24 || !b.is_ascii() || b[12] != b',' || &body[21..] != "E-0" {
            return Err(parse());
        }
        let frequency = field(&body[..12], 9, 1)?;
        let amplitude = field(&body[13..21], 4, 2)?;
        if !(0.0..=40_100_000.0).contains(&frequency) {
            return Err(parse());
        }
        Ok(Some(CatalogReply::Quantity(Quantity {
            value: if unit == Unit::Hertz {
                frequency
            } else {
                amplitude
            },
            unit,
            status: MeasurementStatus::Valid,
        })))
    }
}
