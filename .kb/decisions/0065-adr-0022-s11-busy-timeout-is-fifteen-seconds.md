---
id: kb-decision-0065
title: ADR-0022 §11 is superseded in part — the busy timeout is fifteen seconds
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0065
reversibility: medium
phase: null
supersedes: null
superseded_by: null
summary: >-
  Partly supersedes ADR-0022 at section 11 only, leaving sections 4, 6, 7, 9, 10, 12 and 15
  untouched and standing — the shape kb-decision-0007 used against ADR-0006 and kb-decision-0031
  against ADR-0007, which is why supersedes stays null and kb-decision-0022 stays accepted. Section
  11 set busy_timeout at 5,000 ms and rested the value on a premise — "busy = 0 in every row of the
  64-contender table, so the timeout did real work and never ran out" — and set its own reopen
  condition on that premise: reopen if any run ever reports busy > 0. The premise is false.
  kb-reference-busy-timeout-margin-001 recorded busy > 0 at the shipped CONTENDERS = 64, one launch
  in seven, three attempts in 6,720, the first nonzero busy count in this tree, with a margin of
  1.31x–1.38x on the plateau and every wait figure a lower bound. The decision:
  crates/happenstance-sqlite/src/connection.rs's BUSY_TIMEOUT_MS is 15_000. Measured rather than
  reasoned, and measured on this adapter rather than on the experiment's candidate, which closes
  that experiment's own first caveat — at --test-threads=1, the worst case the host can produce,
  5,000 ms went red in 7 launches of 8, 15,000 ms in 0 of 16 and 30,000 ms in 0 of 8
  (kb-reference-busy-timeout-adapter-cap-sweep-001). The argument that decided it over leaving the
  cap alone is that raising it is free on the healthy path: the handler returns the instant the lock
  is acquired, so the cap bounds the pathological tail rather than delaying every append, and
  passing runs took 4.91–5.65 s at five seconds against 4.99–5.40 s at fifteen. What it is paid for
  by is named: a genuinely stuck writer now takes 15 s rather than 5 s to report, a rare path that
  ends in a red rule rather than a hang, because the cap stays finite. Three alternatives are
  rejected in terms. Ratifying section 11 as still correct — defensible, since what fired was
  liveness and not semantics, committed being correct in every row — loses because the cost is paid
  by the wrong person: a roughly 1-in-7 red at the shipped contender count tells an adapter author
  their store is unsound when it is not. 30,000 ms is equally clean here, buys nothing measurable
  over 15,000, and triples the pathological wait, and the core sweep's fewer-cores-is-better result
  (about 450x from twenty cores to one) is what makes fifteen defensible off this host rather than
  lucky. An unbounded handler stays rejected for ADR-0022's original reason, restated rather than
  assumed: CF-33 is [FROZEN] and forbids the conformance suite a watchdog, so this constant is the
  only liveness bound in the system and an unbounded wait converts a livelock into a hung job naming
  no rule. CONTENDERS is untouched — section 12 says the contender count is not that record's to
  reopen, and it is not this one's either. The replacement falsifier names the instrument the old
  one lacked, which is why the old one was unfalsifiable for as long as it existed: reopen this cap
  if the adapter's own concurrency target goes red on a SQLITE_BUSY at 15_000 on any host in any
  configuration, reproducing with the cap sweep's loop; reopen it downward if the pathological 15 s
  wait is ever what makes a real failure undiagnosable. Three things are deliberately not settled:
  section 9's firing, which still owes the twenty-line reproduction its own wording asks for;
  sections 8 and 16, whose falsifier cannot fire as written; and the conflation of a contended store
  with a broken one, which raising the cap makes rarer and cannot remove, since CF-33 denies a rule
  the clock that would tell them apart — owned by kb-open-question-testkit-contention-tolerance-001,
  whose blocking instrument now exists.
depends_on:
  - kb-decision-0022
related:
  - kb-reference-busy-timeout-adapter-cap-sweep-001
  - kb-reference-busy-timeout-margin-001
  - kb-decision-0064
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-open-question-testkit-contention-tolerance-001
  - kb-open-question-cf-33-cf-34-scope-001
  - kb-open-question-adr-status-vocabulary-001
  - kb-governance-what-may-refute-a-finding-001
source_paths:
  - .kb/_intake/2026-09-21-adr-0022-s11-superseded-busy-timeout-is-fifteen-seconds.md
  - references/adr/0022-append-condition-strategy.md
  - experiments/busy-timeout-margin/results/adapter-cap-sweep.md
  - experiments/busy-timeout-margin/results/busy-timeout-margin.md
  - experiments/busy-timeout-margin/README.md
  - crates/happenstance-sqlite/src/connection.rs
  - crates/happenstance-sqlite/README.md
  - crates/happenstance-testkit/src/faulty.rs
  - crates/happenstance-testkit/tests/contended_store_instruments.rs
last_reviewed: 2026-09-21
---

# ADR-0022 §11 is superseded in part — the busy timeout is fifteen seconds

## Decision

`crates/happenstance-sqlite/src/connection.rs`'s `BUSY_TIMEOUT_MS` moves from `5_000` to `15_000`.
This partly supersedes `kb-decision-0022` (ADR-0022) at §11 only — the paragraph that fixed the
5,000 ms value and its own reopen falsifier. Sections 4 (the `max(position)` guard inside
`BEGIN IMMEDIATE`), 6 (the `event_tag` join table), 7, 9, 10, 12 and 15 all stand unchanged, and so
does §11's own `journal_mode = WAL` and `synchronous = NORMAL` — only the third pragma value moves.

