# Plan: from-contract-to-published-library

Ten projects, 135 stories, all fifteen initiative acceptance criteria owned. This
rollup supersedes the file's own previous body (headed "the rollup plan-prds
owed and did not deliver"), which recorded the state after 2026-08-12T18:23:54Z's
verify pass failed outright and left 39 stories as unauthored template stubs:
the whole of replication-identity-and-ingest (16), retention-and-incomplete-logs
(11), closeout-and-durable-audience (11) and stranger-install-smoke (1). Those 39
have since been re-run through a scoped plan-prds pass and are now specced. This
review re-verifies coverage, the dependency graph, anchors and distillation
quality across the whole initiative, with adversarial depth concentrated on the
39 stories that changed.

## Coverage map

Initiative requirement/AC-### to requirement to project to story(ies). Owner
projects are taken from _decomposition.md's traceability matrix (approved
2026-08-12); the story column is the story (or stories) inside that project's
_storymap.md whose traces_to actually discharges the criterion, verified against
each project's Coverage table.

| AC | Requirement | Project | Story(ies) |
|---|---|---|---|
| AC-01 | Model a boundary before choosing a database | typed-layer-and-alpha-release | domain-event-and-decision-model, adr-0020-fold-query-agreement |
| AC-02 | Compiler protects the domain as it grows | typed-layer-and-alpha-release | compile-fail-proof-artefact |
| AC-03 | Install from the registry and complete a write-then-read cycle | publication-and-positioning | stranger-install-smoke, guarantees-and-docs-rs-presentation |
| AC-04 | Adapter author told pass/fail for every rule, none silently absent | projection-store-freeze | projection-suite-entry-point, projection-capability-skips |
| AC-05 | Declined capability reports the fixture's reason, never vanishes | projection-store-freeze | projection-capability-skips |
| AC-06 | Storage shape not quietly assumed (non-serialising, no-cursor) | postgres-and-neon-stores | postgres-append-and-frontier-head, postgres-concurrency-family, neon-append-and-read-over-http |
| AC-07 | Edge developer's not-Send runtime runs every rule inside the gate | cloudflare-durable-object-store | wasm-execution-gate-step, every-rule-under-workerd |
| AC-08 | Evaluator can check the compliance claim from public artefacts | publication-and-positioning | compliance-claim-and-gaps-promise |
| AC-09 | Positions/gaps promise readable before depending, in plain language | publication-and-positioning | compliance-claim-and-gaps-promise |
| AC-10 | Every clause's maturity is legible and accurate at publish | publication-and-positioning | clause-maturity-audit, deferred-clause-reread |
| AC-11 | Published surface diffed against its prior baseline before release | publication-and-positioning | registry-surface-diff |
| AC-12 | MSRV stated as a promise with recorded justification | publication-and-positioning | msrv-promise-atom, guarantees-and-docs-rs-presentation |
| AC-13 | Replication identity has a written decision or reasoned refusal | replication-identity-and-ingest | adr-0026-peer-ingest-and-transport, adr-0027-merge-compensation-and-message-set, open-questions-resolved-and-indexed |
| AC-14 | Incomplete-log semantics has a written decision or reasoned refusal | retention-and-incomplete-logs | adr-0028-and-the-open-question-wave |
| AC-15 | Personas and journeys promoted into .kb/product/ as durable atoms | closeout-and-durable-audience | persona-and-journey-intake-staging, product-atom-promotion-via-kb-ingest |

Every AC-01 through AC-15 has exactly one owning project and at least one
discharging story. No uncovered ids.

**Fix pass applied to this rollup (this revision).** Two items surfaced by the
prior verify pass are now closed against the underlying artifacts; neither
touches an AC-to-story mapping above. First, the five off-by-one relative
`.kb/decisions/` links inside `retention-and-incomplete-logs`'s
`condition-over-removed-history-does-not-reject/spec.md` (lines 306, 399, 402)
and `cf-27-rule-or-recorded-refusal/spec.md` (lines 468, 474) are corrected from
three `../` levels to the correct four, matching the pattern every other
story-level `spec.md` in the initiative already uses from a directory four
levels below the repo root. Second, `stranger-install-smoke`'s
`depends_on: publish-0-2-0` was re-checked directly against both
`publication-and-positioning/stranger-install-smoke/spec.md`'s Dependencies
section and `publication-and-positioning/_storymap.md`'s Slices table (the
`the-release-event` milestone row): the two agree with each other and
`publish-0-2-0` is a real story in that project's own story map, so nothing on
disk was changed for that edge. What remains open is that this run's 39-story
Stage B-2 seed did not include `publish-0-2-0`, which truncated the computed
wave table below at the project boundary - see Merge order and Open risks.

## Project index

All ten project.md / _decomposition.md / _grounding.md / _storymap.md quartets
were confirmed present on disk this pass (directory listing per project) - none
is an unauthored template skeleton (each project.md runs 340-430 lines, each
_storymap.md 130-190 lines). Frontmatter on all ten project.md items currently
reads status: in-review, stage: design.

| Project | Objective | Briefs present | Stories | Status |
|---|---|---|---|---|
| projection-store-freeze (HS-P0010) | Freeze ProjectionStore only once its own invariant is enforced by a passing suite, not a doc comment | architecture, ux, testing | 17 | in-review / design; specced, previously verified |
| typed-layer-and-alpha-release (HS-P0011) | Put a real consumer above the facade; ship 0.2.0-alpha.1, the semver baseline every later diff reads | architecture, ux, testing, deployment | 16 | in-review / design; specced, previously verified |
| sqlite-durable-store (HS-P0012) | The durability and handle-multiplicity far ends - first fixture handing out two real handles onto one backing store | architecture, testing | 14 | in-review / design; specced, previously verified |
| cloudflare-durable-object-store (HS-P0013) | Run the not-Send flavour under workerd on wasm32, inside the gate rather than beside it | architecture, testing, deployment | 12 | in-review / design; specced, previously verified |
| postgres-and-neon-stores (HS-P0014) | The two far ends that disagree with the port: positions assigned outside the transaction, and no connection/transaction/cursor at all | architecture, testing, deployment | 14 | in-review / design; specced, previously verified |
| ladybug-projection-store (HS-P0015) | A structurally unlike batch shape, and the written verdict on whether the ProjectionStore freeze held | architecture, testing | 10 | in-review / design; specced, previously verified |
| publication-and-positioning (HS-P0016) | Where private opinions become promises: MSRV, public surface, clause maturities, the compliance claim, first contact. Ships 0.2.0 | ux, testing, deployment | 14 | in-review / design; re-verified this pass (stranger-install-smoke, the last of its 14, was one of the 39 re-run stories) |
| replication-identity-and-ingest (HS-P0017) | Replication identity across a store boundary: a decision or reasoned refusal, backed by a conformance suite and two unlike peers | architecture, testing | 16 | in-review / design; all 16 stories re-run and re-verified this pass |
| retention-and-incomplete-logs (HS-P0018) | What a store may forget, and how a reader finds out - the completeness axis with nothing at either end today | architecture, testing | 11 | in-review / design; all 11 stories re-run and re-verified this pass |
| closeout-and-durable-audience (HS-P0019) | Terminal. The only project wired to verify.e2e (the whole gate); re-observes DoD 1-15 as a set; promotes the audience into .kb/product/ | testing only (deliberate - it verifies and records, it designs nothing) | 11 | in-review / design; all 11 stories re-run and re-verified this pass |

