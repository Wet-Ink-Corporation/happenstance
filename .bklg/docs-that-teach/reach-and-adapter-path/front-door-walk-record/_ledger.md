---
item: HS-S0160
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — The observed front-door walk

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "GIVEN a maintainer reading this record a year later who must decide whether to believe it, WHEN they read `walk-record.md` from the top, THEN before any observation they meet a protocol stanza that states the walk's ISO date, the commit sha of the tree walked, exactly what the walker was permitted at t=0 (the crates.io-equivalent README render and the docs.rs-equivalent crate-root render, and nothing else), what was forbidden by name (the repository tree, `docs/`, `spec/SPECIFICATION.md` opened directly, the backlog, this spec, the sibling stories, and the person who installed the pointer), and the medium substitution — that `cargo add happenstance` resolves to a published crate this branch is not, so `cargo doc -p happenstance --no-deps` stood in for docs.rs and the rendered Markdown of `crates/happenstance/README.md` stood in for crates.io — together with the statement that having the tree on disk to build rustdoc was not permission to read it; and any contamination that occurred is disclosed here rather than omitted."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/walk-record.md — the protocol stanza, ahead of the first observation heading; the file itself is linked from .bklg/docs-that-teach/reach-and-adapter-path/project.md's `## Companions` (body prose only)"
  verifying_test: "Reviewed read of .bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/walk-record.md confirming the protocol stanza precedes the first observation heading; static half `rg -n \"^## \" .bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/walk-record.md` (protocol heading first) and `rg -n \"[0-9]{4}-[0-9]{2}-[0-9]{2}\"` returning the walk date, with the named sha resolving under `git cat-file -e`"
- id: AC-002
  criterion: "GIVEN the sibling project HS-P0024, whose only instrument is the non-insider friction log, WHEN it later reads this record, THEN the record has not spent its claim in advance: it names the walker and states in one sentence that they did not install the pointer (`front-door-pointer`, HS-S0158) and did not author the destination material, says what they knew of the surface before starting, and carries an explicit bounded-claim line — this is evidence that the path exists for a non-author, not that a stranger finds it — with no sentence anywhere in the record implying otherwise; and where initiative DoD scenario 7's stronger condition (\"a person with no prior knowledge of this repository's layout\") is not met by the available walker, the record says so plainly instead of quietly reading DoD 7 down to project AC-004."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/walk-record.md — the walker-identity and bounded-claim stanzas, which are what project.md's DoD item 4 (\"each naming its walker and their relationship to the work\") resolves against"
  verifying_test: "Reviewed read against .bklg/docs-that-teach/reach-and-adapter-path/project.md's risk table (\"AC-005 and AC-004 walkers are non-authors, not non-insiders\"), _storymap.md's standing constraints and .bklg/docs-that-teach/initiative.md BR-14; corroborated by `git log --format='%an %s' -- crates/happenstance/src/lib.rs crates/happenstance/README.md` over front-door-pointer's commits not naming the walker"
- id: AC-003
  criterion: "GIVEN the evaluator arriving at a front door with a twenty-minute budget and no map of this repository, WHEN they look at the first screen at 1024x768 without scrolling and before any hop, THEN the record answers in that order and quotes the sentence as it rendered: was an offer of guide-level material on screen at all; was it inside the first 5 rendered lines of the crate root against a first screen of approximately 27 rendered lines (approximately 32 at 1440x900); on the README did it precede the status blockquote (`crates/happenstance/README.md:6-11`) rather than follow it; was it persistent chrome the reader had to expand nothing to see; and did meeting it cost the reader nothing else — no pre-existing content displaced, folded or pushed below the fold."
  satisfied: false
  evidence: ""
  mount_point: "the two rendered surfaces the walk observes — target/doc/happenstance/index.html at `#main-content > .docblock` (the selector _design.md's `## Surfaces` pins for crate-root-front-door) and the rendered Markdown of crates/happenstance/README.md — recorded in walk-record.md's Observation A stanza"
  verifying_test: "Reviewed read of the Observation A stanza against .bklg/docs-that-teach/reach-and-adapter-path/_design.md `## Density budget`, `## Composition` and `## Transience policy`, plus _decomposition.md UX-AC-01 and invariant 2; render half `cargo doc -p happenstance --no-deps` then target/doc/happenstance/index.html at 1024x768 compared with the crate-root-front-door/first-screen-1024x768 and readme-front-door/first-screen-crates-io frames in .bklg/docs-that-teach/reach-and-adapter-path/design/mock.html"
