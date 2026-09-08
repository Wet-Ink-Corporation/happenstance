// The module documentation is `src/overview.md`, included rather than written
// here so the reader who is sent to it and the reader who runs `cargo doc` meet
// the same bytes by construction. Same mechanism, and the same argument, as
// `examples/course-subscriptions/src/main.rs`.
#![doc = include_str!("overview.md")]
#![allow(clippy::print_stdout, reason = "the transcript is the point")]

use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use happenstance::bytes::Bytes;
use happenstance::{
    Boundary, Codec, CodecError, CommandError, CommandOutcome, DecisionModel, DomainEvent,
    EventStore, EventType, InvalidTag, Query, ReadOptions, Retry, SequencePosition, Tag, Tags,
    collect, commit,
};
use happenstance_sqlite::event_store::{SqliteEventStore, SqliteEventStoreError};
use serde::{Deserialize, Serialize};

/// How many times a command may be attempted before it gives up.
///
/// Spelled at every call site for the reason `course-subscriptions` gives: a
/// loop whose only exit is success is a hang with better manners. This run is
/// single-writer, so the bound is never spent and every commit below reports
/// one attempt — a visible bound on an unexercised path.
const ATTEMPTS: u32 = 3;

/// How many handles an owner may hold before anybody has said otherwise.
///
/// A fold's starting value and not a row in a table, which is the whole reason
/// the quota needs no aggregate: an owner who has never been heard of has a
/// plan, because the fold gives them one.
const DEFAULT_PLAN: u32 = 2;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let path = database_path();
    remove_database(&path);

    println!("== the database this run writes to ==");
    println!("   {}", path.display());

    // The run is a function so every store and connection it builds is dropped
    // before the file is removed — on Windows an open handle is what makes the
    // cleanup silently fail and the next run start on the tail of this one.
    let outcome = run(&path).await;
    remove_database(&path);
    outcome
}

