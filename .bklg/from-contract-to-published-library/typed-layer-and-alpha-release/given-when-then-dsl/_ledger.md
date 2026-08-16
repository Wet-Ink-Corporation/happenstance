---
item: HS-S0025
stage: implement
created: "2026-08-12T13:46:22.617Z"
updated: "2026-08-12T13:46:22.617Z"
---

# Acceptance ledger — A given/when/then DSL that cannot hide its own filter

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two obligations this story carries that are not AC rows and must still be recorded in the evidence
when the relevant row is flipped:

- **NF-006 — the `happenstance-testkit` dev-dependency route.** Name which of the two admissible
  routes was taken (path-only versionless, or accepted publish order flagged to
  `publish-0-2-0-alpha-1`) in AC-009's evidence, since that row is the one the dev-dependency
  exists for. Do not leave it undecided (`spec.md`, *Data and migrations*, item 1).
- **Findings, not edits.** A `command-loop` signature that does not fit, or a defect in the frozen
  `happenstance-core`, is recorded as a finding and routed under project AC-012 — never patched
  inside this PR (`spec.md`, *Risks and coupling*, rows 2 and 5).

```yaml
- id: AC-001
  criterion: "GIVEN P1 has a DecisionModel and no database — no connection string, no MemoryEventStore of their own, no fixture — WHEN they write given(model).event(a)?.event(b)?.when(|m| m.decide(cmd)).await? and call .then(&[expected]), THEN the assertion passes, the whole test runs in-process, and the only await they wrote is the one on when"
  satisfied: true
  evidence: "crates/happenstance/src/lib.rs:187 declares `pub mod testing;`, so `given(model).event(a)?.when(..).await?.then(&[..])` is the whole shape and the only `await` is on `when`. Verified by crates/happenstance/src/testing/tests.rs::seeded_then_decided_asserts_emitted_events (PASS) and ::composed_boundary_folds_both_models (PASS), the latter over a `(Account, Account)` tuple, which asserts each member folded only what its own query nominated (1 each, not 2)"
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;)"
  verifying_test: "crates/happenstance/src/testing/mod.rs::seeded_then_decided_asserts_emitted_events; crates/happenstance/src/testing/mod.rs::composed_boundary_folds_both_models"

- id: AC-002
  criterion: "GIVEN P1's fold and their command disagree, so the decision emits the wrong events, WHEN .then(&[…]) fails, THEN P1 does not read the log to find out why: the panic renders the header `assertion failed: the decision emitted different events` and then four labelled regions in order — `expected:`, `actual:`, `selected by the model's query:`, `seeded but NOT selected:` — followed by one line naming the derived query, and #[track_caller] puts the panic's location on P1's own assertion line, not inside the DSL"
  satisfied: true
  evidence: "crates/happenstance/src/testing/render.rs:63-80 emits the header and the four labelled regions in order; crates/happenstance/src/testing/mod.rs:292-317 raises the panic under `#[track_caller]`, beside the attribute rather than in the renderer. Verified by crates/happenstance/tests/dsl_failure_message.rs::four_regions_render_in_order (PASS; asserts the header and three byte-offset orderings, because four `contains` calls are satisfied by four labels in any order) and ::panic_location_is_the_callers_line (PASS; a recording panic hook plus `catch_unwind` over a non-capturing `fn()`, asserting file and the exact caller line)"
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;) — surface dsl-failure-message, route crates/happenstance/src/testing/"
  verifying_test: "crates/happenstance/tests/dsl_failure_message.rs::four_regions_render_in_order; crates/happenstance/tests/dsl_failure_message.rs::panic_location_is_the_callers_line"

