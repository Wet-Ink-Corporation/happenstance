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

## State, as of `2550eaf`

| | |
|---|---|
| Branch | `main`, clean, **and pushed** |
| Gate | `cargo xtask ci` **exit 0 — 35 steps, zero skips**, re-confirmed at `a0a925b` on 2026-09-07 |
| Workspace version | `0.2.0` in the manifests. **Nothing is published at `0.2.0`** |
| On crates.io | `happenstance`, `happenstance-core`, `happenstance-testkit` at `0.2.0-alpha.1`, **not yanked**; `happenstance-sqlite`, `happenstance-cloudflare`, `happenstance-postgres`, `happenstance-neon` at `0.0.0` placeholders. `happenstance-ladybug` and `happenstance-sync` are unclaimed |
| Repository | **`INTERNAL`** — org-readable, not private. Goes public when `0.2.0` ships, not before — decided, not pending |
| Release set | **Five crates**, decided: core, typed layer, testkit, sqlite, cloudflare |

**`main` was 53 commits ahead of `origin/main` until 2026-09-07, and CI had seen
none of them.** That is worth reading as a trap rather than as history, because
the row above it said `Gate | exit 0` the whole time and was true: the local gate
is one machine and one operating system. CI carries a three-OS matrix, `msrv`,
and — the one that matters here — **`live-postgres`**, the job that runs the 101
gated tests against a real server under Docker. Phase 10's entire Postgres half,
the four demonstrations, the benchmark suite and the merged read path had never
been through any of them. The push is `3c4c728..b0d9e67`, and it is green on all
three gate platforms, `msrv` and `live-postgres`.

Everything since has been pushed as it landed, so the gap does not reopen: read
`git status -sb` before trusting the row above, because *ahead by N* is what this
row was silently worth for three weeks.

The general form, since it will recur: **a green `cargo xtask ci` is evidence
about this machine.** Three jobs in `ci.yml` cannot run locally at all —
`live-postgres` needs Docker, `msrv` needs a second toolchain, and `semver` needs
a pull request — so "the gate is green" and "the release is safe" are different
claims and the gap between them is invisible from here.

**Three lines of work were merged in this pass**: the pre-publication remediation
(243 commits), the four demonstration applications plus the 2026-09-04 KB intake
wave, and the standing benchmark suite with the merge-join read path. Phase 10's
Postgres half landed separately and is also in.

## The one thing that will mislead you

**The tree describes `0.2.0` as published, in the present tense, in six places** —
`README.md`'s status banner and table, `SECURITY.md`'s scope and supported-versions
table, `CHANGELOG.md`'s dated `## [0.2.0]` section, `Cargo.toml`'s version,
`spec/SPECIFICATION.md` §1.3, and `.github/workflows/ci.yml`'s `semver` job, whose
comment says *"All five crates now have a real published predecessor"*. It is not
published.

The last of the six was found by the `0.2.0` closeout session and is the one
worth noticing: it is not prose a reader discounts but a comment justifying why a
**check** is configured the way it is. The other five are documents; that one is
an argument about the gate.

That was a deliberate choice: the documents were written when the release looked
imminent, the release was then deferred, and the owner chose to leave them rather
than revert. It is contained by the repository not being open — though note the
containment is weaker than it reads: visibility is **`INTERNAL`**, not private, so
the audience is everyone in the organisation rather than only whoever picks the
tree up.

**There is a seventh, and it is the one that is an action rather than prose.**
`CHANGELOG.md`'s `[0.2.0]` section and `SECURITY.md`'s supported-versions table
both state that `0.2.0-alpha.1` **is yanked**. It is not — all three live crates
still resolve it. Yanking is on phase 12's list; until it happens, that sentence
is the only one of the seven a `cargo add` can disagree with.

`CHANGELOG.md` also carries an `## [Unreleased]` section sitting *above* the
dated `## [0.2.0] — 2026-09-06`. Publishing means folding one into the other and
correcting the date — a small thing that is easy to lose because the section
below it already looks finished. **It is no longer just the ES-10 poll-schedule
change**: the `0.2.0` closeout added the Postgres projection store, the armed
`READ_FAULT`, and two gate changes to it. They were put there rather than into
the dated section deliberately — dating an unpublished release is the defect this
paragraph exists to warn about, one turn further in.

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
| **`cf-18` takes B3, in the *stated-not-inherited* form** | this file, below — CF-18 stands as written and is not narrowed |
| **`happenstance-neon` gates phase 10's closure, not publication** | this file, below; `RUNBOOK.md`'s phase 12 dependency row is being repointed to say so |

