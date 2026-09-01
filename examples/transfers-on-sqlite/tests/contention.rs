//! The retry path, taken for real, through the typed layer, on a real file.
//!
//! `src/main.rs` is single-writer, so `Retry::attempts(3)` is never spent and
//! `CommandError::Exhausted` is unreachable — the same gap
//! `course-subscriptions` has. The adapter's own `tests/concurrency.rs` races
//! `SqliteEventStore` properly, but it does so through the **conformance
//! suite**, against `happenstance-core`'s `append` and a hand-built
//! `AppendCondition`. Nothing raced `happenstance::commit`, which is the thing
//! an application actually calls and the thing that owns the retry.
//!
//! So the loop's central promise — *a contended commit re-reads, re-decides and
//! appends again, and no update is lost* — was held up by neither target. This
//! one holds it.
//!
//! # The contention is forced, not hoped for
//!
//! A test that spawns four threads and hopes they overlap is a test that passes
//! on a slow machine and proves nothing on a fast one. Timing is not evidence.
//!
//! `commit`'s `decide` closure is the seam that makes this deterministic
//! without a single private item: the loop calls it **after** the read and
//! **before** the append, so a `std::sync::Barrier` released inside it leaves
//! every contender holding a decision model built from the same pre-race state.
//! One append then wins and the rest are `ConditionViolated` by construction.
//! Each contender waits on its **first** attempt only — waiting again on the
//! retry would park `CONTENDERS - 1` threads against a barrier the winner has
//! already left, which is a deadlock rather than a test.
//!
//! **The barrier was measured, not assumed.** Seeded in the working tree only:
//! swap `Some(barrier)` for `None` in [`race`], leaving everything else alone,
//! and `a_contended_commit_retries_rather_than_failing` fails with
//! `attempts were [1, 1, 2, 1]` — one natural retry out of four contenders,
//! `left: 1, right: 3`. That is the flaky test this file would otherwise have
//! been: it *did* observe contention, on that run, on this machine, and would
//! have reported none on a slower or busier one. The other two tests stayed
//! green under the seed, correctly — they are about outcomes, not attempts.
//! The seed was reverted and appears in no commit.
//!
//! # Why bare OS threads, each with its own runtime and its own handle
//!
//! Copied from `crates/happenstance-sqlite/tests/concurrency.rs` rather than
//! reinvented, and one detail is load-bearing: ADR-0022 §9 captures the tokio
//! handle **at construction**, so a store built outside a runtime records
//! `None` and every read fails with `SqliteEventStoreError::NoRuntime`. Each
//! thread therefore builds its own `current_thread` runtime *first* and opens
//! its own `SqliteEventStore` *inside* it. One handle per contender onto one
//! file is also what an application deploys — a request handler per connection,
//! not one store shared behind a lock.
//!
//! A `std::sync::Barrier` and not `tokio::sync::Barrier`, because `decide` is a
//! synchronous `FnMut` and cannot await. Blocking it parks that contender's own
//! runtime and nothing else, since no two contenders share one.
//!
//! # Why the transfer domain is restated here
//!
//! This package is a binary with no lib target, so a test cannot import from
//! `src/main.rs` — the same constraint `course-subscriptions/tests/ui.rs`
//! records. The domain below is the smaller half of the application's: two
//! variants, no `AccountOpened`, one refusal. What it keeps is the part that
//! matters under contention — the **tuple boundary**, whose query is the union
//! of both members', so a conflict on *either* account must be caught by the
//! one append condition the composite produces.

#![allow(clippy::unwrap_used, reason = "test code, per the house style")]

use std::path::{Path, PathBuf};
use std::sync::Barrier;

use happenstance::bytes::Bytes;
use happenstance::{
    Boundary, Codec, CodecError, CommandError, Committed, DecisionModel, DomainEvent, EventType,
    InvalidTag, Json, Retry, Tag, Tags, commit, read_decision_model,
};
use happenstance_sqlite::event_store::{SqliteEventStore, SqliteEventStoreError};
use serde::{Deserialize, Serialize};

/// How many contenders race one commit.
///
/// Four rather than sixty-four. The adapter's own concurrency family already
/// establishes that the *store* survives depth; what is under test here is that
/// the *loop* re-reads and re-decides, and one contender losing is the whole of
/// that claim. Four keeps the barrier's failure mode legible: a hang is four
/// threads, not sixty-four.
const CONTENDERS: usize = 4;

/// What each contender moves.
const AMOUNT: u32 = 10;

/// What the payer starts with — enough that every transfer is affordable, so a
/// refusal can never be mistaken for a resolved race.
const OPENING: u32 = 100;

/// The retry bound.
///
/// Generous on purpose. Every contender is stale at the barrier, so the losers
/// re-race as they retry and one can lose repeatedly; a bound of
/// `CONTENDERS` would make an ordinary scheduling order look like a defect.
/// `Exhausted` here must mean the loop is broken, not that the machine was busy.
const ATTEMPTS: u32 = 32;

