//! The fixture contract: what a harness hands the rules, and what a rule may
//! ask of it.
//!
//! A conformance rule needs two things a bare `Fn() -> S` cannot tell apart: a
//! **fresh, isolated backing store**, and a **handle onto one that already
//! exists**. The old signature was asked to promise neither. Its documentation
//! said "a fresh, empty store" while the type would happily have accepted
//! `|| store.clone()`, so no rule could call it twice — it could not know
//! whether the second call bought isolation or sharing. That single ambiguity
//! foreclosed durability, reopen, and every genuinely multi-connection rule at
//! once.
//!
//! [`Fixture`] names the two operations apart. One *fixture instance* is one
//! backing store; each [`connect`](Fixture::connect) returns a handle onto that
//! store; two fixture instances share nothing.
//!
//! # Two fixture traits, one contract
//!
//! [`ProjectionFixture`] says the same three things about a **projection**
//! store. It is a separate trait rather than a second associated type on
//! [`Fixture`] because the associated type is where the port is named:
//! `Fixture::Store: EventStore` binds the wrong one, and an adapter that
//! implements only one of the two ports would otherwise have to invent the
//! other. Everything below the two traits — [`Capability`], [`RuleOutcome`],
//! the one-line skip shape — is shared unchanged, so an author reading one CI
//! log never has to learn two vocabularies.
//!
//! # A rule is handed how to make a fixture, not a made one
//!
//! Every rule takes `impl AsyncFn() -> F`. That is deliberate, and it is what
//! lets *isolation itself* be a conformance rule rather than a testkit-only
//! meta-test: pointing every fixture instance at one temporary directory is an
//! **adapter's** mistake, and no test the testkit writes about the testkit's own
//! fixture could ever observe it. A rule that can call `open()` twice can.
//!
//! # Capabilities
//!
//! Not every store can do everything a rule might want to ask, and the honest
//! answers differ per adapter rather than per rule. [`Capability`] is how a
//! fixture says so, as an associated `const` — available before the rule body
//! runs, and constant after monomorphisation.
//!
//! A rule whose capability is unmet is **still emitted as a test**. It returns
//! [`RuleOutcome::Skipped`] and the harness reports it. The alternative —
//! `#[cfg]`-ing the rule out of the expansion — produces a test binary in which
//! a skipped rule is indistinguishable from a passing one, so an adapter author
//! who declines a capability to turn a red build green gets a green build and no
//! record of the trade.
//!
//! One capability is exempt from that, because it is not a trade at all:
//! [`SECOND_HANDLE`](Fixture::SECOND_HANDLE) is a MUST, and the rule requiring it
//! fails rather than skips. See its documentation for why the enforcement is in
//! the rule and not in a meta-test.
//!
//! # Limits are facts, not trades
//!
//! Three of the associated items below are `Option<usize>` rather than
//! [`Capability`], and the distinction is the one thing about this trait most
//! likely to be got wrong. A capability a fixture declines is a **trade** — it
//! could have co-operated — and the reason it states is the only record of what
//! was given up. A limit is a **fact**: a store either has a ceiling on a
//! payload, a tag count and a batch size, or it has none, and neither answer is
//! a decision anybody has to justify. `None` still produces a reported skip,
//! because the reporting obligation attaches to *a rule that did not run* rather
//! than to *a capability that was declined*.
//!
//! # `Skipped` means *all* of the rule needed the capability
//!
//! This is the distinction an adapter author gets wrong first. A rule returns
//! [`RuleOutcome::Skipped`] only when its **entire** content depends on the
//! capability. A rule whose base assertion runs against every fixture and which
//! merely *strengthens* itself where a second handle exists — reading the same
//! claim back through a second connection, say — has genuinely run, and returns
//! [`RuleOutcome::Ran`].

use core::future::Future;

use happenstance_core::{EventStore, ProjectionId, ProjectionProbe};

/// One isolated backing store, plus the ways a rule is allowed to reach it.
///
/// Implement this for whatever a rule should be given a fresh instance of: a
/// temporary directory holding a SQLite file, a connection pool aimed at a
/// throwaway schema, a `MemoryEventStore` behind an `Arc`. Each instance is one
/// store. Each [`connect`](Self::connect) is one handle onto it.
///
/// # No `Send` bound, and no `trait_variant`
///
/// [`EventStore`] needs two flavours because a *caller* may want to
/// `tokio::spawn` a store's future, and a `Send` bound that helps there is
/// unsatisfiable on `wasm32` (ADR-0001). None of that applies here: **nothing
/// ever spawns a fixture**. The harness owns the executor, and a fixture is only
/// ever driven by the rule holding it. A second flavour would double the surface
/// to buy a property no caller wants — and would exclude precisely the adapters
/// ADR-0001 exists for.
///
/// # Why the methods are spelled `-> impl Future` rather than `async fn`
///
/// The same reason `happenstance-neon`'s transport records: `async fn` in a
/// *public* trait fires rustc's `async_fn_in_trait` lint, and the gate runs
/// `-D warnings`. The desugared form is also strictly better documentation
/// here, because it makes the **absence** of `+ Send` visible at the
/// declaration, which is the whole ADR-0001 point.
///
/// An implementation may still use `async fn` — the two are the same signature
/// after desugaring, and the lint fires only on the declaration.
///
/// # Why `Store` is an owned associated type and not a GAT
///
/// A borrowing `type Store<'a> where Self: 'a` is the obvious shape for "a
/// handle onto this fixture", and it is the shape to keep away from. `where Self:
/// 'a` on a GAT implemented for a foreign trait is one of the five independently
/// necessary ingredients of the rustc ICE this repository already minimised
/// (rust-lang/rust#158983; `experiments/rustc-ice-gat-foreign-trait/`),
/// which still reproduces on 1.97.1.
///
/// Nothing is lost. A pool-backed fixture holds its pool in an `Arc` and returns
/// an owned handle that keeps the pool alive; that is exactly what
/// [`MemoryFixture`](crate::fixtures::MemoryFixture) does with an `Arc` clone,
/// and what a `!Send` fixture would do with an `Rc`. The handle owning a
/// refcount instead of borrowing a lifetime is the difference between a
/// compiling contract and an ICE.
///
/// # Capability constants are checked at codegen, not at `cargo check`
///
/// An empty reason string cannot be written — [`Capability::declined`] rejects
/// it — but for an *associated* const the rejection arrives later than one would
/// like. [`Capability::declined`]'s own documentation states exactly when, and
/// what each of `check`, `clippy` and `build` reports; it is written there rather
/// than here so that a reader looking at the constructor finds it.
pub trait Fixture {
    /// A handle onto this fixture's backing store.
    ///
    /// Bound on [`EventStore`], the flavour with no `Send` requirement, because
    /// it is the weaker one and accepts both kinds of adapter.
    type Store: EventStore;

