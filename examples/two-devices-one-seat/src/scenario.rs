//! Three files, two sync edges, and the steps the story is told in.
//!
//! [`Scenario`] owns every handle the story opens, so dropping it closes them
//! all at once, and the read-back after it can only see what reached disk.

use std::path::{Path, PathBuf};

use happenstance::{
    Boundary, Chunk, CommandOutcome, EventStore, Json, Query, ReadOptions, Retry, SequencedEvent,
    Source, collect, commit, read_decision_model, run_projection,
};
use happenstance_sqlite::connection::open_configured;
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_sqlite::projection_store::SqliteProjectionStore;
use happenstance_sync::runner::{Adjudicate, Edge, SyncStatus};
use happenstance_sync::wire::ResumeToken;
use happenstance_sync::{EventGroup, RemoteSyncPeer, ReplicatedEvent, StoreSyncPeer, SyncPeer};
use rusqlite::Connection;

use crate::domain::{BoxOffice, Device, Place, SaleId, ShowId, ShowLedger, schedule, sell_seat};
use crate::error::StoryError;
use crate::rule::{CentralRule, RuleError};
use crate::timeline::{DDL, Timeline, TimelineView};
use crate::uplink::{Central, Link, Uplink};

/// The show the story sells.
pub const SHOW: &str = "premiere";

/// Its seats.
pub const CAPACITY: u32 = 3;

/// How many times a decision may be attempted before it gives up: three.
/// Every writer here is alone on its file, so the bound is never spent.
/// `MIN.saturating_add` is a `const` way to name a non-zero number without an
/// `unwrap` that could only fail at compile time anyway.
const ATTEMPTS: Retry = Retry::attempts(core::num::NonZeroU32::MIN.saturating_add(2));

/// How many events one projection chunk commits: sixteen.
const CHUNK: core::num::NonZeroUsize = core::num::NonZeroUsize::MIN.saturating_add(15);

/// The most steps one sync takes before it is reported as stuck. A step
/// against an in-process far side either answers or fails, so this is a
/// bound on a loop, not a wait: no sleep, no backoff.
const MAX_STEPS: usize = 4;

/// One place's file, opened in its three roles: the event log, the read
/// model's checkpoint, and the application's own connection for the read
/// model's table. All three share one file, as `transfers-on-sqlite` does.
#[derive(Debug)]
pub(crate) struct Site {
    pub(crate) events: SqliteEventStore,
    models: SqliteProjectionStore,
    app: Connection,
    file: PathBuf,
}

impl Site {
    /// Opens, or reopens, `at`'s file under `dir`.
    pub(crate) fn open(dir: &Path, at: Place) -> Result<Self, StoryError> {
        let file = dir.join(format!("{at}.db"));
        let events = SqliteEventStore::open(&file).map_err(|source| StoryError::Open {
            file: file.clone(),
            source,
        })?;
        let models =
            SqliteProjectionStore::open(&file).map_err(|source| StoryError::OpenModel {
                file: file.clone(),
                source,
            })?;
        let app = open_configured(&file)
            .and_then(|app| app.execute_batch(DDL).map(|()| app))
            .map_err(|source| StoryError::ReadModel {
                file: file.clone(),
                source,
            })?;
        Ok(Self {
            events,
            models,
            app,
            file,
        })
    }

    /// `show`'s read model as this file holds it.
    pub(crate) fn view(&self, show: &ShowId) -> Result<TimelineView, StoryError> {
        TimelineView::read(&self.app, show).map_err(|source| StoryError::ReadModel {
            file: self.file.clone(),
            source,
        })
    }

    /// Brings this file's read model of `show` up to date, and says how many
    /// events that applied. `at` names the place in an error.
    pub(crate) async fn project(&self, at: Place, show: &ShowId) -> Result<usize, StoryError> {
        let mut timeline = Timeline::new(show);
        let progressed = run_projection(
            Source::new(&self.events, self.events.store_id()),
            &self.models,
            &mut timeline,
            &Json,
            Chunk::of(CHUNK),
        )
        .await
        .map_err(|source| StoryError::Project {
            place: at,
            source: Box::new(source),
        })?;
        Ok(progressed.applied)
    }

    /// Every event in this file's log, in its own order.
    pub(crate) async fn log(&self, at: Place) -> Result<Vec<SequencedEvent>, StoryError> {
        collect(self.events.read(&Query::all(), ReadOptions::new()))
            .await
            .map_err(|source| StoryError::Read { place: at, source })
    }
}

/// A device's side of its one sync relationship: the peer it talks to and
/// the edge that remembers how far they got.
///
/// The edge judges nothing it pulls (its rule is the default `Accept`):
/// central is the one place that writes a refund (SY-7), so the rule sits on
/// central's side of the link, on what the device pushes.
#[derive(Debug)]
struct Handset<A> {
    peer: RemoteSyncPeer<Uplink<A>>,
    edge: Edge<ResumeToken>,
}

