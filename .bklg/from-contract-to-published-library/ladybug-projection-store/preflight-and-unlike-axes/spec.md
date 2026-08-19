---
item: HS-S0074
stage: spec
created: 2026-08-12T13:47:12.574Z
updated: 2026-08-12T13:47:12.574Z
template_sig: 87bbf1d0
rendered_sig: 83d9c8b3
---

# Spec — Preflight the merged port, then commit the "structurally unlike" axes

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project charter | `.bklg/from-contract-to-published-library/ladybug-projection-store/project.md` |
| This spec | `.bklg/from-contract-to-published-library/ladybug-projection-store/preflight-and-unlike-axes/spec.md` |
| Key brief — architecture (§ *Mount points* M9, § *The three tensions* T1, § *What this project consumes rather than defines*) | `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md` |
| Key brief — testing (AC-001 row: proven by commit order, not content review) | `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md:471` |
| Signed-off design | `.bklg/from-contract-to-published-library/ladybug-projection-store/_design.md` — **no user-facing surface**, approved 2026-08-12. This story renders none. |
| Story map row (slice, one-line, ordering rationale) | `.bklg/from-contract-to-published-library/ladybug-projection-store/_storymap.md:40`, `:53-59`, `:127-132` |
| Discovery (signal ledger, answered questions, the named wrong implementation) | `.bklg/from-contract-to-published-library/ladybug-projection-store/preflight-and-unlike-axes/discover.md` |
| Roadmap pointer | `RUNBOOK.md:162` (phase 11's row and proof artefact), `RUNBOOK.md:4393-4444` (phase 11's body) |

Traces to project **AC-001** only. `depends_on`: none inside this project; the one inbound edge is
cross-project, on `projection-store-freeze` (HS-P0010), and reading what that project shipped is half
of this story's work.

## One-line PR slice

Assert the merged port really shipped `type Batch;`, the write seam and `projection_store_conformance!`,
then commit the dated "structurally unlike" axes to `references/evaluation/` as its own merge, before
any body is filled in.

## Executive summary

This PR lands **one new document under `references/evaluation/`** and **one edit to that directory's
README**, and touches no Rust at all.

The document carries two things in this order: a **red-or-green preflight finding** — three
observations against the merged `happenstance-core` tree, each stamped with the commit SHA they were
made at — and, if and only if the preflight is green, the **contrastive axes table** that fixes what
"structurally unlike" means for this project before any suite has run against a Ladybug fixture.

The delta over what the project charter already says is three decisions this spec takes and the
charter deliberately left open:

1. **One file, not two.** The preflight finding and the axes live in a single document,
   `references/evaluation/phase-11-preflight-and-unlike-axes.md`, on the naming precedent of
   `references/evaluation/phase-4-reconciliation.md` and `phase-4-5-reconciliation.md`. The discovery
   stage left this open (`discover.md:72-76`); it is settled here.
2. **A red preflight still ships the document.** The finding is committed, the story reports blocked,
   the axes section is left unwritten rather than guessed, and the green run later arrives as a
   *superseding* document — never an edit to this one. That is the only shape compatible with
   `references/evaluation/README.md:8-13`'s immutability rule and with AC-001 being an ordering claim.
3. **The README is the mount.** Two documents already sit in `references/evaluation/` with no
   lifecycle assigned to them by its README. An unclassified evidence file is precisely the file
   someone later edits in place — which would silently destroy the one claim this story exists to
   make. Classifying this document there is what turns a file into evidence.

Everything else in the project — the driver swap, the bodies, the fixture, the run, the verdict —
is downstream and explicitly not here.

## Context pack

Read this section and you can start. The deeper artefacts are behind the anchors table and stay there.

**AC-001 is an ordering claim about commits, not a content claim about a document.** The project
charter states it that way (`project.md:180-184`) and DR-3 says why (`project.md:146-148`): if the
definition of "structurally unlike" can be written after the run, it will be written to match the
result, and the verdict this whole project exists to produce becomes unfalsifiable. The check is
`git log`, and it cannot be satisfied retroactively. Every other decision below is downstream of
protecting that one property.

**The port this story reads has not shipped yet, and the story's own wording depends on what does.**
`crates/happenstance-core/src/projection.rs:97-99` today declares `type Batch<'a> where Self: 'a` — a
GAT with a lifetime — and a repo-wide search for `projection_store_conformance` returns nothing.
`projection-store-freeze` (HS-P0010) commits to dropping the lifetime to `type Batch;` and adding a
write seam. This is why the preflight rides *with* the axes instead of preceding them as its own
story (`_storymap.md:53-59`): if `type Batch;` landed, "no transaction handle type" and "no lifetime"
are two different axes and must be written as two rows, not one.

**The preflight is red-or-green and non-negotiable, and a red one halts the project.** Three
observations, against the merged tree, each recorded with the SHA it was made at
(`_decomposition.md:33-41`, AC-A01): that `crates/happenstance-core/src/projection.rs` declares
`type Batch;` with no lifetime and no `where Self: 'a`; that the write seam exists **and which of the
two candidate shapes shipped** — PS-11's `ProjectionProbe` in the contract crate behind
`feature = "conformance"` (`spec/SPECIFICATION.md:4977-5005`) versus HS-P0010's fixture-level
`write_probe(&mut Batch)` plus an out-of-band `read_probe(&Store)`; and that
`projection_store_conformance!` resolves from `happenstance-testkit`. **Read what shipped — do not
guess between the two seam shapes** (`_decomposition.md:104-114`). None of the three is stubbed,
vendored, feature-gated or worked around locally; a red observation is surfaced to HS-P0010
(`_storymap.md:127-132`).

