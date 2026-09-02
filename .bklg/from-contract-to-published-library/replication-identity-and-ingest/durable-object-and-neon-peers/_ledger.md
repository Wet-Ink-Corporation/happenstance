---
item: "HS-S0111"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — One suite, three peers, two of them genuinely unlike

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

The mount points are the two `sync_peer_conformance!` invocation sites named in the spec's
*Integration contract* — `crates/happenstance-cloudflare/tests/sync_peer_conformance.rs` and
`crates/happenstance-neon/tests/sync_peer_conformance.rs`. Neither `tests/` directory exists today;
creating it *is* the mount, and a peer impl that compiles without one is not this story delivered
(`CLAUDE.md`, *The rule that matters*). The gate-side half of the mount already exists from
`gate-mounts-for-the-sync-suite`.

Two rows deserve a note before anyone flips them. **AC-001 passes on a negative outcome**: an
environment that is genuinely absent, confirmed first and recorded with a raised blocker and a
declined capability, satisfies AC-001 and fails AC-004's live half — the two are deliberately
separable so that "could not reach it" and "did not say so" are different results. **AC-005's
evidence is run output**, not a source line: cite the captured `Skipped { capability, reason }` lines
alongside the file that declares them.

```yaml
- id: AC-001
  criterion: "P4 is told what the infrastructure actually is, before anyone builds on a guess. GIVEN the Durable Object and Neon environments are HS-P0013's and HS-P0014's and this project carries no deployment brief, WHEN the story starts — first task, before a line of peer code — THEN both environments are confirmed reachable and the outcome is recorded either way in this story's own directory, and an environment that is absent produces (a) a raised blocker naming the owning project and (b) a fixture that declines the affected capability with the real reason, never (c) a mock, a `#[cfg]`-out, a feature flag that no-ops, or a leg quietly dropped from the run."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/tests/sync_peer_conformance.rs and crates/happenstance-neon/tests/sync_peer_conformance.rs"
  verifying_test: ".bklg/from-contract-to-published-library/replication-identity-and-ingest/durable-object-and-neon-peers/_environments.md (one row per peer, naming what was probed and what answered)"

- id: AC-002
  criterion: "P3 finds the edge runtime represented by something that could not have been faked. GIVEN `happenstance-cloudflare`, which forbids `unsafe` and whose error carries `Rc<str>` so its `!Send`-ness cannot disappear under a `cfg` (`crates/happenstance-cloudflare/src/lib.rs`, Findings §1), WHEN `SyncPeer` and `IngestStore` are implemented inside that crate on its own store handle, THEN the impls are the bare flavour, no `Send` bound appears anywhere on the path, no `#[async_trait]` is introduced, `crates/happenstance-core/**` is untouched, and the peer is a foreign trait on a local type — the one arrangement coherence permits and no third crate can supply on its behalf."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/tests/sync_peer_conformance.rs"
  verifying_test: "cargo xtask wasm (wasm32 build of happenstance-cloudflare + wasm32 check of the sync harness); the compile_fail,E0117 doctest in crates/happenstance-sync/src/ingest.rs; empty crates/happenstance-core/** diff"

- id: AC-003
  criterion: "P2 learns the one-round-trip promise is measured, not asserted. GIVEN `happenstance-neon`, which has no connection, no interactive transaction, no cursor and a hard 64 MiB response cap (`crates/happenstance-neon/src/lib.rs`, capability table; `transport.rs:54`), WHEN the suite runs against its peer through a fixture transport that counts its own round trips and panics on a second call within one operation, THEN every port method completes in exactly one round trip with no state held between calls — multi-statement work going through the non-interactive batch form (`SqlRequest::batch`, one `BEGIN`/`COMMIT`, server-side, one request) rather than two requests — and the resume token survives the peer handle being dropped and reconstructed."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/sync_peer_conformance.rs"
  verifying_test: "the counting transport in crates/happenstance-neon/tests/ (asserts its own round-trip count, panics on a second call within one operation); resume_survives_a_dropped_peer_handle green against the Neon fixture"

- id: AC-004
  criterion: "P4 reads one CI log and sees three peer names, two of them structurally nothing like the third. GIVEN `sync_peer_conformance!` and its rule registry already in the tree, WHEN the story is done, THEN that macro is invoked at three sites — the existing memory one plus `crates/happenstance-cloudflare/tests/sync_peer_conformance.rs` and `crates/happenstance-neon/tests/sync_peer_conformance.rs` — over one rule enumeration through `for_each_sync_peer_rule!`, and every rule appears in every peer's output as either `Ran` or `Skipped { capability, reason }`: no rule is absent from any of the three, and `ingest_never_rejects` (SY-1), `compensation_is_atomic_with_the_losing_event` (SY-2) and `wire_condition_with_after_is_refused` (SY-6) are green against something other than the oracle for the first time."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/tests/sync_peer_conformance.rs and crates/happenstance-neon/tests/sync_peer_conformance.rs"
  verifying_test: "the three sync_peer_conformance! invocation sites (shape: crates/happenstance-testkit/tests/memory_conformance.rs:27), with the run output compared as a rule-name set difference across the three peers"

