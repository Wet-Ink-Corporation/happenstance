# Completeness critic — what ten reviews of `happenstance` did not look at

Repository: `D:\repos\happenstance` @ `9fd2337`. Date: 2026-08-06.

Ten reviewers examined DCB conformance, peer implementations, ecosystem fit, API guidelines,
correctness, the conformance suite, adapter implementability, ergonomics, packaging, and doc
coherence. Between them they produced ~90 findings and an unusually good set of
"this is right, leave it alone" judgements.

They collectively never opened these questions:

| Category | Mentions anywhere in the repository |
|---|---|
| tracing / metrics / telemetry | 0 (`tracing` appears twice, both times as a *naming precedent* in ADR-0006) |
| cancellation / future-drop semantics | 0 |
| GDPR / erasure / redaction / tombstone / crypto-shredding | 0 |
| retention / archival / compaction / log truncation | 0 |
| multi-tenancy / partitioning | 0 |
| fuzzing / miri / loom / coverage / mutation testing | 0 running (`cargo mutants` output is *gitignored* at `.gitignore:12-14`) |
| benchmarks | 0 |
| SECURITY.md / CODE_OF_CONDUCT / issue templates / dependabot | 0 |
| the word "immutable" applied to the log | 0 |

Everything below is measured or quoted. Probes live in
`…\scratchpad\probe\src\main.rs` and `…\scratchpad\probe\src\bin\lockstall.rs`.

---

## 0. What is right, in one line each

- **`unsafe_code = "forbid"`** at `Cargo.toml:42`, opted into by all eight members
  (`[lints] workspace = true` verified in every manifest). This is the single strongest
  trust signal the project has and it is stated nowhere a user will read it.
- **CI uses `pull_request`, not `pull_request_target`** (`ci.yml:6`) — the safe trigger.
  Many foundational crates get this wrong.
- **`deny.toml` is correct**: `yanked = "deny"`, `wildcards = "deny"`,
  `unknown-registry/unknown-git = "deny"`, a tight SPDX allowlist. `cargo deny check` passes.
- **`ProjectionStore`'s transactional invariant** (`projection.rs:13-30`) is the right
  invariant, correctly identified, and correctly forced into the type system. The GAT
  problems other reviewers found are about *how*, not *whether*.
- **Opaque `Bytes` payloads make crypto-shredding possible** — see finding B3; ADR-0003
  bought a privacy capability it does not know it bought.
- **The DCB bet itself is sound.** See §5. My disagreement is about which part of the
  project is the differentiator, not about whether DCB is.

---

## 1. Cancellation and unknown outcomes

### A1 — Nothing in the contract says whether `append` is cancel-safe, and the error type cannot say "I don't know"

`EventStore::append` (`store.rs:141-145`) is an `async fn`. Its future can be dropped:
`tokio::time::timeout`, `tokio::select!`, a cancelled HTTP request, `JoinHandle::abort`,
a client disconnect. Every one of those is routine in the deployment this library was
written for. The contract's `# Atomicity` paragraph (`store.rs:130-133`) says "either every
event lands or none does" — which is about *partial* batches, not about *cancellation*.
It does not say what a dropped future leaves behind.

The reference store answers the question by accident: `MemoryEventStore::append`
(`memory.rs:184-224`) contains no `.await` at all, so it is atomic with respect to drops.
Measured:

```
dropped before first poll: log len 0
timeout(0):     ok=true  | log len 1
```

Every realistic adapter is different:

- **rusqlite via `spawn_blocking`** — dropping the `JoinHandle` does **not** cancel the
  blocking closure. The `COMMIT` executes; the caller is told nothing. The write landed and
  the position is lost.
- **A Durable Object / network store** — the request may or may not have been processed.
- **`sqlx`** — dropping a future mid-`COMMIT` leaves the connection in an indeterminate
  state; sqlx's own docs warn about this.

Worse, `AppendError` (`error.rs:150-166`) has three variants and none of them means
*"the outcome is unknown."* `Store(E)` is documented as "the adapter failed for its own
reasons" and the crate's headline design note (`lib.rs:49-52`) tells callers that the type
system separates "retry the decision" from "something broke". For an event store the
distinction that actually matters is a third one: **"definitely not written, safe to retry"**
versus **"unknown, a retry may duplicate."** A socket timeout is the second; a malformed
statement is the first. Today they are the same variant.

**Recommendation.** Two changes, both free before 0.1.

1. State the obligation on `EventStore::append`:
   > *Cancel safety.* Dropping the returned future before it completes must leave the store
   > in a state the caller can recover: either the whole batch is durable or none of it is.
   > An adapter that cannot guarantee this — because it hands work to a thread pool or a
   > remote — must say so in its own documentation, and callers must treat a dropped append
   > as an unknown outcome.
