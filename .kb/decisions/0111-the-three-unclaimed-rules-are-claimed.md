---
id: kb-decision-0111
title: The three unclaimed rules are claimed, by ES-44, CF-44, CF-45 and ES-25's Rule line
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0111
reversibility: medium
phase: 21
supersedes: null
superseded_by: null
summary: >-
  Accepted 2026-11-20 for #185, on the owner's yes to every recommendation (proposed 2026-11-18). One record over
  the three UNCLAIMED_PENDING_ADR entries in xtask/src/spec_trace.rs. The read-fault rule gets a
  store clause, ES-44: a failure after read returned its stream is an Err item, never an end of
  stream. Its fixture half gets CF-44, CF-39's read-side twin. Both are frozen on landing, because
  Postgres and Neon arm a real read fault over a real medium. k_disjoint_boundaries_never_conflict
  is claimed by ES-25. Since ADR-0077 and wi-2ab1f3 it asserts only ES-25's if and only if under
  contention, plus the progress anchor exactly_one_of_n_contenders_commits already carries under
  ES-25. ES-25's MUST does not change; its Rule line and one rationale paragraph do, which is a
  repair. ops_agree_with_the_model gets a suite clause, CF-45, which obliges the suite to carry a
  generative family. It obliges no store, so its truth does not depend on PROPTEST_CASES.
  UNCLAIMED_PENDING_ADR stays, at length zero, so a new unclaimed rule still has to be written in.
depends_on:
  - kb-decision-0066
  - kb-decision-0077
  - kb-decision-wi-2ab1f3
related:
  - kb-open-question-read-fault-rule-no-clause-001
  - kb-open-question-disjoint-boundaries-no-clause-001
  - kb-open-question-model-family-rule-no-clause-001
  - kb-playbook-repair-frozen-clause-001
  - kb-reference-model-family-case-cliff-001
source_paths:
  - xtask/src/spec_trace.rs
  - crates/happenstance-testkit/src/suite.rs
  - crates/happenstance-testkit/src/concurrency.rs
  - crates/happenstance-testkit/src/model.rs
  - crates/happenstance-postgres/tests/support/mod.rs
  - crates/happenstance-neon/tests/support/mod.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-11-20
---

# The three unclaimed rules are claimed, by ES-44, CF-44, CF-45 and ES-25's Rule line

**Accepted on 2026-11-20 for #185, as recommended; proposed on 2026-11-18.** What follows describes the tree at acceptance; the follow-ups build it. Nothing under "What acceptance
changes" has been applied: no clause is minted, no `[FROZEN]` text is edited, and
`UNCLAIMED_PENDING_ADR` still has three entries. The plan is
`.temper/plans/audit-debt-adrs.md`. This atom is the whole record; there is no long-form version.

## Context

`cargo xtask spec-trace` check 6 requires every conformance rule to be claimed or retired by a
clause. `UNCLAIMED_PENDING_ADR` (`xtask/src/spec_trace.rs:3052`) holds the three that are neither,
and every green run prints them:

1. `arming_a_read_fault_makes_the_stream_yield_an_error` (`crates/happenstance-testkit/src/suite.rs:3284`).
   It seeds four events and reads them once unarmed as an anchor. Then it arms a fault and requires
   an `Err` **item** (`suite.rs:3313-3330`). ES-2 (`spec/SPECIFICATION.md:2691`) governs `read`'s
   signature and says nothing about what goes in the `Err` arm.
2. `k_disjoint_boundaries_never_conflict` (`crates/happenstance-testkit/src/concurrency.rs:561`).
3. `ops_agree_with_the_model` (`crates/happenstance-testkit/src/model.rs:775`).

The open-question atoms argue each one. The evidence has moved since they were written. Three
facts settle the shape of the answer.

**The read-fault rule now has real-medium evidence.** `happenstance-postgres` and
`happenstance-neon` declare `READ_FAULT` supported and arm a real fault
(`crates/happenstance-postgres/tests/support/mod.rs:503`, `crates/happenstance-neon/tests/support/mod.rs:202`).
SQLite declines it as an incapacity and Cloudflare declines it by scope, each with a reason
(`crates/happenstance-sqlite/tests/support/mod.rs:199`, `crates/happenstance-cloudflare/tests/support/mod.rs:258`).
ADR-0066 froze CF-39 on the same kind of evidence on the write side (`SPECIFICATION.md:9487-9495`).

