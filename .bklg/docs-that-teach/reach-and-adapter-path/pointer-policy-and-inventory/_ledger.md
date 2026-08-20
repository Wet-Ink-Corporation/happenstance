---
item: HS-S0154
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — The pointer policy, landed as in-tree substrate

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Every `verifying_test` below is a `#[cfg(test)]` test inside `xtask/src/pointers.rs`, reached by
`cargo test -p xtask --lib` and by `cargo test --workspace` inside `cargo xtask ci --fast`. The
mount point is the same for every row because this story delivers one thing mounted in one place:
`pub mod pointers;` in `xtask/src/lib.rs`, the lib crate root — the declaration that puts the
register inside clippy `-D warnings`, the workspace test run and the gate's documentation step, and
that leaves `use xtask::pointers::…` available to HS-P0020's future step.

```yaml
- id: AC-001
  criterion: "GIVEN a later story is about to install a pointer for Persona 2, who arrived at `happenstance_core::store` from a compiler diagnostic and never saw the crate root, WHEN its implementer opens `xtask/src/pointers.rs` and reads no other artefact, THEN they learn DT-10's resolution (option (c), both with one authoritative), rule 1's single authoritative front-door text mirrored byte-identically onto two surfaces, all four gates — (i) evidenced stall, (ii) the front door provably cannot reach it, (iii) subordinate and one line, after the in-place fix and introducing no heading, (iv) a guard from the mechanism table — and the href ladder whose last rung is *do not install, escalate*; and they choose a pointer shape without re-reading `_design.md`."
  satisfied: true
  evidence: "xtask/src/pointers.rs:16-60 — the module doc states DT-10's resolution (both, with one authoritative; :16-22), rule 1's byte-identically mirrored single sentence (:24-30), gates (i)-(iv) verbatim in substance (:32-49) and the href ladder whose last rung is 'the pointer is not installed and the project escalates' (:51-59). Verified by the passing test at xtask/src/pointers.rs:489, tests::policy_states_the_dt10_resolution_its_four_gates_and_the_href_ladder, which pins nine short subject strings against the module doc only — it reads SOURCE above the test module, so the test's own literals cannot satisfy it."
  mount_point: "xtask/src/lib.rs — `pub mod pointers;`"
  verifying_test: "xtask/src/pointers.rs::tests::policy_states_the_dt10_resolution_its_four_gates_and_the_href_ladder"

- id: AC-002
  criterion: "GIVEN that implementer has decided *where* a pointer goes and must now decide its *form*, WHEN they reach for one, THEN `PointerForm` offers exactly the four N-3 rows with each variant documenting the guard it carries; `BareUrl` is documented as forbidden to this project outright rather than merely discouraged; the policy states that the README doctest mount at `crates/happenstance/src/lib.rs:10` compiles the README's Rust fences and **not** its prose, so a prose pointer there is unguarded by it and P2's row may not claim otherwise; and it states that `#[doc(alias)]` is a search key that needs no register row because deleting the item deletes the alias — so a fifth form cannot be improvised and the over-broad guard claim cannot be inherited."
  satisfied: true
  evidence: "xtask/src/pointers.rs:129-163 — PointerForm is a closed four-variant enum, each variant documenting the guard it carries: broken_intra_doc_links (:133), the README doctest mount at crates/happenstance/src/lib.rs:10 (:142), the pinned tree's registration check (:150), and BareUrl guarded by nothing and forbidden to this project outright (:157-162). The README fence-not-prose correction is at :78-91; the #[doc(alias)] search-key rule, and why it needs no register row, at :61-76. Passing tests xtask/src/pointers.rs:509 tests::permitted_forms_are_closed_and_each_documents_its_guard (an exhaustive match with no wildcard arm, plus PointerForm::ALL.len() == 4) and xtask/src/pointers.rs:549 tests::policy_records_the_readme_prose_gap_and_the_alias_rule."
  mount_point: "xtask/src/lib.rs — `pub mod pointers;`"
  verifying_test: "xtask/src/pointers.rs::tests::permitted_forms_are_closed_and_each_documents_its_guard; xtask/src/pointers.rs::tests::policy_records_the_readme_prose_gap_and_the_alias_rule"

