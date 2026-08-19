---
item: HS-S0092
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The first screen carries the lead claim, and no sentence on it is false

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
  criterion: "GIVEN an evaluator who reached https://crates.io/crates/happenstance from a registry search with one sitting and no way to run the conformance suite, WHEN they read the first screen at 1024×768 and stop there, THEN they can state both what the library is — a contract for storage plus a *published* conformance suite that decides who meets it, stated as an act they can perform rather than as an adjective — and which of the three crates is theirs, because `crates/happenstance/README.md` renders R1 identity, then R2 dated status callout, then the R3 *Which crate do I want?* triad, in that order, with no other region above or between them."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (packaged by `readme = \"README.md\"`, crates/happenstance/Cargo.toml:12)"
  verifying_test: "xtask/src/docs_copy.rs::tests::first_screen_regions_are_in_order, plus a recorded comparison against the crates-io-happenstance@1024x768 frame of .bklg/from-contract-to-published-library/publication-and-positioning/design/mock.html"

- id: AC-002
  criterion: "GIVEN a reader who has just been told that the specification carries a maturity marker per clause, WHEN they look for what those marks mean without leaving the page, THEN one dated, version-scoped paragraph of at most three rendered lines defines frozen, provisional, deferred and demoted **inline**, carries exactly the four counts read from `spec/SPECIFICATION.md`:219-222 at the publish commit (200 IDs, 139 frozen, 49 provisional, 10 deferred, 2 demoted — never 46), carries exactly one link to the ledger, appears exactly once on that surface, and the R11 Design paragraph no longer leaves *maturity marker* as orphan vocabulary."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md and crates/happenstance-core/README.md (census sited after the status callout); below the fold on crates/happenstance-testkit/README.md"
  verifying_test: "xtask/src/docs_copy.rs check (5) census-numbers-equal-§1.3, and xtask/src/docs_copy.rs::tests::census_appears_once_and_defines_its_terms"

- id: AC-003
  criterion: "GIVEN an evaluator who is comparing happenstance against the peers they already found, WHEN they reach the Prior art section on GitHub or the peer sentence on the packaged README, THEN they read prose — one sentence per peer, saying what that peer is good at and then what happenstance's different bet is, never a ranking and never a matrix — carrying an *As of YYYY-MM-DD* date re-read at the publish commit rather than at decision time, in the slot that already exists rather than in a new section, and the packaged surface carries the reduced form of one sentence plus one absolute link."
  satisfied: false
  evidence: ""
  mount_point: "README.md:218-225 (Prior art, full form) and crates/happenstance/README.md R10 (reduced form)"
  verifying_test: "xtask/src/docs_copy.rs::tests::prior_art_is_dated_prose, and xtask/src/docs_copy.rs check (1) no-relative-link-on-a-packaged-surface"

- id: AC-004
  criterion: "GIVEN a reader deciding whether an adapter is usable, WHEN they read the status table on the GitHub landing page, THEN *published and passes the conformance suite*, *published, no suite applies*, *in tree and compiles*, *skeleton*, and *deliberately out of this release train* are each spelled differently, every cell carries a glyph **and** words, each row's state is taken from the owning adapter project's committed report rather than from the fact that the crate builds, and the table appears on **no** packaged surface."
  satisfied: false
  evidence: ""
  mount_point: "README.md:79-98 (the 8-row status table, github-landing surface only)"
  verifying_test: "xtask/src/docs_copy.rs::tests::status_rows_carry_words_and_distinguish_states and ::tests::status_table_is_github_only"

- id: AC-005
  criterion: "GIVEN a reader arriving at the published `0.2.0` page, which can never be edited afterwards, WHEN they read any sentence on any of the four surfaces or the crate card in a search result, THEN no sentence is false of the tree that shipped: none of the six known-false sentences in the Context pack's table survives anywhere, and `crates/happenstance/Cargo.toml`'s `description` is one sentence of at most 120 characters that is true standalone of the published tree and is the identity line in miniature."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md:6-11, crates/happenstance-core/README.md:8-9, crates/happenstance-testkit/README.md:16-17, README.md:11-16/:85-90/:104-109, crates/happenstance/Cargo.toml:3"
  verifying_test: "xtask/src/docs_copy.rs check (4) forbidden-strings, and xtask/src/docs_copy.rs::tests::description_is_within_budget_and_present"

- id: AC-006
  criterion: "GIVEN somebody who already holds a deep link into this documentation — `README.md#status`, `README.md#licence`, or a link that was copied across the packaging boundary — WHEN the copy for `0.2.0` lands, THEN both held anchors still resolve or every inbound reference moved in the same commit, every link on `crates/*/README.md` is an absolute URL, every link on the repo-root `README.md` is repo-relative, every intra-document `#anchor` resolves to a heading in the same file, and no link text is *here*, *this*, or a bare URL."
  satisfied: false
  evidence: ""
  mount_point: "README.md:9 and :16 (the two held anchors) plus every link on crates/happenstance/README.md, crates/happenstance-core/README.md, crates/happenstance-testkit/README.md"
  verifying_test: "xtask/src/docs_copy.rs checks (1), (2) and (3), each with its seeded wrong implementation in xtask/src/docs_copy.rs::tests"

- id: AC-007
  criterion: "GIVEN a maintainer whose change is entirely Markdown and therefore maps to no workspace package, WHEN they run the story gate `cargo xtask affected --base main`, THEN the copy check runs anyway — unconditionally, in the file-reading block, as a `probe: None` step in `REQUIRED` with a dispatch arm and a sentence in the module doc at `xtask/src/main.rs`:8-24 — so that the discipline is mechanical rather than read, and each of its five checks is demonstrated to fail on the wrong implementation it names."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs's REQUIRED list (probe: None) and dispatch arm, plus xtask/src/affected.rs:116-125's unconditional file-reading block"
  verifying_test: "cargo xtask affected --base main on a docs-only diff; cargo test -p xtask over xtask/src/docs_copy.rs's #[cfg(test)] mod tests; cargo xtask ci --fast"

- id: AC-008
  criterion: "GIVEN a reader whose client blocks images, or who is navigating by headings with assistive technology, WHEN they read any of the four surfaces, THEN the page is complete without the badges — every badge's meaning is also stated in text and on the three packaged surfaces the badges sit below the census rather than under the H1 — and the structure is navigable: exactly one `#` H1 per document, no skipped heading levels, a header row on every table, a declared language on every fence, and no raw HTML, inline style, script, custom colour, image carrying meaning or animated media anywhere."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md R5 (badges below the census) and the heading/fence/table structure of all four surfaces"
  verifying_test: "xtask/src/docs_copy.rs::tests::published_surfaces_carry_no_raw_html_or_media and ::tests::heading_structure_is_navigable"

- id: AC-009
  criterion: "GIVEN that the first screen is **full** at `0.2.0` — five regions measuring ≈343 px against a 340 px budget at real Markdown block metrics (`_design.md` `## Mock`) — WHEN any of this story's copy lands, THEN the first screen still holds exactly five regions in the 14-line allocation (identity 3 + blank 1 + callout 3 + blank 1 + triad 6), the identity line is at most 2 rendered lines, the callout at most 2, the triad at most 6 with **exactly three** entries matching the three-crate decision, at most one primary-ranked element (one blockquote) sits above the fold, and if the budget is exceeded the fixed demotion order — badges, then the census, then compressing the identity line — is followed with the callout and the triad never yielding."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md first screen (R1–R3, with R4 census and R5 badges below the fold)"
  verifying_test: "xtask/src/docs_copy.rs::tests::first_screen_stays_within_budget and ::tests::one_primary_element_above_the_fold"
```
