The canonical DCB worked example: students subscribing to courses.

Three invariants, none of which fits inside a single aggregate:

1. a course may not be defined twice;
2. a course may not exceed its capacity;
3. a student may not subscribe to the same course twice.

Invariant 2 spans every subscription for a course. Invariant 3 spans one
student *and* one course. Classical event sourcing forces a choice here:
make `Course` the aggregate and invariant 3 needs a read model plus a saga,
or make the pair the aggregate and invariant 2 has nowhere to live.

DCB dissolves the problem. Each handler reads exactly the events its
decision depends on, notes where it read to, and appends conditioned on
nothing matching that same query having appeared since. The consistency
boundary is drawn per decision, and it is drawn by the query.

Run with `cargo run -p course-subscriptions`.

# What is not in this file, and used to be

No query literal, no append condition, no retry loop, no byte scraping. The
event set is declared **once**, as `Enrolment`'s `EVENT_TYPES`, and every
query is derived from it and the model's own scope; the whole
read-decide-append-retry cycle is `happenstance::commit`; payloads go
through the JSON codec. What is left is the domain and the three decisions
taken on it.
