# Wave `2026-09-07-intake` — integration summary

Fifty-six staged files (eight standalone briefs plus a 48-file `remediation-2026-09-04-briefs/`
batch), 91 operations, 74 atoms created, 18 existing atoms mutated (17 merges plus one
supersession), three map atoms updated, and reciprocal `related` edges wired into 21 further
existing atoms that named a new atom but were themselves not the destination of any operation.
Full per-operation reasoning is in `02-placement-and-adjudication.md`; `00-corpus-match.md`
establishes the corpus state and scoring; `01-claims-and-classification.md` carries the
per-file claim extraction. This is the roll-up.

## Atoms created (74)

### Reference (8) — Ops 1–8

| Atom | From |
| --- | --- |
| `.kb/reference/position-visibility-adapter-remeasurement-2026-09.md` | `2026-09-07-adr-0024-position-visibility-mechanism.md` |
| `.kb/reference/mutation-coverage-arm-two-measurement-2026-09.md` | `es-22-arm-two-is-reached-the-finding-is-wrong.md`, `f2-5-holds-the-release-for-phase-10.md` |
| `.kb/reference/one-connection-latency-2026-09.md` | `sqlite-blocking-seam.md`, `read-page-budget-rows-bytes-or-caller.md` |
| `.kb/reference/shipped-append-condition-sql-experiment-2026-09.md` | `append-condition-sql-shape.md` |
| `.kb/reference/intake-citation-drift-census-2026-09.md` | `docs-citation-form-and-clause-content.md` |
| `.kb/reference/model-family-case-count-cliff-2026-09.md` | `model-family-case-floor.md` |
| `.kb/reference/spec-trace-unresolved-rule-declarations-2026-09.md` | `prose-guard-retired-and-what-it-owes.md` |
| `.kb/reference/phase-7-macros-ceremony-second-example-2026-09.md` | `adr-0033-reopen-ground.md` |

### Decisions (23) — Ops 9–31, ADR-0024 and ADR-0038 … ADR-0059

`.kb/decisions/` gains twenty-three atoms in one wave — its largest single-wave addition — closing
the numbering gap at ADR-0024 (reserved since phase 10's postgres-adapter split, per `02`'s
Adjudication 1) and running the sequence forward to ADR-0059:

`0024-position-visibility-mechanism.md`, `0038-async-trait-through-testcontainers.md`,
`0039-es-42-frozen-without-an-unpin-bound.md`, `0040-f2-5-holds-the-release-for-phase-10.md`,
`0041-the-repository-is-public-and-security-has-an-email-channel.md`,
`0042-every-fixture-capability-lands-defaulted.md`,
`0043-event-metadata-gets-a-refusal-channel-with-no-floor.md`,
`0044-a-published-crate-re-exports-its-drivers.md`,
`0045-a-citation-anchor-matches-exactly-or-the-lint-refuses.md`,
`0046-commit-reports-a-nothing-to-do-outcome.md`, `0047-tags-and-scope-must-agree-at-commit.md`,
`0048-op-read-is-non-exhaustive.md`, `0049-a-codec-declares-the-tags-it-reads.md`,
`0050-stringified-throw-is-crate-private.md`,
`0051-declension-by-inheritance-discharges-cf-18.md`,
`0052-the-query-partition-constants-stay-public.md`,
`0053-a-read-page-is-bounded-in-rows-and-in-bytes.md`,
`0054-after-opt-applies-to-every-guard-and-says-so.md`,
`0055-append-keeps-its-borrowed-batch-at-0-2-0.md`,
`0056-wf-10s-rule-line-is-repaired-not-amended.md`, `0057-the-testkit-version-key-is-dropped.md`,
`0058-the-sqlite-write-path-stays-inline-and-documents-it.md`,
`0059-the-domain-event-guard-checks-positional-agreement.md`.