2. Add the variant the taxonomy is missing:
   ```rust
   /// The append may or may not have been applied. The caller must
   /// re-read before deciding what to do.
   Indeterminate(E),
   ```
   `AppendError` is `#[non_exhaustive]` so this is additive later — but the *adapters* will
   have been written to squash everything into `Store(E)` by then, and a variant nobody
   produces is worse than none. Decide it while zero adapters exist.

Then add the one conformance rule that is writable today:
`append_is_atomic_under_cancellation` — build the future, poll it once with a no-op waker,
drop it, and assert the log is either unchanged or fully written. `MemoryEventStore` passes
trivially; a `spawn_blocking` adapter fails it, which is the point.

**Severity: high. Kind: gap. Blocks first publish: yes** (the error variant does; the doc
sentence is free either way).

### A2 — DCB gives retries an at-most-once guarantee for free, but only for one condition shape, and nothing in the crate names it

This is the answer to A1, and it is the most valuable unstated property in the design.

When an append's outcome is unknown, the caller retries. Whether that duplicates depends
entirely on whether the batch's own events match the condition's query. Measured
(probe 1, `MemoryEventStore`):

```
A (condition query MATCHES the events being written): retry -> REJECTED (safe)  | log len 1
B (condition query does NOT match them):              retry -> ACCEPTED (dup!)  | log len 3
C (condition = None):                                 retry -> ACCEPTED (dup!)  | log len 2
```

Shape A is safe *structurally*: attempt 1 wrote an event matching `fail_if_events_match` at a
position above `after`, so attempt 2's identical condition is violated. The store rejects it,
the caller re-reads, sees its own event, and learns the first attempt succeeded. **A
self-matching conditional append is idempotent under retry with no idempotency key, no
dedup table, and no event id.** That is a genuinely strong property and it falls straight out
of DCB.

Shape B is a perfectly legal DCB command — "append `StudentSubscribed`, provided nobody has
changed the capacity since I read it" — and it silently double-writes. Shape C always does.

All three handlers in `examples/course-subscriptions/src/main.rs` (`define_course` :86,
`subscribe` :113, `unsubscribe` :177) happen to be shape A, because each writes an event of a
type its own query lists. That is **accidental**: nothing in the example, the docs, or the
suite says the property exists, and a fourth handler written the obvious way could be shape B.

**Recommendation.**

- Name it in `AppendCondition`'s docs, next to "The usual shape" (`append.rs:18-32`):
  > **Self-matching conditions are retry-safe.** If every event in the batch would itself
  > match `fail_if_events_match` above `after`, then re-submitting the batch after an
  > unknown outcome is rejected rather than duplicated. Prefer this shape: it is the
  > cheapest exactly-once mechanism available and it needs no event identity.
- Add a conformance rule `condition_makes_a_self_matching_retry_idempotent`: append a batch
  under a self-matching condition, append the identical batch under the identical condition,
  assert `ConditionViolated` and a log length of one batch. It pins a property real callers
  will depend on and no rule covers.
- Give `happenstance-runtime`'s command loop (RUNBOOK phase 3) a `debug_assert` — or better,
  a returned warning — when the emitted events do not match the condition query. That is the
  same check as the query/fold divergence hazard another reviewer found, from the other side.
- Record it in the ledger alongside the phase-6 idempotent-ingest row, because it is the same
  question one layer down.

**Severity: high. Kind: gap (doc + rule). Blocks first publish: yes — it is one paragraph.**

---

## 2. Security: resource limits and the untrusted-input path

### B1 — The contract bounds the two things no store cares about and leaves unbounded the three that cost real money

`MAX_EVENT_TYPE_LEN = 255` (`event.rs:14`) and `MAX_TAG_LEN = 255` (`tag.rs:14`) exist and are
justified ("so storage adapters can index tags in a fixed-width column"). There is no bound on:

| Unbounded | Measured, accepted without complaint |
|---|---|
| `Event::data` payload size | 67,108,864 bytes |
| tag **cardinality** on one event | 100,000 tags |
| `Query` item count | 50,000 items |
| batch length in `append` | unbounded by construction |

This is exactly backwards. The two bounded fields are short identifiers that no engine
struggles with. The unbounded ones are the DoS surface. Concretely: SQLite's default
`SQLITE_MAX_LENGTH` is 1,000,000,000 bytes and `SQLITE_MAX_VARIABLE_NUMBER` is 32,766 — so an
`Event` the contract cheerfully validates is rejected by the flagship adapter with an opaque
engine error, at write time, after the caller has already decided.

### B2 — An expensive append condition is evaluated **while holding the write lock**, so a large query is a write-availability DoS

`MemoryEventStore::append` takes the write lock at `memory.rs:195` and evaluates
`is_violated_by` against every stored event at `memory.rs:197-209` — still holding it.
Measured (`lockstall.rs`, 2,000-event log, a 20,000-item condition built entirely through
public constructors):

