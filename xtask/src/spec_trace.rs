//! Checks that the architectural specification and the code agree (CF-38).
//!
//! # What this exists to stop
//!
//! `docs/architecture/SPECIFICATION.md` claims, for every normative clause, that
//! some conformance rule can observe a violation of it and that some end-to-end
//! case exercises it. Those claims were written by hand. Nothing has ever checked
//! them, and a specification whose cross-references have quietly rotted is worse
//! than one that never made them — it reads as though it is backed by tests.
//!
//! The one check worth naming separately is the last: **a `[PROVISIONAL]` marker
//! with no falsifier is indistinguishable from a decision nobody wanted to
//! make**, and by the time anyone notices it has been load-bearing for a year.
//!
//! # What it does not do
//!
//! It cannot tell whether a rule is the *right* rule for a clause. That is
//! judgement, it lives in §7.3 through §7.6, and it is why an unclaimed rule can
//! be *disposed of* by a clause rather than merely reported: an unclaimed rule is
//! either a decision someone made or a rule nobody is responsible for deleting,
//! and only the author knows which.
//!
//! # Why it also *writes* §7.1 and §7.2
//!
//! Those two sections are a census and a cross-reference table — 193 rows of
//! exactly the facts this file already parses. They were computed by hand at
//! `2a65d76` and nothing has ever checked them, which is the same defect one
//! level up: a table asserting what every clause is traceable to, itself
//! traceable to nothing.
//!
//! So `--write` renders them. That much is only convenience. **The load-bearing
//! half is the equality check the gate runs without the flag**, because the
//! ability to regenerate a table does nothing on its own — a generator nobody
//! runs decays exactly as fast as a hand-written table, and decays invisibly,
//! since the document now *looks* mechanical. Comparing the committed region
//! against the freshly computed one on every CI run is what converts "we could
//! regenerate this" into "this is what the code says today".
//!
//! # Why it *checks* §1.3 and deliberately does not write it
//!
//! §1.3 states the same census a second time, in prose a person wrote. This file
//! checks it — the total, each maturity count, the "of which N are normative"
//! figure, and the document's own subtraction relating the three — and generating
//! it is the one thing that must never be done.
//!
//! The reason is that §7.1 and §7.2 both come from the same [`parse_clauses`]
//! output, so a parser that quietly stops recognising a clause form shifts the
//! census and the table *together* and the equality check above stays green. §1.3
//! is the only count in the document a human computed by reading it, which is why
//! `docs/RUNBOOK.md` treats its agreement with the checker as the best evidence
//! available that the parser reads the document the way a person does. Moving it
//! inside the generated markers would destroy the very property it is being used
//! to prove — and that will be the next contributor's first instinct, because the
//! section now looks like the only hand-maintained number left.
//!
//! §7.3 through §7.6 stay authored and are never touched: they carry the
//! judgement about *why* a gap exists, which no parser can recover.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

const SPEC: &str = "docs/architecture/SPECIFICATION.md";
const CASES: &str = "docs/scenarios/E2E-CASES.md";
const SUITE: &str = "crates/happenstance-testkit/src/suite.rs";

/// Every file a conformance rule may be defined in.
///
/// Three, not one. Since stage 5 rules live in three files: the event-store
/// family in `suite.rs`, the proptest family in `model.rs` and the threaded
/// family in `concurrency.rs`. [`run`]'s clause checks stay scoped to [`SUITE`]
/// on purpose — only that family's rules are claimed by clauses today — but
/// anything asking *does this rule exist* must ask all three, or it answers a
/// narrower question than its own message claims. Both callers of [`all_rules`]
/// were doing exactly that until stage 6's review: `retired_rules` printed "none
/// naming a rule still in the suite" having looked in one file of three, and
/// CF-29 let a model or concurrency rule land with no changelog entry at all.
///
/// It lives here rather than in `lints` because [`collect_rules`] does, and a
/// list of files kept next to the function that parses them cannot drift from it.
pub(crate) const RULE_FILES: [&str; 3] = [
    SUITE,
    "crates/happenstance-testkit/src/model.rs",
    "crates/happenstance-testkit/src/concurrency.rs",
];

/// A section of the specification, in the order §7.1 and §7.2 present them.
///
/// This is the single place the six clause families are enumerated: the parser
/// takes its accepted prefixes from here, the census takes its rows, and §7.2
/// takes its subsection headings. Three lists that must agree, kept as one so
/// that adding a family cannot half-land.
struct Section {
    /// The clause-ID prefix, hyphen included, as it appears in the document.
    prefix: &'static str,
    /// The `Section` cell in §7.1.
    summary: &'static str,
    /// The §7.2 subsection heading, verbatim.
    heading: &'static str,
}

const SECTIONS: &[Section] = &[
    Section {
        prefix: "VT-",
        summary: "§2.1–§2.6 value types",
        heading: "#### `VT` — value types (§2.1–§2.6)",
    },
    Section {
        prefix: "WF-",
        summary: "§2.7 wire format",
        heading: "#### `WF` — wire format (§2.7)",
    },
    Section {
        prefix: "ES-",
        summary: "§3 `EventStore`",
        heading: "#### `ES` — the `EventStore` port (§3)",
    },
    Section {
        prefix: "PS-",
        summary: "§4 `ProjectionStore`",
        heading: "#### `PS` — the `ProjectionStore` port (§4)",
    },
    Section {
        prefix: "SY-",
        summary: "§5 `SyncPeer`",
        heading: "#### `SY` — the `SyncPeer` port (§5)",
    },
    Section {
        prefix: "CF-",
        summary: "§6 conformance",
        heading: "#### `CF` — conformance obligations (§6)",
    },
];

/// Every maturity marker, in the order §7.1's columns and §1.3's hand count use —
/// most settled first, rather than alphabetically, so that the shape of the
/// document is legible from the row.
const MATURITY: [&str; 4] = ["FROZEN", "PROVISIONAL", "DEFERRED", "NON-NORMATIVE"];

/// The opening words of §1.3's hand-written census sentence.
///
/// An anchor rather than a marker pair, because the point of §1.3 is that it is
/// authored: putting machine-readable delimiters around it would be the first step
/// toward generating it, and generating it destroys the independence that makes it
/// evidence. The cost of the anchor is that rewording this sentence's opening
/// breaks the check loudly — which is the right failure, since a reworded census
/// is exactly when it needs re-reading.
const CENSUS_ANCHOR: &str = "As assembled, this document carries";

/// The English number words §1.3's census may use, valued by index.
///
/// English prose spells small numbers as words, and §1.3 does: "**two
/// `[NON-NORMATIVE]`**" while every larger figure is digits. The alternative was to
/// require digits everywhere and rewrite the sentence to suit the parser, which is
/// the tail wagging the dog on a sentence whose whole value is that a person wrote
/// it. Twelve is where prose conventionally switches to digits, so the table stops
/// there rather than pretending to be exhaustive.
const NUMBER_WORDS: [&str; 13] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    "eleven", "twelve",
];

/// What §7.2 prints where a clause names no rule, or no case.
const NONE_CELL: &str = "*(none — see clause)*";

