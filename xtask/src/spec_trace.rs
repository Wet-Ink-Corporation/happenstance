//! Checks that the architectural specification and the code agree (CF-38).
//!
//! # What this exists to stop
//!
//! `spec/SPECIFICATION.md` claims, for every normative clause, that
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
//! `RUNBOOK.md` treats its agreement with the checker as the best evidence
//! available that the parser reads the document the way a person does. Moving it
//! inside the generated markers would destroy the very property it is being used
//! to prove — and that will be the next contributor's first instinct, because the
//! section now looks like the only hand-maintained number left.
//!
//! §7.3 through §7.6 stay authored and are never touched: they carry the
//! judgement about *why* a gap exists, which no parser can recover.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

const SPEC: &str = "spec/SPECIFICATION.md";
const CASES: &str = "spec/E2E-CASES.md";
const SUITE: &str = "crates/happenstance-testkit/src/suite.rs";

/// Every file a conformance rule may be defined in.
///
/// **Four, not one.** Rules live in four files: the event-store family in
/// `suite.rs`, the proptest family in `model.rs`, the threaded family in
/// `concurrency.rs` and the projection family in `projection.rs`. [`run`]'s
/// clause checks stay scoped to [`SUITE`] on purpose — only that family's rules
/// are claimed by *event-store* clauses — but anything asking *does this rule
/// exist* must ask all four, or it answers a narrower question than its own
/// message claims. Both callers of [`all_rules`] were doing exactly that until
/// stage 6's review: `retired_rules` printed "none naming a rule still in the
/// suite" having looked in one file of three, and CF-29 let a model or
/// concurrency rule land with no changelog entry at all. A fourth family absent
/// from this array reproduces that defect one family later, which is why
/// `projection.rs` joins it in the same change that creates it.
///
/// It lives here rather than in `lints` because [`collect_rules`] does, and a
/// list of files kept next to the function that parses them cannot drift from it.
pub(crate) const RULE_FILES: [&str; 4] = [
    SUITE,
    "crates/happenstance-testkit/src/model.rs",
    "crates/happenstance-testkit/src/concurrency.rs",
    "crates/happenstance-testkit/src/projection.rs",
];

