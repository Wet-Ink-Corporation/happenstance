---
id: kb-map-decision-001
title: Decision map
kind: map
status: accepted
authority_tier: note
summary: >-
  One row per decision atom in .kb/decisions/, its ADR number, status, phase, and what it
  supersedes or is superseded by. First populated by the 2026-08-10 ADR import (ADR-0001 through
  ADR-0016 and ADR-0029, seventeen atoms), which is also this map's first wave. Updated by the
  Maps phase of every kb-ingest wave that lands a new or superseded decision atom. The 2026-08-13
  wave added ADR-0017 through ADR-0019 (kb-decision-0017/0018/0019), phase 6's ProjectionStore
  freeze — no existing row's status or supersession cell changed. The 2026-08-15 wave added
  ADR-0030 (kb-decision-0030), which mints PS-38 rather than superseding any row on this map;
  `supersedes: null` on the new atom is confirmed against the ADR's own "amends nothing, edits
  nothing" bullet. A second, later 2026-08-15 wave (`2026-08-15-intake`) added ADR-0020 and
  ADR-0021 (kb-decision-0020, kb-decision-0021), phase 7's typed-layer decisions — the first
  phase-7 rows on this map — in their own section rather than the ADR-0030 section, since the two
  waves share a date but not a subject; neither atom supersedes any existing row. The 2026-08-17
  wave (`2026-08-17-adr-0022-append-condition`) added ADR-0022 (phase 8) and ADR-0031, ADR-0032 and
  ADR-0033 (phase 7) in one section spanning both phases. ADR-0031 partly supersedes ADR-0007 — the
  first amendment this map has annotated on a row that was itself already annotated as a partial
  supersession — and ADR-0032 fully supersedes ADR-0021, flipping its row to superseded.
  kb-decision-0007's own frontmatter stays `accepted` with `superseded_by: null`, unflipped by this
  map edit; kb-decision-0021's frontmatter was flipped by the ingest wave itself, and this map only
  mirrors it. The 2026-08-20 wave (`2026-08-20-intake-phase-9`) added ADR-0023 (phase 9, the
  Cloudflare `SqlStorage` mapping and its off-tokio harness) and ADR-0034 (phase 9, the fixture
  contract has no single owning document) in one section. Neither supersedes any row on this map —
  both `supersedes: null` — and both are phase 9, the first phase-9 rows the map carries. The
  2026-09-02 wave (`2026-09-02-intake`) added two more sections. The first carries ADR-0035 (phase
  9, exempting `worker`/`worker-macros` from the `async-trait` ban by name) and ADR-0036 (phase 6,
  evaluating PS-2's freeze bar against real adapters and declining to freeze `ProjectionStore` at
  `0.2.0`); neither supersedes any row. The second is this map's first non-ADR section: three
  brand-identity decisions numbered `SD-` rather than `ADR-`, a parallel record series that sits
  outside the ADR sequence on purpose (`kb-reference-brand-source-locations-001` states why) and
  carries `phase: null`, since brand work is not one of RUNBOOK.md's fifteen phases. `SD-0002` is
  shared by two atoms — the mark's own construction and binding rules, and a wider standalone-SVG
  delivery rule split out because its scope reaches past the mark — and the table's `SD` column
  says so rather than collapsing them into one row. The 2026-09-04 wave (`2026-09-04-intake`) added
  ADR-0037 (phase 12), extending the amendment lineage `kb-decision-0004` → `kb-decision-0029`
  by one more link: `0.2.0` turns the 1.97.1 floor from a self-imposed constraint into a promise a
  consumer relies on, without moving the number or superseding either atom. Both `kb-decision-0004`
  and `kb-decision-0029` stay `accepted` and byte-identical — this map's rows for both now carry a
  second "amended by" annotation rather than a flip, the same shape ADR-0029 used against ADR-0004.
depends_on: []
related:
  - kb-map-domain-001
  - kb-map-open-questions-index-001
