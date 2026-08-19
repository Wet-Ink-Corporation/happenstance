---
item: HS-S0078
stage: discover
created: 2026-08-12T13:02:47.949Z
updated: 2026-08-12T13:02:47.949Z
template_sig: 86ce4036
rendered_sig: b56003cb
---

# Discover — `LadybugFixture` and the projection conformance run, registered as a proof artefact

## Signal Ledger

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line — write `LadybugFixture` (a temp directory per instance), invoke `projection_store_conformance!` from a new `tests/` target, extend `port_shape.rs`, and register the target in `xtask/src/proof.rs`'s `ARTEFACTS` so an emptied file cannot pass the gate | `_storymap.md:44` | The fixture, the macro invocation and the registry row are deliberately one story; splitting them is the split M4 forbids. |
| **AC-004** — the suite runs against a Ladybug fixture and **every rule reports an outcome**: pass, or a declined capability printing the fixture's stated reason; no rule absent, and the count emitted equals the count `projection-store-freeze` registered | `project.md:193-196` | The count equality is a diff against HS-P0010's own registration list, not a guess (`_decomposition.md:474`). |
| **M1** — the target's whole body is the macro invocation over a `LadybugFixture`, exactly as CLAUDE.md's rule that matters spells it. **Not a hand-rolled test module, and not a subset** | `_decomposition.md:127-142`; `CLAUDE.md`, *The rule that matters* | An adapter that compiles but has not run the suite is not an adapter, and a suite invoked in part has not been invoked. |
| The fixture contract — one fixture instance is one isolated backing store, each `connect()` is one handle onto it, two fixture instances share nothing; every rule takes `impl AsyncFn() -> F` precisely so that **isolation itself is a conformance rule** rather than a testkit meta-test | `crates/happenstance-testkit/src/contract.rs:14-24` | Pointing every instance at one temporary directory is named in the testkit's own docs as *an adapter's mistake*. `crates/happenstance-testkit/src/fixtures.rs:243-271`'s `MemoryFixture` is the reference to read first. |
| **M2 / DR-5** — every rule the fixture cannot satisfy declines through `Capability::declined("…")` with a real reason and still appears in the run as a reported skip; **never `#[cfg]`** | `_decomposition.md:144-153`; `project.md:154-157`; `crates/happenstance-testkit/src/contract.rs:25-63` | Consumes `.kb/open-questions/cf-40-fixture-limits-ownership.md`; it does not re-decide where fixture constants live. |
| Two mechanical traps before the first red run: an empty reason on an *associated* const is caught at **codegen**, so `cargo clippy` passes and `cargo build`/`cargo test` fail; and the `SKIP` line is only visible because the gate passes `--show-output` | `_decomposition.md:149-152`; `crates/happenstance-testkit/src/contract.rs:412-415`; `xtask/src/main.rs:151` | A declension with an empty reason is not a lint failure, it is a build failure — and a run without `--show-output` reports nothing a verdict can read. |
| **M3** — `tests/port_shape.rs` already instantiates generic code at both batch shapes through a `const _` block, with `weak_flavour` and `send_flavour` in separate modules because both trait names in scope makes every method call `error[E0034]`. **Extend it, do not replace it** | `_decomposition.md:155-164`; `crates/happenstance-ladybug/tests/port_shape.rs:75-80`; `CLAUDE.md` constraint 4 | The higher-ranked `for<'a> S::Batch<'a>: Send` at `:61` collapses to `S::Batch: Send` once the lifetime is gone, and that collapse is PS-5's predicted ergonomic win observed — a verdict finding, not a tidy-up. |
| **M4** — phase 11's proof artefact is *"the projection suite green on a third shape"*, and **nothing else in the gate would notice if the test target ceased to exist**: `cargo test` exits 0 on `running 0 tests`, so an emptied file passes a step a deleted one fails | `_decomposition.md:166-171`; `xtask/src/main.rs:170-178`; `RUNBOOK.md:4427-4430` | The gate runs `cargo xtask proof-artefact` rather than `cargo test` for exactly this reason; the `ARTEFACTS` row (`xtask/src/proof.rs:133`) names package, target and the test names the verdict will cite. |
| **DR-9** — the run must be executable by someone who did not write this adapter: fixture, command and expected output recorded, because *"passed against N adapters" is a snapshot and the snapshot has to be re-takeable* | `project.md:172-174` | The `ARTEFACTS` row is half of this; the recorded command and output is the other half and it feeds the verdict. |
| Risk — the suite may accommodate the shape it was written against; this project is the first consumer of HS-P0010's suite from outside that project, and a suite whose rules were written while looking only at SQL-shaped batches **can pass a graph batch by not asking the question** | `project.md:263-268` | The mitigation is AC-001's axes fixed in writing first, plus AC-005 forcing a behaviour the deferred write set can plausibly fail. This story is where the accommodation would be invisible. |
| `depends_on: fill-the-bodies-and-ps-34-disposition` — supplies four real bodies against the real driver, with `#![allow(clippy::todo)]` gone | `_storymap.md:44`, `:60-66` | There is nothing to drive before them; a suite run against `todo!()` bodies panics rather than reports. |
| Slice rationale — the fixture, the macro invocation and the `ARTEFACTS` row are one story because the test target *is* the mount and the registry row is what makes it noticeable | `_storymap.md:71-74` | Splitting "write the fixture" from "wire it into the proof registry" produces a capability behind a stub. |

