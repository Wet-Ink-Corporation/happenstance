---
item: "HS-S0069"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The suite runs over one-shot HTTP

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story. First, six of the twelve rows can only be flipped from the
**credentialed Neon CI job** — the default gate cannot reach the endpoint by design (DR-9), so their
evidence is a job run and a rule name, never a local `cargo test` line. Second, a `Skipped` outcome
is admissible evidence *only* when it carries the fixture's stated reason in the harness output; a
rule that is absent from the run is never evidence of anything.

```yaml
- id: AC-001
  criterion: 'GIVEN an adapter author whose store is one-shot HTTP, WHEN they run `event_store_conformance!` against a `NeonFixture` backed by a real Neon `/sql` endpoint, THEN the **entire** macro expansion executes to completion and every rule in it reports pass, fail, or `Skipped` with the fixture''s stated reason — no rule absent, no hand-picked subset, and no pooled-Postgres stand-in for the endpoint.'
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/conformance.rs — the event_store_conformance!(mod_name = …, emit = …, fixture = NeonFixture::…) invocation"
  verifying_test: "crates/happenstance-neon/tests/conformance.rs (whole expansion), run as `cargo test -p happenstance-neon --all-features -- --ignored --show-output` in the Neon CI job; rule count asserted equal to crates/happenstance-testkit/src/registry.rs"

- id: AC-002
  criterion: 'GIVEN an author whose store cannot open a cursor or a transaction, WHEN their application reads a multi-item `Query` with `ReadOptions`, THEN one `SELECT` — items disjoined, types OR within an item, tags AND matched by superset — returns the whole result in **one** round trip, so all items share one snapshot for free; and `read` is still not `async`, still returns the stream at the top level, and still sends nothing until the first `poll_next`.'
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/event_store.rs — read_request (:114-116) and the `impl EventStore for NeonEventStore<T>` read path (:171-181), mounted via crates/happenstance-neon/tests/conformance.rs"
  verifying_test: "crates/happenstance-testkit/src/suite.rs::query_items_share_one_snapshot (:5696), ::read_result_is_stable_under_concurrent_append (:5592), ::limit_applies_across_items_not_per_item (:1402), ::read_from_a_gap_position (:1490) in the Neon job; plus the one-statement assertion in crates/happenstance-neon/src/event_store.rs mod tests"

- id: AC-003
  criterion: 'GIVEN a JSON transport that renders `bigint` as a string and `bytea` as `\x…` hex, WHEN the author''s application reads events back, THEN every `SequencedEvent` round-trips byte-identically by OID-directed decoding, a body over `max_response_bytes` fails the read as `NeonError::ResponseTooLarge` rather than returning a silently short prefix, and a `position` that will not fit `NonZeroU64` is `NeonError::InvalidPosition` — never a panic, a clamp, or a zero.'
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/event_store.rs — decode_read_response (:257-263) and NeonReadStream's buffering path (:310-328), mounted via crates/happenstance-neon/tests/conformance.rs"
  verifying_test: "crates/happenstance-neon/src/event_store.rs mod tests — decoder tests over both wire::ResponseBody envelopes, an oversized body, and an out-of-range position string; corroborated by the payload and ordering rules in the Neon job"

- id: AC-004
  criterion: 'GIVEN an author appending under an `AppendCondition`, WHEN the append is issued, THEN the condition check and the insert are computed on one snapshot inside **one** CTE statement in one round trip — never a probe followed by a write — and a zero-event batch is refused as `AppendError::NoEvents` before any request is built.'
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/event_store.rs — conditional_append_request (:159-165) and the append body (:183-206), mounted via crates/happenstance-neon/tests/conformance.rs"
  verifying_test: "crates/happenstance-testkit/src/suite.rs::condition_rejection_is_reported_as_condition_violated (:4949) and ::racing_conditional_appends_elect_one_winner (:5325) in the Neon job; plus the one-statement and empty-batch-against-NullTransport tests in crates/happenstance-neon/src/event_store.rs mod tests"

- id: AC-005
  criterion: 'GIVEN a conditional append that lost, WHEN the single returned row is decoded, THEN `appended` non-null yields `AppendOutcome::Appended` and `conflict` non-null yields `AppendOutcome::Conflict`, both-null or both-non-null is a decode error rather than an `Ok`, and the author''s application therefore never sees a conflict silently swallowed into success.'
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/event_store.rs — decode_append_response (:249-255) and the AppendOutcome enum (:232-247), mounted via crates/happenstance-neon/tests/conformance.rs"
  verifying_test: "crates/happenstance-neon/src/event_store.rs mod tests — all four null/non-null combinations of decode_append_response; `cargo xtask affected --base main` (clippy -D warnings) proving the #[expect(dead_code)] removal; crates/happenstance-testkit/src/suite.rs::condition_rejection_is_reported_as_condition_violated (:4949) in the Neon job"

- id: AC-006
  criterion: 'GIVEN two of the author''s writers appending to the same boundary at the same moment, WHEN both round trips are in flight simultaneously, THEN exactly one succeeds, the store holds exactly one event, and the loser learns it lost as `AppendError::ConditionViolated(_)` — including when the endpoint reports a SQLSTATE `40001` serialisation abort, which on a **conditional** append is reported as `ConditionViolated::unspecified()` and never as `AppendError::Store`. An abort on an **unconditional** append stays a store error.'
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/event_store.rs — the append error mapping (:196-206) over crates/happenstance-neon/src/error.rs:39-49, mounted via crates/happenstance-neon/tests/conformance.rs"
  verifying_test: "crates/happenstance-testkit/src/suite.rs::interleaved_appends_on_one_handle_elect_one_winner (:5411-5484) in the Neon job; plus the synthetic-40001 mapping test (conditional and unconditional paths) in crates/happenstance-neon/src/event_store.rs mod tests"

- id: AC-007
  criterion: 'GIVEN an author reading the crate''s own soundness claim, WHEN they ask what isolation level the append actually ran at, THEN the answer in the docs is the answer on the wire — the CTE either carries the level it claims, or is shown sound at the endpoint''s default and the claim at `lib.rs:74-77` / `transport.rs:56-60` is corrected — **and** the story records that no rule available to a bare-flavour store puts real contention on the append, so a green run is not read as evidence it never gave.'
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/event_store.rs (conditional_append_request, :159-165) together with crates/happenstance-neon/src/transport.rs:56-85 and crates/happenstance-neon/src/config.rs:13-23"
  verifying_test: "crates/happenstance-neon/src/event_store.rs mod tests — an assertion over the emitted SqlRequest's headers (or over the corrected invariant, if the docs are what change); the unfalsifiability statement recorded in this story's implementation-report.md citing crates/happenstance-testkit/src/lib.rs:101-110"

- id: AC-008
  criterion: 'GIVEN a store whose positions become visible after they are assigned, WHEN the author''s application calls `head` or `contains_event_id`, THEN `head` reports the **visibility frontier** in one `SELECT` carrying the same predicate the read does (not `max(position)`), returns `None` on an empty store, read-your-own-writes is not claimed, and `contains_event_id` answers from migration 1''s `origin_store` / `origin_position` columns in one round trip.'
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/event_store.rs — head (:208-218) and contains_event_id (:221-229), mounted via crates/happenstance-neon/tests/conformance.rs"
  verifying_test: "crates/happenstance-testkit/src/suite.rs::head_of_an_empty_store_is_none (:1731), ::head_is_the_highest_visible_position (:1798), ::nothing_below_an_observed_position_appears_later (:5880), ::contains_event_id_reports_membership (:2551) in the Neon job"

- id: AC-009
  criterion: 'GIVEN an author whose events sit near the transport''s ceiling, WHEN they append something too large, THEN the adapter refuses it **before** the round trip as `AppendError::ExceedsStoreLimit { limit: … }` — never as an `AppendError::Store` wrapping a 4xx, never a truncation; and independently, the guaranteed minima are **accepted**: 65,536 bytes of payload, 64 tags, a 128-item query, a 128-event batch, notwithstanding hex `bytea` roughly doubling them on the wire.'
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/event_store.rs — the pre-flight refusal inside append (:183-206), ahead of round_trip; ceilings from NeonFixture per crates/happenstance-testkit/src/contract.rs:237-252"
  verifying_test: "crates/happenstance-testkit/src/suite.rs::append_reports_exceeded_store_limits (:4282-4457) and the four store_accepts_the_guaranteed_minimum_* rules (:3775, :3829, :3880, :3954) in the Neon job; plus the over-ceiling-against-NullTransport test in crates/happenstance-neon/src/event_store.rs mod tests"

- id: AC-010
  criterion: 'GIVEN an author targeting `wasm32` where nothing is `Send`, WHEN they depend on this crate, THEN it still implements only the **bare** `EventStore`, has no `#[async_trait]`, has no second `SendEventStore` impl, keeps `read` non-`async` and lazy, and the `REQUIRED` wasm32 `cargo check` of `happenstance-neon` — selected by name in `xtask/src/main.rs:271`, `:784-789` — is still green under its current step name.'
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/event_store.rs — the `impl EventStore for NeonEventStore<T>` block (:168-230) and its flavour tests (:505-549); the named wasm32 step in xtask/src/main.rs:784-789"
  verifying_test: "crates/happenstance-neon/src/event_store.rs mod tests::both_stores_are_bare_event_stores, ::a_store_can_be_built_without_a_transport, ::the_generic_helper_call_site_compiles; `cargo xtask wasm` and the wasm32 step inside `cargo xtask ci --fast`"

