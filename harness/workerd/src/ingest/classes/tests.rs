use happenstance_cloudflare::happenstance_sync::{FailureClass, Ingested};

use super::{Case, Observed, Verdict, judge};

fn thrown(text: &str) -> Result<Ingested, Observed> {
    Err(Observed {
        class: FailureClass::Stop,
        variant: "Sql(Thrown)",
        error: text.to_owned(),
    })
}

/// What the judge sees of a call that landed.
fn landed() -> Ingested {
    Ingested::default()
}

/// The lazy judge reads only the first failure's class, which D7 assigned,
/// and calls it `Holds`. The class is a claim about the retry, so only the
/// retry can falsify it.
#[test]
fn a_stop_whose_identical_retry_succeeds_falsifies_d7() {
    let verdict = judge(Case::TriggerAbort, &thrown("Error: busy"), &Ok(landed()));

    assert_eq!(
        verdict,
        Verdict::Falsifies("Stop (Sql(Thrown)): Error: busy".to_owned())
    );
}

#[test]
fn a_stop_that_recurs_holds() {
    let first = thrown("Error: no such table: census_event");

    assert_eq!(
        judge(Case::MissingSchema, &first, &first.clone()),
        Verdict::Holds
    );
}

#[test]
fn a_full_object_reported_as_thrown_is_a_missed_marker() {
    let first = thrown("Error: database or disk is fuller than ever");

    assert_eq!(
        judge(Case::StorageFull, &first, &first.clone()),
        Verdict::MarkerMissed("Error: database or disk is fuller than ever".to_owned())
    );
    // The same throw in a case about something else is not a marker's business.
    assert_eq!(
        judge(Case::TriggerAbort, &first, &first.clone()),
        Verdict::Holds
    );
}

#[test]
fn a_case_that_provokes_nothing_is_unreachable_not_holds() {
    assert_eq!(
        judge(Case::StorageFull, &Ok(landed()), &Ok(landed())),
        Verdict::Unreachable("the case provoked no failure".to_owned())
    );
}

#[test]
fn a_transient_holds_only_if_its_retry_lands() {
    let busy = Err(Observed {
        class: FailureClass::Transient,
        variant: "Sql(AlreadyBorrowed)",
        error: "borrowed".to_owned(),
    });

    assert_eq!(
        judge(Case::TriggerAbort, &busy, &Ok(landed())),
        Verdict::Holds
    );
    assert_eq!(
        judge(Case::TriggerAbort, &busy, &busy.clone()),
        Verdict::Falsifies("Transient (Sql(AlreadyBorrowed)): borrowed".to_owned())
    );
}

#[test]
fn every_case_parses_from_its_route_name() {
    for case in Case::ALL {
        assert_eq!(case.name().parse::<Case>(), Ok(case));
    }
    assert_eq!(
        "disk-full"
            .parse::<Case>()
            .map_err(|unknown| unknown.to_string()),
        Err(
            "no ingest class case disk-full; known: trigger-abort, missing-schema, storage-full"
                .to_owned()
        )
    );
}

#[test]
fn a_verdict_names_itself_first() {
    assert_eq!(Verdict::Holds.line(), "verdict=Holds");
    assert_eq!(
        Verdict::Unreachable("PRAGMA refused".to_owned()).line(),
        "verdict=Unreachable PRAGMA refused"
    );
}
