//! The write side: an HTTP server that takes reservations and serves the view.
//!
//! It appends to the event log and it **never** maintains the read model — that
//! is `tickets-runner`'s job, in another process. What this one does with the
//! read model is the interesting half: it reads the *checkpoint* to decide
//! whether the rows it is about to serve are new enough to answer the client
//! that just wrote.
//!
//! Run it directly with `cargo run -p tickets-over-http -- serve <database>`;
//! it prints the address it bound to on its first line of output, which is how
//! the demo role finds it. Port 0, because a demonstration that fails when
//! something else on the machine holds 8080 is a demonstration about port
//! allocation.

#![allow(clippy::print_stdout, reason = "the bound address is the interface")]

use core::fmt::Write as _;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use happenstance_sqlite::connection::open_configured;
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_sqlite::projection_store::SqliteProjectionStore;
use tickets_over_http::http::{Request, respond};
use tickets_over_http::{
    CAPACITY, Reservation, checkpoint_through, ensure_schema, open_show, requested_position,
    reserve, seat_rows,
};
use tokio::net::{TcpListener, TcpStream};

/// What every connection handler needs.
///
/// The path is held as well as the two stores because the read-model table is
/// the *application's*, and `rusqlite::Connection` is `Send` but not `Sync` —
/// so a handler opens its own rather than sharing one behind a lock. That is a
/// real cost of owning your read model and it is not hidden here; a server
/// under load would keep a pool, which is a paragraph of connection management
/// that would teach nothing about happenstance.
#[derive(Debug)]
struct State {
    /// The log this process appends to.
    events: SqliteEventStore,
    /// The checkpoint store this process only ever reads.
    models: SqliteProjectionStore,
    /// The file, for opening a read-model connection per request.
    path: PathBuf,
}

/// Serves until it is stopped.
pub(crate) async fn run(path: PathBuf) -> Result<()> {
    ensure_schema(&path)?;

    // Both stores are built **inside** the runtime, and that is load-bearing
    // rather than stylistic: ADR-0022 §9 captures the tokio handle at
    // construction, so a store built before a runtime exists records `None` and
    // every read fails with `SqliteEventStoreError::NoRuntime`.
    let events = SqliteEventStore::open(&path)?;
    let models = SqliteProjectionStore::open(&path)?;

    // Opening the show is idempotent by refusal: a second process starting
    // against the same file is told the show is already open, and that is the
    // correct answer rather than an error to report.
    drop(open_show(&events, CAPACITY).await);

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).await?;
    // The first line of stdout, and a contract with the demo role. Rust's
    // stdout is line-buffered, so this reaches the pipe before `accept` blocks.
    println!("listening on {}", listener.local_addr()?);

    let state = Arc::new(State {
        events,
        models,
        path,
    });

    loop {
        let (stream, _) = listener.accept().await?;
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            if let Err(err) = serve(&state, stream).await {
                eprintln!("connection failed: {err}");
            }
        });
    }
}

/// Reads one request and answers it.
async fn serve(state: &State, mut stream: TcpStream) -> Result<()> {
    let Some(request) = Request::read(&mut stream).await? else {
        return Ok(());
    };

    match (request.method.as_str(), request.path.as_str()) {
        ("POST", "/reserve") => take_a_seat(state, &request, &mut stream).await,
        ("GET", "/seats") => show_the_seats(state, &request, &mut stream).await,
        _ => Ok(respond(&mut stream, 404, "Not Found", &[], "no such route\n").await?),
    }
}

/// `POST /reserve?seat=…&patron=…`
///
/// Every outcome of the command loop becomes its own status code, and the
/// distinctions are the point. A `409` is a rule refusing, and retrying it
/// unchanged will refuse again. A `503` is the retry bound spent under
/// contention, and retrying it is exactly the right thing to do. Collapsing
/// the two — which a handler that returned `500` for everything that was not
/// `Ok` would do — leaves a client unable to tell "never" from "not yet".
async fn take_a_seat(state: &State, request: &Request, stream: &mut TcpStream) -> Result<()> {
    let (Some(seat), Some(patron)) = (request.param("seat"), request.param("patron")) else {
        return Ok(respond(
            stream,
            400,
            "Bad Request",
            &[],
            "seat and patron are required\n",
        )
        .await?);
    };

    let outcome = reserve(&state.events, seat, patron).await;
    let (status, reason) = outcome.status();

    // The position is what makes the read below able to wait for this write.
    // A client that never sends it back gets whatever the view happens to hold.
    let headers = match &outcome {
        Reservation::Taken { position, attempts } => vec![
            ("X-Happenstance-Position", position.to_string()),
            ("X-Happenstance-Attempts", attempts.to_string()),
        ],
        Reservation::Contended { attempts } => {
            vec![("X-Happenstance-Attempts", attempts.to_string())]
        }
        Reservation::Refused { .. } | Reservation::Failed { .. } => Vec::new(),
    };

    Ok(respond(stream, status, reason, &headers, &outcome.body()).await?)
}

/// `GET /seats` — optionally `?at=<position>`
///
/// Without `at`, this serves whatever the runner has got to, which is the
/// ordinary case and is allowed to be stale. With `at`, the client is saying
/// *"I wrote at this position and I want to see a view that includes it"*, and
/// a view that has not reached it answers `202` with its own checkpoint rather
/// than serving rows that would silently be missing the caller's own write.
///
/// The `202` matters more than it looks. Read-your-own-writes across an
/// asynchronous read model is the failure every projection-backed API has, and
/// the usual shape of it is a client that writes, immediately reads, sees
/// nothing, and concludes the write was lost.
async fn show_the_seats(state: &State, request: &Request, stream: &mut TcpStream) -> Result<()> {
    let through = checkpoint_through(&state.models).await?;
    let checkpoint = through.map_or_else(|| "none".to_owned(), |position| position.to_string());

    if let Some(raw) = request.param("at") {
        let wanted = match requested_position(raw) {
            Ok(position) => position,
            Err(bad) => {
                return Ok(respond(
                    stream,
                    400,
                    "Bad Request",
                    &[],
                    &format!("`{bad}` is not a position\n"),
                )
                .await?);
            }
        };

        if through.is_none_or(|reached| reached < wanted.get()) {
            return Ok(respond(
                stream,
                202,
                "Accepted",
                &[("X-Happenstance-Checkpoint", checkpoint)],
                "the read model has not reached that position yet\n",
            )
            .await?);
        }
    }

    let app = open_configured(&state.path)?;
    let mut body = String::new();
    for (seat, patron) in seat_rows(&app)? {
        // Writing into a `String` cannot fail; the `Result` is discarded rather
        // than `unwrap`ped so this server carries no panic path a request could
        // reach even in principle.
        let _ = writeln!(body, "{seat} {patron}");
    }

    Ok(respond(
        stream,
        200,
        "OK",
        &[("X-Happenstance-Checkpoint", checkpoint)],
        &body,
    )
    .await?)
}