## Questions

**Does `projection_store_conformance!` exist, and what does it take? — answered by
the preflight, not assumed here.** `preflight-and-unlike-axes` (HS-S0074)
established whether it resolves from `happenstance-testkit` and what its argument
is; if it did not, the project halted there
(`_decomposition.md:33-41`). It mirrors `event_store_conformance!`
(`crates/happenstance-testkit/src/lib.rs:311-357`), which builds a **`Fixture`**,
not a store.

**What are `SECOND_HANDLE` and `REOPEN` for a LadybugDB fixture? — deferred to
`spec`, but the *policy* is fixed and not re-openable.** Every declension carries
a real, fixture-specific reason; nothing is `#[cfg]`-ed out; and the most likely
genuine declension is single-writer concurrency — LadybugDB permits many readers
and exactly one writer, so a concurrency-shaped rule will meet
`WriteTransactionInUse`
(`crates/happenstance-ladybug/src/projection_store.rs:204-213`). Whether that is a
stated capability limit or a rule defect is **the verdict's question**, and the
answer is never a retry loop (`project.md:278-283`). What this story owes is that
the behaviour is reported with its reason rather than smoothed away.

**How does `lbug`'s blocking API meet a non-blocking port (ADR-0025)? — answered
upstream; this story's stake is that it must not add a runtime requirement the
harness does not have.** The suite is emitted through the existing registry and
inherits the tokio, blocking and wasm emitters unchanged
(`_decomposition.md:115-119`; `crates/happenstance-testkit/src/registry.rs:93-103`),
and the `__emit_blocking` emitter needs no async runtime at all
(`crates/happenstance-testkit/src/lib.rs:63-67`). A fixture that only works under
a multi-threaded tokio runtime has quietly narrowed the harness, and that is a
finding rather than a configuration detail.

**What counts as "structurally unlike"? — on disk already, and this story is what
makes it *observable*.** The axes were committed by HS-S0074 before any body was
filled in. Here they stop being a document and become a run: the emitted rule
list, the declension list with reasons, and the recorded command and output are
what the verdict reads them against. If every rule passes without any of them
exercising a named axis, that is a **finding about the suite**, recorded and
raised with `projection-store-freeze` — not a licence to add a local rule that
makes the run look more thorough.

**What if a suite rule seems wrong? — answered, with the boundary stated because
it is unusual here.** `crates/happenstance-testkit/**` is explicitly not this
project's to edit (`_decomposition.md:86-88`), so "fix the rule in the same
change" takes the only form available: the rule is never `#[cfg]`-ed out, never
routed around with a retry, and never made to pass by weakening the fixture — the
defect is raised with HS-P0010 and the reason is written down in the same change
that observes it, so the observation and its explanation never travel separately.

**Deferred to `spec`:** the temp-directory strategy and how a `LadybugDB` directory
is created and torn down per fixture instance
(`_decomposition.md:417-419`), and whether the `ARTEFACTS` row names individual
conformance test names or the target alone — which depends on what the macro emits.

## Decision

