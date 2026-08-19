---
id: HS-P0016
uid: bcc86c
type: project
slug: publication-and-positioning
title: 0.2.0 — where private opinions become promises
parent: HS-I0006
initiative: from-contract-to-published-library
project: publication-and-positioning
status: in-review
process: project
stage: design
automation: HITL
severity: null
blocked_by: []
blocks: []
terminal: false
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-12T12:57:13.586Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 7289a0c4
---
# 0.2.0 — where private opinions become promises

## One-line objective

Publish `0.2.0` with every promise it makes — the MSRV, the public surface, each
clause's maturity, the compliance claim and what the crate says on first contact —
checked at the moment of publish rather than asserted.

## How this advances the initiative

The initiative's fourth goal is *"publication converts private opinions into
promises"*, and this project is where that conversion happens. Everything upstream
of it produces evidence; this project decides what is claimed on the strength of
that evidence and then makes the claim unrevocable.

Three things about this repository make that a real body of work rather than a
`cargo publish` invocation.

**Nothing here can currently see a semver break.** `cargo xtask ci` is defined once
in `xtask/src/main.rs` and its mandatory steps are fmt, clippy, tests, four wasm32
steps, docs, `proof-artefact`, `spec-trace`, five file-reading lints and
`package-check` — there is no surface-diff step, because until
`typed-layer-and-alpha-release` ships `0.2.0-alpha.1` there is no baseline to diff
against. `_decomposition.md`'s sequencing section names exactly this: that project
is substrate for this one *because* it creates the baseline this project's central
instrument depends on.

**The MSRV is still a preference.** `.kb/decisions/0004-edition-and-msrv.md:83-86`
says so in terms — *"a preference until first publish (phase 12) and a promise to
downstream consumers afterward"* — and
`.kb/decisions/0029-msrv-raised-to-1-97-1.md:94-95` closes with the standing
instruction that *"phase 12 must revisit the floor at first publish."* This project
is phase 12.

**Forty-nine clauses are provisional.** `spec/SPECIFICATION.md:219-221` counts 200
clause IDs, 198 normative: 139 `[FROZEN]`, 49 `[PROVISIONAL]`, 10 `[DEFERRED]`, two
`[NON-NORMATIVE]`. `spec-trace` already forbids an empty falsifier (CF-38), but the
falsifier *ledger* that says who discharges each one is known to be short by five
rows — `RUNBOOK.md:622-635` records that ES-41, ES-42, CF-39 and CF-40 are missing
and ES-10 is still listed though no longer provisional, and states plainly that the
repair *"cannot wait past"* the phase-12 audit, because it is that audit which reads
the table.

This project carries no `architecture` brief by design (`_decomposition.md`:261,
278) — it makes promises about a surface it does not design. Its briefs are `ux`
(the evaluator's surface: landing page, README, docs.rs — the only "screen" this
library has), `testing` and `deployment` (the real crates.io release event).

## In scope (this project)

- **The `0.2.0` release event itself**, and the decision of which crates ship in it.
  `xtask/src/package.rs` already derives the publishable set from `cargo metadata`
  and reconciles it against a hand-written intention list, so promoting a crate is a
  two-sided change this project owns end to end.
- **The public-surface diff against the `0.2.0-alpha.1` registry baseline**, its
  recorded report, and the version number following what the diff found. A new gate
  instrument — nothing in `xtask/src/main.rs`'s `REQUIRED` or optional steps does
  this today.
- **The clause-ledger audit at the moment of publish**: every clause frozen, demoted
  to non-normative prose, or carrying a non-empty falsifier, with the report's count
  reconciled against `spec/SPECIFICATION.md`'s own §1.3 figure — the one census in
  the document a human computed by reading it, which
  `xtask/src/spec_trace.rs:39-57` explains must never become generated.
- **Repairing the five known-short falsifier-ledger rows** named at
  `RUNBOOK.md:622-635`: ES-41, ES-42, CF-39, CF-40 added, ES-10's row removed.
- **Re-reading each of the ten `[DEFERRED]` clauses** for whether the deferral is
  still honest at publish, each carrying a reason a consumer can read.
- **The MSRV becoming a promise** — a new decision atom recording the justification,
  since `.kb/decisions/0004-edition-and-msrv.md` and `0029-msrv-raised-to-1-97-1.md`
  are both `status: accepted` and therefore immutable.
