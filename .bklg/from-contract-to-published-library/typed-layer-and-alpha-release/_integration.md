---
item: HS-P0011
title: "Integration — The typed layer, the worked example, and 0.2.0-alpha.1"
initiative_slug: from-contract-to-published-library
project_slug: typed-layer-and-alpha-release
terminal: false
stage: integration
created: 2026-08-16
updated: 2026-08-16
dod_green: true
reachability_ok: true
deferred_scenarios:
  - "DoD 3 — the durable store passes event_store_conformance! for real"
  - "DoD 4 — the constrained-runtime store passes the suite on its own wasm32 target"
  - "DoD 5 — a store that does not serialise its writers passes the suite"
  - "DoD 6 — a store with no connection, no interactive transaction and no cursor passes the suite"
  - "DoD 7 — the projection suite discriminates across two unlike batch shapes"
  - "DoD 8 — the ProjectionStore freeze verdict is written"
  - "DoD 9 — @smoke a stranger can install it from the registry"
  - "DoD 10 — the published crate looks finished (rendered docs and registry page)"
  - "DoD 11 — the release is diffed against the prior published baseline"
  - "DoD 12 — the clause ledger is audited at publish"
  - "DoD 13 — cargo xtask ci green on the assembled whole that was published"
  - "DoD 14 — replication has an answer on disk"
  - "DoD 15 — incomplete logs have an answer on disk"
  - "DoD 16 — the audience is durable (persona and journey atoms)"
---

# Integration — The typed layer, the worked example, and 0.2.0-alpha.1

## Verdict

**dod-green**, at this project's own integration bar, and *only* at that bar.

This is a **non-terminal (feature) project**, so the whole-initiative Definition of
Done was deliberately not run whole: fourteen of its sixteen items need an adapter,
a freeze verdict, a semver baseline diff or a closeout artefact that later projects
own, and running them here would fail by design. What was proved instead is the pair
this project is answerable for — every capability it delivered is **mounted and
reachable**, and its **declared integration gate is green**.

Two things are worth saying out loud rather than leaving to be inferred from a green
tick.

- **The two whole-initiative DoD items this project owns outright — DoD 1 and
  DoD 2 — were executed here and both passed.** They are listed under the project
  bar below, not among the deferrals. The terminal project (HS-P0019) re-observes
  them on the assembled whole; that re-observation is its DoD 13, not a substitute
  for this execution.
- **A green gate is a precondition, never a substitute.** `cargo xtask ci --fast`
  exiting 0 is consistent with the worked example never having been run as a
  *binary* and with a compile-fail fixture that fails for a reason nobody read. Both
  were therefore run and read directly: the binary's transcript is quoted below, and
  the snapshot's `error[E0004]` and the one-line delta between the two fixtures were
  diffed by hand.

## Project integration bar

Every scenario this project owns, executed. No row is `fixme`, `#[ignore]`d, or
flag-gated off; there is no `#[ignore]` attribute anywhere in `crates/`,
`examples/`, `experiments/` or `xtask/`.

| # | Scenario | Command | Result |
| --- | --- | --- | --- |
| PB-1 | **Initiative DoD 1 / AC-003 — @smoke, the worked example runs end to end on typed events, no `todo!()` reached** | `cargo run -p course-subscriptions` | **executed, PASS** |
| PB-2 | **Initiative DoD 2 / AC-002 — @smoke, the compiler protects the domain, with its negative control** | `cargo test -p course-subscriptions --all-features --test ui` | **executed, PASS** (2 passed, 0 failed) |
| PB-3 | Project DoD 6 — the declared non-terminal integration bar (`.redkiln/config.yaml`, `verify.integration_scoped`) | `cargo xtask ci --fast` | **executed, PASS** (exit 0; `all required checks passed (--fast: 4 optional step(s) not run)`) |
| PB-4 | Project DoD 6 — the specification's cross-references | `cargo xtask spec-trace` | **executed, PASS** (exit 0; `traceability: no problems found; §7.1–§7.2 matches the checker`) |
| PB-5 | Reachability tripwire (`.redkiln/config.yaml`, `verify.reachability_static`) | `cargo xtask lints` | **executed, PASS** (all seven file-reading lints green) |
| PB-6 | Project DoD 3 / AC-016 — ADR-0020 and ADR-0021 are accepted atoms and the KB validates | `redkiln validate --kb` | **executed, PASS** (`validate passed`) |
| PB-7 | Project DoD 7 / AC-011 — `0.2.0-alpha.1` resolves from the registry | crates.io versions API for all three publishable crates | **executed, PASS** |
| PB-8 | Project DoD 2 — the proof artefacts still hold the tests their clauses name | `cargo xtask proof-artefact` (a REQUIRED step inside PB-3) | **executed, PASS** |

