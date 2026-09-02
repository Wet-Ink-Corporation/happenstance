---
item: HS-S0068
stage: discover
created: 2026-08-12T13:02:36.594Z
updated: 2026-08-12T13:02:36.594Z
template_sig: 86ce4036
rendered_sig: be8cf4dc
---

# Discover — NeonFixture against a live branch, and its credentialed CI job

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one line: `NeonFixture` against a live branch — `SECOND_HANDLE` answered as the MUST it is, `REOPEN` and `MID_BATCH_FAULT` answered honestly, the 64 MiB-anchored ceilings stated as numbers — plus the credentialed Neon CI job that runs it | `_storymap.md`, *Slices* table, `neon-fixture-and-live-job` row | The fixture and the job ship together, because a fixture no workflow runs is not delivered |
| **AC-007** — nothing Neon cannot do passes silently | `project.md`, *Acceptance criteria*, AC-007 | This story owns "capabilities declared with real reasons"; `neon-append-and-read-over-http` owns "every rule present in the run and reporting" (`_storymap.md`, *Coverage*, AC-007 row) |
| **AC-011** — the default gate stays Docker-free and network-free | `project.md`, *Acceptance criteria*, AC-011 | This story owns the Neon job: a credential the default `gate` job does not have, in a workflow job of its own |
| `depends_on: neon-sql-transport` (HS-S0067) | manifest; `_storymap.md`, *Merge order* item 4 | Supplies the real `SqlTransport` the fixture's `connect()` builds a store over, already proven by one live round trip, with its ship/dev-dependency and `cargo deny` questions answered |
| **DR-4**: substituting a pooled Postgres connection "destroys the exact axis the adapter exists to occupy and is **not an acceptable fixture**" | `project.md`, *Derived requirements*, DR-4; risk table row 3 | Named in the charter as "the cheapest possible shortcut". The section below treats it as this story's primary mutant |
| `SECOND_HANDLE` is a **MUST**, and the rule requiring it **panics** on a decline, quoting the fixture's own words, rather than reporting a skip | `crates/happenstance-testkit/src/contract.rs:135-161` | A decline here is loud, not silent. Postgres gets a second pool checkout; Neon gets a second client against the same branch |
| `REOPEN` is a SHOULD; both stores are durable, so both can plausibly support it, and "if a fixture declines, the reason must be real, not convenient" | `_decomposition.md`, *Architecture brief* §6 | Durability's far end is `sqlite-durable-store`'s. This project owes an honest answer, not an investment |
| `MID_BATCH_FAULT` defaults to declined because "an in-memory store has no fault to inject" | `crates/happenstance-testkit/src/contract.rs:207-211` | Neon, over one non-interactive statement, plausibly cannot offer it. Either answer is fine; a *silent default* on a store that could have co-operated is the one to avoid |
| The three ceilings are `Option<usize>` facts, not `Capability` trades: stating a number commits the store to accepting exactly that many bytes and refusing one more with `AppendError::ExceedsStoreLimit { limit: StoreLimit::… }` | `crates/happenstance-testkit/src/contract.rs:253`, `:262`, `:279`; `_decomposition.md`, *Architecture brief* §6 | "A number that is not where the real ceiling sits fails in one direction or the other" |
| The anchor is `MAX_RESPONSE_BYTES` — 64 MiB, hard, with no cursor to fall back on — **halved in effect** because hex `bytea` rendering "roughly doubles its size against [it] in both directions" | `crates/happenstance-neon/src/transport.rs:49-54`, `:91-94` | The stated ceiling is not the constant. Deriving it from the constant without the doubling is the arithmetic mutant below |
| `Capability::declined("")` is rejected by an `assert!` in a `const fn`, but for an **associated** const it fires at *codegen* | `crates/happenstance-testkit/src/contract.rs:404-412` | `cargo build` and `cargo test` catch it; `cargo clippy` does not. A green clippy proves nothing about this fixture's constants |
| A declined capability still emits a test returning `RuleOutcome::Skipped`; `#[cfg]`-ing a rule out is the forbidden move | `crates/happenstance-testkit/src/contract.rs:26-42`; `_decomposition.md`, *Architecture brief*, AC-007 row | DR-5. The decline path is the *only* sanctioned way for Neon to not pass a rule short of an amended clause |
| `connect` panics rather than returning `Result`, so "the database is down" never enters the same channel as "the adapter is wrong" | `crates/happenstance-testkit/src/contract.rs:316-321` | For a fixture reaching a remote endpoint over a credential, the panic message is the operational interface |
| `MemoryFixture` is the reference: `SECOND_HANDLE = Capability::SUPPORTED`, `REOPEN` declined **with the real reason** | `crates/happenstance-testkit/src/fixtures.rs:270-286` | Copy the shape, including the refcount-owning handle from `connect` |
| The nearest in-tree precedent for a job needing a credential the default `gate` does not | `.github/workflows/ci.yml:102` (the `backlog` job); `_decomposition.md`, *Deployment brief* → *Notes*, "CI implication, concretely" | A sibling job, not a step inside `gate`, and the credential reached as an Actions secret |
| A flaky live job is "fixed or reported, never quietly made non-blocking without a recorded decision" | `project.md`, risk table, last row; DR-9 | There is no "revert to skip" path this project sanctions |