## Story index

Story, project, one-line, depends_on, traces_to. The 39 stories re-run in this
pass (marked with a dagger) were checked directly against their spec.md bodies
(scope lock, Context pack, Dependencies section, acceptance table, PR boundary);
the remaining 96 were re-confirmed present and non-skeletal, and their
depends_on/traces_to are reproduced from each project's own _storymap.md
Slices/Coverage tables without re-opening every individual spec.md (those specs
were verified in the prior planning pass whose results this file's predecessor
already recorded, and nothing about them changed in this run).

### projection-store-freeze (17 stories)

| Story | One-line | depends_on | traces_to |
|---|---|---|---|
| ps-clause-pairing-sweep | Sweep PS-1-PS-37 for the coupling/progress pairing defect; scope the frozen-clause repair | none | AC-008 |
| projection-decision-atoms | Write and accept ADR-0017/0018/0019 (batch ownership, reset scope, apply-failure policy) | ps-clause-pairing-sweep | AC-008 |
| projection-api-design-record | Resolve DT-3 and DT-8 in _design.md; the public projection API surface review | projection-decision-atoms | AC-006, AC-007 |
| owned-batch-port-shape | Land type Batch, non-async infallible begin, Checkpoint/Authority/CommitError/ResetError, reset | projection-decision-atoms, projection-api-design-record | AC-009, AC-013 |
| projection-probe-conformance-feature | ProjectionProbe behind a conformance feature, mounted in export + feature table | owned-batch-port-shape | AC-009 |
| memory-projection-store | MemoryProjectionStore as the oracle and doctest target | owned-batch-port-shape, projection-probe-conformance-feature | AC-012 |
| projection-suite-entry-point | ProjectionFixture, for_each_projection_store_rule!, projection_store_conformance!, three harness files | owned-batch-port-shape, projection-probe-conformance-feature, memory-projection-store | AC-001, AC-016 |
| projection-capability-skips | A declined capability emits RuleOutcome::Skipped with a stated reason, distinguishable from a pass | projection-suite-entry-point, projection-api-design-record | AC-005 |
| projection-mutant-registry | CheckpointOnlyStore fails the suite by name; registry + exactness meta-tests | projection-suite-entry-point | AC-002, AC-003 |
| commit-rollback-and-drop-rules | Commit-side rules and the stores that fail each (rollback, drop, foreign batch, regressing position) | projection-mutant-registry | AC-003, AC-010 |
| reset-rules | reset is one unit of work, scoped and refusable | projection-mutant-registry, projection-capability-skips | AC-003, AC-011 |
| read-through-and-rebuild-rules | Read-through-batch, chunk-size-invariant rebuild, rebuilding-vs-live rules | projection-mutant-registry, projection-capability-skips | AC-003, AC-005 |
| buffering-conformant-variant | The CF-5 conformant variant - a buffering, replay-at-commit store, legally unlike MemoryProjectionStore | commit-rollback-and-drop-rules, reset-rules, read-through-and-rebuild-rules | AC-004 |
| ps3-batch-shape-finding | The PS-3 evidence written as a finding, no verdict, handed to publication-and-positioning | buffering-conformant-variant | AC-015 |
| documented-extension-surface | DT-8's outside-author arm discharged, whichever way it was decided | projection-api-design-record, buffering-conformant-variant | AC-007 |
| unstable-projection-gate-and-clause-disposition | Replace the provisional block with an unstable-projection gate and a stated reason; PS-1-PS-37 maturity accurate | ps-clause-pairing-sweep, ps3-batch-shape-finding | AC-014 |
| whole-gate-run-and-proof-artefact | One full cargo xtask ci on a clean checkout; the proof artefact recorded | unstable-projection-gate-and-clause-disposition, documented-extension-surface | AC-013, AC-016 |

### typed-layer-and-alpha-release (16 stories)

