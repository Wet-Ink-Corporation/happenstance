# Aggregates, and the boundary drawn in advance

> **Answers:** `explanation` — How do most event-sourced systems keep a decision safe?

This is the third of four background pages. The second,
[event sourcing, and CQRS](event-sourcing-and-cqrs.md), ended with two requests
deciding at the same time and overbooking a course. This page is the answer
most event-sourced systems give, and where it runs out.

## A consistency boundary

A **consistency boundary** is the set of data that has to stay unchanged while
a decision is being made, for that decision to still be right when it is
written. For "is there a seat left on c1?", the boundary is c1's seat count. If
that changes between the read and the write, the decision was made on stale
information and has to be refused.

## The aggregate: one boundary per thing

Most event-sourced systems organise their events into **streams**, one per
thing in the domain: one stream for course c1, another for course c2, one for
each student. The code that owns one stream, loads it, enforces its rules and
appends to it is called an **aggregate**.

Each stream has a version number, which goes up by one with every event. A
decision reads the stream, notes the version (say 41), and appends with the
instruction "only if the stream is still at version 41". If another request
appended first, the version is now 42, the append is refused, and the decision
is made again. This is called **optimistic concurrency**: nothing is locked
while you think, and a conflict is caught at the moment you write.

So the aggregate *is* the consistency boundary, and it is chosen in advance,
when the streams are designed.

## Where it works well

When a rule is about one thing, this is simple and fast. "A course holds thirty
seats" is entirely about one course. The `Course` aggregate holds the count,
and two requests for the same course are kept apart by the version check.
Requests for different courses never touch the same stream, so they never wait
for each other.

## Where it runs out

Now add a second rule: "a student takes at most five courses". Subscribing one
student to one course has to respect both limits, and the two limits are about
different things. Every option has a cost:

- **Make `Course` the aggregate.** Thirty seats is safe. Five courses per
  student now spans many course streams, so it cannot be checked in one append.
  It is checked against a read model, which can be a moment behind, and
  repaired afterwards by a background process (often called a **saga**) that
  notices when a student went over and undoes the extra subscription.
- **Make `Student` the aggregate.** The same problem, mirrored: five courses is
  safe, and thirty seats needs the read model and the repair.
- **Make one big aggregate hold both.** Both limits are safe, but every
  subscription anywhere now goes through one stream, and every request waits
  for every other.

None of these is a mistake. Each is the honest price of choosing the boundary
before knowing every decision it would have to hold. And the choice is hard to
undo: once years of events sit in course streams, regrouping them by student is
a migration, not a code change.

The last background page shows what happens if the boundary is not chosen in
advance at all, but by each decision:
[dynamic consistency boundaries](what-is-dcb.md).
