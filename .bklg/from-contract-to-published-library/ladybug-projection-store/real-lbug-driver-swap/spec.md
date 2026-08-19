---
item: HS-S0076
stage: spec
created: 2026-08-12T13:47:14.114Z
updated: 2026-08-12T13:47:14.114Z
template_sig: 87bbf1d0
rendered_sig: 0abf8895
---

# Spec — Swap the stand-in for the real `lbug` driver

## Scope lock

| | Path |
|---|---|
| Initiative | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) — Goals; **DoD 7** and **DoD 8**; BR-03 / BR-04ʳ |
| Project | [`.bklg/from-contract-to-published-library/ladybug-projection-store/project.md`](../project.md) — AC-003, DR-1, DR-2; the GAT/ICE and build-cost risk rows |
| This spec | `.bklg/from-contract-to-published-library/ladybug-projection-store/real-lbug-driver-swap/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — Architecture brief §1 (*Modules and seams touched*), §3 (**M6**, **M7**), §6 (**T1**, **T2**, **T3**), §7, §9; Testing brief AC-003 row and *Fixtures / seams* |
| Story map | [`../_storymap.md`](../_storymap.md) — slice `real-adapter`, row `real-lbug-driver-swap`; *Why the slices are cut here*, second bullet |
| Design | [`../_design.md`](../_design.md) — **no user-facing surface**, signed off 2026-08-12, including the *Public API surface note* on the re-sourced error fields |
| Roadmap | `RUNBOOK.md:4406-4425` — phase 11's work list, second box: *"Add `lbug` and measure the cold build"* |

## One-line PR slice

Replace the `lbug` NOTE in `crates/happenstance-ladybug/Cargo.toml:18-22` with the real dependency,
re-point `Value`/`Error` off `stand_in`, delete `stand_in.rs`, retire `live_handle.rs` together with
the `port_shape.rs` lines that instantiate it, and rewrite `lib.rs`'s driver-absent and open-decisions
sections at ADR-0025.

## Executive summary

This PR is the moment `happenstance-ladybug` stops being type-checked against a description of
LadybugDB and starts being type-checked against LadybugDB. Nothing it lands is new behaviour: the four
port bodies remain `todo!()`, `#![allow(clippy::todo)]` stays, no fixture and no conformance
invocation appear. What changes is **who is the authority on `lbug`'s shape** — until this merge it is
the calibration table a human wrote at `crates/happenstance-ladybug/src/stand_in.rs:20-30`; after it,
the compiler.

The delta against the tree as it stands:

- `lbug` becomes a real dependency, pinned once in the workspace manifest's `[workspace.dependencies]`
  beside `rusqlite` and `sqlx`, and from this commit onward every contributor's `cargo xtask ci` builds
  LadybugDB's C++ (`xtask/src/main.rs:115-155`, `:546-556`). That cost is **observed and handed on**
  here; deciding what CI does about it is `cold-build-cost-and-ci-shape`'s.
- `stand_in.rs` is deleted, not deprecated — and with it go the four auto-trait assertions at
  `stand_in.rs:332-360` that are the entire compiler-checked basis for this adapter implementing the
  **`Send`** flavour (`projection_store.rs:252-257`). Re-homing them against the real types is the
  substance of this PR, not a tidy-up.