| Story | One-line | depends_on | traces_to |
|---|---|---|---|
| adr-0020-fold-query-agreement | Decision: a decision model folds a domain enum and derives its Query from EVENT_TYPES + tags | none | AC-001, AC-014, AC-016 |
| adr-0021-payload-evolution-and-codec-tag | Decision: payload evolution, codec tag home, no adapter needs to understand it | none | AC-005, AC-016 |
| domain-event-and-decision-model | DomainEvent, DecisionModel, apply, the derived query() | adr-0020-fold-query-agreement | AC-001, AC-014 |
| decision-model-composition | Declarative macro composing several boundaries' queries at compile time | domain-event-and-decision-model | AC-004 |
| codec-and-feature-forwarding | Codec with JSON default, CBOR/postcard behind forwarded features | adr-0021-payload-evolution-and-codec-tag | AC-005 |
| command-loop | Read then decide then append then retry-on-ConditionViolated, bounded and caller-visible | domain-event-and-decision-model, codec-and-feature-forwarding | AC-005 |
| misbehaving-testkit-stores | FaultyStore and GappyMemoryStore in the testkit, each with the wrong implementation it rejects | none | AC-009 |
| given-when-then-dsl | Given/when/then DSL over MemoryEventStore, no database | command-loop, decision-model-composition, misbehaving-testkit-stores | AC-009 |
| projection-trait-and-runner | Application-facing Projection trait and runner, streaming never collecting | codec-and-feature-forwarding | AC-006 |
| projection-clause-verdicts | PS-33/PS-27/PS-30/PS-18 verdicts as the three-part SPECIFICATION.md edit | projection-trait-and-runner | AC-007, AC-008 |
| polling-cost-measurement | Reproducible experiments/ harness measuring the N-views-by-N-reads polling cost | projection-trait-and-runner | AC-010 |
| worked-example-on-typed-layer | Rewrite course-subscriptions onto happenstance; execute the binary in the gate | command-loop, decision-model-composition | AC-003 |
| compile-fail-proof-artefact | Compile-fail case plus negative control, registered in xtask/src/proof.rs | worked-example-on-typed-layer | AC-002 |
| edge-flavour-and-wasm-claim | Settle the wasm32 claim for this crate by name, or record why not | command-loop, projection-trait-and-runner | AC-015 |
| defect-log-and-macros-verdict | BR-01's defect log; the happenstance-macros in/out verdict | worked-example-on-typed-layer, projection-trait-and-runner | AC-012, AC-013 |
| publish-0-2-0-alpha-1 | Cut 0.2.0-alpha.1, full cargo xtask ci, README/CHANGELOG reconciled | compile-fail-proof-artefact, projection-clause-verdicts, edge-flavour-and-wasm-claim, defect-log-and-macros-verdict | AC-011 |

### sqlite-durable-store (14 stories)

| Story | One-line | depends_on | traces_to |
|---|---|---|---|
| benchmark-harness | event_store_benchmarks! behind an off-by-default, target-gated bench feature | none | AC-012 |
| adr-0022-append-condition-strategy | Measure the three append-condition candidates against real SQLite; accept ADR-0022 | benchmark-harness | AC-013 |
| schema-migration-and-identity | Migration 1, StoreId, WAL, busy timeout | adr-0022-append-condition-strategy | AC-001, AC-004 |
| append-atomicity-and-store-limits | append's todo!() replaced with one BEGIN IMMEDIATE, three ceilings enforced | schema-migration-and-identity | AC-007, AC-009 |
| lazy-read-with-snapshot-ceiling | SqliteReadStream, position ceiling captured no later than the first poll_next | schema-migration-and-identity | AC-001 |
| wide-query-chunked-not-refused | Wide Query decomposed and k-way-merged rather than refused | lazy-read-with-snapshot-ceiling | AC-008 |
| sqlite-fixture-and-whole-suite | SqliteFixture mounted; event_store_conformance! green whole against a real file | append-atomicity-and-store-limits, wide-query-chunked-not-refused | AC-001, AC-002, AC-003, AC-004, AC-009 |
| concurrency-family-and-contender-count | event_store_concurrency_conformance! green with the runtime seam in place | sqlite-fixture-and-whole-suite | AC-005, AC-007 |
| model-family-and-mutant-pass-column | event_store_model_conformance! green; SqliteEventStore in the mutant harness's pass column | concurrency-family-and-contender-count | AC-006, AC-007 |
| reopen-negative-control-and-durability-verdicts | The reopen rule's negative control; CF-17/ES-35/CF-14 verdicts | sqlite-fixture-and-whole-suite | AC-004, AC-010 |
| projection-store-passes-the-borrowed-suite | SqliteProjectionStore real bodies, passes the frozen projection suite | schema-migration-and-identity | AC-011 |
| instrument-markers-removed-and-gate-green | Last todo!() and scoped allow removed together; --fast green | model-family-and-mutant-pass-column, reopen-negative-control-and-durability-verdicts, projection-store-passes-the-borrowed-suite | AC-014 |
| crates-io-name-and-packaging-facts | Reserve happenstance-sqlite; real description/README/licences | instrument-markers-removed-and-gate-green | AC-015 |
| spec-and-code-reconciliation | Clause range reconciled against ADR-0022's discharge; spec-trace citation count unchanged or higher | instrument-markers-removed-and-gate-green, reopen-negative-control-and-durability-verdicts | AC-016 |

### cloudflare-durable-object-store (12 stories)

| Story | One-line | depends_on | traces_to |
|---|---|---|---|
| wasm-execution-gate-step | A named, non-skippable xtask step that executes the wasm32 conformance harness inside cargo xtask ci | none | AC-004 |
| worker-binding-layer | Real Durable Object SqlStorage bindings replacing the worker-free stand-in | none | AC-001, AC-005 |
| durable-object-write-path | migrate/append/head/contains_event_id against real bindings | worker-binding-layer | AC-001, AC-007 |
| durable-object-read-path | read per ADR-0011's ceiling-and-page, cursor never held across an await | worker-binding-layer | AC-001, AC-007 |
| caller-visible-error-verdict | ES-6 artefact: caller-visible error distinguishes constraint violation from transport fault | durable-object-write-path | AC-005 |
| durable-object-host-and-fixture | Durable Object test host + CloudflareFixture; every declined capability carries a real reason | durable-object-write-path, durable-object-read-path | AC-003, AC-008 |
| every-rule-under-workerd | Conformance target invoked with __emit_wasm, registered in the new gate step | wasm-execution-gate-step, durable-object-host-and-fixture | AC-002, AC-003, AC-004 |
| measured-store-limits | Real measured ceilings; REOPEN/MID_BATCH_FAULT decided honestly | durable-object-host-and-fixture, every-rule-under-workerd | AC-007, AC-008 |
| wf-11-memory-ceiling-falsifier | Test WF-11's falsifier against the real measured ceiling | every-rule-under-workerd | AC-011 |
| deferral-re-reads-and-es-32-verdict | CF-14/CF-27 deferral re-reads; ES-32 tail-seam verdict | every-rule-under-workerd | AC-009, AC-010 |
| adr-0023-and-atom-resolutions | ADR-0023 accepted; CF-40 and WF-11 atoms resolved | durable-object-read-path, caller-visible-error-verdict, every-rule-under-workerd, measured-store-limits, wf-11-memory-ceiling-falsifier | AC-005, AC-006, AC-008, AC-011 |
| publish-ready-crate | Licences, README, publish = false removed, name reserved, full gate green | worker-binding-layer, every-rule-under-workerd, measured-store-limits | AC-012 |

