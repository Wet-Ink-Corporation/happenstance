---
item: "HS-S0007"
stage: report
created: "2026-08-14"
updated: "2026-08-14"
---

# Report — projection_store_conformance!, one enumeration, one test per rule

## Findings Ledger

**Ten of ten ACs satisfied, each by real reachable behaviour with a real test.**
Four tests across two runtimes over the same enumeration, two meta-tests in the
library itself, three harness files, and both of the steps the spec singled out
as the ones a story-grain gate could hide — run explicitly and cited.

**The one claim this story does not make.** The projection suite is not shown to
discriminate. `MemoryProjectionStore` is the oracle; it is supposed to pass. The
store that must fail `commit_is_atomic_with_the_read_model` —
`CheckpointOnlyStore` — is `projection-mutant-registry`'s (HS-S0008), which
names this story in its `depends_on`. Both rules carry that debt in their own
rustdoc, with the discharging story named, which is what keeps it a bounded debt
rather than an exemption (ADR-0010).

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — one line, one test per rule, named after the rule | **Met** | `crates/happenstance-testkit/src/lib.rs:439-478` (three arms in `event_store_conformance!`'s order; `__conformance_fixture` spelled identically so a caller-supplied emitter drives any family) and `:484` (`__private`). Reachability is proven by compilation: `tests/projection_conformance.rs` is one `use` and one macro call, and its RED run was `error[E0433]: cannot find `projection_store_conformance``. `-- --list` prints exactly the two rule names. |
| **AC-002** — an owned fixture that makes isolated stores | **Met** | `crates/happenstance-testkit/src/contract.rs:443-468`: `type Store: ProjectionProbe` (the probe, not the port — so an unobservable adapter cannot invoke the suite), no GAT, bare flavour, desugared `connect` that panics rather than returning `Result`. `projection::two_opens_make_two_isolated_stores` commits to one store and asserts the other reports `NeverRun` with no probe row, behind a control that the commit landed. |
| **AC-003** — exactly one place the rule set is written | **Met** | `crates/happenstance-testkit/src/projection.rs:266-281`. `rg -n "commit_advances_the_checkpoint" crates/happenstance-testkit` returns hits in `src/projection.rs` **only**; all three harnesses contain zero rule identifiers and one macro invocation each. No `const RULES: &[&str]`, no crate-doc list treated as authority. |
| **AC-004** — the orphan meta-test can fail, and did | **Met** | `projection::no_orphan_projection_rules` (`src/projection.rs:396-426`), both directions, fail-loud on an empty scan. Demonstrated: an unregistered `pub async fn` made it fail **by name** (`… absent from `for_each_projection_store_rule!`, so no harness runs them: ["deliberately_unregistered_rule"]`), then reverted and re-run green. Transcript in `implementation-report.md`. |
| **AC-005** — `commit_advances_the_checkpoint` | **Met, as a passing rule against the oracle** | `src/projection.rs:152-193`: `begin` → `probe_write` → `commit` at a position held in a binding → `checkpoint` read back through a **fresh handle** and compared against that binding. No literal position (the CF-6 lint now sweeps this file), no clock (CF-33 green). Rustdoc names the defect it rejects — a `commit` returning `Ok` that makes neither write durable — and cites the PS-1 open question as owned elsewhere rather than settling it. |
| **AC-006** — `commit_is_atomic_with_the_read_model` | **Met, as a passing rule against the oracle** | `src/projection.rs:207-256`: the probe row and the checkpoint, both through fresh handles, asserted **both or neither and nothing else**, so a failure can only be the coupling. Read back through `ProjectionProbe` alone, never an inherent method. Rustdoc names `CheckpointOnlyStore` and HS-S0008. |
| **AC-007** — a reference fixture that is documentation | **Met** | `src/fixtures.rs:424-444` — a published item with rustdoc, beside `MemoryFixture`, `core::future::ready` rather than `async move`, plus the `MemoryProjectionHandle` newtype (`:313-386`) an adapter author needs to see to copy the shape. Named from outside the crate by the tokio harness, which compiles only because it is public. |
| **AC-008** — three harnesses, three targets, no new gate step | **Met** | The three files exist and run one enumeration. `cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown` green, run explicitly *and* inside the gate. The wasm emitter routes `skip_line` → `console_log!` and never `report`, which no compiler can check and the review is the instrument for. |
| **AC-009** — the front page describes four families | **Met** | `src/lib.rs:91` ("there are four"), `:93-108` (which line to write, which harness, what to implement first), `:175` (the table row), `:138-143` (why this family's names may be links and the other two's may not — the module is unconditional). `README.md:99-127`, and `:12-17` stops claiming the port has no suite. All three doc builds green under `-D warnings`. |
| **AC-010** — `RULE_FILES`, CF-29, and only the generated spec hunk | **Met** | `xtask/src/spec_trace.rs:88-93` — four entries, doc restated. CF-29 prints `all 97 rules in 4 file(s) have a changelog entry`; the two rules take an entry each, both naming their defect. `cargo xtask spec-trace` green with check 6 accepting both as claimed by PS-1's own `Rule:` field — verified, not assumed. `git diff spec/SPECIFICATION.md` is 4/4 lines, all inside §7.2's generated region. |

### Deferred, and to whom

| Thing | Owner | Why it is not here |
| --- | --- | --- |
| `CheckpointOnlyStore`, `TruncatingResetStore`, `ValidatingCommitStore`, the projection mutant registry and its exactness meta-tests | `projection-mutant-registry` (HS-S0008) | This story's rules document the defects they reject; the stores that embody them are the next story's deliverable, and both rules name it. |
| Capability constants on `ProjectionFixture`, `require!`/`must!` gates, `RuleOutcome::Skipped` assertions | `projection-capability-skips` (HS-S0009) | The two baseline rules gate on nothing, and a capability no rule reads is decorative. Implemented immediately after this story, in the same slice and the same context. |
| The other fifteen §4.11 rules | slices 4–8 | Each is registered in the enumeration this story created and emitted by the three emitters it wrote — no new mechanism. |
| The `unstable-projection` gate, `projection.rs`'s provisional header, widening `has_suite` to `PS-` | `unstable-projection-gate-and-clause-disposition` (HS-S0016) | Out of boundary by name; no maturity marker was moved and nothing `[FROZEN]` was line-edited. |

### Findings raised, not fixed

1. **Two of four rule families have no orphan meta-test.** `no_orphan_rules`
   covers `suite.rs` and `no_orphan_projection_rules` covers `projection.rs`;
   `model.rs` and `concurrency.rs` have none. CF-24's coverage across all four
   is a deliberate decision with its own reasoning, not a side effect of this
   story. Out of boundary; reported.

2. **`projection.rs` is now an ambiguous basename in this workspace.** Thirteen
   `SPECIFICATION.md` citations that resolved uniquely for phases stopped doing
   so the moment the fourth rule file landed. Resolved the way `spec_trace.rs`
   documents — one `BARE_NAME_MAP` entry with the evidence — rather than by
   renaming the module away from the name the spec settled. Any future citation
   meaning the *testkit's* file must be written with its path.

3. **The spec's claim that `cargo xtask ci --fast` omits `spec-trace` is out of
   date.** The step is in `REQUIRED` (`xtask/src/main.rs:315`), and `run_fast`
   drops `OPTIONAL` only. Two of this story's ACs were said to be invisible at
   story grain for that reason; they are not. Both were run explicitly anyway.

4. **23 `standards/rust/` citations were repaired** because inserting a trait
   into `contract.rs` moved every line below it past the lint's ten-line slack.
   `lint-constitution --write` fixes the router but not citations. Worth a tool
   improvement someone should decide on; recorded here rather than absorbed
   silently.
