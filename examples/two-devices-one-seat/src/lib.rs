#![doc = include_str!("overview.md")]
//!
//! # Where things are
//!
//! The modules are private and every public item is re-exported here, so
//! each has one path. [`BoxOffice`] and [`ShowLedger`] are the domain, with
//! its decisions [`schedule`], [`sell_seat`] and [`adjudicate`]; [`Uplink`]
//! is the device's link to central; [`Timeline`] the read model; [`Scenario`]
//! the three files and the steps; [`Converged`] the files read back with every
//! handle dropped; [`tell`] the beats, told into any writer; and
//! [`StoryError`] what any of it can fail with.

mod domain;
mod error;
mod readback;
mod scenario;
mod story;
mod timeline;
mod uplink;

pub use domain::{
    BoxOffice, Device, Place, RefundReason, Refusal, Rule, Sale, SaleId, ShowId, ShowLedger,
    UnknownPlace, adjudicate, schedule, sell_seat,
};
pub use error::{DeviceStep, StoryError};
pub use readback::{Converged, Replica};
pub use scenario::{CAPACITY, SHOW, Scenario, Synced};
pub use story::tell;
pub use timeline::{Row, Status, Timeline, TimelineView};
pub use uplink::{Central, CentralError, CentralPeer, Link, Uplink, UplinkError};
