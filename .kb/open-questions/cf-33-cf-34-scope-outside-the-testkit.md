---
id: kb-open-question-cf-33-cf-34-scope-001
title: Two new timed assertions landed outside the testkit and inside the gate; CF-33 and CF-34 do not say whether that is allowed
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Two lanes each landed a clock-adjacent assertion that CF-33 and CF-34 do not squarely govern.
  I-5's fix in happenstance-sqlite adds a #[cfg(test)] module that measures a 33.5x-40.1x ratio
  between a kept reference implementation and the shipped one, in an adapter's own crate rather
  than happenstance-testkit/src, which is where CF-33 (FROZEN) is deliberately scoped. L2-03's fix
  makes BenchmarkRecord::report panic when a scenario did not complete every pass it owes, which
  is a benchmark family assertion that can turn cargo xtask ci red, in apparent tension with
  bench.rs's own module claim that a benchmark result can never turn a merge red and with CF-34's
  (PROVISIONAL) rule that a performance harness MUST NOT be part of the conformance bar. Both
  landed and both pass today; what is open is whether either is a scoping violation, an
  acceptable instance of a narrower principle CF-33/CF-34's text does not yet state, or a sign
  that one or both clauses need a text change no lane implementing a routine fix was entitled to
  make. The proposed governing rule for the first — a ratio between two implementations measured
  in one process is not the duration CF-33's own Rejects paragraph is written against — is a
  candidate for standards/rust or a widened CF-33 scope paragraph, not yet written down anywhere
  binding.
depends_on: []
related:
  - kb-decision-0058
  - kb-open-question-testkit-contention-tolerance-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/timed-assertions-outside-the-testkit.md
  - .kb/_intake/remediation-2026-09-04-briefs/benchmark-completion-and-the-merge-red-promise.md
last_reviewed: 2026-09-07
---

# Two new timed assertions landed outside the testkit and inside the gate; CF-33 and CF-34 do not say whether that is allowed

## What is true today

**CF-33** (`spec/SPECIFICATION.md:8908-8933`, `[FROZEN]`) forbids a conformance rule from
reading a clock, measuring elapsed time, or asserting an operation count, enforced by a
`cargo xtask ci` grep over `happenstance-testkit/src` for `std::time`, `Instant`, `elapsed` and
`sleep`. The scope is deliberate — the clause "constrains conformance rules, which are the
library's," not `tests/` or an adapter's own crate.

**CF-34** (`spec/SPECIFICATION.md:8935-8949`, `[PROVISIONAL]`) requires performance to be measured
by a separate harness that MUST NOT be part of the conformance bar, and rejects "a benchmark
result gating a merge."

Two lanes each landed something adjacent to both clauses without violating either by the letter.
**I-5** (a complexity fix, `crates/happenstance-sqlite/src/query_sql.rs`, `#[cfg(test)] mod
tests`) added a check that runs inside `cargo test -p happenstance-sqlite`: it keeps a verbatim
copy of the pre-fix quadratic accumulation, runs both arms back to back in one process, asserts
they produce the same map, and asserts the shipped arm is at least 5x faster than the kept
reference — a threshold chosen against an observed 33.5x (debug) to 40.1x (release, per
`experiments/shipped-append-condition-sql/results/selectivity.md`) separation. It asserts no
duration and reads no wall clock value directly; it asserts a ratio between two implementations
measured together. It lives in an adapter's own crate, not `happenstance-testkit/src`, so CF-33's
grep does not reach it and its own scope paragraph does not either.

**L2-03** (`crates/happenstance-testkit/src/bench.rs`) made `BenchmarkRecord::report` panic when
a scenario's declared owed passes (a `&'static [&'static str]` fixed at each scenario's own `new`
call) were not all produced — found because
`conditional_append_under_contention`'s contended pass could be silently absent rather than zero
when a seed never landed, and the run still reported as fine. `bench.rs`'s own module page states
"A benchmark result can never turn a merge red," immediately followed by a sentence naming
completion as one of exactly two things the shipped emitters do assert. The completion assertion
is behind the off-by-default `bench` feature and is not mounted by any shipping adapter today.

## What is not decided

**For I-5:** whether "a ratio between two implementations, measured together, in one process,
asserting no duration" is categorically different from what CF-33's `Rejects:` paragraph
describes — "the plausible-looking rule that asserts a `backwards().limit(1)` read returns within
some bound," which fails on a loaded runner or a debug build because it is a duration compared
against a fixed number. A ratio's terms are supposed to divide out exactly the machine-dependence
CF-33's `Rejects:` paragraph objects to. Nobody has written that principle down as a rule either
adapter authors or reviewers can point to; the candidate home named is
`standards/rust/60-what-a-test-must-prove.md`, or a widened CF-33 scope paragraph — the second of
which is a specification edit and outside what implementing a fix authorizes. The counter-argument
on file: a ratio is still a clock, the 5x threshold is still a guessed number, and "this one is
different" is what every exception says first; the margin between the asserted 5x and the observed
33.5x is offered as the answer to flakiness risk rather than a decisive rebuttal.

**For L2-03:** whether `report`'s completion assertion contradicts `bench.rs`'s own "can never
turn a merge red" sentence, or whether that sentence is about *thresholds* specifically and the
completion/well-formedness assertions were always its stated exception. The text supports the
latter reading on close inspection, but a reader skimming the header sentence reasonably takes it
as a promise about their build rather than a taxonomy of assertion kinds — and the module page has
not been reworded to close that gap. The alternative implementation (report the incompleteness
in the printed line, assert nothing) was rejected on the ground that a line nobody reads is the
same class of gap the fix exists to close, but that alternative was not tested against reviewers
before this was decided.

## What forces it

Either clause's owner reviewing the two additions together, since they raise the same underlying
question from opposite ends (a non-testkit crate reading time; a benchmark-family assertion that
can redden `cargo xtask ci`) and no single lane implementing a routine fix was positioned to
settle a scope question for a `[FROZEN]` clause. Absent that, the next lane that wants a timed
assertion of either shape has this brief's precedent to read and no binding rule to cite.
