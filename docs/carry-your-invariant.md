# Carry your invariant across

> **Answers:** `explanation` — How do I write my own rule in this library's terms?

The first encounter guarded a rule about one course. Most real rules are about
more than one thing at once. This page takes a rule like that and turns it into
code in four steps, then shows the one mistake the compiler cannot catch for you.

## Your rule, in your words

Start with a rule from your own domain, said the way you would say it out loud.
Here is the one this page uses:

> Course c1 has two seats, and a student the registrar has barred may not
> take one of them.

That rule is about two things: a course and a student. Yet a single request has
to respect all of it: *hold a seat on c1 for s1*. There is nothing unusual about
this. Plenty of rules refuse to fit inside one entity.

## Where your streams went

If you have built event-sourced systems before, your first question is probably
*which stream does this go in?* With [one stream per aggregate](aggregates.md),
a rule about a course and a student has to live in one of the two streams, or
be split across both of them.

Here there is no stream to choose. Nothing asks you to name an aggregate, and
nothing is checked against one. Every event goes into one ordered log, tagged
with what it is about, as [dynamic consistency boundaries](what-is-dcb.md)
describes.

What you give the library instead is the set of events your decision depends
on, and that set is what the store guards. The query you write in the next
section draws the boundary for this one decision, so the stream the rule
"belongs to" never has to exist.

So this is a change in how you model a rule, not just a new way of routing
writes to storage. You stop asking where a rule lives, and start asking which
events it reads.

## Tag, query, fold, guard

Each decision draws its own boundary, but that does not mean anything goes.
Every decision follows the same four steps, in the same order, and this page
uses these four names for them throughout.

1. **Tag.** Describe each thing the rule is about as a set of tags, one set per
   thing. Here, `course=c1` is one set and `student=s1` is the other.
