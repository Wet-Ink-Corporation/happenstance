---
item: HS-S0070
stage: spec
created: 2026-08-12T13:47:08.757Z
updated: 2026-08-12T13:47:08.757Z
template_sig: 87bbf1d0
rendered_sig: ae938055
---

# Spec — conflicting_position settled by evidence, and the ledger corrected

## Scope lock

| What | Path |
| --- | --- |
| Initiative | `.bklg/from-contract-to-published-library/initiative.md` — **DoD 6** (`:375-376`), the scenario this story closes the evidence half of |
| Project | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` — **AC-008** (`:253-257`), DR-4, DR-5, DR-7, DR-9; risk-table row 4 ("the intake brief's `conflicting_position` premise is already contradicted in-tree") |
| This spec | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/neon-conflicting-position-verdict/spec.md` |
| Key briefs | `.../postgres-and-neon-stores/_decomposition.md` — *Architecture brief* §5 (Neon's contracts, and "do not delete it, do not fix it"), §9.1 (the rule-function seam), §7 (gate/CI); *Testing brief* Notes §4 (the negative test), §6 (whole-invocation gating), §7 (the one instrument this project consumes rather than authors) |
| Signed-off design | `.../postgres-and-neon-stores/_design.md` — **no user-facing surface**, approved 2026-08-12. This story renders none and declares no public item. |
| Story map row | `.../postgres-and-neon-stores/_storymap.md` — *Slices*, `neon-conflicting-position-verdict`; *Coverage*, the AC-008 row (sole owner); *Merge order* item 4 |
| Discovery | `.../neon-conflicting-position-verdict/discover.md` — the signal ledger, the four answered questions, the three deferred ones this spec settles, and the named wrong implementation |
| Roadmap pointer | `RUNBOOK.md:479` and `RUNBOOK.md:3113-3116` — the two sentences the evidence contradicts, and the exact targets of the ledger correction |

## One-line PR slice

The in-tree CTE's claim that the collapse *keeps* `conflicting_position` is
confirmed or refuted against a real Neon endpoint, `ProbeThenWriteStore` is
shown to **fail** a named rule the real store passes, and the decision ledger's
standing assumption is corrected either way.

## Executive summary

`neon-append-and-read-over-http` (HS-S0069) lands the CTE and turns
`event_store_conformance!` green against a live `/sql` endpoint. That run is
**single-client**, and a single-client run cannot tell a one-snapshot CTE from
two lucky statements: `ProbeThenWriteStore` returns a conflicting position in
exactly the scenario a sequential rule creates, because its probe runs first,
finds the conflict and reports it
(`crates/happenstance-neon/src/event_store.rs:428-503`). This PR is the delta
that makes the green run mean something.

Three things land, and they are one argument:

1. **The four `todo!()` bodies `ProbeThenWriteStore` needs** — `probe_request`
   (`event_store.rs:121-123`), `insert_request` (`:126-128`),
   `decode_probe_response` (`:494-498`), `decode_last_position` (`:501-503`).
   HS-S0069 owns the other six; these four exist only for the wrong
   implementation and are unowned anywhere else, which `deskeleton-and-package-readiness`
   (HS-S0072) would discover far too late.
2. **A deterministic contention harness and the negative control.** The same
   public rule functions, driven from `crates/happenstance-neon/tests/`, over a
   fixture that closes the probe→insert window with a real second writer.
   `ProbeThenWriteStore` must fail a rule **by name**; `NeonEventStore` under the
   identical harness must pass it, because one round trip has no window to close.
3. **The ledger correction, at two named line targets.** `RUNBOOK.md:479` still
   reads "open — Neon-over-HTTP is the forcing case, because it has no
   interactive transaction and so cannot probe and write separately", and
   `RUNBOOK.md:3113-3116` still reads "the only shape it can express yields a
   boolean and no row". Whichever way the endpoint lands, neither sentence may
   still read as it does now.

**No `[FROZEN]` clause changes and no ADR is authored here.** ES-25 already
settles the port-level question — `conflicting_position` is informational, an
adapter MUST be permitted to report `None`, callers MUST NOT depend on it
(`spec/SPECIFICATION.md:3746-3751`, marked FROZEN at `:8607`) — so both outcomes
are inside the contract as written. The one branch that owes a decision record
is a *different* finding, and it is bounded in AC-007.

## Context pack

The load-bearing decisions this story must honour. Everything deeper is a
signposted anchor; do not go looking for it until an AC sends you.

**The question is not whether `conflicting_position` is a promise. That is
settled, and settled as a hint.** ES-25 is `[FROZEN]`: the field is
informational, an adapter that detects a conflict without learning which event
caused it MUST be permitted to report `None`, and callers MUST NOT depend on it
(`spec/SPECIFICATION.md:3746-3751`). ADR-0012 settled it and explicitly rejected
demoting it further, "because a compiled Postgres strategy keeps the field
usefully; `Option` already spares the adapters that cannot afford it"
(`.kb/decisions/0012-append-shape-and-preconditions.md:26`, `:99-101`). This
story does **not** reopen that. It is written down here because the story's own
title invites the opposite reading, and reopening it would be a re-plan.

**What is open is the ledger's *premise*.** Two sentences in the plan of record
justify the hint by asserting a capability limit that the crate's own
documentation contradicts. `RUNBOOK.md:479`: "open — Neon-over-HTTP is the
forcing case, because it has no interactive transaction and so cannot probe and
write separately." `RUNBOOK.md:3113-3116`: `happenstance-neon` "is cited for why
the question was asked: it has no interactive transaction, so the only shape it
can express yields a boolean and no row." The crate says the opposite, in terms,
and quotes the counterexample — "contrary to the standing assumption in the
decision ledger, the collapse **keeps** `ConditionViolated::conflicting_position`"
(`crates/happenstance-neon/src/lib.rs:46-77`). One of the two is wrong and
**neither has met a real endpoint**. That is the whole of this story.

**A confirmation taken with one client is worthless, and it is the wrong
implementation this story exists to reject.** Append A; append B with an `after`
that A invalidated; observe a conflicting position; record AC-008 as confirmed.
Everything is green and nothing was learned, because `ProbeThenWriteStore`
returns the conflicting position in exactly that scenario too. The property
under test is not "does a position come back" — it is "were the probe and the
insert evaluated on **one snapshot**", and a test with no second writer cannot
observe the difference (`discover.md`, *The wrong implementation*).

**So the instrument is the negative control, and it must fail by name.** Drive
the existing public rule functions —
`happenstance_testkit::rules::<name>(open: impl AsyncFn() -> F) -> RuleOutcome`,
each panicking on non-conformance (`crates/happenstance-testkit/src/lib.rs:189`,
`src/suite.rs:89`, e.g. `condition_rejection_is_reported_as_condition_violated`
at `suite.rs:4949`) — from `crates/happenstance-neon/tests/`, over a fixture
whose store is `ProbeThenWriteStore`. Assert the failure with `catch_unwind`
over a **non-capturing** probe and match the rule that panicked: `Probe::run` is
a function pointer precisely so no `AssertUnwindSafe` appears anywhere, and the
first refactor anyone attempts — hoisting a fixture out and capturing it — is
what breaks that (`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:325-340`,
`:450-458`). A transport error, a decode panic or a serialization abort recorded
as a visibility finding is the weak control that reaches the right verdict for
the wrong reason.

**The window is closed deterministically, not raced.** "Real contention" here
means a real second writer against the same live branch, landing its append
inside the probe→insert gap — not a sleep, not a thread pile-up, and not a
retry-until-flaky loop. The seam that makes it deterministic already exists:
`SqlTransport` is a one-method trait, so an interposing transport wrapping the
real one can perform the conflicting append through a second client between the
probe's response and the insert's request. Against `NeonEventStore` the same
interposition has nowhere to land — there is exactly one round trip — so the
conflicting event is committed *before* the CTE's snapshot and the CTE must
reject. Two stores, one harness, opposite verdicts: that is the evidence.
Nothing in this harness reads a clock (CF-33's spirit; this story adds no
conformance rule, so the lint does not sweep it, and the constraint is held by
discipline).

**The most plausible way the endpoint refutes the CTE is the isolation header,
and it is already visible in the tree.** `NeonConfig` defaults to
`IsolationLevel::Serializable` rather than Postgres' `ReadCommitted`, because
the crate says the CTE "needs `IsolationLevel::Serializable` to be sound"
(`crates/happenstance-neon/src/lib.rs:74-77`,
`crates/happenstance-neon/src/config.rs`). But the header "applies only when
[`SqlRequest::statements`] holds more than one statement — a single statement is
its own implicit transaction and the header is ignored"
(`crates/happenstance-neon/src/transport.rs:56-60`). The CTE is **one
statement**. So the shipped configuration may be handing the CTE
`ReadCommitted`, at which the crate's own `IsolationLevel` doc says "a
conditional insert can miss a conflict committed by a concurrent transaction
after this one took its snapshot" (`transport.rs:64-66`). Answer that against
the endpoint, do not reason about it: whether the header reaches a
single-statement request, and if it does not, whether the append is re-shaped
(a batch the header does apply to) or the unsoundness is the finding. Under
`Serializable` the price is SQLSTATE `40001` aborts, surfaced as
`NeonSqlError::is_serialization_failure` — a retryable abort is a legitimate
outcome and must not be laundered into a pass.

**`ProbeThenWriteStore` is consumed, never rebuilt and never fixed.** It is the
named wrong implementation the whole capability argument rests on, and the
architecture brief is explicit: "Do not delete it, do not fix it, and do not
leave it merely documented" (`_decomposition.md` *Architecture brief* §5;
*Testing brief* Notes §7 lists it as the one fixture-adjacent instrument this
project consumes rather than authors). Its defect is a **lost update** — silent,
error-free, indistinguishable after the fact from a legitimate append
(`crates/happenstance-neon/src/event_store.rs:400-415`). Making it pass is
shipping the wrong thing.

