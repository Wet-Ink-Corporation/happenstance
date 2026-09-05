//! The domain two processes share, and the protocol they speak.
//!
//! This is the only example in the workspace with a library target, and the
//! reason is the subject: an API process and a projection runner have to agree
//! about a domain, and the only way to be certain they do is for there to be
//! one copy of it. A `src/main.rs` has nothing importable in it, which is why
//! the other examples' tests restate their domains with `include_str!`
//! assertions instead of using them.
//!
//! See `src/bin/tickets-demo.rs` for what the two processes are made to do.

pub mod http;

use std::path::Path;

use happenstance::bytes::Bytes;
use happenstance::{
    Checkpoint, Codec, CodecError, CommandError, DecisionModel, DomainEvent, EventType, InvalidTag,
    Projection, ProjectionId, ProjectionStore, Retry, SequencePosition, Tag, Tags, commit,
};
use happenstance_sqlite::connection::open_configured;
use happenstance_sqlite::event_store::{SqliteEventStore, SqliteEventStoreError};
use happenstance_sqlite::projection_store::{
    SqliteBatch, SqliteProjectionStore, SqliteProjectionStoreError,
};
use rusqlite::Connection;
use rusqlite::types::Value;
use serde::{Deserialize, Serialize};

/// The one show this application sells seats for.
pub const SHOW: &str = "gala";

/// How many seats that show has.
///
/// Small on purpose. The demonstration is that more clients ask for seats than
/// exist, concurrently, and that the count comes out right anyway.
pub const CAPACITY: u32 = 5;

/// How many times a reservation may be attempted before it gives up.
///
/// Larger than the other examples' three, and for a reason those examples do
/// not have: every reservation contends on the show's own total, so a burst of
/// concurrent requests really does spend attempts. Running out is
/// [`Reservation::Contended`] and a `503`, never a hang.
pub const ATTEMPTS: u32 = 8;

/// How many events one projection chunk commits at a time.
pub const CHUNK: usize = 16;

/// The read model the runner maintains and the API serves.
pub const PROJECTION: &str = "seat_holder";

/// The read-model table this application owns.
///
/// It is the application's table, not the adapter's: `SqliteProjectionStore`
/// migrates `projection_checkpoint` and `projection_meta` and stops there.
pub const READ_MODEL_DDL: &str = "
CREATE TABLE IF NOT EXISTS seat_holder (
    seat   TEXT PRIMARY KEY,
    patron TEXT NOT NULL
) STRICT;
";

/// Creates the read-model table if it is not already there.
///
/// Called by all three binaries rather than by whichever one happens to start
/// first. Two processes racing to create the same table is exactly what
/// `IF NOT EXISTS` is for, and deciding that one of them "owns" startup would
/// be a rule the other one has to be trusted to obey.
///
/// # Errors
///
/// Returns the driver's error if the file cannot be opened or the statement
/// cannot be run.
pub fn ensure_schema(path: &Path) -> rusqlite::Result<()> {
    open_configured(path)?.execute_batch(READ_MODEL_DDL)
}

// ---------------------------------------------------------------------------
// Identity, validated once at the edge
// ---------------------------------------------------------------------------

/// A show, validated once into the tag its events carry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ShowId {
    /// The identifier as the application wrote it.
    id: String,
    /// The same value, validated into a tag.
    tag: Tag,
}

/// A seat, validated once into the tag its events carry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct SeatId {
    /// The identifier as the application wrote it.
    id: String,
    /// The same value, validated into a tag.
    tag: Tag,
}

/// A patron, validated once into the tag its events carry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PatronId {
    /// The identifier as the application wrote it.
    id: String,
    /// The same value, validated into a tag.
    tag: Tag,
}