### PB-1, read rather than assumed

The binary's transcript, from this audit's run: a definition; an idempotent refusal
(*"course c1 is already defined"*); two capacity-bounded admissions; a
double-subscription refusal (*"student s1 is already subscribed to c1"*); a capacity
refusal (*"course c1 is full (2/2)"*); a release; and a re-admission into the freed
seat — then a five-event log at positions 1..5, each event carrying its tags.

That is the canonical DCB cycle. The two middle refusals are what makes AC-004 more
than a type-level claim: they can only both fire if the `(Seats, StudentSeat)` tuple
at `examples/course-subscriptions/src/main.rs:457` really did OR two models into one
query and fold both. `todo!(` appears zero times in `crates/happenstance/src`,
`crates/happenstance-core/src`, `crates/happenstance-testkit/src` and
`examples/course-subscriptions/src`, and `format!(…).into_bytes()` and
`parse_capacity` appear zero times in the example.

### PB-2, read rather than assumed

`examples/course-subscriptions/tests/ui/unhandled_variant.stderr` is a real
`error[E0004]: non-exhaustive patterns` naming `Enrolment::CourseClosed` and
pointing at `tests/ui/unhandled_variant.rs:88` — the fold the application author
owns, not a macro body and not `crates/`. The negative control is one line different
(`diff` of the two fixtures returns exactly
`> Enrolment::CourseClosed => self.capacity = None,`) and **compiles**, which is what
makes the red build mean the stated thing rather than a typo. Both test names are
held out of `cargo test -- --list` by `xtask/src/proof.rs:236` before they are run,
so emptying `tests/ui.rs` fails the gate rather than passing it — the phase-4 failure
mode (`RUNBOOK.md:3614-3627`) is closed by construction.

## Deferred to terminal project

Whole-initiative Definition-of-Done journeys this project does **not** own. Each
needs substrate a later project builds; running it here would fail by design, and
none counts against this project's bar. The terminal project is
`closeout-and-durable-audience` (HS-P0019, `terminal: true`).

| Initiative DoD | Why it cannot pass here | Owner |
| --- | --- | --- |
| 3 — the durable store passes the suite for real | every store crate is still a `todo!()` skeleton | `sqlite-durable-store` (HS-P0012) |
| 4 — the constrained-runtime store passes the suite on `wasm32` | `happenstance-cloudflare` is a skeleton. This project's fifth wasm32 step proves the *typed layer builds* on that target, which is a different claim | `cloudflare-durable-object-store` (HS-P0013) |
| 5 — a store that does not serialise its writers passes the suite | `happenstance-postgres` is a skeleton | `postgres-and-neon-stores` (HS-P0014) |
| 6 — a store with no connection, transaction or cursor passes the suite | `happenstance-neon` is a skeleton | `postgres-and-neon-stores` (HS-P0014) |
| 7 — the projection suite discriminates across two unlike batch shapes | HS-P0010's bar; this project *consumes* the frozen port and adds no batch shape | `projection-store-freeze` (HS-P0010) |
| 8 — the freeze verdict is written | needs the graph-shaped third batch shape; explicitly out of scope per `project.md`, *Out of scope* | `ladybug-projection-store` (HS-P0015) |
| 9 — @smoke a stranger can install it from the registry | this project *creates* the first registry artefact and makes no promise about it; the stranger journey is BR-09's | `publication-and-positioning` (HS-P0016) |
| 10 — the published crate looks finished | registry presentation and positioning are HS-P0016's; see F-3 | `publication-and-positioning` (HS-P0016) |
| 11 — the release is diffed against the prior published baseline | this project *is* the baseline; there is nothing prior to diff against | `publication-and-positioning` (HS-P0016) |
| 12 — the clause ledger is audited at publish | a publish-time run over the whole specification, at the stable number | `publication-and-positioning` (HS-P0016) |
| 13 — `cargo xtask ci` green on the exact tree that was published | the release bar, on the assembled whole, is the terminal project's | `closeout-and-durable-audience` (HS-P0019) |
| 14 — replication has an answer on disk | the ingest decision belongs to a different port | `replication-identity-and-ingest` (HS-P0017) |
| 15 — incomplete logs have an answer on disk | needs a store holding only a suffix of its own log | `retention-and-incomplete-logs` (HS-P0018) |
| 16 — the audience is durable | persona and journey atoms are the closeout's | `closeout-and-durable-audience` (HS-P0019) |

