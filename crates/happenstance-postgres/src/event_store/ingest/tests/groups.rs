//! Write groups against a live server (#420): what the write-group family
//! cannot arrange from outside the crate.
//!
//! The columns read raw, so a test sees what was stored and not only what the
//! read made of it; two writers whose positions interleave, which the family's
//! rules, one handle at a time, never produce; and a schema an older build
//! left, with rows from before migration 3. No literal local position: every
//! one compared was read back from the store.
//!
//! Every test reads right after it writes, so it relies on a quiet server and
//! `--test-threads=1`, as the rest of this module does.

use core::num::NonZeroUsize;

use happenstance_core::{AppendCondition, Query, QueryItem, RecordedAt, SequencePosition};
use happenstance_sync::{GroupPage, IngestGroup, Waiting, WriteGroup, WriteGroupStore};
use sqlx::{PgPool, Row as _};

use super::{append, event, id, ingest, live_store, own, peer, server, shipped};
use crate::event_store::{NewRow, Origin, PostgresEventStore, insert_statement, write_group};
use crate::migration;

/// A page large enough for every arrangement here.
const WHOLE_LOG: NonZeroUsize = NonZeroUsize::MIN.saturating_add(63);

/// Every row's position, group and whether it holds a guard, in position
/// order, read raw.
async fn stored(pool: &PgPool) -> Vec<(i64, Option<i64>, bool)> {
    sqlx::query_as(
        "SELECT position, write_group, group_guard IS NOT NULL FROM event ORDER BY position",
    )
    .fetch_all(pool)
    .await
    .expect("the event table of a reachable schema reads")
}

/// One page through the port, bound on the bare flavour.
async fn read<S: WriteGroupStore>(
    store: &S,
    after: Option<SequencePosition>,
    max: NonZeroUsize,
) -> GroupPage
where
    S::Error: core::fmt::Debug,
{
    store
        .read_groups(after, max)
        .await
        .expect("a group read of a reachable store answers")
}

/// Each group's first and size.
fn shape(page: &GroupPage) -> Vec<(SequencePosition, usize)> {
    page.groups
        .iter()
        .map(|group| (group.first(), group.events().len()))
        .collect()
}

/// A position-free guard: evidence an origin's group might carry.
fn guard() -> AppendCondition {
    AppendCondition::new(Query::from_item(
        QueryItem::of_types(["Shipped"]).expect("a valid query item"),
    ))
}

/// I-2 on the append path: every row of one append records the batch's lowest
/// position, and none records a guard.
///
/// Rejects an append that leaves `write_group` `NULL`, which reads back as
/// groups of one, and one that names its group by its last position.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn an_appends_rows_record_its_lowest_position_as_their_group() {
    let store = live_store().await;
    append(
        &store,
        &[
            event("Grouped", "a"),
            event("Grouped", "b"),
            event("Grouped", "c"),
        ],
    )
    .await;

    let rows = stored(store.pool()).await;

    let lowest = rows.first().map(|&(position, _, _)| position);
    assert_eq!(rows.len(), 3);
    assert!(
        rows.iter()
            .all(|&(_, group, guarded)| group == lowest && !guarded),
        "every row of the append names its lowest position and no guard: {rows:?}"
    );
}