/// Writes the three identical newtype bodies.
///
/// A macro rather than three copies, and it is internal — it is not exported,
/// takes no callback and has one arm, so it is a spelling convenience and not
/// an extension point. The three types are genuinely the same shape, and the
/// resolution behind that shape is the one the other examples reach:
/// `DomainEvent::tags` is infallible while `Tag::key_value` can refuse, so a
/// domain type whose tags come from runtime values holds the validated form
/// instead of rebuilding it on every call.
macro_rules! identity {
    ($type:ident, $key:literal) => {
        impl $type {
            /// Validates `id` into the tag every event for it is written with.
            ///
            /// # Errors
            ///
            /// Returns [`InvalidTag`] when `id` cannot be part of a tag —
            /// empty, too long, or carrying a control character.
            pub fn new(id: &str) -> Result<Self, InvalidTag> {
                Ok(Self {
                    id: id.to_owned(),
                    tag: Tag::key_value($key, id)?,
                })
            }

            /// The identifier as the application wrote it.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.id
            }
        }

        impl TryFrom<String> for $type {
            type Error = InvalidTag;

            fn try_from(id: String) -> Result<Self, InvalidTag> {
                Self::new(&id)
            }
        }

        impl From<$type> for String {
            fn from(value: $type) -> Self {
                value.id
            }
        }
    };
}

identity!(ShowId, "show");
identity!(SeatId, "seat");
identity!(PatronId, "patron");

// ---------------------------------------------------------------------------
// The domain: one enum, named once
// ---------------------------------------------------------------------------

/// Everything that can happen while a show is sold.
///
/// One enum, so a query and a fold cannot name different sets. There is no
/// `_ =>` arm anywhere in this crate.
#[derive(Debug, Serialize, Deserialize)]
pub enum Ticketing {
    /// A show went on sale, with a fixed number of seats.
    ShowOpened {
        /// The show now selling.
        show: ShowId,
        /// How many seats it has.
        capacity: u32,
    },
    /// A patron took a seat.
    SeatReserved {
        /// The show the seat belongs to.
        show: ShowId,
        /// The seat now held.
        seat: SeatId,
        /// Who holds it.
        patron: PatronId,
    },
    /// A patron gave a seat back.
    SeatReleased {
        /// The show the seat belongs to.
        show: ShowId,
        /// The seat now free.
        seat: SeatId,
        /// Who was holding it.
        patron: PatronId,
    },
}

impl DomainEvent for Ticketing {
    const EVENT_TYPES: &'static [EventType] = &[
        EventType::from_static("ShowOpened"),
        EventType::from_static("SeatReserved"),
        EventType::from_static("SeatReleased"),
    ];

    fn event_type(&self) -> EventType {
        match self {
            Self::ShowOpened { .. } => Self::EVENT_TYPES[0].clone(),
            Self::SeatReserved { .. } => Self::EVENT_TYPES[1].clone(),
            Self::SeatReleased { .. } => Self::EVENT_TYPES[2].clone(),
        }
    }

    fn tags(&self) -> Tags {
        match self {
            Self::ShowOpened { show, .. } => [show.tag.clone()].into_iter().collect(),
            Self::SeatReserved { show, seat, patron }
            | Self::SeatReleased { show, seat, patron } => {
                [show.tag.clone(), seat.tag.clone(), patron.tag.clone()]
                    .into_iter()
                    .collect()
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

/// Why a reservation refused.
///
/// Every variant carries the value the response body prints — `a-1 is held by
/// p-2` and never `conflict` — so a client can act on a `409` without a second
/// request.
#[derive(Debug, thiserror::Error)]
pub enum Refusal {
    /// Seats cannot be sold for a show nobody opened.
    #[error("show {show} is not open")]
    NotOpen {
        /// The show nobody opened.
        show: String,
    },
    /// A seat is held by at most one patron.
    #[error("{seat} is held by {patron}")]
    SeatTaken {
        /// The seat somebody else has.
        seat: String,
        /// Who has it.
        patron: String,
    },
    /// The show has sold every seat it has.
    #[error("{show} is full ({taken}/{capacity})")]
    Full {
        /// The show with nothing left.
        show: String,
        /// How many seats are held.
        taken: u32,
        /// How many there are.
        capacity: u32,
    },
}

// ---------------------------------------------------------------------------
// The decision models: one per consistency concern
// ---------------------------------------------------------------------------

/// Who holds one seat, if anybody.
#[derive(Debug, Clone)]
pub struct Seat {
    /// The tags every event inside this boundary carries.
    scope: Tags,
    /// Who holds it now, or `None` if nobody does.
    taken_by: Option<String>,
}

impl Seat {
    /// Validates the scope once, here, where the caller already writes a `?`.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidTag`] when the seat cannot be part of a tag.
    pub fn new(seat: &SeatId) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("seat", seat.as_str())])?,
            taken_by: None,
        })
    }
}

