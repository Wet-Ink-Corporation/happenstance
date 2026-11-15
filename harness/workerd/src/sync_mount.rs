//! The object's two sync mounts and the limits each declares.
//!
//! `/sync/pull` and `/sync/push` declare [`PeerLimits::minimum`]: at most 128
//! events a push, so at most 131,072 tags at the adapter's 1,024 an event,
//! which is half Cloudflare's 262,144 tags a call. No push through them can
//! meet that ceiling. `/sync-wide/pull` and `/sync-wide/push` serve the same
//! log under [`wide_limits`], which can (VT-36).
//!
//! The four wire messages carry no limits (ADR-0097), so both ends must be
//! configured alike. This module is the one definition the object and the
//! native round trip both read, so that holds by construction rather than by
//! two literals.

use happenstance_cloudflare::happenstance_sync::PeerLimits;
use happenstance_testkit::Fixture;

use crate::fixture::WorkerdFixture;

/// The path the wide mount's routes sit under: `/sync-wide/pull` and
/// `/sync-wide/push`.
pub const WIDE_SYNC_PREFIX: &str = "/sync-wide/";

/// The payload bytes the wide mount declares a push: 4 MiB.
///
/// Its only job is to lift the body cap (payload bytes plus
/// `sync_http::ENVELOPE_ALLOWANCE`) above the envelope of a push over the tags
/// ceiling, about 1.3 MB, so the body cap never answers for the ingest.
pub const WIDE_BATCH_BYTES: usize = 4 * 1024 * 1024;

/// What the wide mount declares: [`PeerLimits::minimum`], with the adapter's
/// events-per-call ceiling and [`WIDE_BATCH_BYTES`].
///
/// Owned, because `StoreSyncPeer::with_limits` and `RemoteSyncPeer::new` both
/// keep it.
#[must_use]
pub fn wide_limits() -> PeerLimits {
    // `PeerLimits` is `#[non_exhaustive]`, so no struct literal outside its
    // crate: start from `minimum` and assign the two fields this mount widens.
    let mut limits = PeerLimits::minimum();
    // The `None` arm keeps `minimum`'s count rather than unwrapping; the test
    // below pins that the `Some` arm is the one taken.
    limits.max_batch_events = match <WorkerdFixture<'static> as Fixture>::MAX_EVENTS_PER_BATCH {
        Some(events) => events,
        None => limits.max_batch_events,
    };
    limits.max_batch_bytes = WIDE_BATCH_BYTES;
    limits
}

#[cfg(test)]
mod tests {
    use happenstance_cloudflare::happenstance_sync::PeerLimits;
    use happenstance_testkit::Fixture;

    use super::{WIDE_BATCH_BYTES, wide_limits};
    use crate::fixture::WorkerdFixture;

    /// The wide mount admits as many events a push as the adapter writes in
    /// one call, and `WIDE_BATCH_BYTES` of payload; every other limit is the
    /// narrow mount's.
    ///
    /// Rejects a fixture that loses its events-per-call declaration and so
    /// silently narrows the mount back to `minimum`'s 128, under which no push
    /// meets the tags ceiling (VT-36).
    #[test]
    fn the_wide_mount_declares_the_adapters_events_per_call() {
        let declared = <WorkerdFixture<'static> as Fixture>::MAX_EVENTS_PER_BATCH
            .expect("the harness fixture declares the adapter's events-per-call ceiling");
        let mut expected = PeerLimits::minimum();
        expected.max_batch_events = declared;
        expected.max_batch_bytes = WIDE_BATCH_BYTES;

        assert_eq!(wide_limits(), expected);
    }
}
