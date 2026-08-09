//! Workspace chores, runnable as `cargo xtask <command>`.
//!
//! The point of the [xtask pattern](https://github.com/matklad/cargo-xtask) is
//! that the CI gate is *one command* defined *once*, in Rust, rather than a
//! list of steps duplicated between a YAML file and everyone's memory. If
//! `cargo xtask ci` passes locally, CI passes.
//!
//! # What the gate proves
//!
//! Mandatory, in order: formatting; clippy over every target and feature with
//! `-D warnings`; the test suite; a `wasm32-unknown-unknown` build of
//! `happenstance-core`, which is the only thing keeping [ADR-0001]'s `!Send`
//! port flavour honest; the documentation, both with every feature and with
//! none, because a broken intra-doc link is a hard rustdoc error and the
//! `no_std` configuration had three of them; `cargo xtask proof-artefact`, which
//! holds the conformance suite to its own proof that it discriminates and the
//! two `wire` targets to the negative controls that make them mean anything;
//! `cargo xtask spec-trace`, which holds the architectural specification to its
//! own cross-references and regenerates its traceability table; five
//! file-reading lints described below; and `cargo xtask package-check`, which
//! asserts the licences and README are actually inside each publishable artifact
//! rather than merely promised by its metadata.
//!
//! # The five lints, and why a grep is in a Rust gate
//!
//! Four clauses of `SPECIFICATION.md` name a `cargo xtask ci` step, and each
//! names one because the thing it checks cannot be expressed as a type: CF-33
//! (no conformance rule reads a clock), CF-6 (no rule asserts a literal
//! sequence-position value), CF-29 (a rule lands with a changelog entry naming
//! the defect it detects) and CF-32 (the testkit carries its own `version` key).
//! The fifth belongs to §7.4 and is not a clause: `spec-trace`'s check 6 is
//! satisfied forever by a `Retires:` line, so a rule the specification says is
//! gone can sit in `suite.rs` failing registered mutants with nothing to notice.
//!
//! Each of the five states, in its own documentation, what it does *not* verify.
//! That is not modesty. A check whose limits are undocumented is read as a
//! guarantee, and the one that would be read hardest is CF-6's: the real
//! enforcement is `GappedPositionStore` in the mutant registry, and the grep is
//! the cheap second line. See `lints` and `spec_trace::retired_rules`.
//!
//! Optional, each behind a probe for the tool it needs: the workspace feature
//! powerset, the same powerset restricted to `wasm32`, `cargo deny`, and a
//! nightly rustdoc build with `--cfg docsrs`. Optional means *skipped when the
//! tool is absent* and never *ignored when it fails*. The gate job installs all
//! of them on every runner — `cargo-hack` and `cargo-deny` as tools, nightly as
//! a second toolchain — so nothing is skipped there. That is a property of
//! `.github/workflows/ci.yml`, and a step removed from it turns this sentence
//! into a lie rather than into a warning.
//!
//! Every cargo invocation that resolves dependencies passes `--locked`. A gate
//! that silently updates `Cargo.lock` is a gate that tested a dependency graph
//! nobody committed.
//!
//! [ADR-0001]: ../../docs/adr/0001-async-port-flavours.md

#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::process::{Command, ExitCode, Stdio};

use anyhow::{Context, Result, bail};

mod lints;
mod package;
mod proof;
mod reserve;
mod spec_trace;

/// A step in the CI gate.
struct Step {
    /// What this proves, shown while it runs.
    name: &'static str,
    /// The program to run.
    program: &'static str,
    /// Its arguments.
    args: &'static [&'static str],
    /// Environment variables to set for this step only.
    ///
    /// Exists for the three rustdoc steps, whose whole input is `RUSTDOCFLAGS`
    /// — rustdoc does not read `RUSTFLAGS`, so `ci.yml`'s ambient `-D warnings`
    /// reaches every rustc invocation in the gate and no rustdoc one. Setting it
    /// in the process environment instead would leak into every other step, and
    /// the obvious workaround (`Command::new("sh")` with a `VAR=x cargo …`
    /// string) is not portable to the Windows this repository is developed on.
    env: &'static [(&'static str, &'static str)],
    /// How to detect whether the tool is installed, for steps that depend on a
    /// cargo subcommand or a toolchain that may be absent.
    ///
    /// `None` means the step is mandatory. `Some(cmd)` — a whole command line,
    /// program first — means: run it, skip the step if it fails, and otherwise
    /// treat the step exactly as mandatory. That distinction matters: "the tool
    /// is missing" must be a skip, but "the tool ran and found a problem" must
    /// be a failure. CI installs every optional tool and the nightly toolchain,
    /// so nothing is skipped there.
    ///
    /// The probe carries its own program rather than borrowing the step's, so
    /// that the skip message can name the exact command that answered and so a
    /// probe is not forced to be an invocation of the thing it is probing.
    probe: Option<&'static [&'static str]>,
}

