//! SY-38 against a live server: a claim on this store's own identity that it
//! did not hold when the call began refuses the whole call.
//!
//! The family's rule verifies the hidden-claim half only where the store puts
//! a compensation two positions past the head, and on this store a refused
//! call has consumed sequence values first, so the rule reports that half as
//! skipped. `a_claim_the_calls_own_compensation_would_hide_is_refused` and
//! `a_claim_on_a_row_committed_during_the_call_is_refused` verify it here
//! instead, on arrangements this store's allocation is shown to produce.

use std::time::Duration;

use happenstance_core::{EventId, SequencePosition, StoreId};
use happenstance_sync::{Classify, FailureClass, IngestError, IngestGroup, ReplicatedEvent};
use sqlx::PgPool;

use super::super::PostgresIngestStore;
use super::{
    append, carrying, event, holds, id, ingest, live_store, log, peer, rows, shipped, watermark,
};
use crate::event_store::PostgresEventStore;

/// The position `count` past `position`.
fn past(position: SequencePosition, count: usize) -> SequencePosition {
    (0..count).fold(position, |position, _| {
        position
            .next()
            .expect("test positions are far from the top")
    })
}

/// The arrangement the unheld-claim tests start from.
///
/// One local event is held. Then a call of two groups, the second of which
/// carries, at index 1, an event claiming this store's own identity `ahead`
/// positions past the head the append returned. The claim is neither first in
/// its group nor in the first group, so a store that inspects only the head
/// of a batch, or commits group by group, fails the tests below.
struct Claimed {
    /// The identity the forged event claims.
    claim: EventId,
    /// The first group's foreign events, all new.
    first: [ReplicatedEvent; 1],
    /// The second group's: one new foreign event, then the claim.
    second: [ReplicatedEvent; 2],
}

impl Claimed {
    async fn on(store: &PostgresIngestStore, ahead: usize) -> Self {
        let held = append(store, &[event("Local", "a")]).await;
        let claim = EventId::new(store.store_id, past(held, ahead));
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

    /// Every event the call carries, the claim last.
    fn sent(&self) -> [EventId; 3] {
        [self.first[0].id, self.second[0].id, self.claim]
    }
}

/// Asserts `refused` is the unheld-identity refusal naming `claim`, class Stop.
fn assert_refused<E: Classify + core::fmt::Debug>(
    refused: Result<happenstance_sync::Ingested, IngestError<E>>,
    claim: EventId,
) {
    let refused = refused.expect_err("an unheld own identity refuses the ingest");
    assert!(
        matches!(refused, IngestError::UnheldOwnIdentity { id, .. } if id == claim),
        "expected UnheldOwnIdentity naming {claim}, got {refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Stop);
}

/// An own-identity event this store holds is an ordinary re-delivery: a skip,
/// decided by the insert. Rejects refusing every own identity.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn an_echo_of_an_own_event_is_a_skip() {
    let store = live_store().await;
    let held = append(&store, &[event("Local", "a")]).await;
    let before = log(&store).await;

    let back_home = [shipped(
        EventId::new(store.store_id, held),
        1,
        event("Local", "a"),
    )];
    let skipped = ingest(&store, &[IngestGroup::new(&back_home, &[])])
        .await
        .expect("an echo of a held event is not an error");
    assert_eq!((skipped.appended, skipped.skipped), (0, 1));
    assert_eq!(log(&store).await, before);
    assert_eq!(rows(&store).await, 1);
}

/// SY-38: the claim refuses the whole call, and nothing is written: not the
/// claim, not the new foreign event beside it, not the group before it. The
/// store still writes afterwards, and its watermark has not moved.
///
/// Rejects dropping the claim as a skip, checking only the head of the batch,
/// and committing group by group.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn an_unheld_own_identity_stops_the_ingest_and_writes_nothing() {
    let store = live_store().await;
    let claimed = Claimed::on(&store, 3).await;
    let marks = watermark(&store).await;

    assert_refused(ingest(&store, &claimed.groups()).await, claimed.claim);

    for sent in claimed.sent() {
        assert!(
            !holds(&store, sent).await,
            "{sent} was written by a refused ingest"
        );
    }
    assert_eq!(rows(&store).await, 1);
    assert_eq!(watermark(&store).await, marks);
    let after = append(&store, &[event("Local", "b")]).await;
    assert!(
        holds(&store, EventId::new(store.store_id, after)).await,
        "the store still writes"
    );
}

