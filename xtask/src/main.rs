//! Workspace chores, runnable as `cargo xtask <command>`.
//!
//! The point of the [xtask pattern](https://github.com/matklad/cargo-xtask) is
//! that the CI gate is *one command* defined *once*, in Rust, rather than a
//! list of steps duplicated between a YAML file and everyone's memory. If
//! `cargo xtask ci` passes locally, CI passes.

#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::process::{Command, ExitCode, Stdio};

use anyhow::{Context, Result, bail};

/// A step in the CI gate.
struct Step {
    /// What this proves, shown while it runs.
    name: &'static str,
    /// The program to run.
    program: &'static str,
    /// Its arguments.
    args: &'static [&'static str],
    /// How to detect whether the tool is installed, for steps that depend on a
    /// cargo subcommand that may be absent.
    ///
    /// `None` means the step is mandatory. `Some(args)` means: run this probe
    /// first, skip the step if it fails, and otherwise treat the step exactly
    /// as mandatory. That distinction matters — "the tool is missing" must be a
    /// skip, but "the tool ran and found a problem" must be a failure. CI
    /// installs every optional tool, so nothing is skipped there.
    probe: Option<&'static [&'static str]>,
}

const REQUIRED: &[Step] = &[
    Step {
        name: "formatting",
        program: "cargo",
        args: &["fmt", "--all", "--check"],
        probe: None,
    },
    Step {
        name: "clippy (all targets, all features)",
        program: "cargo",
        args: &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
        probe: None,
    },
    Step {
        name: "tests",
        program: "cargo",
        args: &["test", "--workspace", "--all-features"],
        probe: None,
    },
    Step {
        // The `!Send` port flavour only stays honest if something actually
        // builds for a target where `Send` is unavailable. This is that
        // something, and it runs before any Cloudflare code exists.
        name: "wasm32 build of the contract crate",
        program: "cargo",
        args: &[
            "check",
            "-p",
            "happenstance",
            "--target",
            "wasm32-unknown-unknown",
            "--no-default-features",
            "--features",
            "std",
        ],
        probe: None,
    },
    Step {
        name: "documentation",
        program: "cargo",
        args: &[
            "doc",
            "--workspace",
            "--all-features",
            "--no-deps",
            "--document-private-items",
        ],
        probe: None,
    },
];

const OPTIONAL: &[Step] = &[
    Step {
        // Feature combinations are where multi-crate Rust workspaces rot
        // silently: everything builds with `--all-features` and nothing builds
        // with the combination a user actually picked.
        name: "feature powerset",
        program: "cargo",
        args: &[
            "hack",
            "check",
            "--workspace",
            "--feature-powerset",
            "--no-dev-deps",
        ],
        probe: Some(&["hack", "--version"]),
    },
    Step {
        name: "licences and advisories",
        program: "cargo",
        args: &["deny", "check"],
        probe: Some(&["deny", "--version"]),
    },
];

fn main() -> ExitCode {
    let task = std::env::args().nth(1);

    let result = match task.as_deref() {
        Some("ci") => run_ci(),
        Some("wasm") => run_steps(&REQUIRED[3..4]),
        Some(other) => {
            eprintln!("unknown task: {other}");
            print_help();
            return ExitCode::FAILURE;
        }
        None => {
            print_help();
            return ExitCode::SUCCESS;
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("\nxtask failed: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn print_help() {
    println!("cargo xtask <task>");
    println!();
    println!("Tasks:");
    println!("  ci     Run the full gate: fmt, clippy, tests, wasm32, docs,");
    println!("         plus feature-powerset and cargo-deny when installed.");
    println!("  wasm   Check that happenstance builds for wasm32-unknown-unknown.");
}

fn run_ci() -> Result<()> {
    run_steps(REQUIRED)?;
    run_steps(OPTIONAL)?;
    println!("\nall checks passed");
    Ok(())
}

fn run_steps(steps: &[Step]) -> Result<()> {
    for step in steps {
        println!("\n=== {} ===", step.name);

        if let Some(probe) = step.probe {
            if !is_available(step.program, probe) {
                println!("skipped: `{} {}` is not installed", step.program, probe[0]);
                continue;
            }
        }

        let status = Command::new(step.program)
            .args(step.args)
            .status()
            .with_context(|| format!("failed to launch `{}`", step.program))?;

        if !status.success() {
            bail!("{} failed with {status}", step.name);
        }
    }

    Ok(())
}

/// Whether `program probe...` runs successfully, used to detect an installed
/// cargo subcommand.
fn is_available(program: &str, probe: &[&str]) -> bool {
    Command::new(program)
        .args(probe)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}
