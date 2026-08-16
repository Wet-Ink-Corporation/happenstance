//! AC-003 — the conditions are part of the artefact, not metadata about it.
//!
//! A number without its conditions **is** an estimate wearing a decimal point.
//! Every field below is a condition of the figure beside it, and a committed
//! pass whose measured tree was dirty is a figure nobody can re-derive.

use std::path::{Path, PathBuf};

use serde_json::Value;

fn results() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("results")
}

/// Every committed pass, as `(tag, lines)`.
fn passes() -> Vec<(String, Vec<Value>)> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(results()) else {
        return out;
    };
    for entry in entries.flatten() {
        let dir = entry.path();
        if !dir.is_dir() {
            continue;
        }
        let tag = dir
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let file = dir.join("records.ndjson");
        let Ok(body) = std::fs::read_to_string(&file) else {
            continue;
        };
        let lines = body
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str::<Value>(l).expect("every line is one JSON object"))
            .collect();
        out.push((tag, lines));
    }
    out
}

/// The passes this repository committed, as opposed to any left behind by a
/// local re-run.
fn committed() -> Vec<(String, Vec<Value>)> {
    passes()
        .into_iter()
        .filter(|(tag, _)| tag.starts_with("pass-"))
        .collect()
}

const REQUIRED_ON_EVERY_RECORD: &[&str] = &[
    "schema_version",
    "kind",
    "phase",
    "fan_out",
    "log_size",
    "poll_interval_ms",
    "chunk",
    "arm",
    "repeat",
    "seed",
    "projection_store",
    "reads_issued",
    "events_delivered",
    "events_applied_total",
    "events_applied_distinct",
    "delivery_amplification",
    "elapsed_ns",
];

const REQUIRED_ON_THE_MANIFEST: &[&str] = &[
    "schema_version",
    "kind",
    "run_tag",
    "rustc",
    "cargo",
    "git_rev",
    "git_dirty",
    "profile",
    "os",
    "cpu",
    "logical_cpus",
    "features",
    "started_at",
    "projection_store",
];

#[test]
fn a_pass_is_committed_at_all() {
    assert!(
        !committed().is_empty(),
        "no committed pass under results/: the harness exists and its output does not"
    );
}

#[test]
fn every_record_carries_every_condition() {
    for (tag, lines) in committed() {
        for (index, line) in lines.iter().enumerate() {
            let kind = line["kind"].as_str().unwrap_or_default();
            let required = if kind == "manifest" {
                REQUIRED_ON_THE_MANIFEST
            } else {
                REQUIRED_ON_EVERY_RECORD
            };
            for field in required {
                let value = &line[*field];
                assert!(!value.is_null(), "{tag} line {index}: `{field}` is absent");
                if let Some(text) = value.as_str() {
                    assert!(
                        !text.trim().is_empty(),
                        "{tag} line {index}: `{field}` is empty"
                    );
                }
            }
        }
    }
}

#[test]
fn the_manifest_appears_exactly_once_per_pass() {
    for (tag, lines) in committed() {
        let manifests = lines.iter().filter(|l| l["kind"] == "manifest").count();
        assert_eq!(
            manifests, 1,
            "{tag} carries {manifests} manifest lines; the environment block is \
             written once per pass and referred to by every record after it"
        );
        assert_eq!(
            lines.first().map(|l| &l["kind"]),
            Some(&Value::from("manifest"))
        );
    }
}

#[test]
fn a_committed_pass_measured_a_clean_tree() {
    for (tag, lines) in committed() {
        let manifest = lines.first().expect("a pass has a manifest");
        assert_eq!(
            manifest["git_dirty"],
            Value::Bool(false),
            "{tag} was measured against a tree with uncommitted changes under \
             crates/ or the workspace manifests, so its figures cannot be \
             re-derived from the revision it names"
        );
        assert_eq!(
            manifest["profile"],
            Value::from("release"),
            "{tag} was measured in a debug build"
        );
    }
}
