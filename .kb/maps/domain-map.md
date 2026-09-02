---
id: kb-map-domain-001
title: Domain map
kind: map
status: accepted
authority_tier: note
summary: >-
  The corpus grouped by subject area rather than by kind or directory. One section per domain;
  within a section, atoms are grouped by kind (reference, playbook, open_question, ...) with a
  one-line orientation and a link. Updated by the Maps phase of every kb-ingest wave that adds a
  new canonical concept or domain path; entries are not removed when an atom is superseded, only
  annotated. The 2026-08-13 wave added ADR-0017–0019 (phase 6, ProjectionStore) to the existing
  "Contract ports, conformance, and the ADR corpus" domain and annotated one open question there
  as superseded. The 2026-08-15 wave added ADR-0030 and a new open question (PS-32) to that same
  domain, and a new reference atom on spec-trace's per-family suite switch to "Specification
  governance & conformance," annotating the PS-1 and PS-19 open questions in both places as
  superseded by ADR-0030. A second, later 2026-08-15 wave (`2026-08-15-intake`) added a new third
  domain, "The typed layer: decision models, codecs, and payload evolution," for ADR-0020 and
  ADR-0021 (phase 7) — the two are a subject area neither existing domain frames itself around, and
  the domain cites `kb-decision-0006` and `kb-decision-0007` by reference rather than moving them
  out of the ports domain. The 2026-08-17 wave added ADR-0022 (phase 8, SQLite append-condition
  strategy) to the ports domain, and ADR-0031, ADR-0032 and ADR-0033 (phase 7) to the typed-layer
  domain; ADR-0031 partly supersedes ADR-0007 and ADR-0032 fully supersedes ADR-0021, both
  reflected here by annotation rather than by moving either superseded row. A new open question,
  ES-17, was added to the ports domain, and D-1, CF-36 and "no PS rule name is resolved" were added
  to the typed-layer and specification-governance domains respectively. The 2026-08-20 wave
  (`2026-08-20-intake-phase-9`) added ADR-0023 and ADR-0034 (phase 9) to the ports domain, along
  with a new reference atom on WF-11's memory-ceiling verdict and a new open question on the
  absent workerd-class runner; and a new reference atom, the phase-8 specification reconciliation
  census, to "Specification governance & conformance." A second new open question from this wave,
  on the worker crate's `deny.toml` ban collision, is indexed on `open-questions-index.md` only —
  it names no canonical concept or domain path of its own, so this map is not the atom it touches.
  The 2026-09-02 wave (`2026-09-02-intake`) added ADR-0035 and ADR-0036 to the ports domain along
  with a new open question on the two adapters that have not yet followed happenstance-sqlite's
  default-feature fix, and flipped kb-open-question-worker-async-trait-ban-001 to superseded there
  (by ADR-0035) — the flip itself lives on open-questions-index.md. It also opened two new domains:
  "Documentation standards: the page-need discipline," one playbook atom from the docs-that-teach
  initiative; and "Brand identity: the name, the mark, and where it lives," three SD- decisions,
  two reference atoms and two design-tier concept atoms from four staged brand documents, plus a
  new open question on the unrun trademark search.
depends_on: []
related:
  - kb-map-open-questions-index-001
  - kb-map-decision-001
source_paths:
  - .kb/_governance/integration-waves/2026-08-10-intake/01-claims-and-classification.md
  - .kb/_governance/integration-waves/2026-08-10-intake/02-placement-and-adjudication.md
  - .kb/_governance/integration-waves/2026-08-10-intake-2
  - .kb/_governance/integration-waves/2026-08-13-projection-adrs
  - .kb/_governance/integration-waves/2026-08-15-adr-0030-checkpoint-progress
  - .kb/_governance/integration-waves/2026-08-15-intake
  - .kb/_governance/integration-waves/2026-08-17-adr-0022-append-condition
  - .kb/_governance/integration-waves/2026-08-20-intake-phase-9
  - .kb/_governance/integration-waves/2026-09-02-intake