/// Every file a `wire::`-qualified name a clause cites may be defined in.
///
/// Deliberately **not** part of [`RULE_FILES`], and the distinction is the whole
/// of ADR-0016 §15's third change. [`RULE_FILES`] is the set check 6 sweeps —
/// every rule in it must be claimed by a clause or retired by one — because a
/// conformance rule is something an *adapter* must pass. A round trip of this
/// crate's own encoding is not an adapter obligation, so folding these two files
/// into that set would demand a clause for every helper `#[test]` in `wire.rs`.
///
/// Resolution is therefore **one-way**: a clause may name a wire test, and a wire
/// test need not be named by a clause.
const WIRE_TESTS: [&str; 2] = [
    "crates/happenstance-core/tests/wire.rs",
    "crates/happenstance-sync/tests/wire.rs",
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
///
/// Returns how many it looked at, because a check that does not state its own
/// coverage is indistinguishable from one that sees everything — which is
/// exactly how this step reported success while parsing a quarter of the
/// corpus. The count goes in the summary line.
fn check_citations(
    root: &Path,
    spec: &str,
    index: &BTreeMap<String, Vec<String>>,
    problems: &mut Vec<String>,
) -> (usize, usize, usize) {
    let mut checked = 0usize;
    let mut external = 0usize;
    let mut anchored = 0usize;
    let historical = historical_span(spec);
    for c in citations(spec, index) {
        let (line_no, text) = (c.spec_line, &c.text);
        match c.target {
            Target::External => external += 1,
            Target::Unknown => problems.push(format!(
                "{SPEC}:{line_no} — citation `{text}` names a file that is in neither the \
                 workspace nor `EXTERNAL_CITATIONS`"
            )),
            Target::Ambiguous(candidates) => problems.push(format!(
                "{SPEC}:{line_no} — citation `{text}` is a bare name the workspace defines {} \
                 times ({}). Qualify it with its path, or add it to `BARE_NAME_MAP` with the \
                 evidence for which one is meant.",
                candidates.len(),
                candidates.join(", ")
            )),
            Target::Path(path) => match fs::read_to_string(root.join(&path)) {
                Err(_) => problems.push(format!(
                    "{SPEC}:{line_no} — citation `{text}` names a file that does not exist"
                )),
                Ok(body) => {
                    checked += 1;
                    let lines: Vec<&str> = body.lines().collect();
                    if c.line > lines.len() {
                        problems.push(format!(
                            "{SPEC}:{line_no} — citation `{text}` points past the end of {} \
                             ({} lines)",
                            path.display(),
                            lines.len()
                        ));
                    } else if let Some(subject) = &c.subject
                        && !historical.contains(&line_no)
                        && !UNANCHORED_CITATIONS.iter().any(|(t, _)| t == text)
                    {
                        // If the subject appears nowhere in the cited file, the
                        // derivation picked the wrong word — not the citation the
                        // wrong line. "`limit` cannot stand in for it because
                        // `event.rs:215-217` forbids…" derives `limit`, which is
                        // a `query.rs` name and has no business being looked for
                        // here. Declining is the difference between a check that
                        // reports drift and one that reports its own guesses:
                        // the subject being *elsewhere in the same file* is the
                        // signal worth having, and that is what survives.
                        let present = lines.iter().any(|l| l.contains(subject.as_str()));
                        if !present {
                            continue;
                        }
                        anchored += 1;
                        let lo = c.line.saturating_sub(ANCHOR_SLACK + 1);
                        let hi = (c.line_end + ANCHOR_SLACK).min(lines.len());
                        if !lines[lo..hi].iter().any(|l| l.contains(subject.as_str())) {
                            problems.push(format!(
                                "{SPEC}:{line_no} — citation `{text}` is evidence for `{subject}`, \
                                 and `{subject}` is not within {ANCHOR_SLACK} lines of {}:{}. The \
                                 citation points at the wrong place, or the sentence attributes it \
                                 to the wrong thing.",
                                path.display(),
                                c.line
                            ));
                        }
                    }
                }
            },
        }
    }
    (checked, external, anchored)
}

/// How far from the cited line the subject may sit before the citation is wrong.
///
/// Twelve, where `standards/rust`'s own citation lint uses ten
/// (`lint_constitution.rs:111`) — wider because a derived anchor has further to
/// travel than a written one. There the anchor is quoted beside the line and
/// names the exact text; here it is the identifier the prose happened to use,
/// which may sit a few lines from the item's `fn` line.
///
/// The reason for a window at all is the same in both: an anchor is a claim
/// about *what* is at a location, and a doc comment growing above an item must
/// not red the gate. That is the property that makes the check survivable — a
/// content hash fails on every ordinary edit, and its refresh command becomes a
/// reflex nobody reads.
///
/// (This comment claimed the two constants were equal until it was checked. A
/// citation-drift defect inside the citation-drift check is worth leaving a note
/// about rather than quietly correcting.)
const ANCHOR_SLACK: usize = 12;

/// Citations that are deliberately not about the thing beside them.
///
/// One entry. §2.7 quotes a claim **in order to call it false** — "the second leg
/// this bullet used to offer is false: it said `memory.rs:154` and …" — so the
/// number is part of the quotation. Repairing it, or anchoring it, would falsify
/// the record of what was once claimed.
const UNANCHORED_CITATIONS: [(&str, &str); 1] = [(
    "memory.rs:154",
    "quoted inside a sentence that calls the claim it quotes false (§2.7)",
)];

/// The lines of §6.1 and §6.2, whose citations are a measurement of `b4b593d`.
///
/// Found by content rather than by line number, because line numbers in this
/// document move on every pass and a hard-coded span would come to cover the
/// wrong section silently — which is the failure this whole check exists to
/// prevent, one level up.
///
/// The declaration is §6's own: the measurement is of *that* tree, "line numbers
/// included — which is why those numbers do not resolve against the working copy
/// and are not meant to. That covers the rest of this paragraph as well as §6.1
/// and §6.2 below." So the span runs from that sentence to the start of §6.3,
/// which is where the document says the present tense resumes.
fn historical_span(spec: &str) -> BTreeSet<usize> {
    let lines: Vec<&str> = spec.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.contains("line numbers included"))
        .map(|i| i + 1);
    let end = lines
        .iter()
        .position(|l| l.trim_start().starts_with("### 6.3"))
        .map(|i| i + 1);
    match (start, end) {
        (Some(s), Some(e)) if s < e => (s..=e).collect(),
        // Neither anchor found means the document has been restructured. Return
        // nothing rather than guess: the check then reports the region's
        // citations, which is loud and correct, instead of exempting a span that
        // may no longer be the historical one.
        _ => BTreeSet::new(),
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

    let clauses = parse_clauses(&spec);
    if clauses.is_empty() {
        bail!(
            "parsed no clauses from {SPEC} — the parser and the document disagree about clause format"
        );
    }

    let known_cases = collect_cases(&cases_doc);
    // All three rule files, not just `suite.rs`. [`RULE_FILES`]'s own docs used
    // to say "[`run`]'s clause checks stay scoped to [`SUITE`] on purpose — only
    // that family's rules are claimed by clauses today", and that reason is
    // circular: they were not claimed *because* nothing required them to be.
    // Six rules — five in `concurrency.rs` and one in `model.rs` — had never
    // been named by any clause, and check 6 was structurally unable to notice,
    // including the one that pins the central DCB proposition. Four were
    // attribution errors and are claimed as of this commit, by ES-18, ES-19,
    // ES-25 and VT-11. The other two are in [`UNCLAIMED_PENDING_ADR`].
    let known_rules = all_rules(&root)?;

    // Two sets, not one, and which check gets which is load-bearing (ADR-0016
    // §15). `known_rules` is every conformance rule — check 6 sweeps it, so
    // nothing may enter it that a clause is not obliged to claim. `resolvable`
    // is that set plus the `wire::`-qualified tests, and it answers the *other*
    // question: does a name a clause cites exist anywhere. Check 4 and
    // `rule_cell` ask that one. Wiring only check 4 would leave every written
    // wire test rendering `†` in §7.2 under a legend that defines `†` as "must
    // be written".
    //
    // The two move together by construction, which is what makes the widening
    // above safe: claiming a `concurrency.rs` rule in a clause would fail check
    // 4 if `resolvable` had stayed scoped to `suite.rs`.
    let resolvable: BTreeSet<String> = known_rules.union(&wire_rules(&root)?).cloned().collect();

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
    //
    //    Resolved against `resolvable`, so a `wire::`-qualified name is looked
    //    for in the wire test files rather than in a suite that could never
    //    define it. The prefix is what routes it, which is why
    //    `backticked_idents` keeps the whole qualified string.
    for c in &clauses {
        if c.schedules_new || !has_suite(&c.id) {
            continue;
        }
        for rule in &c.rules {
            if !resolvable.contains(rule) {
                let looked_in = if rule.starts_with("wire::") {
                    WIRE_TESTS.join(" or ")
                } else {
                    SUITE.to_owned()
                };
                problems.push(format!(
                    "{}:{} — {} names rule `{}`, which is not in {} and the clause does not declare it new",
                    SPEC, c.line, c.id, rule, looked_in
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

    // 6. Every conformance rule is claimed by a clause, or disposed of by one,
    //    or listed in [`UNCLAIMED_PENDING_ADR`] as owing a decision.
    let unclaimed_pending = check_rule_ownership(&clauses, &known_rules, &mut problems);

    // 7. Every `file:line` citation resolves to a file that exists and is long
    //    enough — bare names and `.md` targets included, which is three quarters
    //    of them and was none of them until this widening.
    let index = workspace_index(&root);
    let citation_coverage = check_citations(&root, &spec, &index, &mut problems);

    let census = Census::of(&clauses);

    // 8. §1.3's hand count says what this run just computed, and adds up on its
    //    own terms.
    check_stated_census(&spec, &census, &mut problems);

    // 9. §7.1 and §7.2 say what this run just computed.
    let generated = generated_region(&census, &clauses, &resolvable);
    let stale = sync_region(&root, &spec, &generated, mode)?;

    report(
        &census,
        &known_rules,
        &known_cases,
        citation_coverage,
        &unclaimed_pending,
        &problems,
        stale.as_deref(),
    )
}

/// Check 6 — every conformance rule is owned by a clause, retired by one, or on
/// record as owing a decision. Returns the third group, for the summary.
///
/// `known_rules`, deliberately, and never `resolvable`: see [`WIRE_TESTS`]. This
/// is the direction of resolution that stays one-way.
///
/// Extracted from [`run`] when the widening to [`RULE_FILES`] pushed that
/// function past clippy's line limit — the arm that records a pending decision
/// is the whole of the growth, and it reads better beside the array it consults.
fn check_rule_ownership(
    clauses: &[Clause],
    known_rules: &BTreeSet<String>,
    problems: &mut Vec<String>,
) -> Vec<String> {
    let claimed: BTreeSet<&String> = clauses.iter().flat_map(|c| &c.rules).collect();
    let retired: BTreeSet<&String> = clauses.iter().flat_map(|c| &c.retires).collect();
    let mut pending = Vec::new();

    for rule in known_rules {
        if claimed.contains(rule) || retired.contains(rule) {
            // A rule cannot be both claimed and owed a decision. If it is, the
            // exemption has been discharged and must go, or it sits there
            // asserting an open question that is closed. This is the half that
            // makes the array a ratchet rather than an allowlist.
            if UNCLAIMED_PENDING_ADR.iter().any(|(r, _)| r == rule) {
                problems.push(format!(
                    "`{rule}` is claimed by a clause *and* listed in `UNCLAIMED_PENDING_ADR`. \
                     The exemption is discharged — delete its entry."
                ));
            }
            continue;
        }
        if let Some((_, owed)) = UNCLAIMED_PENDING_ADR.iter().find(|(r, _)| r == rule) {
            pending.push(format!("{rule} — {owed}"));
            continue;
        }
        problems.push(format!(
            "a conformance rule in {} — `{rule}` is named by no clause and disposed of by none. \
             Either a clause claims it, or one retires it with `Retires: {rule} — <reason>`, or \
             it goes in `UNCLAIMED_PENDING_ADR` with the decision it is waiting on.",
            RULE_FILES.join(", ")
        ));
    }
    pending
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
    citations: (usize, usize, usize),
    unclaimed_pending: &[String],
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
        "{} conformance rules, {} e2e cases",
        rules.len(),
        cases.len()
    );
    // The coverage number is in the summary rather than in a comment because the
    // failure this step spent a phase inside was not a wrong check, it was a
    // check whose scope nobody could see. A reader who is told "338 citations"
    // can notice that the document has more.
    let (checked, external, anchored) = citations;
    let _ = write!(
        summary,
        ", {checked} citations checked ({anchored} anchored to their subject"
    );
    if external > 0 {
        let _ = write!(summary, ", {external} external");
    }
    let _ = write!(summary, ")");
    println!("{summary}");

    // Printed on a green run, on purpose. An open question that only shows up
    // when something else is already broken is an open question nobody reads.
    if !unclaimed_pending.is_empty() {
        println!(
            "{} rule(s) claimed by no clause and owing a decision:",
            unclaimed_pending.len()
        );
        for r in unclaimed_pending {
            println!("  {r}");
        }
    }

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
///
/// `resolvable` is the suite's rules *plus* the wire tests, not the suite's alone
/// — [`rule_cell`]'s `†` means "looked for and not found", and it may only be
/// rendered against the set the checker actually searched.
fn generated_region(census: &Census, clauses: &[Clause], resolvable: &BTreeSet<String>) -> String {
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
                rule_cell(c, resolvable),
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
///
/// `resolvable` must be the same set check 4 validated against — the suite's
/// rules union the wire tests — or the two disagree and `†` starts meaning
/// "exists, but this function was handed the wrong set".
fn rule_cell(c: &Clause, resolvable: &BTreeSet<String>) -> String {
    let Some(text) = &c.rule_text else {
        return NONE_CELL.to_owned();
    };
    if text.trim_start().to_ascii_lowercase().starts_with("none") {
        return NONE_CELL.to_owned();
    }
    if c.rules.is_empty() || c.rule_elsewhere {
        return cell(text);
    }
    // A qualified name the checker has no source for was never looked for, so
    // daggering it would assert something nothing checked — the same false
    // statement ADR-0016 §15 refuses to make about the wire tests, one family
    // over. `wire::` has [`WIRE_TESTS`]; `mutation_coverage::` and `registry::`,
    // which the `CF` clauses cite, have nothing. Before §15 taught
    // [`backticked_idents`] to keep `::`, those names were dropped and the clause
    // fell through to its own prose above; this keeps that outcome for exactly
    // the names the fix did not make resolvable, and no others.
    if c.rules
        .iter()
        .any(|r| r.contains("::") && !r.starts_with("wire::"))
    {
        return cell(text);
    }
    let names: Vec<String> = c
        .rules
        .iter()
        .map(|r| {
            if resolvable.contains(r) {
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
///
/// # A `Rule:` field may backtick rule names and nothing else
///
/// This hands the **whole** field to [`backticked_idents`], continuation lines
/// included, so any backticked all-lowercase token containing an underscore
/// becomes a name the checker will demand — and since ADR-0016 §15 taught the
/// parser to keep `::` it demands harder, not less. This is the same convention
/// [`retires_of`] documents one field over; the difference was only that
/// `Retires:` said so and `Rule:` did not.
///
/// The evidence it is worth the space: ADR-0016's own amendments broke it three
/// times before landing. `skip_serializing_if`, `serde_json` and
/// `version_is_the_first_field` were all backticked inside draft `Rule:` fields,
/// where each would have produced a "names rule X, which is not in …" problem
/// that no test could ever clear. Write the attribute out in full — the spelling
/// `#[serde(skip_serializing_if = "…")]` is safe, because `#` is not a path
/// character in any version of the parser — and unbacktick the rest.
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
    //
    // Two things about this list are not free to change.
    //
    // `wire::` is deliberately *not* a trigger. ADR-0016 §15's whole point is that
    // `wire::` names became resolvable against [`WIRE_TESTS`], so an unwritten one
    // must render `†` rather than the clause's own prose.
    //
    // "compile test" is what keeps WF-12 here, and it survives in that clause's
    // `Rule:` line only as a *negation* — "a const-evaluation assertion … **not**
    // a compile test". The outcome is right and the mechanism is an accident, so:
    // `read_options_is_not_serialisable` is a `const _` and not a `#[test]`, which
    // means no resolution source can ever find it and WF-12 must stay `elsewhere`
    // permanently. Tidying those two words out of the clause, or out of this list,
    // turns WF-12 into a `†` no test can ever clear.
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
///
/// `:` is a path character, and the **whole qualified string** survives:
/// `wire::query_all_is_unambiguous`, never its last segment. Until ADR-0016 §15
/// it did not, and every `::`-qualified name a clause cited was silently dropped
/// — ten of the twelve `WF` clauses parsed to an empty rule list and rendered
/// their own prose in §7.2, claiming nothing, while `WF-9` and `VT-19` mixed a
/// plain name with `wire::` ones and rendered as fully checked.
///
/// Keeping the prefix is load-bearing twice over. It routes the name to the right
/// resolution source in [`run`]'s check 4 — [`SUITE`] or [`WIRE_TESTS`] — and a
/// bare last segment would collide silently with a suite rule of the same name,
/// because [`collect_rules`] yields unqualified names.
///
/// `.` and `/` stay excluded, which is what keeps a `path/to/file.rs:12` citation
/// out of the rule list now that `:` is allowed through.
fn backticked_idents(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for chunk in text.split('`').skip(1).step_by(2) {
        let c = chunk.trim();
        if !c.is_empty()
            && c.contains('_')
            && c.chars()
                .all(|ch| ch.is_ascii_lowercase() || ch == '_' || ch == ':' || ch.is_ascii_digit())
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
/// The event store's suite does, and so does the projection store's, as of the
/// phase that built it. The replication port's does not and will not until the
/// phase that builds it — until then `SY` rule names are scheduled work, and the
/// honest answer is that nothing can check them rather than that they are all
/// wrong. Checking them against the suites that *do* exist would report every one
/// as missing, which is noise indistinguishable from a real typo.
///
/// `PS-` was excluded for exactly that reason and the exclusion outlived it. The
/// projection family's rules live in
/// `crates/happenstance-testkit/src/projection.rs`, which [`RULE_FILES`] has
/// named since the file was created; what was missing was any clause-side check
/// that a `PS` citation resolves to one of them. Until this line changed, a `PS`
/// clause could cite a rule that had never existed and every gate in the
/// repository stayed green — the seventeen `†` marks §7.2 printed against the
/// family were an accurate statement rather than a checked one.
fn has_suite(clause_id: &str) -> bool {
    clause_id.starts_with("ES-")
        || clause_id.starts_with("VT-")
        || clause_id.starts_with("WF-")
        || clause_id.starts_with("PS-")
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

/// The clause ids `spec/SPECIFICATION.md` defines, resolved through the existing parser.
///
/// Sibling of [`all_rules`], in the same file and for the same stated reason: a list kept
/// next to the function that parses it cannot drift from it (`spec_trace.rs:83-84`).
///
/// # What this does not verify
///
/// Stated before the guarantee, because a check whose limits are undocumented is read as
/// one — `lint_constitution.rs:9-13`'s argument, one function over.
///
/// * **A doubly-declared id collapses to one entry, and nothing notices.** The return is a
///   [`BTreeSet`], so a document declaring `ES-1` twice is indistinguishable here from one
///   declaring it once, and nothing in this repository looks for the duplicate today. If
///   that is ever wanted it is a check beside §1.3's census, not a wider return type.
/// * **Existence is not appropriateness.** An id in this set is an id the document
///   declares. Whether the sentence citing that clause *should* cite it, or has quietly
///   restated it instead of deferring to it, is a judgement no parser makes.
/// * **It parses no citations.** Finding the ids a page cites is the narrative checker's;
///   this answers only which ids exist.
///
/// The accepted prefixes are [`SECTIONS`]', by way of [`clause_id`], so a seventh family
/// added in that one place needs no second edit here. A literal prefix list in the caller
/// was the alternative and it lost: it would be the fourth list of the six families, which
/// [`SECTIONS`]' own doc comment already explains is the one that can half-land. Returning
/// `Vec<Clause>` lost for a neighbouring reason — it would make a thirteen-field
/// parser-internal struct crate-visible to callers that only ever wanted a name.
///
/// # Errors
///
/// When `spec/SPECIFICATION.md` cannot be read, or when it declares no clause at all. The
/// second condition is deliberate and belongs here rather than in a caller: an empty set
/// would report every citation and every pinned id as missing, which is this function being
/// broken rather than the document.
pub(crate) fn clause_ids(root: &Path) -> Result<BTreeSet<String>> {
    collect_clause_ids(&read(root, SPEC)?)
}

/// The clause ids a specification's text declares.
///
/// Split from [`clause_ids`] exactly as [`collect_rules`] is split from [`all_rules`], so
/// the decision is assertable against a `&str` with no filesystem, no fixture tree and no
/// temp-directory dependency.
///
/// # Errors
///
/// When the text declares no clause at all — the condition [`clause_ids`] documents.
fn collect_clause_ids(spec: &str) -> Result<BTreeSet<String>> {
    let ids: BTreeSet<String> = parse_clauses(spec)
        .into_iter()
        .map(|clause| clause.id)
        .collect();

    if ids.is_empty() {
        bail!(
            "{SPEC} declares no clause ids — every citation a page makes and every id the \
             pin enumerates would report as missing, which is the checker being broken \
             rather than the document"
        );
    }
    Ok(ids)
}

/// Every `wire::`-qualified test name a clause may resolve against.
///
/// # Errors
///
/// Returns an error if [`WIRE_PROBE`] stops parsing, if a file in [`WIRE_TESTS`]
/// cannot be read, or if the two together define no tests at all — which would
/// report every `wire::` name a clause cites as a specification defect, when the
/// defect would be here.
fn wire_rules(root: &Path) -> Result<BTreeSet<String>> {
    check_wire_probe()?;
    let mut out = BTreeSet::new();
    for file in WIRE_TESTS {
        out.extend(collect_wire_tests(&read(root, file)?));
    }
    if out.is_empty() {
        bail!(
            "parsed no `#[test]` names from {} — every `wire::` name a clause cites would report \
             as missing, which is the checker being broken rather than the document",
            WIRE_TESTS.join(", ")
        );
    }
    Ok(out)
}

/// A fixture test file, held to the four names [`collect_wire_tests`] must find.
///
/// The real wire files cannot exercise the two cases that break the parser
/// quietly, and a quiet break here is a `†` on a written test or — worse — a
/// missing name that check 4 then reports as a specification defect. Line 4's
/// `        }` sits inside a raw string at exactly a nested item's column, which
/// is the forgery the indentation heuristic has to survive; `not_a_test` is the
/// helper that must not be collected.
const WIRE_PROBE: &str = r##"
mod wire {
    fn helper() {
        let json = r#"{
        }"#;
    }

    #[test]
    fn at_the_top() {}

    fn not_a_test() {}

    mod inner {
        #[test]
        #[should_panic = "…"]
        fn nested_under_two_attributes() {}
    }

    proptest! {
        #[test]
        fn in_a_macro_block(x in 0..1) {}
    }

    #[test]
    fn after_the_block_closed() {}
}
"##;

/// What [`WIRE_PROBE`] contains, in [`BTreeSet`] order.
const WIRE_PROBE_NAMES: [&str; 4] = [
    "wire::after_the_block_closed",
    "wire::at_the_top",
    "wire::in_a_macro_block",
    "wire::inner::nested_under_two_attributes",
];

/// Parses [`WIRE_PROBE`] and fails if it does not yield [`WIRE_PROBE_NAMES`].
///
/// # Errors
///
/// Returns an error if the probe stops parsing as it is known to parse.
fn check_wire_probe() -> Result<()> {
    let parsed: Vec<String> = collect_wire_tests(WIRE_PROBE).into_iter().collect();
    if parsed != WIRE_PROBE_NAMES {
        bail!(
            "the wire-test probe parsed as {parsed:?}, not {WIRE_PROBE_NAMES:?}. `collect_wire_tests` \
             qualifies the names §7.2 renders and check 4 resolves, so a parser that has started \
             missing one reports a written test as a specification defect, and one that has started \
             inventing one hides an unwritten test behind a name nothing runs."
        );
    }
    Ok(())
}

/// Every `#[test]` an integration test file defines, qualified by its modules.
///
/// The second resolution source ADR-0016 §15 adds, and the reason clauses may
/// cite `wire::query_all_is_unambiguous` at all: neither wire test file is in
/// [`RULE_FILES`], nor could be — see [`WIRE_TESTS`] for why check 6 must not
/// sweep them. The qualification is not cosmetic. `mod wire { … }` inside the
/// file is the trick `tests/mutation_coverage.rs` already uses so that `cargo
/// test --list` prints the qualified name the clauses cite, and this must produce
/// exactly that name or the two disagree about what is written.
///
/// # Why it tracks indentation rather than counting braces
///
/// `wire.rs` is ninety kilobytes of JSON string literals, and a `{` inside one is
/// indistinguishable from a block opening it if you count characters. `cargo fmt`
/// is a gate step, so an item's closing brace sits at exactly the column its
/// `mod` keyword did — which is a signal a string literal cannot forge nearly as
/// easily, and it needs no lexer.
fn collect_wire_tests(src: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    // (module name, the column its `mod` keyword sat at).
    let mut path: Vec<(String, usize)> = Vec::new();
    let mut pending_test = false;

    for line in src.lines() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        let indent = line.len() - trimmed.len();

        if trimmed == "}" {
            if path.last().is_some_and(|(_, col)| *col == indent) {
                path.pop();
            }
            continue;
        }
        if let Some(name) = opened_mod(trimmed) {
            path.push((name, indent));
            continue;
        }
        if trimmed.starts_with("#[test]") {
            pending_test = true;
            continue;
        }
        if pending_test && let Some(rest) = trimmed.strip_prefix("fn ") {
            pending_test = false;
            let name: String = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            if !name.is_empty() {
                let mut qualified = String::new();
                for (m, _) in &path {
                    qualified.push_str(m);
                    qualified.push_str("::");
                }
                qualified.push_str(&name);
                out.insert(qualified);
            }
        }
    }
    out
}

/// The module a line opens, as in `mod wire {` or `pub(crate) mod strategies {`.
fn opened_mod(trimmed: &str) -> Option<String> {
    if !trimmed.ends_with('{') {
        return None;
    }
    let rest = trimmed
        .strip_prefix("pub ")
        .or_else(|| trimmed.strip_prefix("pub(crate) "))
        .or_else(|| trimmed.strip_prefix("pub(super) "))
        .unwrap_or(trimmed)
        .strip_prefix("mod ")?;
    let name: String = rest
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    (!name.is_empty()).then_some(name)
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

/// Rules that exist, pass, and are claimed by no clause because claiming them
/// would take a decision this checker is not allowed to take.
///
/// Check 6's bar is that every conformance rule is claimed by a clause or
/// retired by one. Widening it to all of [`RULE_FILES`] found six rules that
/// were neither — five in `concurrency.rs`, one in `model.rs`. Four were
/// attribution errors: the clause already stated the proposition and already
/// named the wrong implementation, and only the rule's name was missing. The
/// remaining two are not errors — they are holes in the specification, and the
/// honest repair is an ADR rather than an edit.
///
/// The two differ in shape, which is why neither can be absorbed by a clause.
/// One states a proposition no clause states. The other states *several*, and
/// belongs to no single clause for that reason.
///
/// This list is the difference between a gap that is **recorded and counted**
/// and one that is invisible, which is the only thing check 6 was ever for. It
/// is not an allowlist in the usual sense, because it cannot be used to make a
/// problem go away quietly: every entry is printed on every green run, and an
/// entry whose rule *becomes* claimed is itself a failure, so the list can only
/// shrink. Deleting the last entry deletes the mechanism.
///
/// Per the drift allowlist that came before it (`3712c9b`), the count is
/// computed and printed rather than written here, so this comment cannot come to
/// disagree with the array beneath it.
const UNCLAIMED_PENDING_ADR: [(&str, &str); 2] = [
    (
        "k_disjoint_boundaries_admit_exactly_k_commits",
        "the central DCB independence proposition — that commands sharing no \
         consistency boundary do not conflict — is enforced by this rule and \
         stated by no clause. The word \"disjoint\" does not appear in the \
         specification. ES-25's *only if* half forbids the false-positive \
         direction and does not say this, so claiming it there would assert \
         that a FROZEN clause contains a proposition it does not. Owed: an ADR, \
         either widening ES-25 or minting a clause",
    ),
    (
        "ops_agree_with_the_model",
        "the model family's single rule enforces no single clause's sentence. \
         It replays a generated sequence of appends, conditional appends and \
         reads against a model and compares every answer, so what it checks is \
         the *composition* of ES-8, ES-9, ES-11, ES-14, ES-15, ES-18 and ES-25 \
         over inputs no clause enumerates — which is the whole reason the \
         family exists, since the named rules are worked examples and this is \
         not. §6.4 names it, but only as CF-22's illustration of a per-family \
         enumeration, and CF-22's MUST is where the rule list lives rather than \
         what any rule asserts. Claiming it under any one of the clauses it \
         exercises would say that clause is what it checks. Owed: an ADR, \
         either minting the clause the model family has never had — a store \
         agrees with the contract over arbitrary operation sequences, not only \
         over the examples the suite enumerates — or deciding that check 6's \
         bar is per-clause and a cross-clause rule is disposed of some other \
         way",
    ),
];

/// A citation whose file name is not a path in this repository.
///
/// One entry, and it earns the mechanism. §3.1 cites `trait_variant`'s own
/// source twelve times, declaring once that `variant.rs` means the crate's
/// `src/variant.rs` under `~/.cargo/registry`. That is a real citation and a
/// useful one — it is the evidence for how the derive expands — but the file is
/// not in the tree, so the moment [`citations`] learned to read bare names all
/// twelve would have turned red.
///
/// The table is the difference between "not in this repository" and "does not
/// exist", which are the same string to a checker and opposite facts to a
/// reader. A bare name that is in neither the table nor the workspace is a
/// failure, so this cannot become a place to hide a typo.
const EXTERNAL_CITATIONS: [&str; 1] = [
    // `trait-variant` 0.1.3's `src/variant.rs`, under `~/.cargo/registry`. §3.1
    // declares the shorthand once and then uses it twelve times as the evidence
    // for how the derive expands.
    "variant.rs",
];

/// Bare file names the workspace defines more than once, and which one the
/// specification means.
///
/// The document cites by bare name on purpose — `event.rs:215` reads better in
/// a sentence than the path does, and most of the 200 bare citations resolve
/// uniquely. A handful do not, and **the obvious default is wrong for some of
/// them**,
/// which is why this is a table rather than a "prefer `happenstance-core`" rule:
/// bare `lib.rs` means the *sync* crate at both of its sites (§1.6's port table
/// and §5), and `happenstance-core` also has a `lib.rs`. A preference rule would
/// have resolved both to the wrong file and passed.
///
/// Each entry is evidence, not preference — it records where the citations
/// actually point, checked one by one. Two things keep it honest. A basename
/// that is ambiguous and *absent* here is a hard failure naming the candidates,
/// so a new collision cannot resolve silently. And the anchor check is the
/// backstop for this table being wrong: if a citation mapped here to `core`
/// really meant `sync`, the anchor it names will not be found in the file this
/// sends it to.
const BARE_NAME_MAP: [(&str, &str); 7] = [
    ("memory.rs", "crates/happenstance-core/src/memory.rs"),
    ("error.rs", "crates/happenstance-core/src/error.rs"),
    ("identity.rs", "crates/happenstance-core/src/identity.rs"),
    ("lib.rs", "crates/happenstance-sync/src/lib.rs"),
    // The collision the projection rule family created, and the entry the
    // paragraph above promises: `crates/happenstance-testkit/src/projection.rs`
    // is the fourth rule file, so the basename stopped resolving uniquely the
    // day it landed and thirteen citations failed at once. Every one of them
    // predates that file and names the **port**: they cite the GAT that used to
    // be at `:97-99`, the module doc's batch paragraph, `ProjectionId`, and
    // `rollback`'s declaration — items that exist only in the contract crate.
    // The anchor check is the backstop, and it is a real one here rather than a
    // formality: the two files overlap in length, so a citation that meant the
    // suite would be sent to the port and its anchor would not be found.
    (
        "projection.rs",
        "crates/happenstance-core/src/projection.rs",
    ),
    // Fourteen manifests carry this name and the root's own relative path *is*
    // the bare name, so qualifying the two citations would not have changed the
    // string they contain. Both mean the workspace root, and §8036 says so in
    // the sentence around it: "inheriting `version` from the workspace root".
    ("Cargo.toml", "Cargo.toml"),
    // The collision phase 8 created, and the same shape as the projection one
    // above: `crates/happenstance-sqlite/tests/append.rs` is the adapter's own
    // append target, so the basename stopped resolving uniquely the day it
    // landed and **twenty-four** citations failed at once. Every one of them
    // predates that file and names the **contract**: `AppendCondition`, `Guard`,
    // `guards()`, `after`, `is_violated_by` and the module doc's precedence
    // paragraph — items that exist only in `happenstance-core`. A test file that
    // did not exist when the sentences were written cannot be what they meant,
    // and the anchor check is the backstop if any of them is.
    ("append.rs", "crates/happenstance-core/src/append.rs"),
];

/// What a citation's file name resolved to.
enum Target {
    /// A file in this repository.
    Path(PathBuf),
    /// Deliberately outside it — see [`EXTERNAL_CITATIONS`].
    External,
    /// Several files carry this name and no [`BARE_NAME_MAP`] entry picks one.
    Ambiguous(Vec<String>),
    /// No file in the workspace carries this name, and it is not external.
    Unknown,
}

/// A parsed citation.
struct Citation {
    /// The line of `SPECIFICATION.md` it appears on.
    spec_line: usize,
    /// The citation verbatim, for the error message.
    text: String,
    /// Where its file name resolved to.
    target: Target,
    /// The line it names.
    line: usize,
    /// The end of the range it names, when it names one — `404-427` ends at 427.
    line_end: usize,
    /// The identifier the sentence attributes to that location, if it names one.
    ///
    /// `None` for a `.md` target. A citation into Markdown is evidence for a
    /// *passage* — an argument, a scenario, a measurement — and the backticked
    /// word beside it is whatever the sentence happened to be discussing, not a
    /// definition that lives at that line. Anchoring them produced a third of
    /// this check's first run as false reports.
    subject: Option<String>,
}

/// The nearest backticked identifier before a citation — what the sentence says
/// is at the place it cites.
///
/// The document has one citation idiom and it is remarkably consistent: a
/// backticked identifier, then the citation, usually parenthesised.
///
/// ```text
/// `is_violated_by` compares raw values (`append.rs:239-253`)
/// ```
///
/// So the anchor need not be written into the citation the way `standards/rust`
/// writes it — it is already in the prose, and deriving it costs no change at
/// 358 sites. The search runs backwards across line breaks, because the
/// document wraps at 80 columns and a subject is frequently on the line above
/// its citation.
///
/// Returns `None` when the nearest span is not an identifier — a quoted phrase,
/// a type with generics, another citation — rather than guessing. A citation
/// with no derivable subject is counted and not anchored, which is the honest
/// outcome: this check tightens the ones it can read and never invents a claim
/// to check.
fn subject_before(spans: &[(usize, String)], i: usize) -> Option<String> {
    let (cite_line, _) = spans.get(i)?;
    let (subj_line, s) = spans.get(i.checked_sub(1)?)?;

    // Only the span *immediately* before, and only on the citation's own line or
    // the one above it. Four attempts got here, and the numbers are the argument
    // for how narrow it ended up: walking back up to four spans reported **118
    // of 316 anchored**; excluding `.md` targets and searching the whole cited
    // range rather than its first line took it to **70 of 262**; this
    // restriction took it to 10; declining when the subject appears nowhere in
    // the cited file took it to 2, and both of those were real defects.
    //
    // Nearly every false report was one shape: a sentence with no backticked
    // subject at all —
    // "the prohibition on arithmetic is already documented at `event.rs:215-217`"
    // — where reaching back far enough always finds *some* identifier, and it
    // belongs to the previous sentence. A derived anchor is only worth having
    // where the derivation is certain, so this declines rather than guesses.
    if cite_line.saturating_sub(*subj_line) > 1 {
        return None;
    }

    // An identifier may carry `::` qualifiers and a trailing `()`. Anything else
    // — spaces, attributes like `#[non_exhaustive]`, generics, another citation —
    // means the span is not a name and the anchor is not derivable.
    let core = s.trim().trim_end_matches("()");
    if core.is_empty()
        || !core
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == ':')
    {
        return None;
    }
    // `Type::method` anchors on the last segment: the definition site spells
    // `fn method`, not `Type::method`.
    let last = core.rsplit("::").next().unwrap_or(core);

    // A type name is a poor anchor even when it is the grammatical subject:
    // "`MemoryEventStore` already behaves that way by accident of ordering
    // (`memory.rs:372-388`)" cites the *behaviour*, and the type is declared four
    // hundred lines away. Names that start lower-case are functions, methods,
    // fields and rules, and those are cited at their definitions.
    if last.len() < 4 || last.starts_with(|c: char| c.is_uppercase()) {
        return None;
    }
    Some(last.to_owned())
}

/// Every file in the workspace that a citation could name, by base name.
///
/// Built once per run by walking the tree. `target/` is skipped because it holds
/// a copy of half the workspace under `package/`, and every one of those copies
/// would register as a second definition of a name that is otherwise unique —
/// turning the ambiguity check from a guard into noise. `.git` is skipped for
/// size alone.
///
/// `.claude/` is skipped for `target/`'s reason, in its purest form: linked git
/// worktrees live under `.claude/worktrees/`, and a linked worktree is a second
/// checkout of this same repository *inside* it. Every file in the workspace is
/// then found once per worktree plus once for real, so `store.rs` resolves four
/// ways and every bare-name citation in the specification reports as ambiguous.
///
/// That is not hypothetical. Running the gate from the repository root with
/// three worktrees present produced **153 traceability problems**, every one of
/// them a name the checker found four times — while the identical tree passed
/// from inside a worktree, because a worktree cannot see its siblings. A check
/// whose verdict depends on which directory it is invoked from is reporting the
/// invocation, not the specification.
fn workspace_index(root: &Path) -> BTreeMap<String, Vec<String>> {
    fn walk(dir: &Path, root: &Path, out: &mut BTreeMap<String, Vec<String>>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            if path.is_dir() {
                if name == "target" || name == ".git" || name == ".claude" || name == "node_modules"
                {
                    continue;
                }
                walk(&path, root, out);
            } else if [".rs", ".toml", ".md"].iter().any(|e| {
                name.len() > e.len() && name[name.len() - e.len()..].eq_ignore_ascii_case(e)
            }) && let Ok(rel) = path.strip_prefix(root)
            {
                out.entry(name)
                    .or_default()
                    .push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(root, root, &mut out);
    for paths in out.values_mut() {
        paths.sort();
    }
    out
}

/// `path:line` citations in backticks, as parsed [`Citation`]s.
///
/// # What this used to skip, and why that mattered
///
/// The filter was `!path.contains('/') || !(has_ext(".rs") || has_ext(".toml"))`
/// — a citation had to be path-qualified *and* name Rust or a manifest. The
/// document carries 338 citations and only 84 satisfy both: **200 are bare file
/// names and 56 name a `.md`**, and neither form was ever parsed, so neither was
/// ever checked. `check_citations` was not a weak check over the corpus; it was a
/// correct check over a quarter of it, and three quarters of the document's
/// evidence sat unverified behind a green step.
///
/// That is not a hypothetical exposure. Commit `2e4407b` found eight citations
/// that were "green and wrong" and repaired them by hand; the ones this widening
/// exposes are the same defect in the part of the corpus nobody could see.
fn citations(spec: &str, index: &BTreeMap<String, Vec<String>>) -> Vec<Citation> {
    // Every backticked span in the document, in order, with the line it sits on.
    // Collected up front rather than per line because a subject and its citation
    // are often on different lines — the document wraps at 80 columns and does
    // not treat the pair as unbreakable.
    let spans: Vec<(usize, String)> = spec
        .lines()
        .enumerate()
        .flat_map(|(n, line)| {
            line.split('`')
                .skip(1)
                .step_by(2)
                .map(move |c| (n + 1, c.to_owned()))
        })
        .collect();

    let mut out = Vec::new();
    for (i, (n, chunk)) in spans.iter().enumerate() {
        {
            let chunk = chunk.as_str();
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
            if !(has_ext(".rs") || has_ext(".toml") || has_ext(".md")) {
                continue;
            }
            // A path has no spaces. Dropping the `contains('/')` requirement let
            // whole backticked *sentences* through: a span reading
            // "[DEFERRED — settled by the experiment named in PRESSURE-TEST.md:688-693"
            // ends in `.md:<digits>` and `rsplit_once(':')` happily calls the
            // entire clause a path. The old filter excluded these by accident,
            // because prose rarely contains a slash; this excludes them on
            // purpose.
            if path.chars().any(char::is_whitespace) {
                continue;
            }
            let first: String = tail.chars().take_while(char::is_ascii_digit).collect();
            let Ok(num) = first.parse::<usize>() else {
                continue;
            };
            // `404-427` names a span, and the subject may be anywhere in it. The
            // first run of the anchor check searched only around the start line
            // and reported `into_parts` missing from `event.rs:404-427` because
            // the doc comment occupies the first fourteen lines of its own
            // citation.
            let end: String = tail
                .strip_prefix(&first)
                .and_then(|r| r.strip_prefix('-'))
                .map(|r| r.chars().take_while(char::is_ascii_digit).collect())
                .unwrap_or_default();
            let num_end = end.parse::<usize>().unwrap_or(num).max(num);

            let base = path.rsplit('/').next().unwrap_or(path);
            let target = if path.contains('/') {
                Target::Path(PathBuf::from(path))
            } else if EXTERNAL_CITATIONS.contains(&base) {
                Target::External
            } else if let Some((_, mapped)) = BARE_NAME_MAP.iter().find(|(n, _)| *n == base) {
                Target::Path(PathBuf::from(*mapped))
            } else {
                match index.get(base).map(Vec::as_slice) {
                    Some([only]) => Target::Path(PathBuf::from(only)),
                    Some(many) if many.len() > 1 => Target::Ambiguous(many.to_vec()),
                    _ => Target::Unknown,
                }
            };

            let markdown = has_ext(".md");
            out.push(Citation {
                spec_line: *n,
                text: chunk.to_owned(),
                target,
                line: num,
                line_end: num_end,
                subject: if markdown {
                    None
                } else {
                    subject_before(&spans, i)
                },
            });
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, reason = "test code, per the house style")]

    use std::fmt::Write as _;

    use super::*;

    /// This module's own path, read as text the way [`crate::lint_constitution`]
    /// reads a source file rather than through `include_str!`, so the assertion
    /// is about the file a contributor opens.
    const THIS_FILE: &str = "xtask/src/spec_trace.rs";

    /// The doc block attached to [`clause_ids`], attributes included.
    ///
    /// Everything after the last blank line before the definition. A doc comment
    /// and its attributes carry no blank line inside them and are separated from
    /// the item above by one, so the rule is exact — and cheaper than a
    /// line-shape predicate, which the multi-line `#[expect(…)]` already broke
    /// once. Splitting on the definition rather than searching the file for a
    /// heading is what keeps this module's own prose, which names both headings,
    /// out of the haystack.
    fn clause_ids_doc_block(source: &str) -> String {
        let head = source
            .split("pub(crate) fn clause_ids(")
            .next()
            .expect("split always yields a first part");
        let mut block: Vec<&str> = head
            .lines()
            .rev()
            .take_while(|line| !line.trim().is_empty())
            .collect();
        block.reverse();
        block.join("\n")
    }

    // ---- AC-001: the accessor exists and reads the pinned document ----------

    /// The only test that touches the real tree, and it touches it through the
    /// existing helpers rather than around them. `ES-1` and `VT-1` are declared
    /// at `spec/SPECIFICATION.md:2460` and `:563`.
    #[test]
    fn clause_ids_reads_the_pinned_specification() {
        let ids = clause_ids(&workspace_root().unwrap())
            .expect("the pinned specification must resolve against the real workspace root");

        assert!(
            !ids.is_empty(),
            "the real document declares clauses; an empty set is the checker being broken"
        );
        for id in ["ES-1", "VT-1"] {
            assert!(
                ids.contains(id),
                "the real document declares {id}; got {} ids",
                ids.len()
            );
        }
    }

    // ---- AC-002: prefixes come from `SECTIONS`, forms from the parser -------

    /// Iterating [`SECTIONS`] rather than a literal list of six prefixes is the
    /// whole point: a seventh family added in that one place cannot leave this
    /// test green by omission.
    #[test]
    fn every_section_family_resolves() {
        let mut document = String::from("## A slice shaped like the document\n\n");
        for section in SECTIONS {
            let _ = write!(
                document,
                "#### {}1 — a declaration in the {} family\n\n[FROZEN]\n\n\
                 **Rule:** none. Prose follows the declaration, as it does in the document.\n\n",
                section.prefix, section.prefix
            );
        }

        let ids = collect_clause_ids(&document).unwrap();

        for section in SECTIONS {
            assert!(
                ids.contains(&format!("{}1", section.prefix)),
                "the {} family must resolve; got {ids:?}",
                section.prefix
            );
        }
        assert_eq!(ids.len(), SECTIONS.len(), "got {ids:?}");
    }

    /// Both declaration forms are the parser's, inherited rather than
    /// re-implemented — and a bold run that closes immediately is a
    /// cross-reference, of which §1 is full.
    #[test]
    fn both_declaration_forms_resolve_and_a_cross_reference_does_not() {
        let document = "\
#### ES-1 — the heading form, which the earlier sections use

[FROZEN]

**Rule:** `es_1_two_flavours`.

**PS-1 — the bold-run form, which the later sections use.**

[DEFERRED — settled when the first projection adapter is built]

**CF-30 is [NON-NORMATIVE] and is prose.**

**ES-40** is named here the way §1 names a clause it is pointing at.
";

        let ids = collect_clause_ids(document).unwrap();

        for id in ["ES-1", "PS-1", "CF-30"] {
            assert!(ids.contains(id), "{id} is a declaration; got {ids:?}");
        }
        assert!(
            !ids.contains("ES-40"),
            "a bold run that closes immediately is a cross-reference, not a declaration; \
             got {ids:?}"
        );
    }

    // ---- AC-003: existence, never eligibility -------------------------------

    /// Applying [`has_suite`] here would silently dangle every `PS-` and `SY-`
    /// citation, so the test asserts both halves: the id resolves, *and* the
    /// filter that must not be on the path would have excluded it.
    #[test]
    fn a_deferred_clause_without_a_suite_still_resolves() {
        let document = "\
#### SY-7 — ingest re-checks the append condition against the receiving store

[DEFERRED — settled by the first peer adapter]

**Rule:** none. No `SyncPeer` conformance suite exists yet.
";

        let ids = collect_clause_ids(document).unwrap();

        assert!(
            ids.contains("SY-7"),
            "a deferred clause is declared, so its id resolves; got {ids:?}"
        );
        assert!(
            !has_suite("SY-7"),
            "if this ever becomes true the test above stops rejecting a maturity filter"
        );
    }

    // ---- AC-004: two hard errors, each naming the artifact that broke -------

    #[test]
    fn an_unreadable_specification_names_the_file_it_could_not_read() {
        let err = clause_ids(Path::new("no/such/root"))
            .expect_err("an unreadable document must never degrade to an empty set");

        let chain = format!("{err:#}");
        assert!(
            chain.contains(&format!("reading {SPEC}")),
            "the chain must carry `read`'s context naming the document, got: {chain}"
        );
    }

    /// The failure `wire_rules` already refuses to defer: a document that parses
    /// to nothing is the checker being broken, and saying so here is what stops
    /// both consumers naming real pages and real pinned ids instead.
    #[test]
    fn a_specification_that_declares_nothing_blames_the_checker() {
        let err = collect_clause_ids("# A document with no declarations\n\nProse only.\n")
            .expect_err("a document that declares nothing must be a hard error");

        let message = err.to_string();
        assert!(
            message.starts_with(SPEC),
            "the artifact comes first, so soft-wrap cannot push it off the first visual \
             row, got: {message}"
        );
        assert!(
            message.contains("the checker being broken rather than the document"),
            "the message must blame the checker rather than the document, got: {message}"
        );
        assert!(
            !message.contains("more"),
            "one composed sentence, never an elided list, got: {message}"
        );
    }

    // ---- AC-005: the limits are proven, then stated, and stated first -------

    /// RS-81-1's order: prove the blind spot in the tests, then state it in the
    /// documentation. A `BTreeSet` cannot see the second declaration, and
    /// nothing in this repository checks for one.
    #[test]
    fn a_doubly_declared_id_collapses_to_one() {
        let document = "\
#### ES-1 — declared once

**Rule:** `es_1_two_flavours`.

#### ES-1 — and declared a second time, which nothing here notices

**Rule:** `es_1_two_flavours`.
";

        let ids = collect_clause_ids(document).unwrap();

        assert_eq!(ids.len(), 1, "got {ids:?}");
        assert!(ids.contains("ES-1"));
    }

    #[test]
    fn the_accessor_documents_its_limits_before_its_errors() {
        let source = read(&workspace_root().unwrap(), THIS_FILE).unwrap();
        let block = clause_ids_doc_block(&source);

        let limits = block
            .find("does not verify")
            .unwrap_or_else(|| panic!("the doc block states no limits at all: {block}"));
        let errors = block
            .find("# Errors")
            .unwrap_or_else(|| panic!("the doc block has no `# Errors` section: {block}"));

        assert!(
            limits < errors,
            "the limits must come before the errors; a check whose limits are undocumented \
             is read as a guarantee: {block}"
        );
        for claim in ["badge", "shield", "✓"] {
            assert!(
                !block.contains(claim),
                "`{claim}` would claim a resolving citation is a correct one"
            );
        }
    }

    /// Check 4 abstains on any family [`has_suite`] does not know, so the set of
    /// prefixes it names is the set of clause families whose rule citations are
    /// checked at all. `PS` was excluded for a true reason — the projection
    /// suite did not exist — and the exclusion outlived it.
    ///
    /// Asserted rather than reviewed because the failure is silence: a `PS`
    /// clause could name `commit_is_atomik_with_the_read_model` and every gate in
    /// the repository would stay green.
    #[test]
    fn the_projection_family_is_checked_against_its_suite() {
        assert!(
            has_suite("PS-1"),
            "the projection suite exists in `crates/happenstance-testkit/src/projection.rs`, \
             so check 4 must resolve `PS` rule citations rather than abstain on them"
        );
        assert!(has_suite("PS-37"));
    }

    /// The other half of the same constant, and the reason this test is not
    /// merely `assert!(has_suite(..))` four times: the families that already had
    /// a suite must keep one, and the family that still has none must keep
    /// abstaining. Widening the prefix set to everything would make check 4
    /// report every unwritten replication rule as a typo — noise indistinguishable
    /// from the real thing, which is the defect the exclusion was written to avoid.
    #[test]
    fn the_replication_family_still_abstains_and_the_rest_do_not() {
        for existing in ["ES-1", "VT-3", "WF-8"] {
            assert!(
                has_suite(existing),
                "{existing} has had a suite since phase 3"
            );
        }
        assert!(
            !has_suite("SY-1"),
            "`happenstance-sync-testkit` does not exist; checking `SY` rule names against \
             the suites that do exist would report all of them as missing"
        );
        assert!(
            !has_suite("CF-1"),
            "`CF` clauses are about the suite, not checked by it"
        );
    }

    /// [`RULE_FILES`] is what `all_rules` sweeps, and check 6 demands every rule
    /// in it be claimed by a clause, retired by one, or on record as owing a
    /// decision. A projection family absent from the array is a family no clause
    /// is obliged to claim — the same hole `concurrency.rs` sat in for a phase.
    #[test]
    fn the_projection_rules_file_is_swept_for_ownership() {
        assert!(
            RULE_FILES.contains(&"crates/happenstance-testkit/src/projection.rs"),
            "check 6 sweeps only {RULE_FILES:?}"
        );
    }
}
