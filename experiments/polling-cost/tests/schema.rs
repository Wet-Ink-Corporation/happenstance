//! AC-005 — the committed corpus and the committed schema cannot drift apart.
//!
//! Results are only reproducible if the thing they must conform to is in the
//! tree beside them, and a schema nothing validates against is a document.

use std::path::{Path, PathBuf};

use serde_json::Value;

fn here() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn schema(name: &str) -> Value {
    let path = here().join("schema").join(name);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
    serde_json::from_str(&body).expect("the schema is valid JSON")
}

fn passes() -> Vec<(String, Vec<Value>)> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(here().join("results")) else {
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
        if !tag.starts_with("pass-") {
            continue;
        }
        let Ok(body) = std::fs::read_to_string(dir.join("records.ndjson")) else {
            continue;
        };
        out.push((
            tag,
            body.lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| serde_json::from_str(l).expect("one JSON object per line"))
                .collect(),
        ));
    }
    out
}

#[test]
fn every_committed_line_validates_against_the_committed_schema() {
    let record_schema = schema("result-record.schema.json");
    let manifest_schema = schema("manifest.schema.json");

    let passes = passes();
    assert!(!passes.is_empty(), "there is no committed pass to validate");

    for (tag, lines) in passes {
        for (index, line) in lines.iter().enumerate() {
            let against = if line["kind"] == "manifest" {
                &manifest_schema
            } else {
                &record_schema
            };
            polling_cost::validate::against(against, line)
                .unwrap_or_else(|why| panic!("{tag} line {index}: {why}"));
        }
    }
}

#[test]
fn the_schema_version_in_the_corpus_matches_the_harness() {
    for (tag, lines) in passes() {
        for (index, line) in lines.iter().enumerate() {
            assert_eq!(
                line["schema_version"],
                Value::from(polling_cost::SCHEMA_VERSION),
                "{tag} line {index} was written under a different record contract \
                 than the harness now ships; bump the version rather than \
                 re-interpreting the file"
            );
        }
    }
}

#[test]
fn the_projection_store_is_one_of_exactly_two_named_values() {
    let admissible = [
        polling_cost::PROJECTION_STORE,
        polling_cost::PROJECTION_STORE_LOCAL,
    ];
    for (tag, lines) in passes() {
        for (index, line) in lines.iter().enumerate() {
            let value = line["projection_store"]
                .as_str()
                .unwrap_or_else(|| panic!("{tag} line {index}: no projection_store field"));
            assert!(
                admissible.contains(&value),
                "{tag} line {index}: `{value}` is free text, not one of the two \
                 stores this harness is allowed to have used"
            );
        }
    }
}

#[test]
fn the_seed_is_in_the_file_rather_than_in_prose() {
    for (tag, lines) in passes() {
        for line in lines.iter().filter(|l| l["kind"] == "record") {
            assert!(
                line["seed"].is_u64(),
                "{tag}: a record carries no seed, so its randomised choices are \
                 described somewhere a re-run cannot read"
            );
        }
    }
}
