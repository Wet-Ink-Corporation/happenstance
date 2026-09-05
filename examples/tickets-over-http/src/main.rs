//! One image, three roles, and two of them run at once in separate processes.
//!
//! ```console
//! cargo run -p tickets-over-http                     # the demonstration
//! cargo run -p tickets-over-http -- serve   <db>     # just the HTTP API
//! cargo run -p tickets-over-http -- project <db>     # just the view's runner
//! ```
//!
//! The demonstration starts the other two roles as **child processes** and
//! drives them over a TCP connection; `src/demo.rs` says what it makes them do
//! and what it does and does not assert about the result.
//!
//! # Why the role is an argument and not a second binary
//!
//! Three `[[bin]]` targets would read more directly, and it was written that
//! way first. `cargo run` builds only the binary it is about to run, so the
//! demonstration's first spawn failed with *"the system cannot find the file
//! specified"* until somebody had run `cargo build` — a first-run failure in
//! the one example whose whole point is that it starts other processes. A role
//! argument and `env::current_exe()` has no such state. It is also what a great
//! many deployed services do: one image, one entry point, several roles chosen
//! by the orchestrator.
//!
//! # Why the runtime is built by hand
//!
//! `#[tokio::main]` would have to pick one flavour for all three roles, and the
//! right flavour is not the same for each. The API wants
//! `rt-multi-thread`, because contended requests being served in *parallel* is
//! what this example is for and cooperative interleaving would make the
//! demonstration depend on where the await points fell. The runner is a single
//! sequential loop with no reason to own a thread pool. Spelling the choice out
//! is two lines and says which is which.

mod api;
mod demo;
mod runner;

use std::path::PathBuf;

use anyhow::{Context as _, Result, bail};

/// The argument that selects the HTTP API.
pub(crate) const SERVE: &str = "serve";

/// The argument that selects the projection runner.
pub(crate) const PROJECT: &str = "project";

/// The argument that selects the demonstration, and the default.
pub(crate) const DEMO: &str = "demo";

fn main() -> Result<()> {
    let mut arguments = std::env::args().skip(1);
    let role = arguments.next().unwrap_or_else(|| DEMO.to_owned());

    match role.as_str() {
        SERVE => in_parallel(api::run(database(arguments.next(), SERVE)?)),
        PROJECT => sequentially(runner::run(database(arguments.next(), PROJECT)?)),
        DEMO => in_parallel(demo::run()),
        other => bail!("`{other}` is not a role: expected `{DEMO}`, `{SERVE}` or `{PROJECT}`"),
    }
}

/// The database path a role was given.
fn database(argument: Option<String>, role: &str) -> Result<PathBuf> {
    argument
        .map(PathBuf::from)
        .with_context(|| format!("usage: tickets-over-http {role} <database path>"))
}

/// Runs `work` on a runtime with a thread pool under it.
///
/// The API and the demonstration both need it: one serves contended requests
/// in parallel, and the other has fourteen clients in flight at once.
fn in_parallel<F: Future<Output = Result<()>>>(work: F) -> Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("could not build a multi-threaded runtime")?
        .block_on(work)
}

/// Runs `work` on a single-threaded runtime.
///
/// `enable_all` and not just the timer, because the SQLite adapter's read
/// stream defers its work into `spawn_blocking`: `current_thread` chooses the
/// *driver*, and a blocking pool exists under it either way.
fn sequentially<F: Future<Output = Result<()>>>(work: F) -> Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("could not build a current-thread runtime")?
        .block_on(work)
}
