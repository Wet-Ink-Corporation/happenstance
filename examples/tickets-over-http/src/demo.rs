//! Two processes over one database file, driven over a real socket.
//!
//! `transfers-on-sqlite` says of itself, in its own module documentation, that
//! *"it is **one process**"* — and nothing else in this workspace has two. This
//! program starts an HTTP API and, later, a separate projection runner; every
//! request below crosses a TCP connection, and the two processes share nothing
//! but a file on disk.
//!
//! Run with `cargo run -p tickets-over-http`.
//!
//! # What this demonstrates that one process cannot
//!
//! 1. **Concurrency that is not arranged.** Eight clients ask for one seat at
//!    the same time, over eight connections, and the seat is sold once. Six
//!    more ask for six different seats against a capacity of five, and five
//!    seats end up sold. No barrier, no injected delay: the contention is real
//!    and the append condition is what resolves it.
//! 2. **A write that is acknowledged while the read model's process is not
//!    even running.** The reservations below are durable and answered `201`
//!    before `tickets-runner` has ever been started. The view is a separate
//!    concern with a separate lifecycle, and this is what that actually means.
//! 3. **Read-your-own-writes across an asynchronous view.** The API answers
//!    `GET /seats?at=N` with `202` and its own checkpoint until the runner has
//!    reached `N`, and `200` after. That is the failure every projection-backed
//!    API has — write, read immediately, see nothing, conclude the write was
//!    lost — handled rather than hoped away.
//!
//! # What is asserted, and what is only reported
//!
//! The seat counts are asserted: this program `bail!`s if two clients are told
//! they hold one seat, or if six seats are sold from a show with five. **How
//! many attempts the contended commits spent is reported and never asserted**,
//! because it depends on how the scheduler interleaved eight sockets on the
//! machine that ran it. `transfers-on-sqlite`'s contention test states the same
//! rule from the other side: timing is not evidence, so a claim that rests on
//! it does not get made.
//!
//! # What it does not demonstrate
//!
//! Nothing here crashes, and no process is killed mid-write. Two processes are
//! enough to show a view falling behind and catching up; they are not a
//! durability argument, and the conformance suite's reopen rule and the
//! adapter's own fault injection own that.

#![allow(clippy::print_stdout, reason = "the transcript is the point")]

use core::time::Duration;
use std::env;
use std::io::{BufRead as _, BufReader};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use anyhow::{Context as _, Result, bail};
use tickets_over_http::http::{self, Response};
use tickets_over_http::{CAPACITY, ensure_schema};

use crate::{PROJECT, SERVE};
use tokio::task::JoinSet;

/// How many clients ask for the same seat at the same time.
const CONTENDERS: usize = 8;

/// The seats the second burst asks for, one client each.
///
/// More than the show has left, so the capacity boundary refuses some of them
/// — under concurrency, which is the only interesting way to refuse anything.
const SPREAD: [&str; 6] = ["a-2", "a-3", "a-4", "a-5", "a-6", "a-7"];

/// How long to wait between asking the view whether it has caught up.
const POLL: Duration = Duration::from_millis(10);

/// How many times to ask before giving up on the runner.
///
/// A bound and not a loop, for the reason every retry count in this workspace
/// is spelled: a loop whose only exit is success is a hang with better manners.
/// At [`POLL`] this is a few seconds, which is far longer than a runner polling
/// every 20ms needs to notice seven events.
const MAX_POLLS: usize = 300;

/// Starts the other two roles, drives them over HTTP, and stops them.
pub(crate) async fn run() -> Result<()> {
    let path = database_path();
    remove_database(&path);

    println!("== the file both processes share ==");
    println!("   {}", path.display());

    let outcome = orchestrate(&path).await;
    remove_database(&path);
    outcome
}

/// Everything between the first spawn and the last assertion.
async fn orchestrate(path: &Path) -> Result<()> {
    ensure_schema(path)?;

    let mut processes = Processes::default();

    processes.start(SERVE, path, Stdio::piped())?;
    let api = processes.newest().context("the api was not started")?;
    let pid = api.id();
    let address = address_of(api)?;

    println!("\n== the write side starts, and only the write side ==");
    println!("   the `{SERVE}` role   pid {pid}, on http://{address}");
    println!("   the `{PROJECT}` role is deliberately not running yet");

    let position = race_for_one_seat(address).await?;
    fill_the_show(address).await?;
    the_view_is_not_running(address, position).await?;
    let rows = the_view_starts(&mut processes, path, address, position).await?;
    nothing_was_oversold(&rows)?;

    Ok(())
}

// ---------------------------------------------------------------------------
// The five things this program shows
// ---------------------------------------------------------------------------

