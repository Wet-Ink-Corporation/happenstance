---
item: HS-S0050
stage: discover
created: 2026-08-12T13:02:12.274Z
updated: 2026-08-12T13:02:12.274Z
template_sig: 86ce4036
rendered_sig: 715f3345
---

# Discover — Append, head, contains_event_id and migrate against a real Durable Object

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one-line: implement `migrate`, `append`, `head` and `contains_event_id` against the real bindings — identity columns, ADR-0014 store-id incarnation, DCB conflicts classified into `AppendError::ConditionViolated` *before* `Self::Error` exists, capacity refusals into `AppendError::ExceedsStoreLimit`, and the 2^53 ceiling surfaced as `StoredPosition` rather than by truncation | `_storymap.md`, **Slices**, `durable-object-write-path` row | Four method bodies, and every one of them is a classification decision as much as a SQL decision |
| AC-001 (no `todo!()` on any adapter path) and AC-007(b) (the 2^53 ceiling reported, never silently passed) | `project.md`, **Acceptance criteria** | This story owns the write half of AC-001 and the position-ceiling half of AC-007 |
| `dependsOn: worker-binding-layer` — it supplies the real `SqlStorage` handle, the synchronous `exec`, and the `Rc`-shaped `worker::Error` this path constructs its failures from | `_storymap.md`, **Slices** and **Merge order** §2 | Without it there is nothing to classify; with it, every refusal path in this story is a real thrown value |
| ARCH-AC-07: no `ConditionViolated` variant is added to the adapter's error; the DCB conflict is classified before `Self::Error` is constructed, and capacity refusals travel as `AppendError::ExceedsStoreLimit` naming a `StoreLimit`, never as `AppendError::Store` and never by truncating | `_decomposition.md`, Architecture brief, **Acceptance Criteria** and Notes §3 | The classification happens early or it cannot happen at all — the error type has no arm for it by construction |
| The absence of a `ConditionViolated` variant is deliberate and documented as a finding made structural | `crates/happenstance-cloudflare/src/event_store.rs:100-104`; `crates/happenstance-cloudflare/src/lib.rs:75-85` | Adding one would look like a fix and would in fact be the defect |
| The four `todo!()` sites this story removes, and the identity columns two of them already name (`origin_store`, `origin_position`) | `crates/happenstance-cloudflare/src/event_store.rs:85`, `:176`, `:184`, `:188`, `:194` | The intended schema predates ingest and must gain the columns before `contains_event_id` can answer at all |
| ADR-0014 makes `contains_event_id` a **required** port method because the provided form would need `Self: Sync`, which this deliberately `!Sync` adapter cannot supply — the ADR names this adapter's flavour by construction | `.kb/decisions/0014-event-identity-and-recorded-time.md`; `_grounding.md` §1 | The method is not optional and not derivable. It is this adapter's obligation specifically |
| The rule asks about a **foreign** identity as well as one this store minted | `crates/happenstance-testkit/src/suite.rs:2551`, `:2575`, `:2594` | A `WHERE position = ?` implementation answers the easy half correctly and the hard half wrongly |
| `CloudflareEventStoreError::StoredPosition` exists for the 2^53 narrowing: Workers SQL widens integers through a JS number, so a `SequencePosition` above `Number.MAX_SAFE_INTEGER` is not round-trippable even though `NonZeroU64` permits it | `crates/happenstance-cloudflare/src/lib.rs:108-112`; `crates/happenstance-cloudflare/src/event_store.rs:136-142` | A declared store limit, reported — never an excused rule and never a truncation |
| ADR-0013 governs position assignment, gaps and reuse, and meets a real narrowing here | `.kb/decisions/0013-position-assignment-and-visibility.md`; `_decomposition.md`, Architecture brief Notes §2 | The ceiling is the store's fact, not the suite's assumption |
| CF-40 requires a stated ceiling to be refused as `ExceedsStoreLimit` naming the corresponding `StoreLimit`, "never as `AppendError::Store`, and never by truncating" | `spec/SPECIFICATION.md:7661-7674`; `crates/happenstance-testkit/src/contract.rs:239-247` | The refusal *channel* is this story's even though the *numbers* are `measured-store-limits`' |
| Append order of refusals is fixed by the suite: an empty batch is refused **before** the condition is evaluated, and a registered mutant already rejects the other order | `crates/happenstance-testkit/src/lib.rs:145`; `crates/happenstance-testkit/tests/mutation_coverage.rs:981-983` (`ConditionBeforeEmptinessStore`) | Ordering is a conformance property, not an implementation detail |
| Atomicity is free here: the whole object is one consistency boundary and `exec` is synchronous, so a `SELECT` for the condition followed by `INSERT … RETURNING position` has nothing awaiting between them | `_decomposition.md`, Architecture brief Notes §3; `crates/happenstance-cloudflare/src/event_store.rs:30-38`; `crates/happenstance-cloudflare/src/sql_storage.rs:1-12` | The usual hard part of DCB append is bought by the runtime. What is left is classification |
| No conformance rule may assert on literal position values; the specification permits gaps and a conformant adapter may leave them | `CLAUDE.md`, **The rule that matters** | Binds the 2^53 boundary test this story writes |

