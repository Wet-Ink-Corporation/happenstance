# Wave `2026-09-07-intake` — claims and classification

Every claim the fifty-seven staged files carry, labelled against the **accepted decision corpus**
— the thirty-three `kind: decision, status: accepted` atoms in `.kb/decisions/`, read rather than
inferred from titles.

The four labels:

- **aligns** — the claim is already true under an accepted decision, and adds no obligation.
- **extends** — the claim adds evidence, scope or consequence to something accepted, without
  contradicting it. Most of this wave.
- **conflicts** — the claim contradicts an accepted decision or the tree. Three claims, and none
  of them is resolved by this wave.
- **requires-new-decision** — the claim is a commitment nothing accepted covers.

**The wave's shape in one line:** twenty-three `requires-new-decision`, three `conflicts`, and
everything else `extends`. That ratio is what a release pass produces — a ratification is by
construction a set of new commitments, and a discharge report is by construction evidence about
them.

---

## A. The release-pass documents (7 files)

| # | Claim | Source file(s) | Label | Against |
| --- | --- | --- | --- | --- |
| A1 | `deny.toml`'s async-trait ban gains `testcontainers` and `tonic` as wrappers | adr-0038 | **requires-new-decision** | amends `kb-decision-0001`, `kb-decision-0035` from outside |
| A2 | Three separate arguments sit behind one wrappers list and must not collapse | adr-0038 | extends | body of A1 |
| A3 | `tonic` as a wrapper admits async-trait from anywhere in the graph — the one real weakening | adr-0038 | extends | body of A1 |
| A4 | ADR-0038 decides neither ADR-0024's mechanism, nor a live-infrastructure gate step, nor `cargo deny`'s probed-optional status | adr-0038 | extends | `kb-decision-0035` names the probed-optional gap already |
| A5 | `happenstance-postgres` adopts `xid8` + `pg_snapshot_xmin` | adr-0024 | **requires-new-decision** | rests on `kb-decision-0013`, which left the mechanism open |
| A6 | The four sub-questions: sqlx compromises, CF-13's pass is not evidence, the staleness bound, B-tag not reconsidered | adr-0024 | extends | body of A5 |
| A7 | Steady-state cost within noise; staleness 0.593 ms control vs 4799.3 ms behind a 5 s hold | adr-0024 | extends | supersedes phase-2 figures *for adapter claims only* |
| A8 | No new spec clause for the three consumer-facing consequences (CF-4) | adr-0024 | extends | body of A5 |
| A9 | The global-vs-per-boundary premise is **inherited**, not settled, by ADR-0024 | adr-0024 | extends | `kb-open-question-global-vs-boundary-visibility-001` |
| A10 | Poll count calibrated at n=3 against live PostgreSQL; the rule's schedule changes, ES-10 does not | poll-count | extends | `kb-decision-0013` predicted exactly this |
| A11 | `PollPaddedPositionStore` passed the old fixed A,B,B,A schedule — the blind spot was real | poll-count | extends | `kb-open-question-poll-count-rule-strength-001` |
| A12 | An adapter advancing **off-poll** cannot be reached by any poll-based schedule | poll-count, adr-0024 | extends | genuinely new; no accepted decision covers it |
| A13 | ES-42's `[PROVISIONAL]` marker is earned off to `[FROZEN]` at 0.2.0 | es-42 | **requires-new-decision** | evidence is `kb-decision-0011`'s E11 wrapper |
| A14 | CF-25's portfolio qualification does not gate the ES-42 freeze — its rule is compile-level | es-42 | extends | body of A13; links the CF-25 question |
| A15 | `0.2.0` waits for phase 10 (record F2-5-HOLD), and the hold is lifted the same day | f2-5 | **requires-new-decision** | no accepted decision covers release gating |
| A16 | F2-5's actionable half is false: ES-22 arm 2 is reached by two registered stores on every run | f2-5, es-22-arm-two | extends | `kb-decision-0010`'s proof obligation |
| A17 | The residual: no store with a **real medium** had answered the rule — discharged by a live trigger | f2-5, es-22-arm-two | extends | body of A15 |
| A18 | Whether CF-5's conformant-control obligation runs per rule or per branch | es-22-arm-two, f2-5 | extends | `kb-decision-0010` mints CF-5 and does not say |
| A19 | Seventeen briefs ratified at the release pass; nine on their own recommendation, eight individually | ratifications | extends | the method, recorded once |
| A20 | The README's index classifies 41 of 49 — not a completeness proof | ratifications | extends | transferable, no accepted owner |
| A21 | Six of the seventeen obliged code; the queue is now empty | discharge | extends | closes A19's own stated gap |
| A22 | `cf-18`/B3's literal wording described a state the type system makes unreachable | discharge, cf-18 brief | **requires-new-decision** | CF-18 stands; what landed is declension by inheritance |
| A23 | `PostgresFixture` inherited a `READ_FAULT` declension that is false about its store | discharge | extends | the gap A22's rule found |
| A24 | `Codec::reads_tag` is per-codec by the orphan rule — narrower than the brief implied | discharge, codec brief | **requires-new-decision** | additive to `kb-decision-0032`'s lineage |
| A25 | Narrowing `StringifiedThrow` made `dead_code` fire on two methods with no caller | discharge, stringified brief | **requires-new-decision** | `kb-decision-0023` names the type incidentally |
| A26 | An ambiguous citation anchor is already broken; only a line number hides it | discharge + 8 briefs | extends | `kb-playbook-anchoring-citations-001` |
| A27 | A count in a document nobody re-reads is a claim, not a fact | discharge, ratifications | extends | no accepted owner; transferable |

