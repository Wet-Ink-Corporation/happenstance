# Maps layer — navigation, not knowledge

The indices that let a reader find an atom without already knowing it exists:
`map` atoms carrying `authority_tier: note`. A map binds nothing and asserts
nothing new — every fact it states is stated more fully by the atom it points
to. Its only job is to make the corpus's shape legible from one file.

## What belongs here

- a **decision map** — one row per `decision` atom, its status, and what it
  supersedes or is superseded by;
- a **domain map** — the corpus grouped by subject area, so a reader arriving
  at one atom can find the others in its neighbourhood without a full-text
  search;
- an **open-questions index** — one bullet per `open_question` atom, so an
  unresolved question is discoverable from the area it concerns rather than
  only from `.kb/open-questions/`'s own file listing.

A `dependency-map` atom belongs here too, the day a major structural edge
between atoms needs a picture rather than a pair of `depends_on` lists — none
exists yet.

## What does not belong here

**A copy of the atom's content.** A map states an id, a title, one line of
orientation, and a link. If a reader needs more than that to decide whether to
open the atom, the line is doing too much work — trim it, don't expand it.

**A judgement.** A map does not rank, endorse, or resolve. An open question
listed here is exactly as open after the listing as before it.

## Maintenance

`/redkiln:kb-ingest`'s Maps phase updates these atoms every wave: a new or
superseded `decision` atom gets a row here; a new canonical concept or domain
area gets an entry on the domain map; a new `open_question` atom gets a
bullet on the open-questions index. Between waves, these atoms drift out of
date the way any index does — trust the atom they point to over the one-line
summary here.

## Why this layer exists

Every other layer answers "what do we know". This one answers "where is it" —
without it, the corpus is only as navigable as its directory listing, and a
reader has to already know an atom's slug to find it.
