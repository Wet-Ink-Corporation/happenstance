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
  binding. A third instance, 2026-09-09, and the first deliberately unreachable from the gate:
  ops/host/preflight.sh asserts on the measurement host's environment (sysfs, systemd,
  --version) before any sample exists, which is what separates it from a budget and keeps it
  inside CF-34 (spec/SPECIFICATION.md:9021); nothing in xtask's step table, .redkiln/config.yaml's
  verify: block or CI invokes it, and ops/ is on affected.rs's INERT list. ADR-0064
  (kb-decision-0064) names the residual: registering the host as a self-hosted runner would make
  the preflight reachable from a merge-blocking job, and that is a decision, not a configuration
  change.
depends_on: []
related:
  - kb-decision-0058
  - kb-open-question-testkit-contention-tolerance-001
  - kb-decision-0064
  - kb-reference-host-clocksource-tsc-hpet-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/timed-assertions-outside-the-testkit.md
  - .kb/_intake/remediation-2026-09-04-briefs/benchmark-completion-and-the-merge-red-promise.md
  - .kb/_intake/2026-09-09-the-measurement-host-and-its-clock.md
  - ops/host/preflight.sh
  - xtask/src/affected.rs
last_reviewed: 2026-09-11
---

# Two new timed assertions landed outside the testkit and inside the gate; CF-33 and CF-34 do not say whether that is allowed

## What is true today

**CF-33** (`spec/SPECIFICATION.md:8994-9019`, `[FROZEN]`) forbids a conformance rule from
reading a clock, measuring elapsed time, or asserting an operation count, enforced by a
`cargo xtask ci` grep over `happenstance-testkit/src` for `std::time`, `Instant`, `elapsed` and
`sleep`. The scope is deliberate — the clause "constrains conformance rules, which are the
library's," not `tests/` or an adapter's own crate.

**CF-34** (`spec/SPECIFICATION.md:9021-9034`, `[PROVISIONAL]`) requires performance to be measured
by a separate harness that MUST NOT be part of the conformance bar, and rejects "a benchmark
result gating a merge."

Two lanes each landed something adjacent to both clauses without violating either by the letter,
and a third instance followed on 2026-09-09 that sits at the opposite end of the same axis: it is
clock-adjacent, and it is built to be unreachable from the gate.

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

**The measurement host's preflight** (`ops/host/preflight.sh`, landed with
[ADR-0064](../decisions/0064-the-measurement-host-has-declared-conditions.md)) asserts on the
*environment* of the dedicated Linux benchmark host — governor, EPP, frequency cap, SMT state,
the tuning unit's status, the current clocksource, the toolchain's `--version` — and fails the
run when any of them is not what `ops/host/host.env` declares. Every value it reads comes out of
sysfs, systemd or a `--version` **before the first sample exists** (`ops/host/preflight.sh:5`),
so it can fail when nothing has been measured. That is the property that separates it from a
budget and keeps it inside CF-34's letter: CF-34 rejects a benchmark *result* gating a merge,
and a check that runs to completion with no result in hand is not gating on one.
`benchmarks/src/paired.rs:71` draws the same line from the other side — it computes drift and
refuses to threshold it, because a drift threshold would be exactly the result-gating CF-34
forbids. What distinguishes this instance from the first two is reachability. I-5 runs under
`cargo test`; L2-03 can redden `cargo xtask ci` on any adapter that mounts the `bench` feature.
The preflight is invoked by nothing that blocks a merge: it is absent from `xtask/src/main.rs`'s
step table, from `.redkiln/config.yaml`'s `verify:` block and from `.github/workflows/`, and
`ops/` is on `is_inert`'s INERT list (`xtask/src/affected.rs:405-462`), held there by
`the_host_provisioning_tree_selects_no_package` (`xtask/src/affected.rs:999`) so that a diff
confined to it selects no package for the story-grain gate.

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

**For the preflight:** nothing about its text is open — ADR-0064 carries the argument that an
environment assertion is not a result assertion, and this atom carries only the instance. What is
open is that its compliance rests on a *wiring* fact rather than a *textual* one, and wiring
facts move. CF-33's compliance is a grep the gate re-runs on every push; CF-34's compliance for
the preflight is the absence of an invocation, which nothing re-checks beyond the INERT test.
ADR-0064 names the one path by which the absence stops being true: registering the host as a
self-hosted CI runner would make the preflight reachable from a merge-blocking job, and its
paragraph on unreachability would be false the moment the runner was enrolled. The brief calls
that *a decision, not a configuration change*, and the sentence is doing real work — a
self-hosted runner is a checkbox in a repository's settings, and nothing in this workspace would
notice it being ticked. Whether CF-34 should say something about environment preconditions
(distinct from results) so that a future enrolment has a clause to be checked against, or whether
ADR-0064's residual-risk paragraph is enough, is the same question the first two instances raise
from the other direction: whether the clauses' text should widen to name the principle each
instance is currently arguing from on its own.

A smaller fact worth carrying, because it is exactly the kind of drift the citation scan exists
to catch and cannot here: `ops/host/preflight.sh:11` cites CF-34 at `spec/SPECIFICATION.md:8747`,
which was true when the script was written and is now `9021`. `ops/` is outside the scan's
directories, so the anchor will keep drifting silently. The clocksource finding the preflight
defers to `paired.rs` for is in
[the reference atom](../reference/host-clocksource-tsc-vs-hpet-2026-09.md).

## What forces it

Either clause's owner reviewing the three additions together, since they raise the same
underlying question from different ends — a non-testkit crate reading time; a benchmark-family
assertion that can redden `cargo xtask ci`; an environment assertion whose CF-34 compliance is
guaranteed by an absence of wiring rather than by any text — and no single lane implementing a
routine fix or provisioning a host was positioned to settle a scope question for a `[FROZEN]`
clause. Absent that, the next lane that wants a timed assertion of any of these shapes has this
atom's precedents to read and no binding rule to cite. The one event that would force the third
instance on its own is enrolling the measurement host as a self-hosted runner, which ADR-0064
already says must arrive as a decision record rather than a settings change.