impl DecisionModel for Seat {
    type Event = Ticketing;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Ticketing::SeatReserved { patron, .. } => self.taken_by = Some(patron.id),
            Ticketing::SeatReleased { .. } => self.taken_by = None,
            // An opening carries no `seat` tag, so this model's query never
            // nominates one. The arm exists because the fold is exhaustive
            // over the enum, and that exhaustiveness is the whole guarantee.
            Ticketing::ShowOpened { .. } => {}
        }
    }
}

/// One show's capacity, and how much of it is spoken for.
///
/// **This is the contended boundary.** Every reservation for this show reads
/// it and conditions on it, so two requests for two *different* seats still
/// collide — which is what makes the retry loop in `commit` do visible work in
/// this example and in none of the others.
#[derive(Debug, Clone)]
pub struct Show {
    /// The tags every event inside this boundary carries.
    scope: Tags,
    /// How many seats there are, once the show has been opened.
    capacity: Option<u32>,
    /// How many are held.
    taken: u32,
}

impl Show {
    /// Validates the scope once, here, where the caller already writes a `?`.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidTag`] when the show cannot be part of a tag.
    pub fn new(show: &ShowId) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("show", show.as_str())])?,
            capacity: None,
            taken: 0,
        })
    }
}

impl DecisionModel for Show {
    type Event = Ticketing;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            // Decoded, never scraped from the payload's bytes: a show with no
            // capacity is a decode failure naming its position, and never a
            // show with a capacity of zero.
            Ticketing::ShowOpened { capacity, .. } => self.capacity = Some(capacity),
            Ticketing::SeatReserved { .. } => self.taken = self.taken.saturating_add(1),
            Ticketing::SeatReleased { .. } => self.taken = self.taken.saturating_sub(1),
        }
    }
}

// ---------------------------------------------------------------------------
// The two commands
// ---------------------------------------------------------------------------

/// Opens the show, once.
///
/// # Errors
///
/// Returns the command loop's error, including the refusal when the show has
/// already been opened.
pub async fn open_show(
    events: &SqliteEventStore,
    capacity: u32,
) -> Result<(), CommandError<SqliteEventStoreError, Refusal>> {
    let show = ShowId::new(SHOW).map_err(|_| unreachable_show())?;
    let boundary = Show::new(&show).map_err(|_| unreachable_show())?;

    commit(events, boundary, retry_bound(), |state: &Show| {
        if state.capacity.is_some() {
            return Err(Refusal::NotOpen {
                show: SHOW.to_owned(),
            });
        }
        Ok(vec![Ticketing::ShowOpened {
            show: show.clone(),
            capacity,
        }])
    })
    .await
    .map(|_| ())
}

/// What answering a reservation came to.
///
/// A value rather than a `Result`, because all four arms are ordinary answers
/// a client has to be told apart: a conflict is not a bug, a spent retry bound
/// is not a conflict, and neither is a broken database. The API turns each one
/// into its own status code and nothing else in the process has to decide.
#[derive(Debug, Clone)]
pub enum Reservation {
    /// The seat was taken. `201`.
    Taken {
        /// Where the event landed. A client sends this back on a read to say
        /// which version of the world it expects to see.
        position: u64,
        /// How many attempts the command loop spent, the successful one
        /// included. Greater than 1 means this request lost a race and
        /// re-decided.
        attempts: u32,
    },
    /// A rule refused it. `409`.
    Refused {
        /// The refusal, rendered with its values.
        why: String,
    },
    /// The retry bound was spent without the condition ever holding. `503`.
    Contended {
        /// How many attempts were made.
        attempts: u32,
    },
    /// Anything else. `500`.
    Failed {
        /// What went wrong.
        why: String,
    },
}