last_reviewed: 2026-09-02
---

# Domain map

This is the corpus's subject-matter index. Where the directory layout
(`decisions/`, `playbooks/`, `reference/`, `open-questions/`, ...) groups
atoms by *kind*, this map groups them by *what they are about* — so a reader
who lands on one atom can find its neighbours without already knowing they
exist. See [`../maps/README.md`](README.md) for what belongs on a map atom
and what does not; see
[`open-questions-index.md`](open-questions-index.md) for the companion index
that this map does not duplicate.

This is the map's first wave. One domain exists so far.

## Specification governance & conformance

The area concerned with `spec/SPECIFICATION.md` itself — whether a clause's
prose, its assigned conformance rules, and the code agree, and what a pass
does when it finds they do not. Established by the 2026-08-10 phase 4/5
specification reconciliation, an unscheduled pass that read the specification
back against the tree phases 4 and 5 had already changed.

**Reference**

- [`phase-4-5-specification-reconciliation-census.md`](../reference/phase-4-5-specification-reconciliation-census.md)
  (`kb-reference-phase-4-5-spec-reconciliation-001`) — the census and pointer:
  what the pass found, counted by defect class, and where the full evidence
  lives. Everything else in this domain cites this atom rather than
  restating its counts.
- [`spec-trace-has-suite-family-switch.md`](../reference/spec-trace-has-suite-family-switch.md)
  (`kb-reference-spec-trace-has-suite-001`) — `cargo xtask spec-trace`'s
  citation check runs only against clause families `has_suite` admits; `PS`
  sat outside that switch for two slices after its conformance suite was
  written, with every gate green throughout. Added 2026-08-15.
- [`phase-8-specification-reconciliation-census.md`](../reference/phase-8-specification-reconciliation-census.md)
  (`kb-reference-phase-8-spec-reconciliation-001`) — the second census of the
  standing post-phase reconciliation criterion, hand-run for phase 8: of 401
  `spec-trace` citations checked only 80 are anchored to their subject, the
  criterion's arithmetic bullet had no clause range to close against, and 14
  of 20 repairs were false sentences no gate step could reach. Extends, does
  not replace, the phase-4/5 census above. Added 2026-08-20.

**Playbooks** — transferable practice this pass extracted

- [`verify-the-referent-and-report-coverage.md`](../playbooks/verify-the-referent-and-report-coverage.md)
  (`kb-playbook-verify-referent-report-coverage-001`) — a cross-reference
  checker must verify the referent, not just the address, and must report
  its own coverage.
- [`anchoring-citations-in-a-long-lived-document.md`](../playbooks/anchoring-citations-in-a-long-lived-document.md)
  (`kb-playbook-anchoring-citations-001`) — how to check `file:line`
  citations against a source tree that moves, without a checker that either
  passes forever or fires on every ordinary edit.
- [`landing-a-stricter-gate-without-a-red-baseline.md`](../playbooks/landing-a-stricter-gate-without-a-red-baseline.md)
  (`kb-playbook-ratchet-gate-landing-001`) — what to do when tightening a
  gate check surfaces violations nobody is authorised to fix in the same
  change: the ratchet, and when it is the wrong instrument.
- [`repairing-a-frozen-clause-without-amending-it.md`](../playbooks/repairing-a-frozen-clause-without-amending-it.md)
  (`kb-playbook-repair-frozen-clause-001`) — the mechanical test for whether
  a correction to a `[FROZEN]` clause is a repair or a gap, and the safe form
  for each.

