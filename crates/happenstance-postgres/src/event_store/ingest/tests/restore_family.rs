//! The Postgres sync peer restored (#133, VT-6): the restore family over
//! `StoreSyncPeer<PostgresEventStore, PostgresEventStore>`, and its negative
//! control.
//!
//! # How a restore is simulated
//!
//! In place, as `tests/remint.rs` does it: a backup records the position
//! sequence's state and the identity row; a restore deletes every row written
//! since, sets the sequence back and puts the row back. That is what restoring
//! a dump or a point-in-time recovery taken at the backup does to this
//! schema's state, which is the `event` table, `event_position_seq` and
//! `store_meta` and nothing else. The schema is the fixture's own, from
//! `server::fresh_pool`, so no other test sees it.
//!
//! Then the documented procedure (ADR-0086, the crate root's "After a
//! restore"): `remint_identity`. Writers are stopped by construction, since the
//! rule drops its peer before it restores, and the restart the procedure
//! needed before #104 is no longer one of its steps.
//!
//! # The negative control
//!
//! [`AfterRestore::NothingMore`] runs the same sequence with the re-mint left
//! out, which is the mistake the procedure exists to prevent, and the rule
//! must report it. Without that control the rule would be decorative here.
//!
//! In the library target beside `peer_family.rs`, for the reason that file
//! gives: CI's `live-postgres` job already runs every `#[ignore]`d test of
//! this target.

use std::cell::Cell;

use happenstance_core::{Event, EventStore};
use happenstance_sync::StoreSyncPeer;
use happenstance_sync_testkit::{RestoreFixture, Violation, restore_rules};
use sqlx::PgPool;
use tokio::sync::OnceCell;

use super::server;
use crate::event_store::PostgresEventStore;

const VT_6: &str = "VT-6";

/// What the fixture does after it has put the storage back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AfterRestore {
    /// The documented procedure: re-mint the identity.
    Remint,
    /// Nothing: the negative control.
    NothingMore,
}

/// The state a restore to the backup brings back.
#[derive(Debug, Clone, Copy)]
struct Backup {
    /// `event_position_seq`'s `last_value`.
    last_value: i64,
    /// Its `is_called`: whether `last_value` was handed out.
    is_called: bool,
    /// The `store_meta` identity row's value.
    identity: [u8; 16],
}

impl Backup {
    /// The highest position the backup can hold a row at.
    const fn highest_kept(self) -> i64 {
        if self.is_called { self.last_value } else { 0 }
    }
}

/// One fresh schema per fixture, restorable in place.
#[derive(Debug)]
struct PostgresRestoreFixture {
    pool: OnceCell<PgPool>,
    backup: Cell<Option<Backup>>,
    after_restore: AfterRestore,
}

impl PostgresRestoreFixture {
    fn new() -> Self {
        Self::with(AfterRestore::Remint)
    }

    fn with(after_restore: AfterRestore) -> Self {
        Self {
            pool: OnceCell::new(),
            backup: Cell::new(None),
            after_restore,
        }
    }

    async fn pool(&self) -> &PgPool {
        self.pool.get_or_init(server::fresh_pool).await
    }

    /// A handle onto the fixture's schema; a refcounted clone of the pool.
    async fn store(&self) -> PostgresEventStore {
        PostgresEventStore::new(self.pool().await.clone())
    }
}

/// A server error during an arrangement, reported under the rule's clause.
fn arranging(step: &str, error: &sqlx::Error) -> Violation {
    Violation::new(VT_6, format!("the fixture could not {step}: {error}"))
}

impl RestoreFixture for PostgresRestoreFixture {
    type Peer = StoreSyncPeer<PostgresEventStore, PostgresEventStore>;

    async fn connect(&self) -> Self::Peer {
        StoreSyncPeer::new(self.store().await, self.store().await)
    }

    async fn append_local(&self, events: &[Event]) -> Result<(), Violation> {
        EventStore::append(&self.store().await, events, None)
            .await
            .map(drop)
            .map_err(|error| Violation::new(VT_6, format!("a local append was refused: {error}")))
    }

