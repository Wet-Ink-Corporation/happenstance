//! The read model every place keeps: each sale, whether it holds a seat, and
//! why not if it does not.
//!
//! # Why it converges
//!
//! The three logs hold the same events in different orders: a device holds
//! its own sale before the other device's, central holds them in arrival
//! order. So [`Timeline`] is written to give the same table under **any**
//! order, and declares [`ConvergentProjection`] because that is true rather
//! than because it is convenient (SY-20):
//!
//! * a sale inserts its row as `held` and never touches the status of a row
//!   that is already there, so a refund that arrived first stays a refund;
//! * a refund sets `refunded` whatever is there, inserting the row if the sale
//!   has not arrived yet;
//! * nothing is ever incremented. Seats held are counted when the view is
//!   read, so arrival order cannot change the count.
//!
//! It reads only the event, never its position (local to each store) and
//! never state of its own. The price is that it cannot say *when*: a
//! convergent fold has no clock every replica agrees on, so the rows print in
//! an order every replica shares (refunds last, then by sale id) rather than
//! in time order.

use happenstance::{ConvergentProjection, Delivered, Projection, ProjectionId, Tags};
use happenstance_sqlite::projection_store::{
    SqliteBatch, SqliteProjectionStore, SqliteProjectionStoreError,
};
use rusqlite::Connection;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, Value, ValueRef};

use crate::domain::{BoxOffice, Place, ShowId};

/// The read model's two tables, created by the application: the projection
/// store migrates its checkpoint and nothing else.
///
/// A sale's row is keyed by its show as well as its id, because a [`SaleId`]
/// counts sales within one show: a second show's first tablet sale would be
/// `tablet-1` too.
///
/// [`SaleId`]: crate::SaleId
pub(crate) const DDL: &str = "
CREATE TABLE IF NOT EXISTS show_seats (
    show     TEXT PRIMARY KEY,
    capacity INTEGER NOT NULL
) STRICT;
CREATE TABLE IF NOT EXISTS seat_ledger (
    show    TEXT NOT NULL,
    sale    TEXT NOT NULL,
    buyer   TEXT,
    sold_by TEXT,
    status  TEXT NOT NULL CHECK (status IN ('held', 'refunded')),
    reason  TEXT,
    PRIMARY KEY (show, sale)
) STRICT;
";

/// The projection's name, validated by the compiler.
const SEAT_TIMELINE: ProjectionId = ProjectionId::from_static("seat_timeline");

/// The read model's projection, over one show.
#[derive(Debug)]
pub struct Timeline {
    id: ProjectionId,
    scope: Tags,
}

impl Timeline {
    /// The projection over `show`'s events.
    #[must_use]
    pub fn new(show: &ShowId) -> Self {
        Self {
            id: SEAT_TIMELINE,
            scope: show.scope(),
        }
    }
}

impl Projection for Timeline {
    type Event = BoxOffice;
    type Store = SqliteProjectionStore;
    type Error = SqliteProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<BoxOffice>,
        batch: &mut SqliteBatch,
    ) -> Result<(), SqliteProjectionStoreError> {
        match event.into_event() {
            BoxOffice::ShowScheduled { show, capacity } => batch.push(
                "INSERT INTO show_seats (show, capacity) VALUES (?1, ?2) \
                 ON CONFLICT (show) DO NOTHING",
                [Value::Text(show.into()), Value::Integer(capacity.into())],
            ),
            BoxOffice::SeatSold {
                show,
                sale,
                buyer,
                sold_by,
            } => batch.push(
                "INSERT INTO seat_ledger (show, sale, buyer, sold_by, status) \
                 VALUES (?1, ?2, ?3, ?4, 'held') \
                 ON CONFLICT (show, sale) DO UPDATE \
                 SET buyer = excluded.buyer, sold_by = excluded.sold_by",
                [
                    Value::Text(show.into()),
                    Value::Text(sale.into_string()),
                    Value::Text(buyer),
                    Value::Text(sold_by.as_str().to_owned()),
                ],
            ),
            BoxOffice::SaleRefunded { show, sale, reason } => batch.push(
                "INSERT INTO seat_ledger (show, sale, status, reason) \
                 VALUES (?1, ?2, 'refunded', ?3) \
                 ON CONFLICT (show, sale) DO UPDATE \
                 SET status = 'refunded', reason = excluded.reason",
                [
                    Value::Text(show.into()),
                    Value::Text(sale.into_string()),
                    Value::Text(reason.to_string()),
                ],
            ),
        }
        Ok(())
    }
}

/// True of [`Timeline`] by construction, as the module says.
impl ConvergentProjection for Timeline {}

/// Whether a sale holds its seat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// It does.
    Held,
    /// It was refunded.
    Refunded,
}

impl FromSql for Status {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "held" => Ok(Self::Held),
            "refunded" => Ok(Self::Refunded),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl FromSql for Place {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value
            .as_str()?
            .parse()
            .map_err(|error| FromSqlError::Other(Box::new(error)))
    }
}

/// One sale, as the read model holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// Which sale.
    pub sale: String,
    /// Who bought it; `None` only while a refund has arrived ahead of its sale.
    pub buyer: Option<String>,
    /// Where it was sold, likewise.
    pub sold_by: Option<Place>,
    /// Whether it holds its seat.
    pub status: Status,
    /// Why it was refunded, if it was.
    pub reason: Option<String>,
}

/// The read model as one place holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelineView {
    /// The show's capacity, once scheduled.
    pub capacity: Option<u32>,
    /// Every sale, refunds last, then by sale id.
    pub rows: Vec<Row>,
}

impl TimelineView {
    /// Reads the view through the application's own connection.
    ///
    /// # Errors
    ///
    /// The driver's error, if a table is missing or a row does not decode.
    pub fn read(app: &Connection) -> rusqlite::Result<Self> {
        let capacity = app
            .prepare("SELECT capacity FROM show_seats")?
            .query_map([], |row| row.get::<_, u32>(0))?
            .next()
            .transpose()?;
        let rows = app
            .prepare(
                "SELECT sale, buyer, sold_by, status, reason FROM seat_ledger \
                 ORDER BY status = 'refunded', sale",
            )?
            .query_map([], |row| {
                Ok(Row {
                    sale: row.get(0)?,
                    buyer: row.get(1)?,
                    sold_by: row.get(2)?,
                    status: row.get(3)?,
                    reason: row.get(4)?,
                })
            })?
            .collect::<rusqlite::Result<_>>()?;
        Ok(Self { capacity, rows })
    }

    /// Seats held, counted now rather than kept as a running total. Every
    /// count here saturates at `u32::MAX`, which no show reaches.
    #[must_use]
    pub fn held(&self) -> u32 {
        self.count(Status::Held)
    }

    /// Sales refunded.
    #[must_use]
    pub fn refunds(&self) -> u32 {
        self.count(Status::Refunded)
    }

    /// Every sale the view knows of, refunded or not.
    #[must_use]
    pub fn sales(&self) -> u32 {
        u32::try_from(self.rows.len()).unwrap_or(u32::MAX)
    }

    fn count(&self, status: Status) -> u32 {
        let n = self.rows.iter().filter(|row| row.status == status).count();
        u32::try_from(n).unwrap_or(u32::MAX)
    }
}

#[cfg(test)]
mod tests;
