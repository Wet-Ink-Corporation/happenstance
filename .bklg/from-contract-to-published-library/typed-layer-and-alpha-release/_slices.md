---
item: HS-P0011
stage: implementation
created: 2026-08-16T00:19:48.476Z
updated: 2026-08-16T00:19:48.476Z
template_sig: 4c5f37d6
rendered_sig: 96fcc01e
---

# Slice ledger — The typed layer, the worked example, and 0.2.0-alpha.1

The review verdict of every vertical slice of this project, sealed as it closed.

A slice's stories commit BEFORE its adversarial review runs, so "committed" is not "approved".
This ledger is the second axis: it records which slices actually cleared review, so a re-launched
implement run re-reviews a rejected slice instead of walking past it as finished. Each row is sealed
by a commit carrying a `Slice-Verdict: <project-slug>/<slice> <verdict>` trailer — the ledger is the
human-readable record, the trailer is what resume greps.

## Verdicts

| Slice | Verdict | Story checkpoints | Sealed by |
| ----- | ------- | ----------------- | --------- |
| decision-records | approved | adr-0020-fold-query-agreement c011143, adr-0021-payload-evolution-and-codec-tag e33dc9f | (this commit) |

## Surviving findings

For each slice whose verdict is `changes-requested`, the findings that survived the in-slice fix
pass, with the `file:line` evidence the reviewer cited. These are the prescription a resumed run —
or a human — starts from. They are hypotheses for the next reviewer to verify, not facts to trust.
