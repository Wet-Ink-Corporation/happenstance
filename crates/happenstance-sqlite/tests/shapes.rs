//! Type-level assertions about the SQLite adapter's associated types.
//!
//! Nothing here runs a query, and that is still the point: every obligation
//! below is discharged by the compiler rather than by an execution, so it holds
//! for code paths no test happens to take. The bodies these types belong to are
//! real now and `tests/conformance.rs` runs them; what this target guards is the
//! shape, which a green suite would not notice losing.
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

    /// The `Send` impl satisfies the weaker bare bound, and the associated type
    /// normalises to this adapter's own owned batch through it.
    ///
    /// # What this test used to cost, and what it costs now
    ///
    /// The `where` clause below was
    /// `P: ProjectionStore<Batch<'static> = SqliteBatch> + 'static`, and the
    /// `+ 'static` was not decoration: naming `P::Batch<'static>` at all
    /// required it, because the port declared `type Batch<'a> where Self: 'a`
    /// and that obligation propagated into every generic consumer — without it,
    /// `error[E0310]: the parameter type 'P' may not live long enough`. The
    /// clause was optional on an *impl* binding an owned type and mandatory
    /// here, which was the demonstration that the GAT cost callers something
    /// even though no adapter in the workspace used the lifetime.
    ///
    /// ADR-0017 removed the lifetime, so the bound is now
    /// `P: ProjectionStore<Batch = SqliteBatch>` and the `+ 'static` is gone
    /// with the obligation that forced it. That deletion is the measurement.
    ///
    /// # Why it can still fail
    ///
    /// The associated-type equality is the assertion. Rebinding
    /// `SqliteProjectionStore::Batch` to anything other than `SqliteBatch` —
    /// including "improving" it to a `rusqlite::Transaction`, the shape the two
    /// findings above reject — makes this instantiation
    /// `error[E0271]: type mismatch resolving <SqliteProjectionStore as
    /// ProjectionStore>::Batch == SqliteBatch`. That is the AC-013 claim, that
    /// this adapter keeps the batch type its author chose, checked by the
    /// compiler rather than read off a diff.
    #[test]
    fn send_impl_satisfies_the_bare_bound() {
        fn takes_any_projection_store<P: ProjectionStore>() {}
        fn batch_normalises_to_the_owned_type<P>(batch: SqliteBatch) -> P::Batch
        where
            P: ProjectionStore<Batch = SqliteBatch>,
        {
            batch
        }

        let _ = takes_any_projection_store::<SqliteProjectionStore>;
        let _ = batch_normalises_to_the_owned_type::<SqliteProjectionStore>;
    }
}
