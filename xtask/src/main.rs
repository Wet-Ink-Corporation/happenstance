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
//! `-D warnings`; the test suite; five `wasm32-unknown-unknown` builds, of which
//! `happenstance-core`'s is what keeps [ADR-0001]'s `!Send` port flavour honest
//! and the typed layer's is what compiles the crate a Workers application
//! actually installs — and none of which says anything about *which* flavour the
//! code bound, because `Send` is available on that target and only a `Send`
//! store is not (`crates/happenstance/tests/flavours.rs` is that instrument);
//! two further `wasm32` rows that are a different claim from those five, because
//! they **execute** rather than compile — the conformance targets are held to
//! their own rule enumerations by a step nothing can skip, and then the rules are
//! run on the target under `wasm-bindgen-test-runner`, which is where CF-23
//! stops being a compile (see `proof::wasm_run` for why the run is probed and
//! the guard beside it is not);
//! the documentation, both with every feature and with
//! none, because a broken intra-doc link is a hard rustdoc error and the
//! `no_std` configuration had three of them; `cargo xtask proof-artefact`, which
//! holds the conformance suite to its own proof that it discriminates and the
//! two `wire` targets to the negative controls that make them mean anything;
//! `cargo xtask spec-trace`, which holds the architectural specification to its
//! own cross-references and regenerates its traceability table; six
//! file-reading lints described below; a seventh manifest lint for D12, outside
//! that clause-named group (ADR-0016 §14) and inside `cargo xtask lints`; and
//! `cargo xtask package-check`, which asserts the licences and README are
//! actually inside each publishable artifact rather than merely promised by
//! its metadata.
//!
//! # The six lints, and why a grep is in a Rust gate
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
//! The sixth is not a clause's either, and it runs in the opposite direction to
//! every other step here: `lints::stated_rule_counts` holds four *documents* to
//! the code. `package-check` proves the testkit's README is inside the published
//! artifact; nothing proved it was true, and it told crates.io the projection
//! suite was two rules of seventeen through the fifteen commits that made it
//! seventeen — as did the crate page, the feature list and the manifest comment
//! beside the feature, in three other file formats.
//!
//! Each of the six states, in its own documentation, what it does *not* verify.
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
//! [ADR-0001]: ../../.kb/decisions/0001-async-port-flavours.md

#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::process::{Command, ExitCode, Stdio};

use anyhow::{Context, Result, bail};

