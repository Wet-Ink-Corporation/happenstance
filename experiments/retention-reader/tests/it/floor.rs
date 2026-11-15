//! The floor reader: what an `earliest_position()` answer would say about a
//! scattered purge. The evidence behind ES-39's *Rejects* line and ADR-0028
//! decision 4, run rather than argued.

use happenstance::SequencePosition;
use happenstance_testkit::block_on;
use retention_reader::holed::Holed;
use retention_reader::seams::seams;
use retention_reader::shared::Shared;

use crate::support::{Outcome, append_each, interleaved};

/// A regulated purge keeps a low survivor and destroys its neighbours. The
/// floor, the lowest retained position, is that survivor's, so "complete
/// above the floor" is false: the instrument's report (not port) names
/// withheld positions above it.
#[test]
fn a_floor_misdescribes_a_scattered_purge() -> Outcome {
    block_on(async {
        let holed = Holed::new(Shared::new());
        let landed = append_each(holed.inner(), &interleaved()?).await?;
        let purge: Vec<SequencePosition> = landed.iter().skip(1).step_by(2).copied().collect();
        holed.view().withhold(&purge).await?;

        let floor = seams(holed.view(), &[]).await?.floor();
        let withheld = holed.view().withheld();

        assert_eq!(
            floor,
            landed.first().copied(),
            "the low survivor sets the floor"
        );
        let above: Vec<_> = withheld.iter().filter(|&&p| Some(p) > floor).collect();
        assert_eq!(above.len(), purge.len(), "every hole sits above the floor");
        Ok(())
    })
}
