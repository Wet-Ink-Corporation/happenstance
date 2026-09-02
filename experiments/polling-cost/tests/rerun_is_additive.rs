//! AC-010 — a re-run writes beside the pass it is compared with, never over it.
//!
//! Committed results are the record of a run that happened. Overwriting one
//! silently is how a comparison loses its baseline, and undoing a re-run should
//! be deleting one directory rather than reconstructing a file.

use std::path::PathBuf;

use polling_cost::{Manifest, Record, SCHEMA_VERSION, write_pass};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("polling-cost-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

fn manifest(tag: &str) -> Manifest {
    Manifest {
        schema_version: SCHEMA_VERSION,
        kind: "manifest".to_owned(),
        run_tag: tag.to_owned(),
        rustc: "rustc 1.97.1".to_owned(),
        cargo: "cargo 1.97.1".to_owned(),
        git_rev: "0000000".to_owned(),
        git_dirty: false,
        profile: "release".to_owned(),
        os: "test".to_owned(),
        cpu: "test".to_owned(),
        logical_cpus: 1,
        ram_bytes: 0,
        features: "unstable-projection,memory,json".to_owned(),
        started_at: "2026-08-16T00:00:00Z".to_owned(),
        projection_store: polling_cost::PROJECTION_STORE.to_owned(),
    }
}

fn record(amplification: f64) -> Record {
    Record {
        schema_version: SCHEMA_VERSION,
        kind: "record".to_owned(),
        phase: "amplification".to_owned(),
        fan_out: 1,
        log_size: 1,
        poll_interval_ms: 10,
        chunk: 1,
        arm: "overlapping".to_owned(),
        repeat: 0,
        seed: 1,
        projection_store: polling_cost::PROJECTION_STORE.to_owned(),
        reads_issued: 1,
        events_delivered: 1,
        events_applied_total: 1,
        events_applied_distinct: 1,
        delivery_amplification: amplification,
        staleness_ns_p50: None,
        staleness_ns_p95: None,
        pending_at_poll_max: None,
        elapsed_ns: 1,
    }
}

#[test]
fn a_second_run_under_the_same_tag_is_refused_and_changes_nothing() {
    let root = scratch("rerun");

    let first = write_pass(&root, "pass-001", &manifest("pass-001"), &[record(1.0)])
        .expect("the first pass writes");
    let before = std::fs::read(&first).expect("the first pass is readable");

    let refused = write_pass(&root, "pass-001", &manifest("pass-001"), &[record(99.0)]);
    let why = refused.expect_err("the second run under the same tag is refused");
    let message = why.to_string();
    assert!(
        message.contains("pass-001"),
        "the refusal does not name the existing pass: {message}"
    );

    let after = std::fs::read(&first).expect("the first pass is still readable");
    assert_eq!(
        before, after,
        "the refused re-run edited the committed bytes"
    );

    // A new tag lands beside it, and both survive independently.
    let second = write_pass(&root, "pass-002", &manifest("pass-002"), &[record(2.0)])
        .expect("a new tag writes");
    assert!(first.is_file() && second.is_file());
    assert_ne!(first, second);
    assert_eq!(
        std::fs::read(&first).expect("still readable"),
        before,
        "writing a second pass disturbed the first"
    );

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_pass_is_one_directory_so_undoing_a_rerun_is_one_delete() {
    let root = scratch("undo");
    write_pass(&root, "pass-001", &manifest("pass-001"), &[record(1.0)]).expect("writes");
    write_pass(&root, "pass-002", &manifest("pass-002"), &[record(2.0)]).expect("writes");

    std::fs::remove_dir_all(root.join("pass-002")).expect("one delete undoes the re-run");
    assert!(root.join("pass-001").is_dir());
    assert!(!root.join("pass-002").exists());

    let _ = std::fs::remove_dir_all(&root);
}
