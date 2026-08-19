//! The conformance fixture for the Durable Object adapter.
//!
//! Declared by both integration targets in this directory — `fixture_contract`,
//! which holds this fixture's own contract honest, and
//! `durable_object_conformance`, which hands it to the shipped
//! `event_store_conformance!` macro unchanged.
//!
//! # Why it lives in `tests/` rather than in `src/`
//!
//! A dependency-graph reason, and it is the only one. `happenstance-testkit` is
//! a **dev**-dependency of this crate; an `impl Fixture` in `src/` would make
//! the conformance suite a runtime edge of a crate this initiative intends to
//! publish, so every consumer of the adapter would resolve the suite that
//! measures it. A `tests/support` module declared by both targets buys the same
//! reachability with none of that — and the reachability is the closed question
//! (the *host* is what has to be nameable from a second compilation unit, and
//! it is, at [`happenstance_cloudflare::host`]).
//!
//! # What one instance is, and what one handle is
//!
//! One `CloudflareFixture` is **one Durable Object**: a fresh
//! [`DurableObjectHost`] with its own storage that nothing else can see. Two
//! instances therefore share nothing, which is not a nicety —
//! `two_fixture_instances_observe_none_of_each_others_appends` is a conformance
//! rule precisely because a fixture that quietly points every instance at one
//! object passes every other rule in the suite, and no test the testkit writes
//! about its *own* fixture could ever observe it.
//!
//! Each [`connect`](Fixture::connect) is **one more handle onto that object** —
//! a fresh `CloudflareEventStore` over a clone of the same `SqlStorage`, whose
//! `Clone` aliases the storage rather than copying it. That is the mechanism
//! CF-16 asks for, not a workaround: the two handles are two views of one
//! object, so an append through either is visible through the other and neither
//! mints a second store-id incarnation.
//!
//! # Which suite families this fixture is handed to, and which it is not
//!
//! The **event-store** family, in full, through the shipped
//! `event_store_conformance!` — no emitter of our own, no rule list, no `#[cfg]`
//! over any individual rule.
//!
//! The **concurrency** family is not invoked, and that is a reasoned
//! non-invocation rather than an omission. `event_store_concurrency_conformance!`
//! binds `F::Store: EventStore + Send` and its module is
//! `#[cfg(not(target_arch = "wasm32"))]`, because the family needs threads to
//! spawn. This adapter's store is `!Send` by construction and this target has no
//! threads, so a `!Send` adapter *cannot* invoke it and is not expected to —
//! and it costs nothing here, because a Durable Object is a single-threaded
//! actor with exclusive ownership of its storage, so there is no second writer
//! for a race to elect a winner between.
//!
//! The **model** family is not invoked either, for a different reason worth
//! stating in the same breath: it sits behind the testkit's off-by-default
//! `proptest` feature, and `fixtures::strategies` and `model` carry the same
//! target condition — because a Cargo feature is *not* target-scoped, so
//! `--all-features` would otherwise set it on `wasm32` where `proptest` is not
//! in the dependency graph at all.
//!
//! # The handle owns a refcount
//!
//! The handle owns a refcount and never borrows a lifetime, which is what lets
//! `Fixture::Store` be an ordinary associated type instead of a GAT — see the
//! trait's own note on the rustc ICE that shape reproduces. `MemoryFixture` does
//! it with an `Arc`; every refcount on this path is an `Rc`, because a real
//! `JsValue` is `Send + Sync` on `wasm32` builds without `atomics` and an `Arc`
//! here would hand this crate's central `!Send` property back by accident.

use core::cell::RefCell;
use core::future::Future;

use happenstance_cloudflare::host::DurableObjectHost;
use happenstance_cloudflare::{CloudflareEventStore, SqlStorage};
use happenstance_testkit::{Capability, Fixture};

/// One Durable Object's storage, as the conformance suite's [`Fixture`].
pub(crate) struct CloudflareFixture {
    /// The object itself, retained so its storage can be bound again.
    ///
    /// This is what makes `REOPEN` answerable at all: a Durable Object's
    /// storage outlives its isolate, so a second binding taken off the same
    /// `state` observes exactly what was durably committed and none of the
    /// previous binding's process-level state.
    host: DurableObjectHost,
    /// The binding every handle is cloned from.
    ///
    /// Replaced — not mutated — by [`Fixture::reopen`]. A `RefCell` and not a
    /// `Cell`, because `SqlStorage` is not `Copy` and the borrow is never held
    /// across an await.
    sql: RefCell<SqlStorage>,
}

impl CloudflareFixture {
    /// Stands up a fresh Durable Object and applies the schema to it once.
    ///
    /// `migrate()` runs here and **not** in [`Fixture::connect`], deliberately.
    /// It is idempotent, so a per-handle call would be correct and still wrong:
    /// schema creation would become a per-connection effect, and — worse — it
    /// would mask a fixture whose two "handles" were actually two objects,
    /// because each would have been handed a schema on the way out.
    ///
    /// # Panics
    ///
    /// If the Durable Object cannot be stood up, or if the schema does not
    /// apply. `Fixture::connect` returns `Self::Store` rather than a `Result`
    /// for the same reason this panics: a fixture that cannot reach its store is
    /// a broken **test environment**, not a non-conformant adapter, and
    /// reporting it as a rule failure would mislabel it.
    #[must_use]
    pub(crate) fn new() -> Self {
        let host = DurableObjectHost::new();
        let sql = host.storage();
        CloudflareEventStore::new(sql.clone())
            .migrate()
            .expect("the Durable Object host applies this adapter's schema");
        Self {
            host,
            sql: RefCell::new(sql),
        }
    }