/// I-2 on the ingest path: a group whose first event is a re-delivery is named
/// by the first row it **landed**, which is not the first position it drew,
/// because the skipped copy burned one. Its compensation joins it, and the
/// guard sits on the group's first row only.
///
/// Rejects naming the group by the first position drawn: no row holds that
/// position, so ES-10 says nothing about it and a cursor resuming above it
/// could not be trusted.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_group_after_a_redelivery_takes_its_first_landed_row_and_its_compensation() {
    let store = live_store().await;
    let from = peer(0x42);
    let held = [shipped(id(from, 1), 10, event("Shipped", "held"))];
    let redelivered = [
        shipped(id(from, 1), 10, event("Shipped", "held")),
        shipped(id(from, 2), 20, event("Shipped", "new-1")),
        shipped(id(from, 3), 30, event("Shipped", "new-2")),
    ];
    let compensation = [event("Compensated", "new")];
    let guard = guard();
    ingest(&store, &[IngestGroup::new(&held, &[])])
        .await
        .expect("a new foreign event lands");

    let second = ingest(
        &store,
        &[IngestGroup::new(&redelivered, &compensation).with_guard(Some(&guard))],
    )
    .await
    .expect("a half-held group lands its new half");
    assert_eq!(
        (second.appended, second.skipped, second.compensated),
        (2, 1, 1)
    );

    let rows = stored(store.pool()).await;
    let [(held_at, held_group, false), new_rows @ ..] = rows.as_slice() else {
        panic!("the held event is the first row, unguarded: {rows:?}");
    };
    assert_eq!(
        *held_group,
        Some(*held_at),
        "the first ingest is its own group"
    );
    let [(first, ..), ..] = new_rows else {
        panic!("the second ingest wrote rows: {rows:?}");
    };
    assert_eq!(new_rows.len(), 3, "two foreign rows and the compensation");
    assert!(
        new_rows
            .iter()
            .all(|&(position, group, guarded)| group == Some(*first)
                && guarded == (position == *first)),
        "every row names the first landed row, which alone holds the guard: {rows:?}"
    );
    let burnt: i64 = sqlx::query_scalar("SELECT count(*) FROM event WHERE position = $1")
        .bind(first - 1)
        .fetch_one(store.pool())
        .await
        .expect("a count of a reachable schema answers");
    assert_eq!(burnt, 0, "the skipped copy's position holds no row");

    let page = read(&store, None, WHOLE_LOG).await;
    let last = page.groups.last().expect("two groups");
    assert_eq!(last.first().get(), u64::try_from(*first).expect("positive"));
    assert_eq!(last.guard(), Some(&guard));
    assert_eq!(last.events().len(), 3, "the compensation joins its group");
}

/// The interleaving case `first` is the cursor for (§3 of the plan, #420):
/// two transactions draw from the one sequence in turn, so their positions
/// alternate, G = {g, g+2, g+4} and H = {g+1, g+3, g+5}, and each writes its
/// rows through the store's own `INSERT` with its lowest position as its
/// group. They read back as two whole groups in `first` order; chained pages
/// of three events return each exactly once; and resuming above G's first
/// returns H whole, where resuming above G's last position would have lost
/// H's first row.
///
/// The interleaving is forced, not raced: the draws are issued one by one
/// across the two open transactions, which is the order two concurrent appends
/// can reach the sequence in and the one a scheduler rarely produces on cue.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn interleaved_writers_read_back_as_whole_groups_in_first_order() {
    let store = live_store().await;
    let own = own(&store).await;
    let pool = store.pool();
    let mut g = pool.begin().await.expect("a transaction opens");
    let mut h = pool.begin().await.expect("a transaction opens");
    let (mut g_at, mut h_at) = (Vec::new(), Vec::new());
    for _ in 0..3 {
        for (transaction, drawn) in [(&mut g, &mut g_at), (&mut h, &mut h_at)] {
            let position: i64 = sqlx::query_scalar("SELECT nextval('event_position_seq')")
                .fetch_one(&mut **transaction)
                .await
                .expect("the sequence answers");
            drawn.push(position);
        }
    }
    let g_events = [
        event("Grouped", "g1"),
        event("Grouped", "g2"),
        event("Grouped", "g3"),
    ];
    let h_events = [
        event("Grouped", "h1"),
        event("Grouped", "h2"),
        event("Grouped", "h3"),
    ];
    for (transaction, events, drawn) in [(&mut g, &g_events, &g_at), (&mut h, &h_events, &h_at)] {
        let rows = events.iter().map(|event| NewRow {
            event,
            origin: Origin::Local,
            recorded_at: RecordedAt::from_millis(1),
        });
        insert_statement(own, rows, drawn, drawn.iter().copied().min())
            .build()
            .execute(&mut **transaction)
            .await
            .expect("the store's own insert writes the rows");
    }
    h.commit().await.expect("the later group commits first");
    g.commit().await.expect("the earlier group commits");
    let position = |raw: &i64| SequencePosition::new(u64::try_from(*raw).expect("positive"));
    let g_first = g_at.first().and_then(position).expect("drawn");
    let h_first = h_at.first().and_then(position).expect("drawn");
    assert!(
        g_at.iter().zip(&h_at).all(|(g, h)| g < h) && h_at.first() < g_at.last(),
        "the draws interleave: G {g_at:?}, H {h_at:?}"
    );

    let whole = read(&store, None, WHOLE_LOG).await;
    assert_eq!(shape(&whole), [(g_first, 3), (h_first, 3)]);
    assert_eq!(whole.waiting, Waiting::Nothing);

    let three = NonZeroUsize::new(3).expect("non-zero");
    let first_page = read(&store, None, three).await;
    assert_eq!(shape(&first_page), [(g_first, 3)]);
    assert_eq!(first_page.waiting, Waiting::Unknown);
    let second_page = read(&store, Some(g_first), three).await;
    assert_eq!(shape(&second_page), [(h_first, 3)]);
    assert_eq!(second_page.waiting, Waiting::Nothing);
    let after_last = read(&store, Some(h_first), three).await;
    assert!(after_last.groups.is_empty());
    assert_eq!(after_last.waiting, Waiting::Nothing);
}

