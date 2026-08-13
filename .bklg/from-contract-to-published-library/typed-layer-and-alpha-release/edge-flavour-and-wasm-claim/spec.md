---
item: HS-S0031
stage: spec
created: 2026-08-12T13:46:28.148Z
updated: 2026-08-12T13:46:28.148Z
template_sig: 87bbf1d0
rendered_sig: 42f5568f
---

# Spec — The typed layer's wasm32 claim, stated either way

## Scope lock

| Artefact | Path | What it fixes for this story |
| --- | --- | --- |
| Initiative (gold source) | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) | AC-07 *"the edge developer keeps their runtime"* (`:327`); the persona at `:216-220` and the journey *Event-source at the edge without hand-rolling it* (`:247-248`); DoD 13 (`:396`) |
| Project | [`.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md`](../project.md) | AC-015 (`:216-217`), DR-11 (`:152`), the project's integration bar (DoD 6, `:237`) |
| This spec | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/edge-flavour-and-wasm-claim/spec.md` | — |
| **Signed-off design (BINDING)** | [`_design.md`](../_design.md) | `## Shape decision`, the `wasm32 claim` row (`:648`) — the claim is **made**, not declined, and the step's exact argument list. `## Anti-patterns`'s standing paragraph (`:1008-1010`). `## Placement and re-export`'s coherence note (`:709-716`) |
| Architecture brief | [`_decomposition.md`](../_decomposition.md) | AC-A06 in full (`:399-410`); the AC-015 seam row (`:430`); composition root 5 (`:472-475`) |
| Testing brief | [`_decomposition.md`](../_decomposition.md) | the AC-015 instrument row (`:793`) — what the four existing steps do **not** prove |
| Story map row | [`_storymap.md`](../_storymap.md) | `:62` (M7 `alpha-release`); merge order `:130-132` |
| Roadmap pointer | `RUNBOOK.md:4131-4162` | why the alpha lands here, and what a published claim costs to retract |

## One-line PR slice

Settle AC-A06 by name — add a fifth `wasm32` step compiling `happenstance` itself
to `xtask/src/main.rs::REQUIRED` and to `wasm_steps()` — and pin the flavour
discipline for this crate's own generic code with an instrument that can fail:
every typed-layer entry point instantiated against a deliberately `!Send` store,
and the `Send` flavour still crossing a `tokio::spawn`.

## Executive summary

**Delta.** At HEAD the gate runs four `wasm32` steps and **none of them compiles
the crate a Workers application installs**: `happenstance-core`
(`xtask/src/main.rs:203`), the conformance harnesses (`:231`),
`happenstance-cloudflare` (`:252`) and `happenstance-neon` (`:271`). Project
AC-015 is phrased as *"all four `wasm32` steps are green"* (`project.md:216-217`)
and is therefore satisfiable with the typed layer never built for the edge target
at all. This PR closes that: a fifth step, `happenstance` at
`wasm32-unknown-unknown` with `--no-default-features --features std,json`, added
to `REQUIRED` (`:105`) and reached by name from `wasm_steps()` (`:784`).

**And the second half, which the step does not buy.** `Send` is a perfectly
available auto trait on `wasm32-unknown-unknown`; what is unavailable there is a
`Send` *store*. So a green `cargo check -p happenstance --target
wasm32-unknown-unknown` proves dependency, `std` and feature hygiene and proves
**nothing at all** about whether this crate's generic code bound `EventStore` or
`SendEventStore` — generic bodies type-check against their declared bounds,
instantiated or not. The flavour half needs a type-level instrument: the typed
layer's entry points instantiated against a `!Send` store. That is the part of
this story that can actually fail, and it is why the story blocks on
`command-loop` and `projection-trait-and-runner` — those are the generic entry
points there is anything to audit.

This PR adds **no public item** (`_design.md`'s `## Items` block, `:229-381`,
claims nothing here) and edits **no** frozen clause. It is a gate change, two
tests and a ban.

## Context pack

The decisions this story must honour, stated as decisions. Everything deeper is a
signposted anchor.

**D1 — The claim is made, not declined, and the design already chose the argument
list.** AC-A06 admits two outcomes: a fifth step, or a recorded statement that
`happenstance` makes no `wasm32` claim at the alpha (`_decomposition.md:407-410`).
`_design.md:648` took the first, and it is binding on this story: *"a fifth
`wasm32` step is added by name to `xtask/src/main.rs::REQUIRED` (`:105`) and to
`wasm_steps()` (`:784`), compiling `happenstance` with `--no-default-features
--features std,json`."* The rejected alternative is recorded there too — *"the
typed layer is what a Workers application `cargo add`s; a claim nothing compiles
is the silence AC-A06 exists to forbid."* **Do not re-open this.** The
implementer's remaining freedom is the step's comment and the instruments below,
not the verdict.

