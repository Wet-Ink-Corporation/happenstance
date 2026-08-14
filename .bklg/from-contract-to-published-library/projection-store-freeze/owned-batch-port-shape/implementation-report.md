---
item: "HS-S0004"
stage: implement
created: "2026-08-13"
updated: "2026-08-13"
---

# Implementation Report — The owned-batch port shape, mounted and with the skeletons restated

> **STATUS: ten of ten ACs satisfied, with one re-plan finding reported at the
> story boundary and deliberately not absorbed.** The port carries §4.0's shape,
> the four new types are mounted unconditionally in the contract crate's export
> block, four surviving impls compile with their own type choices intact, the
> fifth executed ADR-0017's recorded disposition, and all three GAT shape-tests
> were restated and then *falsified by experiment* to prove they can still fail.
>
> What is **not** green is `cargo xtask spec-trace` and `cargo xtask
> lint-constitution`: this change invalidates seven `file:line` citations in
> `spec/SPECIFICATION.md` and sixteen in `standards/rust/`, and breaks three
> compiled examples in the Rust constitution that implement the port with the
> GAT. Both corpora are outside this story's PR boundary and EC-004's required
> response for the first is explicit — *stop and report, do not edit
> `spec/SPECIFICATION.md` inside this story*. See `## Notes`; the full inventory
> with the exact re-points is `_reviewed-diff.md` §7.

Preconditions were checked before the first edit to `projection.rs`, as AC-009
requires: `.kb/decisions/` holds `0017-what-a-projection-batch-owns.md`,
`0018-returning-a-projection-to-never-run.md` and
`0019-what-happens-when-apply-fails.md`, all `status: accepted`, ingested at
`493a194` — a commit preceding this one — and `redkiln validate --kb` reported
`validate passed`. `_design.md`'s `## Signatures` block is populated, so per this
story's Integration contract it is the binding authority and the specification's
fenced block is its check rather than its source. Neither EC-001 nor EC-002
fired.

## TDD Evidence

Red first, in one commit-shaped move: the whole `#[cfg(test)] mod tests` block
was written against the target shape and run **before** the trait changed. For a
story whose entire deliverable is a type shape the compiler is the test runner,
so "fails for the right reason" means the diagnostic names the missing item — and
it did, item for item:

```text
error[E0432]: unresolved imports `super::Authority`, `super::Checkpoint`,
              `super::CommitError`, `super::ResetError`
   --> crates\happenstance-core\src\projection.rs:149:17
error[E0407]: method `reset` is not a member of trait `ProjectionStore`
   --> crates\happenstance-core\src\projection.rs:203:9
error[E0107]: missing generics for associated type `ProjectionStore::Batch`
   --> crates\happenstance-core\src\projection.rs:194:26   (and :205, :212)
    | note: associated type defined here, with 1 lifetime parameter: `'a`
```

Not a typo and not an import slip: four missing types, one missing method, and
three sites demanding the lifetime this story removes. Green after the trait
landed — 69 passed, 0 failed in `happenstance-core`'s lib tests.

| AC | Test | Red → Green |
| --- | --- | --- |
| AC-001 | `projection.rs::tests::the_port_is_implementable_with_an_owned_batch` | `E0432`/`E0407`/`E0107` above → passing; implements all six items on a zero-sized `Witness` with an owned `WitnessBatch` |
| AC-002 | `projection.rs::tests::checkpoint_names_every_state_and_no_others` | `E0432` on `Checkpoint` → passing; two exhaustive matches with no `_` arm |
| AC-003 | `projection.rs::tests::commit_and_reset_errors_stay_separate` | `E0432` on `CommitError`/`ResetError` → passing; one exhaustive match per enum, plus a `Store(WitnessError)` round trip recovering the payload by value |
| AC-004 | `projection.rs::tests::begin_is_neither_async_nor_fallible` | `begin` returned a future of a `Result`, so the `let batch: …::Batch = store.begin();` binding did not typecheck → passing with no `.await` and no `?` |
| AC-005 | `projection.rs::tests::a_non_send_batch_still_implements_the_bare_flavour` | `E0107` on `Witness`'s `type Batch` → passing; the batch holds `Rc<()>` |
| AC-006 | `happenstance-ladybug/tests/port_shape.rs::the_owned_batch_shape_satisfies_generic_code_on_both_flavours` | bound was `for<'a> S::Batch<'a>: Send`; restated to `S::Batch: Send` → passing |
| AC-007 | doctest `crates/happenstance-core/src/projection.rs - projection (line 55)` | the four names did not resolve through the crate root → passing under `cargo test -p happenstance-core --doc` |
| AC-008 | `cargo clippy`/`cargo test` over the four adapter crates | every impl was `E0195`/`E0046`/`E0050` against the new trait → all green |
| AC-009 | `redkiln validate --kb`; the executed-arm record | preconditions verified before the first edit |
| AC-010 | the three restated shape-tests | see the falsification pass below |

**The falsification pass is the part that matters for AC-010 and AC-006.** A
restated shape-test that still compiles is not evidence it can still fail, so
each wrong shape was written into the tree, the compiler's rejection recorded,
and the tree restored:

```text
port_shape.rs      — delete `S::Batch: Send`
                     → error: future cannot be sent between threads safely (:97)
