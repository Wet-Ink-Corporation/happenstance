---
item: HS-P0023
stage: implementation
created: 2026-08-20T03:07:10.455Z
updated: 2026-08-20T03:07:10.455Z
template_sig: 4c5f37d6
rendered_sig: 18d02a6c
---

# Slice ledger — Reach and the Adapter Path

The review verdict of every vertical slice of this project, sealed as it closed.

A slice's stories commit BEFORE its adversarial review runs, so "committed" is not "approved".
This ledger is the second axis: it records which slices actually cleared review, so a re-launched
implement run re-reviews a rejected slice instead of walking past it as finished. Each row is sealed
by a commit carrying a `Slice-Verdict: <project-slug>/<slice> <verdict>` trailer — the ledger is the
human-readable record, the trailer is what resume greps.

## Verdicts

| Slice | Verdict | Story checkpoints | Sealed by |
| ----- | ------- | ----------------- | --------- |
| pointer-policy | approved | pointer-policy-and-inventory ef2eb6b | (this commit) |
| adapter-error-site | changes-requested | adapter-reasoning-account af9a241, store-error-site-rewrite f2c7dbe | (this commit) |

## Surviving findings

For each slice whose verdict is `changes-requested`, the findings that survived the in-slice fix
pass, with the `file:line` evidence the reviewer cited. These are the prescription a resumed run —
or a human — starts from. They are hypotheses for the next reviewer to verify, not facts to trust.

### adapter-error-site

- HS-S0155 (adapter-reasoning-account): story gate red — boundary, provenance — boundary: changed outside declared boundary: CHANGELOG.md, CLAUDE.md, Cargo.lock, Cargo.toml, RUNBOOK.md, [195 files total]; provenance: 16 file(s) changed inside this story's declared boundary and links.commits is empty. Record the checkpoint commit: redkiln record-links <id> --sha <sha>.
