# ADR-0010: The conformance suite's own proof obligation

- **Status:** accepted
- **Date:** 2026-08-07
- **Settles:** CF-1 – CF-29, and in particular CF-1's demand that a rule be
  demonstrated to fail something
- **Extends:** the registry that landed at phase 1, which answered the third of
  the three questions below before this ADR was written

## Context

`happenstance-testkit` is the crate that decides whether an adapter exists. It
carries twenty-seven rules, and **not one of them has ever been shown to reject
anything.** They were written by reading the contract and asserting what it says.
That is how every conformance suite starts and it is not sufficient, because the
failure mode is silent: a rule that asserts something no plausible implementation
gets wrong is a rule that passes forever and certifies nothing, and it is
indistinguishable from a good rule until an adapter with the corresponding bug
passes it.

CLAUDE.md already states the corollary — *a rule that no adapter can fail is
decorative* — and this project has now met the shape three times in its own
tooling rather than in a rule: a `Send` assertion satisfied by auto-trait leakage,
a `docsrs` gate step that printed `skipped` on every runner while two documents
vouched for it, and a documentation step that reported warnings and exited 0. In
every case something was asserted and nothing was checking.

Three questions have to be answered together, because each one's answer
constrains the others.

**What must a rule be demonstrated to fail?** Nothing in the tree currently
answers this, and the reviewer's measurement is the reason it is urgent: four
plausible wrong implementations — `LIMIT` applied before the tag filter, an OR-ed
query returning a matching event twice, `COUNT(*)+1` position allocation, and
probe-then-insert outside the transaction — **pass the suite as it stands.**

**What shape must the fixture take?** `event_store_conformance!` re-evaluates its
factory expression once per test and contractually requires a fresh, empty store.
So no rule can hold two handles onto one backing store, and that single decision
forecloses durability (CF-14), reopen, and every genuinely multi-connection rule
at once. It is why three clauses are `[DEFERRED]` on "nothing in the workspace can
express the question".

**How are rules emitted for runtimes that are not tokio?** Phase 1 answered this
one, and it is included here because PS-35's lesson applies: a decision that
exists only as code someone copied is a decision nobody has taken. The registry
enumerates the rule set exactly once and takes the *runtime wrapper as a
parameter*.

## Decision

### 1. Every rule owes a mutant, and every mutant owes a manifest

**A rule may not be added to the suite until a store exists that fails it.** That
store lives in `happenstance-testkit`'s own `tests/`, and it declares which rules
it fails.

Three meta-tests hold the obligation, and they are the phase's proof artefact:

- `every_rule_has_a_mutant` — for each rule in the registry, at least one mutant
  declares it. This is CF-1 made mechanical: a decorative rule cannot be added,
  because adding it fails this test until its wrong implementation is named.
- `mutants_fail_exactly_their_declared_rules` — a mutant fails the rules it
  declares and **passes every other rule**. The second half is the load-bearing
  one. A mutant that fails everything proves nothing about the rule it was
  written for; it proves the store is broken. Exactness is what makes the
  registry a map from rules to the bugs they catch, rather than a pile of
  failures.
- `conformant_variants_pass_everything` — the positive control. Without it, all
  of the above is satisfied by a harness that reports failure unconditionally,
  which is the same vacuity in a new place.

**Each mutant states its provenance: the real implementation mistake it models.**
A mutant invented to fail a rule is circular — it demonstrates that the rule
rejects the thing the rule was written to reject. A mutant drawn from a mistake
someone would actually make demonstrates that the rule catches something. Where
the provenance is "a reviewer measured this passing the current suite", say so.

**No pass rate is ever quoted over the mutant set.** It is an author-chosen bug
set and the number is a selection artefact: it says how representative the author
was, and reports it as though it said how good the suite is.

### 2. The fixture may hand out repeated handles, and skips are reported

The fixture contract changes from *a fresh empty store* to *a factory that can be
asked more than once for a handle onto the same backing store*. An adapter that
cannot honour that — a genuinely in-memory store with no shared backing — declares
so, and the rules that need it are skipped.

**A skip must be reported, never silent** (CF-18). A capability an adapter does
not exercise is a capability nobody knows is untested, and a suite that quietly
runs twenty-one of twenty-seven rules while printing green is worse than one that
runs twenty-one and says so — the first actively misleads, the second merely
under-tests.

