# Wave `2026-09-11-intake` — retrospective

## What merged vs. what was created

Of fifteen operations, seven created new atoms and eight merged into atoms
already in the corpus — a 7:8 split. Two of the seven creates are the wave's
hardest call: `kb-decision-0062` and `kb-decision-0063` record decisions whose
supporting tree (a live-transaction Postgres projection store, a moved probe
signature, a rewritten specification clause) is not visible from this
worktree at `86a410c` — it sits on `lane/projection-probe-seam`, uncommitted,
and under this wave's hard constraint that branch is not read for authority.
Both atoms are minted `status: accepted` anyway, on the corpus's own stated
convention that a decision atom is history — the choice was made on the date
the brief carries — with every unverifiable count attributed to the brief in
the brief's voice and the tree's state at `HEAD` stated in the first clause of
each summary (`02`'s Adjudication 1 is the full argument, including the
counter-case for holding both atoms out, which the wave did not take).

One resolution happened rather than a merge: `kb-open-question-probe-read-
through-signature-001` flips from `accepted` to `superseded` because ADR-0062
answers the question it asks, wider than the atom's own recommendation
(Adjudication 2). Two decisions were discharged rather than superseded —
`kb-decision-0036` and `kb-decision-0060` stay `accepted`, byte-identical,
because their findings were true when written and the condition they were
conditional on (PS-2's bar) has now been met, not found wrong (Adjudication
5). Nothing in this wave edited an accepted body, flipped a `status` field
that was not `kb-open-question-probe-read-through-signature-001`'s, or wrote
a `superseded_by`.

## Unresolved claims

None dropped for lack of an owner. Three items carry real, named residual
risk rather than a clean resolution — full detail in `02`'s adjudications:

1. **Whether `kb-decision-0062` and `kb-decision-0063` should have been
   minted at all, given their evidence sits on an unlanded branch.** The
   wave took the position that a decision atom states that a choice was made,
   not that its evidence has landed, and that nothing in either atom is false
   at `HEAD`. The counter-case — that if the lane's own falsifier fires
   during the work, the corpus holds two accepted decisions about a port that
   never existed, repairable only by a supersession pair — is recorded in
   `02`'s Adjudication 1 as the argument this wave found strongest and did
   not take. Reversing the call (dropping ops 4, 5, 6, 8, 9, 10, 14) is named
   as a clean, isolable edit if the owner disagrees.
2. **Whether `kb-decision-0064`'s `phase: null` and `kb-decision-0062`'s/
   `kb-decision-0063`'s `phase: 12` are the right reading.** Both are
   judgement calls on a field the corpus does not write a convention down
   for anywhere a validator checks; each is recorded as a judgement call in
   `02` rather than a fact.
3. **The CF-34 boundary's residual risk.** `preflight.sh` is unreachable from
   the gate today by construction (no step, no `verify:` command, no CI job
   invokes it, `ops/` is `INERT`), but a self-hosted runner would change
   that — named in `kb-decision-0064` and in the amended
   `kb-open-question-cf-33-cf-34-scope-001` as "a decision, not a
   configuration change," and left exactly that unresolved.

## Follow-ups

- **The citation-anchor contradiction's actual fix is not this repository's
  to build.** The chosen repair — `/redkiln:kb-ingest` re-resolving every
  `path:line` against `HEAD` as it authors, refusing the wave rather than the
  atom when one does not — lives in the redkiln plugin, whose backlog already
  tracks it. The interim habit recorded in the amended open question: run
  `cargo xtask ci` immediately after an ingest wave merges, rather than
  trusting the pre-merge green — the 2026-09-07 wave's own citation drift
  reddened `main` from its merge commit until the next session's first
  command, and nobody ran the gate in between.
- **`2026-09-08-intake-is-outside-the-citation-scan.md` stays in `.kb/_intake/`.**
  This wave used it as a source for three operations (ops 7, 12, 13 — the
  immutability open question and two merges), but it was not on the
  orchestrator's clear list for this run (the given list names four of the
  five staged files). Left in place rather than removed on this wave's own
  initiative, since the clear list is the orchestrator's call to make, not an
  inference from what got used as a source. Flagging it here so a future wave
  does not re-extract claims already integrated: everything the file
  contributed is now in `kb-open-question-immutability-check-pre-commit-001`
  and `kb-governance-referent-not-reasoning-001`.
- **`2026-09-08-adr-0004-msrv-becomes-a-promise-at-publication.md` mints no
  decision-layer operation.** Five separate digests across this wave and
  prior extraction proposed merging it into `kb-decision-0037`; that atom is
  accepted and already carries every sentence the file wants carried, and a
  merge into an accepted decision's body is the one edit this corpus
  forbids. Its one net-new fact (the 2026-09-08 registry read) landed instead
  in the amended `kb-open-question-stale-0-0-0-name-reservations-001`.

## Doctor problems this wave set aside as out of scope

`redkiln doctor --json` reported 6 gating problems (severity `error`) against
the whole repository. All 6 are outside this wave's path set (`.kb/`) and
none were touched or repaired here:

- unconsumed-foundation: .bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0003-provisional-lift/story.md
- unconsumed-foundation: .bklg/from-contract-to-published-library/replication-identity-and-ingest/open-questions-resolved-and-indexed/story.md
- ledger-chain-broken: .redkiln/telemetry/events/ryan-britton@happenstance@runs.jsonl
- ledger-chain-broken: ../../../.redkiln/telemetry/events/ryan-britton@happenstance@runs.jsonl
- ledger-chain-broken: .redkiln/telemetry/events/ryan-britton@kb-intake-2026-09-07@runs.jsonl
- ledger-chain-broken: ../../../.redkiln/telemetry/events/ryan-britton@kb-intake-2026-09-07@runs.jsonl

None of these is a `.kb/` problem, and this wave's own writes under `.kb/`
validated clean (`redkiln validate --kb` exited 0, and no doctor problem's
`file` falls under `.kb/`). `doctor`'s `warnings[]` (process-pack drift,
template drift, worktree-unpushed, ledger-unledgered, evidence-unledgered)
are pre-existing, repository-wide advisories that neither gate `doctor`
itself nor this wave, and are not repeated here. This is the same 6-problem
set the 2026-09-09 wave recorded and set aside under the same reasoning —
none of them originates in, or was touched by, any `.kb` intake wave.

## Instruments run

- `redkiln validate --kb` — exit 0, "validate passed."
- `redkiln doctor --json` — exit 1 (`ok:false`), 6 in-error `problems[]`, all
  out of scope for this wave as itemized above.
