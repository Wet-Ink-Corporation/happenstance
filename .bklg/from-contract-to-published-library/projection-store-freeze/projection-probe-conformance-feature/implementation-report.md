---
item: "HS-S0005"
stage: implement
created: "2026-08-13"
updated: "2026-08-13"
---

# Implementation Report — ProjectionProbe behind happenstance-core's conformance feature

> **STATUS: six of seven ACs satisfied. AC-007 is a split result and is left
> `satisfied: false` deliberately.** Its boundary half held exactly — nothing
> outside the four declared paths was touched — but its whole-gate half
> (`cargo xtask ci` green) is not met, for a reason inherited from the
> slice-mate `owned-batch-port-shape` and outside this story's control. Every
> gate step this story owns is green, including both feature powersets over the
> widened combination set.

The trait, the feature and the round trip landed as one integrated surface. An
adapter author can now write `impl ProjectionProbe for TheirStore` against a
dependency they already have, with one flag and no new edge in their graph —
which is the whole of D1's economic argument, and the thing they could not do at
all before this PR.

EC-001 did not fire: `crates/happenstance-core/src/projection.rs` declares
`type Batch;` with no lifetime, landed by the dependency, so
`probe_write(&self, batch: &mut Self::Batch, …)` is spellable. EC-007's reshape
trigger was not met.

## TDD Evidence

Two RED transcripts, in the order the two mount points fail:

```text
$ cargo test -p happenstance-core --features conformance --test projection_probe_round_trip
error: the package 'happenstance-core' does not contain this feature: conformance
```

That is AC-002's half of the mount — the feature table — failing first, before
anything can even be compiled. After adding `conformance = []`:

```text
error[E0432]: unresolved import `happenstance_core::ProjectionProbe`
  --> crates\happenstance-core\tests\projection_probe_round_trip.rs:38:55
   |
38 |     Authority, Checkpoint, CommitError, ProjectionId, ProjectionProbe, …
   |                                                       ^^^^^^^^^^^^^^^ no `ProjectionProbe` in the root
```

`no ProjectionProbe in the root` is precisely AC-003's assertion failing: the
test imports through the **crate-root** path, so the export-block mount is
proven by compilation rather than by inspection. Neither red is a typo or a
stray import; each names a mount point that does not yet exist.

| AC | Test | Red → Green |
| --- | --- | --- |
| AC-001 | `projection_probe_round_trip.rs::probe_shape_matches_the_specification` | `E0432` above → passing; every member coerced to an explicitly written signature through a generic `P: ProjectionProbe` |
| AC-001 | `::the_probe_is_a_supertrait_of_the_bare_flavour` | → passing; calls `ProjectionStore::begin` from a bound of `ProjectionProbe` alone, so the supertrait relation is exercised rather than asserted |
| AC-002 | `cargo hack check -p happenstance-core --feature-powerset --no-dev-deps` | `does not contain this feature` → 16 of 16 combinations green |
| AC-003 | the crate-root import compiling | `no ProjectionProbe in the root` → passing |
| AC-004 | `::writes_are_visible_through_the_trait_alone` | → passing; `begin` → `probe_write` → `commit` → `probe_read == Some(7)` bound on the trait alone |
| AC-004 | `::all_five_members_are_reachable_generically` | → passing; `probe_delete_all` through `reset`, `probe_read_through` under `READS_THROUGH_BATCH`, and `rollback` |
| AC-005 | both powersets, plus `cargo xtask wasm` | 16 host / 25 wasm32, all green |
| AC-006 | `clippy -D warnings`; `RUSTDOCFLAGS="-D warnings" cargo doc --no-default-features` | green |
| AC-007 | see `## Notes` | split |

**A third red, and it is the one worth recording**, because it is a house rule
catching exactly the wrong implementation it was written for:

```text
error: use of `async fn` in public traits is discouraged as auto trait bounds
       cannot be specified
   --> crates\happenstance-core\src\projection.rs:435:5
    | async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error>;
    = note: `-D async-fn-in-trait` implied by `-D warnings`
```

The specification's fenced block spells `async fn probe_read`. That spelling is
only legal inside a `#[trait_variant::make]` trait, where the macro has already
rewritten it — and this trait is deliberately not one. RS-22-1
(`standards/rust/22-rpitit-and-lifetime-capture.md:12`) requires the hand-written
desugaring in that case and names the tempting wrong fix by name:
`#[allow(async_fn_in_trait)]`, *"which silences the one line where a reader could
have seen that the future has no `Send` bound and never will have one."* The
allow was not taken. Semantics are identical; see `## Notes`.

## Commits

`cb495ee` — `feat(projection-store-freeze): ProjectionProbe behind a conformance feature`

## Changes

- **`crates/happenstance-core/src/projection.rs:352-453`** — `ProjectionProbe`,
  `#[cfg(feature = "conformance")]` at **item level** rather than in a gated
  module (D4: the probe needs `Self::Batch` and `Self::Error` from the port it
  is a supertrait of, and a gated module would add a second name for intra-doc
  resolution to fail on). Five members, their rustdoc, D1's orphan-rule
  placement argument with its falsifier named, and the `unimplemented!()`
  spelling reproduced verbatim. Plus a `#[cfg(feature = "conformance")] use
  core::future::Future;` at `:78-79`.