### postgres-and-neon-stores (14 stories)

| Story | One-line | depends_on | traces_to |
|---|---|---|---|
| claim-crate-names | Reserve happenstance-postgres and happenstance-neon on crates.io | none | AC-012 |
| postgres-schema-and-live-fixture | Migration 1, testcontainers-backed PostgresFixture/ConcurrentFixture, live-Postgres CI job | none | AC-011 |
| postgres-append-and-frontier-head | Real append/head/contains_event_id; head returns the visibility frontier | postgres-schema-and-live-fixture | AC-002, AC-004, AC-010 |
| postgres-concurrency-family | Concurrency family green at CONTENDERS = 8, writers unserialised | postgres-append-and-frontier-head | AC-003 |
| postgres-rule-controls | Naive nextval() arm shown to fail CF-13 before the chosen mechanism is credited | postgres-append-and-frontier-head | AC-002, AC-004 |
| adr-0024-position-visibility-mechanism | Position-visibility mechanism decided on a re-measured number | postgres-append-and-frontier-head, postgres-concurrency-family, postgres-rule-controls | AC-001 |
| postgres-structural-bill | Frontier/no-RYOW/staleness consequences recorded where a consumer meets them | adr-0024-position-visibility-mechanism | AC-009 |
| neon-sql-transport | Real SqlTransport, host + wasm32, proven by one live round trip | none | AC-006, AC-011 |
| neon-fixture-and-live-job | NeonFixture against a live branch; ceilings and capability declines stated honestly | neon-sql-transport | AC-007, AC-011 |
| neon-append-and-read-over-http | event_store_conformance! against a store with no connection/transaction/cursor | neon-fixture-and-live-job, postgres-append-and-frontier-head | AC-006, AC-007, AC-010 |
| neon-conflicting-position-verdict | The CTE's conflicting_position-preserving claim confirmed or refuted live | neon-append-and-read-over-http | AC-008 |
| postgres-projection-store | PostgresProjectionStore against the frozen Batch, passes the projection suite | postgres-schema-and-live-fixture | AC-005 |
| deskeleton-and-package-readiness | No todo!(), no publish = false; PUBLISHABLE grown to five | claim-crate-names, postgres-structural-bill, neon-conflicting-position-verdict, postgres-projection-store | AC-012 |
| far-end-discharge-record | ES-10/11/12/41/42 and VT-21-24 status recorded for the publication audit | deskeleton-and-package-readiness | AC-013 |

### ladybug-projection-store (10 stories)

| Story | One-line | depends_on | traces_to |
|---|---|---|---|
| preflight-and-unlike-axes | Assert the merged port shipped type Batch; commit the dated unlike-axes document | none | AC-001 |
| adr-0025-three-answers | ADR-0025: checkpoint placement, graph-mutation vocabulary, blocking-API bridge | preflight-and-unlike-axes | AC-002 |
| real-lbug-driver-swap | Real lbug dependency replaces stand_in; live_handle.rs retired | adr-0025-three-answers | AC-003 |
| fill-the-bodies-and-ps-34-disposition | Fill the four projection_store.rs bodies; PS-34 disposition from fresh hands | real-lbug-driver-swap | AC-003, AC-006, AC-008 |
| ladybug-fixture-and-conformance-run | LadybugFixture, projection_store_conformance! invoked, registered in ARTEFACTS | fill-the-bodies-and-ps-34-disposition | AC-004 |
| read-your-own-writes-projection | RYOW projection executed and recorded as supported or a declared limit | ladybug-fixture-and-conformance-run | AC-005 |
| package-completeness-and-name-claim | Licences, README, publish = false removed, crates.io name claimed | fill-the-bodies-and-ps-34-disposition | AC-010 |
| cold-build-cost-and-ci-shape | Cold/warm lbug build measured; CI shape implemented and recorded | ladybug-fixture-and-conformance-run | AC-009 |
| freeze-verdict-document | Dated verdict, held or did not hold, naming rules, clauses, commit SHA | read-your-own-writes-projection, cold-build-cost-and-ci-shape, package-completeness-and-name-claim | AC-007, AC-008 |
| verdict-ordering-and-publication-handoff | Merge ahead of publication-and-positioning's gate; hand off build number and docs.rs findings | freeze-verdict-document | AC-011 |

### publication-and-positioning (14 stories)

| Story | One-line | depends_on | traces_to |
|---|---|---|---|
| crate-set-decision | Decision: 0.2.0 ships exactly three crates | none | AC-001, AC-013 |
| projection-port-ship-shape | PS-3 settled: frozen or unstable-projection, with doc(cfg) treatment either way | crate-set-decision | AC-012, AC-013 |
| msrv-promise-atom | New decision atom (0030+) stating the 1.97.1 floor as a promise | none | AC-006, AC-013 |
| first-contact-design-resolutions | DT-1/DT-4/DT-5/DT-6 resolved in _design.md | none | AC-011, AC-013 |
| falsifier-ledger-repair | Repair the five short rows in RUNBOOK.md's provisional ledger | none | AC-004 |
| clause-maturity-audit | Mandatory xtask clause-audit step, reconciled totals, frozen-clause fingerprint | falsifier-ledger-repair | AC-003, AC-004, AC-015 |
| deferred-clause-reread | All ten [DEFERRED] clauses re-read for an honest, dated reason | clause-maturity-audit | AC-005 |
| registry-surface-diff | Mandatory surface diff against the 0.2.0-alpha.1 baseline | crate-set-decision | AC-002 |
| landing-copy-and-status-truth | DT-1/DT-5/DT-6 land on the packaged README's first screen | first-contact-design-resolutions, crate-set-decision, projection-port-ship-shape | AC-007, AC-011 |
| compliance-claim-and-gaps-promise | Claim-with-evidence form; positions/gaps promise stated plainly | first-contact-design-resolutions | AC-009, AC-010 |
| guarantees-and-docs-rs-presentation | Guarantees slot; docs.rs metadata block; quick-start as the crate's own doctest | msrv-promise-atom, projection-port-ship-shape | AC-006, AC-007, AC-008, AC-014 |
| rendered-page-preflight | Read the rendered crates.io/docs.rs pages against the accessibility floor | landing-copy-and-status-truth, compliance-claim-and-gaps-promise, guarantees-and-docs-rs-presentation | AC-007 |
| publish-0-2-0 | Publish the three crates; full cargo xtask ci on the publish commit | rendered-page-preflight, registry-surface-diff, clause-maturity-audit, deferred-clause-reread, crate-set-decision | AC-001, AC-014, AC-016 |
| stranger-install-smoke (dagger, this pass) | From a scratch project outside this workspace, cargo add the published crate and run the quick-start write-then-read cycle | publish-0-2-0 per spec.md and _storymap.md; the 39-story seed recorded none - see Merge order | AC-008 |

