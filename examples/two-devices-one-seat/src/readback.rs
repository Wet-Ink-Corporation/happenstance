//! The three files read back after every handle is dropped.
//!
//! Nothing is carried in but the directory. Each file is opened fresh, its log
//! read whole and its read model read through a new connection, so what this
//! reports came off disk and not out of a value the story was still holding.

use std::collections::BTreeSet;
use std::path::Path;

use happenstance::{EventId, StoreId};

use crate::domain::{Place, SALE_REFUNDED, ShowId};
use crate::error::StoryError;
use crate::scenario::{SHOW, Site};
use crate::timeline::TimelineView;

/// One place's file, read back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Replica {
    /// The store's own identity, minted when its file was created.
    pub store: StoreId,
    /// Every event's origin identity. Positions are local to each store, so
    /// the identity is what two logs can be compared by.
    pub ids: BTreeSet<EventId>,
    /// The refunds, by identity, in this log's order.
    pub refunds: Vec<EventId>,
    /// The read model.
    pub view: TimelineView,
}

impl Replica {
    async fn read(dir: &Path, at: Place, show: &ShowId) -> Result<Self, StoryError> {
        let site = Site::open(dir, at)?;
        let log = site.log(at).await?;
        Ok(Self {
            store: site.events.store_id(),
            ids: log.iter().map(|event| event.id).collect(),
            refunds: log
                .iter()
                .filter(|event| event.event_type() == &SALE_REFUNDED)
                .map(|event| event.id)
                .collect(),
            view: site.view(show)?,
        })
    }
}

/// All three files, read back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Converged {
    /// Central's file. Its [`store`](Replica::store) is the identity every
    /// refund's id must name (SY-7).
    pub central: Replica,
    /// The tablet's file.
    pub tablet: Replica,
    /// The phone's file.
    pub phone: Replica,
}

impl Converged {
    /// Opens the three files under `dir` afresh and reads them, the read
    /// model as it stands for the story's show.
    ///
    /// # Errors
    ///
    /// Any [`StoryError`] opening or reading a file, naming the file or place.
    pub async fn read(dir: &Path) -> Result<Self, StoryError> {
        let show = ShowId::new(SHOW).map_err(StoryError::Show)?;
        Ok(Self {
            central: Replica::read(dir, Place::Central, &show).await?,
            tablet: Replica::read(dir, Place::Tablet, &show).await?,
            phone: Replica::read(dir, Place::Phone, &show).await?,
        })
    }

    /// Each place and its replica, central first.
    #[must_use]
    pub const fn replicas(&self) -> [(Place, &Replica); 3] {
        [
            (Place::Central, &self.central),
            (Place::Tablet, &self.tablet),
            (Place::Phone, &self.phone),
        ]
    }

    /// Whether the three logs hold the same events and the three read models
    /// are identical.
    #[must_use]
    pub fn agrees(&self) -> bool {
        self.replicas().iter().all(|(_, replica)| {
            replica.ids == self.central.ids && replica.view == self.central.view
        })
    }
}
