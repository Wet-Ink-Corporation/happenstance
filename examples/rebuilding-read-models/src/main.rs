//! Operating derived state: four views over one log, on one SQLite file.
//!
//! `transfers-on-sqlite` proves a projection survives a reopen. It runs *one*
//! view, and a single view is the one arrangement in which none of the
//! questions an operator actually has can be asked: it is never skewed against
//! a peer, never rebuilt, never reset, and never the one that breaks while the
//! others must keep going.
//!
//! This program has four views over one log and does the four things to them
//! that a running system eventually demands.
//!
//! Run with `cargo run -p rebuilding-read-models`.
//!
//! # What this demonstrates that the other examples cannot
//!
//! 1. **Two views sit at two checkpoints, and a reader must tolerate it.** A
//!    checkpoint is per `(store, ProjectionId)`. Halfway through, this program
//!    advances one view and not the other and then joins the two tables, so
//!    the stale row is visible rather than argued about.
//! 2. **A view added late is backfilled beside the one it replaces.** The
//!    typed runner always claims `Authority::Live`, so a rebuild is bought
//!    with a *second* `ProjectionId` writing a *second* table — blue and
//!    green — and the swap is the application's, at the moment it chooses.
//! 3. **Reset is a real operation, and rebuilding is chunk-size invariant.**
//!    `ProjectionStore::reset` clears the read model and the checkpoint as one
//!    unit and leaves `Checkpoint::NeverRun` behind — not a checkpoint at the
//!    first position, which is how event 1 gets skipped permanently and
//!    silently. Rebuilt at chunk 1 and at chunk 500, the rows are identical.
//! 4. **One poisoned view does not stall the others.** The failure is the one
//!    that actually happens in production: a projection written against a
//!    field the log does not carry. It fails to *decode*, names the position
//!    it failed at, commits nothing, and the other three carry on — because
//!    the failure policy is this application's and not the runner's.
//!
//! # What it does not demonstrate, stated so nobody reads more into it
//!
//! It is one process and it never crashes. `transfers-on-sqlite` owns the
//! durability claim and `tickets-over-http` owns the two-process one; the
//! subject here is what an operator does to a view that is already working.
//! Nor is the poisoned view a claim about fault *injection*: nothing here
//! interrupts a commit halfway. That belongs to the conformance suite.
//!
//! # The domain, and why it is not another ledger
//!
//! Order fulfilment, because it is the smallest domain that naturally wants
//! *several* views of one log that disagree about what matters — stock is
//! per-SKU, status is per-order, and the dispatch count is per-day and belongs
//! to neither. Three shapes over one event set is the situation the
//! projection port exists for, and one shape cannot demonstrate it.

#![allow(clippy::print_stdout, reason = "the transcript is the point")]

use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use happenstance::bytes::Bytes;
use happenstance::{
    Checkpoint, Codec, CodecError, CommandError, DecisionModel, DomainEvent, EventStore, EventType,
    InvalidTag, Json, Projection, ProjectionError, ProjectionId, ProjectionStore, Query,
    ReadOptions, Retry, Tag, Tags, collect, commit, run_projection,
};
use happenstance_sqlite::connection::open_configured;
use happenstance_sqlite::event_store::{SqliteEventStore, SqliteEventStoreError};
use happenstance_sqlite::projection_store::{
    SqliteBatch, SqliteProjectionStore, SqliteProjectionStoreError,
};
use rusqlite::Connection;
use rusqlite::types::Value;
use serde::{Deserialize, Serialize};

/// How many times a command may be attempted before it gives up.
///
/// Spelled at every call site for the reason `course-subscriptions` gives: a
/// loop whose only exit is success is a hang with better manners. This run is
/// single-writer, so the bound is never spent.
const ATTEMPTS: u32 = 3;

/// The chunk the ordinary catch-up runs use.
///
/// Small enough that a first run commits more than once, which is the only way
/// a checkpoint's *intermediate* positions are ever written.
const CHUNK: usize = 4;

/// The two chunk sizes a rebuild is run at, to show the rows do not depend on
/// how the replay was divided.
///
/// One is the smallest legal chunk — a commit per event — and the other is
/// larger than this log will ever be, so the whole replay lands in a single
/// batch. If those two disagree, the fold is carrying state it should not.
const REBUILD_CHUNKS: [usize; 2] = [1, 500];

/// The two days this run dispatches on.
///
/// Literals rather than a clock. A transcript that changes with the calendar
/// is a transcript nobody can diff, and the daily view exists to be counted,
/// not to be current.
const DAYS: [&str; 2] = ["2026-09-01", "2026-09-02"];

/// The five read-model tables this application owns.
///
/// They are the application's tables, not the adapter's: `SqliteProjectionStore`
/// migrates `projection_checkpoint` and `projection_meta` and stops there. A
/// projection store that invented your read model's schema would be a framework
/// rather than a port — which is also why the blue/green swap below is plain
/// SQL this file writes, and not a method somebody else designed.
const READ_MODEL_DDL: &str = "
CREATE TABLE IF NOT EXISTS stock_on_hand (
    sku      TEXT PRIMARY KEY,
    on_hand  INTEGER NOT NULL
) STRICT;
CREATE TABLE IF NOT EXISTS order_status (
    order_ref TEXT PRIMARY KEY,
    status    TEXT NOT NULL,
    sku       TEXT NOT NULL,
    quantity  INTEGER NOT NULL
) STRICT;
CREATE TABLE IF NOT EXISTS daily_dispatches (
    day        TEXT PRIMARY KEY,
    dispatched INTEGER NOT NULL
) STRICT;
CREATE TABLE IF NOT EXISTS daily_dispatches_v2 (
    day        TEXT PRIMARY KEY,
    dispatched INTEGER NOT NULL
) STRICT;
CREATE TABLE IF NOT EXISTS couriered (
    order_ref TEXT PRIMARY KEY,
    courier   TEXT NOT NULL
) STRICT;
";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let path = database_path();
    remove_database(&path);

    println!("== the database this run writes to ==");
    println!("   {}", path.display());

    // The run is a function so that every store, connection and projection it
    // builds is dropped before the file is removed — on Windows an open handle
    // is what makes the cleanup silently fail and the next run start on the
    // tail of this one.
    let outcome = run(&path).await;
    remove_database(&path);
    outcome
}