**This lands before phase 8 writes its conformance file, not after.** The fixture
shape is the suite's public API; changing it after the first real adapter binds to
it means changing that adapter too, and every one written between.

### 3. The runtime wrapper stays a parameter, and the rule set stays enumerated once

Ratified rather than decided, because phase 1 built it and nothing has argued with
it since. `for_each_event_store_rule!` is the sole enumeration and takes the *path
of a macro*; the three shipped emitters are conveniences, not a closed set. A
runtime nobody here has heard of needs no release of this crate.

Two corrections to the record, because both were believed and neither is true:

- **`#[tokio::test]` already drives a `!Send` store.** `tokio::spawn` requires
  `Send`; `Runtime::block_on`, which the attribute expands to, does not. The
  parameterisation is right and its justification is **`wasm32` portability**, not
  `Send`-ness. CF-23's reasoning should say so.
- **`RefCell` is `Send`.** It surrenders `Sync`. A `!Send` reference store built
  from `RefCell<Vec<_>>` alone would prove nothing; `Rc` is what does the work,
  and CF-28's wording must name it.

### 4. Two rules are retired, and the specification must say so

`query_all_matches_every_event` and `racing_conditional_appends_elect_one_winner`
are superseded by strengthened successors. **Retiring them in code while leaving
the clause space silent moves the defect rather than fixing it** — `spec-trace`
reports an unclaimed rule forever, and "known noise in the checker" is how a
checker stops being read. Each retirement lands as a `Retires:` line on the clause
that owned it, in the same change as the code.

## Consequences

**Good.** The suite acquires the property it exists to give adapters. It is
strange that a crate whose purpose is *deciding whether an implementation is
correct* has had no way to demonstrate its own discrimination, and stranger that
four wrong implementations were measured passing it.

**Good.** `every_rule_has_a_mutant` makes the decorative-rule failure
*unwriteable* rather than merely discouraged. This repository has found that shape
four times by inspection; inspection does not scale and this does.

**Bad, and it is the real cost.** Every future rule costs a rule *and* a mutant,
roughly doubling the price of a conformance rule. That is the intended effect —
the cheapest rule to write is the one that asserts something obvious, which is
also the one most likely to be decorative — but it is a genuine tax on a crate
whose whole value is having lots of rules. If it ever produces a rule that is
clearly right and has no plausible failing implementation, the honest response is
to record that in the clause and skip the rule, not to invent a mutant for it.

**Bad.** Mutants are a second implementation of the port, in-tree, that must keep
compiling as the port changes. Phase 4 freezes the contract immediately after
this, so they will be rewritten once, early — which is the cheapest moment, and an
argument for doing this phase before phase 4 rather than after.

**Neutral.** The fixture change touches every existing conformance harness. There
are four, all in this workspace, and no published adapter binds to the old shape.
This is the last moment that will be true.

## Alternatives rejected

- **Mutation testing (`cargo-mutants`) over the reference store.** Attractive
  because it is automatic. Rejected: it mutates `MemoryEventStore`'s *code*, so it
  measures whether the suite pins that implementation, not whether the suite
  catches the mistakes a **different** storage engine makes. `LIMIT` before the
  tag filter is a SQL bug that has no in-memory analogue to mutate into. It is
  worth running later as a supplement; it does not answer CF-1.

- **A mutant per rule, mechanically.** Simpler bookkeeping, and it makes
  `every_rule_has_a_mutant` trivially satisfiable — which is the objection. One
  faithful mutant that fails four rules is worth more than four contrived ones,
  and the exactness test is what keeps the mapping honest either way.

- **Requiring a rejection without requiring exactness.** Half the cost and most of
  the appearance. Rejected: a mutant that fails everything is what you get by
  accident, and it satisfies "the rule rejected something" while proving nothing
  about *which* thing.

- **Leaving the fixture alone and deferring durability to phase 8.** The current
  shape is what makes CF-14 and CF-17 undecidable, so phase 8 would have to change
  the fixture *and* write the first real adapter against it in one step, with no
  way to tell a fixture bug from an adapter bug. The instrument goes first.
