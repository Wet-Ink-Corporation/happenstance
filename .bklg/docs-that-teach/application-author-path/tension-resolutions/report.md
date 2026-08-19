---
item: "HS-S0184"
stage: report
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Report — DT-1, DT-4, DT-5 and DT-6 resolved and recorded

## Findings Ledger

The story's outcome as the review gate reads it. Mount point on every row:
`.bklg/docs-that-teach/application-author-path/_design.md` — this story mounts **onto** the
signed-off design by publishing the values its sign-off conditions leave outstanding, at a path
the four downstream specs open by name. The deliverable is
`.bklg/docs-that-teach/application-author-path/tension-resolutions/_resolutions.md`.

| AC | Result | What proves it | Deferred / routed |
| --- | --- | --- | --- |
| AC-001 — DT-1 certified; one decision, one location, one prior named once | **met** | `_resolutions.md:38-82` § DT-1. Names the `conceptual-bridge` heading `Where your streams went` / `#where-your-streams-went` as the one reader-facing home, confirmed against a render. Anti-pattern 6 baseline probe run on the merged tree and transcribed: **0** on `crates/happenstance/src/lib.rs`, **0** across `docs/`, **3** in `examples/course-subscriptions/src/main.rs:3,11,12`. Verdict `holds`. 9/9 assertions green in `check_resolutions.py` | The three occurrences in the worked example's module doc — inside material AC-010 requires surfaced verbatim, on a surface anti-pattern 6 does not govern — routed to `surface-course-subscriptions` (row F3) |
| AC-002 — DT-4 certified with all three mitigations, and step 3 **measured** | **met** | `_resolutions.md:84-176` § DT-4. Step 3's program written against the merged API, compiled and executed, rendered by rustdoc on the pinned 1.97.1 toolchain, measured off the DOM: **24 rendered lines** against a 24-line budget, **68 columns** against 68, **0 hidden lines**. Verdict `holds`; EC-003 did not fire | The zero headroom is carried to `boundary-refusal-encounter` as a constraint: one added line breaks a budget. The 93-character `Debug` payoff line (F-3 said 92) is carried with its existing mitigation |
| AC-003 — DT-5+DT-6 certified as one resolution; the wrong side compiles, runs, and is accepted | **met** | `_resolutions.md:178-296` § DT-5+DT-6. `cargo test --doc` → 3 passed, 0 failed. `cargo run --example narrow_min` → **`Ok(SequencePosition(2))`**; `assert!(accepted.is_ok())` **passes**. One expression different, `--example refuse` → `Err(ConditionViolated(…))`. Binds `EventStore` not `SendEventStore`; imports `happenstance` not `happenstance_core`; hides nothing; marked at both ends. Zero allowance-list entries; **UX-015 vacuously satisfied, with its reason**; **AC-013 discharged explicitly**, in writing. Verdict `reconciled` | The design's sketch at `:373-384` is an elision that cannot compile and cannot be hidden into compiling. Written whole it measured 31 lines / 75 columns; reduced by the design's own yield order to 24 / 68. Reconciliation stated; nothing relaxed, nothing reopened |
| AC-004 — every heading fixed once, every fragment id **read** off a render | **met** | `_resolutions.md:298-372`. § Anchor table: 15 rows, each with final text, character count, whether the budget binds, and an id copied out of rustdoc's emitted `id="…"` — all 15 re-derived and matched by the check. Retitle supplied: `Watch a boundary refuse` (23) → **`A boundary refuses`** (18). § Consumption map: one row per remaining story, with its binding `_design.md` sections, its anchor rows, and the one DT-1 citation string. `cargo xtask lints && cargo xtask spec-trace` exit 0 | **Budget scope reconciled, not relaxed**: 22 characters and anti-pattern 5 derive from rustdoc's 200px sidebar, so they bind `crate-root-encounter` only; on the markdown step surfaces anti-pattern 5 is a check no page can fail and is scoped rather than deleted. Consequence: `Where your streams went` (23) needs no retitle and the design's cited anchor survives |
| AC-005 — every condition and disposition statused; the design untouched | **met** | `_resolutions.md:374-403` § Conditions and dispositions, 11 statused rows: both sign-off conditions (C1 `holds`, C2 `reconciled`) and all four merge dispositions (D1–D4), each three fields, each preserving the design's original *reason* where the row's status changed. `git diff --stat 3fd3866 HEAD -- …/_design.md` empty. Authored diff two files, both inside this story's folder. Zero teaching content; the probe left nothing in the tree. `cargo xtask affected --base main` exit 0 | Five further findings F1–F5 recorded rather than absorbed, including `_design.md` contradicting itself (`## Composition` §6 says "type-only guard"; DT-6 under F-6 says "tagged too narrowly") — routed to the sign-off owner as a note on `invariant-to-appendcondition-bridge`, with `_design.md` **not** edited (EC-007) |

