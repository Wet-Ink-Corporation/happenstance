# Wave `2026-08-13-projection-adrs` — claims and classification

Every claim the extract pass raised, labelled against the accepted decision corpus, with the
destination it was routed to. **Thirty claims in, six operations out** — three atoms created and
three existing `open_question` atoms amended.

## What "against the accepted decision corpus" means this wave

Unlike both previous waves, there **is** an accepted decision corpus: seventeen atoms,
`.kb/decisions/0001`–`0016` and `0029`, every one of them `status: accepted` except
`kb-decision-0002`. So the labels finally do the work they were defined for.

| Label | Meaning here |
| --- | --- |
| `aligns` | Restates or applies a rule an accepted atom or a layer README already carries. Nothing new is committed; the atom cites rather than introduces. |
| `extends` | Net-new knowledge with no owner in `.kb/`, contradicting nothing accepted. The wave's default. |
| `conflicts` | Contradicts something accepted, or two incoming sources contradict each other and the wave declines to pick a winner. **Zero occurrences.** |
| `requires-new-decision` | Cannot be discharged by recording it: an ADR or a human sign-off is needed. **Zero routed to `open-questions/` this wave** — every such claim already has an owner, and `02` Adjudication 3 says why that is a finding rather than a gap. |

**This wave authors no decision.** Three ADRs were written, argued and accepted on 2026-08-13,
each with a long-form record in `references/adr/`; the wave transcribes them into atoms, exactly
as wave 2 transcribed seventeen. The one act that *would* be authoring a decision — repairing a
`[FROZEN]` clause, or picking a winner among the five named pairing defects — is declined in all
five cases, by the source documents first and by this plan second.

## The claim table

### `.kb/_intake/2026-08-13-adr-0017-projection-batch.md` — ADR-0017

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `c0` | Wave id, one wave not three, drop `_intake/README.md` at the gate | none — orchestration | — | **No atom** (CL-4). Applied: this wave is `2026-08-13-projection-adrs` |
| `c1` | Create one `decision` atom; the atom is the ~100-line canonical form, transcripts stay in the long-form record and are cited by `file:line` | ADR-grade | `extends` | **Op 1** — `kb-decision-0017` |
| `c2` | `status` is a closed enum; fold the provisional qualification **and its falsifier** into `summary`'s first clause; do **not** act on `kb-open-question-adr-status-vocabulary-001` | negative constraint | `aligns` | Standing choice 2 in `02`. **No edit** to the status-vocabulary atom |
| `c3` | `type Batch;` — owned, no lifetime parameter (PS-5). Rests on `E0195` and the `DefId::expect_local` ICE, **not** on the `Send` argument, which `live_handle.rs:14-31` refutes | commitment | `extends` | Op 1, §Decision |
| `c4` | An owned batch does **not** close the foreign-batch hazard; a lifetime names a region, not an instance. PS-15 stays `[PROVISIONAL]`, discharged at run time by a per-store-instance stamp surfacing as `CommitError::ForeignBatch` | commitment + marker | `extends` | Op 1, §Provisional |
| `c5` | PS-9 + PS-11 are one split-by-consumer decision: **no universal write vocabulary on `Batch`**; `ProjectionProbe: ProjectionStore` in the **contract crate** behind `feature = "conformance"`, bare flavour only, because the orphan rule rejects a testkit trait from an adapter's `tests/` | MUST-NOT-shaped | `extends` | Op 1, §Decision. Answers sub-question 1 of `kb-open-question-projection-batch-no-apply-001` |
| `c6` | Dropping a batch rolls back **and** leaves the store usable (PS-7); `rollback` stays on the port (PS-8), because Rust has no async `Drop` | commitment | `extends` | Op 1, §Decision |
| `c7` | `LiveHandleProjectionStore` moves to `experiments/live-handle-projection-batch/`, not deleted — the only compiled evidence against this decision's own §1 | disposition | `extends` | Op 1, §Consequences. **Directory does not exist yet** — carried in `unresolved` |
| `c8` | Seven rejected alternatives, each with its reason (the `put`/`get` supertrait contra ADR-0003; the probe in the testkit; keeping the GAT; resting PS-5 on `Send`; a generative brand; the receiver-lifetime tie; deleting the store) | authoring MUST | `extends` | Op 1, §Alternatives rejected |
| `c9` | Three provisional halves, each with falsifier and owning phase (PS-9/PS-11 by a second generic consumer at HS-P0011; PS-15 by a zero-cost type-level instance name; PS-4/PS-5 by an adapter needing a handle before the first write, LadybugDB named) | falsifier convention | `extends` | Op 1, `summary` first clause + §Provisional |
| `c10` | The sweep returned **isolated**; PS-8 and PS-13 are `defective` rows inside PS-4–PS-15; both `[FROZEN]`, both corrections widen the admitted set, so both are **gaps**. The atom names them and defers; it repairs nothing | scope limit | `extends` | Op 1, §What this decision does not repair (CL-3) |
| `c11` | Resolve — do not delete — `kb-open-question-projection-batch-no-apply-001`: status to `withdrawn`/`superseded`, reciprocal `related`, **body verbatim**, dated annotation; sub-questions 1, 2, 4 fall, sub-question 3 does **not** and the omission must be stated | metadata flip + constraint | `extends` | **Op 4** |
| `c12` | Map rows: one decision-map row; the open-questions bullet **stays listed** and gains an annotation; a domain-map row only if the Maps phase places one | map maintenance | `aligns` | `mapsImpact` on Ops 1 and 4; exact text in `02` §What the Maps phase inherits |
| `c13` | Not proposed: no `crates/**`, no `spec/SPECIFICATION.md`, no marker moved, no `[FROZEN]` line edited; **no resolution** of seven named open questions; **no supersession of ADR-0007**, though PS-32's correction to its Context is owed and recorded | negative scope | — | **No atom** (CL-5). The ADR-0007 correction is carried in `unresolved` |

