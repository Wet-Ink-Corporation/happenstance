---
id: kb-decision-sd-0001
title: "\"Happenstance\" names the boundary drawn by what occurred, and what that places on copy"
kind: decision
status: accepted
authority_tier: decision
adr_id: SD-0001
reversibility: medium
phase: null
supersedes: null
superseded_by: null
summary: >-
  The name means the thing DCB does: it draws the consistency boundary around what actually
  occurred, rather than around a structure chosen before anyone knew what would occur. The word is
  happen plus circumstance, and the sense claimed is not the dominant dictionary one — chance — but
  the narrower one carried by the phrase as it happened: what turned out to be the case, as against
  what was arranged in advance. That is the axis DCB moves along, because a classical aggregate
  fixes its boundary when the schema is written while a DCB handler reads exactly the events its
  decision depends on and appends conditioned on nothing matching that query having appeared since.
  The boundary is deliberate but not pre-declared — designed on purpose, not inherited. Five rules
  follow and they bind copy rather than code: name the colloquial reading and displace it rather
  than talking past it; never assert that the name is clever; do not extend the metaphor anywhere,
  in copy, examples, error messages or artwork, with no chance, luck, dice, serendipity, fortune or
  coincidence imagery; keep the name lower-case in prose, recasting a sentence rather than
  capitalising it; and do not make the name the lead claim. What this does not claim is why the
  name was chosen: ADR-0005 renamed the project from eventum on 2026-08-05 because the bare eventum
  name was unavailable, and availability is the entire recorded justification. This assigns a
  meaning after the fact, states that it is doing so, and does not supersede ADR-0005, which stays
  correct about why the rename happened. The costs are permanent and accepted: the colloquial sense
  is wrong and is the default reading, the meaning is retroactive against a public record, and the
  word is twelve letters and three syllables with no natural short form — none of which is a reason
  to propose a rename of four crate names already shipped.
depends_on: []
related:
  - kb-decision-0005
  - kb-decision-0006
  - kb-concept-torn-read-append-boundary-001
source_paths:
  - .kb/_intake/brand-the-name-and-what-it-means.md
  - references/brand/SD-0001-what-happenstance-means.md
  - references/adr/0005-rename-to-happenstance.md
last_reviewed: 2026-09-02
---

# "Happenstance" names the boundary drawn by what occurred, and what that places on copy

## The claim

"Happenstance" names the thing DCB does: it draws the consistency boundary around what
*actually occurred*, rather than around a structure chosen before anyone knew what would
occur. This is a decision about what every sentence written about the project may take as
a shared premise — not a decision about code, and not a restatement of why the name was
chosen.

The word is "happen" plus "circumstance." Its dominant dictionary sense is chance —
Merriam-Webster gives "a circumstance especially that is due to chance." The sense claimed
here is the narrower one carried by the phrase *as it happened*: what turned out to be the
case, as against what was arranged in advance.

That is the axis DCB moves along. A classical aggregate fixes its consistency boundary
when the schema is written, before anyone knows which decisions will be made against it. A
DCB handler instead reads exactly the events its decision depends on and appends
conditioned on nothing matching that query having appeared since — the boundary is not
declared ahead of time and defended, it is whatever that query happened to match.

**The boundary is deliberate but not pre-declared.** The handler designs its query on
purpose; what it does not do is inherit a boundary someone else drew earlier. *Unplanned*
is the wrong word for this; *unarranged in advance* is the right one.

## Rules this places on copy

- **Name the colloquial reading and displace it, never talk past it.** Chance is the
  default sense and is exactly wrong for a durable event store. Copy that simply asserts
  the intended meaning reads as a stretch against the dictionary. Copy that says the
  everyday sense is chance, that this is the wrong idea here, and then supplies the right
  one is doing the work.
- **Never assert that the name is clever.** Introduce the sense and let the reader make
  the connection; a name that announces its own aptness breaks the project's register of
  showing evidence rather than editorialising.
- **Do not extend the metaphor.** No chance, luck, dice, serendipity, fortune or
  coincidence imagery anywhere — copy, examples, error messages, artwork — each instance
  reinforces the reading this decision exists to displace.
- **Lower-case in prose**, matching the crate name, including at a sentence's start, where
  the sentence should be recast rather than the name capitalised.
- **The name is not the lead claim.** It supports whichever lead claim positioning
  settles on; it does not substitute for one.

## What this does not claim

The name was not chosen for this reason. ADR-0005 renamed the project from `eventum` on
2026-08-05 because the bare `eventum` crate name was held by a crate dormant since 2020,
and the entire recorded justification for `happenstance` is availability. This decision
assigns a meaning after the fact, and says so rather than quietly retrofitting it. It
settles the meaning, not the name, and does not supersede ADR-0005 (`kb-decision-0005`),
which stays correct about why the rename happened — nor `kb-decision-0006`, which reversed
the *crate allocation* half of that same decision and is untouched by this one.

## Costs, acknowledged

The colloquial sense is wrong and is the default reading; the meaning is retroactive
against a public record, and a hostile reader can point that out. The word is twelve
letters and three syllables with no natural short form, and many non-native English
speakers will not know it — a permanent cost of a name already shipped under four crate
names on crates.io, and not a reason to propose a rename, which would cost far more than
it recovers.

## Usable forms

Six words: *the boundary is not declared in advance.* One sentence: *a DCB boundary is not
fixed when the schema is written — it is whatever the handler's query happened to match.*
Two sentences, for an evaluator asking why the name: *the everyday sense of happenstance
is chance, which is the wrong idea here. The one meant is as it happened: a classical
aggregate fixes the consistency boundary before anyone knows which decisions will be made,
and DCB lets it be whatever the handler's query turned out to match.*
