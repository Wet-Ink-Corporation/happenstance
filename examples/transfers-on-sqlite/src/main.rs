//! A consuming application, on a real database.
//!
//! `course-subscriptions` is the canonical DCB example and it runs against
//! `MemoryEventStore`. That proves the *semantics*. It cannot prove the thing
//! an application actually needs, because an in-memory store has no disk to
//! survive: that the typed layer and `happenstance-sqlite` compose at all, and
//! that what they write is still there afterwards.
//!
//! Until this package existed, **nothing in the workspace compiled the typed
//! layer against a durable adapter.** `happenstance-sqlite` depends only on
//! `happenstance-core` and its tests drive the conformance suite directly; the
//! worked example depends only on `happenstance` and runs in memory. The two
//! halves a consumer puts together were each tested alone, and the seam between
//! them was tested by nobody. This program is that seam, exercised.
//!
//! Run with `cargo run -p transfers-on-sqlite`.
//!
//! # What this proves that the memory store cannot
//!
//! 1. **`commit` accepts a `SqliteEventStore`.** The typed command loop is
//!    generic over `EventStore`; the adapter implements `SendEventStore` and
//!    reaches the weaker bound through `trait_variant`'s blanket impl. That is
//!    a sentence the type checker had never been asked to agree with.
//! 2. **`run_projection` drives a `SqliteProjectionStore`,** into a read-model
//!    table this application owns, in the same file as the event log.
//! 3. **Everything survives the handles being dropped.** Halfway through, every
//!    store, connection and folded value this run built goes out of scope. What
//!    is read afterwards comes off disk.
//! 4. **The checkpoint survives with it.** Re-running the projection against
//!    the reopened file applies *zero* events — the resume point is durable, so
//!    a restarted application does not replay its whole log.
//!
//! # What the reopen does not prove, stated so nobody reads more into it
//!
//! It is **one process**. Every handle is dropped and every connection closed,
//! and what is read back afterwards is read through fresh connections onto the
//! file — which is a real durability claim, because SQLite keeps no cache
//! across a closed connection. It is not a claim about surviving a *crash*, a
//! power loss, or an `fsync` policy. Those belong to the conformance suite's
//! reopen rule and to the adapter's own durability verdicts, which own the
//! fault injection this program has none of.
//!
//! # The domain, and why it is not the worked example's
//!
//! A transfer moves money between two accounts. It is the smallest decision
//! that genuinely spans two entities: the funds check belongs to the payer, the
//! credit belongs to the payee, and the two events must land together or not at
//! all. So the boundary is a tuple of two `Balance` models — one per account —
//! composed into one query and checked by one append condition, and the append
//! carries two events.
//!
//! That is deliberately *not* a second copy of the course-subscriptions domain.
//! Copying it would have duplicated four hundred lines to change one import,
//! and `examples/course-subscriptions/tests/runs.rs` reads that file with
//! `include_str!` to assert how it is written — a shared domain crate would
//! have to move it out from under those assertions. A different domain of the
//! same shape costs less and says more.

#![allow(clippy::print_stdout, reason = "the transcript is the point")]

use std::num::NonZeroU32;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use happenstance::bytes::Bytes;
use happenstance::{
    Boundary, Codec, CodecError, CommandError, DecisionModel, DomainEvent, EventStore, EventType,
    InvalidTag, Json, Projection, ProjectionId, Query, ReadOptions, Retry, Tag, Tags, collect,
    commit, read_decision_model, run_projection,
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
/// single-writer, so the bound is never spent. Built at compile time, the
/// same way: `attempts` is `const fn`.
const ATTEMPTS: Retry = Retry::attempts(NonZeroU32::new(3).unwrap());

/// How many events one projection chunk commits at a time.
///
/// Small enough that the first run commits more than once, which is the only
/// way a checkpoint's *intermediate* positions are ever written.
const CHUNK: usize = 2;

/// The read model this application owns.
///
/// It is the application's table, not the adapter's: `SqliteProjectionStore`
/// migrates `projection_checkpoint` and `projection_meta` and stops there. A
/// projection store that invented your read model's schema would be a framework
/// rather than a port.
const READ_MODEL_DDL: &str = "
CREATE TABLE IF NOT EXISTS account_balance (
    account TEXT PRIMARY KEY,
    balance INTEGER NOT NULL
) STRICT;
";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let path = database_path();
    remove_database(&path);

    println!("== the database this run writes to ==");
    println!("   {}", path.display());

    before_the_reopen(&path).await?;
    after_the_reopen(&path).await?;

    remove_database(&path);
    Ok(())
}

