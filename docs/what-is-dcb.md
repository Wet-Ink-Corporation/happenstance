# Dynamic consistency boundaries

> **Answers:** `explanation` — How does DCB keep a decision safe without aggregates?

This is the last of four background pages. The third,
[aggregates, and the boundary drawn in advance](aggregates.md), ended with a
rule about a course *and* a student that no single aggregate could hold. This
page is the idea that holds it, and the one happenstance is built on.

## One log, and labels instead of streams

A **dynamic consistency boundary**, or **DCB**, starts by dropping the streams.
Every event goes into one ordered log, and each event carries **tags**: labels
that say what it is about. A subscription is about a course and a student, so
it carries both:

```text
StudentSubscribed  tags: course=c1, student=s7
```

Each event is also given a **position** in the log, a number that only goes up.
Nothing about an event says which stream or aggregate it belongs to, because
there are none.

## Each decision chooses its own boundary

To decide whether s7 may subscribe to c1, the code reads exactly the events
that decision depends on, using a **query**:

```text
StudentSubscribed or StudentUnsubscribed, tagged course=c1
StudentSubscribed or StudentUnsubscribed, tagged student=s7
```

The first line finds everyone holding a seat on c1. The second finds every
course s7 holds. The read returns those events, and the position of the last one.

The decision counts both, and if both limits allow it, appends the new
`StudentSubscribed` with a **condition** attached: "accept this only if no
event matching that query has been added after that position". The store
checks the condition and the append together, as one step.

If another request subscribed someone to c1, or subscribed s7 to another
course, in the meantime, its event matches the query and sits after the
position, so this append is refused. The code reads again and decides again.
If nothing relevant happened, the append goes in.

The query is the consistency boundary. It is *dynamic* because each decision
draws its own, at the moment it runs, rather than inheriting one that was fixed
when the streams were designed.

## What that gives you

- **Rules that span several things are simply enforced.** Thirty seats and five
  courses are checked in one read and guarded by one write. There is no read
  model to trust and no saga to repair things afterwards.
- **No boundary to choose up front.** You do not have to guess, on day one,
  which decisions the system will need to make.
- **Changing a rule means changing a query.** A new rule that looks at
  different events reads them with a different query. The events already in the
  log do not move.
- **A request is only refused by a request it really conflicts with.** Two
  students subscribing to two different courses match neither query, so neither
  is ever refused because of the other.

What DCB asks in return is that you tag events with what they are about, and
write the query each decision needs. That is ordinary code, and the rest of this
guide shows it.

DCB is an open [specification](https://dcb.events/specification/), not a
product. It describes what an event store has to do, and any store that does it
can be used this way.

## Where happenstance fits

happenstance is a Rust library that implements that specification:

- **An event store contract**, the `EventStore` trait, that any storage can
  implement, with stores for SQLite, PostgreSQL, Neon serverless Postgres and
  Cloudflare Durable Objects, and an in-memory store for tests and learning.
- **A typed layer** on top: your events as Rust types, decision models that
  build the query for you, a command loop that reads, decides, appends and
  retries, and projections that keep read models up to date.
- **A conformance suite** that every store has to pass before this project
  calls it a store, so code written against one runs the same on all of them.

The guide's tutorial is next. It runs this exact read, condition and refusal in
three short programs: [your first encounter](first-encounter.md).
