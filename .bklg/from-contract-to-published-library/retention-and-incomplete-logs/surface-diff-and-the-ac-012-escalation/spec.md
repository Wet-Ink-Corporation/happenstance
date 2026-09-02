---
item: HS-S0124
stage: spec
created: 2026-08-12T13:48:03.877Z
updated: 2026-08-12T13:48:03.877Z
template_sig: 87bbf1d0
rendered_sig: d1400b61
---

# Spec — The 0.2.0 surface diff, and the escalation that is a finding not a change

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — **AC-11**, *"A consumer's build is not broken by surprise"* (`:338-340`); **AC-14**, the log-with-holes answer (`:347-349`); **DoD 15** (`:402-405`), which this project's answer discharges and which this story is the condition of |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — **gate decision 4** (`:239-249`): retention's answer is constrained to what needs no published-surface change, *"written into that project's architecture brief as a stated constraint, not carried as folklore"*, with a `0.3.0` outside the exit criteria named as the rejected alternative |
| Project | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` — **AC-011** (`:236-239`), **AC-012** (`:240-243`), **DR-11** and **DR-12** (`:181-185`), and the coupling note that `happenstance-testkit` is one of the three publishable crates (`:320-325`) |
| This spec | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/surface-diff-and-the-ac-012-escalation/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` — **architecture brief**: the seam map with a *Surface class under AC-011* column (`:54-66`), the AC-011 nuance about semver-MINOR-and-still-red (`:77-88`), **DA-7**'s four-row option table (`:363-379`), **DA-8**'s `contains_event_id` lie (`:381-397`), **DA-4** (`:276-308`), **DA-5** (`:310-326`), **AC-A11** (`:532-534`); **testing brief**: the AC-011 row (`:613`) and the AC-012 row (`:614`), and the merge-gate command list (`:620-654`) |
| Signed-off design | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` — binding, and it declares **no surface** (`:38-48`): *"N/A — no user-facing surface"*. Its one live sentence for this story is the Public API surface note (`:32-36`): a defaulted `Fixture` item and two `pub fn` rule bodies are *"scoped and reviewed under this project's own AC-A05/AC-A06/AC-011, not here"* — this story is where that review lands. |
| Grounding | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_grounding.md` — §5, what has not landed in this worktree yet (the `0.2.0` baseline and ADRs `0017`–`0028`, `:12`); §2's verified reading of `store.rs` and `append.rs` (`:55-64`) |
| Story map / merge order | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md` — milestone `clause-exit-and-surface-record`, **last** in the project (`:146-149`), and backbone activity **A6** *"Leave the record and the surface true"* (`:58`) |
| Upstream substrate (the instrument this story runs) | `.bklg/from-contract-to-published-library/publication-and-positioning/registry-surface-diff/spec.md` — HS-S0091, which builds `cargo xtask surface-diff` in two modes and its report format (`:241-258`, `:271-273`); `.bklg/from-contract-to-published-library/publication-and-positioning/publish-0-2-0/` — the story that puts `0.2.0` on the registry and thereby creates the baseline |

## One-line PR slice

Run the public-surface comparison against the `0.2.0` **registry** baseline plus the PR-grain
`cargo-semver-checks` job, commit the dated report, and record the result in two sentences — no
breaking change to `happenstance-core`, `happenstance` or `happenstance-testkit` with every additive
item enumerated and shipping as `0.2.x`, **and** the fact the testkit's own manifest states, that
adding a conformance rule is semver-MINOR and can still turn a passing adapter's CI red — then
either raise the AC-012 escalation naming the surface, DA-7's row and the `0.3.0` consequence, or
record in terms that none is owed; and make no such change either way.

## Executive summary

**What lands.** No source change. Three artefacts and one confirmation: a dated
`spec/audits/surface-diff-<YYYY-MM-DD>.md` captured by a real `cargo xtask surface-diff --run`
against the published `0.2.0`, an AC-011 **finding record** in this story's folder that enumerates
every additive item this project added and states the semver-MINOR-but-still-red mechanism in the
crate's own words, and an AC-012 **escalation record** that either raises a port-surface finding in
DA-7's terms or states in terms that none is owed. The confirmation is mechanical: no diff in this
project touches the public signatures in `crates/happenstance-core/src/store.rs` or
`crates/happenstance-core/src/append.rs`.

**The delta against what the tree already does.** Two comparisons already exist and neither answers
AC-011 on its own.

1. `.github/workflows/ci.yml:279-316` runs `cargo-semver-checks` on every pull request with
   `baseline-rev` pointed at the branch point, over all three publishable crates. Its own comment
   says what it does not buy: *"a break merged two pull requests ago is part of the baseline and
   therefore invisible here"* (`:305-307`). It is the review signal, and this story runs it.
2. `cargo xtask surface-diff` — built by HS-S0091 — is the registry comparison, mandatory in check
   mode and mounted in `xtask/src/main.rs`'s `REQUIRED` list (`:105`). It reads the newest committed
   report; **it does not itself re-diff the tree**. So a green gate after this project's diff proves
   the last report is well-formed and current-by-ancestry, not that the surface has not moved since.
   The only thing that closes that gap is a fresh capture on this project's assembled tree, and this
   story is the one that owes it.

Neither exists in this worktree today: `spec/` holds only `SPECIFICATION.md` and `E2E-CASES.md`, and
`xtask/src/surface_diff.rs` is not in the tree — both arrive from `publication-and-positioning`
(HS-P0016), consumed as built and never reconstructed here (`_grounding.md:12`; `_storymap.md:111-115`).

**What this PR is not.** It moves no manifest version, publishes nothing, adds no `pub` item, writes
no conformance rule, and — most importantly — makes no port-surface change even when it concludes
one is needed. AC-012 exists precisely to make AC-011 *a decision rather than a suppression*
(`project.md:242-243`): the honest finding is raised with its cost priced, and the change is not
made inside this PR.

## Context pack

Everything an implementer needs to start. Deeper material sits behind the anchors, never pasted here.

**Two instruments, two questions, and conflating them is this story's named wrong implementation.**
The PR-grain job proves *this diff* did not break what it branched from. The registry diff proves
*this tree* has not broken what consumers can already `cargo add`. AC-011 asks for the second by
name — *"a public-surface comparison against the `0.2.0` registry baseline"* (`project.md:236-239`)
— and the testing brief's AC-011 row asks for **both** (`_decomposition.md:613`). Running only the
PR job, or running `cargo xtask surface-diff` in check mode over the report HS-P0016 captured before
this project's diff existed, would go green and leave AC-011 unmet. Concretely: the capture is
`--run` against `--baseline-version 0.2.0` per crate; a run carrying `--baseline-rev` is the wrong
instrument wearing this story's label (HS-S0091's spec `:70-77` states this as its own central
distinction).

**The expected verdict is additive-only, and it is checked rather than intended.** The architecture
brief already classed every seam this project touches (`_decomposition.md:54-66`): the instrument
lives in `crates/happenstance-testkit/tests/` (**not** public API); the new rule bodies in
`crates/happenstance-testkit/src/suite.rs` are **additive** `pub fn`s re-exported through
`crates/happenstance-testkit/src/lib.rs:188`; their names inside `for_each_event_store_rule!`
(`crates/happenstance-testkit/src/registry.rs:94`) are additive; any `Fixture` seam is a **defaulted**
const plus a **defaulted** method and never a required item (DA-4, `_decomposition.md:276-308`);
ES-40's obligation lands as rustdoc, which is not a semver surface (DA-5, `:310-326`);
`crates/happenstance-sync` is `publish = false`. If the report disagrees with that table, the table
is wrong and the finding is real — the diff is the authority, the brief is the expectation.

**The record is two sentences, not one, and the second is the load-bearing one.** The testkit carries
its own version key for a stated reason, in its own words at `crates/happenstance-testkit/Cargo.toml:4-13`:
*"Adding a conformance rule is a semver-MINOR change that can turn a passing adapter's CI red."* So
the diff correctly reports **additive**, and the *effect* on a downstream adapter is still a red
build. That is the mechanism `CLAUDE.md`'s rule that matters exists to produce, not a violation of
AC-011 — and the architecture brief is explicit that the recorded finding *"should say both
sentences, not just the first"* (`_decomposition.md:77-88`). A record that stops at "no breaking
change" is technically true and materially misleading to the adapter author it is written for.

**Every additive item is enumerated by name, not summarised.** The list is whatever this project's
merged diff actually added: each new `pub fn` rule body (`positions_are_not_reused_after_removal`,
`condition_over_removed_history_does_not_reject`, and `suffix_store_is_distinguishable_from_a_young_store`
**only if** ADR-0028 decided rather than refused — the two branches of `cf-27-rule-or-recorded-refusal`),
plus any defaulted `Fixture` associated const and defaulted method DA-4 permitted. Enumeration is
what makes AC-011 auditable at closeout (`closeout-and-durable-audience`'s BR-15 audit) rather than
a verdict nobody can re-derive. An item present in the diff and absent from the list is the defect
this AC exists to catch.

**DA-7 pre-computed the escalation so that raising it is a decision, not a discovery.** Four options,
each a port-surface change, each a minor bump under 0.x — i.e. a `0.3.0` this initiative's exit
criteria do not contemplate (`_decomposition.md:363-379`; initiative `_decomposition.md:240-249`):
`earliest_position()` (a floor — ES-39 argues against it by name, because a regulated purge is
scattered and not a prefix); a set of retained ranges (honest for scattered purge, new method **and**
new public type); a third outcome on condition evaluation (a new variant reaching every caller's
`match`, `crates/happenstance-core/src/error.rs:214-248`); and a tri-state `contains_event_id`
(`crates/happenstance-core/src/store.rs:268` — the smallest signature delta, the same semver class,
and a candidate ES-39 does not list). **The escalation names which row it is.** An escalation that
recommends a change without naming its row is the testing brief's named wrong implementation for
AC-012 (`_decomposition.md:614`).

**The escalation's raw material arrives from the two reader stories, and none of it may be dropped.**
AC-A08 requires that any reader which *"cannot be made to fail loudly without a surface the port does
not have"* is written up against AC-011 with the missing surface named (`_decomposition.md:526`;
`project.md:216-218`). DA-8 supplies the likeliest one already observed rather than predicted:
`EventStore::contains_event_id` answers `false` for an event this store minted and acknowledged, and
`IngestStore::holds` (`crates/happenstance-sync/src/ingest.rs:164`) inherits the same lie one crate
up, so a peer that re-sends on `false` re-sends forever. Demonstrating that costs nothing; *fixing*
it is DA-7's fourth row. This story's job is to take each such finding and dispatch it — to a DA-7
row in the escalation, or to the record as reachable inside AC-011 — never to leave it in a sibling
story's ledger and call the surface question closed.

**The refusal branch is an answer that must be written, not an absence.** If ADR-0028 refused, or
answered entirely inside rustdoc and testkit-additive space, then no surface change is owed and
AC-012 is met by **saying so in terms**, naming the ADR branch that made it so and the reader
findings that were dispatched. An empty escalation section reads identically to an escalation nobody
wrote. This mirrors DR-9's discipline one level up: the refusal branch still owes its artefact
(`project.md:174-176`).

**And in either branch, no such change is made here.** The mechanical check is stated by the testing
brief: confirm no diff touches the public signatures in `crates/happenstance-core/src/store.rs` or
`crates/happenstance-core/src/append.rs` (`_decomposition.md:614`). ES-37 is `[FROZEN]`
(`spec/SPECIFICATION.md:4274-4297`) and amending it is a new decision atom plus a re-plan
(`project.md:112-114`), so a helpful one-line signature fix inside this PR would be the single most
expensive thing an implementer could do while feeling productive.

**Fail closed, and never downgrade the instrument to something that passes.** ADR-0010's discipline
is that a skip is reported, never silent (`.kb/decisions/0010-the-suite-must-prove-itself.md`). If
`cargo-semver-checks` is absent, the registry is unreachable, or `0.2.0` is not on the registry
because `publish-0-2-0` has not shipped, this story **halts and reports** — it does not substitute
the PR-grain job, does not hand-write a report, and does not record "additive" from the seam-map
table. A surface record with no capture behind it is exactly the thing IQ-5 forbids: *"run our CI"*
is not evidence, a committed dated artefact is.

**The persona slice.** Backbone activity **A6**, *"Leave the record and the surface true"*
(`_storymap.md:58`). Two readers: the consumer of initiative AC-11 whose build must not break by
surprise (`initiative.md:338-340`), and the adapter author whose green CI can still go red the day
they bump `happenstance-testkit` — the second is the one the two-sentence record exists for, and the
one a single-sentence record silently fails.

**What this story must not settle in passing.** Whether a finding *should* be acted on is not this
story's call. Deciding to change a port is a new ADR and a re-plan; deciding to accept a `0.3.0` is
the initiative's, at its own gate; and re-deciding ADR-0028's branch is `adr-0028-and-the-open-question-wave`'s
and is closed by the time this story runs. This story makes the finding unmissable and priced, and
stops (`_decomposition.md:378-379`).

## Integration contract

| | |
| --- | --- |
| **Archetype** | `capability` — a maintainer runs the comparison and gets an answer a consumer's build depends on, and an initiative-level decision hangs off the other side of it. |
| **Slice / milestone** | `clause-exit-and-surface-record`. Slice-mates, implemented in one context and mounted as one integrated record: `cf-27-rule-or-recorded-refusal` (whose branch decides whether a third `pub fn` rule exists to enumerate) and `marker-moves-and-spec-trace-green` (this story's `depends_on`). Order inside the slice is fixed: the surface diff is an **exit computation over the assembled tree** and runs last (`_storymap.md:146-149`). |
| **Mount point** | **`spec/audits/surface-diff-<YYYY-MM-DD>.md`** — the artefact directory the mandatory `surface-diff` check step reads, itself mounted in **`xtask/src/main.rs`**'s `REQUIRED` list (`:105`) and in `xtask/src/affected.rs`'s unconditional block by HS-S0091. Committing this story's report is what puts its finding inside the gate: `cargo xtask ci --fast` and `cargo xtask affected --base main` both read it on every subsequent story. The report is the mount; **this story adds no wiring edit**, because the step is already mounted — and if it is not, that is the halt condition in the PR boundary, not a licence to mount it here. |
| **Wires into** | `cargo xtask surface-diff --run` (HS-S0091's capture mode) and its check mode; `xtask/src/package.rs:86` `PUBLISHABLE` — the decided crate set the report must match, read-only; `.github/workflows/ci.yml:279-316` — the PR-grain `semver` job, run and recorded, **not edited**; `crates/happenstance-testkit/Cargo.toml:4-13` — the independent version key and the sentence the record must quote; `crates/happenstance-core/src/store.rs` and `.../append.rs` — read-only, as the subject of the no-signature-change confirmation; the two reader stories' recorded findings (`decision-model-and-ingest-observed`, `projection-runner-across-the-hole`) as the escalation's input. |
| **Renders surfaces** | **None.** `_design.md` declares no surface for this project (`:38-48`), and this story renders nothing a user meets. Its one binding line from that file is the Public API surface note (`:32-36`), which routes the defaulted `Fixture` item and the two `pub fn` rule bodies to *"this project's own AC-A05/AC-A06/AC-011"* — the AC-011 half is this story's, and this is where that deferred review is discharged. |
| **Public items** | **None added.** `_design.md`'s `## Items` block is `N/A` (`:46-48`), and this story adds no `pub` item to any crate. It *enumerates* the items its slice-mates and the `owed-rules-and-mutants` slice added — enumeration is the deliverable, addition is not. |
| **Conformance rule(s)** | **None, and it is not adapter-observable, deliberately.** No rule in `crates/happenstance-testkit/src/suite.rs` can observe whether a published surface moved: the subject is a registry artefact and a git diff, not a store. The equivalent falsifiability obligation is discharged by the two instruments themselves — a capture that reports a break fails, and the seeded-break rejection HS-S0091 already proved for the tool is not re-proved here. |
| **Clause(s)** | **Amends none; reads CF-32** (`spec/SPECIFICATION.md:8200-8220`), which is `[FROZEN]` and is why the diff runs per crate against per-crate baselines rather than one workspace verdict — `happenstance-testkit` moves on its own key. Also reads ES-37 (`:4274-4297`), `[FROZEN]`, as the reason a signature change here would take a new ADR and a re-plan rather than an edit. The marker moves themselves are `marker-moves-and-spec-trace-green`'s and are complete before this story starts. |
| **Advances DoD scenario** | Initiative **DoD 15** — *"Incomplete logs have an answer on disk"* (`:402-405`): this story is the condition under which that answer was allowed to land at all, because gate decision 4 constrained it to what needs no published-surface change. It also re-establishes **DoD 11**'s claim (*"The release is diffed, not asserted"*, `:390-392`) on the assembled tree that `closeout-and-durable-audience` re-observes at DoD 13, since HS-P0016's report predates this project's diff. |

