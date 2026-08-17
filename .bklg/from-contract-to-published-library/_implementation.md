# Implementation ledger — From Contract to Published Library

The per-project progress ledger across `/redkiln:implement` runs. Body-only
companion file: it carries no item frontmatter, and nothing here is written by
`redkiln`. The CLI remains the single writer of every item's stage and status.

- **Initiative:** HS-I0006 · `from-contract-to-published-library`
- **Branch:** `initiative/from-contract-to-published-library`
- **Worktree:** `.claude/worktrees/from-contract-to-published-library`
- **Terminal (DoD-owning) project:** HS-P0019 `closeout-and-durable-audience`
  (`dodOwner: true` in the item index; the only project wired to `verify.e2e`)

`entry_baseline: green @ ae77ac4a1a94fdedb4f4a248210140c86e20a16a`

Observed 2026-08-13, before any mutation: `cargo xtask ci` — the **full** bar, not
`--fast` — exited 0 with `all checks passed` on a clean tree at that commit. Later
runs skip the full-suite step on the strength of that line; per-project runs are
affected-scoped inside the workflow, so anything red after this point is
attributable to the initiative rather than inherited.

## Projects, in merge order

| # | Project | Id | Depends on | State | Verdict / blocker |
|---|---------|----|------------|-------|-------------------|
| 1 | `projection-store-freeze` | HS-P0010 | — | **done** | approved 2026-08-15 · `_review.md` · 17/17 · 6 runs |
| 2 | `typed-layer-and-alpha-release` | HS-P0011 | 1 | **done** | approved 2026-08-16 · `_review.md` · 16/16 · 3 runs · `overall: 3` |
| 3 | `sqlite-durable-store` | HS-P0012 | 1, 2 | **in-progress** | run 2 halted at slice 2 review · 7/14 committed · baseRef `90cbca5` |
| 4 | `cloudflare-durable-object-store` | HS-P0013 | — | pending | |
| 5 | `postgres-and-neon-stores` | HS-P0014 | 1 | pending | |
| 6 | `ladybug-projection-store` | HS-P0015 | 1 | pending | |
| 7 | `publication-and-positioning` | HS-P0016 | 2, 3, 4, 5, 6 | pending | |
| 8 | `replication-identity-and-ingest` | HS-P0017 | 7 | pending | |
| 9 | `retention-and-incomplete-logs` | HS-P0018 | 8 | pending | |
| 10 | `closeout-and-durable-audience` | HS-P0019 | 9 | pending | |

Positions 4, 5 and 6 are mutually unordered; the serial trunk is
1 → 2 → 3 → 7 → 8 → 9 → 10.

## Run log

### HS-P0010 `projection-store-freeze` — run 1, 2026-08-13 (`wf_c27a4fc1-63c`)

baseRef `2136ddeb`. Halted at `stories`, verdict `changes-requested`, 3 of 17 stories
committed. **Keep baseRef stable across re-launches of this project.**

| Story | Commit | State |
|-------|--------|-------|
| `ps-clause-pairing-sweep` | `f77f183` | complete — 37 clauses verdicted, ISOLATED against a threshold declared before the count |
| `projection-decision-atoms` | `9520b28` | **blocked** — 8 of 10 ACs need three *accepted* `.kb/decisions/` atoms |
| `projection-api-design-record` | `0df2c1c` | complete — DT-3 and DT-8 resolved in `_design.md` |

**The blocker is a correct refusal, not a failure.** The three ADR atoms may only be
authored by a human-invoked `/redkiln:kb-ingest` wave: CLAUDE.md reserves atom
authorship to that wave (hand-writing them was reverted at `0269720`), and this
story's own AC-007 requires the atoms to *arrive as the output of one such wave* —
so hand-writing them would fail AC-007 in the act of appearing to satisfy AC-001–003.
Its spec's implementation notes say the same thing directly: *"Do not run
`/redkiln:kb-ingest` from inside the implementation."*

Everything the wave needs is staged and committed: three long-form records under
`references/adr/` (0017, 0018, 0019) and three `.kb/_intake/2026-08-13-adr-00{17,18,19}-*.md`
documents carrying the frontmatter conventions, the claims, the rejected alternatives,
the provisional halves with falsifiers, the resolve-not-delete protocol for
`kb-open-question-projection-batch-no-apply-001`, and the `.kb/maps/decision-map.md` rows.
Suggested wave id: **`2026-08-13-projection-adrs`** (`2026-08-10-intake` and `-2` are taken).

The slice `decisions-and-design-record` was **never sealed** — `_slices.md` has no row,
because the run halted before the adversarial slice review. So no story here has been
reviewed by anything, and none was advanced past `plan`.

AC-010's ordering guarantee is not at risk: `crates/happenstance-core/src/projection.rs`
is byte-identical to `main`, and `owned-batch-port-shape` is blocked on this story, so
the port cannot move ahead of the decisions.

**Degraded: 3 × fatal**, all one kind — the isolation audit caught each of the three
story agents writing an ephemeral AC-check `.sh` into the session scratchpad, outside
the worktree. No declared artifact is missing (`missingArtifacts` is empty; all 22
declared files are on disk). It is a real isolation trip and is recorded as one, but it
did not cost a deliverable.

### Blocker cleared — wave `2026-08-13-projection-adrs`, 2026-08-13

The `/redkiln:kb-ingest` wave run 1 was waiting on is merged at `d05d2b3`. Three
decision atoms now exist at `status: accepted` — `kb-decision-0017`, `-0018`, `-0019`
— plus three amended open questions, three synced maps and six reciprocal backlinks.
`.kb/_intake/` holds only `README.md`. `redkiln validate --kb` passes.

The wave was branched from the **initiative branch** rather than `main`, against the
command's stated default. That put the three long-form `references/adr/` records in the
wave worktree so the atoms' `source_paths` resolved, and made the return trip one merge
instead of two — `main` stays untouched until closeout, which is closeout's job.

Two things the wave left open, neither blocking:

- **A superseding atom correcting one sentence of ADR-0007's Context (PS-32) is owed
  and was not written.** ADR-0017's record says the correction belongs to a superseding
  atom rather than an edit; the wave honoured both halves and refused to author it,
  because doing so would be an ingest taking a decision nobody signed. It is a human's
  to place — now, or at HS-P0011.
- **`redkiln doctor` exits 1** on nine `.bklg` story-wiring errors of the form
  *"foundation story `HS-S####` is consumed by no capability slice"* — `HS-S0002`,
  `-0034`, `-0035`, `-0067`, `-0074`, `-0075`, `-0100`, `-0108`, `-0120`. Confirmed
  pre-existing twice: once by the workflow at bare `HEAD`, once independently in a
  second worktree at the same commit carrying none of the wave's changes. It is
  planning-artifact debt in this initiative, unrelated to `.kb/`, and it blocked the
  wave's own commit gate — the wave was finished by hand under an explicit decision to
  treat it as out of scope. **Unrouted; it needs an owner.**

### HS-P0010 `projection-store-freeze` — run 2, 2026-08-13

Re-launched fresh at the **same baseRef `2136ddeb`** — not `resumeFromRunId`, no
cache-buster. Preflight reads both resume axes from git: the three `Story:`-trailered
commits are skipped, and `decisions-and-design-record` is committed but sealed by no
`Slice-Verdict` row, so the run re-enters it at Review rather than dispatching an
implementer. The full-suite gate is skipped against `entry_baseline`;
`cargo xtask affected --base main` was green beforehand ("no package affected" — the
merge touched only `.kb/`, `.bklg/` and `references/`).

**Result.** 6 of 17 stories committed, `degradedSummary: none` (run 1's three isolation
trips did not recur), no `baselineRepairs`. Halted at the **review** step of slice
`projection-port-and-probe`, verdict `changes-requested`. No project transition — HS-P0010
stays at `implementation`; `_integration.md` and `_review.md` were never reached.

| Slice | Verdict | Sealed by | Stories |
|-------|---------|-----------|---------|
| `decisions-and-design-record` | **approved** | `560bb4b` | `f77f183` · `9520b28` · `0df2c1c` |
| `projection-port-and-probe` | **changes-requested** | `0ccf16e` | `2eade38` · `cb495ee` · `5fd62c6` |

Slice 1 re-entered at Review with no implementer dispatched, exactly as the two-axis resume
model intends, took one reconcile commit (`fb4161c`) and sealed approved.

**Story verdicts recorded** (human, batched):

| Story | Stage now | Verdict |
|-------|-----------|---------|
| `ps-clause-pairing-sweep` HS-S0001 | `report` / `in-review` | approved |
| `projection-decision-atoms` HS-S0002 | `report` / `in-review` | approved |
| `projection-api-design-record` HS-S0003 | `report` / `in-review` | approved |
| `owned-batch-port-shape` HS-S0004 | **`plan` / `ready`** | **none — gate red, see below** |
| `projection-probe-conformance-feature` HS-S0005 | `implement` / `in-progress` | changes-requested |
| `memory-projection-store` HS-S0006 | `implement` / `in-progress` | changes-requested |

### Three findings carried into run 3

