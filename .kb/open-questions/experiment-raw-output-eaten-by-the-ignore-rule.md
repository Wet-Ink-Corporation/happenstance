---
id: kb-open-question-experiment-raw-output-ignored-001
title: .gitignore's *-output.txt eats the raw output experiments/ requires be committed
kind: open_question
status: accepted
authority_tier: note
summary: >-
  experiments/ladybug-driver-probes/README.md states that results/probe-output.txt is its
  output, verbatim, but no results/ directory exists in the tree. git check-ignore -v names
  the cause: .gitignore:69's *-output.txt, a pattern written at the pre-publication sweep to
  catch stray gate/test transcripts that embed an absolute C:\Users\<name> path. The pattern is
  good on its own terms and the collision is invisible on inspection: 321 files are tracked
  under other experiments/**/results/ paths and none of them is suffixed -output.txt, so every
  other experiment escapes by accident of naming rather than by design. It matters beyond one
  directory because this repository's own discipline — that a measurement lives beside its raw
  output under experiments/ — is the stated ground on which a sibling decision (ES-11's
  sufficiency condition) declines to commit to a number, while the Ladybug adapter's own
  evidence sits in exactly the position that discipline forbids without looking like it does,
  since the README transcribes the output inline and cites a path a reader will assume
  resolves. No gate checks citations inside experiments/, so nothing mechanical would have
  caught this. Three remedies of different cost are named and none is chosen: rename the file
  out of the *-output.txt shape, carve a tracked exception for experiments/**/results/, or treat
  the README's inline transcription as the evidence of record and stop citing a raw path.
depends_on: []
related:
  - kb-decision-0025
  - kb-decision-0061
  - kb-reference-ladybug-driver-probes-001
  - kb-playbook-anchoring-citations-001
source_paths:
  - .kb/_intake/2026-09-08-adr-0025-ladybug-projection-adapter.md
  - .kb/_intake/2026-09-08-adr-0061-es-11s-sufficiency-condition-assumed-a-queue.md
  - .gitignore
  - experiments/ladybug-driver-probes/README.md
last_reviewed: 2026-09-09
---

# .gitignore's *-output.txt eats the raw output experiments/ requires be committed

## What is true today

`experiments/ladybug-driver-probes/README.md:9` states, as a matter of fact
about its own directory: "`probes.rs` is the program; `results/probe-output.txt`
is its output, verbatim." No `results/` directory exists anywhere under
`experiments/ladybug-driver-probes/` — the directory holds only
`Cargo.toml.reference`, `README.md` and `probes.rs`. Running
`git check-ignore -v experiments/ladybug-driver-probes/results/probe-output.txt`
resolves the cause precisely: `.gitignore:69` — the bare pattern `*-output.txt` —
matches it.

That line is not a mistake in isolation. `.gitignore`'s own comment above it
explains what it was written for: stray transcripts from commands like
`cargo test > test-output.txt`, run at the repository root, which is where they
land because they are not caught by a `target/` exclusion. Five such files were
found sitting untracked-and-unignored at the pre-publication sweep, each
embedding an absolute `C:\Users\<name>` path, each one `git add -A` away from
being committed to a repository about to go public. The pattern closes a real
leak. It was not written with `experiments/**/results/` in mind, and nothing
about its wording excludes that tree.

The collision is invisible by inspection because it is rare in practice:
`git ls-files` shows 321 tracked files under other experiments'
`results/` directories, and zero of them match `-output.txt`. Every other
experiment in the repository escapes the pattern by accident of filename
choice, not because anyone reasoned about the overlap. `probe-output.txt` is,
as far as this search found, the only place the two rules — "raw output is
committed under `experiments/**/results/`" and "`*-output.txt` is never
committed" — actually collide.

## What is not decided

This finding was surfaced during this wave's provenance check, not raised by
any staged intake file — the intake documents for ADR-0025 and ADR-0061 cite
or rely on the experiment's evidence without flagging that the cited raw file
does not resolve. It bears on both: ADR-0061 refuses to state a failure rate
for ES-11's sufficiency condition on the stated ground that a measurement
belongs beside its raw output under `experiments/`, and ADR-0025's own
evidence — the Ladybug driver probes — sits in exactly the position that
ground forbids, while reading as though it does not, because the README
transcribes the probe output inline and a reader has no reason to doubt the
`results/probe-output.txt` citation until they look for the file.

No remedy is chosen here, and three have different costs:

1. **Rename the file** out of the `*-output.txt` shape (e.g.
   `probe-transcript.txt`) — cheapest, but a naming convention now exists by
   negative example only, and the next experiment can repeat the mistake
   under a different filename that happens to match some other broad pattern.
2. **Carve a tracked exception** — a `!experiments/**/results/*-output.txt`
   negation, or narrowing line 69 to the repository root — fixes the class,
   but widens the surface `.gitignore`'s pre-publication rule was written to
   close, and needs the same scrutiny that rule got.
3. **Treat the README's inline transcription as the evidence of record** and
   stop citing a `results/` path that was never meant to be committed —
   cheapest to state, but it means the experiment's citation style silently
   differs from every sibling experiment's, which does commit its raw output.

## What forces it

Nothing forces a choice today; the gap costs nothing until a reader — or a
gate — tries to open the cited file and finds it absent. No CI step checks
citations inside `experiments/`, so this can recur silently in any future
experiment that happens to name its transcript `*-output.txt`. It becomes
urgent the first time a decision atom's authority is challenged on the
specific ground that its cited raw evidence does not exist.

## Ordered sub-questions

1. Is the fix scoped to `experiments/ladybug-driver-probes/` alone, or does a
   grep for other `*-output.txt` citations under `experiments/**/README.md`
   need to run before this is called closed?
2. If a `.gitignore` exception is chosen, does it need the same
   pre-publication scrutiny (no absolute paths, no credentials) applied to
   the file it un-ignores, given that scrutiny is exactly why line 69 exists?
3. Should `references/evaluation` or `spec-trace`-style tooling gain a check
   that every path an `experiments/**/README.md` cites as output actually
   resolves, so this class of drift is caught mechanically rather than by a
   later wave's provenance check?
