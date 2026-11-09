//! The watermark rule (SY-31, #143) against the reference projection store.

#![cfg(not(target_arch = "wasm32"))]

use happenstance_sync_testkit::Violation;
use happenstance_sync_testkit::watermark_rules::watermark_advances_transactionally_with_a_read_model as rule;
use happenstance_testkit::fixtures::MemoryProjectionFixture;

#[tokio::test]
async fn watermark_advances_transactionally_with_a_read_model() -> Result<(), Violation> {
    rule(async || MemoryProjectionFixture::new())
        .await?
        .report("watermark_advances_transactionally_with_a_read_model");
    Ok(())
}
