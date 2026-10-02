---
id: kb-open-question-postgres-read-fault-declension-001
title: PostgresFixture inherited a READ_FAULT declension that is false about its own store
kind: open_question
status: superseded
authority_tier: note
summary: >-
  ADR-0051's cf-18 rule (declension by inheritance, printing the testkit's own prose whenever a
  fixture says nothing) found that PostgresFixture inherits the default READ_FAULT declension --
  "the injection has to come from the adapter and this one has none to offer" -- and that sentence
  is false about this store. PgReadStream opens a REPEATABLE READ transaction, DECLAREs a
  server-side cursor and issues a FETCH per chunk: exactly the paged-adapter shape
  happenstance-sqlite named when it declined the same capability by scope rather than by
  incapacity. The real injection is reachable through the second pooled connection the fixture
  already opens for SECOND_HANDLE -- pg_terminate_backend against the reader's own backend between
  two FETCHes, or closing the cursor beneath it -- and it was deliberately not built in the change
  that found the gap, because a rule the adapter has never run plus a fault path it has never had
  is not something to ship in the same commit that added the check. Owed as phase 10's remainder,
  alongside Neon.
  Resolved 2026-09-29: built at phase 10b (2ed06b4) and shipped in 0.2.0. PostgresFixture declares
  READ_FAULT supported and arms it with a view whose WHERE raises part-way through the FETCH, so
  arming_a_read_fault_makes_the_stream_yield_an_error runs against a real server-side fault and
  passes. Neither injection this atom named survives contact, and the fixture's docs say why. Neon's
  fixture arms the capability too.
depends_on:
  - kb-decision-0034
related:
  - kb-open-question-read-fault-rule-no-clause-001
source_paths:
  - .kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md
  - .kb/_intake/remediation-2026-09-04-briefs/read-fault-clause-and-capability.md
  - crates/happenstance-postgres/src/lib.rs
  - crates/happenstance-postgres/tests/support/mod.rs
  - crates/happenstance-neon/tests/support/mod.rs
  - crates/happenstance-neon/tests/neon_conformance.rs
  - CHANGELOG.md
last_reviewed: 2026-09-29
---

# PostgresFixture inherited a READ_FAULT declension that is false about its own store

## What is true today

`read-fault-clause-and-capability.md` landed `arming_a_read_fault_makes_the_stream_yield_an_error` as a new conformance rule and `Fixture::READ_FAULT` / `Fixture::arm_read_fault` as a defaulted capability pair, under the fixture-declension-policy Option A already adopted for other capabilities — every existing fixture that says nothing inherits a declension the testkit itself wrote, so no fixture had to move to keep compiling.

`2026-09-07-ratifications-discharged-and-what-execution-changed.md` records `cf-18`'s discharge — declension by inheritance, five capabilities defaulting to a shared declension whose prose prints in an adapter's CI log as though it were that adapter's own account of its store — and the audit this mechanism performs found a real defect the moment it ran: `PostgresFixture` inherits the default `READ_FAULT` declension, whose text reads *"the injection has to come from the adapter and this one has none to offer."* That sentence is false about this store specifically. `PgReadStream` opens a `REPEATABLE READ` transaction, `DECLARE`s a server-side cursor, and issues a `FETCH` per chunk — the discharge record calls this "precisely the paged adapter `happenstance-sqlite` was pointing at when it declined the same capability" by scope, naming a real injection point rather than claiming incapacity. `PostgresFixture` has a real injection point too; it simply inherited the wrong sentence.

The real injection is reachable without new plumbing: the fixture already opens a second pooled connection to satisfy `SECOND_HANDLE`, and that connection can either call `pg_terminate_backend` against the reader's own backend between two `FETCH`es, or close the server-side cursor out from under it. Either would make the store's `read` surface a real, medium-level fault rather than the testkit exercising only its own in-process mutants.

