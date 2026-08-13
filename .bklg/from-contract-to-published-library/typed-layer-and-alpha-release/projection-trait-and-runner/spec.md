---
item: HS-S0026
stage: spec
created: 2026-08-12T13:46:23.496Z
updated: 2026-08-12T13:46:23.496Z
template_sig: 87bbf1d0
rendered_sig: 5d4ab9c1
---

# Spec — The application-facing Projection trait and its streaming runner

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` |
| This spec | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/projection-trait-and-runner/spec.md` |
| Key brief — architecture, ux, testing, deployment (one file) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` |
| Key brief — **binding** API surface design (human-approved, DoD 5) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` |
| Story map (slices, merge order, coverage) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md` |
| Cross-project dependency (the fixture, and the port's freeze) | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` |
| Roadmap pointer — phase 7's goal, work list and exit criteria | `RUNBOOK.md:3971-4102` |
| Roadmap pointer — PS-33 evaluated at this phase's exit *and nowhere else* | `RUNBOOK.md:408-413` |

Traces to project **AC-006** (*an application writes a projection against decoded
events*). Depends on **`codec-and-feature-forwarding`** (HS-S0022, M3) — the runner
decodes, so it cannot exist before a `Codec` does. Blocks
**`projection-clause-verdicts`** (HS-S0027), **`polling-cost-measurement`**
(HS-S0028), **`edge-flavour-and-wasm-claim`** (HS-S0031) and
**`defect-log-and-macros-verdict`** (HS-S0032).

## One-line PR slice

Land the application-facing `Projection` trait and its runner in
`crates/happenstance/`, layered over `crates/happenstance-core/src/projection.rs`'s
checkpoint port — nominating events with `Query`, decoding them, opening `begin` and
committing read-model write + checkpoint in the single `commit` (`:126`), advancing
past the inclusive `from` (`:104-106`), and **streaming, never `collect`ing** (AC-A05)
— with its feature gating stated explicitly rather than inherited.

## Executive summary

This PR is the **last of the five vocabulary bullets** on the crate root
(`crates/happenstance/src/lib.rs:49-51`) to become a real item, and the first place in
the workspace where anything actually *runs* an `EventStore` into a `ProjectionStore`.
It lands four things and mounts all four:

1. `happenstance::Projection` — the trait an application implements: its own domain
   enum as `type Event`, its own store as `type Store`, a `ProjectionId`, a `Tags`
   scope, and one `apply` that writes a decoded event into the adapter's open batch;
2. `happenstance::run_projection` — the streaming runner: derive the query, read,
   decode, apply in chunks of a caller-supplied `NonZeroUsize`, and hand batch +
   position back through the port's **single** `commit`;
3. `Progressed` and `ProjectionError`, the two `#[non_exhaustive]` types that make
   "how far did it get" and "why did it stop, and where" representable rather than
   inferable;
4. the **`unstable-projection` feature** — off by default — which is the first honest
   statement in a manifest that the port beneath this surface is not frozen.

**Delta against the tree at HEAD**, so the implementer knows what is new versus what
is being changed in place:

- `crates/happenstance-core/src/projection.rs` is **139 lines and holds no code that
  runs.** It has `ProjectionId` (`:42-71`), and `ProjectionStore` with exactly four
  methods — `checkpoint` (`:110`), `begin` (`:117`), `commit` (`:126`) and `rollback`
  (`:138`). ADR-0007 says `happenstance-core` *"keeps the checkpoint pump"*
  (`.kb/decisions/0007-projection-runner-decodes.md:53-59`); at HEAD **there is no
  pump function there to keep.** That is not a defect to fix in this story — it is the
  observation PS-33's falsifier was written to consume, and this story's job is to
  produce it truthfully rather than to manufacture a caller for it (see the Context
  pack, decision 2).
- `crates/happenstance/src/lib.rs:35-52` still carries five *"Planned, and specified in
  `spec/SPECIFICATION.md`"* bullets. M2, M3 and this story between them delete the
  heading; the fifth bullet — *"The typed projection runner — decoded events, over the
  checkpoint pump that stays in the contract crate"* — is **this story's** to turn into
  a link, in place, in region 4 of `crate-root-rustdoc`.
- **No manifest in the workspace declares `unstable-projection`**, and
  `CHANGELOG.md:19-22` already asserts that *"`ProjectionStore` ships behind an
  off-by-default `unstable-projection` feature."* Today that is a claim in an
  unpublished file; two milestones later `publish-0-2-0-alpha-1` makes it a claim a
  stranger can check. This story is the one that makes the feature exist, so that the
  changelog line becomes true rather than corrected.
- `crates/happenstance/Cargo.toml` has **no `[package.metadata.docs.rs]` block**. If
  `codec-and-feature-forwarding` (the M3 dependency) has already added it, this story
  inherits it; if for any reason it has not, this story adds it, because an
  off-by-default item with no `doc_cfg` badge is `_design.md`'s anti-pattern 15
  verbatim.

What it deliberately does **not** land: the PS-33/PS-27/PS-30/PS-18 *verdicts* and
their `spec/SPECIFICATION.md` edit (slice-mate `projection-clause-verdicts`,
HS-S0027), and the polling-cost measurement (slice-mate `polling-cost-measurement`,
HS-S0028). Both consume what this story builds; neither is written here.

## Context pack

Everything below is a decision already taken. Internalize it before writing code; open
an anchor only when its row says to.

**1. The runner splits at the decode boundary, and the half that decodes is this
one.** ADR-0007 corrected ADR-0006's claim that the whole runner never decodes: a
runner that hands back opaque `Bytes` *"is the easy half of a two-part job sold as the
whole job"* (`.kb/decisions/0007-projection-runner-decodes.md:41-49`). Three shape
decisions ride with the split and none is reopenable here
(`.kb/decisions/0007-projection-runner-decodes.md:61-69`): **a projection nominates
its events with `Query`** — the same type a decision model uses, so there is no second
filtering vocabulary; **`Projection::Store` is an associated type**, making a
projection that spans two stores unrepresentable rather than merely undocumented,
because there is no cross-store transaction and one that appeared to work would
misrepresent the invariant `ProjectionStore` exists to defend; and **checkpoints stay
per `(store, ProjectionId)`**, so two stores projecting the same events sit at
different positions and callers reading both must tolerate the skew.

**2. ADR-0007's falsifier is live while this story runs, and the honest move is to
observe it, not to satisfy it.** The falsifier: *"if the core pump has acquired no
caller but the typed one when phase 7 exits, collapse it upward and supersede this
decision"* (`.kb/decisions/0007-projection-runner-decodes.md:23-25`). At HEAD the
contract crate holds no pump function at all, so the count this story hands to
HS-S0027 is a fact about a thing that does not exist. **Do not write a pump into
`happenstance-core` to make the ADR look right** — that would be an edit under
`crates/happenstance-core/src/**` for a convenience discovered mid-implementation,
which AC-A02 forbids outright, and it would fabricate the evidence PS-33 exists to
gather (`spec/SPECIFICATION.md:5529`; `RUNBOOK.md:408-413`). Record what is true; the
verdict — name an independent caller, or write the superseding ADR that collapses the
pump upward — is HS-S0027's to write.

**3. The read-model write and the checkpoint move together, in one `commit`, and the
port deliberately offers no other way.** *"If the read-model write commits and the
checkpoint write does not, a restart replays events that were already applied; if the
checkpoint commits first, a crash silently skips events. Neither is acceptable, and no
amount of ordering or retrying fixes it"* (`crates/happenstance-core/src/projection.rs:13-30`).
The consequence for the runner is mechanical: `begin` → apply the chunk into the batch
→ `commit(batch, id, position)` → repeat. **There is no `set_checkpoint`, and looking
for one is the error the port's own doc pre-empts** (`:20-26`).

**4. The runner streams. This is the reason the port has the shape two ADRs fought
for.** `EventStore::read` returns the stream at the **top level** and is not `async`
precisely so an adapter can stream a million-event replay without buffering it
(`crates/happenstance-core/src/store.rs:103-123`). A runner that calls `collect`
defeats that and makes E2E-25's chunked rebuild unwritable (AC-A05,
`_decomposition.md:391-398`). Note the asymmetry this story must not smooth over: the
**command** path may buffer — `read_decision_model` collects into a `Vec`
(`crates/happenstance-core/src/store.rs:321`) because DCB queries are narrow by
construction. The **projection** path may not. Same crate, opposite rule, and the
reason is the size of what is being read.

**5. Resuming means advancing *past* the checkpoint, and `None` at either end is a
real state.** `ProjectionStore::checkpoint` returns `Option<SequencePosition>` where
`None` means *never run*; `ReadOptions::from` is **inclusive**
(`crates/happenstance-core/src/projection.rs:101-106`); and `SequencePosition::next`
returns `Option` whose `None` arm is real — it is `checked_add`, not `saturating_add`,
specifically so a consumer resuming from the last representable position is told it
has run out of key space instead of re-reading it forever
(`crates/happenstance-core/src/event.rs:265-280`). Feeding a checkpoint straight into
`from` re-applies the last event on every run; taking `next().unwrap()` reintroduces
the bug that comment exists to describe. **Positions may gap**, so nothing computes
`head − checkpoint` as a lag and no test asserts literal positions (`CLAUDE.md`, *The
rule that matters*).

**6. The batch borrows from its store, is therefore not `'static`, and cannot cross a
`tokio::spawn`.** `type Batch<'a> where Self: 'a` exists because *"a transaction cannot
outlive its connection"* (`crates/happenstance-core/src/projection.rs:92-99`). The
architecture brief states the consequence in advance so it is not discovered as a
compiler error: **a fan-out runner cannot move a batch into a task**
(`_decomposition.md:697-700`). This story ships the single-projection runner; PS-30's
fan-out subject is not built here, and saying so is part of the evidence HS-S0027
needs.

**7. The feature gating is stated by this story, not inherited from anyone.**
`_design.md` decided it: **`unstable-projection`, off by default**, forwarded to
`happenstance-core/unstable-projection` **if** HS-P0010 lands that feature, and gating
only this crate's items if it does not (`_design.md`, *Shape decision*, projection
runner gating row). The reason is not caution for its own sake — the port beneath is
*"not yet frozen"* in its own module doc (`crates/happenstance-core/src/projection.rs:1-11`),
and PS-3's still-open verdict (`RUNBOOK.md:3924-3928`) is exactly the possibility that
it stays that way. A feature only ever **adds** (RS-51-1,
`standards/rust/51-features-and-no-std.md:12`): turning `unstable-projection` on must
take nothing away, and the crate root must read completely with it off, which is its
default state (`_design.md`, *States*, *Feature off*).

