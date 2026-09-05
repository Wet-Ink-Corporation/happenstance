---
id: kb-open-question-query-union-rule-unowned-001
title: query_union_is_item_concatenation is owed by a clause and owned by no one
kind: open_question
status: accepted
authority_tier: note
summary: >-
  VT-31's residue is a conformance rule rather than a signature or a semantics question:
  query_union_is_item_concatenation, named at ES-15's Rejects line as the thing that would catch
  an item-sorting adapter, is owed and unwritten. ADR-0011 declines it explicitly, on the ground
  that no ADR in the phase-4 queue is the right instrument for a rule — "it belongs to whoever
  writes rules next" — and flags this as the one disposition in that document a human should
  confirm rather than inherit, because declining is cheap and losing the rule is not. What is not
  decided is who writes it and against which wrong implementation, given ADR-0010's standing
  requirement that a rule may not be added until a store exists that fails it. Forced by the next
  pass that writes conformance rules, and by phase 4's close, after which an unclaimed rule has no
  obvious reader. This is the open question in the wave that most wants a human's yes rather than a
  later wave's inference.
depends_on: []
related:
  - kb-decision-0011
  - kb-decision-0010
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-open-question-query-plan-parameter-chunking-001
source_paths:
  - .kb/_intake/0011-read-laziness-and-isolation.md
  - references/adr/0011-read-laziness-and-isolation.md
  - spec/SPECIFICATION.md
  - crates/happenstance-testkit/src/suite.rs
last_reviewed: 2026-08-10
---

# query_union_is_item_concatenation is owed by a clause and owned by no one

## What is true today

ADR-0011 (`.kb/decisions/0011-read-laziness-and-isolation.md`) is scoped to one question — what `read`
promises about laziness and isolation — but is instructed to claim or explicitly decline
twenty-nine unclaimed clause IDs nearest it, per the ADR-0011 brief quoted in the ADR itself:
"It must also claim, or explicitly decline, the twenty-nine unclaimed IDs nearest it." VT-31 is one
of them, and the ADR's own header states the disposition plainly: "**Explicitly declines:** VT-31.
Its residue is a *rule* that is owed (`query_union_is_item_concatenation`, named at ES-15's
`Rejects:` as the thing that would catch an item-sorting adapter) rather than a signature or a
semantics question, so no ADR in this queue is the right instrument. It belongs to whoever writes
rules next."

The ADR's closing table, "What this ADR leaves open, and who closes it," lists the same row with
marker `[FROZEN]`, no falsifier, and owner "Nobody yet — declined by this ADR." It adds the
sentence that gives this open question its urgency: "**This is the one disposition in this ADR
that a human should confirm rather than inherit**; ES-15's `Rejects:` names the adapter it would
catch (one that sorts and dedups query *items*) and nothing else in the suite catches it."

So what exists today: a named rule identifier, a named target implementation it must reject (an
adapter that sorts and dedups query items rather than treating a `Query`'s union as ordered
concatenation), and a clause (ES-15, `[FROZEN]`) whose `Rejects:` line already promises the rule
exists. What does not exist is the rule itself, anywhere in `happenstance-testkit`'s
`suite.rs`, and no ADR or phase has been assigned as its author.

## What is not decided

Who writes `query_union_is_item_concatenation`, and against which registered wrong implementation.
ADR-0010's standing requirement — a rule must be demonstrated to fail something before it is added,
CLAUDE.md's own corollary that "a rule that no adapter can fail is decorative" — means the rule
cannot simply be transcribed from ES-15's description. A concrete item-sorting-and-deduplicating
adapter has to exist in the testkit's own fixtures (or be written as part of landing the rule) for
the rule to be anything other than decorative. Neither the adapter nor the rule exists yet.

## What forces it

Two things, on different timelines. Immediately: the next pass that writes conformance rules for
the read side needs to know whether this rule is in scope for it or is deliberately being carried
forward again. Structurally: phase 4's close is the point after which an unclaimed rule has no
obvious reader — the ADR queue that was the mechanism for claiming clause IDs by topic closes with
phase 4, and nothing downstream re-scans for residue the way ADR-0011 was explicitly instructed to.
Without a fresh claim, `query_union_is_item_concatenation` sits exactly where VT-31 left it:
named, owed, and declined.

## Ordered sub-questions

1. Does a human confirm the decline — i.e., agree that no ADR in the phase-4 queue is the right
   instrument for a single conformance rule — or does confirming actually mean assigning an owner
   now rather than later?
2. If assigned, does the rule's registered wrong implementation get written into
   `happenstance-testkit`'s own fixtures first (an adapter that sorts and dedups query items), per
   CLAUDE.md's rule that a rule needs a named plausible wrong implementation before it is added?
3. Does the rule slot into `happenstance-testkit/src/suite.rs` beside ES-15's other registered
   rules, or does it need new fixture machinery the existing query-union tests do not have?
4. Is there a second residue rule anywhere else in the twenty-nine-ID extension ADR-0011 took on,
   with the same "declined, not solved" shape, that this pass's confirmation should also surface?