**If no rule in the set rejects it, that is a finding about the rule set — not a
licence to proceed.** The rule is fixed and the reason given in the same change,
with a `CHANGELOG.md` entry naming the defect it now detects (CF-29; the lint
sweeps all three rule files, `xtask/src/lints.rs:525`,
`xtask/src/spec_trace.rs:85-89`). The rule's **name** may not change: `spec-trace`
check 6 requires every rule in `RULE_FILES` to be claimed by a clause, so a
rename is a specification edit this story is not taking.

**How the ledger may be corrected, and how it may not.** An accepted decision
atom is immutable — `redkiln validate --kb` checks each against `HEAD` — so
editing ADR-0012's body is unavailable. Superseding it is available and is
**wrong**: its conclusion is correct and unchanged either way, and marking a
correct, `[FROZEN]`-backed decision as superseded because one supporting
sentence was factually wrong leaves the phase-12 audit reading a supersession
chain to re-derive whether the outcome moved — exactly the work the ledger
exists to spare it (`discover.md`, *The ledger mutant*). The correction belongs
where the wrong sentences are: `RUNBOOK.md:479`, `RUNBOOK.md:3113-3116`, and
`references/adapter-shapes.md:116-125`, which repeats the claim as an
unverified "unlooked-for result". The crate doc that made the claim
(`crates/happenstance-neon/src/lib.rs:46-77`) gains the verdict and the
isolation finding, so a caller meets the evidence and not the conjecture.

**And the quietest wrong move: correcting the crate doc and stopping there.**
Every check passes; nothing in the gate reads `RUNBOOK.md:479`. AC-008's second
half — "the decision ledger's standing assumption is corrected either way" — is
the only instrument, which is why the two line targets are named in this spec
rather than left as the unbounded noun "the ledger".

**No ADR is authored here, and the one branch that owes a record is bounded.**
If the endpoint shows the CTE's *append semantics* unsound — not a missing field
but a condition that fails to reject — that is a rule Neon cannot pass, DR-7
applies, and the deliverable is a decision record **staged in `.kb/_intake/`**
with the evidence, written before any behaviour change, reaching
`.kb/decisions/` only through `/redkiln:kb-ingest` (Root D; hand-writing atoms
is what commit `0269720` reverted). A missing-`conflicting_position` outcome
owes no record at all: ES-25 already permits `None`, and the ledger correction
simply runs the other way.

**The persona slice.** The reader served is the adapter author deciding whether
this workspace's compliance claim is worth trusting, and the
`publication-and-positioning` (HS-P0016) auditor reading the clause ledger later.
What they meet is not a screen: it is a live-job log in which a named rule fails
against a store that type-checks perfectly and passes against the shipped one,
and a plan of record whose sentence about Neon is true.

## Integration contract

- **Archetype**: `capability` — the deliverable is a run that reports and a
  verdict a future reader is bound by, which is what "user-observable" means in
  this medium (`_storymap.md`, preamble).
- **Slice / milestone**: `neon-live-suite`. Slice-mates, implemented in one
  context and mounted as one integrated surface: `neon-fixture-and-live-job`
  (HS-S0068) and `neon-append-and-read-over-http` (HS-S0069). This story is
  sequenced last within the slice (`_storymap.md`, *Merge order* item 4).
- **Mount point**: `crates/happenstance-neon/tests/conflicting_position.rs` — a
  new integration-test target in the adapter's own `tests/` directory (the crate
  has none today), driving `happenstance_testkit::rules::*` directly against the
  live endpoint, and swept up by the credentialed Neon CI job with **no workflow
  edit** (`cargo test -p happenstance-neon --all-features -- --ignored
  --show-output`, `_decomposition.md` *Testing brief* Notes §6). Secondary
  mounts, both consumed-and-extended rather than authored:
  `crates/happenstance-neon/src/event_store.rs` (the four `ProbeThenWriteStore`
  bodies) and `crates/happenstance-neon/src/lib.rs:46-77` (the crate doc that
  states the claim and must state the verdict).
- **Wires into**:
  - `happenstance_testkit::rules` — the public rule functions
    (`crates/happenstance-testkit/src/lib.rs:189`, `src/suite.rs:89`), returning
    `RuleOutcome` and panicking on failure. Composed, never added to.
  - `happenstance_testkit::{Fixture, RuleOutcome, Capability}`
    (`crates/happenstance-testkit/src/contract.rs`) — the contended fixture is a
    thin wrapper over the `NeonFixture` HS-S0068 authors, inheriting its
    `SECOND_HANDLE` answer (a MUST — a decline makes
    `two_handles_observe_each_others_appends` panic quoting the fixture's own
    words) and its `MAX_RESPONSE_BYTES`-anchored ceilings.
  - `ProbeThenWriteStore` and `NeonEventStore`
    (`crates/happenstance-neon/src/event_store.rs:388-503`, `:168-230`) — the two
    arms of the comparison, one consumed intact, one already wired by HS-S0069.
  - `SqlTransport` and the real transport HS-S0067 (`neon-sql-transport`) ships
    (`crates/happenstance-neon/src/transport.rs`) — the interposing transport
    wraps it; `NullTransport` fails every round trip by design and is not a
    substrate for anything here.
  - The `catch_unwind`/non-capturing-probe pattern
    (`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:325-340`,
    `:450-458`) — the in-tree shape for asserting that a rule *rejects* an
    implementation, reused; the same seam `postgres-rule-controls` (HS-S0064)
    establishes on the Postgres side.
  - The credentialed Neon CI job in `.github/workflows/ci.yml`, authored by
    HS-S0068 — consumed, not changed, unless the gating mechanism it chose is
    not `--ignored`.
- **Renders surfaces**: **none.** `_design.md` records no user-facing surface for
  this project, approved at the `/redkiln:plan` design gate.
- **Public items**: none. `_design.md`'s `## Items` block is `N/A`. Everything
  added is a `#[test]`, `pub(crate)`, or a test-target-local type; the four
  `todo!()` bodies replaced are already-declared private methods and private free
  functions. No item joins `happenstance-neon`'s or `happenstance-testkit`'s
  public API, and none may — an item that became `pub` here is a semver promise
  nobody made.
