# Grounding — The whole gate on the assembled library, and a durable audience (HS-P0019)

Companion to `project.md` and the not-yet-authored `_testing.md`. Cites what already
exists on disk so the testing brief and story map are written against reality, not
against the charter's paraphrase of it.

## What already exists for this project

`project.md` (stage `storymap`) is already fully authored — objective, scope,
DR-1…DR-13, AC-001…AC-014, DoD 1–7 (boundary-level), dependencies, risks and context
anchors are all present and cited. `_intake-brief.md` is approved (all seven intake
gate boxes ticked). `_storymap.md` exists but is an **empty template stub** — backbone
and slice table both blank. **`_testing.md` does not exist yet** — it is the one
warranted brief (`_decomposition.md` *Warranted briefs*: `closeout-and-durable-audience`
carries `testing` only, no `architecture`/`ux`/`deployment`) and is `plan-briefs`'s to
write next, consuming this note.

## Accepted decision atoms that constrain this project

Seventeen decision atoms exist on disk today (`0001`–`0016`, `0029`); **`0017`–`0028`
are still reserved and unwritten** (`RUNBOOK.md:270`, `RUNBOOK.md:284-292`) because they
belong to sibling projects (`projection-store-freeze`, `typed-layer-and-alpha-release`,
adapters, sync) that this project sits downstream of in merge order (rank 6, `_decomposition.md`
*Merge order*). This project's own testing brief cannot cite ADRs that do not exist yet
at grounding time — the DR-6 audit table it plans for is necessarily a template with
rows to be filled once the sibling projects merge, not a table this brief can populate
now.

Directly binding on this project's own conduct (not on the code it re-observes):

