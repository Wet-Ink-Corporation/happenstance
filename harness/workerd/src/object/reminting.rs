//! The `/remint/...` routes: the object's three steps of the bookmark-restore
//! leg for `remint_identity` (#298, [`crate::remint`]).
//!
//! - `seed`: append [`KEPT`]. The runner takes the bookmark next.
//! - `lose`: append [`LOST`] and answer the [`Ledger`] of all four events.
//!   The runner then restores the bookmark, which restarts the object.
//! - `after?mode=remint|control`, with the ledger as the body: on the new
//!   instance, check the restore, re-mint or not, append [`REISSUED`], and
//!   answer the [`Verdict`](crate::remint::Verdict).
//!
//! Every body is text and starts with the instance's witness. Status codes:
//! 200 done (for `after`, the cell passed), 422 the cell failed, 409 the
//! restart or the restore the runner claims is not what the object sees, 400
//! a bad `mode=` or ledger, 404 an unknown route, 500 a harness fault. The
//! storage is cleared after every `after` answer and every fault, since the
//! cell ends there.

use core::fmt;

use happenstance_cloudflare::{
    CloudflareEventStore, CloudflareEventStoreError, InvalidTableNamespace, SqlStorage,
    TableNamespace,
};
use happenstance_core::bytes::Bytes;
use happenstance_core::{AppendError, Event, EventStore, Query, ReadOptions, collect};
use worker::{Request, Response, Result, State};

use crate::incarnation::Incarnation;
use crate::remint::{KEPT, LOST, Ledger, Mode, REISSUED, Sighting, judge, restore_observed};

/// The namespace the leg keeps its log under, apart from every other route's.
const NAMESPACE: &str = "remint";

/// A step that could not be carried out, as opposed to a cell's verdict.
#[derive(Debug)]
enum Fault {
    Namespace(InvalidTableNamespace),
    Store(&'static str, CloudflareEventStoreError),
    Append(AppendError<CloudflareEventStoreError>),
    Event(String),
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Namespace(err) => write!(f, "the {NAMESPACE} namespace: {err}"),
            Self::Store(during, err) => write!(f, "{during}: {err}"),
            Self::Append(err) => write!(f, "appending: {err}"),
            Self::Event(err) => write!(f, "building an event: {err}"),
        }
    }
}

/// The object-side context a re-mint request runs in.
#[derive(Debug)]
pub(super) struct Remint<'a> {
    pub(super) state: &'a State,
    pub(super) incarnation: Incarnation,
    /// This instance's witness, `instance=<nonce> wasm=<nonce>`.
    pub(super) witness: String,
}

impl Remint<'_> {
    /// `seed`, `lose` or `after`.
    pub(super) async fn run(&self, route: &str, mut req: Request) -> Result<Response> {
        let store = match self.open() {
            Ok(store) => store,
            Err(fault) => return self.fail(fault).await,
        };
        let answered = match route {
            "seed" => append_each(&store, &KEPT)
                .await
                .map(|()| self.witness.clone()),
            "lose" => self.lose(&store).await,
            "after" => {
                let mode = req
                    .url()?
                    .query_pairs()
                    .find(|(key, _)| key == "mode")
                    .map(|(_, value)| value.parse::<Mode>());
                let mode = match mode {
                    Some(Ok(mode)) => mode,
                    Some(Err(err)) => return self.finish(err.to_string(), 400).await,
                    None => return self.finish("mode= is required".to_owned(), 400).await,
                };
                let ledger = match req.text().await?.parse::<Ledger>() {
                    Ok(ledger) => ledger,
                    Err(err) => return self.finish(err.to_string(), 400).await,
                };
                return self.after(&store, mode, &ledger).await;
            }
            other => return Response::error(format!("no remint route {other}"), 404),
        };
        // The witness is cloned into the body: a response owns its text.
        match answered {
            Ok(body) => Response::ok(body),
            Err(fault) => self.fail(fault).await,
        }
    }

    /// Appends the events the restore loses, and answers the ledger.
    async fn lose(&self, store: &CloudflareEventStore) -> core::result::Result<String, Fault> {
        append_each(store, &LOST).await?;
        let ledger = Ledger {
            incarnation: self.incarnation,
            events: sightings(store).await?,
        };
        Ok(ledger.to_string())
    }

    /// Checks the restart and the restore, then runs the cell and judges it.
    async fn after(
        &self,
        store: &CloudflareEventStore,
        mode: Mode,
        ledger: &Ledger,
    ) -> Result<Response> {
        if ledger.incarnation == self.incarnation {
            let reason = "no restart: the instance that wrote the ledger answered";
            return self.finish(reason.to_owned(), 409).await;
        }
        let now = match sightings(store).await {
            Ok(now) => now,
            Err(fault) => return self.fail(fault).await,
        };
        if !restore_observed(ledger, &now) {
            let held: Vec<String> = now
                .iter()
                .map(|seen| format!("{} {}", seen.event_type, seen.id))
                .collect();
            let reason = format!(
                "the restore was not observed: the object holds [{}]",
                held.join(", ")
            );
            return self.finish(reason, 409).await;
        }
        let ran = async {
            if mode == Mode::Remint {
                store
                    .remint_identity()
                    .map_err(|err| Fault::Store("re-minting", err))?;
            }
            append_each(store, &REISSUED).await?;
            sightings(store).await
        };
        let after = match ran.await {
            Ok(after) => after,
            Err(fault) => return self.fail(fault).await,
        };
        let verdict = judge(mode, ledger, &after);
        let status = if verdict.passed() { 200 } else { 422 };
        self.finish(verdict.to_string(), status).await
    }

    /// The adapter as a production class builds it, under its own namespace.
    fn open(&self) -> core::result::Result<CloudflareEventStore, Fault> {
        let namespace = TableNamespace::new(NAMESPACE).map_err(Fault::Namespace)?;
        let store =
            CloudflareEventStore::namespaced(SqlStorage::from_state(self.state), &namespace);
        store
            .migrate()
            .map_err(|err| Fault::Store("migrating", err))?;
        Ok(store)
    }

    /// The cell ends with `body` under `status`: the storage is cleared first.
    async fn finish(&self, body: String, status: u16) -> Result<Response> {
        self.state.storage().delete_all().await?;
        let text = format!("{} {body}", self.witness);
        if status == 200 {
            Response::ok(text)
        } else {
            Response::error(text, status)
        }
    }

    /// A harness fault: the storage is cleared, since the cell ends here.
    async fn fail(&self, fault: Fault) -> Result<Response> {
        self.finish(fault.to_string(), 500).await
    }
}

/// Appends each of `types` as its own one-event batch, so each takes its own
/// position.
async fn append_each(
    store: &CloudflareEventStore,
    types: &[&'static str],
) -> core::result::Result<(), Fault> {
    for event_type in types {
        let event = Event::new(*event_type, Bytes::from_static(b"remint"))
            .map_err(|err| Fault::Event(err.to_string()))?;
        store.append(&[event], None).await.map_err(Fault::Append)?;
    }
    Ok(())
}

/// Every event the store yields, as its type and its identity's text.
async fn sightings(store: &CloudflareEventStore) -> core::result::Result<Vec<Sighting>, Fault> {
    let events = collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .map_err(|err| Fault::Store("reading everything back", err))?;
    Ok(events
        .iter()
        .map(|event| Sighting {
            event_type: event.event.event_type().as_str().to_owned(),
            id: event.id.to_string(),
        })
        .collect())
}