/// Everything this program does, against one file.
async fn run(path: &Path) -> Result<()> {
    let events = SqliteEventStore::open(path)?;

    the_boundary_is_a_query()?;
    a_name_is_held_by_one_owner(&events).await?;
    a_released_name_is_claimable_again(&events).await?;
    a_plan_caps_what_an_owner_holds(&events).await?;
    one_command_delivered_twice(&events).await?;

    println!("\n== what the log actually says ==");
    print_log(&events).await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// The five things this program shows
// ---------------------------------------------------------------------------

/// Prints the query a claim derives, before any of them is taken.
///
/// The point the rest of the program is evidence for, made once and in the
/// open: the consistency boundary is a **value**. It can be bound, printed and
/// asserted on, and it is derived from the event set crossed with each model's
/// scope — so the query and the folds that consume it cannot name different
/// sets. Nothing here is hidden inside a read.
fn the_boundary_is_a_query() -> Result<()> {
    println!("\n== the boundary is a query, not an entity ==");

    let owner = OwnerId::new("u-1")?;
    let handle = HandleName::new("alice")?;
    let request = RequestId::new("r-1")?;
    let boundary = (
        Handle::new(&handle)?,
        Holdings::new(&owner)?,
        Request::new(&request)?,
    );

    println!("   claiming `alice` for u-1 reads three things at once:");
    print_boundary(&boundary)?;
    println!(
        "   one event set, three scopes, three query items. There is no Handle\n   \
         object, no Owner row and no Request table — each rule is the set of\n   \
         events that could contradict it, and the append is conditioned on all\n   \
         three of them at once."
    );

    Ok(())
}

/// Claims a name, and shows the second claimant refused.
async fn a_name_is_held_by_one_owner(events: &SqliteEventStore) -> Result<()> {
    println!("\n== a name is held by one owner ==");

    println!(
        "   u-1 claims alice: {}",
        claim(events, "u-1", "alice", "r-1").await?
    );

    match claim(events, "u-2", "alice", "r-2").await {
        Ok(outcome) => bail!("a claim on a held name should have been refused, got: {outcome}"),
        Err(err) => println!("   u-2 claims alice: refused: {err}"),
    }

    Ok(())
}

/// Releases the name and lets somebody else have it.
///
/// The half of rule 1 that a unique index cannot state without a delete: the
/// boundary is not "names that are taken", it is "everything that has happened
/// to this name", and a release is one of the things that can have happened.
async fn a_released_name_is_claimable_again(events: &SqliteEventStore) -> Result<()> {
    println!("\n== a released name is claimable again ==");

    println!(
        "   u-1 releases alice: {}",
        release(events, "u-1", "alice", "r-3").await?
    );
    println!(
        "   u-2 claims alice:   {}",
        claim(events, "u-2", "alice", "r-4").await?
    );
    println!(
        "   the log still holds u-1's claim; what changed is what the fold of\n   \
         that name now says, which is the only thing the condition consults"
    );

    Ok(())
}

/// Fills an owner's plan, is refused, raises the plan, and is not.
///
/// The second boundary, composed with the first in every one of these
/// commands. It spans a set nobody can enumerate in advance — the handles this
/// owner holds — and it is enforced by the same append condition that enforces
/// the name.
async fn a_plan_caps_what_an_owner_holds(events: &SqliteEventStore) -> Result<()> {
    println!("\n== a plan caps what an owner holds ==");

    println!(
        "   u-2 claims bob:   {}",
        claim(events, "u-2", "bob", "r-5").await?
    );

    match claim(events, "u-2", "carol", "r-6").await {
        Ok(outcome) => bail!("a claim over the plan should have been refused, got: {outcome}"),
        Err(err) => println!("   u-2 claims carol: refused: {err}"),
    }

    println!(
        "   u-2 moves to a plan of 3: {}",
        change_plan(events, "u-2", 3, "r-7").await?
    );
    println!(
        "   u-2 claims carol: {}",
        claim(events, "u-2", "carol", "r-8").await?
    );

    Ok(())
}

/// Sends one command twice under one request id, and appends once.
///
/// The third boundary. It is not domain state at all — it is whether *this
/// delivery* has already been applied — and it is enforced the same way as the
/// other two, which is the point: an idempotency key does not need a table, a
/// lock or a separate transaction when it can be a scope.
async fn one_command_delivered_twice(events: &SqliteEventStore) -> Result<()> {
    println!("\n== one command, delivered twice ==");

    let before = head(events).await?;
    println!(
        "   first delivery:  {}",
        claim(events, "u-1", "dave", "r-9").await?
    );
    println!(
        "   second delivery: {}",
        claim(events, "u-1", "dave", "r-9").await?
    );
    let after = head(events).await?;

    let grew = after.saturating_sub(before);
    if grew != 1 {
        bail!("a command delivered twice appended {grew} event(s), not 1");
    }
    println!("   the log grew by {grew} event, not 2");
    println!(
        "   and the second delivery is not an error: it restates what the first\n   \
         one did, which is what a client that timed out and retried needs"
    );

    Ok(())
}

// ---------------------------------------------------------------------------
// Identity, validated once at the edge
// ---------------------------------------------------------------------------

/// An owner, validated once into the tag their events carry.
///
/// The resolution the other examples reach, and for the same reason:
/// `DomainEvent::tags` is infallible while `Tag::key_value` can refuse, so a
/// domain type whose tags come from runtime values holds the validated form
/// instead of rebuilding it on every call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
struct OwnerId {
    /// The identifier as the application wrote it.
    id: String,
    /// The same value, validated into a tag.
    tag: Tag,
}

impl OwnerId {
    /// Validates `id` into the tag every event for this owner is written with.
    fn new(id: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            id: id.to_owned(),
            tag: Tag::key_value("owner", id)?,
        })
    }

    /// The identifier as the application wrote it.
    fn as_str(&self) -> &str {
        &self.id
    }
}

impl TryFrom<String> for OwnerId {
    type Error = InvalidTag;

    fn try_from(id: String) -> Result<Self, InvalidTag> {
        Self::new(&id)
    }
}

impl From<OwnerId> for String {
    fn from(owner: OwnerId) -> Self {
        owner.id
    }
}

/// A handle, validated once into the tag its events carry.
///
/// **This is the type with no entity behind it.** A handle nobody has claimed
/// is not an object in an unclaimed state; it is a string drawn from a set
/// nobody can enumerate. What makes it a consistency boundary is not that it
/// exists, but that a query can be written for it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
struct HandleName {
    /// The handle as the application wrote it.
    id: String,
    /// The same value, validated into a tag.
    tag: Tag,
}

impl HandleName {
    /// Validates `id` into the tag every event for this handle is written with.
    fn new(id: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            id: id.to_owned(),
            tag: Tag::key_value("handle", id)?,
        })
    }

    /// The handle as the application wrote it.
    fn as_str(&self) -> &str {
        &self.id
    }
}

impl TryFrom<String> for HandleName {
    type Error = InvalidTag;