/// Eight clients ask for one seat, over eight connections, at once.
///
/// Returns the position the winning reservation landed at, which the later
/// sections use to ask the view for a state that includes it.
async fn race_for_one_seat(address: SocketAddr) -> Result<u64> {
    println!("\n== {CONTENDERS} clients ask for the same seat at once ==");

    let mut clients = JoinSet::new();
    for index in 0..CONTENDERS {
        let target = format!("/reserve?seat=a-1&patron=p-{index}");
        clients.spawn(async move { http::request(address, "POST", &target).await });
    }

    let (created, conflicts) = collect(clients).await?;
    if created.len() != 1 {
        bail!(
            "{} clients were told they hold seat a-1, and a seat is held by one",
            created.len()
        );
    }
    if conflicts != CONTENDERS - 1 {
        bail!(
            "{conflicts} clients were refused, and {} asked",
            CONTENDERS - 1
        );
    }

    let winner = created.first().context("a winner that is not there")?;
    let position = header(winner, "x-happenstance-position")?;

    println!("   one 201 and {conflicts} 409s — the seat was sold exactly once");
    println!("   the winner's event landed at position {position}");
    println!(
        "   nothing here is arranged: eight sockets, no barrier, no sleep. The\n   \
         append condition is what decided it"
    );

    Ok(position)
}

/// Six clients ask for six different seats against a capacity that is smaller.
///
/// The interesting part is that these requests contend *with each other* even
/// though no two want the same seat: the show's own total is in every one of
/// their boundaries, so each append is conditioned on it, and a request that
/// loses that race re-reads and re-decides.
async fn fill_the_show(address: SocketAddr) -> Result<()> {
    println!(
        "\n== {} clients ask for {} different seats ==",
        SPREAD.len(),
        SPREAD.len()
    );

    let mut clients = JoinSet::new();
    for (index, seat) in SPREAD.iter().enumerate() {
        let target = format!("/reserve?seat={seat}&patron=q-{index}");
        clients.spawn(async move { http::request(address, "POST", &target).await });
    }

    let (created, conflicts) = collect(clients).await?;
    let remaining = usize::try_from(CAPACITY)?
        .checked_sub(1)
        .context("a capacity of zero has nothing to demonstrate")?;

    if created.len() != remaining {
        bail!(
            "{} seats were sold, and {remaining} were left",
            created.len()
        );
    }
    if conflicts != SPREAD.len() - remaining {
        bail!(
            "{conflicts} clients were refused, and {} asked",
            SPREAD.len() - remaining
        );
    }

    let mut attempts = Vec::with_capacity(created.len());
    for response in &created {
        attempts.push(header(response, "x-happenstance-attempts")?);
    }
    let retried = attempts.iter().filter(|spent| **spent > 1).count();

    println!(
        "   {} sold, {conflicts} refused as full — the show has {CAPACITY} seats",
        created.len()
    );
    println!("   attempts spent by the winners: {attempts:?}");
    println!(
        "   {retried} of them lost a race and re-decided. That count is reported\n   \
         and never asserted: it depends on how this machine interleaved six\n   \
         sockets, and timing is not evidence. What *is* asserted is the count\n   \
         of seats, which no interleaving may change"
    );

    Ok(())
}

/// Asks for a view that includes the client's own write, before one exists.
///
/// A `202` is guaranteed here rather than likely: `tickets-runner` has not
/// been started even once, so the checkpoint is `Checkpoint::NeverRun` and
/// there is no timing for this to depend on.
async fn the_view_is_not_running(address: SocketAddr, position: u64) -> Result<()> {
    println!("\n== the view's process is not running ==");

    let response = http::request(address, "GET", &format!("/seats?at={position}")).await?;
    if response.status != 202 {
        bail!(
            "GET /seats?at={position} answered {} with no runner started",
            response.status
        );
    }

    println!(
        "   GET /seats?at={position} -> 202, checkpoint {}",
        response
            .header("x-happenstance-checkpoint")
            .unwrap_or("none")
    );
    println!(
        "   every reservation above is already durable and was answered 201.\n   \
         The view is a separate process with a separate lifecycle, and this is\n   \
         what that costs a reader who wants to see their own write"
    );

    Ok(())
}

/// Starts the runner and asks until the view has reached the client's position.
async fn the_view_starts(
    processes: &mut Processes,
    path: &Path,
    address: SocketAddr,
    position: u64,
) -> Result<String> {
    println!("\n== the view's process starts ==");

    // Its own stdout goes nowhere on purpose. The runner prints a line every
    // time it applies a chunk, and letting those interleave with this
    // transcript would make the output depend on scheduling — which is the
    // thing this program is careful not to assert on anywhere else either.
    processes.start(PROJECT, path, Stdio::null())?;
    let pid = processes
        .newest()
        .context("the runner was not started")?
        .id();
    println!("   the `{PROJECT}` role pid {pid}");

    for poll in 1..=MAX_POLLS {
        let response = http::request(address, "GET", &format!("/seats?at={position}")).await?;
        match response.status {
            200 => {
                println!("   200 after {poll} poll(s): the view now includes position {position}");
                return Ok(response.body);
            }
            202 => tokio::time::sleep(POLL).await,
            status => bail!("GET /seats?at={position} answered {status}"),
        }
    }

    bail!("the view never reached position {position} after {MAX_POLLS} polls")
}

