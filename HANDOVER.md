# Handover — the path to `0.2.0`

**Read this before `RUNBOOK.md`.** The runbook says *what the plan is*; this says
*where the plan actually got to*, which is the thing that goes stale fastest and
did so twice in the pass that produced this file.

`REMEDIATION-HANDOVER.md` is the historical record of the pre-publication
remediation and is **not** superseded — it is still the best account of that
effort's method and its traps. What it is no longer is current: its scoreboard
says F2-5 is blocked on "the phase-10 adapter that does not exist" and that 48
briefs await ratification. Both were true when written.

---

## State, as of `ef1bedf`

| | |
|---|---|
| Branch | `main`, clean |
| Gate | `cargo xtask ci` **exit 0 — 35 steps, zero skips** |
| Workspace version | `0.2.0` in the manifests. **Nothing is published at `0.2.0`** |
| On crates.io | `happenstance`, `happenstance-core`, `happenstance-testkit` at `0.2.0-alpha.1`; `happenstance-sqlite`, `happenstance-cloudflare` at `0.0.0` placeholders |
| Repository | **Private.** Goes public when `0.2.0` ships, not before — decided, not pending |
| Release set | **Five crates**, decided: core, typed layer, testkit, sqlite, cloudflare |

**Three lines of work were merged in this pass**: the pre-publication remediation
(243 commits), the four demonstration applications plus the 2026-09-04 KB intake
wave, and the standing benchmark suite with the merge-join read path. Phase 10's
Postgres half landed separately and is also in.

## The one thing that will mislead you

**The tree describes `0.2.0` as published, in the present tense, in five places** —
`README.md`'s status banner and table, `SECURITY.md`'s scope and supported-versions
table, `CHANGELOG.md`'s dated `## [0.2.0]` section, `Cargo.toml`'s version, and
`spec/SPECIFICATION.md` §1.3. It is not published.

That was a deliberate choice: the documents were written when the release looked
imminent, the release was then deferred, and the owner chose to leave them rather
than revert. It is contained by the repository being private — the only readers
who can be misled are people picking the tree up, which is you.

**`RUNBOOK.md`'s phase 12 preamble is the one place that states the truth
plainly.** Trust it over any prose that says the release happened.

---

## Decisions already taken — do not re-open these

Each is recorded with its reasoning and its counter-argument. Re-litigating them
is the expensive failure mode here, not implementing them.

| Decision | Where |
|---|---|
| **Five crates** in `0.2.0`, cloudflare included | `xtask/src/package.rs`'s `PUBLISHABLE`; README, SECURITY.md and ci.yml reconciled to it |
| **Stable `0.2.0`**, not another alpha | `Cargo.toml`'s version comment |
| **ES-42 frozen** — nothing needs `dyn EventStore` the E11 wrapper cannot box | `.kb/_intake/es-42-marker-earned-off-at-0-2-0.md` |
| **Seventeen briefs ratified** — nine on their own recommendations, eight reviewed | `.kb/_intake/ratifications-2026-09-06-pre-publication.md` |
| **F2-5 discharged** — answered by Postgres's `AFTER INSERT` trigger fault | `.kb/_intake/f2-5-holds-the-release-for-phase-10.md` |
| **Repository stays private until the release** | this file, above |
| **The `0.2.0`-present-tense documents stay as they are** | this file, above |

**ES-11 is settled and needs nothing.** Its falsifier has *not* fired: an async
driver conforms by handing work to the runtime at the first poll. This was
escalated in error during the pass and retracted (`2e0a0ae`); the clause now
carries the async reading (`ef1bedf`). Commit `0341467`'s message asserts the
falsifier fired and is wrong — history, not guidance.

---

## Outstanding, in the order that unblocks the most

### 1. The implementation queue from the seventeen ratifications

Ratifying is not landing. Six items, roughly by size:

1. **`cf-18` / candidate B3** — one extra emitted test per suite that fails if a
   capability is declined without an accompanying declaration file, observable in
   a default `cargo test`. **The only unbounded item here.** The owner chose to
   cost B3 rather than take the brief's fallback of narrowing CF-18; if B3 proves
   too expensive, that fallback is still written down and taking it is a decision
   rather than a drift.
2. **`empty-decision-outcome` / Option 3** — a second success shape on the command
   path, `#[must_use]`, and every call site and doctest naming the existing one.
3. **`tags-scope-agreement` / A + B** — a run-time refusal between
   `command.rs:305` and `:311`, plus two doctest repairs.
4. **`codec-foreign-tag-resolution` / A** — a defaulted method on `Codec`.
   Option C (sealing) stays open and is the cheaper answer if no fourth codec
   ever appears.
