# Carry your invariant across

> **Answers:** `explanation` — How do I say my own rule in this library's terms?

## Your rule, in your words

You already have the rule. Say it the way you would say it to a registrar:

> Course c1 has two seats, and a student the registrar has barred may not
> take one of them.

That rule is about two things at once — a course and a student — and the
command that has to hold it is a single command: *hold a seat on c1 for s1*.
Nothing about it is unusual. It is the ordinary shape of a rule that will not
sit still inside one entity.

## Where your streams went

Your reflex question is probably *which stream does this go in?* — the model
you arrived with puts one stream per entity, so a rule spanning two of them
has to pick a side or be split across both. There is no side to pick here:
nothing asks you to name an aggregate, and your aggregates are not the unit
anything is checked against.

What you are asked for instead is the set of events your decision depends on,
and that set is what gets guarded. The boundary is drawn per decision, by the
query you are about to write, so the stream the rule "belongs to" never has
to exist.

## Tag, query, fold, guard

Four steps, and these same four names every time they come up below.

1. **Tag.** Name each thing the rule is about as a set of tags — one set per
   entity. `course=c1` is one of them; `student=s1` is the other.
2. **Query.** Build one `QueryItem` per tag set, saying which event types
   count for that entity, and put both items in one `Query`. Items are OR'd
   across a query, so it returns everything either half of the rule depends
   on
   ([ES-27](../spec/SPECIFICATION.md#es-27--a-condition-matches-on-tags-not-only-on-types)).
3. **Fold.** Read that query and walk what comes back into whatever the
   decision needs — a count, a total, a flag.
4. **Guard.** Build an `AppendCondition` from that same `Query`, bounded at
   the position the read reached, and append under it.

| Your words | This library's words | Where you saw it |
| --- | --- | --- |
| "course c1", "student s1" | **tag** — one `Tags` set per entity | first encounter, step 1 |
| "the rule spans both" | **query** — one `QueryItem` per tag set | first encounter, step 1 |
| "what I decide from them" | **fold** — over what the read returned | first encounter, step 2 |
| "only if nothing has moved" | **guard** — an `AppendCondition` over that same query | first encounter, step 3 |

## The guard you would write

The whole cycle is one program. Two tag sets, one item per tag set, one query
holding both, a read, a fold over what the read returned, and an append under
a condition built from that same query. Between the read and the append
someone else takes the last seat, so the append is refused.

```rust
use happenstance::{AppendCondition, Event, EventStore, Query};
use happenstance::{MemoryEventStore, QueryItem, Tags};
use happenstance::{AppendError, read_decision_model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let store = MemoryEventStore::new();
    let held = Tags::from_pairs([("course", "c1")])?;
    let whose = Tags::from_pairs([("student", "s1")])?;
    let seats = QueryItem::new(["SeatHeld"], held.clone())?;
    let barred = QueryItem::new(["StudentBarred"], whose)?;
    let rule = Query::from_items([seats, barred])?;
    let seat = Event::new("SeatHeld", &b"{}"[..])?.with_tags(held);
    store.append(&[seat.clone()], None).await?; // already held
    let (seen, upto) = read_decision_model(&store, &rule).await?;
    let free = seen.iter().fold(2, |left, _| left - 1);
    store.append(&[seat.clone()], None).await?; // another writer
    let condition = AppendCondition::new(rule).after_opt(upto);
    let refused = store.append(&[seat], Some(&condition)).await;
    assert!(matches!(refused,
        Err(AppendError::ConditionViolated(_))), "{free} free");
    Ok(())
}
```

The condition is the query you already wrote, bounded at the position your
read reached, and a matching event above that boundary is what refuses it
([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics)).

One `Query` goes into `AppendCondition::new` today; carrying several
independently bounded guards in one condition is described but is not settled
([VT-30](../spec/SPECIFICATION.md#vt-30--an-appendcondition-is-one-or-more-guards-each-with-its-own-boundary)),
so nothing above leans on it.

## What a narrow guard misses

Now change one expression. The guard below is tagged to the student the
command is writing for, rather than to the course whose seats the rule is
about. Everything else — the events appended, the condition, the guarded
append — is the same text as above.

```rust
use happenstance::{AppendCondition, Event, EventStore, Query};
use happenstance::{MemoryEventStore, QueryItem, Tags};
use happenstance::read_decision_model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let store = MemoryEventStore::new();
    let held = Tags::from_pairs([("course", "c1")])?;
    let whose = Tags::from_pairs([("student", "s1")])?;
    // WRONG - tagged to the student, not to the course's seats.
    let seats = QueryItem::new(["SeatHeld"], whose.clone())?;
    let barred = QueryItem::new(["StudentBarred"], whose)?;
    let rule = Query::from_items([seats, barred])?;
    let seat = Event::new("SeatHeld", &b"{}"[..])?.with_tags(held);
    store.append(&[seat.clone()], None).await?; // already held
    let (seen, upto) = read_decision_model(&store, &rule).await?;
    let free = seen.iter().fold(2, |left, _| left - 1);
    store.append(&[seat.clone()], None).await?; // another writer
    let condition = AppendCondition::new(rule).after_opt(upto);
    let accepted = store.append(&[seat], Some(&condition)).await;
    assert!(accepted.is_ok(), "{free} free, and it still went in");
    // END WRONG. The correct guard is the one above.
    Ok(())
}
```

The seat that fills the course carries `course=c1` and no student tag, so it
never enters this guard's query. The read comes back empty, the boundary
guards nothing, the append is accepted, and the course is over capacity with
no error anywhere.

That acceptance is not a defect being exposed. A condition carrying a tag no
stored event carries does not reject the append, even when a stored event
matches the condition's types
([CF-8](../spec/SPECIFICATION.md#62-the-measured-gaps)).

The mirror mistake — dropping the tags and guarding on types alone — fails
the other way, loudly, by refusing commands it should admit
([CF-7](../spec/SPECIFICATION.md#62-the-measured-gaps)); this fence is
deliberately not that one.

Both follow from the same rule, that a condition is matched on tags and not
only on types
([ES-27](../spec/SPECIFICATION.md#es-27--a-condition-matches-on-tags-not-only-on-types)).

## Back to the working version

The wrong guard above compiles, runs, and passes its own assertion. Nothing
catches it for you, which is why it is worth having read. The difference is
one expression: the query item is tagged `held`, the course whose seats the
rule is about, and not `whose`.
[The guard you would write](#the-guard-you-would-write) is the version to
copy.

To see the same cycle inside a running program rather than a fence,
[read the worked example](read-the-worked-example.md).
