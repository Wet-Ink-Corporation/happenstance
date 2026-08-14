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
| 1 | `projection-store-freeze` | HS-P0010 | — | **in-progress** | run 1 halted; blocker cleared 2026-08-13, run 2 dispatched |
| 2 | `typed-layer-and-alpha-release` | HS-P0011 | 1 | pending | |
| 3 | `sqlite-durable-store` | HS-P0012 | 1, 2 | pending | |
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
