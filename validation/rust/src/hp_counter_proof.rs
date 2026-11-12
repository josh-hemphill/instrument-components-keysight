//! Lifecycle and selection checks complement the independently authored wire corpus.
use instrument_api::{catalog::*, *};
use instrument_driver_keysight::hp5334::{Commissioning, Factory, Model};
use instrument_runtime::*;
use instrument_testkit::*;
use std::{collections::BTreeSet, sync::Arc};

fn candidate(model: Model) -> Candidate {
    Candidate {
        endpoint: Endpoint {
            scheme: "hpib".into(),
            address: "GPIB0::3::INSTR".into(),
        },
        identity: Identity {
            manufacturer: Some("HP".into()),
            model: Some(model.name().into()),
            ..Identity::default()
        },
    }
}
fn factory(model: Model, script: &Script) -> Result<Factory> {
    Factory::new(
        model,
        Commissioning::NativeInternalGatingConfirmed,
        Arc::new(script.clone()),
    )
}
fn new_registry(factory: Factory) -> Result<DriverRegistry> {
    let factories: Vec<Arc<dyn DriverFactory>> = vec![Arc::new(factory)];
    DriverRegistry::new(factories)
}
pub fn synchronous() -> Result<()> {
    for model in [Model::Hp5334A, Model::Hp5334B] {
        let empty = Script::new([]);
        for mode in [Commissioning::Unknown, Commissioning::Ciil] {
            assert!(
                matches!(Factory::new(model, mode, Arc::new(empty.clone())), Err(e) if e.kind == ErrorKind::InvalidArgument)
            );
        }
        let registry = new_registry(factory(model, &empty)?)?;
        for manufacturer in ["HP", "Hewlett-Packard", " hp ", "HEWLETT-PACKARD"] {
            let mut c = candidate(model);
            c.identity.manufacturer = Some(manufacturer.into());
            assert_eq!(
                registry.select(&c, &Selection::default())?.driver_id,
                model.driver_id()
            );
        }
        let mut wrong = candidate(model);
        wrong.endpoint.scheme = "scpi".into();
        assert_eq!(
            registry
                .select(&wrong, &Selection::default())
                .unwrap_err()
                .kind,
            ErrorKind::DriverNotFound
        );
        let required = [Operation::Catalog(CatalogOperation::QueryIdentity)]
            .into_iter()
            .collect();
        assert!(
            matches!(registry.open(&candidate(model), &Selection::default(), &required, ExecutionMode::Sync, ConnectionOwnership::Owned), Err(e) if e.kind == ErrorKind::UnsupportedOperation)
        );
        assert_eq!(empty.opens(), 0);
        assert!(!factory(model, &empty)?
            .execution_modes()
            .contains(&ExecutionMode::NativeAsync));

        let borrowed = registry.open(
            &candidate(model),
            &Selection::default(),
            &BTreeSet::new(),
            ExecutionMode::Sync,
            ConnectionOwnership::Borrowed,
        )?;
        borrowed.close(false)?;
        borrowed.close(false)?;
        drop(borrowed);
        assert_eq!(empty.opens(), 1);
        assert_eq!(empty.closes(), 0);
        empty.assert_complete()?;

        let script = Script::new([ScriptStep::Close(Ok(()))]);
        let registry = new_registry(factory(model, &script)?)?;
        let handle = registry.open(
            &candidate(model),
            &Selection::default(),
            &BTreeSet::new(),
            ExecutionMode::Sync,
            ConnectionOwnership::Owned,
        )?;
        for seconds in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(
                handle
                    .invoke(CatalogRequest::CounterSetGateTime { seconds })
                    .unwrap_err()
                    .kind,
                ErrorKind::InvalidArgument
            );
        }
        handle.invoke(CatalogRequest::CounterSelectChannel { channel: 1 })?;
        handle.close(false)?;
        handle.close(false)?;
        drop(handle);
        assert_eq!(script.closes(), 1);
        script.assert_complete()?;

        for failure in [ErrorKind::Timeout, ErrorKind::Transport, ErrorKind::Parse] {
            let reply = if failure == ErrorKind::Parse {
                Ok("F".into())
            } else {
                Err(DriverError::new(failure, "injected"))
            };
            let script = Script::new([
                ScriptStep::Query("FN1,HS0,MD1,WA1,RE".into(), reply),
                ScriptStep::Close(Ok(())),
            ]);
            let registry = new_registry(factory(model, &script)?)?;
            let handle = registry.open(
                &candidate(model),
                &Selection::default(),
                &BTreeSet::new(),
                ExecutionMode::Sync,
                ConnectionOwnership::Owned,
            )?;
            assert_eq!(
                handle
                    .invoke(CatalogRequest::CounterMeasureFrequency)
                    .unwrap_err()
                    .kind,
                failure
            );
            assert_eq!(
                handle
                    .invoke(CatalogRequest::CounterMeasureFrequency)
                    .unwrap_err()
                    .kind,
                ErrorKind::SessionNeedsRecovery
            );
            assert_eq!(
                handle.recover().unwrap_err().kind,
                ErrorKind::SessionNeedsRecovery
            );
            assert_eq!(
                script.events().len(),
                1,
                "no recovery, reset or retry traffic"
            );
            handle.close(false)?;
            script.assert_complete()?;
        }
        let script = Script::new([ScriptStep::Close(Err(DriverError::new(
            ErrorKind::Transport,
            "close failed",
        )))]);
        let registry = new_registry(factory(model, &script)?)?;
        let handle = registry.open(
            &candidate(model),
            &Selection::default(),
            &BTreeSet::new(),
            ExecutionMode::Sync,
            ConnectionOwnership::Owned,
        )?;
        assert_eq!(handle.close(false).unwrap_err().kind, ErrorKind::Shutdown);
        assert_eq!(handle.close(false).unwrap_err().kind, ErrorKind::Shutdown);
        drop(handle);
        assert_eq!(script.closes(), 1);
        script.assert_complete()?;
    }
    println!("PASS HP5334A/B passive selection, commissioning, nonfinite arguments, borrowed/owned lifecycle and quarantine");
    Ok(())
}

