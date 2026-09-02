---
item: HS-S0126
stage: spec
created: 2026-08-12T13:48:05.885Z
updated: 2026-08-12T13:48:05.885Z
template_sig: 87bbf1d0
rendered_sig: 9c69a743
---

# Spec — The whole gate green on the assembled tree, with its SHA and every skip accounted for

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) — DoD 13, BR-12, exit criterion 8 |
| Initiative decomposition | [`.bklg/from-contract-to-published-library/_decomposition.md`](../../_decomposition.md) — the DAG, this project as rank-6 sink |
| Project | [`.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md`](../project.md) — AC-001, AC-002, DR-1, DR-2, DR-5, DR-12; DoD 1 |
| This spec | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/whole-gate-green-on-the-assembled-tree/spec.md` |
| Key brief | [`.bklg/.../closeout-and-durable-audience/_decomposition.md`](../_decomposition.md) — the one warranted brief (`testing`): AC-001 rows (tier E2E), AC-002 (tier Integration-inside-E2E), *Merge-gate commands*, *Fixtures / seams* |
| Signed-off design | [`.bklg/.../closeout-and-durable-audience/_design.md`](../_design.md) — **no public API surface**, `## Items` is `N/A`. This story renders no surface and claims no item. |
| Story map row | [`.bklg/.../closeout-and-durable-audience/_storymap.md`](../_storymap.md) — slice `assembled-tree-gate`, row 2 |
| Grounding | [`.bklg/.../closeout-and-durable-audience/_grounding.md`](../_grounding.md) — what `cargo xtask ci` actually proves, in order |
| Roadmap pointer | [`RUNBOOK.md`](../../../../RUNBOOK.md) — the plan of record; `xtask/src/main.rs` is the gate, defined once |

## One-line PR slice

Run `cargo xtask ci` — the terminal grain at `.redkiln/config.yaml:60`, never `--fast` — to
completion inside the clean checkout `clean-checkout-harness` built, and record into
`_closeout-record.md` the exit code, the commit SHA it ran against, the four `wasm32` step
outcomes named individually, and the absent tool behind every step that printed `skipped`.

## Executive summary

**What this PR lands:** one observed execution of the whole gate on the assembled tree, plus
the accounted record of it — a `gate-run.md` transcript record in this story's folder and the
**Gate run** section mounted into the project-level `_closeout-record.md` that every later
closeout story cites.

**Delta against what already exists.** Nine sibling projects each ran `cargo xtask ci --fast`
(`.redkiln/config.yaml:55`) against a tree containing only their own work. That bar is already
met, nine times, and re-running it here would prove nothing (`project.md` DR-2). Three things
are new and only observable here:

1. The two feature powersets, `cargo deny` and the nightly `--cfg docsrs` build — the whole
   `OPTIONAL` set at `xtask/src/main.rs:535` — run for the first time in this initiative's
   backlog process, because `--fast` drops exactly them (`xtask/src/main.rs:853`).
2. The tree is *assembled*: a cross-project interaction between two independently-green
   projects is invisible until now (`project.md` *How this advances the initiative*).
3. The run happens in a checkout with no residue and no path dependency standing in for a
   published version (DR-1), which is the slice-mate's deliverable and this story's precondition.

The story writes **no production code and fixes nothing**. A red step here is the finding this
project exists to surface, and it is routed, not repaired (`project.md` AC-013, DR-12).

## Context pack

The load-bearing decisions this story must honour. Read this section and you can start; the
deeper artefacts are behind the signposted anchors the second pass appends.

**The command is `cargo xtask ci`, whole, and substituting `--fast` fails the story rather than
saving time.** `.redkiln/config.yaml:56-60` reserves the `e2e` grain for this project alone and
its own comment calls it "this repository's Definition of Done"; `:50-55` is the
`integration_scoped` bar every sibling already met. The gate is long — two feature powersets, a
`cargo deny` run, a nightly rustdoc build — and a command gate fires on **both** entry and exit
of its stage (`.redkiln/config.yaml:50-54`), so budget the wall-clock twice. That cost is the
decision, not an accident of it (`project.md` risk table, row 1).

