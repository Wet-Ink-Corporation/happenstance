//! What `trait_variant` must emit for `SendProjection`, checked every build.
//!
//! `SendProjection` and the blanket impl that makes every implementor a
//! `Projection` are not written in this repository: `#[trait_variant::make]`
//! emits them when the crate compiles. The requirement on `trait-variant` is
//! the workspace's caret, so a consumer's own resolve may pick any later
//! `0.1.z`. ADR-0092 keeps the caret and answers that exposure with a
//! derivation contract in `happenstance-core/src/derivation.rs`; this is the
//! same contract for the second derivation, the one this crate makes.
//!
//! Each item is a generic function bounded by the `Send` flavour, so its
//! obligations are discharged at the definition, from the bound alone. They
//! sit inside an anonymous `const _`, so nothing outside can name them, and
//! they are never called: the leading `_` is how Rust spells a deliberately
//! unused item, so `dead_code` stays quiet without a lint exception.
//!
//! Only `SendProjection` is imported. The bare flavour is reached by path and
//! every call is fully qualified, so nothing here depends on method lookup.

use happenstance_core::ProjectionStore;

use crate::delivered::Delivered;
use crate::runner::SendProjection;

const _: () = {
    /// The bridge: implementing `SendProjection` makes a type a `Projection`.
    ///
    /// `run_projection` binds the bare flavour, so a `SendProjection` reaches
    /// it only through this blanket impl. A `trait-variant` that stopped
    /// emitting it fails here with `E0277`.
    fn _every_send_projection_is_a_projection<P: SendProjection>() {
        fn bare<P: crate::runner::Projection>() {}
        bare::<P>();
    }

    /// `apply`'s future on the `Send` flavour is `Send`, from the trait alone.
    fn _send_flavour_apply_future_is_send<P: SendProjection>(
        projection: &mut P,
        event: Delivered<P::Event>,
        batch: &mut <P::Store as ProjectionStore>::Batch,
    ) {
        fn is_send<T: Send>(_: &T) {}
        is_send(&SendProjection::apply(projection, event, batch));
    }

    /// The `Send` flavour keeps `Error`'s `From` bound.
    ///
    /// So `?` on a store error compiles inside an `apply` written against
    /// `SendProjection`.
    fn _send_flavour_error_converts_from_the_store_error<P: SendProjection>(
        error: <P::Store as ProjectionStore>::Error,
    ) -> P::Error {
        <P::Error as From<_>>::from(error)
    }

    /// The blanket impl forwards `Error` unchanged.
    ///
    /// The error a `SendProjection` declares is the one `run_projection`
    /// reports.
    fn _send_projection_error_is_the_projection_error<P: SendProjection>() {
        fn same<P: crate::runner::Projection<Error = E>, E>() {}
        same::<P, <P as SendProjection>::Error>();
    }

    /// The variant's supertrait: every `SendProjection` is itself `Send`.
    fn _every_send_projection_is_send<P: SendProjection>() {
        fn is_send<T: Send>() {}
        is_send::<P>();
    }
};
