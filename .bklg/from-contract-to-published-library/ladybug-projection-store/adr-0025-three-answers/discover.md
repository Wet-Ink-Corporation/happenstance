---
item: HS-S0075
stage: discover
created: 2026-08-12T13:02:44.859Z
updated: 2026-08-12T13:02:44.859Z
template_sig: 86ce4036
rendered_sig: 5fbd8a7e
---

# Discover — ADR-0025: checkpoint placement, mutation vocabulary, blocking bridge

## Signal Ledger

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line — author ADR-0025 through `.kb/_intake/`: checkpoint placement, graph-mutation vocabulary, blocking-API bridge, each question with its losers named, plus the long-form record and the decision-map entry | `_storymap.md:41` | Three answers, three sets of losers, two files and a map edit. Authoring route is fixed and is not hand-writing. |
| **AC-002** — ADR-0025 is accepted and answers three questions, each with its losers named; `redkiln validate --kb` passes and the atom is reachable from `.kb/maps/decision-map.md` | `project.md:185-188` | Half of the AC is mechanical (validation, reachability) and half is a review-tier read of whether each question actually names what lost. |
| **DR-6** — the losers are stated **per question**, not once for the whole record; an accepted atom is immutable, which is what makes it the only durable record of a rejected option | `project.md:158-161` | A record that lists three rejected options in one closing paragraph satisfies a skim and fails DR-6. |
| **AC-A03** — each question is answered against the constraint already on record; re-deriving a constraint the skeleton recorded, or answering a question the port has closed, is a defect in the ADR | `_decomposition.md:46-50` | The ADR's job is to decide the residue, not to restate `projection_store.rs`'s module docs. |
| Q1 is nearly settled and the argument is already written: the checkpoint as a node property is *"the only way to satisfy the port's transactional invariant"*, because it is what keeps the write inside the same `BEGIN TRANSACTION` | `_decomposition.md:266-274`; `crates/happenstance-ladybug/src/lib.rs:53-58` | The loser to name is the sidecar (a file or table outside the graph) and it loses to PS-1, not to taste. The genuinely open residue is the `INT64`/`NonZeroU64` narrowing and whether the checkpoint node is per-`ProjectionId` or one node with a property per id. |
| Q2's binding constraint is ADR-0008 — a provided body cannot hold the `Batch` GAT across a suspension point under any remedy tried — which rules out shapes before they are weighed; `GraphStatement` is the raw-Cypher answer and is deliberately *the port's* data point for PS-9 and PS-11 | `_decomposition.md:276-291`; `.kb/decisions/0008-one-derivation-for-both-ports.md`; `crates/happenstance-ladybug/src/lib.rs:62-66` | Losers: a typed builder (what it buys, and what it costs a projection needing Cypher the builder does not model), and bound-on-`Batch` versus method-on-port versus second-associated-type. Whatever is chosen must stay compatible with ADR-0007's indicative `Projection::apply`. |
| Q3 — the skeleton records that the owned-`Database` layout leaves both options open and that `spawn_blocking` *"is not used, because reaching for it would put a tokio dependency in a runtime-agnostic adapter"* | `crates/happenstance-ladybug/src/projection_store.rs:39-47`; `_decomposition.md:293-310` | Three candidates, each with a real cost: a runtime-gated `spawn_blocking` feature, blocking the executor thread and documenting it, or a blocking-only adapter. |
| Two hard frames on Q3 — `#[async_trait]` is never an option (it injects `+ Send` and makes wasm32 impossible), and the testkit already ships a `__emit_blocking` harness needing no async runtime, so *"the suite needs tokio"* is not an argument for any candidate | `.kb/decisions/0001-async-port-flavours.md`; `crates/happenstance-testkit/src/lib.rs:63-67` | The design space is smaller than it looks, and one common justification for the `spawn_blocking` feature is already refuted. `Cargo.toml`'s existing tokio dev-dependency is not a precedent for a normal one. |
| The `Send` flavour is **not** open — it is chosen on the evidence that `lbug`'s `Database` and `Connection` are `Send + Sync`; BR-12's `!Send` proof belongs to `cloudflare-durable-object-store` | `crates/happenstance-ladybug/src/projection_store.rs:252-257`; `project.md:120-124` | An ADR that re-opens the flavour choice is answering a question the evidence closed, which AC-A03 calls a defect. |
| **M8** — the atom carries valid `KbFrontmatter` **and** a long-form record under `references/adr/`; atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`, never by hand, and the commit where hand-writing was reverted is named | `_decomposition.md:204-209`; `CLAUDE.md`, *Where the work lives* | Link the atom, cite the record by `file:line`. The two-places rule is not optional and hand-authoring is the failure mode with a precedent. |
| ADR-0025 does not exist; `.kb/decisions/` holds 0001–0016 and 0029, and `RUNBOOK.md:304` reserves **0025** for this project with wording matching AC-002 verbatim | `_grounding.md:32-37`; `RUNBOOK.md:304` | The number is free and there is nothing to disambiguate. |
| `depends_on: preflight-and-unlike-axes` — supplies which port actually merged (`type Batch;` or the surviving GAT) and which of the two write seams shipped | `_storymap.md:41`, `:127-132`; `_decomposition.md:104-114` | Q2 cannot be answered before the seam is known: a vocabulary decision taken against the wrong seam is a bill that arrives in `typed-layer-and-alpha-release`. It also supplies the committed axes, which the ADR must not contradict. |
| Slice rationale — both stories in `preflight-and-decisions` land real in-tree artefacts the `real-adapter` and `freeze-verdict` slices consume by name; neither is a double or a placeholder | `_storymap.md:53-59` | The atom is consumed by `real-lbug-driver-swap`, which rewrites `lib.rs:51-66`'s open-decisions list into a link to it. |

## Questions

**How does `lbug`'s blocking API meet a non-blocking port? — this story's Q3, and
the answer is deferred to `spec`/the ADR itself, but the *decision procedure* is
settled here.** Three candidates, each costed honestly:

- *A runtime-gated `spawn_blocking` feature.* Buys a genuinely non-blocking
  `commit` on a multi-threaded tokio runtime. Costs a tokio dependency in an
  adapter whose whole design premise is runtime-agnosticism, and the cost is not
  softened by making the feature optional: the gate runs `clippy` and `tests` at
  `--workspace --all-features` and `cargo hack --feature-powerset` above them
  (`_decomposition.md:381-387`), so the feature is always on in CI and the
  off-configuration is the one nobody exercises.
- *Block the executor thread and document it.* Correct under the testkit's
  `__emit_blocking` emitter and under a single-threaded runner. A latent hazard
  under a multi-threaded runtime, where a multi-second C++-backed commit stalls
  every other task on that worker. Costs nothing to build and everything to
  discover later.
- *A blocking-only adapter.* Honest, and **self-defeating for this project**: an
  adapter that does not implement `SendProjectionStore` cannot be handed to
  `projection_store_conformance!` at all, and AC-004's entire premise is that the
  suite runs against this store. Naming it as a loser is required; choosing it
  would forfeit the run.

Whatever is chosen, three constraints bind the answer and are not re-litigated:
no `#[async_trait]` (ADR-0001), no tokio in the non-optional dependency graph
without the ADR saying so in as many words, and the adapter must stay runnable
under an emitter that has no async runtime.