Three carry amendment edges rather than fresh ground: ADR-0038 amends ADR-0001 and ADR-0035
(`async_trait` stays forbidden; the testcontainers exception is scoped to dev-dependencies only),
and ADR-0043 amends ADR-0015 (the metadata floor question gets a refusal channel, not a size
limit). **No supersession edges** were signed against any accepted decision — every one of the
twenty-three is either wholly new ground or an amendment that leaves the amended atom's body and
`status` untouched, matching this corpus's three-wave streak of never flipping an accepted
decision's status (`02`, "What the Maps phase inherits").

### Concepts (2) and playbooks (2) — Ops 32–35

| Atom | Kind | From |
| --- | --- | --- |
| `.kb/concepts/a-mutation-kind-needs-a-tethered-bar.md` | concept | `stated-only-defects-and-the-reopen-must.md` |
| `.kb/concepts/growing-a-sealed-trait-is-not-a-breaking-change.md` | concept | `tuple-boundary-event-type.md` |
| `.kb/playbooks/a-count-or-an-index-nobody-re-derives.md` | playbook | `2026-09-07-ratifications-discharged-and-what-execution-changed.md`, `ratifications-2026-09-06-pre-publication.md` |
| `.kb/playbooks/require-the-property-not-the-mechanism.md` | playbook | `typed-layer-promise-guard.md` |

### Open questions (38 new + 1 superseding) — Ops 36–73 and Op 74

Thirty-eight new open-question atoms — this corpus's largest single-wave addition to that layer,
more than doubling the prior record (six, set 2026-09-04) — plus one that both creates and
supersedes: `.kb/open-questions/is-the-dagger-convention-superseded-by-maturity-markers.md`
(`kb-open-question-dagger-convention-vs-maturity-markers-001`) supersedes
`.kb/open-questions/no-ps-rule-name-is-resolved.md`, which described a loop-guard mechanism the
tree no longer has (`02`, Adjudication citing `prose-guard-retired-and-what-it-owes.md`).

The thirty-eight (destination basenames, all under `.kb/open-questions/`):
`off-poll-adapter-visibility-defect-undetected.md`, `cf-5-conformant-control-per-rule-or-per-
branch.md`, `msrv-ratification-conflicts-with-the-accepted-floor.md`,
`postgres-fixture-read-fault-declension-is-owed.md`, `read-fault-rule-has-no-clause.md`,
`cf-18-residuals-after-declension-by-inheritance.md`, `cf-23-emitter-names-mandatory-and-marked-
unstable.md`, `cf-25-cf-26-portfolio-check-does-not-exist.md`, `gate-step-first-check-hides-its-
second.md`, `vt-30-provisional-marker-is-stale-and-unscheduled.md`, `adapter-version-lockstep-and-
cf-32.md`, `happenstance-facade-does-not-match-adr-0006.md`, `stale-0-0-0-name-reservations.md`,
`event-type-positional-mapping-has-no-compiler-check.md`, `references-adr-in-place-correction-
policy.md`, `cf-17-cf-14-maturity-markers-and-the-reopen-must.md`, `model-only-kind-memberless-
dormant-or-withdrawn.md`, `read-to-backwards-limit-composition-gap.md`, `cf-33-cf-34-scope-
outside-the-testkit.md`, `es-23-frozen-doc-musts-adapter-half.md`, `cf-38-case-naming-no-clause-
reading.md`, `es-18-byte-identical-versus-conformance-reading.md`, `probe-read-through-signature-
and-live-transaction-seam.md`, `projection-batch-sql-seam-statement-type.md`, `projection-runner-
chunk-type-and-observation-seam.md`, `reset-refusal-declension-has-no-clause.md`, `testkit-
projection-module-unstable-projection-exemption-scope.md`, `tuple-boundary-heterogeneous-event-
type.md`, `trait-variant-caret-resolves-past-the-locked-gate.md`, `sole-evidence-pin-requirement-
generality.md`, `what-the-exact-anchor-rule-still-leaves-open.md`, `docs-citation-anchor-form-and-
clause-contradiction-check.md`, `es-11-ceiling-sample-cost-on-sqlite-read.md`, `then-empty-
emission-idiom-and-the-nothing-to-do-channel.md`, `scope-coverage-helper-and-the-projection-port-
gap.md`, `cloudflare-worker-feature-gate.md`, `rustdoc-citations-relative-or-url-shaped.md`,
`should-codec-be-sealed.md`.

