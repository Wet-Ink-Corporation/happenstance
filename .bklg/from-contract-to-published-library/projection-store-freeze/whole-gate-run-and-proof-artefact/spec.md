---
item: HS-S0017
stage: spec
created: 2026-08-12T13:46:12.125Z
updated: 2026-08-12T13:46:12.125Z
template_sig: 87bbf1d0
rendered_sig: 43bbcf36
---

# Spec — One clean cargo xtask ci run, and the proof artefact recorded

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` (DoD 7 at `:377-379`; DoD 13 at `:396-397`; AC-07 at `:327-329`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` (AC-013 `:221-224`, AC-016 `:232-234`, DoD 1/2 `:241-245`, DoD 4 `:247-248`, DoD 8 `:255-256`) |
| This spec | `.bklg/from-contract-to-published-library/projection-store-freeze/whole-gate-run-and-proof-artefact/spec.md` |
| Key brief — architecture | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` (AC-A04 `:307-311`, Note 3's PS-2 paragraph `:437-460`, Note 4's emitter/powerset consequences `:483-515`) |
| Key brief — testing | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` (AC-013 row `:783`, AC-016 row `:786`, the E2E tier `:821-826`, merge-gate commands `:828-853`, "nothing real is mocked" `:877-891`) |
| Key brief — ux (text surface) | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` (AC-U11, the reason surviving `wasm32`, `:189-195`) |
| Signed-off design | `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` — `surfaces: []` (`:48-50`), `## Items` N/A (`:52-54`), sign-off `:96-101` |
| Story map row | `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md:69` (slice `port-disposition-and-freeze-record`, merge order `:114`) |
| This story's discover | `.bklg/from-contract-to-published-library/projection-store-freeze/whole-gate-run-and-proof-artefact/discover.md` |
| Roadmap pointer | `RUNBOOK.md:3935-3940` (phase 6's proof artefact) and `RUNBOOK.md:3951-3961` (its exit criteria) |
| Intake pointer | `.bklg/from-contract-to-published-library/projection-store-freeze/_intake-brief.md:72-81` (the proof artefact, named at intake) |

## One-line PR slice

One full `cargo xtask ci` on a clean checkout — every projection rule passing
under the `wasm32` emitter in the same run as the host emitters via the existing
conformance-harness check step, the skeletons compiling, `cargo hack` over the
widened feature powersets — with the proof artefact recorded: the name of the
test `CheckpointOnlyStore` fails, and both passing batch shapes.

## Executive summary

This PR lands the **last** change of the project and the **only** artefact that
survives it as evidence.

Everything executable already exists by the time this story starts: seventeen
projection rules and the single enumeration (`projection-suite-entry-point`
onward), the projection mutant registry and its exactness meta-tests
(`projection-mutant-registry`), two structurally unlike batch shapes green
(`memory-projection-store`, `buffering-conformant-variant`), the
`unstable-projection` gate and the PS clause dispositions
(`unstable-projection-gate-and-clause-disposition`), and the documented extension
surface (`documented-extension-surface`). Nothing here adds a rule, a mutant, a
clause or a port method.

The delta is three things, and the first two are what stop the third from being a
colour:

1. **The names are held by the gate.** `xtask/src/proof.rs` already exists to do
   exactly this for phases 3 and 5 — it asserts each phase's proof-artefact test
   names out of `cargo test -- --list` *before* running them, because "`cargo
   test` exits 0 on `running 0 tests`, so an emptied file passes a step that a
   deleted one fails" (`xtask/src/proof.rs:9-23`). Its `ARTEFACTS` table carries
   three rows today (`:133-149`) and none of them is the projection family. This
   PR adds the phase-6 rows, so the two names DoD 1 and DoD 2 rest on cannot be
   renamed, `#[ignore]`d or emptied in silence.
2. **AC-016's second half is verified rather than assumed.** The gate's `wasm32`
   step *type-checks* the harnesses (`xtask/src/main.rs:231-243`); a harness that
   listed three rules by hand would type-check exactly as well as one that expands
   from `for_each_projection_store_rule!`. A parity guard closes that, by
   comparing each harness's source text against the enumeration's own names.
3. **The artefact itself** — a dated, commit-pinned evidence document under
   `references/evaluation/`, registered in that directory's README, whose content
   is specified as **nouns**: the test `CheckpointOnlyStore` fails, both passing
   fixtures, which optional gate steps ran versus skipped, and what the run does
   not cover.

The single sentence this story exists to refuse is `cargo xtask ci`: green. It is
true, it is the hardest-won sentence in the project, and DoD 8 forbids it as a
substitute in exactly these words: *"a green gate is a **precondition** for
looking at items 1 and 2, never a substitute for them"* (`project.md:255-256`).

## Context pack

Everything below is a decision this story must honour, stated here so no anchor
has to be opened to start. The deeper artefacts are behind the signposted anchors
in the second half of this spec.

### 1. The proof artefact is nouns, not a status — and there are two of them

DoD 1 asks for a **test name**: `CheckpointOnlyStore` fails the projection suite
by name, and *"the failing test's name is quoted in the closeout"*
(`project.md:241-243`). DoD 2 asks for **two fixture names**: two structurally
unlike batch shapes pass (`:244-245`). Neither is satisfied by a green run, and a
green run is consistent with all three of: a `CheckpointOnlyStore` quietly
dropped from the registry, a "second batch shape" that is the oracle wearing a
hat, and a `wasm32` harness running two rules out of seventeen.

There are **two** names on the first half, and conflating them is the trap
`discover.md` question 2 names. The name a reviewer wants is the *conformance
rule* `CheckpointOnlyStore` fails — `commit_is_atomic_with_the_read_model`, per
`projection-suite-entry-point`'s AC-006. The name the *gate* proves is the
*meta-test* that asserts it fails exactly there — the projection sibling of
`mutants_fail_exactly_their_declared_rules`
(`crates/happenstance-testkit/tests/mutation_coverage.rs`, listed in
`xtask/src/proof.rs:88`). The gate exits zero **because** the mutant failed where
it was declared to; the artefact must say both names and say which is which,
plus the exact command that shows the rule failing on its own.

### 2. The mount is `xtask/src/proof.rs`, and it is the only place this can be enforced

`xtask/src/proof.rs` is a gate step in `REQUIRED` — *"each phase's proof
artefacts"* (`xtask/src/main.rs:178-190`) — that, per artefact, asserts a list of
fully qualified test names out of `cargo test --locked -p <pkg> --all-features
--test <target> -- --list` and only then runs the target. Its own documentation
states the argument this story inherits rather than re-makes: naming a target
catches its deletion, and asserting the names catches its *emptying*, which is
the failure a `cargo test --workspace` cannot see (`xtask/src/proof.rs:9-23`).
The lists are *"expectations, and they are meant to be edited"* — the duplication
against the target's own source is the mechanism, because renaming one fails this
step, *"which is exactly the moment to ask whether the clause citing the old name
in `SPECIFICATION.md` was updated too"* (`:25-32`). It is a **subset** check, not
an equality one (`:33-39`), so a later meta-test needs no gate edit.

The projection family is invisible to that table today, and no sibling story
claims it: `projection-mutant-registry` lands
`crates/happenstance-testkit/tests/projection_mutation_coverage.rs` and states
that mounting is by cargo target auto-discovery with *"no manifest edit
required"* (that story's Integration contract). Auto-discovery makes the target
**run**; it does not make its names **held**. Closing that is this story's mount.

### 3. Adding a second `happenstance-testkit` row exposes a real defect in `proof.rs`

`check()` decides whether to print a registry row-count by matching the
**package** alone — `if package == REGISTRY_PACKAGE` (`xtask/src/proof.rs:218`) —
and `registry_len()` reads one hard-coded path, the *event-store* registry at
`crates/happenstance-testkit/tests/mutation_coverage.rs` (`:79`, `:267-290`). A
projection row in the same package therefore prints the event-store registry's
row count beside the projection target, which is a number that is not about that
target at all. The count exists precisely because *"a number quoted in two
documents and computed nowhere goes stale the first time a mutant lands — which
it did"* (`:241-253`), so reintroducing a wrong one here is the same defect one
level up. The fix is to key the registry file to the artefact (package **and**
target, or a per-artefact optional registry path), and it is in scope: this story
is the one that makes the defect reachable.

### 4. AC-016 has a mechanism half and a verification half, and only the first exists

The mechanism is already in the gate and costs this project nothing: `cargo check
--locked -p happenstance-testkit --tests --target wasm32-unknown-unknown`
(`xtask/src/main.rs:231-243`) type-checks every harness under
`crates/happenstance-testkit/tests/`, including the new
`projection_conformance_wasm.rs`. That step is mandatory and is *"the whole of
AC-016's mechanism"* (`_decomposition.md:356`).

The verification half is this story's, and it is not satisfiable by reading a
green tick. A harness naming three rules by hand type-checks as well as one
expanding from the single enumeration, so AC-016's *"not as a separately
maintained subset"* would be false with nothing to say so (`discover.md:87-98`).
The guard is structural and comes from a pattern already in the tree:
`no_orphan_rules` obtains the registered names from
`for_each_event_store_rule!(crate::__emit_rule_names)` and compares them against
a source scan (`crates/happenstance-testkit/src/registry.rs:293-295`, `:410-436`).
The projection parity guard does the mirror: it takes the enumeration's own names
and asserts that **no harness source contains any of them**, and that each of the
three harnesses carries exactly one `projection_store_conformance!` invocation.

### 5. "Runs and passes" on `wasm32` happens in CI, not in `cargo xtask ci` — say so

The local gate *checks*; it does not execute wasm tests. What executes them is
the CI job `conformance on wasm32`, which resolves `wasm-bindgen-cli` out of
`Cargo.lock` and runs `cargo test --locked -p happenstance-testkit --target
wasm32-unknown-unknown` under `wasm-bindgen-test-runner`
(`.github/workflows/ci.yml:204-238`). The new projection wasm harness is picked
up by that job through the same auto-discovery, with no workflow edit.

So project AC-016's *"runs and passes"* is literally true only of that job, and
the artefact must attribute it there rather than to the local run. This is the
same honesty the UX brief asks for one level down: on `wasm32`
`RuleOutcome::report` is a measured no-op, so the skip reason reaches a human
through `console_log!` instead (`_decomposition.md:189-195`;
`crates/happenstance-testkit/src/contract.rs:525-531`).

### 6. The run is last, or it is evidence about a tree that no longer exists

`depends_on` is `unstable-projection-gate-and-clause-disposition` (HS-S0015) and
`documented-extension-surface` (HS-S0016) — the last two changes the project
makes (`_storymap.md:114`). The gate run happens **after** both are merged, on a
clean working tree, and once. Running it mid-slice and recording that is the
fourth wrong implementation `discover.md` names (`:109-114`).

### 7. This project's boundary bar is the full gate, not `--fast`

`--fast` drops `OPTIONAL` — both feature powersets, `cargo deny` and the nightly
`--cfg docsrs` build (`xtask/src/main.rs:535-638`, `:844-846`) — and the project's
own DoD 4 requires *"green on the whole workspace, including the mandatory wasm32
steps and `spec-trace`"* (`project.md:247-248`). Two of sixteen project ACs are
proven by exactly the steps `--fast` omits or by mandatory steps `--fast` keeps
but story-grain runs skip in practice (`_decomposition.md:846-853`). A step also
*skips itself* when its probe fails: `run_steps` prints ``skipped: `<probe>` did
not succeed`` and continues (`xtask/src/main.rs:872-878`). Both `cargo-hack` and
`cargo-deny` resolve on this machine (`CLAUDE.md`, *Commands*), so a skip here is
a signal, not a default — and a green run with four skips is a weaker claim than
a green run with none. The artefact records which.

### 8. Three verdicts are explicitly **not** this story's, and one number is forbidden

- **The freeze verdict** is `ladybug-projection-store`'s (HS-P0015)
  (`project.md:56-58`).
- **The `unstable-projection` exposure verdict at publish** is
  `publication-and-positioning`'s (HS-P0016); this project supplies evidence
  (`project.md:129-131`, AC-015).
- **PS-2's bar is not met in-project and must not be reported as met.** Two
  testkit instruments do not clear a clause whose *Rejects* names *"the schedule
  that freezes this port against `MemoryProjectionStore` and an in-process
  rusqlite transaction"* (`_decomposition.md:437-456`;
  `spec/SPECIFICATION.md:4760-4775`).
