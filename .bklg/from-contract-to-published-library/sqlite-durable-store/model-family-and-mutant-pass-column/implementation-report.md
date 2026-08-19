---
item: "HS-S0042"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Implementation Report — The model family green, and SqliteEventStore in the mutant pass column

> **STATUS: seven of seven ACs satisfied.** `dcb_model_conformance::ops_agree_with_the_model`
> is green against `SqliteFixture` on a real file — the third of the four macros
> `project.md` DoD 2 owes — and it went green **on its first run**, which is a
> claim to check rather than a result to accept.
>
> It was checked. The mutation that makes it red, and the shrunk sequence it
> printed, are below.
>
> The falsifier half landed as the spec predicted rather than as the story map
> instructed: **no `REGISTRY` row was added, because none can exist**, and the
> reason is now written on the `RACERS` row where the next reader will meet it.

## TDD Evidence

| AC | Test | Red, and for what reason | Green |
| --- | --- | --- | --- |
| **AC-001** | `dcb_model_conformance::ops_agree_with_the_model` | **Absent from the binary.** Before this change the generated `query × from × backwards × limit × condition × position policy` space had never been run against this adapter at all. Then, deliberately: `event_store.rs:1339`'s forwards `resume_from` predicate `position >= ?` → `position > ?`, and the rule failed with a two-op shrunk sequence | `ok`, target 89 → **90 passed** |
| **AC-002** | the same test, with **no `--features`** | Red as *absent* without the `[dev-dependencies]` feature edge — a crate cannot `cfg` on a dependency's feature, so the reference harness's inner attribute does not port | Present in `cargo test -p happenstance-sqlite` |
| **AC-003** | the harness module doc | Not a test — a declension in writing. `conformance.rs:45-59` | Compiles as part of the target |
| **AC-004** | `--test conformance`, the literal-position search | No disagreement found; **no line of `src/**` changed**. Liveness carried by the deliberate mutation | green |
| **AC-005** | `mutation_coverage::conformant_variants_pass_everything`, `::the_model_rule_rejects_exactly_what_it_claims` | Re-run through the REQUIRED gate step rather than by memory | `10 passed; 0 failed` |
| **AC-006** | `mutation_coverage::the_concurrency_rules_reject_exactly_what_they_claim`, `::every_mutant_states_its_provenance` | Goes red if `fails` or either `expect` pin moves — both are byte-identical | green |
| **AC-007** | `mutation_coverage::mutant_registry_is_exhaustive` | This is the test that would have rejected the story map's instruction: an empty `fails` list is refused | green with **no new `REGISTRY` row** (79 rows, unchanged) |

### The check on the first green run

Per CLAUDE.md, a green suite is a claim. The forwards `resume_from` predicate in
`crates/happenstance-sqlite/src/event_store.rs:1339` was temporarily changed from
`position >= ?` to `position > ?` — an off-by-one in an inclusivity that the
sequential suite's own `read_from_is_inclusive` also covers, chosen because it is
small enough to be plausible:

```text
test dcb_model_conformance::ops_agree_with_the_model ... FAILED

the store disagreed with the model.

smallest failing sequence (2 op(s)):
  0. Append { events: [Event { event_type: EventType("A"), data: <2 bytes>,
                              tags: {"a", "b", "c"}, metadata: None }] }
  1. Read { query: All, from: First, backwards: false, limit: None }

at op 1, Read { query: All, from: First, backwards: false, limit: None }:
  read(All, ReadOptions { from: Some(SequencePosition(1)), … }) returned [],
  and the model expected [1:A]
```

Two operations, shrunk, with the resolved `Anchor` printed beside the symbolic
one. The mutation was reverted; `git diff` on `event_store.rs` is empty.

**No literal position appears anywhere this story adds.** The generator emits
`Anchor`s resolved once per operation against what the store actually assigned,
which is what makes a generated `from` meaningful against `AUTOINCREMENT`
positions that may gap.

## Commits

