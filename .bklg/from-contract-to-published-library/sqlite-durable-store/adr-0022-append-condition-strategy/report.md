---
item: "HS-S0035"
stage: report
created: "2026-08-16"
updated: "2026-08-16"
---

# Report — ADR-0022, written first and carrying a measured number

## Findings Ledger

**Nine of nine ACs satisfied, each by a real artefact with real, reproducible
evidence.** 465 passing tests across five targets in an out-of-workspace crate,
three results tables backed by five raw output files from one `./run.sh`, one
long-form decision record, two staged KB sources, and a workspace whose only
compiled change is in a crate cargo cannot see.

**Mount point:** `.kb/decisions/0022-append-condition-strategy.md`, reached in
two moves. This story stages
`.kb/_intake/0033-adr-0022-append-condition-strategy.md` and
`.kb/_intake/0034-append-condition-experiment-2026-08.md`; the atom is minted by
a **human-invoked** `/redkiln:kb-ingest`, which also indexes it in
`.kb/maps/decision-map.md`. That handoff is recorded as a named deliverable in
the staged source, because a story that "finishes" without it leaves the corpus
without the atom every downstream story loads.

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — the number comes from somewhere real, and every arm is conformant first | **Met** | `experiments/append-condition/` — three candidates over `rusqlite` (`src/strategy.rs:100`, `:173`, `:286`) sharing one schema, driven by `event_store_benchmarks!` **verbatim**. `cargo test --test candidates_are_conformant` → **445 passed, 0 failed**. Empty `[workspace]` at `Cargo.toml:12`; `git diff --name-only HEAD -- Cargo.toml Cargo.lock crates/` is EMPTY |
| **AC-002** — durability is enforced, not assumed | **Met** | `Durability::read_back` (`src/durability.rs:144`) reads the pragmas off the live connection; `require_shippable` (`:166`) refuses under `synchronous = OFF`. The control is **forced to fire**: `tests/durability_settings_are_enforced.rs` — 3 passed, one of which sets the wrong pragma and asserts the refusal, one of which sets `DELETE` and asserts the reader reports the mode *in force*. Every emitted row carries `journal_mode=wal synchronous=normal busy_timeout_ms=5000 sqlite=3.53.2` |
| **AC-003** — all three tag storages measured under the write lock, and migration 1 specified | **Met** | `results/tag-storage.md`: join table 556 µs probe / 10,744 µs selective read, canonical blob 623 / 33,992, JSON1 434 / 49,766, at 50,000 events, round-robin. The blob arm is **measured, not dismissed**. The record's §7 specifies migration 1 outright — `event_type` covering with the key left `(tag, position)`, `tag_cardinality`, the `EventId` pair `UNIQUE` together, `recorded_at` read back, `AUTOINCREMENT`, `StoreId` minted once — and cites `event_store.rs:36-54` as the wrong sketch. That file is **unchanged** |
| **AC-004** — one winner, two named losers, figures attached, driver ratified | **Met** | Record §4 chooses the monotonic-position guard; §5 names both losers with figures at two log sizes and two boundary shapes; §2 records the driver as **ratified rather than decided**. Every figure is a `SEQUENTIAL` row in `results/raw/contention.txt`, reproducible by `run.sh`. `RUNBOOK.md:302` is struck through and marked **Written** in the shape rows 0008/0009/0010/0016 use |
| **AC-005** — the 64-contender measurement, supplied and not applied | **Met** | `results/contention-64.md`: 64 real OS threads on 64 connections, 10 races per arm, **10/10 committed, 630 rejected, busy=0, failed=0**; 8 measured beside it so the record states what the *raise* costs (10.9x–20.0x). Record §12 states it as a recommendation. `git diff --name-only HEAD -- crates/happenstance-testkit/` is EMPTY; `CONTENDERS` is still 8 |
| **AC-006** — one question, seven consequences, each with its loser | **Met** | One `Settles:` line; §6–§12 are the seven, each naming what lost. The runtime seam is (a) with `try_current` fallback and **`NoRuntime`'s fate spelled out under the winner** (§9). `index_arms()` is **rejected** with its re-open trigger named (§10), and `rg -n "index_arms" crates/happenstance-core/src` still returns **nothing**. Three numeric pragma values (§11) |
| **AC-007** — two non-verdicts, fenced, with owners | **Met** | §13 quotes ADR-0012's falsifier item 1 **verbatim**, states that three candidate stores are not two builds of one adapter, records what the experiment *did* observe as an observation, and states that **no story in this map owns producing it** — escalated as its own ADR-queue row at `RUNBOOK.md:303`. §14 cites CF-40's open question and leaves it open; `git diff --name-only HEAD -- .kb/open-questions/` is EMPTY |
| **AC-008** — the atom is minted by the process, not by hand | **Met** | `git diff --name-only HEAD` contains **no** `.kb/decisions/` or `.kb/maps/` path and exactly **two** new `.kb/_intake/` paths (not the README). Both carry valid `KbFrontmatter`; the evidence source is a separate `reference` atom in `.kb/reference/position-visibility-experiment-2026-08.md`'s shape, so the decision can be superseded without invalidating the numbers. `redkiln validate --kb` passed; `redkiln doctor` reports exactly the six expected `template-drift` advisories |
| **AC-009** — the record precedes the implementation, mechanically | **Met** | `git diff --name-only HEAD -- crates/ spec/ Cargo.lock Cargo.toml .redkiln/config.yaml` returns **EMPTY**. `#![allow(clippy::todo)]` is still at `crates/happenstance-sqlite/src/lib.rs:83`; the rule count is unchanged at 89 registered names. `cargo xtask affected --base main` and `cargo xtask ci --fast` both green |

