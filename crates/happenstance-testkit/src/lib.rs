// The README's code blocks are compiled as doctests. `cfg(doctest)` keeps the
// prose out of the rendered documentation — it would otherwise appear twice, once
// here and once in the module docs below — while still type-checking every
// example. A README example that does not compile is worse than no example: it
// is the first thing a reader tries, and the first impression the crate makes.
// (D10)
#![cfg_attr(doctest, doc = include_str!("../README.md"))]
//! The DCB conformance suite for happenstance event store adapters.
//!
//! "Storage agnostic" is a claim about behaviour, and a claim about behaviour
//! is worth exactly as much as the test that checks it. This crate is that
//! test. An adapter in this workspace is not considered to exist until it
//! invokes [`event_store_conformance!`] and passes.
//!
//! # Usage
//!
//! In your adapter's `tests/` directory or a `#[cfg(test)]` module:
//!
//! ```
//! # #[cfg(feature = "doctest-only")]
//! happenstance_testkit::event_store_conformance!(MyFixture::new());
//! ```
//!
//! The macro takes an expression that builds a **fixture** — see [`Fixture`] —
//! and expands to one `#[tokio::test]` per rule, so a failure names the rule
//! that broke rather than reporting "conformance failed".
//!
//! Your crate needs `tokio` with the `macros` and `rt` features in
//! `dev-dependencies`.
//!
//! # What a fixture is, and why it is not a closure
//!
//! A fixture instance is **one isolated backing store**; each
//! [`connect`](Fixture::connect) on it returns **one handle** onto that store.
//! The distinction is the whole of [`Fixture`]'s reason to exist. Its
//! predecessor was a bare `Fn() -> S` that documented "a fresh, empty store"
//! while accepting `|| store.clone()`, so a rule could not call it twice without
//! knowing which it had been given — and that one ambiguity foreclosed
//! durability, reopen, and every genuinely multi-connection rule.
//!
//! Rules are handed `impl AsyncFn() -> F`, not a made fixture. A rule that can
//! make two isolated stores can check that they *are* isolated, which matters
//! because pointing every fixture at one temporary directory is an **adapter's**
//! mistake and no test over the testkit's own fixture could see it.
//!
//! A fixture also declares what it can do, as [`Capability`] constants. A rule
//! requiring a capability the fixture declines is **still emitted as a test**;
//! it returns [`RuleOutcome::Skipped`] and the harness prints the fixture's
//! stated reason. `#[cfg]`-ing it out instead would make a skipped rule
//! indistinguishable in CI output from a passing one.
//!
//! [`fixtures::MemoryFixture`] is the reference implementation and the one to
//! read before writing your own.
//!
//! # Choosing a harness
//!
//! `#[tokio::test]` is a default, not a requirement. The per-test wrapper is a
//! parameter — an *emitter* macro — because the testkit is in no position to
//! know which runtime an adapter is tested on, and a `cfg` ladder in here would
//! mean every new runtime needs a testkit release. Three emitters ship, and all
//! three are demonstrated in this crate's `tests/`:
//!
//! | Emitter | Wrapper | Adapter needs |
//! |---|---|---|
//! | `__emit_tokio` (default) | `#[tokio::test]` | `tokio` with `macros`, `rt` |
//! | `__emit_blocking` | `#[test]` + [`block_on`] | nothing |
//! | `__emit_wasm` | `#[wasm_bindgen_test]` | `wasm-bindgen-test` |
//!
//! ```
//! # macro_rules! ignore { ($($t:tt)*) => {} }
//! # ignore! {
//! happenstance_testkit::event_store_conformance!(
//!     mod_name = dcb_conformance_blocking,
//!     emit = happenstance_testkit::__emit_blocking,
//!     fixture = MyFixture::new()
//! );
//! # }
//! ```
//!
//! A runtime none of those cover needs no change here: write a `macro_rules!`
//! that accepts a comma-separated list of identifiers and hand it to
//! [`for_each_event_store_rule!`] yourself.
//!
//! # Where the rule set lives
//!
//! In exactly one place: [`for_each_event_store_rule!`]. Every harness, and the
//! `no_orphan_rules` meta-test, is built by invoking it. A rule that exists in
//! [`rules`] without appearing there is a rule nothing runs, which is the
//! failure the meta-test is for.
//!
//! "Exactly one place" is per rule **family** (CF-22), and there are four.
//!
//! `projection_store_conformance!` is the fourth, and the only one that checks a
//! different port. It takes a [`ProjectionFixture`] rather than a [`Fixture`] —
//! one isolated projection store per instance — and expands to one
//! `#[tokio::test]` per projection rule through its own enumeration,
//! [`for_each_projection_store_rule!`], beside the rules it names. Pick a
//! harness exactly as you would for the event-store family: the default arm is
//! tokio, `__emit_projection_blocking` needs no runtime at all, and
//! `__emit_projection_wasm` routes a skipped rule's stated reason to
//! `console_log!` rather than to stdout, which does not exist on
//! `wasm32-unknown-unknown`. The default module name differs from
//! `dcb_conformance`, so one file may invoke both suites.
//!
//! An adapter reaches it by implementing `ProjectionProbe` beside its
//! `ProjectionStore` impl — the write seam a suite that has never heard of the
//! adapter drives its read model through, in `happenstance-core` behind the
//! off-by-default `conformance` feature.
//!
//! **It declares what it can do in the same vocabulary**, and there is
//! deliberately no second one: [`ProjectionFixture`] carries
//! [`Capability`] constants, a rule it declines returns the same
//! [`RuleOutcome::Skipped`] and prints the same one-line `SKIP` shape, so an
//! author reading one CI log never has to learn two. Three constants, all
//! answered deliberately —
//! [`SECOND_HANDLE`](ProjectionFixture::SECOND_HANDLE), which is a **MUST**
//! because every rule in the family reads back through a fresh handle;
//! [`RESET_REFUSAL`](ProjectionFixture::RESET_REFUSAL), which a store with no
//! protection policy declines honestly; and
//! [`COMMIT_FAULT`](ProjectionFixture::COMMIT_FAULT), which is how a store says
//! whether it can make a `commit` report failure — the only way to observe PS-1's
//! second conjunct, and something no caller can do from outside. A fourth
//! switch, `READS_THROUGH_BATCH`, is on the *probe* rather than on the fixture,
//! because whether a batch can be read through is a property of the batch type
//! rather than of the fixture's environment.
//!
//! `event_store_model_conformance!` generates operation sequences and checks the
//! store against a model of the log rather than against a worked example. It
//! carries its own enumeration, `for_each_model_rule!`, beside the rules it
//! names — the `model` module says why, and what it is blind to. It is
//! additive: it replaces no rule in the table below, and it catches no defect
//! whose content is concurrency, durability, a second handle, or the empty
//! batch.
//!
//! `event_store_concurrency_conformance!` runs `concurrency::CONTENDERS`
//! contenders on real threads against one backing store. It is **opt-in**: an
//! adapter invokes it separately, its bound is `F::Store: EventStore + Send`,
//! and a `!Send` adapter cannot invoke it and is not expected to. It is additive
//! too, and pointedly so — it does **not** retire
//! [`rules::racing_conditional_appends_elect_one_winner`], which fixes the
//! semantics a race must have; what it adds is a second caller, so that a probe
//! followed by an insert stops being indistinguishable from an atomic
//! check-and-write. There is deliberately no timeout anywhere in it; the
//! `concurrency` module says why, and the answer is CF-33.
//!
//! **Neither the model nor the concurrency family's names are intra-doc
//! links**, and both paragraphs above
//! spell them plainly on purpose. Each module is absent on some configuration
//! this crate is documented under, and rustdoc treats an unresolved link as a
//! hard error: `model` is behind the off-by-default `proptest` feature, so a
//! link would break `cargo doc` with default features; `concurrency` is
//! `#[cfg(not(target_arch = "wasm32"))]`, so a link would break
//! `cargo doc --target wasm32-unknown-unknown`. That is the D13 failure this
//! workspace has already paid for once.
//!
//! An earlier version of this paragraph said the concurrency module "needs no
//! feature, so linking *into* it is safe on any target that has it" — true, and
//! misleading in the same sentence, because `wasm32-unknown-unknown` is the
//! target this crate's whole two-flavour story exists for and is the one that
//! does not have it. An intra-doc link to `concurrency::CONTENDERS` was written
//! on that reasoning and did fail, with *no item named concurrency in scope*.
//! Nothing in
//! `cargo xtask ci` caught it: the gate runs `cargo doc` for the host and
//! `cargo build`/`cargo check` for wasm32, and never rustdoc for wasm32. The
//! absence of that step is recorded here rather than left as the reason this is
//! green — adding it is a candidate for phase 4, and until then this discipline
//! is the whole of the protection.
//!
//! The projection family's names *are* links, and the difference is the whole
//! of the rule rather than an inconsistency: `projection` is declared
//! unconditionally — no feature, no target gate — so there is no configuration
//! this crate is documented under in which the target is absent. A link is safe
//! exactly when the item cannot disappear.
//!
//! # Writing a projection adapter from outside this workspace
//!
//! The bar here is held for an author this repository did not write, and the
//! extension surface is exactly two items: this crate's
//! [`projection_store_conformance!`] and `happenstance-core`'s
//! `ProjectionProbe`. Six steps, in this order.
//!
//! **1. Take the two dependencies, and note which is which.**
//!
//! ```toml
//! [dependencies]
//! happenstance-core = { version = "0.2", features = ["conformance"] }
//!
//! [dev-dependencies]
//! happenstance-testkit = "0.2"
//! tokio = { version = "1", features = ["macros", "rt"] }
//! ```
//!
//! `conformance` is one flag on a dependency your adapter already has. It pulls
//! in no crate and implies no other feature — not `std`, not `memory` — so your
//! *normal* dependency graph does not grow at all. This crate is a
//! dev-dependency and stays one.
//!
//! **2. Implement `ProjectionStore` for your store, in `src/`.**
//!
//! **3. Implement `ProjectionProbe` for the same type, in `src/` beside it** —
//! not in `tests/`. It is the write seam a suite that has never heard of your
//! store drives your read model through, and its home is the contract crate for
//! a reason you meet immediately if you put the impl in the other obvious place:
//! `tests/` is a **different crate**, where neither the trait nor your type is
//! local, and the orphan rule answers `error[E0117]`.
//!
//! **4. Write a [`ProjectionFixture`] in your `tests/`.** One instance is one
//! isolated backing store; each [`connect`](ProjectionFixture::connect) is one
//! handle onto it. The fixture type is defined *there*, so this impl belongs
//! exactly where the last one did not.
//!
//! **5. Say what your store cannot do, and why.** Its three [`Capability`]
//! constants are required rather than defaulted, so no author is left un-asked.
//! A rule you decline is **still emitted as a test**: it returns
//! [`RuleOutcome::Skipped`] carrying your own sentence, because a rule absent
//! from a binary is indistinguishable in CI output from a rule that passed.
//!
//! **6. Invoke the suite. One line, and you name no rule.**
//!
//! ```
//! # macro_rules! ignore { ($($t:tt)*) => {} }
//! # ignore! {
//! happenstance_testkit::projection_store_conformance!(MyFixture::new());
//! # }
//! ```
//!
//! That line expands in *your* crate, which is why the expansion never assumes
//! what you have in scope: it spells the fixture trait as
//! `$crate::__private::ProjectionFixture`, through a hidden module this crate
//! keeps for the purpose, so the invocation works whether or not you imported
//! the trait and whatever you renamed the dependency to. You never name that
//! module yourself, and it is the one part of this page that is invisible until
//! it is missing.
//!
//! A green run then means what the rule table below says and no more; a red one
//! names the rule that broke. `examples/outside-projection-adapter/` in this
//! repository is an adapter written against this page and nothing else, kept in
//! the tree so the page cannot quietly stop being sufficient.
//!
//! # What is checked
//!
//! Every rule traces to a MUST in the [specification][spec], plus the
//! properties an adapter can plausibly get wrong:
//!
//! | Area | Rules |
//! |---|---|
//! | The fixture contract | two fixture instances share nothing; two handles onto one store observe each other's appends, on the read side and through an append condition; an acknowledged write survives a reopen |
//! | Query semantics | types OR within an item; tags AND within an item; items OR across a query; `Query::all`; supersets match, partial overlaps do not; an untagged event is still matched; nothing is yielded twice; item order does not change the result |
//! | Read options | `from` is inclusive; `backwards` reverses order; `limit` truncates **matches**, not scanned rows; every option composes with a multi-item query; a store nobody has written to reads as empty |
//! | Positions | unique; strictly monotonic; gaps permitted |
//! | Append | atomic; a rejection changes nothing; a batch interrupted by an injected fault lands whole or not at all; an empty batch is refused, and refused *before* the condition is evaluated; the returned position is the last written |
//! | Value edges | a zero-length payload survives; `metadata: None` and `Some(<empty>)` stay two values; an identifier at the 255-byte bound round-trips and still matches itself; non-ASCII identifiers do too; and the four guaranteed minima — 65,536 bytes of payload, 64 tags, a 128-item query, a 128-event batch |
//! | Append conditions | the full matrix, including the exact `after` boundary, tags on both sides of the verdict, an empty store, and `after` beyond the head |
//! | Concurrency | racing appends with overlapping conditions — exactly one wins. The **opt-in** third family adds contention on real threads: one winner of N contenders, K disjoint boundaries admitting exactly K commits, positions unique under concurrent appends, `append` returning the caller's own last position rather than the head, and a reader that never sees a partial batch |
//! | Re-entrancy | two `append` futures on one handle both complete and exactly one wins; a live read stream does not block an append |
//! | Position visibility | two `append` futures interleaved by hand on one thread: nothing becomes visible below a position a reader has already observed |
//! | Projections (a second port, a fourth family) | a commit advances the projection's checkpoint to the position it was given, read back through a fresh handle; and the read-model write and the checkpoint write become durable **together or not at all**, never one — the invariant `ProjectionStore` exists for. All seventeen rules §4.11 assigns to an adapter's own suite, including `fresh_projection_has_no_checkpoint` — PS-38's second sentence, which ADR-0030 minted rather than widening `[FROZEN]` PS-19 to reach it |
//!
//! [spec]: https://dcb.events/specification/
//!
//! # Which flavour to test
//!
//! The macro binds on [`EventStore`](happenstance_core::EventStore), the flavour
//! with no `Send` bound, so it accepts both kinds of adapter. If your adapter
//! implements [`SendEventStore`](happenstance_core::SendEventStore) — as every
//! native one should — you get that bound checked for free, because
//! `SendEventStore` implies `EventStore`.

