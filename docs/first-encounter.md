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

This time the store is not empty. The write carries a guard: append only if
nothing matching that query landed above the position the read observed.

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
    let seat = Event::new("SeatHeld", &b"{}"[..])?.with_tags(held);
    store.append(&[seat.clone()], None).await?;

    let (seen, upto) = read_decision_model(&store, &seats).await?;
    let condition = AppendCondition::new(seats).after_opt(upto);
    let at = store.append(&[seat], Some(&condition)).await?;

    println!("{} seen, up to {upto:?}, at {at:?}", seen.len());
    Ok(())
}
```

```text
1 seen, up to Some(SequencePosition(1)), at SequencePosition(2)
```

A matching event already sits at exactly the position you read to, and the
append is still admitted: `after` is exclusive, and an event at exactly that
position never rejects ([ES-26](../spec/SPECIFICATION.md#es-26--the-ac3-boundary-after-is-exclusive-from-is-inclusive)).

## A condition that refuses

> **From step 2** — an `AppendCondition` built from the query you read and the
> position you read it at.
>
> Arrived here cold? [Start at step one](#append-and-read-back).

One seat is already held; your decision reads it, and the guard carries the
position that read observed. Another writer lands a second seat before yours.

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

    let (_taken, upto) = read_decision_model(&store, &seats).await?;
    store.append(&[seat.clone()], None).await?; // another writer
    let condition = AppendCondition::new(seats).after_opt(upto);
    match store.append(&[seat], Some(&condition)).await {
        Err(AppendError::ConditionViolated(_)) => {
            println!("guarded above {upto:?}: ConditionViolated");
        }
        ok => panic!("the boundary did not hold: {ok:?}"),
    }
    Ok(())
}
```

```text
guarded above Some(SequencePosition(1)): ConditionViolated
```

That line is printed from inside the matched arm, so it cannot appear unless the
store really refused — which it is required to do, and required to report under
exactly that name
([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics)).

### Try it wrong, then put it back

The program above is compiled and run by this repository's own gate, so you can
break it and watch something fail. Two minutes, and one expression.

**The edit.** In the fence above, drop the guard from the append — change
`Some(&condition)` to `None`, and leave everything else alone. It still
compiles, and it still runs. Then:

```text
cargo test -p xtask --doc -- first_encounter
```

**What you should see.** The run goes red, and this is the line to look for. It
is the program telling you what it got instead of a refusal:

```text
the boundary did not hold: Ok(SequencePosition(3))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The failing test is named `narrative::first_encounter (line 96)`, after this
page. The path in the panic above it is not this page: doctests report against a
temporary file, and the test's name is the part that identifies where you are.

**Putting it back.** Change `None` back to `Some(&condition)`. Re-run the same
command and it passes again, with nothing else in your tree to repair.

An append that is accepted where it should have been refused is a lost update
with no error anywhere. You have just watched the single expression that is the
difference.
