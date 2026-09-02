---
item: HS-S0041
stage: discover
created: 2026-08-12T13:02:04.087Z
updated: 2026-08-12T13:02:04.087Z
template_sig: 86ce4036
rendered_sig: b80e4eba
---

# Discover — The concurrency family green, and the 8-versus-64 discrepancy closed

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: take `event_store_concurrency_conformance!` green with ADR-0022's runtime seam in place (a captured `Handle`, not `NoRuntime` from every contender), and close the 8-versus-64 discrepancy by raising `CONTENDERS` with a stated reason **or** amending both proof artefacts. | `_storymap.md`, *Slices* table, row `race-model-and-durability` / `concurrency-family-and-contender-count` | Two deliverables that look unrelated and are not: raising `CONTENDERS` to 64 means 64 `rusqlite::Connection`s onto one file, which is where the runtime seam and the busy timeout are both first put under real load. |
| **AC-005** — the race is real and its size is a decision: the family is green, and the contender count is recorded either as a raised `concurrency::CONTENDERS` with a stated reason or as an amendment to **both** stated proof artefacts. "Leaving the discrepancy is a failure of this criterion, not a deferral." | `project.md`, AC-005; `RUNBOOK.md:2686-2696` | The runbook names the third option and calls it "the one that rots". Two artefacts read 64: `RUNBOOK.md:159` (the status table's phase-8 row) and `RUNBOOK.md:4217-4222` (the phase's proof artefact). |
| **AC-007** — the append is atomic and a probe-then-insert is rejected. This story's half is "the family that rejects the wrong one". | `project.md`, AC-007; `_storymap.md`, *Coverage* | The discrimination mechanism already exists: `Attempt::Rejected` for `AppendError::ConditionViolated`, `Attempt::Failed` for anything else (`crates/happenstance-testkit/src/concurrency.rs:219-231`). |
| `dependsOn: sqlite-fixture-and-whole-suite` (HS-S0040) — the fixture, and a green sequential suite to run the family on top of. | `_storymap.md`, *Merge order* item 3 | "Its three stories all run *on top of* a green sequential suite" (`_storymap.md`, *Slice coherence notes*). A red concurrency run against a red sequential suite diagnoses nothing. |
| The family's contenders are **bare OS threads under `std::thread::scope`**, each driving its own future with the testkit's own park-loop `block_on`, outside any ambient reactor — and the module says so, naming `sqlx` as the case in this workspace that cannot be driven that way. | `crates/happenstance-testkit/src/concurrency.rs:44-68` | `tokio`'s runtime context is thread-local, so `Handle::try_current()` fails inside a contender. This is the risk `project.md` names first. |
| It is **wider than the read path**: writers call `store.append(...)` inside `crate::block_on` on scoped threads, and the reader in `observe_while_writing` calls `collect(store.read(...))` on its own scoped thread — where a failed read is reported as a **sighting**, `"a concurrent read failed: {err}"`. | `_decomposition.md`, *Architecture brief*, §3; `crates/happenstance-testkit/src/concurrency.rs:884-895`, `:945-953` | A `NoRuntime` does not surface as an obviously-wrong failure; **it surfaces as a conformance verdict about atomicity.** That is the failure mode this story must be designed against, not diagnosed into. |
| `SqliteEventStoreError::NoRuntime` exists by design, turning a would-be `spawn_blocking` panic under a non-tokio executor into an ordinary stream error. | `crates/happenstance-sqlite/src/event_store.rs:26-32`, `:178` | Good design for a stray `futures::executor::block_on`; fatal here. ADR-0022's seam — option (a), capture a `Handle` at construction and keep `try_current()` as fallback — is what this story consumes. |
| The seam is solved **inside the adapter**, not by changing the testkit's bound: tightening `ConcurrentFixture` is a breaking change to a published crate for the benefit of one adapter, and `postgres-and-neon-stores` is the project that will have the evidence for whether the seam generalises. | `_decomposition.md`, *Architecture brief*, §3; `crates/happenstance-testkit/src/concurrency.rs:186` | Bounds this story's blast radius. It also means `concurrency.rs:60-68`'s "phase 10 will need the spawn to become a parameter" stays an open prediction rather than being pre-empted. |
| `CONTENDERS`'s own doc: "It is not a tuning knob: the rules assert set properties — *exactly one*, *all distinct* — that hold at any size above one." Eight was chosen as the smallest number at which the OS has to preempt. It also warns: "a fixture whose pool is smaller than this deadlocks rather than failing, and the suite has no way to tell them apart (CF-33 — there is no watchdog)." | `crates/happenstance-testkit/src/concurrency.rs:198-206` | Raising it rewrites no rule — but the constant is **workspace-wide**, so every fixture re-runs at 64: `MemoryFixture`, the mutant fixtures, the racers. And the pool warning is a direct hit on a file-backed store with a busy timeout. |
| The architecture brief recommends raising the constant, and states the cost it must weigh: "64 `rusqlite::Connection`s onto one file is a real file-descriptor and busy-contention claim this project has to verify rather than assume. If verification says otherwise, amend **both** artefacts." | `_decomposition.md`, *Architecture brief*, §9 | The recommendation is conditional on a measurement this story is the only one able to take. |
| CF-33 `[FROZEN]`: no rule may read a clock, and there is deliberately no watchdog anywhere in the family — "a store panicking under contention … into a silent timeout naming no rule." | `spec/SPECIFICATION.md:8236-8245`; `crates/happenstance-testkit/src/concurrency.rs:903-913` | "No test in this project may add a timeout, watchdog, or retry loop around a conformance rule. … If a rule hangs locally, that is evidence for ADR-0022's busy-timeout paragraph, not a reason to add `#[timeout]`" (`_decomposition.md`, *Testing brief*, §4). |
| Rules assert set properties and compare against positions the store assigned — e.g. "exactly one of {CONTENDERS} handlers that decided from the same snapshot…". | `crates/happenstance-testkit/src/concurrency.rs:327`, `:362-389` | Consistent with `CLAUDE.md`'s literal-position prohibition; raising `CONTENDERS` cannot introduce a positional assumption because none exists to scale. |