1. **`crates/happenstance-core/src/projection.rs:495-499` publishes a `[dev-dependencies]`
   block that contradicts the paragraph three lines above it.** `:490-491` states "the only
   way out is a **non-dev** dependency on the testkit, feature-gated"; the snippet then shows
   `[dev-dependencies]`. It cannot work — a dev-dependency does not make `ProjectionProbe`
   exist for the lib build, the impl must live in the adapter's `src/` because the orphan rule
   rejects it in `tests/`, and Rust cannot `#[cfg]` on a dependency's feature. The spec
   anticipated it: **AC-002 reads "`[dependencies]` (not `[dev-dependencies]`)"**. The
   `_ledger.md` records the wrong form as *satisfying* evidence for AC-006.
2. **`projection.rs:504-507` is self-refuting.** "The falsifier is the story that builds an
   outside author's fixture from the documentation alone, **and it is named here**" — no name
   follows. AC-006 requires naming `documented-extension-surface`. The ledger claims it met.
3. **`owned-batch-port-shape`'s checkpoint `2eade38` writes outside its declared boundary** —
   `crates/happenstance-ladybug/src/stand_in.rs`. Found by the story's own deterministic gate,
   not by the reviewer. This is why HS-S0004 sits at `plan`: `advance --to report` runs
   `implement`'s command gate on the way out, the gate is red, and the story never reaches
   `report`, so **no rejection could be recorded against it** — it is held where it started.
   Either the boundary in `spec.md` is too narrow for work the story legitimately owns, or the
   implementer reached into the ladybug skeleton it should not have. Run 3 must settle which.

Findings 1 and 2 are documentation defects in the crate's **public** surface, aimed at the
adapter-author persona the project exists to serve; both were recorded as *passing* ACs, which
is the part that matters more than either defect.

### What run 2 got right, worth keeping

