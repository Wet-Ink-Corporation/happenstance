# `EventStore::read`'s `Err` arm now has a rule, a capability and a mutant. Does it get a clause, and if so which one?

Short answer up front: **it should, and the shape of the sentence is not in
doubt — but minting it is the specification owner's act and this lane did not
take it.** The rule is landed and disposed of through `UNCLAIMED_PENDING_ADR`,
which prints it on every green run and can only shrink.

**This brief did not get the two-critic pass the original thirteen had.** It was
written by the lane implementing `L3-01`, in the same session as the change it
describes. Read it with that discount applied.

---

## Why this is owed

`references/evaluation/review-pre-publication-2026-09-03.md:1621-1660` (L3-01)
states the absence in terms: *"no clause requires a rule to induce a read fault,
and the absence is what this entry is about."* ES-2 governs `read`'s signature —
that the stream is returned at the top level and the method is not `async` — and
says nothing about what the `Err` arm obliges. So there is no clause to attribute
the new rule to, and attributing it to one would mean asserting that clause
contains a sentence it does not.

That is not a technicality. `spec-trace`'s check 6 exists because CF-24's
discipline runs both ways: a rule with no clause is as much a problem as a clause
with no rule. The rule was landed anyway, because the hazard it catches is
concrete and measured; what was not landed is the sentence that makes it
normative.

## What is true today, after the change

- `Fixture::READ_FAULT` and `Fixture::arm_read_fault`, both **defaulted** — which
  is Option A of `fixture-declension-policy.md` applied to the first capability
  to land under it, so no existing fixture had to move.
- One rule, `arming_a_read_fault_makes_the_stream_yield_an_error`: seed four
  events, read them unarmed as an anchor, arm, read again, and require the stream
  to yield an `Err` **item** rather than to end.
- `SwallowedReadFaultStore` registered as a mutant, failing exactly that rule.
  `PagedStreamStore` beside it is the same paging store meeting the same armed
  failure and yielding the `Err` — so the rule is shown to be passable as well as
  failable.
- The rule listed in `xtask/src/spec_trace.rs`'s `UNCLAIMED_PENDING_ADR`, naming
  this document.

## Options for the clause

### Option A — mint an ES-clause of its own

*A store MUST surface a failure encountered part way through a `read` as an
`Err` item on the stream, and MUST NOT report it as the end of the stream.*

- **Costs an adapter author:** nothing they were not already obliged to do; the
  `Err` arm is in the signature and always was.
- **Buys:** the rule becomes normative, the wrong implementation is named in the
  specification the way CF-39's is, and check 6 loses an entry rather than
  gaining one permanently.
- **Marker:** it would land `[PROVISIONAL]` at most — no adapter has armed a read
  fault, and the two that will (`happenstance-cloudflare`, `happenstance-neon`)
  have not.

### Option B — widen ES-2

ES-2 is `[FROZEN]` and about the *signature*. Widening it would put a semantic
obligation inside a clause about shape, and a `[FROZEN]` clause cannot be edited
without a new decision atom under the repair-frozen-clause discipline. This is
strictly more expensive than Option A and buys the same sentence.

### Option C — leave it unclaimed

The rule stays in `UNCLAIMED_PENDING_ADR` indefinitely. Honest, visible on every
run, and free — but it makes the rule advisory in a suite whose whole argument is
that rules are not advisory, and it is the third permanent entry on a list whose
documentation says it *"can only shrink"*.

## Recommendation

**Option A, by the specification owner, before phase 12.** The sentence is not
contested, the rule and its two implementations are already in the tree to cite,
and the alternative leaves the suite's newest rule as the only one an adapter
author could argue with.

### The strongest argument against, in its own words

*A clause minted to justify a rule somebody already wrote is the tail wagging the
dog, and this repository's own precedent is the opposite order.* True, and it is
why this is a brief rather than a clause. The mitigating fact is that the rule was
not invented — it was measured: the store it rejects fails **0 of 89** rules with
no way to arm it and **22** with the fault armed by hand, which is a hole in the
suite rather than a preference about it. What the lane could honestly do was close
the hole and record that the sentence is owed; what it could not do was write the
sentence.

## What this does not settle

- **Whether the capability should have taken an index.** `arm_read_fault` takes
  no argument, unlike `arm_mid_batch_fault(after)`. The reasoning is that a
  `read` is one call whose granularity — page, chunk, item — belongs to the
  adapter, and demanding a fault "after the *k*-th event" would be the testkit
  asserting a paging model the port does not have; `arm_commit_fault` is the
  existing precedent for the argument-free shape. A reviewer who wants the rule
  to distinguish "failed at the first poll" from "failed mid-stream" needs the
  index, and would have to say what a non-paging adapter does with it.
- **Whether `LogError::ReadFailed` belongs in the shared correct core.** It was
  added beside `WriteFailed`, and is the second variant a *conformant* store in
  the mutation binary produces. That is a test-support decision, not a contract
  one, but it is the kind that becomes load-bearing quietly.
- **Whether the two paged adapters should arm one before phase 12.**
  `happenstance-cloudflare`'s fixture declines *by scope, not by incapacity* and
  says so; `happenstance-neon` has no fixture yet. Until one of them arms a real
  fault over a real medium, the only stores exercising this rule are in-process
  and the capability's far end is as empty as `MID_BATCH_FAULT`'s was before
  phase 9.
