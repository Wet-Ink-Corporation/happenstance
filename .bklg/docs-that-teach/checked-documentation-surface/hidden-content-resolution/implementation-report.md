---
item: "HS-S0140"
stage: implement
created: "2026-08-18T01:00:00.000Z"
updated: "2026-08-18T01:00:00.000Z"
---

# Implementation Report — Hidden content is inside the check, or absent

## TDD Evidence

Twelve tests were written into `xtask/src/lint_narrative.rs`'s `#[cfg(test)] mod tests`
against a `check_hidden_markers` with an empty body, a `HIDDEN_MARKERS` holding **one**
token, and no call site in the walk. The one-token constant is the named wrong
implementation this story exists to reject — a case-sensitive `contains` on `<details` — so
the red run exercises the set-pin as a real assertion rather than as a tautology it would
have been had the seven tokens been written first.

Red run: `cargo test --locked -p xtask --bin xtask lint_narrative::` → **52 passed; 12
failed** (the 52 are the two slice-mates', untouched). Green run: **64 passed; 0 failed**.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `every_hidden_marker_is_reported_once_per_occurrence`, `two_markers_on_one_line_are_two_problems`, `marker_and_fence_problems_arrive_in_one_list_in_source_order` | RED: `got: []` — the scan was empty and unwired. GREEN with one problem per occurrence, and a marker problem interleaved with an untagged-fence problem from the same page in line order, out of one returned list. |
| AC-002 | `the_wrapped_fixture_page_fails_by_file_and_line`, `the_same_fixture_page_without_its_wrapper_is_clean` | RED: `got: []` on the wrapped half. The clean half was green throughout and is the other required side: without it the rule cannot be told from one that rejects every page. GREEN with exactly two problems, `<details` and `<summary`, each on its own line. |
| AC-003 | `a_hidden_marker_is_matched_whatever_its_case`, `a_marker_inside_a_fence_is_still_reported`, `the_real_index_carries_no_hidden_marker` | RED: `<Details>`, `<DETAILS open>` and `{{#TABS}}` all passed a case-sensitive `contains`, which is exactly the hole. GREEN on `to_ascii_lowercase` per line. The index test reads `docs/README.md`'s real bytes and was green throughout — it pins that the pre-existing index passes unchanged. |
| AC-004 | `the_hidden_marker_set_is_pinned_to_the_design`, `every_hidden_marker_is_already_lowercase` | RED: ``` `<summary` left HIDDEN_MARKERS; shrinking it re-opens DT-7 and requires a new design record, not an edit ``` — the failure names the token, in both directions, per RS-81-5. GREEN with all seven. |
| AC-005 | `a_marker_problem_is_the_composed_line_the_design_specifies`, `forty_markers_print_as_forty_lines`, `marker_problems_are_ordered_by_page_then_line`, `the_marker_scan_adds_no_step_or_banner_of_its_own` | RED: `got: []` on the first three. GREEN with the whole line asserted character for character against `_design.md` `## States`. The fourth was green throughout and is the executable form of AC-005's review clause: the composition root mentions no marker anywhere. |
| AC-006 | `no_input_makes_a_hidden_marker_pass`, `the_marker_scan_has_no_allowance_environment_or_cfg_hook`, `the_module_docs_state_the_marker_scans_limits` | RED: `got: []` on the six inputs, and `the module docs must state \`a spelling this set does not carry\``. GREEN once the scan and the three limits landed. The hook test reads the scan's own body and asserts it contains no `IGNORE_ALLOWANCES`, `env::var`, `cfg(` or `feature =`. |

**Two tests failed on the first green run and were fixed rather than weakened.**
`every_hidden_marker_is_reported_once_per_occurrence` asserted *one* problem for each token,
and ` ```admonish ` is itself a fence opener — so the slice-mate's unterminated-fence check
correctly fired too. The fixture now closes the fence and the assertion filters to marker
problems, which keeps "one problem per occurrence" intact and stops asserting that no other
check may ever fire. `nothing_in_the_module_claims_a_page_teaches` failed on the word
**`unverified`** inside a new doc comment; the prose was reworded ("a mechanism whose
behaviour nothing in this repository can observe"), because that test is project DoD item 8's
mechanical half and gutting it to keep a phrase would be the wrong trade.

## Commits

| SHA | Subject |
| --- | ------- |
| `<checkpoint>` | feat(checked-documentation-surface): Hidden content resolution |

Recorded as first written; the reachable commit is in the slice digest.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `xtask/src/lint_narrative.rs` | `HIDDEN_MARKERS` (`:208-216`) — the seven tokens from `_design.md` `## Signatures`, with a doc comment (`:180-207`) naming DT-7, stating that there is deliberately no allowance list and why it is not symmetric with `IGNORE_ALLOWANCES`, and that shrinking the set requires a new design record. `check_hidden_markers` (`:727-740`) — a pure function over the page, line-based over the whole file, ASCII-case-insensitive, one problem per occurrence. Its call site inside the existing walk (`:750`), so marker problems interleave with fence problems in line order and are counted by the same `bail!`. Module docs gained three limits (`:35-47`), first rather than last. |
| `.bklg/…/hidden-content-resolution/` | `_ledger.md` flipped with evidence; this report and `report.md`. |

Nothing else. No file under `docs/`, no `Step`, no subcommand, no banner, no dependency, no
`cfg`, no feature, and no allowance list.

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test --locked -p xtask --bin xtask lint_narrative::` | 64 passed; 0 failed (red: 52 passed, 12 failed) |
| `cargo clippy --locked -p xtask --all-targets --all-features -- -D warnings` | clean, no new `#![allow]` |
| `cargo test --locked -p xtask` | 338 across four targets; 0 failed; 3 ignored |
| `cargo fmt --all --check` | clean, run last |
| `cargo run --locked --quiet -p xtask -- narrative` | `  1 pages, all consistent`, exit 0 |
| `cargo run --locked --quiet -p xtask -- ci --fast` | `all required checks passed (--fast: 4 optional step(s) not run)` — the bar for a non-terminal story |
| `cargo run --locked --quiet -p xtask -- affected --base main` | the checker runs in the unconditional file-reading block, then `affected gate passed` |

**The observed `fail-hidden-marker` state.** A page carrying the wrapper was written to
`docs/folded.md`, observed, and **deleted in the same session** — a committed wrapper under
the pinned tree would fail `cargo xtask ci` forever, which is the whole reason the negative
fixture is test material:

```
  docs/folded.md:7 — `<details` is a hidden panel; DT-7 forbids it in docs
  docs/folded.md:8 — `<summary` is a hidden panel; DT-7 forbids it in docs
  xtask/src/narrative.rs — does not include folded.md; its examples are never compiled
  xtask/src/narrative.rs — no `mod folded`; one module per page is what keeps a doctest failure's line number relative to the page

xtask failed: 4 problem(s) in docs
```

`git status docs/` is clean afterwards, and `cargo xtask narrative` returns to
`  1 pages, all consistent`.

## Notes

**DT-7 was not re-derived.** `_design.md` D2 closed it on 2026-08-17 with no conditions, and
this story implements its third clause only. Project AC-006's second arm — observing a claim
broken inside a non-default panel — is **not built**, deliberately: that observation would
establish the property for one construct, one extractor and one toolchain, and nothing would
notice it regressing. That is a rule no implementation can fail, which is the shape this
repository refuses for conformance rules and refuses here one level up.

**The scan is line-based over the whole page, fences and index included.** A fence-aware scan
is the most plausible "improvement" available and it is refused in EC-003's own words: a
marker quoted in a fence still renders as a page telling a reader to fold something, and there
is no allowance path to exempt it. The consequence is stated rather than hidden — a future
page under `docs/` documenting this very rule cannot quote `<details` — and the checker's own
module docs are outside `TREE`, so the rule can document itself safely.

**The problem line does not carry a column.** `_design.md`'s location form is
`{path}:{line}`, and adding a column would change the shape of the primary element. Two
markers on one line are therefore two problems at the same location, each naming the token it
matched, which is what makes the `bail!` count the number of things to fix.

**No hook was left for a future allowance.** `_design.md` `## Open questions` item 3 says a
genuinely non-normative disclosure use petitions through HS-P0021 in its own change with its
own falsification; `the_marker_scan_has_no_allowance_environment_or_cfg_hook` is the test that
keeps this diff from quietly pre-building it.

**What is still not this story's.** DT-8's aside/constraint line is HS-P0021's. The threshold
(≤ 3 scopes × ≤ 25 rendered lines), the 250-line page cap and the 40-character H1 cap are
review rules by decision; only the path-length budget became a gate rule, and that is the
slice-mate's pinning check. The recorded `cargo xtask ci` failure-and-recovery project AC-003
requires is `observed-failure-falsification`'s, on a *fence*, not on a panel — the transcript
above is this step's output on a temporary page and is not offered as that evidence.