sqlite             — rebind `type Batch = Vec<PendingStatement>`
                     → error[E0271]: type mismatch resolving
                       <SqliteProjectionStore as ProjectionStore>::Batch == SqliteBatch
                       (tests/shapes.rs:188)
postgres           — rebind `type Batch = sqlx::PgConnection`
                     → error[E0308]: expected `Transaction<'static, Postgres>`,
                       found `PgConnection` (projection_store.rs:183)
```

None of the three was deleted, so EC-007 did not fire.

## Commits

`98e0298` — `feat(projection-store-freeze): The owned-batch port shape`

One commit, staged so the port edit and the five restatements are readable
separately in the diff (NF-006). `LiveHandleProjectionStore` is recorded as a
rename rather than a delete-plus-add, so `git log --follow` reaches its history.

## Changes

**The port and its mount**

- `crates/happenstance-core/src/projection.rs` — 139 lines → 548. Module doc
  rewritten where it said the batch borrows the store (:20-45), plus a doctest
  (:55-77) that imports the four new names through the crate root. Four public
  types: `Checkpoint` (:133), `Authority` (:165), `CommitError<E>` (:180),
  `ResetError<E>` (:221). `ProjectionStore` restated (:255): `type Batch;`
  (:273), `fn begin` (:283), `checkpoint -> Checkpoint` (:298), `commit` with
  `Authority` and `CommitError` (:319), new `reset` (:334), `rollback` (:352).
  New `#[cfg(test)] mod tests` (:361-548). `#[trait_variant::make]` kept; the
  `# Status: provisional` header kept (NF-007).
- `crates/happenstance-core/src/lib.rs:115-118` — the export block gains
  `Authority`, `Checkpoint`, `CommitError`, `ResetError`, unconditionally.
  `Cargo.toml` untouched (NF-001).

**The five impls** — signature restatements only; see `_reviewed-diff.md` §3 for
the per-impl before/after table.

- `happenstance-postgres/src/projection_store.rs` — `Transaction<'static,
  Postgres>` kept, all bodies `todo!()`; its shape-test restated and re-armed.
- `happenstance-ladybug/src/projection_store.rs` — `GraphWriteSet` kept, all
  bodies `todo!()`.
- `happenstance-sqlite/src/projection_store.rs` — `SqliteBatch` kept; its
  `E0195` comment turned from a live trap into a discharged one.
- `happenstance-neon/src/projection_store.rs` — `NeonWriteBatch` kept, and
  `impl<T: SqlTransport + 'static>` sheds the `+ 'static` the GAT forced.
- `happenstance-ladybug/src/live_handle.rs` → `experiments/live-handle-projection-batch/live_handle.rs`,
  with a `README.md`, per ADR-0017.

**Consequential documentation** — three adapter `lib.rs` module docs that quoted
a signature the crate no longer has, and five intra-doc links to
`crate::live_handle` respelled as plain text (a link into an absent module is a
hard `rustdoc::broken_intra_doc_links` error, and that lint is `deny`).

**Backlog** — `_ledger.md` (ten rows flipped with evidence), `_reviewed-diff.md`,
this report, `report.md`.

