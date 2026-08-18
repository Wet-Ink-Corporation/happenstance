//! Compiles every Rust example in `docs/` — the narrative tree.
//!
//! # What this does not verify
//!
//! Stated first, because a check whose limits are undocumented is read as a
//! guarantee — `xtask/src/lint_constitution.rs` makes the same argument about
//! its own, and this corpus is the one most likely to be quoted as evidence of
//! something it never checked.
//!
//! * **It does not check that an example still demonstrates its claim.** A
//!   fence whose surrounding sentence has drifted compiles exactly as happily
//!   as one that has not. This is the headline blind spot of the whole step,
//!   and the only compensation is compositional: each fence sits immediately
//!   after the sentence it demonstrates, so a reviewer reading the diff meets
//!   both at once.
//! * **It does not lint fence bodies.** Doctests receive neither the workspace
//!   `[lints]` table nor `cargo clippy`, which does not lint doctests at all,
//!   so `unwrap_used = "deny"` is unenforced inside every fence in this tree.
//! * **The file it names on a failure is this harness, not the page.** The
//!   report reads ``xtask\src\../../docs/<page>.md - narrative::<page> (line
//!   N)``: one module per page is what keeps the *module* the page's name and
//!   the *line* relative to the page, and nothing recovers the rest. That is a
//!   residual of the mechanism, recorded rather than routed around.
//! * **A fence tagged `text` is neither compiled nor flagged**, and neither is
//!   one with no tag at all or one marked `ignore`. The fence walk and its
//!   allowance list are `fence-discipline-and-allowance-list`'s; until they
//!   land, this is a known hole and not something to lean on.
//! * **What `RUSTDOCFLAGS=-D warnings` actually enforces inside a narrative
//!   fence is unmeasured here.** The gate step reaches `rustdoc` through an
//!   extra `cargo run -p xtask` hop, and nothing in this change re-ran the
//!   probe through it. [`crate::constitution`]'s finding was measured without
//!   that hop and is deliberately not inherited; measuring it belongs to
//!   `documented-blind-spots-and-their-proofs`.
//! * **This step is silent about whether the page teaches anybody anything.**
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