- id: AC-011
  criterion: 'GIVEN a contributor with no Neon credential and no Docker, WHEN they run the default gate, THEN `cargo test --workspace --all-features` and `cargo xtask ci` are green with nothing reachable, because the live invocation is gated **whole** — and GIVEN the credentialed job, WHEN it runs, THEN every declined capability appears as an emitted test returning `RuleOutcome::Skipped` printing the fixture''s own stated reason, with no `#[cfg]` removing any rule from the expansion.'
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/tests/conformance.rs — the whole-invocation gate around the macro (never a per-rule #[cfg]), against .redkiln/config.yaml:40, :55"
  verifying_test: "`cargo xtask ci --fast` and `cargo xtask affected --base main` green on a clean checkout with no credential; the Neon job's --show-output transcript showing each Skipped rule's stated reason; an assertion over crates/happenstance-neon/tests/** that no per-rule #[cfg] exists"

- id: AC-012
  criterion: 'GIVEN the next reader of this crate — an adapter author copying it as a worked example — WHEN they read its module documentation and its tests, THEN the schema block at `event_store.rs:36-54` describes the schema the adapter actually issues SQL against (migration 1, not `bigserial`), and no test anywhere in this story asserts a literal position value, because the specification permits gaps and this is the project where gaps stop being hypothetical.'
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-neon/src/event_store.rs:36-54 — the module-level schema documentation above the `impl EventStore` block; reconciled against crates/happenstance-postgres/src/event_store.rs:22-35"
  verifying_test: "`cargo xtask lint-position-literals` (xtask/src/main.rs:389, :687; xtask/src/lints.rs:628) and the `docs` step inside `cargo xtask ci --fast`"
```
