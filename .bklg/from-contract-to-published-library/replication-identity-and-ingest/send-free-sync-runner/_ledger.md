---
item: HS-S0109
stage: implement
created: 2026-08-12T13:47:50.038Z
updated: 2026-08-12T13:47:50.038Z
---

# Acceptance ledger — The runner keeps the constrained runtime its runtime

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Per the project testing brief's *Merge-gate commands* (`…/_decomposition.md:775-800`) and
`.redkiln/config.yaml`'s `require_ledger: true`, each row's evidence names **which** command
produced it: a green `cargo xtask ci --fast` proves the gate passed, never on its own that AC-003's
mid-chain spawn is the thing that ran.

```yaml
- id: AC-001
  criterion: "Adding a peer is a configuration change, not a migration. GIVEN an application author running one store against one peer, WHEN they add a second peer, THEN the change is a value handed to the runner — no type on `SyncPeer`/`IngestStore`/`EventStore` changes, no generic bound in the author's own code changes, and the first peer's exchange behaviour is identical before and after. Fan-out, ordering between peers and reconciliation live in the runner (SY-8), never on the port."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/runner_fans_out.rs::second_peer_is_a_configuration_change"

- id: AC-002
  criterion: "The runner admits the constrained runtime it was built for. GIVEN a constrained-runtime developer whose store, peer and error types are all `!Send`, WHEN they instantiate the runner and drive an exchange, THEN it compiles and runs with no `Send` obligation anywhere on the drive path: no `SendEventStore`, `SendSyncPeer` or `SendIngestStore` in any bound on the runner or the ingest path, no `Resume: Send`, no `+ Send + Sync` added to any error on the way past, and no module importing both names of a flavour pair."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/send_free_bounds.rs::the_drive_path_names_no_send_flavour_and_no_send_bound"

- id: AC-003
  criterion: "The `!Send` node is in the middle of the fleet, and it really runs there. GIVEN the Kestrel arrangement — A—B—C where A and C never communicate and B is a Durable-Object-shaped node whose *peer and store are both held behind an `Rc`* — WHEN the runner drives B in both directions inside a real local spawn (`LocalSet` or a current-thread runtime) holding the resume token across an await, THEN both edges exchange and B's receiver holds A's and C's events keyed by `EventId`. A `fn assert_send<T: Send>()` on a concrete future, or a `#[tokio::test]` that never spawns, does not satisfy this criterion."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/mid_chain_not_send.rs::drives_an_rc_held_middle_node_inside_a_real_local_spawn"

- id: AC-004
  criterion: "The thread-capable deployment loses nothing for it. GIVEN an application author on a multi-thread tokio runtime with `Send` types throughout, WHEN they use the spawning entry point to run a peer edge in a real `tokio::spawn` and carry the advanced resume token back out of the task, THEN it compiles and the token is usable in the next exchange — and the only extra bound written to make that possible is on the token (`P::Resume: Send`, plus whatever the caller hands the task), never a port bound swapped for its `Send` flavour. Deleting the spawning entry point leaves the runner fully usable."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/spawned_edge_carries_the_token_out.rs::spawns_from_generic_and_carries_the_resume_token_out"

- id: AC-005
  criterion: "The `wasm32` gate compiles a runner, not an empty crate. GIVEN the adapter author who will write the Cloudflare peer at HS-S0111, WHEN they run `cargo xtask wasm` (and therefore `cargo xtask ci --fast`) on this tree, THEN the sync conformance harness that step checks contains a real *instantiation* of the runner with a `!Send` fixture peer holding a `!Send` store behind an `Rc` — so a `Send`-bound runner fails to compile here, in a crate this project owns, rather than being discovered in a crate HS-S0111 does not own. A crate build alone does not satisfy this criterion: a generic function's `Send` bounds are satisfied at instantiation."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs (the wasm32 step list) and crates/happenstance-sync-testkit/src/"
  verifying_test: "crates/happenstance-sync-testkit/tests/runner_instantiation.rs, plus `cargo xtask wasm`'s check of the sync conformance harness"

- id: AC-006
  criterion: "A fleet resumes after the process that ran it went away. GIVEN an edge deployment whose normal termination path is the handle disappearing — a Worker cancelled mid-flight, a Durable Object evicted, a tablet losing signal — WHEN the runner completes an exchange, hands the per-peer resume token back out to its caller, the peer handle is dropped and reconstructed, and the token is supplied to a fresh exchange, THEN replication continues with no gap and no duplicate. The token is owned by the caller, opaque, `Clone`, and never inspected or serialised by the runner."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/resume_survives_a_dropped_handle.rs::runner_hands_the_token_out_and_resumes_with_no_gap_and_no_duplicate"

- id: AC-007
  criterion: "\"Caught up\" does not mean \"start over\". GIVEN a fleet that has replicated everything currently available, WHEN a peer's `pull` returns an empty batch, THEN the runner still returns the peer's token to its caller, and the next exchange resumes from it rather than from `None` — an empty exchange delivers nothing and re-delivers nothing."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/resume_survives_a_dropped_handle.rs::an_empty_pull_returns_the_token_to_use_next"

- id: AC-008
  criterion: "Two stores that have both committed converge; nothing is held back. GIVEN a receiver whose local state disagrees with an incoming group, WHEN the runner drives that exchange, THEN the events are ingested — the runner holds no quarantine, no parking area, no per-peer conflict state and no content-triggered back-pressure. A transport failure may be retried and is surfaced to the caller with the peer it belongs to; disagreement with content is not a state this runner may hold."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/no_rejection_above_the_port.rs::transport_failure_retries_but_content_is_never_parked"

- id: AC-009
  criterion: "One node is a hub and a spoke at the same time, and the runner never asks which. GIVEN one adapter type deployed as a spoke to an estate store and a hub to many tablets, WHEN it is wired into the runner, THEN it is configured per edge — the runner exposes no role parameter, no `is_hub`, and no two peer collections typed by role — and nothing in the runner's shape prevents HS-S0110 from later attaching a different merge rule to each direction of one edge (SY-10)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/roles_are_edges.rs::one_type_drives_both_edges_without_a_role_parameter"

- id: AC-010
  criterion: "A reader of the docs can run the thing without reading the source. GIVEN an application author meeting the runner for the first time on docs.rs, WHEN they read its rustdoc, THEN every new public item is documented, every fallible public function carries an `# Errors` section naming the conditions rather than the error type, and a compiled doctest shows one exchange over two peers end to end — and that doctest is executed by the gate rather than merely present. Any sentence in `lib.rs`/`peer.rs` this PR makes false is corrected, and any finding it carries is moved into the ADR that consumed it before the paragraph is deleted."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "`cargo test -p happenstance-sync --doc` (the two-peer exchange doctest) plus `cargo xtask ci --fast`'s docs step with `-D warnings`"
```