/// Everything before the reopen, inside one scope.
///
/// The scope is load-bearing. Every store, connection and decision model this
/// function builds is dropped when it returns, so nothing the second half reads
/// can have come from a value this one was still holding.
async fn before_the_reopen(path: &Path) -> Result<()> {
    let events = SqliteEventStore::open(path)?;
    let models = SqliteProjectionStore::open(path)?;
    let app = open_configured(path)?;
    app.execute_batch(READ_MODEL_DDL)?;

    println!("\n== opening two accounts ==");
    open_account(&events, "a1").await?;
    open_account(&events, "a2").await?;
    println!("   a1 and a2 opened");

    println!("\n== depositing 100 into a1 ==");
    deposit(&events, "a1", 100).await?;
    println!("   deposited");

    println!("\n== transferring 40 from a1 to a2 ==");
    transfer(&events, "a1", "a2", 40).await?;
    println!("   transferred: one append, two events, both accounts checked");

    println!("\n== transferring 500 from a1 to a2 ==");
    match transfer(&events, "a1", "a2", 500).await {
        Ok(()) => bail!("an overdrawn transfer should have been rejected"),
        Err(err) => println!("   rejected: {err}"),
    }

    println!("\n== projecting balances into the same file ==");
    let mut balances = Balances::new();
    let done = run_projection(&events, &models, &mut balances, &Json, CHUNK.try_into()?).await?;
    println!("   applied {} event(s)", done.applied);
    print_read_model(&app)?;

    Ok(())
}

/// Everything after the reopen, against the same file.
///
/// Nothing is carried in from above but the path.
async fn after_the_reopen(path: &Path) -> Result<()> {
    println!("\n== everything above is dropped; reopening the same file ==");
    let events = SqliteEventStore::open(path)?;
    let models = SqliteProjectionStore::open(path)?;
    let app = open_configured(path)?;
    println!("   reopened");

    println!("\n== the read model, straight off disk ==");
    print_read_model(&app)?;

    println!("\n== the same balances, folded again from the event log ==");
    for account in ["a1", "a2"] {
        let balance = fold_balance(&events, account).await?;
        println!("   {account}: {}", balance.held);
    }

    println!("\n== the checkpoint survived too ==");
    let mut balances = Balances::new();
    let done = run_projection(&events, &models, &mut balances, &Json, CHUNK.try_into()?).await?;
    println!(
        "   re-running the projection applied {} event(s) — the log is not replayed",
        done.applied
    );

    println!("\n== the reopened store is live, not merely readable ==");
    transfer(&events, "a2", "a1", 15).await?;
    let done = run_projection(&events, &models, &mut balances, &Json, CHUNK.try_into()?).await?;
    println!(
        "   transferred 15 back, and the projection applied {} event(s)",
        done.applied
    );
    print_read_model(&app)?;

    println!("\n== final log ==");
    print_log(&events).await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Identity, validated once at the edge
// ---------------------------------------------------------------------------

/// An account identifier, validated once into the tag its events carry.
///
/// The same resolution `course-subscriptions` reaches and for the same reason:
/// `DomainEvent::tags` is infallible and `Tag::key_value` can refuse, so a
/// domain type whose tags come from runtime values holds the validated form
/// rather than rebuilding it on every call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
struct AccountId {
    id: String,
    tag: Tag,
}

impl AccountId {
    /// Validates `id` into the tag every event for this account is written with.
    fn new(id: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            id: id.to_owned(),
            tag: Tag::key_value("account", id)?,
        })
    }

    /// The identifier as the application wrote it.
    fn as_str(&self) -> &str {
        &self.id
    }
}

impl TryFrom<String> for AccountId {
    type Error = InvalidTag;

    fn try_from(id: String) -> Result<Self, InvalidTag> {
        Self::new(&id)
    }
}

impl From<AccountId> for String {
    fn from(account: AccountId) -> Self {
        account.id
    }
}

// ---------------------------------------------------------------------------
// The domain
// ---------------------------------------------------------------------------

/// Everything that can happen to an account.
///
/// One enum, so a query and a fold cannot name different sets. A fourth variant
/// is a compile error in every fold below that has not been taught it — there
/// is no `_ =>` arm anywhere in this file, and that absence is the guarantee.
#[derive(Debug, Serialize, Deserialize)]
enum Ledger {
    /// An account exists and can hold money.
    AccountOpened {
        /// The account now open.
        account: AccountId,
    },
    /// Money arrived.
    Deposited {
        /// The account credited.
        account: AccountId,
        /// How much, in whole units.
        amount: u32,
    },
    /// Money left.
    Withdrawn {
        /// The account debited.
        account: AccountId,
        /// How much, in whole units.
        amount: u32,
    },
}

