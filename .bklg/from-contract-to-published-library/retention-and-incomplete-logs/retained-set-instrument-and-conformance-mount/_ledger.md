---
item: HS-S0114
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The completeness instrument, mounted through the conformance suite

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, so a row is not flipped on the wrong evidence:

- **`crates/happenstance-testkit/tests/completeness_instrument.rs` does not exist yet.** Every
  `mount_point` and every target-local `verifying_test` below names the file this story creates. A row
  citing a test in `crates/happenstance-testkit/src/**` as its evidence is citing the wrong thing —
  this story adds nothing under `src/`.
- **"Control mount"** means the `event_store_conformance!` invocation whose retained set hides
  nothing; **"hole-carrying mount"** means one that hides something. Rules named against the
  hole-carrying mount are evidence only from a mount that was committed green. A red rule is triaged
  per EC-006 and handed to `cf-27-experiment-and-recorded-pass-list` — never dissolved by widening the
  retained set, and never used to flip a row.

```yaml
- id: AC-001
  criterion: |-
    GIVEN the repository owner is about to run CF-27's experiment and needs a store that can be told what it no longer holds, WHEN they run `happenstance-testkit`'s test targets, THEN `happenstance_testkit::event_store_conformance!` has already been invoked on the retained-set fixture in its **control** configuration inside `crates/happenstance-testkit/tests/completeness_instrument.rs`, every rule the `for_each_event_store_rule!` family emits runs against it and passes, and the instrument is reachable **through that macro alone** — no hand-rolled test driver anywhere in the target, and no `#[tokio::test]` that exercises the store outside a generated module.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs — the `happenstance_testkit::event_store_conformance!` invocation on the control configuration, shaped after crates/happenstance-testkit/tests/fixture_instruments.rs:201-205; macro at crates/happenstance-testkit/src/lib.rs:312-356"
  verifying_test: "cargo test -p happenstance-testkit --test completeness_instrument — the modules generated from for_each_event_store_rule! (crates/happenstance-testkit/src/registry.rs:94-102), all green against the control mount"

- id: AC-002
  criterion: |-
    GIVEN an experimenter who must commit a pass list **per configuration** and cannot publish a result whose configuration is unprintable, WHEN they construct the suffix case (a prune keeping only the top of the log) and the scattered case (a regulated purge with survivors *below* the hole), THEN both are values of the same retained-set type constructed without editing it, `retains()` answers correctly for a position inside the hole, above it and below it in each, and the type's `Debug` rendering names the variant **and its bound** so the recorded row identifies the shape that produced it.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs — the retained-set type (`RetainedSet`) consumed by every `event_store_conformance!` mount in the same file"
  verifying_test: "crates/happenstance-testkit/tests/completeness_instrument.rs — target-local unit tests over `retains()` in the suffix and scattered configurations at a position inside, above and below the hole, plus the `Debug`-names-variant-and-bound assertion"

- id: AC-003
  criterion: |-
    GIVEN an adapter author who needs to know that a position handed out before a hole opened is never handed out again, WHEN a hole opens and a further append lands, THEN the inner `MemoryEventStore` was never rebuilt — `MemoryEventStore::restore` is called nowhere in this target — the newly assigned position is strictly greater than every position the store had already assigned including forgotten ones, and the retained view still reads unique, strictly monotonic positions across the hole.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs — the decorating handle's `append`, over the live inner `MemoryEventStore` held by the fixture; the hole-carrying `event_store_conformance!` mount in the same file"
  verifying_test: "positions_are_unique and positions_are_strictly_monotonic (crates/happenstance-testkit/src/registry.rs:141-142) green in the hole-carrying mount, plus target-local `append_after_a_hole_allocates_above_it` in crates/happenstance-testkit/tests/completeness_instrument.rs"

- id: AC-004
  criterion: |-
    GIVEN a caller paging a store that has forgotten rows in the *middle* of the range it is reading, WHEN it reads with a `limit`, a `from`, a `to` and/or `backwards`, THEN it gets `limit` **retained** events rather than a short page (the limit is spent on survivors, never on forgotten rows), `from` still names a **position and not an index** so a read from an unoccupied position still yields the next retained event, ordering and bounds still come from core — AND `read` is declared non-`async`, returning the stream at the top level, so the `Send` flavour keeps `+ Send` on the stream rather than on a future wrapping it.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs — the decorating handle's non-`async` `read`, mounted through the hole-carrying `event_store_conformance!` invocation in the same file"
  verifying_test: "read_limit_applies_after_filtering, read_backwards_limit_applies_after_filtering, read_limit_truncates, read_limit_zero_yields_nothing, limit_applies_across_items_not_per_item, read_from_is_inclusive, read_to_is_inclusive, read_from_and_to_bound_a_closed_window, read_backwards_from_with_limit, read_from_a_gap_position (crates/happenstance-testkit/src/registry.rs:124-138) green in the hole-carrying mount, plus the target-local generic `requires_send` application"