#[cfg(feature = "async")]
mod cancellation {
    use super::*;
    use instrument_scpi::{AsyncConnector, AsyncMessageIo};
    use std::{
        future::Future,
        pin::Pin,
        sync::atomic::{AtomicUsize, Ordering},
        task::{Context, Poll, Waker},
    };
    #[derive(Clone, Default)]
    struct PendingIo {
        queries: Arc<AtomicUsize>,
        closes: Arc<AtomicUsize>,
    }
    impl AsyncMessageIo for PendingIo {
        fn write<'a>(&'a mut self, _: &'a str) -> DriverFuture<'a, ()> {
            Box::pin(async { panic!("unexpected write") })
        }
        fn query<'a>(&'a mut self, command: &'a str) -> DriverFuture<'a, String> {
            Box::pin(async move {
                assert_eq!(command, "FN1,HS0,MD1,WA1,RE");
                self.queries.fetch_add(1, Ordering::SeqCst);
                std::future::pending().await
            })
        }
        fn close(&mut self) -> Result<()> {
            self.closes.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }
    impl AsyncConnector for PendingIo {
        fn open<'a>(&'a self, _: &'a OpenContext<'_>) -> DriverFuture<'a, Box<dyn AsyncMessageIo>> {
            Box::pin(async { Ok(Box::new(self.clone()) as Box<dyn AsyncMessageIo>) })
        }
    }
    fn poll_once<F: Future>(future: Pin<&mut F>) -> Poll<F::Output> {
        future.poll(&mut Context::from_waker(Waker::noop()))
    }
    pub async fn run() -> Result<()> {
        for model in [Model::Hp5334A, Model::Hp5334B] {
            let sync = Script::new([]);
            let pending = PendingIo::default();
            let registry = new_registry(
                factory(model, &sync)?.with_native_connector(Arc::new(pending.clone())),
            )?;
            let handle = registry
                .open_native(
                    &candidate(model),
                    &Selection::default(),
                    &BTreeSet::new(),
                    ConnectionOwnership::Owned,
                )
                .await?;
            drop(handle.invoke_async(CatalogRequest::CounterMeasureFrequency));
            assert_eq!(pending.queries.load(Ordering::SeqCst), 0);
            handle
                .invoke_async(CatalogRequest::CounterSelectChannel { channel: 1 })
                .await?;
            let mut query = Box::pin(handle.invoke_async(CatalogRequest::CounterMeasureFrequency));
            assert!(poll_once(query.as_mut()).is_pending());
            assert_eq!(pending.queries.load(Ordering::SeqCst), 1);
            drop(query);
            assert_eq!(
                handle
                    .invoke_async(CatalogRequest::CounterMeasureFrequency)
                    .await
                    .unwrap_err()
                    .kind,
                ErrorKind::SessionNeedsRecovery
            );
            assert_eq!(
                handle.recover().await.unwrap_err().kind,
                ErrorKind::SessionNeedsRecovery
            );
            handle.close(false).await?;
            assert_eq!(pending.queries.load(Ordering::SeqCst), 1);
            assert_eq!(pending.closes.load(Ordering::SeqCst), 1);
            assert_eq!(sync.opens(), 0);
        }
        println!("PASS HP5334A/B native cancellation before and during IO; quarantine and no detached host accesses");
        Ok(())
    }
}
#[cfg(feature = "async")]
pub use cancellation::run as asynchronous;
