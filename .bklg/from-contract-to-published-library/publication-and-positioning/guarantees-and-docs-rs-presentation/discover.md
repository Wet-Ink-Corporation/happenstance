---
item: HS-S0094
stage: discover
created: 2026-08-12T13:03:02.959Z
updated: 2026-08-12T13:03:02.959Z
template_sig: 86ce4036
rendered_sig: 1f1661b8
---

# Discover — What depending costs, stated where a consumer reads it, on a page where nothing is invisible

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Storymap slice | `_storymap.md`:66 | State the cost of depending in the Guarantees slot (MSRV promise, minor-bump-is-breaking, `wasm32` feature line); add the missing `[package.metadata.docs.rs]` block to `crates/happenstance/Cargo.toml`; make the quick-start snippet the crate's own doctest via `crates/happenstance/src/lib.rs`:10 so it is the same text `stranger-install-smoke` runs. |
| AC-006 (this story's share) | `project.md`:246-250, `_storymap.md`:129 | "*the sentence a consumer actually reads*", vs. `msrv-promise-atom`'s "*the atom of record*". |
| AC-007 (this story's share) | `project.md`:251-253, `_storymap.md`:130 | "*the docs.rs configuration*". |
| AC-008 (this story's share) | `project.md`:254-257, `_storymap.md`:131 | "*the snippet being the same text the page shows*", vs. `stranger-install-smoke`'s "*the run*". |
| AC-014 (this story's share) | `project.md`:277-280, `_storymap.md`:137 | "*stated so a `wasm32` reader can self-identify*", vs. `publish-0-2-0`'s "*asserted by the four `wasm32` steps*". |
| `dependsOn` | manifest, `_storymap.md`:169 | `msrv-promise-atom` (the atom to link), `projection-port-ship-shape` (whether a `doc(cfg)`-gated port needs a Guarantees line). |
| The missing manifest block, verified | `_design.md`:68-73, 96-102, 510 (Shape decision table) | `[package.metadata.docs.rs]` with `all-features = true` and `rustdoc-args = ["--cfg", "docsrs"]` is present at `crates/happenstance-core/Cargo.toml:54-56` and `crates/happenstance-testkit/Cargo.toml:56-58`, **verified absent** from `crates/happenstance/Cargo.toml` — "the crate a `cargo add` evaluator meets is the one whose docs.rs build is unconfigured." |
| RS-51-5, the mechanism | `standards/rust/51-features-and-no-std.md`:188-236 | `#![cfg_attr(docsrs, feature(doc_cfg))]` plus `#[cfg_attr(docsrs, doc(cfg(feature = "…")))]` on every gated item, and the manifest block above. |
| The doctest identity, exact mechanism | `_design.md`:511, 660-695 (`## The doctest`) | The fence in `crates/happenstance/README.md`, the doctest `crates/happenstance/src/lib.rs`:10 compiles from it via `#![cfg_attr(doctest, doc = include_str!("../README.md"))]`, and `stranger-install-smoke` runs the same text — "the same text three times over." 19 source lines, inside the 20-line budget. |
| Existing Guarantees slot | `crates/happenstance/README.md`:41-49 | Already carries `forbid(unsafe_code)`, feature forwarding, MSRV-with-ADR-link (currently pointing at ADR-0029 only) — this story extends it to ≤7 bullets total (`_design.md`:449), adding DT-4's promise (owned by the sibling story), the new MSRV atom's link, minor-bump-is-breaking, and the `wasm32` self-identification line. |
| `wasm32` self-identification precedent | `README.md`:159-164 (existing text, cited at `_grounding.md`:127-135) | Must be carried to the packaged surface and be true of the *published* tree, not re-invented. |
| Anti-patterns bearing directly | `_design.md`:639, 647-648 (AP-8, AP-12) | AP-8: a docs.rs page where a gated item appears without its `doc(cfg)` annotation, or is absent entirely. AP-12: a quick-start fence that scrolls inside itself at 1440×900 (>20 source lines), *or that differs by one character from the text the stranger-install smoke ran*. |
| Semver promise table | `_design.md`:545-551 | The stability-promise object itself: MSRV floor as a promise, what a bump means, minor-bump-is-breaking under 0.x — each linking the new atom, none editing ADR-0004/0029. |

## Questions

- **Does adding the docs.rs manifest block to `happenstance` risk exposing anything currently hidden by its absence?** No brief flags a risk here, and `_design.md`:510 treats the addition as unconditionally correct ("without `--cfg docsrs` the `doc(cfg)` attributes are inert... without `all-features` they do not render at all" — i.e. the absence is a bug, not a deliberate omission). Answered: add it as specified, byte-identical in shape to the two manifests that already have it.
- **Is the doctest wiring (`#![cfg_attr(doctest, doc = include_str!("../README.md"))]`) already present at `crates/happenstance/src/lib.rs:10`, or does this story add it?** `_design.md`:169-175 (UX brief, Package-surface primitives) and `_design.md`:511 both cite it as an *existing* line, not new. Deferred to spec to confirm the current file state before treating it as "already done" — a quick grep at spec time settles it; this story's real work may be narrower than it first reads (checking the wiring already threads the exact README text now landing there, rather than adding the wiring itself).

## Decision

`crates/happenstance/Cargo.toml` gains the `[package.metadata.docs.rs]` block already present on the other two publishable crates, with `doc(cfg)` annotations on every feature-gated public item per RS-51-5. The Guarantees block gains the MSRV-as-promise line (linking the new atom `msrv-promise-atom` authors), the minor-bump-is-breaking statement, and the `wasm32` self-identification line, staying within the ≤7-bullet budget. The quick-start fence is confirmed to be — or made to be — the crate's own doctest via `crates/happenstance/src/lib.rs`:10's existing `include_str!` mechanism, so it is byte-identical to the text `stranger-install-smoke` runs. Spec confirms the current state of the doctest wiring before treating any part of it as pre-existing.

## The wrong implementation

A Guarantees-block edit and a docs.rs manifest addition that both look complete under a page-level read — the block has ≤7 bullets, each links something, `all-features = true` is present, `doc(cfg)` pills appear on the gated port — but where the quick-start fence shown in `crates/happenstance/README.md` has drifted by even one character from what `crates/happenstance/src/lib.rs`'s doctest actually compiles, or from what `stranger-install-smoke`'s scratch project actually runs (a trailing newline, a renamed variable, an added `#` in the doctest source that the README fence doesn't show). This is AP-12's second clause exactly, and it is a realistic failure mode because the doctest wiring and the stranger-install script are edited by different stories at different times — nothing mechanically forces them to stay identical except the `include_str!` path itself, and a hand-copied "equivalent" snippet in the smoke script would pass every other check while breaking AC-UX-012's actual promise ("a reader who copies what they see gets what the smoke proved"). What catches it: the smoke script must import the same fence via the same `include_str!`-rooted mechanism (or an automated diff against it) rather than embedding its own copy of the example.

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
