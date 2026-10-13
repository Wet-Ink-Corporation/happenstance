//! The routes that run a cross-restart rule's halves, and the store the object
//! holds between them.
//!
//! Every body is text. Status codes: 200 the rule passed (or the before-half
//! ran), 422 the rule rejected the store, 409 the restart the runner claims is
//! not the one the object observed, 500 a harness fault, 404 an unknown name.
//! Errors stay typed until the response is built.

use core::cell::OnceCell;
use core::fmt;

use happenstance_cloudflare::{
    CloudflareEventStore, CloudflareEventStoreError, InvalidTableNamespace, SqlStorage,
    TableNamespace,
};
use happenstance_core::StoreId;
use worker::{Request, Response, Result, State};

use crate::incarnation::{Expectation, Incarnation, check_restart};
use crate::restart::{
    self, Evidence, FreshHighWater, InstanceLog, MintsPerOpen, Rule, Subject, UnknownRule,
    UnknownSubject,
};

/// The namespace the restart subjects keep their logs under, apart from the
/// conformance fixtures' `f0`..`f7`.
const NAMESPACE: &str = "restart";

/// The store a cross-restart rule runs against, held by one object instance.
///
/// Built on first use in each instance and never rebuilt within it: being
/// per-instance is the property under test, so a restart is the only thing
/// that replaces it.
///
/// **One cell per object.** The runner names each object after the one
/// (subject, rule) cell it runs, and the object's storage is cleared when that
/// cell ends: after its after-half, whatever the verdict, or after a
/// before-half that did not succeed, since no after-half will follow it. An
/// object is never reused for a second cell, and an instance asked for a
/// second subject refuses rather than serving it from the first one's store.
#[derive(Debug)]
pub(super) struct Held {
    subject: Subject,
    store: HeldStore,
}

#[derive(Debug)]
enum HeldStore {
    Cloudflare(CloudflareEventStore),
    InstanceLog(InstanceLog),
    FreshHighWater(FreshHighWater<CloudflareEventStore>),
    MintsPerOpen(MintsPerOpen<CloudflareEventStore>),
}

/// Why the instance could not produce the store a request asked for.
#[derive(Debug)]
enum HeldError {
    Namespace(InvalidTableNamespace),
    Migrate(CloudflareEventStoreError),
    SetTwice,
    Missing,
    OtherSubject { held: Subject, asked: Subject },
}

impl fmt::Display for HeldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Namespace(err) => write!(f, "the {NAMESPACE} namespace: {err}"),
            Self::Migrate(err) => write!(f, "migrating {NAMESPACE}: {err}"),
            Self::SetTwice => f.write_str("the held store was set twice"),
            Self::Missing => f.write_str("the held store is missing"),
            Self::OtherSubject { held, asked } => write!(
                f,
                "this instance holds {}, not {}",
                held.name(),
                asked.name()
            ),
        }
    }
}

impl HeldStore {
    fn open(subject: Subject, state: &State) -> core::result::Result<Self, HeldError> {
        let durable = || {
            let namespace = TableNamespace::new(NAMESPACE).map_err(HeldError::Namespace)?;
            let store = CloudflareEventStore::namespaced(SqlStorage::from_state(state), &namespace);
            store.migrate().map_err(HeldError::Migrate)?;
            Ok::<_, HeldError>(store)
        };
        Ok(match subject {
            Subject::Cloudflare => Self::Cloudflare(durable()?),
            Subject::InstanceLog => Self::InstanceLog(InstanceLog::default()),
            Subject::FreshHighWater => Self::FreshHighWater(FreshHighWater::new(durable()?)),
            Subject::MintsPerOpen => {
                Self::MintsPerOpen(MintsPerOpen::new(durable()?, minted_store_id()))
            }
        })
    }
}

/// A store identity minted for this instance alone: [`MintsPerOpen`]'s defect.
fn minted_store_id() -> StoreId {
    let mut bytes = [0_u8; 16];
    let (high, low) = bytes.split_at_mut(8);
    high.copy_from_slice(&Incarnation::mint().bits().to_be_bytes());
    low.copy_from_slice(&Incarnation::mint().bits().to_be_bytes());
    StoreId::from_bytes(bytes)
}

/// `rule <name>` per rule, then `subject <name> rejects <rule>:<check> …` per
/// subject: the runner iterates this and names nothing itself.
pub(super) fn registry() -> String {
    let rules = Rule::ALL.iter().map(|rule| format!("rule {}", rule.name()));
    let subjects = Subject::ALL.iter().map(|subject| {
        let rejects: Vec<String> = subject
            .rejects()
            .iter()
            .map(|(rule, check)| format!("{}:{}", rule.name(), check.name()))
            .collect();
        format!("subject {} rejects {}", subject.name(), rejects.join(" "))
    });
    rules.chain(subjects).collect::<Vec<_>>().join("\n")
}

/// What one restart request asked for: `<subject>/<rule>/<half>`.
#[derive(Debug)]
pub(super) struct Route {
    subject: Subject,
    rule: Rule,
    half: Half,
}

#[derive(Debug)]
enum Half {
    Before,
    After,
}

/// A restart path that names no cell, with the part that did not parse.
#[derive(Debug)]
pub(super) enum RouteError {
    Shape { path: String },
    Half { half: String },
    Subject(UnknownSubject),
    Rule(UnknownRule),
}

impl fmt::Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Shape { path } => write!(f, "expected <subject>/<rule>/<half>, not {path}"),
            Self::Half { half } => write!(f, "no half called {half:?}"),
            Self::Subject(err) => write!(f, "{err}"),
            Self::Rule(err) => write!(f, "{err}"),
        }
    }
}

