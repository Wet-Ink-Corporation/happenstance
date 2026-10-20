//! The ingest conformance family, mounted on the Durable Object.
//!
//! Executed on `wasm32-unknown-unknown` under the gate's Node shim, through
//! `xtask/src/proof.rs`'s `--lib` row for this crate, whose transcribed names
//! include every rule below. The mount is exhaustive by construction: the macro
//! expands `for_each_ingest_store_rule!`.

use core::future::Future;

use happenstance_sync_testkit::{IngestFixture, Violation};
use happenstance_testkit::Capability;

use crate::event_store::ingest::{CloudflareIngestStore, model};
use crate::event_store::{Ceilings, CloudflareEventStore};
use crate::host::DurableObjectHost;
use crate::sql_storage::{SqlStorage, SqlValue};

/// The text the armed fault raises with, which only the trigger carries.
const MID_GROUP_FAULT_TEXT: &str = "the fixture armed a mid-group fault";

/// One fixture instance is one fresh Durable Object, migrated once, with the
/// model's `transactionSync` installed; each `connect` is one handle onto it.
struct CloudflareIngestFixture {
    /// The object, whose storage `transactionSync` lives on.
    host: DurableObjectHost,
    /// A handle onto its storage, cloned (an `Rc` bump) for each connection.
    sql: SqlStorage,
}

impl CloudflareIngestFixture {
    /// A fresh object.
    fn new() -> Self {
        let host = DurableObjectHost::new();
        model::install(&host).expect("the model installs on the shim");
        let sql = host.storage();
        CloudflareEventStore::new(sql.clone())
            .migrate()
            .expect("the schema applies");
        Self { host, sql }
    }
}

impl IngestFixture for CloudflareIngestFixture {
    type Store = CloudflareIngestStore;

    /// Supported, by a real SQLite trigger inside the store, for CF-39's
    /// reason (`host.rs`'s `arm_throw` documentation): a fault armed on the
    /// shim would evaporate when the runtime under the adapter is swapped for
    /// `workerd`.
    const MID_GROUP_FAULT: Capability = Capability::SUPPORTED;

    /// Mirrored from the ceilings the store declares, never a literal.
    const MAX_EVENT_DATA_LEN: Option<usize> = Some(Ceilings::DECLARED.event_data_len);
    /// Mirrored from the ceilings the store declares.
    const MAX_TAGS_PER_EVENT: Option<usize> = Some(Ceilings::DECLARED.tags_per_event);
    /// Mirrored from the ceilings the store declares.
    const MAX_METADATA_LEN: Option<usize> = Some(Ceilings::DECLARED.metadata_len);

    /// Through the public constructor, as a consumer builds one.
    fn connect(&self) -> impl Future<Output = Self::Store> {
        core::future::ready(
            CloudflareIngestStore::from_object_state(&self.host.object_state())
                .expect("a broken test environment: the model's transactionSync is installed"),
        )
    }

    /// A `BEFORE INSERT ON event` trigger counting down a `mid_group_fault`
    /// table and raising on the `after + 1`-th row inserted after it is armed.
    ///
    /// Every row of the armed group is new, so every row is an `INSERT`, a
    /// foreign row's staged insert and the compensation's alike. `RAISE(ABORT)`
    /// backs out the one statement; the store's transaction is what has to
    /// take the rows before it. Inside that transaction the countdown rolls
    /// back too, so the trigger stays armed for the fixture's life, which the
    /// arm's contract allows: each arrangement takes a fresh fixture. The
    /// names are distinct from the event-store fixture's `mid_batch_fault`.
    fn arm_mid_group_fault(&self, after: usize) -> impl Future<Output = Result<(), Violation>> {
        core::future::ready(self.arm(after))
    }
}

impl CloudflareIngestFixture {
    /// [`IngestFixture::arm_mid_group_fault`]'s body, as a `Result`.
    fn arm(&self, after: usize) -> Result<(), Violation> {
        let unarmable = |error| {
            Violation::new(
                "SY-30",
                format!("the fixture could not arm its trigger: {error}"),
            )
        };
        let countdown = i64::try_from(after).map_err(|_| {
            Violation::new("SY-30", format!("a cut at row {after} overflows an i64"))
        })?;
        for (statement, bindings) in [
            (
                "CREATE TABLE IF NOT EXISTS mid_group_fault (remaining INTEGER NOT NULL)"
                    .to_owned(),
                vec![],
            ),
            ("DELETE FROM mid_group_fault".to_owned(), vec![]),
            (
                "INSERT INTO mid_group_fault (remaining) VALUES (?)".to_owned(),
                vec![SqlValue::Integer(countdown)],
            ),
            (
                format!(
                    "CREATE TRIGGER IF NOT EXISTS mid_group_fault_fires \
                     BEFORE INSERT ON event BEGIN \
                     UPDATE mid_group_fault SET remaining = remaining - 1; \
                     SELECT RAISE(ABORT, '{MID_GROUP_FAULT_TEXT}') \
                     FROM mid_group_fault WHERE remaining < 0; \
                     END"
                ),
                vec![],
            ),
        ] {
            self.sql.exec(&statement, &bindings).map_err(unarmable)?;
        }
        Ok(())
    }
}

happenstance_sync_testkit::ingest_store_conformance!(
    mod_name = ingest_conformance,
    emit = happenstance_sync_testkit::emit_ingest_wasm,
    fixture = super::CloudflareIngestFixture::new()
);
