---
item: HS-S0061
stage: implement
created: 2026-08-12
updated: 2026-08-12
---

# Acceptance ledger — Migration 1, the live Postgres fixture, and the CI job that runs it

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes the implementer needs before flipping anything:

1. **No row here is satisfied by a conformance rule outcome.** `append`, `head` and
   `contains_event_id` stay `todo!()` in this PR. Evidence is either a compile-time proof from the
   default gate (`cargo clippy --workspace --all-targets --all-features -- -D warnings`) or a
   story-local gated test reaching the database through `PostgresEventStore::pool()`. See
   `spec.md` *Clarifications resolved during spec*, item 1.
2. **AC-008, AC-009 and AC-010 are the rows that carry project AC-011.** They are also the three
   whose evidence is a transcript or a workflow run rather than a test id — cite the run, not the
   intention.

```yaml
- id: AC-001
  criterion: "GIVEN an adapter author with an empty Postgres database and a checkout of this crate, WHEN a `PostgresFixture` instance brings its backing store up, THEN the event table exists — created from a versioned SQL source under `crates/happenstance-postgres/migrations/`, not from prose in a doc comment — carrying `position`, `event_type`, `data`, `metadata`, `tags`, `origin_store`, `origin_position` and `recorded_at` with the shapes the *Data and migrations* table fixes, plus the tag index and the `(event_type, position)` index."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/postgres_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/postgres_conformance.rs::migration_1_creates_the_settled_column_set"
- id: AC-002
  criterion: "GIVEN ADR-0024 is undecided and is a deliverable of a *later* slice, WHEN the adapter author reads migration 1, THEN nothing in it has answered the position-visibility question by accident: `position` is `bigint` with **no** column default and is not `serial` / `bigserial` / `GENERATED … AS IDENTITY`, and the table carries **no** visibility-mechanism column (`xid8` or otherwise)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/postgres_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/postgres_conformance.rs::migration_1_does_not_preempt_adr_0024"
- id: AC-003
  criterion: "GIVEN a suite run that constructs more than one fixture instance in one process, WHEN each instance's handle writes to the event table, THEN neither instance can observe the other's rows — one `PostgresFixture` instance is one isolated backing store, and the isolation scheme (schema-per-instance or database-per-instance) is recorded with its reason."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/postgres_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/postgres_conformance.rs::two_fixture_instances_are_two_backing_stores"
- id: AC-004
  criterion: "GIVEN `SECOND_HANDLE` is the one MUST among the capabilities and declining it makes `two_handles_observe_each_others_appends` **panic** quoting the fixture's own words, WHEN the adapter author opens two handles from one `PostgresFixture` instance, THEN both address the same backing store and each observes the other's writes — and each handle owns its own refcount rather than borrowing the fixture's lifetime."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/postgres_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/postgres_conformance.rs::two_handles_from_one_instance_share_a_backing_store"
- id: AC-005
  criterion: "GIVEN a capability constant is an *answer* and a silent default is the failure mode, WHEN the adapter author reads `PostgresFixture`'s constants, THEN `SECOND_HANDLE`, `REOPEN` and `MID_BATCH_FAULT` each carry a deliberate value with a non-empty, Postgres-specific reason where declined, `MAX_EVENT_DATA_LEN` / `MAX_TAGS_PER_EVENT` / `MAX_EVENTS_PER_BATCH` are stated as honest `Option<usize>` facts, and CF-40's contested ownership is consumed rather than settled here."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/postgres_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/postgres_conformance.rs::capability_constants_are_answered_not_defaulted"
- id: AC-006
  criterion: "GIVEN the concurrency family starts `CONTENDERS = 8` contenders and a fixture whose pool is smaller than that **deadlocks rather than fails**, with no watchdog to tell the two apart (CF-33), WHEN the adapter author points `event_store_concurrency_conformance!` at `PostgresFixture`, THEN it type-checks with `F::Store: Send` provable at the call site, and one fixture instance genuinely hands out `CONTENDERS` simultaneous live handles onto one backing store without deadlocking."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/postgres_conformance.rs"
  verifying_test: "crates/happenstance-postgres/tests/postgres_conformance.rs::contenders_handles_open_concurrently_without_deadlock (runtime) + the `event_store_concurrency_conformance!(PostgresFixture::…)` invocation compiled by `cargo clippy --workspace --all-targets --all-features -- -D warnings`"
- id: AC-007
  criterion: "GIVEN \"a fixture no macro is invoked with is not delivered,\" WHEN the adapter author looks for where the Postgres suite runs, THEN `crates/happenstance-postgres/tests/postgres_conformance.rs` exists and invokes `event_store_conformance!(PostgresFixture::…)` and `event_store_concurrency_conformance!(…)` over their **full expansion** — no hand-picked subset — so that every rule the suite defines is present in the binary and reports pass, fail or a declared skip."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/postgres_conformance.rs"
  verifying_test: "the live-Postgres CI job's `cargo test -p happenstance-postgres --all-features -- --ignored --list` assertion in .github/workflows/ci.yml, modelled on xtask/src/proof.rs:199-217"
- id: AC-008
  criterion: "GIVEN every other project in this initiative is gated by `cargo xtask affected` and `cargo xtask ci --fast` (`.redkiln/config.yaml:40,55`), WHEN a maintainer of any of them runs the default gate on a laptop with the Docker daemon stopped and the network unplugged, THEN it is green — including the `tests` step's `cargo test --locked --workspace --all-features -- --show-output` — and the new tests target still **compiles and lints** rather than being cfg'd out of existence."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs REQUIRED steps, run via `cargo xtask ci --fast`"
  verifying_test: "`cargo xtask ci --fast` executed with the Docker daemon stopped and networking disabled; transcript in implementation-report.md, showing the gated tests as `ignored` rather than `0 tests`"
- id: AC-009
  criterion: "GIVEN live infrastructure must not enter `xtask/src/main.rs`'s `REQUIRED` array, WHEN CI runs on this PR, THEN a new job in `.github/workflows/ci.yml` sits **beside** `gate` (`:31`), `backlog` (`:102`), `wasm-conformance` (`:204`), `msrv` (`:241`), `semver` (`:279`) and `advisories` (`:325`) — not as a step inside `gate` — starts a `testcontainers` Postgres pinned to a specific minor, and runs the gated tests with `--show-output` so every declined capability's reason lands in the log."
  satisfied: false
  evidence: ""
  mount_point: ".github/workflows/ci.yml (new job, sibling of `gate` at :31)"
  verifying_test: "the new job's own run on this PR (workflow run URL), plus a reviewed diff showing xtask/src/main.rs's REQUIRED array and wasm_steps() (:784-790,816-823) untouched"
- id: AC-010
  criterion: "GIVEN the same flag that makes the default gate green with no server makes a misconfigured live job green with no tests, WHEN the container fails to start or the gating flag is wrong, THEN the live job **fails** rather than reporting success on `running 0 tests` — it cannot be green without having executed the gated tests."
  satisfied: false
  evidence: ""
  mount_point: ".github/workflows/ci.yml (new job, sibling of `gate` at :31)"
  verifying_test: "the job's nonzero executed-test-count assertion step (modelled on xtask/src/proof.rs:199-231), plus a deliberate negative exercise — the job's command run with the gate flag off, shown failing — transcript in implementation-report.md"
```
