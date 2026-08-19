---
item: HS-I0006
stage: decomposition
created: 2026-08-12T03:22:51.597Z
updated: 2026-08-12T03:22:51.597Z
template_sig: 0ddf7230
rendered_sig: d672e9aa
---

# Decomposition — From Contract to Published Library

Ten projects, approved at the decomposition gate on 2026-08-12. The portfolio is
MECE: every BR-01…BR-17, AC-01…AC-15, DoD 1…16 and DT-1…DT-8 has exactly one
owning project, and every project owns at least one requirement *and* at least
one Definition-of-Done scenario.

The ground here was once hand-split into three initiatives (HS-I0002/3/4) and
reverted wholesale at `0269720`. This decomposition was re-derived from the
charter, the intake brief and `RUNBOOK.md`; those bodies were not reused.

## Children

| Child | Slug | Rationale |
| ----- | ---- | --------- |
| HS-P0010 | `projection-store-freeze` | The port carrying the largest provisional block (≈17 PS-* clauses, PS-2 a single gate under thirteen rows) has its invariant enforced by a doc comment and nothing else. Everything that consumes a projection port waits on this, so it is the substrate root. |
| HS-P0011 | `typed-layer-and-alpha-release` | A contract is discovered by use, and today the installable crate is a glob re-export with no consumer. This puts a real one above the facade and ships `0.2.0-alpha.1` — which is also the only way the semver baseline that `publication-and-positioning` diffs against comes to exist. |
| HS-P0012 | `sqlite-durable-store` | The durability and handle-multiplicity far ends. First fixture in the workspace to hand out two real handles onto one backing store rather than refcount clones of one in-process object. |
| HS-P0013 | `cloudflare-durable-object-store` | The `!Send` flavour is carried at real cost (ADR-0001) and has never been run. This executes every rule under `workerd` on `wasm32` inside the gate, not as a separately maintained subset. No blockers — it can start on day one. |
| HS-P0014 | `postgres-and-neon-stores` | The two far ends that disagree with the port: positions allocated outside the transaction, and no connection / no interactive transaction / no cursor. Without these the frozen `EventStore` contract is frozen against one storage shape wearing four hats. |
| HS-P0015 | `ladybug-projection-store` | A batch shape structurally unlike the ones that froze the port, and the written verdict on whether the freeze held. A verdict of "it did not hold" is a result, not a failure. |
| HS-P0016 | `publication-and-positioning` | Where private opinions become promises that cannot be withdrawn: the MSRV, the public surface, the clause maturities, the compliance claim, and what the crate says on first contact. Ships `0.2.0`. |
| HS-P0017 | `replication-identity-and-ingest` | The first of the two silences. A position is a statement about one store's log; what it means across a boundary needs a decision or an explicit reasoned refusal, with a suite and two unlike peers behind it. |
| HS-P0018 | `retention-and-incomplete-logs` | The second silence, and the completeness axis — the one with nothing at either end today. Answer constrained at the gate; see **Sequencing**. |
| HS-P0019 | `closeout-and-durable-audience` | **Terminal.** The only project wired to `verify.e2e` (`cargo xtask ci`, the whole gate), and the only place DoD 1–15 can be re-observed *as a set* on the assembled library from a clean checkout rather than as sixteen separately-remembered ticks. Also promotes the personas into the empty `.kb/product/` layer, through the ingest path. |

### Scope seams — what each project explicitly does not own

Stated so nobody has to guess at the boundary. Each exclusion names the sibling
that owns it.

