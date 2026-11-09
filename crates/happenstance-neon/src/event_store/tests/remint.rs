//! `NeonEventStore::remint_identity`, `migration::remint_statement`, and the
//! append's refusal when the identity row is absent (#104, ADR-0086), held
//! without an endpoint.
//!
//! What only an endpoint can answer, that an open handle stamps the re-minted
//! identity and that a restored, re-minted store issues no pair twice, is
//! `tests/remint.rs`, which CI's `live-neon` job runs.

use core::cell::RefCell;

use happenstance_core::{AppendCondition, AppendError, Event, EventStore, Query, StoreId};

use super::{NeonConfig, NeonError, Scripted};
use crate::event_store::NeonEventStore;
use crate::migration::remint_statement;
use crate::transport::{HttpResponse, NullTransport, SqlRequest, SqlTransport};

/// The identity the scripted re-mint answer carries.
const MINTED: StoreId = StoreId::from_bytes([0xAB; 16]);

/// [`MINTED`] as the endpoint renders `encode(v, 'base64')`, in the answer to
/// the re-mint statement.
const REMINTED: &[u8] = br#"{"fields":[],"command":"UPDATE","rowCount":1,"rows":[{"store_id":"q6urq6urq6urq6urq6urqw=="}]}"#;

fn config() -> NeonConfig {
    NeonConfig::default().with_schema("hs_1")
}

fn event() -> [Event; 1] {
    [Event::new("T", b"x".to_vec()).expect("a valid event type")]
}

/// A transport that keeps every request it is handed and answers each with
/// one body.
struct Recording {
    body: &'static [u8],
    sent: RefCell<Vec<SqlRequest>>,
}

impl Recording {
    fn answering(body: &'static [u8]) -> Self {
        Self {
            body,
            sent: RefCell::new(Vec::new()),
        }
    }
}

impl SqlTransport for &Recording {
    type Error = std::io::Error;

    fn round_trip(
        &self,
        request: SqlRequest,
    ) -> impl Future<Output = Result<HttpResponse, Self::Error>> {
        self.sent.borrow_mut().push(request);
        core::future::ready(Ok(HttpResponse::new(200, self.body)))
    }

    fn reads_settled(&self) -> impl Future<Output = ()> {
        core::future::ready(())
    }
}

/// The re-mint is migration 1's insert turned into an update of the configured
/// meta row, and nothing else: branch tooling runs this text as it stands.
///
/// Rejects an upsert, which would mint into a schema migration 1 never ran on,
/// and an unqualified table, which the `/sql` proxy resolves against whatever
/// backend it lands on.
#[test]
fn remint_statement_updates_the_configured_meta_row() {
    let statement = remint_statement(&config());

    assert_eq!(
        statement.query,
        r#"UPDATE "hs_1"."store_meta" SET v = uuid_send(gen_random_uuid()) WHERE k = 'store_id' RETURNING encode(v, 'base64') AS store_id"#
    );
    assert!(
        statement.params.is_empty(),
        "nothing to bind: {statement:?}"
    );
}

/// The crate root and the README print the default configuration's re-mint
/// for an operator to paste, and both copies are this text.
///
/// Rejects the statement and its documentation drifting apart, which would
/// hand an operator a re-mint the crate no longer runs.
#[test]
fn the_documented_remint_is_the_default_configurations() {
    let documented = r#"UPDATE "public"."store_meta" SET v = uuid_send(gen_random_uuid()) WHERE k = 'store_id' RETURNING encode(v, 'base64') AS store_id"#;

    assert_eq!(remint_statement(&NeonConfig::default()).query, documented);
    for (page, text) in [
        ("src/lib.rs", include_str!("../../lib.rs")),
        ("README.md", include_str!("../../../README.md")),
    ] {
        assert!(text.contains(documented), "{page} prints another re-mint");
    }
}

/// `remint_identity` sends exactly that statement, alone and not read-only, and
/// answers the identity the endpoint returned.
#[tokio::test]
async fn remint_identity_sends_the_statement_and_answers_the_minted_identity() {
    let transport = Recording::answering(REMINTED);
    let store = NeonEventStore::new(&transport, config());

    let minted = store
        .remint_identity()
        .await
        .expect("a body carrying a sixteen-byte identity decodes");

    assert_eq!(minted, MINTED);
    let sent = transport.sent.borrow();
    let [request] = sent.as_slice() else {
        panic!("one round trip, got {}", sent.len());
    };
    let [statement] = request.statements.as_slice() else {
        panic!("one statement, got {:?}", request.statements);
    };
    assert_eq!(statement.query, remint_statement(&config()).query);
    assert!(
        request.headers().is_empty(),
        "a write, so not marked read-only: {:?}",
        request.headers()
    );
}

/// An update that matched no row is a store with no identity row, and says so.
///
/// Rejects decoding the empty answer as a shape error, which would send an
/// operator to this crate's decoder rather than to migration 1.
#[tokio::test]
async fn remint_identity_without_an_identity_row_is_missing_identity() {
    let transport = Scripted::answering(
        200,
        br#"{"fields":[],"command":"UPDATE","rowCount":0,"rows":[]}"#,
    );
    let store = NeonEventStore::new(&transport, config());

    let refused = store
        .remint_identity()
        .await
        .expect_err("there was no row to re-mint");
    assert!(matches!(refused, NeonError::MissingIdentity), "{refused:?}");
}

