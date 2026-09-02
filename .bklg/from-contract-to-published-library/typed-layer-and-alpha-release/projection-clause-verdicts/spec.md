---
item: HS-S0027
stage: spec
created: 2026-08-12T13:46:24.399Z
updated: 2026-08-12T13:46:24.399Z
template_sig: 87bbf1d0
rendered_sig: "79e51406"
---

# Spec — PS-33, PS-27, PS-30 settled and PS-18 excluded, on the record

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` |
| This spec | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/projection-clause-verdicts/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` — *Architecture brief → Changing a clause marker is a three-part edit* (`:616-642`), *Tensions carried forward* (`:658-712`); *Testing brief*, AC-007/AC-008 rows (`:785-786`) |
| Signed-off design | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` — `## Items` binds `happenstance::Projection` to PS-33 (`:320-325`) and `happenstance::run_projection` to PS-27/PS-30 (`:326-331`); `## The states the API must express` (`:951-956`) |
| Story map row | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md:58` (M5 `projection-runner`) |
| Roadmap pointer | `RUNBOOK.md:4064-4088` — phase 7's exit criteria; `RUNBOOK.md:408-413` — PS-33 evaluated here **and nowhere else** |

## One-line PR slice

Write the verdicts the runner's existence makes answerable — PS-33 discharged by naming
an independent caller of the checkpoint pump or by the superseding ADR that collapses it
upward, PS-27 and PS-30 settled by counting with their integration tests, PS-18 recorded
as a documented exclusion (`RUNBOOK.md:4070-4077`; its subject does not exist in the
tree) — as the three-part `spec/SPECIFICATION.md` edit: generated §7.1/§7.2, the
hand-written §1.3 census, and IDs retained on exit (`spec/SPECIFICATION.md:209-212`),
with `cargo xtask spec-trace` green.

## Executive summary

**What this PR lands.** Four provisional/deferred markers in `spec/SPECIFICATION.md` §4
stop being promises to look later and become recorded verdicts, each backed by a count
over the tree the slice-mate story just changed: **PS-33** (`:5529`), **PS-27**
(`:5411`), **PS-30** (`:5462`) and **PS-18** (`:5200`). In the same edit **PS-32, PS-33
and PS-35 leave the clause space** with their IDs retained, the way CF-30 already is
(`:209-212`), and §1.3's hand-written census is re-derived by hand to match. Where a
verdict names a caller rather than an exclusion, an *executed* integration test under
`crates/happenstance/tests/` is what the count is over — a verdict whose only evidence is
the sentence asserting it is the failure mode this story exists to prevent.

**Pointer, not a restatement.** The project charter already says why these four are
owned here (`project.md`, AC-007/AC-008, DR-06) and the runbook already says the
evaluation happens at this phase's exit and nowhere else (`RUNBOOK.md:408-413`). The
delta this spec adds is the *mechanics and the traps*: which of the three edited regions
is generated and which is hand-written, what a valid verdict looks like per clause, why
`cargo xtask spec-trace` currently cannot catch the thing this story is guarding
against, and what this story is forbidden from fixing on the way past.

**The bar.** The sibling `projection-trait-and-runner` (HS-S0026) lands the code. This
story lands the *judgement about the code*, and it is delivered mounted — in the
specification the gate reads, behind the `specification traceability` REQUIRED step at
`xtask/src/main.rs:315`, not in a companion note.

## Context pack

Everything below is a decision this story must honor. Read it before opening anything.

### 1. A verdict is a count, and the escape hatch is named

The runbook's exit criterion for these clauses is one sentence and it is the whole
method: *"name the callers, or promote the clause to a documented exclusion"*
(`RUNBOOK.md:4070-4077`). The reason it is a *decision* rather than an observation is
also stated there — *"no adapter ever implements it" is not something that happens, it
is something you decide to conclude*. §4.1a's question 4 is the same point from the
specification's side, and calls this group **"the one at risk of never being asked"**
(`spec/SPECIFICATION.md:4812`, `:4822-4826`).

So: for each clause, either name the artefact that calls it — by path, and by a test
that runs — or record the exclusion with the reason the count is unavailable. A third
outcome ("still provisional, re-evaluate later") is not admissible here; it is precisely
what `RUNBOOK.md:4073-4077` calls *"the failure mode §1.3 forbids, arriving by patience
instead of by intent"*.

### 2. PS-33: the thing whose callers are being counted does not exist as a function

ADR-0007 split the runner at the decode boundary and set its own falsifier: *"if the
checkpoint pump has acquired no independent caller by the time the typed layer's phase
exits, collapse it upward and supersede this ADR"*
(`references/adr/0007-projection-runner-decodes.md:118-121`; the atom states the same
risk at `.kb/decisions/0007-projection-runner-decodes.md:73-75`).

**Read the tree before writing the verdict.** At HEAD
`crates/happenstance-core/src/projection.rs` holds `ProjectionId`, the `ProjectionStore`
port (`checkpoint` `:110`, `begin` `:117`, `commit` `:126`, `rollback` `:138`) and
**nothing that runs**. There is no pump *function* in the contract crate to have
callers. So the count is not a `grep` — it is a judgement about where the checkpoint
invariant lives, and the architecture brief already fixed the two admissible outcomes
(`_decomposition.md:422`):

- **name the independent caller** — some caller of the core-side checkpoint pump other
  than `happenstance`'s `run_projection`; or
- **write the superseding ADR** that collapses the pump upward into `happenstance`,
  which is ADR-0007's own narrowly-rejected alternative, already written down at
  `references/adr/0007-projection-runner-decodes.md` and restated in PS-33's *Rejects*
  paragraph.

The second branch is a **staging** act, never a hand-write: an ADR is staged into
`.kb/_intake/` and ingested by the human-invoked `/redkiln:kb-ingest`
(`CLAUDE.md`, *Where the work lives*; `_decomposition.md:644-652`). This story does not
run the ingest and does not touch `.kb/decisions/` — an accepted decision atom is
immutable and hand-writing one produces the directory layout of the process without the
process.

### 3. PS-27 and PS-30: counted against the runner the slice-mate lands, not against intent

- **PS-27** (`spec/SPECIFICATION.md:5411-5429`) is falsified *"if no projection ever
  writes a skip record, i.e. if 'record' turns out to mean 'log a warning'"*. Note what
  it rejects: the skip *primitive* already exists — `begin()` then
  `commit(batch, id, poison_position, Live)` applies nothing and advances the checkpoint
  atomically — and what does not exist is any way to record that it happened.
- **PS-30** (`:5462-5478`) is falsified *"if the fan-out runner is not built"*, which
  the clause itself makes contingent on a benchmark. That benchmark is the slice-mate
  `polling-cost-measurement`'s deliverable, not this story's, and it is deliberately
  outside the gate (CF-34).

**The design constrains, and does not settle, both counts.** `_design.md`'s
*States* section says a panic in `apply` rolls the batch back (PS-30) and a skipped
event is recorded in the same transaction as the checkpoint (PS-27) (`:951-955`), while
the signed-off signature of `run_projection` (`_design.md:563-570`) carries **no failure
policy parameter and no `on_error` hook**. Whether the alpha's runner therefore *has* a
skip-and-record path is a fact about what HS-S0026 builds, and the verdict is written
against what is in the tree at the end of this slice — not against the states table and
not against the signature read hopefully. If the path is absent, the exclusion branch is
the honest verdict and the clause says so.

**Where the tests live.** §4.11 lists `skip_and_record_is_atomic` and
`panicking_apply_rolls_back` among six rules that are *"integration-level and belong in
the workspace e2e crate rather than the adapter suite"* (`spec/SPECIFICATION.md:5695-5704`).
The load-bearing half of that sentence is the second half — CF-36 (`:8297-8298`,
`[FROZEN]`) forbids a clause backed only by integration-level cases from naming an
adapter conformance rule, and E2E-28 is marked `**Level:** integration`
(`spec/E2E-CASES.md:725-729`). The first half names a crate that does not exist: no
workspace e2e crate is in the tree, `_design.md` warranted none, and creating a
workspace member is a scope change for the runbook, not a story. **Decision: the tests
land in `crates/happenstance/tests/`**, which is the testing brief's own first candidate
(`_decomposition.md:786`) and satisfies CF-36's normative half — *not the adapter
suite* — without inventing a member.

### 4. PS-18 is not evaluable here; plan for the exclusion and be pleasantly surprised

PS-18's own clause text assigns evaluation to *"the exit of the projection-port
phase"* and asks *"whether the SQLite adapter implemented it"*
(`spec/SPECIFICATION.md:5200-5205`). Two facts close the counting branch at planning
time (`_decomposition.md:660-673`): neither `projection-store-freeze` (HS-P0010) nor
`sqlite-durable-store` (HS-P0012) has shipped an adapter, and **PS-18's subject does not
exist** — there is no `reset` method and no `ResetError` anywhere in
`crates/happenstance-core/src/projection.rs`. The verdict is therefore a **documented
exclusion naming HS-P0010 as the owner of the count**, and the exclusion must say *why*
it is unavailable rather than reading as an unevaluated marker with better manners. If
the count has become available by implementation time, take it — but do not assume it.

### 5. Changing a marker is a three-part edit, and the parts have different rules

This is the mechanical core (`_decomposition.md:616-642`; `xtask/src/spec_trace.rs`
module docs `:1-58`):

1. **§7.1 and §7.2 are generated.** `cargo xtask spec-trace --write` renders them
   between the `<!-- BEGIN GENERATED -->` markers (`spec/SPECIFICATION.md:8507`), and
   the gate without the flag *compares* the committed region against a fresh
   computation. Hand-editing inside the markers is not merely discouraged, it is inert
   (`spec/SPECIFICATION.md:8435-8446`).
2. **§1.3's census is hand-written and only *checked*.** The checker verifies the
   total, each maturity count, the "of which N are normative" figure and the document's
   own subtraction (`xtask/src/spec_trace.rs:447-509`; today's numbers at
   `spec/SPECIFICATION.md:219-222`). It is deliberately never generated, because §7.1,
   §7.2 and the census come from the same parser and would drift together silently —
   §1.3 is the only count a human computed by reading the document. **Edit it by hand
   and let the checker disagree with you if you are wrong.**
3. **An ID that leaves the clause space is retained, not deleted.** The marker becomes
   `[NON-NORMATIVE]` and *"its ID is retained so that citations resolve"* — CF-30 is the
   worked example (`spec/SPECIFICATION.md:209-212`). Deleting an ID breaks every
   citation, `RUNBOOK.md`'s included.

**§7.3 through §7.6 are authored and stay authored** (`spec/SPECIFICATION.md:8462-8468`).
The disposition rows for PS-32/PS-33/PS-35 (`:8769-8783`) and PS-33's §7.5 row
(`:9033`) currently say these three *should* move and are kept only because *"the work
list does not exist yet"*. It does now. Those rows are prose a parser will never
reconcile, so they are part of this edit or they become a lie the gate cannot see.

**The determined half of the census arithmetic.** PS-32 and PS-35 are `[FROZEN]` and
PS-33 is `[DEFERRED]` (`spec/SPECIFICATION.md:8661`, `:8662`, `:8664`). Moving all three
to `[NON-NORMATIVE]` leaves the total at 200 and takes FROZEN 139 → 137, DEFERRED 10 →
9, NON-NORMATIVE 2 → 5 and "of which N are normative" 198 → 195. The PS-18/PS-27/PS-30
verdicts move further counts on top of that; recompute the final figures, do not copy
these.

**A softened marker with no replacement text fails the gate, not the review.** A
`[PROVISIONAL]` or `[DEFERRED]` falsifier shorter than 12 characters is a build failure
under CF-38 (`spec/SPECIFICATION.md:213-217`; `xtask/src/spec_trace.rs:668-680`).

### 6. Two traps in the checker that will otherwise cost a debugging session

- **A backticked rule name that is not in `suite.rs` renders `†` — "must be written" —
  forever.** `spec-trace` resolves rule names against
  `crates/happenstance-testkit/src/suite.rs` plus the `wire::`-qualified test files, and
  nothing else. It exempts a clause from the dagger only when the `Rule:` text contains
  the literal words `unit test`, `compile test` or `meta-test`
  (`xtask/src/spec_trace.rs:1618-1642`), in which case the cell carries the clause's own
  words instead. An integration test in `crates/happenstance/tests/` matches none of
  those triggers, so **naming one as a bare backticked identifier makes §7.2 assert it
  does not exist even after it does.** Write PS-27's and PS-30's `Rule:` lines so the
  rendered cell tells the truth, and check the rendered §7.2 rows, not just the exit
  code.
- **Check 4 is skipped for every `PS` clause today**, because it short-circuits on
  `!has_suite(&c.id)` (`xtask/src/spec_trace.rs:695-697`) and no projection conformance
  suite exists. A green `spec-trace` is therefore *not* evidence that a `PS` clause's
  named rule resolves. Do not read the green as a check that was performed.

### 7. What this story finds and must not fix

CF-36 (`[FROZEN]`) names `cargo xtask spec-trace` as its rule, *"cross-referencing each
case's level marker (E2E-CASES.md:19-28)"* — and `xtask/src/spec_trace.rs` reads no
level marker at all. That is a clause citing a check that is not implemented, in exactly
the class of defect this project exists to surface. **Record it; do not fix it.** The
project's own rule is that a defect found by using the frozen contract is written down
with its clause ID and routed to a decision record, never absorbed and never patched in
passing (`project.md`, AC-012 and *Out of scope*, last bullet). The owning story is the
slice's downstream `defect-log-and-macros-verdict`; this story's obligation is to hand
it a non-speculative entry. The same rule applies to anything else this pass turns up.

### 8. The standing hazard this whole story is a defence against

There is an accepted open-question atom that states the failure mode in one line:
*"a falsifier that has already occurred without changing anything is a marker that has
quietly become decoration, and `spec-trace` cannot detect it — it sees that a marker
exists, not whether its condition has been met"*
(`.kb/open-questions/es-7-and-vt-9-provisional-markers.md`, summary). PS-33 has already
survived one document handover by living in an ADR that no phase read
(`spec/SPECIFICATION.md:8770`, `:8781-8783`). A verdict that restates the marker in
different words has not discharged anything.

### 9. The persona-journey slice this realizes

Activity **A5** of the story map's backbone — *"Read the events back into a read model:
an application-facing `Projection` over decoded events, **and the clause verdicts the
runner's existence settles**"* (`_storymap.md:27`), closing beat 4 of the journey
*"Choose a contract before a database"*. The reader served is not the `cargo add` user
but the next contributor and the stranger auditing the alpha: the specification is the
artefact this project publishes alongside the crate, and DoD 12 is read against it.

### 10. Not this story's to reopen

The four `CLAUDE.md` binding constraints; ADR-0007's runner split as a *design*
(this story evaluates its falsifier, which is not the same as re-litigating the split);
the transactional invariant on `ProjectionStore`; and every `[FROZEN]` `ES-*` clause —
no story in this project is licensed to amend one (`_storymap.md:152-154`). PS-3's
*verdict* is HS-P0016's; only the typed runner's **gating** is settled here, and it is
settled in `_design.md`, not by this story.

## Integration contract

- **Archetype**: `capability` — the observable deliverable is the specification a reader
  meets, held to the tree by a gate step.
- **Slice / milestone**: **M5 `projection-runner`**. Slice-mates implemented in the same
  context and mounted as one surface: `projection-trait-and-runner` (HS-S0026,
  this story's `depends_on`) and `polling-cost-measurement`.
- **Mount point**: **`spec/SPECIFICATION.md`** — the verdicts land in the clause bodies
  of §4.6/§4.8/§4.9 and in §1.3's census, and the document is mounted by the
  `specification traceability` step in `xtask/src/main.rs:315-325`, which is `REQUIRED`
  and therefore runs in `cargo xtask ci`, in `cargo xtask ci --fast`, and in the
  story-grain `affected_gate` (`.redkiln/config.yaml:40`, which runs `spec-trace`
  unconditionally precisely because *"a story whose whole deliverable is an edit to
  SPECIFICATION.md maps to no package"*). A verdict written into a companion note is not
  mounted.
- **Wires into**:
  - `xtask/src/spec_trace.rs` — the checker whose census check (`:447-509`), generated
    region (`:23-37` module docs) and rule-resolution rules (`:1618-1642`, `:695-697`)
    constrain how the edit may be written. Consumed, not modified.
  - `crates/happenstance-core/src/projection.rs` — the port the counted clauses
    constrain; `checkpoint`/`begin`/`commit`/`rollback` and nothing that runs.
  - `crates/happenstance/src/lib.rs` — where the slice-mate mounts `Projection` and
    `run_projection` behind `unstable-projection`; the callers this story counts.
  - `crates/happenstance/tests/` — **created by this story** where a verdict names a
    caller; the integration tests are the count's evidence.
  - `spec/E2E-CASES.md` — E2E-26, E2E-27 (`:677`, `:698`) and E2E-28 (`:725`, level
    `integration`), the cases the counted clauses name.
  - `.kb/_intake/` — staging only, and only on PS-33's superseding-ADR branch.
- **Renders surfaces**: **none.** This story renders none of `_design.md`'s six surface
  ids (`crate-root-rustdoc`, `crate-readme`, `first-program-doctest`,
  `worked-example-transcript`, `dsl-failure-message`, `compile-fail-diagnostic`). Its
  hook into the signed-off design is the `## Items` block, whose rows bind
  `happenstance::Projection` to *"PS-33 (the falsifier this trait's existence
  evaluates)"* (`_design.md:320-325`) and `happenstance::run_projection` to
  *"PS-27, PS-30"* (`_design.md:326-331`) — this is the story that discharges those two
  clause bindings.