**ES-11 is settled and needs nothing.** Its falsifier has *not* fired: an async
driver conforms by handing work to the runtime at the first poll. This was
escalated in error during the pass and retracted (`2e0a0ae`); the clause now
carries the async reading (`ef1bedf`). Commit `0341467`'s message asserts the
falsifier fired and is wrong — history, not guidance.

---

## Outstanding, in the order that unblocks the most

### 1. The implementation queue — **discharged 2026-09-07**

All six landed. Kept as a table rather than deleted, because three of them came
back from execution meaning something other than what was ratified, and that is
the part a future reader needs.

| Brief | Commit | Landed as ratified? |
|---|---|---|
| `op-read-non-exhaustive` / A | `9000f35` | yes |
| `stringified-throw-visibility` / B | `9225c00` | yes, plus a consequence the brief did not name |
| `codec-foreign-tag-resolution` / A | `2d37fc4` | yes, **narrower** than the brief implied |
| `empty-decision-outcome` / 3 + `tags-scope-agreement` / A + B | `0ae10ab` | yes, and they had to land together |
| `cf-18` / B3 | `3df4b6e` | **no — the costing changed the assertion** |

`query-partition-public-surface` (1A) and `read-page-budget` (A) ratified what was
already landed and obliged nothing.

The reasoning for each is in its commit message, and the three that changed are
written up for the KB in
`.kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md`.
The three worth knowing without reading further:

- **`cf-18` / B3's own wording was decorative.** *"Fails if a capability is
  declined without a stated reason"* describes a state `Capability::declined`
  already makes unconstructible, so a rule written to it would pass on every
  fixture that will ever exist. What shipped rejects declension by
  **inheritance** — the five capabilities that default to a declension in the
  testkit's own words. CF-18's MUST is untouched and still `[FROZEN]`; the
  fallback of narrowing it was **not** taken. The `wasm32` blind spot the costing
  expected to accept was closed instead, by a `cfg_attr` pair: a false predicate
  is stripped before name resolution, so no native adapter gains a
  `wasm-bindgen-test` dependency.

- **It found real work in `happenstance-postgres`.** `PostgresFixture` inherited
  `READ_FAULT`, whose default says the injection "has to come from the adapter and
  this one has none to offer" — false about this store. `PgReadStream` `FETCH`es a
  server-side cursor per chunk, so it is exactly the paged adapter the capability
  was invented for. It now declines *by scope*, names the injection that would
  work, and records that building it belongs to phase 10's remainder alongside
  Neon. **That is an addition to phase 10's scope, made by evidence rather than
  by preference.**

- **`Codec::reads_tag` is narrower than the brief implied.** The orphan rule makes
  the repair per-codec, so it serves *"my codec also reads the tag I used to
  write"* and not *"I adopted `Json` and want my history back"* — which is the
  migration `Codec`'s own page had been using as its cautionary story. Sealing the
  trait stays open and is still the cheaper answer if no fourth codec appears.

Nine wrong implementations were built and run to prove the new tests are not
decorative; each is caught by exactly one test, and no test catches another's
defect. Every item passed `cargo xtask ci` (35 steps) before its commit.


### 2. `.kb/_intake` is full and owed an ingest

**Seven staged documents** plus redkiln's own scaffolded `README.md`, plus the
49-brief directory. `/redkiln:kb-ingest` authors the atoms; they must not be
hand-written. Three are this pass's decisions, three came from the postgres lane
(ADR-0024, ADR-0038, the poll-count calibration), and the seventh
(`2026-09-07-ratifications-discharged-…`) is the queue above with the three
things execution changed about it.

**Run the wave after the queue lands, not before**, which is now the case: the
ratifications file records decisions, and three of them mean something different
once executed. Atoms authored from the decisions alone would have been accurate
about what was chosen and wrong about what exists.

**The briefs are not in the default wave and that is correct.**
`remediation-2026-09-04-briefs/` is a *subdirectory*, and the command's default
glob is `.kb/_intake/*.md`, which does not descend. Thirty-two of the forty-nine
carry no deadline or one after this release; ingesting them would be one enormous
adjudication with no forcing event behind it. They stay staged. Note the
consequence, since `_intake`'s own README says a successful ingest **clears** the
directory: after the wave, a `_intake` that still holds that directory is
expected, not a failed run.

The next free ADR number is **0039**. ADR-0038 has a long-form record in
`references/adr/` and is staged here; it is not an atom yet.

### 3. `happenstance-neon` — **decided 2026-09-07**

