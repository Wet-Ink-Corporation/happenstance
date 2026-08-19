---
item: HS-P0011
stage: review
title: "Review — The typed layer, the worked example, and 0.2.0-alpha.1"
initiative_slug: from-contract-to-published-library
project_slug: typed-layer-and-alpha-release
terminal: false
created: 2026-08-16
updated: 2026-08-16
overall: 3
dod_green: true
rubric:
  ac-coverage: 3
  integration-reachability: 3
  test-integrity: 3
  gate-greenness: 3
  brief-fidelity: 3
  intent-fidelity: 3
  presentation-fidelity: 0
---

# Review — The typed layer, the worked example, and 0.2.0-alpha.1

- [x] Every project acceptance criterion (AC-001 – AC-016) is met by real, reachable, committed behaviour
- [x] Every delivered capability is mounted into a real consumer the gate reaches — nothing constructed-but-unmounted
- [x] No test gutted, weakened, skipped, `#[ignore]`d, flag-gated off, or replaced by a double
- [x] The affected-package gate is genuinely green, formatter included — re-run here at HEAD `c791b12`
- [x] This project's applicable Definition-of-Done bar is green (`_integration.md`, `dod_green: true`)
- [x] `0.2.0-alpha.1` re-verified live against the crates.io versions API during this review, not read from a transcript
- [ ] Presentation reviewed — **DOES NOT APPLY and was NEVER OBSERVED**; see the Rubric note

## Verdict

**approved**, at HEAD `c791b12`.

I tried to break this on the four axes that usually break a project of this size —
an AC met by a fixture instead of by behaviour, a deliverable built but never
mounted, a test weakened to make a slice go green, and a gate whose green means
less than it claims — and none of them gave. The specifics are in *Evidence*; the
short version is that the three capabilities most likely to be faked here are each
held by a purpose-built check that a wrong implementation fails **by name**: the
compile-fail pair by `xtask/src/proof.rs:236` (which asserts both test names out of
`cargo test -- --list` *before* running them, so emptying `tests/ui.rs` reds the
gate rather than passing it), the worked example by `xtask/src/proof.rs:215` (the
only thing in the tree that actually executes `main`), and the `testing` module's
mount by `crates/happenstance/tests/mounted_at_the_crate_root.rs:16`, which names
each item through its public `happenstance::testing` path as a value *and* as a
type, so neither a missing `pub` nor a missing `pub mod` can pass.

Three things are worth saying out loud because a reader could otherwise mistake
them for escape hatches, and none of them is:

- **`unstable-projection` is off by default, and that is a decision rather than an
  omission.** `project.md`'s risk table demanded the runner's gating be *stated*
  rather than inherited from PS-3; `_design.md:647` states it, the manifest carries
  it, `crates/happenstance/tests/manifest_contract.rs:181`
  (`unstable_projection_is_declared_off_by_default`) asserts it, the gate compiles
  and runs it through `cargo test --locked --workspace --all-features`, and the
  published 0.2.0-alpha.1 feature map returned by the registry API contains it. An
  off-by-default feature that a `--all-features` gate step drives and a published
  manifest advertises is reachable; an item nothing compiles would not be.
- **`experiments/polling-cost` sits outside the workspace and outside the gate.**
  That is CLAUDE.md's own rule for `experiments/` ("measurements. reproducible, and
  not in the gate"), and AC-010 asks for a recorded number, not a gate step. The
  number is recorded with a machine manifest, so it is a measurement.
- **The two `ignored` doctests the test run reports are pre-existing counterexample
  fences in the Rust constitution**, not skipped work:
  `standards/rust/62-doctests-and-harnesses.md:212` carries an explicit
  `ignore: this fence is the counterexample` marker that
  `xtask/src/lint_constitution.rs` enforces. The diff touches those files only to
  re-anchor citation line numbers.

Nothing in this project is a stub, a no-op seam, an injected double or a
fixture-pinned answer. `git diff --diff-filter=D 74135b8...HEAD` is **empty** — not
one file was deleted — and the only two files with substantial removals are
`crates/happenstance/src/lib.rs` (the five-line facade this project exists to
replace) and `examples/course-subscriptions/src/main.rs` (the rewrite AC-003 asks
for).

## Rubric