- **Conformance rule(s)**: **none, and that is the correct answer.** PS-33 is a phase
  gate, not an adapter obligation, and *"no adapter can fail it"*
  (`spec/SPECIFICATION.md:5537-5540`). PS-27's and PS-30's instruments are
  integration-level and CF-36 forbids them being adapter conformance rules
  (`:8297-8298`). Nothing here is added to `crates/happenstance-testkit/src/suite.rs`.
- **Clause(s)**: **PS-18** (`:5200`), **PS-27** (`:5411`), **PS-30** (`:5462`) —
  verdicts written, markers moved; **PS-32** (`:5513`), **PS-33** (`:5529`), **PS-35**
  (`:5575`) — moved out of the clause space, IDs retained; **§1.3**'s census
  (`:219-222`) and the authored **§7.3** (`:8769-8783`) / **§7.5** (`:9033`) rows
  reconciled. No `[FROZEN]` `ES-*` clause is touched. PS-32 and PS-35 are `[FROZEN]`,
  and moving them is not an edit of convenience: it is the discharge
  `RUNBOOK.md:4079-4088` schedules for this phase by name, and the specification's own
  §7.3 already records that all three *should be prose*.
- **Advances DoD scenario**: initiative **DoD 12** — *"The clause ledger is audited at
  publish: a run over the specification reports every clause's maturity, and no clause
  is provisional with an empty falsifier; the count in the report matches
  `spec/SPECIFICATION.md`'s own stated figure"* (`initiative.md:393-396`). Contributes
  to **DoD 13** (the gate green on the assembled whole, *"including the specification
  cross-reference step"*). Discharges the project's own **DoD 4** and its AC-007 and
  AC-008.

## PR boundary

**In this PR**

- `spec/SPECIFICATION.md` — the four verdicts, the three retained IDs, §1.3's hand
  census, the authored §7.3/§7.5 rows, and the §7.1/§7.2 region as regenerated by
  `cargo xtask spec-trace --write`.
- `crates/happenstance/tests/**` — the integration test(s) backing any verdict that
  names a caller (feature-gated with `unstable-projection`, per `_design.md:746`).
- `crates/happenstance/Cargo.toml` — `[dev-dependencies]` only, if the tests need a
  runtime. The `[features]` block is the slice-mate's.
- `.kb/_intake/**` — only on PS-33's superseding-ADR branch, staged for the human's
  `/redkiln:kb-ingest`.
- `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/projection-clause-verdicts/**`
  — this story's own ledger and report.

The implementer **may** also touch the composition-root/wiring files named in the
Integration contract to mount this slice — that is not scope drift.

**Explicitly not in this PR**

- `crates/happenstance/src/**` — `Projection`, `run_projection`, `Progressed`,
  `ProjectionError` and the `unstable-projection` feature are `projection-trait-and-runner`'s.
- `crates/happenstance-core/src/**` — admissible only as the recorded outcome of AC-012's
  route, never as a convenience discovered mid-implementation (`_storymap.md:152-154`).
- `crates/happenstance-testkit/src/suite.rs` — no conformance rule is added; see CF-36.
- `xtask/src/spec_trace.rs` — the CF-36 gap is recorded, not fixed (Context pack §7).
- A new workspace member for an e2e crate; `MemoryProjectionStore` (HS-P0010's AC-012);
  `trybuild`; the PS-3 verdict; the polling-cost measurement.
- Any edit to `.kb/decisions/**`, and any invocation of `/redkiln:kb-ingest` — both are
  human handoffs (`_storymap.md:163-166`).

**Merge DoD one-liner** — PS-33, PS-27, PS-30 and PS-18 each carry a written verdict
naming its count or its exclusion, PS-32/PS-33/PS-35 have left the clause space with
their IDs retained, and `cargo xtask spec-trace` plus the story-grain affected gate are
green on the committed tree.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| PS-33's verdict is written, in one of exactly two admissible forms | Either an independent caller of the core-side checkpoint pump is named by path, or a superseding ADR collapsing the pump upward into `happenstance` is staged into `.kb/_intake/` for the human-invoked ingest. Read the tree first: the contract crate holds the port and nothing that runs, so "count the callers" is a judgement about where the invariant lives, not a `grep` | `spec/SPECIFICATION.md:5529-5546`; `references/adr/0007-projection-runner-decodes.md:118-121`; `.kb/decisions/0007-projection-runner-decodes.md:73-75`; `crates/happenstance-core/src/projection.rs:88-140`; `_decomposition.md:422` |
| PS-27 is settled by a count over what the slice actually built | Falsified if "record" turns out to mean "log a warning". Verdict names the runner path that writes a skip record into the same batch that advances the checkpoint past the poisoned position — or records the exclusion, if the alpha's `run_projection` has no failure-policy path. The signed-off signature carries no `on_error`; the states table says a skip is recorded. The tree at slice end decides which is true | `spec/SPECIFICATION.md:5411-5429`; `_design.md:563-570`, `:951-955`; `spec/E2E-CASES.md:698-712` |
| PS-30 is settled by a count, with its own contingency stated | Falsified if the fan-out runner is not built, which the clause makes contingent on a benchmark the workspace does not have. Verdict names the rollback-on-panic caller or records the exclusion and says which artefact would reopen it. Note the shape constraint that makes a fan-out runner hard here: `Batch<'a>` borrows from the store, is not `'static`, and cannot move across a `tokio::spawn` | `spec/SPECIFICATION.md:5462-5478`; `crates/happenstance-core/src/projection.rs:97-99`; `_decomposition.md:697-701`; `spec/E2E-CASES.md:725-740` |
| PS-18 is recorded as a documented exclusion naming its owner | Its clause text assigns the count to the projection-port phase and asks whether the SQLite adapter implemented it; neither adapter exists and the clause's subject — `reset`, `ResetError::Refused` — is absent from the port. The exclusion states that, names HS-P0010 as the owner of the eventual count, and does not read as an unevaluated marker | `spec/SPECIFICATION.md:5200-5216`; `crates/happenstance-core/src/projection.rs:88-140`; `_decomposition.md:660-673`; `RUNBOOK.md:4070-4077` |
| PS-32, PS-33 and PS-35 leave the clause space with IDs retained | Marker becomes `[NON-NORMATIVE]`, ID retained so citations resolve, the way CF-30 already is. Deleting an ID breaks every citation including `RUNBOOK.md`'s. The §7.3 disposition rows and the §7.5 row that say these three "should be prose … left because the work list does not exist yet" are updated in the same edit, because they are authored and no parser will reconcile them | `spec/SPECIFICATION.md:209-212`, `:8769-8783`, `:9033`; `RUNBOOK.md:4079-4088`; `_decomposition.md:633-638` |
| The generated region is regenerated, never hand-edited | `cargo xtask spec-trace --write` renders §7.1 and §7.2 between the markers; the gate without the flag fails when the committed copy and a fresh computation disagree. That equality, not the generator, is the load-bearing half | `spec/SPECIFICATION.md:8435-8446`, `:8507`; `xtask/src/spec_trace.rs:23-37` |
| §1.3's census is re-derived by hand and agrees with the checker | The checker verifies the total, each maturity count, the "of which N are normative" figure and the document's own subtraction — and deliberately never writes them, because §7.1/§7.2 and the census share a parser and would drift together silently. The three retained IDs alone take FROZEN 139→137, DEFERRED 10→9, NON-NORMATIVE 2→5, normative 198→195 at an unchanged total of 200; the four verdicts move further counts on top | `spec/SPECIFICATION.md:219-222`; `xtask/src/spec_trace.rs:39-58`, `:447-509` |
| No marker is left softened without replacement text | A `[PROVISIONAL]`/`[DEFERRED]` falsifier under 12 characters is a build failure under CF-38, not a review comment. A verdict that removes a falsifier without supplying the settled marker fails the gate | `spec/SPECIFICATION.md:213-217`; `xtask/src/spec_trace.rs:668-680` |
| Where a verdict names a caller, an executed test is the evidence | Integration test(s) under `crates/happenstance/tests/`, feature-gated `unstable-projection`, run by the story-grain affected gate. Not the adapter conformance suite: CF-36 forbids a clause backed only by integration-level cases from naming one, and E2E-28 is `Level: integration`. The "workspace e2e crate" §4.11 names does not exist and creating one is a runbook-grain scope change | `spec/SPECIFICATION.md:5695-5704`, `:8297-8308`; `spec/E2E-CASES.md:725-729`; `_decomposition.md:786`; `_design.md:746` |
| §7.2's rendered cells tell the truth about what exists | A backticked name not in `suite.rs` (or the `wire::` files) renders `†` = "must be written"; only the literal words `unit test`, `compile test` or `meta-test` route a clause to its own prose instead. An integration test matches none of them. Check the rendered rows, not just the exit code — and note check 4 short-circuits for every `PS` clause today, so a green run is not evidence a `PS` rule name resolves | `xtask/src/spec_trace.rs:1618-1642`, `:695-712`, `:1154`; `spec/SPECIFICATION.md:8469-8476` |
| Defects found on the way past are recorded and routed, never patched | CF-36 names `spec-trace` as cross-referencing each case's level marker and the checker reads no level marker at all. That entry, and anything else this pass turns up, is written down with its clause ID and handed to `defect-log-and-macros-verdict`; no `[FROZEN]` clause is edited to make a finding go away | `spec/SPECIFICATION.md:8297-8300`; `xtask/src/spec_trace.rs`; `project.md`, AC-012 and *Out of scope*; `.redkiln/config.yaml:5` |
| The gate is the assertion | `cargo xtask spec-trace` green, and `cargo xtask affected --base main` green — which runs the five file-reading lints and `spec-trace` unconditionally, for exactly this story's shape | `xtask/src/main.rs:315-325`; `.redkiln/config.yaml:30-40` |

## Data and migrations

**N/A — no schema, no store, no persisted state.** The only artefact this story migrates
is `spec/SPECIFICATION.md` itself, and its migration rule is stated rather than
implicit: **an ID that leaves the clause space is retained, not deleted**
(`spec/SPECIFICATION.md:209-212`). Every clause ID is a public reference target — cited
from `RUNBOOK.md`, from `.kb/` atoms, from `spec/E2E-CASES.md` and from this backlog —
so deleting PS-32, PS-33 or PS-35 would be a breaking change to a citation graph no
compiler checks. The `[NON-NORMATIVE]` marker is the tombstone, CF-30 is the worked
precedent, and §7.1's totals are what prove the tombstone was left rather than the row.

The one migration hazard worth naming: `crates/happenstance/tests/` does not exist yet,
so the first test file added there creates a new integration-test target for a crate
that has none. Feature-gate it with `unstable-projection` so a default-feature
`cargo test` neither compiles nor silently skips it into meaninglessness.

## Acceptance criteria

The persona each row serves is the **evaluator** on the journey *"Decide in one
sitting"* and the **next contributor** on *"Learn when you are finished"*
(`initiative.md:241-250`). Both meet this project through `spec/SPECIFICATION.md`, not
through the crate: the evaluator reads the clause ledger to decide whether the alpha's
promises are load-bearing, and the contributor reads a marker to learn whether a
question is open or answered. A marker that says *"provisional"* about a condition that
has already occurred lies to both of them, and neither has any instrument for detecting
it (`.kb/open-questions/es-7-and-vt-9-provisional-markers.md`).

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | GIVEN an evaluator reading `spec/SPECIFICATION.md` §4.9 to decide whether ADR-0007's runner split is a settled design or an open bet, WHEN they reach PS-33 (`:5529`), THEN the clause body states a verdict in one of exactly two admissible forms — an independent caller of the core-side checkpoint pump named by repo path, **or** a superseding ADR collapsing the pump upward staged under `.kb/_intake/` and cited by filename — and in neither form does the reader have to open an ADR to learn the outcome. | `cargo xtask spec-trace` (`xtask/src/main.rs:315`) green with PS-33's marker no longer `[DEFERRED]`; the verdict text read at the design/closeout gate against the two admissible branches (`_decomposition.md:422`); on branch two, `redkiln validate --kb` clean and the staged file present under `.kb/_intake/`. |
| **AC-002** | GIVEN a contributor who needs to know whether *"the projection runner records a skipped event"* is a promise or a hope, WHEN they read PS-27 (`:5411`), THEN the clause names the runner path that writes a skip record into the **same** batch that advances the checkpoint past the poisoned position — or records the exclusion with the reason the count is unavailable — and the verdict matches what `crates/happenstance/src/` actually contains at the end of the slice rather than what `_design.md`'s states table hoped for. | On the count branch: `crates/happenstance/tests/projection_clauses.rs::skip_and_record_is_atomic` passes under `cargo test -p happenstance --features unstable-projection`, and PS-27's rendered §7.2 row does not carry `†`. On the exclusion branch: `cargo xtask spec-trace` green and the exclusion names the artefact that would reopen it. |
| **AC-003** | GIVEN the same contributor asking whether a panicking projection can corrupt a read model, WHEN they read PS-30 (`:5462`), THEN the clause names the rollback-on-panic caller by path — or records the exclusion **and** names the contingency that governs it, that the fan-out runner is benchmark-gated and the benchmark is `polling-cost-measurement`'s deliverable, deliberately outside the gate (CF-34). | On the count branch: `crates/happenstance/tests/projection_clauses.rs::panicking_apply_rolls_back` passes under the same command, and PS-30's rendered §7.2 row does not carry `†`. On the exclusion branch: the clause text names the benchmark as the reopening artefact; `cargo xtask spec-trace` green. |
| **AC-004** | GIVEN a reader who has followed a `RUNBOOK.md` citation to PS-18 (`:5200`) expecting to learn whether reset protection is required of an adapter, WHEN they read the clause, THEN they meet a **documented exclusion** that states why the count is unavailable — no projection adapter has shipped, and the clause's subject (`reset`, `ResetError::Refused`) does not exist in `crates/happenstance-core/src/projection.rs` — and names `projection-store-freeze` (HS-P0010) as the owner of the eventual count, so the reader can tell a decision from a deferral. | `cargo xtask spec-trace` green with PS-18 no longer carrying an unevaluated maturity marker; the exclusion text reviewed against `_decomposition.md:660-673` at the gate; `grep` for `reset`/`ResetError` in `crates/happenstance-core/src/projection.rs` returning nothing, re-run at implementation time rather than assumed. |
| **AC-005** | GIVEN anyone who has ever cited PS-32, PS-33 or PS-35 — `RUNBOOK.md:4079-4088` has, and so has §7.3 — WHEN those three leave the clause space, THEN each **ID is retained** with a `[NON-NORMATIVE]` marker the way CF-30 already is (`:209-212`), every existing citation still resolves to a row, and the authored §7.3 disposition rows (`:8769-8783`) and PS-33's §7.5 row (`:9033`) — which today say these three are kept only because *"the work list does not exist yet"* — are reconciled in the same edit rather than left as prose no parser will catch. | `cargo xtask spec-trace` green; §7.1's totals show three `[NON-NORMATIVE]` rows added and none deleted; a `grep` over `RUNBOOK.md`, `spec/E2E-CASES.md`, `.kb/**` and `.bklg/**` for `PS-32`, `PS-33`, `PS-35` showing every hit still resolves to a retained ID. |
| **AC-006** | GIVEN the evaluator who trusts §1.3's census as the one count a human computed by reading the document (`:219-222`), WHEN the four verdicts and three retirements land, THEN the census is **re-derived by hand** — total, each maturity count, the "of which N are normative" figure, and the document's own subtraction — and the checker agrees with the human rather than the human copying the checker. | `cargo xtask spec-trace` green: the census check (`xtask/src/spec_trace.rs:447-509`) compares each stated figure against a fresh parse and fails on disagreement. The three retirements alone take FROZEN 139→137, DEFERRED 10→9, NON-NORMATIVE 2→5, normative 198→195 at total 200; the verdicts move further counts, so the committed figures must be recomputed, not copied from this row. |
| **AC-007** | GIVEN a maintainer who regenerates the traceability tables on a later phase, WHEN they run `cargo xtask spec-trace --write` on the committed tree, THEN the §7.1/§7.2 region between the `<!-- BEGIN GENERATED -->` markers (`:8507`) is **byte-identical** to what is committed — this story hand-edited nothing inside the markers — while §7.3–§7.6 stay authored (`:8462-8468`) and carry the reconciled rows from AC-005. | `cargo xtask spec-trace` without `--write` green, which is the equality assertion itself (`xtask/src/spec_trace.rs:23-37`); a second `--write` run leaving the working tree clean (`git status --porcelain` empty). |
| **AC-008** | GIVEN a reader who meets a verdict that says *"this is called"*, WHEN they look for the evidence, THEN they find an **executed** integration test under `crates/happenstance/tests/`, feature-gated `unstable-projection` per `_design.md:746`, that runs in the story-grain affected gate — not an adapter conformance rule, which CF-36 (`:8297-8298`, `[FROZEN]`) forbids for a clause backed only by integration-level cases, and not a "workspace e2e crate", which does not exist and whose creation is a runbook-grain scope change. | `cargo test -p happenstance --features unstable-projection` runs the named tests (non-zero test count — `running 0 tests` exits 0 and is not evidence); `cargo xtask affected --base main` green; `crates/happenstance-testkit/src/suite.rs` unchanged in the diff. |
| **AC-009** | GIVEN the gate itself as the reader of last resort, WHEN this edit lands, THEN no marker is left softened without replacement text — a `[PROVISIONAL]`/`[DEFERRED]` falsifier under 12 characters is a build failure under CF-38 (`:213-217`) — no `[FROZEN]` `ES-*` clause is amended, and `cargo xtask spec-trace` plus the story-grain affected gate are green on the committed tree, so the verdicts are held to the tree by a step that runs whether or not anyone types it. | `cargo xtask spec-trace` green (CF-38 enforcement at `xtask/src/spec_trace.rs:668-680`); `cargo xtask affected --base main` green (`.redkiln/config.yaml:40`); `cargo xtask ci --fast` green as this project's integration bar (`.redkiln/config.yaml:55`); the diff touching no `ES-*` clause body. |
| **AC-010** | GIVEN the project exists partly to find out what using a `[FROZEN]` contract reveals (BR-01), WHEN this pass finds that CF-36 names `cargo xtask spec-trace` as *"cross-referencing each case's level marker"* while the checker reads no level marker at all, THEN that finding is **recorded with its clause ID and routed** to `defect-log-and-macros-verdict` — along with anything else the pass turns up — and neither `xtask/src/spec_trace.rs` nor the frozen clause is edited to make it go away. | A defect entry naming `CF-36` present in this story's own artifacts and handed to the downstream story; the diff showing `xtask/src/spec_trace.rs` unchanged and CF-36's clause body unchanged; reviewed against `project.md`'s AC-012 and *Out of scope* at the gate. |

## Interaction quality

This story **renders none of `_design.md`'s six declared surfaces** (`crate-root-rustdoc`,
`crate-readme`, `first-program-doctest`, `worked-example-transcript`,
`dsl-failure-message`, `compile-fail-diagnostic` — `_design.md:40-86`), so the
composition family does not bind a rendered page here. The surface this story *does*
put in front of a reader is `spec/SPECIFICATION.md` itself, and the state family
transfers to it exactly, because a specification is navigated by citation the way a UI
is navigated by focus. Every invariant below is carried by an `AC-###` **row in the
table above**; none is a bullet here.

**STATE invariants**

| Invariant | The reading in this medium | Carried by | How it fails if unheld |
| --- | --- | --- | --- |
| **In-place, not a context jump** | The verdict lands in the clause body the reader is already looking at, at the mount point, never in a companion note or a story report the reader would have to be told about | AC-001, AC-002, AC-003, AC-004 | A verdict written into `_ledger.md` or a session log is unmounted: `spec-trace` cannot see it and no reader ever reaches it |
| **Non-occlusion** | A retired clause does not disappear; the `[NON-NORMATIVE]` marker is the tombstone, the ID is retained, and every prior citation still lands on a row | AC-005 | Deleting PS-32/PS-33/PS-35 breaks a citation graph no compiler checks, including `RUNBOOK.md:4079-4088`'s own |
| **Preserved selection and surrounding context** | The authored §7.3/§7.5 rows that describe these clauses are reconciled in the same edit, so the prose around the changed row does not contradict it | AC-005 | §7.3 keeps asserting three clauses *should* move "because the work list does not exist yet" after they already have — a lie no parser reconciles |
| **Reversibility** | The generated region is re-derivable from the tree: `--write` on the committed tree is a no-op, so the edit can be recomputed rather than trusted | AC-007 | A hand-edit inside the markers is inert *and* makes the gate's equality check fail on the next regeneration, in a diff nobody expected |
| **Reachability** (the keyboard-reachability analogue) | Every verdict is reachable from the clause ID a citation already uses; no new navigation, no new document, no new section a reader must be told to look in | AC-005, AC-006 | A reader following `RUNBOOK.md:408-413` to "the verdict" arrives at a clause that still reads provisional |
| **Honest status, no silent pass** | §1.3's census is the reader's only whole-document status indicator, and it is recomputed rather than left stale; §7.2's cells must not claim a test "must be written" once it exists | AC-006, AC-002, AC-003 | A stale census, or a `†` beside a rule that runs — both look green and both misinform |

**COMPOSITION invariants** — the surface-composition family from `_design.md` (density
budget `:837-871`, transience `:810-836`, hierarchy `:873-910`) applies to rendered
rustdoc/README/terminal surfaces and is **not applicable** to this story. Three of the
design's constraints do reach it, and each is carried by an AC:

| Constraint | Source | Carried by |
| --- | --- | --- |
| **Presentation exists at all** — a verdict is composed prose naming its count or its exclusion and its owner, never a bare marker flip. A marker changed with no text is the decorative outcome this story exists to prevent, and CF-38 makes the degenerate case a build failure at a **real number: 12 characters** | `spec/SPECIFICATION.md:213-217`; `.kb/open-questions/es-7-and-vt-9-provisional-markers.md` | AC-001–AC-004, AC-009 |
| **`Projection`/`run_projection` are opened-on-demand**, behind `unstable-projection`, off by default — so anything this story adds that touches them is gated the same way | `_design.md:746`, `:829` | AC-008 |
| **Anti-pattern 15** — the runner must not appear without its `unstable-projection` badge, and the changelog must not claim a feature the manifest does not have; this story adds no ungated surface and no manifest feature | `_design.md:1006-1008` | AC-008 |

## Error conditions

| id | Condition | Required handling |
| --- | --- | --- |
| **EC-001** | §1.3's hand census disagrees with the checker's fresh parse | `cargo xtask spec-trace` fails and prints stated-versus-computed. Fix the **census**, never the checker; the census is deliberately hand-written so a human count and a parser count can disagree (`xtask/src/spec_trace.rs:39-58`, `:447-509`) |
| **EC-002** | A verdict is hand-typed inside the `<!-- BEGIN GENERATED -->` markers | The edit is inert *and* fails the gate's equality check. Re-run `cargo xtask spec-trace --write` and put the prose in the clause body instead (`spec/SPECIFICATION.md:8435-8446`) |
| **EC-003** | A marker is softened but its replacement falsifier is shorter than 12 characters | Build failure under CF-38 (`xtask/src/spec_trace.rs:668-680`) — not a review comment. Supply the settled marker rather than an empty one |
| **EC-004** | PS-27's or PS-30's `Rule:` line names a backticked identifier that is not in `crates/happenstance-testkit/src/suite.rs` | §7.2 renders `†` — *"must be written"* — forever, even after the test exists, because only the literal words `unit test`, `compile test` or `meta-test` route a clause to its own prose (`xtask/src/spec_trace.rs:1618-1642`). Write the `Rule:` text so the rendered cell tells the truth, and **read the rendered row**, not the exit code |
| **EC-005** | A green `spec-trace` is read as proof that a `PS` clause's named rule resolves | It is not: check 4 short-circuits on `!has_suite(&c.id)` and no projection conformance suite exists (`xtask/src/spec_trace.rs:695-697`). Verify by reading §7.2's rendered rows |
| **EC-006** | PS-33 resolves onto its superseding-ADR branch and the implementer hand-writes `.kb/decisions/00NN-*.md` | Forbidden. An accepted decision atom is immutable and atoms are `/redkiln:kb-ingest`'s to author (`CLAUDE.md`, *Where the work lives*; the first attempt was reverted at `0269720`). Stage into `.kb/_intake/` and stop; the ingest is a human handoff (`_storymap.md:163-166`) |
| **EC-007** | The slice-mate's `run_projection` turns out to have no skip-and-record path, or no fan-out runner | Take the **exclusion** branch and say so with the reason. The signed-off signature carries no `on_error` (`_design.md:563-570`); writing a count-branch verdict against the states table rather than the tree is the failure this story exists to prevent |
| **EC-008** | A retired clause ID is deleted rather than retained | Every citation to it silently stops resolving. Retain the ID with `[NON-NORMATIVE]`, per CF-30's worked precedent (`spec/SPECIFICATION.md:209-212`) |
| **EC-009** | A contract defect is found mid-pass and looks cheap to fix | Record it with its clause ID and route it to `defect-log-and-macros-verdict`. No `[FROZEN]` clause is edited and `xtask/src/spec_trace.rs` is not patched (`project.md`, AC-012 and *Out of scope*) |
| **EC-010** | The new integration test is added without the `unstable-projection` gate, or with it but never run | Either it fails to compile under default features, or it compiles and is never executed — and `cargo test` exits 0 on `running 0 tests`, so the gate stays green while the evidence for AC-002/AC-003 does not exist. Assert a non-zero test count |
| **EC-011** | Implementation begins before `projection-trait-and-runner` has landed in the same context | Every count branch is unanswerable and the honest verdict is indistinguishable from a lazy one. This story is sequenced after its `depends_on` inside milestone **M5** and is not startable on an empty tree |

## Non-functional

| id | Requirement | Why, and where it is checked |
| --- | --- | --- |
| **NF-001** | The story-grain gate must remain meaningful for a diff that maps to no workspace package | `.redkiln/config.yaml:36-40` runs the five file-reading lints and `spec-trace` unconditionally *"because a story whose whole deliverable is an edit to SPECIFICATION.md maps to no package"* — this is that story, and it is the reason that line exists |
| **NF-002** | No new workspace member, no new runtime dependency | `crates/happenstance/Cargo.toml` may gain `[dev-dependencies]` only if the integration test needs a runtime; the `[features]` block is the slice-mate's. Creating an e2e crate is a runbook-grain scope change (`_decomposition.md:786`) |
| **NF-003** | The generated region is deterministic | `cargo xtask spec-trace --write` run twice must leave the tree clean; the gate's value is the equality, not the generator (`xtask/src/spec_trace.rs:23-37`) |
| **NF-004** | The citation graph stays whole across the edit | Clause IDs are public reference targets cited from `RUNBOOK.md`, `.kb/`, `spec/E2E-CASES.md` and `.bklg/`; no compiler checks them, so retention is the only mechanism (AC-005) |
| **NF-005** | This project's integration bar is `cargo xtask ci --fast`, not `cargo xtask ci` | `project.md` DoD 6 and `.redkiln/config.yaml:55`; `project.md`'s frontmatter carries `terminal: false`. The full gate is `publish-0-2-0-alpha-1`'s |
| **NF-006** | Every verdict is legible without opening an ADR | The evaluator's journey is *"decide in one sitting"* (`initiative.md:249-250`); a clause that says *"see ADR-0007"* has moved the reader, not answered them. ADR links stay opened-on-demand (`_design.md:832`) |

## Implementation notes (non-prescriptive)

Sequencing that has cost people a debugging session elsewhere in this repository, offered
as a route rather than a mandate:

1. **Read the tree before writing a word.** `crates/happenstance/src/lib.rs` and
   `crates/happenstance/tests/` at the end of the slice-mate's work are what the counts
   are over. `crates/happenstance-core/src/projection.rs` holds the port and nothing
   that runs, so PS-33's "count the callers" is a judgement about where the checkpoint
   invariant lives, not a `grep`.
2. **Decide each of the four verdicts before editing the document.** Write them down as
   four sentences first — count or exclusion, and the artefact named. A verdict decided
   while editing tends to become whatever the marker made easy.
3. **Write the clause bodies, then the retirements, then §7.3/§7.5.** The authored rows
   are the ones no parser will remind you about; doing them last means doing them from
   a diff you can see.
4. **Compute §1.3's census by hand and let the checker disagree.** The determined half
   is in the Context pack §5; the verdicts move further counts on top. Copying the
   checker's numbers defeats the only cross-check the document has.
5. **Regenerate last:** `cargo xtask spec-trace --write`, then `cargo xtask spec-trace`,
   then read §7.2's PS rows with your eyes. The exit code does not check what you think
   it checks (EC-005).
6. **If a count branch needs a test**, the smallest honest shape is one file,
   `crates/happenstance/tests/projection_clauses.rs`, `#![cfg(feature = "unstable-projection")]`
   at the top, with the two named tests. `crates/happenstance-testkit/src/fixtures.rs`'s
   `MemoryFixture` is the reference pattern for what "seed a store" means here.
7. **On PS-33's second branch**, stage the ADR into `.kb/_intake/` with valid
   frontmatter, name it in the clause, and stop. Suffix the wave id if a prior wave's
   file is still present, and note that the ingest glob picks up the intake README.
8. **Keep a running defect list as you go** (AC-010). CF-36 is already one; a pass that
   reads this much of the checker usually finds more, and reconstructing them afterwards
   from memory produces speculation.

## Tests and CI (merge gate)

Grounded in the testing brief's grain table (`_decomposition.md:746-751`) and its AC-007
and AC-008 rows (`:785-786`).

| Tier | Command / path | Proves |
| --- | --- | --- |
| Static — specification | `cargo xtask spec-trace` (`xtask/src/main.rs:315`) | Marker/citation consistency, §1.3's census against a fresh parse (`spec_trace.rs:447-509`), CF-38's 12-character floor (`:668-680`), and the §7.1/§7.2 equality between the committed region and a fresh computation. Carries AC-001, AC-004, AC-005, AC-006, AC-007, AC-009 |
| Static — regeneration | `cargo xtask spec-trace --write` then `git status --porcelain` | The generated region is derived, not typed: a second write is a no-op. Carries AC-007, NF-003 |
| Static — rendered read | Human read of §7.2's `PS-18`/`PS-27`/`PS-30`/`PS-33` rows after regeneration | That no rule cell carries `†` for a test that exists, and that check 4's short-circuit was not mistaken for a check (`spec_trace.rs:695-697`, `:1618-1642`). Carries AC-002, AC-003, EC-004, EC-005 |
| Integration | `cargo test -p happenstance --features unstable-projection` over `crates/happenstance/tests/projection_clauses.rs` — `skip_and_record_is_atomic`, `panicking_apply_rolls_back` | On a count branch, that the skip record and the checkpoint advance in one commit and that a panicking `apply` rolls the batch back. Non-zero test count asserted, because `running 0 tests` exits 0. Carries AC-002, AC-003, AC-008 |
| Story grain | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | fmt + `clippy -D warnings` + tests for touched packages and their dependents, **plus the five file-reading lints and `spec-trace` unconditionally** — the clause that makes this story gateable at all. Carries AC-008, AC-009, NF-001 |
| Reachability, static | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | That the specification's cross-references still resolve after three IDs left the clause space. Carries AC-005 |
| Integration, project bar | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | This project's DoD-6 bar: every `REQUIRED` step, all four `wasm32` steps, no feature powerset. Carries AC-009, NF-005 |
| KB validation *(PS-33 branch two only)* | `redkiln validate --kb` | Staged intake frontmatter conformance and accepted-decision immutability against `HEAD`; proves nothing under `.kb/decisions/` was hand-edited. Carries AC-001, EC-006 |
| Record, reviewed | This story's `_ledger.md` + the defect entry handed to `defect-log-and-macros-verdict` | AC-007 is explicitly *"Record, not a test"* in the testing brief (`_decomposition.md:785`): no compiled assertion proves "we looked and decided". The ledger's cited evidence is the substitute, and `require_ledger: true` (`.redkiln/config.yaml:67`) blocks `implement → report` without it. Carries AC-001, AC-010 |

**Not run here, on purpose:** `cargo xtask ci` in full (the terminal project's bar), the
`cargo-hack` feature powerset (dropped by `--fast`, and this story ships no feature), and
`event_store_conformance!` — no conformance rule is added, per CF-36.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Mitigation, in this PR |
| --- | --- | --- |
| **The counts depend on code this story does not write.** PS-27's and PS-30's verdicts are facts about what `projection-trait-and-runner` (HS-S0026) built; if it lands without a failure-policy path, both collapse to exclusions | High / Medium | Both branches are pre-authorised in the acceptance criteria (AC-002, AC-003) and in EC-007. The exclusion is a *verdict*, not a failure — what is forbidden is a third outcome, "still provisional" (`RUNBOOK.md:4073-4077`) |
| **PS-33's second branch cannot complete inside the story.** Staging an ADR into `.kb/_intake/` leaves the atom unwritten until a human runs `/redkiln:kb-ingest` | Medium / Medium | The clause cites the staged filename, and the ledger's evidence is the staged file plus `redkiln validate --kb`. The handoff is named in `_storymap.md:163-166` and is not a gap this story may close itself (EC-006) |
| **The census arithmetic is easy to get almost right.** Four verdicts and three retirements move five figures that must agree with each other and with the checker | High / Low | `spec-trace` fails loudly and prints stated-versus-computed (EC-001). The determined half of the arithmetic is in the Context pack §5, marked as a starting point to recompute rather than copy |
| **A green gate is mistaken for a checked one.** Check 4 skips every `PS` clause and §7.2 will happily render `†` beside a passing test | Medium / High | AC-002 and AC-003 verify on the **rendered row**, not the exit code; EC-004 and EC-005 name the mechanism |
| **Scope creep into the checker.** The CF-36 gap is a one-line-looking fix in `xtask/src/spec_trace.rs` and this story reads that file closely enough to be tempted | Medium / High | AC-010 makes recording the defect the deliverable and the *unchanged* `spec_trace.rs` diff the evidence. `project.md`'s AC-012 route is the only admissible outlet |
| **Slice coupling with `polling-cost-measurement`.** PS-30's contingency names a benchmark that slice-mate produces, deliberately outside the gate (CF-34) | Low / Low | The verdict cites the benchmark as the reopening artefact; it does not wait on the number, and the number is not a gate step |
| **A first `tests/` target for `happenstance`** changes what `cargo test -p happenstance` collects for every later story in the project | Low / Low | One file, feature-gated `unstable-projection` (`_design.md:746`), so default-feature runs are unaffected |
| **Reviewer disagreement on an exclusion.** "Documented exclusion" can read as a polite deferral | Medium / Medium | Each exclusion must name *why* the count is unavailable **and** the owner or artefact that would reopen it (AC-003, AC-004). An exclusion without both is the marker with better manners the runbook forbids |

## Dependencies

**Blocks on**

- `projection-trait-and-runner` — same milestone (**M5 `projection-runner`**), implemented
  first in the same context. It lands `Projection`, `run_projection`, `Progressed`,
  `ProjectionError` and the `unstable-projection` feature in `crates/happenstance/`; those
  are the callers PS-27 and PS-30 are counted against, and PS-33's judgement is about
  where the checkpoint invariant ended up once they exist (EC-011).

**Unlocks**

- `publish-0-2-0-alpha-1` — names this story in its own `depends_on` (`_storymap.md`, M7):
  the alpha does not ship with four unevaluated markers in the document it publishes
  alongside the crate.
- `defect-log-and-macros-verdict` — consumes this story's CF-36 entry (and anything else
  the pass turns up) as non-speculative input to the BR-01 defect record.

**Neither blocks nor is blocked**

- `polling-cost-measurement` — slice-mate; PS-30's verdict cites its benchmark as a
  reopening artefact but does not wait on the number (CF-34 keeps it outside the gate).

## Anchors (progressive disclosure)

Deferred depth. Open each at the moment named — not before, and not instead of the
Context pack.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` | The mount point. §4.6/§4.8/§4.9 hold the four clause bodies being edited (PS-18 `:5200`, PS-27 `:5411`, PS-30 `:5462`, PS-33 `:5529`), §1.3 the hand census (`:219-222`), CF-30 the retention precedent (`:209-212`), CF-38 the 12-character floor (`:213-217`) | Before writing any verdict — this is the file the story edits | AC-001, AC-002, AC-003, AC-004, AC-005, AC-006, AC-009 |
| `xtask/src/spec_trace.rs` | The checker whose behaviour dictates how the edit may be written: the census check (`:447-509`), the generated-region equality (module docs `:23-37`), CF-38 enforcement (`:668-680`), check 4's `PS` short-circuit (`:695-697`) and rule-name resolution with its three exempting phrases (`:1618-1642`). Consumed, never modified | Before editing §1.3 or writing a `Rule:` line; again before believing a green run | AC-006, AC-007, AC-002, AC-003, AC-010 |
| `RUNBOOK.md` | Phase 7's exit criteria state the method in one sentence — *"name the callers, or promote the clause to a documented exclusion"* (`:4064-4088`) — and `:408-413` records that PS-33 is evaluated here and nowhere else | First, before deciding what a valid verdict is | AC-001, AC-002, AC-003, AC-004, AC-005 |
| `crates/happenstance-core/src/projection.rs` | The port the counted clauses constrain: `checkpoint`, `begin`, `commit`, `rollback` — and nothing that runs. Confirms PS-33's subject is not a function and PS-18's subject (`reset`/`ResetError`) is absent entirely | Before writing PS-33's and PS-18's verdicts | AC-001, AC-004 |
| `crates/happenstance/src/lib.rs` | Where the slice-mate mounts `Projection` and `run_projection` behind `unstable-projection`. The tree here at slice end — not the design's states table — decides whether PS-27 and PS-30 take the count branch | Immediately after the slice-mate lands, before writing PS-27/PS-30 | AC-002, AC-003, AC-008 |
| `references/adr/0007-projection-runner-decodes.md` | The long record carrying the falsifier verbatim (`:118-121`) and the narrowly-rejected alternative — collapsing the pump upward — that is PS-33's second admissible branch | While choosing between PS-33's two branches | AC-001 |
| `.kb/decisions/0007-projection-runner-decodes.md` | The canonical atom; states the same risk at `:73-75`. Immutable — cite it, never edit it, and never hand-write its successor | When drafting the superseding ADR for staging | AC-001 |
| `.kb/open-questions/es-7-and-vt-9-provisional-markers.md` | States the standing hazard in one line: a falsifier that has already occurred is a marker that has quietly become decoration, and `spec-trace` cannot detect it | Before deciding a verdict is "written" — it is the test for restating versus discharging | AC-001, AC-002, AC-003, AC-004 |
| `spec/E2E-CASES.md` | E2E-26/E2E-27 (PS-27, `:677`, `:698`) and E2E-28 (PS-30, `:725`, marked `Level: integration`). The level marker is why CF-36 forbids these clauses naming an adapter conformance rule | When siting the integration tests and writing their `Rule:` lines | AC-002, AC-003, AC-008 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` | The architecture brief's three-part edit rules (`:616-642`), the two admissible PS-33 outcomes (`:422`), the tensions closing PS-18's counting branch (`:658-712`), and the testing brief's AC-007/AC-008 rows (`:785-786`) | Before the first edit, and again when choosing where the tests live | AC-001, AC-004, AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` | Binding. `## Items` ties `happenstance::Projection` to PS-33 (`:320-325`) and `run_projection` to PS-27/PS-30 (`:326-331`); the states table (`:951-956`) and the signature with no `on_error` (`:563-570`) are what the tree must be checked against; `unstable-projection` gating at `:746` | Before writing PS-27/PS-30 and before adding the test file | AC-002, AC-003, AC-008 |
| `crates/happenstance-testkit/src/suite.rs` | The only file (with the `wire::` test files) against which `spec-trace` resolves a backticked rule name. Unchanged by this story — read it to confirm why an integration test named there would render `†` | Before writing PS-27's/PS-30's `Rule:` text | AC-002, AC-003, AC-008 |
| `crates/happenstance-testkit/src/fixtures.rs` | `MemoryFixture` is the reference pattern for seeding an isolated store in this workspace, and the shape any new test-side store follows | Only if a count branch needs the integration test | AC-008 |
| `.redkiln/config.yaml` | Wires the four verify grains. `:36-40` explains, in its own comment, why the story gate runs `spec-trace` unconditionally for a story shaped exactly like this one; `:55` names this project's integration bar; `:67` makes the ledger blocking | When running the gate and when filling the ledger | AC-008, AC-009, NF-001, NF-005 |
| `xtask/src/main.rs` | `:315` is the `specification traceability` `REQUIRED` step — the mount that makes these verdicts held to the tree rather than filed | When claiming the mount point is real | AC-007, AC-009 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` | AC-007 (`:185-188`), AC-008 (`:189-191`) and DoD 4 (`:232`) are the criteria this story discharges; AC-012 and *Out of scope* define the defect route this story must use instead of fixing | At the start, and again when a defect is found | AC-001, AC-004, AC-010 |
| `.bklg/from-contract-to-published-library/initiative.md` | DoD 12's clause-ledger audit (`:393-396`) is what this story advances, and the referenced personas/journeys (`:227-250`) are who the verdicts are legible for | When framing why a verdict must read as a decision | AC-001, AC-006, NF-006 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md` | The M5 row and this story's place in it (`:58`), the "no story amends a `[FROZEN]` `ES-*` clause" rule (`:152-154`) and the human-handoff rule for `/redkiln:kb-ingest` (`:163-166`) | Before touching anything outside the PR boundary | AC-009, EC-006 |

## Clarifications resolved during spec

1. **The AC set is exactly the ten the front half decided** — AC-001 through AC-010,
   none added and none dropped. They partition as: four verdicts (AC-001–AC-004,
   discharging project AC-007 and AC-008), three mechanics of the three-part edit
   (AC-005–AC-007), the evidence rule (AC-008), the gate (AC-009) and the defect route
   (AC-010).
2. **"Settled by counting" admits exactly two outcomes per clause, never three.** A
   named caller with running evidence, or a documented exclusion carrying its reason and
   its owner. "Re-evaluate later" is inadmissible here by name (`RUNBOOK.md:4073-4077`).
   This is written into every verdict AC rather than left to reviewer taste.
3. **PS-27's and PS-30's tests land in `crates/happenstance/tests/`.** §4.11 names a
   "workspace e2e crate" that does not exist; creating a workspace member is a
   runbook-grain scope change, and the testing brief's own first candidate is a `tests/`
   directory under `crates/happenstance/` (`_decomposition.md:786`). CF-36's normative
   half — *not the adapter suite* — is satisfied either way.
4. **This story renders no `_design.md` surface**, so the composition family is recorded
   as not applicable rather than silently omitted; the state family is mapped onto the
   specification document, which is the surface a reader actually meets here. Three
   design constraints still bind and each is carried by an AC.
5. **AC-007's verification is a record, not a compiled test** — the testing brief says so
   in as many words (`_decomposition.md:785`). The substitute is the ledger's cited
   evidence plus `require_ledger: true` (`.redkiln/config.yaml:67`), not a test invented
   to make the row look symmetrical.
6. **The integration test file is named** `crates/happenstance/tests/projection_clauses.rs`
   with `skip_and_record_is_atomic` and `panicking_apply_rolls_back`, so the ledger can
   carry a real path. If a count branch collapses to an exclusion, that AC's ledger
   evidence is the clause text and the `spec-trace` run instead — recorded as such, not
   left blank.
7. **PS-33's superseding-ADR branch ends at `.kb/_intake/`.** The atom is authored by the
   human-invoked `/redkiln:kb-ingest`; this story neither runs it nor writes under
   `.kb/decisions/`.
8. **The census figures in the Context pack are a starting point, not the answer.** The
   three retirements' arithmetic is determined; the four verdicts move further counts on
   top, so AC-006 requires recomputation and the checker is the arbiter.
