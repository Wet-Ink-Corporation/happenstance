# `Kind::ModelOnlyMutant` now has no members. Is a memberless kind withdrawn, kept dormant, or is the emptiness itself the finding?

Short answer up front: **keep it, dormant, and revisit at first publish** — but the
recommendation is held at *medium* confidence and the strongest argument against it
is that a sibling kind was withdrawn on this exact question one day earlier, for a
reason that does not apply here and could be mistaken for one that does.

**This brief did not get the author → two-critic → revision pass the original
thirteen had.** It was written by the lane that emptied the kind, in the same
session as the change it describes. It carries its own strongest objection and
answers it, which is the form, but nobody independent argued the other side. Read
it with that discount applied.

---

## What happened, so the question is legible

`Kind::ModelOnlyMutant` records a store with a named defect that **no rule of the
event-store family can see**, and which the generative *model* family therefore
has to. It has carried exactly one member since it landed:
`UnparenthesisedToPredicateStore` — `WHERE a OR b AND position <= ?`, the upper
bound conjoined with the last disjunct alone.

`read_to_composes_with_multi_item_query` landed today and sees that store. The row
is an ordinary `Kind::Mutant` with one entry in `fails`, pinned to the rule's own
assertion, and the store's documentation had said in terms that this is what would
end its filing:

> If someone writes the missing rule, this row becomes an ordinary
> `Kind::Mutant` with one entry in `fails` and the meta-tests say so.
> (`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs`, the
> `UnparenthesisedToPredicateStore` doc, before this change)

So the kind is empty, and `MODEL_ONLY_WITNESSES` with it — a witness whose owner is
not a model-only mutant is rejected by
`every_model_only_mutant_demonstrates_its_defect`'s second loop, by name.

## Why the emptiness could not simply be left alone

`the_model_only_bar_rejects_a_store_with_no_defect` opened with

```rust
assert!(!MODEL_ONLY_WITNESSES.is_empty(), "no witnesses, so this control asserts nothing");
```

That assertion is right while the kind has a member and **wrong the moment it does
not**: with no member, the bar it controls
(`every_model_only_mutant_demonstrates_its_defect`) iterates over nothing too, so a
control that ran would be controlling nothing — and a bare `!is_empty()` demands a
witness for a store that must not have one.

The lane replaced it with an **equivalence**: the witness table is empty exactly
when the kind has no members. A row without a member is an orphan scenario; a
member without a row is the hole `HidingPlaceStore` walked through. Both directions
stay checked, and the day a store is filed here again the control is non-vacuous
with it.

That edit is not the decision this brief is about. It is the *minimum* that keeps
the tree honest under either answer below, and it is reversible under both.

## The question

Three options, and the difference between them is what a future author finds when
they reach for the kind.

### Option A — keep the variant, dormant (what landed)

The `Kind::ModelOnlyMutant` variant, the `Witness` struct, `MODEL_ONLY_WITNESSES`,
`HidingPlaceStore`, and both meta-tests stay in the tree with no rows to drive.

- **Costs a reader**: a vocabulary entry with no example. The doc now says so
  explicitly and says why, which is the mitigation, and it is the same mitigation
  the `[FROZEN]`/`[DEFERRED]` markers use one document over.
- **Costs an adapter author**: nothing. None of this is public API;
  `tests/mutation_coverage.rs` is a test target of a crate that publishes no test
  targets.
- **Semver**: none. Not reachable from outside the crate in any configuration.
- **Buys**: the next defect of this shape has somewhere to go on the day it is
  found, with the three obligations already written and already checked. There
  *will* be another: the shape is "a defect needing a query and a read option
  together, where that option's rules all issue `Query::all()`", and the suite has
  now met it twice — once for `from` (CF-12, phase 3) and once for `to` (today).

### Option B — withdraw the variant

Delete `Kind::ModelOnlyMutant`, `Witness`, `MODEL_ONLY_WITNESSES`,
`HidingPlaceStore` and the two meta-tests over them. Roughly 200 lines, all of it
documentation and machinery, none of it driving anything.

- **Buys**: `Kind` becomes two variants — a store fails rules or it does not — and
  the registry stops carrying an escape hatch that says *"nothing here catches
  this."* CF-1's bar is "name a plausible wrong implementation the rule rejects",
  and a kind whose content is "no rule rejects it" sits uneasily beside that.
- **Costs**: the next author who meets the shape re-derives all three obligations,
  and the review that found `HidingPlaceStore` has to happen again. That review is
  the expensive part: it took an adversarial pass to notice that a kind borrowing
  another *family* for its bar can have the bar compiled out by
  `--no-default-features`, and the repair is not obvious in advance.
