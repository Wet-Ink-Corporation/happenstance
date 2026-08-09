//! Probes for ADR-0016 (the wire format). One test per measured claim.
//!
//! Run everything with `cargo test -- --nocapture` from this directory (see
//! `README.md` for the mapping from test name to ADR claim). This file holds
//! the two decorative-rule measurements that have to be doctests or live
//! beside a reusable detection mechanism; every numeric probe (W1-W6) is an
//! integration test under `tests/`.
//!
//! # D1 — the honest `compile_fail`, written the way WF-12 means it
//!
//! At HEAD, `ReadOptions` DOES implement `Serialize` (it is a plain derive at
//! `query.rs`), so this snippet COMPILES, so the `compile_fail` doctest
//! FAILS. That failure is the control: it proves the honest spelling is
//! capable of catching the thing WF-12 forbids — and it is the one doctest
//! in this file marked `ignore` rather than left live, because a `cargo
//! test` in this experiment must exit zero (see `README.md`), and a doctest
//! whose whole point is to demonstrate a red result cannot do that and also
//! keep the suite green. Its output was captured once, by hand, and is
//! quoted verbatim below rather than re-run on every `cargo test`:
//!
//! ```text
//! $ cargo test --doc -- --nocapture
//! Test compiled successfully, but it's marked `compile_fail`.
//! test src\lib.rs - (line 16) - compile fail ... FAILED
//! ```
//!
//! ```compile_fail,ignore
//! fn assert_serialisable<T: serde::Serialize>() {}
//! assert_serialisable::<happenstance_core::ReadOptions>();
//! ```
//!
//! To re-run D1 live and see the FAILED result yourself: delete `,ignore`
//! from the fence above and run `cargo test --doc`. Restore it afterwards,
//! or the crate's test suite will no longer exit zero.
//!
//! # D2 — the same doctest, type name misspelled
//!
//! `ReadOptionz` does not exist. The snippet fails to compile for an
//! unrelated reason, so the `compile_fail` doctest PASSES — green, while
//! `ReadOptions` is still fully serialisable.
//!
//! ```compile_fail
//! fn assert_serialisable<T: serde::Serialize>() {}
//! assert_serialisable::<happenstance_core::ReadOptionz>();
//! ```
//!
//! # D3 — the same doctest, trait name misspelled
//!
//! British spelling of `Serialize` is not a trait serde defines. Same
//! failure mode as D2, for a different typo.
//!
//! ```compile_fail
//! fn assert_serialisable<T: serde::Serialise>() {}
//! assert_serialisable::<happenstance_core::ReadOptions>();
//! ```
//!
//! # D4 — the same doctest, crate path wrong
//!
//! `happenstance::ReadOptions` does not exist (the type lives in
//! `happenstance_core`, not the facade crate). Same failure mode again.
//!
//! ```compile_fail
//! fn assert_serialisable<T: serde::Serialize>() {}
//! assert_serialisable::<happenstance::ReadOptions>();
//! ```
//!
//! D2, D3 and D4 below are live and all three currently pass (green) —
//! and all three assert nothing: each passes because the *snippet* fails to
//! compile for a reason that has nothing to do with whether `ReadOptions`
//! implements `Serialize`. D1's captured failure is the one case that shows
//! the instrument can ever fire at all. See
//! `decorative_readoptions_const_assert` below for the replacement: a
//! `const` assertion that fails the *build*, not the test, on every one of
//! D2-D4's typos, and fails cleanly and only when `ReadOptions` is genuinely
//! `Serialize`.

use core::marker::PhantomData;

/// Detects `impl Serialize for T` without running `T`'s code — inherent-impl
/// specialisation. `Detect<T>::IS_SERIALIZE` resolves to the inherent
/// `impl<T: Serialize> Detect<T>` when one exists (Rust prefers the more
/// specific impl), and falls back to the blanket `NotSerialize` default
/// otherwise. A typo in `T`'s name is an unresolved-path error — it fails the
/// *build*, so it can never present as a passing test the way a
/// `compile_fail` doctest's typo can.
pub struct Detect<T>(pub PhantomData<T>);

/// Blanket default: not `Serialize`, unless the inherent impl below applies.
pub trait NotSerialize {
    /// `false` unless `T: Serialize`, in which case the inherent impl's
    /// `true` shadows this one.
    const IS_SERIALIZE: bool = false;
}
impl<T> NotSerialize for Detect<T> {}

impl<T: serde::Serialize> Detect<T> {
    /// Present only when `T: Serialize`; shadows the trait default above.
    #[allow(dead_code)]
    pub const IS_SERIALIZE: bool = true;
}

/// A type with no `Serialize` impl at all, for contrast.
pub struct NoImpls;

#[cfg(test)]
mod tests {
    use super::*;
    use happenstance_core::{Event, ReadOptions};

    /// **Decorative measurement.** Backs the WF-12 replacement rule in
    /// ADR-0016 decision §10: a `const _: () = assert!(...)` using
    /// `Detect<T>`, in place of the `compile_fail` doctest above.
    ///
    /// Measured: `Detect::<ReadOptions>::IS_SERIALIZE` is `true` at HEAD
    /// (`ReadOptions` derives `Serialize`), `Detect::<NoImpls>::IS_SERIALIZE`
    /// and a hypothetical `Detect::<ReadOptionz>` (a typo) behave
    /// differently in kind: the typo does not compile at all, rather than
    /// evaluating to `false`. That is the property the doctest form cannot
    /// have.
    #[test]
    fn decorative_readoptions_const_assert() {
        println!("=== decorative: Detect<T> const-evaluation vs compile_fail doctest ===");
        println!(
            "Detect::<ReadOptions>::IS_SERIALIZE = {}  (ReadOptions derives Serialize at HEAD)",
            Detect::<ReadOptions>::IS_SERIALIZE
        );
        println!(
            "Detect::<NoImpls>::IS_SERIALIZE     = {}  (no Serialize impl)",
            Detect::<NoImpls>::IS_SERIALIZE
        );
        println!(
            "Detect::<Event>::IS_SERIALIZE       = {}  (Event does implement Serialize; contrast type)",
            Detect::<Event>::IS_SERIALIZE
        );
        println!();
        println!("The assertion this backs is:");
        println!("  const _: () = assert!(!Detect::<ReadOptions>::IS_SERIALIZE,");
        println!("      \"WF-12: ReadOptions must not implement Serialize\");");
        println!("which currently fails to compile with:");
        println!("  error[E0080]: evaluation of constant value failed");
        println!("  ...WF-12: ReadOptions must not implement Serialize");
        println!("— exactly the state WF-12 exists to forbid, caught at the point the");
        println!("obligation is violated rather than by a doctest that can pass for the wrong");
        println!("reason. A typo in the type name (`ReadOptionz`) is E0425, a build failure,");
        println!("not a green test — measured separately, see README.md's table.");

        assert!(
            Detect::<ReadOptions>::IS_SERIALIZE,
            "measured claim: ReadOptions is Serialize at HEAD"
        );
        assert!(
            !Detect::<NoImpls>::IS_SERIALIZE,
            "control: a type with no impl must read false"
        );
    }
}