- **`.kb/decisions/README.md`** — the immutability rule: an accepted decision's body is
  never edited; a correction is a new atom with `supersedes:`, and the old atom flips to
  `status: superseded`. This is what DR-6's audit checks *for* on every sibling's ADR,
  and what this project must itself respect if BR-15's audit finds a gap — the fix is a
  new decision atom, never an edit (`CLAUDE.md` binding constraint list header: "Changing
  one means writing a new ADR, not editing code around it").
- **`ADR-0006` (bare-name-to-the-typed-layer)** and **`ADR-0029` (msrv-raised-to-1-97-1)**
  — cited already in `project.md`'s context anchors and in `CLAUDE.md`'s binding
  constraints; both are the kind of "answer this initiative settled" DR-6's audit table
  will eventually need to enumerate, alongside the reserved-but-unwritten numbers.
- **The reverted commit `0269720`** ("Revert the hand-authored backlog and knowledge
  base", 2026-08-09) is not an ADR but functions as one for this project's DR-8/AC-008:
  its message states plainly that hand-authoring `.kb/` atoms — even correctly-shaped
  ones — "is not an accelerated version of that process, it is a different process
  wearing its directory layout," and that decomposition/promotion decided in advance and
  "typed... in" is the bypass. It explicitly kept `redkiln init`, the `verify:` block,
  the `backlog` CI job and the `.redkiln/templates/` customisations as configuration, not
  SDLC output — i.e. the same three things `_decomposition.md`'s scope-seam text and
  `project.md`'s DR-11 lean on. This is the citation for why `_testing.md` must not
  itself propose hand-writing any `.kb/product/` atom, even as an example.

## Existing code / process patterns this project must follow

- **`xtask/src/main.rs`** module doc (top of file) is the authoritative, current
  description of what `cargo xtask ci` proves and in what order: fmt → clippy `-D
  warnings` (every target/feature) → tests → `wasm32-unknown-unknown` build of
  `happenstance-core` → docs (with-features and `--no-default-features`) →
  `cargo xtask proof-artefact` (holds the conformance suite to its own mutant-detection
  proof) → `cargo xtask spec-trace` → five file-reading lints + a sixth manifest lint for
  D12 → `cargo xtask package-check` (licence/README actually inside the packaged
  artifact). Optional, tool-gated: feature powerset, wasm32 powerset, `cargo deny`,
  nightly `--cfg docsrs`. Every cargo invocation that resolves dependencies passes
  `--locked`. This is more granular than `CLAUDE.md`'s summary and is the citation
  AC-001/DR-2's "every step that printed `skipped` is listed with the absent tool that
  caused it" should be checked against — the module doc names exactly which four steps
  are tool-gated (`cargo hack`, `cargo deny`, wasm32 powerset, nightly rustdoc).
- **`.github/workflows/ci.yml:142-187`** (`backlog` job, "Backlog health" step) is the
  literal source of the six-file `template-drift` assertion DR-11/AC-011/AC-012 point
  at: `.redkiln/templates/_design.md`, `_intake-brief.md`, `discover.md`,
  `gates/discover.md`, `gates/intake.md`, `spec.md` (alphabetical list, exact `jq`
  equality check, not a superset check). The same step runs `redkiln validate`, then
  `redkiln validate --kb`, then captures `redkiln doctor --json` (exit code ignored,
  `|| true`, because doctor's own exit conflates warnings and problems) and asserts on
  the JSON: zero `problems`, zero `process-drift` warnings, and the `template-drift`
  file list equal (not just same-length) to the six above. The step's own comments
  explain *why* an exact-six check rather than a bare "no drift" or "some drift" check —
  useful precedent for `_testing.md` if it needs to justify AC-012's "dispositioned
  rather than accepted" language for a seventh advisory.
- **`spec/SPECIFICATION.md:219-222`** (not 215-221 as `_intake-brief.md`/AC-005's source
  citation says — checked directly): "As assembled, this document carries 200 clause
  IDs, of which 198 are normative: 139 `[FROZEN]`, 49 `[PROVISIONAL]`, 10 `[DEFERRED]`
  and two `[NON-NORMATIVE]`." `project.md`'s own context anchors already cite the
  corrected `219-222` range — use that, not the intake brief's `215-221`, when
  `_testing.md` or a story references the clause count.
- **`.kb/product/README.md`** and **`.kb/design/README.md`** are both real, populated
  READMEs (not stubs) that already state the promotion mechanism precisely: an
  initiative's discovery cites its own `_discovery/distillation/personas-and-journeys.md`
  and flags atoms for promotion, and "`/redkiln:closeout` performs that promotion, so
  the next initiative inherits them instead of inventing a fresh set from the same
  evidence." `.kb/product/README.md`'s "What does not belong here" section is explicit
  that an "unevidenced sketch" stays in discovery until closeout promotes it, and that
  promoting early "gives a guess the standing of a finding." This is the strongest
  available citation for DR-9's frontmatter-qualification requirement, since the README
  itself frames secondary/unobserved evidence as the exact failure mode the layer exists
  to prevent silently absorbing.
- **`.kb/maps/open-questions-index.md`** is current and lists **nineteen** open-question
  atoms (not the six DR-7 enumerates as "this initiative consumed" — those six are a
  named subset). The index's own closing instruction ("Adding an entry") confirms DR-7's
  "resolved, annotated, never removed" rule is the standing convention, not a
  project-specific invention: "A withdrawn or superseded question stays listed,
  annotated, rather than removed." Six atoms DR-7 names by filename all resolve on disk
  today: `cf-40-fixture-limits-ownership.md`, `projection-store-batch-has-no-apply-seam.md`,
  `es-38-and-gap-read-rules-are-unowned.md`,
  `global-versus-per-boundary-visibility-invariant.md`,
  `ps-1-states-no-progress-obligation.md`,
  `human-readable-payload-encoding-on-a-constrained-peer.md`.
- **`.redkiln/config.yaml:28-73`** (`verify:` block) is the literal source for DR-2's
  `e2e` vs `integration_scoped` distinction (`:55` `cargo xtask ci --fast` for
  non-terminal projects, `:60` `cargo xtask ci` — "this repository's Definition of Done"
  — reserved for the terminal project) and for `require_ledger: true` (`:67`) /
  `require_commit_provenance: true` (`:73`), which is what makes DR-3's "cited artefact,
  re-run rather than re-read" achievable at all: every producing sibling's stories are
  contractually required to carry a `_ledger.md` with real evidence.

## Tensions / open items to flag, not silently resolve

- **The evaluator-persona question (DR-10, AC-010) is still genuinely open** at
  grounding time — it is not decided anywhere on disk yet (`initiative.md` "Referenced
  personas & journeys" and `_decomposition.md` "Carried into the briefs" both carry it
  forward unresolved, and no atom, ADR or brief settles it). `_testing.md` must decide
  it with stated reasoning, not defer it further — this project's own scope statement
  ("This project designs nothing... it carries one brief deliberately") does not exempt
  it from *deciding*, only from designing the consequence.
- **DoD 13's literal phrase is already known to be false by construction** and
  `project.md`'s own risk table records the mitigation path (DR-4/AC-004 state the delta;
  the upstream mitigation is `_decomposition.md`'s gate decision #4 constraining
  `retention-and-incomplete-logs` to an answer needing no published-surface change). No
  new tension here — flagging only that `_testing.md` should not re-litigate it, just
  cite it.
- **DR-6's audit table cannot be populated at grounding/briefs time.** ADRs 0017–0028
  do not exist yet (confirmed above); the testing brief should describe the audit
  *procedure* and the expected row shape (atom id, status, rejected alternatives, or a
  stated reason for an unwritten reserved number per `RUNBOOK.md:276-283`'s precedent
  that a number staying empty can be correct), not attempt to fill in rows that don't
  exist in this tree yet.
- **`_storymap.md` is an unfilled stub.** Its backbone/slice table is empty; the
  vertical-slice story map over AC-001…AC-014 is `plan-briefs`'s to author next, per
  `project.md`'s own Companions section, and this grounding note is upstream of that
  work, not a substitute for it.
- **No tension found against any Accepted decision atom.** This project's own scope
  ("verifies and records; designs nothing") does not touch any `[FROZEN]` clause,
  propose amending an accepted ADR, or attempt to hand-author a `.kb/` atom — all three
  are the specific failure modes `0269720` and `.kb/decisions/README.md` warn against,
  and `project.md` already states the constraints correctly (DR-8, DR-12's routing rule
  for anything touching a `[FROZEN]` clause).

## Anchors for the testing brief

- `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` —
  AC-001…AC-014, DR-1…DR-13, DoD 1–7, risks table (already authored; brief must trace to
  these, not restate the charter).
- `.bklg/from-contract-to-published-library/_decomposition.md` — *Warranted briefs*
  (this project: `testing` only), *Carried into the briefs* (evaluator-persona question
  named explicitly as this project's to decide), *Decisions taken at the gate* item 4
  (retention constraint behind DoD 13's caveat).
- `.redkiln/config.yaml:28-83` — the `verify:` block and the `design:` omission
  rationale (no `design.capture`, so no perceptual review is expected here — consistent
  with "this project designs nothing").
- `.github/workflows/ci.yml:142-187` — the exact `backlog` job assertions (`validate`,
  `validate --kb`, `doctor --json` three-part `jq` check).
- `xtask/src/main.rs` module doc — the authoritative, current gate step list and ordering.
- `.kb/product/README.md`, `.kb/design/README.md` — promotion mechanism and layer
  boundaries.
- `.kb/maps/open-questions-index.md` — current nineteen-atom index and the
  never-delete convention.
- `.kb/decisions/README.md` — immutability rule; `RUNBOOK.md:262-320` — the ADR queue,
  its scheduled amendments, and the coverage-audit precedent (a phase's clause range vs.
  the union of its ADRs' ranges is "two numbers, and nothing checks that they are
  equal" — directly relevant precedent for how DR-6's own audit should be framed:
  compute both, don't assume).
- `spec/SPECIFICATION.md:219-222` — 200/198/139/49/10/2 clause figures (corrected range).
