---
item: HS-S0007
stage: discover
created: 2026-08-12T13:01:14.672Z
updated: 2026-08-12T13:01:14.672Z
template_sig: 86ce4036
rendered_sig: 987e7cbc
---

# Discover — projection_store_conformance! — one enumeration, one test per rule

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: an adapter author writes one line and gets one test per rule — `ProjectionFixture` in `contract.rs` (mirroring `Fixture`, reusing `Capability`/`RuleOutcome`, no borrowing GAT), the single enumeration `for_each_projection_store_rule!`, whatever emitter change Note 4 settles, `projection_store_conformance!` plus its `__private` export, an orphan meta-test, the crate-doc rule-family table, and three harness files running the baseline pair against `MemoryProjectionStore` | `../_storymap.md:59` | Eight mount points and exactly **two** rules. The rules are the smallest thing that proves the machinery; the machinery is the deliverable |
| **AC-001** — `projection_store_conformance!` exists, expands to one test per rule, its rule set is written in exactly one enumeration, and a meta-test fails if a rule in the projection rules module is absent from that enumeration | `../project.md:179-182` | The meta-test is the acceptance criterion, not the macro. "Mirrors `for_each_event_store_rule!` and `no_orphan_rules`" |
| **AC-016** — every projection rule runs and passes under the `wasm32` emitter in the same `cargo xtask ci` run, not as a separately maintained subset | `../project.md:232-234` | This story owns the harness file; `whole-gate-run-and-proof-artefact` owns the whole-rule-set verification (`../_storymap.md:94`) |
| `dependsOn: owned-batch-port-shape, projection-probe-conformance-feature, memory-projection-store` — the trait to bind, the seam to write through, and the store to run against | `../_storymap.md:56-59,109` | This is the first slice where a projection rule executes against a real store |
| The pattern being instantiated a second time, not invented: `for_each_event_store_rule!` is the single place the event-store rule set is written; `no_orphan_rules` is the meta-test AC-001 explicitly asks to be mirrored | `crates/happenstance-testkit/src/registry.rs:94,412`; `../_grounding.md:86-92` | Every design question here has a precedent in the tree, and citing it is cheaper than re-deciding it |
| **The orphan meta-test is a textual scan**: `declared_rules()` does `include_str!("suite.rs")` and filters on the literal prefix `"    pub async fn "`, defensible because rustfmt pins the shape | `crates/happenstance-testkit/src/registry.rs:345-366` | A projection suite in a *different file* needs its own scan or a generalised one. This is the concrete content of AC-001 (`../_decomposition.md:354`) — and the source of the wrong implementation below |
| Architecture brief Note 4, **the rule-running seam**: the three emitters expand to `$crate::rules::$name(__conformance_fixture)` with the module path **hard-coded**, so `projection_store_conformance!` cannot reuse them as written. Three ways out, none pre-selected: parameterise the emitters, add three more, or re-export projection rules into `rules` | `../_decomposition.md:488-509`; `crates/happenstance-testkit/src/registry.rs:228,248,276` | Option 3 is explicitly the worst: it breaks the very meta-test AC-001 is built on. The choice is a recorded decision at `spec`, and option 1 touches `#[macro_export]`ed arms, which is a public-surface change |
| Reshape trigger: if Note 4's option 1 cannot keep `local_conformance.rs`, `memory_conformance*.rs` and `fixture_instruments.rs` compiling, take option 2 and say why — "the story boundary is unchanged" | `../_storymap.md:127`; `../_decomposition.md:720-723`; `crates/happenstance-testkit/tests/` | The three existing consumers are named and present in the tree |
| Architecture brief Note 5: `Fixture::Store: EventStore` binds the event-store port in the associated type, so the projection suite needs its **own** fixture trait — reusing `Capability` and `RuleOutcome` unchanged, forbidding any borrowing GAT, and taking `impl AsyncFn() -> F` rather than a made fixture | `../_decomposition.md:517-544`; `crates/happenstance-testkit/src/contract.rs:125,97-111`; `registry.rs:45-49` | A second trait, not a widened one. The `AsyncFn` shape is what lets `commit_rejects_a_foreign_batch` open two isolated stores later |
| The fixture trait **must** be added to `__private`, or the macro expansion cannot name it in the adapter's crate | `../_decomposition.md:352`; `crates/happenstance-testkit/src/lib.rs:362-363` | A mount point that is invisible until an out-of-crate consumer tries it — which, inside this workspace, is nobody until HS-S0015 |
| The two baseline rules: `commit_advances_the_checkpoint` (PS-1, "the baseline the rest are differential against") and `commit_is_atomic_with_the_read_model` (PS-1, PS-4, PS-11, rejecting "checkpoint on one connection, rows on another") | `spec/SPECIFICATION.md:5661-5662`; `../_storymap.md:59` | Both trace to PS-1, which is `[FROZEN]` (`spec/SPECIFICATION.md:4735`) and carries the pairing defect the sweep scoped |
| The wasm32 harness is discharged by an **existing** step: `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` type-checks a new `projection_conformance_wasm.rs` sibling using `__emit_wasm` unchanged. "AC-016 costs this project a new harness file, not a new gate step" | `../_decomposition.md:786`; `xtask/src/main.rs` (wasm32 conformance-harness check); `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` | The subset risk AC-016 names "is not a risk unless someone invents one" (`../_decomposition.md:511-515`) |
| The crate doc's "What is checked" table and "Where the rule set lives" both describe a **single** rule family and must gain the projection family | `crates/happenstance-testkit/src/lib.rs:84,134-151`; `../_decomposition.md:355` | Documentation is a mount point here too: an adapter author who cannot find the family cannot invoke it |
| Testing brief AC-001: **Unit** (the orphan scan is a `#[test]` that runs and can fail) + **Static** (the macro's expansion is checked by `cargo check` compiling three harness crates). The resolving direction is free — an unregistered path is `error[E0425]` | `../_decomposition.md:771` | Only one of the two directions costs anything, and it is the one the wrong implementation below silently drops |

## Questions

Open questions to resolve before specifying.

1. **Which of Note 4's three emitter options?** Deferred to `spec`, and it must
   be a recorded decision rather than a discovery (Architecture brief AC-A01,
   `../_decomposition.md:292-297`). Option 3 is ruled out here on the grounds the
   brief gives — it breaks the orphan scan AC-001 is built on. Between options 1
   and 2 the deciding evidence is whether `local_conformance.rs`,
   `memory_conformance*.rs` and `fixture_instruments.rs` keep compiling.
2. **Does the projection orphan meta-test generalise `declared_rules()` or get
   its own copy?** Deferred to `spec`, with one requirement fixed here: whichever
   is chosen, the scanned source must be the **projection** rules module. See the
   wrong implementation.
3. **Which capability constants does `ProjectionFixture` declare?** Answered
   upstream — that is `_design.md`'s call via Architecture brief Note 5
   (`../_decomposition.md:526-534`), delivered by `projection-api-design-record`.
   This story consumes the set rather than inventing it.
4. **Do all seventeen §4.11 adapter rules land in this project?** Deferred to
   `spec` and explicitly audited by AC-014 — "whether all seventeen adapter rules
   land here, or a named subset with the remainder carrying an accurate maturity
   marker, is a scoping decision" (`../_decomposition.md:672-676`). This story
   lands two; the slices after it land the rest.
5. **None beyond these.** The six integration-level rules are
   `typed-layer-and-alpha-release`'s under CF-36
   (`spec/SPECIFICATION.md:5693-5703`).

## Decision

The projection port's invariant is stated on the trait and enforced by nothing,
because there is no way to invoke a projection rule at all: no fixture contract,
no rule enumeration, no entry point, no harness. This slice builds the machinery
— a `ProjectionFixture` trait beside `Fixture`, the single enumeration
`for_each_projection_store_rule!` beside `for_each_event_store_rule!`, the
`projection_store_conformance!` entry point and its `__private` export, the
orphan meta-test over the projection rules module, the crate-doc family table,
and three harness files (tokio, blocking, `#![cfg(target_arch = "wasm32")]`) —
and proves it end to end with the two baseline PS-1 rules running green against
`MemoryProjectionStore`. The spec will fix: the fixture trait's shape (owned
handle, no borrowing GAT, `Capability`/`RuleOutcome` reused unchanged, rules
taking `impl AsyncFn() -> F`); the emitter decision from Note 4's three options,
recorded with its reason; the orphan meta-test's scanned source and both
directions of the check; every mount point including `__private` and the crate
doc; and the two rules' text, written so they compare against positions the rule
itself supplied rather than against literals. It edits no `[FROZEN]` clause —
PS-1 is frozen and this story implements two of the three rules §4.11 assigns it,
after `projection-decision-atoms` landed the repair atom for the gap between the
clause's `MUST` and those rules.