- `live_handle.rs` — the instrument, not an adapter — is retired or re-pointed according to what the
  merged port actually shipped (**T1**/**T3**), taking `port_shape.rs:77,79` with it if retired, and
  taking four `spec/SPECIFICATION.md` citations with it either way.

After this merge the crate compiles against the real driver with `todo!()` bodies still in place. That
is deliberately not an adapter — `fill-the-bodies-and-ps-34-disposition` is the story that makes it
one, in the same slice.

## Context pack

Ten decisions. Read them before opening anything under *Anchors*.

**1. The stand-in is an instrument with an output, and deleting it without reading the output throws
the output away.** `stand_in.rs:20-30` is a table of **eight** facts about `lbug` 0.16.1, each with a
cited source, and three of them are load-bearing arguments the whole project rests on: *there is no
`Transaction` type* (which is why the batch is a deferred write set and why PS-4 survives —
`projection_store.rs:1-22`), *`Connection::query` takes `&self`* (which is why an interleaved
transaction is expressible and why `finish` consumes the handle — `stand_in.rs:203-212`), and
*`Database` and `Connection` are `Send + Sync`* (which is why this is the `Send` flavour —
`projection_store.rs:252-257`). The swap **reconciles each of the eight against the real crate and
records every divergence**. A divergence is not a bug to absorb quietly: it is a finding the freeze
verdict will cite, and this is the only commit in the project where it is cheap to see.

**2. The `Send` flavour is chosen on evidence, and this PR deletes the evidence unless it re-homes it.**
`stand_in.rs:332-360` asserts `Send`/`Sync` on `Database` and `Connection`, `Send` on `QueryResult`, and
`Send + Sync` on the driver error — with the comment that "if `lbug` is later found to differ, the fix
is here and the adapter's flavour choice is reopened." Those assertions must survive the module's
deletion as tests in the crate against the real types. If the real `Database` or `Connection` is not
`Send + Sync`, **stop**: `impl SendProjectionStore` is no longer honest, that is a BR-12-shaped finding
for the verdict, and the correct move is to surface it, not to switch flavours in passing.

**3. `Error: Send` is a bound a *caller* writes, not something the port demands — so a non-`Send`
`lbug::Error` fails `port_shape.rs`, and that failure is the finding.** ADR-0009 keeps
`ProjectionStore::Error` at exactly `core::error::Error + 'static`
(`.kb/decisions/0009-error-send-sync.md`, and the port at
`crates/happenstance-core/src/projection.rs:90`), so nothing in the contract forces `Send`. But
`port_shape.rs:58-61` instantiates a real `tokio::spawn` runner under
`S: SendProjectionStore<Error: Send>`, and `LadybugProjectionStoreError` will wrap `lbug::Error` from
this commit onward. Architecture brief §9 is explicit: *do not weaken `port_shape.rs` to make it
compile; if a bound has to change, the change is the finding.*

**4. `stand_in` retires from the whole crate at once, because the error enum is shared.**
`LadybugProjectionStoreError::Driver(#[from] stand_in::Error)` and `Commit(#[source] stand_in::Error)`
live at `projection_store.rs:158-214`, and `live_handle.rs:91-92` imports that same enum plus
`stand_in::{Connection, Database}`. The moment `Driver` wraps `lbug`'s error, `live_handle.rs` stops
compiling. The two modules cannot sit in different type universes while sharing one error type, and
`projection_store.rs:152-157` says the sharing is deliberate. This is **T3**, and it is a compile
argument rather than a stylistic preference — which is why the narrow reading of AC-003 ("no
`stand_in` on any path the *suite* exercises") loses here.

**5. What the merged port shipped decides how much of this story exists, and the mismatch is surfaced
rather than absorbed.** `preflight-and-unlike-axes` has already asserted the shape; read its result
before writing anything.

- *If `type Batch;` (no lifetime) landed:* `LadybugProjectionStore`'s
  `type Batch<'a> = GraphWriteSet where Self: 'a` (`projection_store.rs:265-268`) collapses to
  `type Batch = GraphWriteSet` — a deletion, exactly as the comment at `:261-264` predicted — and
  `port_shape.rs:61`'s `for<'a> S::Batch<'a>: Send` collapses to `S::Batch: Send`. **That collapse is
  written down**, because it is PS-5's predicted ergonomic relief observed rather than argued
  (`port_shape.rs:50-57`). And `LiveHandleProjectionStore` **cannot exist**: its
  `type Batch<'a> = GraphWriteHandle<'a>` (`live_handle.rs:177-180`) is a genuinely borrowed handle
  with no lifetime-free spelling for a `'static` store (**T1**). It is retired.
- *If the GAT survived:* nothing collapses, and `live_handle.rs` is re-pointed at `lbug` in the same
  change rather than deleted (**T3**).
- *If the port shipped neither shape:* halt and surface it to `projection-store-freeze` (HS-P0010).
  Do not work around an upstream port locally — architecture brief AC-A01, and `_grounding.md`
  *The dependency this project cannot see past*.

**6. Retiring `live_handle.rs` costs the project no evidence, and this PR must keep it that way.** Both
transcripts it carries — the rustc 1.97.1 ICE (`live_handle.rs:38-66`) and `error[E0195]`
(`:68-83`) — are already preserved outside the crate, in
`references/adapter-shapes.md:169-195` and `:363-367`, quoted in `spec/SPECIFICATION.md:4600-4615`, and
minimised under `experiments/rustc-ice-gat-foreign-trait/`. The retirement is itself a verdict finding
— *the freeze deletes the counter-example that was evidence for freezing* — and it is recorded as one,
in this story's own report, not smuggled into a `[FROZEN]` clause.

**7. The PS-34 freshness constraint makes this story's implementer a hazard to its slice-mate.**
AC-006 asks whether the documented E0195 diagnostic was **sufficient for someone meeting the port
fresh**, so the record must come from a context that has not read `live_handle.rs:68-83` first
(**T2**, and `_storymap.md`, *PS-34, and who is allowed to write it*). Whoever retires that file has
necessarily read it. Therefore: **do not re-paste either transcript** into `lib.rs`, into a commit
message, into this story's report, or anywhere the sibling's implementer will encounter it — cite
`references/adapter-shapes.md` by line instead. Freshness cannot be reconstructed after the fact; it
can only be spent.

**8. Deleting `live_handle.rs` breaks a REQUIRED gate step, so its citations are repaired here.**
`cargo xtask spec-trace` resolves every `file:line` citation in `spec/SPECIFICATION.md` and checks the
cited line sits within twelve lines of the subject (`xtask/src/spec_trace.rs:291-372`, tolerance at
`:374-378`). Four citations name `live_handle.rs` — `:4592` (`:174`), `:4605` (`:68-83`), `:4612`
(`:38-66`) and `:8095` (`:174-223`) — and `:372`'s status-table row narrates the same file. A deleted
file is a **dangling** citation, not a shifted line, and no later merge can un-break it. So this story
repairs the citations *its own diff* invalidates; the two that name `projection_store.rs`'s body lines
(`:372`'s `:270-292`, `:4590`'s `:258`) move when the bodies land and stay with
`fill-the-bodies-and-ps-34-disposition` (architecture brief **M7**, AC-A05). All of them sit in
non-normative framing prose — nearest clause headings §4.1a at `:4791` and §6.5 at `:7985`. **Repair
the citation and the count the sentence states; never a maturity marker, a clause sentence or a
falsifier.** Project AC-008 is the bar: nothing `[FROZEN]` moves.

**9. The dependency is pinned once, at the workspace, and the version choice is a recorded trade.** The
workspace manifest's `[workspace.dependencies]` block is the single source of truth for every
third-party version — `rusqlite` and `sqlx` are the precedent — and members write `lbug.workspace =
true`. `deny.toml:24` sets `wildcards = "deny"`, so `"*"` is not available; `:6` sets `yanked = "deny"`;
`:10-19` allows eight licences, and a vendored C++ tree is exactly where a ninth turns up. The stand-in
was calibrated against **0.16.1** — the last version docs.rs built — while `lib.rs:32` records that
docs.rs fails on **0.19.1**. Pick deliberately, state which, and note that any gap between the
calibrated version and the depended-on version widens decision 1's reconciliation rather than excusing
it.

**10. The gate is workspace-wide, and this commit is where the workspace starts paying.** `clippy` and
`tests` run `--locked --workspace --all-features` (`xtask/src/main.rs:115-155`),
`cargo hack check --workspace --feature-powerset --no-dev-deps` runs above them
(`:546-556`), and `cargo deny check` runs over the new subtree (`:595-601`). A feature flag saves
nothing — every one of those steps turns it on (architecture brief §7, **M6**). The four `wasm32`
steps select packages by name and do not include this crate, so nothing here is a wasm32 obligation.
The MSRV is the one thing the local gate does not check: if `lbug` or its build script will not build
at **1.97.1**, that is an ADR-0004/ADR-0029-shaped decision (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`,
and CLAUDE.md's *weigh the floor, do not obey it*), so halt and record it — never a silent bump.

**The slice this realises.** The actor is the story map's backbone actor: *an adapter author meeting
`ProjectionStore` for the third time, and the reviewer who will be asked to believe the answer*
(`_storymap.md`, Backbone, activity **B**). What that author gets from this PR is a crate whose
dependency graph, error type and auto-trait facts are the real ones, so that every claim the freeze
verdict later makes about "an unlike shape" is a claim about LadybugDB rather than about a description
of it.

## Integration contract

| | |
|---|---|
| **Archetype** | `foundation` — real in-tree substrate consumed by the capability slices that follow. No double, no fixme. |
| **Slice / milestone** | `real-adapter`. Slice-mate: **`fill-the-bodies-and-ps-34-disposition`**, implemented in the same context, merging after this one. This story merges first and compiles with `todo!()` bodies still in place (`_storymap.md`, *Why the slices are cut here*, second bullet). |
| **Mount point** | **`crates/happenstance-ladybug/src/lib.rs`** — the crate root, and this crate's composition root: the `pub mod` declarations at `:74-76` and the re-exports at `:78-81` are what decide which type universe the crate presents. Removing `pub mod stand_in;` there is what makes this a *swap* rather than an addition; leaving it and adding `lbug` beside it is the exact failure DR-2 names (two type universes in one crate). |
| **Wires into** | The workspace manifest's `[workspace.dependencies]` (version pin, `rusqlite`/`sqlx` precedent) → `crates/happenstance-ladybug/Cargo.toml:14-25` (`lbug.workspace = true`, replacing the NOTE at `:18-22`); `Cargo.lock`, because the gate runs `--locked`. The merged port at `crates/happenstance-core/src/projection.rs` (read, never edited). `crates/happenstance-ladybug/src/projection_store.rs` — `use` at `:68`, the error enum at `:158-214`, `connect()` at `:238-249`, the flavour comment and impl header at `:252-268`. `crates/happenstance-ladybug/src/live_handle.rs` — retired or re-pointed per **T1**/**T3**. `crates/happenstance-ladybug/tests/port_shape.rs:9`, `:58-61`, `:75-80` — the gate-visible instantiation. `spec/SPECIFICATION.md` — citations only. |
| **Renders surfaces** | **None.** `_design.md` records `## Surfaces — N/A`, and the human sign-off approved that no-surface determination explicitly. The one public-API delta is the one that design note anticipates: `LadybugProjectionStoreError::Driver`'s `#[from]` field and `Commit`'s `#[source]` field are re-sourced from `stand_in::Error` to `lbug::Error` on an already-`pub`, already-`#[non_exhaustive]` enum. No item is added, renamed or removed by this story — except the removal of the `stand_in` module and, under **T1**, the `live_handle` module and its two re-exports, both of which the design note and **T3** scope under this project's DR-2. |
| **Conformance rule(s)** | **None — and deliberately.** This story touches no port and adds no rule; `projection_store_conformance!` is invoked by `ladybug-fixture-and-conformance-run`, two slices later. Its compile-time observers instead are `port_shape.rs:75-80`'s `const _` block (which `cargo xtask ci` builds through `--all-targets`) and the auto-trait assertions re-homed from `stand_in.rs:332-360`. Stating this is required by the template precisely because a story that changed a port and named no rule would be a port change nothing can fail — this one changes no port. |
| **Clause(s)** | Discharges none. **Cites** PS-5 (the GAT collapse observed, `spec/SPECIFICATION.md:4868-4883`) and PS-34 (`:5546-5560`) as evidence for the verdict, and **amends neither**. Repairs the four non-normative `live_handle.rs` citations at `:4592`, `:4605`, `:4612`, `:8095` and the status-table narration at `:372`. Nothing `[FROZEN]` moves — project AC-008. |
| **Advances DoD scenario** | Initiative **DoD 7** — *"The projection suite discriminates. Two structurally unlike batch shapes pass it"*: this is the merge that turns the unlike shape from a skeleton into an implementation the suite can be pointed at, and it is a precondition for **DoD 8**'s verdict. At project grain it lands the driver/`stand_in` half of **AC-003** and of DoD item 5; the `todo!()`/allow half is the slice-mate's. |

## PR boundary

The paths this story may touch, as globs. `redkiln verify --grain story` reads the first fenced block
under this heading and fails on any file changed outside it.

```
crates/happenstance-ladybug/**
Cargo.toml
Cargo.lock
spec/SPECIFICATION.md
.bklg/from-contract-to-published-library/ladybug-projection-store/real-lbug-driver-swap/**
```

**In this PR**

- `lbug` added to the workspace `[workspace.dependencies]` at a stated version, consumed as
  `lbug.workspace = true`, replacing the NOTE at `crates/happenstance-ladybug/Cargo.toml:18-22`;
  `Cargo.lock` updated because the gate runs `--locked`.
- `Value` and `Error` re-pointed from `stand_in` to `lbug` across `projection_store.rs` (`:68`,
  `:70-103`, `:158-214`, `:238-249`) and, if it survives, `live_handle.rs`.
- `stand_in.rs` deleted; `pub mod stand_in;` removed from `lib.rs:76`; the eight calibration facts
  reconciled against the real crate and every divergence recorded; the four auto-trait assertions
  re-homed as tests against the real types.
- The GAT collapse absorbed in `projection_store.rs:265-292` if the merged port shipped `type Batch;`,
  with `port_shape.rs:61`'s higher-ranked bound collapsing with it — and the collapse recorded.
- `live_handle.rs` retired (with `lib.rs:74`, `lib.rs:78` and `port_shape.rs:9`, `:77`, `:79`) or
  re-pointed, per **T1**/**T3**.
- `lib.rs`'s *The driver is deliberately absent* section (`:24-32`) rewritten to describe the crate
  that now exists, and *Open decisions* (`:51-66`) replaced by a link to ADR-0025's atom.
- The four `spec/SPECIFICATION.md` citations this diff invalidates, repaired.

**Explicitly not in this PR**

- Any of the four `todo!()` bodies at `projection_store.rs:270-292`, and **not** the
  `#![allow(clippy::todo)]` at `lib.rs:69-72` — both belong to `fill-the-bodies-and-ps-34-disposition`,
  which takes the allow with the last body. The allow's comment stays accurate as written.
- The PS-34 disposition record (AC-006) — and see context decision **7**: this story must not make it
  unwritable.
- `crates/happenstance-core/**` and `crates/happenstance-testkit/**` — the port and the suite are
  `projection-store-freeze`'s (HS-P0010). A mismatch is surfaced upstream, never patched locally.
- `LadybugFixture`, any `projection_store_conformance!` invocation, and any `tests/` target beyond
  extending `port_shape.rs` — `ladybug-fixture-and-conformance-run`.
- `publish = false` at `Cargo.toml:12`, the licence files, the README, `xtask/src/package.rs`'s
  `PUBLISHABLE`, and the crates.io claim — `package-completeness-and-name-claim`.
- The cold-build measurement protocol, `.github/workflows/ci.yml`, the `--exclude` decision and
  `RUNBOOK.md`'s phase 11 body — `cold-build-cost-and-ci-shape`. This story *observes* what the build
  costs and hands the number on; it decides nothing about CI.
- `xtask/src/proof.rs`'s `ARTEFACTS`, the "structurally unlike" axes document and the freeze verdict.
- Any Cypher schema decision, and ADR-0025 itself — authored by `adr-0025-three-answers`, linked here.

**Merge DoD.** `cargo xtask ci` is green on the swapped tree — the whole gate, not `--fast`, because
this is the commit that first exposes `cargo deny`, the feature powerset and `spec-trace` to a new
dependency graph (`_storymap.md`, *Merge order*) — with no `stand_in` symbol anywhere in
`crates/happenstance-ladybug/`, `spec-trace` naming no file this diff deleted, and the `Send`-flavour
claim resting on an assertion the compiler checked against the real `lbug`.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
|---|---|---|
| The real driver is the dependency, pinned once | `lbug` enters the workspace `[workspace.dependencies]` at a stated version with a stated reason; the member writes `lbug.workspace = true`. `"*"` is unavailable (`wildcards = "deny"`), the version must not be yanked, and its transitive licences must fall inside the eight-entry allowlist. `cargo build -p happenstance-ladybug --locked` succeeds. | `Cargo.toml` `[workspace.dependencies]` (`rusqlite`, `sqlx` precedent); `crates/happenstance-ladybug/Cargo.toml:14-25`; `deny.toml:6`, `:10-19`, `:24` |
| The calibration table is reconciled fact by fact | Each of the eight rows is checked against the real crate: `Connection<'a>` and `Connection::new(&Database)`; `Send`/`Sync` on `Database` and `Connection`; `query(&self)`; `QueryResult` `Send` and `!Sync`; **no** `Transaction` type; the five `Error` variants; transactions as Cypher statements with one writer; every method blocking. Agreement is recorded as confirmation; divergence is recorded as a finding with the argument it disturbs named. | `crates/happenstance-ladybug/src/stand_in.rs:20-30`, `:32-40`; `projection_store.rs:1-22`, `:39-47`, `:204-213` |
| The flavour choice keeps a compiler-checked basis | The four assertions that die with `stand_in.rs` (`Database`/`Connection` `Send + Sync`, `QueryResult: Send`, driver error `Send + Sync`) are re-homed as tests against the real types. `impl SendProjectionStore for LadybugProjectionStore` stands only while they hold; if they do not, the story halts to a finding rather than switching flavours. | `stand_in.rs:332-360`; `projection_store.rs:252-258` |
| The error taxonomy is re-sourced, not reshaped | `Driver(#[from] lbug::Error)` and `Commit(#[source] lbug::Error)`; `MalformedCheckpoint`, `PositionOutOfRange` and `WriteTransactionInUse` survive untouched with their doc comments intact, because each exists for a reason no re-source changes. `#[non_exhaustive]` and every `# Errors` section stay. The enum still satisfies `core::error::Error + 'static`. | `projection_store.rs:152-214`; `.kb/decisions/0009-error-send-sync.md`; `crates/happenstance-core/src/projection.rs:90` |
| `GraphStatement` survives contact with the real value type | `parameters: Vec<(Box<str>, Value)>` now carries `lbug::Value`. If the real type does not implement the traits `GraphStatement`/`GraphWriteSet` derive (`Debug`, `Clone`, and `Default` on the write set), the derive change is a **public-surface** change and is recorded as one — not silently dropped. Parameters stay beside the text; nothing is interpolated. | `projection_store.rs:70-103`, `:105-150`; architecture brief §8 |
| `connect()` opens per call against the real `Database` | The store keeps its `Database` by value and builds a `Connection` inside each call. A borrowed database makes the store non-`'static`, which on rustc 1.97.1 ICEs rather than diagnoses. This layout does not change. | `projection_store.rs:24-37`, `:228-250`; `live_handle.rs:38-66`; `experiments/rustc-ice-gat-foreign-trait/` |
| The impl matches the merged port's batch shape | `type Batch<'a> = GraphWriteSet where Self: 'a` collapses to `type Batch = GraphWriteSet` iff the merged port shipped `type Batch;`, with `Self::Batch<'_>` spellings following. Neither the deferred-write-set decision nor the `Send` flavour is reopened by the collapse. | `projection_store.rs:261-292`; `crates/happenstance-core/src/projection.rs:92-99`; `spec/SPECIFICATION.md:4868-4883` |
| The ergonomic collapse is recorded, not just performed | `port_shape.rs:61`'s `for<'a> S::Batch<'a>: Send` becomes `S::Batch: Send`; the module's own commentary (`:50-57`) says that is PS-5's predicted relief and that rustc's suggested spelling does not compile as printed. The observation is written down where the verdict can cite it. | `crates/happenstance-ladybug/tests/port_shape.rs:39-70` |
| `port_shape.rs` is extended, never weakened | Both flavour modules stay separate (having both trait names in scope is `error[E0034]`); the `const _` block keeps instantiating generic code at whatever shapes the crate still has; `LiveHandleProjectionStore`'s two lines go only if the type goes. A bound that has to change is a finding, not a fix. | `port_shape.rs:11-14`, `:75-80`; architecture brief §9; CLAUDE.md constraint 4 |
| `live_handle.rs` is retired or re-pointed as one decision | Under `type Batch;` it cannot compile and is deleted with `lib.rs:74`, `lib.rs:78` and `port_shape.rs:9`, `:77`, `:79`; under a surviving GAT it is re-pointed at `lbug` in the same change. Either way its transcripts are preserved by citation, never re-pasted. | `live_handle.rs:1-88`, `:174-180`; `references/adapter-shapes.md:169-195`, `:363-367`; **T1**, **T3** |
| The crate root stops presenting a stand-in | `pub mod stand_in;` and its module go; `rg stand_in crates/happenstance-ladybug` returns nothing. The re-export list at `lib.rs:78-81` names only types that still exist. | `crates/happenstance-ladybug/src/lib.rs:74-81` |
| The module documentation stops describing an absent driver | `# The driver is deliberately absent` (`:24-32`) is rewritten to describe the crate that now exists — including what the build costs and the docs.rs fact, now that it is a real consequence — and `# Open decisions` (`:51-66`) is replaced by a link to ADR-0025's atom, per CLAUDE.md's link-the-atom / cite-the-record rule. `# What this skeleton establishes` (`:34-49`) is restated against the reconciliation, not left as a claim about a stand-in. | `crates/happenstance-ladybug/src/lib.rs:24-66`; ADR-0025 (authored by `adr-0025-three-answers`) |
| The specification names no file this diff deleted | `cargo xtask spec-trace` is green. The four `live_handle.rs` citations are repaired, and where a sentence states a count of impls the count is corrected with them. No maturity marker, clause sentence or falsifier is edited. | `spec/SPECIFICATION.md:372`, `:4592`, `:4605`, `:4612`, `:8095`; `xtask/src/spec_trace.rs:291-378`; **M7** |
| The gate absorbs the new dependency graph, and the cost is observed | `cargo xtask ci` green, including `cargo deny check`, the workspace feature powerset and, unchanged, the four package-selected `wasm32` steps. The wall-clock cost this adds to a cold gate is noted and handed to `cold-build-cost-and-ci-shape`; nothing about CI shape is decided here. If the crate will not build at MSRV 1.97.1, the story halts and records it rather than moving the floor. | `xtask/src/main.rs:115-155`, `:203-280`, `:546-556`, `:595-601`; `.kb/decisions/0029-msrv-raised-to-1-97-1.md`; `RUNBOOK.md:4419-4421` |

## Data and migrations

**N/A — no schema, no stored data, no migration.** Nothing in this story reads or writes a LadybugDB
database: all four port bodies remain `todo!()` until the slice-mate lands, and the only code path that
touches the driver at all is `connect()` (`projection_store.rs:238-249`), which is not exercised by any
test this story adds — `port_shape.rs` instantiates generic code without calling it (`:72-75`).

Two adjacent obligations are named here so they are not mistaken for absent work:

- **The Cypher schema is not decided here.** Label and property names for read models and for the
  checkpoint node — including whether the checkpoint is per-`ProjectionId` or one node with a property
  per id — are ADR-0025's Q1, and the statement that writes it
  (`MERGE (c:ProjectionCheckpoint {id: $id}) SET c.position = $position`) lives in the instrument at
  `live_handle.rs:203-217` and is re-authored by `fill-the-bodies-and-ps-34-disposition`.
- **`Cargo.lock` is the one generated artefact this story rewrites**, and it is in the PR boundary
  deliberately: the gate runs `--locked`, so a lock file left unregenerated fails every step rather
  than one. The `INT64` ↔ `NonZeroU64` narrowing that `MalformedCheckpoint` and `PositionOutOfRange`
  exist for (`projection_store.rs:175-202`) is a data concern of the *bodies* story; this story's only
  duty toward it is to leave both variants standing.

## Acceptance criteria

The actor throughout is the initiative's **adapter author**, on the journey *Learn when you are
finished* — from a signature that type-checks to a suite that says pass or fail and names why — and
the **evaluator/reviewer** on *Decide in one sitting*, who will be asked to believe the freeze verdict
this crate is evidence for
([`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md);
[`../../initiative.md`](../../initiative.md), *Referenced personas & journeys*). Neither of them wants
a capability; both want to stop being lied to by a skeleton. Every criterion below is that goal
crossing the whole stack — manifest, type universe, compiler, gate, specification.

| id | criterion | verification |
|---|---|---|
| **AC-001** | **GIVEN** an adapter author who was told `happenstance-ladybug` targets LadybugDB and found a hand-written stand-in where the driver should be (`stand_in.rs:1-11`), **WHEN** they clone the tree and run `cargo build -p happenstance-ladybug --locked`, **THEN** the build resolves and compiles the real `lbug` crate — pinned once in the workspace manifest's `[workspace.dependencies]` beside `rusqlite` and `sqlx` with a stated version and a stated reason, consumed by the member as `lbug.workspace = true` — at a version that is neither a wildcard nor yanked and whose transitive licences fall inside the eight-entry allowlist, with the NOTE at `crates/happenstance-ladybug/Cargo.toml:18-22` gone. | `cargo build -p happenstance-ladybug --locked`; `cargo deny check` (gate step, `xtask/src/main.rs:595-601`, against `deny.toml:6`, `:10-19`, `:24`); manifest review that the version literal lives in `Cargo.toml`'s `[workspace.dependencies]` and the member carries only `lbug.workspace = true`. |
| **AC-002** | **GIVEN** the reviewer who is being asked to believe an "unlike shape" claim that rests on eight facts a human transcribed from docs.rs for `lbug` 0.16.1, **WHEN** they read this story's implementation report against the merged crate, **THEN** each of the eight rows at `stand_in.rs:20-30` is recorded as *confirmed against the real crate* with the real item cited, or as a **divergence** naming which argument it disturbs — specifically whether a `Transaction` type exists (PS-4 and the deferred write set, `projection_store.rs:1-22`), whether `Connection::query` takes `&self` (`stand_in.rs:203-212`), and whether `Database`/`Connection` are `Send + Sync` (`projection_store.rs:252-257`) — and no row is silently dropped with the module. | Review tier: an eight-row reconciliation table in `.bklg/…/real-lbug-driver-swap/implementation-report.md`, each row citing a real `lbug` item; cross-checked against the load-bearing prose this story rewrites in `crates/happenstance-ladybug/src/lib.rs:34-49`. `cargo doc --workspace --all-features` green (gate step). |
| **AC-003** | **GIVEN** the same reviewer, being asked to believe `impl SendProjectionStore for LadybugProjectionStore` (`projection_store.rs:252-258`) after the only compiler-checked basis for it — four assertions in a module this PR deletes (`stand_in.rs:332-360`) — has gone, **WHEN** they run `cargo test --locked --workspace --all-features`, **THEN** the same four assertions run against the **real** `lbug` types and pass; **AND** if any of them does not hold, the story halts and records a BR-12-shaped finding for the freeze verdict rather than quietly switching the adapter to the weak flavour. | `crates/happenstance-ladybug/src/projection_store.rs` — a `#[cfg(test)] mod` carrying `database_and_connection_are_send_and_sync`, `query_result_is_send`, `the_driver_error_is_send_and_sync`, re-homed verbatim in intent from `stand_in.rs:342-359` (`standards/rust/61-compile-time-assertions.md`, `standards/rust/21-send-is-not-inherited.md`). Run by the gate's `tests` step (`xtask/src/main.rs:131-155`). |
| **AC-004** | **GIVEN** an application author who already matches on `LadybugProjectionStoreError` and has read its doc comments to learn *why* `MalformedCheckpoint` refuses to collapse a corrupt checkpoint into `Ok(None)`, **WHEN** they read the enum after the swap, **THEN** exactly two things have changed — `Driver`'s `#[from]` field and `Commit`'s `#[source]` field now carry `lbug::Error` — while `MalformedCheckpoint`, `PositionOutOfRange` and `WriteTransactionInUse` stand untouched with their doc comments and every `# Errors` section intact, `#[non_exhaustive]` remains, and the enum still satisfies `core::error::Error + 'static` so ADR-0009's bound is met without the port ever demanding `Send`. | A `const fn` bound assertion over `LadybugProjectionStoreError` in the same `#[cfg(test)] mod` (`standards/rust/30-error-taxonomy.md`, `standards/rust/40-public-surface-and-evolution.md`); `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; hand review of `git diff -- crates/happenstance-ladybug/src/projection_store.rs` scoped to `:152-214` showing two field types and nothing else. |
| **AC-005** | **GIVEN** a projection author who will write parameterised Cypher and was promised parameters travel *beside* the statement text so replay can `prepare` once and `execute` many times (`projection_store.rs:72-75`), **WHEN** `GraphStatement::parameters` becomes `Vec<(Box<str>, lbug::Value)>`, **THEN** `GraphStatement` and `GraphWriteSet` still compile with nothing interpolated into statement text; **AND** if the real `Value` does not implement what those types derive (`Debug`, `Clone`, and `Default` on the write set), the derive that has to go is recorded as a **public-surface change** in the report and in the rustdoc, never dropped in silence. | `cargo build -p happenstance-ladybug --locked` plus the `const _` instantiation at `crates/happenstance-ladybug/tests/port_shape.rs:75-80`; a unit test in `projection_store.rs`'s `#[cfg(test)] mod` constructing a `GraphStatement` with one parameter and pushing it onto a `GraphWriteSet` (`projection_store.rs:82-103`, `:121-150`); `standards/rust/12-manual-impls-and-derive-traps.md` for the derive question. |
| **AC-006** | **GIVEN** the adapter author whom `projection_store.rs:261-264` promised that dropping the port's batch lifetime would be *a deletion here rather than a redesign*, **WHEN** the merged `ProjectionStore` meets this crate, **THEN** the promise is kept and observed: under `type Batch;` the impl's `type Batch<'a> = GraphWriteSet where Self: 'a` collapses to `type Batch = GraphWriteSet` and `port_shape.rs:61`'s `for<'a> S::Batch<'a>: Send` collapses to `S::Batch: Send` — that collapse being PS-5's predicted ergonomic relief *observed*, written down where the freeze verdict can cite it; under a surviving GAT nothing collapses and that is recorded too. Either way `port_shape.rs` is **extended, never weakened**: both flavour modules stay separate, the `const _` block still instantiates generic code at every store the crate still has, and a bound that has to change is surfaced as a finding rather than loosened. | `cargo test -p happenstance-ladybug --test port_shape` — the `const _` block at `:75-80` and `both_batch_shapes_satisfy_the_same_generic_code` at `:82-87`; `git diff -- crates/happenstance-ladybug/tests/port_shape.rs` reviewed so the only removals are lines naming a type that ceased to exist; the recorded observation in the implementation report, cited against `spec/SPECIFICATION.md:4868-4883`. |
| **AC-007** | **GIVEN** the sibling implementer who must record PS-34's disposition from a context that has **not** read `live_handle.rs:68-83` first (`_storymap.md`, *PS-34, and who is allowed to write it*), **WHEN** this story retires or re-points `live_handle.rs` as one decision with `stand_in`'s removal (**T1**/**T3**), **THEN** neither the `error[E0195]` transcript nor the rustc 1.97.1 ICE transcript is re-pasted into `lib.rs`, a commit message, this story's report, or anywhere else the sibling's context will meet it — both are referred to by citation into `references/adapter-shapes.md:169-195` and `:363-367` — **AND** the retirement itself is recorded as a verdict finding (*the freeze deletes the counter-example that was evidence for freezing*), so the evidence survives the file. | `rg -n "E0195|internal compiler error|error: internal" crates/happenstance-ladybug .bklg/from-contract-to-published-library/ladybug-projection-store/real-lbug-driver-swap` returns citations only, never transcript text; `cargo build --locked --workspace --all-features` proves `port_shape.rs:9`, `:77`, `:79` agree with whether `LiveHandleProjectionStore` still exists; the finding recorded in the implementation report. |
| **AC-008** | **GIVEN** an adapter author who opens `lib.rs` first and currently reads *The driver is deliberately absent* — a section that will be false the moment this merges — **WHEN** they read the crate root after the swap, **THEN** there is exactly one type universe and the documentation describes the crate that exists: `pub mod stand_in;` is gone from `lib.rs:76`, `rg stand_in crates/happenstance-ladybug` returns nothing, the re-export list at `:78-81` names only types that exist, `:24-32` is rewritten to state what the build now costs and that docs.rs fails on `lbug` 0.19.1 as a live consequence, `:34-49` is restated against AC-002's reconciliation rather than left as a claim about a stand-in, and `:51-66`'s *Open decisions* is replaced by a link to ADR-0025's atom per CLAUDE.md's link-the-atom / cite-the-record rule. | `rg -n stand_in crates/happenstance-ladybug` empty (merge-gate grep, mirroring the testing brief's AC-003 row); `cargo doc --locked --workspace --all-features` and `cargo clippy … -- -D warnings` green, which is what catches an intra-doc link left pointing at a deleted module; `standards/rust/70-rustdoc-obligations.md` for the bar the rewritten sections must meet. |
| **AC-009** | **GIVEN** the reviewer who must be able to trust that `spec/SPECIFICATION.md`'s `file:line` citations resolve — the property `cargo xtask spec-trace` exists to keep, and the one a deleted file breaks in a way no later merge repairs — **WHEN** the gate runs on this diff, **THEN** `spec-trace` is green: the four citations naming `live_handle.rs` (`:4592`, `:4605`, `:4612`, `:8095`) and the status-table narration at `:372` are repaired together with any count a repaired sentence states, every edit sits in non-normative framing prose, and no maturity marker, clause sentence or falsifier differs from its state at this story's base commit. | `cargo xtask spec-trace` (gate step; resolver and twelve-line tolerance at `xtask/src/spec_trace.rs:291-372`, `:374-378`); `git diff <base>..HEAD -- spec/SPECIFICATION.md` reviewed by hand against project AC-008 and architecture brief **M7**. |
| **AC-010** | **GIVEN** every contributor whose `cargo xtask ci` will, from this commit onward, compile LadybugDB's C++ through `cxx` and `cmake`, **WHEN** the whole gate runs on the swapped tree, **THEN** it is green — `cargo xtask ci`, not `--fast`, because this is the first time `cargo deny`, the workspace feature powerset and `spec-trace` meet a new dependency graph — with the four package-selected `wasm32` steps unchanged and unaffected; **AND** the wall-clock cost this adds to a cold gate is measured and handed on to `cold-build-cost-and-ci-shape` rather than acted on here; **AND** if `lbug` or its build script will not build at MSRV **1.97.1**, the story halts and records an ADR-0004/ADR-0029-shaped finding rather than moving the floor in silence. | `cargo xtask ci` (`xtask/src/main.rs:115-155`, `:546-556`, `:595-601`); `cargo hack check --workspace --feature-powerset --no-dev-deps`; a timed cold `cargo build -p happenstance-ladybug --locked` with a fresh `CARGO_TARGET_DIR`, stamped with toolchain and machine per `references/evaluation/README.md`'s discipline, recorded in the implementation report. |

## Interaction quality

**This story renders no user-facing surface, and that determination is signed off rather than
assumed.** [`../_design.md`](../_design.md) records `## Surfaces — N/A` and carries an explicit human
approval of the no-surface finding (2026-08-12, *Sign-off*); the initiative is `userFacing: false` with
no `interaction-patterns.md` produced ([`../../initiative.md`](../../initiative.md):411). There is no
screen, no control, no route. The **STATE family** — in-place versus context-jump, non-occlusion,
preserved focus/scroll/selection, keyboard reachability — therefore has no referent here, and asserting
it would be decoration.

Two things do carry over, and neither is optional. The **COMPOSITION family's** one live member is
`_design.md`'s *Public API surface note*, which anticipates exactly this diff. And the RFC's
reversibility invariant has a real analogue in a library: a change that destroys evidence or narrows a
public type without saying so cannot be undone by a later merge. Both are already **AC rows** — this
section only says which, and how each is proved.

| Invariant (family) | Applies? | Carried by | Proved by |
|---|---|---|---|
| Presentation exists at all — the crate's own documentation surface is the thing a reader meets first, and after this diff `lib.rs:24-32` would otherwise *describe a crate that no longer exists* | **Yes** | **AC-008** | `cargo doc --locked --workspace --all-features` + `clippy -D warnings` (dangling intra-doc link); `standards/rust/70-rustdoc-obligations.md` |
| Composition / placement — the public surface delta is a **re-source of two existing fields** on an already-`pub`, already-`#[non_exhaustive]` enum, not a new, renamed or removed item ([`../_design.md`](../_design.md), *Public API surface note*) | **Yes** | **AC-004** | bound assertion + a `git diff` scoped to `projection_store.rs:152-214` |
| Composition — a derive silently dropped from `GraphStatement`/`GraphWriteSet` is a public-surface change wearing a compile fix's clothes | **Yes** | **AC-005** | `port_shape.rs`'s `const _`; the unit construction test; recorded in the report |
| Reversibility — evidence deleted with a file cannot be restored by a later commit; a citation can | **Yes** | **AC-007**, **AC-009** | the transcript grep; `cargo xtask spec-trace` |
| Non-occlusion (analogue) — this story must not occlude the sibling's PS-34 freshness by putting the old transcript where that context will read it | **Yes** | **AC-007** | the transcript grep over the crate *and* this story's own artifacts |
| Hierarchy — the crate root presents one type universe, not two side by side (DR-2) | **Yes** | **AC-008** | `rg stand_in crates/happenstance-ladybug` empty |
| Named anti-pattern — *weaken the instrument to make the build green* (architecture brief §9) | **Yes** | **AC-006** | `git diff` over `port_shape.rs`: removals only where a type ceased to exist |
| Named anti-pattern — *two type universes in one crate* (DR-2, **T3**) | **Yes** | **AC-008**, **AC-004** | as above; the shared error enum is what forces them to move together |
| STATE family (in-place vs context-jump, focus/scroll/selection, keyboard reachability, transience, density budget) | **No** | — | no surface exists to hold them ([`../_design.md`](../_design.md), *Surfaces*) |

## Error conditions

| id | condition | required behaviour |
|---|---|---|
| **EC-001** | `lbug` fails to build on a contributor's machine — no C++ toolchain, no `cmake`, or a `cxx` codegen failure | The failure is reported with the toolchain prerequisite named, in the crate README-adjacent prose this story rewrites (`lib.rs:24-32`) and in the report. It is **not** worked around by re-introducing a stand-in, by a feature flag (architecture brief §7: every gate step turns features on), or by `--exclude` — the `--exclude` lever belongs to `cold-build-cost-and-ci-shape`. |
| **EC-002** | The real `lbug::Database` or `lbug::Connection` is **not** `Send + Sync` | **Halt.** `impl SendProjectionStore` is no longer honest. Record a finding for the freeze verdict; do not switch flavours in passing and do not delete the failing assertion (context decision **2**; `projection_store.rs:252-257`). |
| **EC-003** | `lbug::Error` is not `Send`, so `port_shape.rs:58-61`'s `S: SendProjectionStore<Error: Send>` runner no longer instantiates | **The failure is the finding.** ADR-0009 keeps the port at `core::error::Error + 'static` (`.kb/decisions/0009-error-send-sync.md`; `crates/happenstance-core/src/projection.rs:90`), so nothing in the contract is violated — what is violated is a *caller's* assumption, which is precisely what the instrument exists to expose. Do not relax the bound to make it compile (architecture brief §9). |
| **EC-004** | The merged port shipped neither `type Batch;` nor the pre-freeze GAT — a third shape | **Halt and surface to `projection-store-freeze` (HS-P0010).** Do not work around an upstream port locally (architecture brief AC-A01; `_grounding.md`, *The dependency this project cannot see past*). |
| **EC-005** | The real `lbug::Value` does not implement a trait `GraphStatement`/`GraphWriteSet` derive | Record the derive removal as a **public-surface change** (AC-005) with the caller cost stated; do not add a manual impl that papers over a semantic difference (`standards/rust/12-manual-impls-and-derive-traps.md`). |
| **EC-006** | `cargo deny check` rejects the new subtree — a licence outside `deny.toml:10-19`'s eight, a yanked version, or a duplicate major | Fix by version choice where possible. Adding a ninth licence to the allowlist is a **recorded trade with a named reason**, never a silent edit; a vendored C++ tree is exactly where one turns up (context decision **9**). |
| **EC-007** | `lbug` or its build script does not build at MSRV **1.97.1** | **Halt and record.** Raising the floor is an ADR-0004/ADR-0029-shaped decision (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`; CLAUDE.md, *weigh the floor, do not obey it*), and — as ADR-0029 already found — a dependency's build script is exactly the thing `cargo hack --rust-version` cannot protect against. Never a silent bump. |
| **EC-008** | `cargo xtask spec-trace` is still red after the repair, because a repaired citation now sits outside the twelve-line tolerance | Re-point the citation at the line the sentence actually attributes to it (`xtask/src/spec_trace.rs:291-378`). Never widen the tolerance, never delete the citation, never edit the clause sentence to fit the line. |
| **EC-009** | Deleting `stand_in.rs` leaves a `todo!()`-free module but `cargo clippy -D warnings` now fires on the four remaining `todo!()` bodies | It must not: `#![allow(clippy::todo)]` at `lib.rs:69-72` **stays** in this PR and leaves with the last body in `fill-the-bodies-and-ps-34-disposition`. If clippy fires anyway, the allow's scope changed and that is a diff error, not a licence to fill a body here. |

