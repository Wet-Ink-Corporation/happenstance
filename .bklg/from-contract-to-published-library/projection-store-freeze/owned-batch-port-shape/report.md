---
item: "HS-S0004"
stage: report
created: "2026-08-13"
updated: "2026-08-13"
---

# Report — The owned-batch port shape, mounted and with the skeletons restated

## Findings Ledger

**Ten of ten ACs satisfied, with real reachable behaviour and a real test
behind each.** One re-plan finding is reported at the story boundary and
deliberately not absorbed; it does not weaken any AC and is described in full at
the end of this section.

| AC | Result | What proves it | Mount |
| --- | --- | --- | --- |
| **AC-001** — §4.0's shape, item for item | **Met** | `projection.rs::tests::the_port_is_implementable_with_an_owned_batch` implements all six items on a zero-sized witness with an owned batch. Red before the change was `E0432` × 4 types, `E0407` on `reset`, `E0107` × 3 demanding the lifetime | `crates/happenstance-core/src/lib.rs:115-118` |
| **AC-002** — `Checkpoint` names every state, `(None, true)` unspellable | **Met** | `checkpoint_names_every_state_and_no_others` — two exhaustive matches, no `_` arm inside the crate, so a fourth variant fails the test. `missing_docs` under `-D warnings` over every new variant and field | same |
| **AC-003** — two error enums, never one, generic over `E` | **Met** | `commit_and_reset_errors_stay_separate` — one exhaustive match per enum, `Refused` unreachable from `CommitError` by construction, `Store(WitnessError)` recovered by value. `--no-default-features` build green, which a `String` payload would break | same |
| **AC-004** — `begin` neither `async` nor fallible, alternatives-that-lost documented | **Met** | `begin_is_neither_async_nor_fallible` binds the batch with no `.await` and no `?`. Two adapters took the benefit the same day: `happenstance-sqlite` and `happenstance-neon` both return the batch directly | `crates/happenstance-core/src/projection.rs:283` |
| **AC-005** — `wasm32` builds because `type Batch: Send;` is not written | **Met** | `cargo xtask wasm`, all four steps. `a_non_send_batch_still_implements_the_bare_flavour`, whose batch holds `Rc<()>` — illegal the moment a `Send` bound lands on the associated type | `projection.rs:253,273` |
| **AC-006** — the caller-side bound simplifies, and still fails | **Met** | `port_shape.rs` now reads `S::Batch: Send`, was `for<'a> S::Batch<'a>: Send`. **Falsified:** deleting it gives `future cannot be sent between threads safely` inside a real `tokio::spawn` | `crates/happenstance-ladybug/tests/port_shape.rs:96` |
| **AC-007** — the four types resolve through the crate root | **Met** | The module-doc doctest imports all four as `happenstance_core::…`; doctests compile as an external crate, so a missing re-export is `E0432`. Passing under `cargo test -p happenstance-core --doc` | `crates/happenstance-core/src/lib.rs:115-118` |
| **AC-008** — five impls, signature lines only | **Met** | Four surviving impls keep their `Batch` type, `Error` type, storage strategy and bodies; Neon sheds the `+ 'static` the GAT forced. Clippy and tests green over all affected packages, `wasm32` build of `happenstance-neon` green. Per-impl before/after in `_reviewed-diff.md` §3. EC-003 did not fire | the four adapter crates |
| **AC-009** — ADR-0017's arm executed, ordering held | **Met** | The atom is not silent: *"moves to `experiments/live-handle-projection-batch/`"*. Executed by `git mv`, history preserved, with a README recording what it refuted. Three ADRs accepted at `493a194`, a commit preceding this one; `redkiln validate --kb` green before the first edit; `.kb/open-questions/` deletions empty | `experiments/live-handle-projection-batch/` |
| **AC-010** — no shape-test survives asserting nothing | **Met** | All three restated, none removed, **each falsified by experiment**: `E0308` (Postgres), `future cannot be sent` (Ladybug), `E0271` (SQLite). Transcripts in `_reviewed-diff.md` §5. EC-007 did not fire | the three adapter crates |

**Nothing was stubbed, skipped or fixture-pinned.** The `#[cfg(test)]` witness in
`projection.rs` is not a shipped store and does not encroach on
`memory-projection-store`: zero-sized, unexported, and named as an instrument
(EC-008).

### The finding — a re-plan, not a defect in this story's code

Two documentation corpora cite this story's code by `file:line` with text
anchors, and this change invalidates both. Neither is in this story's PR
boundary; the full inventory with the exact re-points is `_reviewed-diff.md` §7.

