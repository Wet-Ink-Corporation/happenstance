//! The sweep driver.
//!
//! One binary, no arguments beyond the environment: `HS_TAG` names the pass,
//! `HS_RUNNER` picks the runner (`polling`, the default, or `fan_out`), and
//! `HS_CELLS` trims it for a smoke run. It writes one newline-delimited JSON
//! file per pass under `results/<tag>/` and refuses to overwrite a pass that is
//! already there.
//!
//! It exits non-zero only when the **harness** fails. There is no threshold
//! here, no assertion about a number, and no recommendation about the tail seam:
//! CF-34 keeps performance out of the conformance bar, and a harness that could
//! fail on a value is a gate step wearing a different name.

use std::path::Path;
use std::process::ExitCode;

use polling_cost::{Cell, Record, Runner, environment, progress, run_cell, sweep, write_pass};

fn main() -> ExitCode {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace = here.join("../..");
    let results = here.join("results");

    let tag = std::env::var("HS_TAG").unwrap_or_else(|_| "pass-001".to_owned());
    // `polling` unless asked otherwise, so pass-001's command still measures
    // what pass-001 measured. Anything else is refused, not run as the default.
    let runner = match std::env::var("HS_RUNNER") {
        Err(std::env::VarError::NotPresent) => Runner::Polling,
        Err(std::env::VarError::NotUnicode(value)) => {
            progress::say(&format!(
                "harness failed: HS_RUNNER={value:?} is not `polling` or `fan_out`"
            ));
            return ExitCode::FAILURE;
        }
        Ok(value) => match Runner::parse(&value) {
            Some(runner) => runner,
            None => {
                progress::say(&format!(
                    "harness failed: HS_RUNNER={value:?} is not `polling` or `fan_out`"
                ));
                return ExitCode::FAILURE;
            }
        },
    };
    let limit = std::env::var("HS_CELLS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok());

    let cells: Vec<Cell> = match limit {
        Some(n) => sweep().into_iter().take(n).collect(),
        None => sweep(),
    };

    progress::say(&format!(
        "polling-cost: {} cells, tag {tag}, runner {}",
        cells.len(),
        runner.as_str()
    ));
    let manifest = environment(&tag, &workspace);
    progress::say(&format!(
        "measured tree {} dirty={}",
        manifest.git_rev, manifest.git_dirty
    ));

    let total = cells.len();
    let mut records: Vec<Record> = Vec::with_capacity(total);
    for (index, cell) in cells.iter().enumerate() {
        progress::say(&progress::cell_line(
            index + 1,
            total,
            &format!(
                "{} n={} log={} poll={}ms arm={}",
                cell.phase.as_str(),
                cell.fan_out,
                cell.log_size,
                cell.poll_interval_ms,
                cell.arm.as_str()
            ),
        ));
        match run_cell(cell, runner) {
            Ok(record) => records.push(record),
            Err(why) => {
                progress::say(&format!("harness failed: {why}"));
                return ExitCode::FAILURE;
            }
        }
    }

    match write_pass(&results, &tag, &manifest, &records) {
        Ok(path) => {
            progress::say(&format!("wrote {}", path.display()));
            ExitCode::SUCCESS
        }
        Err(why) => {
            progress::say(&format!("harness failed: {why}"));
            ExitCode::FAILURE
        }
    }
}