```
trivial append on an idle store : 58.7 us
hostile conditional append      : 123.4 ms
concurrent TRIVIAL append       : 123.5 ms   <- blocked behind the write lock
```

A ~2,100x stall of *every other writer*, from one legal call. And this is not a reference-store
artefact: RUNBOOK:64 has already decided the SQLite strategy is "`BEGIN IMMEDIATE` + a probe
returning the conflicting position", which has the identical shape — an unbounded query
evaluated inside the exclusive write transaction. The cost is `O(events × items)` and the
probe is the *only* thing between a caller and the write lock.

The exposure is not hypothetical. `happenstance-sync` (`happenstance-sync/src/lib.rs:38-43`)
names "whether ingest re-checks append conditions" as *the* central design question of that
crate. If the answer is yes, a peer-supplied `Query` — deserialised through
`Query::deserialize` (`query.rs:314-321`), which validates *semantics* but bounds *nothing* —
goes straight into the write path of the receiving instance.

### B3 — `serde` validates after allocating

`Tag::deserialize` (`tag.rs:304-309`) is `String::deserialize(d)?` **then** `Tag::new(raw)`.
The 255-byte check happens after the full string is materialised. Measured: an 8 MiB JSON
string is built, then rejected in 2.4 ms with *"a tag must be at most 255 bytes, got 8388608"*.
Same pattern for `EventType` (`event.rs:315-320`) and for `Bytes` (no bound at all).

For `serde_json` over an in-memory buffer the amplification is bounded by input size. For a
length-prefixed format read from a socket — `bincode`, and `postcard` over a streaming
reader, both of which are the natural choices for `happenstance-sync`'s Worker-side wire — the
attacker controls a length prefix that becomes a `Vec::with_capacity`. That is the classic
deserialisation bomb, in the one code path whose entire purpose is ingesting bytes from
another machine.

**Recommendation for B1–B3.** These are one change with three parts.

1. **Add the missing constants and validate them**, mirroring what already exists:
   ```rust
   /// Largest permitted payload, in bytes. Chosen to sit under SQLite's
   /// SQLITE_MAX_LENGTH and Cloudflare's request limits with room to spare.
   pub const MAX_EVENT_DATA_LEN: usize = 16 * 1024 * 1024;
   /// Largest permitted number of tags on one event.
   pub const MAX_TAGS: usize = 64;
   /// Largest permitted number of items in a query.
   pub const MAX_QUERY_ITEMS: usize = 128;
   ```
   Raising a limit later is non-breaking; lowering one is not — so err generous, but *have*
   the number. `Event::new` becomes fallible on payload size (it is already fallible), and
   `Query::from_items` / `Tags::from_iter` gain a bound. Note this makes `FromIterator<Tag>
   for Tags` need a fallible sibling, which is a small API question worth settling now.
2. **Write the bounds into `Tag`/`EventType`/`Bytes` deserialisation as a `Visitor`** that
   checks `visit_str`'s length *before* constructing an owned `String`. Ten lines each, and it
   turns a 8 MiB allocation into a 255-byte rejection.
3. **Say in `EventStore::append`'s docs that the condition query is evaluated inside the
   write path**, so adapter authors know the cost is on the critical section, and add
   `Query::arm_count()`-style accounting (another reviewer proposes this for pushdown; it is
   the same helper) so an adapter can refuse an over-large condition with a typed error
   rather than stalling.

**Severity: high (B2), medium (B1, B3). Kind: security / design-risk. Blocks first publish:
yes — the constants are public API and their absence is what freezes.**

---

## 3. Data lifecycle: the three questions an append-only log always gets asked

### C1 — The log is never stated to be immutable, and there is no erasure story; the tag index is the part crypto-shredding cannot reach

Search the whole repository for `immutab`, `delete`, `redact`, `tombstone`, `GDPR`, `erasure`:
zero hits in the contract, the ADRs, and the suite. The most fundamental property of an event
store — *an appended event never changes and never disappears* — is **assumed everywhere and
stated nowhere**, and consequently no conformance rule checks it. An adapter that
silently rewrote a payload would pass all 27 rules.

That is not pedantry, because the reason to state it is that you will eventually need to
break it. GDPR Art. 17 erasure against an append-only log has exactly three known answers:

| Approach | Does `happenstance` permit it today? |
|---|---|
| **Crypto-shredding** — encrypt the payload per data subject, destroy the key | **Yes, and elegantly.** `Event::data` is opaque `Bytes` (ADR-0003) and the contract never parses it, so ciphertext is indistinguishable from plaintext to every adapter. This is a real, unclaimed benefit of ADR-0003. |
| **Tombstone + rewrite in place** — replace the payload, keep the position | **Unknown.** Nothing permits it, nothing forbids it, and no port method expresses it. |
| **Copy-and-transform the whole log** — rebuild without the subject | **Effectively no.** Positions would be reassigned, invalidating every persisted `AppendCondition::after` and every `ProjectionStore` checkpoint. |

