---
item: HS-S0054
stage: discover
created: 2026-08-12T13:02:16.299Z
updated: 2026-08-12T13:02:16.299Z
template_sig: 86ce4036
rendered_sig: a8d50e9e
---

# Discover — Every event-store conformance rule executed on the target, in the same gate run

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one-line: add the crate's conformance target invoking the shipped `event_store_conformance!` with `__emit_wasm`, register it in the gate step from `wasm-execution-seam` so one `cargo xtask ci` executes it under a real Durable Object runtime, and state the concurrency family's non-invocation as a documented reason rather than an absence | `_storymap.md`, **Slices**, `every-rule-under-workerd` row | This is where AC-004 actually closes and where the project's proof artefact first exists |
| AC-002 (every rule runs and passes under `workerd`, no rule `#[cfg]`-ed out, no wasm-only subset anywhere in the tree), AC-003 (declined capabilities emit their reason), AC-004 (inside the gate) | `project.md`, **Acceptance criteria** | Three ACs, and the first two are about what *is not* there as much as what is |
| `dependsOn: wasm-execution-gate-step` — the executing `Step`, the `--target`/runner plumbing and the `--list` name assertion; `dependsOn: durable-object-host-and-fixture` — the fixture and the host to mount it on | `_storymap.md`, **Slices**, `depends_on` column | Both edges are load-bearing: a runner with nothing to run, or a fixture with nowhere to run it, is half a proof |
| ARCH-AC-04: the harness is a test target in this crate invoking the shipped macro with `emit = happenstance_testkit::__emit_wasm`, shaped exactly like the testkit's own wasm harness. No new emitter, no rule list, no `#[cfg]` over any individual rule | `_decomposition.md`, Architecture brief, **Acceptance Criteria** and §4c | Three lines of harness. Anything longer is a signal something is being selected |
| DR-3: *every* rule runs on the target in the same run the rest of the gate uses, and a separately maintained `wasm32` subset is explicitly not an acceptable outcome | `project.md`, **Derived requirements**, DR-3 | The negative half of the requirement is the enforceable half |
| The rule set lives in exactly one place, `for_each_event_store_rule!`, and every harness plus the `no_orphan_rules` meta-test is built by invoking it | `crates/happenstance-testkit/src/lib.rs:84-89`; `crates/happenstance-testkit/src/registry.rs:68` | Structural, not a promise — provided the harness actually goes through it |
| `no_orphan_rules` compares the enumeration against a scan of `suite.rs`'s own source, baked in with `include_str!`, and its stated limits are that it cannot see a rule introduced by a macro expansion, a `pub use`, or a `#[path]` include | `spec/SPECIFICATION.md:7950` and the clause body following it (CF-24, `[FROZEN]`) | It guards the enumeration against the rule set. It does not guard a *harness* against the enumeration |
| The concurrency family binds `F::Store: EventStore + Send`, is `#[cfg(not(target_arch = "wasm32"))]`, and a `!Send` adapter cannot invoke it and is not expected to | `crates/happenstance-testkit/src/lib.rs:100-110`, `:172-173` | AC-002's referent has to be stated precisely, or AC-003 is satisfied by silence — the failure BR-13 exists to prevent |
| `__emit_wasm` wraps each rule in `#[wasm_bindgen_test]` and routes skips through `RuleOutcome::skip_line` because `println!` writes nowhere on this target | `crates/happenstance-testkit/src/registry.rs:55-60`, `:260-291`; `crates/happenstance-testkit/src/contract.rs:500-535` | The reporting path already exists for a target with no stdout. It is why `skip_line` is public |
| `wasm-bindgen-test` must be in **this crate's** wasm-target dev-dependencies, because the attribute resolves in the caller's scope and never in the testkit | `_decomposition.md`, Architecture brief §4c; `crates/happenstance-testkit/Cargo.toml`'s own target-scoped block | A target-scoped dev-dependency, not a feature: a feature is not target-scoped and `--all-features` would set it everywhere |
| `--show-output` is on the gate's `tests` step because a capability-gated rule that a fixture declines still runs, passes, and prints one `SKIP` line that libtest would otherwise suppress | `xtask/src/main.rs:131-142` | The same reasoning has to be ported to the new step, or every stated reason writes into a void |
| CF-23 is `[FROZEN]` and requires three harnesses demonstrated in-tree — tokio, blocking, and `wasm-bindgen-test` — because two would let a one-off accident pass for a design | `spec/SPECIFICATION.md:7920-7951`, ledger row `:8734` | This story is the third harness applied to a real adapter rather than to the testkit's own fixture |
| CF-18's *emission* half is only checkable from outside the process, via `cargo test -- --list`, and "belongs to an `xtask` step, not to a `#[test]`" | `crates/happenstance-testkit/tests/mutation_coverage.rs:3167-3183` | The testkit hands the job to `xtask` in writing. This story is the first caller that needs it |
| The reference shape to copy verbatim | `crates/happenstance-testkit/tests/memory_conformance_wasm.rs:19-27` | Reuse over novelty, and it makes the whole claim reviewable in three lines |

