//! Turns a criterion run into one committed history entry.
//!
//! # Why a history exists at all
//!
//! CF-34 (`spec/SPECIFICATION.md:8747`) does not merely forbid a benchmark from
//! gating a merge; it says what the alternative is:
//!
//! > Benchmarks are published per adapter and compared against **that adapter's
//! > own history**; they decide nothing about conformance.
//!
//! `references/seeds/measured-not-claimed.md:106-112` records that no such
//! history exists, and why that matters: *"A measurement that lives in the
//! commit message of the change it justified is findable only by someone who
//! already knows it exists, which is exactly the person who does not need it."*
//!
//! So every `./run.sh` writes one JSON file into `results/history/`, named for
//! the date and commit it was taken at, and that file is committed. Comparing
//! two of them is a diff.
//!
//! # Why it reads criterion's output rather than criterion reading it
//!
//! criterion already has `--save-baseline` and `--baseline`, and they are the
//! right tool for *"did this change make it slower"* on one machine within one
//! afternoon. What they do not give is a **committed, portable, human-readable**
//! record: a criterion baseline lives under `target/`, is not in the repository,
//! and is unreadable without criterion. This binary reads the same
//! `estimates.json` files criterion writes and flattens them into the same
//! [`Row`] shape `overhead` and `allocations` emit, so all three land in one
//! format a reader can diff by eye.
//!
//! Both are used. `run.sh` passes `--save-baseline` so criterion's own
//! statistical comparison is available locally, *and* runs this so the numbers
//! survive into the repository.
//!
//! # What is carried, and what is dropped
//!
//! The **median** and its 95% confidence bounds, and the mean. Not the
//! bootstrap distribution, not the outlier classification, not the slope — all
//! of which are in `target/criterion/` for anyone who wants them, and none of
//! which a hand-written results table quotes.
//!
//! # Usage
//!
//! ```console
//! cargo run --release --bin collect
//! ```
//!
//! Reads `target/criterion/`, writes `results/history/<date>-<commit>.json` and
//! prints the same rows as CSV to stdout. Exits zero with a warning if there is
//! no criterion output to read — a fresh checkout has none, and a collector
//! that failed there would make `./run.sh` fail on its first ever invocation.

use std::fs;
use std::path::{Path, PathBuf};

use happenstance_benchmarks::report::{Row, RunRecord};
use serde::Deserialize;

/// Where criterion writes, relative to the crate root.
const CRITERION_DIR: &str = "target/criterion";

/// Where a run's record is committed.
const HISTORY_DIR: &str = "results/history";

/// The subset of criterion's `estimates.json` this reads.
///
/// `#[serde(default)]` on every field: criterion's schema is not a stable
/// interface and a version that stopped writing one of these should degrade to
/// a zero in one column rather than fail the whole collection.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Estimates {
    mean: Estimate,
    median: Estimate,
}

/// One estimate: a point and its confidence interval.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Estimate {
    point_estimate: f64,
    confidence_interval: ConfidenceInterval,
}

/// The 95% bounds criterion computes by bootstrap.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct ConfidenceInterval {
    lower_bound: f64,
    upper_bound: f64,
}

fn main() -> anyhow::Result<()> {
    let criterion_dir = Path::new(CRITERION_DIR);
    if !criterion_dir.is_dir() {
        eprintln!(
            "no {CRITERION_DIR} to read — run `cargo bench` first. Exiting zero: a fresh \
             checkout has no criterion output, and failing here would make ./run.sh fail on \
             its first ever invocation."
        );
        return Ok(());
    }

    let mut record = RunRecord::new();
    eprintln!("{}\n", record.conditions);

    let mut found = 0_usize;
    for estimates_path in find_new_estimates(criterion_dir)? {
        let Some((group, arm, shape)) = identify(criterion_dir, &estimates_path) else {
            continue;
        };
        let text = fs::read_to_string(&estimates_path)?;
        let estimates: Estimates = serde_json::from_str(&text)?;
        found += 1;

        for (metric, value) in [
            ("median", estimates.median.point_estimate),
            (
                "median-ci-lower",
                estimates.median.confidence_interval.lower_bound,
            ),
            (
                "median-ci-upper",
                estimates.median.confidence_interval.upper_bound,
            ),
            ("mean", estimates.mean.point_estimate),
        ] {
            record.push(Row {
                group: group.clone(),
                arm: arm.clone(),
                shape: shape.clone(),
                metric: metric.to_owned(),
                value,
                unit: "ns".to_owned(),
                // criterion's own figures are absolutes, and this crate's rule
                // is that an absolute is quotable only from a run whose
                // stability was checked. criterion checks it differently —
                // through the confidence interval carried in the two rows above
                // — so the width of that interval is what a reader judges by,
                // and the flag stays true.
                representative: true,
            });
        }
    }

    if found == 0 {
        eprintln!("{CRITERION_DIR} exists but holds no `new/estimates.json` — nothing collected");
        return Ok(());
    }

    fs::create_dir_all(HISTORY_DIR)?;
    let name = format!(
        "{}-{}.json",
        record.conditions.date, record.conditions.commit
    );
    let path = Path::new(HISTORY_DIR).join(&name);
    fs::write(&path, serde_json::to_string_pretty(&record)?)?;
    eprintln!(
        "wrote {} rows from {found} benchmarks to {}",
        record.rows.len(),
        path.display()
    );

    print!("{}", record.to_csv());
    Ok(())
}

/// Every `new/estimates.json` under `root`.
///
/// `new/` rather than `base/`: `base/` is whatever the previous run left, and
/// collecting it would silently commit a figure from a different commit under
/// this one's name.
fn find_new_estimates(root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut found = Vec::new();
    let mut frontier = vec![root.to_path_buf()];

    while let Some(directory) = frontier.pop() {
        for entry in fs::read_dir(&directory)? {
            let path = entry?.path();
            if path.is_dir() {
                frontier.push(path);
            } else if path
                .file_name()
                .is_some_and(|name| name == "estimates.json")
                && path
                    .parent()
                    .and_then(Path::file_name)
                    .is_some_and(|parent| parent == "new")
            {
                found.push(path);
            }
        }
    }

    found.sort();
    Ok(found)
}

/// Splits criterion's directory path into this crate's `(group, arm, shape)`.
///
/// criterion writes `<root>/<group>/<function>/<parameter>/new/estimates.json`,
/// with `/` in a benchmark id replaced by `_`. A benchmark without a parameter
/// has one fewer level, so the shape is empty rather than the function's name
/// being misread as one.
///
/// Returns `None` for anything that does not fit — criterion also writes a
/// `report/` tree, and a directory layout this does not recognise should be
/// skipped rather than guessed at.
fn identify(root: &Path, estimates: &Path) -> Option<(String, String, String)> {
    let relative = estimates.strip_prefix(root).ok()?;
    // Drop `new/estimates.json`, leaving the identifying components.
    let components: Vec<String> = relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect();
    let identifying = components.get(..components.len().checked_sub(2)?)?;

    match identifying {
        [group, arm, shape] => Some((group.clone(), arm.clone(), shape.clone())),
        [group, arm] => Some((group.clone(), arm.clone(), String::new())),
        _ => None,
    }
}
