//! Reading the harness's own source, so a test can hold it to its own rules.
//!
//! Separated from the measuring code on purpose. `no_verdict.rs` sweeps the
//! harness for thresholds and panics and would otherwise have to carve out an
//! exception for the one helper that legitimately has one — and an exception
//! inside a negative assertion is where the next real threshold hides.

use std::path::{Path, PathBuf};

/// Every source file of this harness, for the assertions that read it.
///
/// Exposed so a test can hold the harness to its own negative requirements — no
/// threshold, no verdict, no subtraction — without re-deriving where the files
/// are.
///
/// # Panics
///
/// Panics if the harness's own `src/` is unreadable, which means the test is
/// running somewhere it cannot see the thing it is asserting about.
#[must_use]
pub fn harness_sources() -> Vec<(PathBuf, String)> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut out = Vec::new();
    let entries = std::fs::read_dir(&dir).expect("the harness's own src/ is readable");
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "rs") {
            let body = std::fs::read_to_string(&path).expect("valid utf-8");
            out.push((path, body));
        }
    }
    assert!(!out.is_empty(), "the harness has sources");
    out
}
