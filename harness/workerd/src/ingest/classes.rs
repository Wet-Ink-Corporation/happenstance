//! F's D7 classes, revisited on a real Durable Object.
//!
//! `impl Classify for CloudflareEventStoreError` left two judgement calls:
//! `SqlError::StorageLimitExceeded → Stop` and `SqlError::Thrown(_) → Stop`,
//! with no message parsed, "until a workerd run shows which throws recur". A
//! class answers one question, **does the identical call fail again?**, so the
//! revisit is a measurement of exactly that: provoke each reachable failure
//! through the production store, retry the identical call on the same object,
//! and [`judge`] the pair.
//!
//! The census records; it changes nothing in the adapter. A [`Verdict`] other
//! than [`Holds`](Verdict::Holds) or an [`Unreachable`](Verdict::Unreachable)
//! storage case is evidence for a follow-up, not a fix made here (D-E3-1).

use core::fmt;
use core::str::FromStr;

use happenstance_cloudflare::happenstance_sync::{
    Classify, FailureClass, IngestError, IngestGroup, IngestStore, Ingested, ReplicatedEvent,
};
use happenstance_cloudflare::worker::State;
use happenstance_cloudflare::worker::worker_sys::DurableObjectState;
use happenstance_cloudflare::{
    CloudflareEventStoreError, CloudflareIngestStore, SqlError, SqlStorage, TableNamespace,
};
use happenstance_sync_testkit::IngestFixture as _;

use crate::ingest::ceiling::{Shape, foreign_batch};
use crate::ingest::fixture::WorkerdIngestFixture;

/// The namespace every census case keeps its log under. Each case runs on its
/// own object, so one name serves them all.
const NAMESPACE: &str = "census";

/// How many maximal events the storage case sends: 8 MiB of data against an
/// object capped one page above what it already holds.
const STORAGE_FULL_EVENTS: usize = 8;

/// SQLite's default page size, in bytes.
const DEFAULT_PAGE_SIZE: u64 = 4096;

/// One reachable ingest failure, staged on its own object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    /// A `RAISE(ABORT)` trigger on the event table, armed to fire on the first
    /// row: `Thrown → Stop` for a throw raised inside `transactionSync`.
    TriggerAbort,
    /// A store over a namespace never migrated: `Thrown → Stop` for
    /// `no such table`.
    MissingSchema,
    /// An object capped by `PRAGMA max_page_count` one page above what it
    /// holds: `StorageLimitExceeded → Stop`, and whether the adapter's markers
    /// match the runtime's own text.
    StorageFull,
}

impl Case {
    /// Every case, in the order a report prints them.
    pub const ALL: [Self; 3] = [Self::TriggerAbort, Self::MissingSchema, Self::StorageFull];

    /// The name a route and a report use.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::TriggerAbort => "trigger-abort",
            Self::MissingSchema => "missing-schema",
            Self::StorageFull => "storage-full",
        }
    }

    /// The arm, by [`Observed::variant`]'s name, the case exists to provoke.
    /// A recurring failure of any other arm says nothing about the D7 arm
    /// under test, so [`judge`] calls it unreachable rather than holding.
    #[must_use]
    pub const fn expected_variant(self) -> &'static str {
        match self {
            Self::TriggerAbort | Self::MissingSchema => THROWN,
            Self::StorageFull => "Sql(StorageLimitExceeded)",
        }
    }
}

/// [`Observed::variant`]'s name for `SqlError::Thrown`, the arm two cases
/// provoke and the one a missed storage marker arrives as.
const THROWN: &str = "Sql(Thrown)";

impl FromStr for Case {
    type Err = UnknownCase;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|case| case.name() == name)
            .ok_or_else(|| UnknownCase(name.to_owned()))
    }
}

/// No census case has this name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownCase(pub String);

impl fmt::Display for UnknownCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "no ingest class case {}; known: {}",
            self.0,
            Case::ALL.map(Case::name).join(", ")
        )
    }
}

impl core::error::Error for UnknownCase {}