**Deferred: nothing, and two things deliberately left open with owners.** No AC
is blocked, no test is skipped, no behaviour is stubbed. ES-17's marker and
CF-40's clause home are **non-verdicts by design** (AC-007), each recorded with
its owner rather than absorbed — and ES-17's gap is escalated to the ADR queue as
a row of its own, because phase 8 as planned does not discharge an obligation
ADR-0012 assigned to it.

## Acceptance

All nine `AC-###` rows in `_ledger.md` are `satisfied: true` with cited evidence
— a `file:line`, a named passing test, or a named command and its output.
`redkiln verify --grain story` reads that block.

The story's own error conditions, all seven answered:

| EC | Answered by |
| --- | --- |
| EC-001 (`synchronous = OFF`, or a journal mode that cannot ship) | `Durability::require_shippable` aborts before measuring; forced to fire by `a_connection_under_synchronous_off_is_refused` |
| EC-002 (`SQLITE_BUSY` under 64 contenders) | Recorded as a **measurement**: `busy = 0` in every row, with a finite 5,000 ms timeout. `Outcome::Busy` exists in the control specifically so a non-zero would be legible |
| EC-003 (an arm that is fast and non-conformant) | Conformance runs first, 445 tests over 5 arms; the README states that such an arm's figure is discarded |
| EC-004 (arms within noise) | Fired, twice. The commit path and both contended counts are reported as **ties** with the tie-break named (record §5), and the noise floor is measured rather than asserted — the unfiltered read, which touches no tag storage, spans ±7% |
| EC-005 (`/redkiln:kb-ingest` fails, or `_intake` non-empty) | Not reachable yet: the handoff is a human's and is written down as a deliverable. `redkiln validate --kb` is green on the staged files |
| EC-006 (the slice-mate's surface differs) | Not reachable: the slice-mate landed first in this same slice, and `tests/measure.rs` binds only its stated public entry point — the arms and the caller-supplied emitter |
| EC-007 (64 connections exhaust a platform limit) | Did not occur on this platform, and is recorded as a fact about this platform rather than a guarantee (`results/contention-64.md`, finding 1) |

## Knowledge Harvest

**Four things worth promoting at closeout**, none of them settled here.

1. **On a shared host, measure arms round-robin inside one process.** Three
   sequential tests were written first and thrown away: two runs an hour apart
   disagreed about the *ordering* of a read, because the machine slowed down and
   whichever arm ran late wore it. Interleaving is what makes a ratio mean
   anything. This is a `playbook` candidate and it generalises past SQLite —
   `postgres-and-neon-stores` will need it.
2. **Subtract a baseline, and then check the subtraction is resolvable.** Guard
   cost was measured as *conditional minus unconditional append*. At 5,000 events
   it resolves; at 50,000 the commit rises to ~18 ms while the guard stays in the
   tens of microseconds, and the difference of two medians three orders of
   magnitude larger is noise. **Reporting that as noise is the finding.**
3. **Measure the path where the arms structurally differ.** All three strategies
   were indistinguishable until the *rejection* path was timed — where no commit
   happens, so the dominating cost leaves the measurement, and where the arms
   genuinely differ in statement count. A benchmark that only measures the happy
   path measures what all the candidates share.
4. **CF-33 and CF-23 land on the same seam, and this story is the second
   observation of it.** The testkit may not read a clock and the wrapper is a
   parameter; both independently force *harness produces counts, caller's emitter
   produces durations*. This story then needed the emitter seam for a second
   reason the first did not anticipate — interleaving — which is evidence that
   the parameter is load-bearing rather than tidy.

**Two candidates for the ADR queue rather than for `.kb/`**, both raised by this
story and neither settled by it:

- **The unowned ES-17 measurement** (record §13, `RUNBOOK.md:303`). ADR-0012
  assigned it to phase 8; phase 8's map does not produce it. It needs a number
  and an owner, or an explicit deferral to a new phase.
- **The multi-tag probe path.** A two-tag consistency boundary costs ~200x a
  single-tag one at 50,000 events, on every strategy. `tag_cardinality` and
  most-selective-tag-first probing are the answer this record specifies, and
  whether they are *sufficient* is unmeasured — the experiment measured the
  problem, not the fix.

**One correction fed forward to the implementing stories**, already in the
record: the published schema sketch at
`crates/happenstance-sqlite/src/event_store.rs:36-54` is wrong and stays wrong
until `schema-migration-and-identity` and
`instrument-markers-removed-and-gate-green` correct it. It is documentation that
is known to be false, which is worse than absent documentation, and the record is
where the correction became citable.
