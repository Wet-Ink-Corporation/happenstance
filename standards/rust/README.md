# The Rust constitution

Rust craftsmanship for **this** workspace. Twenty-seven atoms, each carrying a
handful of rules; every rule has a compiled example and names a wrong
implementation that could ship.

**Load one to three atoms, never the corpus.** Find yours in the trigger table
below, or guess the filename from the band — the numbering is stable.

| Band | Owns |
|---|---|
| `00`–`01` | what may never be traded, and what makes a rule here legitimate |
| `10`–`13` | types: newtypes, `const` constructors, manual impls, sealing |
| `20`–`25` | ports and async: flavours, `Send`, RPITIT, streams, the blocking bridge |
| `30` | errors |
| `40`–`41` | public surface, evolution, exported macros |
| `50`–`52` | Cargo: dependencies, features, `no_std`, wasm32 |
| `60`–`62` | proof: what a test must prove, compile-time assertions, doctests |
| `70` | rustdoc obligations |
| `80`–`81` | the gate, and checks a type cannot express |
| `90`–`92` | skeletons, the adapter recipe, toolchain dead ends |

## Precedence

> **SPECIFICATION clause > ADR > constitution atom > `CLAUDE.md` /
> `CONTRIBUTING.md` summary > `references/evaluation/*`.**
> Within an atom, **the compiled example beats the prose** — if they disagree the
> prose is wrong, because the example is the only part the gate checks.

Two consequences worth knowing before you write anything here:

- **Clause or atom?** Ask: *could a conformant adapter written in another
  language violate this sentence?* Yes → it is a clause in
  [`SPECIFICATION.md`](../../spec/SPECIFICATION.md), and the atom cites it.
  No → it is a Rust idiom, and the atom owns it. An atom never restates a
  clause's content.
- **`references/evaluation/*` is dated evidence, not doctrine.** It is pinned to a
  commit and may be stale. Worked example: its
  [Rust API audit](../../references/evaluation/research-rust-api-guidelines.md) §16 declares
  *"Confirmed defect: `SequencePosition::next()` does not detect overflow"* —
  which has since been fixed, and `crates/happenstance-core/src/event.rs:272`
  now uses `checked_add`. Cite it as a measurement that forced a rule; never
  promote its unexecuted recommendations into rules.

## Start here

| You are… | Load |
|---|---|
| starting any change | `00`, then the band you need |
| writing a new adapter | `91`, then `20`, `21`, `23`, `24` |
| told "cannot be sent between threads safely" | `25`, then `21` |
| designing a public type | `10`, `13`; add `11` for a `const` constructor |
| designing an error | `30` |
| adding a method to a port | `40`, then `20` |
| touching a manifest | `50`; add `51` for features, `52` for wasm32 |
| adding a test or a conformance rule | `60`; add `61` to assert a type property |
| adding a gate step | `80`, then `81` |
| hitting an ICE or an unstable feature | `92` |
| editing this corpus | `01` |

## Index

Generated from the atoms by `cargo xtask lint-constitution --write`, and checked
for equality by `cargo xtask lint-constitution`. The equality is the check; the
generator is not.

