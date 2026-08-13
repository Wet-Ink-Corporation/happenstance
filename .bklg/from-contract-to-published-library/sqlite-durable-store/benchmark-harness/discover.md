---
item: HS-S0034
stage: discover
created: 2026-08-12T13:01:57.059Z
updated: 2026-08-12T13:01:57.059Z
template_sig: 86ce4036
rendered_sig: 703afacf
---

# Discover — event_store_benchmarks! in the testkit, provably not conformance

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: add `event_store_benchmarks!` to `happenstance-testkit` behind an off-by-default **and** target-gated `bench` feature, with its own enumeration outside `for_each_event_store_rule!` and a caller-supplied emitter, mounted against `MemoryFixture`. | `_storymap.md`, *Slices* table, row `bench-harness-and-adr` / `benchmark-harness` | Four independent constraints, not one: off by default, target-gated, separately enumerated, emitter as parameter. Three of the four are invisible to a native `cargo test`. |
| **AC-012** — benchmarks exist and are provably not conformance: the harness runs behind a `bench` feature and *the conformance rule count is unchanged by its arrival* — "CF-34's own claim, checked rather than asserted." | `project.md`, *Acceptance criteria*, AC-012 | The deliverable is two things: a harness, and a **check** that the harness cost conformance nothing. The second is the part a benchmark PR normally omits. |
| No `dependsOn`. This is the project's first story and nothing precedes it. | `_storymap.md`, *Merge order* item 1 | Nothing about SQLite exists yet, so the harness must be developed and mounted against `MemoryFixture` — the reference `Fixture` (`crates/happenstance-testkit/src/fixtures.rs:243-292`). |
| It is consumed immediately: `adr-0022-append-condition-strategy` "cannot quote a measured figure without it," and again by `SqliteFixture` once that exists. | `_storymap.md`, *Why the three foundations are foundations* | The harness must be able to measure **append under contention** and **replay of N events with and without a tag filter**, because those are the shapes ADR-0022 must rank (`RUNBOOK.md:4207-4213`). A throughput-only harness cannot settle the append-condition question. |
| Architecture brief §10: its own enumeration in its own module; feature-gated *and* target-gated; the emitter stays a parameter (CF-23). | `_decomposition.md`, *Architecture brief*, §10 "The testkit surface this project adds, and its blast radius" | These are the story's real acceptance shape. §10 also states the structural reason: benchmarks must not live in `suite.rs` and must not appear in `for_each_event_store_rule!`. |
| `cargo xtask spec-trace` scans only `RULE_FILES` — `suite.rs`, `model.rs`, `concurrency.rs` — for `pub async fn`. A new `bench.rs` is in none of them. | `xtask/src/spec_trace.rs:85-90` (`RULE_FILES`), `crates/happenstance-testkit/src/concurrency.rs:138-144` | A benchmark written in a new module needs no clause, which is correct — and it also means **nothing in the gate would notice** benchmark functions written in the shape of rules. The guard is the rule count, not spec-trace. |
| `for_each_event_store_rule!` is the single enumeration every harness expands, and `registry.rs` already asserts no rule is orphaned from it. | `crates/happenstance-testkit/src/registry.rs:94`, `:413-423` | AC-012's "rule count unchanged" is measurable exactly here: expand `for_each_event_store_rule!(rule_names)` before and after and diff the list. That is the checkable form of CF-34's claim. |
| CF-34 `[PROVISIONAL]`: performance MUST be measured by a separate harness, and the clause names **no rule** — deliberately, because "a rule enforcing it would violate CF-33." | `spec/SPECIFICATION.md:8263-8265`, `:8745`, `:8774` | CF-34 is unfalsifiable *as a clause*. This story is what makes its content structurally true instead. |
| CF-33 `[FROZEN]`: no conformance rule may read a clock, measure elapsed time, or assert on an operation count. | `spec/SPECIFICATION.md:8236-8245` | A benchmark reads a clock by definition. That is legal only because it is **not a rule** — which is the whole reason the separate enumeration is load-bearing rather than tidy. |
| CF-23 `[FROZEN]`: the testkit MUST NOT emit any runtime-specific attribute from its own crate; the per-test wrapper is a parameter. | `spec/SPECIFICATION.md:7920`, `:5001` | The measurement dependency (whatever times the run) belongs in the *adapter's* dev-dependencies, reached through a caller-supplied emitter — never in `happenstance-testkit`'s `[dependencies]`. |
| The testkit's `[dependencies]` today are exactly `happenstance-core` and `futures-core`; the only feature is `proptest`; there is no `bench` feature and no `bench` module. | `crates/happenstance-testkit/Cargo.toml`; `crates/happenstance-testkit/src/lib.rs:168-185` | Everything this story adds is new surface on a **published** crate at `0.2.0` whose independent version number exists precisely because new surface can turn a passing adapter red (`Cargo.toml`, version comment). |
| **A feature is not target-scoped.** `--all-features` sets `feature = "proptest"` on every target including `wasm32`, which is why the `proptest` modules carry a second `not(target_arch = "wasm32")` condition — "and the wasm32 feature-powerset step of `cargo xtask ci` is exactly what finds it." | `crates/happenstance-testkit/src/fixtures.rs:298-305` | The lesson is already written down in this crate, in this exact shape, about this exact mistake. Repeating it is not a hypothetical. |
| This project's bar is `cargo xtask ci --fast`, which is `REQUIRED` without `OPTIONAL` — it **drops the two feature powersets**, `cargo deny` and the nightly docsrs build, keeping the four mandatory wasm32 steps. | `.redkiln/config.yaml:50-55`; `CLAUDE.md`, *Commands* | The single check that catches an untarget-gated feature is the one this project's own gate does not run. Stated here so it is designed against rather than discovered by `closeout-and-durable-audience`. |
| DR-08: nothing added to the shared testkit costs the `!Send` flavour anything. | `project.md`, *Derived requirements*, DR-08 | This story is where DR-08 is earned; `instrument-markers-removed-and-gate-green` only re-checks it. |

