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
#[cfg(feature = "unstable-projection")]
use happenstance::run_projection;
use happenstance::{
    AppendCondition, AppendError, Codec, CodecError, DecisionModel, DomainEvent, Event, EventId,
    EventStore, EventType, Json, MemoryEventStore, MemoryStoreError, Query, ReadOptions, Retry,
    SequencePosition, SequencedEvent, Tags, commit, commit_with,
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

// ---------------------------------------------------------------------------
// HS-S0031 AC-004 — the instrument cannot pass vacuously, and it is `Rc` that
// does the work
// ---------------------------------------------------------------------------

/// Two positive controls and two negative ones, so neither half is vacuous.
///
/// The extra pair is the *reason* the store is `Rc`-backed rather than
/// `RefCell`-backed, asserted rather than left in a comment: `RefCell<T>: Send
/// where T: Send` — it gives up `Sync`, not `Send` — so a store built on a bare
/// `RefCell` would still be `Send` and would prove nothing at all about the weak
/// bound (RS-25-1, RS-25-5). `Rc` is what actually removes it, and it is also
/// the honest shape: a Durable Object holds its store through one.
///
/// Break the probe — delete `send_probe`'s `impl<T: Send> Probe<T>` inherent
/// block — and the two positive controls fail while the two negatives keep
/// passing. That discriminator is what makes this a test rather than a ritual.
#[test]
fn the_local_store_is_not_send() {
    use core::cell::RefCell;
    use core::marker::PhantomData;
    use send_probe::{NotSend as _, Probe};

    // Positive control 1: the reference store *is* `Send`.
    assert!(
        Probe::<MemoryEventStore>(PhantomData).is_send(),
        "probe is broken: MemoryEventStore is Send"
    );
    // Positive control 2: a bare `RefCell` is still `Send`, which is exactly
    // why it is not what this instrument is built on.
    assert!(
        Probe::<RefCell<Vec<SequencedEvent>>>(PhantomData).is_send(),
        "probe is broken: RefCell gives up Sync, not Send"
    );

    // The two negatives: `Rc` is the field that removes `Send`, and the store
    // that holds one inherits the absence.
    assert!(
        !Probe::<Rc<MemoryEventStore>>(PhantomData).is_send(),
        "Rc is Send, so nothing in this file is a !Send store"
    );
    assert!(
        !Probe::<LocalStore>(PhantomData).is_send(),
        "the store every entry point is instantiated against is Send, so the \
         instantiation proves nothing about the weak bound"
    );
}

// ---------------------------------------------------------------------------
// HS-S0031 AC-003 — every entry point, instantiated against that store
// ---------------------------------------------------------------------------

/// Every generic entry point the typed layer exposes, called against a store
/// that is genuinely `!Send`.
///
/// The proof is the **instantiation**, not a bound read off the source: a
/// generic body type-checks against its declared bounds whether or not anything
/// instantiates it, so `SendEventStore` anywhere in the chain is invisible until
/// something hands the chain a store that is not `Send`. That is this call.
///
/// `command-loop`'s `commit_binds_the_weak_flavour` covers `commit_with` alone.
/// This row is the whole surface: `commit`, `commit_with` and — under the
/// feature that ships it — `run_projection`, which M3 could not reach.
#[tokio::test]
async fn every_entry_point_binds_the_weak_flavour() {
    let store = LocalStore::new();

    // `commit` — the JSON door, which is what a first program calls.
    let done = commit(&store, seats(), Retry::once(), |_: &Seats| {
        Ok::<_, Infallible>(vec![Subscribed {
            student: "s1".to_owned(),
        }])
    })
    .await
    .expect("an uncontended commit against a !Send store");
    assert_eq!(done.attempts, 1);

    // `commit_with` — the ungated door, with the codec spelled out.
    let done = commit_with(&store, seats(), &Json, Retry::once(), |_: &Seats| {
        Ok::<_, Infallible>(vec![Subscribed {
            student: "s2".to_owned(),
        }])
    })
    .await
    .expect("an uncontended commit against a !Send store");
    assert_eq!(done.attempts, 1);
    assert_eq!(store.len(), 2);

    // `run_projection` — the entry point HS-S0027 landed, and the one a caller
    // is most likely to hand a store they hold through an `Rc`.
    #[cfg(feature = "unstable-projection")]
    {
        use core::num::NonZeroUsize;

        use happenstance::{MemoryProjectionStore, ProjectionId, run_projection};

        let models = MemoryProjectionStore::new();
        let mut seen = Enrolments {
            id: ProjectionId::new("enrolments"),
            scope: course_tags(),
            count: 0,
        };
        let chunk = NonZeroUsize::new(64).expect("64 is not zero");

        let progressed = run_projection(&store, &models, &mut seen, &Json, chunk)
            .await
            .expect("a replay against a !Send store");
        assert_eq!(progressed.applied, 2);
        assert_eq!(seen.count, 2);
    }
}

/// The import discipline AC-003 folds in, read off the source.
///
/// The instantiation above proves the *bound*; this proves the *spelling*.
/// Having both flavour names in scope makes every method call ambiguous
/// (RS-20-3; CLAUDE.md binding constraint 4), and there is no `dyn EventStore`
/// at all, because the port's methods are generic (RS-20-5).
fn modules_under(dir: &std::path::Path, out: &mut Vec<(std::path::PathBuf, String)>) {
    for entry in std::fs::read_dir(dir).expect("the crate's own src/ is readable") {
        let path = entry.expect("a readable directory entry").path();
        if path.is_dir() {
            modules_under(&path, out);
            continue;
        }
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        let body = std::fs::read_to_string(&path).expect("a source file is readable");
        out.push((path, body));
    }
}

#[test]
fn each_module_imports_one_flavour_name() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut modules = Vec::new();
    modules_under(&src, &mut modules);
    assert!(!modules.is_empty(), "no module was read at all");

    for (path, body) in modules {
        let imports_send = body.lines().any(|line| {
            let trimmed = line.trim_start();
            trimmed.starts_with("use ") && trimmed.contains("SendEventStore")
        });
        assert!(
            !imports_send,
            "{}: imports `SendEventStore`. Generic code binds `EventStore`, \
             the weaker requirement, and the other flavour is reached by full \
             path at the one site that needs it",
            path.display()
        );
        assert!(
            !body.contains("dyn EventStore") && !body.contains("dyn SendEventStore"),
            "{}: there is no `dyn EventStore` — the port's methods are generic, \
             so it is not object-safe",
            path.display()
        );
    }
}