    /// Whether this fixture can hand out a **second, independent handle** onto
    /// the one backing store.
    ///
    /// A fixture that can is what lets a rule catch the adapter whose
    /// correctness is per-session: a cached `max(position)` fast path, a
    /// per-connection repeatable-read snapshot, an advisory lock scoped to one
    /// pool member.
    ///
    /// **This one is a MUST, and it is the only capability here that is.**
    /// SPECIFICATION.md CF-16 requires every fixture to be able to open a second
    /// handle; [`REOPEN`](Self::REOPEN) is a `SHOULD`, because a volatile store
    /// declining it is an honest answer. Declining *this* is not a trade, it is a
    /// fixture that does not meet the contract, and the shape cannot tell the two
    /// apart: the type says `Capability` in both places.
    ///
    /// # What enforces it
    ///
    /// [`two_handles_observe_each_others_appends`](crate::rules::two_handles_observe_each_others_appends)
    /// **panics** on a declined `SECOND_HANDLE` rather than reporting a skip,
    /// quoting the reason the fixture gave. That is deliberately in the rule
    /// rather than in the testkit's own meta-tests, because the meta-tests never
    /// run in an adapter's CI and the fixture declining a MUST is *there*. An
    /// adapter author who meets a red rule, declines the capability and re-runs
    /// gets a red rule again, with an explanation.
    ///
    /// It stays spelled as a `Capability` so that the rules which merely
    /// *strengthen* themselves where a second handle exists have somewhere to
    /// ask, and so that the failure names the fixture's own words.
    ///
    /// The second line is
    /// `mutation_coverage::capability_skips_are_reported`
    /// (`crates/happenstance-testkit/tests/mutation_coverage.rs`), which drives
    /// a fixture declining everything and asserts the rule *rejects* it, and
    /// separately holds the testkit's own registered instruments to the MUST.
    const SECOND_HANDLE: Capability;

    /// Whether this fixture can be **reopened**: every outstanding handle's
    /// process-level state discarded, such that a subsequent
    /// [`connect`](Self::connect) observes only what was durably committed.
    ///
    /// This is deliberately the weaker of the two operations one might mean by
    /// "restart". A Durable Object's storage outlives its isolate, so it can
    /// discard handle state and read the store again; its isolate cannot be
    /// restarted from inside a test at all. Naming the stronger operation would
    /// have bought a capability every fixture in the workspace declines, which
    /// is a skip reported on every run and evidence of nothing.
    const REOPEN: Capability;

    /// Whether this fixture can make its store **fail part way through writing
    /// one batch**.
    ///
    /// ES-18 says either every event of a batch lands or none does, and the two
    /// rules that check it today both reach the store through the *condition*
    /// path — where a conformant store decides before it writes anything, so a
    /// partial write was never on the table. The case that is left is a fault
    /// between two rows, and reaching it needs the store's co-operation.
    ///
    /// # Why this is a fixture capability and not a decorator
    ///
    /// `SPECIFICATION.md` ES-18 asks for "a fault-injecting decorator over any
    /// `EventStore`", and that shape does not exist. A decorator sits **above**
    /// `append`, which is the unit the port makes atomic: the only fault it can
    /// inject is one that happens before the call or after it, and a decorator
    /// that appended `events[..k]` and then returned `Err` would be asserting
    /// that the *store* must undo a partial batch the *decorator* wrote. Nothing
    /// in the port lets an outside caller reach between two rows of one
    /// transaction, which is precisely why the atomicity being tested is worth
    /// having.
    ///
    /// So the injection is the adapter's: a trigger that raises on the third
    /// insert, a `CHECK` constraint armed for one write, a connection killed
    /// mid-statement. Every store that can do it does it differently, which is
    /// what makes it a capability rather than testkit machinery.
    ///
    /// A store with no way to fail one row of a batch declines, and the default
    /// below is that answer. It is defaulted rather than required — unlike
    /// [`SECOND_HANDLE`](Self::SECOND_HANDLE) and [`REOPEN`](Self::REOPEN),
    /// which every fixture must answer deliberately — because an in-memory store
    /// has no fault to inject and demanding an answer would buy one more line of
    /// boilerplate per fixture and no information.
    const MID_BATCH_FAULT: Capability = Capability::declined(
        "this fixture cannot make its store fail between two rows of one batch; \
         nothing in the port can reach inside an `append`, so the injection has \
         to come from the adapter and this one has none to offer",
    );

