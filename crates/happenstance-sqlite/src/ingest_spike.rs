//! VT-10's instrument: `happenstance-sync`'s ingest port, implemented for this
//! adapter over the row writer `append` already uses.
//!
//! # What is being tested
//!
//! VT-10's falsifier reads *"a store adapter cannot implement `IngestStore`
//! without duplicating append's write path"*, and names SQLite as the
//! instrument. This module is that run. The write path it goes through is
//! `event_store::write_batch`, the function `append_locked` calls, generalised
//! to take a per-row `Origin`. There is one `INSERT INTO event` in the crate's
//! write path, and append and ingest both prepare it: every row is bound under
//! `ON CONFLICT (origin_store, origin_position) DO NOTHING`, and a local row's
//! `NULL` origin can never conflict. What is ingest-only is small enough to
//! name, and it lives in `event_store.rs` under `#[cfg(test)]`:
//!
//! * the `Origin::Foreign` arm of `write_batch`'s `match` — which binds the
//!   origin the peer sent instead of `NULL`, and refuses one past `i64::MAX`
//!   rather than saturating it — and the `Written::unheld_own` field it fills
//!   when a foreign row carrying this store's own identity **lands**, which
//!   only happens when the store did not hold it (SY-38);
//! * `ingest_locked`, beside `append_locked` — the same transaction and the
//!   same identity check, with no condition evaluated (SY-1), compensation
//!   written only for a group that landed something (SY-11), and the whole
//!   call rolled back as `IngestError::UnheldOwnIdentity` on that field;
//! * `ingest_groups` and `origin_watermark`, the `&self` entry points: the
//!   ceilings and the lock, and `WATERMARK_SQL`, one `GROUP BY` over the
//!   origin index, whose plan `plan_tests` asserts;
//! * `ingest_begin_error`, `begin_error`'s twin, which reports a write lock
//!   that went elsewhere as `IngestError::Busy`.
//!
//! The port's error is `IngestError<SqliteEventStoreError>`, and this file
//! states the adapter error's class for each variant (SY-36), so a runner
//! never infers one.
//!
//! # Why here, and why `cfg(test)`
//!
//! **Here** because the impl has to reach `write_batch`, `check_identity` and
//! `check_ceilings`, which are private to this crate, and coherence allows it:
//! a foreign trait on a local type. No crate outside this one could write it
//! without the writer being made public, which is the core-grows-a-write-path
//! option VT-10 rejects.
//!
//! **`cfg(test)`** because `happenstance-sync` is `publish = false` and its name
//! is unclaimed, so it can only be a path-only dev-dependency — which cargo
//! strips on publish — and an optional feature carrying a real dependency on it
//! would make `cargo publish` of this crate fail. It follows that every test of
//! the spike is a unit test in this file: a `cfg(test)` impl is invisible to
//! `tests/`. Phase 13 publishes the port and turns the gate into a feature,
//! which is additive.

use happenstance_core::StoreId;
use happenstance_sync::{Classify, FailureClass, IngestError, IngestGroup, Ingested, Watermark};
use rusqlite::ErrorCode;

use crate::event_store::{SqliteEventStore, SqliteEventStoreError};

// The `Send` flavour, because that is the flavour `SqliteEventStore` implements
// `EventStore` in — a `Mutex<Connection>` makes every future here `Send` — and
// `trait_variant`'s blanket impl hands generic code bound on `IngestStore` the
// other one for free, which the tests below rely on.
impl happenstance_sync::SendIngestStore for SqliteEventStore {
    // The bare adapter error. The port's `IngestError` carries the capacity
    // refusal, the busy store and the unheld own identity in arms of its own,
    // so a runner tells them apart without knowing this adapter.
    type Error = SqliteEventStoreError;

    fn store_id(&self) -> StoreId {
        // The inherent method, which path resolution prefers over this one.
        Self::store_id(self)
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        self.ingest_groups(groups)
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        self.origin_watermark()
    }
}

/// SY-36's class for each way this adapter fails.
///
/// Exhaustive over this crate's own enum, with no wildcard, so a new variant
/// does not compile until somebody decides its class. A busy code reaching
/// `Store` is `Transient` too: it is the same contention, raised after
/// `BEGIN IMMEDIATE`, and the transaction rolled back. So are a full disk, a
/// failed read or write and an exhausted allocator, which an operator or time
/// can fix while the batch waits. Every other driver failure is `Stop`: a
/// constraint, a corrupt file or a misused statement fails the same way again.
impl Classify for SqliteEventStoreError {
    fn class(&self) -> FailureClass {
        match self {
            Self::Sqlite(error) if transient_driver_failure(error) => FailureClass::Transient,
            // A cancelled blocking task did not run; a panicked one would again.
            Self::Worker(error) if error.is_cancelled() => FailureClass::Transient,
            // Over SQLite's `INTEGER`, so it can never land here, and the rest
            // of the batch can: VT-25's "park it".
            Self::OriginPositionOutOfRange { .. } => FailureClass::Park,
            Self::Sqlite(_)
            | Self::Worker(_)
            | Self::ConnectionPoisoned
            | Self::NoRuntime(_)
            | Self::InvalidPosition(_)
            | Self::StoredEventType(_)
            | Self::StoredTag(_)
            | Self::UnsupportedSchemaVersion { .. }
            | Self::MissingIdentity
            | Self::MalformedIdentity { .. }
            | Self::IdentityMoved { .. }
            | Self::UnstampedEvent { .. }
            | Self::CorruptTags => FailureClass::Stop,
        }
    }
}

