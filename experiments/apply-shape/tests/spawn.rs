//! The generic-`P` spawn: the runner's future, for a projection bound only as
//! `P: SendProjection`, awaited inside `tokio::spawn` on a multi-thread runtime.
//!
//! `crates/happenstance/tests/flavours.rs` spawns the shipped runner generic
//! over the **event store** with a concrete projection. This is the step the
//! brief asked for beyond it: the projection itself is a type parameter, so
//! the `Send`-ness of `apply`'s future has to come from the trait bound —
//! `trait_variant`'s `SendProjection` — and cannot leak from a concrete impl.

#![allow(clippy::unwrap_used)]

mod common;

use core::num::NonZeroUsize;
use std::sync::Arc;

use apply_shape::domain::{Ticked, scope, tick};
use apply_shape::{Delivered, SendProjection};
use happenstance::{
    MemoryEventStore, MemoryProjectionBatch, MemoryProjectionStore, MemoryProjectionStoreError,
    ProjectionId, Tags,
};

/// `README.md` §3's first weakened spawner: the bounds the work started from,
/// before the compiler asked for more. Expected not to compile, with nine
/// `E0277`s; `results/spawn-minimal-bounds.txt` is its transcript.
#[cfg(feature = "demonstrate-spawn-minimal-bounds")]
pub fn spawn_minimal_bounds<S, P>(
    events: Arc<S>,
    models: Arc<<P as SendProjection>::Store>,
    mut projection: P,
    chunk: NonZeroUsize,
) -> tokio::task::JoinHandle<(P, Result<apply_shape::Ran, String>)>
where
    S: happenstance::SendEventStore + Sync + 'static,
    P: SendProjection + Send + 'static,
    <P as SendProjection>::Store: happenstance::SendProjectionStore + Sync + 'static,
{
    tokio::spawn(async move {
        let ran = apply_shape::run(
            &*events,
            &*models,
            &mut projection,
            &happenstance::Json,
            chunk,
        )
        .await
        .map_err(|error| error.to_string());
        (projection, ran)
    })
}

/// `README.md` §3's second weakened spawner: every bound in §4.1, but with the
/// store written as `<P as SendProjection>::Store` rather than a type parameter.
/// Expected not to compile, with five `E0277`s (F1);
/// `results/spawn-store-as-projection.txt` is its transcript.
#[cfg(feature = "demonstrate-spawn-store-as-projection")]
pub fn spawn_store_as_projection<S, P>(
    events: Arc<S>,
    models: Arc<<P as SendProjection>::Store>,
    mut projection: P,
    chunk: NonZeroUsize,
) -> tokio::task::JoinHandle<(P, Result<apply_shape::Ran, String>)>
where
    S: happenstance::SendEventStore<Error: Send> + Sync + 'static,
    P: SendProjection<Event: Send, Error: Send + Sync> + Send + 'static,
    <P as SendProjection>::Store: happenstance::SendProjectionStore<Batch: Send> + Sync + 'static,
{
    tokio::spawn(async move {
        let ran = apply_shape::run(
            &*events,
            &*models,
            &mut projection,
            &happenstance::Json,
            chunk,
        )
        .await
        .map_err(|error| error.to_string());
        (projection, ran)
    })
}

struct Count {
    id: ProjectionId,
    scope: Tags,
    total: u64,
}

impl SendProjection for Count {
    type Event = Ticked;
    type Store = MemoryProjectionStore;
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        _event: Delivered<Ticked>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        self.total += 1;
        batch.write("total", self.total);
        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_runner_spawns_from_code_generic_over_the_projection() {
    let events = Arc::new(MemoryEventStore::with_events([
        tick("a").unwrap(),
        tick("b").unwrap(),
        tick("c").unwrap(),
    ]));
    let models = Arc::new(MemoryProjectionStore::new());
    let count = Count {
        id: ProjectionId::from_static("count"),
        scope: scope(),
        total: 0,
    };

    let (count, ran) = common::spawn_run(
        Arc::clone(&events),
        Arc::clone(&models),
        count,
        NonZeroUsize::new(2).unwrap(),
    )
    .await
    .unwrap();

    let ran = ran.unwrap();
    assert_eq!(ran.applied, 3);
    assert_eq!(ran.through, events.last_position());
    assert_eq!(count.total, 3);
    assert_eq!(models.get("total"), Some(3));
}
