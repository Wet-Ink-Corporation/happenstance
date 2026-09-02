//! The three CF-40 boundary searches, against the real adapter on the real
//! runtime.
//!
//! Run from this directory:
//!
//! ```console
//! $ CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
//!     cargo test --target wasm32-unknown-unknown -- --nocapture
//! ```
//!
//! # Two phases, and running only one of them is how this measurement goes wrong
//!
//! **Phase A — where the runtime stops.** Statements issued through
//! `SqlStorage::exec`, against the schema `CloudflareEventStore::migrate`
//! created, with the adapter's *policy* deliberately bypassed. This is the half
//! that answers "is there a wall, and where".
//!
//! **Phase B — where the adapter stops.** The same shapes driven through
//! `CloudflareEventStore::append`, which refuses above its own declared ceilings
//! **before issuing any SQL**. This is the half that answers "is the declaration
//! where it claims to be, and does the refusal name the right `StoreLimit`".
//!
//! The split exists because the first version of this file had only phase B, and
//! it stopped being a measurement the moment the adapter declared its ceilings:
//! it dutifully reported 1 MiB, 1,024 and 1,024 — the declaration, read back to
//! itself. A probe that can only rediscover the number it was given is the
//! reproducible *form* of a measurement without the thing itself, which is the
//! failure `experiments/wire-format`'s README was written about.
//!
//! # What each search is asking, and what it is not
//!
//! **The adapter, not the platform.** A published Cloudflare figure — the 2 MiB
//! maximum row size a Durable Object's SQLite enforces — is a legitimate place
//! to *start*. It is not the answer, because the declaration is a promise about
//! this adapter's behaviour, and this adapter's row carries more than `data`: an
//! `event_type`, a nullable `metadata` blob, the canonical `tags` encoding,
//! ADR-0014's `origin_store` and `origin_position`, and a `recorded_at`. What
//! has to be measured is the *subtraction*.
//!
//! **Per-value, not cumulative.** `SqlError::StorageLimitExceeded` is the
//! object's total SQL storage cap and is a function of everything already
//! stored; `MAX_EVENT_DATA_LEN` is a per-value boundary the conformance rule
//! probes against a store earlier rules have already written to. Declaring the
//! first as the second is CF-40's own named falsifier — *a real adapter whose
//! ceiling is not a constant* — so every search below runs on a **fresh** object
//! and M5 re-runs one of them against a pre-loaded one.

#![cfg(target_arch = "wasm32")]

use core::future::poll_fn;

use futures_core::Stream;
use happenstance_cloudflare::host::DurableObjectHost;
use happenstance_cloudflare::{CloudflareEventStore, SqlStorage, SqlValue};
use happenstance_core::{AppendError, Event, EventStore, Query, ReadOptions, StoreLimit, Tag, Tags};
use wasm_bindgen_test::{console_log, wasm_bindgen_test};

/// Cloudflare's documented maximum row size for a Durable Object's SQL storage.
///
/// The **seed** of the payload search, never its result.
const DOCUMENTED_ROW_BYTES: usize = 2 * 1024 * 1024;

/// Where phase A stops looking. Four times the documented row cap: past this a
/// search is measuring the harness's memory rather than the store's ceiling, and
/// EC-007 says a number obtained that way is not to be declared.
const SEARCH_CAP: usize = DOCUMENTED_ROW_BYTES * 4;

/// One fresh Durable Object, migrated, reached through the one constructor.
fn open() -> (DurableObjectHost, CloudflareEventStore) {
    let host = DurableObjectHost::new();
    let store = CloudflareEventStore::new(host.storage());
    store.migrate().expect("the schema applies");
    (host, store)
}

async fn drain<S: Stream>(stream: S) -> Vec<S::Item> {
    let mut stream = Box::pin(stream);
    let mut out = Vec::new();
    while let Some(item) = poll_fn(|cx| stream.as_mut().poll_next(cx)).await {
        out.push(item);
    }
    out
}

/// A repeating non-zero pattern, so a column that stores only a length — or a
/// driver that treats a run of NULs as a terminator — cannot pass.
fn filler(len: usize) -> Vec<u8> {
    (0..len)
        .map(|byte| u8::try_from(byte % 251).unwrap_or(0))
        .collect()
}

