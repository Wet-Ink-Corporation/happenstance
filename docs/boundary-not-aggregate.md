# Boundaries, not aggregates

> **Answers:** `explanation` — Where did my aggregate's consistency boundary go?

You model with aggregates, and this library asks you for none. This page says
what the aggregate was doing for you, what takes its place here, and what you
now do yourself that the aggregate used to do.

## The rule, and the answer you know

One rule, two limits, about two different things:

> A course holds thirty seats, and a student takes at most five courses.

The command that has to respect both is a single command: *subscribe s1 to
c1*. An aggregate is a consistency boundary and a transaction boundary at once,
and you choose it when the schema is written. So you choose:

- **`Course` is the aggregate.** Thirty seats is an invariant it holds. Five
  courses per student spans many `Course` instances, and one transaction
  changes one aggregate, so that limit moves out of the transaction: a read
  model of each student's courses, checked before the command, and a saga
  that compensates when two subscriptions for the same student cross.
- **`Student` is the aggregate.** The mirror image. Five courses is held;
  thirty seats becomes the read model and the saga.
- **One aggregate holds both.** Both limits are transactional, and every
  subscription anywhere now contends with every other on one instance.

None of these is a mistake. Each is the honest price of a boundary fixed
before anyone knew which decisions it would have to hold: either one limit is
enforced after the fact, by compensation, or both are enforced by putting every
subscription through one instance.

## What is drawn instead

The boundary here is drawn per decision, by a query. *Subscribe s1 to c1* rests
on two sets of events — the subscriptions tagged `course=c1` and the
subscriptions tagged `student=s1` — so the decision reads one query with one
item for each, and appends on condition that nothing matching that query has
appeared since the read. A condition's query is matched the way a read's is
([ES-27](../spec/SPECIFICATION.md#es-27--a-condition-matches-on-tags-not-only-on-types)),
so one condition covers both items, and with them both limits.

Neither entity is the boundary, and neither has to be chosen. The tagging, the
query and the guard are spelled out step by step in
[tag, query, fold, guard](carry-your-invariant.md#tag-query-fold-guard); this
page is about what that buys against the aggregate, not how to write it.

## One condition, both limits

The program below takes the subscribe decision three times. Each time another
writer lands one subscription between the read and the append, and the
function reports whether s1's append still went in.

```rust
use happenstance::{AppendCondition, AppendError, Event, EventStore};
use happenstance::{MemoryEventStore, Query, QueryItem, Tags};
use happenstance::read_decision_model;

type Failure = Box<dyn core::error::Error>;

async fn s1_takes_c1_after(other: [(&str, &str); 2]) -> Result<bool, Failure> {
    let store = MemoryEventStore::new();
    let course = Tags::from_pairs([("course", "c1")])?;
    let student = Tags::from_pairs([("student", "s1")])?;
    let seats = QueryItem::new(["Subscribed"], course)?;
    let load = QueryItem::new(["Subscribed"], student)?;
    let decision = Query::from_items([seats, load])?;
    // An empty store: no seats taken, no courses held, room under both.
    let (_seen, upto) = read_decision_model(&store, &decision).await?;

    let theirs = Event::new("Subscribed", &b"{}"[..])?
        .with_tags(Tags::from_pairs(other)?);
    store.append(&[theirs], None).await?; // another writer

    let mine = Event::new("Subscribed", &b"{}"[..])?
        .with_tags(Tags::from_pairs([("course", "c1"), ("student", "s1")])?);
    let condition = AppendCondition::new(decision).after_opt(upto);
    match store.append(&[mine], Some(&condition)).await {
        Ok(_) => Ok(true),
        Err(AppendError::ConditionViolated(_)) => Ok(false),
        Err(failed) => Err(failed.into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Failure> {
    // s2 on c2 touches neither half: a different decision, admitted.
    assert!(s1_takes_c1_after([("course", "c2"), ("student", "s2")]).await?);
    // s2 took a seat on c1: the course's half moved, refused.
    assert!(!s1_takes_c1_after([("course", "c1"), ("student", "s2")]).await?);
    // s1 took c3: the student's half moved, refused.
    assert!(!s1_takes_c1_after([("course", "c3"), ("student", "s1")]).await?);
    Ok(())
}
```

Read the three assertions against the three aggregate designs.

The second one moved the count the course limit is checked against, and the
`Student` aggregate could not have refused it in the transaction. The third
moved the count the student limit is checked against, and the `Course`
aggregate could not have. Here the same append refuses both, with no read model
consulted and nothing to compensate: each refusal is the `ConditionViolated`
the function matches on, and s1's subscription is not in the store after it
([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics), checked by
`condition_rejection_leaves_store_unchanged`).

Neither refusal says a limit was exceeded. It says the events the decision
counted have moved, so the decision is taken again against what is there now,
and that retaking is where thirty and five are compared. What the condition
buys is that no write which could change either count lands between the read
and the append unseen
([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics)).

The first is the one the single large aggregate gets wrong in the other
direction. Nothing s2 did on c2 is in the decision's query, so it gives the
condition nothing to refuse on: its events are the same type, and a condition
matches on tags as well as types
([ES-27](../spec/SPECIFICATION.md#es-27--a-condition-matches-on-tags-not-only-on-types),
checked for every adapter by the conformance rules
`condition_with_an_unheld_tag_does_not_reject` and
`condition_matches_on_tags`). Where two decisions do overlap and race, exactly
one of them lands
([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics), checked by
`racing_conditional_appends_elect_one_winner` and, under real contention, by
`exactly_one_of_n_contenders_commits`).

This fence is compiled and its assertions run by this repository's gate, so
the three outcomes above are checked rather than described.

> **Status.** Under real contention, the independence of decisions that share
> nothing is a narrower promise than the program above shows. The conformance
> rule `k_disjoint_boundaries_never_conflict` holds that they are never refused
> as `ConditionViolated` on each other's account and that at least one of them
> commits; a store may still refuse one as `Busy`
> ([ES-43](../spec/SPECIFICATION.md#es-43--a-busy-refusal-wrote-nothing-and-says-so)).
> No clause states that independence yet, and
> [the open question](../.kb/open-questions/disjoint-boundaries-have-no-clause.md)
> records the decision it is owed.

## What the aggregate did that you now do

The aggregate gave the invariant a home: one class, one place to look, and a
boundary nobody could narrow without noticing. Here the invariant lives in the
query, and a query can be too narrow without anything failing. Tag the seats
item to the student instead of the course, and the seats other students take
never enter the read, the condition does not guard them, and the course can go
over capacity with no error
([CF-8](../spec/SPECIFICATION.md#62-the-measured-gaps)).
[What a narrow guard misses](carry-your-invariant.md#what-a-narrow-guard-misses)
is that failure as a compiled program, and it is the cost of this model that
the aggregate never charged you.

The home comes back one layer up. In the typed layer a decision model is a
value with a scope, and a tuple of decision models is one boundary: the
worked example's `subscribe` handler composes the course's seats with one
student's history with that course, and one append condition checks both,
retaking the decision when that condition refuses.
[The example's source](../examples/course-subscriptions/src/main.rs) is that
handler, and its `tests/runs.rs` runs the program and asserts that it refuses
a course that is full and a student already subscribed to it.

## Where this page stops

The aggregate is a consistency boundary, and that is what this page answers.
The other habit you may have brought — one stream per entity, and the question
of which stream a two-entity rule is written to — is a storage reflex rather
than a boundary, and it is answered under
[where your streams went](carry-your-invariant.md#where-your-streams-went).
Why the store re-runs your query rather than you is
[appending under a condition](append-conditions.md).