<!-- BEGIN GENERATED -->
| Atom | Load when | Rules |
|---|---|---|
| [`00-prime-directives.md`](00-prime-directives.md) | starting any change in this workspace · reaching for a trait | RS-00-1, RS-00-2, RS-00-3, RS-00-4, RS-00-5 |
| [`01-standard-of-evidence.md`](01-standard-of-evidence.md) | adding or editing a rule in `standards/rust/` · reaching for a | RS-01-1, RS-01-2, RS-01-3, RS-01-4 |
| [`10-newtypes-and-niches.md`](10-newtypes-and-niches.md) | choosing between `type X = u64` and `struct X(u64)` · adding a | RS-10-1, RS-10-2, RS-10-3, RS-10-4, RS-10-5 |
| [`11-const-construction-and-panics.md`](11-const-construction-and-panics.md) | writing a `const fn` constructor · a `from_static` with a bad | RS-11-1, RS-11-2, RS-11-3 |
| [`12-manual-impls-and-derive-traps.md`](12-manual-impls-and-derive-traps.md) | adding a field to a type that derives `Eq`/`Hash` · a `HashMap` | RS-12-1, RS-12-2, RS-12-3, RS-12-4, RS-12-5 |
| [`13-sealing-and-exhaustiveness.md`](13-sealing-and-exhaustiveness.md) | adding a public struct or enum that will grow · E0603, E0616, | RS-13-1, RS-13-2, RS-13-3, RS-13-4, RS-13-5 |
| [`20-two-flavour-ports.md`](20-two-flavour-ports.md) | writing a new store or projection adapter · deciding which | RS-20-1, RS-20-2, RS-20-3, RS-20-4, RS-20-5 |
| [`21-send-is-not-inherited.md`](21-send-is-not-inherited.md) | a generic runner will not compile · `S::Error` is not `Send` · | RS-21-1, RS-21-2, RS-21-3 |
| [`22-rpitit-and-lifetime-capture.md`](22-rpitit-and-lifetime-capture.md) | declaring a method on a port · an `async_fn_in_trait` warning | RS-22-1, RS-22-2, RS-22-3 |
| [`23-streams-and-state-machines.md`](23-streams-and-state-machines.md) | writing `poll_next` by hand · E0507 moving a cursor out of a | RS-23-1, RS-23-2, RS-23-3, RS-23-4, RS-23-5 |
| [`24-the-blocking-bridge.md`](24-the-blocking-bridge.md) | calling a synchronous driver from async code · `spawn_blocking` | RS-24-1, RS-24-2, RS-24-3, RS-24-4 |
| [`25-what-removes-send-and-sync.md`](25-what-removes-send-and-sync.md) | "cannot be sent between threads safely" · "cannot be shared | RS-25-1, RS-25-2, RS-25-3, RS-25-4, RS-25-5 |
| [`30-error-taxonomy.md`](30-error-taxonomy.md) | designing an adapter's `Error` · a retry loop must tell | RS-30-1, RS-30-2, RS-30-3, RS-30-4, RS-30-5, RS-30-6 |
| [`40-public-surface-and-evolution.md`](40-public-surface-and-evolution.md) | adding a method to a port · an adapter stops compiling after a | RS-40-1, RS-40-2, RS-40-3, RS-40-4, RS-40-5 |
| [`41-declarative-macros.md`](41-declarative-macros.md) | writing a `macro_rules!` other crates will invoke · "cannot find | RS-41-1, RS-41-2, RS-41-3, RS-41-4, RS-41-5 |
| [`50-dependency-hygiene.md`](50-dependency-hygiene.md) | adding a crate to a manifest · `cargo deny check` failed · | RS-50-1, RS-50-2, RS-50-3, RS-50-4, RS-50-5 |
| [`51-features-and-no-std.md`](51-features-and-no-std.md) | adding a `[features]` entry · a feature must not reach an | RS-51-1, RS-51-2, RS-51-3, RS-51-4, RS-51-5 |
| [`52-wasm32-and-target-cfg.md`](52-wasm32-and-target-cfg.md) | `cargo check --target wasm32-unknown-unknown` failed · a test | RS-52-1, RS-52-2, RS-52-3, RS-52-4 |
| [`60-what-a-test-must-prove.md`](60-what-a-test-must-prove.md) | adding a conformance rule · writing a deliberately wrong store · | RS-60-1, RS-60-2, RS-60-3, RS-60-4 |
| [`61-compile-time-assertions.md`](61-compile-time-assertions.md) | asserting a type is `Send` · asserting a type is **not** `Send` · | RS-61-1, RS-61-2, RS-61-3, RS-61-4 |
| [`62-doctests-and-harnesses.md`](62-doctests-and-harnesses.md) | writing a `# Examples` block · an example that uses `?` or | RS-62-1, RS-62-2, RS-62-3, RS-62-4, RS-62-5 |
| [`70-rustdoc-obligations.md`](70-rustdoc-obligations.md) | `error: missing documentation for …` · adding a public item · | RS-70-1, RS-70-2, RS-70-3, RS-70-4, RS-70-5 |
| [`80-the-gate.md`](80-the-gate.md) | adding a check to CI · `cargo xtask ci` is green and CI is not · | RS-80-1, RS-80-2, RS-80-3, RS-80-4, RS-80-5 |
| [`81-checks-that-cannot-be-types.md`](81-checks-that-cannot-be-types.md) | writing a `cargo xtask` lint · a check must read Rust source, a | RS-81-1, RS-81-2, RS-81-3, RS-81-4, RS-81-5 |
| [`90-skeletons-and-todo.md`](90-skeletons-and-todo.md) | starting an adapter crate before its driver exists · deciding | RS-90-1, RS-90-2, RS-90-3, RS-90-4 |
| [`91-adapter-authoring-recipe.md`](91-adapter-authoring-recipe.md) | starting a new event store adapter · asked to "wire up" a | RS-91-1, RS-91-3, RS-91-4 |
| [`92-toolchain-limits-and-dead-ends.md`](92-toolchain-limits-and-dead-ends.md) | `thread 'rustc' panicked` on an adapter impl · a panic naming | RS-92-2, RS-92-3 |
<!-- END GENERATED -->

## The shape of an atom

Each rule is `## RS-<band>-<n>.` followed by five sections in a fixed order:
**Why.** (the mechanism, ≤3 sentences) · **Do** (a compiled example) · **Not**
(the wrong implementation — a `compile_fail` fence, or better, one that
*compiles* and is caught by the assertion closing it) · **Rejects.** (a wrong
state that could ship, and who finds out when) · **Evidence.** (repo
`path:line (anchor)`, then ADR/clause links, then external URLs with a
`*(checked …)*` stamp).

A rule marked `[PROVISIONAL — settles at …]` is not settled; the bracket names
what settles it. The vocabulary matches
[`SPECIFICATION.md` §1.3](../../spec/SPECIFICATION.md) on purpose, so the
two do not fork.

## What the gate checks, and what it does not

`cargo xtask ci` runs two steps over this directory: `lint-constitution` (the
router is a bijection over the atoms, the index agrees, citations resolve to a
line that still contains their anchor, fences are tagged honestly, every rule
names something) and `the constitution's examples compile` (`cargo test -p xtask
--doc`, which builds every fence through
[`xtask/src/constitution.rs`](../../xtask/src/constitution.rs)).

It does **not** judge whether a rule is right. It does not lint fence bodies as
Rust — `cargo clippy` does not lint doctests at all, so `unwrap_used = "deny"`
is unenforced inside every example here and only a narrow grep compensates. And
`cargo fmt --check` does not reach markdown fences. See
[`01-standard-of-evidence.md`](01-standard-of-evidence.md).