- id: AC-004
  criterion: "GIVEN that nothing in the repository compares the two front doors — the README-as-doctest mount at `crates/happenstance/src/lib.rs:10` compiles the README's ```rust fences and not its prose, and the mirror assertion is asked of HS-P0020 and does not exist — WHEN the walk is performed, THEN it enters through both entry points, quotes the pointer sentence as rendered on each, and states whether the two are the same authoritative text carrying the pinned substring `Guide-level documentation` exactly once per surface; two different sentences is _design.md anti-pattern 2 and is recorded as a finding routed to `front-door-pointer`, never repaired here."
  satisfied: false
  evidence: ""
  mount_point: "both front-door surfaces as rendered — target/doc/happenstance/index.html at `#main-content > .docblock` and the rendered Markdown of crates/happenstance/README.md — with the comparison recorded in walk-record.md's mirror stanza"
  verifying_test: "Reviewed read of the mirror stanza; corroborating static check run after the walk and reported separately — `rg -c \"Guide-level documentation\" crates/happenstance/src/lib.rs crates/happenstance/README.md` returns 1 for each and the two extracted sentences diff clean with `//! ` stripped; design authority _design.md `## Pattern decision` rule 1, `## Signatures` P1/P2, `## Anti-patterns` item 2"
- id: AC-005
  criterion: "GIVEN the evaluator who has read the offer and decides to follow it inside one bounded reading session, WHEN they hop, THEN the record carries one hop table whose every row names the ordinal, the entry point, the surface, the affordance used and the destination, and the walk terminates at a named passage in the narrative material — not \"the guide\" and not a page top — with that page's own stated answered-need recorded and with what the page assumes was read before it (invariant 4's walk-backability made observable); and the reading budget is recorded as elapsed time from t=0 to arrival and the number of surfaces opened, so a path found only after prolonged determined searching is reported as a finding about reach rather than as a success."
  satisfied: false
  evidence: ""
  mount_point: "walk-record.md's single hop table and arrival stanza, whose terminal row lands on a named passage in HS-P0020's pinned narrative tree (the destination front-door-pointer bound off the href ladder)"
  verifying_test: "Reviewed read of the hop table and arrival stanza against _decomposition.md invariants 3 and 4, UX-AC-04, project.md AC-012 and _design.md `## Composition` (adapter-reasoning-account region 1, the answered-need form); independent confirmation that the named passage exists at the cited path and fragment; budget read against .bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:229"
- id: AC-006
  criterion: "GIVEN a keyboard-only or screen-reader reader, for whom the accessibility floor is not advisory, WHEN the walk is performed, THEN it is performed keyboard-only and the record says so once explicitly and names the affordance and the keys used at each hop from the medium's own set — plain links with `Tab`/`Enter`, rustdoc search (`S` or `/`), rustdoc collapse (`+` / `-`), in-page find on the rendered README, a fragment target — with a statement of where focus landed on each arrival; a record that says \"keyboard-only\" and names no key has asserted the floor rather than observed it and does not satisfy this criterion."
  satisfied: false
  evidence: ""
  mount_point: "walk-record.md's keyboard statement and the affordance column of its hop table, made over the rendered surfaces at 1024x768"
  verifying_test: "Reviewed read against .bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md's Accessibility floor (\"Each of the three observed walks (AC-004, AC-005, AC-009) is performed keyboard-only, and the dated record says so\") and UX-AC-09, confirming each named key is drawn from the enumerated affordance set; no linter exists — _design.md `## Density budget`, \"Two named gaps\" (2) states this rather than papering over it"
