# Events, and the systems built on them

> **Answers:** `explanation` — What is an event, and why build a system around them?

This is the first of four short background pages. They start from ideas you
probably already use and build, one step at a time, to the idea happenstance is
built on. If you already know event sourcing well, skip to
[dynamic consistency boundaries](what-is-dcb.md).

## State, and what happened

Most applications store *state*: the way things are right now. A course has a
row in a table, and that row says `seats_taken = 29`. When a student subscribes,
the row is updated to 30.

That row answers "how many seats are taken?" very well. It cannot answer
anything else. Who took them? In what order? Was a seat taken and then given
back? Did the count ever go over the limit for a moment? The update replaced
the old value, and the story of how it got there is gone.

An **event** records the story instead. It is a small, permanent fact about
something that happened, written in the past tense:

```text
StudentSubscribed   { course: "c1", student: "s7" }
StudentUnsubscribed { course: "c1", student: "s3" }
StudentSubscribed   { course: "c1", student: "s9" }
```

An event is never changed or deleted after it is written, because what happened
does not change. If a student leaves a course, that is a new event, not an edit
to the old one.

## Event-driven systems

Many systems already use events to let their parts talk to each other. When an
order is placed, the ordering service publishes `OrderPlaced`. The billing
service, the warehouse and the email service each listen for it and do their
own work. The ordering service does not need to know they exist.

This is an **event-driven system**, and it is popular for good reasons:

- **Parts are loosely coupled.** A new service can start listening to existing
  events without anyone changing the code that publishes them.
- **Work can happen later, and elsewhere.** A slow job, such as sending an
  email, does not hold up the request that caused it.
- **The flow is visible.** Reading the events tells you what the system did.

## What an event-driven system does not change

In most event-driven systems, events are *notifications*. The ordering service
still keeps its orders in an ordinary table, updated in place, and publishes an
event afterwards to tell everyone else.

So the table is still the truth, and the events are copies of what it did.
Two things can go wrong as a result. The table can be updated without the
event being published, so other services never hear about it. And the history
is still lost inside the service that owns the table: the events went out to
other people, not into its own records.

The next idea removes that gap by making the events themselves the record:
[event sourcing, and CQRS](event-sourcing-and-cqrs.md).