**D2 — A `wasm32` build and the flavour discipline are two claims and need two
instruments; conflating them is the trap.** `Send` exists on `wasm32`. Nothing in
a `cargo check` of this crate for that target rejects a `SendEventStore` bound —
the code compiles, and the failure appears later, in a Cloudflare user's crate,
against a Durable Object store that is `!Send` by construction. The gate step is
the *build* claim. The *flavour* claim is discharged by instantiating this crate's
generic entry points with a store that is genuinely `!Send`, following
`crates/happenstance-testkit/tests/local_conformance.rs`: `LocalMemoryEventStore`
is `Rc<RefCell<Vec<_>>>`-backed for a stated reason (`:72-88` — *"`RefCell` gives
up `Sync`, not `Send`"*, so a bare `RefCell` would prove nothing), and its
`!Send`-ness is itself proven by an autoref probe carrying a **positive control**
(`:399-455`). That type is test-local to the testkit and not importable, so this
story writes its own minimal one under `crates/happenstance/tests/` — and it
carries the positive control, or it is a probe that always answers "not Send" and
proves nothing.

**D3 — Bind `EventStore`, never `SendEventStore`, and reach the other flavour by
path.** `trait_variant` emits a blanket `impl<T: SendEventStore> EventStore for T`,
so the bare flavour is the weaker requirement and the one that accepts both
(`standards/rust/20-two-flavour-ports.md:86` RS-20-2; the pattern is written as a
doctest at `crates/happenstance-core/src/store.rs:80-92`). Import **one** flavour
name per module (RS-20-3, `:153`); having both in scope makes method calls
ambiguous (CLAUDE.md, binding constraint 4). A type that must satisfy both
flavours is **two types**, not one with two impls — the blanket impl makes one
type `error[E0119]` (RS-20-4, `:203`), which is why `_design.md:650` ships
`FaultyStore` and `SendFaultyStore` as a pair. There is no `dyn EventStore`
(RS-20-5, `:276`).

**D4 — The `Send` flavour must still compose, and the test that proves it holds
the stream across an await inside a real spawn.** ADR-0008's whole point is that
`read` returns the stream at the top level and is not `async`
(`.kb/decisions/0008-one-derivation-for-both-ports.md`; ES-2,
`spec/SPECIFICATION.md:2492`). The assertion that survives an `async fn read`
refactor is not "this concrete stream is `Send`" — that passes by auto-trait
leakage — but `spawns_from_generic` (`crates/happenstance-core/src/memory.rs:643-680`),
which spawns a task holding the stream across `collect`'s await. Copy the bound
verbatim and the reasoning with it: `S: SendEventStore + Send + Sync + 'static`
(`:666`), where `Sync` and `'static` are load-bearing and `Send` is redundant-but-
stated (`:651-665`). The typed layer's own analogue belongs on whichever entry
point a caller would spawn — the projection runner is the obvious one — and it is
the only place in this crate where `SendEventStore` may legitimately be named.

**D5 — A new gate step goes in `REQUIRED` and is reached by name, or it is not in
the gate.** `xtask/src/main.rs::REQUIRED` (`:105`) is composition root 5
(`_decomposition.md:472-475`): *"a step added to `.github/workflows/` instead is
invisible locally and drifts."* `wasm_steps()` selects by **name** through
`steps_named` (`:816-826`), which panics when a name resolves to nothing — that
replaced an index selection that had silently pointed `cargo xtask wasm` at clippy
while printing green (`:769-782`). That panic is this story's negative control:
deleting the step from `REQUIRED` while the name stays in `wasm_steps()` fails
loudly. The step carries `--locked` (RS-80-4, `standards/rust/80-the-gate.md:245`)
and **no probe** — the four existing `wasm32` steps are mandatory, and a probe
means *skip when the tool is absent*, never *ignore when it fails* (RS-80-2,
`:98`). Adding it to `REQUIRED` puts it inside `cargo xtask ci --fast`
automatically (`xtask/src/main.rs:853-860`), which is this project's integration
bar (`.redkiln/config.yaml:55`).

**D6 — One feature combination is not the claim; the powerset is where the rest
lives.** `--no-default-features --features std,json` is one point in
`happenstance`'s feature space. `default = ["std", "memory"]` today
(`crates/happenstance/Cargo.toml:22-26`) and this project adds `json`, `cbor`,
`postcard` and `unstable-projection` (`_design.md:731-751`) — so the mandatory
step compiles neither the projection runner (off by default) nor the alternative
codecs for the edge target. **A feature is not target-scoped** (RS-52-2,
`standards/rust/52-wasm32-and-target-cfg.md:59`), which is precisely how the
testkit's `proptest` feature came to need a second `cfg` condition
(`xtask/src/main.rs:576-586`). The honest close is to add `-p happenstance` to the
existing `wasm32 feature powerset` step (`:564-590`) — a **widening above** the
mandatory guard, never a replacement for it, because that step is `OPTIONAL` and
probed, and *"a constraint whose only check is skippable is unguarded on every
machine that lacks one tool"* (`:198-202`).

**D7 — `#[async_trait]` is a standing prohibition with no mechanical guard in this
crate, and the cheapest real one is a manifest ban.** It injects `+ Send`, which
makes the Cloudflare target impossible (ADR-0001,
`.kb/decisions/0001-async-port-flavours.md`; CLAUDE.md binding constraint 1;
`_design.md:1008-1010` keeps it standing and not re-litigated). Nothing today
fails if someone adds `async-trait = "0.1"` to a manifest to make `Projection`
object-safe. `deny.toml`'s `[bans]` section exists (`:22-24`) and takes a `deny`
list; that is where the ban goes. Be honest about its strength in the same breath:
`cargo deny` is `OPTIONAL` and probed (`xtask/src/main.rs:595-601`), so the ban is
the *legible* guard and D2's `!Send` instantiation is the unskippable one.

**D8 — The persona slice this realizes.** *The local-first and edge Rust
developer* (`initiative.md:216-220`), journey *Event-source at the edge without
hand-rolling it* (`:247-248`). Their fear is named: *"being on the less-supported
path and being orphaned."* The concrete moment this story serves is the one after
`cargo add happenstance` — the crate ADR-0006 gave the bare name to
(`.kb/decisions/0006-bare-name-to-the-typed-layer.md`) — when they run
`cargo build --target wasm32-unknown-unknown`. Either that has been compiled in
this repository's own gate before they tried it, or the claim was prose.

**D9 — What this story is explicitly not licensed to do.** It adds no public item
(`_design.md`'s `## Items`, `:229-381`, assigns none here) and therefore makes no
semver promise of its own. It does not edit `crates/happenstance-core/src/**` —
inadmissible except as AC-012's recorded route (`project.md:132-134`, AC-A02). It
does not amend a `[FROZEN]` ES-\* clause: ES-2 (`spec/SPECIFICATION.md:2492`),
ES-3 (`:2523`) and ES-6 (`:2629`) are honoured, not touched. And it does not
narrow, reorder or rename the four existing `wasm32` steps — they are the standing
guard on ADR-0001 and *"not something a scope is allowed to narrow"*
(`xtask/src/main.rs:845-848`).

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — the edge developer's build is user-observable, and the gate step is what makes it true rather than asserted |
| **Slice / milestone** | `alpha-release` (M7). Slice-mates: `defect-log-and-macros-verdict` (independent) and `publish-0-2-0-alpha-1`, which blocks on this story (`_storymap.md:64`, `:130-132`). All three are implemented in one context and mounted as one surface |
| **Mount point** | `xtask/src/main.rs::REQUIRED` (`:105`) — composition root 5 (`_decomposition.md:472-475`). The step is *reached* from `wasm_steps()` (`:784-791`) by name via `steps_named` (`:816-826`), and inherited by `run_ci` (`:828-833`) and `run_fast` (`:853-860`) with no further wiring |
| **Wires into** | `crates/happenstance/Cargo.toml` `[features]` (`:22-26`) — the step names `std,json`, so the codec story's feature must exist and be spelled that way · `xtask/src/main.rs:564-590`, the `wasm32 feature powerset` (`OPTIONAL`) · `deny.toml` `[bans]` (`:22-24`) · `.redkiln/config.yaml:55`, which runs `cargo xtask ci --fast` as this project's integration gate · `crates/happenstance-core/src/store.rs:80-92` (the bare-flavour bound, as a doctest) and `crates/happenstance-core/src/memory.rs:643-680` (the spawn shape) as the two patterns copied · `crates/happenstance-testkit/tests/local_conformance.rs:72-88`, `:399-455` as the `!Send` store and probe pattern |
| **Renders surfaces** | **none.** No id in `_design.md`'s `## Surfaces` manifest (`:47-83`) is rendered or changed here. The design row this story implements is `## Shape decision`'s `wasm32 claim` (`:648`), which is a gate decision, not a surface |
| **Public items** | **none.** `_design.md`'s `## Items` block (`:229-381`) claims no item for this story. Any `pub` item appearing in this PR is scope drift, not a bonus |
| **Conformance rule(s)** | **None, and the reason is structural: this story changes no port.** `happenstance-testkit`'s suite observes an *adapter's* behaviour; the flavour discipline of a *consumer* crate's generic code is not adapter-observable, and there is no adapter here to fail a rule. The suite's own two-flavour evidence already exists and is untouched — `local_conformance.rs` runs the whole suite against a `!Send` store (`:461-530`). This story's instruments are type-level and gate-level instead, and both are named in the AC table |
| **Clause(s)** | Discharges none; amends none. **Honoured, and this story is the first consumer-side check of them:** ES-2 (`spec/SPECIFICATION.md:2492`), ES-3 (`:2523`), ES-6 (`:2629`) — all `[FROZEN]` (`:8584-8588`). If the typed layer's shape turns out to contradict one, the route is AC-012's defect log, never an edit (`project.md:132-134`) |
| **Advances DoD scenario** | Initiative **DoD 13** — *"the gate is green on the assembled whole… `cargo xtask ci` passes on the exact tree that was published"* (`initiative.md:396-397`). This is the story that puts the typed layer inside what that sentence means. Contributes to **AC-07** (`:327`) and removes a precondition failure from **DoD 4** (`:369-372`), which is HS-P0013's to close |

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and
fails on any file changed outside it.

```
xtask/src/main.rs
deny.toml
crates/happenstance/tests/**
crates/happenstance/src/**
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/edge-flavour-and-wasm-claim/**
```

`crates/happenstance/src/**` is in the boundary for **documentation only** — the
one-line statement, on the entry points, of which flavour is bound and why
(`standards/rust/70-rustdoc-obligations.md`). No new item, no signature change, no
body change. If an entry point has to change shape to type-check against a `!Send`
store, that is the story finding a real defect: fix it here and say so in the
ledger, because that is exactly the failure this instrument exists to catch.

**In this PR**

- The fifth `wasm32` step in `REQUIRED`, named and commented, and its name added to
  `wasm_steps()`.
- `-p happenstance` added to the `wasm32 feature powerset` `OPTIONAL` step.
- A `!Send` store and an autoref `Send` probe with a positive control, under
  `crates/happenstance/tests/`, instantiating every generic entry point the typed
  layer exposes.
- A `spawns_from_generic`-shaped test for the `Send` flavour over a typed-layer
  entry point.
- `async-trait` banned in `deny.toml`.
- Rustdoc lines stating the bound at the entry points.

**Explicitly not in this PR**

- Any new public item, feature, or signature change (`_design.md:229-381`,
  `:729-751`).
- Any edit under `crates/happenstance-core/src/**` or to any `[FROZEN]` clause
  (`project.md:132-134`).
- Any change to the four existing `wasm32` steps, including reordering
  (`xtask/src/main.rs:845-848`).
- A `wasm-bindgen-test` *execution* harness for `happenstance`. The step is a
  `cargo check`, matching all four existing ones; executing the typed layer's
  tests on the edge runtime is the Cloudflare adapter's story (HS-P0013, initiative
  DoD 4).
- `CHANGELOG.md` and the README's `## Stability` section — `publish-0-2-0-alpha-1`
  owns both (`_storymap.md:64`).
- The `happenstance-macros` verdict and the BR-01 defect log — the slice-mate's
  (`_storymap.md:63`).

**Merge DoD.** `cargo xtask ci --fast` is green with **five** `wasm32` steps
listed in its output, `cargo xtask wasm` names the typed layer among what it runs,
and the `!Send` instantiation test compiles and passes on the host.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The typed layer compiles for the edge target, in the gate** | A `Step` in `REQUIRED` named distinctly (e.g. `"wasm32 build of the typed layer"`), `program: "cargo"`, args `check --locked -p happenstance --target wasm32-unknown-unknown --no-default-features --features std,json`, `env: &[]`, `probe: None`. The argument list is the design's, not the implementer's | `_design.md:648`; `xtask/src/main.rs:203-218` is the shape to copy; `crates/happenstance/Cargo.toml:22-26` |
| **The step is reachable by name, and its removal is loud** | The new name is appended to `wasm_steps()`'s list. `steps_named` panics with ``REQUIRED must contain the `{name}` step`` when a name resolves to nothing, so deleting the `REQUIRED` entry fails the build of the gate itself rather than quietly narrowing `cargo xtask wasm` | `xtask/src/main.rs:784-791`, `:816-826`, and the incident recorded at `:769-782` |
| **The step carries its own reasoning** | The comment states what the step buys (the crate a Workers application installs is compiled for the target it claims) **and what it does not** (`Send` exists on `wasm32`; this proves nothing about which flavour is bound). Every step in `REQUIRED` carries that kind of comment; a step that only says what it runs is the decorative shape this file warns about | `xtask/src/main.rs:192-202`, `:219-231`, `:245-251` |
| **Every feature combination the alpha ships compiles for the edge target** | `-p happenstance` added to the `wasm32 feature powerset` step, so `cbor`, `postcard`, `memory` and `unstable-projection` are compiled on `wasm32` in the combinations a user can select. `OPTIONAL` and probed, so it widens coverage and never becomes the guard | `xtask/src/main.rs:558-594`; RS-52-2 (`standards/rust/52-wasm32-and-target-cfg.md:59`) |
| **Generic code in `happenstance` binds the bare flavour** | A `!Send` store — `Rc<RefCell<Vec<SequencedEvent>>>`-backed, not a bare `RefCell`, which is still `Send` — implements `EventStore` in `crates/happenstance/tests/`. Every generic entry point the typed layer exposes (the command loop, the projection runner, the DSL's seams) is *instantiated* against it. A `SendEventStore` bound anywhere in the chain is a compile error at that instantiation | `crates/happenstance-testkit/tests/local_conformance.rs:72-88`; RS-20-2 (`standards/rust/20-two-flavour-ports.md:86`); `crates/happenstance-core/src/store.rs:80-92` |
| **The `!Send` probe cannot pass vacuously** | The autoref specialisation probe carries a positive control asserting `MemoryEventStore` **is** `Send` alongside the assertion that the local store is not — without it the test also passes when the probe is simply broken | `crates/happenstance-testkit/tests/local_conformance.rs:399-455`, and its stated reason at `:442-444` |
| **The `Send` flavour still composes across a spawn** | A test bounds `S: SendEventStore + Send + Sync + 'static`, wraps the store in `Arc`, and holds a typed-layer read across an await inside a real `tokio::spawn`. Bind the query to a local rather than inlining it (edition 2024 RPITIT lifetime capture, E0716), and collapse `Result<_, S::Error>` before the next await — `Error` carries no `Send` bound and ES-6 is frozen | `crates/happenstance-core/src/memory.rs:643-699`; RS-21-3 (`standards/rust/21-send-is-not-inherited.md:169`); ES-6 (`spec/SPECIFICATION.md:2629`) |
| **One flavour name per module** | Modules under `crates/happenstance/src/` import `EventStore` only; the `Send` flavour is reached by full path at the one spawn-shaped site. Both names in scope makes method calls ambiguous | RS-20-3 (`standards/rust/20-two-flavour-ports.md:153`); CLAUDE.md binding constraint 4 |
| **`#[async_trait]` cannot arrive by accident** | `async-trait` added to `deny.toml`'s `[bans]` deny list with a comment naming ADR-0001. `cargo deny` is probed and `OPTIONAL`, so this is the legible guard; the unskippable one is the `!Send` instantiation above | `deny.toml:22-24`; `xtask/src/main.rs:595-601`; `.kb/decisions/0001-async-port-flavours.md` |
| **The four existing `wasm32` steps stay green and unaltered** | `happenstance-core` (`:203`), the conformance harnesses (`:231`), `happenstance-cloudflare` (`:252`), `happenstance-neon` (`:271`) — unchanged in name, order and arguments. They are the standing guard on ADR-0001 | `xtask/src/main.rs:845-848`; `project.md:216-217` |
| **The claim is stated where a reader meets it** | The step's existence is the claim. The crate's rustdoc says nothing stronger than the gate checks — in particular, no prose promising an *executed* edge test, which is HS-P0013's | `_decomposition.md:407-410`; initiative DoD 4 (`initiative.md:369-372`) |

## Data and migrations

**N/A.** This story adds no persisted state, no schema, no wire-format change and
no on-disk artefact. Its whole deliverable is three build-time facts — a gate step,
two type-level tests and a dependency ban — plus documentation. The only formats
in reach are the event wire format (frozen by ADR-0016 and untouched here) and
`Cargo.lock`, which changes only if `cargo check` for a new target resolves a
dependency the workspace did not already have; per `_design.md:648` it does not,
since `happenstance` adds nothing to `happenstance-core`'s graph beyond `serde`
and `serde_json`, both of which already build for `wasm32-unknown-unknown`. If the
lock file does move, that is a finding for the ledger, not a routine diff.

## Acceptance criteria

Every row is a persona's moment, not a capability. The personas are the
initiative's own — **P3** *the local-first and edge Rust developer*
(`initiative.md:216-220`), journey *Event-source at the edge without hand-rolling
it* (`:247-248`); **P1** *the application author* (`:202-207`); **P4** *the
evaluator* (`:222-226`) — carried from
`_discovery/distillation/personas-and-journeys.md` because no `authority_tier:
product` atom exists in this tree yet (`initiative.md:228-236`). All seven rows
trace to project **AC-015** (`project.md:216-217`) and discharge architecture
**AC-A06** (`_decomposition.md:399-410`).

Verification names a real path. `crates/happenstance/tests/flavours.rs` is
**created by `command-loop`** (its PR boundary and its AC-008 row), so this story
*extends* it rather than opening a second file; the `xtask` unit tests are new and
land in a `mod tests` inside `xtask/src/main.rs`, which is inside this PR's
boundary.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** P3, who has just run `cargo add happenstance` — the crate ADR-0006 gave the bare name to — and whose target is `wasm32-unknown-unknown`, **WHEN** they run `cargo build --target wasm32-unknown-unknown` on their Workers project, **THEN** it compiles, because this repository's own **mandatory** gate compiled that same crate for that same target first: a fifth `Step` sits in `xtask/src/main.rs::REQUIRED` (`:105`, composition root 5) with `program: "cargo"`, args `check --locked -p happenstance --target wasm32-unknown-unknown --no-default-features --features std,json` exactly as `_design.md:648` fixed them, `env: &[]` and **`probe: None`** — persistent chrome, never opened on demand, because a probed step means *skip when the tool is absent* and this guard may never be skippable (RS-80-2, `standards/rust/80-the-gate.md:98`); it carries `--locked` (RS-80-4, `:245`); its name is a grammatical peer of the four it joins (`"wasm32 build of the …"`) so the `=== name ===` transcript still scans as one family; and it carries a real comment stating **both** what it buys (the crate a Workers application installs is compiled for the target it claims) and what it does not (`Send` exists on `wasm32`, so this proves nothing about which flavour is bound) — a step whose comment only restates its arguments is the decorative shape `xtask/src/main.rs:192-202` warns about | `cargo xtask ci --fast` — the project's declared integration bar (`.redkiln/config.yaml:55`) — prints **five** `=== wasm32 … ===` sections and exits 0. Backed by `xtask/src/main.rs::tests::typed_layer_wasm_step_carries_the_designed_arguments` (new): resolves the step out of `REQUIRED` by name and asserts the exact arg vector, `probe.is_none()`, and that its doc comment block is non-empty and names both halves |
| **AC-002** | **GIVEN** a maintainer six months from now refactoring `xtask/src/main.rs` — reordering steps, renaming one, or deleting what looks redundant — **WHEN** they remove or rename the typed layer's step, **THEN** the gate fails **loudly and by name** rather than narrowing in silence: the step's name is listed in `wasm_steps()` (`:784-791`), which resolves through `steps_named` (`:816-826`) and panics with ``REQUIRED must contain the `{name}` step`` — the replacement for an index selection that once pointed `cargo xtask wasm` at clippy while printing green (`:769-782`); **and** the reader's position is preserved: the four existing steps keep their names, their order and their argument lists byte-for-byte, the new one is appended after them, and `cargo xtask wasm` therefore prints the same four sections in the same order followed by one more | `xtask/src/main.rs::tests::wasm_steps_resolve_and_number_five` (new): calls `wasm_steps()` (the panic *is* the assertion) and asserts `len() == 5` with the four existing names in their original order at indices 0..4. Negative control, run by hand and recorded in the ledger: delete the `REQUIRED` entry, confirm `cargo xtask wasm` panics rather than printing green. `cargo xtask wasm` lists five steps |
| **AC-003** | **GIVEN** P3 on a Cloudflare Durable Object, whose store is `Rc`-backed and therefore genuinely `!Send`, **WHEN** they call **any** entry point this crate exposes — `commit`, `commit_with`, `run_projection`, the `testing` DSL's seams — **THEN** every one of them accepts their store, because this crate's generic code binds `EventStore` and never `SendEventStore` (the weaker requirement, which `trait_variant`'s blanket impl makes accept both — RS-20-2, `standards/rust/20-two-flavour-ports.md:86`), each module imports exactly **one** flavour name (RS-20-3, `:153`; CLAUDE.md constraint 4), and no signature says `dyn EventStore` (RS-20-5, `:276`); the proof is an *instantiation*, not a bound read off the source, because a generic body type-checks against its declared bounds whether or not anything instantiates it | `crates/happenstance/tests/flavours.rs::every_entry_point_binds_the_weak_flavour` (extends `command-loop`'s file): a local `Rc<RefCell<Vec<SequencedEvent>>>`-backed store — `Rc`, never a bare `RefCell`, which is still `Send` (RS-25-1/RS-25-5, `standards/rust/25-what-removes-send-and-sync.md:12`, `:191`) — implementing `EventStore`, and one call to **each** entry point against it. A `SendEventStore` anywhere in the chain is a compile error here. Run under `cargo test -p happenstance --all-features` so the `unstable-projection` runner is included |
| **AC-004** | **GIVEN** the same maintainer, who has to be able to *believe* AC-003 rather than take it on trust, **WHEN** the `!Send` instrument runs, **THEN** it cannot pass vacuously: the autoref-specialisation probe that observes the *absence* of an auto trait carries a **positive control** asserting `MemoryEventStore` **is** `Send` in the same test — without it the assertion passes just as happily when the probe is simply broken and always answers `false`, which is the class of decorative check this repository has removed twice (RS-61-3, `standards/rust/61-compile-time-assertions.md:171`, which forbids the probe *without* a control by name) | `crates/happenstance/tests/flavours.rs::the_local_store_is_not_send` (new), written after `crates/happenstance-testkit/tests/local_conformance.rs:399-455`: positive control first, then the negative assertion, each with the failure message that names why. Discriminator, run by hand and recorded: break the probe (delete the `impl<T: Send> Probe<T>` inherent block) and confirm the **positive control** fails |
| **AC-005** | **GIVEN** P1 on a multi-threaded Tokio service who moves a projection replay onto its own task, **WHEN** they `tokio::spawn` a typed-layer read, **THEN** the `Send` flavour still composes: a generic function bounded `S: SendEventStore + Send + Sync + 'static` holds the stream across an await **inside a real spawn**, `Sync` and `'static` load-bearing and `Send` redundant-but-stated exactly as `crates/happenstance-core/src/memory.rs:651-680` records; the query is bound to a local rather than inlined (edition 2024 RPITIT lifetime capture, E0716); and every `Result<_, S::Error>` is collapsed **before** the next await, because `Error` carries no `Send` bound and ES-6 is `[FROZEN]` (`spec/SPECIFICATION.md:2629`; ADR-0009; RS-25-4, `standards/rust/25-what-removes-send-and-sync.md:130`). This is the assertion that survives an `async fn read` refactor, which a "this concrete stream is `Send`" assertion does not (ADR-0008; ES-2, `:2492`) | `crates/happenstance/tests/flavours.rs::run_projection_spawns_from_generic` (new), shaped after `crates/happenstance-core/src/memory.rs:643-699` and sitting beside `command-loop`'s `commit_spawns_from_generic` — this row extends the claim from the write path to the runner, which is the entry point a caller actually spawns. It is the **only** site in this crate where `SendEventStore` may be named, and it is reached by full path |
| **AC-006** | **GIVEN** P3 who does not take the defaults — they want `postcard` for payload size, or `cbor`, or the `unstable-projection` runner on the edge — **WHEN** they select any combination the alpha ships, **THEN** it compiles for `wasm32-unknown-unknown`, because `-p happenstance` is added to the existing `wasm32 feature powerset` step (`xtask/src/main.rs:558-594`) and every combination is compiled on that target rather than only the mandatory step's single `std,json` point. **A feature is not target-scoped** (RS-52-2, `standards/rust/52-wasm32-and-target-cfg.md:59`) — which is precisely how the testkit's `proptest` feature came to need a second `cfg` condition (`xtask/src/main.rs:576-586`). This is a **widening above** the mandatory guard and never a replacement for it: the powerset step is `OPTIONAL` and probed on `cargo hack --version`, and *"a constraint whose only check is skippable is unguarded on every machine that lacks one tool"* (`:198-202`) | `cargo xtask ci` (not `--fast`) runs the step; `cargo hack check -p happenstance-core -p happenstance-neon -p happenstance-testkit -p happenstance --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` run directly. Backed by `xtask/src/main.rs::tests::the_wasm32_powerset_covers_the_typed_layer` (new): resolves the `OPTIONAL` step by name and asserts `-p happenstance` is in its args **and** that the step still carries its `cargo hack` probe (a mandatory powerset would break every machine without the tool) |
| **AC-007** | **GIVEN** P3, whose stated fear is *"being on the less-supported path and being orphaned"* (`initiative.md:216-220`), and a future contributor who reaches for `#[async_trait]` to make `Projection` object-safe, **WHEN** that contributor adds `async-trait` to any manifest in the workspace, **THEN** it is refused rather than merged: `async-trait` is listed in `deny.toml`'s `[bans]` deny list (`:22-24`) with a comment naming ADR-0001 and the reason — the attribute injects `+ Send`, which makes the `wasm32` target impossible (`.kb/decisions/0001-async-port-flavours.md`; CLAUDE.md constraint 1; `_design.md:1008-1010` keeps it standing and not re-litigated). **And the promise is legible where the reader meets it, in place**: each typed-layer entry point's own rustdoc states which flavour it binds and why, on the item itself rather than in a separate "edge notes" section a reader must jump to (`standards/rust/70-rustdoc-obligations.md`), holding the design's density budget — first sentence ≤ 80 characters and a complete claim, doc prose ≤ 80 columns. The ban's strength is stated honestly in the same breath: `cargo deny` is `OPTIONAL` and probed (`xtask/src/main.rs:595-601`), so this is the **legible** guard and AC-003's instantiation is the unskippable one | `cargo deny check bans`. Backed by `crates/happenstance/tests/manifest_contract.rs::async_trait_is_banned` (extends M3's file): `include_str!("../../../deny.toml")` asserts the entry and a non-empty reason comment naming ADR-0001 — an unskippable check that the *ban line* exists even where `cargo deny` does not. The rustdoc claim is asserted by `crates/happenstance/tests/doc_surface.rs::entry_points_state_their_flavour` (extends M5's file) over `include_str!("../src/lib.rs")` and the module sources, plus the first-sentence and column budgets |

## Interaction quality

RFC §6.7/D6. **This story renders no surface** — no id in `_design.md`'s
`## Surfaces` manifest (`:47-83`) is created or changed here, and the design row it
implements is `## Shape decision`'s `wasm32 claim` (`:648`), which is a gate
decision. Two things are nonetheless *composed* and would be silently degraded by a
technically-correct change: the gate's own terminal transcript, and the rustdoc line
that tells a reader which flavour an entry point binds. Every invariant below is
carried by an **AC row in the table above**; this section is the map, not a second
list.

**State invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the step lands in `REQUIRED`, the one composition root, and nowhere else. A step added to `.github/workflows/` instead is invisible locally and drifts (`_decomposition.md:472-475`; RS-80-1, `standards/rust/80-the-gate.md:11`). The rustdoc flavour statement likewise lands on the item, not in a separate section | AC-001, AC-007 | `xtask/src/main.rs::tests::typed_layer_wasm_step_carries_the_designed_arguments`; `doc_surface.rs::entry_points_state_their_flavour`; no file under `.github/` is in the PR boundary |
| **Non-occlusion** — a failure names its own step. `run_steps` prints `=== {name} ===` **before** running (`xtask/src/main.rs:860-861`), so the failing step is identified; and because the step is unprobed, a failure can never be rendered as the word `skipped:` (`:872-875`), which would occlude it | AC-001 | The `probe.is_none()` assertion in AC-001's xtask test |
| **Preserved position** — the four existing `wasm32` steps keep name, order and arguments, so a reader who knows the transcript still recognises it; the fifth is appended, never interleaved (`xtask/src/main.rs:845-848`, *"not something a scope is allowed to narrow"*) | AC-002 | `wasm_steps_resolve_and_number_five` asserts the four original names at their original indices |
| **Reversibility, and it is loud** — removing the step cannot be silent. `steps_named` panics on a name that resolves to nothing, so the deletion fails the build of the gate itself | AC-002 | `wasm_steps_resolve_and_number_five` (the panic is the assertion) plus the hand-run negative control recorded in the ledger |
| **Reachable with what a human already types** — the keyboard-reachability analogue for a library. The step runs under `cargo xtask ci`, `cargo xtask ci --fast` and `cargo xtask wasm` with no extra flag, env var or tool; `REQUIRED` membership puts it inside `--fast` automatically (`:853-860`) | AC-001 | `cargo xtask ci --fast` green with five `wasm32` sections |

**Composition invariants** (from `_design.md`, binding)

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — a `Step` is not done when its arguments are right. Every `REQUIRED` step carries prose stating what it buys and what it does not (`xtask/src/main.rs:192-202`, `:219-231`, `:245-251`); a comment that restates the arg vector is bare markup | AC-001 | The doc-comment assertion in `typed_layer_wasm_step_carries_the_designed_arguments` — non-empty, and naming both halves |
| **Transience — persistent chrome, not opened on demand** | AC-001, AC-006 | `probe: None` on the mandatory step (persistent); the probe **retained** on the `OPTIONAL` powerset step, asserted by `the_wasm32_powerset_covers_the_typed_layer`. Getting either backwards is the failure: a probed guard is unguarded, a mandatory powerset breaks every machine without `cargo hack` |
| **Hierarchy** — the new name is a grammatical peer of the four (`"wasm32 build of the typed layer"`), so the transcript reads as one family rather than one stranger | AC-001 | The name assertion in AC-001's xtask test |
| **Density budget, with its real numbers** — `_design.md`'s *Density budget*: doc prose ≤ 80 columns, first sentence of an item ≤ 80 characters and a complete claim, identifiers ≤ 24 characters. `wasm_steps()` goes from four names to five and no further | AC-002, AC-007 | `doc_surface.rs::entry_points_state_their_flavour` measures per-line columns and first-sentence length; `cargo fmt --all --check` holds the source; `wasm_steps_resolve_and_number_five` holds the count |
| **Named anti-patterns** — `_design.md`'s standing paragraph (`:1008-1010`): *no `#[async_trait]`; generic code binds `EventStore`, not `SendEventStore`; `read` returns the stream at the top level*. Plus anti-pattern 9 (no colour, spinner, percentage or line that rewrites in place — the gate prints plain `println!` sections and this step adds nothing else) and anti-pattern 14 (nothing shadows a contract name) | AC-003, AC-005, AC-007 | `every_entry_point_binds_the_weak_flavour`; `run_projection_spawns_from_generic`; `async_trait_is_banned`; `doc_surface.rs::no_contract_name_is_shadowed` (M5's, unchanged and still green) |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | A dependency in `happenstance`'s graph does not build for `wasm32-unknown-unknown` under `std,json` | The new step **fails the gate**. It is unprobed by construction, so this can never surface as `skipped:`. The fix is the dependency or the feature, never the step: narrowing the arg list to make it pass re-creates the silence AC-A06 exists to forbid (`_decomposition.md:407-410`) |
| **EC-002** | The step is deleted from `REQUIRED` while its name stays in `wasm_steps()` | `steps_named` panics with ``REQUIRED must contain the `{name}` step`` (`xtask/src/main.rs:816-826`). This is the intended failure and the story's negative control — a `find` returning `None` silently would reproduce the index-selection defect it replaced (`:769-782`) |
| **EC-003** | `cargo hack` is absent on the machine | The `wasm32 feature powerset` step prints `skipped: …` and the run continues. Coverage narrows to the mandatory single combination; the guard on the claim does **not** disappear (RS-80-2, `standards/rust/80-the-gate.md:98`). `cargo xtask ci --fast` does not run it at all, by design |
| **EC-004** | A typed-layer entry point does not type-check against the `!Send` store — it bound `SendEventStore`, or inherited the bound through a helper | **This is the story working, not the story blocked.** Fix the bound in `crates/happenstance/src/**` (documentation-and-bound only; no new item, no signature *shape* change) and record the finding in the ledger's evidence. If the fault is in `happenstance-core`, it is **not** fixed here: it routes to AC-012's defect log naming its clause ID (`project.md:132-134`, AC-A02), never a line edit to the frozen crate |
| **EC-005** | `cargo deny` is absent, so the `async-trait` ban is unenforced on that machine | Accepted and stated. The manifest assertion in `manifest_contract.rs::async_trait_is_banned` still runs (it is a string read, not a tool), and AC-003's instantiation is the unskippable behavioural backstop. CI installs every optional tool, so the ban is enforced there |
| **EC-006** | `Cargo.lock` moves as a result of resolving for a new target | Treat as a **finding**, not a routine diff: `_design.md:648` asserts the step is near-free because `happenstance` adds nothing to `happenstance-core`'s graph beyond `serde`/`serde_json`, both of which already build for `wasm32-unknown-unknown`. A lock movement falsifies that reasoning; record it in the ledger and confirm `--locked` still holds (RS-80-4) |
| **EC-007** | `unstable-projection` is off by default, so the mandatory step compiles no runner at all | Expected and covered twice over: AC-006's powerset widening compiles it for `wasm32`, and AC-003/AC-005 exercise it on the **host** under `--all-features`. The mandatory step's arg list is `_design.md:648`'s and is not widened to chase this (see *Clarifications*, item 1) |
| **EC-008** | `projection-trait-and-runner` or `command-loop` has not landed when this story is picked up | **Halt loudly**, naming the missing artefact, rather than writing a placeholder entry point to instantiate. There is nothing to audit before those exist — which is the whole reason this story's `depends_on` is what it is |

## Non-functional

| id | Requirement | How it is held |
| --- | --- | --- |
| **NF-001** | The gate does not get materially slower | One `cargo check` of a small facade crate against a target already installed for the four existing steps. Same shape and same order of cost as `xtask/src/main.rs:203-218`; no new toolchain, no new tool, no test execution |
| **NF-002** | No consumer's dependency graph changes | The PR adds **no** `[dependencies]` entry. `tokio` is already a `[dev-dependencies]` entry on `happenstance` from `command-loop`, so AC-005 needs no manifest edit — which is why `crates/happenstance/Cargo.toml` is deliberately outside the PR boundary (see *Clarifications*, item 3) |
| **NF-003** | The MSRV floor of 1.97.1 does not move | ADR-0029 (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`). No new non-dev dependency, therefore no build script that can raise the floor underneath us (CLAUDE.md, binding constraint 5; RS-80-5, `standards/rust/80-the-gate.md:321`) |
| **NF-004** | The four existing `wasm32` steps are unaltered | Name, order and argument lists byte-identical. They are the standing guard on ADR-0001 and *"not something a scope is allowed to narrow"* (`xtask/src/main.rs:845-848`). Held by AC-002's index assertions and by review of the diff |
| **NF-005** | No public item is added, so no semver promise is made | `_design.md`'s `## Items` block (`:229-381`) claims no item for this story. Any `pub` item in this diff is scope drift (PR boundary, *Explicitly not in this PR*) |
| **NF-006** | No `[FROZEN]` clause is edited and `happenstance-core/src/**` is untouched | ES-2 (`spec/SPECIFICATION.md:2492`), ES-3 (`:2523`), ES-6 (`:2629`) are honoured, not amended. Verified by an empty `git diff --stat main -- crates/happenstance-core/src spec/` |
| **NF-007** | The `wasm32-unknown-unknown` target requirement does not grow | Already required by the four existing mandatory steps and installed in CI; this story adds no second target and no `wasm-bindgen-test` runner (that execution claim is HS-P0013's, initiative DoD 4) |
| **NF-008** | The packaging assertion still passes | `cargo xtask ci`'s packaging step (`xtask/src/main.rs:519-529`) asserts each publishable crate ships both licences and a README. Adding a `tests/` file must not disturb `happenstance`'s include set |

## Implementation notes (non-prescriptive)

Shape suggestions, not instructions. The verdict at D1 and the argument list at
`_design.md:648` are not among them.

- **Write the `!Send` instrument before the gate step.** The step is the cheap half
  and will pass on the first try; the instantiation is the half that can fail, and
  it is better met as a red test than as a surprise during the release story. If
  `command-loop`'s `flavours.rs` already carries a local `!Send` store, extend it —
  a second one in the same crate is two things to keep honest.
- **`Rc`, not `RefCell`.** `RefCell<T>: Send where T: Send`; `RefCell` gives up
  `Sync`, not `Send`. `Rc` is what actually removes `Send`, and it is also the
  honest shape — a Durable Object holds its store through an `Rc` on a
  single-threaded executor (`crates/happenstance-testkit/tests/local_conformance.rs:72-88`;
  RS-25-1/RS-25-5).
- **Gate the probe module on `cfg(not(target_arch = "wasm32"))`.** On `wasm32` an
  ungated probe is three `dead_code` warnings, and CI's ambient `-D warnings` turns
  those into a failure of the very step that type-checks the file for that target
  (`local_conformance.rs:399-412`; RS-52-3, `standards/rust/52-wasm32-and-target-cfg.md:110`).
- **The step's placement inside `REQUIRED`** reads best directly after the four
  existing `wasm32` steps and before `documentation` — the transcript then groups
  the whole target's story in one run of sections. Order inside `REQUIRED` is not
  semantically load-bearing; legibility is the only argument.
- **Say what the comment must say once, then stop.** The two halves are *the crate a
  Workers application installs is compiled for the target it claims* and *`Send`
  exists on `wasm32`, so this proves nothing about which flavour is bound; the
  instrument for that is `crates/happenstance/tests/flavours.rs`*. A cross-reference
  to the test file is what stops the next reader from assuming the step covers both.
- **One flavour name per module, and the exception is one site.** Modules under
  `crates/happenstance/src/` import `EventStore` only. `SendEventStore` appears in
  exactly one place in this PR — AC-005's spawn test — and it is reached by full
  path there, never imported (RS-20-3).
- **`deny.toml`'s `[bans]` section already exists** at `:22-24` with
  `multiple-versions` and `wildcards`; the `deny` array joins them, with a one-line
  comment naming ADR-0001. Verify the ban is real by adding `async-trait` to a
  scratch manifest and watching `cargo deny check bans` fail — a ban nobody has seen
  fire is the decorative shape again.
- **`xtask/src/main.rs` has no `mod tests` today.** `affected.rs`, `package.rs` and
  `lint_constitution.rs` all do (`xtask/src/affected.rs:597`,
  `xtask/src/package.rs:409`, `xtask/src/lint_constitution.rs:828`), so the pattern
  is in-tree; the new module reads `REQUIRED`/`OPTIONAL` directly and spawns no
  process.

## Tests and CI (merge gate)

Tiers as the project testing brief defines them (`_decomposition.md`, *Testing
brief → Acceptance Criteria* preamble). This project's declared integration bar is
`cargo xtask ci --fast` (`.redkiln/config.yaml:55`; `project.md` DoD 6), **not** the
full gate — which is why AC-006's powerset row is marked as a widening that a green
`--fast` does not prove.

| tier | command / path | proves |
| --- | --- | --- |
| Unit (gate wiring) | `cargo test -p xtask` → `xtask/src/main.rs::tests` (**new module**) | AC-001, AC-002, AC-006 — the step resolves by name, carries the designed argument vector, is unprobed, is a name-peer of the four; `wasm_steps()` numbers five with the original four at their original indices; and the `OPTIONAL` powerset covers `-p happenstance` while keeping its probe. Reads compile-time constants; spawns no process |
| Integration (flavour) | `cargo test -p happenstance --all-features --test flavours` → `crates/happenstance/tests/flavours.rs` (**extended**, created by `command-loop`) | AC-003, AC-004, AC-005 — every entry point instantiated against an `Rc`-backed `!Send` store, the autoref probe with its positive control, and the runner held across an await inside a real `tokio::spawn`. This is the tier that can actually fail |
| Unit (manifest) | `cargo test -p happenstance --test manifest_contract` → `crates/happenstance/tests/manifest_contract.rs` (**extended**, M3's) | AC-007 — the `async-trait` ban line and its ADR-0001 reason exist in `deny.toml`, asserted by string read so the check survives on a machine with no `cargo deny` |
| Unit (rendered surface) | `cargo test -p happenstance --test doc_surface` → `crates/happenstance/tests/doc_surface.rs` (**extended**, M5's) | AC-007's composition half — each entry point states its flavour **on the item**, first sentence ≤ 80 characters, doc prose ≤ 80 columns. The tier that fails on a technically-correct, unreadable page |
| Static (target) | `cargo check --locked -p happenstance --target wasm32-unknown-unknown --no-default-features --features std,json` | AC-001 directly, run standalone before it is wired |
| E2E / gate (`wasm32` only) | `cargo xtask wasm` | AC-001, AC-002 — five sections, the four originals first and unchanged |
| **Integration grain (wired)** | `cargo xtask ci --fast` | **The merge gate for this story.** `REQUIRED` without `OPTIONAL`: five `wasm32` sections, fmt, `clippy -D warnings`, the whole workspace test run (which collects every tier above), the proof artefacts, docs, `spec-trace`, the `--no-default-features` doc build and the packaging assertion |
| Story grain (wired) | `cargo xtask affected --base main` | fmt + `clippy -D warnings` + tests for `happenstance`, `xtask` and their dependents, re-derived from git so untracked test files are seen (`.redkiln/config.yaml:28-40`), plus the five lints and `spec-trace` |
| Release bar (**not** this story's gate) | `cargo xtask ci` | AC-006's powerset widening and AC-007's `cargo deny check bans`. Both live in `OPTIONAL` and are dropped by `--fast` (`xtask/src/main.rs:835-857`). Run once by hand before handing to `publish-0-2-0-alpha-1`, which runs the full gate as its own precondition |

**Merge DoD, restated as commands.** `cargo xtask ci --fast` green with five
`wasm32` sections · `cargo xtask wasm` naming the typed layer among what it runs ·
`cargo test -p happenstance --all-features --test flavours` green on the host ·
`cargo xtask ci` run once by hand, green, including `cargo deny check bans` and the
widened powerset.

## Risks and coupling (PR-scoped)

| Risk | Why it bites here | Containment |
| --- | --- | --- |
| **The step passes and proves less than its name suggests** | `Send` is available on `wasm32`; only a `Send` *store* is not. A green `cargo check` for that target says nothing about which flavour this crate's generic code bound. A reader who sees five green `wasm32` sections and concludes AC-015 is discharged has been misled by the gate | D2 is the whole reason there are two instruments. The step's own comment states the limit; AC-003/AC-004/AC-005 are the half that can fail. Do not let the step ship without the comment |
| **The instantiation test is vacuous** | An instrument that no wrong implementation fails is decorative — this repository's most-repeated defect (CLAUDE.md, *The rule that matters*, first corollary) | Two named negative controls, both **hand-run and recorded in the ledger**: break the probe and watch the positive control fail (AC-004); delete the `REQUIRED` entry and watch `steps_named` panic (AC-002). Neither is optional evidence |
| **The entry points do not exist yet** | AC-003 and AC-005 audit `command-loop`'s and `projection-trait-and-runner`'s generic signatures. Both are earlier milestones, and M5 is the *first* port-binding generic code in this crate (`projection-trait-and-runner/spec.md:212-213`) | Hard `depends_on`. EC-008 halts loudly rather than instantiating a placeholder. Both dependency specs already commit to the bare flavour, so this story audits rather than repairs — and if it must repair, EC-004 says how |
| **Overlap with `command-loop`'s AC-008** | That story already writes `commit_spawns_from_generic` and `commit_binds_the_weak_flavour` in the same file. Re-writing them here is duplicated responsibility; assuming they cover everything is a gap | Extend, do not re-open. This story's contribution is **coverage of every entry point** (notably `run_projection`, which M3 could not reach), the two negative controls, and the gate step |
| **The mandatory step's argument list drifts to chase a feature** | `unstable-projection` and the alternative codecs are not in `std,json`, and the temptation is to widen the mandatory step | `_design.md:648` fixed the list and is binding (D1). The uncovered combinations are AC-006's, in the `OPTIONAL` powerset, exactly where a widening belongs |
| **A probe is added "so it does not break anyone's machine"** | A probed guard is unguarded on every machine that lacks the tool, and that is how this constraint would quietly stop being checked | `probe: None` is asserted by AC-001's xtask test, so adding one fails `cargo test -p xtask`. RS-80-2 is the rule and `xtask/src/main.rs:198-202` is the standing comment |
| **`Cargo.lock` moves under `--locked`** | A new target can pull a dependency the workspace did not resolve, and `--locked` then fails the step rather than updating quietly | EC-006. That failure is the desired one: it is the design's cost assumption being falsified in the open |
| **Slice-mate coupling** | `publish-0-2-0-alpha-1` blocks on this story (`_storymap.md:130-132`) and runs the **full** gate as its precondition. A green `--fast` here that hides an `OPTIONAL` failure surfaces there, later and more expensively | Run `cargo xtask ci` whole, once, by hand before handing over — the *Release bar* row in the tests table exists for this reason, and its result belongs in the implementation report |

## Dependencies

**Blocks on** (must be merged first; matches this story's `depends_on`):

- **`command-loop`** (M3) — the write path's generic entry points, `commit` and
  `commit_with`, are two of the things AC-003 instantiates; and it is the story that
  **creates `crates/happenstance/tests/flavours.rs`** and adds `tokio` to
  `crates/happenstance/Cargo.toml`'s `[dev-dependencies]`, both of which this story
  consumes rather than re-creates (`command-loop/spec.md:197-199`, and its AC-008
  row at `:271`).
- **`projection-trait-and-runner`** (M5) — `run_projection` is the entry point a
  caller actually spawns, so it is AC-005's subject and the last entry point AC-003
  must reach. It is also *"the first generic code in `happenstance` that binds a
  port"* (`projection-trait-and-runner/spec.md:212-213`), and its own spec names the
  `wasm32` invocation as *"the exact invocation `edge-flavour-and-wasm-claim` will
  register"* (`:607`) while explicitly declining to register it (`:372-373`).

**Unlocks:**

- **`publish-0-2-0-alpha-1`** (M7) — names this story in its own `depends_on`
  (`_storymap.md:64`; merge order `:130-132`). The alpha cannot honestly carry an
  edge claim the gate has never compiled, and the release story runs the **full**
  `cargo xtask ci` as its precondition.

**Slice-mates, implemented in the same context and mounted as one surface:**
`defect-log-and-macros-verdict` (independent of this story; either order), then
`publish-0-2-0-alpha-1` last — *"the publish is the one irrevocable act in the
project"* (`_storymap.md:130-132`).

**Not a dependency, and named so silence is not read as one:**
`happenstance-cloudflare`. Executing the typed layer's tests on a real edge runtime
is initiative DoD 4 and HS-P0013's; this story's step is a `cargo check`, matching
all four existing ones.

## Anchors (progressive disclosure)

Open these when the row says to, not before. Each is load-bearing for one named
criterion; the `## Context pack` above already carries the decisions, so nothing
here needs reading to *start*.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `xtask/src/main.rs` | `:105` `REQUIRED` is the mount point; `:203-218` is the exact `Step` shape to copy, comment included; `:784-791` `wasm_steps()`; `:816-826` `steps_named`'s panic; `:769-782` the index-selection incident that panic exists to prevent; `:558-594` the `OPTIONAL` powerset; `:845-860` what `--fast` keeps and why the four `wasm32` steps are not narrowable | Before writing the step, and again before touching `wasm_steps()` | AC-001, AC-002, AC-006 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` | `:648` is **binding**: the claim is made, not declined, and the argument list is fixed there with its rejected alternative. `:1008-1010` keeps `#[async_trait]` standing without re-litigation. `:229-381` is the `## Items` block that claims **no** item for this story. `:47-83` is the `## Surfaces` manifest this story renders none of | Before writing a single line — this is the one artefact that can overrule the implementer | AC-001, AC-006, AC-007 |
| `crates/happenstance-testkit/tests/local_conformance.rs` | `:72-88` the `Rc<RefCell<…>>` store with the reason a bare `RefCell` proves nothing; `:399-455` the autoref probe **with its positive control** and the `cfg(not(target_arch = "wasm32"))` gate that keeps it from becoming three `dead_code` warnings under `-D warnings` | Before writing the `!Send` instrument. Copy the shape and the reasoning; the type itself is test-local and not importable | AC-003, AC-004 |
| `crates/happenstance-core/src/memory.rs` | `:643-699` `spawns_from_generic` — the bound `S: SendEventStore + Send + Sync + 'static` with each bound's removal tested and recorded, the query bound to a local for E0716, and the `map_or` that collapses `Result<_, S::Error>` before the next await | Before writing AC-005's spawn test. Copy the bound verbatim **and** the comments explaining which bounds are load-bearing | AC-005 |
| `standards/rust/20-two-flavour-ports.md` | RS-20-2 (`:86`) bind `EventStore`, not `SendEventStore`; RS-20-3 (`:153`) one flavour name per module; RS-20-4 (`:203`) a type satisfying both flavours is two types (`E0119`); RS-20-5 (`:276`) there is no `dyn EventStore`. Each rule carries a compiled example and a named wrong implementation | Before auditing the entry points' bounds | AC-003 |
| `standards/rust/25-what-removes-send-and-sync.md` | RS-25-1 (`:12`) `Rc` removes `Send`, `RefCell` removes `Sync`; RS-25-4 (`:130`) collapse a value with no `Send` bound before the next await; RS-25-5 (`:191`) build a `!Send` instrument out of `Rc`, never an inherited `unsafe impl Send` | Alongside `local_conformance.rs`, when writing the store and the spawn test | AC-003, AC-005 |
| `standards/rust/61-compile-time-assertions.md` | RS-61-3 (`:171`) — observe the absence of an auto trait with autoref specialisation *and never without a positive control*. The rule that makes AC-004 a criterion rather than a nicety | Before writing the probe | AC-004 |
| `standards/rust/80-the-gate.md` | RS-80-1 (`:11`) add a check as a `Step` in `REQUIRED` and reach it by name; RS-80-2 (`:98`) a probe means skip-when-absent, never ignore-when-failing; RS-80-4 (`:245`) every gate invocation that resolves dependencies carries `--locked`; RS-80-5 (`:321`) move the floor only in an ADR | Before writing the step, and again before considering a probe | AC-001, AC-002, AC-006 |
| `standards/rust/52-wasm32-and-target-cfg.md` | RS-52-2 (`:59`) a feature is not target-scoped — the rule behind the powerset widening; RS-52-3 (`:110`) a `cfg` covers the probe *and* its caller, which is why the `!Send` probe module is native-only | Before adding `-p happenstance` to the powerset, and before gating the probe module | AC-006, AC-004 |
| `.kb/decisions/0001-async-port-flavours.md` | The accepted atom behind the `#[async_trait]` prohibition and the two-flavour design; the `deny.toml` ban's comment must name it | When writing the ban's comment | AC-007 |
| `.kb/decisions/0008-one-derivation-for-both-ports.md` | Why `read` returns the stream at the top level and is not `async`, and therefore why AC-005's assertion must be a real spawn rather than a `Send` bound on a concrete stream | If AC-005's test seems replaceable by something simpler — it is not | AC-005 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/command-loop/spec.md` | `:197-199` names the `[dev-dependencies]` entry this story relies on; `:271` is AC-008, the flavour claim already made for the write path in the same test file. Determines what this story **extends** versus writes | Before opening `crates/happenstance/tests/flavours.rs` | AC-003, AC-005 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/projection-trait-and-runner/spec.md` | `:212-223` the runner's flavour obligations; `:393` `run_projection`'s signature and its bare-flavour bounds; `:473` its AC-011; `:607` the `wasm32` invocation it runs by hand; `:372-373` its explicit hand-off of the fifth step to this story | Before instantiating the runner, and before settling the mandatory step's feature list | AC-003, AC-005, AC-006 |
| `deny.toml` | `:22-24` the existing `[bans]` section — `multiple-versions` and `wildcards` — which is the block the `deny` array joins | When writing the ban | AC-007 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` | `:399-410` AC-A06 in full, including the two admissible outcomes and the sentence that silence is the failure; `:430` the AC-015 seam row; `:472-475` composition root 5; and the testing brief's AC-015 row on what the four existing steps do **not** prove | If the scope of "settle AC-A06" is ever in doubt | AC-001, AC-003 |
| `standards/rust/70-rustdoc-obligations.md` | What a doc line on an entry point must carry, and the density expectations AC-007's composition half is measured against | Before writing the flavour statements | AC-007 |

## Clarifications resolved during spec

1. **The mandatory step's feature list is `std,json`, not
   `std,json,unstable-projection`.** `projection-trait-and-runner/spec.md:607` runs
   `--no-default-features --features std,json,unstable-projection` by hand and calls
   it *"the exact invocation `edge-flavour-and-wasm-claim` will register"*, while
   `_design.md:648` — human-approved and therefore binding — fixes the *registered*
   step at `std,json`. **The design wins.** The runner's feature is not dropped: it
   is compiled for `wasm32` by AC-006's powerset widening, which is the correct home
   for a combination rather than a point. Recorded because the two artefacts
   disagree in the tree and the next reader will find both.
2. **AC count unchanged.** The seven ids the front half decided (AC-001 … AC-007)
   are enumerated exactly. Two claims from the *Behavior and interfaces* table are
   carried as non-functional rather than as acceptance rows, deliberately: *the four
   existing `wasm32` steps stay unaltered* is **NF-004** (an absence, verified by
   diff and by AC-002's index assertions, not a persona moment of its own), and *one
   flavour name per module* is folded into **AC-003**, whose instantiation is what
   makes the import discipline observable rather than stylistic.
3. **`crates/happenstance/Cargo.toml` is outside the PR boundary, and that is
   correct.** AC-005 needs an async test harness; `command-loop` already adds
   `tokio` to `[dev-dependencies]` for exactly this purpose
   (`command-loop/spec.md:197-199`), and `tokio` is pinned once in
   `[workspace.dependencies]` (`Cargo.toml:77`). This story therefore needs no
   manifest edit. **If, on pickup, that dev-dependency is absent**, that is a
   dependency contract broken upstream: add the single `[dev-dependencies]` line,
   record it in the ledger as an explicit boundary widening with its reason, and
   flag it in the implementation report — do not silently vendor an alternative
   harness (NF-002).
4. **Why there is no conformance rule.** `happenstance-testkit`'s suite observes an
   *adapter's* behaviour. The flavour discipline of a *consumer* crate's generic
   code is not adapter-observable, and this story ships no adapter for a rule to
   fail; CLAUDE.md's corollary — *"a rule that no adapter can fail is decorative"* —
   is the reason not to add one. The suite's own two-flavour evidence already exists
   and is untouched (`local_conformance.rs:461-530`).
5. **Two negative controls are hand-run, and that is stated rather than hidden.**
   Breaking the probe (AC-004) and deleting the `REQUIRED` entry (AC-002) cannot be
   permanent assertions — the first would be a deliberately broken test and the
   second a deliberately broken build. They are run once, observed, reverted, and
   their observation is cited as ledger evidence. An implementer who skips them has
   shipped two instruments nobody has ever seen fail.
6. **`xtask/src/main.rs` gains its first `mod tests`.** The pattern exists elsewhere
   in the crate (`xtask/src/affected.rs:597`, `xtask/src/package.rs:409`,
   `xtask/src/lint_constitution.rs:828`), and the new module reads compile-time
   constants only — it spawns no process and adds no dependency, so it costs the
   test step nothing measurable.
7. **The claim this story does *not* make.** The step is a `cargo check`. Nothing
   here asserts the typed layer's tests **execute** on an edge runtime; that is
   initiative DoD 4 and belongs to `happenstance-cloudflare` (HS-P0013). The crate's
   rustdoc must not say otherwise, which is the second half of AC-007's composition
   claim.
