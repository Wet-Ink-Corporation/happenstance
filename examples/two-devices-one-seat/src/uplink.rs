//! The device's link to central: an [`Exchange`] that is a function call.
//!
//! Every pull and push a device makes is encoded into the sync envelope by
//! [`RemoteSyncPeer`](happenstance_sync::RemoteSyncPeer), handed here as
//! bytes, answered by [`serve_sync_adjudicated`] over central's
//! [`StoreSyncPeer`], which judges each push by central's rule before it
//! lands, and decoded on the way back. That is the whole network
//! path with the network replaced by a call, so replacing [`Uplink`] with an
//! HTTP client changes the transport and nothing else.
//!
//! # Why `Cell`, and why the bare `Exchange`
//!
//! The story turns the signal off and on while a peer holds the exchange and
//! an edge borrows the peer, so the switch is flipped through `&self`. `Cell`
//! is the smallest tool for that: a `Copy` value replaced whole, no borrow to
//! hold, no lock. It costs `Sync`, which nothing here needs, because the story
//! runs on one thread. So this implements the **bare** [`Exchange`], the
//! flavour without a `Send` bound; the runner takes `!Send` peers (SY-17).
//! The alternative was an `AtomicBool` and `SendExchange`, which buys thread
//! safety nobody uses at the price of a memory-ordering argument.
//!
//! # Why there is no timeout
//!
//! One round trip is one function call into a local file. There is no
//! external I/O to bound; an HTTP `Exchange` puts its timeout here, and maps
//! it to [`FailureClass::Transient`].

use core::cell::Cell;

use happenstance::EventStore;
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_sync::runner::Adjudicate;
use happenstance_sync::wire::Route;
use happenstance_sync::{
    Classify, Exchange, FailureClass, ServeError, StoreSyncPeer, serve_sync_adjudicated,
};

use crate::rule::{CentralRule, RuleError};

/// The central store's type.
///
/// The sync path names central only through this alias, [`CentralError`] and
/// [`CentralPeer`], so on that path a Postgres central changes this one line.
/// It is not the whole swap: central's file is opened by the same SQLite
/// `Site` the devices use, its read model writes through a SQLite batch, and
/// some `StoryError` variants name SQLite's errors for every place. The
/// crate's overview lists each, under *Swapping central for Postgres or Neon*.
pub type Central = SqliteEventStore;

/// What central's store fails with, on its read half and its ingest half
/// alike, as the [`EventStore`] contract names it rather than the adapter.
pub type CentralError = <Central as EventStore>::Error;

/// Central's side of a sync: its log to read, its ingest to write, and the
/// rule it judges each push by before the push lands (SY-7).
///
/// The rule is a type parameter with central's own as its default, so the
/// test that shows what goes wrong without it can mount one that answers
/// nothing, while the binary has no way to.
pub type CentralPeer<A = CentralRule> = StoreSyncPeer<Central, Central, A>;

/// Whether a device has signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Link {
    /// Requests reach central.
    Up,
    /// Nothing leaves the device.
    Down,
    /// A pull reaches central and its answer comes back; a push is lost
    /// before it leaves. The case that shows a failed push loses no event: the
    /// step fails in its push phase, after the pull has already landed.
    PullOnly,
}

/// One device's link to central, whose peer judges what it is pushed by `A`.
///
/// No bound on the struct, as on every generic type here: the bound sits on
/// the `Exchange` impl, the one place that calls the rule.
#[derive(Debug)]
pub struct Uplink<A = CentralRule> {
    far: CentralPeer<A>,
    link: Cell<Link>,
}

impl<A> Uplink<A> {
    /// A link to `far`, with signal.
    #[must_use]
    pub const fn new(far: CentralPeer<A>) -> Self {
        Self {
            far,
            link: Cell::new(Link::Up),
        }
    }

    /// Turns the signal on or off, or leaves it able to pull only.
    pub fn set(&self, link: Link) {
        self.link.set(link);
    }
}

/// Any rule central mounts fails as a [`RuleError`], so every uplink fails
/// with one [`UplinkError`] and a sync failure has one type, whichever rule
/// judged it.
impl<A: Adjudicate<Error = RuleError>> Exchange for Uplink<A> {
    type Error = UplinkError;

    async fn exchange(&self, route: Route, request: Vec<u8>) -> Result<Vec<u8>, UplinkError> {
        // A pull-only link lets exactly the pull through and loses every other
        // route: the push, and, because `Route` is non-exhaustive, any route a
        // later version adds.
        match (self.link.get(), route) {
            (Link::Up, _) | (Link::PullOnly, Route::Pull) => {
                Ok(serve_sync_adjudicated(&self.far, route, &request).await?)
            }
            (Link::Down | Link::PullOnly, _) => Err(UplinkError::Offline),
        }
    }
}

/// Why an exchange failed.
#[derive(Debug, thiserror::Error)]
pub enum UplinkError {
    /// No signal: the request never left the device.
    #[error("no signal: the request never left the device")]
    Offline,
    /// Central answered with a failure of its own, its rule's included.
    #[error(transparent)]
    Serve(#[from] ServeError<CentralError, CentralError, RuleError>),
}

impl Classify for UplinkError {
    /// No signal is the textbook transient failure: nothing was sent, and
    /// the same request will do when the signal returns. Central's own
    /// failures keep the class central stated (SY-36).
    fn class(&self) -> FailureClass {
        match self {
            Self::Offline => FailureClass::Transient,
            Self::Serve(error) => error.class(),
        }
    }
}