And here is the part that is specific to *this* design and that nobody has noticed:
**crypto-shredding cannot cover tags.** `student:s1` is a tag — a plaintext, indexed,
queryable value that is very often a direct personal identifier, and it *must* stay plaintext
because `Tags::contains_all` (`tag.rs:231-245`) is the query mechanism. Encrypt the tag and
DCB stops working. So the one erasure technique the payload design enables leaves the
identifier in the index, permanently, in exactly the column an adapter is told to build an
index on (`tag.rs:11-14`).

There is also an unrecognised coupling to a decision the ledger has already made.
RUNBOOK:67 records event identity as **decided**: a Lamport pair `(origin, origin_position)`,
chosen over a content hash for replication reasons. That choice is *also* the erasure-friendly
one — a content hash makes any redaction indistinguishable from corruption and breaks every
peer's dedup — and the ledger row does not know it made a privacy decision. Write it down
before the ADR lands, because "we picked the pair for sync reasons and it happens to survive
redaction" is a much weaker record than "we picked it for both."

**Recommendation.**
- One sentence on `Event` and one conformance rule: an event, once appended, is byte-identical
  on every subsequent read. `event_is_immutable_across_reads`: append, read, append something
  unrelated, read again, assert equality. Ten lines.
- One short ADR — *Erasure and the append-only log* — that states: (a) the payload is
  opaque so crypto-shredding is the supported mechanism; (b) **tags are plaintext and are
  therefore not erasable**, so callers must not put unpseudonymised personal data in a tag,
  use a surrogate key; (c) whether an adapter MAY offer an out-of-band redaction that
  preserves positions, and if so that it must preserve `SequencePosition` exactly.
- Add the tag guidance to `Tag`'s docs. `Tag::key_value("student", "s1")` is the crate's own
  example (`tag.rs:28`) and it is exactly the shape that becomes a compliance problem.

**Severity: high. Kind: gap / design-risk. Blocks first publish: yes for the doc sentence
and the ADR; the rule is trivial.**

### C2 — Retention and truncation have no representation, so a truncated log is indistinguishable from a complete one

There is no `earliest_position()`, no `TruncatedBefore` error, and no way for a store to say
"history below position P is gone." Every store that runs for years eventually needs one:
retention policies, archival to cold storage, a local-first device that keeps only recent
history and syncs the rest.

The failure is silent and it is the worst kind. `read_decision_model` (`store.rs:198-208`)
reads with `ReadOptions::new()` — from the beginning, unbounded — folds whatever comes back,
and hands the caller a position to condition on. If the beginning has been truncated, the
decision model is built from a **partial history** and the append succeeds. A capacity check
that cannot see the old `StudentUnsubscribed` events oversells. Nothing errors; nothing warns.
Identically for a `ProjectionStore` resuming from a checkpoint below the truncation point
(`projection.rs:86-88`).

Note the interaction with the position contract: `SequencePosition` positions are unique
"across the store" and gaps are permitted (`event.rs:92-97`). Truncation is therefore *already
legal* under the conformance suite — `positions_are_unique` and
`positions_are_strictly_monotonic` (`suite.rs:336-365`) both hold on a truncated log. The
suite says truncation is fine; the contract has no way for a caller to find out it happened.

**Recommendation.** Do not build retention now — but reserve the seam and the ledger row.

```rust
/// The lowest position still retained, or `None` if the store has never
/// discarded history. A caller whose `ReadOptions::from` is below this, or
/// whose read began below it, has an incomplete history and must not
/// treat the result as a decision model.
fn earliest_position(&self) -> impl Future<Output = Result<Option<SequencePosition>, Self::Error>>;
```

That is a provided method with a default body returning `Ok(None)` — which is why the
`trait_variant::make(SendEventStore: Send + Sync)` fix another reviewer identified is a
prerequisite, and why the two should land together. If the seam cannot be added now, at
minimum add a RUNBOOK ledger row and one sentence to `EventStore`: *"this contract assumes a
complete log; retention and archival are not yet expressible and an adapter MUST NOT discard
events."*

**Severity: medium. Kind: gap. Blocks first publish: no — but the ledger row does.**

### C3 — Multi-tenancy has exactly one possible answer and nothing says which

There is no tenant concept, so a tenant is either a tag or a whole store. Both work; they
have opposite consequences and the docs pick neither:

- **Tenant-as-tag.** Every query must carry the tenant tag or it leaks. `Query::all()` — used
  by the suite constantly and by `read_decision_model` implicitly whenever a caller passes it —
  crosses every tenant. An `AppendCondition` built on `Query::all()` is a global write lock
  across all tenants. And nothing can enforce that every `Event` carries the tag, because
  `Event::with_tags` is optional (`event.rs:211`).