## Reachability

Every capability this project delivered, traced to the place it is actually reached
from — not to a passing unit test. For a library the composition root is the
**published item graph plus the gate that compiles and runs it**, so a capability
counts as mounted when a `cargo xtask ci` step reaches it through a real consumer,
or when the published crate exposes it by path.

| Capability (story) | Mount point | Reachable? |
| --- | --- | --- |
| `adr-0020-fold-query-agreement` | `.kb/decisions/0020-fold-query-agreement.md` — `kind: decision`, `status: accepted`. Its decision is *executed* by the sealed `Boundary` at `crates/happenstance/src/boundary.rs:69` and by `DecisionModel` at `crates/happenstance/src/domain.rs:168` | **Yes.** `redkiln validate --kb` checks every accepted atom against `HEAD` and passed; the sealed trait means no downstream type can supply a query that disagrees with its fold |
| `adr-0021-payload-evolution-and-codec-tag` | `.kb/decisions/0021-payload-evolution-and-codec-tag.md`, `status: accepted`. Sited in code as the framing region inside `Event::metadata` | **Yes.** Exercised behaviourally, not by fixture: `crates/happenstance/tests/codec_tag.rs` proves two encodings coexist in one store, that application metadata after the region survives, that an untagged event still decodes with the codec in hand, and that an unknown tag is a typed refusal |
| `domain-event-and-decision-model` | `crates/happenstance/src/domain.rs:61` (`DomainEvent`), `:168` (`DecisionModel`), re-exported at `crates/happenstance/src/lib.rs:229` | **Yes.** Consumed by a real application: `examples/course-subscriptions/src/main.rs:34-37` imports both by their public path, and the binary ran |
| `decision-model-composition` | `crates/happenstance/src/composition.rs:24` (the internal `impl_boundary_for_tuple!`), `:95` onward (arities 2 through 8), re-exported as `Boundary` at `crates/happenstance/src/lib.rs:214` | **Yes.** Mounted through a caller, not a test: `examples/course-subscriptions/src/main.rs:457` builds a `(Seats, StudentSeat)` tuple, and both of that tuple's refusals were observed at runtime in PB-1. No macro invocation reaches the caller, which is the DT-2 resolution `_design.md:639` records |
| `codec-and-feature-forwarding` | `crates/happenstance/src/codec.rs:39` (`Codec`), `:80` (`CodecError`), `:142` (`Json`), re-exported at `crates/happenstance/src/lib.rs:215-224`, one `cfg` per codec | **Yes, and published.** `crates/happenstance/Cargo.toml` forwards `json` (default), `cbor`, `postcard`, `serde`, `memory`, `std` and `unstable-projection`; the crates.io versions API for `happenstance` 0.2.0-alpha.1 returns exactly that feature map, so the forwarding survived packaging |
| `command-loop` | `crates/happenstance/src/command.rs:219` (`commit`), `:265` (`commit_with`), re-exported at `crates/happenstance/src/lib.rs:227-228` | **Yes.** Three real call sites in `examples/course-subscriptions/src/main.rs:425`, `:459`, `:499`; the `ConditionViolated` retry path is driven against `FaultyStore` in `crates/happenstance/tests/retry_without_a_database.rs` and spawned onto a work-stealing runtime in `crates/happenstance/tests/flavours.rs` |
| `misbehaving-testkit-stores` | `crates/happenstance-testkit/src/faulty.rs` and `crates/happenstance-testkit/src/gappy.rs`, re-exported at `crates/happenstance-testkit/src/lib.rs:337` (`FaultyStore`, `FaultyStoreError`, `SendFaultyStore`) and `:340` (`GappyMemoryStore`) | **Yes, cross-crate.** Consumed from outside the testkit by `crates/happenstance/tests/retry_without_a_database.rs`, `crates/happenstance/tests/command_loop.rs`, `crates/happenstance/tests/projection_runner.rs` and `experiments/polling-cost/tests/staleness.rs`. Both also carry their own conformance and instrument targets inside the testkit, so a wrapper that stopped misbehaving fails a named test |
| `given-when-then-dsl` | `crates/happenstance/src/testing/mod.rs:100` (`given`), `:114` (`Given`), `:269` (`Decision`), `:378` (`assert_domain_event`), mounted by `pub mod testing` at `crates/happenstance/src/lib.rs:209` under `memory` plus `json`, **both default features** | **Yes, and the mount is itself asserted.** `crates/happenstance/tests/mounted_at_the_crate_root.rs:16` names the items through their public `happenstance::testing` path as values *and* as types, so neither a missing `pub` nor a missing `pub mod` can pass; `:120` asserts the crate-root page links the module in the region `_design.md` fixes |
| `projection-trait-and-runner` | `crates/happenstance/src/runner.rs:62` (`Projection`), `:416` (`run_projection`), re-exported at `crates/happenstance/src/lib.rs:233-235` behind `unstable-projection` | **Yes, and off by default *by decision* rather than by omission.** The gating is decided on the record at `_design.md:647` and `:746` — *"the port beneath is provisional; the feature name is the promise"* — which is exactly what `project.md`'s risk table demanded instead of an inherited gate. It is compiled and run by the gate's `cargo test --locked --workspace --all-features` step, doc-built by `[package.metadata.docs.rs] all-features = true`, and the feature is present on the published 0.2.0-alpha.1 manifest |
| `projection-clause-verdicts` | `spec/SPECIFICATION.md:5273` (PS-18), `:5568` (PS-27), `:5670` (PS-30), each carrying an *"evaluated at the typed layer's phase exit"* verdict with its count and routing; and `:9086-9088`, where PS-32, PS-33 and PS-35 move to prose with their IDs retained | **Yes.** Held by a gate step rather than by prose: `cargo xtask spec-trace` resolves markers, rule names, case numbers and citations across 201 clauses and 389 citations and reports `no problems found`, so a verdict cannot rot into decoration unnoticed. PS-18, PS-27 and PS-30 stay `[DEFERRED]` with a *named owner* (HS-P0010) rather than an empty falsifier: the count came back, and it is zero or unavailable for stated reasons |
| `polling-cost-measurement` | `experiments/polling-cost/README.md` (the argument and the tables) and `experiments/polling-cost/results/pass-001/records.ndjson` (the raw run, with a rustc, cargo, git-rev, profile, CPU and RAM manifest) | **Yes, as a measurement rather than an estimate.** The recorded number is 32 deliveries of each event and 64 reads at the measured fan-out, with the conditions attached. Deliberately **outside** the workspace and the gate — `Cargo.toml` lists `crates/*`, `examples/*` and `xtask` as members — which is CLAUDE.md's own rule for `experiments/`, not an unmounted deliverable |
| `worked-example-on-typed-layer` | `examples/course-subscriptions/src/main.rs`, a workspace-member binary | **Yes.** Executed as a binary in PB-1, and regression-held: `examples/course-subscriptions/tests/runs.rs` is an `ARTEFACTS` row naming `runs::the_binary_completes_the_dcb_cycle` and `runs::the_transcript_is_the_designed_composition` (`xtask/src/proof.rs:215`, `:273`), asserted out of the test listing before they run |
| `compile-fail-proof-artefact` | `examples/course-subscriptions/tests/ui.rs` plus `examples/course-subscriptions/tests/ui/unhandled_variant.rs`, `unhandled_variant.stderr` and `handled_variant.rs` | **Yes, and it is in the gate.** Executed in PB-2. Held by name at `xtask/src/proof.rs:236` (`COMPILE_FAIL_PAIR`) and `:279`, so the pair cannot be reduced to one test or emptied silently. `trybuild` rather than a `compile_fail` doctest, for the reason `examples/course-subscriptions/tests/ui.rs:8-22` states: rustdoc collects doctests from the lib target only, this package has no lib target, and 1.97.1 silently ignores an unmatched error-code annotation, so under a doctest the negative control could not discriminate |
| `edge-flavour-and-wasm-claim` | The fifth wasm32 step, `wasm32 build of the typed layer`, at `xtask/src/main.rs:319`, selected **by name** in `wasm_steps()` and living in `REQUIRED` | **Yes, and it ran in PB-3.** `--fast` drops `OPTIONAL` only, so all five wasm32 steps executed. The bans are executable rather than conventional: `crates/happenstance/tests/manifest_contract.rs:235` (`async_trait_is_banned`) reds the gate if the attribute arrives; it appears in zero manifests and zero non-comment lines; `SendEventStore` appears zero times in `crates/happenstance/src`; and both read-shape tests survive at `crates/happenstance-core/src/memory.rs:614` and `:643` |
| `defect-log-and-macros-verdict` | `.kb/_intake/contract-defect-log-phase-7.md` — five claims, C1 through C5, each naming a clause ID (VT-18 twice, CF-36, CF-38, and the projection port family) and a routing, plus two findings explicitly recorded as *not* claims; and `references/evaluation/phase-7-macros-verdict.md:173`, whose verdict is that `happenstance-macros` is **OUT** of scope for 0.1, with the prediction recorded before the count and the count shown | **Yes.** Non-speculative: every claim carries `file:line` call sites and asks the ingest wave for a decision record rather than a line edit, which is exactly the AC-012 discipline (`project.md`, *Out of scope*: a defect is recorded and routed, never fixed by editing a frozen clause). The verdict document contradicts its own recorded prediction by name, which is what makes it a measurement rather than a preference |
| `publish-0-2-0-alpha-1` | crates.io: `happenstance` (id 3012640), `happenstance-core` (id 3012639) and `happenstance-testkit` (id 3012642), all at `0.2.0-alpha.1`, all `yanked: false`, published 2026-08-16T21:36-21:37Z | **Yes, verified against the registry API rather than against a transcript.** The three mitigations are present: `crates/happenstance/README.md:15` carries `## Stability`; `CHANGELOG.md` exists; the yank policy is stated at `crates/happenstance/README.md:20` and `CHANGELOG.md:29`. The gate's *packaged artifacts carry their licences and README* step confirmed all three crates ship both licence files and a README (26, 33 and 48 files). `happenstance-testkit` carries its **own** `version` key (CF-32), asserted by a lint step in PB-5 |