`happenstance-ladybug` will, by the time this story opens, be a crate that
compiles against a real graph engine and has never been asked a single question by
the conformance suite — which is precisely the state CLAUDE.md refuses to call an
adapter, and precisely the state in which the freeze re-test has produced no
evidence at all. This story turns it into a run: a `LadybugFixture` giving each
instance its own isolated LadybugDB directory, a new `tests/` target whose entire
body is the `projection_store_conformance!` invocation over that fixture, an
extension of `port_shape.rs`'s existing `const _` instantiations rather than a
replacement, and a row in `xtask/src/proof.rs`'s `ARTEFACTS` so that an emptied
target fails the gate that an intact one passes. The spec will cover the fixture
against the testkit's isolation contract, the capability constants with a stated
reason on every declension, the macro invocation as the whole target with no
hand-rolled subset, the rule-count reconciliation against HS-P0010's registration
list, the `--show-output` requirement that makes `SKIP` lines readable, and the
recorded command and output that make the run re-takeable by a stranger. Nothing
`[FROZEN]` is touched — PS-2 (`spec/SPECIFICATION.md:4760-4774`) is the clause this
run is evidence *for*, and if a rule appears wrong the response is a finding
raised with `projection-store-freeze` plus a written reason, never an edit to a
clause or a local weakening of the suite.

## The wrong implementation

**The "graph" fixture whose batch is a SQL batch in different type names.** A
`LadybugFixture` whose read model is one node label with scalar properties —
every write a `MERGE (r:Row {k: $k}) SET r.v = $v`, no relationships, no
traversal, no second label. It is a key-value row store with a Cypher accent. It
passes every projection rule, emits the full registered rule count, satisfies
AC-004 exactly as written, and re-tests the freeze **against the shape that froze
it** — so the verdict written on that run is worthless, which is the single
failure this whole project exists to prevent. It is also the easy thing to build:
a row-shaped read model is what a projection example looks like everywhere else in
this workspace.

Its falsifier is a negative control, and its placement is forced. It cannot go in
the testkit — `crates/happenstance-testkit/**` is not this project's to edit
(`_decomposition.md:86-88`), and an adapter-specific mutant does not belong in a
suite four other adapters run. It lives in
`crates/happenstance-ladybug/tests/`, alongside the conformance target: a
`RowShapedFixture` whose store accepts only single-label scalar writes, run
against the same suite, with the outcome **recorded rather than asserted green**.
Two outcomes and both are informative. If it passes the whole suite, the run
proves that the registered rules do not distinguish a graph batch from a row batch
— which is a first-class verdict finding ("the suite as registered is blind to the
axis this project was run to test") and goes to HS-P0010, not a reason to add a
local rule. If some rule rejects it, that rule is named in the verdict as the one
that actually bit on the unlike axis, which is what makes AC-004's green
meaningful instead of decorative. Either way the control must be written *before*
the real run is interpreted, for the same reason the axes were.

**The fixture that hands every instance the same directory.** One
`TempDir` created at module scope, every `LadybugFixture::new()` opening the same
path, every `connect()` returning a handle onto it. It looks tidy, it is fast, and
every rule that writes and reads back passes. It is named in the testkit's own
module docs as *an adapter's mistake*, and the rule that catches it —
`two_fixture_instances_observe_none_of_each_others_appends`
(`crates/happenstance-testkit/src/suite.rs:210`) — is inherited, not written here,
which is exactly why isolation is a conformance rule rather than a testkit
meta-test (`crates/happenstance-testkit/src/contract.rs:17-24`). Worth naming
because the LadybugDB variant is more tempting than the SQLite one: a per-instance
directory costs a real filesystem create and a C++ engine open, and sharing one is
the obvious optimisation when the suite starts feeling slow.

**The invocation trimmed to what passes.** The target's body written as a list of
individual rule calls rather than the macro — or the macro invoked with rules
excluded — so that a rule meeting `WriteTransactionInUse` disappears instead of
declining. `cargo test` is green, the file is not empty so M4's `ARTEFACTS` row is
satisfied by the target existing, and the run's rule count silently drops below
HS-P0010's registration count. The guards are two and both already exist: M1's
"not a hand-rolled test module, and not a subset", and AC-004's count equality
check against the registration list — which is the only reason a missing rule is
detectable at all, since an absent rule leaves no trace in the output.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
