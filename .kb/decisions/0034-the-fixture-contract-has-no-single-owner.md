---
id: kb-decision-0034
title: The fixture contract has no single owning document, and CF-40 is ADR-0015's clause
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0034
reversibility: high
phase: 9
supersedes: null
superseded_by: null
summary: >-
  Answers kb-open-question-cf-40-ownership-001 on evidence neither claimant had. First: CF-40 is
  ADR-0015's clause, as its own header and decision-8 prose assert. The hedge in that same
  document's Consequences section — that decision 8 sets out both homes and declines to choose —
  is superseded by use rather than by argument, because two later decisions cite CF-40 to
  ADR-0015 and neither claims it: ADR-0022 records CF-40's clause home as a non-verdict with a
  named owner, twice, and ADR-0023 amends the fixture contract without claiming ownership of it
  either. Neither ADR-0015 nor ADR-0012 nor ADR-0022 is edited to record this; all three are
  accepted, and this atom cites them from outside. Second, and it is the durable half: the
  fixture contract has no single owning document, and that is now a recorded position rather than
  an unanswered one. A CF- clause is minted by the decision that first needs the capability, and
  it carries the reason there, beside the adapter that needed it. The alternative — a single
  named owner for CF- clauses, the way EventStore and ProjectionStore each have one — loses to
  three observations rather than to argument: ADR-0015 minted CF-40 and declined to own it,
  ADR-0012 owns CF-39 and MID_BATCH_FAULT by adjacency, and ADR-0023 is the third amendment
  without a claim. Three ADRs, three phases, no collision. The cost is stated with the position
  rather than discovered later: nobody can answer which document owns CF-40 without reading three
  ADRs, and a reader locating a fixture-contract clause reads the specification's section 6 by
  subject rather than one decision record. The evidence that turns the argument into an
  observation is phase 9's CloudflareFixture, the first fixture in the workspace to declare all
  three of CF-40's numeric ceilings and claim CF-39's MID_BATCH_FAULT at once, where every prior
  fixture left all three constants at None and append_reports_exceeded_store_limits reported a
  skip everywhere and certified nothing. What is not settled is the position's next test: phase
  10's POLL_BUDGET-shaped capability, ADR-0013 section 8's concern, which
  kb-open-question-poll-count-rule-strength-001 owns and which ADR-0013 itself says should be
  decided by whoever owns the fixture contract — a premise this decision answers by denying it.
  If POLL_BUDGET collides with piecemeal minting, that collision is the evidence that would
  supersede this decision, which is why its reversibility is high and its trigger is named.
depends_on:
  - kb-decision-0015
related:
  - kb-decision-0012
  - kb-decision-0022
  - kb-decision-0023
  - kb-decision-0013
  - kb-open-question-cf-40-ownership-001
  - kb-open-question-poll-count-rule-strength-001
source_paths:
  - .kb/_intake/cf-40-fixture-contract-ownership-resolution.md
  - references/adr/0015-validated-identifiers-and-store-limits.md
  - references/adr/0012-append-shape-and-preconditions.md
  - references/adr/0022-append-condition-strategy.md
  - references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - crates/happenstance-cloudflare/tests/support/
  - crates/happenstance-testkit/src/fixtures.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-20
---

# The fixture contract has no single owning document, and CF-40 is ADR-0015's clause

## Decision

This answers `kb-open-question-cf-40-ownership-001` on evidence neither original
claimant had available when it wrote the question. ADR-0015 minted CF-40 — a `Fixture`
MUST be able to declare `MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT` and
`MAX_EVENTS_PER_BATCH`, each an `Option<usize>` defaulting to `None` — and then
contradicted itself about whether it owned the clause: its own header and decision-8
prose say the clause "lands here," while its Consequences section says the same
decision "declines to choose" between itself and ADR-0012, the other candidate owner by
adjacency to CF-39 and `MID_BATCH_FAULT` in the same fixture-contract neighbourhood.

