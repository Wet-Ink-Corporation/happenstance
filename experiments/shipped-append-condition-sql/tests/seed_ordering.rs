//! Does `tag_cardinality`'s most-selective-first ordering buy the shipped chain
//! anything?
//!
//! # The claim being tested
//!
//! `crates/happenstance-sqlite/src/event_store.rs:91-96`, in the crate's own
//! public module documentation:
//!
//! > **`tag_cardinality` is a requirement rather than a convenience.** Multi-tag
//! > items must be probed most-selective-tag-first…
//!
//! and `query_sql.rs:48-54`, which says the intersection chain *"keeps the
//! boundary pushable for the same reason and is what `tag_cardinality` exists to
//! order"*. ADR-0022 §8 (`references/adr/0022-append-condition-strategy.md:374-379`)
//! makes it a migration-1 requirement.
//!
//! Every one of those sentences was written about the `GROUP BY` form, where the
//! ordering question does not even arise — `tag IN (?,?)` is order-independent —
//! and inherited by the chain without being re-measured on it. This file
//! measures it on the chain.
//!
//! # Why this is one file and not a fifth [`Shape`]
//!
//! Ordering cannot change a conjunction's *result*, so there is nothing here for
//! the 89 rules to catch that they do not already catch for
//! [`Shape::Chain`] — and a fifth shape would re-run every table in
//! `results/` to vary something orthogonal to all of them. What it needs instead
//! is a **local** correctness control, and it has one: every round asserts the
//! two orderings return the identical position before either time is recorded.
//!
//! # The one confound this had to remove
//!
//! Two statements measured in a fixed order on one connection are not comparable:
//! whichever goes second inherits the other's page-cache churn, and here the
//! difference turned out to be large enough that a reader would be right to
//! suspect it. So which ordering runs first **alternates every round**, and each
//! median is over samples drawn from both positions.
//!
//! [`Selectivity::inverted`] is the whole of the difference. The SQL builder, the
//! schema, the rows, the connection and the transaction are the same.
//!
//! Run it with `cargo test --release --manifest-path
//! experiments/shipped-append-condition-sql/Cargo.toml --test seed_ordering --
//! --nocapture`.

mod support;

use happenstance_core::SequencePosition;
use happenstance_sqlite::connection::ConnectionSettings;
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_testkit::fixtures::{condition, condition_after, query_of};
use rusqlite::Connection;
use rusqlite::types::Value;
use shipped_append_condition_sql::Shape;
use shipped_append_condition_sql::chain::{Selectivity, arms_sql, guard_sql};

/// Events in the store. One size, because the question is a ratio and not a
/// scale: if most-selective-first buys nothing at 500,000 it is not buying
/// anything at 50,000 either, and 500,000 is where the difference would be
/// largest if there were one.
const SIZE: u64 = 500_000;

/// Rounds per ordering, interleaved.
const ROUNDS: usize = 25;