- **No pass rate over the mutant set, ever** — ADR-0010's prohibition, because
  the denominator is a choice (`.kb/decisions/0010-the-suite-must-prove-itself.md`;
  `xtask/src/proof.rs:255-259`). "Seventeen of seventeen rules ran" is a
  statement about the rule set and is allowed; "48 of 50 mutants caught" is the
  forbidden sentence.

### 9. The persona slice this closes

P2, the adapter author, and P3, the local-first / edge developer
(`_decomposition.md:55-58`). P2's fear is a port that *"quietly assumed something
their storage cannot provide, discovered late"*; the artefact is what lets them
check the discrimination claim instead of trusting it — which is initiative AC-08
for the evaluator, one recipient further out (`initiative.md:330-332`). P3's rule
is AC-016, and their failure mode is *"tooling that quietly stops covering it"*
(`_decomposition.md:58`) — which is precisely what a hand-listed wasm harness
would be.

## Integration contract

- **Archetype**: `capability`. The user-observable slice is the evidence surface:
  what a reviewer, the closeout, and HS-P0015/HS-P0016 can read and re-run. It
  crosses the gate (`xtask/`), the testkit's test targets and the citable record
  (`references/evaluation/`).
- **Slice / milestone**: `port-disposition-and-freeze-record`. Slice-mate:
  **`unstable-projection-gate-and-clause-disposition`** (HS-S0015), which lands
  first inside the slice — its clause dispositions and the `unstable-projection`
  gate must be in the tree before the run, or the run is about a different tree
  (`_storymap.md:114`).
- **Mount point**: **`xtask/src/proof.rs`** — the `ARTEFACTS` table (`:133-149`),
  read by the `REQUIRED` gate step *"each phase's proof artefacts"*
  (`xtask/src/main.rs:178-190`). This is the composition root for this medium: a
  test target that runs but whose names nothing asserts is the library equivalent
  of a component that renders nowhere — `cargo test --workspace` *"passes just as
  happily with one fewer target as with one more"* (`xtask/src/main.rs:156-166`).
  **Second wiring point, and it is not optional**:
  **`references/evaluation/README.md`**, the "Later additions, which are neither"
  section (`:40-55`), where the evidence document is registered with its date,
  its pinned commit and its supersede-rather-than-edit lifecycle. That directory's
  README claims to enumerate the directory, so an unregistered document there is
  reachable only by `ls` — the same mount discipline
  `ps-clause-pairing-sweep` already adopted for its own evidence document.
- **Wires into**:
  - `crates/happenstance-testkit/tests/projection_mutation_coverage.rs` and its
    `#[path]`-reached support modules (landed by `projection-mutant-registry`,
    HS-S0008) — the target whose meta-test names the first `ARTEFACTS` row holds,
    and the harness whose assertion is what a green gate actually proves about
    `CheckpointOnlyStore`.
  - `crates/happenstance-testkit/tests/projection_conformance.rs`,
    `projection_conformance_blocking.rs`, `projection_conformance_wasm.rs`
    (landed by `projection-suite-entry-point`, HS-S0007) — the three harness
    sources the parity guard scans, and the two host targets whose `-- --list`
    output supplies the rule count recorded in the artefact.
  - `for_each_projection_store_rule!` and `__emit_rule_names`
    (`crates/happenstance-testkit/src/registry.rs:293-295`) — the single
    enumeration the parity guard compares against; the same pair `no_orphan_rules`
    uses (`:410-436`).
  - `happenstance_testkit::fixtures::MemoryProjectionFixture` and the CF-5
    buffering conformant variant in `crates/happenstance-testkit/tests/` — the
    two structurally unlike batch shapes named in the artefact (`project.md`
    AC-004; `_decomposition.md:446-448`).
  - `xtask/src/main.rs`'s `REQUIRED` / `OPTIONAL` step tables (`:105`, `:535`) and
    `run_steps`' probe-and-skip contract (`:862-892`) — the source of the
    ran-versus-skipped ledger.
  - `.github/workflows/ci.yml:204-238` (`conformance on wasm32`) and `:241-278`
    (`minimum supported Rust version`) — the two jobs the artefact attributes
    claims to that the local gate does not make.
