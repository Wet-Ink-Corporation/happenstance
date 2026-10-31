//! The `/ingest-...` routes: the ingest family, its ceiling and its failure
//! census, each on the object it is sent to (#108 part E3).
//!
//! Status codes: 200 a rule ran or skipped, an attempt was accepted or a case
//! was judged; 400 a bad shape or count; 404 an unknown rule, case or route;
//! 422 a rule's violation, or an attempt the store refused; 500 a harness
//! fault. Every answer but an unknown route clears the object's storage first,
//! as `/rule/<name>` does, and carries the instance in `x-incarnation`.

use happenstance_cloudflare::TableNamespace;
use happenstance_cloudflare::worker::worker_sys::DurableObjectState;
use happenstance_sync_testkit::IngestFixture as _;
use worker::{Request, Response, Result, State};

use crate::incarnation::Incarnation;
use crate::ingest::ceiling::{self, Attempt, Shape};
use crate::ingest::classes::{self, Case};
use crate::ingest::dispatch;
use crate::ingest::fixture::WorkerdIngestFixture;

/// The namespace a ceiling attempt keeps its log under. Each attempt runs on
/// its own object, so one name serves every attempt.
const CEILING_NAMESPACE: &str = "ceiling";

/// The object-side context an ingest request runs in.
#[derive(Debug)]
pub(super) struct Ingesting<'a> {
    pub(super) state: &'a State,
    /// The same JavaScript object as `state`, the type the ingest store's
    /// published constructors take.
    pub(super) object_state: &'a DurableObjectState,
    pub(super) incarnation: Incarnation,
}

impl Ingesting<'_> {
    /// `ingest-rule/<name>`, `ingest-ceiling/<shape>?events=<n>` or
    /// `ingest-class/<case>`.
    pub(super) async fn run(&self, route: &str, arg: &str, req: &Request) -> Result<Response> {
        let mut response = match route {
            "ingest-rule" => {
                super::answer(dispatch::run(self.state, self.object_state, arg).await)?
            }
            "ingest-ceiling" => self.ceiling(arg, req).await?,
            "ingest-class" => match arg.parse::<Case>() {
                Ok(case) => {
                    Response::ok(classes::provoke(case, self.state, self.object_state).await)?
                }
                Err(unknown) => Response::error(unknown.to_string(), 404)?,
            },
            other => return Response::error(format!("no ingest route {other}"), 404),
        };
        // A deployed object is billed for what it stores, and an attempt's
        // log is worthless once it has answered.
        self.state.storage().delete_all().await?;
        response
            .headers_mut()
            .set("x-incarnation", &self.incarnation.to_string())?;
        Ok(response)
    }

    /// One ceiling attempt: `events` foreign events of `shape`, one group,
    /// one call, through the published constructor on a fresh namespace.
    async fn ceiling(&self, shape: &str, req: &Request) -> Result<Response> {
        let shape = match shape.parse::<Shape>() {
            Ok(shape) => shape,
            Err(unknown) => return Response::error(unknown.to_string(), 400),
        };
        let url = req.url()?;
        let events = url
            .query_pairs()
            .find(|(key, _)| key == "events")
            .map(|(_, value)| value.parse::<usize>());
        let events = match events {
            Some(Ok(events)) => events,
            Some(Err(err)) => return Response::error(format!("events=: {err}"), 400),
            None => return Response::error("events= is required", 400),
        };
        // Built before anything touches storage, and refused past the cap.
        let batch = match ceiling::foreign_batch(shape, events) {
            Ok(batch) => batch,
            Err(err) => return Response::error(err.to_string(), 400),
        };
        let fixture = match TableNamespace::new(CEILING_NAMESPACE)
            .map_err(|err| err.to_string())
            .and_then(|namespace| {
                WorkerdIngestFixture::open(self.state, self.object_state, namespace)
                    .map_err(|err| err.to_string())
            }) {
            Ok(fixture) => fixture,
            Err(fault) => return Response::error(fault, 500),
        };
        let attempt = ceiling::attempt(&fixture.connect().await, &batch).await;
        match attempt {
            Attempt::Accepted { .. } => Response::ok(attempt.line()),
            Attempt::Refused { .. } => Response::error(attempt.line(), 422),
        }
    }
}
