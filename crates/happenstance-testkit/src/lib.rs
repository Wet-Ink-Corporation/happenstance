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
//! **Pass `-- --show-output` to see those lines.** A skipped rule is a test
//! that *passes*, and libtest discards a passing test's stdout, so a default
//! `cargo test` reports `N passed` and no `SKIP` line whatever your fixture
//! declined. The emitted test is what makes the skip *reachable*; the flag is
//! what makes it *read*. A green run over a fixture that declines the optional
//! capabilities is not evidence that every rule ran, and
//! [`RuleOutcome::report`] says the same thing where the mechanism is.
//!
//! [`fixtures::MemoryFixture`] is the reference implementation and the one to
//! read before writing your own.
//!
//! # Choosing a harness
//!
//! `#[tokio::test]` is a default, not a requirement. The per-test wrapper is a
//! parameter — an *emitter* macro — because the testkit is in no position to
//! know which runtime an adapter is tested on, and a `cfg` ladder in here would
//! mean every new runtime needs a testkit release. The emitters that ship are
//! the rows of this table, and the rows are the count: a written-out number is
//! falsified by an edit that never touches it, and this paragraph said *three*
//! through the nine that landed after it.
//!
//! | Family | Emitter | Wrapper | Adapter needs |
//! |---|---|---|---|
//! | event store | `__emit_tokio` (default) | `#[tokio::test]` | `tokio` with `macros`, `rt` |
//! | event store | `__emit_blocking` | `#[test]` + [`block_on`] | nothing |
//! | event store | `__emit_wasm` | `#[wasm_bindgen_test]` | `wasm-bindgen-test` |
//! | projection | `__emit_projection_tokio` (default) | `#[tokio::test]` | `tokio` with `macros`, `rt` |
//! | projection | `__emit_projection_blocking` | `#[test]` + [`block_on`] | nothing |
//! | projection | `__emit_projection_wasm` | `#[wasm_bindgen_test]` | `wasm-bindgen-test` |
//! | model | `__emit_model_tokio` (default) | `#[tokio::test]` | `tokio` with `macros`, `rt` |
//! | model | `__emit_model_blocking` | `#[test]` + [`block_on`] | nothing |
//! | concurrency | `__emit_concurrency_tokio` (default) | `#[tokio::test(flavor = "multi_thread")]` | `tokio` with `macros`, `rt`, `rt-multi-thread` |
//! | concurrency | `__emit_concurrency_blocking` | `#[test]` + [`block_on`] | nothing |
//! | benchmark | `__emit_benchmark_tokio` (default) | `#[tokio::test]` | `tokio` with `macros`, `rt` |
//! | benchmark | `__emit_benchmark_blocking` | `#[test]` + [`block_on`] | nothing |
//!
//! `__emit_rule_names` is the odd one and is listed because it is reachable by
//! the same route: it wraps no test at all, expanding a rule enumeration to a
//! `[&str; N]` for a meta-test to read.
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
//! **Every name in that table carries `#[doc(hidden)]`, and you should know what
//! that costs you before you write one.** The attribute is not a judgement about
//! whether you may use them — CF-23 requires you to name one, and this crate's
//! own `happenstance-cloudflare` target does — it is the only tool the language
//! offers for *"exported because it has to be"*: `macro_rules!` lives in a flat
//! crate-root textual namespace, so a private helper is unreachable from your
//! expansion site and there is nothing to hide behind. What it does cost is
//! visibility in both directions. You will not find these on docs.rs, which is
//! why they are written out here rather than linked. And `#[doc(hidden)]` is the
//! marker `cargo-semver-checks` uses to exclude an item, so the one instrument
//! in this repository that would report a rename of `__emit_wasm` as breaking is
//! the instrument the attribute switches off.
//!
//! Whether these names are a *promise* is an open question rather than a
//! settled one, and the honest answer is that the crate has not decided: §6.6's
//! compatibility policy governs rule addition, rule meaning-change and the
//! version key, and says nothing about the emitters. Until it does, the
//! recommendation in step 1 of *Writing a projection adapter* is the one that
//! covers you — pin this crate exactly, and a rename arrives when you choose to
//! take it rather than on a minor bump you did not read.
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
//! `event_store_benchmarks!` is a fourth macro family and the only one that is
//! **not a bar**. It lives in `bench`, behind this crate's off-by-default
//! `bench` feature and absent on `wasm32-unknown-unknown`, and it measures the
//! three things phase 8 needs measured: append throughput over a batch of *n*,
//! conditional append under *k* contenders reporting the committed and rejected
//! counts separately, and replay of *N* events run both unfiltered and behind a
//! tag filter — with *n*, *k* and *N* supplied at the call site. It adds **no**
//! conformance rule, changes no adapter's bar, and can never fail a merge:
//! CF-34 says performance is measured by a separate harness which is not part
//! of the conformance bar, and this is that harness. There is no threshold in
//! it at any budget, and no clock either — CF-33 forbids one anywhere in this
//! crate's `src/`, so the harness reports counts and the *emitter* reports
//! durations, which puts `criterion`, `divan` or a CSV writer in the caller's
//! `dev-dependencies` and leaves this crate's two.
//!
//! **None of the model, concurrency or benchmark families' names is an
//! intra-doc link**, and all three paragraphs above
//! spell them plainly on purpose. Each module is absent on some configuration
//! this crate is documented under, and rustdoc treats an unresolved link as a
//! hard error: `model` is behind the off-by-default `proptest` feature, so a
//! link would break `cargo doc` with default features; `concurrency` is
//! `#[cfg(not(target_arch = "wasm32"))]`, so a link would break
//! `cargo doc --target wasm32-unknown-unknown`; `bench` is behind a feature
//! *and* a target gate, so it would break both. That is the D13 failure this
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
//! happenstance-core = { version = "0.2.0-alpha.1", features = ["unstable-projection"] }
//!
//! [features]
//! conformance = ["happenstance-core/conformance"]
//!
//! [dev-dependencies]
//! happenstance-testkit = "=0.2.0-alpha.1"
//! tokio = { version = "1", features = ["macros", "rt"] }
//! ```
//!
//! **Both requirements name the pre-release, and that is not decoration.**
//! `version = "0.2"` is the line a Rust author writes without thinking, and it
//! does not resolve: a requirement naming no pre-release never matches a
//! pre-release version, so while `0.2.0-alpha.1` is the only version on the
//! registry, `cargo add` answers that no candidate matches. An outsider taking
//! a pre-release writes the pre-release. When these crates reach a stable
//! number the requirements become ordinary carets, and
//! `xtask`'s `recipe_fence_resolves` is what makes this block move with them
//! rather than going quietly stale here.
//!
//! **The `=` on this crate is a recommendation with a reason** (CF-30). Adding
//! a conformance rule is a semver-*minor* change that can turn a passing
//! adapter's CI red, so treat it as a breaking change in practice and pin this
//! crate exactly. An exact pin is normally poor practice in a Rust library
//! because it propagates — a pinned dependency of a library constrains every
//! downstream lockfile and manufactures duplicate-version conflicts — and that
//! is exactly why the recommendation needs stating rather than assuming: Cargo
//! does not resolve a non-root package's dev-dependencies at all, so this one
//! propagates to nobody, and what it buys you is choosing *when* you take a new
//! bar instead of finding out from a red run you cannot attribute.
//!
//! **The `[dependencies]` and `[features]` halves of that block belong to the
//! port rather than to this crate**, and they are a copy of the fence on
//! `happenstance_core::ProjectionProbe`'s own page rather than a second opinion:
//! `xtask`'s `feature_cost_is_stated` fails if the two drift. A reader of step 1
//! needs one manifest and not two half-manifests on two pages, which is why the
//! copy is here at all.
//!
//! `unstable-projection` is unconditional because your `impl ProjectionStore` is:
//! every port type it names lives behind that feature, and an adapter cannot make
//! its own port impl optional. `conformance` is forwarded from a feature of *your*
//! crate instead, because the `impl ProjectionProbe` lives in `src/` under
//! `#[cfg(feature = "conformance")]` and a crate cannot `cfg` on a dependency's
//! feature.
//!
//! **An earlier version of this page said `conformance` "implies no other
//! feature — not `std`, not `memory`", and told you to turn it on in
//! `[dependencies]`.** The first half was two-thirds right and the missing third
//! is the expensive one. `conformance` costs no crate and no graph edge, and it
//! implies exactly one thing — `unstable-projection`, because `ProjectionProbe`
//! is defined *inside* the module that feature gates. That is the port PS-3 holds
//! **exempt from semver** until two adapters at opposite ends of the batch-shape
//! axis have cleared its suite, and Cargo's feature unification is global and
//! additive: a feature turned on anywhere in a graph is on for everybody in it.
//! Turning it on in `[dependencies]` therefore hands that surface to every
//! application downstream of your adapter, none of which asked for it.
//! Forwarding it costs you one line and gives the choice back to whoever builds
//! your adapter. `examples/outside-projection-adapter` in this repository writes
//! exactly this manifest, and its own `tests/` fails if it stops.
//!
//! **What the dev-dependency costs you is a bigger `happenstance-core` under
//! `cargo test` than under `cargo build`, and it is not your build that finds
//! out.** This crate depends on `happenstance-core` with `std`, `memory` and
//! `conformance` on. Cargo's resolver deliberately does not unify a
//! dev-dependency's features into `cargo build` and does unify them into
//! `cargo test`, so your `src/` compiles against a strictly larger contract crate
//! whenever the suite is in the graph. Name a `memory`- or `std`-gated item in a
//! helper — `MemoryEventStore`, say — and both of your own commands stay green
//! while the `error[E0432]` waits for whoever adds your adapter to an
//! application. Build your library the way they will, `cargo build -p
//! your-adapter` with no `--all-targets`, before you tag a release. This crate is
//! a dev-dependency and stays one.
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
//! **Every item path in every exported expansion goes through it**, and that
//! sentence used to be an aspiration rather than a fact: four of the five suite
//! macros reached past the module for `concurrency::ConcurrentFixture`,
//! `bench::BenchmarkParams`, `block_on` and each family's `rules`, which froze
//! those module paths for callers who had named nothing at all.
//! `tests/macro_expansion_paths.rs` is what makes it a fact — the only exception
//! is a *macro* name, because `macro_rules!` lives in a flat crate-root textual
//! namespace and is not reachable through a module in the first place.
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
//! # Stores that misbehave on purpose
//!
//! The suite above checks an adapter. These three check the **caller**, and
//! they exist because every store an application author can reach today
//! behaves perfectly: the reference store names the conflicting event on every
//! violation and assigns dense positions from 1, so two whole classes of caller
//! bug are not merely hard to test — the input that separates a correct caller
//! from an incorrect one never occurs in-process at all.
//!
//! | Instrument | Produces | Rejects |
//! |---|---|---|
//! | [`FaultyStore`] / [`SendFaultyStore`] | a violated append naming **no** conflicting event, and a read that fails at its first polled item | a retry loop that branches on `conflicting_position` being `Some` |
//! | `GappyMemoryStore` (`memory`) | positions with a caller-chosen stride | a read-model handler that computes its next position by adding one |
//!
//! Both wrappers come in two flavours and are two *types* rather than one, for
//! the coherence reason [`Fixture`]'s own page records. Both new stores run the
//! conformance suite themselves — the wrapper unarmed, so a disarmed fixture is
//! proved not to be lying about ordering or positions, and the gapped store
//! with its stride, so its gaps are proved to be the freedom the specification
//! grants rather than a defect.
//!
//! `GappyMemoryStore` is named in plain text here rather than linked because
//! it is gated on the `memory` feature, and an intra-doc link that resolves in
//! only some configurations is a hard rustdoc error. Its own page carries the
//! `doc_cfg` badge that says so.
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