- **Public items**: **none.** `_design.md` records `surfaces: []` and `N/A — no
  user-facing surface` under `## Items` and `## Signatures` (`:48-58`); the
  sign-off approved that determination itself (`:96-101`). This story adds no
  entry to any crate's `lib.rs` export block and no feature-table row. `xtask` is
  `publish = false` tooling and the new test target is test-target-private, so
  nothing here is a semver promise.
- **Renders surfaces**: **none** — `surfaces: []` in the project's `_design.md`.
  The nearest thing to a surface is the gate's own stdout, which this story reads
  and records rather than changes.
- **Conformance rule(s)**: **adds none, and this is not a port change.** No file
  under `crates/happenstance-core/src/` or `crates/happenstance-testkit/src/suite.rs`
  (or the projection rules module) is touched, so the obligation *"a story that
  changes a port and names no rule is a port change nothing can fail"* is
  discharged by changing no port. This story is *observed by* the whole projection
  rule family (every rule the enumeration carries), by the projection meta-tests
  the new `ARTEFACTS` rows name, and by the parity guard it authors.
- **Clause(s)**: **discharges** project AC-013's second half (the whole-workspace
  `cargo xtask ci`) and AC-016's verification half; **cites** PS-2 and PS-3
  without amending either; **amends nothing.** No `[FROZEN]` clause is edited and
  `spec/SPECIFICATION.md` is not in this story's PR boundary — clause disposition
  was HS-S0015's, and any frozen-clause repair landed as a new accepted atom in
  slice 1 (`discover.md:135-139`).
