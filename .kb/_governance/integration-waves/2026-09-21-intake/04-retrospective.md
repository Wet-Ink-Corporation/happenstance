# Wave `2026-09-21-intake` — retrospective

## What merged vs. what was created

Of five operations, two created new atoms and three merged into atoms already
in the corpus — a 2:3 split, the smallest wave in the run of seven by
operation count, and the first in which one staged file forces the wave's
whole shape rather than requiring cross-file deduplication (`01`'s "Cross-file
dedup" section: one file, no cross-file collapsing, but a real cross-*layer*
risk between the decision and the reference atom, handled by the citation
split `decisions/README.md` and `reference/README.md` both name).

The wave's defining fact, stated in `01`: this is the **first wave in seven**
to carry a claim labelled `conflicts` against an accepted decision atom rather
than only `extends`/`aligns`. `kb-decision-0022` §11 asserted a premise —
`busy = 0` in every row of the 64-contender table — and named its own reopen
condition on that premise (`busy > 0`). `kb-reference-busy-timeout-margin-001`
measured `busy > 0` at the shipped contender count three weeks before this
wave, and the falsifiers-fired open question has carried the conflict,
unresolved, since 2026-09-04. This wave is that conflict resolving: a decision
refuted on its own stated terms, by its own stated trigger.

Nothing in this wave edited an accepted body or flipped a `status` field on
any existing atom. `kb-decision-0022` stays `accepted`,
`superseded_by: null`, at `HEAD` — the partial supersession is expressed
entirely in `kb-decision-0065`'s `supersedes: null` / `depends_on:
[kb-decision-0022]` pair and in the map annotation, per the convention
`kb-decision-0007` and `kb-decision-0031` set and `02`'s Adjudication 1 argues
at length.

## Unresolved claims

Two items carry real, named residual risk rather than a clean resolution —
full detail in `02`'s adjudications:

1. **The corpus still cannot express a scoped supersession in frontmatter.**
   `supersedes` carries no section-level qualifier, so a machine reading
   `kb-decision-0022`'s frontmatter alone cannot see that §11 has moved; a
   reader who cites its immutable summary's "busy_timeout 5,000 ms" as current
   is not caught by anything `redkiln validate --kb` checks. The three
   mitigations applied are the ones the corpus already uses — the new atom's
   title names the superseded section, `depends_on` makes the edge
   machine-visible in one direction, and the `decisionMap` row carries the
   partial-supersession annotation as a *rendered* fact. `kb-open-question-
   adr-status-vocabulary-001`'s amendment (Op 5) records this as a third,
   now section-grain instance and a first datum toward *convention* rather
   than *stopgap* — the staged intake asked in writing for the scoped-`
   supersedes` spelling and the wave declined it on the mechanical
   consequence (Adjudication 1). The open question stays open; nothing here
   resolves whether the convention is permanent.
2. **The falsifiers-fired open question is now two-thirds open, and the wave
   deliberately does not let the resolved third read as the atom's centre.**
   §11 is the finding with a measurement behind it and the one the code
   already acted on, so it is the one that reads as settled. It is not the
   atom's most severe finding: §9 is a correctness gap
   (`SqliteEventStoreError::NoRuntime` unreachable for a store outliving its
   runtime) that outranks a liveness cap on any reading, and it is still
   owed the twenty-line reproduction its own falsifier text demands; §8/§16
   is a shipped SQL shape measured to lose in nine of nine two-tag cells,
   and its repair runs through a `[FROZEN]` specification clause (ES-27's
   `Rejects:` prose), which makes it a decision this wave explicitly did not
   attempt (Adjudication 6). The amendment leads with what stands open and
   records §11's resolution as one dated section inside it, per Adjudication
   3.

## Follow-ups

- **The contention-tolerance question is now blocked on a port change, not
  an instrument.** `kb-open-question-testkit-contention-tolerance-001`'s own
  blocking instrument (`FaultyStore::contend_next`) landed this cycle and
  has already discriminated the two candidate designs: a per-error
  classifier channel cannot be prototyped in the testkit at all, because the
  port offers exactly one classifier (`AppendError::is_condition_violated`)
  and it answers `false` for a busy store exactly as for a broken one; a
  per-fixture capability can be built today. That is new information the
  digest did not extract (`01`, claim C4; `02`, Adjudication 4) and it
  changes the *cost* of trying each arm without choosing between them. The
  question itself — fixture-side, contract-side, or no tolerance — is
  unowned by this wave and stays open.
- **§9's twenty-line reproduction is still not written anywhere in this
  tree.** Both `kb-open-question-adr-0022-falsifiers-fired-001` before this
  wave and its amendment after it say the same thing: a captured-`Handle`
  deployment is a hazard until a deployment shows it. Nobody in this wave
  attempted it, on purpose — it is a correctness question with its own
  falsifier, not a liveness cap this wave's evidence bears on.
- **§8/§16's SQL-shape repair is a `[FROZEN]`-clause edit and was
  correctly left alone.** `grep -n "busy_timeout\|BUSY_TIMEOUT\|5,000
  ms\|5000 ms" spec/SPECIFICATION.md` returns nothing, confirming ADR-0065
  itself needed no `cargo xtask spec-trace` consideration; the same check in
  the other direction confirms ES-27's `Rejects:` prose at
  `spec/SPECIFICATION.md:3902-3905` quotes the §8/§16 aggregate's "roughly
  200x," so *that* repair is a decision CLAUDE.md routes through a new ADR,
  not a documentation sweep this wave could fold in (Adjudication 6).