Phase 10's Postgres half is in and meets the phase's exit criterion.
**Neon gates phase 10's own closure, not publication.** `0.2.0` ships on the
Postgres half; `RUNBOOK.md`'s phase 12 dependency row is repointed from the bare
`10` to name that half explicitly, because a number was what misled the last
reader of it.

ES-11 and ES-12 therefore ship `[PROVISIONAL]`, with their falsifier scheduled in
a later phase — which phase 12's own exit criterion permits in terms. Audit that
against `cargo xtask spec-trace` and the provisional ledger, never against prose.

Neon is **16 `todo!()`**, not six. Six is the count of `EventStore` *trait-method*
bodies; `event_store.rs` holds ten in all and `projection_store.rs` another six.
Whoever closes phase 10 should size it from the larger number.

Neon also carries ES-11's and ES-12's real falsifier: a one-shot-HTTP store with
no connection, no interactive transaction and no cursor. Those clauses stay
`[PROVISIONAL]` until it exists.

**Phase 10's remainder grew by one item on 2026-09-07**, and not by choice:
CF-18's new check found that `PostgresFixture` was declining `READ_FAULT` in the
testkit's words on the one adapter whose read genuinely pages. Arming it —
`pg_terminate_backend` on the reader's own backend between two `FETCH`es, or
closing the cursor beneath it — is now Postgres's owed work as well as Neon's.
Neither gates publication.

### 4. Benchmarks — re-run on a quiet machine

The suite is merged and the instrument works. The last full run is **withdrawn,
not committed**: the machine was under memory pressure and the `raw_*` control
arms — hand-rolled SQL sharing no code with happenstance — moved 1.34×, which is
proof of contention rather than of a regression. `codec_*` was 0.96×, so CPU-only
work was unaffected.

Its allocation counts *are* trustworthy (deterministic, machine-independent) and
showed one real result worth reproducing: a `limit-1` read of a 10,000-event log
went from **50,002 allocations and 3.5 MB to 7 allocations and 1.3 KB**.

**The run that *is* committed has the same defect, larger, and this changes what
the re-run is for.** `2ca9a63`'s own message records `MemoryEventStore`'s append
falling 3.2 M → 1.9 M events/s and the single-tag SQLite guard going 9.1 µs →
18.2 µs, on paths that change contained no line — *"this host ran about 1.7x
slower than the one that produced the previous page."* So both committed history
entries were taken on degraded hosts, and the page holds itself up by drawing
every comparison as a ratio **inside** one run.

That defence is correct and it is stated openly, but it means the task is not
*"replace a withdrawn run"* — it is **"produce the first run whose absolutes are
worth quoting."** Until one exists, the allocation counts are the only figures in
`results/` that survive being compared across runs, and the acceptance bar for the
next run is the `raw_*` and `codec_*` control arms holding against both committed
entries — not that the numbers look better.

`benchmarks/run.sh` takes 35–45 minutes; `--fast` is ~4 minutes and answers "did I
break the instrument". Contended-run artefacts are preserved in this session's
scratchpad for diffing.

**Run `--fast` from a clean tree and `git checkout -- benchmarks/results/`
afterwards.** It overwrites `results/raw/` with a partial run's output, and
there is no guard: `run.sh` refuses to write a `results/history/` entry for a
partial run, on the stated grounds that it must not displace a complete one under
the same filename, and the raw directory is not covered by that refusal.

### The blind spot this crate sits in, which cost three breakages on 2026-09-07

`benchmarks/` carries an empty `[workspace]` table so cargo cannot reach it from
the root, and `affected.rs`'s INERT list names it. Both are right — CF-34 forbids
a benchmark *result* gating a merge, and `run.sh` says it must never become a
gate step.

But the crate **calls the public API** (three `commit`/`commit_with` sites) and
**mounts the conformance suite** over its own SQLite fixture. It is a consumer,
and nothing in thirty-five gate steps compiles it. On 2026-09-07 it was found
carrying three independent breakages, all of them green:

- `commit`'s new return type, unnoticed for four commits — the **first real
  downstream break** this workspace has produced, and it was invisible;
- a seventh fixture inheriting `READ_FAULT`, missed by CF-18's new check because
  the check cannot run where it is not compiled;
- a `Cargo.lock` still recording the path dependencies at `0.2.0-alpha.1`,
  silently rewritten by every run and reverted by everyone who did not commit it.

