//! F1 against the **shipped** trait: `happenstance::Projection`, a plain trait
//! with no `#[trait_variant::make]`, spawned through `happenstance::run_projection`
//! from code generic over the projection.
//!
//! README §4.2's F1 says a generic spawner must name the store as its own type
//! parameter. This file is the compiled evidence that the same holds for the
//! shipped trait, which `trait_variant` does not generate: the cause is the
//! associated type's item bound, `type Store: ProjectionStore`, and not the macro.
//!
//! - By default it builds `spawn_shipped`, written with `Store = St`, and runs it.
//! - Under `--features demonstrate-shipped-f1` it also builds
//!   `spawn_shipped_store_as_projection`, the same spawner with the store written
//!   as `P::Store`. That build is **expected to fail** with `E0277`s, and
//!   `results/spawn-shipped-store-as-projection.txt` is its transcript.

#![allow(clippy::unwrap_used)]

use core::num::NonZeroUsize;
use std::sync::Arc;

use apply_shape::domain::{Ticked, scope, tick};
use happenstance::{
    Json, MemoryEventStore, MemoryProjectionBatch, MemoryProjectionStore,
    MemoryProjectionStoreError, Projection, ProjectionId, SendEventStore, SendProjectionStore,
    Tags, run_projection,
};

/// The shipped runner, spawned from code generic over `P`, with the store named
/// as the type parameter `St`.
fn spawn_shipped<S, P, St>(
    events: Arc<S>,
    models: Arc<St>,
    mut projection: P,
    chunk: NonZeroUsize,
) -> tokio::task::JoinHandle<(P, Result<usize, String>)>
where
    S: SendEventStore<Error: Send> + Sync + 'static,
    P: Projection<Store = St, Event: Send> + Send + 'static,
    St: SendProjectionStore<Batch: Send, Error: Send + Sync> + Sync + 'static,
{
    tokio::spawn(async move {
        let ran = run_projection(&*events, &*models, &mut projection, &Json, chunk)
            .await
            .map(|progressed| progressed.applied)
            .map_err(|error| error.to_string());
        (projection, ran)
    })
}

/// The same spawner with the store written as `P::Store`. Expected not to
/// compile: the item bound `type Store: ProjectionStore` is preferred over the
/// blanket `impl<T: SendProjectionStore> ProjectionStore for T`, so the store's
/// futures are the bare flavour's, whose `Send`-ness is unknown.
#[cfg(feature = "demonstrate-shipped-f1")]
fn spawn_shipped_store_as_projection<S, P>(
    events: Arc<S>,
    models: Arc<P::Store>,
    mut projection: P,
    chunk: NonZeroUsize,
) -> tokio::task::JoinHandle<(P, Result<usize, String>)>
where
    S: SendEventStore<Error: Send> + Sync + 'static,
    P: Projection<Event: Send> + Send + 'static,
    P::Store: SendProjectionStore<Batch: Send, Error: Send + Sync> + Sync + 'static,
{
    tokio::spawn(async move {
        let ran = run_projection(&*events, &*models, &mut projection, &Json, chunk)
            .await
            .map(|progressed| progressed.applied)
            .map_err(|error| error.to_string());
        (projection, ran)
    })
}

struct Count {
    id: ProjectionId,
    scope: Tags,
    total: u64,
}

impl Projection for Count {
    type Event = Ticked;
    type Store = MemoryProjectionStore;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(
        &mut self,
        _event: Ticked,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        self.total += 1;
        batch.write("total", self.total);
        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_shipped_runner_spawns_from_generic_code_when_the_store_is_a_type_parameter() {
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

    let (count, ran) = spawn_shipped(
        Arc::clone(&events),
        Arc::clone(&models),
        count,
        NonZeroUsize::new(2).unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(ran.unwrap(), 3);
    assert_eq!(count.total, 3);
    assert_eq!(models.get("total"), Some(3));
}