- **`projection-store-freeze`** does not own the graph-shaped third batch shape or the freeze verdict (`ladybug-projection-store`); the `Projection` trait, the runner or PS-33's falsifier (`typed-layer-and-alpha-release`); any shipped SQL projection adapter (`sqlite-durable-store`); or the PS-3 verdict on whether the port ships behind `unstable-projection` (`publication-and-positioning` — this project supplies the evidence, not the verdict).
- **`typed-layer-and-alpha-release`** does not own the MSRV promise, the semver diff verdict, the clause-ledger audit or registry presentation (`publication-and-positioning`); any storage adapter (`sqlite-durable-store`, `cloudflare-durable-object-store`, `postgres-and-neon-stores`); or the projection freeze (`projection-store-freeze` — it consumes the frozen port).
- **`sqlite-durable-store`** does not own the non-serialising or no-cursor far ends (`postgres-and-neon-stores`), the `wasm32` far end (`cloudflare-durable-object-store`), or whether adapter crates are published (`publication-and-positioning`).
- **`cloudflare-durable-object-store`** does not own the projection-suite capability-reporting *policy* (`projection-store-freeze` — it is the first adapter to exercise it), reopening the tail seam (post-0.1, outside this initiative), or publishing its own crate (`publication-and-positioning`).
- **`postgres-and-neon-stores`** does not own the durability and reopen far ends (`sqlite-durable-store`); whether ES-10 should have been per-boundary rather than global (settled in `happenstance-core` at phase 4 — reopening it is a new decision atom and a re-plan); or the completeness axis (`retention-and-incomplete-logs`).
- **`ladybug-projection-store`** does not own the freeze itself (`projection-store-freeze`), acting on a "did not hold" verdict by amending a published surface (a new decision atom and a re-plan), or any event store — Ladybug is a projection store only.
- **`publication-and-positioning`** does not own creating the semver baseline (`typed-layer-and-alpha-release` did that), publishing `happenstance-sync` or `happenstance-sync-testkit` (out of this release train per the charter), any `1.0` or post-1.0 commitment, or the two silences (`replication-identity-and-ingest`, `retention-and-incomplete-logs`).
- **`replication-identity-and-ingest`** does not own publishing either sync crate, what a store may forget (`retention-and-incomplete-logs`), or a DCB wire-interoperability bridge (deferred on the stronger reason that the DCB specification publishes no wire format at all).
- **`retention-and-incomplete-logs`** does not own replication identity or the merge rule (`replication-identity-and-ingest`), crypto-shredding / lawful-deletion tooling / a retention policy engine (not in the charter at any grain), or amending a published `[FROZEN]` clause (see the constraint in **Sequencing**).
- **`closeout-and-durable-audience`** does not own any code, adapter or release — every sibling above.

## Traceability matrix

**O** = owns · **e** = exercises or contributes, does not own.