// ---------------------------------------------------------------------------
// HS-S0031 AC-005 — and the `Send` flavour still composes, across a real spawn
// ---------------------------------------------------------------------------

/// A read model over the one domain event this file already has.
#[cfg(feature = "unstable-projection")]
struct Enrolments {
    id: happenstance::ProjectionId,
    scope: Tags,
    count: u64,
}

#[cfg(feature = "unstable-projection")]
impl happenstance::Projection for Enrolments {
    type Event = Subscribed;
    type Store = happenstance::MemoryProjectionStore;

    fn id(&self) -> &happenstance::ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(
        &mut self,
        _event: Self::Event,
        batch: &mut happenstance::MemoryProjectionBatch,
    ) -> Result<(), happenstance::MemoryProjectionStoreError> {
        self.count += 1;
        batch.write("enrolments", self.count);
        Ok(())
    }
}

/// The marker ADR-0009 settled ES-6 with, declared **here** rather than in the
/// library, exactly as that decision says a downstream consumer should.
///
/// The runner cannot produce a `Send` future for an arbitrary `S`, and the
/// reason is structural rather than a slip: on a failure it holds the stop —
/// which carries `S::Error` — across the port's `rollback` await, because the
/// error and the rollback's own outcome are both needed to build one
/// `ProjectionError`. Collapsing it first (RS-25-4) is not available: the value
/// that must survive the await *is* the error. ES-6 is `[FROZEN]` and leaves
/// `Error` unbounded on purpose — `happenstance-cloudflare`'s error holds an
/// `Rc<str>` — so the obligation is the caller's, and ADR-0009 says where it is
/// paid: *"a caller who needs the error itself across an await adds that bound
/// to its own signature and pays for it there; the port never grows one."*
///
/// The rejected alternatives, both recorded by ADR-0009: `Send + Sync` on
/// `Error` itself, which fails the one crate the two-flavour design exists for;
/// and shipping this marker in `happenstance-core`, which is a surface question
/// a later phase owns. A local trait with a blanket impl over a foreign one is
/// ordinary coherence, so declaring it costs nothing.
#[cfg(feature = "unstable-projection")]
trait ThreadSafeEventStore: happenstance::SendEventStore<Error: Send + Sync> {}