/// The width a §7.2 cell is truncated to, the `…` included.
///
/// The clause is authoritative and the table is an index into it, so a cell that
/// wraps across a terminal buys nothing. 79 is the width the hand-written table
/// settled on and there is no reason to move it — changing it would rewrite two
/// hundred rows for no gain.
const CELL_WIDTH: usize = 79;

/// The markers delimiting the region of the specification this file owns.
///
/// A generated region needs a boundary a machine can find, not a convention a
/// reader is expected to honour: without one, `--write` has to guess where the
/// authored prose resumes, and the first wrong guess deletes judgement that no
/// parser can reconstruct.
const BEGIN_MARKER: &str = "<!-- BEGIN GENERATED: spec-trace §7.1–§7.2 -->";
const END_MARKER: &str = "<!-- END GENERATED -->";

/// Where the region goes the first time, before the markers exist.
const REGION_START: &str = "### 7.1 Summary";
const REGION_END: &str = "### 7.3 ";

/// What to do about §7.1 and §7.2.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
    /// Compare the committed region against the computed one and fail if they
    /// differ. This is what the gate runs, and it is the point of the exercise.
    Check,
    /// Rewrite the committed region in place.
    Write,
}

/// A parsed normative clause.
struct Clause {
    id: String,
    line: usize,
    maturity: Option<String>,
    /// The text inside a `[PROVISIONAL — …]` or `[DEFERRED — …]` marker.
    falsifier: String,
    rules: Vec<String>,
    /// Whether the clause declares any of its rules as not yet written.
    schedules_new: bool,
    /// Whether the clause points at a test that lives outside `suite.rs` — a unit
    /// or compile test in the crate it constrains. The checker cannot validate
    /// those names, so §7.2 must not mark them `†`: that would assert "does not
    /// exist yet" about something nothing looked for.
    rule_elsewhere: bool,
    /// The `Rule:` field verbatim, for the clauses §7.2 renders in their own words.
    rule_text: Option<String>,
    cases: Vec<String>,
    /// The `Cases:` field verbatim, for the same reason.
    cases_text: Option<String>,
    retires: Vec<String>,
    has_rejects: bool,
}

/// How many clauses of each maturity each section holds.
///
/// One computation, two consumers: the line `spec-trace` prints and §7.1 of the
/// document it generates. Computing them separately is precisely how a printed
/// summary and the table it summarises come to disagree — and the disagreement
/// would be invisible, because nobody diffs a console line against a committed
/// one.
struct Census {
    /// Per `SECTIONS` entry, counts indexed as `MATURITY` is.
    counts: Vec<[usize; MATURITY.len()]>,
    /// Per `SECTIONS` entry, every clause it declares.
    totals: Vec<usize>,
    /// Clauses carrying no marker this table knows. Already a hard failure in its
    /// own right, so §7.1's four columns need not sum to its `Clauses` column;
    /// hiding the shortfall would be worse than showing it.
    unmarked: usize,
}

impl Census {
    fn of(clauses: &[Clause]) -> Self {
        let mut census = Self {
            counts: vec![[0; MATURITY.len()]; SECTIONS.len()],
            totals: vec![0; SECTIONS.len()],
            unmarked: 0,
        };
        for c in clauses {
            let Some(s) = SECTIONS.iter().position(|s| c.id.starts_with(s.prefix)) else {
                continue;
            };
            census.totals[s] += 1;
            match MATURITY
                .iter()
                .position(|m| Some(*m) == c.maturity.as_deref())
            {
                Some(m) => census.counts[s][m] += 1,
                None => census.unmarked += 1,
            }
        }
        census
    }

    fn total(&self) -> usize {
        self.totals.iter().sum()
    }

    fn column(&self, maturity: usize) -> usize {
        self.counts.iter().map(|row| row[maturity]).sum()
    }
}

/// Checks that every `file:line` citation resolves.
fn check_citations(root: &Path, spec: &str, problems: &mut Vec<String>) {
    for (line_no, citation, path, line) in citations(spec) {
        match fs::read_to_string(root.join(&path)) {
            Err(_) => problems.push(format!(
                "{SPEC}:{line_no} — citation `{citation}` names a file that does not exist"
            )),
            Ok(body) => {
                let len = body.lines().count();
                if line > len {
                    problems.push(format!(
                        "{SPEC}:{line_no} — citation `{citation}` points past the end of {} \
                         ({len} lines)",
                        path.display()
                    ));
                }
            }
        }
    }
}

/// The census §1.3 states in prose, and the line it states it on.
struct StatedCensus {
    line: usize,
    /// "carries N clause IDs".
    total: usize,
    /// "of which N are normative".
    normative: usize,
    /// One figure per `MATURITY` entry, indexed as `MATURITY` is.
    by_maturity: [usize; MATURITY.len()],
}

/// Checks §1.3's hand count against what this run computed, and against itself.
///
/// Every disagreement is reported separately and names both numbers, because the
/// remedy differs by which one is wrong and the reader is the one who decides: a
/// checker that only says "§1.3 is wrong" leaves them diffing two censuses by eye,
/// and a check whose failure cannot be acted on gets deleted rather than fixed.
///
/// Note what is *not* checked: the parenthetical naming which clauses are
/// `[NON-NORMATIVE]` — "(CF-30, and VT-12 …)" — is prose about identity, not a
/// count, and matching clause IDs out of a sentence would be guessing at a
/// structure the sentence does not have. Narrow and certain beats broad and
/// approximate here; the count itself is what decays.
fn check_stated_census(spec: &str, census: &Census, problems: &mut Vec<String>) {
    let stated = match stated_census(spec) {
        Ok(s) => s,
        Err(why) => {
            problems.push(why);
            return;
        }
    };
    let at = format!("{SPEC}:{}", stated.line);

    if stated.total != census.total() {
        problems.push(format!(
            "{at} — §1.3 says the document carries {} clause IDs; the checker counts {}. \
             §1.3 is a hand count and is deliberately not generated, so one of the two is \
             wrong and only a reader can say which.",
            stated.total,
            census.total()
        ));
    }
    for (i, m) in MATURITY.iter().enumerate() {
        if stated.by_maturity[i] != census.column(i) {
            problems.push(format!(
                "{at} — §1.3 says {} `[{m}]`; the checker counts {}.",
                stated.by_maturity[i],
                census.column(i)
            ));
        }
    }

    // §1.3's own arithmetic. The "are normative" figure is derived from the other
    // two, so a sentence that disagrees with itself was edited in pieces — and
    // that can be true while all three figures still match the checker, which is
    // why this is a separate assertion rather than a consequence of the ones above.
    let Some(i) = MATURITY.iter().position(|m| *m == "NON-NORMATIVE") else {
        problems.push(format!(
            "{at} — MATURITY no longer lists NON-NORMATIVE, so §1.3's normative arithmetic \
             cannot be checked. Restore it or delete this check; leaving it is a check that \
             reports nothing, which reads exactly like success."
        ));
        return;
    };
    let derived = stated.total.saturating_sub(stated.by_maturity[i]);
    if derived != stated.normative {
        problems.push(format!(
            "{at} — §1.3 does not add up on its own terms: {} clause IDs minus {} \
             `[NON-NORMATIVE]` is {derived}, but the sentence says {} are normative.",
            stated.total, stated.by_maturity[i], stated.normative
        ));
    }
}

