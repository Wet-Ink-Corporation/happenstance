---
item: HS-S0023
stage: discover
created: 2026-08-12T13:01:44.361Z
updated: 2026-08-12T13:01:44.361Z
template_sig: 86ce4036
rendered_sig: e85b716d
---

# Discover — The command loop — read, decide, append, retry on ConditionViolated

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: land read → decide → append → retry-on-`ConditionViolated`, taking its `after` anchor **from the read** and never from `append`'s return, treating `conflicting_position` as the hint it is, with a bounded caller-visible retry, a `#[must_use]` decision value, and the policy stated in the loop's own rustdoc beside the *verbatim resubmission* distinction | `_storymap.md:54` (M3 row) | Four independent correctness properties, three of which are invisible to any test this project can write against `MemoryEventStore` |
| AC-005 — the story traces to the typed/evolvable-payload criterion because the loop is where encoding meets the store | `project.md:178-180`; `_storymap.md:79` (`codec-and-feature-forwarding` owns AC-005) | This story contributes to AC-005; it does not own it. Its own bar is the loop's correctness |
| `depends_on: domain-event-and-decision-model` — supplies `Boundary` (whose `query()` the loop calls) and `DomainEvent` (whose events the decision returns) | `_storymap.md:54`, `:116-119` | The loop is generic over `B: Boundary`, so both must exist first |
| `depends_on: codec-and-feature-forwarding` — supplies `Codec`, `CodecError` and the `json` feature that gates the convenience `commit` | `_storymap.md:54`; `_design.md:290-300` | `commit` is `#[cfg(feature = "json")]`; `commit_with` is ungated. That split is the DT-2 answer for codec selection (`_design.md:642`) |
| **`append`'s return is not a sound `after`.** The port's own doc says it in terms: positions may have gaps and another writer may hold one *below* this value that this caller never saw, so *"a condition anchored here silently excludes exactly the events a condition exists to catch."* The sound anchor comes from `read_decision_model`, which returns the last position actually observed | `crates/happenstance-core/src/store.rs:125-139` | Read directly, not paraphrased. This is the defect the loop exists to make unwritable |
| **`ConditionViolated::conflicting_position` is a hint, not a promise.** *"A caller must be written for `None`… a store reached over one-shot HTTP has no interactive transaction… A retry loop that branches on this field being `Some` works against an in-process store and stops working against a remote one"* | `crates/happenstance-core/src/error.rs:128-148` | The contract states the mutant and its blast radius in the field's own doc comment |
| The retry decision is expressed through `AppendError::is_condition_violated()`, never a `happenstance`-local duplicate enum; `ConditionViolated` is the DCB concurrency signal and a routine outcome, not a fault | `crates/happenstance-core/src/error.rs:128-133`, `:251-255`; `_decomposition.md:118-123` (AC-U05) | One error vocabulary. Any error this project adds carries the adapter's error as a type parameter with `#[source]`, never a `String` |
| The binding signatures: `commit`/`commit_with` returning `Result<Committed, CommandError<S::Error, D>>`; `Retry` is a **required argument** with `NonZeroU32` inside; `Committed { position, attempts }` is a named `#[non_exhaustive]` struct, not a tuple | `_design.md:473-534`, `:644-645` | A hidden default of 3 fails AC-U13 — *"a loop whose only exit is success is a hang with better manners"*; a tuple freezes the arity on a surface the alpha exists to keep movable |
| `DecisionModel: Clone` exists so the loop can re-fold from the **pristine** model on retry, never from a stale one, and never discarding the caller's command input | `_design.md:638`; `_decomposition.md:179-185` (AC-U13) | Retry semantics are a supertrait choice, made upstream and consumed here |
| The retry policy is stated in the loop's **own rustdoc**, beside the distinction from the retry-safety property of a *verbatim* resubmission. *"A doc comment that says 'see the specification' for the policy fails this"* | `_decomposition.md:155-161` (AC-U10); `project.md:87-88` | The prose is an acceptance criterion, not documentation hygiene |
| The code this replaces exists today, per-handler, and its own doc comment names its destination: *"this is the second half of every DCB command handler, and it is identical every time — which is exactly why it belongs in the typed layer."* Its `ConditionViolated` arm currently `bail!`s instead of retrying | `examples/course-subscriptions/src/main.rs:200-224` | Deleting that function *into* the library is the integration test for this story (`_decomposition.md:460-467`) |
| Bind `EventStore`, not `SendEventStore`, in this crate's generic code; import one flavour name per module | `_design.md:709-716`; `CLAUDE.md`, binding constraint 4 | `commit`'s bound is `S: EventStore` in the design (`_design.md:520`) and must stay the weaker one |
| The decision value is `#[must_use]` with the consequence in the message — building a decision and discarding it must not compile quietly | `_decomposition.md:145-148` (AC-U09); `_design.md:589-590` | For P1, a silently dropped append is indistinguishable from a successful one until production |

