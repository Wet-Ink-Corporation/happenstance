---
item: "HS-S0008"
stage: report
created: "2026-08-14"
updated: "2026-08-14"
---

# Report — a declined projection capability is a reported skip, never a silent absence

## Findings Ledger

**Eight of eight ACs satisfied, each by real reachable behaviour with a real
test.** Two meta-tests over the whole projection enumeration, one hostile
instrument, one fully-capable mirror, and a `compile_fail` doctest paired with a
compiling twin.

**What is demonstrated and what is a guard, stated rather than blurred.** The
**MUST arm is demonstrated on real values**: both baseline projection rules
reject `DecliningProjectionFixture` by panicking with that fixture's own stated
reason, and the RED transcript shows the meta-test rejecting the ungated version
first. The **skip arm is a guard on future registrations** — no projection rule
spells `require!` yet, because every rule that will is claimed by a sibling
story — and `PROJECTION_MUST_SKIP`'s own doc comment says so in those words,
naming `reset-rules` (HS-S0012) and `read-through-and-rebuild-rules` (HS-S0013)
as the in-project forcing functions. Describing it as a demonstrated skip would
have been the worse of the two available failures.

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — the recorded set, transcribed not invented | **Met** | `contract.rs:559-621` declares exactly `_design.md`'s two **fixture** rows — `SECOND_HANDLE` (MUST) and `RESET_REFUSAL` (declinable) — and no third. `READS_THROUGH_BATCH` is the record's *probe* row and stays on `ProjectionProbe`, cited in rustdoc rather than duplicated. Both reasons are fixture-written through `Capability::declined`, per DT-3's default; the family adds no testkit-written reason and the trait's doc says why one would be wrong. `projection_capability_reasons_are_authored_once` asserts every reason is non-empty and that the instrument's reported declensions **are its own two `const`s**, compared against the constants rather than against literals repeated in the test. |
| **AC-002** — no rule vanishes | **Met** | `projection_capability_skips_are_reported` drives the whole enumeration against a fixture declining everything and asserts the outcome count equals the registered count and every name is present. The universe comes from `for_each_projection_store_rule!` itself, never from a second list — so a rule `#[cfg]`-ed out fails here, which is CF-18's named wrong implementation. |
| **AC-003** — one vocabulary, one line shape | **Met** | The diff of `contract.rs` adds a trait and changes not one line of `Capability` or `RuleOutcome`; `skip_line` still renders the single ``SKIP {rule}: fixture declines `{capability}` — {reason}`` shape and nothing projection-local exists. The `capability` is `stringify!` of the const's identifier, so a reader can grep it out of a log and find it in their own fixture. The reason is compared against the fixture's `const`, and the MUST arm's panic is asserted to *contain* that same constant. |
| **AC-004** — asserted on values, and in both directions | **Met** | Every assertion reads `Verdict` / `RuleOutcome`; nothing captures stdout. The mirror is in the same test: `MemoryProjectionFixture` must report an empty `skipped()` **and** `Verdict::Passed` for every rule, so a gate reading the wrong const fails in both directions. `#[must_use]` on `RuleOutcome` is the build failure behind an unreported outcome, and `-D warnings` is green. |
| **AC-005** — a MUST is rejected, not skipped | **Met, demonstrated** | Both rules spell `must!(F: SECOND_HANDLE)` (`projection.rs:213-218`, `:279-283`) and both panic against the declining fixture carrying its reason; both are asserted **absent** from `skipped()`, so a `must!` downgraded to `require!` fails. `assert_projection_declension` checks all three arms in both directions: a `MUST_REJECT` rule that passes, a skip not in `MUST_SKIP`, and a panic not in `MUST_REJECT` each fail **by name**. |
| **AC-006** — the gate cannot be deleted quietly | **Met** | `DecliningProjectionFixture::connect` panics on the second call, over a *correct* `MemoryProjectionStore`, so the only thing wrong with the instrument is what it declares. It fired for real during the RED run — an ungated rule opened a second handle and became a `Verdict::Panicked` the meta-test rejected. The non-vacuity admission is in the code, in the same words the repository's existing precedent uses. |
| **AC-007** — the reason survives `wasm32` | **Met** | The projection wasm emitter routes `skip_line` → `console_log!` and never `report`, copied in shape from `__emit_wasm` rather than re-derived, with the measured no-op stated in its rustdoc. `cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown` green, run explicitly and inside the gate. A `report` call there type-checks and does nothing, so this half is a reviewed claim and is recorded as one. |
| **AC-008** — a reasonless declension fails the build, and the trait says where | **Met** | `contract.rs:437-499` states the codegen/`check` asymmetry and demonstrates it with a **bare** `compile_fail` doctest plus a compiling **twin** differing in one expression, so the pair cannot be passing for an unrelated reason. Both run under `cargo test --doc`. `CHANGELOG.md:28-45` names the defect: a capability-gated projection rule vanishing from the binary, leaving a green build and no record of the trade. |

### Deferred, and to whom

| Thing | Owner | Why it is not here |
| --- | --- | --- |
| A rule that spells `require!`, and therefore a non-empty `PROJECTION_MUST_SKIP` | `reset-rules` (HS-S0012, `refused_reset_changes_nothing` / PS-18) and `read-through-and-rebuild-rules` (HS-S0013, `batch_reads_reflect_pending_writes` / PS-12) | All seventeen §4.11 rules are claimed by sibling stories; taking one here to make the list non-empty would be scope theft dressed as evidence. Both are named in `PROJECTION_MUST_SKIP`'s doc comment. |
| A testkit-written reason const for `READS_THROUGH_BATCH` | `read-through-and-rebuild-rules` (HS-S0013) | `_design.md` puts that constant on `ProjectionProbe`, not on the fixture, and the rule that reads it is that story's. A reason const with no reader is the decorative shape the corollary names. |
| `CheckpointOnlyStore` and the projection mutant registry | `projection-mutant-registry` | This story's instrument is a fixture that *declines*, not a store that is *wrong*. The two answer different questions and the spec's boundary separates them. |

### Findings raised, not fixed

1. **`lint-constitution` has no citation repair, and two stories in a row have
   paid for it.** Inserting items into `contract.rs` moves every line below them
   past the lint's ten-line slack; `--write` repairs the router only. 23
   citations were recomputed in the slice's first story and 17 in this one, each
   from its own unchanged needle. The needle is what identifies the citation, so
   a `--write` that repaired line numbers from needles would be mechanical and
   safe. Recorded rather than absorbed.

2. **`suite.rs`'s `require!`/`must!` are not reusable across fixture traits.**
   They name `$crate::Fixture` in their expansion, so a second family needs its
   own four lines. This story restated them rather than exporting a private macro
   to the crate root and rewriting thirty call sites; the choice is documented at
   the projection `must!`. If a *third* fixture trait ever appears, the trade
   flips and the macro should be parameterised then.

3. **The projection family has no `require!` at all yet**, deliberately: an
   unused macro is a `-D warnings` failure under `unused_macros`, and a gate
   nothing calls is decorative. It lands with the first rule that needs one.
