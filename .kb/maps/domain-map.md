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
  new open question on the unrun trademark search. The 2026-09-04 wave (`2026-09-04-intake`), the
  pre-publication review, added ADR-0037 (phase 12) to the ports domain along with three new
  reference atoms (event-clone allocation cost, the busy-timeout margin at 64 contenders, and the
  testkit's own nested-block_on lost-wakeup), a new playbook (assert a test's execution, not its
  discovery) and a new governance atom (what may refute a finding) — and six new open questions:
  event metadata's undeclared floor, the read page-size budget, no fixture tolerance for transient
  contention, the remint-identity precondition being trust-only, incomplete parameter chunking on
  the query-plan arm axis, and whether ADR-0022's own three falsifiers have fired. It also flipped
  `kb-open-question-adapter-default-projection-feature-001` to superseded there — resolved by a
  direct manifest fix in both adapter crates rather than by an answering decision atom — and
  amended `kb-playbook-anchoring-citations-001` in place with a second dated section rather than
  moving it, per the merge-over-create bias. The 2026-09-07 wave (`2026-09-07-intake`) added
  twenty-three decision atoms across three existing domains and opened a fourth. The ports domain
  gained fifteen (ADR-0024, ADR-0038 through ADR-0040, ADR-0042, ADR-0043, ADR-0048, ADR-0050
  through ADR-0056, ADR-0058), four new reference atoms (a postgres remeasurement, a
  mutation-coverage arm-two measurement, sqlite's one-connection latency under contention, and the
  shipped adapter's append-condition SQL shape), and a new concept — a mutation `Kind` needs a bar
  tethered to a real observed/control pair, the lesson `Kind::StatedOnlyDefect`'s withdrawal left
  behind. The typed layer domain gained four decisions (ADR-0046, ADR-0047, ADR-0049, ADR-0059),
  its first concept (growing a sealed trait's own member set is not a breaking change, correcting
  `boundary.rs`'s own doc comment) and first playbook (require the property a guard exists to
  enforce, not the mechanism that happens to spell it today), plus a second macros-ceremony
  reference atom. Specification governance & conformance gained one decision (ADR-0045), three
  reference atoms (an intake-wide citation-drift census, the `PROPTEST_CASES` cliff a model-family
  clause would be quantified over, and spec-trace's now-declared unresolvable rule-name
  declarations) and one playbook (a count or an index nobody re-derives). A new domain,
  "Publication and release readiness," opened for three decisions about the crate boundary a
  publish draws rather than about a port or the typed layer (ADR-0041, ADR-0044, ADR-0057).
  Thirty-nine new open questions were filed and four existing ones flipped to superseded
  (`kb-open-question-poll-count-rule-strength-001` by ADR-0024's own successor question,
  `kb-open-question-postgres-arm-c-cost-001` and `kb-open-question-event-metadata-no-floor-001` by
  ADR-0024 and ADR-0043 respectively, and `kb-open-question-query-plan-parameter-chunking-001` by
  ADR-0052/ADR-0053 together with the shipped-SQL reference atom); see
  [`open-questions-index.md`](open-questions-index.md) for the full list rather than repeating it
  here. The 2026-09-09 wave (`2026-09-09-intake`) added three decisions to the ports domain —
  ADR-0025 (phase 11, the Ladybug graph-projection adapter: a checkpoint node, raw Cypher, a
  blocking driver), ADR-0060 (phase 11, PS-2's batch-shape bar re-evaluated against four adapters
  and kept on a replaced reason) and ADR-0061 (phase 10, ES-11's asynchronous-driver sufficiency
  condition narrowed after `happenstance-neon` falsified it) — plus one new reference atom (the
  four Ladybug driver probes ADR-0025 could not be written without) and one new open question
  (whether any one-shot-HTTP shape, not only `happenstance-neon`, can satisfy ES-11). It also filed
  a second new open question, on `.gitignore`'s `*-output.txt` rule eating the one experiment whose
  README cites its raw output by name, to "Specification governance & conformance". No new domain
  opened. Four existing atoms were amended in place rather than superseded —
  `kb-governance-referent-not-reasoning-001` and `kb-governance-what-may-refute-a-finding-001` each
  gained a new worked instance from this wave's two re-evaluations, `kb-playbook-repair-frozen-clause-001`
  gained the hardest edge of its own mechanical test (a correction that leaves a `MUST` untouched
  and is still an amendment), and `kb-playbook-count-or-index-nobody-re-derives-001` gained a third
  defect shape, a status claim a fired falsifier made false with no number in it — none of the
  four bullets these atoms already carry below was rewritten, since each one-line orientation still
  holds.
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
  - .kb/_governance/integration-waves/2026-09-04-intake
  - .kb/_governance/integration-waves/2026-09-07-intake
  - .kb/_governance/integration-waves/2026-09-09-intake
last_reviewed: 2026-09-09
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

**Decisions**

- [`0045-a-citation-anchor-matches-exactly-or-the-lint-refuses.md`](../decisions/0045-a-citation-anchor-matches-exactly-or-the-lint-refuses.md)
  (`kb-decision-0045`) — added 2026-09-07. `lint_constitution.rs`'s citation checker now requires
  an exact line match; `ANCHOR_SLACK`'s twelve-line tolerance on `spec-trace`'s derived citations
  is left as a separate, undischarged residual.

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
- [`intake-citation-drift-census-2026-09.md`](../reference/intake-citation-drift-census-2026-09.md)
  (`kb-reference-intake-citation-drift-census-001`) — a census of pre-ingest citation drift across
  `.kb/_intake`: 699 citations checked, 77 drifted, 20 into `references/`, 57 still live, 25
  recoverable. A third instance of the same defect class, over the staging directory rather than
  the specification or the constitution. Added 2026-09-07.
- [`model-family-case-count-cliff-2026-09.md`](../reference/model-family-case-count-cliff-2026-09.md)
  (`kb-reference-model-family-case-cliff-001`) — `MODEL_COVERAGE`'s per-store table is true only at
  `PROPTEST_CASES >= 192` (a cliff between 176 and 192, tied to the `to` generator change); the
  default of 256 carries a 1.33x margin. Names what a model-family clause would have to be
  quantified over. Added 2026-09-07.
- [`spec-trace-unresolved-rule-declarations-2026-09.md`](../reference/spec-trace-unresolved-rule-declarations-2026-09.md)
  (`kb-reference-spec-trace-unresolved-declarations-001`) — after `schedules_new`'s retirement, the
  45 rule-name declarations `spec-trace`'s check 4 now resolves against: 26 `Elsewhere`, 4
  `NotARuleName`, 15 `Scheduled`. Added 2026-09-07.

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
- [`a-count-or-an-index-nobody-re-derives.md`](../playbooks/a-count-or-an-index-nobody-re-derives.md)
  (`kb-playbook-count-or-index-nobody-re-derives-001`) — a stale bare count (CLAUDE.md's
  "seventeen atoms," `ci.yml`'s "101") and a curated index mistaken for a completeness proof share
  one repair: drop the count where a command answers it, use a floor not an equality assertion, and
  verify coverage independently of a curated index. Added 2026-09-07.

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
atom records fixed, is what still leaves every `PS` rule name unresolved. **Superseded** 2026-09-07
by `kb-open-question-dagger-convention-vs-maturity-markers-001`: `schedules_new` is retired and
zero clause `Rule:` lines carry a hand-authored dagger — the premise no longer describes anything in
the file — so the successor restates the question against the generated `†` marker section 7.2 now
emits),
`kb-open-question-dagger-convention-vs-maturity-markers-001` (added 2026-09-07, superseding the
entry above — does a generated `†` say anything `UNRESOLVABLE_RULE_NAMES`'s three kinds
(`Elsewhere`, `NotARuleName`, `Scheduled`) do not),
`kb-open-question-cf-38-case-naming-no-clause-001` (added 2026-09-07 — whether CF-38's fourth
condition means "a case no clause claims" (checked, green) or "a case body naming a clause," the
reading 54 of 58 E2E cases fail and CF-37's own text asks for),
`kb-open-question-gate-step-first-check-hides-001` (added 2026-09-07 — `lints::stated_rule_counts`
bundles three checks in one pass; the ordering/bundling question, scoped narrowly, since the V-6
anchor-derivation half of the same source brief routes to the anchoring-citations playbook
instead),
`kb-open-question-exact-anchor-residue-001` (added 2026-09-07 — three residues ADR-0045's own
ratification left open: `spec_trace.rs`'s undocumented `ANCHOR_SLACK = 12`, no rule requiring
anchor uniqueness, and no shipped idempotent `--repoint`),
`kb-open-question-docs-citation-anchor-contradiction-001` (added 2026-09-07 — three stacked
defects in `docs/append-conditions.md`: a citation of ES-40 that contradicts it, a dead `file:line`
landing mid-paragraph, and an unrecognised bare `:NNN` shorthand; a mechanical
contradiction-detector is judged impossible),
`kb-open-question-sole-evidence-pin-generality-001` (added 2026-09-07 —
`the_shotgun_mutants_sole_coverage_is_pinned` only pins REGISTRY's one shotgun mutant, while a
crude regex found roughly sixteen rules sharing the same sole-evidence hazard CF-1 exists to
police, most unpinned),
`kb-open-question-references-adr-correction-policy-001` (added 2026-09-07 — CLAUDE.md never states
whether `references/adr/`'s full-length records may be corrected in place; grounded in a live
falsified figure at `references/adr/0012-append-shape-and-preconditions.md:172-174`),
`kb-open-question-experiment-raw-output-ignored-001` (added 2026-09-09 — a wave-discovered gap
rather than one sourced from either staged intake file: `.gitignore:69`'s `*-output.txt` pattern,
written at the pre-publication sweep to catch stray transcripts carrying an absolute path, also
eats `experiments/ladybug-driver-probes/results/probe-output.txt`, the one raw output a README
cites by name and no other tracked experiment result happens to be named to collide with).

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
instead behind the off-by-default `unstable-projection` feature. The 2026-09-04 wave added a tenth,
`.kb/decisions/0037`, phase 12: the pre-publication review's MSRV atom, extending the
`kb-decision-0004` → `kb-decision-0029` amendment lineage by one more link rather than superseding
either — `0.2.0` turns the 1.97.1 floor from a self-imposed constraint into a promise a published
consumer relies on, without moving the number. The 2026-09-07 wave added fifteen more: ADR-0024
(phase 10, filling the number RUNBOOK.md had reserved — `happenstance-postgres` buys position
visibility with `xid8` and `pg_snapshot_xmin`), ADR-0038 (phase 10, `async-trait` exempted a
second time, through `testcontainers`/`tonic`), ADR-0040 (phase 10, the same-day F2-5 hold and
discharge), and twelve phase-12 pre-publication ratifications settling the port and the testkit
(ADR-0039, ADR-0042, ADR-0043, ADR-0048, ADR-0050, ADR-0051, ADR-0052, ADR-0053, ADR-0054,
ADR-0055, ADR-0056, ADR-0058). None supersedes a row on this map. The full decision list, including
status and supersession, is [`decision-map.md`](decision-map.md) rather than repeated here.

The 2026-09-09 wave added an eleventh, twelfth and thirteenth: `.kb/decisions/0025`,
`.kb/decisions/0060` and `.kb/decisions/0061`. ADR-0025 settles what a projection adapter over a
graph engine with no transaction handle type must do to satisfy PS-1 — a `__hs_checkpoint` node
written as the last statement before `COMMIT`, raw parameterised Cypher rather than a typed
builder, blocking-only behind an off-by-default feature — and is the fifth owned-buffered-batch
implementer rather than PS-2's second shape, `depends_on` `kb-decision-0017` (the owned-`Batch`
shape) and `kb-decision-0030` (the checkpoint progress obligation its node discharges). ADR-0060
re-evaluates PS-2's `[FROZEN]` bar against the real adapter population a second time and keeps the
`unstable-projection` gate on a reason `kb-decision-0036` did not have — `begin`, `probe_write` and
`probe_read_through` are synchronous and infallible for both drivers PS-2 names, so a genuinely
live-transaction batch must declare a false capability and the suite cannot tell it apart from a
buffering one — `depends_on` `kb-decision-0036`, the decision it reaffirms without editing or
superseding. ADR-0061 narrows ES-11's asynchronous-driver sufficiency condition after
`happenstance-neon` failed the concurrent-append rule it names, a repair to the specification rather
than to any decision atom, `depends_on` `kb-decision-0011`. None of the three supersedes a row on
this map.

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
- [`position-visibility-adapter-remeasurement-2026-09.md`](../reference/position-visibility-adapter-remeasurement-2026-09.md)
  (`kb-reference-position-visibility-adapter-remeasurement-001`) — the 2026-09 remeasurement
  against the now-real `happenstance-postgres`: steady-state ratios and a staleness table, carved
  out of ADR-0024's own intake per the reference-layer separation rule. Added 2026-09-07.
- [`mutation-coverage-arm-two-measurement-2026-09.md`](../reference/mutation-coverage-arm-two-measurement-2026-09.md)
  (`kb-reference-mutation-coverage-arm-two-001`) — ES-22's arm 2 (the byte-identical before/after
  snapshot) is reached and passed by two stores registered as mutants for other rules, with no
  `Kind::ConformantVariant` built to exercise it; the measurement F2-5's discharge and CF-5's
  question both rest on. Added 2026-09-07.