**Why `supersedes` stays `null` and `kb-decision-0022` stays `accepted`.** `.kb/decisions/README.md`
ties `supersedes`/`superseded_by` to a full replacement: the old atom flips to `status: superseded`.
A flip here would be false, since most of ADR-0022's fifteen sections are still the record this
adapter is built on. The corpus already has the partial shape, twice: `kb-decision-0007` "partly
supersedes ADR-0006", and `kb-decision-0031` "partly supersedes ADR-0007's runner allocation" while
stating the consequence plainly — "which is why kb-decision-0007 stays accepted and superseded_by
stays null." This atom follows the same convention: it names the section it corrects in its title,
`depends_on: [kb-decision-0022]` carries the relationship, and it leaves the old atom's frontmatter
untouched rather than setting a full supersession that would misdescribe fourteen other sections.

## What fired, and why it was unfalsifiable until now

§11 rested `busy_timeout = 5,000` ms on a stated premise: "`busy = 0` in every row of the
64-contender table, so the timeout did real work and never ran out." Its falsifier was symmetric —
reopen if any run ever reports `busy > 0`. Nothing in the tree reported a busy count at all until
`kb-reference-busy-timeout-margin-001`'s instrument existed, so the falsifier had been unfalsifiable
for as long as it stood: a defect in the falsifier, not only in the value. That atom then recorded
`busy > 0` at the shipped `CONTENDERS = 64` — one launch in seven, three attempts in 6,720, the
first nonzero busy count anywhere in this tree — with a margin of 1.31x–1.38x on the plateau, and
every `wait_ms` figure in it a lower bound.

## Why 15,000 and not another value

The margin measurement alone answers a prior question (how close the waits run), not the one that
decides a cap (how often the suite fails). `kb-reference-busy-timeout-adapter-cap-sweep-001` closes
that gap by running the real adapter's own `tests/concurrency.rs`, unmodified, at
`--test-threads=1` — the worst case this host produces, since serialising the rules lets all 64
contenders run genuinely simultaneously:

| `BUSY_TIMEOUT_MS` | launches | red |
| ---: | ---: | ---: |
| 5,000 | 8 | 7 |
| **15,000** | 16 | **0** |
| 30,000 | 8 | 0 |

Raising the cap is free on the healthy path — the busy handler returns the instant the lock is
acquired, so the cap bounds only the pathological tail rather than delaying every append. Passing
runs took 4.91–5.65 s at 5,000 ms against 4.99–5.40 s at 15,000 ms, indistinguishable on a shared
host. What the raise costs, named rather than hidden: a genuinely stuck writer now takes 15 s rather
than 5 s to report before failing — still a red rule, not a hang, because the cap stays finite.

## Alternatives rejected

**Ratify §11 as still correct, treating the firing as an accepted cost.** Defensible on the narrow
reading — `committed` was correct in every row, so what fired was liveness rather than a semantic
failure. Rejected because the cost lands on the wrong person: the suite exists to tell an adapter
author whether their store is sound, and a roughly 1-in-7 red at the shipped contender count tells
them it is not when it is.

**30,000 ms.** Equally clean on this host and measurably no better than 15,000, while tripling the
pathological wait. Rejected on the strength of the core sweep in `busy-timeout-margin.md`: fewer
simultaneously runnable cores measured *safer* by roughly 450x from twenty cores to one, which is
what makes 15,000 defensible off this host rather than a figure that got lucky here.

**An unbounded busy handler.** Stays rejected for ADR-0022's original reason, restated rather than
assumed true: CF-33 is `[FROZEN]` and forbids the conformance suite a watchdog
(`kb-open-question-cf-33-cf-34-scope-001`), so this constant is the only liveness bound in the
system, and an unbounded wait would convert a livelock into a hung job naming no rule.

**Lowering `CONTENDERS`.** Out of scope. §12 states the contender count is not that record's to
reopen, and it is not this one's either — `concurrency-family-and-contender-count` owns that
decision, and `CONTENDERS` stays at the shipped 64 here.

## The replacement falsifier

Reopen this cap if the adapter's own concurrency target (`crates/happenstance-sqlite/tests/concurrency.rs`)
goes red on a `SQLITE_BUSY` at `BUSY_TIMEOUT_MS = 15_000`, on any host, in any configuration —
reproduce with `experiments/busy-timeout-margin/results/adapter-cap-sweep.md`'s loop. Reopen it
*downward* if the pathological 15 s wait is ever what makes a real failure undiagnosable.

## What this deliberately leaves open

§9's firing (the captured `tokio::Handle` making `SqliteEventStoreError::NoRuntime` unreachable)
still owes the twenty-line reproduction its own falsifier's wording demands, and none exists in this
tree — that is `kb-open-question-adr-0022-falsifiers-fired-001`'s, amended rather than closed by
this atom. §8/§16's append-condition SQL falsifier still cannot fire as written. And raising the cap
lowers, but cannot remove, the conflation of a merely-contended store with a broken one — no value
of this constant can, since CF-33 denies a rule the clock that would tell them apart. That question
belongs to `kb-open-question-testkit-contention-tolerance-001`, whose blocking instrument —
`happenstance_testkit::FaultyStore::contend_next`, landed 2026-09-21 — now exists, demonstrated by
`crates/happenstance-testkit/tests/contended_store_instruments.rs`.