#![doc(html_no_source)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Named `contract`, not `fixture`: one letter from the public `fixtures` module
// below, which the specification names by that exact path and which therefore
// cannot be renamed to make room.
mod contract;
// Target-gated and nothing else: the concurrency family needs no optional
// dependency, only threads. `wasm32-unknown-unknown` has none it can spawn, and
// the family is opt-in besides — an adapter that cannot race is not asked to.
#[cfg(not(target_arch = "wasm32"))]
pub mod concurrency;
pub mod fixtures;
// The same two conditions `fixtures::strategies` carries, and for the same
// reason (CF-21): a **feature is not target-scoped**, so `--all-features` sets
// `proptest` on `wasm32` too, where the crate is not in the dependency graph at
// all. Without the target condition this module is compiled for a target its
// dependency does not build for, and the mandatory wasm32 step of
// `cargo xtask ci` is what finds it.
#[cfg(all(feature = "proptest", not(target_arch = "wasm32")))]
#[cfg_attr(docsrs, doc(cfg(feature = "proptest")))]
pub mod model;
// Unconditional, unlike its two nearest templates. `concurrency` is
// `#[cfg(not(target_arch = "wasm32"))]` and `model` is behind an optional
// dependency; a projection module gated either way would make the wasm32 harness
// unbuildable by construction, and the mandatory wasm32 `--tests` step would
// then pass while proving nothing about the family it was added for.
pub mod projection;
mod registry;
mod suite;

