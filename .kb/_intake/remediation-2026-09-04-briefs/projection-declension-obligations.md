# Option A moved the projection family's honesty obligation off the trait. Which of the three capabilities has anywhere for it to land?

Short answer up front: **`COMMIT_FAULT` already has one, `SECOND_HANDLE` never
needed one, and `RESET_REFUSAL` has none — so the retraction leaves exactly one
hole, and it is a clause the specification owner has to mint.**

**This brief did not get the two-critic pass the original thirteen had.** It was
written by the lane implementing the ratified Option A of
`fixture-declension-policy.md`, in the same session as the change it describes.
It carries its own strongest objection and answers it, which is the form, but
nobody independent argued the other side. Read it with that discount applied.

---

## Why this is owed

Option A's own sentence is that the honesty obligation moves *from* the trait's
requiredness *to* "a CF-39-shaped clause-level MUST per capability"
(`fixture-declension-policy.md`, Recommendation). Implementing the first half
without checking the second is how a change that was argued as a *trade* ships as
a straight loss. So: for each of `ProjectionFixture`'s three capabilities, is
there a clause-level MUST saying what *declaring* it commits the fixture to?

## What is true today, after the change

`crates/happenstance-testkit/src/contract.rs`, `ProjectionFixture`:

| Capability | Defaulted? | Obligation on declaring it | Enforced by |
|---|---|---|---|
| `SECOND_HANDLE` | yes, declined | none needed — a *declaration* is checked by every rule in the family, which reads back through a fresh `connect()` | the rules themselves; a false claim fails them |
| `COMMIT_FAULT` | yes, declined | **present**, in the constant's `# What declaring it commits the fixture to` section: *"a store that can absorb every fault its fixture is able to arm MUST decline this capability with that as its stated reason"* | `failed_commit_leaves_both_unchanged` requires the armed `commit` to answer `Err` |
| `RESET_REFUSAL` | yes, declined | **absent** | nothing |

`SECOND_HANDLE` is the easy one and it is worth saying why, because it is the
shape that makes the other two legible. A fixture cannot over-claim it and get
away with it: claiming it means every rule in the family opens a second handle
and reads through it, so a fixture that cannot actually do so fails on the
mechanism rather than on a promise about it. The claim is self-checking.

`COMMIT_FAULT` is not self-checking — a fixture can declare it and arm nothing —
and that is precisely why the CF-39-shaped MUST was written onto the constant
before this change, and why the rule
`failed_commit_leaves_both_unchanged` requires an `Err`. The obligation was
already where Option A wants it. Nothing was owed here and nothing was done.

`RESET_REFUSAL` is the hole. A fixture may declare it, override
`protect_from_reset` with an **empty body**, and turn
`refused_reset_changes_nothing` into a green result about a store that was never
asked to protect anything. The trait's provided `protect_from_reset` panics, so a
*forgotten* override aborts loudly; what passes vacuously is an override that
compiles, does nothing, and looks like honest code. That is `NoopFaultFixture`'s
exact shape one port over — the wrong implementation CF-39 was written against —
and this family has no clause and no named wrong implementation for it.

## Options

### Option A — mint a PS-clause of CF-39's shape for `RESET_REFUSAL`

*A fixture declaring `RESET_REFUSAL` supported MUST, after `protect_from_reset(id)`,
cause the next `reset(id)` to answer `ResetError::Refused`. A fixture whose store
refuses no reset MUST decline the capability with that as its stated reason.*
Plus, mirroring CF-39's second sentence, *the fixture MUST state the mechanism*.

- **Costs an adapter author:** nothing they do not already owe. A fixture that
  declares the capability already has to make the rule pass.
- **Costs the suite:** one new rule, of `arming_a_mid_batch_fault_makes_the_append_fail`'s
  exact shape — protect, reset, require `Refused` — plus a wrong implementation
  in the projection registry, which is `NoopFaultFixture` with `commit` swapped
  for `reset`. That rule carries CF-29's practical break.
- **Semver:** additive to the trait; the rule is CF-29's break in practice.
- **Forecloses:** nothing.

### Option B — leave it, and record that the projection family's declaration side is unchecked

- **Costs:** the hole stays, and it is now a hole with *no* compensating
  mechanism, where before the change it had one that was weak (requiredness
  compels a sentence, not a true one) but not nothing.
- **Buys:** no clause is minted in a family whose port is not frozen and whose
  first adapter has not run the suite. PS-1 is `[FROZEN]` and the rest of the
  PS-clause maturity sweep is already routed elsewhere; adding a clause into a
  family mid-sweep is how two sweeps collide.

## Recommendation

**Option A, but not by this lane and not before the PS-clause maturity sweep it
belongs to.** The obligation is owed and its shape is not in doubt — CF-39 is
written, its rule is written, and the projection analogue is a transliteration.
What is in doubt is *who* mints a clause in a family where PS-1 is `[FROZEN]`,
the port is gated behind `unstable-projection`, and a maturity sweep is already
routed to `unstable-projection-gate-and-clause-disposition`. Minting one here
would be the second sweep colliding with the first.

The concrete ask, so it is not lost: when that sweep runs, it should mint the
clause above **and** the rule, and register a `NoopProtectFixture` in
`tests/projection_mutation_coverage.rs` so the obligation has a named wrong
implementation rather than a sentence.

### The strongest argument against, in its own words

*The hole was opened by this change, so this change should close it.* That is the
right instinct and it is nearly right here. What makes it wrong is that the hole
was not opened by the change: requiredness never checked that
`protect_from_reset` did anything, and `refused_reset_changes_nothing` was
vacuous against an empty override before the default landed and is exactly as
vacuous after. What the change did was remove the *appearance* of a mechanism —
which is an improvement in legibility and a loss of nothing measurable. The
answer to "then why not fix it now" is only that a clause is not this lane's to
mint, and the audit's own routing says so for the sibling case
(`review-pre-publication-2026-09-03.md`, L3-01's *Routing*).

## Cost of delay

Low and not zero. No projection adapter has run the suite, so no fixture has
declared `RESET_REFUSAL` supported except the testkit's own instruments — the
population that could be lying about it is empty. It stops being empty at the
first projection adapter, which is also when the port is meant to be frozen, so
the clause wants to exist before that adapter rather than after it.

## What this does not settle

- Whether `Fixture::SECOND_HANDLE` and `Fixture::REOPEN` — the event-store
  family's two remaining **required** constants — should also be defaulted, so
  that Option A's "no fixture trait ever grows a required item" reads as a
  property of the traits rather than a promise about the future. This lane
  defaulted the projection family's three because that is what the ratification
  named, and left the event-store family's two alone, which leaves the two
  families asymmetric in the opposite direction to the one Option A removed. It
  is additive either way and free until stable `0.2.0`.
- Whether the default reason strings this lane wrote are the right sentences.
  They say what the testkit is in a position to know — that the fixture did not
  answer — rather than making a claim about a store nobody asked. A reviewer who
  thinks a default should read in the fixture's own voice, as
  `Fixture::MID_BATCH_FAULT`'s does, has a real argument and it is not taken here.