/// Everything this program does, inside one scope over one file.
async fn run(path: &Path) -> Result<()> {
    let events = SqliteEventStore::open(path)?;
    let models = SqliteProjectionStore::open(path)?;
    let app = open_configured(path)?;
    app.execute_batch(READ_MODEL_DDL)?;

    seed_the_log(&events).await?;
    two_views_two_checkpoints(&events, &models, &app).await?;
    a_third_view_added_late(&events, &models, &app).await?;
    reset_and_rebuild(&events, &models, &app).await?;
    one_poisoned_view(&events, &models).await?;

    println!("\n== final log ==");
    print_log(&events).await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// The five things this program does
// ---------------------------------------------------------------------------

/// Writes the events every view below is derived from.
///
/// One refusal is exercised on the way, so the log is the product of decisions
/// rather than of appends: an order for more units than the shelf holds is
/// rejected against the same boundary that would have let it through.
async fn seed_the_log(events: &SqliteEventStore) -> Result<()> {
    println!("\n== seeding the fulfilment log ==");

    receive_stock(events, "sku-1", 13).await?;
    receive_stock(events, "sku-2", 6).await?;
    place_order(events, "o-1", "sku-1", 3).await?;
    place_order(events, "o-2", "sku-1", 2).await?;
    place_order(events, "o-3", "sku-2", 4).await?;
    dispatch_order(events, "o-1", DAYS[0]).await?;
    dispatch_order(events, "o-2", DAYS[0]).await?;
    println!("   two SKUs received, three orders placed, two dispatched");

    match place_order(events, "o-9", "sku-2", 5).await {
        Ok(()) => bail!("an order against an emptied shelf should have been rejected"),
        Err(err) => println!("   rejected: {err}"),
    }

    Ok(())
}

/// Advances one view and not the other, and shows what a reader then sees.
///
/// The skew is not a defect being confessed to — it is the arrangement. A
/// checkpoint is per `(store, ProjectionId)`, so two views over one log are
/// two independent positions, and an application that joins them is reading
/// two different moments. Printing both checkpoints beside the rows is the
/// only way to say that without it sounding theoretical.
async fn two_views_two_checkpoints(
    events: &SqliteEventStore,
    models: &SqliteProjectionStore,
    app: &Connection,
) -> Result<()> {
    println!("\n== two views over one log, at two checkpoints ==");

    let mut stock = StockOnHand::new();
    let mut status = OrderStatus::new();
    let applied_stock = drive(events, models, &mut stock, CHUNK).await?;
    let applied_status = drive(events, models, &mut status, CHUNK).await?;
    println!("   stock_on_hand applied {applied_stock}, order_status applied {applied_status}");

    // Both are caught up here, which is the only state in which the skew below
    // is attributable to what this function does next rather than to how they
    // were built.
    print_checkpoints(models, &[stock.id(), status.id()]).await?;

    println!("\n   -- a fourth order is placed, and only one view is run --");
    place_order(events, "o-4", "sku-1", 5).await?;
    let applied_stock = drive(events, models, &mut stock, CHUNK).await?;
    println!("   stock_on_hand applied {applied_stock}, order_status was not run");
    print_checkpoints(models, &[stock.id(), status.id()]).await?;

    println!("\n   stock_on_hand:");
    print_table(app, "SELECT sku, on_hand FROM stock_on_hand ORDER BY sku")?;
    println!("   order_status:");
    print_table(
        app,
        "SELECT order_ref, status FROM order_status ORDER BY order_ref",
    )?;
    println!(
        "   o-4's five units are already off the shelf and it has no status row:\n   \
         a join across these two tables is a join across two moments"
    );

    println!("\n   -- order_status is brought level --");
    let applied_status = drive(events, models, &mut status, CHUNK).await?;
    println!("   order_status applied {applied_status}");
    print_checkpoints(models, &[stock.id(), status.id()]).await?;

    Ok(())
}

/// Backfills a view that did not exist when the log was written, then promotes it.
///
/// The typed runner always claims `Authority::Live`, so there is no in-place
/// rebuild to reach for and this is not a limitation being worked around: a
/// second `ProjectionId` writing a second table is blue/green, and it is the
/// arrangement that lets the old view keep answering queries the whole time.
///
/// The promotion is plain SQL this file writes, because the read model belongs
/// to this application. **The `ProjectionId` deliberately does not change with
/// it.** `daily_dispatches_v2` keeps its name after the swap, because the
/// checkpoint is keyed by that id — renaming it to look tidy would throw the
/// resume point away and replay the whole log.
async fn a_third_view_added_late(
    events: &SqliteEventStore,
    models: &SqliteProjectionStore,
    app: &Connection,
) -> Result<()> {
    println!("\n== a third view, added after the fact ==");

    let mut blue = DailyDispatches::counting_orders();
    let applied = drive(events, models, &mut blue, CHUNK).await?;
    println!("   daily_dispatches backfilled from zero: applied {applied}");
    print_table(
        app,
        "SELECT day, dispatched FROM daily_dispatches ORDER BY day",
    )?;

    println!("\n   -- the count was wrong: it wanted units, not orders --");
    let mut green = DailyDispatches::counting_units();
    let applied = drive(events, models, &mut green, CHUNK).await?;
    println!("   daily_dispatches_v2 backfilled beside it: applied {applied}");
    print_table(
        app,
        "SELECT day, dispatched FROM daily_dispatches_v2 ORDER BY day",
    )?;
    println!("   both views are live at once, and neither has stopped answering");

    println!("\n   -- promoted, and the checkpoint comes with it --");
    // Blue is retired *before* the rename, and the order is load-bearing: the
    // delete names `daily_dispatches`, which is about to become green's table.
    // Retiring after the swap would clear the rows that were just promoted.
    retire(models, blue.id(), "DELETE FROM daily_dispatches").await?;
    app.execute_batch(
        "DROP TABLE daily_dispatches;
         ALTER TABLE daily_dispatches_v2 RENAME TO daily_dispatches;",
    )?;
    let mut promoted = DailyDispatches::promoted();
    print_checkpoints(models, &[promoted.id()]).await?;

    dispatch_order(events, "o-3", DAYS[1]).await?;
    let applied = drive(events, models, &mut promoted, CHUNK).await?;
    println!("   one more dispatch, and the promoted view applied {applied} — not the whole log");
    print_table(
        app,
        "SELECT day, dispatched FROM daily_dispatches ORDER BY day",
    )?;

    Ok(())
}

/// Clears a view and builds it again, twice, at two very different chunk sizes.
///
/// `reset` is the operation that makes a read model disposable, and it clears
/// the rows and the checkpoint as **one** unit of work: a delete that succeeded
/// while the checkpoint survived would leave a view that is empty and believes
/// it is current. What it leaves behind is `Checkpoint::NeverRun`, which is a
/// different thing from a checkpoint at the first position — an operator who
/// substitutes one for the other makes the next run resume *after* event 1, and
/// event 1 is then skipped permanently and silently.
///
/// The two chunk sizes are the assertion. Chunk 1 commits once per event and
/// chunk 500 commits the whole replay at once; if the rows differ, the fold is
/// carrying state across events that it should be writing into the row.
async fn reset_and_rebuild(
    events: &SqliteEventStore,
    models: &SqliteProjectionStore,
    app: &Connection,
) -> Result<()> {
    /// The query both rebuilds are compared through.
    const ROWS: &str = "SELECT sku, on_hand FROM stock_on_hand ORDER BY sku";

    println!("\n== reset, and rebuilt at two chunk sizes ==");
    let mut rebuilt = Vec::with_capacity(REBUILD_CHUNKS.len());

    for chunk in REBUILD_CHUNKS {
        let mut stock = StockOnHand::new();
        retire(models, stock.id(), "DELETE FROM stock_on_hand").await?;
        let cleared = models.checkpoint(stock.id()).await?;
        let applied = drive(events, models, &mut stock, chunk).await?;
        println!(
            "   cleared to {}, rebuilt at chunk {chunk}: applied {applied}",
            render_checkpoint(cleared)
        );
        rebuilt.push(render_table(app, ROWS)?);
    }

    match rebuilt.as_slice() {
        [small, large] if small == large => {
            println!("   both rebuilds produced the same rows:");
            print!("{small}");
            Ok(())
        }
        [small, large] => {
            bail!("a rebuild's rows depend on its chunk size\nchunk 1:\n{small}chunk 500:\n{large}")
        }
        // Unreachable while `REBUILD_CHUNKS` has two entries. Spelled as a
        // refusal rather than as an index, because an index would turn a
        // changed constant into a panic in a program whose whole subject is
        // not losing data quietly.
        other => bail!("expected two rebuilds, got {}", other.len()),
    }
}

/// Breaks one view and shows the other three keep moving.
///
/// The failure is the one that actually happens: somebody writes a projection
/// against a field the log does not carry. It is not injected and nothing is
/// corrupted — the events are exactly as they were written, and the *reader*
/// is what disagrees with them. That failure surfaces as a decode error naming
/// the position it stopped at, the chunk is rolled back, and the checkpoint
/// stays where it was, which for a view that never ran is nowhere.
///
/// What this program does next is the part no library can decide: it records
/// the failure and runs the others. `run_projection` drives one projection and
/// halts on its first failure, which is the right primitive — a runner that
/// swallowed the error would be deciding, for every application at once, that
/// a wrong read model is better than a stalled one.
async fn one_poisoned_view(
    events: &SqliteEventStore,
    models: &SqliteProjectionStore,
) -> Result<()> {
    println!("\n== one poisoned view does not stall the others ==");

    let mut couriered = Couriered::new();
    let (stopped_at, why) = poison(events, models, &mut couriered).await?;
    println!("   couriered stopped at position {stopped_at}: {why}");
    println!(
        "   its checkpoint is {} — the chunk was rolled back and nothing was committed",
        render_checkpoint(models.checkpoint(couriered.id()).await?)
    );

    println!("\n   -- an event arrives while that view is broken --");
    cancel_order(events, "o-4").await?;

    let mut stock = StockOnHand::new();
    let mut status = OrderStatus::new();
    let mut daily = DailyDispatches::promoted();
    println!(
        "   stock_on_hand applied {}",
        drive(events, models, &mut stock, CHUNK).await?
    );
    println!(
        "   order_status applied {}",
        drive(events, models, &mut status, CHUNK).await?
    );
    println!(
        "   daily_dispatches applied {}",
        drive(events, models, &mut daily, CHUNK).await?
    );

    let (still_stuck, _) = poison(events, models, &mut couriered).await?;
    println!("   couriered is still stuck at position {still_stuck}");

    Ok(())
}

// ---------------------------------------------------------------------------
// Identity, validated once at the edge
// ---------------------------------------------------------------------------

/// A stock-keeping unit, validated once into the tag its events carry.
///
/// The resolution `course-subscriptions` and `transfers-on-sqlite` both reach,
/// and for the same reason: `DomainEvent::tags` is infallible while
/// `Tag::key_value` can refuse, so a domain type whose tags come from runtime
/// values holds the validated form instead of rebuilding it on every call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
struct Sku {
    /// The identifier as the application wrote it.
    id: String,
    /// The same value, validated into the tag every event for it carries.
    tag: Tag,
}

impl Sku {
    /// Validates `id` into the tag every event for this SKU is written with.
    fn new(id: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            id: id.to_owned(),
            tag: Tag::key_value("sku", id)?,
        })
    }

    /// The identifier as the application wrote it.
    fn as_str(&self) -> &str {
        &self.id
    }
}