## B. The ratified briefs (17, plus the 3 already landed)

Each row is the brief's `## Recommendation` as ratified. **Landed** is from the discharge table.

| # | Brief | Ratified as | Landed | Label |
| --- | --- | --- | --- | --- |
| B1 | `fixture-declension-policy` | Option A — every future fixture capability lands defaulted | yes, on `lane/fixture-declension` | **requires-new-decision** |
| B2 | `event-metadata-floor` | Option 2 — `StoreLimit::MetadataLen`, `guaranteed_minimum() == 0` | not in the queue | **requires-new-decision** (amends `kb-decision-0015`) |
| B3 | `sqlite-blocking-seam` | W1 now; W2/W3 unripe; H2 recommended | W1 landed | **requires-new-decision** |
| B4 | `domain-event-guard-and-decode` | A + E now, B before 0.2.0, C as fallback | not in the queue | **requires-new-decision** |
| B5 | `after-opt-scope` | Option 1 + Option 3 — pin the behaviour, add a scope-carrying name | not in the queue | **requires-new-decision** |
| B6 | `append-batch-ownership` | Option B — do not change ownership; correct four doc sites; restate ES-17 | not in the queue | **requires-new-decision** |
| B7 | `msrv-premise` | Option B — lower the single workspace MSRV to 1.95 | **no; tree still 1.97.1** | **conflicts** with `kb-decision-0037` |
| B8 | `wf-10-instruments-narrower-than-the-clause` | Q1 two `decode_rejects_*` tests; Q2 Option A | not in the queue | **requires-new-decision** |
| B9 | `testkit-dev-dependency-version-requirement` | B then C, neither urgent | not in the queue | **requires-new-decision** |
| B10 | `empty-decision-outcome` | Option 3 — two success shapes | `0ae10ab` | **requires-new-decision** |
| B11 | `op-read-non-exhaustive` | Option A — apply now, with `to` | `9000f35` | **requires-new-decision** |
| B12 | `tags-scope-agreement` | A + B; C-additive held | `0ae10ab` | **requires-new-decision** |
| B13 | `query-partition-public-surface` | 1A — both constants stay public | already landed | **requires-new-decision** |
| B14 | `read-page-budget-rows-bytes-or-caller` | A for 0.2.0; B open | already landed | **requires-new-decision** |
| B15 | `codec-foreign-tag-resolution` | A now; C held open | `2d37fc4`, narrower | **requires-new-decision** |
| B16 | `stringified-throw-visibility` | Option B — `pub(crate)` | `9225c00`, plus dead code | **requires-new-decision** |
| B17 | `cf-18-observable-skip-reporting` | Cost B3 before publication | `3df4b6e`, **differently** | **requires-new-decision** |
| B18 | `adapter-driver-reexport-policy` | D-1/D-4 — a published crate re-exports any crate in a public signature | ratified 2026-09-04, landed | **requires-new-decision** |
| B19 | `citation-anchor-slack` | GATE-02 Option C — exact anchor match, no slack | ratified 2026-09-04, landed | **requires-new-decision** |
| B20 | `repository-url-and-security-channel` | PUB-01 — Option B then Option A | ratified 2026-09-04, landed | **requires-new-decision** |

### The three `conflicts`, stated precisely

1. **B7 / `msrv-premise` against `kb-decision-0037`.** The decision is titled *"The MSRV becomes
   a promise at 0.2.0, and the number does not move"* and its summary states the floor was forced
   by *"a dependency's build script rather than by this workspace's code — rusqlite 0.40 pulls
   libsqlite3-sys 0.38.1, whose build script invokes `cfg_select!`, unavailable before 1.88"*.
   The brief refutes both halves with a bisection transcript: `cfg_select!` stabilised in **1.95**,
   not 1.88, and the raise is attributable to the workspace's own let-chains. It then recommends
   lowering the floor to 1.95. **Three things are true at once**: the ratification says the
   recommendation is the decision; the tree still reads `1.97.1` in both `Cargo.toml` and
   `rust-toolchain.toml`; and ADR-0037 is accepted and immutable. Nothing in the intake resolves
   that, so nothing here does either.