impl<A: Adjudicate<Error = RuleError>> Handset<A> {
    /// A device linked to `central`, through its own endpoint, whose peer
    /// judges each push by `rule` before it lands.
    fn linked_to(central: &Central, rule: A) -> Self {
        // Two handles onto one store: central's read half and ingest half
        // must be the same store, and a `SqliteEventStore` clone is a second
        // handle onto the same connection, not a copy of the log.
        let far = StoreSyncPeer::new(central.clone(), central.clone()).with_adjudicator(rule);
        let limits = far.limits();
        Self {
            peer: RemoteSyncPeer::new(Uplink::new(far), limits),
            edge: Edge::new(),
        }
    }
}

/// What one sync moved, counted where it landed.
///
/// Counted at the destination rather than on the wire, because a device
/// pushes its whole log, the events it pulled from central included (SY-27),
/// and central skips what it already holds. "Sent three" would be true and
/// would say nothing.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Synced {
    /// Events new to the device's log.
    pub pulled: usize,
    /// Events new to central's log.
    pub landed: usize,
}

/// The story's world: central, the tablet and the phone, each on its own file,
/// one sync edge per device, and the rule central judges every push by.
///
/// Generic over the rule, defaulting to central's own, for one reason: the
/// test that shows the oversell left unanswered mounts a rule that answers
/// nothing, through [`judged_by`](Self::judged_by). The story and the binary
/// only ever call [`open`](Self::open).
#[derive(Debug)]
pub struct Scenario<A = CentralRule> {
    show: ShowId,
    central: Site,
    tablet: Site,
    phone: Site,
    tablet_link: Handset<A>,
    phone_link: Handset<A>,
    rule: A,
}

impl Scenario {
    /// Opens the three files under `dir`, creating them if they are new, with
    /// central judging every push by its own rule.
    ///
    /// # Errors
    ///
    /// [`StoryError::Open`], [`StoryError::OpenModel`] or
    /// [`StoryError::ReadModel`] naming the file that would not open.
    pub fn open(dir: &Path) -> Result<Self, StoryError> {
        Self::judged_by(dir, CentralRule::new)
    }
}

impl<A: Adjudicate<Error = RuleError> + Clone> Scenario<A> {
    /// Opens the three files under `dir`, with central judging every push by
    /// the rule `rule` builds from central's store and the show.
    ///
    /// The rule is cloned once per device, because each device's link holds
    /// a peer of its own, and kept once more for
    /// [`judge_again`](Self::judge_again). Central's rule is two handles, so
    /// a clone is a handle onto the same file and a copy of the show's name.
    ///
    /// # Errors
    ///
    /// As [`open`](Scenario::open).
    pub fn judged_by(
        dir: &Path,
        rule: impl FnOnce(Central, ShowId) -> A,
    ) -> Result<Self, StoryError> {
        let show = ShowId::new(SHOW).map_err(StoryError::Show)?;
        let central = Site::open(dir, Place::Central)?;
        let rule = rule(central.events.clone(), show.clone());
        let tablet_link = Handset::linked_to(&central.events, rule.clone());
        let phone_link = Handset::linked_to(&central.events, rule.clone());
        Ok(Self {
            show,
            tablet: Site::open(dir, Place::Tablet)?,
            phone: Site::open(dir, Place::Phone)?,
            central,
            tablet_link,
            phone_link,
            rule,
        })
    }

    const fn site(&self, at: Place) -> &Site {
        match at {
            Place::Central => &self.central,
            Place::Tablet => &self.tablet,
            Place::Phone => &self.phone,
        }
    }

    const fn handset(&self, device: Device) -> &Handset<A> {
        match device {
            Device::Tablet => &self.tablet_link,
            Device::Phone => &self.phone_link,
        }
    }

    /// Schedules the show on central.
    ///
    /// # Errors
    ///
    /// [`StoryError::Schedule`] if the store fails the append.
    pub async fn schedule(&self, capacity: u32) -> Result<(), StoryError> {
        commit(
            &self.central.events,
            ShowLedger::new(&self.show),
            ATTEMPTS,
            |ledger: &ShowLedger| schedule(ledger, capacity),
        )
        .await
        .map(|_| ())
        .map_err(|source| StoryError::Schedule(Box::new(source)))
    }

    /// Sells a seat at `at` to `buyer`, deciding against the history `at`
    /// holds and nothing else: no round trip, with or without signal.
    ///
    /// # Errors
    ///
    /// [`StoryError::Sell`], carrying the [`Refusal`](crate::Refusal) if the
    /// decision said no, or [`StoryError::NothingSold`] if it decided nothing.
    pub async fn sell(&self, at: Place, buyer: &str) -> Result<SaleId, StoryError> {
        let mut minted = None;
        let outcome = commit(
            &self.site(at).events,
            ShowLedger::new(&self.show),
            ATTEMPTS,
            |ledger: &ShowLedger| {
                let events = sell_seat(ledger, at, buyer)?;
                // The event moves into the append; the caller is handed a
                // copy of its id.
                minted = events.iter().find_map(BoxOffice::sale).cloned();
                Ok(events)
            },
        )
        .await
        .map_err(|source| StoryError::Sell {
            place: at,
            source: Box::new(source),
        })?;
        match (outcome, minted) {
            (CommandOutcome::Committed(_), Some(sale)) => Ok(sale),
            _ => Err(StoryError::NothingSold { place: at }),
        }
    }

