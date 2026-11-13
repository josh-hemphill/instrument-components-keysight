//! Native future abandonment and actual delayed-host deadline proofs, no hardware.
use instrument_api::{catalog::*, *};
use instrument_driver_keysight::{hp3497 as daq, hp5334 as counter, hp_radio as rf};
use instrument_runtime::*;
use instrument_scpi::*;
use std::{
    collections::BTreeSet,
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    task::{Context, Waker},
    time::{Duration, Instant},
};

#[derive(Clone, Default)]
struct Host {
    deadline: bool,
    stall_write: bool,
    native_only: bool,
    calls: Arc<AtomicUsize>,
    active: Arc<AtomicUsize>,
    closes: Arc<AtomicUsize>,
}
struct Active(Arc<AtomicUsize>);
impl Drop for Active {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}
impl Connector for Host {
    fn open(&self, _: &OpenContext<'_>) -> Result<Box<dyn MessageIo>> {
        assert!(
            !self.native_only,
            "Native execution must not use sync acquisition"
        );
        Ok(Box::new(self.clone()))
    }
}
impl MessageIo for Host {
    fn write(&mut self, _: &str) -> Result<()> {
        assert!(
            !self.native_only,
            "Native execution must not use sync write"
        );
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.stall_write {
            let (_sender, receiver) = std::sync::mpsc::channel::<()>();
            return receiver
                .recv_timeout(Duration::from_millis(20))
                .map_err(|_| {
                    DriverError::new(ErrorKind::Timeout, "Independent write deadline expired")
                });
        }
        Ok(())
    }
    fn query(&mut self, _: &str) -> Result<String> {
        assert!(
            !self.native_only,
            "Native execution must not use sync query"
        );
        self.calls.fetch_add(1, Ordering::SeqCst);
        let (_sender, receiver) = std::sync::mpsc::channel::<String>();
        receiver
            .recv_timeout(Duration::from_millis(20))
            .map_err(|_| {
                DriverError::new(
                    ErrorKind::Timeout,
                    "Independent deadline expired while response remained pending",
                )
            })
    }
    fn close(&mut self) -> Result<()> {
        self.closes.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}
impl AsyncConnector for Host {
    fn open<'a>(&'a self, _: &'a OpenContext<'_>) -> DriverFuture<'a, Box<dyn AsyncMessageIo>> {
        Box::pin(async { Ok(Box::new(self.clone()) as Box<dyn AsyncMessageIo>) })
    }
}
impl AsyncMessageIo for Host {
    fn write<'a>(&'a mut self, _: &'a str) -> DriverFuture<'a, ()> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            if self.stall_write {
                self.active.fetch_add(1, Ordering::SeqCst);
                let _guard = Active(self.active.clone());
                return if self.deadline {
                    tokio::time::timeout(Duration::from_millis(20), std::future::pending::<()>())
                        .await
                        .map_err(|_| {
                            DriverError::new(
                                ErrorKind::Timeout,
                                "Independent write deadline expired",
                            )
                        })
                } else {
                    std::future::pending().await
                };
            }
            Ok(())
        })
    }
    fn query<'a>(&'a mut self, _: &'a str) -> DriverFuture<'a, String> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.active.fetch_add(1, Ordering::SeqCst);
            let _guard = Active(self.active.clone());
            if self.deadline {
                tokio::time::timeout(Duration::from_millis(20), std::future::pending::<String>())
                    .await
                    .map_err(|_| {
                        DriverError::new(
                            ErrorKind::Timeout,
                            "Independent deadline expired while response remained pending",
                        )
                    })
            } else {
                std::future::pending().await
            }
        })
    }
    fn close(&mut self) -> Result<()> {
        self.closes.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}
