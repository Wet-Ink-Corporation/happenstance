---
item: HS-S0159
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — Somewhere for the second question to go

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, both from `spec.md` and neither a licence to relax a row:

- **The mount point is derived, not confirmed.** `docs/first-encounter.md` is HS-P0022's
  `opening-encounter` page crossed with HS-P0020's `docs/<page>.md` shape. Evidence must cite the
  path **actually registered** in the harness (EC-009), not this derivation.
- **AC-005's observed walks are not this story's evidence.** They belong to
  `second-question-walk-records`. Citing a walk record here would make the author their own witness,
  which project AC-005 forbids.

```yaml
- id: AC-001
  criterion: "GIVEN no evaluator has been observed and DR-4 requires the second questions be named *before* they are answered, WHEN the implementer starts this story, THEN a dated record in this story's folder names exactly two second questions — Q-A (dynamism mistaken for chaos) and Q-B (modelling versus routing) — traces each to its independently observed stall point in research 03, and records why the third stall point is deliberately not walked; and that record exists before the first edit to any page."
  satisfied: false
  evidence: ""
  mount_point: "docs/first-encounter.md — HS-P0022's opening-encounter page in the pinned tree docs/, registered by the narrative harness; the record itself lands at .bklg/docs-that-teach/reach-and-adapter-path/evaluator-onward-links/_named-questions.md"
  verifying_test: ".bklg/docs-that-teach/reach-and-adapter-path/evaluator-onward-links/_named-questions.md present in the PR with each question cited in named-subject-plus-path form against .bklg/docs-that-teach/_discovery/research/03-teaching-the-aggregate-to-boundary-shift-how-dcb-events-disi.md; record commit precedes the page-edit commit"

- id: AC-002
  criterion: "GIVEN an evaluator whose first question was answered well on the origin page and who has formed a sharper one, WHEN they follow the link this story installs, THEN for each of Q-A and Q-B they reach the passage that answers it in exactly one hop, landing on the destination's own heading fragment — never the top of a long page, never an index or a 'start here' detour."
  satisfied: false
  evidence: ""
  mount_point: "docs/first-encounter.md — the registered origin page; destinations are the registered pages HS-P0022 authors, bound to the harness registration at implementation"
  verifying_test: "cargo test -p xtask — the fragment-resolution row test in xtask/src/pointers.rs asserting P4 and P5 carry targets_fragment: true and that each destination file contains the heading its fragment names; plus cargo xtask narrative over the registered tree once HS-P0020's step has landed"

- id: AC-003
  criterion: "GIVEN `_design.md` resolves this surface to an inline link at the end of the answering passage, WHEN a reader — including one navigating by extracted link list or by Tab alone — meets it in the rendered page, THEN it is one sentence-final inline markdown link whose visible text is a noun phrase of at least 3 words naming both the destination *and* the question it answers (never 'here', 'this', 'see this page', 'docs', 'read more', or a raw URL), reachable with the keyboard because it is a plain link and nothing else, and it introduces no heading, list, block, callout, table, fold, raw HTML or inline `style=`."
  satisfied: false
  evidence: ""
  mount_point: "docs/first-encounter.md — the rendered origin page inside the pinned tree docs/"
  verifying_test: "cargo test -p xtask — the validator's link_text rules (at least 3 words, deny list) over the appended rows in xtask/src/pointers.rs; git diff -U0 over the two origin pages showing added lines free of heading markers, list markers, blockquotes, raw HTML and style=; recorded read against _design.md ## Composition, ## Hierarchy and anti-patterns 3, 5, 6, 9, 10"

- id: AC-004
  criterion: "GIVEN a density budget written in real numbers because there will be no perceptual review to catch a bloated one, WHEN the two sentences and two rows land, THEN each added sentence is a single sentence of at least 8 and at most 30 words with no parenthetical and no semicolon, the need it names is at least 4 words, nothing added is folded, tabbed or revealed on hover or focus, the register is still build-time data rendered to no reader, and `POINTER_REGISTER` holds 5 rows against a cap of 8 now and 20 ever."
  satisfied: false
  evidence: ""
  mount_point: "docs/first-encounter.md for the sentences; xtask/src/pointers.rs POINTER_REGISTER for the rows — build-time data, rendered to no reader"
  verifying_test: "cargo test -p xtask — validate(POINTER_REGISTER) is Ok and the cap assertion holds at 5 of 8; rg over the diff for <details>, <summary> and style= returns nothing and no new rendered page appears; word counts over both added sentences recorded in implementation-report.md"

- id: AC-005
  criterion: "GIVEN a reader who never follows either link and only wants their first question answered, WHEN they read the origin passage as it now stands, THEN it still answers that question completely and nothing pre-existing on either page has been reworded, reordered, folded, or moved by more than the added sentence's own height — the link is an offer taken after the answer, never a step required to reach it."
  satisfied: false
  evidence: ""
  mount_point: "docs/first-encounter.md — the origin passage, whose surrounding content is HS-P0022's and unchanged by this story"
  verifying_test: "git diff -U0 main...HEAD over the two origin pages showing additions only with no changed pre-existing line; the revert check recorded in implementation-report.md — removing the two added sentences restores both files byte-for-byte"

- id: AC-006
  criterion: "GIVEN project AC-003 forbids installing any pointer whose only guard is memory, WHEN each of the two links is installed, THEN its row is appended to `POINTER_REGISTER` in the same change, carrying its id (P4, P5), the origin surface, the destination *passage*, an N-3 form from rows 1–3 only (never a bare URL), the href-ladder rung actually used, `targets_fragment: true`, self-describing link text, and a non-empty named guard — a story that installs a pointer and files no row has not finished."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/pointers.rs — POINTER_REGISTER, mounted at xtask/src/lib.rs by pointer-policy-and-inventory"
  verifying_test: "cargo test -p xtask — validate returns Ok over the appended register, plus the row test naming P4 and P5 and asserting each guard string is non-empty and each form is not the bare-URL variant; the deliberately-wrong-register tests landed by pointer-policy-and-inventory still reject their cases"

- id: AC-007
  criterion: "GIVEN the answering passage or its heading may not exist when this story implements, and a pointer into nothing is worse than today's silence, WHEN the implementer binds each destination, THEN the pointer is installed only if that passage exists, answers the named question, and states what it assumes the reader has already read so the hop is walk-backable; otherwise it is not installed — no placeholder href, no 'coming soon' — and the shortfall is recorded as a failed placement and handed to HS-P0022 rather than patched here."
  satisfied: false
  evidence: ""
  mount_point: "docs/first-encounter.md — the origin page; the destination pages are HS-P0022's and are checked, not authored, here"
  verifying_test: "cargo test -p xtask — the fragment-resolution test in xtask/src/pointers.rs fails the build when a row's heading is absent; the resolved file and heading recorded per row in implementation-report.md, and any refusal written up in this story's folder and cited as evidence"

- id: AC-008
  criterion: "GIVEN this story writes prose onto a page HS-P0022 owns and DR-8 forbids any page becoming a second specification, WHEN the linking sentence lands, THEN it opens no second answered-need on that page, makes no normative claim except a citation into `spec/SPECIFICATION.md` that resolves, restates no clause, writes any citation as named subject plus path rather than a bare line range, and this story's `#[doc(alias)]` decision — considered, declined, with its reason — is on record rather than defaulted."
  satisfied: false
  evidence: ""
  mount_point: "docs/first-encounter.md — the sibling-owned page the linking prose lands on, inside HS-P0020's checked surface"
  verifying_test: "cargo xtask spec-trace green (a REQUIRED step inside cargo xtask ci --fast); rg for doc(alias) over the diff returns nothing while the declined decision appears in _named-questions.md; recorded read against HS-P0021's one-need rule, DR-8 and .kb/playbooks/anchoring-citations-in-a-long-lived-document.md"
```