## Questions

**Is the 2^53 ceiling a declined capability, a `StoreLimit`, or a decode error?**
Answered: a **decode error on the read-back path**, reported through
`CloudflareEventStoreError::StoredPosition`. It is not a capacity the store
refuses at write time, so it is not a `StoreLimit`; and `Fixture`'s three
`Option<usize>` ceilings are about payload bytes, tags per event and events per
batch, not positions (`crates/happenstance-testkit/src/contract.rs:214-279`), so
there is nothing to decline. No conformance run will assign 2^53 positions, which
is exactly why it needs a targeted test rather than a rule.

**How is that boundary tested without asserting a literal position?** Answered,
and it is the constraint that shapes the test: seed the position column directly
through `exec` — a store-side fact this adapter is allowed to arrange — and assert
on the **variant returned**, not on a number. The ceiling is the store's declared
fact; the suite's rules stay free to see gaps.

**ADR-0014's store-id incarnation.** Leading answer, confirmed at spec: mint at
`migrate`/first open, persist it in the object's own storage, read it back so
`contains_event_id` can answer for foreign origins, and re-mint only on a
detectable restore or clone (`_decomposition.md`, Architecture brief Notes §2 and
§4a).

**Who owns the `MID_BATCH_FAULT` seam?** Deferred, and named so it is not dropped
between two stories: the *seam* — a `CHECK` constraint or trigger armed for
exactly one write — is a write-path concern, while the *constant* that declares it
is the fixture's (`durable-object-host-and-fixture`) and the decision to claim
`SUPPORTED` is `measured-store-limits`'. Settled jointly at spec.

**Which body takes the `#![allow(clippy::todo)]` with it?** Answered by the story
map: whichever of the write path and the read path lands last removes the last
`todo!()` and the scoped allow together (`_storymap.md`, **Coverage**, AC-001
row).

**CF-39 / CF-40's fixture-limits ownership, and the off-tokio harness shape.**
Not this story's. The refusal channel is; the numbers and the atom are
`measured-store-limits`' and `adr-0023-and-atom-resolutions`', the latter
coordinated with `sqlite-durable-store` (HS-P0012).

## Decision

`migrate`, `append`, `head` and `contains_event_id` are four `todo!()` bodies, and
a `todo!()` type-checks against any signature — which is the whole reason this
crate does not yet count toward the workspace's three-disagreeing-implementations
claim. This slice writes them against the real bindings, and its content is
mostly not SQL: the Durable Object gives away atomicity for free (one consistency
boundary, a synchronous `exec`, nothing awaiting between the condition probe and
the insert), so what is left is the set of decisions about *which channel a
failure travels in*. A DCB conflict must be classified from the thrown error's
message text before `Self::Error` is constructed, because the adapter's error type
deliberately has no arm for it. A capacity refusal must name a `StoreLimit` and
arrive as `ExceedsStoreLimit`, never as `Store` and never as a silent truncation.
A position that will not round-trip through a JS number must arrive as
`StoredPosition`, not as a quietly narrowed integer. And `contains_event_id` must
answer about an identity, which means the schema gains `origin_store` and
`origin_position` before it can answer at all. The spec will cover: the schema
including the identity columns and the store-id incarnation rule; the append
sequence and the fixed order of its refusals; the classification function and its
inputs; each error variant and the caller-visible arm it maps to; and the targeted
2^53 boundary test, written against the store's declared fact rather than a
literal position. Nothing `[FROZEN]` is amended.