- id: AC-005
  criterion: |-
    GIVEN a reader asking the store what it currently has, WHEN the top of the log is outside the retained set and when an event it once held has been forgotten, THEN `head()` reports the highest **retained** position rather than the inner store's, and `contains_event_id` answers `false` for the forgotten event — resolved by looking the id up in the inner store's `snapshot()` and consulting the retained set, **never** by reading `EventId::position()`, because that half of an id belongs to the *origin* store for an ingested event.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs — the decorating handle's `head` and `contains_event_id`, mounted through both the control and the hole-carrying `event_store_conformance!` invocations in the same file"
  verifying_test: "head_is_the_highest_visible_position, head_of_an_empty_store_is_none, head_advances_across_two_handles, contains_event_id_reports_membership (crates/happenstance-testkit/src/registry.rs:144-158) green in both mounts, plus target-local `head_reports_the_highest_retained_position` and `contains_event_id_is_false_for_a_forgotten_event`"

- id: AC-006
  criterion: |-
    GIVEN an ingest author appending under a condition to a store whose matching history has been destroyed, WHEN the append is submitted, THEN an empty batch is still refused **before** the condition is looked at; the condition is evaluated by the instrument against its **retained view alone**, so it passes vacuously where the destroyed history would have rejected — ES-40's specified behaviour, not a bug; a violation found on a *retained* event is reported as `AppendError::ConditionViolated(ConditionViolated::at(p))` naming that retained position; the write is then delegated **unconditionally** so the inner store never re-judges it; and the evaluate-then-write pair is serialised across handles so a race still elects exactly one winner.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs — the decorating handle's `append` (retained-view condition scan, then unconditional delegation) under the fixture's shared `tokio::sync::Mutex`; both `event_store_conformance!` invocations in the same file"
  verifying_test: "empty_batch_is_refused_before_the_condition_is_evaluated, append_rejects_empty_batch, the condition_* family (crates/happenstance-testkit/src/registry.rs:190-203), condition_rejection_is_reported_as_condition_violated, racing_conditional_appends_elect_one_winner and interleaved_appends_on_one_handle_elect_one_winner (:206-209) green in both mounts, plus target-local `condition_over_a_forgotten_match_admits_the_append`"

- id: AC-007
  criterion: |-
    GIVEN an adapter author reading this fixture to learn what an honest one looks like, WHEN they call `connect()` on a fresh fixture instance, THEN they receive a handle onto a store holding **nothing** — no pre-seeded history, no eager sliding window, both named dishonest resolutions in DA-3 — two handles from one instance observe each other's appends while two instances observe neither, `SECOND_HANDLE` is `Capability::SUPPORTED` because two handles onto one `Arc` genuinely do observe each other, `REOPEN` is **declined with the real reason** in `MemoryFixture`'s own wording, and **no item of any kind is added to the `Fixture` trait**.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs — the `happenstance_testkit::Fixture` impl (`connect()`, `SECOND_HANDLE`, `REOPEN`) against the contract at crates/happenstance-testkit/src/contract.rs:120-321, modelled on crates/happenstance-testkit/src/fixtures.rs:243-292"
  verifying_test: "reading_an_empty_store_yields_nothing, head_of_an_empty_store_is_none, condition_against_an_empty_store_admits_the_append, two_fixture_instances_observe_none_of_each_others_appends, two_handles_observe_each_others_appends (crates/happenstance-testkit/src/registry.rs:105-129) green, acknowledged_writes_survive_a_reopen (:107) reported as declined-with-reason, and `git diff --stat` empty over crates/happenstance-testkit/src/**"

- id: AC-008
  criterion: |-
    GIVEN a future maintainer or a downstream user who might mistake the instrument for a shippable adapter, WHEN they look for it from outside the crate, THEN it is not nameable: it lives in `crates/happenstance-testkit/tests/` and never `src/fixtures/`, declares no crate of its own, and no public item of `happenstance-core`, `happenstance` or `happenstance-testkit` refers to it — AND when they read it, its module rustdoc states in terms that it is an instrument that is never a target, that its retained set is arbitrary despite CF-27's word "suffix" and why, that the inner store is concrete as a deliberate narrowing of *"over any `EventStore`"*, and that hiding at the port discharges **falsifiability** and not **implementability**, leaving CF-25/CF-26's completeness-axis exposure recorded **open** rather than implied closed.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs — its location (never crates/happenstance-testkit/src/fixtures/), its module-level rustdoc, and the absence of any reference to it from the public API of happenstance-core, happenstance and happenstance-testkit"
  verifying_test: "`git diff --stat` confined to the PR-boundary globs; a search of crates/happenstance-core/src/, crates/happenstance/src/ and crates/happenstance-testkit/src/ for the instrument's type names returning nothing; `cargo package --list` over the three publishable crates (the existing `cargo xtask ci` step); content review of the module rustdoc against the four named sentences"
```