## Questions

**The off-tokio harness shape — ADR-0023's live question, and this story's
central one.** Answered as far as evidence allows and otherwise **deferred to
spec**. `wasm-bindgen-test-runner` under node is the demonstrated mechanism
(`.github/workflows/ci.yml:213-237`) and is what `wasm-execution-gate-step` builds
the seam for. Whether the *Cloudflare* target additionally needs `workerd` or
`vitest-pool-workers` turns entirely on whether the `SqlStorage` binding can be
satisfied by anything short of a real Durable Object — a question answerable only
against the real `worker` API that `worker-binding-layer` lands, which is why it is
deferred rather than guessed. Two outcomes are acceptable and one is not: a
`workerd`-backed runner inside `cargo xtask ci`, or an escalated blocking finding
(`_intake-brief.md`, **Open Questions**; `project.md`, **Risks**) — never a
`cargo check` relabelled as a run.

**Does "every rule" include the concurrency family?** Answered: AC-002's referent
is the event-store family, plus the model family where the fixture supports it. The
concurrency family binds `F::Store: EventStore + Send` and its module is
`#[cfg(not(target_arch = "wasm32"))]`, so a `!Send` adapter cannot invoke it and is
not expected to (`crates/happenstance-testkit/src/lib.rs:100-110`). What is **not**
optional is saying so: the non-invocation is stated with its reason in the crate's
own docs and in ADR-0023. Leaving it silently absent is precisely the failure AC-003
exists to prevent, and this project's risk register calls it out by name.

**Does the model family run here?** Deferred to **spec**. It sits behind the
off-by-default `proptest` feature and `fixtures::strategies` carries the same target
condition for the same reason — a feature is not target-scoped
(`crates/happenstance-testkit/src/lib.rs:176-180`) — so a `wasm32` invocation must
first answer whether `proptest` is in the graph on that target at all.

**Ordering against CF-40.** Named because the dependency edges make it easy to get
backwards: `measured-store-limits` depends on *this* story, so the first green run
happens with the fixture's three ceilings at whatever `durable-object-host-and-fixture`
left them. That run is **not** the discharge of CF-40, and this story must not be
reported as though it were.

**CF-39 / CF-40's fixture-limits ownership.** Not this story's; the atom belongs to
`adr-0023-and-atom-resolutions`, coordinated with `sqlite-durable-store` (HS-P0012).

## Decision

The workspace has paid for the `!Send` flavour in three binding constraints and has
never once run a rule on the target that flavour exists for: the gate `cargo
check`s the crate for `wasm32`, the testkit's wasm harness is type-checked and not
executed, and every body in the adapter is `todo!()` — which type-checks against any
signature. This slice converts the claim into an execution. It adds one test target
to `happenstance-cloudflare` invoking the shipped `event_store_conformance!` macro
with `__emit_wasm` against `CloudflareFixture`, registers that target with the gate
step built in `wasm-execution-seam`, and makes the concurrency family's
non-invocation a stated, reasoned fact rather than an absence a reader has to infer.
Its whole value is that the rule set is *not* restated anywhere: the harness goes
through `for_each_event_store_rule!` verbatim, so "every rule, no wasm-only subset"
is a structural property rather than a promise. The spec will cover: the three-line
target and its `mod_name`; the target-scoped `wasm-bindgen-test` dev-dependency and
why target-scoped and not a feature; the runner decision inherited from
`wasm-execution-gate-step` and its `workerd` extension, or the blocking finding if
it cannot be had; the `--show-output`-equivalent so skip lines reach a human; the
`Artefact` row that asserts the emitted rule names out of `--list`; and the crate
documentation stating the concurrency family's non-invocation with its reason.
Nothing `[FROZEN]` is amended — CF-23 asks for exactly this third harness.