2. **`kb-decision-0037`'s and `kb-decision-0029`'s `cfg_select!` attribution.** A factual error
   inside two accepted decision bodies, off by seven releases. It cannot be edited and it cannot
   be silently carried. Same destination as (1).
3. **CF-34's maturity marker.** `benchmark-completion-and-the-merge-red-promise.md` and
   `sqlite-blocking-seam.md` both cite CF-34 as `[PROVISIONAL]`;
   `timed-assertions-outside-the-testkit.md` cites it as `[FROZEN]`. One of the three is wrong and
   the intake does not say which. Carried to `unresolved[]`.

## C. The unratified briefs — findings, gaps and deferrals (30 files)

Every claim below is **extends**: it adds a measured gap, a residual, or an unresolved fork to
something accepted, and none contradicts an accepted decision. They are grouped by where they
land.

### C.1 — Clause-versus-instrument gaps (each its own clause, each its own atom)

| Clause | Finding | Source |
| --- | --- | --- |
| CF-23 | Emitter names are mandatory by a `[FROZEN]` clause and marked `#[doc(hidden)]` | `emitter-surface-stability` |
| CF-25 / CF-26 | The portfolio check both clauses claim `spec-trace` performs does not exist | `portfolio-check-and-the-port-freeze-bar` |
| CF-38 | *"a case naming no clause"* is ambiguous between two readings; 54 of 58 cases fail one | `which-reading-of-a-case-naming-no-clause` |
| ES-18 | *"byte-identical"* is satisfied by no adapter; the conformance reading is weaker | `unswept-rows-and-byte-identity` |
| ES-23 | The clause has two MUSTs on two owners and `FROZEN_DOC_MUSTS` records one | `frozen-doc-musts-and-the-adapter-half` |
| VT-30 | The marker names two unbuilt falsifiers; one is built and the gap is narrower | `after-opt-scope` |
| CF-17 / CF-14 | Whether the new declaration obligation moves either marker | `stated-only-defects-and-the-reopen-must` |
| ES-11 | The ceiling sample costs 635 ms on the polling thread, and must | `sqlite-blocking-seam` |
| PS-2 | `probe_read_through`'s signature cannot be implemented by a live-transaction batch | `probe-read-through-and-the-live-transaction-end` |
| (none) | The read-fault rule exists and no clause obliges it — `UNCLAIMED_PENDING_ADR` entry 3 | `read-fault-clause-and-capability` |
| (none) | `RESET_REFUSAL`'s declension has no CF-39-shaped clause | `projection-declension-obligations` |

### C.2 — Gate and instrument findings

| Finding | Source | Destination kind |
| --- | --- | --- |
| A gate step's first check short-circuits its second and third under one name | `a-gate-step-whose-first-check-hides-its-second` | open_question |
| CF-36's check now exists and found thirteen breaches in three groups | `cf-36-thirteen-recorded-breaches` | merge (open_question) + merge (playbook) |
| The prose guard is retired; 45 rule-name declarations resolve against nothing | `prose-guard-retired-and-what-it-owes` | supersede + reference |
| A timed **ratio** landed outside the testkit, inside `cargo xtask ci` | `timed-assertions-outside-the-testkit` | open_question |
| `BenchmarkRecord::report` now panics on an incomplete scenario | `benchmark-completion-and-the-merge-red-promise` | open_question (same atom) |
| A guard required a literal `pub use` glob rather than the property it stood for | `typed-layer-promise-guard` | playbook |
| `MODEL_COVERAGE`'s table is only true at `PROPTEST_CASES >= 192` against a default of 256 | `model-family-case-floor` | reference + merge |
| `Kind::ModelOnlyMutant` is memberless; `Kind::StatedOnlyDefect` was withdrawn as unsound | `model-only-kind-…`, `stated-only-defects-…` | open_question + concept |
| A sole-evidence pin is required only where the shotgun mutant is the evidence | `sole-evidence-pins-and-moved-file-citations` | open_question |

### C.3 — Public-surface and packaging residuals

