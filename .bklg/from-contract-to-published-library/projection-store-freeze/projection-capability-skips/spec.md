---
item: HS-S0008
stage: spec
created: 2026-08-12T13:46:02.497Z
updated: 2026-08-12T13:46:02.497Z
template_sig: 87bbf1d0
rendered_sig: 9cc2935e
---

# Spec — A declined capability is a reported skip, never a silent absence

## Scope lock

| Artefact | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — **AC-05** (*"the adapter author is told, with a reason, where a guarantee does not apply to them… it never vanishes from the binary"*), **AC-04**, **BR-13**, **BR-12**, and **DoD 7** (*"the projection suite discriminates"*) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the scope seams and the traceability matrix that put BR-13 / AC-05 on this project |
| Project charter | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` — **AC-005** (this story's trace), **DR-03**, **DR-07**, and the *Risks* line forbidding a second declension policy |
| **This spec** | `.bklg/from-contract-to-published-library/projection-store-freeze/projection-capability-skips/spec.md` |
| **Signed-off design (BINDING)** | `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` — its `surfaces: []` no-screen determination and its two named non-visual surfaces (a **type surface** and a **text surface**, `_design.md:24-38`) are binding. The DT-3 resolution and the projection capability set this story consumes are written into that file by `projection-api-design-record` (HS-S0003), which is this story's `blocked_by` |
| Key briefs | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` — **UX brief** AC-U08 – AC-U12 (the whole of the text surface) and its *What would make this brief wrong*; **Architecture brief** AC-A06 (reuse `Capability` / `RuleOutcome` unchanged), Note 1 (the seam map), Note 4 (the emitter/`READS_THROUGH_BATCH` consequences), Note 5 (the fixture contract); **Testing brief** row AC-005 |
| Grounding | `.bklg/from-contract-to-published-library/projection-store-freeze/_grounding.md` |
| Story map | `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` — slice `projection-conformance-suite`, merge position 3 of 8; the Coverage table splits AC-005 between this story (machinery + `RuleOutcome` assertion) and `read-through-and-rebuild-rules` (the real `READS_THROUGH_BATCH = false` instance) |
| Roadmap pointer | `RUNBOOK.md:3848-3965` (phase 6 in full); `RUNBOOK.md:3882-3892` (why the read model cannot be observed without the probe) |
| Specification (wins on conflict) | `spec/SPECIFICATION.md:7597-7617` — **CF-18 `[FROZEN]`**, the clause this story is the projection-side instance of; `:5052-5074` (PS-12's gate is a `ProjectionProbe` const, and a `false` adapter still emits a reported skip); `:5200-5217` (PS-18, the refusable reset) |

## One-line PR slice

A rule whose projection capability the fixture declines is still emitted as a test, returns
`RuleOutcome::Skipped` carrying the fixture's stated reason, and is distinguishable from a pass in the
harness output — asserted on `RuleOutcome` values rather than on stdout, with the projection capability
set fixed by DT-3's recorded resolution rather than invented here.

## Executive summary

This PR lands the **projection side of CF-18**: the declension machinery on the `ProjectionFixture`
trait `projection-suite-entry-point` (HS-S0007) has just created, plus the meta-test that makes the
promise machine-checked rather than printed.

Pointer and delta. The event-store side of this already works and is the thing being instanced, not
re-invented: `Capability` is an opaque struct whose private field forces a reason through a `const fn`
`assert!` (`crates/happenstance-testkit/src/contract.rs:355-433`); `RuleOutcome` is a two-variant,
`#[must_use]` enum with no `Failed` arm because a failing rule panics (`:458-537`); `require!` returns
`Skipped` from inside the rule body precisely so the emitters keep **no branch at all**
(`crates/happenstance-testkit/src/suite.rs:37-46`); `must!` is its sibling for a capability that is not
a trade (`:69-84`); and `capability_skips_are_reported` drives the whole enumeration against a fixture
that declines everything and asserts on `Verdict` **values**, because libtest exposes nothing
programmatically (`crates/happenstance-testkit/tests/mutation_coverage.rs:3184-3317`). None of that is
edited. What does not exist yet is any of it on the *projection* side: no projection capability
constants, no projection `require!` sites, no `DecliningProjectionFixture`, and no projection sibling
of the meta-test.

The delta this PR is: (1) the capability constants land on `ProjectionFixture` **transcribed** from
`_design.md`'s recorded set, with reason-authorship per constant as DT-3 resolved it; (2) the two gate
macros reach projection rules unchanged, `require!` for a trade and `must!` for a MUST; (3) a
`DecliningProjectionFixture` instrument lands in the testkit's own `tests/`, panicking on the second
`connect` exactly as `DecliningFixture` does (`tests/mutation_coverage/variants.rs:502-565`), so a rule
that ignores its gate converts a silent pass into a loud `Panicked`; and (4) the projection sibling of
`capability_skips_are_reported`, with its own `MUST_REJECT` / `MUST_SKIP` slices, asserts every
registered projection rule answers, that a declined one carries the *fixture's own words*, and that a
MUST is rejected rather than skipped.

What this PR is **not** is a rule story. It writes no `ProjectionStore` conformance rule: all seventeen
adapter-level rules §4.11 names are already claimed by other stories in this project's map, and taking
one here would be scope theft from `reset-rules` or `read-through-and-rebuild-rules`. That constraint
has a sharp consequence this spec refuses to hide — see *The non-vacuity problem* below — and it is the
single thing an implementer must read before writing a line.

## Context pack

**Read this section and you can start.** Everything below it is a boundary, a contract or an anchor.

### The decision this story is the mechanism for

**A guarantee that does not apply must be reported, not absent.** That is initiative BR-13 and AC-05,
and it is `[FROZEN]` as CF-18: *"a rule whose capability requirement is unmet MUST still be emitted as
a test that **reports** the skip with the fixture's stated reason. A rule MUST NOT be silently
omitted"* (`spec/SPECIFICATION.md:7597-7602`). The wrong implementation the clause names is not
hypothetical and is worth carrying in the head while working: `#[cfg]`-ing a capability-gated rule out
of the expansion, so *"an adapter author who declares `REOPEN: false` to make a red build green gets a
green build and no record of the trade"* (`:7606-7612`).

Four consequences follow, and each is already settled somewhere — none is this story's to re-decide:

1. **The branch lives in the rule body, never in the emitter.** A `macro_rules!` emitter matches tokens
   and cannot read a `const`'s value, so the only branch it could take is `#[cfg]` — the exact
   arrangement CF-18 rejects. `require!` therefore returns `RuleOutcome::Skipped` from inside the rule,
   the emitter just calls `.report(…)` on what comes back, and after monomorphisation the dead half is
   discarded: *"the test still exists; it just does nothing but say so"*
   (`crates/happenstance-testkit/src/suite.rs:15-46`). Projection rules use the same two macros
   unchanged.
2. **One skip vocabulary, one line shape.** `Capability` and `RuleOutcome` are reused **unchanged** —
   Architecture brief AC-A06 decided it, UX brief AC-U08 gives the reason: *"an author reading one CI
   log must not have to learn two shapes"*. There is no projection-local skip type, no second format,
   and no widening of either type. The line is
   ``SKIP {rule}: fixture declines `{capability}` — {reason}`` (`contract.rs:496-509`).
3. **A MUST is not skippable, and the two mechanisms are deliberately different.** CF-18 itself says
   reading it as licence to skip a MUST is the misreading CF-16 now names in terms
   (`spec/SPECIFICATION.md:7614-7617`). `must!` panics *carrying the fixture's stated reason*, so
   nothing is lost by failing rather than skipping — *"the trade is still in the log, it is simply in
   the log of a build that did not pass"* (`suite.rs:49-84`).
4. **The machine-checked half is a value assertion, never stdout.** `RuleOutcome::report` writes to
   stdout, which libtest suppresses for a *passing* test unless `--show-output` is passed — which is
   why the gate passes it — and the function's own doc is blunt that this *"makes the line reachable by
   a human; it does not make anyone read it"* (`contract.rs:511-531`). Project AC-005 therefore names
   the instrument explicitly: assert on `RuleOutcome` values. UX brief AC-U09 states why that is the
   load-bearing half.

### What this story consumes rather than decides

`_design.md` is BINDING and HS-S0003 is this story's `blocked_by` precisely so that two things arrive
already settled (its own spec lists both under *"Genuinely this story's to decide → consumed by
HS-S0008"*):

- **Which projection capability constants exist**, and whether any is a MUST in `SECOND_HANDLE`'s
  sense. Architecture brief Note 5 hands this to `_design.md` by name and gives the floor: the port
  needs at least a way to say *"this store protects this id from reset"* for
  `refused_reset_changes_nothing` (`spec/SPECIFICATION.md:5200-5217`), and PS-12's gate is a
  `ProjectionProbe` **const** (`READS_THROUGH_BATCH`) rather than a fixture const
  (`:5052-5074`).
- **Who writes the reason for each constant** — DT-3 applied. Three reason-writers already exist and
  the resolution disposes of all three: the fixture-written `Capability::declined(reason)`
  (`contract.rs:355-433`), the testkit-written `NO_CEILING_REASON` for a fact no adapter should
  paraphrase (`contract.rs:435-456`), and per-adapter prose. If the record assigns a projection
  constant a testkit-written reason, this story spells it as a testkit `const` beside
  `NO_CEILING_REASON` and does **not** ask each fixture to paraphrase it.

If `_design.md` does not in fact answer one of these when this story opens, that is a **halt and
report** at the story boundary, not a decision made here by whoever needed it next. Inventing the set
inside a rule-plumbing story is exactly how a second, divergent declension policy gets minted — the
thing `project.md`'s risk list forbids in the words *"one policy, or a stated reason for two"*.

### The non-vacuity problem, stated rather than discovered

At this story's merge point the projection enumeration contains **two rules**, both landed by HS-S0007
and neither capability-gated: `commit_advances_the_checkpoint` and
`commit_is_atomic_with_the_read_model`. Every rule that will carry a `require!` gate belongs to a later
story — `refused_reset_changes_nothing` to `reset-rules`, `batch_reads_reflect_pending_writes` to
`read-through-and-rebuild-rules` — and §4.11's seventeen adapter rules are all claimed, so this story
may not take one.

So the skip half of the machinery has, at merge, potentially nothing to fire on. That is precisely the
failure CLAUDE.md's corollary names one level up (*"a rule that no adapter can fail is decorative"*),
and the UX brief already anticipates it: a projection fixture with **no declinable capability at all**
makes AC-U08 – AC-U12 decorative, and *"that outcome is not a licence to quietly drop the reporting
discipline: it is a finding"*. The resolution, in order:

1. **Preferred — gate a rule that already exists.** If `_design.md`'s capability set contains an
   honestly-declinable constant that one of the two baseline rules genuinely needs, gate it with
   `require!` here and the skip is demonstrated on real `RuleOutcome::Skipped` values in this PR.
2. **The MUST arm is demonstrable today regardless.** If the recorded set contains a MUST — a
   second-handle analogue is the live candidate, since `commit_is_atomic_with_the_read_model` reads
   back through a *fresh* handle (Architecture brief Note 6) — then
   `DecliningProjectionFixture` declining it makes that rule panic **with the fixture's own reason**,
   which is a real, non-vacuous rejection landing in this PR and is the projection instance of
   `MUST_REJECT` (`mutation_coverage.rs:3139-3156`, `:3271-3305`).
3. **Fallback — guard-shaped, and said out loud.** If neither is available, the skip half ships as a
   two-direction consistency guard (every projection rule spelling `require!` appears in the projection
   `MUST_SKIP`, and nothing in `MUST_SKIP` passes or panics) and the spec **says** it is a guard on
   future registrations rather than a demonstrated skip. The repository has exactly this precedent and
   states it in the same terms — *"this loop is a guard on future registrations and nothing more, and
   it is worth saying so rather than letting it read as a demonstrated rejection"*
   (`mutation_coverage.rs:3187-3205`). The forcing function is named and in-project:
   `read-through-and-rebuild-rules` (HS-S0013) lands the `READS_THROUGH_BATCH = false` instance and its
   `depends_on` already points here.

What is **not** allowed is inventing a capability nothing needs, or writing a private rule the suite
does not run, in order to make the meta-test look demonstrated. Either buys a green assertion about
code no adapter executes.

### The constrained target is where this promise breaks silently

On `wasm32-unknown-unknown`, `RuleOutcome::report` is a **no-op** — measured under
`wasm-bindgen-test-runner` with `--nocapture`, not assumed: that target's `std` has no host stdio and
`wasm-bindgen-test` hooks `console.*` rather than `println!` (`contract.rs:511-531`). The wasm emitter
therefore calls `skip_line` and hands the string to `console_log!`
(`crates/happenstance-testkit/src/registry.rs:276-291`). The projection wasm harness must route the
same way — UX brief AC-U11 — or AC-016's run is the one where the stated reason silently disappears,
and **P3, the local-first / edge developer, is the persona who pays** (BR-12).

### The persona slice this realizes

Two readers, from the UX brief's own table (`_decomposition.md:215-219`), and only these:

- **P2, the adapter author** (`_discovery/distillation/personas-and-journeys.md:114-153`) wants *"an
  executable definition of 'correct' they can run against their own storage system"* and fears the port
  quietly assumed something their storage cannot provide. This story is the half of that promise that
  covers what happens when their storage genuinely cannot: they get a line naming **the constant they
  can change** — UX brief AC-U10, whose precedent is `NO_STORE_LIMITS` naming all three constants
  because *"a skip naming only one of them would send an adapter author to look for the constant they
  did set"* (`contract.rs:435-442`).
- **Whoever reads the run** sees only the text surface. For them the whole deliverable is one line, and
  the guarantee is that the line exists and is machine-checked to exist.

Both persona citations are **secondary evidence**; no persona in this initiative has been directly
observed (`…/personas-and-journeys.md:361-363`), and promotion into `.kb/product/` is HS-P0019's. Carry
the qualification.

### The traps this story is one step away from

- **A reasonless declension is caught later than one would like.** `Capability::declined("")` is a
  `const fn` `assert!`, but on an **associated** const it is evaluated lazily and fails at *codegen* —
  `cargo build` and `cargo test` catch it, `cargo check` and `cargo clippy` do not
  (`contract.rs:400-419`; RS-91-3 at `standards/rust/91-adapter-authoring-recipe.md:156-158`). Every
  projection fixture constant inherits that, so the trait's rustdoc must say where it fires.
- **`compile_fail` is spelled bare, never `compile_fail,E0080`.** rustdoc on 1.97.1 silently ignores an
  error code it cannot match, so the stricter-looking spelling is the *weaker* check
  (`contract.rs:400-403`).
- **No borrowing GAT anywhere on the fixture.** `type Store<'a> where Self: 'a` on a foreign trait is
  one of five ingredients of a rustc ICE this repository already minimised and which still reproduces
  on 1.97.1 (`contract.rs:97-111`; `experiments/rustc-ice-gat-foreign-trait/`). Hand back an owned
  handle holding a refcount, as `MemoryFixture` does (`crates/happenstance-testkit/src/fixtures.rs`).
- **A skip must not be reachable by deleting a gate.** `DecliningFixture::connect` panics on the second
  call on purpose, so a rule that ignored its gate becomes a loud `Panicked` instead of a quiet pass
  (`tests/mutation_coverage/variants.rs:490-501`). The projection instrument copies that shape.
- **`.kb/` is not written by hand.** This story mints no decision atom and resolves no open question;
  atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/` (`CLAUDE.md`, *Where the work lives*).
  Anything this work turns up that deserves an ADR is recorded as a named gap for the runbook's ADR
  pass.

## Integration contract

- **Archetype**: `capability` — a user-observable slice through every layer. The user is the adapter
  author and the observable is a line in their CI log plus the `RuleOutcome` value behind it; a library
  has no other medium (`_design.md:24-38`, the text surface).
- **Slice / milestone**: `projection-conformance-suite`. Slice-mate, implemented in the same context and
  mounted as one integrated surface: **`projection-suite-entry-point` (HS-S0007)**. This story is second
  within the slice — the entry point creates `ProjectionFixture`, the enumeration and the three
  harnesses; this story gives that fixture its capability block and the reporting obligation that goes
  with it.
- **Mount point**: **`crates/happenstance-testkit/src/contract.rs`** — the `ProjectionFixture` trait's
  associated-`const` capability block. A library has no render tree, so per Architecture brief Note 1 a
  thing is mounted when it is reachable at **both** of two places, and both are named here: the trait
  itself in `contract.rs`, and its two exports in `crates/happenstance-testkit/src/lib.rs` — the public
  re-export beside `Capability`, `Fixture`, `NO_CEILING_REASON`, `NO_STORE_LIMITS`, `RuleOutcome`
  (`lib.rs:187`) and the `__private` module the macro expansion names in the adapter's own crate
  (`lib.rs:362-364`). A capability constant that exists on the trait but is not reachable through the
  macro expansion is a constant no adapter can set.
- **Wires into** (real siblings this consumes, by path — none of them forked):
  - `crates/happenstance-testkit/src/contract.rs:355-433` — `Capability`, and `declined`'s const-eval
    reason check.
  - `crates/happenstance-testkit/src/contract.rs:435-456` — `NO_STORE_LIMITS` / `NO_CEILING_REASON`, the
    testkit-written-reason precedent DT-3's resolution disposes of.
  - `crates/happenstance-testkit/src/contract.rs:458-537` — `RuleOutcome`, `skip_line`, `report` and its
    two honest per-target limitations.
  - `crates/happenstance-testkit/src/suite.rs:37-46` and `:69-84` — `require!` and `must!`, used
    unchanged from the projection rules module.
  - `crates/happenstance-testkit/src/registry.rs:222-295` — the three emitters, and `__emit_wasm`'s
    `console_log!` route at `:276-291`.
  - `crates/happenstance-testkit/tests/mutation_coverage.rs:3139-3317` — `MUST_REJECT`, `MUST_SKIP` and
    `capability_skips_are_reported`, the shape the projection sibling copies.
  - `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:490-565` — `DecliningFixture`, the
    instrument the projection one mirrors, including the panic-on-second-`connect`.
  - `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:96,490-505` — `Verdict`,
    `verdict()`, `skipped()`, the value-assertion vocabulary.
  - `crates/happenstance-core/src/projection.rs` (as amended by `owned-batch-port-shape`) and
    `ProjectionProbe`'s `READS_THROUGH_BATCH` (`spec/SPECIFICATION.md:4998-5031`) — read, not changed;
    PS-12's gate is a probe const and this story states that rather than duplicating it as a fixture
    const.
  - `.bklg/…/projection-store-freeze/_design.md` — the capability set and DT-3's resolution,
    transcribed.
- **Renders surfaces**: **none** in the `_design.md` `## Items` sense — that project records
  `surfaces: []` and no screen (`_design.md:48-54`). What this story changes is the **text surface**
  `_design.md:33-38` already names: the one line a run prints for a declined capability. It adds no new
  line shape; it makes the existing one reachable from the projection family.
- **Public items** (the `## Items` ids HS-S0003 records, that this story implements or changes):
  `happenstance_testkit::ProjectionFixture`'s capability constants and their reason consts; and the
  reused, unmodified `happenstance_testkit::Capability` / `happenstance_testkit::RuleOutcome`. No new
  public type is introduced; if one appears, AC-A06 has been violated.
- **Conformance rule(s)**: this story adds **no** `ProjectionStore` conformance rule — all seventeen
  §4.11 adapter rules are claimed by sibling stories in `_storymap.md`. What it adds is the **gate** on
  whichever of them the recorded capability set applies to, plus a **meta-test**: the projection sibling
  of `mutation_coverage::capability_skips_are_reported`. That meta-test is CF-18's own named rule
  (`spec/SPECIFICATION.md:7602-7605`; clause table row at `:8729`), so this story is not a change to a
  port with no rule behind it — it is the projection instance of a rule that already exists.
- **Clause(s)**: instances **CF-18 `[FROZEN]`** on the projection family; observes but does not amend
  **PS-12** (`READS_THROUGH_BATCH`'s `false` arm is a reported skip, `spec/SPECIFICATION.md:5052-5074`)
  and **PS-18** (the refusable reset the fixture-side capability exists for, `:5200-5217`). Nothing
  `[FROZEN]` is line-edited; no maturity marker is moved — that is
  `unstable-projection-gate-and-clause-disposition`'s.
- **Advances DoD scenario**: initiative **DoD 7** — *"The projection suite discriminates."* Moved
  toward green, not reached: discrimination needs the mutants (`projection-mutant-registry`) and the
  rules; this story is the half that stops discrimination being achievable by *omission*, which is the
  cheapest way to fake it. It also carries initiative **AC-05** in full for the projection family, and
  is a precondition for **DoD 4**'s wasm reading of it via the `skip_line` → `console_log!` route.

## PR boundary

The narrowest set that is honestly true. `redkiln verify --grain story` reads the first fenced block
below and fails on any file changed outside it. `crates/happenstance-testkit/src/**` is deliberately a
directory glob rather than a file list: `projection-suite-entry-point` settles Note 4's emitter question
and therefore the projection rules module's filename in the same slice, and a boundary naming a filename
this story did not choose would fail for a reason that has nothing to do with scope.

```
crates/happenstance-testkit/src/**
crates/happenstance-testkit/tests/**
CHANGELOG.md
.bklg/from-contract-to-published-library/projection-store-freeze/projection-capability-skips/**
```

**In this PR**

- The projection capability constants on `ProjectionFixture`, transcribed from `_design.md`, each with
  a reason authored by whoever DT-3's resolution says authors it, and rustdoc stating where an empty
  reason fires (codegen, not `check`).
- `require!` / `must!` gates applied to whichever projection rules the recorded set applies to, reusing
  both macros unchanged.
- `DecliningProjectionFixture` in `crates/happenstance-testkit/tests/`, declining everything declinable
  and panicking on a second `connect`, mirroring `DecliningFixture`.
- The projection sibling of `capability_skips_are_reported`, with its own `MUST_REJECT` / `MUST_SKIP`
  slices, asserting on `RuleOutcome` / `Verdict` values.
- The wasm route check: the projection wasm harness reports skips through `skip_line` →
  `console_log!`, never `report`.
- The mount: the public and `__private` exports in `lib.rs`, and the testkit crate doc's mention of the
  projection family's declension vocabulary where the event-store one is already described.
- A `CHANGELOG.md` entry naming the defect this machinery detects — a capability-gated projection rule
  vanishing from the binary (CF-29's discipline, `spec/SPECIFICATION.md:8141-8165`).
- This story's own `_ledger.md` and stage artifacts.

The implementer **may** also touch the composition-root / wiring files named in the Integration
contract — `lib.rs`'s export block and `__private`, and the harness files — to mount this slice. That is
not scope drift; an unmounted capability constant is one no adapter can set.

**Explicitly not in this PR**

- **Any `ProjectionStore` conformance rule.** All seventeen are claimed elsewhere in `_storymap.md`.
- **Any change to `Capability` or `RuleOutcome`**, including a widening, a new variant or a second line
  shape (Architecture brief AC-A06; UX brief AC-U08).
- **Any change to the event-store family** — `Fixture`'s capability set, the existing `MUST_SKIP` /
  `MUST_REJECT` slices, or `capability_skips_are_reported` itself. Those are an existing contract with
  an existing user; a change there is a change to the event-store suite and must be justified as such.
- **`crates/happenstance-core/**`** — no port edit, no new feature, no probe change. `ProjectionProbe`
  and `READS_THROUGH_BATCH` are read and cited, not modified.
- **`.kb/**`** — no atom hand-written, no open question's status changed
  (`CLAUDE.md`, *Where the work lives*).
- **`spec/SPECIFICATION.md`** — no clause edited, no maturity marker moved.
- **The mutant registry** (`REGISTRY: &[Declared]`, the three exactness meta-tests) — that is
  `projection-mutant-registry`. This story's instrument is a *fixture that declines*, not a wrong store.
- **DT-3's or DT-8's resolution** — recorded by HS-S0003; consumed here, never re-decided.

**Merge DoD (one line):** against a `DecliningProjectionFixture`, every registered projection rule
produces an outcome, a declined one carries the fixture's own stated reason and names the constant that
set it, a MUST is rejected rather than skipped, none of it is asserted through stdout — and
`cargo xtask ci` is green, including the mandatory wasm32 conformance-harness check.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The capability set is transcribed, not invented** | Each constant on `ProjectionFixture` traces to `_design.md`'s recorded set; each carries a reason whose author is the one DT-3 named (fixture / testkit / adapter prose). A constant with no counterpart in the record is a halt-and-report at the story boundary, not a judgement call. | `.bklg/…/projection-store-freeze/_design.md`; `_decomposition.md` Architecture brief Note 5; `project.md` *Risks* ("one policy, or a stated reason for two") |
| **No rule vanishes** | Against a fixture that declines everything, the count of outcomes equals the count of registered projection rules, and every registered name is present. `#[cfg]`-ing one out is the wrong implementation CF-18 names. | `spec/SPECIFICATION.md:7597-7612`; `crates/happenstance-testkit/tests/mutation_coverage.rs:3218-3236` (the event-store shape) |
| **A skip carries the fixture's own words** | `RuleOutcome::Skipped { capability, reason }` where `reason` is the string that fixture's own `Capability::declined` was given — compared against the fixture's `const`, never against a literal repeated in the test, since two copies would let the report carry someone else's words. | `crates/happenstance-testkit/src/contract.rs:466-483`; `tests/mutation_coverage/variants.rs:507-529` |
| **…and names the constant the author actually set** | `capability` is `stringify!` of the associated const's identifier, so the reader is sent to something they can change. Where a skip is gated by more than one switch, all of them are named — the `NO_STORE_LIMITS` precedent. | `crates/happenstance-testkit/src/suite.rs:37-46`; `contract.rs:435-442`; UX brief AC-U10 |
| **A skip is machine-distinguishable from a pass** | The assertion is on `RuleOutcome` / `Verdict` values, never on captured stdout: libtest exposes nothing programmatically and suppresses a passing test's output without `--show-output`. `#[must_use]` on `RuleOutcome` is what turns "an emitter must report" into a build failure. | `contract.rs:462-472`, `:511-531`; `project.md` AC-005; UX brief AC-U09 |
| **One vocabulary, one line shape** | `Capability` and `RuleOutcome` are reused unchanged; no projection-local skip type, no second format. A fork would compile, so the design review plus this contract is what rejects it. | Architecture brief AC-A06; UX brief AC-U08; `contract.rs:355-537` |
| **A MUST is rejected, not skipped** | A projection capability recorded as a MUST uses `must!`, which panics carrying the fixture's stated reason; the projection `MUST_REJECT` slice asserts each such rule *panics* and does **not** appear in `skipped()`, so a downgrade from `must!` back to `require!` fails the build. | `crates/happenstance-testkit/src/suite.rs:49-84`; `mutation_coverage.rs:3271-3305`; `spec/SPECIFICATION.md:7614-7617` |
| **The two slices stay two-direction consistent** | A projection rule that gains a gate and is not listed fails with a message pointing at the rule; a listed rule that stops skipping fails too. This is what keeps the lists from rotting into decoration as later stories add rules. | `mutation_coverage.rs:3122-3138` (why `MUST_REJECT` is a slice, not a singleton), `:3287-3305` |
| **A fully capable fixture skips nothing** | The mirror assertion: run the enumeration against a fixture supporting everything and assert `skipped()` is empty — a skip there means a gate reads the wrong const. | `mutation_coverage.rs:3307-3316` |
| **The gate cannot be deleted quietly** | `DecliningProjectionFixture::connect` panics on the second call, converting "a rule ignored its gate and passed" into a `Verdict::Panicked` the meta-test rejects. | `tests/mutation_coverage/variants.rs:490-501`, `:539-557` |
| **The reason survives `wasm32`** | The projection wasm harness routes `skip_line`'s `String` to `console_log!` as `__emit_wasm` already does; `report` is a measured no-op there. Type-checked by the mandatory *"wasm32 check of the conformance harnesses"* step. | `contract.rs:511-531`; `registry.rs:276-291`; `xtask/src/main.rs:231-240`; UX brief AC-U11 |
| **A reasonless declension fails the build, and the trait says where** | `Capability::declined("")` on an **associated** const fails at codegen — caught by `cargo build` / `cargo test`, missed by `check` and `clippy`. `ProjectionFixture`'s rustdoc states it; any demonstrating doctest is spelled bare `compile_fail`, never `compile_fail,E0080`. | `contract.rs:400-419`; RS-91-3 at `standards/rust/91-adapter-authoring-recipe.md:156-158`; RS-40-5 at `standards/rust/40-public-surface-and-evolution.md:212` |
| **Non-vacuity is claimed only as far as it is true** | If no gated projection rule exists at merge, the meta-test is documented as a guard on future registrations rather than a demonstrated skip, with `read-through-and-rebuild-rules` (HS-S0013) named as the in-project forcing function. Overstating it is the failure; shipping it silent is the other one. | `mutation_coverage.rs:3187-3205` (the same admission, made in the same words); UX brief *What would make this brief wrong*; `CLAUDE.md` *The rule that matters* |
| **No borrowing GAT on the projection fixture** | The instrument hands back an owned handle holding a refcount. `type Store<'a> where Self: 'a` is one of five ingredients of a rustc ICE that still reproduces on 1.97.1. | `contract.rs:97-111`; `experiments/rustc-ice-gat-foreign-trait/`; `crates/happenstance-testkit/src/fixtures.rs` |
| **A changelog entry names the defect** | `CHANGELOG.md` gains an entry naming what this machinery detects — a capability-gated projection rule silently absent from the binary — under CF-29's discipline, whose lint is explicitly *"the weakest check in the gate"* and does not excuse a thin sentence. | `spec/SPECIFICATION.md:8141-8165` |

## Data and migrations

**N/A — no persisted state, no schema, no serialisation format.** This story adds associated `const`s
and `&'static str` reasons to a trait, two macro call sites, one test-only fixture and one meta-test.
Nothing it touches is written to a medium, sent over a wire, or read back after a process restart:
`Capability` is an `Option<&'static str>` newtype and `RuleOutcome` is a two-variant `Copy` enum
(`crates/happenstance-testkit/src/contract.rs:355-483`), and both live entirely in the test process.

Two adjacent things are deliberately not migrations and are noted so nobody looks for one:

- **The reason strings are not data.** They are compile-time constants read by an assertion in the same
  binary, and the whole point of the shape is that a fixture's own words reach the report without a
  copy in between (`tests/mutation_coverage/variants.rs:507-514`). Nothing serialises them.
- **`happenstance-core` gains no dependency and no feature here.** `ProjectionProbe`'s `conformance`
  feature and the widened feature powerset are `projection-probe-conformance-feature`'s, already merged
  before this slice; `serde` remains outside `happenstance-core`'s default features by binding
  constraint (ADR-0003) and nothing here changes that.

## Acceptance criteria

Eight criteria. Every one is framed from the intent of a reader who crosses the whole stack — the
adapter author who has to declare a limit (P2), the person reading the run (the UX brief's third
reader), or the edge developer on the constrained target (P3) — never as a bare capability. Together
they carry project **AC-005** in full for the machinery half; the `READS_THROUGH_BATCH = false`
instance is `read-through-and-rebuild-rules`'s and is deliberately not claimed here
(`_storymap.md:83`).

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | GIVEN P2, an adapter author whose projection store genuinely cannot provide a guarantee the port assumes, WHEN they open `ProjectionFixture` looking for the constant that lets them say so, THEN they find exactly the capability constants `_design.md` recorded under DT-3 — each carrying a reason written by the author DT-3 named, fixture prose through `Capability::declined` or a testkit-written const beside `NO_CEILING_REASON` — and no constant that record does not contain; if the record is silent on the set, the story halts and reports rather than minting one here. | **Static** — reviewed diff of the constant block against `.bklg/…/projection-store-freeze/_design.md`; the Testing brief's AC-006 row already states a recorded resolution is checked by the design-stage review, not by a tier. **Unit** — `projection_capability_reasons_are_authored_once` in `crates/happenstance-testkit/tests/mutation_coverage.rs`: every constant's reason is non-empty, and a testkit-authored reason is the shared `const` itself, not a paraphrase copied per fixture. |
| **AC-002** | GIVEN an adapter author who declares every declinable projection capability unsupported — the cheapest way to turn a red build green — WHEN they run the projection suite, THEN the run still emits one test per registered projection rule, the number of observed outcomes equals the number of registered rules, and every registered rule name is present; so `#[cfg]`-ing a gated rule out of the expansion, which is the wrong implementation CF-18 names by hand, fails the suite's own meta-test instead of producing a green build with no record of the trade. | **Unit** — the projection sibling of `capability_skips_are_reported` (`crates/happenstance-testkit/tests/mutation_coverage.rs:3184-3236` is the event-store shape) driving the full projection enumeration against `DecliningProjectionFixture` and comparing observed outcome names against the enumeration. |
| **AC-003** | GIVEN whoever reads that run's CI log and has never seen this suite before, WHEN a projection rule is skipped, THEN the run prints exactly one line for it in the shape already in the tree — ``SKIP {rule}: fixture declines `{capability}` — {reason}`` — rule name first, then the constant they can actually change, then the fixture's own words, with no second format, no projection-local skip type and no extra summary block; and the test compares the reason against the fixture's own `const` rather than a literal repeated in the test, since two copies would let the report carry someone else's words. | **Unit** — value assertions on `RuleOutcome::Skipped { capability, reason }` and on `RuleOutcome::skip_line`'s rendered `String` (`crates/happenstance-testkit/src/contract.rs:458-537`), with `reason` compared against `DecliningProjectionFixture`'s declared `const`. **Static** — `cargo check`: a forked skip type would compile, so the reused-type claim is additionally a reviewed diff against Architecture brief AC-A06. |
| **AC-004** | GIVEN the initiative's promise that a guarantee which does not apply is reported rather than absent, WHEN the projection suite's own meta-test checks that promise, THEN it reads `RuleOutcome` / `Verdict` values and never captured stdout — because libtest exposes nothing programmatically and suppresses a passing test's output without `--show-output` — and its mirror, run against a projection fixture that supports everything, reports an empty `skipped()`, so a gate wired to the wrong constant fails in both directions rather than only the convenient one. | **Unit** — the same meta-test asserting through `mutation_coverage/harness.rs`'s `Verdict` / `skipped()` vocabulary (`:96`, `:490-505`); plus the fully-capable mirror run, the projection sibling of `mutation_coverage.rs:3307-3316`. `#[must_use]` on `RuleOutcome` (`contract.rs:462-472`) is what makes an unreported outcome a build failure rather than a review finding. |
| **AC-005** | GIVEN an adapter author who reads the skip machinery as licence to opt out of a MUST, WHEN they decline a projection capability the recorded set marks as one, THEN that rule **panics carrying their own stated reason** rather than skipping, it never appears in `skipped()`, and the projection `MUST_REJECT` / `MUST_SKIP` slices are checked in both directions — so a rule that gains a gate without a listing, a listed rule that stops skipping, and a `must!` quietly downgraded to `require!` each fail by name instead of rotting into decoration as later stories add rules. | **Unit** — projection siblings of `mutation_coverage.rs:3271-3305` (`MUST_REJECT` panics, absent from `skipped()`) and `:3287-3298` (`MUST_SKIP` two-direction consistency), over `must!` / `require!` used unchanged (`crates/happenstance-testkit/src/suite.rs:49-84`, `:37-46`). |
| **AC-006** | GIVEN a future contributor who deletes or bypasses a rule's `require!` gate — or a reviewer asked to believe a meta-test that has nothing to fire on — WHEN the suite runs, THEN `DecliningProjectionFixture::connect` panics on the second call so an ungated rule surfaces as `Verdict::Panicked` rather than a quiet pass; and where no gated projection rule exists at this story's merge point, the meta-test's own doc comment says in those words that it is a guard on future registrations rather than a demonstrated skip, naming `read-through-and-rebuild-rules` as the in-project forcing function. | **Unit** — `DecliningProjectionFixture` in `crates/happenstance-testkit/tests/mutation_coverage/variants.rs`, mirroring `DecliningFixture`'s panic-on-second-`connect` (`:539-557`), with the meta-test asserting no rule reaches `Verdict::Panicked`. **Static** — reviewed doc comment, against the precedent that already makes exactly this admission in the same terms (`mutation_coverage.rs:3187-3205`). |
| **AC-007** | GIVEN P3, the local-first / edge developer running the conformance suite on `wasm32-unknown-unknown`, WHEN a projection rule skips on that target, THEN the reason still reaches them — routed as `skip_line`'s `String` into `console_log!` the way `__emit_wasm` already routes the event-store family — rather than through `RuleOutcome::report`, which is a **measured** no-op there, so AC-016's whole-suite wasm run is not the one where the stated reason silently disappears. | **Static** — the mandatory *wasm32 check of the conformance harnesses* step (`xtask/src/main.rs:231`), which type-checks the projection wasm harness under `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown`. **Static (review)** — the emitter route itself, against `crates/happenstance-testkit/src/registry.rs:276-291`; a `report` call on the wasm path is the defect and a compiler cannot see it. |
| **AC-008** | GIVEN a fixture author who declines a capability and forgets the reason — `Capability::declined("")` — WHEN they build, THEN the `const fn` `assert!` fails at **codegen**, caught by `cargo build` and `cargo test` and missed by `cargo check` and `cargo clippy`; `ProjectionFixture`'s own rustdoc says exactly that, so nobody trusts a green `check`; any doctest demonstrating it is spelled bare `compile_fail`, never `compile_fail,E0080`, because rustdoc on 1.97.1 silently ignores an error code it cannot match; and `CHANGELOG.md` gains an entry naming the defect this machinery detects — a capability-gated projection rule vanishing from the binary. | **Unit** — `cargo test -p happenstance-testkit --doc` runs the bare `compile_fail` doctest on the trait. **Static** — the rustdoc text and the changelog entry, reviewed against `contract.rs:400-419`, RS-91-3 (`standards/rust/91-adapter-authoring-recipe.md:156-158`), RS-40-5 (`standards/rust/40-public-surface-and-evolution.md:212`) and CF-29 (`spec/SPECIFICATION.md:8141-8165`), whose own lint is described there as the weakest check in the gate. |

**Coverage of the traced project AC.** Project **AC-005** — *a rule whose projection capability is
declined is still emitted, reports a skip with a reason, and is distinguishable from a pass* — is
carried by AC-002 (still emitted), AC-003 (with a reason, in one line shape), AC-004
(distinguishable, asserted on values), AC-005 (and a MUST is not skippable), AC-006 (and cannot be
faked) and AC-007 (and survives the constrained target). AC-001 and AC-008 are the preconditions that
make the other six mean something: a set that was signed off, and a declension that cannot be
reasonless.

## Interaction quality

This project records **no screen** — `_design.md` carries `surfaces: []` and answers *Items*,
*Signatures* and *Anti-patterns* with `N/A — no user-facing surface` (`_design.md:48-54`, `:52-90`).
That is not an exemption from this section; it relocates it. `_design.md:33-38` names a **text
surface** and pins it to a verified line shape, and RFC §6.7's invariants are read against that
medium. The translation is stated rather than assumed, because an invariant translated silently is an
invariant nobody can check.

Every invariant below is carried by an **AC-### row in the table above**. None is stated only here.

### STATE invariants

| Invariant, in this medium | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump.** The reason reaches the reader *at the rule that skipped* — in that test's own output — never in a separate summary block or a second artefact they must go and open. | **AC-003** | The emitter calls `.report(…)` on what the rule returned, so the line is emitted inside the skipping test (`registry.rs:222-295`); the value assertion binds `capability`/`reason` to that rule's outcome. |
| **Non-occlusion.** A skipped rule neither displaces nor suppresses any other rule's entry; the rest of the run is byte-for-byte what it would have been. | **AC-002**, **AC-004** | Outcome count equals registered-rule count against the declining fixture (AC-002); against the fully-capable fixture `skipped()` is empty and every rule still passes (AC-004). |
| **Preserved focus and selection.** A rule keeps its own test name and its position in the enumeration when it skips, so `cargo test <name>` and `--exact` still find it — the medium's equivalent of a selection surviving a state change. This is precisely what `#[cfg]` destroys. | **AC-002** | The meta-test compares observed names against the enumeration, so a rule that changes or loses its name under declension fails. |
| **Reversibility.** The state is one constant away in both directions: flipping the fixture's constant back to supported returns the rule to running, with no harness regeneration, no macro re-spelling and no second code path. | **AC-004** | The fully-capable mirror is literally the reverse direction of the declining run, asserted in the same test file. |
| **Reachable without a special invocation.** The machine-readable half needs no flag at all (it is a returned value); the human-readable half is on the ordinary run path, not behind a verbose-only opt-in. | **AC-003**, **AC-004**, **AC-007** | `--show-output` is passed by the gate itself, and for the stated reason (`xtask/src/main.rs:132,151`); on `wasm32` the route is `console_log!`, which needs no flag (AC-007). |

### COMPOSITION invariants

Taken from the signed-off text surface (`_design.md:33-38`) and the shape it verifies at
`crates/happenstance-testkit/src/contract.rs:500-507`.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all.** A returned `RuleOutcome::Skipped` that nothing renders is this medium's bare markup — the value is not the surface. Every declined rule produces a *composed line*, not merely a value the test happens to inspect. | **AC-003** | Assertion on `skip_line`'s rendered `String`, in addition to the enum's fields; `#[must_use]` on `RuleOutcome` makes a dropped outcome a build failure (`contract.rs:462-472`). |
| **Composition and placement.** The line is composed by `skip_line` and placed by the emitter at the rule's own test — host emitters through `report` to stdout, the wasm emitter through `console_log!`. | **AC-003**, **AC-007** | `registry.rs:222-295`, `:276-291`; the wasm harness is type-checked by the mandatory step. |
| **Transience: persistent, never revealed on demand.** The line is printed on every run of the suite. It is not gated behind a verbose flag, a failure, or a `--nocapture` the caller has to remember; the gate passes `--show-output` precisely so a *passing* test's skip line is not swallowed by libtest. | **AC-003** | `xtask/src/main.rs:132,151`, whose own comment states the reason; UX brief AC-U09. |
| **Density budget, in real numbers.** Exactly **1** line per declined rule. **3** interpolated fields — rule, capability, reason. **1** vocabulary and **1** line shape across both rule families. **0** new formats, **0** new public skip types, **0** additional summary blocks, **0** widenings of `Capability` or `RuleOutcome`. | **AC-001**, **AC-003** | AC-003's rendered-string assertion fixes the shape; the `0`s are a reviewed diff against Architecture brief AC-A06 and this spec's PR boundary — a fork would compile, so review is the instrument the UX brief names (AC-U08). |
| **Hierarchy.** The reader's first token says *what happened* (`SKIP`), the second *where* (the rule), the third *which switch they can change* (the capability constant), the last *why* (the fixture's words). Where more than one constant gates a skip, all of them are named — the `NO_STORE_LIMITS` precedent, which exists because a skip naming only one would send an author looking for the constant they did set. | **AC-003** | Field order asserted through `skip_line`'s output; `contract.rs:435-442`; UX brief AC-U10. |
| **Named anti-patterns.** `_design.md`'s *Anti-patterns* section is `N/A` because it recorded no surface, so the binding anti-patterns for this story come from two places that do name them: CF-18's wrong implementation — a gated rule `#[cfg]`-ed out, giving *a green build and no record of the trade* (`spec/SPECIFICATION.md:7606-7612`) — and `project.md`'s risk line forbidding a second declension policy, *one policy, or a stated reason for two*. A third, from this repository's own corollary: a meta-test that asserts a demonstrated skip it did not demonstrate. | **AC-002**, **AC-001**/**AC-003**, **AC-006** | Silent omission fails AC-002's count; a second policy or line shape fails AC-001/AC-003's reviewed diff; an overstated non-vacuity claim fails AC-006's doc-comment review. |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `_design.md` does not in fact record the projection capability set, or records it without disposing of DT-3's three reason-writers. | **Halt and report at the story boundary.** Do not invent the set inside a rule-plumbing story: that is how the second, divergent declension policy `project.md`'s risk list forbids gets minted. The report names what is missing and hands it back to `projection-api-design-record` (HS-S0003). |
| **EC-002** | A fixture declares `Capability::declined("")`. | Build failure from the `const fn` `assert!`, at **codegen** — so `cargo build` and `cargo test` reject it and `cargo check` / `cargo clippy` do not. The trait's rustdoc must state that asymmetry; a green `cargo check` is not evidence here (AC-008; `contract.rs:400-419`). |
| **EC-003** | A projection rule spells `require!` but is absent from the projection `MUST_SKIP` slice — or is listed there and stops skipping. | The meta-test fails **naming the rule**, in both directions. A message that says only *counts differ* sends the next contributor to diff two lists by hand (AC-005; `mutation_coverage.rs:3287-3305`). |
| **EC-004** | A rule ignores its gate and calls `connect` a second time on `DecliningProjectionFixture`. | The fixture panics with a message naming the declined capability, and the meta-test reports `Verdict::Panicked` and fails. A quiet pass here would mean the gate can be deleted without anything noticing (AC-006; `variants.rs:539-557`). |
| **EC-005** | The recorded capability set contains nothing a baseline projection rule honestly needs, so the skip half has nothing to fire on at merge. | Ship it as a **stated guard on future registrations**, in the meta-test's own doc comment, naming `read-through-and-rebuild-rules` as the forcing function. Shipping it silent is one failure; describing it as a demonstrated skip is the other, and the worse of the two (AC-006). |
| **EC-006** | A `must!`-gated rule is downgraded to `require!`, or a `MUST_REJECT` entry starts appearing in `skipped()`. | Test failure, not a warning. CF-18's own text says reading it as licence to skip a MUST is the misreading CF-16 names in terms (`spec/SPECIFICATION.md:7614-7617`). |
| **EC-007** | A projection-local skip type, a second line shape, or a widened `Capability` / `RuleOutcome` appears in the diff. | Rejected at review and by this spec's PR boundary. It compiles, so no compiler catches it — Architecture brief AC-A06 and UX brief AC-U08 are the instruments, and the cost is an adapter author learning two shapes from one CI log. |
| **EC-008** | The projection wasm harness calls `RuleOutcome::report`. | Rejected at review. It type-checks and does nothing: `report` is a measured no-op on `wasm32-unknown-unknown` (`contract.rs:511-531`), so this is the failure mode where the run is green and the reason is gone (AC-007). |

## Non-functional

| id | requirement | why, and where it is anchored |
| --- | --- | --- |
| **NF-001** | **A supporting fixture pays nothing.** After monomorphisation the declined half is discarded; the pass path adds no allocation, no `dyn` dispatch and no branch in the emitter. | The branch lives in the rule body precisely so the emitter keeps none (`crates/happenstance-testkit/src/suite.rs:15-46`). An emitter branch would be `#[cfg]`, which is the thing CF-18 forbids. |
| **NF-002** | **No new dependency, no new feature flag, no new CI step.** The meta-test is an ordinary in-process `#[test]`; the wasm check reuses a gate step that already exists. | Testing brief AC-016 row — *AC-016 costs this project a new harness file, not a new gate step*. `happenstance-core` gains nothing here (`projection-probe-conformance-feature` already landed its `conformance` feature). |
| **NF-003** | **The MSRV stays 1.97.1.** Associated consts, `const fn` `assert!` and `stringify!` are all far below the floor; nothing here moves it. | ADR-0029 / `CLAUDE.md` *Binding constraints* 5 — the floor may be weighed, but never moved in silence, and this story has no reason to weigh it. |
| **NF-004** | **Nothing acquires a `Send` bound.** No `#[async_trait]`, no `+ Send` written into `ProjectionFixture` or the meta-test's plumbing; the wasm harness must keep compiling on the `!Send` flavour. | ADR-0001, `CLAUDE.md` *Binding constraints* 1. The wasm32 conformance-harness step is the standing guard, and it is mandatory rather than skip-if-absent. |
| **NF-005** | **Every new public item carries rustdoc, and the docs build clean on stable and on nightly `--cfg docsrs`.** A capability constant with no doc comment is a constant an adapter author has to guess the meaning of. | `standards/rust/70-rustdoc-obligations.md`; the docs step and the nightly `--cfg docsrs` step of `cargo xtask ci`. |
| **NF-006** | **No borrowing GAT on the projection fixture or its instrument.** Owned handles holding a refcount, as `MemoryFixture` does. | `contract.rs:97-111` and `experiments/rustc-ice-gat-foreign-trait/` — an rustc ICE this repository already minimised and which still reproduces on 1.97.1. |
| **NF-007** | **The skip line stays one log line.** No multi-line reason, no embedded newline in a `Capability` reason, so a CI log grep for `SKIP ` returns one row per skip. | The line shape verified at `contract.rs:500-507`; the density budget above. |

## Implementation notes (non-prescriptive)

Not instructions — the shape of the problem, so the implementer spends their judgement on the parts
that need it.

- **Read `_design.md`'s recorded set first, before writing anything.** Everything in this story is
  downstream of it, and EC-001 is a real outcome rather than a formality: HS-S0003 is a `blocked_by`
  precisely because a set invented here would be indistinguishable, in the diff, from a set that was
  decided.
- **Copy the event-store shape rather than paraphrasing it.** `capability_skips_are_reported`
  (`mutation_coverage.rs:3184-3317`) already solved the hard parts — driving the whole enumeration
  against one hostile fixture, asserting on `Verdict` because libtest gives you nothing, keeping two
  slices two-direction consistent. The projection sibling is the same test with a different
  enumeration; where it must differ, differing is the interesting part and is worth a comment.
- **`DecliningProjectionFixture` belongs beside `DecliningFixture`** in
  `crates/happenstance-testkit/tests/mutation_coverage/variants.rs`, whose module doc already
  explains why a fixture that declines is not a *variant* at all (`variants.rs:24`). Putting it in
  `fixtures.rs` would make it a shipped fixture, which it is not — it is an instrument that exists to
  fail.
- **The projection rules module's filename is `projection-suite-entry-point`'s to choose**, settled
  in the same slice by Note 4's emitter resolution. That is why the PR boundary is a directory glob;
  do not reintroduce a filename this story did not pick.
- **The `must!` arm is the one that is demonstrable today** if the recorded set contains a MUST — a
  second-handle analogue is the live candidate, since `commit_is_atomic_with_the_read_model` reads
  back through a fresh handle. Prefer demonstrating the MUST arm over asserting the skip arm
  vacuously; see *The non-vacuity problem* above for the ordering and for what is not allowed.
- **`stringify!` the constant's identifier, not a prettied string.** The whole point of AC-U10 is
  that a reader can grep the name out of the log and find the `const` in their own fixture.
- **Anything this turns up that deserves an ADR is a named gap, not an ADR.** `.kb/` atoms are
  authored by `/redkiln:kb-ingest` from `.kb/_intake/`; record the gap for the runbook's ADR pass
  (`CLAUDE.md`, *Where the work lives*).

## Tests and CI (merge gate)

Tier vocabulary is the Testing brief's, unchanged: **Static** reads source/config without executing
the code under test; **Unit** is an in-process `#[test]`; **Integration** is a conformance rule
actually driving a `ProjectionStore`; **E2E** is `cargo xtask ci` run whole
(`_decomposition.md:756-767`).

| tier | command / path | proves |
| --- | --- | --- |
| **Unit** | `cargo test -p happenstance-testkit --test mutation_coverage` → the projection sibling of `capability_skips_are_reported`, in `crates/happenstance-testkit/tests/mutation_coverage.rs` | AC-002 (no rule vanishes), AC-003 (the composed line and the fixture's own words), AC-004 (value assertion + the fully-capable mirror), AC-005 (`MUST_REJECT` / `MUST_SKIP`, both directions), AC-006 (`Verdict::Panicked` on a bypassed gate) |
| **Unit** | `cargo test -p happenstance-testkit --test mutation_coverage` → `projection_capability_reasons_are_authored_once` | AC-001's testable half: no empty reason, and a testkit-authored reason is the shared `const` rather than a per-fixture paraphrase |
| **Unit (doc)** | `cargo test -p happenstance-testkit --doc` | AC-008 — the bare `compile_fail` doctest on `ProjectionFixture`, and that the trait's rustdoc compiles as written |
| **Integration** | the projection harness files under `crates/happenstance-testkit/tests/` that `projection-suite-entry-point` lands (tokio and blocking), run inside `cargo xtask ci`'s `tests` step with `--show-output` | AC-003's human half — the line actually reaches stdout in a real harness invocation, which is the Testing brief's AC-005 Integration row |
| **Static** | `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` — the mandatory *wasm32 check of the conformance harnesses* step (`xtask/src/main.rs:231`) | AC-007 — the projection wasm harness type-checks; a `Send` bound or an `#[async_trait]` leaking in fails here first (NF-004) |
| **Static** | `cargo clippy --workspace --all-targets --all-features -D warnings`, `cargo fmt --check` | NF-005's baseline; `#[must_use]` misuse and an unreported `RuleOutcome` surface here |
| **Static** | `cargo hack check --workspace --feature-powerset --no-dev-deps` | NF-002 — no new feature flag has been introduced, and the widened powerset stays green |
| **Static (docs)** | the `docs` step and the nightly `--cfg docsrs` rustdoc build of `cargo xtask ci` | NF-005 — every new constant documented, no broken intra-doc link from the `memory`-gated neighbourhood |
| **Static (review)** | reviewed diff against `.bklg/…/_design.md` and against Architecture brief AC-A06 | AC-001 (the set is transcribed), EC-007 (no second vocabulary), AC-006's doc-comment honesty, AC-008's `CHANGELOG.md` entry — each is a claim no compiler can make |
| **E2E** | `cargo xtask ci` | the merge bar: fmt, clippy, tests, the four wasm32 steps, docs, `spec-trace`, `cargo package --list`, and `cargo hack` / `cargo deny` where present |
| **Story grain** | `cargo xtask affected --base main`, then `cargo xtask ci --fast` | the loop during implementation, per `CLAUDE.md` *Commands* and the Testing brief's merge-gate block. **`--fast` is not evidence for AC-007**: it omits the mandatory wasm32 steps, which is exactly what AC-007 is proven by |
| **Ledger** | `redkiln verify --grain story` over `.bklg/…/projection-capability-skips/_ledger.md` | every AC-### above is present, `satisfied: true` and carries non-placeholder evidence before `implement → report` |

## Risks and coupling (PR-scoped)

| risk | why it bites here | mitigation, in this PR |
| --- | --- | --- |
| **The skip half is vacuous at merge.** Both baseline projection rules are ungated and every §4.11 rule is claimed by a sibling story. | A meta-test that asserts nothing fires is exactly the decorative rule `CLAUDE.md` warns about, and it would read as a demonstrated guarantee in the closeout. | The ordered resolution in *The non-vacuity problem*: gate a real rule if the recorded set allows it; otherwise demonstrate the `must!` arm; otherwise ship a **stated** guard (EC-005, AC-006) with `read-through-and-rebuild-rules` named. |
| **DT-3 has not actually been recorded** when this story opens. | HS-S0003 is a `blocked_by`, but *blocked_by satisfied* is a stage transition, not a guarantee the prose answers this question. | EC-001: halt and report. This spec states it as an outcome so it is not resolved by whoever needed it next at 5pm. |
| **Slice coupling with `projection-suite-entry-point`.** The two are implemented in one context; this story's constants hang off a trait HS-S0007 is still writing. | A merge-order inversion would leave capability constants on a trait that does not exist, or a `require!` on a rule not yet registered. | Merge order is fixed (`_storymap.md:109`): entry point first. The PR boundary is a directory glob so the rules module's filename — Note 4's to settle — cannot fail this story's boundary check. |
| **The event-store family gets edited by accident** while copying its shape. | `Fixture`'s capability set and `capability_skips_are_reported` are an existing contract with an existing user; a "harmless" refactor there changes the event-store suite. | Explicitly out of the PR boundary. If sharing is genuinely warranted, it is a separate change justified as a change to the event-store suite. |
| **The wasm route is written by habit.** `report` type-checks on `wasm32` and does nothing. | AC-016's whole-suite wasm run would then be green with every reason gone — the failure mode BR-12 exists to prevent, paid by P3. | EC-008 plus AC-007's reviewed route; `registry.rs:276-291` is the shape to copy rather than re-derive. |
| **A reasonless declension slips past a green `cargo check`.** | The `assert!` on an *associated* const fires at codegen; a contributor running `check` in a tight loop sees nothing. | AC-008: the trait's rustdoc says where it fires, and the doctest is spelled bare `compile_fail` so it is a real check rather than a decorative one. |
| **`Capability` / `RuleOutcome` grow a projection-shaped arm.** | It compiles, and it is the natural thing to reach for the first time the projection family wants one more field. | EC-007; Architecture brief AC-A06 and UX brief AC-U08 are the recorded decision, and this PR's boundary excludes the widening. |

## Dependencies

**Blocks on** (both are this story's `depends_on`, and both are load-bearing rather than ordering
conveniences):

- **`projection-suite-entry-point`** — creates `ProjectionFixture` itself, the single enumeration
  `for_each_projection_store_rule!`, `projection_store_conformance!` with its `__private` export, the
  three harness files and the two baseline rules. There is no trait to hang a capability constant on
  until it lands, and no enumeration for the meta-test to drive. Same slice; implemented in the same
  context, this story second.
- **`projection-api-design-record`** — records DT-3's resolution and the projection capability set in
  `_design.md`, and DT-8's arm. This story **transcribes** that record; without it, EC-001 fires.

**Unlocks** (both name this story in their own `depends_on`, `_storymap.md:63-64`):

- **`reset-rules`** — `refused_reset_changes_nothing` is the rule the fixture-side reset capability
  exists for (PS-18, `spec/SPECIFICATION.md:5200-5217`).
- **`read-through-and-rebuild-rules`** — lands the real `READS_THROUGH_BATCH = false` instance, the
  second half of project AC-005 and the forcing function this story names if its own skip half ships
  as a guard.

Nothing outside this project is blocked on this story, and nothing in it depends on substrate this
initiative does not already own.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than dropped. The Context pack above is sufficient to start; open
these at the moments named. **Link, do not paste.**

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | The signed-off record. It carries the `surfaces: []` no-screen determination, the two non-visual surfaces (`:24-38`) and — once HS-S0003 lands — DT-3's resolution and the projection capability set this story transcribes. | **First, before writing a line.** If it does not answer the capability set, stop and report (EC-001). | AC-001, AC-003 |
| `crates/happenstance-testkit/src/contract.rs` | `Capability` and `declined`'s const-eval reason check (`:355-433`), the `NO_STORE_LIMITS` / `NO_CEILING_REASON` testkit-written-reason precedent (`:435-456`), and `RuleOutcome` / `skip_line` / `report` with its two honest per-target limitations (`:458-537`). The mount point, and the whole reused vocabulary. | When writing the capability constants and their rustdoc; again when asserting on the rendered line. | AC-001, AC-003, AC-004, AC-008 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `MUST_REJECT` / `MUST_SKIP` and `capability_skips_are_reported` (`:3139-3317`) — the exact shape the projection sibling copies, including the guard-not-demonstration admission at `:3187-3205`. | Before writing the meta-test; read the whole function once rather than sampling it. | AC-002, AC-004, AC-005, AC-006 |
| `crates/happenstance-testkit/tests/mutation_coverage/variants.rs` | `DecliningFixture` (`:502-565`), including the panic-on-second-`connect` that turns an ignored gate into a loud failure, and the module doc explaining why a declining fixture is not a variant (`:24`). | When building `DecliningProjectionFixture`. | AC-006, AC-003 |
| `crates/happenstance-testkit/tests/mutation_coverage/harness.rs` | `Verdict` (`:96`) and `skipped()` (`:490-505`) — the value-assertion vocabulary that exists because libtest exposes nothing programmatically. | When deciding what the meta-test asserts on. Reach for this instead of capturing stdout. | AC-004 |
| `crates/happenstance-testkit/src/suite.rs` | `require!` (`:37-46`) and `must!` (`:49-84`), and the comment explaining why the branch is in the rule body and never in the emitter (`:15-46`). Both are used unchanged. | Before applying a gate to any projection rule. | AC-002, AC-005 |
| `crates/happenstance-testkit/src/registry.rs` | The three emitters (`:222-295`) and `__emit_wasm`'s `console_log!` route (`:276-291`) — the shape the projection wasm harness must copy rather than re-derive. | When wiring the wasm path; before assuming `report` works there. | AC-007 |
| `crates/happenstance-testkit/src/lib.rs` | The public export block beside `Capability` / `Fixture` / `RuleOutcome` (`:187`) and the `__private` module the macro expansion names in the adapter's own crate (`:362-364`). A constant not reachable through both is one no adapter can set. | At mount time, before declaring the story done. | AC-001 |
| `spec/SPECIFICATION.md` | **CF-18 `[FROZEN]`** (`:7597-7617`) — the clause this story instances, its named wrong implementation, and its own warning against reading it as licence to skip a MUST; **PS-12** (`:5052-5074`) and **PS-18** (`:5200-5217`), observed not amended; **CF-29** (`:8141-8165`) for the changelog entry. | Before writing the meta-test's doc comment, and before touching anything that looks like a clause. | AC-002, AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | The UX brief's text-surface criteria AC-U08 – AC-U12 (`:159-208`), the Architecture brief's AC-A06 and Notes 1/4/5, and the Testing brief's AC-005 row (`:775`). This spec distils them; the briefs carry the reasoning and the counter-examples. | When a composition or vocabulary question is not answered by this spec. | AC-001, AC-003, AC-004, AC-007 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` | Project **AC-005** (`:194-197`), **DR-03** (`:152`), **DR-07** (`:164-168`) and the risk line forbidding a second declension policy. The trace target. | When checking that a change still serves the AC it traces to. | AC-001 – AC-008 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` | The slice, the merge order (`:109`), and the Coverage row splitting AC-005 across this story and `read-through-and-rebuild-rules` (`:83`). It is what makes taking a §4.11 rule here scope theft. | Before adding any conformance rule — the answer is that this story adds none. | AC-002, AC-006 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | The open question DT-3 answers by citation rather than re-litigation. Read it to understand what a second declension policy would cost; do **not** change its status here. | If the recorded resolution looks under-argued and the temptation to re-decide appears. | AC-001 |
| `standards/rust/91-adapter-authoring-recipe.md` | RS-91-3 (`:156-158`) — the associated-const lazy-evaluation trap, stated where an adapter author meets it. | When writing `ProjectionFixture`'s rustdoc about where an empty reason fires. | AC-008 |
| `standards/rust/40-public-surface-and-evolution.md` | RS-40-5 (`:212`) — why the `compile_fail` doctest is spelled bare rather than `compile_fail,E0080`. | Before writing the doctest. The stricter-looking spelling is the weaker check. | AC-008 |
| `standards/rust/70-rustdoc-obligations.md` | The rustdoc obligations every new public constant inherits, including naming the alternative that lost where a shape is unusual. | When documenting the capability constants. | NF-005, AC-001 |
| `xtask/src/main.rs` | The mandatory *wasm32 check of the conformance harnesses* step (`:231`) and the `--show-output` flag with the comment stating why it is not noise (`:132,151`). | When claiming AC-007, and when reasoning about whether a line reaches a human. | AC-007, AC-003 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | P2 the adapter author (`:114-153`) and the evidence-strength qualification (`:361-363`) — no persona here has been directly observed. | When writing anything that claims what an author wants. Carry the qualification. | AC-001, AC-002, AC-007 |
| `experiments/rustc-ice-gat-foreign-trait/` | The minimised rustc ICE that still reproduces on 1.97.1, and the reason no borrowing GAT goes on a fixture. | Only if a borrowing associated type starts to look convenient on the instrument. | NF-006 |
| `RUNBOOK.md` | Phase 6 in full (`:3848-3965`), including why the read model cannot be observed without the probe (`:3882-3892`). The plan of record this project executes. | For orientation on where this story sits in the phase, or if the scope feels wrong. | AC-002 |

## Clarifications resolved during spec

1. **The eight AC ids are exactly those the first pass decided** — AC-001 through AC-008 — and none
   was added or dropped. The ledger matches them one for one.
2. **This story adds no `ProjectionStore` conformance rule.** All seventeen §4.11 adapter rules are
   claimed by sibling stories in `_storymap.md`. Taking one would be scope theft from `reset-rules`
   or `read-through-and-rebuild-rules`, and the sharp consequence — the skip half may have nothing to
   fire on at merge — is stated in the Context pack rather than discovered during implementation.
3. **Non-vacuity is resolved as an ordered preference, not a rule.** Gate a real rule if the recorded
   set allows it; otherwise demonstrate the `must!` arm, which is non-vacuous today; otherwise ship a
   **stated** guard. Inventing a capability nothing needs, or writing a private rule the suite does
   not run, is excluded in all three arms (EC-005, AC-006).
4. **RFC §6.7's interaction-quality invariants are read against a text surface, and the translation
   is written down.** `_design.md` records `surfaces: []`, so there is no screen; there *is* a
   surface, and it is one line of CI output. In-place-ness becomes attribution to the skipping rule;
   non-occlusion becomes an unchanged run for every other rule; preserved selection becomes a
   preserved test name; reversibility becomes one constant in both directions. Each is an AC row, not
   a bullet, so `redkiln verify` can extract it.
5. **`_design.md`'s *Anti-patterns* section says `N/A`, while its sign-off refers to *the
   anti-patterns recorded above*.** That is an internal inconsistency in the approved record, noted
   rather than silently reconciled. This spec does not invent anti-patterns to fill the gap: it binds
   to the two that *are* named elsewhere and normative — CF-18's wrong implementation and
   `project.md`'s one-policy risk line — plus the repository's own decorative-rule corollary, and
   says which AC carries each.
6. **The density budget is stated in real numbers** (1 line, 3 fields, 1 vocabulary, 0 new formats /
   types / summary blocks / widenings) because *reuse `RuleOutcome` unchanged* is a decision and *one
   line per skip in this exact shape* is the checkable consequence. The numbers come from the shape
   `_design.md:33-38` verified at `contract.rs:500-507`, not from this spec.
7. **The projection rules module's filename is deliberately not fixed here.** It is settled in the
   same slice by `projection-suite-entry-point` under Architecture brief Note 4, which is why the PR
   boundary uses a directory glob — a boundary naming a filename this story did not choose would fail
   for a reason unrelated to scope.
8. **Test names in the verification column are proposals at real paths.** The files
   (`crates/happenstance-testkit/tests/mutation_coverage.rs`, `.../mutation_coverage/variants.rs`)
   exist today and are where these tests belong; the function names are the implementer's to confirm
   against whatever module HS-S0007 lands. The ledger records the same names so a rename is a visible
   ledger edit rather than a silent divergence.
9. **No `.kb/` atom is written and no open question's status is changed.** DT-3's resolution is
   consumed from `_design.md`; `.kb/open-questions/cf-40-fixture-limits-ownership.md` is read and
   cited only. Anything this work turns up that deserves an ADR is recorded as a named gap for the
   runbook's ADR pass, per `CLAUDE.md`.