## Questions

Open questions to resolve before specifying.

1. **Does the `bench` module compile for `wasm32`, or does it not exist there?**
   *Answered here, and it is the story's central choice.* Architecture brief §10
   allows either "deliberately." **Decision: it does not exist on `wasm32`** —
   gated the way `concurrency` is, with the `not(target_arch = "wasm32")`
   condition written *in addition to* the feature, per the mechanism at
   `fixtures.rs:298-305`. Rationale: a benchmark needs a clock, and the wasm32
   clock story is the same one CF-33's own `Rule:` line is already known to be
   unable to reach (`spec/SPECIFICATION.md:8989-9000`). Making the module compile
   there would import that unresolved problem into a story that does not own it.
2. **Which measurements does the harness take?** Answered by
   `RUNBOOK.md:4207-4213`: append throughput, conditional append under
   contention, replay of N events with and without a tag filter. The middle one
   is the one ADR-0022 actually consumes, so it is not optional padding.
3. **What shape is the emitter?** Deferred to `spec` — CF-23 fixes that it *is*
   a parameter and fixes nothing about its signature. The constraint the spec
   inherits is that no timing crate may appear in
   `crates/happenstance-testkit/Cargo.toml`'s `[dependencies]`.
4. **Does `event_store_benchmarks!` take a `Fixture` or a `ConcurrentFixture`?**
   Deferred to `spec`. "Conditional append under contention" needs several
   handles onto one store, which is what `ConcurrentFixture`
   (`crates/happenstance-testkit/src/concurrency.rs:186`) already means — but
   adopting that bound imports `Store: Send` into the benchmark surface, which
   DR-08 says must cost the `!Send` flavour nothing. Since the module does not
   exist on `wasm32` (question 1), the bound is affordable; the spec should still
   state it rather than inherit it.
5. **The append-condition SQL strategy itself.** *Not this story's, and
   deliberately not settled here.* This story ships the instrument; ADR-0022
   (`adr-0022-append-condition-strategy`, HS-S0035) reads the number it produces.
   Building the harness in a way that presupposes a winner — for example by only
   being able to time an `EXISTS` probe — would settle ADR-0022 by construction.
6. **How is "the rule count is unchanged" recorded as evidence?** Answered:
   expand `for_each_event_store_rule!(rule_names)` on both sides of this story's
   diff and record the two lists in `_ledger.md`. `registry.rs:89` documents that
   expansion form explicitly. A prose assertion is not evidence for a criterion
   whose whole point is that CF-34's claim gets *checked*.