impl TryFrom<String> for Sku {
    type Error = InvalidTag;

    fn try_from(id: String) -> Result<Self, InvalidTag> {
        Self::new(&id)
    }
}

impl From<Sku> for String {
    fn from(sku: Sku) -> Self {
        sku.id
    }
}

/// An order reference, validated once into the tag its events carry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
struct OrderId {
    /// The reference as the application wrote it.
    id: String,
    /// The same value, validated into the tag every event for it carries.
    tag: Tag,
}

impl OrderId {
    /// Validates `id` into the tag every event for this order is written with.
    fn new(id: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            id: id.to_owned(),
            tag: Tag::key_value("order", id)?,
        })
    }

    /// The reference as the application wrote it.
    fn as_str(&self) -> &str {
        &self.id
    }
}

impl TryFrom<String> for OrderId {
    type Error = InvalidTag;

    fn try_from(id: String) -> Result<Self, InvalidTag> {
        Self::new(&id)
    }
}

impl From<OrderId> for String {
    fn from(order: OrderId) -> Self {
        order.id
    }
}

// ---------------------------------------------------------------------------
// The domain: one enum, named once
// ---------------------------------------------------------------------------

/// Everything that can happen while a shelf is emptied into orders.
///
/// One enum, so a query and a fold cannot name different sets. There is no
/// `_ =>` arm anywhere in this file: a fifth variant is a compile error in
/// every decision model and every projection that has not been told what it
/// means, and that absence is the guarantee.
#[derive(Debug, Serialize, Deserialize)]
enum Fulfilment {
    /// Stock arrived on the shelf.
    StockReceived {
        /// What arrived.
        sku: Sku,
        /// How many units.
        quantity: u32,
    },
    /// An order took units off the shelf and is waiting to go out.
    OrderPlaced {
        /// The order now waiting.
        order: OrderId,
        /// What it holds.
        sku: Sku,
        /// How many units it holds.
        quantity: u32,
    },
    /// An order left the building.
    OrderDispatched {
        /// The order that went.
        order: OrderId,
        /// What it held.
        sku: Sku,
        /// How many units it held.
        quantity: u32,
        /// The day it went, as a plain `YYYY-MM-DD` string.
        day: String,
    },
    /// An order was called off and its units went back on the shelf.
    OrderCancelled {
        /// The order called off.
        order: OrderId,
        /// What it was holding.
        sku: Sku,
        /// How many units go back.
        quantity: u32,
    },
}