## Non-functional

| id | requirement | why it is here |
|---|---|---|
| **NF-001** | The cold-build cost is **measured and handed on**, not optimised. One genuinely cold `cargo build -p happenstance-ladybug --locked` (fresh `CARGO_TARGET_DIR`) stamped with toolchain and machine, recorded in this story's report. | AC-009 belongs to `cold-build-cost-and-ci-shape`; this story is where the number first becomes observable, and an unrecorded observation is a re-measurement later. `references/evaluation/README.md` is the discipline for anything kept as evidence. |
| **NF-002** | **No gate step is skipped, excluded, downgraded or feature-gated to make this merge green.** `cargo xtask ci` in full is the bar, not `--fast`. | `_storymap.md`, *Merge order*; architecture brief §7 — a feature flag saves nothing because every step runs `--all-features`. |
| **NF-003** | The build is reproducible: `Cargo.lock` is regenerated and committed in the same commit, because every gate step runs `--locked`. | `xtask/src/main.rs:115-155`; a stale lock fails every step rather than one, which reads as a dependency problem and is not. |
| **NF-004** | No evidence is destroyed. Every transcript, calibration fact or counter-example that leaves the crate leaves behind a live citation to where it still lives. | Context decisions **1**, **6**; `references/adapter-shapes.md:169-195`, `:363-367`; `experiments/rustc-ice-gat-foreign-trait/`. |
| **NF-005** | Rustdoc obligations hold on every item whose documentation this diff touches — the rewritten `lib.rs` sections and the re-sourced error variants included. | `standards/rust/70-rustdoc-obligations.md`, `standards/rust/00-prime-directives.md`; the gate's `docs` step and the `--no-default-features` doc build. |
| **NF-006** | Dependency hygiene: one version of `lbug` in the graph, pinned at the workspace, no wildcard, no duplicate major introduced. | `Cargo.toml`'s `[workspace.dependencies]` comment states the rule; `standards/rust/50-dependency-hygiene.md`; `deny.toml:24`. |