- [`one-connection-latency-2026-09.md`](../reference/one-connection-latency-2026-09.md)
  (`kb-reference-one-connection-latency-001`) — `happenstance-sqlite`'s reactor-stall,
  lock-hold/residency and connection-sharing tail-latency tables under contention, the shared
  evidence ADR-0058 and ADR-0053 each rest on. Added 2026-09-07.
- [`shipped-append-condition-sql-experiment-2026-09.md`](../reference/shipped-append-condition-sql-experiment-2026-09.md)
  (`kb-reference-shipped-append-condition-sql-001`) — a remeasurement of ADR-0022's SQL shape
  against the shipped adapter rather than the phase-8 prototype: the chain-vs-aggregate loss
  widens, a most-selective-tag-first assumption in §8 measures backwards, and a fourth,
  boundary-bound arm beats the chain outright in most cells. Added 2026-09-07.
- [`ladybug-driver-probes-2026-09.md`](../reference/ladybug-driver-probes-2026-09.md)
  (`kb-reference-ladybug-driver-probes-001`) — the four driver probes ADR-0025 could not be written
  without: a second `Database::new` on one directory is refused while two `Connection`s over one
  `Arc<Database>` both succeed and observe each other's writes, and `UINT64` round-trips
  `u64::MAX - 1` exactly. Run out of the workspace and out of the gate; the cited raw output is not
  in the tree (`.gitignore:69`'s `*-output.txt`), so the figures are the source README's own
  transcription. Added 2026-09-09.
- [`event-clone-allocations-and-layout-2026-09.md`](../reference/event-clone-allocations-and-layout-2026-09.md)
  (`kb-reference-event-clone-allocations-001`) — the t+2 allocation cost of `Tags::from_pairs`
  versus `Tag::from_static`, the `Bytes::clone` first-clone allocation, and the corrected 24-byte
  `Cow<'static, str>` layout. Added 2026-09-04.
- [`busy-timeout-margin-2026-09.md`](../reference/busy-timeout-margin-2026-09.md)
  (`kb-reference-busy-timeout-margin-001`) — at the shipped `CONTENDERS = 64` the busy-timeout
  margin is 1.3x to 1.4x, not more; fewer cores is not safer and the build profile barely enters,
  both refuting a plausible prediction. Records `busy > 0`, where the 2026-08-16 append-condition
  experiment recorded `busy = 0` at the same contender count. Added 2026-09-04.
- [`nested-block-on-lost-wakeup-2026-09.md`](../reference/nested-block-on-lost-wakeup-2026-09.md)
  (`kb-reference-nested-block-on-lost-wakeup-001`) — `park`/`unpark` coalesce to one token per
  thread, so a `block_on` nested inside another inside the testkit's own executor can lose its
  wakeup and hang past CF-33's no-clock rule, naming no rule at all. A second mechanism with the
  same signature as the pooled-connection deadlock `spec/SPECIFICATION.md` already records. Added
  2026-09-04.

**Concepts**

- [`torn-reads-and-the-append-condition-boundary.md`](../concepts/torn-reads-and-the-append-condition-boundary.md)
  (`kb-concept-torn-read-append-boundary-001`) — why the append condition, derived from the
  read's own observed maximum, cannot catch a torn read; the mechanism ADR-0011, ADR-0012 and
  ADR-0013 each protect without stating.
- [`a-mutation-kind-needs-a-tethered-bar.md`](../concepts/a-mutation-kind-needs-a-tethered-bar.md)
  (`kb-concept-mutation-kind-tethered-bar-001`) — why `Kind::StatedOnlyDefect` was withdrawn: an
  untethered `fn() -> String` observed/control pair is satisfiable by a defect-free subject, and a
  positive-control repair does not close that gap. `Witness`'s tie to `<T as Defect>::select`
  escapes the same flaw; `Fixture` (RPITIT, not dyn-compatible) has no analogue. Added 2026-09-07.

**Governance**

- [`rewrite-the-referent-never-the-reasoning.md`](../governance/rewrite-the-referent-never-the-reasoning.md)
  (`kb-governance-referent-not-reasoning-001`) — the discrimination between a rename that may be
  rewritten in place and reasoning that, once a decision stands, is never touched, worked out
  against ADR-0001 through ADR-0007.
- [`what-may-refute-a-finding.md`](../governance/what-may-refute-a-finding.md)
  (`kb-governance-what-may-refute-a-finding-001`) — the same precedence ladder applied to
  auditing rather than authoring: dated evidence may annotate a finding and never refute one, a
  refutation needs a quoted answering sentence, and an accepted decision's currency is computed
  against `git log`, not assumed. A distinct verb from `kb-governance-referent-not-reasoning-001`
  above — how a record may be edited, versus what may overturn a finding about it — not a merge
  into it. Added 2026-09-04.

**Playbooks**

- [`one-decision-per-adr-title.md`](../playbooks/one-decision-per-adr-title.md)
  (`kb-playbook-one-decision-per-adr-title-001`) — an "and" in a decision's title is usually a
  strong decision and a weaker one bundled together, and the weaker half is the one likely to be
  reversed.
- [`testing-interleavings-with-cold-futures.md`](../playbooks/testing-interleavings-with-cold-futures.md)
  (`kb-playbook-cold-future-hand-polling-001`) — hand-polling two cold futures out of order to
  make a conformance rule observe a specific interleaving with no executor, thread or clock.
- [`assert-a-tests-execution-not-its-discovery.md`](../playbooks/assert-a-tests-execution-not-its-discovery.md)
  (`kb-playbook-assert-execution-not-discovery-001`) — a gate check built on `cargo test --list`
  cannot see a silenced test, since that output is byte-identical with and without `#[ignore]`;
  the repair asserts on the run's own output. Distinguished from
  `kb-playbook-verify-referent-report-coverage-001`'s address/referent pairing rather than merged
  into it — a test suite's discovery and execution are two behaviours of one program, not two ends
  of one reference. Added 2026-09-04.

**Open questions** — see [`open-questions-index.md`](open-questions-index.md) for the full,
self-contained list. The ones this domain owns:
`kb-open-question-adr-status-vocabulary-001`,
`kb-open-question-projection-batch-no-apply-001` (**superseded** 2026-08-13 by
`kb-decision-0017`; sub-question 3 stays open, with the typed layer),
`kb-open-question-query-union-rule-unowned-001`,
`kb-open-question-global-vs-boundary-visibility-001`,
`kb-open-question-postgres-arm-c-cost-001` (**superseded** 2026-09-07 by `kb-decision-0024`: the
shipped adapter measures no discriminating steady-state cost, and the residual moves to
`kb-open-question-off-poll-visibility-defect-001`),
`kb-open-question-poll-count-rule-strength-001` (**superseded** 2026-09-07 by
`kb-decision-0024`, whose own successor question, `kb-open-question-off-poll-visibility-defect-001`,
carries the residual: an off-poll adapter has no suspension point a poll-based schedule can reach),
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
restore off-by-default there the way it did for `happenstance-sqlite`). **Superseded** 2026-09-03:
both crates took the `happenstance-sqlite` shape (a gated `projection-store` feature forwarding to
`happenstance-core/unstable-projection`, the unconditional dependency removed), verified across
eight feature-combination checks; closed by a direct manifest fix rather than by an answering
decision atom, since `kb-decision-0036` already commits the port to an off-by-default gate and two
adapters honouring it settles no fork of its own.
`kb-open-question-event-metadata-no-floor-001` (added 2026-09-04 — `happenstance-core`'s four
`MIN_SUPPORTED_*` limits and three-variant `StoreLimit` enum declare no floor for `Event::metadata`
itself, and no accepted decision states one. **Superseded** 2026-09-07 by `kb-decision-0043`, which
mints a refusal channel with `guaranteed_minimum() == 0` rather than a non-zero floor, chosen
because a separate floor is incommensurable with `happenstance-sync`'s own `payload_len` budget),
`kb-open-question-read-page-budget-001` (added 2026-09-04 — `happenstance-sqlite`'s private
`PAGE_SIZE = 512` is not a port concept, and ADR-0011 settles `read`'s promises without settling
what a page of it should cost),
`kb-open-question-testkit-contention-tolerance-001` (added 2026-09-04 — CF-33 forbids a clock, an
elapsed-time measurement or an operation-count assertion, so a momentarily-busy store and a broken
one surface as the same failed `Attempt`; no longer hypothetical now that `busy > 0` has been
observed at the shipped contender count),
`kb-open-question-remint-precondition-trust-only-001` (added 2026-09-04 — `remint_identity`'s own
test never actually restores or clones a store, so VT-6's `[PROVISIONAL]` marker rests on a
same-process assertion rather than the cross-instance one its text describes),
`kb-open-question-query-plan-parameter-chunking-001` (added 2026-09-04 — the 30,000-parameter
budget is enforced on `write_tag_rows`'s insert path but not proven never to be hit on the
400-arm-per-statement query-chunking path. **Superseded** 2026-09-07 by `kb-decision-0052` and
`kb-decision-0053` together with `kb-reference-shipped-append-condition-sql-001`: the "400 arms is
conservative enough" branch is refuted by measurement — 400 items at `MAX_TAGS_PER_EVENT` is
51,200 bound parameters against SQLite's 32,766 ceiling — and the chunking mechanism now takes a
fourth `per_arm_extra` parameter this question did not describe; `Selectivity::read_for`'s own
unpartitioned failure mode, found in the same pass, is the residual),
`kb-open-question-adr-0022-falsifiers-fired-001` (added 2026-09-04 — two of ADR-0022's own three
named re-open conditions have fired and the third cannot fire as written, and nobody has yet
decided whether the decision is superseded, re-opened, or ratified as still correct with the
firings recorded against it. Amended 2026-09-07 with the shipped-adapter remeasurement's sharper
figures and a corrected reading of `busy > 0`'s rate; stays **open**),
`kb-open-question-msrv-ratification-conflict-001` (added 2026-09-07 — the ratified pre-publication
recommendation to lower the MSRV to 1.95 is absent from the discharge queue, and the 1.88 figure
both `kb-decision-0029` and `kb-decision-0037` state is contradicted by a compiled bisection naming
1.95.0 as the first passing version; filed as a conflict rather than forced to resolve),
`kb-open-question-off-poll-visibility-defect-001` (added 2026-09-07 — an off-poll adapter,
`happenstance-postgres` hopping onto a captured `sqlx` runtime `Handle`, has a real, demonstrated
visibility defect no poll-based schedule can reach, because the port exposes no suspension point a
hand-polled rule can wedge into; successor to `kb-open-question-poll-count-rule-strength-001` and
`kb-open-question-postgres-arm-c-cost-001`, both superseded above),
`kb-open-question-cf-5-per-rule-or-branch-001` (added 2026-09-07 — whether CF-5's
conformant-control obligation is discharged once per rule or once per branch inside a multi-arm
rule; ES-22's arm 2 is the first place the difference is visible),
`kb-open-question-postgres-read-fault-declension-001` (added 2026-09-07 — `PostgresFixture`
inherits a `READ_FAULT` declension that is false about a store whose `PgReadStream` holds a
server-side cursor, the gap ADR-0051's CF-18 discharge found),
`kb-open-question-read-fault-rule-no-clause-001` (added 2026-09-07 — the third entry in
`UNCLAIMED_PENDING_ADR`, mirroring `kb-open-question-disjoint-boundaries-no-clause-001` and
`kb-open-question-model-family-rule-no-clause-001`: a live rule with no owning clause),
`kb-open-question-cf-18-residuals-after-declension-001` (added 2026-09-07 — what ADR-0051's
declension-by-inheritance does not discharge: no SKIP line for a declined capability, ES-35's
`[PROVISIONAL]` marker, and whether an adapter README must disclose declines),
`kb-open-question-cf-23-emitter-names-unstable-001` (added 2026-09-07 — CF-23 requires named
emitters while the shipped surface marks them `doc(hidden)`, a contradiction `cargo-semver-checks`
cannot see because hidden items are exactly what it excludes),
`kb-open-question-cf-25-cf-26-portfolio-check-001` (added 2026-09-07 — no check in
`xtask/src/spec_trace.rs` performs the portfolio/axis comparison CF-25 and CF-26 both name),
`kb-open-question-vt-30-marker-stale-unscheduled-001` (added 2026-09-07 — VT-30's `[PROVISIONAL]`
marker is stale as written now that `happenstance-testkit/src/bench.rs` exists; the real gap
narrows to no multi-guard workload in the harness),
`kb-open-question-cf-17-cf-14-markers-001` (added 2026-09-07 — CF-17 gained a declaration `MUST`
on `REOPEN` without its `[PROVISIONAL]` marker moving; CF-14 is adjacent, `[DEFERRED]`, and
unchanged),
`kb-open-question-model-only-kind-memberless-001` (added 2026-09-07 — `Kind::ModelOnlyMutant` has
gone memberless now that `read_to_composes_with_multi_item_query` landed; three options recorded,
none taken),
`kb-open-question-read-to-backwards-limit-composition-001` (added 2026-09-07 — the unwritten
backwards-plus-window-plus-limit read composition, deliberately unasserted because it would
double-reject three already-registered mutants for reasons three other rules own),
`kb-open-question-cf-33-cf-34-scope-001` (added 2026-09-07 — a timed `#[cfg(test)]` check in
`happenstance-sqlite` sits outside CF-33's stated scope, and a benchmark completion panic sits
adjacent to CF-34's merge-red prohibition; no governing principle reconciles both with the frozen
text as written),
`kb-open-question-es-23-adapter-half-001` (added 2026-09-07 — `FROZEN_DOC_MUSTS` has no recorded
disposition for ES-23's adapter-side `MUST`, a gap two named instruments both failed to catch),
`kb-open-question-es-18-byte-identical-conformance-001` (added 2026-09-07 — whether ES-18's
"byte-identical" second sentence is amended to the weaker conformance reading its own rules ask, or
narrowed to exclude the position counter; specific to `happenstance-cloudflare`'s
compensation-based atomicity, not every counter-assigning store),
`kb-open-question-probe-read-through-signature-001` (added 2026-09-07 — whether
`ProjectionProbe::probe_read_through` moves to `&mut Self::Batch` / async / fallible before
phase 10, corroborating ADR-0036's part-2-unmet finding with a second causal reading),
`kb-open-question-projection-batch-sql-statement-type-001` (added 2026-09-07 — whether
`SqliteBatch::push`'s landed `&'static str` narrowing is the seam's final shape or a minted
`Statement` newtype follows once `ProjectionStore` freezes),
`kb-open-question-projection-runner-chunk-observation-001` (added 2026-09-07 — the chunk type and
default plus the runner's observation seam, both priced at zero code-cost-of-delay by ADR-0036's
exemption),
`kb-open-question-reset-refusal-declension-001` (added 2026-09-07 — `RESET_REFUSAL` has no
CF-39-shaped clause after ADR-0042's retraction, so a fixture can declare it and override
`protect_from_reset` with an empty body, passing vacuously),
`kb-open-question-projection-module-exemption-scope-001` (added 2026-09-07 — ADR-0036's
unstable-projection exemption text names only `happenstance-core` and `happenstance`;
`happenstance-testkit`'s projection module is unconditional and un-exempt as written),
`kb-open-question-trait-variant-caret-001` (added 2026-09-07 — `trait_variant`'s blanket impl is
pinned only by a caret, guarded only by `#[cfg(test)]`, which does not travel to a consumer of the
three already-published crates),
`kb-open-question-es-11-sqlite-ceiling-sample-cost-001` (added 2026-09-07 — ES-11's ceiling sample
stalls 39.3x the idle floor under contention on `happenstance-sqlite`'s read path, a second native
adapter data point against ES-11's `[PROVISIONAL]` marker),
`kb-open-question-then-empty-emission-idiom-001` (added 2026-09-07 — how `then(&[])` reads once
`commit`'s outcome is two-armed, and whether `decide` can express "nothing to do" distinctly from
"refused"),
`kb-open-question-scope-coverage-helper-projection-gap-001` (added 2026-09-07 — the additive,
enum-total `assert_scope_covered` test helper, and the identical unchecked tags/scope pair on the
projection port ADR-0047's fix does not reach),
`kb-open-question-one-shot-http-es-11-001` (added 2026-09-09 — ADR-0061 narrowed ES-11's
asynchronous-driver sufficiency condition and recorded that `happenstance-neon` does not satisfy it;
open is whether any one-shot-HTTP shape can, since the transport offers exactly one ordering
primitive and nothing else in a pooled-proxy path orders one backend's snapshot against another's
commit).

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
- [`0046-commit-reports-a-nothing-to-do-outcome.md`](../decisions/0046-commit-reports-a-nothing-to-do-outcome.md)
  (`kb-decision-0046`) — added 2026-09-07. `commit` gains an outcome distinct from "refused" for a
  `decide` that emits nothing to append; three of five in-tree call sites needed the new arm.
- [`0047-tags-and-scope-must-agree-at-commit.md`](../decisions/0047-tags-and-scope-must-agree-at-commit.md)
  (`kb-decision-0047`) — added 2026-09-07. An under-tagged model — one whose `DomainEvent::tags`
  does not cover its own `Boundary`'s scope — is refused at commit rather than silently admitted.
- [`0049-a-codec-declares-the-tags-it-reads.md`](../decisions/0049-a-codec-declares-the-tags-it-reads.md)
  (`kb-decision-0049`) — added 2026-09-07. `Codec::reads_tag` is a declared, per-codec,
  orphan-rule-bounded obligation — narrower than "a codec declares the tags it reads" sounds.
- [`0059-the-domain-event-guard-checks-positional-agreement.md`](../decisions/0059-the-domain-event-guard-checks-positional-agreement.md)
  (`kb-decision-0059`) — added 2026-09-07. Named-const doctests and examples plus an xtask lint
  land now; a positional-agreement precondition on `assert_domain_event` is owed before `0.2.0`.

**Reference**

- [`phase-7-macros-ceremony-measurement.md`](../reference/phase-7-macros-ceremony-measurement.md)
  (`kb-reference-macros-ceremony-measurement-001`) — the 29-range, line-by-line ceremony/domain
  classification of `examples/course-subscriptions/src/main.rs` that `kb-decision-0033` rests on.
  Added 2026-08-17.
- [`phase-7-macros-ceremony-second-example-2026-09.md`](../reference/phase-7-macros-ceremony-second-example-2026-09.md)
  (`kb-reference-macros-ceremony-second-example-001`) — the ceremony ratio remeasured against a
  second worked example, `transfers-on-sqlite`, alongside `course-subscriptions` at a later commit:
  6.3% combined, strengthening ADR-0033 rather than reopening it. Added 2026-09-07.

**Concepts**

- [`growing-a-sealed-trait-is-not-a-breaking-change.md`](../concepts/growing-a-sealed-trait-is-not-a-breaking-change.md)
  (`kb-concept-sealed-trait-growth-001`) — corrects `boundary.rs`'s own doc comment, which states
  the sealed-trait growth rule backwards: RS-40-1's `E0046` hazard cannot apply to a sealed trait,
  since no downstream impl exists to be missing an item. The two real residuals are
  method-resolution ambiguity from a new name and a newly nameable associated type. Added
  2026-09-07.

**Playbooks**

- [`require-the-property-not-the-mechanism.md`](../playbooks/require-the-property-not-the-mechanism.md)
  (`kb-playbook-require-the-property-001`) — an xtask assertion and four in-crate tests all
  required the literal `pub use happenstance_core::*` glob by name rather than the property
  contract items resolve only through their own gates, so ten ungated names leaked under
  `cargo test` while every check stayed green. Added 2026-09-07.

**Open questions** — see [`open-questions-index.md`](open-questions-index.md) for the full,
self-contained list. The ones this domain owns:
`kb-open-question-d-1-no-total-path-001` (added 2026-08-17 — `QueryItem::new` is fallible even over
already-validated inputs and `DomainEvent::tags` is total over a fallible `Tags`; no infallible
route exists in either direction, and it is the single condition that would reopen ADR-0033's
verdict),
`kb-open-question-event-type-positional-mapping-001` (added 2026-09-07 — the worked examples'
`EVENT_TYPES[n]` mapping from a fold position to a decoder is unchecked by the compiler in both
`course-subscriptions` and `transfers-on-sqlite`; filed rather than amending ADR-0033, which the
new ceremony measurement strengthens instead),
`kb-open-question-tuple-boundary-event-type-001` (added 2026-09-07 — `composition.rs`'s
macro-generated tuple impls bind every member to the first member's `Event` type, unwritten in
ADR-0020's decision text or the signed-off design),
`kb-open-question-seal-the-codec-001` (added 2026-09-07 — whether `Codec` is later sealed now that
`0.2.0` is live and the window to do so for free has closed; bundles the `UnknownTag`-split and
`Boundary::absorb` sub-questions ADR-0049 left undone).

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
- [`standalone-svg-carries-one-colourway.md`](../decisions/standalone-svg-carries-one-colourway.md)
  (`kb-decision-standalone-svg-one-colourway-001`) — a standalone SVG carries one fixed colour;
  `prefers-color-scheme` is selected at the point of use, not embedded in the file. Shares its
  `SD-0002` **source** identifier with the mark decision above — both derive from that one design
  decision — but not its filename: two files named `sd-0002-*` in one directory read as one atom
  split in two, so the file is named for its own id and the SD column carries the provenance. The
  sharing and the scope split (this rule reaches past logos to any SVG shipped for a surface it does
  not control) are recorded in both
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

## Publication and release readiness

The area concerned with the crate boundary a publish draws — what a published crate's manifest and
public surface owe a consumer that neither a port decision nor a typed-layer decision is about.
Established by the 2026-09-07 wave from the pre-publication review's remediation queue. Distinct
from "Contract ports, conformance, and the ADR corpus": that domain is about `happenstance-core`'s
design and the ADR corpus it rests on, while this one is about what changes the moment a crate is
`cargo add`-able — a re-export policy, a public repository, a dropped dev-dependency version key.

**Decisions**

- [`0041-the-repository-is-public-and-security-has-an-email-channel.md`](../decisions/0041-the-repository-is-public-and-security-has-an-email-channel.md)
  (`kb-decision-0041`) — the repository went public in two stages: `SECURITY.md` and an email
  channel landed first, independent of publication, then the repository itself followed a costed
  pre-publication sweep.
- [`0044-a-published-crate-re-exports-its-drivers.md`](../decisions/0044-a-published-crate-re-exports-its-drivers.md)
  (`kb-decision-0044`) — a published crate re-exports any crate whose type appears in one of its own
  public signatures, publication-scoped and necessary-not-sufficient; `tokio` is excluded by owner
  ruling.
- [`0057-the-testkit-version-key-is-dropped.md`](../decisions/0057-the-testkit-version-key-is-dropped.md)
  (`kb-decision-0057`) — `happenstance-testkit`'s resolvable dev-dependency version key is dropped
  from every publishable crate's manifest, with a gate step asserting none carries one back; a
  deliberate exception stands for `examples/outside-projection-adapter`.

**Open questions** — see [`open-questions-index.md`](open-questions-index.md) for the full,
self-contained list. The ones this domain owns:
`kb-open-question-adapter-version-lockstep-001`,
`kb-open-question-facade-does-not-match-adr-0006-001`,
`kb-open-question-stale-0-0-0-name-reservations-001`,
`kb-open-question-cloudflare-feature-gate-001`,
`kb-open-question-rustdoc-citation-form-001`.

## Adding a domain

Append a new `##` section rather than editing this one — a domain is a
subject area, not a wave, and sections should outlive the ingest that first
populated them. Group entries within a section by kind, cite each atom's id
next to its link, and keep the orientation line to one sentence: the atom
itself is the source of truth, not this map.
