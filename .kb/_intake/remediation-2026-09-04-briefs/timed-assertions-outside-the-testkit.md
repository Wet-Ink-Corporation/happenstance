# May an adapter's own test target assert on a clock, when CF-33 forbids it to a conformance rule and CF-34 puts performance in a separate harness?

Record: **I-5-timed-assertions-outside-the-testkit**. Raised by the lane implementing
`references/evaluation/review-pre-publication-2026-09-03.md:2685-2723` (`I-5`), which is
itself routed as a backlog row and needs no decision record. This is the *side effect* of
implementing it, and it is the part that sets precedent.

**This brief did not get the author → two-critic → revision pass the original thirteen
had.** Read it with that discount applied.

---

## Why this is owed

I-5 is a complexity fix, and CLAUDE.md's bar for one is explicit: *"an optimisation with
no measurement is a preference."* The measurement existed —
`experiments/shipped-append-condition-sql/results/selectivity.md` §1, 257,690 µs against
6,424 µs, **40.1x** — but an experiment is out of the gate by construction (CF-34) and
nothing in the gate would notice the quadratic coming back. The lane therefore landed a
**standing** check, and that check reads a clock inside
`cargo test -p happenstance-sqlite`.

Two frozen clauses sit near it and neither answers it.

- **CF-33** `[FROZEN]` (`spec/SPECIFICATION.md:8831`): *"No conformance rule may read a
  clock, measure elapsed time, or assert on an operation count."* Its own text scopes the
  lint to `happenstance-testkit/src` **deliberately** — *"`tests/` is where the
  concurrency racers live … CF-33 constrains conformance rules, which are the library's"*
  — so it does not reach an adapter's crate at all. But its `Rejects:` paragraph is the
  argument, not the scope: a timed assertion *"passes on the author's machine, fails on a
  loaded CI runner, fails under a debug build, and makes the suite's verdict a property
  of the hardware."*
- **CF-34** `[FROZEN]`: *"Performance MUST be measured by a separate harness, and that
  harness MUST NOT be part of the conformance bar. An adapter that is slow is
  conformant."* The new check is not part of the conformance bar — it is a unit test in
  the adapter — but it is part of `cargo xtask ci`, which is the bar that actually
  gates work.

## What landed, and what it does about the objection

`crates/happenstance-sqlite/src/query_sql.rs`'s `#[cfg(test)] mod tests` — the crate's
only such module, and it is there because `Selectivity::read_for` is `pub(crate)` by
ADR-0022 §10's decision and `planned_statement_count` passes `Selectivity::default()`, so
no target in `tests/` can reach the function at all.

**It asserts no duration.** It measures a **ratio between two implementations, back to
back, in one process, on one input, in whatever profile the caller built** — the shipped
accumulation against a verbatim copy of the one it replaced, kept in the test module for
that purpose. Every term CF-33's `Rejects:` names — the author's machine, the loaded
runner, the debug build — divides out of a ratio. The observed separation is **33.5x** in
a debug build and 40.1x in the experiment's release build; the threshold asserted is
**5**.

It also asserts, **before either arm is timed**, that the two implementations produce the
same map. A faster function that answers a different question fails there rather than
passing as an optimisation.

## Options

### Option A — allow it, scoped as landed: a ratio against a kept reference, in an adapter's own crate

**Cost.** The reference implementation must be kept and must not drift; it is dead code
that exists to be slow. About 1.7 s of every `cargo test -p happenstance-sqlite` in a
debug build goes to running it. And it is precedent: the next lane will read it as
permission for an absolute budget, which is the thing CF-33 rejects.

**Buys.** The quadratic cannot come back green. Nothing else in the gate would notice it.

### Option B — forbid it; route the standing check to `experiments/` and accept that the gate cannot see a regression

**Cost.** `experiments/` is out of the gate by construction, so this is the state I-5 was
found in: a measured defect with nothing standing between it and its return. The audit
found it by reading, not by a failing test.

**Buys.** One rule, no scoping argument, no precedent to police. CF-34 read plainly.

### Option C — allow it, and write the scope down

Option A plus a sentence somewhere that governs: a timed assertion outside
`happenstance-testkit/src` is admissible **only** as a ratio between two implementations
measured in one process, never as a duration, and never in a conformance rule. That is a
constitution atom (`standards/rust/60-what-a-test-must-prove.md` is the obvious home) or
a widening of CF-33's scope paragraph — and the second is a specification edit, which is
not this lane's.

## Recommendation

**Option C, with A as its first instance.**

The distinction that matters is not *where the test lives* but *what it asserts*, and
CF-33's own `Rejects:` paragraph is written about durations rather than about clocks. A
ratio between two implementations is a property of the two algorithms. Writing that down
is what stops the next lane from reading this precedent as permission for the budget
CF-33 actually rejects.

**The strongest argument against, stated in its own words.** *A ratio is still a clock,
and the 5x threshold is still a number somebody guessed. On a machine running twenty
agents, one arm can be descheduled and the other not; the fix for that is more samples,
which is more time, which is how a flaky test earns its place in the gate. CF-34 already
says where performance goes, and "this one is different" is what every exception says
first.* That is the objection the threshold answers rather than dismisses: the margin
between the asserted 5x and the observed 33.5x is nearly seven-fold, and the arms are
seconds against milliseconds, so flipping the verdict needs a perturbation far outside
what scheduling noise produces. If that turns out to be wrong in practice, the failure is
visible and the repair is Option B, not a wider threshold.

## Cost of delay

Low. The check is landed and green; what is undecided is whether it is a precedent or an
exception. The cost is paid by whichever lane next wants a timed assertion and has
nothing to read.

## What this does not settle

- Whether `#[cfg(test)]` modules in `src/` are acceptable in this workspace at all. This
  is the first one in `happenstance-sqlite`, and the reason is reachability rather than
  timing — `Selectivity::read_for` is crate-private on purpose. Whether ADR-0022 §10's
  privacy decision should have come with a `pub(crate)` test seam is a separate question.
- Whether the kept reference implementation needs a drift check of its own.
  `experiments/one-connection-latency` solved the same problem with four tests that
  re-derive its copy from the live tree before any clock runs; nothing equivalent guards
  this one, and it does not need to while the two are twenty lines apart in one file.