impl Route {
    /// Parses `<subject>/<rule>/<half>`.
    pub(super) fn parse(path: &str) -> core::result::Result<Self, RouteError> {
        let mut parts = path.splitn(3, '/');
        let (Some(subject), Some(rule), Some(half)) = (parts.next(), parts.next(), parts.next())
        else {
            return Err(RouteError::Shape {
                path: path.to_owned(),
            });
        };
        let half = match half {
            "before" => Half::Before,
            "after" => Half::After,
            other => {
                return Err(RouteError::Half {
                    half: other.to_owned(),
                });
            }
        };
        Ok(Self {
            subject: subject.parse().map_err(RouteError::Subject)?,
            rule: rule.parse().map_err(RouteError::Rule)?,
            half,
        })
    }
}

/// The object-side context a restart request runs in.
#[derive(Debug)]
pub(super) struct Instance<'a> {
    pub(super) state: &'a State,
    pub(super) incarnation: Incarnation,
    pub(super) held: &'a OnceCell<Held>,
}

impl Instance<'_> {
    /// Runs one half of one cell, and clears the object's storage when the
    /// cell is over: after the after-half on every path, and after a
    /// before-half that did not answer 200.
    pub(super) async fn run(&self, route: Route, mut req: Request) -> Result<Response> {
        let response = self.answer(&route, &mut req).await;
        let over = match route.half {
            Half::After => true,
            Half::Before => !matches!(&response, Ok(answered) if answered.status_code() == 200),
        };
        if over {
            // A deployed object is billed for what it stores.
            self.state.storage().delete_all().await?;
        }
        response
    }

    async fn answer(&self, route: &Route, req: &mut Request) -> Result<Response> {
        let held = match self.held(route.subject) {
            Ok(held) => held,
            Err(fault) => return Response::error(fault.to_string(), 500),
        };
        match route.half {
            Half::Before => self.before(route.rule, held).await,
            Half::After => {
                let url = req.url()?;
                let body = req.text().await?;
                self.after(route.rule, held, &url, &body).await
            }
        }
    }

    /// The store this instance holds for `subject`, opened on first use.
    fn held(&self, subject: Subject) -> core::result::Result<&Held, HeldError> {
        if self.held.get().is_none() {
            let store = HeldStore::open(subject, self.state)?;
            // Nothing is awaited between the check above and this, so the cell
            // is still empty; a refusal would mean it is not.
            self.held
                .set(Held { subject, store })
                .map_err(|_| HeldError::SetTwice)?;
        }
        let held = self.held.get().ok_or(HeldError::Missing)?;
        if held.subject != subject {
            return Err(HeldError::OtherSubject {
                held: held.subject,
                asked: subject,
            });
        }
        Ok(held)
    }

    async fn before(&self, rule: Rule, held: &Held) -> Result<Response> {
        let incarnation = self.incarnation;
        let outcome = match &held.store {
            HeldStore::Cloudflare(store) => restart::before(rule, store, incarnation).await,
            HeldStore::InstanceLog(store) => restart::before(rule, store, incarnation).await,
            HeldStore::FreshHighWater(store) => restart::before(rule, store, incarnation).await,
            HeldStore::MintsPerOpen(store) => restart::before(rule, store, incarnation).await,
        };
        match outcome {
            Ok(evidence) => {
                let mut response = Response::ok(evidence.to_string())?;
                response
                    .headers_mut()
                    .set("x-wasm", &Incarnation::of_wasm_instance().to_string())?;
                Ok(response)
            }
            Err(fault) => Response::error(fault.to_string(), 500),
        }
    }

    async fn after(
        &self,
        rule: Rule,
        held: &Held,
        url: &worker::Url,
        body: &str,
    ) -> Result<Response> {
        let evidence: Evidence = match body.parse() {
            Ok(evidence) => evidence,
            Err(err) => return Response::error(format!("HARNESS {}: {err}", rule.name()), 500),
        };
        let expect = url
            .query_pairs()
            .find(|(key, _)| key == "expect")
            .map(|(_, value)| value.parse::<Expectation>());
        let expect = match expect {
            Some(Ok(expect)) => expect,
            Some(Err(err)) => return Response::error(err.to_string(), 400),
            None => return Response::error("expect= is required", 400),
        };
        if let Err(mismatch) = check_restart(expect, evidence.incarnation(), self.incarnation) {
            return Response::error(mismatch.to_string(), 409);
        }
        let socket_held = url
            .query_pairs()
            .any(|(key, value)| key == "socket" && value == "held");
        if socket_held {
            let sockets = self.state.get_websockets().len();
            if sockets != 1 {
                return Response::error(
                    format!("hibernation not observed: {sockets} WebSockets restored, not 1"),
                    409,
                );
            }
        }
        let verdict = match &held.store {
            HeldStore::Cloudflare(store) => restart::after(rule, store, &evidence).await,
            HeldStore::InstanceLog(store) => restart::after(rule, store, &evidence).await,
            HeldStore::FreshHighWater(store) => restart::after(rule, store, &evidence).await,
            HeldStore::MintsPerOpen(store) => restart::after(rule, store, &evidence).await,
        };
        match verdict {
            Ok(Ok(())) => Response::ok("passed"),
            Ok(Err(failure)) => Response::error(failure.to_string(), 422),
            Err(fault) => Response::error(fault.to_string(), 500),
        }
    }
}
