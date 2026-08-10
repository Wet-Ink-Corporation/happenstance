# Product layer — personas & journeys

The durable **who this is for** and **to what end**. Authored once, referenced by many
initiatives; never re-derived per story.

| Thing       | `kind`     | `authority_tier` |
| ----------- | ---------- | ---------------- |
| **Persona** | `concept`  | `product`        |
| **Journey** | `playbook` | `product`        |

A **persona** is a real, evidenced audience — their goal, their context, what they already do
instead, and what they are afraid of. A **journey** is the moment-by-moment path a persona
takes through a task, written so you can tell whether a build improved it.

An initiative's charter cites these atoms in its `## Referenced personas & journeys` section
and frames its acceptance criteria from them (`GIVEN a <persona> <context>, WHEN they <action>,
THEN <observable, humane outcome>`). Where an initiative's discovery produced personas that are
not yet promoted here, it cites its own
`.bklg/<slug>/_discovery/distillation/personas-and-journeys.md` and flags them for promotion —
and `/redkiln:closeout` performs that promotion, so the next initiative inherits them instead
of inventing a fresh set from the same evidence.

Ground every claim in real discovery evidence and cite it in `source_paths`. A persona nobody
researched is a stock photo with a name, and acceptance criteria framed from it are fiction.

## What does not belong here

An **unevidenced sketch**. A persona assembled from assumptions during one initiative's discovery
stays in that initiative's `_discovery/distillation/personas-and-journeys.md` until closeout
promotes it. Promoting it early gives a guess the standing of a finding, and every later
initiative inherits the guess without the caveat.

A **screen, a flow, or an interaction pattern**. Those are the `design/` layer: a journey says
what the persona is trying to accomplish and in what order, never which control they click. A
journey that transcribes today's UI goes stale the moment the UI moves, and takes the persona's
credibility with it.

**Requirements, scope or acceptance criteria.** What someone is expected to build is a backlog
item in `.bklg/`. This layer records who the work is _for_; an initiative's charter cites these
atoms and derives its criteria from them, which only works while the two stay separable.

A **market segment**. Sizing, pricing and positioning describe a market, not a person with a goal
and a fear, and nothing downstream can frame an acceptance criterion from them.