source_paths:
  - .kb/decisions/README.md
  - .kb/_governance/integration-waves/2026-08-10-intake-2
  - .kb/_governance/integration-waves/2026-08-13-projection-adrs
  - .kb/_governance/integration-waves/2026-08-15-adr-0030-checkpoint-progress
  - .kb/_governance/integration-waves/2026-08-15-intake
  - .kb/_governance/integration-waves/2026-08-17-adr-0022-append-condition
  - .kb/_governance/integration-waves/2026-08-20-intake-phase-9
  - .kb/_governance/integration-waves/2026-09-02-intake
  - .kb/_governance/integration-waves/2026-09-04-intake
last_reviewed: 2026-09-04
---

# Decision map

The corpus's decision index. Where [`../decisions/`](../decisions) holds the atoms themselves,
this map is the one place that shows the whole supersession graph at a glance — which decisions
stand, which were reversed in whole or in part, and by what. See [`README.md`](README.md) for
what belongs on a map atom and what does not; see [`domain-map.md`](domain-map.md) for the same
atoms grouped by subject instead of by lineage.

The immutability rule lives in [`../decisions/README.md`](../decisions/README.md): an accepted
decision's body is never edited, and a correction is either a repair (unchanged in place, because
the set of implementations it admits is unchanged) or a new decision carrying `supersedes`, with
the old atom's frontmatter flipped to `status: superseded` + `superseded_by`. This map reflects
that flip on both sides of every supersession row below.

## 2026-08-10 ADR import (ADR-0001–0016, ADR-0029)

Seventeen decision atoms, phases 0–5, imported from `.kb/decisions/`. Three are partial
supersessions — a decision atom that keeps `status: accepted` because one half of its claim
still binds, corrected in place by a later decision that displaces only the other half. See
[`../governance/rewrite-the-referent-never-the-reasoning.md`](../governance/rewrite-the-referent-never-the-reasoning.md)
for the discrimination this corpus draws between a repair and a reversal, and
[`../playbooks/one-decision-per-adr-title.md`](../playbooks/one-decision-per-adr-title.md) for
why three of these titles carry an "and" that turned out to be two decisions of different
strength.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0001 | [`kb-decision-0001`](../decisions/0001-async-port-flavours.md) | Async ports in two flavours, Send and !Send | accepted | 0 | — |
| ADR-0002 | [`kb-decision-0002`](../decisions/0002-crate-naming.md) | Prefixed crate names, with a parallel claim on eventum | **superseded** | 0 | superseded by `kb-decision-0005` |
| ADR-0003 | [`kb-decision-0003`](../decisions/0003-opaque-payloads.md) | Opaque payloads in the contract crate | accepted (provisional) | 0 | — |
| ADR-0004 | [`kb-decision-0004`](../decisions/0004-edition-and-msrv.md) | Rust 2024 edition, MSRV 1.85 | accepted (amended, not superseded) | 0 | amended by `kb-decision-0029`, `kb-decision-0037` |
| ADR-0005 | [`kb-decision-0005`](../decisions/0005-rename-to-happenstance.md) | Rename the project to happenstance, and make it the contract crate | accepted (partly superseded) | 0 | supersedes `kb-decision-0002`; partly superseded by `kb-decision-0006` |
| ADR-0006 | [`kb-decision-0006`](../decisions/0006-bare-name-to-the-typed-layer.md) | The bare name goes to the typed layer; the contract becomes happenstance-core | accepted (partly superseded) | 0 | partly superseded by `kb-decision-0007` |
| ADR-0007 | [`kb-decision-0007`](../decisions/0007-projection-runner-decodes.md) | The projection runner decodes, and therefore splits across the seam | accepted (partly superseded) | 0 | partly supersedes `kb-decision-0006`; partly superseded by `kb-decision-0031` |
| ADR-0008 | [`kb-decision-0008`](../decisions/0008-one-derivation-for-both-ports.md) | One derivation scheme, both ports, and what a provided body owes | accepted | 1 | — |
| ADR-0009 | [`kb-decision-0009`](../decisions/0009-error-send-sync.md) | Error stays unbounded, and the strength goes in a marker | accepted | 2 | — |
| ADR-0010 | [`kb-decision-0010`](../decisions/0010-the-suite-must-prove-itself.md) | The conformance suite's own proof obligation | accepted | 3 | — |
| ADR-0011 | [`kb-decision-0011`](../decisions/0011-read-laziness-and-isolation.md) | A read is one sample with a ceiling, and &Query stays | accepted | 4 | — |
| ADR-0012 | [`kb-decision-0012`](../decisions/0012-append-shape-and-preconditions.md) | append keeps its borrowed batch, and phase 4 declines what it cannot measure | accepted | 4 | — |
| ADR-0013 | [`kb-decision-0013`](../decisions/0013-position-assignment-and-visibility.md) | Positions are assigned once and become visible in order | accepted | 4 | — |
| ADR-0014 | [`kb-decision-0014`](../decisions/0014-event-identity-and-recorded-time.md) | The store mints identity, records a time, and the caller supplies neither | accepted | 4 | — |
| ADR-0015 | [`kb-decision-0015`](../decisions/0015-validated-identifiers-and-store-limits.md) | Validated identifiers, byte equality, and the two kinds of bound | accepted | 4 | — |
| ADR-0016 | [`kb-decision-0016`](../decisions/0016-the-wire-format.md) | The wire format is happenstance's own, and an unknown version is refused before the message is read | accepted | 5 | — |
| ADR-0029 | [`kb-decision-0029`](../decisions/0029-msrv-raised-to-1-97-1.md) | The MSRV is 1.97.1 | accepted | 2 | amends `kb-decision-0004`; amended by `kb-decision-0037` |