    fn try_from(id: String) -> Result<Self, InvalidTag> {
        Self::new(&id)
    }
}

impl From<HandleName> for String {
    fn from(handle: HandleName) -> Self {
        handle.id
    }
}

/// One delivery of one command, validated once into the tag its events carry.
///
/// Supplied by the caller, not minted here. That is what makes it an
/// idempotency key rather than an event id: two deliveries of one command
/// carry the same value precisely because the client, not the server, decided
/// what it was.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
struct RequestId {
    /// The request as the client wrote it.
    id: String,
    /// The same value, validated into a tag.
    tag: Tag,
}

impl RequestId {
    /// Validates `id` into the tag every event this delivery writes carries.
    fn new(id: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            id: id.to_owned(),
            tag: Tag::key_value("request", id)?,
        })
    }

    /// The request as the client wrote it.
    fn as_str(&self) -> &str {
        &self.id
    }
}

impl TryFrom<String> for RequestId {
    type Error = InvalidTag;

    fn try_from(id: String) -> Result<Self, InvalidTag> {
        Self::new(&id)
    }
}

impl From<RequestId> for String {
    fn from(request: RequestId) -> Self {
        request.id
    }
}

// ---------------------------------------------------------------------------
// The domain: one enum, named once
// ---------------------------------------------------------------------------

/// Everything that can happen in the registry.
///
/// Every variant carries the `request` that wrote it, and that is not
/// bookkeeping — it is what puts a delivery inside the same tag space as the
/// domain, so the same query vocabulary reaches it and the same append
/// condition covers it.
///
/// One enum, so a query and a fold cannot name different sets. There is no
/// `_ =>` arm anywhere in this file.
#[derive(Debug, Serialize, Deserialize)]
enum Registry {
    /// An owner took a handle.
    HandleClaimed {
        /// The handle now held.
        handle: HandleName,
        /// Who holds it.
        owner: OwnerId,
        /// The delivery that wrote this.
        request: RequestId,
    },
    /// An owner gave a handle back.
    HandleReleased {
        /// The handle now free.
        handle: HandleName,
        /// Who was holding it.
        owner: OwnerId,
        /// The delivery that wrote this.
        request: RequestId,
    },
    /// An owner's plan changed how many handles they may hold.
    PlanChanged {
        /// Whose plan.
        owner: OwnerId,
        /// How many handles it now allows.
        handles: u32,
        /// The delivery that wrote this.
        request: RequestId,
    },
}

impl DomainEvent for Registry {
    const EVENT_TYPES: &'static [EventType] = &[
        EventType::from_static("HandleClaimed"),
        EventType::from_static("HandleReleased"),
        EventType::from_static("PlanChanged"),
    ];

    fn event_type(&self) -> EventType {
        match self {
            Self::HandleClaimed { .. } => Self::EVENT_TYPES[0].clone(),
            Self::HandleReleased { .. } => Self::EVENT_TYPES[1].clone(),
            Self::PlanChanged { .. } => Self::EVENT_TYPES[2].clone(),
        }
    }

    fn tags(&self) -> Tags {
        match self {
            Self::HandleClaimed {
                handle,
                owner,
                request,
            }
            | Self::HandleReleased {
                handle,
                owner,
                request,
            } => [handle.tag.clone(), owner.tag.clone(), request.tag.clone()]
                .into_iter()
                .collect(),
            // No handle tag, so a claim's boundary never nominates a plan
            // change — which is correct: what an owner is allowed to hold has
            // nothing to say about who holds one particular name.
            Self::PlanChanged { owner, request, .. } => [owner.tag.clone(), request.tag.clone()]
                .into_iter()
                .collect(),
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

// ---------------------------------------------------------------------------
// What a command did, and why one refused
// ---------------------------------------------------------------------------

/// What a delivery turned out to have done.
///
/// Folded out of the log rather than remembered, which is what makes it
/// survive a restart: a replay learns what the first delivery did by reading
/// the event that delivery wrote.
#[derive(Debug, Clone)]
enum Landed {
    /// It claimed a handle.
    Claimed {
        /// The handle it claimed.
        handle: String,
    },
    /// It released one.
    Released {
        /// The handle it released.
        handle: String,
    },
    /// It set a plan.
    PlanSet {
        /// How many handles the plan now allows.
        handles: u32,
    },
}

impl core::fmt::Display for Landed {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Claimed { handle } => write!(formatter, "claimed {handle}"),
            Self::Released { handle } => write!(formatter, "released {handle}"),
            Self::PlanSet { handles } => write!(formatter, "set the plan to {handles}"),
        }
    }
}