## PR boundary

The paths this story may touch. `redkiln verify --grain story` reads the first fenced block under
this heading and fails on any file changed outside it.

```
spec/audits/surface-diff-*.md
.bklg/from-contract-to-published-library/retention-and-incomplete-logs/surface-diff-and-the-ac-012-escalation/**
```

**In this PR**

- `spec/audits/surface-diff-<YYYY-MM-DD>.md` — one dated report, produced by a real
  `cargo xtask surface-diff --run` against the published `0.2.0`, committed verbatim as the tool
  wrote it. Never hand-edited: `captured-at` is the only staleness anchor check mode has, and editing
  the file destroys its meaning (HS-S0091 spec `:275-278`).
- This story's own folder — the AC-011 finding record, the AC-012 escalation record (in whichever
  branch it takes), the captured PR-grain job output, and the ledger the second pass authors.

**Explicitly not in this PR**

- **Any source file.** No `crates/**`, no `xtask/**`. This story adds no rule, no fixture item, no
  rustdoc sentence; its slice-mates and the `owed-rules-and-mutants` slice landed all of those, and
  re-touching one here would make the diff this story reports on a moving target.
- **Any manifest version.** `Cargo.toml`'s `[workspace.package] version` and
  `crates/happenstance-testkit/Cargo.toml:13` are untouched. The report *implies* a number; moving a
  manifest to it is a release act and this initiative's train is `publication-and-positioning`'s.