| ID | freeze | typed | sqlite | cf | pg+neon | ladybug | publication | replication | retention | closeout |
|---|---|---|---|---|---|---|---|---|---|---|
| BR-01 contract exercised by a consumer | | **O** | | | | | | | | |
| BR-02 suite green vs ≥3 disagreeing impls | | | **O**¹ | **O**² | **O**³ | e | e | | | |
| BR-03 each counted impl passes, not a skeleton | | | **O** | **O** | **O** | **O** | | | | e |
| BR-04 `ProjectionStore` frozen on an enforced invariant, re-tested | **O**ᶠ | | | | | **O**ʳ | | | | |
| BR-05 releases at `0.2.0-alpha.1` and `0.2.0` | | **O**ᵅ | | | | | **O**⁰ | | | |
| BR-06 provisional-clause audit at publish | | | | | | | **O** | | | |
| BR-07 surface diffed vs published baseline | | e | | | | | **O** | | | |
| BR-08 MSRV becomes a promise | | | | | | | **O** | | | |
| BR-09 registry-facing completeness | | e | | | | | **O** | | | |
| BR-10 replication identity answered | | | | | | | | **O** | | |
| BR-11 incomplete-log semantics answered | | | | | | | | | **O** | |
| BR-12 `!Send` exercised end to end | | e | e | **O** | e | | e | e | e | e |
| BR-13 declined guarantee reported with a reason | **O** | | e | e | e | e | | | | |
| BR-14 positions/gaps promise findable | | | | | | | **O** | | | |
| BR-15 each answer is a decision atom | **O**ᵃ | **O**ᵃ | **O**ᵃ | **O**ᵃ | **O**ᵃ | **O**ᵃ | **O**ᵃ | **O**ᵃ | **O**ᵃ | **O**ᵃᵘᵈ |
| BR-16 personas promoted at closeout | | | | | | | | | | **O** |
| BR-17 positioning vs nearest live peers | | | | | | | **O** | | | |
| AC-01 model a boundary before a database | | **O** | | | | | | | | e |
| AC-02 compiler protects the domain | | **O** | | | | | | | | e |
| AC-03 install from the registry and it works | | e | | | | | **O** | | | e |
| AC-04 adapter author is told when finished | **O** | | e | e | e | e | | e | | |
| AC-05 declined capability reports its reason | **O** | | e | e | e | e | | | | |
| AC-06 storage shape not quietly assumed | | | e | e | **O** | | | | | |
| AC-07 edge developer keeps their runtime | | | | **O** | | | | | | e |
| AC-08 evaluator can check the claim | | | e | e | e | | **O** | | | |
| AC-09 positions/gaps readable before depending | | | | | | | **O** | | | |
| AC-10 strength of each promise is legible | | | | | | | **O** | | | |
| AC-11 build not broken by surprise | | | | | | | **O** | | | |
| AC-12 which compiler, and why it moved | | | | | | | **O** | | | |
| AC-13 no guessing at a store boundary | | | | | | | | **O** | | |
| AC-14 no guessing at a log with holes | | | | | | | | | **O** | |
| AC-15 next initiative inherits the audience | | | | | | | | | | **O** |
| DoD 1 worked example runs e2e | | **O** | | | | | | | | e |
| DoD 2 compile-fail case green | | **O** | | | | | | | | e |
| DoD 3 durable store passes for real | | | **O** | | | | | | | e |
| DoD 4 constrained-runtime store passes on its target | | | | **O** | | | | | | e |
| DoD 5 non-serialising store passes, cost measured | | | | | **O** | | | | | e |
| DoD 6 no-connection/no-cursor store passes | | | | | **O** | | | | | e |
| DoD 7 projection suite discriminates | **O** | | e | | | e | | | | e |
| DoD 8 freeze verdict written | | | | | | **O** | | | | e |
| DoD 9 a stranger can install it | | | | | | | **O** | | | e |
| DoD 10 the published crate looks finished | | | | | | | **O** | | | e |
| DoD 11 the release is diffed, not asserted | | | | | | | **O** | | | e |
| DoD 12 clause ledger audited at publish | | | | | | | **O** | | | e |
| DoD 13 gate green on the assembled whole | | | | | | | e | | | **O** |
| DoD 14 replication has an answer on disk | | | | | | | | **O** | | e |
| DoD 15 incomplete logs have an answer on disk | | | | | | | | | **O** | e |
| DoD 16 the audience is durable | | | | | | | | | | **O** |
| DT-1 which claim leads on first contact | | | | | | | **O** | | | |
| DT-2 ceremony before a boundary is checked | | **O** | | | | | | | | |
| DT-3 how a consumer learns a guarantee is absent | **O** | | | | | | | | | |
| DT-4 where the positions/gaps promise is stated | | | | | | | **O** | | | |
| DT-5 is the clause vocabulary published | | | | | | | **O** | | | |
| DT-6 explicit comparison to peers | | | | | | | **O** | | | |
| DT-7 how a reader is told the log is incomplete | | | | | | | | | **O** | |
| DT-8 whose adapter-author bar is held | **O** | | | | | | | | | |

¹ the durable store · ² the constrained-runtime store · ³ the non-serialising store **and** the no-cursor store.