- **Conformance rule(s)**: **no rule is added, renamed or removed.** Existing
  rules are *composed* from the adapter side; the rule this story asserts must
  reject `ProbeThenWriteStore` is named in the ledger by the run itself, with
  `condition_rejection_is_reported_as_condition_violated`
  (`crates/happenstance-testkit/src/suite.rs:4949`) as the expected candidate
  under the contention harness. If none rejects it, a rule changes — schedule and
  body, never name — with its reason and a `CHANGELOG.md` entry in the same
  change (AC-004).
- **Clause(s)**: supplies adapter-level evidence under **ES-25**
  (`spec/SPECIFICATION.md:3746-3751`, FROZEN at `:8607`) and contributes the
  Neon half of the CF-25 residual-exposure discharge `RUNBOOK.md:606` schedules
  for this phase. **No clause text changes.** ES-25 already permits `None`, so
  neither outcome needs an amendment; if append *semantics* prove unsound, the
  clauses at risk are the condition-semantics ones and AC-007's decision-record
  path applies — a `[FROZEN]` clause changes by ADR, never by edit.
- **Advances DoD scenario**: initiative **DoD 6** — "a store with no connection,
  no interactive transaction and no cursor passes the suite, or the contract is
  amended by decision record and the suite re-run"
  (`.bklg/from-contract-to-published-library/initiative.md:375-376`). HS-S0069
  makes the suite *run*; this story is what makes "passes" a claim about one
  snapshot rather than a coincidence of sequential testing, and it hands
  `far-end-discharge-record` (HS-S0073) the ES-25 line of the audit record.

## PR boundary

**In this PR**

- The four remaining `ProbeThenWriteStore` bodies in
  `crates/happenstance-neon/src/event_store.rs`: `probe_request` (`:121-123`),
  `insert_request` (`:126-128`), `decode_probe_response` (`:494-498`),
  `decode_last_position` (`:501-503`).
- The interposing `SqlTransport` and the contended fixture that close the
  probe→insert window with a real second writer against the same live branch.
- `crates/happenstance-neon/tests/conflicting_position.rs`: the run that drives
  `happenstance_testkit::rules::*` against both arms, asserts the named rule
  **fails** for `ProbeThenWriteStore` via `catch_unwind` over a non-capturing
  probe, and asserts it passes for `NeonEventStore` under the identical harness.
- The isolation-header answer: whether `Neon-Batch-Isolation-Level` reaches a
  single-statement request, and — only if it does not — the minimal re-shaping of
  the append request that makes the CTE sound, or the recorded refutation.
- The ledger correction at its exact targets: `RUNBOOK.md:479`,
  `RUNBOOK.md:3113-3116`, `references/adapter-shapes.md:116-125`, and the crate
  doc at `crates/happenstance-neon/src/lib.rs:46-77`.
- If and only if no rule rejects `ProbeThenWriteStore`: the rule change in
  `crates/happenstance-testkit/src/suite.rs` with its reason in the same change
  and a `CHANGELOG.md` entry naming the defect.
- If and only if the CTE's append semantics prove unsound: the decision record
  staged in `.kb/_intake/`, written before any behaviour change.

**Explicitly not in this PR**

- The six `todo!()` bodies on the shipped path — `read_request`,
  `conditional_append_request`, `decode_append_response`, `decode_read_response`,
  `head`, `contains_event_id`. All `neon-append-and-read-over-http` (HS-S0069)'s,
  consumed here.
- `NeonFixture`, the capability declarations, the numeric ceilings, and the
  credentialed Neon CI job — `neon-fixture-and-live-job` (HS-S0068)'s.
- The real `SqlTransport` and its ship-versus-dev-dependency, `cargo deny` and
  `wasm32` answers — `neon-sql-transport` (HS-S0067)'s
  (`_decomposition.md` *Architecture brief* §9.2).
- Any new conformance rule, any rule **rename**, any edit to
  `spec/SPECIFICATION.md`, and any `[FROZEN]` clause.
- Superseding, editing or re-litigating ADR-0012, and reopening whether
  `conflicting_position` is a promise. ES-25 is frozen and the atom's conclusion
  stands.
- Authoring an ADR atom by hand under `.kb/decisions/`, or writing ADR-0024.
  Position visibility is a separate property and is
  `adr-0024-position-visibility-mechanism` (HS-S0065)'s.
- Removing `publish = false`, the scoped `#![allow(clippy::todo)]`
  (`crates/happenstance-neon/src/lib.rs:101-105`), or any package-surface change
  — `deskeleton-and-package-readiness` (HS-S0072)'s, which asserts no `todo!()`
  survives and therefore depends on this story having landed its four.
- "Fixing" `ProbeThenWriteStore`, deleting it, or leaving it merely documented.
- Running the concurrency family against Neon. Its bound is
  `F::Store: EventStore + Send`, Neon implements the bare flavour only, and its
  absence here is by design (`_decomposition.md` *Architecture brief* §2).

**Merge DoD**: the CTE's behaviour against a real endpoint is on record with the
conflicting position it returned (or the refutation it forced);
`ProbeThenWriteStore` is on record as failing a named rule the shipped store
passes, from the same harness in the same run; neither `RUNBOOK.md:479` nor
`RUNBOOK.md:3113-3116` still reads as it does today; `cargo xtask ci --fast` is
green on a machine with no Docker and no credentials, and the live Neon job is
green on the same tree.

The implementer MAY touch the wiring files named in the Integration contract to
mount this slice; that is not scope drift.

