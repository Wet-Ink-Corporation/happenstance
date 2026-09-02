---
item: "HS-S0184"
stage: implement
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Implementation Report — DT-1, DT-4, DT-5 and DT-6 resolved and recorded

## TDD Evidence

This story authors no Rust into the tree, so its tests are the probes the spec's own
verification cells name — plus one instrument the spec does not require and the story is much
weaker without: a runnable check (`check_resolutions.py`, outside the tree) that **re-derives
every number the record states**. It rebuilds the out-of-tree probe crate, runs its doctests,
re-renders it, re-measures both fences off the DOM, re-reads every emitted `id="…"`, re-runs
the anti-pattern-6 `git grep`, and compares each result against what `_resolutions.md` claims.
Transcribing a slug or a line count wrongly is a red test here, not a reviewer's job.

First run, before `_resolutions.md` existed:

```console
$ python check_resolutions.py
== HS-S0184 resolutions check ==
  [FAIL] AC-001..AC-005 the resolution record exists — …/_resolutions.md is missing
RED: 5 acceptance criteria unverifiable; the record this story authors does not exist.
EXIT: 1
```

Final run: **88/88 checks passed, GREEN**.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `check_resolutions.py` AC-001 block (9 assertions): the record names `conceptual-bridge` and `Where your streams went` as the one home, states `#where-your-streams-went`, cites the frequency evidence; and the anti-pattern-6 `git grep` is re-run against all three paths with the counts compared to the record | **Red**: record missing. **Green**: 9/9. The probe found 0 / 0 / **3** — the third being the worked example's module doc, which is a finding rather than a violation and is routed |
| AC-002 | `check_resolutions.py` AC-002 block: rebuilds the probe, re-renders it, re-measures step 3's fence off the DOM, and asserts the record's numbers match AND that height ≤ 24, width ≤ 68, hidden lines = 0; plus the three mitigations and the seven-element composition being present | **Red**: record missing. **Green**: 10/10 — step 3 measured **24 lines / 68 columns / 0 hidden**, exactly at both ceilings |
| AC-003 | `check_resolutions.py` AC-003 block (14 assertions), the load-bearing one: it **runs** `cargo test --doc` on the probe (every transcribed fence executes), asserts the wrong-side fence hides nothing, is marked at both ends, binds `EventStore` not `SendEventStore`, imports from `happenstance` not `happenstance_core`, and that both verbatim transcripts are in the record | **Red twice.** (1) record missing. (2) `the record transcribes the refusal verbatim` failed — the accepted result was pasted but its one-expression-different counterpart was in a different section, so the contrast was not readable where the resolution is certified. **Green**: 14/14 after both transcripts were placed side by side. **Red a third time, at review** — see § Post-review correction: the checker never asserted that the two fences differ in *one* place, so a second delta rode through 14/14 |
| AC-004 | `check_resolutions.py` AC-004 block (39 assertions): for all 15 anchor rows it re-derives the character count and looks the heading text up in the **rendered** slug map, failing on any predicted id; plus one map row per remaining story, the budget scoping, and `cargo xtask lints && cargo xtask spec-trace` | **Red**: record missing. **Green**: 39/39. Every id matched the render exactly, including `#where-your-streams-went` |
| AC-005 | `check_resolutions.py` AC-005 block: ≥6 statused rows, `_design.md` byte-identity, authored diff confined to the story folder, no rust-tagged fence, the probe having left nothing in the tree, and `cargo xtask affected --base main` | **Red once, and it was a real signal**: `cargo xtask affected --base main` exit 1 — `happenstance-sqlite`'s `append_returns_the_callers_own_last_position` failed with `SQLite failed: database is locked` under 64 contenders while a second cargo build was running. **Green**: exit 0 on a clean re-run, twice. Recorded as a load-sensitive flake, not fixed here |

## Commits