**Open questions** — see [`open-questions-index.md`](open-questions-index.md)
for the full, self-contained list. The ones this domain owns:
`kb-open-question-disjoint-boundaries-no-clause-001`,
`kb-open-question-model-family-rule-no-clause-001`,
`kb-open-question-ps-1-no-progress-obligation-001` (**superseded** 2026-08-15
by `kb-decision-0030`; sub-questions 1, 2 and 4 answered, PS-1's own text
byte-identical),
`kb-open-question-ps-19-scope-narrower-001` (**superseded** 2026-08-15 by
`kb-decision-0030`; sub-questions 1 and 3 answered, sub-question 2's
2026-08-13 verdict stands),
`kb-open-question-es-6-unwritable-rule-001`,
`kb-open-question-provisional-falsifiers-001`,
`kb-open-question-post-phase-reconciliation-001`,
`kb-open-question-cf-36-unperformed-cross-reference-001` (added 2026-08-17 — CF-36 is `[FROZEN]`
and claims a level-marker cross-reference `cargo xtask spec-trace` does not perform),
`kb-open-question-no-ps-rule-name-resolved-001` (added 2026-08-17 — CF-38 is `[FROZEN]`; a bare
dagger in a clause's `Rule:` line, not the `has_suite` family switch this domain's own reference
atom records fixed, is what still leaves every `PS` rule name unresolved).

## Contract ports, conformance, and the ADR corpus (2026-08-10 ADR import)

The area concerned with `happenstance-core`'s async port design (`EventStore`,
`ProjectionStore`), the conformance suite that proves an adapter against it, and the
seventeen-ADR decision record — async port flavours through the wire format — that this
project's early phases rest on. Established by the 2026-08-10 ADR import, which brought
`.kb/decisions/0001` through `.kb/decisions/0016` and `.kb/decisions/0029` into `.kb/decisions/` as one wave.
The 2026-08-13 wave added three more to this same domain — `.kb/decisions/0017`, `0018` and
`0019` — settling `ProjectionStore::Batch`'s ownership, checkpoint reset, and the port's
no-op-on-`apply`-failure stance, phase 6's `ProjectionStore` freeze. The 2026-08-15 wave added a
fourth, `.kb/decisions/0030`, minting `[PROVISIONAL]` clause PS-38 — a successful `commit` MUST
advance its `ProjectionId`'s checkpoint, and an id no successful `commit` has named MUST read as
`Checkpoint::NeverRun` — the progress obligation section 4 never stated. The 2026-08-17 wave added
a fifth, `.kb/decisions/0022`, phase 8's first real measurement against SQLite: the append-condition
SQL strategy and tag storage `happenstance-sqlite` ships, settling what ADR-0012 declined to decide
at phase 4. The 2026-08-20 wave added a sixth and seventh, `.kb/decisions/0023` and
`.kb/decisions/0034`, both phase 9: ADR-0023 settles the Cloudflare `SqlStorage` mapping and the
off-tokio harness that proves it, merging in the ES-6 verdict as one body of evidence; ADR-0034
settles that the fixture contract has no single owning document, answering
`kb-open-question-cf-40-ownership-001` from outside the three accepted decisions (ADR-0015,
ADR-0012, ADR-0022) it cites without editing any of them. The 2026-09-02 wave added an eighth and
ninth, `.kb/decisions/0035` and `.kb/decisions/0036`: ADR-0035 exempts `worker` and
`worker-macros` from `deny.toml`'s `async-trait` ban by name, amending `.kb/decisions/0001`'s
exemption set from outside; ADR-0036 evaluates PS-2's `[FROZEN]` freeze bar against the real
adapter set for the first time and declines to freeze `ProjectionStore` at `0.2.0`, shipping it
instead behind the off-by-default `unstable-projection` feature. The full decision list, including
status and supersession, is [`decision-map.md`](decision-map.md) rather than repeated here.

**Reference**

- [`port-traits-compiled-findings.md`](../reference/port-traits-compiled-findings.md)
  (`kb-reference-port-traits-compiled-findings-001`) — what compiling `EventStore` and
  `ProjectionStore`, rather than reasoning about them, found across ADR-0001, ADR-0008,
  ADR-0009, ADR-0010 and ADR-0011. Gained a sixth finding 2026-08-17: the caller-side
  `S::Error: Send + Sync` obligation ADR-0009 assigned is now exercised by a real consumer,
  `happenstance::run_projection` (ADR-0031), and holds as frozen rather than as a defect.