5. **`stringified-throw-visibility` / B** — `pub(crate)`, and the crate root's
   invitation withdrawn with it.
6. **`op-read-non-exhaustive` / A** — the attribute, landing with the `to` field.
   No constructor: D was considered and declined.

`query-partition-public-surface` (1A) and `read-page-budget` (A) ratify what is
already landed and oblige nothing.

### 2. `.kb/_intake` is full and owed an ingest

Seven files plus the 49-brief directory. `/redkiln:kb-ingest` authors the atoms;
they must not be hand-written. Three of the seven are this pass's decisions, three
came from the postgres lane (ADR-0024, ADR-0038, the poll-count calibration).

### 3. `happenstance-neon`, and a question about it

Phase 10's Postgres half is in and meets the phase's exit criterion. Neon is still
a skeleton (six `todo!()`). **Whether Neon gates publication or only phase 10's
own closure is genuinely undecided** — phase 12's dependency row says `7, 8, 10`
and that row was written when phase 10 meant both halves.

Neon also carries ES-11's and ES-12's real falsifier: a one-shot-HTTP store with
no connection, no interactive transaction and no cursor. Those clauses stay
`[PROVISIONAL]` until it exists.

### 4. Benchmarks — re-run on a quiet machine

The suite is merged and the instrument works. The last full run is **withdrawn,
not committed**: the machine was under memory pressure and the `raw_*` control
arms — hand-rolled SQL sharing no code with happenstance — moved 1.34×, which is
proof of contention rather than of a regression. `codec_*` was 0.96×, so CPU-only
work was unaffected.

Its allocation counts *are* trustworthy (deterministic, machine-independent) and
showed one real result worth reproducing: a `limit-1` read of a 10,000-event log
went from **50,002 allocations and 3.5 MB to 7 allocations and 1.3 KB**.

`benchmarks/run.sh` takes 35–45 minutes; `--fast` is ~4 minutes and answers "did I
break the instrument". Contended-run artefacts are preserved in this session's
scratchpad for diffing.

### 5. Then phase 12 itself

`RUNBOOK.md`'s phase 12 work list is accurate. The publish order is forced —
core → testkit → happenstance → sqlite → cloudflare — because both adapters
dev-depend on the testkit at the workspace version. There is **no release
workflow**; publishing is manual with the owner's token.

Two items in that list are still owed and easy to forget: repointing
`cargo-semver-checks` to keep the registry baseline *as well as* `baseline-rev`
(the comment in `ci.yml` already explains why it waits until `0.2.0` exists), and
the DR-8 stranger-install smoke — a scratch project outside the workspace, no path
dependency, completing a write-then-read cycle against the published crates.

### 6. At the moment of going public

- Remove `SECURITY.md`'s… — already done. The channel resolves.
- **Set "require approval for first-time contributors"** in Actions settings.
  `pull_request` has no filter, so every fork PR fires the full three-OS matrix.
- The `backlog` CI job is `if: false` and disabled loudly; see its comment for
  what is unchecked while it is off and how to restore it.

---

## Traps, all of them paid for

**Read the exit code, not the notification.** Three "completions" in the
remediation were false. Launch `cargo xtask ci` detached, write `$LASTEXITCODE` to
a sentinel file, and read the sentinel.

**`cargo hack --no-dev-deps` rewrites all 13 manifests in place** and restores
them on exit. Kill it mid-run — a timeout does this — and the tree is left
stripped of `[dev-dependencies]`. `git checkout -- .` before believing anything
after an interruption.

**After a large merge, the first gate run needs a clean.** A stale rmeta made a
green tree report a source error that was not there (`no futures_core in the
root`), and `cargo clean -p` fixed it. It cuts both ways.

**Repoint citations by anchor, never by offset.** Offsets in one change have run
+9, +25, +49, +50 and +82. `lint-constitution` names the correct line; when it
refuses to choose, the anchor is bad and sharpening it *is* the repair. Roughly 50
citations were repointed in this pass.

**Do not read an intermediate commit as current state.** The ES-11 escalation in
this pass was built on a commit whose conclusion the same lane later overturned,
with the correction in a file already open. This is the single most expensive
mistake made here.

**A `///` doc comment on a `pub mod` declaration** is resolved in the *parent's*
scope and will break the module's own intra-doc links. The `doc(cfg)` badge is an
attribute and is fine; prose belongs in the module's `//!` header.

**Clone to a short path on Windows.** `experiments/` has filenames near the
260-character limit; `git worktree add` into a deep path fails with
`Filename too long` and leaves a half-populated tree.

**`benchmarks/run.sh` needs Git Bash, not WSL.** A detached launch that resolves
`bash` to WSL dies immediately.