- **`.github/workflows/ci.yml`'s `semver` job.** It runs; it is not edited, repointed or deleted.
  The two comparisons answer different questions and neither substitutes for the other
  (`CONTRIBUTING.md:291-301`).
- **Any port-surface change, in either direction — including one this story concludes is needed.**
  That is the whole of AC-012 (`project.md:240-243`, DR-12 `:184-185`).
- **Mounting `cargo xtask surface-diff`.** If the step is absent from `xtask/src/main.rs`'s `REQUIRED`
  list when this story starts, HS-P0016 has not delivered and that is a dependency failure to halt on
  and report — never a licence to build a parallel instrument (`_storymap.md:111-115`).
- **`spec/SPECIFICATION.md`.** Every marker move is `marker-moves-and-spec-trace-green`'s and is done
  before this story opens.

**Merge DoD.** A dated report captured by a real `--run` against `0.2.0` is committed; check mode is
green against it inside `cargo xtask ci --fast`; the PR-grain `semver` job is green and its output
captured; the finding record carries both sentences and the enumerated additive list; the AC-012
record takes one branch explicitly; and `git diff` over the project's merged range shows no change to
the public signatures in `crates/happenstance-core/src/store.rs` or `crates/happenstance-core/src/append.rs`.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **B1 — the registry capture, per crate, against `0.2.0`** | `cargo xtask surface-diff --run --date <YYYY-MM-DD>` on the assembled project tree: one invocation per crate in `PUBLISHABLE`, each carrying `--baseline-version <that crate's published version>` and **never** `--baseline-rev`. Three crates, three baselines, because `happenstance-testkit` moves on its own key (CF-32, `[FROZEN]`). The date is an argument, not a clock read. | `xtask/src/package.rs:86`; `spec/SPECIFICATION.md:8200-8220`; HS-S0091 spec `:245-247` |
| **B2 — the report is committed and enters the gate** | The captured `spec/audits/surface-diff-<date>.md` is committed as written. Check mode — already mandatory in `xtask/src/main.rs`'s `REQUIRED` list — then reads it on every `cargo xtask ci --fast` and `cargo xtask affected`, asserting it is the newest, well-formed, crate-set-matching, `captured-at`-ancestral report. A finding that only exists in this story's prose is not in the gate. | `xtask/src/main.rs:105`; `.redkiln/config.yaml:40`, `:55` |
| **B3 — the PR-grain comparison runs too, and is recorded as the different question it answers** | `cargo-semver-checks` at the branch point (`.github/workflows/ci.yml:279-316`, `baseline-rev` = `base.sha`) is run and its result captured. The record states both comparisons and what each proves — the branch-point job cannot see a break merged two pull requests ago, and the registry job cannot see a break introduced after `captured-at`. Presenting either as the other is this story's named wrong implementation. | `CONTRIBUTING.md:291-301`; `.github/workflows/ci.yml:298-308`; `_decomposition.md:613` |
| **B4 — the verdict is per crate and read from the tool, never from the seam map** | Each crate's verdict is its own invocation's outcome. The architecture brief's *Surface class under AC-011* column (`_decomposition.md:54-66`) is the **expectation**; the report is the **authority**. Where they disagree the finding is real, and it routes to B8, not to a re-reading of the table. | `_decomposition.md:54-66`, `:77-88` |
| **B5 — the finding record is two sentences** | (1) No breaking change to `happenstance-core`, `happenstance` or `happenstance-testkit` against the `0.2.0` registry baseline. (2) Adding a conformance rule is semver-MINOR **and can still turn a passing adapter's CI red** — quoted from the crate's own manifest, which is why that crate carries its own version key. Both sentences, in the record, in that order. | `crates/happenstance-testkit/Cargo.toml:4-13`; `_decomposition.md:77-88`; `CLAUDE.md` *the rule that matters* |
| **B6 — every additive item is enumerated by name and class** | One row per item added by this project's merged diff: each new `pub fn` rule body re-exported through `crates/happenstance-testkit/src/lib.rs:188`; each `for_each_event_store_rule!` registration (`crates/happenstance-testkit/src/registry.rs:94`); any **defaulted** `Fixture` associated const and defaulted method (DA-4), with the explicit note that a **required** item there would have been breaking; and — only if `cf-27-rule-or-recorded-refusal` took the decide branch — `suffix_store_is_distinguishable_from_a_young_store`. Anything in `crates/happenstance-testkit/tests/` or `crates/happenstance-sync/` is recorded as **not public API**, with the reason (`tests/` is not a surface; `publish = false`). | `_decomposition.md:54-66`, `:276-308`; `crates/happenstance-testkit/src/contract.rs:207-211`, `:297-307`; `crates/happenstance-sync/Cargo.toml:12` |
| **B7 — the implied version stays `0.2.x`, and no manifest moves** | The report's per-crate `release-version` is whatever check mode recomputes from the baseline and the findings; with an additive-only verdict that is a `0.2.x`, and **no `0.3.0` is implied or required** (`project.md:236-239`). This story does not move `Cargo.toml`'s workspace version or the testkit's key. A disagreement between the declared and recomputed number is fixed by re-capturing, never by editing the report. | `project.md:236-239`; HS-S0091 spec `:115-117`, `:250` |
| **B8 — the escalation branch: name the surface, name the row, price it, stop** | If the honest answer needs a port surface, the escalation states: the surface (method, type or variant, by path), **which of DA-7's four rows** it is, the version consequence (a minor bump under 0.x — a `0.3.0` this initiative's exit criteria do not contemplate), and the full four-row option table so the reader sees what lost. Then it stops. No signature is written, no variant added, no method declared. | `_decomposition.md:363-379`, `:532-534`; initiative `_decomposition.md:240-249`; `crates/happenstance-core/src/error.rs:214-248`; `crates/happenstance-core/src/store.rs:268` |
| **B9 — the no-escalation branch is stated in terms, and every reader finding is dispatched** | If no surface change is owed, the record says so explicitly, names the ADR-0028 branch that made it so, and lists each *"missing surface"* finding the two reader stories recorded — dispatching each either to a DA-7 row in B8 or to the record as reachable inside AC-011. A finding left in a sibling story's ledger and unmentioned here is a surface question closed by silence. | `_decomposition.md:526`, `:430-434`; `project.md:216-218`, `:174-176`; DA-8 `_decomposition.md:381-397` |
| **B10 — no such change was made, confirmed mechanically** | Over the project's merged range, `git diff` shows no change to the public signatures in `crates/happenstance-core/src/store.rs` or `crates/happenstance-core/src/append.rs`. Rustdoc-only changes there (ES-40's obligation, DA-5) are expected and are recorded as **not** a semver surface, with that distinction stated rather than assumed. | `_decomposition.md:614`, `:310-326`; `spec/SPECIFICATION.md:4274-4297` (ES-37, `[FROZEN]`) |
| **B11 — fail closed, name the reason, halt** | Tool absent, registry unreachable, `0.2.0` not on the registry, `surface-diff` not mounted in `REQUIRED`, or `spec/audits/` absent: each halts this story with the path, the value read and the owner named. **No fallback** — the PR-grain job is not relabelled, no report is hand-written, and the seam map is not recorded as a verdict. | `.kb/decisions/0010-the-suite-must-prove-itself.md`; `_storymap.md:111-115`; HS-S0091 spec `:255` |
| **B12 — gate posture** | Story grain: `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) — this story's diff maps to no package, so the unconditional file-reading block plus check mode is what gates it, which is exactly the case that block exists for. Integration grain: `cargo xtask ci --fast` (`:55`), this project's ceiling; the whole `cargo xtask ci` is `closeout-and-durable-audience`'s. The `--run` capture itself is a deliberate, networked act and is never invoked by the gate. | `.redkiln/config.yaml:40`, `:55`, `:60`; `_decomposition.md:637-654` |

## Data and migrations

**No database, no schema, no runtime data, and no source change at all.** This library defines no
storage schema of its own, this story adds no code, and no manifest version moves — so there is
nothing to migrate and nothing to back-fill. What it does add is **committed text whose format a gate
step reads**, which is specified here rather than discovered during implementation.

| Artefact | Path | Lifecycle |
| --- | --- | --- |
| Registry surface-diff report | `spec/audits/surface-diff-<YYYY-MM-DD>.md` | Written by `cargo xtask surface-diff --run`, committed verbatim, **never hand-edited**. Its header fields are machine-read by check mode on every subsequent gate run; its transcripts are verbatim tool output. It is a snapshot, not a mirror: it records what was true at `captured-at`, and the way to correct it is a new capture at a new date, which leaves the old report in the tree as the record of what was true then. |
| AC-011 finding record | This story's folder | The two-sentence verdict plus the enumerated additive list, written from the report rather than from the seam map. Durable evidence for `closeout-and-durable-audience`'s BR-15 audit, which re-reads it as part of the assembled set. |
| AC-012 escalation record | This story's folder | One branch, explicitly taken. Under the escalation branch it carries the surface, DA-7's row, the version consequence and the four-row table; under the no-escalation branch it carries the ADR-0028 branch that made it so and the dispatch of every reader finding. It is raised to the initiative through the project's own review roll-up — the initiative charter is not edited here, and its system frontmatter is the CLI's to write. |

**The one lifecycle event that looks like a migration** is superseding a report, and it is
deliberately a capture rather than an edit — the same call HS-S0091 made for the same reason
(`registry-surface-diff/spec.md:275-278`).

**Rollback.** Reverting this story removes two records and one audit file and leaves the tree
compiling exactly as before; nothing depends on the report at build time. What a revert does *not*
restore is the knowledge — an unrecorded surface finding is indistinguishable from no finding, which
is why the artefact is the deliverable and the green gate is not.

## Acceptance criteria

Each criterion is written from the intent of a named reader, carried from the initiative's own
distillation (`_discovery/distillation/personas-and-journeys.md`) because no `authority_tier: product`
persona atom exists in this tree yet (`initiative.md:227-234`). Three readers appear: the
**application author / consumer** whose build must not break by surprise (initiative AC-11,
`:338-340`), the **adapter author** on the *"Learn when you are finished"* journey (`initiative.md:210-214`,
`:245-246`), and the **evaluator** deciding in one sitting from public evidence (`:222-225`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** a consumer who already has `happenstance-core = "0.2"`, `happenstance = "0.2"` or `happenstance-testkit = "0.2"` in a manifest and whose build must not break by surprise (initiative `:338-340`), **WHEN** this project's assembled tree is compared against what that consumer can already `cargo add`, **THEN** a dated report exists at `spec/audits/surface-diff-<YYYY-MM-DD>.md`, produced by a real `cargo xtask surface-diff --run --date <YYYY-MM-DD>` carrying `--baseline-version` per crate and **never** `--baseline-rev`, covering exactly the three crates in `xtask/src/package.rs:86`'s `PUBLISHABLE` — three crates, three independent baselines, because `happenstance-testkit` moves on its own version key (CF-32, `[FROZEN]`) — and committed **verbatim as the tool wrote it**, never hand-edited, with a superseding capture at a new date being the only correction path so the earlier report survives as the record of what was true then. | `cargo xtask surface-diff` in check mode over the committed file (mandatory step, `xtask/src/main.rs:105`), which asserts well-formedness, crate-set match against `PUBLISHABLE` and `captured-at` ancestry; plus review that the file contains three per-crate `--run` transcripts and no `--baseline-rev` invocation (HS-S0091 spec `:241-258`, `:271-278`) |
| AC-002 | **GIVEN** the maintainer who must not have to remember a finding, **WHEN** any subsequent story in this project or the next runs its own gate, **THEN** the finding is read **in place, inside the gate they already run** — `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) and `cargo xtask affected --base main` (`:40`) both consume this report through the already-mounted `surface-diff` check step and go red if it is stale, malformed or crate-set-mismatched — and this story adds **no wiring edit** to do it, because the step is mounted by HS-S0091 and an unmounted step is AC-008's halt, not a licence to mount one here. | `cargo xtask ci --fast` green on the assembled tree; `cargo xtask affected --base main` green (this story's diff maps to no package, so the unconditional file-reading block is exactly what gates it); `git diff` shows no change to `xtask/src/main.rs` or `xtask/src/affected.rs` |
| AC-003 | **GIVEN** the reviewer of this pull request, who is entitled to know which question each green check answered, **WHEN** the record is read, **THEN** both comparisons are present and distinguished by what each is blind to: the branch-point `cargo-semver-checks` job (`.github/workflows/ci.yml:279-316`, `baseline-rev` = `base.sha`) cannot see *"a break merged two pull requests ago"* (`:305-307`), and the registry capture cannot see anything introduced after its `captured-at` — and neither is presented as, relabelled as, or substituted for the other. | The `semver` job green on this PR, its output captured into this story's folder; content review that the record names both instruments and both blind spots (`CONTRIBUTING.md:291-301`; `_decomposition.md:613`) |
| AC-004 | **GIVEN** the adapter author whose conformance suite is green today and who will bump `happenstance-testkit` next week, **WHEN** they read this project's surface record, **THEN** they read **two** sentences in order — (1) no breaking change to `happenstance-core`, `happenstance` or `happenstance-testkit` against the `0.2.0` registry baseline; (2) adding a conformance rule is semver-MINOR **and can still turn a passing adapter's CI red**, quoted from the crate's own manifest, which is why that crate carries an independent version key — so the record is materially honest rather than only technically true, and a record that stops at sentence (1) fails this AC even though it is not false. | Content review against the verbatim sentence at `crates/happenstance-testkit/Cargo.toml:4-13`, and against the architecture brief's explicit *"should say both sentences, not just the first"* (`_decomposition.md:77-88`) |
| AC-005 | **GIVEN** the evaluator and the closeout auditor, who must be able to **re-derive** the verdict rather than take it on trust, **WHEN** they read the record, **THEN** every additive item this project's merged diff added is enumerated **by name and by class** — each new `pub fn` rule body re-exported through `crates/happenstance-testkit/src/lib.rs:188`, each `for_each_event_store_rule!` registration (`crates/happenstance-testkit/src/registry.rs:94`), any **defaulted** `Fixture` associated const and defaulted method (DA-4) with the note that a *required* item there would have been breaking, and each item under `crates/happenstance-testkit/tests/` or `crates/happenstance-sync/` recorded as **not public API** with its reason — the verdict for each crate read from the tool's own output and **not** from the architecture brief's *Surface class* column, which is the expectation and not the authority; the implied release is `0.2.x` with **no `0.3.0` implied or required**, and no manifest version key moves in this PR. | Cross-check: for every `pub` item in `git diff <project-base>..HEAD -- crates/`, assert a matching row in the enumeration (an item in the diff and absent from the list is the defect this AC exists to catch); `git diff <project-base>..HEAD -- Cargo.toml crates/happenstance-testkit/Cargo.toml` shows no version-key change; the report's per-crate `release-version` is `0.2.x` and matches what check mode recomputes (`project.md:236-239`; `_decomposition.md:54-66`) |
| AC-006 | **GIVEN** an honest answer that may need a port surface the contract does not have, **WHEN** the record is read at the initiative's gate, **THEN** exactly one branch is taken **explicitly**: the escalation branch names the surface by path (method, type or variant), states **which of DA-7's four rows** it is, prices it as a minor bump under 0.x — a `0.3.0` this initiative's exit criteria do not contemplate — and reproduces the full four-row table so the reader sees what lost; **or** the no-escalation branch states in terms that none is owed, names the ADR-0028 branch that made it so, and **dispatches every** *"missing surface"* finding the two reader stories recorded (`decision-model-and-ingest-observed`, `projection-runner-across-the-hole`) either to a DA-7 row or to the record as reachable inside AC-011. Nothing is left behind in a sibling story's ledger, and an empty section is not a branch. | Content review that exactly one branch is present and complete; a mechanical cross-read of both reader stories' `_ledger.md` and finding records for any AC-A08 *"missing surface"* item, asserting each appears here with a dispatch (`_decomposition.md:363-379`, `:526`, `:532-534`; `project.md:216-218`) |
| AC-007 | **GIVEN** the consumer relying on the `[FROZEN]` ES-37 (`spec/SPECIFICATION.md:4274-4297`), **WHEN** this project's merged range is diffed, **THEN** no public signature in `crates/happenstance-core/src/store.rs` or `crates/happenstance-core/src/append.rs` has changed — **including in the branch where this story concludes a change is needed**, because AC-012 exists to make AC-011 a decision rather than a suppression — and any rustdoc-only change there (ES-40's obligation, DA-5) is recorded as **not** a semver surface with that distinction stated rather than assumed. | `git diff <project-base>..HEAD -- crates/happenstance-core/src/store.rs crates/happenstance-core/src/append.rs` reviewed for signature changes (expected: none, or rustdoc only); corroborated independently by AC-001's report, in which such a change would surface as a breaking finding (`_decomposition.md:614`, `:310-326`; `project.md:240-243`) |
| AC-008 | **GIVEN** the reader of a green gate, who must never be able to mistake an unrun instrument for a clean surface, **WHEN** any precondition fails — `cargo-semver-checks` absent, the registry unreachable, `0.2.0` not published because `publish-0-2-0` has not shipped, `surface-diff` absent from `xtask/src/main.rs`'s `REQUIRED` list, or `spec/audits/` absent — **THEN** this story **halts and reports**, naming the path, the value actually read and the owner, and records **no** verdict: it does not relabel the PR-grain job as the registry comparison, does not hand-write a report, and does not copy the seam map's expectation into the record as a finding. | Content review of the halt record when a halt occurs; structurally, no committed report may exist without a `--run` transcript behind it, and check mode's own staleness assertion is what makes a fabricated report fail (`.kb/decisions/0010-the-suite-must-prove-itself.md`; `_storymap.md:111-115`) |