1. **`spec/SPECIFICATION.md` — 7 problems, `cargo xtask spec-trace`.** Five name
   `crates/happenstance-ladybug/src/live_handle.rs`, the file **ADR-0017 required
   this story to move**; two are anchored citations whose subject left the window
   when `projection.rs` grew from 139 to 548 lines. This story's spec predicted
   the opposite — "check 7 stays green and stale citations pass silently" — and
   deferred re-pointing to `unstable-projection-gate-and-clause-disposition`
   (HS-S0016) on that basis. **The deferral does not work**, because
   `cargo xtask affected --base main` runs `spec-trace` first and
   unconditionally (`xtask/src/affected.rs:117-125`) and that command is what
   `redkiln verify --grain story` runs: no story in this project can pass its own
   gate until these seven are re-pointed, HS-S0016 included. Not fixed here
   because EC-004's required response is explicit — *stop and report, do not edit
   `spec/SPECIFICATION.md` inside this story*.

2. **`standards/rust/` — 16 citation problems and 3 failing compiled examples.**
   The Rust constitution is a fifth corpus that implements `ProjectionStore`, and
   no brief in this project named it. Three of its rules are not drifted
   citations but rules whose subject this story deleted, and each names its own
   settlement condition — **RS-22-4 is marked `[PROVISIONAL — settles at
   SPECIFICATION PS-5, which retires the GAT and with it this trap]`, and PS-5
   has now landed**. Retiring a house-style rule is an editorial decision of ADR
   weight and the corpus has no retirement convention to follow, so it is
   reported rather than invented inside a story about a trait signature.

**What the reviewer should decide:** whether to authorise the citation re-point
inside this slice, or to pull HS-S0016 forward ahead of the remaining projection
slices, and separately who disposes of RS-21-1, RS-22-4 and RS-92-1.

## Acceptance

**Recommended: accept the story, and treat the finding as a planning decision
rather than rework.**

Every criterion in `spec.md` is satisfied by real, reachable behaviour with a
real test, and the two claims a compiler cannot make on its own — that the
adapters kept their own choices, and that the shape-tests can still fail — are
backed by a reviewed diff and by three recorded falsification transcripts rather
than by assertion. The project's own AC-013 bar (`same Batch type, same Error
type, same storage strategy, same bodies`) is met by all four surviving impls,
and AC-009's ordering check held: the three decision atoms were accepted in a
commit preceding this one.

The story-grain merge command `cargo xtask affected --base main` exits non-zero,
and it is important to be precise about why: it fails on `spec-trace`, which it
runs **before** it compiles anything, over seven documentation citations — five
of them pointing at a file an accepted, immutable ADR required this story to
move. Every step that measures the code is green: format, clippy with
`-D warnings` over all seven affected packages, tests over the six code packages,
all four `wasm32` steps, and the doctests.

Two things are owed before this project's remaining slices can run at all, and
both are named above.

## Knowledge Harvest

Four things learned here are worth carrying past this story.

**A shape-test must be falsified, not merely restated.** The Postgres test's own
recorded history — a first spelling that certified the shape it existed to
reject, because `'_` in a turbofish is *inferred* while `'_` in argument position
elides to a fresh universally-quantified lifetime — was the warning. The
discipline that answers it is cheap and was applied to all three: write the wrong
shape into the tree, record the compiler's rejection, restore. Three transcripts,
about ten minutes, and the difference between "these tests compile" and "these
tests can fail". Candidate for `standards/rust/61-compile-time-assertions.md`.

**A port's blast radius includes every corpus that cites it, and citation
checkers with *text anchors* have a wider radius than checkers that only resolve
paths.** This project's briefs inventoried impls (and found five where the story
map said three). Nobody inventoried *citations*. `spec-trace` and
`lint-constitution` between them hold 23 pointers into the exact lines this story
rewrote, and moving one file by ADR mandate broke ten of them. The cheap
prophylactic for the next port change is one command before starting:
`rg -n "<the file being changed>" spec/ standards/ references/ .kb/`.

**A deferral is only valid if the deferred-to story is reachable.** The plan
deferred citation re-pointing to a later slice. Because the story gate runs
`spec-trace` before it compiles, that later slice cannot be reached through the
gate. "Who owns this?" is not sufficient; "can the owner still be reached from
here?" is the question that was missing.

**PS-5's central claim is now settled by a compiler rather than by a paragraph.**
`error[E0195]` is gone: SQLite's own comment said the literal `Self::Batch<'_>`
"stays mandatory until the GAT leaves the port itself", and with the GAT gone
`type Batch = SqliteBatch;` and `commit(&self, batch: Self::Batch, …)` compile.
Neon's four `error[E0311]` — the undocumented port constraint that *any* adapter
generic over a type parameter was forced to `'static` — are gone with it. Both
belong in the phase-6 record, and the second is the stronger of the two because
nobody had written it down before the skeleton found it.