/// One refused call, as the adapter reported it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Observed {
    /// The class the adapter gave it.
    pub class: FailureClass,
    /// Which arm of the error it was, by a fixed name (`"Sql(Thrown)"`, …).
    pub variant: &'static str,
    /// The error, verbatim.
    pub error: String,
}

impl Observed {
    /// What `error` is, read the way a runner reads it.
    #[must_use]
    pub fn of(error: &IngestError<CloudflareEventStoreError>) -> Self {
        Self {
            class: error.class(),
            variant: variant(error),
            error: error.to_string(),
        }
    }
}

/// A fixed name for each arm. The three wildcards are forced, not chosen:
/// `IngestError`, `CloudflareEventStoreError` and `SqlError` are each
/// `#[non_exhaustive]` and another crate's, so a match from here cannot be
/// exhaustive. Each names what it caught, so an arm added later shows up in
/// a report as `other` rather than as something it is not.
fn variant(error: &IngestError<CloudflareEventStoreError>) -> &'static str {
    match error {
        IngestError::ExceedsStoreLimit { .. } => "ExceedsStoreLimit",
        IngestError::UnheldOwnIdentity { .. } => "UnheldOwnIdentity",
        IngestError::Busy(_) => "Busy",
        IngestError::Store(CloudflareEventStoreError::Sql(sql)) => match sql {
            SqlError::Thrown(_) => THROWN,
            SqlError::StorageLimitExceeded => "Sql(StorageLimitExceeded)",
            SqlError::AlreadyBorrowed => "Sql(AlreadyBorrowed)",
            SqlError::CursorInvalidated => "Sql(CursorInvalidated)",
            _ => "Sql(other)",
        },
        IngestError::Store(_) => "Store(other)",
        _ => "other",
    }
}

/// What the pair of identical calls says about the class D7 assigned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// The class and the retry agree.
    Holds,
    /// The retry contradicts the class: a `Stop` whose identical retry
    /// landed, or a `Transient` whose retry failed. The first failure.
    Falsifies(String),
    /// A full object reported as `Thrown`, not `StorageLimitExceeded`: the
    /// class is the same, and the markers missed the runtime's text, quoted.
    MarkerMissed(String),
    /// The case could not be staged, or provoked no failure; why.
    Unreachable(String),
}

impl Verdict {
    /// `verdict=<Name>`, then the detail, if any.
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::Holds => "verdict=Holds".to_owned(),
            Self::Falsifies(detail) => format!("verdict=Falsifies {detail}"),
            Self::MarkerMissed(detail) => format!("verdict=MarkerMissed {detail}"),
            Self::Unreachable(detail) => format!("verdict=Unreachable {detail}"),
        }
    }
}

/// Judges `case` from its two identical calls' outcomes, `first` and `retry`.
///
/// First, the failure must be the one the case exists to provoke
/// ([`Case::expected_variant`]); a storage case may also see `Thrown`, which
/// is the missed marker. Any other arm is [`Verdict::Unreachable`]: the case
/// did not reach the D7 arm it tests. Then a class is a claim about the retry,
/// so the retry decides: a `Transient` holds only if its retry lands, and every
/// other class only if its retry fails too. A storage case whose failure
/// arrived as `Thrown` is a missed marker once the class itself has held.
#[must_use]
pub fn judge(
    case: Case,
    first: &Result<Ingested, Observed>,
    retry: &Result<Ingested, Observed>,
) -> Verdict {
    let Err(observed) = first else {
        return Verdict::Unreachable("the case provoked no failure".to_owned());
    };
    let expected = case.expected_variant();
    let marker_missed = case == Case::StorageFull && observed.variant == THROWN;
    if observed.variant != expected && !marker_missed {
        return Verdict::Unreachable(format!("provoked {}, not {expected}", observed.variant));
    }
    let retry_landed = retry.is_ok();
    let class_holds = if observed.class == FailureClass::Transient {
        retry_landed
    } else {
        !retry_landed
    };
    if !class_holds {
        return Verdict::Falsifies(format!(
            "{:?} ({}): {}",
            observed.class, observed.variant, observed.error
        ));
    }
    if marker_missed {
        // The verdict owns its quote; `first` stays the caller's (H-08).
        return Verdict::MarkerMissed(observed.error.clone());
    }
    Verdict::Holds
}