## Questions

Open questions to resolve before specifying.

1. **Raise `CONTENDERS` to 64, or amend both proof artefacts?** *Answered as a
   method, not as an outcome, because the answer depends on a measurement only
   this story can take.* Run the family against `SqliteFixture` at 64 and record
   what happens: if 64 connections onto one file is sustainable within the busy
   timeout and the platform's descriptor limits, raise the constant with the
   stated reason and accept that **every** fixture in the workspace re-runs at 64.
   If it is not, amend `RUNBOOK.md:159` **and** `RUNBOOK.md:4217-4222` together —
   both, never one. Leaving it is an AC-005 failure.
2. **Where does `spawn_blocking` get its runtime?** ADR-0022's, consumed here.
   This story is where option (a) is proven to work rather than argued to;
   architecture brief §3 is explicit that "what is not available is discovering
   the problem in a red concurrency run."
3. **Does the concurrency family get its own test target?** Deferred to `spec`.
   Testing brief §7 leaves the split open. A separate target buys process
   isolation, which matters more at 64 contenders than at 8.
4. **What busy-timeout value survives `CONTENDERS` writers?** Deferred to `spec`
   as a number, fixed here as a shape: **finite and generous**. An unbounded
   handler converts a livelock into a hung CI job that names no rule, and there is
   no watchdog to catch it.
5. **Is a `SQLITE_BUSY` that outlives the timeout a conformance failure or an
   adapter finding?** Answered: an **adapter finding**, and the ledger must say
   which. `Attempt::Failed` is a legitimate outcome the rules count, so a store
   that times out under contention will fail "exactly one winner" and look like an
   atomicity defect. Distinguishing the two is this story's diagnostic obligation.
6. **Does the testkit's `ConcurrentFixture` bound change?** Answered: **no.** The
   seam is solved inside the adapter; changing a published crate's bound for one
   adapter is `postgres-and-neon-stores`' evidence to gather, not this project's
   to pre-empt.
7. **The append-condition SQL strategy.** Consumed from ADR-0022. If the family
   shows the chosen strategy cannot elect exactly one winner, that is a finding
   for the ADR queue — a superseding atom — not a silent change of strategy.