### `.kb/_intake/2026-08-13-adr-0018-reset.md` — ADR-0018

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C1` | Create one `decision` atom, `adr_id: ADR-0018`, phase 6, provisional qualification + falsifier in `summary`'s first clause. Four claims at **three strengths**, not levelled | ADR-grade | `extends` | **Op 2** — `kb-decision-0018` |
| `C1a` | `reset(batch, id)` applies the batch and returns the checkpoint to `NeverRun` **as one unit of work** (PS-16, `[PROVISIONAL]`). Models the two-connection runbook incident that truncated, died, re-applied sixty-one events into an empty table and **reported healthy**. Falsifier: an adapter whose clearing cannot ride the same batch | MUST-shaped atomicity | `extends` | Op 2, §Decision + §Provisional |
| `C1b` | Scope is one `(store, ProjectionId)` pair (PS-17, `[FROZEN]`) — **settled by ADR-0007, cited and not re-derived**; a `reset` truncating the checkpoint table is the rejected alternative | citation discipline | **`aligns`** | Op 2, `depends_on: kb-decision-0007`. No second derivation is written |
| `C1c` | Refusal is a port **mechanism** (`ResetError::Refused`), policy stays in the domain (PS-18, `[PROVISIONAL]`). A refusal **MUST** leave both halves unchanged and **MUST NOT** be reported as success. Falsifier: no adapter ever implements protection | three modals | `extends` | Op 2, §Decision + §Provisional |
| `C1d` | `Checkpoint` is a three-variant enum — `NeverRun` / `Live { through }` / `Rebuilding { through }` — settled by being made; `(Option<SequencePosition>, bool)` loses because it can spell `(None, true)` | type-shape commitment | `extends` | Op 2, §Decision (the strongest of the four halves) |
| `C1e` | **Not decided:** whether `ResetError::Refused` carries the store's stated reason. Belongs to the design record (`projection-api-design-record`, AC-007) | explicit non-decision | `extends` | Op 2, §What this does not decide. **No open question** — it has an owner in `.bklg/` |
| `C2` | PS-19's `MUST` is scoped after a successful reset; §4.11 also assigns it `fresh_projection_has_no_checkpoint`. Widening changes the admitted set ⇒ **gap**, landing at `unstable-projection-gate-and-clause-disposition`. The atom states this and repairs nothing | gap-naming | `extends` | Op 2 §does-not-repair **and** **Op 5** (CL-1) |
| `C3` | A boundary-scoped projection checkpoint surfaced and was **filed, not absorbed** — it would reopen ADR-0013's globally frozen invariant; `kb-open-question-global-vs-boundary-visibility-001` owns it and is left unchanged | deliberate deferral | `aligns` | Op 2 `related` edge only. **No edit** to the question |
| `C4` | Decision-map row: ADR-0018, atom link, title, status, phase 6, supersession `—` | map maintenance | `aligns` | `mapsImpact.decisionMap` on Op 2 |
| `C5` | Open-questions index: no bullet removed; the PS-19 bullet **may** gain an annotation naming ADR-0018 | optional | `aligns` | Folded into Op 5's annotation, which the sweep independently requires |
| `C6` | Not proposed: no `crates/**`, no `spec/`, no marker moved, no `[FROZEN]` line edited, no supersession of ADR-0007 or ADR-0013 | negative scope | — | **No atom** (CL-5) |

### `.kb/_intake/2026-08-13-adr-0019-apply-failure.md` — ADR-0019

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `c1` | Create one `decision` atom. **The port grows nothing for apply failure** — no method, no associated type, no error variant, no feature. Three supporting facts: the skip primitive already exists as `begin()` + `commit(batch, id, poison_position, Live)` and nothing records that it happened; `rollback` **must** survive the port change (PS-30 / `AssertUnwindSafe`); isolation already works structurally per `(store, ProjectionId)` (ADR-0007). The failure policy is **per projection, not per runner** (PS-26, `[FROZEN]`), rejecting a runner-level `on_error: SkipPolicy` | MUST-NOT architecture commitment | `extends` | **Op 3** — `kb-decision-0019` |
| `c2` | Deferred **by name**, not designed: `PumpError<E::Error, P::Error, A>` with its `Apply { position, error }` variant; the `SkipAndRecord` vocabulary and what "record" means; the supervisor's report-without-being-polled half. All three to HS-P0011 / ADR-0020 / ADR-0021 | scope boundary | `extends` | Op 3, §Deferred by name (a table **inside** the atom) |
| `c3` | PS-29 is `defective` and one of the sweep's three `independent` defects: the `MUST` says "observable through the API", the rule demands a supervisor report *without being polled*. `[FROZEN]` ⇒ **gap**, owned by `unstable-projection-gate-and-clause-disposition` (a **project**, verified, not an atom). Names it and defers | gap-naming | `extends` | Op 3, §does-not-repair |
| `c4` | PS-28 is `undetermined` and stays so: "the last good position" is undefined — under *last applied event* every chunked runner fails while satisfying the `MUST`; under *last committed position* the pairing is sound. Resolution is HS-P0011's. **Cross-cutting:** the defect traces to a rule written to the clause's *intent* rather than its *sentence* — a habit, not a table artefact | non-resolution | `extends` | Op 3, §does-not-repair; the cross-cutting half to **Op 6** (CL-2), which Op 3 links rather than restating |
| `c5` | Decision-map row under a new `## 2026-08-13 …` wave section | map maintenance | `aligns` | `mapsImpact.decisionMap` on Op 3 |
| `c6` | Open-questions index unchanged **on this document's account** | explicit non-action | `aligns` | Honoured. The index does change this wave, on Ops 4, 5 and 6's account |
| `c7` | Not proposed: no `crates/**`, no `spec/`, no marker moved, no `[FROZEN]` line edited, no runner error type / policy enum / supervisor designed | negative scope | — | **No atom** (CL-5) |
| — | Provisional halves: PS-27 falsified if "record" turns out to mean "log a warning"; PS-30 falsified if the fan-out runner is not built, which is a benchmark the workspace has no harness for | falsifier convention | `extends` | Op 3, `summary` first clause + §Provisional |

### `.kb/_intake/2026-08-13-ps-clause-pairing-sweep.md` — the sweep

Its own stated bias: *"every claim below is an AMEND to an atom that already exists. No new atom
is proposed… If the wave's adjudicator finds itself minting a `decision` atom from this document,
it has misread it."* The plan honours that exactly: this file produces **zero** new atoms and
contributes to two amendments.

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `1a` | Sub-question 3 of the PS-1 question is answered **isolated**: 37 clauses, 29 `sound`, 7 `defective`, 1 `undetermined`; only PS-1 and PS-19 involve §4.11's table, so the systematic-population hypothesis is **not supported**. Two qualifications: the shape recurs outside the table on PS-29 (a habit); three further rules (PS-8, PS-21, PS-22) rest on PS-1's missing obligation, so the repair is *"which clause states progress, and which rules rest on it"* | advisory "should" | `extends` | **Op 6**; the habit half is CL-2 with `0019-c4` |
| `1b` | One sentence in the atom's *What is true today* is **refuted**: PS-23's *"One `commit` advances exactly one `ProjectionId`"* (`spec/SPECIFICATION.md:5317-5318`) does state progress — "exactly one" excludes zero. Misfiled twice: on a clause about fan-out **scope**, and on a `[PROVISIONAL]` clause. Does **not** overturn the PS-1 finding, which reproduces independently from `:4733-4759`; it changes the candidate repairs. **Appended dated amendment note, not an edit to the paragraph** | factual refutation | `extends` | **Op 6** — the wave's only correction to an existing atom's stated fact, and it lands as an append |
| `1c` | Sub-questions 1, 2 and 4 stay open; owner stays phase 6; the repair is `unstable-projection-gate-and-clause-disposition`'s | non-change | `aligns` | Op 6, in the same annotation |
| `1d` | `related` gains an edge to `kb-reference-ps-clause-pairing-sweep-001` **if and only if** the wave mints a reference atom for the sweep. Not proposed; precedent cited (`phase-4-5-reconciliation.md` was citable by path before its atom existed) | conditional | — | **Declined** — `02` Adjudication 4. No edge, no atom; carried in `unresolved` so a human can overrule |
| `2a` | Sub-question 2 of the PS-19 question is answered: a single table-wide pass is **not** cheaper, because there is nothing table-wide to fix. Three point repairs (PS-1, PS-19, PS-29) plus one recorded lesson is the right shape | advisory | `extends` | **Op 5** |
| `2b` | The PS-19 finding **reproduces** with a sharper exposing implementation re-derived from `:5218-5223` (`unwrap_or(Checkpoint::Live { through: FIRST })` against `reset`'s explicit `NeverRun` sentinel), satisfying every other PS `MUST` including PS-22. The atom is **confirmed, not corrected** | confirmation | `aligns` | Op 5 |
| `2c` | Sub-questions 1 and 3 stay open; owner unchanged | non-change | `aligns` | Op 5, same annotation |
| `3` | Amend `kb-map-open-questions-index-001`: both bullets stay **Open** and stay listed; each gains one annotating sentence in the existing *"Amended 2026-08-10: …"* style, citing the sweep by path | map maintenance | `aligns` | `mapsImpact.openQuestionIndex` on Ops 5 and 6; proposed text quoted verbatim in `02` |
| `4` | Six explicit non-proposals, including **no** decision atom, **no** `[FROZEN]` change, **no** resolution of either question, **no** amendment to the two adjacent traps, **no** reference atom, and **no** new open question — with two candidates named and declined: (a) PS-3/PS-31/PS-36 carry a documentation obligation with no instrument → routed to rustdoc obligations; (b) `rebuild_is_chunk_size_invariant` is ungated while `batch_reads_reflect_pending_writes` carries `READS_THROUGH_BATCH` → routed to ADR-0017 and the suite stories | declinations | — | **No atom.** Both named candidates are recorded in `02` Adjudication 5 and carried in `unresolved`, so declining them stays visible |

## Conflicts

**None.** For the first time in three waves the corpus is populated enough for the label to have
teeth, and every incoming claim either extends it or cites it. The three near-misses are worth
naming, because each looked like a conflict at first read:

| # | Near-miss | Why it is not a conflict |
| --- | --- | --- |
| 1 | ADR-0017 removes `ProjectionStore::Batch`'s GAT, which `kb-decision-0008` describes at length | 0008 records the GAT as a measured cost and states *"phase 6 must decide whether the GAT survives"*. ADR-0017 is phase 6 deciding. `depends_on`, not `supersedes` — `00`, "The GAT is not a conflict" |
| 2 | ADR-0018's boundary-scoped checkpoint would reopen ADR-0013's global visibility invariant | It was **filed, not absorbed**, before it became a claim. Nothing incoming asserts it; `kb-open-question-global-vs-boundary-visibility-001` already owns it and is untouched |
| 3 | The sweep refutes a sentence in an existing atom's body | The atom is an `open_question` (`authority_tier: note`), not a decision, and the refutation lands as an **appended dated note** per that layer's README. The finding the sentence supported reproduces independently, so nothing that rests on the atom moves |

## Counts

| | |
| --- | --- |
| claims classified | **30** |
| `aligns` | 12 |
| `extends` | 18 |
| `conflicts` | **0** |
| `requires-new-decision` | **0** |
| accepted decision atoms **edited** | **0** |
| accepted decision atoms superseded (frontmatter flip) | **0** |
| decision atoms authored | **3** — transcribed from `references/adr/`, none newly decided |
| open_question atoms created | **0** |
| existing atoms amended | **3**, all `open_question` |
| atoms out | **3 new + 3 amended**, across 6 operations |