- **Registry-facing completeness**: docs.rs green under `--all-features`, and the
  licence, description and README verified by looking at the rendered page rather
  than at the manifest — the exact gap `xtask/src/package.rs:4-18` says the metadata
  hides.
- **The stranger-install smoke**: a scratch project outside this workspace adding the
  published crate from the registry and completing a write-then-read cycle against
  the published version, not a path dependency.
- **What the crate says on first contact** — DT-1 (which claim leads), DT-4 (where
  the positions-and-gaps promise is stated), DT-5 (is the clause-maturity vocabulary
  published), DT-6 (explicit comparison to the nearest live peers) — each resolved in
  this project's `_design.md`.
- **The AC-08 compliance evidence**: what "DCB-compliant" points at, which
  implementations it names, and the date it was checked.
- **PS-3's verdict** — the projection port ships frozen or behind
  `unstable-projection` — decided here on evidence supplied by
  `projection-store-freeze` (`_decomposition.md`:41).

## Out of scope (this project)

Each exclusion names the sibling that owns it.

- **Creating the `0.2.0-alpha.1` semver baseline** → `typed-layer-and-alpha-release`.
  Already done by the time this project starts; this project consumes it.
- **The typed layer, the worked example, and DT-2** → `typed-layer-and-alpha-release`.
- **Designing the public API surface, or amending it** → the crate that owns it. This
  project carries no `architecture` brief; if the surface diff or the clause audit
  says something must change, that is an upstream project's change or a new decision
  atom and a re-plan, not an edit made here.
- **The `ProjectionStore` freeze, the projection suite's capability-declension policy
  and DT-3** → `projection-store-freeze`. **The freeze verdict** →
  `ladybug-projection-store`. This project only records PS-3's ship-shape verdict on
  their evidence.
- **Any adapter's conformance run** → `sqlite-durable-store`,
  `cloudflare-durable-object-store`, `postgres-and-neon-stores`,
  `ladybug-projection-store`. This project cites their results; it does not produce
  them.
- **Replication identity, the merge rule and DT-7** → `replication-identity-and-ingest`.
  **What a store may forget** → `retention-and-incomplete-logs`. Both land *after*
  this release (`_decomposition.md`:150-153).
- **Publishing `happenstance-sync` or `happenstance-sync-testkit`** → out of this
  release train entirely, per the charter. The sync port stays out of the contract
  crate precisely so publishing never waits on replication (`CLAUDE.md`).
- **Any `1.0` or post-1.0 semver commitment.** `0.2.0` is the ceiling; under 0.x the
  minor bump *is* the breaking-change boundary and that is the promise being made.
- **Re-observing DoD 1–15 as a set on the assembled library, and promoting the
  personas** → `closeout-and-durable-audience`, which is the terminal project and the
  only one wired to `verify.e2e` (`.redkiln/config.yaml:60`).
- **Amending anything `[FROZEN]`.** If the audit finds a frozen clause that is wrong,
  it blocks the release and becomes a new decision atom and a re-plan.

## Derived requirements

Expanded from the initiative requirements this project owns (BR-05⁰, BR-06, BR-07,
BR-08, BR-09, BR-14, BR-17 and BR-15 as a standard of work).

1. **DR-1 (BR-05⁰)** — `0.2.0` is published to crates.io, and the crate set that
   ships is a recorded decision rather than a default. The working default is three
   crates — `happenstance`, `happenstance-core`, `happenstance-testkit` — because
   AC-03 and DoD 9 name the installable crate in the singular and
   `xtask/src/package.rs` asserts against exactly the derived publishable set. If the
   answer is more than three, each adapter project owes a name reservation and a
   README, **so it must be decided early enough to change their scope**
   (`_decomposition.md`:302-309).
2. **DR-2 (BR-07)** — a surface-diff instrument exists, runs against the
   `0.2.0-alpha.1` registry baseline, and its report is committed. The version chosen
   for the release is derived from that report, not from intention.
3. **DR-3 (BR-06)** — an audit run over `spec/SPECIFICATION.md` reports every
   clause's maturity at the publish commit; no clause is `[PROVISIONAL]` with an
   empty falsifier; the report's totals reconcile against §1.3's hand-computed
   figures, and the reconciliation is checked rather than eyeballed
   (`xtask/src/spec_trace.rs:39-57`).
