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
use happenstance_cloudflare::{CloudflareEventStore, SqlStorage, SqlValue};
use happenstance_testkit::{Capability, Fixture};

/// The text the armed trigger raises with.
///
/// Spelled once and read back by `fixture_contract`'s control, which is what
/// makes it evidence rather than decoration: the string exists **only** inside a
/// SQL trigger body, so a caller-visible error carrying it cannot have come from
/// anywhere but SQLite, through the adapter's own classifier. A JavaScript shim
/// that faked the fault could not put this string there without being the thing
/// that raised it.
pub(crate) const MID_BATCH_FAULT_TEXT: &str =
    "happenstance conformance fault: the event row is refused inside the write path";

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
    ///
    /// The `allow` is the price of a `tests/support` module, and it is worth
    /// stating rather than reaching for. A support module is compiled **once per
    /// declaring target**, so `durable_object_conformance` gets its own copy in
    /// which this method has no caller — dead code there and live code in
    /// `fixture_contract`, from one source. `expect` is wrong for exactly that
    /// reason: it would fire in the target where the method *is* used. The
    /// alternative shapes are worse — a second support module duplicating the
    /// fixture, or moving the method into the test that calls it and losing the
    /// encapsulation of `sql`.
    #[cfg(target_arch = "wasm32")]
    #[allow(dead_code)]
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
    /// and a Durable Object is not that store: its SQL storage takes triggers,
    /// so a row can be made to fail inside the adapter's own `INSERT`, which is
    /// a mechanism the write path cannot absorb. Inheriting the default would
    /// therefore put a sentence in this run's CI log that is not true of this
    /// adapter.
    ///
    /// **Supported**, with the mechanism stated — which CF-39 requires and which
    /// is the whole difference between this and the registered wrong
    /// implementation.
    ///
    /// **The mechanism:** a real SQLite trigger on the `event` table —
    /// `BEFORE INSERT … RAISE(ABORT, …)` behind a countdown the trigger body
    /// decrements — armed by [`Fixture::arm_mid_batch_fault`] below. It is
    /// SQLite that refuses the row, inside the statement the adapter itself
    /// issued, and the refusal travels back through `worker`'s real bindings
    /// into this crate's own classifier. Nothing is mocked between the fault and
    /// the caller, and — the part that matters for a claim the conformance suite
    /// certifies — **nothing about it belongs to the JavaScript host this crate
    /// ships for its own tests.** Swap that host for `workerd` and the trigger
    /// is still a trigger; a fault armed on the shim would have evaporated,
    /// taking this capability's meaning with it. CF-39 names a trigger armed for
    /// one write as its exemplar for exactly that reason.
    ///
    /// **Why the store cannot absorb it.** A Durable Object rejects transaction
    /// control through `sql.exec()`, so there is no `SAVEPOINT` to roll back to,
    /// and the turn's implicit transaction commits when the handler returns
    /// *normally* — which converting a throw into `Err(…)` does. The adapter
    /// therefore undoes a failed batch itself, by deleting the positions it had
    /// assigned. It cannot retry the row (the throw is not transient) and it
    /// cannot ignore it (the row is genuinely not there), so the append fails
    /// and the log is left byte-identical. That is exactly the pair
    /// `append_is_atomic_under_a_mid_batch_fault` exists to observe, and this is
    /// the first fixture in the workspace to let it observe anything.
    ///
    /// **The negative control was run before the claim.** With the arming
    /// removed, `arming_a_mid_batch_fault_makes_the_append_fail` goes red with
    /// the message that names `NoopFaultFixture` — the shape where the override
    /// is present, empty, and reports a green atomicity result for a store
    /// nothing ever faulted. A seam nobody has watched fail is indistinguishable
    /// from that shape from the outside.
    ///
    /// **The standing control is in the tree**, which is the stronger form of
    /// the same thing: `fixture_contract`'s
    /// `the_armed_fault_is_a_real_trigger_inside_the_store` reads
    /// [`MID_BATCH_FAULT_TEXT`] back out of the caller-visible error. That
    /// string exists only in a SQL trigger body, so an error carrying it came
    /// from SQLite and from nowhere else — which is the assertion a shim-armed
    /// fault could never satisfy.
    const MID_BATCH_FAULT: Capability = Capability::SUPPORTED;

    /// Declined **by scope, not by incapacity**, and the difference matters more
    /// here than anywhere else in the workspace: this is one of the two adapters
    /// the read-fault capability was written for.
    ///
    /// `SqlStorage` is synchronous inside the object, so this adapter's `read`
    /// does not fetch a page at a time across an `await` — which is the shape
    /// the capability exists to fault. The moment it does, the injection is
    /// already to hand and is the one [`MID_BATCH_FAULT`](Self::MID_BATCH_FAULT)
    /// uses: a trigger installed through the object's own SQL. Wiring it is not
    /// this story's, and declaring the capability while arming nothing is
    /// `NoopFaultFixture`'s shape one path over — a green result about a read
    /// nothing ever faulted, which is what the rule's own message forbids.
    const READ_FAULT: Capability = Capability::declined(
        "by scope, not by incapacity. This adapter's read does not fetch a page \
         at a time across an await, so there is no fetch between two pages to \
         fail; the injection a paged read would need is the one MID_BATCH_FAULT \
         already uses here, a trigger installed through the object's own SQL, \
         and declaring the capability before that read exists would report a \
         green result about a read nothing had faulted",
    );

    /// 1 MiB, and it is a *stated* ceiling rather than the physical maximum.
    ///
    /// The distinction is the honest part. CF-40 asks that the declared value be
    /// accepted and one more refused as `ExceedsStoreLimit`; it does not ask for
    /// the largest value the store could ever take, and an unstable exact
    /// maximum is how a green run becomes a flaky one. What is declared here is
    /// the adapter's own refusal policy, kept by
    /// `CloudflareEventStore`'s `check_ceilings` **before any SQL is issued** —
    /// which is what lets the refusal name *which* ceiling was crossed, where a
    /// refusal classified from a thrown storage error could not.
    ///
    /// **The derivation**, in full at `experiments/durable-object-limits/README.md`:
    /// a Durable Object's SQL storage documents a 2 MiB maximum row size, and
    /// this adapter's `event` row carries an `event_type` of up to 255 bytes, a
    /// `metadata` blob the contract does not bound at all, the canonical `tags`
    /// encoding — up to 256 KiB at the tag ceiling below — and ADR-0014's
    /// identity columns. Half the row cap leaves a megabyte for all of it.
    ///
    /// **What was measured**, and what was not: probe M2 drove this adapter's own
    /// `append` and `read` on the executing runtime and found a 1 MiB payload
    /// accepted and read back byte-for-byte, with no refusal observable at all up
    /// to 8 MiB; probe M5 found the same answer against an object already holding
    /// a megabyte, so the boundary is not a function of what is stored. The
    /// physical wall was therefore never located on this host — recorded as a
    /// finding in the evidence package rather than papered over — which is
    /// precisely why the declared number is a conservative position well inside
    /// it rather than a search result.
    const MAX_EVENT_DATA_LEN: Option<usize> = Some(1024 * 1024);

    /// 1,024 tags, sixteen times the `MIN_SUPPORTED_TAGS_PER_EVENT` floor.
    ///
    /// Measured against *this adapter's* tag storage rather than against
    /// SQLite's text limit, which is what the derivation has to be about: probe
    /// M3 found 1,024 tags producing exactly 1,024 `event_tag` rows at a cost of
    /// 135,168 bytes, and found 8,192 accepted and read back with no refusal.
    /// The declaration sits at 1,024 because at `MAX_TAG_LEN` the canonical
    /// `tags` blob is then 256 KiB — an eighth of the documented row cap, which
    /// is what keeps the payload ceiling above safe.
    const MAX_TAGS_PER_EVENT: Option<usize> = Some(1024);

    /// 1,024 events, eight times the `MIN_SUPPORTED_EVENTS_PER_BATCH` floor.
    ///
    /// The bound is not SQL. This adapter renders one `INSERT` per event and one
    /// per tag, each its own statement, so no bound-parameter cap is in play —
    /// which is the mistake the derivation had to avoid. What *is* in play is
    /// that the whole batch runs inside one turn with nothing awaited between
    /// rows, which is what makes the compensating discard exact; so the ceiling
    /// is a statement count the object must get through before it yields. Probe
    /// M4 measured a 1,024-event batch with one tag each issuing 2,055
    /// statements and completing, and found 4,096 accepted with no refusal.
    const MAX_EVENTS_PER_BATCH: Option<usize> = Some(1024);

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

    fn arm_mid_batch_fault(&self, after: usize) -> impl Future<Output = ()> {
        // A real SQLite trigger on the `event` table, and the choice of
        // mechanism is the whole of this capability's honesty.
        //
        // The rejected alternative was a hook on the JavaScript host: ask the
        // shim to throw on the *k*-th statement whose text contains
        // `INSERT INTO event (`. It works, it is still the shape the crate's own
        // transport-fault tests use — and it makes the claim a property of the
        // **double** rather than of the store. Swap this crate's `node:sqlite`
        // host for `workerd` and that fault evaporates, taking `MID_BATCH_FAULT`
        // with it and leaving two conformance rules certifying a mechanism that
        // no longer exists. CF-39's own wording asks for a fault *inside the
        // store's own write path*, and names a trigger armed for one write as
        // the exemplar; so that is what this is.
        //
        // The countdown, and why the trigger holds it rather than a threshold
        // computed here: `remaining` is decremented by the trigger body on every
        // insert, and the `RAISE` fires once it has gone negative — so the fault
        // lands on the (`after` + 1)-th row of the table, wherever the object's
        // position counter happens to be, and re-arming is one `DELETE`. A fault
        // on the *first* row would leave nothing on the ground for the store to
        // have to undo, and the rule it feeds would be asking about an append
        // that never started rather than about atomicity.
        //
        // `RAISE(ABORT)` and not `FAIL` or `ROLLBACK`: ABORT backs out the
        // current statement and nothing else, which is exactly the mid-batch
        // shape — the rows already written stay written, and the store has to
        // undo them itself or fail the rule.
        let sql = self.sql.borrow();
        sql.exec(
            "CREATE TABLE IF NOT EXISTS mid_batch_fault (remaining INTEGER NOT NULL)",
            &[],
        )
        .expect("the fault counter applies");
        sql.exec("DELETE FROM mid_batch_fault", &[])
            .expect("a re-arming clears the previous countdown");
        sql.exec(
            "INSERT INTO mid_batch_fault (remaining) VALUES (?)",
            &[SqlValue::Integer(i64::try_from(after).unwrap_or(i64::MAX))],
        )
        .expect("the countdown is set");
        sql.exec(
            &format!(
                "CREATE TRIGGER IF NOT EXISTS mid_batch_fault_fires \
                 BEFORE INSERT ON event BEGIN \
                 UPDATE mid_batch_fault SET remaining = remaining - 1; \
                 SELECT RAISE(ABORT, '{MID_BATCH_FAULT_TEXT}') \
                 FROM mid_batch_fault WHERE remaining < 0; \
                 END"
            ),
            &[],
        )
        .expect("the fault trigger applies");
        core::future::ready(())
    }
}
