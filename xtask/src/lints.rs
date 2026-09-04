//! The gate's grep-shaped lints (CF-6, CF-29, CF-32, CF-33, D12 — which has no
//! clause, ADR-0016 §14 gives it a lint rather than a WF-13 — and the README
//! count check, which has no clause either and says why in its own docs).
//!
//! # Why a grep is in the gate at all
//!
//! Each of these clauses names a `cargo xtask ci` step, and each one is checking
//! something the type system cannot express. There is no lint that says "this
//! crate may not observe time", no type that says "this literal is not a
//! sequence position", and no compiler pass that reads `CHANGELOG.md`. The
//! clauses say so in their own words rather than implying a stronger mechanism,
//! and this module keeps that honest: every check below states what it does
//! *not* verify, because a check whose limits are undocumented is read as a
//! guarantee.
//!
//! # The direction this repository is weak in
//!
//! Every other step in the gate holds *code* to a document: the suite to the
//! specification, the rules to the changelog, the manifests to their features.
//! [`stated_rule_counts`] runs the other way, and it exists because the failure
//! it catches happened three times before anything could see it — a document
//! shipped to crates.io describing a suite two thirds smaller than the one in
//! the package beside it. `cargo package --list` asserts a README is *present*;
//! nothing asserted it was *true*.
//!
//! # The one thing they all share
//!
//! Three of the four match on Rust source, and prose about a construct reads
//! exactly like the construct. `concurrency.rs` explains at length why CF-33
//! forbids the watchdog it does not have — the word `sleep` appears in that
//! explanation four times — so a lint matching raw bytes fires on the sentence
//! justifying its own existence, and the fix a hurried reader reaches for is to
//! delete the sentence or weaken the lint. [`code_lines`] is what stops that:
//! every match below runs against source with comments removed and string
//! literal *contents* blanked, so only code is looked at.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::spec_trace::{RULE_FILES, all_rules, collect_rules, workspace_root};

/// The directory CF-33 is scoped to.
///
/// `src/` and deliberately not the whole crate. `tests/` is where the
/// concurrency racers live, and a racer that spawns threads to lose an update
/// may legitimately need to synchronise — CF-33 constrains *conformance rules*,
/// which are the library's, not the wrong implementations the testkit drives
/// them against. A whole-crate grep would fire on `tests/` and teach the next
/// person that the remedy is to widen the exclusions, which is the direction
/// that ends with the check switched off.
const TESTKIT_SRC: &str = "crates/happenstance-testkit/src";

/// The testkit's manifest, for CF-32.
const TESTKIT_MANIFEST: &str = "crates/happenstance-testkit/Cargo.toml";

/// The changelog CF-29 reads.
const CHANGELOG: &str = "CHANGELOG.md";

/// `happenstance-core`'s manifest, for the D12 lint.
const CORE_MANIFEST: &str = "crates/happenstance-core/Cargo.toml";

/// The page crates.io renders, which no other step reads for content.
///
/// It is the first document an adapter author meets and the last one this
/// workspace held to anything. Between `79df6b7` and this check it told
/// crates.io that the projection suite "is two rules of seventeen, and neither
/// has been shown to reject a wrong store yet — the hostile stores that will are
/// named in the specification and not yet written", while seventeen rules drove
/// the port and eighteen wrong stores sat in
/// `crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs`.
/// The same commit range that falsified the sentence also shipped it.
///
/// `pub(crate)`: `xtask/src/lint_pages.rs::no_stale_publication_claims` (C2-07)
/// reads it too, for the reason on `TESTKIT_LIB`'s doc comment below.
pub(crate) const TESTKIT_README: &str = "crates/happenstance-testkit/README.md";

/// Every line of a Rust source file with comments removed and string-literal
/// contents blanked, one output line per input line.
///
/// Line numbering is preserved so a hit can be reported at the line a reader
/// will open, and string literals keep their quotes so the code around them
/// still parses to the eye.
///
/// # Why the contents of strings go too
///
/// A panic message that mentions a clock is not a rule that reads one, and the
/// suite's messages are long and discursive by design. Blanking them removes a
/// whole class of false positive at no cost: nothing below is looking for a
/// construct that can be spelled inside a string.
///
/// # What this is not, and why it now says so out loud
///
/// It is not a Rust lexer. Four constructs would defeat it, all of them the same
/// failure — the scanner's idea of "inside a string" flips and the rest of the
/// file is blanked, so **every match below silently stops reporting**: the char
/// literal `'"'`, a block comment, a raw string, and (as the symptom of any of
/// them) a string still open at end of file.
///
/// This function used to *assert* that no scanned file contained one and promise
/// that if one ever did "the failure will be a loud one". It would not have been.
/// A lint that stops reporting prints its success line and exits 0, which is the
/// quietest failure available, and the promise told the next reader not to look
/// for it. It was demonstrated: one `let quote = '"';` inserted into a rule made
/// both CF-33 and CF-6 green over a rule body that read a clock *and* asserted
/// `[1, 2, 3]`.
///
/// So the assumption is checked instead of asserted. Each construct is detected
/// **while the scanner is in code**, which is the only position where it can be
/// told apart from the same bytes inside a string — `"…their"` ends in `r"` and
/// is not a raw string, and a whole-file grep would have to guess. Detection is a
/// hard error naming the file and line, because the honest answer at that point
/// is "this file needs a lexer and this is not one", not a best effort.
///
/// # Errors
///
/// Returns an error if `source` contains a construct this scanner cannot lex.
fn code_lines(file: &str, source: &str) -> Result<Vec<String>> {
    let chars: Vec<char> = source.chars().collect();
    let mut out = vec![String::new()];
    let mut in_string = false;
    let mut escaped = false;
    let mut i = 0;

    // What the scanner has emitted as code immediately before `i`. A raw string
    // opens where `r` (or `br`) is preceded by something that cannot end an
    // identifier; without that test, `char` and `for` and every string ending in
    // the letter `r` would read as one.
    let ident = |c: char| c.is_alphanumeric() || c == '_';

    let unlexable = |line: usize, what: &str| -> anyhow::Error {
        anyhow::anyhow!(
            "{file}:{line} — this lint's scanner is not a Rust lexer and this file contains {what}. \
             It cannot be scanned correctly: the scanner's string state flips and the rest of the \
             file is blanked, so CF-33 and CF-6 would report success over code they never looked \
             at. Either spell the construct another way, or replace `code_lines` with a real \
             tokeniser — do not widen the exclusions."
        )
    };

    while i < chars.len() {
        let c = chars[i];
        i += 1;

        if c == '\n' {
            out.push(String::new());
            // A string literal may span lines; a comment may not.
            continue;
        }

        let at = out.len();
        let line = out
            .last_mut()
            .unwrap_or_else(|| unreachable!("out is never empty"));

        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
                line.push('"');
            }
            continue;
        }

        // In code from here down, which is what makes these four tests precise.
        if c == '\'' && chars.get(i) == Some(&'"') && chars.get(i + 1) == Some(&'\'') {
            return Err(unlexable(at, "a `'\"'` char literal"));
        }
        if c == '/' && chars.get(i) == Some(&'*') {
            return Err(unlexable(at, "a block comment"));
        }
        if c == 'r' && matches!(chars.get(i), Some('"' | '#')) {
            // `br"…"` too: the `b` is the only prefix that may sit in front.
            let before = match line.chars().next_back() {
                Some('b') => line.chars().nth_back(1),
                other => other,
            };
            if !before.is_some_and(ident) {
                return Err(unlexable(at, "a raw string"));
            }
        }

        if c == '"' {
            in_string = true;
            line.push('"');
            continue;
        }

        if c == '/' && chars.get(i) == Some(&'/') {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        line.push(c);
    }

    // The catch-all, and it is deliberately the *last* line rather than the
    // first: an even number of quote-flipping constructs restores the parity and
    // leaves this silent, which is why the three tests above exist rather than
    // this one alone. What it adds is the odd case in a spelling nobody
    // anticipated. It cannot be the whole answer and it does not have to be —
    // the only construct that reaches end of file mid-string and is not one of
    // the three above is an unterminated literal, which does not compile, and
    // the clippy step runs before this one.
    if in_string {
        return Err(unlexable(
            out.len(),
            "a string literal that is still open at end of file",
        ));
    }

    Ok(out)
}

/// Every `.rs` file under a directory, sorted, so the report is stable.
fn rust_files(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(next) = stack.pop() {
        for entry in fs::read_dir(&next).with_context(|| format!("reading {}", next.display()))? {
            let path = entry
                .with_context(|| format!("reading an entry of {}", next.display()))?
                .path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    out.sort();
    Ok(out)
}

/// The constructs CF-33 names, verbatim from the clause.
///
/// Four, and the clause's list is exactly these four. `Duration` and `timeout`
/// are **not** on it and are not added here: a `[FROZEN]` clause's specified
/// check is what the gate runs, and quietly extending it would mean the document
/// and the gate disagree about what the bar is — with the gate winning silently,
/// which is the failure this whole phase exists to remove. If the list is too
/// short the remedy is an edit to CF-33, not to this array.
const CLOCK_CONSTRUCTS: [&str; 4] = ["std::time", "Instant", "elapsed", "sleep"];

/// CF-33: no conformance rule may read a clock.
///
/// # Errors
///
/// Returns an error if the testkit's sources cannot be read, or if any of
/// [`CLOCK_CONSTRUCTS`] appears in code under [`TESTKIT_SRC`].
pub(crate) fn no_clock() -> Result<()> {
    let root = workspace_root()?;
    let dir = root.join(TESTKIT_SRC);
    let files = rust_files(&dir)?;
    if files.is_empty() {
        bail!("{TESTKIT_SRC} holds no .rs files — CF-33's lint is scanning nothing");
    }

    let mut problems = Vec::new();
    let mut scanned = 0usize;

    for path in &files {
        let body =
            fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .display()
            .to_string()
            .replace('\\', "/");
        scanned += 1;
        for (n, line) in code_lines(&rel, &body)?.iter().enumerate() {
            for needle in CLOCK_CONSTRUCTS {
                if line.contains(needle) {
                    problems.push(format!(
                        "{rel}:{} — CF-33 forbids a conformance rule from reading a clock, and \
                         `{needle}` appears in code here: {}",
                        n + 1,
                        line.trim()
                    ));
                }
            }
        }
    }

    if !problems.is_empty() {
        for p in &problems {
            println!("  {p}");
        }
        bail!(
            "{} use(s) of a clock in {TESTKIT_SRC}. A timing assertion passes on the author's \
             machine and fails on a loaded runner, which teaches adapter authors to re-run until \
             green. If the construct is genuinely not a clock, this lint is the wrong shape and \
             CF-33 says so in its own `Rule:` line — fix the clause, not the grep.",
            problems.len()
        );
    }

    println!("CF-33: {scanned} file(s) in {TESTKIT_SRC} read no clock");
    Ok(())
}

/// CF-32: `happenstance-testkit` carries its own `version` key.
///
/// # Errors
///
/// Returns an error if the manifest cannot be read, if `[package]` declares no
/// `version`, or if the one it declares is inherited from the workspace.
pub(crate) fn testkit_version() -> Result<()> {
    let root = workspace_root()?;
    let manifest = fs::read_to_string(root.join(TESTKIT_MANIFEST))
        .with_context(|| format!("reading {TESTKIT_MANIFEST}"))?;

    // A three-line hand parse rather than a TOML dependency, because the
    // question is narrow — one key, in one table, in one file — and adding a
    // parser to the gate to answer it would be the larger claim. What it costs:
    // this reads the *first* `[package]` table and assumes the key is not
    // written in inline-table form on the `[package]` header line, neither of
    // which any manifest in this workspace does.
    let mut in_package = false;
    let mut declared: Option<&str> = None;
    for line in manifest.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            in_package = t == "[package]";
            continue;
        }
        // Both spellings, kept whole: `version = "0.1.0"` and
        // `version.workspace = true`. The value is reported back with its own
        // punctuation because the failure message is the only place a reader
        // sees what the manifest actually says.
        if in_package
            && let Some(rest) = t.strip_prefix("version")
            && rest.trim_start().starts_with(['=', '.'])
        {
            declared = Some(rest);
            break;
        }
    }

    match declared {
        // Unreachable in practice, and kept anyway: cargo defaults a missing
        // `version` to `0.0.0`, and `happenstance-postgres` depends on the
        // testkit at `^0.1.0`, so deleting the key fails resolution before this
        // step is ever launched. That is a better failure than this one and it
        // is not a reason to drop the branch — the day nothing in the workspace
        // depends on the testkit by version, cargo goes quiet and this does not.
        None => bail!(
            "{TESTKIT_MANIFEST}'s `[package]` declares no `version` key. CF-32 requires one: \
             adding a conformance rule is a semver-MINOR change that turns a passing adapter's \
             CI red, so the testkit's number must be able to move without dragging \
             `happenstance-core` to the same release."
        ),
        Some(value) if value.contains("workspace") => bail!(
            "{TESTKIT_MANIFEST} inherits its version from the workspace (`version{value}`). \
             CF-32 forbids that. Under a shared key the contract crate and the bar cannot move \
             independently in either direction: a new rule republishes an unchanged contract, and \
             a patch of the contract forces every adapter to re-run a bar that did not change."
        ),
        Some(value) => {
            println!("CF-32: {TESTKIT_MANIFEST} carries its own `version{value}`");
            Ok(())
        }
    }
}

