# Read the worked example

> **Answers:** `orientation` — Where can I read a whole DCB program?

The canonical example is three rules that no single entity can hold on its
own, and the three decisions taken on them against a running store. It is the
one place in this repository where the read-decide-append cycle works over
domain code rather than inside a fence.

Its own explanation names the model you arrived with, in another voice and
correctly for its purpose; the single place that model is answered is
[where your streams went](carry-your-invariant.md#where-your-streams-went).

You have just written a `Query` and an `AppendCondition` by hand, and the
example holds neither: `happenstance::commit` derives both from a
`DecisionModel`, so what you are about to read is the same boundary one layer
up rather than a different library.

[The example's own explanation](../examples/course-subscriptions/src/overview.md)
is where to begin.

[The example's source](../examples/course-subscriptions/src/main.rs) is the
program that explanation is about.
