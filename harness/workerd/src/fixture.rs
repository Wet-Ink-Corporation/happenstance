//! The conformance fixture over one Durable Object's storage.
//!
//! One fixture instance is one isolated log, kept under its own
//! [`TableNamespace`] in the object's single database. The shim harness gets
//! isolation by opening a second database; a Durable Object has exactly one, and
//! [`CloudflareEventStore::namespaced`] is how a consumer keeps two logs in one.

use core::cell::{Cell, RefCell};
use core::future::Future;

use happenstance_cloudflare::{CloudflareEventStore, SqlStorage, SqlValue, TableNamespace};
use happenstance_testkit::{Capability, Fixture};
use worker::State;

/// The text the armed mid-batch fault raises, so a failure names its cause.
pub const MID_BATCH_FAULT_TEXT: &str =
    "happenstance conformance fault: the event row is refused inside the write path";

/// What went wrong *in the harness* while a rule ran, as opposed to what the rule
/// asserted.
///
/// A fixture method cannot fail through its signature — `connect` and
/// `arm_mid_batch_fault` return plain futures — and this crate may not panic, so
/// a failure to migrate or to arm a fault is recorded here and reported by the
/// dispatcher after the rule returns. A rule that passed over a fixture that
/// silently failed to arm its fault would otherwise be a green result about
/// nothing.
///
/// A `Cell` rather than a `RefCell`: the record is only ever swapped whole, so
/// there is no borrow to conflict and no failure path to swallow.
#[derive(Default)]
pub struct HarnessFaults(Cell<Vec<String>>);

impl core::fmt::Debug for HarnessFaults {
    // Hand-written because `Cell<T>: Debug` needs `T: Copy`; reading the record
    // to print it would also empty it, so the count is what is shown.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let faults = self.0.take();
        let count = faults.len();
        self.0.set(faults);
        f.debug_struct("HarnessFaults")
            .field("count", &count)
            .finish()
    }
}

impl HarnessFaults {
    /// Records one fault.
    pub fn record(&self, fault: String) {
        let mut faults = self.0.take();
        faults.push(fault);
        self.0.set(faults);
    }

    /// Every fault recorded so far, leaving the record empty.
    #[must_use]
    pub fn take(&self) -> Vec<String> {
        self.0.take()
    }
}

/// One isolated log inside the object, and the handles a rule opens onto it.
#[derive(Debug)]
pub struct WorkerdFixture<'a> {
    state: &'a State,
    sql: RefCell<SqlStorage>,
    namespace: TableNamespace,
    faults: &'a HarnessFaults,
}

impl<'a> WorkerdFixture<'a> {
    /// A fresh log under `namespace`, migrated before any handle is handed out.
    #[must_use]
    pub fn open(state: &'a State, namespace: TableNamespace, faults: &'a HarnessFaults) -> Self {
        let sql = SqlStorage::from_state(state);
        if let Err(err) = CloudflareEventStore::namespaced(sql.clone(), &namespace).migrate() {
            faults.record(format!("migrating namespace {}: {err}", namespace.as_str()));
        }
        Self {
            state,
            sql: RefCell::new(sql),
            namespace,
            faults,
        }
    }

    /// Runs one statement, recording rather than panicking on failure.
    fn exec(&self, sql: &SqlStorage, statement: &str, bindings: &[SqlValue]) {
        if let Err(err) = sql.exec(statement, bindings) {
            self.faults
                .record(format!("arming the mid-batch fault: {statement}: {err}"));
        }
    }

    fn current_sql(&self) -> Option<SqlStorage> {
        // A clone of the handle, not of the database: `SqlStorage` aliases.
        if let Ok(sql) = self.sql.try_borrow() {
            Some(sql.clone())
        } else {
            self.faults
                .record("the fixture's storage handle was borrowed".to_owned());
            None
        }
    }
}

impl Fixture for WorkerdFixture<'_> {
    type Store = CloudflareEventStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    // A fresh handle off the object's state, as the shim fixture's is a fresh
    // handle off its host. Neither is an eviction: workerd's
    // `evictDurableObject` is reachable only from the test runner, between
    // requests, and a rule runs inside one request.
    const REOPEN: Capability = Capability::SUPPORTED;
    const MID_BATCH_FAULT: Capability = Capability::SUPPORTED;
    const READ_FAULT: Capability = Capability::declined(
        "by scope, not by incapacity. This adapter's read does not fetch a page \
         at a time across an await, so there is no fetch between two pages to \
         fail; the injection a paged read would need is the one MID_BATCH_FAULT \
         already uses here, a trigger installed through the object's own SQL, \
         and declaring the capability before that read exists would report a \
         green result about a read nothing had faulted",
    );
    // The shim fixture's declarations, unchanged: the same adapter, and the
    // ceilings are the adapter's rather than the host's.
    const MAX_EVENT_DATA_LEN: Option<usize> = Some(1024 * 1024);
    const MAX_TAGS_PER_EVENT: Option<usize> = Some(1024);
    const MAX_EVENTS_PER_BATCH: Option<usize> = Some(1024);

    fn connect(&self) -> impl Future<Output = Self::Store> {
        let sql = self
            .current_sql()
            .unwrap_or_else(|| SqlStorage::from_state(self.state));
        core::future::ready(CloudflareEventStore::namespaced(sql, &self.namespace))
    }

    fn reopen(&self) -> impl Future<Output = ()> {
        match self.sql.try_borrow_mut() {
            Ok(mut sql) => *sql = SqlStorage::from_state(self.state),
            Err(_) => self
                .faults
                .record("the fixture's storage handle was borrowed at reopen".to_owned()),
        }
        core::future::ready(())
    }

    fn arm_mid_batch_fault(&self, after: usize) -> impl Future<Output = ()> {
        if let Some(sql) = self.current_sql() {
            let ns = self.namespace.as_str();
            let counter = format!("{ns}_mid_batch_fault");
            self.exec(
                &sql,
                &format!("CREATE TABLE IF NOT EXISTS {counter} (remaining INTEGER NOT NULL)"),
                &[],
            );
            self.exec(&sql, &format!("DELETE FROM {counter}"), &[]);
            self.exec(
                &sql,
                &format!("INSERT INTO {counter} (remaining) VALUES (?)"),
                &[SqlValue::Integer(i64::try_from(after).unwrap_or(i64::MAX))],
            );
            self.exec(
                &sql,
                &format!(
                    "CREATE TRIGGER IF NOT EXISTS {ns}_mid_batch_fault_fires \
                     BEFORE INSERT ON {ns}_event BEGIN \
                     UPDATE {counter} SET remaining = remaining - 1; \
                     SELECT RAISE(ABORT, '{MID_BATCH_FAULT_TEXT}') \
                     FROM {counter} WHERE remaining < 0; \
                     END"
                ),
                &[],
            );
        }
        core::future::ready(())
    }
}
