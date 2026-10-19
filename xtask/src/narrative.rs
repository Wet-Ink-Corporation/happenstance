//! Compiles every Rust example in `docs/` — the narrative tree.
//!
//! # What this does not verify
//!
//! Stated first, because a check whose limits are undocumented is read as a
//! guarantee — `xtask/src/lint_constitution.rs` makes the same argument about
//! its own, and this corpus is the one most likely to be quoted as evidence of
//! something it never checked.
//!
//! Five of the seven limits below are properties of *this* mechanism —
//! compiling a fence — and are stated here: `_decomposition.md` Note 10's four,
//! and a seventh about *attribution* that Note 10 could not have forecast,
//! because it is a property of the assembled gate and was only visible once the
//! gate was watched failing. Adding it is what EC-005 of
//! `documented-blind-spots-and-their-proofs` provides for: a limit may be added,
//! and none of the six may be dropped, softened or reordered to make room for
//! it. The remaining limit, the fifth, is a property of the fence *walk* —
//! which reads pages as files rather than compiling them — and is stated where
//! it holds, in `xtask/src/lint_narrative.rs`. That is a plain path and not an
//! intra-doc link: the checker is a **bin**-crate module, this file is compiled
//! into the lib target, and the two never link — a link would be a broken
//! intra-doc link, which the gate's `documentation` step turns into a hard
//! error rather than a warning.
//!
//! * **It does not check that an example is still demonstrating the claim above
//!   it.** A fence whose surrounding sentence has drifted compiles exactly as
//!   happily as one that has not. This is the headline blind spot of the whole
//!   step, and **no mechanical test can close it**: the gap is semantic rather
//!   than mechanical, so there is nothing for a checker to compare. None is
//!   written, and writing one that gestured at it would be worse than none —
//!   a decorative instrument is what gets the real one deleted. The
//!   compensation inside the tree is compositional, each fence sitting
//!   immediately after the sentence it demonstrates so a reviewer meets both at
//!   once; the real instrument is HS-P0024's friction log, and nothing in this
//!   repository substitutes for it.
//! * **It does not lint what is inside a fence.** Doctests receive neither the
//!   workspace `[lints]` table nor `cargo clippy`, which does not lint doctests
//!   at all, so `unwrap_used = "deny"` is unenforced in every fence in this
//!   tree — measured, below. The only instrument is the reviewer reading the
//!   diff, and the fence band is composed to make that possible rather than
//!   likely.
//! * **`RUSTDOCFLAGS=-D warnings` reaches nothing inside a narrative fence, and
//!   that is measured rather than cited.** The two claims this repository held
//!   disagreed — [`crate::constitution`]'s own probe recorded that the variable
//!   recovers rustc's *default-on* lints inside a doctest, while the upstream
//!   reports say `cargo test --doc` drops it — so the probe was re-run against
//!   this step as `REQUIRED` declares it, on `rust-toolchain.toml`'s pinned
//!   **1.97.1**. Result: a fence violating `non_snake_case`, a warn-by-default
//!   rustc lint confirmed to fire on the same snippet under plain `rustc`,
//!   compiled and ran with **no diagnostic and exit 0** — with the variable
//!   set, with it removed, and with the extra `cargo run -p xtask` hop taken
//!   out. `unwrap_used` was likewise unenforced. Three transcripts, their exact
//!   commands and the toolchain are in this project's `_limits-evidence.md`;
//!   the upstream reports are context, not the finding.
//! * **A failure does not name this file, and the path it prints is the page's
//!   own — reached through this file's directory, with a line that is not the
//!   failing statement.** Measured: libtest names the doctest
//!   ``xtask\src\../../docs/append-conditions.md - narrative::append_conditions
//!   (line 9)``, where `9` is the *opening* line of the fence rather than the
//!   line that failed. The panic's own `file:line` is worse and is not a
//!   location at all: it is a temporary bundle file under the OS temp
//!   directory, or — under `--show-output` — the page's path with a line
//!   counted inside rustdoc's synthesized doctest source, which lands on a
//!   sentence of prose. So the only stable, actionable identifier is **the
//!   doctest's module name**: one module per page is what keeps it the page's
//!   name, and that is the whole of what registration buys.
//! * **A broken fence here does not fail under this step's banner.** Under
//!   `cargo xtask ci` the first step that compiles these pages is
//!   `=== tests ===` at index 2: `cargo test --locked --workspace
//!   --all-features` compiles the **lib** target's doctests, and this file is
//!   that target's doctest root. `run_steps` bails at the first non-zero status
//!   (`xtask/src/main.rs:1449`), so neither of the tree's own banners prints at
//!   all. What the two compile steps' ordering decides is narrower than it
//!   reads: which of *those two* a failure is attributed to, in a run that
//!   reached them. So the instrument that tells a reader which corpus broke is
//!   the failing doctest's module name — the limit above — and never the banner
//!   over it. Measured rather than forecast, in
//!   `observed-failure-falsification`'s `_falsification.md`, finding **F2**.
//!   The step ordering that record reports is a defect of the gate's shape
//!   rather than of this check, and is owned by the follow-up this project's
//!   `_limits-evidence.md` names — not repaired from here, which would make
//!   every transcript in that record describe a tree that never existed.
//! * **This step says nothing about whether any page teaches anybody
//!   anything.** A green banner means the fences compiled and their assertions
//!   held; comprehension is HS-P0024's friction log, and no run of this step
//!   substitutes for it.
//!
//! # Why the harness is here and not in a published crate
//!
//! `include_str!` resolves at compile time against the file tree, and
//! `crates/happenstance/src/../../../docs/…` does not exist inside a packaged
//! `.crate`. `xtask` is `publish = false` and is built by CI on every push,
//! which is the same argument [`crate`]'s root already makes for the repository
//! README and [`crate::constitution`] makes for the Rust constitution.
//!
//! It must be declared from `xtask/src/lib.rs`, not from `xtask/src/main.rs`:
//! `cargo test -p xtask --doc` compiles the **lib** target's doctests only, so
//! the same module in the bin crate compiles clean while nothing on any page is
//! ever compiled. That mis-mount is invisible to every other check in the
//! repository, which is why `cargo xtask narrative-doctests` asserts the
//! doctests out of `--list` before running them.
//!
//! # One module per page, and how its name is derived
//!
//! The obvious spelling is several `#![doc = include_str!(…)]` attributes on one
//! module. It is wrong for the reason [`crate::constitution`] states at length:
//! they concatenate into a single doc string, so a failure on the nineteenth
//! page reports a line counted from the first.
//!
//! The module name is the page's path below `docs/` with the `.md` suffix
//! removed and both `/` and `-` replaced by `_`: `append-conditions.md` becomes
//! `append_conditions`, and `adapters/sqlite.md` would become
//! `adapters_sqlite`. One pure rule, and it is now *computed* rather than only
//! stated: `xtask::lint_narrative::module_name` is the single derivation the
//! bidirectional page-to-module registration check compares against in both
//! directions, and this paragraph is its prose mirror rather than a second
//! spelling of it.

