---
item: HS-S0095
stage: implement
created: 2026-08-12
updated: 2026-08-12
---

# Acceptance ledger — The rendered pages are read before the irreversible act, and the read is dated

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Note on the instruments below: this story's primary tier is **human observation of a rendered page**,
which is what the testing brief assigns to project AC-007 — *human observation (rendered page) +
static (containment only)* (`.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md`:470),
with its Notes forbidding an automated stand-in in terms (`:548-553`) because
`xtask/src/package.rs`:4-18 states that `cargo package --list` proves containment and never
presentation. The `verifying_test` field therefore names a real command, a real artefact path, or the
content review that is the honest tier for that row. Inventing a compiled test for how a page reads
would be the decorative gate `CLAUDE.md` forbids — and it is precisely the wrong implementation
`discover.md` names for this story.

```yaml
- id: AC-001
  criterion: "The evaluator's page is read from the bytes that will ship, not from the tree that produced them. GIVEN crates.io renders `README.md` from inside the `.crate` (`crates/happenstance/Cargo.toml`:12) and `include_str!` cannot reach outside a package, so the repo-root README is a different document with different links (`README.md`:131-137), WHEN the preflight runs, THEN for each of the three crates `cargo package -p <crate> --allow-dirty --locked` has produced an artifact at this commit, every crates.io verdict in the record was taken from the `README.md` inside that artifact, and the record names the artifact and its version; and no verdict anywhere in the record cites `crates/<name>/README.md` as it stands in the working tree"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md, section Phase 12 — Publish `0.2.0`: the Work checklist (:4466-4479) and the Exit criteria (:4489-4505)"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/rendered-page-preflight/_preflight.md names three artifacts under target/package/ with versions; cargo xtask ci package-check step (xtask/src/package.rs:86, :88-94) for the containment half; content review of every crates.io row for its source file"

- id: AC-002
  criterion: "The docs.rs page is read on the real renderer, including the crate the gate has never built there. GIVEN `xtask/src/main.rs`:621-636 puts the `--cfg docsrs` build behind a toolchain probe and names only `-p happenstance-core -p happenstance-testkit` (`:627-630`), and `--fast` — this project's own integration gate (`.redkiln/config.yaml`:50-56) — drops the step entirely, WHEN the preflight runs, THEN `cargo +nightly doc --locked -p <crate> --all-features --no-deps` with `RUSTDOCFLAGS=\"--cfg docsrs -D warnings\"` has been run on this machine, at this commit, for all three crates including `happenstance`, the record states the nightly version and that the build ran rather than that the gate was green, and every docs.rs verdict was read from `target/doc/<crate>/index.html`"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md, section Phase 12 — Publish `0.2.0`: the Work checklist (:4466-4479) and the Exit criteria (:4489-4505)"
  verifying_test: "cargo +nightly doc --locked -p happenstance -p happenstance-core -p happenstance-testkit --all-features --no-deps under RUSTDOCFLAGS; _preflight.md carries the nightly toolchain string and a per-crate ran/failed line; content review against xtask/src/main.rs:602-636, :82-88 and standards/rust/70-rustdoc-obligations.md:224-234"

- id: AC-003
  criterion: "All seven surfaces are read, at both viewports, in the states this medium actually produces. GIVEN `_design.md`:104-169 declares seven surfaces (crates.io x3, docs.rs x3, GitHub landing x1) and narrow is the design case rather than the degraded one (`:617-630`), WHEN the read is performed, THEN each of the seven has a verdict at 1440x900 and 1024x768; crates.io and GitHub surfaces are additionally read with images blocked; docs.rs surfaces are read in light and dark using rustdoc's own theme switcher; `long-label` is read on `github-landing`; and `docs-build-failed` is recorded as not reached rather than left blank — a state that did not occur is stated to have not occurred"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md, section Phase 12 — Publish `0.2.0`: the Work checklist (:4466-4479) and the Exit criteria (:4489-4505)"
  verifying_test: "the surface x viewport grid in _preflight.md read for 14 populated cells plus the named state rows; content review against .bklg/from-contract-to-published-library/publication-and-positioning/_design.md:104-169, :617-630, :724-732"