## 2026-08-13 projection-store ADRs (ADR-0017–0019)

Three decision atoms, phase 6, `.kb/decisions/`. None supersedes an existing row: ADR-0017
settles what `ProjectionStore::Batch` owns, ADR-0018 settles resetting a checkpoint to
`NeverRun`, and ADR-0019 settles what the port does — nothing — when a projection's `apply`
fails. All three `depends_on: [kb-decision-0007]`, ADR-0007's runner/decode split; ADR-0018 and
ADR-0019 additionally `depends_on: [kb-decision-0017]`, since both use the `ProjectionProbe` and
owned-`Batch` shape ADR-0017 mints. Each carries two provisional halves with a named falsifier —
see the atom's own `## Provisional` section, not this row.

The Status column reads `accepted` for all three, exactly as the seventeen rows above it and
exactly as each atom's `status:` field. It deliberately does **not** read "accepted
(provisional)": that value exists in no `KbFrontmatter` enum, and printing it here would be this
map quietly answering
[`kb-open-question-adr-status-vocabulary-001`](../open-questions/adr-status-vocabulary-exceeds-the-schema.md)
by acting on it. The convention that question records is the one followed — the qualification and
its falsifier live in the first clause of each atom's `summary`, and at length under its
`## Provisional` heading.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0017 | [`kb-decision-0017`](../decisions/0017-what-a-projection-batch-owns.md) | What a projection batch owns, and the seam that is not a write vocabulary | accepted | 6 | — |
| ADR-0018 | [`kb-decision-0018`](../decisions/0018-returning-a-projection-to-never-run.md) | Returning a projection to never run — scope, atomicity, and refusal | accepted | 6 | — |
| ADR-0019 | [`kb-decision-0019`](../decisions/0019-what-happens-when-apply-fails.md) | What happens when apply fails — the port grows nothing | accepted | 6 | — |

## 2026-08-15 checkpoint-progress ADR (ADR-0030)

One decision atom, phase 6, `.kb/decisions/`. ADR-0030 mints `[PROVISIONAL]` clause PS-38 in
§4.7 — a successful `commit` MUST advance its `ProjectionId`'s checkpoint, and an id no successful
`commit` has named MUST read as `Checkpoint::NeverRun` — closing the progress-obligation gap that
`kb-open-question-ps-1-no-progress-obligation-001` and `kb-open-question-ps-19-scope-narrower-001`
recorded and the 2026-08-13 clause-pairing sweep confirmed. It `depends_on` `kb-decision-0007`
(the runner/decode split), `kb-decision-0017` (the owned-`Batch` shape) and `kb-decision-0018`
(the `ProjectionProbe` `reset` mechanism); it does not supersede any row on this map — PS-1, PS-19,
PS-21 and PS-22 stay byte-identical, each gaining a recorded finding rather than a changed
sentence. Both amended open-question atoms are now `status: superseded`, annotated in place; see
[`open-questions-index.md`](open-questions-index.md).

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0030 | [`kb-decision-0030`](../decisions/0030-the-checkpoint-reports-the-commits-that-happened.md) | The checkpoint reports the commits that happened | accepted | 6 | — |