- id: AC-003
  criterion: "GIVEN P1's EVENT_TYPES omits an event type their fold needs — the ADR-0020 hazard, and the one failure the DSL could hide — WHEN they seed that event and the assertion fails, THEN the event appears by name under `seeded but NOT selected:` and not under `selected by the model's query:`, so the diagnosis is on the screen rather than in a second debugging session; the last line names the derived query that did the filtering"
  satisfied: true
  evidence: "crates/happenstance/src/testing/mod.rs:228-238 computes region four as `seeded \ selected`, where *selected* is the boundary's own derived query and nothing else. Verified by crates/happenstance/tests/dsl_failure_message.rs::region_four_names_the_seeded_but_unselected_event (PASS): the `Withdrawn` event is asserted **present** in the region-four slice and **absent** from the region-three slice, and the closing line is asserted to name the derived query. The oracle is the literal type name the test itself seeded — RS-60-4 — never the DSL's own set difference"
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;) — surface dsl-failure-message, state seeded-but-not-selected"
  verifying_test: "crates/happenstance/tests/dsl_failure_message.rs::region_four_names_the_seeded_but_unselected_event"

- id: AC-004
  criterion: "GIVEN P1 is writing the first test of a new model and has seeded nothing yet, WHEN the assertion fails, THEN the fourth region is still there and says `seeded but NOT selected: (nothing was seeded)` — the region never disappears, because a missing region reads as 'the DSL has nothing to say about the filter' when it means 'there was nothing to filter'"
  satisfied: true
  evidence: "crates/happenstance/src/testing/render.rs:106-119 keeps the region and prints `(nothing was seeded)` when the difference is empty. Verified by crates/happenstance/tests/dsl_failure_message.rs::empty_seed_keeps_region_four_and_says_so (PASS) and ::empty_selection_is_not_an_error (PASS; `given(model).when(..)` over an unseeded store returns Ok and the model's counter is still 0, so `apply` never ran)"
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;) — surface dsl-failure-message, state empty-selection"
  verifying_test: "crates/happenstance/tests/dsl_failure_message.rs::empty_seed_keeps_region_four_and_says_so; crates/happenstance/tests/dsl_failure_message.rs::empty_selection_is_not_an_error"

- id: AC-005
  criterion: "GIVEN P1 is debugging a model against a log of 200 seeded events on an 80-column terminal, WHEN the assertion fails, THEN the message stays readable and stays honest: each region prints at most 8 event rows, each row at most 80 columns, `selected by the model's query:` truncates first and `seeded but NOT selected:` truncates last, every truncation prints `… and N more`, and a long event type wraps rather than being cut — position and event type are never truncated to fit, and the `not selected` marker column stays left-aligned"
  satisfied: true
  evidence: "crates/happenstance/src/testing/render.rs:14-17 (8 rows, 80 columns), :127-132 (`… and N more`), :161-194 (wrap, never truncate). Verified by crates/happenstance/tests/dsl_failure_message.rs::overflow_truncates_selected_first_and_the_diagnosis_last (PASS; seeds 20 selected + 1 unselected, asserts `… and 12 more` in region three, no `… and` in region four, and every line at most 80 characters) and ::long_event_type_wraps_without_truncating_type_or_position (PASS; an 84-character event type, every line within budget, the type still present with whitespace removed, and every region-four row still starting at the `not selected` marker column)"
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;) — surface dsl-failure-message, states overflow + long-label"
  verifying_test: "crates/happenstance/tests/dsl_failure_message.rs::overflow_truncates_selected_first_and_the_diagnosis_last; crates/happenstance/tests/dsl_failure_message.rs::long_event_type_wraps_without_truncating_type_or_position"