- id: AC-004
  criterion: "The accessibility floor is answered item by item, and a skip is reported rather than silent. GIVEN the evaluator's decision must be reachable from text alone and the floor is eight checkable items (`_decomposition.md`:201-224; AC-UX-006 at `:340-345`), WHEN the record is written, THEN each of — one `#` H1 per document; no skipped heading levels; every table has a header row; every fence declares a language; every link's text is meaningful standing alone; every image has alt text; no raw HTML, inline `style`, JavaScript, custom colour or animated media; no meaning carried by colour or glyph alone — carries `pass` / `fail` / `n/a` per surface, and every `n/a` carries a reason in the same cell; a blank cell is a defect in the record, on the standing rule that a declined check still reports its stated reason (`.kb/decisions/0010-the-suite-must-prove-itself.md`)"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md, section Phase 12 — Publish `0.2.0`: the Work checklist (:4466-4479) and the Exit criteria (:4489-4505)"
  verifying_test: "the 8 x 7 floor grid in _preflight.md read for blanks and for reasonless n/a; the images-blocked pass required by AC-003 is what makes the colour/glyph and alt-text rows falsifiable; content review against .bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md:201-224, :340-345"

- id: AC-005
  criterion: "Every named anti-pattern gets a per-surface verdict from someone who read only the page. GIVEN all fifteen are phrased so they can be checked against a screenshot of the rendered page by someone who cannot read the code (`_design.md`:641-679), WHEN the read is performed, THEN AP-1 … AP-15 each carry a per-surface verdict, with AP-1 (who the crate is for, visible without scrolling at 1024x768), AP-11 (no 8-row status table on a packaged page) and AP-14 (no badges above the status callout) called out explicitly as the three the sign-off's density finding puts nearest the edge (`:744-752`); and AP-6's link and anchor verdict cites `landing-copy-and-status-truth`'s mechanical check by result rather than re-running or re-implementing it (`_storymap.md`:67)"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md, section Phase 12 — Publish `0.2.0`: the Work checklist (:4466-4479) and the Exit criteria (:4489-4505)"
  verifying_test: "the AP-1..AP-15 x surface grid in _preflight.md; content review that the AP-6 row is a citation of AC-UX-011's result and not a second implementation, and that it names README.md:9 -> #licence and README.md:16 -> #status; cross-read against _design.md:641-679 and .kb/open-questions/es-38-and-gap-read-rules-are-unowned.md for AP-13"

- id: AC-006
  criterion: "Licence, description and README are judged as they read, not as metadata says they are contained. GIVEN `xtask/src/package.rs`:4-18 says in its own module doc that the metadata is what crates.io renders, so the omission is invisible until someone unpacks the tarball, WHEN the read is performed, THEN for each of the three crates the record states: both licence files present and how the licence statement reads on the page; the `description` measured in characters against the <= 120-character budget (`_design.md`:462-465) and re-read for truth against the tree that ships (IQ-7, `_decomposition.md`:305-309); and the README rendering with no broken construct. `crates/happenstance/Cargo.toml`:3 is ~131 characters and claims typed events, decision models and projection runners — the record measures and reports it and names `landing-copy-and-status-truth` as its owner; it is not edited here"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md, section Phase 12 — Publish `0.2.0`: the Work checklist (:4466-4479) and the Exit criteria (:4489-4505)"
  verifying_test: "_preflight.md's licence/description/README block, one row per crate, carrying a character count; cargo xtask ci package-check (xtask/src/package.rs:88-94) for containment; git diff --name-only shows no crates/** path, which is AC-009's assertion applied to this row"

- id: AC-007
  criterion: "The first screen is measured against the budget in pixels, at a stated zoom, on the real render. GIVEN the design is dimensioned against 14 rendered lines ~= 340 px at 1024x768 where crates.io stacks its metadata above the README (`_design.md`:446-458), and the sign-off accepted ~=343 px against 340 px and declared the first screen full at `0.2.0` (`:744-752`, `:777-783`), WHEN the read is performed, THEN the record carries the measured height per first-screen region and the total, in pixels at a stated browser zoom, at both viewports, for `crates-io-happenstance` at minimum; the 24 px-per-line model is used only to explain a number and never to produce one; and if the total exceeds the budget the record names the failure as AP-1 and cites `_design.md`'s demotion order (`:477-489`) as what must yield, without performing the demotion"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md, section Phase 12 — Publish `0.2.0`: the Work checklist (:4466-4479) and the Exit criteria (:4489-4505)"
  verifying_test: "the per-region measurement table in _preflight.md with its zoom level stated; content review against .bklg/from-contract-to-published-library/publication-and-positioning/_design.md:446-458, :477-489, :744-752, :777-783 — a record carrying rendered-line counts instead of pixels fails this row"

