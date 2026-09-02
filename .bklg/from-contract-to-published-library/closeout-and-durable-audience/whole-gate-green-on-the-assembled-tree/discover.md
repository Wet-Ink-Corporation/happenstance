---
item: HS-S0126
stage: discover
created: 2026-08-12T13:03:51.148Z
updated: 2026-08-12T13:03:51.148Z
template_sig: 86ce4036
rendered_sig: a6867428
---

# Discover — The whole gate green on the assembled tree, with its SHA and every skip accounted for

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-liner: "Run `cargo xtask ci`... to completion in that checkout, and record the exit code, the commit SHA, the four `wasm32` step outcomes, and the absent tool behind every step that printed `skipped`." | `_storymap.md:54` | Four distinct things must be recorded, not one aggregate pass/fail: exit code, SHA, four wasm32 outcomes individually, and a per-skip reason. |
| AC-001 / AC-002 | `project.md:192-199` | AC-001: gate exits zero, SHA recorded, every `skipped` step named with its absent tool. AC-002: the four `wasm32` steps specifically pass in *this same run*, evidencing BR-12 at closeout rather than at HS-P0013's own merge. |
| DR-2 — the whole gate, not the fast one | `project.md:132-134` | `cargo xtask ci`, the terminal grain (`.redkiln/config.yaml:60`), never `--fast` (`:55`); a `skipped` step must be tool-probe-absence, not a tool that ran and was ignored. |
| DR-5 — `!Send` survived to the end | `project.md:144-147` | The four wasm32 steps must be green *on the assembled tree*, per ADR-0001 / `CLAUDE.md` binding constraint 1, not inferred from an earlier adapter merge. |
| `xtask/src/main.rs` module doc | `xtask/src/main.rs:8-56` | The authoritative, current step order: fmt → clippy `-D warnings` → tests → `wasm32-unknown-unknown` build of `happenstance-core` → docs (with-features and `--no-default-features`) → `proof-artefact` → `spec-trace` → five file-reading lints + a sixth manifest lint → `package-check`; optional and tool-gated: feature powerset, wasm32 powerset, `cargo deny`, nightly `--cfg docsrs`. |
| `require_ledger` / `require_commit_provenance` | `.redkiln/config.yaml:67,73` | This story's own ledger must carry cited evidence per criterion and a commit; the run this story records is what `dod-set-re-observation-record` and `findings-disposition-register` later cite rather than re-run. |
| `dependsOn: clean-checkout-harness` | `_storymap.md:53` (slice-mate) | Supplies the residue-free checkout and the `_closeout-record.md` scaffold this story's run and record land in — this story cannot run without that seam existing first. |

## Questions

- **Which optional steps actually resolve on the machine this runs on?** `xtask/src/main.rs:43-50` names exactly four tool-gated steps (feature powerset, wasm32 powerset, `cargo deny`, nightly `--cfg docsrs`); `CLAUDE.md` *Commands* states `cargo-hack` and `cargo-deny` both resolve on the machine this initiative has been developed on, so a green local run should exercise them rather than print `skipped`. **Deferred to spec**: the actual tool-probe outcomes are an empirical fact of the clean-checkout environment built by the prior story, not something this discover pass can assert in advance — the spec records whichever it finds, and any `skipped` step must still name the absent tool per AC-001.
- **How are the four `wasm32` steps recorded distinctly enough that AC-002 is actually checked, not inferred from an overall green exit code?** **Answered**: the ledger records each of the four separately — the `wasm32-unknown-unknown` build of `happenstance-core`, the conformance-harness check, and the `happenstance-cloudflare` / `happenstance-neon` builds — as four cited outcomes, per the storymap's own phrasing "the four `wasm32` step outcomes" (plural, itemised).
- DoD 13's caveat (the exact tree that was published) and the evaluator-persona question do not belong to this story — this story runs the gate and records the run; it does not compare against a published tag (that is `published-tree-delta-statement`) and it authors no `.kb/product/` atom. Noted so the absence is legible rather than an oversight.

## Decision

This story is the first tree where an interaction between two independently-green sibling projects becomes visible, because every sibling met only the non-terminal `cargo xtask ci --fast` bar (`project.md` risk table, row 1) against a tree containing its own work and whatever preceded it. Running the whole gate once, on the assembled tree, is what actually tests the composition rather than nine separate green claims about disjoint slices of it. The spec that follows will specify: the literal invocation and working directory (the seam the prior story built); a step-by-step table naming every gate step from the module doc with its own outcome; and, for the four wasm32 steps specifically, individual citations rather than a folded "wasm ok" line, since that is the literal text of AC-002.

## The wrong implementation

Recording "`cargo xtask ci`: green" as a single boolean, with no separate line for each of the four wasm32 steps and no accounting of which optional steps ran versus printed `skipped` and why. It satisfies "the gate passed" and even literally satisfies a sloppy reading of AC-001, but it defeats AC-002's actual purpose: BR-12's "exercised end to end by everything that ships" is supposed to be evidenced *at closeout*, distinctly, rather than assumed from HS-P0013's own run (`project.md:198-199`) — and a single aggregate line cannot distinguish "all four wasm32 steps ran and passed" from "the four wasm32 steps were silently excluded from this invocation of `cargo xtask ci` by a misconfigured toolchain and the rest of the gate still went green." That is exactly DR-5's named risk: the `!Send` flavour quietly dropped while everything else stays green. What catches it: this story's own ledger must itemise every gate step by name, with the four wasm32 builds/checks each cited separately, and every `skipped` line naming the absent tool — a reviewer checking AC-002 against a one-line "green" has nothing to verify.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