    /// The largest `data` payload this fixture's store accepts, in bytes, or
    /// `None` if it has no ceiling.
    ///
    /// # Why this is an `Option<usize>` and not a [`Capability`]
    ///
    /// The three constants here look like capabilities and are deliberately not
    /// spelled as ones. A declined [`Capability`] is a **trade**: the fixture
    /// could have co-operated and chose not to, and the reason string is the
    /// record of that choice, printed on every run so a reviewer and a user of
    /// the adapter can both read what was given up. `None` here is not a trade.
    /// It is a store reporting a **fact about itself** — that it has no ceiling
    /// on this value — and there is nothing it could have done differently and
    /// no reason it owes anybody. Asking it to write one would put a fiction in
    /// the CI log.
    ///
    /// What `None` shares with a declined capability is the *reporting*
    /// obligation, and only that: the rule cannot run, so it must say so. It
    /// therefore returns [`RuleOutcome::Skipped`] through the same path, carrying
    /// [`NO_STORE_LIMITS`] as the capability name and [`NO_CEILING_REASON`] as
    /// the reason. A rule silently omitted is indistinguishable in CI output from
    /// a rule that passed, which is CF-18's whole argument and does not stop
    /// being true because the thing being reported is a fact rather than a
    /// decision.
    ///
    /// # What stating a number commits the store to
    ///
    /// [`append_reports_exceeded_store_limits`](crate::rules::append_reports_exceeded_store_limits)
    /// appends a payload of exactly this many bytes and requires it to be
    /// **accepted**, then a payload one byte larger and requires it to be refused
    /// as `AppendError::ExceedsStoreLimit { limit: StoreLimit::EventDataLen, .. }`
    /// — never as `AppendError::Store`, and never by truncating. A number that is
    /// not where the store's real ceiling is fails that rule in one direction or
    /// the other, which is the point: this constant is the store's documented
    /// limit (VT-21 requires one), and the suite is what holds the documentation
    /// to the code.
    ///
    /// A store MAY state a ceiling below `MIN_SUPPORTED_EVENT_DATA_LEN`, and it
    /// will then fail `store_accepts_the_guaranteed_minimum_payload` —
    /// correctly, because VT-21 makes 65,536 bytes a floor every store must
    /// clear. The two rules are independent and both are owed an answer.
    const MAX_EVENT_DATA_LEN: Option<usize> = None;

    /// The largest number of tags on one event this fixture's store accepts, or
    /// `None` if it has no ceiling.
    ///
    /// See [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN) for why these three
    /// are `Option<usize>` rather than [`Capability`], and for what stating a
    /// number commits the store to. The floor is `MIN_SUPPORTED_TAGS_PER_EVENT`
    /// (VT-22).
    const MAX_TAGS_PER_EVENT: Option<usize> = None;

    /// The largest number of events this fixture's store accepts in one append,
    /// or `None` if it has no ceiling.
    ///
    /// See [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN). The floor is
    /// `MIN_SUPPORTED_EVENTS_PER_BATCH` (VT-24).
    ///
    /// Note what this is **not**: VT-24 forbids a `MAX_EVENTS_PER_BATCH` constant
    /// in `happenstance-core` and forbids an `EventBatch` newtype to carry one,
    /// because the bound is adapter-specific. That prohibition is about the
    /// *contract*. Naming the number here is the opposite move — one store saying
    /// what one store does, in the one place a conformance rule can read it.
    ///
    /// There is deliberately no `MAX_QUERY_ITEMS` beside these three. A
    /// query-item refusal is not an append outcome, so it has no `StoreLimit`
    /// variant to be reported through and no rule to gate.
    const MAX_EVENTS_PER_BATCH: Option<usize> = None;

    /// Arms the store so that the **next** append of more than `after` events
    /// fails while writing event `after + 1`.
    ///
    /// The fault fires once. Whether the store answers `Err` or swallows it is
    /// the adapter's business; what ES-18 requires is that the two answers stay
    /// consistent with what is in the log afterwards, which is what
    /// [`append_is_atomic_under_a_mid_batch_fault`](crate::rules::append_is_atomic_under_a_mid_batch_fault)
    /// checks.
    ///
    /// # Panics
    ///
    /// The provided body panics, for
    /// [`reopen`](Self::reopen)'s reason and with the same two ways of reaching
    /// it: a fixture that declares [`MID_BATCH_FAULT`](Self::MID_BATCH_FAULT)
    /// supported and forgets the override, or a rule that reached here without a
    /// `require!` gate.
    fn arm_mid_batch_fault(&self, after: usize) -> impl Future<Output = ()> {
        let _ = (self, after);
        async move {
            panic!(
                "`Fixture::arm_mid_batch_fault` was called but not implemented: \
                 either this fixture declares MID_BATCH_FAULT supported and does \
                 not override it, or a rule reached it without a \
                 `require!(F: MID_BATCH_FAULT)` gate"
            );
        }
    }