**The disjoint rule no longer asserts more than ES-25.** The atom's objection was that ES-25 does
not say *k disjoint commands all commit*. Since ADR-0077, and the owner's ruling in wi-2ab1f3
that independence is a promise about conflict and not about liveness, the rule no longer asserts
that either. It now asserts four things:

- No contender failed outside `Committed`, `ConditionViolated` or `Busy` (`concurrency.rs:614-622`).
- At least one contender committed somewhere (`:623-627`).
- A boundary with no winner told nobody `ConditionViolated` (`:657-671`). This is ES-25's *only
  if*: no matching event is held, so no rejection.
- A boundary has at most one winner, and holds exactly what it acknowledged (`:672-688`). This is
  ES-25's *if*, with its "returned only when nothing was written".

The progress anchor in the second bullet is also in `exactly_one_of_n_contenders_commits`, as "at
least one, which `BusyAfterWriteStore` fails". ES-25 already claims that rule
(`SPECIFICATION.md:4108-4117`). So claiming the disjoint rule under ES-25 asserts nothing about
ES-25 that ES-25's existing claim does not already assert.

**The model rule's problem is its quantifier, and a suite clause does not have one.** A clause
saying *a store agrees with the contract over arbitrary sequences* would be true or false depending
on `PROPTEST_CASES`. The case cliff is at 192 against a default of 256
(`kb-reference-model-family-case-cliff-001`). The set of clauses it composes also grows with the
generator: ES-16 joined when `to` landed, and ES-10 and ES-19 joined on the trailing path. A clause
that obliges the *suite* to carry such a family is true or false by inspection of the testkit. That
is the same kind of clause as CF-22, which claims `registry::no_orphan_rules`
(`SPECIFICATION.md:9722`).

## Options

**Read fault.**

- **A1, mint ES-44 for the store and CF-44 for the fixture (chosen).** Both frozen on landing.
- A2, widen ES-2. ES-2 is `[FROZEN]` and about signatures. Widening it costs the same record and
  mixes a signature clause with a behaviour clause.
- A3, mint ES-44 only, with the fixture's duty as prose. This leaves a fixture that arms nothing
  with no clause to fail. That is `NoopFaultFixture`'s shape on the read path.
- A4, leave it listed. This is free, and it leaves the suite's only rule that an adapter author
  could correctly contest.

**Disjoint boundaries.**

- **B1, claim it under ES-25 by a Rule-line repair (chosen).** One rationale paragraph names the
  independence proposition and the two wrong stores. ES-25's MUST and marker do not move.
- B2, mint a separate independence clause. It would restate ES-25's *only if* under contention, so
  the specification would hold two frozen sentences for one proposition.
- B3, leave it listed.

**Model family.**

- **C1, mint CF-45, a suite clause (chosen).** It obliges the testkit to carry the family and says
  what a run does and does not check. It does not enumerate the clauses composed.
- C2, mint a store clause, *agrees with the model over arbitrary sequences*. Its truth depends on
  the environment, and an enumerated scope already went stale once.
- C3, add a third disposition to check 6, *family-claimed*, with no clause. That is an `xtask`
  policy change that makes CF-24's discipline weaker for every future property rule.

**The mechanism.**

- **D1, keep `UNCLAIMED_PENDING_ADR` at length zero (chosen).** Its ratchet still refuses a new
  unclaimed rule.
- D2, delete it. Its own doc says this deletes the mechanism.

## Decision

1. **ES-44 is minted `[FROZEN]`**, frozen on landing on the Postgres and Neon evidence, as ES-43
   was by ADR-0077.
2. **CF-44 is minted `[FROZEN]`**, on the same evidence, as CF-39's twin.
3. **ES-25 claims `k_disjoint_boundaries_never_conflict`.** This is a repair under
   `kb-playbook-repair-frozen-clause-001`: the set of stores ES-25 admits does not change.
4. **CF-45 is minted `[FROZEN]`.** It states the suite as it stands, and it reads no environment.
5. **Sub-question 2 of the read-fault atom:** `arm_read_fault` keeps taking no index. Paging belongs
   to the adapter, and the rule's rustdoc says why (`suite.rs:3267-3273`).
