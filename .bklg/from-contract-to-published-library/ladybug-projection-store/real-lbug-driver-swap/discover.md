---
item: HS-S0076
stage: discover
created: 2026-08-12T13:02:45.862Z
updated: 2026-08-12T13:02:45.862Z
template_sig: 86ce4036
rendered_sig: df7f9ab2
---

# Discover — Swap the stand-in for the real `lbug` driver

## Signal Ledger

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line — replace the `lbug` NOTE in `Cargo.toml` with the real dependency, re-point `Value`/`Error` off `stand_in`, delete `stand_in.rs`, retire `live_handle.rs` with the `port_shape.rs` lines that instantiate it, and rewrite `lib.rs`'s driver-absent and open-decisions sections at ADR-0025 | `_storymap.md:42` | One merge, and it deliberately lands with `todo!()` bodies still in place. The crate must compile against the real driver before anything is implemented against it. |
| **AC-003** — no `todo!()`, `#![allow(clippy::todo)]` gone, the crate builds against the real `lbug` dependency, and no `stand_in` type on any path the suite exercises | `project.md:189-192` | This story owns the driver and `stand_in` half; the `todo!()` and allow half is `fill-the-bodies-and-ps-34-disposition`'s (`_storymap.md:107`). |
| **DR-2** — the `stand_in` module must be **retired** when the real `lbug` types land, not left beside them; two type universes in one crate is how a suite ends up running against the stand-in | `project.md:142-145` | The strong reading, and T3 supplies a compile argument for it rather than a stylistic one. |
| **T3** — `stand_in` retires from the whole crate because the error enum is **shared**: `LadybugProjectionStoreError` carries `Driver(#[from] stand_in::Error)` and `live_handle.rs` imports that same enum, so the moment `Driver` wraps `lbug`'s error, `live_handle.rs` stops compiling | `_decomposition.md:361-373`; `crates/happenstance-ladybug/src/projection_store.rs:158-163`; `.../live_handle.rs:91-92` | The two modules cannot sit in different type universes while sharing one error type, and `projection_store.rs:152-157` says the sharing is deliberate. There is no narrow reading available. |
| **T1** — `LiveHandleProjectionStore` binds `type Batch<'a> = GraphWriteHandle<'a>`, a genuinely borrowed handle with no lifetime-free spelling; under a frozen `type Batch;` that impl **cannot exist**, and `port_shape.rs:77,79` instantiate generic code at it and must drop with it | `_decomposition.md:314-331`; `crates/happenstance-ladybug/src/live_handle.rs:177-180`; `.../tests/port_shape.rs:75-80` | The retirement is itself a verdict finding — the freeze deletes the counter-example that was evidence for freezing — and it costs nothing evidentially because both transcripts live outside the crate. |
| The transcripts are preserved outside the crate in `references/adapter-shapes.md:169-195` and `:363-367` and quoted in `spec/SPECIFICATION.md:4600-4615`; the ICE transcript *cannot* be kept in tree, because a file that ICEs the compiler fails the gate | `_decomposition.md:326-331`; `crates/happenstance-ladybug/src/live_handle.rs:85-87` | Retiring the instrument destroys no evidence. This is the fact that makes T1 payable rather than a loss. |
| The seams table — `Cargo.toml`'s NOTE at `:18-22` becomes the real dependency, `publish = false` at `:12` is removed, `lib.rs:24-32`'s driver-absent section is rewritten and `:51-66`'s open-decisions list is replaced by a link to ADR-0025 | `_decomposition.md:70-84`; `crates/happenstance-ladybug/Cargo.toml:12`, `:18-22`; `.../src/lib.rs:24-32`, `:51-66` | The `publish = false` removal is coupled to `PUBLISHABLE` and is `package-completeness-and-name-claim` (HS-S0080)'s to land; doing it here without M5's other half fails the gate at `xtask/src/package.rs:188-195`. |
| Slice rationale — the two `real-adapter` stories cannot be one PR and cannot be two slices; the swap merges first and compiles with `todo!()` bodies still in place, and the bodies follow and take the allow with them | `_storymap.md:60-66` | A driver swap with `todo!()` behind it is exactly the "adapter" this project exists to stop counting — which is why the two share a slice and why the second is the only thing that proves the first. |
| `depends_on: adr-0025-three-answers` — supplies the link target for `lib.rs:51-66`'s rewrite, and Q3's answer decides whether any runtime-gated feature appears in `Cargo.toml` beside the `lbug` dependency | `_storymap.md:42`; `_decomposition.md:293-310` | The open-decisions list cannot be replaced by a link to a record that does not exist, and the dependency block cannot be finalised before the blocking bridge is decided. |
| Standing prohibition — do not weaken `port_shape.rs` to make it compile; if a bound has to change, **the change is the finding** | `_decomposition.md:445-446` | `send_flavour::spawn_a_batch_across_an_await`'s `for<'a> S::Batch<'a>: Send` at `tests/port_shape.rs:61` collapses to `S::Batch: Send` once the lifetime is gone, and that collapse is PS-5's predicted ergonomic win observed. |
| `Cargo.toml:24-25` already carries tokio in dev-dependencies, which is fine and is **not** a precedent for a normal dependency | `_decomposition.md:305-307`; `crates/happenstance-ladybug/Cargo.toml:24-25` | Anything the blocking bridge needs at runtime is a new argument, not an extension of an existing one. |

