# Your first encounter

> **Answers:** `tutorial` — How does happenstance stop two writers breaking a rule?

This page shows the one idea the rest of happenstance is built on, using three
short programs you can run. You do not need to know event sourcing or DCB to
follow it: each term is explained the first time it is used. For the ideas
behind it, start with [events, and the systems built on them](event-driven-systems.md).

## The problem: two people, one last seat

Picture a course with one seat left. Two students press "subscribe" at the same
moment. Each request checks how many seats are taken, sees one free, and books
it. Both succeed, and the course is now over capacity. Nothing crashed and no
error was logged. Each request acted on a count that was already out of date by
the time it wrote. This is called a *lost update*.

happenstance records what your application does as **events**: small, permanent
records of something that happened, such as "a seat was held". You never update
an event or delete one; you append new ones to a **store**. To make a decision,
such as "is there a seat left?", you read back the events that decision depends
on and work the answer out from them.

The defence against the lost update is in the write. When you append, you can
attach a **condition**: "accept this only if nothing matching what I read has
been added since I read it". If another student got there first, the store
refuses your append. You read again, see that the course is now full, and
decide differently. Nothing is overbooked, and nothing fails silently.

The events a decision read are its **consistency boundary**: the part of the
store that has to stay unchanged for the decision to still be right. Each
decision picks its own boundary by choosing what to read, so nothing about it is
fixed in advance. That is the *dynamic* in **dynamic consistency boundary**, or
DCB, the [specification](https://dcb.events/specification/) this library follows.

## Three programs, each building on the last

1. **Append and read back.** Write two events, then find them again with a
   query. This introduces events, tags, queries and positions.
2. **A condition that holds.** Append with a condition when nothing has changed
   since the read, and watch the store accept it.
3. **A condition that refuses.** Let another writer slip in between your read
   and your write, and watch the store refuse.

Each program is complete, so you can copy any one of them into a project and
run it without the others. They all use `MemoryEventStore`, the in-memory store
that ships with happenstance, so there is no database to set up. To run them,
create a binary crate and add the two dependencies they use:

```text
cargo add happenstance
cargo add tokio --features macros,rt-multi-thread
```

## Append and read back

Every event has a **type**, a name for what happened (`SeatHeld`), and **tags**,
key-value labels that say what the event is about (`course=c1`). Tags are how
you find events again later, much as you would filter rows by a column.

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

The program appends two `SeatHeld` events for course `c1`. The `None` passed to
`append` means "no condition": nothing is guarded yet. The `{}` is the event's
payload, its data, left empty here because counting seats needs none.

It then builds a **query**, "events of type `SeatHeld` tagged `course=c1`", and
`read_decision_model` runs it. You get back two things: the matching events,
and the **position** of the last matching event you read. Every event the
store accepts is given a position, a sequence number that only ever goes up.

Why read the events back rather than keep a counter? Because the events are the
only record. Anything a decision needs to know, it works out by reading the
events that matter to it, and the query is how it says which events those are.
In step 3, the same query tells the store which new events should make it
refuse your write.

The position is the other half. Every reader sees the events in the order the
store assigned them, so "up to position 2" means the same thing to every
reader of the store ([ES-8](../spec/SPECIFICATION.md#es-8--ordering)).

## A condition that holds

> **From step 1:** a query for `SeatHeld` events tagged `course=c1`, and the
> position of the last matching event you read.
>
> Arrived here cold? [Start at step one](#append-and-read-back).

Now the write carries a condition. An `AppendCondition` is built from the two
things a read gives you: the query you read with, and the position you read up
to. Together they tell the store: "accept this append only if no event matching
this query has been added after this position".

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
    let condition =
        AppendCondition::new(seats).after_every_guard_opt(upto);
    let at = store.append(&[seat], Some(&condition)).await?;

    println!("{} seen, up to {upto:?}, at {at:?}", seen.len());
    Ok(())
}
```

```text
1 seen, up to Some(SequencePosition(1)), at SequencePosition(2)
```

One seat is already held when the program reads, so the read sees it and stops
at position 1. Nobody writes between the read and the append, so the condition
holds and the store accepts the new event at position 2.

`after_every_guard_opt` takes the position as an `Option` because a read whose query
matched nothing returns `None`. With no position to be "after", every matching
event counts,
so the condition would refuse if anything matching the query existed at all.

The seat already at position 1 does not count against you, even though it
matches the query. Your decision has already taken it into account, so only
events *after* the position you read up to can refuse the append
([ES-26](../spec/SPECIFICATION.md#es-26--the-ac3-boundary-after-is-exclusive-from-is-inclusive)).

## A condition that refuses

> **From step 2:** an `AppendCondition` built from the query you read with and
> the position you read up to.
>
> Arrived here cold? [Start at step one](#append-and-read-back).

This is the lost update from the top of the page, and the store's answer to it.
One seat is already held. Your program reads it and builds its condition. Then,
before your append, another writer holds a seat of its own. In a real
application that would be a second request arriving at the same moment; here it
is one more line, so the timing is the same on every run.

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
    let condition =
        AppendCondition::new(seats).after_every_guard_opt(upto);
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

Your read stopped at position 1. The other writer's seat landed at position 2,
after it, and matches your query, so the store refuses your append with
`AppendError::ConditionViolated`. Your program wrote nothing based on a count
that was out of date.

In an application you would now read again, see two seats taken, and decide
again. You do not have to write that loop yourself: the typed layer's `commit`
function reads, decides, appends and retries on a refusal for you.

The message is printed only inside the arm that matches the refusal, so seeing
it proves the store really refused. The store is required to refuse here, and
to report it under exactly that name
([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics)).

### Try it wrong, then put it back

Every program on this page is compiled and run by this repository's tests, so
you can break one and watch the tests catch it. You need a clone of the
repository for this, and about two minutes.

**The edit.** Open `docs/first-encounter.md`, the file this page is built from.
In the step 3 program, change `Some(&condition)` to `None`, which removes the
condition from the append. The program still compiles and still runs. Then run:

```text
cargo test -p xtask --doc -- first_encounter
```

**What you should see.** The test fails. This is the line to look for, the
program reporting that the append was accepted, at position 3, when it should
have been refused:

```text
the boundary did not hold: Ok(SequencePosition(3))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The failing test is named `narrative::first_encounter (line 170)`, after this
page and the line its program starts on. The file path printed with the panic
is a temporary file the test runner made, so go by the test's name instead.

**Putting it back.** Change `None` back to `Some(&condition)`. Run the same
command again and it passes, with nothing else to repair.

Without the condition, the store accepted a second seat that no decision had
counted, and nothing anywhere reported an error. That is the lost update from
the top of the page, and the condition is the one expression that prevents it.

If "dynamic" sounds like "anything goes", the fixed structure every decision
follows is explained under
[tag, query, fold, guard, on where the structure is](carry-your-invariant.md#tag-query-fold-guard).
If you are wondering whether this is a way of modelling your domain or of
routing writes, read
[where your streams went, on modelling versus routing](carry-your-invariant.md#where-your-streams-went).
