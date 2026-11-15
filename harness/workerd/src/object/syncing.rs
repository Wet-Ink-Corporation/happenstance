//! The object as a sync peer over HTTP (#130), mounted as an application
//! mounts it, over the object's own unprefixed log; and `/sync-reset`, which
//! empties the object.
//!
//! Each sync request migrates the log first (a no-op once it exists, and what
//! mints the identity on the first request after a reset), builds the ingest
//! store from the object's state with the published constructor, and hands
//! the request to `serve_sync_request`. A failure to do either answers the
//! status its class maps to, as a refusal from the peer would.
//!
//! Two mounts serve the one log, each under the limits its clients declare
//! for the far side: the four wire messages carry no limits (ADR-0097), so
//! both ends are configured alike. `/sync/pull` and `/sync/push` declare
//! `PeerLimits::minimum`, which every client of them declares.
//! `/sync-wide/pull` and `/sync-wide/push` declare
//! [`wide_limits`](crate::sync_mount::wide_limits), the only limits under which
//! a push can carry more than Cloudflare's tags-per-call ceiling (VT-36).

use happenstance_cloudflare::happenstance_sync::{Classify, PeerLimits, StoreSyncPeer};
use happenstance_cloudflare::sync_http::{
    SYNC_PATH_PREFIX, serve_sync_request, status_for, sync_route,
};
use happenstance_cloudflare::worker::worker_sys::DurableObjectState;
use happenstance_cloudflare::{CloudflareEventStore, CloudflareIngestStore, SqlStorage};
use worker::{Request, Response, Result, State};

/// The object-side context a sync request runs in.
#[derive(Debug)]
pub(super) struct Syncing<'a> {
    pub(super) state: &'a State,
    /// The same JavaScript object as `state`, the type the ingest store's
    /// published constructors take.
    pub(super) object_state: &'a DurableObjectState,
}

impl Syncing<'_> {
    /// `pull` or `push` under a mount, `rest` being the path after it, served
    /// under the mount's `limits`; 404 for anything else.
    ///
    /// `rest` is read through `sync_route`, so the published crate's table
    /// stays the only one naming the routes.
    pub(super) async fn serve(
        &self,
        request: Request,
        rest: &str,
        limits: PeerLimits,
    ) -> Result<Response> {
        let Some(route) = sync_route(&format!("{SYNC_PATH_PREFIX}{rest}")) else {
            return Response::error(format!("no sync route {}", request.path()), 404);
        };
        let migrated = CloudflareEventStore::new(SqlStorage::from_state(self.state)).migrate();
        let store = match migrated
            .and_then(|()| CloudflareIngestStore::from_object_state(self.object_state))
        {
            Ok(store) => store,
            Err(error) => return Response::error(error.to_string(), status_for(error.class())),
        };
        // Two handles onto one store: a clone is two `Rc` bumps.
        let peer = StoreSyncPeer::with_limits(store.clone(), store, limits);
        serve_sync_request(&peer, route, request).await
    }

    /// `/sync-reset`: every row of the object's storage, gone. The next sync
    /// request migrates a fresh log under a fresh identity, which is how a
    /// restore to nothing looks to a peer.
    pub(super) async fn reset(&self) -> Result<Response> {
        self.state.storage().delete_all().await?;
        Response::ok("reset")
    }
}