- id: AC-003
  criterion: "GIVEN the maintainer performing backbone B1 — deciding once how this surface points outward — WHEN `cargo xtask ci --fast` runs on the merged result, THEN `xtask/src/lib.rs` declares `pub mod pointers;` beside its existing `mod constitution;`, its crate doc no longer opens with a sentence the module has made false, the `#![cfg_attr(doctest, doc = include_str!(...))]` README mount at `xtask/src/lib.rs:21` still stands, `POINTER_REGISTER` is present and empty and `validate(POINTER_REGISTER)` is `Ok`, no `Step` was added to `REQUIRED` in `xtask/src/main.rs`, no file outside the PR boundary changed, and the register is rendered to no reader on any surface."
  satisfied: true
  evidence: "xtask/src/lib.rs:41-47 declares pub mod pointers; beside mod constitution; (:28) and mod narrative; (:36). xtask/src/lib.rs:1 no longer opens 'Nothing but a home for the repository README's doctests.' and the #![cfg_attr(doctest, doc = include_str!(...))] mount still stands at xtask/src/lib.rs:21 — the whole diff to that file is one changed line plus the added declaration. POINTER_REGISTER is present and empty at xtask/src/pointers.rs:238 and validate(POINTER_REGISTER) is Ok. Passing tests xtask/src/pointers.rs:566 tests::the_register_lands_empty_and_valid and xtask/src/pointers.rs:578 tests::the_mount_declares_the_module_and_displaces_nothing. Gates: cargo xtask affected --base main -> 'affected gate passed'; cargo xtask ci --fast -> 'all required checks passed (--fast: 4 optional step(s) not run)'. No Step was added to REQUIRED — xtask/src/main.rs is unmodified — and git status --short is exactly 'M xtask/src/lib.rs' plus '?? xtask/src/pointers.rs', a subset of the PR boundary. The register is rendered to no reader: build-time data only."
  mount_point: "xtask/src/lib.rs — `pub mod pointers;`"
  verifying_test: "xtask/src/pointers.rs::tests::the_register_lands_empty_and_valid (with `cargo xtask ci --fast` and the PR-boundary diff check)"

- id: AC-004
  criterion: "GIVEN a sibling story that has just installed a pointer and is filing its row in the same PR, WHEN it constructs a `Pointer`, THEN the type requires it to state the row id, the surface, the destination, the form, the guard, the visible link text, whether the answer is on the destination's first screen, whether the pointer targets a fragment, and which of gates (i)–(iv) admitted it — and both `Pointer` and `PointerForm` derive `Debug`, so a rejected row prints itself rather than an index."
  satisfied: true
  evidence: "xtask/src/pointers.rs:182-214 — Pointer carries id, surface, destination, form, guard, link_text, answer_on_first_screen, targets_fragment and gates, every field documented under missing_docs. Debug is derived on both Pointer (:181) and PointerForm (:128). Passing test xtask/src/pointers.rs:605 tests::a_row_records_every_fact_a_guard_claim_needs constructs a complete row, reads every field, and asserts the Debug rendering names the row id."
  mount_point: "xtask/src/lib.rs — `pub mod pointers;`"
  verifying_test: "xtask/src/pointers.rs::tests::a_row_records_every_fact_a_guard_claim_needs"

