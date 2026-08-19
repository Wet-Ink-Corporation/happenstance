# Wave `2026-08-15-adr-0030-checkpoint-progress` — integration summary

Five operations from **one** intake document. Three atoms created, two amended, zero superseded by
a decision, zero deferred. The compression runs the other way from the 2026-08-13 wave: that one
took four documents to six operations; this one took a single document to five, because the
staging note carried findings beyond its own headline decision and each found a different layer.

## Atoms created

| # | destPath | id | kind | authority_tier | status |
| --- | --- | --- | --- | --- | --- |
| 1 | `.kb/decisions/0030-the-checkpoint-reports-the-commits-that-happened.md` | `kb-decision-0030` | decision | decision | accepted |
| 4 | `.kb/open-questions/ps-32-adr-0007-context-correction-is-owed.md` | `kb-open-question-ps-32-adr-0007-correction-owed-001` | open_question | note | accepted |
| 5 | `.kb/reference/spec-trace-has-suite-family-switch.md` | `kb-reference-spec-trace-has-suite-001` | reference | note | accepted |

**Op 1 — ADR-0030.** `supersedes: null`, confirmed against the record's own *"Amends nothing, edits
nothing"* bullet. The decision is narrower and sharper than "widen PS-19": §4 obliged `commit` to
**couple** its two writes and never to **advance** anything, so PS-1's `MUST` is satisfied by a
store that makes neither write durable — the *"or not at all"* arm — while four rules assert
progress, three that exist and `fresh_projection_has_no_checkpoint` which does not. Mirrored on
`kb-decision-0019`'s shape (Decision / Provisional / Alternatives rejected / Scope and what is left
open), 579 words.

**Op 4 — the PS-32 debt, finally captured.** ADR-0017's record has stated since the 2026-08-13 wave
that it corrects one sentence of ADR-0007's Context and that the correction belongs to a
superseding atom rather than an edit. That wave declined to write it — correctly, since authoring
it would have been an ingest taking a decision nobody signed — and it has been carried as an
unrouted obligation since. It is now an atom, so the debt is findable by the corpus rather than by
whoever remembers the retrospective. **The correction is still not performed**; see `04`.

The atom adds one finding beyond its intake material, and it narrows the debt usefully:
`kb-decision-0007`'s **atom** does not repeat the defective sentence — only the long-form record at
`references/adr/0007-projection-runner-decodes.md:36-38` does. So what is owed is a correction to
the record, not a supersession forced by the atom's own text.

**Op 5 — a gate finding, filed where gate findings go.** `xtask/src/spec_trace.rs`'s `has_suite`
per-family switch (fn at `:1750`, the loop's abstention check at `:699-701`). Placed in
`.kb/reference/` rather than merged into `kb-reference-phase-4-5-spec-reconciliation-001`, because
that atom is a dated 2026-08-10 census pinned to six commits and the reference README's dating rule
forbids growing it. Every cited line was verified in the tree.

## Atoms amended

| # | destPath | id | change |
| --- | --- | --- | --- |
| 2 | `.kb/open-questions/ps-1-states-no-progress-obligation.md` | `kb-open-question-ps-1-no-progress-obligation-001` | `accepted` → **`superseded`**; answered by `kb-decision-0030` |
| 3 | `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md` | `kb-open-question-ps-19-scope-narrower-001` | `accepted` → **`superseded`**; answered by `kb-decision-0030` |

Both were amended in the shape the 2026-08-13 wave established and both followed it exactly: the
prior body is byte-identical above the new section, the 2026-08-13 amendment clauses survive
verbatim, one dated section is appended rather than rewriting a question into its own answer, and
**`superseded_by` was again deliberately not added** — it is a decision-only key, and inventing it
on an `open_question` validates silently under `.passthrough()` and is then read as corpus fact.
The edge is carried by `related` instead. `kb-decision-0030` already carried the reciprocal link,
so **no accepted decision was opened or edited** to wire it.

Op 3's diff is `+48/-4`, the four being frontmatter lines rewritten in place.

**Two questions the 2026-08-10 wave opened are now closed by one decision.** That wave parked
thirteen `requires-new-decision` claims as seven questions rather than mint decisions an ingest had
no authority to make; five of those seven now have answers, all arriving through the ADR pass.

## Map atoms updated

`.kb/maps/decision-map.md`, `.kb/maps/domain-map.md`, `.kb/maps/open-questions-index.md` — the
ADR-0030 row, the domain placement, and the index rows for two questions moving to `superseded`
plus one new question appearing.

## Intake cleared

`.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md`. `README.md` was dropped from the ingest
set at the approval gate and remains.

## Verification

`redkiln validate --kb` — **passed, exit 0**, run twice: once by the workflow's Verify phase and
once by hand after the intake clearing. No duplicate ids, every reference resolves, `KbFrontmatter`
conformance holds on all five touched atoms, accepted-decision immutability holds against `HEAD`.

`redkiln doctor` — **exit 1, and not on this wave's account.** The same nine
`unconsumed-foundation` errors as the 2026-08-13 wave, all in `.bklg/`. See `04-retrospective.md`.
