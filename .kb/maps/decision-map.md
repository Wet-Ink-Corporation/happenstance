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
  The 2026-09-07 wave (`2026-09-07-intake`) added twenty-three decision atoms in one ingest:
  ADR-0024 (phase 10, the reserved position-visibility-mechanism number, finally filled), ADR-0038
  (phase 10, `async-trait` exempted through `testcontainers`/`tonic`), ADR-0040 (phase 10, the
  same-day F2-5 hold and its discharge), and twenty phase-12 pre-publication ratifications
  (ADR-0039, ADR-0041 through ADR-0059) drawn from the `remediation-2026-09-04-briefs` queue and
  ratified 2026-09-06. None supersedes an existing row — every one of the twenty-three carries
  `supersedes: null` — so this wave adds rows only; see each section below for what each decision
  amends from outside without a status flip, the same shape ADR-0029, ADR-0031 and ADR-0035 already
  used against ADR-0004, ADR-0007 and ADR-0001. The 2026-09-09 wave (`2026-09-09-intake`) added
  three more: ADR-0025 (phase 11, the reserved number RUNBOOK.md held for the Ladybug projection
  adapter, finally filled — a checkpoint node in the graph, raw Cypher, a blocking driver), ADR-0060
  (phase 11, re-evaluating PS-2's `[FROZEN]` batch-shape bar against four adapters instead of one and
  keeping the gate on a replaced reason), and ADR-0061 (phase 10, ES-11's asynchronous-driver
  sufficiency condition narrowed after `happenstance-neon` falsified it). None of the three
  supersedes a row on this map — all three carry `supersedes: null` — and none is an amendment to an
  existing atom's body either: ADR-0060 reaffirms `kb-decision-0036`'s verdict on a replaced reason
  without editing or superseding it, the fourth instance of that shape this map records (after
  `kb-decision-0004`/`kb-decision-0029`, `kb-decision-0006`/`kb-decision-0007`, and
  `kb-decision-0007`/`kb-decision-0031`), and ADR-0061 repairs `spec/SPECIFICATION.md`'s ES-11 clause
  itself under `kb-playbook-repair-frozen-clause-001`'s test rather than touching any decision atom.
  The 2026-09-11 wave (`2026-09-11-intake`) added three more: ADR-0062 (phase 12, the projection port's
  probe seam and `begin` move to `&mut Self::Batch` / async / fallible, and `LivePostgresProjectionStore`
  runs 20 of 20 against a live PostgreSQL, meeting PS-2's MUST as written with no axis change), ADR-0063
  (phase 12, freezing `ProjectionStore` on that evidence while keeping `happenstance-core`'s
  `unstable-projection` feature declared and empty and narrowing `happenstance`'s feature of the same
  name to the runner alone, since `Projection::apply` is still synchronous), and ADR-0064 (phase null,
  the dedicated measurement host's conditions are declared data rather than ritual, and its preflight is
  structurally unreachable from any merge-blocking job). None of the three supersedes a row on this map.
  ADR-0063 discharges ADR-0036 and ADR-0060 rather than superseding either — both said gate the port
  until PS-2's bar was met, both were correct when written, and ADR-0062 met the bar — the same shape
  this map already records for ADR-0060 reaffirming ADR-0036 above, so neither row is flipped or
  annotated. The 2026-09-21 wave (`2026-09-21-intake`) added one more: ADR-0065, partly superseding
  ADR-0022 at section 11 only. Section 11 fixed `busy_timeout` at 5,000 ms on a premise —
  `busy = 0` in every row of the 64-contender table — and named its own reopen condition as
  `busy > 0`; `kb-reference-busy-timeout-margin-001` measured `busy > 0` at the shipped
  `CONTENDERS = 64`, and the condition fired. ADR-0065 sets `BUSY_TIMEOUT_MS` to 15,000, measured
  against the shipped adapter itself (`kb-reference-busy-timeout-adapter-cap-sweep-001`) rather
  than the experiment's candidate. `supersedes: null` on the new atom and `status: accepted`
  unflipped on `kb-decision-0022` — sections 4, 6, 7, 9, 10, 12 and 15 stand untouched — the same
  partial-supersession shape `kb-decision-0031` used against ADR-0007 and `kb-decision-0006` used
  against ADR-0005, both already recorded in *Reading the partial-supersession chain* below.
  The 2026-09-28 pass, the first written by hand since redkiln's retirement, added six Weigh-In
  decision atoms (kb-decision-wi-016abe, -052920, -38373d, -40b321, -ab0a5a, -b9b9ab) in their own
  section; none carries an ADR number or supersedes a row.
  Phase 16's 2026-09-29 pass, also by hand, added ADR-0066 through ADR-0071 (kb-decision-0066 to
  -0071, all phase 16) in their own section. None carries `supersedes`: ADR-0068 partly supersedes
  ADR-0022 at §8 items 1–2 and §16's falsifier for §8, the ADR-0065 shape, so kb-decision-0022's row
  gains a second partial-supersession annotation and stays accepted; ADR-0067 amends
  kb-decision-0037 rather than superseding it, the fourth link in the ADR-0004 lineage.
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
  - .kb/_governance/integration-waves/2026-09-07-intake
  - .kb/_governance/integration-waves/2026-09-09-intake
  - .kb/_governance/integration-waves/2026-09-11-intake
  - .kb/_governance/integration-waves/2026-09-21-intake