- id: AC-005
  criterion: "GIVEN the project's promise that no pointer is installed whose only guard is memory (`project.md` AC-003), WHEN a row is filed whose `guard` is empty or whitespace, THEN `validate` rejects it and the message names the row id and states why the guard cell exists — so the promise is checkable rather than aspirational, and `_design.md` anti-pattern 18 becomes a build failure."
  satisfied: true
  evidence: "xtask/src/pointers.rs:291-303 — check_guard rejects a guard that is empty or whitespace-only, naming the row id and stating that no pointer is installed whose only guard is memory, and that a mechanism which does not exist yet must say so rather than leave the cell blank. Passing tests xtask/src/pointers.rs:627 tests::rejects_a_row_whose_guard_is_empty and xtask/src/pointers.rs:641 tests::rejects_a_row_whose_guard_is_only_whitespace, both asserting the message names row P3."
  mount_point: "xtask/src/lib.rs — `pub mod pointers;`"
  verifying_test: "xtask/src/pointers.rs::tests::rejects_a_row_whose_guard_is_empty; xtask/src/pointers.rs::tests::rejects_a_row_whose_guard_is_only_whitespace"

- id: AC-006
  criterion: "GIVEN an implementer who cannot find a guarded form for a pointer Persona 2 needs and reaches for a URL, WHEN a row carries `PointerForm::BareUrl`, THEN `validate` rejects it unconditionally and the message states the escalation the href ladder prescribes — the pointer is not installed and the project escalates — rather than offering a workaround or a way to record the absence of a guard."
  satisfied: true
  evidence: "xtask/src/pointers.rs:305-318 — check_form rejects PointerForm::BareUrl unconditionally and states the ladder's escalation ('it is do not install, and escalate') rather than a workaround. Passing test xtask/src/pointers.rs:652 tests::rejects_a_bare_url_outright_and_states_the_escalation, which additionally asserts the message does not offer 'guard: none' as a way to record the absence of a guard."
  mount_point: "xtask/src/lib.rs — `pub mod pointers;`"
  verifying_test: "xtask/src/pointers.rs::tests::rejects_a_bare_url_outright_and_states_the_escalation"

- id: AC-007
  criterion: "GIVEN Persona 3 navigating by an extracted link list or a screen reader inside the one session they will spend on this crate, WHEN a row's `link_text` is fewer than three whitespace-separated words, or is (or begins with) 'here', 'this', 'see this page', 'docs', 'read more', `http://` or `https://` — compared case-insensitively and with trailing punctuation trimmed, so 'See this page.' is the same defect wearing a full stop — THEN `validate` rejects it and names the row, and a three-word noun phrase naming the destination passes."
  satisfied: true
  evidence: "xtask/src/pointers.rs:241-269 (LINK_TEXT_MINIMUM_WORDS = 3 and NON_DESCRIPTIVE_LINK_TEXT) and :320-343 (check_link_text, lower-casing and trimming trailing ASCII punctuation before the is-or-begins-with comparison). Passing tests xtask/src/pointers.rs:674 tests::rejects_link_text_that_does_not_name_its_destination — table-driven over all seven denied phrases in four variants each, 28 rejections asserted, plus 'See this page.' and a two-word phrase — and xtask/src/pointers.rs:705 tests::accepts_a_three_word_noun_phrase."
  mount_point: "xtask/src/lib.rs — `pub mod pointers;`"
  verifying_test: "xtask/src/pointers.rs::tests::rejects_link_text_that_does_not_name_its_destination; xtask/src/pointers.rs::tests::accepts_a_three_word_noun_phrase"

- id: AC-008
  criterion: "GIVEN Persona 3 mid-session, taking one hop to the passage that answers their second question, WHEN a row records `answer_on_first_screen: false` together with `targets_fragment: false`, THEN `validate` rejects it — the two booleans exist together so a row cannot claim reachability it does not have, which is exactly invariant 3's stated falsification: an inventory entry whose destination is a page rather than a passage where the answer is not on the first screen."
  satisfied: true
  evidence: "xtask/src/pointers.rs:345-357 — check_fragment rejects answer_on_first_screen == false together with targets_fragment == false, with a message that states the invariant it enforces: land the reader on the answer, not the top of the page. Passing tests xtask/src/pointers.rs:714 tests::rejects_a_row_that_admits_a_deep_answer_and_targets_a_page and xtask/src/pointers.rs:729 tests::accepts_a_deep_answer_that_targets_a_fragment."
  mount_point: "xtask/src/lib.rs — `pub mod pointers;`"
  verifying_test: "xtask/src/pointers.rs::tests::rejects_a_row_that_admits_a_deep_answer_and_targets_a_page; xtask/src/pointers.rs::tests::accepts_a_deep_answer_that_targets_a_fragment"

