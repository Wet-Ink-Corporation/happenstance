---
id: kb-reference-intake-citation-drift-census-001
title: Seventy-seven of 699 intake citations drifted while the briefs waited, and twenty point into pinned evidence
kind: reference
status: accepted
authority_tier: note
summary: >-
  A census of every bare path:line citation in .kb/_intake at commit bd11598,
  checked against the line content each citation named at an earlier commit
  9b06836. Of 699 citations resolvable at both commits, 77 had changed cited-line
  content; 20 of those point into references/ (pinned evidence that must not be
  repointed), leaving 57 into live files, of which 25 are mechanically
  recoverable because the old text is still findable elsewhere in the same file.
  Drift is spread over sixteen briefs, heaviest in event-metadata-floor.md (13)
  and after-opt-scope.md (12), with the heaviest targets in
  happenstance-sqlite/src/event_store.rs (19) and spec/SPECIFICATION.md (16).
depends_on: []
related:
  - kb-playbook-anchoring-citations-001
  - kb-playbook-verify-referent-report-coverage-001
  - kb-reference-phase-8-spec-reconciliation-001
  - kb-decision-0045
  - kb-open-question-disjoint-boundaries-no-clause-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/docs-citation-form-and-clause-content.md
last_reviewed: 2026-09-07
---

# Seventy-seven of 699 intake citations drifted while the briefs waited, and twenty point into pinned evidence

## What this is a moment about, and which moment

**Commits: measured at `bd11598`, cited-line content checked against `9b06836`.**
This is the only honest test available for a citation form that carries no
anchor to re-find itself by — `.kb/_intake/` is deliberately excluded from
`cargo xtask lints`' `CITATION_SCAN_DIRS`, because it is staging that
`/redkiln:kb-ingest` clears, so nothing mechanical watches it while it waits.
The census exists to answer a narrow question: does that exclusion's own
justification — "the gap closes itself at the moment it starts to matter" —
hold in practice, for a directory that in this instance held twenty-three
briefs staged for a day and citing each other.

## The count

| | count |
|---|---:|
| bare `path:line` citations resolvable at both commits | 699 |
| whose cited line's content changed | 77 |
| of those, into `references/` (pinned evidence — must not be repointed) | 20 |
| into live files | 57 |
| of the 57, mechanically recoverable (old text findable elsewhere in the file) | 25 |

Spread over sixteen briefs. The heaviest by citation count:
`event-metadata-floor.md` (13), `after-opt-scope.md` (12) and
`adapter-driver-reexport-policy.md` (9). The heaviest cited targets:
`crates/happenstance-sqlite/src/event_store.rs` (19),
`spec/SPECIFICATION.md` (16) and `crates/happenstance-cloudflare/src/event_store.rs`
(8). The trigger case that prompted the census: inserting one section into
`docs/carry-your-invariant.md` moved everything below it by 62 lines, and three
citations in one brief (`after-opt-scope.md`) pointed into the shifted region —
found only by hand, because the excluded directory gave no checker a chance to
see it.

## What the count does and does not show

**It is not a large-drift finding.** 57 of 699 is under 9%, an order
consistent with what the sibling `citation-anchor-slack.md` measurement found
across `standards/rust/`'s constitution citations (settled as `kb-decision-0045`)
— drift of roughly this order is the background rate in this repository, not an
anomaly specific to this directory. And a drifted citation is not necessarily a
wrong one: 25 of the 57
have their original text intact elsewhere in the same file (the surrounding
prose moved, not the fact), and some of the remaining 32 name text legitimately
rewritten by the very work the citing brief describes.

**It is a timing finding.** The exclusion of `.kb/_intake/` from the citation
scan is justified on the premise that the moment drift starts to matter is
*ingest* — when a brief's citations land inside `CITATION_SCAN_DIRS` and a
checker can finally see them. This census was taken to test that premise
directly, by measuring drift accumulated *before* ingest, while briefs sat
staged. It found drift already present at a strictly earlier moment: when the
repository owner reads a brief to ratify the decision it argues for, which is
the one moment the brief exists for and the one moment no checker covers,
mechanical or otherwise. At the time of this census, twenty-four findings sat
unratified behind these briefs.

**What it does not settle.** Whether the merge-coupling cost of scanning
`.kb/_intake/` (every lane that moves a line becoming responsible for every
brief any other lane wrote) outweighs this timing gap is a decision, not a
measurement, and is not this atom's to make. Nor does this census re-run
itself: it is a snapshot of one interval between two named commits, during a
period five concurrent worktrees were active over one tree, and a repeat
measurement over a different interval would need its own atom under this
layer's dating rule rather than an edit to this one.