## Questions

**Does `live_handle.rs` retire or get re-pointed? — answered, conditionally, and
the condition is read rather than chosen.** T1 and T3 together settle both
branches in advance: if `type Batch;` (no lifetime) merged, the module is
**retired**, because `GraphWriteHandle<'a>` has no lifetime-free spelling for a
`'static` store and the impl cannot exist; if the GAT survived, the module is
**re-pointed** at `lbug` in the same change, because it shares
`LadybugProjectionStoreError` and cannot lag behind it. Which branch fires is a
fact established by `preflight-and-unlike-axes` (HS-S0074), not a decision taken
here. In the retirement branch, `tests/port_shape.rs:77` and `:79` go with it and
`crates/happenstance-ladybug/src/lib.rs:78`'s re-export goes with them.

**Does `GraphStatement::parameters`' `Vec<(Box<str>, Value)>` survive contact with
`lbug`'s real value type? — deferred to `spec`, and answered by the compiler
rather than by argument.** It is flagged as deliberately unprescribed
(`_decomposition.md:414-416`). One outcome is load-bearing enough to call out
now: if the real `lbug` value type is not `Send + 'static`, `GraphWriteSet` stops
being `Send + 'static`, which falsifies one of the axes
`preflight-and-unlike-axes` committed and is therefore a **verdict finding**, not
a shape to work around with a conversion layer. Under M9 the axes document would
be superseded, not edited.

**How does `lbug`'s blocking API meet a non-blocking port (ADR-0025)? — not this
story's to answer; already answered upstream and only *carried* here.** This story
adds whatever `Cargo.toml` entries ADR-0025's Q3 answer implies and nothing more.
Introducing a `spawn_blocking` call, a runtime feature or a tokio dependency that
the record does not name would be deciding Q3 in the dependency block, which is
the private-helper failure `adr-0025-three-answers` (HS-S0075) names as its own
mutant.

**What counts as "structurally unlike"? — already on disk and read-only here.**
The axes were committed by HS-S0074 before this story opened
(`project.md:180-184`). This story must not restate or amend them; where the real
driver contradicts an axis, that is a finding for the verdict and a supersession
of the document, on M9's lifecycle (`references/evaluation/README.md:1-13`).

**Is `publish = false` removed here? — deferred, and to a specific story.** The
seams table lists the removal under this crate's `Cargo.toml`
(`_decomposition.md:76`), but M5 makes it inseparable from adding the crate to
`xtask/src/package.rs:86`'s `PUBLISHABLE` and copying `REQUIRED_FILES` into the
crate directory — otherwise the gate fails with the message at
`package.rs:188-195`. The whole of that lands in
`package-completeness-and-name-claim` (HS-S0080), which is where AC-010 lives.
This story leaves `publish = false` in place.

**Deferred to `spec`:** whether the `lbug` version is pinned exactly or by caret,
and what `--locked` implies for the cold-build measurement
`cold-build-cost-and-ci-shape` (HS-S0081) will take against this dependency graph.

## Decision

