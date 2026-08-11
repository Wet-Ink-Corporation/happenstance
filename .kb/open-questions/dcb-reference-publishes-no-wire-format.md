---
id: kb-open-question-dcb-no-published-format-001
title: There is no DCB wire format to interoperate with
kind: open_question
status: accepted
authority_tier: note
summary: >-
  WF-1's interoperability half stays DEFERRED, and the reason changed when it was checked: the DCB reference implementation publishes no wire format at all. EventStore.ts contains no serialisation code, and the specification's JSON snippets are explicitly a 'potential' representation rather than a required one. So happenstance's format is not diverging from a standard; there is no standard to diverge from, which is what makes the format private and every reversal in it free. What is not decided is what happens if that changes. Refuted by a DCB implementation that publishes an encoding — at which point the question becomes whether interoperability is worth anything to this project, and the answer is not obviously yes. Owned by phase 13, the sync crate and its testkit, which is the first point at which anything of ours is on a wire another implementation could be at the far end of.
depends_on: []
related:
  - kb-decision-0016
  - kb-reference-wire-format-measurements-001
  - kb-open-question-sync-message-set-undesigned-001
source_paths:
  - .kb/_intake/0016-the-wire-format.md
  - docs/adr/0016-the-wire-format.md
  - experiments/wire-format/
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
---

# There is no DCB wire format to interoperate with

## What is true today

WF-1 asks two questions folded into one clause: what is happenstance's wire
format's *scope*, and does it need to *interoperate* with anything external.
ADR-0016 settles the scope half — the format is happenstance's own — and
confirms the interoperability half stays `[DEFERRED]`, but for a different
reason than the one the clause was written with.

The clause's original marker deferred to "a bridge exercised against the DCB
reference implementation's encoding." ADR-0016's own research question W7
went and looked: it fetched `EventStore.ts` from its canonical source and
read the DCB specification page directly. The finding is unambiguous — "The
reference library contains **no serialisation code of any kind**, and the
specification's JSON snippets are labelled *'a **potential** JSON
representation'* beside *'implementations are not required to use the same
terms or function/field names.'*" WF-1's own phrase, "the reference
implementation's published shape," turns out to have no referent at all. That
is a finding, the ADR is careful to note, "not a failure to measure."

The practical consequence is that happenstance's format is not diverging from
a standard — there is no standard to diverge from — which is what makes every
wire-format decision in ADR-0016 free of a compatibility cost. Several land
that way explicitly: WF-3 reverses a documented serialisation intent for
`Query`, WF-6 changes the `EventId` field encoding, WF-11 adds base64 for
human-readable payloads — none is a compatibility break, "because there is
nothing to be compatible with." The ADR also caught and corrected an inverted
claim in the specification about the shape of the (non-existent) divergence:
`SPECIFICATION.md:1875-1879` had the reference and happenstance's
match-all-query encodings backwards, which W7's direct reading corrected.

What W7 explicitly could *not* settle, and the ADR is careful to name rather
than silently assume: whether the DCB specification page has changed since
the 2026-08-05 fetch it was read from; what `JSON.stringify` would literally
emit for the reference's `Query` type, since that conclusion is read off an
object literal rather than executed; and whether any *other* DCB
implementation — there being no canonical one to interoperate with — agrees
with the reference on anything.

## What is not decided

What happens if a DCB implementation does eventually publish an encoding.
Nothing in ADR-0016 commits happenstance to interoperate with one if it
appears — the deferral just says the question cannot be answered before one
exists. Whether interoperability would then be worth pursuing at all is
itself unsettled and not obviously answered "yes": happenstance's format
choices (postcard for the binary wire, private field names, its own
match-all-query encoding) were each made on their own merits once
compatibility stopped being a constraint, and unwinding any of them to match
a late-arriving external encoding would cost real design, not just
transcription.

## What forces it

A DCB implementation — any one — publishing a wire format or a wire-level
reference encoding. Until that happens the deferral is unfalsifiable by
construction, which the ADR states as the correct reading rather than a
weakness: "a bridge built today would be built against an illustration the
specification disclaims, and the first real DCB peer would break it."
Independent of that external trigger, phase 13 — the sync crate and its
testkit — is the internal owner, because it is the first point at which
anything of happenstance's own is on a wire another implementation could sit
at the far end of, DCB-published or not.

## Ordered sub-questions

1. If a DCB implementation publishes an encoding, does happenstance attempt a
   bridge, or does it explicitly decline interoperability and say so as a
   design position rather than a scheduling gap?
2. Is "another implementation publishes a format" the only event that could
   force this, or would a *user* request for cross-implementation
   interoperability (independent of what DCB itself standardises) force the
   same question earlier, through phase 13's own design rather than through
   WF-1's marker?
3. Does re-checking W7's finding belong to phase 13 as a standing task — since
   the DCB specification page could change between the 2026-08-05 fetch and
   whenever phase 13 starts — or does it wait to be re-triggered only by a
   concrete interoperability request?
