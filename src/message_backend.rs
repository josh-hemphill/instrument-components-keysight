//! Vendor-local bounded, fail-fast complete-message transactions.
use instrument_api::{catalog::*, *};
use instrument_scpi::MessageIo;
use std::collections::BTreeSet;

pub(crate) fn error(kind: ErrorKind, message: &str) -> DriverError {
    DriverError::new(kind, message)
}
pub(crate) enum Step<C> {
    Write(String),
    Query(String, C),
}
pub(crate) trait Engine: Send {
    type Check: Send;
    fn operations(&self) -> BTreeSet<Operation>;
    fn prepare(&self, request: &CatalogRequest) -> Result<Vec<Step<Self::Check>>>;
    fn decode(&self, check: Self::Check, body: &str) -> Result<Option<CatalogReply>>;
    fn begin(&mut self, _request: &CatalogRequest) {}
    fn commit(&mut self, _request: &CatalogRequest) {}
}
fn body_checked(body: &str) -> Result<()> {
    if body.is_empty()
        || body.len() > 256
        || !body.bytes().all(|b| b.is_ascii() && !b.is_ascii_control())
    {
        return Err(error(
            ErrorKind::Parse,
            "Expected bounded complete ASCII scalar body without control characters",
        ));
    }
    Ok(())
}
pub(crate) struct Device<E: Engine> {
    engine: E,
    io: Box<dyn MessageIo>,
    ownership: ConnectionOwnership,
    closed: bool,
    healthy: bool,
}
impl<E: Engine> Device<E> {
    pub(crate) fn new(engine: E, io: Box<dyn MessageIo>, ownership: ConnectionOwnership) -> Self {
        Self {
            engine,
            io,
            ownership,
            closed: false,
            healthy: true,
        }
    }
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
impl<E: Engine> Drop for Device<E> {
    fn drop(&mut self) {
        let _ = self.release();
    }
}
impl<E: Engine> CatalogBackend for Device<E> {
    fn invoke(&mut self, request: CatalogRequest) -> Result<CatalogReply> {
        if self.closed {
            return Err(error(ErrorKind::SessionClosed, "Resource closed"));
        }
        if !self.healthy {
            return Err(error(
                ErrorKind::SessionNeedsRecovery,
                "External resynchronization and reopen required",
            ));
        }
        request.validate()?;
        let steps = self.engine.prepare(&request)?;
        self.engine.begin(&request);
        let mut reply = CatalogReply::Unit;
        for step in steps {
            self.healthy = false;
            match step {
                Step::Write(message) => self.io.write(&message)?,
                Step::Query(message, check) => {
                    let body = self.io.query(&message)?;
                    body_checked(&body)?;
                    if let Some(value) = self.engine.decode(check, &body)? {
                        reply = value;
                    }
                }
            }
        }
        self.engine.commit(&request);
        self.healthy = true;
        Ok(reply)
    }
}
impl<E: Engine> DeviceBackend for Device<E> {
    fn operations(&self) -> BTreeSet<Operation> {
        self.engine.operations()
    }
    fn catalog(&mut self) -> Option<&mut dyn CatalogBackend> {
        Some(self)
    }
    fn close(&mut self) -> Result<()> {
        self.release()
    }
}

#[cfg(feature = "async")]
pub(crate) struct AsyncDevice<E: Engine> {
    engine: E,
    io: Box<dyn instrument_scpi::AsyncMessageIo>,
    ownership: ConnectionOwnership,
    closed: bool,
    healthy: bool,
}
#[cfg(feature = "async")]
impl<E: Engine> AsyncDevice<E> {
    pub(crate) fn new(
        engine: E,
        io: Box<dyn instrument_scpi::AsyncMessageIo>,
        ownership: ConnectionOwnership,
    ) -> Self {
        Self {
            engine,
            io,
            ownership,
            closed: false,
            healthy: true,
        }
    }
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
impl<E: Engine> Drop for AsyncDevice<E> {
    fn drop(&mut self) {
        let _ = self.release();
    }
}
#[cfg(feature = "async")]
impl<E: Engine> AsyncCatalogBackend for AsyncDevice<E> {
    fn invoke_async(&mut self, request: CatalogRequest) -> DriverFuture<'_, CatalogReply> {
        Box::pin(async move {
            if self.closed {
                return Err(error(ErrorKind::SessionClosed, "Resource closed"));
            }
            if !self.healthy {
                return Err(error(
                    ErrorKind::SessionNeedsRecovery,
                    "External resynchronization and reopen required",
                ));
            }
            request.validate()?;
            let steps = self.engine.prepare(&request)?;
            self.engine.begin(&request);
            let mut reply = CatalogReply::Unit;
            for step in steps {
                // Set before awaiting: abandoning a future never restores confidence.
                self.healthy = false;
                match step {
                    Step::Write(message) => self.io.write(&message).await?,
                    Step::Query(message, check) => {
                        let body = self.io.query(&message).await?;
                        body_checked(&body)?;
                        if let Some(value) = self.engine.decode(check, &body)? {
                            reply = value;
                        }
                    }
                }
            }
            self.engine.commit(&request);
            self.healthy = true;
            Ok(reply)
        })
    }
}
#[cfg(feature = "async")]
impl<E: Engine> AsyncDeviceBackend for AsyncDevice<E> {
    fn operations(&self) -> BTreeSet<Operation> {
        self.engine.operations()
    }
    fn catalog(&mut self) -> Option<&mut dyn AsyncCatalogBackend> {
        Some(self)
    }
    fn close(&mut self) -> Result<()> {
        self.release()
    }
}

pub(crate) fn endpoint_key(endpoint: &Endpoint) -> (String, String) {
    (
        endpoint.scheme.trim().to_ascii_lowercase(),
        endpoint.address.clone(),
    )
}
pub(crate) fn validate_endpoint(endpoint: &Endpoint) -> Result<()> {
    if !endpoint.scheme.trim().eq_ignore_ascii_case("hpib") || endpoint.address.trim().is_empty() {
        return Err(error(
            ErrorKind::InvalidArgument,
            "Nonempty hpib endpoint required",
        ));
    }
    Ok(())
}
pub(crate) fn descriptor(
    id: &str,
    model: &str,
    operations: BTreeSet<Operation>,
) -> DriverDescriptor {
    DriverDescriptor {
        id: id.into(),
        contract_major: CONTRACT_MAJOR,
        rules: ["HP", "Hewlett-Packard"]
            .into_iter()
            .map(|manufacturer| MatchRule::ExactIdentity {
                scheme: "hpib".into(),
                manufacturer: manufacturer.into(),
                model: model.into(),
            })
            .collect(),
        operations,
    }
}
pub(crate) fn modes(native: bool) -> BTreeSet<ExecutionMode> {
    let result = BTreeSet::from([ExecutionMode::Sync]);
    #[cfg(feature = "async")]
    {
        let mut result = result;
        result.insert(ExecutionMode::WorkerAsync);
        if native {
            result.insert(ExecutionMode::NativeAsync);
        }
        result
    }
    #[cfg(not(feature = "async"))]
    {
        let _ = native;
        result
    }
}
