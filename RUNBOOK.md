# Runbook

The plan for finishing happenstance, and the record of how far it has got.

## What changed, and why

The previous runbook sequenced by artefact — SQLite, then the projection suite,
then the typed layer — and every phase's gate was "the conformance suite is
green". That order settles the contract last, in the phase most expensive to
revise, and accepts a green gate as evidence that a *design* is right when the
gate only tests the code that exists.

[`references/evaluation/revised-runway.md`](references/evaluation/revised-runway.md) proposed the
correction and got the method right. Its bodies did not survive
[`PRESSURE-TEST.md`](references/evaluation/PRESSURE-TEST.md): it re-used ADR number 0007,
which is accepted on disk; it dropped four live ledger rows and one warning; its
phase 2 exit criterion mandated its own outcome; its phase 1 could not exit
because its evidence was required to come from a later phase; and its headline
claim about `trait_variant` was compiled and refuted.

The larger change is that the design questions this file used to *schedule* are
now *answered*. [`spec/SPECIFICATION.md`](spec/SPECIFICATION.md)
carries 201 numbered clauses across three ports — 138 `[FROZEN]`, 46
`[PROVISIONAL]`, 12 `[DEFERRED]`, five `[NON-NORMATIVE]`. Those figures are
`cargo xtask spec-trace`'s rather than this file's: §7.1 is generated, and the
checker fails the gate when §1.3's prose disagrees with it — which is the only
reason a count in a document this long can be trusted at all. This file no longer
decides what a port promises. It executes those clauses, discharges the
provisional and deferred ones against named experiments, and makes the 57 cases
in [`E2E-CASES.md`](spec/E2E-CASES.md) writable in an order that puts the
highest blast radius first.