## Decision

The concurrency family is the only place the adapter's two riskiest decisions are
actually exercised: whether a blocking driver can get a runtime when the caller is
a bare OS thread, and whether `BEGIN IMMEDIATE` plus a busy timeout produces
exactly one winner rather than a hang. A green sequential suite says nothing about
either. This slice takes `event_store_concurrency_conformance!` green against
`SqliteFixture` and closes the count discrepancy that both of phase 8's stated
proof artefacts have carried since it was noticed. The spec will cover: mounting
the family against `SqliteFixture` with ADR-0022's captured `Handle` in place, so
no contender returns `NoRuntime`; a busy timeout that is finite and generous, with
no watchdog, timeout attribute or retry loop added anywhere around a rule (CF-33);
running the family at 64 contenders and recording the result; and then **either**
raising `crates/happenstance-testkit/src/concurrency.rs:206` with the stated
reason and the workspace-wide re-run cost acknowledged, **or** amending both
`RUNBOOK.md:159` and `RUNBOOK.md:4217-4222` — never one of the two. This story
adds **no conformance rule**; it makes an existing family run against a new
implementation, so the literal-position bar is vacuous, and the family's rules
already assert set properties over positions the store assigned. No `[FROZEN]`
clause is amended: CF-33 is frozen and is complied with by *not* adding the
watchdog a hang will tempt someone to add.

## The wrong implementation

**The mutant: the concurrency family is simply never mounted.** No
`event_store_concurrency_conformance!` invocation in any target under
`crates/happenstance-sqlite/tests/`. Nothing fails. `cargo test -p
happenstance-sqlite` is green, `cargo xtask ci --fast` is green, the sequential
suite is green, all of AC-001 – AC-004 have evidence, and the project reports
done. The family that would have found the runtime seam and the busy timeout never
ran, and *nothing anywhere records its absence* — `for_each_event_store_rule!`'s
orphan check (`crates/happenstance-testkit/src/registry.rs:413-423`) is about
rules missing from the enumeration, not about a macro an adapter declined to
invoke. This is the failure the testing brief §4 names in terms: "the same is true
of discovering it in a *green* run that quietly skipped the family." The control is
the AC-005 ledger entry citing the family's own per-rule output, and a reviewer
checking the test target exists — there is no automated check, and saying so is
more useful than pretending there is.

**The second mutant, and it is the dangerous one because it is loud in the wrong
place: an adapter that returns `NoRuntime` from the read path under contention.**
This is the *default* behaviour of the skeleton as written
(`crates/happenstance-sqlite/src/event_store.rs:26-32`) if ADR-0022's seam is not
implemented. It does not present as "the store could not get a runtime". The
reader in `observe_while_writing` runs on its own scoped thread and reports a
failed read as a **sighting** — `"a concurrent read failed: {err}"`
(`crates/happenstance-testkit/src/concurrency.rs:945-953`) — so the run fails a
rule *about batch atomicity*, and an implementer spends a day looking at
`BEGIN IMMEDIATE`. The negative control is not a mutant to write: it is the
sequencing discipline that ADR-0022 settles the seam **before** this story starts,
which is why `adr-0022-append-condition-strategy` is a hard predecessor of the
whole implementation rather than a document written alongside it.

**The third mutant, which the repository has already named and pre-emptively
rejected: leave `CONTENDERS` at 8 and leave both artefacts reading 64.** No code
changes, every rule passes, the proof artefact "the concurrency macro green at 64
contenders" is quoted in the exit criteria and is false. `RUNBOOK.md:2686-2696`
calls this "the third option and it is the one that rots", and AC-005 restates it
as a failure rather than a deferral. The control is the ADR-0022 paragraph plus
the artefact diff, both checked by review — and, if the constant is raised, the
workspace-wide consequence must be *verified* rather than assumed, because
`CONTENDERS`'s own doc warns that a fixture whose pool is smaller than the count
"deadlocks rather than failing, and the suite has no way to tell them apart"
(`crates/happenstance-testkit/src/concurrency.rs:198-206`). A file-backed store
with a finite busy timeout is exactly the fixture that warning is about.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
