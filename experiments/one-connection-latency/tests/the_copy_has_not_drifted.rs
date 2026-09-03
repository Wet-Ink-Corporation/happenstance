//! A copy that has drifted from the original measures nothing.
//!
//! Two of this crate's four copied files were copied *verbatim*, and for those
//! the check is exact: re-derive them from `crates/happenstance-sqlite/src/` at
//! the live tree, apply the documented substitution, and compare. If someone
//! changes the shipped `query_sql.rs` and this crate's figures are re-quoted
//! afterwards, this is what says so.
//!
//! The third copy — `src/replica.rs` — was deliberately modified, so no textual
//! check applies to it. What stands in its place is CONTROL 2:
//! `tests/replica_is_conformant.rs` runs 89 conformance rules against it at each
//! of the four page sizes.
//!
//! The fourth thing checked here is the one number the copy has to restate
//! because it is private in the original: `PAGE_SIZE`. Every other constant is
//! *used* from the original rather than copied — `MAX_QUERY_ARMS_PER_STATEMENT`
//! and the three ceilings are public associated constants on `SqliteEventStore`,
//! and `src/replica.rs` reads them off the real type.

use std::path::{Path, PathBuf};

use happenstance_sqlite::event_store::SqliteEventStore;
use one_connection_latency::replica::SHIPPED_PAGE_SIZE;

/// The shipped adapter's source directory, relative to this manifest.
fn shipped(file: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/happenstance-sqlite/src")
        .join(file)
}

/// This crate's source directory.
fn copy(file: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join(file)
}

/// Reads a file, or explains which half of the comparison is missing.
fn read(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("reading {} failed: {err}", path.display()))
        // Git may check the originals out with CRLF on Windows while this
        // crate's copies were written with LF, or the reverse. Line endings are
        // not drift, and normalising them here is what keeps this test about the
        // code.
        .replace("\r\n", "\n")
}

/// `src/query_sql.rs` is byte-for-byte the shipped module.
///
/// Nothing in it names the event-store error type or any other crate-local item,
/// which is what makes a verbatim copy possible at all — and is the reason this
/// file, rather than a hand-written translation of the same SQL, is what the
/// measured page fetch calls.
#[test]
fn query_sql_is_byte_for_byte_the_shipped_module() {
    let original = read(&shipped("query_sql.rs"));
    let ours = read(&copy("query_sql.rs"));
    assert_eq!(
        original,
        ours,
        "src/query_sql.rs has drifted from {}. Re-copy it and re-run ./run.sh; \
         every figure in results/ was produced by the version that matched.",
        shipped("query_sql.rs").display()
    );
}

/// `src/row.rs` is the shipped module under exactly one rename.
///
/// The rename is the error type, and it is mechanical:
/// `SqliteEventStoreError` becomes `ReplicaError`, and its import moves from
/// `crate::event_store` to `crate::replica`. Nothing else may differ — the tag
/// encoding, the `COLUMNS` list and `to_event`'s three decisions are the
/// measurement's subject, not its scaffolding.
#[test]
fn row_is_the_shipped_module_under_one_rename() {
    let original = read(&shipped("row.rs"))
        .replace(
            "use crate::event_store::SqliteEventStoreError;",
            "use crate::replica::ReplicaError;",
        )
        .replace("SqliteEventStoreError", "ReplicaError");
    let ours = read(&copy("row.rs"));
    assert_eq!(
        original,
        ours,
        "src/row.rs has drifted from {} beyond the one documented rename.",
        shipped("row.rs").display()
    );
}

/// The one private constant the copy has to restate is the one the original has.
///
/// `PAGE_SIZE` is a private `const` in the shipped crate, so no compiler check
/// can tie the two together. This is the substitute: the original's source text
/// must still declare exactly the value this crate calls "shipped", because
/// every table in `results/` labels one column that way.
#[test]
fn the_shipped_page_size_is_still_the_one_this_crate_calls_shipped() {
    let source = read(&shipped("event_store.rs"));
    let declaration = format!("const PAGE_SIZE: usize = {SHIPPED_PAGE_SIZE};");
    assert!(
        source.contains(&declaration),
        "{} no longer declares `{declaration}`. results/ labels a column \
         \"shipped\" on the strength of it.",
        shipped("event_store.rs").display()
    );
}

/// The chunk width and the three ceilings are used, not copied.
///
/// This test does not defend them — the type system does, because
/// `src/replica.rs` names these very constants. It records *that* it does, so
/// that a reader of `results/` can see the difference between a number this
/// crate transcribed and a number it borrowed.
#[test]
fn the_public_constants_are_the_shipped_ones() {
    assert_eq!(SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT, 400);
    assert_eq!(SqliteEventStore::MAX_EVENTS_PER_BATCH, 256);
    assert_eq!(SqliteEventStore::MAX_TAGS_PER_EVENT, 128);
    assert_eq!(SqliteEventStore::MAX_EVENT_DATA_LEN, 1_048_576);
}