- id: AC-005
  criterion: "P2 is told what a real constraint cost, instead of being shown a shorter list. GIVEN a peer that genuinely cannot do something — an unavailable environment, or a transport limit that no implementation can work around — WHEN the suite runs, THEN the fixture declines that `Capability` with a reason naming why rather than that, the rule still executes and still reports, and the reason lands in the CI log where a reviewer and an adapter author can both read it. `Capability::declined(\"\")` is a compile error and a dropped `RuleOutcome` fails a `-D warnings` build, so neither an empty reason nor a swallowed skip can ship."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/tests/sync_peer_conformance.rs and crates/happenstance-neon/tests/sync_peer_conformance.rs"
  verifying_test: "the two peer fixtures' Capability constants consuming crates/happenstance-testkit/src/contract.rs:368-433 and :473-483, plus the captured run output containing a Skipped { capability, reason } line per declined capability"

- id: AC-006
  criterion: "P1/P4 watch a payload cross a real network and come back byte-for-byte. GIVEN an event whose `data` and `metadata` are bytes that are neither valid UTF-8 nor valid JSON, WHEN it crosses a real transport to either networked peer and is read back, THEN the `Bytes` compare equal to the origin's and no path — peer impl, fixture, transport assertion or test — inspects them structurally. Adding a network transport is the moment `serde_json::from_slice` gets reached for \"to check it arrived intact\"; the byte comparison already checks that, and harder."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/tests/sync_peer_conformance.rs and crates/happenstance-neon/tests/sync_peer_conformance.rs"
  verifying_test: "the shared payload-opacity rule run against both new fixtures with a deliberately non-decodable payload as the negative control; rg over the two new fixtures and transports for payload-decoding calls returns nothing"

- id: AC-007
  criterion: "P2's adapter is not failed by an assumption the specification never made. GIVEN two stores that assign positions on entirely different mechanisms, WHEN the new fixtures compare any position, THEN the comparison is against a value the store under test assigned — a head captured into a binding before the operation — and never against a literal, because the specification permits gaps and a conformant adapter may leave them."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/tests/sync_peer_conformance.rs and crates/happenstance-neon/tests/sync_peer_conformance.rs"
  verifying_test: "cargo xtask lint-position-literals (via cargo xtask lints), already scoped to the sync suite by gate-mounts-for-the-sync-suite; CF-6 spec/SPECIFICATION.md:7233-7249"

- id: AC-008
  criterion: "P4 finds the phase-2 evidence still on file after the thing it stood in for arrives. GIVEN `crates/happenstance-sync/tests/real_peer_shapes.rs`, whose own header states its honest limit — a stand-in's `Send`-ness is asserted by whoever wrote it (`:16-25`) — WHEN the two real impls land, THEN the `todo!()` bodies and the file's scoped `#![allow(clippy::todo)]` are gone, the finding that header recorded is carried into the record before the paragraph holding it is deleted, and `crates/happenstance-sync/tests/cursor_shape_probe.rs` is untouched and still compiling, because ADR-0026 cites it for why `pull` returns a batch."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/tests/sync_peer_conformance.rs and crates/happenstance-neon/tests/sync_peer_conformance.rs"
  verifying_test: "cargo clippy --workspace --all-targets -- -D warnings green with the allow deleted; rg -n \"todo!\\(\" crates/happenstance-sync/tests returns nothing; cargo test -p happenstance-sync --all-features still compiling crates/happenstance-sync/tests/cursor_shape_probe.rs"

- id: AC-009
  criterion: "P3 and P4 get a gate that ran the whole of it, on the target that matters. GIVEN this project's ceiling is `cargo xtask ci --fast` (`.redkiln/config.yaml:50-55`), WHEN it runs at exit, THEN it is green including the `wasm32` build of `happenstance-sync`, the `wasm32` check of the sync harness and the `wasm32` build of `happenstance-cloudflare` — the Cloudflare leg being a `wasm32-unknown-unknown` target and its suite run a gate step rather than prose — and no `wasm32` step degrades to `skipped`, because all of them carry `probe: None`."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/tests/sync_peer_conformance.rs and crates/happenstance-neon/tests/sync_peer_conformance.rs"
  verifying_test: "cargo xtask ci --fast; cargo xtask wasm listing six steps rather than four (xtask/src/main.rs:196-218, :769-791); cargo xtask spec-trace green"
```
