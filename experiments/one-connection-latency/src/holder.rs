//! A second connection onto the same file, holding `BEGIN IMMEDIATE`.
//!
//! This is the contention every arm of instrument (a) is measured under, and it
//! is the shape the finding names: *"from a plain `std::thread`, open a second
//! connection onto the same file and hold `BEGIN IMMEDIATE`"*. It is a bare OS
//! thread on purpose — a tokio task would be scheduled by the very reactor the
//! measurement is about.
//!
//! The connection is opened through
//! `happenstance_sqlite::connection::open_configured`, the shipped adapter's own
//! door, so the second connection runs under the same WAL, the same
//! `synchronous` and the same 5,000 ms busy timeout as the first. Opening it any
//! other way would measure a configuration the adapter does not ship.
//!
//! # What it does and does not block
//!
//! Under WAL a writer does **not** block readers. So this holder blocks
//! `BEGIN IMMEDIATE` — which is what `append` opens — and it does *not* block
//! `SELECT max(position)`, which is what `head` and `sample_ceiling` run. That
//! distinction is the whole reason instrument (a) has two contention shapes
//! rather than one, and getting it wrong would have produced a table of zeroes
//! for the two read arms and a wrong conclusion about them.

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::time::Duration;

/// A live `BEGIN IMMEDIATE` on a second connection, for a stated duration.
#[derive(Debug)]
pub struct WriteLockHolder {
    thread: Option<JoinHandle<()>>,
}

impl WriteLockHolder {
    /// Takes the file write lock for `hold`, and returns once it is **held**.
    ///
    /// Returning only after the lock is taken is what makes the arm that
    /// follows it deterministic: a holder that returned before its `BEGIN
    /// IMMEDIATE` landed would produce an append that sometimes waits and
    /// sometimes does not, and a maximum over such runs is a number about the
    /// scheduler.
    ///
    /// # Panics
    ///
    /// Panics if the second connection cannot be opened or the transaction
    /// cannot be started — both mean the measurement cannot be taken, and a
    /// silently un-held lock would report the floor as if it were the stall.
    #[must_use]
    pub fn hold(path: impl AsRef<Path>, hold: Duration) -> Self {
        let path: PathBuf = path.as_ref().to_path_buf();
        let (ready, held) = mpsc::channel();

        let thread = std::thread::spawn(move || {
            let connection = happenstance_sqlite::connection::open_configured(&path)
                .unwrap_or_else(|err| panic!("second connection onto {path:?} failed: {err}"));
            connection
                .execute_batch("BEGIN IMMEDIATE")
                .unwrap_or_else(|err| panic!("BEGIN IMMEDIATE on {path:?} failed: {err}"));
            ready.send(()).ok();
            std::thread::sleep(hold);
            connection
                .execute_batch("COMMIT")
                .unwrap_or_else(|err| panic!("COMMIT on {path:?} failed: {err}"));
        });

        held.recv().expect("the write-lock holder thread died");
        Self {
            thread: Some(thread),
        }
    }
}

impl Drop for WriteLockHolder {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            // Joining rather than detaching: the next arm opens its own holder
            // on the same file, and a still-live `BEGIN IMMEDIATE` from the
            // previous arm would show up inside it as contention nobody
            // declared.
            thread.join().expect("the write-lock holder thread panicked");
        }
    }
}
