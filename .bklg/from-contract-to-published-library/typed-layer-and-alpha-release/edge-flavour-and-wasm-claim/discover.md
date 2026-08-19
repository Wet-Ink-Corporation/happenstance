---
item: HS-S0031
stage: discover
created: 2026-08-12T13:01:53.914Z
updated: 2026-08-12T13:01:53.914Z
template_sig: 86ce4036
rendered_sig: d5aca3ad
---

# Discover — The typed layer's wasm32 claim, stated either way

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: settle AC-A06 **by name** — either a fifth `wasm32` step compiling `happenstance` added to `xtask/src/main.rs::REQUIRED` and `wasm_steps()`, or a recorded statement that the typed layer makes no `wasm32` claim at the alpha and why — and pin the flavour discipline for this crate's own generic code, with all four existing `wasm32` steps green | `_storymap.md:62` (M7 row) | "Either/or, by name" is the criterion. Silence is the failure it exists to prevent |
| AC-015 — no `#[async_trait]` is introduced, `EventStore::read` still returns the stream at the top level, generic code in `happenstance` binds `EventStore` rather than `SendEventStore`, and **all four `wasm32` steps in `cargo xtask ci` are green** | `project.md:214-217` | Read carefully: AC-015 as written is satisfiable without the typed layer ever being compiled for wasm32 |
| `depends_on: command-loop` and `projection-trait-and-runner` — the two places this project writes new generic code over a store, and therefore the two places a `SendEventStore` bound could leak in | `_storymap.md:62`, `:130-132` | The discipline is checked against real code, not asserted in advance |
| **AC-A06's finding, and it is the story's reason to exist:** the four `wasm32` steps are `happenstance-core`, the conformance harnesses, `happenstance-cloudflare` and `happenstance-neon` — **none of them compiles `happenstance`.** So AC-015's "all four green" can be true while the typed layer has never been built for the edge target it exists to support | `_decomposition.md:399-410` | Either a fifth step is added by name, or the absence of a claim is recorded. *"Silence here is the failure this criterion exists to prevent, not a passing default"* |
| Verified in the tree: `wasm_steps()` selects exactly four steps by name — *"wasm32 build of the contract crate"*, *"wasm32 check of the conformance harnesses"*, *"wasm32 build of the Cloudflare adapter"*, *"wasm32 build of the Neon adapter"* — and `REQUIRED` is the single `&[Step]` every gate step lives in | `xtask/src/main.rs` (`REQUIRED` declaration and `wasm_steps()` body, read directly) | The mount point is unambiguous and there is exactly one of it |
| The design's resolution, binding: **a fifth `wasm32` step is added**, compiling `happenstance` with `--no-default-features --features std,json`. Rejected: recording "no claim" — *"the typed layer is what a Workers application `cargo add`s; a claim nothing compiles is the silence AC-A06 exists to forbid. The step is nearly free"* | `_design.md:648` | The either/or is decided. This story executes the chosen branch and records why the other lost |
| Binding constraint 1 — never `#[async_trait]`; it injects `+ Send`, which makes the wasm32 / Cloudflare Workers target impossible. Ports are defined once without a `Send` bound and `trait_variant` derives the `Send` flavour | `CLAUDE.md`, binding constraint 1; ADR-0001 via `_grounding.md:31-36` | Applies to **any new trait this project adds**, not only to the two frozen ports |
| Binding constraint 4 — bind `EventStore`, not `SendEventStore`, in generic code: it is the weaker requirement and accepts both flavours. Import only one of the two names per module, because having both in scope makes method calls ambiguous | `CLAUDE.md`, binding constraint 4; `_design.md:709-716`; RS-20-2/20-3/20-4 | The blanket impl runs one way only, which is why the bare flavour is the one that accepts both |
| The bound pattern for a runner holding a stream across an await inside a real task: `spawns_from_generic`'s own `S: SendEventStore + Send + Sync + 'static`, written at the definition with per-bound reasoning | `_grounding.md:96-104`; `_decomposition.md:430`, citing `crates/happenstance-core/src/memory.rs:643-680` | Where the `Send` flavour is genuinely required, it is *stated at the definition*, not reached for by default |
| Binding constraint 3 — `EventStore::read` returns the stream at the top level and is not `async`; two tests in `memory.rs` assert it and **it takes both**. `send_flavour_stream_is_send_in_generic_code` (`:614`) discharges the obligation before monomorphisation; `spawns_from_generic` (`:643`) rejects the `async fn read` refactor by holding the stream across an await inside a real `tokio::spawn` | `CLAUDE.md`, binding constraint 3; `_grounding.md:96-104` | This story must not regress either, and must not be tempted to "simplify" them |
| The step is nearly free because `happenstance` adds no dependency `happenstance-core` does not already build on that target, plus `serde`/`serde_json`, both of which do | `_design.md:648` | The cost argument is already made; this story verifies it rather than re-deriving it |
| A new gate step goes in `xtask/src/main.rs::REQUIRED` **and nowhere else** — `cargo xtask ci` is defined once and *is* what CI runs; a step added to `.github/workflows/` instead is invisible locally and drifts | `_decomposition.md:471-475` (Composition roots 5); `CLAUDE.md`, *Commands* | One mount point, and the alternative is named as a failure |
| The unit-level pattern test the testing brief names: a `#[test]` or doctest asserting `EventStore` (not `SendEventStore`) is bound in any new generic function — *"the closest this project's own gate gets to proving AC-015 for its own code"* — with the fifth step, if added, being the AC's real instrument | `_decomposition.md:793` | Two instruments at different altitudes, and the brief says which is real |

