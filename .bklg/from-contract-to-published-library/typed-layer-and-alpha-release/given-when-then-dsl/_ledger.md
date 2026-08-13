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
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;)"
  verifying_test: "crates/happenstance/src/testing/mod.rs::seeded_then_decided_asserts_emitted_events; crates/happenstance/src/testing/mod.rs::composed_boundary_folds_both_models"

- id: AC-002
  criterion: "GIVEN P1's fold and their command disagree, so the decision emits the wrong events, WHEN .then(&[…]) fails, THEN P1 does not read the log to find out why: the panic renders the header `assertion failed: the decision emitted different events` and then four labelled regions in order — `expected:`, `actual:`, `selected by the model's query:`, `seeded but NOT selected:` — followed by one line naming the derived query, and #[track_caller] puts the panic's location on P1's own assertion line, not inside the DSL"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;) — surface dsl-failure-message, route crates/happenstance/src/testing/"
  verifying_test: "crates/happenstance/tests/dsl_failure_message.rs::four_regions_render_in_order; crates/happenstance/tests/dsl_failure_message.rs::panic_location_is_the_callers_line"

- id: AC-003
  criterion: "GIVEN P1's EVENT_TYPES omits an event type their fold needs — the ADR-0020 hazard, and the one failure the DSL could hide — WHEN they seed that event and the assertion fails, THEN the event appears by name under `seeded but NOT selected:` and not under `selected by the model's query:`, so the diagnosis is on the screen rather than in a second debugging session; the last line names the derived query that did the filtering"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;) — surface dsl-failure-message, state seeded-but-not-selected"
  verifying_test: "crates/happenstance/tests/dsl_failure_message.rs::region_four_names_the_seeded_but_unselected_event"

- id: AC-004
  criterion: "GIVEN P1 is writing the first test of a new model and has seeded nothing yet, WHEN the assertion fails, THEN the fourth region is still there and says `seeded but NOT selected: (nothing was seeded)` — the region never disappears, because a missing region reads as 'the DSL has nothing to say about the filter' when it means 'there was nothing to filter'"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;) — surface dsl-failure-message, state empty-selection"
  verifying_test: "crates/happenstance/tests/dsl_failure_message.rs::empty_seed_keeps_region_four_and_says_so; crates/happenstance/tests/dsl_failure_message.rs::empty_selection_is_not_an_error"

- id: AC-005
  criterion: "GIVEN P1 is debugging a model against a log of 200 seeded events on an 80-column terminal, WHEN the assertion fails, THEN the message stays readable and stays honest: each region prints at most 8 event rows, each row at most 80 columns, `selected by the model's query:` truncates first and `seeded but NOT selected:` truncates last, every truncation prints `… and N more`, and a long event type wraps rather than being cut — position and event type are never truncated to fit, and the `not selected` marker column stays left-aligned"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;) — surface dsl-failure-message, states overflow + long-label"
  verifying_test: "crates/happenstance/tests/dsl_failure_message.rs::overflow_truncates_selected_first_and_the_diagnosis_last; crates/happenstance/tests/dsl_failure_message.rs::long_event_type_wraps_without_truncating_type_or_position"

- id: AC-006
  criterion: "GIVEN P1's domain refuses the command — the course is full — WHEN they assert with .then_refused(), THEN it passes, and their own error type survives the round trip as a typed value rather than a string; a refusal never routes through when's Err arm, which is store and codec failure only"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;) — happenstance::testing::Decision"
  verifying_test: "crates/happenstance/src/testing/mod.rs::refusal_is_asserted_through_then_refused; crates/happenstance/src/testing/mod.rs::refusal_does_not_route_through_when_err; crates/happenstance/tests/dsl_failure_message.rs::then_on_a_refused_decision_panics_naming_the_refusal"

- id: AC-007
  criterion: "GIVEN P1 wants the test to be evidence about their application, WHEN they seed with Given::event(e), THEN the bytes written are the bytes commit would have written — the same codec, the same tag, the same fallibility surfaced as Result<Self, CodecError> — and if a seeded event is nominated by the query but cannot be decoded, the DSL reports CodecError::UnknownEventType instead of quietly folding a shorter log. The same story ships assert_domain_event, so P1 can prove every variant's event_type() is in EVENT_TYPES — the one agreement the compiler cannot make for them"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;) — happenstance::testing::{Given, assert_domain_event}"
  verifying_test: "crates/happenstance/src/testing/mod.rs::seeded_bytes_are_the_bytes_commit_writes; crates/happenstance/src/testing/mod.rs::nominated_but_undecodable_event_is_an_error; crates/happenstance/src/testing/mod.rs::non_nominated_type_is_skipped_not_an_error; crates/happenstance/tests/dsl_failure_message.rs::assert_domain_event_rejects_a_variant_outside_event_types"

- id: AC-008
  criterion: "GIVEN P1 builds a Decision and then decides not to assert on it — a half-written test, an early return — WHEN the value is dropped, THEN nothing was appended and nothing was mutated: everything up to the append is pure, and the compiler says so through #[must_use = \"a Decision is not appended until it is committed; dropping it discards the events the decision produced\"], verbatim"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (pub mod testing;) — happenstance::testing::Decision"
  verifying_test: "crates/happenstance/src/testing/mod.rs::dropping_a_decision_appends_nothing; crates/happenstance/src/testing/mod.rs::must_use_message_is_verbatim"

- id: AC-009
  criterion: "GIVEN P1 has written a retry loop and cannot reproduce a write conflict on demand, WHEN they wrap MemoryEventStore's handle in FaultyStore::new(h).violate_next(1) and run commit_with under a visible Retry bound, THEN the call succeeds, Committed.attempts reads 2, the loop re-read and re-decided from a pristine model rather than reusing a stale fold — and it never branched on conflicting_position, which this fixture reports as None exactly as a remote store legitimately does"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/tests/ (integration), driving crates/happenstance/src/lib.rs's commit_with against happenstance_testkit::FaultyStore"
  verifying_test: "crates/happenstance/tests/retry_without_a_database.rs::retry_succeeds_after_an_injected_violation; crates/happenstance/tests/retry_without_a_database.rs::retry_refolds_from_a_pristine_model; crates/happenstance/tests/retry_without_a_database.rs::retry_is_bounded"

- id: AC-010
  criterion: "GIVEN P4 has one sitting to decide whether this library is real and will not run its test suite, WHEN they open the crate-root page on docs.rs, THEN they find happenstance::testing revealed — one short region, 'Testing without a database', pointing one click away, not interleaved with the four names of the first program and not buried in a second crate — every gated item there carries its doc_cfg badge, every intra-doc link resolves with default features and with --no-default-features, no item summary ends in an ellipsis, and no name in testing shadows a contract re-export"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs — pub mod testing; plus module-doc region 6 'Testing without a database' (surface crate-root-rustdoc, changed)"
  verifying_test: "crates/happenstance/tests/mounted_at_the_crate_root.rs::testing_is_reachable_by_its_public_path; cargo doc -p happenstance --no-deps and --no-default-features under RUSTDOCFLAGS=\"-D warnings\""
```
