---
item: HS-S0103
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — happenstance-sync-testkit and the sync rule registry

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
  criterion: "**The crate is born unpublishable, and the release train cannot pick it up by accident.** GIVEN the repository owner is holding `0.2.0` to the three crates the charter names, WHEN this PR adds a fourth workspace member through `Cargo.toml:3`'s `crates/*` glob, THEN `crates/happenstance-sync-testkit/Cargo.toml` carries `publish = false` **written explicitly** (not absent, not inherited from the sibling, which has no such key) and its own `version` key rather than `version.workspace = true`, and no publishable crate gains a dependency on it."
  satisfied: false
  evidence: ""
  mount_point: "Cargo.toml:3 — the `crates/*` member glob that admits the new crate"
  verifying_test: "crates/happenstance-sync-testkit/tests/manifest_shape.rs::crate_is_born_unpublishable"

- id: AC-002
  criterion: "**An adapter author points the suite at their peer and it runs.** GIVEN an adapter author who has just written a `SyncPeerFixture` in a crate that has never heard of this workspace's internals, WHEN they write `happenstance_sync_testkit::sync_peer_conformance!(MyFixture::new());` at the top level of a `tests/` file with no other setup, THEN it compiles and `cargo test` emits a `dcb_sync_conformance` module that runs to completion — reporting zero rules at this story, which is honest rather than empty, because the rules are HS-S0105's."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs"
  verifying_test: "crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs — the `sync_peer_conformance!(MemorySyncPeerFixture::new())` invocation"

- id: AC-003
  criterion: "**The same peer is certified on all three runtimes without the author writing three suites.** GIVEN the constrained-runtime developer whose peer must work under tokio, under no runtime at all, and on `wasm32-unknown-unknown`, WHEN they mount the suite three times with the three emitters, THEN all three harnesses expand the **one** `for_each_sync_peer_rule!` enumeration, so a rule added once in HS-S0105 appears in all three without a harness being touched."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs plus its two siblings memory_peer_conformance_blocking.rs and memory_peer_conformance_wasm.rs"
  verifying_test: "crates/happenstance-sync-testkit/tests/memory_peer_conformance_blocking.rs and crates/happenstance-sync-testkit/tests/memory_peer_conformance_wasm.rs"

- id: AC-004
  criterion: "**The rule list is a value the repository can inspect, not just a thing that expands.** GIVEN a maintainer who needs the enumeration in expression position to write any meta-check over it, WHEN they write `let names = for_each_sync_peer_rule!(happenstance_sync_testkit::__emit_rule_names);`, THEN it compiles — because the callback is captured as `$($callback:tt)+` and not as `$cb:path`, a parsed `path` fragment being unable to sit in callee position."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/registry.rs — `for_each_sync_peer_rule!`, reached by the harnesses at crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs"
  verifying_test: "crates/happenstance-sync-testkit/src/registry.rs — the expression-position doctest on `for_each_sync_peer_rule!`, run by `cargo test -p happenstance-sync-testkit --doc`"

- id: AC-005
  criterion: "**No rule can exist and never run, and no name can be enumerated that no longer exists.** GIVEN the adapter author trusting a green run to mean the whole bar was applied, WHEN a rule is defined in the `rules` module but omitted from the macro — or enumerated in the macro after being renamed in the module — THEN the meta-test fails and names the offending rule, in **both** directions. Green and vacuous at this story; load-bearing from HS-S0105."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/registry.rs — the `rules` module and `for_each_sync_peer_rule!` it scans"
  verifying_test: "crates/happenstance-sync-testkit/src/registry.rs::no_orphan_sync_rules"

- id: AC-006
  criterion: "**Declining a capability costs the author a line in the output, never a vanished test.** GIVEN an adapter author whose peer genuinely cannot do something, WHEN their fixture declares that `Capability` as `declined(\"…\")`, THEN the rule is still emitted as a test, returns `RuleOutcome::Skipped`, and the harness prints the fixture's own stated reason — and neither type is redeclared in this crate: both are consumed from `happenstance-testkit` and re-exported from the new crate's root so the author names one crate."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs"
  verifying_test: "crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs (the declined `BOTH_ROLES` reported with its reason) and crates/happenstance-sync-testkit/tests/manifest_shape.rs::contract_types_are_reexported_not_redeclared"