## Implementation notes (non-prescriptive)

Not instructions — the traps that are cheap to avoid and expensive to discover.

- **Read `preflight-and-unlike-axes`'s recorded result before writing a line.** Which branch of context
  decision **5** you are on changes how much of this story exists, and guessing costs a rewrite of
  `projection_store.rs:261-292`, `port_shape.rs:61` and possibly the whole of `live_handle.rs`.
- **A plausible order.** Add the dependency and let it build first (that is where EC-001, EC-006 and
  EC-007 surface, and they are the ones that halt the story). Then re-point `Value`/`Error` and watch
  `live_handle.rs` break — that break *is* **T3**'s argument, and seeing it is worth more than
  pre-empting it. Retire or re-point per **T1**. Delete `stand_in.rs` last, after its four assertions
  have been re-homed and its eight facts reconciled, so nothing is lost in the window where the module
  is gone and its output has not landed.
- **Re-home the assertions in-crate, not in a new `tests/` target.** The PR boundary admits only
  `port_shape.rs` among test targets; `projection_store.rs`'s own `#[cfg(test)] mod` is where
  `stand_in.rs:332-360` lived and is the natural home. `standards/rust/61-compile-time-assertions.md`
  is the atom for the `const fn assert_send::<T>()` shape.
- **`rg stand_in crates/happenstance-ladybug` is the cheapest possible check** and it is the merge-DoD
  bar; run it before the gate, not after.
