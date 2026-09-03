//! The log every figure is taken over, and the queries taken over it.
//!
//! # The shape of the seeded log, and why it is this shape
//!
//! One million events, each carrying **two** tags:
//!
//! * `all:1` — on every event. It is what makes the *one-item* query a query
//!   that matches the whole log, so that at every page size a page fills. A
//!   one-item query matching 800 events would have made the width axis a
//!   selectivity axis as well, and a run that varies two things says less than
//!   either of the runs that vary one.
//! * `w:<n>` for `n = i mod TAG_POOL` — the pool the wide queries are built out
//!   of. With `TAG_POOL = 1200`, the 1,200-item query matches every event and
//!   the 400-item query matches a third of them.
//!
//! Both are single-tag query items, which is the overwhelmingly common shape of
//! a consistency boundary and the one `query_sql.rs:34-45` documents the fast
//! path for. That keeps `Selectivity::read_for` out of the timed region — it
//! issues no statement at all for a query whose every item names at most one tag
//! — so the lock-hold figure is the *page*'s, not the cardinality lookup's.
//!
//! # The payload
//!
//! 128 bytes. Small enough that a million of them fit in a temporary file
//! without the seed dominating the run, and large enough that the residency
//! figure is not entirely `SequencedEvent`'s own structure. The separate
//! ceiling arm in `tests/page_lock_hold.rs` is what covers the other end —
//! events at `MAX_EVENT_DATA_LEN`, which is where R-1's claim actually lives.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use happenstance_core::{Event, Query, QueryItem, SendEventStore, Tags};

use crate::replica::Replica;

/// How many distinct `w:<n>` tags the log carries.
pub const TAG_POOL: usize = 1_200;

/// The payload every seeded event carries, in bytes.
pub const PAYLOAD_BYTES: usize = 128;

/// A temporary database file that removes itself, WAL companions included.
#[derive(Debug)]
pub struct Scratch {
    path: PathBuf,
}

impl Scratch {
    /// A fresh path under the system temporary directory.
    ///
    /// No `tempfile` dependency: a process-local ordinal plus `Drop` cleanup has
    /// precedent at
    /// `crates/happenstance-testkit/tests/fixture_instruments.rs:95-102`, and
    /// this crate adds no dependency the workspace does not already carry.
    #[must_use]
    pub fn new(label: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let name = format!(
            "happenstance-one-connection-latency-{}-{label}-{ordinal}.sqlite3",
            std::process::id()
        );
        Self {
            path: std::env::temp_dir().join(name),
        }
    }

    /// Where the database lives.
    #[must_use]
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        for suffix in ["", "-wal", "-shm"] {
            let mut path = self.path.clone().into_os_string();
            path.push(suffix);
            let _ = std::fs::remove_file(PathBuf::from(path));
        }
    }
}

/// One event of the seeded shape, at ordinal `i`.
///
/// # Panics
///
/// Panics if the contract refuses the type or the tags, which would mean this
/// function is wrong rather than the store.
#[must_use]
pub fn seeded_event(i: usize, payload_bytes: usize) -> Event {
    let tags = Tags::from_pairs([("all", "1"), ("w", &(i % TAG_POOL).to_string()[..])])
        .expect("the seeded tag shape is valid");
    #[allow(
        clippy::cast_possible_truncation,
        reason = "a payload byte pattern, not a value"
    )]
    let data = vec![(i % 251) as u8; payload_bytes];
    Event::new("seeded", data).expect("`seeded` is a valid event type").with_tags(tags)
}

/// Appends `total` events in batches of `batch`, reporting progress.
///
/// # Panics
///
/// Panics if any append fails. A partially seeded log would silently shorten
/// every replay taken over it.
pub async fn seed<const PAGE: usize>(store: &Replica<PAGE>, total: usize, batch: usize) {
    let started = std::time::Instant::now();
    let mut next = 0usize;
    while next < total {
        let end = (next + batch).min(total);
        let events: Vec<Event> = (next..end)
            .map(|i| seeded_event(i, PAYLOAD_BYTES))
            .collect();
        store
            .append(&events, None)
            .await
            .unwrap_or_else(|err| panic!("seeding append at {next} failed: {err}"));
        next = end;
        if next % 100_000 == 0 {
            println!(
                "    seeded {next} / {total} events ({:.1} s)",
                started.elapsed().as_secs_f64()
            );
        }
    }
    println!(
        "    seeded {total} events in {:.1} s",
        started.elapsed().as_secs_f64()
    );
}

/// A query of `width` single-tag items.
///
/// Width 1 is the `all:1` item, which matches the whole log; every wider query
/// is built from the `w:<n>` pool. See the module documentation for why the two
/// are different tags.
///
/// # Panics
///
/// Panics if `width` exceeds [`TAG_POOL`], or if the contract refuses the items
/// — both mean this function is wrong.
#[must_use]
pub fn query_of_width(width: usize) -> Query {
    assert!(width >= 1, "a query of no items is not a query");
    assert!(
        width <= TAG_POOL,
        "the seeded log carries only {TAG_POOL} distinct `w:` tags"
    );

    if width == 1 {
        let tags = Tags::from_pairs([("all", "1")]).expect("`all:1` is a valid tag");
        return Query::from_item(QueryItem::tagged(tags).expect("a one-tag item is constrained"));
    }

    let items: Vec<QueryItem> = (0..width)
        .map(|n| {
            let tags =
                Tags::from_pairs([("w", &n.to_string()[..])]).expect("`w:<n>` is a valid tag");
            QueryItem::tagged(tags).expect("a one-tag item is constrained")
        })
        .collect();
    Query::from_items(items).expect("a non-empty item list is a valid query")
}

/// The `q`-th percentile of an unsorted nanosecond slice, in **milliseconds**.
#[must_use]
pub fn percentile_ms(nanos: &mut Vec<u64>, q: f64) -> f64 {
    nanos.sort_unstable();
    crate::probe::percentile_us(nanos, q) / 1_000.0
}
