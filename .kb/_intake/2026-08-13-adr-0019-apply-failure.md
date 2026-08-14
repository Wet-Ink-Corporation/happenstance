# Staged: ADR-0019 — what happens when `apply` fails

**Staged 2026-08-13** for the next `/redkiln:kb-ingest` wave. **Not an atom.**
Long-form record: [`references/adr/0019-what-happens-when-apply-fails.md`](../../references/adr/0019-what-happens-when-apply-fails.md).

Stage in **one** wave with `…-adr-0017-projection-batch.md` and
`…-adr-0018-reset.md`; the wave-id and `status`-vocabulary notes in the ADR-0017
document apply here unchanged.

---

## The op: create one `decision` atom

`.kb/decisions/0019-what-happens-when-apply-fails.md`, `adr_id: ADR-0019`,
`phase: 6`, `status: accepted` with the provisional qualification **and its
falsifier** in the first clause of `summary`.

## The decision, stated as a decision rather than as an omission

**The port grows nothing for apply failure.** `ProjectionStore` gains no method,
no associated type, no error variant and no feature on the failure path. Three
facts make that true rather than convenient:

1. **The skip primitive already exists and nobody had noticed it.** `begin()`
   followed immediately by `commit(batch, id, poison_position, Live)` applies
   nothing and advances the checkpoint atomically, *"with the port exactly as
   written"* (`spec/SPECIFICATION.md:5420-5427`). What does not exist is any way to
   **record** that it happened — and routing that record through the projection's
   own batch means no new port surface and no store-side knowledge of what a skip
   means. In Kestrel Motor the skip is the Article 17 evidence, so a swallowed skip
   is a compliance failure rather than a missing log line.
2. **`rollback` must survive the port change, and PS-30 is why.** A fan-out runner
   that wraps `&mut P::Batch` in `AssertUnwindSafe` is defensible *only* because
   `rollback` exists: *"`AssertUnwindSafe` is a promise that no observer will see a
   half-mutated value, and `rollback` is what discharges it. Without the rollback
   the promise is a lie"* (`:5471-5475`).
3. **Isolation already works structurally.** `Projection::Store` is an associated
   type and checkpoints are per `(store, ProjectionId)` (ADR-0007), so a failure
   cannot span two (`:5455-5458`). PS-29's first half is discharged by a decision
   already taken.

**The failure policy is declared per projection, not per runner (PS-26,
`[FROZEN]`).** The alternative that lost is a runner-level `on_error: SkipPolicy`
configuration — *"the obvious design, it is what a builder API invites, and it
forces one wrong answer onto one of Wattline's two projections"* (`:5407-5410`).
The scenarios' disagreement **is** the result: halting is right for a revenue
ledger and wrong for an availability board.

## Deferred by name — the boundary this atom must make legible

`project.md`'s *Out of scope* assigns the `Projection` trait, the runner,
ADR-0020 / ADR-0021 and PS-33's falsifier to **HS-P0011**. Every rule PS-26 – PS-30
names is integration-level. So the atom **defers three things by name** rather
than designing them:

| Deferred | Stated at | Owner |
|---|---|---|
| `PumpError<E::Error, P::Error, A>` with its `Apply { position, error: A }` variant — three type parameters, a real cost, and `Box<dyn core::error::Error>` barred by house style and `no_std` | `spec/SPECIFICATION.md:5438-5445` | HS-P0011, ADR-0020 / ADR-0021 |
| the `SkipAndRecord` policy vocabulary, and what *"record"* means | PS-27's falsifier, `:5413-5416` | HS-P0011 |
| the supervisor's observability half — reporting a terminal state *without being polled* | PS-29's rule, `:5449-5453` | HS-P0011 |

An atom that designs any of the three has taken another project's scope and will
be reversed by the project that owns it.

## The evidence to quote: a stated absence

No skeleton exercised an apply failure — every body is `todo!()`. The atom quotes
the absence from `references/adapter-shapes.md:286-303` (*"Durability | **Nothing.
Every body is `todo!()`** | Phase 8"*), not a fabricated diagnostic, because *"a
table showing only `error[E….]` would rank it the most compatible adapter in the
workspace when it is the least"* (`:29-33`). That is the right evidence for this
decision rather than a consolation: six type checkers were shown the port and
raised no objection a failure surface would answer.

## The sweep's finding in this clause range

`references/evaluation/ps-clause-pairing-sweep.md` returned **isolated** for the
family, so this atom's scope is not widened. Two rows fall in PS-26 – PS-30:

- **PS-29 is `defective`, and it is one of the sweep's three `independent`
  defects.** The `MUST` requires the terminal state be *"observable through the
  API"*; the rule additionally demands the supervisor report *"without being polled
  for it"*. A supervisor exposing `fn failures(&self) -> Vec<Poisoned>` satisfies
  the sentence and fails the rule — a plausible first cut, and the shape the
  clause's own `Rejects` field describes losing. PS-29 is `[FROZEN]`, widening it
  changes the admitted set, so it is a **gap**: a new decision atom's at
  `unstable-projection-gate-and-clause-disposition`. The atom **names it and
  defers**; the design of what replaces polling is HS-P0011's.
- **PS-28 is `undetermined`, and the atom says so rather than resolving it.** The
  rule asserts the checkpoint *"sits at the last good position"*, a phrase it does
  not define. Under *last applied event* every chunked runner fails while
  satisfying the `MUST`; under *last committed position* the pairing is sound. What
  resolves it is defining the phrase when the rule is written — HS-P0011's.

Worth carrying into the atom's summary, because it changes how the next projection
rule is written: PS-29's defect is on a rule §4.11's table never touched, so the
sweep's cause is *the rule written to the clause's intent rather than to its
sentence* — a habit, distributed across the family, not one table's artefact.

## Provisional halves, each with falsifier and phase

- **PS-27**: falsified *"if no projection ever writes a skip record, i.e. if
  'record' turns out to mean 'log a warning'"*; evaluated at the typed-layer phase
  exit against the Kestrel Motor shred case.
- **PS-30**: falsified if the fan-out runner is not built, *"which is decided by
  whether the poll cost of N independent reads is real. That is a benchmark, not an
  assertion, and the workspace has no benchmark harness"*; owned by the typed-layer
  phase.

## Rejected alternatives the atom must name, each with its reason

- a `SkipPolicy` on the runner — forces one wrong answer onto one of two
  projections over the same log;
- a port-level `skip` method — it is `commit` with a different name, and it gives
  the store an opinion about the domain;
- a port-level `CommitError::ApplyFailed` — `apply` is the projection's, and the
  failure never reaches `commit`;
- designing `PumpError` here — the most tempting rejection, since the shape is
  already written down and could be transcribed; transcribing it would make this
  the decision HS-P0011 has to supersede on its first day;
- deleting `rollback` because a buffered batch can be dropped — it converts every
  `AssertUnwindSafe` in a fan-out runner into a lie, and Rust has no async `Drop`;
- repairing PS-29 here.

## Open questions

**None is resolved.** Nothing under `.kb/open-questions/` is deleted.

## Map rows

- `.kb/maps/decision-map.md` — one row: ADR-0019, atom link, title, status,
  phase 6, supersession `—`.
- `.kb/maps/open-questions-index.md` — unchanged by this document.

## Not proposed

No change under `crates/**` or `spec/SPECIFICATION.md`; no maturity marker moved;
no `[FROZEN]` clause line-edited; no runner error type, policy enum or supervisor
designed.
