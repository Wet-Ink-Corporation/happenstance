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

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

const SPEC: &str = "docs/architecture/SPECIFICATION.md";
const CASES: &str = "docs/scenarios/E2E-CASES.md";
const SUITE: &str = "crates/happenstance-testkit/src/suite.rs";

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
    cases: Vec<String>,
    retires: Vec<String>,
    has_rejects: bool,
}

/// Runs the traceability check.
///
/// # Errors
///
/// Returns an error if a document cannot be read, or if any check fails.
pub(crate) fn run() -> Result<()> {
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
    for (line_no, citation, path, line) in citations(&spec) {
        let full = root.join(&path);
        match fs::read_to_string(&full) {
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

    report(&clauses, &known_rules, &known_cases, &problems)
}

fn report(
    clauses: &[Clause],
    rules: &BTreeSet<String>,
    cases: &BTreeSet<String>,
    problems: &[String],
) -> Result<()> {
    let mut by_maturity: BTreeMap<&str, usize> = BTreeMap::new();
    for c in clauses {
        *by_maturity
            .entry(c.maturity.as_deref().unwrap_or("(none)"))
            .or_default() += 1;
    }

    let mut summary = String::new();
    let _ = write!(summary, "{} clauses (", clauses.len());
    let parts: Vec<String> = by_maturity
        .iter()
        .map(|(k, v)| format!("{v} {k}"))
        .collect();
    let _ = write!(summary, "{}), ", parts.join(", "));
    let _ = write!(
        summary,
        "{} suite rules, {} e2e cases",
        rules.len(),
        cases.len()
    );
    println!("{summary}");

    if problems.is_empty() {
        println!("traceability: no problems found");
        return Ok(());
    }

    for p in problems {
        println!("  {p}");
    }
    bail!(
        "{} traceability problem(s). These are defects in the specification, not in the checker — \
         §7.2 was computed by hand and has never been verified.",
        problems.len()
    )
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
        out.push(Clause {
            id: id.clone(),
            line: start + 1,
            maturity: maturity_of(&body),
            falsifier: falsifier_of(&body),
            rules: rules_of(&body).0,
            schedules_new: rules_of(&body).1,
            cases: cases_of(&body),
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
    let prefix = ["ES-", "PS-", "SY-", "VT-", "WF-", "CF-"]
        .into_iter()
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
    const ALL: [&str; 4] = ["FROZEN", "PROVISIONAL", "DEFERRED", "NON-NORMATIVE"];

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
    Some(out)
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
fn rules_of(body: &str) -> (Vec<String>, bool) {
    let Some(text) = field_line(body, "Rule") else {
        return (Vec::new(), false);
    };
    // A clause with nothing of its own claims nothing.
    if text.trim_start().to_ascii_lowercase().starts_with("none") {
        return (Vec::new(), false);
    }
    // Not every backticked identifier on a `Rule:` line is a conformance rule.
    // The specification distinguishes, in prose, between a suite rule and a unit
    // or compile test living in the crate it constrains — `position_next_signals_overflow`
    // is a unit test in `event.rs`, and no amount of looking in `suite.rs` will
    // find it. Treating those as missing rules is the checker misreading the
    // document rather than the document being wrong.
    let elsewhere = text.contains("unit test") || text.contains("compile test");
    let schedules_new = text.contains("(new)")
        || text.contains('†')
        || text.trim_start().starts_with("new ")
        || text.contains(" new `")
        || elsewhere;
    (backticked_idents(&text), schedules_new)
}

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

/// Every rule the conformance suite actually defines.
fn collect_rules(suite: &str) -> BTreeSet<String> {
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

fn workspace_root() -> Result<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .context("xtask must live one level below the workspace root")
}