## Gates

Green:

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | clean |
| `cargo clippy --locked -p happenstance-core -p happenstance-testkit -p happenstance-postgres -p happenstance-ladybug -p happenstance-sqlite -p happenstance-neon -p xtask --all-targets --all-features -- -D warnings` | clean |
| `cargo test --locked -p happenstance-core -p happenstance-testkit -p happenstance-postgres -p happenstance-ladybug -p happenstance-sqlite -p happenstance-neon --all-features` | 0 failures |
| `cargo test -p happenstance-core --doc` | 26 passed, including `projection (line 55)` |
| `cargo xtask wasm` | all four steps, including the mandatory `happenstance-core` build (AC-005) and `happenstance-neon` (AC-008) |

Red, and reported rather than absorbed:

| Command | Result |
| --- | --- |
| `cargo xtask spec-trace` | 7 citation problems in `spec/SPECIFICATION.md` |
| `cargo xtask lint-constitution` | 16 citation problems in `standards/rust/` |
| `cargo test -p xtask --doc` | 3 of 230 examples fail — the constitution's own GAT-shaped port impls |
| `cargo xtask affected --base main` | exits non-zero on `spec-trace`, which it runs **first**, before it compiles anything (`xtask/src/affected.rs:117-125`) |

Not one of those is a compile, lint, format or test failure in the code this
story owns. All of them are `file:line` citations and compiled examples in two
documentation corpora that the port change invalidated. See `## Notes`.

## Notes

**Deviation 1 — the spec's `spec-trace` analysis was wrong, and the deferral it
justified does not work.** This story's Behavior table predicted that check 7
"stays green and stale citations pass silently", and on that basis assigned
re-pointing to `unstable-projection-gate-and-clause-disposition` (HS-S0016).
Two things falsify it: five citations name
`crates/happenstance-ladybug/src/live_handle.rs`, which **ADR-0017 required this
story to move**, and two are *anchored* citations whose subject left the window
when `projection.rs` grew. More seriously, `cargo xtask affected --base main` —
which `.redkiln/config.yaml` wires to `redkiln verify --grain story` — runs
`spec-trace` first and unconditionally, so **no story in this project can pass
its own gate until these seven are re-pointed, including HS-S0016 itself**. The
work cannot stay where the plan put it. Not fixed here because EC-004's required
response is explicit and `spec/SPECIFICATION.md` is excluded from the PR
boundary; the exact seven re-points are in `_reviewed-diff.md` §7a.

**Deviation 2 — the Rust constitution is a fifth corpus that implements this
port, and no brief named it.** `standards/rust/` cites this code with line+text
anchors and compiles three examples against `ProjectionStore`. Three of its rules
are not drifted citations but rules whose *subject this story deleted* — and each
names its own settlement condition. RS-22-4 is literally marked `[PROVISIONAL —
settles at SPECIFICATION PS-5, which retires the GAT and with it this trap]`, and
PS-5 has now landed. Restating an adapter skeleton is mechanical; retiring a
house-style rule is an editorial decision of ADR weight, the corpus has no
retirement convention to follow, and inventing one inside a story whose subject
is a trait signature is the side-effect authorship this project's sequencing
exists to prevent. Reported instead, with the full inventory in
`_reviewed-diff.md` §7b.

**Not a deviation — the `(None, true)` sentence has one home, not two.** AC-002
asks for it "at `checkpoint`"; `_design.md` assigns it to the `Checkpoint` type.
Written once on the type, with `checkpoint`'s doc pointing there. RS-70-5's
density rule is what makes that the resolution rather than a compromise.

**Not a deviation — `ResetError::Refused` is bare.** Answered by ADR-0018
(`:108-110`) and `_design.md`, not chosen here. EC-002 did not fire.

**EC-003, EC-005, EC-006, EC-007, EC-008 did not fire.** No impl needed an edit
beyond a signature restatement; no `Send` bound leaked onto `Batch`; `E0195`
did not survive the removal — it is retired, which is PS-5's claim settled by a
compiler; no shape-test was removed; and the `#[cfg(test)]` witness stayed
zero-sized, unexported and clearly not a store worth shipping.