### replication-identity-and-ingest (16 stories, dagger, this pass)

| Story | One-line | depends_on | traces_to |
|---|---|---|---|
| adr-0026-peer-ingest-and-transport | ADR-0026: what a peer is, what the port may assume, what ingest promises | none | AC-001, AC-002, AC-010, AC-011, AC-014 |
| adr-0027-merge-compensation-and-message-set | ADR-0027: merge rule, compensation contract, replication scope, one-or-two-abstraction, message set | adr-0026-peer-ingest-and-transport | AC-001, AC-010, AC-014 |
| open-questions-resolved-and-indexed | Both phase-13 open-question atoms resolved in place, index updated | adr-0026-peer-ingest-and-transport, adr-0027-merge-compensation-and-message-set | AC-013 |
| memory-store-ingest-seam | One additive inherent &self op on MemoryEventStore for a foreign EventId | adr-0026-peer-ingest-and-transport | AC-002 |
| ingest-store-and-memory-peer-round-trip | Real IngestStore/MemorySyncPeer bodies; foreign EventId preserved, atomic, idempotent | memory-store-ingest-seam | AC-002 |
| sync-testkit-crate-and-rule-registry | crates/happenstance-sync-testkit/, for_each_sync_peer_rule! + sync_peer_conformance! | ingest-store-and-memory-peer-round-trip | AC-003, AC-015 |
| gate-mounts-for-the-sync-suite | Teach RULE_FILES/TESTKIT_SRC/lints/wasm32 steps about the fourth suite | sync-testkit-crate-and-rule-registry | AC-004, AC-009, AC-014 |
| headline-rules-and-mutant-registry | ingest_never_rejects, compensation_is_atomic..., wire_condition_with_after_is_refused, each with a mutant | sync-testkit-crate-and-rule-registry, gate-mounts-for-the-sync-suite, adr-0027-merge-compensation-and-message-set | AC-002, AC-003, AC-004, AC-008 |
| message-set-on-the-envelope | PushBatch/EventGroup/ReplicatedEvent instantiating Envelope<T>; FORMAT_VERSION disposition | headline-rules-and-mutant-registry, adr-0027-merge-compensation-and-message-set | AC-006 |
| byte-identical-round-trip-and-idempotent-replay | Payload Bytes byte-identical at the receiver; replay a no-op | message-set-on-the-envelope | AC-006 |
| adr-0003-provisional-lift | New atom lifting ADR-0003's provisional marker, citing the round trip | byte-identical-round-trip-and-idempotent-replay | AC-006 |
| send-free-sync-runner | Runner bound on the weaker EventStore/SyncPeer/IngestStore, never Send | ingest-store-and-memory-peer-round-trip, gate-mounts-for-the-sync-suite | AC-009 |
| hub-and-spoke-and-peer-to-peer-topologies | Both topologies over the same suite/runner; one adapter type in both roles | send-free-sync-runner, adr-0027-merge-compensation-and-message-set | AC-007 |
| durable-object-and-neon-peers | SyncPeer/IngestStore in happenstance-cloudflare and happenstance-neon; one suite, three peers | headline-rules-and-mutant-registry, message-set-on-the-envelope, send-free-sync-runner | AC-005 |
| frozen-clause-repairs | Re-run the deferral sweep; drop (new) markers; repair affected Rejects: clauses | headline-rules-and-mutant-registry, adr-0026-peer-ingest-and-transport | AC-002, AC-012 |
| clause-arithmetic-and-deferral-renewals | Settle/renew every [DEFERRED] SY clause; compute the ADR-0026/0027 union | frozen-clause-repairs, gate-mounts-for-the-sync-suite | AC-010, AC-014 |

### retention-and-incomplete-logs (11 stories, dagger, this pass)

| Story | One-line | depends_on | traces_to |
|---|---|---|---|
| retained-set-instrument-and-conformance-mount | Completeness instrument (decorator over MemoryEventStore, suffix + scattered), mounted via event_store_conformance! | none | AC-001, AC-013 |
| cf-27-experiment-and-recorded-pass-list | CF-27's experiment: full suite in both configurations, pass list committed as data | retained-set-instrument-and-conformance-mount | AC-002 |
| positions-are-not-reused-after-removal | ES-38's rule, registered, its renumbering-on-compaction mutant | cf-27-experiment-and-recorded-pass-list | AC-003, AC-004, AC-011 |
| condition-over-removed-history-does-not-reject | ES-40's rule (vacuous pass as specified behaviour); rustdoc discharge | cf-27-experiment-and-recorded-pass-list | AC-003, AC-005, AC-011 |
| decision-model-and-ingest-observed | Composition roots 4 and 6's actual wrong output recorded, no new type | retained-set-instrument-and-conformance-mount | AC-006 |
| projection-runner-across-the-hole | Composition root 5 (the runner) resuming across the hole, actual state recorded | retained-set-instrument-and-conformance-mount | AC-006 |
| dt-7-signal-shape-and-the-redaction-answer | DT-7 resolved; the redaction question answered or explicitly deferred | decision-model-and-ingest-observed | AC-009, AC-010 |
| adr-0028-and-the-open-question-wave | ADR-0028 (what a store may forget); ES-38's open question narrowed and superseded | cf-27-experiment-and-recorded-pass-list, positions-are-not-reused-after-removal, condition-over-removed-history-does-not-reject, decision-model-and-ingest-observed, projection-runner-across-the-hole, dt-7-signal-shape-and-the-redaction-answer | AC-008, AC-010, AC-015, AC-016 |
| cf-27-rule-or-recorded-refusal | CF-27's own rule written, or its (new)/dagger marker kept with a written refusal | adr-0028-and-the-open-question-wave | AC-007 |
| marker-moves-and-spec-trace-green | ES-39/CF-27 marker moves; spec-trace --write; hand-reconciled section 1.3 census | cf-27-rule-or-recorded-refusal, positions-are-not-reused-after-removal, condition-over-removed-history-does-not-reject | AC-014 |
| surface-diff-and-the-ac-012-escalation | Surface comparison against the 0.2.0 baseline; the AC-012 escalation raised or explicitly declined | marker-moves-and-spec-trace-green | AC-011, AC-012 |

