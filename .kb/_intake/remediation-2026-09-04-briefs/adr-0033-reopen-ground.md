# ADR-0033's reopen condition is scoped to ceremony volume. What carries the soundness ground it excludes?

Decision record: **ADR-0033-reopen-ground**. Brief only — no ADR prose, no atom,
no edit to `.kb/decisions/0033-happenstance-macros-out-of-scope-for-0-1.md`, and
none is possible: the record is accepted, and `redkiln validate --kb` checks
accepted decision bodies against `HEAD`.

From `Y-6` of `references/evaluation/review-pre-publication-2026-09-03.md`
(`:3774-3807`), written by the rendered-pages lane, which holds `docs/` and
`examples/` and holds neither the decision tree nor the specification. It is a
brief rather than a change for that reason and for one more: **re-scoping an
accepted record's reopen condition is an ADR act**, and this repository's rule
is that ADR authorship belongs to the RUNBOOK's ADR pass and never to a review
or a remediation lane as a side effect.

---

## Why this is owed

ADR-0033 decided `happenstance-macros` out of scope for 0.1, and the verdict is
sound. It is published range by range over 532 of 532 lines, both extreme
assignments of the contested block agree (0.50:1 and 0.12:1 against a mechanical
1.0 threshold), and nothing since has moved the ratio — the measurement below
moves it *further* out.

The problem is not the verdict. It is the reopen condition
(`.kb/decisions/0033-happenstance-macros-out-of-scope-for-0-1.md:120-125`):

> *"Reopen **if and only if** D-1 (`kb-open-question-d-1-no-total-path-001`) is
> settled with an infallible `Tags` path. A derive that also handled tags
> constructed from runtime values would reach into the contested 85 lines rather
> than only the 40-line `DomainEvent` impl — but even the full 85-line swing does
> not cross the 1.0 threshold from 0.50:1, so this stays a post-0.1 question."*

Every criterion in it is **volume**: lines, percentages, a ratio against a
threshold. That is the right answer to AC-013's question, which asks whether the
worked example carries more mapping ceremony than domain logic.

It is not an answer to the ground `Y-2` and `B-3` raise, which is not a volume
claim at all: that the hand-written mapping is spelled **positionally** with no
compiler check, and that a documented obligation is discharged nowhere. An `if
and only if` gated on D-1 excludes that ground by construction.

**The wrong outcome is procedural.** The next person who meets the positional
mapping in the field opens ADR-0033, finds an accepted immutable record whose
reopen trigger their evidence does not satisfy, and either files nothing or
writes a superseding record whose relationship to 0033 is unclear — because 0033
never claimed the ground being contested. Making `superseded_by` mean *extended*
is the one thing that graph must not come to mean.

## What is true today

### The volume ground has been re-measured, and it holds harder than before

`kb-reference-macros-ceremony-measurement-001`
(`.kb/reference/phase-7-macros-ceremony-measurement.md`) was taken 2026-08-16
against `examples/course-subscriptions/src/main.rs` at `78a2170`, and has never
been re-taken against the **second** worked example, which was written after
ADR-0033. Y-6 names that as the cheap input available before any decision.

Taken 2026-09-04 against `lane/rendered-pages` at `b87b17b`. This is the
measurement's own cheapest figure — *what a derive would have bought*, the
`impl DomainEvent` block as a share of the file — and not a re-run of the
four-bucket classification, which is 29 hand-adjudicated ranges and is not
something a remediation lane should re-adjudicate:

| Substrate | `impl DomainEvent` | file | share |
|---|---:|---:|---:|
| `course-subscriptions/src/main.rs` @ `78a2170` (the atom) | 40 (`:209-247`) | 532 | 7.5% |
| `course-subscriptions/src/main.rs` @ `b87b17b` | 39 (`:188-226`) | 511 | 7.6% |
| `transfers-on-sqlite/src/main.rs` @ `b87b17b` | **35** (`:281-315`) | 663 | **5.3%** |
| both worked examples, combined | 74 | 1,174 | 6.3% |