- **Advances DoD scenario**: initiative **DoD 7** — *"The projection suite
  discriminates. Two structurally unlike batch shapes pass it, and a deliberately
  wrong implementation that writes a checkpoint without its read model fails it,
  by name"* (`initiative.md:377-379`). This story does not make DoD 7 true; the
  earlier slices did. It makes DoD 7 **legible and re-runnable** — the artefact is
  what DoD 8's *"the freeze verdict is written"* (HS-P0015) and initiative AC-08's
  *"the evaluator can check the compliance claim instead of trusting it"* are read
  from. It also moves initiative **DoD 13** (*"the gate is green on the assembled
  whole"*, `:396-397`) toward green at this project's boundary, and initiative
  **AC-07** (the edge developer's runtime, `:327-329`) via AC-016.

## PR boundary

**In this PR**

- `xtask/src/proof.rs` — the phase-6 `ARTEFACTS` rows (the projection mutant
  target's meta-tests, and the parity target), plus the fix that makes the
  registry row-count belong to the target it is printed beside.
- `crates/happenstance-testkit/tests/projection_harness_parity.rs` — the
  structural guard for AC-016's verification half: every enumerated rule reaches
  all three emitters, and no harness source contains a rule name.
- `references/evaluation/phase-6-projection-proof.md` — the evidence document:
  dated, pinned to the commit the run was made on, immutable lifecycle.
- `references/evaluation/README.md` — one registration row in "Later additions,
  which are neither" (`:40-55`).
- `CHANGELOG.md` — one `[Unreleased]` entry, naming what the gate now holds that
  it did not before (the projection proof artefact's names). This is not CF-29's
  obligation, which is about rule names and lands on the stories that add rules
  (`spec/SPECIFICATION.md:8141-8153`); it is the ordinary entry for a change to
  what the gate enforces.
- This story's own backlog folder — its `_ledger.md` and implementation report.
- The composition-root wiring named in the Integration contract, which for this
  story is exactly `xtask/src/proof.rs` and `references/evaluation/README.md`.
  That is not scope drift.

**Explicitly not in this PR**

- **Any conformance rule, mutant, fixture or registry row.** Rules are slices 3–5's,
  mutants are `projection-mutant-registry`'s and its slice-mate's, the CF-5
  conformant variant is `buffering-conformant-variant`'s. If the run reveals a
  missing mutant or a rule asserting a literal position, that is **reported
  against the story that wrote it**, not repaired here (`discover.md:132-139`).
- **Any port, probe, feature-table or `lib.rs` export change.** `happenstance-core`
  and `crates/happenstance-testkit/src/` are untouched.
- **`spec/SPECIFICATION.md`, any `[FROZEN]` clause, any maturity marker.** HS-S0015
  owns clause disposition; this story runs `spec-trace` and records that it was
  green, it does not make it green.
- **Any new gate step or CI job.** AC-016 *"costs this project a new harness file,
  not a new gate step"* (`_decomposition.md:786`), and the harness file was
  HS-S0007's. The `wasm-conformance` and `msrv` workflow jobs are read, not
  edited.
- **Fleshing out a skeleton body** to make AC-013 integration-provable. AC-013's
  bar here is Static and compile-only; *"no story should attempt to flesh out a
  skeleton body… that adapter's own project owns doing so"*
  (`_decomposition.md:882-886`).
- **The PS-3 finding** (`ps3-batch-shape-finding`, HS-S0014), **the freeze
  verdict** (HS-P0015) and **the `unstable-projection` exposure verdict**
  (HS-P0016). The artefact links the first and makes no claim on the other two.

**Merge DoD one-liner**: one `cargo xtask ci` on a clean checkout after HS-S0015
and HS-S0016 have merged is green with every optional step recorded as run or
skipped, and a reviewer who has never seen this project can read
`references/evaluation/phase-6-projection-proof.md`, paste one command out of it,
and watch the named test reject `CheckpointOnlyStore`.

```
xtask/src/proof.rs
crates/happenstance-testkit/tests/projection_harness_parity.rs
references/evaluation/phase-6-projection-proof.md
references/evaluation/README.md
CHANGELOG.md
.bklg/from-contract-to-published-library/projection-store-freeze/whole-gate-run-and-proof-artefact/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The run is last, clean and single** | The gate runs after HS-S0015 and HS-S0016 are merged, from a working tree with no uncommitted changes, and its commit sha is recorded in the artefact. `--locked` is already carried by every step that takes it, so the run is against the committed lockfile. Re-running after a later edit means re-recording, not amending the pin. | `_storymap.md:114`; `discover.md:109-114`; `references/evaluation/README.md:40-55` (pinned lifecycle) |
| **The bar is the full gate, not `--fast`** | `cargo xtask ci` whole. `--fast` drops `OPTIONAL` — both feature powersets, `cargo deny`, the nightly `--cfg docsrs` build — and prints how many it skipped; that is the story-grain bar and is not evidence for project AC-013/AC-014/AC-016. | `xtask/src/main.rs:535-638`, `:844-858`; `project.md:247-248`; `_decomposition.md:828-853` |
| **Every optional step is recorded as ran or skipped** | `run_steps` prints ``=== <step> ===`` and, when a probe fails, ``skipped: `<probe>` did not succeed``. The artefact carries one row per `OPTIONAL` step — feature powerset, wasm32 feature powerset, licences and advisories, docs.rs configuration (nightly) — with its probe and its outcome. A green run with skips is a different claim and must read as one. | `xtask/src/main.rs:535`, `:546`, `:564`, `:596`, `:621`, `:862-892`; `CLAUDE.md`, *Commands* |
| **The widened powersets are exercised, not assumed** | `happenstance-core` gained `conformance` and (per AC-A04) `unstable-projection`, taking its combination count from eight to thirty-two across the host powerset and the wasm32 powerset that names the crate explicitly. The artefact records that both powerset steps ran, since they are the only steps that would catch `conformance` being silently coupled to `memory`. | `_decomposition.md:483-487`; `xtask/src/main.rs:546-563`, `:564-595` |
| **`ARTEFACTS` gains the phase-6 rows** | One `Artefact` for `happenstance-testkit` / `projection_mutation_coverage`, whose `tests` list is the projection meta-tests **read out of `cargo test --locked -p happenstance-testkit --all-features --test projection_mutation_coverage -- --list`**, not guessed; one for the parity target. Fully qualified exactly as libtest lists them — `proof.rs` expects an inner `mod` of the target's own name, so if the projection target does not wrap its tests that way, the row states the names as listed and the deviation is reported to HS-S0008 rather than repaired here. Subset, not equality: a later meta-test needs no gate edit. | `xtask/src/proof.rs:58-71`, `:85-102`, `:133-149`, `:192-238`, `:298-325` |
| **The registry count belongs to the target it is printed beside** | `check()` selects the count by package alone (`if package == REGISTRY_PACKAGE`) and `registry_len()` reads one hard-coded event-store path, so a second `happenstance-testkit` row would print the event-store registry's row count against the projection target. Keyed to the artefact instead (package **and** target, or an optional per-artefact registry path), and both counts printed beside their own rows. | `xtask/src/proof.rs:72-84`, `:216-227`, `:241-290` |
| **The parity guard: no harness lists rules by hand** | A host-only test target obtains the rule names from `for_each_projection_store_rule!(happenstance_testkit::__emit_rule_names)` and asserts, over `include_str!` of all three harness sources, that (a) none contains any enumerated rule name as an identifier, and (b) each contains exactly one `projection_store_conformance!` invocation. Failure message names the offending harness and rule. This is the mirror of `no_orphan_rules`, which compares the enumeration against a source scan of `suite.rs`. | `crates/happenstance-testkit/src/registry.rs:290-295`, `:410-436`; `_decomposition.md:786`; `discover.md:87-98` |
| **Why a test and not a review** | The alternative considered and rejected: a reviewed `rg` over the three harnesses, recorded in the artefact. It cannot fail twice — it is not re-run when the eighteenth rule lands — and this repository's own gate-design argument is that a check whose absence nothing notices is decorative (`proof.rs`'s opening paragraphs make exactly this case about naming a target). A second rejected option, a `#[cfg(test)]` unit test beside `no_orphan_rules`, loses because `ARTEFACTS` can only name a **test target**, so the guard could not itself be held. | `xtask/src/proof.rs:9-23`; `CLAUDE.md`, *The rule that matters* (a rule no adapter can fail is decorative) |
| **`wasm32`: type-checked locally, executed in CI, attributed separately** | The gate's mandatory step is `cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown`. Execution is the CI job `conformance on wasm32`, which pins `wasm-bindgen-cli` to the `Cargo.lock` version and runs `cargo test --locked -p happenstance-testkit --target wasm32-unknown-unknown` under `wasm-bindgen-test-runner`; the new harness is auto-discovered with no workflow edit. The artefact states which claim rests on which, and records the CI run it read. | `xtask/src/main.rs:231-243`; `.github/workflows/ci.yml:204-238` |
| **Both batch shapes are named, with how each was observed** | `MemoryProjectionFixture` (apply-on-write) through the three conformance harnesses, and the CF-5 buffering conformant variant (replay-at-commit) through the projection mutant registry's conformant-variant positive control — both inside the **same** `cargo xtask ci` invocation, which is what makes the artefact one run naming two fixtures rather than two runs a reviewer reconciles by hand. | `_decomposition.md:774`, `:446-448`; `project.md:193-196` |
| **The failing test is quoted with a runnable reproduction** | The artefact quotes the conformance rule `CheckpointOnlyStore` fails, the meta-test that asserts it fails exactly there, and the command that shows the rule failing directly — so DoD 1 is checkable by a reader who does not already know the harness catches the panic. | `project.md:241-243`; `xtask/src/proof.rs:9-23`; `projection-mutant-registry/spec.md` (Integration contract, mutants and meta-tests) |
| **Skeletons: compile-only, and no body is fleshed out** | `happenstance-postgres`, `happenstance-ladybug` and `happenstance-sqlite` compile in the run's `tests`/build steps with every method body still `todo!()`. The artefact records that they compiled and that no skeleton body changed in this PR; AC-013's *reviewed diff* half was `owned-batch-port-shape`'s. | `_decomposition.md:783`, `:882-886`; `project.md:221-224` |
| **What the run does not cover, stated in the artefact** | The MSRV (CI's `minimum supported Rust version` job, `cargo hack check --no-dev-deps --rust-version` plus a full test run at 1.97.1 — the local gate never checks it); `wasm32` execution (CI, above); PS-2's bar, which two testkit instruments do not clear; and the semver / advisories jobs that are CI-side. Claiming coverage the run does not have is the artefact's own failure mode. | `.github/workflows/ci.yml:241-278`; `CLAUDE.md`, *Commands* (the MSRV paragraph); `_decomposition.md:437-456` |
| **No verdict, and no pass rate** | No freeze verdict (HS-P0015), no `unstable-projection` exposure verdict (HS-P0016), no PS-3 finding (HS-S0014 — linked, not restated), and no ratio over the mutant set in any form. Rule-set counts ("seventeen rules, three emitters") are statements about the enumeration and are allowed. | `project.md:120-131`; `.kb/decisions/0010-the-suite-must-prove-itself.md`; `xtask/src/proof.rs:241-259` |
| **The document is evidence, not instruction, and is registered** | `references/evaluation/` is an enumerated, lifecycle-classified corpus; the new document takes the immutable arm — dated, pinned to a commit, superseded rather than edited — and its README row says so, following `review-citation-drift.md`'s precedent as a byproduct that is not one of the original fourteen. Every `file:line` in it resolves at the pinned commit; citation drift that still passes `spec-trace` is this directory's own named failure mode. | `references/evaluation/README.md:40-55`, `:50-55`; `CLAUDE.md` (`references/` — evidence kept for citation, binding nothing) |

## Data and migrations

**N/A — no schema, no persisted user data, no migration.** This story adds no
type, no column, no wire-format field and no on-disk format; the projection port,
its `Batch`, its `Checkpoint` and the read models the suite writes through
`ProjectionProbe` are all earlier stories' and are consumed here unchanged.

Three pieces of state do change, and they are named here so the absence above is
a finding rather than an omission:

1. **`xtask/src/proof.rs`'s `ARTEFACTS`** — compile-time `const` data, migrated by
   editing it. `proof.rs`'s own documentation is explicit that these lists *"are
   expectations, and they are meant to be edited"*, and that the check is a subset
   rather than an equality one, so a later meta-test lands without touching this
   file (`xtask/src/proof.rs:25-39`).
2. **One markdown document under `references/evaluation/`** — append-only in the
   strong sense: its lifecycle is *supersede rather than edit*, so a later
   correction lands as a second document that names this one, never as an in-place
   fix (`references/evaluation/README.md:8-13`, `:40-55`).
3. **`CHANGELOG.md`'s `[Unreleased]` section** — one added entry. Nothing is
   published yet, so no version is affected and no consumer is pinned to anything
   (`CHANGELOG.md:24-30`).

## Acceptance criteria

The personas are the ones this project's brief names: **P2**, the adapter author,
whose fear is a port that quietly assumed something their storage cannot provide;
**P3**, the local-first / edge developer, whose failure mode is tooling that
silently stops covering `wasm32`; and **P4**, the evaluator, who wants to *check*
the compliance claim rather than trust it
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114`,
`:182`, `:249`). The fourth reader is internal and just as real: the author of the
freeze verdict (HS-P0015) and of the `unstable-projection` exposure verdict
(HS-P0016), each of whom must be able to reach a decision from this document
without re-running the project.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** P4 is deciding whether this library's projection port is worth adopting and has been handed a claim that the workspace is green, **WHEN** they open `references/evaluation/phase-6-projection-proof.md`, **THEN** it records one full `cargo xtask ci` — not `--fast` — run on a working tree with no uncommitted changes, **after** `unstable-projection-gate-and-clause-disposition` (HS-S0015) and `documented-extension-surface` (HS-S0016) merged, pinned to the commit sha the run was made on, with the exit status and the wall-clock date; and it records that `happenstance-postgres`, `happenstance-ladybug` and `happenstance-sqlite` compiled inside that run with every affected body still `todo!()` and no skeleton body changed in this PR. A run recorded mid-slice, a `--fast` run, or a run on a dirty tree does not satisfy this row. | **E2E** — `cargo xtask ci` whole (`xtask/src/main.rs:105`, `:535`), evidence transcribed into `references/evaluation/phase-6-projection-proof.md`; cleanliness proven by `git status --porcelain` empty at the recorded sha and by the sha being a descendant of both dependency stories' merge commits. Discharges project **AC-013**'s whole-workspace half. |
| AC-002 | **GIVEN** P2 is weighing whether this conformance suite can actually reject their mistake, **WHEN** they read the artefact's first section, **THEN** it names **two** tests and says which is which — the *conformance rule* `CheckpointOnlyStore` fails (`commit_is_atomic_with_the_read_model`, per `projection-suite-entry-point`'s AC-006) and the *meta-test* that asserts it fails exactly there (the projection sibling of `mutants_fail_exactly_their_declared_rules`) — plus one copy-pasteable command that shows the rule failing on its own, so the reader does not have to already know the harness catches the panic. Quoting only the meta-test, or only "the suite is green", fails this row. | **Unit + review**, machine-anchored: the two names are carried as `ARTEFACTS` rows (AC-004) so neither can be renamed in silence, and an `xtask` `#[cfg(test)]` assertion reads the artefact at `workspace_root()` and fails if either name is absent from it — the same read `registry_len` already does (`xtask/src/proof.rs:267-290`). Discharges project **DoD 1** (`project.md:241-243`). |
| AC-003 | **GIVEN** P2 suspects "two batch shapes pass" could mean the oracle wearing a hat, **WHEN** they read the artefact's second section, **THEN** it names both fixtures — `MemoryProjectionFixture` (apply-on-write) and the CF-5 buffering conformant variant (replay-at-commit) — states how each was observed, and states that both were observed **inside the same `cargo xtask ci` invocation**, so the evidence is one gate run naming two fixtures rather than two runs a reviewer reconciles by hand. | **Integration + E2E**: `crates/happenstance-testkit/tests/projection_conformance.rs` and the CF-5 variant both driven to completion inside the run's `tests` step (`_decomposition.md:774`); the same `xtask` `#[cfg(test)]` assertion as AC-002 requires both fixture names to be present in the artefact. Discharges project **DoD 2** (`project.md:244-245`). |
| AC-004 | **GIVEN** the two names DoD 1 and DoD 2 rest on are only as durable as what holds them, **WHEN** a later change renames, `#[ignore]`s or empties the projection meta-tests, **THEN** `cargo xtask ci` fails at the *"each phase's proof artefacts"* step with a message naming the absent tests — because `ARTEFACTS` gained the phase-6 rows: one for `happenstance-testkit` / `projection_mutation_coverage` whose `tests` list was **read out of `cargo test --locked -p happenstance-testkit --all-features --test projection_mutation_coverage -- --list`** rather than guessed, and one for the parity target. The check stays a **subset** check, so a ninth projection meta-test lands with no gate edit. | **Static + Unit** — the gate step itself (`xtask/src/proof.rs:183-240`, reached from `xtask/src/main.rs:178-190`). Negative control: rename one listed test locally and observe the step bail with `is missing N of the tests the gate names` (`xtask/src/proof.rs:207-216`) — a target that runs but whose names nothing asserts is the failure `proof.rs:9-23` exists to reject. |
| AC-005 | **GIVEN** a maintainer reads the gate's own stdout to learn how many mutants the projection registry declares, **WHEN** the proof step prints a row for each `happenstance-testkit` artefact, **THEN** each row's registry count is the count of **that target's** registry — because the count is keyed to the artefact (package **and** target, or a per-artefact optional registry path) rather than to the package alone. Printing the event-store registry's row count beside the projection target is the defect this row forbids; it is the same "a number quoted in two documents and computed nowhere goes stale" failure `registry_len`'s doc comment was written about, one level up. | **Unit** — an `xtask` `#[cfg(test)]` test over `ARTEFACTS` asserting that every artefact declaring a registry names a file under its own package and target, and that no two artefacts share one; plus the gate's own printed output showing two distinct counts (`xtask/src/proof.rs:72-84`, `:218-228`, `:241-290`). |
| AC-006 | **GIVEN** P3 ships to `wasm32` and needs the suite to keep covering them without anyone remembering to update a list, **WHEN** an eighteenth projection rule is registered in the single enumeration and a developer forgets the wasm harness, **THEN** a host test target fails by name — `crates/happenstance-testkit/tests/projection_harness_parity.rs` obtains the rule names from `for_each_projection_store_rule!(happenstance_testkit::__emit_rule_names)` and asserts (a) that no harness source (`projection_conformance.rs`, `projection_conformance_blocking.rs`, `projection_conformance_wasm.rs`) contains any enumerated rule name as an identifier, and (b) that each of the three carries exactly one `projection_store_conformance!` invocation. The failure message names the offending harness and rule. A hand-listed harness type-checks exactly as well as a generated one, so the mandatory `wasm32` check cannot see this and a reviewed `rg` cannot fail twice. | **Unit + Static** — `crates/happenstance-testkit/tests/projection_harness_parity.rs`, mirroring `no_orphan_rules` (`crates/happenstance-testkit/src/registry.rs:290-295`, `:410-436`); its own names are held by an `ARTEFACTS` row (AC-004). Discharges project **AC-016**'s verification half; the mechanism half is the existing `cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown` step (`xtask/src/main.rs:231-243`). |
| AC-007 | **GIVEN** P4 knows a green gate with four skipped steps is a weaker claim than a green gate with none, **WHEN** they read the artefact's run ledger, **THEN** every `OPTIONAL` step appears as a row with its probe and its outcome — feature powerset, `wasm32` feature powerset, licences and advisories, docs.rs configuration (nightly) — and the ledger states explicitly that both feature powersets **ran**, since they are the only steps that would catch `conformance` being silently coupled to `memory` or `unstable-projection` failing to compile with the port gated off (thirty-two combinations, up from eight). A ledger that omits a step, or that reports "green" without distinguishing ran from skipped, fails this row. | **E2E + review** — the run's own stdout: `run_steps` prints `` === <step> === `` and, on a failed probe, `` skipped: `<probe>` did not succeed `` (`xtask/src/main.rs:862-892`); one artefact row per `OPTIONAL` entry (`:535-638`). Grounded in `_decomposition.md:828-853` and `CLAUDE.md`, *Commands*. |
| AC-008 | **GIVEN** P4's whole reason for reading evidence is that they do not trust a summary, **WHEN** they reach the artefact's limits section, **THEN** it states what this run does **not** cover — the MSRV (CI's `minimum supported Rust version` job; the local gate never checks it), `wasm32` **execution** (CI's `conformance on wasm32` job, attributed to the CI run that was read, because the local gate only type-checks), PS-2's bar, which two testkit instruments do not clear, and the CI-side advisories/semver jobs — and makes **no** freeze verdict, **no** `unstable-projection` exposure verdict, and **no ratio over the mutant set in any form**. "Seventeen rules across three emitters" is a statement about the enumeration and is allowed; "48 of 50 mutants caught" is the forbidden sentence. | **Static + Unit** — ADR-0010's prohibition (`.kb/decisions/0010-the-suite-must-prove-itself.md`; restated at `xtask/src/proof.rs:255-259`); an `xtask` `#[cfg(test)]` assertion that the artefact matches no `N of M` / `N/M` ratio over mutants and that the limits section names the MSRV job and the wasm32 execution job. Attribution checked against `.github/workflows/ci.yml:204-238` and `:241-278`. |
| AC-009 | **GIVEN** the closeout and HS-P0015 must find this evidence a month from now without being told where it is, **WHEN** a reader opens `references/evaluation/README.md`, **THEN** the new document appears as a row in *"Later additions, which are neither"* carrying its date, its pinned commit and its **supersede-rather-than-edit** lifecycle — so a later correction lands as a second document naming this one, never as an in-place edit — and every `file:line` citation inside the document resolves at the pinned commit. A document present in the directory but absent from the README is reachable only by `ls`, which is that directory's own named failure mode. | **Static + review** — `references/evaluation/README.md:40-55` enumerates the directory; the registration row is the check, following `review-citation-drift.md`'s precedent. Citation resolution verified by checking out the pinned sha and resolving each `file:line` in the document. |
| AC-010 | **GIVEN** a maintainer reading `CHANGELOG.md` wants to know what the gate enforces that it did not before, **WHEN** they read the `[Unreleased]` section, **THEN** one entry names the new obligation — that the projection proof artefact's test names are now held by the gate, and that harness parity is enforced by a test — without restating rule names (that is CF-29's obligation and belongs to the stories that add rules) and without implying a published version, since nothing is published. | **Static + review** — diff of `CHANGELOG.md`'s `[Unreleased]` section (`CHANGELOG.md:24-30`); CF-29's scope at `spec/SPECIFICATION.md:8141-8153` is the boundary this entry must not cross into. |

## Interaction quality

**Composition family: N/A, and that is a signed-off determination rather than an
omission.** The project's `_design.md` records `surfaces: []` (`:48-50`) and
`N/A — no user-facing surface` under `## Items`, `## Signatures`, `## States` and
`## Anti-patterns` (`:52-90`); the sign-off approved *the no-surface
determination itself* together with those anti-patterns (`:96-101`). There is no
presentation, no placement, no transience policy and no density budget to hold
this story to, because there is nothing rendered. The nearest thing to a
presentation decision in this medium — `RuleOutcome::skip_line`'s one printed line
(`crates/happenstance-testkit/src/contract.rs:500-507`, called by `report` at
`:532`) — is *read* by this story and changed by none of it; it was
`projection-capability-skips`' surface, and this spec must not re-decide it.

**State family, translated into this story's two media** — the gate's stdout and
the evidence document. Each invariant that applies is carried by an AC row above;
none is left as a prose bullet, because a bullet here gets no ledger row and is
never gated.

| Invariant | Carried by | How it is verified here |
| --- | --- | --- |
| **In-place, not a context jump** | **AC-009** | The evidence is reached from the enumeration a reader is already in (`references/evaluation/README.md`), not by discovering a file with `ls`. An unregistered document is the context jump. |
| **Non-occlusion** | **AC-007**, **AC-002** | A skipped optional step is *printed*, not swallowed by a zero exit code; and the failing-test name is not occluded by the green exit that the mutant's own capture produces. Both are cases of the run's most important information being the information a status hides. |
| **Reversibility** | **AC-009**, **AC-004** | The artefact is superseded rather than edited, so a correction never destroys the record it corrects; and the `ARTEFACTS` check is a **subset** check (`xtask/src/proof.rs:34-39`), so adding a meta-test is not a change anyone has to undo a gate edit for. |
| **Preserved context** | **AC-001** | The analogue of preserved focus/scroll in a medium with no view: the run is pinned to a commit, so the evidence keeps the tree it was taken in. Re-running after a later edit means re-recording, not amending the pin. |
| **Reachability without special tooling** | **AC-002**, **NF-005** | The terminal's equivalent of keyboard reachability: every claim the artefact makes is backed by a command a reader can paste, `--locked`, with no state the reader is assumed to already have. |
| **Selection/focus preservation, occlusion by overlay, opened-on-demand chrome** | **N/A** | No stateful view exists to preserve, occlude or reveal (`_design.md:48-50`). Recorded so the absence is a finding, not a gap. |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | A test named by a phase-6 `ARTEFACTS` row is absent from `-- --list` (renamed, `#[ignore]`d, or the file emptied to its `#![cfg(…)]` attributes). | The proof step **bails before running anything**, naming the absent tests and the full listing, with the existing message that tells the reader to update `proof.rs` *and* the clause citing the old name in the same change (`xtask/src/proof.rs:207-216`). No new error path is written; the new rows inherit this one. |
| **EC-002** | A harness source contains an enumerated projection rule name as an identifier. | `projection_harness_parity.rs` fails, naming **which** harness and **which** rule — not a bare count. The message must be actionable by someone who has never seen the enumeration. |
| **EC-003** | A harness carries zero or two `projection_store_conformance!` invocations. | The same target fails, distinguishing the two cases: zero means the harness stopped covering the port entirely; two means a second, hand-scoped invocation was added beside the generated one, which is the hand-listing failure wearing a macro. |
| **EC-004** | An `OPTIONAL` step's probe does not resolve (`cargo-hack`, `cargo-deny`, a nightly toolchain absent). | `run_steps` prints `` skipped: `<probe>` did not succeed `` and continues (`xtask/src/main.rs:862-892`). The artefact records that step as **skipped, with its probe**, and the run is **not** described as fully green. Both tools resolve on this machine (`CLAUDE.md`, *Commands*), so a skip is a signal to investigate before recording, not a default to accept. |
| **EC-005** | The registry file a phase-6 row points at has moved, declares no `REGISTRY`, or is unterminated. | The count errors rather than printing a plausible `0` — the existing contract at `xtask/src/proof.rs:267-290`, which the keying change (AC-005) must preserve for **both** registries rather than only the event-store one. |
| **EC-006** | The working tree is dirty, or the sha is not a descendant of both dependency stories' merges, at the moment the run is made. | The run is **not** recorded. The artefact is re-made after the tree is clean; a partially-amended artefact is worse than none, because it claims a provenance it does not have (`discover.md:109-114`). |
| **EC-007** | The run reveals a real defect owned by an earlier story — a missing mutant, a rule asserting a literal position, a fixture that is the oracle wearing a hat. | It is **reported against the story that wrote it** and fixed there, not repaired inside this PR (`discover.md:132-139`). This story's PR boundary does not include any rule, mutant, fixture or registry row. The artefact may name the finding and link the story. |
| **EC-008** | The projection mutant target does not wrap its tests in an inner `mod` of the target's own name, so the fully-qualified names differ from the shape `ARTEFACTS` assumes (`xtask/src/proof.rs:64-69`). | The row states the names **exactly as `-- --list` printed them** — the listing is the authority — and the structural deviation is reported to `projection-mutant-registry` (HS-S0008) rather than repaired here. Guessing a name that `--list` did not print is the one thing this row forbids. |
| **EC-009** | `cargo xtask ci` fails on a step this story does not own (a clippy lint, a doc build, `spec-trace`). | The failure is triaged to its owning story and fixed there; this story does not make `spec-trace` green, it records that it **was** green (clause disposition was HS-S0015's). If the failure is in `xtask/src/proof.rs` or `projection_harness_parity.rs`, it is this story's. |

## Non-functional

| id | requirement | why it binds |
| --- | --- | --- |
| **NF-001** | The two new `ARTEFACTS` rows use `cargo_args`' existing flag set (`--locked`, `--all-features`) unchanged, so the step reuses the `tests` step's build artifacts instead of re-fingerprinting the crate. | `cargo_args`' own doc comment states this is the difference between a few seconds and a whole rebuild, *per entry* (`xtask/src/proof.rs:151-174`). Two new entries would pay it twice. |
| **NF-002** | The parity guard adds **no dependency** to `happenstance-testkit`, runs host-only, and reads harness sources at compile time (`include_str!`) rather than walking the filesystem at runtime. | The testkit is a published crate's dev-facing dependency; a filesystem walk would also break under `cargo package`'s extracted tree, where the harness paths a runtime scan assumes are not where it expects. |
| **NF-003** | **No new gate step and no new CI job.** The parity target is reached by cargo target auto-discovery and held by an `ARTEFACTS` row; the wasm harness is picked up by `conformance on wasm32` unchanged. | AC-016 *"costs this project a new harness file, not a new gate step"* (`_decomposition.md:786`); `.github/workflows/ci.yml:204-238` needs no edit. |
| **NF-004** | The MSRV floor is unmoved. No new dependency, no language feature newer than 1.97.1, in either `xtask` or the new test target. | ADR-0029 raised the floor deliberately and the rule is *weigh it, do not move it in silence* (`CLAUDE.md`, *Binding constraints* 5). A gate-only story is the worst possible place to move it by accident. |
| **NF-005** | Every command quoted in the artefact is copy-pasteable verbatim, carries `--locked`, and assumes no state the reader does not have after a clone at the pinned sha. | The artefact's entire value to P4 is that they can re-run it. A command that needs an undocumented preceding step is a claim, not evidence. |
| **NF-006** | New `xtask` and testkit code passes `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -D warnings`, and follows the constitution atoms the task pulls (start at `standards/rust/README.md`'s trigger table; do not load the corpus). | Both are `REQUIRED` steps of the very run this story records. A story whose own diff fails the gate it is recording is self-refuting. |
| **NF-007** | The proof step's added output stays two lines per artefact at most, and each line names the target it is about. | The ran/skipped ledger (AC-007) is read out of this stdout by a human. A step that prints a paragraph per artefact makes the skip lines harder to see, which is EC-004's failure by another route. |

## Implementation notes (non-prescriptive)

- **Read the listing before writing the row.** `ARTEFACTS`' `tests` entries are
  fully-qualified names as libtest prints them. Run `cargo test --locked -p
  happenstance-testkit --all-features --test projection_mutation_coverage -- --list`
  first and transcribe; do not derive the names from the source, and do not
  assume the `mod` wrapping (EC-008).
- **The registry keying has more than one acceptable shape.** Adding
  `registry: Option<&'static str>` to `Artefact` and threading it through
  `registry_len` is the smallest change; keying on `(package, target)` is
  equivalent. What the spec requires is only that the count printed beside a row
  is that row's count, and that the missing-registry error path
  (`xtask/src/proof.rs:267-290`) survives for both.
- **The parity guard's scan should match on identifier boundaries**, not
  substrings — a rule named `commit_is_atomic_with_the_read_model` must not
  false-positive on a doc comment mentioning commit atomicity in prose. Reporting
  the offending line makes the distinction reviewable when it does fire.
- **`no_orphan_rules` is the pattern to copy, not to fork**
  (`crates/happenstance-testkit/src/registry.rs:410-436`). It already obtains
  registered names via `for_each_event_store_rule!(crate::__emit_rule_names)` and
  compares them against a source scan; the projection guard is the mirror image
  (names must be **absent** from the harnesses rather than present in `suite.rs`).
- **The artefact's assertions want a home inside this PR's boundary.** The spec
  requires the assertions (AC-002, AC-003, AC-008), not the file they live in;
  `xtask/src/proof.rs`'s own `#[cfg(test)]` module is the one home already inside
  the boundary, and it can reach the document the same way `registry_len` reaches
  the registry — via `workspace_root()`. If the implementer finds a better home
  that does not widen the PR, take it and say so in the report.
- **Write the artefact last, and write it from the run's actual output** —
  transcribed, not reconstructed from memory or from this spec. The step counts,
  the skip lines and the rule count all come from stdout.
- **Two names, always.** Every sentence about `CheckpointOnlyStore` in the
  artefact has to be unambiguous about whether it is the conformance rule or the
  meta-test. The single easiest way to fail AC-002 is to write one name and let
  the reader assume the other.

## Tests and CI (merge gate)

Tiers are the project testing brief's own definitions (`_decomposition.md:760-768`):
**Static** reads/lints/builds without executing the code under test; **Unit** is
an in-process `#[test]`; **Integration** is a rule actually driving a
`ProjectionStore`; **E2E** is `cargo xtask ci` run whole, or a claim provable only
by inspecting that run's combined output.

| tier | command / path | proves |
| --- | --- | --- |
| **Static** | `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -D warnings` (`xtask/src/main.rs:105` `REQUIRED`) | NF-006 — this story's own diff clears the gate it records. |
| **Static** | `cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown` (`xtask/src/main.rs:231-243`) | AC-016's **mechanism** half: `projection_conformance_wasm.rs` type-checks on `wasm32`. Mandatory, pre-existing, and *insufficient on its own* — which is why AC-006 exists. |
| **Static** | `cargo hack check --workspace --feature-powerset --no-dev-deps` and the `wasm32` feature-powerset step (`xtask/src/main.rs:546-595`) | AC-007's powerset half: `conformance` and `unstable-projection` are not silently coupled to `memory`; thirty-two combinations, up from eight (`_decomposition.md:483-487`). |
| **Static** | `cargo xtask spec-trace` (`REQUIRED`) | Recorded as green in the artefact. This story does not make it green — HS-S0015 did (AC-008's attribution honesty). |
| **Unit** | `crates/happenstance-testkit/tests/projection_harness_parity.rs` | **AC-006** — every enumerated rule reaches all three emitters through the single enumeration; no harness lists a rule by hand; exactly one `projection_store_conformance!` per harness. EC-002, EC-003. |
| **Unit** | `xtask/src/proof.rs` `#[cfg(test)]` — registry keying and artefact-content assertions, run by `cargo test --workspace` | **AC-005** (each row's count is its own), **AC-002 / AC-003** (both test names and both fixture names present in the artefact), **AC-008** (no ratio over the mutant set). |
| **Unit** | `crates/happenstance-testkit/tests/projection_mutation_coverage.rs` (landed by HS-S0008) | The exactness claim the artefact quotes: `CheckpointOnlyStore` fails **exactly** its declared rules. Read by this story, written by that one. |
| **Integration** | `crates/happenstance-testkit/tests/projection_conformance.rs`, `projection_conformance_blocking.rs`, and the CF-5 buffering variant, inside the run's `tests` step | **AC-003** — two structurally unlike batch shapes drive the whole suite to completion in **one** invocation (`_decomposition.md:774`). |
| **E2E** | `cargo xtask ci` (whole, not `--fast`) on a clean checkout at the pinned sha | **AC-001** — the whole-workspace gate, including the skeletons compiling and the mandatory `wasm32` steps; **AC-004** via the *"each phase's proof artefacts"* step; **AC-007** via `run_steps`' ran/skipped output. Project **DoD 4** (`project.md:247-248`). |
| **E2E (CI)** | `.github/workflows/ci.yml:204-238` — `conformance on wasm32` (`wasm-bindgen-test-runner`, `wasm-bindgen-cli` pinned from `Cargo.lock`) | The **execution** half of AC-016's *"runs and passes"*, which the local gate does not make. AC-008 requires the artefact to attribute it here and cite the CI run read. |
| **E2E (CI)** | `.github/workflows/ci.yml:241-278` — `minimum supported Rust version` | Named in the artefact's limits section as **not covered locally** (AC-008). Not run by this story. |
| **Static (backlog)** | `redkiln validate` on this project's items; `redkiln verify --grain story` against `_ledger.md` | Every AC in this spec has a ledger row carrying real evidence before `implement → report`. |

Story-grain during implementation is `cargo xtask affected --base main` plus
`cargo xtask ci --fast`; the **merge bar for this story is the full run**, because
this story *is* the project boundary run (`_decomposition.md:828-853`;
`project.md:247-248`).

## Risks and coupling (PR-scoped)

| risk | why it bites here | containment |
| --- | --- | --- |
| **The run is last, so a defect found here has no repair budget in this story.** | Everything executable landed in slices 1–7; this is where a missing mutant or an oracle-shaped second fixture first becomes visible in one place. | EC-007: report against the owning story and fix there. The artefact may name the finding. Widening this PR to repair a rule is the scope failure this row exists to name. |
| **The registry-keying fix touches a code path the event-store row already depends on.** | `check()` and `registry_len()` are shared; a keying change that drops the event-store row's count, or breaks its missing-registry error path, is a silent regression in a *different* proof artefact. | AC-005's unit test asserts over **all** artefacts, not just the new ones; the gate's stdout must still show the event-store count beside its own row (EC-005). |
| **The parity guard is a textual scan and could false-positive.** | A rule name appearing in a harness doc comment would fail a naive substring match, and a guard that cries wolf gets `#[ignore]`d — which is exactly what `ARTEFACTS` then catches, loudly, at the worst time. | Identifier-boundary matching; the failure message names the file and line so a false positive is diagnosable in seconds (NF-002, EC-002). |
| **HS-S0015 or HS-S0016 slips.** | The run is then evidence about a tree that no longer exists — the fourth wrong implementation `discover.md` names (`:109-114`). | Hard ordering: this story does not start its run until both are merged. The recorded sha must be a descendant of both (AC-001). |
| **The artefact's `file:line` citations rot.** | `references/evaluation/`'s own named failure mode is citation drift that still passes `spec-trace`, because `spec-trace` reads `SPECIFICATION.md`, not this document. | Pin the commit and take the immutable arm — supersede rather than edit (AC-009, `references/evaluation/README.md:40-55`). |
| **A green run with skipped optional steps gets recorded as "green".** | Both tools resolve on this machine, so a skip means something changed about the environment — and that is precisely the run whose claim is weakest. | AC-007 makes ran-versus-skipped a per-step row rather than a footnote; EC-004 forbids describing such a run as fully green. |
| **The artefact drifts into a verdict.** | It is the document HS-P0015 and HS-P0016 read, which is exactly the pressure that turns evidence into a recommendation. | AC-008: no freeze verdict, no exposure verdict, no PS-3 restatement (link it), no pass rate in any form (ADR-0010). |
| **Coupling to `projection_mutation_coverage`'s internal `mod` shape.** | `ARTEFACTS`' fully-qualified names assume the target wraps its tests in a `mod` of its own name (`xtask/src/proof.rs:64-69`). | EC-008: transcribe from `-- --list`, report the deviation to HS-S0008, do not repair it here. |

## Dependencies

**Blocks on** (must be merged before this story's run is made — the run is
evidence about the tree they produce):

- `unstable-projection-gate-and-clause-disposition` (HS-S0015) — the
  `unstable-projection` gate on `crates/happenstance-core/src/projection.rs` and
  every `PS-1` – `PS-37` maturity marker, plus a green `cargo xtask spec-trace`.
  Without it the run records a module that still calls itself provisional, and
  AC-001's *"after both merged"* is false.
- `documented-extension-surface` (HS-S0016) — DT-8's arm discharged. It is the
  last change the project makes to what the suite holds implementers to; a run
  before it is a run of a different extension surface.

Both are slice-mates in `port-disposition-and-freeze-record`, and HS-S0015 lands
first inside the slice (`_storymap.md:114`).

**Unlocks** — **no story in this project.** This is the last row of the merge
order (`_storymap.md`, *Merge order*, item 8), so nothing downstream of it is a
story slug. What it unlocks is at the project grain and is named here so the
hand-off is not lost:

- `ladybug-projection-store` (HS-P0015) — writes the **freeze verdict**, and reads
  this artefact to write it (`project.md:56-58`). This story makes no verdict.
- `publication-and-positioning` (HS-P0016) — decides the `unstable-projection`
  **exposure at publish**, supplied with evidence by this project
  (`project.md:129-131`, AC-015).
- This project's own **closeout**, which quotes the failing test's name
  (`project.md:241-243`) — that quotation is copied from AC-002's section of the
  artefact rather than re-derived.

## Anchors (progressive disclosure)

Link, do not paste. Everything needed to *start* is in the Context pack above;
these are what to open at the moment named.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `xtask/src/proof.rs` | The mount. Its opening paragraphs carry the argument this story inherits (naming a target catches deletion, asserting names catches emptying), the `Artefact` shape, the subset-check rule, and `registry_len`'s package-keyed defect verbatim. | **First**, before writing anything — it is both the file being edited and the rationale for the edit. | AC-002, AC-004, AC-005 |
| `xtask/src/main.rs` | The `REQUIRED` / `OPTIONAL` step tables and `run_steps`' probe-and-skip contract — the source of the ran-versus-skipped ledger, and of what `--fast` drops. | Before recording the run ledger, and before claiming what `cargo xtask ci` covers. | AC-001, AC-007 |
| `crates/happenstance-testkit/src/registry.rs` | `no_orphan_rules` and `__emit_rule_names` — the exact pattern the parity guard mirrors, including how names are obtained from the enumeration rather than from a list. | Before writing `projection_harness_parity.rs`; copy the shape, do not invent one. | AC-006 |
| `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` | The existing `wasm32` harness this project's projection sibling was modelled on — what a correct, generated harness actually looks like, and therefore what the guard must accept. | While writing the guard's positive case, so it does not reject the shape the repo already ships. | AC-006 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | The event-store meta-tests the phase-6 rows are the sibling of, and the `REGISTRY: &[Declared]` shape `registry_len` parses. | When transcribing names into `ARTEFACTS` and when keying the registry path. | AC-004, AC-005 |
| `references/evaluation/README.md` | The directory's lifecycle contract: which arm a new document takes, what a registration row must carry, and the citation-drift failure mode this corpus names about itself. | Before creating the evidence document, not after — the arm chosen changes how the document is written. | AC-009 |
| `.github/workflows/ci.yml` | The two jobs the local gate does not make: `conformance on wasm32` (execution) and `minimum supported Rust version`. Attribution comes from here. | When writing the artefact's limits section. | AC-008 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The accepted decision behind the no-pass-rate prohibition and behind "a rule that no adapter can fail is decorative" — the reason the artefact is nouns rather than a ratio. | Before writing any number into the artefact. | AC-008 |
| `spec/SPECIFICATION.md` | PS-2's clause and its *Rejects* text — the bar this project's two testkit instruments do **not** clear, cited without amendment. | Only when writing the limits section's PS-2 sentence. Do not edit this file. | AC-008 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` | DoD 1, DoD 2, DoD 4 and DoD 8 in their own words — the four sentences the artefact is graded against, including *"a green gate is a precondition… never a substitute"*. | Before drafting the artefact's structure. | AC-001, AC-002, AC-003 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | The testing brief's tier definitions, merge-gate command grains, the same-run requirement for both fixtures, and the powerset widening (eight → thirty-two). | When building the run ledger and when deciding what tier each claim sits in. | AC-003, AC-007 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/whole-gate-run-and-proof-artefact/discover.md` | The four wrong implementations this story rejects, stated as failures rather than rules — including the two-names trap and the retrospective-run trap. | Re-read at the moment of writing the artefact; it is the review checklist for the deliverable. | AC-002, AC-001 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/projection-mutant-registry/spec.md` | Names the projection meta-tests and the conformant-variant positive control this story's `ARTEFACTS` rows hold — and its Integration contract's *"no manifest edit required"*, which is exactly the gap this story closes. | Immediately before transcribing `ARTEFACTS` names; check what that story actually landed. | AC-004, AC-002 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/projection-suite-entry-point/spec.md` | The three harnesses and the single enumeration the parity guard is about, and the conformance-rule name `CheckpointOnlyStore` fails. | Before writing the guard and before naming the rule in the artefact. | AC-006, AC-002 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | The signed-off `surfaces: []` determination and its sign-off note — the authority for this story rendering no surface and re-deciding no presentation. | If any question arises about whether something here is a surface. | AC-009 |
| `CHANGELOG.md` | The `[Unreleased]` section's existing shape and register, and the boundary against CF-29's rule-name obligation. | When writing the single entry, last. | AC-010 |

## Clarifications resolved during spec

1. **Where the artefact lives** (`discover.md` question 1) — `references/evaluation/phase-6-projection-proof.md`, registered in that directory's README under *"Later additions, which are neither"*, taking the immutable arm. `references/` is the repository's own home for *evidence kept for citation, binding nothing* (`CLAUDE.md`), which is exactly this document's status: the closeout and HS-P0015 cite it; nothing in the gate is bound by its prose.
2. **How a failure is demonstrated in a run whose exit code is zero** (question 2) — by naming **two** tests and saying which is which. The gate exits zero *because* the mutant failed where it was declared to; the artefact quotes the meta-test that asserts that, the conformance rule the mutant fails, and one command that shows the rule failing on its own. This is AC-002, and conflating the two names is its single most likely failure.
3. **Ran versus skipped** (question 3) — one artefact row per `OPTIONAL` step, carrying its probe and its outcome, read from `run_steps`' own stdout. AC-007, EC-004.
4. **The MSRV** (question 4) — answered *no* at discover and kept: the artefact states the MSRV as **not covered**, attributed to CI's `minimum supported Rust version` job. AC-008.
5. **AC-016's second half needed a mechanism, and it is a test rather than a review.** The verification half was specified as prose at discover (*"verify it held"*). This spec fixes it as `crates/happenstance-testkit/tests/projection_harness_parity.rs`, because a reviewed `rg` cannot fail twice — it is not re-run when the eighteenth rule lands — and this repository's own gate-design argument is that a check whose absence nothing notices is decorative. A `#[cfg(test)]` unit test beside `no_orphan_rules` was the other candidate and lost, because `ARTEFACTS` can only name a **test target**, so the guard could not itself be held.
6. **A defect in `proof.rs` was found while specifying, and taking it is in scope.** `check()` selects the registry row-count by package alone and `registry_len()` reads one hard-coded event-store path, so a second `happenstance-testkit` row prints a number that is not about the target it sits beside. This story is what makes that reachable, so it fixes it (AC-005) rather than shipping a wrong number and a follow-up.
7. **The AC set is exactly AC-001 – AC-010, as the front half decided.** None added, none dropped. AC-013's whole-workspace half sits inside AC-001 (skeletons compiling is a property of that run, not a separate observation), and AC-016's two halves are split across AC-006 (verification, this story's) and the Tests table's Static row (mechanism, pre-existing).
8. **`spec/SPECIFICATION.md` is read, never edited.** PS-2 and PS-3 are cited in the limits section; clause disposition was HS-S0015's and any frozen-clause repair landed as a new accepted atom in slice 1 (`discover.md:135-139`).
9. **The interaction-quality composition family is N/A by a signed-off determination**, not by silence — `_design.md`'s `surfaces: []` and its sign-off (`:48-50`, `:96-101`). The state-family invariants that do apply were translated into this medium and each attached to an existing AC row rather than left as prose, so every one of them is gated.