- [`position-visibility-experiment-2026-08.md`](../reference/position-visibility-experiment-2026-08.md)
  (`kb-reference-position-visibility-experiment-001`) — the four-arm measurement against real
  PostgreSQL that ADR-0013's visibility invariant rests on.
- [`wire-format-encoding-measurements.md`](../reference/wire-format-encoding-measurements.md)
  (`kb-reference-wire-format-measurements-001`) — the encoding-size measurements ADR-0016 rests
  on, plus two instruments that measured wrong.
- [`append-condition-experiment-2026-08.md`](../reference/append-condition-experiment-2026-08.md)
  (`kb-reference-append-condition-experiment-001`) — the phase-8 measurement ADR-0022 rests on:
  three append-condition strategies and three tag storages against real SQLite. Added 2026-08-17.
- [`projection-fan-out-costs-n-reads.md`](../reference/projection-fan-out-costs-n-reads.md)
  (`kb-reference-projection-fan-out-cost-001`) — `ProjectionStore::apply`'s `&mut P` fixes N
  projections at N reads, derivable from the signature alone; a shape, not a verdict, on the
  `[PROVISIONAL]` projection family. Added 2026-08-17.
- [`wf-11-memory-ceiling-verdict-2026-08.md`](../reference/wf-11-memory-ceiling-verdict-2026-08.md)
  (`kb-reference-wf-11-memory-ceiling-verdict-001`) — WF-11's falsifier fired at directly on
  `happenstance-cloudflare`'s wasm32 harness: the memory-ceiling condition is not constructible
  on this runtime (a Node isolate has no per-isolate cap), and the category finding — no streaming
  entry point for a human-readable payload — reproduces the published cost table exactly. Added
  2026-08-20.

**Concepts**

- [`torn-reads-and-the-append-condition-boundary.md`](../concepts/torn-reads-and-the-append-condition-boundary.md)
  (`kb-concept-torn-read-append-boundary-001`) — why the append condition, derived from the
  read's own observed maximum, cannot catch a torn read; the mechanism ADR-0011, ADR-0012 and
  ADR-0013 each protect without stating.

**Governance**

- [`rewrite-the-referent-never-the-reasoning.md`](../governance/rewrite-the-referent-never-the-reasoning.md)
  (`kb-governance-referent-not-reasoning-001`) — the discrimination between a rename that may be
  rewritten in place and reasoning that, once a decision stands, is never touched, worked out
  against ADR-0001 through ADR-0007.

**Playbooks**

- [`one-decision-per-adr-title.md`](../playbooks/one-decision-per-adr-title.md)
  (`kb-playbook-one-decision-per-adr-title-001`) — an "and" in a decision's title is usually a
  strong decision and a weaker one bundled together, and the weaker half is the one likely to be
  reversed.
- [`testing-interleavings-with-cold-futures.md`](../playbooks/testing-interleavings-with-cold-futures.md)
  (`kb-playbook-cold-future-hand-polling-001`) — hand-polling two cold futures out of order to
  make a conformance rule observe a specific interleaving with no executor, thread or clock.