**8. These four items make no semver promise, and the feature name is how that promise
is made.** `Projection`, `run_projection`, `Progressed` and `ProjectionError` are
`pub`, the two types are `#[non_exhaustive]`, and the stability column says
*"explicitly none"* (`_design.md`, *Visibility and stability*). `Progressed` is
`{ through, applied }` — a named struct rather than a tuple, because a tuple freezes
the arity of the thing a later observability pass most wants to grow (RS-13-4,
`standards/rust/13-sealing-and-exhaustiveness.md:144`).

**9. A decode failure has to have a home, and at this layer it does.** E2E-26
falsifies ADR-0007's pump signature on exactly this point: the pump types its
callback's error as a **projection store** error, *"so a decode failure has no
representable home: the application must forge one into the adapter's
`#[non_exhaustive]` error enum, which belongs to the adapter, or panic"*
(`spec/E2E-CASES.md:677-696`). The typed runner escapes that because `CodecError` is
**concrete** — the design chose a concrete error precisely so `CommandError`,
`Boundary::absorb`, `Decision` and *the runner* do not each grow a third type
parameter (`_design.md`, *Shape decision*, `Codec::Error` row). So `ProjectionError`
carries two type parameters and a decode arm, and the runner reports **the position it
failed at** with the checkpoint sitting at the last good position.

**10. The transactional test's fixture is another project's deliverable, and writing a
throwaway is not the workaround.** `MemoryProjectionStore` behind the `memory` feature
is `projection-store-freeze`'s own AC-012
(`.bklg/from-contract-to-published-library/projection-store-freeze/project.md`,
AC-012). Writing an in-memory `ProjectionStore` inside this story to unblock testing
*"would itself be freezing a fixture shape for the port"* — the exact thing this
project's *Out of scope* assigns to HS-P0010 (`_decomposition.md:843-857`). If the
dependency has not landed when this story is picked up, **halt loudly**; do not
substitute.

**11. The edge flavour is a standing constraint, and this is the first generic code in
`happenstance` that binds a port.** Bind `EventStore`, never `SendEventStore` — it is
the weaker requirement and accepts both flavours (RS-20-2,
`standards/rust/20-two-flavour-ports.md:86`); import one flavour name per module and
reach the other by full path (RS-20-3, `:153`); never `#[async_trait]`, whose injected
`+ Send` makes the `wasm32` target impossible (`CLAUDE.md`, binding constraint 1); and
never make `read` an `async fn` (binding constraint 3). Where a bound must be stated
at a signature rather than left to inference, `spawns_from_generic` is the worked
pattern with per-bound reasoning attached (`crates/happenstance-core/src/memory.rs:643-680`).
There is no `dyn Projection` and there is none wanted — `apply` is generic over the
batch and `run_projection` over the codec, mirroring RS-20-5's *"there is no
`dyn EventStore`"* (`standards/rust/20-two-flavour-ports.md:276`).

**12. The persona-journey slice.** This is the application author's first hour,
journey *"Choose a contract before a database"*, activity **A5 — read the events back
into a read model**, which closes **beat 4** (`_storymap.md`, *Backbone*). What this
story makes true for that person: having modelled a boundary and run a command, they
can write a read model over their **own decoded events** — not over `Bytes`, not over
a second filter vocabulary they have to learn, and not with a checkpoint they have to
remember to advance — and the one thing they must opt into is named
`unstable-projection`, so the instability is something they typed rather than
something they discovered. **Polling renders nothing**: no spinner, no percentage, no
line that rewrites in place; its cost is a number in `experiments/`, not a performance
(`_design.md`, *States*, *Loading*; AC-U16).

## Integration contract

- **Archetype**: `capability` — a user-observable slice through every layer: an
  application author implements one trait over their own domain enum and calls one
  function, and a read model plus its checkpoint move forward together.
- **Slice / milestone**: **M5 `projection-runner`**. Slice-mates:
  **`projection-clause-verdicts`** (HS-S0027) and **`polling-cost-measurement`**
  (HS-S0028), which are independent of each other and both depend on this story. The
  three are implemented in one context and mounted as one surface: the runner is the
  *instrument* both siblings measure, so a verdict written before the runner compiles
  is a verdict about an intention.
- **Mount point**: **`crates/happenstance/src/lib.rs`** — the crate root, *"the only
  render path the library has"* (`_decomposition.md`, *Composition roots*, #1). Every
  item lands `pub use`d at the root beside the surviving `pub use
  happenstance_core::*;` (`:75`, AC-A01), and the module doc's fifth vocabulary bullet
  (`:49-51`) becomes an intra-doc link to the real item **in place**, in region 4 of
  `crate-root-rustdoc` (`_design.md`, *Composition*). Co-equal feature mount:
  **`crates/happenstance/Cargo.toml` `[features]`**, which gains the
  `unstable-projection` row and its forwarding. A runner that compiles but is not
  re-exported at the root, or a feature that exists in `#[cfg(...)]` but not in the
  manifest, is unmounted and this story is not done.
- **Wires into**:
  - `crates/happenstance-core/src/projection.rs` — `ProjectionStore::checkpoint`
    (`:110`), `begin` (`:117`), `commit` (`:126`), `rollback` (`:138`), the
    `Batch<'a>` GAT (`:92-99`) and `ProjectionId` (`:42-71`). The transactional
    invariant at `:13-30` is what the runner exists to honour.
  - `crates/happenstance-core/src/store.rs` — `EventStore::read` (`:119`) and its
    laziness contract (`:103-118`); `ReadOptions::from` is **inclusive**.
  - `crates/happenstance-core/src/event.rs` — `SequencePosition::next` (`:272`) and
    `EventType::from_static` (`:108`, already `const`).
  - `crates/happenstance-core/src/query.rs` — `Query` / `QueryItem`, the single
    nomination vocabulary. The derivation this story performs is the same one
    `Boundary::query` performs for a decision model; it must reuse it, not restate it.
  - M2's `happenstance::DomainEvent` (`EVENT_TYPES`, `decode`) and M3's
    `happenstance::Codec` / `CodecError` — the decode half of the runner is entirely
    theirs, and this story adds no second decode path.
  - `crates/happenstance-testkit/` — **read-only here.** No conformance rule is added;
    see below.
  - HS-P0010's `MemoryProjectionStore` behind the `memory` feature — the fixture the
    transactional test runs against.
- **Public items** (from `_design.md` `## Items`): `happenstance::Projection`,
  `happenstance::run_projection`, and — named in `## Signatures` and
  `## Visibility and stability` though not carrying their own `## Items` rows —
  `happenstance::Progressed` and `happenstance::ProjectionError`. It also owns the
  `unstable-projection` element of the feature row
  `happenstance [features] json / cbor / postcard / unstable-projection` (the other
  three are `codec-and-feature-forwarding`'s), and it changes
  `happenstance (crate module doc)` in regions 4 and 5.
- **Conformance rule(s)**: **none added, and that is deliberate.**
  `happenstance_testkit::event_store_conformance!` observes *store* behaviour, and
  this story adds an application-facing layer **above** the port that no adapter can
  see — an adapter that behaves correctly cannot fail a rule about it, and a rule no
  adapter can fail is decorative (`CLAUDE.md`, *The rule that matters*). The rules that
  *will* observe this behaviour are the **projection** suite's, and they belong to
  HS-P0010: `skip_and_record_is_atomic` (PS-27, `spec/SPECIFICATION.md:5411`) and
  `panicking_apply_rolls_back` (PS-30, `:5462`) are named in the specification against
  a suite this story does not own and must not pre-empt. This story is observed
  instead by integration tests in `crates/happenstance/tests/` against
  `MemoryProjectionStore`, plus a wrong implementation of its own (a runner that
  commits the checkpoint without the batch) written into those tests so the atomicity
  assertion can fail.
- **Clause(s)**: **discharges none by edit — this story writes no line of
  `spec/SPECIFICATION.md`.** It *supplies the evidence* for three clauses whose
  verdicts are HS-S0027's: **PS-33** (`:5529`, `DEFERRED`, ADR-0007's falsifier —
  evidence: whether anything but this runner calls a core checkpoint pump, and whether
  such a pump exists at all), **PS-27** (`:5411`, `PROVISIONAL` — evidence: whether
  the runner's failure path writes a skip record into the same batch that advances the
  checkpoint) and **PS-30** (`:5462`, `PROVISIONAL` — evidence: whether a fan-out
  runner is built at all, which decision 6 above answers *no* and gives the reason).
  It **honours** `ES-32` (`:4021`) — no tail or subscription seam at 0.1, so the runner
  polls, which is what makes `polling-cost-measurement` a story. `PS-18` is not
  evaluable in this tree and this story does not make it so: its subject, a `reset`
  method, does not exist in `crates/happenstance-core/src/projection.rs`
  (`_decomposition.md:660-673`).
- **Advances DoD scenario**: initiative **DoD 7** (*the projection suite
  discriminates*) — this is the first application-facing consumer whose existence lets
  a *"writes a checkpoint without its read model"* implementation be named and
  rejected, and this story ships that wrong implementation in its own tests even
  though the suite rule is HS-P0010's. Contributes to **DoD 13** (`cargo xtask ci`
  green on the assembled whole) and, through the surface it renders, to **DoD 10**
  (*the published crate looks finished* — the rendered documentation build green under
  all features, which is the build in which this off-by-default surface first appears
  at all).

## PR boundary

```
crates/happenstance/src/**
crates/happenstance/tests/**
crates/happenstance/Cargo.toml
Cargo.toml
Cargo.lock
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/projection-trait-and-runner/**
```

**In this PR.** The `Projection` trait; `run_projection`; `Progressed` and
`ProjectionError` with their variants and `#[non_exhaustive]` attributes; the query
derivation from `Self::Event::EVENT_TYPES` plus `scope()`; the chunk loop with its
`begin` / apply / `commit` / `rollback` discipline; the resume-past-checkpoint
arithmetic; the `unstable-projection` feature in `crates/happenstance/Cargo.toml` and
its forwarding decision; the root `pub use`s and the fifth vocabulary bullet becoming
an intra-doc link; the `# Features` table gaining its `unstable-projection` row; the
`[package.metadata.docs.rs]` block and `#![cfg_attr(docsrs, feature(doc_cfg))]` if M3
has not already added them; integration tests against `MemoryProjectionStore`
including the checkpoint-without-rows wrong implementation; and this story's own
backlog folder, which carries the ledger.

The implementer **may** also touch the composition-root and wiring files named in the
Integration contract to mount this slice — that is the mount, not scope drift, and
`crates/happenstance/src/lib.rs`, `crates/happenstance/Cargo.toml` and the workspace
`Cargo.toml` are inside the fence above for exactly that reason.

**Explicitly not in this PR.**