**First finding: CF-40 is ADR-0015's clause.** The hedge in ADR-0015's own Consequences
section is superseded by use, not by argument. Two later, accepted decisions have since
cited CF-40 back to ADR-0015 and neither has claimed it for itself: `kb-decision-0022`
records CF-40's clause home as a non-verdict with a named owner — twice, once in its
summary and once in its body — pointing at `kb-open-question-cf-40-ownership-001` rather
than annexing the clause; and `kb-decision-0023` amends the fixture contract (its
`CloudflareFixture` declares CF-40's constants) without claiming ownership of the clause
either. Two independent decisions treating CF-40 as ADR-0015's is stronger evidence than
either original claimant's own prose, and it resolves the contradiction without editing
either accepted document: `kb-decision-0015` and `kb-decision-0012` are both left
byte-identical, and this atom cites them from outside.

**Second finding, and the durable one: the fixture contract has no single owning
document.** A `CF-` clause is minted by whichever decision first needs the capability,
and it carries its reason there, beside the adapter that needed it, rather than in a
central fixture-contract document. The alternative — a single named owner for `CF-`
clauses, on the model of `EventStore` and `ProjectionStore` each having exactly one
owning trait definition — loses to three observations rather than to argument: ADR-0015
minted CF-40 and declined to own it; ADR-0012 owns CF-39 and `MID_BATCH_FAULT` by
adjacency rather than by counter-assertion; and ADR-0023 is now the **third** decision
to amend the fixture contract without claiming ownership of any part of it. Three ADRs,
three phases, and no collision between any of them — the piecemeal pattern has been
tried three times and has worked each time.

**The evidence that turns this from argument into observation** is phase 9's
`CloudflareFixture` (`crates/happenstance-cloudflare/tests/support/`), the first fixture
in the workspace to declare all three of CF-40's numeric ceilings — `MAX_EVENT_DATA_LEN
= 1 MiB`, `MAX_TAGS_PER_EVENT = 1024`, `MAX_EVENTS_PER_BATCH = 1024` — and claim CF-39's
`MID_BATCH_FAULT` in the same fixture, backed by a real SQLite trigger in the store's
own write path. Before it, every fixture in the tree left all three constants at `None`,
and `append_reports_exceeded_store_limits` reported a skip everywhere: the rule existed
and certified nothing.

## The cost, stated rather than discovered later

Nobody can answer which document owns CF-40 without reading three ADRs. A reader
locating a fixture-contract clause reads the specification's §6 "by subject" — as
ADR-0015 itself put it — rather than consulting one authoritative decision record. That
cost is accepted deliberately: the alternative (a single owning document, amended by
every fixture-contract change going forward) was not tried and is not free either — it
would require choosing, retroactively, which of ADR-0012 or ADR-0015 absorbs the other's
clauses, an edit this decision's own authority rules forbid making to either accepted
atom.

## What is not settled

Phase 10's `POLL_BUDGET`-shaped capability — ADR-0013 §8's concern, owned by
`kb-open-question-poll-count-rule-strength-001` — is the next test of this position.
ADR-0013 itself suggests the question should be decided by whoever owns the fixture
contract as a whole; this decision answers that premise by denying that a single owner
exists. If `POLL_BUDGET` cannot be minted piecemeal — if it genuinely needs a
central fixture-contract document to be coherent — that collision is the evidence that
would supersede this decision. Its `reversibility` is set to `high` and its trigger is
named for exactly that reason: this is a position taken on three data points, not a
structural guarantee.

## Alternatives rejected

A single named owner for the fixture contract, rejected on the three-ADR observation
above rather than on principle — nothing rules it out permanently, and `POLL_BUDGET`'s
arrival is the named test that could reopen it. Editing ADR-0015's or ADR-0012's body to
resolve the contradiction directly, rejected because both are accepted and immutable;
this decision adjudicates between their two halves from outside on evidence neither had,
and both stand as originally written.