    /// Opens a handle onto this fixture's backing store.
    ///
    /// Async because a real fixture acquires its handle over I/O — a pool
    /// checkout, a connection, an HTTP client's first request.
    ///
    /// # Panics
    ///
    /// Implementations panic rather than returning `Result`. A fixture that
    /// cannot connect is a broken **test environment**, not a non-conformant
    /// adapter, and a `Result` here would put "the database is down" into the
    /// same channel as "the adapter is wrong" — where the suite's own error
    /// messages would then have to guess which it was reading.
    fn connect(&self) -> impl Future<Output = Self::Store>;

    /// Closes and reopens the underlying storage, discarding every outstanding
    /// handle's process-level state.
    ///
    /// After this returns, a fresh [`connect`](Self::connect) must observe
    /// exactly what was durably committed and nothing else. Handles obtained
    /// before the call may stop working; a rule must not use one afterwards.
    ///
    /// # Panics
    ///
    /// The provided body panics, and it names both ways of reaching it because
    /// only one of them is the suite's fault. Nothing in the trait ties
    /// `REOPEN: Capability::SUPPORTED` to overriding this method, so the
    /// reachable path an adapter author actually hits is: declare it supported,
    /// forget the override, run the suite. Blaming a missing `require!` gate
    /// there would send them reading the testkit for a bug that is in their
    /// fixture.
    fn reopen(&self) -> impl Future<Output = ()> {
        // `let _ = self;` is load-bearing rather than decorative: the body never
        // touches `self`, that is an `unused_variables` warning, and the gate
        // denies warnings.
        let _ = self;
        async move {
            panic!(
                "`Fixture::reopen` was called but not implemented: either this \
                 fixture declares REOPEN supported and does not override \
                 `reopen`, or a rule reached `reopen` without a \
                 `require!(F: REOPEN)` gate"
            );
        }
    }
}

