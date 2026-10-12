# Boundaries, not aggregates

> **Answers:** `explanation` — If I don't define aggregates, what keeps my decisions consistent?

If you design event-sourced systems with aggregates, happenstance will feel
like something is missing: it never asks you for one. This page shows what the
aggregate was doing for you, what does that job here, and the one thing you now
have to take care of yourself.

## The rule, and the answer you know

Take one request that has to respect two limits about two different things:

> A course holds thirty seats, and a student takes at most five courses.

The request is *subscribe s1 to c1*. An aggregate is both a consistency
boundary and a transaction boundary, and you choose it when you design the
streams. As [aggregates, and the boundary drawn in advance](aggregates.md)
describes, that leaves three choices:

- **`Course` is the aggregate.** It holds the thirty-seat limit. The five-course
  limit spans many courses, and one transaction changes one aggregate, so that
  limit moves outside the transaction. It is checked against a read model of
  each student's courses, and a saga undoes the extra subscription when two
  requests for the same student cross.
- **`Student` is the aggregate.** The mirror image. The five-course limit is
  held, and the thirty-seat limit becomes the read model and the saga.
- **One aggregate holds both.** Both limits are held in the transaction, but
  every subscription anywhere now queues behind every other on one aggregate.

None of these is a mistake. Each is the price of fixing the boundary before
knowing which decisions it would have to hold. Either one limit is enforced
after the fact, by repairing it, or both are enforced by sending every
subscription through one place.

## What is drawn instead

Here, each decision draws its own boundary with a query. *Subscribe s1 to c1*
depends on two sets of events: subscriptions tagged `course=c1`, and
subscriptions tagged `student=s1`. So the decision reads one query with one
item for each set, and appends on condition that nothing matching that query
has arrived since the read.

A condition's query matches events the same way a read's query does
([ES-27](../spec/SPECIFICATION.md#es-27--a-condition-matches-on-tags-not-only-on-types)),
so one condition covers both items, and with them both limits.

Neither the course nor the student is the boundary, and you never have to pick
one. How to write the tags, the query and the guard is covered step by step in
[tag, query, fold, guard](carry-your-invariant.md#tag-query-fold-guard). This
page is about what that buys you compared with an aggregate.

## One condition, both limits

The program below takes the subscribe decision three times, each time against
a fresh store. Every time, another writer adds one subscription between the
read and the append. The function returns `true` if s1's subscription was still
accepted, and `false` if the store refused it.

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

Now compare the three outcomes with the three aggregate designs.

The second call adds a subscription to c1, which changes the seat count the
course limit is checked against. A `Student` aggregate could not have refused
that inside its transaction. The third call subscribes s1 to another course,
which changes the count the student limit is checked against, and a `Course`
aggregate could not have refused that.

Here the same append refuses both, with no read model and nothing to repair
afterwards. Each refusal is the `ConditionViolated` error the function matches
on, and s1's subscription is not in the store after it
([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics), checked by the
conformance test `condition_rejection_leaves_store_unchanged`).

A refusal does not mean a limit was exceeded. It means the events the decision
counted have changed, so the decision has to be made again against what is
there now. That second attempt is where thirty and five are actually compared.
What the condition guarantees is that no write which could change either count
slips in unseen between the read and the append
([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics)).

The first call is the one the single large aggregate gets wrong, in the other
direction. s2 subscribing to c2 matches neither item of s1's query, so it gives
the condition nothing to refuse on, even though its event has the same type. A
condition matches on tags as well as types
([ES-27](../spec/SPECIFICATION.md#es-27--a-condition-matches-on-tags-not-only-on-types),
checked against every store by the conformance tests
`condition_with_an_unheld_tag_does_not_reject` and `condition_matches_on_tags`).

When two decisions do overlap and race, exactly one of them is accepted
([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics), checked by
`racing_conditional_appends_elect_one_winner` and, with many writers at once,
by `exactly_one_of_n_contenders_commits`).

The program above is compiled and its assertions are run by this repository's
tests, so these three outcomes are checked, not just described.

> **A narrower promise under load.** With many writers at once, decisions that
> share no events are still never refused with `ConditionViolated` because of
> each other, and at least one of them is accepted. But a store may still
> refuse one of them as `Busy`, meaning it wrote nothing and you can retry
> ([ES-43](../spec/SPECIFICATION.md#es-43--a-busy-refusal-wrote-nothing-and-says-so)).
>
> The conformance test `k_disjoint_boundaries_never_conflict` checks this. The
> specification has no clause stating that independence yet, and
> [the open question](../.kb/open-questions/disjoint-boundaries-have-no-clause.md)
> records the decision still to be made.

## What the aggregate did that you now do

The aggregate gave each rule a home: one class, one place to look, and a
boundary nobody could narrow without noticing. Here the rule lives in a query,
and a query can be too narrow without anything failing.

Tag the seats item with the student instead of the course, and seats taken by
other students never enter the read. The condition does not guard them, and the
course can go over capacity with no error
([CF-8](../spec/SPECIFICATION.md#62-the-measured-gaps)).
[What a narrow guard misses](carry-your-invariant.md#what-a-narrow-guard-misses)
shows that failure as a runnable program. It is the one cost of this approach
that aggregates never charged you.

The typed layer gives the rule a home again. There, a *decision model* is a
value that knows its own scope (the tags it reads), and several decision models
combined in a tuple form one boundary. The worked example's `subscribe` handler
combines the course's seats with one student's history on that course. One
append condition checks both, and the decision is retried when the condition
refuses.

[The example's source](../examples/course-subscriptions/src/main.rs)
contains that handler, and its `tests/runs.rs` runs the program and checks that
it refuses a full course and a student who is already subscribed.

## Where this page stops

This page was about the aggregate as a consistency boundary. The other habit
you may have brought with you, one stream per entity and the question of which
stream a two-entity rule is written to, is about storage rather than
consistency. It is answered under
[where your streams went](carry-your-invariant.md#where-your-streams-went). Why
the store, rather than your code, checks that nothing has changed is explained
in [appending under a condition](append-conditions.md).
