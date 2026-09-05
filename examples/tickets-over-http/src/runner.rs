//! The read side: a process whose only job is to keep one view current.
//!
//! It shares nothing with `tickets-api` but a file. It never accepts a
//! connection, never takes a decision and never appends an event; it reads the
//! log, folds it, and commits the rows and the checkpoint together. Stopping
//! it stops the view from advancing and stops nothing else — which is the
//! property the whole example exists to show, and the one a single-process
//! program cannot demonstrate no matter how it is arranged.
//!
//! Run it directly with `cargo run -p tickets-over-http -- project <database>`.
//! It runs until it is stopped.

#![allow(clippy::print_stdout, reason = "the progress log is the interface")]

use core::time::Duration;
use std::path::PathBuf;

use anyhow::Result;
use happenstance::{Json, run_projection};
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_sqlite::projection_store::SqliteProjectionStore;
use tickets_over_http::{CHUNK, SeatHolders, ensure_schema};

/// How long to wait after a pass that had nothing to do.
///
/// Polling, and it is worth being honest about why: `EventStore` has no
/// subscription in its port, so a runner learns that the log grew by looking.
/// Short enough that a demonstration does not spend its time asleep, long
/// enough that an idle runner is not a spin loop competing with the API for
/// the same file's write lock.
const IDLE: Duration = Duration::from_millis(20);

/// Keeps the view current until it is stopped.
pub(crate) async fn run(path: PathBuf) -> Result<()> {
    ensure_schema(&path)?;

    // Built inside the runtime for the reason the API gives: ADR-0022 §9
    // captures the tokio handle at construction.
    let events = SqliteEventStore::open(&path)?;
    let models = SqliteProjectionStore::open(&path)?;
    let mut view = SeatHolders::new();
    let chunk = CHUNK.try_into()?;

    println!("runner started against {}", path.display());

    loop {
        match run_projection(&events, &models, &mut view, &Json, chunk).await {
            Ok(progress) => {
                if progress.applied > 0 {
                    println!(
                        "applied {} event(s), through {:?}",
                        progress.applied, progress.through
                    );
                }
            }
            // A failure here is reported and retried rather than fatal. The
            // most likely one by far is `SQLITE_BUSY` surviving the busy
            // timeout while the API holds the write lock, and a runner that
            // exited on it would turn a moment of contention into an operator
            // page. A *decode* failure would be permanent and would spin —
            // which is why `rebuilding-read-models` exists to show what a
            // policy for that looks like, and why this one says out loud that
            // it does not have one.
            Err(err) => eprintln!("projection pass failed: {err}"),
        }

        tokio::time::sleep(IDLE).await;
    }
}