Two things follow, and the second is the one that matters.

**The second worked example moves the number down.** 35 lines of 663 against 40
of 532: three event variants in both, and the impl is a near-fixed cost while the
domain around it grew. That is the atom's own structural explanation confirmed on
a substrate it never saw, and it is a *strengthening* of ADR-0033's verdict.

**The atom's substrate has moved and the atom has not, correctly.** It is pinned
to `78a2170` and reads under the same rule as `references/evaluation/*` — dated
evidence, not a claim held current. `main.rs` is 21 lines shorter today and the
impl sits at `:188-226`. Anyone re-deriving the ratio from the atom's line ranges
against `HEAD` gets a different file. Noted here rather than repaired, because
repairing a dated measurement is how it stops being one.

### The ground the reopen condition excludes is present in both examples

The positional mapping `Y-2` names is not one example's local choice. Both
worked examples spell `event_type` the same way — three `Self::EVENT_TYPES[n]`
subscripts each, six in the tree, each of which pairs a variant with a list
position and none of which the compiler checks
(`examples/course-subscriptions/src/main.rs:195-201`,
`examples/transfers-on-sqlite/src/main.rs:288-294`):

```rust
    fn event_type(&self) -> EventType {
        match self {
            Self::AccountOpened { .. } => Self::EVENT_TYPES[0].clone(),
            Self::Deposited { .. } => Self::EVENT_TYPES[1].clone(),
            Self::Withdrawn { .. } => Self::EVENT_TYPES[2].clone(),
        }
    }
```

Reorder `EVENT_TYPES` and every event in the store is written under the wrong
type, silently, on the write path. Nothing in the workspace rejects it: it is not
a clause, not a conformance rule, and not a lint. It is also exactly six lines of
the 74 the volume measurement counts, so **a criterion made of volume cannot see
it** — six lines is 0.5% of the two files and the threshold is 1.0.

That is the whole of Y-6 in one number. The ground in dispute is not small in
consequence and *is* small in lines, and ADR-0033's condition is denominated in
lines.

### The precedent for widening a record without superseding it exists here

ADR-0029 **amends** ADR-0004 rather than superseding it — `CLAUDE.md`'s own
binding-constraints section says so in those words, and constraint 5 is written
as a struck-through rule with its amendment beside it. So the repository has
already run the move this question needs, once, on its MSRV floor.

## Options

### Option A — A new record that amends ADR-0033's reopen condition

Written the way ADR-0029 amends ADR-0004: 0033 stays accepted and unedited, the
new record widens the reopen ground to *either* D-1 settling with an infallible
`Tags` path *or* an unchecked mapping obligation being demonstrated in the tree,
and the supersession graph is untouched.

Costs a decision-pass slot and nothing else. Costs no caller anything: a derive
is an additive item on a crate that does not exist, so there is **no semver
surface at all** and nothing here competes for the `0.2.0` window.

Its weakness is that it spends an ADR on a reopen condition for a decision
nobody currently wants reopened, which is a real thing to spend an ADR on only
if the condition is going to be consulted.

### Option B — Leave ADR-0033 alone and record the ground as an open question

`Y-2`'s subject is *"a positional mapping with no compiler check"*, which is a
question about the typed layer's ergonomics and not about `happenstance-macros`
at all. Filed as its own open question beside D-1, it needs no reopen condition,
no amendment and no supersession — and when it is settled, whether it reaches a
derive is a consequence rather than a premise.

Cheapest of the three and the least irreversible. Its weakness is the failure
Y-6 actually names: the next reader still opens ADR-0033, still finds a
condition their evidence does not satisfy, and there is nothing in 0033 pointing
them at the open question. An open question nobody is routed to is a file.

### Option C — Both, sequenced: the open question first, the amendment only if it is consulted

