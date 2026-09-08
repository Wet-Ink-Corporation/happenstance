---
id: kb-open-question-postgres-read-fault-declension-001
title: PostgresFixture inherited a READ_FAULT declension that is false about its own store
kind: open_question
status: accepted
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
depends_on:
  - kb-decision-0034
related:
  - kb-open-question-read-fault-rule-no-clause-001
source_paths:
  - .kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md
  - .kb/_intake/remediation-2026-09-04-briefs/read-fault-clause-and-capability.md
  - crates/happenstance-postgres/src/lib.rs
last_reviewed: 2026-09-07
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
