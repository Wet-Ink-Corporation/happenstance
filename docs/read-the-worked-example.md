# Read the worked example

> **Answers:** `orientation` — Where can I read a complete DCB program?

The worked example is a short program about students subscribing to courses,
with three rules that each depend on more than one thing. It is the best place
to see the ideas from the first encounter working together in real domain code.

The example's explanation describes the problem in the terms of classical
event sourcing, which suits its purpose. If those terms leave you wondering
where your own event streams would go, the answer is under
[where your streams went](carry-your-invariant.md#where-your-streams-went).

The code will look different from what you have written so far. You built a
`Query` and an `AppendCondition` by hand; the example writes neither. Instead,
`happenstance::commit` builds both from a `DecisionModel`, a type that says
which events a decision reads and what it works out from them, and then reads,
decides, appends and retries for you. It is the same boundary, one layer up.

Start with [the example's own explanation](../examples/course-subscriptions/src/overview.md),
which sets out the three rules and how each decision is taken.

Then read [the example's source](../examples/course-subscriptions/src/main.rs),
the program that explanation describes.