const ACCOUNT_OPENED: EventType = EventType::from_static("AccountOpened");
const DEPOSITED: EventType = EventType::from_static("Deposited");
const WITHDRAWN: EventType = EventType::from_static("Withdrawn");

impl DomainEvent for Ledger {
    const EVENT_TYPES: &'static [EventType] = &[ACCOUNT_OPENED, DEPOSITED, WITHDRAWN];

    fn event_type(&self) -> EventType {
        match self {
            Self::AccountOpened { .. } => ACCOUNT_OPENED,
            Self::Deposited { .. } => DEPOSITED,
            Self::Withdrawn { .. } => WITHDRAWN,
        }
    }

    fn tags(&self) -> Tags {
        match self {
            Self::AccountOpened { account }
            | Self::Deposited { account, .. }
            | Self::Withdrawn { account, .. } => [account.tag.clone()].into_iter().collect(),
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
/// Every variant carries the value the transcript prints — `a1 holds 60, needs
/// 500` and never `insufficient funds` — so a reader can act on the refusal
/// where it is printed.
#[derive(Debug, thiserror::Error)]
enum Refusal {
    /// An account cannot be opened twice.
    #[error("account {account} is already open")]
    AlreadyOpen {
        /// The account that already exists.
        account: String,
    },
    /// Money cannot move through an account nobody opened.
    #[error("account {account} does not exist")]
    NotOpen {
        /// The account nobody opened.
        account: String,
    },
    /// An account may not go overdrawn.
    #[error("account {account} holds {held}, needs {needed}")]
    Overdrawn {
        /// The account that cannot cover the debit.
        account: String,
        /// What it currently holds.
        held: u64,
        /// What the transfer asked for.
        needed: u32,
    },
    /// An account cannot pay itself.
    #[error("account {account} cannot transfer to itself")]
    SelfTransfer {
        /// The account on both ends.
        account: String,
    },
}

// ---------------------------------------------------------------------------
// The decision model
// ---------------------------------------------------------------------------

/// One account: whether it exists, and what it holds.
///
/// The whole consistency concern for a deposit, and *half* of one for a
/// transfer. That it is the same type in both places is the point — a decision
/// model is a value, not a per-handler blob.
#[derive(Debug, Clone)]
struct Balance {
    scope: Tags,
    opened: bool,
    held: u64,
}

impl Balance {
    /// Validates the scope once, here, where the caller already writes a `?`.
    fn new(account: &AccountId) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("account", account.as_str())])?,
            opened: false,
            held: 0,
        })
    }
}