/// Parses §1.3's census paragraph.
///
/// The paragraph is joined into one line before anything is matched: the figures
/// are hard-wrapped away from the markers they qualify — "**two\n`[NON-NORMATIVE]`**"
/// today — so a line-at-a-time parser would find the marker with no number in
/// front of it and report the document as unparseable every time someone reflowed
/// a paragraph.
fn stated_census(spec: &str) -> Result<StatedCensus, String> {
    let lines: Vec<&str> = spec.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.trim_start().starts_with(CENSUS_ANCHOR))
        .ok_or_else(|| {
            format!(
                "{SPEC} — §1.3 has no sentence beginning \"{CENSUS_ANCHOR}\", so the hand count \
                 the checker is cross-checked against cannot be found. Restore the sentence, or \
                 update CENSUS_ANCHOR to match its new wording."
            )
        })?;
    let end = lines[start..]
        .iter()
        .position(|l| l.trim().is_empty())
        .map_or(lines.len(), |i| i + start);
    let paragraph = lines[start..end]
        .iter()
        .map(|l| l.trim())
        .collect::<Vec<_>>()
        .join(" ");
    let line = start + 1;
    let blame = |why: String| format!("{SPEC}:{line} — §1.3's census {why}");

    let total = count_between(&paragraph, CENSUS_ANCHOR, "clause IDs").map_err(&blame)?;
    let normative = count_between(&paragraph, "of which", "are normative").map_err(&blame)?;
    let mut by_maturity = [0; MATURITY.len()];
    for (i, m) in MATURITY.iter().enumerate() {
        by_maturity[i] = count_before(&paragraph, m).map_err(&blame)?;
    }

    Ok(StatedCensus {
        line,
        total,
        normative,
        by_maturity,
    })
}

/// The count between two fixed runs of words, as in "carries **193** clause IDs".
fn count_between(text: &str, after: &str, before: &str) -> Result<usize, String> {
    let from = text
        .find(after)
        .ok_or_else(|| format!("does not contain \"{after}\""))?
        + after.len();
    let rest = &text[from..];
    let to = rest
        .find(before)
        .ok_or_else(|| format!("does not contain \"{before}\" after \"{after}\""))?;
    count_word(&rest[..to])
        .map_err(|why| format!("figure between \"{after}\" and \"{before}\" {why}"))
}

/// The count immediately preceding a maturity marker, as in "**46 `[PROVISIONAL]`**".
fn count_before(text: &str, maturity: &str) -> Result<usize, String> {
    let needle = format!("`[{maturity}]`");
    let mut hits = text.match_indices(&needle);
    let Some((at, _)) = hits.next() else {
        return Err(format!("states no `[{maturity}]` figure"));
    };
    // Two occurrences means the sentence mentions the marker somewhere other than
    // its count, and picking the first would be a guess. Refusing is the honest
    // answer: a census sentence that got discursive needs re-reading anyway.
    if hits.next().is_some() {
        return Err(format!(
            "names `[{maturity}]` more than once, so which figure is its count is a guess"
        ));
    }
    let before = text[..at].trim_end().trim_end_matches(['*', ' ']);
    let word = before.rsplit(' ').next().unwrap_or(before);
    count_word(word).map_err(|why| format!("figure for `[{maturity}]` {why}"))
}

/// A census figure: ASCII digits, or one of the number words §1.3 spells out.
fn count_word(text: &str) -> Result<usize, String> {
    let t = text.trim().trim_matches(['*', '`', ' ']);
    if let Ok(n) = t.parse::<usize>() {
        return Ok(n);
    }
    NUMBER_WORDS
        .iter()
        .position(|w| w.eq_ignore_ascii_case(t))
        .ok_or_else(|| {
            format!("is \"{t}\", which is neither digits nor a number word from zero to twelve")
        })
}

/// Runs the traceability check, and checks or rewrites §7.1–§7.2.
///
/// # Errors
///
/// Returns an error if a document cannot be read, if any check fails, or if the
/// committed §7.1–§7.2 region does not match the computed one.
pub(crate) fn run(mode: Mode) -> Result<()> {
    let root = workspace_root()?;
    let spec = read(&root, SPEC)?;
    let cases_doc = read(&root, CASES)?;
    let suite = read(&root, SUITE)?;

    let clauses = parse_clauses(&spec);
    if clauses.is_empty() {
        bail!(
            "parsed no clauses from {SPEC} — the parser and the document disagree about clause format"
        );
    }

    let known_cases = collect_cases(&cases_doc);
    let known_rules = collect_rules(&suite);

    let mut problems: Vec<String> = Vec::new();

    // 1. Every clause carries a maturity marker.
    // 2. Every PROVISIONAL / DEFERRED marker names a falsifier or an experiment.
    // 3. Every clause names the wrong implementation it forbids.
    for c in &clauses {
        match c.maturity.as_deref() {
            None => problems.push(format!(
                "{}:{} — {} has no maturity marker",
                SPEC, c.line, c.id
            )),
            Some("PROVISIONAL" | "DEFERRED") if c.falsifier.trim().len() < 12 => {
                problems.push(format!(
                    "{}:{} — {} is {} with no falsifier. A provisional marker with nothing that \
                     could refute it is a decision nobody wanted to make.",
                    SPEC,
                    c.line,
                    c.id,
                    c.maturity.as_deref().unwrap_or("?")
                ));
            }
            _ => {}
        }
        if !c.has_rejects && c.maturity.as_deref() != Some("NON-NORMATIVE") {
            problems.push(format!(
                "{}:{} — {} names no rejected implementation. A clause no adapter can fail is \
                 decorative; mark it [NON-NORMATIVE] and move it to prose.",
                SPEC, c.line, c.id
            ));
        }
    }

    // 4. Every named conformance rule exists, unless the clause schedules new
    //    ones — in which case the prose cannot tell us which name is which, and
    //    reporting them all would drown the check that matters.
    //
    //    Only clauses whose rules would live in a suite that *exists*. `PS` and
    //    `SY` rules belong to the projection and replication suites, and neither
    //    crate has been written — checking those names against the event-store
    //    suite is a category error that reports every one of them as missing,
    //    which is noise indistinguishable from a real typo.
    for c in &clauses {
        if c.schedules_new || !has_suite(&c.id) {
            continue;
        }
        for rule in &c.rules {
            if !known_rules.contains(rule) {
                problems.push(format!(
                    "{}:{} — {} names rule `{}`, which is not in {} and the clause does not declare it new",
                    SPEC, c.line, c.id, rule, SUITE
                ));
            }
        }
    }

    // 5. Every named case exists.
    for c in &clauses {
        for case in &c.cases {
            if !known_cases.contains(case) {
                problems.push(format!(
                    "{}:{} — {} names {}, which does not exist in {}",
                    SPEC, c.line, c.id, case, CASES
                ));
            }
        }
    }

    // 6. Every rule in the suite is claimed by a clause, or disposed of by one.
    let claimed: BTreeSet<&String> = clauses.iter().flat_map(|c| &c.rules).collect();
    let retired: BTreeSet<&String> = clauses.iter().flat_map(|c| &c.retires).collect();
    for rule in &known_rules {
        if !claimed.contains(rule) && !retired.contains(rule) {
            problems.push(format!(
                "{SUITE} — rule `{rule}` is named by no clause and disposed of by none. Either a \
                 clause claims it, or one retires it with `Retires: {rule} — <reason>`."
            ));
        }
    }

    // 7. Every `file:line` citation resolves to a file that exists and is long enough.
    check_citations(&root, &spec, &mut problems);

    let census = Census::of(&clauses);

    // 8. §1.3's hand count says what this run just computed, and adds up on its
    //    own terms.
    check_stated_census(&spec, &census, &mut problems);

    // 9. §7.1 and §7.2 say what this run just computed.
    let generated = generated_region(&census, &clauses, &known_rules);
    let stale = sync_region(&root, &spec, &generated, mode)?;

    report(
        &census,
        &known_rules,
        &known_cases,
        &problems,
        stale.as_deref(),
    )
}

