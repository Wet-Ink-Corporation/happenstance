//! Write groups against the live endpoint (#420): what the write-group family
//! cannot see from outside the crate.
//!
//! That the provisional tags an ingest writes never survive its commit, read
//! raw; that a guard round-trips through a real `bytea` on the group's first
//! row only; and a schema an older build left, with rows from before migration
//! 3. No literal local position: every one compared was read back.
//!
//! Every test reads right after it writes, so it relies on a quiet endpoint and
//! `--test-threads=1`, as CI's `live-neon` job runs this module.

use core::num::NonZeroUsize;

use happenstance_core::{AppendCondition, Query, QueryItem};
use happenstance_sync::{GroupPage, IngestGroup, Waiting, WriteGroup, WriteGroupStore};

use super::endpoint::{empty_schema, execute, live_store};
use super::{append, event, id, ingest, peer, select, shipped};
use crate::event_store::NeonEventStore;
use crate::migration;
use crate::transport::{SqlRequest, SqlStatement, SqlTransport};

/// A page large enough for every arrangement here.
const WHOLE_LOG: NonZeroUsize = NonZeroUsize::MIN.saturating_add(63);

/// One page from the start, through the port.
async fn read<S: WriteGroupStore>(store: &S) -> GroupPage
where
    S::Error: core::fmt::Debug,
{
    store
        .read_groups(None, WHOLE_LOG)
        .await
        .expect("a group read of a reachable store answers")
}

/// Every row's position, group and whether it holds a guard, in position
/// order, read raw.
async fn stored<T: SqlTransport>(store: &NeonEventStore<T>) -> Vec<(i64, Option<i64>, bool)>
where
    T::Error: core::fmt::Debug,
{
    let table = store.config().qualified_event();
    let rows = select(
        store,
        format!(
            "SELECT position::text AS p, write_group::text AS g, \
             (group_guard IS NOT NULL)::text AS h FROM {table} ORDER BY {table}.position"
        ),
    )
    .await;
    rows.rows
        .iter()
        .map(|row| {
            let number = |column: &str| {
                row.get(column)
                    .and_then(serde_json::Value::as_str)
                    .and_then(|text| text.parse::<i64>().ok())
            };
            (
                number("p").expect("every row has a position"),
                number("g"),
                row.get("h").and_then(serde_json::Value::as_str) == Some("true"),
            )
        })
        .collect()
}

fn guard(kind: &str) -> AppendCondition {
    AppendCondition::new(Query::from_item(
        QueryItem::of_types([kind]).expect("a valid query item"),
    ))
}