last_reviewed: 2026-09-29
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
| ADR-0022 | [`kb-decision-0022`](../decisions/0022-append-condition-strategy.md) | The append condition is a max(position) guard inside BEGIN IMMEDIATE, and tags live in a join table keyed (tag, position) | accepted (partly superseded) | 8 | partly superseded by `kb-decision-0065`, `kb-decision-0068` |
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

`kb-decision-0022` → `kb-decision-0065` is the same partial shape at section grain rather than
document grain: ADR-0065 touches only ADR-0022's §11 (the `busy_timeout` value), leaving §4, §6,
§7, §9, §10, §12 and §15 standing, so `kb-decision-0022` keeps `status: accepted` with
`superseded_by: null` and this map's row is annotated rather than flipped — see
[`../open-questions/adr-status-vocabulary-exceeds-the-schema.md`](../open-questions/adr-status-vocabulary-exceeds-the-schema.md)
for why the convention scaling down to one section of fifteen, rather than half a document, is
itself live evidence in that open question.

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
| ADR-0037 | [`kb-decision-0037`](../decisions/0037-msrv-becomes-a-promise-at-0-2-0.md) | The MSRV becomes a promise at 0.2.0, and the number does not move | accepted | 12 | amends `kb-decision-0004`, `kb-decision-0029`; amended by `kb-decision-0067` |

## 2026-09-07 intake: phase 10 (ADR-0024, ADR-0038, ADR-0040)

Three decision atoms, one wave (`2026-09-07-intake`), phase 10, `.kb/decisions/`. ADR-0024 fills
the number RUNBOOK.md had reserved for it and settles how `happenstance-postgres` buys position
visibility — `xid8` plus `pg_snapshot_xmin` rather than a serialised sequence table — `depends_on`
`kb-decision-0013`, the global visibility invariant it implements without reopening. ADR-0038
exempts `testcontainers` and `tonic` from `deny.toml`'s `async-trait` ban by name, amending
`kb-decision-0001`'s exemption set from outside for the second time (ADR-0035 was the first,
for `worker`); neither `kb-decision-0001` nor `kb-decision-0035` is flipped, the same treatment
their own rows already carry. ADR-0040 records that `0.2.0` waited for phase 10 on a stale
`RUNBOOK.md` status row, that the hold's mutation-coverage half was measured false the same day,
and that the real residual — no real-medium adapter — closed hours later when `happenstance-postgres`
became real. None of the three supersedes any row on this map.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0024 | [`kb-decision-0024`](../decisions/0024-position-visibility-mechanism.md) | happenstance-postgres buys position visibility with xid8 and pg_snapshot_xmin | accepted | 10 | — |
| ADR-0038 | [`kb-decision-0038`](../decisions/0038-async-trait-through-testcontainers.md) | async-trait is exempted where it is reached through testcontainers or tonic | accepted | 10 | — |
| ADR-0040 | [`kb-decision-0040`](../decisions/0040-f2-5-holds-the-release-for-phase-10.md) | 0.2.0 waited for phase 10, and the hold was lifted the same day | accepted | 10 | — |

## 2026-09-07 intake: pre-publication ratifications, ports and testkit half (ADR-0039, ADR-0042, ADR-0043, ADR-0048, ADR-0050–ADR-0056, ADR-0058)