**What counts as "structurally unlike"? — not this story's, and deliberately not
touched.** It is on disk from `preflight-and-unlike-axes` (HS-S0074) before this
story opens (`project.md:180-184`). The ADR *reads* the axes and must not
contradict them; if an answer here would change an axis — for instance if Q3's
answer altered the "synchronous driver behind an async port" characterisation —
that is a supersession of the axes document under M9's lifecycle
(`references/evaluation/README.md:1-13`), recorded as such, never a quiet edit.

**Is the per-`ProjectionId` checkpoint node question ADR-0025's to settle? —
partly, and the boundary is explicit.** Whether *this adapter* uses one node per
id or one node with a property per id is ADR-0025's. Whether the **port** permits
one commit to advance two ids is PS-23 (`spec/SPECIFICATION.md:5317`), which is
`projection-store-freeze`'s; this adapter is a data point for it
(`_decomposition.md:270-274`). Deferred, with the owner named.

**Does Q2 need the write seam settled first? — yes, and that is the `depends_on`
edge.** Answered by reading what HS-P0010 shipped, not by guessing between PS-11's
contract-crate `ProjectionProbe` and the fixture-level `write_probe`. If neither
can express a replayable parameterised Cypher statement, ADR-0025 does not invent
a third — it records the finding and the verdict carries it
(`_decomposition.md:104-114`).