ᵃ BR-15 is a **standard of work applied inside every project**, not a work item one
project holds. Each project writes its own records from the runbook's ADR queue
(`RUNBOOK.md:262-284`): 0017–0019 / 0020–0021 / 0022 / 0023 / 0024 / 0025 / — /
0026–0027 / 0028. `closeout-and-durable-audience` owns only the *audit* that they
exist, name the alternatives that lost, and that the matching open-question atoms
are **resolved rather than deleted**.

**Gaps:** none. **Scope creep:** none — `closeout-and-durable-audience` is the
thinnest project and still owns DoD 13, DoD 16, AC-15, BR-16 and exit criteria 7–8.

**Mutually exclusive.** The only shared surfaces are declared and one-directional:
`projection-store-freeze` writes the projection suite that four projects run;
`typed-layer-and-alpha-release` creates the baseline `publication-and-positioning`
diffs against; adapters exercise a capability-declension policy they do not own.

**Collectively exhaustive.** The ten projects cover `RUNBOOK.md` phases 6, 7, 8, 9,
10, 11, 12, 13 and 14 with no phase unclaimed and none claimed twice, plus the two
release events and the closeout obligation that no phase carries. Nothing in the
charter's *In scope* list is unassigned; nothing assigned falls in its *Out of
scope* list.

## Sequencing

### Dependency DAG

| Project | `blocked_by` | `blocks` |
|---|---|---|
| `projection-store-freeze` | — | typed-layer, postgres-and-neon, ladybug |
| `cloudflare-durable-object-store` | — | publication |
| `typed-layer-and-alpha-release` | projection-store-freeze | sqlite, publication |
| `sqlite-durable-store` | projection-store-freeze, typed-layer | publication |
| `postgres-and-neon-stores` | projection-store-freeze | publication |
| `ladybug-projection-store` | projection-store-freeze | publication |
| `publication-and-positioning` | typed-layer, sqlite, cloudflare, postgres-and-neon, ladybug | replication |
| `replication-identity-and-ingest` | publication | retention |
| `retention-and-incomplete-logs` | replication | closeout |
| `closeout-and-durable-audience` | retention | — |

**Acyclic.** Two roots (`projection-store-freeze`, `cloudflare-durable-object-store`),
one sink (`closeout-and-durable-audience`). The rank assignment

```
0: {projection-store-freeze, cloudflare-durable-object-store}
1: {typed-layer-and-alpha-release, postgres-and-neon-stores, ladybug-projection-store}
2: {sqlite-durable-store}
3: {publication-and-positioning}
4: {replication-identity-and-ingest}
5: {retention-and-incomplete-logs}
6: {closeout-and-durable-audience}
```

gives every edge a strictly increasing rank, which is a topological order and
therefore a proof of acyclicity.

**Substrate ahead of its consumers.** `projection-store-freeze` is the substrate
project and is a root — nothing above it depends on a projection port owned
outside this initiative. `typed-layer-and-alpha-release` is substrate for
`publication-and-positioning`, since it creates the semver baseline that project's
entire instrument depends on, and is sequenced before it. No project assumes a
store, a suite or a baseline that is not an owned, earlier project here.

### Merge order

1. `projection-store-freeze`
2. `typed-layer-and-alpha-release` — ships **`0.2.0-alpha.1`**
3. `sqlite-durable-store`
4. `cloudflare-durable-object-store`
5. `postgres-and-neon-stores`
6. `ladybug-projection-store`
7. `publication-and-positioning` — ships **`0.2.0`**
8. `replication-identity-and-ingest`
9. `retention-and-incomplete-logs`
10. `closeout-and-durable-audience`

Positions 4, 5 and 6 are mutually unordered and may be interleaved or run
concurrently; position 4 has no blockers at all and can start on day one alongside
position 1. Positions 1→2→3→7→8→9→10 are the serial trunk.

### Provenance against `RUNBOOK.md:244-258` (a binding input)

- **6 → 7** (the projection port before the runner that drives it) and **7 → 8**
  — *"the typed layer is the consumer that discovers contract defects, and
  discovering them after the flagship adapter is written is the sequence this plan
  exists to avoid"* — both carried verbatim.
