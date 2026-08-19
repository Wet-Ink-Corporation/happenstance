# Your first encounter

> **Answers:** `tutorial` — How do I watch a consistency boundary refuse a write?

Three programs, in order. Each one compiles and runs on its own, so a step you
arrive at cold is a whole thing rather than a fragment of the one before it.

The third is the point. It ends with an append this library refuses, because the
consistency boundary a decision was read over still held when the write arrived.

## Append and read back

A store, two events carrying the same tag, and a query that finds them again.
Nothing is guarded yet — this is the vocabulary the next two steps build on.

```rust
use happenstance::{Event, EventStore, MemoryEventStore};
use happenstance::{Query, QueryItem, Tags, read_decision_model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let store = MemoryEventStore::new();
    let held = Tags::from_pairs([("course", "c1")])?;
    let seat = Event::new("SeatHeld", &b"{}"[..])?
        .with_tags(held.clone());
    store.append(&[seat.clone(), seat], None).await?;

    let item = QueryItem::new(["SeatHeld"], held)?;
    let seats = Query::from_items([item])?;
    let (events, upto) = read_decision_model(&store, &seats).await?;

    println!("{} seats held, read up to {upto:?}", events.len());
    Ok(())
}
```

```text
2 seats held, read up to Some(SequencePosition(2))
```

Events come back in the order the store assigned, and that order is the same one
every other reader of the store sees
([ES-8](../spec/SPECIFICATION.md#es-8--ordering)).

## A condition that holds

> **From step 1** — a `Query` over `SeatHeld` tagged `course=c1`, and the
> position the store had reached when you read it.
>
> Arrived here cold? [Start at step one](#append-and-read-back).

The write now carries a guard: append only if nothing matching that query has
landed above the position the read observed.

```rust
use happenstance::{AppendCondition, Event, EventStore};
use happenstance::{MemoryEventStore, Query, QueryItem, Tags};
use happenstance::read_decision_model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let store = MemoryEventStore::new();
    let held = Tags::from_pairs([("course", "c1")])?;
    let item = QueryItem::new(["SeatHeld"], held.clone())?;
    let seats = Query::from_items([item])?;

    let (taken, upto) = read_decision_model(&store, &seats).await?;
    let seat = Event::new("SeatHeld", &b"{}"[..])?.with_tags(held);

    let condition = AppendCondition::new(seats).after_opt(upto);
    let at = store.append(&[seat], Some(&condition)).await?;

    println!("{} seats seen, appended at {at:?}", taken.len());
    Ok(())
}
```

```text
0 seats seen, appended at SequencePosition(1)
```

Nothing had landed, so the append is admitted — `after` names the last position
you did see, and an event at exactly that position never rejects
([ES-26](../spec/SPECIFICATION.md#es-26--the-ac3-boundary-after-is-exclusive-from-is-inclusive)).

## A condition that refuses

> **From step 2** — an `AppendCondition` built from the query you read and the
> position you read it at.
>
> Arrived here cold? [Start at step one](#append-and-read-back).

This time another writer gets there first, carrying the same tag. The guard is
unchanged, and it is now standing over an event your decision never saw.

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

    let (_taken, upto) = read_decision_model(&store, &seats).await?;
    let seat = Event::new("SeatHeld", &b"{}"[..])?.with_tags(held);
    store.append(&[seat.clone()], None).await?;
    let condition = AppendCondition::new(seats).after_opt(upto);
    match store.append(&[seat], Some(&condition)).await {
        Err(AppendError::ConditionViolated(_)) => {
            println!("refused: ConditionViolated");
        }
        ok => panic!("the boundary did not hold: {ok:?}"),
    }
    Ok(())
}
```

```text
refused: ConditionViolated
```

That line is printed from inside the matched arm, so it cannot appear unless the
store really refused — which it is required to do, and required to report under
exactly that name
([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics)).
