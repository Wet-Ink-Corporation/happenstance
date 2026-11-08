//! `NeonEventStore::remint_identity` and the append that has to see it,
//! against a live Neon endpoint (#104, ADR-0086, VT-6).
//!
//! A Neon branch, a `pg_restore` or a point-in-time recovery carries the
//! identity row together with an older position sequence. Without a re-mint
//! the copy hands out `(StoreId, SequencePosition)` pairs already given to
//! events its peers hold. The restore is simulated in place: the tail row is
//! deleted and the sequence is set back to the value it held before that row
//! was written.
//!
//! The oracles read raw SQL, never the adapter's decoders, and never wait on
//! the visibility frontier. What needs no endpoint, the statement text and
//! the decoding of each answer, is `src/event_store/tests/remint.rs`.
//!
//! # How this target is gated
//!
//! As `neon_conformance.rs` is: every case is `#[ignore]`d with a reason, so
//! the default gate compiles and lints it, and CI's `live-neon` job runs it
//! with `-- --ignored` when the endpoint secret is configured.
//!
//! ```console
//! NEON_CONNECTION=… cargo test -p happenstance-neon --all-features --test remint -- --ignored --test-threads=1
//! ```

#![cfg(not(target_arch = "wasm32"))]

mod support;

use happenstance_core::{
    AppendCondition, AppendError, Event, EventStore, Query, SequencePosition, StoreId,
};
use happenstance_neon::event_store::NeonEventStore;
use happenstance_neon::wire::{ResponseBody, ResultSet};
use happenstance_neon::{NeonConfig, NeonError, SqlRequest, SqlStatement, SqlTransport};
use happenstance_testkit::Fixture;
use support::NeonFixture;
use support::transport::HyperTransport;

type Store = NeonEventStore<HyperTransport>;

fn event(event_type: &str) -> [Event; 1] {
    [Event::new(event_type.to_owned(), &b"payload"[..]).expect("a valid event type")]
}

/// One unconditional append, answering the position the store assigned.
async fn append(store: &Store, event_type: &str) -> SequencePosition {
    EventStore::append(store, &event(event_type), None)
        .await
        .expect("an unconditional append to a migrated store lands")
}

/// Runs `sql` with `params` through `store`'s transport and answers the parsed
/// body, panicking on anything else: these are the test's own statements, so
/// a failure is the environment's.
async fn query(store: &Store, sql: String, params: Vec<String>) -> ResponseBody {
    let params = params.into_iter().map(serde_json::Value::String).collect();
    let response = store
        .transport()
        .round_trip(SqlRequest::single(SqlStatement::with_params(sql, params)))
        .await
        .expect("the endpoint answers");
    assert!(
        response.is_success(),
        "the test's own statement failed (HTTP {}): {}",
        response.status,
        String::from_utf8_lossy(&response.body)
    );
    ResponseBody::parse(&response.body).expect("the answer parses")
}

/// The one result set a single statement answers with.
fn only(body: &ResponseBody) -> &ResultSet {
    let [set] = body.result_sets() else {
        panic!("one statement answers one result set");
    };
    set
}

fn position_text(position: SequencePosition) -> String {
    position.get().to_string()
}

fn decode_identity(base64: &str) -> StoreId {
    use base64::Engine as _;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(base64.replace(['\n', '\r'], ""))
        .expect("the endpoint renders base64");
    StoreId::from_bytes(bytes.try_into().expect("an identity is sixteen bytes"))
}

/// The `(origin_store, origin_position)` the row at `position` carries.
async fn stamped(store: &Store, position: SequencePosition) -> (StoreId, String) {
    let sql = format!(
        "SELECT encode(origin_store, 'base64') AS s, origin_position::text AS p \
         FROM {} WHERE position = $1::bigint",
        store.config().qualified_event()
    );
    let body = query(store, sql, vec![position_text(position)]).await;
    let set = only(&body);
    (
        decode_identity(set.text(0, "s").expect("the row is stamped")),
        set.text(0, "p")
            .expect("the row has an origin position")
            .to_owned(),
    )
}

/// Every identity row, as base64.
async fn persisted(store: &Store) -> Vec<String> {
    let sql = format!(
        "SELECT encode(v, 'base64') AS v FROM {} WHERE k = 'store_id'",
        store.config().qualified_meta()
    );
    let body = query(store, sql, Vec::new()).await;
    let set = only(&body);
    (0..set.rows.len())
        .filter_map(|row| set.text(row, "v").map(ToOwned::to_owned))
        .collect()
}

async fn rows(store: &Store) -> String {
    let sql = format!(
        "SELECT count(*)::text AS n FROM {}",
        store.config().qualified_event()
    );
    let body = query(store, sql, Vec::new()).await;
    only(&body)
        .text(0, "n")
        .expect("a count answers")
        .to_owned()
}

async fn delete_identity(store: &Store) {
    let sql = format!(
        "DELETE FROM {} WHERE k = 'store_id'",
        store.config().qualified_meta()
    );
    query(store, sql, Vec::new()).await;
}