**Left open deliberately, and it is a decision worth taking before the release.**
A `cargo check --benches --tests` on this crate produces no number, so it is
arguably not what CF-34 rejects — that clause is about a threshold nobody can
justify becoming a threshold everybody raises. Against that: adding a step to the
gate for a crate whose entire design is to sit outside it deserves an argument,
and the cheaper half-measure is to run the check in CI only, where it costs a
developer nothing. Whoever takes phase 12 should decide rather than inherit
this.

### 5. Then phase 12 itself

`RUNBOOK.md`'s phase 12 work list is accurate. The publish order is forced —
core → testkit → happenstance → sqlite → cloudflare — because both adapters
dev-depend on the testkit at the workspace version. There is **no release
workflow**; publishing is manual with the owner's token.

**The status-table tripwire is resolved — phase 10 is split into `10a` and
`10b`** (decided 2026-09-07). `10a` is the Postgres event store and reads
`done`; `10b` is the Postgres projection store and Neon and reads `in progress`.
Phase 12 depends on `10a`; phase 13 depends on both, because nothing decided
otherwise and splitting a node must not silently drop an edge.

Verified by simulation rather than by reasoning: inserting the `0.2.0` milestone
row phase 12 will need leaves **one** complaint, and it is phase 12's own
`not started`. So the release sequence is publish → mark phase 12 `done` → add
the milestone row, and the lint is satisfied at each step. `10b` is not in that
closure at all, which is the whole point.

What follows for whoever runs phase 12: add the milestone row **after** flipping
phase 12's state, not before, or the gate goes red between two commits that are
each individually correct.

The paragraph below is what that replaced, kept because the reasoning is the
record of why the split was the answer rather than the obvious alternatives.

**The tripwire, as it was found.** `xtask`'s `runbook_status_matches_the_registry`
holds the status table against the changelog on two axes, and axis 1 is: *a
released version vouches for every phase it waited on.* It finds the milestone
row whose name equals a dated `## [x]` heading and requires every phase in that
row's dependency closure to read `done`.

`CHANGELOG.md` already carries a dated `## [0.2.0]`, and the table has **no
`0.2.0` milestone row** — so the version is silently skipped and the axis is
currently held up by `0.2.0-alpha.1` alone. Add the row phase 12 obviously wants
and the lint fails: its closure reaches phase 10, which reads `in progress` and
which the 2026-09-07 decision says should stay that way, because Neon and now
Postgres's read fault are both unbuilt.

Three ways out, and the first is the trap. **Leaving the row out** keeps the
gate green and is exactly the omission the lint's own bail message warns
against — *"name the release in the milestone row, or delete this axis
deliberately rather than by omission"* — so the release would ship with the axis
holding nothing. **Marking phase 10 `done`** is a lie about two unbuilt things.
**Splitting the row** — `10a` Postgres `done`, `10b` Neon `in progress`, with the
`0.2.0` milestone depending on `7, 8, 10a` — is honest, parses (the lint matches
dependency entries as strings against the `#` column), and says structurally what
the phase 12 preamble had been saying only in prose. **That is the one that was
taken**, and the cost was the edit to the critical-path diagram plus a definition
of what `10a` can honestly claim.

That definition is the part worth carrying: `10a` is the **Postgres event
store**, not `happenstance-postgres`. The crate still holds five `todo!()` in its
projection store and `publish = false`, so a row named after the crate could not
have read `done` either, and the split would have bought nothing. What is
finished is the thing that answered F2-5 — the event store — and that is exactly
what publication needs. A split drawn along crate boundaries instead of along
finished work would have reproduced the original problem one row down.

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

**`redkiln doctor`'s telemetry integrity error is real and its diagnosis is
not.** It reports that
`.redkiln/telemetry/events/ryan-britton@happenstance@runs.jsonl` has an event
that does not hash to its own `id`, concludes *"the line was EDITED after it was
written — the one thing an append-only log must never show"*, and tells you to
recover the partition from git history.

Do not. The file has **one** line, introduced by **one** commit (`9f05da2`), and
`git diff HEAD --quiet` passes on it: git has no record of an edit, and
recovering from history would restore identical bytes. The mismatch is upstream
of the file — the digest was computed differently from the way it is now
verified, which is what a repository that has moved off the redkiln version it
pins should expect, and is the same cause as the four process-pack and six
template advisories.

Worth knowing for two reasons beyond this one file. `doctor` states a cause it
cannot distinguish from version skew, which is a discount to apply to its other
diagnoses. And it flags one partition out of the sixteen in that directory
because the other fifteen belong to other worktrees — so read its silence about
them as *not checked here*, never as *verified*.

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
