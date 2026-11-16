//! The box office: who sells, what happens to a show, and the one rule.
//!
//! Nothing in this module knows about SQLite or sync. The same decision model
//! runs on a device, offline, and on central, over the whole history; that it
//! is one model is the point (the pillar *true to your rules*).

use core::convert::Infallible;
use core::fmt;
use std::collections::BTreeMap;

use happenstance::bytes::Bytes;
use happenstance::{
    Codec, CodecError, DecisionModel, DomainEvent, EventType, InvalidTag, Tag, Tags,
};
use serde::{Deserialize, Serialize};

/// Every place that holds a copy of the log and can sell a seat.
///
/// A closed set, so every `match` on it names all three.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Place {
    /// The central store: the box office's own till, and the one place that
    /// sees every device's history once they sync.
    Central,
    /// The box-office tablet, a store of its own on its own file.
    Tablet,
    /// A phone, likewise.
    Phone,
}

impl Place {
    /// All three, central first, in the order the story prints them.
    pub const ALL: [Self; 3] = [Self::Central, Self::Tablet, Self::Phone];

    /// The name a sale id and the transcript use.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Central => "central",
            Self::Tablet => "tablet",
            Self::Phone => "phone",
        }
    }
}

impl fmt::Display for Place {
    /// `pad` rather than `write_str`, so `{:<8}` aligns a column.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.as_str())
    }
}

impl core::str::FromStr for Place {
    type Err = UnknownPlace;

    fn from_str(name: &str) -> Result<Self, UnknownPlace> {
        Self::ALL
            .into_iter()
            .find(|place| place.as_str() == name)
            .ok_or(UnknownPlace)
    }
}

/// A place name that is none of the three.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("not a place: expected central, tablet or phone")]
pub struct UnknownPlace;

/// The two places that sync with central. Central does not sync with itself,
/// so a function taking a `Device` cannot be asked to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Device {
    /// The box-office tablet.
    Tablet,
    /// The phone.
    Phone,
}

impl Device {
    /// The device that is not this one.
    #[must_use]
    pub const fn other(self) -> Self {
        match self {
            Self::Tablet => Self::Phone,
            Self::Phone => Self::Tablet,
        }
    }
}

impl From<Device> for Place {
    fn from(device: Device) -> Self {
        match device {
            Device::Tablet => Self::Tablet,
            Device::Phone => Self::Phone,
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&Place::from(*self), f)
    }
}

/// A show, validated once into the tag every one of its events carries.
///
/// The shape `transfers-on-sqlite` reaches for its accounts: `DomainEvent::tags`
/// cannot fail and `Tag::key_value` can, so the type holds the validated tag
/// rather than rebuilding it on every call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ShowId {
    name: String,
    tag: Tag,
}

impl ShowId {
    /// Validates `name` into the show's tag.
    ///
    /// # Errors
    ///
    /// [`InvalidTag`] if `name` cannot be a tag value.
    pub fn new(name: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            name: name.to_owned(),
            tag: Tag::key_value("show", name)?,
        })
    }

    /// The name as the application wrote it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }

    /// The scope every model and view of this show reads: its one tag.
    #[must_use]
    pub fn scope(&self) -> Tags {
        [self.tag.clone()].into_iter().collect()
    }
}

impl TryFrom<String> for ShowId {
    type Error = InvalidTag;

    fn try_from(name: String) -> Result<Self, InvalidTag> {
        Self::new(&name)
    }
}

impl From<ShowId> for String {
    fn from(show: ShowId) -> Self {
        show.name
    }
}

impl fmt::Display for ShowId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

/// One sale, named by the place that made it and its count there:
/// `central-1`, `tablet-1`, `phone-1`.
///
/// Minted by the decision from the history it folded, so a run's ids are the
/// same every time and the transcript can be quoted. Not a tag: nothing queries
/// by sale, and a field that is never queried does not need to be indexed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SaleId(String);

impl SaleId {
    /// The `n`th sale made at `at`.
    fn mint(at: Place, n: usize) -> Self {
        Self(format!("{at}-{n}"))
    }

