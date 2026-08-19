---
item: HS-S0055
stage: implement
created: 2026-08-12
updated: 2026-08-12
---

# Acceptance ledger — The fixture's numeric limits are measurements, not guesses

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "The payload ceiling is a fact about this adapter. GIVEN a constrained-runtime developer who must know, before modelling a write path on the edge, how large an event this store will actually take — and who today can only read a platform blog post, WHEN they open `impl Fixture for CloudflareFixture` and this crate's documentation, THEN `MAX_EVENT_DATA_LEN` is `Some(N)` where `N` was located by driving this adapter's own `append` through `CloudflareEventStore::new(sql)` against a real Durable Object — never read off a platform page — carrying a doc comment naming what bounds it and pointing at the recorded derivation; and the run proves the promise in both directions: an `N`-byte payload accepted and readable back, an `N + 1`-byte payload refused as `AppendError::ExceedsStoreLimit` naming `StoreLimit::EventDataLen` (never `AppendError::Store`, never by truncation), with nothing of the refused event left in the log"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs — the `MAX_EVENT_DATA_LEN` associated constant in the `impl Fixture for CloudflareFixture` block"
  verifying_test: "crates/happenstance-cloudflare/tests/durable_object_conformance.rs — dcb_conformance_wasm::append_reports_exceeded_store_limits (payload arm, crates/happenstance-testkit/src/suite.rs:4330-4375), executed by the wasm32 step of `cargo xtask ci`"

- id: AC-002
  criterion: "The tag ceiling is a fact about this adapter's tag storage, not about SQLite in general. GIVEN an adapter author on Learn when you are finished who is modelling a boundary with many tags and needs to know where this store stops accepting them, WHEN they read the fixture, THEN `MAX_TAGS_PER_EVENT` is `Some(N)` measured against the tags table this adapter actually renders — including whatever `durable-object-write-path` chose for tag storage — and the run accepts an event with exactly `N` tags and refuses one with `N + 1` as `ExceedsStoreLimit` naming `StoreLimit::TagsPerEvent`, leaving nothing of the refused event behind"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs — the `MAX_TAGS_PER_EVENT` associated constant in the `impl Fixture for CloudflareFixture` block"
  verifying_test: "crates/happenstance-cloudflare/tests/durable_object_conformance.rs — dcb_conformance_wasm::append_reports_exceeded_store_limits (tag arm, crates/happenstance-testkit/src/suite.rs:4378-4434)"

- id: AC-003
  criterion: "The batch ceiling is a fact about the statement this adapter renders. GIVEN the same author appending a multi-event batch atomically at the edge, WHEN they read the fixture, THEN `MAX_EVENTS_PER_BATCH` is `Some(N)` measured against the parameter count this adapter's batch statement actually renders against the real bindings, and the run accepts a batch of exactly `N` events and refuses `N + 1` as `ExceedsStoreLimit` naming `StoreLimit::EventsPerBatch` — refused whole, with no partial chunk written"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs — the `MAX_EVENTS_PER_BATCH` associated constant in the `impl Fixture for CloudflareFixture` block"
  verifying_test: "crates/happenstance-cloudflare/tests/durable_object_conformance.rs — dcb_conformance_wasm::append_reports_exceeded_store_limits (batch arm, crates/happenstance-testkit/src/suite.rs:4437-4480)"

- id: AC-004
  criterion: "The gate reader stops seeing an honest-looking skip where the workspace's one capacity-capped runtime should be reporting a boundary. GIVEN a gate reader at activity D reading the wasm32 step's output in the same terminal scroll as the rest of `cargo xtask ci`, and GIVEN that with all three ceilings at their `None` defaults the rule reports `Skipped { capability: NO_STORE_LIMITS }` with the testkit's own `NO_CEILING_REASON` and certifies nothing, WHEN they read the run after this story merges, THEN `append_reports_exceeded_store_limits` reports Ran for `CloudflareFixture` and no `NO_STORE_LIMITS` skip line for it appears anywhere in the output — the absence of that line being the observable this story is judged by"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs — the three ceiling constants in the `impl Fixture for CloudflareFixture` block, read by monomorphised suite code at run time"
  verifying_test: "the wasm32 gate step's per-rule output for dcb_conformance_wasm::append_reports_exceeded_store_limits, captured into the implementation report; plus crates/happenstance-cloudflare/tests/fixture_contract.rs::the_three_store_limits_are_measured_not_defaulted"