### closeout-and-durable-audience (11 stories, dagger, this pass)

| Story | One-line | depends_on | traces_to |
|---|---|---|---|
| clean-checkout-harness | The clean-checkout seam; stands up _closeout-record.md | none | AC-001 |
| whole-gate-green-on-the-assembled-tree | cargo xtask ci (never --fast) to completion; exit code, SHA, four wasm32 outcomes, skip reasons | clean-checkout-harness | AC-001, AC-002 |
| dod-set-re-observation-record | DoD 1-12, 14-15 re-observed as one table with commands and artefact paths | whole-gate-green-on-the-assembled-tree | AC-003 |
| published-tree-delta-statement | The DoD 13 delta stated: tag, commits, owning projects, the cited gate decision | dod-set-re-observation-record | AC-004 |
| decision-atom-audit-table | Audit table: every settled question to accepted atom to alternatives rejected | none | AC-005 |
| open-question-preservation-audit | Zero-deletion diff of .kb/open-questions/; consumed atoms resolution-bearing | decision-atom-audit-table | AC-006 |
| persona-and-journey-intake-staging | Three personas, four journeys staged in .kb/_intake/ under a distinct wave id | none | AC-010 |
| product-atom-promotion-via-kb-ingest | Promotion through /redkiln:kb-ingest; .kb/product/ atoms, intake cleared | persona-and-journey-intake-staging | AC-007, AC-008, AC-009 |
| backlog-and-kb-health-at-closeout | validate/validate --kb/doctor --json clean; template-drift set equal to six | product-atom-promotion-via-kb-ingest | AC-011, AC-012 |
| findings-disposition-register | Every finding routed (support / new item / decision+re-plan); zero fixed in this project | backlog-and-kb-health-at-closeout, decision-atom-audit-table, dod-set-re-observation-record, open-question-preservation-audit, published-tree-delta-statement, whole-gate-green-on-the-assembled-tree | AC-013 |
| initiative-closeout-readiness | Every sibling HS-P0010 through HS-P0018 closed, no open child of HS-I0006 | findings-disposition-register, product-atom-promotion-via-kb-ingest | AC-014 |

## Merge order

### Project-level (unchanged; _decomposition.md, Sequencing)

1. projection-store-freeze
2. typed-layer-and-alpha-release - ships 0.2.0-alpha.1
3. sqlite-durable-store
4. cloudflare-durable-object-store (no blockers; may start day one alongside 1)
5. postgres-and-neon-stores
6. ladybug-projection-store
7. publication-and-positioning - ships 0.2.0
8. replication-identity-and-ingest
9. retention-and-incomplete-logs
10. closeout-and-durable-audience

Positions 4, 5 and 6 are mutually unordered. The serial trunk is 1 to 2 to 3 to 7
to 8 to 9 to 10.

### This run's computed story-level waves (the 39 re-run stories; reproduced verbatim)

Computed by Kahn over the story map seed this run was launched with, ties broken
by seed order. Acyclic.

wave 1: closeout-and-durable-audience/clean-checkout-harness, closeout-and-durable-audience/decision-atom-audit-table, closeout-and-durable-audience/persona-and-journey-intake-staging, replication-identity-and-ingest/adr-0026-peer-ingest-and-transport, retention-and-incomplete-logs/retained-set-instrument-and-conformance-mount

wave 2: closeout-and-durable-audience/whole-gate-green-on-the-assembled-tree, closeout-and-durable-audience/open-question-preservation-audit, closeout-and-durable-audience/product-atom-promotion-via-kb-ingest, replication-identity-and-ingest/adr-0027-merge-compensation-and-message-set, replication-identity-and-ingest/memory-store-ingest-seam, retention-and-incomplete-logs/cf-27-experiment-and-recorded-pass-list, retention-and-incomplete-logs/decision-model-and-ingest-observed, retention-and-incomplete-logs/projection-runner-across-the-hole

wave 3: closeout-and-durable-audience/dod-set-re-observation-record, closeout-and-durable-audience/backlog-and-kb-health-at-closeout, replication-identity-and-ingest/open-questions-resolved-and-indexed, replication-identity-and-ingest/ingest-store-and-memory-peer-round-trip, retention-and-incomplete-logs/positions-are-not-reused-after-removal, retention-and-incomplete-logs/condition-over-removed-history-does-not-reject, retention-and-incomplete-logs/dt-7-signal-shape-and-the-redaction-answer

wave 4: closeout-and-durable-audience/published-tree-delta-statement, replication-identity-and-ingest/sync-testkit-crate-and-rule-registry, retention-and-incomplete-logs/adr-0028-and-the-open-question-wave

wave 5: closeout-and-durable-audience/findings-disposition-register, replication-identity-and-ingest/gate-mounts-for-the-sync-suite, retention-and-incomplete-logs/cf-27-rule-or-recorded-refusal

wave 6: closeout-and-durable-audience/initiative-closeout-readiness, replication-identity-and-ingest/headline-rules-and-mutant-registry, replication-identity-and-ingest/send-free-sync-runner, retention-and-incomplete-logs/marker-moves-and-spec-trace-green

wave 7: replication-identity-and-ingest/message-set-on-the-envelope, replication-identity-and-ingest/hub-and-spoke-and-peer-to-peer-topologies, replication-identity-and-ingest/frozen-clause-repairs, retention-and-incomplete-logs/surface-diff-and-the-ac-012-escalation

wave 8: replication-identity-and-ingest/byte-identical-round-trip-and-idempotent-replay, replication-identity-and-ingest/durable-object-and-neon-peers, replication-identity-and-ingest/clause-arithmetic-and-deferral-renewals

wave 9: replication-identity-and-ingest/adr-0003-provisional-lift

