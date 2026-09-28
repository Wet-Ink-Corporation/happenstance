# Wave `2026-09-21-intake` — integration summary

One staged file, five operations, ordered and adjudicated in
`02-placement-and-adjudication.md`. This file states the result: what exists
now that did not before, what changed in place, and which map atoms carry the
wiring.

## Atoms created (2)

| Atom | Kind | id | Sources |
| --- | --- | --- | --- |
| `.kb/reference/busy-timeout-adapter-cap-sweep-2026-09.md` | reference | `kb-reference-busy-timeout-adapter-cap-sweep-001` | §11 |
| `.kb/decisions/0065-adr-0022-s11-busy-timeout-is-fifteen-seconds.md` | decision | `kb-decision-0065` | §11 |

The reference atom is created first — `decisions/README.md` puts the evidence
outside the decision and the decision cites it by id, so the decision cannot
be written before its own citation exists. `0065` takes the next unclaimed
decision number (`kb-decision-0064` was the last-taken, from the
`2026-09-11-intake` wave).

## Atoms amended (3)

| Atom | id | What changed |
| --- | --- | --- |
| `.kb/open-questions/adr-0022-falsifiers-have-fired.md` | `kb-open-question-adr-0022-falsifiers-fired-001` | **amended, not closed** — one dated section recording that §11 is now decided (the fourth reading the atom itself proposed for §8 — "supersede in part" — applied instead to §11), that the routing question is half-answered (`kb-open-question-testkit-contention-tolerance-001` now owns the contention half), and that §9 and §8/§16 stand exactly as before. Title deliberately unchanged — see `02`, Op 3 |
| `.kb/open-questions/no-fixture-tolerance-for-transient-contention.md` | `kb-open-question-testkit-contention-tolerance-001` | **amended, not closed** — one dated section recording that the blocking instrument it specified now exists (`FaultyStore::contend_next`, `crates/happenstance-testkit/src/faulty.rs:333`), that its own prediction (the two arms differ in testkit-buildability) has been tested and held — the per-error arm needs an `AppendError::Busy` variant the port does not have, the per-fixture arm can be built today — and that the underlying tolerance question is unmoved, only rarer |
| `.kb/open-questions/adr-status-vocabulary-exceeds-the-schema.md` | `kb-open-question-adr-status-vocabulary-001` | **discretionary amendment** — three sentences recording a third instance of the "partly superseded" convention, the first at section grain (one of fifteen) rather than half-document grain, and a first datum toward convention-vs-stopgap: the staged intake asked in writing for `supersedes` scoped to §11 and the wave declined it (Adjudication 1) |

## Decision corpus: one partial supersession, zero accepted bodies edited

One decision atom added (`kb-decision-0065`); zero accepted bodies edited;
zero `status` flips on any existing atom. `kb-decision-0065` partly
supersedes `kb-decision-0022` at **section 11 only** — the `busy_timeout`
pragma value — leaving §4, §6, §7, §9, §10, §12 and §15 standing.
`supersedes: null` on the new atom, `kb-decision-0022` keeps
`status: accepted` with `superseded_by: null`, and `depends_on:
[kb-decision-0022]` carries the edge in the other direction. This is the same
shape `kb-decision-0007` used against ADR-0006 and `kb-decision-0031` used
against ADR-0007 — both `supersedes: null`, both prose-only partial
supersessions — applied here for the first time at **section** grain rather
than half-a-document grain (`02`, Adjudication 1).

The decision is forced by a `conflicts` claim against an accepted, immutable
atom (claim A2 in `01`): `kb-decision-0022` §11 asserted `busy = 0` held in
every row of the 64-contender table and named its own reopen condition as
`busy > 0`; `kb-reference-busy-timeout-margin-001` (2026-09-03) measured
`busy > 0` at the shipped `CONTENDERS = 64`, firing that condition. This is
the first wave in seven to carry a `conflicts` claim against an accepted
decision rather than only `extends`/`aligns` claims.

## Map atoms updated (3)

- **`maps/decision-map.md`** — one new row (ADR-0065, phase `null`,
  "partly supersedes `kb-decision-0022`"), the existing ADR-0022 row
  annotated ("accepted (partly superseded)" / "partly superseded by
  `kb-decision-0065`") rather than flipped, and a new paragraph in *Reading
  the partial-supersession chain* naming the section-grain instance.
- **`maps/domain-map.md`** — the measurement/host area gains ADR-0065 and
  the adapter cap-sweep reference atom in its "2026-09-21 wave" paragraph;
  the reference-atom bullet list gains the sweep atom, cross-linked to its
  sibling margin atom; the phase-9-and-later decision list gains ADR-0065 as
  a seventeenth entry with an explicit pointer to `decision-map.md` for the
  row. No new domain opened.
- **`maps/open-questions-index.md`** — no bullet removed, no status flip;
  three existing bullets annotated in place (falsifiers-fired,
  contention-tolerance, status-vocabulary) plus a dated paragraph in the
  map's own summary section naming all three amendments together.

## Links wired

`kb-reference-busy-timeout-adapter-cap-sweep-001`'s `related` names its
sibling (`kb-reference-busy-timeout-margin-001`), the append-condition and
one-connection-latency reference atoms, both relevant decisions
(`kb-decision-0022`, `kb-decision-0064`), and both open questions it bears
on — written after those atoms existed, since it is Op 1. `kb-decision-0065`'s
`depends_on` and `related` both point only at atoms already on disk when it
was written (Op 2, after Op 1). The three `merge_existing` ops (3–5) each
gain `kb-decision-0065` in `related`; ops 3 and 4 additionally gain
`kb-reference-busy-timeout-adapter-cap-sweep-001`. Reciprocal backlinks — the
sweep atom's own `related` gaining the open questions, and five *other*,
pre-existing atoms — three reference, one governance, one open question
(`busy-timeout-margin-2026-09.md`, `append-condition-experiment-2026-08.md`,
`one-connection-latency-2026-09.md`, `cf-33-cf-34-scope-outside-the-testkit.md`,
`what-may-refute-a-finding.md`) gaining a pointer to `kb-decision-0065` or the
new reference atom — are the Maps phase's writes, applied after all five ops,
per `02`'s Ordering section.

## Intake cleared

The one staged file,
`2026-09-21-adr-0022-s11-superseded-busy-timeout-is-fifteen-seconds.md`,
removed from `.kb/_intake/` on success (step 3 of this wave).