    /// Turns a device's signal on or off, or leaves it able to pull only.
    pub fn set_link(&self, device: Device, link: Link) {
        self.handset(device).peer.exchange().set(link);
    }

    /// Syncs a device with central: pull and ingest, then push, step by step
    /// until the edge says both sides hold the same history.
    ///
    /// A failure is returned, not retried: the story wants to show it.
    ///
    /// # Errors
    ///
    /// [`StoryError::Sync`] carrying the step's failure and its class, or
    /// [`StoryError::NotSynced`] if the bound ran out first, or
    /// [`StoryError::Read`] if central's log cannot be counted.
    pub async fn sync(&mut self, device: Device) -> Result<Synced, StoryError> {
        // What landed is counted on central's log before and after: two whole
        // reads, which a log of six events can afford and a real one would
        // replace with a count query.
        let before = self.central.log(Place::Central).await?.len();
        let (site, handset) = match device {
            Device::Tablet => (&self.tablet, &mut self.tablet_link),
            Device::Phone => (&self.phone, &mut self.phone_link),
        };
        let mut synced = Synced::default();
        for _ in 0..MAX_STEPS {
            let stepped = handset
                .edge
                .step(&handset.peer, &site.events)
                .await
                .map_err(|source| StoryError::Sync {
                    device,
                    source: Box::new(source),
                })?;
            synced.pulled = synced.pulled.saturating_add(stepped.ingested.appended);
            if handset.edge.status().is_synced() {
                let after = self.central.log(Place::Central).await?.len();
                synced.landed = after.saturating_sub(before);
                return Ok(synced);
            }
        }
        Err(StoryError::NotSynced {
            device,
            steps: MAX_STEPS,
        })
    }

    /// A device's edge, as of its last step.
    #[must_use]
    pub const fn status(&self, device: Device) -> &SyncStatus {
        self.handset(device).edge.status()
    }

    /// Asks central's rule about everything central holds, as though it were
    /// all pushed again in one group, and says how many events it answered.
    ///
    /// Nothing is written: the rule decides and never writes, and the answer
    /// is only counted. Central's rule answers a history it has already
    /// judged with nothing, so a push delivered twice could not earn a second
    /// refund even if the runner did not skip it (SY-11).
    ///
    /// # Errors
    ///
    /// [`StoryError::Read`] if central's log cannot be read, and
    /// [`StoryError::Adjudicate`] if the rule cannot judge it.
    pub async fn judge_again(&self) -> Result<usize, StoryError> {
        let held = self
            .central
            .log(Place::Central)
            .await?
            .into_iter()
            .map(|event| ReplicatedEvent::new(event.id, event.recorded_at, event.event))
            .collect();
        let answered = self
            .rule
            .compensation(&EventGroup::new(None, held), &[])
            .await
            .map_err(StoryError::Adjudicate)?;
        Ok(answered.len())
    }

    /// Brings the read model up to date on all three files, and says how many
    /// events that applied in all.
    ///
    /// # Errors
    ///
    /// [`StoryError::Project`] naming the place that failed.
    pub async fn project(&self) -> Result<usize, StoryError> {
        let mut applied = 0usize;
        for at in Place::ALL {
            let progressed = self.site(at).project(at, &self.show).await?;
            applied = applied.saturating_add(progressed);
        }
        Ok(applied)
    }

    /// The read model as `at` holds it.
    ///
    /// # Errors
    ///
    /// [`StoryError::ReadModel`] if the table cannot be read.
    pub fn view(&self, at: Place) -> Result<TimelineView, StoryError> {
        self.site(at).view(&self.show)
    }

    /// The show, folded from the history `at` holds.
    ///
    /// # Errors
    ///
    /// [`StoryError::Read`] or [`StoryError::Decode`] naming the place.
    pub async fn ledger(&self, at: Place) -> Result<ShowLedger, StoryError> {
        let mut ledger = ShowLedger::new(&self.show);
        let query = ledger.query()?;
        let (found, _) = read_decision_model(&self.site(at).events, &query)
            .await
            .map_err(|source| StoryError::Read { place: at, source })?;
        for event in &found {
            ledger
                .absorb(event, &Json)
                .map_err(|source| StoryError::Decode { place: at, source })?;
        }
        Ok(ledger)
    }

    /// Every event in `at`'s log.
    ///
    /// # Errors
    ///
    /// [`StoryError::Read`] naming the place.
    pub async fn log(&self, at: Place) -> Result<Vec<SequencedEvent>, StoryError> {
        self.site(at).log(at).await
    }
}