/// A row that came back without the `store_id` column, or with it NULL, is a
/// shape error naming the column, not a missing identity row: the update
/// matched a row, so the store has one.
///
/// Rejects folding every unreadable answer into `MissingIdentity`, which would
/// send an operator to migration 1 for a decoder fault.
#[tokio::test]
async fn remint_identity_with_a_row_lacking_the_column_is_missing_column() {
    const ANSWERS: [&[u8]; 2] = [
        br#"{"fields":[],"command":"UPDATE","rowCount":1,"rows":[{"v":"AQIDBAUGBwgJCgsMDQ4PEA=="}]}"#,
        br#"{"fields":[],"command":"UPDATE","rowCount":1,"rows":[{"store_id":null}]}"#,
    ];
    for body in ANSWERS {
        let transport = Scripted::answering(200, body);
        let store = NeonEventStore::new(&transport, config());

        let refused = store
            .remint_identity()
            .await
            .expect_err("the answer carries no identity to decode");
        assert!(
            matches!(refused, NeonError::MissingColumn { column: "store_id" }),
            "{refused:?}"
        );
    }
}

/// Both append shapes insert only where the identity row exists, so a store
/// without one inserts nothing rather than rows with a NULL origin, and the
/// conditional probe reports whether the row was there.
///
/// Rejects the stamp alone, a scalar subquery that is NULL when the row is
/// absent and that the nullable `origin_store` column accepts.
#[test]
fn both_append_shapes_insert_only_where_the_identity_row_exists() {
    let store = NeonEventStore::new(NullTransport::new(), config());
    let condition = AppendCondition::new(Query::all());
    let guard = r#"EXISTS (SELECT 1 FROM "hs_1"."store_meta" WHERE k = 'store_id')"#;

    let unconditional = store.insert_request(&event());
    let conditional = store.conditional_append_request(&event(), Some(&condition));
    for insert in [&unconditional.statements[0], &conditional.statements[1]] {
        assert!(
            insert.query.contains(&format!("WHERE {guard}")),
            "the insert is not guarded on the identity row: {}",
            insert.query
        );
    }
    assert!(
        conditional.statements[0]
            .query
            .contains(&format!("({guard})::text AS identified")),
        "the probe reports whether the row exists: {}",
        conditional.statements[0].query
    );
}

/// An unconditional append that inserted nothing found no identity row, and
/// refuses as that.
///
/// Rejects answering `MissingColumn { column: "position" }`, which names this
/// crate's decoder as the culprit.
#[tokio::test]
async fn an_unconditional_append_that_inserts_nothing_is_missing_identity() {
    let transport = Scripted::answering(
        200,
        br#"{"fields":[],"command":"INSERT","rowCount":0,"rows":[]}"#,
    );
    let store = NeonEventStore::new(&transport, config());

    let refused = EventStore::append(&store, &event(), None)
        .await
        .expect_err("nothing was inserted");
    assert!(
        matches!(refused, AppendError::Store(NeonError::MissingIdentity)),
        "{refused:?}"
    );
}

/// A conditional append whose probe found no conflict and no identity row, and
/// whose insert wrote nothing, refuses as a missing identity.
#[tokio::test]
async fn a_conditional_append_without_an_identity_row_is_missing_identity() {
    let transport = Scripted::answering(
        200,
        br#"{"results":[{"fields":[],"rows":[{"conflict":null,"identified":"false"}],"command":"SELECT","rowCount":1,"rowAsArray":false},{"fields":[],"rows":[],"command":"INSERT","rowCount":0,"rowAsArray":false}]}"#,
    );
    let store = NeonEventStore::new(&transport, config());
    let condition = AppendCondition::new(Query::all());

    let refused = EventStore::append(&store, &event(), Some(&condition))
        .await
        .expect_err("nothing was inserted");
    assert!(
        matches!(refused, AppendError::Store(NeonError::MissingIdentity)),
        "{refused:?}"
    );
}

/// A conditional append that wrote nothing while its probe saw the identity row
/// and no conflict is neither outcome, and is not blamed on the row either.
///
/// Rejects folding every empty insert into `MissingIdentity`: a guard that
/// silently matched nothing would then send an operator to re-run a migration
/// that is already there.
#[tokio::test]
async fn an_empty_insert_beside_an_identity_row_stays_a_shape_error() {
    let transport = Scripted::answering(
        200,
        br#"{"results":[{"fields":[],"rows":[{"conflict":null,"identified":"true"}],"command":"SELECT","rowCount":1,"rowAsArray":false},{"fields":[],"rows":[],"command":"INSERT","rowCount":0,"rowAsArray":false}]}"#,
    );
    let store = NeonEventStore::new(&transport, config());
    let condition = AppendCondition::new(Query::all());

    let refused = EventStore::append(&store, &event(), Some(&condition))
        .await
        .expect_err("nothing was inserted");
    assert!(
        matches!(
            refused,
            AppendError::Store(NeonError::MissingColumn { column: "position" })
        ),
        "{refused:?}"
    );
}
