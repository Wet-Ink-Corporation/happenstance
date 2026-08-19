---
item: HS-S0095
stage: discover
created: 2026-08-12T13:03:04.041Z
updated: 2026-08-12T13:03:04.041Z
template_sig: 86ce4036
rendered_sig: 3d3ff3e8
---

# Discover — The rendered pages are read before the irreversible act, and the read is dated

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Storymap slice | `_storymap.md`:67 | Read the *rendered* crates.io and docs.rs pages for all three crates against the accessibility floor, plus licence/description/README "as they read rather than as `cargo package --list` says they are contained," committed as a dated check before the irreversible act. |
| AC-007 (this story's share) | `project.md`:251-253, `_storymap.md`:130 | "*the dated read of the rendered page*" — the third of three AC-007 splits, distinct from the copy and the docs.rs configuration. |
| `dependsOn` | manifest, `_storymap.md`:171 | `landing-copy-and-status-truth`, `compliance-claim-and-gaps-promise`, `guarantees-and-docs-rs-presentation` — this story reads what the other three wrote; it cannot start before they land. |
| Why containment cannot substitute | `xtask/src/package.rs`:4-18 | "the metadata is what crates.io renders, so the omission is invisible until someone unpacks the tarball... Running `cargo package --list` and discarding its output would be the decorative-gate failure this repository names in CLAUDE.md." Cited repeatedly across the project as the exact reason this check must be a human read, not an automated stand-in (`project.md`:369-371, `_decomposition.md` UX brief line 549-553, `_design.md`:41-45). |
| Ordering constraint, protected explicitly | `_storymap.md`:183-184 | "`rendered-page-preflight` strictly precedes `publish-0-2-0`: IQ-4 puts the whole of this project's reversibility before the act." |
| IQ-4, reversibility bought before the act | `_decomposition.md`, UX brief (272-287) | Publication is the one irreversible act; a yank removes a version from the resolver but "leaves the rendered page exactly as it is." The rendered pages must be read *before* publish, not after. |
| The accessibility floor, itemised | `_decomposition.md`, UX brief "Accessibility floor" (201-224) | Colour/glyph never alone; no custom colour; no reduced-motion violations; one H1, no skipped heading levels, header row on every table, every fence languaged, meaningful link text, alt text on every image. |
| The mock and its reference captures | `_design.md`:697-749 | 51-frame static mock at `.bklg/.../design/mock.html`, built because `design.capture` is undeclared and there is no app to screenshot. Reference PNGs per surface per viewport at `.bklg/.../design/reference/` are what this story's actual reads compare against — "the review compares the *published* page against the frame that was signed off." |
| Density finding carried forward from sign-off | `_design.md`:723-731, 756-762 | The 14-line first screen is accepted as **full at 0.2.0** — measured at ≈343px against a 340px budget after tightening the callout to two lines and each triad bullet to one. Any sixth region or regrowth fails AP-1. This preflight is where that budget gets checked against the *actual* rendered page, not the mock. |
| Anti-patterns checked here specifically | `_design.md`:625-654 | All 15 (AP-1 through AP-15) are checkable "against a screenshot of the rendered page by someone who cannot read the code" — this story is that someone, at the real URLs. |
| Seven surfaces, not three | `_design.md`:106-169 (`## Surfaces`) | crates.io ×3, docs.rs ×3, GitHub landing ×1 — all seven, not only the packaged READMEs, per the mock's own "All **seven** surfaces" note (`_design.md`:701). |
| AC-UX-006/007 | `_decomposition.md`, UX brief (340-350) | Accessibility floor holds on every rendered page for all three crates, recorded as a dated check; all three crates carry the docs.rs manifest block and build green under all features. |

## Questions

- **The URLs don't resolve yet — how is this a "rendered page read" before publish?** Resolved by the design's own answer: docs.rs and crates.io pages can be previewed pre-publish via `cargo doc --all-features` locally (for docs.rs's rendering, modulo theme) and a local Markdown render of each crate README against github-markdown-css / crates.io's approximate geometry for the crates.io pages — this is exactly why `_design.md`'s mock exists, dimensioned to the real chrome bands (`_design.md`:719-721). Not fully mechanical; deferred to spec to pin the exact preview tooling, but the direction is answered: a faithful local render stands in for the not-yet-live registry page, checked again for real once `publish-0-2-0` actually happens (this is why `stranger-install-smoke` and this story are different instruments at different times).
- **Does this story re-run after `publish-0-2-0`, or only before?** `_storymap.md`'s merge order (171-173) places `rendered-page-preflight` strictly before `publish-0-2-0` — "preflight" in the name is deliberate. Whether a second, post-publish confirmation read is also warranted is not stated by any brief. Deferred to spec: the DoD's own item 3 ("committed artefacts with a date, not a remembered observation," `project.md`:295-296) suggests the dated preflight record is what's required, and a second post-publish spot-check is good practice but not a named AC.

## Decision

Before `publish-0-2-0`, a human reads all seven rendered surfaces — crates.io ×3, docs.rs ×3, GitHub landing ×1 — against the accessibility floor and all 15 named anti-patterns, using faithful local renders where the registry URLs do not yet resolve, and records the read as a dated, committed artefact. `cargo package --list` is run too, but only for containment; it is explicitly not a substitute for the read, because `xtask/src/package.rs:4-18` names exactly the omission class (a licence promised by metadata but absent from the artefact) that only unpacking or rendering the real page reveals. Spec pins the concrete local-preview tooling for the pre-publish read.

## The wrong implementation

A "rendered-page preflight" that runs `cargo package -p happenstance --list` (and the same for the other two crates), confirms `LICENSE-MIT`, `LICENSE-APACHE` and `README.md` are present in the listing, and records that as the dated AC-007 check — satisfying a naive "was a check run and recorded" audit, and even correctly using the tool this project's own `package-check` step already runs. It is still wrong, because `xtask/src/package.rs`'s own module doc states plainly what this proves and does not prove: containment, never presentation. A licence file can be present in the artefact and still render as broken Markdown, a table with no header row, a heading skip, or a badge that is the only carrier of some fact — none of which `cargo package --list` can see, because it lists filenames, not rendered output. This is precisely the prompt's own named example of a wrong implementation for this project: "a registry page whose completeness was asserted from `cargo package --list` rather than by looking at the rendered page." What catches it: the check must be a human read of the actual rendered page (or a faithful local render pre-publish) against the itemised accessibility floor and the 15 anti-patterns, with the read's date and findings recorded as the artefact — not a `cargo package --list` transcript relabelled as if it answered a different question.

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