The slice's self-heal commit `cc0f158` found a real gate blind spot: `cargo doc -p
happenstance-core` on the **default** feature set was a hard error, because
`projection_memory.rs` linked `ProjectionProbe::READS_THROUGH_BATCH` — a `conformance` item —
from a page that renders under `memory`. `--all-features` resolved the link and
`--no-default-features` never rendered it, so **both existing doc steps were green over a
broken consumer build**. It added a third gate step (`documentation (default features)`),
recorded it in `CLAUDE.md:276-283`, and re-pointed nineteen constitution citations plus five
`SPECIFICATION.md` ones after ADR-0017's `live_handle.rs` move — retiring two rules at their
own stated PS-5 trigger rather than by fresh editorial judgement.

### Orchestrator error, corrected

`record-links` was given the slice-wide commits (`fb4161c`, `cc0f158`) against every story in
their slice, not just the stories that own those files. `fb4161c` carries
`references/evaluation/**` (`ps-clause-pairing-sweep`'s mount point) and `cc0f158` carries
`standards/rust/**` and `CLAUDE.md`, so the boundary check — which reads `links.commits` —
failed HS-S0002 correctly. Removed with `record-links --remove` from HS-S0002, HS-S0003,
HS-S0004, HS-S0005 and HS-S0006; each story now records only its own checkpoint, which is what
`_slices.md` attributes. **Rule for later runs: `links.commits` is boundary-checked, so a
slice-wide fix commit belongs only on the stories whose declared boundary covers its files.**

### HS-P0010 `projection-store-freeze` — run 3, 2026-08-14 (`wf_2f6669c4-fd8`)

baseRef `2136ddeb` (unchanged). `degradedSummary: none`, no `baselineRepairs`.
**10 of 17 stories committed; three slices sealed `approved`.** Halted mid-slice 4 at
`commit-rollback-and-drop-rules`, `blocked-dependency`.

| Slice | Verdict |
|-------|---------|
| `decisions-and-design-record` | approved |
| `projection-port-and-probe` | **approved** — was `changes-requested`; flipped by `b7c1600` |
| `projection-conformance-suite` | approved |
| `commit-atomicity-and-mutants` | **unsealed** — `c385e40`, `5d9b4fd` committed, review never ran |

The re-review found **six** findings, not the three carried in. Both rustdoc defects fixed — the
snippet is `[dependencies]` with a forwarding feature and a comment *refuting* the wrong form
rather than merely omitting it; the falsifier names `documented-extension-surface`. The false
AC-006 ledger row was **withdrawn with both wrong descriptions quoted**, not silently overwritten.
Two nobody had flagged: four adapter `//!` module docs still described the deleted GAT port in the
present tense while the non-rendered `//` comments beside them had been updated, and the `E0195`
narrative was duplicated across two rendered pages.

Finding 3 (the boundary) was settled the right way — a deliberate widening with the reason in the
spec, since `broken_intra_doc_links` at `deny` made the edit compelled. It also went further than
asked and ratified `cc0f158`'s own widening in `_slices.md`, observing that *"a commit message
asserting its own authorisation was the only record"*.

**Eight stories now approved** (HS-S0001–HS-S0008), all at `report`/`in-review`.

### The boundary problem is structural, not three accidents

`projection-suite-entry-point` and `projection-capability-skips` both failed their own boundary
check on `standards/rust/**` — eleven and eight atoms — **after their slice had already sealed
`approved`**. Every hunk was line-number re-pointing (`lib.rs:370` → `lib.rs:384`), 23/23 and
17/17 insertions to deletions. Compelled work: the constitution cites the testkit sources these
stories own, editing them moves the cited lines, and `lint-constitution` is a gate step.

Both boundaries were widened at `aef8990` with the argument stated and **scoped to line-number
repair only** — rule text, evidence selection, retirement and new atoms stay out, so the widening
cannot be reused to justify editing a rule. Third instance of this shape.

Two observations that outlive this project:

- **This repo has two deny-level cross-reference checkers** (`broken_intra_doc_links`,
  `lint-constitution`) whose targets sit outside any story's natural boundary. Any story touching
  a cited file is forced across its own boundary or into a red gate. Expect it again.
- **The slice reviewer has now sealed `approved` twice over work the deterministic per-story gate
  rejects.** The two instruments disagree in a consistent direction, and the slice review is the
  one that does not run `verify --grain story`. Worth raising where the loop is maintained.
- **A sealed-`approved` slice is skipped by a re-launch**, so a defect found in one after sealing
  cannot fix itself. That is why these two were settled by hand rather than deferred to run 4.

### DT-3 — human decision recorded

Amend DT-3 and **add the commit-fault capability**, rather than record PS-1's second conjunct as
unenforceable. Written into `_slices.md` for run 4 with what it obliges: amend the `_design.md`
enumeration in place with date and reason, grow `contract.rs`'s set to match, and write a **mutant
that fails the new rule and passes the others** — without it the rule is the decorative rule
ADR-0010 exists to make unwriteable, which was escape (2)'s defect and is not cured by arriving via
a capability. PS-1's second conjunct is unenforced by any rule until this lands.

### HS-P0010 `projection-store-freeze` — runs 4 and 5, 2026-08-14/15

**Run 4** (`wf_e3e04418-77c`, `degradedSummary: none`) — 12 of 17 committed. `commit-atomicity-and-mutants`
sealed **approved**: the DT-3 amendment landed with `COMMIT_FAULT` *required* rather than defaulted,
so no testkit-written reason had to be minted, and `PartialCommitStore` fails
`failed_commit_leaves_both_unchanged` and nothing else — proven by deleting its one line and watching
the exactness meta-test go red by name. **PS-1's second conjunct is no longer a hole.**
`reset-and-rebuild-rules` sealed **changes-requested** on a correctness finding, not a procedural one.

**Run 5** (`wf_e8627580-c45`, `resumeSlice: second-batch-shape-and-evidence`, `degradedSummary: none`)
— the last three slices sealed approved, project verdict **approved**, `dodGreen`, `reachabilityOk`,
`missingArtifacts: []`, `uncoveredAcs: []`. Rubric: ac-coverage / integration-reachability /
test-integrity / gate-greenness **3**; brief-fidelity / intent-fidelity **2**; presentation-fidelity
**0 — does not apply** (no `design.capture`; a library has no visual surface).

`resumeSlice` was used deliberately: slice 5 is sealed `changes-requested` and no amount of work
inside this project can seal it, so a normal re-launch would have re-entered it at Review and halted
a third time without reaching slices 6–8.

**15 of 17 stories approved** (HS-S0001–HS-S0010, HS-S0013–HS-S0017). HS-S0011 and HS-S0012 remain at
`plan`/`ready`. **The project was NOT advanced** — see the decision below.

### The PS-19 finding was the most valuable thing this project produced

`fresh_projection_has_no_checkpoint` was withdrawn because it **convicted a conformant store**: a
projection whose missing row resolves as `Live { through: FIRST }` breaks no frozen MUST, yet
`PresumedLiveCheckpointStore` was registered as a *mutant*. CF-5's positive control could not catch
it, because no conformant variant in the registry modelled that store. In a project whose product is
a suite that can fail, a rule that fails conformant stores is the more expensive of the two errors —
and it was found by refusing to satisfy an AC rather than by any check.

Path (a) was chosen and path (b) explicitly refused, so there is no authorisation to cite. The repair
is staged as **ADR-0030** (`.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md`, long form at
`references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md`), written but **not
accepted** — acceptance is one human-invoked `/redkiln:kb-ingest` wave.

**Standing suggestion for run 6:** add a conformant variant modelling the legal
`.unwrap_or(Checkpoint::Live { through: FIRST })` store to the projection registry, so the positive
control can catch the next rule that over-convicts.

### Open, disclosed by the project review rather than hidden

1. Slice 5 unsealed; the absence is machine-visible at `spec/SPECIFICATION.md:8833`.
2. Four doc sites still say the reference run produces **one** skip; `assert_reference_projection_declensions`
   pins **two**. Deliberately not fixed by run 5 — they live in the skipped slice's files, and repairing
   them from a slice-6 commit would put slice-5 work under a slice-6 boundary.
3. **`crates/happenstance-testkit/README.md:18,:127` — shipped by `cargo package` to crates.io** —
   still says the suite "is two rules of seventeen, and neither has been shown to reject a wrong store
   yet" and the hostile stores are "not yet written". All false now, in the *understating* direction.
   Routed to `publication-and-positioning` (HS-P0016), which owns initiative DoD 10.
4. `_design.md` shows the outside author under `[dev-dependencies]`; the landed manifest correctly uses
   `[dependencies]`. The design record is superseded in fact by the shipped surface.
5. **CI's `backlog` job asserts the `unconsumed-foundation` list is empty**, and it is not — nine
   problems, verified pre-existing from planning commit `ae77ac4`. **This makes upstream issue #122 a
   release blocker for the initiative, not tidiness.** It must be settled before the PR to `main`.

### Decision — wave, then run 6, then review

Advance nothing yet. Invoke `/redkiln:kb-ingest` for ADR-0030; run 6 then re-enters slice 5, lands the
seventeenth rule with its mutant, fixes the four doc sites and read-through's AC-006 record; the
project reaches review at 17/17. Closing now would ship the freeze at sixteen of seventeen with the
repair staged and never accepted, which is the outcome path (a) was chosen to avoid.

### Blocker cleared — wave `2026-08-15-adr-0030-checkpoint-progress`, 2026-08-15

Merged at `9efda45`. **`kb-decision-0030` is on the branch at `status: accepted`**, which is the
precondition `reset-rules` AC-005 has been waiting on since run 4: the seventeenth rule can now be
written against a clause that exists rather than widening a `[FROZEN]` one by test. Two open
questions (PS-1, PS-19) move to `superseded`, answered by it. `validate --kb` passes;
`cargo xtask affected --base main` green.

The decision is sharper than the "widen PS-19" framing it started as: §4 obliged `commit` to
**couple** its two writes and never to **advance** anything, so PS-1's `MUST` is satisfied by a
store that makes neither write durable. **Three shipped rules already sit over that gap**, not just
the missing one.

Two atoms arrived beyond the headline decision, each in a different layer:

- **`ps-32-adr-0007-context-correction-is-owed`** — the obligation carried unrouted since the
  2026-08-13 wave, now findable by the corpus instead of by memory. It narrows the debt: only
  `references/adr/0007-projection-runner-decodes.md:36-38` carries the defective sentence;
  `kb-decision-0007`'s **atom** never repeats it, so what is owed is a correction to the long-form
  record, not a supersession. Still not performed.
- **`spec-trace-has-suite-family-switch`** — `xtask/src/spec_trace.rs`'s per-family abstention.

**Second consecutive wave finished by hand.** `doctor` exits 1 on the same nine pre-existing
`.bklg` errors; confirmed independently again in a worktree carrying none of the wave's changes.
The 2026-08-13 retrospective predicted a gate that can establish innocence but not act on it would
be routed around every time; that came true in two days. And the severity has risen — CI's
`backlog` job asserts the list is empty, so **upstream issue #122 is now a release blocker for the
initiative**, to be settled before `publication-and-positioning`, not at the PR.

### HS-P0010 `projection-store-freeze` — run 6, 2026-08-15

Launched fresh at the same baseRef `2136ddeb`, **no `resumeSlice`** — git truth alone now points at
the right place, because `reset-and-rebuild-rules` is the only slice committed but sealed
`changes-requested`, and slices 6–8 are sealed `approved` so they are skipped. Run 5 needed the
hatch only because that slice could not be cleared by any work inside the project; that is no longer
true.

### HS-P0010 `projection-store-freeze` — run 6 and CLOSE, 2026-08-15

**`wf_8a4f9e52-01b`. 17/17 stories, all eight slices `approved`, `degradedSummary: none`,
`missingArtifacts: []`, `uncoveredAcs: []`, no baseline repairs. Project verdict `approved` by the
human at `dd559812`; the item is held at `review`/`in-review` with `--stay`, awaiting
`/redkiln:closeout`.**

Rubric: integration-reachability / test-integrity / gate-greenness **3**; ac-coverage /
brief-fidelity / intent-fidelity **2**; presentation-fidelity **0 — does not apply**. `overall: 2`,
and the frontmatter is **deliberately stale** — the body states the post-repair values would be 3
and says so at `_review.md:80-86`.

DoD bar ran for real: 17 scenarios, zero `fixme`, nothing unmounted. Both batch shapes pass (memory
17/17 with two reported skips, buffering 18/18 with none); `CheckpointOnlyStore` convicted **by
name** from inside and from outside the workspace; all 17 rules green on wasm32 under
`wasm-bindgen-test-runner`; 21 registry rows, 7/7 exactness meta-tests. The 15 whole-initiative DoD
journeys are deferred to their owning projects, correctly for a feature project.

The reviewer verified the four prior findings **against the tree rather than the report**, and
checked adversarially that the new `lint-rule-counts` step is not tautological: its counts derive
from the same parse `spec-trace` uses, it **bails** if a rule file parses to zero rules and **bails**
if no document states a count, closing both vacuous-pass routes; its first unit test is red-first
against the shipped false sentence; and its changelog edits are all inside `[Unreleased]`, so no
released history was rewritten to keep a check green.

### The boundary class, six and seven — and one that should go upstream

Both surfaced during the story gate, after the slice had sealed.

**Six — `reset-rules` (`10ace94`).** Its spec reasoned that `spec/SPECIFICATION.md` was wholly out
of boundary because *"the PS-16 – PS-20 clause rows already name these five rules, so nothing needs
writing there"*. The premise is true and the conclusion does not follow: those rows name the rules
**with a `†`**, and landing the rules is exactly what removes it. Falsified by a checkable fact, so
the sentence was corrected rather than argued around.

**Seven — `read-through-and-rebuild-rules` (`5be22ab`), and this one is a tooling trap.** Its fence
read `spec/SPECIFICATION.md   # ONLY inside the BEGIN/END GENERATED region`. **`redkiln verify
--grain story` matches fence lines literally, so a trailing `#` comment makes the entry match
nothing.** The spec appeared to admit two paths under stated limits and admitted neither; the gate
rejected the story for writing a file the fence looked like it allowed. Limits moved to prose,
unchanged in substance.

That second one is worth filing upstream beside **#122**: a boundary fence that silently ignores an
entry is worse than one that rejects it, because the author reads their own spec as permitting the
path. It fails closed here, which is the safe direction — but it fails *silently*, and the author's
stated limit is discarded with it.

### Carried into the next projects

- **redkiln #122 is a release blocker.** CI's `backlog` job (`.github/workflows/ci.yml:177`) asserts
  the `unconsumed-foundation` list is empty; nine problems persist, one this project's story and
  eight in five unstarted projects, all from planning commit `ae77ac4`. Settle before
  `publication-and-positioning`, not at the PR.
- **`crates/happenstance-testkit/README.md`** understates the suite on the page `cargo package`
  ships. Routed to HS-P0016 (owns DoD 10).
- **Eight rustdoc citations resolve ~191 lines short**, landing in §4.9's PS-32 instead of §4.11.
  Correct when authored, invalidated by this project's own spec growth. No AC covers them.
- **AC-016's execution half is CI's, not the local gate's** — the local step type-checks the wasm
  harnesses. Stated in the proof artefact rather than papered over.
- **PS-32's correction to ADR-0007's Context is still owed** — now a findable atom
  (`ps-32-adr-0007-context-correction-is-owed`), and narrowed: only the long-form record carries the
  defective sentence, not `kb-decision-0007`'s atom.

### HS-P0011 `typed-layer-and-alpha-release` — run 1, 2026-08-15 (`wf_72b8772d-b46`)

baseRef **`74135b8`** — the telemetry commit, captured after the affected gate passed at that sha
(227 tests, exit 0). The full-suite step was skipped against `entry_baseline`. HS-P0011's design
verdict was already `approved` from 2026-08-12, so `advance --to implementation` walked cleanly
(`1b09e54`). **Keep baseRef stable across re-launches of this project.**

`degradedSummary: none`, no `baselineRepairs`, `missingArtifacts` not reached. **2 of 16 stories
committed. Halted at the *implement* step of slice `decision-records`, `blocked-dependency`.**

| Story | Commit | State |
|-------|--------|-------|
| `adr-0020-fold-query-agreement` HS-S0018 | `c011143` | 7 of 8 ACs satisfied — **AC-008 blocked** |
| `adr-0021-payload-evolution-and-codec-tag` HS-S0019 | `e33dc9f` | 7 of 8 ACs satisfied — **AC-008 blocked** |

Both intake documents and both long-form records are on the branch:
`.kb/_intake/0020-fold-query-agreement.md`, `.kb/_intake/0021-payload-evolution-and-codec-tag.md`,
`references/adr/0020-fold-query-agreement.md`, `references/adr/0021-payload-evolution-and-codec-tag.md`.

**The block is AC-008 on both stories, and it is a correct refusal.** AC-008 asserts an *accepted*
atom at `.kb/decisions/0020-…` / `-0021-…`, which only a human-invoked `/redkiln:kb-ingest` wave may
author on its own worktree branch. Hand-writing it is the anti-pattern reverted at `0269720`, and
each spec instructs against it directly. Both implementers left the row `satisfied: false` with empty
evidence rather than minting the atom — which is what the ledger contract asks of an implementer who
cannot produce the wave sha (`_ledger.md:29`). Project DoD 3 is undischarged until the wave runs.

**This is the second time an M1 decision-records slice has blocked on a wave, and it is the same
shape as HS-P0010 run 1's `projection-decision-atoms`.** What changed is where it surfaces: the
planning prose of `adr-0020-…/spec.md:286` says the story "ends at staged and ready", but its own
AC-008 asserts the accepted atom, so the story cannot close on the staging alone. The prose and the
criterion disagree, and the criterion is what the gate reads. Worth noting for the three later
projects that carry foundation decision stories.

**No transitions recorded.** The slice was never sealed — `_slices.md` has no row, because the run
halted before the adversarial slice review, so **no reviewer has looked at either story**. Advancing
them to `report` would put a human gate over unreviewed work with a known-unsatisfied AC on each.
HS-P0011 stays at `implementation`/`implementing`; both stories stay at `plan`/`ready`.

`links.commits` was recorded by the run itself — `c011143` on HS-S0018 and `e33dc9f` on HS-S0019,
each story carrying only its own checkpoint, which is the rule run 2 of HS-P0010 established.

**Next:** invoke `/redkiln:kb-ingest` over both intake documents in **one** wave with a suffixed id,
then re-launch the workflow fresh at the same baseRef. Git truth re-enters at slice
`decision-records`'s Review — both stories are committed but the slice is unsealed — and the fix pass
flips both AC-008 rows against the wave sha before the slice seals.

### HS-P0011 — run 2 not launched, 2026-08-15 (preflight halt)

Preflight was otherwise clean: tree clean, branch confirmed, full-suite step skipped against
`entry_baseline`, and `cargo xtask affected --base main` green at `68a5c99` (227 tests, exit 0).
**No workflow was launched and no transition was recorded**, by human decision at the entry gate.

The wave run 1 asked for **exists but is unmerged**. `.claude/worktrees/kb-intake-2026-08-15`
(branch `worktree-kb-intake-2026-08-15`) holds `.kb/decisions/0020-fold-query-agreement.md` and
`-0021-payload-evolution-and-codec-tag.md` authored at `status: accepted`, with `decision-map.md`
and `domain-map.md` freshly written — but its branch is still at `68a5c99`, identical to this one,
so **nothing is committed**, and its `.kb/_intake/` still holds both source documents rather than
having been cleared. The wave stopped short of its clear-and-commit step.

On this branch, therefore, `.kb/decisions/0020-*` and `0021-*` do not exist, and both stories'
AC-008 asserts an accepted atom at exactly those paths. A re-launch would re-enter slice
`decision-records` at Review — both stories committed, slice unsealed — and find the same two rows
`satisfied: false`, spending a run to reconfirm a block already recorded above. baseRef for the
eventual run 2 stays **`74135b8`**, unchanged.

**Precondition for run 2:** the wave commits and merges onto `initiative/from-contract-to-published-library`,
leaving `.kb/_intake/` holding only `README.md` and `redkiln validate --kb` passing.

### ROUTED TO HS-P0016 `publication-and-positioning` — the eight rustdoc citations

Carried out of HS-P0010 as *"eight rustdoc citations resolve ~191 lines short, landing in §4.9's
PS-32 instead of §4.11 … No AC covers them"*, re-verified against the tree on 2026-08-16 during
HS-P0011 run 2, and **routed here rather than fixed in passing**. Second item routed to HS-P0016,
after the `crates/happenstance-testkit/README.md` understatement.

**Still broken, and now pointing somewhere new.** The eight sites, all in `happenstance-testkit`:

| Site | Cites |
|------|-------|
| `crates/happenstance-testkit/src/projection.rs:47` | `5696-5705` |
| `crates/happenstance-testkit/src/projection.rs:899` | `5682-5687` |
| `crates/happenstance-testkit/tests/projection_mutation_coverage.rs:286` | `5696-5705` |
| `crates/happenstance-testkit/tests/projection_mutation_coverage.rs:518` | `5682-5687` |
| `.../projection_mutation_coverage/buffering.rs:6` | `5688-5693` |
| `.../projection_mutation_coverage/variants.rs:48` | `5688-5693` |
| `.../projection_mutation_coverage/mutants.rs:33` | `5680-5687` |
| `.../projection_mutation_coverage/mutants.rs:216` | `5682-5687` |

**The identification is proven, not inferred.** At baseRef `74135b8`, `spec/SPECIFICATION.md:5680-5687`
was exactly `**PS-32 — ADR-0007's Context MUST be corrected …**`, and §4.9 opened at 5679 with §4.11
at 5832 — so `5681 + 191 = 5872`, inside §4.11. HS-P0010's arithmetic checks out line for line.

**Nothing has repaired them.** The full set of 15 distinct `SPECIFICATION.md:<line>` citations across
`crates/**/*.rs` is byte-identical at `74135b8` and at HEAD. That is expected: no AC covers them.

**The drift has widened.** HS-P0011's `projection-clause-verdicts` (`55a2370`) grew the specification
from 9,262 to 9,399 lines, all above this region:

| Section | baseRef | HEAD |
|---------|---------|------|
| §4.8 Failure policy | 5521 | 5546 |
| §4.9 The runner split | 5679 | 5755 |
| §4.11 The suite this section obliges | 5832 | **5955** |

So the eight now resolve into **§4.8 Failure policy** — a *third* wrong section, neither the intended
§4.11 nor the PS-32 they were recorded against. `5680-5687` today reads *"**Rejects:** a fan-out runner
that wraps `&mut P::Batch` in `AssertUnwindSafe`…"*. The shortfall has gone from ~191 lines to ~275,
and it will keep widening every time the specification grows above §4.11.

**Why no instrument caught it, and why that is the real finding.** `spec-trace` checks citations
*within* `SPECIFICATION.md`; `lint-constitution` checks `standards/rust/` citations *into code*.
A rustdoc-into-spec citation is checked by neither. That is exactly why this run repaired eleven
constitution citations automatically (`b77cffb`, `d95c760`) and left these eight untouched — the two
classes differ only in which direction the reference points, and one has a gate. Same family as
HS-P0010's `cargo doc` default-features hole and run 2's `is_a_model_rule_body` gate: a cross-reference
class with no instrument over it.

**Why it belongs to HS-P0016.** `happenstance-testkit` is publishable, so this rustdoc ships to
docs.rs — the surface HS-P0016 owns. The eight are wrong on a page a prospective adapter author reads,
in the *understating* direction again.

**Recommended disposition, not prescribed.** A third file-reading lint extending `lint-constitution`'s
mechanism to rustdoc→spec citations would close the class rather than the instance; repairing eight
line numbers by hand leaves the next spec edit to reopen it. HS-P0016 owns the call — including the
option to record a refusal with the reason, which is a legitimate outcome for a defect no AC covers.

### HS-P0011 `typed-layer-and-alpha-release` — run 2, 2026-08-16 (`wf_a20b39d1-a5b`)

baseRef **`74135b8`**, unchanged from run 1. The ADR-0020/0021 wave was merged onto this branch
first (`3fb28a1`), which is what unblocked AC-008 on both M1 stories.

**`degradedSummary: none`, `degraded: []`** — no agent dropped, no isolation trip, nothing missing.
**16 of 16 stories committed. Six of seven slices sealed `approved`.** One baseline repair
(`90421d0`). Halted at `alpha-release` / `publish-0-2-0-alpha-1`, `blocked-dependency`.

| Slice | Verdict |
|-------|---------|
| `decision-records` · `typed-vocabulary` · `codec-and-command-loop` · `testing-surface` | approved |
| `projection-runner` · `worked-example-and-proof` | approved |
| `alpha-release` | **unsealed** — the run halted before its review |

**The blocker is a correct refusal.** AC-007 requires a live `cargo publish` and a post-publish
resolution from outside the workspace. `_storymap.md:163-166` names `cargo publish` one of exactly
two human handoffs in this map, and no gate step can reach a live registry. Two of AC-007's three
conjuncts were discharged **before** the cut — the publish order with its reasoning, and the
yank-and-republish instruction in `_release-log.md` §1 — and the tree is cut and gated whole
(`cargo xtask ci` → `all checks passed` at `952a870`, all four OPTIONAL steps **run**;
`cargo publish --dry-run -p happenstance-core` green). The ledger row reads `satisfied: false` and
names the blocker. Nothing was stubbed.

Because the run halted at stories, **Integration and the project review never ran** — there is no
`_integration.md` and no `_review.md`. HS-P0011 stays at `implementation`/`implementing`.

### Story gate — 13 approved, 3 held (human, batched)

`HS-S0018`–`HS-S0030` are at `report`/`in-review` with `approved` recorded. `HS-S0031`,
`HS-S0032` and `HS-S0033` were **deliberately left at `plan`/`ready`**: their slice never reached
review, so no reviewer has looked at them, and advancing them would put a human gate over unchecked
work. Same refusal run 1 made, for the same reason.

### The per-story gate rejected six stories inside slices the reviewer had sealed `approved`

Every one was a boundary failure, and every one was **compelled** — the story's own legitimate work
touched a file its fence did not admit. Instances 11 through 16 of the class HS-P0010 named and
predicted. Settled by human decision, each widening carrying its argument **and its limit** beside
the fence:

| Widening | Stories | Scope clause |
|----------|---------|--------------|
| `Cargo.lock` (`390e1e0`) | `domain-event-and-decision-model`, `command-loop`, `given-when-then-dsl`, `worked-example-on-typed-layer` | lockfile only; `[workspace.dependencies]` and every other manifest stay out |
| `standards/rust/**` (`34d5311`) | `command-loop`, `misbehaving-testkit-stores`, `compile-fail-proof-artefact` | **citation re-anchoring only** — rule text, evidence selection, retirement and new atoms stay out |
| `xtask/src/main.rs` (`34d5311`) | `projection-trait-and-runner` | **only assertions this story's own deliverable falsified**; no new gate step |

The `Cargo.lock` case is a **repair, not a widening**: each fence already authorised a `Cargo.toml`
edit, and cargo rewrites the lockfile as the mechanical consequence — the boundary permitted the
cause and forbade the effect, so no implementation could satisfy both. Four sibling specs in the
same project already carried the entry, and `worked-example-on-typed-layer`'s own body *predicted*
the movement in two places. It was an inconsistency inside one planning pass.

`projection-trait-and-runner`'s is the most interesting: landing the runner made an existing `xtask`
test's premise false — *"`happenstance` re-exports no projection item"* — and the implementer
restated the assertion in the direction that still has content rather than deleting it or leaving a
false test standing.

**Worth filing upstream.** The slice reviewer sealed all seven slices `approved`; the deterministic
per-story gate then rejected six of the thirteen stories inside them. HS-P0010 recorded this
disagreement twice and it is now systemic — the slice review is the instrument that never runs
`verify --grain story`.

### Two instrument defects found by this gate, neither of them a code problem

**1. `HS-S0024`'s acceptance ledger was unparseable for a two-character reason** (`b30e70e`). Two
`evidence:` values embedded grep regexes — `'unwrap\(\)|expect\(|unreachable!'` and
`'assert_eq!\(.*\[…'` — inside YAML **double-quoted** scalars, where backslash is an escape
character and `\(`, `\)`, `\[` are invalid escapes. The YAML parse throws,
`parseLedgerBlock` returns `null`, and `verify` reports *"`_ledger.md` has no parseable
acceptance-ledger block"* — which reads as a **missing ledger** rather than as two mis-encoded
characters in an otherwise complete ten-row ledger. The backslashes were doubled so the parsed
string is byte-for-byte what the author wrote; no criterion was edited and no row flipped. **The
error message is the defect worth reporting**: it names the wrong failure and gives the author no
way to find the two characters. The passing ledgers avoid it only by accident, using `evidence: >-`
block scalars, where backslash is literal.

**2. A stale `index.lock` silently ate commits for fifteen minutes** (`52d93a4`). A zero-byte
`.git/worktrees/…/index.lock` held by no git process made every `redkiln advance --commit` and
`record-links` after 13:02:55 write its files and fail to commit them — while `advance` still
exited **0**. `HS-S0022`'s approval sat in the working tree as an uncommitted
`stage: report` / `status: in-review`: exactly the state this command file warns is invisible to
`git status` reasoning and destroyed by any later tree reset. Caught by inspecting the tree rather
than by any check. **An `advance --commit` whose commit fails should not exit 0.**

**A false lead, recorded so it is not re-run.** Two ledgers (`misbehaving-testkit-stores`,
`given-when-then-dsl`) have **CRLF** working-tree line endings under the global
`core.autocrlf=true` with no `.gitattributes`; the committed blobs are LF. This looked like the
cause and **is not** — `ledgerBlock` splits on `/\r?\n/`, and `given-when-then-dsl` passed while
CRLF. The same artifact does defeat anchored `grep '^…$'` searches over `.bklg` specs, which cost a
wrong answer earlier in this session.

### HS-P0011 `typed-layer-and-alpha-release` — run 3 and CLOSE, 2026-08-16 (`wf_6bd6e644-a7d`)

Launched fresh at the same baseRef **`74135b8`**, no `resumeSlice` and no cache-buster. Git truth
alone pointed at the right place: all sixteen stories were already committed and six slices sealed
`approved`, so only `alpha-release` — committed but unsealed — was re-entered, at **Review**, with
no implementer dispatched.

**`degradedSummary: none`, `degraded: []`, `missingArtifacts: []`, `uncoveredAcs: []`, no
`baselineRepairs`, no blockers. 16/16 stories, all seven slices `approved`. Project verdict
`approved` by the human at `80984dd1`; the item is held at `review`/`in-review` with `--stay`,
awaiting `/redkiln:closeout`.**

Rubric: ac-coverage / integration-reachability / test-integrity / gate-greenness / brief-fidelity /
intent-fidelity **3**; presentation-fidelity **0 — does not apply**. `overall: 3`.

### The blocker was cleared by the act itself, not by a wave

`0.2.0-alpha.1` is **live on crates.io** — `happenstance-core` 21:36:30Z, `happenstance` 21:36:55Z,
`happenstance-testkit` 21:37:19Z, all `yanked: false`. The repository owner ran the three
`cargo publish` commands at the entry gate; the orchestrator recorded the transcripts verbatim into
`_release-log.md` §5.1 at `47b407f` **before** launching, because they existed only in that terminal
and HS-S0033's own ledger note says a publish transcript is one-shot. That commit carries no `Story:`
trailer and was deliberately **not** recorded into any `links.commits`.

Two things the transcripts proved rather than asserted:

- **The publish order verified itself.** Verifying `happenstance` printed `Downloaded
  happenstance-core v0.2.0-alpha.1` and compiled against it — cargo resolved the requirement *from
  the registry*, which is only possible because core went first. Had `[workspace.dependencies]` still
  said `0.2.0`, that line is where it would have failed, with core already permanently live.
- **`happenstance-testkit`'s published manifest carries no entry for `happenstance` at all.** Cargo
  stripped the versionless path dev-dependency, confirming NF-006/CF-32 from the registry side.

The MSRV shipped as `1.97.1` on all three — ADR-0029's floor reaching a published artefact for the
first time, the event ADR-0004's **provisional** marker is scheduled to lose at phase 12.

### The boundary class did not recur, for the first time in three projects

Every one of the three story gates passed clean. HS-P0010 recorded seven instances of compelled
boundary failure and HS-P0011 run 2 recorded six more; run 3 recorded **none**. The reason is
visible in `0a3e021`, which repaired two widenings that had gone unrecorded rather than needing new
ones. Not evidence the class is closed — the two deny-level cross-reference checkers are still there
— but the first run in this initiative where no fence had to move.

The orchestrator recorded **only each story's own checkpoint** in `links.commits` (`0a010c9`,
`344f2c0`, `448e1ac`) and deliberately withheld `0a3e021`, whose files span *two* stories' folders
plus telemetry and therefore lie outside either fence. That is HS-P0010 run 2's rule applied
pre-emptively rather than after a boundary failure.

### Open, disclosed by the review rather than hidden — four that need an owner

1. **Every documentation link on the published crates.io page 404s** (N-3, `_release-log.md` §5.6).
   Not a wrong URL: `github.com/Wet-Ink-Corporation/happenstance` is 404 to an anonymous client, so
   the repository is simply not public. `## Stability`'s own pointer — *"What changed is in
   `CHANGELOG.md`"* — lands a stranger on a 404, and AC-004's verifying test names that link.
   **Correctly not a yank and not `0.2.0-alpha.2`:** a new number changes nothing, because the links
   are identical strings that start resolving the moment the repository is public, with no republish.
   Routed to **HS-P0016**. The decision it asks for — publish the repository, or point the published
   links somewhere that resolves while it is private — is a visibility decision no story may take.
   **Third item routed to HS-P0016**, after the testkit README understatement and the eight rustdoc
   citations.
2. **PS-18 / PS-27 / PS-30 took a THIRD disposition** (N-4). `project.md`'s AC-008 offers two
   outcomes — name their callers, or promote to a documented exclusion — and what shipped is a
   counted `[DEFERRED]` marker carrying the count, the evaluation point and a named owner
   (`spec/SPECIFICATION.md:5273`, `:5568`, `:5670`). Judged *within* AC-008, because the failure mode
   the criterion exists to prevent is a provisional marker nobody evaluates and the evaluation
   demonstrably happened — but it is a judgement the project made, and it was put in front of the
   human at the gate rather than left to pass in a file.
3. **`RUNBOOK.md`'s phase-7 exit boxes are still unticked for six things now demonstrably done**
   (N-5, `RUNBOOK.md:4060-4088`). CLAUDE.md calls RUNBOOK *"the plan of record, and how far it has
   got"*, so six false negatives read as forgotten work. Only the `happenstance-macros` box was
   ticked. Not in this project's DoD, hence unrouted.
4. **ADR-0031 is staged intake only** (N-2) — `.kb/_intake/0032-adr-0031-the-runner-collapses-upward.md`;
   `.kb/decisions/` stops at 0030 and `kb-decision-0007` still reads `superseded_by: null`. The
   accepted atom was correctly left **unedited** rather than amended in place. Project DoD 3 required
   only ADR-0020 and ADR-0021 accepted, and both are. Carried to the next ingest wave, which now owes
   **two** things — this and PS-32's correction to ADR-0007's long-form Context.

### Found by the story gate, owed to nobody yet

`cargo xtask spec-trace` reports **two conformance rules claimed by no clause and owing a decision**,
and one of them matters: `k_disjoint_boundaries_admit_exactly_k_commits` enforces *the central DCB
independence proposition* — that commands sharing no consistency boundary do not conflict — and **no
clause states it**. The word *disjoint* does not appear in the specification. ES-25's *only if* half
forbids the false-positive direction and does not say this, so claiming it there would assert that a
`[FROZEN]` clause contains a proposition it does not. It does not fail the gate. **Unrouted.**

### Still carried, unchanged

- **redkiln #122 is a release blocker.** CI's `backlog` job asserts the `unconsumed-foundation` list
  is empty; the nine problems from planning commit `ae77ac4` persist. Settle before
  `publication-and-positioning`, not at the PR.
- The two upstream instrument defects from run 2 stand: an `advance --commit` whose commit fails
  should not exit 0, and `verify --grain story`'s fence matching should not silently ignore an entry
  it cannot parse.

### HS-P0012 `sqlite-durable-store` — run 1, 2026-08-17 (`wf_f59e2fdc-003`) — STOPPED AT A BUDGET CHECKPOINT

baseRef **`90cbca5`** — the telemetry commit closing HS-P0011, captured on a clean tree after
`cargo xtask affected --base main` passed there (227 tests, exit 0). The full-suite step was skipped
against `entry_baseline`. HS-P0012's design verdict was already `approved` from 2026-08-12, so
`advance --to implementation` walked cleanly (`67310f2`). **Keep baseRef stable across re-launches of
this project.**

**Not a halt and not a blocker** — the repository owner stopped the run at 80% of weekly token usage,
by decision at a checkpoint the orchestrator offered. `TaskStop`, not a workflow exit: there is **no
manifest**, no `_integration.md` and no `_review.md`, and `degraded` is empty because nothing reported.
Recorded as `stopped:operator-checkpoint`.

**0 of 14 stories committed. `_slices.md` was scaffolded empty. The only commit in `baseRef..HEAD` is
the orchestrator's own advance.** So a re-launch fresh at `90cbca5` re-enters at story 1 with both
resume axes empty, and forfeits nothing but this run's preflight.

**The partial work was preserved off-branch rather than discarded.** Story 1's implementer was mid-flight
and had written ~975 lines — `crates/happenstance-testkit/src/bench.rs` (767), `tests/memory_benchmarks.rs`
(208), plus `Cargo.toml` and `lib.rs` edits. It compiles but is genuinely unfinished: `fold_attempt` and
`replay` are dead code, which `-D warnings` would reject. It is at
**`wip/hs-p0012-benchmark-harness` (`d8fd819`)** and deliberately **not** on
`initiative/from-contract-to-published-library`, because a commit with no `Story:` trailer inside
`baseRef..HEAD` would pollute the project's cumulative review diff for no gain. It is reference material,
**not a resume point** — the two-axis model re-implements HS-S0034 from scratch, so run 2 should expect to
regenerate it. Drop the branch whenever it stops being interesting.

### What run 2 should know before it launches

- **The ADR story will not block the way HS-P0010's and HS-P0011's did.** Both of those halted because a
  foundation story's AC asserted an *accepted* `.kb/decisions/` atom only a human-invoked
  `/redkiln:kb-ingest` wave may author. `adr-0022-append-condition-strategy`'s **AC-008 inverts that**: it
  requires that **no** `.kb/decisions/**` or `.kb/maps/**` path appears in the diff, and that exactly two
  files be staged under `.kb/_intake/` — the atom source and a *separate* evidence source, so the decision
  can be superseded without invalidating the numbers. The story genuinely ends at staged-and-ready, and its
  criterion agrees with its prose for the first time in this initiative.
- **The wave dependency moved up a level, to project DoD 3**, which demands ADR-0022 as an accepted atom.
  So the block surfaces at **Integration**, after all five slices, rather than at story 2. Planning a wave
  between run 2 and the project review is the cheaper order than discovering it at the DoD bar.
- **Story 13 (`crates-io-name-and-packaging-facts`) performs a live, irreversible registry act** — reserving
  `happenstance-sqlite` on crates.io. `RUNBOOK.md:4191-4193` says claim the name when the phase *starts*,
  so doing it by hand up front converts a probable mid-run halt into a precondition. Same class as
  HS-P0011's `cargo publish`, which halted run 2 and was cleared by the owner running it at the entry gate.
- **Two of the nine `unconsumed-foundation` errors are this project's own stories** — `HS-S0034`
  (`benchmark-harness`) and `HS-S0035` (`adr-0022-…`). Still redkiln **#122**, still a release blocker to
  settle before `publication-and-positioning`, and still unrouted.
- **Two escalate-don't-settle items**, per `_storymap.md`: CF-40's ownership contradiction, whose open
  question names *this* phase as the forcing one, and the DoD 7 second-unlike-batch-shape question, which is
  assigned to `projection-store-freeze` — if it lands here it is a **blocking re-plan** that inverts the
  6 → 8 DAG edge, not something `projection-store-passes-the-borrowed-suite` absorbs.
- **Schedule shape.** Largest single adapter on the trunk (ten runbook-days, `RUNBOOK.md:4235`) with
  `publication-and-positioning` blocked behind it. CF-33 forbids a watchdog, so a `BEGIN IMMEDIATE` deadlock
  in `concurrency-family-and-contender-count` hangs until the CI timeout — a finding about ADR-0022's timeout
  paragraph, not something a test may paper over. The initiative DoD wants 64 contenders; the code sets
  `CONTENDERS = 8`; story 8 owns closing it.

### HS-P0012 `sqlite-durable-store` — run 2, 2026-08-17 (`wf_5c32a42d-c62`)

baseRef **`90cbca5`**, unchanged from run 1 and to be held unchanged again. The full-suite step was
skipped against `entry_baseline`; `cargo xtask affected --base main` was green beforehand (227 tests,
exit 0). Both resume axes were empty — run 1 committed no stories — so the run started at story 1 and
forfeited nothing.

**`degradedSummary: none`, `degraded: []`, no `baselineRepairs`.** 11 agents, 0 errors, ~4.3h.
**7 of 14 stories committed. Halted at the *review* step of slice `durable-event-store`,
verdict `changes-requested`.** Integration and the project review never ran, so there is no
`_integration.md` and no `_review.md`; HS-P0012 stays at `implementation`/`implementing`.

| Slice | Verdict | Sealed by | Stories |
|-------|---------|-----------|---------|
| `bench-harness-and-adr` | **approved** | `20842af` | `2665883` · `791b929` |
| `durable-event-store` | **changes-requested** | `1f29a46` | `8381c89` · `23bc776` · `0c6ce2b` · `11596b4` · `41a2064` |

Slices 3–5 never ran, so neither the crates.io reservation (story 13) nor the ADR-0022 wave
dependency at project DoD 3 was reached. Both remain as run 1 predicted them.

### The blocking finding is a new instance of the gate blind-spot class, and it was verified

`RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance-sqlite --no-deps` fails at
`crates/happenstance-sqlite/src/event_store.rs:39` — the module doc links `[`SqliteEventStore::migrate`]`,
which is private (`-D rustdoc::private_intra_doc_links`). Introduced by `8381c89`. **Reproduced by the
orchestrator against the tree, not relayed on the reviewer's word.**

`cargo xtask affected` deliberately excludes the documentation build (`xtask/src/affected.rs:41-42`),
so every per-story gate passed green over a red `cargo xtask ci --fast`. This is the same family as
HS-P0010's `cargo doc` default-features hole and HS-P0011's eight uncheckable rustdoc→spec citations:
**a cross-reference class with no instrument over it in the loop that runs per story.**

The second blocking finding is the consequence rather than a separate defect —
`schema-migration-and-identity/_ledger.md` AC-008 was flipped `satisfied: true` with evidence closing
*"the `docs` step of `cargo xtask ci --fast` builds the rustdoc"*, which is exactly the step that fails,
on the very block AC-008 delivers.

Four non-blocking findings stand, in `_slices.md:33-51`: an `UPDATE … WHERE origin_position IS NULL`
that full-scans `event` inside `BEGIN IMMEDIATE` on every append (correct, but the precise cost class
the schema amendment exists to remove); four drifted `file:line` citations across two ledgers; a
self-contradicting `verifying_test` on `wide-query`'s AC-007; and `xtask/src/spec_trace.rs` edited
outside every story's fence — necessary, correct, unattributed. Instance seventeen of the boundary class.

### NO STORY VERDICT COULD BE RECORDED — the boundary check now fails every story in the initiative

The human gate was answered — **approve slice 1, hold slice 2** — and **neither half could be
executed.** `redkiln advance HS-S0034 --to report` exits 1 on `implement`'s command gate:
`verify --grain story` reports `[FAIL] boundary` naming roughly **180 files**, spanning
`crates/happenstance/**`, `examples/outside-projection-adapter/**`, `experiments/polling-cost/**` and
the whole of `standards/rust/` — that is, every code path touched by HS-P0010 and HS-P0011, none of
which `2665883` goes near. The story's own commit touches 14 files.

**This is not this story's defect, and it is not a fence that needs widening.** The decisive check:
`redkiln verify --item HS-S0018 --grain story` — an HS-P0011 story **approved, recorded and closed on
2026-08-16** — fails today with the same whole-branch list. The changed-set the check compares each
fence against is the entire initiative branch (`ce933d8..HEAD`, 1,091 files, less the non-code trees),
not the story's `links.commits`. The two reports differ only by each story's own declared boundary
being subtracted, which is what confirms the base is shared.

Consequence, and it is the part that matters: **an approval and a rejection are equally unrecordable.**
`--to report` runs the red command gate on the way out, so a held story cannot reach `report` to have
its `changes-requested` verdict recorded either. All seven stay at `plan`/`ready` — the state this
command file names as the one case where a rejection cannot be recorded, now reached for every story
at once rather than one.

Provenance was recorded first and did commit (`694dccd`): each story carries **only its own
checkpoint**, and the slice-wide fix commit `9a10dbf` was deliberately withheld, its files spanning
several stories' folders. `verify`'s `provenance` and `ledger` checks both report `[ok]`.

### A concurrent redkiln session was mutating shared state throughout

`main` moved during this session — from `be1712a` at entry to `570fefa` — and `redkiln record-links`
then failed outright with `EC-001`: the id lock at `D:\repos\happenstance\.git\redkiln\.id.lock` held
by pid 27500 in `.claude/worktrees/docs-that-teach`, still running. The index build at entry had
already reported `partitionsDiverged: 5` with 692,144 telemetry bytes unindexed across worktrees, for
the same reason: *a partition name must have a single writer*.

Whether the moved merge base is what regressed the boundary check is **not established** — it is a
candidate, not a finding, and it is recorded here as one. What is established is that the check passed
for these same stories yesterday and fails for them today. **Do not widen a single fence against this.**
Sixteen prior boundary widenings in this initiative were each argued from a compelled edit; this one
would be a fence widened to accommodate an instrument reading the wrong diff, and it would be the first
that could not be justified from the story's own work.

### Next

The two blocking findings are cleared by a fresh re-launch at the same baseRef `90cbca5` — git truth
re-enters `durable-event-store` at Review with no implementer dispatched, hands the six findings to a
new reviewer as hypotheses, runs the fix→re-review→seal tail, then continues into slices 3–5. That was
the human's answer at the gate and it stands; it was not launched, because the boundary regression and
the live concurrent session both want settling first — a run whose per-story gate cannot pass would
halt at the first story's checkpoint.

### HS-P0012 — run 3 not launched, 2026-08-17 (preflight investigation)

Preflight was clean: tree clean, branch and worktree confirmed, full-suite step skipped against
`entry_baseline`, and the affected gate green (227 tests, exit 0). baseRef stays **`90cbca5`**.
**No workflow was launched and no transition was recorded**, by human decision at the entry gate:
the ADR-0022 ingest wave goes first, because project DoD 3 demands the accepted atom and a run that
reached Integration without it would halt there. The boundary blocker was investigated instead.

The concurrent-session hazard has cleared on its own — `.git/redkiln/.id.lock` is gone and no
redkiln process holds it.

### The boundary blocker: two candidates falsified, and the real finding is worse

**Falsified — the moved `main`.** Run 2 recorded this as a candidate, correctly marking it
unestablished. It is now ruled out. `merge-base` is `ce933d8` against **both** the old main
(`be1712a`) and the new (`570fefa`), and `git diff --name-only <main>...HEAD` returns **1,089 files
either way**. Neither telemetry commit is an ancestor of this branch, so main's movement cannot
have moved the merge base.

**Falsified — a plugin upgrade.** `0.14.0` and `0.18.0` are both installed, both dated 2026-08-09,
before any implementation work. No version changed under the initiative.

**The mechanism, read from the shipped bundle and not only the source.**
`changedFiles(root, base)` computes `git diff --name-only ${base}...HEAD` — the *cumulative* branch
diff from the merge base — plus working-tree and untracked files. `verify` defaults `base` to
`main` and the pinned `story.yaml` gate declares no `--base`, so there is no override:
`redkiln advance` has no `--base` flag at all, only standalone `verify` does. `boundaryCheck` then
exempts `.bklg/`, `.kb/` and `.redkiln/` and requires **every remaining file** to match the one
story's fence. On this branch that is ~180 non-exempt files from two completed projects against a
fence admitting one or two.

So it is **structural, not a fence problem**: after the first story on a shared initiative branch,
no story can pass. That is why `HS-S0018` — approved, recorded and closed on 2026-08-16 — fails
today, with `affected-gate`, `ledger` and `provenance` all still `[ok]` and only `boundary` red.
**No single static base would fix it**, because story 5's fence would still see stories 1–4's files.

### The finding that matters: the story gate has never once passed by evaluation

The repository's own committed telemetry settles what run 2 could only guess at. Census of
`gate_result` events in `.redkiln/telemetry/events/ryan-britton@from-contract-to-published-library.jsonl`:

| type/stage | pass: evaluated | pass: memoized | fail |
|---|---|---|---|
| `story/discover` | **135** | 0 | 0 |
| `project/intake` | **10** | 0 | 0 |
| `story/implement` | **0** | **37** | **18** |
| `project/integration` | **0** | **4** | 0 |

**Every one of the 37 story `implement` passes in this initiative is `basis: memoized`. Not one is
`basis: evaluated`. Every one of the 18 evaluations failed** — seventeen on `boundary`, one on
`ledger`. `story/discover` shows 135 evaluated passes in the same file, so this is not the memo
replacing evaluation everywhere; it is specific to `implement` and `integration`.

**Both closed projects reached `integration` the same way** — `HS-P0010` and `HS-P0011` each show
two `pass=true basis=memoized` integration gates and zero evaluations.

The replay is exact rather than inferred. Reimplementing `declaredBoundary` and `boundaryRegExp`
from the shipped bundle and running HS-S0018's three declared patterns against the branch diff at
`4314349` — the tree at 2026-08-16T20:00, the minute its pass was cached — yields **1,034 changed,
161 non-exempt, 160 stray**. The check would have failed. Telemetry records it passing, memoized,
at 20:00:23.

**This reframes the "boundary class" narrative recorded across HS-P0010 and HS-P0011.** Seventeen
widenings were argued through by hand, each from a genuinely compelled edit, and each was followed
by a pass that was *replayed rather than computed*. The stray-file reports that prompted them were
real. What is **not** established is that any widening ever made the check pass — no evaluation
records one. Run 2's `HS-S0034` failure is not a regression; it is the first time no memo was
available to paper over a check that has been failing all along.

### A second, independent defect: the check silently disables itself on 13 of 135 specs

`declaredBoundary` finds the **first** heading matching `/^#{1,6}\s+.*boundary/i` anywhere in
`spec.md`, then scans forward for a fence and returns `undefined` — a silent, warn-free **pass** —
if it meets another heading first. In a DCB library "boundary" is domain vocabulary, so the regex
collides with prose headings.

Replicating it across all 135 specs: **122 enforce a boundary, 13 do not.** Five lose it to a
domain heading matching ahead of `## PR boundary` — `### What an arm is, and where the chunk
boundary comes from`, `# Spec — A payload survives the boundary unchanged, and replay changes
nothing`, `### The packaging boundary is the placement rule…`, `### 7. This project's boundary bar
is the full gate, not --fast`, `### 10. Where the verdict is written, and the one boundary tension
it raises`. The other eight reach `## PR boundary` but carry a sub-heading before the fence.

Two are this project's: **`wide-query-chunked-not-refused` (HS-S0039)**, one of the seven stories
run 2 committed, whose boundary check never ran at all; and
**`reopen-negative-control-and-durability-verdicts` (HS-S0043)**, upcoming in slice 3.

This is the same family as run 6's fence-comment trap, one level up: there, an unparseable fence
*entry* was silently ignored; here the entire *check* is. Both fail closed in the safe direction
and both discard the author's stated limit without saying so.

### Upstream — the headline defect was already filed AND already fixed

**Corrects the framing above.** The whole-branch scoping defect is **redkiln #94** — *"The story
boundary gate diffs against a hardcoded main, so a narrow PR-boundary fence can never pass on a
long-lived initiative branch"*. It was filed **against 0.18.0, the exact version pinned here**, and
**closed COMPLETED on 2026-08-11**. Its report describes this symptom precisely, down to the ~180
stray paths and the observation that the gate is strictest on the most disciplined specs.

**0.19.0 fixes it** (`src/store/verify.ts:917-932`), taking the issue's own candidate (2):

```js
// #94 — the BOUNDARY question is "did THIS STORY stay in its lane", so it is
// asked of the story's own recorded commits when it has any, falling back to
// the base-derived set when it has none (a story mid-flight, before its first
// checkpoint). Deliberately scoped to this one check: …
const ownScope = ownChangedFiles(root, commitLinks(located.data));
checks.push(boundaryCheck(located.dir, ownScope ?? changed));
```

`changedFiles` and `boundaryCheck` are byte-identical in 0.19.0; the fix is entirely in what the
caller *passes*. `affected-gate`, `ledger` and `provenance` deliberately keep the broad set.

**So nothing here needs filing, and nothing needs a fence widened. It needs a plugin upgrade.**
`redkiln@redkiln-local` is installed at **0.18.0**, `lastUpdated: 2026-08-10T02:21` — one day
before #94 closed. **`v0.19.0` is tagged and published.** Run 2's stories already record
`links.commits` correctly (one checkpoint each, `694dccd`), which is exactly the input 0.19.0's
boundary check consumes, so the seven committed stories should gate cleanly straight after the
upgrade with no rework.

**Do not run `redkiln adopt --templates` as part of the upgrade** — CLAUDE.md forbids it, and
`upgrade` recommends it. It would overwrite all six deliberate template customisations and then
fail the `backlog` CI job on the absence it created.

### Filed: redkiln #135 — the one defect here that was genuinely new

**<https://github.com/Wet-Ink-Corporation/redkiln/issues/135>** — *"declaredBoundary matches the
first heading containing 'boundary' anywhere, so a prose heading hijacks the parse and the fence is
silently skipped."* Unfixed on 0.19.0 (`declaredBoundary` is byte-identical there), orthogonal to
#94, and not masked by it: #94 fixed the *scope* of the changed set, this is the *parse*.

Evidence filed: 122 of 135 specs enforce a boundary, 13 do not. Five lose it to a prose heading
matching ahead of `## PR boundary` — including `byte-identical-round-trip-and-idempotent-replay`,
whose **H1 title** contains the word, so its parse is over before the body begins. Eight reach
`## PR boundary` and find no fence under it. Both are silent, warn-free passes.
`wide-query-chunked-not-refused` (HS-S0039) is the worked case: a valid three-line fence at
`spec.md:294` that the parser never reaches, on a story already committed and slice-reviewed.

### Not filed — the memo census, which the code says should be impossible

`readVerifyPass` matches on an exact key with a TTL, and `writeVerifyPass` runs **only** after a
real pass, immediately before `basis: "evaluated"` is returned (`src/process/gates.ts:406-408`). So
37 memoized passes require at least one evaluated pass, and the census finds **none** — not in this
partition, not in `ryan-britton@happenstance.jsonl`, not in the kb-intake partitions, not in the
`docs-that-teach` worktree's.

Left unfiled deliberately, because a report with no mechanism is a guess, and two known problems
could produce this shape without a memo defect: telemetry partitions diverged across worktrees
(`partitionsDiverged: 5`, ~692 KB unindexed), and run 2's stale-`index.lock` finding — an
`advance --commit` that wrote its files, failed to commit, and **still exited 0** — means gate
telemetry from these runs is not fully trustworthy either. **Re-run the census after the 0.19.0
upgrade**: if evaluated passes start appearing, the anomaly was the boundary defect all along and
there is nothing to file. Adjacent, already open: #86.

### Still standing, unchanged

- **redkiln #122 is a release blocker** — CI's `backlog` job asserts the `unconsumed-foundation`
  list is empty; nine problems persist from planning commit `ae77ac4`.
- **#129** — a trailing `#` comment silently voids a PR-boundary fence entry. Same family as #135,
  one level down.
- **#130** — the slice review seals `approved` without running the story boundary gate. This is the
  instrument disagreement HS-P0010 and HS-P0011 each recorded; it is filed.
- An `advance --commit` whose commit fails should not exit 0 (run 2's finding, still unfiled).

### Next

1. **The ADR-0022 ingest wave** — `/redkiln:kb-ingest` over `.kb/_intake/`, which holds six
   documents: `0033-adr-0022-append-condition-strategy.md` and its separate evidence source
   `0034-append-condition-experiment-2026-08.md` (the pair AC-008 requires), plus four carried
   forward — `0031-adr-0021-serde-attribution-correction.md`,
   `0032-adr-0031-the-runner-collapses-upward.md`, `contract-defect-log-phase-7.md` and
   `happenstance-macros-verdict.md`. Drop `README.md` at the approval gate. Suffix the wave id.
2. **Reserve `happenstance-sqlite` on crates.io by hand**, converting story 13's live irreversible
   act into a precondition — the same move that cleared HS-P0011's `cargo publish`.
3. **Then re-launch run 3 fresh at `90cbca5`.** Git truth re-enters `durable-event-store` at
   Review.

**Do item 0 first: upgrade the redkiln plugin to 0.19.0.** It is the fix for #94, it is published,
and it turns the story gate from unpassable back into a real check — including for the seven
stories run 2 already committed, whose `links.commits` are recorded correctly. Verify afterwards
with `redkiln verify --item HS-S0018 --grain story`, which fails today and should pass on 0.19.0
without a single fence being touched. **Do not run `redkiln adopt --templates`.**

### Item 0 DONE — 0.19.0 installed, and the prediction held exactly, 2026-08-17

`redkiln@redkiln-local` is at **0.19.0** (`lastUpdated 2026-08-17T19:33`, `gitCommitSha 93eac24`);
`redkiln --version` and `workflow-root` both resolve to it, and `ownChangedFiles` is present in the
installed bundle.

**`redkiln verify --item HS-S0018 --grain story` now exits 0** — `[ok] affected-gate`,
**`[ok] boundary`**, `[ok] ledger`, `[ok] provenance` — with **not one fence touched**. That is the
falsifiable claim this diagnosis rested on, and it held. The seventeen widenings argued through in
HS-P0010 and HS-P0011 are retrospectively vindicated as *unnecessary* for the reported symptom: the
instrument was reading the wrong diff, and no fence needed to move for it.

### The instrument went from unusable to precise, and immediately found real work

Replaying 0.19.0's `ownChangedFiles(links.commits)` per story across HS-P0012 (`redkiln verify`
confirmed on HS-S0034; the rest computed with the same logic, on a clean tree):

| Story | Own files | Boundary |
|---|---|---|
| `benchmark-harness` HS-S0034 | 14 | **FAIL — 4 stray** |
| `adr-0022-append-condition-strategy` HS-S0035 | 31 | pass |
| `schema-migration-and-identity` HS-S0036 | 10 | pass |
| `append-atomicity-and-store-limits` HS-S0037 | 10 | **FAIL — 1 stray** |
| `lazy-read-with-snapshot-ceiling` HS-S0038 | 6 | pass |
| `wide-query-chunked-not-refused` HS-S0039 | — | **not checked (#135)** |
| `sqlite-fixture-and-whole-suite` HS-S0040 | 6 | pass |

The seven unstarted stories have no `links.commits` and correctly fall back to the branch diff.

**HS-S0034 — four `standards/rust/**` atoms** (`41-declarative-macros`, `52-wasm32-and-target-cfg`,
`62-doctests-and-harnesses`, `91-adapter-authoring-recipe`). **9 insertions, 9 deletions**: pure
citation re-anchoring, `lib.rs:490→:516`, `:466→:492`, `:505→:531`. Adding `event_store_benchmarks!`
moved the cited lines and `lint-constitution` is a gate step, so stale citations are a red gate.
**Instance eighteen** of the compelled class, and it matches the precedent set at `aef8990` and
`34d5311` — a widening scoped to *citation re-anchoring only*, rule text and evidence selection out.

**HS-S0037 — `xtask/src/spec_trace.rs`.** Run 2's reviewer had already found this by reading and
recorded it in `_slices.md:33-51` as *"edited outside every story's fence — necessary, correct,
unattributed"*. The repaired instrument now attributes it, independently, to the right story. Two
findings converging from different directions is the strongest evidence either of them is real.

**HS-S0039 is not clean — it is unchecked**, and that is #135 in this project rather than in the
abstract. Its verdict should be read as *unverified*, not *passed*.

Both failures are genuine and both need a human decision on the fence; neither is the instrument.
Nothing was widened here.
