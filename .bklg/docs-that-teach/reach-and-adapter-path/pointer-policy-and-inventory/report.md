---
item: "HS-S0154"
stage: report
created: "2026-08-17T13:16:12.681Z"
updated: "2026-08-17T13:16:12.681Z"
---

# Report — The pointer policy, landed as in-tree substrate

## Findings Ledger

Ten acceptance criteria, all satisfied by reachable behaviour in `xtask`'s **lib** target, each
with a `file:line` citation and a passing test. Nothing deferred, nothing blocked, no `#[allow]`
added, no test weakened to reach green.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **The policy is one file, and it is asserted rather than trusted** — DT-10's resolution, rule 1's single byte-identically mirrored sentence, gates (i)-(iv), and the href ladder whose last rung is *do not install, escalate* | `xtask/src/pointers.rs:16-60`; `tests::policy_states_the_dt10_resolution_its_four_gates_and_the_href_ladder` (`:489`) pins nine short subjects against the module doc only | The four gates are now copyable by HS-S0156, HS-S0158 and HS-S0159 without re-opening `_design.md` |
| **The permitted forms are closed at four, and each documents what catches it rotting** | `xtask/src/pointers.rs:129-163`; `tests::permitted_forms_are_closed_and_each_documents_its_guard` (`:509`) — an exhaustive `match` with no wildcard arm, so a fifth variant is a compile error in the test rather than a silent widening | Adding a variant re-opens `_decomposition.md` N-3; the module doc says so at `:122-127` |
| **The over-broad README guard claim was corrected in the substrate, not inherited** — the doctest mount compiles the README's Rust fences and *not* its prose | `xtask/src/pointers.rs:78-91`; `tests::policy_records_the_readme_prose_gap_and_the_alias_rule` (`:549`) | **HS-S0158 must file P2 with the honest guard** (the mirror assertion, which does not exist yet), never with `project.md` AC-002's flattering one |
| **The mount is real and displaces nothing** — `pub mod pointers;` in the lib crate root, one changed line elsewhere in the file | `xtask/src/lib.rs:41-47`, `:1`; `tests::the_mount_declares_the_module_and_displaces_nothing` (`:578`) | — |
| **The register lands empty, capped, and unrendered** | `xtask/src/pointers.rs:238`, `:216-229`; `tests::the_register_lands_empty_and_valid` (`:566`) | Four sibling stories each file their own rows; the cap at 8 is the alarm they will hit if the policy is wrong |
| **Six rejection rules, each with a named wrong register** — empty guard, bare URL, non-self-describing link text, deep answer with no fragment, duplicate id, ninth row | `xtask/src/pointers.rs:291-385`; tests at `:627`, `:641`, `:652`, `:674`, `:714`, `:739`, `:796` | — |
| **The messages are the deliverable, and the bar is enforced** — every message opens with its row id and carries at least 120 characters of reason | `tests::every_message_names_its_row_and_states_its_reason` (`:766`) | This is the "unstyled render" guard in a medium with no pixels: a `validate` returning a bare `false` would satisfy every other assertion here |
| **Two obligations are named rather than faked** — a pointer installed with no row filed (EC-004), and a guard that names a mechanism which does not exist (EC-002) | `xtask/src/pointers.rs:110-124` | Both are review obligations carried in each installing story's own spec; the register is what makes them reviewable |
| **`redkiln verify --grain story` reports `boundary` FAIL on the branch's accumulated diff** | The check diffs against `main`; `initiative/docs-that-teach` carries three completed projects ahead of this story. This story's own diff is `xtask/src/lib.rs`, `xtask/src/pointers.rs` and its backlog folder — a strict subset of `spec.md`'s PR-boundary block | Not actionable here; it resolves when the branch merges |
| **`provenance` FAIL is expected pre-commit** | `links.commits` is empty until the checkpoint commit exists | `redkiln record-links HS-S0154 --sha <sha>` — the orchestrating command's step, not this story's |

## Acceptance

