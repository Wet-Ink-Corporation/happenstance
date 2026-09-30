//! The generic spawner every native case shares.
//!
//! A directory module, so cargo does not build it as a test target of its own.

use core::num::NonZeroUsize;
use std::sync::Arc;

use apply_shape::{Ran, SendProjection, run};
use happenstance::Json;

/// Spawns the runner onto a multi-thread runtime **from code generic over `P`**.
///
/// Every bound below is one the compiler asked for; `README.md` §4.1 records the
/// minimal set this started from (`results/spawn-minimal-bounds.txt`) and why
/// each one is there.
pub fn spawn_run<S, P, St>(
    events: Arc<S>,
    models: Arc<St>,
    mut projection: P,
    chunk: NonZeroUsize,
) -> tokio::task::JoinHandle<(P, Result<Ran, String>)>
where
    S: happenstance::SendEventStore<Error: Send> + Sync + 'static,
    P: SendProjection<Store = St, Event: Send, Error: Send + Sync> + Send + 'static,
    St: happenstance::SendProjectionStore<Batch: Send> + Sync + 'static,
{
    tokio::spawn(async move {
        let ran = run(&*events, &*models, &mut projection, &Json, chunk)
            .await
            .map_err(|error| error.to_string());
        (projection, ran)
    })
}
