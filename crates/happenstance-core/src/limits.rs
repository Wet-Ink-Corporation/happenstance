//! Capacity floors every store must clear, and the name a store uses when it
//! refuses beyond one.
//!
//! These are **floors, never ceilings** (VT-21 – VT-24). A single
//! `MAX_EVENT_DATA_LEN` would be either a straitjacket on Postgres — which will
//! hold a 340 KB seat map without noticing — or a lie on a KV-backed peer that
//! cannot hold it at all, and freezing one at 0.1 would fix the number for
//! good. A floor instead tells an application what it may write and still
//! expect to replicate, and gives the sync layer something to compare a peer's
//! declared limit against *before* it starts pushing.
//!
//! A store MAY accept more than any floor here, MUST document what it actually
//! accepts, and MUST refuse beyond that with
//! [`AppendError::ExceedsStoreLimit`](crate::AppendError::ExceedsStoreLimit)
//! rather than by truncating.

/// Smallest `data` payload every store must accept, in bytes.
///
/// The tightest target in the plan is the Cloudflare Durable Object, whose
/// legacy KV-backed form caps a value at 128 KiB — this clears it with room for
/// the envelope.
pub const MIN_SUPPORTED_EVENT_DATA_LEN: usize = 65_536;

/// Smallest number of tags on one event every store must accept.
///
/// The richest event in the six worked scenarios carries eight.
pub const MIN_SUPPORTED_TAGS_PER_EVENT: usize = 64;

/// Smallest number of query items every store must be able to evaluate.
///
/// The largest decision model in the six worked scenarios uses four.
pub const MIN_SUPPORTED_QUERY_ITEMS: usize = 128;

/// Smallest number of events in one append every store must accept.
///
/// Worth doing the multiplication once, because two floors meet here and an
/// adapter inherits the product rather than either factor. A multi-row tag
/// insert at these floors binds `128 × 64 × 3 = 24,576` parameters, against
/// SQLite's `SQLITE_MAX_VARIABLE_NUMBER` of 32,766 — leaving 8,190 for the
/// event rows themselves. The pair fits, with 23% headroom rather than the 41%
/// a hundred-event batch would have implied, so an adapter that binds one extra
/// parameter per tag is within a factor of 1.3 of the ceiling.
pub const MIN_SUPPORTED_EVENTS_PER_BATCH: usize = 128;

/// Which capacity limit an append exceeded.
///
/// Three variants and not four: a query-item refusal is not an append outcome,
/// so a `QueryItems` variant would name a refusal no `append` could ever
/// produce. `MIN_SUPPORTED_QUERY_ITEMS` is a floor on evaluation, and a store
/// that will not evaluate a large query fails it through its own error rather
/// than through this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StoreLimit {
    /// One event's `data` payload was larger than the store accepts.
    EventDataLen,
    /// One event carried more tags than the store accepts.
    TagsPerEvent,
    /// The batch held more events than the store accepts in one append.
    EventsPerBatch,
}

impl StoreLimit {
    /// The floor every store must clear for this limit.
    ///
    /// A caller that has hit a store's actual limit can compare against this to
    /// tell "this store is stricter than the contract allows" — a conformance
    /// bug — from "this payload was always going to be too big to replicate".
    #[must_use]
    pub const fn guaranteed_minimum(self) -> usize {
        match self {
            Self::EventDataLen => MIN_SUPPORTED_EVENT_DATA_LEN,
            Self::TagsPerEvent => MIN_SUPPORTED_TAGS_PER_EVENT,
            Self::EventsPerBatch => MIN_SUPPORTED_EVENTS_PER_BATCH,
        }
    }
}

impl core::fmt::Display for StoreLimit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Self::EventDataLen => "event data length",
            Self::TagsPerEvent => "tags per event",
            Self::EventsPerBatch => "events per batch",
        };
        f.write_str(name)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        MIN_SUPPORTED_EVENT_DATA_LEN, MIN_SUPPORTED_EVENTS_PER_BATCH, MIN_SUPPORTED_TAGS_PER_EVENT,
        StoreLimit,
    };

    #[test]
    fn every_limit_names_its_floor() {
        assert_eq!(
            StoreLimit::EventDataLen.guaranteed_minimum(),
            MIN_SUPPORTED_EVENT_DATA_LEN
        );
        assert_eq!(
            StoreLimit::TagsPerEvent.guaranteed_minimum(),
            MIN_SUPPORTED_TAGS_PER_EVENT
        );
        assert_eq!(
            StoreLimit::EventsPerBatch.guaranteed_minimum(),
            MIN_SUPPORTED_EVENTS_PER_BATCH
        );
    }

    #[test]
    fn the_two_floors_fit_inside_sqlites_parameter_ceiling() {
        // The arithmetic in `MIN_SUPPORTED_EVENTS_PER_BATCH`'s docs, asserted so
        // that raising either floor fails here rather than at an adapter's
        // write path in phase 8.
        const SQLITE_MAX_VARIABLE_NUMBER: usize = 32_766;
        let tag_parameters = MIN_SUPPORTED_EVENTS_PER_BATCH * MIN_SUPPORTED_TAGS_PER_EVENT * 3;
        assert_eq!(tag_parameters, 24_576);
        assert!(tag_parameters < SQLITE_MAX_VARIABLE_NUMBER);
    }
}
