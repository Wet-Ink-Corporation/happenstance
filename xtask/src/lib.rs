//! A home for the repository README's doctests, and for the [`pointers`] register.
//!
//! The README at the repository root shows a Quick start, and until this file
//! existed nothing compiled it — its example used `?` and `.await` at the top
//! level of a `rust` block, so it had never compiled and could not have. That is
//! the first code a visitor reads.
//!
//! It cannot be attached to a published crate. `include_str!` resolves at compile
//! time against the file tree, and `crates/happenstance/src/../../../README.md`
//! does not exist inside a packaged `.crate` — so a published crate carrying that
//! attribute would fail `cargo test` for anyone who ran it. `xtask` has
//! `publish = false` and is built by CI on every push, which makes it the one
//! place the repository README can be type-checked without shipping a hazard.
//!
//! Each *crate* README is compiled by its own crate, where the relative path is
//! inside the package and stays correct after publication.

// `cfg(doctest)` so the README's prose is not spliced into xtask's rendered
// documentation, where it would be actively confusing — this crate is a task
// runner, not the library the README describes.
#![cfg_attr(doctest, doc = include_str!("../../README.md"))]

// The Rust constitution's examples, one private module per atom. Same argument
// as the README above — the files live outside every publishable package — but
// with a stricter obligation: `standards/rust/` claims its examples compile, and this
// is the only place that claim is checked. See the module's own docs for why the
// atoms are not all attached to one module.
mod constitution;

// The narrative tree's pages, one private module per page. Same argument again
// — `docs/` lives outside every publishable package — and the same obligation
// as the constitution's: `docs/` claims its examples compile against the crates
// a reader installed, and this is the only place that claim is checked. It must
// stay in *this* target: `cargo test --doc` compiles the lib crate's doctests,
// so the same line in `main.rs` would compile clean and check nothing.
mod narrative;

// The pointer policy and its register (`docs-that-teach`, HS-S0154). In the
// *lib*, deliberately: `main.rs` is a separate crate root with private modules,
// so a register there is unreachable to a consumer. Here it is `pub`, which is
// what keeps `dead_code` quiet over a register nothing reads yet and what lets
// a same-package bin write `use xtask::pointers::…` when HS-P0020's step lands.
pub mod pointers;
