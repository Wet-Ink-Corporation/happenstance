---
item: HS-S0068
stage: implement
created: "2026-08-12T13:47:06.902Z"
updated: "2026-08-12T13:47:06.902Z"
---

# Acceptance ledger — NeonFixture against a live branch, and its credentialed CI job

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
  criterion: "GIVEN an adapter author who needs to know whether the contract survives a store with no connection, no interactive transaction and no cursor, WHEN the Neon suite is run, THEN the store under test reached a real Neon /sql endpoint over the transport neon-sql-transport delivered — and the fixture cannot be satisfied by a pooled Postgres connection, a local shim returning Neon-shaped JSON, or NullTransport, because the endpoint is read from the job's credentialed environment and a loopback or non-/sql endpoint is rejected at construction."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/neon_conformance.rs; .github/workflows/ci.yml (the `neon` job)"
  verifying_test: "crates/happenstance-neon/tests/neon_fixture.rs::fixture_refuses_a_local_endpoint"

- id: AC-002
  criterion: "GIVEN an adapter author running the suite twice over on one shared CI branch, WHEN the harness constructs two NeonFixture instances, THEN neither instance can see a row the other wrote — isolation bought by a per-instance namespace threaded through NeonConfig into every statement, because there is no connection and no search_path to set."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/neon_conformance.rs"
  verifying_test: "crates/happenstance-neon/tests/neon_fixture.rs::two_fixture_instances_do_not_share_a_namespace"

- id: AC-003
  criterion: "GIVEN a maintainer who pays for a real, billed third-party branch, WHEN a job finishes normally or is killed mid-run, THEN the namespace it created is gone — dropped at teardown in the normal case, and swept by name prefix at the start of the next run in the killed case — and a leftover namespace can never be silently adopted by a later instance instead of a fresh one."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/neon_conformance.rs; .github/workflows/ci.yml (the `neon` job)"
  verifying_test: "crates/happenstance-neon/tests/neon_fixture.rs::namespace_is_dropped_at_teardown; ::a_leftover_namespace_is_never_adopted; ::sweep_removes_only_the_test_prefix"

- id: AC-004
  criterion: "GIVEN an adapter author checking that Neon can meet the one capability the suite treats as a MUST, WHEN the harness asks for a second handle, THEN NeonFixture::SECOND_HANDLE is Capability::SUPPORTED and the handle it hands back is a second client addressing the same branch and the same table set, so the second handle observes what the first wrote — never a decline, which would make two_handles_observe_each_others_appends panic quoting the fixture's own words rather than report a skip."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/neon_conformance.rs"
  verifying_test: "crates/happenstance-neon/tests/neon_fixture.rs::second_handle_addresses_the_same_table_set"

- id: AC-005
  criterion: "GIVEN an adapter author who has been burned by a fixture that \"reopened\" by doing nothing, WHEN they read NeonFixture::REOPEN and its reopen(), THEN the answer is deliberate and non-vacuous: SUPPORTED with a reopen() that discards the client and builds a fresh one against the same endpoint and namespace, so acknowledged_writes_survive_a_reopen re-reads a durable medium rather than an untouched in-process handle — and never an empty reopen() body, and never a SUPPORTED declaration left on the provided body, which panics."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/neon_conformance.rs"
  verifying_test: "crates/happenstance-neon/tests/neon_fixture.rs::reopen_builds_a_fresh_client"

- id: AC-006
  criterion: "GIVEN an adapter author who cannot tell \"we decided\" from \"nobody looked\", WHEN they read NeonFixture::MID_BATCH_FAULT, THEN it is written explicitly in the impl with Neon's own reason — not inherited from the trait's in-memory default, whose text is about a store that \"has no fault to inject\" and would be a fiction here — and if the answer is SUPPORTED, arm_mid_batch_fault is overridden rather than left on the panicking provided body."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/neon_conformance.rs"
  verifying_test: "crates/happenstance-neon/tests/neon_fixture.rs::mid_batch_fault_is_an_explicit_answer"