impl Reservation {
    /// The status code and reason phrase this outcome is answered with.
    #[must_use]
    pub fn status(&self) -> (u16, &'static str) {
        match self {
            Self::Taken { .. } => (201, "Created"),
            Self::Refused { .. } => (409, "Conflict"),
            Self::Contended { .. } => (503, "Service Unavailable"),
            Self::Failed { .. } => (500, "Internal Server Error"),
        }
    }

    /// The response body this outcome is answered with.
    #[must_use]
    pub fn body(&self) -> String {
        match self {
            Self::Taken { position, attempts } => {
                format!("reserved at position {position} in {attempts} attempt(s)\n")
            }
            Self::Refused { why } => format!("refused: {why}\n"),
            Self::Contended { attempts } => {
                format!("still contended after {attempts} attempt(s); try again\n")
            }
            Self::Failed { why } => format!("failed: {why}\n"),
        }
    }
}

/// Reserves one seat for one patron.
///
/// The boundary is a tuple of two models with two different scopes — the seat
/// and the show — so one append condition carries "nobody else took this seat"
/// and "the show has not sold out" at the same time. Neither invariant fits
/// inside the other's entity, and under the concurrency this example creates
/// they are contradicted by different requests.
pub async fn reserve(events: &SqliteEventStore, seat: &str, patron: &str) -> Reservation {
    let (Ok(show), Ok(seat), Ok(patron)) =
        (ShowId::new(SHOW), SeatId::new(seat), PatronId::new(patron))
    else {
        return Reservation::Failed {
            why: "a seat and a patron must be non-empty and free of control characters".to_owned(),
        };
    };

    let (Ok(one), Ok(all)) = (Seat::new(&seat), Show::new(&show)) else {
        return Reservation::Failed {
            why: "the boundary's scope could not be validated".to_owned(),
        };
    };
    let boundary = (one, all);

    let outcome = commit(
        events,
        boundary,
        retry_bound(),
        |(one, all): &(Seat, Show)| {
            let Some(capacity) = all.capacity else {
                return Err(Refusal::NotOpen {
                    show: show.as_str().to_owned(),
                });
            };
            if let Some(holder) = one.taken_by.clone() {
                return Err(Refusal::SeatTaken {
                    seat: seat.as_str().to_owned(),
                    patron: holder,
                });
            }
            if all.taken >= capacity {
                return Err(Refusal::Full {
                    show: show.as_str().to_owned(),
                    taken: all.taken,
                    capacity,
                });
            }
            Ok(vec![Ticketing::SeatReserved {
                show: show.clone(),
                seat: seat.clone(),
                patron: patron.clone(),
            }])
        },
    )
    .await;

    match outcome {
        Ok(committed) => Reservation::Taken {
            position: committed.position.get(),
            attempts: committed.attempts,
        },
        Err(CommandError::Refused(refusal)) => Reservation::Refused {
            why: refusal.to_string(),
        },
        Err(CommandError::Exhausted { attempts, .. }) => Reservation::Contended { attempts },
        Err(other) => Reservation::Failed {
            why: other.to_string(),
        },
    }
}

/// The retry bound, built from [`ATTEMPTS`].
///
/// A `const` conversion would be tidier and is not available: `NonZeroU32`'s
/// checked constructor is `const` but `Retry::attempts` takes the result, so
/// the fallible step has to happen somewhere. It happens here, once, against a
/// constant this crate owns — and falls back to one attempt rather than
/// panicking, because a zero in that constant should degrade to "try once"
/// and not take a web server down.
fn retry_bound() -> Retry {
    ATTEMPTS
        .try_into()
        .map_or_else(|_| Retry::once(), Retry::attempts)
}