// ---------------------------------------------------------------------------
// The tests
// ---------------------------------------------------------------------------

/// A contended commit retries, and the retry is visible in `Committed`.
///
/// `attempts` is the only part of the loop's per-attempt state that is public,
/// and it is documented as *"the observable proof that the retry ran"*. This is
/// the test that observes it.
#[test]
fn a_contended_commit_retries_rather_than_failing() {
    let db = TempDb::new("retries");
    seed(&db);

    let outcomes = race(&db);

    let retried = outcomes
        .iter()
        .filter(|committed| committed.attempts > 1)
        .count();

    assert_eq!(
        retried,
        CONTENDERS - 1,
        "every contender but one was stale at the barrier, so exactly \
         {} of {CONTENDERS} must have re-read and re-decided; attempts were {:?}",
        CONTENDERS - 1,
        outcomes.iter().map(|c| c.attempts).collect::<Vec<_>>(),
    );
}

/// Every contender commits. None is refused, and none exhausts the bound.
///
/// The negative control for the test above. A loop that reported a retry and
/// then dropped the write would satisfy `attempts > 1` and lose money.
#[test]
fn every_contender_eventually_commits() {
    let db = TempDb::new("all-commit");
    seed(&db);

    let outcomes = race(&db);

    assert_eq!(
        outcomes.len(),
        CONTENDERS,
        "a contender did not commit at all"
    );
}

/// No update is lost, and none is applied twice.
///
/// The claim the retry exists to keep. Folded back through the same `Boundary`
/// the command loop uses, off the reopened file, so the arithmetic is read from
/// the log rather than from anything the race was still holding.
#[test]
fn no_update_is_lost_under_contention() {
    let db = TempDb::new("conserved");
    seed(&db);

    race(&db);

    let moved = AMOUNT * u32::try_from(CONTENDERS).unwrap();
    let (debited, credited) = final_balances(&db);

    assert_eq!(
        debited,
        u64::from(OPENING - moved),
        "the payer lost or double-counted a transfer"
    );
    assert_eq!(
        credited,
        u64::from(moved),
        "the payee did not receive them all"
    );
    assert_eq!(
        debited + credited,
        u64::from(OPENING),
        "money was created or destroyed"
    );
}

// ---------------------------------------------------------------------------
// The race
// ---------------------------------------------------------------------------

/// Runs `CONTENDERS` transfers that are all stale at the same instant.
///
/// Returns each contender's `Committed`. Any failure panics inside its own
/// thread, which `std::thread::scope` propagates on join.
fn race(db: &TempDb) -> Vec<Committed> {
    let barrier = Barrier::new(CONTENDERS);
    let path = db.path();

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..CONTENDERS)
            .map(|_| {
                let barrier = &barrier;
                scope.spawn(move || {
                    // The runtime first, then the store inside it: ADR-0022 §9
                    // captures the handle at construction, and a store built on
                    // a bare thread records `None`.
                    let runtime = tokio::runtime::Builder::new_current_thread()
                        .build()
                        .unwrap();

                    runtime.block_on(async {
                        let events = SqliteEventStore::open(path).unwrap();
                        transfer(&events, AMOUNT, Some(barrier)).await.unwrap()
                    })
                })
            })
            .collect();

        handles
            .into_iter()
            .map(|handle| handle.join().expect("a contender panicked"))
            .collect()
    })
}

/// Moves `amount` from the payer to the payee, atomically.
///
/// `gate`, when supplied, is waited on inside `decide` on the **first** attempt
/// only — after the read, before the append. That is what makes every
/// contender stale at the same moment.
async fn transfer(
    events: &SqliteEventStore,
    amount: u32,
    gate: Option<&Barrier>,
) -> Result<Committed, CommandError<SqliteEventStoreError, Refusal>> {
    // Unwrapped rather than `?`, because `CommandError` has no `From<InvalidTag>`
    // — its `Boundary` arm carries `InvalidQuery`, which is a different failure.
    // `src/main.rs` hides the distinction behind `anyhow::Result`; a function
    // returning the loop's own error type cannot, and a `"a1"` that stops being
    // a valid tag is a broken test rather than a refusal to report.
    let debit = AccountId::new("a1").unwrap();
    let credit = AccountId::new("a2").unwrap();
    let boundary = (
        Balance::new(&debit).unwrap(),
        Balance::new(&credit).unwrap(),
    );

    let mut first = true;

    commit(
        events,
        boundary,
        Retry::attempts(ATTEMPTS.try_into().unwrap()),
        // The payee's model is bound and deliberately not read. It earns its
        // place in the *query*: the composite's is the union of both members',
        // so the append condition catches a conflict on the payee as well as on
        // the payer. A boundary member is a consistency claim, not an input the
        // decision has to consume.
        |(from, _payee): &(Balance, Balance)| {
            if first {
                first = false;
                if let Some(gate) = gate {
                    gate.wait();
                }
            }

            if from.held < u64::from(amount) {
                return Err(Refusal::Overdrawn {
                    held: from.held,
                    needed: amount,
                });
            }

            Ok(vec![
                Ledger::Withdrawn {
                    account: debit.clone(),
                    amount,
                },
                Ledger::Deposited {
                    account: credit.clone(),
                    amount,
                },
            ])
        },
    )
    .await
}