**Nothing is constructed-but-unmounted, exported-but-unconsumed, or reachable only
through a test that nothing holds.** The two capabilities that could most easily
have been are each held by a purpose-built mount check that was executed: the
`testing` module by `crates/happenstance/tests/mounted_at_the_crate_root.rs`, and the
compile-fail pair by the two `ARTEFACTS` rows in `xtask/src/proof.rs`.

## Missing dependencies

**None at this project's bar.**

Nothing this project should have provided is absent. There is no `#[ignore]`
attribute in `crates/`, `examples/`, `experiments/` or `xtask/`; no `todo!()` in the
typed layer, the contract crate, the testkit or the worked example; no scenario
gated off behind a flag; and no double, no-op seam or injected stub standing in for
a real contract. `MemoryEventStore` is the *specified* substrate for this phase
(`RUNBOOK.md:3977-3979`), not a stand-in for a store that was supposed to exist.

The one dependency this project's risk table flagged as possibly unavailable —
`trybuild`, whose adoption was HS-P0010's decision to make — **landed**, so AC-002's
instrument is the strong one rather than the measurably weaker substitute.

## Findings

Recorded and routed. None makes a delivered capability unreachable, none reds the
declared integration gate, and none blocks this project's bar — but each is
something the next reader should meet as a note rather than as a surprise.