/// One isolated **projection** store, plus the ways a projection rule is
/// allowed to reach it.
///
/// [`Fixture`]'s sibling, and a second trait rather than a second associated
/// type on the first, because [`Fixture::Store`] binds [`EventStore`] — the
/// wrong port. An adapter may implement one of the two ports and not the other,
/// and a single trait would oblige a projection-only adapter to invent an event
/// store to satisfy a bound no projection rule reads.
///
/// Implement it for whatever a rule should be given a fresh instance of: a
/// temporary directory holding a SQLite file, a connection pool aimed at a
/// throwaway schema, a `MemoryProjectionStore` behind an `Arc`. Each instance is
/// one store. Each [`connect`](Self::connect) is one handle onto it.
///
/// # Why `Store` is bound on the **probe** rather than on the port
///
/// [`ProjectionProbe`] is the write seam the suite drives an adapter's read
/// model through, and it is a supertrait of `ProjectionStore` — so one bound
/// buys both. Binding the port instead would compile and would quietly buy a
/// suite that cannot see a read model at all: with `ProjectionStore` alone, the
/// only things generic code can do with a batch are commit it and roll it back,
/// and the rule carrying this port's entire reason for existing degenerates into
/// a checkpoint test that a store writing *only* checkpoints passes.
///
/// Binding the probe here is what turns "your store must be observable" from a
/// convention into a compile error: an adapter that has not implemented
/// [`ProjectionProbe`] cannot name a type that satisfies this trait, so it
/// cannot invoke the suite, and by CLAUDE.md's rule it does not exist.
///
/// # No `Send` bound, and no `trait_variant`
///
/// The bare flavour, never `SendProjectionStore`: it is the weaker requirement
/// and accepts both kinds of adapter, and only one of the two names may be in
/// scope per module. Nothing ever spawns a fixture — the harness owns the
/// executor — so the argument that gives the *port* two flavours (ADR-0001) does
/// not reach here, and a second flavour would double the surface to buy a
/// property no caller wants.
///
/// # Why the methods are spelled `-> impl Future` rather than `async fn`
///
/// [`Fixture`]'s reason, unchanged: `async fn` in a *public* trait fires rustc's
/// `async_fn_in_trait` lint and the gate runs `-D warnings`. The desugared form
/// also puts the **absence** of `+ Send` at the declaration, where a reader can
/// see it. An implementation may still write `async fn` — the two are the same
/// signature after desugaring, and the lint fires only on the declaration.
///
/// # Why `Store` is an owned associated type and not a GAT
///
/// The same rustc ICE [`Fixture`] records, for the same five ingredients, still
/// reproducing on 1.97.1 (`experiments/rustc-ice-gat-foreign-trait/`). Hand back
/// an owned handle holding a refcount;
/// [`MemoryProjectionFixture`](crate::fixtures::MemoryProjectionFixture) is the
/// worked example.
///
/// # Capability constants, and where an empty reason fires
///
/// Three constants, all required and all answered deliberately:
/// [`SECOND_HANDLE`](Self::SECOND_HANDLE) is a MUST, and
/// [`RESET_REFUSAL`](Self::RESET_REFUSAL) and
/// [`COMMIT_FAULT`](Self::COMMIT_FAULT) are the port's two genuinely
/// declinable capabilities. A declined one must name a reason —
/// [`Capability::declined`] rejects the empty string in a `const fn` `assert!`
/// — but on an **associated** const that rejection arrives later than one would
/// like. An associated const is evaluated lazily, only when monomorphised code
/// reads it, which is *after* `cargo check` and `cargo clippy` have both
/// stopped. **It fails at codegen, so `cargo build` and `cargo test` catch it
/// and `cargo check` and `cargo clippy` do not** — a green `check` is not
/// evidence here.
///
/// This is that failure, and reading the constant is the load-bearing line: a
/// fixture nobody ever looks at compiles perfectly well.
///
/// ```compile_fail
/// use happenstance_core::MemoryProjectionStore;
/// use happenstance_testkit::fixtures::MemoryProjectionHandle;
/// use happenstance_testkit::{Capability, ProjectionFixture};
///
/// struct Reasonless(std::sync::Arc<MemoryProjectionStore>);
///
/// impl ProjectionFixture for Reasonless {
///     type Store = MemoryProjectionHandle;
///
///     const SECOND_HANDLE: Capability = Capability::SUPPORTED;
///     // The whole difference from the block below.
///     const RESET_REFUSAL: Capability = Capability::declined("");
///     const COMMIT_FAULT: Capability = Capability::declined(
///         "this store cannot make a commit fail once it has been accepted",
///     );
///
///     fn connect(&self) -> impl core::future::Future<Output = Self::Store> {
///         core::future::ready(MemoryProjectionHandle::new(std::sync::Arc::clone(&self.0)))
///     }
/// }
///
/// fn main() {
///     let _ = <Reasonless as ProjectionFixture>::RESET_REFUSAL;
/// }
/// ```
///
/// The doctest above is spelled bare `compile_fail`, never
/// `compile_fail,E0080`: rustdoc on 1.97.1 silently ignores an error-code
/// annotation it cannot match, so the stricter-looking spelling is the weaker
/// check (`Capability::declined` records the measurement).
///
/// Bare `compile_fail` passes when the snippet fails to compile for *any*
/// reason, so the **twin** below is what makes the pair sound. It is the same
/// snippet with one expression changed — the empty string becomes a sentence —
/// and it must compile. A typo, a renamed item or a wrong path breaks the twin,
/// and a broken twin is a hard test failure, so the only thing the pair can be
/// reporting is the one expression that differs between them.
///
/// ```
/// use happenstance_core::MemoryProjectionStore;
/// use happenstance_testkit::fixtures::MemoryProjectionHandle;
/// use happenstance_testkit::{Capability, ProjectionFixture};
///
/// struct Reasoned(std::sync::Arc<MemoryProjectionStore>);
///
/// impl ProjectionFixture for Reasoned {
///     type Store = MemoryProjectionHandle;
///
///     const SECOND_HANDLE: Capability = Capability::SUPPORTED;
///     const RESET_REFUSAL: Capability = Capability::declined(
///         "this store holds no protection policy, so there is no projection it \
///          could decline to reset",
///     );
///     const COMMIT_FAULT: Capability = Capability::declined(
///         "this store cannot make a commit fail once it has been accepted",
///     );
///
///     fn connect(&self) -> impl core::future::Future<Output = Self::Store> {
///         core::future::ready(MemoryProjectionHandle::new(std::sync::Arc::clone(&self.0)))
///     }
/// }
///
/// let _ = <Reasoned as ProjectionFixture>::RESET_REFUSAL;
/// ```
///
/// # Examples
///
/// The whole trait, over the reference store — this is what
/// [`projection_store_conformance!`](crate::projection_store_conformance) is
/// handed:
///
/// ```
/// use std::sync::Arc;
///
/// use happenstance_core::MemoryProjectionStore;
/// use happenstance_testkit::fixtures::MemoryProjectionHandle;
/// use happenstance_testkit::{Capability, ProjectionFixture};
///
/// struct MyFixture(Arc<MemoryProjectionStore>);
///
/// impl ProjectionFixture for MyFixture {
///     type Store = MemoryProjectionHandle;
///
///     const SECOND_HANDLE: Capability = Capability::SUPPORTED;
///     const RESET_REFUSAL: Capability = Capability::declined(
///         "this store protects nothing, so it has no reset to refuse",
///     );
///     const COMMIT_FAULT: Capability = Capability::declined(
///         "this store applies both halves of a commit under one lock and has \
///          no write it can be made to fail between them",
///     );
///
///     fn connect(&self) -> impl core::future::Future<Output = Self::Store> {
///         core::future::ready(MemoryProjectionHandle::new(Arc::clone(&self.0)))
///     }
/// }
/// ```
pub trait ProjectionFixture {
    /// A handle onto this fixture's backing projection store.
    ///
    /// Bound on [`ProjectionProbe`], which implies `ProjectionStore` — see the
    /// trait documentation for why the *probe* is the bound that earns its keep
    /// and the port is the one that does not.
    type Store: ProjectionProbe;

    /// Whether this fixture can hand out a **second, independent handle** onto
    /// the one backing projection store.
    ///
    /// **This one is a MUST**, as it is on [`Fixture`], and for a sharper
    /// reason. PS-1 — the read-model write and the checkpoint write become
    /// durable together or not at all — is only observable from *outside* the
    /// connection that made the commit: a store whose commit is visible only to
    /// its own session satisfies every single-handle assertion and loses the
    /// row, or the checkpoint, or both, the moment anything else looks. So
    /// every rule in this family reads back through a fresh
    /// [`connect`](Self::connect), and a fixture that cannot open one cannot
    /// observe the invariant this port exists for at all.
    ///
    /// Declining it is therefore **not a trade** the suite may record as a skip
    /// — it is a fixture that does not meet the contract — and the type cannot
    /// tell the two apart, because both spell [`Capability`].
    ///
    /// # What enforces it
    ///
    /// The rules themselves, through `must!` rather than `require!`, so the
    /// enforcement is on the path an adapter's own CI executes. An adapter
    /// author who meets a red rule, writes
    /// `const SECOND_HANDLE: Capability = Capability::declined("…")` and re-runs
    /// gets a red rule again, quoting their own stated reason back at them — not
    /// a green suite and one `SKIP` line. The second line is
    /// `mutation_coverage::projection_capability_skips_are_reported`, which
    /// drives the whole enumeration against a fixture declining everything and
    /// asserts each of those rules *rejects* it.
    ///
    /// It stays spelled as a [`Capability`] rather than as a `bool` so the
    /// failure carries the fixture's own words.
    const SECOND_HANDLE: Capability;