Building it was deliberately deferred rather than folded into the change that found the gap: a rule this adapter had never run, paired with a fault path it had never had, is not something to ship in the same commit that adds the check that revealed the need for it. It is named as "phase 10's remainder, alongside Neon" — `happenstance-neon` has no fixture at all yet, and inherits the same false declension the moment one is written, since it shares the paged, server-side-cursor shape this gap is about.

## What is not decided

Which of the two named injections — `pg_terminate_backend` against the reader's own backend, or closing the cursor beneath it — is the right mechanism, and whether `PostgresFixture`'s `Capability` declaration needs new machinery beyond what `SECOND_HANDLE` already opens, or can reuse that connection as-is.

## What forces it

Phase 10's remainder. Until this is built, `happenstance-postgres`'s fixture prints a declension sentence about itself that this same audit pass proved false, which is exactly the shape `cf-18`'s mechanism exists to catch — and it will keep printing that sentence, uncorrected, for as long as the deferral stands.

## Ordered sub-questions

1. Does `happenstance-postgres` decline by scope now (naming the injection, as `happenstance-sqlite` did for its own capability) as an interim step before phase 10's remainder builds the real fault, so the inherited default sentence stops being printed in the meantime?
2. Between `pg_terminate_backend` and closing the cursor: which reproduces a real client-visible read fault more faithfully, and does the answer change if `happenstance-neon`'s HTTP-based cursor model needs a different mechanism entirely once its fixture exists?
3. Once built, does `arming_a_read_fault_makes_the_stream_yield_an_error` still pass against `happenstance-postgres`, or does a real severed-cursor fault surface differently than the in-process `SwallowedReadFaultStore` mutant the rule was originally proved against?

## Closed — 2026-09-29

This was answered in code at phase 10b, and nobody came back to update this atom. Commit `2ed06b4`
(*"arm READ_FAULT, which this fixture is the one that owed"*) landed on 2026-09-07 and shipped in
`0.2.0`; the entry is `CHANGELOG.md:435-449`, under `[0.2.0]`.
`crates/happenstance-postgres/tests/support/mod.rs:488` declares
`READ_FAULT: Capability = Capability::SUPPORTED`. `arm_read_fault` and its documentation
(`:540-592`) run `READ_FAULT_INJECTION` (`:621-634`), which renames `event` aside and puts a view
in its place. The view's `WHERE` calls a `plpgsql` guard that raises above position 2. The reader's
`DECLARE … CURSOR` plans and opens normally, and the `FETCH` answers with the raise instead of a
page. So the fault arrives while rows are being produced, which is the failure this capability
exists to test. The rename is reversible, and no rows are destroyed.

The sub-questions, in order:

1. **Moot.** No interim declension by scope was needed, because the real arming landed in the same
   phase the gap was found in.
2. **Neither injection.** The fixture's own docs (`:556-574`) record why both fail. Arming happens
   *before* the read starts, because a rule connects before it arms, so there is no reader backend
   yet to terminate and no cursor to close. The pool absorbs a terminated idle backend, which is
   CF-39's named hazard arriving one step early, and a fixture whose arming is silently absorbed
   would pass the rule vacuously. And a cursor is session-local, so no second connection can close
   it. Neon did not need a different kind of mechanism. Its fixture claims `READ_FAULT` as well
   (`crates/happenstance-neon/tests/support/mod.rs:198`, armed at `:247`), and
   `crates/happenstance-neon/tests/neon_conformance.rs:392-406` asserts that the declaration is an
   answer rather than the trait's inherited default.
3. **Yes, it passes.** The stream yields an `Err` **item** rather than ending, so this adapter does
   not report a failed fetch as the end of the log.

The "What is not decided" paragraph above is answered along with them. No new `Capability`
machinery was needed, and the arming goes through the pool the fixture already holds. The row this
question held in `runbook/ledgers.md`'s *Open decisions* table is answered by this section. Closed
by hand in phase 16. No accepted decision was edited.