- id: AC-007
  criterion: "**A `!Send` peer holding a `!Send` store is a first-class citizen of the bar.** GIVEN the constrained-runtime developer on Cloudflare Workers, whose store is behind an `Rc` and can never be `Send`, WHEN they implement `SyncPeerFixture` for it, THEN the suite compiles and runs against it — because the fixture trait has **no** `Send` bound, no `trait_variant` second flavour, no GAT, and spells its methods `-> impl Future` rather than `async fn`. This is SY-17's own prescribed artefact (`spec/SPECIFICATION.md:6341-6360`) and a `Send` bound anywhere on the trait fails a `[FROZEN]` clause by construction."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs — the same macro, mounted in tests/send_free_fixture.rs against the `!Send` variant"
  verifying_test: "crates/happenstance-sync-testkit/tests/send_free_fixture.rs"

- id: AC-008
  criterion: "**The bar is not shaped like the in-process oracle it was first written against.** GIVEN a one-shot-HTTP peer that cannot hold a transport open and must be reconstructed between exchanges, WHEN it is driven through the same fixture contract as `MemorySyncPeer`, THEN it passes everything — because the fixture exposes `round_trips()` as a counter it increments itself (never inferred from a return type) and `reconnect()` that drops the peer object and rebuilds it against the same far side, which is the mechanism SY-15's `Rule:` field names and the shape SY-16's `Rejects:` demands."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs — the same macro, mounted in tests/one_shot_http_fixture.rs against the one-shot variant"
  verifying_test: "crates/happenstance-sync-testkit/tests/one_shot_http_fixture.rs::OneShotHttpFixture"

- id: AC-009
  criterion: "**What a fixture may honestly decline is decided here, once, and each call is a MUST or a trade on the record.** GIVEN HS-S0110 needing one fixture in both roles and HS-S0112 needing an unavailable environment reported rather than hidden, WHEN a fixture author reads the trait, THEN they find exactly two capability constants — `PEER_RECONSTRUCTION`, a **MUST** whose decline makes rules fail rather than skip (as `SECOND_HANDLE` does), and `BOTH_ROLES`, a genuine trade — **zero** limit constants, because `SyncPeer::limits()` already carries them and SY-18 tests them there, and rustdoc stating that an associated `const` cannot mean \"the network was down today\": a transiently unreachable environment is a failure."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/contract.rs — `SyncPeerFixture`'s capability constants, reached through tests/memory_peer_conformance.rs"
  verifying_test: "crates/happenstance-sync-testkit/tests/one_shot_http_fixture.rs::declining_peer_reconstruction_fails_rather_than_skips"

- id: AC-010
  criterion: "**The suite cannot certify a peer that reads payloads, because it cannot read one itself.** GIVEN SY-35 `[FROZEN]` — replication reasons only about `EventType` and `Tags` — WHEN a rule author tries to assert on a decoded `Event::data` or `Event::metadata`, THEN there is no fixture method that hands them one: nothing returns a parsed payload, and a position reaches a rule only inside an `EventId` or as a value the receiving store reported, never as a bare foreign integer. CF-6's lint does not cover this crate until HS-S0104, so the trait shape is the enforcement."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/contract.rs — the `SyncPeerFixture` method signatures, reached through tests/memory_peer_conformance.rs"
  verifying_test: "crates/happenstance-sync-testkit/tests/one_shot_http_fixture.rs::payloads_are_undecodable, with the source assertion in crates/happenstance-sync-testkit/tests/manifest_shape.rs"

- id: AC-011
  criterion: "**The adapter author's first contact with the suite is a worked example, not a signature.** GIVEN someone landing on `happenstance-sync-testkit`'s docs with no context, WHEN they read the entry macro's page, THEN they find a compiled example that invokes `sync_peer_conformance!` the way a foreign crate would (`$crate::`-qualified emitter and all), rustdoc on every public item, the CHANGELOG entry for the new crate, and — recorded rather than repaired — the note that SY-15's `Rule:` field says `peer_conformance!` while this crate ships `sync_peer_conformance!`, handed forward to HS-S0114."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/lib.rs — the crate root that re-exports the entry macro mounted at tests/memory_peer_conformance.rs"
  verifying_test: "crates/happenstance-sync-testkit/src/lib.rs — the `sync_peer_conformance!` doctest, run by `cargo test -p happenstance-sync-testkit --doc`"
```
