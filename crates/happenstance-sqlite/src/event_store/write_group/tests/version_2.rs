//! The version-2 schema, as migration 2 leaves a file.
//!
//! Until migration 3 (#483, #484) the public open path stopped here, and
//! `tests/migration.rs` held these assertions against it. Every public open
//! now goes on to version 3, so the version-2 half of those tests lives here,
//! where `migrate`'s own steps can be composed and stopped after migration 2:
//! `apply_baseline`, `write_group::migrate_to_2` and `mint_identity`, in
//! `migrate`'s order, with nothing of migration 3. Each test names the
//! integration test it came from; those now assert version 3 on the public
//! path.

use std::collections::BTreeSet;
use std::sync::{Arc, Barrier};

use happenstance_core::StoreId;
use rusqlite::Connection;

use super::super::{WRITE_GROUP_SCHEMA_VERSION, migrate_to_2};
use crate::event_store::{SCHEMA_VERSION_KEY, apply_baseline, mint_identity};
use crate::test_support::TempFile;

/// `migrate` stopped after migration 2: the same steps in the same
/// `BEGIN IMMEDIATE`, and the incarnation read back inside it. Read raw, not
/// through `read_identity`, which refuses any version but the current one.
fn migrate_to_version_2(connection: &mut Connection) -> StoreId {
    let transaction = connection
        .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
        .expect("the migration takes the write lock");
    apply_baseline(&transaction).expect("migration 1 applies");
    migrate_to_2(&transaction).expect("migration 2 applies");
    mint_identity(&transaction).expect("the incarnation is minted");
    let raw: Vec<u8> = transaction
        .query_row(
            "SELECT CAST(v AS BLOB) FROM store_meta WHERE k = 'store_id'",
            [],
            |row| row.get(0),
        )
        .expect("the incarnation reads back");
    transaction.commit().expect("the migration commits");
    StoreId::from_bytes(raw.try_into().expect("an incarnation is sixteen bytes"))
}

/// The marker the file carries.
fn marker(connection: &Connection) -> i64 {
    connection
        .query_row(
            "SELECT CAST(v AS INTEGER) FROM store_meta WHERE k = ?",
            [SCHEMA_VERSION_KEY],
            |row| row.get(0),
        )
        .expect("the marker reads")
}

/// `sqlite_master`'s `(type, name)` pairs, SQLite's own objects filtered out,
/// as `tests/migration.rs::schema_objects` reads them.
fn schema_objects(connection: &Connection) -> BTreeSet<(String, String)> {
    let mut statement = connection
        .prepare("SELECT type, name FROM sqlite_master ORDER BY type, name")
        .expect("sqlite_master reads");
    let rows = statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .expect("sqlite_master reads");
    rows.collect::<Result<BTreeSet<_>, _>>()
        .expect("every row reads")
        .into_iter()
        .filter(|(_, name)| !name.starts_with("sqlite_"))
        .collect()
}

/// The names of `table`'s columns, in declared order.
fn column_names(connection: &Connection, table: &str) -> Vec<String> {
    let mut statement = connection
        .prepare("SELECT name FROM pragma_table_info(?) ORDER BY cid")
        .expect("the table info reads");
    let rows = statement
        .query_map([table], |row| row.get(0))
        .expect("the table info reads");
    rows.collect::<Result<_, _>>().expect("every column reads")
}

/// A file as a version-1 build left it: `tests/migration.rs::version_1_file`'s
/// text, migration 1 as published at `0.2.0`, which is frozen.
fn version_1_file(connection: &Connection) {
    connection
        .execute_batch(
            "CREATE TABLE event (
                position        INTEGER PRIMARY KEY AUTOINCREMENT,
                event_type      TEXT    NOT NULL,
                data            BLOB    NOT NULL,
                metadata        BLOB,
                tags            BLOB    NOT NULL,
                origin_store    BLOB,
                origin_position INTEGER,
                recorded_at     INTEGER NOT NULL,
                UNIQUE (origin_store, origin_position)
            );
            CREATE INDEX event_type_idx ON event(event_type, position);
            CREATE TABLE event_tag (
                tag        TEXT    NOT NULL,
                position   INTEGER NOT NULL REFERENCES event(position),
                event_type TEXT    NOT NULL,
                PRIMARY KEY (tag, position)
            ) WITHOUT ROWID;
            CREATE TABLE tag_cardinality (
                tag    TEXT    PRIMARY KEY,
                events INTEGER NOT NULL
            ) WITHOUT ROWID;
            CREATE TABLE store_meta (
                k TEXT PRIMARY KEY,
                v BLOB NOT NULL
            ) WITHOUT ROWID;
            INSERT INTO store_meta (k, v) VALUES ('schema_version', 1);
            INSERT INTO store_meta (k, v) VALUES ('store_id', X'0123456789ABCDEF0123456789ABCDEF');
            INSERT INTO event (event_type, data, metadata, tags, origin_store, origin_position, recorded_at)
                VALUES ('Legacy', X'7B7D', NULL, X'1F', X'0123456789ABCDEF0123456789ABCDEF', 1, 0);",
        )
        .expect("the version-1 file is written");
}