/// What a command call returned.
///
/// Two arms rather than a `Result<(), _>` with a special error, because a
/// replay is a **success**: the client asked for a thing to be true and it is
/// true. Modelling it as a failure is how retries turn into user-visible
/// errors that nobody can act on.
#[derive(Debug)]
enum Outcome {
    /// The decision was taken and its events were appended.
    Applied {
        /// Where the last appended event landed.
        position: u64,
        /// How many attempts the command loop spent, the successful one
        /// included.
        attempts: u32,
    },
    /// This delivery had already been applied, and nothing was appended.
    Replayed {
        /// What the first delivery did.
        landed: Landed,
    },
}

impl core::fmt::Display for Outcome {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Applied { position, attempts } => write!(
                formatter,
                "applied at position {position}, in {attempts} attempt(s)"
            ),
            Self::Replayed { landed } => {
                write!(formatter, "already applied — {landed}; nothing appended")
            }
        }
    }
}

/// Why a decision refused.
///
/// Every variant carries the value the transcript prints — `alice is held by
/// u-1` and never `conflict` — so a reader can act on the refusal where it is
/// printed instead of opening a second document to learn what happened.
#[derive(Debug, thiserror::Error)]
enum Refusal {
    /// Rule 1: a handle is held by at most one owner.
    #[error("{handle} is held by {owner}")]
    HandleTaken {
        /// The handle somebody else has.
        handle: String,
        /// Who has it.
        owner: String,
    },
    /// A handle cannot be released by somebody who is not holding it.
    #[error("{handle} is not held by {owner}")]
    NotHeld {
        /// The handle in question.
        handle: String,
        /// Who tried to give it back.
        owner: String,
    },
    /// Rule 2: an owner may not exceed their plan.
    #[error("{owner} holds {held} of {cap}")]
    PlanFull {
        /// The owner at their limit.
        owner: String,
        /// How many they hold.
        held: u32,
        /// How many they may hold.
        cap: u32,
    },
    /// Rule 3: this delivery has already been applied.
    ///
    /// **Not a failure**, and the only variant here that is matched on by
    /// value rather than printed: `settle` turns it into
    /// [`Outcome::Replayed`]. It is spelled as a refusal because that is what
    /// it is to the decision — there is nothing left to append — and the
    /// translation into a success belongs one layer out, where the client is.
    #[error("already applied — {landed}")]
    AlreadyApplied {
        /// What the first delivery did.
        landed: Landed,
    },
}

// ---------------------------------------------------------------------------
// The decision models: one per rule
// ---------------------------------------------------------------------------

/// Who holds one handle, if anybody.
///
/// Scoped to a single `handle` tag, so its query nominates every event ever
/// written about that name — including the releases, which is what makes the
/// boundary dynamic.
#[derive(Debug, Clone)]
struct Handle {
    /// The tags every event inside this boundary carries.
    scope: Tags,
    /// Who holds it now, or `None` if nobody does.
    held_by: Option<String>,
}

impl Handle {
    /// Validates the scope once, here, where the caller already writes a `?`.
    fn new(handle: &HandleName) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("handle", handle.as_str())])?,
            held_by: None,
        })
    }
}

impl DecisionModel for Handle {
    type Event = Registry;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Registry::HandleClaimed { owner, .. } => self.held_by = Some(owner.id),
            Registry::HandleReleased { .. } => self.held_by = None,
            // A plan change carries no `handle` tag, so this model's query
            // never nominates one. The arm exists because the fold is
            // exhaustive over the enum, and that exhaustiveness is the whole
            // guarantee.
            Registry::PlanChanged { .. } => {}
        }
    }
}

/// How many handles one owner holds, and how many they may.
///
/// Scoped to a single `owner` tag. The count is a fold over a set nobody
/// enumerated — every handle this owner has ever taken or given back — which
/// is the shape a quota actually has and the shape an aggregate cannot hold
/// without owning every handle in the system.
#[derive(Debug, Clone)]
struct Holdings {
    /// The tags every event inside this boundary carries.
    scope: Tags,
    /// How many handles are currently held.
    held: u32,
    /// How many may be held.
    cap: u32,
}

impl Holdings {
    /// Validates the scope once, here, where the caller already writes a `?`.
    fn new(owner: &OwnerId) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("owner", owner.as_str())])?,
            held: 0,
            cap: DEFAULT_PLAN,
        })
    }
}