/// Checks that no rule a clause disposes of is still live (§7.4).
///
/// # The hole this closes
///
/// [`run`]'s check 6 accepts `claimed || retired`, so a `Retires:` line satisfies
/// it *forever*: the specification can go on saying a rule is gone while the rule
/// sits in `suite.rs` rejecting registered mutants, and nothing notices. That is
/// not hypothetical. Phase 3 retired three rules, reversed all three
/// retirements, and in every case the document and the suite disagreed for a
/// whole phase with the checker silent — §7.4 is the record, and it schedules
/// this lint.
///
/// # Why a claimed rule fails too
///
/// §7.4 proposes the exemption "unless the clause also claims it", and this does
/// not implement it. Two reasons, and they point the same way.
///
/// The first is that a claimed-and-retired rule is a document contradicting
/// itself. One clause says the rule is disposed of and another relies on it;
/// whichever is stale, a reader cannot tell which by reading, and check 6 stays
/// green either way.
///
/// The second is [`retires_of`]'s trap. It reads the whole `Retires:` field, so a
/// *successor* rule backticked in the reasoning — "…the strengthened successor is
/// `duplicate_items_do_not_duplicate_events`" — registers as retired while being
/// legitimately claimed. The exemption exists to tolerate exactly that shape, and
/// tolerating it is what leaves the trap armed: drop the claim later and the
/// successor is silently disposed of. Failing instead makes the convention
/// `retires_of` documents — a `Retires:` field names the retired rule and
/// backticks nothing else — a build failure rather than a note.
///
/// So the rule this enforces is the simple one: **a `Retires:` line is discharged
/// by deleting the rule, in the same change.** Until the deletion lands the line
/// is a claim to re-examine, which is §7.4's own conclusion.
///
/// # The two vacuity guards, and why the second one is not the tree's job
///
/// Its two sibling lints both bail when the thing they scan is empty, and this
/// one had neither guard. The first is easy: [`RULE_FILES`] must parse to
/// something, or "no retired rule is still live" is a statement about nothing.
///
/// The second is the one that matters, because it cannot be answered from the
/// tree. There are **no `Retires:` fields in `SPECIFICATION.md` at all** — all
/// three of phase 3's retirements were reversed — so a run over the real document
/// exercises the parse of the field's *spelling* not at all. Rename the field to
/// `Retired:`, or move it inside a `**bold**` paragraph that [`field_line`]'s
/// continuation loop breaks on, and this prints the same green line for ever.
/// [`RETIRES_PROBE`] is therefore parsed on every run: a fixture clause, held to
/// the name it is known to contain, so "the field spelling still parses" is a
/// failure rather than an assumption.
///
/// # Errors
///
/// Returns an error if a document cannot be read, if the probe stops parsing, or
/// if any rule named by a `Retires:` field is still defined in [`RULE_FILES`].
pub(crate) fn retired_rules() -> Result<()> {
    let root = workspace_root()?;
    let spec = read(&root, SPEC)?;

    let parsed = retires_of(RETIRES_PROBE);
    if parsed != [RETIRES_PROBE_NAME] {
        bail!(
            "the `Retires:` probe parsed as {parsed:?}, not `[{RETIRES_PROBE_NAME}]`. This lint \
             reads a field that appears nowhere in {SPEC} today, so the tree cannot exercise it \
             and a green run would prove nothing. Either `field_line`/`retires_of` have stopped \
             recognising the field — in which case every disposition in the document is now \
             invisible — or the convention changed and `RETIRES_PROBE` is what records it."
        );
    }

    let clauses = parse_clauses(&spec);
    if clauses.is_empty() {
        bail!("parsed no clauses from {SPEC} — the disposed-rule lint would pass vacuously");
    }
    let known_rules = all_rules(&root)?;
    if known_rules.is_empty() {
        bail!(
            "parsed no rules from {} — the disposed-rule lint would pass vacuously",
            RULE_FILES.join(", ")
        );
    }
    let claimed: BTreeSet<&String> = clauses.iter().flat_map(|c| &c.rules).collect();

    let mut problems = Vec::new();
    let mut dispositions = 0usize;

    let defined_in = |rule: &str| {
        RULE_FILES
            .iter()
            .find(|file| read(&root, file).is_ok_and(|body| collect_rules(&body).contains(rule)))
            .copied()
            .unwrap_or(SUITE)
    };

    for c in &clauses {
        for rule in &c.retires {
            dispositions += 1;
            if !known_rules.contains(rule) {
                continue;
            }
            let file = defined_in(rule);
            if claimed.contains(rule) {
                problems.push(format!(
                    "{SPEC}:{} — {} retires `{rule}`, which is still in {file} *and* claimed by \
                     a clause. The document contradicts itself. Either the `Retires:` prose \
                     backticks a name it should not — it names the retired rule and nothing else \
                     — or the disposition is stale and the line goes.",
                    c.line, c.id
                ));
            } else {
                problems.push(format!(
                    "{SPEC}:{} — {} retires `{rule}`, which is still in {file} and claimed by no \
                     clause. Check 6 is satisfied by the disposition and will stay satisfied \
                     forever, so nothing else can notice. Delete the rule, or reverse the \
                     retirement and have a clause claim it.",
                    c.line, c.id
                ));
            }
        }
    }

    if !problems.is_empty() {
        for p in &problems {
            println!("  {p}");
        }
        bail!(
            "{} disposition(s) naming a rule that is still live. A `Retires:` line is a \
             hypothesis about a wrong implementation and it is discharged by deleting the rule, \
             not by writing the line (§7.4: three for three were reversed).",
            problems.len()
        );
    }

    println!(
        "§7.4: {dispositions} disposition(s) against {} rule(s) in {} file(s), none naming a rule \
         still live",
        known_rules.len(),
        RULE_FILES.len()
    );
    Ok(())
}