fn tags_of(count: usize) -> Tags {
    (0..count)
        .map(|n| Tag::new(format!("probe-key-{n:06}=probe-value-{n:06}")).expect("a valid tag"))
        .collect()
}

/// Phase A's one statement: the adapter's own `event` insert, issued directly.
///
/// Column-for-column what `CloudflareEventStore::append` renders, so what this
/// measures is the runtime under *this adapter's schema* rather than SQLite in
/// the abstract. What it does not carry is the adapter's ceiling check, which is
/// the entire point.
fn insert_row_directly(sql: &SqlStorage, payload: &[u8]) -> bool {
    sql.exec(
        "INSERT INTO event (event_type, data, metadata, tags, recorded_at) \
         VALUES (?, ?, ?, ?, ?)",
        &[
            SqlValue::Text("DirectProbe".to_owned()),
            SqlValue::Blob(payload.to_vec()),
            SqlValue::Null,
            SqlValue::Blob(vec![0x1f]),
            SqlValue::Integer(0),
        ],
    )
    .is_ok()
}

/// Reads the newest payload back out, so a store that accepted and truncated
/// cannot pass an append-only search.
fn stored_payload_len(sql: &SqlStorage) -> Option<usize> {
    let mut cursor = sql
        .exec("SELECT data FROM event ORDER BY position DESC", &[])
        .ok()?;
    match cursor.next_row()?.ok()?.values() {
        [SqlValue::Blob(bytes)] => Some(bytes.len()),
        _ => None,
    }
}

/// M1 — the bytes an `event` row costs *besides* its payload.
///
/// Two appends onto a fresh object and the object's own `databaseSize` between
/// them. `databaseSize` is page-granular, so this is read as a bound rather than
/// as a byte count, and every declaration treats it that way.
#[wasm_bindgen_test]
async fn m1_the_row_overhead_this_adapter_adds_beyond_the_payload() {
    let (host, store) = open();
    let sql = host.storage();

    let before = sql.database_size();
    let empty = Event::new("OverheadProbe".repeat(19), Vec::new())
        .expect("a valid event type")
        .with_tags(tags_of(8));
    store
        .append(core::slice::from_ref(&empty), None)
        .await
        .expect("the empty-payload append lands");
    let after_empty = sql.database_size();

    let sized = Event::new("OverheadProbe".repeat(19), filler(64 * 1024))
        .expect("a valid event type")
        .with_tags(tags_of(8));
    store
        .append(core::slice::from_ref(&sized), None)
        .await
        .expect("the 64 KiB append lands");
    let after_sized = sql.database_size();

    console_log!("M1 databaseSize before any append: {before}");
    console_log!("M1 after one empty-payload event (type 247 B, 8 tags): {after_empty}");
    console_log!("M1 after one 64 KiB-payload event (same shape): {after_sized}");
    console_log!(
        "M1 overhead bound for the empty row: {} bytes",
        after_empty.saturating_sub(before)
    );
    console_log!(
        "M1 marginal cost of 64 KiB of payload: {} bytes",
        after_sized.saturating_sub(after_empty)
    );
    console_log!(
        "M1 documented Durable Object row cap: {DOCUMENTED_ROW_BYTES} bytes (SEED, not a result)"
    );
}

/// M2A — phase A: where the **runtime** stops accepting a payload.
#[wasm_bindgen_test]
fn m2a_the_runtime_payload_boundary() {
    let mut accepted = 0_usize;
    let mut refused = None;
    let mut candidate = 64 * 1024_usize;

    while candidate <= SEARCH_CAP {
        let (host, _store) = open();
        let sql = host.storage();
        let payload = filler(candidate);
        if insert_row_directly(&sql, &payload) && stored_payload_len(&sql) == Some(candidate) {
            accepted = candidate;
            candidate *= 2;
        } else {
            refused = Some(candidate);
            break;
        }
    }

    console_log!("M2A runtime: largest payload accepted and read back: {accepted} bytes");
    match refused {
        Some(at) => console_log!("M2A runtime: first payload refused or corrupted: {at} bytes"),
        None => console_log!(
            "M2A runtime: NO refusal observed at or below {SEARCH_CAP} bytes. FINDING, not a \
             ceiling: the physical wall is outside the searchable range on this host, so a \
             declared ceiling is a policy this adapter keeps rather than a wall the runtime \
             imposes."
        ),
    }
}