## The wrong implementation

**The subset that is not a subset anywhere you can grep for.** Not a hand-copied
rule list — `no_orphan_rules` makes that hard and AC-002 forbids it outright — but a
**bespoke emitter**. The emitter is a parameter by deliberate design: the testkit is
in no position to know which runtime an adapter is tested on, so it accepts
`emit = <path>` and invokes it as `emitter!(rule_a, rule_b, …)`
(`crates/happenstance-testkit/src/registry.rs:16-26`, `:28-53`). Nothing anywhere
checks that an emitter emits one item per identifier it is handed. A
`macro_rules! __emit_wasm_cloudflare` sitting in this crate's `tests/`, wrapping most
rules in `#[wasm_bindgen_test]` and quietly dropping the three this runtime cannot
pass, satisfies every check in the tree: `no_orphan_rules` compares the enumeration
against a scan of `suite.rs`'s source and never looks at a harness at all
(`spec/SPECIFICATION.md:7950`, CF-24, whose own limits paragraph says it cannot see
a rule introduced by a macro expansion); `capability_skips_are_reported` runs inside the
**testkit's** binary against `DecliningFixture` and never sees an adapter's harness,
which its own documentation states plainly. The gate's output would show a green run
of a suite three rules short, and the rule count is not printed anywhere. A softer
variant of the same mutant needs no new macro: `#[cfg(not(target_arch = "wasm32"))]`
on the whole conformance target, so the file exists, `cargo check --tests --target
wasm32-unknown-unknown` compiles it to nothing, and the run finds zero tests and
exits 0.

**Where the detector must live, and the testkit says so itself.** In
`xtask/src/proof.rs`, as the `Artefact` row this slice's foundation story adds: the
wasm conformance target, with the emitted rule names asserted out of `cargo test
--list` before anything runs (`xtask/src/proof.rs:191-217`). That is the only place
in the workspace that can hold a *harness* to the enumeration, and
`crates/happenstance-testkit/tests/mutation_coverage.rs:3167-3183` hands the job over
in writing: libtest exposes nothing programmatically, a `#[test]` cannot enumerate
the binary it lives in, so the emission half "is only checkable from outside the
process … That belongs to an `xtask` step, not to a `#[test]`." The complementary
obligation inside this crate is negative and cheap: **no emitter is defined in
`crates/happenstance-cloudflare/`**, the target reads
`emit = happenstance_testkit::__emit_wasm` verbatim, and the whole claim is
reviewable in three lines. Nothing new belongs in the mutant registry — the mutant
here is a *harness*, not a store, and no store fails a rule by being under-emitted.

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

**Box 6.** No conformance rule is added here — that is the point of the story. The
existing `for_each_event_store_rule!` enumeration is invoked unchanged, and the only
new assertions are over rule *names* out of `cargo test --list`. Nothing added here
asserts a position value at all, literal or otherwise, so the specification's
permission for gaps stays intact.

**Box 7.** CF-23 is `[FROZEN]` and this story touches it. It is **not changed**:
CF-23 requires three harnesses demonstrated in-tree and forbids the testkit emitting
a runtime attribute from its own expansion, and this story satisfies both by using
the shipped `__emit_wasm` parameter rather than adding anything to the testkit. If
the runtime turned out to be unable to host the family CF-23 assumes, the response
is a new decision atom and a re-plan, written first — never a clause edit and never
a quietly narrowed harness.

**Box 8.** No conformance rule here seems wrong. The one arrangement that looks
like an omission and is not — the concurrency family's `#[cfg(not(target_arch =
"wasm32"))]` and its `Send` bound — is correct as written, and this story's response
is to document the non-invocation with its reason in the same change rather than to
argue with the `cfg`.