/// Whether a driver failure is one that waiting can cure: the write lock held
/// elsewhere, a full disk, an I/O failure, or memory exhausted.
fn transient_driver_failure(error: &rusqlite::Error) -> bool {
    crate::connection::is_busy(error)
        || matches!(
            error.sqlite_error_code(),
            Some(ErrorCode::DiskFull | ErrorCode::SystemIoFailure | ErrorCode::OutOfMemory)
        )
}

// The whole file is `cfg(test)` already, through its declaration in `lib.rs`.
// Said again here so the module reads as test code where it stands.
#[cfg(test)]
mod tests {
    //! Each test names the clause it exercises. None asserts a literal local
    //! position: every position compared is one the store assigned, read back
    //! from `append`, `head` or the log. The literal positions below are
    //! **origin** positions — facts a peer supplies, not ones this store
    //! assigns.

    #![allow(clippy::unwrap_used)]

    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use futures_core::Stream;
    use happenstance_core::{
        AppendCondition, AppendError, Event, EventId, EventStore, Query, QueryItem, ReadOptions,
        RecordedAt, SequencePosition, SequencedEvent, StoreId, StoreLimit, Tags,
    };
    use happenstance_sync::{
        Classify, FailureClass, IngestError, IngestGroup, IngestStore, Ingested, ReplicatedEvent,
        Watermark,
    };

    use crate::event_store::{SqliteEventStore, SqliteEventStoreError};

    /// A store another peer runs.
    fn peer(byte: u8) -> StoreId {
        StoreId::from_bytes([byte; 16])
    }

    /// The identity `store` minted at its own `position`.
    fn id(store: StoreId, position: u64) -> EventId {
        EventId::new(store, SequencePosition::new(position).unwrap())
    }

    /// An event of `kind`, tagged `k:<tag>`.
    fn event(kind: &str, tag: &str) -> Event {
        Event::new(kind, b"payload".to_vec())
            .unwrap()
            .with_tags(Tags::from_pairs([("k", tag)]).unwrap())
    }

    /// An event as a peer ships it.
    fn shipped(id: EventId, millis: i64, event: Event) -> ReplicatedEvent {
        ReplicatedEvent::new(id, RecordedAt::from_millis(millis), event)
    }