## Decision

This slice buys the project the one thing a decision record cannot be written
without: a number. ADR-0022 has to rank three append-condition strategies and
three tag-storage layouts against each other, and `RUNBOOK.md:4227-4228` requires
"a benchmark *number*, not a claim" — so the measuring instrument has to exist,
in the testkit, before the adapter it will eventually measure. The spec will
cover: a new `crates/happenstance-testkit/src/bench.rs` carrying
`event_store_benchmarks!` and its **own** enumeration macro; a `bench` feature,
off by default, with the module additionally gated `not(target_arch = "wasm32")`
so that `--all-features` on the wasm target cannot reach it; the three
measurements `RUNBOOK.md:4207-4213` names; a caller-supplied emitter so no timing
dependency enters the testkit's `[dependencies]` (CF-23); a mount against
`MemoryFixture` in `crates/happenstance-testkit/tests/`, because a harness that
has never run is the same failure mode as an adapter that has never run; and the
before/after expansion of `for_each_event_store_rule!` recorded as AC-012's
evidence. No `[FROZEN]` clause is amended: CF-23 and CF-33 are both frozen and
this story is written to **comply** with them — the separate enumeration and the
parameterised emitter are exactly what compliance looks like. CF-34 is
`[PROVISIONAL]` and this story supplies evidence toward it without changing its
text; any marker change is `reopen-negative-control-and-durability-verdicts`' and
`spec-and-code-reconciliation`'s to make.

## The wrong implementation

**The mutant: `bench` as an ordinary Cargo feature, with no target gate.** It is
not a strawman — it is what a careful implementer writes, because "off by
default" *sounds* like the whole obligation and the crate's `proptest` feature
looks, at a glance, like exactly that pattern.

It survives every check this project runs. `cargo test -p happenstance-testkit`
is green. `cargo test --workspace --all-features` on the host is green.
`for_each_event_store_rule!`'s expansion is unchanged, so AC-012's headline
assertion passes. `cargo xtask spec-trace` never looks at `bench.rs` — it reads
`RULE_FILES`, which is `suite.rs`, `model.rs` and `concurrency.rs` and nothing
else (`xtask/src/spec_trace.rs:85-90`). And **`cargo xtask ci --fast`, this
project's entire gate (`.redkiln/config.yaml:50-55`), drops the wasm32
feature-powerset step** — the only step in the repository that sets
`feature = "bench"` on `wasm32` and discovers that a timing API, a `tokio`
handle, or a `std::time::Instant` is not there. The defect lands green, ships in
a **published** crate, and surfaces months later in
`closeout-and-durable-audience`'s full `cargo xtask ci`, or in an adapter
author's CI rather than ours. `crates/happenstance-testkit/src/fixtures.rs:298-305`
records this identical mistake being made and caught once already, for
`proptest`; the second occurrence is the one that has no comment warning about
it yet.

Where the control lives: not in `mutation_coverage/`. A benchmark is not a
conformance rule, so it gets no `REGISTRY` row — `mutant_registry_is_exhaustive`
rejects a row whose `fails` list is empty
(`crates/happenstance-testkit/tests/mutation_coverage/racers.rs:10-18`), and this
mutant fails no rule by construction. The control is **structural and must be
added by this story**: the `not(target_arch = "wasm32")` condition written beside
the feature in `crates/happenstance-testkit/src/lib.rs`, plus a `cargo xtask
wasm`-reachable check that compiles the testkit for `wasm32` with
`--features bench`. Without that second half the story ships a rule about itself
that nothing enforces.

**The second mutant, and the reason the enumeration is separate:**
`event_store_benchmarks!` implemented by adding timing rules to
`crates/happenstance-testkit/src/suite.rs` and naming them in
`for_each_event_store_rule!`. Every adapter then inherits a wall-clock assertion
as part of conformance — which is CF-33 violated in terms
(`spec/SPECIFICATION.md:8236-8245`) and CF-34 inverted — and it is *green on the
author's machine*, failing first on a loaded CI runner, on somebody else's
adapter. AC-012's rule-count diff is precisely the check that rejects it, which
is why that diff is evidence rather than ceremony.

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