- **`spec/SPECIFICATION.md`, in any form.** The PS-33 / PS-27 / PS-30 / PS-18 verdicts
  and the three-part edit that carries them — generated §7.1/§7.2, the hand-written
  §1.3 census, and IDs retained on exit — are `projection-clause-verdicts`
  (HS-S0027). This story produces evidence and writes it into its own implementation
  report; it does not change a marker, and it does not run `cargo xtask spec-trace` as
  its own bar.
- **`experiments/`.** The N views × N reads polling cost is
  `polling-cost-measurement` (HS-S0028). Do not benchmark here, and do not put a
  number in a doc comment that no harness produced.
- **A fan-out runner over N projections.** PS-30's subject. Not built (Context pack,
  decision 6), and *not built* is the finding — not a gap to fill quietly.
- **Any edit under `crates/happenstance-core/src/**`.** Admissible only as the
  recorded outcome of AC-012's route — a defect written down with its clause ID and
  routed to a decision record — never as a convenience discovered mid-implementation
  (AC-A02). **This includes adding a checkpoint pump to make ADR-0007's text match the
  tree.** If the runner wants something the port does not offer, that is a defect-log
  entry for `defect-log-and-macros-verdict` (HS-S0032), not a patch.
- **`MemoryProjectionStore`, or any other in-memory `ProjectionStore`.** HS-P0010's
  AC-012. A throwaway written here freezes a fixture shape this project does not own.
- **Any conformance rule in `crates/happenstance-testkit/src/suite.rs`**, and any
  projection-suite rule at all. HS-P0010's.
- **`xtask/src/main.rs` and `xtask/src/proof.rs`.** The fifth `wasm32` step compiling
  `happenstance` is `edge-flavour-and-wasm-claim`'s (HS-S0031) to *register*; this
  story only has to leave it able to pass, which for an off-by-default feature means
  checking the combination locally rather than assuming it.
- **`CHANGELOG.md` and `crates/happenstance/README.md`.** `publish-0-2-0-alpha-1`'s
  (HS-S0033). This story makes `CHANGELOG.md:19-22`'s existing claim *true*; it does
  not edit the file to say so.