impl DecisionModel for Holdings {
    type Event = Registry;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Registry::HandleClaimed { .. } => self.held = self.held.saturating_add(1),
            Registry::HandleReleased { .. } => self.held = self.held.saturating_sub(1),
            Registry::PlanChanged { handles, .. } => self.cap = handles,
        }
    }
}

/// Whether one delivery has already been applied, and what it did.
///
/// Scoped to a single `request` tag. It is the smallest useful decision model
/// in this file and the one that does the most: because it is part of the same
/// boundary as the other two, the check and the write are one conditional
/// append, and two concurrent deliveries of a command cannot both land.
#[derive(Debug, Clone)]
struct Request {
    /// The tags every event inside this boundary carries.
    scope: Tags,
    /// What this delivery already did, if it has run.
    landed: Option<Landed>,
}

impl Request {
    /// Validates the scope once, here, where the caller already writes a `?`.
    fn new(request: &RequestId) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("request", request.as_str())])?,
            landed: None,
        })
    }
}

impl DecisionModel for Request {
    type Event = Registry;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        self.landed = Some(match event {
            Registry::HandleClaimed { handle, .. } => Landed::Claimed { handle: handle.id },
            Registry::HandleReleased { handle, .. } => Landed::Released { handle: handle.id },
            Registry::PlanChanged { handles, .. } => Landed::PlanSet { handles },
        });
    }
}

// ---------------------------------------------------------------------------
// The three command handlers
// ---------------------------------------------------------------------------

/// Claims a handle for an owner, under one delivery.
///
/// The three-tuple is the boundary, and there is no macro invocation and no
/// builder in the way: a tuple of decision models is a `Boundary`, its query
/// is the union of theirs, and one append condition carries all three.
async fn claim(
    events: &SqliteEventStore,
    owner: &str,
    handle: &str,
    request: &str,
) -> Result<Outcome> {
    let owner = OwnerId::new(owner)?;
    let handle = HandleName::new(handle)?;
    let request = RequestId::new(request)?;
    let boundary = (
        Handle::new(&handle)?,
        Holdings::new(&owner)?,
        Request::new(&request)?,
    );

    let result = commit(
        events,
        boundary,
        Retry::attempts(ATTEMPTS.try_into()?),
        |(name, holdings, delivery): &(Handle, Holdings, Request)| {
            // Rule 3 first. A replayed delivery must not be re-judged against
            // rules 1 and 2 — the world has moved on since it was applied, and
            // re-judging it would report a conflict for work that succeeded.
            if let Some(landed) = delivery.landed.clone() {
                return Err(Refusal::AlreadyApplied { landed });
            }
            if let Some(holder) = name.held_by.clone() {
                return Err(Refusal::HandleTaken {
                    handle: handle.id.clone(),
                    owner: holder,
                });
            }
            if holdings.held >= holdings.cap {
                return Err(Refusal::PlanFull {
                    owner: owner.id.clone(),
                    held: holdings.held,
                    cap: holdings.cap,
                });
            }
            Ok(vec![Registry::HandleClaimed {
                handle: handle.clone(),
                owner: owner.clone(),
                request: request.clone(),
            }])
        },
    )
    .await;

    settle(result)
}

/// Gives a handle back, under one delivery.
async fn release(
    events: &SqliteEventStore,
    owner: &str,
    handle: &str,
    request: &str,
) -> Result<Outcome> {
    let owner = OwnerId::new(owner)?;
    let handle = HandleName::new(handle)?;
    let request = RequestId::new(request)?;
    let boundary = (Handle::new(&handle)?, Request::new(&request)?);

    let result = commit(
        events,
        boundary,
        Retry::attempts(ATTEMPTS.try_into()?),
        |(name, delivery): &(Handle, Request)| {
            if let Some(landed) = delivery.landed.clone() {
                return Err(Refusal::AlreadyApplied { landed });
            }
            if name.held_by.as_deref() != Some(owner.as_str()) {
                return Err(Refusal::NotHeld {
                    handle: handle.id.clone(),
                    owner: owner.id.clone(),
                });
            }
            Ok(vec![Registry::HandleReleased {
                handle: handle.clone(),
                owner: owner.clone(),
                request: request.clone(),
            }])
        },
    )
    .await;

    settle(result)
}