out of band: publication-and-positioning/stranger-install-smoke, scheduled after
publication-and-positioning/publish-0-2-0 (milestone the-release-event, position
4.3 in that project's _storymap.md). It is deliberately absent from the nine
waves above rather than placed in one, because publish-0-2-0 is outside this
run's 39-story seed and so has no wave number to sit after.

Edge defects in this computed graph, and how each was disposed of at the
2026-08-13 spec gate:

- Dangling edge, in the SEED - now corrected in this table by hand.
  publication-and-positioning/stranger-install-smoke declares depends_on:
  publish-0-2-0 in both its own spec.md (section Dependencies, "Blocks on -
  publish-0-2-0. Structural rather than conventional...") and its project's
  _storymap.md (Slices table, the-release-event milestone row). publish-0-2-0 is
  a real story in publication-and-positioning - it is simply outside the
  39-story seed this Stage B-2 pass computed over, so the edge pointed at a node
  the seed never included and Kahn scheduled the story into wave 1 as if it had
  no dependencies. That wave-1 placement is REMOVED above and replaced with the
  out-of-band line, which states the true position.
- The item graph was never wrong. `redkiln link` recorded the edge on the items
  themselves during Stage A step 9: HS-S0097 (stranger-install-smoke) carries
  blocked_by HS-S0096 (publish-0-2-0), verified directly at the gate. So
  `redkiln next`, the ready queue, `doctor` and any future scheduler have always
  had the correct predecessor; the defect was confined to the wave table in this
  file, which reproduced the truncated seed's computed order verbatim.
- Disposition. Approved at the spec gate on 2026-08-13 (repository owner) as a
  hand correction to this table rather than a Stage B-2 re-run. Re-running Stage
  B-2 with the full publication-and-positioning seed would recompute the order
  correctly, but it would also re-author 39 specs that this pass verified
  line-by-line, to repair one row of a narrative table whose authoritative
  counterpart - the item graph - is already right. If a future pass re-runs
  Stage B-2 over a seed that includes publish-0-2-0, the computed table will
  reproduce the out-of-band line above as a real wave number, and this note
  becomes redundant rather than wrong.

## Open risks and unresolved cuts

- **Closed at the 2026-08-13 spec gate.** The stranger-install-smoke edge defect
  was a scheduling hazard as long as the wave table was the thing being read,
  and this revision removes the hazard at its only real site. The claim the
  previous revision made - that the defect "is not fixable from inside this
  file" - was true only of the *computed* order; it overlooked that the table
  itself may be corrected by hand once a human has approved the correction, and
  that the authoritative graph was never wrong in the first place. Three facts
  settle it. The on-disk planning artifacts always agreed with each other and
  correctly declared `depends_on: publish-0-2-0`. The item graph always carried
  the edge, `HS-S0097 blocked_by HS-S0096`, written by `redkiln link` at Stage A
  step 9 and verified at the gate - so every consumer that schedules off
  `redkiln` rather than off this file was already correct. And the wave table
  above no longer places stranger-install-smoke in wave 1: it is listed out of
  band, after publish-0-2-0, which is where the artifacts always said it goes.
  The standing instruction is unchanged and now needs no caveat: no work starts
  against stranger-install-smoke until publish-0-2-0 has merged.
- **Fixed this pass.** Five dead relative-link anchors, all in
  retention-and-incomplete-logs, all the same off-by-one-directory-level error
  (three levels of ../../../ instead of four, from a story directory that sits
  four levels below the repo root): condition-over-removed-history-does-not-reject/spec.md
  lines 306, 399 and 402, and cf-27-rule-or-recorded-refusal/spec.md lines 468
  and 474, all citing either ADR-0001 (.kb/decisions/0001-async-port-flavours.md)
  or ADR-0029 (.kb/decisions/0029-msrv-raised-to-1-97-1.md). Each previously
  resolved one directory short of the repo root (.bklg/.kb/decisions/...), which
  does not exist. All five are now corrected to ../../../../.kb/decisions/...,
  matching the pattern already used correctly by story-level spec.md files
  elsewhere in the initiative (e.g. closeout-and-durable-audience/whole-gate-green-on-the-assembled-tree/spec.md:93,
  cloudflare-durable-object-store/durable-object-read-path/spec.md:22,
  sqlite-durable-store/adr-0022-append-condition-strategy/spec.md:78). Re-swept
  after the edit: zero remaining three-level `.kb/` relative links in either
  file, and no other relative link in either file needed correction.
- Story-level re-verification in this pass is scoped to the 39 stories that
  changed plus mechanical presence checks on the other 96. The other six
  projects' individual spec.md bodies were not re-opened here; their prior
  verification (recorded in this file's superseded predecessor) is carried
  forward on the assumption that nothing about them changed between that run and
  this one - true as far as git status on
  .bklg/from-contract-to-published-library shows (only the untracked initiative
  directory itself, no modifications since this worktree's creation).
- design.capture remains a deliberate skip, not a pass, for every project except
  publication-and-positioning's _design.md (which does carry a static mock,
  design/mock.html, 51 frames, manually captured - the one place in the
  initiative a perceptual review is possible). closeout-and-durable-audience,
  replication-identity-and-ingest and retention-and-incomplete-logs correctly
  carry no ux brief and no _design.md surfaces; there is nothing to check under
  the design-coverage / composition-AC checks for them, and that absence is
  itself the correct answer rather than a gap.
- The 133 not-yet-existing cited paths this file's predecessor already found
  (test files, ADR long-forms, decision atoms, the new happenstance-sync-testkit
  crate) are unchanged in character: every one is a deliverable a story in this
  plan exists to create, not a wrong citation. Re-confirmed by this pass's own
  path sweep across the 39 re-run stories, which found the same shape (glob
  patterns, angle-bracket placeholder slugs, and named-but-not-yet-created files)
  and zero citations to a file that should already exist but does not.

## Verification result

pass: false, **as the verifier returned it**. That verdict is left standing
rather than edited to green, because it is the record of what the automated pass
actually found; what follows it below is the finding, and what follows this
paragraph is its disposition by a human. Both outstanding items are now closed:
the dead-anchor cluster by repair, and the merge-order disagreement by hand
correction to the wave table at the 2026-08-13 spec gate, once the item graph
was verified to carry the edge the seed had dropped (`HS-S0097 blocked_by
HS-S0096`). Nothing below this paragraph is amended; read it as the verifier's
own words, with the Merge order and Open risks sections carrying what changed
afterwards.

Of the two items the prior verify pass left outstanding, one is now
fixed and one remains open. The dead-anchor cluster (five relative links, two
stories) is fixed in this revision - see Open risks. The disagreement between
the computed merge order above and the true dependency graph remains open, and
is not fixable by editing any file in this repository: both
stranger-install-smoke's spec.md and publication-and-positioning's _storymap.md
already correctly declare depends_on: publish-0-2-0 (re-confirmed here), so the
defect is in this run's Stage B-2 seed, not in either artifact. Stage B-2 must
be re-run with the full seed for the wave table to be recomputed correctly; a
fix pass cannot change this run's computed order, only report where it
disagrees with the graph the artifacts describe. Neither item is a coverage
gap and neither implicates the newly-authored specs' substance - the fixed one
was a mechanical repair, and the open one is a scheduling hazard confined to
one story's readiness, not a defect in the plan's traceability.

- Traceability: complete. AC-01 through AC-15 each have exactly one owning
  project and at least one discharging story (Coverage map above). Every one of
  the ten projects' own AC lists (16, 16, 16, 12, 13, 11, 16, 15, 16, 14 - 145
  total) is fully covered by its _storymap.md Coverage table with no orphan and
  no duplicated single-responsibility claim, re-checked directly for all four
  projects touched this pass and spot-checked for AC-count agreement against the
  other six.
- Uncovered ids: none.
- MECE: held. No responsibility found owned by two stories inside any of the
  four re-verified projects; the "AC appears twice" rows are all real splits (a
  decision atom versus the code it authorises, or two halves of one criterion),
  each with the split named in its project's Coverage table.
- DAG / merge order: the ten-project DAG (_decomposition.md) is unchanged and
  acyclic. The 39-story computed waves are acyclic and reproduced verbatim above
  - unchanged by this fix pass, per instruction, since the order is a function
  of the seed this run was launched with and not something a fix pass may
  recompute. One disagreement remains, already flagged by the harness and
  re-confirmed against the artifacts in this pass: stranger-install-smoke's real
  dependency on publish-0-2-0 is absent from the 39-story seed (dangling edge /
  wave-1 mis-scheduling), even though both the story's spec.md and its project's
  _storymap.md agree with each other and correctly declare the edge - there is
  no on-disk text to correct, only a seed to re-run (recorded under Open risks).
  All other 38 stories' spec.md Dependencies sections were checked line-by-line
  against their project _storymap.md depends_on columns and against the
  dependsOn/specDependsOn values supplied for this run; all matched.
- Dead anchors: zero remaining. Five were found, all in
  retention-and-incomplete-logs, by mechanically resolving every relative
  markdown link in the 39 re-run stories' spec.md/discover.md/_ledger.md files
  against the citing file's own directory; all five are corrected in this
  revision (file:line and the fix applied are recorded under Open risks). No
  other dead relative links and no dead backtick-path repo citations among the
  roughly 240 distinct non-relative paths swept (all either exist or are
  labelled/globbed future deliverables).
- Artifact postcondition: all ten projects carry a real, non-skeletal project.md
  plus _decomposition.md plus _grounding.md plus _storymap.md quartet, confirmed
  present on disk this pass for every one of the ten, and read in full for the
  four re-verified this pass.
- Distillation faithfulness: sampled in depth across all four re-verified
  projects (stranger-install-smoke, adr-0026-peer-ingest-and-transport,
  retained-set-instrument-and-conformance-mount, and the Context pack openers of
  the remaining 36 via their framing sentences). Every Context pack states
  load-bearing decisions inline - clause ranges and which ADR owns which clause,
  the cursor-shape probe's finding and why pull returns a batch not a stream, the
  scaffolding gap AC-UX-012 has to name, the never-empty-reason discipline reused
  from RS-40-5 - rather than pointing at an anchor and stopping. No hollow or
  intent-flattening core found in the sample.
- Signposted, AC-bound anchors: every anchor cited in the sampled Context packs
  carries both a stated reason it is load-bearing and a citation resolvable to a
  real file:line or clause id, and each section's anchors bind to the AC row(s)
  they serve, stated explicitly in the "Coverage of the traced project AC"
  closing paragraph every spec in this batch carries.
- Design coverage: not applicable for three of the four re-verified projects
  (closeout-and-durable-audience, replication-identity-and-ingest,
  retention-and-incomplete-logs carry no ux brief and their _design.md files
  declare no surfaces - a correct skip, not an absence). For
  publication-and-positioning, all seven surfaces _design.md declares
  (crates-io-happenstance, crates-io-happenstance-core,
  crates-io-happenstance-testkit, docs-rs-happenstance,
  docs-rs-happenstance-core, docs-rs-happenstance-testkit, github-landing) are
  claimed as "Renders surfaces" by at least one story in that project
  (landing-copy-and-status-truth, compliance-claim-and-gaps-promise,
  guarantees-and-docs-rs-presentation, projection-port-ship-shape,
  rendered-page-preflight); stranger-install-smoke correctly declares it
  consumes crates-io-happenstance without owning it. No orphaned surface, no
  surface cited that _design.md does not declare.
- Composition ACs: not applicable. userFacing is false for the initiative and
  none of the ten projects' _design.md files declare an interactive composed
  surface in the sense that check targets (publication-and-positioning's seven
  surfaces are rendered registry/docs pages, not application UI, and their
  composition invariants - region order, transience, density, hierarchy,
  anti-patterns - are already carried as real AC table rows in the stories that
  render them, e.g. compliance-claim-and-gaps-promise's AC-004 and
  stranger-install-smoke's own AC-004 in its Interaction quality section, not
  left as prose bullets).
- Scope: git status --porcelain -- .bklg/from-contract-to-published-library
  prints one line - the whole initiative directory as untracked in this fresh
  worktree. No production code path, no frontmatter field and nothing outside
  the initiative's own tree was touched by this review. This fix pass stayed
  inside the same boundary: the five anchor edits landed in the bodies of two
  existing story spec.md files (below their frontmatter, which was not
  touched), and this file's own body was rewritten; git status --porcelain
  still prints the same single untracked-directory line, confirmed after the
  edits.
