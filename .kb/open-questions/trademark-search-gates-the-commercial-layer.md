---
id: kb-open-question-trademark-search-001
title: The trademark search on "happenstance" has not been run, and it gates the commercial layer
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Two of the four staged brand documents close on the same unresolved gate, in almost the
  same words, and this atom is both of them. What is true today is that no trademark search
  on "happenstance" has been run. What makes it load-bearing rather than administrative is
  that the anti-appropriation lever the commercial seam rests on is trademark and not
  copyright, because Apache-2.0 section 6 grants no trademark rights — so a permissive
  licence protects the code and protects the name not at all. What is not decided is whether
  the name is clear, and nothing downstream of that can be: no part of the identity should be
  filed, registered or applied to physical goods until the search returns, and the commercial
  layer waits on the same answer. The identity itself is unaffected and continues to ship —
  the wordmark, the mark and the palette are decided (SD-0001, SD-0002) and are used in the
  repository today; what is gated is registration, filing and physical application, which is
  a narrower set than use. Forced by the first of those three, whichever comes first.
depends_on: []
related:
  - kb-decision-sd-0002
  - kb-reference-brand-source-locations-001
source_paths:
  - .kb/_intake/brand-identity-commitments.md
  - .kb/_intake/brand-where-the-identity-lives.md
  - references/seeds/licensing-and-the-commercial-seam.md
  - references/brand/SD-0002-the-mark.md
last_reviewed: 2026-09-02
---

# The trademark search on "happenstance" has not been run, and it gates the commercial layer

## What is true today

No trademark search on the name "happenstance" has been run. Two independently
staged brand documents — the identity's rules and commitments, and the pointer
document naming where the identity lives — each close on this same fact, in almost
the same words, and each names it as the thing gating a category of future action.
This atom is the single home for that gate rather than a duplicate paragraph carried
in two atoms; a reader who needs the identity's rules or its document map goes to
those atoms, and a reader who needs to know what is *not yet settled* about the name
comes here.

## Why it is load-bearing rather than administrative

The commercial seam this project is expected to eventually stand on rests on an
anti-appropriation lever, and that lever is trademark, not copyright. Apache-2.0
§6 is explicit that the licence grants no trademark rights — it disclaims them by
name. A permissive source licence protects the *code*: anyone may fork it, modify
it, and redistribute it under the licence's terms. It does nothing at all to
prevent someone else from calling a competing product "happenstance," registering
that name, or shipping goods under it, because none of that is a copyright question.
If the name itself is not defensible, the licence choice buys nothing on that axis.
That is what turns "the search hasn't run yet" from a routine to-do into a genuine
open question: the project does not yet know whether its name is the kind of asset
copyright already protects, or the kind that needs a second, unstarted process to
protect at all.

## What is not decided

Whether "happenstance" clears a trademark search — for the software category it
ships in today, and separately for any physical-goods category a future commercial
layer might reach. Nothing about the identity's own content is in question here:
the wordmark, the mark and the palette are decided and documented (the mark's
geometry and rules in `references/brand/SD-0002-the-mark.md`, the name's meaning
in the sibling `SD-0001` record) and are already in use in this repository — the
README, the crate metadata, and the artwork under `assets/brand/` all carry the
identity as shipped. What is undecided is narrower and sits one layer downstream of
that: whether the name may be *filed*, *registered*, or *applied to physical
goods* — merchandise, stickers as a sold product rather than a giveaway, a
registered wordmark — none of which the project has done and none of which should
happen before the search returns an answer.

## What forces it

The first of three triggers, whichever arrives first: a decision to file a
trademark application, a decision to register the mark in any jurisdiction, or a
decision to apply the identity to a physical good sold or distributed outside the
repository's own promotional use. Until one of those is imminent, the gate costs
nothing — the identity ships freely in software contexts today, which is a
narrower and already-safe use than any of the three triggers name.

## Where the fuller reasoning lives

`references/seeds/licensing-and-the-commercial-seam.md` records the anti-
appropriation argument this atom summarises; it is cited by path here rather than
reproduced, and is not present in every checkout of this tree — the intake wave
that staged the two source brand documents references it as the seed material
behind their shared closing paragraph. `references/brand/SD-0002-the-mark.md`
carries the mark's full decision record and is the document this question sits
downstream of, not in tension with.