    /// Whether this fixture's store can be made to **refuse** a reset for a
    /// projection it protects.
    ///
    /// The projection port's one genuinely declinable capability, and a real
    /// trade rather than a fact: PS-18 makes refusal a *mechanism* the port
    /// supplies and leaves what to protect to the domain, so a store with no
    /// protection policy has nothing to refuse and declining is the honest
    /// answer. `MemoryProjectionStore` is exactly that store, and
    /// [`MemoryProjectionFixture`](crate::fixtures::MemoryProjectionFixture)
    /// declines with the real reason.
    ///
    /// It is **required rather than defaulted**, unlike
    /// [`Fixture::MID_BATCH_FAULT`], and that is a deliberate difference of one
    /// line per fixture. A default would have to carry a *testkit-written*
    /// reason, and the projection family's declension policy is that the fixture
    /// writes the reason — a store's account of a trade only it can describe.
    /// The one standing exception to that policy on the event-store side
    /// ([`NO_CEILING_REASON`]) exists because "this store has no ceiling" is the
    /// same sentence for every store that says it; "this store refuses no reset"
    /// is not, because *why* it refuses none is the interesting half.
    ///
    /// It is read by exactly one rule,
    /// [`refused_reset_changes_nothing`](crate::projection::rules::refused_reset_changes_nothing),
    /// which is PS-18's; a fixture that declines it gets that rule as a reported
    /// skip carrying its own stated reason, and a fixture that declares it must
    /// also override [`protect_from_reset`](Self::protect_from_reset), which is
    /// the mechanism this constant gates.
    const RESET_REFUSAL: Capability;

    /// Whether this fixture can make a `commit` **report failure**.
    ///
    /// PS-1's second conjunct is a claim about a commit that failed: the read
    /// model and the checkpoint must be exactly as they were. Nothing a caller
    /// holds can make a conformant `commit` fail — that is the property under
    /// test — so, exactly as with [`Fixture::MID_BATCH_FAULT`], the injection
    /// belongs to the adapter: a trigger that raises on the third row, a `CHECK`
    /// armed for one write, a connection killed between the read-model write and
    /// the checkpoint write. Every store that can do it does it differently,
    /// which is what makes it a capability rather than testkit machinery.
    ///
    /// # Why it is *required* rather than defaulted
    ///
    /// [`Fixture::MID_BATCH_FAULT`] carries a default declension and this one
    /// deliberately does not, for [`RESET_REFUSAL`](Self::RESET_REFUSAL)'s
    /// reason: a default has to carry a *testkit-written* reason, and this
    /// family's declension policy is that the fixture writes it. "This store has
    /// no ceiling" is the same sentence for every store that says it, which is
    /// why [`NO_CEILING_REASON`] exists; *why a particular store cannot make a
    /// commit fail* is not — an in-memory map applies both halves under one lock,
    /// a one-shot HTTP backend has no interactive transaction to abort, and a
    /// pooled adapter usually can. The cost is one line per fixture and the
    /// return is that no fixture author is left un-asked.
    ///
    /// # What declaring it commits the fixture to
    ///
    /// [`failed_commit_leaves_both_unchanged`](crate::projection::rules::failed_commit_leaves_both_unchanged)
    /// arms the fault and requires the next `commit` to answer `Err`. A fixture
    /// whose [`arm_commit_fault`](Self::arm_commit_fault) does nothing would
    /// otherwise turn that rule into a green result about a store nothing ever
    /// faulted, which is CF-39's argument one port over. So a store that can
    /// absorb every fault its fixture is able to arm MUST **decline** this
    /// capability with that as its stated reason, rather than declare it and
    /// contribute an `Ok`.
    const COMMIT_FAULT: Capability;

    /// Arms the store so that the **next** `commit` fails.
    ///
    /// The fault fires once, and where inside the commit it fires is the
    /// adapter's business: what PS-1's second conjunct requires is that a commit
    /// which reported failure left the read model and the checkpoint exactly as
    /// they were, whichever half the store had got to.
    ///
    /// # Panics
    ///
    /// The provided body panics, for
    /// [`Fixture::arm_mid_batch_fault`]'s reason and with the same two ways of
    /// reaching it: a fixture that declares [`COMMIT_FAULT`](Self::COMMIT_FAULT)
    /// supported and forgets the override, or a rule that reached here without a
    /// gate. That is what stops "declared and never implemented" from passing
    /// vacuously — it aborts loudly instead.
    fn arm_commit_fault(&self) -> impl Future<Output = ()> {
        let _ = self;
        async move {
            panic!(
                "`ProjectionFixture::arm_commit_fault` was called but not \
                 implemented: either this fixture declares COMMIT_FAULT \
                 supported and does not override it, or a rule reached it \
                 without a `require!(F: COMMIT_FAULT)` gate"
            );
        }
    }