impl DomainEvent for Fulfilment {
    const EVENT_TYPES: &'static [EventType] = &[
        EventType::from_static("StockReceived"),
        EventType::from_static("OrderPlaced"),
        EventType::from_static("OrderDispatched"),
        EventType::from_static("OrderCancelled"),
    ];

    fn event_type(&self) -> EventType {
        match self {
            Self::StockReceived { .. } => Self::EVENT_TYPES[0].clone(),
            Self::OrderPlaced { .. } => Self::EVENT_TYPES[1].clone(),
            Self::OrderDispatched { .. } => Self::EVENT_TYPES[2].clone(),
            Self::OrderCancelled { .. } => Self::EVENT_TYPES[3].clone(),
        }
    }

    fn tags(&self) -> Tags {
        match self {
            Self::StockReceived { sku, .. } => [sku.tag.clone()].into_iter().collect(),
            Self::OrderPlaced { order, sku, .. }
            | Self::OrderDispatched { order, sku, .. }
            | Self::OrderCancelled { order, sku, .. } => {
                [order.tag.clone(), sku.tag.clone()].into_iter().collect()
            }
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

/// The event set the `couriered` view was written against, and the log is not.
///
/// It declares the same event *type* as `Fulfilment`'s dispatch variant and a
/// different *shape*: a `courier` this application has never written. Nothing
/// about that is detectable at compile time — the type name matches, the query
/// nominates the right events, and the mistake surfaces where every schema
/// mistake surfaces, at the first payload that has to be decoded.
///
/// It is a separate enum rather than a variant of `Fulfilment` because it
/// belongs to a different reader. That is the whole situation being modelled:
/// two teams, one log, and a disagreement about what was written.
#[derive(Debug, Serialize, Deserialize)]
enum Handover {
    /// A dispatch, as a later reader expected it to have been recorded.
    OrderDispatched {
        /// The order that went.
        order: OrderId,
        /// Who carried it. **Never written by this application.**
        courier: String,
    },
}

impl DomainEvent for Handover {
    const EVENT_TYPES: &'static [EventType] = &[EventType::from_static("OrderDispatched")];

    fn event_type(&self) -> EventType {
        match self {
            Self::OrderDispatched { .. } => Self::EVENT_TYPES[0].clone(),
        }
    }

    fn tags(&self) -> Tags {
        match self {
            Self::OrderDispatched { order, .. } => [order.tag.clone()].into_iter().collect(),
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

/// Why a decision refused.
///
/// Every variant carries the value the transcript prints — `sku-2 holds 2,
/// needs 5` and never `out of stock` — so a reader can act on the refusal
/// where it is printed instead of opening a second document to learn what
/// happened.
#[derive(Debug, thiserror::Error)]
enum Refusal {
    /// A receipt of nothing is not a receipt.
    #[error("cannot receive zero units of {sku}")]
    ZeroQuantity {
        /// The SKU nothing arrived for.
        sku: String,
    },
    /// An order reference may be used once.
    #[error("order {order} has already been placed")]
    AlreadyPlaced {
        /// The reference already in use.
        order: String,
    },
    /// The shelf cannot cover the order.
    #[error("{sku} holds {on_hand}, needs {needed}")]
    OutOfStock {
        /// The SKU that cannot cover it.
        sku: String,
        /// What is on the shelf.
        on_hand: i64,
        /// What the order asked for.
        needed: u32,
    },
    /// Nothing can happen to an order nobody placed.
    #[error("order {order} has not been placed")]
    NotPlaced {
        /// The reference nobody placed.
        order: String,
    },
    /// An order goes out once.
    #[error("order {order} has already been dispatched")]
    AlreadyDispatched {
        /// The order already gone.
        order: String,
    },
    /// A cancelled order is finished.
    #[error("order {order} has already been cancelled")]
    AlreadyCancelled {
        /// The order already called off.
        order: String,
    },
}

// ---------------------------------------------------------------------------
// The decision models: one per consistency concern
// ---------------------------------------------------------------------------

/// What is on the shelf for one SKU.
///
/// Scoped to a single `sku` tag, so the query this model derives nominates
/// every event ever written about that SKU and nothing else.
#[derive(Debug, Clone)]
struct Stock {
    /// The tags every event inside this boundary carries.
    scope: Tags,
    /// Units currently on the shelf, after placements and cancellations.
    on_hand: i64,
}

impl Stock {
    /// Validates the scope once, here, where the caller already writes a `?`.
    fn new(sku: &Sku) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("sku", sku.as_str())])?,
            on_hand: 0,
        })
    }
}