- **6 → 10** and **6 → 11** — the runbook's own table.
- **4 → 8/9/10** (`EventId` and `recorded_at` are migration-1 columns in every
  store) is already discharged: phase 4 is `done`.
- Phases 9, 10 and 11 touch disjoint crates and parallelise freely
  (`RUNBOOK.md:239-242`); the DAG reflects that — none of them blocks another.
- **12 → 13 → 14** — the runbook's table. Phase 13's other dependencies (5, 8, 9,
  10) are all satisfied by rank 3.

### Decisions taken at the gate

Four forks the charter does not settle, decided by the repository owner on
2026-08-12. Each is recorded here because the DAG is a consequence of it.

1. **`0.2.0` waits for all four adapter projects**, not for SQLite alone. The
   runbook makes phase 12 depend on 7 and 8 only; this deliberately adds
   `cloudflare`, `postgres-and-neon` and `ladybug` as blockers of
   `publication-and-positioning`, because the charter states the target as *"`0.2.0`
   after the adapters"*, AC-08 requires the published compliance claim to name
   **which** implementations it was checked against, and the risk register's stated
   mitigation is that the freeze is not called earned until a structurally unlike
   batch shape has passed. Publishing after SQLite alone would publish a
   one-adapter claim. Cost accepted: roughly 25–30 runbook-days between the alpha
   and `0.2.0`. The rejected alternative — publish after SQLite with the projection
   port behind `unstable-projection` (PS-3 exists precisely for this) and land the
   rest in `0.2.x` — is defensible and makes a weaker public claim.
2. **Postgres and Neon are one project.** Neon *is* Postgres over one-shot HTTP: it
   inherits migration 1, the tag storage and the append-condition SQL, and both are
   settled by one ADR pass (0024). Splitting them would create a horizontal seam
   (schema below, transport above) rather than a vertical one. It is the largest
   non-trunk item at ~11 runbook-days and is the most reasonable candidate for a
   split if project grain runs long.
3. **Closeout stays a project.** It is the only home for the terminal
   `verify.e2e: cargo xtask ci` grain that `.redkiln/config.yaml` reserves for
   exactly one project, and DoD 13/16, AC-15 and BR-16 get an owner with an id.
   Persona promotion still runs through the ingest path, never hand-authored.
   Rejected: dissolving it into the initiative's own closeout stage, which would
   push DoD 13 and the terminal grain onto `publication-and-positioning` and leave
   DoD 16 an initiative-stage obligation with no project id.
4. **Retention's answer is constrained to what needs no published-surface change.**
   ES-39 and ES-40 are `EventStore` clauses owned by phase 14, which lands *after*
   `0.2.0`. If `retention-and-incomplete-logs` answered with a decision rather than
   a reasoned refusal, that would be a change to a published port — under 0.x a
   minor bump, meaning a `0.3.0` this initiative's exit criteria do not contemplate.
   The runbook already states that an explicit written refusal is a legitimate
   answer, so the constraint costs nothing the charter wanted. **This must be
   written into that project's architecture brief as a stated constraint, not
   carried as folklore.** Rejected: accepting a `0.3.0` outside the exit criteria;
   and pulling retention ahead of publication, which is the largest re-wire of the
   DAG and pushes `0.2.0` out furthest.

### Warranted briefs

| Project | architecture | ux | testing | deployment |
|---|---|---|---|---|
| `projection-store-freeze` | ✅ | ✅ | ✅ | |
| `typed-layer-and-alpha-release` | ✅ | ✅ | ✅ | ✅ |
| `sqlite-durable-store` | ✅ | | ✅ | |
| `cloudflare-durable-object-store` | ✅ | | ✅ | ✅ |
| `postgres-and-neon-stores` | ✅ | | ✅ | ✅ |
| `ladybug-projection-store` | ✅ | | ✅ | |
| `publication-and-positioning` | | ✅ | ✅ | ✅ |
| `replication-identity-and-ingest` | ✅ | | ✅ | |
| `retention-and-incomplete-logs` | ✅ | | ✅ | |
| `closeout-and-durable-audience` | | | ✅ | |