## 2026-08-15 typed-layer ADRs (ADR-0020, ADR-0021)

Two decision atoms, phase 7, `.kb/decisions/` — the second `kb-ingest` wave to land on 2026-08-15
(`2026-08-15-intake`), and given its own section rather than rows appended to the
`## 2026-08-15 checkpoint-progress ADR (ADR-0030)` section above: the two waves share a date, not
a subject, and the map's own *Adding a row* rule opens a new section per wave, not per day. ADR-0020
settles how a `DecisionModel`'s query is derived rather than hand-maintained; ADR-0021 settles
where the codec tag lives in `Event::metadata` and how payload evolution is handled at decode.
Neither supersedes any row on this map — both `supersedes: null` — and both are phase 7, the
first phase-7 rows the map carries; the map's ADR numbering is already non-monotonic across
sections (ADR-0029 sits inside the 2026-08-10 import, ADR-0030 precedes these two here), which the
*Adding a row* rule makes correct rather than untidy. The two atoms carry a mutual `related` edge
to each other, wired by both atoms at authoring time rather than by this map: ADR-0021's per-type
query-naming cost is the tax ADR-0020's derived `Boundary::query` already collapsed to one place.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0020 | [`kb-decision-0020`](../decisions/0020-fold-query-agreement.md) | A decision model folds a domain enum, and its query is derived on a sealed trait the caller cannot override | accepted | 7 | — |
| ADR-0021 | [`kb-decision-0021`](../decisions/0021-payload-evolution-and-codec-tag.md) | The codec tag lives in Event::metadata, event types do not carry versions, and upcasting happens at decode | **superseded** | 7 | superseded by `kb-decision-0032` |

## 2026-08-17 append-condition, runner-collapse, and macros-scope ADRs (ADR-0022, ADR-0031, ADR-0032, ADR-0033)

Four decision atoms, one wave (`2026-08-17-adr-0022-append-condition`), `.kb/decisions/`, spanning
phase 7 and phase 8 — the wave is a single ingest, not a single phase, so it gets one section per
this map's own *Adding a row* convention. ADR-0022 settles `happenstance-sqlite`'s append-condition
SQL strategy and tag storage, phase 8's first real measurement against SQLite. ADR-0031 fires
ADR-0007's own falsifier — no checkpoint pump exists in `happenstance-core` after phase 7 — and
partly supersedes it: only the allocation of a runnable pump moves, while ADR-0007's discriminator
and all three shape decisions stand, so `kb-decision-0007` keeps `status: accepted` with
`superseded_by: null` and this map's ADR-0007 row is annotated rather than flipped, the same
treatment `kb-decision-0006`'s row already carries. ADR-0032 fully supersedes ADR-0021: a repair
that withdraws one incorrect justification (a backwards reading of ADR-0003) for an already-correct
rejection, leaving all three of ADR-0021's decisions intact — full rather than partial, because the
defect was in ADR-0021's own body rather than in an allocation one of its parts made. ADR-0033
records `happenstance-macros` out of scope for 0.1 against a measured ceremony ratio; it supersedes
nothing, since ADR-0020 published its own contrary prediction as explicitly falsifiable.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0022 | [`kb-decision-0022`](../decisions/0022-append-condition-strategy.md) | The append condition is a max(position) guard inside BEGIN IMMEDIATE, and tags live in a join table keyed (tag, position) | accepted | 8 | — |
| ADR-0031 | [`kb-decision-0031`](../decisions/0031-the-runner-collapses-upward.md) | One runner, in happenstance — the checkpoint pump collapses upward | accepted | 7 | partly supersedes `kb-decision-0007` |
| ADR-0032 | [`kb-decision-0032`](../decisions/0032-adr-0021-serde-attribution-correction.md) | The serde-encoded framing region is rejected on two grounds, and ADR-0003 was never one of them | accepted | 7 | supersedes `kb-decision-0021` |
| ADR-0033 | [`kb-decision-0033`](../decisions/0033-happenstance-macros-out-of-scope-for-0-1.md) | happenstance-macros is out of scope for 0.1 | accepted | 7 | — |