/// I-9 on a schema an older build migrated: rows written before migration 3
/// read as groups of one, and once `apply` has run, the next append reads as
/// one group.
///
/// The old rows are written with migration 1's columns only, as the build
/// before #420 wrote them.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn rows_from_before_migration_3_read_as_groups_of_one() {
    let pool = server::empty_pool().await;
    sqlx::raw_sql(migration::MIGRATION_1)
        .execute(&pool)
        .await
        .expect("migration 1 applies");
    sqlx::raw_sql(
        "INSERT INTO event (position, event_type, data, metadata, tags, origin_store, \
         origin_position, recorded_at) \
         SELECT p, 'Legacy', '\\x00'::bytea, NULL, '{}', \
         (SELECT v FROM store_meta WHERE k = 'store_id'), p, 1 \
         FROM (SELECT nextval('event_position_seq') AS p FROM generate_series(1, 3)) s",
    )
    .execute(&pool)
    .await
    .expect("an older build's three-row append");

    migration::apply(&pool)
        .await
        .expect("migration 3 applies over a populated schema");
    let store = PostgresEventStore::new(pool);
    let appended = append(
        &store,
        &[event("Grouped", "after-1"), event("Grouped", "after-2")],
    )
    .await;

    let page = read(&store, None, WHOLE_LOG).await;
    let sizes: Vec<usize> = page
        .groups
        .iter()
        .map(|group| group.events().len())
        .collect();
    assert_eq!(
        sizes,
        [1, 1, 1, 2],
        "three legacy singletons, then one group"
    );
    let guards: Vec<Option<&AppendCondition>> = page.groups.iter().map(WriteGroup::guard).collect();
    assert_eq!(guards, [None, None, None, None]);
    let last = page.groups.last().expect("four groups");
    assert!(
        last.first() < appended,
        "the group is named by its first row"
    );
    assert_eq!(page.waiting, Waiting::Nothing);
}

/// The group read finds a page's rows by key, never by scanning the table:
/// its plan, over 20,000 rows in groups of four with planner statistics,
/// names `event_write_group_idx` and holds no sequential scan. The text form
/// of the plan, read as lines, because this crate builds `sqlx` without its
/// JSON types.
///
/// **Named wrong statement:** the outer read written as
/// `write_group IN (…) OR (write_group IS NULL AND position IN (…))`, which
/// the planner can only run as a filter over every row (measured at 29.9 ms
/// on 200,000 rows, against 0.09 ms for the two keyed joins).
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn the_group_read_seeks_on_the_write_group_index() {
    let store = live_store().await;
    sqlx::raw_sql(
        "INSERT INTO event (position, event_type, data, metadata, tags, origin_store, \
         origin_position, recorded_at, write_group) \
         SELECT p, 'Bulk', '\\x00'::bytea, NULL, '{}', \
         (SELECT v FROM store_meta WHERE k = 'store_id'), p, 1, \
         min(p) OVER (PARTITION BY (n - 1) / 4) \
         FROM (SELECT n, nextval('event_position_seq') AS p \
         FROM generate_series(1, 20000) AS n) s; \
         ANALYZE event;",
    )
    .execute(store.pool())
    .await
    .expect("20,000 rows in groups of four, and their statistics");
    let middle: i64 =
        sqlx::query_scalar("SELECT write_group FROM event ORDER BY position OFFSET 10000 LIMIT 1")
            .fetch_one(store.pool())
            .await
            .expect("a group in the middle of the log");

    let sql = format!("EXPLAIN {}", write_group::groups_sql(store.visibility));
    let plan: Vec<String> =
        write_group::groups_query(&sql, middle, NonZeroUsize::new(3).expect("non-zero"))
            .fetch_all(store.pool())
            .await
            .expect("the group read explains")
            .iter()
            .map(|row| row.get::<String, _>(0))
            .collect();

    let plan = plan.join("\n");
    assert!(plan.contains("event_write_group_idx"), "{plan}");
    assert!(!plan.contains("Seq Scan"), "{plan}");
}