`userFacing` is false and there is no screen anywhere in this initiative, so `ux`
is warranted only where there is a genuine developer-facing surface and is
deliberately absent elsewhere rather than stubbed:

- `projection-store-freeze` — `ProjectionStore`, its `Batch` vocabulary and `reset`
  are public API a caller writes against, and DT-3 is literally *what does the
  suite's output say to a human*.
- `typed-layer-and-alpha-release` — this is the API a `cargo add` user meets. It is
  the primary `ux` brief in the portfolio.
- `publication-and-positioning` — the evaluator's surface: the crate landing page,
  the README and the docs.rs presentation, which is the only "screen" this library
  has. It carries **no** `architecture` brief, because it makes promises about a
  surface it does not design.

`deployment` is warranted only where infrastructure that does not exist today has
to: a `workerd` runner in CI (`cloudflare`), a live Postgres **and** a live Neon
HTTP endpoint (`postgres-and-neon` — the Neon one cannot be faked without
destroying the axis it exists to test), and the two real crates.io release events
(`typed-layer`, `publication`).

`closeout-and-durable-audience` carries **one** brief deliberately. It verifies and
records; it designs nothing. Forcing `architecture` or `ux` onto it would produce
exactly the stub the right-sizing rule exists to prevent.

### Carried into the briefs, not settled here

Four items that are real but belong to a project's own brief rather than to the
decomposition. Recorded so none of them is settled silently in passing.

- **The DoD 7 watched edge.** `projection-store-freeze` owes *two unlike batch
  shapes passing* while the only SQL projection store in the tree ships from
  `sqlite-durable-store`, two positions later in merge order. Either the second
  shape is built inside the testkit — which is what the `MemoryFixture` precedent
  suggests — or this is a latent 6→8 inversion. It must be settled in
  `projection-store-freeze`'s architecture brief. It is **not** a DAG edge today
  because nothing in the runbook makes it one.
- **Which crates publish at `0.2.0`.** `CLAUDE.md`'s `cargo package --list`
  assertion covers three crates, RUNBOOK phase 0 reserved three names, and the
  adapters are `publish = false` today. Working default: **three crates only**,
  since AC-03 and DoD 9 name the installable crate in the singular. But the charter
  never says so, and the evaluator persona will look for a SQLite adapter on the
  landing page. `publication-and-positioning`'s deployment brief decides it; if the
  answer is more than three, each adapter project owes a name reservation and a
  README.
- **Is the evaluator its own persona**, or an earlier stage of the application
  author's journey? Carried unresolved from the charter. It changes what
  `closeout-and-durable-audience` promotes and whether DT-1's *two entry points, one
  per audience* option is even coherent. Decide it in that project's brief, not at
  atom-authoring time.
- **DT-8's cut depth.** If the adapter-author bar is held for an *outside* author
  within this timeframe, `projection-store-freeze` grows a documented testkit
  extension surface — and AC-04/AC-05 have to hold against implementations nobody
  in this repository wrote.

### Ordering that must not be reordered

Carried forward from `RUNBOOK.md:244-258` and the intake brief's constraints, so
that a later re-plan cannot lose it:

- **Phase 4 before 8/9/10** — `EventId` and `recorded_at` are migration-1 columns in
  every store. Already discharged; phase 4 is `done`.
- **Phase 7 before phase 8** — the typed layer is what discovers contract defects,
  and doing it after the flagship adapter is precisely the sequence the plan exists
  to avoid. This is the `typed-layer → sqlite` edge and it is not negotiable.

## Story map

Empty by design. This is a **standard**-tier initiative, so each project's story
map lives in its own `_storymap.md`, authored by `plan-briefs`. This section is
filled in only on a `project-lite` item, which has no `storymap` stage.