- **Do not import both `ProjectionStore` and `SendProjectionStore` into one module** — CLAUDE.md
  constraint 4, and `port_shape.rs:11-14` already explains the `error[E0034]` it produces. The two
  flavour modules exist for that reason; keep them.
- **Where the divergence log goes.** AC-002's reconciliation and AC-006's collapse observation belong
  in this story's implementation report, which is what `freeze-verdict-document` reads. They do **not**
  belong in `spec/SPECIFICATION.md` — nothing here amends a clause.
- **Two atoms worth pulling, and no more:** `standards/rust/25-what-removes-send-and-sync.md` if an
  auto-trait assertion fails, and `standards/rust/50-dependency-hygiene.md` before touching
  `[workspace.dependencies]`. Do not load the corpus (CLAUDE.md, *House style*).
- **The `#![allow(clippy::todo)]` stays.** Its comment already names the phase that removes it, and
  removing it here strands four `todo!()` bodies against a `-D warnings` gate
  (`standards/rust/90-skeletons-and-todo.md`).

## Tests and CI (merge gate)

Grounded in the project testing brief's tier table ([`../_decomposition.md`](../_decomposition.md),
*Testing brief*, AC-003 row and *Test mix, summarised by tier*). Note what is **not** here: this story
adds no conformance invocation and no fixture, because the suite is two slices away.