/// A fixture clause, parsed on every run of [`retired_rules`].
///
/// It exists because the tree contains no `Retires:` field to exercise the parse
/// against — see that function's second vacuity guard. Every spelling convention
/// [`field_line`] tolerates is deliberately *not* exercised here; one canonical
/// shape is enough to answer the question this guards, which is whether the field
/// is recognised at all.
///
/// The name it retires is nonsense on purpose. It is parsed from this constant
/// and never from the document, so it can never collide with a real rule.
const RETIRES_PROBE: &str = "\
#### ES-0 — a fixture clause, read by `retired_rules` and by nothing else

`[FROZEN]`
`Rule:` `a_rule_the_probe_claims`
`Retires:` `a_rule_the_probe_retires` — and continuation prose that deliberately
backticks nothing else, because that is the convention this field is held to.
`Cases:` none
`Rejects:` nothing. It is not a clause; it is a parser test with no test harness
to live in.
";

/// The one rule [`RETIRES_PROBE`] disposes of.
const RETIRES_PROBE_NAME: &str = "a_rule_the_probe_retires";

fn report(
    census: &Census,
    rules: &BTreeSet<String>,
    cases: &BTreeSet<String>,
    problems: &[String],
    stale: Option<&str>,
) -> Result<()> {
    let mut summary = String::new();
    let _ = write!(summary, "{} clauses (", census.total());
    let mut parts: Vec<String> = MATURITY
        .iter()
        .enumerate()
        .map(|(i, m)| format!("{} {m}", census.column(i)))
        .collect();
    if census.unmarked > 0 {
        parts.push(format!("{} unmarked", census.unmarked));
    }
    let _ = write!(summary, "{}), ", parts.join(", "));
    let _ = write!(
        summary,
        "{} suite rules, {} e2e cases",
        rules.len(),
        cases.len()
    );
    println!("{summary}");

    if problems.is_empty() && stale.is_none() {
        println!("traceability: no problems found; §7.1–§7.2 matches the checker");
        return Ok(());
    }

    for p in problems {
        println!("  {p}");
    }
    if let Some(diff) = stale {
        println!();
        println!("{diff}");
    }

    match (problems.len(), stale) {
        (0, _) => bail!(
            "{SPEC}'s §7.1–§7.2 is not what the checker computes. Run `cargo xtask spec-trace \
             --write` to regenerate it; the checker is the authority, which is the entire reason \
             the region is generated rather than authored."
        ),
        (n, None) => bail!(
            "{n} traceability problem(s). These are defects in the specification, not in the \
             checker."
        ),
        (n, Some(_)) => bail!(
            "{n} traceability problem(s), and §7.1–§7.2 is stale. Fix the clauses first: \
             regenerating the table over unfixed defects only records them in one more place."
        ),
    }
}

/// Renders §7.1 and §7.2, markers included, with no trailing newline.
fn generated_region(census: &Census, clauses: &[Clause], known_rules: &BTreeSet<String>) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "{BEGIN_MARKER}");
    let _ = writeln!(out);
    let _ = writeln!(out, "{REGION_START}");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "| Section | Prefix | Clauses | `[FROZEN]` | `[PROVISIONAL]` | `[DEFERRED]` | \
         `[NON-NORMATIVE]` |"
    );
    let _ = writeln!(out, "|---|---|---|---|---|---|---|");
    for (i, section) in SECTIONS.iter().enumerate() {
        let _ = write!(
            out,
            "| {} | `{}` | {} |",
            section.summary,
            section.prefix.trim_end_matches('-'),
            census.totals[i]
        );
        for m in 0..MATURITY.len() {
            let _ = write!(out, " {} |", census.counts[i][m]);
        }
        let _ = writeln!(out);
    }
    let _ = write!(out, "| **Total** | | **{}** |", census.total());
    for m in 0..MATURITY.len() {
        let _ = write!(out, " **{}** |", census.column(m));
    }
    let _ = writeln!(out);
    if census.unmarked > 0 {
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "{} clause(s) carry no maturity marker and appear in no column above. That is a \
             build failure under CF-38, not a category.",
            census.unmarked
        );
    }

    let _ = writeln!(out);
    let _ = writeln!(out, "### 7.2 The table");

    for section in SECTIONS {
        let mut rows: Vec<&Clause> = clauses
            .iter()
            .filter(|c| c.id.starts_with(section.prefix))
            .collect();
        // Document order is already numeric today. Sorting anyway costs nothing and
        // means a clause inserted out of order cannot silently reorder the table.
        rows.sort_by_key(|c| clause_number(&c.id, section.prefix));

        let _ = writeln!(out);
        let _ = writeln!(out, "{}", section.heading);
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "| Clause | Maturity | Conformance rule — † = does not exist yet | Cases |"
        );
        let _ = writeln!(out, "|---|---|---|---|");
        for c in rows {
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} |",
                c.id,
                c.maturity.as_deref().unwrap_or("*(no marker)*"),
                rule_cell(c, known_rules),
                cases_cell(c)
            );
        }
    }

    let _ = writeln!(out);
    let _ = write!(out, "{END_MARKER}");
    out
}

/// The `Conformance rule` cell for one clause.
///
/// Two shapes, and which one a clause gets is decided by what the checker could
/// actually verify. Where it validated a list of names against `suite.rs`, the
/// cell is that list and every `†` is the checker's own answer. Where it declined
/// — the clause points at a unit or compile test living in the crate it
/// constrains, or it names no rule at all and describes an obligation in prose —
/// the cell is the clause's own words, because a `†` there would assert something
/// nothing checked.
fn rule_cell(c: &Clause, known_rules: &BTreeSet<String>) -> String {
    let Some(text) = &c.rule_text else {
        return NONE_CELL.to_owned();
    };
    if text.trim_start().to_ascii_lowercase().starts_with("none") {
        return NONE_CELL.to_owned();
    }
    if c.rules.is_empty() || c.rule_elsewhere {
        return cell(text);
    }
    let names: Vec<String> = c
        .rules
        .iter()
        .map(|r| {
            if known_rules.contains(r) {
                format!("`{r}`")
            } else {
                format!("`{r}` †")
            }
        })
        .collect();
    cell(&names.join(", "))
}

/// The `Cases` cell for one clause.
fn cases_cell(c: &Clause) -> String {
    let Some(text) = &c.cases_text else {
        return NONE_CELL.to_owned();
    };
    let lower = text.trim_start().to_ascii_lowercase();
    // "all contract-level cases" is the obligation §6 states of itself, and
    // enumerating fifty-six numbers in a cell would hide rather than show it.
    if lower.starts_with("all") {
        return "*all*".to_owned();
    }
    if lower.starts_with("none") {
        // A clause can serve no case *directly* and still cite ones that motivate
        // it. Dropping the citation would make it indistinguishable from a clause
        // no case touches at all, which is the distinction §7.5 turns on.
        let cited = case_ids(text);
        return if cited.is_empty() {
            NONE_CELL.to_owned()
        } else {
            cell(&format!("*(none directly; cites {})*", cited.join(", ")))
        };
    }
    if c.cases.is_empty() {
        return cell(text);
    }
    cell(&c.cases.join(", "))
}