4. **DR-4 (BR-06)** — the falsifier ledger at `RUNBOOK.md:622-635` is repaired before
   the audit reads it: ES-41, ES-42, CF-39 and CF-40 gain rows with a falsifier and
   an owning phase; ES-10's row is removed. Each added row names its falsifier
   deliberately — the runbook says adding a row *"is not a thing to do in passing."*
5. **DR-5 (BR-06)** — each of the ten `[DEFERRED]` clauses is re-read at publish and
   carries a stated reason a consumer can read for why the deferral is still honest.
6. **DR-6 (BR-08)** — the MSRV is stated as a promise with a recorded justification,
   in a **new** decision atom authored through the ingest path. It may not be an edit
   to `.kb/decisions/0004-edition-and-msrv.md` or `0029-msrv-raised-to-1-97-1.md`:
   both are `status: accepted`, `validate --kb` checks accepted atoms against `HEAD`,
   and the decision map states the immutability rule
   (`.kb/maps/decision-map.md:30-34`). The atom must also state whether an MSRV bump
   remains a minor bump now that the policy at
   `.kb/decisions/0004-edition-and-msrv.md:83-86` binds a real consumer.
7. **DR-7 (BR-09)** — docs.rs renders green under `--all-features`, and licence,
   description and README are verified on the rendered registry page. `cargo package
   --list`'s assertion proves the files are *inside* the artifact; it proves nothing
   about how the page reads.
8. **DR-8 (AC-03 / DoD 9)** — a scratch project outside this workspace resolves the
   published version from the registry, with no path dependency and no workspace
   feature unification, and completes a write-then-read cycle.
9. **DR-9 (BR-14 / DT-4)** — what the library promises about positions and gaps is
   stated somewhere a newcomer reaches before depending on it, in plain language. Two
   DCB-labelled stores already disagree on this in public; the open question
   `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` records that the
   gap-read rule is named by two accepted decisions and owned by neither, so the
   statement must say what is actually true today rather than what the rule set
   implies.
10. **DR-10 (AC-08)** — the compliance claim points at reachable evidence that names
    which implementations the suite was checked against and when. The initiative's
    risk register requires published confidence be scoped to what was actually
    checked and when, because *"passed against N adapters"* is a snapshot.
11. **DR-11 (BR-17 / DT-1, DT-5, DT-6)** — positioning is decided, not defaulted: the
    lead claim, whether the clause-maturity vocabulary is published to consumers, and
    whether an explicit peer comparison is stated. Each recorded in `_design.md`.
12. **DR-12 (PS-3)** — the projection port's ship shape at `0.2.0` is decided on
    `projection-store-freeze`'s and `ladybug-projection-store`'s evidence.
13. **DR-13 (BR-15)** — every answer this project settles lands as a decision atom
    naming the alternatives that lost, authored through `/redkiln:kb-ingest` from
    `.kb/_intake/`, never hand-written (the first attempt at hand-authoring was
    reverted at `0269720`). ADR numbers 0017–0028 are allocated to sibling projects
    by `_decomposition.md`:119-121 and 0029 is on disk, so this project's atoms take
    the next free numbers above the corpus rather than any of those.
14. **DR-14 (BR-12)** — nothing in the release path drops the `!Send` flavour. The
    four wasm32 steps in `cargo xtask ci` stay mandatory on the published tree, and
    the published feature set is checked to still admit the constrained-runtime
    target.
15. **DR-15** — nothing `[FROZEN]` is amended by this project. A frozen clause the
    audit finds wrong **blocks the release** and becomes a new decision atom and a
    re-plan.

## Acceptance criteria

Project-grain and testable. These are the spine the story map must cover.

- **AC-001 — `0.2.0` is published, for a decided crate set.** The release exists on
  crates.io at `0.2.0`, and the set of crates in it matches a recorded decision that
  names the rejected alternative; `xtask/src/package.rs`'s derived set and its
  intention list agree on that set with no reconciliation failure. *(BR-05⁰, DR-1)*
- **AC-002 — the release is diffed, not asserted.** A surface comparison runs against
  the `0.2.0-alpha.1` registry baseline, its report is committed in the tree, and the
  published version number is the one that report's findings imply. Running it
  against a deliberately breaking change fails it. *(BR-07, AC-11, DoD 11, DR-2)*