    /// Every statement this fixture's object has been asked to run, oldest
    /// first.
    ///
    /// Read off the host's own log rather than counted here, so that a test can
    /// assert on *how many times* something was issued. The difference between
    /// one migration per object and one per connection is invisible everywhere
    /// else, because `CREATE TABLE IF NOT EXISTS` is idempotent.
    ///
    /// `wasm32`-only, and the gate is not tidiness: its one caller needs a real
    /// object under it, so on the host this would be an item nothing can reach
    /// — and `dead_code` is denied under `-D warnings`.
    #[cfg(target_arch = "wasm32")]
    #[must_use]
    pub(crate) fn issued_statements(&self) -> Vec<String> {
        happenstance_cloudflare::host::statements(&self.sql.borrow())
    }
}

impl Default for CloudflareFixture {
    fn default() -> Self {
        Self::new()
    }
}

impl Fixture for CloudflareFixture {
    /// The bare, `!Send` flavour, and the only adapter in the workspace that
    /// implements it.
    type Store = CloudflareEventStore;

    /// CF-16 is a MUST, and this runtime meets it with no trade at all.
    ///
    /// A second handle is a second `CloudflareEventStore` over a clone of one
    /// `SqlStorage`, and `SqlStorage: Clone` aliases the object's storage rather
    /// than copying it. Declining this to turn a red rule green would fail
    /// louder rather than quieter — `two_handles_observe_each_others_appends`
    /// uses `must!` and panics quoting the fixture's own reason.
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    /// Supported, and this runtime is the case the trait's own documentation
    /// names.
    ///
    /// A Durable Object's storage outlives its isolate, so *discarding handle
    /// state and reading the store again* is something this runtime genuinely
    /// does — [`Fixture::reopen`] below re-derives the binding from the object's
    /// retained `state`, which throws away the cursor generation and the cached
    /// store-id incarnation and touches not one row. Restarting the isolate from
    /// inside a test is the stronger operation, and it is not what this
    /// capability names.
    const REOPEN: Capability = Capability::SUPPORTED;

    /// Restated here rather than inherited, and the restatement is the point.
    ///
    /// The trait's default is written for a store with **no fault to inject**,
    /// and a Durable Object is not that store: its SQL storage can be made to
    /// throw on a chosen statement, which is a mechanism the adapter's write
    /// path cannot absorb. Inheriting the default would therefore put a sentence
    /// in this run's CI log that is not true of this adapter.
    ///
    /// Declined **for now** with the honest reason: the mechanism exists but
    /// arming it is a verdict owed to the real runner, and CF-39 requires a
    /// fixture claiming the capability to state the mechanism it uses. Settling
    /// that against the executed suite is `measured-store-limits`', and it is a
    /// one-line change to this constant plus one override by construction.
    const MID_BATCH_FAULT: Capability = Capability::declined(
        "this Durable Object host can throw on a chosen statement, so the \
         mechanism exists; what has not been settled against an executed \
         conformance run is which statement of this adapter's write path is the \
         k-th row's, and CF-39 requires a fixture claiming the capability to \
         name the mechanism rather than to hope",
    );

    /// `None`, stated rather than inherited — and the value is not this story's.
    ///
    /// A ceiling is a **fact**, not a trade: stating `Some(N)` promises that
    /// exactly `N` bytes are accepted and `N + 1` is refused as
    /// `AppendError::ExceedsStoreLimit`, in both directions, on every gate run.
    /// A guessed number therefore fails `append_reports_exceeded_store_limits`
    /// one way or the other, which is the rule doing its job.
    ///
    /// Locating the number is a **measurement** against the real runner and
    /// belongs to `measured-store-limits`. What is owed here is that the
    /// constant is *written out*, so that a deliberate `None` is distinguishable
    /// from a ceiling inherited by omission — the two read identically in a CI
    /// log and only this source can tell them apart.
    const MAX_EVENT_DATA_LEN: Option<usize> = None;

    /// `None`, stated. See [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN);
    /// the value is `measured-store-limits`'.
    const MAX_TAGS_PER_EVENT: Option<usize> = None;

    /// `None`, stated. See [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN);
    /// the value is `measured-store-limits`'.
    const MAX_EVENTS_PER_BATCH: Option<usize> = None;

    fn connect(&self) -> impl Future<Output = Self::Store> {
        // `ready` rather than `async move`: acquiring this handle is a refcount
        // bump and one JS property read, and pretending otherwise would hide
        // that a real fixture's `connect` does I/O and this one does not.
        //
        // `migrate()` is deliberately absent — see `CloudflareFixture::new`.
        core::future::ready(CloudflareEventStore::new(self.sql.borrow().clone()))
    }

    fn reopen(&self) -> impl Future<Output = ()> {
        // The whole operation, and it is one line because the host is what makes
        // it one: a fresh binding off the same object's `state`. Every handle
        // taken before this call keeps working against the old binding, which is
        // why the trait says a rule must not use one afterwards; every handle
        // taken after it sees only what was durably committed.
        //
        // What must *not* happen here is a new object (the events would go with
        // it) or a re-mint of the store id (the store would stop recognising its
        // own past, and `reopened_store_does_not_reissue_an_event_id` is what
        // would say so). Neither is possible by construction: the `state` is the
        // same value, and the incarnation lives in a `store_meta` row that this
        // call does not touch.
        *self.sql.borrow_mut() = self.host.storage();
        core::future::ready(())
    }
}