/// One table cell: pipes escaped, then truncated.
fn cell(text: &str) -> String {
    // A literal pipe ends the cell and shifts every column after it. No clause
    // carries one today; escaping is what keeps that a fact rather than a
    // coincidence nobody would notice breaking.
    let escaped = text.replace('|', "\\|");
    if escaped.chars().count() <= CELL_WIDTH {
        return escaped;
    }
    let mut kept: String = escaped.chars().take(CELL_WIDTH - 1).collect();
    // Never end on the backslash half of an escaped pipe: it would escape the
    // ellipsis instead and render as a stray backslash.
    while kept.ends_with('\\') {
        kept.pop();
    }
    kept.push('…');
    kept
}

/// The number in a clause ID, for ordering. `0` for an ID that has none, which
/// `clause_id` cannot produce.
fn clause_number(id: &str, prefix: &str) -> u32 {
    id.get(prefix.len()..)
        .and_then(|n| n.parse().ok())
        .unwrap_or(0)
}

/// Compares or rewrites the generated region, and returns a description of the
/// difference when there is one.
///
/// In [`Mode::Write`] it returns `None` after writing — the region has just been
/// made true, so there is nothing left to report — but it still prints what it
/// displaced, because the first `--write` swallows whatever authored prose sat
/// between §7.1 and §7.3 and losing that silently is the one failure mode a
/// generator has that a hand-written table does not.
fn sync_region(root: &Path, spec: &str, generated: &str, mode: Mode) -> Result<Option<String>> {
    let lines: Vec<&str> = spec.lines().collect();
    let span = region_span(&lines)?;
    let committed: Vec<&str> = lines[span.clone()].to_vec();
    let computed: Vec<&str> = generated.lines().collect();

    if committed == computed {
        return Ok(None);
    }

    let difference = describe_difference(span.start + 1, &committed, &computed);

    if mode == Mode::Check {
        return Ok(Some(difference));
    }

    let mut out: Vec<&str> = lines[..span.start].to_vec();
    out.extend_from_slice(&computed);
    out.extend_from_slice(&lines[span.end..]);
    let mut body = out.join("\n");
    body.push('\n');
    fs::write(root.join(SPEC), body).with_context(|| format!("rewriting {SPEC}"))?;

    println!("{difference}");
    println!();
    println!("rewrote {SPEC} §7.1–§7.2 ({} lines)", computed.len());
    Ok(None)
}

/// The half-open line range the generated region occupies.
///
/// Falls back to the span the headings describe when the markers are not there
/// yet, so the first `--write` can install them. That fallback is deliberately
/// the *only* guess this file makes about document structure: everything after it
/// is anchored to markers a machine put there.
fn region_span(lines: &[&str]) -> Result<Range<usize>> {
    let begin = lines.iter().position(|l| l.trim() == BEGIN_MARKER);
    let end = lines.iter().position(|l| l.trim() == END_MARKER);
    match (begin, end) {
        (Some(b), Some(e)) if e > b => return Ok(b..e + 1),
        (Some(_), _) | (_, Some(_)) => bail!(
            "{SPEC} has one of the generated-region markers without the other. Restore both, or \
             delete both and let `--write` reinstall them around §7.1–§7.2."
        ),
        (None, None) => {}
    }

    let start = lines
        .iter()
        .position(|l| l.trim() == REGION_START)
        .with_context(|| format!("{SPEC} has no `{REGION_START}` heading to anchor §7.1 to"))?;
    let mut end = lines[start..]
        .iter()
        .position(|l| l.starts_with(REGION_END))
        .map(|i| i + start)
        .with_context(|| format!("{SPEC} has no `{REGION_END}` heading to bound §7.2 at"))?;
    // The blank line separating §7.2 from §7.3 belongs to the document, not to
    // the region. Swallowing it would leave the end marker jammed against the
    // §7.3 heading, and every subsequent run would faithfully reproduce that.
    while end > start && lines[end - 1].trim().is_empty() {
        end -= 1;
    }
    Ok(start..end)
}

/// A readable account of how the committed region and the computed one differ.
///
/// Not a real diff. An LCS would align an insertion and report one changed line
/// where this reports the rest of the table; the extra fidelity is not worth the
/// code, because the remedy is never "edit the row" — it is always `--write`, and
/// what a reader needs is enough evidence to believe that.
fn describe_difference(first_line: usize, committed: &[&str], computed: &[&str]) -> String {
    const SHOWN: usize = 10;

    let mut out = format!(
        "{SPEC}:{first_line} — §7.1–§7.2 differs from what the checker computes \
         ({} committed lines, {} computed):\n",
        committed.len(),
        computed.len()
    );
    let mut shown = 0;
    let mut differing = 0;
    for i in 0..committed.len().max(computed.len()) {
        let have = committed.get(i).copied();
        let want = computed.get(i).copied();
        if have == want {
            continue;
        }
        differing += 1;
        if shown < SHOWN {
            shown += 1;
            let _ = writeln!(out, "  line {}:", first_line + i);
            let _ = writeln!(out, "    committed: {}", have.unwrap_or("(end of region)"));
            let _ = writeln!(out, "    computed:  {}", want.unwrap_or("(end of region)"));
        }
    }
    if differing > shown {
        let _ = writeln!(out, "  … and {} more differing line(s)", differing - shown);
    }
    out
}

/// Splits the specification into clauses.
///
/// Two forms are in use — `#### ES-1 — …` in the earlier sections and
/// `**PS-1 — …**` in the later ones — because they were authored in parallel.
/// Both are accepted rather than normalised: rewriting 193 headings to satisfy a
/// parser is the tail wagging the dog, and the parser is the cheaper thing to
/// make tolerant.
fn parse_clauses(spec: &str) -> Vec<Clause> {
    let lines: Vec<&str> = spec.lines().collect();
    let mut starts: Vec<(usize, String)> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if let Some(id) = clause_id(line) {
            starts.push((i, id));
        }
    }

    let mut out = Vec::new();
    for (n, (start, id)) in starts.iter().enumerate() {
        let end = starts.get(n + 1).map_or(lines.len(), |(s, _)| *s);
        let body = lines[*start..end].join("\n");
        let rules = rules_of(&body);
        out.push(Clause {
            id: id.clone(),
            line: start + 1,
            maturity: maturity_of(&body),
            falsifier: falsifier_of(&body),
            rules: rules.names,
            schedules_new: rules.schedules_new,
            rule_elsewhere: rules.elsewhere,
            rule_text: field_line(&body, "Rule"),
            cases: cases_of(&body),
            cases_text: field_line(&body, "Cases"),
            retires: retires_of(&body),
            has_rejects: field_line(&body, "Rejects").is_some(),
        });
    }
    out
}