### Reading the partial-supersession chain

`kb-decision-0005` → `kb-decision-0006` → `kb-decision-0007` is one lineage, not three
independent decisions: 0005 bundled a rename with a crate-allocation choice, 0006 kept the naming
half and reversed the allocation half, and 0007 kept 0006's naming discriminator and reversed
only the projection-runner placement. Each later atom's `depends_on` names the one it corrects;
none of the three is `status: superseded` in full, because each still has a half standing. Only
`kb-decision-0002` is superseded outright, by `kb-decision-0005`.

`kb-decision-0031` extends this lineage by one more link, on a different axis: it does not touch
0006's naming discriminator at all, only 0007's allocation of a runnable checkpoint pump to
`happenstance-core`. `kb-decision-0007` therefore now carries two annotations rather than one —
"partly supersedes `kb-decision-0006`" and "partly superseded by `kb-decision-0031`" — and stays
`status: accepted` throughout, for the same reason 0006 does: a status flip would retire shape
decisions that are still standing and implemented, one of which `kb-decision-0030` itself depends
on.

`kb-decision-0004` and `kb-decision-0029` are the other shape: an amendment, not a supersession.
0004 stays `status: accepted` because its reasoning (the floor is a preference until first
publish) is what 0029 acted on, not what it reversed — `superseded_by` is deliberately left
`null` on 0004, and 0029's `depends_on` carries the edge instead.

## 2026-08-20 phase-9 wave (ADR-0023, ADR-0034)

Two decision atoms, one wave (`2026-08-20-intake-phase-9`), phase 9, `.kb/decisions/`. ADR-0023
settles how an event store maps onto a Cloudflare Durable Object's `SqlStorage` and what harness
proves it, merging the SqlStorage-mapping intake with the ES-6 verdict as one body of evidence per
`kb-playbook-one-decision-per-adr-title-001`'s stated exception rather than as two atoms. ADR-0034
answers `kb-open-question-cf-40-ownership-001` from outside, without editing any of the three
accepted decisions it cites — ADR-0015 (CF-40's clause home), ADR-0012 (CF-39 and
`MID_BATCH_FAULT`) and ADR-0022 (a non-verdict naming a fixture-contract owner) — establishing that
the fixture contract has no single owning document. Neither atom supersedes any row on this map.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0023 | [`kb-decision-0023`](../decisions/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md) | The SqlStorage mapping and the off-tokio harness, settled by one body of evidence | accepted | 9 | — |
| ADR-0034 | [`kb-decision-0034`](../decisions/0034-the-fixture-contract-has-no-single-owner.md) | The fixture contract has no single owning document, and CF-40 is ADR-0015's clause | accepted | 9 | — |

## 2026-09-02 intake: async-trait exemption and the projection port (ADR-0035, ADR-0036)

Two decision atoms, one wave (`2026-09-02-intake`), `.kb/decisions/`, spanning phase 9 and phase
6 — one section per this map's own *Adding a row* convention, since it is one wave rather than one
phase. ADR-0035 resolves `kb-open-question-worker-async-trait-ban-001`: `deny.toml`'s
`async-trait` ban gains two `wrappers` entries, `worker` and `worker-macros`, amending
`kb-decision-0001`'s exemption set from outside rather than editing its body — the same shape
`kb-decision-0029` used against `kb-decision-0004`. ADR-0036 is PS-3's `SHOULD` evaluated against
PS-2's `[FROZEN]` two-part bar, part by part, against the real adapter set for the first time:
part 1 (a hostile store must fail the suite) is met, part 2 (two adapters at opposite ends of the
batch-shape axis must pass) is not, so `ProjectionStore` is **not** frozen at `0.2.0` and ships
behind the off-by-default `unstable-projection` feature instead. Neither atom supersedes any row
on this map — both `supersedes: null`.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0035 | [`kb-decision-0035`](../decisions/0035-async-trait-through-worker.md) | async-trait is exempted where it is reached through worker | accepted | 9 | — |
| ADR-0036 | [`kb-decision-0036`](../decisions/0036-the-projection-port-ships-gated.md) | The projection port is not frozen at 0.2.0 and ships behind unstable-projection | accepted | 6 | — |