/// Checks the view against the capacity the show was opened with.
fn nothing_was_oversold(rows: &str) -> Result<()> {
    println!("\n== what the view holds ==");

    let held: Vec<&str> = rows
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    for line in &held {
        println!("   {line}");
    }

    let capacity = usize::try_from(CAPACITY)?;
    if held.len() != capacity {
        bail!(
            "the view holds {} seats, and the show has {capacity}",
            held.len()
        );
    }

    println!(
        "   {} seats held out of {capacity}. Fourteen clients asked, across two\n   \
         bursts and fourteen connections, and the count is exactly right",
        held.len()
    );

    Ok(())
}

// ---------------------------------------------------------------------------
// Talking to the API
// ---------------------------------------------------------------------------

/// Waits for every client and sorts the answers into created and refused.
///
/// Any status that is neither is a failure: a `500` or a `503` here would mean
/// the run had stopped demonstrating what it claims to, and counting it as a
/// refusal would hide that.
async fn collect(
    mut clients: JoinSet<std::io::Result<Response>>,
) -> Result<(Vec<Response>, usize)> {
    let mut created = Vec::new();
    let mut conflicts = 0_usize;
    let mut unexpected = Vec::new();

    while let Some(joined) = clients.join_next().await {
        let response = joined.context("a client task did not finish")??;
        match response.status {
            201 => created.push(response),
            409 => conflicts += 1,
            status => unexpected.push((status, response.body)),
        }
    }

    if !unexpected.is_empty() {
        bail!("statuses that are neither 201 nor 409: {unexpected:?}");
    }

    Ok((created, conflicts))
}

/// Reads a numeric header the API is supposed to have sent.
fn header(response: &Response, name: &str) -> Result<u64> {
    response
        .header(name)
        .with_context(|| format!("the response carried no `{name}`"))?
        .parse()
        .with_context(|| format!("`{name}` was not a number"))
}

// ---------------------------------------------------------------------------
// The processes this program owns
// ---------------------------------------------------------------------------

/// The children this demonstration started, stopped when it goes out of scope.
///
/// A `Drop` impl and not a `stop()` at the end of `run`, because `run` has a
/// `?` on almost every line: a failure between starting the API and the end of
/// the program would otherwise leave two processes holding a file this program
/// is about to delete, and on Windows that is a delete which silently does
/// nothing and a next run that starts on the tail of this one.
#[derive(Debug, Default)]
struct Processes(Vec<Child>);

impl Processes {
    /// Starts this program again, in another role, against `path`.
    ///
    /// `current_exe` and not a sibling binary. Three `[[bin]]` targets would
    /// have read more directly, and `cargo run` builds only the binary it is
    /// about to run — so the first spawn would fail with "the system cannot
    /// find the file specified" until somebody had run `cargo build` first.
    /// One image with a role argument has no such state, and it is what a
    /// great many deployed services do anyway.
    fn start(&mut self, role: &str, path: &Path, stdout: Stdio) -> Result<()> {
        let program = env::current_exe().context("this program cannot locate itself")?;
        let child = Command::new(program)
            .arg(role)
            .arg(path)
            .stdout(stdout)
            .spawn()
            .with_context(|| format!("could not start the `{role}` role"))?;
        self.0.push(child);
        Ok(())
    }

    /// The most recently started child.
    fn newest(&mut self) -> Option<&mut Child> {
        self.0.last_mut()
    }
}

impl Drop for Processes {
    fn drop(&mut self) {
        for child in &mut self.0 {
            drop(child.kill());
            drop(child.wait());
        }
    }
}

/// Reads the address the API printed on its first line.
///
/// The child keeps ownership of the pipe, so the reader below borrows it and
/// the API's stdout stays open afterwards. It prints nothing else, but a
/// closed pipe would turn its next `println!` into a broken-pipe panic, and a
/// server that dies because its parent stopped listening would be a defect
/// this program had introduced.
fn address_of(api: &mut Child) -> Result<SocketAddr> {
    let stdout = api
        .stdout
        .as_mut()
        .context("the api was not started with a piped stdout")?;

    let mut line = String::new();
    BufReader::new(stdout)
        .read_line(&mut line)
        .context("the api printed nothing before exiting")?;

    line.trim()
        .strip_prefix("listening on ")
        .with_context(|| format!("the api's first line was `{}`", line.trim()))?
        .parse()
        .context("the api printed an address that will not parse")
}

// ---------------------------------------------------------------------------
// The file this run owns
// ---------------------------------------------------------------------------

/// Where this run's database lives.
///
/// Under the process id, so a `cargo run` and the test that spawns this binary
/// never meet on one file.
fn database_path() -> PathBuf {
    env::temp_dir().join(format!("happenstance-tickets-{}.db", std::process::id()))
}

/// Removes the database and the two files WAL mode keeps beside it.
///
/// Deleting only the `.db` and leaving `-wal` behind is how a "fresh" run
/// starts on the tail of the last one.
fn remove_database(path: &Path) {
    for suffix in ["", "-wal", "-shm"] {
        let mut name = path.as_os_str().to_owned();
        name.push(suffix);
        drop(std::fs::remove_file(PathBuf::from(name)));
    }
}