The slice's two checkpoints. The first exists and is citable; this story's own sha cannot be
inside this file and is recorded on the item by `redkiln record-links HS-S0184 --sha <sha>`,
which the orchestrating command runs (the CLI is the only writer of an item's `links`).

| SHA | Subject |
| --- | ------- |
| `63fa959` | `feat(application-author-path): Merge-forward preflight` — the slice-mate, whose `_baseline.md` this record consumes |
| `a5c0f30` | `Merge initiative/from-contract-to-published-library into docs-that-teach` — the merged API every probe here compiled against |

This story's checkpoint is `feat(application-author-path): Tension resolutions`, carrying
`_resolutions.md`, the five flipped ledger rows and both stage-artifact bodies.

## Changes

| File | Shape of the change |
| --- | --- |
| `.bklg/docs-that-teach/application-author-path/tension-resolutions/_resolutions.md` | **New, ~420 lines.** The deliverable: `§ DT-1`, `§ DT-4`, `§ DT-5+DT-6` (each a certification with a transcribed probe and a verdict), `§ Anchor table` (15 rows, every id read off a render), `§ Consumption map` (6 rows, one per remaining story), `§ Conditions and dispositions` (11 statused rows) |
| `.bklg/docs-that-teach/application-author-path/tension-resolutions/_ledger.md` | Five rows flipped `false → true` with cited evidence. No criterion re-worded or removed |
| `.bklg/docs-that-teach/application-author-path/tension-resolutions/implementation-report.md`, `report.md` | New stage artifacts, authored bodies |

**Nothing else.** `_design.md` byte-identical; no file under `crates/`, `examples/`, `spec/`,
`xtask/` or `docs/` touched; the probe crate lives outside the repository and only its
transcripts are committed (EC-008, NF-006), verified by
`git status --porcelain -- crates examples spec xtask docs standards` being empty.

## Gates

| Command | Result |
| --- | --- |
| `python check_resolutions.py` (the story's own tests, outside the tree) | **88/88 GREEN** |
| `cargo test --doc` in the probe crate | **2 passed, 0 failed** on the re-run — every transcribed fence compiles *and* executes |
| `cargo run --quiet --example refuse` | `Err(ConditionViolated(ConditionViolated { conflicting_position: Some(SequencePosition(1)) }))` |
| `cargo run --quiet --example narrow_min` | `Ok(SequencePosition(2))` — DT-6's assertion holds |
| `cargo doc --no-deps` in the probe crate | **exit 0**; the render every id and every geometry number is read off |
| `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`) | **exit 0** |
| `cargo xtask affected --base main` (`.redkiln/config.yaml:40`, the story grain) | **exit 0** (one earlier run flaked on a SQLite lock under concurrent load; green twice since) |
| `redkiln verify --grain story --item HS-S0184` | `[ok] affected-gate`, `[ok] ledger`, `[FAIL] boundary`, `[FAIL] provenance` — both forecast; see Notes |

The formatter is the first REQUIRED step of `cargo xtask ci --fast`
(`cargo fmt --all --check`, `xtask/src/main.rs:124`), green on the slice-mate's checkpoint and
unaffected here: this story touched no Rust in the tree.

## Notes

**The DT-6 probe was the half hour the spec said it would be, and it did not fail.** The
wrong-side fence — a guard tagged to the student the command writes rather than to the course
whose seats the invariant spans — compiles, runs, and `assert!(accepted.is_ok())` **passes**,
printing `Ok(SequencePosition(2))`. The conflicting `SeatHeld` never enters a guard tagged
`student=s1`, so the store accepts an append that exceeds capacity and nothing objects. F-6's
correction is confirmed rather than re-broken, and EC-001 did not fire.

**Two contradictions were found while certifying, and neither was absorbed.**

1. **`_design.md` disagrees with itself.** `## Composition` §6 names the bridge's wrong-side
   section `## What a type-only guard misses`, while `## Pattern decision` DT-6 was re-decided
   at the design gate (F-6) to a *too-narrowly-tagged* guard — the opposite failure. Two
   binding sections of the same signed-off file contradict each other. EC-007 applied: the
   final title `What a narrow guard misses` (26, `#what-a-narrow-guard-misses`) is supplied in
   the anchor table, the disagreement is carried as **BC-001** in
   `invariant-to-appendcondition-bridge/_conditions.md` — a named blocking condition stating
   the authoritative title pending the sign-off owner's call, and where that call is recorded —
   and `_design.md` was **not** edited.
2. **The design's sketch of the wrong-side fence cannot compile.** `_design.md:373-384` is five
   lines with `// ... same append, same condition shape ...`, and the elided lines are the
   append call and the condition — both forbidden to hide by the transience policy (`:524`).
   Written honestly it first measured **31 rendered lines and 75 columns** against the bridge
   page's 24 and 68. Reduced by the design's *own* yield order — comments to one marker line at
   each end, interior blanks removed, the tag construction bound rather than inlined, the
   assertion message shortened — it reached 24 lines and 68 columns, and after the correction
   below it lands at **23 lines and 68 columns**, still complete, still runnable, still marked
   at both ends, nothing hidden. `reconciled`, not relaxed and not reopened.

**The budget reconciliation went the way that saves the design's own anchor.** Sign-off
condition 2 says the three step surfaces are markdown; the merge made that concrete (`docs/`,
pinned at `xtask/src/narrative.rs:124`). The 22-character budget, anti-pattern 5 and the
`Long label` state all derive from rustdoc's 200px sidebar, so they bind `crate-root-encounter`
alone. Consequence: `Where your streams went` is 23 characters, one over — and needs **no
retitle**, because the budget does not bind its surface. Retitling it would have moved the one
string every other page links. The number 22 is unchanged; only the set of surfaces it governs
is now written down, which is a reconciliation and not a relaxation.

**EC-006 did not fire, and that is a result rather than a shortcut.** The three markdown
surfaces have no local renderer, so their ids could have been recorded as provisional guesses.
Instead every heading text was put through rustdoc in the probe and its id read off, and
GitHub's slugger was verified *by in-repo example* — `standards/rust/00-prime-directives.md:158`
links `spec/SPECIFICATION.md#es-1--one-definition-two-flavours-and-generic-code-binds-the-weaker-one`,
whose target heading is at `spec/SPECIFICATION.md:2508`. The two renderers agree on every
heading in the table, so no row is provisional.

**One deviation from the spec's stated method, with its reason.** AC-002 and AC-004 name
`cargo doc -p happenstance --no-deps` as the instrument. The PR boundary admits nothing under
`crates/`, so measuring step 3's program that way would have required editing
`crates/happenstance/src/lib.rs`. The probe crate is rendered by the same rustdoc on the same
pinned 1.97.1 toolchain and read the same way, and it leaves nothing in the tree — which is
what EC-008 requires. The slice-mate's `_baseline.md § Composition baseline` carries the real
crate root's own numbers, taken with the spec's exact command.

## Post-review correction, 2026-08-18

Four findings came back from the slice review. Three were rework and are done; the fourth is a
closeout carry. What re-ran is stated exactly, because a correction whose evidence is "I read
it again" is the defect the whole record is written against.

1. **AC-003's single-expression clause was unmet, and the record certified the opposite.** The
   first DT-6 transcript differed from § DT-4's step-3 program in **two** places: the guard's
   tag *and* the event appended under the condition, which was an untagged
   `Event::new("SeatHeld", &b"{}"[..])?`. The second delta is on tags — the axis the lesson
   exists to teach — so the contrast did not isolate the guard, and the shape
   `invariant-to-appendcondition-bridge` would have rendered shipped an untagged domain event.
   **Rebuilt and re-run**, not re-read: the out-of-tree probe crate was reconstructed against
   `crates/happenstance` by path, the wrong-side fence rewritten so the only delta is
   `QueryItem::new(["SeatHeld"], held.clone())` → `QueryItem::new(["SeatHeld"], narrow)`, and
   `cargo test --doc` (**2 passed, 0 failed**), `cargo run --quiet --example refuse`
   (`Err(ConditionViolated(…))`), `cargo run --quiet --example narrow_min`
   (`Ok(SequencePosition(2))`) and `cargo doc --no-deps` all executed on rustc 1.97.1. The
   geometry was read off the DOM the same way as before — the `<pre class="rust
   rust-example-rendered">` block, tags stripped and entities unescaped — and measures **23
   rendered lines / 68 columns / 0 hidden**, inside the bridge page's 24 and 68. The
   certification in `§ DT-5+DT-6` is re-stated against that fence, and the finding is row
   **F6** in `§ Conditions and dispositions` rather than a silent edit. The checker that gave
   14/14 never asserted the *count* of deltas, which is why it passed: a rule that cannot fail
   the wrong implementation is decorative, and this is the concrete instance.
2. **The record was reachable from nothing outside its own folder.** AC-004 exists to make
   "decide once, cite one place" a mechanism, and the six consuming stories' specs still
   re-derived the DT-1 slug from `_design.md:108-114` because nothing pointed them here. Each
   of the six now carries a `_resolutions.md` row in its own `## Anchors (progressive
   disclosure)` table naming the sections it loads and when to open them, and `§ Consumption
   map` states the reciprocity as a checkable rule. `git grep -l "_resolutions.md" -- .bklg`
   now returns all six spec files rather than this folder alone.
3. **The §6 retitle was a second live string, not one string.** `_resolutions.md` fixed
   `What a narrow guard misses` while `invariant-to-appendcondition-bridge/spec.md:429` still
   states its own AC-004 against `## What a type-only guard misses`, and the "note to the
   sign-off owner" existed nowhere in that story's folder. It is now **BC-001** in
   `invariant-to-appendcondition-bridge/_conditions.md`: a named condition that blocks that
   story's completion, names `What a narrow guard misses` as authoritative pending the sign-off
   owner's call, and records that this is a *semantic* rename — `conceptual-bridge` is not
   budget-bound, so the old title is wrong about what the section teaches rather than too long.
4. **Non-blocking, carried rather than reworked.** The slice-mate's PR-boundary fence was
   completed *after* its conflict resolutions twice, against its own EC-001; the review
   verified all seven resolved files independently and found nothing hidden by it. Carried to
   HS-P0025 in `merge-forward-preflight/_baseline.md § Carried forward to HS-P0025`, with the
   `redkiln verify --grain story` "arrived by parentage" question (EC-007) recorded beside it.
