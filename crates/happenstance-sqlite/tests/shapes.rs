//! Type-level assertions about the SQLite skeleton's real associated types.
//!
//! Nothing here runs a query — every body is `todo!()` in the crate under test.
//! These are compile-time obligations, and the whole point of the skeleton is
//! that the compiler is the instrument.
//!
//! Each half is gated on the feature that provides it, so that the feature
//! powerset `cargo hack` runs does not compile a test against a module that was
//! configured out.

#[cfg(feature = "event-store")]
mod event_store {
    use futures_core::Stream;
    use happenstance_core::{EventStore, Query, ReadOptions, SequencedEvent};
    use happenstance_sqlite::event_store::{
        SqliteEventStore, SqliteEventStoreError, SqliteReadStream,
    };

    /// The read stream is `Send`, which is the whole reason the `Send` flavour
    /// is implementable at all. It is `Unpin` too, which is why `poll_next`
    /// needs no pin projection and therefore no `unsafe` — `unsafe_code` is
    /// `forbid`den workspace-wide, so a projection would have had to be a
    /// dependency instead.
    #[test]
    fn read_stream_is_send_and_unpin() {
        fn assert_send<T: Send>() {}
        fn assert_unpin<T: Unpin>() {}

        assert_send::<SqliteReadStream>();
        assert_unpin::<SqliteReadStream>();
    }

    /// `Sync` on the store is the load-bearing half. `rusqlite::Connection` is
    /// `!Sync`, and every future in the `Send` flavour captures `&self`, so
    /// without the `Mutex` this impl simply does not exist.
    #[test]
    fn store_is_send_and_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<SqliteEventStore>();
        assert_sync::<SqliteEventStore>();
    }

    /// The error satisfies the port's `core::error::Error + 'static` bound, and
    /// is `Send + Sync` besides — not required by the port, but required by
    /// anyone putting one in a `tokio::spawn`ed task's `Result`.
    #[test]
    fn error_is_send_sync_and_static() {
        fn assert_error<T: core::error::Error + Send + Sync + 'static>() {}

        assert_error::<SqliteEventStoreError>();
    }

    /// Generic code binds the weaker `EventStore`, per the workspace's
    /// constraint 4, and the `Send` impl satisfies it.
    #[test]
    fn send_impl_satisfies_the_bare_bound() {
        fn takes_any_store<S: EventStore>(store: &S, query: &Query) {
            let _stream = store.read(query, ReadOptions::new());
        }

        let _ = takes_any_store::<SqliteEventStore>;
    }

    /// ADR-0008, applied to a real driver. The stream is `Send` where the bound
    /// is written at the *definition*, so the obligation is discharged before
    /// monomorphisation rather than by auto-trait leakage from a concrete type.
    ///
    /// `SendEventStore` is named through its full path rather than imported,
    /// because `EventStore` is already in scope here and having both in scope
    /// makes `store.read(..)` ambiguous (`error[E0034]`).
    #[test]
    fn send_flavour_stream_is_send_in_generic_code() {
        fn assert_stream_is_send<S: happenstance_core::SendEventStore>(store: &S, query: &Query) {
            fn is_send<T: Send>(_: &T) {}

            is_send(&happenstance_core::SendEventStore::read(
                store,
                query,
                ReadOptions::new(),
            ));
        }

        let _ = assert_stream_is_send::<SqliteEventStore>;
    }

    /// The concrete stream really is the trait's `impl Stream`, item type and
    /// all — a weaker assertion than the ones above, but it is what catches a
    /// refactor that quietly starts returning something else.
    #[test]
    fn read_returns_the_named_stream_type() {
        fn accepts_the_stream<S>(stream: S) -> S
        where
            S: Stream<Item = Result<SequencedEvent, SqliteEventStoreError>> + Send,
        {
            stream
        }

        let _ = accepts_the_stream::<SqliteReadStream>;
    }
}

#[cfg(feature = "projection-store")]
mod projection_store {
    use happenstance_core::ProjectionStore;
    use happenstance_sqlite::projection_store::{
        SqliteBatch, SqliteProjectionStore, SqliteProjectionStoreError,
    };

    /// The batch crosses an await by construction — the port passes it between
    /// `begin` and `commit`, both `async` — so it has to be `Send`. A
    /// `rusqlite::Transaction<'_>` is not, which is the second of the two
    /// independent reasons it cannot be this adapter's batch.
    ///
    /// `'static` is the stronger claim, and it is the one the port's
    /// `type Batch<'a>` exists to avoid requiring.
    #[test]
    fn batch_is_send_sync_and_owns_everything() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        fn assert_static<T: 'static>() {}

        assert_send::<SqliteBatch>();
        assert_sync::<SqliteBatch>();
        assert_static::<SqliteBatch>();
    }

    /// Same reasoning as the event store: the `Mutex` is what makes `&Self`
    /// `Send`, and `&Self` is captured by every future in the trait.
    #[test]
    fn store_is_send_and_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<SqliteProjectionStore>();
        assert_sync::<SqliteProjectionStore>();
    }

    /// The error satisfies the port's bound and is `Send + Sync` besides.
    #[test]
    fn error_is_send_sync_and_static() {
        fn assert_error<T: core::error::Error + Send + Sync + 'static>() {}

        assert_error::<SqliteProjectionStoreError>();
    }

    /// The `Send` impl satisfies the weaker bare bound, and the GAT normalises
    /// to the owned batch through it — at `'static`, which is the observation
    /// that an owned batch makes available and a borrowed one does not.
    #[test]
    fn send_impl_satisfies_the_bare_bound() {
        fn takes_any_projection_store<P: ProjectionStore>() {}
        fn batch_normalises_to_the_owned_type<P>(batch: SqliteBatch) -> P::Batch<'static>
        where
            // `+ 'static` is not decoration. Naming `P::Batch<'static>` at all
            // requires it, because the port declares `type Batch<'a> where Self: 'a`
            // and that obligation propagates to every generic consumer:
            // without it this is
            // `error[E0310]: the parameter type 'P' may not live long enough`.
            // The clause is optional on an *impl* that binds an owned type, but
            // it is mandatory here — so the GAT costs callers something even
            // when no adapter uses the lifetime.
            P: ProjectionStore<Batch<'static> = SqliteBatch> + 'static,
        {
            batch
        }

        let _ = takes_any_projection_store::<SqliteProjectionStore>;
        let _ = batch_normalises_to_the_owned_type::<SqliteProjectionStore>;
    }
}
