//! AC-006 and AC-009 — the artefact is composed, and it states its own gap.
//!
//! The strong check here is the last one: the headline figure printed in §1 is
//! recomputed from the committed corpus and compared. Prose and numbers in the
//! same directory drift, and the only thing that stops them is a test that reads
//! both.

use std::path::{Path, PathBuf};

use serde_json::Value;

fn here() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn readme() -> String {
    std::fs::read_to_string(here().join("README.md"))
        .expect("the harness has a README")
        .replace("\r\n", "\n")
}

fn records() -> Vec<Value> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(here().join("results")) else {
        return out;
    };
    for entry in entries.flatten() {
        let dir = entry.path();
        let tag = dir
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if !dir.is_dir() || !tag.starts_with("pass-") {
            continue;
        }
        let Ok(body) = std::fs::read_to_string(dir.join("records.ndjson")) else {
            continue;
        };
        for line in body.lines().filter(|l| !l.trim().is_empty()) {
            let value: Value = serde_json::from_str(line).expect("one object per line");
            if value["kind"] == "record" {
                out.push(value);
            }
        }
    }
    out
}

/// The sections, in the order the sibling established and this artefact keeps.
const SECTIONS: [&str; 6] = [
    "## 1. What was measured, on what",
    "## 2. The numbers",
    "## 3. Staleness",
    "## 4. What this does not prove",
    "## 5. The verdict",
    "## 6. How to re-run it",
];

#[test]
fn the_sections_are_present_and_in_the_declared_order() {
    let readme = readme();
    let mut cursor = 0usize;
    for heading in SECTIONS {
        let at = readme[cursor..].find(heading).unwrap_or_else(|| {
            panic!("the README has no `{heading}` section, or it is out of order")
        });
        cursor += at + heading.len();
    }
}

#[test]
fn the_gap_is_named_in_the_falsifiers_own_words() {
    let readme = readme();
    let start = readme
        .find("## 4. What this does not prove")
        .expect("the section exists");
    let end = readme[start..]
        .find("\n## ")
        .map_or(readme.len(), |at| start + at);
    let section = &readme[start..end];

    for phrase in ["on a real deployment", "their staleness budget", "floor"] {
        assert!(
            section.contains(phrase),
            "`## 4. What this does not prove` does not contain `{phrase}`, so a \
             reader cannot tell how far this number is from ES-32's own terms"
        );
    }
}

#[test]
fn the_verdict_quotes_the_figure_and_stops_there() {
    let readme = readme();
    let start = readme
        .find("## 5. The verdict")
        .expect("the section exists");
    let end = readme[start..]
        .find("\n## ")
        .map_or(readme.len(), |at| start + at);
    let section = readme[start..end].to_lowercase();

    for recommendation in [
        "we recommend",
        "should add a tail",
        "should not add a tail",
        "therefore add",
        "therefore withhold",
    ] {
        assert!(
            !section.contains(recommendation),
            "the verdict section recommends something (`{recommendation}`); this \
             artefact produces evidence and the decision belongs elsewhere"
        );
    }
}

#[test]
fn the_headline_figure_matches_the_committed_corpus() {
    let records = records();
    assert!(
        !records.is_empty(),
        "there is no committed corpus to check against"
    );

    let peak = records
        .iter()
        .filter(|r| r["phase"] == "amplification" && r["arm"] == "overlapping")
        .filter_map(|r| r["delivery_amplification"].as_f64())
        .fold(0.0_f64, f64::max);
    let printed = format!("{peak:.2}");

    let readme = readme();
    let start = readme
        .find("## 1. What was measured, on what")
        .expect("the section exists");
    let end = readme[start..]
        .find("\n## 3.")
        .map_or(readme.len(), |at| start + at);
    assert!(
        readme[start..end].contains(&printed),
        "the README's headline does not carry `{printed}`, which is the peak \
         overlapping-arm amplification recomputed from results/. Prose and \
         corpus have drifted"
    );
}

#[test]
fn the_ratio_precedes_the_first_timing_figure() {
    let readme = readme();
    let ratio = readme
        .find("amplification")
        .expect("the README names the headline");
    // A *timing figure*, not the two letters `ns`, which occur inside
    // `consumers` and half a dozen other words. A duration here is a number
    // followed by a unit.
    let timing = [" ms", " s ", " ns", "milliseconds"]
        .into_iter()
        .filter_map(|unit| readme.find(unit))
        .min()
        .unwrap_or(usize::MAX);
    assert!(
        ratio < timing,
        "a timing figure appears before the ratio it qualifies, which is the \
         first mutant's reading order"
    );
}

#[test]
fn no_table_runs_past_eight_rows_without_saying_it_was_trimmed() {
    let readme = readme();
    let mut rows = 0usize;
    let mut overflowed = false;
    for line in readme.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('|') {
            rows += 1;
            if rows > 10 {
                overflowed = true;
            }
        } else {
            if overflowed {
                assert!(
                    readme.contains("and N more") || readme.contains("… and"),
                    "a table runs past its density budget with no overflow marker"
                );
            }
            rows = 0;
            overflowed = false;
        }
    }
}