## Questions

**Answered.**

1. *May the fixture be backed by anything other than a real Neon `/sql` endpoint?*
   No. DR-4 states it, AC-006 states it, and the risk table names the substitution
   as the cheapest shortcut available. `spec` makes the endpoint an acceptance
   criterion, not an implementation note.
2. *Is `SECOND_HANDLE` declinable?* Technically yes and practically no: the rule
   panics quoting the fixture's stated reason
   (`crates/happenstance-testkit/src/contract.rs:135-161`), so a decline fails the
   job loudly. The answer is a second client against the same branch.
3. *Are the ceilings capabilities?* No — `Option<usize>` facts. A ceiling is a store
   reporting a fact about itself; a decline is a trade.
4. *Is the 64 MiB constant the ceiling to state?* No. `MAX_RESPONSE_BYTES` is the
   response cap, and hex `bytea` rendering roughly doubles a payload against it in
   both directions (`crates/happenstance-neon/src/transport.rs:91-94`), so the
   effective ceiling for event data is materially below the constant. `spec` states
   the derivation, not just the number.

**Deferred to `spec`.**

5. *`REOPEN` and `MID_BATCH_FAULT`'s answers*, each with its real reason. Both are
   deliberate answers rather than defaults, and either verdict is admissible.
6. *The ceilings' actual values*, which need a live branch to establish, and their
   relationship to VT-21 – VT-24's guaranteed minima — a store MAY state a ceiling
   below the VT-21 floor and will then correctly fail
   `store_accepts_the_guaranteed_minimum_payload`
   (`crates/happenstance-testkit/src/suite.rs:3775`); the two rules are independent.
7. *How the job behaves where the secret is unavailable* — a fork pull request, for
   instance. DR-9 forbids quietly making it non-blocking, so the behaviour must be a
   stated design rather than an emergent one.
8. *Branch lifecycle per run* — one branch per job, per fixture instance, or per
   `connect()`, and how it is torn down. This is the Neon analogue of HS-S0061's
   schema-versus-database isolation question.

**Deferred to the owning stories.**

9. *How the adapter buys ES-10's visibility invariant.*
   `adr-0024-position-visibility-mechanism` (HS-S0065). Neon inherits migration 1
   and the mechanism's column; it does not choose either.
10. *Whether `conflicting_position` is a promise every adapter owes or a hint one may
    omit.* `neon-conflicting-position-verdict` (HS-S0070). This story must not
    pre-empt it by declining a capability that stands in for the answer: there is no
    `Capability` for `conflicting_position`, and inventing a decline-shaped
    workaround would settle by omission what AC-008 requires be settled by evidence.

## Decision

