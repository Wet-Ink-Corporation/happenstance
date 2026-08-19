---
item: "HS-S0042"
stage: report
created: "2026-08-17"
updated: "2026-08-17"
---

# Report — The model family green, and SqliteEventStore in the mutant pass column

## Findings Ledger

**Seven of seven ACs satisfied. Nothing blocked, nothing deferred.** The third of
the four macros `project.md` DoD 2 enumerates is green against a real file, and the
racing family's falsifier now reads in this adapter's own spelling.

**Mount point:** `crates/happenstance-sqlite/tests/conformance.rs:89` —
`happenstance_testkit::event_store_model_conformance!(SqliteFixture::new())`, in
the *same* target and against the *same* `SqliteFixture` as the sequential suite,
so an adapter author meets both families in one output from one command with no
extra flag.

**Second wiring point:** `crates/happenstance-sqlite/Cargo.toml` `[dev-dependencies]`
— `features = ["proptest"]` on the existing testkit entry. This is the whole of the
feature edge, and it is at the dependency site because a crate cannot `cfg` on a
dependency's feature.

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — the generated space answers for this store | **Met** | `dcb_model_conformance::ops_agree_with_the_model ... ok`; the target's result line moves from 89 to **90 passed; 0 failed; 0 ignored**, 5.74 s. The outcome word is `Ran`, not `Skipped` and not absent. Liveness proved by mutation — see *The check* |
| **AC-002** — no second flag, no second command | **Met** | `cargo test -p happenstance-sqlite` with no `--features` shows the model test. `cargo xtask wasm` green: the dev-dependency's feature is unconditional, the invocation is gated by `not(target_arch = "wasm32")`. The manifest carries the reason as a comment |
| **AC-003** — the emitter that cannot work is declined out loud | **Met** | `tests/conformance.rs:45-59` names `__emit_model_blocking`, states the mechanism (`block_on` with no tokio runtime anywhere; the read stream's deferred `spawn_blocking` resolves through the captured handle and then `Handle::try_current`, both absent), calls it a runtime fact rather than a conformance defect, and says CF-23 is satisfied for this family inside the testkit |
| **AC-004** — a generated disagreement is fixed in the adapter | **Met, vacuously and honestly** | **No disagreement was found** and no line of `crates/happenstance-sqlite/src/**` changed. The instrument's liveness is carried by the deliberate mutation instead of by a fix. No literal position is asserted anywhere this story adds |
| **AC-005** — the pass-column claim is discharged where it honestly can be | **Met** | `tests/conformance.rs:60-73` carries the verdict and **both** refusal reasons. `cargo run -p xtask -- proof-artefact` re-runs the harness: `conformant_variants_pass_everything ... ok`, `the_model_rule_rejects_exactly_what_it_claims ... ok`, 10 passed |
| **AC-006** — the racing falsifier reads in this adapter's spelling | **Met** | `mutation_coverage.rs:2564` opens with `BEGIN DEFERRED` and explains why the verb is load-bearing; the matching paragraph is at `racers.rs:362-381`. `fails` and both `expect` pins byte-identical. `the_concurrency_rules_reject_exactly_what_they_claim` and `every_mutant_states_its_provenance` green; `spec-trace` green, so ES-25 is discharged rather than edited |
| **AC-007** — the impossible instruction is corrected in the tree | **Met** | `mutation_coverage.rs:2548-2557`, a comment on the `RACERS` row, citing `README.md:187-190` and `mutant_registry_is_exhaustive`; repeated at `racers.rs:377-382`, which now carries the README anchor itself. `mutant_registry_is_exhaustive` green with **79 registry rows, unchanged**. The range was `:106-113` when this story merged and was corrected after review — see *The citation that did not resolve* below |

## The check, and why a first-run green needed one

`ops_agree_with_the_model` passed on its first run against `SqliteFixture`. Per
CLAUDE.md that is a claim, so it was falsified on purpose. The forwards
`resume_from` predicate at `crates/happenstance-sqlite/src/event_store.rs:1339` was
changed from `position >= ?` to `position > ?`:

```text
the store disagreed with the model.

smallest failing sequence (2 op(s)):
  0. Append { events: [Event { event_type: EventType("A"), … }] }
  1. Read { query: All, from: First, backwards: false, limit: None }

at op 1: read(All, ReadOptions { from: Some(SequencePosition(1)), … })
  returned [], and the model expected [1:A]
```

Two operations, shrunk, with the resolved `Anchor` shown beside the symbolic one.
The mutation was reverted and `git diff` on `event_store.rs` is empty. The family
is an instrument, not decoration.

## The finding: a `REGISTRY` row that cannot exist

`_storymap.md` instructs this story to add *"the `BEGIN DEFERRED` probe-then-insert
row to `mutation_coverage/mutants.rs`'s `REGISTRY`"*. Following it literally would
have spent the PR fighting `mutant_registry_is_exhaustive`, which rejects a
`Declared` whose `fails` list is empty — and a probe-then-insert store fails **no**
sequential rule, which is the entire content of the defect. A concurrency rule's
wrong store belongs in `racers.rs`/`RACERS`, and the store the map wanted has been
there since phase 3 as `RacingProbeStore`, named by ES-25 `[FROZEN]` itself.

The deliverable is therefore the **correction**, written where the next reader meets
it rather than in a commit message: a comment on the `RACERS` row and a closing
paragraph in the store's doc comment, both citing
`crates/happenstance-testkit/README.md:187-190`. Discharging the story map's *"the
registry row that proves the rejection is live"* by silence is indistinguishable
from not having looked.

### The citation that did not resolve

Both places first cited `crates/happenstance-testkit/README.md:106-113`, and that
range is the README's *"### The projection suite"* section — not the sentence
about where a concurrency rule's wrong store goes, which is at `:187-190` under
*"Two rules govern what goes in it"*. AC-007 is written from the persona of the
next reader six months on, and that reader follows the citation; a correction
that sends them to the wrong section fails at the one thing it exists to do. Both
were re-anchored after review, and `racers.rs` now carries the README range
itself instead of forwarding to the `RACERS` comment for it.

The stale range was not invented here: it is what this story's own `spec.md`
carries at `:33`, `:95`, `:125`, `:151` and in the Context pack's reading table
at `:260`, and it is quoted inside the AC-007 criterion, which a ledger may never
edit. The same is true of the code half it pairs with,
`mutation_coverage.rs:3389-3395` — `mutant_registry_is_exhaustive` is at `:2826`,
which is why the in-tree comment names the test rather than a line. Both stale
spec ranges are flagged in the ledger for `spec-and-code-reconciliation` so the
next story that reads this spec does not copy them forward again.

The **provenance** amendment beside it is the other half. The row described the
autocommit spelling only — `SELECT 1 … then INSERT, with no BEGIN between them`.
This adapter's spelling is `BEGIN DEFERRED`, which is worse for looking *more*
correct: it is SQLite's default, it is what `rusqlite::Connection::transaction`
opens, and the read lock taken at the probe is only promoted at the insert, so the
window is inside an open transaction. `happenstance-sqlite` opens `BEGIN IMMEDIATE`
precisely to close it.

## Declined, with reasons

- **Testkit-side registration of `SqliteEventStore`** in
  `crates/happenstance-testkit/tests/mutation_coverage/`: it would make a published
  crate dev-depend on its own consumer and drag `rusqlite` plus a tokio runtime into
  a harness deliberately built to need neither. Recorded at `conformance.rs:60-73`.
- **The blocking model emitter**: it would fail on the *read* path with `NoRuntime`,
  which is a runtime fact and not a conformance defect. Recorded at
  `conformance.rs:45-59`.
- **The optional `.kb/_intake/` note**: the correction is already durable in two
  code locations and in the ledger, and `.kb/_intake/` is a staging area a different
  command consumes. No atom under `.kb/decisions/` was authored.

## Deferred

Nothing. Two things a reviewer will notice and should read as scope rather than
gaps: `todo!()` and the scoped `#![allow(clippy::todo)]` are still present
(`instrument-markers-removed-and-gate-green`), and the reopen negative control with
the ES-35 / CF-17 / CF-14 verdicts is the slice-mate's
(`reopen-negative-control-and-durability-verdicts`).