| AC | Status | Verified by |
| --- | --- | --- |
| **AC-001** — the policy states DT-10's resolution, the four gates and the href ladder | **satisfied** | `xtask/src/pointers.rs:16-60`; `tests::policy_states_the_dt10_resolution_its_four_gates_and_the_href_ladder` |
| **AC-002** — four closed forms each documenting its guard; the README-prose correction; the alias rule | **satisfied** | `:129-163`, `:78-91`, `:61-76`; `tests::permitted_forms_are_closed_and_each_documents_its_guard`, `tests::policy_records_the_readme_prose_gap_and_the_alias_rule` |
| **AC-003** — mounted, register present and empty and valid, no new `Step`, nothing rendered | **satisfied** | `xtask/src/lib.rs:41-47`; `xtask/src/pointers.rs:238`; `tests::the_register_lands_empty_and_valid`, `tests::the_mount_declares_the_module_and_displaces_nothing`; `cargo xtask ci --fast` green |
| **AC-004** — the row type requires every fact a guard claim needs, and prints itself | **satisfied** | `:182-214`; `tests::a_row_records_every_fact_a_guard_claim_needs` |
| **AC-005** — an empty or whitespace guard is rejected, by name and with the reason | **satisfied** | `:291-303`; `tests::rejects_a_row_whose_guard_is_empty`, `tests::rejects_a_row_whose_guard_is_only_whitespace` |
| **AC-006** — a bare URL is rejected outright and the message states the escalation | **satisfied** | `:305-318`; `tests::rejects_a_bare_url_outright_and_states_the_escalation` |
| **AC-007** — link text under three words, or beginning with a denied phrase, is rejected | **satisfied** | `:241-269`, `:320-343`; `tests::rejects_link_text_that_does_not_name_its_destination` (28 cases), `tests::accepts_a_three_word_noun_phrase` |
| **AC-008** — a deep answer that targets a page rather than a fragment is rejected | **satisfied** | `:345-357`; `tests::rejects_a_row_that_admits_a_deep_answer_and_targets_a_page`, `tests::accepts_a_deep_answer_that_targets_a_fragment` |
| **AC-009** — duplicate ids named with both surfaces, every problem reported, the cap as an alarm | **satisfied** | `:272-289`, `:359-385`, `:223-229`; `tests::rejects_duplicate_row_ids`, `tests::reports_every_problem_not_only_the_first`, `tests::every_message_names_its_row_and_states_its_reason`, `tests::rejects_a_ninth_row_as_the_dt10_alarm` |
| **AC-010** — the declined widget on record, and `prelude` recorded as deferred rather than rejected | **satisfied** | `:93-108`; `tests::policy_records_the_declined_widget_and_the_out_of_boundary_prelude` |

**Mount point**: `xtask/src/lib.rs` — `pub mod pointers;` beside `mod constitution;` and
`mod narrative;`. That declaration is what puts the register inside
`cargo clippy --all-targets --all-features -D warnings`, `cargo test --workspace` and the gate's
`documentation` step; without it the module compiles nowhere and runs nothing (EC-007).

**Surfaces rendered**: none, and that is the design's `## Transience policy` decision rather than
an omission — the register is build-time data, because a page listing every pointer is exactly the
second navigation surface DT-10 warns about. The five reader-facing surfaces belong to this
story's four dependents.

**Deferred**: nothing. **Blocked**: nothing.

## Knowledge Harvest

Candidates for `.kb/` at closeout, authored through `/redkiln:kb-ingest` from `.kb/_intake/` and
not by hand. None is promoted here.

1. **A policy that is not data rots invisibly.** The pattern this story landed — prose policy in a
   module doc, its rows as a `const` slice, and a pure validator with a named wrong register per
   rule — is the third instance in this repository of the `SUMMARIES` / `check_summaries` shape
   (`xtask/src/lint_constitution.rs:64`, `:463-473`). Three instances is where a *playbook* starts
   being cheaper than a fourth rediscovery.
2. **Testing a doc comment without pinning paragraphs.** `production_source()` plus
   whitespace-normalised prose plus short pinned subjects is a reusable technique for asserting
   that a policy text still says the load-bearing things, and it has a stated failure mode: pin a
   paragraph and the first person to reword the policy deletes the test.
   `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` is the neighbouring atom this
   would extend rather than duplicate.
3. **A message-length floor as the "presentation exists" bar in a non-visual medium.**
   `MIN_MESSAGE_CHARS = 120` is `MIN_REJECTS_CHARS` transplanted, and the argument transfers: a
   check that returns a verdict without a reason satisfies every structural assertion and teaches
   nobody anything. Worth recording as a concept because a design review cannot see a build log.
4. **Not for `.kb/`: the pointer policy itself.** It is project substrate, scoped to
   `docs-that-teach`. Promoting it is a closeout decision, and `_grounding.md` records the audit
   finding that no Accepted decision atom governs pointer policy today — which is why this story
   wrote substrate rather than citing an ADR, and why it authored none.
5. **A live, evidenced proposal that is owed an ADR and did not get one here.** A `prelude` module
   exporting `EventStore` and not `SendEventStore` would make the `E0034` collision unreachable by
   the default import path (`references/evaluation/review-dx-ergonomics.md:421-422`). It is
   recorded in `xtask/src/pointers.rs:100-108` as out of boundary and **not rejected**, so a later
   reader cannot mistake its absence for a decision against it.
