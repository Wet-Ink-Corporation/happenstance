# Staged: ADR-0017 — what a projection batch owns, and the seam that is not a write vocabulary

**Staged 2026-08-13** for the next `/redkiln:kb-ingest` wave. **Not an atom.**
Long-form record: [`references/adr/0017-what-a-projection-batch-owns.md`](../../references/adr/0017-what-a-projection-batch-owns.md).

**Wave-id note.** `.kb/_governance/integration-waves/` already holds
`2026-08-10-intake` and `2026-08-10-intake-2`. Give this wave an id colliding with
neither — `2026-08-13-projection-adrs` is free. Stage this document with its two
siblings (`…-adr-0018-reset.md`, `…-adr-0019-apply-failure.md`) in **one** wave:
three waves would produce three audit trails for one decision set and three
chances at an id collision. Drop `.kb/_intake/README.md` at the approval gate — a
README ingested as an atom is a corpus-shaped artefact with no decision in it.

---

## The op: create one `decision` atom

`.kb/decisions/0017-what-a-projection-batch-owns.md`.

**Frontmatter — invent no keys.** `kind: decision`, `authority_tier: decision`,
`adr_id: ADR-0017`, `phase: 6`, `reversibility` per the corpus's existing values,
`status: accepted`.

**On `status`.** Parts of this decision are provisional and `KbFrontmatter`'s
`status` is a closed enum with no *"accepted — provisional"* value. Follow the
convention the 2026-08-10 import already applied: `status: accepted`, with the
qualification **and its falsifier** folded into the first clause of `summary`, and
`supersedes` / `superseded_by` reserved for *full* supersession. Do **not** answer
`kb-open-question-adr-status-vocabulary-001` by acting on it, and leave
`.kb/open-questions/adr-status-vocabulary-exceeds-the-schema.md` unchanged.

**Density budget.** The atom is the ~100-line canonical form. The transcripts, the
five-ingredient ICE minimisation and the full rejected-alternatives argument stay
in the long-form record and are **cited by `file:line`, not copied**. Collapsing
the two would discard the evidence a summary cannot hold; inflating the atom to
carry the transcripts breaks the other way.

## Claims the atom carries

1. **`type Batch;` — owned, no lifetime parameter (PS-5).** The clause rests on
   `error[E0195]` (`references/adapter-shapes.md:186-194`) and on the
   `DefId::expect_local` ICE proving today's port is implementable only by stores
   that outlive every batch lifetime (`:307-365`;
   `crates/happenstance-ladybug/src/live_handle.rs:36-66`). It does **not** rest on
   the `Send` argument, which `live_handle.rs:14-31` already refutes: LadybugDB's
   `Connection` is `Send` *and* `Sync`, so a genuinely borrowed
   `GraphWriteHandle<'a>` binds to `type Batch<'a>` on the `Send` flavour with real
   bodies.
2. **The owned batch does not close the foreign-batch hazard.** Tying the batch to
   the receiver's lifetime was compiled and refuted — a lifetime names a region,
   not an instance. PS-15 stays `[PROVISIONAL]` and is discharged at run time by a
   per-store-instance stamp surfacing as `CommitError::ForeignBatch`.
3. **PS-9 and PS-11 are one split-by-consumer decision, not "the port grows a
   write method".** No universal write vocabulary on `Batch`; a
   `ProjectionProbe: ProjectionStore` in the **contract crate** behind
   `feature = "conformance"`, bare flavour only. Contract crate rather than testkit
   for a coherence reason: an adapter's `tests/` is a different crate where neither
   a testkit trait nor the adapter's type is local, so the impl is rejected by the
   orphan rule (`spec/SPECIFICATION.md:5015-5031`).
4. **Dropping a batch rolls back *and* leaves the store usable (PS-7); `rollback`
   stays on the port (PS-8),** because Rust has no async `Drop`.
5. **`LiveHandleProjectionStore`'s disposition is named: moved to
   `experiments/live-handle-projection-batch/`,** not deleted — it is the only
   compiled evidence against this decision's own §1, and `experiments/` is the
   tree's existing home for a reproducible measurement that is not in the gate.

## Rejected alternatives the atom must name, each with its reason