- **AC-003 — the clause ledger is audited at publish and reconciles.** An audit over
  `spec/SPECIFICATION.md` at the publish commit reports every clause's maturity, no
  clause is `[PROVISIONAL]` with an empty falsifier, and the report's totals equal
  §1.3's stated 200 / 198 / 139 / 49 / 10 / 2. A seeded disagreement between the two
  fails the check. *(BR-06, AC-10, DoD 12, DR-3)*
- **AC-004 — the falsifier ledger is no longer short.** `RUNBOOK.md`'s provisional
  groups carry rows for ES-41, ES-42, CF-39 and CF-40, each with a falsifier and an
  owning phase; ES-10's row is gone; and the ledger's clause set equals
  `spec-trace`'s list of `[PROVISIONAL]` IDs, checked rather than counted by hand.
  *(BR-06, DR-4)*
- **AC-005 — every deferral is still honest, in writing.** Each of the ten
  `[DEFERRED]` clauses carries a stated reason, readable by a consumer, that was
  re-read at this publish rather than inherited. *(BR-06, DR-5)*
- **AC-006 — the MSRV is a promise with a recorded justification.** A new accepted
  decision atom states the floor, why it is where it is, and what an MSRV bump means
  to a consumer now that one exists; `.kb/decisions/0004-edition-and-msrv.md` and
  `0029-msrv-raised-to-1-97-1.md` are unmodified; `redkiln validate --kb` passes.
  *(BR-08, AC-12, DR-6)*
- **AC-007 — the published crate looks finished.** The rendered documentation build is
  green under all features, and the licence, description and README have been checked
  **on the rendered registry page**, with the check recorded. *(BR-09, DoD 10, DR-7)*
- **AC-008 — a stranger can install it.** From a scratch project outside this
  workspace, adding the published crate from the registry and running the smallest
  write-then-read cycle succeeds against the published version, not a path
  dependency. This is the project's proof artefact. *(AC-03, DoD 9, DR-8)*
- **AC-009 — the compliance claim can be checked instead of trusted.** The evidence
  behind "DCB-compliant" is reachable from a published artefact, names the
  implementations the suite was run against, and carries the date it was run.
  *(AC-08, DR-10)*
- **AC-010 — the positions-and-gaps promise is findable and readable.** A newcomer can
  learn what the library promises about sequence positions and gaps, in plain
  language, without reading the source, from wherever DT-4 decides it lives.
  *(BR-14, AC-09, DT-4, DR-9)*
- **AC-011 — the first-contact decisions are recorded, not defaulted.** DT-1, DT-4,
  DT-5 and DT-6 each have a written resolution in this project's `_design.md`, each
  naming the option that lost and why. None is left unowned at release. *(BR-17,
  DT-1/4/5/6, DR-11)*
- **AC-012 — the projection port's ship shape is decided.** PS-3 has a recorded
  verdict — frozen, or behind `unstable-projection` — citing the evidence
  `projection-store-freeze` and `ladybug-projection-store` produced. *(PS-3, DR-12)*
- **AC-013 — every answer this project settled is on disk as a decision atom.** Each
  states the alternatives that lost, was authored through the ingest path from
  `.kb/_intake/`, and takes a number free of the 0017–0028 allocation and of 0029.
  *(BR-15, DR-13)*
- **AC-014 — the constrained-runtime flavour survives the release.** The published
  tree still passes all four wasm32 steps of `cargo xtask ci`, and the published
  feature set admits the `!Send` flavour — asserted, not assumed. *(BR-12, ADR-0001,
  DR-14)*
- **AC-015 — nothing frozen was amended to get here.** No `[FROZEN]` clause differs
  between the tree this project received and the tree it published. If the audit
  found one wrong, the release is blocked and a decision atom records it. *(DR-15)*
- **AC-016 — the full gate is green on the exact tree that was published.** `cargo
  xtask ci` — the whole gate, not `--fast` — passes on the publish commit, including
  `spec-trace`, `package-check` and the nightly `--cfg docsrs` build that `--fast`
  drops. *(supports DoD 13, which `closeout-and-durable-audience` owns)*

## Definition of done (boundary-level)

1. AC-001 through AC-016 are each observed, on the tree that was published, from a
   clean checkout.
2. `0.2.0` resolves from crates.io and the stranger-install smoke (AC-008) has been
   run against it by someone who did not build it.
