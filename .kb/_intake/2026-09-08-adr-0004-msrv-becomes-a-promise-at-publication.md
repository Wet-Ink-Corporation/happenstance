# ADR-0004 loses `provisional` at publication, and it cannot be done before

> **HELD OUT OF THE INGEST WAVE — do not pass this file to `/redkiln:kb-ingest`.**
>
> **This brief is already discharged**, by
> [`.kb/decisions/0037-msrv-becomes-a-promise-at-0-2-0.md`](../decisions/0037-msrv-becomes-a-promise-at-0-2-0.md),
> which is `status: accepted, phase: 12` and records the identical proposition —
> and records it with the *correct* wiring, which this brief gets wrong. Atom 0037
> makes it an **amendment lineage**: ADR-0004 and ADR-0029 stay accepted and
> byte-identical and neither gets a `superseded_by` flip, on the precedent ADR-0029
> itself set against ADR-0004. Step 3 of this brief instead asks the ingest to
> author *"the superseding atom"*, and an adjudicator biased toward MERGE would aim
> that at a body-frozen atom.
>
> Kept rather than deleted because its middle section — *it is not true yet,
> nothing is on crates.io at `0.2.0`* — is correct, is still correct at the time of
> writing, and is the fact `CLAUDE.md`'s MSRV constraint was corrected against
> (measured: `max_stable_version` is `0.0.0` for all seven crates, nothing yanked).
> Retire this file once the tag lands.

**Date:** 2026-09-08
**Kind:** scheduled amendment, staged rather than applied
**Owner:** whoever runs the publish

## What is owed

`RUNBOOK.md`'s phase 12 work list carries *"ADR-0004 loses `provisional`; the MSRV
becomes a promise."* ADR-0004's marker was always conditional on exactly this
event: the floor is provisional *because* nothing is published and no downstream
consumer is pinned, and `0.2.0` is what ends that.

## Why this is staged and not applied

Two reasons, and the second is the one that decides it.

**It is not true yet.** Nothing is on crates.io at `0.2.0`. Lifting the marker
now would put a promise in the record before the thing that makes it a promise
exists — which is the same present-tense defect `HANDOVER.md` already tracks in
six places, added to deliberately rather than by inheritance.

**And an accepted decision atom is immutable.** `.kb/decisions/0004-…` carries
`status: accepted`, and the discipline is that a correction is a *new atom that
supersedes*, never an edit to the body. Lifting a `provisional` marker is a change
to what the decision claims, not a repointed referent — so unlike a drifted line
number, it genuinely is the case the immutability rule is for. It needs an
authored atom, and atoms come from `/redkiln:kb-ingest` rather than from a hand.

## What to do, in order

1. Publish `0.2.0`.
2. Stage a brief recording that ADR-0004's condition has been met — the MSRV
   1.97.1 is now a promise to real consumers, and raising it is a breaking change
   needing its own decision record rather than a commit message.
3. Run `/redkiln:kb-ingest`, which authors the superseding atom.

## The thing worth carrying into that atom

The floor moved once already, at phase 2, and ADR-0029 records why: it was a
**dependency's build script** that forced it, not this workspace's code. Five of
the five database crates here declare no `rust-version` at all, so `cargo hack
--rust-version` cannot protect a floor against them and neither can
`resolver = "3"`. Only running the compiler finds it.

That matters for the promise's shape. What `0.2.0` promises is that *this
workspace* compiles at 1.97.1; it cannot promise that a future patch release of a
dependency will. The `msrv` CI job is the instrument, and it currently proves
nothing because the pinned toolchain and the floor are the same version — it is
kept for the day they diverge, and ADR-0029 says so.
