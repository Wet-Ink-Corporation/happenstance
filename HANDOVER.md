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

## State, as of `b0d9e67`

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
been through any of them. The push is `3c4c728..b0d9e67`.

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

**The tree describes `0.2.0` as published, in the present tense, in five places** —
`README.md`'s status banner and table, `SECURITY.md`'s scope and supported-versions
table, `CHANGELOG.md`'s dated `## [0.2.0]` section, `Cargo.toml`'s version, and
`spec/SPECIFICATION.md` §1.3. It is not published.

That was a deliberate choice: the documents were written when the release looked
imminent, the release was then deferred, and the owner chose to leave them rather
than revert. It is contained by the repository not being open — though note the
containment is weaker than it reads: visibility is **`INTERNAL`**, not private, so
the audience is everyone in the organisation rather than only whoever picks the
tree up.

**There is a sixth, and it is the one that is an action rather than prose.**
`CHANGELOG.md`'s `[0.2.0]` section and `SECURITY.md`'s supported-versions table
both state that `0.2.0-alpha.1` **is yanked**. It is not — all three live crates
still resolve it. Yanking is on phase 12's list; until it happens, that sentence
is the only one of the six a `cargo add` can disagree with.

`CHANGELOG.md` also carries an `## [Unreleased]` section (the ES-10 poll-schedule
change) sitting *above* the dated `## [0.2.0] — 2026-09-06`. Publishing means
folding one into the other and correcting the date — a small thing that is easy
to lose because the section below it already looks finished.

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

### 1. The implementation queue from the seventeen ratifications

Ratifying is not landing. Six items, roughly by size:

1. **`cf-18` / candidate B3** — **costed on 2026-09-07, and the costing changed
   the item.** Read the section below before touching it: B3 *as the brief words
   it* is vacuous, because `Capability::declined` already refuses an empty
   reason. The version that was taken asserts something different and larger.
   Estimated **1–1.5 days**, not unbounded. The fallback (narrow CF-18 to option
   A) was **not** taken and remains written down.
2. **`empty-decision-outcome` / Option 3** — a second success shape on the command
   path, `#[must_use]`, and every call site and doctest naming the existing one.
3. **`tags-scope-agreement` / A + B** — a run-time refusal between
   `command.rs:312` and `:318`, plus two doctest repairs. **The briefs' line
   numbers are stale**: they were pinned before `835028f` added seven doc lines
   to `commit_with`, so every `command.rs` citation in the brief corpus is `+7`
   and `domain.rs`'s are roughly `+50`. Repoint by anchor, as always; the point
   here is that the briefs are not a fixed frame of reference either.
4. **`codec-foreign-tag-resolution` / A** — a defaulted method on `Codec`.
   Option C (sealing) stays open and is the cheaper answer if no fourth codec
   ever appears.
5. **`stringified-throw-visibility` / B** — `pub(crate)`, and the crate root's
   invitation withdrawn with it.
6. **`op-read-non-exhaustive` / A** — the attribute, landing with the `to` field.
   No constructor: D was considered and declined.

`query-partition-public-surface` (1A) and `read-page-budget` (A) ratify what is
already landed and oblige nothing.

#### `cf-18` / B3, costed — read this before implementing item 1

**The brief's wording describes a state that cannot be constructed.**
`Capability` is `Capability(Option<&'static str>)`
(`crates/happenstance-testkit/src/contract.rs:983`): `SUPPORTED` *is*
`Self(None)`, so declining and carrying a reason are the same act, and
`Capability::declined` already refuses an empty string at `contract.rs:1031`
with an `assert!` that says why. A test asserting *"declined without a stated
reason"* would pass on every fixture that will ever exist. **B3 as written is
decorative**, which is the corollary `CLAUDE.md` warns about, arriving from the
direction nobody watches.

**The two real gaps are named by the trait's own documentation.**
`contract.rs:1022-1028` says the `assert!` above is a `const fn` precondition
and that for an *associated* const it is evaluated lazily at **codegen** — so
`cargo build` and `cargo test` catch an empty reason and `cargo clippy` does
not, and nothing forces a default run to read the const at all. The larger gap
is **declension by inheritance**: `Fixture::MID_BATCH_FAULT`,
`Fixture::READ_FAULT`, `ProjectionFixture::RESET_REFUSAL`,
`ProjectionFixture::COMMIT_FAULT` and `ProjectionFixture::SECOND_HANDLE` all
*default to a declension*, so a fixture that says nothing declines them and the
`SKIP` line then prints **the testkit's prose as though it were the adapter's
own account of its store**. That is precisely the misrepresentation CF-18's
`Rejects:` paragraph exists to prevent, and two fixtures are in that state
today: `MemoryFixture` inherits `MID_BATCH_FAULT` and `PostgresFixture`
inherits `READ_FAULT`.

**What was decided.** B3 in the **stated-not-inherited** form, emitted from the
**suite macro body**, with the two inherited declensions restated in the same
change. CF-18 stands as written and is not narrowed; ES-35's `[PROVISIONAL]`
marker is undisturbed.

**Why it is cheap, which was the brief's own open question.** The brief asked
whether `capability_skips_are_reported`'s assertion could be macro-emitted
without dragging the mutation-coverage harness into an adapter's build. **It
cannot, and it does not need to be** — those are two different assertions. That
meta-test asserts over *rule outcomes*, which structurally needs the driving
harness, `catch_unwind`, and an adversarial fixture: about 13,000 lines, all of
it `tests/`-local. B3 asserts over *associated consts* and needs one `pub fn` in
`src/contract.rs`, reached through a generic bound on `Fixture`. Nothing from
`tests/mutation_coverage/` enters an adapter's build.

**Why this is the right bar rather than an invention.** Two adapter authors
already hand-wrote it, independently and without a clause telling them to:
`happenstance-postgres`'s `capability_constants_are_answered_not_defaulted`
(`tests/postgres_conformance.rs:411`), whose own doc calls it *"the belt to
`Capability::declined("")`'s suspender, which fires at codegen"*, and four tests
in `happenstance-cloudflare`'s `tests/fixture_contract.rs`, including
`mid_batch_fault_is_restated_not_inherited`. B3 is *"generalise a test two
adapters already wrote, so the third gets it for free."*

**The one thing it will not cover, and it must be written into the clause.**
`memory_conformance_wasm.rs` and `durable_object_conformance.rs` invoke with
`emit = __emit_wasm`, and a plain `#[test]` emitted into those modules is
neither run nor listed by `wasm-bindgen-test-runner`. Emitting from the twelve
*emitters* instead would cover wasm32 — and would break worse: CF-23 makes the
emitter the **caller's**, so a third-party emitter would silently drop the
check, which is the same incentive inversion CF-18 exists to close. The suite
body is the right seam and the wasm32 blind spot is the price, stated rather
than absorbed.

**Do not let the new test be decorative either.** It must go red against a
fixture that inherits a declension, and that wrong fixture belongs in the
testkit's own `tests/`. It must also be observable in a **default**
`cargo test -p happenstance-sqlite --test conformance`, with no `--show-output`
— that is the whole point of the item, and the one thing a passing test cannot
tell you about itself.

### 2. `.kb/_intake` is full and owed an ingest

**Six staged documents** plus redkiln's own scaffolded `README.md`, plus the
49-brief directory. `/redkiln:kb-ingest` authors the atoms; they must not be
hand-written. Three of the six are this pass's decisions, three came from the
postgres lane (ADR-0024, ADR-0038, the poll-count calibration).

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