The problem this slice solves is that Neon's capability limits have only ever been
asserted in a doc comment. The crate says it has no connection, no transaction
handle, no cursor, one round trip per operation and a hard 64 MiB response cap —
and until a fixture states those as `Capability` and `Option<usize>` constants
against a live branch, none of it has been tested and every "skip" the suite might
report is hypothetical. This story writes `NeonFixture`: `connect()` building a
`NeonEventStore` over the real transport, `SECOND_HANDLE` supported by a second
client onto the same branch, deliberate answers for `REOPEN` and `MID_BATCH_FAULT`
with real reasons, three ceilings derived from `MAX_RESPONSE_BYTES` *after* the hex
`bytea` doubling, and a panic message on connect failure that reads as an
environment fault rather than a conformance one. Alongside it, the credentialed
workflow job that actually runs it, a sibling of `gate` rather than a step inside
it, so `cargo xtask ci` stays green on a clean checkout with no credentials. The
spec will cover: the fixture type and its `Store`; each capability constant and its
stated reason; the ceiling derivation; the connect-failure message; the job, its
secret, its trigger, and its behaviour where the secret is absent. No `[FROZEN]`
clause is changed here, so no ADR is owed by this story.

## The wrong implementation

**`NeonFixture` backed by a pooled Postgres connection.** Live HTTP in CI is
awkward — a secret, a branch to create and tear down, latency, an external
dependency that can be down when the build is not — so point the fixture at the
`testcontainers` Postgres HS-S0061 already stands up, either directly through
`sqlx` or, more plausibly and much harder to see in review, through a `SqlTransport`
shim that speaks to a local Postgres and returns Neon-shaped JSON. Every existing
check passes, and passes *better*: `event_store_conformance!` runs to completion,
every rule reports pass or a declared skip, `capability_skips_are_reported` is
satisfied, the job is fast and never flakes, and AC-006's wording — "the run
completes with every rule reporting pass, fail, or a skip carrying the fixture's
stated reason" — is literally true. And the transport axis is now filled at its far
end by a pooled Postgres connection, which is where its **near** end already was
(`RUNBOOK.md:688`, "empty at both ends"). CF-25's residual exposure on ES-11 and
ES-12 is reported as discharged by an instrument that never left the machine. This
is DR-4's named failure, and no automated check in this repository can distinguish
it from the real thing — the only instrument is the job definition itself, which is
why `spec` must make the endpoint, the secret and the branch part of the acceptance
criteria rather than of the setup.

**The arithmetic mutant: ceilings copied from the constant.** Set
`MAX_EVENT_DATA_LEN = MAX_RESPONSE_BYTES`. It looks like the most defensible number
in the file — it is the documented hard cap, named and public. But parameters cross
the wire as JSON and a `bytea` is rendered as a `\x…` hex string that roughly
doubles its size in both directions
(`crates/happenstance-neon/src/transport.rs:91-94`), so the store refuses well below
the stated ceiling. The rule then appends exactly the stated number of bytes,
requires acceptance, and gets a failure — reported as a **defect in the adapter**,
because a ceiling is a fact the store asserted about itself. The mirror image, a
number set far too low "to be safe", fails the opposite rule by making the store
refuse something it would have accepted, and quietly drops Neon below VT-21's
guaranteed minimum without anyone deciding to.

**And `Capability::declined("")`**, or a reason of convenience. The empty string is
caught, but only at codegen — `cargo clippy` will not see it
(`crates/happenstance-testkit/src/contract.rs:404-412`), so a green lint run proves
nothing. A *plausible-sounding but untrue* reason is caught by nothing at all, and
it is the shape AC-007 exists to forbid: "no rule is `#[cfg]`-ed out" is the easy
half; "every decline carries a real reason" is the half only review enforces.

One mutant this story cannot commit, worth recording as the contrast: declining
`SECOND_HANDLE`. It is a MUST, and
`two_handles_observe_each_others_appends` panics with the fixture's own words rather
than skipping (`crates/happenstance-testkit/src/contract.rs:135-161`). The testkit
bites hard where it can; everything above is where it cannot, which is why they are
the ones written down.

This story defines a fixture, not a conformance rule, and its mutants require a live
endpoint — so none belongs in `crates/happenstance-testkit/tests/`, where DR-9 keeps
live infrastructure out of `cargo test --workspace`. Their home is the Neon job's own
assertions in `crates/happenstance-neon/tests/`.

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