    async fn back_up(&self) -> Result<(), Violation> {
        let pool = self.pool().await;
        let (last_value, is_called): (i64, bool) =
            sqlx::query_as("SELECT last_value, is_called FROM event_position_seq")
                .fetch_one(pool)
                .await
                .map_err(|error| arranging("read the position sequence", &error))?;
        let identity: Vec<u8> = sqlx::query_scalar("SELECT v FROM store_meta WHERE k = 'store_id'")
            .fetch_one(pool)
            .await
            .map_err(|error| arranging("read the identity row", &error))?;
        let identity = <[u8; 16]>::try_from(identity).map_err(|row| {
            Violation::new(
                VT_6,
                format!("the identity row holds {} bytes, not 16", row.len()),
            )
        })?;
        self.backup.set(Some(Backup {
            last_value,
            is_called,
            identity,
        }));
        Ok(())
    }

    async fn restore(&self) -> Result<(), Violation> {
        let backup = self
            .backup
            .take()
            .ok_or_else(|| Violation::new(VT_6, "the fixture restored before any backup"))?;
        let pool = self.pool().await;
        let mut restore = pool
            .begin()
            .await
            .map_err(|error| arranging("open the restore", &error))?;
        sqlx::query("DELETE FROM event WHERE position > $1")
            .bind(backup.highest_kept())
            .execute(&mut *restore)
            .await
            .map_err(|error| arranging("delete the rows written since the backup", &error))?;
        sqlx::query("SELECT setval('event_position_seq', $1, $2)")
            .bind(backup.last_value)
            .bind(backup.is_called)
            .execute(&mut *restore)
            .await
            .map_err(|error| arranging("set the position sequence back", &error))?;
        sqlx::query("UPDATE store_meta SET v = $1 WHERE k = 'store_id'")
            .bind(backup.identity.as_slice())
            .execute(&mut *restore)
            .await
            .map_err(|error| arranging("put the identity row back", &error))?;
        restore
            .commit()
            .await
            .map_err(|error| arranging("commit the restore", &error))?;
        match self.after_restore {
            AfterRestore::Remint => self
                .store()
                .await
                .remint_identity()
                .await
                .map(drop)
                .map_err(|error| {
                    Violation::new(VT_6, format!("the documented re-mint failed: {error}"))
                }),
            AfterRestore::NothingMore => Ok(()),
        }
    }
}

/// `happenstance_sync_testkit::emit_restore_tokio`, with this crate's
/// `#[ignore]` reason added, as every live test here carries.
macro_rules! emit_ignored_restore_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            #[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
            async fn $name() -> ::core::result::Result<(), ::happenstance_sync_testkit::Violation> {
                ::happenstance_sync_testkit::restore_rules::$name(__restore_fixture)
                    .await?
                    .report(::core::stringify!($name));
                Ok(())
            }
        )*
    };
}
pub(crate) use emit_ignored_restore_tokio;

happenstance_sync_testkit::restore_conformance!(
    mod_name = restore_conformance,
    emit = super::emit_ignored_restore_tokio,
    fixture = super::PostgresRestoreFixture::new()
);

/// The negative control: the same restore without the documented re-mint
/// reissues the identity the lost event was given, and the rule says so.
///
/// Rejects a rule that passes this store whatever the procedure does, which a
/// rule judging only the restore's shape would.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_restore_without_the_re_mint_reissues_an_identity() {
    let verdict = restore_rules::restored_peer_does_not_reissue_identities(async || {
        PostgresRestoreFixture::with(AfterRestore::NothingMore)
    })
    .await;

    let violation = verdict.expect_err("a restore left un-reminted must reissue an identity");
    assert_eq!(violation.clause(), "VT-6", "{violation}");
    assert!(
        violation
            .message()
            .contains("reissued an identity it had already given away"),
        "the control failed for another reason: {violation}"
    );
}