// Each module exists only while rustdoc is collecting doctests, so a normal
// build carries neither the module nor the included prose.

#[cfg(doctest)]
mod append_conditions {
    #![doc = include_str!("../../docs/append-conditions.md")]
}

// The opening encounter. Three fences, each a whole program, and the third one
// asserts the refusal the page is about — so removing the boundary from it
// turns this module red rather than turning a sentence stale.
#[cfg(doctest)]
mod first_encounter {
    #![doc = include_str!("../../docs/first-encounter.md")]
}

// The conceptual bridge. Four whole programs; the first and third are one
// expression apart: the guard tagged to the course refuses, the guard tagged to
// the student accepts (`is_ok()`). Between them a newtype refuses a bad tag, and
// the last shows the corrected item matching. Each assertion is the argument.
#[cfg(doctest)]
mod carry_your_invariant {
    #![doc = include_str!("../../docs/carry-your-invariant.md")]
}

// The prior-model bridge. One program takes the same decision three times
// against three other writers, and its assertions are the argument: the write
// that touches neither limit is admitted, and the two that touch one limit
// each are refused by the same condition. Dropping either query item, or the
// guard, turns this module red rather than turning the page stale.
#[cfg(doctest)]
mod boundary_not_aggregate {
    #![doc = include_str!("../../docs/boundary-not-aggregate.md")]
}

// The handoff. It carries no fence either, and for a different reason from the
// page below: its teaching is somebody else's file, reached in one hop. What
// registration buys here is that the page cannot rot into an orphan under the
// tree while the two links on it still look fine in a diff.
#[cfg(doctest)]
mod read_the_worked_example {
    #![doc = include_str!("../../docs/read-the-worked-example.md")]
}

// The adapter author's reading order. It carries no fence either, and for the
// page above's reason: everything it teaches is somebody else's file, reached
// by name. Registration is the whole of what guards that — a page deleted
// leaves a `mod` naming nothing, and a page renamed stops resolving in
// `include_str!`, so the six citations on it and every cross-reference into it
// fail the build rather than rotting into prose that still reads fine.
#[cfg(doctest)]
mod adapter_reading_order {
    #![doc = include_str!("../../docs/adapter-reading-order.md")]
}