- a `ProjectionBatch` supertrait carrying `put`/`get` — obliges every store into a
  key-value table and reintroduces at the read-model layer the opaque blob ADR-0003
  confined to event payloads;
- the probe trait in `happenstance-testkit` — rejected by the orphan rule from an
  adapter's own `tests/`, forcing a non-dev dependency on the testkit;
- keeping the GAT — defensible for one driver family, refuted by two
  driver-independent costs (`E0195` on every impl; an ICE for any non-`'static`
  store);
- resting PS-5 on the `Send` argument — already refuted in this workspace;
- a generative brand for the foreign-batch hazard — works, and forbids the batch
  escaping the closure, which defeats the caller the hazard is about;
- tying the batch to the receiver's lifetime — compiled and refuted;
- deleting `LiveHandleProjectionStore`.

## Provisional halves, each with falsifier and phase

- **PS-9 / PS-11**: falsified by a *second generic consumer* — library code
  happenstance itself ships that must write into an unknown adapter's batch. A
  generic dead-letter recorder and a generic counter projection are the candidates.
  Evaluated at the typed-layer phase exit (HS-P0011) by counting consumers.
- **PS-15**: falsified by a zero-cost type-level construction naming an instance
  that composes with `async fn` and permits a batch in a collection. If found, the
  clause is replaced by a compile error, which is strictly better.
- **PS-4 / PS-5**: falsified together by an adapter that can satisfy PS-1 only with
  a handle acquired before the first write; LadybugDB is the named candidate.

## The sweep's finding, and what this atom does with it

`references/evaluation/ps-clause-pairing-sweep.md` returned **isolated**. Two of
its `defective` rows sit inside PS-4 – PS-15 — **PS-8** (the `MUST` is about the
method's existence; the rule tests its behaviour) and **PS-13** (the `MUST` binds a
projection; the rule runs in the adapter suite). Both are `[FROZEN]`, both
corrections change the admitted set, so both are **gaps** and a gap is a new
decision atom's. The atom **names them and defers**; it repairs nothing.

## Resolve — do not delete — one open question

`kb-open-question-projection-batch-no-apply-001`
(`.kb/open-questions/projection-store-batch-has-no-apply-seam.md`).

- Its `status` moves to `withdrawn` or `superseded`; the new decision atom and the
  question are linked through reciprocal `related` edges.
- **Its body is left verbatim.** Per `.kb/open-questions/README.md`: do not rewrite
  a question into its own answer — the value of the record is that it shows the
  state of knowledge on the day the choice was made. If the wave wants to note the
  resolution, append a dated line; do not edit the original paragraphs.
- **Sub-questions 1, 2 and 4 fall to this wave. Sub-question 3 does not** — whether
  closing this retroactively validates ADR-0006's encoding-versus-orchestration
  discriminator belongs with the runner (HS-P0011). Say so explicitly in the
  annotation, so the omission reads as a decision.

## Map rows

- `.kb/maps/decision-map.md` — one row: ADR-0017, atom link, title, status,
  phase 6, supersession column `—`. An atom absent from the map is a `lib.rs`
  export block's missing `pub use`: it validates, and nobody can find it.
- `.kb/maps/open-questions-index.md` — the batch-has-no-apply-seam bullet stays
  listed and gains an annotation naming ADR-0017 as its answer and sub-question 3
  as still open. **Not removed.**
- `.kb/maps/domain-map.md` — a row only if the wave's Maps phase places one.

## Not proposed by this document

No change under `crates/**` or `spec/SPECIFICATION.md`; no maturity marker moved;
no `[FROZEN]` clause line-edited. No resolution of
`kb-open-question-ps-1-no-progress-obligation-001`,
`kb-open-question-ps-19-scope-narrower-001`,
`kb-open-question-cf-40-ownership-001`,
`kb-open-question-projection-id-unvalidated-001`,
`kb-open-question-global-vs-boundary-visibility-001`,
`kb-open-question-post-phase-reconciliation-001` or
`kb-open-question-adr-status-vocabulary-001` — each has an owner and is named in
the long-form record's context. No supersession of ADR-0007: PS-32's correction to
its Context is *owed and recorded*, and performing it is a superseding atom's job,
not this wave's.
