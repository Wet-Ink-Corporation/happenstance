//! The convergence family against its reference fixtures.
//!
//! `StockOnHandFixture` is the declared projection SY-20's rule is proven
//! against before it may fail anyone's, and SY-22's rule carries its own
//! cases: a rule red here is a wrong rule.

use happenstance_sync_testkit::{StockOnHandFixture, Violation, convergence_rules};
use happenstance_testkit::{RuleOutcome, block_on};

/// A sum of stock movements commits one read model under both interleavings.
#[test]
fn stock_on_hand_is_interleaving_independent() -> Result<(), Violation> {
    let outcome = block_on(
        convergence_rules::convergent_projection_is_interleaving_independent(async || {
            StockOnHandFixture::new()
        }),
    )?;
    assert_eq!(outcome, RuleOutcome::Ran);
    Ok(())
}

/// Cost layers are left out of the check, and would be failed by it.
#[test]
fn cost_layers_are_excluded_not_failed() -> Result<(), Violation> {
    let outcome =
        block_on(convergence_rules::non_convergent_projections_are_excluded_not_failed())?;
    assert_eq!(outcome, RuleOutcome::Ran);
    Ok(())
}
