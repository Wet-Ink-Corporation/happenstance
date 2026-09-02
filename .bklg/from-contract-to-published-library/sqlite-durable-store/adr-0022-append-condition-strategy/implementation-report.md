---
item: "HS-S0035"
stage: implement
created: "2026-08-16"
updated: "2026-08-16"
---

# Implementation Report — ADR-0022, written first and carrying a measured number

> **STATUS: nine of nine ACs satisfied.** ADR-0022 exists, quotes figures rather
> than a preference, and **the figures overturned the architecture brief's own
> recommendation** — the append condition is a `max(position)` guard, not an
> `EXISTS` probe, because on the rejection path the guard measured 23 µs against
> the probe's 32 at 5,000 events and 213 against 311 at 50,000, in every run at
> both scales and both boundary shapes.
>
> **Not one file under `crates/happenstance-sqlite/src/` moved**, and
> `git diff --name-only HEAD -- crates/ spec/ Cargo.lock Cargo.toml
> .redkiln/config.yaml` returns **empty**. That ordering is AC-013's entire
> content, and the diff is the mechanical proof of it.

## TDD Evidence

This story's deliverable is a decision record and the measurement underneath it,
so its "tests" are the two **positive controls** the experiment is built around —
each written before the thing it guards, and each made to fire. The rest is
review, which is where the project's own testing brief puts AC-013.

RED, from the durability control before `require_shippable` existed:

```text
error[E0599]: no method named `require_shippable` found for struct `Durability`
  --> tests\durability_settings_are_enforced.rs:41:10
```

RED, from the conformance mount before the candidates existed — and it is the
load-bearing red, because `event_store_conformance!` expands in an *integration*
test file, so the whole 445-test file fails to compile if a candidate is not a
real, publicly-nameable `EventStore`:

```text
error[E0432]: unresolved import `append_condition_probes::BeginImmediateProbe`
```

GREEN: **445 + 3 + 15 + 1 + 1 = 465** passing, across five test targets.

| AC | Test / instrument | Red → Green |
| --- | --- | --- |
| AC-001 | `tests/candidates_are_conformant.rs` (445 tests, 89 rules × 5 arms) and `tests/measure.rs` (15) | `E0432` on every candidate → 445 + 15 passing |
| AC-002 | `tests/durability_settings_are_enforced.rs` — 3 tests, one of which **forces** `synchronous = OFF` and asserts the refusal | `E0599: no method named require_shippable` → 3 passing |
| AC-003 | `tests/tag_storage_probe.rs` — 4 arms round-robin, asserts the two filtered reads select proper subsets and that all arms agree on what matched | a mis-encoded blob arm matched a different set → all four agree at 516 / 16,684 / 50,050 |
| AC-004 | `results/append-condition.md` §1 read against `results/raw/contention.txt`; `RUNBOOK.md:302` | the first three runs disagreed about the *ordering* → round-robin interleaving, and an ordering that held in every run since |
| AC-005 | `tests/contention_at_64.rs` — asserts exactly one commit per race and that all four counters sum to *k*, at 8 and 64 | `condition_after(query, 0)` panicked `non-zero position` → unbounded guard, 10/10 committed, `busy=0` at both counts |
| AC-006 | review checklist against the record; `rg -n "index_arms" crates/happenstance-core/src` returns nothing | — |
| AC-007 | `rg` for the restated falsifier; `git diff --name-only` over `.kb/open-questions/` | — |
| AC-008 | `redkiln validate --kb` and `redkiln doctor` | — |
| AC-009 | `git diff --name-only HEAD -- crates/ spec/`; `cargo xtask affected --base main`; `cargo xtask ci --fast` | — |

**Three reds worth recording because each changed the design.**

1. **`condition_after(query, 0)` panics.** `SequencePosition::new(0)` is `None`,
   so an *unbounded* guard is `AppendCondition::new(query)` and not a boundary of
   zero. The rejection control uses `fixtures::condition`.
2. **The first tag-storage measurement gave the wrong answer twice.** Three
   sequential tests, run an hour apart, disagreed about the ordering of the
   filtered read — the host slowed down and whichever arm ran late wore it. That
   is what produced the round-robin design, and it is the single most important
   methodological finding in this story.
3. **The join table's probe looked *worse* than the scan arms.** Chasing it found
   that `GROUP BY … HAVING COUNT(DISTINCT tag)` blocks predicate pushdown of the
   guard's `position > ?` boundary. The single-tag fast path landed with its own
   **negative control** — `JoinTableGrouped`, the same table always taking the
   grouped form — so the 1,093 µs → 556 µs improvement is measured side by side
   rather than across runs.

## Commits

`feat(sqlite-durable-store): ADR-0022 — the append-condition strategy, measured`
— the second and last story checkpoint of slice `bench-harness-and-adr`, on
`initiative/from-contract-to-published-library`, immediately after
`feat(sqlite-durable-store): Add event_store_benchmarks! to happenstance-testkit`.

Named by subject and by predecessor rather than by hash, because this report is
committed *inside* the commit it describes.
`git log --grep "Story: sqlite-durable-store/adr-0022-append-condition-strategy"`
resolves it.