```
crates/happenstance-neon/src/**
crates/happenstance-neon/tests/**
crates/happenstance-neon/Cargo.toml
crates/happenstance-testkit/src/suite.rs
CHANGELOG.md
RUNBOOK.md
references/adapter-shapes.md
.kb/_intake/**
.bklg/from-contract-to-published-library/postgres-and-neon-stores/neon-conflicting-position-verdict/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The two-statement path becomes executable | `probe_request` compiles an `AppendCondition` into a `SELECT min(position)` conflict probe; `insert_request` compiles events into an unconditional `INSERT … RETURNING position`; `decode_probe_response` yields `Option<SequencePosition>`; `decode_last_position` yields the last assigned position. The `append` above them is already written and unchanged — it maps a decoded conflict to `AppendError::ConditionViolated(ConditionViolated::at(position))` and refuses the empty batch first. | `crates/happenstance-neon/src/event_store.rs:121-128`, `:439-503` |
| The bodies are real, not repaired | The two round trips stay two round trips and the window stays open. The defect under test is the shape, not a bug in the SQL: an implementation that quietly collapses the two statements has deleted the control. | `crates/happenstance-neon/src/event_store.rs:400-415`; `_decomposition.md` *Architecture brief* §5 |
| The window is closed by a real second writer, deterministically | An interposing `SqlTransport` wraps the real one and lands a conflicting append through a second handle onto the same branch between the probe's response and the insert's request. No sleep, no retry loop, no clock, no thread race. `SECOND_HANDLE` is inherited from `NeonFixture` and is a MUST, so a decline panics rather than skipping. | `crates/happenstance-neon/src/transport.rs` (`SqlTransport`, one method); `crates/happenstance-testkit/src/contract.rs` (`SECOND_HANDLE`); `_decomposition.md` *Architecture brief* §6 |
| One harness, two stores, opposite verdicts | The identical fixture shape is run against `ProbeThenWriteStore` and `NeonEventStore`. The CTE has exactly one round trip, so the injected append necessarily commits *before* its snapshot and must be seen; the two-statement store necessarily misses it. A harness under which both arms agree has not manufactured contention and is not evidence. | `crates/happenstance-neon/src/lib.rs:46-77`; `crates/happenstance-neon/src/event_store.rs:130-165` |
| Failure is asserted **by name** | `catch_unwind` over a non-capturing probe, matching the rule that panicked. `Probe::run` stays a function pointer; no `AssertUnwindSafe` anywhere. A transport error, a decode panic, or a SQLSTATE `40001` abort is not a visibility finding and must not be recorded as one. | `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:325-340`, `:450-458` |
| Rules are composed, never added | `happenstance_testkit::rules::<name>(open: impl AsyncFn() -> F) -> RuleOutcome`, invoked from `crates/happenstance-neon/tests/`. No rule body is added to `suite.rs`, no dependency edge from the testkit to any adapter, and no live endpoint enters `cargo test --workspace`. | `crates/happenstance-testkit/src/lib.rs:189`; `src/suite.rs:89`, `:4949`; `_decomposition.md` *Architecture brief* §9.1 |
| A rule set that cannot reject it is the finding | If no rule fails `ProbeThenWriteStore` under the harness, the rule changes — schedule/body, with its reason in the same change and a `CHANGELOG.md` entry naming the defect. The name is fixed: `spec-trace` check 6 requires every rule in `RULE_FILES` to be claimed by a clause. | `xtask/src/lints.rs:525`; `xtask/src/spec_trace.rs:85-89`; `CLAUDE.md` (*The rule that matters*) |
| The isolation header is answered, not assumed | `Neon-Batch-Isolation-Level` "applies only when `SqlRequest::statements` holds more than one statement — a single statement is its own implicit transaction and the header is ignored", and the CTE is one statement. Establish against the endpoint whether the CTE runs `Serializable` or `ReadCommitted`; at `ReadCommitted` "a conditional insert can miss a conflict committed by a concurrent transaction after this one took its snapshot". | `crates/happenstance-neon/src/transport.rs:56-60`, `:64-66`; `crates/happenstance-neon/src/config.rs` (the `Serializable` default and why) |
| A serialization abort is an outcome, not a pass | Under `Serializable` the endpoint may abort with SQLSTATE `40001`, surfaced through `NeonSqlError::is_serialization_failure`. Whether the adapter retries, and what the suite sees when it does not, is recorded rather than smoothed over. | `crates/happenstance-neon/src/error.rs`; `crates/happenstance-neon/src/config.rs` |
| The verdict is asserted on store-assigned positions | The conflicting position compared against is the one the store assigned to the blocking append, read back from the append that assigned it — never a literal. `cargo xtask lint-position-literals` scans only `suite.rs`, `model.rs` and `concurrency.rs`, so `crates/happenstance-neon/tests/` is unswept and CF-6 holds here by discipline. | `xtask/src/lints.rs:628-640`; `xtask/src/spec_trace.rs:85-89`; `CLAUDE.md` (CF-6 corollary) |
| The ledger correction has exact targets | `RUNBOOK.md:479` (the open `conflicting_position` row) and `RUNBOOK.md:3113-3116` (the phase-4 exit record's "a boolean and no row") are rewritten to what the endpoint showed; `references/adapter-shapes.md:116-125` gains the verdict beside its "unlooked-for result"; `crates/happenstance-neon/src/lib.rs:46-77` gains the evidence and the isolation finding. | `RUNBOOK.md:479`, `:3113-3116`; `references/adapter-shapes.md:116-125`; `crates/happenstance-neon/src/lib.rs:46-77` |
| ADR-0012 is neither edited nor superseded | Its conclusion — a hint, `Option`, demotion rejected — is correct under both outcomes. Accepted atoms are immutable and validated against `HEAD`; superseding a correct decision over a wrong supporting sentence costs the phase-12 audit a chain to re-derive. | `.kb/decisions/0012-append-shape-and-preconditions.md:26`, `:99-101`; `CLAUDE.md` (*Where the work lives*); `discover.md`, *The ledger mutant* |
| A refutation of append semantics goes through the intake | Only if the CTE fails to reject a conflict it should have seen: the record is staged in `.kb/_intake/` with the evidence, written **before** any behaviour change, and reaches `.kb/decisions/` only via `/redkiln:kb-ingest`. No atom is hand-authored. A missing-`conflicting_position` outcome owes no record — ES-25 permits `None`. | `_decomposition.md` *Architecture brief* §2 (Root D); `project.md` DR-5, DR-7; `spec/SPECIFICATION.md:3746-3751` |
| Gating is whole-invocation | `#[ignore]`, `required-features`, or an env read — whichever HS-S0068 chose, inherited unchanged. Never a `#[cfg]` hiding a rule out of a macro expansion (DR-5). `cargo test --workspace --all-features` exits zero with no endpoint reachable and no credentials present. | `_decomposition.md` *Architecture brief* §7, *Testing brief* Notes §6; `.redkiln/config.yaml` (`integration_scoped: cargo xtask ci --fast`) |
| The wasm32 claim does not regress | Anything added must keep `happenstance-neon` building for `wasm32-unknown-unknown` in the gate, and the store stays on the **bare** `EventStore` on both targets — a second `SendEventStore` impl is `error[E0119]` against `trait_variant`'s blanket impl. Test-only code that cannot cross-compile is `cfg`-scoped, not excused. | `crates/happenstance-neon/src/lib.rs:79-89`; `crates/happenstance-neon/src/event_store.rs:4-34`; `xtask/src/main.rs:264-282`; `CLAUDE.md` constraint 1 |
| Nothing becomes public API | Every addition is a `#[test]`, `pub(crate)`, or local to the test target. `_design.md` declares no items and no visibility changes. | `.../postgres-and-neon-stores/_design.md` (`## Items`, `## Visibility and stability` — both `N/A`) |

## Data and migrations

**No migration is authored or altered here.** Migration 1 — identity, time, tag
storage and the ES-10 mechanism's column — belongs to
`postgres-schema-and-live-fixture` (HS-S0060), and Neon inherits it unchanged;
that inheritance is the reason Postgres and Neon are one project and not two
(`project.md`, *Coupling notes*; `_storymap.md`, `neon-append-and-read-over-http`).

Three data-shaped facts this story runs against, and the constraint on each:

1. **The probe and the insert read and write the same table the CTE does.**
   `probe_request` and `insert_request` must compile against the *same* schema
   and the *same* tag encoding as `conditional_append_request`, or the negative
   control is comparing two adapters rather than two append shapes. Table names
   come from `NeonConfig` (`event_table`), not from a literal
   (`crates/happenstance-neon/src/config.rs`).
2. **The contended run needs an isolated backing store, and a second handle onto
   it.** One fixture instance is one isolated backing store; each `connect()` is
   one handle onto that store (`CLAUDE.md`, *The rule that matters*). Whichever
   isolation HS-S0068 chose for `NeonFixture` — a branch, a schema, or a
   table-name prefix — is inherited unchanged. An interposing writer that lands
   its append in a *different* store proves nothing, and a run that shares a
   store with the shipped-arm run can fail for reasons neither arm owns.
3. **Payloads travel as hex, and the ceiling is halved in effect.** `bytea`
   parameters are rendered `\x…` in JSON, roughly doubling their size against
   `MAX_RESPONSE_BYTES` in both directions
   (`crates/happenstance-neon/src/transport.rs:87-102`, `:45-54`). This story
   asserts nothing about ceilings — that is HS-S0068's and HS-S0069's — but its
   fixtures must not silently exceed one and report the resulting
   `NeonError::ResponseTooLarge` as a conformance verdict.

**No schema change falls out of either outcome.** A confirmation changes nothing.
A refutation caused by the isolation header changes the *request shape* — how
many statements the append is sent as — not the tables it touches.

## Acceptance criteria