**Nothing is blocked, and nothing is `raised as reopen condition`.** Every probe that could have
failed passed. Had the wrong-side fence refused (EC-001) or been unwritable (EC-002), this story
would have escalated to the sign-off owner and no downstream page could have started.

**The one process discrepancy, recorded not silenced.**
`redkiln verify --grain story --item HS-S0184` returns `[ok] affected-gate`, `[ok] ledger`,
`[FAIL] boundary`, `[FAIL] provenance`. Both are the slice-mate's EC-007 one story over: the
branch carries merge `a5c0f30`, so every file its second parent brings reads as "outside the
fence" to a tool that cannot express "arrived by parentage", and `links.commits` is written by
`redkiln record-links` after the checkpoint exists. This story's authored diff is two files.
Neither failure was answered by widening the fence.

## Acceptance

| AC | Verification status |
| --- | --- |
| AC-001 | satisfied — ledger row cites `_resolutions.md:38-82`, the transcribed `git grep` probe and the `check_resolutions.py` AC-001 block |
| AC-002 | satisfied — ledger row cites `_resolutions.md:84-176`, the 24-line / 68-column / 0-hidden measurement taken off the render |
| AC-003 | satisfied — ledger row cites `_resolutions.md:178-296`, `cargo test --doc` 3 passed and both `cargo run --example` transcripts |
| AC-004 | satisfied — ledger row cites `_resolutions.md:298-372`, all 15 ids re-derived from the render, 6 consumption-map rows |
| AC-005 | satisfied — ledger row cites `_resolutions.md:374-403`, byte-identity of `_design.md` and the confined diff |

Tiers 2 and 4 are n/a with the reasons `spec.md § Tests and CI` states. Tier 3 was exercised
out-of-tree by design (`§ PR boundary`), and its transcript is the artifact.

## Knowledge Harvest

Four candidates for `.kb/` at closeout, each earned by a measurement rather than a preference:

1. **A design that mandates a value and declines to supply it will be supplied four times.**
   `_design.md:582-584` states the retitles are structural and does not give them. Fixing every
   heading text and its emitted id once, before any page renders one, cost nothing today; the
   same decision taken per page would have produced four answers and a set of inbound links
   correct only by luck.
2. **Scope a check before deleting it.** Anti-pattern 5 (a sidebar TOC entry clipped with an
   ellipsis) can never fire on a markdown surface, which makes it decorative by CLAUDE.md's own
   test. The right move was neither deleting it nor leaving it universal but **stating the
   surfaces it governs** — and the reconciliation then *saved* the design's own cited anchor,
   because the 23-character heading it cites lives on a surface the budget does not bind.
3. **A fence that elides cannot compile, and hiding the elision is usually forbidden.** The
   design sketched the wrong-side contrast as five lines with `// ... same append, same
   condition shape ...`. The elided lines were exactly the ones the transience policy forbids
   hiding. The general rule: *an illustrative excerpt and a compiled fence are different
   artifacts, and a design that asks for both in one block has not yet decided.*
4. **The quiet failure is the one worth compiling.** DT-6 asserts that a too-narrowly-tagged
   guard **accepts** an append that should have been refused, and the probe proves it:
   `Ok(SequencePosition(2))`. Its mirror, CF-7's broadening failure, would have made the same
   assertion fail. Getting that backwards was the blocking defect at the design gate (F-6), and
   it stayed corrected only because something ran it.