## Doctor problems this wave set aside as out of scope

`redkiln doctor --json` reported 11 gating problems (severity `error`)
against the whole repository. All 11 are outside this wave's path set
(`.kb/`) and none were touched or repaired here:

- unconsumed-foundation: .bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0003-provisional-lift/story.md
- unconsumed-foundation: .bklg/from-contract-to-published-library/replication-identity-and-ingest/open-questions-resolved-and-indexed/story.md
- ledger-chain-broken: .redkiln/telemetry/events/ryan-britton@happenstance@runs.jsonl
- ledger-chain-broken: ../../../.redkiln/telemetry/events/ryan-britton@happenstance@runs.jsonl
- ledger-chain-broken: ../kb-intake-2026-09-11/.redkiln/telemetry/events/ryan-britton@happenstance@runs.jsonl
- ledger-chain-broken: .redkiln/telemetry/events/ryan-britton@kb-intake-2026-09-07@runs.jsonl
- ledger-chain-broken: ../../../.redkiln/telemetry/events/ryan-britton@kb-intake-2026-09-07@runs.jsonl
- ledger-chain-broken: ../kb-intake-2026-09-11/.redkiln/telemetry/events/ryan-britton@kb-intake-2026-09-07@runs.jsonl
- ledger-chain-broken: .redkiln/telemetry/events/ryan-britton@kb-intake-2026-09-11@runs.jsonl
- ledger-chain-broken: ../../../.redkiln/telemetry/events/ryan-britton@kb-intake-2026-09-11@runs.jsonl
- ledger-chain-broken: ../kb-intake-2026-09-11/.redkiln/telemetry/events/ryan-britton@kb-intake-2026-09-11@runs.jsonl

None of these is a `.kb/` problem, and this wave's own writes under `.kb/`
validated clean (`redkiln validate --kb` exited 0, "validate passed.", and no
doctor problem's `file` falls under `.kb/`). The two `unconsumed-foundation`
problems are pre-existing backlog-structure findings against a different
initiative's stories; the nine `ledger-chain-broken` problems are telemetry
files under `.redkiln/telemetry/events/` — some referenced by relative paths
that resolve outside this worktree entirely (`../../../...`,
`../kb-intake-2026-09-11/...`), which is itself evidence they are not this
wave's to fix. `doctor`'s `warnings[]` (process-pack drift, template drift,
and similar pre-existing repository-wide advisories) neither gate `doctor`
itself nor this wave and are not repeated here.

## Instruments run

- `redkiln validate --kb` — exit 0, "validate passed." (warnings only: 210
  `.bklg` items with no ledger transition record, none under `.kb/`.)
- `redkiln doctor --json` — exit 1 (`ok:false`), 11 in-error `problems[]`, all
  out of scope for this wave as itemized above.
