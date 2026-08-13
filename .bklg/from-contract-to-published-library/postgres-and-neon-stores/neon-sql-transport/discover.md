---
item: HS-S0067
stage: discover
created: 2026-08-12T13:02:35.694Z
updated: 2026-08-12T13:02:35.694Z
template_sig: 86ce4036
rendered_sig: b39c4457
---

# Discover — A real SqlTransport, host and wasm32

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one line: a real `SqlTransport` — host client and `wasm32` client, correctly `cfg`-scoped — proven by one live `/sql` round trip, with ship-vs-dev-dependency, `cargo deny` admissibility and the wasm32 build all answered **before any fixture depends on it** | `_storymap.md`, *Slices* table, `neon-sql-transport` row | The ordering is the deliverable as much as the code: a transport bug and a conformance failure must never be debugged as the same failure |
| **AC-006** — Neon runs the suite over one-shot HTTP | `project.md`, *Acceptance criteria*, AC-006 | This story owns "the real endpoint reached"; `neon-append-and-read-over-http` owns "the suite run against it" (`_storymap.md`, *Coverage*, AC-006 row) |
| **AC-011** — the wasm32 build of `happenstance-neon` is still part of the default gate | `project.md`, *Acceptance criteria*, AC-011 | This story owns the wasm32 third of AC-011: whatever the transport is, it must not cost the crate its second target |
| No inbound `depends_on`; "the transport work does **not** wait on the Postgres schema… which is the one place this plan buys real parallelism" | manifest (`dependsOn: []`); `_storymap.md`, *Merge order* item 2 | It can start on day one, alongside `claim-crate-names` |
| `SqlTransport` is a one-method trait and `NullTransport` fails every round trip by design | `crates/happenstance-neon/src/transport.rs:245-251`, `:267-298` | There is no real implementation in the tree. This is unbudgeted scope named by `_grounding.md` tension 5 and gating both Neon capability stories |
| **A non-2xx status is not a transport error** — "it is an answer, and it is where Neon puts SQL errors, so it must reach the decoder intact" | `crates/happenstance-neon/src/transport.rs:249-250`; `:258-260` | The single most load-bearing sentence in the trait, and the one an idiomatic HTTP wrapper violates by reflex |
| `round_trip` is written `-> impl Future` rather than `async fn` so no `Send` bound is implied; "a `wasm32` implementation over `fetch` cannot supply one" | `crates/happenstance-neon/src/transport.rs:253-256`, and the module preamble at `:40-43` | ADR-0001 and DR-8. Nothing in the implementation may reintroduce a `Send` bound, including by way of a helper's signature |
| The crate owns no HTTP client on purpose: a host client needs a TLS stack and `wasm32-unknown-unknown` has none, "they are two different clients" | `crates/happenstance-neon/src/lib.rs:91-99` | A single implementation cannot serve both targets. `cfg` scoping is structural, not a tidiness measure |
| `happenstance-neon` has **no `[dev-dependencies]` at all** today | `crates/happenstance-neon/Cargo.toml` | Every option — ship, off-by-default feature, dev-only — is a first manifest decision rather than an adjustment to an existing one |
| The TLS licence note is measured, not assumed: `sqlx`'s `tls-rustls` fails `cargo deny` on `webpki-roots` (`CDLA-Permissive-2.0`), **not** on `ring`, which is `Apache-2.0 AND ISC` and passes | `crates/happenstance-postgres/src/lib.rs:40-47` | The nearest precedent, and it says the failure is the CA bundle rather than the crypto. `deny.toml:10-19` allows MIT / Apache-2.0 / Apache-2.0-WITH-LLVM-exception / BSD-2 / BSD-3 / ISC / Unicode-3.0 / Zlib, with `[graph] all-features = true` |
| `cargo xtask wasm` selects steps **by name** and panics on a miss, "precisely so that inserting a step cannot silently repoint" it | `xtask/src/main.rs:785-790`; the step itself at `:264-282` | Renaming "wasm32 build of the Neon adapter" is a gate change, not a cosmetic one |
| A feature that is not target-scoped is caught by the wasm32 **feature-powerset** step, which is OPTIONAL and `cargo hack`-gated | `_decomposition.md`, *Architecture brief* §9.2 point 3; *Deployment brief* → *Notes*, "Feature flags / config gating" | The mandatory plain wasm32 build passes with the feature off. If `cargo hack` is absent the step prints `skipped`, and the defect ships |
| The isolation level "travels as the `Neon-Batch-Isolation-Level` header, not in the JSON body, and applies only when `SqlRequest::statements` holds **more than one** statement — a single statement is its own implicit transaction and the header is ignored" | `crates/happenstance-neon/src/transport.rs:56-60` | Load-bearing, and in tension with the one-statement CTE that AC-008 rests on. See question 5 |
| The transport is proven by one small live round trip *before* the fixture is wired to a macro | `_decomposition.md`, *Testing brief* → *Notes* §5 | "so a transport bug and a conformance failure are never debugged as the same failure" |
| `MAX_RESPONSE_BYTES` is 64 MiB and hard, and hex `bytea` rendering "roughly doubles its size against [it] in both directions" | `crates/happenstance-neon/src/transport.rs:49-54`, `:91-94` | The transport is where the ceiling is observed. `neon-fixture-and-live-job` turns it into fixture constants |

## Questions

**Answered.**

1. *Does one implementation serve both targets?* No. A host client needs a TLS
   stack; `wasm32-unknown-unknown` has none and reaches the network through
   `fetch`. Two `cfg`-scoped implementations, and the crate already says so
   (`crates/happenstance-neon/src/lib.rs:91-99`).