**Open questions** — see [`open-questions-index.md`](open-questions-index.md) for the full,
self-contained list. The ones this domain owns:
`kb-open-question-adr-status-vocabulary-001`,
`kb-open-question-projection-batch-no-apply-001` (**superseded** 2026-08-13 by
`kb-decision-0017`; sub-question 3 stays open, with the typed layer),
`kb-open-question-query-union-rule-unowned-001`,
`kb-open-question-global-vs-boundary-visibility-001`,
`kb-open-question-postgres-arm-c-cost-001`,
`kb-open-question-poll-count-rule-strength-001`,
`kb-open-question-es-38-and-gap-read-unowned-001`,
`kb-open-question-projection-id-unvalidated-001`,
`kb-open-question-cf-40-ownership-001`,
`kb-open-question-dcb-no-published-format-001`,
`kb-open-question-human-readable-encoding-limits-001`,
`kb-open-question-sync-message-set-undesigned-001`,
`kb-open-question-ps-32-adr-0007-correction-owed-001` (added 2026-08-15 — ADR-0007's Context
overstates what cannot be written against the port; only a superseding atom may correct it.
**Superseded** 2026-08-17 by `kb-decision-0031`, which carries the corrected Context as part of its
own partial supersession of ADR-0007),
`kb-open-question-es-17-two-adapter-measurement-001` (added 2026-08-17 — ADR-0012's falsifier item
1 asks for two builds of one SQLite adapter differing only in `append`'s batch ownership; the
phase-8 append-condition experiment measured three strategies against the same `&[Event]` signature
instead, and nothing currently scheduled produces the two-build evidence),
`kb-open-question-workerd-runner-absent-001` (added 2026-08-20 — the whole Cloudflare conformance
suite runs on `wasm32-unknown-unknown` under `wasm-bindgen-test-runner` against a `node:sqlite`
shim, never under `workerd`; ADR-0023 records the exclusion as an escalated, not a rejected,
finding, and WF-11's memory-ceiling non-fire is a second independent consequence of the same
absent runner),
`kb-open-question-adapter-default-projection-feature-001` (added 2026-09-02 — ADR-0036 ships
`ProjectionStore` behind off-by-default `unstable-projection`, `happenstance-sqlite` now forwards
that gate correctly, and `happenstance-neon`/`happenstance-postgres` still carry
`default = ["event-store", "projection-store"]` while also naming `happenstance-core`'s
`unstable-projection` unconditionally in `[dependencies]`, so toggling `default` alone would not
restore off-by-default there the way it did for `happenstance-sqlite`).

## The typed layer: decision models, codecs, and payload evolution

