//! Where the tables live, and how much of a response the adapter will accept.

use crate::transport::{IsolationLevel, MAX_RESPONSE_BYTES};

/// Configuration shared by both Neon-backed stores.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct NeonConfig {
    /// The table events are written to.
    pub event_table: Box<str>,
    /// The table projection checkpoints are written to.
    pub checkpoint_table: Box<str>,
    /// The isolation level for the batched, non-interactive transaction.
    ///
    /// Defaults to [`IsolationLevel::Serializable`], which is *not* Postgres'
    /// default. At `ReadCommitted` a conditional insert can miss a conflict a
    /// concurrent transaction committed after this one took its snapshot, and
    /// with no interactive transaction there is no second look to catch it. The
    /// price is SQLSTATE `40001` aborts under contention, which the caller sees
    /// as [`NeonSqlError::is_serialization_failure`](crate::NeonSqlError::is_serialization_failure).
    pub isolation: IsolationLevel,
    /// The response-size ceiling, at most [`MAX_RESPONSE_BYTES`].
    pub max_response_bytes: usize,
}

impl Default for NeonConfig {
    fn default() -> Self {
        Self {
            event_table: "event".into(),
            checkpoint_table: "projection_checkpoint".into(),
            isolation: IsolationLevel::Serializable,
            max_response_bytes: MAX_RESPONSE_BYTES,
        }
    }
}

impl NeonConfig {
    /// Overrides the event table name.
    #[must_use]
    pub fn with_event_table(mut self, table: impl Into<Box<str>>) -> Self {
        self.event_table = table.into();
        self
    }

    /// Overrides the checkpoint table name.
    #[must_use]
    pub fn with_checkpoint_table(mut self, table: impl Into<Box<str>>) -> Self {
        self.checkpoint_table = table.into();
        self
    }

    /// Overrides the isolation level for batched requests.
    #[must_use]
    pub const fn with_isolation(mut self, isolation: IsolationLevel) -> Self {
        self.isolation = isolation;
        self
    }

    /// Lowers the response ceiling below [`MAX_RESPONSE_BYTES`].
    ///
    /// Raising it above has no effect on the endpoint, which enforces its own
    /// cap; the value is clamped so the adapter's error and the endpoint's
    /// behaviour cannot disagree.
    #[must_use]
    pub const fn with_max_response_bytes(mut self, bytes: usize) -> Self {
        self.max_response_bytes = if bytes < MAX_RESPONSE_BYTES {
            bytes
        } else {
            MAX_RESPONSE_BYTES
        };
        self
    }
}