#[cfg(feature = "unstable-projection")]
impl<S> ThreadSafeEventStore for S where S: happenstance::SendEventStore<Error: Send + Sync> {}

/// Awaits the **runner** inside `tokio::spawn`, from generic code.
///
/// The bound is `happenstance_core`'s own `spawns_from_generic` verbatim, one
/// marker further out, and each part of it is load-bearing in the way that file
/// records: `Sync` because `Arc<S>: Send` needs `S: Send + Sync` and the body
/// holds `&*events` across an await; `'static` because `tokio::spawn` erases the
/// future into a task that outlives this frame; `Send` redundant-but-stated,
/// since `trait_variant` emits `SendEventStore: Send` as a supertrait.
/// [`ThreadSafeEventStore`] is what supplies the error's own strength, and its
/// documentation is why it is the caller's to supply.
///
/// This is the assertion that survives an `async fn read` refactor, and the
/// extra marker does not blunt it: that refactor makes the *stream* `!Send`,
/// which no bound on `Error` repairs. A "this concrete stream is `Send`"
/// assertion does not survive it — after the refactor the outermost item is a
/// future, `trait_variant` marks *the future* `Send`, and the wrong thing
/// satisfies it (ADR-0008; ES-2).
///
/// It is also the one site in this crate's tests where `SendEventStore` is
/// named, and it is reached by full path rather than imported (RS-20-3).
#[cfg(feature = "unstable-projection")]
fn spawns_the_projection_runner<S>(
    events: Arc<S>,
    models: Arc<happenstance::MemoryProjectionStore>,
    mut projection: Enrolments,
) -> tokio::task::JoinHandle<usize>
where
    S: ThreadSafeEventStore + Send + Sync + 'static,
{
    tokio::spawn(async move {
        // Bound to a local rather than inlined: edition 2024 RPITIT captures
        // every in-scope lifetime, and an inlined temporary is E0716.
        let chunk = core::num::NonZeroUsize::new(64).expect("64 is not zero");

        // `map_or` rather than `?`: the whole `ProjectionError<S::Error, _>` is
        // collapsed to a `usize` here, so nothing of it is alive at the second
        // await below. That is RS-25-4 applied at the one place this caller can
        // apply it — the runner's *internal* hold is what the marker pays for.
        let applied = run_projection(&*events, &*models, &mut projection, &Json, chunk)
            .await
            .map_or(0, |progressed| progressed.applied);

        // A second await against the same borrow, so `&S` really does cross two
        // suspension points rather than one.
        let head = happenstance::SendEventStore::head(&*events)
            .await
            .is_ok_and(|position| position.is_some());

        applied + usize::from(head)
    })
}

#[cfg(feature = "unstable-projection")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn run_projection_spawns_from_generic() {
    use happenstance::{MemoryProjectionStore, ProjectionId};

    let events = Arc::new(MemoryEventStore::new());
    let _seeded = commit_with(&*events, seats(), &Json, Retry::once(), |_: &Seats| {
        Ok::<_, Infallible>(vec![Subscribed {
            student: "s1".to_owned(),
        }])
    })
    .await
    .expect("an uncontended commit");

    let models = Arc::new(MemoryProjectionStore::new());
    let seen = Enrolments {
        id: ProjectionId::new("enrolments"),
        scope: course_tags(),
        count: 0,
    };

    let counted = spawns_the_projection_runner(Arc::clone(&events), Arc::clone(&models), seen)
        .await
        .expect("the spawned task did not panic");

    assert_eq!(counted, 2, "one applied event plus a non-empty head");
    assert_eq!(models.get("enrolments"), Some(1));
}
