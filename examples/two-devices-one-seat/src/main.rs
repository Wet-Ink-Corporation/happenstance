//! Tells the story on three fresh files and removes them afterwards.
//!
//! ```console
//! cargo run -p two-devices-one-seat
//! ```
//!
//! Everything is in the library; this is the edge, where `anyhow` adds the
//! context a person reading a failure needs, and where stdout is locked once
//! and written to with `writeln!`, which reports a closed pipe as an error
//! instead of panicking the way `println!` does.

use std::path::{Path, PathBuf};
use std::{env, fs, io, process};

use anyhow::{Context as _, Result};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let dir = scratch_dir();
    fresh(&dir)?;
    two_devices_one_seat::tell(&dir, &mut io::stdout().lock())
        .await
        .context("telling the story")?;
    fs::remove_dir_all(&dir).with_context(|| format!("removing {}", dir.display()))?;
    Ok(())
}

/// Where this run's three files live: under the process id, so a `cargo run`
/// and the test that spawns this binary never meet on one file. Not printed,
/// so the transcript is the same on every run.
fn scratch_dir() -> PathBuf {
    env::temp_dir().join(format!("happenstance-two-devices-{}", process::id()))
}

/// Makes `dir` empty: whatever a crashed earlier run with the same process id
/// left behind, a log's `-wal` and `-shm` included, is removed first.
fn fresh(dir: &Path) -> Result<()> {
    match fs::remove_dir_all(dir) {
        Err(error) if error.kind() != io::ErrorKind::NotFound => {
            return Err(error).with_context(|| format!("clearing {}", dir.display()));
        }
        _ => {}
    }
    fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))
}
