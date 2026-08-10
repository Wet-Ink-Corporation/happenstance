# Open-questions layer — what is not settled yet

The questions this repo has deliberately left open: `open_question` atoms carrying
`authority_tier: note`. They bind nothing, and that is the point — an open question records the
absence of a decision so that the absence is legible instead of being rediscovered.

## What belongs here

Anything an ingest or a closeout could not honestly resolve:

- an **unresolved conflict** between two sources, or between a source and the shipped code —
  `/redkiln:kb-ingest` routes these here as `defer_open_question` rather than picking a winner;
- a **known gap** the work uncovered and did not close, with what is actually true today;
- a **deferral**, where a decision was possible but was consciously postponed, and what would
  force the choice.

Write it so the next reader can act on it. An open question atom reads roughly like:

> **What is true today:** _\<the verified current state, and where you verified it\>_.
> **What is not decided:** _\<the question, stated so an answer would be recognisable\>_.
> **What forces it:** _\<the event or piece of work that would make this urgent\>_.
> **Ordered sub-questions:** _\<the order they probably have to be answered in\>_.

Ground every claim the way a `reference` atom would — cite the file, the config key, the command
you ran — and put those paths in `source_paths`. A question grounded in nothing is indistinguishable
from a worry, and the next reader has to re-derive the state before they can even start.

Add a bullet for the atom on the `map` atom that indexes its area. A question nobody can find from
the map is a question that gets asked again from scratch.

## What does not belong here

A resolved decision wearing a question mark. If the answer is settled, it is a `decision` atom (or a
`concept` atom, if it explains rather than commits) — filing it here strips it of its authority and
the next initiative will re-litigate it.

Nor does a task belong here. Work that someone is expected to do is a backlog item in `.bklg/`, not
a KB atom; the KB records what is _known_ and what is _not known_, never what is queued.

## Resolving one

When the question is answered, the answer is a **new atom** — and the record of the question stays.
Link the two (`related`), move this atom's `status` to `withdrawn` or `superseded`, and leave the
body describing what was not known at the time. Do not rewrite a question into its own answer: the
value of the record is that it shows the state of knowledge on the day the choice was made.

`KbFrontmatter` is a `.passthrough()` object, so an imported corpus's own tracking keys on an open
question (`tracks`, `resolution_ref`) survive validation untouched. Redkiln neither owns nor
validates them — do not strip them to make an atom "conform", and do not invent them on an atom you
are authoring.

## Why this layer exists

Every wave produces things it could not settle. Without a home for them, the honest outcome — "we
looked, and this is genuinely open" — has nowhere to land, so it gets normalised into a confident
claim or dropped entirely. Both cost the next wave the same way: it re-runs the research and reaches
the same undecided place, with no record that anyone had been there before.
