# Does a benchmark that did not complete fail the build, and who declares what a scenario owes?

**Lane:** suite self-reports (`L2-03`). **Written by the lane that implemented the
change it describes, in the same session, and without the author → two-critic →
revision pass the original thirteen briefs had.** Nobody independent argued the
other side. Read it with that discount applied.

**Confidence:** medium on question 1, high on question 2.

**Semver:** none. `BenchmarkRecord::new` and `push` are private, so no caller can
construct a record; `is_complete` is additive and behind the off-by-default
`bench` feature. Not a `0.2.0` deadline.

## The question, as two questions

1. `BenchmarkRecord::report` now **panics** when a scenario did not run every pass
   it owes. `bench.rs`'s own module page says *"A benchmark result can never turn
   a merge red."* Does the completion assertion contradict that sentence, or is
   the sentence about thresholds only?
2. A record declares its owed passes as a `&'static [&'static str]` written at
   the scenario's own `BenchmarkRecord::new` call. Should that list be *derived*
   from something instead?

## Why it is owed

`L2-03` found that `report`'s documented assertion — *"it asserts that the
scenario completed and that the record is well-formed"* (`bench.rs:425`) — was
discharged by a predicate that cannot be false. Neither conjunct of
`is_well_formed` could fail: `BenchmarkPass::is_well_formed` is an invariant of
the private `count`, and `!passes.is_empty()` held because all three scenarios
push a pass unconditionally before any early return.

The early return it was supposed to catch is
`conditional_append_under_contention`'s, two lines below its own pass: a seed
that never lands leaves no boundary to race for, so the **contended pass — the
entire measurement — is absent rather than zero**, and the run reports as fine.
Driven, before the fix, with a fixture whose first append is armed to violate:

```text
BENCH conditional_append_under_contention: seed attempts=1 committed=0 \
  rejected=1 refused=0 failed=0 events=0
note: test did not panic as expected
```

The fix makes that a panic. That is a benchmark turning a build red, which is the
thing the module page says cannot happen.

## What is true today, from the tree

`crates/happenstance-testkit/src/bench.rs:22-27` (line numbers as of this brief;
the anchor is *"A benchmark result can never turn a merge red"*):

> **A benchmark result can never turn a merge red.** There is no threshold
> anywhere, on any budget: the shipped emitters assert that a scenario completed
> and that its record accounts for every attempt it made, and nothing else.

Read carefully, the sentence *already* names the two assertions as the exception
to itself — "there is no threshold" is the content, and "a scenario completed"
is listed among what the emitters do assert. The pre-existing well-formedness
assertion was likewise a panic. So the fix does not introduce a new class of
failure; it makes an assertion the page already claimed actually able to fire.

CF-34 (`spec/SPECIFICATION.md`, `[PROVISIONAL]`) says performance MUST be
measured by a separate harness and that harness MUST NOT be part of the
conformance bar. Its `Rule:` line reads *none*. **No clause governs what the
harness asserts about its own records** — which is why this is a brief and not a
clause reading.

## Option 1A — the completion assertion stays a panic (implemented)

An incomplete run is not a slow run. The failure names the scenario and the
missing pass, says in its own message that it is not a threshold and not a
timing, and fires only when a store fell over before the measurement began.

- **Costs an adapter author:** a red benchmark test on a day their store's
  unconditional seed append fails. That store is broken in a way the conformance
  suite would also catch, so the redness is not new information — but it arrives
  from the benchmark family, which the page told them could not go red.
- **Costs a caller:** nothing. `is_complete` is available for a caller's own
  emitter to assert or ignore.
- **Semver:** none.

## Option 1B — completion is reported, never asserted

`report` prints `BENCH … INCOMPLETE (missing: contend)` and returns. Nothing goes
red; the absence becomes legible in the line rather than in an exit code.

- **Costs an adapter author:** nothing, ever — which is the objection. The whole
  finding is that a line nobody reads is what let the gap exist; the same
  argument applies to a longer line nobody reads.
- **The strongest case for it:** CF-34's discipline is that this family decides
  nothing, and an adapter author who chose to mount the benchmark harness did not
  choose to add a failure mode. A store whose seed append fails under a benchmark
  it did not ask to gate on gets a red build from a harness explicitly excluded
  from the bar.

## Recommendation and the strongest argument against it

**1A, implemented.** The page's own sentence lists the completion assertion among
what the emitters do; the fix aligns code with the page rather than the reverse,
and CF-34's content is about thresholds rather than about assertions.

**The strongest argument against:** an adapter author reads *"a benchmark result
can never turn a merge red"* as a promise about their build, not as a taxonomy of
assertion kinds, and a distinction between *result* and *record* that has to be
explained is a distinction that will not survive contact with someone debugging
at speed. If that reading wins, 1B is right and the page's sentence should be
rewritten to say so unambiguously either way.

## Question 2 — who declares the owed passes

Implemented as a `&'static [&'static str]` at each scenario's `new` call, pinned
from **both** sides so it cannot be filled in by something with nothing to do
with the run: `push` panics on a label the scenario did not declare, so the list
cannot be narrowed to what an unhappy path reaches; `report` panics on an owed
label that never arrived, so it cannot be padded either. What is left is exactly
the set of passes the scenario produces when nothing went wrong.

That two-sided tie is the direct lesson of `Kind::StatedOnlyDefect`, withdrawn
the same day because its row held plain `fn` pointers with no type-level tie to
the subject and could be satisfied by two string literals.

The alternative — deriving the owed set from the labels a scenario produced on
some reference fixture — was rejected: it makes the reference store's behaviour
the definition of completeness, which is the same class of circularity as an
oracle that calls the implementation (RS-60-4).

**Not settled here:** whether a fourth scenario should be able to declare a pass
as *conditional* — owed only when a precondition held. Nothing needs it today,
and inventing the shape before a scenario needs it is how a `Capability` gets
declined by everything.

## Cost of delay

Low. The implemented behaviour is behind an off-by-default feature that no
adapter in the workspace mounts, and reversing 1A to 1B is a two-line change with
no public surface implication.

## What this does not settle

- CF-34's `[PROVISIONAL]` marker, which turns on whether a complexity property is
  expressible as a deterministic assertion. This brief neither supplies nor
  refutes that.
- Whether `bench.rs`'s module page should say "no benchmark *threshold* can turn
  a merge red" instead. That is 1A's follow-through and belongs with whoever
  ratifies 1A.