## Questions

**Answered here.**

- *Where does the `after` anchor come from?* `read_decision_model`'s second return value.
  Never `append`'s (`crates/happenstance-core/src/store.rs:125-139`; `_design.md:646`).
- *Is the retry bound caller-supplied?* Yes. `Retry` is a required argument, not a default
  (`_design.md:644`). `Retry::once()` is the explicit no-retry spelling, so "I do not want a
  retry" is stated rather than achieved by omission.
- *One entry point or two?* Two: `commit` (JSON, gated on the `json` feature) and
  `commit_with` (any `C: Codec`, ungated). One entry point taking `&C` always would make
  every first program name a codec before it names a domain (`_design.md:642`).
- *How is a refusal distinguished from contention?* By type. `CommandError::Refused(D)`
  carries the caller's own domain error as a type parameter with `#[source]`;
  `CommandError::Exhausted { attempts, source }` is a bounded retry that ran out. Neither is
  a store failure, and none of the three is flattened to a string (`_design.md:489-506`).

**Deferred to `spec`.**

- *Whether `Retry` grows a backoff policy field.* `Retry` is `#[non_exhaustive]` with
  private fields precisely so it can (`_design.md:743`); whether the alpha ships one is
  spec-grain.
- *The `decide` closure's exact bound* — `FnMut(&B)` in the design (`_design.md:524`), which
  permits re-invocation per attempt. Whether it is `FnMut` or a trait is spec's to confirm
  against the retry semantics above.

**Not blocked.** Neither `trybuild` nor `MemoryProjectionStore` is an input to this story.
`FaultyStore<S>` is the instrument that proves the retry behaviour (AC-U13), and it is
`misbehaving-testkit-stores`' — an in-project sibling with no dependency on this story, so
the two can be sequenced either way; the DSL story depends on both.

## Decision

The problem this slice solves is that the second half of every DCB command handler is
identical every time and is currently written out by hand once per handler, wrongly: the
example's `commit` at `examples/course-subscriptions/src/main.rs:207-224` does not retry at
all — it turns the routine concurrency signal into a `bail!` — and any author who fixes that
by hand has three chances to get it wrong in ways nothing in this repository can catch,
because `MemoryEventStore` never exhibits the behaviour that distinguishes right from
wrong. This story lands the loop once, in the library, with the anchor taken from the read,
the retry driven by `is_condition_violated()` and never by `conflicting_position`, a bound
the caller states and can see, and a re-fold from a pristine model on every attempt. The
spec for this story covers `commit` and `commit_with`'s signatures and generic bounds
(`S: EventStore`, never `SendEventStore`), `Retry`, `Committed` and `CommandError` with
every variant's meaning, the `#[must_use]` on any value representing events-to-be-appended,
the loop's own rustdoc carrying the retry policy and the verbatim-resubmission distinction
in full rather than by reference, and the tests that discriminate — including one driven by
`FaultyStore<S>` reporting `conflicting_position: None`. No `[FROZEN]` clause is amended;
`append`, `read_decision_model`, `AppendCondition` and `AppendError` are consumed exactly as
the contract defines them, which is the entire point.

## The wrong implementation

**The mutant: a retry loop that branches on `Some(conflicting_position)`.**