**Merge DoD.** `cargo xtask ci --fast` green (`.redkiln/config.yaml:55`), plus — because
`--fast` drops both feature powersets (`xtask/src/main.rs:830-836`) — `cargo hack check
--feature-powerset -p happenstance` and an explicit `cargo test -p happenstance
--features unstable-projection,memory` run locally, the crate-root page rendering the
runner as a real link with an `unstable-projection` badge under `--cfg docsrs`, and the
page still reading completely with the feature **off**.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| `Projection` is a public trait with `type Event: DomainEvent`, `type Store: ProjectionStore`, `fn id(&self) -> &ProjectionId`, `fn scope(&self) -> &Tags` and `fn apply(&mut self, event, batch) -> Result<(), StoreError>` | Signature fixed by the design, item for item. `apply` takes the **decoded** event and `&mut <Self::Store as ProjectionStore>::Batch<'_>`; its error is the *adapter's*, and the runner rolls the batch back on it. `scope` returns `&Tags` — the projection *holds* validated tags, for the same reason `DecisionModel::scope` does: a fallible construction inside an infallible signature forces an `unwrap` | `_design.md` `## Signatures` (the projection-runner block), `## Shape decision` (`DecisionModel::scope` row), `## Visibility and stability` |
| Whether `apply` is synchronous or asynchronous is the implementer's call, and it has a stated cost | Deliberately not prescribed (`_decomposition.md:712-723`): the callback-driven runner ADR-0007's Context correction already compiled is synchronous; if async, it is `trait_variant`, **never** `#[async_trait]`, and the cost is a doubled surface an application must implement — a DT-2 cost as much as a technical one. `_design.md` writes `fn apply`, so synchronous is the default and a departure needs its reason in the doc comment | `_decomposition.md:712-723`; `.kb/decisions/0008-one-derivation-for-both-ports.md`; `CLAUDE.md` binding constraint 1 |
| `run_projection(events, models, projection, codec, chunk)` returns `Result<Progressed, ProjectionError<S::Error, StoreError>>` | Five arguments, two type parameters on the error, `chunk: core::num::NonZeroUsize` — a zero-sized chunk has no honest meaning, which is the same reasoning `Retry`'s `NonZeroU32` rests on. Generic bounds are `S: EventStore, P: Projection, C: Codec` — the **bare** flavour, which accepts both | `_design.md` `## Signatures`; `standards/rust/20-two-flavour-ports.md:86` (RS-20-2) |
| The query is **derived**, from `P::Event::EVENT_TYPES` plus `scope()`'s tags, and there is no second filter vocabulary | The same derivation a decision model gets from `Boundary::query`. A projection cannot hand-maintain a subscription any more than a decision model can hand-maintain a query — that is the hazard ADR-0020 closes, applied one layer over. The runner never exposes a filter type of its own | `.kb/decisions/0007-projection-runner-decodes.md:61-64`; `_decomposition.md` AC-U04; `_design.md` `## Shape decision` (`DecisionModel::query` row) |
| The runner **streams**: no `collect`, no `Vec` of the log, memory flat in the length of the replay | `EventStore::read` returns the stream at the top level for exactly this reason; buffering here defeats ADR-0001/ADR-0008 and makes E2E-25's chunked rebuild unwritable. Chunking is a bounded buffer of at most `chunk` events between commits, which is not the same thing and is what the port's `begin`/`commit` pair is for | `crates/happenstance-core/src/store.rs:103-123`; `_decomposition.md:391-398` (AC-A05); `spec/E2E-CASES.md:654-676` (E2E-25) |
| One chunk is one transaction: `begin` → apply each decoded event → `commit(batch, id, last_applied_position)` | Never an `apply()`/`set_checkpoint()` pair, because the port does not offer one and the reason is written into its module doc. The position committed is the position of the **last event actually applied** in that chunk, never a computed or anticipated one | `crates/happenstance-core/src/projection.rs:13-30`, `:20-26`, `:117`, `:126` |
| Resume advances **past** the checkpoint | `checkpoint()` returns `Option<SequencePosition>`; `None` means never run and the replay starts from the beginning. Otherwise `from` is set to `position.next()`, and `next()`'s `None` arm — key-space exhaustion — is handled, not `unwrap`ped. `ReadOptions::from` is inclusive, so feeding the checkpoint in directly re-applies the last event on every run | `crates/happenstance-core/src/projection.rs:101-106`; `crates/happenstance-core/src/event.rs:265-280` |
| Positions may gap, and nothing in the runner does arithmetic on them beyond `next()` | No `position + 1`, no `head − checkpoint` lag, no test asserting literal positions. The specification permits gaps everywhere and `GappyMemoryStore` (M4) is the instrument that proves a consumer respects it | `CLAUDE.md`, *The rule that matters*; `crates/happenstance-core/src/event.rs:270-271`; `_design.md` `## The states the API must express` (*Gapped*) |
| `Progressed { through, applied }`, `#[non_exhaustive]` | `through` is how far the checkpoint moved; `applied` is how many events were applied. A named struct, not a tuple: the arity is exactly what a later observability pass wants to grow. A caller can distinguish "caught up" from "stopped early" without inspecting the store | `_design.md` `## The states the API must express` (*Partially applied*); `standards/rust/13-sealing-and-exhaustiveness.md:144` (RS-13-4) |
| `ProjectionError<S::Error, StoreError>`, `#[non_exhaustive]`, carries the **position** it stopped at | Its arms cover: the event store's read failure, the projection store's failure (from `begin`, `commit`, `rollback` or `apply`), and a **decode** failure carrying `CodecError` and the position. The decode arm is the one E2E-26 says ADR-0007's pump signature cannot represent; it is representable here only because `CodecError` is concrete rather than an associated type. Every foreign error is a typed `#[source]`, never a `String` | `spec/E2E-CASES.md:677-696` (E2E-26); `_design.md` `## Shape decision` (`Codec::Error` row); `standards/rust/30-error-taxonomy.md:71` (RS-30-2) |
| A failure inside a chunk rolls the batch back and leaves the checkpoint at the last **good** position | The half-applied chunk is discarded through `rollback` (`:138`), so a restart re-reads from the last committed checkpoint and no event is applied twice and none skipped. The runner reports *"stopped at P because of the event at P"*, which is E2E-26's THEN | `crates/happenstance-core/src/projection.rs:138`; `spec/E2E-CASES.md:677-696` |
| The runner offers **no** per-runner failure policy, and offering one would be wrong | PS-27's *Rejects* names it in terms: a runner-level `on_error: SkipPolicy` configuration *"is the obvious design, it is what a builder API invites, and it forces one wrong answer onto one of Wattline's two projections."* Failure policy is per projection. This story therefore ships halt-on-error and states the exclusion; skip-and-record is PS-27's subject and HS-P0010's suite | `spec/SPECIFICATION.md:5405-5425` (PS-27 and its *Rejects*) |
| No fan-out runner over N projections | `Batch<'a>` borrows from its store, so it is not `'static` and cannot move into a `tokio::spawn`ed task. One `run_projection` call drives one projection; N views cost N independent reads, which is the number `polling-cost-measurement` records | `crates/happenstance-core/src/projection.rs:92-99`; `_decomposition.md:697-700`; `spec/SPECIFICATION.md:4021` (ES-32) |
| No core checkpoint pump is written, and the absence is recorded rather than repaired | ADR-0007 allocates a pump to `happenstance-core`; at HEAD none exists, and `crates/happenstance-core/src/**` is outside this PR. The runner therefore drives the port directly, and that fact — with the caller count it implies — is written into this story's implementation report as the input HS-S0027's PS-33 verdict consumes | `.kb/decisions/0007-projection-runner-decodes.md:23-25`, `:53-59`; `spec/SPECIFICATION.md:5529`; `RUNBOOK.md:408-413`; `references/adr/0007-projection-runner-decodes.md` |
| The `unstable-projection` feature exists in the manifest, is **off by default**, and only adds | Forwarded to `happenstance-core/unstable-projection` if HS-P0010 landed that feature; gating only this crate's items if it did not — and the manifest says which, in a comment, beside the AC-A04 comment already there. Every other feature keeps meaning exactly what it means today, and `--no-default-features` is unchanged by this story | `_design.md` `## Shape decision` (projection runner gating row); `crates/happenstance/Cargo.toml`; `standards/rust/51-features-and-no-std.md:12` (RS-51-1); `_decomposition.md:381-390` (AC-A04) |
| The crate root renders the surface, and renders it correctly **with the feature off** | The fifth vocabulary bullet (`lib.rs:49-51`) becomes an intra-doc link in region 4; region 5's `# Features` table gains an `unstable-projection` row with one clause. Because the item is gated and the link is not conditional-safe otherwise, the linking form itself must be gated — no intra-doc link may resolve in only some feature configurations, and rustdoc treats a broken one as a **hard error** | `_design.md` `## Composition` (regions 4-5), `## States` (*Feature off*), `## Anti-patterns` 15; `standards/rust/70-rustdoc-obligations.md:93` (RS-70-2), `:195` (RS-70-4); `standards/rust/51-features-and-no-std.md:188` (RS-51-5) |
| Nothing shadows a contract name, and `pub use happenstance_core::*;` survives | `ProjectionId`, `ProjectionStore`, `Query`, `ReadOptions` and `SequencePosition` all arrive through the glob and are used, not redefined. A `happenstance::ProjectionStore` of our own is a silent breaking change to a published facade and an ambiguity in every existing doctest | `crates/happenstance/src/lib.rs:75`; `_design.md` `## Placement and re-export`, `## Anti-patterns` 14 (AC-A01) |
| The edge flavour survives: bare-flavour bounds, one flavour name per module, no `#[async_trait]`, `read` untouched | This is the first generic code in `happenstance` to bind a port, so the discipline stops being theoretical here. Where a bound is genuinely required it is written at the signature with its reason, the way `spawns_from_generic` does. The two `memory.rs` tests that pin `read`'s shape are not touched | `CLAUDE.md` binding constraints 1, 3, 4; `standards/rust/20-two-flavour-ports.md:86`, `:153`, `:276`; `crates/happenstance-core/src/memory.rs:643-680` |
| Every fallible public item documents `# Errors` by **condition**, and every unusual construct names the alternative that lost, once | Including: why the query is derived rather than supplied, why there is no per-runner failure policy, why the batch cannot cross a spawn, and why the feature is off. The audience is fluent in event sourcing and new to idiomatic Rust; the Rust-specific reasoning is the part that must be written down | `standards/rust/70-rustdoc-obligations.md:243` (RS-70-5); `standards/rust/00-prime-directives.md`; `CLAUDE.md`, *Who you are working with*; `_decomposition.md` AC-U15 |
| No `unwrap` / `expect` / `panic!` on the runner path | `-D warnings` clippy under the workspace lints. Three specific temptations: `next().unwrap()`, `NonZeroUsize::new(n).unwrap()` on a caller-supplied value, and unwrapping the derived `Query`'s `Result`. Each has a designed answer — handle the `None`, take the `NonZero` as an argument, absorb the `Result` into the error type | `crates/happenstance-core/src/event.rs:265-280`; `_design.md` *the residual* (how `Boundary::query`'s `Result` is absorbed rather than unwrapped) |
| The transactional claim is tested against a **real** `ProjectionStore`, with a named wrong implementation beside it | The test opens a batch, applies decoded events, and asserts the read-model rows and the advanced checkpoint are both present or both absent. Its discriminator is a deliberately wrong runner that commits the checkpoint without the batch — the same shape as HS-P0010's `CheckpointOnlyStore` — and the test must **fail** against it. Fixture is HS-P0010's `MemoryProjectionStore`; if it has not landed, halt | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` (AC-002, AC-012); `_decomposition.md:843-857`; `CLAUDE.md`, *The rule that matters* (a rule no adapter can fail is decorative) |

## Data and migrations

**No schema, no migration, and no deployed data — but this story introduces the first
piece of *durable state the library itself owns*, and that is worth stating precisely.**

There is no database in this project: `MemoryEventStore` is all it needs
(`project.md`, *Out of scope*), nothing is published to the registry until
`publish-0-2-0-alpha-1` two milestones later, and the only `ProjectionStore` in reach
is HS-P0010's in-memory fixture. So there is nothing to migrate *from* and no
compatibility window to keep open.

What this story does establish is the **meaning of a checkpoint row**, and every
property of it is decided elsewhere and merely honoured here:

| Datum | Where it lives | Who decided | Consequence if this story gets it wrong |
| --- | --- | --- | --- |
| The checkpoint position for `(store, ProjectionId)` | the adapter's own checkpoint storage, reached only through `checkpoint` / `commit` | `ProjectionStore`'s transactional invariant (`crates/happenstance-core/src/projection.rs:13-30`) and ADR-0007's *per `(store, ProjectionId)`* rule (`.kb/decisions/0007-projection-runner-decodes.md:66-69`) | A checkpoint advanced outside the batch means a crash silently skips events — the exact failure the port's shape exists to make unwritable |
| `None` as "never run" | `checkpoint()`'s return type (`:110`) | the port | Treating `None` as position zero, or as "caught up", turns a first run into either a panic or a silent no-op |
| The resume anchor | `ReadOptions::from`, **inclusive** (`:104-106`) | the port; `SequencePosition::next` is the only sanctioned way to advance past a position without doing arithmetic on an opaque key (`crates/happenstance-core/src/event.rs:265-280`) | Feeding the checkpoint straight in re-applies the last event on every run. Adding one by hand reintroduces the gap assumption `GappyMemoryStore` exists to catch |
| The read-model rows themselves | inside the adapter's `Batch`, opaque to this crate | the application's `apply`; the runner never inspects them | A runner that reads or interprets read-model rows has taken on domain knowledge the whole crate split exists to keep out |
| `ProjectionId`'s value | `ProjectionId::new`, **infallible today, and that is an open question rather than a decision** (`crates/happenstance-core/src/projection.rs:44-64`) | HS-P0010, at the port's freeze | This story must not add validation beside it. *"Two constructors enforcing different rules is the defect that makes an invalid value reachable through the weaker one"* — and the empty-string identifier that reaches a checkpoint primary key is a **defect-log candidate** for HS-S0032, not a patch |

**One rebuild question is raised and deliberately left unanswered here.** E2E-25 asks
that a projection rebuilding across many invocations be able to distinguish *"caught up
to N"* from *"rebuilding, currently at N, do not treat these rows as authoritative"*,
and falsifies the claim that one `Option<SequencePosition>` is sufficient projection
state (`spec/E2E-CASES.md:654-676`). The port has no second field and no status, and
the honest cheaper alternative — rebuild into a second `ProjectionId` and swap — *"is
already permitted by the port and documented nowhere."* Documenting it is within this
story's reach and costs one paragraph of rustdoc; **changing the port to carry rebuild
state is not**, and if the runner's implementation shows that a documented swap
protocol is insufficient, that is a defect-log entry naming E2E-25 and routed to
HS-S0032 (AC-012, AC-A02) — never an edit under `crates/happenstance-core/src/**`.

## Acceptance criteria

Each criterion is a persona goal crossing the full stack — the application author's
first hour (P1, journey *"Choose a contract before a database"*, activity **A5**, which
closes **beat 4**), the edge developer whose target forbids `Send` (P3), and the
evaluator with one bounded sitting (P4), as
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`
writes them. Verification names a real path. `crates/happenstance/tests/` is created by
M3 (`codec-and-feature-forwarding`); this story adds **one new file** to it,
`projection_runner.rs`, and **extends** two that M3 created, `manifest_contract.rs` and
`doc_surface.rs`. Every test in the new file runs under `--features
unstable-projection,memory`, because with the feature off there is nothing to test —
which is itself AC-008's point.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an application author who has modelled a boundary and run a command (beats 1-3) and now wants a read model, **WHEN** they implement `happenstance::Projection` over **their own domain enum** — `type Event`, `type Store`, a `ProjectionId`, a `scope()` returning validated `Tags`, and one `apply` — **THEN** the events their projection is handed are nominated by a `Query` **derived** from `Self::Event::EVENT_TYPES` plus `scope()`, so there is no second filtering vocabulary to learn and no subscription to hand-maintain; and a projection spanning two stores is **unrepresentable** rather than merely undocumented, because `Store` is an associated type and no cross-store transaction exists. | `crates/happenstance/tests/projection_runner.rs::derived_query_matches_event_types_and_scope` (new): assert the derived `Query` equals the union of the model's own `EVENT_TYPES` and tags, and that an event outside the scope is never handed to `apply`. The cross-store claim is a **compile** claim, not a runtime one — the associated type is what makes it hold. |
| AC-002 | **GIVEN** that same author, whose events are a Rust enum and not `Bytes`, **WHEN** the runner reads an event out of the store, **THEN** `apply` receives a **decoded** `Self::Event` produced by M3's `Codec` — the decode half ADR-0007 assigned to the typed layer — and the author writes no `serde_json::from_slice`, no manual match on `EventType`, and no second decode path anywhere in their projection. | `crates/happenstance/tests/projection_runner.rs::apply_receives_decoded_domain_events` (new), whose `apply` matches on its own enum variants and asserts variant equality — a signature taking `Bytes` would not compile. Paired with `no_second_decode_path` in `doc_surface.rs`, asserting the runner path names `Codec` and nothing else. |
| AC-003 | **GIVEN** an operator restarting a service after a crash, **WHEN** they compare the read model against the checkpoint, **THEN** they never find one ahead of the other: each chunk's read-model rows and its checkpoint move in the **single** `commit` (`crates/happenstance-core/src/projection.rs:126`), the runner never calls an `apply()`/`set_checkpoint()` pair because the port deliberately offers none (`:20-26`), and the position committed is the position of the **last event actually applied**, never a computed or anticipated one. | `crates/happenstance/tests/projection_runner.rs::read_model_and_checkpoint_commit_together` (new) against `MemoryProjectionStore`, asserting rows and checkpoint are both present or both absent; discriminated by `checkpoint_without_rows_is_rejected` (new), a deliberately wrong runner in the test file that commits the checkpoint without the batch and **must fail** the assertion. |
| AC-004 | **GIVEN** an author whose projection has already run once, **WHEN** they run it again, **THEN** it resumes **past** the checkpoint and not at it — no event applied twice and none skipped, because `ReadOptions::from` is inclusive (`projection.rs:104-106`) — `None` from `checkpoint()` means *never run* and replays from the beginning rather than from position zero, `SequencePosition::next()`'s `None` arm (key-space exhaustion) is a typed refusal rather than an `unwrap` (`event.rs:265-280`), and positions may **gap**, so nothing computes `head − checkpoint` as a lag and no assertion names a literal position. | `crates/happenstance/tests/projection_runner.rs::resume_advances_past_the_checkpoint`, `::first_run_starts_from_the_beginning`, `::tolerates_gapped_positions` (all new). The gap test runs against `happenstance_testkit::GappyMemoryStore` (M4's `misbehaving-testkit-stores`, merged before M5 by milestone order) and compares against positions the store actually assigned (`CLAUDE.md`, *The rule that matters*). |
| AC-005 | **GIVEN** an author rebuilding a read model over a log far larger than memory, **WHEN** they call `run_projection` with a `chunk: NonZeroUsize`, **THEN** memory stays flat in the length of the replay: the runner never `collect`s the stream, commits happen **while the stream is still being pulled**, and the only buffer is the at-most-`chunk` events between one `begin` and its `commit` — which is the whole reason `EventStore::read` returns the stream at the top level and is not `async` (ADR-0001/ADR-0008), and the precondition for E2E-25's chunked rebuild. | `crates/happenstance/tests/projection_runner.rs::commits_before_the_stream_ends` (new): a store whose stream records its pull order, asserting the first `commit` is observed **before** the final event is pulled. A runner that `collect`s the stream passes every other row in this table and fails this one. |
| AC-006 | **GIVEN** an author whose log contains one event their codec cannot decode, **WHEN** the runner reaches it, **THEN** they receive a typed `ProjectionError` naming the **position** it stopped at and carrying the concrete `CodecError` as a `#[source]` — the representable home E2E-26 says ADR-0007's pump signature cannot provide — the half-applied chunk is discarded through `rollback` so the checkpoint still sits at the last **good** position, the partial progress comes back as a returned `Progressed { through, applied }` value, and the runner **renders nothing while it works**: no spinner, no percentage, no line that rewrites in place (AC-U16). | `crates/happenstance/tests/projection_runner.rs::decode_failure_names_its_position_and_rolls_back` (new), asserting the variant, the carried position, the walkable `#[source]` chain, and that the checkpoint did not move; plus `doc_surface.rs::runner_prints_nothing`, a source assertion that no `println!`/`eprintln!`/`print!` appears on the runner path. |
| AC-007 | **GIVEN** the author of `projection-clause-verdicts` (HS-S0027), who must write the PS-33 / PS-27 / PS-30 verdicts from **evidence** rather than intention, **WHEN** they read this story's implementation report, **THEN** they find each count stated as a fact about the tree — whether a checkpoint pump exists in `happenstance-core` at all and who calls it (PS-33), that no per-runner `on_error: SkipPolicy` was offered because failure policy is per projection (PS-27's *Rejects*), and that no fan-out runner was built because `Batch<'a>` borrows its store and cannot cross a `tokio::spawn` (PS-30) — with **no line of `spec/SPECIFICATION.md` edited, no maturity marker moved, and no pump written into the contract crate to make ADR-0007's text match the tree**. | The implementation-report body, plus an empty `git diff --stat main -- spec/ crates/happenstance-core/src` asserted at review. `cargo xtask spec-trace` is deliberately **not** this story's bar — it is HS-S0027's, and a green run here would prove only that nothing changed. |
| AC-008 | **GIVEN** an evaluator with one bounded sitting reading the manifest to decide whether the projection surface is something they can depend on, **WHEN** they look for it, **THEN** they find a real `unstable-projection` feature that is **off by default** — making `CHANGELOG.md:19-22`'s existing claim true rather than aspirational — whose forwarding to `happenstance-core/unstable-projection` (or its deliberate absence, if HS-P0010 has not landed that feature) is stated in a manifest comment beside the AC-A04 comment already there; and turning it on **only adds**: every item that compiled before still compiles, and `default-features = false` means exactly what it meant before this PR (RS-51-1). | `crates/happenstance/tests/manifest_contract.rs::unstable_projection_is_declared_off_by_default` (extends M3's file) reading `include_str!("../Cargo.toml")`; plus `cargo hack check --feature-powerset -p happenstance`, which `cargo xtask ci --fast` does **not** run (`xtask/src/main.rs:830-836`) and which is therefore run by hand. |
| AC-009 | **GIVEN** an evaluator landing on the crate-root page after the alpha, **WHEN** they scan it for how this crate reads events back into a read model, **THEN** they meet the real surface and not a roadmap: the **fifth** vocabulary bullet at `crates/happenstance/src/lib.rs:49-51` has become an intra-doc link to the real item **in place** — same position, same order, same discriminator prose — the `# Features` region gains an `unstable-projection` row of one clause, below the vocabulary and recessive, region 7 still last; the *"Planned, and specified in `spec/SPECIFICATION.md`"* heading is **gone**, because this is the last of the five bullets; and the density budget holds: first doc sentence ≤ 80 characters, identifier ≤ 24 characters, code inside a doc fence ≤ 72 columns, doc prose ≤ 80 columns, module doc ≤ 130 lines. | `crates/happenstance/tests/doc_surface.rs::crate_root_renders_the_projection_surface` (extends M3's file) over `include_str!("../src/lib.rs")`, measuring each budget and asserting no `Planned` heading and no `Planned` bullet survives; plus `cargo doc -p happenstance --no-deps`. |
| AC-010 | **GIVEN** the same evaluator reading the published docs.rs page rather than the source, **WHEN** they open `Projection` or `run_projection`, **THEN** each carries a `doc_cfg` badge naming `unstable-projection` (the design's anti-pattern 15 is the failure this forbids) — `[package.metadata.docs.rs]` and `#![cfg_attr(docsrs, feature(doc_cfg))]` present, added by this story if M3 did not add them — and the crate-root page renders **complete and warning-free with the feature off**, which is its default state: no intra-doc link resolves in only some configurations, and rustdoc treats a broken one as a hard error rather than a warning (RS-70-2/RS-70-4/RS-51-5). | `cargo doc -p happenstance --no-deps`; the same `--no-default-features`; `cargo +nightly doc -p happenstance --no-deps --all-features` under `RUSTDOCFLAGS="--cfg docsrs -D warnings"`; plus `crates/happenstance/tests/manifest_contract.rs::docs_rs_metadata_is_declared` (M3's, asserted again here because this story is the first to gate an item in this crate). |
| AC-011 | **GIVEN** the edge developer (P3) whose target is `wasm32-unknown-unknown` and whose futures are not `Send`, **WHEN** this crate's **first** port-binding generic code lands, **THEN** it binds `EventStore` and never `SendEventStore` — the weaker requirement, which accepts both flavours — imports one flavour name per module and reaches the other by full path, introduces no `#[async_trait]`, leaves `EventStore::read` non-`async` with both `memory.rs` tests that pin its shape untouched, and leaves `pub use happenstance_core::*;` (`lib.rs:75`) surviving with **nothing shadowing a contract name**: `ProjectionId`, `ProjectionStore`, `Query`, `ReadOptions` and `SequencePosition` all arrive through the glob and are used, not redefined. | `cargo check -p happenstance --target wasm32-unknown-unknown --no-default-features --features std,json,unstable-projection` (the invocation `edge-flavour-and-wasm-claim` will later register, run here by hand); `crates/happenstance/tests/doc_surface.rs::no_contract_name_is_shadowed`; and an empty `git diff --stat main -- crates/happenstance-core/src`. |
| AC-012 | **GIVEN** a maintainer who must be able to *believe* the atomicity claim rather than take it on trust, **WHEN** the transactional test runs, **THEN** it runs against HS-P0010's **real** `MemoryProjectionStore` behind the `memory` feature — never a throwaway `ProjectionStore` written inside this crate, which would freeze a fixture shape this project explicitly does not own — and if that cross-project dependency has not landed when this story is picked up, the story **halts loudly**, naming the missing artefact, rather than substituting one. | `crates/happenstance/tests/projection_runner.rs` importing `MemoryProjectionStore` from the crate that owns it, plus an **absence** assertion in `doc_surface.rs::no_local_projection_store`: no `impl ProjectionStore for` appears anywhere under `crates/happenstance/`. The halt is a preflight check, recorded in the implementation report if it fires. |

**Coverage of the traced project AC.** Project **AC-006** (*an application writes a
projection against decoded events*, `_decomposition.md:421`, `:784`) is discharged by
AC-001 (the trait and its derived nomination), AC-002 (decoded, not `Bytes`), AC-003
(the atomic commit the brief names in terms), AC-004 (resume), AC-005 (streaming) and
AC-006 (the failure path), with AC-012 supplying the fixture discipline the brief flags
as a cross-project dependency. AC-007 through AC-011 are the surface, feature and
flavour obligations this project imposes on *every* story it ships
(`_decomposition.md` AC-A01, AC-A04, AC-A05, AC-U14, AC-U16).

## Interaction quality

Every invariant that applies is carried by an **AC row above**; this section says which
row carries which, and how it is observed. Nothing here is a free-floating bullet,
because a bullet in this section would get no ledger row, no gate and no test.

**STATE invariants.**

| Invariant | Carried by | How it is observed |
| --- | --- | --- |
| **In place, not a context jump** | AC-006, AC-009 | A decode failure is actionable *at the call site*: `ProjectionError` carries the position and the `CodecError`, so nothing sends the reader to a second document (AC-U10). And the fifth vocabulary bullet becomes a link **at its own line**, in the same order and with the same prose — the page is not restructured and no new page is introduced. |
| **Non-occlusion — a filter must not hide what it filtered** | AC-001, AC-006, AC-010 | The projection's `Query` is **derived and inspectable** as a value, not private machinery inside `read` (AC-U11); the runner offers **no** silent skip policy, so an event the projection cannot handle is a named error at a named position rather than a row that quietly never arrived; and a feature-gated item does not vanish from the rendered page — it renders with the badge that says which feature would produce it. |
| **Preserved state — the checkpoint *is* the preserved selection** | AC-003, AC-004, AC-008, AC-011 | Restarting resumes exactly where the last successful `commit` left off, applying nothing twice and skipping nothing; a rolled-back chunk leaves the previous state intact; turning the feature off leaves every unrelated item compiling; and the glob re-export survives so no caller's existing path breaks. |
| **Reversibility** | AC-003, AC-006 | Every act before the `commit` is reversible: `begin` opens a batch, `apply` writes into it, and a failure discards the whole chunk through `rollback`. The `commit` is the single irreversible act of a chunk, and it is the one that also advances the checkpoint — reversibility and atomicity are the same property here (AC-U12). |
| **Reachability without a search engine** (the library reading of keyboard reachability) | AC-009, AC-010 | Every item this story adds is reachable from the crate root's module doc by intra-doc link, in **every** feature configuration — including the default one, where the item is absent and the link must therefore be gated too. A reader can tell what `unstable-projection` turns on without opening `Cargo.toml` (AC-U14). |

**COMPOSITION invariants**, taken from the project's signed-off `_design.md` — this
story renders into `crate-root-rustdoc`, regions 4 and 5, and into two item pages.

| Invariant | Carried by | How it is observed |
| --- | --- | --- |
| **Presentation exists at all** | AC-009, AC-006 | The surface is *composed* rustdoc — a linked vocabulary bullet, a `# Features` row of one clause, and item docs that name the alternative that lost (RS-70-5) — not four bare `pub use`s that happen to be public. A crate whose only change is re-exports satisfies every compile assertion in this spec and fails this one. |
| **Composition / placement** | AC-009 | The regions in the order `_design.md`'s `## Composition` fixes: the projection bullet stays in **region 4** in its existing fifth position; the `# Features` table is **region 5**, below the vocabulary; **region 7** (the adapter-author pointer) stays last and is not displaced. |
| **Transience** | AC-009, AC-010 | `Projection` and `run_projection` are **opened on demand** — off by default, because *"a reader must perform an act that names its instability before the item is in their build"* (`_design.md`, *Transience policy*). They must **not** be promoted to persistent chrome: they do not enter the first program and they do not join the four names region 2 cannot be written without. `Progressed` and `ProjectionError` are **revealed** — reached from `run_projection`'s signature, not listed on the landing page. The feature row is a **revealed** table entry plus a `doc_cfg` badge. |
| **Density budget, with its numbers** | AC-009 | First doc sentence ≤ **80** characters (rustdoc truncates the item-table column near there — anti-pattern 5); public identifier ≤ **24** characters (`run_projection` is 14, `ProjectionError` 15, `Progressed` 10 — all comply); code inside a doc fence ≤ **72** columns (rustdoc `pre` blocks scroll rather than wrap — anti-pattern 3); doc prose ≤ **80** columns; crate-root module doc ≤ **130** lines total; code ≤ **100** columns. |
| **Hierarchy** | AC-009 | The link *is* the emphasis — nothing is bolded that is not also reachable — and the Features table stays **recessive**: below the vocabulary, in table form rather than prose, with no link pointing back up into the first program. The runner must not acquire a heading of its own on the landing page; it is a bullet that became a link. |
| **Named anti-patterns** | AC-009 (1, 3, 5, 14), AC-010 (6, **15**), AC-006 (9) | 1: no bulleted list under the word *"Planned"* survives — this story deletes the last one, and the heading with it. 3: no fence on the page needs horizontal scrolling at 1024px. 5: no item summary this story adds ends in an ellipsis. 6 and **15**: the runner never appears on the default docs.rs page without an `unstable-projection` badge, and the changelog never claims a feature the manifest does not have — **15 is this story's own anti-pattern**, named in `_design.md` for exactly this surface. 9: no spinner, no percentage, no line that rewrites in place — the runner polls and polling renders nothing. 14: no `happenstance::ProjectionStore`/`Query`/`ReadOptions` of our own. |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | An event nominated by the query cannot be decoded into `P::Event`. | `ProjectionError`'s decode arm, carrying the **position** and the concrete `CodecError` as a typed `#[source]` (RS-30-2). The open batch is discarded through `rollback` (`projection.rs:138`), the checkpoint stays at the last good position, and the caller can restart without applying anything twice. Carried by AC-006. |
| EC-002 | `Projection::apply` returns the adapter's own error. | Same discipline: `rollback`, then `ProjectionError`'s projection-store arm with the adapter's error as `#[source]`. The runner never converts an adapter error into a `String` and never re-words it. |
| EC-003 | `begin`, `commit` or `rollback` itself fails. | A distinct arm — a `commit` that failed must **never** be reported as applied, and `Progressed` must not be returned for a chunk whose `commit` did not succeed. A failed `rollback` after a failed `apply` is reported as the *original* cause with the rollback failure chained, not the other way round. |
| EC-004 | `EventStore::read`'s stream yields an error mid-replay. | `ProjectionError`'s read arm with the store's error as `#[source]`. Chunks already committed stay committed — that is the point of committing them — and the checkpoint names exactly how far it got. |
| EC-005 | `SequencePosition::next()` returns `None` — the key space is exhausted at the resume point. | A typed refusal, never `unwrap` and never a silent re-read from the same position. `next` is `checked_add` rather than `saturating_add` precisely so this is representable (`crates/happenstance-core/src/event.rs:265-280`); consuming it with `unwrap` reintroduces the bug that comment exists to describe. |
| EC-006 | `ProjectionStore::checkpoint` returns `None`. | **Not an error.** It means *never run*: the replay starts from the beginning of the query's selection, with no `from` anchor. Treating `None` as position zero, or as "caught up", turns a first run into a panic or a silent no-op. Carried by AC-004. |
| EC-007 | `Projection::apply` **panics**. | The runner does not `catch_unwind` and must not be structured so that an unwind can leave a batch committed: the batch's own `Drop` is what discards it, and that rule is HS-P0010's to *state* on the port (`commit-rollback-and-drop-rules`). This story's obligation is not to defeat it — no `mem::forget`, no batch escaping the scope of its chunk. PS-30 is the clause; the suite rule is not this story's. |
| EC-008 | The chunk boundary is reached with events still pending, and the caller wants to know. | Not an error: `run_projection` returns `Progressed { through, applied }` and the caller decides whether to call again. A runner that loops until the stream is exhausted *and* offers no way to observe partial progress makes the *Partially applied* state unrepresentable (`_design.md`, *The states the API must express*). |
| EC-009 | HS-P0010's `MemoryProjectionStore` has not landed when this story is picked up. | **Halt loudly**, naming the missing artefact. Do not write a throwaway in-memory `ProjectionStore` here — that freezes a fixture shape this project's *Out of scope* assigns to HS-P0010 (`_decomposition.md:843-857`). Carried by AC-012. |
| EC-010 | The runner wants something the frozen contract does not offer — a pump, a `set_checkpoint`, a fallible `ProjectionId::new`, a second field of projection state. | A **defect-log entry naming its clause ID**, routed to a decision record for AC-012 and consumed by `defect-log-and-macros-verdict` (HS-S0032) — never a patch under `crates/happenstance-core/src/**` (AC-A02). This explicitly includes *"write the pump ADR-0007 describes so the ADR looks right"*, which would fabricate the evidence PS-33 exists to gather. |

## Non-functional

| id | requirement | how it is held |
| --- | --- | --- |
| NF-001 | **Memory is flat in the length of the replay.** | The stream is never `collect`ed; the only buffer is the at-most-`chunk` events between `begin` and `commit`. Observed by AC-005's pull-order test rather than by a memory measurement, because a pull-order assertion is deterministic and a memory assertion is not. |
| NF-002 | **No `unwrap` / `expect` / `panic!` on the runner path in library code.** | `-D warnings` clippy under the workspace lints. Three named temptations, each with a designed answer: `next().unwrap()` → handle the `None` (EC-005); `NonZeroUsize::new(n).unwrap()` → take the `NonZero` as an argument; unwrapping the derived `Query`'s `Result` → absorb it into `ProjectionError`, the way `_design.md`'s residual absorbs `Boundary::query`'s. |
| NF-003 | **No `Send` bound is introduced and no `#[async_trait]` appears.** | Generic code binds `EventStore`, never `SendEventStore`; one flavour name per module. This is the first port-binding generic code in `happenstance`, so the discipline stops being theoretical here — `spawns_from_generic` (`crates/happenstance-core/src/memory.rs:643-680`) is the worked pattern for writing a bound at the signature with its reason attached. |
| NF-004 | **Three rustdoc configurations build warning-free**: default, `--no-default-features`, and `--all-features --cfg docsrs` on nightly. | A broken intra-doc link is a hard rustdoc error, and this story is the first in the crate to link *from* an ungated page *to* a gated item — the configuration in which that link must not resolve is the default one. Conditional doc lines are the shape the design mock found (`_design.md`, mock finding 2). |
| NF-005 | **No new third-party dependency, and no new dependency edge except the feature forwarding.** | The runner is built from `happenstance-core`'s port, M2's `DomainEvent` and M3's `Codec`. If `unstable-projection` forwards, it forwards to `happenstance-core/unstable-projection` and to nothing else (RS-50-1: a dependency is justified by counting nodes). |
| NF-006 | **The MSRV floor is checked by running the compiler at 1.97.1**, not by reading metadata. | ADR-0029's lesson is procedural. `cargo +1.97.1 check -p happenstance --all-features`. Let-chains are available (stabilised 1.88) and may be used; nothing else about the floor moves, and moving it would need its own ADR. |
| NF-007 | **`cargo package --list` still shows both licence files and a README for `happenstance`.** | `cargo xtask ci`'s packaging step already asserts it for all three publishable crates; adding a feature, a `tests/` file and a `[package.metadata.docs.rs]` block must not disturb the include set. |
| NF-008 | **No timing or throughput assertion appears in any test this story ships.** | The polling cost is a recorded number with its conditions in `experiments/`, owned by `polling-cost-measurement` (HS-S0028) and outside the gate by construction (CF-34). A timing assertion inside `cargo test` is a flaky gate wearing a performance claim. |

## Implementation notes (non-prescriptive)

*Shape decisions are settled above and in `_design.md`; these are the traps, not
instructions.*

- **Preflight before writing anything: does `MemoryProjectionStore` exist behind the
  `memory` feature?** If not, halt (EC-009). It is the first thing to check and the
  cheapest thing to get wrong, because the temptation to write "just a small one"
  arrives at hour three and is unrecoverable by hour six.
- **Do not write a checkpoint pump into `happenstance-core`.** ADR-0007's text
  describes one; the tree has none. The gap is the finding, and it is HS-S0027's to
  adjudicate. Writing one to make the ADR read correctly is the single most plausible
  wrong move in this story and it fails AC-007 and EC-010 together.
- **Chunking is a bounded buffer; `collect` is not chunking.** The distinction is easy
  to lose while writing the loop: pull at most `chunk` events, apply them into an open
  batch, commit, then pull again from the **same** stream. Re-`read`ing per chunk is
  also wrong — it re-derives the query, re-anchors `from`, and turns one replay into N
  and is the shape `polling-cost-measurement` is measuring the cost *of*, not a design
  this story should adopt.
- **The batch's lifetime is the chunk's lifetime, and the borrow checker will tell you
  so.** `type Batch<'a> where Self: 'a` means the batch cannot outlive its store and
  cannot move into a task (`projection.rs:92-99`, `_decomposition.md:697-700`). Scope
  it inside the loop iteration. The compiler error you get if you do not is the
  architecture brief's paragraph, restated by rustc.
- **Reuse the query derivation; do not write a second one.** `Boundary::query` already
  derives from `EVENT_TYPES` plus tags, and ADR-0008 is *one derivation for both ports*.
  Its `Result` is absorbed, not unwrapped.
- **`apply` is synchronous by default and a departure needs its reason in the doc
  comment.** `_design.md` writes `fn apply`. If it must be async it is `trait_variant`,
  never `#[async_trait]`, and the cost is a doubled surface every application must
  implement (`_decomposition.md:712-723`).
- **Gate the *link*, not just the item.** An intra-doc link on the ungated crate root
  pointing at a gated item is a hard rustdoc error with the feature off, which is the
  default configuration. `#![cfg_attr(...)]`-style conditional doc lines are the shape
  that made all three builds clean in the design mock; budget time for it rather than
  discovering it in the docs step.
- **State the forwarding decision in a comment, whichever way it goes.** The manifest
  already carries a five-line comment stating AC-A04's failure mode in its own words;
  extend it, do not rewrite it.
- **Write the wrong runner into the test file, not the library.** The
  checkpoint-without-rows implementation exists so AC-003's assertion can fail; a rule
  no implementation can fail is decorative (`CLAUDE.md`, *The rule that matters*).
- **Name the alternative that lost, once, where the reader is** (RS-70-5): why the
  query is derived rather than supplied, why there is no per-runner failure policy, why
  the batch cannot cross a spawn, why `Progressed` is a struct and not a tuple, and why
  the feature is off. One doc comment each, not repeated across the module. The reader
  is fluent in event sourcing and new to idiomatic Rust — the Rust-specific half is the
  half that must be written down.

## Tests and CI (merge gate)

Grounded in the project testing brief's **AC-006 row** (*"a runner test that opens a
batch, applies decoded events, and asserts the commit is atomic … this test's fixture
does not exist in the tree yet"*, `_decomposition.md:784`) and its *Intent*, which fixes
this project's own bar at `cargo xtask ci --fast` (`.redkiln/config.yaml:50-55`).

| tier | command / path | proves |
| --- | --- | --- |
| Integration | `cargo test -p happenstance --features unstable-projection,memory` → `crates/happenstance/tests/projection_runner.rs` (**new**) | AC-001 … AC-006, AC-012 — the derived query, decoded events reaching `apply`, the atomic commit and its wrong-implementation discriminator, resume past the checkpoint, gap tolerance, the streaming pull-order assertion, and the decode failure that names its position. |
| Integration (discriminator) | same file → `::checkpoint_without_rows_is_rejected` | AC-003 — the named wrong implementation, without which the atomicity assertion is decorative. It must **fail** the atomicity assertion and the test asserts that it does. |
| Integration (misbehaving store) | same file → `::tolerates_gapped_positions` against `happenstance_testkit::GappyMemoryStore` | AC-004 — that no arithmetic assumes contiguity. M4's instrument, consumed here rather than re-built. |
| Unit (manifest) | `cargo test -p happenstance` → `crates/happenstance/tests/manifest_contract.rs` (**extended**) | AC-008, AC-010 — the feature declared off by default with its forwarding comment, and the docs.rs metadata block. |
| Unit (rendered surface) | `cargo test -p happenstance` → `crates/happenstance/tests/doc_surface.rs` (**extended**) | AC-002, AC-006, AC-009, AC-011, AC-012 — the composition and density invariants an unstyled render satisfies silently, plus the four absence assertions (no second decode path, no printing on the runner path, no shadowed contract name, no local `ProjectionStore`). |
| Doctest | `cargo test -p happenstance --doc --features unstable-projection,memory` | `Projection`'s and `run_projection`'s own examples compile; a `?`-using example closes with `# Ok::<(), E>(())` and any `.await` example gets a hidden runtime (RS-62-2). |
| Feature powerset (**not** in `--fast`) | `cargo hack check --feature-powerset -p happenstance` | AC-008 — that `unstable-projection` composes with `json`/`postcard`/`memory`/`std` and subtracts nothing. `--fast` drops both powersets (`xtask/src/main.rs:830-836`), so this is run by hand and a green `--fast` is not evidence for it. |
| Docs, three configurations | `cargo doc -p happenstance --no-deps`; the same `--no-default-features`; `cargo +nightly doc -p happenstance --no-deps --all-features` under `RUSTDOCFLAGS="--cfg docsrs -D warnings"` | AC-009, AC-010, NF-004 — the badge only exists under `docsrs`, and the page must read completely with the feature **off**. |
| `wasm32` | `cargo check -p happenstance --target wasm32-unknown-unknown --no-default-features --features std,json,unstable-projection` | AC-011, NF-003 — the exact invocation `edge-flavour-and-wasm-claim` (M7) will register. This story leaves it able to pass; it does not register it. |
| MSRV | `cargo +1.97.1 check -p happenstance --all-features` | NF-006 — the floor is checked by running the compiler, per ADR-0029. |
| Static (boundary) | `git diff --stat main -- spec/ crates/happenstance-core/src crates/happenstance-testkit/src experiments/ xtask/src` — expected **empty** | AC-007, AC-011 — the five exclusions the PR boundary declares, asserted rather than assumed at review. |
| Story grain (wired) | `cargo xtask affected --base main` | fmt + `clippy -D warnings` + tests for the touched packages and their dependents (`.redkiln/config.yaml:28-40`). |
| Integration grain (wired) | `cargo xtask ci --fast` | This project's declared bar, including all four existing `wasm32` steps and the `--no-default-features` doc build of `happenstance-core`. |

**Merge DoD.** `cargo xtask ci --fast` green, **plus** `cargo hack check
--feature-powerset -p happenstance`, **plus** the explicit `cargo test -p happenstance
--features unstable-projection,memory` (the default-feature test run does not compile a
single line of this story's code), **plus** the three doc builds and the `wasm32` check.
A green `--fast` alone discharges neither AC-008 nor AC-010 and must not be reported as
if it did.

## Risks and coupling (PR-scoped)

| risk | why it bites here | mitigation inside this PR |
| --- | --- | --- |
| **`MemoryProjectionStore` has not landed when this story is picked up.** | It is another *project*'s deliverable (HS-P0010 AC-012), not another story's in this project, so no declared `depends_on` edge blocks the pick-up. Every transactional assertion in this spec is unrunnable without it. | EC-009 and AC-012: preflight it as the first act and **halt loudly**. A throwaway written here freezes a fixture shape this project's *Out of scope* explicitly assigns elsewhere (`_decomposition.md:843-857`). |
| **ADR-0007 describes a pump the tree does not contain.** | The most plausible wrong move in the story: writing the pump makes the ADR read correctly, makes the runner look like the design, and destroys the exact evidence PS-33's falsifier exists to collect. | AC-007 makes the *absence* a deliverable and the boundary assertion (empty `git diff` under `crates/happenstance-core/src`) makes it checkable. The verdict is HS-S0027's; this story only reports. |
| **`collect` is the shortest path to a working runner.** | It compiles, it passes AC-001 through AC-004 and AC-006, and it silently defeats ADR-0001 and ADR-0008 — the two ADRs that fought for `read`'s shape — and makes E2E-25 unwritable. | AC-005's pull-order test is written to fail against it specifically. Write that test before the loop, not after. |
| **Slice-mate coupling: HS-S0027 and HS-S0028 both consume this runner.** | The three are implemented in one context. A verdict or a measurement written against an intended runner is a verdict about an intention. | Land the runner complete and self-testing first, then the verdicts, then the measurement — the slice's internal order. Do not let a sibling's need reshape the runner's signature mid-slice; `_design.md` arbitrates. |
| **The feature is off by default, so the default test run compiles none of this.** | `cargo test -p happenstance` is green with the whole story missing, and so is `cargo xtask ci --fast` unless `--all-features` reaches it. | The merge DoD names the explicit `--features unstable-projection,memory` invocation and the powerset. Report both, or report neither as passing. |
| **The first intra-doc link from an ungated page to a gated item.** | Rustdoc treats a broken link as a hard error, and the configuration in which it breaks is the **default** one — the one a casual `cargo doc` runs. | NF-004 and AC-010 make all three doc configurations part of done; the conditional doc-line shape is already established by the design mock. |
| **`Batch<'a>`'s borrow is discovered as a compiler error rather than a design fact.** | It reads as an obstacle to route around (a clone, an owned wrapper, an `Arc`), and each route around it breaks the transactional invariant it encodes. | Context pack decision 6 states it in advance, with `_decomposition.md:697-700` behind it. If the borrow genuinely blocks a required behaviour, that is EC-010's defect-log route, not a workaround. |
| **`ProjectionId::new` is infallible, and an empty identifier can reach a checkpoint key.** | The runner is the first code that would notice. The convenient fix is one line in the contract crate. | EC-010: a defect-log entry for HS-S0032 naming the open question already recorded at `crates/happenstance-core/src/projection.rs:44-64`. Adding validation beside an existing infallible constructor is the two-constructors defect this repository names in prose. |
| **PS-27's `on_error: SkipPolicy` is what a builder API invites.** | It is the obvious design and it forces one wrong answer onto applications with two projections of different tolerance. | The spec ships halt-on-error and states the exclusion as evidence (AC-007). Skip-and-record is PS-27's subject and HS-P0010's suite rule; adding it here pre-empts both. |

## Dependencies

**Blocks on** (must be merged first):

- `codec-and-feature-forwarding` (HS-S0022, M3) — `run_projection` takes a `&C: Codec`
  and the runner decodes (ADR-0007), so the trait and its concrete `CodecError` must
  exist before the runner's error type can have a decode arm at all. M3 also creates
  `crates/happenstance/tests/`, `[package.metadata.docs.rs]` and
  `#![cfg_attr(docsrs, feature(doc_cfg))]`; this story extends them if they are there
  and adds them if they are not.

Also merged before this story runs, by milestone order rather than by a declared edge:
**M2** (`domain-event-and-decision-model`), whose `DomainEvent::EVENT_TYPES` and
`decode` the derived query and the decode step are built from; and **M4**
(`misbehaving-testkit-stores`), whose `GappyMemoryStore` AC-004's gap test consumes.

**Cross-project precondition, not a story edge:** HS-P0010
(`projection-store-freeze`) → `memory-projection-store`, which ships
`MemoryProjectionStore` behind the `memory` feature as its AC-012. It is in another
project, so nothing in this project's dependency graph blocks on it; AC-012 and EC-009
make the check explicit instead.

**Unlocks**:

- `projection-clause-verdicts` (HS-S0027, M5, slice-mate) — PS-33, PS-27 and PS-30 are
  answerable by counting only once the runner exists to be counted.
- `polling-cost-measurement` (HS-S0028, M5, slice-mate) — the N views × N reads cost is
  a measurement *of this runner*.
- `edge-flavour-and-wasm-claim` (HS-S0031, M7) — the fifth `wasm32` step compiling
  `happenstance` can only be registered against a crate that already builds there with
  this story's feature on.
- `defect-log-and-macros-verdict` (HS-S0032, M7) — every EC-010 routing this story
  produces is an entry in that story's BR-01 record.

## Anchors (progressive disclosure)

Everything load-bearing that is **not** in the context pack. Open a row when its
*when to open* column says to — not before, and not all at once.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/projection-store-freeze/memory-projection-store/spec.md` | The fixture this story's every transactional assertion runs against — its feature name, its construction and whether it has landed. The preflight in EC-009 is answered here, not by guessing. | **First act, before writing any test.** | AC-012, AC-003 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` | `## Signatures` (`:536-570`) fixes `Projection` and `run_projection` item for item; `## Shape decision` (`:647`) fixes the gating; `## Visibility and stability` (`:746`) says the stability promise is *"explicitly none"*; `## Transience policy` (`:828`) makes the runner **opened on demand**; `## Composition`, `## Density budget`, `## States` (*Feature off*) and `## Anti-patterns` (15) are the composition invariants. Human-approved (DoD 5) and **binding**. | Before writing the trait (Signatures), and again before touching `lib.rs` (Composition onward). | AC-001, AC-005, AC-008, AC-009, AC-010 |
| `.kb/decisions/0007-projection-runner-decodes.md` | The accepted atom: the split at the decode boundary (`:41-49`), the three shape decisions that ride with it (`:61-69`), the pump allocation (`:53-59`) and the **falsifier** this story's exit evaluates (`:23-25`). Immutable — a disagreement produces a new atom, never an edit. | Before the first line of the runner, and again before writing the implementation report's PS-33 evidence. | AC-001, AC-002, AC-007 |
| `references/adr/0007-projection-runner-decodes.md` | The long record behind the atom — the rejected alternatives and the reasoning a ~100-line atom cannot hold. Cite it by `file:line`; do not treat the atom as a replacement for it. | Only if the atom's summary is insufficient to settle a shape question. | AC-001, AC-007 |
| `crates/happenstance-core/src/projection.rs` | The port, all 139 lines: the transactional invariant in prose (`:13-30`), the pre-empted `set_checkpoint` search (`:20-26`), `ProjectionId` and its infallible-constructor open question (`:42-71`), the `Batch<'a>` GAT (`:92-99`), the inclusive `from` (`:101-106`) and the four methods (`:110`, `:117`, `:126`, `:138`). | Continuously, from the first line of the loop. | AC-003, AC-004, AC-012 |
| `crates/happenstance-core/src/store.rs` | `EventStore::read` (`:119`) and the laziness contract above it (`:103-118`) — why the stream is at the top level and not inside a future. Also `read_decision_model`'s `Vec` (`:321`), the asymmetry this story must not smooth over. | Before writing the read + chunk loop. | AC-005, AC-011 |
| `crates/happenstance-core/src/event.rs` | `SequencePosition::next` (`:265-280`) — `checked_add`, not `saturating_add`, with the comment explaining why the `None` arm is real; `EventType::from_static` (`:108`). | When writing the resume arithmetic. | AC-004 |
| `crates/happenstance-core/src/memory.rs` | `spawns_from_generic` (`:643-680`) — the worked pattern for writing a bound at the signature with per-bound reasoning attached, and one of the two tests that pin `read`'s shape. Do not touch either; read them as the model. | When a generic bound needs stating and it is not obvious where. | AC-011 |
| `crates/happenstance-core/src/query.rs` | `Query` / `QueryItem` — the single nomination vocabulary, and the derivation the projection must reuse rather than restate. | Before deriving the query. | AC-001 |
| `spec/E2E-CASES.md` | E2E-25 (`:654-676`) the chunked rebuild and the "caught up vs rebuilding" question the port cannot express; E2E-26 (`:677-696`) the decode failure with no representable home under the pump signature — the case `ProjectionError`'s decode arm answers. | Before designing `ProjectionError`, and again before writing the rebuild paragraph in the rustdoc. | AC-005, AC-006 |
| `spec/SPECIFICATION.md` | PS-27 and its *Rejects* (`:5405-5425`) — the per-runner failure policy named as the obvious wrong design; PS-30 (`:5462`); PS-33 (`:5529`); ES-32 (`:4021`) no tail seam at 0.1. **Read, never edited by this story.** | When tempted to add a failure policy or a fan-out runner, and when writing AC-007's evidence. | AC-007 |
| `standards/rust/20-two-flavour-ports.md` | RS-20-2 (`:86`) bind `EventStore`, not `SendEventStore`; RS-20-3 (`:153`) one flavour name per module; RS-20-5 (`:276`) there is no `dyn EventStore`. Each rule carries a compiled example and a named wrong implementation. | Before writing the runner's generic signature. | AC-011 |
| `standards/rust/13-sealing-and-exhaustiveness.md` | RS-13-4 (`:144`) `#[non_exhaustive]` and why a named struct beats a tuple when the arity is what a later pass wants to grow — the reasoning behind `Progressed { through, applied }`. | Before declaring `Progressed` and `ProjectionError`. | AC-006 |
| `standards/rust/30-error-taxonomy.md` | RS-30-2 (`:71`) carry a foreign error with `#[source]`, never a `String`; the rules for a two-parameter error enum with a concrete third cause. | Before writing `ProjectionError`. | AC-006 |
| `standards/rust/51-features-and-no-std.md` | RS-51-1 (`:12`) *a feature adds*; RS-51-2 the `dep:` / `?/` mechanism and the bare-`dep/feature` trap; RS-51-5 (`:188`) the docs.rs manifest configuration. | Before editing `[features]`. | AC-008, AC-010 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-2 (`:93`) no intra-doc link may resolve in only some configurations; RS-70-4 (`:195`) `doc_cfg` behind `cfg_attr(docsrs, …)`; RS-70-5 (`:243`) name the alternative that lost, once. | Before writing the module doc and the two item docs. | AC-009, AC-010 |
| `standards/rust/62-doctests-and-harnesses.md` | RS-62-2 close a `?`-using example with `# Ok::<(), E>(())`; how an `.await` example gets a hidden runtime. Governs both new items' doc examples. | While writing `run_projection`'s doc comment. | AC-009 |
| `crates/happenstance/src/lib.rs` | The mount point. `:35-52` carries the five *"Planned"* bullets — the fifth is this story's; `:49-51` is the exact bullet to convert; `:75` is `pub use happenstance_core::*;`; `:10` is the doctest-gated README that must keep compiling. | Before touching the crate root. | AC-009, AC-011 |
| `crates/happenstance/Cargo.toml` | The features that exist today and the comment that already states AC-A04's failure mode in the manifest's own words — the contract this story extends, not rewrites. | Before the first manifest edit. | AC-008, AC-010 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/codec-and-feature-forwarding/spec.md` | The dependency's own spec: what `Codec` and `CodecError` will look like, and which of `tests/`, the docs.rs block and `cfg_attr(docsrs, …)` it already created. Determines what this story inherits versus adds. | Before writing the decode step or the manifest edit. | AC-002, AC-010 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` | AC-A05 (`:391-398`) the streaming rule; AC-A04 (`:381-390`) forwarding; the architecture brief's AC-006 row (`:421`); *Deliberately not prescribed* (`:712-723`) on sync vs async `apply`; the fan-out consequence (`:697-700`); the testing brief's AC-006 row (`:784`) and *AC-006's fixture is a cross-project dependency* (`:843-857`); the UX brief's AC-U10 … AC-U16 (`:150-200`). | When this spec is silent and you need the brief's own reasoning. | AC-003, AC-005, AC-006, AC-012 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` | The other project's ACs — AC-002 (the transactional rule) and AC-012 (`MemoryProjectionStore`) — and the suite rules `skip_and_record_is_atomic` and `panicking_apply_rolls_back` that this story must **not** pre-empt. | When judging whether a test belongs here or there. | AC-007, AC-012 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | P1's first hour and activity A5 (beat 4), P3 the edge developer, P4's one bounded sitting — the goals every criterion above is framed from. | If a criterion's intent is unclear, or before rewording one. | AC-001, AC-009, AC-011 |
| `RUNBOOK.md` | `:3971-4102` phase 7's goal, work list and exit criteria; `:408-413` PS-33 evaluated at this phase's exit *and nowhere else*; `:3924-3928` PS-3's still-open verdict, which is why the feature is off. | For roadmap context on why this phase exists, and before writing AC-007's evidence. | AC-007, AC-008 |
| `.redkiln/config.yaml` | `:28-40` the story grain; `:50-55` this project's integration bar, `cargo xtask ci --fast`. What "green" is wired to mean regardless of who types the command. | Before claiming the gate passed. | AC-008 |
| `xtask/src/main.rs` | `:830-836` the `--fast` split — both feature powersets, `cargo deny` and the nightly `--cfg docsrs` build all dropped. **Outside this PR's boundary**: read to know what green proves, never edited here (the fifth `wasm32` step is M7's). | When deciding what a green `--fast` discharges. | AC-008, AC-010 |
| `CHANGELOG.md` | `:19-22` already claims `unstable-projection` exists. This story makes that true; it does not edit the file (that is HS-S0033's), and anti-pattern 15 forbids the discrepancy shipping. | When writing the manifest feature row. | AC-008 |

## Clarifications resolved during spec

1. **The AC set is exactly the twelve the front half enumerated** — AC-001 … AC-012.
   None added, none dropped. The ledger carries the same twelve ids.
2. **No conformance rule is added, and that is a decision rather than an omission.** The
   runner sits *above* the port, so no adapter can observe it and no adapter can fail a
   rule about it — decorative by this repository's own standard (`CLAUDE.md`, *The rule
   that matters*). The rules that will observe this behaviour are the **projection**
   suite's and belong to HS-P0010. AC-012's absence assertion (no `impl ProjectionStore
   for` under `crates/happenstance/`) is the observable form of "this story did not
   quietly become a port story".
3. **AC-005's streaming claim is verified by pull order, not by memory.** A memory
   assertion inside `cargo test` is non-deterministic and would be a flaky gate; a
   store whose stream records when each item was pulled gives a deterministic
   discriminator that a `collect`ing runner fails and a streaming one passes. NF-008
   states the general form of that rule: this story ships no timing or throughput
   assertion at all.
4. **AC-012 exists as its own criterion rather than as a note on AC-003** because the
   fixture is a *cross-project* dependency with no declared edge to block on. A note
   would have got no ledger row and no gate; a criterion halts the story loudly at the
   right moment (EC-009).
5. **The composition ACs are checked by tests that read the crate's own source**, not
   by a new `xtask` lint — `xtask/` is outside this story's boundary. `doc_surface.rs`
   (M3's file, extended here) using `include_str!` is the in-boundary instrument, and it
   is a real test: an unstyled, uncomposed render — four bare `pub use`s with the fifth
   roadmap bullet still standing — passes every compile assertion above and fails it.
6. **`GappyMemoryStore` is consumed, not built.** It is M4's
   (`misbehaving-testkit-stores`), merges before M5 by milestone order, and is named in
   `_design.md`'s `## Items` as `happenstance_testkit::GappyMemoryStore` behind the
   `memory` feature. If M4 slipped, AC-004's gap test seeds non-contiguous positions
   through the existing `MemoryEventStore` instead; what it must **never** do is assert
   literal positions to make the test easier.
7. **PS-18 is not in this story's clause list, and its absence is deliberate.** Its
   subject — a `reset` method and `ResetError` — does not exist in
   `crates/happenstance-core/src/projection.rs`, so it is not evaluable in this tree
   (`_decomposition.md:660-673`). Recording it as a documented exclusion is HS-S0027's.
8. **The story deliberately produces an *absence* as a deliverable** (AC-007): no core
   pump, no fan-out runner, no failure policy, no spec edit. Absences are hard to
   verify, so each is bound to a mechanical check — the empty `git diff --stat` over
   five paths — rather than to reviewer memory.