- id: AC-005
  criterion: "No ceiling is declared under a floor, and a sub-floor measurement stops the story rather than shrinking the promise. GIVEN the four guaranteed minima every store must clear — 65,536 payload bytes, 64 tags, 128 events per batch (crates/happenstance-core/src/limits.rs:22, :27, :43) — and GIVEN that the trait explicitly permits a fixture to state a ceiling below a floor and then fail the corresponding rule (crates/happenstance-testkit/src/contract.rs:249-252), WHEN each measured value is compared against its floor before being written into the impl, THEN every declared ceiling is greater than or equal to its floor, the three guaranteed-minimum rules stay green in the same run, and any measurement that came in under a floor was escalated as a blocking finding to phase 9 and ADR-0023 rather than declared as a smaller number"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs — the three ceiling constants in the `impl Fixture for CloudflareFixture` block"
  verifying_test: "crates/happenstance-cloudflare/tests/fixture_contract.rs::no_declared_ceiling_is_below_its_floor; plus dcb_conformance_wasm::store_accepts_the_guaranteed_minimum_payload, ::store_accepts_the_guaranteed_minimum_tag_count and ::store_accepts_the_guaranteed_minimum_batch_size green in the same run"

- id: AC-006
  criterion: "The next person can re-derive the numbers without asking anyone, and the gate never re-derives them. GIVEN an adapter author or a reviewer six months from now who wants to know whether `MAX_EVENT_DATA_LEN` is still true after a schema change, WHEN they open `experiments/durable-object-limits/README.md` and run its runner, THEN they get the same three boundaries by the same method, with the environment and runner version recorded beside the results — and none of it runs inside `cargo xtask ci`, because what the gate checks on every invocation is the declared promise, not a search whose outcome could drift"
  satisfied: false
  evidence: ""
  mount_point: "experiments/durable-object-limits/ — README.md, runner and results/, deliberately outside the gate; the numbers it produces mount at crates/happenstance-cloudflare/src/lib.rs's fixture impl"
  verifying_test: "the runner at experiments/durable-object-limits/ executed at least twice with both result sets recorded under experiments/durable-object-limits/results/ and compared in the implementation report; `cargo xtask ci --fast` showing no new step"

- id: AC-007
  criterion: "A cap that is not a constant is reported as a falsification, not smuggled in as a number. GIVEN that `SqlError::StorageLimitExceeded` (crates/happenstance-cloudflare/src/sql_storage.rs:119-121) is the object's cumulative SQL storage limit while `MAX_EVENT_DATA_LEN` is a per-value boundary the rule probes against a store earlier rules have already written to, and GIVEN that CF-40's own PROVISIONAL falsifier is a real adapter whose ceiling is not a constant (spec/SPECIFICATION.md:7676-7684), WHEN a limit's only real cap turns out to depend on what is already stored, THEN that limit is declared `None` and a written falsification finding is produced naming CF-40's falsifier as having fired on one of its three named instruments — never a number that happens to pass on one ordering of the suite"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs — the affected ceiling constant in the `impl Fixture for CloudflareFixture` block; the finding lands in this story's evidence package"
  verifying_test: "the boundary search re-run against a pre-loaded and a fresh fixture instance, both recorded under experiments/durable-object-limits/results/, with the two-run stability check; the finding written into .bklg/from-contract-to-published-library/cloudflare-durable-object-store/measured-store-limits/_evidence.md"