/// M2B — phase B: where the **adapter** stops, and how it says so.
#[wasm_bindgen_test]
async fn m2b_the_declared_payload_boundary_is_kept_in_both_directions() {
    let declared = 1024 * 1024_usize;
    let (_host, store) = open();

    let at = Event::new("AtTheCeiling".to_owned(), filler(declared)).expect("a valid event type");
    let accepted = store.append(core::slice::from_ref(&at), None).await.is_ok();
    let read_back = drain(store.read(&Query::all(), ReadOptions::new()))
        .await
        .first()
        .and_then(|item| item.as_ref().ok().map(|item| item.event.data().len()));

    let over =
        Event::new("OverTheCeiling".to_owned(), filler(declared + 1)).expect("a valid event type");
    let refusal = store.append(core::slice::from_ref(&over), None).await;

    console_log!("M2B adapter: a {declared}-byte payload is accepted: {accepted}");
    console_log!("M2B adapter: and reads back as {read_back:?} bytes");
    console_log!(
        "M2B adapter: {} bytes is refused as ExceedsStoreLimit(EventDataLen): {}",
        declared + 1,
        matches!(
            refusal,
            Err(AppendError::ExceedsStoreLimit {
                limit: StoreLimit::EventDataLen,
                ..
            })
        )
    );
}

/// M3 — the tag boundary, both phases.
#[wasm_bindgen_test]
async fn m3_the_tag_boundary() {
    let mut accepted = 0_usize;
    let mut refused = None;
    for candidate in [64_usize, 128, 256, 512, 1024, 2048, 4096, 8192, 16384] {
        let (host, _store) = open();
        let sql = host.storage();
        assert!(insert_row_directly(&sql, b"x"), "the anchor row lands");

        let mut ok = true;
        for n in 0..candidate {
            if sql
                .exec(
                    "INSERT INTO event_tag (tag, position, event_type) VALUES (?, ?, ?)",
                    &[
                        SqlValue::Text(format!("k{n:06}=v{n:06}")),
                        SqlValue::Integer(1),
                        SqlValue::Text("DirectProbe".to_owned()),
                    ],
                )
                .is_err()
            {
                ok = false;
                break;
            }
        }
        if ok {
            accepted = candidate;
        } else {
            refused = Some(candidate);
            break;
        }
    }
    console_log!("M3A runtime: largest tag-row count written directly: {accepted}");
    match refused {
        Some(at) => console_log!("M3A runtime: first tag-row count refused: {at}"),
        None => console_log!(
            "M3A runtime: NO refusal observed at or below 16384 tag rows. FINDING, not a ceiling."
        ),
    }

    let declared = 1024_usize;
    let (host, store) = open();
    let sql = host.storage();
    let before = sql.database_size();
    let at = Event::new("TagsAtTheCeiling".to_owned(), Vec::new())
        .expect("a valid event type")
        .with_tags(tags_of(declared));
    let accepted_at = store.append(core::slice::from_ref(&at), None).await.is_ok();
    let cost = sql.database_size().saturating_sub(before);

    let over = Event::new("TagsOverTheCeiling".to_owned(), Vec::new())
        .expect("a valid event type")
        .with_tags(tags_of(declared + 1));
    let refusal = store.append(core::slice::from_ref(&over), None).await;

    let mut cursor = sql
        .exec("SELECT count(*) AS n FROM event_tag", &[])
        .expect("the count runs");
    let rows = match cursor.next_row() {
        Some(Ok(row)) => match row.values() {
            [SqlValue::Integer(n)] => *n,
            _ => -1,
        },
        _ => -1,
    };

    console_log!("M3B adapter: {declared} tags accepted: {accepted_at}");
    console_log!("M3B adapter: event_tag rows written for that one event: {rows}");
    console_log!("M3B adapter: storage cost of that one event: {cost} bytes");
    console_log!(
        "M3B adapter: {} tags refused as ExceedsStoreLimit(TagsPerEvent): {}",
        declared + 1,
        matches!(
            refusal,
            Err(AppendError::ExceedsStoreLimit {
                limit: StoreLimit::TagsPerEvent,
                ..
            })
        )
    );
}