| tier | command / path | proves |
|---|---|---|
| Static — formatting and lints | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` (`xtask/src/main.rs:115-129`) | AC-004, AC-008 — a dangling intra-doc link to the deleted module, an unused import left by the re-source, a derive that no longer holds |
| Static — the one-line bar | `rg -n stand_in crates/happenstance-ladybug` returns nothing | AC-008; DR-2's stronger reading, and the merge DoD |
| Static — transcript containment | `rg -n "E0195\|internal compiler error" crates/happenstance-ladybug .bklg/…/real-lbug-driver-swap` returns citations only | AC-007 — the sibling's PS-34 freshness is not spent by this diff |
| Static — specification integrity | `cargo xtask spec-trace`; `git diff <base>..HEAD -- spec/SPECIFICATION.md` reviewed by hand | AC-009; project AC-008 (`xtask/src/spec_trace.rs:291-378`) |
| Unit / in-crate | `cargo test --locked --workspace --all-features -- --show-output` over `crates/happenstance-ladybug/src/projection_store.rs`'s `#[cfg(test)] mod` — the three re-homed auto-trait tests, the error-bound assertion, the `GraphStatement`/`GraphWriteSet` construction test | AC-003, AC-004, AC-005 |
| Compile-time / integration | `cargo test -p happenstance-ladybug --test port_shape` — `crates/happenstance-ladybug/tests/port_shape.rs:75-80`'s `const _` block and `:82-87`'s named test | AC-006 — generic code still instantiates at every batch shape the crate has, with no bound loosened |
| Integration — the real driver | `cargo build -p happenstance-ladybug --locked`; `cargo doc --locked --workspace --all-features` | AC-001, AC-005, AC-008 — this is the step that first pays the C++ build |
| Supply chain | `cargo deny check` (`xtask/src/main.rs:595-601`); `cargo hack check --workspace --feature-powerset --no-dev-deps` (`:546-556`) | AC-001, AC-010; EC-006 |
| Whole gate (merge bar) | **`cargo xtask ci`** — fmt, clippy, tests, four `wasm32` steps, docs, `spec-trace`, the `--no-default-features` doc build, `cargo package --list`, then `cargo-hack`, `cargo-deny` and nightly `docsrs` where the tool resolves | AC-010, and every row above; **not `--fast`** ([`../_storymap.md`](../_storymap.md), *Merge order*; testing brief, *Merge-gate commands*) |
| Process / measurement | one cold `cargo build -p happenstance-ladybug --locked` with a fresh `CARGO_TARGET_DIR`, toolchain and machine stamped, recorded in the implementation report | NF-001; hands `cold-build-cost-and-ci-shape` its starting number |
| Process / review | the eight-row reconciliation table and the GAT-collapse observation in the implementation report | AC-002, AC-006 — neither is a test-framework assertion, and the testing brief says so of its own process tiers |

