//! Where `workerd`'s SQLite stops, measured inside the runtime that stops it.
//!
//! `experiments/durable-object-limits`' phase A, moved onto the real runtime.
//! The shim it ran on is `node:sqlite` at SQLite's compiled defaults, and a
//! Durable Object is not: `workerd` lowers several `sqlite3_limit`s on every
//! database it opens. Reading those numbers out of `workerd`'s source is an
//! argument; this module is the measurement.
//!
//! Each axis is searched by bisection for the largest size the runtime accepts,
//! and the refusal one step above it is reported verbatim, so a reader can see
//! *which* limit refused rather than trust the axis's name. Four axes go
//! through the adapter rather than raw SQL — query items, append-condition
//! items, tags in one item, and maximum-length tags in one item — because those
//! are the shapes the conformance rules and the partition constants are about.
//! One raw axis, `json_each` elements in one parameter, measures the function
//! the adapter's rendering rests on (ADR-0079): the authorizer must allow it.

use core::fmt::Write as _;
use core::future::poll_fn;
use core::pin::pin;

use futures_core::Stream;
use happenstance_cloudflare::{CloudflareEventStore, SqlStorage, SqlValue, TableNamespace};
use happenstance_core::{
    AppendCondition, AppendError, Event, EventStore, MAX_TAG_LEN, Query, QueryItem, ReadOptions,
    Tag, Tags,
};

/// One measured axis.
#[derive(Debug, PartialEq, Eq)]
pub struct Wall {
    /// What was varied.
    pub axis: &'static str,
    /// The largest size accepted, or `None` if even the smallest was refused.
    pub accepted: Option<usize>,
    /// The upper end of the search.
    pub searched_to: usize,
    /// The refusal at `accepted + 1`, or `None` if nothing up to `searched_to`
    /// was refused.
    pub refusal: Option<String>,
}

impl Wall {
    /// One line of the report.
    #[must_use]
    pub fn line(&self) -> String {
        let accepted = self
            .accepted
            .map_or_else(|| "none".to_owned(), |n| n.to_string());
        match &self.refusal {
            None => format!(
                "{}: accepted {accepted}; no wall up to {}",
                self.axis, self.searched_to
            ),
            Some(refusal) => format!(
                "{}: accepted {accepted}; refused above it: {refusal}",
                self.axis
            ),
        }
    }
}