impl DecisionModel for Stock {
    type Event = Fulfilment;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Fulfilment::StockReceived { quantity, .. }
            | Fulfilment::OrderCancelled { quantity, .. } => {
                self.on_hand = self.on_hand.saturating_add(i64::from(quantity));
            }
            Fulfilment::OrderPlaced { quantity, .. } => {
                self.on_hand = self.on_hand.saturating_sub(i64::from(quantity));
            }
            // A dispatch moves nothing: the units left the shelf when the order
            // was placed, and counting them again here would show a shelf going
            // negative for stock that was already reserved. The arm exists
            // because the fold is exhaustive over the enum, and that
            // exhaustiveness is the whole guarantee.
            Fulfilment::OrderDispatched { .. } => {}
        }
    }
}

/// One order's own history.
///
/// Scoped to a single `order` tag. It is what makes a dispatch buildable from
/// folded state rather than from arguments the caller has to repeat: the SKU
/// and the quantity come out of the order's own placement.
#[derive(Debug, Clone)]
struct Order {
    /// The tags every event inside this boundary carries.
    scope: Tags,
    /// What this order holds, once it has been placed.
    held: Option<(Sku, u32)>,
    /// Whether it has already gone out.
    dispatched: bool,
    /// Whether it has already been called off.
    cancelled: bool,
}

impl Order {
    /// Validates the scope once, here, where the caller already writes a `?`.
    fn new(order: &OrderId) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("order", order.as_str())])?,
            held: None,
            dispatched: false,
            cancelled: false,
        })
    }
}

impl DecisionModel for Order {
    type Event = Fulfilment;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Fulfilment::OrderPlaced { sku, quantity, .. } => self.held = Some((sku, quantity)),
            Fulfilment::OrderDispatched { .. } => self.dispatched = true,
            Fulfilment::OrderCancelled { .. } => self.cancelled = true,
            // A stock receipt carries no `order` tag, so this model's query
            // never nominates one. The arm exists for exhaustiveness.
            Fulfilment::StockReceived { .. } => {}
        }
    }
}

// ---------------------------------------------------------------------------
// The four command handlers
// ---------------------------------------------------------------------------

/// Puts units on the shelf.
async fn receive_stock(events: &SqliteEventStore, sku: &str, quantity: u32) -> Result<()> {
    let sku = Sku::new(sku)?;
    let boundary = Stock::new(&sku)?;

    commit(
        events,
        boundary,
        Retry::attempts(ATTEMPTS.try_into()?),
        |_shelf: &Stock| {
            if quantity == 0 {
                return Err(Refusal::ZeroQuantity {
                    sku: sku.id.clone(),
                });
            }
            Ok(vec![Fulfilment::StockReceived {
                sku: sku.clone(),
                quantity,
            }])
        },
    )
    .await
    .map(|_| ())
    .map_err(rejected)
}

/// Takes units off the shelf for an order that does not exist yet.
///
/// The one decision here that spans two entities: *can the shelf cover it* is
/// the SKU's concern and *is this reference already in use* is the order's, and
/// the tuple is both, OR'd into one query and checked by one append condition.
/// This example does not dwell on it — `handles-and-quotas` is where the
/// composition argument is made — but the shelf number every view below is
/// derived from is only trustworthy because this boundary is drawn correctly.
async fn place_order(
    events: &SqliteEventStore,
    order: &str,
    sku: &str,
    quantity: u32,
) -> Result<()> {
    let order = OrderId::new(order)?;
    let sku = Sku::new(sku)?;
    let boundary = (Stock::new(&sku)?, Order::new(&order)?);

    commit(
        events,
        boundary,
        Retry::attempts(ATTEMPTS.try_into()?),
        |(shelf, existing): &(Stock, Order)| {
            if existing.held.is_some() {
                return Err(Refusal::AlreadyPlaced {
                    order: order.id.clone(),
                });
            }
            if shelf.on_hand < i64::from(quantity) {
                return Err(Refusal::OutOfStock {
                    sku: sku.id.clone(),
                    on_hand: shelf.on_hand,
                    needed: quantity,
                });
            }
            Ok(vec![Fulfilment::OrderPlaced {
                order: order.clone(),
                sku: sku.clone(),
                quantity,
            }])
        },
    )
    .await
    .map(|_| ())
    .map_err(rejected)
}