/// The clause ID a heading or bold-run line declares, if it declares one.
fn clause_id(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let is_heading = trimmed.starts_with('#');
    let is_bold = trimmed.starts_with("**");

    // Everything else is prose. This matters more than it looks: a paragraph that
    // wraps onto a line beginning "ES-15." reads to a naive parser exactly like a
    // declaration, and the phantom clause it invents has no maturity marker and no
    // rejected implementation — so it is reported as two defects that cannot be
    // fixed, in a document where nothing is wrong.
    if !is_heading && !is_bold {
        return None;
    }

    let t = trimmed.trim_start_matches('#').trim_start();
    let t = t.strip_prefix("**").unwrap_or(t);
    let prefix = SECTIONS
        .iter()
        .map(|s| s.prefix)
        .find(|p| t.starts_with(p))?;
    let digits: String = t[prefix.len()..]
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();
    if digits.is_empty() {
        return None;
    }
    // A heading or a bold run beginning with a clause ID is a declaration. The
    // narrower test this replaces required an em dash, a full stop or the end of
    // the bold run, and it silently dropped `**CF-30 is [NON-NORMATIVE] and is
    // prose.**` — a real clause, missing from the census, in the one category
    // whose whole point is to be visible.
    let rest = &t[prefix.len() + digits.len()..];
    // `**ES-40**` closes immediately: that is a cross-reference, and §1 is full of
    // them. `**PS-1 — …` and `**CF-30 is …` continue inside the bold run: those
    // are declarations. The distinction is the whole difference between a census
    // of the document and a census of its own footnotes.
    let declares = is_heading || (!rest.starts_with("**") && rest.starts_with([' ', '.']));
    declares.then(|| format!("{prefix}{digits}"))
}

/// The clause's own marker: the one that appears *first*, not the one that
/// sorts first.
///
/// Priority order is the obvious implementation and it is wrong. A provisional
/// clause routinely names the marker it would earn — ES-10 says "one affordable
/// answer lifts this clause to `[FROZEN]` at phase 4" — and a search that looks
/// for `[FROZEN]` before `[PROVISIONAL]` reads that sentence as the clause's
/// status. The census then over-reports frozen clauses, which is the direction
/// that flatters the document.
fn maturity_of(body: &str) -> Option<String> {
    const ALL: [&str; MATURITY.len()] = MATURITY;

    // A clause states its marker at the start or the end of a line. Anything
    // mid-sentence is the clause talking *about* a marker — CF-38 quotes
    // "`[PROVISIONAL]` or `[DEFERRED]` marker with an empty falsifier", which is
    // the check it performs, and a positional-first search read that as its own
    // status. So: prefer a marker in declaration position, and only fall back to
    // first-anywhere when a clause states none.
    let declared = body.lines().find_map(|line| {
        let t = line.trim().trim_matches(['*', '`', ' ']);
        ALL.into_iter().find(|m| {
            t.starts_with(&format!("[{m}]"))
                || t.starts_with(&format!("[{m} —"))
                || t.ends_with(&format!("[{m}]`"))
                || t.ends_with(&format!("[{m}]"))
        })
    });
    if let Some(m) = declared {
        return Some(m.to_owned());
    }

    ALL.into_iter()
        .filter_map(|m| {
            let at = [format!("[{m}]"), format!("[{m} —")]
                .iter()
                .filter_map(|pat| body.find(pat))
                .min()?;
            Some((at, m.to_owned()))
        })
        .min_by_key(|(at, _)| *at)
        .map(|(_, m)| m)
}

fn falsifier_of(body: &str) -> String {
    for m in ["PROVISIONAL", "DEFERRED"] {
        let open = format!("[{m} —");
        if let Some(i) = body.find(&open) {
            let after = &body[i + open.len()..];
            let end = after.find(']').unwrap_or(after.len());
            return after[..end].to_owned();
        }
    }
    String::new()
}

/// The full text of a `Field:` entry in a clause body, continuation lines included.
///
/// Three spellings are in use, because the sections were authored in parallel:
/// `Rule:`, `- **Rule:**` and `` `Rule:` ``. Rather than enumerate them, the
/// line's leading punctuation is stripped and the marker is matched against the
/// remainder with `*` and backticks removed. Enumerating spellings is how a
/// checker silently stops checking a section — it reports nothing, which looks
/// exactly like success.
///
/// Continuation matters as much as spelling. A `Rule:` naming two rules wraps,
/// and reading only the first line drops the second — which reported live rules
/// as owned by no clause and would have had someone delete one.
fn field_line(body: &str, field: &str) -> Option<String> {
    let lines: Vec<&str> = body.lines().collect();
    let start = lines
        .iter()
        .position(|l| field_head(l).as_deref() == Some(field))?;

    let first = lines[start];
    let colon = first.find(':')?;
    let mut value = &first[colon + 1..];
    // Skip the marker's own closing punctuation, and *exactly* that. A stray
    // backtick shifts the parity of every code span after it, so the value parses
    // as though the prose between rule names were the rule names. Trimming a whole
    // run is worse: it eats the opening backtick of the first real name and
    // produces the same silence from the other direction.
    for closer in ['`', '*'] {
        value = value.strip_prefix(closer).unwrap_or(value);
    }
    let mut out = value.trim_start().to_owned();

    for line in &lines[start + 1..] {
        let t = line.trim();
        if t.is_empty() || field_head(line).is_some() || t.starts_with('#') || t.starts_with("**") {
            break;
        }
        out.push(' ');
        out.push_str(t);
    }

    // A field whose value is a bullet list puts nothing on the marker's own line,
    // so the joined value opens with the first bullet's `*`. That is list
    // punctuation, not content, and leaving it defeats the `none` test every
    // caller runs — `* none; a documented exclusion …` read as a clause naming a
    // rule, which is the opposite of what it says.
    let out = out.trim();
    let out = out
        .strip_prefix("* ")
        .or_else(|| out.strip_prefix("- "))
        .unwrap_or(out);
    Some(out.trim_start().to_owned())
}

/// The field name a line declares, if it declares one.
fn field_head(line: &str) -> Option<String> {
    let t = line.trim_start().trim_start_matches(['-', '*', '`', ' ']);
    if !t.contains(':') {
        return None;
    }
    let head: String = t
        .chars()
        .take_while(|c| *c != ':')
        .filter(|c| *c != '*' && *c != '`')
        .collect();
    let head = head.trim();
    ["Rule", "Cases", "Rejects", "Retires"]
        .contains(&head)
        .then(|| head.to_owned())
}