```rust
match store.append(&events, Some(&condition)).await {
    Ok(last) => return Ok(Committed { position: last, attempts }),
    Err(AppendError::ConditionViolated(v)) => match v.conflicting_position {
        Some(p) => { /* re-read from p and try again */ }
        None    => return Err(CommandError::Append(/* … */)),  // "nothing to retry from"
    },
    Err(e) => return Err(CommandError::Append(e)),
}
```

It looks careful. It is the shape an author reaches for precisely *because* they noticed the
field. And it passes everything this project can run: `MemoryEventStore` holds an
interactive view of its own log and can always name the conflicting event, so it reports
`Some` on every violation, so the retry path is the only path any test ever takes.
`cargo xtask ci` is green; the worked example runs; the DSL's retry test passes; all four
`wasm32` steps pass, because the flaw is behavioural and not a compile error.

It is wrong because `conflicting_position` is `Option` for a reason the contract states in
the field's own doc comment: *"an adapter that detects the conflict without learning which
event caused it reports `None`, and that is not a deficient adapter"* — a store reached over
one-shot HTTP can only express a conditional `INSERT … SELECT … WHERE NOT EXISTS`, which
yields a boolean and no row (`crates/happenstance-core/src/error.rs:135-147`). Against
`happenstance-neon`, then, this loop **never retries**: the first contention returns an
error to the caller, the bounded retry the API advertises does nothing, and the failure is
reported as a store problem rather than as the routine concurrency signal it is. A retry
loop that only retries on in-process stores is worse than no retry loop, because the API
promised one.

**The instrument that rejects it, and it must exist for this story to be checkable:**
`FaultyStore<S>::violate_next(n)` is specified to fail the next `n` appends with
`ConditionViolated` reporting **`conflicting_position: None`** — *"which a remote store
legitimately does"* (`_design.md:613-615`). This story's spec must include a test that drives
`commit` against `FaultyStore::new(MemoryEventStore::new()).violate_next(1)` and asserts the
call **succeeds** with `attempts == 2`. Under the mutant, that test fails; under every other
test in the workspace, the mutant survives.

**A second mutant: threading `append`'s return into the next condition.**

```rust
let last = store.append(&events, Some(&condition)).await?;
let condition = AppendCondition::new(query.clone()).after(last);   // for the follow-up
```

It compiles, it is the only value in hand, and it is right in every single-writer test ever
written — which is every test in this repository. The port's doc forbids it in terms:
positions may have gaps, and another writer may hold one *below* this value that this caller
never saw, so *"a condition anchored here silently excludes exactly the events a condition
exists to catch"* (`crates/happenstance-core/src/store.rs:125-139`). The failure is not a
crash; it is an append that succeeds when it should have been rejected, under concurrency,
in production. `MemoryEventStore` cannot produce it and neither can any fixture this project
ships, which is why the discriminator has to be the *code shape* — the anchor comes from
`read_decision_model`'s second return value — asserted by review and pinned by the loop's
own rustdoc, not by a test.

**A third mutant: a `happenstance`-local `RetryableError` enum.** Wrapping
`AppendError::ConditionViolated` in a local `enum { Retryable, Fatal }` and matching on that
is tidier at the call site and passes every check — and it gives callers a second error
vocabulary to learn and to keep in sync with the contract's, which RS-30-1 and AC-U05 refuse
(`_decomposition.md:118-123`). `is_condition_violated()` already exists
(`crates/happenstance-core/src/error.rs:251-255`); a duplicate is not an abstraction, it is
a fork.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes, and this story is the one where the first is **not** vacuous.
**Literal positions:** no conformance rule is added, but this story's tests do observe
positions — `Committed.position` comes back from `append`. The rule discovery fixes: every
assertion compares against the position the store actually assigned, never against a
literal, because the specification permits gaps and `GappyMemoryStore` exists to make that
concrete. The retry test asserts on `attempts`, which is a count and not a position.
**Frozen clauses:** ES-10's visibility invariant is what the after-anchor rests on
(`_design.md:294`), and this story *depends* on it rather than altering it. Nothing under
`crates/happenstance-core/src/**` is touched; a defect found here is logged with its clause
ID and routed under AC-012.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