- id: AC-008
  criterion: "The reviewer compares the built page against the frame a human signed off, not against a description of it. GIVEN `_design.md`:755-770 declares fourteen exact reference paths and states that capture stays manual at AC-007 because `design.capture` is undeclared and there is no route, WHEN this story lands, THEN a PNG exists at each of `.bklg/from-contract-to-published-library/publication-and-positioning/design/reference/<surface>@1440x900.png` and `@1024x768.png` for all seven surfaces — fourteen files, committed, no external hosting, each clipped to the viewport it names — and each is a capture of the rendered page in a browser at true viewport width with the host's chrome, never a screenshot of Markdown source or a plaintext dump"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/publication-and-positioning/design/reference/ (the fourteen frame paths declared at _design.md:755-764), reached through the same RUNBOOK.md Phase 12 mount"
  verifying_test: "ls .bklg/from-contract-to-published-library/publication-and-positioning/design/reference/ shows exactly the fourteen declared filenames; each frame opened beside its counterpart in .bklg/from-contract-to-published-library/publication-and-positioning/design/mock.html"

- id: AC-009
  criterion: "A finding stops the release; it never gets quietly fixed by the reader who found it. GIVEN DR-15 makes a wrong `[FROZEN]` clause a blocker and a re-plan rather than an edit (`project.md`:217-219) and `_storymap.md`:186-191 extends that to anything a copy owner cannot absorb, WHEN a floor item, an anti-pattern or a budget fails, THEN the record states the finding, the surface, and the owning story by name — copy, stale strings, the description, the census, the status table -> `landing-copy-and-status-truth`; compliance block, positions-and-gaps promise, the `read_from_a_gap_position` non-ownership line -> `compliance-claim-and-gaps-promise`; Guarantees, MSRV promise, `wasm32` line, the docs.rs manifest block, `doc(cfg)`, the quick-start doctest -> `guarantees-and-docs-rs-presentation`; a `[FROZEN]` clause or an API shape -> re-plan — and the PR's own diff contains no path under `crates/`, `spec/` or `xtask/`, which is also what proves no renderer, capture harness or Markdown-diff was invented here (`_decomposition.md`:548-553, `:766-769`)"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md, section Phase 12 — Publish `0.2.0`: the Work checklist (:4466-4479) and the Exit criteria (:4489-4505)"
  verifying_test: "git diff --name-only against the base contains only .bklg/.../rendered-page-preflight/**, .bklg/.../design/reference/** and RUNBOOK.md; redkiln verify --grain story reads the fenced PR boundary; cargo xtask affected --base main maps the diff to no workspace package while running the five file-reading lints and spec-trace unconditionally; content review that every fail row names an owner"

- id: AC-010
  criterion: "The read is a dated artefact that gates the irreversible act, not a remembered observation. GIVEN a `yank` removes a version from the resolver and leaves every rendered page exactly as it was (`standards/rust/51-features-and-no-std.md`:226-231; IQ-4 at `_decomposition.md`:272-287), and DoD item 3 requires committed artefacts with a date, not a remembered observation (`project.md`:295-296), WHEN this story lands, THEN `_preflight.md` exists, committed, carrying the date, the commit SHA read, the toolchain and browser used, and a single `GO` / `BLOCK` line at the top — `BLOCK` naming every unresolved finding and its owner; the record states its own three limits with their reasons (crates.io's Markdown pipeline and sanitizer not reproduced; the crates.io chrome geometry taken from `_design.md`'s dimensioned bands rather than the live site; the live URLs not resolving until after publish); it states that no post-publish re-read is scheduled and why; and `RUNBOOK.md`'s phase 12 carries one Work line above the publish-in-dependency-order item (`:4469-4471`) and one Exit criterion naming this record's path"
  satisfied: false
  evidence: ""
  mount_point: "RUNBOOK.md, section Phase 12 — Publish `0.2.0`: the Work checklist (:4466-4479) and the Exit criteria (:4489-4505)"
  verifying_test: "head of _preflight.md read for the GO/BLOCK line and the dated header; git log shows _preflight.md committed before the publish-0-2-0 commit; git diff RUNBOOK.md shows exactly two added lines, one in the Work checklist above :4469-4471 and one in the Exit criteria naming the record's path; redkiln validate --kb && redkiln doctor clean with exactly six template-drift advisories"
```
