//! The `/idle/...` routes: arm an [`IdleHold`] in this instance, and release
//! it.
//!
//! Every body is text and starts with the instance's witness, so the runner
//! can tell which construction answered. Status codes: 200 done, 400 a bad
//! `stream=`, 404 an unknown route, 409 armed twice in one instance, 500 a
//! harness fault.

use core::cell::RefCell;
use core::fmt;

use happenstance_cloudflare::{
    CloudflareEventStore, CloudflareEventStoreError, InvalidTableNamespace, SqlStorage,
    TableNamespace,
};
use worker::{Request, Response, Result, State};

use crate::idle::{EVENTS, IdleHold, StreamHold};

/// The namespace the held store keeps its log under, apart from the
/// conformance fixtures' and the restart subjects'.
const NAMESPACE: &str = "idle";

/// Why the held store could not be opened.
#[derive(Debug)]
enum OpenError {
    Namespace(InvalidTableNamespace),
    Migrate(CloudflareEventStoreError),
}

impl fmt::Display for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Namespace(err) => write!(f, "the {NAMESPACE} namespace: {err}"),
            Self::Migrate(err) => write!(f, "migrating {NAMESPACE}: {err}"),
        }
    }
}

/// The object-side context an idle request runs in.
#[derive(Debug)]
pub(super) struct Idle<'a> {
    pub(super) state: &'a State,
    /// This instance's witness, `instance=<nonce> wasm=<nonce>`.
    pub(super) witness: String,
    pub(super) slot: &'a RefCell<Option<IdleHold>>,
}

impl Idle<'_> {
    /// `arm?stream=none|held` or `release`.
    pub(super) async fn run(&self, route: &str, req: &Request) -> Result<Response> {
        match route {
            "arm" => self.arm(req).await,
            "release" => self.release().await,
            other => Response::error(format!("no idle route {other}"), 404),
        }
    }

    /// Appends, opens what `stream=` asks for, parks it in this instance and
    /// answers. From here on the object holds memory and nothing else.
    async fn arm(&self, req: &Request) -> Result<Response> {
        let url = req.url()?;
        let hold = url
            .query_pairs()
            .find(|(key, _)| key == "stream")
            .map(|(_, value)| value.parse::<StreamHold>());
        let hold = match hold {
            Some(Ok(hold)) => hold,
            Some(Err(err)) => return Response::error(err.to_string(), 400),
            None => return Response::error("stream= is required", 400),
        };
        let store = match self.open() {
            Ok(store) => store,
            Err(fault) => return self.fail(fault).await,
        };
        let armed = match IdleHold::arm(store, hold).await {
            Ok(armed) => armed,
            Err(err) => return self.fail(format!("arming: {err}")).await,
        };
        // The slot is only ever borrowed with nothing awaited under the
        // borrow, so it cannot overlap another; a refusal is still reported,
        // never panicked on.
        let refused = match self.slot.try_borrow_mut() {
            Ok(mut slot) if slot.is_none() => {
                *slot = Some(armed);
                None
            }
            Ok(_) => Some("this instance already holds an idle store"),
            Err(_) => Some("the idle slot is borrowed"),
        };
        match refused {
            None => Response::ok(format!("{} appended={EVENTS}", self.witness)),
            Some(reason) => Response::error(format!("{} {reason}", self.witness), 409),
        }
    }

    /// Releases whatever this instance holds, which after an eviction is
    /// nothing, and clears the object's storage.
    async fn release(&self) -> Result<Response> {
        let taken = match self.slot.try_borrow_mut() {
            Ok(mut slot) => slot.take(),
            Err(_) => return self.fail("the idle slot is borrowed").await,
        };
        let released = match taken.map(IdleHold::release).transpose() {
            Ok(released) => released,
            Err(err) => return self.fail(format!("releasing: {err}")).await,
        };
        // A deployed object is billed for what it stores.
        self.state.storage().delete_all().await?;
        let body = released.map_or_else(|| "held=nothing".to_owned(), |done| done.to_string());
        Response::ok(format!("{} {body}", self.witness))
    }

    /// The adapter as a production class builds it, under its own namespace.
    fn open(&self) -> core::result::Result<CloudflareEventStore, OpenError> {
        let namespace = TableNamespace::new(NAMESPACE).map_err(OpenError::Namespace)?;
        let store =
            CloudflareEventStore::namespaced(SqlStorage::from_state(self.state), &namespace);
        store.migrate().map_err(OpenError::Migrate)?;
        Ok(store)
    }

    /// A harness fault: the storage is cleared, since no release will follow.
    async fn fail(&self, fault: impl fmt::Display) -> Result<Response> {
        self.state.storage().delete_all().await?;
        Response::error(format!("{} {fault}", self.witness), 500)
    }
}