/// Item 40: an ingest of three groups commits no negative `write_group`; each
/// group's rows, its compensation's included, name the lowest row it landed,
/// though the first group's first event is a re-delivery whose skipped copy
/// burned a position; and each guard reads back equal, from its group's first
/// row only.
///
/// Rejects a stamp sent outside the ingest's request, whose provisional tags
/// would commit, and a group named by its first drawn position.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn an_ingest_of_three_groups_commits_no_provisional_tag_and_keeps_its_guards() {
    let store = live_store().await;
    let from = peer(0x4E);
    let held = [shipped(id(from, 1), 10, event("Shipped", "held"))];
    ingest(&store, &[IngestGroup::new(&held, &[])])
        .await
        .expect("a new foreign event lands");
    let a = [
        shipped(id(from, 1), 10, event("Shipped", "held")),
        shipped(id(from, 2), 20, event("Shipped", "a2")),
    ];
    let b = [shipped(id(from, 3), 30, event("Shipped", "b1"))];
    let c = [
        shipped(id(from, 4), 40, event("Shipped", "c1")),
        shipped(id(from, 5), 50, event("Shipped", "c2")),
    ];
    let reversal = [event("Reversed", "c")];
    let (guard_a, guard_c) = (guard("ShippedA"), guard("ShippedC"));

    let ingested = ingest(
        &store,
        &[
            IngestGroup::new(&a, &[]).with_guard(Some(&guard_a)),
            IngestGroup::new(&b, &[]),
            IngestGroup::new(&c, &reversal).with_guard(Some(&guard_c)),
        ],
    )
    .await
    .expect("three groups land");
    assert_eq!(
        (ingested.appended, ingested.skipped, ingested.compensated),
        (4, 1, 1)
    );

    let table = store.config().qualified_event();
    let negative = select(
        &store,
        format!("SELECT count(*)::text AS n FROM {table} WHERE write_group < 0"),
    )
    .await;
    assert_eq!(
        negative.text(0, "n"),
        Some("0"),
        "a provisional tag committed"
    );

    let page = read(&store).await;
    let sizes: Vec<usize> = page
        .groups
        .iter()
        .map(|group| group.events().len())
        .collect();
    assert_eq!(
        sizes,
        [1, 1, 1, 3],
        "held, a's landed half, b, c with its reversal"
    );
    let guards: Vec<Option<&AppendCondition>> = page.groups.iter().map(WriteGroup::guard).collect();
    assert_eq!(guards, [None, Some(&guard_a), None, Some(&guard_c)]);
    let firsts: Vec<i64> = page
        .groups
        .iter()
        .map(|group| i64::try_from(group.first().get()).expect("a stored position"))
        .collect();
    let [_, a_first, _, c_first] = firsts[..] else {
        panic!("four groups: {firsts:?}");
    };

    let rows = stored(&store).await;
    let guarded: Vec<i64> = rows
        .iter()
        .filter(|&&(_, _, guarded)| guarded)
        .map(|&(position, _, _)| position)
        .collect();
    assert_eq!(
        guarded,
        [a_first, c_first],
        "a guard on each guarded group's first row only"
    );
    let named: Vec<Option<i64>> = rows.iter().map(|&(_, group, _)| group).collect();
    let expected: Vec<Option<i64>> = page
        .groups
        .iter()
        .zip(&firsts)
        .flat_map(|(group, &first)| core::iter::repeat_n(Some(first), group.events().len()))
        .collect();
    assert_eq!(
        named, expected,
        "every row, in position order, names its group's first landed row"
    );
    let burnt = select(
        &store,
        format!(
            "SELECT count(*)::text AS n FROM {table} WHERE position = {}",
            a_first - 1
        ),
    )
    .await;
    assert_eq!(
        burnt.text(0, "n"),
        Some("0"),
        "the re-delivered copy's position holds no row, so it cannot name the group"
    );
}

/// I-9 on a schema an older build migrated: rows written before migration 3
/// read as groups of one, and once migration 3 is applied the next append
/// reads as one group.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn rows_from_before_migration_3_read_as_groups_of_one() {
    let (config, transport) = empty_schema().await;
    execute(
        &transport,
        migration::request(migration::MIGRATION_1, &config),
        "migration 1",
    )
    .await;
    let table = config.qualified_event();
    let insert = format!(
        "INSERT INTO {table} (position, event_type, data, metadata, tags, origin_store, \
         origin_position, recorded_at) \
         SELECT p, 'Legacy', '\\x00'::bytea, NULL, '{{}}', \
         (SELECT v FROM {meta} WHERE k = 'store_id'), p, 1 \
         FROM (SELECT nextval({sequence}) AS p FROM generate_series(1, 3)) s",
        meta = config.qualified_meta(),
        sequence = config.sequence_literal(),
    );
    execute(
        &transport,
        SqlRequest::single(SqlStatement::new(insert)),
        "an older build's three-row append",
    )
    .await;
    execute(
        &transport,
        migration::request(migration::MIGRATION_3, &config),
        "migration 3 over a populated schema",
    )
    .await;
    let store = NeonEventStore::new(transport, config);
    let appended = append(
        &store,
        &[event("Grouped", "after-1"), event("Grouped", "after-2")],
    )
    .await;

    let page = read(&store).await;

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