/// The whole call rolls back, so the compensation goes too: the refusing
/// group's, and the one written for the group before it.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_compensation_whose_group_carries_an_unheld_own_identity_is_not_written() {
    let store = live_store().await;
    let claimed = Claimed::on(&store, 3).await;
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
    assert_refused(refused, claimed.claim);
    assert_eq!(log(&store).await, before);
    assert_eq!(rows(&store).await, 1, "a refused ingest left rows behind");
}

/// A claim this call's own compensation would hide is refused too.
///
/// Group 1 lands a foreign event and its compensation; group 2 claims the
/// identity that compensation takes. The insert meets it under the origin
/// index and reads as a re-delivery, so a store that refuses only the claims
/// that land would commit two events under one `EventId`.
///
/// A twin store first shows the arrangement is real here: on the first call
/// after an append, this store's compensation takes the identity two
/// positions past the head. Rejects a store that refuses only landed claims.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_claim_the_calls_own_compensation_would_hide_is_refused() {
    let from = peer(0x5B);
    let first = [shipped(id(from, 1), 1, event("Shipped", "f1"))];
    let answer = [event("Compensated", "c1")];

    let twin = live_store().await;
    let twin_head = append(&twin, &[event("Local", "a")]).await;
    ingest(&twin, &[IngestGroup::new(&first, &answer)])
        .await
        .expect("a new foreign event and its compensation land");
    let compensated = log(&twin)
        .await
        .into_iter()
        .find(|held| held.event == answer[0])
        .expect("the compensation is held");
    assert_eq!(
        compensated.id,
        EventId::new(twin.store_id, past(twin_head, 2))
    );

    let store = live_store().await;
    let head = append(&store, &[event("Local", "a")]).await;
    let before = log(&store).await;
    let claim = EventId::new(store.store_id, past(head, 2));
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
    assert_refused(refused, claim);
    assert_eq!(log(&store).await, before);
    assert_eq!(rows(&store).await, 1);
}

/// A claim on a row another transaction committed **during** the call is
/// refused: this store's own axis, because positions are allocated outside
/// the transaction and commits do not follow them.
///
/// A local-shaped row is inserted in an open transaction T, at a position
/// allocated before the ingest begins. The ingest takes its begin snapshot
/// while T is open, then blocks in a trigger on an advisory lock. T commits,
/// the lock is released, and the claim meets T's row. Rejects a boundary on
/// positions, under which T's row sits below the call's first position and is
/// taken as held, and any design where "skipped" means "held".
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_claim_on_a_row_committed_during_the_call_is_refused() {
    let store = live_store().await;
    let pool = store.store.pool();
    let own = store.store_id;
    // Installed first: `CREATE TRIGGER` waits on T's row lock otherwise.
    sqlx::raw_sql(
        "CREATE FUNCTION gate() RETURNS trigger LANGUAGE plpgsql AS \
         $$ BEGIN PERFORM pg_advisory_xact_lock_shared(108); RETURN NEW; END $$; \
         CREATE TRIGGER gate BEFORE INSERT ON event FOR EACH ROW \
         WHEN (NEW.event_type = 'Gate') EXECUTE FUNCTION gate();",
    )
    .execute(pool)
    .await
    .expect("the gate installs");

    let mut writer = pool.begin().await.expect("T opens");
    let held: i64 = sqlx::query_scalar(
        "INSERT INTO event (position, event_type, data, metadata, tags, origin_store, \
         origin_position, recorded_at) VALUES (nextval('event_position_seq'), 'Local', \
         '\\x00', NULL, '{}', $1, currval('event_position_seq'), 0) RETURNING position",
    )
    .bind(own.to_bytes().to_vec())
    .fetch_one(&mut *writer)
    .await
    .expect("T writes a local-shaped row");
    let claim = id(
        own,
        u64::try_from(held).expect("a sequence value is positive"),
    );

    let mut gate = pool.acquire().await.expect("G connects");
    sqlx::query("SELECT pg_advisory_lock(108)")
        .execute(&mut *gate)
        .await
        .expect("G takes the gate");

    let ingesting = tokio::spawn({
        let store = store.clone();
        async move {
            let groups = [
                shipped(id(peer(0x6C), 1), 1, event("Gate", "g")),
                shipped(claim, 2, event("Local", "forged")),
            ];
            happenstance_sync::SendIngestStore::ingest(&store, &[IngestGroup::new(&groups, &[])])
                .await
        }
    });
    wait_for_a_blocked_gate(pool).await;

    writer.commit().await.expect("T commits during the call");
    sqlx::query("SELECT pg_advisory_unlock(108)")
        .execute(&mut *gate)
        .await
        .expect("G releases the gate");
    drop(gate);

    let refused = ingesting.await.expect("the ingest task finishes");
    assert_refused(refused, claim);
    assert!(
        !holds(&store, id(peer(0x6C), 1)).await,
        "the gated event was written"
    );
}