/// Bisects `1..=searched_to` for the largest size `attempt` accepts.
///
/// Assumes acceptance is monotone in size, which is what a limit is.
async fn bisect<F>(axis: &'static str, searched_to: usize, attempt: F) -> Wall
where
    F: AsyncFn(usize) -> Result<(), String>,
{
    if let Err(refusal) = attempt(1).await {
        return Wall {
            axis,
            accepted: None,
            searched_to,
            refusal: Some(refusal),
        };
    }
    if attempt(searched_to).await.is_ok() {
        return Wall {
            axis,
            accepted: Some(searched_to),
            searched_to,
            refusal: None,
        };
    }
    // Invariant: `low` accepted, `high` refused.
    let (mut low, mut high) = (1, searched_to);
    while high - low > 1 {
        let mid = low + (high - low) / 2;
        if attempt(mid).await.is_ok() {
            low = mid;
        } else {
            high = mid;
        }
    }
    Wall {
        axis,
        accepted: Some(low),
        searched_to,
        refusal: attempt(high).await.err(),
    }
}

/// Runs `statement` and drains it, so a refusal at step time is seen too.
fn run(sql: &SqlStorage, statement: &str, bindings: &[SqlValue]) -> Result<(), String> {
    let mut cursor = sql.exec(statement, bindings).map_err(|e| e.to_string())?;
    while let Some(row) = cursor.next_row() {
        row.map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn compound_select(sql: &SqlStorage, terms: usize) -> Result<(), String> {
    run(sql, &vec!["SELECT 1"; terms].join(" UNION ALL "), &[])
}

fn variables(sql: &SqlStorage, count: usize) -> Result<(), String> {
    let placeholders = vec!["?"; count].join(",");
    let bindings: Vec<SqlValue> = (0..count)
        .map(|n| SqlValue::Integer(i64::try_from(n).unwrap_or(i64::MAX)))
        .collect();
    run(sql, &format!("SELECT {placeholders}"), &bindings)
}

fn sql_length(sql: &SqlStorage, bytes: usize) -> Result<(), String> {
    const HEAD: &str = "SELECT 1 --";
    let padding = "x".repeat(bytes.saturating_sub(HEAD.len()));
    run(sql, &format!("{HEAD}{padding}"), &[])
}

fn expression_depth(sql: &SqlStorage, terms: usize) -> Result<(), String> {
    run(sql, &format!("SELECT {}", vec!["1"; terms].join("+")), &[])
}

/// `SELECT count(*) FROM json_each(?)` over a JSON array of `elements` short
/// strings: whether the runtime's authorizer allows the table-valued function
/// at all, and how many elements one bound parameter can carry.
fn json_each_elements(sql: &SqlStorage, elements: usize) -> Result<(), String> {
    let mut array = String::with_capacity(elements.saturating_mul(6).saturating_add(2));
    array.push('[');
    for n in 0..elements {
        if n > 0 {
            array.push(',');
        }
        array.push_str("\"t");
        array.push_str(&n.to_string());
        array.push('"');
    }
    array.push(']');
    run(
        sql,
        "SELECT count(*) FROM json_each(?)",
        &[SqlValue::Text(array)],
    )
}

fn row_bytes(sql: &SqlStorage, bytes: usize) -> Result<(), String> {
    run(
        sql,
        "CREATE TABLE IF NOT EXISTS probe_row (payload BLOB NOT NULL)",
        &[],
    )?;
    let outcome = run(
        sql,
        "INSERT INTO probe_row (payload) VALUES (?)",
        &[SqlValue::Blob(vec![0_u8; bytes])],
    );
    run(sql, "DELETE FROM probe_row", &[])?;
    outcome
}

/// A store for the adapter-level axes, in its own namespace so it touches no
/// conformance log.
///
/// Holding one event, because a read of an empty store has no ceiling to
/// sample and so issues no query at all: on an empty store every width
/// "passes", which is a measurement of nothing.
async fn probe_store(sql: &SqlStorage) -> Result<CloudflareEventStore, String> {
    let namespace = TableNamespace::new("probe").map_err(|e| e.to_string())?;
    // A second handle onto the object's one database; `SqlStorage` aliases.
    let store = CloudflareEventStore::namespaced(sql.clone(), &namespace);
    store.migrate().map_err(|e| e.to_string())?;
    let seed = Event::new("Probed", &b"probe"[..])
        .map_err(|e| e.to_string())?
        .with_tags(core::iter::once(tag(0)?).collect());
    store
        .append(&[seed], None)
        .await
        .map_err(|e| e.to_string())?;
    Ok(store)
}

/// `items` one-tag items as an append condition — the write path's translation
/// of the same query, where a refusal arrives as `AppendError::Store`.
async fn condition_items(store: &CloudflareEventStore, items: usize) -> Result<(), String> {
    let condition = AppendCondition::new(one_tag_items(items)?);
    let event = Event::new("Guarded", &b"probe"[..]).map_err(|e| e.to_string())?;
    match store.append(&[event], Some(&condition)).await {
        Ok(_) | Err(AppendError::ConditionViolated(_)) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

fn tag(n: usize) -> Result<Tag, String> {
    Tag::new(format!("t{n}")).map_err(|e| e.to_string())
}

/// A distinct tag of exactly `MAX_TAG_LEN` bytes: the number, left-padded.
fn long_tag(n: usize) -> Result<Tag, String> {
    Tag::new(format!("{n:x>MAX_TAG_LEN$}")).map_err(|e| e.to_string())
}

/// Drains `store.read(query)`, reporting the first error.
async fn read_all(store: &CloudflareEventStore, query: &Query) -> Result<(), String> {
    let mut stream = pin!(store.read(query, ReadOptions::default()));
    while let Some(item) = poll_fn(|cx| stream.as_mut().poll_next(cx)).await {
        item.map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// `items` items of one tag each: the VT-23 shape.
fn one_tag_items(items: usize) -> Result<Query, String> {
    let items = (0..items)
        .map(|n| {
            let tags: Tags = core::iter::once(tag(n)?).collect();
            QueryItem::new(core::iter::empty::<String>(), tags).map_err(|e| e.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    Query::from_items(items).map_err(|e| e.to_string())
}

/// One item per tag, one tag per item, read.
async fn query_items(store: &CloudflareEventStore, items: usize) -> Result<(), String> {
    read_all(store, &one_tag_items(items)?).await
}

/// One item carrying `tags` distinct tags, each made by `make`.
async fn tags_in_one_item(
    store: &CloudflareEventStore,
    tags: usize,
    make: fn(usize) -> Result<Tag, String>,
) -> Result<(), String> {
    let tags: Tags = (0..tags).map(make).collect::<Result<_, _>>()?;
    let item = QueryItem::new(core::iter::empty::<String>(), tags).map_err(|e| e.to_string())?;
    read_all(store, &Query::from_item(item)).await
}

/// Every axis, as report lines.
///
/// The upper ends are SQLite's compiled defaults or a little past them, so a
/// runtime at the defaults reports "no wall" rather than a number the search
/// invented; the row axis stops at 16 MiB, twice `workerd`'s own row ceiling.
/// The maximum-length tag axis stops at 40,000 tags of 255 bytes, about 10 MB
/// of JSON, past the deployed object's 8,388,637-byte length limit.
pub async fn report(sql: &SqlStorage) -> String {
    let mut walls = vec![
        bisect("compound SELECT terms", 600, async |n| {
            compound_select(sql, n)
        })
        .await,
        bisect("bound parameters", 33_000, async |n| variables(sql, n)).await,
        bisect("statement length (bytes)", 1_100_000, async |n| {
            sql_length(sql, n)
        })
        .await,
        bisect("expression terms (1+1+...)", 1_100, async |n| {
            expression_depth(sql, n)
        })
        .await,
        bisect("json_each elements in one parameter", 100_000, async |n| {
            json_each_elements(sql, n)
        })
        .await,
        bisect("row payload (bytes)", 16 * 1024 * 1024, async |n| {
            row_bytes(sql, n)
        })
        .await,
    ];
    match probe_store(sql).await {
        Ok(store) => {
            walls.push(
                bisect("adapter: query items, one tag each", 1_024, async |n| {
                    query_items(&store, n).await
                })
                .await,
            );
            walls.push(
                bisect(
                    "adapter: append-condition items, one tag each",
                    1_024,
                    async |n| condition_items(&store, n).await,
                )
                .await,
            );
            walls.push(
                bisect("adapter: tags in one query item", 1_024, async |n| {
                    tags_in_one_item(&store, n, tag).await
                })
                .await,
            );
            walls.push(
                bisect(
                    "adapter: max-length tags in one query item",
                    40_000,
                    async |n| tags_in_one_item(&store, n, long_tag).await,
                )
                .await,
            );
        }
        Err(err) => walls.push(Wall {
            axis: "adapter axes",
            accepted: None,
            searched_to: 0,
            refusal: Some(format!("the probe store did not open: {err}")),
        }),
    }
    let mut out = String::new();
    for wall in walls {
        // Writing to a `String` cannot fail; `writeln!` returns `fmt::Result`
        // only because the trait is shared with sinks that can.
        if writeln!(out, "{}", wall.line()).is_err() {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use happenstance_testkit::block_on;

    use super::{Wall, bisect};

    /// A runtime whose wall sits at `wall`: everything up to it is accepted.
    fn walled(wall: usize) -> impl AsyncFn(usize) -> Result<(), String> {
        async move |n: usize| {
            if n <= wall {
                Ok(())
            } else {
                Err(format!("refused {n}"))
            }
        }
    }

    #[test]
    fn a_wall_inside_the_range_is_found_exactly() {
        for wall in [1, 2, 5, 99, 100, 101, 599] {
            assert_eq!(
                block_on(bisect("axis", 600, walled(wall))),
                Wall {
                    axis: "axis",
                    accepted: Some(wall),
                    searched_to: 600,
                    refusal: Some(format!("refused {}", wall + 1)),
                },
                "wall at {wall}"
            );
        }
    }

    #[test]
    fn no_wall_in_the_range_is_reported_as_none_found() {
        let wall = block_on(bisect("axis", 600, walled(600)));
        assert_eq!(wall.accepted, Some(600));
        assert_eq!(wall.refusal, None);
        assert_eq!(wall.line(), "axis: accepted 600; no wall up to 600");
    }

    #[test]
    fn refusing_the_smallest_size_is_reported_as_nothing_accepted() {
        let wall = block_on(bisect("axis", 600, walled(0)));
        assert_eq!(wall.accepted, None);
        assert_eq!(wall.refusal.as_deref(), Some("refused 1"));
        assert_eq!(
            wall.line(),
            "axis: accepted none; refused above it: refused 1"
        );
    }
}
