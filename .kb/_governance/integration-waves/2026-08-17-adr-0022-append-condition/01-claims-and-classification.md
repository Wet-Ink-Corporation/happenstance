# Wave `2026-08-17-adr-0022-append-condition` — claims and classification

Every claim the extract pass raised, labelled against the accepted decision corpus, with the
destination it was routed to. **Twenty-nine claims across six files, fourteen operations out** —
eleven atoms created, one open question resolved, one accepted decision flipped, one reference
atom extended.

## What "against the accepted decision corpus" means this wave

Twenty-three accepted decision atoms exist (`.kb/decisions/0001`–`0021`, `0029`, `0030`; every one
`status: accepted` except `kb-decision-0002`). The labels are the ones the previous four waves
used, with the same meanings:

| Label | Meaning here |
| --- | --- |
| `aligns` | Restates or applies a rule an accepted atom or a layer README already carries. Nothing new is committed; the claim cites rather than introduces. |
| `extends` | Net-new knowledge with no owner in `.kb/`, contradicting nothing accepted. |
| `conflicts` | Contradicts something accepted, or two sources contradict each other and the wave declines to pick a winner. **Two occurrences — `0031`-C1 and the ADR-0031 number collision.** |
| `requires-new-decision` | Cannot be discharged by recording it: an ADR or a human sign-off is needed. **Five occurrences, all in the defect log**, and none of them is authored here. |

**This wave authors no decision it did not receive, and declines five it was asked for.** ADR-0022
arrives with a 35 KB record and a five-minute experiment behind it; ADR-0031 arrives with a fired
falsifier and an executed test; ADR-0032 arrives with a defect verified against `HEAD`; ADR-0033
arrives with a published 29-row classification. Those four are transcribed. The defect log's five
"wanted from the wave: a decision record" requests are **not** — they become three open questions
(C1 and C2 merged), because writing them would be the wave inventing four ADRs in the same pass it
was handed four.

## The claim table

### `.kb/_intake/0033-adr-0022-append-condition-strategy.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C1` | ADR-0022 in full: one `SELECT max(position)` per guard inside `BEGIN IMMEDIATE` (23/213/972/42,399 µs against two alternatives on the rejection path, a tie on the accepted path and under contention); `event_tag(tag, position)` `WITHOUT ROWID` with `event_type` covering (10.7 ms vs 34.0 blob / 49.8 JSON1, paid for with a 1.5–2.1× write); the single-tag fast path (1,093 → 556 µs) making `tag_cardinality` and most-selective-first **requirements**; three fixed pragmas (`WAL`, `NORMAL`, 5,000 ms) absorbing 64-way contention with zero `SQLITE_BUSY`; a `tokio` `Handle` captured at construction so `NoRuntime` keeps a meaning; `index_arms()` rejected with its re-open trigger named; `rusqlite` without a pool ratified rather than decided | `must`-equivalent throughout — *"requirements rather than tuning"*, *"is rejected"*, *"fixed"* | `extends` | **Op 3** — `kb-decision-0022` |
| `C2` | Why the measurement is a separate atom: `decisions/README.md` keeps evidence out of decisions so a decision can be superseded without invalidating the numbers; `kb-reference-position-visibility-experiment-001` is the precedent | restates a layer contract | `aligns` | **No op.** Realised as the Op 1 / Op 3 split. A README is not a merge target — `00`, *Scoring method* |
| `C3` | Four prohibitions: do not fold the measurement in; do not edit `kb-decision-0012`; do not edit `kb-open-question-cf-40-…`; do not mint a clause | four `must not` | `aligns` | **No op.** All four are obeyed and none is new: three are `.kb/decisions/README.md`'s immutability rule applied, and the fourth is a statement about `spec/` this wave does not touch |
| `C4` | ADR-0012's falsifier item 1 requires *"two builds of the same SQLite adapter differing only in `append`'s ownership, measured on the same harness"*; three candidate stores in an experiment crate are not that, and **no story in this project's map produces it** — the four implementation stories after ADR-0022 build one adapter. A phase-8 obligation phase 8 as planned does not discharge | `requires`, and *"needs a queue row of its own or an explicit deferral with a new owner"* | `extends` | **Op 2** — a new `open_question`. ADR-0012 owns ES-17 and is immutable; the queue row is `.bklg/`'s and is carried in `unresolved` |
| `C5` | The handoff: mint two atoms, index the decision in `decision-map.md`, clear both files | procedural | `aligns` | **No op of its own.** `mapsImpact.decisionMap` on Op 3; the clearing is the ingest's |