/// The ingest's backend, waiting on the gate: an ungranted advisory lock on
/// the `bigint` key 108 (`classid` 0, `objid` 108, `objsubid` 1), in this
/// database, held up in an `INSERT INTO event`. Narrowed so that another
/// client's advisory wait on the server cannot satisfy it.
const BLOCKED_GATE_SQL: &str = "SELECT count(*) FROM pg_locks AS l \
     JOIN pg_stat_activity AS a ON a.pid = l.pid \
     WHERE l.locktype = 'advisory' AND NOT l.granted \
     AND l.classid = 0 AND l.objid = 108 AND l.objsubid = 1 \
     AND l.database = (SELECT oid FROM pg_database WHERE datname = current_database()) \
     AND a.query LIKE 'INSERT INTO event%'";

/// How long the ingest may take to reach the gate before the arrangement is
/// declared wrong. Far above the few round trips it needs.
const GATE_DEADLINE: Duration = Duration::from_secs(30);

/// The pause between two polls, so the wait is bounded by time and does not
/// spin the server.
const GATE_POLL: Duration = Duration::from_millis(10);

/// Polls until the ingest is blocked in the gate.
///
/// # Panics
///
/// After [`GATE_DEADLINE`], because then the arrangement is wrong.
async fn wait_for_a_blocked_gate(pool: &PgPool) {
    let blocked = async {
        loop {
            let waiting: i64 = sqlx::query_scalar(BLOCKED_GATE_SQL)
                .fetch_one(pool)
                .await
                .expect("pg_locks and pg_stat_activity read");
            if waiting == 1 {
                return;
            }
            tokio::time::sleep(GATE_POLL).await;
        }
    };
    tokio::time::timeout(GATE_DEADLINE, blocked)
        .await
        .unwrap_or_else(|_| {
            panic!(
                "a broken arrangement: the ingest never blocked on the advisory gate within \
                 {GATE_DEADLINE:?}"
            )
        });
}

/// No permanent wedge after a refused claim, and a refused claim leaves no row
/// behind to collide with.
///
/// Postgres never had `happenstance-sqlite`'s wedge, because the sequence
/// never returns a value twice; what this pins is that the local append that
/// reaches the claimed position lands, carrying this store's identity. The
/// claim is eight past the head and the refused call consumed three, so the
/// fifth append reaches it.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn local_appends_are_not_wedged_after_a_refused_claim() {
    let store = live_store().await;
    let claimed = Claimed::on(&store, 8).await;
    assert_refused(ingest(&store, &claimed.groups()).await, claimed.claim);

    let mut assigned = Vec::new();
    for _ in 0..16 {
        let position = append(&store, &[event("Local", "next")]).await;
        assigned.push(position);
        if position >= claimed.claim.position() {
            break;
        }
    }
    assert!(
        assigned.contains(&claimed.claim.position()),
        "no append reached the claimed position, so this proves nothing: {assigned:?}"
    );
    assert!(holds(&store, claimed.claim).await);
}

/// The remedy SY-38 names converges, by the manual re-mint ADR-0098 says is
/// the only recovery here until #104 decides the procedure: after the
/// identity row is replaced and a fresh handle opened, the same groups land
/// whole, the claim as the foreign event it now is, and earlier local rows
/// keep the identity they were minted under.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_claim_lands_as_foreign_after_a_manual_remint() {
    let store = live_store().await;
    let claimed = Claimed::on(&store, 3).await;
    assert_refused(ingest(&store, &claimed.groups()).await, claimed.claim);
    let old: StoreId = store.store_id;

    let pool = store.store.pool().clone();
    sqlx::query("UPDATE store_meta SET v = uuid_send(gen_random_uuid()) WHERE k = 'store_id'")
        .execute(&pool)
        .await
        .expect("the identity row is replaced");
    let reminted = PostgresIngestStore::open(PostgresEventStore::new(pool))
        .await
        .expect("a fresh handle reads the new identity");
    assert_ne!(reminted.store_id, old);

    let landed = ingest(&reminted, &claimed.groups())
        .await
        .expect("after the re-mint the claim is a foreign event");
    assert_eq!((landed.appended, landed.skipped), (3, 0));
    let log = log(&reminted).await;
    assert_eq!(carrying(&log, claimed.claim).id.store(), old);
    let local = log
        .iter()
        .find(|held| held.event == event("Local", "a"))
        .expect("the arrangement's local event is held");
    assert_eq!(local.id.store(), old, "a re-mint restamped an earlier row");
}