- id: AC-006
  criterion: "GIVEN P1's domain refuses the command — the course is full — WHEN they assert with .then_refused(), THEN it passes, and their own error type survives the round trip as a typed value rather than a string; a refusal never routes through when's Err arm, which is store and codec failure only"
  satisfied: true
  evidence: "crates/happenstance/src/testing/mod.rs:222-225 captures the decide closure's `Err(d)` **into** the `Decision` as `Outcome::Refused(Box<dyn Error>)`, so `when`'s Err arm is store and codec failure only. Verified by crates/happenstance/src/testing/tests.rs::refusal_is_asserted_through_then_refused (PASS; downcasts the boxed refusal back to the caller's own `Overdrawn`, so it is a typed value rather than a rendered string) and ::refusal_does_not_route_through_when_err (PASS), plus crates/happenstance/tests/dsl_failure_message.rs::then_on_a_refused_decision_panics_naming_the_refusal (PASS; EC-006's wrong implementation — a refusal compared against `&[]` and passing — is rejected)"
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;) — happenstance::testing::Decision"
  verifying_test: "crates/happenstance/src/testing/mod.rs::refusal_is_asserted_through_then_refused; crates/happenstance/src/testing/mod.rs::refusal_does_not_route_through_when_err; crates/happenstance/tests/dsl_failure_message.rs::then_on_a_refused_decision_panics_naming_the_refusal"

- id: AC-007
  criterion: "GIVEN P1 wants the test to be evidence about their application, WHEN they seed with Given::event(e), THEN the bytes written are the bytes commit would have written — the same codec, the same tag, the same fallibility surfaced as Result<Self, CodecError> — and if a seeded event is nominated by the query but cannot be decoded, the DSL reports CodecError::UnknownEventType instead of quietly folding a shorter log. The same story ships assert_domain_event, so P1 can prove every variant's event_type() is in EVENT_TYPES — the one agreement the compiler cannot make for them"
  satisfied: true
  evidence: "crates/happenstance/src/testing/mod.rs:144-159 encodes with the boundary's codec and attaches `crate::codec::frame::<Json>(None)` — the same framing region `commit` writes. Verified by crates/happenstance/src/testing/tests.rs::seeded_bytes_are_the_bytes_commit_writes (PASS; reads the raw `Event` back through the store and compares its data against `Json.encode(&value)` and its metadata against `frame::<Json>(None)`, never against a literal), ::nominated_but_undecodable_event_is_an_error (PASS; a nominated event the fold cannot decode surfaces as `CommandError::Decode`, not a shorter log) and ::non_nominated_type_is_skipped_not_an_error (PASS). `assert_domain_event` is at mod.rs:378-393 and its wrong implementation is rejected by crates/happenstance/tests/dsl_failure_message.rs::assert_domain_event_rejects_a_variant_outside_event_types (PASS)"
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;) — happenstance::testing::{Given, assert_domain_event}"
  verifying_test: "crates/happenstance/src/testing/mod.rs::seeded_bytes_are_the_bytes_commit_writes; crates/happenstance/src/testing/mod.rs::nominated_but_undecodable_event_is_an_error; crates/happenstance/src/testing/mod.rs::non_nominated_type_is_skipped_not_an_error; crates/happenstance/tests/dsl_failure_message.rs::assert_domain_event_rejects_a_variant_outside_event_types"

- id: AC-008
  criterion: "GIVEN P1 builds a Decision and then decides not to assert on it — a half-written test, an early return — WHEN the value is dropped, THEN nothing was appended and nothing was mutated: everything up to the append is pure, and the compiler says so through #[must_use = \"a Decision is not appended until it is committed; dropping it discards the events the decision produced\"], verbatim"
  satisfied: true
  evidence: "crates/happenstance/src/testing/mod.rs:266-268 carries the `#[must_use]` verbatim. Verified by crates/happenstance/src/testing/tests.rs::dropping_a_decision_appends_nothing (PASS; holds an `Arc` of the store `given` created, drops the `Decision`, re-reads and asserts the seeded set is unchanged — the decision's two events never landed) and ::must_use_message_is_verbatim (PASS; reads the attribute's own literal out of `mod.rs` with `include_str!`, unescapes the line continuation and compares it against the `DECISION_MUST_USE` const kept beside it). The lint itself is enforced by `cargo clippy --all-targets -- -D warnings`"
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;) — happenstance::testing::Decision"
  verifying_test: "crates/happenstance/src/testing/mod.rs::dropping_a_decision_appends_nothing; crates/happenstance/src/testing/mod.rs::must_use_message_is_verbatim"