/// Sends an order out, on a named day.
async fn dispatch_order(events: &SqliteEventStore, order: &str, day: &str) -> Result<()> {
    let order = OrderId::new(order)?;
    let boundary = Order::new(&order)?;

    commit(
        events,
        boundary,
        Retry::attempts(ATTEMPTS.try_into()?),
        |state: &Order| {
            let Some((sku, quantity)) = state.held.clone() else {
                return Err(Refusal::NotPlaced {
                    order: order.id.clone(),
                });
            };
            if state.dispatched {
                return Err(Refusal::AlreadyDispatched {
                    order: order.id.clone(),
                });
            }
            if state.cancelled {
                return Err(Refusal::AlreadyCancelled {
                    order: order.id.clone(),
                });
            }
            Ok(vec![Fulfilment::OrderDispatched {
                order: order.clone(),
                sku,
                quantity,
                day: day.to_owned(),
            }])
        },
    )
    .await
    .map(|_| ())
    .map_err(rejected)
}

/// Calls an order off and puts its units back.
async fn cancel_order(events: &SqliteEventStore, order: &str) -> Result<()> {
    let order = OrderId::new(order)?;
    let boundary = Order::new(&order)?;

    commit(
        events,
        boundary,
        Retry::attempts(ATTEMPTS.try_into()?),
        |state: &Order| {
            let Some((sku, quantity)) = state.held.clone() else {
                return Err(Refusal::NotPlaced {
                    order: order.id.clone(),
                });
            };
            if state.dispatched {
                return Err(Refusal::AlreadyDispatched {
                    order: order.id.clone(),
                });
            }
            if state.cancelled {
                return Err(Refusal::AlreadyCancelled {
                    order: order.id.clone(),
                });
            }
            Ok(vec![Fulfilment::OrderCancelled {
                order: order.clone(),
                sku,
                quantity,
            }])
        },
    )
    .await
    .map(|_| ())
    .map_err(rejected)
}

// ---------------------------------------------------------------------------
// The four views
// ---------------------------------------------------------------------------

/// Units on the shelf, per SKU.
///
/// The fold carries **no state of its own** and writes a *delta* against the
/// row. That is deliberate rather than minimal: a runner resumes from its
/// checkpoint, so a projection holding a running total in memory would start
/// from zero and write absolute numbers wrong by everything it did not replay.
/// It is also what makes the chunk-size invariance above true.
#[derive(Debug)]
struct StockOnHand {
    /// Which read model this is, within the projection store.
    id: ProjectionId,
    /// Every event, so the tags constrain nothing.
    scope: Tags,
}

impl StockOnHand {
    /// The view over every SKU.
    fn new() -> Self {
        Self {
            id: ProjectionId::new("stock_on_hand"),
            scope: Tags::empty(),
        }
    }
}

impl Projection for StockOnHand {
    type Event = Fulfilment;
    type Store = SqliteProjectionStore;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(
        &mut self,
        event: Fulfilment,
        batch: &mut SqliteBatch,
    ) -> Result<(), SqliteProjectionStoreError> {
        let (sku, delta) = match event {
            Fulfilment::StockReceived { sku, quantity }
            | Fulfilment::OrderCancelled { sku, quantity, .. } => (sku, i64::from(quantity)),
            Fulfilment::OrderPlaced { sku, quantity, .. } => (sku, -i64::from(quantity)),
            Fulfilment::OrderDispatched { sku, .. } => (sku, 0),
        };

        batch.push(
            "INSERT INTO stock_on_hand (sku, on_hand) VALUES (?1, ?2) \
             ON CONFLICT(sku) \
             DO UPDATE SET on_hand = stock_on_hand.on_hand + excluded.on_hand",
            [Value::Text(sku.into()), Value::Integer(delta)],
        );

        Ok(())
    }
}

/// Where each order has got to.
///
/// Unlike the shelf, this view writes an *absolute* value: the status is the
/// last one seen, not an accumulation, so replaying it from any checkpoint
/// lands on the same answer.
#[derive(Debug)]
struct OrderStatus {
    /// Which read model this is, within the projection store.
    id: ProjectionId,
    /// Every event, so the tags constrain nothing.
    scope: Tags,
}

impl OrderStatus {
    /// The view over every order.
    fn new() -> Self {
        Self {
            id: ProjectionId::new("order_status"),
            scope: Tags::empty(),
        }
    }
}

impl Projection for OrderStatus {
    type Event = Fulfilment;
    type Store = SqliteProjectionStore;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(
        &mut self,
        event: Fulfilment,
        batch: &mut SqliteBatch,
    ) -> Result<(), SqliteProjectionStoreError> {
        let (order, status, sku, quantity) = match event {
            Fulfilment::OrderPlaced {
                order,
                sku,
                quantity,
            } => (order, "placed", sku, quantity),
            Fulfilment::OrderDispatched {
                order,
                sku,
                quantity,
                ..
            } => (order, "dispatched", sku, quantity),
            Fulfilment::OrderCancelled {
                order,
                sku,
                quantity,
            } => (order, "cancelled", sku, quantity),
            // Nominated and ignored. The derived query is the event set crossed
            // with the scope, and this view's scope is empty, so a stock
            // receipt arrives here whether it is wanted or not — the checkpoint
            // still advances past it, which is what "the position considered"
            // means.
            Fulfilment::StockReceived { .. } => return Ok(()),
        };

        batch.push(
            "INSERT INTO order_status (order_ref, status, sku, quantity) \
             VALUES (?1, ?2, ?3, ?4) \
             ON CONFLICT(order_ref) DO UPDATE SET status = excluded.status",
            [
                Value::Text(order.into()),
                Value::Text(status.to_owned()),
                Value::Text(sku.into()),
                Value::Integer(i64::from(quantity)),
            ],
        );

        Ok(())
    }
}

