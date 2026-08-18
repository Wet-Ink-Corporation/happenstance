//! Compiles every Rust example in `docs/` — the narrative tree.
//!
//! # What this does not verify
//!
//! Stated first, because a check whose limits are undocumented is read as a
//! guarantee — `xtask/src/lint_constitution.rs` makes the same argument about
//! its own, and this corpus is the one most likely to be quoted as evidence of
//! something it never checked.
//!
//! Four of the six limits below are properties of *this* mechanism — compiling
//! a fence — and are stated here. The fifth is a property of the fence *walk*,
//! which reads pages as files, and is stated where it holds, in
//! `xtask/src/lint_narrative.rs`. That is a plain path and deliberately not an
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

// Registered like any other page, and it carries no `rust` fence at all. That
// is the point: it is the retained fixture behind the `text`-fence limit stated
// in `xtask/src/lint_narrative.rs`, walked by the checker on every run and
// reported by nothing. It is also why the count this step prints is a count of
// pages that *produce* a doctest rather than of pages the harness registers.
#[cfg(doctest)]
mod text_fences {
    #![doc = include_str!("../../docs/text-fences.md")]
}
