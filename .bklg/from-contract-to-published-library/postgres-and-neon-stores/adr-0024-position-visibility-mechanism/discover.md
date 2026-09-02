---
item: HS-S0065
stage: discover
created: 2026-08-12T13:02:33.456Z
updated: 2026-08-12T13:02:33.456Z
template_sig: 86ce4036
rendered_sig: ed89a1aa
---

# Discover — ADR-0024 — the position-visibility mechanism, decided on numbers

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one line: ADR-0024 lands accepted, choosing the mechanism on a number **re-measured against the real adapter** — steady state and with a long-running transaction held open on the same database — and naming the two arms that lost | `_storymap.md`, *Slices* table, `adr-0024-position-visibility-mechanism` row | The deliverable is a decision atom carrying a measurement, not a preference and not a citation of a prior measurement |
| **AC-001** — an accepted atom under `.kb/decisions/` names the chosen mechanism, the two that lost and why, and carries the measured cost including behaviour with a long-running transaction; `redkiln validate --kb` passes and `.kb/maps/decision-map.md` carries its row | `project.md`, *Acceptance criteria*, AC-001 | Sole owner. Four checkable artefacts: the atom, the two rejections with reasons, the number, and the map row |
| `depends_on: postgres-append-and-frontier-head`, `postgres-concurrency-family`, `postgres-rule-controls` | manifest; `_storymap.md`, *Merge order* item 3 | HS-S0062 supplies the adapter the number is measured against; HS-S0063 supplies the evidence that writers stayed unserialised; HS-S0064 supplies the control proving CF-13 bites — and, if the poll-padding decorator is excused rather than built, the text explaining why belongs in this record |
| The ADR lands **after** a working append path, not before it | `_decomposition.md`, *Architecture brief* §8 point 2 | Deliberate inversion of the usual order, because the number owed is a re-measurement against an adapter, not against four SQL strategies |
| The experiment "measured four SQL strategies, not four implementations of `SendEventStore`" — no connection pooling, no transaction lifetime tied to a trait method's async boundary, no cursor, no error mapping | `.kb/open-questions/postgres-arm-c-structural-cost.md`, *What is true today* | The four named gaps between "a SQL strategy passed" and "an adapter passed". This is what makes citing phase 2's number insufficient |
| Its four ordered sub-questions are the shape of the "measured cost" section | `.kb/open-questions/postgres-arm-c-structural-cost.md`, *Ordered sub-questions* | (1) does `sqlx`'s pooling/transaction model let `append` hold the snapshot without contorting the signature; (2) does CF-13 still pass against a real adapter given its poll-count limitation; (3) what staleness bound should the docs promise; (4) does arm B-tag get reconsidered |
| ADR-0013 settles that **one affordable mechanism exists** and explicitly does not choose the adapter's | `.kb/decisions/0013-position-assignment-and-visibility.md`; quoted in `_grounding.md`, *Accepted decision atoms* | "Which mechanism a real `happenstance-postgres` adapter uses is left open for the adapter's own phase." ADR-0024 is that phase |
| ADR-0013's global-invariant premise rests on the projection checkpoint being global, which is unsettled and **owned by phase 6** | `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`; `project.md`, *Out of scope* | ADR-0024 inherits the risk without owning it and must say so in the record (`_decomposition.md`, *Architecture brief* §10) |
| Phase 2's arms: arm C at 0.987 / 0.993 / 1.015 / 1.026× at 1 / 8 / 32 / 64 clients; arms A and B-const correct at 16× and 30× slower; arm B-tag cheap because it buys a *per-boundary* invariant ES-10 does not state | `experiments/position-visibility/README.md:13-19`; `RUNBOOK.md:520` | The prior art the record cites — and must not present as its own measurement |
| Staleness measured at 0.688 ms unloaded and 4010.719 ms behind an unrelated five-second write in an unrelated database | `spec/SPECIFICATION.md:2836-2840`; `experiments/position-visibility/results/staleness_pinned.txt` | A range spanning four orders of magnitude. Sub-question 3 is which end a caller of this crate should plan for |
| "The ADR owes a number, not a preference", with the third arm measured under contention and the first under a deliberately held transaction | `RUNBOOK.md:4341-4342` | The runbook's own framing of DR-3, and the second half is the part the phase-2 run cannot supply |
| Atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`, **not by hand** | `CLAUDE.md`, *Where the work lives*; `_decomposition.md`, *Architecture brief* §2, Root D | Hand-writing them "produces the directory layout of the process without the process, which is why the first attempt at this was reverted (`0269720`)" |
| A decision lives in two places on purpose: the ~100-line atom in `.kb/decisions/`, the full record in `references/adr/`, and the atom links it | `CLAUDE.md`, *Where the work lives* | The compiler transcripts, rejected alternatives and measurement tables a summary cannot hold belong in the long form. `spec-trace` cites the long form by line |
| The experiment directory is deliberately not in the gate | `experiments/position-visibility/README.md:21-23` | "measurements… and not in the gate". The number this story produces is read out of a one-off run and becomes ADR prose; no test asserts it |

## Questions

**Answered.**

1. *Which mechanism does the ADR choose?* Not decided at `discover`, and it must not
   be: the evidence is produced by the three upstream stories and the record's whole
   value is that the choice follows the evidence rather than the other way round.
   What *is* settled is the shape of the answer — the four ordered sub-questions of
   `.kb/open-questions/postgres-arm-c-structural-cost.md` are its section headings.
2. *May it cite phase 2's numbers?* As prior art, yes. As the adapter's cost, no —
   the open question says in terms that the experiment measured SQL strategies and
   not implementations of the port, and names the four gaps.
3. *Does it own the global-versus-per-boundary question?* No. It inherits ADR-0013's
   global premise without owning it, states that inheritance plainly, and records
   that if `projection-store-freeze` (HS-P0010) makes the checkpoint
   boundary-scoped, ADR-0013 reopens and this measurement reopens with it.
4. *How does the atom reach `.kb/decisions/`?* Through `.kb/_intake/` and
   `/redkiln:kb-ingest`, with the long form in `references/adr/` and the atom
   linking it. Never hand-authored.

**Deferred to `spec`.**

5. *The harness that produces the re-measurement* — whether it extends
   `experiments/position-visibility/` or is a new one-off beside the live Postgres
   job. Either is admissible; it stays out of `cargo xtask ci` at every tier.
6. *Whether arm B-tag is reconsidered* (sub-question 4). Genuinely open, and the
   answer depends on numbers this story has not taken yet.

**Deferred to the owning stories.**

7. *How the adapter buys ES-10's visibility invariant, as code.*
   `postgres-append-and-frontier-head` (HS-S0062) already wired the mechanism
   ADR-0013 established as affordable. This story decides and records; if it lands
   somewhere other than what HS-S0062 wired, the change is a follow-on to that
   story's code and is named as such rather than folded in silently.
8. *Whether `conflicting_position` is a promise every adapter owes or a hint one may
   omit.* `neon-conflicting-position-verdict` (HS-S0070). Named here only to record
   that ADR-0024 is **not** the place it gets settled in passing: ES-25 already
   answers it at the port level (`spec/SPECIFICATION.md:3746-3748`), and the ledger
   correction is a separate record.

## Decision

The problem this slice solves is that the repository has been carrying a mechanism
decision as an experiment result for two phases. ADR-0013 lifted ES-10 to
`[FROZEN]` on the strength of a phase-2 harness and said, in terms, that it was
establishing that *one affordable mechanism exists* rather than choosing the
adapter's — and the open question that fell out of that names four concrete gaps
between "a SQL strategy passed" and "an adapter passed": no connection pooling, no
transaction lifetime tied to a trait method's async boundary, no cursor, no error
mapping. By the time this story runs, all four gaps have been closed by real code,
so the number can finally be taken against the thing it is about. The spec will
cover: the intake document's structure, following the open question's four ordered
sub-questions; the steady-state re-measurement and the long-running-transaction
scenario DR-3 names; the two arms that lost and the reason each lost, including
that arm B-tag is cheap because it buys a per-boundary invariant ES-10 does not
state; the inherited-not-owned statement about the global premise; the poll-shape
finding from HS-S0064 and, if the decorator was excused, why; the split between the
atom and the long record in `references/adr/`; and the `.kb/maps/decision-map.md`
row. **No `[FROZEN]` clause is changed.** ES-10 stays as written — this record says
how an adapter *buys* it, and `spec/SPECIFICATION.md:2848-2850` has already settled
that if the poll-padding instrument fires, the rule changes and the clause does not.

## The wrong implementation

**An ADR-0024 that cites phase 2's numbers as the adapter's cost.** Write the atom.
Choose `xid8` + `pg_snapshot_xmin`. Quote 0.987–1.026× at 1 / 8 / 32 / 64 clients,
16× and 30× for the serialising arms, 0.688 ms and 4010.719 ms for staleness. Name
the two losers with reasons. File it through `.kb/_intake/` and `/redkiln:kb-ingest`,
add the `.kb/maps/decision-map.md` row. **Every check passes**: `redkiln validate --kb`
checks `KbFrontmatter` conformance and accepted-decision immutability, not
provenance; `redkiln doctor` still reports exactly its six expected advisories;
`cargo xtask ci` never reads `.kb/`; every figure quoted is a real figure from a
real server. AC-001 reads as satisfied by inspection. And the record is a
restatement of ADR-0013 with a different number in the header, because the
measurement it reports was taken against a bare SQL harness with no pool, no
`EventStore::append` lifetime, no cursor and no error mapping — the four gaps
`.kb/open-questions/postgres-arm-c-structural-cost.md` exists to name — and against
a database with no long-running transaction deliberately held open, which is the
half DR-3 and `RUNBOOK.md:4341-4342` specifically demand. An accepted decision atom
is **immutable**, so this one is not correctable by edit: the remedy would be a
second atom superseding it, and the audit at HS-P0016 would be reading a supersession
chain instead of a decision.

Two quieter siblings, both of which also pass everything:

- **The atom written by hand into `.kb/decisions/`.** Well-formed frontmatter,
  sensible id, a good body. `redkiln validate --kb` is green. It is the exact move
  `0269720` reverted — the directory layout of the process without the process —
  and it skips the claim extraction and cross-file adjudication that would have
  caught the conflict with ADR-0013's own caveats (`CLAUDE.md`, *Where the work
  lives*).
- **The atom that presents the global invariant as permanently settled.** Nothing
  validates the premise a decision rests on, so an ADR-0024 that simply does not
  mention `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`
  reads as more settled than it is, and phase 6 later moves the ground under it with
  no record that anyone knew it could.

None of these is a store, so none belongs in `crates/happenstance-testkit/tests/`;
this story adds no conformance rule. The instrument that rejects all three is the
review at `spec` and the adversarial verification on the record itself — which is
why `spec` must state the provenance requirement as an acceptance criterion in
words, not leave it to be inferred from "measured, not preferred".

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