    /// Ingests through the port, bound on the weaker flavour, so the blanket
    /// impl `trait_variant` derives is what every test goes through.
    async fn ingest<S: IngestStore>(
        store: &S,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<S::Error>> {
        store.ingest(groups).await
    }

    /// Every event in the store, in position order.
    async fn log<S: EventStore>(store: &S) -> Vec<SequencedEvent>
    where
        S::Error: core::fmt::Debug,
    {
        let everything = Query::all();
        let mut stream = Box::pin(store.read(&everything, ReadOptions::default()));
        let mut out = Vec::new();
        while let Some(item) =
            core::future::poll_fn(|context| stream.as_mut().poll_next(context)).await
        {
            out.push(item.unwrap());
        }
        out
    }

    /// The one event in `log` carrying `id`.
    fn carrying(log: &[SequencedEvent], id: EventId) -> &SequencedEvent {
        let mut matching = log.iter().filter(|event| event.id == id);
        let found = matching.next().unwrap();
        assert!(matching.next().is_none(), "two events carry {id}");
        found
    }

    /// A store on a file of its own, so a second connection can look at the
    /// tables or reach the schema. Dropped after the store it backs, because
    /// Windows will not unlink an open file.
    struct TempFile(PathBuf);

    impl TempFile {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(1);
            let mut path = std::env::temp_dir();
            path.push(format!(
                "happenstance-sqlite-ingest-spike-{}-{}.db",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            let file = Self(path);
            file.remove();
            file
        }

        fn remove(&self) {
            for suffix in ["", "-wal", "-shm"] {
                let mut path = self.0.clone().into_os_string();
                path.push(suffix);
                let _ = std::fs::remove_file(PathBuf::from(path));
            }
        }

        fn inspect(&self) -> rusqlite::Connection {
            rusqlite::Connection::open(&self.0).unwrap()
        }
    }

    impl Drop for TempFile {
        fn drop(&mut self) {
            self.remove();
        }
    }

    /// VT-9's restated falsifier (ADR-0066): *`IngestStore` fails to carry a
    /// foreign `RecordedAt` unchanged*. One of the two stamps is before 1970,
    /// which the `u64` placeholder this port used to carry could not express.
    #[tokio::test]
    async fn ingest_preserves_foreign_event_id_and_recorded_at() {
        let store = SqliteEventStore::open_in_memory().unwrap();
        let from = peer(0xA1);
        let batch = [
            shipped(id(from, 3), -86_400_000, event("Shipped", "a")),
            shipped(id(from, 7), 1_700_000_000_000, event("Shipped", "b")),
        ];

        let ingested = ingest(&store, &[IngestGroup::new(&batch, &[])])
            .await
            .unwrap();
        assert_eq!(ingested.appended, 2);

        let log = log(&store).await;
        for sent in &batch {
            let held = carrying(&log, sent.id);
            assert_eq!(held.id, sent.id, "the origin identity was re-minted");
            assert_eq!(
                held.recorded_at, sent.recorded_at,
                "the origin's recorded time was restamped"
            );
            assert_eq!(held.event, sent.event);
        }
    }

    /// SY-11: a peer offers the same event again, and the store skips it —
    /// no error, no second row, no second tag row, no second count.
    #[tokio::test]
    async fn redelivery_is_a_skip_not_an_error() {
        let file = TempFile::new();
        let store = SqliteEventStore::open(&file.0).unwrap();
        let from = peer(0xB2);
        let first = [
            shipped(id(from, 1), 10, event("Shipped", "a")),
            shipped(id(from, 2), 20, event("Shipped", "b")),
        ];
        ingest(&store, &[IngestGroup::new(&first, &[])])
            .await
            .unwrap();

        let tables = |connection: &rusqlite::Connection| -> (i64, i64) {
            (
                connection
                    .query_row("SELECT count(*) FROM event_tag", [], |row| row.get(0))
                    .unwrap(),
                connection
                    .query_row("SELECT sum(events) FROM tag_cardinality", [], |row| {
                        row.get(0)
                    })
                    .unwrap(),
            )
        };
        let head = store.head().await.unwrap();
        let before = tables(&file.inspect());

        let again = ingest(&store, &[IngestGroup::new(&first, &[])])
            .await
            .unwrap();
        assert_eq!((again.appended, again.skipped), (0, 2));
        assert_eq!(store.head().await.unwrap(), head);
        assert_eq!(
            tables(&file.inspect()),
            before,
            "a skipped row wrote tag rows or counted its tags"
        );

        // Half held, half new: the held one is skipped and the new one lands.
        let mixed = [
            first[1].clone(),
            shipped(id(from, 3), 30, event("Shipped", "c")),
        ];
        let partial = ingest(&store, &[IngestGroup::new(&mixed, &[])])
            .await
            .unwrap();
        assert_eq!((partial.appended, partial.skipped), (1, 1));
        assert_eq!(log(&store).await.len(), 3);
    }

    /// SY-5: an ingested event lands at the tail, whatever position its origin
    /// gave it — here a lower one than the local head.
    #[tokio::test]
    async fn ingested_events_land_above_the_local_head() {
        let store = SqliteEventStore::open_in_memory().unwrap();
        store
            .append(
                &[
                    event("Local", "a"),
                    event("Local", "b"),
                    event("Local", "c"),
                ],
                None,
            )
            .await
            .unwrap();
        let local_head = store.head().await.unwrap().unwrap();

        let from = peer(0xC3);
        let batch = [shipped(id(from, 1), 1, event("Shipped", "x"))];
        ingest(&store, &[IngestGroup::new(&batch, &[])])
            .await
            .unwrap();

        let landed = carrying(&log(&store).await, batch[0].id).position;
        assert!(
            landed > local_head,
            "ingested at {landed}, below {local_head}"
        );
        assert_eq!(store.head().await.unwrap(), Some(landed));

        let next = store.append(&[event("Local", "d")], None).await.unwrap();
        assert!(next > landed);
    }

    /// SY-2 and SY-11: compensation lands with the losing event, after it, minted
    /// here — and a group whose foreign events are all held writes none.
    ///
    /// The last case is the ceiling: a group whose compensation could never fit
    /// is refused before the transaction opens, and the losing event it answers
    /// is not written either.
    #[tokio::test]
    async fn compensation_is_atomic_with_the_losing_event_and_skipped_on_redelivery() {
        let store = SqliteEventStore::open_in_memory().unwrap();
        let from = peer(0xD4);
        let losing = [shipped(id(from, 5), 50, event("Reserved", "seat-1"))];
        let compensation = [event("ReservationCompensated", "seat-1")];

        let first = ingest(&store, &[IngestGroup::new(&losing, &compensation)])
            .await
            .unwrap();
        assert_eq!((first.appended, first.compensated), (1, 1));

        let log_after = log(&store).await;
        assert_eq!(log_after.len(), 2);
        let lost = carrying(&log_after, losing[0].id);
        let answer = log_after
            .iter()
            .find(|held| held.event == compensation[0])
            .unwrap();
        assert!(answer.position > lost.position);
        assert_eq!(
            answer.id,
            EventId::new(store.store_id(), answer.position),
            "compensation is minted here, like any local append"
        );

        let again = ingest(&store, &[IngestGroup::new(&losing, &compensation)])
            .await
            .unwrap();
        assert_eq!(
            (again.appended, again.skipped, again.compensated),
            (0, 1, 0)
        );
        assert_eq!(log(&store).await.len(), 2, "compensation written twice");

        // Over a ceiling: refused as capacity, and neither half is written.
        let never_fits = [Event::new(
            "ReservationCompensated",
            vec![0u8; SqliteEventStore::MAX_EVENT_DATA_LEN + 1],
        )
        .unwrap()];
        let doomed = [shipped(id(from, 6), 60, event("Reserved", "seat-2"))];
        let refused = ingest(&store, &[IngestGroup::new(&doomed, &never_fits)]).await;
        assert!(matches!(
            refused,
            Err(IngestError::ExceedsStoreLimit {
                limit: StoreLimit::EventDataLen,
                ..
            })
        ));
        assert!(!store.contains_event_id(doomed[0].id).await.unwrap());
    }

    /// SY-2, inside the transaction: the losing event has been inserted and
    /// the compensation insert then fails, and the losing event goes with it.
    ///
    /// The fault is injected by a trigger another connection installs, because
    /// nothing a caller can put in an `Event` fails once the ceilings have
    /// passed. What this rejects is a writer that commits per group half — the
    /// foreign rows, then the compensation — which every other test here would
    /// pass.
    #[tokio::test]
    async fn a_compensation_that_fails_in_the_transaction_takes_the_losing_event_with_it() {
        let file = TempFile::new();
        let store = SqliteEventStore::open(&file.0).unwrap();
        file.inspect()
            .execute_batch(
                "CREATE TRIGGER poison BEFORE INSERT ON event \
                 WHEN NEW.event_type = 'Poisoned' \
                 BEGIN SELECT RAISE(ABORT, 'poisoned compensation'); END;",
            )
            .unwrap();

        let losing = [shipped(id(peer(0xE5), 1), 1, event("Reserved", "seat"))];
        let poisoned = [event("Poisoned", "seat")];
        let failed = ingest(&store, &[IngestGroup::new(&losing, &poisoned)]).await;

        assert!(matches!(
            failed,
            Err(IngestError::Store(SqliteEventStoreError::Sqlite(_)))
        ));
        assert!(
            !store.contains_event_id(losing[0].id).await.unwrap(),
            "the losing event committed without its compensation"
        );
        assert_eq!(store.head().await.unwrap(), None);
    }

    /// A local append after an ingest stamps its own rows and leaves the
    /// foreign ones as they came.
    ///
    /// Not evidence for the `origin_position IS NULL` marker, and not cited as
    /// such: the append's rows all sit above every foreign row, so the stamp's
    /// positional bound excludes the foreign rows on its own, and this passes
    /// with the marker deleted. The marker's test is
    /// `event_store::tests::a_mixed_batch_does_not_restamp_its_foreign_rows`,
    /// which builds the one batch shape it guards.
    #[tokio::test]
    async fn a_local_append_after_ingest_does_not_restamp_foreign_rows() {
        let store = SqliteEventStore::open_in_memory().unwrap();
        let from = peer(0xF6);
        let batch = [
            shipped(id(from, 4), 4, event("Shipped", "a")),
            shipped(id(from, 8), 8, event("Shipped", "b")),
        ];
        ingest(&store, &[IngestGroup::new(&batch, &[])])
            .await
            .unwrap();
        store
            .append(&[event("Local", "a"), event("Local", "b")], None)
            .await
            .unwrap();

        let log = log(&store).await;
        for sent in &batch {
            assert_eq!(carrying(&log, sent.id).id, sent.id);
        }
        let local: Vec<&SequencedEvent> = log
            .iter()
            .filter(|held| held.id.store() == store.store_id())
            .collect();
        assert_eq!(local.len(), 2);
        for held in local {
            assert_eq!(held.id.position(), held.position);
        }
    }

    /// An origin position past `i64::MAX` is refused, and the ingest it came in
    /// is rolled back whole.
    ///
    /// Rejects a writer that saturates the conversion, as the query-bound
    /// helper `as_i64` does: both events below would be written at `i64::MAX`,
    /// the second would meet the first under the origin pair's `UNIQUE`, and the
    /// ingest would report `skipped == 1` with no error anywhere.
    #[tokio::test]
    async fn an_origin_position_past_i64_is_refused_not_skipped() {
        let store = SqliteEventStore::open_in_memory().unwrap();
        let from = peer(0xE7);
        let batch = [
            shipped(id(from, 1), 1, event("Shipped", "a")),
            shipped(id(from, u64::MAX - 1), 2, event("Shipped", "b")),
            shipped(id(from, u64::MAX), 3, event("Shipped", "c")),
        ];

        let refused = ingest(&store, &[IngestGroup::new(&batch, &[])]).await;
        match refused {
            Err(IngestError::Store(SqliteEventStoreError::OriginPositionOutOfRange {
                position,
            })) => assert_eq!(position.get(), u64::MAX - 1),
            other => panic!("expected OriginPositionOutOfRange, got {other:?}"),
        }
        assert!(
            log(&store).await.is_empty(),
            "the representable event before it was not rolled back"
        );
    }

    /// `EventStore::contains_event_id` answers for a foreign identity, and
    /// does not also answer for a local identity at the row's local position:
    /// a row carries one identity.
    #[tokio::test]
    async fn contains_event_id_answers_for_a_foreign_id() {
        let store = SqliteEventStore::open_in_memory().unwrap();
        let from = peer(0x17);
        let sent = id(from, 42);
        assert!(!store.contains_event_id(sent).await.unwrap());

        let batch = [shipped(sent, 1, event("Shipped", "a"))];
        ingest(&store, &[IngestGroup::new(&batch, &[])])
            .await
            .unwrap();

        assert!(store.contains_event_id(sent).await.unwrap());
        assert!(!store.contains_event_id(id(peer(0x18), 42)).await.unwrap());
        let landed = carrying(&log(&store).await, sent).position;
        assert!(
            !store
                .contains_event_id(EventId::new(store.store_id(), landed))
                .await
                .unwrap()
        );
    }

    /// SY-1: an ingest takes no condition and is refused by none — including
    /// one every append would violate right now. A conditional append after it
    /// sees the ingested event as the conflict.
    #[tokio::test]
    async fn ingest_ignores_append_conditions() {
        let store = SqliteEventStore::open_in_memory().unwrap();
        let seat = Query::from_item(
            QueryItem::tagged(Tags::from_pairs([("k", "seat-9")]).unwrap()).unwrap(),
        );
        let decided_at = store
            .append(&[event("Reserved", "seat-9")], None)
            .await
            .unwrap();

        // Violated already: the seat is taken, and no append under it lands.
        let taken = AppendCondition::new(seat.clone());
        assert!(
            store
                .append(&[event("Reserved", "seat-9")], Some(&taken))
                .await
                .unwrap_err()
                .is_condition_violated()
        );

        let rival = [shipped(id(peer(0x29), 3), 3, event("Reserved", "seat-9"))];
        let ingested = ingest(&store, &[IngestGroup::new(&rival, &[])])
            .await
            .unwrap();
        assert_eq!(ingested.appended, 1);

        let landed = carrying(&log(&store).await, rival[0].id).position;
        let since_decision = AppendCondition::new(seat).after_every_guard(decided_at);
        match store
            .append(&[event("Reserved", "seat-9")], Some(&since_decision))
            .await
        {
            Err(AppendError::ConditionViolated(violated)) => {
                assert_eq!(violated.conflicting_position, Some(landed));
            }
            other => panic!("expected the ingested event as the conflict, got {other:?}"),
        }
    }

    /// The watermark is the highest origin position held per origin store, this
    /// store's own included.
    ///
    /// The oracle is the log read back and folded in Rust, which shares nothing
    /// with the `GROUP BY` under test, and the two foreign marks are also held
    /// against the maxima of what was sent.
    #[tokio::test]
    async fn watermark_is_max_origin_position_per_origin() {
        let store = SqliteEventStore::open_in_memory().unwrap();
        assert!(ingest_watermark(&store).await.is_empty());

        let (one, two) = (peer(0x31), peer(0x32));
        let batch = [
            shipped(id(one, 2), 2, event("Shipped", "a")),
            shipped(id(one, 9), 9, event("Shipped", "b")),
            shipped(id(one, 5), 5, event("Shipped", "c")),
            shipped(id(two, 4), 4, event("Shipped", "d")),
        ];
        ingest(
            &store,
            &[
                IngestGroup::new(&batch[..2], &[]),
                IngestGroup::new(&batch[2..], &[]),
            ],
        )
        .await
        .unwrap();
        store
            .append(&[event("Local", "a"), event("Local", "b")], None)
            .await
            .unwrap();

        let mut expected = Watermark::new();
        for held in log(&store).await {
            expected.advance(held.id.store(), held.id.position());
        }
        let watermark = ingest_watermark(&store).await;
        assert_eq!(watermark, expected);
        assert_eq!(watermark.len(), 3);

        for from in [one, two] {
            let sent = batch
                .iter()
                .filter(|shipped| shipped.id.store() == from)
                .map(|shipped| shipped.id.position())
                .max();
            assert_eq!(watermark.get(from), sent);
        }
    }

    /// The watermark through the port, bound on the weaker flavour.
    async fn ingest_watermark<S: IngestStore>(store: &S) -> Watermark
    where
        S::Error: core::fmt::Debug,
    {
        store.watermark().await.unwrap()
    }

    /// An own-identity event this store **does** hold is an ordinary
    /// re-delivery: a skip, decided by the insert like any other.
    #[tokio::test]
    async fn an_echo_of_an_own_event_is_a_skip() {
        let store = SqliteEventStore::open_in_memory().unwrap();
        let held = store.append(&[event("Local", "a")], None).await.unwrap();

        let back_home = [shipped(
            EventId::new(store.store_id(), held),
            1,
            event("Local", "a"),
        )];
        let skipped = ingest(&store, &[IngestGroup::new(&back_home, &[])])
            .await
            .expect("an echo of a held event is not an error");
        assert_eq!((skipped.appended, skipped.skipped), (0, 1));
        assert_eq!(store.head().await.unwrap(), Some(held));
    }

    /// The arrangement every unheld-own-identity test starts from.
    ///
    /// One local event is held. Then a call of two groups, the second of which
    /// carries, at index 1 rather than 0, an event claiming this store's own
    /// identity three positions past its head: the VT-6 breach of a store
    /// restored or rewound under an identity it should have re-minted. The
    /// claim sits neither first in its group nor in the first group, so a store
    /// that inspects only the head of a batch, or writes group by group and
    /// commits between them, fails the tests below.
    struct Claimed {
        /// The identity the forged event claims, which this store minted and
        /// does not hold.
        claim: EventId,
        /// The first group's foreign events, all new.
        first: [ReplicatedEvent; 1],
        /// The second group's: one new foreign event, then the claim.
        second: [ReplicatedEvent; 2],
    }

    impl Claimed {
        async fn on(store: &SqliteEventStore) -> Self {
            let held = store.append(&[event("Local", "a")], None).await.unwrap();
            // Computed from the position the store assigned, never a literal:
            // past the positions this call's own rows would take, and within
            // reach of the appends that follow.
            let claimed = held
                .next()
                .and_then(SequencePosition::next)
                .and_then(SequencePosition::next)
                .expect("three positions past a fresh head exist");
            let claim = EventId::new(store.store_id(), claimed);
            let from = peer(0x4A);
            Self {
                claim,
                first: [shipped(id(from, 1), 1, event("Shipped", "f1"))],
                second: [
                    shipped(id(from, 2), 2, event("Shipped", "f2")),
                    shipped(claim, 3, event("Local", "forged")),
                ],
            }
        }

        fn groups(&self) -> [IngestGroup<'_>; 2] {
            [
                IngestGroup::new(&self.first, &[]),
                IngestGroup::new(&self.second, &[]),
            ]
        }
    }

    /// SY-38: an event claiming this store's identity at a position it does
    /// not hold refuses the **whole** call as `UnheldOwnIdentity`, class
    /// `Stop`, and writes nothing — not the claim, not the new foreign event
    /// beside it, not the group before it.
    ///
    /// Replaces the pin that recorded the policy before it: the claim was
    /// ingested, and the local append that later reached its position wedged.
    #[tokio::test]
    async fn an_unheld_own_identity_stops_the_ingest_and_writes_nothing() {
        let store = SqliteEventStore::open_in_memory().unwrap();
        let claimed = Claimed::on(&store).await;
        let head = store.head().await.unwrap();
        let watermark = ingest_watermark(&store).await;

        let refused = ingest(&store, &claimed.groups())
            .await
            .expect_err("an unheld own identity must refuse the ingest");
        assert!(
            matches!(refused, IngestError::UnheldOwnIdentity { id, .. } if id == claimed.claim),
            "expected UnheldOwnIdentity naming {}, got {refused:?}",
            claimed.claim
        );
        assert_eq!(refused.class(), FailureClass::Stop);

        for sent in [&claimed.first[0], &claimed.second[0], &claimed.second[1]] {
            assert!(
                !store.contains_event_id(sent.id).await.unwrap(),
                "{} was written by a refused ingest",
                sent.id
            );
        }
        assert_eq!(store.head().await.unwrap(), head);
        assert_eq!(ingest_watermark(&store).await, watermark);
    }

    /// The refusal leaves local appends working. Under the old policy the
    /// claim was written, and the append assigned its position collided on the
    /// origin pair's `UNIQUE` and rolled back, so every retry collided again.
    ///
    /// That one of the five appends lands at the claimed position is this
    /// adapter's fact rather than the contract's: SQLite assigns positions
    /// here without gaps, which is what made the old wedge reachable at all.
    #[tokio::test]
    async fn local_appends_are_not_wedged_after_a_refused_claim() {
        let store = SqliteEventStore::open_in_memory().unwrap();
        let claimed = Claimed::on(&store).await;
        ingest(&store, &claimed.groups())
            .await
            .expect_err("the claim is refused");

        let mut assigned = Vec::new();
        for _ in 0..5 {
            assigned.push(
                store
                    .append(&[event("Local", "next")], None)
                    .await
                    .expect("a local append after a refused claim lands"),
            );
        }
        assert!(
            assigned.contains(&claimed.claim.position()),
            "no append reached the claimed position, so this proves nothing \
             about the wedge: {assigned:?}"
        );
        assert!(
            store.contains_event_id(claimed.claim).await.unwrap(),
            "the local event at the claimed position carries this store's identity"
        );
    }

    /// The remedy SY-38 names converges. After the operator re-mints the
    /// store's identity, the same batch lands whole, and the claim is held as
    /// the foreign event it now is, under the identity that minted it.
    #[tokio::test]
    async fn a_claim_lands_as_foreign_after_the_store_is_reminted() {
        let file = TempFile::new();
        let store = SqliteEventStore::open(&file.0).unwrap();
        let claimed = Claimed::on(&store).await;
        ingest(&store, &claimed.groups())
            .await
            .expect_err("before the re-mint the claim is refused");
        drop(store);

        let reminted = SqliteEventStore::remint_identity(&file.0).unwrap();
        let store = SqliteEventStore::open(&file.0).unwrap();
        assert_eq!(store.store_id(), reminted);
        assert_ne!(store.store_id(), claimed.claim.store());

        let landed = ingest(&store, &claimed.groups())
            .await
            .expect("after the re-mint the claim is a foreign event");
        assert_eq!((landed.appended, landed.skipped), (3, 0));
        assert!(store.contains_event_id(claimed.claim).await.unwrap());
        let log = log(&store).await;
        let held = carrying(&log, claimed.claim);
        assert_eq!(held.id.store(), claimed.claim.store());
        assert_ne!(held.id.store(), store.store_id());
    }

    /// The whole call rolls back, so the compensation is absent too: the
    /// refusing group's and the one written for the group before it.
    #[tokio::test]
    async fn a_compensation_whose_group_carries_an_unheld_own_identity_is_not_written() {
        let store = SqliteEventStore::open_in_memory().unwrap();
        let claimed = Claimed::on(&store).await;
        let before = log(&store).await;
        let answers = [[event("Compensated", "one")], [event("Compensated", "two")]];

        let refused = ingest(
            &store,
            &[
                IngestGroup::new(&claimed.first, &answers[0]),
                IngestGroup::new(&claimed.second, &answers[1]),
            ],
        )
        .await;
        assert!(
            matches!(refused, Err(IngestError::UnheldOwnIdentity { .. })),
            "{refused:?}"
        );
        assert_eq!(
            log(&store).await,
            before,
            "a refused ingest left compensation, or a foreign event, behind"
        );
    }

    /// A claim this call's **own** compensation would hide is refused too.
    ///
    /// Group 1 lands a new foreign event and its compensation, which this
    /// store stamps with its own identity at the next position but one. Group
    /// 2 carries the peer's event claiming exactly that identity. The insert
    /// meets the compensation under the `UNIQUE` constraint and reads as a
    /// re-delivery, so a store that refuses only the claims that land would
    /// commit two events under one `EventId`. The store did not hold that
    /// identity when the call began, and that is what decides.
    #[tokio::test]
    async fn a_claim_the_calls_own_compensation_would_hide_is_refused() {
        let store = SqliteEventStore::open_in_memory().unwrap();
        let held = store.append(&[event("Local", "a")], None).await.unwrap();
        let before = log(&store).await;
        // The foreign event takes the next position and its compensation the
        // one after: computed from the head the store assigned, never a literal.
        let compensated_at = held
            .next()
            .and_then(SequencePosition::next)
            .expect("two positions past a fresh head exist");
        let claim = EventId::new(store.store_id(), compensated_at);
        let from = peer(0x5B);
        let first = [shipped(id(from, 1), 1, event("Shipped", "f1"))];
        let answer = [event("Compensated", "c1")];
        let second = [
            shipped(id(from, 2), 2, event("Shipped", "f2")),
            shipped(claim, 3, event("Local", "forged")),
        ];

        let refused = ingest(
            &store,
            &[
                IngestGroup::new(&first, &answer),
                IngestGroup::new(&second, &[]),
            ],
        )
        .await;
        assert!(
            matches!(refused, Err(IngestError::UnheldOwnIdentity { id, .. }) if id == claim),
            "expected UnheldOwnIdentity naming {claim}, got {refused:?}"
        );
        assert_eq!(
            log(&store).await,
            before,
            "a refused ingest left its compensation or a foreign event behind"
        );
    }

    /// D6's table, at the boundaries a runner acts on.
    #[test]
    fn sqlite_errors_state_their_class() {
        let driver = |code| {
            SqliteEventStoreError::Sqlite(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(code),
                None,
            ))
        };
        let cases = [
            (driver(rusqlite::ffi::SQLITE_BUSY), FailureClass::Transient),
            (
                driver(rusqlite::ffi::SQLITE_LOCKED),
                FailureClass::Transient,
            ),
            (driver(rusqlite::ffi::SQLITE_FULL), FailureClass::Transient),
            (driver(rusqlite::ffi::SQLITE_IOERR), FailureClass::Transient),
            (driver(rusqlite::ffi::SQLITE_NOMEM), FailureClass::Transient),
            (driver(rusqlite::ffi::SQLITE_CONSTRAINT), FailureClass::Stop),
            (driver(rusqlite::ffi::SQLITE_CORRUPT), FailureClass::Stop),
            (
                SqliteEventStoreError::OriginPositionOutOfRange {
                    position: SequencePosition::FIRST,
                },
                FailureClass::Park,
            ),
            (
                SqliteEventStoreError::IdentityMoved {
                    handle: peer(1),
                    persisted: peer(2),
                },
                FailureClass::Stop,
            ),
            (
                SqliteEventStoreError::ConnectionPoisoned,
                FailureClass::Stop,
            ),
        ];
        for (error, class) in cases {
            assert_eq!(error.class(), class, "{error}");
        }
    }

    /// `happenstance-sync-testkit`'s ingest family, against this adapter.
    ///
    /// Mounted here rather than in `tests/` for the reason the module docs
    /// give: the impl is `cfg(test)`, and a `cfg(test)` impl is invisible to an
    /// integration test. The unit tests above pin the spike's own behaviour;
    /// this is the bar every adapter that implements the port clears.
    mod ingest_family {
        use happenstance_sync_testkit::{IngestFixture, Violation};
        use happenstance_testkit::Capability;

        use super::TempFile;
        use crate::event_store::SqliteEventStore;

        /// One file per fixture, and a connection onto it per `connect`.
        ///
        /// A file, not `open_in_memory`, so that two handles a rule holds are
        /// one store. Each handle is dropped before the fixture that backs it,
        /// so the file unlinks on Windows too.
        struct SqliteIngestFixture {
            file: TempFile,
        }

        impl SqliteIngestFixture {
            fn new() -> Self {
                Self {
                    file: TempFile::new(),
                }
            }
        }

        impl IngestFixture for SqliteIngestFixture {
            type Store = SqliteEventStore;

            /// Supported, through an `AFTER INSERT` trigger another connection
            /// installs: the mechanism this module's
            /// `a_compensation_that_fails_in_the_transaction_takes_the_losing_event_with_it`
            /// already uses.
            ///
            /// The event-store fixture in `tests/support/mod.rs` declines
            /// `MID_BATCH_FAULT` by scope, because arming it would retire
            /// ES-35's residual falsifier. That scoping does not reach here:
            /// this is a different fixture, a different capability and a
            /// different port, and ES-35 is about `EventStore` durability.
            const MID_GROUP_FAULT: Capability = Capability::SUPPORTED;

            async fn connect(&self) -> Self::Store {
                SqliteEventStore::open(&self.file.0)
                    .expect("a fresh temporary file opens as a store")
            }

            /// Installs an `AFTER INSERT` trigger that raises once the
            /// `event` table holds more than `after` rows past what it holds
            /// now, so the `after + 1`-th row the next ingest inserts aborts
            /// its transaction. The trigger counts only rows actually
            /// inserted, so a row the store skips is not counted, and it stays
            /// for the fixture's life, which the arm's contract allows.
            async fn arm_mid_group_fault(&self, after: usize) -> Result<(), Violation> {
                let unarmable = |error: rusqlite::Error| {
                    Violation::new(
                        "SY-30",
                        format!("the SQLite fixture could not arm its trigger: {error}"),
                    )
                };
                let connection = self.file.inspect();
                let held: i64 = connection
                    .query_row("SELECT count(*) FROM event", [], |row| row.get(0))
                    .map_err(unarmable)?;
                // Integers only are formatted into the statement, so there is
                // nothing to inject.
                let ceiling = i64::try_from(after)
                    .ok()
                    .and_then(|after| held.checked_add(after))
                    .ok_or_else(|| {
                        Violation::new("SY-30", format!("a cut at row {after} overflows an i64"))
                    })?;
                connection
                    .execute_batch(&format!(
                        "CREATE TRIGGER mid_group_fault AFTER INSERT ON event \
                         WHEN (SELECT count(*) FROM event) > {ceiling} \
                         BEGIN SELECT RAISE(ABORT, 'the fixture armed a mid-group fault'); END;"
                    ))
                    .map_err(unarmable)
            }
        }

        happenstance_sync_testkit::ingest_store_conformance!(
            mod_name = ingest_conformance,
            fixture = super::SqliteIngestFixture::new()
        );
    }
}
