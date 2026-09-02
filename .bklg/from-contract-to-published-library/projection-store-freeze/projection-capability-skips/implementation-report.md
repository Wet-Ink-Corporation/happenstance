---
item: "HS-S0008"
stage: implement
created: "2026-08-14"
updated: "2026-08-14"
---

# Implementation Report — a declined projection capability is a reported skip, never a silent absence

> **STATUS: eight of eight ACs satisfied.** CF-18 now has a projection instance,
> and its MUST arm is **demonstrated on real values rather than asserted
> vacuously**: both baseline projection rules reject a fixture that declines
> `SECOND_HANDLE`, carrying that fixture's own stated reason.
>
> The skip arm is a **stated guard**, and the meta-test says so in those words.
> No projection rule spells `require!` yet, because the rules that will —
> `refused_reset_changes_nothing` (PS-18) and `batch_reads_reflect_pending_writes`
> (PS-12) — belong to `reset-rules` and `read-through-and-rebuild-rules`. Both
> are named in the code as the forcing functions. Overstating this would have
> been the worse of the two available failures.

**EC-001 did not fire.** `_design.md`'s DT-3 resolution records the projection
capability set in a table with a row per constant, its home, whether it is a
MUST, and who writes its reason (`_design.md:276-298`). It was transcribed, not
invented.

## TDD Evidence

Two Red/Green cycles, because the surface and the behaviour fail differently.

**Cycle 1 — RED (AC-001).** The instrument, the harness and both meta-tests were
written first, naming constants that did not exist:

```text
error[E0438]: const `RESET_REFUSAL` is not a member of trait `ProjectionFixture`
   --> crates\happenstance-testkit\tests\mutation_coverage\variants.rs:632:5
error[E0599]: no associated function or constant named `SECOND_HANDLE` found for
              type parameter `S` in the current scope
   --> crates\happenstance-testkit\tests\mutation_coverage\harness.rs:607:12
```

**Cycle 1 — GREEN.** `SECOND_HANDLE` and `RESET_REFUSAL` landed on
`ProjectionFixture`, both required rather than defaulted, and
`projection_capability_reasons_are_authored_once` went green.

**Cycle 2 — RED (AC-002, AC-005, AC-006), and this one is behavioural.** With
the constants declared but no rule gated on them, the meta-test compiled and
**failed on values**:

```text
running 2 tests
test mutation_coverage::projection_capability_skips_are_reported ... FAILED
test mutation_coverage::projection_capability_reasons_are_authored_once ... ok

---- mutation_coverage::projection_capability_skips_are_reported stdout ----
panicked at crates\happenstance-testkit\tests\mutation_coverage.rs:3443:21:
`commit_advances_the_checkpoint` must reject a projection fixture declining the
`SECOND_HANDLE` MUST *and carry the fixture's own stated reason*, so the failing
build says why. Message: `DecliningProjectionFixture` declines SECOND_HANDLE, so
a rule that opened a second handle ignored the capability gate
```

That transcript is worth reading twice, because it is EC-004 firing on its own
first run. The ungated rule did exactly what an ungated rule does — it opened a
second handle — and `DecliningProjectionFixture::connect`'s panic-on-second-call
converted what would have been a quiet pass into a loud `Verdict::Panicked` that
the meta-test then rejected for carrying the *instrument's* message rather than
the *fixture's stated reason*. The instrument was proven to work by the absence
of the thing it guards, before the thing it guards existed.

**Cycle 2 — GREEN.** `must!(F: SECOND_HANDLE)` on both rules:

```text
running 2 tests
test mutation_coverage::projection_capability_skips_are_reported ... ok
test mutation_coverage::projection_capability_reasons_are_authored_once ... ok
```

**AC-008 — the doctest pair.** Both blocks run under
`cargo test -p happenstance-testkit --all-features --doc`:

```text
test crates\happenstance-testkit\src\contract.rs - contract::ProjectionFixture (line 437) - compile fail ... ok
test crates\happenstance-testkit\src\contract.rs - contract::ProjectionFixture (line 473) ... ok
```

The first is spelled **bare** `compile_fail`. Bare `compile_fail` passes when a
snippet fails to compile for *any* reason, so the second block is its twin: the
same fixture with one expression changed — `Capability::declined("")` becomes a
sentence — and it must compile. A typo, a renamed item or a wrong path breaks the
twin and turns the pair red, so the only thing the pair can be reporting is the
expression that differs between them.

## Commits

`feat(projection-store-freeze): A declined capability is not a pass` — one
checkpoint commit, the second and last of slice `projection-conformance-suite`,
on `initiative/from-contract-to-published-library`, immediately after
`The projection suite entry point`.

