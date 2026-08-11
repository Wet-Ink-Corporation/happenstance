# Design layer — resolved interaction patterns

The interaction-design decisions this repo has already made and a human has already signed off:
`concept` atoms carrying `authority_tier: design`.

## What belongs here

The **decision and its reasoning**, harvested from a project's `_design.md` at closeout:

- the **pattern chosen** for a class of surface, and the class it applies to;
- the **alternatives rejected**, and why;
- the **documented failure mode** the choice mitigates, with its source;
- the **anti-patterns** that proved real in the build;
- the **fit conditions** — the shape of problem where this holds (fanout, depth, item count,
  label length, viewport budget) and where it stops holding.

Write it so the next initiative can reuse the judgement rather than re-deriving it. A design atom
reads roughly like:

> **Pattern:** _\<the pattern chosen\>_, for _\<the class of surface it applies to\>_.
> **Rejected:** _\<alternative\>_, because _\<the property that ruled it out here\>_.
> **Mitigates:** _\<the documented failure mode\>_ (cite the source).
> **Holds when:** _\<the fit conditions\>_. **Stops holding when:** _\<the boundary\>_.

The load-bearing parts are the _rejected_ alternative and the _boundary_. A note that records only
what was chosen tells the next reader nothing about when to choose differently.

## What does not belong here

The pixel layout of one screen. Component inventories and token values live with the code and
go stale the moment it changes; a design atom that transcribes last quarter's panel is worse
than nothing, because it is confidently wrong. Keep the atom at the altitude of the decision.

## Why this layer exists

These decisions were previously made inside an initiative folder and archived with it. The next
initiative started from zero, re-ran the same research, re-hit the same documented failure mode,
and shipped it again. Harvesting the resolution — not the artifact — is what stops the third
attempt from repeating the second.