mod affected;
mod lint_constitution;
mod lint_narrative;
mod lint_pages;
mod lints;
mod narrative_doctests;
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
    /// — rustdoc does not read `RUSTFLAGS`, so the ambient `-D warnings`
    /// reaches every rustc invocation in the gate and no rustdoc one. Setting
    /// it in the process environment instead would leak into every other step,
    /// and the obvious workaround (`Command::new("sh")` with a `VAR=x cargo …`
    /// string) is not portable to the Windows this repository is developed on.
    ///
    /// *"The ambient `-D warnings`"* used to mean `ci.yml`'s job `env:` alone,
    /// which made the sentence true on a runner and false on a developer's
    /// machine — nothing set it locally. `.cargo/config.toml`'s `[build]
    /// rustflags` now does, so it is true in both places and this field stays
    /// what its name says.
    ///
    /// `RUSTFLAGS` is deliberately **not** set here per step, and the attempt is
    /// worth recording because it looks obviously right: `cargo xtask` is an
    /// alias for `cargo run -p xtask`, so flags applied only to the steps'
    /// children stop matching the fingerprint of the running `xtask.exe`, and
    /// the `proof-artefact` step's own `cargo run -p xtask` then tries to
    /// relink the executable it is running inside — `failed to remove file
    /// target\debug\xtask.exe: Access is denied`. CI never meets that because
    /// its variable is ambient before cargo builds xtask at all.
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
        // nothing. Phase 9 swapped the stand-in for the real `worker` bindings
        // and this step is what notices if that stops being true.
        //
        // `--tests`, and it is not tidiness. The adapter's `!Send` probes gained
        // a `wasm32` twin when `worker` landed, because the auto-trait leak they
        // exist to catch is written `#[cfg(not(target_feature = "atomics"))]` in
        // `wasm-bindgen` and can therefore only be observed on the target it is
        // compiled for. Without this flag `#[cfg(test)]` code is not compiled at
        // all, and the twin would have been built by nothing in this gate on the
        // day it merged — a decoration rather than a detector. `--tests` and not
        // `--all-targets` for the reason the sibling step at `:219-244` gives:
        // `--all-targets` reaches benchmark and example targets that have no
        // wasm story.
        //
        // Compiling is not executing, and for a while that gap was the whole
        // story here: this step built the adapter's eighty-one wasm32 cases and
        // nothing in the gate ran one. It does now — `proof::WASM_UNIT_TARGETS`
        // carries a `--lib` row for this package and the `wasm32 run of the
        // conformance rules` step below executes it. What is still only
        // *compiled* is the conformance suite against a `CloudflareFixture`,
        // which arrives when `every-rule-under-workerd` registers a
        // `WASM_TARGETS` row for it.
        //
        // The **name** is unchanged, deliberately: `wasm_steps()` selects by
        // name, and an index-selected step once pointed `cargo xtask wasm` at
        // clippy while printing green.
        name: "wasm32 build of the Cloudflare adapter",
        program: "cargo",
        args: &[
            "check",
            "--locked",
            "-p",
            "happenstance-cloudflare",
            "--tests",
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
        // What it buys: `happenstance` is the crate a Workers application
        // `cargo add`s — ADR-0006 gave it the bare name — and until this step
        // existed the gate compiled the contract, the harnesses and two adapters
        // for this target and never the crate a consumer actually installs. The
        // four steps above were satisfiable with the typed layer never built for
        // the edge at all, which is the silence this closes.
        //
        // What it does NOT buy, and the distinction is the whole reason this
        // comment is longer than the argument list: `Send` is a perfectly
        // available auto trait on `wasm32-unknown-unknown`. What is unavailable
        // there is a `Send` *store*. So a green check here proves dependency,
        // `std` and feature hygiene, and proves nothing at all about whether
        // this crate's generic code bound `EventStore` or `SendEventStore` — a
        // generic body type-checks against its declared bounds whether or not
        // anything instantiates it. The instrument for that half is
        // `crates/happenstance/tests/flavours.rs`, which instantiates every
        // entry point against a store that is genuinely `!Send`.
        //
        // `std,json` and not more: that is the combination the design fixed, and
        // the rest of the feature space is compiled on this target by the
        // `wasm32 feature powerset` step in OPTIONAL. Widening this one to chase
        // `unstable-projection` would put a combination where a point belongs.
        name: "wasm32 build of the typed layer",
        program: "cargo",
        args: &[
            "check",
            "--locked",
            "-p",
            "happenstance",
            "--target",
            "wasm32-unknown-unknown",
            "--no-default-features",
            "--features",
            "std,json",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // Mandatory, and it is the half that makes the step below allowed to
        // carry a probe at all. It needs no runner: it reads the executed
        // targets' own sources and the enumeration each is held to, so an emptied
        // target, one rewired to `__emit_tokio`, one carrying a hand-written
        // wasm32-only rule list, or a row deleted outright fails here on every
        // machine — including the machines where the run below prints
        // `skipped:`. Without this row the pair would be a bare probe-gated
        // step, which is the one configuration the project's AC-004 forbids.
        //
        // It also holds `WASM_TARGETS` to the *directory*: a harness in
        // `happenstance-testkit/tests` that drives a suite through a wasm32
        // emitter and has no row fails here. That is the check the first cut of
        // this seam did not have, and it is what the retirement note in
        // `ci.yml` now rests on — the job it replaced ran the whole package and
        // named no target, so a list that names them can fall a row behind it
        // silently, and did.
        //
        // Its stated limit: it cannot say the rules *passed*, and it cannot see
        // an `#[ignore]` on a macro-generated test. Both need the runner, and
        // both are the step below's.
        name: "wasm32 conformance targets are non-vacuous",
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "wasm-conformance-enumeration",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // The step that makes the five above mean what a reader assumes they
        // mean. They *compile* the wasm32 harnesses; nothing in this gate ever
        // executed a conformance rule on the target the two-flavour port design
        // exists for, and `#[tokio::test]` type-checks for wasm32 and then
        // cannot run there — precisely the failure CF-23 is about. Until this
        // row landed, the only place a rule actually ran on wasm32 was a
        // GitHub Actions job on one of the three runners the gate matrices
        // over, which is a claim about CI rather than about the gate.
        //
        // It runs two registries, and the name under-claims rather than over-
        // claims: `proof::WASM_TARGETS` is the conformance harnesses, and
        // `proof::WASM_UNIT_TARGETS` is every other wasm32 test target — today
        // the Cloudflare adapter's own `--lib` cases, which the step above had
        // been compiling and nothing had been running. The name is left alone
        // because `wasm_steps()` selects it by string, and an index-selected
        // step once pointed `cargo xtask wasm` at clippy while printing green.
        //
        // `--nocapture` reaches the runner through `proof.rs`, and it is load-
        // bearing rather than verbose: `println!` is a silent discard on
        // `wasm32-unknown-unknown`, so a declined capability's
        // `SKIP <rule>: <reason>` line is written through `console_log!` and
        // swallowed unless the runner is told not to capture. It is this
        // target's idiom for the `--show-output` the `tests` step above
        // carries — and not a synonym for it: this runner rejects
        // `--show-output` outright.
        //
        // PROBED, and the argument is owed because a probe is normally the
        // weaker choice. The rule this file states at `:192-202` — a constraint
        // whose only check is skippable is unguarded on every machine that
        // lacks one tool — is honoured by the mandatory row above, not waived
        // here. What decides the shape is which class of tool this is. Every
        // mandatory step in this array needs only what `rust-toolchain.toml`
        // pins, and that file pins the `wasm32-unknown-unknown` *target*;
        // `wasm-bindgen-cli` is a separately installed binary that must match
        // the `wasm-bindgen` schema version in `Cargo.lock` exactly, so a
        // `cargo update` can invalidate an installed one. rustup cannot supply
        // it, which puts it in the same class as `cargo hack` and nightly and
        // not in the same class as a target.
        //
        // The rejected alternative, stated because it was close: shape (i), a
        // mandatory row with `probe: None`. It buys a stronger local guarantee
        // and costs every contributor a `cargo install wasm-bindgen-cli` plus a
        // node host before `cargo xtask ci --fast` can pass at all — a bar this
        // project's own eleven remaining stories would pay at every seam, and
        // one that breaks on a dependency bump rather than on a code change. It
        // was declined because the row above recovers the part of it that is
        // about *this repository's* code, and CI recovers the rest: the gate
        // job installs the runner on all three matrix runners, so nothing is
        // skipped where the claim is made. That last sentence is a property of
        // `.github/workflows/ci.yml`, and removing the install there turns it
        // into a lie rather than into a warning.
        name: "wasm32 run of the conformance rules",
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "wasm-conformance",
        ],
        env: &[],
        probe: Some(&[proof::WASM_RUNNER, "--version"]),
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
        // The only step that reads a document for content rather than reading
        // code against one. `package-check` proves `README.md` is inside the
        // published artifact (D11); nothing proved it was true, and the README
        // told crates.io the projection suite was "two rules of seventeen"
        // through the fifteen commits that made it seventeen. Three audits
        // raised it as a finding. A finding raised three times is a missing
        // check, so this is the check.
        name: "every stated rule count matches the suite",
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "lint-rule-counts",
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
        // D12. A manifest check, the same shape as CF-32 above: `serde/alloc`
        // and `base64/alloc` are stated in `happenstance-core`'s `serde`
        // feature line rather than left to arrive by accident through a
        // dependency's own default. Not reproducible at HEAD — deleting
        // either token still compiles today, because `bytes` 1.12.1 happens
        // to enable `alloc` for its own optional `serde` dependency — which
        // is exactly why this is a gate step and not a conformance rule
        // (ADR-0016 §14): a behavioural check would be decorative here.
        name: "happenstance-core names serde/alloc and base64/alloc",
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "lint-core-alloc-features",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // The Rust constitution claims its examples compile and its citations
        // resolve. This step discharges the citations and the corpus's own
        // shape; the step below it discharges the examples.
        name: "the Rust constitution is internally consistent",
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "lint-constitution",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // The narrative tree's examples, under their own banner. Two steps
        // rather than one, and this one *first*, because the step below is
        // unfiltered and therefore compiles these pages too: `run_steps` bails
        // at the first failure, so the ordering is the whole of what keeps a
        // broken narrative fence attributed to the narrative corpus. Fixing it
        // from the other side — filtering the step below — would drop the
        // repository README's doctest out of the gate, because its test name
        // carries neither corpus's module path.
        //
        // It runs through `cargo run -p xtask` rather than invoking `cargo test`
        // directly for the reason `proof-artefact` does: a filtered
        // `cargo test --doc` exits 0 over `running 0 tests`, so the subcommand
        // asserts the doctests out of `--list` before running them. The
        // `RUSTDOCFLAGS` below is inherited by the cargo it spawns.
        name: narrative_doctests::STEP,
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "narrative-doctests",
        ],
        env: &[("RUSTDOCFLAGS", "-D warnings")],
        probe: None,
    },
    Step {
        // Its own step rather than a line in `tests`, for the reason
        // `proof-artefact` has one: the workspace test step passes just as
        // happily with one fewer doctest as with one more, so an atom whose
        // examples quietly stopped being compiled would not show up there. And
        // `RUSTDOCFLAGS` is the only way `-D warnings` reaches rustdoc — clippy
        // does not lint doctests at all, so this is the whole of what the
        // constitution's examples are held to.
        name: "the constitution's examples compile",
        program: "cargo",
        args: &["test", "--locked", "-p", "xtask", "--doc"],
        env: &[("RUSTDOCFLAGS", "-D warnings")],
        probe: None,
    },
    Step {
        // Compile, then check: the tree's second banner. The step above hands
        // every registered page to rustdoc; this one reads the tree and the
        // harness as *files* and answers what `cfg(doctest)` hides — a page
        // nobody registered, a registration nobody deleted, a tree someone
        // moved, and a tree someone emptied. Each of those is green under every
        // other step in this array.
        //
        // It sits after `the constitution's examples compile` rather than
        // between the two compile steps, because those two are adjacent on
        // purpose and `narrative_doctests`' own test pins the adjacency: the
        // constitution's step is unfiltered and compiles the narrative pages
        // too, so their order is the whole of what keeps a broken narrative
        // fence under the narrative banner. Compile-then-check survives the
        // move; the compile pair's adjacency would not.
        name: lint_narrative::STEP,
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "narrative",
        ],
        env: &[],
        probe: None,
    },
    Step {
        // The page-need discipline, read as files: `standards/pages/` — the
        // rules — and the pinned narrative tree — the pages the rules govern.
        // It sits directly after the step above because the two read the same
        // tree from opposite sides: that one asks whether a page is registered
        // and compiled, this one asks whether it says which reader's question
        // it answers.
        //
        // `probe: None`, and that is not a formality. This is a directory read
        // with no external tool and no compilation, so a probe would be a lie
        // in the shape RS-80-2 names (`standards/rust/80-the-gate.md:98`): a
        // step that can only be skipped for a reason that cannot occur.
        name: lint_pages::STEP,
        program: "cargo",
        args: &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "xtask",
            "--",
            "lint-pages",
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
        // The configuration the two steps above cannot see, and the one every
        // consumer who types `cargo add happenstance-core` builds: `memory` on,
        // `conformance` off. `--all-features` resolves a link into a
        // `cfg`-gated item because the gate is open; `--no-default-features`
        // never renders the page that carries the link. So a link from
        // `MemoryProjectionStore`'s page to `ProjectionProbe` — a `conformance`
        // item — was a hard error on the default feature set and green on both
        // sides of it. That is the same D13 defect one axis over, and the fix
        // for the defect is not a gate step; this is.
        //
        // `--document-private-items` for parity with the workspace step above:
        // `rustdoc::redundant_explicit_links` fires only when both ends of a
        // link are documented destinations, which private items are what makes
        // true — so without the flag this step would render the right feature
        // set through a narrower lint surface than the gate already runs.
        name: "documentation (default features)",
        program: "cargo",
        args: &[
            "doc",
            "--locked",
            "-p",
            "happenstance-core",
            "--no-deps",
            "--document-private-items",
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
            // The typed layer, and it is where the *most* of this step's value
            // now sits: the mandatory step above compiles one point of its
            // feature space, `std,json`, so it compiles neither the projection
            // runner (off by default) nor `postcard` nor `cbor` for this target.
            // A feature is not target-scoped, so those combinations are checked
            // here or nowhere. This is a widening **above** the mandatory guard
            // and never a replacement for it — the probe below is why.
            "-p",
            "happenstance",
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
            "happenstance",
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
        Some("ci") => match std::env::args().nth(2).as_deref() {
            None => run_ci(),
            Some("--fast") => run_fast(),
            Some(flag) => {
                eprintln!("unknown flag for ci: {flag}");
                print_help();
                return ExitCode::FAILURE;
            }
        },
        Some("wasm") => run_steps(wasm_steps()),
        Some("affected") => match (
            std::env::args().nth(2).as_deref(),
            std::env::args().nth(3).as_deref(),
        ) {
            (None, _) => affected::run(None),
            (Some("--base"), Some(base)) => affected::run(Some(base)),
            (Some("--base"), None) => {
                eprintln!("--base needs a ref");
                print_help();
                return ExitCode::FAILURE;
            }
            (Some(flag), _) => {
                eprintln!("unknown flag for affected: {flag}");
                print_help();
                return ExitCode::FAILURE;
            }
        },
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
        Some("wasm-conformance") => proof::wasm_run(),
        Some("wasm-conformance-enumeration") => proof::wasm_enumeration(),
        Some("narrative-doctests") => narrative_doctests::run(),
        // Read-only, and no `--write` arm: unlike `lint-constitution` there is
        // nothing here a checker could rewrite, so the surface is safe to invoke
        // at any time and nothing needs undoing.
        Some("narrative") => lint_narrative::run(),
        Some("lints") => run_steps(lint_steps()),
        Some("lint-clock") => lints::no_clock(),
        Some("lint-testkit-version") => lints::testkit_version(),
        Some("lint-core-alloc-features") => lints::core_alloc_features(),
        Some("lint-changelog") => lints::changelog_names_every_rule(),
        Some("lint-position-literals") => lints::no_position_literals(),
        Some("lint-rule-counts") => lints::stated_rule_counts(),
        Some("lint-retired-rules") => spec_trace::retired_rules(),
        Some("lint-pages") => match std::env::args().nth(2).as_deref() {
            None => lint_pages::run(lint_pages::Mode::Check),
            Some("--write") => lint_pages::run(lint_pages::Mode::Write),
            Some(flag) => {
                eprintln!("unknown flag for lint-pages: {flag}");
                print_help();
                return ExitCode::FAILURE;
            }
        },
        Some("lint-constitution") => match std::env::args().nth(2).as_deref() {
            None => lint_constitution::run(lint_constitution::Mode::Check),
            Some("--write") => lint_constitution::run(lint_constitution::Mode::Write),
            Some(flag) => {
                eprintln!("unknown flag for lint-constitution: {flag}");
                print_help();
                return ExitCode::FAILURE;
            }
        },
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
    println!("  ci [--fast]");
    println!("         Run the full gate: fmt, clippy, tests, wasm32, docs with and");
    println!("         without default features, spec-trace, package-check — then, when");
    println!("         the tool is installed, the workspace and wasm32 feature powersets,");
    println!("         cargo-deny, and a nightly `--cfg docsrs` rustdoc build. --fast runs");
    println!("         the mandatory steps only, dropping that last group; it is the bar a");
    println!("         non-terminal project's integration gate runs, never the release bar.");
    println!("  affected [--base <ref>]");
    println!("         The story-grain gate: unconditionally, every check the whole gate");
    println!("         reads a document for, bar lint-constitution — affected's own docs");
    println!("         argue that one. Then fmt, clippy and tests for the packages this diff");
    println!("         could have broken and everything depending on them. Base defaults to");
    println!("         `main`, and it errs toward more packages — the module docs say why.");
    println!("  wasm   The whole wasm32-unknown-unknown family. Five builds — happenstance-core,");
    println!("         the conformance harnesses, the two wasm32 adapters (cloudflare, neon)");
    println!("         and the typed layer (happenstance, the crate a Workers application");
    println!("         installs) — none of which says which port flavour the code bound,");
    println!("         because Send exists on that target. Then two rows that are a");
    println!("         different claim: every wasm32-capable conformance target is held");
    println!("         to its own rule enumeration, and the rules are EXECUTED on the");
    println!("         target under wasm-bindgen-test-runner. Also available on their own");
    println!("         as wasm-conformance-enumeration and wasm-conformance.");
    println!("  spec-trace [--write]");
    println!("         Check the specification's clauses against the suite and the e2e");
    println!("         cases: markers, falsifiers, rule names, case numbers, citations.");
    println!("         Also compares SPECIFICATION.md's generated §7.1-§7.2 region against");
    println!("         what the checker computes, and fails when they differ. --write");
    println!("         rewrites that region; §7.3 onward is authored and never touched.");
    println!("  lints  Run just the file-reading checks — every step that reads a document");
    println!("         rather than compiling a package. The rows below are printed from the");
    println!("         selection this command runs; each is also a subcommand of its own:");
    for step in lint_steps() {
        let subcommand = step.args.last().copied().unwrap_or(step.name);
        println!("           {subcommand:<25}{}", step.name);
    }
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
    println!("  narrative-doctests");
    println!("         Compile every Rust example in docs/, the narrative tree, as doctests");
    println!("         of xtask's lib target. Same argument as above, one corpus further on:");
    println!("         a filtered `cargo test --doc` exits 0 over `running 0 tests`, so the");
    println!("         pages are asserted out of `--list` and counted before they are run.");
    println!("  narrative");
    println!("         Check docs/, the narrative tree, as files: the tree is where this");
    println!("         gate pins it and is not empty, every page is registered in");
    println!("         xtask/src/narrative.rs, every registration names a page that still");
    println!("         exists, and no page path is long enough to starve the report.");
    println!("  lint-pages [--write]");
    println!("         Check the page-need discipline: every governed page in the narrative");
    println!("         tree declares exactly one need from the closed set, at a line number,");
    println!("         and standards/pages/ — the rules that say so — keeps its own shape,");
    println!("         its ceilings and a router index generated from the atoms. --write");
    println!("         rewrites that index; nothing else in the tree is ever written.");
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
/// Five since phase 7, and the fifth is appended rather than interleaved: the
/// four above are the standing guard on [ADR-0001] and a reader who knows this
/// transcript should still recognise it. The typed layer is last because it is
/// the crate furthest from the port — and because it is the one a Workers
/// application installs, which is the gap the other four left open.
///
/// Seven since HS-S0048, and the last two are a different kind of claim rather
/// than two more compilations. Everything above them type-checks; the pair below
/// asserts that the wasm32 conformance targets are wired to the one rule set and
/// then **executes** those rules on the target. Appended for the same reason the
/// fifth was, and in that order: the guard that cannot be skipped comes before
/// the run that can.
///
/// [ADR-0001]: ../../.kb/decisions/0001-async-port-flavours.md
fn wasm_steps() -> Vec<&'static Step> {
    steps_named(&[
        "wasm32 build of the contract crate",
        "wasm32 check of the conformance harnesses",
        "wasm32 build of the Cloudflare adapter",
        "wasm32 build of the Neon adapter",
        "wasm32 build of the typed layer",
        "wasm32 conformance targets are non-vacuous",
        "wasm32 run of the conformance rules",
    ])
}

/// Every `REQUIRED` step that reads a document rather than compiling a package,
/// selected by name. It carried a count until the count went wrong — it said
/// *five* while naming nine of the eleven — so [`print_help`] prints the
/// selection, and `affected.rs` holds the selection to the step table.
fn lint_steps() -> Vec<&'static Step> {
    steps_named(&[
        "specification traceability",
        "no retired rule is still live",
        "no conformance rule reads a clock",
        "no literal position values in the suite",
        "every conformance rule has a changelog entry",
        "every stated rule count matches the suite",
        "the testkit carries its own version",
        "happenstance-core names serde/alloc and base64/alloc",
        "the Rust constitution is internally consistent",
        lint_narrative::STEP,
        lint_pages::STEP,
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

/// `REQUIRED` without `OPTIONAL` — the project-scoped bar, not the release bar.
///
/// It exists for one caller: `verify.integration_scoped` in
/// `.redkiln/config.yaml`, which a **non-terminal** project's integration gate
/// runs. That gate fires on both sides of its stage, so the full gate would run
/// the feature powerset and `cargo deny` twice per project against a tree that
/// has not changed, and the phases those projects carry are gated on the release
/// bar anyway — `verify.e2e` on the terminal project runs [`run_ci`] whole.
///
/// What it drops is exactly `OPTIONAL`: the two feature powersets, `cargo deny`
/// and the nightly `--cfg docsrs` rustdoc build. What it keeps is everything a
/// missing tool could never have skipped — including the four `wasm32` steps,
/// which are the standing guard on ADR-0001 and are not something a *scope* is
/// allowed to narrow.
///
/// # Errors
///
/// When any mandatory step fails.
fn run_fast() -> Result<()> {
    run_steps(REQUIRED)?;
    println!(
        "\nall required checks passed (--fast: {} optional step(s) not run)",
        OPTIONAL.len()
    );
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use std::fs;

    use crate::spec_trace::workspace_root;
    use crate::{OPTIONAL, REQUIRED, Step, wasm_steps};

    /// The feature gating the projection module and its re-exports.
    const GATE: &str = "unstable-projection";

    /// The warning policy, spelled once and compared everywhere it appears.
    ///
    /// `.cargo/config.toml`'s `[build] rustflags` is what a developer's cargo
    /// reads; `.github/workflows/ci.yml`'s `RUSTFLAGS:` is what a runner's
    /// does, and the environment variable **replaces** the config key rather
    /// than merging with it. So the two must be identical, not merely
    /// compatible — and this is the check that keeps them so.
    ///
    /// They were not. CI set the variable and nothing set it locally, which is
    /// how `happenstance-core` kept a dead private method through every green
    /// local gate it ever ran. Widening one side — say to `-D warnings -D
    /// clippy::pedantic` — must fail here, so whoever widens it widens both.
    const DENY_WARNINGS: &str = "-D warnings";

    /// The gate and CI deny the same warnings, read out of both files.
    #[test]
    fn the_gate_and_ci_deny_the_same_warnings() {
        let root = workspace_root().unwrap();

        let workflow = fs::read_to_string(root.join(".github/workflows/ci.yml")).unwrap();
        let in_ci = workflow
            .lines()
            .find_map(|line| line.trim().strip_prefix("RUSTFLAGS:"))
            .map(str::trim)
            .expect("ci.yml must set RUSTFLAGS in its job env, or CI is laxer than the gate");

        assert_eq!(
            in_ci, DENY_WARNINGS,
            "ci.yml denies `{in_ci}`, which is not `{DENY_WARNINGS}`"
        );

        // Compared as the rendered flag string rather than by parsing TOML: the
        // key is a list, and what has to match CI is what cargo hands rustc.
        let config = fs::read_to_string(root.join(".cargo/config.toml")).unwrap();
        let in_config = config
            .lines()
            .find_map(|line| line.trim().strip_prefix("rustflags = "))
            .map(|list| {
                list.trim_matches(['[', ']'].as_slice())
                    .split(',')
                    .map(|flag| flag.trim().trim_matches('"'))
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .expect("`.cargo/config.toml` must set `[build] rustflags`, or a local gate is laxer than CI");

        assert_eq!(
            in_config, DENY_WARNINGS,
            "`.cargo/config.toml` denies `{in_config}` and ci.yml denies              `{DENY_WARNINGS}`; a local gate is not what CI runs"
        );
    }

    /// The fifth `wasm32` step's name, spelled once.
    ///
    /// A grammatical peer of the four it joins, so the `=== name ===` transcript
    /// still scans as one family rather than one stranger.
    const TYPED_LAYER_WASM: &str = "wasm32 build of the typed layer";

    /// The four `wasm32` steps that existed before the typed layer's, in order.
    ///
    /// They are the standing guard on ADR-0001 and are not something a scope —
    /// or a refactor — is allowed to narrow or reorder.
    const ORIGINAL_WASM_STEPS: &[&str] = &[
        "wasm32 build of the contract crate",
        "wasm32 check of the conformance harnesses",
        "wasm32 build of the Cloudflare adapter",
        "wasm32 build of the Neon adapter",
    ];

    /// The step that **executes** conformance rules on `wasm32`, spelled once.
    const WASM_CONFORMANCE_RUN: &str = "wasm32 run of the conformance rules";

    /// Its mandatory compensator, spelled once.
    ///
    /// The half of shape (ii) that may never be skipped: an emptied, rewired or
    /// hand-subsetted wasm32 conformance target fails on every machine, whether
    /// or not the runner that would have executed it is installed.
    const WASM_CONFORMANCE_SHAPE: &str = "wasm32 conformance targets are non-vacuous";

    /// The `wasm32` family as `cargo xtask wasm` runs it, in order.
    ///
    /// The five checks keep their position — a reader who knows the transcript
    /// should still recognise it — and the two rows that turn the family from a
    /// compile into a run are appended after them, never interleaved.
    const WASM_STEPS: &[&str] = &[
        "wasm32 build of the contract crate",
        "wasm32 check of the conformance harnesses",
        "wasm32 build of the Cloudflare adapter",
        "wasm32 build of the Neon adapter",
        TYPED_LAYER_WASM,
        WASM_CONFORMANCE_SHAPE,
        WASM_CONFORMANCE_RUN,
    ];

    /// The named step, out of a compile-time step table.
    fn step<'a>(table: &'a [Step], name: &str) -> &'a Step {
        table
            .iter()
            .find(|step| step.name == name)
            .unwrap_or_else(|| panic!("no step named `{name}`"))
    }

    /// Every crate that names a projection item in its own `src/`, and must
    /// therefore ask for [`GATE`] in its own manifest rather than inherit it from
    /// whatever else the workspace build happened to turn on.
    ///
    /// Feature unification is per *build*. `cargo doc -p <crate>` and every
    /// `cargo hack` combination build one crate at a time, so a manifest that
    /// relies on a sibling's feature selection compiles in the workspace build
    /// and nowhere else.
    const DEPENDENTS: &[&str] = &[
        "happenstance-sqlite",
        "happenstance-ladybug",
        "happenstance-postgres",
        "happenstance-neon",
        "happenstance-sync",
        "happenstance-testkit",
    ];

    /// Read a workspace file with its line endings normalised.
    ///
    /// The tree is mixed: `core.autocrlf` is on for the Windows this repository
    /// is developed on, so a file git has checked out carries `\r\n` and one an
    /// editor wrote carries `\n`. rustfmt's `newline_style = "Auto"` is happy
    /// with either, and an assertion matching `\n` on a `\r\n` file would fail
    /// for a reason that has nothing to do with what it is asserting.
    fn read(rel: &str) -> String {
        let root = workspace_root().unwrap();
        fs::read_to_string(root.join(rel))
            .unwrap_or_else(|e| panic!("reading {rel}: {e}"))
            .replace("\r\n", "\n")
    }

    /// The `default` line of a `[features]` table, as written.
    fn default_features(manifest: &str) -> &str {
        manifest
            .lines()
            .find(|l| l.starts_with("default = "))
            .unwrap_or("")
    }

    /// The maturity signal an adapter author meets first is the feature table,
    /// because that is what `cargo add` shows them — not a doc comment three
    /// screens inside a module they have to already be reading.
    #[test]
    fn the_projection_port_is_behind_an_off_by_default_feature() {
        let manifest = read("crates/happenstance-core/Cargo.toml");

        assert!(
            manifest.contains(&format!("\n{GATE} = ")),
            "`{GATE}` is not declared in happenstance-core's `[features]`"
        );
        assert!(
            !default_features(&manifest).contains(GATE),
            "`{GATE}` is on by default, which hands the port to every caller who \
             never asked for it: {}",
            default_features(&manifest)
        );
    }

    /// A feature that gates nothing is a feature table telling a story the
    /// compiler does not: the module and the re-exports are the two places the
    /// item is either reachable or invisible.
    #[test]
    fn the_gate_is_mounted_on_the_module_and_its_re_exports() {
        let lib = read("crates/happenstance-core/src/lib.rs");
        let cfg = format!("#[cfg(feature = \"{GATE}\")]");
        let doc_cfg = format!("#[cfg_attr(docsrs, doc(cfg(feature = \"{GATE}\")))]");

        assert!(
            lib.contains(&format!("{cfg}\n{doc_cfg}\npub mod projection;")),
            "`pub mod projection;` is not gated the way `memory`'s module is"
        );
        assert!(
            lib.contains(&format!("{cfg}\n{doc_cfg}\npub use projection::{{")),
            "the projection re-exports are not gated, so the module is invisible \
             and its items are not"
        );
    }

    /// The powerset proves the combinations compile; this proves the *manifests*
    /// are the reason, which is the half a workspace build cannot distinguish
    /// because it unifies features across every member at once.
    #[test]
    fn every_crate_that_names_a_projection_item_opts_in() {
        for package in DEPENDENTS {
            let manifest = read(&format!("crates/{package}/Cargo.toml"));
            assert!(
                manifest.contains(GATE),
                "{package} names a projection item and never asks for `{GATE}`"
            );
        }
    }

    /// A semver promise rather than an oversight: the typed layer's feature
    /// table and its public surface say the same thing.
    ///
    /// This assertion used to read the other way — `happenstance` re-exported
    /// no projection item, so a public feature there would have promised a
    /// surface the crate did not expose. The typed projection runner made the
    /// premise false rather than the rule wrong, so the rule is stated in the
    /// direction that still has content: a passthrough **and** a surface, or
    /// neither. What it still forbids is the pair coming apart — a feature
    /// table advertising a runner nobody can name, or four `pub use`s a
    /// consumer cannot turn on.
    #[test]
    fn the_typed_layer_makes_no_promise_it_does_not_keep() {
        let manifest = read("crates/happenstance/Cargo.toml");
        let lib = read("crates/happenstance/src/lib.rs");

        let cfg = format!("#[cfg(feature = \"{GATE}\")]");
        let doc_cfg = format!("#[cfg_attr(docsrs, doc(cfg(feature = \"{GATE}\")))]");

        let passthrough = manifest.contains(&format!("\n{GATE} = "));
        // The badge is part of the surface, not an extra: an item that only
        // exists behind a feature and renders no gate is anti-pattern 15.
        let surface = lib.contains(&format!("{cfg}\n{doc_cfg}\npub use runner::{{"));

        assert_eq!(
            passthrough, surface,
            "`happenstance`'s `{GATE}` passthrough and its projection surface \
             disagree: the manifest declares it = {passthrough}, the crate root \
             re-exports behind it = {surface}"
        );

        if passthrough {
            assert!(
                !default_features(&manifest).contains(GATE),
                "`{GATE}` joined the typed layer's defaults, which hands an \
                 unfrozen port to every caller who never asked for it: {}",
                default_features(&manifest)
            );
            assert!(
                manifest.contains(&format!("\"happenstance-core/{GATE}\"")),
                "the typed layer gates a runner on `{GATE}` without forwarding \
                 it to the contract crate, whose port the runner is built on"
            );
            assert!(
                lib.contains(&format!("{cfg}\nmod runner;")),
                "the runner's own module is not gated, so the feature table and \
                 the compiler disagree about what `{GATE}` turns on"
            );
        }
    }

    // -----------------------------------------------------------------------
    // HS-S0031 AC-001, AC-002, AC-006 — the typed layer's wasm32 claim
    // -----------------------------------------------------------------------

    /// The `//` comment block immediately above a `name:` line in this file.
    ///
    /// The gate's steps carry their reasoning as ordinary line comments inside
    /// the `Step` literal, so it is unreachable at runtime and has to be read
    /// off the source. That is the point: a step whose comment only restates its
    /// argument vector is the decorative shape this file's own documentation
    /// warns about, and nothing but a source read can observe the difference.
    fn step_comment(name: &str) -> String {
        let source = read("xtask/src/main.rs");
        let lines: Vec<&str> = source.lines().collect();
        let at = lines
            .iter()
            .position(|line| line.trim() == format!("name: \"{name}\","))
            .unwrap_or_else(|| panic!("no `name: \"{name}\",` line in xtask/src/main.rs"));

        let mut comment: Vec<&str> = Vec::new();
        for line in lines[..at].iter().rev() {
            let trimmed = line.trim();
            let Some(rest) = trimmed.strip_prefix("//") else {
                break;
            };
            comment.push(rest.trim());
        }
        comment.reverse();
        comment.join(" ")
    }

    /// AC-001. The crate a Workers application installs is compiled for the
    /// target it claims, by a step nothing can skip.
    #[test]
    fn typed_layer_wasm_step_carries_the_designed_arguments() {
        let step = step(REQUIRED, TYPED_LAYER_WASM);

        assert_eq!(step.program, "cargo");
        assert_eq!(
            step.args,
            [
                "check",
                "--locked",
                "-p",
                "happenstance",
                "--target",
                "wasm32-unknown-unknown",
                "--no-default-features",
                "--features",
                "std,json",
            ],
            "the argument list is `_design.md`'s, not the implementer's"
        );
        assert!(step.env.is_empty(), "the step needs no environment");

        // Persistent chrome, never opened on demand. A probe means *skip when
        // the tool is absent*, and this guard may never be skippable (RS-80-2).
        assert!(
            step.probe.is_none(),
            "a probed guard is unguarded on every machine that lacks the tool"
        );

        // Hierarchy: a grammatical peer of the four it joins.
        assert!(
            step.name.starts_with("wasm32 build of the"),
            "`{}` does not scan as one of the wasm32 family",
            step.name
        );

        // Presentation exists at all: the comment states both halves.
        let comment = step_comment(TYPED_LAYER_WASM).to_lowercase();
        assert!(
            !comment.is_empty(),
            "the step carries no reasoning at all, only arguments"
        );
        assert!(
            comment.contains("workers"),
            "the comment does not say what the step buys: {comment}"
        );
        assert!(
            comment.contains("send") && comment.contains("flavour"),
            "the comment does not say what the step does NOT prove — `Send` \
             exists on wasm32, so this proves nothing about which flavour is \
             bound: {comment}"
        );
        assert!(
            comment.contains("flavours.rs"),
            "the comment does not point at the instrument that covers the half \
             it cannot: {comment}"
        );
    }

    /// AC-002 (HS-S0031, and HS-S0048 after it). Removing a step is loud, and
    /// the originals keep their position so a reader who knows the transcript
    /// still recognises it.
    ///
    /// Renamed from `wasm_steps_resolve_and_number_five` when the two execution
    /// rows landed: a count in a test name is a second copy of the list it
    /// describes, and it drifts the first time the list moves. Every assertion
    /// the old name carried is still here.
    #[test]
    fn wasm_steps_resolve_and_the_check_family_keeps_its_order() {
        // `wasm_steps` resolves by name through `steps_named`, which panics on a
        // name that resolves to nothing. Calling it *is* the assertion.
        let steps = wasm_steps();

        let resolved: Vec<&str> = steps.iter().map(|step| step.name).collect();
        assert_eq!(
            resolved, WASM_STEPS,
            "`cargo xtask wasm` no longer runs the designed family in the \
             designed order"
        );
        for (at, expected) in ORIGINAL_WASM_STEPS.iter().enumerate() {
            assert_eq!(
                steps[at].name, *expected,
                "the four original wasm32 steps moved; everything since is \
                 appended, never interleaved"
            );
        }
        assert_eq!(
            steps[4].name, TYPED_LAYER_WASM,
            "the typed layer's step is not the appended fifth"
        );

        // And the four originals keep their argument lists byte-for-byte
        // (NF-004): they are the standing guard on ADR-0001.
        for name in ORIGINAL_WASM_STEPS {
            let original = step(REQUIRED, name);
            assert!(
                original.args.contains(&"--target") && original.args.contains(&"--locked"),
                "`{name}` lost an argument it had before the typed layer joined"
            );
            assert!(original.probe.is_none(), "`{name}` acquired a probe");
        }
    }

    /// AC-006. The powerset widens the claim above the mandatory point, and
    /// keeps its probe: a mandatory powerset breaks every machine without
    /// `cargo hack`.
    #[test]
    fn the_wasm32_powerset_covers_the_typed_layer() {
        let step = step(OPTIONAL, "wasm32 feature powerset");

        let named: Vec<&&str> = step
            .args
            .iter()
            .zip(step.args.iter().skip(1))
            .filter_map(|(flag, package)| (*flag == "-p").then_some(package))
            .collect();
        assert!(
            named.contains(&&"happenstance"),
            "the typed layer's own feature combinations are never compiled for \
             wasm32: {named:?}"
        );

        assert!(
            step.args.contains(&"--feature-powerset")
                && step.args.contains(&"wasm32-unknown-unknown"),
            "the step stopped being a wasm32 powerset"
        );

        // Retained, deliberately. This is a widening *above* the mandatory
        // guard and never a replacement for it.
        let probe = step
            .probe
            .expect("a mandatory powerset breaks every machine without cargo hack");
        assert!(
            probe.contains(&"hack"),
            "the powerset's probe no longer names the tool it needs: {probe:?}"
        );
    }

    // -----------------------------------------------------------------------
    // HS-S0048 AC-001, AC-002, AC-004, AC-006 — the wasm32 execution seam
    // -----------------------------------------------------------------------

    /// AC-001. The gate *runs* rules on `wasm32`, and runs them legibly.
    ///
    /// The wrong implementation this rejects is a sixth `cargo check`: a step
    /// whose name promises execution and whose arguments type-check. It also
    /// rejects the run that captures its own output, because `println!` is a
    /// silent discard on this target and a declined capability's reason reaches
    /// the terminal only under the runner's `--nocapture`.
    #[test]
    fn the_wasm32_conformance_run_executes_rather_than_checks() {
        let step = step(REQUIRED, WASM_CONFORMANCE_RUN);

        assert_eq!(step.program, "cargo");
        assert!(
            step.args.contains(&"wasm-conformance"),
            "the step does not drive the executed-target list: {:?}",
            step.args
        );
        assert!(
            !step.args.contains(&"check"),
            "the step is a `cargo check`, which is the claim it exists to \
             replace: {:?}",
            step.args
        );
        assert!(
            step.args.contains(&"--locked"),
            "a gate step that resolves dependencies without `--locked` tested a \
             graph nobody committed"
        );

        let comment = step_comment(WASM_CONFORMANCE_RUN).to_lowercase();
        assert!(
            comment.contains("nocapture"),
            "the comment does not say why the run is uncaptured — `println!` is a \
             silent discard on this target: {comment}"
        );
        assert!(
            comment.contains("execut"),
            "the comment does not distinguish executing from type-checking, \
             which is the whole of what this step adds: {comment}"
        );
    }

    /// AC-004. The chosen shape is (ii), and both halves of it are real.
    ///
    /// A bare probe-gated step with no compensator is precisely the
    /// configuration project AC-004 forbids: on a machine without the runner it
    /// prints `skipped:` and the gate still goes green over a target that could
    /// have been emptied. So the probe is admissible only while a `probe: None`
    /// row asserts the target and its rule enumeration regardless.
    #[test]
    fn the_wasm32_run_is_probed_and_its_compensator_never_is() {
        let run = step(REQUIRED, WASM_CONFORMANCE_RUN);
        let shape = step(REQUIRED, WASM_CONFORMANCE_SHAPE);

        let probe = run
            .probe
            .expect("shape (i) was chosen; then the comment and this test disagree");
        assert!(
            probe
                .iter()
                .any(|arg| arg.contains("wasm-bindgen-test-runner")),
            "the run's probe does not name the runner it needs: {probe:?}"
        );

        assert!(
            shape.probe.is_none(),
            "the compensator is skippable, which makes the pair a bare \
             probe-gated step — the one configuration AC-004 forbids"
        );
        assert!(
            shape.args.contains(&"wasm-conformance-enumeration"),
            "the compensator does not assert the executed targets: {:?}",
            shape.args
        );

        // The argument is written down, not merely acted on: the house habit is
        // that the gate carries its own reasoning, and a step whose comment
        // restates its arguments is the decorative shape this file warns about.
        let comment = step_comment(WASM_CONFORMANCE_RUN).to_lowercase();
        assert!(
            comment.contains("mandatory"),
            "the comment does not name the rejected alternative — shape (i), a \
             mandatory step: {comment}"
        );
        assert!(
            comment.contains("rust-toolchain.toml"),
            "the comment does not say why this tool is in a different class from \
             the wasm32 target the other steps rely on: {comment}"
        );
        assert!(
            step_comment(WASM_CONFORMANCE_SHAPE)
                .to_lowercase()
                .contains("skip"),
            "the compensator does not say what it compensates for"
        );
    }

    /// AC-002. `cargo xtask --help` stops claiming the wasm32 tasks only build.
    ///
    /// Read off the source for the reason [`step_comment`] is: `print_help`
    /// writes to stdout, and the sentence a reader is misled by is a string
    /// literal rather than a value anything can observe at runtime.
    #[test]
    fn the_help_text_says_the_wasm32_tasks_execute_rules() {
        let source = read("xtask/src/main.rs");
        let help = source
            .split("println!(\"  wasm")
            .nth(1)
            .expect("no `wasm` entry in print_help")
            .split("println!(\"  spec-trace")
            .next()
            .expect("the `wasm` help entry runs to the end of the file")
            .to_lowercase();

        assert!(
            help.contains("execut") || help.contains(" run"),
            "the `wasm` help still describes the family as builds and checks \
             alone: {help}"
        );
        assert!(
            !help.contains("five checks"),
            "the `wasm` help still counts five checks, which is the sentence the \
             execution rows falsify: {help}"
        );
    }

    /// AC-006. No gate step hard-codes the executed target.
    ///
    /// The named wrong implementation: `-p happenstance-testkit --test
    /// memory_conformance_wasm` inside a `Step`'s own `args`. It passes every
    /// other criterion here and forces HS-S0054 to write a second execution
    /// step, a second runner variable and a second anti-vacuity guard to add one
    /// row's worth of coverage.
    #[test]
    fn no_gate_step_hard_codes_an_executed_wasm_target() {
        for table in [REQUIRED, OPTIONAL] {
            for step in table {
                for wasm in crate::proof::WASM_TARGETS {
                    assert!(
                        !step.args.contains(&wasm.target),
                        "`{}` names the executed target `{}` in its own \
                         arguments; the targets are a declared list so a second \
                         one is a row, not a step",
                        step.name,
                        wasm.target
                    );
                }
            }
        }
    }

    /// The two names `ARTEFACTS` holds for `projection_harness_parity`.
    ///
    /// One row of the nine, and the smallest: two names, one target, and a
    /// transcript short enough to quote whole.
    /// In the row's own order, which is not libtest's — the assertion below
    /// reads them out of `ARTEFACTS` rather than trusting this copy.
    const PARITY: &[&str] = &[
        "projection_harness_parity::no_harness_lists_a_rule_by_hand",
        "projection_harness_parity::each_harness_invokes_the_suite_exactly_once",
    ];

    /// That target's libtest stdout when its tests run, verbatim from the gate's
    /// own transcript
    /// (`experiments/gate-vacuity/results/raw/baseline-ci.txt:7570-7574`).
    const PARITY_RAN: &str = "running 2 tests
test projection_harness_parity::each_harness_invokes_the_suite_exactly_once ... ok
test projection_harness_parity::no_harness_lists_a_rule_by_hand ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
";

    /// The same target with an `#[ignore = \"…\"]` on both tests, verbatim from
    /// `experiments/gate-vacuity/results/raw/ignore-all-ignore-reason-ci.txt:7082-7086`.
    ///
    /// The process exited **0** over this and the step printed *"2 named tests
    /// present"* about it.
    const PARITY_IGNORED: &str = "running 2 tests
test projection_harness_parity::each_harness_invokes_the_suite_exactly_once ... ignored, measured by experiments/gate-vacuity
test projection_harness_parity::no_harness_lists_a_rule_by_hand ... ignored, measured by experiments/gate-vacuity

test result: ok. 0 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.00s
";

    /// The same *count* of passes, from two tests the gate does not name.
    ///
    /// Held here because it is what separates the two remediations the audit
    /// left open: comparing the reported `passed` count against `tests.len()`
    /// reads `2 passed` and is satisfied, while both named tests are gone.
    const PARITY_SUBSTITUTED: &str = "running 2 tests
test projection_harness_parity::a_test_the_gate_does_not_name ... ok
test projection_harness_parity::another_test_the_gate_does_not_name ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
";

    /// A proof artefact's named test that did not **run** fails the gate.
    ///
    /// This is the assertion behind `proof.rs`'s central claim (`:29-30`, *"the
    /// names are asserted, out of `--list`, before the tests run"*) and behind
    /// the other places in this workspace that say an `#[ignore]` on a named
    /// proof test cannot pass — among them `proof.rs:217`, `:2050` and `:2408`,
    /// and RS-81-4's own `Rejects:` line.
    ///
    /// It was measured false. `experiments/gate-vacuity/results/raw/list-diff.txt`
    /// is **empty**: libtest's `--list` output is byte-identical with and without
    /// an `#[ignore]`, so an assertion over that listing cannot observe the
    /// attribute, and `cargo xtask ci` exits 0 with all 31 named tests ignored.
    ///
    /// The wrong implementations it rejects, in order of how tempting they are:
    /// a presence check over the run's text — both names appear in
    /// [`PARITY_IGNORED`], on their `ignored` lines — and a comparison of the
    /// reported `passed` count against `tests.len()`, which [`PARITY_SUBSTITUTED`]
    /// satisfies with neither named test having run.
    ///
    /// It lives in `main.rs` rather than beside its subject for two reasons. This
    /// file is already where the gate says an `#[ignore]` on a named test is
    /// caught by the runner rather than by a listing (`:402-404`), so the claim
    /// and its proof sit together; and a test inside `proof.rs` could not tell an
    /// implementation reverted from a test reverted along with it.
    #[test]
    fn a_named_proof_test_that_did_not_run_fails_the_gate() {
        // The fixture's own non-vacuity first, in the shape `proof.rs`'s
        // `a_registry_count_belongs_to_the_target_it_is_printed_beside` uses:
        // every assertion below holds trivially over an empty `PARITY`, so
        // emptying it is a one-token edit that keeps this test green and retires
        // it. Read out of `ARTEFACTS` rather than believed, so the names are the
        // row's own and a rename has this to disagree with too.
        let row = crate::proof::ARTEFACTS
            .iter()
            .find(|artefact| artefact.target == "projection_harness_parity")
            .expect("`projection_harness_parity` is an `ARTEFACTS` row");
        assert_eq!(
            row.tests, PARITY,
            "the transcripts below are that row's, and this test is about names \
             the gate actually holds"
        );
        assert_eq!(
            PARITY.len(),
            2,
            "one name would still exercise the mechanism; zero would exercise \
             nothing and pass"
        );

        assert!(
            crate::proof::unexecuted(PARITY, PARITY_RAN).is_empty(),
            "a target whose named tests both passed is reported as unexecuted"
        );

        assert_eq!(
            crate::proof::unexecuted(PARITY, PARITY_IGNORED),
            PARITY.to_vec(),
            "both named tests were `ignored` and the gate found nothing to say; \
             this is the measured defect, and a step that reads only the exit \
             status is checking that the target compiles"
        );

        assert_eq!(
            crate::proof::unexecuted(PARITY, PARITY_SUBSTITUTED),
            PARITY.to_vec(),
            "two tests passed, neither of them the ones the clauses cite — a \
             `passed`-count comparison is satisfied here and the names it was \
             counting are gone"
        );
    }

    /// S-3: `constitution.rs` and this file's own step comment claimed
    /// `RUSTDOCFLAGS=-D warnings` partially recovers lint coverage inside a
    /// doctest fence. RS-01-4 (`standards/rust/01-standard-of-evidence.md:180-188`)
    /// already denied that, and `experiments/gate-vacuity/results/`
    /// `constitution-fence.md` measured it false a second time, against this
    /// very corpus: the same `RUSTDOCFLAGS` arm let a `non_snake_case`
    /// violation compile and run at exit 0. A text scan, not a doctest — the
    /// false sentence compiles fine, so only reading the prose catches its
    /// return.
    ///
    /// Deliberately not placed in `constitution.rs`: `lint-constitution`'s
    /// `check_harness` reads that file's `mod` lines as the atom registry, so
    /// a bare `mod tests {` there is flagged as naming an atom that does not
    /// exist. This file carries no such reading.
    #[test]
    fn constitution_rs_does_not_restate_the_rustdocflags_recovery_claim() {
        let production = include_str!("constitution.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        assert!(
            !production.contains("recovers rustc's")
                && !production.contains("the recovery is partial"),
            "constitution.rs restates the RUSTDOCFLAGS partial-recovery claim RS-01-4 denies"
        );
    }

    /// The companion claim, in this file's own step comment: it leaned on
    /// the same false premise to justify what the constitution's examples
    /// are "held to".
    ///
    /// Scoped to the *line-anchored* `#[cfg(test)]` marker, normalising CRLF
    /// first — same reason `lint_narrative.rs`'s `production_source()` does
    /// both: a bare substring split would also cut at line 293's comment,
    /// which names the attribute in backticks and sits well above this
    /// module, truncating "production" before the target line is even
    /// reached.
    #[test]
    fn main_rs_does_not_restate_the_rustdocflags_recovery_claim() {
        let normalised = include_str!("main.rs").replace("\r\n", "\n");
        let production = normalised.split("\n#[cfg(test)]\n").next().unwrap();
        assert!(
            !production.contains("is the only way `-D warnings` reaches rustdoc"),
            "main.rs restates the RUSTDOCFLAGS partial-recovery claim RS-01-4 denies"
        );
    }
}
