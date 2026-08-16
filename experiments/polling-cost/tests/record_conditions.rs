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
    "ram_bytes",
    "features",
    "started_at",
    "projection_store",
];

// ---------------------------------------------------------------------------
// Enough of RFC 3339 to tell two instants apart
// ---------------------------------------------------------------------------

/// Seconds since the Unix epoch, for the two shapes this file meets.
///
/// The harness writes `YYYY-MM-DDTHH:MM:SSZ`; git's `%cI` writes the same date
/// and time with a `±HH:MM` offset and **never** a `Z`. Both are parsed here so
/// the two instants can be compared as instants rather than as strings, which
/// is the only comparison that survives a change of time zone.
fn epoch_seconds(text: &str) -> Option<i64> {
    let field = |from: usize, to: usize| -> Option<i64> { text.get(from..to)?.parse::<i64>().ok() };
    if text.len() < 20 || !text.is_char_boundary(19) {
        return None;
    }
    let separators = [(4, '-'), (7, '-'), (10, 'T'), (13, ':'), (16, ':')];
    if separators
        .iter()
        .any(|&(at, ch)| text.as_bytes().get(at) != Some(&(ch as u8)))
    {
        return None;
    }

    let date = days_from_civil(field(0, 4)?, field(5, 7)?, field(8, 10)?);
    let clock = field(11, 13)? * 3_600 + field(14, 16)? * 60 + field(17, 19)?;

    let zone = &text[19..];
    let offset = if zone == "Z" {
        0
    } else if zone.len() == 6 && zone.as_bytes().get(3) == Some(&b':') {
        let sign = match zone.as_bytes().first() {
            Some(b'+') => 1,
            Some(b'-') => -1,
            _ => return None,
        };
        let hours = zone.get(1..3)?.parse::<i64>().ok()?;
        let minutes = zone.get(4..6)?.parse::<i64>().ok()?;
        sign * (hours * 3_600 + minutes * 60)
    } else {
        return None;
    };

    Some(date * 86_400 + clock - offset)
}

/// Howard Hinnant's `days_from_civil`, the inverse of the one the harness uses
/// to format. Transcribed rather than depended on: the harness takes no date
/// crate and neither does the instrument that checks it.
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let month_prime = (month + 9) % 12;
    let day_of_year = (153 * month_prime + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// The commit instant of the revision a pass names, straight from git.
fn commit_instant(rev: &str) -> i64 {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(&workspace)
        .args(["log", "-1", "--format=%cI", rev])
        .output()
        .unwrap_or_else(|e| panic!("git is how a pass records its revision at all: {e}"));
    assert!(
        out.status.success(),
        "git does not know `{rev}`, which the committed pass names as the tree it \
         measured"
    );
    let text = String::from_utf8_lossy(&out.stdout).trim().to_owned();
    epoch_seconds(&text).unwrap_or_else(|| panic!("git printed `{text}`, which is not RFC 3339"))
}

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

/// The regression this test exists for: `started_at` read from
/// `git log -1 --format=%cI`, which is the **commit** instant wearing the run
/// instant's name.
///
/// Two passes taken months apart at the same revision were byte-identical in
/// the one field that separates them, which is the additive-pass story failing
/// silently. Both halves matter: the shape, because `%cI` renders UTC as
/// `+00:00` and never as `Z`; and the ordering, because a run happens strictly
/// after the commit it measures, so equality is the defect.
#[test]
fn started_at_is_the_run_instant_and_not_the_commit_instant() {
    for (tag, lines) in committed() {
        let manifest = lines.first().expect("a pass has a manifest");
        let started = manifest["started_at"]
            .as_str()
            .unwrap_or_else(|| panic!("{tag}: `started_at` is not a string"));

        assert!(
            started.ends_with('Z'),
            "{tag}: `started_at` is `{started}`, which carries a UTC offset \
             rather than the `Z` this harness writes. git's `%cI` renders UTC \
             as `+00:00`, so an offset here is the commit instant leaking back \
             into the field"
        );
        let run = epoch_seconds(started).unwrap_or_else(|| {
            panic!("{tag}: `started_at` is `{started}`, which does not parse as an instant")
        });

        let rev = manifest["git_rev"]
            .as_str()
            .unwrap_or_else(|| panic!("{tag}: `git_rev` is not a string"));
        let committed_at = commit_instant(rev);
        assert!(
            run > committed_at,
            "{tag}: `started_at` is at or before the instant `{rev}` was \
             committed, so it is reporting the commit rather than the run. A \
             pass is taken after the revision it measures exists"
        );
    }
}

/// AC-003 enumerates RAM among the conditions, so a committed pass carries one.
///
/// Zero is admissible in the schema — a host may decline to answer — but not in
/// a *recorded* pass, for the same reason `git_dirty: true` is not: an
/// exploratory run cannot be mistaken for the one the README quotes.
#[test]
fn a_committed_pass_records_the_ram_it_ran_on() {
    for (tag, lines) in committed() {
        let manifest = lines.first().expect("a pass has a manifest");
        let ram = manifest["ram_bytes"]
            .as_u64()
            .unwrap_or_else(|| panic!("{tag}: `ram_bytes` is not an integer"));
        assert!(
            ram > 0,
            "{tag}: `ram_bytes` is 0, which the schema reads as *the host was \
             asked and did not say*. A committed pass records the machine it \
             ran on"
        );
    }
}