## The wrong implementation

**The ceiling nobody declared.** An `append` that maps the Durable Object's SQL
storage cap (`SqlError::StorageLimitExceeded`,
`crates/happenstance-cloudflare/src/sql_storage.rs:119-121`) into
`CloudflareEventStoreError::Sql` and out through `AppendError::Store` — or, one
character worse, `&events[..CEILING]` where `events.chunks(CEILING)` was meant, so
`Ok` comes back carrying a real position and nothing downstream has anything to
notice. Every existing check passes, and it passes for a **structural** reason
rather than by luck: `Fixture`'s three ceilings default to `None`
(`crates/happenstance-testkit/src/contract.rs:253`, `:262`, `:279`), so
`append_reports_exceeded_store_limits` reports a skip through `NO_STORE_LIMITS`
(`contract.rs:442`) and never reaches the store at all. The rule is emitted, it
answers, it prints the fixture's reason — and it is testing nothing. The testkit's
own `MUST_SKIP` list names that rule for exactly this gate and says so:
"its gate is CF-40's three `Option<usize>` ceilings rather than a `Capability`"
(`crates/happenstance-testkit/tests/mutation_coverage.rs:3148-3164`).

**The mutants are already registered, and this story's obligation is to make them
reachable rather than to write new ones.** `PayloadCeilingStore` refuses through
`AppendError::Store` — the channel CF-40 replaced; `TruncatingPayloadStore` does
not refuse at all; `BatchParameterCeilingStore` meets its driver's parameter
ceiling at write time after the caller has taken its side effects; and
`ChunkLosingBatchStore` is the clamp-instead-of-chunk defect verbatim. All four
live in `crates/happenstance-testkit/tests/mutation_coverage.rs:1919-2040` and all
four are registered against `append_reports_exceeded_store_limits`. What this
story owes them is a refusal path that already names a `StoreLimit` before the
fixture's numbers exist, so `measured-store-limits` does not inherit a store that
can only fail.

**And one this story must reject in its own tree.** `contains_event_id`
implemented as `SELECT 1 FROM event WHERE position = ?`, ignoring
`origin_store`. It answers correctly for every identity this store minted and
wrongly for every foreign one — an `EventId` from another store whose position
happens to collide reads as present. `contains_event_id_reports_membership`
reaches exactly that half (`crates/happenstance-testkit/src/suite.rs:2594`) and a
store is already registered against it
(`crates/happenstance-testkit/tests/mutation_coverage.rs:1496`), so the detector
exists; what does not yet exist is a schema that can tell the two apart, and
adding those two columns is this story's.

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

**Box 6.** No conformance rule is added here — the existing enumeration is what
exercises this path. The one targeted test this story does add is the 2^53
boundary, and it is written to construct the condition (seeding the position
column through `exec`) and assert on the returned **variant**, never on a literal
position: the specification permits gaps, a conformant adapter may leave them, and
the ceiling is the store's declared fact rather than the suite's assumption
(`CLAUDE.md`, **The rule that matters**).

**Box 7.** Nothing `[FROZEN]` is changed. CF-40 and CF-39 are `[PROVISIONAL]` and
are discharged rather than amended; ES-17 is `[PROVISIONAL]`; ADR-0013 and
ADR-0014 are accepted and are honoured, not edited. If the real `SqlStorage` API
made the `ExceedsStoreLimit` channel unreachable, that would be a new decision
atom and a re-plan before any code — not a clause edit.

**Box 8.** No conformance rule here seems wrong. The one that looks odd on first
reading — `append_reports_exceeded_store_limits`, which can be switched off by a
fixture leaving three constants at their defaults — is deliberately gated that
way, because a store with no ceiling has nothing to promise; CF-18's reporting
obligation is what keeps the off state honest, and this story's answer to it is to
build the refusal channel rather than to change the rule.
