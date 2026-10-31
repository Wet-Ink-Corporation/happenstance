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

/// The lazy judge answers `Holds` for any failure that recurs, so a case that
/// staged the wrong failure (here a claim refused before any SQL ran) would
/// pass as evidence for `Thrown → Stop` without ever reaching a throw.
#[test]
fn a_failure_of_another_variant_is_unreachable_not_holds() {
    let unheld = Err(Observed {
        class: FailureClass::Stop,
        variant: "UnheldOwnIdentity",
        error: "event … claims this store's identity".to_owned(),
    });
    let refused = Err(Observed {
        class: FailureClass::Split,
        variant: "ExceedsStoreLimit",
        error: "ingest exceeds the store's events per batch limit: 1025".to_owned(),
    });

    assert_eq!(
        judge(Case::TriggerAbort, &unheld, &unheld.clone()),
        Verdict::Unreachable("provoked UnheldOwnIdentity, not Sql(Thrown)".to_owned())
    );
    assert_eq!(
        judge(Case::MissingSchema, &refused, &refused.clone()),
        Verdict::Unreachable("provoked ExceedsStoreLimit, not Sql(Thrown)".to_owned())
    );
    assert_eq!(
        judge(Case::StorageFull, &unheld, &unheld.clone()),
        Verdict::Unreachable(
            "provoked UnheldOwnIdentity, not Sql(StorageLimitExceeded)".to_owned()
        )
    );
}

#[test]
fn a_full_object_reported_as_the_limit_holds() {
    let full = Err(Observed {
        class: FailureClass::Stop,
        variant: "Sql(StorageLimitExceeded)",
        error: "the Durable Object's SQL storage limit was exceeded".to_owned(),
    });

    assert_eq!(
        judge(Case::StorageFull, &full, &full.clone()),
        Verdict::Holds
    );
}

/// No case provokes a `Transient` today, because the adapter classes every
/// `Thrown` as `Stop`. A message-marked `Transient` arm for some `Thrown` text
/// (D-E3-1's remedy) would make one, and the judge must then read the retry the
/// other way round.
#[test]
fn a_transient_holds_only_if_its_retry_lands() {
    let busy = Err(Observed {
        class: FailureClass::Transient,
        variant: "Sql(Thrown)",
        error: "borrowed".to_owned(),
    });

    assert_eq!(
        judge(Case::TriggerAbort, &busy, &Ok(landed())),
        Verdict::Holds
    );
    assert_eq!(
        judge(Case::TriggerAbort, &busy, &busy.clone()),
        Verdict::Falsifies("Transient (Sql(Thrown)): borrowed".to_owned())
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
