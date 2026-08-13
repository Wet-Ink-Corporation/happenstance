---
item: HS-I0006
stage: intake
created: 2026-08-12T02:35:56.000Z
updated: 2026-08-12T02:35:56.000Z
template_sig: ab516678
rendered_sig: 63ffdb75
---

# Intake Brief — From Contract to Published Library

## Problem

`happenstance` has settled everything a type checker can settle and nothing a
database can. Nine crates exist and 37 `todo!()` bodies are spread across 20 files
in six of them — and a `todo!()` type-checks against *any* signature, so "it
compiles" has so far been evidence of nothing. Every implementation that has
actually passed the conformance suite serialises its writers and assigns positions
under a lock it holds until commit: four adapters, one storage shape wearing four
hats (`RUNBOOK.md:669-671`, `RUNBOOK.md:695-698`). `ProjectionStore` carries the
largest block of provisional clauses, with PS-2 a single gate sitting under
thirteen rows (`RUNBOOK.md:588-610`), and the invariant it exists to protect — that
a read-model write and its checkpoint land in one transaction — is documented on
the trait and enforced by nothing. The crate people will actually install is a
glob re-export (`crates/happenstance/src/lib.rs:75`), so no consumer has ever
discovered a contract defect. And nothing is published, so the MSRV, the public
surface and the semver commitment are each an opinion nobody is relying on.

## Desired Outcome

A library an application author can `cargo add` and rely on, and that an adapter
author can implement against with a bar that tells them when they are done.

We will know by four observable things, not by argument:

1. **The contract has been used.** A typed layer and a worked example exist above
   the facade, and the defects that use discovered are recorded — `trybuild`
   compile-fail coverage on an unhandled event variant is the shape of the proof.
2. **Adapters have passed the suite across shapes that genuinely disagree.** Not
   four instruments at the same end of every axis: at minimum a real durable store,
   a store that does not serialise its writers, and a store with no connection and
   no cursor, each green against `event_store_conformance!`.
3. **A published release whose promises are real** — `0.2.0-alpha.1` after the
   typed layer, `0.2.0` after the adapters, with every provisional clause audited
   against the ledger rather than against prose, and docs.rs green.
4. **Honest written answers on replication and on what a store may forget** — a
   decision or an explicit, reasoned refusal. Silence is not an outcome.

## Constraints

Binding, from `.kb/decisions/` and `CLAUDE.md`. Changing one means a new ADR, not
code written around it.

- **No `#[async_trait]`, ever.** It injects `+ Send` and deletes the `wasm32`
  target. Ports are defined once without a `Send` bound; `trait_variant` derives
  the `Send` flavour. (ADR-0001)
- **No `serde` in `happenstance-core`'s default features.** Payloads stay opaque
  `Bytes`. This constrains the *contract* crate only — `happenstance`, the typed
  layer, is the crate whose whole job is encoding. (ADR-0003, as re-read after
  ADR-0006)
- **`EventStore::read` returns the stream at the top level and is not `async`.**
  Two tests in `memory.rs` assert this and it takes both. (ADR-0001, ADR-0008)
- **An adapter that compiles but has not run the conformance suite is not an
  adapter.** No exceptions, including for this project's own adapters.
- **A rule no adapter can fail is decorative.** Before adding one, name the wrong
  implementation it rejects and put that implementation in the testkit's `tests/`.
- **Never assert on literal position values** in a conformance rule; the
  specification permits gaps.
- **The local-first / edge case is load-bearing, not a nice-to-have.** The `!Send`
  flavour is carried at real cost so a store can live in a Cloudflare Durable
  Object on `wasm32`. Work that quietly drops it has changed the product.
- **Evidence beats argument.** A design is settled against something that compiles
  or against a measurement, and the artefact that proves it must be one that
  *would not exist if the design were wrong*. A green `cargo xtask ci` is a
  precondition for looking at the evidence, never the evidence itself.
- **Where any document and `spec/SPECIFICATION.md` disagree, the specification
  wins.** A `[FROZEN]` clause changes by new decision record, not by edit.
- **Ordering that must not be reordered** (`RUNBOOK.md:244-258`): phase 4 before
  8/9/10, because `EventId` and `recorded_at` are migration-1 columns in every
  store; and **phase 7 before phase 8**, because the typed layer is what discovers
  contract defects and doing it after the flagship adapter is precisely the
  sequence the plan exists to avoid.