    /// The id as the transcript prints it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The id, given up as the string it is.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for SaleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Everything that can happen at the box office.
///
/// One enum, so a query and a fold cannot name different sets, and no `_ =>`
/// arm anywhere in this crate: a fourth variant is a compile error in every
/// fold that has not been taught it.
///
/// # The payload's shape is pinned
///
/// Every payload is an object that names itself, `{"type": "seat_sold",
/// "show": ..., ...}`. The attribute says so instead of leaving it to serde's
/// default, which keys the object by the Rust variant's name: under that
/// default, renaming a variant would silently change what goes into the log,
/// while here it changes nothing on disk.
///
/// A v2 of an event keeps every v1 already in a log readable, and the log is
/// never rewritten. The changed event takes a new event type, and
/// [`decode`](DomainEvent::decode) starts reading the event-type argument it
/// ignores today: the old type decodes into a private DTO of the v1 shape,
/// mapped by a total `upcast` into this enum, so no fold ever sees two shapes.
/// Dispatching on the type rather than trying one shape and then the other
/// means a payload that fits neither is reported, not guessed at.
/// `telemetry-across-codecs` upcasts a v1 shape in exactly this place.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BoxOffice {
    /// A show exists and has this many seats.
    ShowScheduled {
        /// The show.
        show: ShowId,
        /// How many seats it has.
        capacity: u32,
    },
    /// A seat was sold. Holding the seat is the sale's until a refund says
    /// otherwise.
    SeatSold {
        /// The show.
        show: ShowId,
        /// Which sale this is.
        sale: SaleId,
        /// Who bought it.
        buyer: String,
        /// Where it was sold.
        sold_by: Place,
    },
    /// A sale was refunded, and why. The compensation SY-2 describes: an
    /// ordinary append, which says what happened instead of rewriting it.
    SaleRefunded {
        /// The show.
        show: ShowId,
        /// The sale refunded.
        sale: SaleId,
        /// Why, as data, so every replica prints the same audit line.
        reason: RefundReason,
    },
}

impl BoxOffice {
    /// The sale this event is about, if it is about one.
    #[must_use]
    pub const fn sale(&self) -> Option<&SaleId> {
        match self {
            Self::ShowScheduled { .. } => None,
            Self::SeatSold { sale, .. } | Self::SaleRefunded { sale, .. } => Some(sale),
        }
    }
}

const SHOW_SCHEDULED: EventType = EventType::from_static("ShowScheduled");
const SEAT_SOLD: EventType = EventType::from_static("SeatSold");
/// The refund's event type, which the read-back counts by.
pub(crate) const SALE_REFUNDED: EventType = EventType::from_static("SaleRefunded");

impl DomainEvent for BoxOffice {
    const EVENT_TYPES: &'static [EventType] = &[SHOW_SCHEDULED, SEAT_SOLD, SALE_REFUNDED];

    fn event_type(&self) -> EventType {
        match self {
            Self::ShowScheduled { .. } => SHOW_SCHEDULED,
            Self::SeatSold { .. } => SEAT_SOLD,
            Self::SaleRefunded { .. } => SALE_REFUNDED,
        }
    }

    fn tags(&self) -> Tags {
        match self {
            Self::ShowScheduled { show, .. }
            | Self::SeatSold { show, .. }
            | Self::SaleRefunded { show, .. } => show.scope(),
        }
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(
        codec: &C,
        _event_type: &EventType,
        data: &Bytes,
    ) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

/// Why central refunded a sale: what it saw, and which rule it applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefundReason {
    /// The show's capacity.
    pub capacity: u32,
    /// How many sales were holding a seat when central decided.
    pub sales_seen: u32,
    /// The rule that chose this sale.
    pub rule: Rule,
}

impl fmt::Display for RefundReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} sales reached central for {} seats; rule: {}",
            self.sales_seen, self.capacity, self.rule
        )
    }
}

/// The rules central can refund by. One today, and an enum rather than a
/// string so the audit line is data a fold can match on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rule {
    /// The sales that reached central first keep their seats, by central's
    /// own log order; the ones past capacity are refunded.
    FirstToReachCentralKeepsTheSeat,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::FirstToReachCentralKeepsTheSeat => {
                "the first sale to reach central keeps the seat"
            }
        })
    }
}

/// Why a sale was refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Refusal {
    /// The show has not reached this store yet.
    #[error("show {show} is not scheduled here yet")]
    NotScheduled {
        /// The show asked for.
        show: String,
    },
    /// Every seat is held.
    #[error("sold out: {held} of {capacity} seats are held")]
    SoldOut {
        /// The show's capacity.
        capacity: u32,
        /// Seats held, as this store knows them.
        held: u32,
    },
}

/// One sale as a ledger keeps it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    /// Which sale.
    pub sale: SaleId,
    /// Who bought it.
    pub buyer: String,
    /// Where it was sold.
    pub sold_by: Place,
}

/// One show, as the history a store holds tells it.
///
/// A decision model over **one** log, folded in that log's own order. On
/// central that order is the order sales arrived in, which is what the rule
/// refunds by. It is not a convergent fold, and does not need to be: the
/// decision it informs is written down as an event, and that event is what
/// replicates.
#[derive(Debug, Clone)]
pub struct ShowLedger {
    show: ShowId,
    scope: Tags,
    capacity: Option<u32>,
    sales: Vec<Sale>,
    refunds: BTreeMap<SaleId, RefundReason>,
}