**Traceability.** Project **AC-011** (`project.md:235-239`) is carried by AC-001, AC-002, AC-003,
AC-004, AC-005 and AC-008. Project **AC-012** (`project.md:240-243`) is carried by AC-006 and AC-007,
with AC-008 protecting both. Every behavior B1–B12 has a home: B1→AC-001, B2→AC-002, B3→AC-003,
B4→AC-005, B5→AC-004, B6→AC-005, B7→AC-005, B8→AC-006, B9→AC-006, B10→AC-007, B11→AC-008,
B12→AC-002.

## Interaction quality

**This story renders no user-facing surface.** The signed-off `_design.md` declares
`N/A — no user-facing surface` for **Surfaces**, **Items**, **Signatures**, **Shape decision**,
**Placement**, **Visibility**, **What a user meets first**, **The states the API must express** and
**Anti-patterns** (`:38-88`), and the repository declares no `design.capture`, so the perceptual
review is a **declared skip**, not a silent pass. The no-surface determination is itself what the
owner approved on 2026-08-12 (`_design.md:88-93`).

That does **not** empty this section. This story's artefacts are read by humans under a gate, and the
same invariants apply in the medium this project actually ships: committed text. Each is carried by
an AC row above — none is a prose-only bullet, because `redkiln verify` extracts ACs from table cells
and a bullet here would be ungated and untested.