/// How much went out each day, counted one of two ways.
///
/// One type with two configurations rather than two types, because the blue
/// and green halves of a rebuild differ in exactly the way a real one does:
/// the fold changed its mind about what it was counting, and everything else
/// stayed put.
#[derive(Debug)]
struct DailyDispatches {
    /// Which read model this is. **It does not change when the table is
    /// promoted** — the checkpoint is keyed by this id, and renaming it to
    /// match the promoted table would discard the resume point.
    id: ProjectionId,
    /// Every event, so the tags constrain nothing.
    scope: Tags,
    /// The table this instance writes into.
    table: &'static str,
    /// What one dispatch adds to its day.
    counting: Counting,
}

/// What one dispatch adds to its day's total.
#[derive(Debug, Clone, Copy)]
enum Counting {
    /// One per order that went out.
    Orders,
    /// One per unit that went out.
    Units,
}

impl DailyDispatches {
    /// The first attempt: one per order.
    fn counting_orders() -> Self {
        Self {
            id: ProjectionId::new("daily_dispatches"),
            scope: Tags::empty(),
            table: "daily_dispatches",
            counting: Counting::Orders,
        }
    }

    /// The replacement, backfilled beside the first into its own table.
    fn counting_units() -> Self {
        Self {
            id: ProjectionId::new("daily_dispatches_v2"),
            scope: Tags::empty(),
            table: "daily_dispatches_v2",
            counting: Counting::Units,
        }
    }

    /// The replacement after the swap: the same id, the promoted table.
    fn promoted() -> Self {
        Self {
            id: ProjectionId::new("daily_dispatches_v2"),
            scope: Tags::empty(),
            table: "daily_dispatches",
            counting: Counting::Units,
        }
    }
}

impl Projection for DailyDispatches {
    type Event = Fulfilment;
    type Store = SqliteProjectionStore;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(
        &mut self,
        event: Fulfilment,
        batch: &mut SqliteBatch,
    ) -> Result<(), SqliteProjectionStoreError> {
        let (day, delta) = match event {
            Fulfilment::OrderDispatched { day, quantity, .. } => match self.counting {
                Counting::Orders => (day, 1),
                Counting::Units => (day, i64::from(quantity)),
            },
            // Nominated and ignored, for the reason `order_status` gives.
            Fulfilment::StockReceived { .. }
            | Fulfilment::OrderPlaced { .. }
            | Fulfilment::OrderCancelled { .. } => return Ok(()),
        };

        // The table name is interpolated rather than bound, because SQLite
        // binds values and not identifiers. It is safe here for a reason worth
        // stating rather than assuming: `table` is a `&'static str` chosen by
        // one of the three constructors above and can never hold user input.
        batch.push(
            format!(
                "INSERT INTO {table} (day, dispatched) VALUES (?1, ?2) \
                 ON CONFLICT(day) \
                 DO UPDATE SET dispatched = {table}.dispatched + excluded.dispatched",
                table = self.table
            ),
            [Value::Text(day), Value::Integer(delta)],
        );

        Ok(())
    }
}

/// The view that was written against a field the log does not carry.
///
/// It compiles, its query is well formed, and it nominates exactly the events
/// it means to. The disagreement is in the payload, and the payload is the one
/// thing no signature describes.
#[derive(Debug)]
struct Couriered {
    /// Which read model this is, within the projection store.
    id: ProjectionId,
    /// Every dispatch, so the tags constrain nothing.
    scope: Tags,
}

impl Couriered {
    /// The view over every dispatch.
    fn new() -> Self {
        Self {
            id: ProjectionId::new("couriered"),
            scope: Tags::empty(),
        }
    }
}