**The run happens inside the slice-mate's checkout, not in the working tree these planning
artefacts live in.** `clean-checkout-harness` (HS-S0125, `blocked_by` on this story's card)
constructs the one seam this project builds deliberately: a fresh checkout with no untracked or
ignored residue and no path dependency standing in for a published version (`project.md` DR-1;
`_decomposition.md` *Fixtures / seams*: "it is the thing AC-001 is actually checking for"). The
harness is not a setup step this story may re-do more cheaply — the slice exists precisely
because "make a clean checkout" and "run the gate in it" are not separable observations
(`_storymap.md` *Why the slices are cut here*).

**Every `skipped` line is accounted for by a named absent tool, and on this machine a skip is
itself a finding.** `run_steps` at `xtask/src/main.rs:862` prints exactly
``skipped: `<probe>` did not succeed`` and continues, only for the four `OPTIONAL` steps that
carry a probe: *feature powerset* and *wasm32 feature powerset* (`cargo hack --version`),
*licences and advisories* (`cargo deny --version`), *docs.rs configuration (nightly)*
(`cargo +nightly --version`). `CLAUDE.md` *Commands* records that `cargo-hack` and `cargo-deny`
both resolve on this machine, so those two steps are expected to **run**, and a `skipped` line
for either is a degraded gate to record and route — not a green run with a footnote. Optional
means *skipped when the tool is absent*, never *ignored when it fails* (`xtask/src/main.rs:43-51`).

**The four `wasm32` steps are named individually, never counted.** They are
`wasm32 build of the contract crate`, `wasm32 check of the conformance harnesses`,
`wasm32 build of the Cloudflare adapter` and `wasm32 build of the Neon adapter`
(`xtask/src/main.rs:769-793`, which selects them **by name** and documents why an index was
wrong there). They are the standing guard on ADR-0001's `!Send` port flavour
([`.kb/decisions/0001-async-port-flavours.md`](../../../../.kb/decisions/0001-async-port-flavours.md);
`CLAUDE.md` binding constraint 1), and `project.md` DR-5/AC-002 pull them out of "the gate was
green" deliberately: BR-12's "exercised end to end by everything that ships" is observed here,
on the assembled tree, rather than inherited from HS-P0013's own merge. Recording "4/4 wasm32
green" without the four names is what this AC forbids.

**A red step is a finding to route, not a defect to fix.** `project.md` DR-12 and the risk-table
row "Pressure to fix what re-observation finds": an incidental defect goes to the `support`
initiative (`.redkiln/config.yaml:5`), a genuine cross-project interaction becomes a new item
against the owning sibling, anything touching a `[FROZEN]` clause takes a new decision atom and
a re-plan. This story's card `blocks` HS-S0134 (`findings-disposition-register`) for that
reason. Zero commits from this story may touch `crates/**`, `xtask/**` or `spec/**`.

**Do not overclaim what green means.** `CLAUDE.md` *Commands* is explicit that the MSRV is the
one thing the local gate does not check — CI carries a dedicated `msrv` job — and that a step
skips only when its probe fails. The record states what ran, on which SHA, on which host
toolchain, and says so; it does not translate a green local gate into a claim about a consumer's
compiler.

**The persona-journey slice.** This project's reader is the one from
`_storymap.md`'s preamble: someone who must be able to believe the initiative closed honestly
**without re-deriving the evidence**. That is the evaluation journey the project's brief settled
as a first-class journey atom of its own (`_decomposition.md` *The evaluator-persona decision*,
as amended). For this story it means the record is written for a reader who was not present at
the run: a step-by-step outcome table with the SHA, the host facts and the verbatim `skipped`
lines, not a sentence asserting a green gate.

**Evidence converges on one artefact, not fourteen.** Each story carries its own `_ledger.md`
(`.redkiln/config.yaml:62-73`, `require_ledger` and `require_commit_provenance`), and the
cited evidence lands in the project-level `_closeout-record.md` that the slice-mate stands up —
"fourteen ledger entries in fourteen places is the failure mode the charter's DoD preamble is
written against" (`_storymap.md` *Where the evidence lands*).

## Integration contract

- **Archetype**: `capability` — the whole observable slice, from a residue-free checkout through
  the gate to a record a reader can follow back (`story.md` frontmatter `archetype: capability`).
- **Slice / milestone**: `assembled-tree-gate`. Slice-mate: `clean-checkout-harness` (HS-S0125,
  `foundation`), implemented in the same context and landed as one integrated surface.
- **Mount point**:
  `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` —
  the project-level closeout record, created by the slice-mate in this same slice and the single
  artefact every later closeout story appends cited evidence to (`_storymap.md` *Where the
  evidence lands*). This story mounts a **Gate run** section into it. A `gate-run.md` sitting in
  this story's folder and cited by nothing is the unmounted-component failure: the record is the
  composition root here, and the transcript is only reachable through it.
- **Wires into**:
  - `xtask/src/main.rs` — the gate, defined once; `REQUIRED` (`:105`), `OPTIONAL` (`:535`),
    `wasm32_steps` (`:769-793`), `run_ci` (`:828`), `run_steps`'s skip line (`:876`). Consumed by
    invocation only; **not modified**.
  - `.redkiln/config.yaml:56-60` — `verify.e2e`, the terminal grain that runs this command
    whether or not anyone types it; `:62-73` — the ledger and commit-provenance requirements.
  - The clean checkout produced by `clean-checkout-harness` — a process seam, not a code
    fixture, and the only fixture this slice constructs (`_decomposition.md` *Fixtures / seams*).
  - Downstream consumers of the mounted section: `dod-set-re-observation-record` (HS-S0127) and
    `findings-disposition-register` (HS-S0134), both listed in this story's `blocks`.
- **Renders surfaces**: **none**. `_design.md` states "**No public API surface**" and its
  `## Items` block is `N/A`; this story claims no item id, adds no `pub` item, and changes no
  signature. There is no screen anywhere in this initiative
  (`../../_decomposition.md`, *userFacing: false*).
- **Conformance rule(s)**: none, and deliberately. This story is **not adapter-observable**: it
  writes no port, no rule and no adapter, and it *runs* the existing suite as the gate's `tests`
  step rather than adding to it (`_decomposition.md` *Fixtures / seams*: "it introduces no new
  fixtures"). A conformance rule invented here would be a rule no adapter can fail — the
  decorative-rule failure `CLAUDE.md` names.
- **Clause(s)**: none discharged or amended. `spec/SPECIFICATION.md` is *exercised* by the gate's
  `cargo xtask spec-trace` step and is not edited by this story; a clause change would take a new
  ADR, which is out of scope here (`project.md` *Out of scope*).
- **Advances DoD scenario**: initiative **DoD 13** — "The gate is green on the assembled whole.
  `cargo xtask ci` passes, including the specification cross-reference step"
  (`../../initiative.md:396-397`) — the "on the exact tree that was published" half being
  qualified by the slice-mate story `published-tree-delta-statement` (AC-004), not by this one.
  It also carries **BR-12** to closeout via the four `wasm32` steps (`project.md` DR-5) and is
  project **DoD 1**.

## PR boundary

```
.bklg/from-contract-to-published-library/closeout-and-durable-audience/whole-gate-green-on-the-assembled-tree/**
.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md
```

The boundary is honestly this narrow because the story owns no code. `crates/**`, `xtask/**`,
`spec/**`, `.github/**` and `.kb/**` are all **outside** it, and that exclusion is the mechanism
behind AC-013's "zero were fixed inside this project": a repair to a red gate step would show up
as a boundary violation under `redkiln verify --grain story`, which is exactly the alarm wanted.

**In this PR**

- The gate run itself, `cargo xtask ci`, executed to completion in the slice-mate's clean
  checkout, with its stdout/stderr captured verbatim.
- `whole-gate-green-on-the-assembled-tree/gate-run.md` — the accounted record: command,
  host/toolchain facts, commit SHA, exit code, per-step outcome table (every `REQUIRED` and
  `OPTIONAL` step by its `xtask` name), the four `wasm32` rows called out, and each verbatim
  ``skipped: `<probe>` did not succeed`` line with the tool it names.
- The **Gate run** section mounted into `_closeout-record.md`, citing `gate-run.md` by path and
  the SHA by value, in the shape the slice-mate's record defines.
- This story's `_ledger.md` (second pass) and the `redkiln record-links --sha` provenance the
  `require_commit_provenance` setting demands.

**Explicitly not in this PR**

- Any fix, workaround, `#[allow]`, lock-file bump or feature-flag change made to turn a red step
  green — that is a finding, routed by HS-S0134 (`project.md` DR-12, AC-013).
- Any change to `xtask/src/main.rs`, including "obviously right" ones such as adding a probe or
  reordering a step. The gate is the instrument; an instrument edited to produce the reading is
  no longer an instrument.
- Constructing or repairing the clean checkout — `clean-checkout-harness` (HS-S0125) owns DR-1's
  residue and path-dependency checks; this story consumes them and cites them.
- The DoD 1–12 / 14–15 re-observation table (HS-S0127) and the DoD 13 published-tree delta
  (`published-tree-delta-statement`). This story supplies the run those stories cite; it does not
  write their tables.
- `cargo xtask ci --fast`, in any form, for any reason (`project.md` DR-2).

**Merge DoD:** the gate ran whole in the clean checkout, and a reader who was not there can
open `_closeout-record.md`, read the SHA, see all four `wasm32` steps green by name, and find
either zero `skipped` lines or a named absent tool for each one.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The whole gate is invoked, not the fast subset | `cargo xtask ci` with no flags, from the clean checkout's root. `run_ci` runs `REQUIRED` then `OPTIONAL`; `run_fast` is the sibling bar and prints "(--fast: N optional step(s) not run)" — that string appearing anywhere in the transcript falsifies the run. | `xtask/src/main.rs:828`, `:853-860`; `.redkiln/config.yaml:55`, `:60` |
| The run is bound to a tree, by SHA | The record states the commit SHA the run started from and that the checkout was residue-free at that moment, taking the harness's own residue evidence as its precondition rather than restating it. | `project.md` AC-001, DR-1; `../_decomposition.md` *Fixtures / seams* |
| Exit code is recorded as observed | Zero, or non-zero with the first failing step's name. `run_steps` bails with `"{step.name} failed with {status}"`, so the failing step is nameable from the transcript without re-running. | `xtask/src/main.rs:862-890` |
| Every step appears in the record by its `xtask` name | The record enumerates each `REQUIRED` and `OPTIONAL` step with its outcome (`passed` / `failed` / `skipped`). The names are the `Step::name` values, not paraphrases, so a step added or renamed upstream shows as a mismatch rather than blending in. | `xtask/src/main.rs:105` (`REQUIRED`), `:535` (`OPTIONAL`) |
| The four `wasm32` steps are green, individually named | `wasm32 build of the contract crate`, `wasm32 check of the conformance harnesses`, `wasm32 build of the Cloudflare adapter`, `wasm32 build of the Neon adapter` — each with its own row. These are `REQUIRED` and carry no probe, so a `skipped` line for any of them is impossible and its absence from the transcript is a real failure. | `xtask/src/main.rs:769-793`, `:203-283`; `.kb/decisions/0001-async-port-flavours.md`; `project.md` AC-002, DR-5 |
| Every `skipped` line names the absent tool | The verbatim ``skipped: `<probe>` did not succeed`` line is quoted with the probe it prints (`cargo hack --version`, `cargo deny --version`, `cargo +nightly --version`). A step reported skipped with no matching transcript line is unaccounted for and fails the story. | `xtask/src/main.rs:872-878`; `project.md` AC-001, DR-2 |
| A skipped `cargo hack` / `cargo deny` is a finding, not a footnote | `CLAUDE.md` *Commands* records that both resolve on this machine and that "a green local gate now proves more than it used to". A skip of either narrows the gate's coverage and is recorded as a finding for HS-S0134 to route. | `CLAUDE.md` *Commands*; `.redkiln/config.yaml:5` |
| Green is stated at the width it was proven | The record names the host OS, the toolchain that ran (the `rust-toolchain.toml` pin), and that the MSRV is checked by CI's `msrv` job rather than by this run. No sentence in the record may read as a consumer-compiler claim. | `CLAUDE.md` *Commands* (MSRV paragraph); `.kb/decisions/0029-msrv-raised-to-1-97-1.md` |
| The run is mounted, not filed | The **Gate run** section exists in `_closeout-record.md` and cites `gate-run.md` and the SHA. HS-S0127 and HS-S0134 reach this evidence through that record only. | `_storymap.md` *Where the evidence lands*; `.redkiln/config.yaml:62-73` |
| Nothing is repaired | The diff touches only the two globs in *PR boundary*. Any red step leaves the tree unchanged and produces a routed finding instead. | `project.md` AC-013, DR-12; `../_storymap.md` *Grain notes* |
| No public API surface is added or changed | `_design.md` declares `N/A` for `## Items`, `## Signatures` and `## Visibility and stability`; this story claims none of them. | `../_design.md` *Items* |

## Data and migrations

**N/A.** This story writes no production code, defines no schema and touches no store. Its only
persisted artefacts are two markdown files inside the PR boundary — `gate-run.md` and the
**Gate run** section of `_closeout-record.md` — and neither is read by a program: `redkiln`
reads item frontmatter, which the CLI alone writes, and the gate reads the source tree, which
this story does not modify. The one durable structure the story depends on is the
`_closeout-record.md` section layout the slice-mate defines; a change to that layout is the
slice-mate's to make, and this story follows it rather than negotiating a second shape
(`_storymap.md` *Where the evidence lands*).

## Acceptance criteria

The persona is the one the project's story map names in its preamble and the amended DR-10
settles as a journey atom of its own: **the evaluator reading a closed initiative, who must be
able to believe it closed honestly without re-deriving the evidence, and who cannot run the
suite themselves** (`../_decomposition.md` *The evaluator-persona decision*, as amended;
`../../_discovery/distillation/personas-and-journeys.md:249-283`, *Persona 4*). Every criterion
below is that reader's goal crossing the full stack — from the checkout, through the gate, to a
record they can follow back — not a capability the run happens to have.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** the residue-free checkout `clean-checkout-harness` produced, **WHEN** the implementer runs `cargo xtask ci` with no flags from its root and lets it finish, **THEN** the run reaches its own end (`REQUIRED` then `OPTIONAL`, `xtask/src/main.rs:828-831`) and the record states the observed exit code — zero, or non-zero naming the first failing step from `run_steps`'s ``{step.name} failed with {status}`` bail — **AND** the string `--fast: ` appears nowhere in the captured transcript, so the reader can tell the release bar from the sibling bar without trusting a claim. | The captured transcript in `whole-gate-green-on-the-assembled-tree/gate-run.md` §*Command and exit*: the verbatim command line, the exit code, and a grep of the transcript for `--fast: ` returning nothing (`xtask/src/main.rs:853-860`). Process check: `redkiln verify --grain story` against `.redkiln/config.yaml:60`'s `e2e` command. |
| **AC-002** | **GIVEN** the evaluator wants to re-derive the run rather than accept it, **WHEN** they open the record, **THEN** it names the 40-character commit SHA the gate ran against and cites the slice-mate's residue evidence for that same SHA by path, **SO THAT** they can check out that commit themselves and find the tree the gate saw — not "a recent tree", not "the closeout branch". | `gate-run.md` §*Tree under test* carries `git rev-parse HEAD` and `git status --porcelain` output captured immediately before the run, and cites `clean-checkout-harness`'s own residue artefact by path. Cross-check: the SHA in `_closeout-record.md`'s **Gate run** section equals the SHA in `gate-run.md` (`project.md` AC-001, DR-1). |
| **AC-003** | **GIVEN** the evaluator wants to know what "the gate was green" actually covered, **WHEN** they read the record's step table, **THEN** every one of the gate's **24** steps — the 20 in `REQUIRED` (`xtask/src/main.rs:105-534`) and the 4 in `OPTIONAL` (`:535-637`) — appears as its own row carrying its verbatim `Step::name` and one of `passed` / `failed` / `skipped`, **SO THAT** a step added, renamed or dropped upstream shows as a mismatch instead of blending into a summary sentence. | `gate-run.md` §*Steps* compared row-for-row against the `name:` values in `xtask/src/main.rs` (24 rows, no paraphrase, no roll-up row). Any transcript `=== <name> ===` banner (`:864`) with no matching table row, or any table row with no banner, fails the criterion. |
| **AC-004** | **GIVEN** BR-12's promise that the `!Send` port flavour is "exercised end to end by everything that ships", **WHEN** the evaluator looks for it at closeout rather than at `cloudflare-durable-object-store`'s own merge, **THEN** the record shows `wasm32 build of the contract crate`, `wasm32 check of the conformance harnesses`, `wasm32 build of the Cloudflare adapter` and `wasm32 build of the Neon adapter` each **named individually and each `passed`** — a count such as "4/4 wasm32 green" fails this criterion. | The four rows in `gate-run.md` §*Steps*, matched by name against `wasm32_steps` (`xtask/src/main.rs:769-793`, which selects by name precisely so an index cannot silently reselect), and their four `=== … ===` banners in the transcript. These are `REQUIRED` with `probe: None`, so `skipped` is impossible and absence is a real failure (`project.md` AC-002, DR-5; `.kb/decisions/0001-async-port-flavours.md`). |
| **AC-005** | **GIVEN** the evaluator cannot distinguish "green" from "green with the advisory scan switched off" unless the record tells them, **WHEN** any step prints ``skipped: `<probe>` did not succeed`` (`xtask/src/main.rs:876`) or the run exits non-zero, **THEN** the record quotes that line verbatim and names the absent tool it identifies (`cargo hack --version`, `cargo deny --version`, `cargo +nightly --version`), **AND** — because `CLAUDE.md` *Commands* records that `cargo-hack` and `cargo-deny` both resolve on this machine — a skip of either, or any failing step, is written into the record as a **finding with a destination** for `findings-disposition-register` (HS-S0134) rather than repaired, with the diff confined to the two globs in *PR boundary*. | `gate-run.md` §*Skips and findings*: one verbatim quoted line per skip, each mapped to its probe; or the explicit sentence "no step printed `skipped`". Boundary proof: `git diff --stat <base>..HEAD` touches only `…/whole-gate-green-on-the-assembled-tree/**` and `…/_closeout-record.md` — zero paths under `crates/`, `xtask/`, `spec/`, `.github/` (`project.md` AC-013, DR-12; `.redkiln/config.yaml:5`). |
| **AC-006** | **GIVEN** the evaluator arrives at `_closeout-record.md` and nowhere else, **WHEN** they read its **Gate run** section, **THEN** they reach the SHA, the exit code, the four `wasm32` outcomes and the skip accounting from that section — by reading it or by following its one cited link to `gate-run.md` — **AND** the section states the width the green was proven at (host OS, the `rust-toolchain.toml` pin that ran, and that the MSRV floor is checked by CI's `msrv` job, not by this run), **SO THAT** nothing in it reads as a claim about a consumer's compiler. | The **Gate run** section exists in `.bklg/…/closeout-and-durable-audience/_closeout-record.md`, in the shape `clean-checkout-harness` defined, and cites `gate-run.md` by repo-relative path plus the SHA by value. Static check: `rg -n "gate-run.md" .bklg/…/_closeout-record.md` returns a hit; `rg` for the host/toolchain/MSRV sentences returns all three (`CLAUDE.md` *Commands*, MSRV paragraph; `.kb/decisions/0029-msrv-raised-to-1-97-1.md`; `rust-toolchain.toml`). |

**Coverage of the traced project ACs.** `project.md` **AC-001** ("whole gate green from a clean
checkout, SHA recorded, every `skipped` step listed with the absent tool") is covered by AC-001
(whole, not `--fast`, run to completion), AC-002 (the SHA and the residue-free precondition),
AC-003 (every step accounted for) and AC-005 (the skip accounting). `project.md` **AC-002** (the
four `wasm32` steps green in that same run) is covered by AC-004, with AC-006 making it
*reachable* — an unmounted `gate-run.md` would satisfy AC-004 and still leave the project AC
unevidenced. No AC here claims `project.md` AC-003, AC-004 or AC-013: those belong to
`dod-set-re-observation-record`, `published-tree-delta-statement` and
`findings-disposition-register` respectively, and this story only feeds them.

## Interaction quality

This story **renders no surface**. `_design.md` states "**No public API surface**" and returns
`N/A` for `## Items`, `## Signatures`, `## Visibility and stability`, `## The states the API must
express` and `## Anti-patterns`; the initiative is `userFacing: false` and
`../../_decomposition.md:266` records "there is no screen anywhere in this initiative";
`design.capture` is deliberately absent from `.redkiln/config.yaml`, which makes the perceptual
review a declared skip rather than a silent pass (`CLAUDE.md`, *Where the work lives*).

So the surface-render invariants — non-occlusion of a viewport, preserved focus/scroll/selection,
keyboard reachability, composed presentation over bare markup, persistent-versus-revealed chrome —
are **N/A here, and stated as N/A rather than skipped**. What is *not* N/A is the same family of
invariants applied to the artefact this story actually delivers: a record a reader who was not
present must be able to use. Those are blocking, and each is carried by an AC row in the table
above, never by a bullet here.

**STATE family — how the evidence behaves for its reader.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump.** The evaluator reaches every fact from the **Gate run** section they landed on, or from its single cited link. A second, uncited artefact they must know to look for is the context jump. | **AC-006** | `_closeout-record.md`'s section cites `gate-run.md` by path and carries the SHA and exit code inline; no third hop exists. |
| **Non-occlusion.** No summary sentence stands in front of the detail it summarises: the 24 step outcomes and every verbatim `skipped` line are present, not folded behind "all steps passed". | **AC-003**, **AC-005** | Row count equals 24; one quoted line per skip, or the explicit "no step printed `skipped`". |
| **Reversibility.** The record is append-only within its section: a second gate run adds a dated run alongside the first rather than overwriting it, so a reader can see that it was run twice and why. | **AC-006** | The **Gate run** section's shape (defined by `clean-checkout-harness`) admits more than one dated run; a re-run edits nothing already written. |
| **Provenance is preserved, not restated.** The SHA is the identity of the observation; a record whose SHA cannot be checked out is a claim, not evidence. | **AC-002** | `git rev-parse HEAD` and `git status --porcelain` captured before the run and quoted in `gate-run.md`. |

**COMPOSITION family — taken from the signed-off `_design.md` and the project's own grain rules.**
`_design.md` supplies no visual composition to honour (it declares none), so the composition
invariants that bind here are the ones the project settled instead, and the density budget is a
real number rather than a gesture.

| Invariant | The real number / rule | Carried by |
| --- | --- | --- |
| **Presentation exists at all.** The record is a composed artefact — command, tree, steps, skips, findings, width-of-claim — not a pasted terminal dump with a sentence on top. A raw log satisfies every "the evidence is present" check and is unusable to the reader this project exists for. | Six named sections in `gate-run.md`; one **Gate run** section in `_closeout-record.md`. | **AC-006** |
| **Density budget.** **24** step rows (20 `REQUIRED` + 4 `OPTIONAL`), of which **4** are the `wasm32` rows called out by name; **1** verbatim quoted line per `skipped` step (0–4); **1** SHA; **1** exit code. Fewer rows than 24 means a step was summarised away; more means the gate changed and the record noticed. | Counted against `xtask/src/main.rs:105` and `:535`. | **AC-003**, **AC-004**, **AC-005** |
| **Hierarchy.** Exit code and SHA lead; the step table follows; skips and findings sit beneath it with their verbatim lines; the width-of-claim sentences close. The reader's first question ("did it pass, on what?") is answered before the detail that qualifies it. | Section order in `gate-run.md` and in the mounted section. | **AC-006** |
| **Transience.** Nothing here is revealed-on-demand: the mounted **Gate run** section is persistent chrome in the closeout record, because two later stories (HS-S0127, HS-S0134) read it as an input. `gate-run.md` is the opened-on-demand depth behind it. | The mount is a section, not a link-only stub. | **AC-006** |

**The design's named anti-patterns, as they land here.** `_design.md`'s `## Anti-patterns` is
`N/A` for API shape and points at the standing `CLAUDE.md` constraints as "not exercised by this
one". The anti-patterns that *are* live are the project's, and each is the negative of an AC:

- **"4/4 wasm32 green."** Counting instead of naming — forbidden by **AC-004**
  (`project.md` DR-5; `xtask/src/main.rs:771-781` records why an index was wrong in the gate for
  the same reason).
- **A green assertion with no SHA.** Forbidden by **AC-002** — unverifiable by construction.
- **Substituting `cargo xtask ci --fast`.** Forbidden by **AC-001**; it is the bar nine siblings
  already met (`.redkiln/config.yaml:55`; `project.md` DR-2).
- **Ticking a step from the memory of the project that produced it.** Forbidden by **AC-003**
  (`project.md` DR-3; `_storymap.md` preamble).
- **A `gate-run.md` nobody cites.** The unmounted-component failure — forbidden by **AC-006**.
- **Turning a red step green.** Forbidden by **AC-005**'s boundary clause (`project.md` AC-013,
  DR-12).

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | `cargo xtask ci` exits non-zero. | Record the exit code and the failing step name from `run_steps`'s ``{step.name} failed with {status}`` bail (`xtask/src/main.rs:887`), capture its output verbatim, and open a finding row with a proposed destination for HS-S0134. **Do not** fix, `#[allow]`, re-run-until-green or bump a lock file. The story is *complete* with a red gate and a routed finding; it is *failed* by a repair. |
| **EC-002** | An `OPTIONAL` step prints `skipped` for `cargo hack` or `cargo deny`. | Quote the line, name the probe, and record it as a **finding**, not a footnote — `CLAUDE.md` *Commands* states both resolve on this machine, so a skip means the local gate proved less than it claims. Installing the tool and re-running is permitted (it changes no tracked file); silently accepting the skip is not. |
| **EC-003** | `cargo +nightly --version` fails and the `docs.rs configuration (nightly)` step skips. | Quote the line and name the toolchain. This one is an expected-absence case rather than a finding on a host without a nightly toolchain; the record says which it was, and never leaves the reader to guess. |
| **EC-004** | The transcript's `=== <name> ===` banners do not match the 24 names in `REQUIRED`/`OPTIONAL`. | A step was added, renamed or removed upstream since this spec was written. Record the mismatch as a finding against the owning change; do **not** edit `xtask/src/main.rs`, and do not quietly re-count. The mismatch is the signal the by-name table exists to produce. |
| **EC-005** | The string `--fast: ` appears in the transcript, or the invocation carried the flag. | The run is void. Discard it, do not record it as evidence, and re-run the whole gate (`project.md` DR-2). |
| **EC-006** | The run is interrupted — timeout, host sleep, cancelled shell — before `OPTIONAL` completes. | A partial run is not evidence for AC-001. Record the interruption in `gate-run.md` for honesty, then re-run whole from the same SHA; the record shows both attempts (see the reversibility invariant). |
| **EC-007** | `clean-checkout-harness` has not landed, or `_closeout-record.md` does not exist / has no defined section shape. | **Stop.** This story's `blocked_by: HS-S0125` is real: running the gate in the planning worktree would satisfy no criterion here (DR-1) and mounting into a record that does not exist is the unmounted failure. Report the missing dependency rather than improvising a second record shape. |
| **EC-008** | A step fails on one run and passes on a re-run at the same SHA. | Record **both** runs. A flake is a finding with a destination, not a result to select from — picking the green run is the precise failure `project.md` DR-3 and the risk-table row "Pressure to fix what re-observation finds" are written against. |
| **EC-009** | The working tree acquires residue (build artefacts, generated files) *during* the run, so the post-run `git status` is dirty. | Expected and benign for ignored build output; state it. Any **untracked, non-ignored** file appearing during the run is a finding — the gate wrote something the repository does not know about. |

## Non-functional

| id | Requirement | Why, and where it comes from |
| --- | --- | --- |
| **NF-001** | **Wall-clock budget: the full gate, twice.** The `e2e` command gate fires on both entry to and exit from the stage (`.redkiln/config.yaml:50-54`), and the full gate includes two feature powersets, `cargo deny` and a nightly rustdoc build. Plan for both firings; the cost is the decision, not an accident of it. | `project.md` risk table, row 1. |
| **NF-002** | **Reproducibility.** Every dependency-resolving invocation in the gate passes `--locked` (`xtask/src/main.rs:52-54`), so the run tested the committed dependency graph. The record does not alter `Cargo.lock`; if the gate would, that is EC-001, not a fix. | `xtask/src/main.rs` module doc. |
| **NF-003** | **Transcript fidelity.** stdout **and** stderr are captured together, in order, unmodified — no filtering, no re-wrapping, no ANSI stripping that removes text. The `skipped:` and `=== … ===` lines are the record's primary sources, and a lossy capture destroys AC-003 and AC-005 silently. On this Windows host, capture must not mangle UTF-8. | `xtask/src/main.rs:864`, `:876`; the steps inherit stdio (`Command::status`, `:880-884`). |
| **NF-004** | **Zero production delta.** The story's diff contains no `.rs`, `.toml`, `.yml` or `spec/` change. This is measured, not asserted: it is AC-005's boundary check and the mechanism behind `project.md` AC-013. | `project.md` *Out of scope*; `_storymap.md` *Grain notes*. |
| **NF-005** | **Legibility to an absent reader.** The record is written so that someone who was not present, cannot run the suite, and has one sitting to spend can answer "did it pass, on what tree, and what did it not cover" without opening a second repository. | `../_decomposition.md` *The evaluator-persona decision* (amended); `../../_discovery/distillation/personas-and-journeys.md:333-340`. |
| **NF-006** | **Claim width.** No sentence in the record generalises the local run into a consumer-compiler or CI claim. The MSRV floor is CI's `msrv` job (`.github/workflows/ci.yml:241`), and the local gate does not check it. | `CLAUDE.md` *Commands*; `.kb/decisions/0029-msrv-raised-to-1-97-1.md`. |

## Implementation notes (non-prescriptive)

These are the seams an implementer will hit; none is binding beyond the ACs.

- **Order of operations.** Enter the slice-mate's checkout → capture `git rev-parse HEAD`,
  `git status --porcelain`, `rustc -Vv` and the `rust-toolchain.toml` pin → run
  `cargo xtask ci` capturing combined output to a file → write `gate-run.md` from the capture →
  mount the **Gate run** section into `_closeout-record.md`. Capturing the tree facts *before*
  the run rather than after is what makes AC-002 a precondition rather than a post-hoc guess.
- **Deriving the 24 step names.** They are the `name:` fields of `REQUIRED`
  (`xtask/src/main.rs:105`) and `OPTIONAL` (`:535`). Read them from the source at run time rather
  than from this spec, and reconcile against the transcript's `=== … ===` banners — the spec's
  count is a check, not the authority. If they disagree, that is EC-004.
- **The four `wasm32` names** are literal strings in `wasm32_steps` (`:786-789`). Copy them; do
  not re-title them to something tidier, because the by-name match is the whole point.
- **Capturing on Windows.** The steps inherit stdio and print progress; a capture that swallows
  the console breaks NF-003. Tee rather than redirect, and check the file for the
  `=== formatting ===` banner before trusting it.
- **`cargo xtask ci` is invoked, never edited.** If a step looks wrong, that is a finding
  (EC-004). The gate is the instrument; an instrument adjusted to produce a reading is no longer
  one (`_storymap.md` *Grain notes*).
- **Do not pre-empt the neighbours.** The fourteen-scenario DoD table is HS-S0127's and the
  published-tree delta is `published-tree-delta-statement`'s. This story supplies the run they
  cite; writing their tables here duplicates evidence in two places, which is the exact failure
  `_storymap.md` *Where the evidence lands* names.
- **Ledger and provenance.** `.redkiln/config.yaml:67` requires the `_ledger.md` rows to carry
  real cited evidence, and `:73` requires a work commit recorded via `redkiln record-links --sha`
  because this story does change files inside its own boundary.

## Tests and CI (merge gate)

Grounded in the testing brief's tier assignments (`../_decomposition.md` *Acceptance Criteria*,
AC-001 row → E2E, AC-002 row → Integration-inside-E2E) and its *Merge-gate commands*. This story
writes **no new automated test**: its tiers are a run, and static/process checks over what the run
produced (`../_decomposition.md` *Test mix, summarised by tier*).

| Tier | Command / path | Proves |
| --- | --- | --- |
| **E2E** | `cargo xtask ci`, run whole from the clean checkout's root (`.redkiln/config.yaml:60`; `xtask/src/main.rs:828`) | AC-001 — the release bar, not the sibling bar, ran to completion and produced an observed exit code. This is the project's only `e2e` grain and its Definition of Done. |
| **Integration (inside that E2E run)** | The four `wasm32` steps of the same invocation (`xtask/src/main.rs:769-793`, `:203-283`) | AC-004 — BR-12 / ADR-0001's `!Send` flavour is exercised on the *assembled* tree. Build-and-check, not build-only, which is what makes them integration rather than static. |
| **Unit / Integration (inherited)** | The gate's `tests` step: `cargo test --workspace --all-features` (`xtask/src/main.rs:143`) | Every crate's existing suite — including the `memory.rs` pair `CLAUDE.md` binding constraint 3 names and every adapter's `event_store_conformance!` invocation — re-run on the assembled tree rather than trusted from each sibling's own run. Feeds AC-003's step row; adds nothing. |
| **Static (inside the gate)** | `cargo xtask spec-trace`, the five file-reading lints, `documentation (no default features)`, `packaged artifacts carry their licences and README` (`xtask/src/main.rs:315`, `:342-447`, `:502`, `:519`) | AC-003 — the steps that check binding constraint 2 / ADR-0003 and the specification's own cross-references are present in the record by name, so "green" is enumerable rather than atmospheric. |
| **Static (over the record)** | `rg -n "^\| " …/gate-run.md` row count vs. the `name:` fields at `xtask/src/main.rs:105` and `:535`; `rg -n "wasm32 (build\|check)" …/gate-run.md`; `rg -n "skipped:" <capture>` | AC-003, AC-004, AC-005 — 24 rows, four named `wasm32` rows, one verbatim quoted line per skip. |
| **Static (over the mount)** | `rg -n "gate-run.md" .bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md`; SHA equality between the two files | AC-002, AC-006 — the evidence is mounted and reachable, and the two artefacts describe the same tree. |
| **Process** | `git diff --stat <base>..HEAD` restricted to the *PR boundary* globs; `redkiln verify --grain story` (`.redkiln/config.yaml:62-73`) | AC-005's boundary clause and NF-004 — zero paths under `crates/`, `xtask/`, `spec/`, `.github/`, `.kb/`; ledger rows carry real evidence; the work commit is recorded. |
| **Process (project bar, not this story's)** | `redkiln validate`, `redkiln validate --kb`, `redkiln doctor --json` (`../_decomposition.md` *Merge-gate commands*) | Named for completeness: these are `backlog-and-kb-health-at-closeout`'s deliverable (AC-011/AC-012), not this story's, and are listed so an implementer does not adopt them here. |

## Risks and coupling (PR-scoped)

| Risk / coupling | Note |
| --- | --- |
| **The gate fires twice and takes a long time** | `.redkiln/config.yaml:50-54`: a command gate runs on stage entry *and* exit, and the `e2e` grain is the whole thing. The mitigation is scheduling, not scope: substituting `--fast` fails AC-001 outright (`project.md` DR-2, risk row 1). |
| **Temptation to repair a red step** | This is the first tree where a cross-project interaction is visible, which makes it the run most likely to find something and the story most tempted to absorb it. EC-001 and AC-005's boundary clause are the mechanism; `findings-disposition-register` (HS-S0134) is the destination (`project.md` risk table; DR-12). |
| **Hard dependency on the slice-mate** | `blocked_by: HS-S0125`. The checkout *and* the `_closeout-record.md` section shape both come from `clean-checkout-harness`; neither is re-decidable here (EC-007). They are implemented in one context precisely because "make a clean checkout" and "run the gate in it" are not separable observations (`_storymap.md` *Why the slices are cut here*). |
| **Two downstream readers** | `blocks: HS-S0127, HS-S0134`. `dod-set-re-observation-record` reads the step table as its evidence for the DoD scenarios the gate itself re-proves; `findings-disposition-register` reads the findings rows. A record that is complete but unmounted blocks both — AC-006. |
| **The step set can drift under the spec** | The 24 names are read from `xtask/src/main.rs` at planning time. If a sibling project changed the gate, EC-004 fires. That is a feature of the by-name table, and the correct response is a finding, never a quiet re-count. |
| **A skipped optional step narrows the claim invisibly** | `cargo hack` and `cargo deny` are exactly the steps `--fast` drops, so a skip here reproduces the sibling bar while the record says "whole gate". EC-002 makes it a finding (`CLAUDE.md` *Commands*). |
| **The record can overclaim without lying** | "The gate is green" reads to an evaluator as "this compiles for me". NF-006 and AC-006 force the host, the toolchain pin and the MSRV attribution into the record so the width of the claim is visible (`CLAUDE.md` *Commands*, MSRV paragraph). |
| **Windows capture fidelity** | The gate is developed on Windows (`xtask/src/main.rs:85-87` says so about `RUSTDOCFLAGS`), and a mangled or truncated capture silently destroys AC-003/AC-005. NF-003 makes fidelity a requirement rather than an assumption. |

## Dependencies

**Blocks on** (this story cannot start until these land):

- `clean-checkout-harness` (HS-S0125, `foundation`, same slice `assembled-tree-gate`) — supplies
  the residue-free checkout with no path dependency standing in for a published version (DR-1),
  and stands up `_closeout-record.md` with the section shape this story mounts into. Matches this
  story's `blocked_by: [HS-S0125]`. There is no other blocking edge.

**Unlocks** (these read this story's mounted evidence):

- `dod-set-re-observation-record` (HS-S0127) — cites the run for every DoD scenario the gate
  itself re-proves.
- `findings-disposition-register` (HS-S0134) — routes the findings rows this run produces.

Both appear in this story's `blocks` frontmatter and in `_storymap.md`'s dependency column.
Nothing in the `answers-audit` or `durable-audience` slices depends on this story, and it depends
on neither of them.

## Anchors (progressive disclosure)

Linked, not pasted. Each is deferred depth behind the Context pack, with the moment it becomes
load-bearing.

| Anchor (real path) | Why it is load-bearing | When to open it | Serves |
| --- | --- | --- | --- |
| `xtask/src/main.rs` | The gate, defined once. `REQUIRED` at `:105`, `OPTIONAL` at `:535`, `wasm32_steps` at `:769-793`, `run_ci` at `:828`, `run_fast`'s sentinel string at `:853-860`, `run_steps`'s banner at `:864` and skip line at `:876`. The 24 step names and the four `wasm32` names are read from here, not from this spec. | Before writing the step table — and again if any transcript banner does not match (EC-004). Never to edit. | AC-001, AC-003, AC-004, AC-005 |
| `.redkiln/config.yaml` | `:55` is `integration_scoped: cargo xtask ci --fast` (the sibling bar); `:60` is `e2e: cargo xtask ci` (this project's terminal grain); `:50-54` explains the both-sides firing; `:67`/`:73` are `require_ledger` and `require_commit_provenance`; `:5` is `support_initiative`, where findings route. | Before running anything, to confirm the exact command string; again before the ledger and the work commit. | AC-001, AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` | The charter this story serves: AC-001 and AC-002 verbatim, DR-1 (clean checkout), DR-2 (not `--fast`), DR-5 (`!Send` survived), DR-12 (findings routed), and the risk table's first and third rows. | When judging whether a record is sufficient, and whenever tempted to fix something. | AC-001…AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` | The one warranted brief (`testing`): the AC-001 row assigning tier **E2E**, the AC-002 row assigning **Integration inside the E2E run**, *Merge-gate commands*, *Fixtures / seams* ("the clean checkout… is the thing AC-001 is actually checking for"), and the amended evaluator-persona decision that fixes who the record is written for. | Before writing the *Tests and CI* evidence and before choosing the record's voice. | AC-001, AC-004, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md` | *Where the evidence lands* (one `_closeout-record.md`, not fourteen scattered ledger entries), *Why the slices are cut here* (why the checkout and the run are one slice), and *Grain notes* (no story fixes anything; no story substitutes `--fast`). | Before deciding where to write the record, and before treating the slice-mate as a setup step. | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/clean-checkout-harness/spec.md` | The slice-mate's own spec: the checkout's residue guarantees and the `_closeout-record.md` section shape this story mounts into. This story follows that shape rather than negotiating a second one. | Immediately before mounting the **Gate run** section — and at story start, to confirm the dependency actually landed (EC-007). | AC-002, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_grounding.md` | *Existing code / process patterns this project must follow* — what `cargo xtask ci` proves and in what order — and *Tensions / open items to flag, not silently resolve*, which is the standing instruction behind EC-001's "record, do not repair". | When a step's purpose is unclear from its name, and before classifying anything as incidental. | AC-003, AC-005 |
| `.kb/decisions/0001-async-port-flavours.md` | The accepted decision the four `wasm32` steps stand guard on: ports defined once without a `Send` bound, `trait_variant` deriving the `Send` flavour, `#[async_trait]` forbidden because it makes `wasm32` impossible. It is why AC-004 names the steps instead of counting them. | Before writing the `wasm32` rows, so the record says what their green *means*. | AC-004 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | Why the MSRV is 1.97.1, why it now equals the `rust-toolchain.toml` pin, and why the `msrv` CI job is kept although it currently proves nothing. The record's claim-width sentences depend on getting this right. | When writing AC-006's host/toolchain/MSRV sentences. | AC-006 |
| `rust-toolchain.toml` | The pinned toolchain that actually ran, which the record must name rather than paraphrase as "stable". | While capturing the tree facts, before the run. | AC-002, AC-006 |
| `.github/workflows/ci.yml` | `:88` runs the same `cargo xtask ci`; `:241` is the dedicated `msrv` job the local gate does not replace. Confirms that CI installs every optional tool, which is why a local `skipped` is a *local* narrowing (EC-002). | When classifying a skip, and when bounding the claim. | AC-005, AC-006 |
| `CLAUDE.md` | *Commands*: `cargo-hack` and `cargo-deny` both resolve on this machine, so a green local gate proves more than it used to — and the MSRV paragraph. Binding constraint 1 for the `wasm32` steps. The `adopt --templates` prohibition (not this story's, but a standing trap). | Before deciding whether a skip is expected or a finding. | AC-004, AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/initiative.md` | DoD 13 at `:396-397` — "the gate is green on the assembled whole… including the specification cross-reference step" — the scenario this story advances, and BR-12, which AC-004 carries to closeout. | When writing the record's opening sentence, so it answers the charter's phrasing. | AC-001, AC-004 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | *Persona 4 — The evaluator (pre-adoption)* (`:249-283`) and *Cross-persona tensions* (`:333-340`): time-boxed, one-shot, cannot run the suite. The concrete reader NF-005 and AC-006 are written for. | Before writing prose in the record; not needed to run the gate. | AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` | The signed-off design: **no public API surface**, `## Items` `N/A`, and the recorded reason `design.capture` is a declared skip. It is what makes the *Interaction quality* surface family N/A here rather than forgotten. | Only if someone proposes that this story renders or changes a surface. | AC-006 |

## Clarifications resolved during spec

1. **The AC set is exactly the six the first pass enumerated.** None added, none dropped.
   AC-001…AC-006 split `project.md` AC-001 across four criteria (whole-not-fast, SHA binding, step
   enumeration, skip accounting) and give `project.md` AC-002 its own (AC-004), with AC-006
   carrying reachability for both. The `_ledger.md` rows match these six exactly.
2. **"Every step that printed `skipped` is listed with the absent tool" is only checkable if the
   steps that did *not* skip are listed too.** `project.md` AC-001 asks for the skip list; a skip
   list alone cannot be distinguished from a run in which the step never existed. AC-003 therefore
   requires all 24 steps by name, which is stronger than the charter asks and is the minimum that
   makes the charter's own requirement verifiable.
3. **A skip is scoped as a *local* narrowing, not a gate defect.** CI installs every optional tool
   (`.github/workflows/ci.yml`; `xtask/src/main.rs:46-50`), so a local skip narrows what *this
   run* proved without implying the gate is broken. EC-002 and EC-003 split accordingly:
   `cargo hack` / `cargo deny` skipping is a finding because `CLAUDE.md` says they resolve here;
   nightly skipping is an expected absence, stated rather than routed.
4. **A red gate completes this story rather than failing it.** `project.md` DR-12 and AC-013 make
   routing the deliverable, so AC-001 requires an *observed exit code*, not a zero. The story
   fails on a repair, a `--fast` substitution or an unaccounted skip — never on a finding. The
   ledger rows are written so this stays true: AC-001's evidence is the exit code as observed.
5. **The record's shape is inherited, not negotiated.** `clean-checkout-harness`'s `spec.md` is
   still at its stage stub, so the `_closeout-record.md` section layout is not yet written down.
   This spec therefore binds the *content* of the **Gate run** section (SHA, exit code, 24 rows,
   four named `wasm32` rows, verbatim skip lines, claim-width sentences) and explicitly defers its
   *layout* to the slice-mate, per `_storymap.md` *Where the evidence lands*. If the slice-mate
   lands a different layout, this story follows it; if it lands none, that is EC-007.
6. **`_design.md` binds by declaring nothing.** It is a signed-off design that records a
   no-surface determination. The *Interaction quality* section therefore states the render family
   as N/A **with the citation**, and applies the state/composition families to the record itself —
   with a real density budget (24 / 4 / one-quote-per-skip) rather than an omitted section.
7. **The `wasm32` step names are literals, not descriptions.** `xtask/src/main.rs:771-781`
   documents that selecting these steps by index was a real defect and that by-name selection
   replaced it. The record inherits that discipline: AC-004 fails on any re-titling, however
   tidier.
8. **The 24-step count is a check, not the authority.** Read from `xtask/src/main.rs` on
   2026-08-12 (20 `REQUIRED` + 4 `OPTIONAL`). If the gate has changed by implementation time, the
   source wins and the discrepancy is EC-004 — a finding against the change that moved it, not a
   silent correction to this spec.