## Risks and coupling (PR-scoped)

- **The upstream port is the one thing this PR cannot see past.** Everything in context decision **5**
  branches on what `projection-store-freeze` merged. Mitigation: the preflight is a hard dependency of
  the slice, and EC-004 halts rather than improvises. *Coupling: HS-P0010, read-only.*
- **The real `lbug` may not match the calibration table**, and the divergence may land on one of the
  three load-bearing facts. Mitigation: AC-002 makes reconciliation an output rather than a side
  effect, and AC-003 / EC-002 give the `Send + Sync` divergence a defined halt.
- **This is the commit where a workspace-wide build cost lands on every contributor.** Mitigation:
  NF-001 measures it and AC-010 keeps the gate whole; the *decision* is deliberately elsewhere, so the
  risk here is scope creep into `cold-build-cost-and-ci-shape`, not the cost itself.
- **Deleting `live_handle.rs` deletes a counter-example that argued against the freeze.** Mitigation:
  NF-004 and AC-007 — cited, not re-pasted; recorded as a verdict finding rather than absorbed.
  *Coupling: `freeze-verdict-document` consumes this finding.*
- **The PS-34 freshness hazard runs the other way through the same file.** Whoever writes this story
  has read the transcript; the sibling must not. Mitigation: AC-007's grep is over this story's own
  artifacts as well as the crate. *Coupling: `fill-the-bodies-and-ps-34-disposition`, one-directional
  and unrecoverable if breached.*
- **A dangling `spec/SPECIFICATION.md` citation is unrepairable later** — a deleted file is not a
  shifted line. Mitigation: AC-009 repairs the citations *this* diff invalidates, and only those; the
  two that move when the bodies land stay with the slice-mate (**M7**).
- **The temptation to fill one body "while we are in here."** A `todo!()` body type-checks against any
  signature, which is exactly why DR-1 exists. Mitigation: the PR boundary's *Explicitly not in this
  PR* list, and EC-009.
- **Supply-chain surprise from a vendored C++ tree** (licence, yanked version, duplicate major).
  Mitigation: EC-006, and `cargo deny check` as a gate step rather than a courtesy.

## Dependencies

**Blocks on**

- **`adr-0025-three-answers`** — the only edge, and it is real rather than ceremonial: AC-008 replaces
  `lib.rs:51-66`'s *Open decisions* with a **link to ADR-0025's atom**, and CLAUDE.md's two-places rule
  means that link must resolve to an atom under `.kb/decisions/` that exists. It does not exist in this
  tree yet (`ls .kb/decisions/` stops at `0016` and `0029`), so writing this story before that one
  produces either a dangling link or a hand-written atom — the exact failure CLAUDE.md names and commit
  `0269720` reverted. `adr-0025-three-answers` in turn blocks on `preflight-and-unlike-axes`, which is
  what makes context decision **5**'s branch knowable.

**Unlocks**

- **`fill-the-bodies-and-ps-34-disposition`** — its slice-mate, implemented in the same context and
  merging immediately after. It inherits a crate whose error type, `Value` type and auto-trait facts are
  the real ones, and it takes the four `todo!()` bodies, the `#![allow(clippy::todo)]` and the six
  remaining `SPECIFICATION.md` citations with it.