/// M4 — the batch boundary, both phases.
///
/// The bound here is not SQL: the adapter renders one `INSERT INTO event (…)`
/// per event and one `INSERT INTO event_tag (…)` per tag, each its own
/// statement, so no bound-parameter cap is in play. What *is* in play is that
/// the whole batch runs inside one turn with nothing awaited between rows —
/// which is what makes the compensating discard exact — so the ceiling is a
/// statement count the object has to get through before it yields.
#[wasm_bindgen_test]
async fn m4_the_batch_boundary() {
    let mut accepted = 0_usize;
    let mut refused = None;
    for candidate in [128_usize, 256, 512, 1024, 2048, 4096, 8192] {
        let (host, _store) = open();
        let sql = host.storage();

        let mut ok = true;
        for _ in 0..candidate {
            if !insert_row_directly(&sql, b"batch") {
                ok = false;
                break;
            }
        }
        if ok {
            accepted = candidate;
        } else {
            refused = Some(candidate);
            break;
        }
    }
    console_log!("M4A runtime: largest run of event inserts in one turn: {accepted}");
    match refused {
        Some(at) => console_log!("M4A runtime: first run refused: {at}"),
        None => console_log!(
            "M4A runtime: NO refusal observed at or below 8192 inserts. FINDING, not a ceiling."
        ),
    }

    let declared = 1024_usize;
    let (host, store) = open();
    let sql = host.storage();
    let batch: Vec<Event> = (0..declared)
        .map(|n| {
            Event::new("BatchAtTheCeiling".to_owned(), filler(64))
                .expect("a valid event type")
                .with_tags(
                    core::iter::once(Tag::new(format!("n={n:06}")).expect("a valid tag")).collect(),
                )
        })
        .collect();
    let accepted_at = store.append(&batch, None).await.is_ok();
    let statements = happenstance_cloudflare::host::statements(&sql).len();

    let over: Vec<Event> = (0..=declared)
        .map(|_| Event::new("BatchOverTheCeiling".to_owned(), Vec::new()).expect("a valid type"))
        .collect();
    let refusal = store.append(&over, None).await;

    console_log!("M4B adapter: a {declared}-event batch is accepted: {accepted_at}");
    console_log!("M4B adapter: statements issued for it (1 tag each): {statements}");
    console_log!(
        "M4B adapter: {} events refused as ExceedsStoreLimit(EventsPerBatch): {}",
        declared + 1,
        matches!(
            refusal,
            Err(AppendError::ExceedsStoreLimit {
                limit: StoreLimit::EventsPerBatch,
                ..
            })
        )
    );
}

/// M5 — EC-002 and EC-005: is the payload boundary a **constant**, or a function
/// of what is already stored?
///
/// Phase A, necessarily: the adapter's own ceiling would answer this question
/// for the runtime and hide whatever the runtime was going to say. If a payload
/// the runtime accepts on a fresh object is refused on a pre-loaded one, the cap
/// is cumulative, CF-40's own falsifier has fired on one of its three named
/// instruments, and the honest declaration for that limit is `None` plus a
/// written finding — never a number that happens to pass on one ordering of the
/// suite.
#[wasm_bindgen_test]
fn m5_the_payload_boundary_is_a_constant_not_a_function_of_what_is_stored() {
    let (host, _store) = open();
    let sql = host.storage();
    let fresh = insert_row_directly(&sql, &filler(DOCUMENTED_ROW_BYTES));

    let (ballast, _ballast_store) = open();
    let ballast_sql = ballast.storage();
    for _ in 0..16 {
        assert!(
            insert_row_directly(&ballast_sql, &filler(64 * 1024)),
            "the ballast lands"
        );
    }
    let after_ballast = insert_row_directly(&ballast_sql, &filler(DOCUMENTED_ROW_BYTES));

    console_log!("M5 a {DOCUMENTED_ROW_BYTES}-byte row onto a fresh object: {fresh}");
    console_log!(
        "M5 the same row onto an object already holding 1 MiB: {after_ballast} (equal means the \
         boundary is a constant; unequal means the cap is cumulative and CF-40's falsifier has \
         fired)"
    );
}
