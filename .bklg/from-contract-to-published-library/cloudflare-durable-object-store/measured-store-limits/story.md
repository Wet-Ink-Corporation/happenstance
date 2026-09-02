---
id: HS-S0055
uid: 1ca9ce
type: story
slug: measured-store-limits
title: The fixture's numeric limits are measurements, not guesses
parent: HS-P0013
initiative: from-contract-to-published-library
project: cloudflare-durable-object-store
status: in-review
process: story
stage: report
automation: HITL
severity: null
blocked_by:
  - HS-S0053
  - HS-S0054
blocks:
  - HS-S0058
  - HS-S0059
archetype: capability
slice: durable-object-conformance-run
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-20T20:42:45.046Z
links:
  pr: null
  commits:
    - 22529d516db09d2eb1b1c05ddc75e7e6dcb5097a
    - 84d5ab9331ab2acbc6afd1dddd2ad66a907834b8
    - 22529d5
    - 1c23740
  kb: []
gate_open: false
schema: 1
process_rev: b611dd09
---
# The fixture's numeric limits are measurements, not guesses

> **Title superseded 2026-08-20 by Amendment ADR-0023-A** (`spec.md`). The three ceilings
> are not measurements: they are this adapter's **declared refusal policy**, seeded from a
> documented platform cap and confirmed accepted and refused at the boundary, because no
> physical wall is observable on the runtime the gate runs against
> (`kb-decision-0023`, `kb-open-question-workerd-runner-absent-001`). `spec.md`'s heading
> carries the corrected title. The heading and `title:` field above are **not** edited
> here: the `redkiln` CLI is the only writer of an item's system frontmatter, so renaming
> the item is a `redkiln` command a human runs, and this note is the residual until they
> do.

> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
