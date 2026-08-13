---
item: HS-S0094
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — What depending costs, stated where a consumer reads it, on a page where nothing is invisible

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, both from `spec.md`:

- **AC-004 is counted once, at the end of the slice**, after `compliance-claim-and-gaps-promise` has
  landed its DT-4 bullet. The 7-bullet budget is a property of the finished list, not of this diff
  alone.
- **A skipped step is not a pass.** AC-005, AC-006 and AC-010 lean on the nightly `--cfg docsrs`
  build, which is *optional* in the gate and dropped entirely by `--fast` (spec EC-002). Its evidence
  must be the committed output of a run, never "the gate was green".

```yaml
- id: AC-001
  criterion: "GIVEN an evaluator reading https://crates.io/crates/happenstance with no checkout and no way to run our suite, WHEN they scroll to Guarantees to answer \"what does depending on you cost me, and when can that cost change?\" (U6), THEN one bullet states 1.97.1 as a promise this release makes — not a measurement someone took — says what an MSRV bump will cost them, and carries exactly one absolute link, to the new decision atom msrv-promise-atom authored (path taken from that story's report, never a guessed number); ADR-0029 is no longer the link, because it records the trade rather than the promise"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (Guarantees block, :41-49) — the render path of crates-io-happenstance R9, packaged by readme = \"README.md\" (crates/happenstance/Cargo.toml:12)"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/guarantees-and-docs-rs-presentation/_checks.md (T4: one MSRV bullet; href equals the atom path in msrv-promise-atom's report; neither .kb/decisions/0004-edition-and-msrv.md nor 0029-msrv-raised-to-1-97-1.md is the target) + cargo xtask affected --base main"

- id: AC-002
  criterion: "GIVEN the same reader, now deciding whether a future upgrade can break them, WHEN they read the same Guarantees block, THEN they learn without leaving the page that under 0.x the minor bump is the breaking-change boundary — the semver contract for the whole release, which today exists only in backlog prose (project.md, Out of scope) and in _decomposition.md's deployment brief (AC-DEP-006)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (Guarantees block, :41-49)"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/guarantees-and-docs-rs-presentation/_checks.md (T4: the 0.x minor-bump sentence present once, stated as a consequence for the caller rather than a policy citation)"

- id: AC-003
  criterion: "GIVEN a constrained-runtime developer who runs on wasm32 and is checking whether that is a first-class target or a footnote (U5), WHEN they scan the packaged page — the only surface they have — THEN one bullet tells them they are supported and which feature set to take, without reading source and without a section, callout or lead being spent on it (DT-1's demotion), and the sentence claims no more than publish-0-2-0's four mandatory wasm32 steps assert"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (Guarantees block, :41-49)"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/guarantees-and-docs-rs-presentation/_checks.md (T4: wasm32 in exactly one bullet and no new heading; feature set matches crates/happenstance/Cargo.toml:22-26) + cargo xtask wasm (xtask/src/main.rs:652)"

- id: AC-004
  criterion: "GIVEN an evaluator scanning Guarantees in a few seconds, WHEN the block has absorbed this story's two bullets and the slice-mate's DT-4 bullet, THEN it is still scannable: <= 7 bullets, each <= 3 rendered lines, exactly one link per bullet, every link absolute, a flat list with no nested bullet except the DT-4 non-ownership line (which is not this story's), and the MSRV bullet rewritten in place so no two sentences on the page can disagree about the floor"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (Guarantees block, :41-49) — counted at the end of the published-surface-copy slice"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/guarantees-and-docs-rs-presentation/_checks.md (T4: bullet count, rendered lines per bullet, links per bullet, every href starts https://) measured against .bklg/from-contract-to-published-library/publication-and-positioning/_design.md '## Density budget'"

- id: AC-005
  criterion: "GIVEN an evaluator who cargo adds the crate and clicks through to docs.rs, WHEN docs.rs builds the crate they actually installed, THEN it builds it the way the other two are built: crates/happenstance/Cargo.toml carries [package.metadata.docs.rs] with all-features = true and rustdoc-args = [\"--cfg\", \"docsrs\"], byte-identical to crates/happenstance-core/Cargo.toml:54-56 and crates/happenstance-testkit/Cargo.toml:56-58 — closing the gap where the one crate most people install was the one whose docs build was unconfigured"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/Cargo.toml ([package.metadata.docs.rs]) — the co-mount that decides what docs-rs-happenstance renders"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/guarantees-and-docs-rs-presentation/_checks.md (T4: byte equality of the three manifest blocks including key order) + T3: cargo +nightly doc --locked -p happenstance-core -p happenstance -p happenstance-testkit --all-features --no-deps with RUSTDOCFLAGS=\"--cfg docsrs -D warnings\""

- id: AC-006
  criterion: "GIVEN a reader deciding whether a capability exists, WHEN they look at the docs.rs page for any of the three published crates, THEN no feature-gated public item is invisible: every gated item renders with its doc(cfg) pill under --all-features, never absent and never bare — because an item that vanishes reads as \"not supported\", which is a different and false claim (IQ-2, AP-8). AND -p happenstance has joined the nightly --cfg docsrs -D warnings step at xtask/src/main.rs:621-635, per that step's own stated joining rule, with the step run and its output recorded here rather than left to a gate that may skip it"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:621-635 (the nightly --cfg docsrs step) and crates/happenstance/src/lib.rs (the docs-rs-happenstance surface)"
  verifying_test: "T3: cargo +nightly doc --locked -p happenstance-core -p happenstance -p happenstance-testkit --all-features --no-deps with RUSTDOCFLAGS=\"--cfg docsrs -D warnings\", output committed; plus T2: cargo doc --workspace --all-features --no-deps --locked; plus the recorded read of target/doc/happenstance/index.html and target/doc/happenstance_core/index.html in _checks.md"

- id: AC-007
  criterion: "GIVEN an evaluator who has decided the crate is real and now wants to try it (U7), WHEN they copy the quick-start fence off the page, THEN they copy a write-then-read cycle that compiles — _design.md '## The doctest''s text, fenced rust (never rust,ignore), <= 20 source lines so it does not scroll inside itself at 1440x900 (AP-12) — and it is the crate's own doctest, compiled by the existing #![cfg_attr(doctest, doc = include_str!(\"../README.md\"))] at crates/happenstance/src/lib.rs:10, replacing today's nine-line read-only count_everything that never writes an event"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (quick-start fence, :30-39) compiled through crates/happenstance/src/lib.rs:10"
  verifying_test: "cargo test -p happenstance --doc + .bklg/from-contract-to-published-library/publication-and-positioning/guarantees-and-docs-rs-presentation/_checks.md (T4: fence diffed against _design.md '## The doctest', source lines counted, info string is exactly `rust`)"

- id: AC-008
  criterion: "GIVEN a stranger with an empty cargo new project and no access to this workspace, WHEN they paste the toml fence and then the rust fence, THEN it resolves and runs: the manifest fence names the published crate and the default feature set (default = [\"std\", \"memory\"] — what MemoryEventStore and collect need, crates/happenstance/Cargo.toml:22-26), with no path dependency and nothing that only works under workspace feature unification, and it is the same text handed to stranger-install-smoke (AC-UX-012 — one text in three places)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (the toml fence accompanying the quick start, R7)"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/guarantees-and-docs-rs-presentation/_checks.md (T4: no path =, no git =, no [patch]; version requirement inside 0.2.x; rust fence byte-compared) — re-proved end-to-end later by the stranger-install-smoke story"

- id: AC-009
  criterion: "GIVEN a maintainer auditing how the MSRV became a promise, WHEN they diff this story's changes, THEN no accepted decision atom was edited to get there: .kb/decisions/0004-edition-and-msrv.md and .kb/decisions/0029-msrv-raised-to-1-97-1.md are byte-identical to the tree this story received (project DR-6; .kb/maps/decision-map.md:30-34), and the backlog and knowledge base are clean at the checkpoint"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/ (verified-unchanged) — asserted over this story's diff at crates/happenstance/README.md's mount"
  verifying_test: "git diff --exit-code <merge-base> -- .kb/decisions/0004-edition-and-msrv.md .kb/decisions/0029-msrv-raised-to-1-97-1.md (recorded in _checks.md) + redkiln validate --kb && redkiln doctor (exactly six template-drift advisories, no dependency-cycle)"

- id: AC-010
  criterion: "GIVEN an evaluator who landed on docs.rs rather than crates.io, WHEN the page renders, THEN the sidebar is a section list rather than a single entry and the ladder answers their questions in their order: one-line summary (<= 2 rendered lines) -> # Status -> # Using it today (runnable) -> the rest, with the first # heading inside 12 rendered lines (_design.md '## Composition', docs-rs-*); the Status paragraph is true of the tree that ships; #![doc(html_no_source)] (crates/happenstance/src/lib.rs:73) is left alone"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (the //! module doc — the docs-rs-happenstance surface's source of record)"
  verifying_test: "T2: cargo doc --workspace --all-features --no-deps --locked + T3 nightly --cfg docsrs build + .bklg/from-contract-to-published-library/publication-and-positioning/guarantees-and-docs-rs-presentation/_checks.md (T4: recorded read of target/doc/happenstance/index.html showing >= 3 sidebar sections; rendered-line count to the first # heading)"

- id: AC-011
  criterion: "GIVEN the reader whose whole first screen is 14 rendered lines at 1024x768 and measured full at ~343 px against a 340 px budget (_design.md '## Sign-off'), WHEN this story's copy lands, THEN nothing it writes is promoted into the first screen and nothing already there is displaced: R7 (quick start) and R9 (Guarantees) stay revealed on scroll, the region order R6 -> R7 -> R8 -> R9 is preserved, no sixth first-screen region appears, and no heading this story touches is renamed — so every inbound anchor a reader may already hold still resolves (IQ-3, AP-1, AP-14, AP-15)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (region order R6-R9 and the heading-slug set of the whole page)"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/guarantees-and-docs-rs-presentation/_checks.md (T4: ordered ## heading + slug capture before and after — the slug set may grow, never shrink; region count above the first ## unchanged), compared at rendered-page-preflight against .bklg/from-contract-to-published-library/publication-and-positioning/design/reference/crates-io-happenstance@1024x768.png"

- id: AC-012
  criterion: "GIVEN a reader on a page whose CSS we do not own, with images blocked, in either theme, WHEN they read the regions this story wrote, THEN those regions carry real composed presentation from this repository's own prose forms and not bare markup: the Guarantees flat bulleted list under its ## (_design.md '## Hierarchy', R9), the claim-with-evidence form for the MSRV promise (README.md:227-234's form), every fence declaring its language (rust, toml), every link carrying meaningful text — never here, this, or a bare URL — and no raw HTML, inline style, image, glyph-alone signal or animated media anywhere in them (AP-7, AP-2, the UX brief's accessibility floor)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (the Guarantees block and the quick-start region) and crates/happenstance-core/README.md:43-54 (the mirrored MSRV bullet)"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/guarantees-and-docs-rs-presentation/_checks.md (T4 over the diffed regions only: zero HTML tags, zero image links, every fence has a non-empty info string, every link text is >= 2 words and not a URL, no nested bullet other than the reserved DT-4 line), re-read with images disabled at rendered-page-preflight"
```
