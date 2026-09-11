---
id: kb-map-open-questions-index-001
title: Open-questions index
kind: map
status: accepted
authority_tier: note
summary: >-
  One bullet per open_question atom in .kb/open-questions/, grouped by the domain it concerns, so
  an unresolved question is discoverable from the area it belongs to rather than only from the
  directory listing. Updated whenever a defer_open_question disposition lands a new atom, per
  open-questions/README.md's instruction to add a bullet "on the map atom that indexes its area."
  A withdrawn or superseded question stays listed, annotated, rather than removed — the record
  that it was once open is itself worth keeping. The 2026-08-13 wave flipped
  kb-open-question-projection-batch-no-apply-001 to Superseded (by ADR-0017) and annotated the
  PS-1 and PS-19 gaps with the clause-pairing sweep's findings and ADR-0018/ADR-0019's attribution
  of the same defect shape; all three stayed listed rather than being replaced. The 2026-08-15
  wave flipped kb-open-question-ps-1-no-progress-obligation-001 and
  kb-open-question-ps-19-scope-narrower-001 to Superseded (both by ADR-0030, which mints PS-38)
  and added kb-open-question-ps-32-adr-0007-correction-owed-001 (ADR-0007's Context still
  overstates what cannot be written against the port). The 2026-08-17 wave flipped
  kb-open-question-ps-32-adr-0007-correction-owed-001 to Superseded (by ADR-0031, which carries the
  corrected Context riding with its own partial supersession of ADR-0007) and added four new
  questions: kb-open-question-es-17-two-adapter-measurement-001 (the two-build append-ownership
  measurement ADR-0012's falsifier asks for is scheduled by nobody), kb-open-question-d-1-no-total-path-001
  (no infallible route into or out of a validated `QueryItem`/`Tags`), kb-open-question-cf-36-unperformed-cross-reference-001
  (CF-36 names a level-marker cross-reference `spec-trace` does not perform), and
  kb-open-question-no-ps-rule-name-resolved-001 (a bare dagger, not the `has_suite` family switch,
  is what still leaves every `PS` rule name unresolved). The 2026-08-20 wave
  (`2026-08-20-intake-phase-9`) added two new questions —
  kb-open-question-workerd-runner-absent-001 (the Cloudflare conformance suite runs on a
  `node:sqlite` shim, never on `workerd`) and kb-open-question-worker-async-trait-ban-001 (taking
  the real `worker` crate turns `cargo deny check bans` red, and neither the ratify-a-wrapper nor
  the refuse-and-record shape is chosen) — and flipped two existing questions to Superseded:
  kb-open-question-cf-40-ownership-001 (by ADR-0034, `kb-decision-0034`: the fixture contract has
  no single owning document) and kb-open-question-human-readable-encoding-limits-001 (by the WF-11
  memory-ceiling verdict, `kb-reference-wf-11-memory-ceiling-verdict-001`: the condition is not
  constructible on this runtime). kb-open-question-es-6-unwritable-rule-001,
  kb-open-question-poll-count-rule-strength-001 and kb-open-question-post-phase-reconciliation-001
  stayed Open but were each annotated in place with the new wave's findings. The 2026-09-02 wave
  (`2026-09-02-intake`) flipped kb-open-question-worker-async-trait-ban-001 to Superseded (by
  ADR-0035, `kb-decision-0035`, which ratifies a `wrappers` entry for `worker`/`worker-macros`) and
  added two new questions: kb-open-question-adapter-default-projection-feature-001 (ADR-0036 ships
  `ProjectionStore` gated, `happenstance-sqlite` was fixed to stop forwarding the gate through its
  own `default`, and `happenstance-neon`/`happenstance-postgres` have not followed) and
  kb-open-question-trademark-search-001 (no trademark search on "happenstance" has been run, and it
  gates filing, registration and physical application of the brand identity), the latter opening
  this map's first "Brand identity" section. The 2026-09-04 wave (`2026-09-04-intake`), the
  pre-publication review, flipped kb-open-question-adapter-default-projection-feature-001 to
  Superseded — both adapters took happenstance-sqlite's gated-feature shape, closed by a direct
  manifest fix rather than by an answering decision atom — and added six new questions:
  kb-open-question-event-metadata-no-floor-001 (Event::metadata has no declared floor among
  happenstance-core's limits), kb-open-question-read-page-budget-001 (happenstance-sqlite's
  PAGE_SIZE is a private constant, not a port promise), kb-open-question-testkit-contention-tolerance-001
  (the fixture contract has no declared tolerance for a momentarily-busy store, and busy > 0 is no
  longer hypothetical), kb-open-question-remint-precondition-trust-only-001 (remint_identity's own
  test never restores or clones a store), kb-open-question-query-plan-parameter-chunking-001 (the
  30,000-parameter budget is unproven on the query-chunking arm axis), and
  kb-open-question-adr-0022-falsifiers-fired-001 (two of ADR-0022's three named re-open conditions
  have fired and nobody has re-opened it). The 2026-09-07 wave (`2026-09-07-intake`), the
  phase-10/pre-publication remediation intake, added thirty-nine new questions and flipped five
  existing ones to Superseded: kb-open-question-poll-count-rule-strength-001 and
  kb-open-question-postgres-arm-c-cost-001 (both by kb-decision-0024, ADR-0024's own successor
  question — kb-open-question-off-poll-visibility-defect-001 — carrying the residual),
  kb-open-question-event-metadata-no-floor-001 (by kb-decision-0043), kb-open-question-query-plan-parameter-chunking-001
  (by kb-decision-0052/kb-decision-0053 together with the shipped-SQL reference atom), and
  kb-open-question-no-ps-rule-name-resolved-001 (by kb-open-question-dagger-convention-vs-maturity-markers-001,
  since `schedules_new` is now retired and the premise it stated no longer describes anything in
  the tree). A further eight existing questions were amended in place without a status flip:
  kb-open-question-adr-0022-falsifiers-fired-001, kb-open-question-es-17-two-adapter-measurement-001,
  kb-open-question-read-page-budget-001 (partly answered by ADR-0053, the caller-stated-budget half
  left open), kb-open-question-testkit-contention-tolerance-001, kb-open-question-es-6-unwritable-rule-001,
  kb-open-question-disjoint-boundaries-no-clause-001, kb-open-question-workerd-runner-absent-001 and
  kb-open-question-global-vs-boundary-visibility-001, plus two retitled without changing scope:
  kb-open-question-model-family-rule-no-clause-001 (the stale "seven clauses" count dropped) and
  kb-open-question-cf-36-unperformed-cross-reference-001 (the cross-reference it named absent now
  exists; the question moved to what the new check still leaves open). The 2026-09-09 wave
  (`2026-09-09-intake`) added two new questions — kb-open-question-one-shot-http-es-11-001 (ADR-0061
  narrowed ES-11's asynchronous-driver sufficiency condition and recorded that `happenstance-neon`
  does not satisfy it; open is whether any one-shot-HTTP shape can) and
  kb-open-question-experiment-raw-output-ignored-001 (`.gitignore`'s `*-output.txt` pattern eats the
  one experiment result a README cites by name, a wave-discovered gap rather than one sourced from
  either staged intake file) — and amended four existing questions in place without a status flip:
  kb-open-question-probe-read-through-signature-001 (widened from one method to the whole probe seam,
  and ties the sharpened PS-2 bar to ADR-0060's gate), kb-open-question-provisional-falsifiers-001
  (two more markers whose falsifiers never could fire, PS-4 and PS-2, joining ES-7 and VT-9's
  already-fired pair), kb-open-question-es-11-sqlite-ceiling-sample-cost-001 (its third sub-question
  answered by ADR-0061; sub-questions 1 and 2 stay open) and
  kb-open-question-reset-refusal-declension-001 (the window it named has closed — four adapters now
  run the projection suite and all four decline the capability — without answering the question).
  The 2026-09-11 wave (`2026-09-11-intake`) added two new questions — kb-open-question-apply-synchronous-live-store-001
  (ADR-0062 built a live-transaction projection store and ADR-0063 froze the port on that evidence, and
  both deliberately left `Projection::apply`'s still-synchronous signature to the typed layer; the runner
  cannot drive a live batch through it, and neither record owns whether `apply` moves) and
  kb-open-question-immutability-check-pre-commit-001 (`redkiln validate --kb`'s accepted-decision
  immutability check compares the working tree against `HEAD`, so it is a dirty-tree guard that clears the
  moment an edit is committed and cannot tell a referent repair from a reversal — demonstrated by its own
  refusal of `kb-decision-0058`'s citation repointing) — and flipped
  kb-open-question-probe-read-through-signature-001 to Superseded (by ADR-0062, `kb-decision-0062`: the
  whole probe seam moved, `begin` with it, and PS-2's MUST is met as written). Four further questions were
  amended in place without a status flip: kb-open-question-provisional-falsifiers-001 (ADR-0062 rewrote
  PS-6's MUST rather than moving another marker, and the runner-side residual moves to the new
  apply-synchronous question), kb-open-question-projection-module-exemption-scope-001 (ADR-0063's freeze
  narrows the premise: the exemption question dissolves for a port that is no longer gated, and what
  remains is the testkit manifest's mechanical forward), kb-open-question-cf-33-cf-34-scope-001 (a third
  instance — `ops/host/preflight.sh` — joins the sqlite `#[cfg(test)]` check and the benchmark completion
  panic, re-anchored to CF-33/CF-34's live lines) and kb-open-question-stale-0-0-0-name-reservations-001
  (the registry read confirmed: seven crates at `0.0.0`, three additionally at `0.2.0-alpha.1`, nothing
  yanked, and `0.2.0` a first real release for all seven). kb-open-question-docs-citation-anchor-contradiction-001
  gained a dated answer to its Option-C sub-question (re-anchor at promotion, refusing the wave, is the
  chosen remedy) without closing.
depends_on: []
related:
  - kb-map-domain-001
  - kb-map-decision-001
source_paths:
  - .kb/open-questions/README.md
  - .kb/_governance/integration-waves/2026-08-10-intake/02-placement-and-adjudication.md
  - .kb/_governance/integration-waves/2026-08-10-intake-2
  - .kb/_governance/integration-waves/2026-08-13-projection-adrs
  - .kb/_governance/integration-waves/2026-08-15-adr-0030-checkpoint-progress
  - .kb/_governance/integration-waves/2026-08-17-adr-0022-append-condition
  - .kb/_governance/integration-waves/2026-08-20-intake-phase-9
  - .kb/_governance/integration-waves/2026-09-02-intake
  - .kb/_governance/integration-waves/2026-09-04-intake
  - .kb/_governance/integration-waves/2026-09-07-intake
  - .kb/_governance/integration-waves/2026-09-09-intake
  - .kb/_governance/integration-waves/2026-09-11-intake
last_reviewed: 2026-09-11
---

# Open-questions index

Every `open_question` atom, current status first. See
[`../open-questions/README.md`](../open-questions/README.md) for what belongs
in that layer and how a question gets resolved; see
[`domain-map.md`](domain-map.md) for the subject-area grouping this index's
questions sit inside.

## Specification governance & conformance

The first seven questions below were filed by the 2026-08-10 phase 4/5
specification-reconciliation intake wave. Full grounding for each is in the
[census reference atom](../reference/phase-4-5-specification-reconciliation-census.md)
(`kb-reference-phase-4-5-spec-reconciliation-001`); this index states only
what the question is, not its evidence. The last two were added by the
2026-08-17 wave from the phase-7 contract-defect log.

- **Open** — [`disjoint-boundaries-have-no-clause.md`](../open-questions/disjoint-boundaries-have-no-clause.md)
  (`kb-open-question-disjoint-boundaries-no-clause-001`) — the independence
  proposition DCB exists for is enforced by a live conformance rule and
  stated by no clause. Amended 2026-09-07: the rule citation and the
  `UNCLAIMED_PENDING_ADR` range had both drifted since authoring and are repointed to their live
  lines; the array now holds three entries, not two — the read-fault gap
  (`kb-open-question-read-fault-rule-no-clause-001`) joins this one and the model-family gap below.
- **Open** — [`model-family-rule-has-no-clause.md`](../open-questions/model-family-rule-has-no-clause.md)
  (`kb-open-question-model-family-rule-no-clause-001`) — the model-based
  rule checks a composition of clauses and belongs to none of them (retitled 2026-09-07: the count
  had already drifted once, exactly the defect `kb-playbook-count-or-index-nobody-re-derives-001`
  names). Amended 2026-09-07: ES-16 joins the composed set now that the `to` generator field
  landed, and the composition's truth is case-count dependent —
  `kb-reference-model-family-case-cliff-001` measures the cliff at 192 against a default of 256.
- **Superseded** — [`ps-1-states-no-progress-obligation.md`](../open-questions/ps-1-states-no-progress-obligation.md)
  (`kb-open-question-ps-1-no-progress-obligation-001`) — PS-1's `MUST` is a
  coupling, not a progress obligation; the third rule assigned to it does
  not follow from the sentence. Owned by phase 6. Amended 2026-08-13: the
  clause-pairing sweep confirmed the defect isolated (29 sound / 7 defective
  / 1 undetermined) and ADR-0019 (`kb-decision-0019`) named the same
  intent-not-sentence habit recurring on PS-29; sub-questions 1, 2 and 4
  stayed open. Resolved 2026-08-15 by ADR-0030 (`kb-decision-0030`), which
  mints PS-38 rather than widening PS-1: sub-question 1 answered a clause of
  its own, sub-question 2 by reattributing `commit_advances_the_checkpoint`
  to PS-38, sub-question 4 by the decision itself; PS-1's own text stays
  byte-identical.
- **Superseded** — [`ps-19-scope-narrower-than-its-rule.md`](../open-questions/ps-19-scope-narrower-than-its-rule.md)
  (`kb-open-question-ps-19-scope-narrower-001`) — PS-19's `MUST` is scoped
  to after a reset; its second assigned rule asks about an id never seen.
  Owned by phase 6; interacts with the PS-1 question above. Amended
  2026-08-13: the same sweep confirmed this finding too, and ADR-0018
  (`kb-decision-0018`) scoped the defect out of its own clause range by
  name without repairing it; sub-questions 1 and 3 stayed open. Resolved
  2026-08-15 by ADR-0030 (`kb-decision-0030`): PS-19 keeps its post-reset
  scope and the never-seen-id obligation becomes PS-38's second sentence;
  sub-question 2's 2026-08-13 "isolated" verdict stands untouched.
- **Open** — [`es-6-names-an-unwritable-rule.md`](../open-questions/es-6-names-an-unwritable-rule.md)
  (`kb-open-question-es-6-unwritable-rule-001`) — ES-6 is `[FROZEN]` and
  names a conformance rule that cannot be written against today's port.
  Amended 2026-08-10: ADR-0008 and ADR-0009 (`kb-decision-0008`,
  `kb-decision-0009`) are now imported and supply the marker the rule would
  name; the question is still open because neither assigns an owning phase.
  Amended 2026-08-20: ADR-0023 (`kb-decision-0023`) judges ADR-0009's
  *prediction*, not this atom's *rule* — phase 9's Cloudflare adapter is the
  first runtime to produce a live `!Send` error carrying a JavaScript value,
  exercised by four `es6_reconstruction` tests on `wasm32`, with no
  `Send + Sync` bound added anywhere. The premise is now observed under
  execution rather than only imported; `store_error_crosses_a_join_handle`
  is still unwritten and unowned, so the question narrows rather than
  closes. Amended 2026-09-07: ADR-0050 (`kb-decision-0050`) narrows
  `StringifiedThrow` to `pub(crate)`, endorsing ES-6's wrapped-driver-error shape without settling
  whether the wrapped type is part of the promise; two stale spec citations on this atom were
  repointed to their live lines in the same pass.
- **Open** — [`es-7-and-vt-9-provisional-markers.md`](../open-questions/es-7-and-vt-9-provisional-markers.md)
  (`kb-open-question-provisional-falsifiers-001`) — ES-7 and VT-9 are
  `[PROVISIONAL]` and each names a falsifier that no longer discriminates.
  Amended 2026-08-10: both markers now have named ADR owners in the imported
  corpus — ES-7 is ADR-0001's lift condition, discharged by ADR-0008
  (`kb-decision-0001`, `kb-decision-0008`); VT-9 is one of ADR-0014's four
  provisional parts (`kb-decision-0014`), owned by phase 9's Workers
  skeleton. Moving either marker is still an ADR's act, not this atom's.
  Amended 2026-09-09: two further markers join them, and they sharpen the observation rather than
  repeat it, because these falsifiers never could fire rather than having already fired harmlessly.
  PS-4's Rust-level limb is foreclosed by the port for every batch shape, since `Projection::apply`
  is synchronous and a traversal is I/O; PS-2 is the same defect one level over and wider, since that
  clause is `[FROZEN]` — what cannot be met is its bar rather than a falsifier, and thirteen
  `[PROVISIONAL]` clauses wait on it alone. ADR-0060 (`kb-decision-0060`) keeps the port's gate on
  that ground without rewording PS-2's `MUST` or moving a marker. A falsifier can be decoration a
  priori, not only in retrospect, and `spec-trace` detects neither shape. Amended 2026-09-10/11: ADR-0062
  (`kb-decision-0062`) moves the seam rather than the marker and finds PS-6's own falsifier had already
  fired unremarked (it is `sqlx`'s `BEGIN`), rewriting its MUST to the discipline the old signature used
  to protect by construction. ADR-0063 (`kb-decision-0063`) narrows what "thirteen clauses wait on PS-2
  alone" means: PS-4, PS-5 and PS-12 freeze with the axis now built at both ends, while PS-9, PS-11,
  PS-15 and the rebuild cluster stay on their own separate falsifiers, the RUNBOOK grouping having been a
  simplification the clauses themselves never made. PS-4's own Rust-level limb — `apply` synchronous, a
  traversal I/O — is unmoved and unowned, and is now `kb-open-question-apply-synchronous-live-store-001`'s
  territory rather than this atom's.
- **Open** — [`nothing-owns-the-post-phase-reconciliation.md`](../open-questions/nothing-owns-the-post-phase-reconciliation.md)
  (`kb-open-question-post-phase-reconciliation-001`) — no phase carries an
  item obliging anyone to read the specification back against the tree a
  phase just changed. Forced by phase 6's exit and, secondarily, by first
  publish at phase 12. Depends conceptually on the PS-1, PS-19 and
  ES-7/VT-9 questions above. Amended 2026-08-20: a second hand-run census,
  phase 8's (`kb-reference-phase-8-spec-reconciliation-001`), supplies
  further evidence without settling any of the five ordered sub-questions —
  a minority of its repairs were citation line numbers a machine could
  plausibly catch and the majority were sentences simply false, which argues
  for the pass having a named owner rather than a gate step replacing it.
- **Open** — [`cf-36-names-a-cross-reference-nothing-performs.md`](../open-questions/cf-36-names-a-cross-reference-nothing-performs.md)
  (`kb-open-question-cf-36-unperformed-cross-reference-001`) — CF-36 is
  `[FROZEN]` and its `Rule:` line claims `cargo xtask spec-trace`
  cross-references each case's level marker; it does not, and no other check
  performs the comparison under another name. Added 2026-08-17; forced by
  the next reader who cites a green `spec-trace` as evidence for a level
  marker, and by phase 12.
- **Superseded** — [`no-ps-rule-name-is-resolved.md`](../open-questions/no-ps-rule-name-is-resolved.md)
  (`kb-open-question-no-ps-rule-name-resolved-001`) — CF-38 is `[FROZEN]`;
  PS-27 and PS-30 are the visible symptom of a rule-name check that never
  runs against any `PS` clause carrying a bare dagger, since the dagger sets
  `schedules_new` regardless of what `has_suite` now admits. Added
  2026-08-17. **Superseded 2026-09-07** by
  [`is-the-dagger-convention-superseded-by-maturity-markers.md`](../open-questions/is-the-dagger-convention-superseded-by-maturity-markers.md)
  (`kb-open-question-dagger-convention-vs-maturity-markers-001`): `schedules_new` is retired
  (verified by grep — it survives only in three doc comments recording its own deletion), and
  `grep -n "Rule:.*†" spec/SPECIFICATION.md` returns zero — no clause's `Rule:` line carries a
  hand-authored dagger any more. The premise this atom stated no longer describes anything in the
  tree; its body stays byte-identical rather than being rewritten.
- **Open** — [`is-the-dagger-convention-superseded-by-maturity-markers.md`](../open-questions/is-the-dagger-convention-superseded-by-maturity-markers.md)
  (`kb-open-question-dagger-convention-vs-maturity-markers-001`) — all 59 daggers in
  `spec/SPECIFICATION.md` are now section 7.2's generated cells or its legend, where `†` means the
  checker looked in `suite.rs` (and the two `wire.rs` files for `wire::`-qualified names) and did
  not find it. Restates the predecessor's third sub-question against the generated marker: does `†`
  say anything `UNRESOLVABLE_RULE_NAMES`'s three kinds (`Elsewhere`, `NotARuleName`, `Scheduled`)
  do not. Added 2026-09-07; supersedes the entry above.
- **Open** — [`cf-38-case-naming-no-clause-reading.md`](../open-questions/cf-38-case-naming-no-clause-reading.md)
  (`kb-open-question-cf-38-case-naming-no-clause-001`) — whether CF-38's fourth condition means "a
  case no clause claims" (landed, checked, green) or "a case body naming a clause," the reading
  CF-37's own text asks for and under which 54 of 58 E2E cases fail. Both readings and the
  misleading green-summary consequence are recorded; neither is chosen. Added 2026-09-07.
- **Open** — [`gate-step-first-check-hides-its-second.md`](../open-questions/gate-step-first-check-hides-its-second.md)
  (`kb-open-question-gate-step-first-check-hides-001`) — whether `lints::stated_rule_counts`
  should collect or split the three checks it bundles into one pass. Scoped narrowly: the V-6
  anchor-derivation half of the same source brief is routed to the anchoring-citations playbook
  instead of restated here. Added 2026-09-07.
- **Open** — [`what-the-exact-anchor-rule-still-leaves-open.md`](../open-questions/what-the-exact-anchor-rule-still-leaves-open.md)
  (`kb-open-question-exact-anchor-residue-001`) — three residues ADR-0045's own exact-anchor
  ratification left open: `spec_trace.rs`'s undocumented `ANCHOR_SLACK = 12` (kept deliberately,
  never measured), no rule requiring anchor uniqueness, and no shipped idempotent `--repoint`.
  Added 2026-09-07.
- **Open** — [`docs-citation-anchor-form-and-clause-contradiction-check.md`](../open-questions/docs-citation-anchor-form-and-clause-contradiction-check.md)
  (`kb-open-question-docs-citation-anchor-contradiction-001`) — three stacked defects in
  `docs/append-conditions.md`: a citation of ES-40 that contradicted it, a dead `file:line` landing
  mid-paragraph in an unrelated doc comment, and an unrecognised bare `:NNN` shorthand. Recommends
  the anchored form and declines a mechanical contradiction check as impossible. Added 2026-09-07. Amended
  2026-09-11: the Option-C sub-question is answered at review — re-anchor at promotion in
  `/redkiln:kb-ingest`, refusing the wave, is the chosen remedy, and this wave's own two staged citation
  drifts (`lints.rs` and CF-34) were caught and repointed that way; the warning-scan alternative is
  declined as the wrong side of the citation-scan boundary. The question does not close.
- **Open** — [`accepted-atom-immutability-check-is-pre-commit-only.md`](../open-questions/accepted-atom-immutability-check-is-pre-commit-only.md)
  (`kb-open-question-immutability-check-pre-commit-001`) — `redkiln validate --kb`'s accepted-decision
  immutability check compares the working tree against `HEAD`, not a commit against a merge base, so it
  is a dirty-tree guard: it reports a refusal on an uncommitted edit and clears the instant that edit is
  committed, and cannot distinguish a referent-only repair from a reversal — its own refusal of
  `kb-decision-0058`'s stale-citation repointing (`4e13ee2`) demonstrates both halves at once. Three
  remedies are named and none chosen: a carried body hash, a documented referent-only carve-out, or a CI
  diff against the merge base. Added 2026-09-11.
- **Open** — [`sole-evidence-pin-requirement-generality.md`](../open-questions/sole-evidence-pin-requirement-generality.md)
  (`kb-open-question-sole-evidence-pin-generality-001`) — `the_shotgun_mutants_sole_coverage_is_pinned`
  only requires a pin for REGISTRY's one shotgun mutant, while a crude regex found roughly sixteen
  rules sharing the same sole-evidence hazard CF-1 exists to police, most unpinned. A blanket
  requirement risks pressure to add a second mutant instead of a pin; proper costing is owed a
  Rust-side census. Added 2026-09-07.
- **Open** — [`references-adr-in-place-correction-policy.md`](../open-questions/references-adr-in-place-correction-policy.md)
  (`kb-open-question-references-adr-correction-policy-001`) — CLAUDE.md makes accepted `.kb` atoms
  immutable and describes `references/adr/` as the full record kept for citation, but never states
  whether that longer record may be corrected in place. Grounded in a live falsified figure at
  `references/adr/0012-append-shape-and-preconditions.md:172-174`, superseded by ADR-0015's change
  and the measured allocation cost. Added 2026-09-07.
- **Open** — [`experiment-raw-output-eaten-by-the-ignore-rule.md`](../open-questions/experiment-raw-output-eaten-by-the-ignore-rule.md)
  (`kb-open-question-experiment-raw-output-ignored-001`) — `experiments/ladybug-driver-probes/README.md`
  states that `results/probe-output.txt` is its output, verbatim, but no `results/` directory exists
  in the tree; `.gitignore:69`'s `*-output.txt` pattern, written at the pre-publication sweep to
  catch stray transcripts carrying an absolute path, is the cause, invisible on inspection since 321
  files are tracked under other `experiments/**/results/` paths and none happens to collide with the
  suffix. Discovered by this wave itself rather than sourced from either staged intake file. Three
  remedies of different cost are named and none is chosen: rename the file, carve a tracked
  exception, or treat the README's inline transcription as the evidence of record. Added 2026-09-09.

## Contract ports, conformance, and the ADR corpus (2026-08-10 ADR import)

The first eleven questions below were filed by the 2026-08-10 ADR-import intake wave, deferred
rather than settled because each is forced by a phase or an adapter that has not arrived yet. The
twelfth, ES-17, was added by the 2026-08-17 wave. Grounding for each is in the atom itself; see
[`domain-map.md`](domain-map.md#contract-ports-conformance-and-the-adr-corpus-2026-08-10-adr-import)
for the reference, concept, governance and playbook atoms this domain also owns.

- **Open** — [`adr-status-vocabulary-exceeds-the-schema.md`](../open-questions/adr-status-vocabulary-exceeds-the-schema.md)
  (`kb-open-question-adr-status-vocabulary-001`) — `KbFrontmatter`'s status
  enum has no value for "accepted, provisional" or "partly superseded," both
  load-bearing in the imported ADR corpus.
- **Superseded** — [`projection-store-batch-has-no-apply-seam.md`](../open-questions/projection-store-batch-has-no-apply-seam.md)
  (`kb-open-question-projection-batch-no-apply-001`) — `ProjectionStore::Batch`
  carries no trait bounds, so generic code can open and commit a batch and
  cannot write anything into it. Answered 2026-08-13 by ADR-0017
  (`kb-decision-0017`): `Batch` becomes an owned type with no lifetime
  parameter and no universal write vocabulary; sub-question 3 — whether this
  retroactively validates ADR-0006's discriminator — stays open, with the
  typed layer.
- **Open** — [`query-union-rule-is-owed-and-unowned.md`](../open-questions/query-union-rule-is-owed-and-unowned.md)
  (`kb-open-question-query-union-rule-unowned-001`) — `query_union_is_item_concatenation`
  is named as owed and declined by ADR-0011, which flags it as the one
  disposition a human should confirm rather than inherit.
- **Open** — [`global-versus-per-boundary-visibility-invariant.md`](../open-questions/global-versus-per-boundary-visibility-invariant.md)
  (`kb-open-question-global-vs-boundary-visibility-001`) — ADR-0013 froze the
  visibility invariant globally by decision, not by evidence; a boundary-scoped
  projection checkpoint at phase 6 would reopen it. Amended 2026-09-07: ADR-0024
  (`kb-decision-0024`) inherited this premise rather than settling it, and a boundary-scoped
  checkpoint would now reopen ADR-0024's mechanism as well as ADR-0013's argument; no measurable
  steady-state cost was found against the built adapter.
- **Superseded** — [`postgres-arm-c-structural-cost.md`](../open-questions/postgres-arm-c-structural-cost.md)
  (`kb-open-question-postgres-arm-c-cost-001`) — whether a real `sqlx`
  adapter can express ADR-0013's chosen mechanism (xid8 + pg_snapshot_xmin)
  cleanly; the experiment measured four SQL strategies, not four
  implementations. Owned by phase 10 / ADR-0024. **Resolved 2026-09-07** by `kb-decision-0024`:
  the shipped `happenstance-postgres` expresses `xid8` + `pg_snapshot_xmin` cleanly, with no
  measurable steady-state cost against the built adapter; the residual moves to
  `kb-open-question-off-poll-visibility-defect-001`.
- **Superseded** — [`poll-count-bounds-the-visibility-rule.md`](../open-questions/poll-count-bounds-the-visibility-rule.md)
  (`kb-open-question-poll-count-rule-strength-001`) — the cold-future
  hand-polling rule that checks ADR-0013's invariant has a window bounded by
  an adapter's poll count, uncalibrated above two polls. Owned by phase 10.
  Amended 2026-08-20: ADR-0034 (`kb-decision-0034`) records that the fixture
  contract has no single owning document, so ADR-0013's "whoever owns the
  fixture contract" has no referent — a `POLL_BUDGET` capability would be
  minted by the decision that needs it, and this question is that position's
  named next test; a collision is the evidence that would supersede
  `kb-decision-0034`. **Resolved 2026-09-07** by `kb-decision-0024`: the shipped adapter's poll
  schedule (n = 3 at one-millisecond pacing) passes the naive arm cleanly, so `POLL_BUDGET` is
  moot rather than deferred; the same rule stays blind to an off-poll adapter, and that residual
  moves to `kb-open-question-off-poll-visibility-defect-001`.
- **Open** — [`es-38-and-gap-read-rules-are-unowned.md`](../open-questions/es-38-and-gap-read-rules-are-unowned.md)
  (`kb-open-question-es-38-and-gap-read-unowned-001`) — ES-38's rule needs a
  removal-capable store the fixture cannot declare, and `read_from_a_gap_position`
  is named by two accepted decisions (ADR-0011, ADR-0013) and owned by
  neither.
- **Open** — [`projection-id-is-unvalidated.md`](../open-questions/projection-id-is-unvalidated.md)
  (`kb-open-question-projection-id-unvalidated-001`) — `ProjectionId::new` is
  infallible and unvalidated; ADR-0015 declined to validate it, on the
  ground that the omission was never a decision. Forced by phase 6.
- **Superseded** — [`cf-40-fixture-limits-ownership.md`](../open-questions/cf-40-fixture-limits-ownership.md)
  (`kb-open-question-cf-40-ownership-001`) — ADR-0015 both claims and
  disclaims ownership of CF-40 in its own text; ADR-0012 is the other
  claimant. Forced by phase 8's first adapter with real limits. **Resolved
  2026-08-20** by ADR-0034 (`kb-decision-0034`): CF-40 is confirmed ADR-0015's
  clause, and the fixture contract has no single owning document — a CF-
  clause is minted by the decision that first needs the capability. Neither
  ADR-0015 nor ADR-0012 nor ADR-0022 was edited to record this. Sub-question
  3, phase 10's `POLL_BUDGET`-shaped capability, moves to
  `kb-open-question-poll-count-rule-strength-001` and stays open there.
- **Open** — [`dcb-reference-publishes-no-wire-format.md`](../open-questions/dcb-reference-publishes-no-wire-format.md)
  (`kb-open-question-dcb-no-published-format-001`) — WF-1's interoperability
  half stays `[DEFERRED]` because the DCB reference publishes no wire format
  to interoperate with at all. Owned by phase 13.
- **Superseded** — [`human-readable-payload-encoding-on-a-constrained-peer.md`](../open-questions/human-readable-payload-encoding-on-a-constrained-peer.md)
  (`kb-open-question-human-readable-encoding-limits-001`) — WF-11's
  falsifier is broader than base64: `serde`'s `Serializer` has no streaming
  entry point for a human-readable string, for any encoding. Owned by phase
  9's Durable Object adapter. **Answered 2026-08-20**
  (`kb-reference-wf-11-memory-ceiling-verdict-001`): the memory-ceiling
  condition is not constructible on this runtime — a Node isolate has no
  per-isolate cap — so the falsifier could not be fired at all; the category
  finding (no streaming entry point for a human-readable string) is
  reconfirmed. WF-11 itself stays `[PROVISIONAL]`, unmoved. A residual on
  the runtime property this exposed is homed to
  `kb-open-question-workerd-runner-absent-001`.
- **Open** — [`sync-message-set-and-format-version.md`](../open-questions/sync-message-set-and-format-version.md)
  (`kb-open-question-sync-message-set-undesigned-001`) — `FORMAT_VERSION = 1`
  is fully tested and names no message set yet; the vocabulary is phase 13's
  design.
- **Superseded** — [`ps-32-adr-0007-context-correction-is-owed.md`](../open-questions/ps-32-adr-0007-context-correction-is-owed.md)
  (`kb-open-question-ps-32-adr-0007-correction-owed-001`) — PS-32 is `[FROZEN]`
  and states ADR-0007's Context must be corrected: a callback-driven pump
  *can* be written against the port as it stands, falsified by compilation
  rather than argument. What is not decided is who performs the correction
  and in which atom — ADR-0007 is accepted and immutable, so it is a
  superseding decision's act. Added 2026-08-15. **Resolved 2026-08-17** by
  ADR-0031 (`kb-decision-0031`): the correction rides with a partial
  supersession of ADR-0007 that collapses the checkpoint pump upward into
  `happenstance::run_projection`; `kb-decision-0007` stays accepted because
  its three shape decisions are implemented as written, and the pump
  sub-question is answered in the negative — no pump is written.
- **Open** — [`es-17-two-adapter-measurement-is-unscheduled.md`](../open-questions/es-17-two-adapter-measurement-is-unscheduled.md)
  (`kb-open-question-es-17-two-adapter-measurement-001`) — ADR-0012's
  falsifier item 1 asks for two builds of one SQLite adapter differing only
  in `append`'s batch ownership; the phase-8 append-condition experiment
  (`kb-reference-append-condition-experiment-001`) measured three strategies
  against the same `&[Event]` signature instead, and no story currently
  scheduled produces the two-build evidence. Added 2026-08-17; forced by
  whoever next proposes lifting ES-17 to `[FROZEN]`, or by phase 12. Amended 2026-09-07: ADR-0055
  (`kb-decision-0055`) fixes the subject (`append` keeps its borrowed batch at `0.2.0`) and
  restates the falsifier to name adapter shape, tag regime and tag count; the two-build
  measurement is now scheduled against `happenstance-cloudflare`, not SQLite.
- **Open** — [`no-workerd-class-runner-in-the-gate.md`](../open-questions/no-workerd-class-runner-in-the-gate.md)
  (`kb-open-question-workerd-runner-absent-001`) — the whole Cloudflare
  conformance suite executes on `wasm32-unknown-unknown` under
  `wasm-bindgen-test-runner`, against a `node:sqlite`-backed shim, never
  under `workerd`; ADR-0023 (`kb-decision-0023`) records that as an
  escalated, not a rejected, finding — `workerd` has no Windows-native
  story and is versioned by a Node lockfile this repository does not own.
  WF-11's memory-ceiling falsifier could not be made to fire on this
  runtime (`kb-reference-wf-11-memory-ceiling-verdict-001`), a second,
  independent consequence of the same absent runner. Added 2026-08-20;
  forced by the next platform-shaped clause, and by phase 12. Amended 2026-09-07: a third
  consequence of the absent runner — ADR-0052's two query-partition constants are adopted
  unchanged by `happenstance-cloudflare` because no measurement locates the wall for SQL text
  inside a Durable Object isolate, so a narrower width would be invented rather than measured.
- **Superseded** — [`deny-bans-red-on-the-worker-dependency.md`](../open-questions/deny-bans-red-on-the-worker-dependency.md)
  (`kb-open-question-worker-async-trait-ban-001`) — taking the real `worker`
  0.8.5 crate (ADR-0023, `kb-decision-0023`) turns `cargo deny check bans`
  red: `worker`/`worker-macros` depend on `async-trait` unconditionally,
  banned under ADR-0001, and `deny.toml`'s `wrappers` list covers only
  `wasm-bindgen-test` (a dev-dependency). No `happenstance` port gains a
  `Send` bound from this. Neither ratifying a `wrappers` entry nor refusing
  and recording the exception is chosen. Added 2026-08-20; forced by
  `publish-ready-crate`'s AC-012, which cannot claim a green gate while the
  ban is red. **Resolved 2026-09-02** by ADR-0035 (`kb-decision-0035`), which
  takes the ratify shape: `deny.toml`'s `wrappers` list gains `worker` and
  `worker-macros`, one entry each, with the argument for each written into
  the file beside it, amending ADR-0001's exemption set without touching its
  body. Sub-question 2 is not reached — the ban is green, `bans ok` — and
  sub-question 3 is answered yes: the exemption was minted by the adapter
  that first needed it, the pattern `kb-decision-0034` records, with no
  umbrella ADR over dependency exceptions required.
- **Superseded** — [`projection-store-in-adapter-default-features.md`](../open-questions/projection-store-in-adapter-default-features.md)
  (`kb-open-question-adapter-default-projection-feature-001`) — ADR-0036
  ships `ProjectionStore` behind off-by-default `unstable-projection`, and
  `happenstance-sqlite` was fixed 2026-09-02 to stop forwarding the gate
  through its own `default` set. `happenstance-neon` and
  `happenstance-postgres` still carry `default = ["event-store",
  "projection-store"]`, and unlike `happenstance-sqlite`'s exposure, both
  crates also name `happenstance-core`'s `unstable-projection` unconditionally
  in `[dependencies]`, outside any feature — so toggling `default` alone
  would not restore off-by-default there. Owned by the
  `postgres-and-neon-stores` project; forced before either crate's first
  publish. Added 2026-09-02. **Resolved 2026-09-03**: both crates took
  `happenstance-sqlite`'s shape — `default = ["event-store"]`, a
  `projection-store` feature forwarding to
  `happenstance-core/unstable-projection`, the unconditional dependency
  removed — verified across eight feature-combination checks. Closed by a
  direct manifest fix rather than by an answering decision atom:
  `kb-decision-0036` already commits the port to an off-by-default gate, and
  two adapters honouring it settles no fork of its own.
- **Superseded** — [`event-metadata-has-no-declared-floor.md`](../open-questions/event-metadata-has-no-declared-floor.md)
  (`kb-open-question-event-metadata-no-floor-001`) — `happenstance-core`'s
  four `MIN_SUPPORTED_*` constants and three-variant `StoreLimit` enum name
  `EventDataLen`, `TagsPerEvent` and `EventsPerBatch`; `Event::metadata` has
  no declared floor among them, and neither ADR-0003, ADR-0015 nor ADR-0021
  states one. Added 2026-09-04. **Resolved 2026-09-07** by `kb-decision-0043`: a refusal channel
  (`StoreLimit::MetadataLen`, `Fixture::MAX_METADATA_LEN`) with `guaranteed_minimum() == 0`
  rather than a non-zero floor, chosen because a separate floor is incommensurable with
  `happenstance-sync`'s already-shipped `ReplicatedEvent::payload_len` budget.
- **Open** — [`read-page-budget-is-unspecified.md`](../open-questions/read-page-budget-is-unspecified.md)
  (`kb-open-question-read-page-budget-001`) — `happenstance-sqlite`'s
  `PAGE_SIZE = 512` is a private implementation constant, not a port
  concept; ADR-0011 settles what `read` promises without settling what a
  page of it should cost, and the WF-11 and projection-fan-out reference
  atoms already carry adjacent figures with no owning question until now.
  Added 2026-09-04. **Partly answered 2026-09-07** by `kb-decision-0053`: the row-and-byte shape
  (`MAX_PAGE_BYTES_PER_STATEMENT`) lands for `0.2.0`; a caller-stated budget (Option B) is
  explicitly not taken, so the question stays open on that half.
- **Open** — [`no-fixture-tolerance-for-transient-contention.md`](../open-questions/no-fixture-tolerance-for-transient-contention.md)
  (`kb-open-question-testkit-contention-tolerance-001`) — CF-33 forbids a
  conformance rule a clock, an elapsed-time measurement or an
  operation-count assertion, which guarantees a momentarily-busy store and a
  broken one surface as the same failed `Attempt`; no longer hypothetical
  now that the 2026-09-03 busy-timeout measurement observed `busy > 0` at
  the shipped contender count, one launch in seven. What is not decided is
  whether the fixture contract grows a declared tolerance, and in what
  shape. Added 2026-09-04. Amended 2026-09-07: the defect is three conformance rules, not one
  classification arm, since two of the three that can fail under contention assert on a count
  rather than `Attempt::Failed`; `CONTENDERS` has already moved once after publication,
  unversioned (8 to 64), so a lowering to 8 would be a revert to the published value.
- **Open** — [`remint-identity-precondition-is-trust-only.md`](../open-questions/remint-identity-precondition-is-trust-only.md)
  (`kb-open-question-remint-precondition-trust-only-001`) —
  `SqliteEventStore::remint_identity`'s own test runs same-file,
  same-process, and never actually restores or clones a store; VT-6's
  `[PROVISIONAL]` marker rests on that narrower assertion rather than the
  cross-instance one its text describes. Added 2026-09-04.
- **Superseded** — [`query-plan-parameter-chunking-incomplete.md`](../open-questions/query-plan-parameter-chunking-incomplete.md)
  (`kb-open-question-query-plan-parameter-chunking-001`) — the
  30,000-parameter budget is enforced on `write_tag_rows`'s insert path but
  is not proven never to be hit on the 400-arm-per-statement query-chunking
  path, a second, independent axis from
  `kb-open-question-query-union-rule-unowned-001`. Added 2026-09-04. **Resolved 2026-09-07** by
  `kb-decision-0052` and `kb-decision-0053` together with the shipped-SQL reference atom: the
  "400 arms is conservative enough" branch is refuted by measurement (51,200 bound parameters
  at 400 items x `MAX_TAGS_PER_EVENT`, against SQLite's 32,766 ceiling), and the chunking
  mechanism now takes a fourth `per_arm_extra` parameter this question did not describe.
  `Selectivity::read_for`'s own unpartitioned failure mode, found in the same pass, is the
  residual.
- **Open** — [`adr-0022-falsifiers-have-fired.md`](../open-questions/adr-0022-falsifiers-have-fired.md)
  (`kb-open-question-adr-0022-falsifiers-fired-001`) — ADR-0022 named three
  conditions under which it would be re-opened; the 2026-09-03
  pre-publication review found two fired (`busy > 0` observed, and the
  captured `tokio::Handle` makes `NoRuntime` unreachable by a different
  route than the one anticipated) and the third unfireable as written.
  ADR-0022 is accepted and immutable, so a fired falsifier cannot amend it;
  what is not decided is whether it is superseded, re-opened, or ratified as
  still correct with the firings recorded against it. Added 2026-09-04. Amended 2026-09-07: the
  shipped adapter's remeasurement corrects the `busy > 0` inference to a measured rate (0.045%,
  2 of 44 rows at `CONTENDERS = 64`) and widens the chain-vs-aggregate loss to 1.54x-1.86x warm,
  11.8x cold, with a fourth, boundary-bound arm beating the chain outright in most cells; a
  fourth route now exists: superseding ADR-0022's SQL-shape items only (§8, items 1 and 2), narrower
  than superseding the whole decision and wider than ratifying it unchanged.

- **Open** — [`off-poll-adapter-visibility-defect-undetected.md`](../open-questions/off-poll-adapter-visibility-defect-undetected.md)
  (`kb-open-question-off-poll-visibility-defect-001`) — an off-poll adapter (`happenstance-postgres`,
  hopping onto a captured `sqlx` runtime `Handle`) has a real, demonstrated visibility defect
  (`naive_arm_probe.rs`) that no poll-based schedule can reach, because the port exposes no
  suspension point a hand-polled rule can wedge into. Successor to
  `kb-open-question-poll-count-rule-strength-001` and `kb-open-question-postgres-arm-c-cost-001`,
  both superseded above. Added 2026-09-07.
- **Open** — [`cf-5-conformant-control-per-rule-or-per-branch.md`](../open-questions/cf-5-conformant-control-per-rule-or-per-branch.md)
  (`kb-open-question-cf-5-per-rule-or-branch-001`) — whether CF-5's conformant-control obligation is
  discharged once per rule or once per branch inside a rule with more than one assertion arm; ES-22's
  arm 2 is the first place the difference is visible, passed by two stores filed as mutants for
  other rules rather than by a purpose-built `Kind::ConformantVariant`. Added 2026-09-07.
- **Deferred** — [`msrv-ratification-conflicts-with-the-accepted-floor.md`](../open-questions/msrv-ratification-conflicts-with-the-accepted-floor.md)
  (`kb-open-question-msrv-ratification-conflict-001`) — the ratified pre-publication recommendation
  to lower the MSRV to 1.95 is absent from the six-item discharge queue, and the 1.88 figure both
  `kb-decision-0029` and `kb-decision-0037` state is contradicted by a compiled `cfg_select`
  bisection naming 1.95.0 as the first passing version. Filed as a conflict rather than forced to
  resolve. Added 2026-09-07.
- **Open** — [`postgres-fixture-read-fault-declension-is-owed.md`](../open-questions/postgres-fixture-read-fault-declension-is-owed.md)
  (`kb-open-question-postgres-read-fault-declension-001`) — `PostgresFixture` inherits a
  `READ_FAULT` declension that is false about a store whose `PgReadStream` holds a server-side
  cursor, the gap ADR-0051's CF-18 discharge found. Deferred to phase 10's remainder. Added
  2026-09-07.
- **Open** — [`read-fault-rule-has-no-clause.md`](../open-questions/read-fault-rule-has-no-clause.md)
  (`kb-open-question-read-fault-rule-no-clause-001`) — the third entry in `UNCLAIMED_PENDING_ADR`,
  mirroring `kb-open-question-disjoint-boundaries-no-clause-001` and
  `kb-open-question-model-family-rule-no-clause-001`: `arming_a_read_fault_makes_the_stream_yield_an_error`
  is a live, measured rule (`SwallowedReadFaultStore`: 0/89 unarmed, 22/89 armed) with no owning
  clause. Added 2026-09-07.
- **Open** — [`cf-18-residuals-after-declension-by-inheritance.md`](../open-questions/cf-18-residuals-after-declension-by-inheritance.md)
  (`kb-open-question-cf-18-residuals-after-declension-001`) — what ADR-0051's declension-by-inheritance
  does not discharge: a stranger's default `cargo test` still prints no SKIP line for a declined
  capability, ES-35's `[PROVISIONAL]` marker rests on the same unread mechanism, and whether an
  adapter README must disclose declines is unowned. CF-18 itself stays `[FROZEN]` and untouched.
  Added 2026-09-07.
- **Open** — [`cf-23-emitter-names-mandatory-and-marked-unstable.md`](../open-questions/cf-23-emitter-names-mandatory-and-marked-unstable.md)
  (`kb-open-question-cf-23-emitter-names-unstable-001`) — CF-23 requires named emitters while the
  shipped surface marks them `doc(hidden)`; the documentation contradiction already has a fix, the
  policy question (support the names, or declare them unstable) does not, and `cargo-semver-checks`
  cannot see the gap because hidden items are exactly what it excludes. Added 2026-09-07.
- **Open** — [`cf-25-cf-26-portfolio-check-does-not-exist.md`](../open-questions/cf-25-cf-26-portfolio-check-does-not-exist.md)
  (`kb-open-question-cf-25-cf-26-portfolio-check-001`) — no check in `xtask/src/spec_trace.rs`
  performs the portfolio/axis comparison CF-25 and CF-26 both name (verified: zero occurrences of
  "portfolio", "far end" or "axis"). Added 2026-09-07.
- **Open** — [`vt-30-provisional-marker-is-stale-and-unscheduled.md`](../open-questions/vt-30-provisional-marker-is-stale-and-unscheduled.md)
  (`kb-open-question-vt-30-marker-stale-unscheduled-001`) — split off from ADR-0054, which discharges
  only `after_opt`'s naming/pinning half; VT-30's `[PROVISIONAL]` marker is stale as written now that
  `happenstance-testkit/src/bench.rs` exists, and the real gap narrows to no multi-guard workload in
  the harness. Added 2026-09-07.
- **Open** — [`adapter-version-lockstep-and-cf-32.md`](../open-questions/adapter-version-lockstep-and-cf-32.md)
  (`kb-open-question-adapter-version-lockstep-001`) — whether an adapter's version implies a
  `happenstance-core` version; the brief's original lockstep claim collides with CF-32 `[FROZEN]`,
  which mandates `happenstance-testkit`'s own independent version key. Added 2026-09-07.
- **Open** — [`happenstance-facade-does-not-match-adr-0006.md`](../open-questions/happenstance-facade-does-not-match-adr-0006.md)
  (`kb-open-question-facade-does-not-match-adr-0006-001`) — `kb-decision-0006` is accepted and
  states `happenstance` re-exports the contract and feature-gates the adapters; the live code is a
  bare `pub use happenstance_core::*` glob with no adapter dependency at all to feature-gate. Filed
  as unresolved rather than forcing either side to change. Added 2026-09-07.
- **Open** — [`stale-0-0-0-name-reservations.md`](../open-questions/stale-0-0-0-name-reservations.md)
  (`kb-open-question-stale-0-0-0-name-reservations-001`) — `happenstance-sqlite` and
  `happenstance-cloudflare` are live on crates.io at `0.0.0` only, while `happenstance-core`,
  `happenstance` and `happenstance-testkit` carry both `0.0.0` and `0.2.0-alpha.1`; two unrelated
  briefs hit the same ambiguity independently and both worked around it. Added 2026-09-07.
- **Open** — [`model-only-kind-memberless-dormant-or-withdrawn.md`](../open-questions/model-only-kind-memberless-dormant-or-withdrawn.md)
  (`kb-open-question-model-only-kind-memberless-001`) — `Kind::ModelOnlyMutant` has gone memberless
  now that `read_to_composes_with_multi_item_query` landed; dormant, withdrawn, and
  self-asserting-emptiness are all recorded, none taken. Added 2026-09-07.
- **Open** — [`read-to-backwards-limit-composition-gap.md`](../open-questions/read-to-backwards-limit-composition-gap.md)
  (`kb-open-question-read-to-backwards-limit-composition-001`) — the unwritten backwards-plus-window-plus-limit
  read composition, deliberately unasserted because it would double-reject three already-registered
  mutants for reasons three other rules own; forced before the first SQL adapter ships a windowed
  query. Added 2026-09-07.
- **Open** — [`cf-33-cf-34-scope-outside-the-testkit.md`](../open-questions/cf-33-cf-34-scope-outside-the-testkit.md)
  (`kb-open-question-cf-33-cf-34-scope-001`) — a ratio-based `#[cfg(test)]` check in
  `happenstance-sqlite` sits outside CF-33's stated `src/` scope, and a benchmark completion panic
  sits adjacent to CF-34's merge-red prohibition; no governing principle reconciles both with the
  frozen text as written. Added 2026-09-07. Amended 2026-09-11: a third instance,
  `ops/host/preflight.sh` (`kb-decision-0064`) — an environment assertion made before any sample
  exists, unreachable from `xtask`'s step table, the `verify:` block or CI the same way `ops/` sits on
  the `INERT` list — joins the sqlite check and the benchmark panic; CF-33 and CF-34 re-anchored to
  their live lines (`spec/SPECIFICATION.md:8994-9019`, `:9021-9034`), both stale as previously cited.
- **Open** — [`es-23-frozen-doc-musts-adapter-half.md`](../open-questions/es-23-frozen-doc-musts-adapter-half.md)
  (`kb-open-question-es-23-adapter-half-001`) — `FROZEN_DOC_MUSTS` has no recorded disposition for
  ES-23's adapter-side `MUST`, a gap two named instruments (ADR-0012's proposed gate step, the
  phase 8-11 per-adapter review) both failed to catch; `kb-decision-0058` discharges the port-adjacent
  half only. Added 2026-09-07.
- **Open** — [`es-18-byte-identical-versus-conformance-reading.md`](../open-questions/es-18-byte-identical-versus-conformance-reading.md)
  (`kb-open-question-es-18-byte-identical-conformance-001`) — whether ES-18's "byte-identical"
  second sentence is amended to the weaker conformance reading its own rules ask, left with
  contradicting commentary, or narrowed to exclude the position counter; specific to
  `happenstance-cloudflare`'s compensation-based atomicity, since `happenstance-sqlite`'s
  `sqlite_sequence` is under transaction control and a rollback restores it. Added 2026-09-07.
- **Superseded** — [`probe-read-through-signature-and-live-transaction-seam.md`](../open-questions/probe-read-through-signature-and-live-transaction-seam.md)
  (`kb-open-question-probe-read-through-signature-001`) — whether `ProjectionProbe::probe_read_through`
  moves to `&mut Self::Batch` / async / fallible before phase 10, or the live-transaction adapter is
  built against today's shape and PS-2 part 2 is judged on a declared-false capability; corroborates
  ADR-0036's part-2-unmet finding with a second causal reading. Added 2026-09-07. Amended 2026-09-09:
  the honest scope is the whole probe seam, not one method — `probe_write` and `probe_delete_all` are
  synchronous and infallible too, so the recommended `&mut`/async/`Result` move on
  `probe_read_through` is not sufficient alone — and ADR-0060 (`kb-decision-0060`) keeps the port's
  gate on exactly this ground, declining the signature change as PS-2's owner's call rather than an
  adapter lane's; the question stays open, wider than when it was written. **Resolved 2026-09-10** by
  ADR-0062 (`kb-decision-0062`): the whole seam moves, `begin` with it, `LivePostgresProjectionStore`
  runs 20 of 20 against a live PostgreSQL, and PS-2's MUST is met as written; the apply-side residual
  the resolution hands on is `kb-open-question-apply-synchronous-live-store-001`.
- **Open** — [`projection-batch-sql-seam-statement-type.md`](../open-questions/projection-batch-sql-seam-statement-type.md)
  (`kb-open-question-projection-batch-sql-statement-type-001`) — whether `SqliteBatch::push`'s
  landed `&'static str` narrowing is the seam's final shape or a minted `Statement` newtype follows
  once `ProjectionStore` freezes under PS-2. Added 2026-09-07.
- **Open** — [`projection-runner-chunk-type-and-observation-seam.md`](../open-questions/projection-runner-chunk-type-and-observation-seam.md)
  (`kb-open-question-projection-runner-chunk-observation-001`) — a named chunk type plus its default,
  and the runner's observation seam; both priced at zero code-cost-of-delay by ADR-0036's exemption,
  with the chunk-size curve still unmeasured. Added 2026-09-07.
- **Open** — [`reset-refusal-declension-has-no-clause.md`](../open-questions/reset-refusal-declension-has-no-clause.md)
  (`kb-open-question-reset-refusal-declension-001`) — `RESET_REFUSAL` has no CF-39-shaped clause
  after ADR-0042 retracted the trait-level honesty requirement, so a fixture can declare it and
  override `protect_from_reset` with an empty body, passing `refused_reset_changes_nothing`
  vacuously. Added 2026-09-07. Amended 2026-09-09: the window this atom originally named has closed
  without answering it — four storage adapters (`happenstance-sqlite`, `happenstance-postgres`,
  `happenstance-neon`, `happenstance-ladybug`, the last new at ADR-0025) now invoke
  `projection_store_conformance!`, and all four decline `RESET_REFUSAL`, so the population is still
  empty and the family still has no named wrong implementation; the cost of leaving it open no longer
  has a first-adapter deadline behind it.
- **Open** — [`testkit-projection-module-unstable-projection-exemption-scope.md`](../open-questions/testkit-projection-module-unstable-projection-exemption-scope.md)
  (`kb-open-question-projection-module-exemption-scope-001`) — ADR-0036's unstable-projection
  exemption text names only `happenstance-core` and `happenstance`; `happenstance-testkit`'s
  projection module is unconditional, making `ProjectionFixture` ordinary un-exempt published API
  on the documents as written. Added 2026-09-07. Amended 2026-09-11: ADR-0063 (`kb-decision-0063`)
  freezes the port the exemption gated, dissolving the scope question for a trait that is no longer
  exempt anywhere — `ProjectionFixture` is a committed surface on a frozen trait, the ordinary case —
  and leaving only the mechanical residue (the testkit manifest's forward of the now-empty feature, and
  a third `xtask` test asserting nothing still forwards it). Annotated, not closed.
- **Open** — [`trait-variant-caret-resolves-past-the-locked-gate.md`](../open-questions/trait-variant-caret-resolves-past-the-locked-gate.md)
  (`kb-open-question-trait-variant-caret-001`) — the blanket impl binding constraint 4 rests on is
  pinned only by a caret (`0.1.3`); every guard on its shape is `#[cfg(test)]` and thus
  `--locked`-only, but `Cargo.lock` does not travel to a consumer of the three already-published
  crates, so a consumer's resolve can float today. Added 2026-09-07.
- **Open** — [`es-11-ceiling-sample-cost-on-sqlite-read.md`](../open-questions/es-11-ceiling-sample-cost-on-sqlite-read.md)
  (`kb-open-question-es-11-sqlite-ceiling-sample-cost-001`) — `SqliteEventStore::read`'s first poll
  stalls 635ms (39.3x the idle floor) under contention because ES-11 forces `sample_ceiling` to take
  the connection mutex synchronously before the `spawn_blocking` hop; a second native-adapter data
  point against ES-11's `[PROVISIONAL]` marker. Added 2026-09-07. Amended 2026-09-09: its third
  sub-question — whether this counts as native-adapter evidence bearing on ES-11 — is now answered by
  ADR-0061 (`kb-decision-0061`), which narrows the asynchronous-driver sufficiency condition and
  keeps ES-11 `[PROVISIONAL]`, moving what stays open to
  `kb-open-question-one-shot-http-es-11-001`; happenstance-sqlite meets the narrowed condition for
  exactly the reason the stall exists (the pre-spawn sample takes the process mutex the writer
  takes). Sub-questions 1 and 2 are untouched — neither candidate remedy is measured and the 635ms
  figure stands — so this atom stays open.
- **Open** — [`then-empty-emission-idiom-and-the-nothing-to-do-channel.md`](../open-questions/then-empty-emission-idiom-and-the-nothing-to-do-channel.md)
  (`kb-open-question-then-empty-emission-idiom-001`) — how `then(&[])` reads once `commit`'s outcome
  is two-armed (three of five in-tree call sites would break), and whether `decide` can express
  "nothing to do" distinctly from "refused". Added 2026-09-07.
- **Open** — [`scope-coverage-helper-and-the-projection-port-gap.md`](../open-questions/scope-coverage-helper-and-the-projection-port-gap.md)
  (`kb-open-question-scope-coverage-helper-projection-gap-001`) — the additive, enum-total
  `assert_scope_covered` test helper ADR-0047 endorses but sequences after this, and the identical
  unchecked tags/scope pair on the projection port ADR-0047's fix does not reach. Added 2026-09-07.
- **Open** — [`one-shot-http-conformance-to-es-11.md`](../open-questions/one-shot-http-conformance-to-es-11.md)
  (`kb-open-question-one-shot-http-es-11-001`) — ADR-0061 narrowed ES-11's asynchronous-driver
  sufficiency condition to require ordering against a later append by something the store itself
  honours, and recorded that `happenstance-neon` does not satisfy it. Open is not whether this
  adapter can be fixed but whether any one-shot-HTTP shape can meet the obligation at all: the
  transport offers exactly one ordering primitive (HTTP/2 over one multiplexed connection, which
  narrows the race without closing it) and nothing else in a pooled-proxy path orders one backend's
  snapshot against another's commit. Nothing forces an answer today, because `NEON_CONNECTION` is
  not a repository secret and the live-neon job is gated on it. Added 2026-09-09.

## The typed layer: decision models, codecs, and payload evolution

Four questions. The first was added by the 2026-08-17 wave from the phase-7 contract-defect log;
the other three by the 2026-09-07 wave. See
[`domain-map.md`](domain-map.md#the-typed-layer-decision-models-codecs-and-payload-evolution) for
the decision and reference atoms this domain also owns.

- **Open** — [`d-1-the-validated-type-has-no-total-path.md`](../open-questions/d-1-the-validated-type-has-no-total-path.md)
  (`kb-open-question-d-1-no-total-path-001`) — `QueryItem::new` is fallible
  even over already-validated `EventType`/`Tag` inputs, and `Boundary`'s
  seal makes that error arm untestable from outside the crate; `DomainEvent::tags`
  is total over a `Tags` every constructor route into which is fallible, so an
  implementor with runtime tag values has no total path without a hand-rolled
  newtype. ADR-0020 named the pair defect candidate D-1 and routed it to a
  decision record that has not been written. Added 2026-08-17; forced by the
  first API change after 0.1, and named by ADR-0033 as the single condition
  that would reopen the `happenstance-macros` scope verdict.
- **Open** — [`event-type-positional-mapping-has-no-compiler-check.md`](../open-questions/event-type-positional-mapping-has-no-compiler-check.md)
  (`kb-open-question-event-type-positional-mapping-001`) — the worked examples' `EVENT_TYPES[n]`
  mapping from a fold position to a decoder is unchecked by the compiler, in both
  `course-subscriptions` and `transfers-on-sqlite`; filed rather than amending ADR-0033, which the
  new second-example ceremony measurement strengthens instead. Added 2026-09-07.
- **Open** — [`tuple-boundary-heterogeneous-event-type.md`](../open-questions/tuple-boundary-heterogeneous-event-type.md)
  (`kb-open-question-tuple-boundary-event-type-001`) — `composition.rs`'s macro-generated tuple
  impls bind every member to the first member's `Event` type, unwritten in ADR-0020's decision text,
  the signed-off design (which contradicts itself about it), or `standards/rust/`. Added 2026-09-07.
- **Open** — [`should-codec-be-sealed.md`](../open-questions/should-codec-be-sealed.md)
  (`kb-open-question-seal-the-codec-001`) — whether `Codec` is later sealed, now that `0.2.0` is
  live and the window to do so for free has closed; bundles the `UnknownTag`-split and
  `Boundary::absorb` sub-questions ADR-0049 left undone. Added 2026-09-07.
- **Open** — [`projection-apply-is-synchronous-against-a-live-store.md`](../open-questions/projection-apply-is-synchronous-against-a-live-store.md)
  (`kb-open-question-apply-synchronous-live-store-001`) — `Projection::apply` is synchronous
  (`crates/happenstance/src/domain.rs:249`) and `run_projection` folds events through it before
  handing the batch to the store, exactly right for a buffered batch and exactly wrong for one that
  is a live transaction; ADR-0062 built the live end and ADR-0063 froze the port on it while
  deliberately leaving `apply`'s shape to the typed layer, narrowing `happenstance`'s
  `unstable-projection` feature to gate the runner alone for that reason. Neither record owns
  whether `apply` moves, to what shape, or whether the runner's gate comes off with it; forced by the
  first application needing a row-writing projection against a live-transaction store through the
  runner, or a decision to publish the runner ungated first. Added 2026-09-11.

## Brand identity: the name, the mark, and where it lives

One question, added by the 2026-09-02 wave from two staged brand documents that closed on the
same unresolved gate. See
[`domain-map.md`](domain-map.md#brand-identity-the-name-the-mark-and-where-it-lives) for the
decision, reference and design atoms this domain also owns.

- **Open** — [`trademark-search-gates-the-commercial-layer.md`](../open-questions/trademark-search-gates-the-commercial-layer.md)
  (`kb-open-question-trademark-search-001`) — no trademark search on
  "happenstance" has been run. The commercial layer's anti-appropriation
  lever is trademark, not copyright — Apache-2.0 §6 grants no trademark
  rights — so the licence protects the code and protects the name not at
  all. The identity itself is unaffected and ships today; what is gated is
  filing, registration, or applying the identity to physical goods. Forced
  by the first of those three, whichever comes first. Added 2026-09-02.

## Publication and release readiness

Five questions, added by the 2026-09-07 wave alongside the new decision domain of the same name.
See [`domain-map.md`](domain-map.md#publication-and-release-readiness) for the decisions
(ADR-0041, ADR-0044, ADR-0057) this domain also owns.

- **Open** — [`adapter-version-lockstep-and-cf-32.md`](../open-questions/adapter-version-lockstep-and-cf-32.md)
  (`kb-open-question-adapter-version-lockstep-001`) — whether an adapter's version implies a
  `happenstance-core` version; collides with CF-32 `[FROZEN]`, which mandates
  `happenstance-testkit`'s own independent version key with a manifest-check enforcement. Added
  2026-09-07.
- **Open** — [`happenstance-facade-does-not-match-adr-0006.md`](../open-questions/happenstance-facade-does-not-match-adr-0006.md)
  (`kb-open-question-facade-does-not-match-adr-0006-001`) — `kb-decision-0006` states `happenstance`
  re-exports the contract and feature-gates the adapters; the live crate root is a bare glob with no
  adapter dependency to feature-gate. Added 2026-09-07.
- **Open** — [`stale-0-0-0-name-reservations.md`](../open-questions/stale-0-0-0-name-reservations.md)
  (`kb-open-question-stale-0-0-0-name-reservations-001`) — two publishable crates are live on
  crates.io at `0.0.0` only while three others carry both `0.0.0` and `0.2.0-alpha.1`; ADR-0044's
  re-export policy and the security-channel decision both hit the ambiguity independently. Added
  2026-09-07. Amended 2026-09-08: a second registry read grounded two more rows (postgres, neon) in
  `e597c34` and `RUNBOOK.md`'s "remaining names are claimed at their phases" rule — all seven
  publishable crates carry `0.0.0`, three additionally carry `0.2.0-alpha.1`, nothing is yanked, and
  `0.2.0` is a first real release for all seven; the sub-question naming "two" crates without a
  `0.2.0-alpha.1` companion now names four.
- **Open** — [`cloudflare-worker-feature-gate.md`](../open-questions/cloudflare-worker-feature-gate.md)
  (`kb-open-question-cloudflare-feature-gate-001`) — `happenstance-cloudflare`'s manifest declares
  no `[features]` table and `worker`'s types (`SqlStorage`, `State`, `Error`) sit on every public
  construction/error path; distinguished from
  `kb-open-question-worker-async-trait-ban-001` (superseded, about the `deny.toml` ban rather than a
  Cargo feature gate). Added 2026-09-07.
- **Open** — [`rustdoc-citations-relative-or-url-shaped.md`](../open-questions/rustdoc-citations-relative-or-url-shaped.md)
  (`kb-open-question-rustdoc-citation-form-001`) — now that the repository is public
  (`kb-decision-0041`), whether a rustdoc citation into `spec/` or `.kb/` should be a relative path
  or a URL an outside reader can actually open. Added 2026-09-07.

## Adding an entry

Append the bullet under the domain section the question belongs to (see
[`domain-map.md`](domain-map.md) for the list of domains); start a new `##`
section only when the question's domain has no section yet. State the
status (`Open`, `Withdrawn`, `Superseded`) first, then the id and one
sentence — the atom itself carries the "what is true today / what is not
decided / what forces it" structure, this index does not repeat it.