/// D12: `happenstance-core`'s `serde` feature names `serde/alloc` and
/// `base64/alloc` explicitly.
///
/// # Errors
///
/// Returns an error if the manifest cannot be read, if `[features]` declares
/// no `serde` line, or if the line it declares omits either `serde/alloc` or
/// `base64/alloc`.
pub(crate) fn core_alloc_features() -> Result<()> {
    let root = workspace_root()?;
    let manifest = fs::read_to_string(root.join(CORE_MANIFEST))
        .with_context(|| format!("reading {CORE_MANIFEST}"))?;

    // A three-line hand parse rather than a TOML dependency, for the same
    // reason `testkit_version` above is one: the question is narrow — one
    // key, in one table, in one file — and adding a parser to the gate to
    // answer it would be the larger claim. What it costs: this reads the
    // *first* `[features]` table and the *first* line beginning `serde =`,
    // neither of which any manifest in this workspace does more than once.
    let mut in_features = false;
    let mut declared: Option<&str> = None;
    for line in manifest.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            in_features = t == "[features]";
            continue;
        }
        if in_features && t.starts_with("serde =") {
            declared = Some(t);
            break;
        }
    }

    match declared {
        None => bail!(
            "{CORE_MANIFEST}'s `[features]` declares no `serde = [...]` line. D12 (closed at \
             phase 0, commit 927d291) requires one that names `serde/alloc` and \
             `base64/alloc` explicitly: this crate is `no_std`, its derives need `alloc`, and \
             today that arrives only because `bytes` 1.12.1 happens to enable it for its own \
             optional `serde` dependency — a transitive default one dependency's PATCH release \
             could change, landing the breakage in a downstream crate that changed nothing. \
             This check asserts a manifest fact that no test and no compile can currently see: \
             D12 cannot be made to fail today, which is exactly why a conformance rule for it \
             would be decorative by construction (ADR-0016 §14). There is deliberately no \
             WF-13; this is a gate step instead."
        ),
        Some(line) if !line.contains("serde/alloc") || !line.contains("base64/alloc") => bail!(
            "{CORE_MANIFEST}'s `serde` feature line omits {}. D12 (closed at phase 0, commit \
             927d291) requires both `serde/alloc` and `base64/alloc` named explicitly: this \
             crate is `no_std` and its derives need `alloc`, and today that arrives only \
             because `bytes` 1.12.1 happens to enable it for its own optional `serde` \
             dependency — a transitive default one dependency's PATCH release could change, \
             landing the breakage in a downstream crate that changed nothing. This check \
             asserts a manifest fact that no test and no compile can currently see: D12 cannot \
             be made to fail today, which is exactly why a conformance rule for it would be \
             decorative by construction (ADR-0016 §14). There is deliberately no WF-13; this \
             is a gate step instead.\n  found: `{line}`",
            match (line.contains("serde/alloc"), line.contains("base64/alloc")) {
                (false, false) => "both `serde/alloc` and `base64/alloc`",
                (false, true) => "`serde/alloc`",
                (true, false) => "`base64/alloc`",
                (true, true) => unreachable!("outer guard requires at least one to be missing"),
            }
        ),
        Some(line) => {
            println!("D12: {CORE_MANIFEST}'s `{line}` names both `serde/alloc` and `base64/alloc`");
            Ok(())
        }
    }
}

/// How much prose an entry must carry for each rule it names.
///
/// # Why a length and not a vocabulary
///
/// The obvious implementation is a keyword test — look for "defect", "rejects",
/// "detects", "the wrong implementation". It was written first and it was wrong,
/// which is worth recording because it is what the next person will reach for
/// too. Run against this repository's own changelog it produced **nine false
/// positives on nine entries that name a defect as well as anything in the
/// file**: "a condition probe that compares serialised tag sets with `=` instead
/// of matching supersets, so a stored event carrying an *extra* tag stops
/// violating a condition it does violate" contains not one word from any
/// plausible list. A check that fails on that prose does not teach anyone to
/// name defects; it teaches them to paste in the word "rejects".
///
/// So this measures the one thing a machine can actually see: whether the entry
/// naming the rule is a *sentence about it* or a name in a list. Calibration,
/// against the file as it stands: the thinnest entry a reader would call
/// adequate carries 168 characters per rule it names, and a bare
/// comma-separated list carries about fifty. 120 sits between them with margin
/// on both sides.
///
/// **What it does not verify, stated plainly:** that the prose names a defect,
/// that the defect is this rule's, or that it is true. It cannot. Those are
/// review's, CONTRIBUTING carries the obligation, and CF-29 says so. What this
/// catches is the failure that actually happens — a rule landing with no entry,
/// or with its name in a list — which was the state of twenty-five of the
/// fifty-five rules the first time it ran.
const MIN_CHARS_PER_RULE: usize = 120;

/// One changelog entry: a bullet and everything under it up to the next bullet.
struct Entry {
    line: usize,
    text: String,
}

/// Splits `CHANGELOG.md` into entries.
///
/// An entry starts at a line whose first non-space character begins a Markdown
/// bullet and ends at the next such line, the next heading, or the end of the
/// file. Nesting is deliberately *not* modelled: a nested bullet ends its
/// parent's entry, which makes the check finer-grained rather than coarser —
/// the concurrency family's entry names five rules in five nested bullets, each
/// with its own defect, and a parent-swallowing parse would let one defect
/// sentence answer for all five.
fn entries(changelog: &str) -> Vec<Entry> {
    let mut out: Vec<Entry> = Vec::new();
    for (n, line) in changelog.lines().enumerate() {
        let t = line.trim_start();
        let starts_entry = t.starts_with("- ") || t.starts_with("* ") || t.starts_with('#');
        if starts_entry || out.is_empty() {
            out.push(Entry {
                line: n + 1,
                text: String::new(),
            });
        }
        if let Some(entry) = out.last_mut() {
            entry.text.push(' ');
            entry.text.push_str(t);
        }
    }
    out
}

/// Whether a changelog entry names a rule, as a **whole** identifier.
///
/// A bare `contains` is a substring test, and conformance rule names nest.
/// `append_is_atomic` is a prefix of `append_is_atomic_under_a_mid_batch_fault`,
/// which is the only entry in the file carrying either name — so the longer
/// rule's entry silently discharged the shorter rule's obligation, and
/// `append_is_atomic`, the one thing in the suite that rejects the
/// write-then-check shape (§ES-18), went a whole phase with no entry of its own
/// while CF-29 reported it satisfied. The collision also inflated the *share*
/// arithmetic below, because the same entry was counted twice.
fn names_rule(text: &str, rule: &str) -> bool {
    let ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    text.match_indices(rule).any(|(at, _)| {
        let before = text[..at].chars().next_back().is_none_or(|c| !ident(c));
        let after = text[at + rule.len()..]
            .chars()
            .next()
            .is_none_or(|c| !ident(c));
        before && after
    })
}

/// CF-29: every conformance rule has a changelog entry naming a defect.
///
/// Two questions, and only the first has a clean answer: does the rule's name
/// appear in `CHANGELOG.md` at all, and is the entry naming it a sentence rather
/// than a listing. [`MIN_CHARS_PER_RULE`] carries the second, including why it
/// is a length rather than the vocabulary test written first.
///
/// # Every family, not just `suite.rs`
///
/// The rule set comes from [`all_rules`] and therefore from all three of
/// [`RULE_FILES`]. Reading `suite.rs` alone — which this did until stage 6's
/// review — meant a rule of the model or concurrency family could land with no
/// changelog entry at all while the step printed `all 55 suite rules have a
/// changelog entry`, a sentence that is true and answers a different question
/// than the one the step's name asks. The two newest families are exactly the
/// ones whose authors have had the least practice with the convention.
///
/// # Errors
///
/// Returns an error if either file cannot be read, or if any rule in
/// [`RULE_FILES`] has no changelog entry naming a defect.
pub(crate) fn changelog_names_every_rule() -> Result<()> {
    // Bundled here, not wired as its own step: `lint-changelog` is `cargo xtask
    // lints`'s one hook into this file for prose about the document as a whole.
    changelog_scope_matches_publishable()?;

    let root = workspace_root()?;
    let changelog =
        fs::read_to_string(root.join(CHANGELOG)).with_context(|| format!("reading {CHANGELOG}"))?;

    let rules = all_rules(&root)?;
    if rules.is_empty() {
        bail!(
            "parsed no rules from {} — CF-29's lint would pass vacuously",
            RULE_FILES.join(", ")
        );
    }
    let entries = entries(&changelog);

    // Per entry: which rules it names, and how much prose each of them gets. An
    // entry naming eight rules in one paragraph gives each an eighth of it,
    // which is the arithmetic that stops a single long bullet from answering for
    // a whole family.
    let shares: Vec<(usize, usize)> = entries
        .iter()
        .map(|e| {
            let named = rules.iter().filter(|r| names_rule(&e.text, r)).count();
            (named, e.text.chars().count() / named.max(1))
        })
        .collect();

    let mut problems = Vec::new();
    for rule in &rules {
        let naming: Vec<(&Entry, usize)> = entries
            .iter()
            .zip(&shares)
            .filter(|(e, _)| names_rule(&e.text, rule))
            .map(|(e, (_, share))| (e, *share))
            .collect();

        if naming.is_empty() {
            problems.push(format!(
                "{CHANGELOG} — `{rule}` is in the suite and in no changelog entry. CF-29: a rule \
                 lands with its mutant *and* an entry naming the defect it detects."
            ));
            continue;
        }
        if !naming.iter().any(|(_, share)| *share >= MIN_CHARS_PER_RULE) {
            let at: Vec<String> = naming
                .iter()
                .map(|(e, share)| format!("{} ({share} chars)", e.line))
                .collect();
            problems.push(format!(
                "{CHANGELOG}:{} — `{rule}` is named, but every entry naming it is a listing \
                 rather than a sentence about it. An adapter author whose CI has gone red needs \
                 the defect to tell \"my adapter is wrong\" from \"the bar moved\"; the name \
                 alone tells them neither.",
                at.join(", "),
            ));
        }
    }

    if !problems.is_empty() {
        for p in &problems {
            println!("  {p}");
        }
        bail!(
            "{} conformance rule(s) without a changelog entry naming a defect (CF-29).",
            problems.len()
        );
    }

    println!(
        "CF-29: all {} rules in {} file(s) have a changelog entry",
        rules.len(),
        RULE_FILES.len()
    );
    Ok(())
}

/// CF-6: no rule asserts on a literal sequence-position value.
///
/// # The variant is the check; this is the cheap second line
///
/// `GappedPositionStore` — positions from 4,096 in steps of seven — is what
/// actually enforces CF-6, and it already runs: a rule that asserts a literal
/// position passes against `MemoryEventStore` and fails against that store,
/// which is a *behavioural* refutation and not a spelling one. This grep exists
/// because the variant only rejects a literal that happens to be wrong for it,
/// and because the failure it produces names the rule rather than the habit.
///
/// **Deleting the variant and keeping this would be the wrong trade**, and it is
/// the trade a reader makes if the grep is described as the enforcement. It
/// catches two spellings and no semantics: an array literal of integers written
/// where a position list is expected, and a `SequencePosition` constructed from
/// a literal. It does not catch `let expected = 3;` two lines earlier, nor
/// `event.position.get() == 3`, nor a literal reached through a helper — every
/// one of which the variant still fails.
///
/// It *does* now catch a suffixed or non-decimal element, which it did not until
/// stage 6's review: `[1u64, 2, 3]` sat in the middle of the shape this claims to
/// catch, and passed, while the identical unsuffixed line fired. See
/// [`is_integer_list`] for why that spelling is the one an author reaches for.
///
/// # Errors
///
/// Returns an error if the suite cannot be read or cannot be scanned, or if a
/// position-shaped literal is found in its code.
pub(crate) fn no_position_literals() -> Result<()> {
    let root = workspace_root()?;
    let mut problems = Vec::new();
    for file in RULE_FILES {
        let body =
            fs::read_to_string(root.join(file)).with_context(|| format!("reading {file}"))?;
        scan_for_position_literals(file, &body, &mut problems)?;
    }

    if !problems.is_empty() {
        for p in &problems {
            println!("  {p}");
        }
        bail!(
            "{} literal position value(s) in the rule files (CF-6). The specification permits \
             gaps, so a rule that assumes density passes against the reference store and fails a \
             conformant adapter in the field.",
            problems.len()
        );
    }

    println!(
        "CF-6: no position-shaped literals in {} rule file(s)",
        RULE_FILES.len()
    );
    Ok(())
}