`happenstance-ladybug` currently type-checks against a hand-calibrated stand-in
for a driver it has never linked, and every claim the project wants to make —
that the suite ran, that the freeze was tested against a real graph engine — is
worthless until the stand-in is gone and the real `lbug` is in the dependency
graph. This story does that swap and nothing else: the real dependency replaces
the NOTE, `Value` and `Error` are re-pointed off `stand_in`, `stand_in.rs` is
deleted, `live_handle.rs` is retired or re-pointed according to which port merged
(with `tests/port_shape.rs`'s instantiations following it either way), and
`lib.rs`'s driver-absent and open-decisions sections are rewritten to point at
ADR-0025 — leaving a crate that compiles against the real engine with its
`todo!()` bodies and its scoped allow still in place, deliberately, so that the
next story's deletion of both is the thing that proves this one. The spec will
cover the dependency edit, the `stand_in` deletion with the shared-error-enum
consequence spelled out, the T1 branch selection as a read of the merged port
rather than a choice, the `port_shape.rs` line removals, and the `lib.rs`
documentation rewrite — with `port_shape.rs`'s bounds left un-weakened, because a
bound that has to change is a finding for the verdict. Nothing `[FROZEN]` is
touched here: `spec/SPECIFICATION.md:4600-4615` quotes the transcripts this story
retires the source of, and it is left exactly as it stands; the citation repairs
that AC-008 owes travel with the bodies in the next story
(`_storymap.md:67-70`).

## The wrong implementation

**`stand_in.rs` deleted and its behaviour kept as a shim.** A `mod driver` that
re-exports `lbug`'s types under the crate's existing names, with a thin wrapper
preserving the stand-in's calibrated behaviour wherever the real driver differs —
a `Value` conversion that smooths over a type mismatch, a `Connection::new` that
swallows an error variant the stand-in never had. `grep -rn stand_in
crates/happenstance-ladybug/` returns nothing, AC-003's literal check passes, the
crate compiles, `cargo xtask ci` is green, and the conformance suite that runs two
stories later is still measuring a fiction. The tell is any type in this crate
that exists to make `lbug` behave the way the skeleton assumed; the rule is that
where the real driver disagrees with the stand-in, the *crate* changes, and the
disagreement is written into the verdict as evidence about how good a stand-in
calibrated from published docs actually was.

**The `lbug` dependency made optional to keep the gate fast.** `lbug = { version =
"…", optional = true }` with `#[cfg(feature = "lbug")]` on the real paths and the
stand-in types retained behind `#[cfg(not(feature = "lbug"))]`. It looks
considerate — nobody pays the multi-minute C++ build on a plain `cargo check` —
and it is the single most damaging thing this story could do, for two reasons.
First, it saves nothing: the gate's `clippy` and `tests` steps are `--workspace
--all-features` and `cargo hack --feature-powerset` runs above them
(`_decomposition.md:381-387`), so every gate step turns the feature on. Second,
the `not(feature)` arm is **off** under `--all-features` and therefore never
compiled by the gate at all, so it is exactly the configuration in which a
developer's local `cargo test` runs the whole crate against the stand-in and
reports success. The guard is structural rather than vigilant: `lbug` is a
**non-optional** dependency, and `crates/happenstance-ladybug/Cargo.toml` carries
no `[features]` entry naming it or gating any module — which makes the mutant
unwritable rather than merely detectable. `cold-build-cost-and-ci-shape`
(HS-S0081) is where the build cost is actually addressed, and `--exclude` plus a
dedicated job is the lever that exists (`_decomposition.md:388-393`).

**`port_shape.rs` weakened to compile.** The quiet version of the same instinct:
`for<'a> S::Batch<'a>: Send` at `tests/port_shape.rs:61` relaxed, or the
`const _` block at `:75-80` trimmed to whatever still instantiates, so the swap
merges green. Every check passes and the one instrument in the crate that
disagrees with the type checker on purpose has been taught not to. The standing
prohibition is explicit (`_decomposition.md:445-446`): lines are removed only
because the type they name has been retired (T1), never because a bound stopped
holding — and a bound that stops holding is written into the verdict, because
`send_flavour`'s higher-ranked bound collapsing to `S::Batch: Send` is PS-5's own
predicted relief, observed.

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