- id: AC-007
  criterion: "GIVEN the project's Definition-of-done item 4, which today is a promise with no file behind it, WHEN this PR merges, THEN `walk-record.md` is linked from `.bklg/docs-that-teach/reach-and-adapter-path/project.md`'s `## Companions` with self-describing link text of at least three words naming the record and the walk it carries — never \"here\", never a bare URL into this repository's tree — body prose only, with no YAML frontmatter line touched and nothing else in that file changed, so DoD item 4 resolves against an artefact rather than a memory for one of its three walks."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md — the project item's body under `## Companions`; body prose only, the redkiln CLI owns the frontmatter and a PreToolUse hook denies the edit (CLAUDE.md, \"Where the work lives\")"
  verifying_test: "`git diff -U0 <merge-base> -- .bklg/docs-that-teach/reach-and-adapter-path/project.md` shows exactly one added link inside `## Companions`, zero deletions and no line above the frontmatter's closing `---`; `rg -n \"front-door-walk-record\" .bklg/docs-that-teach/reach-and-adapter-path/project.md` returns the link and `rg -n \"\\[here\\]|https?://\"` over the added line returns nothing; `redkiln validate --kb && redkiln doctor` green"
- id: AC-008
  criterion: "GIVEN that a record which cannot fail is decorative, WHEN the walk hits a dead end — the sentence below the fold, absent from one surface, drifted between the two, a link that resolves nowhere, or a destination that is a page rather than a passage — THEN that failure is the artefact this PR lands: the entry is closed as a failed walk with its stop point named, every finding is routed by story slug (`front-door-pointer` for placement, wording, mirror and ladder rung; `evaluator-onward-links` for the onward hop; `pointer-policy-and-inventory` for a register row with an absent or empty guard), nothing is repaired in this PR, no person is named in a finding, and a later attempt after a fix is a new dated entry citing the failed one rather than an edit to it — closed entries are never edited."
  satisfied: false
  evidence: ""
  mount_point: "walk-record.md's entry structure (one dated entry per attempt, newest last) plus this story's PR-boundary fence, which deliberately carries no `crates/**` glob so that \"nothing repaired\" is checkable rather than promised"
  verifying_test: "Reviewed read against _decomposition.md invariant 5, _design.md `## The states the API must express` (\"Refused\") and `## Anti-patterns` item 18; structural half `git diff --name-only <merge-base>` showing zero files under crates/, spec/, standards/, xtask/, docs/ or .kb/, enforced by `redkiln verify --grain story`; where a second entry exists, `git log --follow` shows the first entry's lines unmodified since the commit that added them"
- id: AC-009
  criterion: "GIVEN a reviewer who cannot run `cargo` and reads only the rendered record, WHEN they open it, THEN it is prose plus exactly one hop table composed from markdown's own primitives — no raw HTML, no inline `style=`, no `<details>`, tab strip or accordion, no \"See also\" / \"Next steps\" / \"Further reading\" block, no fewer-than-three-row table used as a navigation device, an unskipped heading ladder, and self-describing link text throughout — so that nothing in the artefact this story ships trips a _design.md anti-pattern or a _storymap.md standing constraint."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/front-door-walk-record/walk-record.md as rendered by the git host — the artefact this story itself composes, as opposed to the two surfaces it observes"
  verifying_test: "Static over walk-record.md: `rg -n \"<[a-z]+|style=|<details|See also|Next steps|Further reading\"` returns nothing, `rg -n \"^#+ \"` shows no skipped heading level, and exactly one markdown table is present; reviewed against .bklg/docs-that-teach/reach-and-adapter-path/_design.md `## Anti-patterns` items 3, 5, 6, 9, 10 and _storymap.md's standing constraints"
```
