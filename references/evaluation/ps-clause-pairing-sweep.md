# The PS clause/rule pairing sweep

**Lens:** two `[FROZEN]` clauses in §4 are known to be paired with conformance
rules their own `MUST` does not support. Is that systematic across the `PS`
family, or two isolated incidents?
**Date:** 2026-08-13 · **Repo:** `D:\repos\happenstance`, working tree at
`2136dde` — the branch point of `initiative/from-contract-to-published-library`
· **Toolchain:** `rustc 1.97.1 (8bab26f4f 2026-07-14)`
· **Corpus:** `spec/SPECIFICATION.md`, 9,070 lines, §4 at `:4632-5680`

---

## What this is

The evidence behind phase 6's decision about how wide the frozen-clause repair
has to be. It is **immutable evidence** under
[this directory's rules](README.md): dated, pinned to the commit above, cited by
`file:line` from elsewhere, and **superseded rather than edited**.

It **decides nothing**. Seven clauses come out of it carrying a defect and none
of them is repaired here, because
[`repairing-a-frozen-clause-without-amending-it.md:108-119`](../../.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md)
records why discovery and decision were kept in separate passes: *"a pass that
both discovers and decides cannot be audited, because its findings and its
resolutions arrive in one artifact with no baseline to judge whether the decision
was forced by the evidence or convenient for closing the pass."* The repair is
`unstable-projection-gate-and-clause-disposition`'s; the ADR scope is
`projection-decision-atoms`'. Not one normative byte moved in the commit that
carries this file.

### Why it exists, given the gate already runs a checker

`cargo xtask spec-trace` runs on every gate and cannot answer this question. It
says so in its own header: *"It cannot tell whether a rule is the **right** rule
for a clause. That is judgement"* (`xtask/src/spec_trace.rs:17-18`). What it
proves is mechanical — that a named rule exists, that a `[PROVISIONAL]` marker
carries a falsifier, that §7.1 and §7.2 are derived rather than hand-maintained
(`:22-29`).

The phase 4/5 reconciliation established the method and the artefact shape but
stopped at six recorded gaps *"without sweeping the neighbourhood of any of
them"* (`.kb/reference/phase-4-5-specification-reconciliation-census.md:108-112`).
What is new here is a **census of soundness** — one row per clause, one verdict
per row, one stated test — over one clause family, and a verdict on whether
§4.11's table was populated from a systematic assumption that does not match the
clause text beside it.

### The instruction this carries out

Both open-question atoms end with the same sentence and neither had been acted
on:

> **Before deciding phase 6's answer for PS-1 specifically, check the other 35 PS
> clauses for the same shape**
> (`.kb/open-questions/ps-1-states-no-progress-obligation.md:77-81`)

> **Before deciding, check the other 35 PS clauses for the same shape.** … that
> pairing at a similarity score of 40, the closest of the six gaps in this batch,
> suggests §4.11's table may have been populated with a systematic assumption
> that does not match the clause text it sits beside
> (`.kb/open-questions/ps-19-scope-narrower-than-its-rule.md:69-75`)

That is the hypothesis under test, and it is named as such in
[the verdict](#the-verdict) below.

---

## Method

### Step 0 — name what the `MUST` binds

Applied before the classifier, because the classifier asks about
*implementations* and a clause only has implementations once you know what it
binds. Four subjects occur in the `PS` family:

| Subject | What an "implementation" is | Clauses |
|---|---|---|
| **A** — an adapter | a `ProjectionStore` impl | PS-1, PS-7, PS-8ᵈ, PS-11, PS-12, PS-15 – PS-19, PS-21 – PS-24 |
| **R** — a projection or a runner | code above the port | PS-10, PS-13, PS-14, PS-20, PS-25 – PS-30 |
| **D** — the port's definition, or this document | a candidate trait or a sentence | PS-4, PS-5, PS-6, PS-8, PS-9, PS-31, PS-32, PS-34 – PS-37 |
| **P** — the project's own process | a decision to freeze or to ship | PS-2, PS-3, PS-33 |

ᵈ PS-8's `MUST` binds **D** (the method must remain on the port); the rule
assigned to it tests **A**. That split is the whole of its finding.

### Step 1 — the classifier

Taken verbatim from `.kb/decisions/README.md:20-22` and restated in
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:56-72`:

> a correction to a `[FROZEN]` clause is a **repair** if the set of
> implementations the clause admits is unchanged; otherwise it is a **gap**, and
> a gap is a decision's.

Applied to a `(clause, rule)` pair it becomes one question with a yes/no answer:

> **Is there an implementation — of the subject Step 0 named — that satisfies the
> clause's `MUST` verbatim and fails that rule?**

`yes` → **`defective`** (the pairing is a gap, and a gap is a decision's).
`no` → **`sound`**.
cannot tell → **`undetermined`**, recorded as a gap finding, never rounded to
`sound` for tidiness nor to `defective` for narrative
(`repairing-a-frozen-clause-without-amending-it.md:56-72`: inability to tell *is*
a gap finding).

The vocabulary is closed at three. A fourth value — *"out of scope"*, *"N/A"* —
was considered and rejected: it is where the seven clauses with no assigned rule
would have gone, and their pairing is exactly what this sweep exists to judge.

### Step 2 — a rule whose subject is not the clause's

Common in this family, because §4 mixes port-shape clauses, adapter obligations,
runner obligations and process gates in one numbering. A rule assigned to a
clause whose `MUST` binds a *different* subject is a **borrowed instrument**: it
cannot falsify the clause, and failing it does not mean the clause was violated.

The discriminator is the clause's own honesty, and it is not a courtesy — §7.2
prints the rule in a column headed *"Conformance rule"*, so a reader takes it as
the clause's falsifier unless the clause says otherwise:

- borrow **stated in the clause** → `sound`. The specification does this often
  and well: PS-4 (*"the port's obligation is PS-1"*), PS-5 (*"a compile-time
  observation rather than a runtime rule"*), PS-6 (*"the signature; no runtime
  rule"*), PS-9 (*"the obligation it creates is PS-11, which is checkable"*),
  PS-11 (*"the probe is the mechanism, not a rule of its own"*), PS-20
  (*"no separate rule, because the property is only observable through a
  replay"*).
- borrow **not stated**, or contradicted by the clause → `defective`.

### Step 3 — strength, recorded on every `defective` row

A defect is only as interesting as the implementation that exposes it, and two
kinds of exposing implementation are not worth the same:

- **`independent`** — the exposing implementation satisfies **every other** `PS`
  clause's `MUST`, and the missing obligation is not another row's. The suite has
  no other grounds to reject it, so the misattribution is the only thing standing
  between a conformant store and a red gate.
- **`dependent`** — the exposing implementation violates another `PS` clause's
  `MUST` (named), or the defect is the *same* missing obligation another row
  already carries (named). Real, and worth recording, but it inflates a count
  without adding a cause.

This field is why the tally below is presented twice. Counting `defective` rows
without it would have crossed the systematic threshold on an artefact of the
classifier rather than on evidence.

### Step 4 — the exposing implementation

Every `defective` row names the shape of store, projection or runner that
satisfies the `MUST` and fails the rule, concretely enough to be built, and says
whether it is a **plausible first cut** or a **contrivance**. The bar is the one
the existing priors already clear — *"the natural implementation, not a
contrivance"*
(`.kb/open-questions/ps-19-scope-narrower-than-its-rule.md:41-48`) — and it is
`CLAUDE.md`'s conformance corollary one level up: *a finding no implementation
can exhibit is decorative in exactly the way a rule no adapter can fail is.* A row
for which no such implementation can be named is `undetermined`, never an
unsupported `defective`.

### The three attribution sources, and what is out of scope

A `PS` clause's rule attribution is stated in three places, and the **union** of
the pairs is swept, not the intersection:

1. the clause body's own `**Rule:**` field (e.g. `spec/SPECIFICATION.md:4735-4743`);
2. §4.11's rule → clause table, seventeen adapter-level rules (`:5653-5676`);
3. §7.2's clause → rule index (`:8628-8666`).

**Mechanical** disagreement between (1) and (3) is out of scope for repair:
`spec-trace` derives §7.1 and §7.2 from `parse_clauses` (`xtask/src/spec_trace.rs:22-29`),
so a mismatch there is a tooling bug with an existing owner. Three were seen and
are recorded as [tooling observations](#tooling-observations) rather than
repaired. The sweep's subject is the **semantic** disagreement none of the three
tables can detect.

### Scope

The `PS` family only — PS-1 through PS-37. `CF`, `ES` and `SY` are not swept. The
hypothesis under test is about §4.11's table specifically, and widening to other
families would turn a bounded first story into a specification-wide audit. Where
a `PS` finding suggests the same shape elsewhere it is recorded as an
observation, not as extra rows.

---

## The threshold, declared before the count

Stated here, above the census and the tally, because the ordering is the
integrity mechanism: a threshold written after the count is indistinguishable
from a threshold written to fit it. This section was fixed before clause PS-2 was
opened.

**The verdict is `systematic` if either arm fires:**

- **Arm (a) — rate.** Five or more clauses carry an **`independent`** defect of
  the **same shape**. Five is a little over one clause in seven of the family,
  and it is the point at which the cost comparison PS-19's atom asks for flips:
  *"a single pass fixing the table's assumption may be cheaper than two point
  fixes"* (`ps-19-scope-narrower-than-its-rule.md:74-75`). Below five, point
  fixes are cheaper and can each carry their own argument; at or above five, a
  per-clause repair is re-deriving one cause seven times.
- **Arm (b) — locus.** Three or more **`independent`** same-shape defects fall
  **inside §4.11's seventeen-rule table**, on rules that table introduced rather
  than rules the clause bodies name. Three is the smallest number that can
  distinguish "the table was populated from an assumption" from "two clauses were
  written the same way", because **two is the prior itself** and a hypothesis
  cannot be confirmed by the observation that raised it.

**Otherwise the verdict is `isolated`.**

**Shape is part of both arms.** Same-shape means the same mismatch mode. Three
modes occur in this family and they do not aggregate with one another:

| Shape | The mismatch |
|---|---|
| **S1 — the rule reaches past the `MUST`** | the clause's sentence is narrower than the rule assigned to it; a conformant implementation fails the rule. *This is PS-1's and PS-19's shape, and it is the hypothesis under test.* |
| **S2 — subject mismatch** | the rule is falsified by a different actor than the `MUST` binds, and the clause does not say so. |
| **S3 — inherited** | the rule presupposes an obligation stated by no clause's `MUST`, which another row already reports. |

A third defect of a *different* shape is weaker evidence of a systematic cause
than a third of the *same* shape, so the tally is broken out by shape as well as
by strength.

---

## The census

One row per clause, PS-1 through PS-37, no omissions and no *"not applicable"*
escapes. The seven clauses whose §7.2 rule cell reads `*(none — see clause)*` —
PS-9, PS-31, PS-32, PS-33, PS-35, PS-36, PS-37 — are **not** exempt: *"no rule
assigned"* is itself a pairing to verdict on, and a clause with no rule and no
falsifier is the defect `spec-trace`'s own header names as indistinguishable from
a decision nobody wanted to make (`xtask/src/spec_trace.rs:11-13`).

Columns: **Src** is which attribution source supplied the pair — `C` the clause
body's `**Rule:**` field, `T` §4.11's table, `X` §7.2's index. **Shape** is `—`
on a sound row. **Exposing implementation / why sound** carries the store,
projection or runner on a `defective` row and the reason on a `sound` one.

| Clause | Maturity | Rule(s) under test · Src | Verdict | Shape · strength | Exposing implementation / why sound | Owner |
|---|---|---|---|---|---|---|
| PS-1 | FROZEN | `commit_is_atomic_with_the_read_model`, `commit_advances_the_checkpoint`, `failed_commit_leaves_both_unchanged` · C T X | defective | S1 · independent | A store whose backing state lives per **handle** rather than per store: `connect()` mints a fresh map instead of a fresh handle onto a shared one. Nothing survives a handle, so `commit` returns `Ok`, both the row and the checkpoint are absent through the fresh handles `commit_is_atomic_with_the_read_model` reads with — the *"or not at all"* arm, satisfied verbatim — and `commit_advances_the_checkpoint` fails. **Plausible first cut**, and the workspace already knows it: it is the fixture bug `CLAUDE.md`'s *"one fixture instance is one isolated backing store; each `connect()` on it is one handle onto that store"* exists to forbid. Every other `PS` `MUST` is satisfiable inside one handle, so nothing else rejects it. | **new decision** — the progress obligation is outside all three ADR ranges (PS-1 is §4.1). Repair at `unstable-projection-gate-and-clause-disposition`. |
| PS-2 | FROZEN | `CheckpointOnlyStore` asserted to fail `commit_is_atomic_with_the_read_model`, plus the suite green against two adapters at opposite ends of the batch-shape axis · C X | sound | — | Subject **P**. Both conjuncts of the `MUST` are operationalised one-for-one by the named artefacts; the clause names process instruments rather than an adapter rule, and does so explicitly. | — |
| PS-3 | PROVISIONAL | `cargo hack --feature-powerset` in `cargo xtask ci` · C X | sound | — | Subject **P**. An implementation satisfying the SHOULD — shipping behind an off-by-default feature — passes the powerset check. The mismatch runs the *other* way and the clause states it: *"the exemption is a doc obligation, not an adapter obligation"*. See [inverse-shape observations](#inverse-shape-observations). | — |
| PS-4 | PROVISIONAL | `commit_is_atomic_with_the_read_model` run against a buffering adapter · C T X | sound | — | Subject **D**; borrowed instrument, **stated**: *"the port's obligation is PS-1, and PS-1 is satisfiable by opening the transaction inside `commit` around a buffered write set"*, and *"the rule is shape-blind by construction, which is the point"* (`spec/SPECIFICATION.md:4849-4867`). | — |
| PS-5 | PROVISIONAL | `MemoryProjectionStore` and one real adapter compiling without `where Self: 'a` · C X | sound | — | Subject **D**; borrow **stated**: *"a compile-time observation rather than a runtime rule. Stated here because it is what PS-34 is contingent on"* (`:4874-4878`). | — |
| PS-6 | PROVISIONAL | the signature; no runtime rule · C X | sound | — | Subject **D**; borrow **stated**: *"the signature; no runtime rule. Enforced by the compiler on every implementer"* (`:4889-4891`). The compiler is the falsifier and it cannot be evaded. | — |
| PS-7 | FROZEN | `dropped_batch_leaves_store_usable` · C T X | sound | — | Subject **A**. The rule's two assertions map one-for-one onto the `MUST`'s two conjuncts — *"the first row is absent"* onto "MUST roll back", *"the second commit succeeds"* onto "MUST leave the store usable" (`:4898-4910`). No implementation satisfies both conjuncts and fails either assertion. | — |
| PS-8 | FROZEN | `rollback_leaves_both_unchanged` · C T X | defective | S1 · dependent | The `MUST` binds **D** and is about the method's **existence** — *"`rollback` MUST remain on the port even though a buffered batch could be dropped"* (`:4911-4912`). The rule tests **A** and is about the method's **behaviour**. A write-through adapter — one whose `Batch` is a token and whose inherent write API, which PS-9 explicitly blesses, hits the read model immediately, deferring only the checkpoint — keeps `rollback` on the port, satisfying the `MUST` verbatim, and fails `rollback_leaves_both_unchanged` because the rows are already durable. **Plausible first cut** for a store with no transaction at all. **Dependent**: that store also violates PS-1 and PS-7, so no *independent* exposing implementation was found. The finding is one of **attribution** — *"rollback undoes"* is stated by no clause's `MUST` in §4, and the rule that enforces it hangs off a clause about the method's presence. | **ADR-0017** names it (PS-4 – PS-15); the repair is a **new decision**, PS-8 being `[FROZEN]`. |
| PS-9 | PROVISIONAL | *(none — see clause)* · C X | sound | — | Subject **D**. No-rule pairing, dispositioned in-clause and routed: *"none checks the absence of a bound. This is a port-shape clause; the obligation it creates is PS-11, which is checkable"* (`:4954-4956`). The `[PROVISIONAL]` marker carries a live falsifier (a second generic consumer), so `spec_trace.rs:11-13`'s defect does not apply. | — |
| PS-10 | FROZEN | `compile_fail` doctest showing `error[E0271]` · C X | sound | — | Subject **R** and compile-time. The rule is an exact falsifier for both conjuncts: a projection spanning two stores is what fails to compile. Observation: the doctest's home is the `Projection` trait, which does not exist in this workspace yet — a scheduling fact (HS-P0011's), not a pairing defect. | — |
| PS-11 | PROVISIONAL | `commit_is_atomic_with_the_read_model` and every rule downstream of it · C T X | sound | — | Subject **A**; borrowed instrument, **stated** in the clause's own `Rule:` field: *"the probe is the mechanism, not a rule of its own"* (`:4982-4983`). An adapter that implements the probe can of course fail an atomicity rule — but the clause has already said the rules are downstream consumers rather than falsifiers of its `MUST`. | — |
| PS-12 | PROVISIONAL | `batch_reads_reflect_pending_writes`, gated on `ProjectionProbe::READS_THROUGH_BATCH` · C T X | sound | — | Subject **A**. Arm one (reads reflect pending writes) passes the rule; arm two (no read path at all) declares the const `false` and takes a **reported skip**, which CF-18 requires and which is not a failure. No implementation satisfies the `MUST` and fails. The clause's third sentence has no falsifier and the clause says so itself — *"Declining by returning stale data is not detectable by anything"* (`:5071-5074`). See [inverse-shape observations](#inverse-shape-observations). | — |
| PS-13 | FROZEN | `rebuild_is_chunk_size_invariant` · C T X | defective | S2 · dependent | The `MUST` binds **R** — *"A projection MUST NOT read its own read model other than through the batch it is writing"* — and the rule runs in the **adapter** suite, where the projection is the testkit's own probe and is PS-13-conformant by construction. So the rule can never observe a PS-13 violation, and it *can* fail with a conformant projection: the exposing configuration is the testkit probe (conformant) against a write-behind adapter that declines `READS_THROUGH_BATCH`, which PS-4 and PS-12's second arm both bless. At chunk size 3 the second increment in a chunk cannot see the first's pending write, the read model diverges from the chunk-size-1 run, and the rule fails. **Plausible first cut**: it is the Ladybug shape §4.4 and PS-12 both name by hand. **Dependent**: that adapter violates PS-14. The clause asserts the converse of what holds — *"a projection that reads out-of-band fails it"* is true; "a failure means a projection read out of band" is not — and §7.2's index prints the rule as PS-13's falsifier. | **ADR-0017** names it (PS-4 – PS-15 — the probe's design is ADR-0017's); the repair is a **new decision**, PS-13 being `[FROZEN]`. |
| PS-14 | FROZEN | `rebuild_is_chunk_size_invariant` · C T X | sound | — | The `MUST` — *"A rebuild MUST produce the same read model at every chunk size"* — binds the (runner, projection, store) triple, and the rule falsifies exactly that. A rebuild that is invariant passes. See the [cross-clause finding](#cross-clause-findings) on what this costs a write-behind adapter; that is a clause-to-clause tension, not a pairing defect. | — |
| PS-15 | PROVISIONAL | `commit_rejects_a_foreign_batch` · C T X | sound | — | Subject **A**. An implementation satisfying the `MUST` rejects a foreign batch at `commit` and leaves both stores unchanged, which is what the rule asserts. The rule covers one of the three methods the `MUST` names; that is the rule being *weaker*, and it is an [inverse-shape observation](#inverse-shape-observations), not a defect under this classifier. | — |
| PS-16 | PROVISIONAL | `reset_clears_rows_and_checkpoint_together` plus its failure-injecting variant · C T X | sound | — | Subject **A**. *"As one unit of work"* entails both the both-gone assertion and the failure-injected both-unchanged assertion; no implementation satisfies the `MUST` and fails either. §7.2 additionally lists `probe_delete_all`, which is a probe method rather than a rule — a [tooling observation](#tooling-observations). | — |
| PS-17 | FROZEN | `reset_is_scoped_to_one_projection` · C T X | sound | — | Subject **A**. Two ids in one store, reset one, assert the other untouched: an exact falsifier for both sentences of the `MUST` (`:5188-5199`). | — |
| PS-18 | PROVISIONAL | `refused_reset_changes_nothing` · C T X | sound | — | Subject **A**. An adapter that *is able to* refuse, and whose refusal leaves both halves intact and is not reported as success, passes. An adapter with no protection policy declines the fixture capability the rule needs and takes a reported skip. Neither satisfies the `MUST` and fails. Observation: the rule as written names *"a testkit fixture store configured to protect one id"*, which in an adapter suite must mean the adapter's own fixture — a wording ambiguity for `reset-rules` and `projection-capability-skips` to close. | — |
| PS-19 | FROZEN | `reset_is_not_commit_at_first`, `fresh_projection_has_no_checkpoint` · C T X | defective | S1 · independent | Re-derived from `spec/SPECIFICATION.md:5218-5223` rather than from the prior. The `MUST` is scoped *"After a successful `reset`"*; `fresh_projection_has_no_checkpoint` asks about an id never seen — a different point in the lifecycle. A SQLite adapter whose `checkpoint` is `SELECT position, authority FROM checkpoints WHERE id = ?` and whose Rust resolves the missing row with `.unwrap_or(Checkpoint::Live { through: FIRST })` — the cheapest default, since `SequencePosition` is `NonZeroU64` and `FIRST` is its minimum — and whose `reset` writes an explicit `NeverRun` sentinel row, answers `NeverRun` post-reset (satisfying the `MUST`, and distinguishable from a commit at `FIRST` exactly as required) and `Live` for an unseen id. **Plausible first cut**. **Independent**: it satisfies every other `PS` `MUST`, including PS-22, since `FIRST` is the minimum position and nothing can regress below it. | **ADR-0018** names it (PS-16 – PS-20) and defers; the repair is a **new decision** at `unstable-projection-gate-and-clause-disposition`. |
| PS-20 | FROZEN | `reset_is_not_commit_at_first`, second half · C X | sound | — | Subject **R**; borrowed instrument, **stated**: *"covered by `reset_is_not_commit_at_first`'s second half; no separate rule, because the property is only observable through a replay"* (`:5254-5256`). The replay is the suite's own, so no adapter can make it violate the `MUST` — and the clause says it has no rule of its own. | — |
| PS-21 | FROZEN | `commit_accepts_a_position_the_batch_did_not_write` · C T X | defective | S3 · dependent | The `MUST` forbids validating the position against what the batch wrote; the rule additionally asserts *"the checkpoint advanced"*. A store that skips the checkpoint write when the batch is empty — a natural optimisation, and this rule commits an **empty** batch by design — validates nothing, satisfies the `MUST` verbatim, and fails the rule. **Plausible first cut**. **Dependent on PS-1**: this is the same missing progress obligation, surfacing under a second clause, and PS-1's per-handle store fails this rule for the same reason. Recorded because it shows the gap is not local to PS-1's own rule. | **new decision** — same repair as PS-1's; PS-21 is outside all three ADR ranges (§4.7). |
| PS-22 | PROVISIONAL | `commit_rejects_a_regressing_position` · C T X | defective | S3 · dependent | The `MUST` requires rejecting a position *"strictly below the current checkpoint"*. Under PS-1's per-handle store there is never a current checkpoint, so nothing is strictly below it and the `MUST` is vacuously satisfied — while the rule, which commits at *P* and then attempts *Q* < *P* expecting `CheckpointRegression`, fails. **Plausible first cut** (the same store). **Dependent on PS-1**. Cheaper to fix than the others: PS-22 is `[PROVISIONAL]`, so if the progress obligation lands elsewhere this row closes with it. | **new decision** — same repair as PS-1's. |
| PS-23 | PROVISIONAL | `distinct_projections_advance_independently` · C T X | sound | — | Subject **A**. A store satisfying *"One `commit` advances exactly one `ProjectionId`"* passes a rule that commits two ids and reads each back. Read literally, **this is the only `PS` `MUST` that entails progress at all** — see the [headline finding](#the-headline-finding-a-prior-is-partly-refuted). | — |
| PS-24 | PROVISIONAL | `rebuilding_is_distinguishable_from_live` · C T X | sound | — | Subject **A**. A store that distinguishes an authoritative read model from one being rebuilt reports `Rebuilding` after each rebuilding commit and `Live` after the live one, which is the rule verbatim. A store that never records authority does not distinguish and so violates the `MUST` first. | — |
| PS-25 | PROVISIONAL | `changed_query_starts_a_new_checkpoint` · C X | sound | — | Subject **R**. A runner deriving its `ProjectionId` from name + query digest passes. Observation: run at contract level the rule degenerates into an id-independence check already covered by PS-23, which is why the clause pins it as *"a typed-layer rule"* until `Query` has a canonical encoding (`:5372-5374`) — the limitation is stated. | — |
| PS-26 | FROZEN | `failure_policy_is_per_projection` · C T X | sound | — | Subject **R**; home **stated**: *"Integration-level; it needs a runner, so it belongs in the workspace e2e crate rather than the adapter suite"* (`:5403-5406`). A runner declaring policy per projection passes it. | — |
| PS-27 | PROVISIONAL | `skip_and_record_is_atomic` · C T X | sound | — | Subject **R**. A policy that offers skip-and-record and writes the record into the same batch that advances the checkpoint passes the crash-injected assertion, which is the `MUST`'s second conjunct restated. §7.2 additionally lists `on_error`, which is a projection method rather than a rule — a [tooling observation](#tooling-observations). | — |
| PS-28 | FROZEN | `pump_reports_the_failing_position` · C T X | undetermined | S1 (unresolved) · — | The `MUST` requires a failing `apply` to report the position it failed at and to carry an application error type distinct from the store's. The rule adds a second assertion — *"that `checkpoint` sits at the last good position"* — which the `MUST` does not state. Whether that is over-reach turns entirely on what *"the last good position"* means, and the rule does not say. Under *last successfully applied event*, every chunked runner fails it while satisfying the `MUST`, because a chunked runner's checkpoint sits at the previous **chunk** boundary after a mid-chunk failure — a `defective` pairing. Under *last successfully committed position*, the two coincide and the pairing is `sound`. Both readings are available from the text and the sweep will not pick one; the reasoning stalls there. **What resolves it:** defining the phrase when the rule is written. It is integration-level and its home is HS-P0011. | — (recorded; resolution belongs with the rule's author) |
| PS-29 | FROZEN | `one_poisoned_projection_does_not_stall_the_others` · C X | defective | S1 · independent | The `MUST` is *"One poisoned projection MUST NOT stall the others, and its terminal state MUST be observable through the API."* The rule asserts the other nineteen advance **and** *"that the supervisor reports the failure without being polled for it"* (`:5449-5453`). A supervisor exposing `fn failures(&self) -> Vec<Poisoned>` makes the terminal state observable through the API — satisfying the `MUST` verbatim — and requires polling, so it fails the rule. **Plausible first cut**: a query method is the obvious API, and it is what the clause's own `Rejects` field describes losing (*"the `JoinHandle` went into a set nobody drained"*). **Independent**: it violates no other `PS` `MUST`. Note the locus — this rule is **not** one of §4.11's seventeen; it exists only in the clause body. | **ADR-0019** names it (PS-26 – PS-30) and defers the observability design to HS-P0011; the clause repair is a **new decision**, PS-29 being `[FROZEN]`. |
| PS-30 | PROVISIONAL | `panicking_apply_rolls_back` · C T X | sound | — | Subject **R**. Given a PS-1-conformant store — one whose `commit` is atomic, which forces the batch to be buffered or transactional and its `rollback` to undo — a runner that rolls back after catching a panic leaves no partial rows and does not move the checkpoint, which is the rule verbatim. The store-side half of that is PS-8's finding, recorded there rather than counted twice here. | — |
| PS-31 | FROZEN | *(none — see clause)* · C X | sound | — | Subject **D**. No-rule pairing, dispositioned in-clause: *"none; a documented exclusion is not adapter-checkable. Stated as a clause rather than as prose because silence here is what produces the wrong implementation"* (`:5504-5507`). The second conjunct — *"the port MUST say so"* — is a documentation obligation with no assigned instrument; see [inverse-shape observations](#inverse-shape-observations). | — |
| PS-32 | FROZEN | *(none — see clause)* · C X | sound | — | Subject **D**. No-rule pairing, dispositioned in-clause with a named artefact: *"this is a correction to a document, and the artefact that proves it is the compiled pump recorded in PRESSURE-TEST §3.4"* (`:5518-5520`). Observation: the correction itself is not yet made, and ADR-0007 is an accepted, immutable atom — so "correcting" it is a superseding atom, never an edit. Routed to `projection-decision-atoms`, whose ADR-0017 already owes the sentence that ADR-0007's Context overstates itself. | — |
| PS-33 | DEFERRED | *(none — see clause)* · C X | sound | — | Subject **P**. No-rule pairing, dispositioned in-clause: *"it is a phase gate, not an adapter obligation, and no adapter can fail it. Recorded as a clause because `0007:119-121` sets the falsifier, no phase evaluates it, and an unevaluated falsifier is indistinguishable from none"* (`:5537-5541`). The ownership hole is the clause's own recorded finding and is HS-P0011's. | — |
| PS-34 | PROVISIONAL | a doctest on `ProjectionStore` implementing the port for a toy store · C X | sound | — | Subject **D**, and the `MUST` is **conditional** — *"If PS-5 is falsified and `Batch` keeps its lifetime…"*. The antecedent is false and about to be permanently false, so every implementation satisfies the `MUST` while nothing owes the doctest. The contingency is **stated**, in the maturity marker itself: *"contingent on PS-5; dead the moment `type Batch;` lands"* (`:5549-5550`). Observation: §7.2 prints the rule unconditionally beside a conditional `MUST`; re-marking it is project AC-014's, at `unstable-projection-gate-and-clause-disposition`. | — |
| PS-35 | FROZEN | *(none — see clause)* · C X | sound | — | Subject **D**. No-rule pairing, dispositioned in-clause — *"none at adapter level; the ADR is the artefact"* (`:5591-5592`) — and the artefact exists: ADR-0008 takes both ports together, and the clause cites it. Discharged, not open. | — |
| PS-36 | FROZEN | *(none — see clause)* · C X | sound | — | Subject **D**. No-rule pairing, dispositioned in-clause at length and honestly: *"none that gates, and the reason is a finding rather than an omission"* — the diagnostic carries `code: None`, rustdoc silently ignores a `compile_fail,E0308` annotation, so the annotation asserts nothing (`:5605-5615`). The deferred mechanism (`trybuild`) is named. See [inverse-shape observations](#inverse-shape-observations) for the documentation half. | — |
| PS-37 | FROZEN | *(none — see clause)* · C X | sound | — | Subject **D**. No-rule pairing, dispositioned in-clause with a named artefact: *"The obligation is on the contract crate, and the artefact that discharges it is a generic helper compiled against both flavours in the crate's own tests"* (`:5631-5634`). The obligation is vacuous until a provided method exists, and the clause says who will find it: *"whoever adds the first defaulted method"*. | — |

---

## The tally, against the threshold

**37 rows. 29 `sound`, 7 `defective`, 1 `undetermined`.**

Broken out the way [the threshold](#the-threshold-declared-before-the-count)
asks:

| | S1 — rule reaches past the `MUST` | S2 — subject mismatch | S3 — inherited | total |
|---|---|---|---|---|
| **independent** | PS-1, PS-19, **PS-29** | — | — | **3** |
| **dependent** | PS-8 | PS-13 | PS-21, PS-22 | 4 |
| **undetermined** | PS-28 | — | — | 1 |

Against arm (a): three `independent` same-shape defects. **Threshold is five.**
Not crossed.

Against arm (b): of those three, the rules involved are
`commit_advances_the_checkpoint` (PS-1) and `fresh_projection_has_no_checkpoint`
(PS-19), both introduced by §4.11's table — and
`one_poisoned_projection_does_not_stall_the_others` (PS-29), which is **not one
of §4.11's seventeen** and exists only in PS-29's own clause body. So the count
inside the table is **two — the prior itself**. **Threshold is three.** Not
crossed.

### The verdict

> **ISOLATED.** The hypothesis that §4.11's table was populated from a systematic
> assumption that does not match the clause text beside it is **not supported**.
> Inside that table the sweep found exactly the two defects that raised the
> hypothesis and no third.

Two qualifications, and both matter more than the headline:

**The shape recurs outside the table.** PS-29 carries precisely PS-1's and
PS-19's shape — a `[FROZEN]` sentence narrower than the rule written beside it —
on a rule §4.11 never touched. So the cause is not *"the table was populated
carelessly"*; it is **a habit of writing the rule to the clause's intent rather
than to its sentence**, and it is distributed across the family rather than
concentrated in one artefact. That is a smaller repair than a table-wide pass and
a *larger* warning: the next rule written for this port will do it again unless
the discipline is stated. Three clauses, three separate repairs, one lesson.

**The dependent rows are not noise.** Four of them, and three trace to one cause:
`rollback_leaves_both_unchanged`, `commit_accepts_a_position_the_batch_did_not_write`
and `commit_rejects_a_regressing_position` each enforce an obligation no clause's
`MUST` states. The PS-1 repair should therefore be scoped as *"which clause
states that a successful `commit` advances the checkpoint, and which rules rest on
it"* rather than as *"add a sentence to PS-1"* — the sentence is one clause's, the
consequence is four rules'.

**No re-plan is raised.** The escalation arm of
`_decomposition.md`'s Architecture brief Note 8 does not fire: phase 6's budget —
three ADRs and one clause-disposition story — is the right size for three point
repairs and one recorded lesson.

---

## The headline finding: a prior is partly refuted

Both priors reproduce **as pairing defects**. PS-1's and PS-19's rows above were
derived from the clause text at `spec/SPECIFICATION.md:4733-4759` and `:5218-5249`
before either atom was re-read, and both derivations stand — with concrete
exposing implementations the atoms did not name (a per-handle backing store; a
`SELECT`-plus-`unwrap_or` checkpoint resolution).

But one *supporting* sentence does not survive, and it is load-bearing for how the
repair is scoped. `.kb/open-questions/ps-1-states-no-progress-obligation.md:48-51`
says:

> That a successful commit *advances* anything — as opposed to merely being atomic
> about whatever it does — is stated by no clause's `MUST` anywhere in the
> document. PS-22 presupposes progress happens; §4.1a's prose asserts it, but
> non-normatively.

Read literally, **PS-23 states it**: *"One `commit` advances exactly one
`ProjectionId`"* (`spec/SPECIFICATION.md:5317-5318`). "Exactly one" excludes zero,
so a `commit` that advances nothing violates PS-23's `MUST`. The obligation is not
absent from the document — it is **misfiled**.

Three things follow, and none of them is a smaller finding than the atom's:

1. **It sits on the wrong clause.** PS-23 is about fan-out *scope* — its `Rejects`
   field names *"an adapter with a single-row checkpoint table"* (`:5327-5329`),
   which is the "not more than one" reading. Progress arrives there as a side
   effect of the word "exactly", almost certainly unintended.
2. **It sits on a `[PROVISIONAL]` clause**, falsified by *"a pair of read models
   in one store that must be mutually consistent at every observable instant"*.
   So the only normative statement that a commit makes progress is scheduled to be
   rewritten by a question about Norvant's control tower. An obligation three
   other rules rest on cannot live there.
3. **It changes the repair.** *"Add a progress sentence to PS-1"* is now one of at
   least three candidate repairs: state it on PS-1, mint a clause for it, or
   *split PS-23's two readings* and put progress where it is already half-written.
   The clause-disposition story inherits a choice, not a foregone edit.

The atom is **amended, not deleted**, and the amendment is staged for the ingest
wave rather than hand-written here — see [what is staged for `.kb/`](#what-is-staged-for-kb).

---

## Cross-clause findings

Not pairing defects — the classifier returned `sound` on both rows below — but
found while sweeping and recorded here because the next reader of either clause
needs them and there is nowhere else they would sit.

**1. A conformant write-behind adapter cannot pass `rebuild_is_chunk_size_invariant`.**
PS-4 blesses a `Batch` that is a buffer replayed at commit; PS-12's second arm
blesses an adapter that exposes **no** read path on the batch; and §4.11 then runs
`rebuild_is_chunk_size_invariant` against every adapter, ungated. The rule's probe
write is defined as *"an increment of what the batch can see"* (`:5090-5093`) — a
projection that reads what it writes. Against an adapter with no batch read path
that increment must fall back to committed state, so at chunk size 3 the three
increments inside a chunk collapse to one and the run diverges from the
chunk-size-1 run. The rule fails, and PS-14's `MUST` is genuinely violated — by
the *pairing* of a blessed adapter shape with a probe projection that needs the
capability the adapter legitimately declined.

Nothing here is wrong per clause. What is missing is the gate: PS-12's rule
carries `READS_THROUGH_BATCH` and this one does not. Either
`rebuild_is_chunk_size_invariant` needs the same gate, or the probe projection
needs a chunk-size-invariant definition that does not require read-your-writes.
**Routed to `projection-decision-atoms` (ADR-0017 owns the probe's design) and to
the suite stories that write the rule.**

**2. PS-15's rule covers one of the three methods its `MUST` names.**
The `MUST` binds `commit`, `reset` **and** `rollback` to reject a foreign batch;
`commit_rejects_a_foreign_batch` exercises `commit` only (`:5136-5143`). That is
the rule being weaker than the clause, which this classifier does not call a
defect — but two of three named methods have no falsifier, and PS-15 is exactly
the clause whose `Rejects` field says *"every adapter that can be written today,
all of which corrupt silently"*. **Routed to the suite stories.**

---

## Inverse-shape observations

The classifier asks one direction: does the rule reach **past** the `MUST`? Five
clauses have the opposite mismatch — the rule is **weaker** than the `MUST`, so
part of the sentence has no falsifier. That is a different defect with a different
owner (it produces unchecked obligations, not false failures), it is not the
hypothesis under test, and each is recorded rather than counted:

| Clause | The half with no falsifier | Stated in the clause? |
|---|---|---|
| **PS-3** | *"off-by-default"* and *"a documented exemption from semver"* — `cargo hack --feature-powerset` checks neither | yes (`:4780-4782`) |
| **PS-12** | *"MUST NOT expose a read path that answers from committed state"* — unreachable when the const is `false` | yes (`:5071-5074`) |
| **PS-15** | `reset` and `rollback` (see [cross-clause finding 2](#cross-clause-findings)) | no |
| **PS-31** | *"and the port MUST say so"* — a documentation obligation with no instrument | partly (`:5504-5507` covers the exclusion, not the saying) |
| **PS-36** | *"the port MUST document it rather than leaving it to be discovered"* — the clause explains at length why the doctest cannot gate, but nothing checks the documentation exists | yes (`:5605-5615`) |

The pattern across PS-3, PS-31 and PS-36 is one shape: **a documentation
obligation with no instrument.** This project already carries the instrument that
would close all three — the rustdoc obligations at
`standards/rust/70-rustdoc-obligations.md` and the design record's own
`## Visibility and stability` block. Recorded as a candidate open question, not
opened here.

---

## Tooling observations

Mechanical disagreements, reported and not repaired, because `spec-trace` derives
§7.1 and §7.2 from `parse_clauses` (`xtask/src/spec_trace.rs:22-29`) and a
mechanical mismatch there is a tooling bug with an existing owner. All three are
the same bug: the parser lifts an identifier out of the `**Rule:**` field's prose
that is not a conformance rule.

| Where | What §7.2 prints | What it is |
|---|---|---|
| **PS-10** | `compile_fail` † | the doctest *annotation*, not a rule name |
| **PS-16** | `probe_delete_all` † | a `ProjectionProbe` method the rule calls |
| **PS-27** | `on_error` † | a projection's own callback, named in the rule's prose |

None of the three changes a verdict: the real rule is present in the same cell and
was swept. Filed here so the next reader of §7.2 does not go looking for a rule
that was never proposed.

---

## What is staged for `.kb/`

Nothing under `.kb/` was hand-edited by the commit that carries this document.
`CLAUDE.md` (*Where the work lives*) is explicit that atoms are authored by
`/redkiln:kb-ingest` from `.kb/_intake/` and not by hand — hand-writing them
*"produces the directory layout of the process without the process"*, which is why
the first attempt was reverted at `0269720`. So the KB-side answers are staged as
one intake document for the next wave:

**`.kb/_intake/2026-08-13-ps-clause-pairing-sweep.md`**, which carries:

- the answer to `kb-open-question-ps-1-no-progress-obligation-001`'s
  **sub-question 3** (*check the other 35 for the same shape*) — isolated, with
  the qualification above — **and** the amendment to its
  *"stated by no clause's `MUST`"* sentence, which PS-23 refutes;
- the answer to `kb-open-question-ps-19-scope-narrower-001`'s **sub-question 2**
  (the same instruction, and the systematic hypothesis it raised) — not supported;
- the two bullets in `.kb/maps/open-questions-index.md` that change with them.

Until that wave runs, the answer is findable from `.kb/` only through this
document; both the intake file and this document's registration row say so.

---

## Citations

Every `file:line` above is the working tree's at `2136dde`, the commit named in
the header. A reader following one into a later tree should expect the **item**,
not the number — the subject text is quoted alongside every range for exactly that
reason, which is
`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`'s *verify the
referent, not the address*. This directory's own recorded failure mode is
citations that resolve, pass `spec-trace`, and point at the wrong line
(`README.md:50-55`).