- **Store-per-tenant.** Natural for SQLite (one file), impossible to query across, and it
  multiplies the connection pool by the tenant count.

**Recommendation.** One paragraph in the crate docs and a `Tags` convention: state that a
tenant is a tag by convention (`tenant:acme`), that the typed layer's `DecisionModel` should
inject it into every query and every event so it cannot be forgotten, and that
`Query::all()` is a **single-tenant or administrative** operation. Free, and it prevents the
first multi-tenant user inventing something incompatible. Add the ledger row.

**Severity: medium. Kind: doc. Blocks first publish: no.**

---

## 4. The instruments the project's own thesis demands

### D1 — Adding a conformance rule is a semver-*minor* change that turns every passing adapter's CI red. The testkit has no compatibility policy, and it is the moat.

This is the finding I would put first if I had to pick one, because it attacks the
differentiator.

`happenstance-testkit` will be published (`crates/happenstance-testkit/Cargo.toml`, no
`publish = false`). A third-party adapter writes `happenstance-testkit = "0.1"` — the caret
default. Any `cargo update` picks up 0.1.5. If 0.1.5 adds the seven Tier-1 rules that another
reviewer has (correctly) shown the suite needs, that adapter's CI breaks on a day when the
adapter did not change. Cargo says this is a compatible upgrade. Every adapter author will
experience it as a breaking one.

The consequence is worse than an annoyance: it creates a standing incentive **not to
strengthen the suite**, which is precisely the thing the project must do continuously for the
suite to be worth anything. A conformance suite that cannot get stricter is a badge, not a bar.

Nothing in `README.md`, `CONTRIBUTING.md`, or the RUNBOOK addresses this. `CONTRIBUTING.md:54-64`
tells contributors how to *write* a rule and says nothing about the compatibility of *adding*
one.

**Recommendation.** Publish a testkit compatibility policy before the first adapter ships,
and put it in the crate docs where an adapter author will read it:

> **New rules may appear in any release, including a patch.** A conformance suite that
> cannot get stricter is worthless. Pin `happenstance-testkit` exactly (`= "0.1.3"`) if you
> need a stable bar for a release branch, and treat a new failing rule as a bug report about
> your adapter, not about the suite.

Two mechanical supports worth building at the same time:
- **A rule registry with a version tag per rule** — the registry macro is already scheduled
  for phase 1 (RUNBOOK:72). Adding `since = "0.1.4"` to each entry costs nothing and lets an
  adapter's CI report *"3 new rules since the version you pinned"* rather than a bare failure.
- **Separate the suite's version from the contract's.** They are currently locked together by
  `version.workspace = true`. The contract should move slowly; the suite must move fast. Give
  `happenstance-testkit` its own version key now — it is one line and impossible later.

**Severity: high. Kind: process. Blocks first publish: yes.**

### D2 — `cargo-mutants` is gitignored and never run, and it is the automated form of the experiment that found this suite's biggest weakness

`.gitignore:12-14` reserves `**/mutants.out*/`. Nothing in the workspace runs mutation
testing; there is no `cargo-mutants` in the gate, in CI, or in `CONTRIBUTING.md`. So the tool
was considered and dropped.

It should not have been, because it is the exact instrument this project's thesis calls for.
One reviewer hand-built six deliberately-broken adapters to measure whether the 27 rules
discriminate, and found four passed. That is `cargo-mutants`, done manually, on a sample of
six. Run against `crates/happenstance` with the conformance suite as the test command, it
does the same thing exhaustively and reports every mutation of the contract's own semantics
that no rule detects. For a project whose stated position is *"a claim about behaviour is
worth exactly as much as the test that checks it"* (`testkit/src/lib.rs:3-4`), shipping
without ever measuring the test is the one inconsistency that matters.

The other three instruments, ranked honestly:

- **`loom`** — the reference store is the conformance oracle and its atomicity claim
  (`memory.rs:189-194`) is asserted in a comment. A ~40-line loom test of interleaved
  `append`/`read` would turn that comment into a proof. Worth doing; it is the *oracle*.
- **Coverage (`cargo-llvm-cov`)** — measuring the conformance suite's line coverage of
  `crates/happenstance` tells you which contract code paths no rule reaches. Cheap, and it
  is the same instrument as D2 at lower resolution.
- **`miri`** — low value here and worth saying so, because it is the reflexive suggestion.
  `unsafe_code = "forbid"` means there is no UB of the project's own to find; miri would only
  exercise `bytes`' internals, which upstream already tests. **Skip it.**