**Non-goals for this brief.** How the work decomposes — whether a RUNBOOK phase is
a project, whether several collapse into one, and where the release boundaries
fall — belongs to `/redkiln:plan`, not here. This brief states the problem and the
vision and deliberately proposes no architecture, no tech choices and no design.

## Open Questions

Carried for distillation; several are already atoms under `.kb/open-questions/`
(20 files) and several are rows in the ADR queue at `RUNBOOK.md:262-284`. None is
to be settled in passing.

- **Does ingest re-check the append conditions a writer asserted?** The central
  unanswered question of replication. A `SequencePosition` is a statement about one
  store's log and cannot cross a boundary unchanged.
- **What may a store forget, and how does a reader find out?** Retention, lawful
  deletion and crypto-shredding all produce a log with holes. A reader that quietly
  builds a wrong answer from a truncated log is the failure mode nothing currently
  detects.
- **Is the ES-10 visibility invariant global or per-boundary?** DCB evaluates
  conditions against a boundary; the experiment deliberately refused to settle this
  (`experiments/position-visibility/README.md:402-412`).
- **How does a Postgres adapter buy position visibility?** `nextval()` allocates
  outside the transaction. The measurement exists — only `xid8` +
  `pg_snapshot_xmin` buys it at no throughput cost, two arms cost 16×/30×, and the
  cheap fourth does not buy it at all — but the choice is still owed a decision.
- **Does `ProjectionStore`'s freeze hold once a third batch shape exists?** Phase 6
  freezes it; phase 11 is the first honest test of that freeze and owes a written
  verdict either way.
- **Are hub-and-spoke and peer-to-peer one abstraction or two?** Open in
  `crates/happenstance-sync/src/lib.rs`, along with the shape of the peer port.
- **Which provisional clauses actually get frozen at publish, and which are
  demoted?** The publish gate audits against the ledger, not against prose.

Two accuracy items found during intake, recorded so nobody re-derives them:

- The seed's claim that `grep -rn "ProjectionStore for"` matches nothing is
  **stale**. It matches six sites, all `todo!()`-bodied skeletons — so the seed's
  *argument* stands unchanged, but the probe does not, and `RUNBOOK.md:362` already
  says there are five impls.
- Provisional-clause counts disagree by source: `RUNBOOK.md:22-28` reports 139
  frozen / 49 provisional / 10 deferred, while a raw marker grep over
  `spec/SPECIFICATION.md` returns 303 / 117 / 34. One counts clauses and one counts
  marker occurrences. Not resolved here; distillation should reconcile them before
  any count is quoted as fact.

## Proof artefact

**The conformance suite, green against three stores that disagree with each other
on the axes the contract is most likely to be wrong about — and a published crate
whose provisional clauses were audited against the clause ledger at the moment of
publish.**

That artefact could not exist if the design were wrong, and each half covers what
the other cannot:

- A suite green against `MemoryEventStore` alone proves nothing about the contract,
  because memory gives away durability and concurrency for free. A suite green
  against a store that assigns positions *outside* the transaction, and against one
  with no connection, no interactive transaction and no cursor, cannot be satisfied
  by a contract that assumed a held lock. Those two stores fail to compile — or fail
  the suite — under any port shape that quietly assumed serialised writers.
- A `todo!()` body type-checks against any signature at all, so every skeleton-based
  proof to date has been decorative in exactly the way `RUNBOOK.md:91-100` records.
  Only a *passing implementation* is a far end.
- Publication is what converts each private opinion into a promise that cannot be
  withdrawn. `cargo-semver-checks` against a registry baseline is an instrument that
  did not exist before a baseline existed — and it already found seven major lints
  once, which is why `0.1.0` became `0.2.0` (`RUNBOOK.md:4108-4124`).

Subordinate artefacts, each of which would also not exist if the corresponding
design were wrong: a `trybuild` compile-fail case on an unhandled event variant; an
acknowledged write surviving a process reopen; a concurrency macro green at 64
contenders; every rule green under `workerd` on `wasm32`; and a byte-identical
payload round trip across a replication boundary.

## Clauses

