//! Both flavours, and the `!Send` trap the loop must not walk into.
//!
//! `S::Error` carries no `Send` bound (ADR-0009), so a `Result<_, S::Error>`
//! held across the *next* read's await makes the whole future `!Send` — and
//! every single-threaded test still passes. `spawns_the_command_loop` is the
//! one that does not: it holds the loop across a real `tokio::spawn`, which is
//! exactly `happenstance_core`'s own `spawns_from_generic` applied one layer up.
//!
//! One flavour name is imported here — [`EventStore`], the weaker requirement —
//! and the other is reached by full path (RS-20-3).

use core::convert::Infallible;
use std::rc::Rc;
use std::sync::Arc;

use futures_core::Stream;
use happenstance::bytes::Bytes;
use happenstance::{
    AppendCondition, AppendError, Codec, CodecError, DecisionModel, DomainEvent, Event, EventId,
    EventStore, EventType, Json, MemoryEventStore, MemoryStoreError, Query, ReadOptions, Retry,
    SequencePosition, SequencedEvent, Tags, commit_with,
};
use serde::{Deserialize, Serialize};

const SUBSCRIBED: EventType = EventType::from_static("StudentSubscribed");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Subscribed {
    student: String,
}

impl DomainEvent for Subscribed {
    const EVENT_TYPES: &'static [EventType] = &[SUBSCRIBED];

    fn event_type(&self) -> EventType {
        SUBSCRIBED
    }

    fn tags(&self) -> Tags {
        course_tags()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(
        codec: &C,
        event_type: &EventType,
        data: &Bytes,
    ) -> Result<Self, CodecError> {
        if event_type != &SUBSCRIBED {
            return Err(CodecError::UnknownEventType {
                event_type: event_type.clone(),
            });
        }
        codec.decode(data)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Seats {
    scope: Tags,
    taken: u32,
}

impl DecisionModel for Seats {
    type Event = Subscribed;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, _event: Self::Event) {
        self.taken += 1;
    }
}

fn course_tags() -> Tags {
    Tags::from_pairs([("course", "c1")]).expect("a valid tag pair")
}

fn seats() -> Seats {
    Seats {
        scope: course_tags(),
        taken: 0,
    }
}

// ---------------------------------------------------------------------------
// AC-008 — the `Send` flavour, inside a real spawn
// ---------------------------------------------------------------------------

/// Awaits the loop inside `tokio::spawn`, from **generic** code.
///
/// Every bound here is load-bearing and none of them is `S::Error: Send`. That
/// absence is the assertion: if the loop held an `AppendError<S::Error>` across
/// the next read's await, this function would not compile.
fn spawns_the_command_loop<S>(store: Arc<S>) -> tokio::task::JoinHandle<bool>
where
    S: happenstance::SendEventStore + Send + Sync + 'static,
{
    tokio::spawn(async move {
        commit_with(&*store, seats(), &Json, Retry::once(), |_: &Seats| {
            Ok::<_, Infallible>(vec![Subscribed {
                student: "s1".to_owned(),
            }])
        })
        .await
        .is_ok()
    })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn commit_spawns_from_generic() {
    let store = Arc::new(MemoryEventStore::new());

    let committed = spawns_the_command_loop(Arc::clone(&store))
        .await
        .expect("the spawned task did not panic");

    assert!(committed);
    assert_eq!(store.len(), 1);
}

// ---------------------------------------------------------------------------
// AC-008 — and the bare flavour, which is the weaker requirement
// ---------------------------------------------------------------------------

/// A store that is genuinely `!Send`: an `Rc` in a field, not an omission.
///
/// A direct `impl EventStore for` a local type in a downstream crate, alongside
/// the blanket `impl<T: SendEventStore> EventStore for T` that `trait_variant`
/// emits. No `error[E0119]`, and no second flavour on one type (RS-20-4).
#[derive(Debug)]
struct LocalStore {
    /// `Rc`, not `Rc<RefCell<_>>`: the reference store already owns its own
    /// interior mutability, and a `RefCell` here would only add a borrow that
    /// clippy would rightly refuse to see held across an await.
    inner: Rc<MemoryEventStore>,
}

impl LocalStore {
    fn new() -> Self {
        Self {
            inner: Rc::new(MemoryEventStore::new()),
        }
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl EventStore for LocalStore {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        // Note the absence of `+ Send`. That is the whole point of the bare
        // flavour, and the reason this type exists.
        let held = self.inner.snapshot();
        let selected: Vec<Result<SequencedEvent, MemoryStoreError>> = held
            .into_iter()
            .filter(|event| query.matches(event.event_type(), event.tags()))
            .filter(|event| options.from.is_none_or(|from| event.position >= from))
            .map(Ok)
            .collect();
        Replay(selected.into_iter())
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        happenstance::SendEventStore::append(&*self.inner, events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        happenstance::SendEventStore::head(&*self.inner).await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        happenstance::SendEventStore::contains_event_id(&*self.inner, id).await
    }
}

/// A stream over a snapshot. `!Send` by construction — it is built inside a
/// `!Send` store and never leaves this file.
struct Replay(std::vec::IntoIter<Result<SequencedEvent, MemoryStoreError>>);

impl Stream for Replay {
    type Item = Result<SequencedEvent, MemoryStoreError>;

    fn poll_next(
        mut self: core::pin::Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Option<Self::Item>> {
        core::task::Poll::Ready(self.0.next())
    }
}

#[tokio::test]
async fn commit_binds_the_weak_flavour() {
    // The same call, against a store that is not `Send` at all. It compiles
    // because the loop binds `EventStore`, the requirement both flavours meet.
    let store = LocalStore::new();

    let done = commit_with(&store, seats(), &Json, Retry::once(), |_: &Seats| {
        Ok::<_, Infallible>(vec![Subscribed {
            student: "s1".to_owned(),
        }])
    })
    .await
    .expect("an uncontended commit against a !Send store");

    assert_eq!(done.attempts, 1);
    assert_eq!(store.len(), 1);
}

/// An inherent method shadows a trait method when the bound holds, and falls
/// through to it when it does not — a compile-time decision read as a bool.
mod send_probe {
    use core::marker::PhantomData;

    #[derive(Debug)]
    pub(crate) struct Probe<T>(pub(crate) PhantomData<T>);

    pub(crate) trait NotSend {
        fn is_send(&self) -> bool {
            false
        }
    }

    impl<T> NotSend for Probe<T> {}

    impl<T: Send> Probe<T> {
        // `&self` is the entire mechanism: an associated function would be
        // resolved by path and would never fall through to the trait candidate.
        #[allow(clippy::unused_self)]
        pub(crate) fn is_send(&self) -> bool {
            true
        }
    }
}

#[test]
fn the_weak_flavour_store_is_genuinely_not_send() {
    use core::marker::PhantomData;
    use send_probe::{NotSend as _, Probe};

    // Positive control, so this assertion cannot pass on a broken probe.
    assert!(
        Probe::<MemoryEventStore>(PhantomData).is_send(),
        "probe is broken: MemoryEventStore is Send"
    );
    assert!(
        !Probe::<LocalStore>(PhantomData).is_send(),
        "the bare-flavour store is Send, so it proves nothing about the weak bound"
    );
}