- `0ad702f` — `feat(sqlite-durable-store): The model family green, and SQLite in the mutant pass column`

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-sqlite/Cargo.toml` | `happenstance-testkit = { workspace = true, features = ["proptest"] }` in `[dev-dependencies]`, with a comment carrying the reason a mirrored feature was rejected. **The only manifest change**, and it is a dev-dependency, so nothing reaches a consumer |
| `crates/happenstance-sqlite/tests/conformance.rs` | `event_store_model_conformance!(SqliteFixture::new())` added beside the sequential invocation, plus three module-doc sections: why there is no `cfg(feature = …)` here, why only the tokio emitter, and where the pass-column claim is honestly discharged |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `RacingProbeStore`'s `RACERS` **provenance** rewritten to open with `BEGIN DEFERRED` and say why the transaction verb is load-bearing; a comment above the row recording that no `REGISTRY` row can exist for this defect. `fails` and both `expect` pins untouched |
| `crates/happenstance-testkit/tests/mutation_coverage/racers.rs` | `RacingProbeStore`'s doc comment gains the same `BEGIN DEFERRED` paragraph — including that `rusqlite::Connection::transaction` opens deferred, which is what makes the defect a *default* rather than a mistake — and closes with the `REGISTRY`-impossibility sentence |

Nothing under `crates/happenstance-sqlite/src/**` changed.

## Gates

| Command | Result |
| --- | --- |
| `cargo test -p happenstance-sqlite --test conformance` | **90 passed; 0 failed; 0 ignored**, 5.74 s — 89 sequential rules + `dcb_model_conformance::ops_agree_with_the_model` |
| `cargo test -p happenstance-sqlite` (no `--features`) | the model test appears in the output |
| `cargo test -p happenstance-testkit --all-features --test mutation_coverage` | **10 passed; 0 failed**, 2.27 s |
| `cargo run -p xtask -- proof-artefact` | `happenstance-testkit/mutation_coverage: 8 named tests present, **79 registry rows**` then all ten meta-tests `ok` |
| `cargo xtask wasm` | green — the dev-dependency's feature is unconditional, the invocation is not |
| `cargo xtask affected --base main` | **`affected gate passed`** |
| `cargo fmt --all -- --check` | clean |

## Notes

**The story map asked for something that cannot exist, and the spec was right about
it.** `_storymap.md` instructs this story to add *"the `BEGIN DEFERRED`
probe-then-insert row to `mutation_coverage/mutants.rs`'s `REGISTRY`"*.
`mutant_registry_is_exhaustive` rejects a `Declared` whose `fails` list is empty,
and a probe-then-insert store fails **no** sequential rule — that is the whole
content of the defect. The store the map wanted already exists as `RacingProbeStore`
in `RACERS`, pinned to both racing rules and named by ES-25 `[FROZEN]` itself.

So the deliverable became the *correction*, written in two code locations rather
than in a commit message: a comment on the `RACERS` row and a closing paragraph in
the store's doc comment. Discharging `_storymap.md`'s *"the registry row that proves
the rejection is live"* by silence would read identically to not having looked.

**The provenance amendment is prose and it is load-bearing prose.** The row said
`SELECT 1 … then INSERT, with no BEGIN between them` — true, and it describes the
autocommit spelling only. This adapter's spelling of the same defect is
`BEGIN DEFERRED`, which looks *more* correct than autocommit and is worse for it:
it is SQLite's default, it is what `rusqlite::Connection::transaction` opens, and
the read lock it takes at the probe is only promoted at the insert. An adapter
author who has just written `BEGIN DEFERRED` should find their own words on the
store that models their bug.

**The `.kb/_intake/` note was declined.** The spec makes it optional. The
correction is durable in two code locations and in the ledger; `.kb/_intake/` is a
staging area a *different* command consumes, so a note there would be a fourth copy
waiting on an unrelated workflow rather than a record. No atom under `.kb/decisions/`
was authored — `git diff --name-only` over `.kb/` is empty for this story.

**The pass column was discharged from this adapter's own target, and the refusal is
in the code.** Registering `SqliteEventStore` inside
`crates/happenstance-testkit/tests/mutation_coverage/` would make a published crate
dev-depend on its own consumer and drag `rusqlite` plus a tokio runtime into a
harness built to need neither. What `conformant_variants_pass_everything` asserts is
*every registered rule driven against a conformant store*, and `tests/conformance.rs`
plus `tests/concurrency.rs` are exactly that. Both reasons are in the harness module
doc at `conformance.rs:60-73` — where a reviewer holding `RUNBOOK.md:4221-4222` meets
them — as well as in the ledger.