The area concerned with `happenstance` (the typed layer) rather than `happenstance-core`: how a
`DecisionModel`'s query is derived instead of hand-maintained, and where a codec tag and payload
evolution live once decoding is above the port. Established by the second 2026-08-15 wave
(`2026-08-15-intake`), which minted ADR-0020 and ADR-0021, phase 7. This is a new subject area
rather than an extension of "Contract ports, conformance, and the ADR corpus": that domain frames
itself around `happenstance-core`'s async port design, and these two decisions are about the crate
one seam above it. The domain stands on two atoms that stay in the ports domain rather than moving
here — `kb-decision-0006` allocated the typed-layer crate these decisions live in, and
`kb-decision-0007` drew the line that decoding sits strictly above the port, the premise ADR-0021's
first decision consumes; `kb-decision-0031`, added 2026-08-17, is the third atom to stay in the
ports domain's decision-map rows while governing conduct in this one — it is where
`kb-decision-0007`'s checkpoint pump ends up, `happenstance::run_projection`, one seam above the
port. See [`decision-map.md`](decision-map.md#2026-08-15-typed-layer-adrs-adr-0020-adr-0021)
for status and supersession rather than repeated here.

The 2026-08-17 wave added three more decisions to this domain. ADR-0031 fires ADR-0007's own
falsifier and collapses the checkpoint pump upward into `happenstance::run_projection` — a partial
supersession of ADR-0007, tracked on `decision-map.md`'s ADR-0007 row rather than moved here.
ADR-0032 fully supersedes ADR-0021, withdrawing one incorrect justification (a backwards reading of
ADR-0003) for an already-correct rejection while leaving all three of ADR-0021's own decisions
intact; ADR-0021's row below is annotated **superseded** rather than removed. ADR-0033 records
`happenstance-macros` out of scope for 0.1 against a measured ceremony ratio — a record, not a
supersession, since ADR-0020's own contrary prediction was published as explicitly falsifiable.

**Decisions**

- [`0020-fold-query-agreement.md`](../decisions/0020-fold-query-agreement.md)
  (`kb-decision-0020`) — a `DecisionModel`'s query is not hand-written; it is derived on a sealed
  `Boundary::query`, so the query and the fold can no longer name different event sets.
- [`0021-payload-evolution-and-codec-tag.md`](../decisions/0021-payload-evolution-and-codec-tag.md)
  (`kb-decision-0021`) — **superseded** by `kb-decision-0032` (2026-08-17). Three decisions still
  stand: the codec tag lives in `Event::metadata`'s framing region, `EventType` carries no version
  suffix, and an older payload shape is tolerated at decode rather than by a hook on `EventStore`.
  What was withdrawn was one incorrect justification for a rejected alternative, not any of those
  three.
- [`0031-the-runner-collapses-upward.md`](../decisions/0031-the-runner-collapses-upward.md)
  (`kb-decision-0031`) — added 2026-08-17. `happenstance-core` publishes no checkpoint pump;
  `happenstance::run_projection` is the only runner, reading the checkpoint, deriving the query,
  decoding through `Codec` and committing chunk-by-chunk. Partly supersedes `kb-decision-0007`.
- [`0032-adr-0021-serde-attribution-correction.md`](../decisions/0032-adr-0021-serde-attribution-correction.md)
  (`kb-decision-0032`) — added 2026-08-17. Supersedes `kb-decision-0021` in full: the framing
  region's rejection of `serde` stands on its two codec-independence grounds alone, never on
  ADR-0003, which constrains `happenstance-core` and positively assigns encoding to this crate.
- [`0033-happenstance-macros-out-of-scope-for-0-1.md`](../decisions/0033-happenstance-macros-out-of-scope-for-0-1.md)
  (`kb-decision-0033`) — added 2026-08-17. The worked example's ceremony-to-domain ratio is
  0.50:1/0.12:1 under both extreme classifications of its contested lines, against the 1.0
  threshold AC-013 set — `happenstance-macros` does not ship in 0.1.

**Reference**

- [`phase-7-macros-ceremony-measurement.md`](../reference/phase-7-macros-ceremony-measurement.md)
  (`kb-reference-macros-ceremony-measurement-001`) — the 29-range, line-by-line ceremony/domain
  classification of `examples/course-subscriptions/src/main.rs` that `kb-decision-0033` rests on.
  Added 2026-08-17.

**Open questions** — see [`open-questions-index.md`](open-questions-index.md) for the full,
self-contained list. The one this domain owns:
`kb-open-question-d-1-no-total-path-001` (added 2026-08-17 — `QueryItem::new` is fallible even over
already-validated inputs and `DomainEvent::tags` is total over a fallible `Tags`; no infallible
route exists in either direction, and it is the single condition that would reopen ADR-0033's
verdict).

## Documentation standards: the page-need discipline

The area concerned with `standards/pages/` — the rule that every page in the user documentation
declares, in its own visible body text, the single reader-question it answers, spelled from a
closed enumerated set and checked mechanically by `xtask/src/lint_pages.rs`. Established by the
2026-09-02 wave, which brought the `docs-that-teach` initiative's lesson in as one playbook atom.
This is a new subject area rather than an extension of any domain above: those are about
`happenstance-core`'s port design and the ADR corpus, and this one is about `docs/`, a directory
none of them touches.

**Playbooks**

- [`making-the-need-a-page-answers-declared-singular-checkable.md`](../playbooks/making-the-need-a-page-answers-declared-singular-checkable.md)
  (`kb-playbook-declared-page-need-001`) — a mechanical check verifies a *declaration*, never an
  *answer*; a written non-author procedure supplies the judgement the check cannot make, and the
  check's own documentation names the seam between them. The same pairing
  `kb-playbook-verify-referent-report-coverage-001` establishes on a different subject — a
  cross-reference checker that verifies an address and reports its own coverage.

## Brand identity: the name, the mark, and where it lives

The area concerned with what "happenstance" means, what the mark and wordmark look like, the
rules that bind anything carrying the name, and where the source material for all of it lives.
Established by the 2026-09-02 wave from four staged brand documents. Numbered `SD-` rather than
`ADR-`, deliberately outside the architecture decision sequence — `kb-reference-brand-source-locations-001`
states why — and carrying `phase: null` throughout, since brand work is not one of RUNBOOK.md's
fifteen phases.

**Decisions**

- [`sd-0001-the-name-names-the-boundary-of-what-occurred.md`](../decisions/sd-0001-the-name-names-the-boundary-of-what-occurred.md)
  (`kb-decision-sd-0001`) — "happenstance" names the boundary DCB draws around what actually
  occurred rather than a structure chosen in advance, and five rules that places on copy. Assigns
  a meaning after the fact and says so; does not supersede ADR-0005 (`kb-decision-0005`), which
  stays correct about why the rename happened.
- [`sd-0002-the-mark-and-the-rules-that-bind-it.md`](../decisions/sd-0002-the-mark-and-the-rules-that-bind-it.md)
  (`kb-decision-sd-0002`) — seven equal blocks radiate from a solid disc, the wordmark is always
  lowercase, and eight commitments bind anything carrying the name. Equality of the blocks and the
  seven-element count are established by rendering, cited from the two design atoms below rather
  than restated.
- [`sd-0002-standalone-svg-carries-one-colourway.md`](../decisions/sd-0002-standalone-svg-carries-one-colourway.md)
  (`kb-decision-standalone-svg-one-colourway-001`) — a standalone SVG carries one fixed colour;
  `prefers-color-scheme` is selected at the point of use, not embedded in the file. Shares its
  `SD-0002` identifier with the mark decision above — the sharing and the scope split (this rule
  reaches past logos to any SVG shipped for a surface it does not control) are recorded in both
  bodies.

**Reference**

- [`brand-mark-geometry-and-palette-2026-08.md`](../reference/brand-mark-geometry-and-palette-2026-08.md)
  (`kb-reference-brand-geometry-palette-001`) — the mark's build-grid geometry, the lockup's four
  placement constants, and the WCAG 2.1 contrast table the palette rests on, measured 2026-08-18.
  Draws no conclusion; the decision and design atoms below cite its numbers rather than restate
  them.
- [`brand-identity-source-locations-2026-08.md`](../reference/brand-identity-source-locations-2026-08.md)
  (`kb-reference-brand-source-locations-001`) — a pointer, not a copy: what `assets/brand/` and
  `references/brand/` each hold, which source answers "how do I apply this" versus "why is it like
  that", and the `SD-` numbering convention's precedence over the separate strategy workspace the
  identity was developed in. True of the working tree on 2026-08-18.

**Design patterns** — resolved interaction/visual-composition decisions, `authority_tier: design`

- [`symbol-annotates-the-wordmark.md`](../design/symbol-annotates-the-wordmark.md)
  (`kb-design-symbol-annotates-the-wordmark-001`) — a symbol beside a wordmark is set small and
  raised, overlapping the word's advance and nested into the final letter's open counter-space,
  rather than placed beside the word with an ordinary lockup gap. Holds when the final letter is
  round or open (e, o, c, a); stops holding on a full-height vertical (l, k, t, d).
- [`a-radial-mark-between-two-glyph-collisions.md`](../design/a-radial-mark-between-two-glyph-collisions.md)
  (`kb-design-radial-mark-collisions-001`) — a ring of radial elements sits between the settings
  gear and the near-universal eight-fold brightness glyph; an odd count (seven) defeats both,
  verified by rasterising at 16 and 32px rather than by reasoning about the vector. The defence is
  reasoned and rendered but not yet tested on a stranger.

**Open questions** — see [`open-questions-index.md`](open-questions-index.md) for the full,
self-contained list. The one this domain owns:
`kb-open-question-trademark-search-001` (the trademark search on "happenstance" has not been run;
the anti-appropriation lever the commercial layer rests on is trademark, not copyright, since
Apache-2.0 §6 grants no trademark rights — gates filing, registration and physical application,
not the identity's continued use in software today).

## Adding a domain

Append a new `##` section rather than editing this one — a domain is a
subject area, not a wave, and sections should outlive the ingest that first
populated them. Group entries within a section by kind, cite each atom's id
next to its link, and keep the orientation line to one sentence: the atom
itself is the source of truth, not this map.