// Registered like any other page, and it carries no `rust` fence at all. That
// is the point: it is the retained fixture behind the `text`-fence limit stated
// in `xtask/src/lint_narrative.rs`, walked by the checker on every run and
// reported by nothing. It is also why the count this step prints is a count of
// pages that *produce* a doctest rather than of pages the harness registers.
#[cfg(doctest)]
mod text_fences {
    #![doc = include_str!("../../docs/text-fences.md")]
}

// The upgrade guide. Its `rust` fences are the 0.4 spellings of the breaks the
// facade and the core crate carry, each asserting what the break changed; the
// 0.3 spellings and the adapter-only code sit in `text` fences and say so,
// because neither builds against this target.
#[cfg(doctest)]
mod upgrading_to_0_4 {
    #![doc = include_str!("../../docs/upgrading-to-0-4.md")]
}

// The first how-to. Three fences: a zero `Retry` refused at compile time
// (E0080); the reply each exhausted or failed command gets; and two deliveries
// of one request racing, where the loser is refused and its retry and a late
// resend both find the first. Deleting the request check in `attempt` lands
// three deposits and turns this module red.
#[cfg(doctest)]
mod retry_a_command {
    #![doc = include_str!("../../docs/retry-a-command.md")]
}

// The rebuild how-to. Two fences over the port, because the runner is behind
// `unstable-projection` and this target does not enable it. The first resets a
// view and commits a second at the first position, and counts what each replay
// reads; the second has a backwards commit refused and a reset take its place.
// A reset that stopped clearing the checkpoint, or a store that accepted the
// backwards commit or wrote the row it carried, turns this module red.
#[cfg(doctest)]
mod rebuild_a_read_model {
    #![doc = include_str!("../../docs/rebuild-a-read-model.md")]
}

// The store how-to. Three fences, and none names an adapter crate, because no
// adapter is a dependency of this target. The first runs one function bound on
// the bare flavour against the reference store, a gappy store and a bare-only
// store, and asserts that the count it reads agrees on all three while the
// position distance does not. The second has a `SendEventStore` bound refuse
// the bare-only store (E0277), and the third is its compiling control: the same
// bound accepting the reference store, because stable rustdoc does not check a
// `compile_fail` fence's error code (RS-62-1). Binding the first on
// `SendEventStore`, or loosening the second to `EventStore`, turns this module
// red.
#[cfg(doctest)]
mod choose_a_store {
    #![doc = include_str!("../../docs/choose-a-store.md")]
}

// The Durable Object how-to. Three fences, and none names the Cloudflare crate,
// because it is not a dependency of this target. The first is a stand-in with
// that store's shape, an `Rc` over the memory store implementing only the bare
// flavour, run through one decision function bound on `EventStore` beside the
// memory store, with a clone of it filling the course the original opened. The
// second spawns a task holding an `Rc` handle and is refused. rustc prints that
// refusal with no error code, so the fence names none; the third is its
// compiling control with an `Arc`, because a `compile_fail` fence passes on any
// failure (RS-62-1). A clone that stopped aliasing the log, a seat limit off by
// one, a bound on `SendEventStore`, or an `Rc` handle the spawn accepted, turns
// this module red.
#[cfg(doctest)]
mod run_on_a_durable_object {
    #![doc = include_str!("../../docs/run-on-a-durable-object.md")]
}

// The conformance how-to. Two fences. The first is a whole fixture over
// `GappyMemoryStore`, with the one-line suite invocation an adapter writes; its
// `main` runs every rule `for_each_event_store_rule!` names under `block_on`, as
// `emit_blocking` does, and asserts the skips are exactly the three
// capabilities the fixture declined plus the ceilings it states none of, each
// reason in the fixture's own words rather than the testkit's default. The
// second is a fixture whose every `connect` opens a fresh store: the isolation
// rule passes it and the second-handle rule fails it. A handle that stopped
// sharing its store, a capability claimed and not provided, or a declension
// left to the default wording turns this module red.
#[cfg(doctest)]
mod pass_conformance_suite {
    #![doc = include_str!("../../docs/pass-conformance-suite.md")]
}

// The four background pages a newcomer reads before the opening encounter:
// events, event sourcing and CQRS, aggregates, and DCB. Prose and `text` fences
// only, so nothing here compiles; they are mounted so the registration check
// and the fence walk read them like every other page in the tree.
#[cfg(doctest)]
mod event_driven_systems {
    #![doc = include_str!("../../docs/event-driven-systems.md")]
}

#[cfg(doctest)]
mod event_sourcing_and_cqrs {
    #![doc = include_str!("../../docs/event-sourcing-and-cqrs.md")]
}

#[cfg(doctest)]
mod aggregates {
    #![doc = include_str!("../../docs/aggregates.md")]
}

#[cfg(doctest)]
mod what_is_dcb {
    #![doc = include_str!("../../docs/what-is-dcb.md")]
}
