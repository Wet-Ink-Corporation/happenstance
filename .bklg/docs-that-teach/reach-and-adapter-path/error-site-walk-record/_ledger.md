---
item: HS-S0157
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — The observed error-site walk

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Note on evidence in this story.** Nothing in `cargo xtask ci` can observe a walk
(`_design.md`, `## Density budget`, gap 2). Evidence here is therefore a `file:line` into
`walk-record.md` plus, where one exists, the re-runnable command named in `verifying_test` —
never a green gate, which proves only that this story disturbed nothing.

```yaml
- id: AC-001
  criterion: "GIVEN Persona 2 has just hit the two-flavour collision and searches the string rustc printed, WHEN the walker reproduces it themselves at the pinned toolchain rather than quoting the rewrite back at itself, THEN walk-record.md carries a reproduction stanza with the verbatim scratch source, the exact cargo invocation, `rustc -V` as printed, and the whole `error[E0034]` stderr including both `= note:` candidate lines and `TraitVariantBlanketType` — and no file under `crates/` is added by this PR."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md"
  verifying_test: "Re-run of the reproduction stanza in .bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md at rust-toolchain.toml's channel 1.97.1, plus `rg -F 'TraitVariantBlanketType' .bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md`, `rg -cF '= note:'` on the same file, and `git diff --name-only main...HEAD -- crates/` returning empty."

- id: AC-002
  criterion: "GIVEN the project may not stand in for HS-P0024's non-insider instrument, WHEN the record makes its claim, THEN it names the walker, states in one sentence that they authored neither `store-error-site-rewrite` nor `adapter-reasoning-account` and what they knew of the surface beforehand, and carries an explicit bounded-claim line saying this is evidence that the path exists and not that a stranger finds it — with no sentence anywhere in the record implying otherwise."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md"
  verifying_test: "Authorship cross-check `git log --format='%an %ae' -- crates/happenstance-core/src/store.rs` (and the reasoning account's page path) must not list the named walker, plus a human read of .bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md against project.md's risk-table row on non-authors versus non-insiders."

- id: AC-003
  criterion: "GIVEN initiative DoD scenario 10 fixes the starting conditions at the file and the message and nothing else, WHEN the walk begins, THEN the record states the cold-start protocol before its first hop: what the walker had open at t=0 (their own failing build output and `crates/happenstance-core/src/store.rs`) and what was forbidden (the backlog, this spec, the sibling stories, the author, and prior knowledge of where the account was filed), with any contamination disclosed rather than omitted."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md"
  verifying_test: "Heading-order check on .bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md — protocol before reproduction before Observation A before the hop table — read against .bklg/docs-that-teach/initiative.md DoD scenario 10 and this story's discover.md wrong-implementation line."

- id: AC-004
  criterion: "GIVEN Persona 2 wants the reasoning behind the two-flavour split and not only the fix, WHEN they take the offer at the end of `## Import one flavour, not both`, THEN the record's hop table reaches the adapter reasoning account in exactly one hop and proves arrival at the passage: it names the account's stated answered-need, the position marker of the section landed on (`n of 6`), and whether the caveat that `MemoryEventStore` is the conformance oracle and reference implementation and not an adapter was present where `memory.rs` is first sequenced — two hops, or an arrival that cannot be located in the sequence, is recorded as a finding."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md"
  verifying_test: "Hop-row count in .bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md (exactly one row sourced at store.rs), plus `rg -F` of the quoted answered-need and position marker against the reasoning account's page in HS-P0020's pinned tree, compared to _design.md's `## Composition` regions 1, 3 and 4 for adapter-reasoning-account."

- id: AC-005
  criterion: "GIVEN a keyboard-only reader who searches before they read, WHEN each hop is taken, THEN the record states that no pointing device was used and names the affordance and the keys per hop (plain link, rustdoc search `S` or `/`, a `#[doc(alias)]` key, or a fragment), and at least one hop or recorded attempt uses a string the reader would really type — `E0034` or `TraitVariantBlanketType` — with what it returned written down."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md"
  verifying_test: "Column completeness check on the hop table in .bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md (no empty affordance cell; at least one recorded search string and its result), read against the accessibility floor in .bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md and its UX-AC-09."

- id: AC-006
  criterion: "GIVEN invariant 1, that getting unstuck must never require leaving the surface the reader is on, WHEN the walker reaches `store.rs` and before any hop is taken, THEN the record answers first whether `store.rs` alone named which call was ambiguous and how to resolve it, quoting the sentence that did it verbatim from the file (or recording that none did) — and that observation appears in document order ahead of the hop table."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md"
  verifying_test: "`rg -F` of the record's quoted sentence against crates/happenstance-core/src/store.rs returning a hit on the merged tree, plus the heading-order check that Observation A precedes the hop table in .bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md."

- id: AC-007
  criterion: "GIVEN that a walk which was not written down is indistinguishable from one that did not happen, WHEN the PR merges, THEN walk-record.md exists in this story's folder carrying the ISO date of the walk itself, and project.md's `## Companions` links it with self-describing link text naming the record and the walk it carries, so project Definition-of-done item 4 resolves against a file — with nothing else in project.md changed."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md"
  verifying_test: "`test -f .bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md`; `rg -n 'error-site-walk-record/walk-record.md' .bklg/docs-that-teach/reach-and-adapter-path/project.md` hitting inside `## Companions`; and `git diff main...HEAD -- .bklg/docs-that-teach/reach-and-adapter-path/project.md` showing only the added bullet and no frontmatter line."

- id: AC-008
  criterion: "GIVEN invariant 5 and _design.md's Refused state, WHEN a hop fails, resolves nowhere, or takes two, THEN that entry is closed in the record as a failed walk with the finding routed by owning story slug (`store-error-site-rewrite` for the pointer, `adapter-reasoning-account` for the destination) and nothing repaired in this PR, and any later attempt is a new dated entry citing the failed one — the file's history shows entries appended, never a closed entry edited."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md"
  verifying_test: "The declared failure protocol in the record's protocol stanza, plus `git log -p -- .bklg/docs-that-teach/reach-and-adapter-path/error-site-walk-record/walk-record.md` showing additions only below the last closed entry, and `git diff --name-only main...HEAD` showing no file of a routed-to story touched in this PR."

- id: AC-009
  criterion: "GIVEN the standing composition constraints every story in this project inherits, WHEN the record and its mount are rendered by GitHub's Markdown renderer, THEN the record is composed prose plus one hop table with a header row and its named columns, and neither it nor the project.md link introduces raw HTML, an inline `style=` attribute, a folded or tabbed element, a See also / Next steps / Further reading block, a fewer-than-three-row table used as a navigation device, link text reading here / this / docs, or a bare URL into this repository's own tree."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/reach-and-adapter-path/project.md"
  verifying_test: "`rg -n '<details|<table|<div|style='` and `rg -in '^#+ *(see also|next steps|further reading)'` over walk-record.md and project.md returning nothing, plus a rendered read of both against _design.md's `## Anti-patterns` items 3, 5, 6, 9, 10 and its `## Density budget` link-text minimum."
```