**F-1 — three story items' frontmatter never left `plan`, although their slice is
sealed `approved`.** `publish-0-2-0-alpha-1`, `edge-flavour-and-wasm-claim` and
`defect-log-and-macros-verdict` each carry `status: ready` and `stage: plan` in
their `story.md`, while `_slices.md`'s verdict table records the `alpha-release`
slice as `approved` with all three story checkpoints (`0a010c9`, `344f2c0`,
`448e1ac`), and commit `ab1cb21` seals it. All three have a full artefact set —
`spec.md`, `_ledger.md`, `implementation-report.md`, `report.md` — and their work is
on disk and green. This is **backlog bookkeeping, not a capability gap**: the CLI is
the only writer of those fields and this audit does not touch them. Flagged because
`redkiln status` will under-report this project until the transitions are recorded.

**F-2 — ADR-0031 exists only as staged intake, so ADR-0007's supersession is not yet
in the decision graph.** PS-33's verdict *is* written and is unambiguous
(`spec/SPECIFICATION.md:9087`: the falsifier fired, `happenstance-core` publishes
two free functions and neither is a checkpoint pump, so the pump collapses upward
and ADR-0007 is superseded), and the superseding record is authored at
`.kb/_intake/0032-adr-0031-the-runner-collapses-upward.md`. But `.kb/decisions/`
stops at `0030`, so a reader following the supersession graph out of
`.kb/decisions/0007-projection-runner-decodes.md` will not find 0031 until the
ingest wave runs. This is the **documented authoring route**, not a shortcut:
CLAUDE.md is explicit that atoms are authored by `/redkiln:kb-ingest` from
`.kb/_intake/` and never by hand, the specification's own row calls the correction
*"staged"* rather than done, and `redkiln validate --kb` passes on the graph as it
stands. Project DoD 3 requires only ADR-0020 and ADR-0021 to be accepted atoms, and
both are. Carried to the next human-invoked ingest wave.

