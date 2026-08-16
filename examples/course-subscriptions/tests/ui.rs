//! The compiler protects the domain, asserted rather than believed.
//!
//! Once a decision model's fold is a `match` over its own domain enum with no
//! wildcard arm, adding a variant is `error[E0004]: non-exhaustive patterns` in
//! the file the application author owns. That is the project's central claim,
//! and it was a convention until this target existed.
//!
//! # Why `trybuild` and not a `compile_fail` doctest
//!
//! Two defects, both measured in this repository rather than assumed. rustdoc
//! collects doctests from the **lib target only**, so a `compile_fail` block in
//! `tests/` is compiled as prose and handed to no compiler — phase 4 shipped
//! exactly that and phase 5 found it (`RUNBOOK.md:3614-3627`), and this package
//! has no lib target at all. And on 1.97.1 rustdoc *silently ignores* an
//! error-code annotation it cannot match, so `compile_fail,E0004` asserts no
//! more than bare `compile_fail`, which passes on **any** compile error
//! (`spec/SPECIFICATION.md:8772`; the same warning is written into
//! `crates/happenstance-core/src/event.rs:95-106`).
//!
//! The consequence is precise and it is the whole reason for the dependency:
//! under a doctest the **negative control cannot discriminate**, and the
//! negative control is the entirety of what is being claimed here.
//!
//! # Why the fixtures live beside the example rather than under `crates/`
//!
//! A `trybuild` fixture's `-->` span is its own path, and the guarantee has to
//! name a file the author wrote — a diagnostic pointing into a macro body or
//! into `crates/` is a guarantee they cannot act on. So the fixtures sit in
//! this package, which is also the only placement that works mechanically: a
//! variant cannot be added to an imported enum, and this package is a binary
//! crate with no lib target to import from.
//!
//! # Why two tests rather than one
//!
//! `xtask/src/proof.rs`'s `ARTEFACTS` row names tests, not targets, and asserts
//! them out of `cargo test -- --list` before running them. One test driving
//! both cases could only be named once, and the case it did not name could then
//! be dropped from the pair in silence. Two tests, two names, one row.

mod ui {
    /// A variant the fold has not been taught stops the build.
    ///
    /// The fixture restates the worked example's domain enum plus one variant
    /// and keeps the `Seats` fold otherwise verbatim, **with no `_ =>` arm** —
    /// that absence is the protection. `trybuild` asserts both that the build
    /// fails and that its stderr matches the checked-in snapshot byte for byte,
    /// so a span that moves fails this test rather than passing quietly.
    ///
    /// The snapshot is *generated*, under `rust-toolchain.toml`'s pinned
    /// 1.97.1, with `TRYBUILD=overwrite`, and then read before it is committed.
    /// It is never hand-written and never hand-edited to make a case pass: a
    /// snapshot edited to agree with whatever happened is a test that asserts
    /// nothing.
    #[test]
    fn an_unhandled_variant_fails_to_compile() {
        let cases = trybuild::TestCases::new();
        cases.compile_fail("tests/ui/unhandled_variant.rs");
    }

    /// The red build is red for the stated reason, and not for another one.
    ///
    /// The same program with the new variant handled by a real arm — one line
    /// is the whole delta between the two files — which must **compile and
    /// run**. Without it, a typo, a renamed import or an item moved behind a
    /// feature would fail the case above and be banked as the guarantee. With
    /// it, any unrelated breakage turns both cases red at once and is legible
    /// as breakage (RS-62-1: pair every compile-fail with a compiling one, and
    /// do not trust the error code).
    #[test]
    fn the_negative_control_compiles() {
        let cases = trybuild::TestCases::new();
        cases.pass("tests/ui/handled_variant.rs");
    }
}
