//! Compiles every example in `docs/rust/` — the Rust constitution.
//!
//! The constitution's whole enforcement model is that its examples are checked
//! by the compiler rather than by a reader. That requires a crate to attach them
//! to, and it cannot be a published one: `include_str!` resolves at compile time
//! against the file tree, and `crates/happenstance/src/../../../docs/rust/…`
//! does not exist inside a packaged `.crate`. `xtask` is `publish = false` and is
//! built by CI on every push, which is the same argument [`crate`]'s root already
//! makes for the repository README.
//!
//! # One module per atom, deliberately
//!
//! The obvious spelling is several `#![doc = include_str!(…)]` attributes on one
//! module. It is wrong: they concatenate into a single doc string, so a failure
//! in the nineteenth atom reports a line number counted from the first, which
//! maps to no file a reader can open. One module per atom makes the failure read
//! `xtask::constitution::send_is_not_inherited (line 42)`, with 42 relative to
//! the atom.
//!
//! # What this file does not prove
//!
//! `cfg(doctest)` means these includes are never expanded under `cargo check`,
//! `cargo clippy` or `cargo build`, so a renamed or deleted atom is invisible to
//! every step except `cargo test`. That is why `lint-constitution`'s router and
//! module checks read the directory rather than trusting the compiler to notice.
//!
//! Doctests also do not receive the workspace `[lints]` set, and `cargo clippy`
//! does not lint doctests at all, so `unwrap_used = "deny"` and the `pedantic`
//! group are unenforced inside every example here.
//!
//! `RUSTDOCFLAGS=-D warnings` recovers rustc's *default-on* lints inside a
//! doctest — a probe confirmed `non_snake_case` fails the build under it — but
//! not the workspace's `[lints]` table and not clippy, which is where
//! `unwrap_used` lives. So the recovery is partial, and `lint-constitution`
//! greps the fences for the two spellings that matter rather than pretending
//! otherwise.

// Each module exists only while rustdoc is collecting doctests, so a normal
// build carries neither the module nor the included prose.

#[cfg(doctest)]
mod prime_directives {
    #![doc = include_str!("../../docs/rust/00-prime-directives.md")]
}

#[cfg(doctest)]
mod standard_of_evidence {
    #![doc = include_str!("../../docs/rust/01-standard-of-evidence.md")]
}

#[cfg(doctest)]
mod newtypes_and_niches {
    #![doc = include_str!("../../docs/rust/10-newtypes-and-niches.md")]
}

#[cfg(doctest)]
mod const_construction_and_panics {
    #![doc = include_str!("../../docs/rust/11-const-construction-and-panics.md")]
}

#[cfg(doctest)]
mod manual_impls_and_derive_traps {
    #![doc = include_str!("../../docs/rust/12-manual-impls-and-derive-traps.md")]
}

#[cfg(doctest)]
mod sealing_and_exhaustiveness {
    #![doc = include_str!("../../docs/rust/13-sealing-and-exhaustiveness.md")]
}

#[cfg(doctest)]
mod two_flavour_ports {
    #![doc = include_str!("../../docs/rust/20-two-flavour-ports.md")]
}

#[cfg(doctest)]
mod send_is_not_inherited {
    #![doc = include_str!("../../docs/rust/21-send-is-not-inherited.md")]
}

#[cfg(doctest)]
mod rpitit_and_lifetime_capture {
    #![doc = include_str!("../../docs/rust/22-rpitit-and-lifetime-capture.md")]
}

#[cfg(doctest)]
mod streams_and_state_machines {
    #![doc = include_str!("../../docs/rust/23-streams-and-state-machines.md")]
}

#[cfg(doctest)]
mod the_blocking_bridge {
    #![doc = include_str!("../../docs/rust/24-the-blocking-bridge.md")]
}

#[cfg(doctest)]
mod what_removes_send_and_sync {
    #![doc = include_str!("../../docs/rust/25-what-removes-send-and-sync.md")]
}

#[cfg(doctest)]
mod error_taxonomy {
    #![doc = include_str!("../../docs/rust/30-error-taxonomy.md")]
}

#[cfg(doctest)]
mod public_surface_and_evolution {
    #![doc = include_str!("../../docs/rust/40-public-surface-and-evolution.md")]
}

#[cfg(doctest)]
mod declarative_macros {
    #![doc = include_str!("../../docs/rust/41-declarative-macros.md")]
}

#[cfg(doctest)]
mod dependency_hygiene {
    #![doc = include_str!("../../docs/rust/50-dependency-hygiene.md")]
}

#[cfg(doctest)]
mod features_and_no_std {
    #![doc = include_str!("../../docs/rust/51-features-and-no-std.md")]
}

#[cfg(doctest)]
mod wasm32_and_target_cfg {
    #![doc = include_str!("../../docs/rust/52-wasm32-and-target-cfg.md")]
}

#[cfg(doctest)]
mod what_a_test_must_prove {
    #![doc = include_str!("../../docs/rust/60-what-a-test-must-prove.md")]
}

#[cfg(doctest)]
mod compile_time_assertions {
    #![doc = include_str!("../../docs/rust/61-compile-time-assertions.md")]
}

#[cfg(doctest)]
mod doctests_and_harnesses {
    #![doc = include_str!("../../docs/rust/62-doctests-and-harnesses.md")]
}

#[cfg(doctest)]
mod rustdoc_obligations {
    #![doc = include_str!("../../docs/rust/70-rustdoc-obligations.md")]
}

#[cfg(doctest)]
mod the_gate {
    #![doc = include_str!("../../docs/rust/80-the-gate.md")]
}

#[cfg(doctest)]
mod checks_that_cannot_be_types {
    #![doc = include_str!("../../docs/rust/81-checks-that-cannot-be-types.md")]
}

#[cfg(doctest)]
mod skeletons_and_todo {
    #![doc = include_str!("../../docs/rust/90-skeletons-and-todo.md")]
}

#[cfg(doctest)]
mod adapter_authoring_recipe {
    #![doc = include_str!("../../docs/rust/91-adapter-authoring-recipe.md")]
}

#[cfg(doctest)]
mod toolchain_limits_and_dead_ends {
    #![doc = include_str!("../../docs/rust/92-toolchain-limits-and-dead-ends.md")]
}