pub use contract::{
    Capability, Fixture, NO_BATCH_READ_PATH, NO_BATCH_READ_PATH_REASON, NO_CEILING_REASON,
    NO_STORE_LIMITS, ProjectionFixture, RuleOutcome,
};
pub use registry::block_on;
pub use suite::rules;

/// VT-26's compile test, run from a crate that really is downstream of
/// `happenstance-core`.
///
/// [`Query::Items`] carries `#[non_exhaustive]`, so outside the defining crate
/// it can be matched but not constructed. The seal is load-bearing rather than
/// tidy: `Query::Items(Vec::new().into_boxed_slice())` is a query with no items,
/// `Query::matches` then answers `false` for every event, and an
/// `AppendCondition` over it can never be violated — a conditional append that
/// is silently unconditional, which is a lost update with no diagnostic
/// anywhere. The check has to run *downstream*, because inside
/// `happenstance-core` the variant is ordinary and nothing there can fail.
///
/// **Rejects:** a `Query::Items` without `#[non_exhaustive]`. Strike the
/// attribute and the first block below compiles, so the test fails with
/// *"Test compiled successfully, but it's marked `compile_fail`"*.
///
/// ```compile_fail
/// use happenstance_core::{Query, QueryItem};
///
/// let items = vec![QueryItem::of_types(["CourseDefined"]).unwrap()].into_boxed_slice();
/// let query = Query::Items(items);
/// assert!(!query.is_all());
/// ```
///
/// # Why a doctest, in a repository this one has bitten before
///
/// `tests/` cannot host it: an integration test that fails to compile fails the
/// build, so the only instrument that can assert a *non*-compile is one rustdoc
/// runs. And a bare `compile_fail` passes when the snippet fails to compile for
/// **any** reason — measured in `experiments/wire-format/`, where of four
/// spellings of one assertion a type-name typo, a misspelt trait and a wrong
/// crate path all reported ok against a false claim. Annotating the code does
/// not fix it: rustdoc on 1.97.1 silently ignores an error-code annotation it
/// cannot match, so `compile_fail,E0639` is the weaker check rather than the
/// stricter one (`happenstance-core/src/event.rs`'s `from_static`, and
/// [`Capability::declined`], both record this).
///
/// The **twin** below is what makes the pair sound. It is the same snippet with
/// one expression changed — `Query::Items(items)` becomes
/// `Query::from_items(items)` — and it must *compile*. A typo, a renamed item
/// or a wrong path breaks the twin, and a broken twin is a hard test failure,
/// so the only thing the pair can be reporting is the one expression that
/// differs between them.
///
/// ```
/// use happenstance_core::{Query, QueryItem};
///
/// let items = vec![QueryItem::of_types(["CourseDefined"]).unwrap()].into_boxed_slice();
/// let query = Query::from_items(items).unwrap();
/// assert!(!query.is_all());
///
/// // Reading the variant from outside the crate still works, which is what
/// // variant-level `#[non_exhaustive]` buys over enum-level: matching is
/// // allowed and construction is not.
/// //
/// // **In the struct-pattern spelling, and only that one.** `Query::Items(..)`
/// // — the tuple form the specification and `query.rs` both name — is
/// // `error[E0603]: tuple variant `Items` is private` downstream, because a
/// // tuple pattern resolves through the variant's *constructor* and
/// // `#[non_exhaustive]` is what makes that constructor private outside the
/// // crate. Braces reach the fields directly and never name the constructor,
/// // so `{ .. }` and `{ 0: …, .. }` both compile. Measured here, on 1.97.1;
/// // `Query::items()` remains the accessor nobody has to know this for.
/// assert!(matches!(query, Query::Items { .. }));
/// assert!(matches!(query, Query::Items { 0: ref held, .. } if held.len() == 1));
/// ```
///
/// `trybuild` would pin the diagnostic outright and make the twin unnecessary;
/// ADR-0015 records that phase 6 owns that dependency decision.
///
/// [`Query::Items`]: happenstance_core::Query::Items
#[cfg(doctest)]
mod query_items_is_not_constructible_downstream {}