Each criterion is written from the intent of a reader this initiative names: the
**adapter author** on the *Learn when you are finished* journey, who needs the
run to say pass or fail and name why; the **evaluator** on *Decide in one
sitting*, whose whole budget is a bounded look at public evidence; and the
**constrained-runtime developer** on *Event-source at the edge*, for whom Neon is
the store with no connection at all
(`.bklg/from-contract-to-published-library/initiative.md:241-250`). None of them
meets a screen here. What they meet is a run that reports and a plan of record
that is true.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** an adapter author who has been told `ProbeThenWriteStore` is this workspace's named wrong implementation for Neon, **WHEN** they run the adapter's tests the ordinary way — the credentialed job's `cargo test -p happenstance-neon --all-features -- --ignored --show-output`, with **no workflow edit** — **THEN** the store actually executes against the live endpoint over its two round trips (all four remaining bodies real, compiled against the same `NeonConfig` table names and the same tag encoding the CTE uses) instead of panicking at a `todo!()`; **AND** the same invocation with no credential present exits zero rather than failing, because the gating is whole-invocation and never a `#[cfg]` hiding a rule out of a macro expansion (DR-5). | `crates/happenstance-neon/tests/conflicting_position.rs::probe_then_write_store_appends_over_two_round_trips` (live-gated) + `cargo test --workspace --all-features` green on a machine with no endpoint and no credential (`cargo xtask ci --fast`, `tests` step) |
| **AC-002** | **GIVEN** an evaluator who knows that a single-client "conflict comes back" run proves nothing, **WHEN** they read what the harness did, **THEN** a **real second writer** landed a conflicting append onto the *same* fixture-isolated backing store, through a second handle, strictly between the probe's response and the insert's request — closed by an interposing `SqlTransport`, not by a sleep, a clock read, a thread pile-up or a retry-until-green loop; **AND** the run fails loudly if that interleaving did not in fact occur, rather than reporting a pass it did not earn. | `crates/happenstance-neon/tests/conflicting_position.rs::interposed_writer_lands_inside_the_probe_insert_window` — asserts the injected append's store-assigned position was committed before the insert request left, and `panic!`s on EC-004 |
| **AC-003** | **GIVEN** an adapter author asking whether this workspace's compliance claim can distinguish a one-snapshot CTE from two lucky statements, **WHEN** the identical contended harness is driven over both stores in one run, **THEN** `ProbeThenWriteStore` **fails a rule by name** and `NeonEventStore` **passes that same rule**, the failure asserted through `catch_unwind` over a **non-capturing** probe (`Probe::run` stays a function pointer; no `AssertUnwindSafe` appears anywhere); **AND** the `--show-output` log names the rule that panicked and quotes the fixture's own words, so the verdict is legible from the log alone and not only from the exit code. | `crates/happenstance-neon/tests/conflicting_position.rs::probe_then_write_fails_the_rule_the_cte_passes` — expected candidate `condition_rejection_is_reported_as_condition_violated` (`crates/happenstance-testkit/src/suite.rs:4949`); the panic payload is matched against the rule name, and EC-005 rejects any other panic |
| **AC-004** | **GIVEN** the same author, **WHEN** no rule in the existing set rejects `ProbeThenWriteStore` under contention, **THEN** that is recorded as a finding **about the rule set** and the rule is fixed in the same change — schedule and body only, **never the name** (`spec-trace` check 6 requires every rule in `RULE_FILES` to be claimed by a clause) — with its reason stated in the change and a `CHANGELOG.md` entry naming the defect it now detects; **AND** proceeding without either is not available. | `cargo xtask spec-trace` + `cargo xtask lint-changelog` (`xtask/src/lints.rs:525`, `changelog_names_every_rule`) + `cargo xtask lint-retired-rules`, all inside `cargo xtask ci`; the amended rule re-run by `crates/happenstance-neon/tests/conflicting_position.rs::probe_then_write_fails_the_rule_the_cte_passes` |
| **AC-005** | **GIVEN** an evaluator reading the crate's claim that "contrary to the standing assumption in the decision ledger, the collapse **keeps** `ConditionViolated::conflicting_position`", **WHEN** the CTE meets a real `/sql` endpoint under the AC-002 contention, **THEN** the claim is settled by evidence: either the append is rejected with `Some(p)` where `p` is the position **the store assigned** to the interposed append — read back from the append that assigned it, never a literal (CF-6 holds here by discipline; `lint-position-literals` does not sweep `crates/happenstance-neon/tests/`) — or the refutation is recorded with what the endpoint actually returned. | `crates/happenstance-neon/tests/conflicting_position.rs::cte_reports_the_position_the_interposed_append_was_assigned` (live-gated), asserting on the returned position, plus the recorded verdict in `crates/happenstance-neon/src/lib.rs` |
| **AC-006** | **GIVEN** the constrained-runtime developer, for whom the CTE's soundness is the whole guarantee, **WHEN** they ask at what isolation the append actually ran, **THEN** the spec's open question is **answered against the endpoint, not reasoned about**: whether `Neon-Batch-Isolation-Level` reaches a one-statement request at all, and — if it does not — either the minimal re-shaping of the append into a form the header applies to, or the unsoundness recorded as the finding; **AND** a SQLSTATE `40001` serialization abort is surfaced through `NeonSqlError::is_serialization_failure` and recorded as a retryable abort, never laundered into either a pass or a conflict rejection. | `crates/happenstance-neon/tests/conflicting_position.rs::append_isolation_level_observed_at_the_endpoint` (live-gated) + the recorded answer in `crates/happenstance-neon/src/lib.rs`; EC-002 covers the abort path |
| **AC-007** | **GIVEN** the `publication-and-positioning` (HS-P0016) auditor who will later have to say what the contract promises, **WHEN** the endpoint shows the CTE's **append semantics** unsound — a condition that fails to reject, not merely a missing field — **THEN** a decision record carrying the evidence is **staged in `.kb/_intake/` before any behaviour change**, reaching `.kb/decisions/` only through `/redkiln:kb-ingest` and never hand-authored; **AND** where the outcome is instead a missing `conflicting_position`, no record is owed at all and none is written, because ES-25 already permits `None`. | `redkiln validate --kb` + `redkiln doctor` green; the staged file under `.kb/_intake/` present in the diff *before* any `event_store.rs` behaviour change, checked at review against `spec/SPECIFICATION.md:3746-3751` |
| **AC-008** | **GIVEN** anyone who later reads the plan of record to learn why `conflicting_position` is only a hint, **WHEN** they read it after this PR, **THEN** they meet the evidence and not the conjecture: `RUNBOOK.md:479` and `RUNBOOK.md:3113-3116` no longer read as they do today, `references/adapter-shapes.md:116-125` carries the verdict beside its "unlooked-for result", and `crates/happenstance-neon/src/lib.rs:46-77` carries the live-endpoint evidence and the isolation finding — the correction landing **at the wrong sentences**, not in a new document beside them; **AND** ADR-0012 is neither edited nor superseded, its body byte-identical to `HEAD`, because its conclusion is correct under both outcomes. | `crates/happenstance-neon/tests/conflicting_position.rs::ledger_targets_are_corrected` — a credential-free `#[test]` asserting the two superseded sentences are absent from `RUNBOOK.md`; plus `redkiln validate --kb` (accepted-atom immutability against `HEAD`) and `cargo doc` in `cargo xtask ci` |