6. **Sub-question 3 of the read-fault atom:** `LogError::ReadFailed` stays a test-support
   convenience. The contract gains nothing.
7. **Out of scope, and recorded so it is not settled in passing.** The trailing model assumes that
   visibility never moves backward (`model.rs:771-774`), and no clause states it. CF-45 does not
   claim that assumption. `.kb/_intake/monotone-visibility-is-unstated.md` is still the material
   for its own record. Until that record exists, a store whose frontier retreats fails a rule on an
   assumption the specification does not make.
8. **Also out of scope:** the progress anchor in the second bullet above is unstated. It is shared
   with `exactly_one_of_n_contenders_commits`, so claiming the disjoint rule adds no new exposure.

## What acceptance changes

**`spec/SPECIFICATION.md`.**

- **ES-44 is new**, placed after ES-43. Proposed text:

  > **ES-44 — A failure part way through a read is an `Err` item, not the end of the stream.**
  > Once `read` has returned its stream, a store that cannot produce an event the read owes MUST
  > yield an `Err` item carrying that failure. It MUST NOT end the stream as though the log had
  > ended there.

  Rule: `arming_a_read_fault_makes_the_stream_yield_an_error`. Cases: E2E-25, E2E-22. Rejects:
  `SwallowedReadFaultStore`, that is, `let Ok(page) = fetch().await else { return Poll::Ready(None) };`.

- **CF-44 is new**, placed after CF-43. Proposed text:

  > **CF-44.** A fixture declaring `READ_FAULT` supported MUST, when armed, cause the next read on
  > a handle it has already connected to fail inside the store's own read path, so that the stream
  > yields an `Err` item. The fixture MUST state the mechanism. A fixture whose store can absorb
  > every read fault it is able to arm MUST decline the capability with that as its stated reason.

  Rule: the same. Rejects: `NoopReadFaultFixture`, which is written into the testkit's own
  `tests/` in the same change. It declares the capability and arms nothing.

- **ES-25:** add `k_disjoint_boundaries_never_conflict` to its Rule line. Add one paragraph:
  *the DCB independence proposition, that appends whose conditions match nothing the others write
  never conflict, is this clause's* only if *seen under contention. It is a promise about
  conflict, not about liveness (ES-43).* Name `SpuriousConflictStore` and `GlobalVersionStore` in
  its Rejects.

- **CF-45 is new**, after CF-44. Proposed text:

  > **CF-45.** The suite MUST carry a generative family: a rule that draws sequences of appends,
  > conditional appends and reads, replays each against the store under test and against a model
  > of this specification kept in the testkit, and fails at the first answer on which they
  > disagree. The clauses it composes are the ones its generator reaches, and those are stated on
  > the rule, not here. A run checks only the sequences it draws.

  Rule: `ops_agree_with_the_model`. Rejects: a suite of worked examples only, against which a
  store can pass every named rule and fail a composition no example enumerates.

- **§6.4:** CF-22's illustration points at CF-45.

- **§7.1 and §7.2:** regenerated.

**`xtask/src/spec_trace.rs`.** The three entries in `UNCLAIMED_PENDING_ADR` are removed and the
type becomes `[(&str, &str); 0]`. Its doc comment's sentence "Deleting the last entry deletes the
mechanism" becomes "the array is empty and stays, so a new unclaimed rule must be written in". The
literal `\n` inside the model entry's string at `:3073` goes with the entry. The comments
at `:671` and `:769` that point into the array are updated in the same edit.

**Testkit.** `NoopReadFaultFixture` goes into the mutation registry, failing only
`arming_a_read_fault_makes_the_stream_yield_an_error`.

**`.kb/open-questions/`.** The three atoms are closed by a dated section citing this record.

## Costs

Three clauses are added to the census, and one Rule line and one paragraph change in a frozen
clause. The work is about 60 lines of specification, a fixture of about 50 lines, and a net
deletion in `spec_trace.rs`. No port, signature or adapter changes.

## Door

Mostly one-way. Each new clause is `[FROZEN]` from the moment it lands, so taking one back needs
a further record. The ES-25 repair is two-way, because it changes no admitted set.
