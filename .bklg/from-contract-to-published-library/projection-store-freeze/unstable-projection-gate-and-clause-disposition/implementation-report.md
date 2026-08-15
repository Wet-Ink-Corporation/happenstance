---
item: "HS-S0016"
stage: implement
created: "2026-08-15"
updated: "2026-08-15"
---

# Implementation Report — The module stops lying about its own maturity

> **STATUS: nine of nine ACs satisfied. Nothing blocked, nothing stubbed.**
> `cargo xtask ci` green, run whole, with all four `OPTIONAL` steps **run** and
> none skipped. `redkiln validate --kb` passes; `redkiln doctor` reports the six
> standing `template-drift` advisories and no seventh.

Three things landed, and only the first is what the story's title says.

1. **The gate.** `unstable-projection` exists on `happenstance-core`, off by
   default, and `pub mod projection;` plus its re-exports are behind it. The
   module header no longer says the conformance suite does not cover the port —
   it says what PS-2 actually requires, what this project shipped instead, what
   would clear the bar, and who decides the 0.1 exposure.
2. **`spec-trace` started answering.** `has_suite` gained the `PS-` prefix, which
   turned §7.2's seventeen daggers on the `PS` block from an accurate statement
   into a checked one and surfaced **ten** rule citations that had never been
   resolved by anything.
3. **The residual repair.** [ADR-0030](../../../../references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md)
   mints **PS-38**. Four rules already enforced that a successful `commit`
   advances the checkpoint and no clause's `MUST` said so; PS-1, PS-19, PS-21 and
   PS-22 are **byte-identical** across the repair.

One thing found and discharged that the spec did not anticipate: **PS-31's second
conjunct was not met.** *"The port MUST say so"* had no instrument, and it turned
out to have no discharge either — nothing in `projection.rs` mentioned the
outward-writing exclusion. It does now.

## TDD Evidence

Two families of test, both written **before** the production change and both
observed failing for the behaviour they assert rather than for a compile error.

| AC | Test | Red (before) | Green (after) |
| --- | --- | --- | --- |
| **AC-005** | `spec_trace::tests::the_projection_family_is_checked_against_its_suite` (`xtask/src/spec_trace.rs:2354-2371`) | `panicked at xtask\src\spec_trace.rs:2350: the projection suite exists in crates/happenstance-testkit/src/projection.rs, so check 4 must resolve PS rule citations rather than abstain on them` — the assertion, not a compile failure | passes once `has_suite` gains `PS-` (`xtask/src/spec_trace.rs:1750-1755`) |
| **AC-005** (guard) | `spec_trace::tests::the_replication_family_still_abstains_and_the_rest_do_not` | **green on arrival**, and recorded as such: it is the ratchet that stops the prefix set from being widened to everything, which would report every unwritten `SY` rule as a typo | unchanged |
| **AC-005** (ratchet) | `spec_trace::tests::the_projection_rules_file_is_swept_for_ownership` | **green on arrival** — `RULE_FILES` already named the projection rules file; the test exists so that removing it is a build failure | unchanged |
| **AC-001** | `tests::the_projection_port_is_behind_an_off_by_default_feature` (`xtask/src/main.rs:1003-1020`) | `` `unstable-projection` is not declared in happenstance-core's `[features]` `` | passes on `crates/happenstance-core/Cargo.toml:68` |
| **AC-001** | `tests::the_gate_is_mounted_on_the_module_and_its_re_exports` (`xtask/src/main.rs:1022-1039`) | `` `pub mod projection;` is not gated the way `memory`'s module is `` | passes on `crates/happenstance-core/src/lib.rs:128-130`, `:160-166` |
| **AC-004** | `tests::every_crate_that_names_a_projection_item_opts_in` (`xtask/src/main.rs:1042-1053`) | `happenstance-sqlite names a projection item and never asks for unstable-projection` | passes once all six manifests name it |
| **AC-004** | `tests::the_typed_layer_makes_no_promise_it_does_not_keep` (`xtask/src/main.rs:1056-1062`) | **green on arrival** — `happenstance` re-exports no projection item and must keep not naming the feature | unchanged |