**T1 is raised from here, in the same breath, because the crate will otherwise not compile.**
HS-P0010's charter claims the Ladybug and Postgres skeletons compile with no change other than the
lifetime parameter's removal. That is true of `LadybugProjectionStore`, whose `Batch` is the owned
`GraphWriteSet`. It is **false** of `LiveHandleProjectionStore`, which binds
`type Batch<'a> = GraphWriteHandle<'a>` (`crates/happenstance-ladybug/src/live_handle.rs:177-180`) —
a genuinely borrowed handle with no lifetime-free spelling for a `'static` store. Under a frozen
`type Batch;` that impl cannot exist. This story does not delete it (that is
`real-lbug-driver-swap`'s); it *reports* it upstream as HS-P0010's own compatibility claim being
false for one of the two impls in this crate (`_decomposition.md:314-331`).

**The axes are written as a contrast, never as a description.** Each row names three things: the
property the Ladybug batch has, the property every shape that froze the port has instead, and **what
a conformance rule would have to do to tell them apart**. A row that cannot answer the third question
is not an axis and does not go in the table. The four axes AC-001 already enumerates are the floor —
no transaction handle type in the driver at all; a deferred, owned, `Send + 'static` write set where
nothing executes until `commit`; a synchronous driver behind an `async` port; single-writer
concurrency in which two commits racing is routine rather than exceptional
(`crates/happenstance-ladybug/src/projection_store.rs:204-213`) — and the preflight's finding
determines whether "no lifetime" splits off from the first as its own row.

**The mutant this document exists to make detectable is a graph batch that is a SQL batch wearing
Cypher.** A `GraphWriteSet` whose statements are all `MERGE (r:Row {k: $k}) SET r.v = $v` — one
label, scalar properties, no relationships, no traversal — is a key-value row store with a Cypher
accent. It passes the suite, satisfies the rule count downstream, and re-tests the freeze against
precisely the shape that froze it. It is caught by a **document, not a test**: only if this story
writes the axis it violates ("the batch expresses relationships and traversal, not rows") can the
negative control in `ladybug-fixture-and-conformance-run` (HS-S0078) be recognised as necessary
later (`discover.md:109-123`).

**PS-2 is the bar the axes are written against, and it is `[FROZEN]`.** It requires "two adapters at
opposite ends of the batch-shape axis" (`spec/SPECIFICATION.md:4760-4774`). The axes document is this
project's statement of *where the opposite end is*. Nothing in this story amends a clause, a clause
marker or a citation: if the port that merged contradicts what this story expects, the response is a
finding raised upstream, never a local edit (`discover.md:92-95`).

**This story writes no design decision.** The synchronous driver is recorded as an *axis of
unlikeness* — a fact about the shape — and the spec says nothing about how the blocking bridge is
built. That is ADR-0025's Q3 and belongs to the slice-mate `adr-0025-three-answers` (HS-S0075). An
axis that encodes a preferred remedy is no longer a neutral yardstick for the verdict
(`discover.md:46-52`).

**`references/evaluation/` documents are superseded, never edited** (`references/evaluation/README.md:8-13`).
One file amended in place cannot make an ordering claim, which is why this story is a merge of its
own and why a corrected preflight arrives as a second document rather than a second draft.

## Integration contract

- **Archetype**: `foundation`. It lands a real in-tree artefact — a dated, commit-pinned evidence
  document — that the `real-adapter`, `conformance-run` and `freeze-verdict` slices consume by name.
  It is not a double, not a placeholder and not terminal (`_storymap.md:53-59`).
- **Slice / milestone**: `preflight-and-decisions`. Slice-mate: **`adr-0025-three-answers`**
  (HS-S0075), which depends on this story and consumes its preflight finding — the port shape it
  answers Q1–Q3 against is the one this story recorded.
- **Mount point**: **`references/evaluation/README.md`**. It is the file that assigns every document
  in that directory to a lifecycle — *immutable evidence, dated, pinned to a commit, superseded
  rather than edited* (`:8-13`) versus *mutable speculation* (`:14-38`) versus the "later additions,
  which are neither" case it already carries for `review-citation-drift.md` (`:40-55`). The new
  document is classified there, in the first lifecycle, by name. This is a real mount and not a
  gesture: `phase-4-reconciliation.md` and `phase-4-5-reconciliation.md` currently sit in that
  directory unclassified by the README, and an unclassified evidence file is exactly the one a later
  contributor edits in place — the single move that destroys AC-001.
- **Wires into** (all read-only from this story; none is edited here):
  - `crates/happenstance-core/src/projection.rs` — the port whose *shipped* shape the preflight
    asserts (`:97-99` is the pre-freeze snapshot the finding is measured against).
  - `crates/happenstance-testkit/src/lib.rs` and `crates/happenstance-testkit/src/registry.rs` —
    where `projection_store_conformance!` must resolve from, mirroring `event_store_conformance!`
    (`lib.rs:311-357`, emitter table at `lib.rs:63-67`, registry at `registry.rs:93-103`).
  - `crates/happenstance-ladybug/src/projection_store.rs` — the module docs that already argue the
    deferred-write-set shape (`:1-64`), the single-writer variant (`:204-213`) and the four `todo!()`
    bodies (`:270-292`) the axes describe without touching.
  - `crates/happenstance-ladybug/src/live_handle.rs:177-180` — the borrowed-handle `Batch` binding
    that T1 says cannot survive `type Batch;`.
  - `spec/SPECIFICATION.md` — PS-2 (`:4760-4774`), PS-5 (`:4868-4883`), PS-11 (`:4977-5005`),
    PS-12 (`:5052-5075`), PS-34 (`:5546-5560`). Cited, never amended.
  - `references/evaluation/phase-4-reconciliation.md` and `phase-4-5-reconciliation.md` — the genre
    and naming precedent for a dated, commit-pinned phase document.
- **Renders surfaces**: **none.** `_design.md` records `N/A — no user-facing surface` for every
  block and was approved on that determination (`_design.md:40-98`). This story ships no Rust public
  item, so no `## Items` row is claimed and none is owed.
- **Conformance rule(s)**: **none, and deliberately.** This story adds no rule and changes no port,
  so there is nothing for a rule to observe; `crates/happenstance-testkit/**` is explicitly not this
  project's to edit (`_decomposition.md:86-88`). Its own falsifiable claim is checked by `git log`
  and by review, which is what the testing brief's AC-001 row already specifies
  (`_decomposition.md:471`). The rule this document makes *writable* — the negative control against
  the SQL-batch-wearing-Cypher mutant — lands in `crates/happenstance-ladybug/tests/` in
  `ladybug-fixture-and-conformance-run` (HS-S0078), not here.
- **Clause(s)**: **discharges none, amends none.** It reads PS-2 as the bar and records the data PS-5,
  PS-11, PS-12 and PS-34 will later be judged against. `[FROZEN]` clause text and markers are
  untouched by construction — this PR's boundary contains no `spec/` path.
- **Advances DoD scenario**: initiative **DoD 8** — *"After the unlike batch shape exists, a written
  verdict states whether the `ProjectionStore` freeze held, either way, with what it was checked
  against"* (`initiative.md:380-382`). This story lands the *"with what it was checked against"* half,
  and lands it first, which is the only ordering under which the verdict is evidence. It is also the
  precondition that makes **DoD 7**'s "two structurally unlike batch shapes" judgeable rather than
  asserted (`initiative.md:377-379`).

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file changed
outside it.

```
references/evaluation/phase-11-preflight-and-unlike-axes.md
references/evaluation/README.md
.bklg/from-contract-to-published-library/ladybug-projection-store/preflight-and-unlike-axes/**
```

**In this PR**

- The new evidence document at `references/evaluation/phase-11-preflight-and-unlike-axes.md`:
  the dated, SHA-stamped preflight finding (three observations plus T1's separate finding), and —
  on a green preflight — the contrastive axes table.
- The mount: `references/evaluation/README.md` gains the new document under the immutable-evidence
  lifecycle, by name, with one line saying what it is.
- This story's own backlog folder (the ledger and the implementation report).

**Explicitly not in this PR**

- Any file under `crates/**`. No `todo!()` is filled, no `stand_in` type is re-pointed, no
  `Cargo.toml` dependency is added, no test target is written. `real-lbug-driver-swap` (HS-S0076)
  and `fill-the-bodies-and-ps-34-disposition` (HS-S0077) own that, and the whole point of this
  story is that it merges *before* any of it.
- `spec/SPECIFICATION.md` — the six citation repairs ride with the bodies
  (`_decomposition.md:66-70`), and nothing `[FROZEN]` moves anywhere in this project.
- `.kb/**` — ADR-0025 is the slice-mate `adr-0025-three-answers`'s, authored through `.kb/_intake/`
  by `/redkiln:kb-ingest`, never by hand (`_decomposition.md:204-209`).
- `xtask/**`, `.github/workflows/ci.yml`, `RUNBOOK.md` — the proof-artefact row, the packaging
  registry and the phase-11 checkboxes belong to HS-S0078, HS-S0080 and HS-S0081/HS-S0082.
- The verdict itself. It is `freeze-verdict-document`'s (HS-S0082), it is unconditional, and it must
  cite this document's commit SHA alongside its own so an inverted order is visible in the artefact
  rather than only in `git log` (`discover.md:125-132`).
- Any local repair of a red preflight. It halts and escalates; it is not routed around
  (`_decomposition.md:33-41`).

**Merge DoD**: the document exists, is dated, is SHA-stamped against the merged `happenstance-core`
tree, is classified in `references/evaluation/README.md`, and is the sole content of a commit that
`git log` shows preceding every commit touching a `projection_store_conformance!` invocation against
a Ladybug fixture — with `cargo xtask ci` green, which for a docs-only diff is a no-regression check
rather than a proof of anything this story claims.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The preflight is three named observations, not a feeling** | Against the merged tree, record: (1) `ProjectionStore::Batch` is declared `type Batch;` — no lifetime parameter, no `where Self: 'a`; (2) the write seam exists **and which shape shipped** — PS-11's `ProjectionProbe` behind `feature = "conformance"` in the contract crate, or HS-P0010's fixture-level `write_probe(&mut Batch)` + out-of-band `read_probe(&Store)`; (3) `projection_store_conformance!` resolves from `happenstance-testkit`. Each observation is recorded with the command that produced it and the commit SHA it was run at. | `_decomposition.md:33-41` (AC-A01); `_decomposition.md:96-119` (what is consumed, not defined); `crates/happenstance-core/src/projection.rs:97-99` (today's pre-freeze snapshot); `crates/happenstance-testkit/src/lib.rs:311-357` (the macro shape being mirrored) |
| **The seam shape is read, never guessed** | Two candidate shapes are in play and they are not interchangeable. If neither can express a replayable parameterised Cypher statement, that is **not** an adapter problem to route around — it is the freeze not holding, and the observation is written down here for the verdict to carry. | `_decomposition.md:104-114`; `spec/SPECIFICATION.md:4977-5005` (PS-11); `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` |
| **A red preflight halts and escalates; it does not get worked around** | Any of the three observations coming back negative stops the project at this story. The finding is still committed (a halt that leaves no artefact is indistinguishable from not having looked), the escalation names `projection-store-freeze` (HS-P0010), and nothing is stubbed, vendored, feature-gated or locally patched to proceed. | `_decomposition.md:33-41`; `_storymap.md:127-132`; `discover.md:54-64` |
| **T1 is a separate finding raised in the same breath** | `LiveHandleProjectionStore` binds `type Batch<'a> = GraphWriteHandle<'a>` — a borrowed handle with no lifetime-free spelling for a `'static` store — so under a frozen `type Batch;` that impl cannot exist, and HS-P0010's "compiles with no change other than the lifetime parameter's removal" claim is false for one of this crate's two impls. Reported here; **retired** by `real-lbug-driver-swap`, not by this story. Costs nothing evidentially: both transcripts are preserved outside the crate. | `crates/happenstance-ladybug/src/live_handle.rs:177-180`; `_decomposition.md:314-331`; `references/adapter-shapes.md:169-195`; `spec/SPECIFICATION.md:4600-4615` |
| **The axes are a contrastive table, one row per axis, three columns** | Ladybug's property · the property every shape that froze the port has instead · what a conformance rule would have to **do** to tell them apart. A row that cannot answer the third column is struck, because it is a description and not an axis. This is the direct guard against the wrong implementation named at discovery: a correctly dated, correctly ordered document that describes Ladybug and names nothing a rule could be blind to, making every verdict written against it unfalsifiable. | `project.md:180-184` (the four axes AC-001 enumerates); `discover.md:30-44`; `discover.md:99-107` (the named wrong implementation) |
| **The four floor axes** | (a) LadybugDB exposes no transaction handle type at all — a transaction is entered by *executing* `BEGIN TRANSACTION`, and nothing in the type system records that one is open; (b) the batch is a deferred, owned, `Send + 'static` write set holding no connection, no transaction and no borrow, where nothing touches the database until `commit`; (c) the driver is synchronous behind an `async` port; (d) many readers and exactly one writer, so two commits racing is routine rather than exceptional and any concurrency-shaped rule meets `WriteTransactionInUse`. | `crates/happenstance-ladybug/src/projection_store.rs:1-64` (a, b), `:39-47` (c), `:204-213` (d) |
| **The fifth axis is decided here, not left implicit** | The Cypher-versus-SQL mutation-vocabulary axis — *the batch expresses relationships and traversal, not rows* — is **included**, because it is the only axis that makes the SQL-batch-wearing-Cypher mutant detectable, and that mutant is the one failure mode that would let this project re-test the freeze against exactly the shape that froze it. If the implementer concludes it should be omitted, the omission is written into the document with its reason; silence is not permitted. | `discover.md:39-44`; `discover.md:109-123`; `spec/SPECIFICATION.md:4760-4774` (PS-2's "opposite ends of the batch-shape axis") |
| **Axis wording tracks the preflight's finding** | If `type Batch;` landed, "no transaction handle type in the driver" and "no lifetime on the port's batch" are two distinct rows — one is a fact about LadybugDB, the other a fact about the frozen port — and conflating them loses the distinction the verdict needs. If the GAT survived, the second row is instead a recorded incompatibility (T1), not an axis. | `_storymap.md:53-59`; `spec/SPECIFICATION.md:4868-4883` (PS-5's predicted relief) |
| **No design decision is smuggled into an axis** | The synchronous driver is recorded as a property of the shape. How it meets the non-blocking port — runtime-gated `spawn_blocking`, blocking the executor thread and documenting it, or a blocking-only adapter — is ADR-0025 Q3 and is answered by the slice-mate. `#[async_trait]` is never a candidate anywhere (ADR-0001). | `discover.md:46-52`; `_decomposition.md:293-310`; `.kb/decisions/0001-async-port-flavours.md` |
| **One document, one commit, and that commit precedes the first run** | The preflight finding and the axes are one file — `references/evaluation/phase-11-preflight-and-unlike-axes.md`, named on the `phase-4-reconciliation.md` precedent — landed alone. The ordering claim is against the first commit touching a `projection_store_conformance!` invocation over a Ladybug fixture, which is HS-S0078's. | `discover.md:72-76`; `_decomposition.md:211-218` (M9); `project.md:180-184` |
| **Superseded, never edited** | If the preflight was red and later goes green, or an axis is later found wrong, the correction is a **new** dated document that names the one it supersedes. Amending this file in place would erase the only claim it makes. The mount edit in `README.md` is what makes that lifecycle binding on the new file rather than a convention someone remembers. | `references/evaluation/README.md:8-13`, `:77-85`; `_decomposition.md:211-218` |
| **Nothing `[FROZEN]` is touched, by construction** | The PR boundary contains no `spec/` path, so no clause body, marker or citation can move. If the merged port contradicts a clause, that is a finding raised upstream. | `project.md:210-213` (AC-008); `discover.md:92-95` |
| **Interfaces changed** | **None.** No Rust item is added, removed, renamed or re-typed; no feature is added; no dependency is added. `cargo xtask ci` on this diff is a no-regression check, and the story's actual claim lives outside anything the compiler can see. | `_design.md:40-98` (no user-facing surface); PR boundary above |

## Data and migrations

**N/A.** This story adds no schema, no persisted format, no serialized envelope and no stored state.
It ships two markdown files in `references/`, and `references/` binds nothing — its README says so
directly: the material there is *"evidence kept for citation"* whose contents lose to
`spec/SPECIFICATION.md` wherever they disagree (`references/evaluation/README.md:87-92`; `CLAUDE.md`,
repository map). The LadybugDB on-disk graph schema — label names, property names, indexes, and
whether the checkpoint node is per-`ProjectionId` or one node with a property per id — is
deliberately not prescribed here and is ADR-0025 Q1's, with PS-23 (`spec/SPECIFICATION.md:5317`)
owned by `projection-store-freeze` (`_decomposition.md:265-274`).

## Acceptance criteria

Framed from the journeys this initiative carries (`initiative.md:241-250`): the **adapter author**'s
*Learn when you are finished* — from a signature that type-checks to a suite that says pass or fail
and names why — and the **evaluator**'s *Decide in one sitting* — a bounded look at public evidence
ending in adopt or decline for a stated reason. Both read this document; neither can read the
backlog. Every criterion below is therefore satisfied only if the artefact in `references/` carries
it, not if the spec or the implementation report says it.

The document's normative shape, which the greps below assume: a `## Preflight finding` section
holding a four-column table (`Observation | Command | Result | Commit SHA`), then a
`## Axes of unlikeness` section holding a three-column table
(`Ladybug's property | What the shapes that froze the port have instead | What a rule would have to do to tell them apart`).
Heading text is fixed so the checks are commands rather than opinions; row content is the
implementer's.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** an adapter author about to write the first Ladybug batch against a port they did not freeze, **WHEN** they open `references/evaluation/phase-11-preflight-and-unlike-axes.md`, **THEN** its `## Preflight finding` table gives them three named observations against the *merged* `happenstance-core` tree — (1) `ProjectionStore::Batch` is declared `type Batch;`, no lifetime parameter and no `where Self: 'a`; (2) the write seam exists; (3) `projection_store_conformance!` resolves from `happenstance-testkit` — each with the exact command that produced it, a green/red result, and the commit SHA it was run at, so the author never has to re-derive whether the port they are coding against is the port that shipped. | `rg -n "^## Preflight finding" references/evaluation/phase-11-preflight-and-unlike-axes.md` and the table's three rows each carrying a 7+ hex SHA (`rg -n "[0-9a-f]{7,40}" …`); the three commands independently re-run at review — `rg -n "type Batch" crates/happenstance-core/src/projection.rs`, `rg -n "projection_store_conformance" crates/` — and their output pasted in `preflight-and-unlike-axes/implementation-report.md`. Tier: **static + process** (`_decomposition.md:33-41`, AC-A01). |
| **AC-002** | **GIVEN** the same author, facing two mutually incompatible write seams — PS-11's `ProjectionProbe` in the contract crate behind `feature = "conformance"` versus HS-P0010's fixture-level `write_probe(&mut Batch)` plus an out-of-band `read_probe(&Store)` — **WHEN** they read observation 2, **THEN** it names **which shape actually shipped**, by `file:line` in the merged tree, and states whether that shape can express a replayable parameterised Cypher statement; **AND** if neither shipped, or the shipped one cannot, the document records that as *the freeze not holding* for the verdict to carry rather than as an adapter problem to route around. | Review-tier read of observation 2 against `spec/SPECIFICATION.md:4977-5005` (PS-11) and `../projection-store-freeze/project.md` (In scope item 4); the cited `file:line` resolved by hand at the recorded SHA. A row whose seam column says "either" or is absent fails. Tier: **review** (`_decomposition.md:104-114`). |
| **AC-003** | **GIVEN** a maintainer who needs to know whether phase 11 may proceed, **WHEN** any of the three observations comes back red, **THEN** the document still merges — carrying the red result, an escalation naming `projection-store-freeze` (HS-P0010) and the specific observation that failed — with the `## Axes of unlikeness` section present but explicitly left unwritten and *why*, and with nothing anywhere in the PR stubbing, vendoring, feature-gating or locally patching the missing port; the story reports blocked rather than green. | `redkiln verify --grain story` proves the PR boundary contains no `crates/**` path, which is what makes "not worked around locally" checkable rather than promised; `rg -n "^## Axes of unlikeness" references/evaluation/phase-11-preflight-and-unlike-axes.md` returns the heading on a red run too. Escalation recorded in `preflight-and-unlike-axes/implementation-report.md` naming HS-P0010. Tier: **process + boundary** (`_storymap.md:127-132`; `_decomposition.md:33-41`). |
| **AC-004** | **GIVEN** an adapter author whose very next command is `cargo build -p happenstance-ladybug`, **WHEN** they read the finding, **THEN** they find T1 raised as its own named item — that `LiveHandleProjectionStore` binds `type Batch<'a> = GraphWriteHandle<'a>` with no lifetime-free spelling for a `'static` store, that HS-P0010's "compiles with no change other than the lifetime parameter's removal" claim is therefore **false for one of this crate's two impls**, that its retirement belongs to `real-lbug-driver-swap` and not to this story, and that the instrument's transcripts survive its deletion at `references/adapter-shapes.md:169-195` — so they hit the compile error already knowing it is expected, whose it is, and what it costs. | `rg -n "LiveHandleProjectionStore" references/evaluation/phase-11-preflight-and-unlike-axes.md` returns the finding, and review confirms it cites `crates/happenstance-ladybug/src/live_handle.rs:177-180` and `references/adapter-shapes.md:169-195`. The PR touches no `crates/**` file, so the impl is reported and not deleted here. Tier: **static + review** (`_decomposition.md:314-331`). |
| **AC-005** | **GIVEN** an evaluator with one sitting and no access to this backlog, **WHEN** they read `## Axes of unlikeness`, **THEN** every row is a *contrast* and not a description: three populated columns — Ladybug's property, the property every shape that froze the port has instead, and **what a conformance rule would have to do to tell them apart** — covering at minimum the four axes AC-001 of the project enumerates (no transaction handle type in the driver at all; a deferred, owned, `Send + 'static` write set where nothing executes until `commit`; a synchronous driver behind an `async` port; single-writer concurrency where two racing commits are routine), so the evaluator can say what the eventual verdict could have come out as, not merely what it did. | Structural check: `rg -c "^\|" references/evaluation/phase-11-preflight-and-unlike-axes.md` confirms the axes table has ≥ 4 body rows, and review confirms **no** third-column cell is empty or restates the first. A row that cannot answer column three is struck, not softened — this is the direct rejection of the named wrong implementation at `discover.md:99-107`. Cross-checked against `crates/happenstance-ladybug/src/projection_store.rs:1-64`, `:39-47`, `:204-213` and PS-2 (`spec/SPECIFICATION.md:4760-4774`). Tier: **static + review**. |
| **AC-006** | **GIVEN** the implementer of HS-S0078's negative control, who will later have to recognise a `GraphWriteSet` of `MERGE (r:Row {k: $k}) SET r.v = $v` statements as a key-value row store wearing a Cypher accent, **WHEN** they consult this document, **THEN** it has already decided the Cypher-versus-SQL mutation-vocabulary axis explicitly — either carrying it as a row ("the batch expresses relationships and traversal, not rows") with a third column naming what a rule would have to do, or carrying a written omission with its reason — so the mutant is caught by a document that existed first rather than by hindsight. Silence on this axis fails the criterion. | `rg -n -i "relationship|traversal|vocabulary" references/evaluation/phase-11-preflight-and-unlike-axes.md` returns either the axis row or the recorded omission; review confirms one of the two is present and reasoned. Tier: **static + review** (`discover.md:109-123`; PS-2's "opposite ends of the batch-shape axis", `spec/SPECIFICATION.md:4760-4774`). |
| **AC-007** | **GIVEN** a reviewer of the eventual freeze verdict who must decide whether it is evidence or rationalisation, **WHEN** they run `git log` over the repository, **THEN** the axes document is the sole content of one commit — together with the `references/evaluation/README.md` line that classifies it **by name** under the immutable-evidence lifecycle (dated, commit-pinned, superseded rather than edited) — and that commit precedes every commit touching a `projection_store_conformance!` invocation against a Ladybug fixture, so the definition provably could not have been chosen to match the result. | `git log --diff-filter=A --format=%H -- references/evaluation/phase-11-preflight-and-unlike-axes.md` versus `git log -S "projection_store_conformance" --format=%H -- crates/happenstance-ladybug/`, with the two SHAs and their order recorded in `implementation-report.md`; `git show --stat <sha>` proves the commit's only files are the document and the README; `rg -n "phase-11-preflight-and-unlike-axes" references/evaluation/README.md` proves the mount. Tier: **process / commit order — proven by commit order, not content review** (`_decomposition.md:471`; `project.md:180-184`; `references/evaluation/README.md:8-13`). |

Project **AC-001** is covered in full: its ordering half by AC-007, its content half ("naming the
axes on which the Ladybug batch differs") by AC-005 and AC-006, and the precondition that makes the
axes' wording correct rather than merely early by AC-001, AC-002, AC-003 and AC-004.

## Interaction quality

**This is the repository's `## Surface quality` block** (`.redkiln/templates/spec.md`), which
deliberately replaces the bundled interaction-quality list because a library has the same hole in
another medium. This story goes one step further: it ships **no Rust item at all**, so most of that
list cannot fail here and is struck rather than ticked — the template's own instruction. What it is
replaced by is not nothing. This story's deliverable is a *document a stranger reads without the
backlog*, and every invariant below names something that a green `cargo xtask ci`, a green
`spec-trace` and a correctly dated commit are all satisfied by.

**Composition invariants.** `_design.md` records `N/A — no user-facing surface` for every block and
was approved on that determination (`_design.md:40-98`), so there is no signed-off composition to
honour and none is invented here. The composition this artefact *does* owe is the one
`references/evaluation/README.md` already binds on that directory, and the genre precedent
`phase-4-reconciliation.md` sets:

- **Presentation exists at all** — the artefact is a composed evidence document (dated, SHA-pinned,
  two named sections, tabular findings), not a paste of shell output or a bullet list of
  impressions. Carried by **AC-001** (four-column finding table with commands and SHAs) and
  **AC-005** (three-column axes table).
- **Placement and transience** — the document is *persistent chrome* in its directory: reachable
  from `references/evaluation/README.md` by name rather than by knowing it exists. Carried by
  **AC-007**. This is the invariant `phase-4-reconciliation.md` currently fails, and an unclassified
  evidence file is exactly the file someone later edits in place.
- **Density budget, with the real numbers** — the axes table is **≥ 4 body rows × 3 populated
  columns** (**AC-005**), plus the fifth axis decided either way (**AC-006**); the preflight table is
  **exactly 3 observation rows × 4 columns** (**AC-001**), with T1 as a separate finding rather than
  a fourth row (**AC-004**) because it is a report about an impl, not an observation about the port.
- **Hierarchy** — preflight first, axes second, in one file. The reader must know *which port shape
  was observed* before reading a single axis, because an axis written against the wrong shape is
  worse than no axis. Carried by the section order asserted in **AC-001** and **AC-003**.
- **Named anti-patterns, all three from `discover.md`** — a document that *describes* Ladybug
  instead of contrasting it, caught by **AC-005**'s third column; an axis that encodes a preferred
  remedy (ADR-0025 Q3's blocking bridge), kept out by **AC-005** and by the PR boundary excluding
  `.kb/**`; and the ordering mutant — axes and first run in one merge, or an in-place amendment
  afterwards — caught by **AC-007**.

**State invariants**, translated to this medium:

- **Reversibility** — a correction is a *new* superseding document, never an edit; the mount edit is
  what makes that lifecycle binding rather than remembered. **AC-007**.
- **Non-occlusion** — a red preflight does not hide the artefact. The finding merges either way, and
  the axes section is present-and-explicitly-unwritten rather than absent. **AC-003**.
- **Preserved context** — the SHA that each observation was made at travels with the observation, so
  a reader six months later can tell whether the finding is still about the tree they have.
  **AC-001**.

**Struck, with the reason — each cannot fail in this story:** *the surface exists as designed*, *both
flavours*, *the example compiles*, *errors are documented*, *visibility is as signed off* — this PR
adds, removes, renames and re-types no Rust item and no feature (`_design.md:40-98`; PR boundary).
Every **suite invariant** (*the rule can fail*, *no literal position values*, *no clock*, *a
changelog entry*) is struck too: `crates/happenstance-testkit/**` is explicitly not this project's to
edit (`_decomposition.md:86-88`) and this story adds no rule. The negative control this document
makes *writable* is HS-S0078's, in `crates/happenstance-ladybug/tests/`.

## Error conditions

| id | Condition | Required handling |
| --- | --- | --- |
| **EC-001** | Any of the three preflight observations is **red** — `type Batch;` did not land, the write seam is absent, or `projection_store_conformance!` does not resolve from `happenstance-testkit`. | Halt the project at this story. Commit the finding anyway (a halt that leaves no artefact is indistinguishable from not having looked), leave `## Axes of unlikeness` explicitly unwritten with the reason, escalate to `projection-store-freeze` (HS-P0010) by name, and report the story blocked. Do not stub, vendor, feature-gate or locally patch anything to proceed. (`_decomposition.md:33-41`; `_storymap.md:127-132`.) |
| **EC-002** | The write seam shipped, but in **neither** candidate shape — or in a shape that cannot express a replayable parameterised Cypher statement. | Not an adapter problem to route around: record it in observation 2 as the freeze not holding, in the words the verdict will need, and let `freeze-verdict-document` (HS-S0082) carry it. No local shim, no fixture-side workaround. (`_decomposition.md:104-114`; `.kb/open-questions/projection-store-batch-has-no-apply-seam.md`.) |
| **EC-003** | The merged port is `type Batch;` and `crates/happenstance-ladybug/src/live_handle.rs:177-180` therefore cannot compile. | Expected, and this story's job is to *say so*, not to fix it: record T1 as a separate finding (AC-004), name `real-lbug-driver-swap` (HS-S0076) as its owner including the `port_shape.rs:77,79` lines that instantiate generic code at `LiveHandleProjectionStore`, and touch no `crates/**` file here. |
| **EC-004** | The GAT survived — `type Batch<'a> where Self: 'a` is still what merged. | This is a red observation 1 (EC-001 applies) **and** it changes the axes' wording: "no lifetime on the port's batch" is then not an axis at all but a recorded incompatibility, and no axis may be written that assumes a lifetime-free `Batch`. (`_storymap.md:53-59`; PS-5, `spec/SPECIFICATION.md:4868-4883`.) |
| **EC-005** | An axis is later found wrong, or a red preflight later goes green. | A **new** dated document under `references/evaluation/` that names the one it supersedes. Never an edit to this one — amending it in place erases the only claim it makes. (`references/evaluation/README.md:8-13`, `:77-85`.) |
| **EC-006** | A `file:line` citation inside the new document drifts. | `cargo xtask spec-trace` scans `spec/SPECIFICATION.md`, **not** `references/`, so nothing in the gate will catch this. Every citation in the document is re-resolved by hand before commit and the document states the SHA it was pinned at, which is what converts drift from a silent error into a dated one. (`references/evaluation/README.md:40-55` records this directory's own history of exactly this failure.) |
| **EC-007** | The implementer concludes the fifth axis should be omitted. | Permitted, but the omission and its reason are written into the document. Silence fails **AC-006** — a reader cannot distinguish "considered and rejected" from "never thought of". (`discover.md:39-44`.) |

## Non-functional

| id | Requirement | How it is checked |
| --- | --- | --- |
| **NF-001** | **Docs-only diff.** No file under `crates/**`, `spec/**`, `xtask/**`, `.kb/**`, `.github/**` or `RUNBOOK.md` changes. | `redkiln verify --grain story` against the PR boundary block; `cargo xtask affected --base main` names no affected package. |
| **NF-002** | **No regression, and no more claimed than that.** `cargo xtask ci` is green on this diff, and the story does **not** cite it as evidence for any AC: for a docs-only change it proves nothing this story asserts. | `cargo xtask ci` (`xtask/src/main.rs`). |
| **NF-003** | **Self-contained for a reader with no backlog access.** Every claim in the document resolves to a repo path, a clause id, or a commit SHA — never to a `.bklg/` item, which the evaluator persona cannot read from a published repository. | Review: `rg -n "\.bklg/" references/evaluation/phase-11-preflight-and-unlike-axes.md` returns nothing. |
| **NF-004** | **Legibility budget.** The preflight finding fits on one screen above the axes (≈ 60 lines), and no axes-table cell exceeds three sentences. An axis nobody finishes reading is an axis nobody checks the verdict against. | Review at merge. |
| **NF-005** | **Immutable after merge.** No commit after this story's may modify the document's body; only a superseding document may. | `git log --follow -- references/evaluation/phase-11-preflight-and-unlike-axes.md` shows exactly one content commit at the project's close, checked in `freeze-verdict-document`'s review. |

## Implementation notes (non-prescriptive)

- **Do the preflight before writing a word of the axes.** The wording of at least one axis depends on
  its result (`_storymap.md:53-59`), and writing the axes first is how a stale signature gets
  laundered into evidence. Read `crates/happenstance-core/src/projection.rs` at the merged tree,
  `rg projection_store_conformance crates/`, and locate the seam by reading the shipped code rather
  than the two charters that predict it.
- **Record the SHA once, at the top, and per observation.** `git rev-parse HEAD` at the moment the
  observations are made. If the tree moves mid-story, the observations are re-run rather than
  re-labelled.
- **Write the third column first.** For each candidate axis, answer "what would a conformance rule
  have to *do* to tell these apart?" before writing columns one and two. Rows that cannot answer it
  die early, which is cheaper than discovering at verdict time that the table is a description.
- **Keep the axes free of remedies.** "The driver is synchronous behind an `async` port" is an axis;
  "so we will use `spawn_blocking`" is ADR-0025 Q3 and belongs to the slice-mate
  `adr-0025-three-answers` (HS-S0075) (`discover.md:46-52`; `_decomposition.md:293-310`).
- **The README edit is one line under the immutable-evidence lifecycle**, matching how
  `review-citation-drift.md` is handled at `references/evaluation/README.md:40-55` — name, date,
  pinned SHA, one sentence on what it is. Classifying the two unclassified `phase-4*` documents at
  the same time is tempting and is *out of scope*; it widens the diff of a commit whose whole value
  is being narrow.
- **Naming**: `references/evaluation/phase-11-preflight-and-unlike-axes.md`, on the
  `phase-4-reconciliation.md` / `phase-4-5-reconciliation.md` precedent. The verdict later takes its
  own file (M9, `_decomposition.md:211-218`).
- **Commit shape**: one commit, two files, no backlog files mixed in if that can be avoided — a
  `git show --stat` that a reviewer can read in one glance is what makes AC-007 checkable by
  inspection rather than by archaeology.

## Tests and CI (merge gate)

Grounded in the testing brief's AC-001 row: *"checked by commit order, not by content review — DR-3's
whole point is that the definition cannot be dated after the result"* (`_decomposition.md:471`). The
content still has to be right; it is simply proven by a different tier.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Process — commit order** | `git log --diff-filter=A --format=%H -- references/evaluation/phase-11-preflight-and-unlike-axes.md` compared with `git log -S "projection_store_conformance" --format=%H -- crates/happenstance-ladybug/`; both SHAs recorded in `preflight-and-unlike-axes/implementation-report.md` | **AC-007** — and through it project AC-001's ordering half. The one claim that cannot be satisfied retroactively. |
| **Process — commit isolation** | `git show --stat <axes-sha>` | **AC-007** — the commit's content is the document plus the README line and nothing else. |
| **Static — mount** | `rg -n "phase-11-preflight-and-unlike-axes" references/evaluation/README.md` | **AC-007** — the document is classified under the immutable-evidence lifecycle by name, not merely present in the directory. |
| **Static — finding shape** | `rg -n "^## Preflight finding" references/evaluation/phase-11-preflight-and-unlike-axes.md`; three observation rows each carrying a commit SHA | **AC-001** — three named observations, with commands and SHAs, not a summary. |
| **Static — re-run of the observations** | `rg -n "type Batch" crates/happenstance-core/src/projection.rs`; `rg -n "projection_store_conformance" crates/`; the seam located by `file:line` in the merged tree | **AC-001**, **AC-002** — the reviewer reproduces the finding rather than accepting it. |
| **Static — axes shape** | `rg -n "^## Axes of unlikeness" …`; ≥ 4 body rows; no empty third column; `rg -n -i "relationship\|traversal\|vocabulary" …` | **AC-005**, **AC-006** — the table is a contrast and the fifth axis is decided either way. |
| **Static — T1 finding** | `rg -n "LiveHandleProjectionStore" references/evaluation/phase-11-preflight-and-unlike-axes.md` | **AC-004** — raised here, retired elsewhere. |
| **Boundary** | `redkiln verify --grain story` over this spec's PR-boundary block | **AC-003**, **NF-001** — no `crates/**` path in the diff, which is what makes "not worked around locally" a checked claim. |
| **Gate — no regression** | `cargo xtask ci` (`xtask/src/main.rs`); `cargo xtask affected --base main` | **NF-001**, **NF-002** — the docs-only diff moves nothing. Deliberately proves none of the ACs. |
| **Review — adversarial** | The story's `_review.md`, read against `discover.md:99-107` (the named wrong implementation) and `discover.md:109-123` (the SQL-batch-wearing-Cypher mutant) | **AC-002**, **AC-005**, **AC-006** — the tier that catches a correctly dated, correctly ordered, unfalsifiable document. |
| **Backlog hygiene** | `redkiln validate --kb && redkiln doctor` | Six expected `template-drift` advisories and no more (CLAUDE.md, *Where the work lives*); this story authors no `.kb/` atom. |

## Risks and coupling (PR-scoped)

- **The upstream port may not have merged.** This story's single hard coupling is cross-project, on
  `projection-store-freeze` (HS-P0010). Today `crates/happenstance-core/src/projection.rs:97-99`
  still declares the GAT and `projection_store_conformance!` does not exist. If that is still true
  when the story runs, EC-001 and EC-004 apply and the whole project halts here. That is the designed
  outcome, not a failure of this story — this is the project's only halt point.
- **Guessing the seam is the most likely silent defect.** Both candidate shapes are documented,
  plausible and incompatible, and a spec-shaped guess reads exactly like a finding. Mitigated by
  AC-002's requirement that the shipped shape be named by `file:line` at a SHA.
- **The unfalsifiable-document risk is not caught by any command.** Every automatable check passes on
  a table of accurate descriptions. Only the review tier and AC-005's third column stand between this
  story and a verdict worth nothing (`discover.md:99-107`).
- **Ordering is destroyed by convenience, not by malice.** The realistic failure is a later
  contributor amending the file "for clarity" after the run. Mitigated structurally by the README
  classification (AC-007) and by `freeze-verdict-document` (HS-S0082) being required to cite this
  document's SHA alongside its own (`discover.md:125-132`).
- **Citation drift inside the document is ungated** (EC-006): `spec-trace` does not scan
  `references/`. This directory has already been burned by exactly this
  (`references/evaluation/README.md:40-55`).
- **Coupling out**: `adr-0025-three-answers` (HS-S0075) answers Q1–Q3 *against the port shape this
  story records* — a wrong preflight propagates into an accepted decision atom, which is immutable
  once accepted. `ladybug-fixture-and-conformance-run` (HS-S0078) writes its negative control against
  AC-006's axis. `freeze-verdict-document` (HS-S0082) is judged against this table.
- **No coupling to the compiler at all**, which is the deceptive part: nothing here can be caught by
  `cargo`. The gate's greenness on this PR is evidence of nothing the story claims.

## Dependencies

- **Blocks on (story slugs)**: **none** — `depends_on: []`. Nothing inside this project precedes it;
  it is the project's first merge (`_storymap.md:120-132`, merge order slice 1).
- **Blocks on (cross-project, not a story edge)**: `projection-store-freeze` (**HS-P0010**) must have
  merged `type Batch;`, the write seam and `projection_store_conformance!`. This is not modelled as a
  `depends_on` because it is another project's; reading what it shipped *is* half of this story's
  work, and finding it absent is a recorded outcome (EC-001) rather than a scheduling error.
- **Unlocks (immediately)**: `adr-0025-three-answers` (HS-S0075), the slice-mate, which depends on
  this story and answers ADR-0025's three questions against the port shape recorded here.
- **Unlocks (downstream)**: `real-lbug-driver-swap` (HS-S0076) inherits T1's retirement;
  `ladybug-fixture-and-conformance-run` (HS-S0078) may not run the suite until this document's commit
  exists; `freeze-verdict-document` (HS-S0082) is written against this table and must cite its SHA.

## Anchors (progressive disclosure)

Everything needed to *start* is above. These are the deeper artefacts — open each at the moment named,
and never in bulk.

| Anchor (real path) | Why it is load-bearing | When to open | Serves AC |
| --- | --- | --- | --- |
| `crates/happenstance-core/src/projection.rs` | The port itself. `:97-99` is the pre-freeze GAT snapshot every observation is measured against; the merged shape here is observation 1 and decides the axes' wording. | First action of the story, before writing anything. | AC-001, AC-002 |
| `crates/happenstance-testkit/src/lib.rs` | `:311-357` is `event_store_conformance!`, the macro shape `projection_store_conformance!` must mirror; `:63-67` is the emitter table it inherits. Observation 3 is "does the projection twin resolve from here". | With observation 3, immediately after observation 1. | AC-001 |
| `crates/happenstance-testkit/src/registry.rs` | `:93-103` is the registry the projection macro is emitted through. If observation 3 is ambiguous, this is where the answer is. | Only if `rg projection_store_conformance` is ambiguous. | AC-001 |
| `spec/SPECIFICATION.md` | PS-11 (`:4977-5005`) sketches one of the two candidate write seams; PS-2 (`:4760-4774`, `[FROZEN]`) is *the bar the axes are written against* — "two adapters at opposite ends of the batch-shape axis"; PS-5 (`:4868-4883`) is the lifetime-free `Batch`; PS-34 (`:5546-5560`) is the E0195 trap T1 touches. Cited, never amended. | PS-11 with observation 2; PS-2 before writing the first axis. | AC-002, AC-005, AC-006 |
| `../projection-store-freeze/project.md` | The *other* candidate seam — HS-P0010's own charter commits to a fixture-level `write_probe(&mut Batch)` plus out-of-band `read_probe(&Store)` (In scope item 4), and to the "compiles with no change other than the lifetime parameter's removal" claim T1 falsifies. | With observation 2, and again when writing the T1 finding. | AC-002, AC-004 |
| `crates/happenstance-ladybug/src/projection_store.rs` | The module docs already argue the shape the axes describe: `:1-64` the deferred owned write set and no transaction handle type, `:39-47` the synchronous driver and why `spawn_blocking` was left unused, `:204-213` the single-writer variant and `WriteTransactionInUse`. Do not re-derive these — quote them. | Immediately before writing the axes table. | AC-005 |
| `crates/happenstance-ladybug/src/live_handle.rs` | `:177-180` binds `type Batch<'a> = GraphWriteHandle<'a>` — the borrowed handle with no lifetime-free spelling. This is the evidence T1's finding rests on. | When writing the T1 finding. | AC-004 |
| `references/adapter-shapes.md` | `:169-195` preserves the `live_handle` transcript outside the crate, which is why retiring the instrument costs nothing evidentially — the sentence T1's finding needs so its escalation is not read as an objection. | With the T1 finding. | AC-004 |
| `references/evaluation/README.md` | `:8-13` states the immutable-evidence lifecycle (dated, commit-pinned, superseded rather than edited); `:40-55` shows the exact shape of a later addition being classified, and its §1 records this directory's own citation-drift failure. This file is the **mount**. | Before writing the README line; before deciding how a correction would land. | AC-007 |
| `references/evaluation/phase-4-reconciliation.md` | The genre precedent — how a dated, commit-pinned phase document is structured and named in this repository. Read for shape, not content. | Once, before drafting the document's skeleton. | AC-001, AC-005 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/preflight-and-unlike-axes/discover.md` | `:99-107` names the wrong implementation this story rejects (a description masquerading as a contrast); `:109-123` names the SQL-batch-wearing-Cypher mutant that AC-006's axis exists to make detectable; `:125-132` names the ordering mutant. | Before writing the axes, and again at self-review. | AC-005, AC-006, AC-007 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md` | `:33-41` AC-A01 (the red-or-green preflight and the halt); `:104-114` "read what shipped, do not guess"; `:211-218` M9 (two documents, two commits); `:314-331` T1 in full; `:471` the testing brief's AC-001 row. | Section by section, at the AC each serves. | AC-001, AC-002, AC-004, AC-007 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/project.md` | `:180-184` is project AC-001's exact wording, including the four axes it enumerates; `:146-148` is DR-3, the reason the ordering is the deliverable. | Before writing the axes table; the four axes are the floor, not a menu. | AC-005, AC-007 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_storymap.md` | `:53-59` explains why the preflight rides *with* the axes (the wording depends on what merged); `:127-132` is merge-order gate 1 and the escalation path. | When deciding whether a red preflight halts or degrades. | AC-003 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The evaluator's *Decide in one sitting* and the adapter author's *Learn when you are finished* — the two readers every AC above is framed for, and the reason NF-003 forbids `.bklg/` citations in the artefact. | When judging whether the document reads for a stranger. | AC-005, AC-006 |
| `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` | The open question the write seam answers: generic code can `begin` and `commit` and cannot write anything between. Sub-questions 1 and 2 are what the shipped seam had to close. | With observation 2, if the shipped seam's adequacy is in doubt. | AC-002 |
| `.kb/decisions/0001-async-port-flavours.md` | `#[async_trait]` is never a candidate for the synchronous-driver axis, in any story. Prevents the axis from drifting into a remedy that is already forbidden. | Before writing the synchronous-driver axis. | AC-005 |
| `RUNBOOK.md` | `:162` is phase 11's row and its proof artefact; `:4393-4444` is phase 11's body. Orientation for where this document sits in the plan of record — **not edited by this story**. | Only if the story's place in the phase is unclear. | AC-007 |

## Clarifications resolved during spec

1. **One file, not two** (deferred by `discover.md:72-76`). The preflight finding and the axes live in
   a single document, `references/evaluation/phase-11-preflight-and-unlike-axes.md`, because a reader
   must know which port shape was observed before reading an axis, and two files make that ordering a
   convention rather than a layout. The *verdict* remains a separate file in a separate commit (M9).
2. **A red preflight still ships the document** (`discover.md:54-64` answered "halt"; the artefact
   question was open). It merges with the red finding and an explicitly unwritten axes section. A halt
   that leaves no artefact is indistinguishable from not having looked, and a later green run arrives
   as a *superseding* document, never an edit.
3. **The fifth axis is included** (deferred by `discover.md:39-44`). It is the only axis that makes the
   SQL-batch-wearing-Cypher mutant detectable, and that mutant is the one failure mode that would let
   this project re-test the freeze against exactly the shape that froze it. AC-006 permits omission —
   but only in writing, with a reason.
4. **The README is the mount, and this is a real integration.** `references/evaluation/` has no
   composition root in the software sense; the README is the file that assigns lifecycle, and an
   unclassified evidence file is precisely the one edited in place later — the single move that
   destroys project AC-001. The two unclassified `phase-4*` documents are left alone: fixing them here
   widens a commit whose value is its narrowness.
5. **Section headings are normative; row content is not.** `## Preflight finding` and
   `## Axes of unlikeness` are fixed so the merge-gate checks are commands rather than opinions. What
   goes in the rows is the implementer's, subject to AC-005's third-column rule.
6. **T1 is a finding, not an axis, and not a fourth observation.** It is a report about one impl in
   *this* crate, whereas the three observations are about the *port*. Mixing them would let a reader
   conclude the port was red when it was green.
7. **AC ids are exactly the seven the first pass decided** — AC-001 … AC-007. None added, none
   dropped; the ledger matches one-for-one.
8. **`cargo xtask ci` is listed in the merge gate but proves no AC.** Stating that explicitly is
   deliberate: on a docs-only diff a green gate is the most available and least relevant evidence in
   the repository, and a reviewer who accepts it has accepted nothing.