async fn sequence_value(store: &Store) -> String {
    let sql = format!(
        "SELECT last_value::text AS v FROM {}",
        store.config().qualified_sequence()
    );
    let body = query(store, sql, Vec::new()).await;
    only(&body)
        .text(0, "v")
        .expect("the sequence reads")
        .to_owned()
}

/// Restores the store to the moment before `tail` was written: the row goes,
/// and the sequence goes back to `saved`, as a copy taken then would leave both.
async fn rewind(store: &Store, tail: SequencePosition, saved: String) {
    let config: &NeonConfig = store.config();
    let delete = format!(
        "DELETE FROM {} WHERE position = $1::bigint",
        config.qualified_event()
    );
    query(store, delete, vec![position_text(tail)]).await;
    let setval = format!(
        "SELECT setval({}, $1::bigint)::text AS v",
        config.sequence_literal()
    );
    query(store, setval, vec![saved]).await;
}

/// A re-mint through one handle changes what an already open handle, and a
/// clone of it, stamp on their next append.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn remint_changes_the_id_stamped_by_an_already_open_handle() {
    let fixture = NeonFixture::new();
    let open = fixture.connect().await;
    // A clone shares whatever the handle holds, which is the point.
    let clone = open.clone();
    let first = append(&open, "BeforeRemint").await;
    let (old, _) = stamped(&open, first).await;

    let other = fixture.connect().await;
    let minted = other
        .remint_identity()
        .await
        .expect("a migrated store re-mints");

    assert_ne!(minted, old, "a re-mint mints a different identity");
    let persisted = persisted(&open).await;
    assert_eq!(persisted.len(), 1, "one identity row: {persisted:?}");
    for handle in [&open, &clone] {
        let after = append(handle, "AfterRemint").await;
        let (store, origin) = stamped(&open, after).await;
        assert_eq!(
            store, minted,
            "an open handle stamps the re-minted identity"
        );
        assert_eq!(
            origin,
            position_text(after),
            "the origin half is the row's own position"
        );
    }
    assert_eq!(
        stamped(&open, first).await.0,
        old,
        "an earlier event keeps its origin"
    );
}

/// Both append shapes refuse with `MissingIdentity`, and write nothing, when
/// the identity row is gone.
///
/// Rejects the stamp alone, a scalar subquery that writes a NULL origin.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn append_refuses_without_an_identity_row() {
    let fixture = NeonFixture::new();
    let store = fixture.connect().await;
    append(&store, "Warm").await;
    delete_identity(&store).await;

    let condition = AppendCondition::new(Query::all());
    for condition in [None, Some(&condition)] {
        let refused = EventStore::append(&store, &event("Refused"), condition)
            .await
            .expect_err("an append with no identity row cannot stamp an EventId");
        assert!(
            matches!(refused, AppendError::Store(NeonError::MissingIdentity)),
            "{refused:?}"
        );
    }
    assert_eq!(rows(&store).await, "1", "a refused append writes nothing");
}

/// A re-mint of a schema with no identity row refuses, and creates none.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn remint_identity_refuses_without_an_identity_row() {
    let fixture = NeonFixture::new();
    let store = fixture.connect().await;
    delete_identity(&store).await;

    let refused = store
        .remint_identity()
        .await
        .expect_err("there is no row to re-mint");
    assert!(matches!(refused, NeonError::MissingIdentity), "{refused:?}");
    assert!(persisted(&store).await.is_empty(), "no row was created");
}

/// The negative control: a restored store that is not re-minted re-issues the
/// identity of an event it lost, to a different event.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn without_a_remint_a_restored_store_reissues_an_event_id() {
    let fixture = NeonFixture::new();
    let store = fixture.connect().await;
    append(&store, "Kept").await;
    let saved = sequence_value(&store).await;
    let lost = append(&store, "Lost").await;
    let lost_id = stamped(&store, lost).await;

    rewind(&store, lost, saved).await;
    let reissued = append(&store, "Different").await;

    assert_eq!(
        stamped(&store, reissued).await,
        lost_id,
        "an un-reminted restore re-issues the lost event's identity to a different event"
    );
}

/// After a restore and a re-mint, the next event's identity is not one
/// already issued.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn a_reminted_restored_store_does_not_reissue_an_event_id() {
    let fixture = NeonFixture::new();
    let store = fixture.connect().await;
    let kept = append(&store, "Kept").await;
    let saved = sequence_value(&store).await;
    let lost = append(&store, "Lost").await;
    let issued = [stamped(&store, kept).await, stamped(&store, lost).await];

    rewind(&store, lost, saved).await;
    let minted = store
        .remint_identity()
        .await
        .expect("the restored store re-mints");
    let next = append(&store, "Different").await;
    let next_id = stamped(&store, next).await;

    assert!(
        !issued.contains(&next_id),
        "{next_id:?} was already issued: {issued:?}"
    );
    assert_eq!(next_id.0, minted, "the next event carries the new identity");
}