### `.kb/_intake/0034-append-condition-experiment-2026-08.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `0034-C1` | The instrument: three strategies × three tag storages against real SQLite 3.53.2 under WAL / `NORMAL` / 5,000 ms, `rusqlite` 0.40 bundled, `--release` on a named host; all five arms cleared `event_store_conformance!` (445 tests, 89 rules) **before** measurement; two positive controls fired — the runner refuses to emit a number under `synchronous = OFF`, and the journal mode is read back rather than trusted. Lives in `experiments/append-condition/`, outside the workspace and outside the gate | none — figures, not commitments | `extends` | **Op 1** — `kb-reference-append-condition-experiment-001` |
| `0034-C2` | What the instrument cannot do (two harness figures noise-dominated, up to 45% between runs, one arm's unconditional append varying 4×; nothing measured the `tokio` seam) and three transferable lessons: measure arms round-robin in one process on a shared host; subtract a baseline and check the subtraction resolves; measure the path where the arms structurally differ | none | `extends` | **Op 1**, same atom. **Not a playbook**: the lessons are stated as findings of this experiment under its own conditions, and a playbook is a procedure someone follows. `02`, Adjudication 5 |
| `0034-C3` | The new atom needs a `domain-map.md` entry mirroring the position-visibility row, and reciprocal `related` wiring to `kb-decision-0022` and `kb-decision-0012` | corpus maintenance | `aligns` | **No op of its own.** `mapsImpact.domainMap` on Op 1; the reciprocal edge is the Maps phase's |

### `.kb/_intake/0032-adr-0031-the-runner-collapses-upward.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C1` | The ingest must write a new atom rather than edit `kb-decision-0007`; the long-form record is not immutable and may be edited directly | `must` ×1, `may` ×2 | `aligns` | **No op.** `.kb/decisions/README.md:9-13` and `kb-governance-referent-not-reasoning-001:50-59` already say it |
| `C2` | ADR-0007's falsifier fired: phase 7 exited, `happenstance-core` publishes exactly two module-level free functions (`store.rs:285`, `:321`), neither a pump, and **no pump function exists in the contract crate at all** — it was allocated by an ADR and never written. Held by an executed test at `crates/happenstance/tests/projection_clauses.rs` | evidentiary | `extends` | **Op 4**, `## Context`. The count was re-verified against this worktree |
| `C3` | **One runner, in `happenstance`.** `happenstance-core` keeps the `ProjectionStore` port and its transactional invariant and nothing that runs; `happenstance::run_projection` reads the checkpoint, derives the query, streams the replay, decodes through `Codec`, applies in chunks, and hands each chunk's write set and last applied position to the port's single `commit` | declarative commitment — *"is the only runner"* | `extends` / **partial supersession** | **Op 4** — `kb-decision-0031`, `depends_on: [kb-decision-0007]`, `supersedes: null`. `02`, Adjudication 2 |
| `C4` | What survives untouched: the discriminator (encoding, not orchestration) and all three shape decisions — `Query` nominates events, `Projection::Store` is an associated type, checkpoints stay per `(store, ProjectionId)`. Only the pump allocation is superseded | scope limit | `aligns` | **Op 4**, `## What this decision is not`. This clause is *why* Op 4 is partial, and it is what keeps `kb-decision-0007` at `status: accepted` |
| `C5` | ADR-0007's stated cost for this alternative — *"it puts the checkpoint invariant in a crate that sits above the port that states it"* — is still the true reason, now mitigated by something that did not exist when ADR-0007 was written: the projection conformance suite drives the port through `ProjectionProbe` and can fail a store that commits the two halves apart | none | `aligns` | **Op 4**, `## Alternatives rejected`; `related` to `kb-decision-0017` and `kb-decision-0010` |
| `C6` | A saving: the old pump typed its callback's error as the projection store's, so a decode failure had no representable home (E2E-26). The collapsed runner has none, because `CodecError` is concrete and `ProjectionError` carries a decode arm with the failing position | none | `extends` | **Op 4**, `## Consequences` |
| `C7` | PS-32's correction: a callback-driven pump **can** be written against the port as it stands (compiled, `PRESSURE-TEST §3.4`); what could not be written was the conformance suite. The superseding atom should carry it, and the long-form record should be amended in the same wave | `should` ×2 | `extends` | **Op 4**, `## Context`, **and Op 5** — the open question this answers. `02`, Adjudication 3 |
| `C8` | Three prohibitions: do not edit `kb-decision-0007`; do not widen the supersession; do not delete PS-32 or PS-33 from `spec/SPECIFICATION.md` | three `do not` | `aligns` | **No op.** Obeyed. The third names a `spec/` file this wave does not touch |
| `C9` | Provenance: HS-S0027, from the tree left by HS-S0026; counts re-derivable and held by an executed test | none | `aligns` | **Op 4**, `source_paths` |

### `.kb/_intake/0031-adr-0021-serde-attribution-correction.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C1` | The defect: `kb-decision-0021:141-144`'s third rejection ground — *"also barred by ADR-0003, which forbids `serde` in `happenstance-core`'s default features"* — **inverts** the constraint it cites. ADR-0003 positively assigns encoding to `happenstance` (`0003-opaque-payloads.md:17-18`) and constrains `happenstance-core` only; the framing region is `happenstance`'s, one crate above the port. The outcome survives: the other two grounds carry the rejection alone | descriptive, but the defect is in a `[FROZEN]`-adjacent accepted body | **`conflicts`** | **Ops 6 and 7.** The one body-level defect the wave finds |
| `C2` | The op is a supersession plus a metadata flip, not an edit: mint the new atom with `supersedes: [kb-decision-0021]`; flip `kb-decision-0021` to `status: superseded`, `superseded_by: <new id>`, body verbatim. Classified a **repair** on `README.md:20-24`'s mechanical test — the admitted implementation set is unchanged | `must`, in terms | `aligns` | **Ops 6 and 7.** The proposed number `0031` is **overridden** — `02`, Adjudication 1 |
| `C3` | What the superseding atom must say: restate the rejection on its two sound grounds only (codec-feature independence; no added dependency, `--no-default-features` + `wasm32`); **drop** the ADR-0003 attribution rather than correcting it; state the boundary in the right direction once; carry `phase: 7`, `reversibility: low` | four `must` | `extends` | **Op 6**, body and frontmatter |
| `C4` | Maps to re-sync: `decision-map.md:138`'s ADR-0021 row → `superseded`, a new row for the superseding ADR, the partial-supersession-chain section gains this lineage; `domain-map.md:189-194`'s typed-layer bullet re-points | mechanical | `aligns` | **No op of its own.** `mapsImpact.decisionMap` + `domainMap` on Ops 6 and 7 |
| `C5` | The long-form rule-3 tightening is **already applied**, line-count-neutral by appending an appendix so a dozen `:NNN` citations stay valid; the file grew 454 → 499 | past tense, explicitly needs no op | `aligns` | **No op. Verified**: 499 lines, *"Which instrument covers rule 3"* at `:458` |
| `C6` | A reviewer's proposed citation widening (`projection.rs:152-154` → `:152-155`) is **wrong** and must not be applied; `:155` is the function signature | `must not`, effecting no action | `aligns` | **No op. Verified**: `:152-154` is the quoted prose exactly, `:155` is `pub fn new`. Recorded so it is not re-raised |

### `.kb/_intake/contract-defect-log-phase-7.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C1` | No infallible `QueryItem` constructor for pre-validated inputs: every derived query carries a `Result` unreachable for a well-formed model (`query.rs:48-62`). Stronger than first recorded — `Boundary` is sealed, so the error arm is **untestable from outside the crate**. Bears on VT-18 `[FROZEN]` | *"wanted: a decision record"* | **`requires-new-decision`** | **Op 8**, merged with C2 |
| `C2` | `DomainEvent::tags` is infallible over a fallible `Tags`: `fn tags(&self) -> Tags` is total while every route into `Tags` is fallible, so an implementor with runtime tag strings has no total path but a validated newtype — 81 lines in the worked example (`main.rs:102-182`). *"D-1's other face"* | *"wanted: a decision record"* | **`requires-new-decision`** | **Op 8**, merged with C1. `00`, CL-4 |
| `C3` | CF-36 `[FROZEN]` claims `spec-trace` cross-references each case's level marker; `grep -c "Level" xtask/src/spec_trace.rs` returns **0**. A green `spec-trace` reads as evidence for CF-36 and is not | *"both are decisions; neither is a patch"* | **`requires-new-decision`** | **Op 11** |
| `C4` | No `PS` rule name is resolved — check 4 short-circuits (`:695-697`) — with PS-27 and PS-30 the visible symptom; and §7.2's `†` duplicates the clause's own maturity marker while naming no owner, where the clause does. Bears on CF-38 `[FROZEN]` | *"wanted: a decision record"* | **`requires-new-decision`**, with its stated mechanism corrected | **Op 12.** The short-circuit is real but its cause is `schedules_new`, not `has_suite` — `00`, provenance finding 3 |
| `C5` | `&mut P` makes N projections cost N reads, at the API level. Recorded as a **shape**, not a wrong answer — the runner's defence (an owned write set cannot be shared; one failure policy for all projections would be wrong) is sound for the alpha. Bears on the `[PROVISIONAL]` projection-port family | none; asks for evidence attachment, not a decision | `extends` | **Op 13.** The requested destination — an open question owning the tail seam — does not exist; `kb-open-question-projection-batch-no-apply-001` was superseded 2026-08-13 |
| `N1` | `run_projection` cannot be spawned without a caller-side bound — **not a defect**. ES-6 leaves `Error` unbounded on purpose and ADR-0009 assigns the obligation to the caller in terms; what was missing is now on `run_projection`'s own page (`runner.rs:297-310`). *"At most an amendment to ADR-0009's atom … never a new decision"* | explicit `never` | `aligns` | **Op 14** — the register, not ADR-0009, which takes no amendment |
| `N2` | `read_through` is dead code in eight `wasm32` feature combinations (`projection_memory.rs:233`); CI's `-D warnings` makes it a failure there. No clause ID, therefore `support`. *"The hand-off is owed as a `redkiln new` invocation"* | none | — | **No atom.** `unresolved` — a task, which `open-questions/README.md:37-38` refuses |
| `N3` | A private module can shadow a glob-re-exported one — `mod projection;` shadowed `happenstance_core::projection` through `pub use happenstance_core::*`, warning only `hidden_glob_reexports`. `_design.md`'s anti-pattern 14 is about *type* names. *"Proposed destination: `standards/rust/`, not `.kb/`"* | none | — | **No atom.** `unresolved` — the intake names a destination outside this corpus |

### `.kb/_intake/happenstance-macros-verdict.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `claim-1` | **`happenstance-macros` is out of scope for 0.1.** AC-013's criterion is mechanical (ceremony > domain ⇒ in; exactly 1.0 is out); measured against `examples/course-subscriptions/src/main.rs` at `78a2170`, the ratio is **0.50 : 1** at worst and **0.12 : 1** at best. Both readings say out | a scope commitment, `proposed_status: accepted` | `extends` | **Op 10** — `kb-decision-0033` |
| `claim-2` | Do not edit ADR-0020's atom; whether this is a new atom or a supersession is the wave's | `must not` | `aligns` | **No op**, and the adjudication is made: **new atom, not supersession**. `02`, Adjudication 7 |
| `claim-3` | The classification: 29 published contiguous line ranges exhausting 532/532; ceremony 40, domain 249, neither 158, contested 85; checked **plain** — no `macro_rules!`, `impl DomainEvent` hand-written — not the design's doctest | none | `extends` | **Op 9** — a `reference` atom |
| `claim-4` | `_design.md:1104-1111` predicted **in** at 2.4 : 1 over its own doctest; the example measures 0.50 : 1, about 5× the other way. The durable reconciliation: the `DomainEvent` impl is near-fixed (26 lines for two variants, 40 for three) while domain grows with concerns, refusals and handlers, so the ceremony ratio is a function of how much domain an artefact holds. Both numbers are true; a first-program page is still held to 2.4 : 1 (AC-U01), a scope decision is not taken on it (AC-013) | `should` / `should not` | `extends` | **Op 10**, `## Context`, citing ADR-0020's own prediction. **Not a separate atom** — it is the reason the verdict is worth recording |
| `claim-5` | 40/532 lines (7.5%) is what a derive would have bought, against a fourth published crate and a proc-macro in every consumer's build graph. **Reopen if and only if** defect C2 is settled with an infallible `Tags` path — which reaches the contested 85, not just the 40, and even an 85-line swing does not cross 1.0 from 0.50 : 1, so it stays post-0.1 | conditional trigger | `extends` | **Op 10**, `## Consequences`, with a `related` edge to **Op 8**. `00`, CL-3 |
| `claim-6` | Consequences carried out: no `crates/happenstance-macros/` created (an *out* verdict escalates nothing, `_decomposition.md:428`); `RUNBOOK.md:525`'s row moves off `open`; `publish-0-2-0-alpha-1` is unblocked because it depends on the record, never on a crate | none | `aligns` | **Op 10**, `## Consequences` |

## Conflicts

**Two, and they are different kinds.**

| # | Claim | Why it is a conflict, and what the wave does with it |
| --- | --- | --- |
| 1 | `0031`-C1 — the ADR-0003 attribution | An accepted decision atom's **body** states a binding constraint backwards, and the inversion is verifiable against another accepted atom (`0003-opaque-payloads.md:17-18`) and against `CLAUDE.md`'s binding constraint 2, which warns about this exact misreading after ADR-0006's rename. An M3 implementer reading it would conclude `serde` is barred from the crate whose entire job is encoding. **Resolved by supersession** — Ops 6 and 7 — because the corpus's own route for a defective accepted body is a new atom plus a metadata flip, and because the intake supplies the corrected text rather than leaving the wave to invent it. |
| 2 | The ADR-0031 number | Two intake files claim the same ADR number for unrelated decisions, and neither knows about the other. Not a knowledge conflict — an allocation one, and the kind that produces a silent overwrite rather than a visible contradiction. **Resolved against the backlog**, which allocates ADR-0031 to the runner collapse by filename at `_implementation.md:704`. `02`, Adjudication 1. |

Three near-misses, all of which read as conflicts on first pass and are not:

| # | Near-miss | Why it is not a conflict |
| --- | --- | --- |
| 1 | Defect C4 says no `PS` rule name is resolved; `kb-reference-spec-trace-has-suite-001` says `PS` was added to `has_suite` and the flip is held by a test | Both are true, and they are two terms of one guard. `has_suite("PS-1")` is `true`; the loop at `spec_trace.rs:699` skips on `c.schedules_new || !has_suite(&c.id)`, and a bare `†` in the clause's `Rule:` line sets `schedules_new` (`:1630-1634`). The reference atom's own text anticipates it — *"the seventeen `†` marks §7.2 printed against the family were an accurate statement rather than a checked one."* Op 12 `extends` it and carries the corrected mechanism |
| 2 | The macros verdict measures 0.50 : 1 where `kb-decision-0020` predicted 2.4 : 1 and *"in"* | A falsifiable prediction being falsified is the prediction working, and ADR-0020 says so itself: *"a consequence stated here, not a second decision; the record itself asserts no `must` about the derive."* Nothing accepted is contradicted, so `extends` — and `create_new`, not `supersede` |
| 3 | ADR-0031 reverses half of ADR-0007, which `kb-decision-0030` `depends_on` | ADR-0030 rests on ADR-0007's *checkpoint-per-`(store, ProjectionId)`* shape decision, which C4 keeps untouched. The half that moves — where the pump lives — is the half ADR-0030 never used, because the pump was never written |

## Counts

| | |
| --- | --- |
| intake files | **6** (the seventh, `README.md`, is dropped at the approval gate) |
| claims classified | **29** |
| `aligns` | 15 |
| `extends` | 12 |
| `conflicts` | **2** |
| `requires-new-decision` | **5** (defect log C1–C4; C1/C2 merged into one destination). **None authored** |
| accepted decision atoms **edited** | **0** |
| accepted decision atoms flipped (frontmatter only, body verbatim) | **1** — `kb-decision-0021` |
| decision atoms authored | **4** — ADR-0022, ADR-0031, ADR-0032, ADR-0033; all transcribed from records or verdicts already taken |
| `open_question` atoms created | **4** |
| `open_question` atoms resolved (flip + dated section, body verbatim) | **1** — PS-32, carried unperformed by three previous waves |
| `reference` atoms created | **3** |
| `reference` atoms extended | **1** — the compiled-findings register |
| map atoms inheriting work | **3**, via `mapsImpact` rather than ops |
| atoms out | **11 new + 3 mutated**, across 14 operations |
| cross-file clusters collapsed | **4**, two of which would have produced duplicate or colliding atoms |
| claims producing no operation, deliberately | **11** |
| claims carried to `unresolved` | **3** |