## 2026-09-02 intake: brand identity decisions (SD-0001, SD-0002)

Three decision atoms, the same wave (`2026-09-02-intake`), `.kb/decisions/`, given their own
section rather than folded into the ADR-0035/ADR-0036 section above: the two halves of the wave
share a date and an ingest run, not a subject, and this half uses a different numbering sequence
entirely. `SD-` records are brand-identity decisions that sit deliberately outside the `ADR-`
sequence in `references/adr/` — `kb-reference-brand-source-locations-001` states why — and carry
`phase: null` rather than a number from RUNBOOK.md's fifteen phases, since brand work is not one
of them. `SD-0001` (`kb-decision-sd-0001`) settles what the name "happenstance" means and the
five rules that places on copy; it does not supersede ADR-0005 (`kb-decision-0005`), which stays
correct about *why* the rename happened; this decision only assigns a meaning after the fact and
says so. `SD-0002` is shared by two atoms, split because their scopes differ: `kb-decision-sd-0002`
carries the mark's own construction (seven equal blocks, the wordmark, the eight binding rules),
and `kb-decision-standalone-svg-one-colourway-001` carries a wider delivery rule — a standalone SVG
ships one fixed colour, with surface selection left to the point of use — that generalises past
the mark to any SVG this repository ships for a surface it does not control. None of the three
supersedes any row on this map.

| SD | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| SD-0001 | [`kb-decision-sd-0001`](../decisions/sd-0001-the-name-names-the-boundary-of-what-occurred.md) | "Happenstance" names the boundary drawn by what occurred, and what that places on copy | accepted | — | — |
| SD-0002 | [`kb-decision-standalone-svg-one-colourway-001`](../decisions/standalone-svg-carries-one-colourway.md) | A standalone SVG carries one fixed colour, and the surface is selected at the point of use | accepted | — | — |
| SD-0002 | [`kb-decision-sd-0002`](../decisions/sd-0002-the-mark-and-the-rules-that-bind-it.md) | Seven equal blocks and a lowercase wordmark, and the rules that bind anything carrying the name | accepted | — | — |

## 2026-09-04 intake: the MSRV becomes a promise (ADR-0037)

One decision atom, phase 12, `.kb/decisions/`, from the pre-publication review wave
(`2026-09-04-intake`). ADR-0037 extends the amendment lineage `kb-decision-0004` →
`kb-decision-0029` by one more link, on the same axis ADR-0029 used against ADR-0004: `0.2.0`
publishes four crates — `happenstance-core`, `happenstance`, `happenstance-testkit` and
`happenstance-sqlite` — from which moment `1.97.1` binds a consumer who may never build the crate
that forced it. The number does not move and neither `kb-decision-0004` nor `kb-decision-0029` is
superseded; both stay `accepted` and byte-identical, each gaining a second "amended by" annotation
on this map rather than a status flip, the same treatment `kb-decision-0007`'s row carries for two
separate partial supersessions.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0037 | [`kb-decision-0037`](../decisions/0037-msrv-becomes-a-promise-at-0-2-0.md) | The MSRV becomes a promise at 0.2.0, and the number does not move | accepted | 12 | amends `kb-decision-0004`, `kb-decision-0029` |

## Adding a row

A new decision atom gets a row in ADR-number order under the wave section that introduced it
(open a new `##` section per wave, the way [`domain-map.md`](domain-map.md) opens a new section
per domain). A supersession updates two rows in the same edit: the new atom's row, and the old
atom's `Status` and `Supersedes / superseded by` cells — never delete a superseded row.
