//! The ingest fixture over one Durable Object's storage.
//!
//! One fixture instance is one isolated log under its own [`TableNamespace`],
//! as [`WorkerdFixture`]'s is, and the store a
//! rule ingests into is built by the **published** constructor,
//! [`CloudflareIngestStore::namespaced_from_object_state`], over the object's
//! own `DurableObjectState`. No `cfg(test)` item of the adapter is reachable
//! from here: those are private to the adapter's own test build.

use core::fmt;
use core::future::Future;

use happenstance_cloudflare::worker::State;
use happenstance_cloudflare::worker::worker_sys::DurableObjectState;
use happenstance_cloudflare::{
    CloudflareEventStore, CloudflareEventStoreError, CloudflareIngestStore, SqlStorage, SqlValue,
    TableNamespace,
};
use happenstance_sync_testkit::{IngestFixture, Violation};
use happenstance_testkit::{Capability, Fixture};

use crate::fixture::WorkerdFixture;

/// The text the armed mid-group fault raises, so a failure names its cause.
pub const MID_GROUP_FAULT_TEXT: &str =
    "happenstance conformance fault: the ingest row is refused inside the group";

/// One isolated log inside the object, and the ingest store over it.
///
/// `Clone` aliases: the store is two `Rc` handles onto the object, and the
/// storage handle is one. The dispatcher hands the pre-flight fixture out by
/// clone when a later [`open`](Self::open) fails, so a rule never receives a
/// fixture whose store does not exist.
#[derive(Debug, Clone)]
pub struct WorkerdIngestFixture {
    /// Built once, by `open`, through the published constructor.
    store: CloudflareIngestStore,
    /// The fault trigger's way in, through the same database.
    sql: SqlStorage,
    /// The tables this log lives in.
    namespace: TableNamespace,
}

impl WorkerdIngestFixture {
    /// Migrates `namespace` and builds the ingest store over it.
    ///
    /// The schema is `CloudflareEventStore`'s, as the adapter's documentation
    /// requires before the first ingest; the store is then built with
    /// [`CloudflareIngestStore::namespaced_from_object_state`].
    ///
    /// # Errors
    ///
    /// [`OpenError`]: the migration's or the constructor's error, naming the
    /// namespace.
    pub fn open(
        state: &State,
        object_state: &DurableObjectState,
        namespace: TableNamespace,
    ) -> Result<Self, OpenError> {
        let sql = SqlStorage::from_state(state);
        // A clone of the handle, not of the database: `SqlStorage` aliases.
        CloudflareEventStore::namespaced(sql.clone(), &namespace)
            .migrate()
            .map_err(|source| OpenError::Migrate {
                namespace: namespace.as_str().to_owned(),
                source,
            })?;
        let store = CloudflareIngestStore::namespaced_from_object_state(object_state, &namespace)
            .map_err(|source| OpenError::Construct {
            namespace: namespace.as_str().to_owned(),
            source,
        })?;
        Ok(Self {
            store,
            sql,
            namespace,
        })
    }

    /// [`IngestFixture::arm_mid_group_fault`]'s body, as a `Result`.
    fn arm(&self, after: usize) -> Result<(), Violation> {
        for (statement, bindings) in mid_group_fault_statements(&self.namespace, after)? {
            self.sql.exec(&statement, &bindings).map_err(|error| {
                Violation::new(
                    "SY-30",
                    format!("the fixture could not arm its trigger: {error}"),
                )
            })?;
        }
        Ok(())
    }
}

/// Why [`WorkerdIngestFixture::open`] could not build a fixture.
#[derive(Debug)]
pub enum OpenError {
    /// The namespace's schema could not be applied.
    Migrate {
        /// The namespace.
        namespace: String,
        /// The adapter's error.
        source: CloudflareEventStoreError,
    },
    /// The ingest store's constructor refused the object's state.
    Construct {
        /// The namespace.
        namespace: String,
        /// The adapter's error.
        source: CloudflareEventStoreError,
    },
}

impl fmt::Display for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Migrate { namespace, source } => {
                write!(f, "migrating namespace {namespace}: {source}")
            }
            Self::Construct { namespace, source } => {
                write!(f, "building the ingest store over {namespace}: {source}")
            }
        }
    }
}

impl core::error::Error for OpenError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Migrate { source, .. } | Self::Construct { source, .. } => Some(source),
        }
    }
}

impl IngestFixture for WorkerdIngestFixture {
    type Store = CloudflareIngestStore;

    /// Supported, by a real SQLite trigger inside the object's own database,
    /// so the fault fires inside the real `transactionSync`.
    const MID_GROUP_FAULT: Capability = Capability::SUPPORTED;

    // Mirrored from the event-store fixture over the same adapter, never a
    // second literal: the ceilings are the adapter's, not the harness's.
    const MAX_EVENT_DATA_LEN: Option<usize> =
        <WorkerdFixture<'static> as Fixture>::MAX_EVENT_DATA_LEN;
    const MAX_TAGS_PER_EVENT: Option<usize> =
        <WorkerdFixture<'static> as Fixture>::MAX_TAGS_PER_EVENT;
    const MAX_METADATA_LEN: Option<usize> = <WorkerdFixture<'static> as Fixture>::MAX_METADATA_LEN;

    /// A handle onto the one store `open` built: `Clone` is two `Rc` bumps.
    fn connect(&self) -> impl Future<Output = Self::Store> {
        core::future::ready(self.store.clone())
    }

    /// A `BEFORE INSERT ON {ns}_event` trigger counting down a
    /// `{ns}_mid_group_fault` table, raising on the `after + 1`-th row.
    ///
    /// Inside the store's transaction the countdown rolls back with the call,
    /// so the trigger stays armed for the fixture's life, which the arm's
    /// contract allows: each arrangement takes a fresh fixture.
    fn arm_mid_group_fault(&self, after: usize) -> impl Future<Output = Result<(), Violation>> {
        core::future::ready(self.arm(after))
    }
}

/// The four statements that arm the mid-group fault on `namespace`'s own event
/// table, raising [`MID_GROUP_FAULT_TEXT`] on the `after + 1`-th insert.
///
/// The names are distinct from the event-store fixture's
/// `{ns}_mid_batch_fault`, so the two families' triggers never meet.
///
/// # Errors
///
/// A [`Violation`] of SY-30 if `after` does not fit the `i64` SQLite binds.
pub fn mid_group_fault_statements(
    namespace: &TableNamespace,
    after: usize,
) -> Result<[(String, Vec<SqlValue>); 4], Violation> {
    let countdown = i64::try_from(after)
        .map_err(|_| Violation::new("SY-30", format!("a cut at row {after} overflows an i64")))?;
    let ns = namespace.as_str();
    let counter = format!("{ns}_mid_group_fault");
    Ok([
        (
            format!("CREATE TABLE IF NOT EXISTS {counter} (remaining INTEGER NOT NULL)"),
            vec![],
        ),
        (format!("DELETE FROM {counter}"), vec![]),
        (
            format!("INSERT INTO {counter} (remaining) VALUES (?)"),
            vec![SqlValue::Integer(countdown)],
        ),
        (
            format!(
                "CREATE TRIGGER IF NOT EXISTS {ns}_mid_group_fault_fires \
                 BEFORE INSERT ON {ns}_event BEGIN \
                 UPDATE {counter} SET remaining = remaining - 1; \
                 SELECT RAISE(ABORT, '{MID_GROUP_FAULT_TEXT}') \
                 FROM {counter} WHERE remaining < 0; \
                 END"
            ),
            vec![],
        ),
    ])
}

#[cfg(test)]
mod tests;