/// Whether a bracketed run is a list of integer literals.
///
/// # Why suffixes and radix prefixes count
///
/// `positions_of` returns `Vec<u64>` (`crates/happenstance-testkit/src/suite.rs:181`),
/// so `[1, 2, 3]` only compiles against it because inference obliges. The moment
/// it does not — a helper signature, a `matches!`, a generic comparison — the
/// spelling an author reaches for is `[1u64, 2, 3]`, and the digits-only test
/// this replaces passed it while firing on the identical unsuffixed line. Hex
/// went the same way, and `GappedPositionStore` starts at 4,096, which is the one
/// number in this repository somebody might write as `0x1000`.
///
/// It stays a *shape* test, not a parse: `[u8; 32]`, `[b'a', b'b']` and every
/// non-integer bracket are rejected by the token test rather than enumerated.
fn is_integer_list(content: &str) -> bool {
    /// Every integer suffix, longest-first so `u128` is stripped before `u1`
    /// could be (there is no `u1`, but the order is the invariant that keeps it
    /// true if one is ever added).
    const SUFFIXES: [&str; 12] = [
        "usize", "isize", "u128", "i128", "u64", "i64", "u32", "i32", "u16", "i16", "u8", "i8",
    ];

    let mut any = false;
    for token in content.split(',') {
        let token = token.trim();
        // A trailing comma is legal, so an empty tail is not a rejection.
        if token.is_empty() {
            continue;
        }
        let (body, radix) = match token.get(..2) {
            Some("0x" | "0X") => (&token[2..], 16),
            Some("0b" | "0B") => (&token[2..], 2),
            Some("0o" | "0O") => (&token[2..], 8),
            _ => (token, 10),
        };
        let body = SUFFIXES
            .iter()
            .find_map(|suffix| body.strip_suffix(suffix))
            .unwrap_or(body);
        let digits = |c: char| c.is_digit(radix);
        if !(body.chars().any(digits) && body.chars().all(|c| digits(c) || c == '_')) {
            return false;
        }
        any = true;
    }
    any
}

/// The two spellings [`no_position_literals`] rejects, over one file.
///
/// # Errors
///
/// Returns an error if the file contains a construct [`code_lines`] cannot lex.
fn scan_for_position_literals(file: &str, body: &str, problems: &mut Vec<String>) -> Result<()> {
    let lines = code_lines(file, body)?;
    let text: Vec<char> = lines.join("\n").chars().collect();
    let line_at = |i: usize| text[..i].iter().filter(|c| **c == '\n').count() + 1;

    // Shape one: a bracketed run of integer literals — `[1, 2, 3]`, which is the
    // spelling CF-6's own `Rejects:` line quotes. Indexing and slicing are
    // excluded by what precedes the bracket: `all[1]` and `all[..1]` are how the
    // suite *legitimately* anchors on a position the store assigned, so a check
    // that flagged them would be uninstallable.
    for (i, c) in text.iter().enumerate() {
        if *c != '[' {
            continue;
        }
        // `close` indexes into the slice, so the closing character sits at
        // `i + 1 + close` — an off-by-one here silently disables the whole shape
        // rather than misreporting, which is how it survived its own first run
        // and was caught only by the deliberate failure demonstration.
        let Some(close) = text[i + 1..].iter().position(|c| *c == ']' || *c == '[') else {
            continue;
        };
        let content: String = text[i + 1..=i + 1 + close].iter().collect();
        let Some(content) = content.strip_suffix(']') else {
            continue;
        };
        if !is_integer_list(content) {
            continue;
        }
        let indexes = text[..i]
            .iter()
            .rev()
            .find(|c| !c.is_whitespace())
            .is_some_and(|c| c.is_alphanumeric() || *c == '_' || *c == ']' || *c == ')');
        if indexes {
            continue;
        }
        problems.push(format!(
            "{file}:{} — `[{content}]` is a literal integer list. CF-6: anchor every position \
             assertion on a value the store under test assigned — compare against \
             `positions_of(&all[..n])`, never against numbers.",
            line_at(i)
        ));
    }

    // Shape two: a position built from a literal. A store is free to start at
    // 4,096, so `SequencePosition::new(1)` is a number the suite invented rather
    // than one it observed, whatever it is then compared against.
    let joined: String = text.iter().collect();
    for (i, _) in joined.match_indices("SequencePosition::") {
        let after = &joined[i..];
        let Some(open) = after.find('(') else {
            continue;
        };
        // Only the constructor call immediately following the path; anything
        // further away is a different expression.
        if after[..open].contains(['\n', ';']) {
            continue;
        }
        if after[open + 1..]
            .trim_start()
            .starts_with(|c: char| c.is_ascii_digit())
        {
            problems.push(format!(
                "{file}:{} — a `SequencePosition` built from a literal. CF-6: the only positions \
                 a rule may name are the ones the store handed it back.",
                joined[..i].matches('\n').count() + 1
            ));
        }
    }

    Ok(())
}

/// Words that make a paragraph a claim about the conformance suite.
///
/// Matched as lower-cased substrings, so `ProjectionStore` and `projection`
/// both hit. The scope is what keeps the check installable: the testkit's README
/// says *"Two rules govern what goes in it, both learned expensively"* about its
/// own contribution rules, in a paragraph naming none of these words, and a
/// check that fired there would be uninstallable for a true sentence.
const SUITE_WORDS: [&str; 3] = ["suite", "conformance", "projection"];

/// A number a document might spell as a word, `zero` through `ninety`.
///
/// Compounds are summed across hyphens, which is the only spelling English
/// writes them in: `eighty-nine` is 80 + 9, and the testkit's README opens on
/// exactly that word. Anything at or above one hundred must be written in
/// digits to be read — stated here rather than left as a surprise, because a
/// count this workspace could reach (`one hundred and twelve` rules across the
/// four files) is on the far side of that line.
fn cardinal(word: &str) -> Option<usize> {
    /// The words, paired with their values.
    const WORDS: [(&str, usize); 28] = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("ten", 10),
        ("eleven", 11),
        ("twelve", 12),
        ("thirteen", 13),
        ("fourteen", 14),
        ("fifteen", 15),
        ("sixteen", 16),
        ("seventeen", 17),
        ("eighteen", 18),
        ("nineteen", 19),
        ("twenty", 20),
        ("thirty", 30),
        ("forty", 40),
        ("fifty", 50),
        ("sixty", 60),
        ("seventy", 70),
        ("eighty", 80),
        ("ninety", 90),
    ];

    let word = word.to_ascii_lowercase();
    if word.is_empty() {
        return None;
    }
    let mut total = 0usize;
    for part in word.split('-') {
        let value = match part.parse::<usize>() {
            Ok(n) => n,
            Err(_) => WORDS.iter().find(|(w, _)| *w == part).map(|(_, n)| *n)?,
        };
        total = total.checked_add(value)?;
    }
    Some(total)
}

/// A word with the Markdown and punctuation around it removed.
fn bare(word: &str) -> &str {
    word.trim_matches(|c: char| !c.is_alphanumeric() && c != '-')
}