/// Puts `OPENING` into the payer, uncontended.
fn seed(db: &TempDb) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();

    runtime.block_on(async {
        let events = SqliteEventStore::open(db.path()).unwrap();
        let payer = AccountId::new("a1").unwrap();
        let boundary = Balance::new(&payer).unwrap();

        let committed = commit(
            &events,
            boundary,
            Retry::attempts(ATTEMPTS.try_into().unwrap()),
            |_: &Balance| {
                Ok::<_, Refusal>(vec![Ledger::Deposited {
                    account: payer.clone(),
                    amount: OPENING,
                }])
            },
        )
        .await
        .unwrap();

        assert_eq!(
            committed.attempts, 1,
            "the seed is uncontended and must not retry"
        );
    });
}

/// Both balances, folded off the file through the same `Boundary` the loop uses.
fn final_balances(db: &TempDb) -> (u64, u64) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();

    runtime.block_on(async {
        let events = SqliteEventStore::open(db.path()).unwrap();
        (fold(&events, "a1").await, fold(&events, "a2").await)
    })
}

/// One account's balance, read back through `query` and `absorb`.
async fn fold(events: &SqliteEventStore, account: &str) -> u64 {
    let account = AccountId::new(account).unwrap();
    let mut balance = Balance::new(&account).unwrap();

    let query = balance.query().unwrap();
    let (found, _) = read_decision_model(events, &query).await.unwrap();
    for event in &found {
        balance.absorb(event, &Json).unwrap();
    }

    balance.held
}

// ---------------------------------------------------------------------------
// The domain, restated
// ---------------------------------------------------------------------------

/// An account identifier, validated once into the tag its events carry.
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

    /// The identifier as the test wrote it.
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

/// Money in, money out. No `AccountOpened`: the seed is the opening balance.
#[derive(Debug, Serialize, Deserialize)]
enum Ledger {
    /// The account was credited.
    Deposited {
        /// Who.
        account: AccountId,
        /// How much.
        amount: u32,
    },
    /// The account was debited.
    Withdrawn {
        /// Who.
        account: AccountId,
        /// How much.
        amount: u32,
    },
}

impl DomainEvent for Ledger {
    const EVENT_TYPES: &'static [EventType] = &[
        EventType::from_static("Deposited"),
        EventType::from_static("Withdrawn"),
    ];

    fn event_type(&self) -> EventType {
        match self {
            Self::Deposited { .. } => Self::EVENT_TYPES[0].clone(),
            Self::Withdrawn { .. } => Self::EVENT_TYPES[1].clone(),
        }
    }

    fn tags(&self) -> Tags {
        match self {
            Self::Deposited { account, .. } | Self::Withdrawn { account, .. } => {
                [account.tag.clone()].into_iter().collect()
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

/// The only way a decision here can refuse.
#[derive(Debug, thiserror::Error)]
enum Refusal {
    /// The payer cannot cover the transfer.
    #[error("holds {held}, needs {needed}")]
    Overdrawn {
        /// What the payer holds.
        held: u64,
        /// What the transfer asked for.
        needed: u32,
    },
}

/// One account's balance.
#[derive(Debug, Clone)]
struct Balance {
    scope: Tags,
    held: u64,
}

impl Balance {
    /// Validates the scope once, here.
    fn new(account: &AccountId) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("account", account.as_str())])?,
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
// The file each test owns
// ---------------------------------------------------------------------------

/// A database file removed when the test that owns it ends.
///
/// Named per test rather than per process, because these run concurrently in
/// one binary and two tests sharing a file would race on more than the commit.
struct TempDb(PathBuf);

impl TempDb {
    /// Creates a path nothing else in this run will use.
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "happenstance-contention-{}-{name}.db",
            std::process::id()
        ));
        let db = Self(path);
        db.remove();
        db
    }

    /// The file the stores open.
    fn path(&self) -> &Path {
        &self.0
    }

    /// Removes the database and the two files WAL mode keeps beside it.
    fn remove(&self) {
        for suffix in ["", "-wal", "-shm"] {
            let mut name = self.0.as_os_str().to_owned();
            name.push(suffix);
            drop(std::fs::remove_file(PathBuf::from(name)));
        }
    }
}

impl Drop for TempDb {
    fn drop(&mut self) {
        self.remove();
    }
}