const REQUIRED: &[Step] = &[
    Step {
        // No `--locked`: `cargo fmt` resolves no dependencies, so the flag would
        // be noise here and nowhere else.
        name: "formatting",
        program: "cargo",
        args: &["fmt", "--all", "--check"],
        env: &[],
        probe: None,
    },
    Step {
        name: "clippy (all targets, all features)",
        program: "cargo",
        args: &[
            "clippy",
            "--locked",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // `--show-output` is not noise. A capability-gated conformance rule that
        // a fixture declines still runs as a test, *passes*, and prints one
        // `SKIP <rule>: …` line naming the fixture's stated reason (CF-18).
        // libtest suppresses a passing test's stdout by default, so without this
        // flag `RuleOutcome::report` writes into a void and the trade an adapter
        // author made by declining a capability leaves no record anywhere.
        //
        // The honest limit of it: this makes the line *reachable* by a human, and
        // nothing here makes anyone read it. The machine-checked half of CF-18 is
        // the `capability_skips_are_reported` meta-test, and that is what fails a
        // build; this only makes the failure legible.
        name: "tests",
        program: "cargo",
        args: &[
            "test",
            "--locked",
            "--workspace",
            "--all-features",
            "--",
            "--show-output",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // Redundant against the step above on a green tree, and that is not what
        // it is for. Each phase's proof artefact — the suite's own
        // `mutation_coverage` (CF-1 – CF-6, ADR-0010), and the two `wire` targets
        // the wire format was frozen against (ADR-0016) — is a single test
        // target, and **nothing else in the gate would notice if one ceased to
        // exist**: `cargo xtask spec-trace` reads only `suite.rs`, so it neither
        // resolves the meta-tests' names nor reports them as orphans, and
        // `cargo test --workspace` passes just as happily with one fewer target
        // as with one more.
        //
        // Naming the target is what closes that. Deleting
        // `tests/mutation_coverage.rs` fails with `no test target named
        // `mutation_coverage``, which is a legible message rather than silence —
        // the same reason `wasm_steps` selects by name after an index-selected
        // step was found pointing at the wrong thing.
        //
        // Naming the target is not sufficient, though, which is why this runs
        // `cargo xtask proof-artefact` rather than `cargo test` directly:
        // `cargo test` exits 0 on `running 0 tests`, so an emptied file passes a
        // step that a deleted one fails. `proof.rs` asserts the named tests
        // out of `--list` first, and states the rest of the argument.
        name: "each phase's proof artefacts",
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "proof-artefact",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // The `!Send` port flavour only stays honest if something actually
        // builds for a target where `Send` is unavailable. This is that
        // something, and it runs before any Cloudflare code exists.
        //
        // Mandatory, and deliberately still a plain `cargo check`. The
        // powerset widening of this same target lives in OPTIONAL, behind the
        // `cargo hack` probe — so if that tool is absent the coverage narrows
        // but the guard on standing constraint 1 does not disappear. A
        // constraint whose only check is skippable is unguarded on every
        // machine that lacks one tool.
        name: "wasm32 build of the contract crate",
        program: "cargo",
        args: &[
            "check",
            "--locked",
            "-p",
            "happenstance-core",
            "--target",
            "wasm32-unknown-unknown",
            "--no-default-features",
            "--features",
            "std",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // CF-23 requires three harnesses in-tree, and the wasm one is the only
        // one no native `cargo test` can reach: `memory_conformance_wasm.rs`
        // and `local_conformance.rs`'s wasm half are both behind
        // `cfg(target_arch = "wasm32")`, so on a native run they compile to
        // nothing at all. Type-checking them here is what stops `__emit_wasm`
        // from rotting into a macro nobody has compiled since the day it was
        // written — the same decorative-check failure the nightly docs step had.
        //
        // `--tests` and not `--all-targets`: the latter would pull in the
        // benchmark and example targets, which have no wasm story and are not
        // what this step is asserting about.
        name: "wasm32 check of the conformance harnesses",
        program: "cargo",
        args: &[
            "check",
            "--locked",
            "-p",
            "happenstance-testkit",
            "--tests",
            "--target",
            "wasm32-unknown-unknown",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // The workspace's only `!Send` adapter and the sole instrument for ES-6.
        // Its own rustdoc claims it compiles for this target; until this step
        // existed nothing checked that, which is the decorative-gate shape this
        // file's module documentation warns about — asserted in prose, guarded by
        // nothing. Phase 9 swaps the stand-in for the real `worker` bindings and
        // this step is what will notice if that stops being true.
        name: "wasm32 build of the Cloudflare adapter",
        program: "cargo",
        args: &[
            "check",
            "--locked",
            "-p",
            "happenstance-cloudflare",
            "--target",
            "wasm32-unknown-unknown",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // Neon is the only crate that must build for *both* targets, so it is
        // checked twice: here for `wasm32`, and by the ordinary workspace steps
        // for the host. Note what the pair does and does not claim — it compiles
        // the *bare* flavour on each target, which is not the same as satisfying
        // both flavours (`store.rs`'s implication table).
        name: "wasm32 build of the Neon adapter",
        program: "cargo",
        args: &[
            "check",
            "--locked",
            "-p",
            "happenstance-neon",
            "--target",
            "wasm32-unknown-unknown",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // `RUSTDOCFLAGS` rather than the ambient `RUSTFLAGS: -D warnings` that
        // `ci.yml` sets, because rustdoc does not read `RUSTFLAGS` — so until
        // this line existed the gate denied every rustc lint and no rustdoc one.
        // It printed "generated 3 warnings" and exited 0 for as long as it ran,
        // which is the decorative-gate shape this repository keeps finding.
        name: "documentation",
        program: "cargo",
        args: &[
            "doc",
            "--locked",
            "--workspace",
            "--all-features",
            "--no-deps",
            "--document-private-items",
        ],
        env: &[("RUSTDOCFLAGS", "-D warnings")],
        probe: None,
    },
    Step {
        // CF-38. The specification claims, for every clause, that some rule can
        // observe a violation and some case exercises it. Those claims were
        // written by hand and nothing checked them until this step existed — its
        // first run found 156 problems, of which the great majority were the
        // checker misreading the document and 24 were real dangling citations.
        // A specification whose cross-references have rotted is worse than one
        // that never made them, because it reads as though it is backed by tests.
        //
        // It also holds §7.1 and §7.2 to what it computes, rather than merely
        // being able to regenerate them. See `spec_trace`'s module docs for why
        // that equality, and not the generator, is the part that matters.
        name: "specification traceability",
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "spec-trace",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // §7.4. `spec-trace`'s check 6 accepts a rule that a clause *disposes
        // of* via `Retires:`, which means a disposition satisfies it forever —
        // the document can say a rule is retired while the rule sits in
        // `suite.rs` failing registered mutants, and the checker stays silent.
        // Phase 3 did exactly that three times and reversed all three. Its own
        // step rather than a tenth check inside `spec-trace` because the two
        // answer different questions: `spec-trace` asks whether the document is
        // internally consistent, this asks whether a decision the document
        // records has actually been carried out. It resolves rule names across
        // all three files rules live in, and it parses a fixture clause of its
        // own on every run — there are no `Retires:` fields in the document
        // today, so nothing else would notice if the field stopped parsing.
        name: "no retired rule is still live",
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "lint-retired-rules",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // CF-33, and the clause specifies the shape: a grep, not a type. There
        // is no lint that expresses "this crate may not observe time". See
        // `lints::no_clock` for why it is scoped to `src/` and why prose about
        // a clock does not trip it.
        name: "no conformance rule reads a clock",
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "lint-clock",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // CF-6's cheap second line. `GappedPositionStore` is the enforcement and
        // it runs in the step above this file's `proof-artefact` entry; this
        // catches the habit at the spelling, which is the half that names the
        // rule rather than the store.
        name: "no literal position values in the suite",
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "lint-position-literals",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // CF-29's changelog half. Its first run found twenty-five of fifty-five
        // rules with no entry — including every one of the founding rules, which
        // had never been named anywhere a consumer reads.
        //
        // Twenty-five was an undercount, and both halves of the reason are worth
        // knowing. It read `suite.rs` alone, so the model and concurrency
        // families were invisible to a step whose name claims *every* rule; and
        // it matched a rule name as a bare substring, so a rule that is a prefix
        // of a longer one was discharged by the longer one's entry.
        // `append_is_atomic` and `positions_are_unique` were both passing on a
        // collision and neither had an entry. See `lints::names_rule`.
        name: "every conformance rule has a changelog entry",
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "lint-changelog",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // CF-32. A manifest check, and the cheapest step in the gate: the
        // testkit's version must not be `version.workspace = true`, because
        // adding a rule is semver-MINOR for the bar and nothing at all for the
        // contract.
        name: "the testkit carries its own version",
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "lint-testkit-version",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // The step above passes with every feature on, which is the one
        // configuration where every intra-doc link resolves. Three links to
        // `MemoryEventStore` were broken without `memory` for as long as this
        // gate existed, because nothing ever built the docs without it — and
        // rustdoc treats a broken intra-doc link as a hard error, so
        // `cargo doc --no-default-features` did not merely warn, it failed.
        // A `no_std` consumer would have hit it on their first build. (D13)
        name: "documentation (no default features)",
        program: "cargo",
        args: &[
            "doc",
            "--locked",
            "-p",
            "happenstance-core",
            "--no-default-features",
            "--no-deps",
        ],
        env: &[("RUSTDOCFLAGS", "-D warnings")],
        probe: None,
    },
    Step {
        // D11. Manifest metadata promising two licences is not the same thing as
        // an artifact containing them, and only the second is what a consumer
        // unpacks. See `package`'s module docs.
        name: "packaged artifacts carry their licences and README",
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "package-check",
        ],
        env: &[],
        probe: None,
    },
];

const OPTIONAL: &[Step] = &[
    Step {
        // Feature combinations are where multi-crate Rust workspaces rot
        // silently: everything builds with `--all-features` and nothing builds
        // with the combination a user actually picked.
        //
        // No `--locked` on either `cargo hack` step, and not by oversight:
        // `--no-dev-deps` works by rewriting each manifest with its
        // dev-dependencies removed, which changes the dependency graph and so
        // must rewrite the lock file. The two flags are mutually exclusive by
        // construction, and cargo says so rather than ignoring one.
        name: "feature powerset",
        program: "cargo",
        args: &[
            "hack",
            "check",
            "--workspace",
            "--feature-powerset",
            "--no-dev-deps",
        ],
        env: &[],
        probe: Some(&["cargo", "hack", "--version"]),
    },
    Step {
        // The mandatory wasm32 step above checks one feature combination. This
        // checks all of them on the same target, because `no_std` and `wasm32`
        // fail in different combinations: a `std`-only import guarded by the
        // wrong `cfg` compiles fine under `--features std` and not at all
        // without it. (D13)
        name: "wasm32 feature powerset",
        program: "cargo",
        args: &[
            "hack",
            "check",
            "-p",
            "happenstance-core",
            // `happenstance-neon` is the only *adapter* with features that must
            // hold on this target; the Cloudflare crate has none, so the
            // mandatory plain check above is already its powerset.
            "-p",
            "happenstance-neon",
            // Added with the fixture contract, and it is a real check rather
            // than symmetry: the testkit gained a `proptest` feature, and a
            // **feature is not target-scoped**. `--all-features` therefore sets
            // `feature = "proptest"` on wasm32, where the optional dependency
            // lives in a `cfg(not(target_arch = "wasm32"))` table and resolves
            // to nothing. Only the powerset on *this* target compiles that
            // combination; `fixtures::strategies` carries the second `cfg`
            // condition that makes it build, and this step is what would notice
            // if it were dropped.
            "-p",
            "happenstance-testkit",
            "--target",
            "wasm32-unknown-unknown",
            "--feature-powerset",
            "--no-dev-deps",
        ],
        env: &[],
        probe: Some(&["cargo", "hack", "--version"]),
    },
    Step {
        name: "licences and advisories",
        program: "cargo",
        args: &["deny", "check"],
        env: &[],
        probe: Some(&["cargo", "deny", "--version"]),
    },
    Step {
        // `docsrs` is a cfg nobody sets except docs.rs, which builds *after*
        // publication — so a `#[cfg_attr(docsrs, doc(cfg(…)))]` that does not
        // compile fails where it can no longer be fixed, on a version that can
        // be yanked but never removed. `doc_cfg` is an unstable feature and has
        // changed spelling before, which makes this exactly the kind of thing
        // that breaks quietly.
        //
        // Nightly-only, therefore probed rather than mandatory: the MSRV is
        // 1.85 stable and no contributor should need a second toolchain to run
        // the gate. The gate job in `.github/workflows/ci.yml` installs nightly
        // on all three runners, so the check is real there. (D13)
        //
        // Named crate by crate rather than `--workspace`, because a skeleton's
        // `todo!()` bodies have nothing to say to docs.rs. `happenstance-testkit`
        // joined the list when it acquired `#![cfg_attr(docsrs, feature(doc_cfg))]`
        // and a `doc(cfg(feature = "proptest"))`: it is one of the three
        // publishable crates, so its rendering fails in the same
        // yankable-but-not-removable place.
        name: "docs.rs configuration (nightly)",
        program: "cargo",
        args: &[
            "+nightly",
            "doc",
            "--locked",
            "-p",
            "happenstance-core",
            "-p",
            "happenstance-testkit",
            "--all-features",
            "--no-deps",
        ],
        env: &[("RUSTDOCFLAGS", "--cfg docsrs -D warnings")],
        probe: Some(&["cargo", "+nightly", "--version"]),
    },
];

fn main() -> ExitCode {
    let task = std::env::args().nth(1);

    let result = match task.as_deref() {
        Some("ci") => run_ci(),
        Some("wasm") => run_steps(wasm_steps()),
        Some("reserve") => reserve::run(std::env::args().nth(2).as_deref()),
        Some("spec-trace") => match std::env::args().nth(2).as_deref() {
            None => spec_trace::run(spec_trace::Mode::Check),
            Some("--write") => spec_trace::run(spec_trace::Mode::Write),
            Some(flag) => {
                eprintln!("unknown flag for spec-trace: {flag}");
                print_help();
                return ExitCode::FAILURE;
            }
        },
        Some("package-check") => package::run(),
        Some("proof-artefact") => proof::run(),
        Some("lints") => run_steps(lint_steps()),
        Some("lint-clock") => lints::no_clock(),
        Some("lint-testkit-version") => lints::testkit_version(),
        Some("lint-changelog") => lints::changelog_names_every_rule(),
        Some("lint-position-literals") => lints::no_position_literals(),
        Some("lint-retired-rules") => spec_trace::retired_rules(),
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
    println!("  ci     Run the full gate: fmt, clippy, tests, wasm32, docs with and");
    println!("         without default features, spec-trace, package-check — then, when");
    println!("         the tool is installed, the workspace and wasm32 feature powersets,");
    println!("         cargo-deny, and a nightly `--cfg docsrs` rustdoc build.");
    println!("  wasm   Check that happenstance-core, the conformance harnesses and the");
    println!("         two wasm32 adapters (cloudflare, neon) build for");
    println!("         wasm32-unknown-unknown.");
    println!("  spec-trace [--write]");
    println!("         Check the specification's clauses against the suite and the e2e");
    println!("         cases: markers, falsifiers, rule names, case numbers, citations.");
    println!("         Also compares SPECIFICATION.md's generated §7.1-§7.2 region against");
    println!("         what the checker computes, and fails when they differ. --write");
    println!("         rewrites that region; §7.3 onward is authored and never touched.");
    println!("  lints  Run just the five file-reading checks four clauses name as gate");
    println!("         steps: CF-33 (no clock in the suite), CF-6 (no literal position");
    println!("         values), CF-29 (a changelog entry per rule), CF-32 (the testkit's");
    println!("         own version key), and §7.4's disposed-rule check. Each is also");
    println!("         available on its own as lint-clock, lint-position-literals,");
    println!("         lint-changelog, lint-testkit-version and lint-retired-rules.");
    println!("  package-check");
    println!("         Assert that `cargo package --list` shows LICENSE-MIT, LICENSE-APACHE");
    println!("         and README.md inside each publishable crate's artifact.");
    println!("  proof-artefact");
    println!("         Assert that each phase's proof artefact still holds the tests its");
    println!(
        "         clauses name — {} across {} targets — then run them. `cargo test` exits 0",
        proof::ARTEFACTS
            .iter()
            .map(|a| a.tests.len())
            .sum::<usize>(),
        proof::ARTEFACTS.len()
    );
    println!("         on an empty target, so the names are checked out of `--list` first.");
    println!("  reserve <name>");
    println!("         Generate the 0.0.0 placeholder for a crates.io name. Prints the");
    println!("         publish command; never publishes anything itself.");
}

/// The `wasm32` steps, selected by name.
///
/// It used to be `&REQUIRED[3..4]`. An index is silent about what it selects, so
/// inserting a step above it would have pointed `cargo xtask wasm` at clippy and
/// left the one check that guards [ADR-0001]'s `!Send` design running nothing —
/// while still printing green. Panicking here is the right failure: the steps are
/// compile-time constants, so a miss is a bug in this file and never a user error.
///
/// Two steps rather than one since phase 1: the contract crate proves the *port*
/// compiles without `Send`, and the testkit's harnesses prove the *suite* does.
/// The second is not implied by the first — `#[tokio::test]` type-checks on
/// wasm32 and then cannot run there, which is a failure no `cargo check` of
/// `happenstance-core` can see.
///
/// [ADR-0001]: ../../docs/adr/0001-async-port-flavours.md
fn wasm_steps() -> Vec<&'static Step> {
    steps_named(&[
        "wasm32 build of the contract crate",
        "wasm32 check of the conformance harnesses",
        "wasm32 build of the Cloudflare adapter",
        "wasm32 build of the Neon adapter",
    ])
}

/// The five checks phase 3 stage 6 added, selected by name.
///
/// They are ordinary `REQUIRED` steps and `cargo xtask ci` runs them like any
/// other; this exists so that working on one does not mean running the whole
/// gate to see it. Each is a file read and a string match, so the whole set
/// finishes in the time it takes cargo to decide `xtask` is up to date.
fn lint_steps() -> Vec<&'static Step> {
    steps_named(&[
        "no retired rule is still live",
        "no conformance rule reads a clock",
        "no literal position values in the suite",
        "every conformance rule has a changelog entry",
        "the testkit carries its own version",
    ])
}

/// The named steps, in the order given.
///
/// Panicking is the right failure: the steps are compile-time constants, so a
/// name that resolves to nothing is a bug in this file and never a user error.
/// A `find` returning `None` silently would reproduce the index-selection defect
/// this exists to replace, one level further in.
fn steps_named(names: &[&str]) -> Vec<&'static Step> {
    names
        .iter()
        .map(|name| {
            REQUIRED
                .iter()
                .find(|step| step.name == *name)
                .unwrap_or_else(|| panic!("REQUIRED must contain the `{name}` step"))
        })
        .collect()
}

fn run_ci() -> Result<()> {
    run_steps(REQUIRED)?;
    run_steps(OPTIONAL)?;
    println!("\nall checks passed");
    Ok(())
}

fn run_steps<'a>(steps: impl IntoIterator<Item = &'a Step>) -> Result<()> {
    for step in steps {
        println!("\n=== {} ===", step.name);

        // A let-chain, and the first in the workspace. It is here because
        // ADR-0029 raised the MSRV to 1.97.1 and clippy noticed within the hour:
        // `collapsible_if` had been suppressed by `clippy.toml`'s `msrv = 1.85`
        // for as long as this function existed, and raising the floor turned the
        // lint on rather than off. Worth a comment once, so the next person to
        // meet a let-chain here knows it is deliberate and not a slip past a
        // constraint that used to be real.
        if let Some(probe) = step.probe
            && !is_available(probe)
        {
            println!("skipped: `{}` did not succeed", probe.join(" "));
            continue;
        }

        let status = Command::new(step.program)
            .args(step.args)
            .envs(step.env.iter().copied())
            .status()
            .with_context(|| format!("failed to launch `{}`", step.program))?;

        if !status.success() {
            bail!("{} failed with {status}", step.name);
        }
    }

    Ok(())
}

/// Whether a probe command runs successfully, used to detect an installed cargo
/// subcommand or toolchain. `probe[0]` is the program.
///
/// A probe that cannot even be launched is a `false` rather than an error: a
/// `cargo` that is not on the path is the same answer as a `cargo` without the
/// subcommand.
///
/// `RUSTUP_AUTO_INSTALL=0` is what makes `cargo +nightly --version` a *probe*.
/// Without it rustup treats `+nightly` on an uninstalled toolchain as a request
/// to fetch one, so the probe always succeeds — after a silent multi-hundred-
/// megabyte download on a contributor's first `cargo xtask ci`. `rustup which`
/// behaves identically; there is no read-only spelling of the `+toolchain`
/// shorthand, only this variable.
fn is_available(probe: &[&str]) -> bool {
    let Some((program, args)) = probe.split_first() else {
        return false;
    };
    Command::new(program)
        .args(args)
        .env("RUSTUP_AUTO_INSTALL", "0")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}