/// The prose of a document, whatever the document is made of.
///
/// Markdown passes through. Rust keeps its `//!` and `///` lines with the marker
/// stripped, and blanks everything else, so a paragraph never runs across a
/// function body. A manifest keeps its `#` comments the same way. Line numbering
/// is preserved in every case, because the whole value of a failure here is that
/// it names the line to open.
///
/// The rest of a Rust file is deliberately invisible: a doc comment is prose an
/// author writes for a reader, and code is not. `PROJECTION_MUST_REJECT` has
/// thirteen entries and no sentence claiming it.
fn prose_of(path: &str, body: &str) -> String {
    let keep_prefixed = |markers: &[&str]| {
        body.lines()
            .map(|line| {
                let t = line.trim_start();
                markers
                    .iter()
                    .find_map(|m| t.strip_prefix(m))
                    .unwrap_or("")
                    .trim()
                    .to_owned()
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    // The extension through `Path` rather than `str::ends_with`, which
    // `clippy::case_sensitive_file_extension_comparisons` denies: a `README.MD`
    // would take the Markdown arm on one filesystem and the fallthrough on
    // another, and this list is spelled by hand anyway.
    match Path::new(path).extension().and_then(|e| e.to_str()) {
        Some(e) if e.eq_ignore_ascii_case("rs") => keep_prefixed(&["//!", "///"]),
        Some(e) if e.eq_ignore_ascii_case("toml") => keep_prefixed(&["#"]),
        _ => body.to_owned(),
    }
}

/// Every `<cardinal> rule(s)` claim in a document that is about the suite, as
/// `(line, phrase, count)`.
///
/// Paragraphs rather than lines, because prose here wraps at 80 columns and the
/// sentence this check was written for put `ProjectionStore` on one line and
/// `two rules of seventeen` on the next. Each word keeps the line it was written
/// on, so a failure names the sentence rather than the top of a fifteen-line doc
/// block. Fenced code is skipped: a code sample is not a claim, and the README's
/// samples are the one place a bare number sits next to a rule name for reasons
/// that have nothing to do with counting.
fn rule_count_claims(md: &str) -> Vec<(usize, String, usize)> {
    let mut claims = Vec::new();
    let mut fenced = false;
    // One paragraph is a run of non-blank lines, as `(word, line)` pairs. A
    // blockquote's `>` is stripped so the status banner reads as prose, which is
    // what it is.
    let mut paragraphs: Vec<Vec<(String, usize)>> = vec![Vec::new()];
    for (n, raw) in md.lines().enumerate() {
        let line = raw.trim().trim_start_matches('>').trim();
        if raw.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced || line.is_empty() {
            paragraphs.push(Vec::new());
            continue;
        }
        if let Some(current) = paragraphs.last_mut() {
            current.extend(line.split_whitespace().map(|w| (w.to_owned(), n + 1)));
        }
    }

    for words in paragraphs {
        let about_the_suite = words.iter().any(|(w, _)| {
            let lowered = w.to_ascii_lowercase();
            SUITE_WORDS.iter().any(|s| lowered.contains(s))
        });
        if !about_the_suite {
            continue;
        }
        for (i, (word, line)) in words.iter().enumerate() {
            let noun = bare(word).to_ascii_lowercase();
            if noun != "rule" && noun != "rules" {
                continue;
            }
            // `one test per rule` is a rate rather than a count, and the README
            // states two of them. Excluded at the word `per` rather than by
            // demanding adjacency, because `sixteen projection rules` — the
            // spelling three of these documents went stale in — is a count with
            // an adjective in the way, and a check that could not read it would
            // teach the next author which spelling is unguarded.
            if i > 0 && bare(&words[i - 1].0).eq_ignore_ascii_case("per") {
                continue;
            }
            for back in 1..=2usize {
                let Some(candidate) = i.checked_sub(back).map(|j| bare(&words[j].0)) else {
                    break;
                };
                if let Some(count) = cardinal(candidate) {
                    claims.push((*line, format!("{candidate} {noun}"), count));
                    break;
                }
            }
        }
    }
    claims
}

/// The counts a claim about this workspace's rules may legitimately state.
///
/// One per rule file plus the union, each labelled, so the failure message can
/// print what the document *could* have meant instead of only what it may not
/// say. Derived from [`collect_rules`], the same parse CF-29 and `spec-trace`
/// run on, so landing a rule moves this bar without anyone editing this file —
/// which is the whole difference between a check and a second place to be stale.
fn true_rule_counts(root: &Path) -> Result<Vec<(usize, String)>> {
    let mut counts = Vec::new();
    for file in RULE_FILES {
        let body =
            fs::read_to_string(root.join(file)).with_context(|| format!("reading {file}"))?;
        let rules = collect_rules(&body);
        if rules.is_empty() {
            bail!("parsed no rules from {file} — this check would accept any count instead");
        }
        let name = file.rsplit('/').next().unwrap_or(file);
        counts.push((rules.len(), name.to_owned()));
    }
    counts.push((all_rules(root)?.len(), "all four rule files".to_owned()));
    counts.sort_unstable();
    Ok(counts)
}

/// Every document that tells a consumer how many rules there are.
///
/// The README is the one crates.io renders and the one that went stale; the
/// other three carry the same claim in the three other places a consumer meets
/// it — the crate page, the feature list they read before enabling anything, and
/// the manifest comment beside the feature itself. All four were wrong about the
/// projection suite at the same time, in three different file formats, which is
/// why this list is not just the README.
///
/// **`CHANGELOG.md` is deliberately absent.** Its entries are dated records of
/// what was true at a release, and a released entry saying *"sixteen rules"*
/// was true then. Holding it to today's count would demand rewriting history to
/// keep a check green, which is the opposite of what a changelog is for.
const COUNT_BEARING_DOCS: [&str; 4] = [
    TESTKIT_README,
    "crates/happenstance-testkit/src/lib.rs",
    "crates/happenstance-core/src/lib.rs",
    "crates/happenstance-core/Cargo.toml",
];

/// The rule counts a consumer is told are counts this workspace actually has.
///
/// # The defect, and why it needed a step rather than a reviewer
///
/// D11's `package-check` asserts `README.md` is inside the packaged artifact.
/// Presence is not truth, and the gap is not hypothetical: the README shipped
/// two sentences saying the projection suite was *"two rules of seventeen"*
/// through the fifteen commits that made it seventeen, and three consecutive
/// audits raised it as a finding rather than a red build. A finding raised three
/// times is a missing check, and the failure is worse than an ordinary stale
/// document — it tells an adapter author that a port they can build against does
/// not exist yet.
///
/// The sweep that landed this check found three more instances of the same
/// sentence in [`COUNT_BEARING_DOCS`], in Markdown, in rustdoc and in a manifest
/// comment. That is why the check reads all four rather than the one the audit
/// named: correcting an instance is not the same as retiring a class.
///
/// # Where it runs, and why that took a second correction
///
/// `REQUIRED` runs it as `lint-rule-counts`, and so does the story-grain gate
/// in `affected::run` — which is the entry point that matters most for it, and
/// the one it was missing from until the pre-publication review found it. The
/// change that makes a stated count wrong is a story landing a conformance
/// rule, and `cargo xtask affected` is the gate that story clears at every
/// `redkiln advance` seam; a check that runs only in the release gate reaches
/// the author after the branch is merged, if at all. `affected`'s own tests now
/// hold its unconditional block to every lint this module exports.
///
/// # What it does not verify
///
/// Only a **cardinal qualifying `rule` or `rules`** within two words, only in a
/// paragraph naming one of [`SUITE_WORDS`], and only in the four documents
/// above. It says nothing about counts of anything else — mutants, registry
/// rows, fixtures, adapters — nothing about a count above ninety-nine spelled in
/// words, and nothing at all about whether the surrounding sentence is
/// *otherwise* true. A README claiming the suite runs on Postgres sails through.
///
/// The narrowness is deliberate rather than provisional. Every number in these
/// files was a candidate; a check over all of them would have to be told which
/// numbers name what, and a check nobody can predict the verdict of gets
/// switched off the first time it is wrong. The counts of *rules* are the ones
/// that went stale, and they are the ones a machine can resolve to a single
/// authority.
///
/// # Errors
///
/// Returns an error if a document or a rule file cannot be read, if a rule file
/// parses to nothing, if a document states a rule count this workspace does not
/// have, or if **no** document states one at all — a check whose subject can
/// vanish is one that retires without anybody deciding to.
pub(crate) fn stated_rule_counts() -> Result<()> {
    // Bundled here rather than wired as its own step, the way
    // `changelog_scope_matches_publishable` is bundled into
    // `changelog_names_every_rule`: the same direction — a document held to the
    // tree — and the same two constraints on where it can go.
    //
    // It is not a `pub(crate) fn`, because `affected.rs`'s export scan
    // (`the_unconditional_block_runs_every_lint_the_module_exports`) requires
    // every entry point of that shape in this file to be named in the
    // story-grain block, and `xtask/src/affected.rs` is not this change's to
    // edit. And it is bundled *here*, past line 693, rather than into
    // `changelog_names_every_rule`, because `standards/rust/81-...:88-89` cites
    // `lints.rs:627` and `:693` by line and `lint-constitution` checks those
    // citations resolve; inserting above them moves the anchors out from under a
    // file this change may not correct.
    runbook_status_matches_the_registry()?;

    // V-6 (`references/evaluation/review-pre-publication-2026-09-03.md`): a
    // second, unrelated document-held-to-the-tree check, bundled for exactly
    // the same reason and under exactly the same constraint. Wiring
    // `citation_ranges_resolve` as its own `lint-*` step would mean adding a
    // `Step` to `main.rs`'s `REQUIRED` array and a match arm beside it, and
    // making it `pub(crate)` would obligate `affected.rs`'s export scan the
    // same way `runbook_status_matches_the_registry` is exempted from above —
    // neither file is this fix's to edit. `xtask/src/lints.rs` is; this is
    // its one door into `cargo xtask lints`.
    citation_ranges_resolve()?;

    let root = workspace_root()?;
    let counts = true_rule_counts(&root)?;
    let legend = counts
        .iter()
        .map(|(n, name)| format!("{n} ({name})"))
        .collect::<Vec<_>>()
        .join(", ");

    let mut checked = 0usize;
    let mut problems = Vec::new();
    for doc in COUNT_BEARING_DOCS {
        let body = fs::read_to_string(root.join(doc)).with_context(|| format!("reading {doc}"))?;
        for (line, phrase, count) in rule_count_claims(&prose_of(doc, &body)) {
            checked += 1;
            if !counts.iter().any(|(n, _)| *n == count) {
                problems.push(format!(
                    "{doc}:{line} — `{phrase}` names a count of conformance rules this workspace \
                     does not have. The counts it has are {legend}."
                ));
            }
        }
    }

    if checked == 0 {
        bail!(
            "none of {} states a rule count at all, so this check has nothing to hold. Say how \
             many rules the suites carry, or delete this step deliberately rather than by \
             omission.",
            COUNT_BEARING_DOCS.join(", ")
        );
    }

    if !problems.is_empty() {
        for p in &problems {
            println!("  {p}");
        }
        bail!(
            "{} stale rule count(s) in the documents a consumer reads. `cargo package --list` \
             asserts the README is in the artifact; only this step asserts it is true.",
            problems.len()
        );
    }

    println!("{checked} stated rule count(s) checked against {legend}");
    Ok(())
}

/// The testkit's own rustdoc — the second rendered surface C2-07 names,
/// alongside [`TESTKIT_README`].
///
/// `pub(crate)`, and the whole C2-07 group below with it up to
/// [`stale_publication_claims`]: the *check* itself — the
/// `Result<()>`-returning entry point — lives in `xtask/src/lint_pages.rs`
/// rather than here, deliberately. `exported_lints`
/// (`xtask/src/affected.rs:1016-1023`) scans this file for exactly the shape
/// `pub(crate) fn NAME() -> Result<()> {` and requires `affected::run`'s
/// unconditional block to call each one it finds by name — the invariant
/// that catches a lint wired into `REQUIRED` and forgotten in the story
/// grain. A seventh entry point of that shape here would trip the same check
/// for the wrong reason: it already runs in the story grain, transitively,
/// because `affected::run` calls `lint_pages::run` unconditionally
/// (`xtask/src/affected.rs:183`) and `lint_pages::run` calls this check.
/// `xtask/src/affected.rs` is not a path this change owns, so the fix is to
/// keep the checked shape out of this file rather than teach a name-matching
/// scanner about an indirection it cannot see through.
pub(crate) const TESTKIT_LIB: &str = "crates/happenstance-testkit/src/lib.rs";

/// The claim C2-07 is named for. `happenstance-testkit` has been on
/// crates.io at `0.2.0-alpha.1` since `448e1ac` (2026-08-16, recorded at
/// `CHANGELOG.md:306`), so a reader meeting this sentence on the rendered
/// page meets a claim the registry already contradicted the day it shipped.
pub(crate) const STALE_NOTHING_PUBLISHED: &str = "nothing in this workspace is published yet";

/// The commit that introduced the `factory =` keyword the rustdoc explains
/// the removal of.
pub(crate) const FACTORY_INTRODUCED: &str = "23fd446";

/// The commit that removed `factory =` — two days later and eight days
/// before the first publish, so no published version of
/// `happenstance-testkit` ever accepted it. That is the durable
/// justification C2-07's remediation asks for in place of a claim about the
/// registry: a fact about two commits cannot go stale in the direction
/// [`STALE_NOTHING_PUBLISHED`] did.
pub(crate) const FACTORY_REMOVED: &str = "1c1a6b7";

/// The claim the README's status blockquote carried. It is false for exactly
/// as long as [`SQLITE_CONFORMANCE_TEST`] exists: that file mounts the suite.
pub(crate) const STALE_NO_ADAPTER: &str = "No adapter has run this suite";

/// The file whose existence falsifies [`STALE_NO_ADAPTER`]. Its own rustdoc,
/// `crates/happenstance-sqlite/src/lib.rs:3`, opens `# Status: an adapter,
/// and it has run the suite`.
pub(crate) const SQLITE_CONFORMANCE_TEST: &str = "crates/happenstance-sqlite/tests/conformance.rs";

/// One line, its leading `///`, `//!` or `>` marker stripped and the line
/// dropped if that leaves nothing — so a phrase [`stale_publication_claims`]
/// searches for reads as one line even when the source hard-wraps it, the way
/// `crates/happenstance-testkit/README.md`'s blockquote splits `No adapter
/// has run this` from `suite` at a line boundary with `> ` in between.
///
/// Only one marker per line, and only a prefix: this is not a Markdown
/// parser, the way [`code_lines`] states plainly it is not a Rust lexer for
/// the same reason (RS-81-2, `standards/rust/81-checks-that-cannot-be-types.md:95`).
/// It is enough to make a substring search blind to hard-wrapping, and no
/// more.
fn unwrapped(text: &str) -> String {
    text.lines()
        .filter_map(|line| {
            let mut t = line.trim();
            for prefix in ["///", "//!", ">"] {
                if let Some(rest) = t.strip_prefix(prefix) {
                    t = rest.trim();
                    break;
                }
            }
            (!t.is_empty()).then(|| t.to_owned())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The problems C2-07 checks for, decided from text rather than paths — the
/// half `lint_pages::no_stale_publication_claims` hands to a filesystem read
/// is only `sqlite_conformance_exists`, so this half is unit-testable without
/// a workspace checkout.
///
/// # Why the rustdoc half also checks the *replacement*, not only the claim
///
/// A grep for [`STALE_NOTHING_PUBLISHED`] alone is satisfied by deleting the
/// sentence, which teaches nothing durable: C2-07's own "why a defect" names
/// the cost as the sentence reading as *licence* to remove a deprecated arm
/// again, and a deletion leaves no justification in its place for the next
/// person to read before doing that. So once the stale claim is gone, this
/// requires the two commits it should be replaced with — the fact that
/// cannot go stale in the same direction.
pub(crate) fn stale_publication_claims(
    lib: &str,
    readme: &str,
    sqlite_conformance_exists: bool,
) -> Vec<String> {
    let lib = unwrapped(lib);
    let readme = unwrapped(readme);
    let mut problems = Vec::new();

    if lib.contains(STALE_NOTHING_PUBLISHED) {
        problems.push(format!(
            "{TESTKIT_LIB} — claims `{STALE_NOTHING_PUBLISHED}`, but happenstance-testkit has \
             been on crates.io at 0.2.0-alpha.1 since 448e1ac (2026-08-16, CHANGELOG.md:306). \
             Ground the `factory =` justification in {FACTORY_INTRODUCED} and {FACTORY_REMOVED} \
             instead (C2-07)."
        ));
    } else if !lib.contains(FACTORY_INTRODUCED) || !lib.contains(FACTORY_REMOVED) {
        problems.push(format!(
            "{TESTKIT_LIB} — the stale `{STALE_NOTHING_PUBLISHED}` claim is gone, but the \
             migration note names neither {FACTORY_INTRODUCED} nor {FACTORY_REMOVED} — the two \
             commits its replacement justification must rest on (C2-07)."
        ));
    }

    if readme.contains(STALE_NO_ADAPTER) && sqlite_conformance_exists {
        problems.push(format!(
            "{TESTKIT_README} — claims `{STALE_NO_ADAPTER}`, but {SQLITE_CONFORMANCE_TEST} \
             already mounts it (C2-07)."
        ));
    }

    problems
}

/// Where `xtask/src/package.rs` declares the crates this workspace intends to
/// publish.
///
/// `PUBLISHABLE` itself is private to that module — on purpose, per RS-81-5:
/// `reconcile` already answers "does this list match what Cargo will publish",
/// and widening its visibility to answer a second, unrelated question (does
/// `CHANGELOG.md`'s scope line match it) would be a second copy of the const's
/// *meaning* reachable from two places that could drift from each other. This
/// reads the same bytes `reconcile` guards instead.
const PACKAGE_RS: &str = "xtask/src/package.rs";

/// Substrings found between successive pairs of `delim` in `text`.
///
/// A text unbalanced in `delim` (an odd count) silently stops after its last
/// complete pair — callers that need the whole list to be present check that
/// separately, the way [`publishable_from_package_rs`] and
/// [`changelog_scope_crates`] both do.
fn delimited(text: &str, delim: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find(delim) {
        let after = &rest[start + delim.len_utf8()..];
        let Some(end) = after.find(delim) else {
            break;
        };
        out.push(after[..end].to_string());
        rest = &after[end + delim.len_utf8()..];
    }
    out
}

/// The crate names inside `package::PUBLISHABLE`, read as source text.
///
/// # Errors
///
/// Returns an error if `xtask/src/package.rs` cannot be read, if it no longer
/// declares `const PUBLISHABLE: &[&str] = &[` with a matching `];`, or if that
/// span contains no quoted names. Any of the three would otherwise make this
/// silently compare `CHANGELOG.md`'s scope line against an empty or partial
/// list — passing over exactly the drift this check exists to catch.
fn publishable_from_package_rs(root: &Path) -> Result<Vec<String>> {
    const OPEN: &str = "const PUBLISHABLE: &[&str] = &[";

    let src = fs::read_to_string(root.join(PACKAGE_RS))
        .with_context(|| format!("reading {PACKAGE_RS}"))?;

    let after_open = src
        .find(OPEN)
        .with_context(|| format!("{PACKAGE_RS} — no `{OPEN}`; PUBLISHABLE moved or was renamed"))?
        + OPEN.len();
    let body = &src[after_open..];
    let close = body
        .find("];")
        .with_context(|| format!("{PACKAGE_RS} — PUBLISHABLE's `[` has no matching `];`"))?;

    let names = delimited(&body[..close], '"');
    if names.is_empty() {
        bail!("{PACKAGE_RS} — parsed zero crate names out of PUBLISHABLE");
    }
    Ok(names)
}

/// The crates `CHANGELOG.md`'s opening scope sentence names.
///
/// Reads only the *paragraph* starting at the line containing "Notable changes
/// to" — the contiguous run of non-blank lines from there to the next blank
/// line or end of file — not the whole file. Markdown wraps prose across lines,
/// and this sentence does; a single-line read would silently see only its first
/// clause. `happenstance-sqlite` also appears twice more in `CHANGELOG.md`,
/// both times inside historical entries about other crates, and neither
/// occurrence is a statement of what the document covers, which is why the scan
/// stops at the paragraph's blank-line boundary rather than continuing.
///
/// # Errors
///
/// Returns an error if no such paragraph exists, or if it names no crate at
/// all — both would otherwise make the comparison in
/// [`changelog_scope_matches_publishable`] pass vacuously.
fn changelog_scope_crates(changelog: &str) -> Result<Vec<String>> {
    let lines: Vec<&str> = changelog.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.contains("Notable changes to"))
        .with_context(|| format!("{CHANGELOG} — no scope sentence (\"Notable changes to\")"))?;
    let end = lines[start..]
        .iter()
        .position(|l| l.trim().is_empty())
        .map_or(lines.len(), |offset| start + offset);
    let paragraph = lines[start..end].join(" ");

    let names = delimited(&paragraph, '`');
    if names.is_empty() {
        bail!("{CHANGELOG} — the scope sentence names no crate: {paragraph:?}");
    }
    Ok(names)
}

/// The changelog's stated scope names exactly the crates this workspace
/// publishes.
///
/// Neither list is hard-coded here: [`publishable_from_package_rs`] reads
/// `PUBLISHABLE` and [`changelog_scope_crates`] reads the scope sentence, so a
/// crate promoted to publishable after this lands — the next `happenstance-*`
/// to drop its `publish = false` — fails this the same way `happenstance-sqlite`
/// does today, with no second edit required here. CLAUDE.md records why that
/// matters: a scope sentence spelling three crates by name already drifted once
/// when a fourth joined, because spelling the members is not what keeps a
/// sentence honest — something reading it is.
///
/// # What this does not verify
///
/// That `[Unreleased]` carries an entry for the newly-scoped crate's actual
/// changes, only that the document says the crate is in scope at all.
///
/// # Errors
///
/// Returns an error if either file cannot be read or parsed (see the two
/// functions above), or if the two crate sets disagree — the message names
/// which crates are on which side, because "missing from the scope line" and
/// "no longer publishable" are different bugs.
fn changelog_scope_matches_publishable() -> Result<()> {
    let root = workspace_root()?;
    let changelog =
        fs::read_to_string(root.join(CHANGELOG)).with_context(|| format!("reading {CHANGELOG}"))?;

    let publishable: BTreeSet<String> = publishable_from_package_rs(&root)?.into_iter().collect();
    let scoped: BTreeSet<String> = changelog_scope_crates(&changelog)?.into_iter().collect();

    let unscoped: Vec<&String> = publishable.difference(&scoped).collect();
    let stale: Vec<&String> = scoped.difference(&publishable).collect();

    if !unscoped.is_empty() || !stale.is_empty() {
        let mut msg = format!(
            "changelog_scope_matches_publishable: {CHANGELOG}'s scope sentence disagrees with \
             {PACKAGE_RS}'s PUBLISHABLE."
        );
        if !unscoped.is_empty() {
            let _ = write!(
                msg,
                " Publishable but not in scope: {unscoped:?} — add it to the scope sentence and \
                 give it `[Unreleased]` entries."
            );
        }
        if !stale.is_empty() {
            let _ = write!(
                msg,
                " In scope but not publishable: {stale:?} — either it lost `publish = false` \
                 without joining PUBLISHABLE, or the scope sentence is stale."
            );
        }
        bail!(msg);
    }

    println!(
        "changelog_scope_matches_publishable: {CHANGELOG}'s scope sentence names exactly \
         {PACKAGE_RS}'s {} publishable crate(s)",
        publishable.len()
    );
    Ok(())
}

/// The plan of record, whose `## Status` table routes the next contributor.
const RUNBOOK: &str = "RUNBOOK.md";

/// The four states `RUNBOOK.md`'s own legend permits, plus the em dash a
/// *milestone* row carries in place of one.
///
/// Asserted rather than assumed. The cheapest way to defeat everything below is
/// not to argue with it but to spell a state it does not recognise — `pending`,
/// `todo`, `not-started` — at which point a row that claims nothing passes a
/// check whose whole subject is what rows claim. An unrecognised state is a
/// problem, and the message says which words are allowed.
const PHASE_STATES: [&str; 4] = ["not started", "in progress", "blocked", "done"];

/// One row of `RUNBOOK.md`'s `## Status` table.
#[derive(Debug)]
struct PhaseRow {
    /// 1-based line number in `RUNBOOK.md`, so a problem names the line a
    /// reader will open.
    line: usize,
    /// The `#` cell: a phase number, or `—` for a milestone row.
    number: String,
    /// The `Phase` cell with its markdown emphasis and backticks removed.
    phase: String,
    /// The `Depends on` cell, split into the phase numbers it names.
    depends_on: Vec<String>,
    /// The `State` cell, emphasis stripped and lowercased.
    state: String,
    /// The row as written, for the crate-name scan in [`names_crate`].
    raw: String,
}

/// A cell with markdown emphasis, backticks and link syntax reduced to its text.
///
/// The milestone row spells its version bolded *and* backticked, and
/// `CHANGELOG.md`'s heading spells it bare; four phase rows spell their state
/// bolded and the legend spells it bare. Both differences are decoration a
/// reader does not see, so neither may be a difference this check sees either.
fn unmark(cell: &str) -> String {
    cell.replace(['*', '`'], "").trim().to_owned()
}

/// Whether `text` names `crate_name` as a whole crate name.
///
/// The boundary treats `-` as **part of** the name, which is the difference
/// between this and [`names_rule`] and the reason it is a separate function:
/// with `-` as a boundary character, `happenstance` matches inside
/// `happenstance-postgres` and every publishable crate would be found in every
/// row. Hyphens are name characters in a crate name, so they are name characters
/// here.
fn names_crate(text: &str, crate_name: &str) -> bool {
    let part = |c: char| c.is_ascii_alphanumeric() || c == '_' || c == '-';
    text.match_indices(crate_name).any(|(at, _)| {
        let before = text[..at].chars().next_back().is_none_or(|c| !part(c));
        let after = text[at + crate_name.len()..]
            .chars()
            .next()
            .is_none_or(|c| !part(c));
        before && after
    })
}

/// The rows of the `## Status` table, in order.
///
/// The scan starts at the `## Status` heading and takes the first contiguous run
/// of `|`-led lines after it, minus the header and separator rows. It stops at
/// the first line that is not a table row, which is what keeps it off the second
/// four-column table further down the same document — the effort estimates,
/// whose first cell is also a phase number and which would otherwise contribute
/// rows with no state at all.
///
/// # Errors
///
/// Returns an error if the heading is absent or the run holds no data rows:
/// either would make every check below pass over an empty list, which is the one
/// failure mode a table-reading check cannot report on its own.
fn phase_rows(runbook: &str) -> Result<Vec<PhaseRow>> {
    let lines: Vec<&str> = runbook.lines().collect();
    let heading = lines
        .iter()
        .position(|l| l.trim() == "## Status")
        .with_context(|| {
            format!("{RUNBOOK} — no `## Status` heading; the table moved or was renamed")
        })?;

    let mut rows = Vec::new();
    let mut seen_table = false;
    for (offset, line) in lines[heading + 1..].iter().enumerate() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            if seen_table {
                break;
            }
            continue;
        }
        seen_table = true;

        let cells: Vec<String> = trimmed
            .trim_matches('|')
            .split('|')
            .map(|c| c.trim().to_owned())
            .collect();
        if cells.len() < 5 {
            continue;
        }
        // The header row and the `|---|` separator, recognised by content rather
        // than by position so a blank line between heading and table cannot
        // shift the offsets.
        if cells[0] == "#" || cells[0].chars().all(|c| c == '-' || c == ':') {
            continue;
        }

        rows.push(PhaseRow {
            line: heading + 2 + offset,
            number: unmark(&cells[0]),
            phase: unmark(&cells[1]),
            depends_on: unmark(&cells[2])
                .split(',')
                .map(str::trim)
                .filter(|d| !d.is_empty() && *d != "—")
                .map(str::to_owned)
                .collect(),
            state: unmark(&cells[3]).to_lowercase(),
            raw: trimmed.to_owned(),
        });
    }

    if rows.is_empty() {
        bail!("{RUNBOOK} — the `## Status` heading is followed by no table rows");
    }
    Ok(rows)
}

/// The versions `CHANGELOG.md` records as **released** — every `## [x]` heading
/// carrying a date, and never `[Unreleased]`.
///
/// A dated heading is the document's own statement that the version left this
/// repository. That is the fact the status table is held to below: work that
/// shipped cannot be work nobody has begun.
fn released_versions(changelog: &str) -> Vec<String> {
    changelog
        .lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("## [")?;
            let (tag, after) = rest.split_once(']')?;
            (tag != "Unreleased" && after.contains('—')).then(|| tag.to_owned())
        })
        .collect()
}

/// Every phase a row depends on, transitively, through the `Depends on` column.
///
/// A milestone row names only its immediate prerequisite; the phases *that*
/// phase waited on shipped with it just as surely, so the closure is what the
/// released version actually vouches for.
fn prerequisites(rows: &[PhaseRow], start: &[String]) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    let mut queue: Vec<String> = start.to_vec();
    while let Some(number) = queue.pop() {
        if seen.contains(&number) {
            continue;
        }
        if let Some(row) = rows.iter().find(|r| r.number == number) {
            queue.extend(row.depends_on.iter().cloned());
        }
        seen.push(number);
    }
    seen.sort_unstable();
    seen
}

