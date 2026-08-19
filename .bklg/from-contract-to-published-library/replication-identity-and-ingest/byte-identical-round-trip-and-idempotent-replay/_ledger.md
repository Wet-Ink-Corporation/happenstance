---
item: "HS-S0107"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — A payload survives the boundary unchanged, and replay changes nothing

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
  criterion: |-
    GIVEN an adapter author ships an event whose `data` and `metadata` are a binary codec no crate in
    the receiving process has ever heard of — bytes that are not valid UTF-8, carrying high bytes and
    embedded nulls — WHEN that event is appended to an origin `MemoryEventStore`, carried across the
    boundary by `MemorySyncPeer` and ingested into a second, separate `MemoryEventStore`, THEN reading
    the event back out of the receiving store through `EventStore::read` yields `data` and `metadata`
    `Bytes` equal to the bytes appended at the origin — and the author learns this from the suite
    rather than from a corrupted production log. Measured on the way out of the receiver, never on the
    `Bytes` handed to `IngestStore::ingest`.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/registry.rs — the `for_each_sync_peer_rule!` enumeration, emitted through `sync_peer_conformance!` into the tokio, blocking and wasm32 harnesses"
  verifying_test: "rule `the_sync_suite_never_decodes` in happenstance-sync-testkit's rules module, run via `cargo test -p happenstance-sync-testkit --all-features` and `cargo xtask wasm` (clause SY-35, spec/SPECIFICATION.md:6868-6881)"

- id: AC-002
  criterion: |-
    GIVEN the same adapter author's replication run drops a socket mid-batch and the runner redelivers
    the identical `PushBatch`, WHEN the receiver ingests it a second time, THEN nothing the receiver
    holds changes: the set of `EventId`s in the receiving store is identical before and after, the
    typed witness reads `Ingested { appended: 0, skipped: n }` for the batch's n events, and no
    compensation event was authored — so the author can tell a healthy overlap from a peer that quietly
    superseded an event it had already accepted. All three observations together; none may stand alone.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/registry.rs — the `for_each_sync_peer_rule!` enumeration, emitted through `sync_peer_conformance!` into the tokio, blocking and wasm32 harnesses"
  verifying_test: "rule `redelivery_of_an_accepted_group_is_a_no_op` in happenstance-sync-testkit's rules module, run via `cargo test -p happenstance-sync-testkit --all-features` and `cargo xtask wasm` (clause SY-11, spec/SPECIFICATION.md:6144-6152)"

- id: AC-003
  criterion: |-
    GIVEN an adapter author whose peer is entirely correct but forwards a payload in a codec their crate
    cannot parse, WHEN they run the whole sync suite against it, THEN every rule passes — and any
    assertion anywhere in the suite that reached for a decoder fails immediately, locally and by name,
    instead of silently certifying a peer that decodes. Every failure message this story authors names
    the rule, the peer and the assertion and renders no payload byte — not through `Display`, not
    through `{:?}` over the payload, not through `from_utf8` "for a readable message", which is the
    spelling that looks most reasonable in review.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/tests/ — the CF-1–CF-5 declared-variant registry, in the shape at crates/happenstance-testkit/tests/mutation_coverage.rs:140-175"
  verifying_test: "the `ConformantVariant` entry carrying undecodable payloads, declared `fails: &[]` with a non-empty provenance, run against every sync rule by `cargo test -p happenstance-sync-testkit --all-features` (CF-4/CF-5, spec/SPECIFICATION.md:7209-7232)"

- id: AC-004
  criterion: |-
    GIVEN the author wants a green replay rule to mean something, WHEN two deliberately wrong peers run
    against the suite — `BalancedReplayPeer` (appends one duplicate and drops one unrelated event from
    the same batch, so the count is preserved and `Ingested { appended: 1, skipped: n-1 }` reads like a
    healthy overlap) and the inversion peer (redelivery supersedes the previously accepted event,
    leaving count and identities right and the decision wrong) — THEN each fails
    `redelivery_of_an_accepted_group_is_a_no_op` and passes every rule it does not declare, with an
    `expect` pin naming the exact assertion it trips.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/tests/ — the CF-1–CF-5 declared-variant registry, in the shape at crates/happenstance-testkit/tests/mutation_coverage.rs:140-175"
  verifying_test: "the two registered replay mutants and their per-rule `expect` pins, run by `cargo test -p happenstance-sync-testkit --all-features` (CF-2/CF-3, spec/SPECIFICATION.md:7195-7208)"

- id: AC-005
  criterion: |-
    GIVEN the author runs the suite on the constrained runtime they actually ship to, WHEN the harnesses
    are built, THEN both rules are mounted, not merely written: present in `for_each_sync_peer_rule!`
    and therefore emitted into the tokio, blocking and wasm32 harnesses, no rule vanishing from any
    binary, and no rule existing outside the enumeration or the enumeration naming a rule that does not
    exist. Both names are already claimed by [FROZEN] clauses, so `spec-trace`'s check 6 resolves with
    no edit to `spec/SPECIFICATION.md`.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/registry.rs — the `for_each_sync_peer_rule!` enumeration and the rules module it enumerates"
  verifying_test: "the `no_orphan_sync_rules` meta-test (both directions, shape at crates/happenstance-testkit/src/registry.rs:412-434), plus `cargo xtask spec-trace` check 6 (xtask/src/spec_trace.rs:85-89, :756), `cargo xtask wasm`, and an empty `git diff --stat spec/SPECIFICATION.md`"

- id: AC-006
  criterion: |-
    GIVEN `adr-0003-provisional-lift` (HS-S0108) must cite this measurement rather than re-derive it,
    WHEN this story merges, THEN the evidence exists in citable form — `_ledger.md`'s AC-001 row states
    the rule name, the real test path and the commit, and `CHANGELOG.md` carries one entry per new rule
    naming the defect it detects (a payload that was re-encoded; an inversion on redelivery) rather than
    that a rule was added. AND if byte-identity cannot be achieved, the deliverable is a written finding
    naming the hop that mutated the bytes, routed to HS-S0106 or raised as a blocker — never a
    decoded-value comparison that satisfies the wording while proving nothing.
  satisfied: false
  evidence: ""
  mount_point: "CHANGELOG.md and this story's `_ledger.md` — the two artefacts HS-S0108's `.kb/_intake/` document quotes; read by `cargo xtask lints` and `redkiln verify --grain story` respectively"
  verifying_test: "`cargo xtask lints` — CF-29's `changelog_names_every_rule` (xtask/src/lints.rs:525) — plus `redkiln verify --grain story` over this ledger (.redkiln/config.yaml:62-67, `require_ledger: true`)"
```
