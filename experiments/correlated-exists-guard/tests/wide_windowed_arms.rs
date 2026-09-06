//! The windowed arm at VT-23's floor: 128 items in one statement.
//!
//! # The gap this closes
//!
//! [`results/windowed-arms.md`](../results/windowed-arms.md) measured a shape
//! that bounds **each arm** by the page budget, and found it beats both the
//! shipped wrapper and `wrapper-exists` on every cell. Its own *"what this page
//! does not show"* names the gap that matters most: **every cell there is a
//! union of one arm**, while the soundness argument is about a union of many,
//! and the shape's cost is `budget x arms`.
//!
//! At VT-23's floor that is `512 x 128 = 65,536` positions materialised where
//! one arm materialises 512. A shape whose whole claim is *"the matched set is
//! bounded"* has to be asked what the bound is when the specification's widest
//! conformant query is the one being served.
//!
//! `SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT` is 400, so 128 items is
//! **one** statement rather than a merge across several. The width lands
//! entirely inside one `UNION`, which is what makes this a measurement of arm
//! count rather than of the chunk-and-merge loop.
//!
//! # Two queries, because width has two ends
//!
//! Both are 128 items over [`Corpus::ManyBuckets`], and both name two tags and
//! a type per item, so the arm shape is the one the other tables measured.
//!
//! * **`partition-128`** — item *i* names `bucket:b{i}`. Every event matches
//!   exactly one arm, so the union is the whole log and no arm duplicates
//!   another. This is where a per-arm budget has the most arms to be multiplied
//!   by: 65,536 positions against the shipped shape's 500,000.
//! * **`needle-128`** — 127 arms naming buckets that do not exist, and one that
//!   does. The union matches about one event in 128. This is the adversarial
//!   cell: the matched set is *already* small, so the windowed shape has almost
//!   nothing to save and pays 128 subquery preambles to save it. If per-arm
//!   overhead is what makes this shape a bad idea, it shows up here.
//!
//! The prediction, stated before the run: the windowed set is never *larger*
//! than the shipped one — `min(arm matches, budget)` summed cannot exceed the
//! matched set — so the only way it loses is per-arm overhead, and `needle-128`
//! is where that is undiluted.
//!
//! **The prediction held and answered the wrong question.** The windowed shape
//! does beat the shipped one in all eight cells, on both queries, exactly as the
//! bound argues. It loses on `partition-128` to `wrapper-exists`, which is not
//! bounded by `budget x arms` because it materialises nothing at all — so the
//! comparison that decides is the one the prediction was not about.
//! [`results/wide-arms.md`](../results/wide-arms.md).
//!
//! # What is asserted before anything is timed
//!
//! **The returned page**, every column of every row, across all three shapes,
//! every round. At 128 arms the thing most likely to go wrong is not speed: it
//! is an arm shape that drops rows and returns a page that is merely *short*.
//!
//! Run it with `cargo test --release --test wide_windowed_arms -- --nocapture`.

use correlated_exists_guard::Shape;
use correlated_exists_guard::chain::{
    PAGE_COLUMNS, Selectivity, Window, Wrapper, arms_sql, correlated_arms_sql, page_sql_directed,
    windowed_arms_sql,
};
use correlated_exists_guard::seed::{BUCKETS, Corpus, SEED_TYPE, bucket_tag};
use happenstance_core::{Query, QueryItem, Tags};
use happenstance_sqlite::connection::ConnectionSettings;
use happenstance_sqlite::event_store::SqliteEventStore;
use rusqlite::Connection;
use rusqlite::types::Value;

/// Events in the store, matching every other read table in this crate.
const SIZE: u64 = 500_000;

/// `PAGE_SIZE` from `crates/happenstance-sqlite/src/event_store.rs:141`.
const PAGE_SIZE: i64 = 512;

/// VT-23's floor: the item count every conformant store must evaluate.
const ITEMS: u64 = 128;

/// Rounds per cell, interleaved.
///
/// Five rather than the narrow file's ten. `needle-128` under `wrapper-exists`
/// walks `event` until 512 of one event in 128 have matched, testing 128 arms
/// per row it rejects, and it is the slowest statement in this crate.
const ROUNDS: usize = 5;