- id: AC-009
  criterion: "GIVEN four sibling stories appending rows to one register across four separate PRs, WHEN `validate` runs over the result, THEN two rows sharing an id are rejected with both surfaces named; **every** problem is reported rather than the first; each message names the offending row id and carries the reason the rule exists, in the message style of `check_summaries` (`xtask/src/lint_constitution.rs:463-473`); and a ninth row fails with the design's own sentence attached — a 21st row means the pointer policy is wrong, not that the register needs a scrollbar — with the 20-rows-ever ceiling documented on the constant."
  satisfied: true
  evidence: "xtask/src/pointers.rs:272-289 — validate accumulates into a Vec<String> in a stable order (register order, then guard/form/link-text/fragment/duplicate within a row, then the cap), documented under its # Errors section. check_unique_id (:359-370) names both surfaces; check_cap (:372-385) names the offending row and carries 'A twenty-first row means the pointer policy is wrong, not that the register needs a scrollbar', with the 20-rows-ever ceiling documented on MAX_ROWS_EVER (:223-229). Passing tests xtask/src/pointers.rs:739 tests::rejects_duplicate_row_ids, :754 tests::reports_every_problem_not_only_the_first, :766 tests::every_message_names_its_row_and_states_its_reason (every message opens with its row id and is at least 120 characters, the MIN_REJECTS_CHARS discipline of xtask/src/lint_constitution.rs applied to this medium) and :796 tests::rejects_a_ninth_row_as_the_dt10_alarm."
  mount_point: "xtask/src/lib.rs — `pub mod pointers;`"
  verifying_test: "xtask/src/pointers.rs::tests::rejects_duplicate_row_ids; xtask/src/pointers.rs::tests::reports_every_problem_not_only_the_first; xtask/src/pointers.rs::tests::every_message_names_its_row_and_states_its_reason; xtask/src/pointers.rs::tests::rejects_a_ninth_row_as_the_dt10_alarm"

- id: AC-010
  criterion: "GIVEN a reviewer or a later implementer asking what navigation was considered and declined (`project.md` AC-011), WHEN they read `xtask/src/pointers.rs`'s module doc, THEN they find the bespoke navigation widget — breadcrumb, master-detail rail, per-crate index page — recorded as declined with its reason, that rustdoc renders a per-item nav bar and any narrative surface renders a TOC so the evaluator's gap is evidence of a missing *link* and not a missing *widget*; and they find the `prelude` module recorded as **out of this project's boundary and owed an ADR, not rejected**, so its absence can never be read as a decision against it."
  satisfied: true
  evidence: "xtask/src/pointers.rs:93-108 — the module doc records the declined bespoke navigation widget (a breadcrumb, a master-detail rail, a per-crate index page) with its reason, that the evaluator's gap is evidence of a missing *link*, not a missing *widget*; and records the prelude module as out of this project's boundary and owed an ADR, not rejected, citing references/evaluation/review-dx-ergonomics.md:421-422. Passing test xtask/src/pointers.rs:826 tests::policy_records_the_declined_widget_and_the_out_of_boundary_prelude."
  mount_point: "xtask/src/lib.rs — `pub mod pointers;`"
  verifying_test: "xtask/src/pointers.rs::tests::policy_records_the_declined_widget_and_the_out_of_boundary_prelude"
```
