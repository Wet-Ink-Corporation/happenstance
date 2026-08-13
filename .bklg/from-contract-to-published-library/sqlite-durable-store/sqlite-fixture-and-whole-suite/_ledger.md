---
item: "HS-S0040"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — SqliteFixture, and the first green conformance run against a file

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three rows — AC-002, AC-003 and AC-004 — cannot be discharged by a green rule alone: a fixture that
shares one directory, clones one store, or no-ops `reopen()` produces a green run for each. Their
evidence must cite the fixture's own body as well as the test id (`spec.md`, *Clarifications* 3).

```yaml
- id: AC-001
  criterion: "The bar is cleared whole, and the run says so. GIVEN an adapter author who has been told all phase that \"an adapter that compiles is not an adapter\", WHEN they run `cargo test -p happenstance-sqlite --test conformance -- --nocapture` against a `SqliteFixture` backed by a real file on disk, THEN `event_store_conformance!` is green and every rule named in `for_each_event_store_rule!` appears in that output as either `Ran` or `Skipped` carrying this fixture's own stated reason — none is absent from the binary, none is `#[ignore]`d, and none was removed to make the run green."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/conformance.rs"
  verifying_test: "crates/happenstance-sqlite/tests/conformance.rs::dcb_conformance (whole module; `cargo test -p happenstance-sqlite --test conformance -- --nocapture`, audited against crates/happenstance-testkit/src/registry.rs:94-218)"
- id: AC-002
  criterion: "Two fixtures are two stores, so one rule's data can never explain another's pass. GIVEN an adapter author who has just been handed a green suite and wants to know whether it means anything, WHEN two `SqliteFixture` instances are constructed in the same process and each appends through its own handle, THEN neither observes the other's events — because `SqliteFixture::new()` mints one fresh temp file per instance and two instances get two paths, which is the fixture-contract mistake `contract.rs:17-23` says no testkit meta-test can catch for an adapter."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/conformance.rs"
  verifying_test: "crates/happenstance-sqlite/tests/conformance.rs::dcb_conformance::two_fixture_instances_observe_none_of_each_others_appends, plus review of `SqliteFixture::new` in the same file"
- id: AC-003
  criterion: "The second handle is a second connection, not a second reference. GIVEN a library consumer who intends to run this store from more than one place in their process, WHEN the fixture declares `SECOND_HANDLE = Capability::SUPPORTED` and a rule calls `connect()` twice on one instance, THEN it receives two distinct `rusqlite::Connection`s onto one file — `SqliteEventStore::open(path)`, never an `Arc`/`Rc` clone of one in-process store and never `open_in_memory()` — and an append through one is visible to the other, filling the handle-multiplicity far end `RUNBOOK.md:691` records as empty."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/conformance.rs"
  verifying_test: "crates/happenstance-sqlite/tests/conformance.rs::dcb_conformance::two_handles_observe_each_others_appends and ::head_advances_across_two_handles, plus review of `SqliteFixture::connect` in the same file"
- id: AC-004
  criterion: "An acknowledged write outlives the process that acknowledged it. GIVEN a consumer who is choosing this adapter because it is durable, WHEN the fixture declares `REOPEN = Capability::SUPPORTED` and `reopen()` drops every live connection while leaving the file and its WAL intact, THEN a fresh `connect()` afterwards still sees both events, the exact `SequencePosition` `append` returned, the same `recorded_at` read back rather than re-stamped, and no reissued `EventId` — filling the durability far end `RUNBOOK.md:692` records as empty."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/conformance.rs"
  verifying_test: "crates/happenstance-sqlite/tests/conformance.rs::dcb_conformance::acknowledged_writes_survive_a_reopen, ::recorded_time_survives_a_reopen and ::reopened_store_does_not_reissue_an_event_id, plus review of `SqliteFixture::reopen` in the same file"
- id: AC-005
  criterion: "The store's ceilings are stated numbers, so the rule that needs them runs. GIVEN an adapter author who must know what this store refuses before they ship on it, WHEN the fixture states real values for `MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT` and `MAX_EVENTS_PER_BATCH` — each at or above its VT floor of 65,536 bytes / 64 tags / 128 events and each small enough that the suite can allocate a payload of exactly that size — THEN `append_reports_exceeded_store_limits` reports `Ran`, accepting at the limit and returning `AppendError::ExceedsStoreLimit` one byte past it, rather than reporting the `NO_STORE_LIMITS` / `NO_CEILING_REASON` skip that a `None` leaves behind and that reads as a pass to anyone with only the exit code."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/conformance.rs"
  verifying_test: "crates/happenstance-sqlite/tests/conformance.rs::dcb_conformance::append_reports_exceeded_store_limits observed as Ran (not Skipped), with ::store_accepts_the_guaranteed_minimum_payload, ::store_accepts_the_guaranteed_minimum_tag_count and ::store_accepts_the_guaranteed_minimum_batch_size green"
- id: AC-006
  criterion: "The one capability this adapter declines says so in its own words, and the rules it gates stay visible. GIVEN an adapter author reading this crate's CI log to learn what it does not do, WHEN `MID_BATCH_FAULT` is declined with a reason written for this adapter — that it supplies the reopen far end and not the fault far end, which is ES-35's live residual falsifier — rather than inherited from the trait's generic default, THEN `append_is_atomic_under_a_mid_batch_fault` and `arming_a_mid_batch_fault_makes_the_append_fail` both still appear in the binary and both print a `SKIP` line quoting that sentence, which is CF-18's whole argument."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/conformance.rs"
  verifying_test: "crates/happenstance-sqlite/tests/conformance.rs::dcb_conformance::append_is_atomic_under_a_mid_batch_fault and ::arming_a_mid_batch_fault_makes_the_append_fail, observed under `-- --nocapture` as Skipped with this fixture's own reason (not contract.rs:207-211's default)"
- id: AC-007
  criterion: "It went green by passing, and the repository's own gate is what says so. GIVEN a reviewer who has been told this is the first green run against durable storage, WHEN they check how it went green, THEN the target contains no `#[ignore]`, no timeout, watchdog or retry loop around any rule (CF-33 keeps a clock out of the suite), nothing under `crates/happenstance-testkit/` was added, reworded or reordered, no SQL body was edited to make a rule pass, `crates/happenstance-sqlite/tests/shapes.rs` still passes unchanged in intent, and the new target is reached by `cargo xtask affected --base main` with no script edit because `tests/` is auto-discovered."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/conformance.rs"
  verifying_test: "crates/happenstance-sqlite/tests/shapes.rs (`cargo test -p happenstance-sqlite --test shapes`) green, `cargo xtask affected --base main` green, and a clean `git diff --stat crates/happenstance-testkit/`"
- id: AC-008
  criterion: "The ceiling numbers leave as documented facts, and the clause question they touch leaves as still open. GIVEN a planner who must not have CF-40's ownership contradiction silently resolved by an implementer choosing three integers, WHEN the three ceilings are recorded in the ledger as facts about this adapter with the reasoning for each number, THEN `.kb/open-questions/cf-40-fixture-limits-ownership.md` is still an open question atom — untouched in status, escalated to the ADR queue, and cited from this story's evidence — because this story needs the capability and gets no say in which ADR owns the clause."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/conformance.rs"
  verifying_test: "Review gate: the three numbers and their rationale recorded in this ledger's AC-005 and AC-008 evidence; `.kb/open-questions/cf-40-fixture-limits-ownership.md` unchanged (empty `git diff` for that path) and cited; `redkiln validate --kb` clean"
```