// The same two conditions `model` carries below, and the second one is not
// tidiness: a **feature is not target-scoped**, so `--all-features` sets `bench`
// on `wasm32-unknown-unknown` too, where there are no threads and no host clock.
// The only gate step that compiles that combination is the wasm32 feature
// powerset, which is OPTIONAL and is skipped by `cargo xtask ci --fast` — so
// this is the one condition here that can reach `main` unnoticed, and it is
// copied from `model` rather than written fresh.
#[cfg(all(feature = "bench", not(target_arch = "wasm32")))]
#[cfg_attr(docsrs, doc(cfg(feature = "bench")))]
pub mod bench;
// Named `contract`, not `fixture`: one letter from the public `fixtures` module
// below, which the specification names by that exact path and which therefore
// cannot be renamed to make room.
mod contract;
// Target-gated and nothing else: the concurrency family needs no optional
// dependency, only threads. `wasm32-unknown-unknown` has none it can spawn, and
// the family is opt-in besides — an adapter that cannot race is not asked to.
#[cfg(not(target_arch = "wasm32"))]
pub mod concurrency;
mod faulty;
pub mod fixtures;
// Gated on this crate's own `memory` feature, which is in `default`. The gate is
// what makes `--no-default-features` drop the store *and* its test file
// together, so the feature powerset stays honest; being in `default` is what
// makes the item a `cargo add happenstance-testkit` user actually meets.
#[cfg(feature = "memory")]
mod gappy;
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
pub use faulty::{FaultyStore, FaultyStoreError, SendFaultyStore};
#[cfg(feature = "memory")]
#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]
pub use gappy::GappyMemoryStore;
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
/// deprecated arm: `factory =` was introduced at `23fd446` and removed at
/// `1c1a6b7`, eight days before this crate was first published at
/// `0.2.0-alpha.1` on 2026-08-16, so no published version ever accepted it.
/// Change the keyword and hand it a [`Fixture`] instead of a store.
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
///
/// **The complete inventory, not two thirds of one.** It carried `Fixture` and
/// `ProjectionFixture` while four fixture-shaped types existed, and the four
/// suite macros outside those two reached around it —
/// `$crate::concurrency::ConcurrentFixture`, `$crate::bench::BenchmarkParams`,
/// `$crate::block_on` and each family's `rules` module. Twenty-two paths in all,
/// every one of them a module layout frozen by a caller who wrote one line and
/// named nothing. `tests/macro_expansion_paths.rs` is what keeps the inventory
/// complete; widening it is additive and is what makes those relocations cheap
/// later rather than a major break of this crate.
///
/// Emitter and enumeration *macros* are deliberately absent, and the absence is
/// mechanical rather than a judgement: `macro_rules!` lives in a flat crate-root
/// textual namespace, so `$crate::__private::__emit_tokio` does not exist and
/// cannot be made to. What those names promise is C2-03's open question.
///
/// Each `cfg` here is copied from the module it re-exports rather than written
/// fresh: `concurrency` is absent on `wasm32-unknown-unknown`, `bench` is behind
/// a feature *and* that target gate, and `model` is behind an optional
/// dependency that does not build for wasm32. A re-export without the matching
/// gate is an `error[E0432]` in exactly the configuration the two-flavour design
/// exists for.
#[doc(hidden)]
pub mod __private {
    pub use crate::contract::{Fixture, ProjectionFixture};
    pub use crate::registry::block_on;
    // The probe is `happenstance-core`'s, not this crate's, and it is here for
    // the same reason as everything else: `require_read_through!` named it bare,
    // so it resolved against `projection.rs`'s own `use` rather than against the
    // crate the expansion lands in.
    pub use crate::suite::rules;
    pub use happenstance_core::ProjectionProbe;

    #[cfg(all(feature = "bench", not(target_arch = "wasm32")))]
    pub use crate::bench::{BenchmarkParams, scenarios as benchmark_scenarios};
    #[cfg(not(target_arch = "wasm32"))]
    pub use crate::concurrency::{ConcurrentFixture, rules as concurrency_rules};
    #[cfg(all(feature = "proptest", not(target_arch = "wasm32")))]
    pub use crate::model::rules as model_rules;
    pub use crate::projection::rules as projection_rules;
}