/// `RUNBOOK.md`'s status table may not disagree with the registry and the
/// changelog about what has shipped.
///
/// # The defect, and why prose could not hold it
///
/// This repository's session protocol routes the next contributor by that table.
/// A row that reads `not started` about work that is finished does not merely
/// age — it sends somebody to build a crate that is already on crates.io. That
/// is what it was doing: phases 6, 7, 8 and 9 all read `not started` while
/// `0.2.0-alpha.1` was released, `happenstance-sqlite` was in
/// [`publishable_from_package_rs`]'s list, and the suite it exists to pass was
/// mounted in its own `tests/`. Every other reader of that table is a person,
/// and four consecutive documents' worth of people read past it.
///
/// # The two axes, and why neither is a judgement
///
/// Both compare the table against a file outside it, so neither can be settled
/// by rewording the table.
///
/// 1. **A released version vouches for its prerequisites.** `CHANGELOG.md`
///    records `0.2.0-alpha.1` with a date. The milestone row naming that version
///    depends on phase 7, which depends on 4 and 6, and so on: every phase in
///    that closure shipped when the version did, so none of them may be anything
///    but `done`.
/// 2. **A publishable crate is not unbegun work.** A phase row naming a crate in
///    `xtask/src/package.rs`'s `PUBLISHABLE` — the same constant
///    [`changelog_scope_matches_publishable`] treats as the authority on what
///    this workspace ships — may not read `not started`.
///
/// # What this does not verify
///
/// It does not catch a phase whose row names no crate and which no released
/// version depends on. Phase 9 is exactly that today: `happenstance-cloudflare`
/// is publishable and its Durable Object conformance suite runs, but the row
/// spells neither the crate nor a version, so axis 2 has nothing to match and
/// axis 1 does not reach it. Nor does it check the *other* direction — a row
/// reading `done` over work that is not — because nothing mechanical in this
/// tree distinguishes an unwritten phase from an unfinished one, and a check
/// whose verdict cannot be predicted gets switched off the first time it is
/// wrong.
///
/// Axis 2 scans the **whole** row, proof-artefact cell included, and phase 5's
/// cell names `happenstance-core` for that reason — two mentions are found
/// today, not one. That direction is the safe one: a future phase that is
/// genuinely unstarted and happens to name a published crate in its criterion
/// fires a false positive, which a reader resolves by reading the row, and not
/// a false negative, which nobody sees at all.
///
/// # Errors
///
/// Returns an error if either document cannot be read or parsed, if a row states
/// a state outside [`PHASE_STATES`], if no released version matches a milestone
/// row or no row names a publishable crate — either of which would leave an axis
/// with nothing to hold — or if the table disagrees with the changelog or the
/// registry on any row.
fn runbook_status_matches_the_registry() -> Result<()> {
    let root = workspace_root()?;
    let runbook =
        fs::read_to_string(root.join(RUNBOOK)).with_context(|| format!("reading {RUNBOOK}"))?;
    let changelog =
        fs::read_to_string(root.join(CHANGELOG)).with_context(|| format!("reading {CHANGELOG}"))?;
    let publishable = publishable_from_package_rs(&root)?;

    let rows = phase_rows(&runbook)?;
    let mut problems = Vec::new();

    for row in &rows {
        let milestone = row.number == "—";
        if !(PHASE_STATES.contains(&row.state.as_str()) || (milestone && row.state == "—")) {
            problems.push(format!(
                "{RUNBOOK}:{} — `{}` states `{}`, which is not one of {}. A state this table's \
                 own legend does not name is a row that claims nothing, and every check here \
                 passes over it.",
                row.line,
                row.phase,
                row.state,
                PHASE_STATES.join(", ")
            ));
        }
    }

    // Axis 1 — a released version vouches for every phase it waited on.
    let mut vouched = 0usize;
    for version in released_versions(&changelog) {
        let Some(milestone) = rows.iter().find(|r| r.phase == version) else {
            continue;
        };
        vouched += 1;
        for number in prerequisites(&rows, &milestone.depends_on) {
            let Some(row) = rows.iter().find(|r| r.number == number) else {
                problems.push(format!(
                    "{RUNBOOK}:{} — `{}` depends on phase {number}, which the table has no row \
                     for.",
                    milestone.line, version
                ));
                continue;
            };
            if row.state != "done" {
                problems.push(format!(
                    "{RUNBOOK}:{} — phase {number} reads `{}`, but {CHANGELOG} records `{version}` \
                     as released and that version's row waits on it. Work that shipped is not \
                     work nobody has started.",
                    row.line, row.state
                ));
            }
        }
    }
    if vouched == 0 {
        bail!(
            "runbook_status_matches_the_registry: no released version in {CHANGELOG} matches a \
             row of {RUNBOOK}'s status table, so the released-version axis holds nothing. Name \
             the release in the milestone row, or delete this axis deliberately rather than by \
             omission."
        );
    }

    // Axis 2 — a crate this workspace publishes is not unbegun work.
    let mut named = 0usize;
    for row in &rows {
        for crate_name in publishable.iter().filter(|c| names_crate(&row.raw, c)) {
            named += 1;
            if row.state == "not started" {
                problems.push(format!(
                    "{RUNBOOK}:{} — phase {} reads `not started` and names `{crate_name}`, which \
                     {PACKAGE_RS}'s PUBLISHABLE says this workspace publishes. A crate on \
                     crates.io is not a phase nobody has started.",
                    row.line, row.number
                ));
            }
        }
    }
    if named == 0 {
        bail!(
            "runbook_status_matches_the_registry: no row of {RUNBOOK}'s status table names any of \
             {PACKAGE_RS}'s publishable crates, so the registry axis holds nothing."
        );
    }

    if !problems.is_empty() {
        for p in &problems {
            println!("  {p}");
        }
        bail!(
            "runbook_status_matches_the_registry: {} row(s) of {RUNBOOK}'s status table disagree \
             with {CHANGELOG} or {PACKAGE_RS} about what has shipped. This repository routes its \
             next contributor by that table.",
            problems.len()
        );
    }

    println!(
        "runbook_status_matches_the_registry: {} status row(s) agree with {vouched} released \
         version(s) and {named} publishable-crate mention(s)",
        rows.len()
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// CF-24 — no rule may exist without appearing in its family's enumeration
// ---------------------------------------------------------------------------

/// One file of the workspace, read relative to its root.
fn at(root: &Path, rel: &str) -> Result<String> {
    fs::read_to_string(root.join(rel)).with_context(|| format!("reading {rel}"))
}

/// One rule family: the file that defines it, and the macro that must list it.
struct Enumeration {
    /// The file whose `pub async fn` items are the family's rules.
    file: &'static str,
    /// The `for_each_*!` macro that must name every one of them.
    macro_name: &'static str,
    /// The file that macro is defined in, which is not always the rule file:
    /// `for_each_event_store_rule!` lives in `registry.rs` and enumerates
    /// `suite.rs`.
    macro_file: &'static str,
}

/// The families this check covers.
///
/// **Two of five in this commit, which is the finding.** CF-24 is `[FROZEN]` and
/// its deferral paragraph says the mechanical fix was left because "a third
/// family is plausible in phase 4 and one scanner written against three is
/// better than three written one at a time". Three more families arrived. The
/// third got a *second* scanner — `no_orphan_projection_rules`, which is
/// `registry.rs`'s `declared_rules` verbatim but for the `include_str!`
/// argument — and the other two got none.
const ENUMERATIONS: [Enumeration; 2] = [
    Enumeration {
        file: "crates/happenstance-testkit/src/suite.rs",
        macro_name: "for_each_event_store_rule",
        macro_file: "crates/happenstance-testkit/src/registry.rs",
    },
    Enumeration {
        file: "crates/happenstance-testkit/src/projection.rs",
        macro_name: "for_each_projection_store_rule",
        macro_file: "crates/happenstance-testkit/src/projection.rs",
    },
];

/// CF-24: every rule a family defines appears in that family's enumeration, and
/// every name the enumeration lists is a rule the family defines.
///
/// # Errors
///
/// Returns an error if a file cannot be read, if a macro cannot be located, or
/// if either direction of the comparison finds anything.
pub(crate) fn rules_are_enumerated() -> Result<()> {
    let root = workspace_root()?;
    let mut problems: Vec<String> = Vec::new();
    let mut checked = 0usize;

    for family in &ENUMERATIONS {
        let defined = collect_rules(&at(&root, family.file)?);
        let listed = enumerated_names(
            &at(&root, family.macro_file)?,
            family.macro_file,
            family.macro_name,
        )?;
        checked += defined.len();

        for rule in defined.difference(&listed) {
            problems.push(format!(
                "{} — `{rule}` is defined and `{}!` does not name it. CF-24: a rule written, \
                 reviewed, merged, and never run because its registration line was forgotten.",
                family.file, family.macro_name
            ));
        }
        for name in listed.difference(&defined) {
            problems.push(format!(
                "{} — `{}!` names `{name}`, which {} does not define.",
                family.macro_file, family.macro_name, family.file
            ));
        }
    }

    if !problems.is_empty() {
        for p in &problems {
            println!("  {p}");
        }
        bail!(
            "{} rule(s) out of step with their family's enumeration (CF-24)",
            problems.len()
        );
    }
    println!(
        "CF-24: {checked} rule(s) across {} enumeration(s) appear in the macro that drives them",
        ENUMERATIONS.len()
    );
    Ok(())
}

/// The names a `for_each_*!` macro lists.
///
/// The shape is fixed across all five macros and rustfmt pins it: the body is
/// `$($callback)+! {` followed by one bare identifier per line, comma-separated,
/// closing on a line that is `}` at the macro's own indentation. Comments are
/// blanked by [`code_lines`] first, because two of the five carry section
/// headings inside the list and a raw split on commas takes a heading for a
/// name.
///
/// # Errors
///
/// Returns an error if the macro is not in the file, or if its body does not
/// have that shape — never an empty set, which would make the check pass by
/// finding nothing.
fn enumerated_names(source: &str, file: &str, macro_name: &str) -> Result<BTreeSet<String>> {
    let lines = code_lines(file, source)?;
    let Some(start) = lines
        .iter()
        .position(|l| l.contains(&format!("macro_rules! {macro_name}")))
    else {
        bail!("{file} defines no `macro_rules! {macro_name}`");
    };
    let Some(open) = lines[start..]
        .iter()
        .position(|l| l.contains("$($callback)+! {"))
        .map(|at| start + at)
    else {
        bail!("`{macro_name}!` in {file} has no `$($callback)+! {{` body");
    };
    let Some(close) = lines[open + 1..]
        .iter()
        .position(|l| l.trim() == "}")
        .map(|at| open + 1 + at)
    else {
        bail!("`{macro_name}!` in {file} has no closing brace for its list");
    };

    let mut out = BTreeSet::new();
    for name in lines[open + 1..close]
        .join(" ")
        .split(',')
        .map(str::trim)
        .filter(|n| !n.is_empty())
    {
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            bail!("`{macro_name}!` in {file} lists `{name}`, which is not a bare identifier");
        }
        out.insert(name.to_owned());
    }
    if out.is_empty() {
        bail!("`{macro_name}!` in {file} lists nothing — a check that finds no names passes");
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// V-6 — a citation names a place, and the place may have moved
// ---------------------------------------------------------------------------

/// Where `citation_ranges_resolve` looks for `path:START-END` citations.
///
/// V-6's own remediation names these three. Two subtrees of `.kb/` are
/// excluded even though the constant says `.kb/` whole:
/// `.kb/_governance/integration-waves/` is a wave's own audit trail — dated
/// minutes of what a corpus looked like at ingest time, read under the same
/// rule as `references/evaluation/*` (RS-01-3,
/// `standards/rust/01-standard-of-evidence.md:132`) rather than held current —
/// and `.kb/_intake/` is staging that `/redkiln:kb-ingest` clears (CLAUDE.md's
/// repository map). Scanning either would hold this lint to citations nobody
/// is keeping in step with the tree, which is a different failure from V-6's:
/// noisy rather than blind.
const CITATION_SCAN_DIRS: [&str; 3] = ["examples", "docs", ".kb"];

/// Repository-relative subtrees [`CITATION_SCAN_DIRS`] does not descend into.
/// See that constant's doc comment for why.
const CITATION_SCAN_EXCLUDE: [&str; 2] = [".kb/_governance", ".kb/_intake"];

/// A `path:START` or `path:START-END` citation, and the line of the scanned
/// file it was written on (for the failure message, not for the check).
struct Citation {
    written_at: usize,
    target: String,
    start: usize,
    end: usize,
}

/// A backtick-quoted span, parsed as a citation if it has the shape
/// `some/path.rs:START` or `some/path.rs:START-END`.
///
/// # What this does not parse, on purpose
///
/// A path with no `/` — `store.rs:205`, the shape a citation takes when it
/// means "relative to the crate this prose already sits inside", which
/// several citations in this workspace's own `.kb/` use — is not recognised
/// at all: without a directory to anchor it, the *right* root is exactly the
/// fact this scanner cannot know without reading the paragraph around it, and
/// guessing would trade a blind spot for a false positive. It is a
/// documented gap, not a silent one.
fn parse_citation(span: &str, written_at: usize) -> Option<Citation> {
    let colon = span.rfind(':')?;
    let path = &span[..colon];
    let rest = &span[colon + 1..];
    let ext = Path::new(path).extension().and_then(|e| e.to_str());
    let ext_ok =
        matches!(ext, Some(e) if e.eq_ignore_ascii_case("rs") || e.eq_ignore_ascii_case("md"));
    if !path.contains('/') || !ext_ok {
        return None;
    }
    let path_chars_ok = path
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '.' | '_' | '-'));
    if !path_chars_ok {
        return None;
    }
    let (start_s, end_s) = rest.split_once('-').unwrap_or((rest, rest));
    let start: usize = start_s.parse().ok()?;
    let end: usize = end_s.parse().ok()?;
    if start == 0 || end < start {
        return None;
    }
    Some(Citation {
        written_at,
        target: path.to_owned(),
        start,
        end,
    })
}

/// Every citation on one already-de-commented line of prose.
///
/// No state carried between lines, and that is a second documented gap
/// rather than an oversight: V-6's own fixtures write
/// ``` `examples/course-subscriptions/src/main.rs:194-207` (the domain enum)
/// and `:341-379` (the fold) ```, and the second span names no path of its
/// own — it means "the same file as the citation before it" to a reader, and
/// nothing here tracks that. It is caught only because the *first* citation
/// on the same line is independently stale.
fn citations_in_line(line: &str, written_at: usize, out: &mut Vec<Citation>) {
    let mut rest = line;
    while let Some(open) = rest.find('`') {
        let after = &rest[open + 1..];
        let Some(close) = after.find('`') else {
            break;
        };
        if let Some(citation) = parse_citation(&after[..close], written_at) {
            out.push(citation);
        }
        rest = &after[close + 1..];
    }
}

/// A trimmed Rust source line that reads as a continuation of an enclosing
/// construct rather than the start of one.
///
/// # What this catches, and what a stronger version of it broke
///
/// Exactly V-6's own shape: `Self::CourseDefined { .. } =>
/// Self::EVENT_TYPES[0].clone(),` is a match arm, and `Self::`/`self.` are
/// expression positions — they cannot open an item, an attribute or a doc
/// comment, so seeing one at the start of a cited line is unambiguous. That
/// is the whole signal, and it is deliberately the only one. A first version
/// of this function instead required the line to *open* with a keyword
/// (`fn`, `struct`, `///`, ...) and rejected everything else; run once
/// against every citation [`citation_ranges_resolve`] currently finds under
/// [`CITATION_SCAN_DIRS`], it flagged a bare `///` continuation line inside a
/// multi-paragraph doc comment and a snippet deliberately cited from the
/// middle of a chained call — both legitimate citations already in this
/// tree, and both would have needed a fix this change does not own to reach
/// green. This function does not catch V-6's *other* citation
/// (`main.rs:341`, a bare `match event {`) for the same reason: `match` is
/// too common a legitimate citation target to blacklist safely.
fn looks_mid_construct(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with("Self::") || t.starts_with("self.")
}

/// Whether a citation's range is defensible: present in `lines`, non-blank at
/// its first line, and — only when the target is a Rust source file — not
/// [`looks_mid_construct`] there.
///
/// Pure and filesystem-free on purpose, so every case below is a four-line
/// fixture rather than a file on disk: [`citation_ranges_resolve`] is the one
/// caller that touches a filesystem.
///
/// # What this does not verify
///
/// That the citation says what the sentence around it claims — only that the
/// range it names exists and its first line is not obviously the wrong kind
/// of place to send a reader. A citation that resolves cleanly to the wrong
/// construct entirely (a real method that is not the one under discussion)
/// passes. Confirming the claim is what a reviewer is for; this catches the
/// cheaper, mechanical failure of a citation nobody re-pointed after the file
/// moved under it.
fn citation_resolves(
    lines: &[&str],
    start: usize,
    end: usize,
    is_rust: bool,
) -> Result<(), String> {
    if start == 0 || start > lines.len() || end > lines.len() {
        return Err(format!(
            "range {start}-{end} is out of bounds ({} line(s) in the target)",
            lines.len()
        ));
    }
    if !is_rust {
        return Ok(());
    }
    let first = lines[start - 1];
    if first.trim().is_empty() {
        return Err(format!("line {start} is blank"));
    }
    if looks_mid_construct(first) {
        return Err(format!(
            "line {start} looks mid-construct: `{}`",
            first.trim()
        ));
    }
    Ok(())
}

/// One citation, read against the file it names.
///
/// `None` covers two different situations on purpose, and both are
/// documented gaps rather than passes: the citation resolved cleanly, or its
/// path does not exist under the repository root at all — `src/main.rs`
/// written inside a citation that means "relative to this example's own
/// crate", for one, which is a shape this workspace's own `.kb/` already
/// uses. An unresolved path is not evidence of a stale citation; it is
/// evidence this scanner was not told which root to read it against, so it
/// says nothing rather than guessing wrong.
fn check_citation(root: &Path, citing_file: &str, citation: &Citation) -> Option<String> {
    let body = fs::read_to_string(root.join(&citation.target)).ok()?;
    let lines: Vec<&str> = body.lines().collect();
    let is_rust = Path::new(&citation.target)
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("rs"));
    citation_resolves(&lines, citation.start, citation.end, is_rust)
        .err()
        .map(|why| {
            let range = if citation.start == citation.end {
                citation.start.to_string()
            } else {
                format!("{}-{}", citation.start, citation.end)
            };
            format!(
                "{citing_file}:{} — cites `{}:{range}`, {why}",
                citation.written_at, citation.target
            )
        })
}