- **Semver**: none, as above.

### Option C — keep it and treat the emptiness as the standing measurement

Option A plus a named test asserting the kind is empty, so that *filing* a store
here is a deliberate act that turns a green test red and forces the author to
justify it.

- **Buys**: the kind stops being a place to put something quietly.
- **Costs**: it inverts the meaning of the machinery — the kind becomes something
  to be argued *out of* rather than a category — and it would have to be deleted
  the moment a legitimate member arrives, which is a check that exists to be
  removed. `reopen_over_claiming_is_undetectable_and_this_is_the_record` is the
  precedent for a test that records rather than checks, and it records something
  that cannot change; this would record something that is expected to.

## Recommendation

**Option A**, at medium confidence, and revisited at phase 12 alongside the
testkit's public-surface pass rather than now.

The argument is asymmetry of cost. Keeping an empty variant costs a paragraph of
documentation and nothing else — it is not public, not compiled into anything a
consumer sees, and not a maintenance burden the way an unused *feature* would be.
Withdrawing it costs the recovery of a non-obvious design if the shape recurs, and
the shape has recurred once already in nine phases. The moment to withdraw a
vocabulary entry is when the *shape* it names has been shown not to exist, and what
was shown today is the opposite: the shape exists, it has now been met twice, and
both times the answer was a new rule rather than the kind.

## The strongest argument against, in its own words

> *`Kind::StatedOnlyDefect` was withdrawn one day ago on exactly this question and
> the tree is better for it. Keeping a memberless kind is how a registry acquires
> vocabulary nobody uses, and the argument "someone might need it" is the argument
> that keeps every dead abstraction alive. Delete it; `git log` remembers the
> design, and the review that produced it is on file in
> `stated-only-defects-and-the-reopen-must.md` and in the `HidingPlaceStore`
> doc-comment, neither of which goes away.*

That is a real argument and it is the reason this brief is medium rather than high.
The answer is that the two withdrawals are not the same withdrawal.
`Kind::StatedOnlyDefect` was withdrawn because its **bar was unsound** — the
scenario it demanded was unsatisfiable in principle for the only case it was
invented for, and two string literals satisfied it. It was not withdrawn for being
empty; it was withdrawn for being wrong. `Kind::ModelOnlyMutant`'s bar was also
found unsound once, in July's `HidingPlaceStore` review, and it was **repaired**
rather than withdrawn: obligation 2 — a feature-independent witness driving the
store's own `Defect::select` — is what that repair added, and it held. A kind whose
bar has been attacked and survived is a different asset from a kind whose bar was
shown to be unsatisfiable.

The objection's second half stands unanswered and should be recorded as such: if
phase 12 arrives with the kind still empty, "the shape recurs" will have been a
prediction that did not pay, and Option B becomes the better answer on its own
terms. That is the review trigger, and it is why this brief recommends revisiting
rather than settling.

## Cost of delay

Low, and it does not compound. Nothing blocks on this: no adapter, no clause, no
release. The one thing that would make it urgent is a *third* store arriving that
genuinely no rule can see, which would settle it in the other direction by
supplying the member.

## What this does not settle

- **Whether the two meta-tests should be merged.** With the kind empty, three tests
  (`every_model_only_mutant_demonstrates_its_defect`,
  `the_model_only_bar_rejects_a_store_with_no_defect`, and the `Kind` arm inside
  `mutant_registry_is_exhaustive`) police a category with no members. If Option A
  is ratified, whether that is three tests or one is a separate and smaller
  question.
- **The backwards `to` × `limit` composition.** A separate residual from the same
  lane, recorded here because it has no other home:
  `read_to_composes_with_limit` reads forwards only. The wrong implementation that
  needs backwards *and* a window *and* a budget — a windowed statement written
  ascending with the `LIMIT` inside it and the direction applied by an outer
  `ORDER BY` — is constructible and was deliberately not written, because
  asserting it in that rule would make `BackwardsToIsAnUpperBoundStore`,
  `BackwardsIgnoredStore` and `FetchOneExtraStore` fail it for reasons three other
  rules own. Whether that is a fourteenth read-option rule or an accepted gap is
  the rule-set owner's, before the first SQL adapter ships a windowed query.
- **Whether `Kind` should be `#[non_exhaustive]`.** It is a private test-target
  enum, so the question is style rather than semver, and it is not this brief's.