impl DecisionModel for Balance {
    type Event = Ledger;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Ledger::AccountOpened { .. } => self.opened = true,
            Ledger::Deposited { amount, .. } => {
                self.held = self.held.saturating_add(u64::from(amount));
            }
            Ledger::Withdrawn { amount, .. } => {
                self.held = self.held.saturating_sub(u64::from(amount));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// The command handlers
// ---------------------------------------------------------------------------

/// Opens an account, rejecting a second opening of the same one.
async fn open_account(events: &SqliteEventStore, account: &str) -> Result<()> {
    let account = AccountId::new(account)?;
    let boundary = Balance::new(&account)?;

    commit(events, boundary, ATTEMPTS, |balance: &Balance| {
        if balance.opened {
            return Err(Refusal::AlreadyOpen {
                account: account.id.clone(),
            });
        }
        Ok(vec![Ledger::AccountOpened {
            account: account.clone(),
        }])
    })
    .await
    .map(|_| ())
    .map_err(rejected)
}

/// Credits an account that exists.
async fn deposit(events: &SqliteEventStore, account: &str, amount: u32) -> Result<()> {
    let account = AccountId::new(account)?;
    let boundary = Balance::new(&account)?;

    commit(events, boundary, ATTEMPTS, |balance: &Balance| {
        if !balance.opened {
            return Err(Refusal::NotOpen {
                account: account.id.clone(),
            });
        }
        Ok(vec![Ledger::Deposited {
            account: account.clone(),
            amount,
        }])
    })
    .await
    .map(|_| ())
    .map_err(rejected)
}

/// Moves money between two accounts, atomically.
///
/// The case that motivates both DCB and composition. *Can the payer cover it*
/// is one consistency concern; *does the payee exist* is another, on a
/// different entity. The tuple below is both, OR'd into one query and checked
/// by one append condition — and the two events it produces land in a single
/// `append`, which on this adapter is a single `BEGIN IMMEDIATE` transaction. A
/// crash between the debit and the credit is not a state this store can be
/// found in.
async fn transfer(events: &SqliteEventStore, from: &str, to: &str, amount: u32) -> Result<()> {
    let from = AccountId::new(from)?;
    let to = AccountId::new(to)?;

    // Two models with the same scope would compose into a query that nominates
    // each event twice and a fold that counts it twice. Refused here, in the
    // domain, rather than left to produce a plausible wrong number.
    if from.as_str() == to.as_str() {
        return Err(Refusal::SelfTransfer {
            account: from.id.clone(),
        }
        .into());
    }

    let boundary = (Balance::new(&from)?, Balance::new(&to)?);

    commit(
        events,
        boundary,
        ATTEMPTS,
        |(payer, payee): &(Balance, Balance)| {
            if !payer.opened {
                return Err(Refusal::NotOpen {
                    account: from.id.clone(),
                });
            }
            if !payee.opened {
                return Err(Refusal::NotOpen {
                    account: to.id.clone(),
                });
            }
            if payer.held < u64::from(amount) {
                return Err(Refusal::Overdrawn {
                    account: from.id.clone(),
                    held: payer.held,
                    needed: amount,
                });
            }
            Ok(vec![
                Ledger::Withdrawn {
                    account: from.clone(),
                    amount,
                },
                Ledger::Deposited {
                    account: to.clone(),
                    amount,
                },
            ])
        },
    )
    .await
    .map(|_| ())
    .map_err(rejected)
}

// ---------------------------------------------------------------------------
// The projection
// ---------------------------------------------------------------------------

/// Balances, kept as a table rather than folded on every read.
///
/// The fold carries **no state of its own**, and that is deliberate rather than
/// minimal. After a reopen the runner resumes from the checkpoint, so a
/// projection holding a running total in memory would start from zero and write
/// absolute numbers wrong by everything it did not replay. Writing a *delta*
/// against the row makes the table authoritative and the resume correct — which
/// is the shape a durable projection has to have, and a shape a memory store
/// never forces anybody to discover.
#[derive(Debug)]
struct Balances {
    id: ProjectionId,
    scope: Tags,
}

impl Balances {
    /// The projection over every account.
    fn new() -> Self {
        Self {
            id: ProjectionId::new("account_balance"),
            scope: Tags::empty(),
        }
    }
}

impl Projection for Balances {
    type Event = Ledger;
    type Store = SqliteProjectionStore;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(
        &mut self,
        event: Ledger,
        batch: &mut SqliteBatch,
    ) -> Result<(), SqliteProjectionStoreError> {
        let (account, delta) = match event {
            Ledger::AccountOpened { account } => (account, 0),
            Ledger::Deposited { account, amount } => (account, i64::from(amount)),
            Ledger::Withdrawn { account, amount } => (account, -i64::from(amount)),
        };

        batch.push(
            "INSERT INTO account_balance (account, balance) VALUES (?1, ?2) \
             ON CONFLICT(account) \
             DO UPDATE SET balance = account_balance.balance + excluded.balance",
            [Value::Text(account.into()), Value::Integer(delta)],
        );

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Reading, after the reopen
// ---------------------------------------------------------------------------

/// Folds one account's balance straight out of the event log.
///
/// The read half of the typed layer, spelled out: derive the query from the
/// model, read what it nominates, and absorb each event through the same
/// `Boundary` the command loop uses. It exists here to say that the read model
/// printed beside it is a *cache* and the log is the record — the two numbers
/// agreeing is the assertion.
async fn fold_balance(events: &SqliteEventStore, account: &str) -> Result<Balance> {
    let account = AccountId::new(account)?;
    let mut balance = Balance::new(&account)?;

    let query = balance.query()?;
    let (found, _) = read_decision_model(events, &query).await?;
    for event in &found {
        balance.absorb(event, &Json)?;
    }

    Ok(balance)
}

/// Prints the read-model table as the application's own query sees it.
fn print_read_model(app: &Connection) -> Result<()> {
    let mut statement =
        app.prepare("SELECT account, balance FROM account_balance ORDER BY account")?;
    let rows = statement.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    })?;

    for row in rows {
        let (account, balance) = row?;
        println!("   {account}: {balance}");
    }

    Ok(())
}

/// Prints every event in the store, in the order the store assigned.
async fn print_log(events: &SqliteEventStore) -> Result<()> {
    let log = collect(events.read(&Query::all(), ReadOptions::new())).await?;

    for event in log {
        println!(
            "   {:>3}  {:<15} {:?}",
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
/// Under the process id, so two concurrent runs — a `cargo run` and the test
/// that spawns the binary — never meet on one file.
fn database_path() -> PathBuf {
    std::env::temp_dir().join(format!("happenstance-transfers-{}.db", std::process::id()))
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