**F-3 — every documentation link on the published crates.io page 404s.** Recorded in
`publish-0-2-0-alpha-1/_release-log.md` and routed to HS-P0016, whose PR boundary
already names registry presentation as its own. The cause is that the GitHub
repository is not publicly reachable, so a new version number contains the identical
strings and fixes nothing, which is why this is *not* EC-007's yank. It belongs to
initiative DoD 10 (*"the published crate looks finished"*), which is deferred above;
this project's AC-011 asks only that the version resolve and carry its three
mitigations, and it does.

**F-4 — a green `spec-trace` is weaker evidence than CF-36's own `Rule:` line
claims.** This is the defect log's C3, self-reported by this project:
`spec/SPECIFICATION.md:8611-8622` says `cargo xtask spec-trace` cross-references
each case's level marker, and `xtask/src/spec_trace.rs` performs no such check.
Routed to the ingest wave for a decision record — implement the cross-reference, or
supersede CF-36 — which is correct, since CF-36 is `[FROZEN]` and a line edit is
forbidden here. It does not weaken this audit's verdict, because PB-1 and PB-2 were
executed and read directly rather than inferred from `spec-trace`.

**F-5 — `spec-trace` reports two conformance rules claimed by no clause and owing a
decision**: `k_disjoint_boundaries_admit_exactly_k_commits` and
`ops_agree_with_the_model`. Pre-existing, contract-crate scope, reported
informationally with exit 0. Not this project's to settle and not counted against it.

**F-6 — one disclosed deviation from the signed-off `_design.md`, with no code
owed.** `commit` and `commit_with` ship `B: Boundary + Clone` where the design
writes `B: Boundary`, because `Boundary` is sealed and cannot gain a `Clone`
supertrait without editing a frozen surface. Disclosed at the time in
`command-loop`'s reports and carried in `_slices.md`'s *Deviations* table; it costs a
caller nothing, since every `DecisionModel` is already `Clone` and tuples of `Clone`
are `Clone`.

## Integration verdict

**dod_green: true** — at the non-terminal bar defined for this project, with the
whole-initiative journeys it does not own deferred above and their owners named. The
two it *does* own, initiative DoD 1 and DoD 2, were executed here and passed.

**reachability_ok: true** — all sixteen delivered capabilities trace to a real mount
point: a `pub use` at `crates/happenstance/src/lib.rs`, a published feature on
crates.io, a call site in the worked example, a `REQUIRED` gate step selected by
name, an accepted KB atom, or a clause a gate step resolves. None is reachable only
through a test that nothing holds.

Evidence, all executed during this audit: `cargo xtask ci --fast` exit 0;
`cargo xtask spec-trace` exit 0; `cargo xtask lints` all seven green;
`redkiln validate --kb` passed; `cargo run -p course-subscriptions` completed the
DCB cycle with no `todo!()` reached;
`cargo test -p course-subscriptions --all-features --test ui` 2 passed, 0 failed;
and the crates.io versions API returns `0.2.0-alpha.1`, `yanked: false`, for all
three publishable crates.
