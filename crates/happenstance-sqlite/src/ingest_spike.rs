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
//!   rather than saturating it;
//! * `ingest_locked`, beside `append_locked` — the same transaction and the
//!   same identity check, with no condition evaluated (SY-1), and compensation
//!   written only for a group that landed something (SY-11);
//! * `ingest_groups` and `origin_watermark`, the `&self` entry points: the
//!   ceilings and the lock, and one `GROUP BY` over the origin index.
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

use happenstance_core::{AppendError, StoreId};
use happenstance_sync::{IngestGroup, Ingested, Watermark};

use crate::event_store::{SqliteEventStore, SqliteEventStoreError};

// The `Send` flavour, because that is the flavour `SqliteEventStore` implements
// `EventStore` in — a `Mutex<Connection>` makes every future here `Send` — and
// `trait_variant`'s blanket impl hands generic code bound on `IngestStore` the
// other one for free, which the tests below rely on.
impl happenstance_sync::SendIngestStore for SqliteEventStore {
    // `AppendError` rather than the bare adapter error, so that a capacity
    // refusal arrives as `ExceedsStoreLimit` and a runner can tell "park this
    // batch, it will never fit here" from "retry". Its `NoEvents` and
    // `ConditionViolated` arms are never produced by an ingest. Whether the
    // port wants an ingest-specific error enum is phase 13's to settle.
    type Error = AppendError<SqliteEventStoreError>;

    fn store_id(&self) -> StoreId {
        // The inherent method, which path resolution prefers over this one.
        Self::store_id(self)
    }

    async fn ingest(&self, groups: &[IngestGroup<'_>]) -> Result<Ingested, Self::Error> {
        self.ingest_groups(groups)
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        self.origin_watermark().map_err(AppendError::Store)
    }
}

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
    use happenstance_sync::{IngestGroup, IngestStore, Ingested, ReplicatedEvent, Watermark};

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
    ) -> Result<Ingested, S::Error> {
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
            Err(AppendError::ExceedsStoreLimit {
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
            Err(AppendError::Store(SqliteEventStoreError::Sqlite(_)))
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
            Err(AppendError::Store(SqliteEventStoreError::OriginPositionOutOfRange {
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
        let since_decision = AppendCondition::new(seat).after(decided_at);
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

    /// **A pin, not an endorsement.** What happens today when an event claims
    /// this store's own `StoreId` at a position the store does not hold — the
    /// VT-6 breach, a store restored or rewound under an identity it should
    /// have re-minted. Phase 13 owns the policy; this records the behaviour
    /// that policy starts from, so that changing it is a visible decision.
    ///
    /// An own-id event the store *does* hold is an ordinary skip. One it does
    /// not hold is **ingested**: the insert sees no conflict, and refusing it
    /// would be a refusal on local state, which SY-1 forbids. The row takes the
    /// next local position and carries the claimed identity. The cost arrives
    /// later, at the local append whose assigned position equals the claim: its
    /// stamp collides on `UNIQUE (origin_store, origin_position)`, the append
    /// fails as a store error and rolls back — and because the rollback returns
    /// `AUTOINCREMENT` to where it was, **every retry is assigned the same
    /// position and collides again.** Local appends are wedged until something
    /// moves the sequence or the identity.
    #[tokio::test]
    async fn pinned_vt6_breach_an_unheld_own_id_is_ingested_and_wedges_the_append_that_reaches_it()
    {
        let store = SqliteEventStore::open_in_memory().unwrap();
        let own = store.store_id();
        let held = store.append(&[event("Local", "a")], None).await.unwrap();

        let back_home = [shipped(EventId::new(own, held), 1, event("Local", "a"))];
        let skipped = ingest(&store, &[IngestGroup::new(&back_home, &[])])
            .await
            .unwrap();
        assert_eq!((skipped.appended, skipped.skipped), (0, 1));

        // Three past the head this store assigned: past the position the
        // ingested row itself will take, and within reach of the appends below.
        let claim = id(own, held.get() + 3);
        let forged = [shipped(claim, 1, event("Local", "forged"))];
        let ingested = ingest(&store, &[IngestGroup::new(&forged, &[])])
            .await
            .unwrap();
        assert_eq!(ingested.appended, 1);
        assert!(store.contains_event_id(claim).await.unwrap());

        let mut collision = None;
        for _ in 0..4 {
            match store.append(&[event("Local", "next")], None).await {
                Ok(_) => {}
                Err(error) => {
                    collision = Some(error);
                    break;
                }
            }
        }
        let Some(AppendError::Store(SqliteEventStoreError::Sqlite(error))) = collision else {
            panic!("no local append collided with the claimed identity: {collision:?}");
        };
        assert_eq!(
            error.sqlite_error_code(),
            Some(rusqlite::ErrorCode::ConstraintViolation)
        );
        let stuck_at = store.head().await.unwrap().unwrap();
        assert!(
            stuck_at < claim.position(),
            "an append took the claimed position"
        );

        let retry = store.append(&[event("Local", "retry")], None).await;
        assert!(
            matches!(
                retry,
                Err(AppendError::Store(SqliteEventStoreError::Sqlite(_)))
            ),
            "the retry was assigned a new position: {retry:?}"
        );
        assert_eq!(store.head().await.unwrap(), Some(stuck_at));
    }
}
