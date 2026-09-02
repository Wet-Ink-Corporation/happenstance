//! AC-007 — outside the gate by construction, not by omission.
//!
//! "Outside the gate" is a property of two manifests, so it is checked by
//! reading two manifests. Discovery's second mutant is a gate step with a timing
//! threshold: it passes on the machine it was written on, and its first red
//! build is resolved by raising the threshold, after which it measures nothing
//! and blocks everything.

use std::path::{Path, PathBuf};

fn here() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn read(rel: &str) -> String {
    let path = here().join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()))
        .replace("\r\n", "\n")
}

#[test]
fn the_harness_manifest_opens_with_a_bare_workspace_table() {
    let manifest = read("Cargo.toml");
    assert!(
        manifest.contains("\n[workspace]\n"),
        "without a bare `[workspace]` table cargo walks up to the repository \
         root and adopts this crate as a member, which puts a benchmark in the \
         conformance bar CF-34 keeps it out of"
    );
    // And the reason is beside it, because a bare table with no comment is a
    // line the next reader deletes.
    let at = manifest.find("[workspace]").expect("the table is there");
    assert!(
        manifest[..at].contains("outside the workspace"),
        "the bare `[workspace]` table carries no note saying why it is there"
    );
    assert!(manifest.contains("publish = false"));
}

#[test]
fn the_root_members_list_names_no_experiment() {
    let root = read("../../Cargo.toml");
    let start = root
        .find("members")
        .expect("the workspace has a members list");
    let end = root[start..].find(']').map_or(root.len(), |at| start + at);
    let members = &root[start..end];
    assert!(
        !members.contains("experiments"),
        "the root `members` list names an experiment: {members}"
    );
}

#[test]
fn no_gate_step_names_this_harness() {
    let gate = read("../../xtask/src/main.rs");
    assert!(
        !gate.contains("polling-cost") && !gate.contains("polling_cost"),
        "`xtask/src/main.rs` names this harness, so a measurement has entered \
         the bar"
    );
}

#[test]
fn the_affected_gate_still_treats_experiments_as_inert() {
    let affected = read("../../xtask/src/affected.rs");
    assert!(
        affected.contains("\"experiments/\""),
        "`is_inert`'s `experiments/` prefix is gone, so a diff touching only \
         this directory would start selecting packages"
    );
}

#[test]
fn no_crate_manifest_depends_on_this_harness() {
    let crates = here().join("../../crates");
    for entry in std::fs::read_dir(&crates)
        .expect("crates/ is readable")
        .flatten()
    {
        let manifest = entry.path().join("Cargo.toml");
        let Ok(body) = std::fs::read_to_string(&manifest) else {
            continue;
        };
        assert!(
            !body.contains("polling-cost"),
            "{} depends on the harness",
            manifest.display()
        );
    }
    let root = read("../../Cargo.toml");
    assert!(
        !root.contains("polling-cost"),
        "`[workspace.dependencies]` names the harness"
    );
}
