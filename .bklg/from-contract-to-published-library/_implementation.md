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