/// Generates the full DCB conformance suite for an event store adapter.
///
/// Takes an expression that produces a [`Fixture`] — one isolated backing store
/// per instance. See the [crate documentation](crate) for what is checked and
/// what the adapter must provide.
///
/// It is an **expression** rather than a type deliberately, and the awkward case
/// is the one that decides it: a Postgres fixture needs a connection URL, and a
/// type with an argument-less constructor would have to reach into the
/// environment for it.
///
/// The rule list is not written here; it comes from
/// [`for_each_event_store_rule!`](crate::for_each_event_store_rule), which is
/// the only place it is written at all.
///
/// # Examples
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// use happenstance_testkit::fixtures::MemoryFixture;
///
/// happenstance_testkit::event_store_conformance!(MemoryFixture::new());
/// # }
/// ```
///
/// Choosing a different harness — the emitter is a parameter, so the testkit
/// never decides which async runtime an adapter is tested on:
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// happenstance_testkit::event_store_conformance!(
///     mod_name = blocking_conformance,
///     emit = happenstance_testkit::__emit_blocking,
///     fixture = MemoryFixture::new()
/// );
/// # }
/// ```
///
/// # Migrating from `factory =`
///
/// The keyword was `factory =` and took a store expression. There is no
/// deprecated arm, because nothing in this workspace is published yet and this
/// is the last release in which that is true. Change the keyword and hand it a
/// [`Fixture`] instead of a store.
#[macro_export]
macro_rules! event_store_conformance {
    // The general form. Listed first so that arm matching never has to back out
    // of `fixture = $fixture:expr` to reach it.
    (mod_name = $mod_name:ident, emit = $emit:path, fixture = $fixture:expr) => {
        mod $mod_name {
            #![allow(clippy::unwrap_used, unused_imports)]

            use super::*;

            // The fixture expression, hoisted behind a function so that emitters
            // need to know nothing about it — including its type, which they
            // could not know, since an expression is all this macro was handed.
            //
            // `impl Trait` costs nothing here: an `async fn` returning an opaque
            // type satisfies `impl AsyncFn() -> F` exactly as a concrete one
            // does, and `F` infers to the opaque type across the crate boundary
            // with no turbofish and no named type. Verified against a concrete
            // control before this shape was chosen.
            //
            // `async` because a real fixture's construction is I/O; the function
            // is re-evaluated per test, so every rule gets its own fixture.
            async fn __conformance_fixture() -> impl $crate::__private::Fixture {
                $fixture
            }

            // `$emit` is `$crate::`-qualified by the caller. A bare name here
            // would be substituted verbatim and resolve in the *adapter's*
            // crate, where the testkit's emitters do not exist.
            $crate::for_each_event_store_rule!($emit);
        }
    };
    (mod_name = $mod_name:ident, fixture = $fixture:expr) => {
        $crate::event_store_conformance!(
            mod_name = $mod_name,
            emit = $crate::__emit_tokio,
            fixture = $fixture
        );
    };
    ($fixture:expr) => {
        $crate::event_store_conformance!(
            mod_name = dcb_conformance,
            emit = $crate::__emit_tokio,
            fixture = $fixture
        );
    };
}

