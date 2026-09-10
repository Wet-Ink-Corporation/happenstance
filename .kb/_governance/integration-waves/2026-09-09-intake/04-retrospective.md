# Wave `2026-09-09-intake` — retrospective

## What merged vs. what was created

Of fourteen operations, six created new atoms and eight merged into atoms
already in the corpus — a 6:8 split, and every merge had a direct, pre-existing
owner rather than a near-miss forced to fit. The two decision-shaped intake
files (`adr-0060`/`ps-2` and `adr-0061`/`es-11`) each collapsed a
finding-plus-decision pair into one atom rather than two, because the finding
names itself as owned by the decision in its own text (`ps-2`: *"Owner: PS-2's,
not this adapter's"*; `es-11`'s own callout: *"Settled on 2026-09-08 by
ADR-0061"*). The real dedup this wave did — the one no single-file digest
found — was Op 8: two separately-routed claims (a Ladybug Rust-level finding
and a PS-2 thirteen-clause claim) turned out to be the same family as an
existing atom about falsifiers that can never fire, and both merged into it
rather than becoming a second atom on the same idea.

## Unresolved claims

None. Every claim across the five staged files resolved to a `create_new`,
`defer_open_question`, or `merge_existing` disposition — nothing was dropped
for lack of an owner, and nothing was forced into resolution to avoid an
`open-questions/` entry it should have gotten instead.

## Follow-ups

Three items this wave deliberately left open rather than guessed at
(full detail in `02-placement-and-adjudication.md`'s closing section):

1. **Whether `lbug`'s file lock (`Error: 33` on two `Database`s over one
   directory) is the driver's property or the Windows platform's.** One run,
   one host. `kb-decision-0025`'s conclusion (`Arc<Database>` with a second
   `Connection` per handle) is forced by the observation either way; only the
   *generality* of the observation is unresolved, and the new reference atom
   states the single-host observation rather than promoting it to a property.
2. **What to do about `.gitignore`:69's `*-output.txt` rule eating
   `experiments/ladybug-driver-probes/results/probe-output.txt`.** Recorded as
   a wave-discovered open question (`kb-open-question-experiment-raw-output-ignored-001`)
   rather than answered — renaming the file, carving a `.gitignore` exception,
   and accepting the README's verbatim transcription as sufficient evidence
   are three different-cost answers, and choosing among them isn't an ingest's
   call.
3. **Whether `kb-decision-0060`'s `phase` field should read `11` or `6`.**
   Recorded as `11` (the phase that did the work, per the ADR-0024/ADR-0058
   precedent) rather than `6` (the phase that owns the clause, per
   `kb-decision-0036`'s own `phase: 6`). Both readings are defensible and the
   corpus does not write the convention down anywhere a validator checks.

## Doctor problems this wave set aside as out of scope

`redkiln doctor --json` reported 6 gating problems (severity `error`) against
the whole repository. All 6 are outside this wave's path set (`.kb/`) and none
were touched or repaired here:

- unconsumed-foundation: .bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0003-provisional-lift/story.md
- unconsumed-foundation: .bklg/from-contract-to-published-library/replication-identity-and-ingest/open-questions-resolved-and-indexed/story.md
- ledger-chain-broken: .redkiln/telemetry/events/ryan-britton@happenstance@runs.jsonl
- ledger-chain-broken: ../../../.redkiln/telemetry/events/ryan-britton@happenstance@runs.jsonl
- ledger-chain-broken: .redkiln/telemetry/events/ryan-britton@kb-intake-2026-09-07@runs.jsonl
- ledger-chain-broken: ../../../.redkiln/telemetry/events/ryan-britton@kb-intake-2026-09-07@runs.jsonl

None of these is a `.kb/` problem, and this wave's own writes under `.kb/`
validated clean (`redkiln validate --kb` exited 0, and no doctor problem's
`file` falls under `.kb/`). `doctor`'s `warnings[]` (process-pack drift,
template drift, worktree-unpushed, ledger-unledgered, evidence-unledgered) are
pre-existing, repository-wide advisories that neither gate `doctor` itself nor
this wave, and are not repeated here.

## Instruments run

- `redkiln validate --kb` — exit 0, "validate passed."
- `redkiln doctor --json` — exit 1 (`ok:false`), 6 in-error `problems[]`, all
  out of scope for this wave as itemized above.