- **`cargo-fuzz`** — moderate value, and specifically on `Tag::new` / `EventType::new` /
  the `serde` `Deserialize` impls once B3 is fixed, because those are the only functions that
  parse untrusted bytes. A structure-aware round-trip fuzz target
  (`serialize(deserialize(x))` idempotence) is a day's work and covers the wire format that
  currently has zero tests.

**Recommendation.** Add `cargo mutants --package happenstance` as an optional xtask step now
(it fits the existing `probe:`-based skip mechanism at `xtask/src/main.rs:30`, alongside
`hack` and `deny`) and a weekly CI job. Record the baseline score in the README next to the
"27 rules" claim. *"27 rules, and they kill 94% of mutations of the contract's own
semantics"* is a far stronger claim than a count, and no competing crate can make it.

**Severity: medium. Kind: process. Blocks first publish: no — but the baseline number is
worth having before the first adapter is written against the suite.**

---

## 5. Observability, and where the port's shape forces it to live

### E1 — The contract cannot depend on `tracing`, which means the only thing it can standardise is the *names* — and that is worth doing

One reviewer noted tracing is absent from the ledger. The additive point is *why it cannot
simply be added later in the obvious place*, and what the cheap answer is.

The contract crate is `no_std`-capable (`lib.rs:74`), targets wasm32, and has four
dependencies. It will never take `tracing`. So instrumentation must live in each adapter —
and left to itself, every adapter will name its spans differently, and a user with a SQLite
store and a Durable Object store will have two incomparable trace shapes for the same
operation.

There is also a shape constraint nobody has spelled out. `read` is **not** `async` and returns
the stream at the top level (`store.rs:117-121`) — the single most load-bearing decision in
the design. That means `#[tracing::instrument]` on `read` opens a span that **closes before a
single event has been read**, because the function returns as soon as the stream is
constructed. An adapter must instrument `poll_next` instead, and must not hold an `Entered`
guard across polls. That is the classic tracing footgun, and here the port makes it
mandatory rather than optional. An adapter author who reaches for the attribute macro gets a
span that measures nothing and will not notice.

**Recommendation.** Write *semantic conventions* into the contract crate's documentation —
prose, zero dependencies, zero API surface, and the thing that makes traces comparable across
adapters:

| Span / field | Meaning |
|---|---|
| `happenstance.append` | one call to `EventStore::append` |
| `happenstance.event_count` | batch length |
| `happenstance.conditional` | whether a condition was supplied |
| `happenstance.position` | the position returned, on success |
| `happenstance.condition_violated` | `true` on the DCB rejection |
| `happenstance.read` | one `read`, **opened on first poll and closed on stream end** |
| `happenstance.query.items` / `query.all` | query shape, for pushdown debugging |

Plus one sentence of policy that will otherwise be got wrong everywhere:
**`ConditionViolated` is not an error-level event.** It is the routine DCB outcome
(`error.rs:91-92` says so). Adapters that log it at `ERROR` will produce a page-per-retry
under contention. It belongs at `DEBUG`, with a counter.

Then add the `tracing` feature to `happenstance-sqlite` and the phase-3 command loop as an
optional, off-by-default feature that follows those names. Costs nothing in the contract and
makes "production-ready" a checkable claim rather than a hope.

**Severity: medium. Kind: gap. Blocks first publish: no — but the conventions section is
an hour and belongs in 0.1's docs, because that is when adapter authors read them.**

### E2 — `read` is a synchronous function on an async port, and nothing forbids it from blocking the executor

`MemoryEventStore::read` (`memory.rs:150-182`) acquires a `std::sync::RwLock` read guard,
filters, clones, and collects — synchronously, in a function a caller invokes from an async
context. On the reference store this is fine by fiat ("not built for scale", `memory.rs:25`).
On an adapter it is a reactor stall, and the reference implementation is the example every
adapter author will copy.

The contract already says `read` is lazy (`store.rs:103-105`) and other reviewers have
correctly attacked that as untrue of the oracle. The complementary obligation is the one
nobody stated: **`read` must not block.** All I/O, all locking, all `spawn_blocking` belongs
in `poll_next`. That is not a nicety — one reviewer verified that `tokio::task::spawn_blocking`
*panics* if called at `read` time outside a runtime, so laziness is load-bearing for
correctness, not just for memory.

**Recommendation.** One sentence on `EventStore::read`: *"`read` itself must not block, do
I/O, or acquire a contended lock. It constructs a stream; all work happens in `poll_next`."*
Then fix the oracle to match, which is the same change the read-laziness findings already ask
for.

**Severity: medium. Kind: doc. Blocks first publish: yes — one sentence.**

### E3 — There is no panics-vs-errors policy for adapters, and no rule that a store survives a failure

