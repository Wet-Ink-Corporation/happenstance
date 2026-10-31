use happenstance_cloudflare::TableNamespace;
use happenstance_sync_testkit::IngestFixture;
use happenstance_testkit::Fixture;

use super::{MID_GROUP_FAULT_TEXT, WorkerdIngestFixture, mid_group_fault_statements};
use crate::fixture::WorkerdFixture;

fn namespace() -> TableNamespace {
    TableNamespace::new("f3").expect("a short lowercase name is a valid namespace")
}

/// The plausible wrong copy is the adapter's own `cfg(test)` fixture, whose
/// trigger sits on the unprefixed `event` table: in a namespaced log that
/// trigger would never fire, and SY-30's rule would pass over nothing.
#[test]
fn the_mid_group_trigger_is_on_the_namespaced_event_table() {
    let statements = mid_group_fault_statements(&namespace(), 2).expect("2 fits an i64");
    let [create, clear, seed, trigger] = &statements;

    assert!(create.0.contains("f3_mid_group_fault "), "{}", create.0);
    assert!(clear.0.ends_with("FROM f3_mid_group_fault"), "{}", clear.0);
    assert!(seed.0.contains("INTO f3_mid_group_fault "), "{}", seed.0);
    assert_eq!(
        seed.1,
        vec![happenstance_cloudflare::SqlValue::Integer(2)],
        "the countdown is the cut"
    );
    assert!(
        trigger
            .0
            .contains("TRIGGER IF NOT EXISTS f3_mid_group_fault_fires"),
        "{}",
        trigger.0
    );
    assert!(
        trigger.0.contains("BEFORE INSERT ON f3_event BEGIN"),
        "{}",
        trigger.0
    );
    assert!(trigger.0.contains(MID_GROUP_FAULT_TEXT), "{}", trigger.0);
    for (statement, _) in &statements {
        assert!(
            !statement.contains(" ON event ") && !statement.contains(" mid_group_fault "),
            "an unprefixed table: {statement}"
        );
        assert!(
            !statement.contains("mid_batch_fault"),
            "the event-store fixture's names: {statement}"
        );
    }
}

#[test]
fn a_cut_past_i64_is_a_sy_30_violation_not_a_panic() {
    let refused = mid_group_fault_statements(&namespace(), usize::MAX);

    assert_eq!(
        refused.map_err(|violation| violation.clause()).err(),
        Some("SY-30")
    );
}

#[test]
fn the_ingest_fixture_mirrors_the_event_store_fixtures_ceilings() {
    assert_eq!(
        <WorkerdIngestFixture as IngestFixture>::MAX_EVENT_DATA_LEN,
        <WorkerdFixture<'static> as Fixture>::MAX_EVENT_DATA_LEN
    );
    assert_eq!(
        <WorkerdIngestFixture as IngestFixture>::MAX_TAGS_PER_EVENT,
        <WorkerdFixture<'static> as Fixture>::MAX_TAGS_PER_EVENT
    );
    assert_eq!(
        <WorkerdIngestFixture as IngestFixture>::MAX_METADATA_LEN,
        <WorkerdFixture<'static> as Fixture>::MAX_METADATA_LEN
    );
    // Every ceiling declared, so SY-18's rule cannot skip (invariant 3).
    assert_ne!(
        <WorkerdIngestFixture as IngestFixture>::MAX_EVENT_DATA_LEN,
        None
    );
    assert_ne!(
        <WorkerdIngestFixture as IngestFixture>::MAX_TAGS_PER_EVENT,
        None
    );
    assert_ne!(
        <WorkerdIngestFixture as IngestFixture>::MAX_METADATA_LEN,
        None
    );
    assert_eq!(
        <WorkerdIngestFixture as IngestFixture>::MID_GROUP_FAULT.reason(),
        None
    );
}
