---
item: "HS-S0003"
stage: implement
created: "2026-08-13"
updated: "2026-08-13"
---

# Implementation Report — DT-3 and DT-8 resolved in the public-API design record

## TDD Evidence

Prose deliverable, no executable change — the Testing brief names the design-stage
review as the *only* instrument for project AC-006, *"named here only so no story
tries to invent a test for it"*. So each AC's **deterministic half** is a static
assertion over `_design.md`, and those were written first and run against the
file as the design gate signed it off, when every section this story fills read
`N/A — no user-facing surface.`

Red: 38 of 53 assertions failed. Every failure asserted missing content, not a
parse error — the headings all existed and matched, which is why five checks
(`## Signatures` present, `## Anti-patterns` present, `surfaces: []` intact, the
prior sign-off intact, the diff clean) passed at red and are recorded as
non-regression guards rather than as red-then-green evidence.

Green: 53/53.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | DT-3 section present; `Capability::declined`, `NO_CEILING_REASON` and `per-adapter` all named; `cf-40-fixture-limits-ownership` cited | **red** 5 `FAIL` → **green** 5 `PASS`. One authoritative source named, all three reason-writers disposed in a table, CF-40 cited without preferring a reading. |
| AC-002 | `wasm32`, `__emit_wasm`, `RuleOutcome` and `capability_skips_are_reported` all named inside DT-3 | **red** 4 `FAIL` → **green** 4 `PASS`. The measured no-op, the `console_log!` route, and the value-assertion half. |
| AC-003 | `refused_reset_changes_nothing`, `READS_THROUGH_BATCH`, `SECOND_HANDLE` all named | **red** 3 `FAIL` → **green** 3 `PASS`. Three constants enumerated with MUST-ness, reason-author and gated rule; the "nothing declinable" outcome recorded as a finding that does not obtain. |
| AC-004 | DT-8 section present; `projection_store_conformance!`, the *no new edge in the graph* bound, and `documented-extension-surface` named | **red** 4 `FAIL` → **green** 4 `PASS`. Outside-author arm taken, cost stated, HS-S0015's obligation named. |
| AC-005 | `## Signatures` filled; `type Batch;`, `fn begin(&self) -> Self::Batch`, `fn reset(`, `ProjectionProbe`; ≥20 `PS-` ids | **red** `PASS` (heading) + 5 `FAIL` → **green** all pass, **56** `PS-` ids, zero unattributed signatures. |
| AC-006 | `(None, true)`, the two-enums sentence, the `one-shot` reason, *no idea what the read model is* | **red** 4 `FAIL` → **green** 4 `PASS`. Four rationale sentences, four items, each once, each with a rustdoc home. |
| AC-007 | `ResetError::Refused` named with a decision, not only a signature | **red** `FAIL` → **green** `PASS`. Decided bare, with where the reason lives instead, the cost, and the reversal path. |
| AC-008 | `#[non_exhaustive]`, `lib.rs:98-124`, `[features]`, `doc(cfg(`, `unstable-projection`, `intra-doc` | **red** 6 `FAIL` → **green** 6 `PASS`. Three lines per item for all eight; both halves of the mount; the rustdoc hazard stated. |
| AC-009 | `## The doctest` names `crates/happenstance-core/src/projection.rs`, `cargo test --doc`, and the bare-`compile_fail` rule | **red** 2 `FAIL` (2 already passed on the heading and the path in the Surfaces prose) → **green** all pass. |
| AC-010 | `async_trait`, `type Batch: Send`, `ICE`, `ProjectionId::new` | **red** 4 `FAIL` → **green** 4 `PASS`. Ten entries, every one with a path or an atom id. |
| AC-011 | `surfaces: []` and the prior sign-off intact; an appended block that says `Unsigned`; only `N/A` lines removed; lines 1–50 byte-identical | **red** 2 `FAIL` (no amendment block) with the three non-regression guards already green → **green** all pass. Exactly **ten** deletions in the whole file, all the placeholder line. |
| AC-012 | empty `git diff --stat main...HEAD` under `crates/`, `spec/`, and `.kb/` outside `_intake/` | **green** at red and at green — a non-regression guard for a boundary this story must not cross, stated as such rather than dressed up as red-then-green. |

Check script: `scratchpad/design-checks.sh`, run as `design-checks.sh <rev>` for a
committed tree and with no argument for the working tree. Kept out of the
repository: this story's PR boundary is two paths and a shell script is neither.

## Commits

One checkpoint commit, no fixups. Its sha cannot be written into a file the commit
contains; it is recorded on the item through `redkiln record-links` and returned in
the slice digest.

| SHA | Subject |
| --- | ------- |
| *on the item's `links.commits`* | `feat(projection-store-freeze): The projection API design record` |

## Changes

Two paths. `redkiln verify --grain story` reads the spec's first fenced PR-boundary
block and fails on anything outside it; nothing is outside it.