**One red that was fixed without weakening anything, and it is worth recording
because it is a Windows trap this repository has hit before.** The mount test
first failed on a *true* tree: the working copy is mixed CRLF/LF (`core.autocrlf`
is on and some files were written by an editor), so a `\n`-anchored match failed
on a `\r\n` file. The fix is in the test's `read` helper
(`xtask/src/main.rs:986-996`), which normalises line endings and says why —
rustfmt's `newline_style = "Auto"` is happy with either, so an assertion that is
not is asserting the wrong thing.

**No test was written for AC-002, AC-006, AC-007 or AC-009, and that is stated
rather than omitted.** *Is this reason honest*, *is this marker still true*, *is
this sentence byte-identical*, and *is there a verdict-shaped sentence in the
diff* are judgements. `spec-trace`'s own header says so: *"It cannot tell whether
a rule is the right rule for a clause. That is judgement"* (`xtask/src/spec_trace.rs:17-18`).
The gate is the floor under those four rows; the written disposition below is the
deliverable, and inventing a conformance rule for a feature flag would be exactly
the decorative rule CLAUDE.md's corollary forbids.

## Commits

| SHA | What |
| --- | --- |
| `__STORY1_SHA__` | The whole story as one checkpoint: the gate, the six manifests, the `spec-trace` prefix flip, the PS-1 – PS-38 dispositions, ADR-0030, the intake staging, the RUNBOOK queue row and the changelog entry. |

## Changes

### The gate and its mount

| File | Change |
| --- | --- |
| `crates/happenstance-core/Cargo.toml` | `unstable-projection = []` at `:68`, absent from `default` at `:35`. `conformance = ["unstable-projection"]` at `:97` — the relation, stated. |
| `crates/happenstance-core/src/lib.rs` | `:128-130` gates `pub mod projection;`, `:160-166` the six re-exports, `:174-178` `MemoryProjectionStore` on **both** `memory` and the gate. `:38-45` de-links `[`ProjectionStore`]` from the crate-doc table with the D13 reason. `:100-108` documents the new feature. |
| `crates/happenstance-core/src/projection.rs` | `:3-42` — the status block, replaced. `:56-71` — PS-31's exclusion, stated. Everything else byte-identical. |
| `crates/happenstance-core/tests/projection_memory.rs` | `#![cfg(all(feature = "memory", feature = "unstable-projection"))]` — the test target that proves the store is *mounted* is gated on the pair the store is mounted on. |
| six dependent manifests | sqlite `:37` (its own `projection-store` **enables** the gate), ladybug `:18`, postgres `:18`, neon `:18`, sync `:26`, testkit `:47`. |
| `xtask/src/main.rs`, `xtask/src/spec_trace.rs` | the four feature-table tests and the three `has_suite`/`RULE_FILES` tests; `has_suite` gains `PS-`. |

**The `conformance` × `unstable-projection` relation, stated as EC-002 requires.**
`conformance` **implies** `unstable-projection`. `ProjectionProbe` is defined
*inside* the gated module, so `conformance` alone can only mean one of two things:
it implies the gate, or the probe is gated on both and `conformance` alone becomes
a feature that turns on no item at all. The first was chosen, for a reason the
second would have cost immediately —
`examples/outside-projection-adapter/Cargo.toml:21-23` exists to model *"one
feature flag on a dependency the adapter already has"*, and gating on both would
have made that manifest a two-flag one. The manifest comment at
`crates/happenstance-core/Cargo.toml:69-96` addresses the pre-existing tripwire
head-on: what it guards is the *economic* argument, and an implied feature that is
itself `[]` adds no `dep:` and no graph edge.