- id: AC-009
  criterion: "GIVEN P1 has written a retry loop and cannot reproduce a write conflict on demand, WHEN they wrap MemoryEventStore's handle in FaultyStore::new(h).violate_next(1) and run commit_with under a visible Retry bound, THEN the call succeeds, Committed.attempts reads 2, the loop re-read and re-decided from a pristine model rather than reusing a stale fold — and it never branched on conflicting_position, which this fixture reports as None exactly as a remote store legitimately does"
  satisfied: true
  evidence: "crates/happenstance/tests/retry_without_a_database.rs::retry_succeeds_after_an_injected_violation (PASS; `FaultyStore::new(fixture.connect().await).violate_next(1)` under `Retry::attempts(3)`, `Committed.attempts == 2` — and because the injected violation reports `conflicting_position: None`, a loop branching on `Some` could not have reached success), ::retry_refolds_from_a_pristine_model (PASS; the decide closure asserts the folded state is 1 on **both** attempts, so a stale fold fails it, and the closure is proved to have run twice) and ::retry_is_bounded (PASS; `violate_next(9)` under a bound of 3 gives `CommandError::Exhausted { attempts: 3 }` rather than a hang). No connection, socket, temp file or clock appears anywhere in the file; the store is `MemoryEventStore` behind the testkit's reference fixture"
  mount_point: "crates/happenstance/tests/ (integration), driving crates/happenstance/src/lib.rs's commit_with against happenstance_testkit::FaultyStore"
  verifying_test: "crates/happenstance/tests/retry_without_a_database.rs::retry_succeeds_after_an_injected_violation; crates/happenstance/tests/retry_without_a_database.rs::retry_refolds_from_a_pristine_model; crates/happenstance/tests/retry_without_a_database.rs::retry_is_bounded"

- id: AC-010
  criterion: "GIVEN P4 has one sitting to decide whether this library is real and will not run its test suite, WHEN they open the crate-root page on docs.rs, THEN they find happenstance::testing revealed — one short region, 'Testing without a database', pointing one click away, not interleaved with the four names of the first program and not buried in a second crate — every gated item there carries its doc_cfg badge, every intra-doc link resolves with default features and with --no-default-features, no item summary ends in an ellipsis, and no name in testing shadows a contract re-export"
  satisfied: true
  evidence: "crates/happenstance/src/lib.rs:125-136 is the added `# Testing without a database` region, between Features and the adapter pointer; :154-161 spells the link target conditionally so it resolves with the features on and off. Verified by crates/happenstance/tests/mounted_at_the_crate_root.rs::testing_is_reachable_by_its_public_path (PASS; names `given`, `Given`, `Decision` and `assert_domain_event` through `happenstance::testing::…`, so a module that compiles but is never declared fails here), ::the_crate_root_page_points_at_the_testing_module (PASS; asserts the region exists and sits between the two regions the design fixes) and ::every_doc_fence_line_fits_the_column_budget (PASS; anti-pattern 3, every doc-fence line at most 72 columns). `RUSTDOCFLAGS='-D warnings' cargo doc -p happenstance --no-deps` and the same with `--no-default-features` are both clean, as is the gate's own workspace docs step under `--document-private-items`"
  mount_point: "crates/happenstance/src/lib.rs — pub mod testing; plus module-doc region 6 'Testing without a database' (surface crate-root-rustdoc, changed)"
  verifying_test: "crates/happenstance/tests/mounted_at_the_crate_root.rs::testing_is_reachable_by_its_public_path; cargo doc -p happenstance --no-deps and --no-default-features under RUSTDOCFLAGS=\"-D warnings\""
```