## Questions

**Answered here.**

- *Fifth step, or recorded no-claim?* Fifth step. The design settles it and states the
  rejected branch's reason (`_design.md:648`): the typed layer is what a Workers application
  actually `cargo add`s, so a claim nothing compiles is the silence AC-A06 forbids.
- *Which feature set does it compile?* `--no-default-features --features std,json`
  (`_design.md:648`). That is the smallest configuration in which the crate is useful on the
  target, and it also exercises the forwarding discipline `codec-and-feature-forwarding` set
  up.
- *Where does the step go?* `xtask/src/main.rs::REQUIRED`, and its name is added to
  `wasm_steps()` so `cargo xtask wasm` picks it up too. Nowhere else
  (`_decomposition.md:471-475`).
- *Does this story touch the two `memory.rs` tests?* No — it depends on them. They pin
  `read`'s shape and CLAUDE.md says it takes both; neither is deleted or "simplified".

**Deferred to `spec`.**

- *The step's exact name string.* It must read like its four siblings and is matched by name
  in `wasm_steps()`, so the string is load-bearing rather than cosmetic; spec fixes it.
- *Whether the projection runner's `unstable-projection` feature is included in the wasm32
  step's feature set.* The design's step names `std,json` and the runner is off by default;
  whether a second configuration is worth compiling is spec's call, informed by whether the
  runner is expected to work on the edge at all.
- *The exact form of the unit-level bound test.* `_decomposition.md:793` names the pattern
  and `spawns_from_generic` is the model; the expression is spec's.

**Not blocked.** Neither `trybuild` nor `MemoryProjectionStore` is an input to this story.
Note the ordering consequence: this story cannot land before `command-loop` and
`projection-trait-and-runner`, because the fifth step compiles the crate those stories fill.

## Decision

The problem this slice solves is that AC-015 is worded in terms of an existing gate that
cannot see the crate this project is building: all four `wasm32` steps compile
`happenstance-core`, the conformance harnesses, `happenstance-cloudflare` and
`happenstance-neon`, and every one of them can stay green forever while `happenstance` —
the crate a Workers application actually installs — has never once been built for
`wasm32-unknown-unknown`. This story closes that hole in the only two ways it can be closed
honestly, and takes the first: a fifth step compiling `happenstance` with
`--no-default-features --features std,json`, added by name to `REQUIRED` and to
`wasm_steps()`, so the claim the crate makes about the edge is a claim something checks. It
also pins the flavour discipline for this crate's own new generic code, which is where a
`Send` bound would otherwise leak in unnoticed. The spec for this story covers the fifth
step's declaration, its name string and its two registration sites, the audit of every
generic bound `command-loop` and `projection-trait-and-runner` introduced — `EventStore`,
never `SendEventStore`, with the `spawns_from_generic` pattern used where the `Send` flavour
is genuinely required and stated at the definition — the one-flavour-name-per-module import
rule, the absence of `#[async_trait]` anywhere including the testkit, the unit-level bound
test, and the recorded reason the "no claim" branch lost. No `[FROZEN]` clause is amended;
this story's whole subject is obeying binding constraints 1, 3 and 4 rather than changing
anything they govern.

