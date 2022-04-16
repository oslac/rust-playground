use crate::{doom, error::cached_error::CachedError};

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Weak},
    time::Duration,
};

use color_eyre::eyre;
use eyre::eyre;
use parking_lot::Mutex;
use tokio::{sync::broadcast, time::Instant};

pub type BoxFut<'f, T> = Pin<Box<dyn Future<Output = T> + Send + 'f>>;

/// CachedData<T> represents some kind of data you want to enable caching for.
#[derive(Clone)]
pub struct CachedData<T>
where
    T: Clone + Send + Sync + 'static,
{
    inner: Arc<Mutex<CachedDataInner<T>>>,
    refresh_interval: Duration,
}

struct CachedDataInner<T>
where
    T: Clone + Send + Sync + 'static,
{
    last_update: Option<(Instant, T)>,
    inflight: Option<Weak<broadcast::Sender<Result<T, CachedError>>>>,
}

impl<T> Default for CachedDataInner<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self { last_update: None, inflight: None }
    }
}

impl<T> CachedData<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new(refresh_interval: Duration) -> Self {
        Self { inner: Default::default(), refresh_interval }
    }

    pub async fn get_cached<F, E>(&self, fetch: F) -> Result<T, CachedError>
    where
        F: FnOnce() -> BoxFut<'static, Result<T, E>>,
        E: std::fmt::Display + 'static,
    {
        let mut rx = {
            let mut inner = self.inner.lock();

            // CHECK IF DATA IS FRESH OR OLD
            if let Some((fetched_at, value)) = inner.last_update.as_ref() {
                if fetched_at.elapsed() < self.refresh_interval {
                    tracing::debug!("FRESH DATA DETECTED, RETURNING IT");
                    return Ok(value.clone());
                } else {
                    tracing::debug!("OLD DATA DETECTED, REFRESHING!");
                }
            }

            // REQUEST is `IN-FLIGHT` if it can be upgraded to `Arc`
            if let Some(inflight) = inner.inflight.as_ref().and_then(Weak::upgrade) {
                tracing::debug!("IN-FLIGHT REQUEST ACTIVE, SUBSCRIBING");
                inflight.subscribe()
            } else {
                tracing::debug!("NO IN-FLIGHT REQUESTS, FETCHING NEW DATA");
                // 1. There isn't an `IN-FLIGHT` request,
                // 2. Make a new request, creating the in-flight req.
                let (tx, rx) = broadcast::channel::<Result<T, CachedError>>(1);
                // 3. Reference count a single Sender ..
                let tx = Arc::new(tx);
                // 4. .. and only store a Weak reference:
                inner.inflight = Some(Arc::downgrade(&tx));

                let inner = self.inner.clone();
                let fetch = fetch();
                tokio::spawn(async move {
                    // Performs the request (ie. to fetch new data):
                    let res = fetch.await;
                    tracing::debug!("Request for new data performed");
                    // Trigger
                    if doom::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst) == 0 {
                        panic!("DOOM!")
                    };

                    {
                        // ONLY SYNC CODE IN THIS BLOCK
                        let mut inner = inner.lock();
                        inner.inflight = None;

                        match res {
                            Ok(new_data) => {
                                inner.last_update.replace((Instant::now(), new_data.clone()));
                                let _ = tx.send(Ok(new_data));
                            }
                            Err(e) => {
                                let _ = tx.send(Err(CachedError::new(e.to_string())));
                            }
                        };
                    }
                });
                rx
            }
        };

        // IF WE REACHED HERE, we are waiting for an in-flight request
        // Meaning, we were not able to serve from cache.
        tracing::debug!("WAITING FOR DEDUPLICATED FETCH TO FINISH ...");
        rx.recv().await.map_err(|_| eyre!("IN-FLIGHT-REQUEST DIED"))?
    }
}