## The wrong implementation

**A projection orphan meta-test that still scans `suite.rs`.** This is the mutant
that matters, and it is one line of copy-paste: `declared_rules()` is `const
SOURCE: &str = include_str!("suite.rs");` filtering on the literal prefix
`"    pub async fn "` (`crates/happenstance-testkit/src/registry.rs:353-355`).
Duplicate it for the projection family and forget to change the filename, and the
scan returns the *event-store* rule names — none of which appear in
`for_each_projection_store_rule!`, so a naive port of the assertion reports every
event-store rule as an orphan and fails loudly, which is fine. Change the
assertion's direction to match, or scope the scan to a module that does not exist
in that file, and it returns **the empty set**. Every check passes: the meta-test
is green, `cargo xtask ci` is green, three harnesses compile, and the projection
family's orphan check now asserts nothing at all. A rule written in the projection
rules module and never added to the enumeration is silently not run, for the whole
life of the suite. This is precisely the decorative check `CLAUDE.md`'s first
corollary forbids, and the falsifier is mechanical: add a `pub async fn` to the
projection rules module, omit it from the enumeration, and require the meta-test
to go **red** — exercised at implementation time and recorded in the story's
report, exactly as the event-store side's own scan is defended by its shape
(`registry.rs:345-351`).

**A meta-test that only checks the resolving direction.** Cheaper still, and
superficially reasonable: assert that every *enumerated* name resolves. It is
free and permanently green, because an unregistered path is already
`error[E0425]` in every harness (`../_decomposition.md:771`) — the compiler was
doing that job before the test existed. The direction that costs something is the
other one: every `pub async fn` the projection rules module declares must appear
in the enumeration. A meta-test with one direction is a test that can never fail.