## The wrong implementation

**The mutant: a generic function in `happenstance` bound `S: SendEventStore`.**

```rust
pub async fn commit<S, B, D, F>(store: &S, boundary: B, retry: Retry, decide: F)
    -> Result<Committed, CommandError<S::Error, D>>
where
    S: SendEventStore,          // instead of EventStore
    // …
```

Nothing complains. It compiles. Every test in the workspace passes, because every store any
test uses is `Send`. `cargo xtask ci` is green — including **all four `wasm32` steps**,
because not one of them compiles `happenstance`. `cargo xtask ci --fast`, this project's own
DoD-6 bar, is green. Clippy is silent. AC-015 read literally is satisfied: no
`#[async_trait]` was introduced, `read` still returns its stream at the top level, and the
four wasm32 steps are green. The bound even looks *stronger*, which is how it gets through
review.

It is wrong because `trait_variant`'s blanket impl runs one way only: `SendEventStore` is the
**stronger** requirement, so a `!Send` store — the Cloudflare Durable Object adapter, the
workspace's only such store, and the entire reason ADR-0001 refused `#[async_trait]` — cannot
satisfy it. The typed layer, which is the crate that whole design exists to make usable at
the edge, silently stops accepting the edge store. And nothing in the repository says so:
the failure surfaces when someone tries to write a Workers application against
`happenstance`, which is after the alpha is on the registry.

The instrument that rejects it is the fifth `wasm32` step, and this story is where it lands.
That is the precise sense in which AC-A06 calls silence the failure — the mutant is not
caught by a rule that was skipped, it is caught by no rule at all, because the coverage gap
is in the *selection of what the gate compiles* rather than in what it asserts. A secondary
instrument, cheaper and named in the testing brief (`_decomposition.md:793`), is a bound test
over the new generic functions; it is weaker because it must be remembered for each new
function, whereas the step covers the crate.

**A second mutant: adding the fifth step to `.github/workflows/ci.yml`.** It runs in CI, it
goes green, and the pull request shows a passing wasm32 job. `cargo xtask ci` is defined once
and *is* what CI runs (`CLAUDE.md`, *Commands*), so a step that lives only in the workflow
file is invisible to every local run, drifts from the gate it is supposed to be part of, and
is the first thing deleted when the workflow is next refactored. `REQUIRED` is the one mount
point (`_decomposition.md:471-475`), and `wasm_steps()` — which selects by name — is what
makes `cargo xtask wasm` see it too.

**A third mutant: importing both flavour names into one module.**
`use happenstance_core::{EventStore, SendEventStore};` compiles until the first method call
that is ambiguous between them, and then produces an error whose fix looks like "add a turbofish"
rather than "you imported two traits that both have this method". Binding constraint 4 says
import one per module and reach the other by full path, and RS-20-3 is where the rule lives.
This one does eventually fail to compile — it is named because the cost is an hour of
confusion, and because the habit of importing both is what precedes the first mutant above.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes. **Literal positions:** this story adds no conformance rule; it adds
a gate step and a bound test, neither of which constructs or observes a `SequencePosition`.
Ticked as vacuously true, checked rather than assumed. **Frozen clauses:** none is amended —
the story's entire subject is *obeying* binding constraints 1, 3 and 4 and the ES-\* clauses
behind them. It explicitly does not touch the two tests in `crates/happenstance-core/src/memory.rs`
that pin `read`'s shape; CLAUDE.md says it takes both, and this story depends on them holding
rather than proposing to change them.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
