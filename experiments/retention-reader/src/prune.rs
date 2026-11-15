//! Removal outside the port: a raw `DELETE` on a closed SQLite file, as an
//! operator pruning a replica would do it.
//!
//! Why SQLite and not the instrument: only a physically pruned store re-admits
//! a re-delivered event, because ingest deduplication lives in the adapter's
//! `UNIQUE (origin_store, origin_position)`, and an unheld own identity is
//! detected only when the row lands. `AUTOINCREMENT` means the `DELETE` leaves
//! ES-38 intact: the pruned file is a legal deleted-from store.

use core::fmt;
use core::num::TryFromIntError;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::{fs, process};

use happenstance_core::SequencePosition;
use happenstance_sqlite::rusqlite::{self, Connection};

/// A directory under the system temporary directory, unique to this process
/// and scenario, holding one SQLite file.
///
/// Remove it with [`finish`](Self::finish), which reports a failure; dropping
/// it unfinished removes it too, and can only write a failure to stderr.
#[derive(Debug)]
pub struct TempDb {
    /// `None` once [`finish`](Self::finish) or `Drop` has removed it.
    dir: Option<PathBuf>,
    /// The SQLite file inside `dir`, fixed at construction.
    file: PathBuf,
}

impl TempDb {
    /// Creates `retention-reader-<pid>-<scenario>` under the temporary
    /// directory, emptying it first if a previous run left it behind.
    ///
    /// # Errors
    ///
    /// The filesystem refused to remove or create the directory.
    pub fn new(scenario: &str) -> io::Result<Self> {
        let dir =
            std::env::temp_dir().join(format!("retention-reader-{}-{scenario}", process::id()));
        if dir.exists() {
            fs::remove_dir_all(&dir)?;
        }
        fs::create_dir_all(&dir)?;
        Ok(Self {
            file: dir.join("store.sqlite"),
            dir: Some(dir),
        })
    }

    /// The SQLite file's path. It does not exist until a store opens it.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.file
    }

    /// Removes the directory and everything in it.
    ///
    /// # Errors
    ///
    /// The filesystem refused to remove it.
    pub fn finish(mut self) -> io::Result<()> {
        match self.dir.take() {
            Some(dir) => fs::remove_dir_all(dir),
            None => Ok(()),
        }
    }
}

impl Drop for TempDb {
    fn drop(&mut self) {
        let Some(dir) = self.dir.take() else {
            return;
        };
        if let Err(error) = fs::remove_dir_all(&dir) {
            // `Drop` cannot return the failure, so stderr is the last place to
            // report it. If stderr is closed too there is nowhere left, and
            // that write's own result is the one thing deliberately dropped.
            drop(writeln!(
                io::stderr().lock(),
                "retention-reader: could not remove {}: {error}",
                dir.display()
            ));
        }
    }
}

/// Why [`prune_outside_port`] removed nothing.
#[derive(Debug)]
pub enum PruneError {
    /// The driver refused a statement. The transaction rolled back.
    Sqlite(rusqlite::Error),
    /// A position does not fit SQLite's signed 64-bit rowid.
    Position(TryFromIntError),
}

impl fmt::Display for PruneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(_) => f.write_str("the out-of-port prune failed in SQLite"),
            Self::Position(_) => f.write_str("a pruned position does not fit a SQLite rowid"),
        }
    }
}

impl core::error::Error for PruneError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Sqlite(error) => Some(error),
            Self::Position(error) => Some(error),
        }
    }
}

impl From<rusqlite::Error> for PruneError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Sqlite(error)
    }
}

/// Deletes the rows at `positions` from the SQLite event store at `path`,
/// outside the port, in one transaction, and returns how many events went.
///
/// The store must not be open: the caller drops every handle first and
/// reopens afterwards, so there is no second writer.
///
/// The schema is `happenstance-sqlite`'s migrations 1 and 2
/// (`crates/happenstance-sqlite/src/event_store.rs`, `MIGRATION_1`;
/// `event_store/write_group.rs`, `MIGRATION_2`). `event_tag.position`
/// references `event(position)`, so tag rows go first; migration 2 adds
/// columns only. The planner's `tag_cardinality` hint is recounted afterwards.
///
/// # Errors
///
/// [`PruneError`]; nothing was deleted.
pub fn prune_outside_port(
    path: &Path,
    positions: &[SequencePosition],
) -> Result<usize, PruneError> {
    let mut connection = Connection::open(path)?;
    let transaction = connection.transaction()?;
    let mut removed = 0_usize;
    for position in positions {
        let rowid = i64::try_from(position.get()).map_err(PruneError::Position)?;
        transaction.execute("DELETE FROM event_tag WHERE position = ?1", [rowid])?;
        removed = removed
            .saturating_add(transaction.execute("DELETE FROM event WHERE position = ?1", [rowid])?);
    }
    transaction.execute(
        "UPDATE tag_cardinality SET events = \
         (SELECT count(*) FROM event_tag WHERE event_tag.tag = tag_cardinality.tag)",
        [],
    )?;
    transaction.commit()?;
    Ok(removed)
}

#[cfg(test)]
mod tests {
    use happenstance_core::{Event, EventStore, Query, ReadOptions, collect};
    use happenstance_sqlite::event_store::SqliteEventStore;
    use happenstance_testkit::block_on;

    use super::{TempDb, prune_outside_port};
    use crate::group_read::GroupRead;

    #[test]
    fn prune_outside_port_deletes_rows_and_the_store_reopens_conformant()
    -> Result<(), Box<dyn core::error::Error>> {
        let db = TempDb::new("prune-unit")?;
        block_on(async {
            let store = SqliteEventStore::open(db.path())?;
            for data in ["1", "2", "3", "4"] {
                let event = Event::new("Written", data.as_bytes())?;
                store.append(&[event], None).await?;
            }
            let before = collect(
                GroupRead::new(&store, store.store_id()).read(&Query::all(), ReadOptions::new()),
            )
            .await?;
            drop(store);

            let gone: Vec<_> = before.iter().skip(1).step_by(2).collect();
            let positions: Vec<_> = gone.iter().map(|e| e.position).collect();
            assert_eq!(prune_outside_port(db.path(), &positions)?, 2);

            let store = SqliteEventStore::open(db.path())?;
            let after = collect(
                GroupRead::new(&store, store.store_id()).read(&Query::all(), ReadOptions::new()),
            )
            .await?;
            let kept: Vec<_> = before.iter().step_by(2).map(|e| e.id).collect();
            assert_eq!(after.iter().map(|e| e.id).collect::<Vec<_>>(), kept);
            for event in &gone {
                assert!(!store.contains_event_id(event.id).await?);
            }
            // ES-38: the next position is above every one ever assigned.
            let next = store
                .append(&[Event::new("Written", &b"5"[..])?], None)
                .await?;
            assert!(before.iter().all(|e| e.position < next));
            drop(store);
            Ok::<(), Box<dyn core::error::Error>>(())
        })?;
        db.finish()?;
        Ok(())
    }
}