**A `projection_conformance_wasm.rs` that names its rules directly.** The gate
step type-checks the harness (`cargo check -p happenstance-testkit --tests
--target wasm32-unknown-unknown`), and a harness that invokes two rules by hand
type-checks just as well as one that goes through the enumeration. AC-016 is then
false — the wasm target runs a separately maintained subset — and nothing says
so. The guard is structural rather than assertive: the wasm harness must expand
from `for_each_projection_store_rule!` like the other two, which is what makes a
subset unrepresentable. `whole-gate-run-and-proof-artefact` is where the claim is
finally read, and it can only be read if this story made the subset impossible.

**And Note 4's option 3, named so it is not chosen for its diff size:**
re-exporting the projection rules into the existing `rules` module so the
hard-coded `$crate::rules::$name` emitters work untouched. Cheapest change,
compiles immediately, and it puts projection rules into the exact file the
event-store orphan scan reads — breaking the meta-test AC-001 is built on
(`../_decomposition.md:506-509`).

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

The two judgement boxes, both with real content here. **Literal positions**: this
story adds the first two projection rules, so the box is not vacuous.
`commit_advances_the_checkpoint` and `commit_is_atomic_with_the_read_model` each
commit at a position and read the checkpoint back; both compare against the
`SequencePosition` value the rule itself handed to `commit`, never against a
literal expectation such as `FIRST` or a third position in a run. The
specification permits gaps and `GappedPositionStore` exists to convict a rule that
forgets (`crates/happenstance-testkit/tests/mutation_coverage/variants.rs:116`;
`CLAUDE.md`, *The rule that matters*), so the spec states the requirement as: a
rule must still pass when the positions it is given come from a gapped log.
**`[FROZEN]` clauses**: PS-1 is `[FROZEN]` (`spec/SPECIFICATION.md:4735`) and both
baseline rules trace to it. Its text is not edited here; the gap between its
`MUST` and `commit_advances_the_checkpoint` was scoped by `ps-clause-pairing-sweep`
and repaired by a new accepted atom in `projection-decision-atoms`, two slices
earlier (`../_storymap.md:107-108`).
