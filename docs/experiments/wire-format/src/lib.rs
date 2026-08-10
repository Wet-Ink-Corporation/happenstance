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

/// Blanket default: not `DeserializeOwned`, unless the inherent impl below
/// applies.
///
/// **This half is not a transcription of the `Serialize` half above it**, and
/// the difference is the whole reason it is written out here rather than left
/// to inference. `Deserialize<'de>` is generic over the lifetime of the data it
/// borrows *from*, so `impl<T: serde::Deserialize> Detect<T>` is
/// `error[E0106]: missing lifetime specifier`. What is wanted is a
/// higher-ranked bound — `for<'de> Deserialize<'de>`, read as "for *every*
/// lifetime `'de`" — and the compiler's own suggestion of `impl<'a, T:
/// Deserialize<'a>>` is the wrong repair, because that makes the impl generic
/// over one caller-chosen lifetime instead of requiring the bound for all of
/// them. `serde::de::DeserializeOwned` is shorthand for exactly that HRTB, and
/// is the spelling to use (ADR-0016 §13).
///
/// One honest limit, measured: a *borrowing* `Deserialize` — a type holding a
/// `&'a str` — does not fall back to `false` here; it is a hard "implementation
/// of `Deserialize` is not general enough". So this answers "does `T`
/// deserialise from owned data", not "does `T` implement `Deserialize` at all".
/// That does not bite for `ReadOptions`, which owns every field.
pub trait NotDeserialize {
    /// `false` unless `T: DeserializeOwned`, in which case the inherent impl's
    /// `true` shadows this one.
    const IS_DESERIALIZE: bool = false;
}
impl<T> NotDeserialize for Detect<T> {}

impl<T: serde::de::DeserializeOwned> Detect<T> {
    /// Present only when `T: DeserializeOwned`; shadows the trait default above.
    #[allow(dead_code)]
    pub const IS_DESERIALIZE: bool = true;
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

        // The reading this experiment was written to take was
        // `Detect::<ReadOptions>::IS_SERIALIZE == true`, on 2026-08-09 against
        // pre-removal HEAD; that is the measurement §13 rests on and it is
        // recorded above rather than asserted, because WF-12's whole point was
        // to make it false. The assertion moved with the tree instead of being
        // left to panic: an experiment is a record of a measurement, but a
        // record that aborts is not a record of anything.
        assert!(
            !Detect::<ReadOptions>::IS_SERIALIZE,
            "WF-12 landed: ReadOptions lost Serialize at ADR-0016 §13, and the \
             true reading above is the dated one this experiment took"
        );
        assert!(
            !Detect::<NoImpls>::IS_SERIALIZE,
            "control: a type with no impl must read false"
        );
        assert!(
            Detect::<Event>::IS_SERIALIZE,
            "positive control: a negated assertion is otherwise satisfied by a \
             detector whose inherent impl never applies, and that fails silently"
        );

        // The `Deserialize` half, which ADR-0016 §13 owes this crate so that the
        // HRTB finding stops resting on a throwaway one. It is the same three
        // readings, and it is not a transcription: the bound is
        // `DeserializeOwned`, because `serde::Deserialize` cannot be named
        // without a lifetime.
        println!();
        println!(
            "Detect::<ReadOptions>::IS_DESERIALIZE = {}  (deleted with Serialize at ADR-0016 §13)",
            Detect::<ReadOptions>::IS_DESERIALIZE
        );
        println!(
            "Detect::<NoImpls>::IS_DESERIALIZE     = {}  (no impl)",
            Detect::<NoImpls>::IS_DESERIALIZE
        );
        println!(
            "Detect::<Event>::IS_DESERIALIZE       = {}  (derived; contrast type)",
            Detect::<Event>::IS_DESERIALIZE
        );
        assert!(
            !Detect::<ReadOptions>::IS_DESERIALIZE,
            "WF-12's other half: ReadOptions must not deserialise either"
        );
        assert!(!Detect::<NoImpls>::IS_DESERIALIZE, "control");
        assert!(
            Detect::<Event>::IS_DESERIALIZE,
            "positive control, for the same reason as the Serialize one"
        );
    }
}
