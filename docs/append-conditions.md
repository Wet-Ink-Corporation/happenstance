# Appending under a condition

> **Answers:** `explanation` — Why does the store re-check my query when I append?

Making a decision and writing it down happen at two different moments. Other
writers can act in between, and the store is the only party present at both
moments. Handing the store your query and position with the append lets it
refuse a write whose decision has gone out of date.

This page explains why the check belongs in the store, what exactly the store
checks, and the one thing a condition cannot promise.

## Why the store re-reads, and not you

You could run your query again yourself, just before appending. It would not
help. By the time your append reaches the store, what you learned is already
out of date, because the gap you are trying to close is between your last look
and the store's write. Only the store sees both ends of that gap.

Handing the condition to the store moves the check to the one place where
"nothing has changed since I read" can actually be confirmed. The store checks
the condition and writes the events as one step.

That is also why a condition is not a lock. Nothing is held between the read
and the append. You say afterwards which events your decision depended on, and
the store checks that no new ones have arrived. A lock costs something on every
request. A condition costs something only on the requests that lose a race,
because the check happens where the writes are already being put in order.

## What the store re-runs

The condition carries two things: the query you read with, and the position
your read reached. The store refuses the append if any event matching that
query has been added *after* that position
([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics)).

After, and not at. The event sitting exactly at that position is one your read
returned to you, so your decision already took it into account. Refusing you
because of it would be refusing you over something you had already counted
([ES-26](../spec/SPECIFICATION.md#es-26--the-ac3-boundary-after-is-exclusive-from-is-inclusive)).

The program below shows both sides of that line. The first append is accepted
even though a matching seat sits at the boundary. Then the same condition is
used a second time, and this time it is refused, because the first append put
a matching event after the boundary.

```rust
use happenstance::{AppendCondition, AppendError, Event, EventStore};
use happenstance::{MemoryEventStore, Query, QueryItem, Tags};
use happenstance::read_decision_model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let store = MemoryEventStore::new();
    let held = Tags::from_pairs([("course", "c1")])?;
    let item = QueryItem::new(["SeatHeld"], held.clone())?;
    let seats = Query::from_items([item])?;
    let seat = Event::new("SeatHeld", &b"{}"[..])?.with_tags(held);
    store.append(&[seat.clone()], None).await?; // already held

    let (_seen, upto) = read_decision_model(&store, &seats).await?;
    let condition = AppendCondition::new(seats).after_opt(upto);

    // The seat at the boundary was read, so it does not refuse you.
    store.append(&[seat.clone()], Some(&condition)).await?;

    // Your own append now sits after the boundary, so a second use of
    // the same condition is refused.
    let again = store.append(&[seat], Some(&condition)).await;
    assert!(matches!(again, Err(AppendError::ConditionViolated(_))));
    Ok(())
}
```

The second refusal is worth remembering. A condition describes one decision,
made from one read. Once you have appended, your own event is part of what
changed, so the next decision starts with a new read.

## What the condition does not claim

The store can only check the events it holds, and those are not necessarily
every event that ever happened. If events your query would match have been
removed from the store, the check finds nothing and the append is accepted.
It is accepted silently, because nothing the store returns can tell a log that
lost history apart from one that never had any
([ES-40](../spec/SPECIFICATION.md#es-40--a-conditional-append-is-sound-only-over-a-complete-store)).

So a refusal is only as trustworthy as the store is complete. For an
application's own store, that is normally the whole log, and there is nothing
to do. For a store filled by copying events in from somewhere else, or one
that has had history removed outside this library, check completeness first.

## Notes on particular stores

Every store gives conditions the same meaning. They differ in how they order
the writes the check depends on.

### happenstance-postgres

PostgreSQL hands out positions outside the transaction, so two writers can
commit out of order. The store makes sure a reader never sees a later position
before an earlier one becomes visible
([ES-10](../spec/SPECIFICATION.md#es-10--position-order-is-visibility-order)).
Without that, an event could appear below a position you had already read past,
and a condition would never see it.

### happenstance-sqlite

One writer at a time. Positions are assigned while the store holds its write
lock, so they become visible in order without any extra work.