Named as a starting ledger for distillation, not as a settled list. Counts are the
disputed ones recorded above; distillation reconciles them before any is quoted.

- **`ProjectionStore` (PS-*) — `[PROVISIONAL]`, the largest block** (≈17 clauses,
  `RUNBOOK.md:588-606`). **PS-2 is a single gate under thirteen rows**
  (`RUNBOOK.md:608-610`) — it is the clause this initiative most has to get right.
  Freezing these is the substance of the port freeze, and it requires the ADRs
  queued at `RUNBOOK.md:262-284`, not an edit.
- **`SyncPeer` (SY-*) — `[PROVISIONAL]` (≈9) plus five `[DEFERRED]`** (SY-14, SY-18,
  SY-27, SY-28, SY-32). These are the replication clauses; they are deferred on a
  decision, not on effort.
- **`VersionedTransport` / wire (VT-*) — `[PROVISIONAL]` (≈9)**, against a frozen
  wire format with negative controls asserted by name.
- **`EventStore` (ES-*) — `[PROVISIONAL]` (≈8), including ES-41 and ES-42**, which
  `RUNBOOK.md:622-635` records as missing from its own table. **ES-10** is the
  position-visibility clause the Postgres measurement was run for; **ES-39** is
  `[DEFERRED]`.
- **Cloudflare (CF-*) — CF-39 and CF-40 `[PROVISIONAL]`; CF-14 and CF-27
  `[DEFERRED]`.** CF-40's ownership is itself an open question owned by phase 8.
- **Workflow / format (WF-*) — WF-1 `[DEFERRED]`; WF-11**'s human-readable encoding
  is open.
- **Everything `[FROZEN]`** — the `EventStore` contract, the values crossing it and
  the wire format — is out of scope for amendment. If this work needs one changed,
  that is a new ADR and a re-plan, not a line edit. **Where this brief and
  `spec/SPECIFICATION.md` disagree, the specification wins.**

## Research angles

Approved at the intake gate for the Stage B discovery fan-out:

1. `dcb-implementations-and-competing-event-stores`
2. `rust-event-sourcing-crate-landscape-and-sentiment`
3. `conformance-testkit-as-a-shipped-product`
4. `storage-agnostic-ports-that-survived-a-second-database`
5. `position-visibility-in-postgres-and-distributed-logs`
6. `replication-and-position-identity-across-store-boundaries`
7. `retention-deletion-and-incomplete-log-semantics`
8. `rust-publication-semver-and-msrv-discipline`
9. `wasm32-edge-and-non-send-async-ecosystem`

`userFacing` is **false**: this is a library with no screen. The standing
`interaction-pattern-prior-art` angle is deliberately not appended — the public API
shape is still reviewed, by the planning design stage, which is a different gate.

Tier is **standard**, scored 12/12 against the six-axis rubric with every axis at 2
and each score carrying cited evidence.

## Source paths

- `D:\repos\happenstance\references\seeds\remaining-runway.md`

## Gate: Intake

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/intake.md` — where each box's rationale is
written — and will not leave `intake` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem and desired outcome are stated.
- [x] Constraints and non-goals are recorded.
- [x] Open questions are captured for distillation.
- [x] The proof artefact is named, and it would not exist if the design were wrong.
- [x] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [x] The clauses this work discharges or amends are listed by id.
- [x] Where this brief and `SPECIFICATION.md` disagree, the specification wins.

**On the port-freeze box.** `ProjectionStore` is the port this initiative freezes, and the axis it is most likely to be wrong about is **batch shape and transactional seam** — every implementation so far assumes it can hold one transaction open across a read-model write and its checkpoint write. The far ends that disagree already exist in the workspace as skeletons: `happenstance-ladybug` is a graph projection store with a structurally unlike batch, and `happenstance-neon` has no connection, no interactive transaction and no cursor at all, so it cannot hold a transaction open by construction. Neither has been built, which is exactly why the freeze is not yet earned — and `RUNBOOK.md` puts the written verdict on whether the freeze held at phase 11, after the third batch shape exists, rather than at the freeze itself.

For `EventStore`, already frozen, the same axis is **who assigns positions and when**: `happenstance-postgres` (positions allocated outside the transaction) is the standing far end, and it has not been built either.
