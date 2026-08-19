---
item: HS-P0022
stage: implementation
created: 2026-08-19T03:00:34.233Z
updated: 2026-08-19T03:00:34.233Z
template_sig: 4c5f37d6
rendered_sig: fe398193
---

# Slice ledger — The Application Author's Path

The review verdict of every vertical slice of this project, sealed as it closed.

A slice's stories commit BEFORE its adversarial review runs, so "committed" is not "approved".
This ledger is the second axis: it records which slices actually cleared review, so a re-launched
implement run re-reviews a rejected slice instead of walking past it as finished. Each row is sealed
by a commit carrying a `Slice-Verdict: <project-slug>/<slice> <verdict>` trailer — the ledger is the
human-readable record, the trailer is what resume greps.

## Verdicts

| Slice | Verdict | Story checkpoints | Sealed by |
| ----- | ------- | ----------------- | --------- |
| preflight-and-anchor | approved | merge-forward-preflight 63fa959, tension-resolutions 661ebfa | (this commit) |
| opening-encounter | changes-requested | boundary-refusal-encounter 9493276, boundary-falsification-drill cc9c4a4 | (this commit) |

## Surviving findings

For each slice whose verdict is `changes-requested`, the findings that survived the in-slice fix
pass, with the `file:line` evidence the reviewer cited. These are the prescription a resumed run —
or a human — starts from. They are hypotheses for the next reviewer to verify, not facts to trust.

### opening-encounter

- **BLOCKING** — HS-S0185 AC-007 is `satisfied: false`
  (`.bklg/docs-that-teach/application-author-path/boundary-refusal-encounter/_ledger.md:81`) and the
  deterministic gate agrees: `redkiln verify --item HS-S0185 --grain story --base main` returns
  `[FAIL] ledger - AC-007: not satisfied`. The story's own report.md:99-104 predicts exactly this and
  calls it the intended reading. The page half of AC-007 is genuinely met; the open half is the three
  inherited crate-root overages that BC-002 struck and recorded as owed to HS-P0016, which lives on
  the unmerged sibling branch and is not a destination this branch can route to. A struck clause with
  no owner is not a satisfied one, so the row is correctly red — but the slice cannot be approved
  while it is.
  **Fix:** This needs one CLI call the implementer is not permitted to make, so it belongs to the
  orchestrator or the human. Either (a) open the receiving item and flip the row against its id —
  `redkiln new story --initiative support --title "The crate-root fence and headings exceed the
  documentation density budget"`, body carrying F-1/F-2/F-3 verbatim from `_conditions.md:205-213`
  and citing `spec.md § Amendment - BC-002` as the origin, then flip AC-007 with that item id as
  evidence; or (b) record the descope formally with `redkiln advance HS-S0185 --note "descope:
  spec.md Amendment - BC-002 ...; AC-007 carried open"` as `_conditions.md:190-193` itself proposes,
  and carry AC-007 to the project-level review as an explicit open item. Do not flip the row against a
  prose sentence naming a project on another branch — that is the defect `_conditions.md:229` was
  written to refuse.

- **BLOCKING (mechanical, cheap)** — `redkiln verify --item HS-S0185 --grain story` also fails
  `provenance`: "14 file(s) changed inside this story's declared boundary and links.commits is
  empty". Both stories' story.md carry `links: commits: []` even though implementation-report.md
  records the checkpoint SHAs. Because redkiln 0.19.0 scopes the `boundary` check to `ownScope =
  ownChangedFiles(root, commitLinks(...))` and falls back to the branch-wide diff when that is empty,
  the empty provenance is also what produces the `[FAIL] boundary` naming ~200 files from the a5c0f30
  merge. I checked every slice commit's file list by hand against the two amended fences and found no
  real escape, so the boundary red is an artifact, not drift.
  **Fix:** `redkiln record-links HS-S0185 --sha 9493276` and `redkiln record-links HS-S0186 --sha
  cc9c4a4`, then re-run verify. One caution before recording the fix-pass commit: 5af116b touches
  `.bklg/.../boundary-falsification-drill/_ledger.md` and `report.md`, which sit inside HS-S0186's
  fence but outside HS-S0185's five/six-entry fence. If 5af116b is recorded against HS-S0185 the
  boundary check will go red naming exactly those two files. Record it against HS-S0186 only, or
  split it, rather than widening the encounter's fence to absorb it.

- **NOT AN ESCAPE HATCH**, stated so silence is not read as an oversight. The BC-002 amendment strikes
  one clause each from AC-002, AC-007 and AC-008 after implementation discovered them unbuildable,
  which is the surface shape of a fixmed DoD. I did not fire the detector, and the reasons are
  evidential rather than deferential: the blocker is external and mechanically verifiable (I read
  `crates/happenstance/tests/doc_budget.rs:157` and `:16` myself); the route was written into
  spec.md as EC-006/EC-008 before implementation began; the amendment is a separate human-authored
  commit that leaves every original word standing and states the loss as not delivered; and — the
  decisive tell — the implementer refused to flip AC-007, which is the opposite of gaming. Firing it
  here would push a fix agent to re-litigate a human sign-off, which is precisely the move EC-008
  exists to prevent.
  **Fix:** No change requested on this point. Carry the note forward to the project-level review so
  that closeout weighs the descope on its merits rather than rediscovering it.

- **PROVENANCE, recorded on the fix pass 2026-08-19, so an absence is not read as an omission.**
  Two commits on this slice are **cross-story slice fixes and are deliberately attributed to
  neither story's `links.commits`**: `5af116b` (the ledger/amendment reconciliation and step 2's
  corrected citation) and the fix-pass commit that follows this note (the BC-004 correction of
  step 3, the drill re-run, and the four review findings). Each touches `docs/first-encounter.md`,
  `xtask/tests/first_encounter.rs` **and both** story directories, and the two stories declare
  different PR-boundary fences: `boundary-falsification-drill/**` sits outside HS-S0185's fence
  and `boundary-refusal-encounter/**` outside HS-S0186's. Recording either commit against a
  single story turns `redkiln verify --grain story`'s `boundary` check red on files that were
  correctly changed — the caution the finding above states in its own words — and recording it
  against *both* would claim each story owns the other's directory. The slice, not the story, is
  the unit that owns them; the per-story commits recorded by `redkiln record-links` are the
  implementation checkpoints `9493276` and `cc9c4a4`, and every cross-story commit is enumerated
  here with its subject so the audit trail is complete without a fence being widened to hold it.
  `4232b34` (the two PR-boundary amendments) is a third, and is already named in both stories'
  reports. **Fix:** none requested by this note — it exists because the reviewer asked for the
  reasoning to be stated rather than inferred.

- **OBSERVATION**, no change requested. `redkiln verify --item HS-S0186 --grain story` reports
  `boundary: no boundary declared` and `provenance: no boundary declared`, even though
  boundary-falsification-drill/spec.md:199-207 does declare a seven-entry fence. Its pass is
  therefore partly vacuous — two of four checks did not run. The encounter's identically shaped fence
  at spec.md:302-309 parses fine, so this looks like a parser edge in the drill's spec (possibly the
  amendment blockquote immediately following the fence) rather than a missing declaration.
  **Fix:** Worth a look when the provenance calls above are made — if `redkiln verify` still says "no
  boundary declared" for HS-S0186 after `record-links`, it is a tooling finding for the support
  initiative, in the same shape as the redkiln#136 record already in
  `.bklg/docs-that-teach/_implementation.md:55-131`.
