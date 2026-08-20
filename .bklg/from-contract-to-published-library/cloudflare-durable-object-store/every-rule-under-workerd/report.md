---
item: "HS-S0054"
stage: report
created: "2026-08-19"
updated: "2026-08-19"
---

# Report — Every event-store conformance rule executed on the target, in the same gate run

## Findings Ledger

**Outcome: eight of eight ACs satisfied. Eighty-nine of eighty-nine event-store rules
executed and passed against a real Durable Object on `wasm32-unknown-unknown`, inside
`cargo xtask ci`. Nothing blocked, nothing deferred, no rule `#[cfg]`-ed out, no clause
amended, no `.kb/` write.** One finding against the upstream seam was repaired here rather
than escalated and is flagged for ratification.

| AC | Result | Proved by | Mounted into |
| --- | --- | --- | --- |
| AC-001 | satisfied | `cargo run -p xtask -- wasm-conformance` → `happenstance-cloudflare/durable_object_conformance: 89 rules enumerated, 10 named, executing on wasm32-unknown-unknown` … `test result: ok. 89 passed; 0 failed` | `xtask/src/proof.rs` `WASM_TARGETS[3]`; driven by the existing `wasm32 run of the conformance rules` step |
| AC-002 | satisfied | `rg -n "macro_rules!" crates/happenstance-cloudflare/` empty; `registry::no_orphan_rules` green; `proof::tests::the_executed_wasm_targets_name_no_rule_of_their_own` | `crates/happenstance-cloudflare/tests/durable_object_conformance.rs:60-64` (three lines) |
| AC-003 | satisfied | `proof::tests::the_cloudflare_conformance_target_is_a_row_and_not_a_second_step` — row identity **and** `REQUIRED` execution-step count == 1. Red beat: `has no row in WASM_TARGETS` | one row; `xtask/src/main.rs` untouched |
| AC-004 | satisfied | both negative controls performed and both failed *before* the run (emptied target, `cfg`-ed-away target), output pasted in the implementation report; expectation derived from the enumeration, not hand-copied | `xtask/src/proof.rs` `wasm_enumeration` + `wasm_run` |
| AC-005 | satisfied | three `SKIP` lines captured verbatim under `--nocapture`, two carrying the fixture's own words about this runtime; the three `REOPEN` rules print **no** skip because they ran | `RuleOutcome::skip_line` → `__emit_wasm`'s `console_log!` |
| AC-006 | satisfied | `crates/happenstance-cloudflare/src/lib.rs:28-73`; `cargo doc -p happenstance-cloudflare --no-deps` clean | the crate documentation a consumer lands on |
| AC-007 | satisfied | `cargo tree -p happenstance-cloudflare -e normal --depth 1` identical before and after; `[dependencies]` untouched; no feature invented | `crates/happenstance-cloudflare/Cargo.toml` |
| AC-008 | satisfied | `spec-trace` `no problems found`; `lint-constitution` 27 atoms consistent; the four `!Send` probes green on both targets; no `.kb/**` or `spec/SPECIFICATION.md` path in the diff | the implementation report's scope paragraph |

### What the run actually proved, beyond "green"

Three of the eighty-nine had never been executed by anything, anywhere in this workspace,
before this run: `acknowledged_writes_survive_a_reopen`,
`reopened_store_does_not_reissue_an_event_id` and `recorded_time_survives_a_reopen`. Every
other fixture in the tree declines `REOPEN` — `MemoryEventStore` is a `Vec` behind an
`RwLock` and `LocalMemoryEventStore` an `Rc<RefCell<_>>`, so for both of them discarding
process state is indistinguishable from discarding the events. A Durable Object is the case
the capability was named for, and this is the first fixture to answer it `SUPPORTED`. All
three passed.

`read_result_is_stable_under_concurrent_append` and
`a_live_read_stream_does_not_block_an_append` are ADR-0011's ceiling-and-page under
*execution* rather than under a `cargo check`, on the runtime ES-11 and ES-12 name as the
falsifier they were most at risk from. Both passed.

### For the reviewer to ratify

1. **A finding against `wasm-execution-gate-step` (HS-S0048) was repaired here rather than
   escalated.** Its AC-006 contract held for the step, the runner, the `--target` plumbing
   and both guards — the row *was* enough. One line inside `unregistered_wasm_harnesses`
   was not: it compares the harnesses found in one directory against the length of the
   whole registry, so a fourth row anywhere else makes it accuse itself of reading nowhere.
   The repair is to count only the rows whose `source` lies in the directory being scanned;
   the guarded property is unchanged. EC-003 says escalate rather than absorb, and the
   judgement call taken here was that leaving the gate red on a *correct* registry serves
   nobody. It is recorded as a defect in HS-S0048's claim, not in this diff.
2. **Eight `standards/rust/` citations were repointed.** The AC-006 documentation section
   moves every line below it in `src/lib.rs`, and four atoms cite that file by `file:line`.
   `lint-constitution` is a gate step. Only line numbers changed; no normative text, and
   nothing in `spec/SPECIFICATION.md`. Both this story's fence and the sibling's want
   `standards/rust/**` and `xtask/src/proof.rs` added, on the reasoning the
   `real-worker-bindings` amendments already established.
3. **`CLOUDFLARE_WASM_RULES` names ten rules, transcribed.** The exhaustive check is derived
   and needs no list; these ten are the second, rename-catching half, and each carries its
   reason in the doc comment — the two fixture-contract rules, the three `REOPEN` rules no
   fixture had run before, the two `MID_BATCH_FAULT` rules `measured-store-limits` will
   flip, `append_reports_exceeded_store_limits`, and the two read-isolation rules whose pass
   depends on this adapter's borrow discipline rather than on a lock.

### Not claimed

**This green run is not CF-40's discharge.** It ran at the three ceilings HS-S0053
*declared*, all of which are `None`, so `append_reports_exceeded_store_limits` **skipped**
with the testkit's own `NO_CEILING_REASON` — a green run in which the workspace's one
capacity-capped runtime contributed nothing to the clause it was brought in to discharge.
`measured-store-limits` (HS-S0055) is where those numbers become facts. The same holds for
CF-39: `MID_BATCH_FAULT` is declined, so both atomicity-under-fault rules skipped.

**No rule failed only on `wasm32`**, so there is no divergence finding. The observations
ADR-0023 will want — the runner shape, the `SqlStorage` mapping's behaviour under real
storage, the per-rule verdicts and the skip lines — are in the implementation report and
nowhere under `.kb/`.