- id: AC-007
  criterion: "GIVEN a constrained-runtime developer deciding whether Neon can carry their payloads, WHEN they read the three ceilings, THEN each is a number derived from MAX_RESPONSE_BYTES after the hex bytea doubling and measured against the live endpoint — not the 64 MiB constant copied, and not a small number chosen \"to be safe\" — with the derivation written as rustdoc beside the constants it is derived from."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/neon_conformance.rs"
  verifying_test: "crates/happenstance-neon/tests/neon_fixture.rs::ceilings_are_derived_not_copied; ::stated_ceiling_is_where_the_endpoint_actually_refuses"

- id: AC-008
  criterion: "GIVEN an application author reading whether Neon clears the minima every store must clear, WHEN they compare the stated ceilings against VT-21/VT-22/VT-24, THEN either all three clear MIN_SUPPORTED_EVENT_DATA_LEN (65,536), MIN_SUPPORTED_TAGS_PER_EVENT (64) and MIN_SUPPORTED_EVENTS_PER_BATCH (128), or the shortfall is recorded as a finding with its measured number in this story's ledger evidence — never a number nudged upward to make store_accepts_the_guaranteed_minimum_payload green."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/neon_conformance.rs"
  verifying_test: "crates/happenstance-neon/tests/neon_fixture.rs::stated_ceilings_are_compared_against_the_guaranteed_minima"

- id: AC-009
  criterion: "GIVEN an adapter author reading a red CI log for a job they cannot reproduce locally without a credential they do not hold, WHEN connect() fails, THEN the panic tells them it is the environment and not the adapter — naming the endpoint host, the branch, the namespace, and whether a credential was present — and the credential's value appears nowhere in the message or the log."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/neon_conformance.rs"
  verifying_test: "crates/happenstance-neon/tests/neon_fixture.rs::connect_failure_names_the_environment; ::connect_failure_never_prints_the_credential"

- id: AC-010
  criterion: "GIVEN a contributor on a clean checkout with no Neon credential and no network, WHEN they run cargo xtask ci before saying \"done\", THEN it exits zero — the new tests target and the macro's full expansion are compiled by clippy and by the doc build, so the mount cannot rot, and the credentialed tests are reported as ignored rather than being absent from the binary."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/neon_conformance.rs"
  verifying_test: "cargo xtask ci on a credential-less checkout (xtask/src/main.rs:105,116,143); cargo test --locked --workspace --all-features -p happenstance-neon reporting the gated tests as ignored"

- id: AC-011
  criterion: "GIVEN a maintainer merging a change that could break Neon, WHEN CI runs, THEN a neon job — a sibling of gate, backlog, wasm-conformance, msrv, semver and advisories, not a step inside gate — runs the suite against the live branch using an Actions secret, and its behaviour where that secret is unavailable is stated in the workflow: red rather than a quiet skip where the secret should be present, and not scheduled at all on a fork pull request, which is merged through a branch where it does run. It carries no continue-on-error and is never made non-blocking without a recorded decision."
  satisfied: false
  evidence: ""
  mount_point: ".github/workflows/ci.yml (the `neon` job, sibling of `gate` at :31)"
  verifying_test: "crates/happenstance-neon/tests/neon_fixture.rs::the_neon_job_is_not_continue_on_error"

- id: AC-012
  criterion: "GIVEN an evaluator reading the CI log as public evidence that Neon's limits are real, WHEN the neon job goes green, THEN it demonstrably executed the expected rules rather than nothing — the expected test names are asserted out of cargo test -- --list before the run, and the run's executed count is nonzero — and the log carries --show-output, so every declined capability's stated reason is readable rather than swallowed by the harness's default capture."
  satisfied: false
  evidence: ""
  mount_point: ".github/workflows/ci.yml (the `neon` job); crates/happenstance-neon/tests/neon_conformance.rs"
  verifying_test: "the `neon` job's --list name assertion (shape: xtask/src/proof.rs:15-21), then cargo test -p happenstance-neon --all-features --test neon_conformance -- --ignored --show-output"
```