2. *Does a non-2xx response become `Self::Error`?* No, and this is the one thing the
   trait's own documentation forbids in terms: Neon puts SQL errors there, so the
   body must reach the decoder intact
   (`crates/happenstance-neon/src/transport.rs:249-250`).
3. *May the implementation attach a `Send` bound?* No. `round_trip` is desugared
   specifically so none is implied (`:253-256`), and DR-8 makes the `!Send` story
   non-negotiable.
4. *Is the transport proven before the fixture?* Yes — one small live round trip
   that decodes correctly, landing in this story rather than in
   `neon-fixture-and-live-job`.

**A question this story surfaces and does not own.**

5. *Does a one-statement CTE actually run at `Serializable`?* The crate says the
   collapse "needs `IsolationLevel::Serializable` to be sound, which is why
   `NeonConfig`'s default is `Serializable`" (`crates/happenstance-neon/src/lib.rs:74-77`),
   and the transport says the isolation header "applies only when… more than one
   statement" and that "a single statement is its own implicit transaction and the
   header is ignored" (`crates/happenstance-neon/src/transport.rs:56-60`). Those two
   sentences are in tension, and the live round trip this story performs is the
   cheapest place in the plan to find out. **Surfacing it is this story's; the
   verdict belongs to `neon-conflicting-position-verdict` (HS-S0070)**, which owns
   the CTE's behaviour against a real endpoint, with
   `neon-append-and-read-over-http` (HS-S0069) as the consumer. `spec` records the
   observation and routes it; it does not resolve it.

**Deferred to `spec`.**

6. *Ships, off-by-default feature, or dev-dependency* — Architecture §9.2 question 1,
   and the deployment posture differs materially between "a published crate that can
   make live HTTP calls at runtime" and one that cannot.
7. *Which client, and whether `cargo deny check` admits it* — §9.2 question 2, run
   against the candidate before committing to it. Growing `deny.toml`'s allowlist is
   a gate weakening and needs a recorded decision.

**Deferred to the owning stories.**

8. *How the adapter buys ES-10's visibility invariant.*
   `adr-0024-position-visibility-mechanism` (HS-S0065). No signal here bears on it;
   the transport carries SQL and does not author it.
9. *Whether `conflicting_position` is a promise every adapter owes or a hint one may
   omit.* HS-S0070, per question 5.

## Decision

The problem this slice solves is that `happenstance-neon`'s only transport fails
every round trip by design, so the crate that exists to occupy the far end of the
transport axis has never spoken to anything. Building a real one is not a detail
below the adapter: it is where three decisions with consequences outside this
project get made — whether the published crate gains a runtime HTTP dependency,
whether the licence allowlist still holds, and whether the crate keeps the second
target that is the whole reason it is written `!Send`. The story answers those three
in order and then proves the result with a single live `/sql` round trip, before any
fixture exists to confuse a transport defect with a conformance failure. The spec
will cover: the two `cfg`-scoped implementations and their boundary; the
error/answer split at non-2xx; the absence of any `Send` bound; the manifest
decision and its recorded rationale; the `cargo deny check` result against the
chosen client; the wasm32 build staying green under its existing step name; and the
single round-trip test, including what it observes about the isolation header on a
one-statement request. No `[FROZEN]` clause is read or changed, so no ADR is owed.

## The wrong implementation

**A transport built on `error_for_status()`.** It is the first line any experienced
Rust author writes around an HTTP client, it compiles, and a happy-path round-trip
test against a `SELECT 1` passes cleanly. It also destroys the adapter: Neon returns
SQL errors as non-2xx responses, so every constraint violation, every syntax error
and — critically — every outcome the append path needs to inspect becomes
`NeonError::Transport` before `decode_append_response` ever sees a row. `append`
then maps it through `AppendError::Store` (`crates/happenstance-neon/src/event_store.rs:194-197`),
`AppendError::ConditionViolated` becomes unreachable, and AC-008's whole question is
answered "no" by an accident in a layer that was not supposed to have an opinion.
The trait forbids it in one sentence — "a non-2xx status is **not** one of these — it
is an answer, and it is where Neon puts SQL errors, so it must reach the decoder
intact" (`crates/happenstance-neon/src/transport.rs:249-250`) — and nothing
mechanical enforces it, because `Self::Error` is the implementor's own type and any
mapping into it type-checks.

**Its quieter sibling: a transport that carries the isolation level in the JSON
body.** Everything compiles and every single-statement operation behaves identically,
because a single statement is its own implicit transaction. The header is where the
endpoint reads it (`crates/happenstance-neon/src/transport.rs:56-60`), so a batched
request silently runs at the endpoint's default instead of `Serializable`, and the
soundness precondition the CTE rests on is gone. The failure mode is a lost update
under contention — no error, no failing test, and not reliably reproducible by the
conformance suite, which is the same silent shape `ProbeThenWriteStore` exists to
demonstrate one layer up.

**And the target mutant: a host-only transport behind a feature that is not
target-scoped.** `cargo check --target wasm32-unknown-unknown -p happenstance-neon`
(`xtask/src/main.rs:264-282`) passes with the feature off, so the mandatory gate step
is green. Only the wasm32 **feature-powerset** step notices, and that step is
OPTIONAL and `cargo hack`-gated: where the tool does not resolve it prints `skipped`
and the crate ships claiming a target it cannot build for under its own feature.
DR-8 makes this non-negotiable, which is why `spec` requires the powerset step's
result to be reported as evidence rather than assumed.

No conformance rule is added by this story and no store is defined, so nothing is
owed to `crates/happenstance-testkit/tests/`. The instruments that reject these three
are, respectively, the round-trip test asserting that a deliberately failing
statement's body reaches the decoder; an assertion on the request the transport
actually builds; and the powerset step's reported output.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
