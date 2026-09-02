---
item: HS-S0133
stage: discover
created: 2026-08-12T13:04:00.055Z
updated: 2026-08-12T13:04:00.055Z
template_sig: 86ce4036
rendered_sig: 64e9f9b6
---

# Discover — Backlog and knowledge base clean, with exactly six drift advisories

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-liner: "Run `redkiln validate`, `redkiln validate --kb` and `doctor --json` on the closeout tree, assert zero problems, zero `process-drift` and a `template-drift` set equal to the six CI names, and disposition any seventh or missing sixth with a decision — without ever running `adopt --templates`." | `_storymap.md:61` | Three commands, three JSON assertions, one standing prohibition. |
| AC-011 / AC-012 | `project.md:229-236` | AC-011: exactly six `template-drift` advisories, zero `problems`, zero `process-drift`. AC-012: any deviation is recorded with a decision (revert or deliberate adopt with a stated reason), and `redkiln adopt --templates` is confirmed not run either way. |
| The literal `jq` assertion this mirrors | `.github/workflows/ci.yml:176-187` | Exact equality (`sort == [...]`) against six named files: `.redkiln/templates/_design.md`, `_intake-brief.md`, `discover.md`, `gates/discover.md`, `gates/intake.md`, `spec.md`. Not a superset check. |
| Why exact-six, not "no drift"/"some drift" | `.github/workflows/ci.yml:160-171` (step comments) | "'no drift' — that would fail forever, and a warning nobody can clear teaches its reader the channel is noise. Not 'some drift' either: an EXTRA entry is a template someone changed without deciding to, and a MISSING entry means a customisation was reverted." |
| The `adopt --templates` prohibition | `CLAUDE.md`, *Where the work lives* section | "**Never run `redkiln adopt --templates`.**... it would overwrite all six customisations with the bundled defaults, silently — and the CI assertion above would then fail on the *absence* it created." |
| DR-11 — Tooling health | `project.md:175-178` | Same three commands and same exact-six requirement, restated at the project's own requirements level. |
| `dependsOn: product-atom-promotion-via-kb-ingest` | `_storymap.md:85-88` ("Why the slices are cut here") | "Health must be measured *after* the new product atoms exist (they are inputs to `validate --kb`)" — this story cannot run meaningfully before the promotion story lands. |
| `doctor`'s exit-code caveat | `.github/workflows/ci.yml:147-150` | "`doctor` exits non-zero on a problem and zero on a warning, so its exit code is captured rather than trusted" — the JSON assertions are the actual check, not the process exit code. |

## Questions

- **Does promoting three (not four) persona/journey atoms in the prior story change anything this story checks?** No — `redkiln validate --kb`'s frontmatter/status conformance check is the same regardless of atom count; this story validates whatever the prior story actually produced. **Not deferred**, since the answer does not depend on the count.
- **What if the six-file `template-drift` set has actually drifted by the time this story runs** (a seventh customisation added, or one of the six reverted, by any sibling project along the way)? AC-012 already specifies the response — record the decision (revert, or a deliberate adopt with a stated reason) and confirm `adopt --templates` was not run. **Answered by the acceptance criterion itself**; this story's spec need only state the concrete disposition procedure, not invent a new rule.
- The evaluator-persona question and DoD 13's caveat do not touch this story's own scope; the evaluator finding is routed by `findings-disposition-register`, not measured here as backlog/KB health.

## Decision

`redkiln validate`, `redkiln validate --kb` and `doctor --json` are the mechanical proof that everything the prior slices wrote — the closeout record, the audit tables, the three new product atoms — actually conforms to the schemas and process the tooling enforces, and that nothing about this project's own work introduced backlog or knowledge-base drift beyond the six deliberate template customisations this repository has carried since before this initiative began. The spec that follows will specify the exact three invocations, the `jq` assertion mirroring `.github/workflows/ci.yml:176-187` byte-for-byte (so this story's local check and CI's check cannot silently diverge), and the disposition procedure for AC-012's deviation case.

## The wrong implementation

Running `redkiln doctor` locally, seeing a green (zero) exit code, and reporting AC-011 satisfied on that basis. This is exactly the failure mode `.github/workflows/ci.yml:147-150`'s own comment warns against: doctor's exit code "conflates warnings and problems" — it can exit zero while carrying a seventh `template-drift` warning, a `process-drift` warning, or any other warning kind, none of which a bare exit-code check would surface. It would satisfy "the tool ran and reported success" while missing the actual acceptance criterion, which is a three-part JSON assertion (zero `problems`, zero `process-drift`, template-drift set exactly equal to six named files), not a process exit code. What catches it: this story's ledger must cite the parsed `doctor.json` output against the literal `jq` expression from CI, not the shell's `$?` — and if the two ever disagree (exit zero, JSON assertion fails), that disagreement is itself worth recording, since it would mean CI and this story's local run are checking different things.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