**Deferred to `spec`:** the Cypher schema itself — label names, property names,
indexes — which `_decomposition.md:411-413` leaves to the implementer with only
transaction membership fixed; and whether `GraphStatement::parameters`'
`Vec<(Box<str>, Value)>` survives contact with `lbug`'s real value type, which is
answered by compiling in `real-lbug-driver-swap` (HS-S0076), not by argument here.

## Decision

Three decisions that the skeleton deliberately left open are about to be made by
whoever fills in the bodies, and if they are made in code they will be made
invisibly and irreversibly — so this story makes them in a record instead, with
each question's rejected alternatives named against it rather than pooled at the
end. The spec will cover ADR-0025 as a `.kb/decisions/` atom authored through
`.kb/_intake/` and `/redkiln:kb-ingest` (never by hand), with a long-form record
under `references/adr/` and an entry in `.kb/maps/decision-map.md`, structured so
each of Q1, Q2 and Q3 carries its own losers-and-why: the sidecar checkpoint
losing to PS-1's transactional invariant, the typed builder and the
bound-versus-method-versus-second-associated-type choices losing against ADR-0008's
GAT-across-a-suspension-point finding and ADR-0007's indicative `apply` signature,
and two of the three blocking-bridge candidates losing on stated costs — with the
blocking-only option recorded as forfeiting AC-004's suite run outright. Nothing
`[FROZEN]` is touched and no clause marker or clause sentence is edited: PS-9 and
PS-11 are cited as the clauses this adapter is a data point *for*, and if the ADR's
answers imply a clause should move, that routes to `projection-store-freeze` and,
if it comes to it, a superseding decision atom — never an edit here.

## The wrong implementation

**The blocking bridge hidden inside a private helper.** A
`fn blocking_commit(&self, batch: GraphWriteSet, …) -> Result<(), …>` in
`projection_store.rs` that calls `lbug` synchronously, invoked directly from the
body of the `async fn commit`. There is no `spawn_blocking`, no feature, no tokio
dependency, no documented hazard, and no sentence in ADR-0025 — and everything
passes: the compiler is satisfied, `clippy -D warnings` is satisfied, the suite
runs green under the testkit's `__emit_blocking` emitter, and `cargo xtask ci` is
green end to end. What it produces is an adapter that *appears* to express
something the port cannot deliver: the signature promises a future that yields,
and the body monopolises the thread for the duration of a C++ transaction. The
privacy of the helper is the whole trick — it makes the decision unreviewable by
turning it into an implementation detail, which is exactly the class of decision
DR-6 says an immutable atom exists to hold.

Naming where its falsifier lives matters, because no conformance rule can catch
it: the testkit cannot observe executor starvation from inside a rule, and
`crates/happenstance-testkit/**` is not this project's to edit
(`_decomposition.md:86-88`). The falsifier is an adapter-local test in
`crates/happenstance-ladybug/tests/` — call it
`blocking_bridge::commit_does_not_starve_a_concurrent_task`: a current-thread
tokio runtime, a `commit` in flight, and a second task that must make progress
while it runs. Its **expected outcome is set by ADR-0025 and never the reverse**.
If the ADR chooses `spawn_blocking`, the test asserts the concurrent task
progresses and fails if the helper mutant is introduced. If the ADR chooses
"blocks, and this is documented", the same test is kept as the recorded
demonstration of the hazard — asserting the starvation and citing the ADR — rather
than deleted. A test that is deleted because the answer changed is how a
documented hazard becomes an undocumented one.

**The ADR that names its losers once, at the end.** Three questions, three
answers, and a closing paragraph reading "alternatives considered: a sidecar
checkpoint, a typed builder, a blocking-only adapter." `redkiln validate --kb`
passes, the atom is reachable from the decision map, AC-002's mechanical half is
green — and DR-6 is violated, because no reader can tell which loser lost to which
argument, and the record's one durable job is to preserve exactly that. The guard
is a review-tier read against `_decomposition.md:261-310`'s per-question
constraints, which is why AC-002's content check is explicitly not automatable
(`_decomposition.md:472`).

**The ADR that re-derives what is already recorded.** An atom that argues at length
for the `Send` flavour, or for per-call connections, or for the deferred write set
— all three already settled on evidence in `projection_store.rs:24-47` and
`:252-257`. It reads as thorough, passes every check, and buries the three genuine
residues in material that decides nothing. AC-A03 names this as a defect in the
ADR rather than a stylistic complaint.

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