2. **Query.** Build one `QueryItem` per tag set, naming the event types that
   matter for that thing, and put both items in one `Query`. An event matches
   the query if it matches either item, so one read returns everything both
   halves of the rule depend on
   ([ES-27](../spec/SPECIFICATION.md#es-27--a-condition-matches-on-tags-not-only-on-types)).
3. **Fold.** Read with that query, and work the events you get back into
   whatever the decision needs: a count, a total, or a yes-or-no flag.
4. **Guard.** Build an `AppendCondition` from that same `Query` and the
   position the read reached, and append under it. If anything matching the
   query arrives in the meantime, the append is refused.

Each step maps onto something you already said in plain words, and onto
something you already ran in the first encounter:

| Your words | This library's words | Where you saw it |
| --- | --- | --- |
| "course c1", "student s1" | **tag** — one `Tags` set per entity | first encounter, step 1 |
| "the rule spans both" | **query** — one `QueryItem` per tag set | first encounter, step 1 |
| "what I decide from them" | **fold** — over what the read returned | first encounter, step 2 |
| "only if nothing has moved" | **guard** — an `AppendCondition` over that same query | first encounter, step 3 |

## Where your identifiers come from

In this page's examples, `c1` and `s1` are written into the code. In your
application they arrive as strings, from a request, a queue or a file, and a
tag built from one of them can be refused. `Tag::key_value` rejects an empty
value, a value that is too long, and certain control and text-direction
characters. That is why the examples use `?` when they build tags.

That matters most once your events become Rust types. The typed layer asks each
event for its tags through a `DomainEvent` method, `fn tags(&self) -> Tags`.
It returns `Tags`, not a `Result`, so there is nowhere for an error to go.
Building a tag from a raw string inside that method would leave only one way
out: calling `expect` on data from outside your program, on the write path,
after the decision has already been made.

The fix is to check the identifier once, at the edge of your program, where a
`Result` is still welcome. Wrap it in its own type (a *newtype*), whose only
constructor builds the `Tag` and keeps it. From then on, handing out that tag
cannot fail.

The struct keeps both the original string and the tag built from it. Because
`new` is the only way to make a `CourseId`, every `CourseId` in your program
holds a tag that was already checked:

```rust
use happenstance::{InvalidTag, Tag, Tags};

#[derive(Debug, Clone)]
struct CourseId {
    id: String,
    tag: Tag,
}

impl CourseId {
    // The only constructor, and the only place a bad identifier is refused.
    fn new(id: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            id: id.to_owned(),
            tag: Tag::key_value("course", id)?,
        })
    }
}

fn main() -> Result<(), InvalidTag> {
    let course = CourseId::new("c1")?; // the edge, where a `?` is at home
    let refused = CourseId::new("");
    assert!(refused.is_err(), "an empty value is not a tag");

    // And this is the call that cannot fail. `Tags` has no infallible
    // constructor from strings, but it does have one from tags that already
    // exist, so a `tags` method built this way needs no `Result` and no
    // `expect`.
    let tags: Tags = [course.tag.clone()].into_iter().collect();
    assert_eq!(tags.len(), 1);
    Ok(())
}
```

Both worked examples do exactly this, and say so in comments: `CourseId` and
`StudentId` in [the canonical example](../examples/course-subscriptions/src/main.rs),
and `AccountId` in [the SQLite one](../examples/transfers-on-sqlite/src/main.rs).
Each keeps the same two fields, the string and its tag.
The typed layer makes the same choice for a decision model's scope: `scope`
returns tags the model already holds, so it cannot fail either.

## The guard you would write

Here are all four steps in one program. It builds two tag sets, one query item
for each, and one query holding both. It reads, folds the result into a count
of free seats, and appends under a condition built from the same query.

To show the guard working, someone else takes the last seat between the read
and the append. The store notices, and refuses the append.

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

The condition is just the query you already wrote, plus the position your read
reached. A matching event after that position is what makes the store refuse
([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics)).

Today, `AppendCondition::new` takes one `Query`. The specification describes a
condition holding several guards, each with its own position, but that is not
settled yet
([VT-30](../spec/SPECIFICATION.md#vt-30--an-appendcondition-is-one-or-more-guards-each-with-its-own-boundary)),
so nothing on this page depends on it.

## What a narrow guard misses

The query is the boundary, so a query that is too narrow guards too little, and
nothing fails to tell you. This section shows that happening, so you can
recognise it in your own code.

The program below changes one expression. The seats item is tagged with the
student the command is for, instead of the course whose seats the rule is
about. Everything else is the same as above: the events appended, the
condition, and the guarded append.

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

The seat that fills the course is tagged `course=c1` and carries no student
tag, so it never matches this query. The read comes back empty, the condition
guards nothing, and the store accepts the append. The course is now over
capacity, and no error was reported anywhere.

The store is behaving correctly here. A condition whose tags no stored event
carries does not refuse the append, even when a stored event has a matching
type ([CF-8](../spec/SPECIFICATION.md#62-the-measured-gaps)).

The opposite mistake is to drop the tags and guard on event types alone. That
fails the other way, and loudly: it refuses requests it should accept, because
any seat on any course now counts
([CF-7](../spec/SPECIFICATION.md#62-the-measured-gaps)). The program above is
deliberately not that one.

Both behaviours follow from one rule: a condition matches on tags as well as on
event types
([ES-27](../spec/SPECIFICATION.md#es-27--a-condition-matches-on-tags-not-only-on-types)).

## Back to the working version

The wrong guard compiles, runs, and passes its own assertion. Nothing catches
it for you, which is why it was worth seeing. The fix is one expression: tag
the seats item with `held`, the course whose seats the rule is about, not with
`whose`. With that tag, the seat that fills the course is inside the boundary:

```rust
use happenstance::{EventType, QueryItem, Tags};

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let held = Tags::from_pairs([("course", "c1")])?;
    let seats = QueryItem::new(["SeatHeld"], held.clone())?;
    let filling = EventType::from_static("SeatHeld");
    assert!(seats.matches(&filling, &held), "the seat is guarded");
    Ok(())
}
```

[The guard you would write](#the-guard-you-would-write) is the whole program to
copy.

To see the same steps inside a complete program rather than a single example,
[read the worked example](read-the-worked-example.md).