Named by subject and by predecessor rather than by hash, for the reason its
slice-mate's report gives: this file is committed inside the commit it
describes. `git log --grep "Story: projection-store-freeze/projection-capability-skips"`
resolves it.

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-testkit/src/contract.rs` | `ProjectionFixture` gains `SECOND_HANDLE` and `RESET_REFUSAL`, both **required**; a trait-level section on where an empty reason fires (codegen, not `check`) carrying the bare `compile_fail` doctest and its compiling twin. `Capability` and `RuleOutcome` are untouched — the diff of this file contains no line of either. |
| `crates/happenstance-testkit/src/projection.rs` | A projection-local `must!` (a second definition, not a fork of the policy — see Notes) and `must!(F: SECOND_HANDLE)` on both baseline rules. |
| `crates/happenstance-testkit/src/fixtures.rs` | `MemoryProjectionFixture` declares the two constants: `SECOND_HANDLE` supported, `RESET_REFUSAL` declined with the reason taken from the store's own `reset`. |
| `crates/happenstance-testkit/src/lib.rs` | The crate doc's projection paragraph gains the declension vocabulary: same `Capability`, same `RuleOutcome`, same one-line shape, two constants, and why `READS_THROUGH_BATCH` is on the probe instead. |
| `crates/happenstance-testkit/tests/mutation_coverage/harness.rs` | `ProjectionSubject`, `projection_probes`, `run_projection_subject`, `projection_declines` — the event-store trio's siblings, sharing `Probe`, `Verdict`, `run_probe` and `SubjectReport` unchanged. |
| `crates/happenstance-testkit/tests/mutation_coverage/variants.rs` | `DecliningProjectionFixture` (correct store, hostile declarations, panic on the second `connect`) and a `ProjectionSubject` impl for `MemoryProjectionFixture` as the fully-capable mirror. |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `all_projection_rules`, `PROJECTION_MUST_REJECT`, `PROJECTION_MUST_SKIP` (empty, documented as a guard), `assert_projection_declension`, and the two new meta-tests. |
| `CHANGELOG.md` | One entry naming the defect the machinery detects. |
| `standards/rust/*.md` (7 files) | 17 `file:line` citations repaired after `contract.rs` and `lib.rs` moved again. |

## Gates

| Command | Result |
| --- | --- |
| `cargo test -p happenstance-testkit --all-features --test mutation_coverage projection` | 2 passed |
| `cargo test -p happenstance-testkit --all-features` | every target green, including 22 doctests (4 ignored) |
| `cargo test -p happenstance-testkit --all-features --doc` | the `compile_fail` doctest and its twin both ok |
| `cargo xtask affected --base main` | **`affected gate passed`** |
| `cargo xtask ci --fast` | **`all required checks passed (--fast: 4 optional step(s) not run)`** |
| `cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown` *(explicit, AC-007)* | `Finished dev profile` |
| `cargo xtask spec-trace` *(explicit)* | `traceability: no problems found; §7.1–§7.2 matches the checker` |
| `cargo fmt --all --check` | clean |

## Notes

**The one design judgement this story had to make, and why.**
`require!`/`must!` in `suite.rs` are spelled
`<$fixture as $crate::Fixture>::$capability` — the *event-store* trait. They
cannot be reused verbatim by a projection rule, and there were two ways out:
parameterise them over a trait path (which means exporting a private macro to the
crate root and touching thirty existing call sites to buy four lines), or restate
four lines beside the family that uses them. This story took the second, which is
the same choice the family already made about its emitters, and the projection
`must!` carries a comment saying so.

**This is not a second declension policy**, and the distinction is worth being
precise about because `project.md`'s risk list forbids one. What a second policy
would mean is a second *vocabulary*: another skip type, another line shape,
another set of reason-writers. None of those exist — `Capability`, `RuleOutcome`
and `skip_line` are reused byte-for-byte, the trait's constants are `Capability`,
and the reasons are fixture-written exactly as DT-3 says. What was duplicated is
four lines of `if let Some(reason) = … { panic!(…) }`, and the panic *message*
differs because it has to: the event-store one cites CF-16 and sends the reader
to `MemoryFixture`, and neither sentence is true for a projection adapter.

**Why `RESET_REFUSAL` is required rather than defaulted.** `MID_BATCH_FAULT` is
defaulted on `Fixture`, and copying that here would have meant a *testkit-written*
default reason on a constant DT-3 assigns to the fixture. The one standing
exception to "the fixture writes the reason" is `NO_CEILING_REASON`, which earns
it because "this store has no ceiling" is the same sentence for every store that
says it. "This store refuses no reset" is not: *why* it refuses none is the
interesting half, and it differs per adapter. So the cost is one line per fixture
and the benefit is that no adapter can decline by accident.

**What was deliberately not landed.** A testkit-written reason const for
`READS_THROUGH_BATCH`. `_design.md` assigns that constant a testkit-written
reason — and puts it on `ProjectionProbe` in the contract crate, not on the
fixture. The rule that reads it (`batch_reads_reflect_pending_writes`, PS-12) is
`read-through-and-rebuild-rules`'s, so a reason const landed here would be a
capability string no rule reads: the decorative shape CLAUDE.md's corollary
names. AC-001's "exactly the constants the record contains" is about
`ProjectionFixture`'s block, and that block is exactly the record's two fixture
rows. The trait's rustdoc and the crate doc both say where the third switch
lives.

**Two things this story did not touch, deliberately.** The event-store family's
`MUST_REJECT` / `MUST_SKIP` and `capability_skips_are_reported` — an existing
contract with an existing user — and `crates/happenstance-core/**`, whose
`READS_THROUGH_BATCH` is read and cited rather than modified.

**The citation repair recurred**, as the slice-mate's report predicted it would:
inserting the constants and the doctest into `contract.rs` moved every line below
them past `lint-constitution`'s ten-line slack, and 17 citations were recomputed
from their own needles. Two stories in a row have paid this, which is the signal
that the tool should grow a `--write` that repairs citations rather than only the
router. Recorded as a finding rather than absorbed.