    /// Puts `id` under this store's protection, so the next
    /// [`reset`](happenstance_core::ProjectionStore::reset) of it is refused.
    ///
    /// [`arm_commit_fault`](Self::arm_commit_fault)'s sibling, and it exists for
    /// the same reason: the capability above is a claim, and a rule cannot
    /// *exercise* the claim without telling the store which projection to
    /// protect. Nothing a caller holds can make a conformant `reset` answer
    /// [`ResetError::Refused`](happenstance_core::ResetError::Refused) —
    /// PS-18's own words are that the port supplies the mechanism and the domain
    /// decides what to protect, so the domain is where the choice lives and a
    /// fixture is how a suite reaches it.
    ///
    /// It takes the [`ProjectionId`] rather than declaring a protected one,
    /// which is the difference between a rule that can name its own subject and
    /// a rule that has to share one id with every other rule in the family. An
    /// adapter implements it however its policy is spelled: a row in a
    /// protected-projections table, a `CHECK`, a `beforeDelete` trigger, a
    /// hard-coded list.
    ///
    /// **This is not a fourth capability**, and deliberately so. The projection
    /// family's declension set is the three constants above; this is the
    /// *mechanism* the second of them gates, exactly as `arm_commit_fault` is
    /// the mechanism the third gates. A fixture that declines
    /// [`RESET_REFUSAL`](Self::RESET_REFUSAL) never has it called.
    ///
    /// # Panics
    ///
    /// The provided body panics, for
    /// [`arm_commit_fault`](Self::arm_commit_fault)'s reason and with the same
    /// two ways of reaching it: a fixture that declares `RESET_REFUSAL`
    /// supported and forgets the override, or a rule that reached here without a
    /// `require!(F: RESET_REFUSAL)` gate. "Declared and never implemented" then
    /// aborts loudly rather than passing vacuously — a fixture whose body did
    /// nothing would leave
    /// [`refused_reset_changes_nothing`](crate::projection::rules::refused_reset_changes_nothing)
    /// asserting about a store nothing had ever asked to protect anything.
    fn protect_from_reset(&self, id: &ProjectionId) -> impl Future<Output = ()> {
        let _ = (self, id);
        async move {
            panic!(
                "`ProjectionFixture::protect_from_reset` was called but not \
                 implemented: either this fixture declares RESET_REFUSAL \
                 supported and does not override it, or a rule reached it \
                 without a `require!(F: RESET_REFUSAL)` gate"
            );
        }
    }

    /// Opens a handle onto this fixture's backing store.
    ///
    /// Async because a real fixture acquires its handle over I/O — a pool
    /// checkout, a connection, an HTTP client's first request. A fixture whose
    /// `connect` is a refcount bump should return [`core::future::ready`]
    /// rather than an `async move` block, so it does not pretend to do I/O it
    /// does not do.
    ///
    /// # Panics
    ///
    /// Implementations panic rather than returning `Result`, for
    /// [`Fixture::connect`]'s reason and it is worth restating here rather than
    /// linking: a fixture that cannot connect is a broken **test environment**,
    /// not a non-conformant adapter, and a `Result` would put "the database is
    /// down" into the same channel as "the adapter is wrong" — where the suite's
    /// own messages would then have to guess which one they were reading.
    fn connect(&self) -> impl Future<Output = Self::Store>;
}

/// Whether a fixture supports one optional operation, and if not, why not.
///
/// # Why this is an opaque struct rather than a public enum
///
/// The obvious spelling is `enum Capability { Supported, Declined(&'static str) }`,
/// and it has one hole: `Declined("")` is a perfectly good value of that type.
/// The requirement that a declined capability *names a reason* would then be
/// prose, enforced by a meta-test somebody has to remember to write.
///
/// Making the field private moves the check into the only constructor, where an
/// `assert!` inside a `const fn` can reject the empty string at compile time.
/// See [`declined`](Self::declined) for exactly when that fires — it is later
/// than one would like, and worth knowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// No `#[non_exhaustive]`: the field is already private, which seals the type
// against outside construction more tightly than the attribute would, and the
// sealing is the entire point of the shape.
pub struct Capability(Option<&'static str>);

impl Capability {
    /// The fixture supports this operation, and rules requiring it will run.
    pub const SUPPORTED: Self = Self(None);

    /// The fixture does not support this operation, for the stated reason.
    ///
    /// The reason is not a formality. It is printed on every run for every rule
    /// the decision skips, so it lands in the adapter's CI log where both a
    /// reviewer and a user of the adapter can read it. Write the real one:
    /// *why* this store cannot do it, not *that* it cannot.
    ///
    /// # Panics
    ///
    /// Panics if `reason` is empty. Because this is a `const fn`, a panic at
    /// const-evaluation time is a **compile error** rather than a run-time one:
    ///
    /// ```compile_fail
    /// use happenstance_testkit::Capability;
    ///
    /// const REOPEN: Capability = Capability::declined("");
    ///
    /// fn main() {
    ///     let _ = REOPEN;
    /// }
    /// ```
    ///
    /// The doctest above is spelled bare `compile_fail`, never
    /// `compile_fail,E0080`. This diagnostic family has bitten the repository
    /// before: rustdoc on 1.97.1 silently ignores an error-code annotation it
    /// cannot match, so the stricter-looking spelling is the weaker check.
    ///
    /// **Where it does *not* fire.** A free `const` like the one above fails at
    /// `cargo check`. An **associated** const in a `Fixture` impl does not: an
    /// associated const is evaluated lazily, only when monomorphised code reads
    /// it, and that is after `check` and `clippy` have both stopped. It fails at
    /// codegen — so `cargo build` and `cargo test` catch it and `cargo clippy`
    /// does not.
    #[must_use]
    pub const fn declined(reason: &'static str) -> Self {
        assert!(
            !reason.is_empty(),
            "a declined capability must name a reason: it is printed on every \
             run and is the only record of the trade"
        );
        Self(Some(reason))
    }

    /// Whether rules requiring this capability should run.
    #[must_use]
    pub const fn is_supported(self) -> bool {
        self.0.is_none()
    }

    /// The stated reason for declining, or `None` if the capability is
    /// supported.
    #[must_use]
    pub const fn reason(self) -> Option<&'static str> {
        self.0
    }
}