Twelve decision atoms, one wave (`2026-09-07-intake`), phase 12, `.kb/decisions/`, drawn from the
`remediation-2026-09-04-briefs` queue and ratified 2026-09-06. Grouped here because each settles a
question about `happenstance-core`'s ports, the conformance suite, or an adapter, rather than the
typed layer or publication process — see the two sections below for those. ADR-0039 freezes ES-42
(`read`'s return carries no `+Unpin` bound) at the deadline its own `[PROVISIONAL]` marker named.
ADR-0042 retracts the fixture contract's growing-required-item hazard: every future capability
lands defaulted, and honesty moves to a clause-level MUST per capability rather than a trait-level
one, `depends_on` `kb-decision-0034`. ADR-0043 mints a refusal channel for `Event::metadata` with a
guaranteed minimum of zero, amending `kb-decision-0015` from outside rather than widening it.
ADR-0048 marks `Op::Read` `#[non_exhaustive]` in the same release that already spent the breaking
change adding its `to` field, and adds no constructor. ADR-0050 narrows `StringifiedThrow` to
`pub(crate)`. ADR-0051 discharges CF-18 by declension-by-inheritance rather than the literal,
unconstructible rule the ratified costing described. ADR-0052 keeps the query-partition width
constants public pending a `workerd`-class measurement. ADR-0053 bounds a read page in both rows
and bytes, `depends_on` `kb-decision-0011`. ADR-0054 pins `after_opt`'s behaviour across every
guard with two unit tests and a corrected doc block, `depends_on` `kb-decision-0012`. ADR-0055
keeps `append`'s borrowed batch at `0.2.0`, restating ES-17's falsifier rather than reallocating the
signature. ADR-0056 repairs WF-10's `Rule:` line with two `decode_rejects` tests and an exhaustive
`Serialize` destructuring, a repair rather than an amendment per
[`kb-playbook-repair-frozen-clause-001`](../playbooks/repairing-a-frozen-clause-without-amending-it.md).
ADR-0058 keeps `happenstance-sqlite`'s write path inline and documents why, `depends_on`
`kb-decision-0022`. None of the twelve supersedes any row on this map.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0039 | [`kb-decision-0039`](../decisions/0039-es-42-frozen-without-an-unpin-bound.md) | ES-42 freezes without a Unpin bound, at the deadline the marker named | accepted | 12 | — |
| ADR-0042 | [`kb-decision-0042`](../decisions/0042-every-fixture-capability-lands-defaulted.md) | Every future fixture capability lands defaulted, and honesty moves to a clause | accepted | 12 | — |
| ADR-0043 | [`kb-decision-0043`](../decisions/0043-event-metadata-gets-a-refusal-channel-with-no-floor.md) | Event::metadata gets a refusal channel and no guaranteed capacity | accepted | 12 | — |
| ADR-0048 | [`kb-decision-0048`](../decisions/0048-op-read-is-non-exhaustive.md) | Op::Read is non-exhaustive, in the release that already spent the break | accepted | 12 | — |
| ADR-0050 | [`kb-decision-0050`](../decisions/0050-stringified-throw-is-crate-private.md) | StringifiedThrow is crate-private, and narrowing it made the compiler look | accepted | 12 | — |
| ADR-0051 | [`kb-decision-0051`](../decisions/0051-declension-by-inheritance-discharges-cf-18.md) | Declension-by-inheritance discharges CF-18 | accepted | 12 | — |
| ADR-0052 | [`kb-decision-0052`](../decisions/0052-the-query-partition-constants-stay-public.md) | The query-partition constants stay public | accepted | 12 | — |
| ADR-0053 | [`kb-decision-0053`](../decisions/0053-a-read-page-is-bounded-in-rows-and-in-bytes.md) | A read page is bounded in rows and in bytes | accepted | 12 | — |
| ADR-0054 | [`kb-decision-0054`](../decisions/0054-after-opt-applies-to-every-guard-and-says-so.md) | after_opt applies to every guard and says so | accepted | 12 | — |
| ADR-0055 | [`kb-decision-0055`](../decisions/0055-append-keeps-its-borrowed-batch-at-0-2-0.md) | append keeps its borrowed batch at 0.2.0 | accepted | 12 | — |
| ADR-0056 | [`kb-decision-0056`](../decisions/0056-wf-10s-rule-line-is-repaired-not-amended.md) | WF-10's Rule line is repaired, not amended | accepted | 12 | — |
| ADR-0058 | [`kb-decision-0058`](../decisions/0058-the-sqlite-write-path-stays-inline-and-documents-it.md) | The sqlite write path stays inline and documents it | accepted | 12 | — |

## 2026-09-07 intake: pre-publication ratifications, typed-layer half (ADR-0046, ADR-0047, ADR-0049, ADR-0059)

Four decision atoms, the same wave, phase 12, settling `happenstance` (the typed layer) rather than
the port. ADR-0046 gives `commit` a nothing-to-do outcome distinct from a refused one, `depends_on`
`kb-decision-0012`, `kb-decision-0030` and `kb-decision-0031`. ADR-0047 refuses an under-tagged
model at commit rather than admitting it silently, `depends_on` `kb-decision-0020` and landing in
the same commit as ADR-0046. ADR-0049 gives a codec a declared, narrower-than-it-sounds
`reads_tag` obligation, `depends_on` `kb-decision-0032`. ADR-0059 has the domain-event guard check
positional agreement between a fold and its query, `depends_on` `kb-decision-0020` and
`kb-decision-0032`. None supersedes any row on this map; see
[`domain-map.md`](domain-map.md#the-typed-layer-decision-models-codecs-and-payload-evolution) for
where these four sit beside ADR-0020, ADR-0021, ADR-0031, ADR-0032 and ADR-0033.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0046 | [`kb-decision-0046`](../decisions/0046-commit-reports-a-nothing-to-do-outcome.md) | commit reports a nothing-to-do outcome rather than an error | accepted | 12 | — |
| ADR-0047 | [`kb-decision-0047`](../decisions/0047-tags-and-scope-must-agree-at-commit.md) | An under-tagged model is refused at commit, not silently admitted | accepted | 12 | — |
| ADR-0049 | [`kb-decision-0049`](../decisions/0049-a-codec-declares-the-tags-it-reads.md) | A codec declares the tags it reads, and that is narrower than it sounds | accepted | 12 | — |
| ADR-0059 | [`kb-decision-0059`](../decisions/0059-the-domain-event-guard-checks-positional-agreement.md) | The domain-event guard checks positional agreement between a fold and its query | accepted | 12 | — |

## 2026-09-07 intake: pre-publication ratifications, spec-governance and publication halves (ADR-0041, ADR-0044, ADR-0045, ADR-0057)

Four decision atoms, the same wave. ADR-0045 requires a citation anchor to match its line exactly
or have the lint refuse to guess, phase 12, joining
[`domain-map.md`](domain-map.md#specification-governance--conformance)'s conformance domain
alongside the anchoring-citations playbook it `related`s. ADR-0041, ADR-0044 and ADR-0057 open this
map's first "Publication and release readiness" decisions: ADR-0041 records the repository going
public and the vulnerability channel that does not depend on it; ADR-0044 requires a published
crate to re-export any crate whose type appears in one of its own public signatures; ADR-0057 drops
`happenstance-testkit`'s resolvable dev-dependency version key from every publishable crate's
manifest. None supersedes any row on this map. See
[`domain-map.md`](domain-map.md#publication-and-release-readiness) for the new domain these three
open.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0041 | [`kb-decision-0041`](../decisions/0041-the-repository-is-public-and-security-has-an-email-channel.md) | The repository is public, and the vulnerability channel does not depend on it | accepted | 12 | — |
| ADR-0044 | [`kb-decision-0044`](../decisions/0044-a-published-crate-re-exports-its-drivers.md) | A published crate re-exports any crate whose type appears in one of its public signatures | accepted | 12 | — |
| ADR-0045 | [`kb-decision-0045`](../decisions/0045-a-citation-anchor-matches-exactly-or-the-lint-refuses.md) | A citation anchor matches its line exactly, or the lint refuses to guess | accepted | 12 | — |
| ADR-0057 | [`kb-decision-0057`](../decisions/0057-the-testkit-version-key-is-dropped.md) | The testkit dev-dependency version key is dropped from every publishable manifest | accepted | 12 | — |

## 2026-09-09 intake: the Ladybug adapter and two re-evaluations (ADR-0025, ADR-0060, ADR-0061)

Three decision atoms, one wave (`2026-09-09-intake`), spanning phase 10 and phase 11,
`.kb/decisions/`. ADR-0025 is the fifth event-store-adjacent adapter and the first over a graph
engine: `happenstance-ladybug`'s projection store keeps its checkpoint as a `__hs_checkpoint` node
written as the last statement before `COMMIT`, writes through raw parameterised Cypher rather than a
typed builder, and ships blocking-only behind an off-by-default feature because the driver is a 1.44
GB prebuilt static archive with an OpenSSL toolchain dependency; it `depends_on` `kb-decision-0017`
(the owned-`Batch` shape its checkpoint node relies on) and `kb-decision-0030` (the checkpoint's
progress obligation, PS-1, that the node discharges). ADR-0060 re-evaluates PS-2's `[FROZEN]`
batch-shape bar against the real adapter population for the second time — `kb-decision-0036` found
one adapter at one end at `0.2.0`; by phase 11 four have run the projection suite, spanning a file, a
pooled server, a one-shot HTTP proxy and an embedded graph database, and phase 11's own pre-registered
re-open condition did not fire — and keeps the `unstable-projection` gate anyway, on a reason
`kb-decision-0036` did not have: `begin`, `probe_write` and `probe_read_through` are synchronous and
infallible for both drivers PS-2 names by independent mechanisms, so a store whose batch genuinely is
a live transaction must declare `READS_THROUGH_BATCH = false`, a false statement about itself, and the
suite cannot distinguish that store from a buffering one. It `depends_on` `kb-decision-0036` — the
decision it reaffirms without editing or superseding, the fourth instance of that shape this map
records. ADR-0061 is the falsifier ES-11's own `[PROVISIONAL]` marker named arriving: `happenstance-neon`,
the first one-shot-HTTP event store, fails `read_result_is_stable_under_concurrent_append`
intermittently because ES-11's asynchronous-driver sufficiency sentence was a fact about pooled
drivers (ordering at the client) presented as a fact about async ones (ordering at the store); the
sentence is narrowed to require ordering against a later append by something the store itself honours,
a repair under `kb-playbook-repair-frozen-clause-001`'s test applied to the specification rather than
to a decision atom, so no existing row on this map is touched. It `depends_on` `kb-decision-0011` (the
read-is-one-sample-with-a-ceiling shape ES-11 qualifies). None of the three supersedes a row on this
map.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0025 | [`kb-decision-0025`](../decisions/0025-the-ladybug-projection-adapter.md) | The Ladybug projection adapter — a checkpoint node, raw Cypher, and a blocking driver | accepted | 11 | — |
| ADR-0060 | [`kb-decision-0060`](../decisions/0060-ps-2s-axis-re-evaluated.md) | The projection port keeps its gate, and the reason ADR-0036 gave has expired | accepted | 11 | — |
| ADR-0061 | [`kb-decision-0061`](../decisions/0061-es-11s-sufficiency-condition-assumed-a-queue.md) | ES-11's sufficiency condition assumed a queue, and one-shot HTTP has none | accepted | 10 | — |

## 2026-09-11 intake: the probe seam moves, the port freezes, and the measurement host gets declared conditions (ADR-0062, ADR-0063, ADR-0064)

Three decision atoms, one wave (`2026-09-11-intake`), `.kb/decisions/`. ADR-0062 moves the probe seam
`probe_write`/`probe_delete_all`/`probe_read_through` to `&mut Self::Batch` / async / fallible and moves
`begin` with it, because a probe seam moved without `begin` would let a live-transaction store report
itself and still be unable to exist for `sqlx`; `LivePostgresProjectionStore` (`happenstance-postgres`)
runs 20 of 20 against a live PostgreSQL with `READS_THROUGH_BATCH = true` now a true statement, so PS-2's
MUST is met as written and no port-axis change is needed. It `depends_on` `kb-decision-0060` (the bar it
meets), `kb-decision-0036` (the gate it was first shipped behind) and `kb-decision-0017` (the owned-`Batch`
shape the moved seam still owns). ADR-0063 freezes `ProjectionStore`, `ProjectionProbe` and their value
types on that evidence — a signature change to any of them is now a breaking change with a decision record
behind it — keeps `happenstance-core`'s `unstable-projection` feature declared and empty rather than
removed, and narrows `happenstance`'s feature of the same name to gate the projection runner alone, because
`Projection::apply` is still synchronous and cannot drive a live-transaction batch; it `depends_on`
`kb-decision-0062` (the evidence the freeze rests on), `kb-decision-0036`, `kb-decision-0060` and
`kb-decision-0017`. ADR-0064 declares the dedicated Linux measurement host's conditions as data —
`ops/host/host.env`, applied by scripts that only write and asserted by a `preflight.sh` that only reads —
and records that the preflight is structurally unreachable from `xtask`'s step table, `.redkiln/config.yaml`'s
`verify:` block or any CI job, with `ops/` held on `xtask/src/affected.rs`'s `INERT` list. Both atoms carry a
provenance note: at this worktree's `HEAD` neither long-form ADR record is present and the code the atom
describes (`begin`'s async signature, the narrowed `happenstance` feature) has not yet landed, so each states
what the lane binds when it lands rather than a fact already true of `main`, the same shape `kb-decision-0037`
used. None of the three supersedes a row on this map — all three carry `supersedes: null`.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0062 | [`kb-decision-0062`](../decisions/0062-the-probe-seam-moves-and-the-far-end-is-built.md) | The probe seam moves, begin moves with it, and the far end is built | accepted | 12 | — |
| ADR-0063 | [`kb-decision-0063`](../decisions/0063-the-projection-port-is-frozen.md) | The projection port is frozen, and the typed layer keeps a gate of the same name | accepted | 12 | — |
| ADR-0064 | [`kb-decision-0064`](../decisions/0064-the-measurement-host-has-declared-conditions.md) | The measurement host has declared conditions, and its preflight is unreachable from the gate | accepted | — | — |

## 2026-09-21 intake: the busy timeout is fifteen seconds (ADR-0065)

One decision atom, one wave (`2026-09-21-intake`), `.kb/decisions/`. ADR-0065 partly supersedes
ADR-0022 at section 11 only — the `busy_timeout` value `happenstance-sqlite` ships. Section 11 set
5,000 ms on the premise that `busy = 0` held in every row of the 64-contender table and named its
own reopen condition as `busy > 0`; `kb-reference-busy-timeout-margin-001` (2026-09-04) measured
`busy > 0` at the shipped `CONTENDERS = 64`, firing that condition. ADR-0065 sets
`BUSY_TIMEOUT_MS` to 15,000, this time measured against the shipped adapter's own concurrency
target rather than the experiment's candidate (`kb-reference-busy-timeout-adapter-cap-sweep-001`,
created the same wave): at `--test-threads=1`, the worst case the host can produce, 5,000 ms went
red 7 launches of 8 and 15,000 ms went red 0 of 16. `depends_on: [kb-decision-0022]`; `supersedes`
is `null` and `kb-decision-0022` is not flipped — sections 4, 6, 7, 9, 10, 12 and 15 stand — the
same shape `kb-decision-0031` used against ADR-0007. See *Reading the partial-supersession chain*
above.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0065 | [`kb-decision-0065`](../decisions/0065-adr-0022-s11-busy-timeout-is-fifteen-seconds.md) | ADR-0022 §11 is superseded in part — the busy timeout is fifteen seconds | accepted | — | partly supersedes `kb-decision-0022` |

## 2026-09-28: Weigh-In decisions, written by hand (phase 15)

Six decision atoms, and the first wave written without `/redkiln:kb-ingest`, which is retired in
this repository. Weigh-In staged each one in `.kb/_intake/decisions/` when the owner decided it;
phase 15's intake pass gave each one this layer's frontmatter shape — `adr_id: null`, `phase: 15`,
`supersedes: null` and its intake path in `source_paths` — kept Weigh-In's own fields as
provenance, and left every body verbatim. None carries an ADR number, and none supersedes a row on
this map. They are dated by `decided_at`, which Weigh-In writes in UTC, so three of them read
2026-09-29.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| — | [`kb-decision-wi-016abe`](../decisions/wi-016abe-reconcile-hs-i0006-by-closing-it-as-is-and.md) | Close HS-I0006 and re-plan | accepted | 15 | — |
| — | [`kb-decision-wi-052920`](../decisions/wi-052920-do-changelog-unreleased-s-entries-sqlite-busy.md) | Do CHANGELOG [Unreleased]'s entries (SQLite busy timeout, ADR-0065; FaultyStore::contend_next) ship as 0.3.3 or ride 0.4.0: Ride 0.4.0 | accepted | 15 | — |
| — | [`kb-decision-wi-38373d`](../decisions/wi-38373d-what-checks-kb-frontmatter-and-accepted-atom.md) | What checks .kb frontmatter and accepted-atom immutability until redkiln-rs: xtask lint | accepted | 15 | — |
| — | [`kb-decision-wi-40b321`](../decisions/wi-40b321-does-v1-0-include-happenstance-sync-or-does-1-0.md) | Sync inside 1.0 | accepted | 15 | — |
| — | [`kb-decision-wi-ab0a5a`](../decisions/wi-ab0a5a-how-much-merge-authority-does-the-phase-15-afk.md) | How much merge authority does the phase 15 AFK session have over its own PRs: Self-merge on green | accepted | 15 | — |
| — | [`kb-decision-wi-b9b9ab`](../decisions/wi-b9b9ab-d-3-delete-the-merged-branches.md) | Delete all seven | accepted | 15 | — |
| — | [`kb-decision-wi-6c9f77`](../decisions/wi-6c9f77-may-claude-md-binding-constraint-5-s-stale.md) | May CLAUDE.md binding constraint 5's stale present-tense registry narrative (0.2.0 'has not happened yet', 'nothing is yanked', max_stable_version reads 0.0.0) be rewritten to the registry's current state, leaving the MSRV constraint itself unchanged: rewrite the tense; the MSRV constraint is unchanged | accepted | 15 | — |

| — | [`kb-decision-wi-2798d5`](../decisions/wi-2798d5-which-crates-does-1-0-promise-and-is.md) | Which crates does 1.0 promise: nine; ladybug outside | accepted | 16 | — |
| — | [`kb-decision-wi-d61f21`](../decisions/wi-d61f21-does-a-workerd-class-conformance-run-land.md) | Does a workerd-class conformance run land before 1.0: a workerd sibling job before 1.0 | accepted | 16 | — |
| — | [`kb-decision-wi-8e5bd4`](../decisions/wi-8e5bd4-how-are-crate-versions-managed-after-1-0.md) | How are crate versions managed after 1.0: lockstep 1.0.0, then independent | accepted | 16 | — |
| — | [`kb-decision-wi-460397`](../decisions/wi-460397-what-is-the-msrv-policy-after-1-0-and-what.md) | The MSRV policy after 1.0: hold 1.97.1; bounded rises | accepted | 16 | — |
| — | [`kb-decision-wi-1408e8`](../decisions/wi-1408e8-how-long-a-soak-between-1-0-0-rc-1-and-1-0-0.md) | The rc soak: conditions only, no time floor | accepted | 16 | — |
| — | [`kb-decision-wi-cbc941`](../decisions/wi-cbc941-what-does-the-project-promise-for-security.md) | Security support after 1.0: latest minor, plus the previous major for six months | accepted | 16 | — |
| — | [`kb-decision-wi-7899af`](../decisions/wi-7899af-does-adr-0066-promise-that-the-licence-does-not.md) | The licence promise: the nine crates stay MIT OR Apache-2.0 for 1.x | accepted | 16 | — |

The seventh row, `kb-decision-wi-6c9f77`, was added on 2026-09-29, when the owner
decided the tense of `CLAUDE.md`'s binding constraint 5 and closed phase 15. The
seven rows after it are the owner's phase-16 calls, taken in the planning session
of 2026-09-29 and recorded in ADR-0066. They were moved from
`.kb/_intake/decisions/` in phase 15's shape, with `phase: 16`, and their bodies
are verbatim.

## 2026-09-29: defining 1.0, written by hand (phase 16)

Six ADR-numbered decision atoms, written by hand in phase 16. ADR-0066 and ADR-0068 have long-form
records under `references/adr/`; the other four are atoms only. ADR-0066 is the 1.0 charter and the other five
settle what it needed settled first: the MSRV after 1.0 (ADR-0067), what remains of ADR-0022's
fired falsifiers (ADR-0068), D-1's total path (ADR-0069), the runner's chunk type and observation
seam (ADR-0070), and whether ES-10 stays global (ADR-0071). Between them they close seven open
questions. None carries `supersedes`. ADR-0068 partly supersedes ADR-0022 at §8 items 1 and 2 and
§16's falsifier for §8, leaving §9 undecided and ratifying the rest, so `kb-decision-0022` stays
accepted and its row gains a second annotation, the ADR-0065 shape. ADR-0067 amends
`kb-decision-0037` and corrects the `cfg_select!` threshold both it and `kb-decision-0029` state,
editing neither body.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0066 | [`kb-decision-0066`](../decisions/0066-what-1-0-promises.md) | What 1.0 promises — nine crates, one disposition per unfrozen clause, and CF-39 frozen | accepted (partly superseded) | 16 | partly superseded by `kb-decision-0076` (§5's `#[doc(hidden)]` exemption, where it reached a promised emitter) |
| ADR-0067 | [`kb-decision-0067`](../decisions/0067-msrv-after-1-0-rises-are-bounded.md) | The MSRV holds at 1.97.1 into 1.0, and after 1.0 a rise is bounded | accepted | 16 | amends `kb-decision-0037` |
| ADR-0068 | [`kb-decision-0068`](../decisions/0068-adr-0022-sections-8-9-16-settled.md) | ADR-0022 §8 and §16 are superseded in part, §9 is not decided here, and the rest is ratified | accepted | 16 | partly supersedes `kb-decision-0022` |
| ADR-0069 | [`kb-decision-0069`](../decisions/0069-queryitem-gains-a-total-constructor.md) | QueryItem gains a total constructor, and the outward face of D-1 is closed as intended | accepted | 16 | — |
| ADR-0070 | [`kb-decision-0070`](../decisions/0070-the-runner-takes-a-named-chunk.md) | The runner takes a named Chunk, and gets no observation seam at 1.0 | accepted | 16 | — |
| ADR-0071 | [`kb-decision-0071`](../decisions/0071-es-10-stays-global.md) | ES-10 stays global, because the frozen checkpoint is one position | accepted | 16 | — |

## 2026-09-29: the breaking window, written by hand (phase 17)

Phase 17's records, written by hand as each of its lanes lands. ADR-0072 is the first: the phase is split
at its release, and the additive items go to a new phase, 17b.
ADR-0073 settles VT-10's foreign-identity write path. ADR-0028 is written under the number the
original queue reserved for it, which is why it sorts first. It takes the retention refusal, with
an additive reservation, and answers sub-question 3 of `kb-open-question-es-38-and-gap-read-unowned-001`,
which stays open for sub-question 2. ADR-0074 is the `Projection::apply` record: `apply` becomes
`async`, on evidence from a spike that drove a live transaction, and PS-9 and PS-11 are frozen. It
supersedes `kb-open-question-apply-synchronous-live-store-001`. ADR-0075 settles the projection
port's remaining 1.0 clauses: PS-15 narrowed and frozen, PS-23 and PS-24 frozen, and PS-38's
obligation documented. ADR-0076 makes the conformance emitters CF-23 obliges public API, un-hidden
and without their `__` prefix, and mints CF-41 to pin them. It partly supersedes ADR-0066 at §5's
`#[doc(hidden)]` exemption only, the ADR-0065 shape, so `kb-decision-0066` stays accepted and its
row gains the annotation. ADR-0077 promises `AppendError::Busy` at 1.0, has the typed commit loop
retry it inside the same `Retry` bound, mints ES-43, and supersedes
`kb-open-question-testkit-contention-tolerance-001` with the per-error arm.
ADR-0078 retires `happenstance-ladybug` on the owner's Weigh-In call
(`wi-630032`): excluded from the workspace and kept in the tree as a frozen record, because its
files are cited by line. It supersedes nothing.
ADR-0079 is lane L6b's: on the `workerd` job's measurements, a `happenstance-cloudflare` query
item binds a constant number of parameters through `json_each`, the two partition widths become
`workerd`'s 5 and 90, and the test shim takes `workerd`'s four statement limits. It supersedes
nothing, and answers the first item ADR-0052 left undecided: the adapters' widths need not agree.
ADR-0080 is lane L7's: on the two-build measurement ADR-0012's falsifier asked for, taken against
`happenstance-cloudflare` with calibrated replica arms, `append` keeps `&[Event]` and ES-17 is
frozen. It rests on ADR-0012 and ADR-0055 without superseding either, records what neither can
say, and supersedes `kb-open-question-es-17-two-adapter-measurement-001`.
ADR-0086, proposed, settles VT-6's phase-17 half for the two server adapters: neither mints per
open, both keep mint-once and earn it through a documented re-mint, and default-refusing detection
is ruled out after 1.0. On acceptance it supersedes
`kb-open-question-postgres-neon-store-id-no-restore-001`. It corrects the claim that mint-per-open
fails `reopened_store_does_not_reissue_an_event_id`.

| ADR | Atom | Title | Status | Phase | Supersedes / superseded by |
| --- | --- | --- | --- | --- | --- |
| ADR-0028 | [`kb-decision-0028`](../decisions/0028-what-a-store-may-forget.md) | What a store may forget is decided outside the port, and a report of it can only arrive additively | accepted | 17 | — |
| ADR-0072 | [`kb-decision-0072`](../decisions/0072-phase-17-is-split-at-the-release.md) | Phase 17 is split at the release — what must ship in 0.4.0, and 17b after it | accepted | 17 | — |
| ADR-0073 | [`kb-decision-0073`](../decisions/0073-the-foreign-identity-write-path-is-the-adapters.md) | The write path that keeps a foreign identity is the adapter's row writer, and core grows nothing | accepted | 17 | — |
| ADR-0074 | [`kb-decision-0074`](../decisions/0074-projection-apply-is-async.md) | Projection::apply is async, is handed a Delivered event, and fails with the projection's own error | accepted | 17 | — |
| ADR-0075 | [`kb-decision-0075`](../decisions/0075-the-projection-ports-1-0-clauses.md) | The projection port's 1.0 clauses — PS-15 narrowed and frozen, PS-23 and PS-24 frozen, PS-38 documented | accepted | 17 | — |
| ADR-0076 | [`kb-decision-0076`](../decisions/0076-the-cf-23-emitters-are-public-api.md) | The emitters CF-23 obliges are public API — un-hidden, renamed without the prefix, and pinned by CF-41 | accepted | 17 | partly supersedes `kb-decision-0066` |
| ADR-0077 | [`kb-decision-0077`](../decisions/0077-appenderror-busy.md) | A busy store is not a broken one — AppendError::Busy is promised, and the typed loop retries it | accepted | 17 | — |
| — | [`kb-decision-wi-2ab1f3`](../decisions/wi-2ab1f3-does-dcb-independence-extend-to-busy-may-a.md) | DCB independence is a promise about conflict, not liveness; `Busy` allowed; rule renamed (owner, Weigh-In) | accepted | 17 | — |
| ADR-0078 | [`kb-decision-0078`](../decisions/0078-happenstance-ladybug-is-retired.md) | happenstance-ladybug is retired — excluded from the workspace, kept in the tree as a frozen record | accepted | 17 | — |
| — | [`kb-decision-wi-ff17f4`](../decisions/wi-ff17f4-ps-15-freeze-after-narrowing-to-commit-and.md) | PS-15 frozen narrowed to commit and reset; rollback non-normative (owner, Weigh-In) | accepted | 17 | — |
| — | [`kb-decision-wi-630032`](../decisions/wi-630032-keep-happenstance-ladybug-in-the-workspace-or.md) | Abandon happenstance-ladybug: exclude it from the workspace, keep the directory as a frozen record (owner, Weigh-In) | accepted | 17 | — |
| ADR-0079 | [`kb-decision-0079`](../decisions/0079-a-query-item-binds-a-constant-number-of-parameters.md) | A query item binds a constant number of parameters, and happenstance-cloudflare's widths are workerd's | accepted | 17 | — |
| ADR-0080 | [`kb-decision-0080`](../decisions/0080-append-keeps-a-borrowed-batch.md) | append keeps its borrowed batch, and ES-17 is frozen on the two-build measurement | accepted | 17 | — |
| ADR-0086 | [`kb-decision-0086`](../decisions/0086-postgres-and-neon-keep-mint-once.md) | Postgres and Neon keep mint-once, earned by a documented re-mint, and mint-per-open is declined | proposed | 17 | — |

## Adding a row

A new decision atom gets a row in ADR-number order under the wave section that introduced it
(open a new `##` section per wave, the way [`domain-map.md`](domain-map.md) opens a new section
per domain). A supersession updates two rows in the same edit: the new atom's row, and the old
atom's `Status` and `Supersedes / superseded by` cells — never delete a superseded row.