**State family**

| Invariant | Read in this medium as | Carried by | Verified by |
| --- | --- | --- | --- |
| In place, not a context jump | The finding is reachable from the gate the maintainer already runs, not from a document they must remember to open | **AC-002** | `cargo xtask ci --fast` and `cargo xtask affected --base main` both read the committed report |
| Non-occlusion | A new capture never overwrites the old record: the report is committed verbatim and superseded by a new dated file, so the earlier truth stays legible | **AC-001** | check mode's `captured-at` assertion; `git log` over `spec/audits/` |
| Preserved selection | Every *"missing surface"* finding the two reader stories recorded survives into this record with a dispatch — nothing is dropped in the hand-off between sibling stories | **AC-006** | cross-read of both reader stories' `_ledger.md` and finding records |
| Reversibility | Reverting this story restores a compiling tree exactly (no source, no manifest, no wiring change) — the only thing not restored is the knowledge, which is why the artefact and not the green gate is the deliverable | **AC-002**, **AC-007** | `git revert` leaves `cargo xtask ci --fast` green minus the report; `git diff` empty over `crates/**` and `xtask/**` |
| Reachability without special tooling | Every artefact is plain committed markdown in the repository, readable by an evaluator with only a browser on the git host — no dashboard, no CI log retention, no external service | **AC-001**, **AC-004** | the artefacts' own paths under `spec/audits/` and this story's folder |

**Composition family**

`_design.md` names no surface composition to honour, so the composition obligations land on the one
thing this story does compose — the record — and the design's live sentence for this story is its
**Public API surface note** (`:32-36`), which routes the defaulted `Fixture` item and the two `pub fn`
rule bodies to *"this project's own AC-A05/AC-A06/AC-011"*. AC-005 is where that deferred AC-011
review is discharged.