/// The `capability` name a rule reports when a fixture states **no ceiling at
/// all** on any store limit.
///
/// Not a [`Fixture`] associated const, and it names all three because the rule
/// gated on it needs all three to be absent before it has nothing to do. A skip
/// naming only one of them would send an adapter author to look for the constant
/// they did set.
pub const NO_STORE_LIMITS: &str = "MAX_EVENT_DATA_LEN, MAX_TAGS_PER_EVENT, MAX_EVENTS_PER_BATCH";

/// The reason a rule reports when a fixture states no ceiling on any store
/// limit.
///
/// Written by the testkit rather than by the fixture, which is the one place the
/// skip machinery here differs from [`Capability::declined`]'s, and it is the
/// difference the shape is for: a declined capability's reason is the adapter's
/// account of a trade it made and only the adapter can write it, while "this
/// store has no ceiling" is the same sentence for every store that says it.
/// Asking each fixture to phrase it would buy a paraphrase per adapter and no
/// information.
pub const NO_CEILING_REASON: &str = "this fixture states no ceiling for any store limit, so there is no capacity \
     refusal for a rule to observe; a store with no ceiling is reporting a fact \
     about itself rather than declining to co-operate";

/// What a conformance rule did.
///
/// There is no `Failed` variant, and its absence is deliberate: a failing rule
/// **panics**. That is what fails the enclosing `#[test]`, and it is what gives
/// libtest a message, a location and a backtrace to print. Threading failure
/// back as a value would buy a second, worse reporting channel.
///
/// The `#[must_use]` is what turns "an emitter must report the skip" from prose
/// into a build failure. A caller-supplied emitter that drops the outcome warns,
/// and CI denies warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Deliberately exhaustive. The house rule puts `#[non_exhaustive]` on types that
// will grow; this one is designed not to — see the note above on `Failed`.
#[must_use = "a rule's outcome must be reported, or a skipped rule is \
              indistinguishable in CI output from a rule that passed"]
pub enum RuleOutcome {
    /// The rule ran, and every assertion in it held.
    Ran,
    /// The rule required a capability this fixture declines, and did nothing.
    Skipped {
        /// The name of the [`Fixture`] associated const, e.g. `"REOPEN"`.
        capability: &'static str,
        /// The fixture's stated reason, from [`Capability::declined`].
        reason: &'static str,
    },
}

impl RuleOutcome {
    /// The one line worth showing a human, or `None` when there is nothing to
    /// say.
    ///
    /// This is the target-independent half of reporting, and it is public
    /// because *where* a line goes is a property of the harness rather than of
    /// the rule. [`report`](Self::report) writes it to stdout, which is right on
    /// every target that has one; `__emit_wasm` routes it to `console.log`,
    /// because `wasm32-unknown-unknown` does not (see [`report`](Self::report)).
    /// A caller-supplied emitter for a runtime the testkit has never heard of —
    /// CF-23's extension point — picks its own sink from here.
    ///
    /// The returned [`Option`] is itself `#[must_use]`, so the obligation to
    /// report survives one step further down the chain.
    #[must_use]
    pub fn skip_line(self, rule: &str) -> Option<String> {
        match self {
            Self::Ran => None,
            Self::Skipped { capability, reason } => Some(format!(
                "SKIP {rule}: fixture declines `{capability}` — {reason}"
            )),
        }
    }

    /// Reports this outcome to stdout, under the name of the rule that produced
    /// it.
    ///
    /// Prints nothing for [`Ran`](Self::Ran) — libtest already reports a test
    /// that passed — and one `SKIP` line for a skip.
    ///
    /// # Two honest limitations, one per target
    ///
    /// Natively, libtest suppresses a *passing* test's stdout unless it is run
    /// with `--show-output` (or `--nocapture`), which is why the gate passes it.
    /// That makes the line reachable by a human; it does not make anyone read
    /// it. The machine-checked half of the obligation is the
    /// `mutation_coverage::capability_skips_are_reported` meta-test in
    /// `crates/happenstance-testkit/tests/mutation_coverage.rs`, which asserts on
    /// [`RuleOutcome`] *values* rather than on stdout — and not this function.
    ///
    /// On `wasm32-unknown-unknown` this function is a **no-op**, measured rather
    /// than assumed: the suite was run under `wasm-bindgen-test-runner` with
    /// `--nocapture` and no `SKIP` line appeared. That target's `std` has no
    /// host stdio to write to and declares no import that would give it one, and
    /// `wasm-bindgen-test` hooks `console.*` rather than `println!`. So the wasm
    /// emitter does not call this; it calls [`skip_line`](Self::skip_line) and
    /// hands the result to `console_log!`, which the runner does capture.
    pub fn report(self, rule: &str) {
        if let Some(line) = self.skip_line(rule) {
            println!("{line}");
        }
    }
}