`missing_panics_doc = "warn"` is on (`Cargo.toml:55`) and `unwrap_used = "warn"`, so the
*crate's own* discipline is good. But the port says nothing about what an **adapter** may
panic on, and the suite has no rule that a store is still usable after a failed operation.
Concretely: after `AppendError::Store(e)` from a SQLite adapter whose transaction aborted, is
the store usable? After a `ProjectionStore::Batch` is dropped without commit, is the store
usable? (One reviewer found that in their rusqlite probe it returns `Busy` **forever** — that
is the same class of bug, discovered in the other port.)

**Recommendation.** One paragraph in `CONTRIBUTING.md`'s "Writing an adapter" section:
adapters return `Err`, never panic, for every condition a caller can trigger — invalid input,
contention, I/O failure, resource exhaustion. Panicking is reserved for broken invariants of
the adapter's own. And one rule: `store_is_usable_after_a_failed_append` — trigger a
`ConditionViolated`, then perform a successful unconditional append and read it back.

**Severity: low. Kind: process. Blocks first publish: no.**

---

## 6. Governance and supply chain

### F1 — CI runs a mutable branch reference and has no `permissions:` block

`.github/workflows/ci.yml:52-54`:

```yaml
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: "1.85"
```

`@master` is a *branch*, not a tag — the workflow executes whatever is at that branch's tip at
run time. The other four actions (`actions/checkout@v4`, `Swatinem/rust-cache@v2`,
`taiki-e/install-action@v2`, `obi1kenobi/cargo-semver-checks-action@v2`) are on mutable major
tags, which is conventional but is not what a foundational crate should do. The workflow also
has **no `permissions:` key**, so every job runs with the repository's default `GITHUB_TOKEN`
scope. And `taiki-e/install-action` downloads prebuilt `cargo-hack` and `cargo-deny`
binaries from GitHub releases and runs them over the source tree.

None of this is exotic; all of it is on every supply-chain checklist; none of it costs
anything to fix.

**Recommendation.**
```yaml
permissions:
  contents: read
```
at the workflow level, SHA-pin all five actions with a `# v4.2.2`-style comment, and switch
`@master` to a tag. Add `--locked` to the gate invocation so CI tests the pinned dependency
set. When phase 7 arrives, publish via **crates.io Trusted Publishing (OIDC)** rather than a
long-lived `CARGO_REGISTRY_TOKEN` — for a crate positioning itself as infrastructure, the
provenance of the published artefact is part of the product. There is no release workflow
today; write it with OIDC from the start rather than retrofitting.

**Severity: medium. Kind: process. Blocks first publish: the release workflow does.**

### F2 — None of the artefacts that signal "safe to depend on" exist, and the strongest signal the project has is invisible

No `SECURITY.md`, no `CODE_OF_CONDUCT.md`, no issue or PR templates, no `dependabot.yml`,
no `CHANGELOG.md`, no `homepage` key, no advisory-only scheduled job. More pointedly:

- **`unsafe_code = "forbid"`** is set at `Cargo.toml:42` and applied to all eight members —
  and it appears **nowhere** in `README.md`. It is the single most persuasive fact about this
  codebase for a sceptical evaluator and it is buried in a manifest.
- **The MSRV policy** lives in ADR-0004, which users do not read. The README does not state
  the MSRV at all.
- **What semver covers** is unstated: are the conformance rules covered? Is
  `MemoryEventStore`'s exact position assignment? Is `Query`'s exhaustiveness (which one
  reviewer correctly identified as a *deliberate* breaking-change acceptance)?
- **Bus factor is one.** `authors = ["Ryan Britton"]`, four commits, one contributor. For a
  crate asking to be "the premiere framework for event sourced systems", that is the risk a
  serious adopter weighs first, and the honest mitigation — the conformance suite means an
  adapter survives the maintainer, the dual licence means a fork is legitimate — is a
  paragraph the project can write and currently does not.

**Recommendation.** A "Guarantees" section in the README, above "Prior art":

> - `#![forbid(unsafe_code)]` across every crate in the workspace, enforced by CI.
> - MSRV 1.85, raised only in a minor release and called out in the changelog.
> - Semver covers the public API of `happenstance` and `happenstance-testkit`. **New
>   conformance rules may appear in any release** — see the testkit docs.
> - Security reports: <address>. See `SECURITY.md`.
> - Four runtime dependencies: `bytes`, `futures-core`, `thiserror`, `trait-variant`.

Then the files: `SECURITY.md` (one paragraph and an address), `CODE_OF_CONDUCT.md`
(the Rust CoC verbatim), `dependabot.yml` for `cargo` and `github-actions`, and a
`CHANGELOG.md` started today with `## [Unreleased]` — ADR-0004 already promises MSRV bumps
are "called out in the changelog", against a file that does not exist.

**Severity: medium. Kind: process. Blocks first publish: yes for SECURITY.md, the licence
files (already found by another reviewer), and the guarantees section.**

---

## 7. The unexamined premise: is the testkit the moat, or is the typed layer?

The brief asks for both steelmen and a verdict. Here they are, and then a third answer that I
think is the right one.