| SHA | Subject |
| --- | ------- |
| *(see above)* | `feat(sqlite-durable-store): ADR-0022 — the append-condition strategy, measured` |

## Changes

| File | Shape of the change |
| --- | --- |
| `experiments/append-condition/` | **New, out-of-workspace crate.** `src/candidate.rs` (migration 1, the read path, identity — everything held fixed), `src/strategy.rs` (the three candidates), `src/tags.rs` (the three tag storages plus the fast-path negative control), `src/durability.rs` (read the pragmas back, refuse to measure under a wrong one), `tests/support/mod.rs` (`CandidateFixture`, in `tests/` exactly where an adapter's belongs), five test targets, `run.sh`, `README.md`, `results/` with three tables and five raw files. Empty `[workspace]` table; no workspace manifest or lockfile touched. |
| `references/adr/0022-append-condition-strategy.md` | **New, the long record.** Head, one question, the driver ratified, the instrument and its limits, the decision, the two losers with figures, seven consequences each with its rejected alternative, two fenced non-verdicts, what it deliberately does not do, and a falsifier per verdict. |
| `.kb/_intake/0033-…`, `.kb/_intake/0034-…` | **New, staged.** The atom source and a **separate** evidence source, both with valid `KbFrontmatter`, for a human-invoked `/redkiln:kb-ingest`. |
| `RUNBOOK.md` | Queue row 0022 struck through and marked **Written**; a new escalation row beneath it for the unowned ES-17 measurement; the phase-8 session log; and two now-stale rows in the lifecycle table corrected against the measurement. |
| `.bklg/…/adr-0022-append-condition-strategy/` | Nine AC rows flipped with cited evidence; these two reports. |

## Gates

| Command | Result |
| --- | --- |
| `cargo test --manifest-path experiments/append-condition/Cargo.toml --test candidates_are_conformant` | **445 passed, 0 failed** |
| `… --test durability_settings_are_enforced` | 3 passed |
| `experiments/append-condition/run.sh` | green end to end, ~5 min, `results/raw/` rewritten |
| `cargo xtask affected --base main` | **affected gate passed** |
| `cargo xtask ci --fast` | **all required checks passed** (4 optional steps not run) |
| `cargo xtask lints` | green, including `27 atoms, all consistent` |
| `cargo xtask spec-trace` | `traceability: no problems found` |
| `redkiln validate --kb` | `validate passed` |
| `redkiln doctor` | exactly the six expected `template-drift` advisories |
| `cargo fmt --all -- --check` (workspace and experiment) | clean |
| `cargo clippy --all-targets` (experiment) | clean |

## Notes

**The verdict reverses this project's own brief, and that is the story working.**
The architecture brief recommends `BEGIN IMMEDIATE` + an `EXISTS` probe. The
measurement says the monotonic-position guard, on the one axis where the three
arms separated reproducibly. AC-013 exists to refuse a preference; had the
measurement agreed with the brief, this story would have proved much less.

**Two caller-side controls were added beyond the harness, and the record says
why.** `event_store_benchmarks!` ran verbatim — that is EC-006 and AC-001 — but
none of its three scenarios evaluates a condition against a populated store, and
its per-scenario timer covers fixture construction and gives each arm its own
time slot. On this host that produced 45% run-to-run disagreement and a 4x
spread on a path no strategy participates in. AC-003 asks for the tag storages
measured *under a probe held under the write lock*, which no scenario does, so
the measurement had to be the caller's — which is exactly the seam CF-23 makes a
parameter and `bench.rs`'s own docs delegate. Both controls are labelled as
controls everywhere they are quoted and neither replaces a harness figure.

**Ties are reported as ties.** The commit path and both contended counts
separated by less than the arms moved between runs. EC-004 required the record to
say so and name its tie-break rather than fabricate a margin, and §5 does.

**Three findings the front half did not anticipate**, all in the record: the
`GROUP BY` pushdown barrier and its single-tag fast path (1,093 µs → 556 µs
against a negative control); the ~200x cost of a two-tag boundary at 50,000
events, which promotes `tag_cardinality` from a nicety to a requirement of
migration 1; and `busy = 0` at 64 contenders, which is what makes the finite
5,000 ms timeout a measured value rather than a guess.

**One file was renamed for the gate.** `src/store.rs` became `src/candidate.rs`
because `cargo xtask spec-trace` resolves bare filename citations across the
whole repository, and a second `store.rs` made three of `SPECIFICATION.md`'s
citations ambiguous — 19 traceability problems. Renaming is the correct fix here
rather than extending `BARE_NAME_MAP`: `xtask/**` and `spec/**` are both outside
this PR's declared boundary, and the collision is the experiment's to avoid.

**What this story deliberately did not do**, each verified by `git diff`: no
`todo!()` replaced, no `#![allow(clippy::todo)]` deleted, no clause or marker
edited, no `.kb/decisions/` or `.kb/maps/` file written by hand, no
`concurrency::CONTENDERS` raised, no `crates/happenstance-testkit/**` touched.
The `/redkiln:kb-ingest` handoff is a human's — all seven Redkiln SDLC commands
carry `disable-model-invocation` — and it is written down as a named deliverable
in the staged atom source rather than left as a footnote.