This file has since been walked adversarially against the specification, the case
catalogue and the previous runbook's ledger. Eighteen defects were found and are
fixed in place; they are listed at [What the adversarial pass
changed](#what-the-adversarial-pass-changed) so that nobody re-derives them, and
the three that need a human are listed there as open questions.

Four things are kept from the revised runway, because they are the good part:

- Sequence by **blast radius**, not by artefact.
- A phase's proof is a named **artefact that would not exist if the design were
  wrong**. `cargo xtask ci` being green is a precondition for *looking* at the
  exit criteria, never one of them.
- Settle a port against something that **compiles**, not against an argument.
- **A port frozen against one storage shape is shaped like that shape.**

## How to use this

1. **This file is updated in the same commit as the work it describes.**
2. **A phase is done when its proof artefact exists in the repository and every
   exit criterion is ticked.** Not when the gate is green.
3. **The specification is current truth; ADRs are history.** Changing a
   `[FROZEN]` clause takes a new ADR, not an edit to the specification. The
   [ADR queue](#the-adr-queue) lists them in write order, one question each.
4. **A `decided` ledger row is not settled.** It records an agreed answer with no
   ADR yet. A row whose answer the specification now *binds* cites the clause ID
   instead, and stays in the table — nothing is deleted from the ledger.
5. **If a phase body and the specification disagree, the specification wins**, and
   the phase body is fixed in the same commit.

## Session protocol

```
1. Read the status table, then `git log --oneline -10`.
2. Run `cargo xtask ci`. Establish the baseline is green before touching anything.
3. Pick the first phase that is not `done` and whose dependencies are `done`.
4. Write the phase's ADRs first. Then the code they constrain.
5. Build the phase's proof artefact. If you cannot, the phase is not done —
   say so in the session log rather than ticking the box.
6. Reconcile the specification against the code the phase just wrote. Not the
   MUSTs — the prose and citations underneath them. See
   [Between 5 and 6](#between-5-and-6--the-reconciliation-nothing-owned) for
   what skipping it cost.
7. Re-run the gate. Tick the exit criteria. Add a dated session-log line.
8. Commit the code and this file together.
```

## What the adversarial pass changed

Recorded rather than silently applied, because a plan that has been corrected and
does not say so teaches the next reader to trust it more than it has earned.

**Dependency inversions.** Phase 8's dependency row omitted phase 7 while the
prose and the diagram both required 7 before 8 — a reader following the session
protocol would have started the flagship adapter before the consumer that
discovers contract defects, which is the exact sequence this plan exists to
avoid. The critical-path diagram routed phase 3 into phase 6 when the table makes
phase 4 depend on it. Phase 2's projection-store skeletons could not be written at
all without borrowing phase 6's batch-shape decision, and nothing said so. Phase 3
writes three rules whose *content* — not merely their spelling — comes from phases
4 and 5.

**Decorative proof artefacts.** Four of the fifteen would have existed unchanged
if the design under test were wrong. Phase 4's was the worst: a skeleton whose
bodies are all `todo!()` compiles against *any* signature, because `todo!()` has
type `!` and `!` coerces to everything, so "the six skeletons compile" is
satisfied by a wrong freeze exactly as readily as by a right one. Its second half
was phase 3's exit criterion re-used, which leaves phase 4 with no proof of its
own. Phase 2's was a Markdown document, and a document that records outcomes
exists whatever the outcomes are. Phase 1's did not discriminate the question
ADR-0008 asks, because both flavours' *bare* half is identical whether the second
is derived or hand-written. Phase 9's ES-6 criterion had no artefact at all.

**Unsatisfiable criteria.** Phase 6 required the Ladybug and Postgres skeletons to
"compile unchanged" while itself dropping the `Batch` lifetime those skeletons
declare. Phase 4 required the same of all six.

**Orphans.** `cargo xtask spec-trace` — the only conformance rule CF-35 through
CF-38 name, and a tool this file cited twice as a live gate — was owned by no
phase. The `[PROVISIONAL]` clauses had no ledger, so phase 12's audit of them was
unauditable. The blocked-case table named E2E-37, which is not blocked, instead of
E2E-38, which is. E2E-25's runner half, the two missing cases for VT-17 and WF-12,
the clause amendments that retire the suite's two orphaned rules, and the removal
of PS-32/PS-33/PS-35 from the clause space were each specified somewhere and
scheduled nowhere.

**Clean on inspection, recorded so it is not re-derived.** The ADR queue collides
with nothing on disk — 0001–0007 exist, 0008–0028 are free, and ADR-0002 and
ADR-0005 already carry superseded statuses, so phase 0's rename does not orphan
them. All eighteen ledger rows from the previous runbook (`git show
HEAD:RUNBOOK.md`, rows at `:61-80`) are either carried or resolved against a
clause ID; the three SQLite rows and the three Ladybug rows were merged rather than
dropped. All thirteen `[DEFERRED]` clauses match the specification's maturity
column exactly, and every one names an owning phase. All 56 E2E cases are claimed
by at least one phase's *"Cases this makes writable"*. All thirteen defects D1–D13
have a work item.

**What a human still owes an answer to.** Three questions stood here; all three are
now settled, two of them by withdrawal rather than by answer. *Can phases 4 and 5 run in parallel?* — the phases were re-split
along the seam that actually exists, so the question stopped existing with it; see
the estimate section. *Is ES-24 a deferred clause on a published surface?* — it
was, and it is now settled at phase 4, because the guarantee it was waiting for
turned out to be one DCB already provides.

1. **Nothing, for now.** The three questions this section carried are settled —
   the phase 4/5 split, ES-24, and who wins when `spec-trace` disagrees with §7.2.
   The last is worth recording because the answer generalises: **the checker is
   authoritative on the five mechanical facts and §7.2 becomes its output; §7.3 –
   §7.6 stay authored, and a clause may `Retires:` a rule it deliberately leaves
   unclaimed.** The first run's disagreements are a defect list, not a
   constitutional question, and they should be expected to be real.

   This heading stays, because a plan of this size that reaches zero open
   questions has usually stopped looking rather than finished.

---

## Status

| # | Phase | Depends on | State | Proof artefact |
|---|---|---|---|---|
| 0 | [Ground clear](#phase-0--ground-clear) | — | done | a `.crate` that contains its licences and README, a README compiled by CI, three owned names, and `cargo xtask spec-trace` failing on a deliberately broken clause |
| 1 | [The `!Send` proof](#phase-1--the-send-proof-and-the-derivation-decision) | 0 | done | one provided body that type-checks under both flavours at once, two error shapes that disagree, and every rule green against a `!Send` store on `wasm32` |
| 2 | [The instrument portfolio](#phase-2--the-instrument-portfolio) | 1 | done | six crates compiling on their real targets with real associated types — no `Error = ()`, no stubbed stream — and three named signature attempts, each with its compiler error or its compiling call site |
| 3 | [The suite becomes an instrument](#phase-3--the-suite-becomes-an-instrument) | 1 | done | the mutant registry: every rule has a mutant that fails it, and every mutant fails exactly its declared rules |
| 4 | [Freeze the contract](#phase-4--freeze-the-contract-signatures-value-types-and-identity) | 2, 3 | **done** | `frozen_signatures.rs` — a generic consumer returning a read stream from a function, holding one in a struct, spawning a replay under the `Send` flavour, and keeping its batch after appending it; plus a `compile_fail` doctest pinning the arrangement that does **not** compile. Two of the four cases turned out not to fail pre-freeze, and the criterion says which and why |
| 5 | [Freeze the wire format](#phase-5--freeze-the-wire-format) | 4 | **done** | two `wire.rs` files — 21 tests and four const assertions across `happenstance-core` and `happenstance-sync`, every envelope shape round-tripping in JSON *and* postcard, sparse shapes included, each postcard round trip framed against a trailer so a field-count desynchronisation reports as a wrong value rather than as a short buffer; plus three negative controls that fail without the fix and are asserted **by name** in `xtask/src/proof.rs`. **Floats: anywhere between phase 4 and phase 12** |
| 6 | [Freeze `ProjectionStore`](#phase-6--freeze-projectionstore) | 4 | done | `CheckpointOnlyStore` **failing** the projection suite, and two unlike batch shapes passing it |
| 7 | [The typed layer and the example](#phase-7--the-typed-layer-and-the-worked-example) | 4, 6 | done | a `trybuild` compile-fail case: add an event variant, the crate stops compiling until the fold handles it |
| — | **`0.2.0-alpha.1`** | 7 | — | — |
| 8 | [`happenstance-sqlite`](#phase-8--happenstance-sqlite) | 4, 6, 7 | done | the concurrency macro green at 64 contenders, and an acknowledged write surviving a process reopen |
| 9 | [Cloudflare Durable Object](#phase-9--cloudflare-durable-object) | 2, 4 | done | every rule green under `workerd`, and a real `worker::Error`-carrying error type that either loses information the caller needs or demonstrably does not |
| 10 | [Postgres and Neon](#phase-10--happenstance-postgres-and-happenstance-neon) | 2, 4, 6 | in progress | the concurrency macro green on a store that does **not** serialise its writers, with the visibility cost measured |
| 11 | [Ladybug projection store](#phase-11--ladybug-projection-store) | 6 | not started | the projection suite green on a non-SQL batch, and a written verdict on whether phase 6's freeze held |
| 12 | [**Publish `0.2.0`**](#phase-12--publish-020) | 7, 8, **10** | not started | docs.rs green under `--all-features` and the `docsrs` cfg; `cargo-semver-checks` reporting against a registry baseline |
| 13 | [`happenstance-sync`](#phase-13--happenstance-sync-and-its-testkit) | 5, 8, 9, 10, 12 | not started | one suite green against three peers, two of them unlike, and a byte-identical payload round trip |
| 14 | [Retention and completeness](#phase-14--retention-deletion-and-completeness) | 13 | not started | a store that holds only a suffix of its own log, and a runner that fails loudly against it |

State is one of `not started`, `in progress`, `blocked`, `done`. Edit it in
place.

### The pre-publication remediation, and what it did to phase 12

Not a phase, and deliberately not given a row above: it is a review of everything
phases 0–9 built, and it gates phase 12 rather than sitting beside it.

`references/evaluation/review-pre-publication-2026-09-03.md` raised **96 finding
IDs across 84 entries** against `56ef6c5`. On branch `remediation/pre-publication`,
**65 are closed with the full gate green** and **48 briefs await ratification** in
`.kb/_intake/remediation-2026-09-04-briefs/`. One finding, `F2-5`, is
genuinely blocked, and narrower than the audit filed it: half of it was refuted
during the work — the rule's second assertion **does** execute, twice per run —
and what survives is that it has never been answered by a store with a real
medium under it, which is the phase-10 adapter that does not exist. Five decisions
were ratified by the owner during the work, including publishing the repository and
closing `ANCHOR_SLACK`.

**What it changes about phase 12's exit criteria, which are stated below at
`Phase 12 — Publish 0.2.0`.** Those criteria audit every `[PROVISIONAL]` and
`[DEFERRED]` clause on a published surface and ask nothing about whether a
`[FROZEN]` one is *met by the adapter being published*. This remediation found
four cases where it was not — `Q-01`, `X-3`, `Q-02` and `F2-5` — with the disproof
already in the tree in every case, written by the adapter's own author. Whoever
takes phase 12 should read that as a gap in the criteria rather than as four
findings that happen to be closed now.

The other thing it changes is what "the gate is green" is worth. Several checks
this runbook relies on were found to be reporting about themselves rather than
their subject; they are repaired, and `REMEDIATION-HANDOVER.md` lists them. The
scoped-gate command set has grown by four commands, each added because something
went red downstream of a gate that could not see it.

**`REMEDIATION-HANDOVER.md` is the entry point.** It carries the scoreboard, the
method, the briefs awaiting ratification, and the operational traps — including
the three false "completions" that were read as results before being caught.

### The critical path

Drawn as an edge list rather than as box art, because the previous drawing routed
phase 3 into phase 6 while the table above makes phase 4 depend on it, and nobody
noticed for a whole document revision.

```
0 ─▶ 1 ─▶ 2 ─▶ 4 ─▶ 6 ─▶ 7 ─▶ [0.2.0-alpha.1] ─▶ 8 ─▶ 10 ─▶ 12 ─▶ 13 ─▶ 14

one branch that rejoins the trunk:
    1 ─▶ 3 ─▶ 4          phase 4 is frozen against the instrument phase 3 builds

one that floats — after 4, before 12, otherwise unconstrained:
    4 ─▶ 5               the wire format; nothing between it and 12 reads it

two that never rejoin — off the 0.1 path:
    2, 4    ─▶ 9
    6       ─▶ 11
```

**Serial and unavoidable: 0 → 1 → 2 → 4 → 6 → 7 → 8 → 10 → 12**, with 3 on it too
unless a second pair of hands takes it. Phase 3 depends only on phase 1, so it
*can* run alongside the trunk — but it is not optional and cannot be skipped,
because 4 waits on it. Solo it is serial and the estimate below says so.

**Phase 5 is the exception and the only one.** It depends on phase 4 and nothing
depends on it before phase 12, so it is neither on the trunk nor a branch that
rejoins — it is three days that can be spent whenever three days are available.
Treat it as the buffer, because it is the only one this plan has.

| # | Phase | Days | Solo cumulative |
|---|---|---|---|
| 0 | Ground clear | 2 | 2 |
| 1 | `!Send` proof | 4 | 6 |
| 2 | Instrument portfolio | 5 | 11 |
| 3 | Suite as instrument | 6 | 17 |
| 4 | Freeze the contract | 10 | 27 |
| 5 | Freeze the wire format | 3 | 30 |
| 6 | Freeze `ProjectionStore` | 6 | 36 |
| 7 | Typed layer | 8 | 44 |
| — | **alpha** | — | **≈ 9 weeks** |
| 8 | SQLite | 10 | 54 |
| 12 | Publish 0.2.0 | 2 | 56 |

Phase 0 gained a day for `cargo xtask spec-trace`, which nothing was building and
two later phases were already citing as a gate.

**≈ 11 weeks of focused solo work to 0.1.** With a second pair of hands, ≈ 10.

An earlier revision offered ≈ 8.5 weeks conditional on phases 4 and 5 running in
parallel, and asked the reader to decide whether they could. They could not: both
edited `Event`, `SequencedEvent` and `append`, and phase 5's own body said the two
"land as one contract freeze". The question was a symptom of a bad split rather
than a real scheduling choice, so the split was moved instead of the question
answered. Phase 4 now carries the whole contract freeze — signatures and the
values those signatures carry — and phase 5 is the wire format alone.

That is worth more than the week the parallelism would have bought, because
**phase 5 leaves the critical path entirely.** Nothing between it and publication
consumes the wire format: `serde` is off by default, no adapter touches it, and
phases 8 through 11 do not mention it once. It must land after phase 4 and before
phase 12, and anywhere in between is equally correct. One phase on this plan can
now absorb a bad week without moving anything else.

No comparison against the old plan is offered, because the old plan carried no
estimate anywhere in its 529 lines (`PRESSURE-TEST.md:401-407`) and the two 0.1s
are not like-for-like.

Post-0.1: phases 9 (8 days), 10 (11), 11 (6), 13 (12) and 14 (5). Phases 9, 10
and 11 touch disjoint crates and parallelise freely. Phase 13 does not — it needs
three real stores, because proving a port takes two unlike implementations and an
oracle.

**What must not be parallelised.** 1 before 2: the skeletons are written against
whatever ADR-0008 decides, and writing them first means writing them twice.
2 before 4: the skeletons *are* the evidence, and a freeze written before them is
a freeze written on intention. 3 before 4: every semantic clause phase 4 freezes
is paired with a rule, and a rule written after the signature it protects is
written by someone who already believes the signature is right. **4** before 8, 9
and 10: `EventId` and `recorded_at` are columns in migration 1 of every store, and
they are settled in phase 4 now rather than phase 5 — this constraint used to name
phase 5 and was the clearest evidence that the identity work was in the wrong
phase, since it bound three adapters to a phase whose other half nothing needed. 7
before 8: the typed layer is the consumer that discovers contract defects, and
discovering them after the flagship adapter is written is the sequence this plan
exists to avoid — which is why phase 8's dependency row names 7, where the
previous revision listed only 4, 5 and 6 and left the prose to carry a constraint
the session protocol reads out of the table.

---

## The ADR queue

In the order they must be written, numbered **from 0008** —
`.kb/decisions/0007-projection-runner-decodes.md` is accepted on disk and dated
2026-08-06, and both prior planning documents allocated 0007 a second time
(`PRESSURE-TEST.md:254-262`). Each line is the single question that ADR answers;
an ADR that cannot be stated as one question is two ADRs.

Checked against `.kb/decisions/` at this revision: 0001–0007 exist, 0008–0028 are free,
and no queued number collides with one on disk. ADR-0002 and ADR-0005 need no
entry here despite phase 0 executing a rename that invalidates their content —
both already carry a superseded status (`0002:3`, `0005:3`), which is what makes
them history rather than wrong instructions.

Two numbers are ordered by convenience rather than by dependency. **0009 (phase 2)
and 0010 (phase 3)** sit on branches that do not depend on each other, so either
may be written first; and **0009 may not be written at phase 2 at all** — that
phase's exit criterion permits ES-6 to be restated as deferred with a compiled
reason, in which case the number is held until phase 9. A reserved number that
stays empty for seven phases is the cost of ES-6 being genuinely open, not a
scheduling defect.

| ADR | Phase | The question it answers |
|---|---|---|
| ~~**0008**~~ | 1 | ~~Is the second trait flavour derived by `trait_variant` or hand-written — for `EventStore` **and** `ProjectionStore` in one decision (PS-35) — given that a provided body is cloned into the variant and must type-check under both flavours' bounds at once?~~ **Written**, as [ADR-0008](.kb/decisions/0008-one-derivation-for-both-ports.md). The first half of the question was stale on arrival — ES-1 is `[FROZEN]` on "MUST be derived" — so the ADR answers the second and third halves and says so |
| ~~**0009**~~ | 2 | ~~Does the store's `Error` associated type carry `Send + Sync + 'static`, and may the two flavours differ in it? (ES-6)~~ **Written**, as [ADR-0009](.kb/decisions/0009-error-send-sync.md). No to the first, no to the second — and *may they differ* turned out not to be a policy question: there is no mechanism, which one edit to one declaration demonstrated by reporting against both flavours. The strength moves to a marker trait that works from downstream, so the contract crate need not change |
| **0029** | 2 | *(unscheduled — the queue had no number for it)* What is the MSRV, now that a dependency's build script forces the question? [ADR-0029](.kb/decisions/0029-msrv-raised-to-1-97-1.md), amending ADR-0004: **1.97.1** |
| ~~**0010**~~ | 3 | ~~What is the conformance suite's own proof obligation — what must every rule be demonstrated to fail, what shape must the fixture take, and how are rules emitted for runtimes that are not tokio? (CF-1 – CF-29)~~ **Written**, as [ADR-0010](.kb/decisions/0010-the-suite-must-prove-itself.md). The third question was already answered by phase 1's registry and is ratified rather than decided; the first two are the phase's work |
| **0011** | 4 | What does `read` promise about laziness and isolation — when is the store's state sampled, and do the items of one `Query` share one sample? (ES-11 – ES-13; **written**, and it also discharges ES-8, ES-9, ES-14 – ES-16 and VT-26 – VT-31's read half, which the scope below did not name) |
| **0012** | 4 | What shape does `append` take and what are its preconditions — who owns the batch, what an empty batch is, whether a batch can violate its own condition, and what a dropped future may have done? (ES-17 – ES-24; **written**, and it also discharges ES-25 – ES-29, ES-37 and VT-30) |
| **0013** | 4 | What does a store promise about position assignment and visibility — gaps, reuse, and the invariant that makes `AppendCondition::after` sound? (VT-11 – VT-13, ES-10, ES-38; **written**, and it also discharges ES-30 – ES-32's head/count questions, ES-35 and ES-40) |
| **0014** | 4 | What does an event carry beyond type, data and tags — identity, store incarnation, recorded time — and who assigns each? (VT-4 – VT-10; **written**, and it also adds ES-41 and adopts **VT-2**, which this queue assigned to nobody) |
| **0015** | 4 | How is a validated identifier constructed, and is `Tag` equality byte equality? (VT-14 – VT-25; **written**) |
| ~~**0016**~~ | 5 | ~~What is the wire format, and whose format is it? (WF-1 – WF-12)~~ **Written**, as [ADR-0016](.kb/decisions/0016-the-wire-format.md), and accepted. The format is **private to happenstance**, which is what makes every reversal in it free rather than breaking. WF-1's interoperability half stays `[DEFERRED]` on a *stronger* reason than the one its marker gave. The ADR adds no clause, removes none and moves no marker — D12's fix gets an xtask manifest lint rather than a WF-13 (§14), so §1.3's census is unchanged |
| **0017** | 6 | What does a projection batch own, what vocabulary writes into it, and what happens when it is dropped? (PS-4 – PS-15) |
| **0018** | 6 | How is a projection returned to "never run", what is that operation's transactional scope, and what may refuse it? (PS-16 – PS-20) |
| **0019** | 6 | What happens when `apply` fails? (PS-26 – PS-30) |
| **0030** | 6 | *(unscheduled — the queue had no number for it)* Which clause states that a successful `commit` advances the checkpoint, given that four rules already enforce it and PS-1's `MUST` is a coupling rather than a progress obligation? [ADR-0030](references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md): **a clause of its own, PS-38**, minted `[PROVISIONAL]` in §4.7. PS-1, PS-19, PS-21 and PS-22 are byte-identical across it |
| **0020** | 7 | How does a decision model guarantee that its query and its fold cannot disagree? |
| **0021** | 7 | How does a payload's shape evolve — codec tag, versioned event types, upcasting, and does the read path need a hook it does not have? |
| ~~**0022**~~ | 8 | ~~SQLite: driver, schema, tag storage, and the append-condition strategy.~~ **Written**, as [ADR-0022](references/adr/0022-append-condition-strategy.md), and staged for ingest at `.kb/_intake/0033-adr-0022-append-condition-strategy.md`. The **driver** half was stale on arrival — `rusqlite` without a pool was already settled at `crates/happenstance-sqlite/src/lib.rs:47-53` — so it is ratified rather than decided, in the shape row 0008 already uses. The rest is one question with seven consequences: the append condition is a `max(position)` guard inside `BEGIN IMMEDIATE`, which **reverses the architecture brief's recommendation on a measurement** (23 µs against the `EXISTS` probe's 32 and the conditional insert's 45 on the rejection path over a 5,000-event log; 213 against 311 and 306 over 50,000); tags go in `event_tag(tag, position)` with `event_type` covering (a selective read 3.16x and 4.63x faster than a canonical blob and JSON1); three pragma values; the runtime seam; `index_arms()` rejected. Measured in `experiments/append-condition/`, out of the workspace and out of the gate, because `append` is still `todo!()` and AC-013 puts the record first. Two non-verdicts are fenced with owners — **ES-17 is not lifted**, and the gap is escalated on the row below |
| **—** | 8 → ? | *(escalated by ADR-0022 §13, and owned by nobody yet)* ADR-0012 names phase 8 as the measurement that could lift `append`'s `&[Event]` marker, and its falsifier item 1 requires *"two builds of the **same** SQLite adapter differing only in `append`'s ownership, measured on the same harness."* Three candidate stores in an experiment crate are not that, and no story in phase 8's map produces it — the four implementation stories build **one** adapter. Items 2 to 5 are also unproduced, and item 5 is a *design* obligation (*"a cheap way to keep a copy for retry"*) no measurement alone supplies. This needs a number and an owner, or an explicit deferral with a new phase; what it must not have is a silence |
| ~~**0023**~~ | 9 | ~~Cloudflare: the `SqlStorage` mapping and the off-tokio conformance harness.~~ **Written**, as [ADR-0023](.kb/decisions/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md), and accepted. It is one question with two halves and **one body of evidence settles both** — the conformance suite executing against the real `worker` bindings, off tokio, inside one `cargo xtask ci` — so the mapping holds every binding `!Send` behind an `Rc` and retains the thrown value rather than stringifying it (which is what makes ADR-0009's ES-6 prediction hold rather than break), and the harness's shape is recorded as `every-rule-under-workerd`'s **finding** rather than chosen here: `wasm32-unknown-unknown` under `wasm-bindgen-test-runner`, one row in the executed-target registry, and **not** the `vitest-pool-workers` CI job this row used to queue, which loses to *the same run as the rest of the gate* (`references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md:157`). Amendment **ADR-0023-A** then moved nine acceptance sentences at three grains onto what actually executes, and ADR-0001's `provisional` marker is cited and **not** retired, because phase 1 already lifted it |
| **0024** | 10 | Postgres: how does the adapter buy the position-visibility invariant when `nextval()` allocates outside the transaction — measured, not preferred? |
| **0025** | 11 | Ladybug: checkpoint placement, how a projection expresses graph mutations, and the blocking API. |
| **0026** | 13 | What is a sync *peer* — what may the port assume about a transport it cannot see, and what does ingest promise? (SY-8 – SY-18) |
| **0027** | 13 | How do two logs reconcile — the merge rule, the compensation contract, and whether hub-and-spoke and peer-to-peer are one abstraction or two? (SY-1 – SY-7, SY-19 – SY-31) |
| **0028** | 14 | What is a store permitted to forget, and how does it say so? (ES-39, CF-27, SY-32) |

**The parenthesised clause ranges are a scope statement, and phase 4 proved they
are not a coverage guarantee.** Phase 4's body says it discharges ES-8 – ES-40
and VT-1 – VT-31 — 64 clause IDs — while its five queue rows named 35. The
missing 29 were found by a coverage audit at the end of the ADR pass, not by any
gate: `spec-trace` cannot see this, because every one of those clauses already
exists, already carries a marker and already names a rule, so nothing is
dangling. A clause owned by no ADR is invisible in exactly the way a clause that
is already finished is invisible.

Audited clause by clause against the five written ADRs, most of the 29 turned out
to be covered anyway — the rows above were stale rather than the ADRs short, and
they now say what each ADR actually discharges. Four IDs were mentioned by no ADR
at all, and they split two ways:

- **ES-33, ES-34 and ES-36 need nothing.** All three are `[FROZEN]` and all three
  already have passing rules from phase 3 —
  `two_handles_observe_each_others_appends`,
  `interleaved_appends_on_one_handle_elect_one_winner` and
  `a_live_read_stream_does_not_block_an_append`. The phase-4 work item that asks
  for ES-34 to "become a clause with a rule" is describing work that is done.
- **VT-2 was a genuine hole**, and the most expensive kind: `[FROZEN]`, with a
  rule (`appending_equal_events_yields_two_events`) that does not exist, and
  unwritable before this phase because its third conjunct is about `EventId`.
  Adopted by **ADR-0014 §9**, with the mutant that fails it.

**The rule this leaves behind, for every later phase:** a phase's clause range and
the union of its ADRs' clause ranges are two numbers, and nothing checks that they
are equal. Compute both at the phase's exit.

**Amendments to accepted ADRs, scheduled.**

- ~~**ADR-0001** loses `provisional` at phase 1 — or is superseded by ADR-0008. Its
  full proof (a real `!Send` adapter) arrives at phase 9 and is cited there.~~
  **Done at phase 1.** Not superseded: ADR-0008 *extends* it, since ADR-0001's
  reasoning about `EventStore` is untouched and what was missing was the second
  port. The banner is rewritten rather than deleted, so the record still shows
  what the marker meant and what took it off.
- **Four amendments to `SPECIFICATION.md` are owed by ADR-0008** and are listed
  in its own closing section: ES-2's named rule is necessary but not sufficient,
  ES-3's impl-site argument is false, ES-5's cited mechanism is incomplete, and
  PS-36's `compile_fail` doctest cannot be pinned as specified — the diagnostic
  carries no error code and rustdoc on 1.97.1 silently ignores the annotation.
  Three touch `[FROZEN]` clauses, which is why they are an ADR's output rather
  than an edit. **None changes a normative MUST**; all four are corrections to
  citations and to rule adequacy. Applying them to the document is scheduled at
  **phase 3**, which owns the rule registry's spellings and is already opening
  §6 and §7 — doing it there costs one pass instead of two.
- **Phase 2's wave falsified fourteen claims in `SPECIFICATION.md`**, and they
  join ADR-0008's four in the same phase-3 pass for the same reason. These are
  **not** line-number rot — that was fixed in phase 2's own commit, fourteen
  citations re-anchored — they are sentences whose *subject* stopped existing.
  Grouped by cause, with the phase that owns the new words:
  - **§4's "the port has zero implementers"** and its three supporting greps.
    There are five `ProjectionStore` impls. Phase 6 needs this correct before it
    freezes anything.
  - **ES-7's "the bare flavour has zero implementers"**, contradicted by four —
    `happenstance-cloudflare`, two in `happenstance-neon`, and phase 1's
    `LocalMemoryEventStore`. ES-7 is `[PROVISIONAL]`; whether that survives the
    corroboration is phase 4's.
  - **ES-30's "the cost of *required* is two impls today."** There are seven.
  - **ES-6's "`SqliteEventStoreError` is a single placeholder variant."** It is
    seven real variants. The clause is settled by ADR-0009 regardless, but the
    argument quoted in it no longer describes the tree.
  - **§5's account of `happenstance-sync`** as "a doc comment and a one-variant
    enum", and the *"None of this is settled"* sentence it quotes, which was
    deleted. Two ports in two flavours now exist.
  - **Three clauses rejecting a metadata-borne identity** (VT-3, VT-5, SY-12's
    neighbourhood) whose named wrong implementation was `happenstance-sync`'s own
    proposal. The crate now proposes `(StoreId, SequencePosition)` — what the
    clauses mandate — so the rejection has no exemplar and needs a new one, or an
    honest note that the workspace no longer contains a wrong answer to point at.
  - **Two clauses framing ingest as re-checking conditions.** The crate now takes
    the opposite position explicitly: the origin's condition travels as evidence,
    not as an instruction. Phase 13's, but the words are wrong now.
  - **Two deleted premises** — the ordering sentence and the transport-floor
    phrase — quoted by §5.4 and its neighbours.

  **The checker cannot catch any of this**, and that is worth stating where the
  next person will look: `check_citations` validates that a file exists and that
  a range's first number is within it, so a citation goes on passing when the line
  it names has come to say something else. All eighteen amendments would have sat
  green indefinitely.
- **ADR-0003** loses `provisional` at phase 13, when a payload round-trips
  byte-identically between two peers.
- **ADR-0004** loses `provisional` at phase 12, when the MSRV becomes a promise.
- **ADR-0006** is *executed* at phase 0 and gains an "On the historical record"
  section saying that ADR-0001/0003/0004 and CLAUDE.md's constraints are rewritten
  to `happenstance-core` because they were always statements about the ports crate.
  **Done at phase 0**, and the section enumerates **ADR-0007** too — leaving it out
  is what let a dead citation survive in it, so the list is the load-bearing part
  rather than the prose around it. The rule it states, for reuse: *rewrite the
  referent, never the reasoning.* A superseded body records a reversal and stays
  verbatim; a renamed crate inside a standing decision is not a reversal at all.
- **ADR-0001** and **ADR-0003** had their banners' *phase numbers* repointed at
  phase 0 under that same rule — Cloudflare is phase 9, not 5; sync is phase 13,
  not 6. The lifting *conditions* are untouched. ADR-0006's own three citations of
  "phase 3" are deliberately left wrong: this runbook **replaced** its predecessor
  rather than renumbering it, so no phase 3 became phase 7 and there is no
  referent to rewrite.
- **ADR-0007**'s Context is corrected at phase 6 (PS-32): a runner that itself
  writes into the batch cannot be written today; a callback-driven one can, and
  was compiled (`PRESSURE-TEST.md:203-234`). Its **falsifier** — *"if the
  checkpoint pump has acquired no independent caller by the time the typed layer's
  phase exits, collapse it upward and supersede"* (`0007:119-121`) — is evaluated
  at **phase 7's exit** and nowhere else (PS-33).

---

## Decision ledger

Every question that is deliberately open, the phase that owns the answer, and its
status. **Nothing is deleted from this table.** A row the specification now binds
cites the clause ID and stays, so that a reader who remembers the question can
find where it went.

`decided` means an agreed answer with no ADR yet. `settled — <clause>` means the
specification binds it; changing it takes a new ADR against that clause.

### Restored rows

These four were live in the previous runbook and were dropped by
`revised-runway.md`. Three of them own decisions phase 6 cannot exit without
(`PRESSURE-TEST.md:274-290`).

| Decision | Was | Phase | Status | ADR |
|---|---|---|---|---|
| Does the port grow an `apply` seam a generic runner can drive, or is applying an event adapter-bound by design | `RUNBOOK.md:68` | 6 | **settled — PS-9, PS-10, PS-11.** The port grows a write seam *because the suite needs one*: generic suite code holding a batch can only `commit` or `rollback` it, so three of the six projection rules cannot observe the read model at all, and a suite that cannot reject a store which commits the checkpoint and drops the write is decorative | 0017 |
| Read-model lifecycle: where DDL and migrations live, and how a projection is reset for a rebuild | `RUNBOOK.md:69` | 6 | **settled — PS-16 – PS-20.** Adding `reset` after the freeze is breaking, and a rebuild is the commonest thing anyone does to a read model. It is refusable, not merely discouraged | 0018 |
| Where the projection runner lives, and whether it hands the application decoded events | `RUNBOOK.md:72` | 7 | **settled — ADR-0007**, whose falsifier is evaluated at phase 7's exit (PS-33). The application-facing half — the `Projection` trait and the runner — is phase 7 work; the previous plans left it with no owning phase at all | 0007 |
| Projection failure policy: retry, skip, halt or dead-letter | `RUNBOOK.md:73` | 6 | **settled — PS-26 – PS-30.** It is not one policy; it is per projection, and there is a fourth option ("skip and record") that none of the original four covers | 0019 |

### The warning that came with them

> Deferring the sync port **leaks `EventId` and a tail seam back into
> `EventStore`** — `RUNBOOK.md:80`.

That is the exact coupling every deferral of replication assumes absent, and the
revised runway dropped the sentence while keeping the deferral. The specification
discharges it in three places, and the discharge is why phase 13 can sit after
publication:

- **Identity does not enter `EventStore`.** A foreign `EventId` arrives through
  `IngestStore`, a trait defined in `happenstance-sync` (VT-10, SY-8), not through
  `EventStore::append`. Coherence is what makes this work: the orphan rule says
  the crate defining a trait is the only crate that can grow it, so putting a
  replication-shaped method on the store port would oblige every store adapter to
  have an opinion about replication with no way to opt out.
- **The tail seam's absence is a clause, not an omission.** ES-32 states that
  there is no tail or subscription seam at 0.1 and that the runner polls. An
  absence that is written down cannot be added under pressure without an ADR.
- **The sync suite never reaches into the store port.** SY-8 keeps every sync rule
  in `happenstance-sync-testkit`.

**Residual risk, stated rather than assumed away.** If phase 13 discovers that a
real peer needs a store-side seam, that is a breaking change to a *published*
port. Two mitigations, both already scheduled: the `SyncPeer` sketch lands in
phase 2, nine phases early, precisely so this is discovered before the freeze; and
ADR-0026 must be written against two unlike peers, not one.

### Event store and value types

| Decision | Phase | Status | ADR |
|---|---|---|---|
| Derived vs hand-written `Send` flavour; `Self: Sync` at the point of use; one decision covering both ports | 1 | **settled — ADR-0008.** One scheme, both ports, derived — though "derived vs hand-written" was already `[FROZEN]` at ES-1 and the live question was PS-35's second port. The scheme is shared; the **provided-method budget is not**, because `ProjectionStore`'s GAT admits a body that cannot be made to compile on either flavour and `EventStore` structurally cannot exhibit it | 0008 |
| Does `Error` carry `Send + Sync + 'static`, and may the flavours differ | 2 | **settled — ADR-0009. No, and no.** The bound stays; the strength becomes a marker trait, which compiles *from downstream*, so the contract crate need not change. "May the flavours differ" was never a policy question — one edit to one declaration reported against both, which is ES-5 observed. What actually decided it was that the derived flavour does not imply a `Send` error either, so ES-6's named rule was **unwritable for every adapter**: a deferral behind an impossible experiment | 0009 |
| Does `EventStore` grow `head` / `count`, and as provided or required methods | 4 | **settled — ES-30, ES-31.** Provided, hand-desugared, overridable. The claim that a provided method could never be added was compiled and refuted | 0012 |
| `read` laziness, isolation, and whether one `Query`'s items share one snapshot | 4 | **settled — ES-11, ES-12, ES-13.** The contract promised laziness that the reference store does not provide (D7) | 0011 |
| `append` ownership, empty batch, self-conflict, cancellation | 4 | **settled — ES-17 – ES-23** (D8) | 0012 |
| Whether a reissued batch after a dropped `append` future lands once | 4 | **settled — ES-24.** A conditional append whose events match its own condition is at-most-once under verbatim reissue; the retry's `ConditionViolated` is the answer. Unconditional appends and conditions that do not match their own events get no guarantee, and a rule pins each | 0012 |
| Position visibility invariant; reuse after removal | 4 | **settled — VT-12, ES-10, ES-38.** The far end is phase 10 | 0013 |
| `conflicting_position`: a promise every adapter owes, or a hint one may omit | 4 | open — Neon-over-HTTP is the forcing case, because it has no interactive transaction and so cannot probe and write separately | 0012 |
| Event identity across instances | 5 | **settled — VT-4 – VT-10.** `EventId` is a store-assigned `(StoreId, SequencePosition)` on `SequencedEvent`; `StoreId` names a store *incarnation*, never a device and never a peer, so a restored backup cannot reissue identities | 0014 |
| A store-assigned time on `SequencedEvent` | 5 | **provisional — VT-9** | 0014 |
| Const-constructible `EventType` / `Tag`; Unicode normalisation | 5 | **settled — VT-14 – VT-20** (D4, D9). Doing nothing and saying nothing was the only option that was definitely wrong | 0015 |
| Store limits: payload size, tag count, query items, batch size | 5 | **provisional — VT-21 – VT-25.** Guaranteed minima the store MUST accept, plus an error variant for what it refuses | 0015 |
| Wire format: `Query::All`'s encoding, sparse shapes, versioning | 5 | **settled — WF-2 – WF-12** (D1 critical, D6). The format is **private to happenstance**; the `serde` feature moves events between happenstance instances and is not an interoperability surface. Two qualifications this row used to swallow: **WF-1 is settled in its scope half only** and its interoperability half stays `[DEFERRED]` on the row below; and **D12 is not phase 5's to close** — it was closed at phase 0 by `927d291` and phase 5 adds a manifest lint against its recurrence rather than a fix | 0016 |
| DCB wire interoperability | 13 | **deferred — WF-1**, and the reason changed at phase 5. The named home is still ADR-0026's envelope section, but the deferral no longer waits on *"a bridge exercised against the DCB reference implementation's encoding"*: **there is no such encoding.** The DCB specification and its reference TypeScript library publish **no wire format at all** — `EventStore.ts` contains no serialisation code of any kind, and the specification's JSON snippets are labelled *"a **potential** JSON representation"* beside an explicit disclaimer that *"implementations are not required to use the same terms or function/field names"*. A bridge built today would be built against an illustration the specification disclaims, and the first real DCB peer would break it. **That makes the deferral stronger rather than weaker.** The surviving divergence is recorded rather than fixed: happenstance spells match-all as a value that cannot syntactically collide with a filtered query (`null` before phase 5, `"All"` after), while the reference spells it `{items: []}` — structurally identical to an illegal empty filtered query and distinguished only by which function built it | 0016, 0026 |
| Durability across a process boundary | 8 | **deferred — CF-14; provisional — CF-17.** Nothing in the workspace could express the question, because the fixture took one handle. **Phase 3 changed the fixture and went one step further than CF-14's deferral allows, deliberately**: `REOPEN` and `acknowledged_writes_survive_a_reopen` landed against `DurableFixture`, with `LosingFixture` failing it, because without a gated rule `capability_skips_are_reported` had nothing to observe and CF-17's `[PROVISIONAL]` marker was untested. The far end is untouched — nothing in the tree loses a write to a *fault* rather than to an instruction — so CF-14 stays `[DEFERRED]` and phase 8 is still the first store that can lose one. **Phase 9 has now re-read it against a real Durable Object fixture**; the reading is the CF-14 row of *The 10 `[DEFERRED]` clauses* below, and it does not move the marker | 0010, 0022 |

### Projections

| Decision | Phase | Status | ADR |
|---|---|---|---|
| Does the `ProjectionStore` port survive contact with a real transaction API | 6 | **settled — PS-4 – PS-8.** The borrowed GAT does not survive on the `Send` flavour; the batch becomes owned. Note that this does **not** make the foreign-batch hazard unrepresentable — that claim was compiled and refuted (`PRESSURE-TEST.md:179-201`): a lifetime names a region, not an instance | 0017 |
| Does `commit` reject a batch begun on another store instance | 6 | **provisional — PS-15.** Only a generative brand rejects the call; the owned batch does not | 0017 |
| Is a `Batch` read-your-writes within one chunk | 6 | **provisional — PS-12** | 0017 |
| Ship the projection port behind `unstable-projection` at 0.1, or freeze it | 6 | **provisional — PS-3.** The honest option if the two batch shapes disagree, and it decouples publication from this phase | 0017 |
| May `commit` name a position no applied event occupies; may a checkpoint move backwards | 6 | **settled — PS-21; provisional — PS-22** | 0018 |
| Ladybug checkpoint placement; how a projection expresses graph mutations; `lbug`'s blocking API | 11 | open | 0025 |
| Do projections poll, or does `EventStore` grow a tail/subscription seam | 9 | **settled for 0.1 — ES-32: absent, and stated as absent.** Phase 7's runner polls and phase 7 records the cost of N views × N reads; **phase 9 has now recorded its half** — the one-paragraph verdict is in phase 9's session log, and it says the seam is cheap to reopen post-0.1 *only* in the additive shape this clause already names. The previous plan named an owner whose work list had no item for it; both phases now carry one | — (post-0.1) |

### Sync

| Decision | Phase | Status | ADR |
|---|---|---|---|
| Does sync ingest re-check append conditions | 13 | **settled — SY-1 – SY-7. Ingest is unconditional, with compensation.** A replicated event is never refused for a reason that is a function of the receiving store's state; where a local condition would have been violated, the losing event and a domain-supplied compensating event land as one atomic append. Rejection is a function of local state, so a rejecting ingest never converges; a compensation is an append rather than a refusal, so no peer deletes a fact its user was told had landed | 0027 |
| Merge rule for two independently-ordered logs | 13 | **settled — SY-19 – SY-26; provisional — SY-10, SY-20 – SY-23** | 0027 |
| Is replication a port, or one protocol | 13 | **settled — SY-8, SY-9.** A port. `happenstance-sync` holds it, `happenstance-sync-testkit` holds its suite, peers ship as sibling crates, and one adapter can serve both roles | 0026 |
| Is replication whole-log or scoped | 13 | **deferred — SY-27, SY-28.** A spoke holding a filtered subset cannot distinguish "not yet received" from "filtered out", so a position-based resume watermark against a hub is unsound. This decides whether the round-trip rule can assert log equality at all | 0027 |
| Idempotent bulk ingest inside one round trip | 13 | **deferred — SY-14.** `append` takes one condition per batch, which is what makes a peer's unit of work decompose into N conditional appends | 0027 |
| Does a peer declare its own limits | 13 | **deferred — SY-18** | 0026 |
| Where a per-peer watermark lives transactionally | 13 | **provisional — SY-31** | 0026 |
| Hub-and-spoke as a first-class topology beside peer-to-peer | 13 | **settled — SY-9, SY-33, SY-34.** A hub may refuse a spoke, and a refused spoke retains its log; refusal is never condition-derived | 0027 |

### Lifecycle, and the two things that are neither

| Decision | Phase | Status | ADR |
|---|---|---|---|
| What is a store permitted to forget, and how does it say so | 14 | **deferred — ES-39, CF-27, SY-32.** A store that has been deleted from is currently indistinguishable from a young one at every value in §2, and four of the six scenarios reach that from unrelated doors. An explicit written refusal — deletion is out of scope for `EventStore`, and here is what a deleted-from store may look like — is a legitimate answer | 0028 |
| A projection's `Query` changed under its checkpoint | 6 | **provisional — PS-25** | 0018 |
| SQLite driver, append-condition strategy, tag storage | 8 | **decided, and one third of this row was wrong before it was measured.** `rusqlite` without a pool — ratified, stale on arrival. `BEGIN IMMEDIATE`, yes; a **`max(position)` guard**, not a probe, because `max()` answers *whether* and *by which event* in one statement where `EXISTS` needs a second query to name the conflict, and on the rejection path it measured 23 µs against the probe's 32 at 5,000 events and 213 against 311 at 50,000. Tags in `event_tag(tag, position)` `WITHOUT ROWID` with `event_type` covering — a selective read 3.16x faster than a canonical blob and 4.63x faster than JSON1, paid for with a 1.5-2.1x more expensive write. `tag_cardinality` is promoted from a nicety to a requirement: a two-tag boundary costs ~200x a single-tag one | 0022 |
| How a Postgres adapter buys position visibility | 10 | **measured at phase 2 — `xid8` + `pg_snapshot_xmin`, at 0.99–1.03× baseline and blocking nobody.** The other two are correct and 16×/30× slower at 64 writers; the cheap fourth (advisory locks keyed by tags) is cheap because it buys a *per-boundary* invariant where ES-10 states a global one. The mechanism is settled and its **structural** costs are not — `head` becomes a frontier, read-your-own-writes does not hold, and staleness is bounded by the longest write transaction anywhere in the cluster. ADR-0024 records the choice; phase 10 pays for it | 0024 |
| Is ES-10's global visibility statement what happenstance needs, or would a per-boundary one do | 4 | **open, and newly so.** Raised by the phase-2 measurement rather than by a reader: the per-boundary mechanism is nearly free and the global one is not. DCB evaluates conditions against a boundary, so the question is not rhetorical. It is a clause question, not a measurement, and it is phase 4's | 0013 |
| Benchmark harness | 8 | **decided, and landed early** — `event_store_benchmarks!` is on disk in `crates/happenstance-testkit/src/bench.rs` behind an off-by-default, target-gated `bench` feature, because ADR-0022 is its first paying customer. Adapters inherit it in one line. Not a conformance rule (CF-34): the rule-name count is byte-for-byte unchanged by its arrival, it carries its own enumeration, and it has no threshold at any budget. It reads **no clock** either (CF-33): the harness reports counts and the caller's emitter reports durations, which is the same seam CF-23 already makes a parameter | 0022 |
| A store holding only a suffix of its own log, as a testkit instrument | 14 | **deferred — CF-27.** The completeness axis has nothing at its far end | 0028 |
| Is `happenstance-macros` in scope for 0.1 | 7 | **no — measured at phase 7 and out.** The rewritten example is 40 lines of mapping ceremony against 249 of domain; even counting the whole contested identity block (`CourseId`/`StudentId`, 85 lines) as ceremony it is 0.50:1, and the threshold is 1:1. This **contradicts** the design's own 2.4:1 prediction, which was taken over a minimal doctest: the `DomainEvent` impl is a fixed cost that barely grows with the domain, so the ratio is a function of how much domain the artefact has. `references/evaluation/phase-7-macros-verdict.md` publishes the classification range by range. Reopen only if the `DomainEvent::tags` defect is settled with an infallible `Tags` path | 0020 |
| Snapshotting decision-model state | post-0.1 | deferred — DCB queries are narrow by construction; revisit if replay cost is measured | — |
| `tracing` spans and metrics | post-0.1 | deferred — purely additive, no port change | — |
| Is `happenstance-runtime` the right name and the right seam | 0 | **settled — ADR-0006**, executed in phase 0 | 0006 |
| Conformance harness for a `!Send`, non-tokio runtime | 1 | **settled — CF-20, CF-22, CF-23, CF-28.** A rule registry plus three emitters; no testkit restructure beyond that | 0010 |

---

## Where the open clauses and the blocked cases land

Three obligations, all mechanical to check, all previously answerable only by
prose search — and the middle one previously not answerable at all, which is why
phase 12's audit of it had nothing behind it.

### The 12 `[DEFERRED]` clauses

Seventeen rows are listed; four are struck. Two were settled at phase 2 and their
markers moved in the specification at phase 3; the third, CF-13, was settled at
phase 3 by running the experiment its own marker named; the fourth, PS-33, was
**demoted rather than settled** — §7.2 carries it as `NON-NORMATIVE`, so it is not
a deferral at all any more. **Twelve remain.** The struck rows stay because a
reader arriving from an older commit needs to find them, and because a table that
quietly loses a row cannot be checked against anything.

Three of the twelve — PS-18, PS-27 and PS-30 — are new to this table and were not
new to the specification. They sat in the **provisional** ledger below while §7.2
carried them as deferred, which is most of why that ledger's heading said
forty-nine against a specification that said forty-seven. They did not disappear;
they moved.

| Clause | What it defers | Owning phase |
|---|---|---|
| ~~ES-6~~ | ~~`Error: Send + Sync`~~ | **settled at 2 — ADR-0009**, and `[FROZEN]` in the specification since phase 3 |
| ES-39 | a store reporting history it does not hold | 14 |
| WF-1 | DCB wire interoperability | 13 |
| ~~PS-33~~ | ~~evaluating ADR-0007's falsifier~~ | **demoted at §7.2 to `NON-NORMATIVE`**, and therefore not a deferral. It was a clause about this document's own work rather than about a store's behaviour |
| ~~PS-35~~ | ~~one derivation ADR covering both ports~~ | **settled at 2 — ADR-0008**, and `[FROZEN]` in the specification since phase 3 |
| PS-18 | a refusable reset — `refused_reset_changes_nothing` | 6 — `projection-store-freeze` (HS-P0010), which takes the count when the first adapter over storage this workspace does not control clears the projection suite. Evaluated at the typed layer's phase exit and the count came back **unavailable rather than zero**: the mechanism and its rule both exist; no projection adapter has shipped to implement protection |
| PS-27 | "skip and record" atomicity on a projection failure | 6 — `projection-store-freeze` (HS-P0010), which owns both the suite rule and the port surface a skip record would be written through. The count is **zero**: the alpha's runner halts on the first failure and offers no failure-policy seam, so no path could write a skip record. **Not withdrawn** — the Kestrel Motor shred case still needs it |
| PS-30 | a panicking `apply` rolls back | 6 — `projection-store-freeze` (HS-P0010), once a fan-out runner is buildable at all. The runner is not built, so the MUST binds nothing today. Its contingency is a **benchmark rather than an assertion** and is deliberately outside the gate under CF-34; the harness now exists as `experiments/polling-cost`, which is the artefact that reopens this clause |
| SY-14 | idempotent bulk ingest in bounded round trips | 13 |
| SY-18 | peer-declared limits | 13 |
| SY-27 | scoped versus whole-log replication | 13 |
| SY-28 | scope preservation across a round trip | 13 |
| SY-32 | retention gaps reported rather than silent | 14 |
| ~~CF-13~~ | ~~a fixture that can *fail* the visibility rule~~ | **settled at 3**: `PreCommitPositionStore` fails `nothing_below_an_observed_position_appears_later` deterministically on one thread, the `Send + Sync` sub-trait the marker held in reserve was not needed, and the clause is `[FROZEN]`. The **adapter** far end stays open and is §6.5's position-allocation row, owned by phase 10 |
| CF-14 | durability across a reopen | 8 — the *fixture* half landed early at phase 3 as a named exception (`REOPEN`, `acknowledged_writes_survive_a_reopen`, `LosingFixture`). **Phase 9's reading: the deferral still holds, and one more of the three named implementations has answered with one shape** — `CloudflareFixture` expressed `REOPEN` through the same contract shape `MemoryFixture`, `DurableFixture` and `SqliteFixture` use, and `acknowledged_writes_survive_a_reopen` with its two neighbours ran and passed with **zero** `SKIP` in the wasm32 conformance run, so "durable" needed no grading. **The far end is untouched** — this project supplied no store that can lose an acknowledged write to a *fault* rather than to an instruction, so that half stays phase 8's and `sqlite-durable-store`'s (HS-P0012), and what the run could *not* exercise is in phase 9's session log, *The CF-14 re-read* |
| CF-27 | the suffix-store instrument | 14 — **phase 9's reading: more real, not less, and this runtime is why.** *Eviction* is the wrong hazard — a Durable Object's storage outlives its isolate — but the same `Storage` object this adapter reaches through carries `delete_all()` (`worker-0.8.5/src/durable.rs:449`), so a wholesale purge happens with no `EventStore` method involved and the store passes every rule unchanged afterwards, which is how the specification's **No, and nothing is planned** completeness row comes to understate the exposure by one adapter. The instrument that would settle it — a testkit-adjacent store holding only a suffix and reporting that it does — is **deliberately not built here**: it stays `retention-and-incomplete-logs`' (HS-P0018) and phase 14's, and the argument is in phase 9's session log, *The CF-27 re-read* |

Checked against `SPECIFICATION.md` §7.2's maturity column, and **the two sets are
equal again as of the `0.2.0` release pass**. `cargo xtask spec-trace` counts
**twelve** `[DEFERRED]` clauses — WF-1, ES-39, PS-18, PS-27, PS-30, SY-14, SY-18,
SY-27, SY-28, SY-32, CF-14, CF-27 — and this table now lists exactly those twelve.

Phase 9 read this and recorded the drift rather than repairing it, on the ground
that reconciling a maturity table is a clause question and a prose re-read of
CF-14 and CF-27 had no licence to move a row. That was right then and it is what
made the repair cheap now: the three projection clauses were **named** as missing
rather than merely absent, so this pass had only to move them, and each one's
falsifier and owner came from its own marker rather than from a judgement made
here. **No deferred clause listed here is unowned**, which is the property that
paragraph was written to protect.

**One of them sits on a surface phase 12 publishes**, and it is safe. WF-1 (DCB
wire interoperability) is owned by phase 13, after publication, but the format is
private and WF-8 puts a version first, so phase 13 can change it without a wire
break.

ES-24 was the second and is no longer deferred. It asked how a caller resolves an
unknown outcome after a dropped `append` future, and was deferred on the
assumption that answering it needed an identity `Event` does not carry. It does
not: a conditional append whose events match its own condition is already
at-most-once under verbatim reissue, because the retry's `ConditionViolated` *is*
the answer "it landed". The clause is now `[FROZEN]` at phase 4, with the two
shapes that get no such guarantee stated as limits and a rule pinning each. The
deferral was larger than the question.

### The 46 `[PROVISIONAL]` clauses

Phase 12 cannot audit "every provisional clause has its falsifier scheduled"
against prose. Grouped by what falsifies them, because they do not fail
independently — seventeen of the `PS` rows wait on the same missing adapter, and
counting them as seventeen open questions overstates the exposure by a factor of
seventeen.

| Group | Clauses | Falsified by | Owning phase |
|---|---|---|---|
| Identity's queryability and store-assigned time | VT-6, VT-9 | a peer that must dedupe without parsing `metadata`, and a rule that `recorded_at` is non-decreasing with position | 5, exercised 13 |
| The ingest seam's placement | VT-10 | a real peer needing a foreign identity through `EventStore::append` after all | 13 |
| Const-constructible identifiers | VT-14 | `from_static` failing to move the `?` count in the worked example | 5 |
| Store limits | VT-21 – VT-24 | a real adapter that cannot honour a stated minimum | 5, tested 8 and 10 |
| Per-item boundaries on a condition | VT-30 | E2E-04 and E2E-05 still unwritable after phase 4 | 4 |
| `Bytes`' human-readable form | WF-11 | a peer that cannot buffer a payload through any human-readable encoder | 9 |
| The `!Send` flavour and `append` ownership | ES-7, ES-17 | the Cloudflare adapter, and `dynosaur` failing to erase a generic `append` | 1 and 4, confirmed 9 |
| No tail seam at 0.1 | ES-32 | a Durable Object making one cheap enough to reopen | 9 (verdict), post-0.1 — **answered**; the one-paragraph verdict is in phase 9's session log. It answers *this* falsifier only: the clause's own benchmark-shaped one — E2E-32's fan-out runner holding N views within their staleness budget — is untouched and stays phase 7's |
| The whole batch shape and write seam | PS-4 – PS-6, PS-9, PS-11, PS-12, PS-15 | **PS-2 alone** — `CheckpointOnlyStore` passing, or a third adapter disagreeing with the two that froze it | 6, re-tested 11 |
| Reset, checkpoint regression, chunked rebuild, query drift | PS-16, PS-22 – PS-25 | a rebuild that skips event 1, or a projection that cannot refuse a reset | 6 |
| Ship behind `unstable-projection` | PS-3 | the two batch shapes disagreeing at phase 6 | 6, decided at 12 |
| The E0195 spelling trap | PS-34 | a third implementer hitting it after the diagnostic is documented | 6, re-tested 11 |
| Compensation's shape and the merge rule's details | SY-7, SY-10, SY-20 – SY-23, SY-29, SY-30 | two unlike peers that cannot both express it | 13 |
| Where a per-peer watermark lives | SY-31 | a peer with no transaction to put it in | 13 |
| Membership as a port operation | ES-41 | an adapter that cannot answer membership without a structure VT-8 does not already oblige. Phase 8 answered the **in-process, connection-holding** half — `happenstance-sqlite` reads the `UNIQUE (origin_store, origin_position)` pair migration 1 already creates. The **transport** half is open: a store with no connection, no interactive transaction and no cursor, for which the probe is a whole extra round trip | 9 or 10, whichever adapter lands first. Completeness has an instrument at neither end |
| Checkpoint visibility after commit | PS-38 | a store answering `checkpoint` from a replica that may lag its own `commit` — the shape a projection store over an eventually-consistent read model has. If real, the obligation narrows to "a subsequent read through the same handle" and every rule downstream gains a handle constraint | 7 — the first projection adapter over storage this workspace does not control |
| A fixture's fault-injection promise | CF-39 | a real adapter whose only injectable mid-batch fault is one its driver transparently absorbs — a connection killed mid-statement behind a reconnect-and-retry pool — which would make "the append returns `Err`" a promise no fixture over that adapter can keep | 8 and 10. **No adapter has armed a fault yet** |
| A fixture's stated capacity ceilings | CF-40 | a real adapter whose ceiling is **not a constant** — a Postgres row whose TOAST threshold moves with the rest of the row, or a KV store whose per-value cap moves with the key — for which a single `Option<usize>` cannot say where the boundary is, and the rule built on it would assert a number the store cannot honour | 9 and 10, **whichever states a varying ceiling first**. Phase 9 has landed and answered the *constant-ceiling* half — `CloudflareFixture` states all three as `Some(…)`, as `SqliteFixture` does — so it added a second constant-ceiling store and left the live half untouched. That half is **phase 10's**: Postgres is where a ceiling that moves with the row first appears |
| Durability's rule shape; benchmarks are not conformance | CF-17, CF-34 | a store that loses an acknowledged write, and an adapter that scans where it should seek and passes every rule | 8 |
| **The portfolio's residual exposure** — the clauses §1.3 names as carrying CF-25's risk in their own markers rather than in a preamble. **Four, not five**: ES-10 was lifted at phase 4 and `[FROZEN]` since, and carrying it here is what made this table's count disagree with the specification's | ES-11, ES-12, ES-35, ES-40 | the far-end **adapter** on each axis, and nothing short of it: transport (ES-11, ES-12) by a one-shot-HTTP store that self-paginates; durability (ES-35) by a store that can lose a write to a fault; completeness (ES-40) by a store holding a suffix. A **fixture** instrument does not falsify any of them — CF-26 says so in terms | 10 (ES-11, ES-12), 8 (ES-35), 14 (ES-40) |

Every group names a phase in the [status table](#status). **PS-2 is the single
gate under thirteen of these rows**, which is why phase 6 is worth its six days
and why phase 11's verdict on whether the freeze held is a result either way.

**The five in the last row were missing from this table until phase 3's close,
and the omission is the exact defect the table exists to prevent.** They are the
clauses `SPECIFICATION.md` §1.3 names, by number, as the ones holding the CF-25
exposure — the five phase 4's exit criteria have to cite — and a phase-12 audit
reading this table alone would have found forty-one falsifiers scheduled under a
heading that says forty-six and concluded that everything was owned. The heading
was right; the rows were short. Counted again at phase 3's close, group by group,
against `spec-trace`'s own list of `[PROVISIONAL]` clause IDs: the two sets were
equal **at that commit**.

**They were not equal, and the `0.2.0` release pass repaired it.** Phase 5
recounted and left the rows; phase 5's own note said the repair could not wait
past phase 12's audit, and this is that audit.

**Phase 5's arithmetic was wrong in both directions, which is why the heading
said 49 against a specification that said 47.** It read the delta as *phase 4
added CF-39, CF-40, ES-41 and ES-42, and lifted ES-10 — 46 − 1 + 4 = 49*. Counted
clause by clause against §7.2's generated maturity column, the real delta is
**46 − 4 + 5 = 47**: a fifth addition, **PS-38**, minted by ADR-0030 and listed
nowhere here; and three further removals, **PS-18, PS-27 and PS-30**, which were
not lifted at all — they moved to `[DEFERRED]`, where they are now listed, and
where phase 9 had already named them as missing.

Each new row's falsifier is the clause's own marker rather than a judgement made
here, and six of the eight rows that moved name their owner in the same marker:
PS-18, PS-27, PS-30 and PS-38 to `projection-store-freeze` (HS-P0010), ES-41 to
whichever of phases 9 and 10 lands first, CF-39 to phases 8 and 10.

**ES-42 was the eighth row and is no longer in this table**, because it is no
longer provisional. Its marker named a deadline rather than a phase — *"must be
re-evaluated before phase 12"* — which made it, while it was missing from here,
**the one clause in the specification whose own marker gated the release and
which a phase-12 audit reading this table could not have seen.** That is what
the repair was for, and the re-evaluation it forced ran immediately: no consumer
needs `dyn EventStore` the E11 erasure wrapper cannot box, so the marker was
**earned off rather than allowed to expire** and the clause is `[FROZEN]`. The
count above is 46 rather than 47 for that reason.

**Every row names an owner.** The first pass at this repair recorded CF-40 as
owed one, and that was an error in the repair rather than a gap in the
specification: CF-40's clause is written as `**CF-40.**` in §6 rather than under
a `####` heading, so a search that assumed the heading form found the *nearest
marker after a mention of it* — VT-22's, three thousand lines away — and copied
that clause's falsifier and its silence about phases. CF-40's own marker names
phases 9 and 10 explicitly. Corrected here, and worth leaving on the record:
a clause written in a different shape from its neighbours is exactly what a
mechanical read gets wrong, and the anchor discipline the citation checkers
enforce exists for the same reason.

### The blocked cases

Twelve case headers in `E2E-CASES.md` carry a blocked marker; the catalogue's
*"What cannot be written yet"* enumerates eleven **decisions**, which is a
different count of a different thing — E2E-05 shares E2E-04's blocker, and several
decisions block cases whose headers are not marked.

| Case | Blocked on | Unblocked by |
|---|---|---|
| E2E-04, E2E-05 | one condition cannot carry per-item boundaries | phase 4 (VT-30) |
| E2E-11 | `ReadOptions` has no upper bound | phase 4 (ES-16, VT-29) |
| E2E-20 | the apply seam's write vocabulary | phase 6 (PS-9 – PS-11) |
| E2E-25 | a chunked rebuild lying about its own completeness | the port half at phase 6 (PS-24); the runner half at phase 7, which is what calls `head` |
| E2E-33, E2E-34 | the shape of `EventId` | phase 4 (VT-4 – VT-8), exercised phase 13 |
| E2E-38 | `after` cannot cross a store boundary, and there is no translation | phase 13 (ADR-0027's ingest-side condition type) |
| E2E-43 | a store-assigned time | phase 4 (VT-9) |
| E2E-46, E2E-48, E2E-49 | deletion, redaction and retained history | phase 14 |

The previous revision's table named **E2E-37** in E2E-38's row. E2E-37's header is
not marked blocked and its own text says the refusal is checkable today
(`E2E-CASES.md:970-973`) — it needs the sync crate to exist, which is a missing
crate rather than a missing decision. E2E-38 is the blocked one, and what blocks
it is that `after` is store-local with no translation and adding a field is
`error[E0599]` (`E2E-CASES.md:988-993`). Getting this wrong pointed phase 5 at a
case it does not unblock and left phase 13's hardest sync case looking settled.

---

## The instrument portfolio

CF-25 forbids declaring a port frozen while an axis has no passing implementation
at its far end, unless the freeze names the axis and an ADR accepts the risk.
**Seven axes, and not one adapter instrument at a far end.** Four of the seven
now carry a *fixture* instrument and three are empty at both ends — which is the
honest caveat that outranks every `[FROZEN]` marker in the specification:
`MemoryEventStore`, the rusqlite skeleton and the Durable Object stand-in all
serialise their writers and assign positions under a lock they hold until commit.
That is one storage shape wearing several hats.

Phase 2 changed the fourth column but **not the fifth**, and phase 3 changed the
fifth only as far as a fixture reaches. The difference is the whole point of the
table, and CF-26 is the clause that states it: a **fixture instrument** proves a
rule at that end *can fail*; only an **adapter instrument** proves a real
implementation at that end *can pass*. Six skeletons exist and every one has
disagreed with the ports in some way worth writing down
([`adapter-shapes.md`](references/adapter-shapes.md)) — but none has run the conformance
suite, because none has a body. An instrument is not a target.

| Axis | Near end | Far end | What phase 2's skeleton settled | Far end today, and what fills it |
|---|---|---|---|---|
| Position allocation | `MemoryEventStore` — under the append lock | `happenstance-postgres` — `nextval()` outside the transaction | That the cost is a **measurement, not a signature**: nothing in the port's types can express the invariant `nextval()` breaks. So the skeleton settled the shape and [an experiment](experiments/position-visibility/README.md) settled the number | **fixture, phase 3** — `PreCommitPositionStore` fails `nothing_below_an_observed_position_appears_later` deterministically on one thread (CF-13). Adapter at phase 10 |
| Transport | in-process | `happenstance-neon` — one-shot HTTP, no cursor, no interactive transaction | That a store with no connection, no cursor and no interactive transaction satisfies `EventStore` **as written** — and that a probe-then-write `append` compiles and races. Eight capability limits, none of them a type error | **empty at both ends.** Adapter at phase 10 |
| Async flavour | `Send` native | `LocalMemoryEventStore`, then a Durable Object | That a genuinely `!Send` error and a genuinely `!Send` store compile against the bare flavour on `wasm32`, and that the *derived* flavour does not imply a `Send` error either (ADR-0009) | **fixture, phase 1** — `LocalMemoryEventStore` passes the suite natively and on `wasm32` (CF-28). Adapter at phase 9 |
| Batch shape | a SQL transaction | Ladybug's graph write handle | That an owned batch serves SQL, HTTP and a graph handle — **and that a borrowed GAT still works too**, which cuts against §4.2's argument and is phase 6's to weigh | **empty at both ends**, and there is no projection suite to run either end against. Adapter at phase 11 |
| Handle multiplicity | one handle per fixture | two handles onto one backing store | Nothing. No skeleton has two handles onto one backing store | **fixture, phase 3** — `Fixture::connect` and `SECOND_HANDLE` (CF-16); `CachedHeadFixture` fails `two_handles_observe_each_others_appends`. Every fixture still hands out refcount clones of one in-process object, so no *connection* has been opened twice. Adapter at phase 8 |
| Durability | in-memory | a store that survives a process reopen | Nothing. Every body is `todo!()` | **fixture, phase 3** — `REOPEN` and `acknowledged_writes_survive_a_reopen` (CF-17); `LosingFixture` fails it. Nothing yet loses a write to a *fault* rather than to an instruction. Adapter at phase 8 |
| Completeness | a whole log | a store holding only a suffix | Nothing | **empty at both ends**, and nothing is planned before phase 14 (CF-27) |

**Neither a skeleton nor a fixture fills a far end**, and the fifth column exists
so that nobody reads the fourth as covered. A skeleton falsifies a *signature*; a
fixture proves a *rule bites*; a far end is a **passing implementation**, and
phase 3 produced none — every instrument it built is a fixture, which is the most
that phase could produce. What the two waves bought is that five storage shapes
have now disagreed with the ports instead of one, and that four axes have a rule
that is known to be capable of failing rather than assumed to be. Neither is the
thing CF-25 asks for.

Until a row's far end exists, the freeze on the clauses that depend on it is
conditional, and the ADR that lands the specification must record the risk
acceptance. Phase 4's exit criteria name the five `ES` clauses carrying the
exposure — **ES-10** (position allocation), **ES-11** and **ES-12** (transport),
**ES-35** (durability) and **ES-40** (completeness) — and `SPECIFICATION.md`
§1.3, §1.6, §1.7 and §6.5 all state the same seven-axis, zero-adapter position.

---

## Phase 0 — Ground clear

**Goal.** Every mechanical, undesigned thing that gets more expensive with each
commit, done in one pass, before any design work starts.

**Why here.** The ADR-0006 rename costs more with every document phases 1–7 will
produce, and it is already generating wrong instructions to the two files an agent
loads first (`PRESSURE-TEST.md:389-393`).

**Decisions it settles.** None. Everything here is executing decisions already
taken.

**Work**

- [x] **Execute the ADR-0006 rename**, in its own commit with nothing else in it.
      `happenstance` → `happenstance-core`; `happenstance-runtime` →
      `happenstance`, shipping day one as a five-line facade
      (`pub use happenstance_core::*;`) so `cargo add happenstance` is true
      throughout. Two places a mechanical rename goes wrong without failing:
      `xtask/src/main.rs:60-77` hard-codes `-p happenstance` in the wasm32 step
      (repoint it, or the gate silently starts checking the *typed* layer and
      standing constraint 1 goes unguarded behind a green gate), and
      `xtask/src/main.rs:121` selects that step as `&REQUIRED[3..4]` by index —
      select it by name. `.github/workflows/ci.yml`'s `package:` list needs the
      same repoint.
- [x] **Re-anchor the specification's crate paths.** `SPECIFICATION.md` was
      written against the tree at `2a65d76` and cites `crates/happenstance/src/…`
      for the contract crate. After the rename those are `crates/happenstance-core/src/…`.
      CF-38's checker will otherwise dangle on every one of them. The document is
      a single assembled file, so this is one pass over one file — but it must run
      *after* the rename lands, not alongside it, because a citation rewritten
      before the path it names exists cannot be verified by anything.
- [x] **Build `cargo xtask spec-trace`** (CF-38), which is the *only* conformance
      rule CF-35, CF-36 and CF-37 name and which nothing in the workspace builds.
      This file cites it twice as a live gate — here, and in phase 13's exit
      criterion that a renewed deferral without a named experiment is a build
      failure — so until it exists, both citations are decorative. It parses the
      specification's clauses and asserts: every clause carries a maturity marker;
      every `[PROVISIONAL]` and `[DEFERRED]` marker carries a non-empty falsifier
      or experiment; every `Rule:` name either exists in `suite.rs` or is marked †
      as unwritten; every `Cases:` name exists in `E2E-CASES.md`; every `file:line`
      citation resolves; and **every rule in `suite.rs` is either claimed by a
      clause or disposed of by one** through a `Retires: <rule> — <reason>` line.
      That last check is the one with a subtlety: two rules
      (`query_all_matches_every_event`, `racing_conditional_appends_elect_one_winner`)
      are deliberately unclaimed because phase 3 retires them, and a checker that
      cannot tell "decided" from "forgotten" would fail on them forever or would
      have to drop the check that catches a rule outliving its clause. Add it to
      `xtask`'s `REQUIRED` list **by name, not by index** — the same defect the
      wasm32 step already has at `xtask/src/main.rs:121`.
- [x] **The checker and §1.3's hand count agree exactly** — 193 clauses, 132
      FROZEN, 46 PROVISIONAL, 13 DEFERRED, 2 NON-NORMATIVE. They did not at first,
      and every discrepancy turned out to be a checker bug rather than a document
      one; converging on the number a reader computed by hand is the best evidence
      available that the parser now reads the document the way a person does.
- [x] **Regenerate `SPECIFICATION.md` §7.2 from the checker, and stop hand-editing
      it.** The table was computed by hand at `2a65d76` and has never been
      verified; where it and the checker disagree, the checker is right, which is
      the entire argument for having one. §7.3 – §7.6 stay authored — they carry
      the judgement about *why* a gap exists, which is where the dispositions come
      from. Expect the first run to find real errors in the table; a first run that
      finds none means the checker is not checking.

      §7.1 was regenerated with it, and had to be: it was the wrong half. Its
      hand-rolled totals read 137 / 41 / 14 / 1 while §7.2's own rows aggregate to
      132 / 46 / 13 / 2 — so the *table* was right, the *summary* of it was wrong,
      and it contradicted both the rows twenty lines below it and §1.3 six
      thousand lines above it for as long as the three coexisted. Both miscounts
      inflated FROZEN, which is the direction that flatters the document. 81 of 193
      rows changed; the two substantive classes were thirteen phantom rule names
      that were code-formatted ordinary words — including `` `happenstance` ``
      itself, listed as a conformance rule — and **six alias pairs**, where §6
      names a rule differently from the §3 clause specifying the same behaviour.
      The unwritten-rule backlog was mis-costed at twelve where it is six. Which
      spelling wins is recorded in §7's preamble as owed reconciliation and is
      **phase 3's** to settle, since it is the phase that writes the registry.

      The generated region is delimited by HTML comments and the gate recomputes
      and compares it, so hand-editing inside the markers is inert rather than
      merely discouraged. §1.3 is deliberately *not* generated: it is a count a
      human computed by reading, which is what makes its agreement with the checker
      evidence that the parser reads the document the way a person does — and
      generating it would destroy the very thing it is being used to prove. It is
      checked against the computed census instead, including its own arithmetic,
      which is how the fossil "192 are normative" (193 − 2 = 191) surfaced.
- [x] **Reserve three crates.io names now — `happenstance`, `happenstance-core`,
      `happenstance-testkit`** — and thereafter **one name per phase, when that
      phase starts.** All three were verified free on 2026-08-06, as was
      `happenstance-cloudflare`.

      Not ten. crates.io's policy prohibits a crate that *"exists only to reserve
      a name for a prolonged period of time … without having any genuine
      functionality, purpose, or significant development activity on the
      corresponding repository"*, and says the team will normally give the author
      a chance to justify the crate first. These three exist as code today and are
      published by phase 12, so the justification is a `cargo package --list`.
      Seven of the original ten are stubs or nothing at all —
      `happenstance-postgres`, `happenstance-neon`, `happenstance-sync-testkit` and
      `happenstance-cloudflare` have no code whatsoever — and a placeholder for a
      crate that may not exist for ten weeks is the thing the policy describes,
      whether or not it survives a challenge.

      **This buys less protection than it looks like, and that is not a reason to
      claim more.** crates.io has no prefix reservation: owning `happenstance`
      does nothing to protect `happenstance-sqlite`. So the exposure on the
      adapter names is identical whether they are claimed today or at their phase,
      and claiming them early trades a policy violation for no additional
      security.

      A `0.0.0` placeholder buys nothing for `cargo-semver-checks` either, because
      0.0.x versions are mutually incompatible in Cargo; the baseline is fixed
      separately below.

      The per-phase claims, so nothing is forgotten: `happenstance-sqlite` at
      phase 8, `happenstance-cloudflare` at 9, `happenstance-postgres` and
      `happenstance-neon` at 10, `happenstance-ladybug` at 11, `happenstance-sync`
      and `happenstance-sync-testkit` at 13.

      **`cargo xtask reserve <name>` generates the placeholder**; `cargo xtask
      reserve` with no argument lists every name and its phase. It writes a
      standalone `0.0.0` crate under `target/reserve/` carrying both licences and
      a README that says plainly it has no functionality, then prints the publish
      command — it never publishes anything itself. The list of names lives in
      `xtask/src/reserve.rs`, which is what makes "which names do we intend to
      hold?" answerable by a command rather than by reading this file.

      The reason it is a command and not a note here: the claims are weeks apart,
      and a procedure run that rarely from memory is one that drifts. Forgetting
      the licence files once ships a crate without them permanently.
- [x] **Rewrite the documents the rename inverts.** Partly done: CLAUDE.md's
      repository map, dependency rule and constraint 2 are corrected, the settled
      `happenstance-runtime` open question is struck through rather than deleted,
      README's status table is repointed, and CONTRIBUTING now names
      `happenstance-core` in the gate description. Constraint 2 was the one that
      *inverted*: it forbade `serde` to the crate whose job is now encoding, and
      would have permitted it into the contract crate — the exact reverse of
      ADR-0003. It now says which crate it means and why the distinction matters.

      Still to do here: CLAUDE.md's "What this is" paragraph, the ADR-0001/0003/0004
      bodies, and the old runtime crate docs (that file is now
      `crates/happenstance/src/lib.rs`, rewritten as the facade). Add the "On the historical
      record" section to ADR-0006. Correct CLAUDE.md's claim that `cargo hack` and
      `cargo deny` are not installed — both resolve on this machine
      (`PRESSURE-TEST.md:398-400`), so the gate is stricter locally than the file
      says. Append to constraint 5: *until first publish the MSRV is a preference,
      not a promise — weigh it, do not obey it.*

      All of that is now done, and two things were found in the doing. ADR-0007
      was in nobody's list: it is accepted, not superseded, and it linked
      `ProjectionStore` to `crates/happenstance/src/projection.rs` — a path that
      does not exist, because the facade has only `lib.rs`. It was missed
      precisely because ADR-0006's new "On the historical record" section
      enumerated 0001/0003/0004 and stopped; the section now names it. And
      `crates/happenstance-sync/src/lib.rs` carried the ADR-0003 **inversion** in
      running prose — "keeping `happenstance` free of `serde`", citing a
      `happenstance/serde` feature path the manifest does not have — one sentence
      of which said `happenstance` while its own intra-doc link two words later
      said `happenstance_core`. That is the failure constraint 2 was rewritten
      this same pass to warn about, surviving inside the crate that most needed to
      get it right.

      Two stale *phase* numbers were repointed with them: ADR-0001's banner said
      the Cloudflare proof arrives at phase 5 (it is phase 9) and ADR-0003's said
      sync round-trips at phase 6 (it is phase 13, and phase 6 is now
      `ProjectionStore`). ADR-0006's three citations of "phase 3" are deliberately
      **left wrong**, and the distinction is worth keeping: the runbook was
      *replaced*, not renumbered, so there is no phase 3 that became phase 7 and
      the rewrite-the-referent rule does not reach them.
- [x] **Fix D10.** Make the README compile:
      `#![cfg_attr(doctest, doc = include_str!("../README.md"))]`, then fix
      `README.md:89-102` and the Quick start with hidden
      `# #[tokio::main] async fn main() -> Result<(), Box<dyn Error>> {` wrappers,
      and re-fence the deliberate fragment as `rust,ignore`. **Name which README
      CI doctests**: `include_str!("../README.md")` from a crate's `src/lib.rs`
      resolves to the *per-crate* README the next item creates, not the workspace
      one whose Quick start is broken. Say whether the other is duplicated or left
      unverified; the previous plan's exit criterion was not achieved by its own
      task list (`PRESSURE-TEST.md:318-323`).
- [x] **Fix D11.** Copy `LICENSE-MIT` and `LICENSE-APACHE` into each publishable
      crate directory — Cargo will not follow paths outside the package root and
      Windows makes symlinks awkward — write a per-crate `README.md`, add
      `readme = "README.md"`, add `homepage` to `[workspace.package]`. Add
      `cargo package -p <crate> --list` to the gate asserting the licences and
      README are in the artifact; `cargo publish --dry-run` does not warn.

      **This box was ticked before its last sentence was true.** The licences and
      READMEs landed; the gate step asserting they *stay* there did not exist for
      three commits, which is why the exit criterion below it stayed unticked
      while the work item read as done. The step exists now, and it parses the
      file list rather than trusting the exit status — `cargo package --list`
      succeeds whether or not a licence is present, which is the same reason
      `--dry-run` does not warn. Its publishable set is derived from
      `cargo metadata`, so deleting a `publish = false` from a stub is caught
      rather than remembered.
- [x] **Fix D12.** `serde = ["dep:serde", "bytes/serde", "serde/alloc"]`. The
      feature currently compiles only because `bytes` happens to enable
      `serde/alloc` transitively.
- [x] **Fix D13.** Add `cargo doc -p happenstance-core --no-default-features
      --no-deps` to the gate — three unconditional intra-doc links currently make
      the `no_std` configuration a hard error, and the powerset step runs `check`,
      not `doc`. Add a nightly `RUSTDOCFLAGS="--cfg docsrs -D warnings"` step
      behind a toolchain probe. Widen the wasm32 step to the feature powerset
      while keeping one mandatory plain `cargo check`, so the constraint-1 guard
      cannot become skippable. Add `--locked`.

      **Ticked on its first sentence; the other three were never done.** They are
      now. The nightly step needed more than itself: it sits in `OPTIONAL` behind
      a `cargo +nightly --version` probe, and **nothing in `.github/` installed
      nightly**, so it printed `skipped` on all three runners while
      `xtask/src/main.rs` asserted "CI installs nightly, so the check is real
      there" and phase 12 below spent it as a proof artefact. A step that always
      skips is the decorative-rule failure applied to tooling, and it had two
      documents vouching for it. The gate job now installs nightly with
      `rustup toolchain install`, deliberately **not** a second
      `dtolnay/rust-toolchain@nightly` — that action runs `rustup default`, so a
      pair of them is order-sensitive and getting it backwards runs the *entire*
      gate on nightly, which is worse than the skip it replaces.

      A fourth thing surfaced with them, in the same class and not in D13's list:
      rustdoc does not read `RUSTFLAGS`, so `ci.yml`'s ambient `-D warnings`
      reached every rustc invocation in the gate and no rustdoc one. The
      `documentation` step had been printing "generated 3 warnings" and exiting 0
      for as long as it had existed. Both mandatory doc steps now carry
      `RUSTDOCFLAGS=-D warnings`, and the three warnings are fixed.
- [x] **Close the lint hole.** `Cargo.toml:59` is `todo = "allow"` workspace-wide.
      Make it `deny`, with `#![allow(clippy::todo)]` in the stub crates that need
      it. Same for `unwrap_used`, currently `warn` at `Cargo.toml:57`, since the
      test modules already opt out locally.
- [x] **Repoint `cargo-semver-checks`** at
      `--baseline-rev ${{ github.event.pull_request.base.sha }}` so the job does
      something today, and correct CONTRIBUTING.md, which claims a registry
      baseline that does not exist.

      The checkout in that job needs `fetch-depth: 0` with it. `actions/checkout`
      defaults to a shallow clone of one commit, so the base SHA the baseline
      names is not in the local object store and the job fails on the thing it was
      just repointed at.
- [x] Weekly `schedule:` job running only `cargo deny check advisories`. Only
      advisories, because a new advisory against an unchanged dependency is the
      one failure that arrives with no commit to trigger CI — it needs a clock.
      Licences and bans change only when a manifest does, and the gate already
      catches those on every push.
- [x] A CI job running `cargo test --workspace` at the 1.85 MSRV. `proptest
      1.11.0` and `getrandom 0.4.3` both declare `rust-version = 1.85`, and CI's
      `--no-dev-deps` cannot see them: zero headroom, verified nowhere.
- [x] `CHANGELOG.md` with an `## [Unreleased]` section, started now rather than
      reconstructed from twelve phases of history at publish time.
- [x] **`.idea/` and `.mcp.json` are gitignored.** Done ahead of this phase,
      because every phase's exit gate references a clean tree and until this landed
      that phrase meant nothing. Both are ignored whole: JetBrains' own
      `.idea/.gitignore` already drops `workspace.xml`, and the only remaining file
      with settings in it enables ESLint in a Rust workspace; `.mcp.json`'s port is
      assigned per machine, so a committed copy is a config that fails silently on
      the next checkout. What the project enforces lives in `Cargo.toml`'s lints
      and `cargo xtask ci`, which is the same for every editor.
- [x] Soften the README's "with batteries" tagline to what phase 12 will actually
      ship, or move the missing items into phase 7's scope explicitly. Softened:
      0.1 is the contract, the conformance suite, the typed layer and SQLite, and
      the differentiated claim was never the batteries anyway — it is a *published*
      suite that makes "storage agnostic" checkable by a third party.

**Proof artefact.** `cargo package -p happenstance-core --list` and
`cargo package -p happenstance --list` each show both licence files and a README;
`cargo test --doc -p happenstance-core` compiles the README's Quick start;
crates.io shows the three principal names owned; and `cargo xtask spec-trace` **fails** when a
maturity marker is deleted from a clause and passes when it is restored. Four
checkable facts, none of which is "the gate is green", and the fourth is the only
one of the four that could have been faked by a checker that does nothing.

**All four hold.** The fourth is the one that paid, and it paid immediately: its
first run over §7 found that §7.1's totals had been wrong since the document was
assembled, in the direction that flatters it. See the `§7.2` work item above.

**Exit criteria**

- [x] Three names owned on crates.io — `happenstance`, `happenstance-core`,
      `happenstance-testkit` — each with a description, licence, repository link
      and a one-paragraph README, so that each is a crate with a stated purpose
      rather than a parked name. The remaining seven are claimed at their phases;
      each of those phases carries the item.
- [x] The README's code blocks are compiled by CI, and the document says which
      README that is. It is the `xtask` crate that compiles the *repository*
      README, and the reason is worth stating where a reader will find it:
      `include_str!("../../README.md")` cannot resolve inside a packaged `.crate`,
      so attaching it to a published crate would make `cargo test` fail for anyone
      who ran it. `xtask` is `publish = false` and is built on every push. Each
      per-crate README is compiled by its own crate, where the path stays inside
      the package after publication.
- [x] `cargo package --list` is a gate step and asserts on its output — it parses
      the file list for both licences and the README rather than trusting the exit
      status, and its publishable set is derived from `cargo metadata` rather than
      hand-maintained. Both halves were demonstrated to fail: removing a licence,
      and promoting a stub by deleting its `publish = false`.
- [x] No document in the repository names `happenstance` when it means the
      contract crate — including `SPECIFICATION.md`'s file citations. Verified by
      sweep rather than asserted: every bare occurrence across the Markdown, the
      crate sources, the examples, the manifests and `.github/` was read in
      context, and `happenstance-runtime` now survives only where it is
      deliberately history.

      **One stated exemption, which is what keeps this criterion honest rather
      than absolute.** The dated documents under `references/evaluation/` are *not*
      rewritten: they predate ADR-0006, the runbook cites several of them by
      `file:line` as evidence, and a rewritten review is no longer the review that
      was performed — the same rule ADR-0006 applies to superseded ADR bodies.
      [`references/evaluation/README.md`](references/evaluation/README.md) states the substitution
      a reader must apply and names the one file where applying it mechanically
      would garble the content, because that file *is* a rename migration table.
- [x] `cargo xtask spec-trace` is a named step in `REQUIRED`, and it has been
      demonstrated to fail on a deliberately broken clause. A checker nobody has
      seen reject anything is the decorative-rule failure applied to tooling.

      Three further demonstrations, because the checker grew three more claims:
      corrupting a maturity cell inside the generated §7.1–§7.2 region, corrupting
      §1.3's census, and breaking §1.3's internal arithmetic each fail it, each
      naming both disagreeing numbers. The two §1.3 checks are independent —
      corrupting a count leaves the arithmetic satisfied and vice versa — so
      neither masks the other.
- [x] `git status` clean; `cargo xtask ci` green with the five new steps —
      specification traceability, documentation without default features, the
      packaged-artifact assertion, the wasm32 feature powerset, and the nightly
      docs.rs configuration. Twelve steps, none skipped locally.

**Cases this makes writable.** None. This phase writes no clause and unblocks no
case; it removes the friction that would otherwise be paid twelve times.

**Estimate.** 1 day, plus 1 for `spec-trace`.

**Session log**

- 2026-08-06 — **phase 0 closed.** Gate green with twelve steps, none skipped
  locally; `git status` clean. The five remaining work items landed together:
  §7.1–§7.2 regenerated from the checker and held to it, the `cargo package
  --list` assertion, D13's three unwritten halves, the semver rev-baseline and the
  weekly advisories job, and the documents the rename inverted.

  **What the closing pass found is more useful than what it built**, and three
  findings are worth carrying forward because each is the same shape: a check that
  had been believed rather than watched fail.

  1. **§7.1 had been wrong since the document was assembled** — 137 / 41 / 14 / 1
     against §7.2's own rows aggregating to 132 / 46 / 13 / 2, both miscounts
     inflating FROZEN. Three copies of the census coexisted and disagreed, and no
     reader had caught it. This file carried the wrong one too, in four places.
  2. **The nightly `docs.rs` step was skipped on every CI runner**, because
     nothing in `.github/` installed nightly — while `xtask/src/main.rs` asserted
     the opposite and phase 12 below spent the step as a proof artefact. It
     guards `#![cfg_attr(docsrs, feature(doc_cfg))]`, an unstable feature whose
     first real compilation would otherwise have been on docs.rs, after
     publication, when a release cannot be edited.
  3. **`RUSTFLAGS: -D warnings` never reached rustdoc.** The `documentation` step
     had been printing "generated 3 warnings" and exiting 0 for its whole life.

  Two work items in this phase were ticked before they were true (D11's gate step,
  D13's last three halves), which is how (2) and (3) survived to be found late.
  Recorded in the items themselves rather than quietly corrected — a plan whose
  boxes run ahead of its work teaches the next reader to trust the boxes.

  Method note, since it is repeatable: every one of the three was found by an
  adversarial pass whose brief was *try to prove this check is decorative*, run
  after the pass that built them and against their author's own report. The
  builders reported all three as done, in good faith, and all three were.

- 2026-08-06 — ADR-0006 rename executed (`7d6c1b0`); D11 fixed, all three
  publishable crates now package both licences and a README (`48565e5`);
  `cargo xtask reserve` added and `.cargo/credentials.toml` gitignored
  (`7004010`). **`happenstance`, `happenstance-core` and `happenstance-testkit`
  reserved on crates.io at `0.0.0`** — metadata verified through the API:
  description, repository, `MIT OR Apache-2.0`, README present, `rust-version`
  1.85, not yanked. The remaining seven names are claimed at their phases, per
  the rule above. Gate green throughout.

  Still open in this phase at the time: `spec-trace` (CF-38), D10, D12, D13, the
  `todo = "allow"` lint hole, the MSRV job and `CHANGELOG.md`. All closed by the
  entry above.

- 2026-08-05 — carried from the previous runbook, because it is the only dated
  evidence in the repository that the gate was ever green: `cargo xtask ci`
  verified green at `9fd2337`, with `cargo hack` and `cargo deny` skipped locally.
  Both now resolve on this machine (`PRESSURE-TEST.md:398-400`), so that skip no
  longer applies and the baseline is stricter than the line that recorded it.

---

## Phase 1 — The `!Send` proof and the derivation decision

**Goal.** Turn ADR-0001 from a recorded intention into precedent, and settle how
the second trait flavour is produced — for both ports, in one decision.

**Why here.** The derivation rule shapes every signature in the workspace, and
every later phase writes signatures.

**Decisions it settles.** ADR-0008 (derivation, both ports — PS-35). Discharges
ES-1 – ES-5, ES-7, PS-37, CF-20, CF-22, CF-23, CF-28.

**The three facts it must reconcile**, each reproduced by compilation:

1. `trait_variant` does not rewrite default `async fn` bodies (E0728), so a
   provided method must be hand-desugared to `-> impl Future` with an
   `async move` block. It *does* accept that form, and the derived flavour's
   provided future is `Send` in generic code — the claim that `head()` and
   `count()` could never be added was compiled and refuted
   (`PRESSURE-TEST.md:32-54`). Do not reintroduce it.
2. The desugared body captures `&self` across an await, so the `Send` flavour
   needs `Self: Sync`. Taking it at the *point of use* — `where Self: Sync` on the
   method — works and leaves the stream alone. Putting `Sync` in the attribute
   instead demands a `Sync` stream and `Sync` futures from every adapter: a stream
   whose hidden type is `Send` but not `Sync` compiles today and stops compiling
   under `make(Send + Sync)` with `error[E0277]` (`PRESSURE-TEST.md:154-176`).
3. `variant.rs` clones a provided body into the variant, so **one body must
   type-check under both flavours' bounds simultaneously.** Nobody has written
   that obligation down; ADR-0008 is where it goes.

**Work**

- [x] Replace the hand-duplicated rule list in
      `crates/happenstance-testkit/src/lib.rs:93-129` with a **rule registry**
      using the callback ("x-macro") pattern: `for_each_rule!($callback:path)`
      hands the list to a named emitter. Two gotchas: the emitter must be
      `#[macro_export]`ed even when `#[doc(hidden)]`, because `macro_rules` items
      live in a flat crate-root textual namespace; and the callback must be
      `$crate::`-prefixed or it resolves in the caller's scope, which is the exact
      bug being fixed. Emit `__emit_tokio`, `__emit_blocking`
      (`futures::executor::block_on`) and `__emit_wasm` (`#[wasm_bindgen_test]`).
      Not `libtest-mimic`: the case list is statically known, and one `#[test]` per
      rule is what makes `cargo test <rule_name>` and IDE gutters work.

      Landed as `crates/happenstance-testkit/src/registry.rs`, spelled
      `for_each_event_store_rule!` — **CF-22's spelling, not this file's**
      `for_each_rule!`; where a phase body and the specification disagree the
      specification wins, and this is the first time that rule has actually been
      exercised. Both gotchas reproduced before being fixed. A third was not
      anticipated and is the useful one: **`$callback:path` does not work as a
      macro callee in expression position.** It is fine in item position, so every
      harness would have compiled and only the CF-24 meta-test — which needs
      `let names = for_each_event_store_rule!(…)` — would have failed, with
      `error: macro expansion ignores '!' and any tokens following`, a diagnostic
      that points nowhere near the fix. `($($callback:tt)+)` works in both
      positions and still accepts a `path` fragment forwarded from an outer macro.
- [x] `LocalMemoryEventStore` — `RefCell<Vec<SequencedEvent>>` implementing the
      bare `EventStore` and nothing else. It is `!Send`, so it exercises the path
      no code in the workspace has ever exercised (ES-7, CF-28).

      Landed as `Rc<RefCell<Vec<SequencedEvent>>>`, and the `Rc` is not
      decoration: **`RefCell<T>: Send where T: Send`** — it surrenders `Sync`, not
      `Send`. A store that was literally `RefCell<Vec<SequencedEvent>>`, which is
      what this line and CF-28 both say, would have been `Send` and would have
      proved nothing at all about the bare flavour. CF-28's wording is owed the
      same correction.
- [x] A CI job running the full registry under `wasm-bindgen-test` on
      `wasm32-unknown-unknown` against that store. No Cloudflare, no `workerd`.

      `wasm-conformance` in `ci.yml`, and run locally before it was claimed: 27
      rules green against the `!Send` store and 27 against `MemoryEventStore`, on
      `wasm32-unknown-unknown` under `wasm-bindgen-test-runner`. The job reads the
      `wasm-bindgen` version out of `Cargo.lock` rather than hard-coding it,
      because the runner refuses a schema mismatch and a `cargo update` would
      otherwise turn a version bump into a confusing runtime failure. `cargo xtask
      ci` gained a `--target wasm32` *type-check* of the same harnesses, which is
      a weaker and different claim: `#[tokio::test]` type-checks for wasm32 and
      then cannot run there.
- [x] A **generic** Send-composition compile test. The existing one
      (`read_stream_is_send`, `memory.rs:328-340` in `7d6c1b0`; replaced at
      phase 1) asserted the property on a concrete type, where
      auto-trait leakage makes it pass regardless of whether the design works —
      which means CLAUDE.md constraint 3 currently protects a test that cannot
      fail. Write the bound at the definition:
      `fn spawns_from_generic<S: SendEventStore + Send + Sync + 'static>(s: Arc<S>)`,
      so the obligation is discharged before monomorphisation (ES-2, ES-3).

      **Two tests landed, not one, and the second is the one that matters.** The
      rule ES-2 actually names —
      `send_flavour_stream_is_send_in_generic_code` — does **not** reject the
      refactor ES-2 says it rejects. Under `async fn read(..) -> Result<impl
      Stream, E>` the outermost item is the *future*, `trait_variant` marks the
      future `Send`, and an assertion on the call's result is discharged against
      the future while the stream stays `!Send`. The clause's own account of the
      mechanism implies this; its rule was written as though it did not. Landing
      only the named rule would have swapped a test that cannot fail for one that
      cannot catch the break it was written for. `spawns_from_generic` holds the
      stream across an await inside a real `tokio::spawn` and is what bites.

      Two bound findings ride along. `+ Send` in this file's spelling is
      **redundant** — `trait_variant` emits `pub trait SendEventStore: Send`, a
      supertrait, confirmed by macro expansion; `Sync` and `'static` are both
      load-bearing and each was removed in turn to see which diagnostic appeared.
      And the natural read-then-append body **does not compile**: a
      `Result<_, S::Error>` held across the second await makes the spawned future
      `!Send`, which is E2E-53 arriving nine phases early. The test survives only
      by collapsing the read to a `usize` first. **ES-6 is therefore blocking a
      composition a caller will obviously write**, which is a stronger statement
      than "semver-visible" and belongs in phase 2's brief.
- [x] **Two throwaway error-shape probes**, so this phase can exit on its own
      evidence rather than on phase 2's. One error type that is `Send + Sync`, one
      that is `!Send` (a `PhantomData<*mut u8>` field is enough), each implementing
      the port under both candidate declarations. Record which declarations both
      can satisfy. This is the item whose absence meant the previous plan's phase 1
      could not exit (`PRESSURE-TEST.md:555-558`). The *decision* about the `Error`
      bound stays deferred to phase 2 (ES-6); what this settles is whether the two
      flavours *can* differ at all.

      **They cannot**, and ES-5 survived five distinct falsification attempts:
      naming the associated type in the attribute (`expected '+'` — the macro
      grammar is `Ident : TraitBound (+ TraitBound)*`), a supertrait owning a
      stricter `Error` (`E0221`), the same with an associated-type bound
      (`E0221` + `E0308` + `E0053` inside the generated blanket impl), and a
      where-clause on the bare trait, which is copied onto *both* flavours by
      `mk_variant`'s `..tr.clone()` and additionally breaks the blanket impl with
      `error[E0275]: overflow evaluating the requirement`. ES-5's conclusion holds
      and its cited mechanism is incomplete — it credits `transform_item`, and the
      copying is `mk_variant`'s.

      The matrix result that matters for phase 2: the **current** declaration
      genuinely admits a `!Send` error on the *derived* flavour. Constructing and
      returning one compiles; the failure appears only when the value is held
      across an await, or at a call site demanding `F::Output: Send`. So
      `store_error_crosses_a_join_handle` must assert on the **output type** — a
      rule that checks only whether the future is `Send` passes against a `!Send`
      error and is decorative. Applying `+ Send + Sync` to the real trait leaves
      the whole workspace green, which is the specification's own warning made
      concrete: nothing in the tree can fail that bound today.
- [x] Prove a provided method is writable: add a throwaway
      `fn probe(&self) -> impl Future<Output = ()> { async move {} }` to the port,
      confirm an adapter that overrides it and one that does not both compile on
      both flavours, then delete it. The real provided methods land in phase 4.

      Done with a body that *does* real work, because the spelling this line
      proposes proves nothing: `async move {}` captures nothing, so it cannot
      exhibit the `&self`-across-an-await problem the whole rule is about. The
      probe used a `head` whose body reads through `&self` and awaits. It compiles
      under both flavours; `-Zunpretty=expanded` shows three copies — the bare
      default, the derived default with `+ Send` appended, and a blanket-impl
      *override* delegating `<Self as SendEventStore>::head(self)`, so a provided
      body is a fallback for the bare flavour and never a shared implementation.
      All seven of the specification's `variant.rs` citations verify exactly.
- [x] State the rule ADR-0008 exists to make quotable: *any provided or extension
      body that holds `&self` across an await requires `Self: Sync` at the point of
      use* — and check it against `ProjectionStore`, which carries the identical
      construction at `projection.rs:70` and appears in no ADR at all (PS-37).

      Checked, and the two ports **do not cost the same**. On `EventStore` the
      rule is sufficient. On `ProjectionStore` a provided body that also holds the
      GAT across a suspension point does not compile at all and has no remedy:
      `for<'a> Self::Batch<'a>: Send` fails with `E0311` on
      `TraitVariantBlanketType`, and the only clause that does compile is cloned
      onto the `!Send` flavour where it rejects an `Rc` batch. Isolated with three
      local traits under the same attribute — no GAT works, a GAT *without*
      `where Self: 'a` works, a GAT *with* it gives `E0311` — so `EventStore`
      structurally cannot exhibit it. That asymmetry is the direct answer to
      PS-35: one scheme, but a different provided-method budget per port, and only
      a document holding both halves can show it.

      A second cost lands on both: **`Self: Sync` makes a provided method
      uncallable on a `RefCell` store**, which is the shape the bare flavour
      exists for. There is one escape — take the arguments as parameters so the
      future captures owned values rather than `&self` — and it closes when the
      method builds its own `Query`, because edition-2024 RPITIT ties the stream
      to the query's lifetime. Phase 4 designs `head`/`count` and should spend
      that lever deliberately rather than meet it.

**Proof artefact.** Three things, and the order matters, because only the first
discriminates the decision this phase exists to make.

1. **A single provided body that type-checks under both flavours' bounds at once**,
   committed with the attribute that clones it into the variant — plus the two
   throwaway error shapes, one `Send + Sync` and one `!Send`, recording which
   declarations both can satisfy. This is ADR-0008's evidence. It would not exist
   if the derived flavour could not carry a provided body, which is the claim under
   test.
2. A green CI job — every conformance rule passing against a `!Send` store on
   `wasm32-unknown-unknown`.
3. A committed compile test that a *generic* `SendEventStore`-bound function is
   spawnable.

Two and three were the previous revision's whole proof, and **neither
discriminates derived from hand-written**: both flavours' bare half is identical
either way, so a green wasm job and a spawnable generic exist whichever choice
ADR-0008 makes. They are the two halves of *ADR-0001's* claim, which is a
different claim and also worth proving. Item one is the half that was missing.

**All three hold.** One caveat on item one, stated because it changes what the
artefact proves: the phase's framing — *derived versus hand-written* — was
already settled by ES-1, which is `[FROZEN]` on "MUST be derived". So the body
that type-checks under both flavours is not evidence for a choice this phase
made; it is evidence for the **obligation** ES-4 and PS-37 state and nothing had
ever compiled. ADR-0008 is a ratification plus PS-35's extension to the second
port, and says so in its own Context rather than claiming a decision it did not
take.

**Exit criteria**

- [x] ADR-0008 written, quoting the compiled evidence for the derived-versus-hand-written
      choice, and covering both ports in one decision (PS-35).
      [ADR-0008](.kb/decisions/0008-one-derivation-for-both-ports.md). See the caveat above
      on what "the choice" turned out to be.
- [x] ADR-0001's `provisional` marker removed, or ADR-0001 superseded. Its full
      proof is cited forward to phase 9. Removed, with the banner rewritten to
      record what lifted it rather than deleted — a marker that vanishes teaches
      the next reader that it was never there.
- [x] The rule list exists in exactly one place —
      `registry.rs`'s `for_each_event_store_rule!`. `no_orphan_rules` was watched
      failing against a deliberately unregistered 28th rule, naming it.
- [x] The wasm32 conformance job is in `ci.yml` and green. Run locally first: 27
      rules against the `!Send` store and 27 against `MemoryEventStore`, executed
      on `wasm32-unknown-unknown`.
- [x] The generic spawn test exists and CLAUDE.md constraint 3 no longer points
      at a test that cannot fail. It now points at **two** tests and says why one
      is not enough.

**Cases this makes writable.** E2E-52 (a whole command path with no `Send`
bound), E2E-53 (`SendEventStore` from several tasks — its `Error` half waits on
ES-6), E2E-30 (a `!Send` projection store cannot be spawned, and the port says so).

**Estimate.** 4 days.

**Session log**

- 2026-08-06 — **phase 1 closed.** Gate green with thirteen steps, none skipped
  locally; `git status` clean. `LocalMemoryEventStore` is the workspace's first
  `!Send` implementer of either port and passes all twenty-seven rules under four
  harnesses — the testkit's runtime-free `block_on`, the default multi-threaded
  `#[tokio::test]`, a caller-supplied `current_thread` emitter, and
  `wasm-bindgen-test` on `wasm32-unknown-unknown`. ADR-0008 landed; ADR-0001's
  provisional marker came off against its own stated condition.

  **Four things this phase assumed and compilation refuted.** Each is the same
  shape as phase 0's findings — a belief nobody had asked the compiler about —
  and each is recorded in the work item it belongs to rather than only here.

  1. **The rule ES-2 names does not reject what ES-2 says it rejects.** Landing
     only it would have replaced a test that cannot fail with one that cannot
     catch the specific break it exists for. Two tests now, and CLAUDE.md
     constraint 3 explains why one is not enough.
  2. **`RefCell` is `Send`.** It surrenders `Sync`. The store this phase and
     CF-28 both specify — `RefCell<Vec<SequencedEvent>>` — would have been `Send`
     and would have lifted nothing. `Rc` is what does the work.
  3. **`#[tokio::test]` already drove a `!Send` store.** `tokio::spawn` requires
     `Send`; `Runtime::block_on`, which the attribute expands to, does not. The
     registry is still right, but its justification is wasm portability, not
     `Send`-ness — and this phase's plan had the causation backwards.
  4. **`$callback:path` cannot be a macro callee in expression position.** Every
     harness would have compiled; only the CF-24 meta-test would have failed, with
     a diagnostic that points nowhere near the fix.

  **What this phase found that a later one owns.** ES-6 is not merely
  semver-visible — it blocks the read-then-append composition inside
  `spawns_from_generic`, because `Result<_, S::Error>` held across the second
  await makes the spawned future `!Send`. The test survives by collapsing the read
  to a `usize`, which is not what a caller would write. Phase 2 inherits that as
  evidence rather than as a question. A third option for it surfaced too and
  neither prior document weighed it: a `ThreadSafeEventStore: SendEventStore<Error:
  Send + Sync>` marker with a blanket impl, which lets generic code demand the
  stronger property without `wasm32` paying for a native concern.

  **Method note, repeatable and worth the cost.** The evidence was gathered by six
  independent probes in isolated worktrees, each briefed to compile rather than to
  argue and to paste transcripts. Two things followed. The probes disagreed with
  the specification in six places and with each other in none — which is what
  gives the six corrections above their weight. And their worktrees were seeded
  from a stale commit; five detected it and reset, one did not and produced a
  correct design against pre-rename paths. That its output still transferred is
  luck, not method: **a probe's report must state the commit it compiled
  against**, and the next wave's brief should require it.

---

## Phase 2 — The instrument portfolio

**Goal.** Six in-tree adapter skeletons that compile against the ports on their
real targets and `todo!()` every body, plus a sketch of the third port — and a
table recording, per skeleton, the most ambitious signature attempted and what
happened to it.

**Why here.** A port's shape is falsified by a *type*, not by a behaviour, and
freezing a port before the falsifiers exist is the one decision in this plan that
cannot be undone cheaply.

**Decisions it settles.** ADR-0009 (ES-6, on the Cloudflare skeleton's `!Send`
error). Supplies the evidence for ADR-0011 – ADR-0013 and ADR-0017.

**Work**

Each skeleton declares real types and stubs real bodies. The point is the type
checker.

**The one place this phase borrows from a later one, stated rather than
discovered.** Two skeletons implement `ProjectionStore`, whose `Batch` shape phase
6 decides — and this phase's own exit criterion requires attempting
`type Batch<'a> = rusqlite::Transaction<'a>` on `SendProjectionStore`, which phase
6 records as *failing* for two independent reasons. So the attempt cannot also be
what ships. Resolve it this way: the skeletons adopt the **owned** `type Batch;`
as a working hypothesis, and the GAT attempt is recorded in
`references/adapter-shapes.md` as a failed-signature row with its compiler errors. That
is a pre-emption of phase 6, and it is legitimate only because phase 6 is where it
becomes a *decision* — if the owned shape turns out wrong there, these skeletons
change and phase 6's ADR says so. What is not legitimate is discovering the
borrow silently, which is how a plan acquires a cycle.

- [x] **`happenstance-sqlite`** — `rusqlite::Connection` behind a `Mutex`, a real
      error enum, a real `read` stream type, and a real `Batch` for the projection
      store. Implements `SendEventStore` and `SendProjectionStore`. Because `read`
      is not `async`, `tokio::task::spawn_blocking` *panics* if called at `read`
      time outside a runtime, so the spawn must be deferred into `poll_next`:
      laziness stops being a nicety and becomes load-bearing. This is the
      *serialising, `Send`, native* shape.
- [x] **The Cloudflare skeleton**, `happenstance-cloudflare` — a `RefCell`-backed stand-in
      for `SqlStorage` and an error type that is genuinely `!Send`, implementing
      the bare `EventStore` only. This is the shape that decides ES-6, and it is
      the only instrument for the question ADR-0009 asks: does stringifying a
      `JsValue` lose information the caller needs? `worker::Error::JsError(String)`
      suggests it does not, which is a hypothesis, not evidence.

      **Built, and the question it was pointed at was aimed slightly wrong.**
      Stringifying loses a *capability* (reading a field nobody has thought of
      yet), not information any caller needs — because the conflict signal never
      travels in `Self::Error` on any adapter, `AppendError::ConditionViolated`
      having lifted it out. And the premise underneath was half wrong: `JsValue`
      is `Send + Sync` on non-`atomics` `wasm32`, so `worker::Error` was never the
      hazard. `Rc` is, which is why the instrument holds an `Rc<str>`. What
      actually decided ADR-0009 was a fourth finding nobody had scheduled — see
      the exit criteria.
- [x] **`happenstance-ladybug`** — a stand-in write-handle with `lbug`'s
      `Send`/lifetime characteristics but no `lbug` dependency, implementing
      `ProjectionStore`. The *owned-handle, non-SQL* shape. Deferring the real
      dependency keeps a cold C++ build out of the gate until phase 11.

      **Built, and it ships two impls rather than one.** The owned `GraphWriteSet`
      is the hypothesis this phase was told to adopt; `GraphWriteHandle<'a>` is a
      second, *compiling*, GAT-borrowed impl driven across a real `tokio::spawn`.
      So the borrowed GAT does not fail on the `Send` flavour, and §4.2's
      rusqlite-derived argument for dropping it does not generalise. **Phase 6
      inherits that as an input rather than a settled question.** A third shape —
      a non-`'static` store — produced a reproducible **rustc ICE**, minimised and
      recorded in [`adapter-shapes.md`](references/adapter-shapes.md) §6.
- [x] **`happenstance-postgres`** — `sqlx` behind a pool, a real pooled-cursor
      `read` stream, and `type Batch = sqlx::Transaction<'static, Postgres>`, which
      owns its `PoolConnection` and is `Send`, unlike `rusqlite::Transaction<'a>`.
      The *networked, pooled, non-serialising* shape, and the only one on the
      roadmap that can violate the visibility invariant.
- [x] **`happenstance-neon`** — one-shot HTTP against Neon's `/sql` endpoint. **No
      connection, no transaction handle, no cursor, one round trip per operation,
      64 MB on a response.** Implements the bare `EventStore` and compiles for both
      the host and `wasm32-unknown-unknown` — note that this means it *compiles for
      both targets implementing the bare flavour on each*, not that it satisfies
      both flavours; `store.rs:17-18`'s implication table is what makes those
      different claims (`PRESSURE-TEST.md:445-450`).
- [x] **A `SyncPeer` sketch** in `happenstance-sync`, plus a `MemorySyncPeer` that
      compiles. Not the protocol — just enough of a trait to learn whether a peer
      can be stated without naming a transport, given that the two real peers are a
      Durable Object over a socket and a Postgres over one-shot HTTP. If it cannot
      be, that is phase 13's most useful input and it is worth knowing eleven
      phases early. This is also the mitigation for the leak warning above.

      **Built, and then trimmed, because the probe overshot into phases 4 and 5.**
      The answer it found is the deliverable: a peer *can* be stated without
      naming a transport — `pull` returns a bounded batch and an owned resume
      token rather than a stream — and **the type checker did not force that
      choice**, which is the more useful half. What was trimmed: the three
      identity types (`StoreId`, `EventId`, `RecordedAt`) are phase 4's under
      VT-4 – VT-10 and `EventId` is `[FROZEN]` at VT-5, so they are no longer
      re-exported from the crate root — a peer adapter that also imports from
      `happenstance` would otherwise have had two `EventId`s in scope with a plain
      `use` picking whichever came first. And the `Serialize`/`Deserialize`
      derives came off nine public structs: a `#[derive]` on a public struct with
      no version field *is* a wire format, WF-8 puts a version first, and phase 5
      owns it. Nothing in the crate serialised anything, so the derives committed
      it to a shape nobody had authorised and bought the sketch nothing.

      **The demotion the plan called for was not available**, and the reason is
      worth keeping: `ReplicatedEvent` and `Watermark` carry all three identity
      types in their public fields, so `pub(crate) mod identity` would have gutted
      the peer port rather than scoping a placeholder. Withdrawing the re-exports
      achieves the same thing — the collision becomes unwriteable by accident, and
      every site phase 4 must revisit is greppable by one name.
- [x] Write `references/adapter-shapes.md`. For each skeleton: the flavour it
      implements, its `Error`, its `Batch`, its stream type, **the most ambitious
      signature attempted and its outcome**, and — a second kind of row —
      **capability** limits that are not type errors. `happenstance-neon` will
      compile against signatures it cannot honour (an interactive `append` probe, a
      streaming `read`), and a table recording only `error[E….]` would show it as
      the most compatible adapter in the workspace when it is the least.
- [x] Fill in the [portfolio table](#the-instrument-portfolio)'s `Far end exists`
      column with what a *skeleton* proves and what it does not. A skeleton
      falsifies a signature; it does not fill a far end, because a far end is a
      *passing* implementation.

**Proof artefact.** **Six crates that compile on their real targets with real
associated types** — no `type Error = ()`, no `type Batch = ()`, no stream stubbed
as `Pending`, because a skeleton that stubs its associated types has stubbed the
only part of it a type checker can disagree with. Plus three named signature
attempts, each landing as either a quoted compiler error or a compiling call site:
`Batch<'a> = rusqlite::Transaction<'a>` on `SendProjectionStore`, a two-statement
probe-then-write `append` on `happenstance-neon`, and an `Error: Send + Sync` on
the Cloudflare skeleton.

`references/adapter-shapes.md` is where all of that is written down, and it is the
evidence base for ADR-0009 through ADR-0017 — but **the document is not the
proof.** A document that records outcomes exists whatever the outcomes are; the
previous revision named it as the artefact and thereby named something that
survives any result. What cannot be faked is six type checkers agreeing, and the
capability table's second kind of row, which records the limits that are *not*
type errors: `happenstance-neon` will compile against signatures it cannot honour,
and a table showing only `error[E….]` would rank it the most compatible adapter in
the workspace when it is the least.

**And one measurement, which is not a skeleton.** Position allocation is the axis
the pressure test and all six scenarios ranked first, and it is the only one where
what is missing is a number rather than a type. `nextval()` allocates outside the
transaction, so a Postgres store violates ES-10's visibility invariant by
construction unless it buys its way out, and the three candidate mechanisms —
`xid8` + `pg_snapshot_xmin`, transaction-scoped advisory locks, a serialised
sequence table — each cost something real. Run all three against CF-13's hostile
fixture on a throwaway Postgres and record pass/fail and cost. This is a probe,
not the adapter: `happenstance-postgres` is still phase 10. It sits here because
ES-10 is frozen at phase 4 and an invariant nothing can afford is not an invariant
— and discovering that at phase 10 means discovering it after the alpha.

**Exit criteria**

- [x] Six skeletons compile: four on the host target, the Cloudflare one on
      `wasm32-unknown-unknown`, `happenstance-neon` on both. **And the two
      `wasm32` claims are now guarded rather than asserted** — until this phase,
      both crates stated that target in their own rustdoc and no gate step built
      them for it, which is the decorative-gate shape in its purest form. `cargo
      xtask ci` carries `wasm32 build of the Cloudflare adapter` and `wasm32 build
      of the Neon adapter` as mandatory steps, selected by name.
- [x] For **each** skeleton, `references/adapter-shapes.md` records the most ambitious
      signature attempted and its outcome — a rejection with its compiler error, or
      agreement. Both outcomes are results. (The previous plan required a rejection
      per skeleton, which is an exit criterion that mandates its own outcome;
      `sqlx::Transaction<'static, Postgres>` agreeing with the owned-batch shape,
      from a networked pool that had every opportunity to hand back a borrowed
      handle, is exactly the kind of finding that rule would have suppressed.)
      That prediction paid: `sqlx` **agreed**.
- [x] At least these three were attempted, because they are the ones later phases
      spend: `Batch<'a> = rusqlite::Transaction<'a>` on `SendProjectionStore`; an
      `append` that probes and writes in two statements on `happenstance-neon`; and
      an `Error` carrying `Send + Sync` on the Cloudflare skeleton.

      All three, and **each is backed by an artefact rather than by prose.** The
      probes arrived carrying two of the three as assertions with no transcript
      and one as an uncommitted working-tree edit that any checkout would have
      destroyed; all three were re-run and pasted. One by-product is worth
      promoting: the six diagnostics rejecting the rusqlite GAT carry **no error
      code** — `--message-format=json` reports `code: None` on every one. That
      generalises ADR-0008's PS-36 finding from one diagnostic to the whole
      `Send`-obligation family, and it means no `compile_fail,E….` doctest can pin
      any of them. `trybuild` is the only mechanism, and it stays phase 6's
      decision.
- [x] ADR-0009 written, or ES-6 restated as deferred with the compiled reason.
      [ADR-0009](.kb/decisions/0009-error-send-sync.md) — **written, and it settles rather
      than renews.** `Error` keeps its bound; the strength moves into a marker
      trait. Note what decided it, because it was not the question the phase body
      asked: the *derived* flavour does not imply a `Send` error either, so ES-6's
      named rule `store_error_crosses_a_join_handle` was **unwritable against
      today's port for every adapter**, not merely for the `!Send` one. A deferral
      behind an impossible experiment is not a deferral. The marker makes the rule
      writable, the wrong implementation it must reject already exists in the tree,
      and — the part nobody predicted — **it works from a downstream crate, so
      `happenstance-core` need not change at all.**
- [x] **The position-visibility probe has produced a number.**
      [`experiments/position-visibility/`](experiments/position-visibility/README.md)
      records, for each of `xid8` + `pg_snapshot_xmin`, transaction-scoped advisory
      locks and a serialised sequence table: whether it defeats **the two-connection
      inversion probe that the unguarded baseline demonstrably fails**, and what it
      costs — write throughput against an unguarded baseline on the same instance.
      A mechanism that passes is enough to lift ES-10 from `[PROVISIONAL]` at phase
      4; three that do not is the finding that reopens ES-25 and ES-26 with it.

      **The fixture clause is amended, and only the fixture clause** — not the
      measurement, not the threshold. It read "whether it makes CF-13's hostile
      fixture pass", and three things were wrong with that. The fixture does not
      exist: neither spelling of the rule, nor `PreCommitPositionStore`, has a
      single hit in any crate. Its ownership is disputed on the record — CF-13's
      own deferral marker (`SPECIFICATION.md:5767-5774`) assigns it to "the
      instrument-portfolio pass", which is this phase, while the [deferred-clause
      table](#the-10-deferred-clauses) assigns it to phase 3. And it would have
      been the **wrong instrument** either way: `PreCommitPositionStore` is an
      in-memory Rust mutant and you cannot run `pg_snapshot_xmin` against one.
      What this probe needs from CF-13 is the *predicate*, not the store that
      fails it — and Postgres running `nextval()` unguarded **is** the hostile
      fixture, failing the predicate natively with no help from the testkit.
      Phase 3 still owed the mutant, the rule and its one settled spelling; it
      wrote them at its stage 4, and this probe's predicate is what
      `nothing_below_an_observed_position_appears_later` is equivalent to —
      reached, as predicted, without any Postgres. CF-13 is `[FROZEN]`.

      **Answered: `xid8` + `pg_snapshot_xmin` buys ES-10 at 0.99–1.03× baseline,
      blocking nobody.** So ES-10 lifts at phase 4 and **ES-25 and ES-26 are not
      reopened.** Three things phase 4's freeze must carry with it, none of which
      is a throughput number: arm C's cost is *structural* — `head` becomes a
      frontier, read-your-own-writes does not hold, and staleness is bounded by
      the longest open write transaction **anywhere in the cluster** (a 5 s write
      in an unrelated database moved it from 0.7 ms to 4,010 ms); the two
      serialising mechanisms are correct and 16×/30× slower at 64 writers, which
      buys the invariant by deleting the reason the adapter exists; and **B-tag
      is cheap because it buys a per-boundary invariant where ES-10 states a
      global one**, which is a clause question rather than a measurement and is
      left open on purpose. Arm C also pushes *nothing* back toward the port,
      where B-tag would have.

      **A methods finding, because it nearly produced the wrong answer.** The
      sequential design this criterion specified — arms first, baseline re-run
      last to bound drift — *failed*: the baseline moved 2.7× at one client and
      3.0× at 64, larger than two of the three effects. The published numbers use
      a **paired** design, re-measuring the baseline between every pair of arms.
      The discarded pass is kept as evidence rather than deleted. Any later
      benchmark in this repository should assume the same instability.
- [x] `cargo xtask ci` green, with `clippy::todo` allowed per-crate in the
      skeletons only. Fifteen steps, none skipped locally, and two of them new.
      `cargo hack check --workspace --no-dev-deps --rust-version` passes at 1.97.1
      across all eleven packages — see ADR-0029 for why that is the number.

**Cases this makes writable.** E2E-54 (a store chosen at runtime — the skeletons
decide whether `append`'s argument type is erasable by `dynosaur`), and the
evidence half of E2E-24 (a `Batch` need not be a live transaction).

**Estimate.** 6 days — 5 for the skeletons, 1 for the position-visibility probe.

**Session log**

- 2026-08-07 — **phase 2 closed.** Gate green with fifteen steps, none skipped —
  two of them new, and both guarding a target that two crates had been claiming in
  their own documentation with nothing checking it. Six skeletons compile on their
  real targets with real associated types. `references/adapter-shapes.md` is the evidence
  base; ADR-0009 settles ES-6; the position-visibility measurement produced a
  number and ES-10 is affordable.

  **The wave was already on disk.** Six probe worktrees from an earlier session
  had built all six skeletons and none of them had been landed, so this session
  was harvest-and-close rather than build. That is phase 1's method working
  exactly as its session log describes, and it repaid its own warning: **two
  artefacts existed only as uncommitted working-tree state that any checkout would
  have destroyed** — the ES-6 `Send + Sync` experiment, which is one of the three
  attempts the exit criteria name, and the rustc ICE reproduction. Both were
  captured first, before anything else was touched. Add to the next wave's brief,
  beside "state the commit you compiled against": **commit your evidence, or it is
  not evidence.**

  **Five things this phase assumed and compilation refuted.**

  1. **The `Send + Sync` bound is nearly free.** Every crate in the workspace
     compiles with it except the one the two-flavour design exists for. The
     question was never "what does it cost" but "what does the one exception
     mean".
  2. **`JsValue` is `Send + Sync` on the target builds.** `wasm-bindgen` carries
     `unsafe impl`s gated on `not(target_feature = "atomics")`, and Workers builds
     without atomics. ES-6's premise — that a `JsValue` error is the hazard — is
     half wrong. `Rc` is the hazard, and `unsafe_code = "forbid"` means an adapter
     can only ever *inherit* that escape hatch, never write it.
  3. **The derived flavour does not imply a `Send` error.** A store satisfying
     every `Send` obligation the flavour states, with a `!Send` `Error`, compiles.
     So ES-6's named rule was unwritable against today's port for **every**
     adapter. This is what decided ADR-0009, and no document had scheduled it.
  4. **The borrowed GAT does not fail on the `Send` flavour.** Ladybug ships a
     compiling one, driven across a real `tokio::spawn`. §4.2's argument for
     dropping the GAT is rusqlite-derived and does not generalise — the rusqlite
     rejection is about `Connection` being `!Sync`, not about GATs. Phase 6
     inherits an input, not a settled question.
  5. **The benchmark design this file specified was not sound.** Sequential arms
     with the baseline re-run last: the baseline moved 2.7×–3.0×, larger than two
     of the three effects. The paired design that replaced it is what the numbers
     came from, and the failed pass is kept as evidence.

  **Three findings that outlive this phase.**

  *`code: None`.* The six diagnostics rejecting the rusqlite GAT carry **no error
  code at all**. ADR-0008 found this for one diagnostic (PS-36); it holds for the
  whole `Send`-obligation family. No `compile_fail,E….` doctest can pin any of
  them, which makes `trybuild` the only mechanism and leaves that squarely with
  phase 6.

  *A rustc ICE, minimised.* Recorded in `adapter-shapes.md` §6. It duplicates the
  open rust-lang/rust#158983, and the minimisation here is **smaller than the one
  upstream** — two crates, no dependencies, and neither `async` nor `Send`
  required. Five ingredients are each independently necessary, and one of them is
  **`where Self: 'a` on the port's GAT**. If phase 6 drops the GAT, the
  workspace's exposure goes with it. Reproduces on 1.85.1, 1.97.1 and nightly:
  not a regression, not fixed. Reported upstream as
  [a comment](https://github.com/rust-lang/rust/issues/158983#issuecomment-5218463761)
  on the existing issue rather than as a new one, and the reproduction lives in
  [`experiments/rustc-ice-gat-foreign-trait/`](experiments/rustc-ice-gat-foreign-trait/README.md)
  with a script that regenerates the bisection — because the table is an **input
  to phase 6's decision**, not trivia, and a finding that only a session
  remembers is one phase 6 will have to re-derive.

  *The MSRV moved, and the way it broke is the lesson.*
  [ADR-0029](.kb/decisions/0029-msrv-raised-to-1-97-1.md) raises it to 1.97.1.
  `libsqlite3-sys` uses `cfg_select!` in a **build script** and declares no
  `rust-version` — and neither do `rusqlite`, `sqlx`, `sqlx-core` or
  `sqlx-postgres`. Five of five. So `cargo hack --rust-version` cannot protect a
  floor against a database driver and `resolver = "3"` cannot either; only running
  the compiler finds it. Two consequences land immediately: **let-chains are now
  available** (CLAUDE.md's constraint 5 is rewritten), and the `msrv` CI job now
  runs the same compiler as the gate and proves nothing until the pin and the
  floor diverge again.

  **What this phase found that a later one owns.** Fourteen `file:line` citations
  in `SPECIFICATION.md` were re-anchored, and **fourteen more were left alone
  because the claim around them is now false** — the port with "zero implementers"
  has five, `happenstance-sync` is no longer "a doc comment and a one-variant
  enum", the metadata-identity proposal those clauses reject no longer exists, and
  ES-7's "the bare flavour has zero implementers" is contradicted by four. None is
  a line number; each needs new words. They are batched into **phase 3**, which
  already owes ADR-0008's four amendments and is opening §6 and §7 anyway. Note
  that `spec-trace` passes over all fourteen and always would: `check_citations`
  validates existence and a first-number bound, not that the cited line still says
  the cited thing.

---

## Phase 3 — The suite becomes an instrument

**Goal.** Turn 27 examples into a suite that can be *shown* to reject a wrong
adapter, and change the fixture so the questions nobody can currently ask become
askable.

**Why here.** It depends only on phase 1's registry, so it does not wait on the
skeletons — and every rule written after the signatures freeze is a rule written
by someone who already believes the signatures are right.

**Decisions it settles.** ADR-0010 (CF-1 – CF-29).

**The honest cost of running this before phases 4 and 5.** Two costs, and only one
of them is mechanical.

**Mechanical.** Every rule is re-spelled when `read` takes its query by value and
`append` takes its events by value. A diff, and the price of keeping this phase
off the skeletons' critical path.

**Not mechanical.** Three rules here have *content* that phases 4 and 5 supply,
and writing them now means writing a placeholder and revisiting it:
`read_limit_applies_after_filtering` depends on D5's `limit: Option<usize>` and on
`limit(0)` returning nothing (VT-28, phase 4); the value-edge rules for the four
guaranteed minima assert numbers phase 4 fixes (VT-21 – VT-24 — and note that
this sentence used to say "the 1 MiB payload", which is not what VT-21 says: the
clause says 65,536 bytes and the clause wins); and
`empty_batch_is_refused_before_the_condition_is_evaluated`
encodes ES-19 and ES-20, whose ADR is 0012. Write all three against the
specification's clauses rather than against the code, since the clauses are
already settled and ADR-0012 and ADR-0015 are recording them rather than deciding
them — and record in this phase's session log that they are the three to re-check
when 4 and 5 land. The runbook's protocol says ADRs come before the code they
constrain; these three are the exception, and an exception that is named is not a
violation.

**Work**

- [x] **The mutant registry.** *Landed at stage 3 of phase 3, extended at stages
      4 and 6.* Fifty-two stores drive the suite: **fifty mutants and two
      conformant variants**, declared as data in
      `crates/happenstance-testkit/tests/mutation_coverage.rs` and checked by
      eight meta-tests, six of which are CF-1 – CF-5 and CF-18 and two of which
      belong to CF-22's two new families. `RACERS` is a second table of six
      `Send + Sync` stores for the concurrency family, kept separate because every
      store in it fails no rule of the event-store family and
      `mutant_registry_is_exhaustive` rejects a row with an empty `fails` list.
      Every one of the fifty-five rules has at least one mutant.
      **The re-entrancy pair is in**: `BorrowHoldingStore` and
      `AwaitAcrossBorrowStore` moved out of `local_conformance.rs`'s
      `#[should_panic]` drivers at stage 4, in the same change as ES-36's two
      rules, and that file's `mutants` and `reentrancy` modules are both gone.
      **What is still owed** is `CachedMaxPositionStore` — no rule written at
      stage 4 produced the shape it needs, two *successful* appends through two
      handles, and inventing one to host it is what ADR-0010 forbids. The
      mid-batch fault landed at stage 6, and not as the decorator this item
      predicted: the injection is the fixture's (`MID_BATCH_FAULT`), because a
      decorator sits above `append` and `append` is the unit the port makes
      atomic.

      **One entry in the "known set" below is not a mutant, and finding that out
      is a result rather than a shortfall.** `COUNT(*)+1` position allocation is
      what `MemoryEventStore` already does, and `MemoryEventStore` is conformant:
      in an append-only log with no pre-existing gaps, `COUNT(*)+1` and
      `MAX(position)+1` are the same function, and no fixture that starts empty
      can separate them — the separating input is a store that has had history
      removed, which is ES-39's deferred surface and phase 14's. Its *observable*
      sibling is `CachedMaxPositionStore` — a per-handle cache of
      `max(position)`, which is a genuine defect and is the entry above's real
      form — and that is the one still owed a rule, which is why it appears in
      "what is still owed" rather than in the registry. `REGISTRY`'s own doc
      comment records both, so the next reader does not re-derive them from this
      line.
      The original wording of this item follows.
      Every rule gets a mutant store that fails it, and
      every mutant declares exactly which rules it fails (CF-1 – CF-5). Note what
      ADR-0010 makes load-bearing: **exactness**, not merely rejection. A mutant
      that fails everything is what you get by accident and proves nothing about
      the rule it was written for; and `conformant_variants_pass_everything` is
      the positive control without which the whole registry is satisfied by a
      harness that reports failure unconditionally. Each mutant
      states its provenance: the real implementation mistake it models. The known
      set, four of which a reviewer measured *passing* the current suite:
      `LIMIT` applied before the tag filter; an OR-ed query returning a matching
      event twice; `COUNT(*)+1` position allocation, observable only across a
      reopen; probe-then-insert outside the transaction; `metadata` `None` and
      `Some(empty)` conflated; `from` handled as an index rather than a threshold.
      Do **not** quote a pass rate over an author-chosen bug set — it is a
      selection artefact and carries no information.
- [x] **`PreCommitPositionStore`** — *landed at stage 4 of phase 3, with
      `nothing_below_an_observed_position_appears_later`.* Positions allocated
      before commit, so a slow writer's event becomes visible *below* a position a
      reader has already observed. It is the one mutant whose bug is a faithful
      model of a real database rather than an implementation slip: it is what
      Postgres does by default. **If it passes the suite, the visibility
      obligation is decorative and this phase is not done.** It does not pass:
      the rule creates two `append` futures from one handle, polls them A, B, B, A
      and reads after every step, and the mutant is rejected at the ordering
      assertion. Two things the criterion did not ask for and are worth recording
      anyway. The `Send + Sync` sub-trait CF-13's marker held in reserve **was not
      needed** — nothing is scheduled, so nothing needs to be `Send`. And the
      reversal on resumption is load-bearing, measured rather than argued: under
      plain A, B, A, B alternation the mutant commits in allocation order and
      **passes**, which `mutants_fail_exactly_their_declared_rules` reports as the
      rule failing to catch the defect it was written for. CF-13 is `[FROZEN]`;
      its **adapter** far end is still phase 10 and is §6.5's position-allocation
      row.
- [x] **The append-condition path must require tags** (CF-7, CF-8). *Landed at
      stage 4.* `condition_matches_on_tags` and
      `condition_with_an_unheld_tag_does_not_reject`, with
      `ExactTagMatchConditionStore` and `TagBlindConditionStore` beside them.
      The first rule's store carries an **extra** tag the condition does not
      name, which is what separates the two defects: without it the exact-match
      probe returns the correct verdict and only CF-8's defect is caught. The
      original wording follows.

      Every
      condition in the suite today is built from `query_of_types`, and the one
      tagged condition runs against a store where a type-only probe returns
      identical verdicts (`suite.rs:600-621` in `b4b593d`). An adapter that drops the tag join
      from its condition probe — the natural first cut, since the join is the
      expensive half — passes the whole suite while rejecting *every* command that
      touches any course. That is a total-availability failure certified as
      conformant, and it is the canonical DCB uniqueness shape. Two rules: two
      events sharing a type and differing in tags with a condition tagged for one,
      asserting rejection; and its mirror, asserting acceptance for a tag no event
      carries.
- [x] `duplicate_items_do_not_duplicate_events` (CF-9). *Landed at stage 4*, with
      `TagJoinFanOutStore`, plus `untagged_events_match_query_all` and
      `query_item_order_does_not_change_the_result_set` from the same clause.
      The fan-out is modelled only where no item constrains tags, because the
      tag-AND path has to be `GROUP BY … HAVING COUNT(*) = n` and the grouping
      collapses duplicates for free — so the defect lives exactly on
      `Query::all()`. `query_all_matches_every_event` keeps its `Retires:` line at
      ES-15 and the untagged rule carries the half it was accidentally good at.
      One finding: ES-15's own `Rejects:` names an adapter that *sorts and dedups
      items*, and no order-invariance rule can catch that — sorting is what makes
      order stop mattering. `ItemOrderedUnionStore`, whose output order is the
      item order, is the mutant that earns the rule. The original wording
      follows.

      A store holding a
      *multi-tagged* event, so a tag join without `DISTINCT` fails. Name the
      disposition of the existing `query_all_matches_every_event` in the same
      change: it appends three untagged events, so it cannot produce the fan-out,
      and leaving it alongside its strengthened successor leaves an unowned rule
      (`SPECIFICATION.md §7.4`).
- [x] `read_limit_applies_after_filtering` and its backwards mirror. *Landed at
      stage 4* with `LimitBeforeFilterStore`, laid out `Miss, Hit, Hit, Hit,
      Miss` so that a `LIMIT` pushed into the scan returns one match from either
      end. The original wording follows.

      The existing
      `read_limit_truncates` uses `Query::all()`, so `SELECT … WHERE type IN (..)
      LIMIT 2` followed by in-memory tag filtering — the classic event-store bug —
      passes today.
- [x] A condition evaluated against an **empty** store (CF-10); `after` at and
      beyond the head; `after` beyond the last *matching* position while matching
      events exist — the clause the DCB specification singles out with a Note and
      the one with zero coverage. *All landed at stage 4*, as
      `condition_against_an_empty_store_admits_the_append`,
      `condition_after_beyond_head_admits_the_append` and
      `condition_after_beyond_the_last_matching_position_admits_the_append`, with
      `NullAggregateProbeStore`, `AfterValidatedAgainstHeadStore` and
      `UncorrelatedProbeStore`. The *at* half of "at or beyond" got no rule and
      ES-28 says why: `condition_after_ignores_events_at_the_boundary` already
      runs it, and a second rule over it is one no adapter could fail.
      `reading_an_empty_store_yields_nothing` landed with them, against
      `NullHeadPagingStore` — the ES-11 paging shape decoding a `NULL`
      `max(position)` — which settles whether that rule had a plausible failing
      implementation. It did, and ES-9's own `Rejects:` had already named it.
- [x] `read_from_composes_with_multi_item_query` (CF-12). *Landed at stage 4*
      with `UnparenthesisedPredicateStore`, and the SHOULD was taken: `from`,
      `backwards` and `limit` are all exercised against one two-item query in the
      one rule, because the measured finding is one gap and three rules would
      have suggested three. The original wording follows.

      `from` commutes with
      filtering semantically but not in generated SQL: `WHERE a OR b AND position
      > ?` without parentheses is a textbook precedence bug.
- [x] **Fix D8**: `empty_batch_is_refused_before_the_condition_is_evaluated`
      (CF-11). *Landed at stage 4, and it had **three** sites rather than the two
      expected*: `MemoryEventStore`, `LocalMemoryEventStore` — which copied the
      ordering deliberately, and whose comment moved with the fix — and the
      mutant registry's own correct core, which had copied it a third time.
      `NoEvents` is now documented on `EventStore::append`, which it was not.
      `ConditionBeforeEmptinessStore` is the old order, kept as the mutant. The
      original wording follows.

      `memory.rs` evaluated the condition first, so
      `append(&[], Some(&c))` returns `NoEvents` or `ConditionViolated` depending
      on store contents, and a caller whose retry loop branches on
      `is_condition_violated()` loops forever. Emptiness is a precondition on the
      argument and is checked before the lock is taken.
- [x] **Change the fixture shape** (CF-15 – CF-21). *Landed at stage 2 of phase
      3.* `event_store_conformance!` re-evaluated the factory expression per test
      and each rule called it exactly once, and the macro contractually required
      a fresh empty store, so no rule could hold two handles — which foreclosed
      durability, reopen and genuine multi-connection rules all at once. The
      `Fixture` trait replaced it: one instance is one backing store, each
      `connect()` is a handle onto it, capabilities are associated `const`s, and
      a rule whose capability is declined still runs and reports the skip
      (CF-18). Three rules came with it, 27 → 30, and the testkit's own `tests/`
      carry the wrong implementations that fail them. What is **not** done, and
      is what phase 8 inherits: every fixture in the workspace hands out refcount
      clones of one in-process object, so the far end of the handle-multiplicity
      and durability axes is still a fixture rather than an adapter — §6.5 of the
      specification says so in both rows.
- [x] `event_store_model_conformance!` — a stateful model-based proptest
      generating **symbolic** position anchors. *Landed at stage 5 of phase 3*, in
      `crates/happenstance-testkit/src/model.rs`, with its own enumeration
      (`for_each_model_rule!`) and its own two emitters. Three things the plan
      got wrong, all worth carrying forward:

      **The justification below was false of the tree and had to be built
      first.** "Both are already property-tested against naive definitions" was
      checked and was not true: `Query::matches` was constrained only
      *algebraically* — order-insensitivity, monotonicity, `Query::all` as the
      top element, every one of which a function returning `true` satisfies — and
      `AppendCondition::is_violated_by` had no property at all. The two oracle
      properties (`query_matches_agrees_with_a_naive_definition`,
      `is_violated_by_agrees_with_a_naive_definition` in
      `crates/happenstance-testkit/tests/properties.rs`) were written before the
      model, and each was shown to bite by mutating `happenstance-core` and
      reverting. The grounding sentence is now true, and `model.rs` says which
      side of that line the reader is on.

      **It is not a `proptest!` block.** That macro generates a *synchronous*
      test body, so a rule written inside one would have to pick a `block_on`
      inside the testkit — which is CF-23's prohibition one level down. The rule
      is an ordinary `async fn(open) -> RuleOutcome` driving `TestRunner` by
      hand, including the shrink loop, so the adapter's own emitter still decides
      how it is driven. The RNG is deterministic on purpose: a conformance rule
      that fails one run in twenty teaches an adapter author to rerun CI.

      **Its blind spot is measured, not argued.**
      `mutation_coverage::the_model_rule_rejects_exactly_what_it_claims` pins the
      answer for all fifty-two registered stores: thirty of fifty mutants
      rejected, both conformant variants passed — `GappedPositionStore`
      included, which is the whole symbolic-anchor claim discharged. The twenty
      it misses are three shapes: nine unreachable (empty batch, second handle,
      second fixture instance, durability, and the three whose defect is a
      *window* between overlapping futures), of which three are the concurrency
      family's subject matter; one whose defect only exists once a *fixture* has
      been armed, which is not a store operation and so not an `Op`; and ten
      outside the generators' value range, which stage 6 added and which the
      named value-edge rules cover. The
      original wording follows.

      A stateful model-based proptest
      generating **symbolic** position anchors (`None | First | Middle | Head |
      BeyondHead`) resolved at execution against positions the store actually
      assigned. That is the generalisation of the no-literal-positions house rule
      and what lets one test run against dense and gapped stores alike. Build the
      model on `Query::matches` and `AppendCondition::is_violated_by` — not
      circular, because both are already property-tested against naive definitions
      — and *predict* each conditional append's outcome before calling. Hand-roll
      `Vec<Op>` with `prop::collection::vec`; do not add `proptest-state-machine`.
- [x] `event_store_concurrency_conformance!` *(landed at stage 5 of phase 3;
      `crates/happenstance-testkit/src/concurrency.rs`)*. Five rules, its own
      enumeration `for_each_concurrency_rule!`, two emitters, invoked under both
      from `tests/memory_concurrency_conformance.rs`. **Four things about it
      differ from the wording below, and each is a finding rather than a
      shortcut.**

      **The timeout is deliberately not asserted, and CF-33 is why.** The item
      asked for *every task terminates within a timeout*; it is not written and
      must not be added. **CF-33 is `[FROZEN]`** — no conformance rule may read a
      clock, measure elapsed time, or assert on an operation count — and its own
      `Rejects:` paragraph is the argument verbatim: a deadline passes on the
      author's machine, fails on a loaded runner, fails under a debug build, and
      makes the suite's verdict a property of the hardware. A watchdog is the
      most tempting clock in the whole suite because it looks like a safety
      feature rather than an assertion. **Liveness rests on the CI job timeout**,
      which is the trade `harness.rs` and ES-10 already made twice; the cost — a
      hung job names no rule — is stated in the module documentation rather than
      discovered.

      **The bound is `F::Store: EventStore + Send`, not
      `SendEventStore + Send + Sync + 'static`.** Three of the four predicted
      bounds are not needed, measured by removing each and asking the compiler
      (`spawns_from_generic`'s method). `Sync` and `'static` are properties of
      `tokio::spawn` rather than of the port, and scoped threads need neither;
      `SendEventStore` is not needed because **the future never crosses a
      thread** — the handle does, and the future is created and driven on the
      contender's own thread by `block_on`. So the family binds the weaker
      flavour, as CLAUDE.md's fourth constraint asks. The direction this can
      move is the safe one, and the circumstance is named: if phase 10's `sqlx`
      adapter needs its contenders to be tasks on a harness-supplied runtime,
      the bound tightens toward CF-22's prediction.

      **"K seats, exactly K commits" is written as K disjoint boundaries.** The
      capacity form needs a retry loop, a retry loop needs a budget, and
      asserting on an exhausted budget is asserting on an operation count —
      CF-33 again. K disjoint boundaries buy the same arithmetic with one
      attempt each, and reject a defect the capacity form cannot see: an adapter
      whose concurrency control is coarser than its condition, which is what a
      `SERIALIZABLE` store mapping `40001` onto `ConditionViolated` is.

      **`racing_conditional_appends_elect_one_winner` is NOT retired.** Its
      disposition is named, as this item asks — and the name is *retained*.
      §7.4 claims it for ES-25 on the strength of the three mutants it rejects.
      The family is additive and says so in its own module documentation.

      **Two later phases name a number this stage did not set.** Phase 8's and
      phase 10's proof artefacts both read *the concurrency macro green under a
      multi-thread runtime at 64 contenders*. `concurrency::CONTENDERS` is **8**,
      and it is a testkit constant rather than an adapter knob — the rules assert
      *set* properties (exactly one, all distinct) that hold at any size above
      one, so 8 was chosen as the smallest number at which an operating system
      has to preempt somewhere. Neither the number nor the phrase "multi-thread
      runtime" is right as written: the parallelism is `std::thread::scope`, not
      a runtime, and 64 OS threads per rule is a cost those phases should decide
      on deliberately. Whoever reaches phase 8 either raises the constant and
      says why, or amends both proof artefacts. Leaving it is the third option
      and it is the one that rots.

      Assertions written: exactly one winner of N contenders; K disjoint
      boundaries admit exactly K commits; positions unique after N concurrent
      unconditional appends; `append`'s return is the caller's own last event
      rather than the global head; a concurrent reader never sees a partial
      batch. Each is paired with a `Send + Sync` store that fails it and nothing
      else — `RACERS` in `tests/mutation_coverage.rs`, checked in both
      directions by the eighth meta-test,
      `the_concurrency_rules_reject_exactly_what_they_claim`.
- [x] ~~Merge `append_is_atomic` with `condition_rejection_leaves_store_unchanged`
      (they assert the same property)~~ **— struck, and the striking is the
      finding.** The tick is over struck text on purpose: the merge did **not**
      happen and must not, and the second half — the real atomicity rule — landed
      at stage 6. Both halves are disposed, which is what the box now records.
      The merge was ADR-0010's phase-3 amendment and it was *itself*
      amended at stage 3: `WriteThenCheckStore` extends the log and *then* probes,
      so the batch does reach the write path and `append_is_atomic` catches the
      partial write. Three registry rows name the rule. `SPECIFICATION.md`'s
      ES-18 and §7.4 were reconciled to the ADR at stage 4 — the `Retires:` line
      is gone and ES-18 claims both names. What remains is the second half, and
      it is unchanged: add a real atomicity rule using a **fault-injecting
      decorator** that fails the write of the *k*-th event of a batch. The two
      are complementary, not successive: one rejects a store that writes before
      it decides, the other a store that cannot roll back what it wrote.

      **Stage 5 did not land it, and the reason is a seam rather than a
      shortfall.** The concurrency family was the obvious place to look, and it
      is the wrong one: a fault between two rows of a batch is a **decorator over
      a store**, not a race, and the rule that needs it is single-threaded. What
      stage 5 *did* land is the neighbouring half — `RowAtATimeStore` and
      `a_concurrent_reader_never_sees_a_partial_batch` reject a store whose batch
      is *visible* part-written — and that is visibility, not rollback: every row
      lands there in the end. The rule ES-18 still owes is the one where a row
      does **not** land and the store has to undo the rest. It belongs with the
      value edges and the fault instruments at stage 6.

      **Landed at stage 6, and the decorator does not exist.** A decorator sits
      *above* `append`, which is the unit the port makes atomic, so the only
      faults it can inject are before the call and after it; the one shape that
      looks like it works — append `events[..k]`, then return `Err` — is the
      decorator writing a partial batch and then asserting the *store* should
      have undone it. The injection has to be the **fixture's**, and it is:
      `Fixture::MID_BATCH_FAULT` plus `arm_mid_batch_fault(after)`, defaulted to
      declined so an in-memory store answers honestly by saying nothing.
      `GappedPositionStore` supplies it and rolls back; `NoTransactionStore` is
      the same store with the `BEGIN` removed and is the registry row. ES-18 and
      §6.2 both carry the correction.
- [x] Value edges *(landed at stage 6)*: ~~empty payload~~
      (`append_preserves_an_empty_payload`, against `EmptyPayloadIsNullStore`);
      ~~`metadata` `None` versus `Some(empty)`~~
      (`metadata_distinguishes_absent_from_empty`, against
      `MetadataConflatingStore` **and** `DropsMetadataStore`);
      ~~an untagged event matching `Query::all()`~~ *(landed at stage 4 as
      `untagged_events_match_query_all`, against `InnerJoinTagStore`)*;
      ~~max-length tag and event type~~
      (`store_accepts_a_max_length_identifier`, against
      `NarrowIdentifierColumnStore`); ~~non-ASCII~~
      (`store_accepts_non_ascii_identifiers`, against `Latin1IdentifierStore`);
      ~~the guaranteed minimum tag count~~
      (`store_accepts_the_guaranteed_minimum_tag_count`, against
      `PackedTagColumnStore`); ~~a 1 MiB payload~~ — **65,536 bytes**, because
      VT-21 says so and this line's "1 MiB" was never a clause
      (`store_accepts_the_guaranteed_minimum_payload`, against
      `PayloadCeilingStore`); ~~reading an empty store~~ *(landed at stage 4
      as `reading_an_empty_store_yields_nothing`, against
      `NullHeadPagingStore`)*. Two the item did not name landed with them, because
      VT-23 and VT-24 are the same clause family as VT-21 and VT-22 and a rule for
      two of the four minima would have been an arbitrary half:
      `store_evaluates_a_query_at_the_guaranteed_minimum_item_count` (against
      `ChunkedQueryStore`) and `store_accepts_the_guaranteed_minimum_batch_size`
      (against `BatchParameterCeilingStore`).
- [x] **No rule may read a clock** (CF-33), and no rule may assert a literal
      position value (CF-6) — the second protects a `MAY`, and converting a `MAY`
      into a `MUST` by accident is what a conformance suite does most easily.
      *Landed at stage 6, as `cargo xtask lint-clock` and
      `lint-position-literals`, with CF-32's manifest check, CF-29's changelog
      check and §7.4's disposed-rule check beside them — five mandatory steps,
      selected by name, and `cargo xtask lints` runs just those five.*

      Three things worth carrying forward. **CF-33's grep matches code only** —
      comments stripped, string contents blanked — because `concurrency.rs`
      argues *about* timeouts and a byte-level grep fires on the sentence
      explaining why the construct is absent; weakening the check to let the
      prose through would have been the wrong repair, and deleting the prose
      worse. **CF-6's grep found nothing, and one of its two shapes was broken
      when it did**: an off-by-one in the closing-bracket index meant the
      integer-list half matched nothing at all, which the clean first run looked
      exactly like. It was caught by the deliberate failure demonstration and by
      nothing else. **CF-29's check is the weakest of the five and the clause now
      says so** — a keyword test for "names a defect" produced nine false
      positives against nine of this repository's best entries, so the shipped
      check measures prose *volume* per rule named. It still found twenty-five of
      fifty-five rules with no changelog entry at all, which is what the founding
      rules' entry in `CHANGELOG.md` was written to fix.

**Proof artefact.** The mutant registry:
`crates/happenstance-testkit/tests/mutation_coverage.rs` and its module
directory, holding fifty-two subjects — fifty mutants and two conformant
variants — plus `RACERS`' six. `every_rule_has_a_mutant`,
`mutants_fail_exactly_their_declared_rules` and `conformant_variants_pass_everything`
all green, with `PreCommitPositionStore` in the failing column. A suite whose own
test suite demonstrates what it rejects.

**It exists, and the gate holds it there.** `cargo xtask proof-artefact` asserts
all eight meta-test names out of `cargo test -- --list` *before* running them,
because `cargo test` exits 0 on `running 0 tests` and a file truncated to its
attributes would otherwise pass the step its deletion fails. Verified at this
phase's close: `cargo test -p happenstance-testkit --all-features --test
mutation_coverage` reports **8 passed, 0 failed**. The seventh is behind the
testkit's `proptest` feature, which is why the gate's invocation carries
`--all-features` and why a run without it lists seven.

**Exit criteria**

- [x] ADR-0010 written before the code it constrains.
      [ADR-0010](.kb/decisions/0010-the-suite-must-prove-itself.md). Two corrections it
      makes to clauses this phase will re-spell: CF-23's parameterised wrapper is
      justified by **`wasm32` portability, not `Send`-ness** — `#[tokio::test]`
      drives a `!Send` store perfectly well, because `Runtime::block_on` is not
      `tokio::spawn` — and CF-28 must name **`Rc`** rather than `RefCell`, which
      is `Send` and surrenders only `Sync`.
- [x] Every rule has at least one mutant that fails it; every mutant fails exactly
      the rules it declares; the conformant control passes everything.
      *Landed at stage 3 of phase 3.* `GappedPositionStore` (positions in steps of
      seven from 4096) and `PagedStreamStore` (a read stream never ready on first
      poll) are the controls, and `conformant_variants_pass_everything` also
      requires every rule to have *executed* against one of them.
      **Evidence at close:** `every_rule_has_a_mutant`,
      `mutants_fail_exactly_their_declared_rules` and
      `conformant_variants_pass_everything` green over fifty-five rules and
      fifty-two subjects, with `mutant_registry_is_exhaustive` holding `REGISTRY`
      to the stores that exist so the denominator cannot drift from the tree.
- [x] `PreCommitPositionStore` fails at least one named rule. **It fails two, and
      both are pinned by message.** `nothing_below_an_observed_position_appears_later`
      (CF-13, ES-10) and `interleaved_appends_on_one_handle_elect_one_winner`
      (ES-36) are its declared `fails` list in `REGISTRY`, and
      `mutants_fail_exactly_their_declared_rules` — green — is the assertion that
      it fails those and nothing else. The visibility rule discharges it
      deterministically on one thread: two `append` futures from one handle,
      driven **A, B, B, A**, with a full `read` after every step. The pin is why
      `expect` is keyed by `(mutant, rule)` rather than per mutant — that rule has
      two failure surfaces, so an unpinned declaration would certify the
      visibility obligation on a row count.
- [x] The fixture can hand out two handles onto one backing store, and rules that
      need one report a skip rather than silently passing. *Landed at stage 2 of
      phase 3*, matching the work item above. Amended at stage 3: `SECOND_HANDLE`
      is a MUST (CF-16) rather than a trade, so the rule needing it now **fails**
      a fixture that declines it instead of skipping. `REOPEN` is the SHOULD, and
      it is the one that reports a skip.
- [x] The model and concurrency macros run in CI against `MemoryEventStore`.
      *Landed at stage 5.* `tests/memory_model_conformance.rs` and
      `tests/memory_concurrency_conformance.rs`, each under both of its family's
      emitters. `MemoryEventStore` passes the concurrency family for a
      *structural* reason — it is an `RwLock` around a `Vec`, so it serialises
      its writers — which is worth knowing and is not the same as the family
      being non-vacuous. That second obligation is `RACERS`', and the axis still
      uncovered is a store that does **not** serialise its writers:
      `happenstance-postgres` is the instrument, and it is still a skeleton.
- [x] **The two orphaned rules are dealt with in the specification, not only in
      the testkit.** `SPECIFICATION.md` §7.4 records
      `query_all_matches_every_event` and
      `racing_conditional_appends_elect_one_winner` as named by no clause, both
      because they are being retired, and both as defects that CF-38's checker
      will report forever until a clause states their disposition. Retiring them
      in code and leaving the clause space silent moves the defect rather than
      fixing it: amend ES-15 and ES-25 or ES-27 in the same change.

      **They were not retired. Both were reclaimed, and that is a different
      disposition** — ticked anyway, because the defect this criterion names is
      *an unowned rule nobody is responsible for deleting*, and reclamation
      closes it exactly as retirement would have. What was found is that the
      retirement was wrong on the merits in both cases, and in a third the
      criterion did not know about. `query_all_matches_every_event` was rewritten
      to descending event types and is now the only thing in the suite that
      rejects `ORDER BY type, position` on the untagged `Query::all()` path; ES-15
      claims it. `racing_conditional_appends_elect_one_winner` rejects three
      registered mutants, two of them shapes **ES-25**'s own `Rejects:` names by
      hand, so ES-25 claims it — neither of the two clauses this line guessed at.
      `append_is_atomic` was the third, disposed of at ES-18 before this phase
      began and reversed at stage 3. **Three dispositions written, zero survived,
      and every one had been decided by reading rather than by compiling** — which
      is the method §6.1 forbids for adapters, applied to the specification by
      this phase. §7.4 records it, and ADR-0010 §4 carries an amendment saying its
      "there are two" heading is false. There is no live `Retires:` field anywhere
      in the document.
- [x] `cargo xtask spec-trace` reports no dangling `Rule:` name for any rule this
      phase writes — every new rule name is either in `suite.rs` or marked † in
      the specification. Clean at close: *193 clauses (135 FROZEN, 46 PROVISIONAL,
      10 DEFERRED, 2 NON-NORMATIVE), 55 suite rules, 56 e2e cases; no problems
      found; §7.1–§7.2 matches the checker.* Note what it cannot see, because the
      next reader should not mistake a clean run for coverage: `spec-trace` reads
      only `suite.rs`, so the model family, the concurrency family and the eight
      meta-tests are outside its enumeration by construction — `cargo xtask
      proof-artefact` is what names those, and `cargo xtask lints` is what holds
      the five prose-level rules the checker cannot express.

**Cases this makes writable.** E2E-01 (visibility, against a fixture that can fail
it), E2E-06, E2E-08, E2E-09, E2E-10, E2E-32, E2E-55, E2E-56.

**Estimate.** 6 days.

**Session log**

- 2026-08-07 — **phase 3 stage 4.** Fifteen rules and fourteen mutants, 31 → 46
  rules and 27 → 41 registered subjects. Every rule has a mutant; all six
  meta-tests green; `cargo xtask ci` and `cargo xtask wasm` green; `spec-trace`
  clean with §7.1–§7.2 regenerated. Six things worth carrying forward:

  **D8 had three sites, not two.** `MemoryEventStore`, `LocalMemoryEventStore`
  and the mutant registry's own correct core had each written condition-then-
  emptiness, and the third was found only because the new rule ran against every
  store in the binary. That is the argument for the registry as an instrument
  rather than as paperwork.

  **`append_is_atomic`'s retirement is reversed and the specification now says
  so.** ES-18 claims the rule beside `append_is_atomic_under_a_mid_batch_fault`,
  §7.4 records the reversal, and CF-38 gained the mechanism note: **a disposed
  rule satisfies check 6 forever**, so a `Retires:` line is a claim to re-examine
  rather than a filing. Nothing mechanical could have caught it.

  **`condition_rejection_leaves_store_unchanged` got the `before` guard.** Two
  empty reads compare equal, so a store returning nothing passed a rule about a
  store holding still; `InnerJoinTagStore` now fails it. The subject is "the
  store is unchanged", and an unchanged store is only observable if there was
  something there to be unchanged.

  **`Defect` grew a seventh primitive, `select`.** Two real defects couple a
  filter to a read option — `WHERE a OR b AND position >= ?` and `LIMIT` pushed
  into the scan — and `matching` is handed no options while `ordered` is handed
  no query. It is the composed step, so overriding it is still one method.

  **`FailureMode` is one field per mutant and stage 4 hit the limit twice.**
  `EmptyBatchPanicsStore` was reordered so both of its declared rules fail the
  same way; `AwaitAcrossBorrowStore` pins `"borrowed"` rather than
  `"already borrowed"`, because `RefCell` says "already borrowed" for a
  conflicting `borrow_mut` and "already mutably borrowed" for a conflicting
  `borrow`, and it reaches both. The next mutant that fails one rule by assertion
  and another by a store panic is the signal to change the shape.

  **`CachedMaxPositionStore` still has nowhere to live.** None of the fifteen
  rules produced two *successful* appends through two handles. The rule worth
  having is the multi-connection form of `positions_are_unique`; no clause asks
  for one, and writing a rule to host a mutant is the tail wagging the dog.

- 2026-08-07 — **phase 3 stage 4, review findings applied.** Three reviews, nine
  changes that matter:

  **Two more `Retires:` lines were reversed, and three-for-three is the
  finding.** ES-15's `query_all_matches_every_event` and ES-27's
  `racing_conditional_appends_elect_one_winner` were still disposed of while
  living in `suite.rs` and rejecting five registered mutants between them —
  precisely the hole `append_is_atomic`'s reversal had just documented, sitting
  two clauses away, unfixed. ES-15 claims the first; **ES-25** claims the second,
  which is neither of the two clauses whose names its own title evokes: two of
  the three mutants it rejects are shapes ES-25's `Rejects:` names by hand. Every
  disposition this document ever wrote has now been reversed. The method that
  produced all three is the one §6.1 forbids for adapters — reasoning about what
  a rule can reject — and §7.4 now says so and schedules the lint for stage 6.

  **`expect` is per-(mutant, rule) and `StorePanic` is origin-checked.** The
  first was the signal `FailureMode`'s own doc describes, met and filed under
  `None`: `PreCommitPositionStore` — "the one entry whose bug is a faithful model
  of a real database" — lost its pin when it acquired a second declared failure,
  so the visibility claim rested on a `seen.len() == 2` row count. Both rules are
  pinned again, and `AwaitAcrossBorrowStore` pins `RefCell`'s two distinct
  messages instead of the eight shared characters. The second was a plain gap:
  `Assertion` demanded the panic come from `suite.rs`, `StorePanic` demanded
  nothing at all, so any panic containing the needle certified the row — on
  needles that are themselves on `RUNTIME_PANICS`. Both new checks were verified
  by breaking them.

  **A rule introduced a MUST that no clause states.** ES-10's visibility rule
  drives a full `read` at four points where an `append` future on the same handle
  is pinned and unfinished. For a `RefCell` store that panics; for an adapter
  holding one pooled connection across the suspension it **deadlocks**, and the
  suspended future cannot be re-polled because the rule is inside the read on the
  same stack — a hung CI job naming no rule. ES-36 is `[FROZEN]`, so the sentence
  is recorded as prose and owed an ADR; the rule's doc comment carries the
  diagnosis where an adapter author will hit it.

  **The four `is_ok()`-only condition rules grew liveness mirrors.** Each was
  satisfied by a probe that returns `None` unconditionally. The mirror asserts
  `is_err()` and deliberately *not* `ConditionViolated`: which error a rejection
  is reported as is ES-25's MUST, and demanding the discriminant would make
  `ViolationAsStoreErrorStore` fail a rule about tags for a reason that has
  nothing to do with tags. `AfterDefaultsToFirstStore` gained two honest rows.

  **`NullHeadPagingStore` errors instead of panicking**, so
  `reading_an_empty_store_yields_nothing` rejects it at `read_ok` rather than
  surviving it — and ES-9 now records that the rule's *other* half, "yields
  nothing", has no plausible saboteur, which is ADR-0010 §1's disclosure. That
  cost `Defect::select` a `Result` return and `LogError` a third
  unproducible-by-a-correct-store variant.

  **Nine stale citations into `memory.rs` and `store.rs`**, invalidated by
  stage 4's own D8 insertion — which shifted `append`'s body down twenty-two
  lines — are requoted. `check_citations` verifies that the file exists and the
  first number is in range, which is exactly the check all nine passed. The
  historical ones are pinned to a commit, which is the only citation form that
  survives a refactor.

  **One anomaly, recorded because it is unreproduced and the next person to see
  it deserves a record.** During stage 4's review, one run in forty-two of
  `mutation_coverage` reported "`TagBlindConditionStore` failed
  `condition_with_an_unheld_tag_does_not_reject`, which it does not declare" —
  a verdict logically impossible against the committed source, since that rule is
  in that mutant's `fails` list, so `entry.fails.contains(rule)` cannot be false.
  It appeared on a rebuild immediately after an edit was reverted and did not
  recur in sixty-one subsequent runs (thirty direct, twenty single-threaded, five
  via `cargo test`, six forced rebuilds). Every nondeterminism source in the
  binary was audited and none was found — no `Instant`, no RNG, no hash-ordered
  collection, no threads, no memoised `reports()`, panic-hook state thread-local
  — which is consistent with an incremental-compilation artefact rather than a
  flaky rule. **If it recurs, bisect with `CARGO_INCREMENTAL=0`**; if that is
  immune, it belongs beside `experiments/rustc-ice-gat-foreign-trait/` as a
  second toolchain observation and not in the testkit.

- 2026-08-08 — **phase 3 stage 6: the value edges, and ES-18's owed rule.** Nine
  rules and **eleven** mutants, 46 → 55 rules and 41 → **52** registered subjects.
  *(This line first read "nine mutants … 41 → 50"; recounted against `REGISTRY`
  at the phase's close, where `cargo xtask proof-artefact` now prints the row
  count in the gate's own output for exactly this reason. Two of the eleven are
  second saboteurs for rules that already had one — `TruncatingPayloadStore`
  beside `PayloadCeilingStore`, and `ChunkLosingBatchStore` beside
  `BatchParameterCeilingStore` — which is why counting mutants by rules is wrong
  in the direction that undercounts. `DropsMetadataStore` moves the other way: it
  is a stage-4 row that gained a second declaration rather than a new mutant.)*
  All eight meta-tests green; `cargo xtask ci` and `cargo xtask wasm` green;
  `spec-trace` clean at 193 clauses (135 FROZEN, 46 PROVISIONAL, 10 DEFERRED, 2
  NON-NORMATIVE) with §7.1–§7.2 regenerated — **the census did not move**, because
  no clause was added and no marker changed.

  **"A fault-injecting decorator over any `EventStore`" does not exist, and
  ES-18 said it from stage 1 until stage 6 tried to build it.** A decorator sits
  above `append`, which is the unit the port makes atomic: it can inject a fault
  before the call or after it and nowhere between. The shape that looks like it
  works — append `events[..k]`, then return `Err` — is a decorator writing a
  partial batch and then asserting the *store* should have undone it, which tests
  the decorator. So the injection is the fixture's: `Fixture::MID_BATCH_FAULT`
  and `arm_mid_batch_fault(after)`, **defaulted to declined**, unlike
  `SECOND_HANDLE` and `REOPEN`, because an in-memory store has no fault to offer
  and demanding the answer buys boilerplate rather than information.
  `GappedPositionStore` gained the capability rather than a fourth variant being
  written, because `capability_skips_are_reported`'s closing assertion — a
  fixture supporting everything skips nothing — needs one fixture that genuinely
  supports everything, and splitting the axis would have left it asserting over
  nothing. **The capability has no CF clause of its own**, which §6.1 now states
  in terms; phase 4 owns whether it earns one, and the reason stage 6 did not
  write it is that adding a clause moves §1.3's census for a capability one
  adapter has exercised.

  **The minima are asserted from the clauses, not from the code, and here is the
  list phase 4 must re-check.** `MIN_SUPPORTED_EVENT_DATA_LEN` and its three
  siblings do not exist in `happenstance-core`; the four rules carry private
  `const`s in `suite.rs` citing VT-21 – VT-24 by number. When phase 4 freezes
  them, **each of these is a rule that moves with the number**:

  | Clause | Asserted | Rule | What phase 4 owes |
  |---|---|---|---|
  | VT-21 | 65,536 bytes | `store_accepts_the_guaranteed_minimum_payload` | replace the `const` with `MIN_SUPPORTED_EVENT_DATA_LEN`; if the number moves, `PayloadCeilingStore`'s 4 KiB ceiling must stay strictly below it |
  | VT-22 | 64 tags | `store_accepts_the_guaranteed_minimum_tag_count` | as above with `MIN_SUPPORTED_TAGS_PER_EVENT`; `PackedTagColumnStore` packs into 255 bytes and its tags are nine bytes each, so it survives any floor above ~25 |
  | VT-23 | 128 query items | `store_evaluates_a_query_at_the_guaranteed_minimum_item_count` | as above with `MIN_SUPPORTED_QUERY_ITEMS`; `ChunkedQueryStore`'s chunk is 64 and must stay strictly below the floor |
  | VT-24 | 128 events | `store_accepts_the_guaranteed_minimum_batch_size` | as above with `MIN_SUPPORTED_EVENTS_PER_BATCH`; `BatchParameterCeilingStore`'s ceiling is 100 and must stay strictly below the floor |

  Three more items belong to the same reconciliation. **`append_reports_exceeded_store_limits`
  is still unwritten** and is VT-19's, VT-21's and VT-25's — it needs the
  `AppendError::ExceedsStoreLimit` variant phase 4 introduces, and until it
  exists the three value-edge stores above report a capacity refusal through
  `AppendError::Store`, which is exactly VT-25's complaint modelled rather than
  fixed. **`read_limit_zero_yields_nothing` is still unwritable** and VT-28 still
  records why: `ReadOptions::limit` stores a `NonZeroUsize`, so `limit(0)` is
  `None` before any adapter sees it and the input the rule needs is not
  expressible through the API. Stage 6 confirmed the clause and left the rule.
  And **this phase's own body said "a 1 MiB payload"** where VT-21 says 65,536
  bytes; the clause won, the runbook line is corrected, and the discrepancy is
  recorded here rather than silently resolved.

  **Every value-edge rule tags its events, and that is a decision.** An untagged
  event is its own edge and `untagged_events_match_query_all` owns it; leaving
  these untagged would have made all eight rules fail `InnerJoinTagStore` at
  their setup anchor rather than at the property each is named for — five more
  rows of the inflation the registry's own doc comment warns about. One tag per
  event also keeps a fan-out join invisible, which is `TagJoinFanOutStore`'s axis
  and not theirs. Two cross-declarations survive because they could not be
  designed away and are honest: `TagJoinFanOutStore` fails the tag-count rule
  (a sixty-four-tag event can only be read through `Query::all()`, where it fans
  out, or through a tag query, where `ExactTagMatchReadStore` drops it — one of
  the two has to be declared), and `ItemsAreAndStore` fails the query-item rule
  (a 128-item query with one matching item is the OR defect at scale).

  **The model family's blind spot doubled, and the second half is a different
  kind.** It now rejects thirty of fifty mutants. The twenty it misses are three
  shapes rather than twenty defects: nine are *reachability* — one
  handle, one fixture, sequential, non-empty batches, no reopen — one is a defect
  that does not exist until a *fixture* is armed, and the other ten are *value
  range*, because `any_event` generates typical values and
  every value-edge mutant is wrong only at a boundary none of them reach. Two of
  those exclusions were written on the generator at stage 5 with a note that the
  clause owning them had not been written. It has now, and the answer is that
  they are named rules rather than generated cases.

  **An operational hazard, worse than the one CLAUDE.md's neighbours warn about,
  and worth writing down because it looked like data loss.** An interrupted
  `cargo xtask ci` on Windows died inside `cargo hack`'s manifest restore and
  left **every one of the eleven `Cargo.toml` files and `Cargo.lock`
  zero-filled**, and `.git/index` corrupt with it (`error: bad signature
  0x00000000`). No `.rs` or `.md` file was touched. The recovery, in order:
  `mv .git/index …` then `git read-tree HEAD` rebuilds the index from the commit
  without touching the working tree — safe because nothing was staged — and then
  `git checkout --` restores every manifest. Only one manifest differed from
  `HEAD`, and it was reconstructed from a diff printed earlier in the same
  session and verified by blob hash. **Check the whole manifest set, not just
  `xtask/Cargo.toml`**, and check `.git/index` before trusting `git status`.

  **`LogError` grew four variants and every one is unproducible by a correct
  store bar one.** `NotNullViolation`, `ValueTooLarge` and `TooManyParameters`
  model capacity refusals with nowhere to go until VT-25; `WriteFailed` is the
  injected fault, and it is the first variant a *conformant* store in this binary
  can return.

- 2026-08-08 — **phase 3, closed.** The suite is an instrument. Twenty-seven
  rules became **fifty-five** in `suite.rs`, beside a stateful model family and a
  five-rule concurrency family with their own enumerations and their own
  emitters; `cargo xtask lints` walks sixty-one rules across the three files. The
  fixture contract replaced `F: Fn() -> S` with `Fixture` and three capabilities.
  Five gate lints landed, each demonstrated to fail against a deliberate
  violation and then restored, and every one selecting its subject **by name**
  rather than by index. The proof artefact exists:
  `tests/mutation_coverage.rs` and its module directory, **fifty-two subjects —
  fifty mutants and two conformant variants** — plus `RACERS`' six, held by eight
  meta-tests that `cargo xtask proof-artefact` asserts *by name* out of `cargo
  test -- --list` before running them. All seven exit criteria are met.

  Every count in the paragraph above is now printed by a gate step rather than
  computed by a reader, and that is stage 6's own miscount fixed at its cause:
  `proof-artefact` prints the registry's row count, `lints` prints the rule
  count, `spec-trace` prints the census. Prose numbers in this document
  disagreed with those commands the first time they were asked — stage 6's
  registered-subject count, and the header's marker distribution — and a number
  a reader has to recompute to check is a number nobody checks.

  **The three retirement reversals, which are this phase's thesis turned on
  itself.** ADR-0010 exists because a rule that has never been shown to reject
  anything certifies nothing. This phase opened by disposing of three rules —
  `append_is_atomic` at ES-18, `query_all_matches_every_event` at ES-15,
  `racing_conditional_appends_elect_one_winner` at ES-27 — each on a reading of
  what the rule could possibly reject. **All three were wrong, and each was shown
  wrong by compiling a store.**

  * `append_is_atomic`. `WriteThenCheckStore` writes the batch and *then* probes
    and *then* errors, so the batch reaches the write path and this rule is the
    only thing in the suite that sees it. Retiring it would have deleted the
    rule that rejects the commonest shape an adapter takes when its driver's
    convenience API is one statement per call.
  * `query_all_matches_every_event`. Rewritten to append descending event types
    — `Cee`, `Bee`, `Ay` — it is the only rule that rejects `ORDER BY type,
    position` on the untagged `Query::all()` path. Its disposition had been
    argued from a version whose fixture data was in alphabetical order by
    accident, which is precisely the reading error a mutant cannot make.
  * `racing_conditional_appends_elect_one_winner`. It rejects three registered
    mutants, and two of them are shapes **ES-25**'s own `Rejects:` paragraph
    names by hand. ES-25 claims it — neither of the two clauses the exit
    criterion guessed at.

  Zero of three dispositions survived. What makes this the paragraph worth
  keeping is the *method*, not the score: all three were decided by reading a
  rule and reasoning about what a wrong store might do, which is exactly the
  method §6.1 forbids for adapters, applied to the specification by the phase
  that wrote §6.1's argument down. Nothing mechanical could have caught it,
  because CF-38's checker is satisfied by a disposition — **a disposed rule
  satisfies the orphan check forever**, so a `Retires:` line is a claim to
  re-examine rather than a filing, and CF-38 now says so. Stage 6's §7.4 lint
  fails the gate on a disposition naming a rule that is still live, §7.4 records
  all three, and ADR-0010 §4 carries an amendment stating that its own "there are
  two" heading is false in both of its nouns. No live `Retires:` field survives
  anywhere in the document.

  **CF-13 was discharged deterministically on one thread, and the alternative
  was measured rather than argued.** Two `append` futures from one handle, driven
  **A, B, B, A**, with a full `read` after every step: the writer that started
  second resumes first and commits first, so its higher position becomes visible
  while the lower one is still in flight. Patched to A, B, A, B the mutant
  commits in allocation order and the registry reports *declares that it fails …
  but the rule passed* — so the schedule is load-bearing, and it is commented as
  such where a future reader will otherwise tidy it. The `Send + Sync` sub-trait
  CF-13's marker held in reserve was not needed, which also means phase 8 does
  not inherit it.

  **The model's grounding did not exist and had to be built before the model.**
  The plan's justification — "not circular, because both are already
  property-tested against naive definitions" — was checked against the tree and
  was false. `Query::matches` carried only *algebraic* properties
  (order-insensitivity, monotonicity, `Query::all` as the top element), every one
  of which a function returning `true` unconditionally satisfies, and
  `AppendCondition::is_violated_by` had no property at all. The two naive-oracle
  properties were written first and each was shown to bite by breaking
  `happenstance-core` and reverting. `properties.rs` also records which *level*
  is independently written and which is not, because "the model is grounded" is
  true at the item level and only partly true one level up.

  **Two rules were passing for reasons that had nothing to do with their
  subject, and the instrument found both on its first run.**
  `read_defaults_to_ascending_order` asserted `windows(2)` over whatever came
  back, and `windows(2)` over an empty slice is trivially true — so a store
  returning *nothing at all* passed a rule about ordering. It now asserts
  `positions.len() == 3` first, and the visibility rule carries the same anchor
  for the same reason. Separately, CF-29's changelog lint found **two entries
  hidden by substring collisions** on its first run once it matched on word
  boundaries: `append_is_atomic` inside `append_is_atomic_under_a_mid_batch_fault`,
  and `positions_are_unique` inside `positions_are_unique_under_concurrent_appends`.
  One of the two had been reported by no reviewer.

  **The named exceptions, so that nobody has to guess which were deliberate.**

  * `acknowledged_writes_survive_a_reopen` landed although **CF-14 is
    `[DEFERRED]` to phase 8**. Without it, `capability_skips_are_reported` had a
    single gated rule to observe and CF-17's `[PROVISIONAL]` marker was untested.
    Its far end — a store that loses a write to a *fault* rather than to an
    instruction — stays phase 8's, and CF-14's deferred-table row carries the
    same note.
  * **Three rules are written against clauses rather than against code**, which
    the phase body names and which reverses the protocol's usual direction:
    `read_limit_applies_after_filtering` (VT-28),
    `empty_batch_is_refused_before_the_condition_is_evaluated` (ES-19, ES-20) and
    the four value-edge minima rules (VT-21 – VT-24). Phases 4 and 5 supply their
    content, and an exception that is named is not a violation.
  * **`Fixture::MID_BATCH_FAULT` has no CF clause**, and that is a decision
    rather than an omission. ES-18's "fault-injecting decorator over any
    `EventStore`" cannot be built: a decorator sits *above* `append`, which is the
    unit the port makes atomic, so it can inject before the call or after it and
    nowhere between. The injection is therefore the fixture's, defaulted to
    declined, and §6.1 states in terms that it is unclaused. Phase 4 owns whether
    it earns a clause and now carries a work item saying so; stage 6 did not
    write one because adding one moves §1.3's census.

  **What phase 4 must re-check, in one list.** The four minima are asserted from
  private `const`s in `suite.rs` citing clause numbers, because
  `MIN_SUPPORTED_EVENT_DATA_LEN` and its three siblings do not exist in
  `happenstance-core`. Each is a rule that moves with its number, and each has a
  mutant whose threshold must stay **strictly below** the floor or the mutant
  stops being one. The stage-6 entry above carries the same four rows as a table;
  where a number here and a clause disagree, the clause wins.

  1. **VT-21, 65,536 bytes** — `store_accepts_the_guaranteed_minimum_payload`;
     `PayloadCeilingStore` refuses above 4 KiB.
  2. **VT-22, 64 tags** — `store_accepts_the_guaranteed_minimum_tag_count`;
     `PackedTagColumnStore` packs into 255 bytes at nine bytes a tag, so it
     survives any floor above roughly 25.
  3. **VT-23, 128 query items** —
     `store_evaluates_a_query_at_the_guaranteed_minimum_item_count`;
     `ChunkedQueryStore` chunks at 64.
  4. **VT-24, 128 events** — `store_accepts_the_guaranteed_minimum_batch_size`;
     `BatchParameterCeilingStore` stops at 100.

  Four more items belong to the same reconciliation.
  **`append_reports_exceeded_store_limits` is still unwritten**: it is VT-19's,
  VT-21's and VT-25's, and it needs the `AppendError::ExceedsStoreLimit` variant
  phase 4 introduces. Until that exists, all three capacity mutants report a
  refusal through `AppendError::Store`, which is VT-25's complaint modelled
  rather than fixed. **`read_limit_zero_yields_nothing` is still unwritable**:
  `ReadOptions::limit` holds a `NonZeroUsize`, so `limit(0)` is `None` before any
  adapter sees it and the input the rule needs cannot be expressed through the
  API. VT-28 records why, and stage 6 confirmed the clause rather than working
  around it. **VT-14 states a bidirectional-override MUST that nothing
  implements** — `event.rs` and `tag.rs` carry a bare `char::is_control` and no
  bidi codepoint is refused anywhere; the clause now names phase 4 as owner, and
  the four-site "ASCII control characters" doc correction was **deliberately held
  back to land with it**, because that correction argues *from* the current
  wrongness and landing it alone makes three provenance paragraphs false in one
  commit. And **`MID_BATCH_FAULT`'s clause**, above.

  **Two things phase 4 should know before it freezes anything.**

  **`Fixture` cannot say "my `append` needs *n* polls", and the visibility rule
  is weaker against some adapters than others because of it.** The rule drives
  the hostile schedule and then drains whatever is unfinished in the same order.
  Draining is *correct* — an adapter with real I/O under it may take any number
  of polls, and a rule a legal adapter fails is a finding about the rule (CF-6) —
  but it means the interleaving window closes after two polls and never opens
  against a store that needs three. The rule's *strength* therefore varies
  silently with the adapter, and nothing in the fixture contract lets an adapter
  declare its poll count. Freezing CF-16 – CF-21 freezes that — and **no phase
  in this document owned that freeze**, which is why closing phase 3 added a
  phase-4 work item for it. A finding handed forward in a session log nobody is
  told to read is a finding filed, not handed.

  **CF-22's predicted bound for a concurrency family is wrong in three of its
  four parts**, and `concurrency.rs`'s module documentation carries the
  measurement rather than the prediction. CF-22 predicts `S: SendEventStore +
  Send + Sync + 'static` behind an `Arc`; what compiles, with every rule
  unchanged, is `F::Store: EventStore + Send`. `Send` is required and is the whole
  of what makes the family opt-in, because the *handle* is moved onto the
  contender's thread. `SendEventStore` is not required: a future needs `Send`
  only if it crosses a thread boundary, and here the handle crosses while the
  future is created on the contender's thread by `block_on` and finishes there —
  which is a property of the *spawner*, not of the port, and is where CF-22's
  prediction came from. `Sync` and `'static` both fall to `std::thread::scope`.
  Each was removed and the compiler asked, which is the method `memory.rs`'s
  `spawns_from_generic` already documents for `EventStore`. Should phase 10 need
  the contenders to become tasks on a harness-supplied runtime, the bound
  tightens back toward CF-22's prediction — the safe direction, and the reason it
  is recorded rather than hedged against.

  **What the mutant set does not cover.** ADR-0010 requires this list and forbids
  a pass rate over the set: the denominator is the author's choice, so such a
  number reports how representative the author was while appearing to report how
  good the suite is. The axes, then.

  * **No adapter instrument at any far end.** Seven axes in the portfolio, four
    now carrying a *fixture* instrument and three empty at both ends. A fixture
    proves a rule *can fail*; only an adapter proves that a real implementation
    at that end *can pass*. Phase 3 produced no adapter instruments, which is the
    most that phase could produce, and phase 4's freeze inherits the exposure on
    ES-10, ES-11, ES-12, ES-35 and ES-40.
  * **Every subject is in-process.** `RACERS` is `Arc`/`Mutex` stores on scoped
    threads; nothing crosses a connection, a socket or a transaction manager, so
    no adapter's actual isolation level has been tested by anything. And every
    racer bar one still serialises its writers under a single lock — the axis
    genuinely uncovered is a store that does *not*, and that instrument is
    `happenstance-postgres`, which is still a skeleton.
  * **Removed history.** `COUNT(*) + 1` position allocation **is not a mutant**,
    contrary to this phase's own plan. `MemoryEventStore` already allocates that
    way and is conformant, because in an append-only log with no deletions and no
    pre-existing gaps `COUNT(*) + 1` and `MAX(position) + 1` are the same
    function, and no fixture that starts empty can separate them. The separating
    input is removed history — ES-39's deferred surface, and phase 14's. Its
    observable sibling `CachedMaxPositionStore`, a per-handle cache, is the real
    form of the defect and is still owed a rule: the rule worth having is the
    multi-connection form of `positions_are_unique`, no clause asks for one, and
    writing a rule to host a mutant is the tail wagging the dog.
  * **Liveness.** There is no watchdog anywhere in the suite and there must not
    be one. CF-33 is `[FROZEN]` and its own `Rejects:` paragraph is the argument
    verbatim: a deadline passes on the author's machine and fails on a loaded
    runner. A store that deadlocks hangs the binary and the CI job timeout is the
    only thing that notices — the cost is a hung job that names no rule, and it
    is stated here rather than discovered at phase 8.
  * **The model family misses twenty subjects, over three shapes rather than
    twenty defects**, pinned by `the_model_rule_rejects_exactly_what_it_claims`
    so the shapes cannot decay into an anecdote. Nine are *reachability* — one
    handle, one fixture instance, sequential, non-empty batches, no reopen — of
    which three are the concurrency family's subject matter. One does not exist
    until a *fixture* is armed, which is not a store operation and so not an
    `Op`. Ten are *value range*, and that is the largest and the least fixable by
    running longer: the generators emit typical values, so **no value-edge defect
    is reachable however many cases run**. That band is covered by named rules,
    not by generated cases, and it is why stage 6 exists.

  **One number three later phases carry that the testkit does not implement.**
  Phase 8's and phase 10's proof artefacts both read *the concurrency macro green
  under a multi-thread runtime at 64 contenders*, and the status table's phase-8
  row says the same. `concurrency::CONTENDERS` is **8**, and the parallelism is
  `std::thread::scope` rather than a runtime. The rules assert *set* properties —
  exactly one winner, all positions distinct — which hold at any size above one,
  so 8 was chosen as the smallest number at which an operating system has to
  preempt somewhere; 64 OS threads per rule is a cost those phases should choose
  deliberately. Whoever reaches phase 8 either raises the constant and says why,
  or amends both proof artefacts. Leaving it is the third option and it is the
  one that rots.

  **If an interrupted `cargo xtask ci` leaves the tree looking destroyed, this is
  the recipe.** The diagnosis is in the stage-6 entry above and is not repeated;
  the commands are, because they should not have to be reassembled at 2 a.m. The
  symptom is every `Cargo.toml` and `Cargo.lock` zero-filled and `git status`
  refusing to run with `error: bad signature 0x00000000`. In order: move
  `.git/index` aside; `git read-tree HEAD`, which rebuilds the index from the
  commit without touching the working tree and is safe precisely because nothing
  was staged; then `git checkout --` over the manifests. Check the **whole**
  manifest set rather than the one you were editing, and check `.git/index`
  before trusting anything `git status` says. Any manifest that legitimately
  differed from `HEAD` has to be reconstructed by hand and verified by blob hash
  — one did, and was. No `.rs` or `.md` file was touched, which is worth knowing
  before concluding that a day is gone.

  **Two derived tables were wrong, and the second one is the interesting
  failure.** This document's header carried a marker distribution stale by one
  clause — 134 `[FROZEN]` and 11 `[DEFERRED]` where the checker says 135 and 10,
  both distributions summing to 193, which is why it survived every reading. And
  the `[PROVISIONAL]` groups table above covered **41** clauses under a heading
  saying 46. The five it omitted were
  exactly ES-10, ES-11, ES-12, ES-35 and ES-40: the five carrying CF-25's
  residual exposure, and the five phase 4's exit criterion has to cite. A
  phase-12 audit reading that table alone would have found every falsifier
  scheduled and concluded the exposure was owned. Both are fixed, and the two
  sets are now equal by construction against §7.2's generated rows.

  **The citation sweep found the same defect class twice, in the one place no
  checker looks.** Phase 3 grew `suite.rs` from 639 lines to 2,904 and pushed
  `append` six lines down `store.rs`, and the requoting pass that followed
  covered `SPECIFICATION.md` and stopped there. It should not have: twelve
  `suite.rs` citations and ten `store.rs` ones in `references/scenarios/` were left
  pointing at unrelated text — `append_is_atomic` landing inside another rule's
  body, a quotation from `collect`'s own documentation landing on `append`'s
  signature — and every one of them *looks* checked, because the same span is
  correct for `append` a dozen times elsewhere in the same file. All twenty-two
  are requoted in the closing commit, together with seven inside
  `SPECIFICATION.md` that the sweep had missed for the same reason.

  Two things are worth carrying rather than the count. The first is that
  `spec_trace::check_citations` reads `SPECIFICATION.md` **alone**, so nothing in
  the gate could have seen any of the twenty-two — and extending it would have
  caught none of them either, because it verifies that the file exists and the
  first number is in range and every one of them passed both. Phase 4 owns the decision and
  carries a work item stating what extending it would and would not buy. The
  second is the form that does not rot: §6 of the specification declares its own
  measurement to be of `b4b593d`, line numbers included, and pins the two spans
  that matter with `` in `b4b593d` ``. A citation into a *rule body* should be
  pinned that way by default. It is the only citation in the workspace that
  survived a 2,469-line refactor of the file it names.

  **And two documents were still asserting things this phase had disproved.**
  CF-38's body said in the present tense that `query_all_matches_every_event` and
  `racing_conditional_appends_elect_one_winner` "are orphaned because they are
  being retired", two thousand lines above a §7.4 recording that every
  disposition was reversed; `references/scenarios/README.md` said no rule states the
  position-visibility invariant, that the suite has never rejected a concurrency
  defect, and that a parallel rule would need `Send + Sync + 'static`. Both files
  had been edited in the same working tree and both sentences were read past.
  Nothing mechanical could see either: `lint-retired-rules` matches `Retires:`
  *fields*, of which the reversals left none, so it printed green over prose that
  *was* the disposition. That is the phase's thesis pointed a third time at the
  phase — a claim decided by reading, inside the clause defining the mechanism
  that hid it.

  **Gate at close.** `cargo xtask ci` green. `cargo xtask proof-artefact` —
  eight meta-tests present, fifty-two registry rows, 8 passed. `cargo xtask
  lints` — five green over sixty-one rules in three files. `cargo xtask
  spec-trace` clean: 193 clauses (135 FROZEN, 46 PROVISIONAL, 10 DEFERRED, 2
  NON-NORMATIVE), 55 suite rules, 56 e2e cases, §7.1 – §7.2 regenerated. §1.3's
  census was recounted by hand rather than edited to agree, and the recount
  matched. **The trap the next hand recount will fall into is recorded here,
  because nothing in the specification records it**: the naive scan disagreed
  with the checker by exactly one and the checker was right, at **CF-38**, whose
  own marker sits inline at the end of its statement while its body then quotes
  `[PROVISIONAL]` twice in prose about the mechanism. Any first-marker-in-span
  scan reads the prose and lands one clause off. §1.3 is deliberately not
  generated, so it is only ever as good as the method used on it — enumerate
  clause openers in all three forms the document uses (`#### ID — ` in §2 and §3,
  `**ID — ` in §4, `**ID.` in §5 and §6), accept both marker spellings, and
  remember that CF-30 has no opener at all, being the clause demoted to prose.

---

## Phase 4 — Freeze the contract: signatures, value types and identity

**Goal.** Every signature and every semantic promise on `EventStore` settled, each
paired with a rule from phase 3's instrument — *and* the values those signatures
carry: `EventId`, `recorded_at`, the validated identifiers and the store limits.

**Why here.** An API defect discovered after an adapter depends on it is the most
expensive discovery in the plan, and phase 2 has just bought the evidence to avoid
it. The value types are here rather than in their own phase because they are not
separable from it: identity and `recorded_at` land on `SequencedEvent`, which is
`read`'s item type and `append`'s return, so a phase that froze the signatures and
left the values for later would be freezing a shape around a type it had not
finished. The previous revision split them and then had to ask whether the two
halves could run in parallel; the honest answer was that they are one freeze, and
this is that answer applied rather than negotiated.

**What is *not* here.** The wire format. It depends on these types but nothing on
the 0.1 critical path depends on *it* — `serde` is off by default and its only
consumer is replication, at phase 13. It is phase 5, and phase 5 floats.

**Decisions it settles.** ADR-0011 (`read`), ADR-0012 (`append`), ADR-0013
(positions), ADR-0014 (identity and time), ADR-0015 (validated identifiers and
limits). Discharges ES-8 – ES-40, VT-1 – VT-31.

**Work — signatures**

- [ ] **`append` takes its events by value** (D2's family). `&[Event]` forces an
      owning adapter to clone — `Bytes` is a refcount bump but `EventType` is one
      allocation and `Tags` is n+1 — and it makes `Event::into_parts`, whose doc
      comment says it exists to avoid that clone, unreachable through the port;
      `memory.rs:233-235` performs exactly the clone the method was written to
      prevent. Choose between `impl IntoIterator<Item = Event>` and an owned batch
      type on the phase-2 evidence, weighing one specific cost: `impl IntoIterator`
      makes `append` generic, and `dynosaur` — which ADR-0001 names as the escape
      hatch for the missing `dyn EventStore` — cannot erase generic methods.
- [ ] **`read` takes its query by value.** Under RPITIT the opaque return type
      captures every in-scope lifetime including the query's, so a caller cannot
      return a stream from a function, store one in a struct, or spawn a replay
      without keeping the `Query` alive (E0716). `Query` is a `Box<[QueryItem]>`;
      one clone per read is nothing beside the I/O, and every adapter translates it
      to SQL immediately. This must land *before or with* the read-laziness rule,
      which cannot otherwise be written.
- [ ] **`EventStoreExt`**, a blanket extension trait
      (`impl<S: EventStore + ?Sized> EventStoreExt for S {}`) — the
      `Iterator`/`Itertools` pattern. Everything derivable from `read`/`append`
      lives here and can be added later non-breakingly.
- [ ] **Provided `head()` and `count(&Query)` on the port itself** (ES-30, ES-31),
      hand-desugared per ADR-0008. These must *not* go on the blanket ext trait: a
      blanket impl cannot be overridden, and a SQLite adapter wants
      `SELECT max(position)`, not a scan. `head` is not optional — the projection
      runner has to answer "am I caught up?", and ES-31 says the answer is not a
      position difference.
- [ ] `ReadOptions::to` (ES-16, VT-29) — a forward read needs an upper bound, and
      E2E-11 is blocked on its absence.
- [ ] **Fix D5**: `ReadOptions::limit → Option<usize>`, `limit(0)` returns
      nothing (VT-28). Today it silently means unlimited, so a paging loop writing
      `.limit(budget - fetched)` that reaches parity reads the whole log. This is a
      deliberate divergence from the DCB reference implementation, which treats
      `limit: 0` as unlimited through JavaScript falsiness — the ADR must say so.
- [ ] `ReadOptions::after` (exclusive), so resuming a projection is
      `after_opt(checkpoint)` with no arithmetic. `from` is inclusive and
      `AppendCondition::after` is exclusive; the checkpoint recipe at
      `projection.rs:84-93` currently makes the reader get that asymmetry right by
      hand, and the only API for advancing is `SequencePosition::next()` — the
      exact method D3 says returns the wrong answer where it can fail.
- [ ] **Fix D3**: `SequencePosition::next()` uses `checked_add`, so the method
      whose sole purpose is signalling overflow can do it (VT-13).
- [ ] **Fix D2**: `Query::Items` becomes non-constructible from outside the crate
      (VT-26), so `Query::Items(Box::new([]))` — an `AppendCondition` that can
      never be violated — stops being reachable. Drop the `Default` derive; make
      `Query::from_item` infallible, as its own doc already says it is; add
      `Query::index_arms()`.
- [ ] A condition that can carry **per-item boundaries** (VT-30) — four reads
      produce four boundaries and one condition cannot carry them today, which is
      what blocks E2E-04 and E2E-05.
- [ ] `happenstance_core::prelude` exporting `EventStore` (not `SendEventStore`),
      so the default import path cannot produce E0034. This item's second half —
      `pub use futures_core;` beside `pub use bytes;` — **landed** (`lib.rs:193`),
      and each adapter re-exports its driver with it. The prelude is what is left.
- [ ] `ConditionViolated`'s `Display` interpolates the `conflicting_position` the
      store already populates, and names the remedy.

**Work — semantics, each a documented sentence *and* a rule**

- [ ] **Laziness and isolation** (ES-11 – ES-13, D7). The trait says nothing is
      executed until the stream is first polled (`store.rs:103-108`);
      `MemoryEventStore` snapshots under the lock at call time and documents that
      it does. The difference is observable, and it must be decided against
      **three** shapes: snapshot-under-a-lock, a chunked cursor, and a single
      response body that cannot stream at all and is capped at 64 MB. The third is
      what forces the ADR to describe a class of stores rather than choose between
      the two that happen to exist. Note that DCB does not *need* snapshot reads,
      because the append condition re-checks.
- [ ] **Self-conflict** (ES-21). The condition is evaluated against the store
      *before* any event in the batch is written; a batch can never conflict with
      itself. The reference store answers this by ordering alone. An adapter that
      re-probes per row — what a trigger-maintained index or a conditional `INSERT`
      naturally produces — rejects *every* conditional append, because the standard
      DCB uniqueness shape has the condition query matching the very event being
      written.
- [ ] **Cancellation** (ES-22, ES-23). One paragraph stating whether an adapter
      may commit after its `append` future is dropped. The honest statement is
      probably "may or may not have committed", which is a documentable contract
      and is currently not documented at all. What must be checkable is ES-22: a
      dropped future leaves no *partial* batch.
- [ ] **Position visibility** (VT-12, ES-10). *Once a reader has observed position
      P, no event may subsequently become visible at a position ≤ P.* Add the
      "an adapter that allocates positions before commit does NOT satisfy this"
      warning. Phase 3 supplies the store that can fail it; phase 10 supplies the
      database that does.
- [ ] **`from` is a threshold, not an identity** (ES-9). The read is bounded by
      `position >= from`, and `from` need not name an existing event. Under
      exact-seek semantics the projection resume recipe stalls silently on any
      gapped log while passing all 27 rules.
- [ ] **`conflicting_position`: promise or hint.** Decide explicitly and say which
      in the field's own documentation. Neon-over-HTTP has no interactive
      transaction, so `INSERT … SELECT … WHERE NOT EXISTS` — the shape the
      implementability review rejected for SQLite because it yields a boolean — is
      the only shape it can express. A caller writing a retry loop against the
      field needs to know before it freezes, not after.
- [x] ~~**Two handles share one consistency boundary** (ES-34) and **acknowledged
      writes survive a reopen** (ES-35) become clauses with rules, gated on the
      phase-3 fixture.~~ **Done at phase 3, and this item was stale when phase 4
      opened.** Both are clauses with rules —
      `two_handles_observe_each_others_appends` (ES-34, `must!`-gated on
      `SECOND_HANDLE` per CF-16) and `acknowledged_writes_survive_a_reopen`
      (ES-35, `require!`-gated on `REOPEN` per CF-17). What is left of ES-35 is
      not a rule but an *adapter* at the durability axis's far end, which CF-26
      says no fixture can supply and phase 8 owns. Kept struck rather than
      deleted because the coverage audit reached this item from the other
      direction — ES-33, ES-34 and ES-36 are named by no phase-4 ADR, and the
      reason is that there is nothing left for one to decide.
- [ ] **Decide whether `Fixture::MID_BATCH_FAULT` earns a `CF` clause of its
      own.** Phase 3 added the capability and the rule it gates
      (`append_is_atomic_under_a_mid_batch_fault`) and left the capability
      **unclaused**: `SECOND_HANDLE` is CF-16 and `REOPEN` is CF-17, and the third
      constant has no clause naming it. That was deliberate — adding one moves
      §1.3's census, which is the one count in the specification a human computes
      by reading, and moving it in a stage whose whole job was reconciling it
      would have destroyed the evidence. It is not deliberate a second time.
      Either write the clause and recount §1.3, or record in §6.3 why the
      capability is the one that stays prose.
- [ ] **Name the owner of freezing CF-16 – CF-21, and settle the poll-count
      limitation first.** No phase in this runbook owns that freeze today, and the
      fixture contract is the thing every rule phase 4 writes is expressed
      through. The specific defect to settle: **`Fixture` cannot say "my `append`
      needs *n* polls".** `nothing_below_an_observed_position_appears_later`
      therefore drains whatever the schedule left unfinished, which is correct —
      a legal adapter with real I/O may take any number of polls, and a rule a
      legal adapter fails is a finding about the rule — but it means the rule's
      *strength* varies silently with the adapter: against a store needing three
      polls the interleaving window never closes and the rule cannot fail. A
      freeze that does not state that is a freeze over a rule whose power is
      unknown.

**Work — value types and identity**

- [ ] **`EventId` and `StoreId`** (VT-4 – VT-8). Store-assigned `(StoreId,
      SequencePosition)` on `SequencedEvent`. `StoreId` names a store
      **incarnation**, never a device and never a peer: a tablet restored from
      backup, or an image cloned onto a second device, reissues the same origin
      positions to different events, and every peer's dedup then silently drops
      real facts. That means it is a 128-bit value minted at database creation and
      re-minted on restore, and the peer's human-readable identity lives elsewhere.
      Decide **whether it is queryable** (VT-7): a peer must dedupe without parsing
      `metadata`, or ADR-0003 breaks at the point it claims to win, and a field
      `Query` cannot see does not solve the dedup problem.
- [ ] **`recorded_at`** (VT-9), a plain `u64` of milliseconds since the epoch —
      not `SystemTime`, not a `chrono`/`jiff` dependency: the crate is
      `no_std`-capable and the store is the only thing that ever sets it. A rule
      that it is non-decreasing with position.
- [ ] **The constructor shape decided with them.** `#[non_exhaustive]` does not
      make this free — it makes `SequencedEvent::new` the only way a downstream
      crate can construct one, so the attribute's whole mitigation is "no struct
      literal" and the constructor it forces everyone through is exactly what a
      field addition breaks. Add via a builder or a second constructor, not by
      widening `new`, and make `into_parts` return a struct so its arity stops
      being public API.
- [ ] **Fix D4**: `Event::new` accepts an already-built `EventType` (VT-18). The
      bound `TryInto<EventType, Error = InvalidEventType>` excludes the infallible
      identity conversion, so a typed layer interning one `EventType` per
      `DomainEvent` must round-trip through `&str` and re-validate on every
      construction — and a `const EventType` does not compile at the call site
      (E0271 at `event.rs:198`). The crate already knows the fix and applied it two
      files over: `QueryItem::new` uses `T: TryInto<EventType>, InvalidQuery:
      From<T::Error>` plus `impl From<Infallible> for InvalidQuery`. **Ship this
      with the const-constructor item below**, because either alone is inert.
- [ ] **Const-constructible `EventType` and `Tag`** (VT-14) — `Cow<'static, str>`
      with `pub const fn from_static`, validating with `assert!` in a const
      context. `Cow`'s `PartialEq`/`Ord`/`Hash` all delegate, so `Borrowed("A") ==
      Owned("A")` and both hash identically, and every existing derive stays
      correct. Cost: 16 → 24 bytes on a heap-indirected type. State the trade: this
      buys compile-time validation and a zero-allocation hot path at the price of
      "an `EventType` value is always validated" becoming "always validated, but
      some of that validation happened at compile time". Rejected alternatives, and
      why: a proc macro validates at expansion, still allocates, and adds a
      dependency to the contract crate; `&'static str` only cannot express a
      runtime-derived type, which ingest needs; `new_unchecked` is a footgun with
      no performance story under `forbid(unsafe_code)`.
- [ ] **One umbrella validation error** (E2E-51). `InvalidTag`,
      `InvalidEventType`, `InvalidQuery` and `AppendError<E>` do not unify, so the
      only worked example needs `anyhow` — permitted for examples, but it
      demonstrates a style a library author cannot copy, and the library ships
      nothing to copy instead.
- [ ] **Fix D9, and write the MUST VT-14 already states** (VT-14, VT-15, VT-20).
      `char::is_control` is Unicode `Cc`, not ASCII, and
      four sites say "ASCII" (`tag.rs:47`, `event.rs:37`, `error.rs:29`,
      `error.rs:53`) — a two-word fix. **VT-14 also requires `EventType::new` and
      `Tag::new` to reject the seven explicit bidirectional controls
      (U+202A–U+202E, U+2066–U+2069), and phase 3 wrote none of it.** The clause
      now says so in terms; before stage 6's review its `Rule:` line read
      "extended", which a reader takes as done, and `spec-trace` cannot notice
      because the rules it cites are *unit tests*. The two land together on
      purpose — the prose fix argues from the current wrongness, which is what
      `Latin1IdentifierStore`'s provenance cites — and the check itself is a
      `matches!` over a closed list with no dependency, which is why it is
      affordable when a general-category predicate is not. Then take a position
      on `Cf` format
      characters (U+200B and U+202E produce two visually identical tags that are two
      different consistency boundaries) and on normalisation: `"café"` in NFC and
      NFD are different `Tag`s, so a macOS client and a Linux client writing the
      "same" tag silently fail to conflict. Note that blanket `Cf` rejection bans
      U+200C/U+200D, which Persian, Hindi and emoji sequences require, and that
      normalising at construction breaks ADR-0003's byte-for-byte forwarding
      promise and adds a dependency the `no_std` build cannot take. **Doing nothing
      and saying nothing is the only option that is definitely wrong.**
- [ ] **Store limits** (VT-21 – VT-25): guaranteed minima a store MUST accept for
      payload size, tag count, query items and batch size, plus the error variant
      for what it refuses. `SQLITE_MAX_VARIABLE_NUMBER = 32766` applies to a
      multi-row INSERT exactly as it applies to tags, so an over-large batch
      otherwise fails at write time after the caller has already decided.
- [ ] **`is_violated_by`'s comment argues from a floor that has moved.**
      `append.rs:100-101` still says it is "written without a let-chain so the
      crate keeps its 1.85 MSRV; let-chains only stabilised in 1.88". ADR-0029
      raised the MSRV to 1.97.1 at phase 2 and `memory.rs` uses a let-chain. Two
      words, but the specification points readers at that function as the
      canonical statement of the condition predicate, so the stale reason reads
      as a live constraint. It sits here rather than in phase 3 because phase 4
      is where `AppendCondition`'s surface is reopened anyway.
- [ ] **Decide what checks citations outside `SPECIFICATION.md`.**
      `spec_trace::check_citations` reads that one document, so the ~150
      `file:line` citations in `references/scenarios/README.md` and
      `E2E-CASES.md` are checked by nothing — which is how nine of them came to
      point at unrelated text after phase 3 grew `suite.rs` by 2,469 lines. Note
      before extending it that the check would have caught **none** of the nine:
      it verifies that the file exists and the first number is in range, and all
      nine passed both. What it *would* catch today is the six citations spelled
      `sync/lib.rs:NN`, a shorthand for `crates/happenstance-sync/src/lib.rs`
      that resolves to no path in the tree. So the decision is two questions, not
      one: whether to teach the checker that shorthand or to spell the six out,
      and whether a check that cannot see content is worth extending at all or
      whether the honest answer is the pinned-commit form §6 already uses.
- [ ] **`ProjectionId`** — validate it the way `EventType` is validated, or state
      in the docstring that it is a deliberately opaque operator-chosen key. It
      becomes the primary key of a checkpoint row, and the unexplained
      inconsistency with its two validating siblings teaches the reader that
      validation here is optional.
- [ ] `Borrow<str>` and `FromStr` for `Tag`/`EventType` — a `HashMap<EventType,
      Handler>` is the core data structure of a codec registry and without `Borrow`
      every lookup allocates; `Hash` on the wire types, which phase 13's dedup
      needs; owned `IntoIterator for Tags`; `Extend<Tag>` re-canonicalising per
      call; `Tags::value_of(key)` as a `partition_point` on the `"key:"` prefix —
      O(log n) rather than the O(n) scan the worked example writes by hand twice,
      and the crate already pays for the sort. Decide whether repeated keys are
      legal (VT-17): they are today, and an `Option` return would silently pick one.

**Proof artefact.** `crates/happenstance-core/tests/frozen_signatures.rs`: a
**generic** consumer, bound on `EventStore` and never on a concrete store, doing
the four things today's signatures forbid and the frozen ones must permit —

1. return a `read` stream from a function, which E0716 refuses today because the
   opaque type captures the borrowed `Query`'s lifetime;
2. store one in a struct field, same reason;
3. `tokio::spawn` a replay under `SendEventStore`, which is where the `Self: Sync`
   rule from ADR-0008 is either right or is discovered to be wrong;
4. `append` a `Vec<Event>` and afterwards use `Event::into_parts` on an event it
   still owns — the method whose doc comment says it exists to avoid a clone and
   which is unreachable through the port today (`memory.rs:233-235` performs
   exactly the clone it was written to prevent).

Each is a compile, and each fails against the current signatures. **That is the
whole point, and it is why the previous revision's artefact was decorative**: it
named "the six phase-2 skeletons compiling unmodified", but a skeleton whose
bodies are all `todo!()` compiles against *any* signature — `todo!()` has type `!`
and `!` coerces to everything — so six green skeletons are equally consistent with
a right freeze and a wrong one. Its second half, `PreCommitPositionStore` failing
the visibility rule, is phase 3's exit criterion re-used, which leaves phase 4
proving nothing of its own.

The skeletons still matter, but as a *regression* check with a stated tolerance:
after the freeze, none of the six may change its `Error`, its `Batch`, or its
stream type from what `references/adapter-shapes.md` recorded. Only the port's own
signatures may differ, and phase 6 owes the same distinction for the same reason.

**Exit criteria**

- [x] ADR-0011, 0012, 0013 written and merged before the code they constrain.
      **All five** — 0011 through 0015 — landed at `00795d0`, adversarially
      reviewed, and were signed off at `2506912` before slice A cut any code.
- [x] ~~Every semantic sentence has a rule that fails without it.~~ **Rewritten,
      and met in the rewritten form.** As stated it is unsatisfiable, because
      three `[FROZEN]` clauses have `Rule:` lines that deliberately read *none* —
      ES-23 (an adapter MAY commit a dropped append; the caller-facing half is
      not observable), ES-32 and ES-37 (the absence of a method is not
      checkable). The honest form: **every semantic sentence either has a rule
      that fails without it, or says in its own clause why it cannot have one.**
      89 rules, and `spec-trace` check 6 holds the two sets equal in both
      directions.
- [x] ~~`frozen_signatures.rs` exists, is in the gate, and each of its four cases
      is demonstrated to fail against the pre-freeze signatures.~~ **The file
      exists and is in the gate; the second half is rewritten, because two of the
      four cases do not fail against the pre-freeze signatures and never did.**
      Case 3 was already green before this phase, as `memory.rs`'s
      `spawns_from_generic`; `frozen_signatures.rs` re-makes it for a *generic*
      consumer, which is the half that was missing. Case 4 — append a
      `Vec<Event>`, then `into_parts` an event you still own — **compiles and
      runs against the pre-freeze signature**, because `append` borrows and the
      caller never gives the `Vec` away; it was verified by running it. What
      `&[Event]` costs is inside the adapter, which is ES-17's measurement and
      phase 8's. Cases 1 and 2 are real and are discharged by owning the query in
      a **parameter** rather than by taking `Query` by value, which ES-13
      `[FROZEN]` forbids — the file's `compile_fail` doctest pins the four
      diagnostics the local-variable arrangement produces.
- [x] ~~The six skeletons compile, **with no change to any associated type** —
      byte-identical to `references/adapter-shapes.md`.~~ **Rewritten: the criterion
      had no referent and the stronger half is now false by design.**
      `adapter-shapes.md:41-47` is a prose table of type *names*, so
      "byte-identical" was never checkable against it. And ES-30 and ES-41 make
      `head` and `contains_event_id` required, so all six skeletons necessarily
      changed. The surviving claim, which is the one that was worth having:
      **no skeleton changed its `Error`, its `Batch` or its stream type**, and
      each new body is `todo!()` rather than an invented implementation.
- [x] `conflicting_position` is documented as a promise or as a **hint**, and
      `happenstance-neon` is cited for why the question was asked: it has no
      interactive transaction, so the only shape it can express yields a boolean
      and no row. Its `Display` now renders the position when there is one.
- [x] **The CF-25 exposure is discharged three ways, not asserted once**, with
      one correction: the axis count is **four accepted and three carried**, not
      six. "Six" appears in this file and in `SPECIFICATION.md:260`, and because
      the two documents *agree*, rule 5 gave no arbitration — both were wrong in
      the same way. Six is seven axes minus the one *measured*; the exit
      criterion's own wording is exclusive ("they are not part of this freeze and
      need no acceptance"), under which an axis is either carried by a live
      `[PROVISIONAL]` clause naming it or accepted by name, never both and never
      neither. ADR-0013 states the partition. **ES-10 lifted to `[FROZEN]`** on
      arm C's measurement, so ES-25 and ES-26 do not reopen — with the caveat
      ADR-0013 §4 records, that `head()` on such an adapter is the visibility
      frontier and not `max(position)`.

- [x] ~~`EventType::from_static("")` is a compile error, demonstrated by a
      `trybuild` case.~~ **It is a compile error, demonstrated by a
      `compile_fail` doctest, and the substitution is a finding rather than a
      shortcut.** Compiled: rustdoc on 1.97.1 *silently ignores* an error-code
      annotation it cannot match, so `compile_fail,E0080` is not a stronger check
      than bare `compile_fail` — it is the same check wearing a claim. A
      `trybuild` snapshot would pin the diagnostic, and pulling that dependency
      in is a decision **phase 6** owns; ADR-0015 records it rather than
      pre-empting it here. Note also that the guarantee is weaker than the
      criterion implies and `from_static`'s own docs now say so: an *associated*
      const that nothing reads carries an invalid value through `check`,
      `clippy`, `build` and `test`.
- [x] A stated position on Unicode normalisation exists in `Tag`'s docs, with
      both worked failures — NFC/NFD `café` as two consistency boundaries, and a
      trailing space that sorts adjacent and never matches.
- [x] ~~`SequencedEvent` carries `id` and `recorded_at`, and adding a *third*
      field would not break `new`.~~ **First half met; second half rewritten,
      because it is false for the fields this phase added and true only for a
      different kind of field.** A *required* store-assigned fact has no
      non-breaking constructor shape at all — adding one to a positional
      constructor is `error[E0061]`, and `#[non_exhaustive]` is exactly what
      makes the constructor the only surface. So `new` went from two arguments to
      four in one commit with no deprecated arm, because a shim would have to
      invent a `StoreId` and a time, which is the wrong implementation VT-4
      rejects by name. The honest form: **`new` is superseded rather than
      widened, and the `with_*` shape is in place so a *defaultable* field lands
      without touching it again** — `with_recorded_at` is the worked example.
- [x] **`EventStore::append`'s signature is unchanged by identity**, and VT-10 is
      cited as the reason — a foreign identity arrives through `IngestStore` in the
      sync crate. If it did change, the signature half of this phase is reopened by
      its value half, and that is worth saying out loud rather than discovering in
      a diff.
- [x] **Phase 3's value-edge rules are reconciled against the minima frozen here.**
      `suite.rs`'s four private constants became `use happenstance_core::{…}`, so
      the clause and the code now agree by construction and raising a floor takes
      its rules with it. The stale comment claiming the runbook still said "1 MiB"
      went with them — it had been correct about which document wins and wrong
      about the state of the other one, which is the failure mode it was written
      to prevent.
- [x] **VT-17's missing E2E case is written.** `SPECIFICATION.md` §7.5 records it
      as naming no case at all and calls it a genuine hole: construct a `Tag` with
      no colon and one with two, assert both are accepted and neither acquires
      structure. `key:value` is convention, the clause says so, and a store that
      started enforcing it would be wrong with nothing to catch it.

**2026-08-08 — the code. Phase 4 is done.** Seven slices, each ending with a
green `cargo xtask ci` and its own commit, so a regression is bisectable to the
change that caused it rather than to a three-thousand-line freeze.

`73bd647` value types · `634f635` read options and `Query` · `b327e48` identity
and recorded time · `1b2a565` append-condition guards · `d2b55d7` the two
required port methods · `d480446` the rules, mutants and clauses · this commit,
the proof artefact and these criteria.

**The suite went from 55 rules to 89 and the mutant registry from 52 rows to
80.** The specification went from 193 clauses to 200: ES-41 (membership), CF-39
(a fixture's injected fault must not be absorbable), CF-40 (a fixture declares
its numeric limits), VT-32 (const-constructible identifiers) and VT-33 (the
standard-library trait surface), with ES-10 lifted to `[FROZEN]`.

**Five of the twelve exit criteria were rewritten rather than ticked**, and each
says why above. The pattern in four of the five is the same and is worth naming
once: the criterion asserted that something would *fail* before the freeze, and
it did not. Case 3 of the proof artefact was already green as phase 1's
`spawns_from_generic`; case 4 compiles and runs against the pre-freeze signature,
because `append` borrows and the caller never surrenders its `Vec`. A criterion
that cannot fail is the same defect as a rule that cannot fail, one level up, and
this plan's own adversarial pass found four of those in the previous revision
without noticing that the replacement had the same shape.

**What phase 4 did not settle, recorded so the next phase does not inherit it as
a surprise.** ES-11 and ES-12 stay `[PROVISIONAL]` on the transport axis: a rule
that distinguishes a snapshot read from a self-paginating one needs a fixture
that can pause between round trips, and no such fixture exists — writing one that
passes both would have been worse than writing none.
`recorded_time_survives_a_reopen` has no mutant reaching its headline assertion,
only its setup anchor, so the sentence it exists for has no negative control
until a durable adapter exists at phase 8. VT-22 and half of VT-23 carry
falsifiers no scheduled phase can reach — they are adoption-gated, which is
honest labelling rather than a resolution, and phase 12 decides whether 0.1 ships
with them.

**One thing to carry forward.** The `[FROZEN]` markers on `EventStore` now rest
on 89 rules and 80 mutants, and on **no adapter at all**. CF-26 draws the line
this phase spent seven slices respecting: a fixture instrument proves a rule can
fail; only an adapter instrument proves a real implementation can pass. Phase 8
is the first time anything in this repository will have done the second.

**Cases this makes writable.** E2E-02, E2E-03, E2E-04, E2E-05, E2E-07, E2E-11,
E2E-12, E2E-13, E2E-14, E2E-43, E2E-51, E2E-54.

**Estimate.** 10 days — 7 for the signatures, 3 for the values that were phase 5.

**Session log**

**2026-08-08 — the ADR pass. Decisions written, no code.** State stays
`not started`, because a phase is done when its proof artefact exists and phase
4's does not. What exists is
[`references/evaluation/phase-4-reconciliation.md`](references/evaluation/phase-4-reconciliation.md)
and ADR-0011 – ADR-0015, all five adversarially reviewed and repaired, awaiting
sign-off. Baseline and close both green (`cargo xtask ci`, exit 0).

The pass was organised the way step 4 of the session protocol asks — ADRs before
the code they constrain — and the first thing it found is that **this phase body
disagrees with the specification in two places**, so rule 5 applies:

- *"`read` takes its query by value"* (`:2768-2774`) contradicts **ES-13**
  `[FROZEN]`, whose `Rejects:` line names that exact fix verbatim. ADR-0011
  keeps `&Query`. It does not pay ES-13's stated price either: the escaping
  cases are discharged by owning the query in a **parameter** rather than a
  local, which keeps all four proof cases instead of costing two. Precise
  capturing (`+ use<'a, Self>`) was compiled to work on a plain trait and
  compiled to be unavailable under `#[trait_variant::make]` (E0799), so the
  option exists but costs ADR-0001's and ADR-0008's derivation mechanism.
- *"`append` takes its events by value"* (`:2759-2767`) contradicts **ES-17**,
  whose falsifier is a measurement phase 8 owns. ADR-0012 keeps `&[Event]` and
  leaves the marker provisional. Both alternatives this body offers are
  independently dead: VT-24 forbids an `EventBatch` type in terms, and
  `impl IntoIterator<Item = Event>` was compiled to be unerasable by `dynosaur`
  (E0191) — which is `RUNBOOK.md:563`'s own falsifier for ES-17 *firing*.

**The proof artefact is partly invalid as written, and two of its four cases were
executed rather than argued about.** Case 3 is already green as
`spawns_from_generic`. Case 4 — append a `Vec<Event>`, then `into_parts` an event
you still own — **compiles and runs today** against unmodified
`happenstance-core` (`test tests::runs ... ok`), because the caller never gave
the Vec away; the real cost `&[Event]` imposes is inside the adapter, which is
ES-17's measurement and not this phase's. Exit criteria 2, 3, 4, 6 and 7 collide
with something and are enumerated in the dossier.

**Two findings nobody was looking for.** The ADR queue (`:284-288`) scopes the
five ADRs to 35 clause IDs while `:2755` says this phase discharges 64 — **29
clauses, including everything governing `head()`, all of `ReadOptions` and
`Query`, and all of the condition semantics, are claimed by no ADR in the
queue.** And `:2866`'s *"No phase owns freezing CF-16 – CF-21"* is stale: five of
the six are `[FROZEN]` and CF-17 is phase 8's. Its poll-count half is live,
unclaused, and verbatim true.

**What the body still owes.** Every edit rule 5 requires here is enumerated,
current-text-to-required-text, in the five ADRs' closing sections. They are
applied with the code rather than now, because the decisions that motivate them
are not signed off yet, and a body edited ahead of its decision is a body that
records an intention.

**2026-08-08, later — the coverage gap is closed.** The 29 unclaimed clause IDs
were audited one at a time against the five written ADRs rather than against the
queue's stated scopes. Most were already covered and the queue rows were stale;
they now say what each ADR discharges. Four IDs were named by no ADR: ES-33,
ES-34 and ES-36 need nothing, because phase 3 already gave all three passing
rules, and the ES-34 work item above is struck as stale. **VT-2 was the real
hole** — `[FROZEN]`, its rule `appending_equal_events_yields_two_events` absent
from `suite.rs`, and unwritable before this phase because the clause's third
conjunct is about `EventId`. ADR-0014 §9 adopts it, specifies the rule and names
`ContentHashIdentityStore` as the mutant that fails it and nothing else. The
standing lesson is recorded at the ADR queue: **a phase's clause range and the
union of its ADRs' clause ranges are two numbers, and nothing checks that they
agree.**

**2026-08-08, sign-off. All five ADRs are accepted.** The five questions the pass
refused to answer on its own were taken by a human, and each went the cheaper way
on the same principle: *where the evidence is not in yet, prefer the marker that
lifts over the marker that must be superseded.*

1. **ES-41 (`contains_event_id`) ships `[PROVISIONAL]`,** not frozen. The marker
   does not gate the method — a provisional clause is normative, so the required
   method ships in 0.1 either way — and VT-7's dangling forward reference is
   discharged by the clause existing rather than by its maturity. Lifts at
   phase 8 or 9, whichever adapter implements it first. **This dissolves the
   ordering constraint ADR-0014 had on ADR-0013**, which existed only to inherit
   a CF-25 risk acceptance a frozen clause would have needed.
2. **The fixture's numeric-limit declaration lands in ADR-0015, as CF-40.**
   Ownership follows the obligation: the rule it unblocks
   (`append_reports_exceeded_store_limits`) checks VT-25 and VT-19, both
   ADR-0015's, and both mutants owed with it are value-type mutants. Without it a
   `[FROZEN]` clause would have shipped implemented and checked by nothing.
3. **CF-25's partition is the exclusive one: four axes accepted by name, three
   carried by live `[PROVISIONAL]` clauses.** It is the only reading that is a
   partition, and it is auditable at phase 12. The number is bookkeeping; the
   load-bearing part is ADR-0013's caveat that batch shape is a `ProjectionStore`
   axis whose acceptance here is pro forma and **non-transferable** — phase 6 may
   not treat that row as discharged. `:3032` and `SPECIFICATION.md:260` both say
   "six" and both need correcting.
4. **Both of ADR-0012's proposed lines are accepted**, which moves it from
   `proposed` to `accepted`. `AppendCondition::guards` becomes private with an
   accessor, because `#[non_exhaustive]` seals the struct *expression* and not the
   struct, so `c.guards = Box::new([])` compiles downstream today and yields a
   conditional append that is silently unconditional. And CF-39 requires an armed
   fixture to produce `Err`, narrowing `[FROZEN]` ES-18 — bounded by CF-39 itself
   being `[PROVISIONAL]`, so a driver that absorbs every injectable fault reverses
   it by a marker edit rather than by a superseding ADR.
5. **`read`'s return type takes no `Unpin` bound,** and the question is scheduled
   rather than closed: a new `[PROVISIONAL]` clause whose owner is a **deadline,
   not a phase** — re-evaluate before phase 12, because after first publish the
   bound cannot be added at all. ADR-0001:110-112 and `E2E-CASES.md:1432` both
   assert a `dynosaur` erasure that does not work as written and are amended with
   it.

**What this does not settle.** The three questions the reviews raised and nobody
has taken: VT-31's owner, and with it `query_union_is_item_concatenation` — the
only rule that would catch an adapter reordering query *items*, which ES-15's own
`Rejects:` names; `read_from_a_gap_position`, declined in writing by both
ADR-0011 and ADR-0013; and the five VT-4 – VT-9 rules with no named wrong
implementation, which the code run must either supply mutants for or argue cannot
have one. All three are rule-ownership questions rather than contract questions,
so they are the code run's to close — but they are closed by decision, not by
writing code, and a run that discovers them at the end will be tempted to skip
them.

---

## Phase 5 — Freeze the wire format

**Goal.** The `serde` representation of every envelope type settled, versioned and
proved to round-trip in a self-describing *and* a non-self-describing format.

**Why here — and why "here" is a range.** This phase depends on phase 4, because
you cannot serialise an `EventId` before deciding one exists. Nothing else depends
on it. `serde` is off by default (`Cargo.toml:26-33`), no adapter touches it, and
its only consumer is replication at phase 13 — phases 8 through 11 do not mention
the wire format once. So it must land **after phase 4 and before phase 12**, and
anywhere in between is equally correct. It is the one phase on this plan that can
be scheduled around a bad week.

It cannot slip past phase 12, because publishing makes the representation
semver-visible, and a private format that has shipped wrong is still a format two
peers disagree about.

**Do not wait for this phase to fix D1.** D1 was a defect, not a decision: five
`skip_serializing_if` attributes made sparse shapes fail to decode in postcard and
bincode. Any phase could have deleted them the moment anyone was annoyed by them;
none did, and this phase deleted them at `20ab863`. What this phase owns is the
*freeze* — the representation, the versioning and the proof — not the bug.

**Decisions it settles.** ADR-0016 (the wire). Discharges WF-2 – WF-12, and WF-1's
scope half only: the format is private, and the interoperability half stays
`[DEFERRED]` to phase 13 on a reason this phase made stronger rather than weaker.

**Work**

- [x] **Fix D1 (critical)** (WF-2): delete every `skip_serializing_if`. Five sites
      across `event.rs`, `query.rs` and `append.rs`, deleted at commit `20ab863`.
      This body used to cite them as `event.rs:342`, `:344`, `query.rs:281`, `:283`
      and `append.rs:121`; they had drifted some two hundred lines before anyone
      reached them and the attributes are gone now, so a closed defect names its
      **commit** rather than a line — a line number for deleted code is a citation
      that can only ever be wrong. They shortened the field count passed to
      `serialize_struct`, and a format with no field names feeds the deserializer
      exactly `FIELDS.len()` values positionally, so `Event`, `SequencedEvent`,
      `QueryItem`, `Query` and `AppendCondition` produced bytes that cannot be
      deserialised in *any* non-self-describing format, for their most common
      shapes. Serialisation succeeded silently and produced plausible bytes.
      Postcard and bincode are exactly what a Worker-side wire would choose.
- [x] **`#[serde(default)]` goes with them**, at the same five sites. This body used
      to say *"Keep `#[serde(default)]` so existing JSON still parses"*, and that is
      **reversed** — the one place the phase body and the plan disagreed. It is
      reversed on ADR-0016 §4's own reasoning and **not under rule 5**: nothing is
      published and `happenstance-sync` is `publish = false`, so the compatibility
      the attribute protects does not exist. Measured, it is inert on the write side
      (postcard output is byte-identical with it, without it, and with a
      whole-struct `default`), so it was never the positional hazard WF-12's "fifth
      hazard" sentence claimed. Its only effect is on decode, and a `default` on a
      field the encoder **always** writes means the decoder accepts a document the
      encoder can never produce — a second, wider, undocumented format only the
      decoder knows about, in which a peer's bug survives a round trip and
      reappears as a default value three hops later. **What the deletion does not
      buy**: it closes the read side at **three of the five sites only**.
      `Event::metadata` and `Guard::after` are `Option`-typed, and `missing_field`
      succeeds for any type whose `Deserialize` calls `deserialize_option`, so an
      `Option` field accepts absence with **no attribute at all**. The presence
      obligation therefore binds the **encoder only**, which ADR-0016 §5 records as
      a decision rather than leaving to inference — it fails closed, because a
      missing `after` decodes to `None` and `None` checks the whole log.
- [x] The cost in JSON is **+26 bytes per value**, not a fixed pair. This body used
      to quote *"a bare `Event` going from 35 to 61 bytes"*; that figure holds only
      for a single-digit payload. Worked instances: **35 → 61**, **39 → 65** and
      **28 → 54**. In postcard the saving the attributes bought was one byte per
      absent field — the tagged `Event` 18 → 19, the all-defaults `Event` 7 → 9 —
      and what it bought back was a decoder that reads a neighbour's bytes. The
      trade was always correct and both of its numbers were wrong.
- [x] **Fix D6, with D1, in the same change** (WF-3, WF-4). Both landed at
      `20ab863`. `Query::All` serialised to JSON `null`, so the most destructive
      value in the protocol — the one an `AppendCondition` uses to reject any append
      at all — was what a buggy peer produced by accident. It is now an externally
      tagged `"All"`. Two things the defect list did not note: `query.rs` documented
      the old behaviour as *intent*, so the ADR reverses a choice rather than fixing
      an oversight; and the field this item called
      `AppendCondition::Wire.fail_if_events_match` is **`GuardWire::query`** — VT-30
      renamed it at `1b2a565` and WF-4 froze the dead spelling (ADR-0016 §2). The
      hazard is real and the input was wrong: bare `"{}"` **already** errored at
      HEAD, and the live input is `{"guards":[{}]}`, which decoded to
      match-everything through the same `missing_field` mechanism above.
      `wire::empty_object_is_not_a_condition` tests both, because a rule that only
      tries `"{}"` rejects no implementation anyone would write.
- [x] **The format is private** (WF-1, scope half). The `serde` feature moves events
      between happenstance instances; it is not an interoperability surface. That is
      what makes every fix above free rather than breaking. The interoperability
      half stays `[DEFERRED]` and its reason changed: the DCB reference publishes no
      wire format to bridge against at all. Recorded on the
      [DCB-interoperability ledger row](#event-store-and-value-types), not as
      silence.
- [x] **WF-5 was already implemented and this body did not know it.** `Guard::after`
      survives the round trip and reaches an ingest policy; what it cost was one
      round-trip test, `wire::condition_after_is_visible_to_an_ingest_policy`
      (ADR-0016 §8).
- [x] **WF-6: split the two formats** (`9706f9f`). A `StoreId` crosses a
      human-readable format as the thirty-two lowercase hex digits its `Display`
      already produced, and a binary one as sixteen raw bytes with no length prefix.
      The clause froze the field as `origin`; ADR-0014 renamed it `store`, and both
      numbers in the clause's `Rejects:` line were wrong (the JSON cost is 1.7353×,
      not "four times"). The wrong implementation each module names is an
      **inverted** `is_human_readable` branch, which is internally consistent in
      each arm and so round-trips cleanly in **both** formats — which is why the
      rules assert on the encoding and not on the round trip.
- [x] **WF-7: the two-format matrix is the instrument** (ADR-0016 §6), and its
      *"maximum-size values"* half was owned by nobody. Both round-trip proptests
      run over every envelope shape, and the strategies are local rather than
      imported from `happenstance-testkit` — core cannot dev-depend on the testkit
      without a cycle, and the wire generator must also produce the two shapes
      `any_event` deliberately excludes: the empty payload, and
      `metadata: Some(<empty>)` as distinct from `None`.
- [x] **Version the envelope** (WF-8, `99ba69d`): a format version as the first
      field, so a bound change is a private-format change rather than an
      undiagnosable parse failure. `happenstance-sync` gains `FORMAT_VERSION`,
      `Envelope<T>` and its own `WireError`. The `Deserialize` is **hand-written**,
      because a derive reads every field into a local `Option` and only then
      constructs the struct — so a derived envelope decodes the message before
      anything examines the version, which is the partial decode WF-8's MUST NOT
      forbids.
- [x] **WF-9's decode half** (ADR-0016 §12). The variant meaning "refused, park
      this" and
      its suite rule both predate this phase; what phase 5 owed was
      `wire::decode_accepts_an_over_capacity_value`, and the clause has to name the
      ceiling the value exceeds or the rule is decorative — `happenstance-core`
      enforces no store ceiling, and `MIN_SUPPORTED_EVENT_DATA_LEN` and its
      neighbours are floors an adapter must *support*.
- [x] **WF-10's four rules.** A decode re-runs the constructor's invariants:
      `wire::decode_rejects_a_non_canonical_tag_set`,
      `wire::decode_rejects_an_unconstrained_query_item`,
      `wire::decode_rejects_a_zero_item_query`, and the over-capacity rule above.
      `Tags` re-canonicalises rather than erroring, so the first asserts a
      **canonical `Ok`**; its teeth are that `Tags::contains` is a binary search, so
      an unsorted `Tags` silently answers `false` for a tag that is present.
- [x] **WF-12: `ReadOptions` leaves the wire** (`99ba69d`). `from` is a store-local
      position with no meaning at a peer. The clause's stated reason and its stated
      instrument are both corrected — see the exit criterion below.
- [x] Decide `Bytes`' human-readable representation (WF-11): branch on
      `is_human_readable()` and emit standard-alphabet base64. The clause's *"1 KiB
      payload is ~4 KiB of unreadable JSON"* is measured at **3.5715×** on a 340 KiB
      seat map — so ~3.6 KiB — against base64's 1.3333×. `Some(Bytes::new())` stays
      distinct from `None` (`""` against `null`), because ADR-0003 promises
      byte-for-byte forwarding and an empty metadata blob is not an absent one.

**Proof artefact.** `crates/happenstance-core/tests/wire.rs` **and
`crates/happenstance-sync/tests/wire.rs`** — 21 tests and four const assertions,
both files wrapping everything in an inner `mod wire` so `cargo test --list`
prints the qualified name the clauses cite. A `round_trip<T>` proptest asserts
equality through **both** `serde_json` and `postcard`, exercising the sparse
shapes specifically — a bare `Event` with no tags, a types-only `QueryItem`, a
tags-only `QueryItem`, `Query::All`, an `AppendCondition` with and without
`after` — plus direct tests that deliberately unsorted and duplicated tags come
back canonical and that an over-length `Tag` is rejected on the way in. Two
formats, because a self-describing and a non-self-describing format break
differently, and the whole class of defect above is invisible to one of them. A
proptest that passes in `serde_json` alone is the test that would have shipped D1.

Two things the plan did not anticipate. **Every postcard round trip is framed
against a known trailer** — a standalone round trip reports a field-count
desynchronisation as *"unexpected end of buffer"*, which is the weaker regression
test; a value with something after it reports the same defect as a **wrong value
with no error at all**, which is what D1 actually was in a real message. And
**three negative controls** put a paired half through identical framing, so the
failure is attributable to the defect rather than to the harness or the byte
count; `xtask/src/proof.rs` generalises from a three-const single-artefact
checker to a table and asserts them **by name**, because nothing about a negative
control looks load-bearing and the first person to find them confusing will
delete them as dead code.

**Exit criteria**

- [x] ADR-0016 written first. Commit `7331453`, before any code.
- [x] The wire proptest passes in `serde_json` **and** postcard. The committed
      proptest seed is the corner that matters: the shrunk counterexample is the
      all-defaults `Event`, which is D1's shape exactly.
- [x] **D1 and D6** are closed, and each is demonstrated to have failed before the
      change — a decode that returns the wrong value is a better regression test
      than a decode that errors, and the three negative controls make that
      demonstration a **standing property** rather than a transcript in a commit
      message. **D12 is not on this list and could not be.** It was closed at
      phase 0 by commit `927d291`, and its failure is not reproducible today:
      `bytes` 1.12.1 declares its own optional `serde` with `features = ["alloc"]`,
      so deleting the token would still compile *by the manifest*. A behavioural
      rule would observe nothing in every configuration on this lockfile and start
      observing something only on the day `bytes` changes its mind — the day the
      check was meant to have prevented. The failure mode is a manifest edit, so
      the check is a **manifest lint** in `xtask`, in the shape
      `lints::testkit_version` already has, and deliberately **no WF-13**: a clause
      in a frozen section to describe a `grep`. §1.3's census does not move
      (ADR-0016 §14).
- [x] **The envelope carries a format version** (WF-8). The rule is a **witness**
      that the message's `Deserialize` never ran, not the byte-layout assertion the
      clause reached for — measured, a derive plus a version check afterwards
      returns the same `Err` in both formats and produces byte-identical postcard
      framing, so neither the error nor the front of the buffer distinguishes the
      two designs. The one observation that does is whether the message decoded at
      all.
- [x] **An append that exceeds a store limit is refused with a variant meaning
      "refused, park this"** (WF-9), and **the demonstration is core-only and needs
      no peer.** This and the criterion above were one sentence, and the word
      *"peer"* is what made this half look like phase 13's work. WF-9 is about a
      **store limit**, not a format version — a bound change is explicitly *not* a
      wire change, which is WF-9's whole MUST, so a version bump is the wrong
      instrument for it. `AppendError::ExceedsStoreLimit { limit, len }`
      (`error.rs:237-244`) and `append_reports_exceeded_store_limits`
      (`suite.rs:4282`, registered at `registry.rs:187`) both predate this phase.
      What phase 5 owed was the decode half, and it is written (ADR-0016 §12).
- [x] **WF-12's missing case is written.** `SPECIFICATION.md` §7.5 recorded it as a
      genuine hole: assert `ReadOptions` does not implement `Serialize`. Its
      instrument is a **const-evaluation assertion** in
      `crates/happenstance-core/tests/wire.rs`, not a compile test — three of four
      `compile_fail` spellings were measured green against a false assertion
      (ADR-0016 §13). Proved three ways before it landed: pointed at a serialisable
      type it fails with `E0080`; misspelt it fails with `E0425`, which is a build
      failure rather than a green test; and the `Deserialize` half needs
      `DeserializeOwned`, since `impl<T: serde::Deserialize>` is `E0106`. Two
      **inverse** assertions on `Event` stay in the file permanently, because every
      negated assertion is otherwise satisfied by a detector whose inherent impl
      never applies, and that failure is silent.
- [x] `cargo xtask ci` green, including `cargo hack --feature-powerset`, which is
      the only thing that compiles this feature in isolation. Green on the whole
      branch, with `cargo-hack` and `cargo-deny` both resolving on this machine so
      those steps ran rather than printing `skipped`.

**Cases this makes writable.** E2E-33, E2E-34, E2E-37 (its wire half), E2E-40.

**Estimate.** 3 days.

**Session log**

**2026-08-09 — the phase, in seven commits.** ADR-0016 was written and accepted
before any code, so for once the code ran downhill from a decision instead of
alongside one. Every number quoted below is reproducible from
[`experiments/wire-format/`](experiments/wire-format) — seven programs and
four doctests, outside the workspace and outside the gate, which is what
`experiments/` is for. It is a **record of a measurement taken on a date**,
not a test of the current tree, and two of its doctests document pre-removal HEAD
by construction.

1. **`7331453` — ADR-0016 and the experiment.** The finding that set the phase's
   tone: **the specification got here first, and three `[FROZEN]` clauses
   described a type that no longer existed.** §2.7 already carried WF-1 – WF-12,
   nine of them frozen, each naming the test in
   `crates/happenstance-core/tests/wire.rs` that discharges it — and that file did
   not exist, nor did a single serde round-trip assertion anywhere in the
   workspace. 354 lines of hand-written wire format across five modules, checked
   by nothing. WF-4 froze a wire field named `fail_if_events_match`, which VT-30
   deleted at `1b2a565`; WF-6 froze `origin`, which ADR-0014 renamed `store` —
   two clauses in one document disagreeing, and one of them having to lose. So
   the ADR discharges several clauses by **amending** them rather than by
   satisfying their letter, and says so in its own status line.
2. **`a7d6779` — the two dependencies**, `base64` for WF-11 and `postcard` for
   WF-7's second format. The commit that found RUSTSEC-2023-0089; see below.
3. **`20ab863` — D1 and D6, one commit**, because neither fix is worth much
   alone. Five `skip_serializing_if` attributes and five `#[serde(default)]`s
   deleted; `Query` becomes externally tagged.
4. **`9706f9f` — WF-6's and WF-11's human-readable halves**, both branching on
   `is_human_readable`. The hex decoder refuses uppercase **deliberately rather
   than by omission**: `from_str_radix` accepts `A`–`F` silently, so a peer
   emitting two spellings of one store would round-trip perfectly and reach the
   log as two identities. No round-trip test can catch that, which is why the
   comment on `nibble` says so.
5. **`99ba69d` — WF-8's envelope, and `ReadOptions` off the wire.** They land
   together because both are about what the format refuses to promise rather than
   about what it encodes.
6. **`dfc7bcd` — the proof artefact.** Twenty-one tests, four const assertions,
   three negative controls, and `xtask/src/proof.rs` generalised from a
   three-const single-artefact checker to a table. Seventeen of the eighteen
   rules the WF clauses name now exist under the spelling the clauses use; the
   eighteenth, `condition_round_trips`, is deliberately absent because ADR-0016
   drops it from WF-4 — it rejects nothing the two round-trip rules do not
   already reject.
7. **This commit — `spec-trace`'s parser, D12's manifest lint, and the
   documents.** Planned as two and landed as one; the reason is the last finding
   below.

**`frozen_signatures.rs:221-235`'s `compile_fail` doctest has never run, and this
is phase 4's artefact found by phase 5.** rustdoc collects doctests from the
**lib target only**, so a `compile_fail` block inside an integration test is
compiled as prose — it is never handed to a compiler and can never fail. Verified
twice, independently: `cargo test -p happenstance-core --all-features --doc --
--list` lists 28 doctests and every one of them comes from a file under `src/`;
nothing from `tests/` appears at all. Phase 4's proof artefact is *"plus a
`compile_fail` doctest pinning the arrangement that does **not** compile"*, and
that half of it has been decorative since the day it landed. **It is recorded
here rather than silently fixed.** WF-12's own instrument moved from a
`compile_fail` doctest to a const assertion in `tests/wire.rs` for the adjacent
reason — three of four spellings were measured green against a false assertion —
and that is the precedent: an instrument that cannot fail is a finding, and a
finding belongs to whoever owns the clause, not to whoever tripped over it.

**`postcard`'s default features fail `cargo deny`, and the flag is the whole of
that commit's surprise.** The default turns on `heapless-cas`, which reaches
`heapless 0.7` and then `atomic-polyfill 1.0.3`: unmaintained,
RUSTSEC-2023-0089, *"no safe upgrade is available"*, and `cargo deny check
advisories` **fails** on it. Measured rather than reasoned about — the first
attempt at the commit added **eleven** crates and turned the gate red. With
`default-features = false` it adds **three** (`postcard`, `cobs`,
`embedded-io`), all dev-only, all four `deny` checks pass, and the
duplicate-crate warning list is unchanged at seventeen. `base64` is pinned
`"0.22"` and not `"0.23"` on the same kind of measurement: `sqlx` already pulls
`base64 0.22.1` into this graph and `base64` has no transitive dependencies of
its own, so the addition is a new **edge to an existing node** — zero new crates
for the workspace, exactly one leaf crate for a consumer taking
`happenstance-core/serde` without `sqlx`. That is the number ADR-0003 should be
priced against.

**`postcard::Error` is a fieldless enum, so a marker string does not survive the
crossing — and the test found it on the first run.** `serde::de::Error::custom`
takes a `Display` and `postcard`'s implementation drops it **entirely**, mapping
every custom error onto one variant. `WireError`'s marker text therefore crosses
`serde_json` and does **not** cross `postcard`. The doc on that constant now says
so and points `postcard` receivers at `check_format_version`, which is what it
was made public for. Worth stating as a general fact rather than as a bug: any
contract carried in the *text* of a serde error is a contract only
self-describing formats keep.

**`spec-trace`'s `Rule:`-field parser demands any backticked snake_case token in
the field, and ADR-0016's own amendment texts tripped it three times before they
landed.** The whole field is handed to `backticked_idents`, so a clause that
writes *"`wire::round_trips_in_postcard` over every envelope shape, including the
all-defaults `event`"* has just demanded a rule named `event`. The rule is: **a
`Rule:` field may backtick rule names and nothing else.** `retires_of` already
carries that comment one field over; `rules_of` did not, and now does. It is
load-bearing rather than tidy — it is the rule this phase's own ADR broke three
times.

**Commits 7 and 8 of the planned sequence landed as one, and the argument is
§15's one level up.** The plan had `spec-trace`'s parser fix as one commit and
the clause amendments as the next. They cannot be separated: making
`backticked_idents` accept `::` un-silences ten WF clauses at once, and two of
the names they then resolve are **stale** — WF-4's `wire::condition_round_trips`,
which ADR-0016 §7 drops, and WF-8's `wire::version_is_the_first_field`, which §11
replaces with a witness. So the parser fix alone leaves check 4 reporting two
clauses naming rules that do not exist, and **writing more tests does not clear
it**, because the fix is to the clauses. §15 already makes this argument about
the three changes *inside* the parser fix — that a phase landing only the obvious
half breaks a gate step and cannot repair it by doing more of the work the
clauses ask for. The same shape holds one level up, between the tool and the
document it checks.

**That box was the last one open, and the run that closed it is the one this text
landed in.** `cargo xtask spec-trace` reported the two stale rule names and a
§7.1–§7.2 region stale against what the checker now computes for VT-19 and
WF-1 – WF-9; the clause amendments and one `--write` closed both. The checker now
reports `200 clauses (139 FROZEN, 49 PROVISIONAL, 10 DEFERRED, 2 NON-NORMATIVE),
89 suite rules, 58 e2e cases` and `no problems found; §7.1–§7.2 matches the
checker`, and `cargo xtask ci` prints `all checks passed`. The census is
**unchanged** from before the phase, which is the intended outcome: ADR-0016 §14
gives D12 a manifest lint rather than a WF-13 precisely so that §1.3 does not move.

**2026-08-09 — the completeness critic ran, and the citations were the story.**
It was recorded here as never having run, with three questions attached; this is
the answer to all three, taken by seven parallel auditors — one per "Owed to"
bucket of ADR-0016's amendment ledger, plus one for the census and one for the
citations — each of whose findings was then handed to an independent verifier
prompted to *refute* rather than to confirm. 300 items checked, 16 findings
confirmed and one refuted — 14 distinct, two of them reported independently by
two auditors each, which is the only cross-check a fan-out of this shape gets for
free. A seventeenth was found not by reading but by *running*, and it is the one
below that matters most. The refuted one is worth naming because it is the shape
the verifier exists to catch: an auditor read WF-12's "fifth hazard" paragraph,
saw a citation to impls the phase had deleted, and reported the paragraph as
unamended — when the paragraph *is* the amendment, and only its anchor was stale.

**Question two first, because its answer is the boring one and it is supposed to
be.** §1.3's census survives a hand count: 200 clause IDs, 139 `[FROZEN]`, 49
`[PROVISIONAL]`, 10 `[DEFERRED]`, two `[NON-NORMATIVE]`, agreeing with §7.1's
totals row and with what `spec-trace` computes. ADR-0016 §14 predicted exactly
that and gave D12 a manifest lint rather than a WF-13 in order to get it. A
prediction that survives a count is worth more than the same prediction trusted.

**Question three is where the damage was: eight of §2.7's twenty-seven source
citations were green and wrong.** Not stale by a few lines — pointing at the
wrong item entirely. `query.rs:380`, cited for `Query::Items` going through
`serialize_some`, is now the line that routes `QueryItem` through its
constructor; `query.rs:394-425`, cited for the `ReadOptions` impls, is now
`Query`'s own wire mirror; three of WF-10's four "the current impls already do
this correctly" anchors landed on a *serialiser* while claiming to name a
deserialiser. **Every one of them passes `check_citations`**, because that check
proves a file exists and a line is in range and nothing else — which is what the
question was written to expose, and it was right to suspect it.

The mechanism is worth stating, because it will recur in every phase that
amends the specification and the code together. ADR-0016's amendment texts were
written **before** the code moved, against anchors that were correct the day the
ADR was written. The phase then applied the amendments *and* moved the code, and
the amendments carried their own anchors in verbatim. So the defect is not
carelessness; it is that an ADR which specifies replacement prose containing line
numbers is specifying something guaranteed to be stale by the time it lands. The
repair applied here is to anchor on the **item** — `EventWire`, `GuardWire`,
`QueryItemWire` — and give its current range, rather than on the line an
attribute used to sit on. A closed defect names its commit; a live citation
should name a thing that can still be found by looking.

**Question one found one genuine gap and one hole nothing else could have
found.** The gap: ADR-0016's "Owed to the code" asks for the `Deserialize` half
of `Detect<T>` in `experiments/wire-format/src/lib.rs`, "so §13's HRTB
finding stops resting on a throwaway crate", and it was never added — the half
landed only in the real target. The hole is worse and no document audit could
have reached it: **that crate's own test was failing.**
`decorative_readoptions_const_assert` asserts `Detect::<ReadOptions>::IS_SERIALIZE`
is `true` — "measured claim: `ReadOptions` is `Serialize` at HEAD" — and WF-12
deleted that impl in this very phase, so the assertion panicked. It is outside
the workspace and outside the gate, exactly as `experiments/` is meant to
be, so nothing observed it. **The lesson is not that experiments belong in the
gate**; it is that "a record of a measurement taken on a date" and "an `assert!`
that runs" are different artefacts, and this file was both. The assertion now
moves with the tree and the dated reading is recorded in prose beside it, with a
positive control added on the same argument the wire tests already make: a
negated assertion is otherwise satisfied by a detector whose inherent impl never
applies, and that failure is silent.

**And WF-12's `Cases:` line never gained E2E-58**, while §7.5 said "WF-12 closed
at phase 5 with E2E-58" and §7.6 said "all 58 cases are claimed by at least one
clause". The case existed, the two summaries claiming it existed had both been
written, and the one line that would have made either true had not — so §7.6's
stated method, parsing every clause's `Cases:` line, could not have produced its
own stated result. Its method paragraph was stale in three places at once
(`E2E-01`…`E2E-57`, "its 57 headings", and an index citation off by one at both
ends), which is the same class as question three and the reason to fix both in
one pass. One thing learned while fixing it: a `Cases:` line is parsed for
**every** `E2E-nn` it contains, so a comparison drawn in prose there — "as VT-17's
did with E2E-57" — is read as a claim. The line now says so.

**State at hand-off.** Phase 5's work is complete and committed on
`phase-5-freeze-the-wire-format`; what is left is a pull request and a merge,
matching how phase 4 landed (`19f0901`). One thing is recorded here rather than
done, because it is not this phase's:

- **`wire::bare_event_postcard_encoding_is_pinned` is named by no clause.** It is
  the golden vector for a bare `Event`'s postcard bytes, and it is deliberately
  uncited: after §15, resolution is one-way — clauses may name wire tests, wire
  tests need not be named by clauses — and check 6 sweeps only the suite's own
  rules. If a later phase wants the encoding itself frozen rather than merely
  pinned, that is the clause to write and this is the test it would name.

---

## Between 5 and 6 — the reconciliation nothing owned

Not a phase. A pass that had to happen and that this plan had not scheduled,
recorded here so the next one is scheduled rather than noticed.

**What it was.** Six commits on `redkiln-adoption`, `3c704d3` through `84dcc67`,
reconciling [`SPECIFICATION.md`](spec/SPECIFICATION.md) with the tree
phases 4 and 5 produced. Sixteen clauses stated things about the code that were
false — `AppendCondition` described as having public fields after VT-30 made
`guards` private, `ReadOptions` described as having no upper bound beside a `to`
that shipped, `SequencePosition::next` described as `saturating_add` after it
became `checked_add`. Nine documentation MUSTs that `[FROZEN]` clauses impose on
`happenstance-core`'s own doc comments had never been discharged; five tests that
clauses name did not exist. The evidence is
[`references/evaluation/phase-4-5-reconciliation.md`](references/evaluation/phase-4-5-reconciliation.md).

**Why nothing caught it.** `check_citations` required a citation to be
path-qualified *and* name Rust or a manifest. Of 338 citations, 84 satisfied both.
The other 254 — 200 bare file names, 56 into Markdown — were never parsed, so the
step reported "no problems found" over a quarter of the corpus. It now reads 358
and states its coverage in the summary line, which is the half of that change
worth keeping: **a check that does not say what fraction it sees cannot be
distinguished from one that sees all of it.**

**Why this file did not schedule it.** The last reconciliation was phase 3's —
ADR-0008's four amendments and phase 2's fourteen falsified claims, pooled there
because that phase was already opening §6 and §7. Phases 4 and 5 were the two
largest changes to the contract in the plan and neither carried an equivalent
item. The rule that would have caught it is already written down, at the end of
[the ADR queue](#the-adr-queue): *a phase's clause range and the union of its
ADRs' clause ranges are two numbers, and nothing checks that they are equal.*
Nothing implements it either.

**So it becomes a standing exit criterion.** Every phase from 6 onward, before
its box is ticked:

- [ ] Every clause the phase's ADRs discharge has been read against the code as
      it now stands, not as it stood when the clause was written. A clause whose
      supporting prose describes a superseded implementation is a defect even
      when its MUST is untouched.
- [ ] The phase's clause range and the union of its ADRs' clause ranges are
      computed and compared.
- [ ] `cargo xtask spec-trace`'s citation count has not fallen, and any clause the
      phase froze names a rule that exists or is marked `†`.

**What it deliberately did not do.** It wrote no ADR. Seven findings needed one
and are recorded rather than decided — the sharpest being that
`k_disjoint_boundaries_admit_exactly_k_commits` enforces the central DCB
independence proposition and no clause states it; the word "disjoint" does not
occur in the specification. Two are held in `UNCLAIMED_PENDING_ADR` in
`xtask/src/spec_trace.rs`, which prints them on every green run. The rest are in
[`.kb/open-questions/`](.kb/open-questions/) — one atom per gap, since the
intake wave of 2026-08-10 refused to merge six independent questions into one —
indexed at [`.kb/maps/open-questions-index.md`](.kb/maps/open-questions-index.md),
for the next ADR pass.

**Session log**

- **2026-08-10.** Audited all 338 citations across two parallel passes and an
  adversarial refutation; 128 pointed at a different item than the sentence citing
  them claimed. Repaired ~130, discharged nine documentation MUSTs, wrote the five
  missing tests, widened check 6 to all three rule files, and landed a content
  anchor that immediately found two more the sweep had just repaired into the
  wrong place. Independently corroborated by
  [`review-citation-drift.md`](references/evaluation/review-citation-drift.md), written the
  same day from the `standards/rust/` work, which found six of the same citations and
  recommended the same remedy. Two passes converging on one root cause is the
  argument that it is structural.

---

## Phase 6 — Freeze `ProjectionStore`

**Goal.** The port settled against two structurally different implementations, its
conformance suite written, and a reference implementation shipped so the port has
an oracle and a doctest that cannot rot.

**Why here.** A port frozen by one implementation is a port shaped like that
implementation, and `grep -rn "ProjectionStore for"` matches nothing in the
workspace today — so it is currently shaped like nothing at all.

**Decisions it settles.** ADR-0017 (the batch), ADR-0018 (reset), ADR-0019
(failure policy). Discharges PS-1 – PS-37.

**What phase 2's evidence says.** `type Batch<'a> = rusqlite::Transaction<'a>`
compiles on the **bare** flavour and fails on `SendProjectionStore` for two
independent reasons: `Connection` is `Send` but not `Sync`, so `&Self` is not
`Send`; and `Transaction<'_>` is not `Send`, so the `commit` future cannot be. The
GAT's stated justification at `projection.rs:80-82` is therefore unearned on the
flavour every native adapter will implement. `sqlx::Pool::begin()` supplies the
confirming half from the other direction: `Transaction<'static, Postgres>` owns
its `PoolConnection`, carries no borrow of the store, and is `Send`.

**Work**

- [ ] **Drop the lifetime: `type Batch;`** (PS-4 – PS-6). An adapter buffers its
      read-model writes into an owned value and opens the transaction inside
      `commit`, alongside the checkpoint write — which preserves the invariant the
      port exists for and satisfies `Send`. **Do not claim this makes the
      foreign-batch hazard unrepresentable**: that was compiled and refuted
      (`PRESSURE-TEST.md:179-201`). A lifetime names a region, not an instance, and
      two `&Store` references unify to a common region; even tying the batch to the
      receiver's lifetime accepts `let b = a.begin(); other.commit(b);`. Only a
      generative brand rejects it, and PS-15 records that as provisional rather
      than pretending the owned shape closed it.
- [ ] **The write seam** (PS-9 – PS-11). The port grows one *because the suite
      needs one*. A callback-driven pump can be written against the port as it
      stands — that was compiled — but generic suite code holding a `P::Batch` can
      only pass it to `commit` or `rollback`, so three of the six projection rules
      cannot observe the read model at all, and a suite that cannot reject a store
      which commits the checkpoint and drops the read-model write is decorative by
      CLAUDE.md's own corollary. The suite additionally needs a fixture trait
      supplying `write_probe(&mut Batch)` and an out-of-band `read_probe(&Store)`.
- [ ] **`CheckpointOnlyStore`** (PS-2) — the hostile store that commits the
      checkpoint and silently drops the read-model write. It is this phase's whole
      bar: if it passes, the port is not frozen.
- [ ] **A stated Drop contract** (PS-7). "Dropping a `Batch` without
      `commit`/`rollback` MUST roll back AND MUST release any resource `begin`
      acquired." A reviewer's probe found a dropped batch permanently losing the
      connection, after which the store returned `Busy` forever — so the rule is
      not "rolls back", it is "and the store remains usable".
- [ ] **`MemoryProjectionStore`** behind the `memory` feature — the oracle, the
      doctest target, and something application authors can test against before any
      real adapter exists. About 50 lines. Its absence is why the port has a
      cold-start problem: implementing it currently fails with `E0195` unless you
      spell the parameter `Self::Batch<'_>` (`PRESSURE-TEST.md:236-251`), nothing
      says so, and there is nothing to copy. Document the diagnostic (PS-34, PS-36).
- [ ] **`reset`** (PS-16 – PS-20). Clears rows and checkpoint in one unit of work;
      scoped to one `(store, ProjectionId)`; refusable by a projection that must not
      be rebuilt; and **not** `commit(empty, id, FIRST)`, which silently skips
      event 1 — the substitute all six scenarios reached for and all six got wrong.
- [ ] **`commit`'s position** (PS-21, PS-22): it is the position *considered*, not
      the position applied, so a chunk containing no matching event still advances.
      Whether it may regress is provisional and belongs to reset's ADR.
- [ ] **Read-your-writes within a chunk** (PS-12). Two events touching the same
      row in one `begin`/`commit` cycle. The three candidate answers — the batch is
      read-your-writes, the runner guarantees one event per batch, or a projection
      may not read what it writes — are all design decisions, and E2E-21 is what
      forces one.
- [ ] **The failure policy** (PS-26 – PS-30). Per projection, not per runner; a
      failing `apply` names the position it failed at; one poisoned projection does
      not stall the others; a panicking `apply` rolls back. Halting is correct for a
      revenue ledger and wrong for an availability board, and a deliberate
      crypto-shred needs a fourth option — "skip and record" — that neither retry,
      halt nor dead-letter covers.
- [ ] `projection_store_conformance!`, emitted through phase 1's registry so it
      inherits the tokio/blocking/wasm flavours.
- [ ] **Decide the 0.1 exposure** (PS-3). Either the port is genuinely frozen, or
      it ships behind an off-by-default `unstable-projection` feature with a doc
      paragraph exempting it from semver until a third adapter has cleared the
      suite — the `tokio_unstable` idiom. The second decouples publication from
      this phase entirely and is the honest option if the two batch shapes disagree.
- [ ] Correct ADR-0007's Context (PS-32): a runner that itself writes into the
      batch cannot be written today; a callback-driven one can, and was compiled.
- [ ] State that outward-writing projections are out of scope (PS-31). A
      documented exclusion is not adapter-checkable, and it is in the clause space
      because silence here is what produces the wrong implementation.

**Proof artefact.** `CheckpointOnlyStore` **failing** the projection suite, and two
implementations at opposite ends of the batch-shape axis passing it: the phase-2
rusqlite skeleton fleshed out far enough to commit a real transaction, and
`MemoryProjectionStore`. Plus the Ladybug and Postgres skeletons compiling against
the frozen shape — the second being the only one whose transaction crosses a
network.

**"Without amendment" is the wrong bar and the previous revision used it.** This
phase drops the GAT, so every skeleton that spells `type Batch<'a>` must change;
an exit criterion requiring them to compile unchanged is unsatisfiable by the very
decision the phase makes. The bar that carries information is narrower: the
skeletons' `Batch` *types* — `sqlx::Transaction<'static, Postgres>`, Ladybug's
owned write handle — are unchanged, and only the lifetime parameter's removal
differs. Phase 4 owes the same distinction for the same reason.

**Exit criteria**

- [ ] ADR-0017, 0018, 0019 written first, quoting the compiler errors from
      `references/adapter-shapes.md` rather than asserting the port "survives".
- [ ] `CheckpointOnlyStore` fails at least one named rule.
- [ ] Two implementations green against the projection suite.
- [ ] `projection.rs` no longer says "provisional", **or** the module is behind
      `unstable-projection` and says why.
- [ ] The Ladybug and Postgres skeletons compile with **no change other than the
      removal of the batch's lifetime parameter** — same underlying `Batch` type,
      same error type, same bodies.

**Cases this makes writable.** E2E-15 – E2E-24, E2E-26 – E2E-29, E2E-50, and the
port half of E2E-25 and E2E-31.

**Estimate.** 6 days.

**Session log**

---

## Phase 7 — The typed layer and the worked example

**Goal.** `happenstance` — `DomainEvent`, `DecisionModel`, `Codec`, the command
loop, the application-facing `Projection` trait and runner, a testing DSL — and
the worked example rewritten on top of it.

**Why here.** This is the consumer that discovers contract defects, and it needs
only `MemoryEventStore` to do so; discovering them after the flagship adapter is
written costs the adapter a rewrite.

**Decisions it settles.** ADR-0020 (fold/query agreement), ADR-0021 (payload
evolution). Evaluates PS-33 — ADR-0007's falsifier.

**The hazard this phase exists to close.** A DCB handler names its event set twice
— in the `Query` it reads with, and in the `match` arms it folds with. Add a type
to the fold and forget the query, and the decision is made on state that excludes
those events *and* the append condition fails to guard against them. Both halves
of DCB break from one omission, with no compiler, clippy or conformance signal.
It is undetectable by review because the omission is an absence, and undetectable
at runtime because the store behaves exactly as instructed.
`examples/course-subscriptions/src/main.rs:114-173` demonstrates it today.

**The mechanism, stated as a design constraint rather than a macro decision.**
Exhaustiveness comes from folding over a *decoded* enum, not from a proc macro. If
`DecisionModel::apply` takes `Self::Event` — a domain enum — rather than a
`SequencedEvent`, the `match` is exhaustive and the compiler catches the
divergence. The derive then only removes boilerplate from the `EventType`/`Tags`
mapping, which is what makes deferring `happenstance-macros` past 0.1 defensible.

**Work**

- [ ] `DomainEvent` with `const EVENT_TYPES: &'static [EventType]` — const-constructible
      after phase 5 — designed against a hand-written expansion of what the derive
      would emit, so the trait does not have to move when the derive lands.
- [ ] `DecisionModel` — `type Event: DomainEvent`, `fn apply(&mut self,
      Self::Event)`, `fn query(&self) -> Query` derived from `EVENT_TYPES` and the
      model's tag constraints, never maintained by hand.
- [ ] Composition for tuples `(P1, P2)`, `(P1, P2, P3)`, … by a small macro — the
      axum extractor-tuple trick — so N is compile-time and each model's query
      fragment is OR'd automatically. Composing several models into one query is
      the mechanism that makes a dynamic consistency boundary dynamic.
- [ ] `Codec` — JSON first, CBOR and postcard behind features. Events carry a
      codec tag so one store can hold more than one encoding, which is what makes a
      migration possible.
- [ ] The command loop: read → decide → append → retry on `ConditionViolated`.
      **State the retry policy in the docs, not only in the code**, including that
      after a violation you must re-read and re-decide — which is why consuming the
      batch (phase 4) makes the correct thing the easy thing. Distinguish it from
      the retry-safety property of a *verbatim* resubmission; they are different
      guarantees and collapsing them is a lost update.
- [ ] **The application-facing projection runner** — the `Projection` trait an
      application implements (decoded events, the projection's `Query`, the store's
      own `Batch`). **Corrected 2026-08-17 by ADR-0031, which superseded ADR-0007's
      allocation: no checkpoint pump stays in the contract crate — it collapsed upward and none was ever written.** A projection nominates its events with `Query`, the same
      type a decision model uses; there is no second filter vocabulary. **This is
      the half that had no owning phase in either previous plan**
      (`PRESSURE-TEST.md:270-272`).
- [ ] Record the polling cost that ES-32 makes the runner pay: N views means N
      independent reads of the log. A number here is what a post-0.1 tail-seam ADR
      would be argued from.
- [ ] A given/when/then testing DSL: seed a `MemoryEventStore`, invoke the
      decision, assert on emitted events or the error. This is the ergonomic sugar
      that most directly produces the delight the project is aiming at, and it is
      cheap because the seeding target already exists.
- [ ] `FaultyStore<S>` in `happenstance-testkit` — fail the first N appends, every
      Nth, with an adapter error rather than a violation, fail reads — so a user's
      retry loop is testable at all. Pair with a `GappyMemoryStore` that
      deliberately leaves position gaps, so an application handler assuming
      `position + 1` fails in a test rather than in production. CLAUDE.md already
      forbids the conformance suite from asserting literal positions for this
      reason; application authors deserve the same protection.
- [ ] One paragraph on command-retry idempotency — an HTTP client retrying a POST
      — showing `AppendCondition` and `Event::metadata` as the intended mechanism.
      The crate discusses idempotency twice and neither time is about command retry.
- [ ] ADR-0021: payload evolution. Does `EventType` carry a version suffix? Does
      an upcaster need a read-path hook `EventStore` does not have? Both touch the
      contract's public surface, which is why this cannot wait until it is needed.
- [ ] Rewrite `examples/course-subscriptions`: typed events, a decision model,
      `const` event types, no `format!("…").into_bytes()` payloads, no hand-rolled
      `parse_capacity`.

**Proof artefact.** A `trybuild` compile-fail case: add a variant to the worked
example's domain event enum, and the crate fails to compile until the decision
model's fold handles it. A test that passes proves nothing here; a test that *must
fail to compile* is the whole claim.

**Exit criteria**

- [ ] ADR-0020 and ADR-0021 written first.
- [ ] The `trybuild` compile-fail case exists and is in the gate.
- [ ] `cargo run -p course-subscriptions` demonstrates the library an application
      would actually use — typed events and a decision model, not hand-rolled byte
      payloads.
- [ ] **ADR-0007's falsifier is evaluated and the verdict recorded** (PS-33):
      *"if the checkpoint pump has acquired no independent caller by the time the
      typed layer's phase exits, collapse it upward and supersede this ADR"*
      (`0007:119-121`). Name the independent caller, or write the superseding ADR.
      An unevaluated falsifier is indistinguishable from none, and this one has
      already survived one document handover by living in an ADR no phase read.
- [ ] **The three "does anyone call this?" clauses are evaluated by counting**
      (SPECIFICATION §4.1a, question 4): PS-18's protection variant, PS-27's skip
      record, PS-30's fan-out runner. Each is provisional against a condition that
      cannot be observed by waiting — "no adapter ever implements it" is not
      something that happens, it is something you decide to conclude. So each is
      settled here, on a count: name the callers, or promote the clause to a
      documented exclusion. A provisional marker nobody ever evaluates is the
      failure mode §1.3 forbids, arriving by patience instead of by intent.
- [x] The `happenstance-macros` criterion is evaluated in the session log: *if the
      rewritten example carries more mapping boilerplate than domain logic, the
      derive is in scope for 0.1.* Record the answer either way. **Answered: out.**
      See the session log below and
      [`references/evaluation/phase-7-macros-verdict.md`](references/evaluation/phase-7-macros-verdict.md).
- [ ] **PS-32, PS-33 and PS-35 leave the clause space.**
      `SPECIFICATION.md` §7.3 states that all three are instructions to
      the pass that lands the specification rather than constraints on any
      implementation, that they belong in this runbook's work list, and that they
      were left as clauses only because the work list did not exist yet. It does
      now, and all three are discharged by the end of this phase — PS-35 at phase
      1, PS-32 at phase 6, PS-33 here. Move them out and leave the IDs retained so
      citations resolve, the way CF-30 already is.
- [x] `publish = false` removed from `happenstance`. Discharged early, at phase 0:
      ADR-0006 ships the facade published from day one so that `cargo add
      happenstance` is true throughout, so `crates/happenstance/Cargo.toml` never
      carried the key and the name is reserved at `0.0.0`. Kept rather than
      deleted, because the *other* phases' identical rows are still live and a
      silently vanished one reads as forgotten.

**Cases this makes writable.** E2E-31, E2E-51, the application half of E2E-26 –
E2E-29, and **the runner half of E2E-25** — the chunked rebuild that must not lie
about its own completeness needs the runner that calls `head()`, and phase 6
supplies only the port half. The previous revision left this half unclaimed by any
phase.

**Estimate.** 8 days.

**Session log**

**2026-08-16 — the two records BR-01 asked for. Two documents, no code.** Pinned to
`78a2170`, and the diff touches no path under `crates/**` or `spec/**` — which is
the mechanical form of *a defect is logged and routed, never absorbed*.

- [`references/evaluation/phase-7-contract-defects.md`](references/evaluation/phase-7-contract-defects.md)
  — five entries, each naming a clause ID with its maturity marker, what was
  attempted with the call site, what the contract did, why it is a defect rather
  than a misuse, and the routing. **D-1** and **D-2** are two faces of **VT-18**
  `[FROZEN]`: no infallible `QueryItem` constructor for pre-validated inputs, and
  an infallible `DomainEvent::tags` over a `Tags` with no infallible constructor —
  the second costing the worked example 81 lines of identity newtype. **D-3** and
  **D-4** are the traceability step's own: **CF-36** `[FROZEN]` names a
  cross-reference `spec_trace.rs` does not perform, and check 4's short-circuit
  means no `PS` clause's rule name is resolved at all, so a green `spec-trace` is
  not evidence that a `PS` rule exists. **D-5** records `&mut P`'s N-reads-for-N-views
  shape as evidence for the tail-seam decision rather than as a wrong answer. Two
  further findings are recorded as explicitly *not* entries — one ES-6 and ADR-0009
  already answer, one with no clause ID that is classified `support` at the moment
  of finding rather than promoted by inventing a citation. A reconciliation table
  gives every M2–M6 story that declared a routing an explicit disposition, because
  silence and a dropped finding look identical.
- [`references/evaluation/phase-7-macros-verdict.md`](references/evaluation/phase-7-macros-verdict.md)
  — the macros criterion, answered **out**, and the exit box above is ticked on it.
  The classification is published range by range over
  `examples/course-subscriptions/src/main.rs` and the 29 ranges partition the file
  exactly, 532 of 532, so the totals are re-derivable rather than asserted: 40 lines
  of ceremony against 249 of domain, and 125 : 249 even with the whole contested
  identity block counted as ceremony. **Both extremes are out**, so the contested
  block is a footnote rather than the decision.

  **This contradicts the design's own recorded prediction**, which was 2.4 : 1 and
  *in*, and that is a successful outcome rather than a problem: the prediction was
  written down as falsifiable so a measurement could falsify it. The two disagree
  because the design counted a minimal doctest, where the domain is almost absent —
  the `DomainEvent` impl is a fixed cost that grows barely at all (26 lines for two
  variants, 40 for three) while the domain grows with every consistency concern,
  refusal and handler. Both numbers are true; they answer different questions, and
  the decision-table row at the head of this file now says which is which.

Both records are staged for `/redkiln:kb-ingest` at
`.kb/_intake/contract-defect-log-phase-7.md` and
`.kb/_intake/happenstance-macros-verdict.md`, carrying **proposed** frontmatter for
the wave to author from. **No `.kb/` atom was hand-written and no ingest was run** —
that is a human handoff. In particular the macros verdict is staged as its own claim
and **ADR-0020 was not edited**: it is accepted and therefore immutable, and its own
spec assigned this verdict elsewhere. Whether the wave produces a supersession is
the wave's adjudication.

No `crates/happenstance-macros/` was created. An *out* verdict escalates nothing.

---

## Why the first release is `0.2.0` and not `0.1.0`

The plan said `0.1.0` until phase 4 landed. It moved because **phase 4 is a
breaking change to a version that had not moved**, and `cargo-semver-checks`
said so on the pull request: seven major lints against `happenstance-core`,
every one of them a decision an ADR had already taken — `Query::Items` gaining
`#[non_exhaustive]` (VT-26), `AppendCondition`'s public fields becoming a private
guard sequence (VT-30), `SequencedEvent::new` going from two arguments to four
(VT-4), `Event::new`'s widened bound (VT-18), `head` and `contains_event_id`
arriving on the trait (ES-30, ES-41), and `AppendCondition`'s constructors
ceasing to be `const` because a boxed slice allocates.

Nothing is published, so nothing downstream broke. The number still has to move,
for the reason this repository applies to every other gate: **the finding was
true, and a true finding is not silenced.** In `0.x`, the breaking position is
the minor, so `0.1.0` becomes `0.2.0` and the planned alpha becomes
`0.2.0-alpha.1`.

`happenstance-testkit` moves with it, on its own reasoning rather than by
inheritance — CF-32 gives it an independent number precisely because adding a
conformance rule can turn a passing adapter's CI red, and phase 4 added
thirty-four.

## Release: `0.2.0-alpha.1`, here

**Publish `0.2.0-alpha.1` immediately after phase 7, and not before.**

Name reservation and feedback have different deadlines and different instruments.
Reservation is urgent and phase 0 handles it. Feedback is only worth having about
something a person can use: today the installable surface is a contract crate plus
an in-memory store, so a reader can review it but cannot build anything, and
between phases 4 and 6 the API changes daily *by design* — an alpha there
generates issues about churn rather than about design. After phase 7 a person can
write a real domain model, test it with the given/when/then DSL, and run it. That
is the first version whose feedback is about the decisions actually worth
challenging: the decision-model composition, the retry policy, and whether the
typed layer's ergonomics clear the `disintegrate` bar.

Waiting for SQLite costs two more weeks of not listening, for a benefit —
durability — that none of that feedback depends on.

Three cheap mitigations against an alpha being read as a commitment: a
`## Stability` section in the README naming the phase at which the API stops
moving; a `CHANGELOG.md` that lists what broke between alphas, which is the
artefact that converts "it churned" into "it churned for these stated reasons";
and yanking each alpha when the next lands, so the resolvable set is always one
version. Cargo will not resolve a pre-release without an explicit pre-release
requirement, so nobody gets it by accident.

The failure mode to guard against is not users; it is the author. An alpha in the
wild makes "we can't change that now" available as an argument, and that argument
is the bias the previous runbook diagnosed at its own `:457-464` and did not act
on: treating an unpublished API as something to protect biases every decision
toward the option that changes least, which is not the same as the option that is
best.

---

## Phase 8 — `happenstance-sqlite`

**Goal.** The flagship adapter, complete: event store and projection store, every
conformance macro green, plus the benchmark harness.

**Why here.** It is where frozen decisions get *spent*, not where they get made —
every ADR-0022 decision is a storage or performance choice that presupposes a
settled port.

**Decisions it settles.** ADR-0022. Discharges CF-14 and CF-17 — durability across
a reopen, which nothing in the workspace can currently express.

**Two amendments the evaluation supplies to the schema sketch.** The sketch's
`event_tag(tag, position)` carries no type, so a `QueryItem`'s type constraint
becomes a join back to `event` — which collapses SQLite's `MERGE (UNION)` streaming
plan into a co-routine over a temp b-tree, and makes the append-condition probe
walk the full posting list through a join *while holding the `BEGIN IMMEDIATE`
write lock*. That serialises every writer. Carry `event_type` as a covering column
on `event_tag`, keeping the key `(tag, position)` so the range stays sorted by
position, and add a `tag_cardinality` table: multi-tag arms must be probed
most-selective-tag-first, and SQLite cannot supply per-value cardinality —
`ANALYZE` stores only an average.

**Work**

- [ ] **Claim `happenstance-sqlite` on crates.io**, per phase 0's
      rule that a name is reserved when its phase starts, not before — the point
      being that by now there is a crate to justify it with.

- [ ] ADR-0022, including the schema and the `EventId`/`recorded_at` columns from
      phase 5 in migration 1. `AUTOINCREMENT` is load-bearing: positions must never
      be reused after a delete, and plain `rowid` does not guarantee that.
- [ ] `read` — the real stream, with the `spawn_blocking` deferred into
      `poll_next`.
- [ ] `append` — condition evaluation and write in one transaction, returning the
      caller's own last position.
- [ ] `SqliteProjectionStore` against phase 6's owned `Batch`.
- [ ] Handle `Query::index_arms()` exceeding SQLite's pushdown limits by chunking
      — prepare `ceil(arms/400)` statements and merge their cursors in Rust —
      rather than failing. `Query` bounds nothing by design; the limit is
      adapter-specific and a hard cap in the contract would be wrong.
- [ ] `event_store_benchmarks!(fixture)` in the testkit behind a `bench` feature:
      append throughput, conditional append under contention, replay of N events
      with and without a tag filter. Adapters inherit it the way they inherit
      conformance, which no other Rust DCB library does. It is **not** a
      conformance rule (CF-34): an adapter that scans where it should seek passes
      every rule that can be written, and a suite that asserted on timings would be
      flaky.
- [ ] Two connections onto one database file, so ES-34 and the durability rules
      have an adapter that can actually supply the fixture's second handle.

**Proof artefact.** The concurrency macro green under a multi-thread runtime at 64
contenders — a read-then-write adapter fails that within a handful of iterations
and passes the sequential rule forever — plus an acknowledged write surviving a
genuine process reopen, which is the first time the workspace can even ask the
question. Third: the phase-3 mutant harness re-run with `SqliteEventStore` in the
pass column.

**The 64 is now the code's number too, and the "25 rounds" that used to sit beside
it is gone rather than carried forward.** `concurrency::CONTENDERS` was 8 for two
phases while this line and the status table both read 64; the discrepancy is stated
at `:2686-2696` above, along with the observation that leaving it is the option that
rots. It was resolved by *raising the constant* — ADR-0022 §12 measured sixty-four
`rusqlite::Connection`s on one file with no ceiling reached, exactly one winner per
race and `busy = 0`, at a cost of one order of magnitude in wall time — and the doc
comment at `crates/happenstance-testkit/src/concurrency.rs` now carries that reason
and the workspace-wide cost. The *rounds* half was the other error and had no fixing
by a number: the only `ROUNDS` in the family is a rule-local `4` inside
`a_concurrent_reader_never_sees_a_partial_batch`, which is one rule's shape rather
than a property of the family, so the phrase is struck instead of restated.

**Exit criteria**

- [ ] All four macros green: conformance, model, concurrency, reopen.
- [ ] A benchmark *number* in ADR-0022, not a claim. A decision deferred to a
      measurement that never happens is not a decision.
- [ ] No `todo!()` on any SQLite path.
- [ ] `publish = false` removed.

**Cases this makes writable.** The durability half of E2E-07 and E2E-46's
precondition; E2E-08 against a real second handle.

**Estimate.** 10 days.

**Session log**

**2026-08-16 — the ADR pass. ADR-0022 written and measured; no SQLite code.**
State stays `not started`, because a phase is done when its proof artefact exists
and phase 8's does not: no macro has been run against `SqliteEventStore`, every
`todo!()` is still there and `#![allow(clippy::todo)]` is still at the top of
`crates/happenstance-sqlite/src/lib.rs`. Nothing under `crates/happenstance-sqlite/`
moved at all, which is the ordering AC-013 exists to prove.

What exists is
[`references/adr/0022-append-condition-strategy.md`](references/adr/0022-append-condition-strategy.md),
`experiments/append-condition/` and two files staged in `.kb/_intake/` for a
human-invoked `/redkiln:kb-ingest`, plus the benchmark family the record was
measured through — `happenstance_testkit::bench`, behind an off-by-default
`bench` feature, which is the phase's `event_store_benchmarks!` work item landed
early because ADR-0022 is its first paying customer.

**The exit criterion "a benchmark *number*, not a claim" is met, and the number
overturned this phase body's own recommendation.** The append condition is a
`SELECT max(position)` guard inside `BEGIN IMMEDIATE` rather than the `EXISTS`
probe the schema amendment above assumes: on the rejection path — the one a DCB
loop takes whenever it loses a race — the guard costs 23 µs against the probe's
32 and the conditional insert's 45 over a 5,000-event log, and 213 against 311
and 306 over 50,000. On the accepted-append path and under contention at 8 and 64
connections the three arms are a **tie within measurement noise**, and the record
says so rather than manufacturing a margin.

**Three things this pass found that this phase body did not anticipate.**

- **`GROUP BY … HAVING COUNT(DISTINCT tag)` is an optimisation barrier.** SQLite
  cannot push an append condition's `position > ?` boundary through an aggregate,
  so the probe materialises every matching position and discards the ones below
  it. Skipping the aggregate for a single-tag item halved the probe (1,093 µs →
  556 µs, measured against its own negative control). A **two-tag** boundary
  costs roughly **200x** a single-tag one at 50,000 events, which promotes
  `tag_cardinality` and most-selective-tag-first probing from the amendment's
  "add a table" to a hard requirement of migration 1.
- **64 contenders is supportable and costs one order of magnitude.** Sixty-four
  `rusqlite::Connection`s opened on one file on every race, with `busy = 0` and
  exactly one winner each; a race goes from ~130 ms at 8 to 1.4-2.7 s at 64. The
  proof artefact above reads 64 while `concurrency::CONTENDERS` is 8, and that
  raise now has a measured claim behind it. This pass **did not apply it** —
  `crates/happenstance-testkit/**` is untouched by the ADR change.
- **ES-17 cannot be lifted here, and nothing in this phase's plan lifts it.**
  ADR-0012's falsifier item 1 wants two builds of the *same* adapter; three
  candidate stores are not that. Escalated to the ADR queue as a row of its own
  rather than absorbed — see the queue, immediately under row 0022.

The measurement's own limitation is recorded rather than hidden: on a shared
developer host the harness's per-scenario timer gives each arm its own time slot
and two runs an hour apart disagreed by up to 45%, so the two figures the record
decides on come from caller-side controls that interleave the arms round-robin in
one process. That is the emitter's job by design (CF-23), and it is the strongest
argument this pass produced for the wrapper being a parameter.

**Session log**

---

## Phase 9 — Cloudflare Durable Object

**Goal.** The phase-2 skeleton finished against the real `SqlStorage` API, passing
the suite under `workerd`.

**Why here.** Its *design* risk moved to phase 1, which already proved a `!Send`
store can implement the port and that the rules run off tokio — so this is
integration, not design validation, which is what makes it safe off the 0.1 path.
It depends on phase 4 as well as 2, because its schema needs the identity and time
columns like every other store — a dependency the previous plan omitted
(`PRESSURE-TEST.md:558`) and then mis-attributed to phase 5, which is where those
columns lived before the contract freeze was re-split.

**The crate is `happenstance-cloudflare`** (verified free 2026-08-06). It names
the vendor rather than the primitive, which is a deliberate departure from the
`-sqlite`/`-postgres`/`-neon` convention: the distinguishing property here is the
whole Workers execution model — `!Send`, single-threaded, off tokio — and not the
storage engine, which is SQLite like two other adapters. The cost is that a second
Cloudflare storage primitive would need a name this one has taken; if that happens,
this crate keeps the Durable Object and the newcomer is named for its primitive.
**Claim the name at the start of this phase**, per phase 0's rule.

**Decisions it settles.** ADR-0023. Confirms or refutes ADR-0009's ES-6 prediction.

**Work**

- [x] ADR-0023: the `SqlStorage` mapping and the off-tokio harness. **Written and
      accepted** as
      [ADR-0023](.kb/decisions/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md),
      by the `2026-08-20-intake-phase-9` wave. What landed is **not**
      `vitest-pool-workers` as its own CI job: the conformance suite executes on
      `wasm32-unknown-unknown` under `wasm-bindgen-test-runner` against a
      `node:sqlite`-backed `DurableObjectState` shim, driven by **one row** in
      `xtask/src/proof.rs`'s executed-target registry, **inside one `cargo xtask ci`**.
      The separate-job shape lost to *the same run as the rest of the gate* — a
      separate CI job is a claim about CI, and nobody's local gate checks it
      (`references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md:157`).
      A `workerd`-class runner is not rejected on merit and stays open at
      `kb-open-question-workerd-runner-absent-001`.
- [ ] Finish the skeleton; run the registry's `__emit_wasm` flavour.
- [ ] Record whether the `!Send` `Error` asymmetry ADR-0009 predicted actually
      bites — this is the adapter that decides it, and whether stringifying a
      `JsValue` loses information the caller needs.
- [x] Record whether a Durable Object's storage API makes a tail or subscription
      seam cheap enough to reopen ES-32 post-0.1. This is the tail-seam ledger
      row's work item; the previous plan named an owner and gave it nothing to do.
      **Recorded** in this phase's session log, *The tail seam (ES-32) — the
      verdict*, and recorded only: no port method, no sub-trait and no alarm
      plumbing lands with it.

**Proof artefact.** Two things, because the first does not decide ES-6 and the
previous revision offered only the first.

**Every conformance rule green in a real Workers runtime**, with ADR-0001's
`provisional` marker formally retired and this adapter cited. Phase 1's `RefCell`
store lifted the marker; this is the full proof ADR-0001 named.

**And a committed `WorkerStoreError` that actually carries a `worker::Error`**,
together with a test that reconstructs, from the error a caller receives, the one
fact a caller needs to branch on — whether the failure was a constraint violation
or a transport fault. ES-6's whole question is whether stringifying a `JsValue`
loses information the caller needs, and a green suite exists whether it does or
not: every conformance rule asserts on the *success* path or on a store-produced
`AppendError`, and none of them reads an adapter error's contents. Without this
half, the exit criterion "ES-6 is decided" has no artefact behind it and is
discharged by assertion.

**Exit criteria**

- [ ] Every rule runs and passes under `workerd`, with capability skips reported
      rather than silent.
- [ ] ES-6 is decided — the bound is added, or the deferral is renewed with the
      compiled reason from a real `!Send` error type.
- [x] A one-paragraph verdict on the tail seam, in this file's ledger. **Ticked
      because the paragraph is on disk**, in this phase's session log below, under
      *The tail seam (ES-32) — the verdict*; the two ledger rows that promised it
      point at it rather than repeat it.
- [ ] `publish = false` removed; `cargo xtask ci` green including the wasm32 step.
      **Half done, and the halves are recorded rather than averaged.** The
      manifest flag is gone and `happenstance-cloudflare` is in `PUBLISHABLE`
      (`xtask/src/package.rs`) — the pair that puts this crate inside the step
      asserting both licence texts and its README are in the packaged artifact,
      on every run rather than in one working tree. Every one of the seven steps
      `wasm_steps()` names is green, and so is `cargo xtask ci --fast`, which is
      the bar this non-terminal project's integration gate holds. The box stays
      **unticked** because the sentence says *`cargo xtask ci`*, and the full
      gate is red at one `OPTIONAL` step: `cargo deny check` fails `bans`,
      because `worker` and `worker-macros` depend unconditionally on
      `async-trait`, which `deny.toml` bans under ADR-0001. That is
      `kb-open-question-worker-async-trait-ban-001` — open on purpose, ratified
      by nobody, and not settled here: widening the `wrappers` list to make this
      box tickable would delete the finding the ban exists to produce, which is
      exactly what the slice repair of `2ea99fd` reverted once already.

**Cases this makes writable.** The real-adapter half of E2E-30, E2E-52 and E2E-54.

**Estimate.** 8 days.

**Session log**

**2026-08-19 — the tail seam (ES-32) — the verdict.** *This answers the falsifier
the provisional-clause table poses — "a Durable Object making one cheap enough to
reopen" — and only that one; the clause's own benchmark-shaped falsifier, E2E-32's
fan-out runner holding N views inside their staleness budget, is untouched and
stays phase 7's.* **A Durable Object does make a tail seam cheap to reopen
post-0.1, and it makes it cheap in exactly the additive shape ES-32 already
names — which is a confirmation of the clause rather than a case against it.** The
real bindings settled the decisive property rather than a preference: a Workers
`SqlStorage` cursor **is not a snapshot and cannot be held across an `await`**
(`crates/happenstance-cloudflare/src/sql_storage.rs:1-24`, property 2 — the reason
ADR-0011's ceiling-and-page mechanism exists in this adapter's read path at all),
so a tail on this runtime can never be a held stream; it has to be a wake-up plus a
bounded re-read from a position the caller already holds, which is a poll with a
push trigger in front of it. The object genuinely *can* push — an alarm is the DO
half of the `LISTEN`/`NOTIFY` pair ES-32 names — so this adapter is squarely in the
clause's "loses nothing" column: a separate `TailingEventStore` sub-trait composes
over it without touching any existing adapter, while a required `subscribe` on
`EventStore` would have forced *this* adapter to implement it by re-reading behind
a name that says it does not, which is precisely the shape ES-32's *Rejects*
paragraph forbids. So the asymmetry argument survives contact with the first real
constrained runtime: adding the capability later is additive, removing it later is
not, and nothing observed here makes the 0.1 absence look like a mistake. **The
verdict is bounded, and the bound is named rather than hidden:** nobody has measured
what a DO alarm's wake-up actually costs — latency, and per-object price at N
objects — and it cannot be measured from this gate, whose runner is
`wasm-bindgen-test-runner` over a `node:sqlite`-backed shim with no alarms and no
isolate (`every-rule-under-workerd`'s escalated blocking finding). That measurement
is what would sharpen this paragraph from *cheap in shape* to *cheap in cost*; it
needs a `workerd`-class runner and it belongs post-0.1, outside this initiative.
Recorded, not acted on: no port method, no sub-trait, no feature flag and no alarm
plumbing lands with this verdict.

**2026-08-20 — the CF-14 re-read, in full.** *The verdict is the CF-14 row of* The 10
`[DEFERRED]` clauses *above; this is the detail behind it, held here so that cell keeps
the one-to-three-sentence budget its neighbours keep.* `CloudflareFixture` expressed
`REOPEN` through the same contract shape `MemoryFixture`, `DurableFixture` and
`SqliteFixture` use — a fresh binding derived off the Durable Object's retained `state`,
with nothing added to the `Fixture` trait to accommodate it — and
`acknowledged_writes_survive_a_reopen`, `reopened_store_does_not_reissue_an_event_id` and
`recorded_time_survives_a_reopen` all ran and passed with **zero** `SKIP` lines on that
row, so "durable" needed no grading and no second grade of the capability was invented.
**The limit is the runner, and it is named here rather than left to be discovered:** this
executed on `wasm32-unknown-unknown` under `wasm-bindgen-test-runner` over a
`node:sqlite`-backed shim and **not** `workerd`, so what was exercised is the capability's
own weaker operation — a re-derived binding, not an isolate restart, which still cannot be
performed from inside a test (`kb-open-question-workerd-runner-absent-001`). The far end is
untouched: this project supplied no store that can lose an acknowledged write to a *fault*
rather than to an instruction, so that half stays phase 8's and `sqlite-durable-store`'s
(HS-P0012), and `happenstance-neon` (HS-P0014) is the one named implementation still
unanswered. Nothing here moves CF-14's `[DEFERRED]` marker.

**2026-08-20 — the CF-27 re-read, in full.** *The verdict is the CF-27 row of the same
table; this is the argument behind it.* The intuitive hazard is the wrong one: a Durable
Object's storage **outlives** its isolate — `CloudflareFixture::reopen` re-derives a binding
off the retained `state` and every row is still there — so eviction is not how one of these
loses history. What is real is the verb: this adapter reaches storage as
`state.storage().sql()`, and the same `Storage` object carries `delete_all()`
(`worker-0.8.5/src/durable.rs:449`), so a wholesale purge happens with **no `EventStore`
method involved at all** and the store passes every conformance rule unchanged afterwards —
exactly the silence CF-27's *Rejects* paragraph describes, now with a named verb behind it.
The specification's **No, and nothing is planned** completeness row therefore understates the
exposure by one adapter, and this re-read is handed on rather than answered: the instrument
that would settle it — a testkit-adjacent store holding only a suffix of its own log and
reporting that it does — is deliberately **not** built here and stays
`retention-and-incomplete-logs`' (HS-P0018) and phase 14's. Nothing here moves CF-27's
`[DEFERRED]` marker.

---

## Phase 10 — `happenstance-postgres` and `happenstance-neon`

**Goal.** Two adapters that fill two empty far ends: a store that does not
serialise its writers, and a store with no connection, no interactive transaction
and no cursor.

**Why here.** Their design contribution was already spent in phases 2, 4 and 6 —
the skeletons falsified the position model, the capability table fed three ADRs,
and `Transaction<'static, Postgres>` confirmed the projection freeze. What is left
is two adapters, and an adapter is where frozen decisions get spent.

**Decisions it settles.** ADR-0024. Fills the position-allocation and transport
axes; supplies CF-13's far end.

**The one decision that is not a storage preference.** `nextval()` allocates
outside the transaction, so without deliberate machinery a Postgres store violates
the visibility invariant by construction, and a projection checkpointing on
`after` skips the events that appear below it: data loss, no error, no failing
test. Three strategies, each with a real cost:

- **`xid8` + `pg_snapshot_xmin(pg_current_snapshot())`** — an extra column and a
  `WHERE transaction_id < pg_snapshot_xmin(…)` clause. Cheap to write; the failure
  mode is that *any* long-running transaction anywhere in the database becomes a
  ceiling, so one forgotten `BEGIN` in an unrelated application stalls every reader
  and every projection.
- **Transaction-scoped advisory locks** — readers derive a safe watermark from
  `pg_locks`. No long-transaction stall, materially more machinery.
- **A serialised sequence table** — dissolves the problem, and throws away the
  write concurrency that was the reason to reach for Postgres at all.

Measure the third under contention and the first under a deliberately held
transaction. **The ADR owes a number, not a preference.**

**Work**

- [ ] **Claim `happenstance-postgres` and `happenstance-neon` on crates.io**, per phase 0's
      rule that a name is reserved when its phase starts, not before — the point
      being that by now there is a crate to justify it with.

- [ ] ADR-0024, with the visibility strategy chosen on numbers and phase 5's
      columns in migration 1.
- [ ] The Postgres event store and `PostgresProjectionStore` against phase 6's
      owned batch.
- [ ] All four macros, plus the phase-3 mutant harness re-run with
      `PostgresEventStore` as a control.
- [ ] `testcontainers` for the fixture, pinned to a specific Postgres minor, in
      **its own CI job**: the default gate must stay runnable without Docker, the
      same reasoning the repository already applies to Ladybug's C++ build.
- [ ] **`happenstance-neon`** finished far enough to run the suite over one-shot
      HTTP, including the capability skips it must report — no interactive
      transaction, no cursor, a 64 MB response ceiling. Where it cannot pass a rule,
      the rule's clause is what has to give, and that is a specification amendment,
      not an adapter workaround.
- [ ] A deployment note for Hyperdrive plus a `worker::Socket`-backed driver on
      the Worker side — a note, not a supported configuration: it needs a forked
      driver with unnamed-statement support and a hand-rolled binding.

**Proof artefact.** The concurrency macro green under a multi-thread runtime
against a store that **does not serialise its writers** — the first time any
adapter in the portfolio clears that bar — plus a committed number for the
visibility strategy's cost, including its behaviour with a long-running
transaction held open on the same database. And `happenstance-neon`'s capability
skip list, which is the transport axis's far end stated honestly.

**Exit criteria**

- [ ] Four macros green against a real Postgres.
- [ ] `PreCommitPositionStore`'s rule is one this adapter had to *work* to pass,
      and ADR-0024 says what that work cost.
- [ ] Every rule `happenstance-neon` cannot pass is either a reported capability
      skip or an amended clause — never a silent pass.
- [ ] The default `cargo xtask ci` path still runs without Docker.
- [ ] No `todo!()` on either path; `publish = false` removed.

**Cases this makes writable.** E2E-01 against a store that can genuinely fail it.

**Estimate.** 11 days.

**Session log**

---

## Phase 11 — Ladybug projection store

**Goal.** A third implementation of the projection port, against a transaction API
deliberately unlike SQLite's.

**Why here.** Its *structural* contribution was pulled into phase 2 as a skeleton
and consumed by phase 6's freeze, so what remains is the adapter, which blocks
nothing.

Projections only. This crate will not offer an event store: an event log needs a
monotonic append with a conditional write, and forcing that onto an engine built
for analytical traversal produces something that satisfies the trait and not the
specification.

**Decisions it settles.** ADR-0025. Fills the batch-shape axis.

**Work**

- [ ] **Claim `happenstance-ladybug` on crates.io**, per phase 0's
      rule that a name is reserved when its phase starts, not before — the point
      being that by now there is a crate to justify it with.

- [ ] ADR-0025: checkpoint placement — in the graph as a node or beside it, which
      hinges on what Ladybug's transaction API actually guarantees; how a projection
      expresses graph mutations (raw Cypher or a typed builder); and whether
      `lbug`'s synchronous API is wrapped in `spawn_blocking` or the adapter is
      offered as blocking-only.
- [ ] Add `lbug` and measure the cold build. It compiles C++ through `cxx` and
      `cmake`; a multi-minute native build is exactly why this is a separate crate
      and not a feature flag.
- [ ] Implement `SendProjectionStore`; run the projection suite.
- [ ] Give the crate its own CI job if the build cost is material, rather than
      slowing the three-OS gate everyone runs.

**Proof artefact.** The projection suite green on a third shape, and a written
answer to whether phase 6's freeze needed amending. If it did, that is a
superseding ADR and a data point that the two-implementation freeze rule should
have been three.

**Exit criteria**

- [ ] Projection conformance green, with capability skips reported.
- [ ] The verdict on phase 6's freeze is written down either way — "it held" is a
      result and must be recorded as one.
- [ ] Build cost measured and the CI decision recorded here.
- [ ] `publish = false` removed.

**Cases this makes writable.** The third-shape half of E2E-19 and E2E-24.

**Estimate.** 6 days.

**Session log**

---

## Phase 12 — Publish `0.2.0`

**Goal.** `happenstance-core`, `happenstance`, `happenstance-testkit`,
`happenstance-sqlite` and `happenstance-cloudflare` — **five crates**, decided at
the `0.2.0` release pass — on crates.io, rendering on docs.rs.

**Why here.** Publication no longer waits on replication: with identity settled in
phase 5 and `IngestStore` living in the sync crate, nothing in `happenstance-sync`
touches the contract's public surface.

**It does now wait on phase 10, and that is a decision rather than a discovery.**
The dependency row above reads `7, 8, 10`; it read `7, 8` until the `0.2.0`
release pass, and the phase that moved it is **F2-5**.

F2-5's actionable half was refuted by execution — `dropped_append_future_leaves_no_partial_batch`'s
second arm runs twice per run, in both feature configurations, and has since the
rule's first commit. What survives is that **the rule has never been answered by a
store with a real medium under it**: every store that has ever reached either arm
is an `Rc<RefCell<…>>` in the testkit's own test target, where *"the future was
dropped"* means a local went out of scope rather than a connection was severed.
The store that would answer it is phase 10's.

The repository owner chose to hold the release for that answer. It is worth
recording that this runs **against** the pre-publication review's own verdict on
this class — *"no fix here is a breaking change, and every one of them is
available at the same price after `0.2.0`"* — and against this file's own
sequencing argument, which put phase 10 off the trunk precisely because its
design contribution was already spent. The trade is roughly eleven days and two
adapters against publishing a conformance suite whose atomicity rule has only
ever been proved against a `RefCell`. Taken deliberately, with the counter-argument
in front of it.

**Two consequences follow, and neither is a cost.** The seventeen briefs ratified
at the release pass were ratified because their window closed at `0.2.0`;
deferring the release widens that window rather than closing it, so the
implementation queue they created can be worked unhurried. And the repository
stays private until the release, so the documents that now describe `0.2.0` in the
present tense are visible only to whoever picks this tree up — which is the one
group they can still mislead.

And what "why here" is *not*: it is not "publishing freezes the public API". The
API was frozen in phases 4 – 6, on evidence, which is what makes publishing safe.
Publishing starts the feedback loop; it does not create the obligation.

**Work**

Most of the old phase-7 list moved to phase 0, where it was cheaper. What remains
is the release.

- [ ] `CHANGELOG.md` finalised for 0.2.0 — it has been accumulating since phase 0.
- [ ] `cargo publish --dry-run` per crate; verify each `.crate` against phase 0's
      `--list` assertion.
- [ ] Publish in dependency order: `happenstance-core` → `happenstance-testkit` →
      `happenstance` → `happenstance-sqlite`. Each must be live before the next
      resolves against it.
- [ ] Set `clippy::todo` to `deny` with no per-crate exemptions in any published
      crate. A clean build with it denied is the proof that no stub survives.
- [ ] Tag; cut the GitHub release.
- [ ] ADR-0004 loses `provisional`; the MSRV becomes a promise.
- [ ] Repoint `cargo-semver-checks` to keep *both* baselines — the registry for
      release safety, `--baseline-rev` for review signal.
- [ ] README status table: no more 🔲 for what shipped, and the "batteries"
      tagline reconciled with what actually shipped.

**Proof artefact.** docs.rs green for every published crate under `--all-features`
*and* the `docsrs` cfg — phase 0's nightly job already proves the second, which is
the one that cannot be fixed after the fact because a crates.io release cannot be
edited — plus a `cargo-semver-checks` run on the next pull request that
demonstrably reports something.

**Exit criteria**

- [ ] Crates live; docs.rs builds green.
- [ ] `cargo-semver-checks` compares against a real published baseline, verified
      on the next pull request.
- [ ] Every `[PROVISIONAL]` clause published at 0.1 either has its falsifier
      scheduled in a later phase of this file, or is behind an unstable feature.
      Audit it against [the provisional
      ledger](#the-46-provisional-clauses) and `cargo xtask spec-trace`, not
      against prose — the previous revision carried this criterion with nothing to
      check it against.
- [ ] **Every `[DEFERRED]` clause on a published surface is resolved, made
      additively resolvable, or accepted in writing as a possible breaking 0.2.**
      One qualifies, and it is safe: **WF-1** defers DCB wire interoperability to
      phase 13, but the format is private and WF-8 puts a version first, so phase
      13 can change it without a wire break. Confirm that is still true — it stops
      being true the moment anything outside this workspace parses the format.
      (**ES-24** was the other. It is settled at phase 4 and no longer deferred;
      the criterion is kept because the next clause to land here will not be.)

**Cases this makes writable.** None. Publication writes no clause; it makes the
frozen ones cost something to change.

**Estimate.** 2 days.

**Session log**

---

## Phase 13 — `happenstance-sync` and its testkit

**Goal.** Replication between happenstance instances, expressed as a **port with
adapters** rather than a protocol with one peer.

**Why here.** The hard questions are internal to the crate, and the easy part is
already paid for by ADR-0003: a peer forwards opaque bytes, never needs the
sender's domain types, cannot fail to parse a payload it does not understand, and
cannot corrupt one by re-encoding it.

**Phase 12 is in the dependency row as ordering, not as a technical
prerequisite.** Nothing in this phase needs a crate to be on crates.io; what it
needs is phases 5, 8, 9 and 10. The 12 is there because the alternative — holding
0.1 for replication — is the sequencing this whole plan exists to reject, and a
dependency row is where that intent is enforced. If 12 slips, this phase is not
blocked by anything real, and a session that reaches here with 12 outstanding
should say so in the log rather than wait.

**Decisions it settles.** ADR-0026 (what a peer is), ADR-0027 (how logs
reconcile). Discharges SY-1 – SY-35 and WF-1's interop deferral.

**The two structural decisions, already settled by the specification.** The port
lives in `happenstance-sync`, not the contract crate — symmetry would argue for
putting it beside `EventStore`, and doing so would put replication back on the
publish path for no gain. And `SyncPeer` describes **one peer**; a `SyncRunner`
fans out. Multi-peer reconciliation, primary/secondary ordering and what to do when
two peers disagree are *policy*, and policy on the port makes every adapter author
inherit the merge problem and makes the conformance suite test a policy rather than
a transport.

**Ingest is unconditional, with compensation** (SY-1 – SY-7). A replicated event is
never refused for a reason that is a function of the receiving store's state.
Where a local condition would have been violated, the receiving side appends the
losing event **and** a domain-supplied compensating event as one atomic unit. The
rationale is convergence, not politeness: rejection is a function of local state,
so different peers reject different events and the union of facts is never reached;
a compensation is an append rather than a refusal, so no peer deletes a fact its
user was told had landed. The domain decides what a compensation means; the port
provides the atomicity and the identity that makes it idempotent.

**Work**

- [ ] **Claim `happenstance-sync` and `happenstance-sync-testkit` on crates.io**, per phase 0's
      rule that a name is reserved when its phase starts, not before — the point
      being that by now there is a crate to justify it with.

- [ ] ADR-0026. What ingest promises, what makes re-delivery harmless, and what
      the port may assume about a transport it cannot see. The two real peers are a
      Durable Object over a socket and a Postgres over one-shot HTTP with no
      interactive transaction; a `SyncPeer` that cannot be implemented by the second
      is a `SyncPeer` shaped like the first. Phase 2's sketch is the evidence.
- [ ] ADR-0027. The merge rule, the compensation contract, whether replication is
      whole-log or scoped (SY-27, SY-28 — a spoke holding a filtered subset cannot
      distinguish "not yet received" from "filtered out", so a position-based resume
      watermark against a hub is unsound), idempotent bulk ingest in bounded round
      trips (SY-14), and hub-and-spoke as a first-class topology beside
      peer-to-peer.
- [ ] `IngestStore` in `happenstance-sync` (VT-10) — the seam through which a
      foreign identity arrives, and the reason `EventStore::append` never grew a
      slot for one.
- [ ] `sync_peer_conformance!` in `happenstance-sync-testkit`, emitted through
      phase 1's registry so it inherits the tokio/blocking/wasm flavours. **The
      suite never decodes a payload** (SY-35) — a suite that parses `data` would
      certify a peer that does, and ADR-0003's guarantee is exactly that no peer
      needs to.
- [ ] `MemorySyncPeer` behind a `memory` feature — the oracle, the doctest target,
      and something an application author can test against before any real peer
      exists. The same three-part rationale `memory.rs:16-23` gives for
      `MemoryEventStore`, and the same cold-start problem the projection port had
      without one.
- [ ] Envelope types on phase 5's tested wire format, with the format version
      first.
- [ ] Ingest bound on `EventStore`, not `SendEventStore` — the Cloudflare side is
      single-threaded, and CLAUDE.md rule 4 binds the sync runner too because the
      `!Send` peer sits mid-chain rather than at a leaf.
- [ ] Two real peers — the phase-9 Durable Object and the phase-10 Postgres — plus
      the round trip between a native SQLite store and each.
- [ ] Record the DCB wire interop decision (WF-1) in ADR-0026's envelope section:
      named, deferred, with the experiment being a specific external implementation
      to interoperate with. Not silence.

**Proof artefact.** `sync_peer_conformance!` green against `MemorySyncPeer` and
**two structurally unlike networked peers** — a socket-reachable Durable Object and
a one-shot-HTTP Postgres — plus a round-trip test asserting the payload `Bytes` are
**byte-identical** end to end and that replaying the same batch twice changes
nothing. The byte-identity half is what lifts ADR-0003 from provisional; the
two-peer half is what makes this a port rather than a protocol.

**Exit criteria**

- [ ] ADR-0026 and ADR-0027 written before the code they constrain.
- [ ] `SyncPeer` implemented by three peers, one of which cannot hold a
      transaction open across a round trip.
- [ ] Hub-and-spoke and peer-to-peer are both expressible, and the crate's module
      doc no longer describes only one.
- [ ] `ingest_never_rejects` and `compensation_is_atomic_with_the_losing_event`
      green, with a mutant that fails each.
- [ ] The byte-identical round trip is green, and ADR-0003 loses `provisional`.
- [ ] Every `[DEFERRED]` `SY` clause is either settled or renewed against a named
      experiment; a renewal with no experiment is a build failure under CF-38.

**Cases this makes writable.** E2E-33 – E2E-42, E2E-45, and the idempotency half
of E2E-07 (ES-24).

**Estimate.** 12 days.

**Session log**

---

## Phase 14 — Retention, deletion and completeness

**Goal.** Decide what a store is permitted to forget and how it says so — or
refuse the whole area in writing, which is a legitimate answer that silence is not.

**Why here.** It is the last empty far end in the portfolio, and it is the one
where an accidental answer is most likely: four of the six scenarios reach the same
missing primitive from unrelated doors — a pruned slice, a purged cohort, a
compacted peer — and a store that has been deleted from is currently
indistinguishable from a young one at every value in §2.

**Decisions it settles.** ADR-0028. Discharges ES-39, CF-27, SY-32.

**Work**

- [ ] ADR-0028. Either a port surface by which a store reports the history it does
      not hold, or an explicit written refusal: deletion is out of scope for
      `EventStore`, and here is what a store that has been deleted from is permitted
      to look like. Both close E2E-46 and E2E-47; only one of them adds API.
- [ ] **The suffix store** (CF-27) — a testkit-adjacent store that deliberately
      holds only a suffix of its own log, so a runner or an ingest path written
      against it fails loudly rather than being accidentally correct. It is small,
      and it is the completeness axis's far end.
- [ ] `condition_over_removed_history_does_not_reject` (ES-40) and
      `positions_are_not_reused_after_removal` (ES-38): an append condition's
      meaning is scoped to the store that evaluates it, so a condition over
      destroyed history must refuse rather than pass.
- [ ] Retention coordinated across the peer set (SY-32): a retention gap is
      reported, not silent.
- [ ] Redaction (E2E-49): whether a tag can be redacted at all, given that
      `Tag` equality is byte equality and every index is keyed on it.

**Proof artefact.** The suffix store, committed, with a runner and an ingest path
that both fail against it — plus the rules that catch them. A store that lies about
its own completeness is the one wrong implementation nothing in the workspace can
currently detect.

**Exit criteria**

- [ ] ADR-0028 written, and it either adds a port surface or refuses the area in
      writing.
- [ ] The suffix store exists and at least two rules fail against it.
- [ ] ES-39, CF-27 and SY-32 are no longer `[DEFERRED]`.

**Cases this makes writable.** E2E-44, E2E-46, E2E-47, E2E-48, E2E-49.

**Estimate.** 5 days.

**Session log**

---

## Standing constraints

Carried forward from [CLAUDE.md](CLAUDE.md), restated here because phases 4
through 13 write the most new adapter code and are the likeliest to trip over them.
Names are the post-phase-0 ones.

1. **Never introduce `#[async_trait]`.** It injects `+ Send`, which makes the
   `wasm32` / Workers target impossible. Ports are defined once without a `Send`
   bound and `trait_variant` derives the `Send` flavour.
   ([ADR-0001](.kb/decisions/0001-async-port-flavours.md), ADR-0008)
2. **Never put `serde` in `happenstance-core`'s default features.** Payloads are
   opaque `Bytes`; the `serde` feature covers envelope types only.
   **This attaches to the ports crate, not to the string on the front of it** —
   after the rename, `happenstance` is the crate whose job *is* encoding and it
   depends on `serde` by design.
   ([ADR-0003](.kb/decisions/0003-opaque-payloads.md), [ADR-0006](.kb/decisions/0006-bare-name-to-the-typed-layer.md))
3. **`EventStore::read` returns the stream at the top level and is not `async`.**
   Nesting it inside a future silently drops `+ Send` from the stream on the `Send`
   flavour, defeating the entire two-trait design. The unit test that asserts this
   is currently vacuous — it asserts on a concrete type, where auto-trait leakage
   makes it pass regardless — and **phase 1 replaces it with a generic one**. Until
   then this constraint protects a test that cannot fail.
   ([ADR-0001](.kb/decisions/0001-async-port-flavours.md))
4. **Bind `EventStore`, not `SendEventStore`, in generic code.** It is the weaker
   requirement and accepts both flavours. Import only one of the two names per
   module — having both in scope makes method calls ambiguous. Prefer
   `happenstance_core::prelude` once phase 4 ships it.
5. **No let-chains.** Stable only from 1.88; the MSRV is 1.85 — but until first
   publish the MSRV is a *preference*, not a promise. Weigh it; do not obey it.
   ([ADR-0004](.kb/decisions/0004-edition-and-msrv.md))

And the rules that outrank the rest, with the two amendments this plan adds:

- **An adapter that has not run the conformance suite is not an adapter.** If a
  rule looks wrong, fix the rule and say why in the same change — do not skip it.
- **A conformance suite that has not been shown to fail a wrong adapter is not a
  suite.** That is what phase 3's mutant registry is for, and CLAUDE.md's own
  corollary — *a rule that no adapter can fail is decorative* — is what makes it
  binding rather than nice.
- **A port whose adapters all share a shape is a port shaped like that shape.**
  Before freezing a port, name the axis it is most likely to be wrong about and
  check that something in the workspace sits at the other end of it. The
  [portfolio table](#the-instrument-portfolio) is that check, and seven of its far
  ends are empty today.

Two smaller corrections to the record, so nobody re-derives them: `cargo hack` and
`cargo deny` both resolve on this machine, so those gate steps run rather than
printing `skipped`; and the sequential
`racing_conditional_appends_elect_one_winner` is **not** the rule a naive
read-then-write implementation fails — it is sequential by construction and says
so. Phase 3's concurrency macro is that rule.
