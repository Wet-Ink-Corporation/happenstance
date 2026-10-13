//! The routes that run a cross-restart rule's halves, and the store the object
//! holds between them.
//!
//! Every body is text. Status codes: 200 the rule passed (or the before-half
//! ran), 422 the rule rejected the store, 409 the restart the runner claims is
//! not the one the object observed, 500 a harness fault, 404 an unknown name.

use core::cell::OnceCell;

use happenstance_cloudflare::{CloudflareEventStore, SqlStorage, TableNamespace};
use worker::{Request, Response, Result, State};

use crate::incarnation::{Expectation, Incarnation, check_restart};
use crate::restart::{self, Evidence, FreshHighWater, InstanceLog, Rule, Subject};

/// The namespace the restart subjects keep their logs under, apart from the
/// conformance fixtures' `f0`..`f7`.
const NAMESPACE: &str = "restart";

/// The store a cross-restart rule runs against, held by one object instance.
///
/// Built on first use in each instance and never rebuilt within it: being
/// per-instance is the property under test, so a restart is the only thing
/// that replaces it.
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
}

impl HeldStore {
    fn open(subject: Subject, state: &State) -> core::result::Result<Self, String> {
        let durable = || {
            let namespace = TableNamespace::new(NAMESPACE).map_err(|err| err.to_string())?;
            let store = CloudflareEventStore::namespaced(SqlStorage::from_state(state), &namespace);
            store
                .migrate()
                .map_err(|err| format!("migrating {NAMESPACE}: {err}"))?;
            Ok::<_, String>(store)
        };
        Ok(match subject {
            Subject::Cloudflare => Self::Cloudflare(durable()?),
            Subject::InstanceLog => Self::InstanceLog(InstanceLog::default()),
            Subject::FreshHighWater => Self::FreshHighWater(FreshHighWater::new(durable()?)),
        })
    }
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

impl Route {
    /// Parses `<subject>/<rule>/<half>`, or answers the 404 that says which
    /// part is unknown.
    pub(super) fn parse(path: &str) -> core::result::Result<Self, String> {
        let mut parts = path.splitn(3, '/');
        let (Some(subject), Some(rule), Some(half)) = (parts.next(), parts.next(), parts.next())
        else {
            return Err(format!("expected <subject>/<rule>/<half>, not {path}"));
        };
        let half = match half {
            "before" => Half::Before,
            "after" => Half::After,
            other => return Err(format!("no half called {other:?}")),
        };
        Ok(Self {
            subject: subject.parse::<Subject>().map_err(|err| err.to_string())?,
            rule: rule.parse::<Rule>().map_err(|err| err.to_string())?,
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
    /// Runs one half of one cell.
    pub(super) async fn run(&self, route: Route, mut req: Request) -> Result<Response> {
        let held = match self.held(route.subject) {
            Ok(held) => held,
            Err(fault) => return Response::error(fault, 500),
        };
        match route.half {
            Half::Before => self.before(route.rule, held).await,
            Half::After => {
                let url = req.url()?;
                let body = req.text().await?;
                let response = self.after(route.rule, held, &url, &body).await;
                // A deployed object is billed for what it stores, and the cell
                // is over whatever the verdict.
                self.state.storage().delete_all().await?;
                response
            }
        }
    }

    /// The store this instance holds for `subject`, opened on first use.
    fn held(&self, subject: Subject) -> core::result::Result<&Held, String> {
        if self.held.get().is_none() {
            let store = HeldStore::open(subject, self.state)?;
            // Nothing is awaited between the check above and this, so the cell
            // is still empty; a refusal would mean it is not.
            self.held
                .set(Held { subject, store })
                .map_err(|_| "the held store was set twice".to_owned())?;
        }
        let held = self
            .held
            .get()
            .ok_or_else(|| "the held store is missing".to_owned())?;
        if held.subject != subject {
            return Err(format!(
                "this instance holds {}, not {}",
                held.subject.name(),
                subject.name()
            ));
        }
        Ok(held)
    }

    async fn before(&self, rule: Rule, held: &Held) -> Result<Response> {
        let outcome = match &held.store {
            HeldStore::Cloudflare(store) => restart::before(rule, store, self.incarnation).await,
            HeldStore::InstanceLog(store) => restart::before(rule, store, self.incarnation).await,
            HeldStore::FreshHighWater(store) => {
                restart::before(rule, store, self.incarnation).await
            }
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
        };
        match verdict {
            Ok(Ok(())) => Response::ok("passed"),
            Ok(Err(failure)) => Response::error(failure.to_string(), 422),
            Err(fault) => Response::error(fault.to_string(), 500),
        }
    }
}