/// The refusal used when the show's own identifier will not validate.
///
/// Unreachable: [`SHOW`] is a `&'static str` of seven ASCII letters. It is
/// spelled as a refusal rather than an `unwrap` because this crate is linked
/// into a server, and a panic path that no request can reach is still a panic
/// path in the binary.
fn unreachable_show() -> CommandError<SqliteEventStoreError, Refusal> {
    CommandError::Refused(Refusal::NotOpen {
        show: SHOW.to_owned(),
    })
}

// ---------------------------------------------------------------------------
// The read model, and the two processes' view of it
// ---------------------------------------------------------------------------

/// Who holds which seat, kept as a table rather than folded on every read.
#[derive(Debug)]
pub struct SeatHolders {
    /// Which read model this is, within the projection store.
    id: ProjectionId,
    /// Every event, so the tags constrain nothing.
    scope: Tags,
}

impl Default for SeatHolders {
    fn default() -> Self {
        Self::new()
    }
}

impl SeatHolders {
    /// The view over every seat.
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: ProjectionId::new(PROJECTION),
            scope: Tags::empty(),
        }
    }
}

impl Projection for SeatHolders {
    type Event = Ticketing;
    type Store = SqliteProjectionStore;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(
        &mut self,
        event: Ticketing,
        batch: &mut SqliteBatch,
    ) -> Result<(), SqliteProjectionStoreError> {
        match event {
            Ticketing::SeatReserved { seat, patron, .. } => batch.push(
                "INSERT INTO seat_holder (seat, patron) VALUES (?1, ?2) \
                 ON CONFLICT(seat) DO UPDATE SET patron = excluded.patron",
                [Value::Text(seat.into()), Value::Text(patron.into())],
            ),
            Ticketing::SeatReleased { seat, .. } => batch.push(
                "DELETE FROM seat_holder WHERE seat = ?1",
                [Value::Text(seat.into())],
            ),
            // Nominated and ignored. The derived query is the event set crossed
            // with the scope, and this view's scope is empty, so an opening
            // arrives here whether it is wanted or not — the checkpoint still
            // advances past it, which is what "the position considered" means.
            Ticketing::ShowOpened { .. } => {}
        }

        Ok(())
    }
}

/// Every held seat, as the application's own query sees it.
///
/// # Errors
///
/// Returns the driver's error if the statement cannot be prepared or run.
pub fn seat_rows(app: &Connection) -> rusqlite::Result<Vec<(String, String)>> {
    let mut statement = app.prepare("SELECT seat, patron FROM seat_holder ORDER BY seat")?;
    let rows = statement.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;
    rows.collect()
}

/// How far the read model has been brought, as a bare position.
///
/// `None` covers both "never run" and "rebuilding", which is right for the one
/// question the API asks of it: *are the rows I am about to serve at least as
/// new as the write this client just made?* A rebuilding view's rows are not
/// authoritative, so the honest answer for it is the same as for a view that
/// has never run.
///
/// # Errors
///
/// Returns the projection store's error if the checkpoint cannot be read.
pub async fn checkpoint_through(
    models: &SqliteProjectionStore,
) -> Result<Option<u64>, SqliteProjectionStoreError> {
    let id = ProjectionId::new(PROJECTION);

    Ok(match models.checkpoint(&id).await? {
        Checkpoint::Live { through } => Some(through.get()),
        // `NeverRun`, `Rebuilding`, and — `Checkpoint` being
        // `#[non_exhaustive]` — anything added upstream. Every one of them
        // answers this function's only question with "no": a rebuilding view's
        // rows are explicitly not authoritative, and a state this build has
        // never heard of must not be silently read as "caught up".
        _ => None,
    })
}

/// The position a client asked to read at, if it named one.
///
/// # Errors
///
/// Returns the raw value when it is not a position: zero does not exist —
/// `SequencePosition` wraps a `NonZeroU64` — and neither does anything that is
/// not a number.
pub fn requested_position(raw: &str) -> Result<SequencePosition, &str> {
    raw.parse().ok().and_then(SequencePosition::new).ok_or(raw)
}