One deferral is folded into this set structurally rather than by a distinct op: `msrv-
ratification-conflicts-with-the-accepted-floor.md` is the `defer_open_question` disposition named
in the wave brief — a ratification brief asked to move the MSRV again, and this wave declines to
settle it in passing, recording the conflict against ADR-0029's accepted floor instead of
resolving it (per `.kb/open-questions/README.md`'s deferral contract).

## Atoms mutated (18)

**One supersession** — `.kb/open-questions/no-ps-rule-name-is-resolved.md`
(`kb-open-question-no-ps-rule-name-resolved-001`): `status: accepted` → `status: superseded`,
`superseded_by: kb-open-question-dagger-convention-vs-maturity-markers-001` added. Body kept
verbatim.

**Seventeen merges** (`merge_existing`, Ops 75–91) — each extended with a dated subsection rather
than reworded, per this corpus's merge discipline:

- `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` — a second citation-drift
  instance folded in from nine source files at once, the largest single merge this playbook has
  taken.
- `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md` — extended from
  `cf-36-thirteen-recorded-breaches.md`.
- `.kb/governance/what-may-refute-a-finding.md` — extended from `f2-5-holds-the-release-for-
  phase-10.md`, `domain-event-guard-and-decode.md` and `es-22-arm-two-is-reached-the-finding-is-
  wrong.md`.
- `.kb/open-questions/poll-count-bounds-the-visibility-rule.md` — **resolved**
  (`2026-09-06-poll-count-calibrated-and-a-second-limitation.md`).
- `.kb/open-questions/postgres-arm-c-structural-cost.md` — **resolved**
  (`2026-09-07-adr-0024-position-visibility-mechanism.md`).
- `.kb/open-questions/event-metadata-has-no-declared-floor.md` — **resolved**
  (`event-metadata-floor.md`, `ratifications-2026-09-06-pre-publication.md`).
- `.kb/open-questions/query-plan-parameter-chunking-incomplete.md` — **resolved**
  (`query-partition-public-surface.md`, `append-condition-sql-shape.md`).
- `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md` — extended from
  `2026-09-07-adr-0024-position-visibility-mechanism.md`.
- `.kb/open-questions/adr-0022-falsifiers-have-fired.md` — extended from
  `append-condition-sql-shape.md`, `transient-contention-tolerance.md`, `sqlite-blocking-seam.md`.
- `.kb/open-questions/no-fixture-tolerance-for-transient-contention.md` — extended from
  `transient-contention-tolerance.md`.
- `.kb/open-questions/cf-36-names-a-cross-reference-nothing-performs.md` — extended from
  `cf-36-thirteen-recorded-breaches.md`, `stated-only-defects-and-the-reopen-must.md`.
- `.kb/open-questions/es-17-two-adapter-measurement-is-unscheduled.md` — extended from
  `append-batch-ownership.md`.
- `.kb/open-questions/read-page-budget-is-unspecified.md` — extended from
  `read-page-budget-rows-bytes-or-caller.md`, `sqlite-lane-spec-citation-repoints.md`.
- `.kb/open-questions/es-6-names-an-unwritable-rule.md` — extended from `prose-guard-retired-and-
  what-it-owes.md`, `stated-only-defects-and-the-reopen-must.md`, `adapter-driver-reexport-
  policy.md`.
- `.kb/open-questions/disjoint-boundaries-have-no-clause.md` — extended from
  `sole-evidence-pins-and-moved-file-citations.md`.
- `.kb/open-questions/model-family-rule-has-no-clause.md` — extended from
  `op-read-non-exhaustive.md`, `model-family-case-floor.md`.
- `.kb/open-questions/no-workerd-class-runner-in-the-gate.md` — extended from
  `query-partition-public-surface.md`.

Four of the seventeen flip `status: accepted` → `status: superseded` on resolution (poll-count,
postgres-arm-c, event-metadata-floor, query-plan-chunking), each keeping its body verbatim under an
appended `## Resolved 2026-09-0…` section — the same resolution-by-amendment convention this
corpus has used for three prior waves running.

## Map atoms updated (3)

- `.kb/maps/decision-map.md` — 23 new rows (ADR-0024 landed in its reserved slot; ADR-0038 through
  ADR-0059 appended in order), plus the three amendment edges (0038→0001, 0038→0035, 0043→0015).
  No supersession edges drawn.
- `.kb/maps/domain-map.md` — every decision-bearing domain touched: contract ports (0043, 0048,
  0054, 0055, 0056), the typed layer (0046, 0047, 0049, 0050, 0059), adapters (0024, 0052, 0053,
  0058), the conformance suite (0042, 0051), the gate (0038, 0045, 0057), and publication (0039,
  0040, 0041, 0044).
- `.kb/maps/open-questions-index.md` — 39 bullets added (the 38 new open questions plus the
  supersession target's replacement), 4 flipped to **Resolved** (poll-count, postgres-arm-c,
  event-metadata-floor, query-plan-chunking), 1 flipped to **Superseded** with its successor named
  (no-ps-rule-name), and 10 existing rows annotated in the index's running-commentary style.

## Reciprocal links wired

Per the corpus's outbound-only authoring convention, the Maps phase wired return edges into 21
existing atoms that a new or merged atom named but that were not themselves an operation's
destination: `.kb/concepts/torn-reads-and-the-append-condition-boundary.md`,
`.kb/governance/rewrite-the-referent-never-the-reasoning.md`,
`.kb/open-questions/cf-40-fixture-limits-ownership.md`,
`.kb/open-questions/d-1-the-validated-type-has-no-total-path.md`,
`.kb/open-questions/deny-bans-red-on-the-worker-dependency.md`,
`.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`,
`.kb/open-questions/es-7-and-vt-9-provisional-markers.md`,
`.kb/open-questions/projection-store-batch-has-no-apply-seam.md`,
`.kb/open-questions/ps-19-scope-narrower-than-its-rule.md`,
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`,
`.kb/playbooks/verify-the-referent-and-report-coverage.md`, and eight `.kb/reference/` atoms
(`append-condition-experiment-2026-08.md`, `busy-timeout-margin-2026-09.md`,
`event-clone-allocations-and-layout-2026-09.md`, `nested-block-on-lost-wakeup-2026-09.md`,
`phase-4-5-specification-reconciliation-census.md`, `phase-7-macros-ceremony-measurement.md`,
`phase-8-specification-reconciliation-census.md`, `port-traits-compiled-findings.md`,
`position-visibility-experiment-2026-08.md`, `spec-trace-has-suite-family-switch.md`). Each gains
only its `related` list; no other frontmatter key or body sentence changed.

## Post-integration repair (this stage)

`redkiln validate --kb`'s first pass failed on 24 dangling `related` references across 15
decision atoms, 2 playbook atoms and one reference atom — every one a hand-typed candidate id that
did not match the id the referenced atom actually carries (for example
`kb-open-question-cf-40-fixture-limits-001` where the real atom's id is
`kb-open-question-cf-40-ownership-001`, an atom that predates this wave and was itself resolved by
ADR-0034). Each was corrected in place to the atom's real `id` — no atom was created, renamed, or
had its body reworded to fix these; see `04-retrospective.md` for the full before/after list.
`redkiln validate --kb` exits 0 on the corrected tree.