impl ShowLedger {
    /// An empty ledger for `show`.
    #[must_use]
    pub fn new(show: &ShowId) -> Self {
        Self {
            show: show.clone(),
            scope: show.scope(),
            capacity: None,
            sales: Vec::new(),
            refunds: BTreeMap::new(),
        }
    }

    /// The show's capacity, once it is scheduled here.
    #[must_use]
    pub const fn capacity(&self) -> Option<u32> {
        self.capacity
    }

    /// Every sale, in this log's order, refunded ones included.
    pub fn sales(&self) -> impl Iterator<Item = &Sale> + '_ {
        self.sales.iter()
    }

    /// The sales still holding a seat, in this log's order.
    pub fn holding(&self) -> impl Iterator<Item = &Sale> + '_ {
        self.sales
            .iter()
            .filter(|sale| !self.refunds.contains_key(&sale.sale))
    }

    /// The sales refunded, by id.
    pub fn refunded(&self) -> impl Iterator<Item = &SaleId> + '_ {
        self.refunds.keys()
    }

    /// Why `sale` was refunded, if it was.
    #[must_use]
    pub fn refund_reason(&self, sale: &SaleId) -> Option<&RefundReason> {
        self.refunds.get(sale)
    }

    /// How many seats are held. Saturates at `u32::MAX`, which a show's
    /// capacity can never reach past.
    #[must_use]
    pub fn held(&self) -> u32 {
        u32::try_from(self.holding().count()).unwrap_or(u32::MAX)
    }
}

impl DecisionModel for ShowLedger {
    type Event = BoxOffice;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: BoxOffice) {
        match event {
            BoxOffice::ShowScheduled { capacity, .. } => self.capacity = Some(capacity),
            BoxOffice::SeatSold {
                sale,
                buyer,
                sold_by,
                ..
            } => self.sales.push(Sale {
                sale,
                buyer,
                sold_by,
            }),
            BoxOffice::SaleRefunded { sale, reason, .. } => {
                self.refunds.insert(sale, reason);
            }
        }
    }
}

/// The decision behind scheduling the show: one `ShowScheduled`, or nothing
/// if this store already holds one, so scheduling twice is not an error.
///
/// # Errors
///
/// None: the error type is [`Infallible`].
pub fn schedule(ledger: &ShowLedger, capacity: u32) -> Result<Vec<BoxOffice>, Infallible> {
    Ok(match ledger.capacity {
        Some(_) => Vec::new(),
        None => vec![BoxOffice::ShowScheduled {
            show: ledger.show.clone(),
            capacity,
        }],
    })
}

/// The decision behind a sale at `at`: one `SeatSold` if a seat is free in the
/// history this store holds, a [`Refusal`] if not.
///
/// The sale's id is minted here, from the same fold, so a retried decision
/// mints it again from the history it re-read.
///
/// # Errors
///
/// [`Refusal::NotScheduled`] before the show reaches this store, and
/// [`Refusal::SoldOut`] when every seat is held.
pub fn sell_seat(ledger: &ShowLedger, at: Place, buyer: &str) -> Result<Vec<BoxOffice>, Refusal> {
    let Some(capacity) = ledger.capacity else {
        return Err(Refusal::NotScheduled {
            show: ledger.show.to_string(),
        });
    };
    let held = ledger.held();
    if held >= capacity {
        return Err(Refusal::SoldOut { capacity, held });
    }
    let made_here = ledger.sales().filter(|sale| sale.sold_by == at).count();
    Ok(vec![BoxOffice::SeatSold {
        show: ledger.show.clone(),
        sale: SaleId::mint(at, made_here.saturating_add(1)),
        buyer: buyer.to_owned(),
        sold_by: at,
    }])
}

/// Central's decision over the whole history: one `SaleRefunded` for each
/// sale past capacity, refunding from the most recent arrival, and **nothing**
/// when no seat is oversold, which `commit` reports as
/// `CommandOutcome::Nothing`.
///
/// # Errors
///
/// None: the error type is [`Infallible`], which says in the signature that
/// central's rule always has an answer.
pub fn adjudicate(ledger: &ShowLedger) -> Result<Vec<BoxOffice>, Infallible> {
    let Some(capacity) = ledger.capacity else {
        return Ok(Vec::new());
    };
    let sales_seen = ledger.held();
    let keep = usize::try_from(capacity).unwrap_or(usize::MAX);
    Ok(ledger
        .holding()
        .skip(keep)
        .map(|sale| BoxOffice::SaleRefunded {
            show: ledger.show.clone(),
            sale: sale.sale.clone(),
            reason: RefundReason {
                capacity,
                sales_seen,
                rule: Rule::FirstToReachCentralKeepsTheSeat,
            },
        })
        .collect())
}

#[cfg(test)]
mod tests;