**`memory` does not imply the gate; `MemoryProjectionStore` needs both.**
`projection_memory` implements the port the gate is on, so leaving it exported
while the port is invisible would not compile, and making `memory` imply the gate
would hand every default-feature caller a port they never asked for — which is
AC-001's whole point, inverted.

### The per-clause disposition, PS-1 – PS-38

Vocabulary: **accurate** = the marker and the rule citation were read against
today's tree and stand; **repair** = the `Rule:` field or supporting prose
corrected with the `MUST` byte-identical; **discharged** = an obligation now met,
recorded as a discharge with its evidence; **gap** = recorded inside the clause it
is about and routed, never edited. Verdicts marked *(sweep)* are
`references/evaluation/ps-clause-pairing-sweep.md`'s, re-read against the tree as
it now stands.

| Clause | Marker | Disposition | Evidence / route |
| --- | --- | --- | --- |
| **PS-1** | FROZEN, accurate | **gap → repaired by ADR-0030** | `MUST` byte-identical. The clause's own *"Phase 6 owns the repair"* answered in place: a clause of its own. The sweep's amendment travels with it — the obligation was **misfiled** on PS-23, not absent. |
| **PS-2** | FROZEN, accurate | accurate | The bar is unmet and nothing in this diff says otherwise. |
| **PS-3** | PROVISIONAL, **unmoved** | **discharged** | The SHOULD is satisfied by a feature that exists (`Cargo.toml:68`, `lib.rs:130`), guarded by two `xtask` tests. Falsifier *(PS-2's bar met before 0.1)* has not occurred, so the marker stays. |
| **PS-4** | PROVISIONAL, accurate | accurate *(sweep: sound, borrow stated)* | — |
| **PS-5** | PROVISIONAL, accurate | accurate *(sweep: sound, borrow stated)* | — |
| **PS-6** | PROVISIONAL, accurate | accurate *(sweep: sound, the compiler is the falsifier)* | — |
| **PS-7** | FROZEN, accurate | accurate *(sweep: sound, one-for-one)* | — |
| **PS-8** | FROZEN, accurate | **gap recorded and routed** | The `MUST` binds the method's *existence*, the rule its *behaviour*. `dependent`; ADR-0017's range. Written into the clause. |
| **PS-9** | PROVISIONAL, accurate | accurate *(sweep: sound, no-rule pairing dispositioned in-clause)* | — |
| **PS-10** | FROZEN, accurate | **repair** | `compile_fail` is a rustdoc annotation, not a rule. Named as a **compile test**, which is the category `spec-trace` already has; §7.2 now renders the clause's own words rather than daggering a name nothing looked for. |
| **PS-11** | PROVISIONAL, accurate | accurate *(sweep: sound, borrow stated)* | — |
| **PS-12** | PROVISIONAL, accurate | accurate *(sweep: sound; the third sentence's missing falsifier is the clause's own recorded finding)* | — |
| **PS-13** | FROZEN, accurate | **gap recorded and routed** | The rule asserts the **converse** of the `MUST` and runs where the projection is conformant by construction. `dependent`; ADR-0017's range plus §4.11's `READS_THROUGH_BATCH` tension. |
| **PS-14** | FROZEN, accurate | accurate *(sweep: sound)* | The write-behind tension is recorded at PS-13, not counted twice. |
| **PS-15** | PROVISIONAL, accurate | accurate; **inverse-shape observation routed** | The rule covers one of three named methods — the rule being *weaker*, which is a different defect with a different owner. |
| **PS-16** | PROVISIONAL, accurate | **repair** | `probe_delete_all` qualified as `ProjectionProbe::probe_delete_all` so the parser reads it as the seam it is. |
| **PS-17** | FROZEN, accurate | accurate *(sweep: sound, exact falsifier)* | — |
| **PS-18** | PROVISIONAL, accurate | accurate *(sweep: sound)* | The fixture-wording ambiguity is the sweep's observation and stays there. |
| **PS-19** | FROZEN, accurate | **gap → repaired by ADR-0030**; rule **marked new** | `MUST` byte-identical and still scoped *after a successful `reset`*. The unseen-id half is PS-38's second sentence. `fresh_projection_has_no_checkpoint` still does not exist and now says so. |
| **PS-20** | FROZEN, accurate | accurate *(sweep: sound, borrow stated)* | — |
| **PS-21** | FROZEN, accurate | **gap recorded → PS-38** | `MUST` byte-identical. The rule's *"assert the checkpoint advanced"* rests on PS-38. |
| **PS-22** | PROVISIONAL, **unmoved** | **gap recorded → PS-38** | The `MUST` is vacuous against a store that never advances. Marker unmoved: its falsifier is a compacting store and none has appeared. |
| **PS-23** | PROVISIONAL, accurate | accurate; **the sweep's headline recorded at PS-1** | *"Exactly one"* excludes zero, so this is the only sentence that entails progress — incidentally, on a clause about fan-out scope. Not moved; ADR-0030 rejects putting the obligation here. |
| **PS-24** | PROVISIONAL, accurate | accurate *(sweep: sound)* | — |
| **PS-25** | PROVISIONAL, accurate | **repair** — rule **marked new** | Typed-layer until `Query` has a canonical encoding, which the clause already said. |
| **PS-26** | FROZEN, accurate | **repair** — rule **marked new** | Integration-level; CF-36's six. Owner HS-P0011. |
| **PS-27** | PROVISIONAL, accurate | **repair** ×2 — rule **marked new**, `Projection::on_error` qualified | — |
| **PS-28** | FROZEN, accurate | **gap recorded (undetermined) and routed**; rule **marked new** | Turns on what *"the last good position"* means. Resolved by defining the phrase when the rule is written. |
| **PS-29** | FROZEN, accurate | **gap recorded and routed**; rule **marked new** | *"Without being polled for it"* reaches past *"observable through the API"*. `independent`; ADR-0019 defers the observability design to HS-P0011. |
| **PS-30** | PROVISIONAL, accurate | **repair** — rule **marked new** | — |
| **PS-31** | FROZEN, accurate | **discharged** — and it was not before | Second conjunct now met at `crates/happenstance-core/src/projection.rs:56-71`. Verified against §2: VT-5 and VT-10 are what make the exclusion a consequence rather than an open question. Still no instrument; the PS-3/PS-31/PS-36 shape is routed as a candidate open question. |
| **PS-32** | FROZEN, accurate | **recorded as owed, deliberately not performed** | ADR-0007 is accepted and immutable, so the correction is a **superseding atom's** and never an edit. ADR-0017 already records it as owed (`references/adr/0017-…md:356-361`); staged for the next ingest wave. |
| **PS-33** | DEFERRED, accurate | accurate *(sweep: sound; the ownership hole is the clause's own finding)* | — |
| **PS-34** | PROVISIONAL, accurate | accurate | The `MUST` is **conditional** on PS-5 and the contingency is stated in the marker itself. §7.2 printing the rule unconditionally is a rendering fact, not a marker defect. |
| **PS-35** | FROZEN, accurate | accurate — **discharged by ADR-0008**, which the clause cites | — |
| **PS-36** | FROZEN, accurate | accurate *(sweep: sound; the `code: None` finding is the clause's own)* | Documentation half routed with PS-3 and PS-31. |
| **PS-37** | FROZEN, accurate | accurate *(sweep: sound; vacuous until a provided method exists, and the clause says who will find it)* | — |
| **PS-38** | PROVISIONAL, **new** | **minted by ADR-0030** | `spec/SPECIFICATION.md:5447`. Falsifier: a store answering `checkpoint` from a replica that may lag its own `commit`. |

**Ten check-4 failures, and none was closed by attaching a rule to a nearby
clause.** Flipping `has_suite` produced exactly the queue the spec's EC-003 warned
about. Three were parser artefacts (PS-10, PS-16, PS-27), seven were rules that
genuinely do not exist (PS-19, PS-25, PS-26, PS-27, PS-28, PS-29, PS-30) and are
now marked new, which is §1.4's own convention — *"a rule this document specifies
but which does not exist yet is marked as new"* — applied to the family that had
never followed it.

**Check 6 opened nothing.** Its `claimed` set is built from *all* clauses
regardless of `has_suite`, so the projection rules were already claimed when
slices 3–5 landed them. The two `UNCLAIMED_PENDING_ADR` entries are the
pre-existing event-store pair and are untouched.

## Gates

`cargo xtask ci`, run whole on this tree, **exit 0**. Every step, in order, with no
`skipped:` line anywhere in the log:

| Step | Tier | Outcome |
| --- | --- | --- |
| formatting · clippy (all targets, all features) · tests | REQUIRED | ran, green |
| each phase's proof artefacts | REQUIRED | ran, green |
| wasm32 build of the contract crate · wasm32 check of the conformance harnesses · wasm32 build of the Cloudflare adapter · wasm32 build of the Neon adapter | REQUIRED | ran, green |
| documentation · specification traceability · no retired rule is still live | REQUIRED | ran, green |
| the five lints + the D12 manifest lint | REQUIRED | ran, green |
| the Rust constitution is internally consistent · the constitution's examples compile | REQUIRED | ran, green — 27 atoms, all consistent |
| documentation (no default features) · documentation (default features) | REQUIRED | ran, green — **EC-001 did not fire**, because the crate-doc link was repaired in the same change as the gate |
| packaged artifacts carry their licences and README | REQUIRED | ran, green |
| **feature powerset** | OPTIONAL (`cargo hack --version`) | **ran**, green |
| **wasm32 feature powerset** | OPTIONAL (`cargo hack --version`) | **ran**, green |
| **licences and advisories** | OPTIONAL (`cargo deny --version`) | **ran**, green |
| **docs.rs configuration (nightly)** | OPTIONAL (`cargo +nightly --version`) | **ran**, green |

`redkiln validate --kb`: **passed**. `redkiln doctor`: the six standing
`template-drift` advisories and no seventh; the `foundation story … consumed by no
capability slice` advisories are pre-existing and untouched.
`redkiln verify --grain story` (HS-S0016): **pass** — affected-gate ok, ledger ok.

**NF-002 — the powerset cost, measured rather than assumed.** The workspace
powerset ran **62 combinations** (`cargo hack` prints `(n/62)`) and the wasm32
powerset its own set, both inside a whole-gate run that completed comfortably
within the tool's ten-minute ceiling on this machine. The widening is real and it
is paid; the slice-mate inherits a number rather than a surprise.

**Two pre-existing conditions observed and deliberately not repaired here.** A
`dead_code` warning on `MemoryProjectionBatch::read_through` fires in every
combination that compiles `projection_memory` without `conformance`; it predates
this change (thirteen occurrences in the powerset at the branch point, confirmed
by a stashed run) and is not `-D warnings` in the local gate. And the two
`UNCLAIMED_PENDING_ADR` entries are the event-store family's, untouched.

## Notes

### Phase 6's exit reconciliation (RUNBOOK.md:3810-3819), answered with figures

**Box 1 — every clause the phase's ADRs discharge, read against the code as it
now stands.** Done, as the thirty-eight-row table above. Two findings the read
produced that a summary would have missed: PS-31's documentation `MUST` was **not
met** and is now discharged, and §4.11's *"Every one is new"* was true when written
and false since slice 3 — sixteen of the seventeen exist.

**Box 2 — the phase's clause range against the union of its ADRs' ranges.**

| | Range |
| --- | --- |
| Phase 6's clause family | PS-1 – PS-37 (now PS-38) |
| ADR-0017 | PS-4 – PS-15 |
| ADR-0018 | PS-16 – PS-20 |
| ADR-0019 | PS-26 – PS-30 |
| **Union** | PS-4 – PS-20, PS-26 – PS-30 — **22 clauses** |
| **Disagreement** | **PS-1, PS-2, PS-3, PS-21 – PS-25, PS-31 – PS-38 — 16 clauses inside the phase's family that no phase-6 ADR's stated range contains** |

**Reported, not reconciled by widening an ADR.** ADR-0030 takes four of the
sixteen (PS-1, PS-19 — which *is* inside ADR-0018's range and was explicitly
deferred there — PS-21, PS-22). PS-2 and PS-3 are process clauses whose owner is
the phase itself and HS-P0016. PS-31 – PS-37 are port-definition clauses the
runbook assigns to this phase directly (`RUNBOOK.md:3929-3933`), two of which
(PS-31, PS-32) it names by hand. The residue is **PS-23, PS-24, PS-25 and PS-33**,
each `[PROVISIONAL]` or `[DEFERRED]` with a live falsifier and an owner named in
its own marker — none is orphaned, and none is claimed by a phase-6 ADR. That is
the disagreement, stated.

**Box 3 — `spec-trace`'s citation count has not fallen.**

| | Clauses | Rules | Cases | Citations |
| --- | --- | --- | --- | --- |
| Before (branch point) | 200 | 111 | 58 | **359** |
| After | 201 | 111 | 58 | **376** |

Seventeen citations gained, none lost. Three that had drifted — `projection.rs`
line ranges shifted by the module header — were re-anchored to their subjects
rather than deleted, and ten citations in `standards/rust/` were re-anchored for
the same reason after `lint-constitution` caught them.

### Deviations from the plan, and why

**The spec expected `RULE_FILES` to gain the projection rules file; it already
had it.** `xtask/src/spec_trace.rs:88-93` has named
`crates/happenstance-testkit/src/projection.rs` since the file was created. Only
`has_suite` was stale. Recorded rather than glossed: half of AC-005's mechanism was
already green, and the test for it is a ratchet rather than a repair.

**Two extra files were touched that the PR boundary did not enumerate, and both
are the mount rather than drift.** `crates/happenstance-core/tests/projection_memory.rs`
gains one `cfg` line — without it, `cargo test -p happenstance-core` on default
features stops compiling, and that file's own module doc says its whole purpose is
to prove the store is *mounted*. And `examples/outside-projection-adapter/` was
**not** touched, which is the point: `conformance` implying the gate is what kept
its one-flag manifest a one-flag manifest.

**PS-31 was discharged rather than merely dispositioned.** The spec's AC-006 asked
for PS-31 to be *"verified against §2's identity decisions as they now stand"*. It
verified as **not met**: `rg` over `projection.rs` found no mention of the
exclusion. Discharging it is a doc-comment addition inside NF-001's stated
boundary, it is what `RUNBOOK.md:3932-3934` assigns to this phase in as many
words, and leaving a `[FROZEN]` `MUST` unmet while writing a report that says every
clause was read would have been the exact failure the disposition pass exists to
catch.

**Ten check-4 failures were absorbed inside this story rather than routed.** EC-003
permits either. None was closed by attaching a rule to a nearby clause; three were
category corrections to the `Rule:` field (the `MUST` untouched, so a repair by the
classifier) and seven were §1.4's own *marked as new* convention applied to a family
that had never used it.

### What is deliberately absent

No conformance rule, mutant, fixture or `Capability` was added. No port signature
moved. No `.kb/` atom was hand-written and no accepted atom's body was edited —
`git diff --stat` shows nothing under `.kb/decisions/` and exactly one addition
under `.kb/_intake/`. No sentence in this diff says the freeze held (HS-P0015's) or
that 0.1 ships behind the feature (HS-P0016's, at phase 12); both the module header
and PS-3's discharge route those explicitly. No pass rate over the mutant set
appears anywhere, per ADR-0010.
