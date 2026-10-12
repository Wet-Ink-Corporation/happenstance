# Event sourcing, and CQRS

> **Answers:** `explanation` — What changes when events become the source of truth?

This is the second of four background pages. The first,
[events, and the systems built on them](event-driven-systems.md), ended with
events used as notifications while a table stayed the truth. This page makes the
events the truth.

## Event sourcing: the events are the record

In an **event-sourced** system, the events are not copies of a change made
somewhere else. They *are* the change. The application never updates a row to
say a seat was taken; it appends a `StudentSubscribed` event to an **event
store**, a database whose main job is to keep events in order, forever.

Current state is no longer stored. It is worked out when it is needed, by
reading the events in order and applying each one to a running total. To know
how many seats course `c1` has taken, you read its subscription events and
count: plus one for each `StudentSubscribed`, minus one for each
`StudentUnsubscribed`. Working out state from events like this is often called
**folding** or **replaying** them.

That sounds like more work than reading a row, and it is. What it buys:

- **The whole history, for free.** Every change is kept, with its order, so
  "who took the last seat, and when?" is always answerable.
- **New questions about old data.** If next year you want to know how often
  students leave a course in its first week, the events to answer it have been
  there all along. With a table of current state, that information was
  overwritten.
- **Bugs you can replay.** A wrong result can be reproduced by reading the same
  events again, in the same order.
- **One record, nothing to keep in sync.** The gap from the previous page
  closes: the event *is* the write, so there is no table that can change
  without its event.

## CQRS: one side decides, the other side answers

Reading every event to answer every question would be slow. A course list page
that shows seat counts for a thousand courses should not replay a thousand
histories on every visit.

So event-sourced systems usually split into two sides. This split is called
**CQRS**, for *command query responsibility segregation*:

- **The write side** handles *commands*: requests to do something, such as
  "subscribe s7 to c1". It reads the events the decision needs, checks the
  rules, and appends new events. It is optimised for being correct.
- **The read side** answers *queries*: requests for information, such as "list
  every course with its free seats". It keeps **read models** (also called
  projections or views): ordinary tables, built by reading the events as they
  arrive and kept in whatever shape the screen wants. It is optimised for being
  fast.

Because read models are built from the events, they can be thrown away and
rebuilt at any time, or a new one built beside the old one, as
[rebuild a read model](rebuild-a-read-model.md) shows. The events are the only
thing that has to be kept safe.

A read model usually runs a moment behind the events. That is fine for a page
that lists courses. It is not fine for the rule "a course holds thirty seats",
which is checked on the write side, against the events themselves.

## The question this leaves

The write side reads events, decides, and appends. Two requests can do that at
the same time. If both read twenty-nine seats taken, both decide there is room,
and both append, the course ends up with thirty-one.

Every event-sourced system needs an answer to that. The most common answer
draws the line in advance, around things called aggregates:
[aggregates, and the boundary drawn in advance](aggregates.md).