type Row = (i64, String, Vec<u8>, Option<Vec<u8>>, Vec<u8>, i64);

fn median(samples: &mut [u128]) -> u128 {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

/// A 128-item query whose *i*th item names `bucket:b{buckets[i]}` and
/// `shard:cold`.
fn wide_query(buckets: &[u64]) -> Query {
    let items = buckets
        .iter()
        .map(|k| {
            let value = bucket_tag(*k);
            let value = value
                .split_once(':')
                .expect("bucket_tag writes one colon")
                .1
                .to_owned();
            let tags = Tags::from_pairs([("bucket", value.as_str()), ("shard", "cold")])
                .expect("two non-empty pairs");
            QueryItem::new([SEED_TYPE], tags).expect("a non-empty item")
        })
        .collect::<Vec<_>>();
    Query::from_items(items).expect("a non-empty item list")
}

fn timed_page(connection: &Connection, sql: &str, params: &[Value]) -> (u128, Vec<Row>) {
    let started = std::time::Instant::now();
    let mut statement = connection.prepare(sql).expect("the statement must prepare");
    let mut rows = statement
        .query(rusqlite::params_from_iter(params.iter()))
        .expect("the page must run");
    let mut page = Vec::with_capacity(512);
    while let Some(row) = rows.next().expect("a page row") {
        page.push((
            row.get(0).expect("position"),
            row.get(1).expect("event_type"),
            row.get(2).expect("data"),
            row.get(3).expect("metadata"),
            row.get(4).expect("tags"),
            row.get(6).expect("origin_position"),
        ));
    }
    (started.elapsed().as_micros(), page)
}

fn shapes(
    items: &[QueryItem],
    selectivity: &Selectivity,
    window: Window,
    ceiling: i64,
) -> Vec<(&'static str, String, Vec<Value>)> {
    let tail = [
        Value::Integer(window.lo),
        Value::Integer(ceiling),
        Value::Integer(window.budget),
    ];
    let mut out = Vec::new();

    let mut params = Vec::new();
    let matched = arms_sql(Shape::ChainExists, items, selectivity, 0, &mut params);
    params.extend_from_slice(&tail);
    out.push((
        "in-exists (ships)",
        page_sql_directed(
            Wrapper::InSubquery,
            &matched,
            PAGE_COLUMNS,
            window.backwards,
        ),
        params,
    ));

    let mut params = Vec::new();
    let matched = correlated_arms_sql(items, selectivity, &mut params);
    params.extend_from_slice(&tail);
    out.push((
        "wrapper-exists",
        page_sql_directed(
            Wrapper::CorrelatedExists,
            &matched,
            PAGE_COLUMNS,
            window.backwards,
        ),
        params,
    ));

    let mut params = Vec::new();
    let matched = windowed_arms_sql(items, selectivity, window, &mut params);
    params.extend_from_slice(&tail);
    out.push((
        "windowed-in-exists",
        page_sql_directed(
            Wrapper::InSubquery,
            &matched,
            PAGE_COLUMNS,
            window.backwards,
        ),
        params,
    ));

    out
}

#[test]
fn the_windowed_arm_at_the_item_floor() {
    let path = tempdb("wide");
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
        correlated_exists_guard::seed::seed_corpus(
            &mut connection,
            store_id,
            SIZE,
            Corpus::ManyBuckets,
        )
        .expect("seeding must succeed");
    }
    let connection =
        happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");

    let settings = ConnectionSettings::read_back(&connection).expect("pragmas must read back");
    println!(
        "CONDITIONS\tjournal_mode={}\tsynchronous={}\tbusy_timeout_ms={}\tsqlite={}\t\
         size={SIZE}\titems={ITEMS}\tbuckets={BUCKETS}\tarms_per_statement={}",
        settings.journal_mode(),
        settings.synchronous(),
        settings.busy_timeout_ms(),
        rusqlite::version(),
        SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT,
    );

    // Every bucket that exists, so the union is a partition of the log.
    let partition: Vec<u64> = (0..ITEMS).collect();
    // 127 buckets that do not exist, and one that does. `BUCKETS` is 128, so
    // anything at or above it matches nothing.
    let mut needle: Vec<u64> = (BUCKETS + 100..BUCKETS + 227).collect();
    needle.push(7);
    assert_eq!(partition.len(), usize::try_from(ITEMS).expect("128 fits"));
    assert_eq!(needle.len(), usize::try_from(ITEMS).expect("128 fits"));

    let ceiling = i64::try_from(SIZE).unwrap_or(i64::MAX);
    let half = i64::try_from(SIZE / 2).unwrap_or(i64::MAX);
    let late = i64::try_from(SIZE - SIZE / 10).unwrap_or(i64::MAX);

    for (query_name, buckets) in [("partition-128", &partition), ("needle-128", &needle)] {
        let query = wide_query(buckets);
        let selectivity =
            Selectivity::read_for(&connection, &query).expect("the lookup must succeed");
        let items = query.items().expect("the query has items");

        // How many events the whole union matches, read independently of any
        // shape under test — the number that says which end of the width axis
        // this cell is.
        let present: Vec<String> = buckets
            .iter()
            .filter(|k| **k < BUCKETS)
            .map(|k| bucket_tag(*k))
            .collect();
        let matched: i64 = if present.is_empty() {
            0
        } else {
            let placeholders = vec!["?"; present.len()].join(",");
            connection
                .query_row(
                    &format!(
                        "SELECT count(DISTINCT position) FROM event_tag WHERE tag IN ({placeholders})"
                    ),
                    rusqlite::params_from_iter(present.iter()),
                    |row| row.get(0),
                )
                .expect("the matched count must read")
        };
        println!("MATCHED\tquery={query_name}\tarms={ITEMS}\tevents={matched}");

        for (cell, window) in [
            (
                "first-page",
                Window {
                    lo: 1,
                    hi: ceiling,
                    budget: PAGE_SIZE,
                    backwards: false,
                },
            ),
            (
                "mid-replay-page",
                Window {
                    lo: half,
                    hi: ceiling,
                    budget: PAGE_SIZE,
                    backwards: false,
                },
            ),
            (
                "late-replay-page",
                Window {
                    lo: late,
                    hi: ceiling,
                    budget: PAGE_SIZE,
                    backwards: false,
                },
            ),
            (
                "backwards-from-head",
                Window {
                    lo: 1,
                    hi: ceiling,
                    budget: PAGE_SIZE,
                    backwards: true,
                },
            ),
        ] {
            let built = shapes(items, &selectivity, window, ceiling);

            // The statement's own size, which is the thing arm count buys or
            // costs and is invisible in a timing.
            for (name, sql, params) in &built {
                println!(
                    "STATEMENT\tquery={query_name}\tcell={cell}\tshape={name}\t\
                     sql_bytes={}\tparams={}",
                    sql.len(),
                    params.len(),
                );
            }

            let mut samples: Vec<Vec<u128>> = vec![Vec::new(); built.len()];
            let mut control: Option<Vec<Row>> = None;

            for round in 0..ROUNDS {
                for offset in 0..built.len() {
                    let index = (round + offset) % built.len();
                    let (micros, page) = timed_page(&connection, &built[index].1, &built[index].2);
                    match &control {
                        None => control = Some(page),
                        Some(expected) => assert_eq!(
                            &page, expected,
                            "shape {} returned a different page at {ITEMS} arms",
                            built[index].0
                        ),
                    }
                    samples[index].push(micros);
                }
            }

            let rows = control.as_ref().map_or(0, Vec::len);
            for (index, (name, _, _)) in built.iter().enumerate() {
                println!(
                    "PAGE\tquery={query_name}\tcell={cell}\tshape={name}\trows={rows}\t\
                     us_median={}",
                    median(&mut samples[index]),
                );
            }
        }
    }

    drop(connection);
    remove(&path);
}

fn tempdb(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "happenstance-exists-wide-{}-{name}.sqlite3",
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