| Invariant | Read in this medium as | Carried by | Verified by |
| --- | --- | --- | --- |
| Presentation exists at all | The record is composed, not a bare verdict string: a two-sentence finding **plus** an enumerated item-by-item table with a class column, so "additive" is re-derivable rather than asserted | **AC-004**, **AC-005** | content review; an item in the diff with no row fails AC-005 |
| Density budget, with its real numbers | Three crates and three independent baselines (AC-001); exactly **two** sentences in the finding, in order (AC-004); **four** DA-7 rows reproduced in full under the escalation branch (AC-006); one dated report, one branch, no more | **AC-001**, **AC-004**, **AC-006** | check mode's crate-set match; content review against `_decomposition.md:77-88`, `:363-379` |
| Hierarchy | The verdict leads, the mechanism that survives it follows, the enumeration supports it, and the escalation is a separate record with its own branch — never a caveat buried under a green headline | **AC-004**, **AC-006** | content review of ordering |
| Named anti-patterns | This project's design records none for surfaces (`_design.md:78-80`), so the binding anti-patterns are the ones the briefs name: a one-sentence record (AC-004); an escalation recommending a change without naming its DA-7 row (AC-006); the PR-grain job presented as the registry comparison (AC-003); a verdict copied from the seam map instead of read from the tool (AC-005); a change made instead of escalated (AC-007) | **AC-003** – **AC-007** | each named wrong implementation is the failing case of its AC's verification |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `cargo-semver-checks` is not installed, or `cargo xtask surface-diff` is absent from the tree | Halt and report: the command attempted, the failure text, and HS-P0016 as the owner. Do **not** build a parallel instrument, and do not record a verdict (`_storymap.md:111-115`) |
| **EC-002** | `0.2.0` is not on the registry — `publish-0-2-0` has not shipped, or shipped a different number | Halt and report the version actually found. The baseline is the *registry*, and a branch-point comparison is not a stand-in for it (AC-003's distinction is the reason) |
| **EC-003** | The registry is unreachable (network, rate limit, auth) | Halt and report; retry is fine, fabrication is not. `--run` is a deliberate networked act and is never invoked by the gate, so a failure here blocks the story rather than the gate (B12) |
| **EC-004** | `surface-diff` is absent from `xtask/src/main.rs`'s `REQUIRED` list, or `spec/audits/` does not exist | Halt and report as a dependency failure against HS-P0016. Mounting the step here is explicitly outside the PR boundary |
| **EC-005** | The report disagrees with the architecture brief's *Surface class* column — the tool reports a **breaking** change where the brief expected additive | The tool wins. Record the finding as real, route it to AC-006's escalation branch, and do **not** fix the signature in this PR. This is the case AC-007 is most likely to be tested by |
| **EC-006** | A `pub` item exists in the project's merged diff that the enumeration does not list | AC-005 fails. Re-enumerate from the diff; never trim the diff to match the list |
| **EC-007** | A reader story recorded a *"missing surface"* finding that no branch of the record dispatches | AC-006 fails. A surface question closed by silence is the exact failure AC-012 exists to prevent (`project.md:240-243`) |
| **EC-008** | The declared `release-version` in the report disagrees with what check mode recomputes | Re-capture at a new date. **Never** hand-edit the report — editing destroys the only staleness anchor check mode has (HS-S0091 spec `:275-278`) |
| **EC-009** | `cf-27-rule-or-recorded-refusal` took the refuse branch, so `suffix_store_is_distinguishable_from_a_young_store` does not exist | Not an error. Enumerate the two rule bodies that do exist and state the refusal branch explicitly; the refusal branch still owes its artefact (`project.md:174-176`) |

## Non-functional

| id | requirement | why |
| --- | --- | --- |
| **NF-001** | The capture is **reproducible from the report alone**: every invocation, its flags and its verbatim output are in the file, so a third party can re-run it without reading this spec | The evaluator decides from public evidence in one sitting (`initiative.md:222-225`); a verdict whose derivation lives only in a planning artefact is not evidence |
| **NF-002** | The report is **committed text in the repository**, not a CI log — no artefact retention window, no external service, no dashboard | Initiative DoD 11, *"The release is diffed, not asserted"* (`:390-392`), and the same durability bar `closeout-and-durable-audience` re-reads at DoD 13 |
| **NF-003** | **No gate slowdown.** This story adds no step; check mode is already mandatory and reads one file. `--run` is networked and manual, and never fires from `ci --fast` or `affected` | `.redkiln/config.yaml:40`, `:55`; the gate must stay runnable offline |
| **NF-004** | **Fail closed, never silent.** Any degraded path reports rather than passes, per ADR-0010's discipline that a skip is reported and never silent | `.kb/decisions/0010-the-suite-must-prove-itself.md`; a green gate over an unrun instrument is worse than a red one |
| **NF-005** | The finding record is legible to an adapter author who has read **none** of this project's planning artefacts — it quotes the manifest sentence rather than citing it, and names items by full path | The record's whole audience is downstream of `.bklg/`, which they will never open |
| **NF-006** | The date in the filename is an **argument**, not a clock read at an arbitrary moment, and matches the report's `captured-at` | Two dates that disagree make ancestry unverifiable, which is the only thing check mode can assert |

## Implementation notes (non-prescriptive)

Judgement calls, not instructions.

- **Run the slice-mates first, then capture.** The report is an exit computation over the assembled
  tree (`_storymap.md:146-149`). Capturing before `cf-27-rule-or-recorded-refusal` settles means
  re-capturing, and a stale report in the tree is worse than no report because check mode will pass it.
- **Enumerate from the diff, not from memory or from the brief.** `git diff <project-base>..HEAD` over
  `crates/` filtered to `pub` items is the source list; the brief's seam map is a checklist to notice
  surprises against, and a disagreement is a finding rather than a transcription error to smooth over.
- **Write the two sentences before running anything.** Sentence (2) does not depend on the capture — it
  is a property of the crate, stated in its own manifest. Drafting it first makes it much harder to
  ship a record that quietly stops at sentence (1) because the green result felt like the answer.
- **Decide the AC-006 branch by reading the two reader stories' records, not by predicting them.**
  DA-8's `contains_event_id` observation is the likeliest input and it is *observed* in
  `decision-model-and-ingest-observed`, not assumed here. If it landed as an observation, it needs a
  dispatch; if it landed as a demonstrated hazard needing a surface, it is DA-7's fourth row.
- **Keep the escalation short and priced.** Its reader is a gate, not a design review. Surface, row,
  consequence, table, stop — the temptation to sketch the signature is the thing AC-007 forbids, and
  a sketched signature in prose is one careless copy away from being a change.
- **If a halt fires, halt loudly and early.** Every EC-001 – EC-004 condition is checkable in under a
  minute before any other work; discovering an unpublished baseline after writing the enumeration
  wastes the enumeration, because the verdict it supports cannot be produced.

## Tests and CI (merge gate)

Grounded in the project testing brief's *Merge-gate commands* (`_decomposition.md:637-654`) and its
AC-011 / AC-012 rows (`:613-614`). This project is rank 5 and **not** terminal, so `cargo xtask ci`
in full is `closeout-and-durable-audience`'s, not this story's.

| tier | command / path | proves |
| --- | --- | --- |
| **static — story grain** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | This story's diff maps to no workspace package, so the unconditional block — the five file-reading lints, `spec-trace`, and the mandatory `surface-diff` check step — is what gates it. Proves the committed report is present, well-formed, newest and crate-set-matching. **AC-002** |
| **static — integration grain, cheap tripwire** | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | The record's clause citations (CF-32, ES-37, ES-40) still resolve, and no marker moved under this story's feet after `marker-moves-and-spec-trace-green` closed. **AC-002** |
| **static — integration grain, non-terminal ceiling** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | The whole project's assembled tree is green *with* this report in the gate: fmt, clippy `-D warnings`, tests, the four mandatory `wasm32` steps, `spec-trace`, doc builds, and check mode over `spec/audits/`. **AC-002** |
| **static — registry comparison (manual, networked)** | `cargo xtask surface-diff --run --date <YYYY-MM-DD>`, once per crate in `xtask/src/package.rs:86`'s `PUBLISHABLE`, each with `--baseline-version` | The published `0.2.0` surface versus this tree. The one instrument AC-011 names by name; never run by the gate, always run by this story. **AC-001**, **AC-005** |
| **static — PR grain** | The `semver` job, `.github/workflows/ci.yml:279-316` (`cargo-semver-checks`, `baseline-rev` = `base.sha`) | This diff does not break what it branched from — a different question, with a different blind spot, recorded as such. **AC-003** |
| **static — mechanical diff assertion** | `git diff <project-base>..HEAD -- crates/happenstance-core/src/store.rs crates/happenstance-core/src/append.rs` | No public signature moved, in either AC-006 branch. Rustdoc-only changes are expected and recorded as not a semver surface. **AC-007** |
| **static — enumeration cross-check** | `git diff <project-base>..HEAD -- crates/` filtered to `pub` items, checked row-by-row against the record's list | Every additive item is named; an item in the diff and absent from the list fails. **AC-005** |
| **static — manifest immobility** | `git diff <project-base>..HEAD -- Cargo.toml crates/happenstance-testkit/Cargo.toml` | No version key moved; the implied `0.2.x` is a finding, not a release act. **AC-005** |
| **static — content review** | This story's finding record and escalation record | Both sentences present and in order; exactly one AC-006 branch taken and complete; every reader-story *"missing surface"* finding dispatched; halts (if any) name path, value and owner. **AC-004**, **AC-006**, **AC-008** |
| **not run here** | `cargo xtask ci` (whole gate, `.redkiln/config.yaml:60`); `redkiln validate --kb` | HS-P0019's and the ADR stories' respectively. Useful locally; never this story's recorded proof (`_decomposition.md:650-654`) |

**No new compiled test.** No conformance rule can observe whether a published surface moved — the
subject is a registry artefact and a git range, not a store — and the seeded-break rejection that
proves the *tool* honest was already discharged by HS-S0091 and is not re-proved here (Integration
contract, *Conformance rule(s)*). The falsifiability obligation is met by the instruments themselves:
a capture that finds a break fails this story rather than passing it.

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation, in this PR |
| --- | --- | --- |
| **HS-P0016 has not landed** — no `xtask/src/surface_diff.rs`, no `spec/audits/`, no `REQUIRED` entry. Neither exists in this worktree today (`_grounding.md:12`) | Medium / High — it blocks the story outright | EC-001/EC-004 halt with the owner named. Explicitly **not** mitigated by building a parallel instrument; that would fork the gate and leave two reports disagreeing |
| **`publish-0-2-0` has not shipped**, so there is no registry baseline to diff against | Medium / High | EC-002 halt. The PR-grain job is **not** promoted to fill the gap — that substitution is this story's named wrong implementation (Context pack) |
| **The report is captured too early** and check mode passes it while the slice-mates keep landing | Medium / Medium — it fails *silently*, which is the dangerous shape | Slice order is fixed: this story runs last inside `clause-exit-and-surface-record` (`_storymap.md:146-149`). The enumeration is derived from the merged diff, so a late `pub` item shows up as an EC-006 mismatch |
| **The one-sentence record** — the capture goes green, "no breaking change" feels like the answer, and the mechanism sentence never gets written | High / High — it is the most likely defect in this story, and it looks like success | AC-004 makes both sentences a gated criterion; the implementation notes push the second sentence to be drafted **before** the run |
| **The helpful signature fix** — the escalation branch identifies a real gap and a one-line fix in `store.rs` looks free | Medium / Very High — ES-37 is `[FROZEN]`, so it takes a new decision atom plus a re-plan (`project.md:112-114`) | AC-007's mechanical diff assertion; the PR boundary excludes `crates/**` entirely, so `redkiln verify --grain story` fails on the file before review does |
| **The escalation without a row** — a recommendation that names no DA-7 option, leaving the initiative to re-derive the option table | Medium / Medium | AC-006 requires the row **and** the full four-row table; it is the testing brief's own named wrong implementation for AC-012 (`_decomposition.md:614`) |
| **Reader findings stranded in sibling ledgers** — `decision-model-and-ingest-observed` and `projection-runner-across-the-hole` each record findings this story must ingest | Medium / High | AC-006's cross-read is a verification step, not a courtesy. EC-007 fails the AC on any undispatched finding |
| **Coupling to `cf-27-rule-or-recorded-refusal`'s branch** — whether a third `pub fn` exists to enumerate is not known when this spec is written | Certain / Low | EC-009 makes both branches valid inputs; the enumeration is derived from the diff, so neither branch needs a spec change |
| **Coupling to `xtask` internals** — report format and check-mode semantics are HS-S0091's, not this story's | Certain / Low | Consumed as built and never reconstructed. If the format differs from what this spec describes, the tool is authoritative and the divergence is worth a note in the report's own preamble |

## Dependencies

**Blocks on**

- **`marker-moves-and-spec-trace-green`** (this story's sole `depends_on`) — every `[DEFERRED]` /
  `[PROVISIONAL]` marker move on ES-39, CF-27 and ES-40 lands there, with `cargo xtask spec-trace`
  green afterwards. This story reads CF-32 and ES-37 and cites ES-40's rustdoc-only outcome
  (AC-007); if markers were still moving, the clause citations in the record would be describing a
  tree that no longer exists by the time anyone reads them, and `spec/SPECIFICATION.md` is outside
  this story's PR boundary so it could not fix them anyway.
- **Slice-mate, same context, ordered before this story:** `cf-27-rule-or-recorded-refusal` — its
  branch decides whether a third `pub fn` rule body exists for AC-005 to enumerate (EC-009 accepts
  both outcomes).
- **Upstream, cross-project, consumed as built:** `publication-and-positioning`'s
  `registry-surface-diff` (HS-S0091 — the instrument and the report format) and `publish-0-2-0` (the
  registry baseline itself). Neither is in this worktree yet (`_grounding.md:12`); their absence is
  EC-001/EC-002, a halt, not a build.
- **Sibling inputs, not blockers on the gate but blockers on AC-006's completeness:**
  `decision-model-and-ingest-observed` and `projection-runner-across-the-hole` — their recorded
  *"missing surface"* findings are this story's escalation input, and AC-006 cannot be satisfied
  before both have recorded.

**Unlocks**

- **The project's own exit.** This is the last story in `clause-exit-and-surface-record`, the last
  milestone in the project; the project's Definition of done reads its record.
- **`closeout-and-durable-audience`** (HS-P0019) — its BR-15 audit re-reads this finding record as
  part of the assembled evidence set, and initiative DoD 13 re-observes on the whole tree the claim
  DoD 11 makes here.
- **The initiative's gate decision 4** (`initiative _decomposition.md:239-249`) — the constraint that
  retention's answer needs no published-surface change is *discharged* by AC-005's verdict or
  *escalated* by AC-006, and either way the initiative gets a decision rather than an assumption.

## Anchors (progressive disclosure)

Open these at the named moment; none is pasted above. Every path was confirmed present in this
worktree before citation, except where marked **arrives with HS-P0016** — those are the dependency
this story halts on rather than reconstructs.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/publication-and-positioning/registry-surface-diff/spec.md` | The instrument's own contract: the two modes, the exact `--run` / `--baseline-version` vs `--baseline-rev` distinction (`:70-77`), the report format (`:241-258`), the `release-version` recomputation (`:115-117`, `:250`) and the never-hand-edit rule (`:275-278`). Getting the flags wrong produces a green run of the wrong instrument | **Before running anything** — first artefact to open, ahead of the capture | AC-001 |
| `xtask/src/package.rs` | `PUBLISHABLE` at `:86` is the decided crate set the report must cover exactly; check mode asserts the match. Read-only here | While composing the capture invocations, and again when checking the report's crate blocks | AC-001, AC-005 |
| `xtask/src/main.rs` | The `REQUIRED` list at `:105` is the mount that puts this report inside the gate. Its **presence** is the precondition; its absence is EC-004 | First, as a 30-second precondition check before any capture | AC-002 |
| `crates/happenstance-testkit/Cargo.toml` | `:4-13` is the crate's own statement of why it carries an independent version key, and the sentence AC-004 requires quoted — semver-MINOR that can still turn a passing adapter's CI red | **Before writing the finding record**, ideally before the capture runs | AC-004 |
| `.github/workflows/ci.yml` | `:279-316` is the PR-grain `semver` job; `:305-307` is its own admission of what it cannot see. Run it, record it, do not edit it | When writing the two-comparison paragraph | AC-003 |
| `CONTRIBUTING.md` | `:291-301` states the two-comparison discipline in the repository's contributor-facing words — the phrasing the record should agree with | Alongside `ci.yml`, when distinguishing the instruments | AC-003 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` | The architecture brief's seam map with the *Surface class under AC-011* column (`:54-66`), the semver-MINOR-and-still-red nuance (`:77-88`), **DA-4**'s defaulted-item rule (`:276-308`), **DA-5** (`:310-326`), **DA-7**'s four-row option table (`:363-379`), **DA-8**'s `contains_event_id` lie (`:381-397`), and the testing brief's AC-011/AC-012 rows (`:613-614`). The expectation the report is checked against, never the authority | `:54-66` and `:77-88` before the enumeration; `:363-379` before the escalation | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` | AC-011 (`:235-239`) and AC-012 (`:240-243`) verbatim, DR-11/DR-12 (`:181-185`), the refusal-branch-still-owes-an-artefact discipline (`:174-176`), and AC-A08's *"missing surface"* obligation (`:216-218`) | When taking the AC-006 branch, and when checking the enumeration's scope | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/decision-model-and-ingest-observed/spec.md` | One of the two reader stories whose recorded findings are AC-006's raw material; DA-8's `contains_event_id` / `IngestStore::holds` observation lands here | When assembling the escalation, after that story has recorded | AC-006 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/projection-runner-across-the-hole/spec.md` | The second reader story; any surface its runner could not be made to fail loudly without must be dispatched here | Same moment as the above — read both before choosing a branch | AC-006 |
| `crates/happenstance-core/src/store.rs` | Read-only subject of AC-007's diff assertion; `:268` is `contains_event_id`, DA-7's fourth row and the smallest signature delta — which is exactly why it is tempting | When writing the escalation (to name the path) and when running the diff assertion. **Never to edit** | AC-006, AC-007 |
| `crates/happenstance-core/src/append.rs` | The other half of AC-007's assertion; `Guard::is_violated_by` is where a "helpful" fail-closed change would land | Same moment as `store.rs` | AC-007 |
| `crates/happenstance-core/src/error.rs` | `:214-248` is the error enum a DA-7 third-outcome variant would reach, and the reason that row is priced as touching every caller's `match` | Only if the escalation branch is taken and that row is a candidate | AC-006 |
| `crates/happenstance-testkit/src/contract.rs` | `MID_BATCH_FAULT` at `:207-211` and `:297-307` is the **defaulted** `Capability` + defaulted-method precedent DA-4 requires any new `Fixture` seam to mirror; a *required* item there would be the breaking change AC-011 exists to catch | When classifying any `Fixture` addition in the enumeration | AC-005 |
| `crates/happenstance-testkit/src/lib.rs` / `src/registry.rs` | `lib.rs:188` is the re-export path that makes a rule body public surface; `registry.rs:94` is `for_each_event_store_rule!`, where a registration is itself additive surface | While enumerating each new rule body by name and class | AC-005 |
| `spec/SPECIFICATION.md` | CF-32 (`:8200-8220`, `[FROZEN]`) is why the diff is per crate against per-crate baselines; ES-37 (`:4274-4297`, `[FROZEN]`) is why a signature change here costs a new decision atom and a re-plan rather than an edit | CF-32 before the capture; ES-37 before any temptation to fix something | AC-001, AC-007 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The accepted decision atom behind fail-closed: a skip is reported, never silent. It is the authority for halting instead of substituting an instrument that passes | The moment any precondition looks shaky | AC-008 |
| `.redkiln/config.yaml` | `:40` (story grain), `:55` (integration grain, this project's ceiling) and `:60` (whole gate, HS-P0019's) — the commands that fire automatically at each stage transition, which is why the report must be *in* the gate rather than beside it | When verifying AC-002, and to confirm what this story is **not** required to run | AC-002 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_grounding.md` | §5 (`:12`) records exactly what has not landed in this worktree — the `0.2.0` baseline and the surface-diff tooling. The evidence that EC-001/EC-002 are live conditions, not hypotheticals | Before starting, to decide whether this story can run at all | AC-008 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` | Binding and signed off. `:38-88` is the no-surface determination with every section `N/A`; `:32-36` is the Public API surface note that defers the testkit surface review to AC-011 — this story | Once, when writing the record, to confirm no surface obligation was missed | AC-005 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md` | `:78` is this story's row; `:111-115` is the consumed-as-built dependency on HS-P0016; `:146-149` fixes this story last in the slice | At slice sequencing time | AC-001, AC-008 |
| `.bklg/from-contract-to-published-library/initiative.md` | AC-11 (`:338-340`) — the consumer's build not broken by surprise; `:200-225` — the four personas the ACs are framed from; DoD 11 (`:390-392`) and DoD 15 (`:402-405`) | When framing the record for its downstream reader | AC-001, AC-004 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The initiative's own persona/journey source, cited because `.kb/product/` is structurally present and functionally empty (`initiative.md:227-234`). The adapter author's *"Learn when you are finished"* journey is the one AC-004's second sentence serves | When checking the record reads for its audience rather than for the gate | AC-004 |
| `.bklg/from-contract-to-published-library/_decomposition.md` | Gate decision 4 (`:239-249`): retention's answer is constrained to what needs no published-surface change, with a `0.3.0` outside the exit criteria as the named rejected alternative — the constraint AC-005 discharges or AC-006 escalates | Before choosing the AC-006 branch, to price the consequence correctly | AC-006 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the first pass enumerated** — AC-001 through AC-008, none added
   and none dropped. B1–B12 map onto them without residue (see *Traceability* under the acceptance
   criteria); the behaviours are finer-grained than the criteria on purpose, because several are one
   reader's single expectation seen from two angles (B4/B6/B7 are all "the enumeration is
   re-derivable", which is AC-005).
2. **"Two sentences" is a floor on content, not a cap on length.** AC-004 requires both claims, in
   order, with the second quoted from the manifest. A record that says more is fine; a record that
   says only the first fails even though it is true. This was the one place the one-line slice could
   be read as permitting brevity, and it does not.
3. **AC-006 is one criterion with two branches, not two criteria.** The escalation branch and the
   no-escalation branch are mutually exclusive and each is complete on its own terms, so a single row
   with an explicit "exactly one branch, taken explicitly" test is what the ledger can actually check.
   Splitting them would leave one row permanently unsatisfiable and block the gate on a branch that
   correctly did not happen.
4. **The no-escalation branch is an artefact, not an absence.** Resolved against `project.md:174-176`'s
   DR-9 precedent one level up: the refusal branch still owes its writeup. An empty escalation section
   reads identically to an escalation nobody wrote, so AC-006 fails on emptiness in both directions.
5. **This story renders no surface, and the interaction-quality section is still binding.**
   `_design.md` declares `N/A` throughout (`:38-88`) and the owner signed off the no-surface
   determination itself. The state and composition invariants were therefore translated into the
   medium this story ships — committed text under a gate — and each is carried by an existing AC row
   rather than added as a prose bullet, because `redkiln verify` extracts ACs from table cells and a
   bullet here would be ungated and untested.
6. **No new compiled test is owed, and that is a decision rather than an omission.** No conformance
   rule can observe a published-surface move; the instruments themselves are the falsifiers, and
   HS-S0091 already proved the tool rejects a seeded break. Re-proving it here would be the
   decorative rule `CLAUDE.md`'s corollary forbids.
7. **The `<YYYY-MM-DD>` in the mount point stays a placeholder in this spec.** The date is the
   implementer's `--date` argument at capture time and must equal the report's `captured-at`
   (NF-006); writing a concrete date into the spec would guarantee one of the two is wrong.
8. **`git diff <project-base>..HEAD` means the project's merged range**, not this story's PR diff —
   AC-005 and AC-007 are assertions about what the *project* did to the surface, which is why this
   story runs last and why its own diff touches no source at all.
9. **A halt is a legitimate terminal state for this story.** EC-001 – EC-004 are live in this
   worktree today (`_grounding.md:12`), and AC-008 is written so that halting-and-reporting satisfies
   it. What does not satisfy it is proceeding with a substitute instrument and a verdict nobody can
   re-derive.
