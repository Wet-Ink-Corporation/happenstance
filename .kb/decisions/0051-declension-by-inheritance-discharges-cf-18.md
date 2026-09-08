---
id: kb-decision-0051
title: Declension by inheritance discharges CF-18, and the rule B3 asked for could never fail
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0051
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  CF-18's costing changed the assertion. The ratified B3 — fail if any capability is declined
  without an accompanying declaration — describes a state the type system already makes
  unreachable, because Capability::declined asserts a non-empty reason and SUPPORTED is the
  absence of one. What landed instead is declension by inheritance: five capabilities default to a
  declension carrying the testkit's own prose, with a cfg_attr pair closing the wasm32 blind spot.
  CF-18's MUST is unchanged and ES-35's provisional marker is undisturbed. MemoryFixture, the
  fixture every adapter author copies, failed the new rule itself.
depends_on:
  - kb-decision-0010
related:
  - kb-open-question-cf-18-residuals-after-declension-001
  - kb-open-question-postgres-read-fault-declension-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/cf-18-observable-skip-reporting.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
  - .kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md
last_reviewed: 2026-09-07
---

# Declension by inheritance discharges CF-18, and the rule B3 asked for could never fail

## Decision

CF-18 (`[FROZEN]`) requires a rule whose capability requirement is unmet to be
emitted as a test that *reports* the skip with the fixture's stated reason, on
the ground that a green run in a third-party adapter's own CI is where a
reviewer and a user of that adapter can both see the trade being made. The
brief `cf-18-observable-skip-reporting.md` measured, rather than argued, that
this obligation had never been met outside this repository: libtest discards a
passing test's stdout, so a declined capability is invisible in a default
`cargo test` run and visible only under `--show-output`, which exactly one CI in
the world passes. The brief declined to recommend and offered a fallback —
narrow CF-18 to what libtest permits — only if a costed mechanism, "candidate
B3", proved too expensive to build before `0.2.0`.

**The fallback was not taken.** B3 was costed, at `3df4b6e`, and what it found
changed the decision from a reporting fix into a mechanism fix.

## Why B3's literal wording was decorative

B3 asked for "one extra emitted test per suite that fails if any capability is
declined without an accompanying declaration." That describes a state the type
system had already made unreachable: `Capability::declined` asserts a
non-empty reason string, and `Capability::SUPPORTED` *is* the absence of one —
declining and carrying a reason are the same act, by construction. A rule
written to the brief's literal words would pass on every fixture that will ever
exist, which is the decorative-rule failure `CLAUDE.md` names, arriving from a
direction nobody had watched: not a rule too weak to catch a defect, but a rule
whose subject the type system had already made unreachable. The lane that wrote
B3 had measured the *reporting* gap correctly and then described the
*mechanism* by analogy to it rather than by construction.

## What landed instead: declension by inheritance

Five capabilities now default to a declension carrying the testkit's own prose,
so a fixture that states nothing prints this crate's account of the trade in an
adapter's CI log as though it were that adapter's own — discharging CF-18's
`Rejects:` clause, which asked that "requiring a non-empty reason string
alongside each `false` puts the trade in the log where a reviewer and a user of
the adapter can both see it," in the form CF-18 actually names rather than the
form B3's wording implied. A `cfg_attr` pair closes a `wasm32` blind spot the
costing predicted having to merely document: because CF-23 makes the conformance
emitter the caller's own, a plain `#[test]` is neither run nor listed by
`wasm-bindgen-test-runner`, and `cfg_attr`'s false predicate strips the
attribute before name resolution, so `::wasm_bindgen_test` is never resolved on
a native build and no native adapter gains a dependency from it.

**`MemoryFixture` — the fixture every adapter author copies — failed the new
rule against itself.** It had inherited `MID_BATCH_FAULT`'s declension three
lines below its own comment arguing that "the reference fixture is the one an
adapter author copies and a capability nobody mentions is a capability nobody
thinks about." The author who wrote that sentence, applied it to the
neighbouring constant, and missed this one, is the concrete demonstration of
the rule this decision states: a rule everyone agrees with and nothing checks
holds only until the second time somebody is busy.

## What is unchanged

CF-18's MUST is untouched and stays `[FROZEN]`; ES-35's `[PROVISIONAL]` marker,
which depends on the same mechanism, is undisturbed by either the diagnosis or
the repair. The brief's fallback — narrowing CF-18 to Option A — remains
available and was not exercised.

## What this surfaced and left owed

`happenstance-postgres`'s fixture had inherited `READ_FAULT`'s default reason —
"the injection has to come from the adapter and this one has none to offer" —
which is false about that store: `PgReadStream` opens a `REPEATABLE READ`
transaction with a server-side cursor, exactly the paged shape
`happenstance-sqlite` had pointed at when it declined the same capability on
the grounds that paged adapters are where the injection has something to act
on. It now declines by scope, naming the injection that would work
(`pg_terminate_backend` against the reader's own backend, or closing the cursor
beneath it, both reachable from the fixture's existing second pooled
connection) rather than building it — building it is phase 10's remainder,
alongside Neon, and was deliberately not rushed into the commit that discovered
the gap.

## What this does not decide

Whether an adapter's README claiming conformance should have to state its
declined capabilities — a documentation-obligation question left with whoever
owns publication. Whether `capability_skips_are_reported`'s own assertion
could be macro-emitted without dragging the mutation-coverage harness into an
adapter's build — the first question the next residual owes an answer to.
