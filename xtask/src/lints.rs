//! The gate's grep-shaped lints (CF-6, CF-29, CF-32, CF-33; D12, which has no
//! clause — ADR-0016 §14 gives it a lint rather than a WF-13; and the ADR-path
//! check, which has no clause either and belongs to the repository's layout
//! rather than to its contract).
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
//! # The one thing most of them share
//!
//! The ADR-path check is the exception and reads raw bytes on purpose: prose
//! naming the old directory is as wrong as a link to it, since both tell a reader
//! to look somewhere that does not exist. Everything below it matches on Rust
//! source, where the opposite holds.
//!
//! Three of the four match on Rust source, and prose about a construct reads
//! exactly like the construct. `concurrency.rs` explains at length why CF-33
//! forbids the watchdog it does not have — the word `sleep` appears in that
//! explanation four times — so a lint matching raw bytes fires on the sentence
//! justifying its own existence, and the fix a hurried reader reaches for is to
//! delete the sentence or weaken the lint. [`code_lines`] is what stops that:
//! every match below runs against source with comments removed and string
//! literal *contents* blanked, so only code is looked at.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::spec_trace::{RULE_FILES, all_rules, workspace_root};

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

/// The path the ADRs used to live at, and must never be referenced by again.
const OLD_ADR_DIR: &str = "docs/adr/"; // superseded by .kb/decision/

/// Where they live now, named here so the failure message can say it.
const NEW_ADR_DIR: &str = ".kb/decision/";

/// Directories this lint does not walk.
///
/// `target/` and `.git/` for cost; `.bklg/` because a backlog item may quote a
/// historical path while describing the migration itself, and `CHANGELOG.md` for
/// the same reason one level up — a changelog entry describing the move has to be
/// able to say what moved.
const ADR_PATH_SKIP: &[&str] = &[
    "target",
    ".git",
    ".bklg",
    ".redkiln",
    ".idea",
    ".claude",
    "node_modules",
];

/// No file references the ADRs at their old `docs/adr/` path, now `.kb/decision/`.
///
/// # Why this is a gate step and not a one-time edit
///
/// The ADRs moved into `.kb/decision/` when redkiln was adopted, and 75 references
/// across 29 files moved with them — in `SPECIFICATION.md`, in `CLAUDE.md`, in
/// three crate READMEs, and as rustdoc link definitions in four crate sources and
/// two `xtask` modules. Nothing else in the gate would notice a new one: a
/// markdown link to a path that does not exist renders as a link and resolves to a
/// 404, and rustdoc's `-D warnings` covers *intra-doc* links, not raw relative
/// paths. So the failure mode is silent, gradual, and only visible to a reader who
/// clicks.
///
/// It is a whole-repository scan rather than a list of the 29 files, because the
/// files that will get this wrong are the ones nobody has written yet.
///
/// # What it does not verify
///
/// That the new paths *resolve*. A reference to `.kb/decision/0099-invented.md`
/// passes this check happily. It also reads raw bytes rather than code, so a
/// sentence about the old layout inside a comment fires exactly like a live link
/// would — deliberately, since the remedy in both cases is to say `.kb/decision/`.
/// The trees where a historical path is legitimately quoted are skipped wholesale;
/// see [`ADR_PATH_SKIP`].
///
/// The exemption below is per **line**, which has a cost worth stating because it
/// is met immediately: a wrapped sentence naming the old path on one line and the
/// new one on the next still fires. Prose that discusses the move has to keep both
/// names on a single line. Widening the window to neighbouring lines would make the
/// check fuzzy in exchange for prettier paragraphs, which is the wrong trade for a
/// gate step.
///
/// # Errors
///
/// Returns an error if the workspace cannot be walked, or if any file outside the
/// skipped trees still names the old directory.
pub(crate) fn no_old_adr_paths() -> Result<()> {
    let root = workspace_root()?;
    let mut problems = Vec::new();
    let mut scanned = 0usize;

    scan_tree_for_old_adr_paths(&root, &root, &mut scanned, &mut problems)?;

    if !problems.is_empty() {
        for p in &problems {
            println!("  {p}");
        }
        bail!(
            "{} reference(s) to `{OLD_ADR_DIR}`, which no longer exists — the ADRs are KB \
             decision atoms at `{NEW_ADR_DIR}`. Rewrite each one: the two directories have the \
             same number of path segments, so a `../../` prefix stays exactly as it is.",
            problems.len()
        );
    }

    println!("no reference to `{OLD_ADR_DIR}` in {scanned} scanned file(s)");
    Ok(())
}

/// Walk `dir`, recording every line naming [`OLD_ADR_DIR`].
///
/// # Errors
///
/// Returns an error if a directory cannot be read. An unreadable *file* is
/// skipped rather than fatal: the walk reaches binaries and whatever an editor
/// has left half-written, and neither is this lint's business.
fn scan_tree_for_old_adr_paths(
    root: &Path,
    dir: &Path,
    scanned: &mut usize,
    problems: &mut Vec<String>,
) -> Result<()> {
    let entries =
        fs::read_dir(dir).with_context(|| format!("reading directory {}", dir.display()))?;

    for entry in entries {
        let entry = entry.with_context(|| format!("walking {}", dir.display()))?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if path.is_dir() {
            if !ADR_PATH_SKIP.contains(&name.as_ref()) {
                scan_tree_for_old_adr_paths(root, &path, scanned, problems)?;
            }
            continue;
        }

        // Extension-filtered rather than content-sniffed: the references live in
        // prose, in Rust doc comments and in manifests, and every other file type
        // in this tree is either generated or binary.
        let interesting = matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("rs" | "md" | "toml" | "yml" | "yaml")
        );
        if !interesting || name == "CHANGELOG.md" {
            continue;
        }

        let Ok(body) = fs::read_to_string(&path) else {
            continue;
        };
        *scanned += 1;

        let shown = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .display()
            .to_string();
        for (number, line) in body.lines().enumerate() {
            // A line naming BOTH paths is a line *about* the move — this lint's own
            // documentation, its failure message, a migration note. A stale
            // reference names only the old one, and that is the whole difference.
            //
            // This is the exemption rather than a list of skipped files, and the
            // choice is the one `code_lines` above is about. The failure mode of a
            // raw-byte lint is that it fires on the sentence explaining why it
            // exists, and the two tempting remedies are to delete the sentence or to
            // exclude the file. Excluding `lints.rs` and `main.rs` would have blinded
            // this check to the two modules that carried rustdoc link definitions to
            // the old path in the first place — the exact files it most needs to
            // watch.
            if line.contains(OLD_ADR_DIR) && !line.contains(NEW_ADR_DIR) {
                problems.push(format!("{}:{}: {}", shown, number + 1, line.trim()));
            }
        }
    }

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