fn median(samples: &mut [u128]) -> u128 {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

/// Runs one guard statement inside `BEGIN IMMEDIATE`, exactly as the shipped
/// `append` does, and rolls back.
fn timed_guard(
    connection: &Connection,
    sql: &str,
    params: &[Value],
) -> (u128, Option<SequencePosition>) {
    let started = std::time::Instant::now();
    connection
        .execute_batch("BEGIN IMMEDIATE")
        .expect("the write lock must be available");
    let highest: Option<i64> = connection
        .query_row(sql, rusqlite::params_from_iter(params.iter()), |row| {
            row.get(0)
        })
        .expect("the guard must run");
    connection.execute_batch("ROLLBACK").expect("rollback");
    (
        started.elapsed().as_micros(),
        highest.and_then(|value| SequencePosition::new(value.unsigned_abs())),
    )
}

#[test]
fn most_selective_first_against_least_selective_first() {
    let path = tempdb("seed-ordering");

    let store_id = {
        drop(SqliteEventStore::open(&path).expect("migrating must succeed"));
        let connection =
            happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");
        connection
            .query_row(
                "SELECT CAST(v AS BLOB) FROM store_meta WHERE k = 'store_id'",
                [],
                |row| {
                    let raw: Vec<u8> = row.get(0)?;
                    let mut bytes = [0u8; 16];
                    bytes.copy_from_slice(&raw[..16]);
                    Ok(happenstance_core::StoreId::from_bytes(bytes))
                },
            )
            .expect("the adapter minted an incarnation")
    };
    {
        let mut connection =
            happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");
        shipped_append_condition_sql::seed::seed(&mut connection, store_id, SIZE)
            .expect("seeding must succeed");
    }

    let connection =
        happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");
    let settings = ConnectionSettings::read_back(&connection).expect("pragmas must read back");
    let check =
        shipped_append_condition_sql::seed::verify(&connection, SIZE).expect("verify must succeed");
    assert!(
        check.is_sound(SIZE),
        "the seed must be sound before anything is timed: {}",
        check.line()
    );
    let conditions = format!(
        "journal_mode={} synchronous={} busy_timeout_ms={} sqlite={}",
        settings.journal_mode(),
        settings.synchronous(),
        settings.busy_timeout_ms(),
        rusqlite::version(),
    );
    println!("SEED\tsize={SIZE}\t{}\t{conditions}", check.line());

    let query = query_of(&["Seeded"], &[("shard", "cold"), ("row", "r7")]);
    let selectivity = Selectivity::read_for(&connection, &query).expect("the lookup must succeed");
    let inverted = selectivity.inverted();
    let items = query.items().expect("the query has items");
    println!(
        "CARDINALITY\tshard_cold={:?}\trow_r7={:?}",
        selectivity.count_of("shard:cold"),
        selectivity.count_of("row:r7"),
    );

    for (label, guard) in [
        ("accepted-2tag-at-head", condition_after(query.clone(), SIZE)),
        ("rejected-2tag-unbounded", condition(query.clone())),
        (
            "rejected-2tag-midlog",
            condition_after(query.clone(), SIZE / 2),
        ),
    ] {
        let boundary = guard
            .guards()
            .first()
            .expect("the condition carries one guard")
            .after
            .map_or(0, |after| i64::try_from(after.get()).unwrap_or(i64::MAX));

        let mut most = Vec::new();
        let mut least = Vec::new();
        let mut most_params = Vec::new();
        let most_sql = guard_sql(&arms_sql(
            Shape::Chain,
            items,
            &selectivity,
            boundary,
            &mut most_params,
        ));
        let mut least_params = Vec::new();
        let least_sql = guard_sql(&arms_sql(
            Shape::Chain,
            items,
            &inverted,
            boundary,
            &mut least_params,
        ));
        assert_ne!(
            most_params, least_params,
            "the two orderings must actually bind the tags in different \
             positions, or this file is timing the same statement twice"
        );

        // **Which ordering runs first alternates every round.** Without that,
        // one of the two always follows the other's page-cache churn, and the
        // whole difference could be read as an artefact of going second. With
        // it, each ordering runs first in exactly half the rounds and the medians
        // are over samples that saw both positions.
        for round in 0..ROUNDS {
            let (a, b) = if round % 2 == 0 {
                let (us, a) = timed_guard(&connection, &most_sql, &most_params);
                most.push(us);
                let (us, b) = timed_guard(&connection, &least_sql, &least_params);
                least.push(us);
                (a, b)
            } else {
                let (us, b) = timed_guard(&connection, &least_sql, &least_params);
                least.push(us);
                let (us, a) = timed_guard(&connection, &most_sql, &most_params);
                most.push(us);
                (a, b)
            };
            // The local correctness control: ordering a conjunction cannot change
            // its answer, and if it ever did, the faster one would be wrong.
            assert_eq!(
                a, b,
                "{label}: the two orderings must return the same position"
            );
        }

        println!("SEED-ORDER\tsize={SIZE}\tscenario={label}\tboundary={boundary}\t\
                  most_selective_first_us_median={}\tleast_selective_first_us_median={}\t\
                  rounds={ROUNDS}\t{conditions}",
            median(&mut most),
            median(&mut least),
        );
        // The two statements are the same *text*: which tag is the seed and
        // which is the chained subquery lives in the bound parameters, not in
        // the SQL. The `assert_ne!` above is what proves they differ, and the
        // parameters are printed so a reader can see the swap rather than take
        // it on trust.
        println!("   statement (identical text for both orderings): {most_sql}");
        println!("   most-selective-first bound (what ships): {most_params:?}");
        println!("   least-selective-first bound (the control): {least_params:?}");
    }

    remove(&path);
}

fn tempdb(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "happenstance-shipped-guard-{}-{name}.sqlite3",
        std::process::id()
    ));
    remove(&path);
    path
}

fn remove(path: &std::path::Path) {
    for suffix in ["", "-wal", "-shm"] {
        let mut candidate = path.to_path_buf().into_os_string();
        candidate.push(suffix);
        let _ = std::fs::remove_file(std::path::PathBuf::from(candidate));
    }
}