/// Every rule name a clause mentions, and whether it declares any of them new.
///
/// The two are separate answers to separate questions, and conflating them was a
/// bug worth recording. *Claiming* a rule is what keeps it from being reported as
/// unowned, and a clause claims every rule it names. *Validating* a rule against
/// the suite only makes sense when the clause does not also schedule new ones —
/// the prose is freeform, so there is no reliable way to tell which name in
/// "`existing_rule`; new `planned_rule`" the word "new" attaches to.
///
/// Returning early on the first new-marker made a clause like that claim
/// *nothing*, which reported eighteen live rules as owned by no clause. A checker
/// whose false positives look exactly like its true ones is not usable.
fn rules_of(body: &str) -> Rules {
    let Some(text) = field_line(body, "Rule") else {
        return Rules::default();
    };
    // A clause with nothing of its own claims nothing.
    if text.trim_start().to_ascii_lowercase().starts_with("none") {
        return Rules::default();
    }
    // Not every backticked identifier on a `Rule:` line is a conformance rule.
    // The specification distinguishes, in prose, between a suite rule and a unit
    // or compile test living in the crate it constrains — `position_next_signals_overflow`
    // is a unit test in `event.rs`, and no amount of looking in `suite.rs` will
    // find it. Treating those as missing rules is the checker misreading the
    // document rather than the document being wrong.
    //
    // `meta-test` is the third family and it was missing, which mattered more
    // than the other two: the CF clauses' meta-tests live in
    // `happenstance-testkit`'s `tests/`, `collect_rules` scans only `suite.rs`,
    // and `backticked_idents` drops a path like
    // `mutation_coverage::every_rule_has_a_mutant` because of the colons — so a
    // clause naming one parsed with an empty rule list and §7.2 rendered a cell
    // that *looked* checked. Marking it `elsewhere` makes both that and
    // `schedules_new` true, which is exactly right: nothing here looked, and the
    // table now says so in the clause's own words.
    let elsewhere =
        text.contains("unit test") || text.contains("compile test") || text.contains("meta-test");
    let schedules_new = text.contains("(new)")
        || text.contains('†')
        || text.trim_start().starts_with("new ")
        || text.contains(" new `")
        || elsewhere;
    Rules {
        names: backticked_idents(&text),
        schedules_new,
        elsewhere,
    }
}

/// What a clause's `Rule:` field says, decomposed.
#[derive(Default)]
struct Rules {
    /// Every rule name the clause mentions.
    names: Vec<String>,
    /// Whether it declares any of them not yet written.
    schedules_new: bool,
    /// Whether it points at a test outside `suite.rs`.
    elsewhere: bool,
}

/// Every rule a clause disposes of.
///
/// The trap, and it is invisible from the document: this reads the *whole*
/// `Retires:` field, continuation lines included, so **every** backticked rule
/// name anywhere in the reasoning is registered as retired — including a live
/// rule the same clause claims two lines earlier. Nothing fails while the rule is
/// also claimed, because check 6 accepts either; the day someone drops the claim,
/// the rule is silently treated as disposed of forever. Keep a `Retires:` field
/// to the retired name and prose that backticks nothing else.
///
/// [`retired_rules`] is the lint that turns that paragraph from advice into a
/// failure, and it is worth reading the two together: it fails a `Retires:` name
/// that `collect_rules` still finds, *whether or not* a clause also claims it.
/// Which means the trap above is no longer merely documented — a successor rule
/// named in the reasoning is a build failure, so the "backticks nothing else"
/// convention is enforced rather than remembered. That is the whole reason the
/// lint does not exempt a claimed rule; §7.4 proposed the exemption, and the
/// exemption is exactly the hole this function's own doc comment describes.
fn retires_of(body: &str) -> Vec<String> {
    field_line(body, "Retires")
        .map(|t| backticked_idents(&t))
        .unwrap_or_default()
}

/// Snake-case identifiers inside backticks — the shape every rule name has.
fn backticked_idents(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for chunk in text.split('`').skip(1).step_by(2) {
        let c = chunk.trim();
        if !c.is_empty()
            && c.contains('_')
            && c.chars()
                .all(|ch| ch.is_ascii_lowercase() || ch == '_' || ch.is_ascii_digit())
        {
            out.push(c.to_owned());
        }
    }
    out
}

fn cases_of(body: &str) -> Vec<String> {
    let Some(text) = field_line(body, "Cases") else {
        return Vec::new();
    };
    if text.trim_start().to_ascii_lowercase().starts_with("none") {
        return Vec::new();
    }
    case_ids(&text)
}

/// Every `E2E-nn` a run of prose names.
fn case_ids(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (i, _) in text.match_indices("E2E-") {
        let digits: String = text[i + 4..]
            .chars()
            .take_while(char::is_ascii_digit)
            .collect();
        if !digits.is_empty() {
            out.push(format!("E2E-{digits}"));
        }
    }
    out
}

/// Whether a clause's rules would live in a conformance suite that exists today.
///
/// The event store's suite does. The projection store's and the replication
/// port's do not, and will not until the phases that build those ports. Until
/// then their rule names are scheduled work, and the honest answer is that
/// nothing can check them rather than that they are all wrong.
fn has_suite(clause_id: &str) -> bool {
    clause_id.starts_with("ES-") || clause_id.starts_with("VT-") || clause_id.starts_with("WF-")
}

/// Every rule defined anywhere in [`RULE_FILES`].
///
/// # Errors
///
/// Returns an error if any of [`RULE_FILES`] cannot be read. A missing rule file
/// is a hard failure rather than an empty contribution: silently skipping one is
/// how a check comes to scan two files while reporting on three.
pub(crate) fn all_rules(root: &Path) -> Result<BTreeSet<String>> {
    let mut out = BTreeSet::new();
    for file in RULE_FILES {
        out.extend(collect_rules(&read(root, file)?));
    }
    Ok(out)
}

/// Every rule the conformance suite actually defines.
pub(crate) fn collect_rules(suite: &str) -> BTreeSet<String> {
    suite
        .lines()
        .filter_map(|l| {
            let t = l.trim();
            let rest = t.strip_prefix("pub async fn ")?;
            let name: String = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            (!name.is_empty()).then_some(name)
        })
        .collect()
}

/// Every case the e2e document defines, from its `### E2E-nn` headings.
fn collect_cases(doc: &str) -> BTreeSet<String> {
    doc.lines()
        .filter_map(|l| {
            let t = l.trim_start_matches('#').trim();
            let digits: String = t
                .strip_prefix("E2E-")?
                .chars()
                .take_while(char::is_ascii_digit)
                .collect();
            (!digits.is_empty()).then(|| format!("E2E-{digits}"))
        })
        .collect()
}

/// `path:line` citations in backticks, as `(spec line, citation, path, line)`.
fn citations(spec: &str) -> Vec<(usize, String, PathBuf, usize)> {
    let mut out = Vec::new();
    for (n, line) in spec.lines().enumerate() {
        for chunk in line.split('`').skip(1).step_by(2) {
            let Some((path, tail)) = chunk.rsplit_once(':') else {
                continue;
            };
            // Case-insensitively, because the repository is developed on Windows
            // and clippy is right that `.RS` would otherwise slip through — a
            // citation the checker silently declines to verify is the failure mode
            // this whole step exists to prevent.
            let has_ext = |e: &str| {
                path.len() > e.len() && path[path.len() - e.len()..].eq_ignore_ascii_case(e)
            };
            if !path.contains('/') || !(has_ext(".rs") || has_ext(".toml")) {
                continue;
            }
            let first: String = tail.chars().take_while(char::is_ascii_digit).collect();
            let Ok(num) = first.parse::<usize>() else {
                continue;
            };
            out.push((n + 1, chunk.to_owned(), PathBuf::from(path), num));
        }
    }
    out
}

fn read(root: &Path, rel: &str) -> Result<String> {
    fs::read_to_string(root.join(rel)).with_context(|| format!("reading {rel}"))
}

pub(crate) fn workspace_root() -> Result<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .context("xtask must live one level below the workspace root")
}
