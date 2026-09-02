---
item: HS-S0035
stage: discover
created: 2026-08-12T13:01:58.228Z
updated: 2026-08-12T13:01:58.228Z
template_sig: 86ce4036
rendered_sig: 033668a9
---

# Discover — ADR-0022, written first and carrying a measured number

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: measure the three append-condition candidates and the tag-storage options against real SQLite through that harness, and commit ADR-0022 — atom plus long record — carrying the winner, the losers, the amended schema, the runtime seam, the `index_arms()` rejection and a number. | `_storymap.md`, *Slices* table, row `bench-harness-and-adr` / `adr-0022-append-condition-strategy` | Six payloads, one record. The measurement is only the first; the other five are decisions the implementation stories read as substrate. |
| **AC-013** — the decision record carries a number: ADR-0022 is committed **before** the implementation, states the strategy with the alternatives that lost, and quotes a measured benchmark figure rather than a preference. | `project.md`, *Acceptance criteria*, AC-013; `RUNBOOK.md:4227-4228` | "Before" is checkable in git, and it is the half most likely to be lost under schedule pressure — this is the largest single adapter on the trunk at ten runbook-days (`RUNBOOK.md:4235`). |
| `dependsOn: benchmark-harness` (HS-S0034) — it supplies `event_store_benchmarks!`, the instrument the figure comes from. | `_storymap.md`, *Why the three foundations are foundations*: "cannot quote a measured figure without it" | The dependency is real rather than sequencing convenience: with no harness, "a number" degrades into a hand-timed loop nobody can re-run. |
| DR-06: ADR-0022 answers **one** question and names the append-condition alternatives that lost — the three candidates at `crates/happenstance-sqlite/src/lib.rs:57-61` and the tag-storage options at `:62-64`. | `project.md`, *Derived requirements*, DR-06 | The candidates are already written down in the crate's own published module doc: `BEGIN IMMEDIATE` + `EXISTS` probe; conditional `INSERT … SELECT … WHERE NOT EXISTS`; a monotonic-position guard. Tag storage: join table vs canonical serialised blob vs SQLite JSON1. |
| The crate's own doc states the coupling: "Which one wins depends on how tag matching is indexed." | `crates/happenstance-sqlite/src/lib.rs:57-64` | The two open questions are **not** independent, so they cannot be measured or decided separately. A benchmark matrix, not two benchmarks. |
| Architecture brief §12 enumerates what ADR-0022 must carry: strategy + losers + figure; tag storage; the amended schema and why the crate's sketch was wrong; the runtime seam and its rejected alternative; the `index_arms()` rejection and its re-open trigger; busy timeout, `synchronous`, journal mode; the `CONTENDERS` resolution; and the `&[Event]` vs `Vec<Event>` verdict. | `_decomposition.md`, *Architecture brief*, §12 | Nine items. Any one omitted becomes a decision made silently by whoever writes the code next. |
| Architecture brief §3 recommends option (a) — capture a `tokio::runtime::Handle` at construction, prefer it, keep `try_current()` as fallback — and asks the ADR to record (b) (inline on the calling thread) as the alternative that lost. | `_decomposition.md`, *Architecture brief*, §3 | The seam is not a read-path detail: writers call `append` inside `crate::block_on` on scoped threads (`crates/happenstance-testkit/src/concurrency.rs:884-895`), so `NoRuntime` would reach every contender. |
| ADR-0012 names **this** measurement as what lifts its own `[PROVISIONAL]` marker, and fixes the successor question as binary: keep `events: &[Event]`, or move to `Vec<Event>`. Nothing else. | `.kb/decisions/0012-append-shape-and-preconditions.md`; `_grounding.md`, first bullet | ADR-0022 owes a *finding* here, not a change. Inventing a third option would be settling another ADR's question in passing. |
| ADR-0011 already requires a read snapshot ceiling for any adapter issuing more than one statement per `read`, and `PAGE_SIZE = 512` makes this adapter one. | `.kb/decisions/0011-read-laziness-and-isolation.md`; `crates/happenstance-sqlite/src/event_store.rs:81` | The ceiling is *discharge of an accepted ADR*, so ADR-0022 records it as inherited, never as a choice it made. |
| `EventStore` is `[FROZEN]` at `spec/SPECIFICATION.md:371`, and `Query::index_arms()` / `IndexArm` **do not exist** — the name is inherited from an evaluation *proposal*, and `RUNBOOK.md:4203-4206` restates it verbatim as if it did. | `_grounding.md`, *Tensions*; `crates/happenstance-core/src/query.rs`; `references/evaluation/ARCHITECTURAL-EVALUATION.md:827` | ADR-0022 must state the rejection explicitly, "because the runbook's own work item names the API and a reader will otherwise think it was forgotten" (`_decomposition.md`, *Architecture brief*, §5). |
| CF-14 names `PRAGMA synchronous = OFF` **by name** as a wrong implementation the reopen rule exists to reject. | `spec/SPECIFICATION.md:7471-7490` | The `synchronous` setting is not a tuning knob this ADR may leave unstated; it is the parameter a named clause is watching. |
| `CLAUDE.md`'s two-places rule: a decision lives as an **atom** under `.kb/decisions/` (immutable once accepted, validated against `HEAD`) *and* as the long record under `references/adr/` carrying transcripts, rejected alternatives and measurement tables. | `CLAUDE.md`, *Where the work lives*; `redkiln validate --kb` | The benchmark table is exactly the kind of content the atom cannot hold. One artifact is not this story's deliverable; two are. |
| Neither `.kb/decisions/0022-*.md` nor `references/adr/0022-*.md` exists today. | `_grounding.md`, final bullet of *Accepted decision atoms* | This story **authors**; it consumes nothing numbered 0022. |
| Atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`, not by hand — hand-writing them "produces the directory layout of the process without the process," which is why the first attempt was reverted (`0269720`). | `CLAUDE.md`, *Where the work lives* | The story's spec must route the atom through intake, not write it into `.kb/decisions/` directly. |

## Questions

Open questions to resolve before specifying.

1. **Which append-condition strategy wins?** *Deliberately open, and it is the
   genuinely open one in this project.* This story exists to close it **with a
   measurement**, not with a preference, and nothing in discover pre-empts it.
   What is fixed here is only the shape of an acceptable answer: a ranked
   comparison of the three candidates at
   `crates/happenstance-sqlite/src/lib.rs:57-61`, crossed with the tag-storage
   options at `:62-64` because the crate's own doc says the two are coupled,
   taken on a real file with the journal mode and `synchronous` setting the ADR
   also fixes.
2. **Which tag storage wins?** Same answer, same matrix, and not separable from
   question 1.
3. **Where does `spawn_blocking` get its runtime?** Answered in the architecture
   brief with a recommendation — (a) capture a `Handle` at construction — and
   this story is where the recommendation becomes a decision with (b) recorded as
   the loser. The implementer may argue for (b) in the ADR; what is *not*
   available is discovering it in a red concurrency run
   (`_decomposition.md`, *Architecture brief*, §3).
4. **Does `Query::index_arms()` land in `happenstance-core`?** Answered: **no** —
   decompose privately inside `happenstance-sqlite`. One implementor is not a
   spread (`CLAUDE.md`), and the re-open trigger is named rather than left to
   memory: if `postgres-and-neon-stores` independently needs the same
   decomposition, that is two unlike shapes agreeing and *that* mints the API, as
   its own ADR. The ADR must say this out loud.
5. **`&[Event]` or `Vec<Event>`?** ADR-0012 made this binary and named this
   measurement as the thing that answers it. ADR-0022 records the **finding**;
   changing the port's signature is not this story's and not this project's.
6. **Busy timeout, `synchronous`, journal mode — what values?** Answered as
   *shape*, deferred as *numbers* to this story's own measurement: finite and
   generous, never infinite, because there is no watchdog anywhere in the suite
   and an unbounded busy handler converts a livelock into a hung CI job that
   names no rule (`_decomposition.md`, *Architecture brief*, §6; CF-33).
7. **`CONTENDERS`: raise to 64, or amend both proof artefacts?** Recorded here as
   ADR-0022's to carry (architecture brief §9 recommends raising) but **verified**
   by `concurrency-family-and-contender-count` (HS-S0041), which is the story that
   can actually run 64 `rusqlite::Connection`s onto one file. The ADR states the
   resolution; it must not state it as measured if it was not.
8. **CF-40's clause home.** Not this story's, not this project's. Record and
   escalate to the ADR queue; `.kb/open-questions/cf-40-fixture-limits-ownership.md`
   says plainly that the KB has no standing to resolve it.

## Decision

Nothing in `crates/happenstance-sqlite/` can be written honestly until the append
condition's SQL shape, the tag layout it depends on, and the runtime seam that
decides whether the concurrency family can run at all are settled — and the
runbook requires that settlement to quote a measurement rather than a preference.
This slice produces ADR-0022 in the two places `CLAUDE.md` requires: an atom under
`.kb/decisions/`, routed through `.kb/_intake/` and `/redkiln:kb-ingest` rather
than hand-written, and the long record under `references/adr/` carrying the
benchmark table an atom cannot hold. The spec will cover: the benchmark matrix
(three append-condition candidates × the tag-storage options, on a real file, at
stated journal/`synchronous`/busy-timeout settings) and the figure it yields; the
winner and both losers; the amended schema of `RUNBOOK.md:4178-4187` and why the
crate's own published sketch at `crates/happenstance-sqlite/src/event_store.rs:36-54`
is wrong; the runtime seam with option (b) recorded as the alternative that lost;
the explicit rejection of `Query::index_arms()` with its named re-open trigger;
the `CONTENDERS` resolution; and the binary `&[Event]` vs `Vec<Event>` **finding**
ADR-0012 asked this measurement for. This story adds **no conformance rule** and
touches no code, so the literal-position bar is vacuous here. It amends no
`[FROZEN]` clause: `EventStore`'s frozen cell (`spec/SPECIFICATION.md:371`) is the
reason `index_arms()` is rejected rather than added, and the clauses this record
discharges — CF-14, CF-17, ES-35, CF-34 — are `[DEFERRED]` or `[PROVISIONAL]`,
with their verdicts owned by `reopen-negative-control-and-durability-verdicts`.

## The wrong implementation

**The mutant: an ADR whose number was measured against
`SqliteEventStore::open_in_memory()`.** It satisfies every check that exists. The
atom is committed before the first non-`todo!()` SQL body, so the git-order test
passes. It cites a figure, so AC-013's "carries a number" passes.
`redkiln validate --kb` is green and `redkiln doctor` is clean. A reviewer reading
the record sees three candidates, a winner, two losers and a table.

And the number is meaningless, because a private in-memory database
(`crates/happenstance-sqlite/src/event_store.rs:132`) has no journal file, no
`fsync`, and **no second connection** — a private in-memory database is
per-*connection*. Every cost the append-condition decision actually turns on is
absent: `BEGIN IMMEDIATE`'s write-lock hold time, WAL versus rollback-journal
commit cost, and the contention that makes "conditional append under contention"
a distinct measurement from throughput. The three candidates come out within
noise of each other, the tag-storage options come out ranked by CPU rather than
by I/O and index-page locality, and the ADR picks a winner on a substrate that
does not exist in production. Nothing downstream ever re-checks it: ADR-0022 is an
accepted decision atom and therefore **immutable** — correcting it means writing a
new atom that supersedes it (`CLAUDE.md`, *Where the work lives*), so a wrong
number ships permanently.

This mutant has no home in the testkit's `tests/`, and that is the honest answer
rather than an evasion: `crates/happenstance-testkit/tests/mutation_coverage/`
holds wrong *stores* falsifying conformance rules, and `mutant_registry_is_exhaustive`
rejects a row whose `fails` list is empty
(`crates/happenstance-testkit/tests/mutation_coverage/racers.rs:10-18`). A record
that measures the wrong substrate fails no rule; that is exactly what makes it
dangerous. The control is therefore in the record itself, and the spec must
require it: **the ADR states the measurement's substrate as data** — file-backed
path, journal mode, `synchronous` setting, busy timeout, connection count and
contender count — so that a reader can tell whether the number was taken where
the adapter will live. A figure with no stated substrate is a preference wearing a
decimal point.

**The second mutant, and it is the one the repository has explicitly legislated
against: the ADR written as a side effect of the code change.** Concretely — the
implementation lands first, `git commit` carries both the SQL and a new
`.kb/decisions/0022-*.md` describing what was just written, and the atom is
hand-authored straight into `.kb/decisions/` rather than through `.kb/_intake/`.
Everything is green: `validate --kb` checks frontmatter conformance and
accepted-decision immutability against `HEAD`, not authorship order, and the file
exists so AC-013 reads as satisfied. What is lost is the entire function of the
record — it documents a decision instead of making one, and the alternatives
"that lost" are reconstructed from a result rather than measured. `CLAUDE.md`
names the hand-authoring failure directly ("the directory layout of the process
without the process", reverted at `0269720`). The control is checkable and the
spec must name it: the atom's commit precedes the first commit that removes a
`todo!()` from `crates/happenstance-sqlite/src/`, and the intake file is in the
history.

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