File Y-2's ground as an open question now (Option B), and let the amendment
(Option A) wait for the first time somebody actually reaches ADR-0033 with
non-volume evidence. The trigger is observable: it is a comment, an issue or a
review entry citing 0033's reopen condition against soundness ground.

Its weakness is that "wait for it to happen" has no owner and no date, and this
repository's own history is that a finding raised three times is a missing check
— the argument `xtask/src/main.rs:625-635` makes about a README that told
crates.io the wrong rule count through fifteen commits.

## Recommendation

**Option C**, and with low confidence — this brief is one lane's reading of a
decision tree it does not own, and the RUNBOOK's ADR pass has context it does
not.

The reason is the semver line, which is Y-6's own strongest point and is easy to
lose: **this is not a freeze-window decision.** A derive is additive on a crate
that would not exist, D-1 stays where it is, and nothing about the reopen
condition gets more expensive after `0.2.0`. Everything that argues for acting
now is an argument about a reader's experience of the decision tree, and the
cheap half of that — a routable open question — is available without an ADR.

### The strongest argument against, in its own words

*Option C is Option B with an intention attached, and an intention is not a
mechanism. The whole finding is that a reader will arrive at ADR-0033 and be
turned away by a condition that was never about their evidence; C leaves that
reader turned away and asks them to be the trigger. The one moment the
amendment is cheap is now, while somebody has the finding in hand and the
measurement is fresh — and the amendment is four sentences in a record that
costs no caller anything. "Wait until it bites" is how the reopen condition got
written narrowly in the first place.*

The objection is right that C's trigger has no owner, and that is the reason the
recommendation is C-and-not-B rather than C-with-confidence. Where it is wrong
is the claim that the amendment is cheap *because* it is small. An amendment to
an accepted record is a permanent edge in a graph whose meaning this repository
guards closely enough to have a validator for; spending one to widen a condition
on a decision nobody is contesting is exactly the kind of ceremony ADR-0033
itself declined to buy.

## Cost of delay

**Low, and it does not compound.** Nothing here is in the `0.2.0` window, no
consumer is exposed, and the positional-mapping hazard the ground is about is
unchanged whether or not the reopen condition mentions it.

One asymmetry, and it is small: the re-taken measurement above is fresh now and
will be stale the next time either worked example is edited. Whoever takes this
gets it for free today and pays for it later.

## What this does not settle

- **D-1.** Untouched. The finding does not settle it and does not need it
  settled; the whole point of Y-6 is that the reopen condition should not
  depend on it exclusively.
- **Whether the positional mapping should be fixed at all**, and how. A derive
  is one answer; a `const` assertion pairing each variant with its index is
  another and is not a macro crate. This brief prices neither.
- **Whether `kb-reference-macros-ceremony-measurement-001` should be re-taken
  properly.** The measurement above is the atom's *cheapest* figure over a
  second substrate, not a second 29-range classification. If a decision is going
  to turn on the ratio, the classification is what it turns on, and re-taking it
  is a judgement call this lane is not entitled to make.
- **What routes a reader from ADR-0033 to wherever Y-2's ground ends up.** All
  three options above leave that as prose in some file; none of them makes it
  mechanical, and nothing in `redkiln validate --kb` checks that a decision
  points at the open questions that qualify it.

## Provenance

Author: the rendered-pages remediation lane, 2026-09-04, in the same session as
the `docs/` and `examples/` changes it describes, and **without** the author →
two-critic → revision pass the original thirteen briefs in this directory had.
Nobody independent argued the other side. Its strongest objection is stated and
answered above, which is the form, and the discount this directory's `README.md`
applies to the later briefs applies to this one.

Measurements are reproducible: `wc -l` and the two `impl DomainEvent` block
bounds at `lane/rendered-pages` `b87b17b`. The line numbers cited into
`examples/` were taken by grep at that commit and are subject to the drift this
brief itself records one section up.