/// Stages `case` on this object, makes the identical call twice, and answers
/// the report: the verdict's line, then the case and both outcomes.
pub async fn provoke(case: Case, state: &State, object_state: &DurableObjectState) -> String {
    match stage(case, state, object_state).await {
        Ok((store, batch)) => {
            let first = ingest(&store, &batch).await;
            let retry = ingest(&store, &batch).await;
            format!(
                "{}\ncase={}\nfirst={}\nretry={}",
                judge(case, &first, &retry).line(),
                case.name(),
                outcome_line(&first),
                outcome_line(&retry),
            )
        }
        Err(refusal) => format!(
            "{}\ncase={}",
            Verdict::Unreachable(refusal).line(),
            case.name()
        ),
    }
}

/// One call, with its failure read as a runner reads it.
async fn ingest(
    store: &CloudflareIngestStore,
    batch: &[ReplicatedEvent],
) -> Result<Ingested, Observed> {
    store
        .ingest(&[IngestGroup::new(batch, &[])])
        .await
        .map_err(|error| Observed::of(&error))
}

/// `landed appended=<n> skipped=<m>`, or the failure's class, arm and text.
fn outcome_line(outcome: &Result<Ingested, Observed>) -> String {
    match outcome {
        Ok(ingested) => format!(
            "landed appended={} skipped={}",
            ingested.appended, ingested.skipped
        ),
        Err(observed) => format!(
            "class={:?} variant={} error={}",
            observed.class, observed.variant, observed.error
        ),
    }
}

/// The store and the batch for `case`, or the runtime's refusal to stage it,
/// verbatim.
async fn stage(
    case: Case,
    state: &State,
    object_state: &DurableObjectState,
) -> Result<(CloudflareIngestStore, Vec<ReplicatedEvent>), String> {
    let namespace = TableNamespace::new(NAMESPACE).map_err(|error| error.to_string())?;
    let small = || foreign_batch(Shape::Minimal, 2).map_err(|error| error.to_string());
    match case {
        Case::TriggerAbort => {
            let fixture = WorkerdIngestFixture::open(state, object_state, namespace)
                .map_err(|error| error.to_string())?;
            fixture
                .arm_mid_group_fault(0)
                .await
                .map_err(|violation| violation.to_string())?;
            Ok((fixture.connect().await, small()?))
        }
        Case::MissingSchema => {
            let store =
                CloudflareIngestStore::namespaced_from_object_state(object_state, &namespace)
                    .map_err(|error| format!("building the store: {error}"))?;
            Ok((store, small()?))
        }
        Case::StorageFull => {
            let fixture = WorkerdIngestFixture::open(state, object_state, namespace)
                .map_err(|error| error.to_string())?;
            cap_pages(&SqlStorage::from_state(state))?;
            let batch = foreign_batch(Shape::MaxData, STORAGE_FULL_EVENTS)
                .map_err(|error| error.to_string())?;
            Ok((fixture.connect().await, batch))
        }
    }
}

/// `PRAGMA max_page_count` set one page above what the database holds, so the
/// next page it needs is refused. Its refusal is the answer, verbatim: the
/// object's authoriser decides which PRAGMAs run.
///
/// The page count is derived from the size the binding reports, at SQLite's
/// default page size, because local `workerd` 1.20260815.1 refuses
/// `PRAGMA page_count` itself (`not authorized: SQLITE_AUTH`, measured).
fn cap_pages(sql: &SqlStorage) -> Result<(), String> {
    let cap = sql
        .database_size()
        .div_ceil(DEFAULT_PAGE_SIZE)
        .saturating_add(1);
    let statement = format!("PRAGMA max_page_count = {cap}");
    let mut cursor = sql
        .exec(&statement, &[])
        .map_err(|error| format!("{statement}: {error}"))?;
    // Drained, so a refusal at step time is seen too.
    while let Some(row) = cursor.next_row() {
        row.map_err(|error| format!("{statement}: {error}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
