---
item: "HS-S0063"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The concurrency family green on unserialised writers

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
  criterion: "GIVEN an adapter author has built a Postgres store and has no instrument that tells them whether it is correct under contention, WHEN they run the crate's live test target against a pinned Postgres, THEN all five rules of the concurrency family execute and report by name — none absent, none reporting `Skipped`, none `#[cfg]`-ed out of the macro expansion — AND WHEN a colleague with no Docker and no network runs the default gate, the same target does not run and the command still exits zero."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/concurrency_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/concurrency_conformance.rs::dcb_concurrency_conformance (all five emitted tests, read from `cargo test -p happenstance-postgres --all-features -- --ignored --show-output` in the live-Postgres CI job), plus `cargo xtask ci --fast` green with no Docker"
- id: AC-002
  criterion: "GIVEN eight instances of an application each decide, from the same read of the same consistency boundary, that they may append, WHEN all eight append under that one `AppendCondition::after` at once, THEN exactly one is acknowledged, the other seven are told `ConditionViolated` rather than being silently accepted, and a reader opening a fresh handle afterwards finds exactly the one batch that was acknowledged — so the probe-then-insert shape that passes every sequential rule is caught here instead of in production."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/concurrency_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/concurrency_conformance.rs::dcb_concurrency_conformance::exactly_one_of_n_contenders_commits"
- id: AC-003
  criterion: "GIVEN four teams' commands touch four unrelated consistency boundaries — the independence property Dynamic Consistency Boundary exists to provide — WHEN three writers contend on each of the four at once, THEN exactly four commits result: one winner per boundary, no boundary left with none, and no boundary with two — so an author who reached for `SERIALIZABLE` and mapped `40001 serialization_failure` onto `ConditionViolated` learns from the suite that their store now refuses commands that never conflicted."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/concurrency_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/concurrency_conformance.rs::dcb_concurrency_conformance::k_disjoint_boundaries_admit_exactly_k_commits"
- id: AC-004
  criterion: "GIVEN eight writers append unconditionally at the same moment and every one of them is acknowledged, WHEN a consuming application later reads the store to build any position-ordered view, THEN no two events anywhere in the store share a position — so a `SELECT max(position)` taken outside the transaction that consumes it cannot hand two writers the same slot and make one event unreachable by position."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/concurrency_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/concurrency_conformance.rs::dcb_concurrency_conformance::positions_are_unique_under_concurrent_appends (with `cargo xtask lint-position-literals` green)"
- id: AC-005
  criterion: "GIVEN a consuming application appends a batch and checkpoints its projection at the position `append` returned, WHEN seven other writers were committing to the same store at that instant, THEN the position it received is the last position of its own batch and not the store's head — so the projection resumes from its own work rather than skipping every event another writer landed in between."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/concurrency_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/concurrency_conformance.rs::dcb_concurrency_conformance::append_returns_the_callers_own_last_position"
- id: AC-006
  criterion: "GIVEN a projection runner reads the store continuously while four writers append three-event batches around it, WHEN it samples the stream mid-flight and again after every writer has finished, THEN it never observes a batch half-present, and the post-hoc read on a fresh handle sees every acknowledged event — including under whatever ES-10 mechanism the adapter buys visibility with, so a frontier `head` and a pooled connection left idle-in-transaction cannot make acknowledged events invisible and turn a conformant adapter red."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/concurrency_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/concurrency_conformance.rs::dcb_concurrency_conformance::a_concurrent_reader_never_sees_a_partial_batch (both halves), corroborated by the post-hoc reads inside ::exactly_one_of_n_contenders_commits and ::positions_are_unique_under_concurrent_appends"
- id: AC-007
  criterion: "GIVEN the family is only an instrument if the eight contenders genuinely contend, WHEN one `PostgresFixture` instance hands out `CONTENDERS = 8` handles onto one backing store, THEN each handle holds an independent connection resource — the pool admits at least eight concurrent checkouts, stated in the code as a number derived from `happenstance_testkit::concurrency::CONTENDERS` rather than hard-coded — AND WHEN it is short, the run fails with a message naming pool exhaustion instead of hanging, because CF-33 forbids the watchdog that would otherwise tell the two apart."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/concurrency_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/concurrency_conformance.rs — the pre-flight capacity `#[tokio::test]` sibling to `mod dcb_concurrency_conformance`, asserting configured concurrent-checkout capacity >= happenstance_testkit::concurrency::CONTENDERS"
- id: AC-008
  criterion: "GIVEN each contender drives its future to completion on a bare OS thread with no ambient reactor, WHEN an adapter whose futures need one is put under that harness, THEN the family runs to a real verdict rather than panicking on a missing reactor — AND the fix does not cost the adapter its `SendEventStore` impl, because a `!Send` `EnterGuard` held across an `.await` would demote the store and break every consumer that binds the `Send` flavour. The route taken is recorded, not left to be inferred from the diff."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/concurrency_conformance.rs"
  verifying_test: "crates/happenstance-postgres/src/event_store.rs:177-197 (`store_is_send_and_sync` and its sibling) green under `cargo test -p happenstance-postgres`, plus the five rules of crates/happenstance-postgres/tests/concurrency_conformance.rs::dcb_concurrency_conformance reaching a verdict, plus the recorded route in .bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-concurrency-family/_evidence.md"
- id: AC-009
  criterion: "GIVEN AC-003 of the project requires that write concurrency was retained and not traded away, and GIVEN the family's own conformant control is `LockedStore` — one mutex across the whole append — so a green run proves nothing about concurrency on its own, WHEN the author of ADR-0024 opens this story's record, THEN they find server-side, clock-free evidence that at least two append transactions were in flight simultaneously (or an honest statement that overlap was not observed, reported as evidence rather than proof), the shipped append path's SQL quoted with the claim that it contains no store-wide serialisation point, and the run's parameters — which five rules ran, at `CONTENDERS = 8`, against which pinned Postgres minor — together with the two structural observations this story surfaced and does not settle."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/concurrency_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/write_overlap_witness.rs (clock-free, live-Postgres job), plus the record at .bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-concurrency-family/_evidence.md"
```