- Transitively, everything downstream of that: `ladybug-fixture-and-conformance-run`,
  `read-your-own-writes-projection`, `package-completeness-and-name-claim`,
  `cold-build-cost-and-ci-shape` (which starts from NF-001's number), and `freeze-verdict-document`
  (which consumes AC-002's divergences, AC-006's collapse observation and AC-007's retirement finding).

## Anchors (progressive disclosure)

Linked, not pasted. The *Context pack* above is the must-read core; open these when the table says to.

| anchor | why it is load-bearing | when to open | serves |
|---|---|---|---|
| `crates/happenstance-ladybug/src/stand_in.rs` | Carries the eight calibration facts at `:20-30` and the four auto-trait assertions at `:332-360` — the entire output of the instrument this PR deletes. Its `:32-40` note on *what a stand-in cannot reproduce* is why reconciliation is not a formality. | First, and before deleting anything | AC-002, AC-003 |
| `crates/happenstance-ladybug/src/projection_store.rs` | The edit surface: `use` at `:68`, `GraphStatement` at `:70-103`, the error enum at `:152-214`, `connect()` at `:228-250`, the flavour comment and impl header at `:252-268`, the GAT-collapse comment at `:261-264`. | Before the first line of the swap | AC-003, AC-004, AC-005, AC-006 |
| `crates/happenstance-ladybug/src/lib.rs` | The mount point and this crate's composition root — `pub mod` at `:74-76`, re-exports at `:78-81`, and the three doc sections AC-008 rewrites (`:24-32`, `:34-49`, `:51-66`). | When removing `pub mod stand_in;` and again when rewriting the prose | AC-008 |
| `crates/happenstance-ladybug/tests/port_shape.rs` | The gate-visible instrument. `:11-14` explains the `error[E0034]` that forces two flavour modules; `:39-70` is the PS-5 commentary the collapse observation must be written against; `:75-80` is the `const _` block. | Before touching any bound, and again to confirm removals only where a type went | AC-006 |
| `crates/happenstance-ladybug/src/live_handle.rs` | The module **T1**/**T3** retire or re-point. `:38-66` and `:68-83` are the two transcripts AC-007 forbids re-pasting; `:91-92` is the shared-error-enum import that makes this one decision; `:177-180` is the borrowed `Batch` that cannot survive `type Batch;`. | When the re-source breaks it — which is the moment **T3**'s argument becomes visible | AC-007 |
| `crates/happenstance-core/src/projection.rs` | The merged port. `:13-30` states the one-transaction invariant; `:90` is ADR-0009's error bound; `:92-99` is the batch shape everything branches on. **Read-only** — a mismatch is surfaced upstream. | Immediately after the preflight result, before choosing the branch | AC-006, AC-004 |
| [`../_decomposition.md`](../_decomposition.md) | Architecture brief §1 (seams), §6 (**T1**, **T2**, **T3**), §7 (why a feature flag saves nothing), §9 (do not weaken the instrument); testing brief AC-003 row. | When the port shape is ambiguous, and before any decision to "just make it compile" | AC-006, AC-007, AC-010 |
| [`../_storymap.md`](../_storymap.md) | *Why the slices are cut here* (second bullet) is why this merges with `todo!()` bodies intact; *PS-34, and who is allowed to write it* is the freshness constraint AC-007 enforces. | Before writing the report or any commit message | AC-007 |
| [`../_design.md`](../_design.md) | The signed-off no-surface determination and the *Public API surface note* that anticipates the error re-source. Binding on the *Interaction quality* section above. | Before claiming this story changes no public surface | AC-004, AC-005 |
| `.kb/decisions/0009-error-send-sync.md` | Why the port stops at `core::error::Error + 'static` and never demands `Send` — which is what makes EC-003 a *finding* rather than a contract violation. | The moment a `Send` bound on the error looks like the fix | AC-004 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | The precedent for EC-007: the floor moved for a dependency's build script, deliberately and in an ADR. Also why `cargo hack --rust-version` cannot protect it. | Only if the crate will not build at 1.97.1 | AC-010 |
| `.kb/decisions/0001-async-port-flavours.md` | Why there is a `Send` flavour to choose at all, and why `#[async_trait]` is never the remedy for a blocking driver. | If EC-002 fires, or if the blocking bridge starts to look like this story's problem | AC-003 |
| `references/adapter-shapes.md` | Where both retired transcripts survive — `:169-195` (ICE) and `:363-367` (E0195). The citation AC-007 requires in place of a paste. | When writing the retirement finding | AC-007 |
| `xtask/src/spec_trace.rs` | The resolver (`:291-372`) and the twelve-line tolerance (`:374-378`) — what "repaired" has to mean, and why widening it is not an option. | Before repairing the first citation | AC-009 |
| `xtask/src/main.rs` | The gate, defined once: clippy and tests at `:115-155`, feature powerset at `:546-556`, `cargo deny` at `:595-601`. Reading it is how you learn a feature flag saves nothing. | Before proposing any way to make the build cheaper | AC-010, AC-001 |
| `deny.toml` | `:6` yanked, `:10-19` the eight-licence allowlist, `:24` `wildcards = "deny"` — the three constraints on the version choice. | While choosing the `lbug` version | AC-001 |
| `standards/rust/50-dependency-hygiene.md` | The house rule for adding a dependency and pinning it once. | Before editing `[workspace.dependencies]` | AC-001 |
| `standards/rust/61-compile-time-assertions.md` | The shape the re-homed auto-trait assertions must take, and the wrong implementation each rejects. | While re-homing `stand_in.rs:332-360` | AC-003 |
| `standards/rust/30-error-taxonomy.md` | What a re-sourced `#[from]` may and may not change on a `#[non_exhaustive]` error enum. | While editing `projection_store.rs:152-214` | AC-004 |
| `standards/rust/70-rustdoc-obligations.md` | The bar the rewritten `lib.rs` sections must meet — this crate's only reader-facing surface. | While rewriting `:24-32`, `:34-49`, `:51-66` | AC-008 |
| `standards/rust/90-skeletons-and-todo.md` | Why the scoped allow stays until the last body goes, and what a skeleton is for. | If deleting `#![allow(clippy::todo)]` starts to look tidy | EC-009 |
| `references/evaluation/README.md` | The discipline any kept measurement must meet — dated, pinned to a commit, superseded rather than edited. | While recording the cold-build number | NF-001 |
| `RUNBOOK.md` | `:4406-4425` — phase 11's work list, whose second box is *"Add `lbug` and measure the cold build"*, and `:602`'s "6, re-tested 11" row that PS-34 belongs to. | For the roadmap framing the report should cite | AC-010, AC-007 |

## Clarifications resolved during spec

1. **The AC set is exactly the ten the first pass decided** — AC-001 … AC-010, one per load-bearing row
   of *Behavior and interfaces*, with two pairs merged where a single user-visible outcome covers both
   (the crate-root cleanup and the documentation rewrite are one thing a reader meets, AC-008; the
   whole-gate green and the observed build cost are one thing a contributor meets, AC-010). Nothing was
   added or dropped, and the ledger matches.
2. **`_design.md` records no surface, so the *Interaction quality* section reports which invariants have
   no referent rather than inventing UI-shaped ACs.** The COMPOSITION family survives in the one form
   the signed-off design actually names — the public API surface note — and it is carried by AC-004 and
   AC-005 as table rows, not as prose bullets, so `redkiln verify` can extract them.
3. **The re-homed auto-trait assertions live in `projection_store.rs`'s own `#[cfg(test)] mod`, not a
   new `tests/` target.** The PR boundary's *Explicitly not in this PR* list admits only `port_shape.rs`
   among test targets, and `stand_in.rs:332-360` is where they lived; there is no third option that is
   both in-boundary and gate-visible.
4. **ADR-0025's atom is cited as a link target, never as an anchor path**, because
   `.kb/decisions/0025-*.md` does not exist in this tree yet — `adr-0025-three-answers` authors it
   through `.kb/_intake/`, which is why that story is this one's only `depends_on`. The anchors table
   cites only files that exist today.
5. **`#![allow(clippy::todo)]` stays in this PR**, and EC-009 says so explicitly, because "delete the
   stand-in" and "delete the allow" read as the same tidy-up and are not: the allow leaves with the last
   body, in the slice-mate.
6. **AC-002's reconciliation is an output, not a check.** The first pass's context decision **1** made
   it a duty; this half gives it a home (the implementation report) and a shape (eight rows, each
   confirmed or diverging with the argument it disturbs named), so `freeze-verdict-document` has
   something to cite rather than an absence to interpret.
7. **EC-002 and EC-003 are deliberately *halts* rather than remedies.** Both have an obvious local fix —
   switch flavours; relax the bound — and both fixes would destroy the finding the project exists to
   produce (architecture brief §9; context decisions **2** and **3**).