3. The surface-diff report, the clause-audit report and the rendered-page check are
   each committed artefacts with a date, not a remembered observation.
4. `redkiln validate --kb` and `redkiln doctor` are clean, and `doctor` reports
   exactly the six expected `template-drift` advisories and no `dependency-cycle`.
5. The non-terminal integration gate `cargo xtask ci --fast`
   (`.redkiln/config.yaml:55`) is green at the project's integration stage, **and**
   the full `cargo xtask ci` is green on the publish commit — the two are not
   interchangeable here, because `--fast` drops precisely the docs-under-all-features
   and feature-powerset steps AC-007 depends on.
6. Every story in `_storymap.md` traces to at least one AC above, and every AC above
   is covered by at least one story.
7. Nothing in this project's diff touches a `[FROZEN]` clause, a sibling project's
   adapter, or `happenstance-sync`.

## Dependencies

From `_decomposition.md`'s DAG (`:150`). Rank 3 of 6; the CLI owns the `blocked_by`
and `blocks` frontmatter fields, and this section is the human-readable record of
what it should say.

**Depends on (blocked by)** — all five, a deliberate gate decision recorded at
`_decomposition.md`:214-225 that adds three blockers `RUNBOOK.md` does not require,
because publishing after SQLite alone would publish a one-adapter claim and AC-08
requires the claim to name *which* implementations it was checked against:

- `typed-layer-and-alpha-release` — creates the `0.2.0-alpha.1` registry baseline
  AC-002's entire instrument diffs against. Substrate, not a peer.
- `sqlite-durable-store` — the durable-store evidence AC-009 names.
- `cloudflare-durable-object-store` — the `!Send` / `wasm32` evidence AC-009 and
  AC-014 name. A DAG root; it can run from day one.
- `postgres-and-neon-stores` — the non-serialising and no-cursor evidence AC-009
  names.
- `ladybug-projection-store` — the unlike batch shape and the freeze verdict AC-012
  reads.

**Unlocks (blocks)**

- `replication-identity-and-ingest` — and, transitively,
  `retention-and-incomplete-logs` and `closeout-and-durable-audience`.

**Cost accepted:** roughly 25–30 runbook-days between the alpha and `0.2.0`
(`_decomposition.md`:222-225). The rejected alternative — publish after SQLite with
the projection port behind `unstable-projection` and land the rest in `0.2.x` — is
recorded there as defensible and as making a weaker public claim.

## Risks and coupling notes

| Risk | Likelihood / Impact | Mitigation |
| --- | --- | --- |
| The falsifier ledger is repaired *during* the audit rather than before it, and a row's falsifier is chosen to make the audit pass | Medium / High | DR-4 is a separate, earlier obligation than DR-3. `RUNBOOK.md:625-628` already says adding a row means deciding a falsifier and an owning phase and *"is not a thing to do in passing"* |
| A breaking change ships that nobody could see in review | Medium / High | AC-002's diff is the instrument, and it must be shown to fail on a seeded break — a check that cannot fail is decorative (`CLAUDE.md`) |
| The MSRV promise is made by editing an accepted atom | Medium / High | DR-6 forbids it explicitly; `validate --kb` checks accepted atoms against `HEAD` and would fail the change |
| Positioning goes stale between the decision and the release | Medium / Medium | The nearest live peer shipped the day before intake. DT-6's resolution is re-checked at the publish commit, not at decision time; the initiative's assumptions already require this |
| The "which crates publish" answer arrives late and retroactively expands adapter scope | Medium / High | DR-1 is flagged *decide early* in `_decomposition.md`:302-309: more than three crates means each adapter project owes a name reservation and a README |
| `--fast` is mistaken for the gate at publish | Low / High | DoD item 5 states both explicitly; `.redkiln/config.yaml:50-60` documents what `--fast` drops |
| The audit finds a `[FROZEN]` clause wrong, and it is quietly amended to keep the release date | Low / High | AC-015 makes it an observable property of the diff, and DR-15 makes it a blocker with a decision atom |
| An open question is resolved in passing by the release rather than by decision | Medium / High | AC-013; and the open-questions index requires a question atom be *resolved*, never deleted (`.kb/open-questions/README.md`, indexed at `.kb/maps/open-questions-index.md`) |

**Coupling notes.**