- **`crates/happenstance-core/Cargo.toml:60-79`** — `conformance = []`, with the
  manifest comment carrying why the empty list is load-bearing and what would
  void the placement decision.
- **`crates/happenstance-core/src/lib.rs:83-92`** — the `# Feature flags` row,
  name spelled plainly with the non-link reason inline. **`:132-134`** — the
  export-block `pub use` under the `#[cfg]` + `#[cfg_attr(docsrs, doc(cfg(…)))]`
  pair, matching the `memory` items exactly.
- **`crates/happenstance-core/tests/projection_probe_round_trip.rs`** — new, 297
  lines, file-level `#![cfg(feature = "conformance")]`, four tests and a
  `RefCell`-backed `TestStore` defined in the file itself. The word `memory`
  does not appear anywhere in it.
- This story's backlog folder: `_ledger.md`, this report, `report.md`.

Density against the spec's own budget: 5 members + 1 associated const on one
trait; **0** new dependencies; **3 → 4** features; **8 → 16** powerset
combinations per target; **1** new test file; **0** new modules; **0** new
crate-graph edges for a consuming adapter. Nothing above those numbers.

## Gates

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | clean |
| `cargo clippy --locked -p happenstance-core -p happenstance-testkit -p happenstance-postgres -p happenstance-ladybug -p happenstance-sqlite -p happenstance-neon -p xtask --all-targets --all-features -- -D warnings` | clean |
| `cargo test --locked` over the six code packages, `--all-features` | 0 failures, including the four new tests |
| `cargo hack check -p happenstance-core --feature-powerset --no-dev-deps` | 16/16 green |
| `cargo hack check -p happenstance-core -p happenstance-neon -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` | 25/25 green |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance-core --no-default-features` | green — the step where an inbound intra-doc link into the gated item is a hard error |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance-core --all-features` | green |
| `cargo xtask wasm` | all four steps green |
| `cargo xtask ci` / `spec-trace` / `lint-constitution` / `cargo test -p xtask --doc` | **red, inherited — see `## Notes`** |

The two powersets are the story's own cost, accepted rather than discovered:
`happenstance-core` went from 8 combinations per target to 16, and the wasm32
selection from 17 to 25.

## Notes

**AC-007 is a split result, and it is left `satisfied: false` on purpose.** Its
boundary half held exactly: `spec/SPECIFICATION.md` is not edited, PS-11 and
PS-12 remain `[PROVISIONAL]`, no rule, fixture, mutant, registry entry or
`MemoryProjectionStore` was added, `crates/happenstance-testkit/` is untouched,
no adapter skeleton moved, and no ADR was written. Its whole-gate half is not
met, and nothing in this story causes that: the slice-mate
`owned-batch-port-shape` invalidated 7 `file:line` citations in
`spec/SPECIFICATION.md` and 16 in `standards/rust/`, and broke 3 compiled
constitution examples, by moving a file ADR-0017 required it to move and by
growing `projection.rs`. Both corpora are outside both stories' PR boundaries;
the inventory with the exact re-points is
`../owned-batch-port-shape/_reviewed-diff.md` §7. Marking the row `true` would
require reading "green whole" as "green apart from the parts that are not", and
that is the kind of ledger entry this project exists to make impossible.

**Deviation, forced by a house rule with a named wrong implementation.**
`probe_read` is spelled `fn probe_read(&self, key: &str) -> impl
Future<Output = Result<Option<u64>, Self::Error>>` where the specification's
fenced block writes `async fn probe_read(&self, key: &str) -> Result<Option<u64>,
Self::Error>`. The semantics are identical and the desugaring is what RS-22-1
requires of any public trait not under `#[trait_variant::make]`. Two things
follow that the spec's own D2 would have wanted: the gate rejects the `async fn`
spelling outright (`-D async-fn-in-trait`), so this is not a preference; and the
hand-written form puts the **absence** of `+ Send` at the declaration, which is
the property the whole bare-flavour design rests on. `#[allow(async_fn_in_trait)]`
— the fix RS-22-1 names as wrong — was not taken.

**One test beyond the three the spec enumerates.**
`the_probe_is_a_supertrait_of_the_bare_flavour` calls `ProjectionStore::begin`
from a bound of `ProjectionProbe` alone. It exists because "supertrait, not a
free-standing trait" is a Behavior-table claim with no other test behind it, and
because a `SendProjectionProbe` added later would collide with `trait_variant`'s
blanket impl (`error[E0275]`) — this is the test that would notice the design
drifting toward one.

**EC-002 – EC-006, EC-008 did not fire.** No inbound intra-doc link (the
`--no-default-features` doc build is green); no powerset combination failed; no
`Send` flavour of the probe was attempted; only `ProjectionStore` is in scope in
the test file; the `unimplemented!()` spelling is what the doc carries; and the
round trip uses a test-local store rather than waiting for the oracle.
