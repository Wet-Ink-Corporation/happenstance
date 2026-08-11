# Concepts layer — durable ideas and the mechanisms behind them

A durable idea or term, explained: `concept` atoms carrying `authority_tier: note`. A concept
commits the project to nothing and prescribes no procedure — it exists so that a mechanism
found once, while writing a decision or a playbook, does not have to be re-derived by the next
reader who meets it from a different document.

## What belongs here

An explanation that has outgrown the one document that happened to state it — the shared
premise behind two or more decisions, or a term this repository uses with a precise meaning
that is easy to get wrong from the name alone. The test that earns an idea its own atom: is it
assumed, rather than stated, by more than one other atom? If folding it into any one of its
consumers would bury a premise the others also depend on, it belongs here instead.

Ground it the same way every other layer does. Cite the file, the type, the compiler behaviour
that makes the mechanism true, and put those paths in `source_paths`.

## What does not belong here

**A commitment.** *Must*, *shall*, *must not* — anything a future change would need a decision
to reverse — is a `decision` atom in [`../decisions/`](../decisions). A concept explains; it
does not bind.

**A procedure.** Steps with an order, or a technique arrived at by discarding alternatives, is
a `playbook` atom in [`../playbooks/`](../playbooks). A concept says how something *works*; a
playbook says what to *do*.

**A one-off measurement.** A number taken at a point in time is a `reference` atom in
[`../reference/`](../reference). A concept is timeless in the way a mechanism is timeless; a
reference is dated in the way a measurement is dated.

## Why this layer exists

A mechanism explained inside a decision atom is invisible to the next decision that rests on
the same mechanism — the reader either re-derives it or never notices the dependency at all.
Pulling the shared premise out into its own atom is what lets each consumer cite it once
instead of restating it, and what lets the premise be corrected in one place if it turns out to
be wrong.