- **The only inbound coupling that is substrate rather than evidence is the alpha
  baseline.** The four adapter projects supply *claims content*; if one slipped, AC-009
  could in principle narrow. The baseline cannot: without it AC-002 has nothing to
  compare against and the project's central instrument does not exist.
- **`nothing-owns-the-post-phase-reconciliation`** — the open question at
  `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` records that no
  phase obliges anyone to read the specification back against the tree, and names
  first publish as its secondary forcing event. AC-003 and AC-005 are that read for
  this release; whether they resolve the atom or merely discharge one instance of it
  is a question for AC-013's decision pass, not to be settled in passing.
- **`spec-trace` is both an input and a subject.** The audit reads what `spec-trace`
  parses. Extending the parser to emit a maturity report changes §7.1/§7.2's
  generated regions, and `xtask/src/spec_trace.rs:39-57` warns that §1.3 must stay
  hand-computed for the reconciliation in AC-003 to mean anything.
- **`cargo xtask package-check` proves containment, not presentation.** AC-007's
  rendered-page check is deliberately a human observation; `xtask/src/package.rs:4-18`
  explains why the metadata cannot stand in for it.

## Context anchors

Backlog:

- [`../initiative.md`](../initiative.md) — BR-05…BR-09, BR-14, BR-15, BR-17;
  AC-03, AC-08…AC-12; DoD 9–12; DT-1, DT-4, DT-5, DT-6
- [`../_decomposition.md`](../_decomposition.md) — the traceability matrix (`:56-113`),
  the DAG (`:142-153`), the gate decisions (`:214-249`) and the two items carried into
  this project's briefs (`:302-318`)
- [`_intake-brief.md`](_intake-brief.md) — the approved problem, constraints, open
  questions, proof artefact and clause list

Knowledge base:

- `.kb/decisions/0004-edition-and-msrv.md:83-86` — the MSRV is a preference until
  first publish and a promise afterward
- `.kb/decisions/0029-msrv-raised-to-1-97-1.md:94-95` — *"phase 12 must revisit the
  floor at first publish"*
- `.kb/decisions/0006-bare-name-to-the-typed-layer.md` — why `happenstance` is the
  crate a consumer installs
- `.kb/decisions/0001-async-port-flavours.md` — the `!Send` flavour AC-014 protects
- `.kb/maps/decision-map.md:30-34, 76-79` — the immutability rule, and the
  amendment-not-supersession shape 0004/0029 already use
- `.kb/maps/open-questions-index.md` — the index a resolved question must be updated on
- `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` — forced
  secondarily by first publish
- `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` — the gap-read rule
  owned by no decision, which DR-9's plain-language promise must not overstate

Specification and plan:

- `spec/SPECIFICATION.md:213-222` — the empty-falsifier prohibition and the census
  AC-003 reconciles against
- `spec/SPECIFICATION.md:224-235` — CF-25's standing qualification on every frozen
  port clause
- `RUNBOOK.md:163` — phase 12's row and its proof artefact
- `RUNBOOK.md:622-635` — the five known-short ledger rows that cannot wait past this
  audit

Code and config:

- `xtask/src/main.rs:8-54` — what the gate proves today, and what it does not
- `xtask/src/package.rs:1-42` — the derived-plus-intention publishable set DR-1 changes
- `xtask/src/spec_trace.rs:23-57` — why §7.1/§7.2 are generated and §1.3 must not be
- `crates/happenstance/src/lib.rs` — the crate a consumer installs
- `.redkiln/config.yaml:50-60` — `--fast` for a non-terminal project, the whole gate
  for the terminal one
- `.redkiln/templates/_design.md` — the repurposed design stage where DT-1/4/5/6 are
  resolved

## Companions

- [`_intake-brief.md`](_intake-brief.md) — the approved intake for this project
- [`_storymap.md`](_storymap.md) — the vertical-slice story map covering AC-001…AC-016
- [`../_decomposition.md`](../_decomposition.md) — the initiative decomposition: the
  DAG, the traceability matrix and the scope seams this card restates
- [`../initiative.md`](../initiative.md) — the parent charter
- [`../_plan.md`](../_plan.md) — the initiative-wide plan rollup

The `ux`, `testing` and `deployment` briefs warranted for this project
(`../_decomposition.md`:261) are authored alongside this card by `plan-briefs`; no
`architecture` brief is warranted, deliberately.