| Finding | Source | Destination kind |
| --- | --- | --- |
| Whether the five publishable crates state a version relationship to the contract (CF-32) | `adapter-driver-reexport-policy` | open_question |
| `happenstance`'s facade does not match ADR-0006's stated shape | `adapter-driver-reexport-policy` | open_question |
| ES-6 says nothing about whether the *wrapped type* is part of the promise | `adapter-driver-reexport-policy` | merge into `es-6-names-an-unwritable-rule` |
| Two crates sit on crates.io at `0.0.0` as name reservations | `adapter-driver-reexport-policy`, `repository-url-…` | open_question |
| `happenstance-cloudflare` has no `[features]` table and `worker` is unconditional | `adapter-driver-reexport-policy`, `stringified-throw-…` | open_question |
| `trait-variant`'s caret requirement resolves past the `--locked` gate for a consumer | `trait-variant-caret-and-the-consumer-resolve` | open_question |
| The testkit's root `version` key outlives its last non-dev consumer | `testkit-dev-dependency-version-requirement` | decision (B9) |
| Whether rustdoc's `file:line` citations should be URL-shaped now the repo is public | `repository-url-and-security-channel` | open_question |
| Whether a tuple `Boundary` admits members with differing `Event` types | `tuple-boundary-event-type` | open_question |
| Growing a **sealed** trait cannot produce `E0046` downstream — the inverse of RS-40-1 | `tuple-boundary-event-type` | concept |

### C.4 — Projection-port residuals (all gated by `kb-decision-0036`)

| Finding | Source |
| --- | --- |
| Whether the SQL seam's `&'static str` is final or a way-station to a `Statement` newtype | `projection-batch-sql-seam` |
| Whether `chunk` gets a named type and a default, and whether the runner gets an observation seam | `projection-runner-chunk-and-observation` |
| Whether `unstable-projection`'s exemption reaches `happenstance-testkit`'s `ProjectionFixture` | `fixture-declension-policy` |
| `PostgresFixture` owes a `READ_FAULT` declension by scope, and the injection is named | `discharge`, `read-fault-clause-…` |

### C.5 — Evidence with no commitment attached

| Measurement | Source | Destination |
| --- | --- | --- |
| Four guard shapes on one schema; the chain loses 9/9; the seed ordering is 38–44× backwards | `append-condition-sql-shape` | `reference/shipped-append-condition-sql-experiment-2026-09.md` |
| Eight-arm reactor-stall measurement on a `current_thread` runtime | `sqlite-blocking-seam`, `read-page-budget-…` | `reference/one-connection-latency-2026-09.md` |
| 699 intake citations resolvable at both commits; 77 drifted; 20 point into pinned evidence | `docs-citation-form-and-clause-content` | `reference/intake-citation-drift-census-2026-09.md` |
| Arm 2 of `dropped_append_future_leaves_no_partial_batch` is reached by two stores, both configs | `es-22-arm-two`, `f2-5` | `reference/mutation-coverage-arm-two-measurement-2026-09.md` |
| The macros-ceremony ratio re-measured on a second worked example: 74/1174 (6.3%) | `adr-0033-reopen-ground` | `reference/phase-7-macros-ceremony-second-example-2026-09.md` |
| ADR-0024's steady-state and staleness pair, against the built adapter | `adr-0024` | `reference/position-visibility-adapter-remeasurement-2026-09.md` |
| `MODEL_COVERAGE`'s cliff between 176 and 192 cases | `model-family-case-floor` | `reference/model-family-case-count-cliff-2026-09.md` |
| 45 rule-name declarations resolving against nothing, in three kinds | `prose-guard-retired` | `reference/spec-trace-unresolved-rule-declarations-2026-09.md` |

## D. Claims that earn no operation

Recorded so the next wave does not re-derive them.

| Claim | Source | Why no operation |
| --- | --- | --- |
| The `Event` clone allocation table, restated with two extra rows | `sqlite-blocking-seam` | `kb-reference-event-clone-allocations-001` already holds the experiment; the dating rule forbids extending it and two rows do not earn a second atom |
| ADR-0029's amend-without-supersede precedent | `adr-0033-reopen-ground` | Already fully documented in `kb-decision-0029`; the brief cites it, adding nothing |
| ADR-0036 restated as routing context | `projection-runner-chunk-…` | Restatement of an accepted decision |
| The `prelude` / `E0034` RUNBOOK bullet | `adapter-driver-reexport-policy` | Explicitly scoped out by its own source, with no grounding to place it on |
| Telemetry publication scope | `repository-url-…` | Answered by the act: the repository is public and `.redkiln/telemetry/` shipped with it. Recorded in ADR-0041's body, not as a live question |
| The `PartialBatch` variant rename; the `PreCommitPositionStore` doc comment | `unswept-rows`, `es-22-arm-two` | Both are one-line code edits routed to a lane. `open-questions/README.md`: *"a task … is a backlog item in `.bklg/`, not a KB atom"* |
| The nine per-brief *"did not get the two-critic pass"* caveats | 9 briefs | Carried as a discount in each sourced atom's summary, in the brief's own words — not as nine atoms |
