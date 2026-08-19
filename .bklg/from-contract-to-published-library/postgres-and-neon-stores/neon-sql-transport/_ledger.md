---
item: "HS-S0067"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — A real SqlTransport, host and wasm32

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two rows below are satisfied by **recorded command output** rather than by a test function — AC-007's
`wasm32 feature powerset` result and AC-008's `cargo deny check` result. Both steps are `OPTIONAL` in
`xtask/src/main.rs` and are dropped by `cargo xtask ci --fast` (`:840-846`), so their evidence must
come from a full `cargo xtask ci` run and must be pasted literally — including a `skipped`, which is
itself the finding. AC-001 and AC-002 are satisfied only by a run against a **real** Neon `/sql`
endpoint; this story wires no CI job, so that run is a recorded one-off.

```yaml
- id: AC-001
  criterion: "The adapter author's transport reaches their own Neon branch. GIVEN an adapter author holding a Neon /sql endpoint URL and a credential, WHEN they construct the host transport from those two values and call round_trip with a one-statement SqlRequest the endpoint can answer with no schema at all (SELECT 1), THEN they get Ok(HttpResponse) with a 2xx status and a body that deserialises through wire::ResponseBody into a ResultSet holding the row — the first time anything in this workspace has spoken to a real endpoint."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/lib.rs"
  verifying_test: "crates/happenstance-neon/tests/live_sql_round_trip.rs::select_one_decodes_through_result_set"

- id: AC-002
  criterion: "A rejected statement reaches the author as a rejection, not as \"the network failed\". GIVEN the same transport, WHEN round_trip sends a statement the endpoint refuses (a select against a table that does not exist), THEN it returns Ok(HttpResponse) carrying the non-2xx status and a body that deserialises as NeonSqlError with a code, and Self::Error is NOT constructed — so decode_append_response can still tell AppendError::ConditionViolated from AppendError::Store."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/lib.rs"
  verifying_test: "crates/happenstance-neon/tests/live_sql_round_trip.rs::rejected_statement_returns_ok_with_the_sql_error_body"

- id: AC-003
  criterion: "The isolation the author configured is the isolation the endpoint runs. GIVEN a SqlRequest of two statements at IsolationLevel::Serializable, WHEN the transport turns it into an HTTP request, THEN the bytes sent are exactly SqlRequest::body(), the headers sent are exactly SqlRequest::headers() plus Content-Type and credentials, no isolation key appears anywhere in the JSON body; and GIVEN a one-statement request, THEN neither Neon-Batch-* header is sent."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/lib.rs"
  verifying_test: "crates/happenstance-neon/src/transport.rs::tests::request_parts_carry_the_batch_headers, ::tests::single_statement_request_sends_no_batch_headers, ::tests::isolation_never_enters_the_body"

- id: AC-004
  criterion: "An oversize answer is refused as an oversize answer. GIVEN a response whose body exceeds MAX_RESPONSE_BYTES, WHEN the transport handles it, THEN it returns its own distinct over-ceiling error variant — one a caller can separate from DNS, TLS, a rejected fetch or a timeout — rather than folding it in with them, and it does not buffer the whole body before deciding."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/lib.rs"
  verifying_test: "crates/happenstance-neon/src/transport.rs::tests::refuses_a_body_over_max_response_bytes"

- id: AC-005
  criterion: "The edge developer's target survives the arrival of a host client. GIVEN a developer targeting wasm32-unknown-unknown, WHEN cargo xtask wasm runs, THEN the step named \"wasm32 build of the Neon adapter\" is green under that exact name, and what compiles under it is the fetch implementation — the host client is absent from the wasm32 dependency graph entirely, not merely unused."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/lib.rs"
  verifying_test: "cargo xtask wasm — step \"wasm32 build of the Neon adapter\" (xtask/src/main.rs:264-282, selected by name at :783-791); corroborated by cargo tree --target wasm32-unknown-unknown -p happenstance-neon"

- id: AC-006
  criterion: "The !Send promise the crate is written on still holds. GIVEN a caller on a single-threaded runtime, WHEN they drive NeonEventStore through the bare EventStore flavour inside a tokio::task::LocalSet, holding the future across an await, THEN it compiles and runs; and no Send bound appears on either implementation, on any helper either calls, or on any boxed future, and NeonEventStore gains no SendEventStore impl."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/lib.rs"
  verifying_test: "crates/happenstance-neon/tests/not_send_is_preserved.rs::drives_on_a_local_set_through_a_generic_bound"

- id: AC-007
  criterion: "What a consumer gets when they cargo add happenstance-neon is a decision somebody made. GIVEN a consumer reading the manifest and the transport module rustdoc, WHEN they look for the HTTP client, THEN they find it in exactly one of [dependencies], a target-scoped off-by-default feature, or [dev-dependencies], with the published-surface consequence stated in prose; the version pin is in [workspace.dependencies]; the chosen shape is reachable from crates/happenstance-neon/tests/; and the wasm32 feature-powerset step's actual result — green or skipped — is recorded rather than assumed."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/lib.rs"
  verifying_test: "cargo xtask ci — step \"wasm32 feature powerset\" (xtask/src/main.rs:558-593), literal output; plus crates/happenstance-neon/tests/live_sql_round_trip.rs compiling under the chosen manifest configuration as the reachability proof"

- id: AC-008
  criterion: "The licence gate is not weakened to make this land. GIVEN deny.toml byte-identical to main, WHEN cargo deny check runs over the tree with the chosen client in it, THEN it exits zero and the output is recorded; and if the candidate cannot clear the existing allowlist, the client changes, not the allowlist — the rejection text is attached and escalated instead."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/lib.rs"
  verifying_test: "cargo deny check (xtask/src/main.rs:596-600, OPTIONAL — run the full cargo xtask ci, not --fast); plus an empty git diff --stat deny.toml"

- id: AC-009
  criterion: "Every other project's gate stays runnable on a laptop with no network. GIVEN a clean checkout with no Neon credential, no Docker and no network, WHEN cargo xtask affected --base main and the full cargo xtask ci run, THEN both exit zero, the live round trip does not execute, and the mechanism that skipped it is whole-invocation — no #[cfg] removed an assertion from a compiled test binary."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/lib.rs"
  verifying_test: "cargo xtask affected --base main and cargo xtask ci, both run offline with no credential; plus review of the gating mechanism in crates/happenstance-neon/tests/live_sql_round_trip.rs against DR-5"

- id: AC-010
  criterion: "The next reader is told the truth about what this crate now owns. GIVEN a reader opening happenstance-neon's docs after this PR, WHEN they read the crate-level and transport module rustdoc, THEN the \"the crate owns no HTTP client… cannot demonstrate that a licence-clean client exists for both targets\" prose has been replaced by what is now true; the three §9.2 answers and their reasons are stated where the decision is met; every new public item carries the crate's rustdoc bar (an # Errors section naming conditions rather than types, #[non_exhaustive] where a caller might match); and the one-statement isolation-header observation is written down and explicitly routed to neon-conflicting-position-verdict, not concluded here."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/lib.rs"
  verifying_test: "cargo doc --workspace --all-features --no-deps under -D warnings (missing_docs and clippy::missing_errors_doc are warn workspace-wide, Cargo.toml:101-117); plus a recorded review read of crates/happenstance-neon/src/lib.rs:91-99 and src/transport.rs:1-15 against the diff"
```