/// Generates the projection conformance suite for a `ProjectionStore` adapter.
///
/// Takes an expression that produces a [`ProjectionFixture`] — one isolated
/// backing projection store per instance, each `connect()` one handle onto it.
/// See the [crate documentation](crate) for what is checked and what the adapter
/// must provide.
///
/// The rule list is not written here; it comes from
/// [`for_each_projection_store_rule!`](crate::for_each_projection_store_rule),
/// which is the only place it is written at all. A caller writes one line and
/// names no rule, so a failure names the rule that broke rather than reporting
/// "conformance failed".
///
/// # What your adapter needs first
///
/// `ProjectionFixture::Store` is bound on `ProjectionProbe`, so an adapter
/// implements that beside its `ProjectionStore` impl. It lives in
/// `happenstance-core` behind the off-by-default `conformance` feature — one
/// flag on a dependency you already have, and no new edge in your dependency
/// graph.
///
/// # Examples
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// use happenstance_testkit::fixtures::MemoryProjectionFixture;
///
/// happenstance_testkit::projection_store_conformance!(MemoryProjectionFixture::new());
/// # }
/// ```
///
/// Choosing a different harness — the emitter is a parameter here for the same
/// reason it is on the event-store family, and the default module name differs
/// from `dcb_conformance` so both suites can be invoked from one file:
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// happenstance_testkit::projection_store_conformance!(
///     mod_name = projection_conformance_blocking,
///     emit = happenstance_testkit::__emit_projection_blocking,
///     fixture = MyProjectionFixture::new()
/// );
/// # }
/// ```
#[macro_export]
macro_rules! projection_store_conformance {
    // The general form. Listed first so that arm matching never has to back out
    // of `fixture = $fixture:expr` to reach it.
    (mod_name = $mod_name:ident, emit = $emit:path, fixture = $fixture:expr) => {
        mod $mod_name {
            #![allow(clippy::unwrap_used, unused_imports)]

            use super::*;

            // Named exactly as `event_store_conformance!`'s is, so a
            // caller-supplied emitter — CF-23's extension point — drives any
            // family without knowing which one it was handed. `macro_rules!`
            // hygiene applies to local variables rather than items, which is
            // what lets one macro's expansion define this function and another's
            // refer to it.
            async fn __conformance_fixture() -> impl $crate::__private::ProjectionFixture {
                $fixture
            }

            // `$emit` is `$crate::`-qualified by the caller. A bare name here
            // would be substituted verbatim and resolve in the *adapter's*
            // crate, where the testkit's emitters do not exist.
            $crate::for_each_projection_store_rule!($emit);
        }
    };
    (mod_name = $mod_name:ident, fixture = $fixture:expr) => {
        $crate::projection_store_conformance!(
            mod_name = $mod_name,
            emit = $crate::__emit_projection_tokio,
            fixture = $fixture
        );
    };
    ($fixture:expr) => {
        $crate::projection_store_conformance!(
            mod_name = projection_conformance,
            emit = $crate::__emit_projection_tokio,
            fixture = $fixture
        );
    };
}

/// Re-exports the macro expansions need to name, so an adapter is not required
/// to have this crate in scope under that exact name.
#[doc(hidden)]
pub mod __private {
    pub use crate::contract::{Fixture, ProjectionFixture};
}