/// Every `.rs` or `.md` file under `root.join(dir)`, sorted, skipping any
/// subtree whose repository-relative path is (or is under) one of `exclude`.
fn citation_source_files(
    root: &Path,
    dir: &str,
    exclude: &[&str],
) -> Result<Vec<std::path::PathBuf>> {
    let mut out = Vec::new();
    let mut stack = vec![root.join(dir)];
    while let Some(next) = stack.pop() {
        for entry in fs::read_dir(&next).with_context(|| format!("reading {}", next.display()))? {
            let path = entry
                .with_context(|| format!("reading an entry of {}", next.display()))?
                .path();
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .display()
                .to_string()
                .replace('\\', "/");
            if exclude
                .iter()
                .any(|e| rel == *e || rel.starts_with(&format!("{e}/")))
            {
                continue;
            }
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs" || e == "md") {
                out.push(path);
            }
        }
    }
    out.sort();
    Ok(out)
}

/// V-6: every `path:START-END` citation under [`CITATION_SCAN_DIRS`] resolves
/// to a range that exists, is non-blank at its first line, and — when the
/// target is a `.rs` file — is not [`looks_mid_construct`] there.
///
/// # Why this and not a general citation lint
///
/// V-6's own routing is explicit that it is not deciding this: "whether the
/// gate acquires a general citation lint is a decision for the RUNBOOK's pass
/// on gate scope... and this document does not take it." This check is scoped
/// to exactly the three directories and the two checks the finding names, and
/// no wider — see [`parse_citation`], [`citations_in_line`],
/// [`looks_mid_construct`] and [`check_citation`] for the blind spots that
/// scoping leaves, each stated where the decision was made rather than left
/// for a reader to discover by watching the check pass over something wrong.
///
/// # Errors
///
/// Returns an error if a scanned or cited file cannot be read, or if any
/// citation's range fails [`citation_resolves`].
fn citation_ranges_resolve() -> Result<()> {
    let root = workspace_root()?;
    let mut files = Vec::new();
    for dir in CITATION_SCAN_DIRS {
        files.extend(citation_source_files(&root, dir, &CITATION_SCAN_EXCLUDE)?);
    }
    if files.is_empty() {
        bail!(
            "{} hold no .rs or .md files between them — V-6's citation lint would scan nothing",
            CITATION_SCAN_DIRS.join(", ")
        );
    }

    let mut problems = Vec::new();
    let mut checked = 0usize;
    for path in &files {
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .display()
            .to_string()
            .replace('\\', "/");
        let body = fs::read_to_string(path).with_context(|| format!("reading {rel}"))?;
        let prose = prose_of(&rel, &body);
        let mut citations = Vec::new();
        for (n, line) in prose.lines().enumerate() {
            citations_in_line(line, n + 1, &mut citations);
        }
        for citation in &citations {
            checked += 1;
            if let Some(problem) = check_citation(&root, &rel, citation) {
                problems.push(problem);
            }
        }
    }

    if !problems.is_empty() {
        for p in &problems {
            println!("  {p}");
        }
        bail!(
            "{} citation(s) under {} whose cited range does not resolve cleanly (V-6). A citation \
             is the guard against drift only for as long as it points at what it claims to.",
            problems.len(),
            CITATION_SCAN_DIRS.join(", ")
        );
    }

    println!(
        "V-6: {checked} path:line citation(s) across {} file(s) in {} resolve cleanly",
        files.len(),
        CITATION_SCAN_DIRS.join(", ")
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    /// The counts this workspace had when these tests were written are not the
    /// point — the shapes are — so the fixtures state their own.
    const COUNTS: [usize; 2] = [17, 89];

    fn stale(md: &str) -> Vec<String> {
        rule_count_claims(md)
            .into_iter()
            .filter(|(_, _, n)| !COUNTS.contains(n))
            .map(|(_, phrase, _)| phrase)
            .collect()
    }

    /// The sentence this check exists for, verbatim from
    /// `crates/happenstance-testkit/README.md` as `79df6b7` wrote it and as the
    /// crate shipped it for fifteen commits afterwards.
    ///
    /// It is the wrong implementation the check is required to reject, and it is
    /// quoted rather than paraphrased because the wrap is the hard part: the
    /// subject that scopes the paragraph is on one line and the false count is
    /// on the next.
    #[test]
    fn the_shipped_sentence_this_check_was_written_for_is_rejected() {
        let md = "\
> What is still early is everything around that. **No adapter has run this
> suite**; the workspace's storage crates are skeletons. The `ProjectionStore`
> suite exists but is two rules of seventeen, and neither has been shown to
> reject a wrong store yet.
";
        assert_eq!(stale(md), vec!["two rules"]);
    }

    /// The second one, which is bold, mid-sentence and on one line.
    #[test]
    fn a_bolded_count_is_read_through_its_markdown() {
        let md = "**Two rules of the seventeen the specification names, today.** The port is\n\
                  `[PROVISIONAL]` and this suite is what will freeze it.\n";
        assert_eq!(stale(md), vec!["Two rules"]);
    }

    /// The corrected spelling, and the one at the top of the same file.
    #[test]
    fn a_count_the_enumeration_supports_passes() {
        let md = "The conformance suite for happenstance event store adapters. Eighty-nine\n\
                  rules, each tracing to a MUST.\n\n\
                  The `ProjectionStore` suite is all seventeen rules the specification names.\n";
        assert!(stale(md).is_empty(), "{:?}", stale(md));
        assert_eq!(rule_count_claims(md).len(), 2);
    }

    /// The near miss that set the scope, and the reason it is a paragraph test
    /// rather than a file-wide one: the same README says this, truthfully, about
    /// its own contribution rules.
    #[test]
    fn a_paragraph_that_is_not_about_the_suite_is_not_a_claim() {
        let md = "Two rules govern what goes in it, both learned expensively:\n";
        assert!(rule_count_claims(md).is_empty());
    }

    /// The spelling with an adjective in the way, which is the one the CHANGELOG
    /// went stale in — *"passing all sixteen projection rules"* — and the rate
    /// beside it, which is not a count at all and appears twice in the README.
    #[test]
    fn an_adjective_does_not_hide_a_count_and_a_rate_is_not_one() {
        let stale_count = "A projection example passing all sixteen projection rules.\n";
        assert_eq!(stale(stale_count), vec!["sixteen rules"]);

        let rate = "The projection suite expands to one test per rule, so a failure names it.\n";
        assert!(rule_count_claims(rate).is_empty(), "{:?}", stale(rate));
    }

    /// A code sample is not a claim. `event_store_conformance!` puts the word
    /// `conformance` in the paragraph, so without the fence test the sample
    /// would be scanned as prose.
    #[test]
    fn fenced_code_is_not_prose() {
        let md = "```rust,ignore\n\
                  // 2 rules, in the conformance suite\n\
                  happenstance_testkit::event_store_conformance!(MyFixture::new());\n\
                  ```\n";
        assert!(rule_count_claims(md).is_empty());
    }

    /// The other two formats a stale count was found in. A `//!` block is prose
    /// and is read; the code under it is not prose and is not, which is what
    /// stops `PROJECTION_MUST_REJECT`'s thirteen entries from reading as a claim
    /// about thirteen rules.
    #[test]
    fn rustdoc_and_manifest_comments_are_prose_and_code_is_not() {
        let rust = "\
//! * **`unstable-projection`** — the port. The reason is not that nothing
//!   tests it: sixteen conformance rules do.
const PROJECTION_RULES: [&str; 2] = [\"one\", \"two\"];
";
        let claims = rule_count_claims(&prose_of("crates/happenstance-core/src/lib.rs", rust));
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].2, 16);
        assert_eq!(claims[0].0, 2, "the failure must name the line to open");

        let manifest = "\
# The `ProjectionStore` port. The suite exists and sixteen rules drive it.
unstable-projection = []
";
        let claims = rule_count_claims(&prose_of("crates/happenstance-core/Cargo.toml", manifest));
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].2, 16);
    }

    /// Digits and words are the same claim, and a hyphenated compound is one
    /// number rather than two.
    #[test]
    fn cardinals_are_read_in_both_spellings() {
        assert_eq!(cardinal("17"), Some(17));
        assert_eq!(cardinal("seventeen"), Some(17));
        assert_eq!(cardinal("Eighty-nine"), Some(89));
        assert_eq!(cardinal("the"), None);
        assert_eq!(cardinal(""), None);
    }

    /// The exact sentence C2-07 was written for, verbatim from
    /// `crates/happenstance-testkit/src/lib.rs:486-489` as it stood at HEAD
    /// before the fix. Quoted rather than paraphrased for the same reason
    /// `the_shipped_sentence_this_check_was_written_for_is_rejected` above
    /// quotes its README sentence: the wrap is the hard part to reproduce by
    /// hand.
    #[test]
    fn c2_07_rejects_the_shipped_migration_note() {
        let lib = "\
/// The keyword was `factory =` and took a store expression. There is no
/// deprecated arm, because nothing in this workspace is published yet and this
/// is the last release in which that is true. Change the keyword and hand it a
/// [`Fixture`] instead of a store.
";
        let problems = stale_publication_claims(lib, "", false);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains(TESTKIT_LIB));
        assert!(problems[0].contains(STALE_NOTHING_PUBLISHED));
    }

    /// The exact sentence C2-07's README half was written for, verbatim from
    /// `crates/happenstance-testkit/README.md:18` as it stood at HEAD, with
    /// `happenstance-sqlite`'s conformance test present — which is the
    /// workspace's actual state, not a hypothetical one.
    #[test]
    fn c2_07_rejects_the_shipped_readme_sentence_once_an_adapter_has_run_the_suite() {
        let readme = "\
> What is still early is everything around that. **No adapter has run this
> suite**; the workspace's storage crates are skeletons. The `ProjectionStore`
";
        // A correct rustdoc half, so only the README half is under test here.
        let lib = "/// Ground it in `23fd446` and `1c1a6b7`.\n";
        let problems = stale_publication_claims(lib, readme, true);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains(TESTKIT_README));
        assert!(problems[0].contains(STALE_NO_ADAPTER));
    }

    /// The claim is true, and stays true, while no adapter's conformance test
    /// exists — the check must not fire ahead of the fact it is grounded in.
    #[test]
    fn c2_07_readme_claim_passes_while_no_adapter_has_run_the_suite() {
        let readme = "> **No adapter has run this suite**; the workspace's storage crates \
                       are skeletons.\n";
        let lib = "/// Ground it in `23fd446` and `1c1a6b7`.\n";
        assert!(stale_publication_claims(lib, readme, false).is_empty());
    }

    /// Deleting the stale sentence without grounding the replacement in the
    /// two commits is not the fix C2-07 asks for: it is the sentence's
    /// silent removal, which the finding's own "why a defect" names as
    /// reading like licence to do the same thing again.
    #[test]
    fn c2_07_rejects_a_bare_deletion_with_no_replacement_grounding() {
        let lib = "/// The keyword was `factory =`. Change it and hand the macro a \
                    [`Fixture`] instead of a store.\n";
        let problems = stale_publication_claims(lib, "", false);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains(FACTORY_INTRODUCED));
        assert!(problems[0].contains(FACTORY_REMOVED));
    }

    /// The corrected form: both halves pass together.
    #[test]
    fn c2_07_passes_the_corrected_text() {
        let lib = "/// There is no deprecated arm: `factory =` was introduced at `23fd446` \
                    and removed at `1c1a6b7`, eight days before this crate's first publish, so \
                    no published version ever accepted it.\n";
        let readme = "> **`happenstance-sqlite` has run this suite** — its own rustdoc says so.\n";
        assert!(
            stale_publication_claims(lib, readme, true).is_empty(),
            "{:?}",
            stale_publication_claims(lib, readme, true)
        );
    }

    // ---- runbook_status_matches_the_registry ------------------------------
    //
    // The table below is `RUNBOOK.md`'s status table as `4a7ca16` left it, cut
    // to the rows that carry the argument and quoted rather than paraphrased:
    // the milestone row's em-dash `#` cell, the bolded `**done**` states and the
    // backticked version are each a spelling the parser has to see through, and
    // a paraphrase would quietly drop the one that matters.
    const STALE_TABLE: &str = "\
## Status