Each dimension scored 0 (absent) → 3 (excellent). `approved` requires every dimension >= 2, with
`gate-greenness` = 3, `integration-reachability` = 3, `intent-fidelity` >= 2, and this project's
applicable Definition-of-Done bar green.

`presentation-fidelity` is the single exception to "every dimension >= 2", and the exemption is narrow.
Here the design review **does not apply**: `.redkiln/config.yaml` declares no `design.capture` command —
there is no `design:` block at all, and its absence is deliberate (`config.yaml:75-82`, *"there is no app
to screenshot"*) — so the six surfaces `_design.md` declares cannot be rendered or perceived, and
`require_design_review` is likewise absent, so `blocking = false`. It scores **0**, and it is exempt from
the BAR, never from the RECORD.

| Dimension | Score | Rationale |
| --------- | ----- | --------- |
| ac-coverage | 3 | All sixteen project ACs met by real, reachable, committed behaviour with a test behind each — the per-AC map is in *Evidence*. The two that most often go soft did not: AC-002's negative control is a *second named test* the gate holds by name (`xtask/src/proof.rs:236`), and AC-011 was re-verified against the crates.io versions API during this review (`happenstance` id 3012640, `0.2.0-alpha.1`, `yanked: false`, feature map intact through packaging). AC-007 and AC-008 are verdicts rather than code, and both are written into `spec/SPECIFICATION.md` in a form `cargo xtask spec-trace` resolves rather than as prose that can rot. |
| integration-reachability | 3 | All sixteen capabilities trace to a real mount: `pub use` at `crates/happenstance/src/lib.rs:214-235`, a `pub mod testing` at `:209` whose mount is itself asserted, three call sites in the worked-example binary (`examples/course-subscriptions/src/main.rs:425`, `:459`, `:499`), a `REQUIRED` gate step selected **by name** (`xtask/src/main.rs:319`, the fifth wasm32 step — I confirmed all five executed in my own `ci --fast` run), accepted KB atoms, and a live published feature map. `_integration.md` is `dod_green: true` / `reachability_ok: true`; zero rows `fixme`, zero `#[ignore]` attributes in `crates/`, `examples/`, `experiments/` or `xtask/`. The fourteen whole-initiative DoD journeys under `deferred_scenarios` are the terminal project's and are not held against this bar. |
| test-integrity | 3 | Nothing deleted (`git diff --diff-filter=D 74135b8...HEAD` returns nothing), nothing weakened. Wrong implementations are written down and rejected by name across the diff: `composition.rs::a_narrowing_union_is_rejected` and `::routing_by_arrival_is_rejected`, `projection_runner.rs::checkpoint_without_rows_is_rejected`, `gappy_store_instruments.rs::handler_assuming_position_plus_one_disagrees_with_the_store`, `ui::the_negative_control_compiles`. The one gate relaxation — `deny.toml`'s `allow-wildcard-paths = true` — is scoped to *path* dependencies, explained in-file, and lands in the same hunk as a **new** `async-trait` ban with a by-name wrapper exemption, so the net movement is stricter, not looser. |
| gate-greenness | 3 | Re-executed by me at HEAD `c791b12`, not carried from a report. `cargo fmt --all --check` exit 0. Scoped clippy over all twelve affected packages with `-D warnings` exit 0. Scoped `cargo test --locked --all-features` over the same twelve: **0 failed** across every target. Then the project's own declared bar, `cargo xtask ci --fast`, exit 0 — `all required checks passed (--fast: 4 optional step(s) not run)` — with all five wasm32 steps, `spec-trace`, `proof-artefact`, the seven file-reading lints and the three-configuration doc builds among the 23 steps executed. |
| brief-fidelity | 3 | No Accepted KB decision is deviated from. ADR-0001: zero `#[async_trait]`, banned in `deny.toml` and asserted by `manifest_contract.rs:235`. ADR-0003 as scoped by ADR-0006: `happenstance-core`'s `default = ["std", "memory"]` is unchanged and `serde` stays opt-in, while the typed layer takes the `serde` dependency the split exists to allow. ADR-0007's falsifier is *evaluated*, not inherited. CLAUDE.md constraints 3 and 4 hold: both read-shape tests survive at `crates/happenstance-core/src/memory.rs:614` and `:643`, and `SendEventStore` appears **zero** times in `crates/happenstance/src`. `_design.md`'s *Signatures* block shipped as written but for one disclosed row (`+ Clone`), escalated in `_slices.md`'s *Deviations* table exactly as the design's risk table requires. Its *Anti-patterns* list is not prose — items 5, 6 and 14 are machine-checked by `doc_surface.rs::new_identifiers_fit_the_item_table`, `::every_gated_item_carries_its_badge` and `::no_contract_name_is_shadowed`. |
| intent-fidelity | 3 | The interaction-quality invariants are met in this medium's idiom and are *tested*, which is rarer than meeting them. **In place, no context jump:** `dsl_failure_message.rs::panic_location_is_the_callers_line` puts the assertion failure on the caller's own line, and `examples/course-subscriptions/tests/ui/unhandled_variant.stderr:19` points its `-->` at `tests/ui/unhandled_variant.rs:88` — the application author's own `match` arm, never a macro body or anything under `crates/` (`_design.md` anti-pattern 13). **Non-occlusion:** `::region_four_names_the_seeded_but_unselected_event` and `::overflow_truncates_selected_first_and_the_diagnosis_last` mean a filter never hides what it filtered, and truncation never eats the diagnosis — the encoded form of the hazard ADR-0020 exists to close. **Preserved state / reversibility:** `Given` and `Decision` are `#[must_use]` so a decision cannot be silently dropped; the command loop re-folds a *fresh clone* and never mutates the caller's value (`command.rs:289`); a failed chunk is rolled back whole (`runner.rs:495`); each superseded alpha is yanked. **Reachability of every control:** the three `cfg_attr` intra-doc link pairs at `lib.rs:149-176` mean no vocabulary entry becomes a dead link in any feature configuration — the design's own mock caught that before a line was written. |
| presentation-fidelity | 0 | **Presentation was NEVER OBSERVED.** No `_design-review.md` exists and none could: `.redkiln/config.yaml` declares no `design.capture` command (no `design:` block at all — deliberate per `config.yaml:75-82`, *"there is no app to screenshot"*), so the six surfaces `_design.md` declares — `crate-root-rustdoc`, `crate-readme`, `first-program-doctest`, `worked-example-transcript`, `dsl-failure-message`, `compile-fail-diagnostic` — were never rendered or perceived by anyone in this run. This score is the absence of evidence, not evidence of absence: I am explicitly **not** claiming those surfaces look right, only that several of their *encodable* properties are asserted in `doc_surface.rs`, `doc_budget.rs` and `docs_composition.rs`, which is a different and weaker claim than a human looking at the page. Scored 0 and exempt from the >= 2 bar. |

`intent-fidelity` scores BEHAVIOR from the diff; `presentation-fidelity` scores FORM from perceptual
evidence. They are separate on purpose, and the second one is 0 here because nothing looked at anything.

## Evidence

### Project acceptance-criteria coverage

| AC | Met by reachable behaviour? | Evidence |
| --- | --- | --- |
| AC-001 — the fold and the query cannot disagree | **Yes** | `crates/happenstance/src/boundary.rs:69` — `Boundary: crate::sealed::Sealed`, so a third impl cannot be written downstream; `:112` blanket-implements it for every `DecisionModel`; `query()` is **not** on `DecisionModel` (`crates/happenstance/src/domain.rs:168`), so there is nowhere to put a hand-maintained one. `absorb` asks the derived query itself (`boundary.rs:131-140`) rather than a second predicate. Held by `crates/happenstance/tests/composition.rs::the_seal_holds_for_tuples` and the `compile_fail` fence at `boundary.rs:34`. ADR-0020 is accepted at `.kb/decisions/0020-fold-query-agreement.md`. |
| AC-002 — the compiler protects the domain, and the protection is checked | **Yes** | `examples/course-subscriptions/tests/ui.rs`, `tests/ui/unhandled_variant.rs`, `unhandled_variant.stderr` (a real `error[E0004]: non-exhaustive patterns` naming `Enrolment::CourseClosed`), `tests/ui/handled_variant.rs`. Both names are held in `xtask/src/proof.rs:236` (`COMPILE_FAIL_PAIR`) and asserted out of `--list` before running, so an emptied file fails the gate rather than passing it. Executed in my `ci --fast` run under *each phase's proof artefacts*. |
| AC-003 — the worked example runs on the typed layer | **Yes** | `examples/course-subscriptions/src/main.rs` on `happenstance`; `parse_capacity` and the byte-payload construction appear **zero** times; zero `todo!()`. Executed as a binary — `examples/course-subscriptions/tests/runs.rs` is an `ARTEFACTS` row (`xtask/src/proof.rs:215`) naming `runs::the_binary_completes_the_dcb_cycle` and `runs::the_transcript_is_the_designed_composition`, the only thing in the tree that calls `main`. |
| AC-004 — consistency boundaries compose | **Yes** | `crates/happenstance/src/composition.rs:24` (internal macro; **no** exported `compose!`), arities 2..=8 at `:95-166`. Mounted through a caller, not a test: `examples/course-subscriptions/src/main.rs:457` builds a `(Seats, StudentSeat)` tuple, and PB-1's transcript shows both of that tuple's refusals firing — which they can only both do if the union query really OR'd two models and folded both. `tests/composition.rs::composed_query_is_the_union_of_member_queries`, plus two wrong-implementation controls. |
| AC-005 — payloads are typed and can evolve | **Yes** | `crates/happenstance/src/codec.rs:39` (`Codec::TAG`), `:142`/`:173`/`:199` (`Json` default, `Postcard`, `Cbor` behind features), `:258` (`frame`), `:324` (`decode_event`). Proved behaviourally rather than by fixture in `crates/happenstance/tests/codec_tag.rs`: `two_encodings_coexist_in_one_store`, `application_metadata_after_the_region_is_carried_through`, `an_untagged_event_decodes_with_the_codec_in_hand`, `unknown_tag_is_a_typed_refusal`. ADR-0021 accepted at `.kb/decisions/0021-payload-evolution-and-codec-tag.md`. |
| AC-006 — an application writes a projection against decoded events | **Yes** | `crates/happenstance/src/runner.rs:62` (`Projection`), `:416` (`run_projection`), re-exported at `lib.rs:233-235`. Runs against `happenstance_core::MemoryProjectionStore` — a real sibling contract from HS-P0010, not a double invented here — and `crates/happenstance/tests/projection_runner.rs::read_model_and_checkpoint_commit_together` plus `::checkpoint_without_rows_is_rejected` hold the one-commit invariant. Nominates with `Query` and nothing else (`doc_surface.rs::no_second_decode_path`). |
| AC-007 — ADR-0007's falsifier is evaluated, not inherited | **Yes**, with a carry | `spec/SPECIFICATION.md:9087` — the falsifier **fired**: `happenstance-core` publishes two free functions and neither is a checkpoint pump, so the pump collapses upward and ADR-0007 is superseded. The superseding record is authored at `.kb/_intake/0032-adr-0031-the-runner-collapses-upward.md`, which is CLAUDE.md's *documented* authoring route (atoms are authored by the kb-ingest command, never by hand — the first hand-written attempt was reverted at `0269720`). Carry: `.kb/decisions/0007-projection-runner-decodes.md` still reads `superseded_by: null` until the human-invoked ingest wave runs. See finding **N-2**. |
| AC-008 — the three "does anyone call this?" clauses are settled by counting | **Yes** | `spec/SPECIFICATION.md:5273` (PS-18 — the count came back *unavailable* rather than zero), `:5568` (PS-27 — the count is *zero*: the alpha's runner halts on the first failure and offers no failure-policy seam at all), `:5670` (PS-30 — the fan-out runner is not built, so the MUST binds nothing today). Each carries the count, the evaluation point and a **named owner** (HS-P0010) rather than an empty marker. `cargo xtask spec-trace` resolves the markers and citations and reported `no problems found` in my run. See finding **N-4** on the disposition. |
| AC-009 — a consumer can test a decision, and can test misbehaviour | **Yes** | `crates/happenstance/src/testing/mod.rs:100` (`given`), `:180` (`when`), `:378` (`assert_domain_event`); `crates/happenstance-testkit/src/faulty.rs` and `src/gappy.rs`, re-exported at the testkit root. Consumed **cross-crate** by `crates/happenstance/tests/retry_without_a_database.rs`, `tests/command_loop.rs`, `tests/projection_runner.rs` and `experiments/polling-cost/tests/staleness.rs`. The named wrong implementation exists and fails: `crates/happenstance-testkit/tests/gappy_store_instruments.rs:86` — `handler_assuming_position_plus_one_disagrees_with_the_store`. |
| AC-010 — the polling cost is a measurement | **Yes** | `experiments/polling-cost/README.md` — delivery amplification **32.00** at 32 overlapping views (3 200 000 deliveries of 100 000 distinct events) and **64 reads**, against **1.00** disjoint with the same 64 reads. `results/pass-001/records.ndjson` carries the machine manifest: rustc 1.97.1, cargo, git rev, `git_dirty: false`, release profile, CPU, 20 logical cores, 34 069 602 304 bytes RAM, run instant. A number with its conditions, not an estimate. |
| AC-011 — `0.2.0-alpha.1` is on the registry | **Yes**, re-verified live in this review | The crates.io versions API for `happenstance` returns id 3012640, `num: 0.2.0-alpha.1`, with the full forwarded feature map (`cbor`, `default: [std, memory, json]`, `json`, `memory`, `postcard`, `serde`, `std`, `unstable-projection`), so the forwarding survived packaging. The three mitigations: `crates/happenstance/README.md:15` `## Stability` (above the first code block, per anti-pattern 7), `CHANGELOG.md:29` with the alpha's own section, and the yank policy stated in both. The gate's *packaged artifacts carry their licences and README* step passed for all three crates (26 / 33 / 48 files). |
| AC-012 — contract defects found by use are recorded | **Yes** | `references/evaluation/phase-7-contract-defects.md` — D-1 and D-2 against **VT-18 `[FROZEN]`**, D-3 against **CF-36 `[FROZEN]`**, D-4 against **CF-38**, D-5 against the projection port family; each names the clause, its maturity marker, the call site, and the routing to a decision record. Two findings are recorded as explicitly *not* entries so silence is not read as a drop, and a reconciliation table gives every M2–M6 story that declared a routing an explicit disposition. Staged at `.kb/_intake/contract-defect-log-phase-7.md`. **No `[FROZEN]` clause was edited**: the diff touches `spec/SPECIFICATION.md` only for the verdicts AC-007 and AC-008 own. |
| AC-013 — the `happenstance-macros` question is answered either way | **Yes** | `references/evaluation/phase-7-macros-verdict.md` — verdict **OUT** for 0.1. 40 lines of mapping ceremony against 249 of domain; 125 : 249 even counting the whole contested identity block; threshold 1:1, so **both extremes are out**. The 29 classified ranges partition the file exactly (532 of 532), so the totals are re-derivable rather than asserted. It **contradicts the design's own recorded 2.4:1 prediction by name**, which is what makes it a measurement rather than a preference. `RUNBOOK.md:4079` ticked, decision-table row updated. |
| AC-014 — DT-2 is resolved on the record | **Yes** | `_design.md:631-651` *Shape decision* — eight rows resolve DT-2 explicitly (`Boundary::query`'s return type and the `const` that makes an empty `EVENT_TYPES` a compile error; `scope(&self) -> &Tags`; composition by tuple with **zero caller-visible syntax**; `commit` beside `commit_with`). `_design.md:1014` *The doctest* is the artefact it is judged on, and that program ships as the first fence at `crates/happenstance/src/lib.rs:24-62` with only the runtime wrapper and the closer hidden. |
| AC-015 — the edge flavour survives this layer | **Yes** | The fifth wasm32 step, `wasm32 build of the typed layer`, at `xtask/src/main.rs:319`, in `REQUIRED` and selected by name in `wasm_steps()`. I watched all five wasm32 steps execute in my `ci --fast` run (`--fast` drops `OPTIONAL` only). Zero `#[async_trait]` (banned in `deny.toml`, asserted by `manifest_contract.rs:235`); **zero** `SendEventStore` in `crates/happenstance/src`; `read` still returns the stream at the top level, with both pinning tests alive at `crates/happenstance-core/src/memory.rs:614` and `:643`; `crates/happenstance/tests/flavours.rs` drives every entry point against a genuinely `!Send` store. |
| AC-016 — ADR-0020 and ADR-0021 are written first | **Yes** | Both accepted atoms with valid KbFrontmatter (`kind: decision`, `status: accepted`, `adr_id: ADR-0020` / `ADR-0021`), landed in milestone **M1** — checkpoints `c011143` and `e33dc9f`, before every code checkpoint. Long records at `references/adr/0020-fold-query-agreement.md` (411 lines) and `references/adr/0021-payload-evolution-and-codec-tag.md` (499 lines). `redkiln validate --kb` passed at integration (`_integration.md`, PB-6). |

**Uncovered project ACs: none.**

### Gate and Definition-of-Done results

Re-executed by me at HEAD `c791b12`, in this worktree, scoped to the project's twelve affected packages —
not carried from a report.

| Check | Command | Result |
| --- | --- | --- |
| Formatter (non-negotiable) | `cargo fmt --all --check` | **exit 0** |
| Lint, affected-scoped | `cargo clippy --locked --all-targets --all-features` with twelve `-p` filters and `-- -D warnings` | **exit 0** |
| Tests, affected-scoped | `cargo test --locked --all-features` over the same twelve `-p` filters | **0 failed**, every target green. The only `ignored` count is 2, both pre-existing counterexample fences in the Rust constitution (see *Verdict*) |
| This project's integration bar | `cargo xtask ci --fast` | **exit 0** — `all required checks passed (--fast: 4 optional step(s) not run)`. 23 steps executed, including all **five** wasm32 steps, `each phase's proof artefacts`, `specification traceability`, the seven file-reading lints, three doc configurations, and the packaging assertion |
| Registry resolution | crates.io versions API, `happenstance` | **`0.2.0-alpha.1`**, `yanked: false`, feature map intact |

The twelve filters were `happenstance`, `happenstance-core`, `happenstance-testkit`, `course-subscriptions`,
`xtask`, `happenstance-cloudflare`, `happenstance-ladybug`, `happenstance-neon`, `happenstance-postgres`,
`happenstance-sqlite`, `happenstance-sync`, `outside-projection-adapter`. No Playwright or `*.spec.ts`
surface exists in this repository, so the collection-only pass does not apply.

**Definition of Done.** `_integration.md` records `dod_green: true` and `reachability_ok: true` at this
project's own non-terminal bar, with all eight owned scenarios (PB-1 through PB-8) executed and passing and
**no row `fixme`d, `#[ignore]`d or flag-gated off**. The two whole-initiative DoD items this project owns
outright — **DoD 1** (`cargo run -p course-subscriptions` completes the DCB cycle, no `todo!()` reached)
and **DoD 2** (the compile-fail case and its negative control) — were executed there and re-executed here
through `proof-artefact`. The fourteen journeys under `deferred_scenarios` each name the later project
that owns them and are **not** held against this bar; the terminal project is
`closeout-and-durable-audience` (HS-P0019).

**No out-of-band baseline repair occurred this run.** Three in-flight repair commits exist inside the
project's own range and are expected workflow behaviour, not scope drift: `26a3772` (repair gate for slice
codec-and-command-loop), `b77cffb` (repair three stale constitution citations) and `90421d0` (repair
contaminated baseline test).

### Escape-hatch, unmounted and scope-drift findings

**None blocking.** Specifically checked and clean:

- **No double, no-op seam or injected stub** stands in for a real contract. `MemoryEventStore` is the
  *specified* substrate for this phase (`RUNBOOK.md:3977-3979`), and `MemoryProjectionStore` predates this
  project (HS-P0010, `crates/happenstance-core/src/projection_memory.rs:101`) —
  `doc_surface.rs::no_local_projection_store` asserts this crate did not grow its own.
- **No fixture-pinned assertion.** The composition tests recompute the union from the members
  (`tests/composition.rs:188`, `union_of`) rather than comparing against a literal; the house rule against
  asserting literal position values is upheld by `gappy_store_instruments.rs`, which compares against
  positions the store actually assigned.
- **No unmounted deliverable.** Every one of the sixteen capabilities has a mount point outside its own
  tests, enumerated in `_integration.md`'s reachability table and spot-checked by me at the `pub use` block
  (`crates/happenstance/src/lib.rs:214-235`), the example's three `commit` call sites, and the registry's
  feature map.
- **No `#[ignore]`**, no fixme'd scenario, no flag-gated-off scenario anywhere in `crates/`, `examples/`,
  `experiments/` or `xtask/`.
- **`deny.toml`'s one relaxation is net-strengthening.** `allow-wildcard-paths = true` permits a wildcard
  only where it is a *path* dependency — a registry wildcard, the case the rule is actually for, still
  fails — and it arrives in the same hunk as a **new** `async-trait` ban whose sole exemption is by name
  (`wasm-bindgen-test`), so a second route into the graph reds the check.

Carried notes, none of which blocks and each of which the next reader should meet as a note rather than a
surprise. **N-1 through N-4 restate findings `_integration.md` already recorded (F-1, F-3, F-2, and the
AC-008 disposition); N-5 and N-6 are this review's own.**

- **N-1 — three story items' frontmatter never left `plan`.** I confirmed it directly:
  `publish-0-2-0-alpha-1/story.md`, `edge-flavour-and-wasm-claim/story.md` and
  `defect-log-and-macros-verdict/story.md` each carry `status: ready` and `stage: plan`, while `_slices.md`
  records the `alpha-release` slice `approved` with all three checkpoints and `ab1cb21` seals it. All three
  have a full artefact set and their work is on disk and green, so this is **backlog bookkeeping, not a
  capability gap** — and the CLI is the only writer of those fields, so neither `_integration.md` nor this
  review may touch them. `redkiln status` will under-report this project until the transitions are recorded.
- **N-2 — ADR-0031 exists only as staged intake.** `.kb/decisions/` stops at `0030`, so a reader following
  the supersession graph out of `.kb/decisions/0007-projection-runner-decodes.md` (still
  `superseded_by: null`) will not find 0031 until the human-invoked ingest wave runs. This is the documented
  route, not a shortcut, and the accepted atom was correctly left **unedited** rather than amended in place.
  Project DoD 3 asks only for ADR-0020 and ADR-0021 as accepted atoms, and both are.
- **N-3 — every documentation link on the published crates.io page 404s**, because the GitHub repository is
  not publicly reachable. Recorded in `publish-0-2-0-alpha-1/_release-log.md` and routed to HS-P0016, whose
  PR boundary already owns registry presentation. It belongs to initiative DoD 10, which is deferred; AC-011
  asks only that the version resolve and carry its three mitigations, and it does.
- **N-4 — PS-18, PS-27 and PS-30 took a third disposition.** `project.md`'s AC-008 offers two outcomes
  (name their callers **or** promote the clause to a documented exclusion); what shipped is a *counted*
  `[DEFERRED]` marker carrying the count, the evaluation point and a named owner. I judge this **within**
  AC-008 rather than a miss — the failure mode the criterion exists to prevent is a provisional marker
  nobody ever evaluates (`RUNBOOK.md:4077`), and the evaluation demonstrably happened and is written into
  the clause bodies — but the disposition is a judgement the project made and should be met knowingly by
  whoever next reads those three clauses.
- **N-5 — `RUNBOOK.md`'s phase-7 exit boxes are still unticked** for work that is now demonstrably done:
  ADR-0020 and ADR-0021 written first, the `trybuild` case, `cargo run -p course-subscriptions`, PS-33's
  falsifier, the three counted clauses, and PS-32/PS-33/PS-35 leaving the clause space
  (`RUNBOOK.md:4060-4088`). Only the `happenstance-macros` box was ticked. CLAUDE.md calls `RUNBOOK.md` the
  plan of record *and how far it has got*, so six false negatives there will read as forgotten work. Not in
  this project's Definition of Done, hence a note rather than a required change.
- **N-6 — the tuple `Boundary` impls require every member to share `type Event`**
  (`crates/happenstance/src/composition.rs:29-31`), and the rendered composition page does not say so. It is
  *forced* by the signed-off shape — `_design.md:423` gives `Boundary` a single associated `Event` — and it
  is the right shape for DCB, where one boundary folds one domain enum; the worked example demonstrates it
  at `examples/course-subscriptions/src/main.rs:457`. But a reader new to idiomatic Rust who composes two
  models over different enums meets an `E0271` with no prose to have warned them, which is exactly the
  first-hour cost DT-2 is being measured on. One sentence on the arity-2 impl's doc block would close it.
- **N-7 — one disclosed deviation from `_design.md`, with no code owed.** `commit` and `commit_with` ship
  `B: Boundary + Clone` (`crates/happenstance/src/command.rs:227`, `:274`) where the design writes
  `B: Boundary`, because `Boundary` is sealed and cannot gain a `Clone` supertrait without editing a frozen
  surface. Disclosed when it landed and carried in `_slices.md`'s *Deviations* table — which is the design's
  own risk-table instruction (escalate rather than adapt silently) followed, not broken. It costs a caller
  nothing: every `DecisionModel` is already `Clone`, and tuples of `Clone` are `Clone`.

### Per-story checkpoint SHAs

Derived from git — `git log 74135b8..HEAD --grep "Story: typed-layer-and-alpha-release/"` — which is the
source of truth, and cross-checked against `_slices.md`. All seventeen workflow-reported checkpoints are
present and accounted for.

| Milestone / slice | Story | Checkpoint |
| --- | --- | --- |
| M1 `decision-records` | `adr-0020-fold-query-agreement` | `c011143` |
| M1 `decision-records` | `adr-0021-payload-evolution-and-codec-tag` | `e33dc9f` |
| M2 `typed-vocabulary` | `domain-event-and-decision-model` | `996853f` |
| M2 `typed-vocabulary` | `decision-model-composition` | `4dc6aeb` |
| M3 `codec-and-command-loop` | `codec-and-feature-forwarding` | `da530cd` |
| M3 `codec-and-command-loop` | `command-loop` | `9971600` |
| M4 `testing-surface` | `misbehaving-testkit-stores` | `6c59c46` |
| M4 `testing-surface` | `given-when-then-dsl` | `bd054c0` |
| M5 `projection-runner` | `projection-trait-and-runner` | `60b8072` |
| M5 `projection-runner` | `projection-clause-verdicts` | `55a2370` |
| M5 `projection-runner` | `polling-cost-measurement` | `049d513` |
| M6 `worked-example-and-proof` | `worked-example-on-typed-layer` | `597a20e` |
| M6 `worked-example-and-proof` | `compile-fail-proof-artefact` | `1044a95` |
| M7 `alpha-release` | `edge-flavour-and-wasm-claim` | `0a010c9` |
| M7 `alpha-release` | `defect-log-and-macros-verdict` | `344f2c0` |
| M7 `alpha-release` | `publish-0-2-0-alpha-1` | `448e1ac` |
| M7 `alpha-release` | slice seal | `ab1cb21` |

Slice seals, one per milestone, each carrying a `Slice-Verdict: typed-layer-and-alpha-release/<slice>
approved` trailer: `d4aedfc`, `3452a5b`, `f63c2e0`, `b4d7003`, `f9fb33c`, `ab2ca90`, `ab1cb21`.

Out-of-band fixes the workflow made — expected, not scope drift: `26a3772`, `b77cffb`, `90421d0`
(slice and gate repairs); `3fde7d1`, `0ea1ea0`, `c4e36c4`, `7abff7d`, `d95c760`, `952a870` (constitution
citation re-anchoring and slice-review answers); `0a3e021` (AC-007 discharge and two unrecorded widenings,
no source change); `6a01d90`, `78a2170`, `4314349` (report SHA pinning).

## Required Changes

None. The verdict is `approved`.

The seven carried notes above (**N-1 through N-7**) are records, not requirements, and none is a
precondition for this project's closeout:

- **N-1** and **N-2** are discharged by the CLI and by the human-invoked kb-ingest wave respectively,
  neither of which a reviewer may drive.
- **N-3** belongs to initiative DoD 10 and is already routed to **HS-P0016 `publication-and-positioning`**.
- **N-4** is a disposition to be met knowingly by whoever next reads PS-18, PS-27 and PS-30; **HS-P0010**
  owns all three.
- **N-5** (six unticked `RUNBOOK.md` phase-7 exit boxes) and **N-6** (one sentence documenting the shared
  `type Event` constraint on the tuple impls) are the two that could be closed cheaply in this repository,
  and are offered to the next pass rather than demanded of this one.
- **N-7** is closed already: the design and the shipped signature do not diverge silently, because
  `_slices.md` carries the row.