fn factory(index: usize, host: &Host) -> Result<Arc<dyn DriverFactory>> {
    let endpoint = Endpoint {
        scheme: "hpib".into(),
        address: "fixture".into(),
    };
    let connector = Arc::new(host.clone());
    Ok(match index {
        0 | 1 => Arc::new(
            counter::Factory::new(
                if index == 0 {
                    counter::Model::Hp5334A
                } else {
                    counter::Model::Hp5334B
                },
                counter::Commissioning::NativeInternalGatingConfirmed,
                connector.clone(),
            )?
            .with_native_connector(connector),
        ),
        2 => Arc::new(
            daq::Factory::new(
                vec![daq::Binding {
                    endpoint,
                    configuration: daq::Configuration {
                        option001: true,
                        normal_dvm_routing_confirmed: true,
                        inputs: vec![daq::Input {
                            channel: 7,
                            slot: 0,
                            local: 0,
                            module: daq::Module::Option010,
                            compensation: daq::Compensation::None,
                        }],
                    },
                }],
                connector.clone(),
            )?
            .with_native_connector(connector),
        ),
        _ => {
            let model = match index {
                3 => rf::Model::Hp8920A,
                4 => rf::Model::Hp8920B,
                _ => rf::Model::Hp8921A,
            };
            Arc::new(
                rf::Factory::new(
                    model,
                    vec![rf::Binding {
                        endpoint,
                        configuration: rf::Configuration {
                            model,
                            firmware: match model {
                                rf::Model::Hp8920A => "A.18.00",
                                rf::Model::Hp8920B => "B.06.00",
                                rf::Model::Hp8921A => "A.15.00",
                            }
                            .into(),
                            attenuator: rf::Attenuator::Electronic,
                            input_power: rf::InputPower::Standard,
                            source_port: rf::SourcePort::RfInOut,
                            receiver_input: rf::ReceiverInput::RfInOut,
                            commissioning: rf::Commissioning::Confirmed,
                        },
                    }],
                    connector.clone(),
                )?
                .with_native_connector(connector),
            )
        }
    })
}
fn request(index: usize) -> CatalogRequest {
    match index {
        0 | 1 => CatalogRequest::CounterMeasureFrequency,
        2 => CatalogRequest::DataAcquisitionMeasureVoltageDc {
            channel: 7,
            range: None,
        },
        _ => CatalogRequest::RfReceiverMeasureFrequency,
    }
}
pub async fn run() -> Result<()> {
    for index in 0..6 {
        let candidate = Candidate {
            endpoint: Endpoint {
                scheme: "hpib".into(),
                address: "fixture".into(),
            },
            identity: Identity::default(),
        };
        for stall_write in [false, true] {
            if stall_write && index < 3 {
                continue;
            }
            for ownership in [ConnectionOwnership::Owned, ConnectionOwnership::Borrowed] {
                let host = Host {
                    stall_write,
                    native_only: true,
                    ..Default::default()
                };
                let f = factory(index, &host)?;
                let selection = Selection {
                    driver_id: Some(f.descriptor().id),
                    ..Default::default()
                };
                let registry = DriverRegistry::new(vec![f])?;
                let handle = registry
                    .open_native(&candidate, &selection, &BTreeSet::new(), ownership)
                    .await?;
                drop(handle.invoke_async(request(index)));
                assert_eq!(host.calls.load(Ordering::SeqCst), 0);
                let mut future = Box::pin(handle.invoke_async(request(index)));
                assert!(Future::poll(
                    Pin::as_mut(&mut future),
                    &mut Context::from_waker(Waker::noop())
                )
                .is_pending());
                assert_eq!(host.active.load(Ordering::SeqCst), 1);
                drop(future);
                assert_eq!(host.active.load(Ordering::SeqCst), 0);
                let calls = host.calls.load(Ordering::SeqCst);
                assert_eq!(
                    handle.invoke_async(request(index)).await.unwrap_err().kind,
                    ErrorKind::SessionNeedsRecovery
                );
                handle.close(false).await?;
                assert_eq!(host.calls.load(Ordering::SeqCst), calls);
                assert_eq!(
                    host.closes.load(Ordering::SeqCst),
                    usize::from(ownership == ConnectionOwnership::Owned)
                );
            }
            for native in [false, true] {
                let host = Host {
                    deadline: true,
                    stall_write,
                    native_only: native,
                    ..Default::default()
                };
                let f = factory(index, &host)?;
                let selection = Selection {
                    driver_id: Some(f.descriptor().id),
                    ..Default::default()
                };
                let registry = DriverRegistry::new(vec![f])?;
                let started = Instant::now();
                if native {
                    let handle = registry
                        .open_native(
                            &candidate,
                            &selection,
                            &BTreeSet::new(),
                            ConnectionOwnership::Owned,
                        )
                        .await?;
                    assert_eq!(
                        handle.invoke_async(request(index)).await.unwrap_err().kind,
                        ErrorKind::Timeout
                    );
                    assert_eq!(
                        handle.invoke_async(request(index)).await.unwrap_err().kind,
                        ErrorKind::SessionNeedsRecovery
                    );
                    handle.close(false).await?;
                } else {
                    let handle = registry.open(
                        &candidate,
                        &selection,
                        &BTreeSet::new(),
                        ExecutionMode::Sync,
                        ConnectionOwnership::Owned,
                    )?;
                    assert_eq!(
                        handle.invoke(request(index)).unwrap_err().kind,
                        ErrorKind::Timeout
                    );
                    assert_eq!(
                        handle.invoke(request(index)).unwrap_err().kind,
                        ErrorKind::SessionNeedsRecovery
                    );
                    handle.close(false)?;
                }
                assert!(
                    started.elapsed() >= Duration::from_millis(15)
                        && started.elapsed() < Duration::from_secs(5)
                );
                assert_eq!(
                    host.calls.load(Ordering::SeqCst),
                    if stall_write || index < 3 { 1 } else { 3 }
                );
                assert_eq!(host.closes.load(Ordering::SeqCst), 1);
                assert_eq!(host.active.load(Ordering::SeqCst), 0);
            }
        }
    }
    println!("PASS all six HP models native abandonment, owned/borrowed lifetime and independent pending-read deadlines; all RF models pending-write cancellation/deadlines (sync/native); no detached access or retries");
    Ok(())
}