| # | Phase | Depends on | State | Proof artefact |
|---|---|---|---|---|
| 4 | [Freeze the contract](#phase-4) | 2, 3 | **done** | `frozen_signatures.rs` |
| 6 | [Freeze `ProjectionStore`](#phase-6) | 4 | not started | `CheckpointOnlyStore` failing the suite |
| 7 | [The typed layer](#phase-7) | 4, 6 | not started | a `trybuild` compile-fail case |
| — | **`0.2.0-alpha.1`** | 7 | — | — |
| 8 | [`happenstance-sqlite`](#phase-8--happenstance-sqlite) | 4, 6, 7 | not started | the concurrency macro at 64 contenders |
| 10 | [Postgres and Neon](#phase-10--happenstance-postgres-and-happenstance-neon) | 2, 4, 6 | not started | the concurrency macro on a store that does not serialise |

State is one of `not started`, `in progress`, `blocked`, `done`.
";

    /// The released heading this check reads, and the `[Unreleased]` one it must
    /// not: an unreleased version vouches for nothing.
    const CHANGELOG_FIXTURE: &str = "\
## [Unreleased]

## [0.2.0-alpha.1] — 2026-08-16
";

    /// `PUBLISHABLE` as `xtask/src/package.rs` spells it today.
    const PUBLISHED: [&str; 5] = [
        "happenstance-core",
        "happenstance",
        "happenstance-testkit",
        "happenstance-sqlite",
        "happenstance-cloudflare",
    ];

    /// The two axes over a fixture, as strings, so a test can name what fired.
    fn disagreements(table: &str, changelog: &str) -> Vec<String> {
        let rows = phase_rows(table).unwrap();
        let mut out = Vec::new();
        for version in released_versions(changelog) {
            let Some(milestone) = rows.iter().find(|r| r.phase == version) else {
                continue;
            };
            for number in prerequisites(&rows, &milestone.depends_on) {
                if rows
                    .iter()
                    .find(|r| r.number == number)
                    .is_some_and(|r| r.state != "done")
                {
                    out.push(format!("{version} waits on phase {number}"));
                }
            }
        }
        for row in &rows {
            if row.state == "not started" {
                for c in PUBLISHED.iter().filter(|c| names_crate(&row.raw, c)) {
                    out.push(format!("phase {} names {c}", row.number));
                }
            }
        }
        out
    }

    /// The table as it shipped, and the three rows it was wrong about.
    ///
    /// Phase 4 is in the same closure and is `done`, so its absence here is the
    /// evidence that the closure is not simply reporting everything it walks.
    #[test]
    fn the_shipped_status_table_is_rejected_on_both_axes() {
        assert_eq!(
            disagreements(STALE_TABLE, CHANGELOG_FIXTURE),
            vec![
                "0.2.0-alpha.1 waits on phase 6",
                "0.2.0-alpha.1 waits on phase 7",
                "phase 8 names happenstance-sqlite",
            ]
        );
    }

    /// The same table with the three rows corrected, which is the tree this
    /// change leaves behind.
    ///
    /// Phase 10 stays `not started` and stays green: its row names
    /// `happenstance-postgres` and `happenstance-neon`, neither of which is
    /// publishable, and no released version waits on it. A check that could not
    /// leave a genuinely unstarted phase alone would be one this table is
    /// rewritten to satisfy.
    #[test]
    fn the_corrected_status_table_passes_and_leaves_phase_ten_alone() {
        // Only the three rows the two axes named, which is the edit this change
        // makes to the real table. A blanket replace would have corrected phase
        // 10 as well and the last assertion below is what caught that.
        let fixed = STALE_TABLE
            .lines()
            .map(|line| {
                if ["| 6 |", "| 7 |", "| 8 |"]
                    .iter()
                    .any(|n| line.starts_with(n))
                {
                    line.replace("| not started |", "| done |")
                } else {
                    line.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            disagreements(&fixed, CHANGELOG_FIXTURE).is_empty(),
            "{:?}",
            disagreements(&fixed, CHANGELOG_FIXTURE)
        );
        let rows = phase_rows(&fixed).unwrap();
        assert_eq!(
            rows.iter().find(|r| r.number == "10").unwrap().state,
            "not started"
        );
    }

    /// The boundary that separates the two axes' subjects, and the reason
    /// [`names_crate`] is not [`names_rule`].
    ///
    /// With `-` treated as a boundary rather than as part of the name, the bare
    /// `happenstance` matches inside `happenstance-postgres` and every row in
    /// the table names a publishable crate — the check then fires on phase 10,
    /// the author widens an exclusion list, and it is off.
    #[test]
    fn a_hyphenated_crate_name_does_not_match_its_own_prefix() {
        let row =
            "| 10 | [Postgres and Neon](#phase-10--happenstance-postgres-and-happenstance-neon) |";
        assert!(!names_crate(row, "happenstance"));
        assert!(!names_crate(row, "happenstance-sqlite"));
        assert!(names_crate(
            "| 8 | [`happenstance-sqlite`](#phase-8--happenstance-sqlite) |",
            "happenstance-sqlite"
        ));
    }

    /// The cheapest defeat, refused: a state the legend does not name.
    ///
    /// `pending` is not `not started`, so axis 2's equality test would pass over
    /// it in silence and the row would claim nothing to anybody. The parser
    /// reports the vocabulary instead.
    #[test]
    fn a_state_outside_the_legend_is_visible_to_the_parser() {
        let odd = STALE_TABLE.replace("| 8 | [`happenstance-sqlite`](#phase-8--happenstance-sqlite) | 4, 6, 7 | not started |", "| 8 | [`happenstance-sqlite`](#phase-8--happenstance-sqlite) | 4, 6, 7 | pending |");
        let rows = phase_rows(&odd).unwrap();
        let eight = rows.iter().find(|r| r.number == "8").unwrap();
        assert_eq!(eight.state, "pending");
        assert!(!PHASE_STATES.contains(&eight.state.as_str()));
    }

    /// `[Unreleased]` is not a release, and the second table in the document is
    /// not the status table.
    #[test]
    fn only_dated_headings_are_releases_and_only_one_table_is_read() {
        assert_eq!(
            released_versions(CHANGELOG_FIXTURE),
            vec!["0.2.0-alpha.1".to_owned()]
        );

        let two_tables = format!(
            "{STALE_TABLE}\n### Estimates\n\n| # | Phase | Days | Cum |\n|---|---|---|---|\n| 0 | Ground clear | 2 | 2 |\n"
        );
        let rows = phase_rows(&two_tables).unwrap();
        assert!(
            rows.iter().all(|r| r.number != "0"),
            "the estimate table was read as status rows: {rows:?}"
        );
    }

    /// The exact shape V-6 found: a match arm reached from `Self::`, cited as
    /// though it opened the construct.
    #[test]
    fn a_match_arm_is_mid_construct() {
        let lines = [
            "enum Enrolment {",
            "    CourseDefined { capacity: u32 },",
            "}",
            "",
            "fn event_type(e: &Enrolment) {",
            "    match e {",
            "        Self::CourseDefined { .. } => Self::EVENT_TYPES[0].clone(),",
            "    }",
            "}",
        ];
        assert!(citation_resolves(&lines, 7, 7, true).is_err());
    }

    /// The rule's positive control: an item keyword opening a construct at
    /// the cited line passes, whatever the keyword.
    #[test]
    fn an_item_start_resolves() {
        let lines = [
            "enum Enrolment {",
            "    CourseDefined { capacity: u32 },",
            "}",
        ];
        assert!(citation_resolves(&lines, 1, 3, true).is_ok());
    }

    /// A blank first line is rejected before mid-construct is even asked.
    #[test]
    fn a_blank_first_line_does_not_resolve() {
        let lines = ["enum Enrolment {", "", "}"];
        assert!(citation_resolves(&lines, 2, 2, true).is_err());
    }

    /// A range past the end of the file is out of bounds, not silently
    /// clamped.
    #[test]
    fn a_range_past_the_file_end_does_not_resolve() {
        let lines = ["enum Enrolment {", "}"];
        assert!(citation_resolves(&lines, 1, 5, true).is_err());
        assert!(citation_resolves(&lines, 9, 9, true).is_err());
    }

    /// The mid-construct check is Rust-only: a non-Rust target is held only
    /// to "the range exists and the first line is not blank".
    #[test]
    fn mid_construct_is_not_checked_outside_rust_targets() {
        let lines = ["        Self::still not Rust,"];
        assert!(citation_resolves(&lines, 1, 1, false).is_ok());
    }

    /// [`parse_citation`]'s positive control, and the shape V-6's own
    /// fixtures use.
    #[test]
    fn a_full_path_range_citation_parses() {
        let citation =
            parse_citation("examples/course-subscriptions/src/main.rs:194-207", 3).unwrap();
        assert_eq!(citation.target, "examples/course-subscriptions/src/main.rs");
        assert_eq!(citation.start, 194);
        assert_eq!(citation.end, 207);
    }

    /// The documented blind spot: no `/` in the span means no anchor to read
    /// it against, so this is not parsed as a citation at all — not parsed
    /// as one that then fails to resolve.
    #[test]
    fn a_pathless_span_does_not_parse_as_a_citation() {
        assert!(parse_citation("store.rs:205-215", 1).is_none());
        assert!(parse_citation(":341-379", 1).is_none());
    }

    /// A backtick span that is not `path:START` at all — an ordinary code
    /// identifier — is not mistaken for one.
    #[test]
    fn a_non_citation_span_does_not_parse() {
        assert!(parse_citation("EventType", 1).is_none());
        assert!(parse_citation("crates/happenstance-core/Cargo.toml", 1).is_none());
    }

    /// Two citations on one line, the second bare — V-6's fixtures verbatim.
    /// Only the first parses; the second is the stated no-carry-over gap.
    #[test]
    fn only_the_path_bearing_citation_on_a_shared_line_parses() {
        let line = "A mirror of `examples/course-subscriptions/src/main.rs:194-207` (the \
                     domain enum) and `:341-379` (the fold).";
        let mut out = Vec::new();
        citations_in_line(line, 3, &mut out);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].start, 194);
    }

    /// A citation whose target does not resolve under the repository root —
    /// [`check_citation`]'s second documented gap — is silently out of
    /// reach rather than a reported problem.
    #[test]
    fn an_unresolvable_target_path_is_silently_skipped() {
        let root = workspace_root().unwrap();
        let citation = Citation {
            written_at: 1,
            target: "src/main.rs".to_owned(),
            start: 1,
            end: 1,
        };
        assert!(check_citation(&root, "some/fixture.rs", &citation).is_none());
    }

    /// [`check_citation`] against a real file in this tree: a construct that
    /// is there and a line number past the end of it that is not.
    #[test]
    fn check_citation_reads_a_real_file_and_reports_its_own_line() {
        let root = workspace_root().unwrap();
        let good = Citation {
            written_at: 42,
            target: "xtask/Cargo.toml".to_owned(),
            start: 1,
            end: 1,
        };
        // `.toml` is not `.rs`, so only "does the range exist" is asked.
        assert!(check_citation(&root, "some/fixture.md", &good).is_none());

        let bad = Citation {
            written_at: 42,
            target: "xtask/Cargo.toml".to_owned(),
            start: 999_999,
            end: 999_999,
        };
        let problem = check_citation(&root, "some/fixture.md", &bad).unwrap();
        assert!(problem.starts_with("some/fixture.md:42"));
        assert!(problem.contains("out of bounds"));
    }

    // ---- M-4: CF-24's orphan scanner covers two of five enumerations -------

    /// Every file the gate already treats as holding conformance rules must
    /// have an enumeration row. `RULE_FILES` is the derived half — it is what
    /// `all_rules`, CF-29's changelog check and `spec-trace`'s check 6 all read
    /// — so a fifth rule file cannot be added without this failing (RS-81-5).
    #[test]
    fn every_rule_file_has_an_enumeration() {
        let missing: Vec<&str> = RULE_FILES
            .into_iter()
            .filter(|f| !ENUMERATIONS.iter().any(|e| e.file == *f))
            .collect();
        assert!(
            missing.is_empty(),
            "CF-24 says no rule may exist without appearing in its enumeration, and these \
             files' enumerations are scanned by nothing: {missing:?}"
        );
    }

    /// The benchmark scenarios are a sixth `pub async fn` family behind a
    /// `for_each_*!` macro, and deleting a name from that macro was measured to
    /// pass `spec-trace`, `lints`, `lint-changelog`, clippy and the testkit's own
    /// tests. Whether CF-24 *governs* a benchmark scenario is CF-34's question;
    /// whether one can vanish silently is not.
    #[test]
    fn the_benchmark_enumeration_is_scanned() {
        assert!(
            ENUMERATIONS.iter().any(|e| e.file.ends_with("/bench.rs")),
            "a scenario deleted from `for_each_event_store_benchmark!` still compiles, still \
             has its function, and is run by nothing"
        );
    }
}