**The case that the testkit is the moat.** The DCB specification is public and small; the
contract crate is ~1,500 lines a competent Rust developer could reproduce from the spec in a
week. What cannot be reproduced in a week is a *reference-validated behavioural oracle* plus a
population of adapters that have all cleared it. It compounds: every third-party adapter that
goes green is an asset the project did not pay for, and it makes the next one cheaper. It is
also a direct answer to the observed failure mode of this niche — `thalo` unmaintained,
half-working adapters, no shared verification. And it is legible: a sceptical evaluator reads
27 rule names in sixty seconds and forms a judgement, which is a *marketing* asset as much as
an engineering one.

**The case that the typed layer is the moat.** Nobody chooses a library for its test suite.
They choose it for the code they write every day, and that code lives entirely above the port:
`DecisionModel`, `Codec`, the command loop, the retry policy, the derive. `disintegrate`
already ships DCB in Rust with `#[derive(StateQuery)]`; if happenstance ships a better contract
and a worse daily-driver experience, users pick disintegrate and never learn the contract was
better. The conformance suite is consumed by *adapter authors* — a population ADR-0006 itself
describes as "small, and sophisticated". The typed layer is consumed by everyone.

**My verdict: both are real and neither is the differentiator, and the roadmap has the
ordering backwards for a third reason.**

The testkit is a **credibility** moat: it buys the first hundred users among the people who
are right to be sceptical. It is not an **adoption** moat, because nobody's daily work touches
it. The typed layer is the adoption moat and has no credibility without the contract beneath
it. They are complements, not alternatives, and the project needs both — which it knows.

What the project appears not to know is that **neither is what makes it unique.** Look at what
this library is actually built for, stated plainly in the stubs:
`happenstance-sync/src/lib.rs:6-11` — a local-first application holding its own store on the
device, syncing with SQLite inside a Cloudflare Durable Object. The `!Send`/`Send` two-flavour
port (ADR-0001) is the hardest engineering in the repository and it exists *entirely* to serve
that. No other Rust event-sourcing library can run in a Worker at all: `cqrs-es` still
recommends `async-trait`, `disintegrate` is Postgres-bound, `evento` is server-side. DCB is a
good bet that `disintegrate` has already validated. A conformance suite is good practice that
anyone can copy. **"The event sourcing library for local-first Rust applications that sync"
is a position nobody else occupies**, and it is the one thing here a competitor cannot
replicate by reading the README.

The RUNBOOK schedules the wasm proof at phase 5 and replication at phase 6 — last and
second-to-last, behind two adapter phases and the typed layer. The differentiator is the thing
that gets built last and is proven least. Meanwhile the README leads with DCB (`README.md:15-53`,
where the competitor is named at :161) and the tagline promises "batteries" (`README.md:5`)
that phase 3 does not schedule.

**Recommendation.** This is a positioning decision, not a code change, but it should be made
before 0.1 because it re-orders everything downstream:

1. **Pull the `!Send`/wasm proof forward to phase 1.5**, as the ledger row at RUNBOOK:72
   already half-decides — a `RefCell`-backed `!Send` reference store plus the rule registry,
   run under `wasm-bindgen-test`. Roughly two days, and it converts the project's actual
   differentiator from a claim into a demonstration before any adapter ossifies.
2. **Make the testkit strong and give it a strengthening policy now** (D1, D2). It is the
   current differentiator and it is measurably weak; both are cheap to fix and neither
   competes with anything else on the roadmap.
3. **Settle the typed layer's *shape* before the SQLite adapter**, for the reason another
   reviewer gives (it is what discovers contract defects) *and* for mine (it is what decides
   adoption).
4. **Rewrite the README's lead.** DCB is the *mechanism*; local-first-that-syncs is the
   *position*. `disintegrate` owns "DCB in Rust" today and the README currently competes with
   it head-on, on its ground, from behind.

**Severity: medium. Kind: strategy. Blocks first publish: no — but it should be settled
before the README is frozen for 0.1.**

---

## Categories that are genuinely fine

- **Backpressure.** `Stream`'s poll model gives natural per-item backpressure and a
  `read` consumer sets its own pace. Nothing to add beyond the `limit(0)` bug others found.
- **Zeroization / secrets in memory.** Not applicable at this layer — payloads are opaque
  `Bytes` the contract never inspects, and a caller who needs `zeroize` applies it to the
  plaintext before it becomes an `Event`. Correct division of labour; leave it.
- **Structured logging beyond tracing.** `tracing` subsumes it; no separate concern.
- **Lint and dependency hygiene.** `unsafe_code = "forbid"`, opted into by all eight members;
  four healthy runtime dependencies; `deny.toml` correct; `cargo deny check` green. The only
  gap is that none of this is *said* anywhere a user will see it (F2).