/// The version-2 half of `tests/migration.rs::migration_creates_the_amended_schema`:
/// a fresh file at version 2 holds exactly migration 1's five objects, because
/// migration 2 adds columns and no object, and its marker reads 2.
#[test]
fn version_2_creates_the_amended_schema() {
    let file = TempFile::new("amended-schema");
    let mut connection = file.inspect();
    migrate_to_version_2(&mut connection);

    let expected: BTreeSet<(String, String)> = [
        ("index", "event_type_idx"),
        ("table", "event"),
        ("table", "event_tag"),
        ("table", "store_meta"),
        ("table", "tag_cardinality"),
    ]
    .into_iter()
    .map(|(kind, name)| (kind.to_owned(), name.to_owned()))
    .collect();
    assert_eq!(
        schema_objects(&connection),
        expected,
        "sqlite_master after migration 2"
    );
    assert_eq!(marker(&connection), 2);
}

/// The version-2 half of
/// `tests/migration.rs::concurrent_opens_of_a_fresh_file_agree_on_one_store_id`:
/// eight migrations to version 2 of one fresh file, released together, agree
/// on one incarnation, leave one identity row, and leave the five objects.
#[test]
fn concurrent_version_2_migrations_agree_on_one_store_id() {
    let file = TempFile::new("concurrent");
    let contenders = 8;
    let barrier = Arc::new(Barrier::new(contenders));

    let ids: BTreeSet<StoreId> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..contenders)
            .map(|_| {
                // A cheap handle onto the one barrier every contender waits on.
                let barrier = Arc::clone(&barrier);
                let file = &file;
                scope.spawn(move || {
                    let mut connection = file.inspect();
                    barrier.wait();
                    migrate_to_version_2(&mut connection)
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().expect("a contender finished"))
            .collect()
    });

    assert_eq!(ids.len(), 1, "two incarnations for one file: {ids:?}");
    let connection = file.inspect();
    let identity_rows: i64 = connection
        .query_row(
            "SELECT count(*) FROM store_meta WHERE k = 'store_id'",
            [],
            |row| row.get(0),
        )
        .expect("the identity rows count");
    assert_eq!(identity_rows, 1);
    assert_eq!(schema_objects(&connection).len(), 5);
}

/// The version-2 half of
/// `tests/migration.rs::migration_2_adds_write_groups_to_a_version_1_file`:
/// migration 2 adds the write-group columns to a version-1 file, raises its
/// marker to 2, and leaves the identity and the legacy row alone. Run twice,
/// because `ADD COLUMN` has no `IF NOT EXISTS` and only the marker gate keeps
/// the second run from refusing with a duplicate column.
#[test]
fn migration_2_adds_write_groups_to_a_version_1_file() {
    let file = TempFile::new("migration-2");
    let mut connection = file.inspect();
    version_1_file(&connection);

    migrate_to_version_2(&mut connection);
    let store_id = migrate_to_version_2(&mut connection);

    assert_eq!(store_id.to_bytes()[0], 0x01, "the identity survives");
    let names = column_names(&connection, "event");
    assert!(
        names.iter().any(|name| name == "write_group")
            && names.iter().any(|name| name == "group_guard"),
        "migration 2 did not add the write-group columns; got {names:?}"
    );
    assert_eq!(marker(&connection), 2, "the marker was not raised to 2");
    assert_eq!(WRITE_GROUP_SCHEMA_VERSION, 2);
    let legacy: (i64, Option<i64>) = connection
        .query_row(
            "SELECT count(*), max(write_group) FROM event WHERE event_type = 'Legacy'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .expect("the legacy row reads");
    assert_eq!(
        legacy,
        (1, None),
        "the legacy row must survive with no write group: nothing recorded which rows one call wrote"
    );
}

/// The version-2 half of
/// `tests/migration.rs::new_refuses_a_version_1_file_until_it_is_migrated`:
/// migrating the version-1 file `new` refused leaves its marker at 2.
///
/// That test's last line, `new` accepting the migrated file, is not
/// reachable at version 2 by this build: `new` accepts only the current
/// version, 3, and refuses a version-2 file as `SchemaNeedsMigration`, which
/// `tests/migration.rs::a_version_two_file_migrates_to_three_on_open_and_keeps_its_rows`
/// asserts. So the integration test keeps that line against the version-3
/// file `open_on` now leaves.
#[test]
fn migrating_a_version_1_file_leaves_it_at_2() {
    let file = TempFile::new("new-v1");
    let mut connection = file.inspect();
    version_1_file(&connection);
    assert_eq!(marker(&connection), 1);

    migrate_to_version_2(&mut connection);

    assert_eq!(marker(&connection), 2);
}

/// The version-2 half of what
/// `tests/migration.rs::a_fresh_file_is_created_at_version_3_with_write_groups`
/// asserted as `a_fresh_file_is_created_at_version_2_with_write_groups`: a
/// fresh file takes the same path as an old one and lands at 2, with the
/// write-group column.
#[test]
fn a_fresh_file_is_created_at_version_2_with_write_groups() {
    let file = TempFile::new("fresh-v2");
    let mut connection = file.inspect();
    migrate_to_version_2(&mut connection);

    assert_eq!(marker(&connection), 2);
    let names = column_names(&connection, "event");
    assert!(
        names.iter().any(|name| name == "write_group"),
        "got {names:?}"
    );
}
