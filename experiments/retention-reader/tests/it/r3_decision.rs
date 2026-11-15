//! R3, a decision model over a holed log. Recorded only, and not part of the
//! verdict: ES-40 and #154 own it.

use core::fmt;
use core::num::NonZeroU32;

use happenstance::{CommandError, Retry, commit};
use happenstance_testkit::block_on;
use retention_reader::domain::{ClaimEvent, ClaimFile};
use retention_reader::holed::Holed;
use retention_reader::seams::{seams, young_twin};
use retention_reader::shared::Shared;

use crate::support::{Outcome, append_each, claim};

/// The decision's refusal: the claim's history already registers it.
#[derive(Debug)]
struct AlreadyRegistered;

impl fmt::Display for AlreadyRegistered {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("the claim is already registered")
    }
}

impl core::error::Error for AlreadyRegistered {}

/// **A recorded hazard, specified behaviour (ES-40's MAY).** "Register claim
/// `a` unless it is registered": the full store refuses; the store that has
/// forgotten `a`'s registration admits a second one, its append condition
/// passing vacuously, and no port seam differs from a young store that never
/// held the registration. A decision layer that refused on an
/// `Unknown` completeness answer would refuse on every published adapter
/// (C3a), since all of them would answer `Unknown` for ever.
#[test]
fn a_decision_over_a_holed_log_admits_what_the_full_log_refuses() -> Outcome {
    block_on(async {
        let a = claim("a")?;
        let history = [
            ClaimEvent::registered(a.clone()),
            ClaimEvent::noted(a.clone(), 1),
        ];
        let full = Shared::new();
        append_each(&full, &history).await?;
        let holed = Holed::new(Shared::new());
        let landed = append_each(holed.inner(), &history).await?;
        holed.view().withhold(landed.get(..1).ok_or("two")?).await?;
        let pruned = seams(holed.view(), &[]).await?;
        let young = seams(&young_twin(&pruned.held).await?, &[]).await?;

        let retry = Retry::attempts(NonZeroU32::MIN);
        // Each decision stores its own claim: a short string, copied once.
        let register = |file: &ClaimFile| {
            if file.is_registered() {
                Err(AlreadyRegistered)
            } else {
                Ok(vec![ClaimEvent::registered(a.clone())])
            }
        };
        let on_full = commit(&full, ClaimFile::of(&a), retry, register).await;
        let on_holed = commit(holed.view(), ClaimFile::of(&a), retry, register).await;

        assert!(
            matches!(on_full, Err(CommandError::Refused(AlreadyRegistered))),
            "{on_full:?}"
        );
        let admitted = on_holed?.committed().ok_or("the decision appended")?;
        assert!(
            Some(admitted.position) > landed.last().copied(),
            "a second registration"
        );
        assert_eq!(
            pruned.shape(),
            young.shape(),
            "the port cannot tell them apart"
        );
        Ok(())
    })
}
