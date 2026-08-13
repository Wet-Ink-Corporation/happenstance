---
item: HS-S0104
stage: spec
created: 2026-08-12T13:47:43.978Z
updated: 2026-08-12T13:47:43.978Z
template_sig: 87bbf1d0
rendered_sig: e955b976
---

# Spec — The gate is taught about a fourth suite

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project (charter) | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/gate-mounts-for-the-sync-suite/spec.md` |
| Key briefs | `…/replication-identity-and-ingest/_decomposition.md` — *architecture: Composition root* §6 **Gate mounts** (`:162-190`), the touched-files table (`:43-46`), AC-A05 and AC-A06; *testing: Merge-gate commands* (`:630-665`) |
| Story map row | `…/replication-identity-and-ingest/_storymap.md`, *Slices*, `sync-conformance-suite` row 2; merge order §3 |
| Signed-off design | `…/replication-identity-and-ingest/_design.md` — **no user-facing surface**, approved 2026-08-12. No surface id to render; nothing in this story's diff is a public library item at all |
| Discover stage (this story) | `…/gate-mounts-for-the-sync-suite/discover.md` — the signal ledger, the three named wrong implementations, and the four questions this spec settles |
| Roadmap pointer | `RUNBOOK.md:4516-4620` (phase 13); the `!Send`-path work item at `RUNBOOK.md:4588-4590`, the proof artefact at `:4597-4602` |

## One-line PR slice

Teach every gate constant that names only `happenstance-testkit` about the fourth suite —
`RULE_FILES`, `TESTKIT_SRC`, `TESTKIT_MANIFEST`, the position-literal lint and the
changelog-per-rule lint — and add the two `wasm32` steps (a build of `happenstance-sync` and a
check of the sync harness) beside the existing four.

## Executive summary

**This PR lands no rule, no fixture and no library code. It lands the reason the next story's
rules can fail anything.**

`crates/*` is a workspace glob (`Cargo.toml:3`), so `happenstance-sync-testkit` became a
workspace member the moment HS-S0103 created its directory — which is precisely how it escapes
every check that names a crate explicitly. Five constants hard-code the one testkit:
`RULE_FILES` (`xtask/src/spec_trace.rs:85-89`), which is both `spec-trace`'s sweep set *and*
CF-6's lint scope; `TESTKIT_SRC` (`xtask/src/lints.rs:42`) and `TESTKIT_MANIFEST` (`:45`), which
are CF-33's and CF-32's scopes; and the changelog lint, which resolves rule names out of the same
`RULE_FILES` (`xtask/src/lints.rs:525-595`). Separately, the four `wasm32` steps
(`xtask/src/main.rs:203-283`) name the contract crate, the *existing* harness, Cloudflare and
Neon, and none of them compiles a line of the sync path.

The delta over HS-S0103, which built the crate: after that story the sync suite exists and is
invisible. Its rules can be orphaned, its clauses render `scheduled` for ever, it may assert
literal positions, it may fold its `version` back into the workspace, it owes no changelog entry,
and SY-17's `[FROZEN]` `Rule:` field — literally *"`happenstance-sync-testkit` compiling its own
suite against a `!Send` fixture peer holding a `!Send` store behind an `Rc`"*
(`spec/SPECIFICATION.md:6341-6347`) — has nothing compiling it. This PR closes all six holes and
then proves each closure can fail, which is the half that separates a mount from a green line in
CI output.

One consequence is not obvious from the story title and is settled here rather than discovered in
review: **the specification's prose stops being true the moment the constants change.** CF-6's
`Rule:` field enumerates *"the three files rules live in — `suite.rs`, `model.rs` and
`concurrency.rs`"* (`spec/SPECIFICATION.md:7233-7245`) and CF-33's scopes itself to
`happenstance-testkit/src` (`:8236-8250`). Those sentences describe a mechanism this diff
supersedes, so the repair travels in the same change — under
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, MUST verbatim, prose only.

## Context pack

The decisions this story must honour. Each is stated as a decision; the deeper artefacts sit
behind the anchors.

**1. The wrong implementation is the diff that is *not* written, and it is green.** Land the sync
suite and change nothing under `xtask/`: `cargo fmt`, `cargo clippy -D warnings`,
`cargo test --workspace --all-features`, `cargo xtask lints`, `cargo xtask spec-trace` and
`cargo xtask ci --fast` are all green, because every one of them looks only where it was told to
look. The specification goes on reporting replication clauses as awaiting rules while the rules
sit in the tree enforcing them (`…/gate-mounts-for-the-sync-suite/discover.md`, *The wrong
implementation*). This is the **decorative-suite** mutant and its signature is that it looks like
*more* work done. Every check this story widens must therefore be shown to **fail** on the sync
crate — not shown to run over it.

**2. `RULE_FILES` grows a fourth entry; it does not become a per-suite structure — this time.**
The array lives beside `collect_rules` deliberately, *"so a list of files kept next to the
function that parses them cannot drift from it"* (`xtask/src/spec_trace.rs:83-89`), and it feeds
two different checks: `spec-trace`'s check 6, where every rule in the set must be claimed by a
clause, retired by one, or carried in `UNCLAIMED_PENDING_ADR` (`:765-800`), and CF-6's
position-literal lint, which iterates the same array (`xtask/src/lints.rs:628-651`). A flat
fourth entry is the smallest correct change and gets both checks in one edit. What it costs is
real and is accepted with its eyes open: *"orphaned in which suite?"* becomes unanswerable from
the array, and a clause cannot say which suite claims its rule. The trigger that would force the
structure is named rather than left to taste — **the first time a rule name exists in both
suites**, because check 6's `claimed` set is keyed on the bare name and would then let one
suite's clause discharge the other suite's rule. Until then a per-suite structure touches every
call site to buy nothing.

**3. `TESTKIT_SRC` and `TESTKIT_MANIFEST` become *scoped lists*, and the scope stays `src/`.**
CF-33's comment states the failure mode it is guarding against: `src/` and not the whole crate,
because `tests/` holds racers that may legitimately synchronise, and *"a whole-crate grep would
fire on `tests/` and teach the next person that the remedy is to widen the exclusions, which is
the direction that ends with the check switched off"* (`xtask/src/lints.rs:33-42`). So the edit
adds a second **scope**, never a broader glob and never an exclusion. Same for CF-32: the second
manifest is checked by the same three-line hand parse, and the failure messages must name *which*
manifest, because `{TESTKIT_MANIFEST}` interpolated into a message that now covers two crates
reports the wrong file half the time (`xtask/src/lints.rs:289-345`).

**4. Two `wasm32` steps, and the second is not implied by the first.** A build of
`happenstance-sync` proves the *library* compiles without `Send`. It says nothing about the
*suite*, because a conformance harness lives behind `cfg(target_arch = "wasm32")` and compiles to
nothing on a native run — which is exactly why the existing harness step exists separately and
says so (`xtask/src/main.rs:219-244`). The sharp mutant here is the partial fix: add the crate
build, tick AC-009, and leave SY-17's frozen `Rule:` field discharged by a step that cannot fail
for the reason the clause names.

**5. Neither new step may carry a probe.** A step skips only when its probe fails to find a tool
(`CLAUDE.md`, *Commands*); the mandatory `wasm32` build of the contract crate is deliberately a
plain `cargo check` with `probe: None` because *"a constraint whose only check is skippable is
unguarded on every machine that lacks one tool"* (`xtask/src/main.rs:196-218`). Both new steps go
in `REQUIRED` with `probe: None`, and both are added to `wasm_steps()` **by name** — that
function panics on a name that resolves to nothing precisely because it replaced an index that
once silently selected clippy (`xtask/src/main.rs:769-791`). The *feature powerset* on this
target is the one thing that may sit in `OPTIONAL` behind the `cargo hack` probe
(`xtask/src/main.rs:558-592`), because it widens coverage above a mandatory plain check rather
than replacing one.

**6. The document and the gate must agree at the end of this diff, and the repair travels with
it.** CF-6, CF-29, CF-32 and CF-33 are `[FROZEN]`, and CF-33 states the principle in its own
words: a step that quietly checks more than the clause specifies *"makes the gate and the
document disagree about what the bar is, with the gate winning silently"*, and extending it *"is
an edit to this line, in the same change"* (`spec/SPECIFICATION.md:8236-8258`). Apply the
playbook's mechanical test: **is there an implementation that was conformant before the edit and
is not after?** No — the MUSTs are scope-neutral ("no conformance rule may read a clock"; "no
rule may assert on a literal sequence-position value"), and what changes is only the *description
of the mechanism*, which the freeze does not protect
(`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, *The test: repair or gap*).
So this is a **repair**: keep every MUST verbatim, correct the `Rule:` prose to name both suites,
change no maturity marker and no `Rejects:` line. If the implementer finds a change that fails
that test — a normative sentence that would have to move — it is a **gap**, and the correct
output is a recorded finding and a blocker raised to ADR-0026's author, never a smaller edit.

**7. Repairing a `Rule:` line moves a generated region, so `spec-trace --write` is part of the
work.** §7.1 and §7.2 are held *equal* to what the tool computes, not merely regenerable
(`xtask/src/spec_trace.rs:735-744`), and §7.2's rule cell is the `Rule:` text truncated to 79
characters (`spec/SPECIFICATION.md:8717`). Editing four `Rule:` lines therefore turns
`spec-trace` red in `Mode::Check` until the region is regenerated through
`cargo xtask spec-trace --write` — regenerated by the tool and never by hand, which is the whole
point of the equality.

**8. Land the checks against a green baseline of zero, deliberately.** No sync rule exists yet
(they are HS-S0105's), so every widened check passes vacuously on the day it lands — and that is
the ratchet order, not an accident: land the stricter gate while the baseline is green so the
first rule that arrives without a changelog entry, or with a literal position, or claimed by no
clause, is the change that goes red
(`.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md`). The obligation this creates
is the one in decision 1: a vacuous pass and a real pass are indistinguishable from the exit
code, so the non-vacuity must be asserted in `xtask`'s own tests rather than observed once by the
author.

**9. Who this is for, and what they observe.** The persona is the adapter author of
`.kb/product/` — the one who reads a clause to find out what the bar is. Today they read SY-1 or
SY-17, follow the `Rule:` field to a rule, and find the clause rendered as *scheduled* while the
rule exists; after this story the clause's citation is resolvable and every sync rule is on the
hook for a clause, a changelog entry and a mutant. Nothing about their *API* changes here. What
changes is that the specification stops being a document that describes checks nobody runs — the
failure mode `CLAUDE.md` names as decorative and this repository has hit repeatedly (the CF-29
lint's first run: twenty-five of fifty-five rules with no entry, `xtask/src/main.rs:394-405`).

**10. The seams with the slice-mates, stated so none of the three re-decides them.**
`sync-testkit-crate-and-rule-registry` (HS-S0103) owns the crate, its `version` key, its
`publish = false`, the fixture contract and the `!Send`/`Rc` harness this story compiles for
`wasm32` — if that harness is absent, this story is **blocked**, and a stand-in harness written
here would discharge SY-17 against something nobody has to keep working.
`headline-rules-and-mutant-registry` (HS-S0105) owns every rule, mutant and changelog entry, and
depends on this story for the ability to fail. `frozen-clause-repairs` (HS-S0114) owns dropping
the `(new, happenstance-sync-testkit)` markers from `SY` `Rule:` lines once the rules exist
(`xtask/src/spec_trace.rs:1626-1634` is what makes `(new)` mean unresolved) and every `Rejects:`
repair; this story makes those resolutions *possible* and performs none of them.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate (the gate itself), consumed by
  `headline-rules-and-mutant-registry` in this same slice. No double, no flag, no `todo!()`.
- **Slice / milestone**: `sync-conformance-suite`. Slice-mates:
  `sync-testkit-crate-and-rule-registry` (HS-S0103, predecessor),
  `headline-rules-and-mutant-registry` (HS-S0105, successor). All three are implemented in one
  context and mounted as one integrated surface; merge order is fixed at
  `…/replication-identity-and-ingest/_storymap.md`, *Merge order* §3.
- **Mount point**: **`xtask/src/main.rs`** — the `REQUIRED` step table (`:203-283` for the
  `wasm32` block) and the `wasm_steps()` name selector (`:784-791`). This file *is* the gate's
  composition root: `CLAUDE.md` (*Commands*) states it is defined once here and is exactly what
  CI runs. A step that is not in this table is not in the gate.
- **Wires into**:
  - `xtask/src/spec_trace.rs:85-89` — `RULE_FILES`, consumed by `check_rule_ownership` (`:765`),
    `retired_rules` (`:857`), `all_rules` (`:1748`) and `lints::no_position_literals` (`:628`).
  - `xtask/src/lints.rs:42`, `:45` — `TESTKIT_SRC` (CF-33) and `TESTKIT_MANIFEST` (CF-32),
    consumed by `no_clock` (`:231`) and `testkit_version` (`:289`).
  - `xtask/src/lints.rs:525` — `changelog_names_every_rule`, which resolves its rule set out of
    `RULE_FILES` and reads `CHANGELOG.md`.
  - `xtask/src/affected.rs:118-125` — the file-reading checks the **story grain** runs
    unconditionally; they call the same four functions, so widening the constants widens the
    per-story gate at no extra wiring.
  - `crates/happenstance-sync-testkit/` (HS-S0103) — the crate the constants must name, its
    `Cargo.toml` `version` key, its `src/` rules module and its `cfg(target_arch = "wasm32")`
    harness.
  - `crates/happenstance-sync/` — the crate the fifth `wasm32` step builds, and the `memory`
    feature (`crates/happenstance-sync/Cargo.toml:37-44`) the powerset step exercises on that
    target.
  - `.redkiln/config.yaml:40-55` — `affected_gate`, `reachability_static`
    (`cargo xtask lints && cargo xtask spec-trace`) and `integration_scoped`
    (`cargo xtask ci --fast`) all run these constants whether or not anyone types them.
- **Renders surfaces**: **none.** `…/replication-identity-and-ingest/_design.md` records no
  user-facing surface for this project, and this story is further from one than any of its
  siblings: it adds no public Rust item at all. `xtask` is not published
  (`CLAUDE.md`, *Repository map*).
- **Public items**: none. The only externally visible names this story creates are two `Step`
  names in a private `const` table, and they are load-bearing as *strings* — `wasm_steps()`
  selects them by name and panics on a miss.
- **Conformance rule(s)**: **none added, and this story is why the next story's rules can fail
  anything.** It is not adapter-observable: no adapter can pass or fail differently because of
  this diff. What it changes is which checks an adapter's *testkit* is subject to. The
  substitute obligation, per `CLAUDE.md`'s *a rule that no adapter can fail is decorative*
  corollary applied one level up, is AC-002 and AC-010: each widened check is demonstrated to
  reject something.
- **Clause(s)**:
  - **Repaired** (prose only, MUST verbatim): **CF-6** (`spec/SPECIFICATION.md:7233-7249`),
    **CF-29** (`:8141-8168`), **CF-32** (`:8200-8228`), **CF-33** (`:8236-8262`) — each `Rule:`
    field names a single-crate scope this diff supersedes. Under
    `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` this is a repair, not an
    amendment; no ADR is required and none is claimed. Project DoD 7 (`git diff` over
    `spec/SPECIFICATION.md` shows only what a record authorises) is the check.
  - **Made checkable, not changed**: **SY-17** (`:6341-6360`), whose `Rule:` field is a compile
    that nothing performed until the sixth `wasm32` step existed; and every `SY` clause whose
    `Rule:` field names a rule in `happenstance-sync-testkit`, which cannot resolve until
    `RULE_FILES` includes that crate (**project AC-014 cannot pass without this edit** —
    `…/_decomposition.md`, *Gate mounts*, first bullet).
  - **Amended**: none. No `[FROZEN]` normative sentence, maturity marker or `Rejects:` line is
    touched.
- **Advances DoD scenario**: initiative **DoD 4** — *"Every rule is green under the edge runtime
  on `wasm32`, executed in the gate rather than asserted in prose"*
  (`.bklg/from-contract-to-published-library/initiative.md:369-372`): this story is the
  *executed-in-the-gate* half for the replication path. Secondarily initiative **DoD 13** (the
  gate green on the assembled whole, including the cross-reference step) and project **DoD 2**
  (`cargo xtask lints && cargo xtask spec-trace` green, `.redkiln/config.yaml:48`).

## PR boundary

**In this PR**

- `xtask/src/spec_trace.rs` — `RULE_FILES` grows the sync suite's rule file(s); the message that
  names where a rule was looked for stops hard-coding `SUITE`.
- `xtask/src/lints.rs` — `TESTKIT_SRC` and `TESTKIT_MANIFEST` become scoped lists; `no_clock`,
  `testkit_version` and `changelog_names_every_rule` iterate them and name the offending crate in
  every message and every success line.
- `xtask/src/main.rs` — two new `REQUIRED` steps (`wasm32` build of `happenstance-sync`;
  `wasm32` check of the sync conformance harness), both `probe: None`, both added to
  `wasm_steps()` by name; the `wasm32` feature powerset in `OPTIONAL` gains the sync crates; the
  help text's step counts and descriptions follow.
- New `#[cfg(test)]` assertions in `xtask` (the non-vacuity guards of AC-010) — placed beside the
  code they guard, in the module-local `mod tests` style already used at
  `xtask/src/affected.rs:596` and `xtask/src/package.rs:408`.
- `spec/SPECIFICATION.md` — **`Rule:` prose repairs only** for CF-6, CF-29, CF-32, CF-33, plus
  §7.1/§7.2 regenerated by `cargo xtask spec-trace --write`.
- `CHANGELOG.md` — the entry recording the widened scopes and the two new steps.
- `crates/happenstance-sync-testkit/Cargo.toml` — **only** if the sixth step needs the
  dev-dependency split by target that `crates/happenstance-testkit/Cargo.toml:43-54` already
  carries (tokio's multi-threaded runtime cannot exist on `wasm32`). A manifest edit, never a
  fixture or rule edit.
- This story's own backlog folder (ledger, report).

**Explicitly not in this PR**

- Any conformance rule, mutant, fixture or changelog entry *for a rule* — HS-S0105's.
- The sync testkit's `src/**` and `tests/**` — HS-S0103's. A missing `!Send`/`Rc` harness is a
  blocker to raise, not a stand-in to write.
- Dropping the `(new, happenstance-sync-testkit)` markers from any `SY` `Rule:` line, and any
  `Rejects:` repair — `frozen-clause-repairs` (HS-S0114) owns both, and both need the rules to
  exist first.
- Any change to a normative sentence, maturity marker or `Rejects:` line — that is a gap, an ADR
  and a re-plan (project AC-002, DoD 7).
- `crates/happenstance-core/**` and any port signature. `EventStore`'s trait signature is
  byte-identical across this whole project (AC-A01).
- Publishing, or claiming, either sync crate name (project AC-015).

**Merge DoD**: `cargo xtask ci --fast` green with six `wasm32` steps rather than four, and each
widened check demonstrated — in `xtask`'s own tests — to fail when the sync suite violates it.

```
xtask/src/**
spec/SPECIFICATION.md
CHANGELOG.md
crates/happenstance-sync-testkit/Cargo.toml
.bklg/from-contract-to-published-library/replication-identity-and-ingest/gate-mounts-for-the-sync-suite/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The sweep set names the fourth suite** | `RULE_FILES` gains the sync suite's rule file(s) as flat entries. One edit reaches four consumers: check 6's ownership sweep, `retired_rules`, `all_rules` and CF-6's lint. A sync rule claimed by no `SY` clause then fails `spec-trace` **by name**, with the array's own message telling the author their three options (claim, retire, or `UNCLAIMED_PENDING_ADR`). | `xtask/src/spec_trace.rs:85-89`, `:765-800`, `:857-942`, `:1739-1755`; `xtask/src/lints.rs:628-651` |
| **The array is flat, and the trigger for a structure is written down** | Rejected: a per-suite structure keyed by crate. It touches every call site and buys an answer to *"orphaned in which suite?"* that nothing asks yet. It becomes necessary the day a rule name exists in both suites, because `claimed` is keyed on the bare name; that trigger is recorded in the array's doc comment beside the reason the array lives where it does. | `xtask/src/spec_trace.rs:71-89`; `…/gate-mounts-for-the-sync-suite/discover.md`, *Questions* 3 |
| **Where a rule was looked for is reported truthfully** | Check 4's message hard-codes `SUITE` as the place a missing rule was sought (`xtask/src/spec_trace.rs:699-708`), and `retired_rules`' `defined_in` falls back to `SUITE` (`:888-894`). With four files across two crates, both send the reader to the wrong file. Each reports the file set actually searched. | `xtask/src/spec_trace.rs:690-710`, `:886-920` |
| **CF-33's no-clock scope covers both testkits' `src/`** | `TESTKIT_SRC` becomes a list of scopes; `no_clock` iterates it, keeps the existing per-scope empty-directory `bail!`, and reports the scanned count per scope. `src/` only — `tests/` stays out for the reason the constant's own comment gives, and no exclusion is added. | `xtask/src/lints.rs:33-42`, `:231-281`; `spec/SPECIFICATION.md:8236-8262` |
| **CF-32's own-version check covers both manifests** | `TESTKIT_MANIFEST` becomes a list; each manifest is parsed by the same three-line hand parse and each failure message names *which* manifest and repeats why the number must move independently. `version.workspace = true` in `crates/happenstance-sync-testkit/Cargo.toml` fails the gate. | `xtask/src/lints.rs:44-45`, `:289-345`; `xtask/src/main.rs:420-438` |
| **CF-29's changelog lint covers sync rules from the day they land** | No code change is needed beyond `RULE_FILES` — the lint already resolves its rule set from it — but the success line must state the file count it swept so a silent narrowing is visible, and the prose-per-rule budget applies unchanged. Landing at a zero-rule baseline is deliberate: the first sync rule without an entry is the change that goes red. | `xtask/src/lints.rs:504-595`; `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md` |
| **Fifth `wasm32` step: the sync library builds for the constrained target** | `cargo check --locked -p happenstance-sync --target wasm32-unknown-unknown`, `probe: None`, in `REQUIRED` beside the existing four and in `wasm_steps()` by name. Proves the port and the runner compile where `Send` is unavailable. | `xtask/src/main.rs:192-283`, `:784-791`; `standards/rust/52-wasm32-and-target-cfg.md` |
| **Sixth `wasm32` step: the sync *harness* type-checks for the constrained target** | `cargo check --locked -p happenstance-sync-testkit --tests --target wasm32-unknown-unknown`. `--tests`, not `--all-targets`, for the reason the existing harness step gives. This is the only thing that discharges SY-17's `[FROZEN]` `Rule:` field, which is a compile of the suite against a `!Send` fixture peer holding a `!Send` store behind an `Rc`. A crate build does not imply it: the harness is behind `cfg(target_arch = "wasm32")` and compiles to nothing natively. | `xtask/src/main.rs:219-244`; `spec/SPECIFICATION.md:6341-6360` |
| **Neither new step can skip** | Both carry `probe: None` and live in `REQUIRED`, so `cargo xtask ci --fast` — the non-terminal project's integration bar — runs both. Both are selected in `wasm_steps()` by name, where a name that resolves to nothing panics rather than silently selecting a neighbour. | `xtask/src/main.rs:196-218`, `:769-791`, `:835-860`; `CLAUDE.md`, *Commands* |
| **The `wasm32` feature powerset learns the sync crates** | `happenstance-sync` carries a non-default `memory` feature and a feature is not target-scoped, which is the exact shape that caught `happenstance-testkit`'s `proptest` feature on this target. The powerset step gains `-p happenstance-sync` (and `-p happenstance-sync-testkit` if it declares a feature). It stays in `OPTIONAL` behind the `cargo hack` probe — it widens coverage above a mandatory plain check rather than replacing one. | `xtask/src/main.rs:558-592`; `crates/happenstance-sync/Cargo.toml:37-44` |
| **No check may pass vacuously** | `xtask`'s own tests assert that each widened constant resolves to files that exist and parse to rules (not that the list has four entries), that each lint scope resolves to a non-empty `.rs` set, and that each manifest path exists. A scope pointed at a directory the rules do not live in prints a success line, and a green line in CI is worse than an absent check. | `xtask/src/spec_trace.rs:873-882` (the existing vacuity guards this extends); `xtask/src/affected.rs:596`, `xtask/src/package.rs:408` (the in-module test style) |
| **The position-literal lint can fail on a sync rule** | Verified by writing a literal-position assertion into the sync rules file, running `cargo xtask lint-position-literals`, confirming it names the file and line, and reverting. CF-5's `GappedPositionStore` remains the behavioural enforcement and the lint the cheap second line, in that order. | `xtask/src/lints.rs:628-651`, `:704-720`; `spec/SPECIFICATION.md:7233-7249`; `crates/happenstance-testkit/tests/mutation_coverage.rs` |
| **The document says what the gate does** | CF-6, CF-29, CF-32 and CF-33 `Rule:` fields are repaired to name both suites — MUST verbatim, `Rejects:` untouched, maturity markers untouched — and §7.1/§7.2 are regenerated with `cargo xtask spec-trace --write`, never by hand, because the region is held *equal* to what the tool computes. | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`; `spec/SPECIFICATION.md:8717`; `xtask/src/spec_trace.rs:735-744`, `:1243-1265` |
| **A repair that is really a gap stops the story** | The playbook's test is mechanical: if any correction would change the set of implementations a clause admits, it is a gap — record the finding, raise the blocker, write no smaller edit. Project AC-002 and DoD 7 are the checks. | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, *The test: repair or gap*; `…/replication-identity-and-ingest/project.md`, AC-002, DoD 7 |
| **The story grain inherits all of it** | `cargo xtask affected` runs `retired_rules`, `no_clock`, `no_position_literals`, `changelog_names_every_rule`, `testkit_version` and `spec-trace` unconditionally, before any package selection. Widening the constants therefore widens the per-story gate for every later story in this project with no further wiring. | `xtask/src/affected.rs:112-126`; `.redkiln/config.yaml:28-40` |

## Data and migrations

**N/A — no persistent data, no schema, no stored state.** This story's entire diff is compile-time
constants, gate steps, `xtask` tests and markdown; nothing it touches is read at runtime by a
library consumer, and no adapter's storage is involved.

The nearest analogue, recorded here because it is the thing a migration section would otherwise
be asked to carry, is a **ratchet ordering rather than a data migration**: four checks are widened
while the sync suite holds zero rules, so every one of them lands green and the first violating
change is what turns them red
(`.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md`). The obligation that ordering
creates is discharged in this spec rather than deferred — see the non-vacuity rows in *Behavior
and interfaces*, because a check that lands green over an empty set and a check that lands green
over the wrong directory produce the identical exit code.

## Acceptance criteria

Every criterion is framed from the intent of a persona this initiative names, carried from
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` (the
`.kb/product/` layer is structurally present and functionally empty —
`.bklg/from-contract-to-published-library/initiative.md:227-234`). Two personas do the work here:
**the adapter author** (Persona 2, `:114`) on the journey *Learn when you are finished*, and **the
edge Rust developer** (Persona 3, `:182`) on *Event-source at the edge without hand-rolling it*.
The **evaluator** (Persona 4, `:249`) reads the specification as public evidence and is the reason
AC-011 exists at all.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author reading SY-1 to find out what the ingest bar actually is, **WHEN** they follow the clause's `Rule:` field to the named rule, **THEN** `spec-trace` resolves it against a file it genuinely sweeps — because `RULE_FILES` names the sync suite's rule file(s) as well as the three under `crates/happenstance-testkit/src/` — and the clause stops rendering as *scheduled* for the reason that it could not be found; **AND** a sync rule claimed by no `SY` clause fails `spec-trace` check 6 **by name**, with the array's own message offering claim, retire, or `UNCLAIMED_PENDING_ADR`. | `cargo xtask spec-trace` green with the widened sweep set; new `#[cfg(test)] mod tests` in `xtask/src/spec_trace.rs` asserting every `RULE_FILES` entry resolves to a file that exists under the workspace root and is parseable by `collect_rules` (`xtask/src/spec_trace.rs:85-89`, `:765-800`, `:857-942`, `:1739-1755`) |
| AC-002 | **GIVEN** an adapter author who reads a green gate as "the sync suite was checked", **WHEN** the sync suite is made to violate each widened check in turn — a rule claimed by no clause, a literal position inside a rule, `version.workspace = true` in the sync manifest, a clock construct under the sync suite's `src/`, and a rule with no `CHANGELOG.md` entry — **THEN** each check **fails**, naming the sync crate's file and the reason, so the **decorative-suite mutant** (land the suite, change nothing under `xtask/`, stay green) is dead. Demonstrating each check *runs* over the sync crate is explicitly not sufficient. | Five negative controls in `xtask`'s own `mod tests`, each driving the check's scope-taking inner function against a fixture tree and asserting the error names the offending path; for any check that cannot be driven without `workspace_root()`, a transient-violation transcript recorded verbatim in `implementation-report.md` alongside the revert (`xtask/src/lints.rs:231`, `:289`, `:525`, `:628`; `xtask/src/spec_trace.rs:765`) |
| AC-003 | **GIVEN** an adapter author running the sync conformance suite on a loaded CI runner, **WHEN** any sync conformance rule under `crates/happenstance-sync-testkit/src/` reads a clock, **THEN** CF-33's lint fails naming that file and line — because `TESTKIT_SRC` has become a list of **scopes** covering both testkits' `src/` — **AND** `tests/` in either crate stays outside the scope and no exclusion is added, because widening exclusions is the direction that ends with the check switched off. | `cargo xtask lint-clock` reporting a per-scope scanned count for both scopes; the existing per-scope empty-directory `bail!` preserved for each; unit test asserting each scope resolves to a non-empty `.rs` set (`xtask/src/lints.rs:33-42`, `:231-281`) |
| AC-004 | **GIVEN** an adapter author who must know whether a new sync rule was a semver-MINOR change to the *bar*, **WHEN** `crates/happenstance-sync-testkit/Cargo.toml` inherits its version from the workspace, **THEN** CF-32 fails and the message names **which** manifest is at fault and repeats why that number must move independently of the contract's — a message that interpolates one constant while covering two crates reports the wrong file half the time. | `cargo xtask lint-testkit-version` over both manifests, each parsed by the same three-line hand parse; unit test asserting the failure message for each manifest contains that manifest's own path (`xtask/src/lints.rs:44-45`, `:289-345`; `xtask/src/main.rs:420-438`) |
| AC-005 | **GIVEN** an adapter author who reaches `CHANGELOG.md` to learn what a new bar rejects, **WHEN** a sync rule lands with no entry naming a defect, **THEN** CF-29 fails from the day that rule appears — the lint resolves its rule set from `RULE_FILES`, so AC-001's edit is the whole mechanism — **AND** the success line states the file count actually swept, so a later narrowing of the sweep set is visible in the output rather than silent. | `cargo xtask lint-changelog` green at the zero-sync-rule baseline with a success line naming the swept count; the AC-002 negative control for a sync rule with no entry (`xtask/src/lints.rs:504-595`; `xtask/src/main.rs:394-405`) |
| AC-006 | **GIVEN** an adapter author sent by a gate failure to a file to fix a rule, **WHEN** the message says where the rule was looked for, **THEN** it names the file set actually searched rather than the hard-coded `SUITE` — check 4's "not found" message and `retired_rules`' `defined_in` fallback both currently send the reader to `crates/happenstance-testkit/src/suite.rs` regardless of which of the now-four files was swept, which is a wrong answer delivered with confidence. | `cargo xtask spec-trace` and `cargo xtask lint-retired-rules`; unit test asserting the "looked for in" text enumerates the swept set (`xtask/src/spec_trace.rs:690-710`, `:886-920`) |
| AC-007 | **GIVEN** the edge Rust developer who needs replication on Cloudflare Workers, where `Send` is unavailable, **WHEN** the gate runs, **THEN** a fifth `wasm32` step builds `happenstance-sync` for `wasm32-unknown-unknown` — so the port, the ingest seam and the runner are proved to compile without `Send` rather than asserted to in prose — **AND** the step sits in `REQUIRED` beside the existing four rather than replacing any of them. | `cargo xtask wasm` and `cargo xtask ci --fast` both executing `cargo check --locked -p happenstance-sync --target wasm32-unknown-unknown` (`xtask/src/main.rs:192-283`; `standards/rust/52-wasm32-and-target-cfg.md`) |
| AC-008 | **GIVEN** the same developer relying on SY-17's `[FROZEN]` promise that the sync port binds `EventStore` and not `SendEventStore`, **WHEN** the gate runs, **THEN** a sixth `wasm32` step type-checks the sync **conformance harness** (`--tests`, not `--all-targets`) on that target — which is the only thing that performs the compile SY-17's `Rule:` field literally names, a suite compiled against a `!Send` fixture peer holding a `!Send` store behind an `Rc` — **AND** the crate build of AC-007 is not accepted as discharging it, because a harness behind `cfg(target_arch = "wasm32")` compiles to nothing on a native run. | `cargo check --locked -p happenstance-sync-testkit --tests --target wasm32-unknown-unknown` as a `REQUIRED` step; `cargo xtask spec-trace` resolving SY-17's citation (`xtask/src/main.rs:219-244`; `spec/SPECIFICATION.md:6341-6360`) |
| AC-009 | **GIVEN** an adapter author on a machine missing an optional tool, **WHEN** they run the gate, **THEN** neither new `wasm32` step can silently skip — both carry `probe: None` and live in `REQUIRED`, and both are selected in `wasm_steps()` **by name**, where a name resolving to nothing panics rather than quietly selecting a neighbour — so `cargo xtask wasm` reports **six** steps, not four; **AND** the `wasm32` **feature powerset** gains the sync crates and stays in `OPTIONAL` behind the `cargo hack` probe, because it widens coverage above a mandatory plain check rather than replacing one. | `cargo xtask wasm` listing six steps; unit test asserting every name passed to `wasm_steps()` resolves in `REQUIRED` and that both new steps have `probe: None`; `cargo xtask ci --fast` running both (`xtask/src/main.rs:196-218`, `:558-592`, `:769-791`) |
| AC-010 | **GIVEN** a maintainer who cannot tell a vacuous pass from a real one by reading an exit code, **WHEN** any widened constant is pointed at a directory the rules do not live in, or at a file that has moved, **THEN** the gate **fails** rather than printing a success line: every `RULE_FILES` entry exists and parses, the aggregate rule set over `RULE_FILES` is non-empty, every lint scope resolves to a non-empty `.rs` set, and every manifest path exists. Asserting the array's *length* is explicitly not this criterion — a count passes while pointing at the wrong tree. | New `#[cfg(test)] mod tests` in `xtask/src/spec_trace.rs` and `xtask/src/lints.rs`, in the module-local style at `xtask/src/affected.rs:597` and `xtask/src/package.rs:409`; the existing emptiness `bail!`s extended per scope (`xtask/src/spec_trace.rs:873-882`; `xtask/src/lints.rs:236-238`) |
| AC-011 | **GIVEN** the evaluator reading `spec/SPECIFICATION.md` as public evidence of what this library actually checks, **WHEN** they read CF-6, CF-29, CF-32 and CF-33 after this change, **THEN** each `Rule:` field describes the mechanism that now exists — both suites, not one — with every MUST **verbatim**, every `Rejects:` line and maturity marker untouched, and §7.1/§7.2 regenerated by `cargo xtask spec-trace --write` rather than by hand; **AND** if any correction would change the set of implementations a clause admits, it is a **gap**: the story stops, records the finding and raises a blocker to ADR-0026's author, and writes no smaller edit. | `cargo xtask spec-trace` green in `Mode::Check` after `--write`; `git diff spec/SPECIFICATION.md` reviewed against project DoD 7 showing only `Rule:` prose and generated regions; the playbook's mechanical test applied and its answer recorded in `implementation-report.md` (`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`; `spec/SPECIFICATION.md:8717`; `xtask/src/spec_trace.rs:735-744`) |

**Traceability.** Project **AC-004** (the suite discriminates; no literal position) → AC-002,
AC-006, AC-010 — the lint scope half, the mutant registry itself being HS-S0105's. Project
**AC-009** (the constrained runtime keeps its runtime, built and exercised in the gate rather than
asserted in prose) → AC-007, AC-008, AC-009. Project **AC-014** (the clause arithmetic, with
`spec-trace` green) → AC-001, AC-005, AC-011 — *"AC-014 cannot pass without this edit"*
(`…/replication-identity-and-ingest/_decomposition.md`, *Gate mounts*, first bullet).

## Interaction quality

**Composition invariants: N/A, and declared rather than skipped.** The project's signed-off design
records **no user-facing surface** — no screen, no CLI TUI, no documentation site
(`…/replication-identity-and-ingest/_design.md`, *Surfaces*, approved by the repository owner
2026-08-12; its *Anti-patterns*, *States* and *Doctest* sections read `N/A — no user-facing
surface`). This story is further from a surface than any of its siblings: it adds **no public Rust
item at all**, and `xtask` is not published. There is therefore no composition, transience,
density budget, hierarchy or design anti-pattern to bind to, and `design.capture` is a declared
skip repository-wide (`CLAUDE.md`, *Where the work lives*). Inventing composition invariants here
would be inventing a surface the design stage explicitly refused.

**State invariants do apply, in the medium this story actually has: the gate's output and the
tree it runs over.** The unstyled-render analogue is exact — a widened check that runs and prints
green satisfies every "the constant contains four entries" assertion perfectly, and proves
nothing. Each invariant below is carried by an AC **row in the table above**, never as a bullet
here, so `redkiln verify` extracts it and the ledger gates it.

| Invariant (RFC §6.7/D6 family: state) | Carried by | How it is verified |
| --- | --- | --- |
| **Non-occlusion** — a failure names the offending file, line and *which* scope or manifest; a message covering two crates may not interpolate a single constant and report the wrong one. | **AC-004**, **AC-006** | Unit tests asserting the failure text contains the offending manifest's own path and the enumerated swept set |
| **In-place, not context-jump** — the path a message sends the reader to is a path they can open and find the rule in; `SUITE` as a fallback for a four-file set is a context jump to the wrong file. | **AC-006** | `cargo xtask lint-retired-rules` and `cargo xtask spec-trace` diagnostics over a rule defined outside `suite.rs` |
| **Reversibility** — every negative control is a transient violation that reverts to a byte-identical tree; the gate green before and after, and the demonstration recorded rather than remembered. | **AC-002** | `git status` clean after each control; transcripts in `implementation-report.md` |
| **No silent state change** — a step may not skip; a scope may not narrow; a success line must state what it swept, so a later narrowing is legible in the output instead of invisible. | **AC-005**, **AC-009**, **AC-010** | `cargo xtask wasm` listing six; per-scope counts in the success lines; `probe: None` asserted in a unit test |
| **Reachability** — every widened check is reachable from the commands the repository actually runs, not only from a hand-typed subcommand: the story grain (`cargo xtask affected`) runs all five file-reading checks unconditionally, and `cargo xtask ci --fast` runs both new steps. | **AC-007**, **AC-008**, **AC-009** | `.redkiln/config.yaml:40-55`; `xtask/src/affected.rs:112-126` |
| **Preserved meaning across an edit** — the document and the gate agree at the end of the diff; a `Rule:` repair that would move a MUST is a gap and halts the story rather than being trimmed to fit. | **AC-011** | `git diff spec/SPECIFICATION.md` against project DoD 7; the playbook's mechanical test recorded |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `crates/happenstance-sync-testkit/` does not exist — HS-S0103 has not landed, or landed without the rules module or the `!Send` harness. | **Blocked, not stubbed.** Every widened constant would point at nothing and the checks would `bail!` — which is correct behaviour, and the correct *story* behaviour is to raise the blocker. Writing a stand-in harness here would discharge SY-17 against something nobody has to keep working (*PR boundary*, second exclusion). |
| **EC-002** | A lint scope resolves to a directory that exists but holds no `.rs` files. | `bail!` **per scope**, naming that scope, in the existing shape at `xtask/src/lints.rs:236-238`. A whole-check emptiness guard would pass while one of two scopes scanned nothing. |
| **EC-003** | A `RULE_FILES` entry names a file that has moved or been renamed. | Fail. The array's contract is that it cannot drift from `collect_rules`, which is why it lives beside it (`xtask/src/spec_trace.rs:83-89`); a missing entry that reads as "no rules here" is the exact vacuity AC-010 forbids. |
| **EC-004** | A name passed to `wasm_steps()` resolves to no `REQUIRED` step. | **Panic**, deliberately and unchanged. The steps are compile-time constants, so a miss is a bug in `xtask/src/main.rs` and never a user error — this is why the selector replaced an index that once silently pointed `cargo xtask wasm` at clippy (`xtask/src/main.rs:769-791`). |
| **EC-005** | `wasm32-unknown-unknown` is not installed on the machine. | The step **fails**; it does not skip. `probe: None` is the whole point, and it is the same behaviour the four existing `wasm32` steps already have — a constraint whose only check is skippable is unguarded on every machine that lacks one tool (`xtask/src/main.rs:196-218`). |
| **EC-006** | `spec-trace` is red in `Mode::Check` immediately after the `Rule:` repairs. | Expected and transient: §7.1/§7.2 are held **equal** to what the tool computes and §7.2's rule cell is the `Rule:` text truncated to 79 characters. Regenerate with `cargo xtask spec-trace --write` — never by hand, because hand-editing a region held equal to a computation is how the equality stops meaning anything (`xtask/src/spec_trace.rs:735-744`; `spec/SPECIFICATION.md:8717`). |
| **EC-007** | A proposed correction to a `[FROZEN]` clause would change the set of implementations it admits. | **Gap, not repair.** Record the finding, raise the blocker to ADR-0026's author, write no smaller edit, and do not proceed by trimming the correction until it passes the test (`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, *The test: repair or gap*; project AC-002, DoD 7). |
| **EC-008** | A rule name exists in **both** suites. | Out of scope for this diff and recorded as the named trigger: check 6's `claimed` set is keyed on the bare name, so one suite's clause would discharge the other suite's rule. If it occurs, the flat array is no longer correct and `RULE_FILES` must become a per-suite structure — that trigger belongs in the array's doc comment in this PR, and the restructure does not. |
| **EC-009** | `crates/happenstance-sync-testkit`'s dev-dependencies do not build for `wasm32` (tokio's multi-threaded runtime cannot exist there). | Split the dev-dependencies **by target**, exactly as `crates/happenstance-testkit/Cargo.toml:43-54` already does. A manifest edit is in this PR's boundary; a fixture or rule edit is not. |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| **NF-001** | The two new steps are `cargo check`, never `cargo build` or `cargo test`, and their incremental wall-clock cost on `cargo xtask ci --fast` is measured once and recorded in `implementation-report.md`. | `--fast` is the bar a non-terminal project meets (`CLAUDE.md`, *Commands*; `.redkiln/config.yaml:55`) and is run per integration pass. A cost nobody measured is a cost that gets removed under pressure by someone who also cannot measure it. No target number is invented here; the measurement is the deliverable. |
| **NF-002** | The five file-reading checks stay a file read and a string match. No parser dependency is added to `xtask` — CF-32 keeps its three-line hand parse. | `cargo xtask affected` runs all five **unconditionally, before any package selection** (`xtask/src/affected.rs:112-126`), so their cost is paid on every story in the repository. The existing comment states the trade explicitly: the question is one key in one table in one file, and adding a parser to answer it would be the larger claim (`xtask/src/lints.rs:295-300`). |
| **NF-003** | Every path reported by a widened check is normalised to forward slashes before it is printed or matched. | The repository is developed on Windows; `no_clock` already does `.replace('\\', "/")` (`xtask/src/lints.rs:245-251`) and a second scope that forgets it produces messages that differ by host and unit tests that pass on one machine only. |
| **NF-004** | The gate stays defined **once**. Both new steps go in the `REQUIRED` table in `xtask/src/main.rs`; no step is added to CI configuration directly, and the help text's step counts and descriptions are updated in the same change. | `CLAUDE.md`, *Commands*: the gate is defined once in `xtask/src/main.rs` and is exactly what CI runs. A step added anywhere else is a second definition, and the two diverge on the first busy week. |
| **NF-005** | The widened checks land against a **green baseline of zero** sync rules, deliberately and in that order. | The ratchet order: land the stricter gate while the baseline is green so the first violating change is what goes red (`.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md`). The obligation it creates — that a vacuous pass and a real pass share an exit code — is AC-002 and AC-010, discharged here rather than deferred. |

## Implementation notes (non-prescriptive)

- **Make the checks testable by giving each an inner function that takes its scope.** The five
  file-reading checks are zero-argument and resolve `workspace_root()` themselves, which is why
  none of them has a unit test today. The smallest change that makes AC-002's negative controls
  ordinary tests rather than transcripts is an inner `fn …_in(root: &Path, scopes: &[&str])` that
  the existing public function calls with the constant. That is a suggestion with a reason, not a
  requirement: if the implementer finds a shape that discharges AC-002 without it, take it — what
  is not negotiable is that the demonstration exists in the tree rather than in someone's memory.
- **Naming.** `TESTKIT_SRC` → a plural constant, `TESTKIT_MANIFEST` → a plural constant. The
  rename is the cheap part; the part that gets missed is that `{TESTKIT_SRC}` is interpolated into
  four separate messages inside `no_clock` (`xtask/src/lints.rs:236`, `:271`, `:279`) and each one
  now needs the per-scope variable instead. Grep for the constant name, not for the function.
- **Keep `RULE_FILES` flat and write the trigger down.** Add the fourth entry and extend the
  existing doc comment with the condition that would force a per-suite structure (a rule name in
  both suites, because `claimed` is keyed on the bare name). The comment already explains why the
  array lives beside `collect_rules`; this is the same kind of note, one level further on.
- **Place the two new steps beside the existing `wasm32` pair**, so the block reads as one
  argument: contract crate, its harness, sync crate, its harness, then the two adapters. Ordering
  is presentational and the implementer may disagree; what matters is that both are in `REQUIRED`,
  both carry `probe: None`, and both are named in `wasm_steps()`.
- **The powerset step is a separate decision from the two required ones.** Adding `-p
  happenstance-sync` to the `wasm32` feature powerset (`xtask/src/main.rs:558-592`) is worth doing
  because `happenstance-sync` carries a non-default `memory` feature and a feature is not
  target-scoped — the exact shape that caught `happenstance-testkit`'s `proptest` feature. It
  stays in `OPTIONAL`. If `happenstance-sync-testkit` declares no feature, adding it buys nothing
  and should be left out with a one-line reason rather than added for symmetry.
- **Do the specification repair last, and regenerate rather than hand-edit.** Land and prove the
  code, then repair the four `Rule:` fields, then `cargo xtask spec-trace --write`, then re-run
  `Mode::Check`. Doing it first means fighting a red `spec-trace` through the whole story for no
  information.
- **One `CHANGELOG.md` entry for this change, and it is not a per-rule entry.** CF-29's
  per-rule obligation begins with HS-S0105's rules; this entry records the widened scopes and the
  two new steps.
- **The three named wrong implementations to keep in view** while working, from
  `…/gate-mounts-for-the-sync-suite/discover.md`: the decorative suite (change nothing under
  `xtask/` and stay green), the partial `wasm32` fix (crate build only, SY-17 undischarged), and
  the widened *exclusion* (fix a `tests/` hit by excluding rather than by scoping).

## Tests and CI (merge gate)

Grounded in `…/replication-identity-and-ingest/_decomposition.md`, *Testing brief* — this story
sits almost entirely in that brief's **static** tier, which is exactly where it belongs: its
deliverable is the gate itself.

| tier | command / path | proves |
| --- | --- | --- |
| Static — story grain | `cargo xtask affected --base main` | The five file-reading checks run unconditionally before any package selection, so the widened constants are exercised on this diff and on every later story in the project with no further wiring (`xtask/src/affected.rs:112-126`; `.redkiln/config.yaml:28-40`) |
| Static — reachability | `cargo xtask lints && cargo xtask spec-trace` | `.redkiln/config.yaml:48`'s `reachability_static`. AC-001, AC-003, AC-004, AC-005, AC-006 in one command; project DoD 2 |
| Static — targeted | `cargo xtask lint-clock`, `cargo xtask lint-testkit-version`, `cargo xtask lint-changelog`, `cargo xtask lint-position-literals`, `cargo xtask lint-retired-rules` | Each widened check individually, with its per-scope success line legible rather than buried in a full gate run (`xtask/src/main.rs:682-688`) |
| Unit — `xtask`'s own | `cargo test -p xtask` over new `#[cfg(test)] mod tests` in `xtask/src/spec_trace.rs`, `xtask/src/lints.rs`, `xtask/src/main.rs` | AC-002's five negative controls and AC-010's non-vacuity guards; the `wasm_steps()` name-resolution and `probe: None` assertions of AC-009. Module-local style per `xtask/src/affected.rs:597` and `xtask/src/package.rs:409` |
| Static — `wasm32` | `cargo xtask wasm` | Six steps rather than four; AC-007 and AC-008 executed rather than asserted. `standards/rust/52-wasm32-and-target-cfg.md` is the constitution atom this tier answers to |
| Integration — project ceiling | `cargo xtask ci --fast` | `.redkiln/config.yaml:55`'s `integration_scoped` and project DoD 1. Contains fmt, clippy `-D warnings`, the full test run, the now-six `wasm32` steps, docs, `spec-trace`, the `--no-default-features` doc build and the `cargo package --list` assertion (`CLAUDE.md`, *Commands*) |
| Merge review — not a command | `git diff spec/SPECIFICATION.md` | Project DoD 7: the specification diff shows only `Rule:` prose and the generated §7.1/§7.2 regions — no MUST, no maturity marker, no `Rejects:` line. AC-011 |
| Not run by this story | `cargo test -p happenstance-sync-testkit --all-features` | Named for completeness: it is HS-S0103's and HS-S0105's bar. This story adds no rule and runs no suite |

**Merge DoD**, restated from the *PR boundary*: `cargo xtask ci --fast` green with six `wasm32`
steps rather than four, and each widened check demonstrated — in `xtask`'s own tests — to fail
when the sync suite violates it.

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | how this PR holds it |
| --- | --- | --- |
| **HS-S0103 has not landed, or landed without the `!Send` harness.** Every constant in this diff points at a crate that does not exist. | Medium / High | This is a **blocker to raise**, and the spec says so in three places (*Context pack* 10, *PR boundary*, EC-001). The failure mode to refuse is a stand-in harness written here: it would discharge SY-17 against something no story owns |
| **The negative controls become transcripts instead of tests**, because the checks are zero-argument and resolve the workspace root themselves. | Medium / High | AC-002 is written so that a transcript is the *fallback*, not the plan, and the inner-function shape is offered in *Implementation notes*. A transcript-only discharge is the decorative-suite mutant one level up: the demonstration lives in a report nobody re-runs |
| **The flat `RULE_FILES` becomes wrong quietly.** With two suites, "orphaned in which suite?" is unanswerable and a name collision would let one suite's clause discharge the other's rule. | Low now / Medium later | Accepted with the trigger named and recorded in the array's doc comment (EC-008). The cost is real and is paid deliberately; what is forbidden is paying it without writing down what would reverse it |
| **A `Rule:` repair turns out to be a gap.** | Low / High | EC-007. The playbook's mechanical test is applied *before* the edit, its answer is recorded either way, and a gap halts the story rather than shrinking the edit until it passes |
| **`spec-trace --write` regenerates more than the `Rule:` repair implies**, and the extra churn hides a real change in review. | Medium / Low | The regenerated regions are §7.1 and §7.2 only, held equal to a computation; `git diff spec/SPECIFICATION.md` is reviewed as a distinct merge-gate step rather than skimmed as part of the whole diff |
| **`cargo xtask ci --fast` gets measurably slower** and someone later moves a required step to `OPTIONAL` to recover the time. | Medium / Medium | NF-001 measures and records the cost now, so the conversation starts from a number. The `probe: None` placement and its reason are restated in the step's own comment, where the person considering the move will read it |
| **Coupling to `xtask/src/affected.rs` is invisible from this diff.** Widening the constants widens the per-story gate for every later story in the project. | High / Low | Stated in the *Integration contract* and re-stated in the tests table. It is a benefit, not a hazard — but a later story failing on a check it did not introduce should be able to find out here why |

## Dependencies

**Blocks on**

- `sync-testkit-crate-and-rule-registry` (HS-S0103) — supplies `crates/happenstance-sync-testkit/`
  itself: the rules module `RULE_FILES` must name, the `version` key CF-32 checks, the `src/` tree
  CF-33 scopes, and the `cfg(target_arch = "wasm32")` harness the sixth step type-checks. There is
  nothing to point a constant at until it exists (`…/_storymap.md`, *Slices*, `depends_on`).

**Unlocks**

- `headline-rules-and-mutant-registry` (HS-S0105) — slice-mate and successor. Its rules cannot be
  claimed by a clause, cannot be caught asserting a literal position, and owe no changelog entry
  until this story lands. This is the dependency the story map states as *"the suite is
  [not] mounted until the gate can see it"* (`…/_storymap.md`, *Merge order* §3).
- `send-free-sync-runner` (HS-S0106) — depends on this story for *"the `wasm32` gate steps
  actually running it"* (`…/_storymap.md`, *Slices*, `runner-and-topologies` row 1); its half of
  project AC-009 is the native `tokio::spawn` proof, this story's half is the constrained target.
- `clause-arithmetic-and-deferral-renewals` (HS-S0115) — its arithmetic is computed through
  `cargo xtask spec-trace`, which cannot resolve a single `SY` `Rule:` field until `RULE_FILES`
  names the sync suite (`…/_storymap.md`, *Coverage*, AC-014 row).
- `frozen-clause-repairs` (HS-S0114) — not a declared dependency, but this story makes its work
  *possible*: dropping the `(new, happenstance-sync-testkit)` markers only makes sense once a
  resolution can succeed (`xtask/src/spec_trace.rs:1626-1634`). This story drops no marker.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Every path was confirmed to exist before it was
cited. Nothing below is required to *start* — the *Context pack* is — but each is required at the
moment named.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `xtask/src/spec_trace.rs` | `RULE_FILES` (`:85-89`) and its own doc comment explaining why it lives beside `collect_rules`; check 6's ownership sweep (`:765-800`); `retired_rules` and its `SUITE` fallback (`:857-942`); `all_rules` (`:1739-1755`); the `--write` equality (`:735-744`); the `(new)` / `†` scheduled-marker logic (`:1626-1634`) | Before the first line of AC-001; again before AC-006 and AC-011 | AC-001, AC-006, AC-010, AC-011 |
| `xtask/src/lints.rs` | `TESTKIT_SRC` and its comment naming the exclusion-widening failure mode (`:33-42`); `TESTKIT_MANIFEST` (`:44-45`); `no_clock` with its per-scope `bail!` and path normalisation (`:231-281`); `testkit_version`'s three-line hand parse and its stated cost (`:289-345`); `changelog_names_every_rule` (`:504-595`); `no_position_literals` (`:628-651`) | Before AC-003, AC-004 and AC-005 — each constant's comment states the failure mode the edit must not reintroduce | AC-003, AC-004, AC-005, AC-002 |
| `xtask/src/main.rs` | The `REQUIRED` step table and the four `wasm32` steps with the reasons they are separate (`:192-283`); the `wasm32` feature powerset (`:558-592`); `wasm_steps()` and why it selects by name (`:769-791`); the lint subcommand wiring (`:682-688`) | Before AC-007, AC-008 and AC-009; the `wasm_steps()` doc comment is the argument for why two steps and not one | AC-007, AC-008, AC-009 |
| `spec/SPECIFICATION.md` | SY-17's `[FROZEN]` `Rule:` field, which names the exact compile the sixth step performs (`:6341-6360`); CF-6 (`:7233-7249`), CF-29 (`:8141-8168`), CF-32 (`:8200-8228`) and CF-33 (`:8236-8262`), the four clauses whose `Rule:` prose this diff supersedes; §7.2's 79-character rule cell (`:8717`) | Read SY-17 before writing the sixth step; read the four CF clauses before touching a character of them | AC-008, AC-011 |
| `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | The mechanical repair-or-gap test and the three-part repair form. This is the authority that makes editing a `[FROZEN]` clause's prose legitimate without an ADR — and the authority that stops the story if the test fails | Immediately before editing any `Rule:` line, and again when writing the finding into the report | AC-011 |
| `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md` | The ratchet ordering this whole story is an instance of, and the reason a green landing is correct rather than suspicious | Once, before AC-002 — it is what turns "it passed" into "and here is why that is not enough" | AC-002, AC-005 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/gate-mounts-for-the-sync-suite/discover.md` | The signal ledger with every constant quoted at its own line, the three named wrong implementations, and the four questions this spec settled — including the two it deliberately deferred to the implementer | When a decision in the *Context pack* needs its evidence rather than its conclusion | AC-001, AC-002, AC-009 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` | *Architecture: Composition root* §6 **Gate mounts** (`:162-190`) is the authoritative change list and the source of *"AC-014 cannot pass without this edit"*; the *Testing brief*'s static tier (`:625-665`) and *Merge-gate commands* (`:775-790`) are where this story's tests table comes from | Before starting, to confirm the change list has not moved; again when writing the report | AC-001, AC-005, AC-007 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` | AC-004, AC-009 and AC-014 in full, and DoD 1, 2 and 7 — DoD 7 is the check on AC-011's specification diff | When mapping this story's ACs back to the project's, and at merge review | AC-002, AC-007, AC-011 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` | The signed-off, approved **no-surface** determination, which is why this spec's *Interaction quality* carries state invariants and no composition invariants | Once, if the absence of composition ACs is ever questioned | AC-003, AC-006 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The adapter author (`:114`), the edge Rust developer (`:182`) and the evaluator (`:249`) — the three intents the acceptance criteria are framed from, and the only adjudicated audience this initiative has | When an AC's *GIVEN* needs its evidence, or when a criterion is being rewritten | AC-001, AC-008, AC-011 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_storymap.md` | The slice table (`:61-62`), the `sync-conformance-suite` merge order §3, and the coverage rows for project AC-004, AC-009 and AC-014 | Before assuming what a slice-mate owns — the boundaries in *Context pack* 10 come from here | AC-002, AC-007 |
| `crates/happenstance-testkit/Cargo.toml` | `:43-54` is the dev-dependency split by target that exists because tokio's multi-threaded runtime cannot live on `wasm32` — the template for EC-009 | Only if the sixth `wasm32` step fails on a dev-dependency | AC-008 |
| `crates/happenstance-sync/Cargo.toml` | `:37-44` — the non-default `memory` feature, which is why the `wasm32` powerset step is worth widening | Before deciding whether to add the sync crates to the `OPTIONAL` powerset step | AC-009 |
| `xtask/src/affected.rs` | `:112-126` — the file-reading checks the story grain runs unconditionally, which is how this diff silently widens the per-story gate; `:597` is the module-local `mod tests` style the new tests follow | When writing the new `mod tests`, and when explaining the coupling in the report | AC-010, AC-002 |
| `xtask/src/package.rs` | `:409` — the second in-repository example of the module-local `mod tests` shape, and the closer analogue: a check over manifests and paths | Alongside `affected.rs` when writing AC-010's guards | AC-010 |
| `standards/rust/52-wasm32-and-target-cfg.md` | The constitution atom governing `wasm32` and `cfg` work — read the router's trigger table first and pull only this atom | Before adding either `wasm32` step | AC-007, AC-008 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `GappedPositionStore` (`:326-338`) — the conformant variant that enforces CF-6 behaviourally, of which the position-literal lint is only the cheap second line. Relevant because AC-002's control must not be mistaken for CF-6's real enforcement | When writing AC-002's position-literal control, to keep the two roles straight | AC-002 |
| `.redkiln/config.yaml` | `:28-40` and `:40-55` — `affected_gate`, `reachability_static` and `integration_scoped`, the wiring that runs these constants whether or not anyone types the commands | When the tests table needs to be believed rather than read | AC-009 |
| `CHANGELOG.md` | The document CF-29 reads, and where this story's own entry goes | At the end, when writing the entry for the widened scopes and the two new steps | AC-005 |

## Clarifications resolved during spec

1. **The AC set is exactly the eleven the front half enumerated** — AC-001 through AC-011, none
   added and none dropped. The ledger matches.
2. **AC-010's vacuity guard asserts resolution, not count, and explicitly not "at least one sync
   rule".** At the moment this story lands, `happenstance-sync-testkit` holds **zero** rules
   (HS-S0105 owns them), so a guard demanding a non-empty per-file rule set would fail on the day
   it is written. The guard is therefore: every `RULE_FILES` entry exists and is parseable; the
   **aggregate** over `RULE_FILES` is non-empty — which the existing `bail!` at
   `xtask/src/spec_trace.rs:873-882` already provides; every lint scope resolves to a non-empty
   `.rs` set; every manifest path exists. Asserting the array's length is explicitly rejected: a
   count of four passes while pointing at four wrong directories.
3. **Discover deferred two questions to spec and both are settled here.** *Does `RULE_FILES` grow
   or become a per-suite structure?* — it grows, flat, with the trigger for the structure written
   into its doc comment (*Context pack* 2, EC-008). *Do the lint constants become lists or does
   the lint take a crate argument?* — they become **scoped lists**, because AC-A05's bar is that
   the *absence* of the sync crate must fail rather than silently pass, and a constant that
   enumerates its scopes fails closed while an argument defaults open (*Context pack* 3).
4. **"Are two `wasm32` steps enough?"** — yes, and the two are not interchangeable. The crate
   build covers the runner if the runner lands in `happenstance-sync`; the harness check is a
   separate step because a harness behind `cfg(target_arch = "wasm32")` compiles to nothing on a
   native run, which is the reason the *existing* pair is a pair. AC-007 and AC-008 are separate
   criteria precisely so a partial fix cannot tick both.
5. **The negative controls are tests where they can be and transcripts where they cannot, and the
   spec says which is the fallback.** All five checks are zero-argument functions resolving
   `workspace_root()` internally, so none is unit-testable as written. AC-002 therefore names the
   inner-function refactor as the recommended route and permits a recorded transcript only for a
   check that resists it — with the risk of transcript-only discharge written into the risk table
   rather than left implicit.
6. **`Interaction quality` carries state invariants and no composition invariants, and that is a
   determination rather than an omission.** `_design.md` records no user-facing surface and was
   approved on exactly that basis; this story adds no public Rust item at all. Every applicable
   invariant is carried by an AC **row in the table**, per the extraction rule — none is left as a
   prose bullet.
7. **CF-29 needs no code change beyond `RULE_FILES`, and AC-005 still exists.** The lint already
   resolves its rule set from the array. What AC-005 adds is the *success line stating the swept
   count*, without which a later narrowing of the sweep set is invisible — the same
   silent-narrowing hazard AC-010 guards structurally.
8. **`frozen-clause-repairs` is listed under *unlocks* although it does not declare a dependency
   on this story.** The story map's `depends_on` for HS-S0114 names `headline-rules-and-mutant-registry`
   and `adr-0026-peer-ingest-and-transport`, not this story. It is listed as an unlock with that
   qualification stated, because its `(new)`-marker work is only meaningful once resolution can
   succeed — recorded rather than silently asserted as a hard edge.