- id: AC-008
  criterion: "MID_BATCH_FAULT is either a real armed seam or an honest decline, and never a claim with an empty arm. GIVEN CF-39 PROVISIONAL (spec/SPECIFICATION.md:7631-7660), which requires a fixture declaring the capability supported to make the k-th write fail inside the store's own write path by a mechanism the store cannot absorb and to state that mechanism, and GIVEN the registered wrong implementation `NoopFaultFixture` — supported, `arm_mid_batch_fault` overridden by an empty body, reporting a green atomicity result for a store that was never faulted, WHEN the verdict is settled against the real runner, THEN either `MID_BATCH_FAULT` is SUPPORTED with `arm_mid_batch_fault` overridden by a real unabsorbable seam whose mechanism is stated in the impl and the two fault rules Ran and passed — making this the first adapter in the workspace to exercise CF-39 for real — or it is declined in this runtime's own words naming why the store absorbs every fault this host can arm, with those two rules skipping on that stated reason"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs — the `MID_BATCH_FAULT` associated constant and, if supported, the `arm_mid_batch_fault` override in the `impl Fixture for CloudflareFixture` block"
  verifying_test: "dcb_conformance_wasm::arming_a_mid_batch_fault_makes_the_append_fail and ::append_is_atomic_under_a_mid_batch_fault (crates/happenstance-testkit/src/suite.rs:2794, :2706); crates/happenstance-cloudflare/tests/fixture_contract.rs::mid_batch_fault_is_restated_not_inherited and ::a_supported_capability_has_its_method_overridden; the negative control (arm removed, atomicity rule observed red) pasted into the report if supported"

- id: AC-009
  criterion: "REOPEN survived contact with the real runner, or was overturned on evidence. GIVEN HS-S0053 declared `REOPEN` from the trait's own Durable-Object reasoning (storage outlives the isolate, so handle state can be discarded and the store read again; the isolate cannot be restarted from inside a test — crates/happenstance-testkit/src/contract.rs:163-173), WHEN the suite runs against the real host, THEN either the three reopen rules Ran and passed, or the constant is changed — a one-line change by HS-S0053's design — to a decline whose reason names why this runner cannot discard handle state, and those three rules skip on that reason rather than on a sentence written for another store"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs — the `REOPEN` associated constant (and its `reopen` override, if supported) in the `impl Fixture for CloudflareFixture` block"
  verifying_test: "dcb_conformance_wasm::acknowledged_writes_survive_a_reopen, ::reopened_store_does_not_reissue_an_event_id and ::recorded_time_survives_a_reopen (crates/happenstance-testkit/src/suite.rs:332, :2310, :2473); crates/happenstance-cloudflare/tests/fixture_contract.rs::every_declined_capability_names_this_runtime"

- id: AC-010
  criterion: "A reader can tell a measurement from an omission without running anything. GIVEN that no store can fail a rule whose defect is in the fixture's declaration and that CLAUDE.md forbids writing a conformance rule nothing can fail, so the `None` defect gets a review obligation instead, WHEN a reviewer reads the diff and a consumer reads the crate's docs, THEN CHANGELOG.md carries a CF-29-shaped entry naming the defect each newly-reachable rule now detects — a sentence, not a listing — and the crate documentation states the three declared limits and what bounds each, discharging VT-21's obligation that a store documents its actual limit"
  satisfied: false
  evidence: ""
  mount_point: "CHANGELOG.md (the CF-29 entry) and crates/happenstance-cloudflare/src/lib.rs (the crate-level and per-constant documentation rendered by `cargo doc`)"
  verifying_test: "`cargo xtask lint-changelog` (xtask/src/lints.rs:504-593) green with the new entry read at review for the sentence-not-listing bar; `cargo doc -p happenstance-cloudflare` output read at review; `cargo xtask spec-trace` green"

- id: AC-011
  criterion: "The evidence leaves this story complete, and no .kb/ file is written here. GIVEN that accepted decision atoms are immutable and `redkiln validate --kb` checks them against HEAD, and that every KB write in this project belongs to `adr-0023-and-atom-resolutions` (HS-S0058), WHEN this story closes, THEN its folder carries an evidence package containing the three numbers with the derivation of each, the CF-39 verdict and its mechanism or decline reason, the REOPEN verdict, the CF-40 ownership finding (that declaring numbers was not blocked by the ownership contradiction, because the clause text is identical under either reading — narrowing the atom to its own sub-question 2), the coordination check against `sqlite-durable-store` so CF-40 is minted once, any falsification finding from AC-007, and the stale PROVISIONAL marker sentences quoted verbatim as a residual — and `git status` shows no file under .kb/ touched by this PR"
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/cloudflare-durable-object-store/measured-store-limits/_evidence.md — the hand-off artefact HS-S0058 consumes; no mount in .kb/ by construction"
  verifying_test: "`git diff --name-only` over the PR showing no .kb/** path; `redkiln validate --kb && redkiln doctor` green; the evidence package reviewed against the six items AC-011 enumerates"
```
