//! The derivation contract: what `trait_variant` must emit, checked by every build of this crate.
//!
//! [`SendEventStore`], [`SendProjectionStore`] and the blanket impls that make each of them
//! the bare flavour as well are not written in this repository. `#[trait_variant::make]`
//! emits them when the crate compiles. The workspace requires `trait-variant = "0.1.3"`, a
//! caret, so a consumer's own resolve may pick any later `0.1.z`, and this workspace's
//! `Cargo.lock` does not travel with the published crate. ADR-0092 keeps the caret and
//! answers the exposure here.
//!
//! The items below are library code, not `#[cfg(test)]`. They are type-checked wherever
//! `happenstance-core` is compiled: in this workspace's `--locked` gate, in the weekly
//! `floating-deps` CI job that resolves without the lockfile, and in a consumer's build at
//! whatever `trait-variant` that consumer's resolve picked. A release that emits a different
//! shape fails that build here, at a line that names what moved, rather than in the
//! consumer's generic code with an error about their own bounds.
//!
//! # How the assertions work
//!
//! Each one is a generic function bounded by a `Send` flavour, so its obligations are
//! discharged at the definition, from the bound alone (RS-61-1). Nothing has to be
//! instantiated, so the block needs no concrete store and no feature, and it holds under
//! `--no-default-features` and on `wasm32`. A concrete store would answer the `Send`
//! questions by auto-trait leakage from its hidden types, whatever the trait says.
//!
//! They sit inside `const _: () = { … };`, an anonymous constant, so nothing outside the
//! block can name them and the crate's surface does not grow. The functions are never
//! called. Their names start with `_`, which is how Rust spells a deliberately unused item,
//! so `dead_code` does not report them. The alternative, `#[expect(dead_code)]`, is a lint
//! exception and would say the same thing with more ceremony.
//!
//! # What this does not cover
//!
//! It checks what generic code depends on: the bridge (CLAUDE.md binding constraint 4), the
//! `Send` supertrait, and `Send` on what the `Send` flavour returns. It does not check that
//! the derived trait still copies the base trait's attributes. That is documentation, not a
//! type: `missing_docs` guards the doc comment, and `tests/trait_variant_pin.rs` ties the
//! two `#[doc(alias)]` keys to the locked version.
//!
//! It does not reject the `async fn read` refactor binding constraint 3 forbids either. That
//! is a change to this crate's source, not to the derivation, and `spawns_from_generic` in
//! `memory.rs` rejects it.

// Only the `Send` flavours are imported, as binding constraint 4 asks of every module: with
// both names in scope a method call is ambiguous. The bare flavours are named by path below,
// and every call is fully qualified, so no line here depends on method resolution.
use crate::{Event, Query, ReadOptions, SendEventStore, SendProjectionStore};

const _: () = {
    /// The bridge, event-store side. Implementing [`SendEventStore`] must make a type an
    /// [`EventStore`](crate::EventStore).
    ///
    /// Binding constraint 4 depends on it: generic code binds the bare flavour and receives
    /// every native adapter, which implements only the derived one. A `trait-variant` that
    /// stops emitting `impl<T: SendEventStore> EventStore for T` fails here with `E0277`,
    /// "the trait bound `S: EventStore` is not satisfied".
    fn _every_send_event_store_is_an_event_store<S: SendEventStore>() {
        fn bare<S: crate::EventStore>() {}
        bare::<S>();
    }

    /// `append`'s future on the `Send` flavour is `Send`, read from the trait alone.
    ///
    /// A `trait-variant` that stops adding the variant's bound to an `async fn`'s future
    /// fails here, and `tokio::spawn` of an append in a consumer's generic code would
    /// otherwise be the first thing to notice.
    fn _send_flavour_append_future_is_send<S: SendEventStore>(store: &S, events: &[Event]) {
        fn is_send<T: Send>(_: &T) {}
        is_send(&SendEventStore::append(store, events, None));
    }

    /// `read`'s stream on the `Send` flavour is `Send` at the top level of the return type.
    ///
    /// The stream, not a future producing it, because `read` is not `async` (binding
    /// constraint 3). A `trait-variant` that stops adding the bound to `-> impl Trait`
    /// return types fails here.
    fn _send_flavour_read_stream_is_send<S: SendEventStore>(store: &S, query: &Query) {
        fn is_send<T: Send>(_: &T) {}
        is_send(&SendEventStore::read(store, query, ReadOptions::new()));
    }

    /// The bridge, projection side. Implementing [`SendProjectionStore`] must make a type
    /// a [`ProjectionStore`](crate::ProjectionStore). The same derivation emits it, so a
    /// release that changes one bridge has changed both.
    fn _every_send_projection_store_is_a_projection_store<S: SendProjectionStore>() {
        fn bare<S: crate::ProjectionStore>() {}
        bare::<S>();
    }

    /// `begin`'s future on the `Send` projection flavour is `Send`. It is the first await of
    /// every projection runner, so a runner spawned from generic code needs it.
    fn _send_projection_flavour_begin_future_is_send<S: SendProjectionStore>(store: &S) {
        fn is_send<T: Send>(_: &T) {}
        is_send(&SendProjectionStore::begin(store));
    }

    /// Each `Send` flavour is itself `Send`: `trait_variant` emits the variant's bound as a
    /// supertrait, `pub trait SendEventStore: Send`. Generic code that moves a store into a
    /// spawned task may lean on that rather than spell `+ Send`, and `spawns_from_generic`
    /// spells it, so without this a release that dropped the supertrait would pass the
    /// workspace's own tests.
    fn _every_send_flavour_is_send<E: SendEventStore, P: SendProjectionStore>() {
        fn is_send<T: Send>() {}
        is_send::<E>();
        is_send::<P>();
    }
};