impl Projection for Couriered {
    type Event = Handover;
    type Store = SqliteProjectionStore;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(
        &mut self,
        event: Handover,
        batch: &mut SqliteBatch,
    ) -> Result<(), SqliteProjectionStoreError> {
        let Handover::OrderDispatched { order, courier } = event;

        batch.push(
            "INSERT INTO couriered (order_ref, courier) VALUES (?1, ?2) \
             ON CONFLICT(order_ref) DO UPDATE SET courier = excluded.courier",
            [Value::Text(order.into()), Value::Text(courier)],
        );

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Driving views, and reporting on them
// ---------------------------------------------------------------------------

/// Brings one view up to date and reports how much it applied.
///
/// Generic over the projection rather than written four times, and generic is
/// as far as it can go: `Projection` has an associated event type and an
/// associated store, so there is no `dyn Projection` to put in a registry and
/// no loop to write. A heterogeneous set of views is a sequence of calls, and
/// that is a property of the port, not a shortcoming of this file.
async fn drive<P>(
    events: &SqliteEventStore,
    models: &SqliteProjectionStore,
    projection: &mut P,
    chunk: usize,
) -> Result<usize>
where
    P: Projection<Store = SqliteProjectionStore>,
{
    let progress = run_projection(events, models, projection, &Json, chunk.try_into()?).await?;
    Ok(progress.applied)
}

/// Runs the broken view, which is expected to fail, and says where it stopped.
///
/// Returns the position and the decoder's own message. A run that *succeeded*
/// would be the interesting failure — it would mean the log had grown a
/// `courier` field nobody in this file wrote — so it is refused out loud
/// rather than reported as a pass.
async fn poison(
    events: &SqliteEventStore,
    models: &SqliteProjectionStore,
    projection: &mut Couriered,
) -> Result<(u64, String)> {
    match run_projection(events, models, projection, &Json, CHUNK.try_into()?).await {
        Ok(progress) => bail!(
            "the couriered view decoded {} event(s) it could not have",
            progress.applied
        ),
        Err(ProjectionError::Decode {
            position, source, ..
        }) => Ok((position.get(), error_chain(&source))),
        // `ProjectionError` is `#[non_exhaustive]`. A read failure or a commit
        // failure here would be a different defect wearing this one's clothes.
        Err(other) => bail!("expected a decode failure, got: {other}"),
    }
}

/// Renders an error and everything under it, joined by `: `.
///
/// `CodecError`'s own `Display` says the payload could not be decoded, which
/// is a category. The value a reader needs — *which field was missing* — is
/// one link down the source chain, and a message that stops at the top of that
/// chain is the same defect `rejected` exists to avoid one layer up.
fn error_chain(error: &dyn core::error::Error) -> String {
    let mut rendered = error.to_string();
    let mut source = error.source();

    while let Some(cause) = source {
        rendered.push_str(": ");
        rendered.push_str(&cause.to_string());
        source = cause.source();
    }

    rendered
}

/// Clears a view's rows and its checkpoint as one unit of work.
///
/// The deletes are the *application's*, because the read model is: the port
/// takes a batch and commits it alongside the checkpoint reset, and it has no
/// idea which tables this program invented. Used twice below, for the two
/// things reset is actually for — retiring a view, and preparing a rebuild.
async fn retire(
    models: &SqliteProjectionStore,
    id: &ProjectionId,
    delete: &'static str,
) -> Result<()> {
    let mut batch = models.begin();
    batch.push(delete, []);
    models.reset(batch, id).await?;
    Ok(())
}

/// Prints where each named view has got to.
async fn print_checkpoints(models: &SqliteProjectionStore, ids: &[&ProjectionId]) -> Result<()> {
    for id in ids {
        println!(
            "   {id} is {}",
            render_checkpoint(models.checkpoint(id).await?)
        );
    }
    Ok(())
}

/// Renders a checkpoint as the transcript prints it.
fn render_checkpoint(checkpoint: Checkpoint) -> String {
    match checkpoint {
        Checkpoint::NeverRun => "never run".to_owned(),
        Checkpoint::Live { through } => format!("live through {through}"),
        Checkpoint::Rebuilding { through } => format!("rebuilding through {through}"),
        // `Checkpoint` is `#[non_exhaustive]`, so the wildcard is required. It
        // is not laziness: a fourth state added upstream must not silently
        // render as one of these three.
        other => format!("{other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Reading what the views wrote
// ---------------------------------------------------------------------------

/// Prints a read-model query's rows, three spaces in.
fn print_table(app: &Connection, sql: &str) -> Result<()> {
    print!("{}", render_table(app, sql)?);
    Ok(())
}

/// Renders a read-model query's rows, so two of them can be compared.
///
/// Every column comes back as a `rusqlite::types::Value` rather than as a
/// typed `get`, because this one function serves five tables with different
/// column types and a typed read would need one function per table.
fn render_table(app: &Connection, sql: &str) -> Result<String> {
    let mut statement = app.prepare(sql)?;
    let columns = statement.column_count();
    let mut rows = statement.query([])?;
    let mut rendered = String::new();

    while let Some(row) = rows.next()? {
        let mut cells = Vec::with_capacity(columns);
        for index in 0..columns {
            cells.push(render_cell(&row.get::<_, Value>(index)?));
        }
        rendered.push_str("      ");
        rendered.push_str(&cells.join("  "));
        rendered.push('\n');
    }

    Ok(rendered)
}

/// Renders one cell of a read-model row.
fn render_cell(value: &Value) -> String {
    match value {
        Value::Null => "NULL".to_owned(),
        Value::Integer(number) => format!("{number}"),
        Value::Real(number) => format!("{number}"),
        Value::Text(text) => text.clone(),
        Value::Blob(bytes) => format!("<{} bytes>", bytes.len()),
    }
}

/// Prints every event in the store, in the order the store assigned.
async fn print_log(events: &SqliteEventStore) -> Result<()> {
    let log = collect(events.read(&Query::all(), ReadOptions::new())).await?;

    for event in log {
        println!(
            "   {:>3}  {:<16} {:?}",
            event.position.get(),
            event.event_type().as_str(),
            event.tags()
        );
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// The file this run owns
// ---------------------------------------------------------------------------

/// Where this run's database lives.
///
/// Under the process id, so a `cargo run` and the test that spawns the binary
/// never meet on one file.
fn database_path() -> PathBuf {
    std::env::temp_dir().join(format!("happenstance-rebuilding-{}.db", std::process::id()))
}

/// Removes the database and the two files WAL mode keeps beside it.
///
/// Deleting only the `.db` and leaving `-wal` behind is how a "fresh" run
/// starts on the tail of the last one.
fn remove_database(path: &Path) {
    for suffix in ["", "-wal", "-shm"] {
        let mut name = path.as_os_str().to_owned();
        name.push(suffix);
        drop(std::fs::remove_file(PathBuf::from(name)));
    }
}

/// Renders a command failure as the message the transcript prints.
///
/// `CommandError::Refused` carries the handler's *own* refusal, and that is
/// what a reader must see: `CommandError`'s own `Display` says "the decision
/// refused", which is a category rather than a value.
fn rejected(err: CommandError<SqliteEventStoreError, Refusal>) -> anyhow::Error {
    match err {
        CommandError::Refused(refusal) => refusal.into(),
        other => other.into(),
    }
}