Project **AC-008** ("`conflicting_position` is settled by evidence, not by
assumption", `project.md:253-257`) is covered end to end: its first half by
AC-005 with AC-002/AC-003 supplying the contention and the negative control that
make the observation worth anything, its escape hatch by AC-007, and its second
half — "the decision ledger's standing assumption is corrected either way" — by
AC-008.

## Interaction quality

This story renders **no surface**. `_design.md` records the project's
no-surface determination explicitly and it was approved at the `/redkiln:plan`
design gate on 2026-08-12, with `## Surfaces`, `## Items` and
`## Visibility and stability` all `N/A`
(`.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md:11-20`,
`:46-64`, `:90-95`). So the **composition family owes no AC rows here** — there
is no presentation, placement, transience, density budget or hierarchy to hold,
and saying otherwise would invent a design a human never signed off. The medium
this story does deliver into is a **run's report and a plan of record**, and the
state family applies to it in full, unweakened.

Every invariant below is carried by a row in the table above; none is left as
prose, because `redkiln verify` extracts ACs from table cells and a bullet here
would be gated by nothing.

**State invariants, and the AC that carries each.**

- **In-place, not a context jump** — AC-008. The correction is written where the
  wrong sentences are (`RUNBOOK.md:479`, `:3113-3116`,
  `references/adapter-shapes.md:116-125`, `lib.rs:46-77`). A new note beside them
  leaves a reader who lands on line 479 still reading "open — Neon-over-HTTP is
  the forcing case", which is the quietest wrong move this spec names.
- **Non-occlusion** — AC-003 and AC-006, with EC-002 and EC-005. The verdict must
  not be hidden behind something that merely looks like it: a transport error, a
  decode panic, or a `40001` abort recorded as a visibility finding is the weak
  control that reaches the right answer for the wrong reason.
- **Preserved referent** (the medium's analogue of preserved selection) — AC-002
  and AC-005. The thing asserted on is the position **the store assigned**, read
  back from the append that assigned it, and the fixture's inherited capability
  answers and ceilings travel unchanged from `NeonFixture`. A literal position or
  a re-derived expectation silently swaps the referent under the assertion.
- **Reversibility** — AC-007. The record is staged **before** any behaviour
  change, so the change can be read against the evidence that justified it rather
  than reconstructed from it afterwards.
- **Reachability without a bespoke incantation** (the analogue of keyboard
  reachability) — AC-001. The run is swept up by the credentialed job that
  already exists, with no workflow edit and no hand-picked subset, and the same
  invocation is harmless on a machine with no credential.
- **Legibility of the report** — AC-003. `--show-output` names the rule that
  failed and quotes the fixture's own words. An exit code alone tells the
  evaluator on a one-sitting budget nothing, and it is the only "presentation"
  this medium has.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | No live endpoint or no credential present. | The **whole invocation** is skipped by the mechanism HS-S0068 chose, inherited unchanged; `cargo test --workspace --all-features` exits zero. No verdict is recorded and no ledger correction is written from a run that did not happen (AC-001, AC-008). |
| **EC-002** | The endpoint aborts the CTE with SQLSTATE `40001` under contention. | Surfaced through `NeonSqlError::is_serialization_failure` (`crates/happenstance-neon/src/error.rs`) and recorded as a **retryable abort** — a legitimate outcome of `Serializable`, and neither a conflict rejection nor a pass. Whether the adapter retries, and what the suite sees when it does not, is written down rather than smoothed over (AC-006). |
| **EC-003** | A fixture payload exceeds the `MAX_RESPONSE_BYTES`-anchored ceiling, halved in effect by hex `bytea` rendering (`crates/happenstance-neon/src/transport.rs:87-102`). | `NeonError::ResponseTooLarge` fails the run as a **harness defect**. It is never recorded as a conformance verdict; ceilings are HS-S0068's and HS-S0069's ACs, not this story's. |
| **EC-004** | The interposed writer's append does not land inside the probe→insert window — it commits after the insert request left, or against a different backing store. | The test **panics with "contention not manufactured"**. A run in which the window was never closed is exactly the single-client confirmation this story exists to reject, and it must never report a pass (AC-002). |
| **EC-005** | `catch_unwind` catches a panic that is not the rule's non-conformance panic — a transport failure, a decode panic, an assertion inside the harness. | The test fails. Only a panic whose payload names the rule counts as the rejection AC-003 requires. The probe stays non-capturing so no `AssertUnwindSafe` can quietly widen what is caught (`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:325-340`). |
| **EC-006** | The inherited `NeonFixture` declines `SECOND_HANDLE`. | The run **panics quoting the fixture's stated reason** rather than skipping. `SECOND_HANDLE` is a MUST for Neon (`_decomposition.md` *Testing brief* Notes §7); with it declined there is no second writer and the whole instrument is decorative. |
| **EC-007** | No rule in the existing set rejects `ProbeThenWriteStore`. | Not an error to route around: AC-004's path is taken — the rule is fixed with its reason and a `CHANGELOG.md` entry in the same change, name unchanged. |
| **EC-008** | The endpoint shows the CTE failing to reject a conflict it should have seen (unsound append semantics). | AC-007's path: DR-7 applies, the record is staged in `.kb/_intake/` **first**, and no behaviour change lands ahead of it. A `[FROZEN]` clause changes by decision record, never by edit. |

## Non-functional

| id | requirement | why it binds here |
| --- | --- | --- |
| **NF-001** | **Determinism.** No clock read, no `sleep`, no retry-until-green loop, no thread race anywhere in the harness. The interleaving is produced by the interposing transport, so the same run is repeatable. | CF-33's spirit. `cargo xtask lint-clock` sweeps the rule files, not `crates/happenstance-neon/tests/`, so this is held by discipline and stated here rather than assumed. A flaky live job gets quietly weakened, which `project.md`'s risk row 4 (DR-9) names. |
| **NF-002** | **The default gate stays Docker-free and network-free.** `cargo xtask ci --fast` is green on a clean checkout with no Docker and no credentials. | Project AC-011, DR-9, `_decomposition.md` *Testing brief* Notes §6. This story adds a test target to a crate the default gate compiles. |
| **NF-003** | **The `wasm32` claim does not regress.** `happenstance-neon` still builds for `wasm32-unknown-unknown` in the gate, and the store stays on the **bare** `EventStore` flavour — a second `SendEventStore` impl is `error[E0119]` against `trait_variant`'s blanket impl. Test-only code that cannot cross-compile is `cfg`-scoped, not excused. | `CLAUDE.md` constraint 1 and 4; `crates/happenstance-neon/src/lib.rs:79-89`; `xtask/src/main.rs:264-282`. |
| **NF-004** | **No public API growth.** Every addition is a `#[test]`, `pub(crate)`, or local to the test target; the four bodies replaced are already-declared private methods and private free functions. | `_design.md` declares no items and no visibility changes; an item that became `pub` here is a semver promise nobody made, on a crate that is about to lose `publish = false`. |
| **NF-005** | **No dependency edge from `happenstance-testkit` to any adapter**, and no live endpoint inside `cargo test --workspace`. | The rules are *composed* from the adapter's `tests/` through the public rule functions precisely so neither happens (`_decomposition.md` *Architecture brief* §9.1, *Testing brief* Notes §4; DR-9; `CLAUDE.md`'s adapter-on-adapter prohibition). |
| **NF-006** | **The contended run leaves the shared branch as it found it.** Fixture isolation is inherited from `NeonFixture` — one fixture instance is one isolated backing store — and the interposed writer targets that same store, never a neighbour's. Round trips are bounded; this is not a load test. | `CLAUDE.md`, *The rule that matters*; `_decomposition.md` *Testing brief* Notes §7. A run that shares a store with the shipped-arm run can fail for reasons neither arm owns. |
| **NF-007** | **`ProbeThenWriteStore` is not made faster, safer, or correct.** Its two round trips stay two round trips and its window stays open. | It is the control. `_decomposition.md` *Architecture brief* §5: "Do not delete it, do not fix it, and do not leave it merely documented." |

## Implementation notes (non-prescriptive)

Shapes that fit the constraints above; none of them is mandated, and a better
one that meets the ACs is welcome.

- **The interposing transport is the cheapest deterministic seam in the tree.**
  `SqlTransport` has one method, so a wrapper holding the real transport plus a
  one-shot `FnOnce` can inspect each outgoing request, and — on seeing the
  insert follow the probe — run the conflicting append through a second handle
  before forwarding. Against `NeonEventStore` the same wrapper has nowhere to
  land, because there is exactly one round trip; that asymmetry *is* the
  experiment, so prefer one wrapper used twice over two bespoke harnesses.
- **Recognising "the insert" without parsing SQL.** A counter over the round
  trips of a single `append` call is usually enough, and it survives the SQL
  being rewritten. Matching on statement text is a hidden coupling to
  `insert_request`'s formatting.
- **The `catch_unwind` shape already exists — copy it, do not invent it.**
  `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:325-340` and
  `:450-458` show the non-capturing probe and why. The first refactor anyone
  reaches for — hoisting the fixture out and capturing it in the closure — is
  precisely what forces `AssertUnwindSafe` and quietly widens what the assertion
  accepts.
- **Silence the panic output for the expected failure** if the log becomes
  unreadable, but do not suppress the payload: AC-003 needs the rule's name in
  `--show-output`.
- **Answering the isolation question may cost one extra probe request.** A
  single round trip that makes the endpoint state its effective isolation is
  cheaper evidence than inferring it from whether a conflict was seen — and it
  separates "the header was ignored" from "the CTE is wrong", which are different
  findings with different consequences (AC-006).
- **Sequencing inside the slice.** This story is item 4 of the merge order
  (`_storymap.md`). Land AC-001's four bodies first — they are inert until the
  harness exists — then AC-002's harness, then the two-arm run. The ledger
  correction (AC-008) is written **last**, from what the run showed, never
  drafted ahead of it in either direction.
- **The staged decision record, if AC-007 fires,** is a document in
  `.kb/_intake/` written in the repository's ADR voice; it does not carry
  `KbFrontmatter` as an atom and must not be placed under `.kb/decisions/` by
  hand. `/redkiln:kb-ingest` is the only path in, and hand-authoring is what
  commit `0269720` reverted.
- **The credential-free ledger test** (AC-008) can locate the two files from
  `env!("CARGO_MANIFEST_DIR")`; keeping it in the same target as the live tests
  is fine, since the gating is whole-invocation on the *ignored* ones and this
  one is not ignored.

## Tests and CI (merge gate)

| tier | command / path | proves |
| --- | --- | --- |
| **Static / process** | `cargo xtask ci --fast` (`.redkiln/config.yaml`, `verify.integration_scoped`) | fmt, clippy `-D warnings`, `cargo test --workspace --all-features`, the wasm32 steps and docs are green with **no Docker, no network, no credential** — NF-002, NF-003, and AC-001's credential-free half. |
| **Static / process** | `cargo xtask affected --base main` (`verify.affected_gate`) | the story grain: only what this diff could break, run before the checkpoint commit. |
| **Static / process** | `cargo xtask spec-trace`, `cargo xtask lint-retired-rules`, `cargo xtask lint-changelog`, `cargo xtask lint-position-literals` (all inside `cargo xtask ci`) | no rule was renamed or retired out from under a clause, and — if AC-004 fires — the amended rule carries a `CHANGELOG.md` entry naming the defect (`xtask/src/lints.rs:525`, `xtask/src/spec_trace.rs:85-89`). |
| **Static / process** | `redkiln validate --kb && redkiln doctor` | ADR-0012's accepted body is byte-identical to `HEAD` (AC-008), and any staged AC-007 record has not been smuggled into `.kb/decisions/`. |
| **Unit (credential-free)** | `crates/happenstance-neon/tests/conflicting_position.rs::ledger_targets_are_corrected` | AC-008: neither `RUNBOOK.md:479` nor `RUNBOOK.md:3113-3116` still reads as it does today. This is the only mechanised guard against the "correct the crate doc and stop there" failure — nothing else in the gate reads that table. |
| **Conformance / live Neon job** | `cargo test -p happenstance-neon --all-features -- --ignored --show-output` (the job HS-S0068 authors in `.github/workflows/ci.yml`; **consumed, not edited**) | AC-001, AC-002, AC-003, AC-005, AC-006 — the four bodies execute, the window is closed by a real second writer, `ProbeThenWriteStore` fails a named rule the CTE passes in the same run, and the CTE's verdict and effective isolation are on record. `_decomposition.md` *Testing brief* Notes §6. |
| **Conformance / live Neon job** | `event_store_conformance!(NeonFixture::…)` — HS-S0069's, re-run unchanged on this tree | the shipped store still passes the full macro expansion after the four `ProbeThenWriteStore` bodies land; this story adds no rule and subtracts none (`_decomposition.md` *Testing brief* Notes §8). |
| **Whole gate (pre-merge)** | `cargo xtask ci` | the terminal bar including `cargo hack`, `cargo deny`, the nightly `--cfg docsrs` rustdoc build and `package-check`; run once before saying done. |

The two live families are excluded from the default gate **by construction** —
the gating is whole-invocation (`#[ignore]`, `required-features`, or an env read,
whichever HS-S0068 chose), never a `#[cfg]` hiding one rule out of a macro's
expansion, which DR-5 forbids and which would make project AC-007's "no rule is
absent from the run" unprovable.

## Risks and coupling (PR-scoped)

| risk | why it is live here | the control |
| --- | --- | --- |
| **The single-client confirmation.** A green run that proves nothing, because `ProbeThenWriteStore` returns a conflicting position in exactly the sequential scenario too. | It is the cheapest way to close AC-008 and every check passes. | AC-002 and AC-003 make the negative control the deliverable, and EC-004 fails the run when the window was not actually closed. |
| **The verdict reached for the wrong reason.** A transport error, a decode panic or a `40001` abort recorded as the rejection. | Under `Serializable` an abort is a *likely* outcome of manufactured contention, and it panics in roughly the right place. | AC-003's by-name panic match, EC-002 and EC-005. |
| **The ledger correction quietly narrowed to the crate doc.** | Nothing in the gate reads `RUNBOOK.md:479`, and the crate doc is where the claim feels like it lives. | AC-008's named line targets and the credential-free `ledger_targets_are_corrected` test. |
| **Superseding ADR-0012 because the KB machinery makes it easy.** | `validate --kb` stays green, `doctor` stays clean, the supersession graph is well formed — and the phase-12 audit inherits a chain to re-derive for a conclusion that never moved. | AC-008 states the atom is neither edited nor superseded; `discover.md`, *The ledger mutant*, records why. |
| **`ProbeThenWriteStore` "improved" while its bodies are written.** Collapsing the two statements, or adding a retry, deletes the control. | The four `todo!()`s sit inside the wrong implementation, and writing correct-looking code there is the natural instinct. | NF-007 and the *Architecture brief* §5 instruction; AC-003 fails immediately if the control starts passing. |
| **Blocked on HS-S0069 in a way that invites stubbing.** There is nothing to verify until the CTE actually runs, and a `NullTransport`-backed stand-in compiles fine. | Slice pressure: three stories land in one context. | Dependencies below; DR-4's prohibition on a pooled Postgres connection standing in for the endpoint; `NullTransport` fails every round trip by design and is named in the PR boundary as not a substrate. |
| **A flaky live job made non-blocking.** | Two of this project's gates need infrastructure, and contention harnesses are where flakiness usually enters. | NF-001's no-clock/no-race constraint; DR-9 — a flaky live job is fixed or reported, never made non-blocking without a recorded decision (`project.md:340`). |
| **Coupling out:** `deskeleton-and-package-readiness` (HS-S0072) asserts no `todo!()` survives in `happenstance-neon`. | These four bodies are unowned by any other story. | AC-001; the story map lists this story among HS-S0072's dependencies (`_storymap.md:71`). |

## Dependencies

**Blocks on**

- `neon-append-and-read-over-http` (HS-S0069) — supplies the CTE's six shipped
  `todo!()` bodies, the decoder that can report a conflicting position, the wired
  `NeonEventStore`, and the first green `event_store_conformance!` run. There is
  nothing to verify until the CTE actually runs
  (`_storymap.md`, *Merge order* item 4). Transitively: `neon-fixture-and-live-job`
  (HS-S0068) for `NeonFixture`, its capability answers and the credentialed CI
  job; `neon-sql-transport` (HS-S0067) for the real `SqlTransport`.

**Unlocks**

- `deskeleton-and-package-readiness` (HS-S0072) — cannot assert "no `todo!()` on
  either path" or remove the scoped `#![allow(clippy::todo)]` until this story's
  four bodies land (`_storymap.md:71`).
- `far-end-discharge-record` (HS-S0073) — receives the ES-25 line of the audit
  record, and the corrected ledger sentences the publication audit will read.

## Anchors (progressive disclosure)

Load-bearing depth is deferred, not optional. Open each one at the moment named;
do not read the column of them up front.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-neon/src/event_store.rs` | Carries `ProbeThenWriteStore` in full (`:388-503`), the four `todo!()` bodies this story owns (`:121-128`, `:494-503`), the lost-update explanation (`:400-415`), and the CTE the shipped store sends (`:130-165`). The comparison's two arms are both here. | Before writing a line of AC-001; again before AC-003, to see exactly which round trip the interposition sits between. | AC-001, AC-003 |
| `crates/happenstance-neon/src/transport.rs` | The one-method `SqlTransport` the interposing wrapper wraps; the isolation-header caveat in terms (`:56-60`), the `ReadCommitted` consequence (`:64-66`), and the hex `bytea` doubling that halves the ceiling (`:87-102`, `:45-54`). | Before designing the harness (AC-002), and before answering the isolation question (AC-006). | AC-002, AC-006 |
| `crates/happenstance-testkit/tests/mutation_coverage/harness.rs` | The in-tree pattern for asserting that a rule **rejects** an implementation: `catch_unwind` over a non-capturing probe, `Probe::run` as a function pointer, and why no `AssertUnwindSafe` may appear (`:325-340`, `:450-458`). | Before writing the negative-control assertion; copy this shape rather than inventing one. | AC-003 |
| `crates/happenstance-testkit/src/suite.rs` | The rule bodies and the public per-rule entry points (`:89`); `condition_rejection_is_reported_as_condition_violated` at `:4949` is the expected candidate for the rule that must fail. Also the file AC-004 would amend — body and schedule only, never a name. | Before choosing which rule to drive (AC-003); again only if no rule rejects the control (AC-004). | AC-003, AC-004 |
| `crates/happenstance-testkit/src/lib.rs` | Exports the `rules` module (`:189`) — the seam that lets an adapter's `tests/` drive the rule set without the testkit depending on an adapter. | Before wiring the test target's imports. | AC-003, NF-005 |
| `crates/happenstance-testkit/src/contract.rs` | `Fixture`, `Capability`, `SECOND_HANDLE`, `REOPEN` and the ceiling constants the contended fixture inherits from `NeonFixture` — and the fact that a declined capability still runs the rule, quoting the fixture's reason. | Before writing the contended fixture wrapper. | AC-002, EC-006 |
| `crates/happenstance-neon/src/config.rs` | `NeonConfig`'s table names (the probe and insert must not hard-code them) and the `IsolationLevel::Serializable` default with its stated reason. | Before AC-001's two request builders; again for AC-006. | AC-001, AC-006 |
| `crates/happenstance-neon/src/error.rs` | `NeonSqlError::is_serialization_failure` — the SQLSTATE `40001` path that must be recorded as a retryable abort rather than a pass or a rejection. | When the first `40001` appears, which under `Serializable` is likely. | AC-006 |
| `crates/happenstance-neon/src/lib.rs` | Lines `:46-77` are the claim under test, the CTE written out, and the sentence that contradicts the ledger; `:79-89` the wasm32 claim; `:101-105` the scoped `todo` allow. This file is also an AC-008 correction target. | Read `:46-77` before AC-005; edit it last, from what the run showed. | AC-005, AC-006, AC-008, NF-003 |
| `RUNBOOK.md` | The two sentences the evidence contradicts — `:479` (the open `conflicting_position` row) and `:3113-3116` (the phase-4 exit record's "a boolean and no row"). These are AC-008's exact targets, and nothing in the gate reads them but the test AC-008 adds. | At AC-008, after the verdict exists. Not before — drafting the correction ahead of the run is how a preferred answer gets written down. | AC-008 |
| `references/adapter-shapes.md` | Lines `:116-125` repeat the CTE claim as an unverified "unlooked-for result" — the third correction target, and the one most easily forgotten. | With the other two, at AC-008. | AC-008 |
| `spec/SPECIFICATION.md` | ES-25 at `:3746-3751`, marked `[FROZEN]` in the clause row at `:8607`: the field is informational, `None` is permitted, callers MUST NOT depend on it. This is why **neither** outcome needs a clause amendment — and the boundary of what AC-007 covers. | Before concluding that any outcome forces a specification change. | AC-005, AC-007 |
| `.kb/decisions/0012-append-shape-and-preconditions.md` | The accepted atom whose *conclusion* stands under both outcomes (`:26`, `:99-101`), and which is immutable — `redkiln validate --kb` checks it against `HEAD`. | Before touching anything in `.kb/`; it is the atom this story must leave alone. | AC-008 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` | *Architecture brief* §5 ("do not delete it, do not fix it"), §9.1 (the rule-function seam and the dependency direction), §7 (gate topology); *Testing brief* Notes §4 (the negative test), §6 (whole-invocation gating and the exact job invocations), §7 (the one instrument this project consumes rather than authors). | Notes §4 and §7 before the harness; §6 before deciding anything about gating. | AC-001, AC-002, AC-003, NF-002, NF-005 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/neon-conflicting-position-verdict/discover.md` | The signal ledger with every line citation, and the three named wrong implementations — the single-client confirmation, superseding ADR-0012, and correcting the crate doc alone. | If any AC's intent is unclear, or before choosing a vehicle for the ledger correction. | AC-003, AC-008 |
| `xtask/src/lints.rs` | `changelog_names_every_rule` (`:525`) and `no_position_literals` (`:628-640`) — what the gate checks if AC-004 fires, and the precise reason CF-6 is discipline rather than mechanism in `crates/happenstance-neon/tests/`. | Only if no rule rejects the control (AC-004); and before writing any position assertion. | AC-004, AC-005 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` | The signed-off, human-approved determination that this project renders **no** user-facing surface (`:11-20`, `:46-64`, `:90-95`). It is why this spec owes no composition ACs and why nothing here may become public API. | Before adding any `pub` item, or if a surface seems to be owed. | AC-001, NF-004 |
| `.github/workflows/ci.yml` | The job topology this story's test target must be swept up by **without a workflow edit** — the Neon job is HS-S0068's to author, a sibling of `gate` / `wasm-conformance` / `msrv` / `semver` / `advisories`. | Before assuming the run needs new CI wiring; if it does, the assumption about gating was wrong. | AC-001 |

## Clarifications resolved during spec

1. **Discovery §5 — the vehicle for the ledger correction.** Resolved: the
   RUNBOOK rows themselves, plus `references/adapter-shapes.md:116-125` and the
   crate doc. **No new KB atom, no edit to ADR-0012, no supersession.** The
   correction belongs where the wrong sentences are; a superseding atom would
   pass every check and cost the phase-12 audit a chain to re-derive for a
   conclusion that never moved (`discover.md`, *The ledger mutant*). AC-008
   carries it, and the credential-free `ledger_targets_are_corrected` test is what
   stops the correction narrowing to the crate doc.
2. **Discovery §6 — how contention is manufactured.** Resolved: **one**
   interposing `SqlTransport` wrapping the real one, landing a single conflicting
   append through a second handle strictly between the probe's response and the
   insert's request. Not a client count, not a race, not a retry loop, no clock.
   The asymmetry that makes it evidence is that the same wrapper has nowhere to
   land against the CTE's single round trip. AC-002 carries it; EC-004 fails the
   run when the interleaving did not occur; NF-001 forbids the timing-based
   alternatives.
3. **Discovery §7 — whether the `Serializable` header reaches a one-statement
   request.** Deliberately **not** resolved on paper: it is AC-006, and the
   answer is taken from the endpoint. What the spec fixes is the shape of the
   answer — the header's reach is observed, and if it does not reach, the choice
   is between re-shaping the append into a form the header applies to and
   recording the unsoundness as the finding. Reasoning to a preferred answer here
   would reproduce exactly the failure this story exists to correct.
4. **Where the credential-free ledger assertion lives.** Resolved: in the same
   test target as the live tests, un-`#[ignore]`d. The gating is whole-invocation
   on the *ignored* tests (DR-5), so a non-ignored sibling runs everywhere and
   costs the default gate nothing.
5. **AC count.** Exactly the eight the front half enumerated — AC-001 through
   AC-008 — with the two ids the front half named by hand kept where it put them:
   AC-004 is the rule-set finding
   (*Integration contract*, "if none rejects it… (AC-004)") and AC-007 is the
   staged decision record (*Executive summary*, "bounded in AC-007"). None added,
   none dropped.
6. **Interaction quality's composition family.** Resolved as **not owed**, on the
   signed-off `_design.md`'s record of no user-facing surface, rather than left
   silent. What the medium does have — the legibility of the run's report — is an
   AC row (AC-003), not a prose bullet, because a bullet in that section is
   extracted by nothing and gated by nothing.