/// Sets how many handles an owner may hold, under one delivery.
///
/// Two models here rather than three: a plan has nothing to say about any
/// particular name, so the handle boundary is not part of this decision and is
/// not in its condition. Composing it in anyway would widen the condition to
/// events that cannot contradict it, and every unrelated claim would then be a
/// spurious retry.
async fn change_plan(
    events: &SqliteEventStore,
    owner: &str,
    handles: u32,
    request: &str,
) -> Result<Outcome> {
    let owner = OwnerId::new(owner)?;
    let request = RequestId::new(request)?;
    let boundary = (Holdings::new(&owner)?, Request::new(&request)?);

    let result = commit(
        events,
        boundary,
        Retry::attempts(ATTEMPTS.try_into()?),
        |(_holdings, delivery): &(Holdings, Request)| {
            if let Some(landed) = delivery.landed.clone() {
                return Err(Refusal::AlreadyApplied { landed });
            }
            Ok(vec![Registry::PlanChanged {
                owner: owner.clone(),
                handles,
                request: request.clone(),
            }])
        },
    )
    .await;

    settle(result)
}

// ---------------------------------------------------------------------------
// Turning a command's result into what a client is told
// ---------------------------------------------------------------------------

/// Renders a command's result as an outcome, or as the error to print.
///
/// The one place `Refusal::AlreadyApplied` becomes a success. Everything else
/// travels out with its source chain intact: `CommandError::Refused` carries
/// the handler's own refusal, and that is what a reader must see, because
/// `CommandError`'s own `Display` says "the decision refused", which is a
/// category rather than a value.
fn settle(
    result: core::result::Result<CommandOutcome, CommandError<SqliteEventStoreError, Refusal>>,
) -> Result<Outcome> {
    match result {
        Ok(CommandOutcome::Committed(committed)) => Ok(Outcome::Applied {
            position: committed.position.get(),
            attempts: committed.attempts,
        }),

        // **This example refuses instead of deciding nothing, and the arm is
        // here to say that is a choice.** `CommandOutcome::Nothing` and
        // `Refusal::AlreadyApplied` both mean *this delivery was already
        // handled*; they differ in what they can carry. `Nothing` carries no
        // payload, and the whole point of the idempotent path below is
        // reporting `landed` — what the *first* delivery did — which a handler
        // that returned an empty `Vec` would have thrown away before this
        // function saw it.
        //
        // So reaching this arm means a handler returned no events on a path
        // that was supposed to refuse, and it is reported rather than folded
        // into `Applied` with a fabricated position. Deciding nothing is the
        // right shape when there is nothing to say; here there is.
        Ok(CommandOutcome::Nothing) => bail!(
            "a handler decided no events: this example's idempotent path              refuses with `AlreadyApplied` so that it can report what the              first delivery landed, and an empty decision has nowhere to put it"
        ),

        Err(CommandError::Refused(Refusal::AlreadyApplied { landed })) => {
            Ok(Outcome::Replayed { landed })
        }
        Err(CommandError::Refused(refusal)) => Err(refusal.into()),
        Err(other) => Err(other.into()),
    }
}

// ---------------------------------------------------------------------------
// Reading
// ---------------------------------------------------------------------------

/// Prints the query items a boundary derives, one per line.
///
/// Generic over `Boundary`, which is what a tuple of decision models becomes.
/// The trait is sealed, so this signature can consume one and no crate outside
/// `happenstance` can add a third kind of implementor to surprise it.
fn print_boundary<B: Boundary>(boundary: &B) -> Result<()> {
    let query = boundary.query()?;

    match query.items() {
        Some(items) => {
            for item in items {
                let types: Vec<&str> = item.types().iter().map(EventType::as_str).collect();
                println!("      {} where {:?}", types.join(" | "), item.tags());
            }
        }
        // Unreachable for a well-formed model, and spelled rather than
        // `unwrap`ped: `Query::All` is what a boundary constraining neither
        // types nor tags would produce, and an empty `EVENT_TYPES` is already
        // a compile error.
        None => println!("      the whole log"),
    }

    Ok(())
}

/// The position of the last event in the store, or 0 if there is none.
async fn head(events: &SqliteEventStore) -> Result<u64> {
    Ok(events.head().await?.map_or(0, SequencePosition::get))
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
/// Under the process id, so a `cargo run` and the test that spawns the binary
/// never meet on one file.
fn database_path() -> PathBuf {
    std::env::temp_dir().join(format!("happenstance-handles-{}.db", std::process::id()))
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