| Path | Shape of the change |
| --- | --- |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | **Amended, additively.** Ten `N/A — no user-facing surface.` placeholders replaced with real content, in the bundled template's own slots and order: `## Items` (nine ids, ordered as the trait declares them, each with its claiming story), `## Signatures` (the trait transcribed item for item, 56 `PS-` ids), `## Shape decision` (the DT-3 and DT-8 resolutions plus a pointer to the ADR records for everything else), `## Placement and re-export` (both halves of the mount), `## Visibility and stability` (three lines per item, plus the intra-doc hazard), `## What it costs a caller` (four rationale sentences with homes), `## What a user meets first` (the module doc's required order), `## The states the API must express` (nine states plus the `ResetError::Refused` payload decision), `## Anti-patterns` (ten entries with evidence), `## The doctest` (example, home, gate step, and the owed bare `compile_fail`). Lines 1–50 and `## Sign-off` are byte-identical; a new `## Amendment sign-off` block is appended **unsigned**. |
| `.bklg/.../projection-api-design-record/**` | `_ledger.md` (twelve rows flipped `false` → `true` with evidence; no criterion touched), this report, and `report.md`. |

## Gates

| Command | Result |
| --- | --- |
| `scratchpad/design-checks.sh HEAD` (red baseline) | 38 `FAIL`, exit 1 |
| `scratchpad/design-checks.sh` (green) | 53/53 `PASS` |
| `redkiln validate` | `validate passed` — this story's item, body-only discipline kept |
| `redkiln validate --kb` | `validate passed` |
| `redkiln doctor` | exactly the six expected `template-drift` advisories, plus the nine pre-existing foundation-story-not-consumed advisories the base branch already emits; unchanged |
| `cargo fmt --all --check` | exit 0 |
| `cargo xtask affected --base main` | *no package affected — nothing to compile*; **affected gate passed** |
| `cargo xtask ci` | **all checks passed** (run once for the slice; the Rust tree is untouched by all three stories) |

`cargo xtask ci` is a non-regression check only. This story compiles nothing.

## Notes

**Both DT resolutions took the arm that costs more to be wrong about, and both say
so.**

- **DT-3** could have been resolved as *"per-adapter documentation is
  authoritative"*, which is cheaper to write and impossible to check. The
  resolution names the run instead — the only source produced by the code under
  test — and then does the part that is easy to skip: disposes of all three
  reason-writers, so no second declension policy is minted by omission. The
  testkit-written reason survives as a **stated exception** rather than being
  flattened away, because `NO_CEILING_REASON`'s own argument is right and applies
  again to `READS_THROUGH_BATCH`.
- **DT-8** could have taken the narrow arm and cost nothing. It takes the
  outside-author arm and therefore *creates a live obligation*:
  `documented-extension-surface` must build a fixture from the documentation alone.
  That is the only instrument that can tell a bar held in fact from a bar claimed
  in prose, and taking the arm is what makes it owed.

**CF-40 was cited and not resolved, and the record says what it does instead.** The
atom records a contradiction imported from two accepted, immutable decisions, and
is explicit that no later wave has standing to prefer a reading. The record
therefore states only what the *projection* fixture does given that: it declares no
numeric-limit constants at all, and the projection port has no `ExceedsStoreLimit`
analogue, so the surface CF-40 is about is never reached. That is deliberately a
statement about scope rather than a second instance of the question.

**The "nothing declinable" outcome was checked, not assumed.** The UX brief's *what
would make this brief wrong* names it: a projection fixture with no declinable
capability at all makes the reporting discipline decorative and leaves initiative
AC-05 undemonstrable. Two of the three capability constants are declinable, so the
failure does not obtain — and it is written into DT-3's resolution as the finding it
would have been, rather than left for someone to notice later.

**`ResetError::Refused` stays bare, and the cost is named.** Silence would have been
the failure; a payload would have pushed a domain sentence through a port type that
cannot validate, localise or synchronise it. The record states the decision, where
the reason lives instead, what it costs an operator (one more hop), and the reversal
path (`#[non_exhaustive]`, so a payload is additive later).

**One dependency is honest rather than hidden.** This story's spec says accepted
ADR-0017 / 0018 / 0019 atoms are immutable before this record is written and that
the atom wins on conflict. The atoms are **staged, not yet accepted** — the upstream
story `projection-decision-atoms` is blocked on a human-invoked
`/redkiln:kb-ingest` wave. The record was therefore written against the long-form
records in `references/adr/` and against `spec/SPECIFICATION.md`, agrees with both,
and says so in `## Signatures`: **no sentence has had to yield**, and if the wave's
atoms differ from a record on any point this block yields and the sentence that
yielded is named there rather than the atom being edited. That is the authority
order preserved, not bypassed.

**No re-plan is raised, and the escalation path is left open.** If the design
reviewer holds that the `N/A` determination was meant to cover the type surface as
well, project DoD 7 cannot be met by any story in this project — that is a re-plan
at this story's boundary, and the amendment sign-off block says so rather than
assuming the answer. Nothing here widens a section quietly.

**One thing flagged for the reviewer.** The prior approval covered *"the
anti-patterns recorded above"* at a moment when `## Anti-patterns` read `N/A`. That
section now carries ten entries. It is called out inside the section itself and
again in the amendment block, so the reviewer sees an addition rather than a
re-reading of what they signed.
