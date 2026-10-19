//! `happenstance-sync-testkit`'s ingest family, against this adapter.
//!
//! Mounted here rather than in `tests/` because the impl is `cfg(test)`, and a
//! `cfg(test)` impl is invisible to an integration test. The tests beside this
//! pin the port's own behaviour; this is the bar every adapter that implements
//! the port clears.
//!
//! Expected outcome: VT-5, SY-5 and SY-12 run; SY-30 runs, faulted by this
//! fixture's trigger at every cut of its group; SY-18 runs against the two
//! per-event ceilings this store imposes; SY-38 passes its refusals and
//! reports its hidden-claim half as skipped, because a refused call here has
//! consumed sequence values, so the compensation the rule then writes lands
//! past the identity it claimed. On this store the hidden case reduces to the
//! landed one, which `claims.rs` verifies directly.

use happenstance_sync_testkit::{IngestFixture, Violation};
use happenstance_testkit::Capability;
use tokio::sync::OnceCell;

use super::endpoint::{LiveTransport, fresh_schema};
use crate::config::{NeonConfig, quote_ident};
use crate::event_store::NeonEventStore;
use crate::transport::{IsolationLevel, SqlRequest, SqlStatement, SqlTransport};

/// One fresh schema per fixture, and a handle onto it per `connect`.
///
/// The schema is minted and migrated on first `connect`, on the rule's own
/// runtime. A handle is a new `NeonEventStore` over a clone of the one
/// transport, so every handle a rule holds sends to one schema.
#[derive(Debug)]
struct NeonIngestFixture {
    schema: OnceCell<(NeonConfig, LiveTransport)>,
}

impl NeonIngestFixture {
    fn new() -> Self {
        Self {
            schema: OnceCell::new(),
        }
    }

    /// This fixture's schema and transport, minted on first use.
    async fn schema(&self) -> &(NeonConfig, LiveTransport) {
        self.schema.get_or_init(fresh_schema).await
    }
}

impl IngestFixture for NeonIngestFixture {
    type Store = NeonEventStore<LiveTransport>;

    /// Supported, by an `AFTER INSERT` row trigger in this fixture's schema
    /// that counts the rows the armed transaction inserts and raises on the
    /// `after + 1`-th. Row-level `AFTER` triggers fire only for rows that
    /// were inserted, so a row `ON CONFLICT` skipped is not counted, and they
    /// fire in insertion order across both of the call's inserts, its foreign
    /// rows first and its compensation after. The raise aborts the request's
    /// one transaction, which is the unit an ingest commits.
    const MID_GROUP_FAULT: Capability = Capability::SUPPORTED;

    /// Mirrored from the constants `exceeded` holds every foreign and
    /// compensation event to before the request is built. No metadata
    /// ceiling: this store states none, and `exceeded` does not bound it.
    const MAX_EVENT_DATA_LEN: Option<usize> =
        Some(NeonEventStore::<LiveTransport>::MAX_EVENT_DATA_LEN);
    const MAX_TAGS_PER_EVENT: Option<usize> =
        Some(NeonEventStore::<LiveTransport>::MAX_TAGS_PER_EVENT);

    async fn connect(&self) -> Self::Store {
        let (config, transport) = self.schema().await;
        // A cheap handle onto the one client and ledger, and a copy of a few
        // names, which the store keeps.
        NeonEventStore::new(transport.clone(), config.clone())
    }

    /// Installs the trigger `MID_GROUP_FAULT` describes. The counter is a
    /// transaction-local setting, named apart from the event-store fixture's,
    /// so each ingest counts from zero; the trigger stays for the fixture's
    /// life, which the arm's contract allows. Twinned with
    /// `tests/support/mod.rs`'s `arm_mid_batch_fault`, which is in another
    /// crate.
    async fn arm_mid_group_fault(&self, after: usize) -> Result<(), Violation> {
        // Only an integer is formatted into the statement, so there is
        // nothing to inject.
        let threshold = after
            .checked_add(1)
            .and_then(|row| i64::try_from(row).ok())
            .ok_or_else(|| {
                Violation::new("SY-30", format!("a cut at row {after} overflows an i64"))
            })?;
        let (config, transport) = self.schema().await;
        let schema = quote_ident(&config.schema);
        let event = config.qualified_event();
        let statements = vec![
            SqlStatement::new(format!(
                "CREATE OR REPLACE FUNCTION {schema}.mid_group_fault() RETURNS trigger AS $fn$ \
                 DECLARE seen bigint; BEGIN \
                 seen := coalesce(nullif(current_setting('hs.ingest_fault_seen', true), ''), \
                 '0')::bigint + 1; \
                 PERFORM set_config('hs.ingest_fault_seen', seen::text, true); \
                 IF seen = {threshold} THEN \
                 RAISE EXCEPTION 'the fixture armed a mid-group fault'; END IF; \
                 RETURN NEW; END $fn$ LANGUAGE plpgsql"
            )),
            SqlStatement::new(format!("DROP TRIGGER IF EXISTS mid_group_fault ON {event}")),
            SqlStatement::new(format!(
                "CREATE TRIGGER mid_group_fault AFTER INSERT ON {event} \
                 FOR EACH ROW EXECUTE FUNCTION {schema}.mid_group_fault()"
            )),
        ];
        let armed = transport
            .round_trip(SqlRequest::batch(statements, IsolationLevel::ReadCommitted))
            .await
            .map_err(|error| {
                Violation::new(
                    "SY-30",
                    format!("the Neon fixture could not reach the endpoint to arm: {error}"),
                )
            })?;
        if armed.is_success() {
            Ok(())
        } else {
            Err(Violation::new(
                "SY-30",
                format!(
                    "the Neon fixture could not arm its trigger (HTTP {}): {}",
                    armed.status,
                    String::from_utf8_lossy(&armed.body)
                        .chars()
                        .take(400)
                        .collect::<String>()
                ),
            ))
        }
    }
}

/// `happenstance_sync_testkit::emit_ingest_tokio`, with this crate's
/// `#[ignore]` reason added, as every live test here carries.
macro_rules! emit_ignored_ingest_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            #[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
            async fn $name() -> ::core::result::Result<(), ::happenstance_sync_testkit::Violation> {
                ::happenstance_sync_testkit::ingest_rules::$name(__ingest_fixture)
                    .await?
                    .report(::core::stringify!($name));
                Ok(())
            }
        )*
    };
}
pub(crate) use emit_ignored_ingest_tokio;

happenstance_sync_testkit::ingest_store_conformance!(
    mod_name = ingest_conformance,
    emit = super::emit_ignored_ingest_tokio,
    fixture = super::NeonIngestFixture::new()
);
