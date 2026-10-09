//! GitHub Issues is the tracker (ADR-0088), and the runbook says which issue
//! carries each piece of open work. These two checks hold it to saying so.
//!
//! # What they do not verify
//!
//! Both are offline. Neither asks GitHub whether issue `#N` exists, is open, or
//! is about the work it is written beside: a well-formed number that points at
//! the wrong issue passes. What they catch is the cheaper failure that happens
//! first — a phase or a work item added with no issue at all, or two phases
//! pointed at one issue, so that closing it reports progress on both.

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fs;
use std::io::{self, Write as _};
use std::num::NonZeroU32;

use anyhow::{Context, Result, bail};

use super::{PhaseRow, RUNBOOK, phase_rows, runbook_status_matches_the_registry, unmark};
use crate::spec_trace::workspace_root;

/// The directory holding one file per open phase.
const RUNBOOK_PHASES: &str = "runbook/phases";

/// The status table's other checks, then both of this module's.
///
/// One entry point so that `stated_rule_counts` wires all three through the
/// single line that used to call `runbook_status_matches_the_registry` alone.
/// `lints.rs` is cited by line number from `references/adr/`, `.kb/` and
/// `standards/rust/`, and an extra call line there would move every citation
/// below it.
///
/// # Errors
///
/// Returns the first failing check's error: the registry axes, then the
/// status table's Tracker column, then the phase files' work items.
pub(super) fn runbook_status_and_tracking() -> Result<()> {
    runbook_status_matches_the_registry()?;
    let root = workspace_root()?;
    let runbook =
        fs::read_to_string(root.join(RUNBOOK)).with_context(|| format!("reading {RUNBOOK}"))?;
    status_rows_are_tracked(&phase_rows(&runbook)?)?;
    runbook_work_items_are_tracked()
}

/// The `Tracker` cell of one row of the status table.
#[derive(Debug, PartialEq, Eq)]
enum Tracker {
    /// The row has no sixth cell. Distinct from [`Tracker::Untracked`] because
    /// a row someone forgot to widen is not a row someone decided tracks
    /// nothing.
    Absent,
    /// `—`: the row tracks nothing, which only a finished phase or a
    /// milestone may say.
    Untracked,
    /// `#N`, with `N` a positive issue number.
    Issue(NonZeroU32),
    /// Anything else, as written once its markdown is removed.
    Malformed(String),
}

impl Tracker {
    /// The sixth cell of `row`, read from the row as written.
    ///
    /// Read here rather than stored on [`PhaseRow`] by `phase_rows`, because a
    /// field there would add lines above citations into `lints.rs`.
    fn of(row: &PhaseRow) -> Self {
        Self::parse(row.raw.trim_matches('|').split('|').nth(5))
    }

    /// The cell's meaning; `None` when the row has no sixth cell.
    fn parse(cell: Option<&str>) -> Self {
        let Some(cell) = cell else {
            return Self::Absent;
        };
        let text = unmark(cell);
        if text == "—" {
            return Self::Untracked;
        }
        match issue_number(&text) {
            Some(n) => Self::Issue(n),
            None => Self::Malformed(text),
        }
    }
}

/// `#N` as a positive issue number, or `None` for any other spelling.
///
/// The digits are checked before they are parsed because `str::parse` accepts
/// a leading `+`, and `#+5` is not how anyone writes an issue reference. A
/// leading `0` is refused too: `#061` would parse as 61 and collide with `#61`
/// under a spelling nobody searching for `#61` would find.
fn issue_number(text: &str) -> Option<NonZeroU32> {
    let digits = text.strip_prefix('#')?;
    if digits.starts_with('0') || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    // An empty string and overflow both land here, and both are "not an issue
    // number". Zero was refused above as a leading `0`.
    digits.parse().ok()
}

/// Every way `rows` fails to say which issue tracks each live phase.
///
/// A row is live when it is a phase (its `#` cell is not `—`) that is not
/// `done`. A live row must carry `#N`. A finished phase or a milestone may
/// carry `—` or `#N`. No row may carry a malformed cell or omit the cell, and
/// no two rows may carry the same `N`.
fn tracker_problems(rows: &[PhaseRow]) -> Vec<String> {
    let mut problems = Vec::new();
    let mut first_line: BTreeMap<NonZeroU32, usize> = BTreeMap::new();
    for row in rows {
        let live = row.number != "—" && row.state != "done";
        match Tracker::of(row) {
            Tracker::Issue(n) => match first_line.entry(n) {
                Entry::Vacant(slot) => {
                    slot.insert(row.line);
                }
                Entry::Occupied(earlier) => problems.push(format!(
                    "{RUNBOOK}:{} — `{}` is tracked by #{n}, which {RUNBOOK}:{} already \
                     names. Closing one issue would report progress on two phases.",
                    row.line,
                    row.phase,
                    earlier.get()
                )),
            },
            Tracker::Untracked if live => problems.push(format!(
                "{RUNBOOK}:{} — phase {} reads `{}` and its Tracker cell is `—`. Open work \
                 names the issue that tracks it (ADR-0088).",
                row.line, row.number, row.state
            )),
            Tracker::Untracked => {}
            Tracker::Absent => problems.push(format!(
                "{RUNBOOK}:{} — `{}` has no Tracker cell. Write `#N`, or `—` if the row is \
                 done or a milestone.",
                row.line, row.phase
            )),
            Tracker::Malformed(text) => problems.push(format!(
                "{RUNBOOK}:{} — `{}`'s Tracker cell `{text}` is not `#N` or `—`.",
                row.line, row.phase
            )),
        }
    }
    problems
}

/// The status table's Tracker column.
///
/// # Errors
///
/// Returns an error listing every problem [`tracker_problems`] finds, or if no
/// row carries an issue at all, which would leave the duplicate check holding
/// nothing.
fn status_rows_are_tracked(rows: &[PhaseRow]) -> Result<()> {
    let problems = tracker_problems(rows);
    if !problems.is_empty() {
        bail!(
            "{} row(s) of {RUNBOOK}'s status table do not say which issue tracks them:\n  {}",
            problems.len(),
            problems.join("\n  ")
        );
    }
    let tracked = rows
        .iter()
        .filter(|r| matches!(Tracker::of(r), Tracker::Issue(_)))
        .count();
    if tracked == 0 {
        bail!(
            "no row of {RUNBOOK}'s status table carries an issue number, so the Tracker \
             column holds nothing."
        );
    }
    writeln!(
        io::stdout().lock(),
        "status_rows_are_tracked: {tracked} status row(s) name distinct issues"
    )?;
    Ok(())
}

/// What [`scan_work_items`] found in one phase file.
#[derive(Debug)]
struct WorkItemScan {
    /// Unticked boxes that carry their issue numbers.
    tracked: usize,
    /// One message per unticked box that does not, naming its `file:line`.
    problems: Vec<String>,
}

/// The heading text of `line`, or `None` if it is not a heading.
///
/// A heading is an unindented `#` line, or an unindented line opening in bold:
/// `**Work**`, `**Exit criteria**`, and the paragraph leads `**Goal.**` and
/// `**Proof artefact.**` that end the section above them.
fn heading_text(line: &str) -> Option<&str> {
    if line.starts_with('#') {
        Some(line.trim_start_matches('#').trim_start())
    } else if line.starts_with("**") {
        Some(line.trim_start_matches('*'))
    } else {
        None
    }
}

/// Whether a box's text, after `- [ ] `, opens with `#N, #N · `.
fn opens_with_issues(text: &str) -> bool {
    text.split_once(" · ")
        .is_some_and(|(list, _)| list.split(", ").all(|item| issue_number(item).is_some()))
}

/// Every unticked box in `body` that is not an exit criterion, checked for its
/// issue numbers.
///
/// The scope is every section except the exit criteria, rather than only the
/// sections headed `**Work**`: phase 19's work sits under `## 19a` and `## 19b`
/// with no Work heading, and an allowlist would hold none of it. A box is exempt
/// when it is ticked, when its text opens with `~~` (struck: moved or withdrawn,
/// and tracked where it went), or when it sits under a heading reading
/// `Exit criteria`. Lines inside a fenced code block are skipped, so an example
/// box is not an item and a `#` comment is not a heading.
///
/// Only unindented boxes are read. A nested box is a step of the item above it,
/// which carries the issue.
fn scan_work_items(file: &str, body: &str) -> WorkItemScan {
    let mut scan = WorkItemScan {
        tracked: 0,
        problems: Vec::new(),
    };
    let mut in_fence = false;
    let mut exit_criteria = false;
    for (index, line) in body.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        if let Some(heading) = heading_text(line) {
            exit_criteria = heading.starts_with("Exit criteria");
            continue;
        }
        let Some(text) = line.strip_prefix("- [ ]") else {
            continue;
        };
        if exit_criteria || text.trim_start().starts_with("~~") {
            continue;
        }
        if text.strip_prefix(' ').is_some_and(opens_with_issues) {
            scan.tracked += 1;
        } else {
            scan.problems.push(format!(
                "{file}:{} — an unticked work item with no issue. Open it on GitHub and \
                 write `- [ ] #N · ` (or `#N, #M · `) before its text (ADR-0088).",
                index + 1
            ));
        }
    }
    scan
}

/// Every unticked work item in `runbook/phases/*.md` names the issue that
/// tracks it.
///
/// # Errors
///
/// Returns an error if the directory or a file in it cannot be read, if any
/// work item carries no well-formed issue list, or if no work item carries one
/// at all, which would mean the grammar moved and this check holds nothing.
fn runbook_work_items_are_tracked() -> Result<()> {
    let root = workspace_root()?;
    let dir = root.join(RUNBOOK_PHASES);
    let mut files = Vec::new();
    for entry in fs::read_dir(&dir).with_context(|| format!("reading {RUNBOOK_PHASES}"))? {
        let entry = entry.with_context(|| format!("reading an entry of {RUNBOOK_PHASES}"))?;
        let path = entry.path();
        if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
        {
            files.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    files.sort();

    let mut tracked = 0usize;
    let mut problems = Vec::new();
    for name in &files {
        let rel = format!("{RUNBOOK_PHASES}/{name}");
        let body = fs::read_to_string(dir.join(name)).with_context(|| format!("reading {rel}"))?;
        let scan = scan_work_items(&rel, &body);
        tracked += scan.tracked;
        problems.extend(scan.problems);
    }

    if !problems.is_empty() {
        bail!(
            "runbook_work_items_are_tracked: {} work item(s) under {RUNBOOK_PHASES} name no \
             issue:\n  {}",
            problems.len(),
            problems.join("\n  ")
        );
    }
    if tracked == 0 {
        bail!(
            "runbook_work_items_are_tracked: no unticked work item under {RUNBOOK_PHASES} \
             carries an issue number, so the grammar this check reads has moved."
        );
    }
    writeln!(
        io::stdout().lock(),
        "runbook_work_items_are_tracked: {tracked} work item(s) across {} phase file(s) name \
         their issues",
        files.len()
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lints::phase_rows;

    const TABLE: &str = "\
## Status

| # | Phase | Depends on | State | Proof artefact | Tracker |
|---|---|---|---|---|---|
| 12 | [Publish](#p12) | 7 | done | seven crates | — |
| — | **`0.2.0`** | 12 | — | released | — |
| 17 | [Window](phases/17.md) | 16 | in progress | `0.4.0` released | #61 |
| 18 | [Runner](phases/18.md) | 17 | not started | the example | #63 |
| 19 | [Wasm](phases/19.md) | 17 | blocked | the skeleton | #64 |
";

    fn problems(table: &str) -> Vec<String> {
        tracker_problems(&phase_rows(table).expect("the fixture has a status table"))
    }

    #[test]
    fn done_and_milestone_rows_carrying_a_dash_pass() {
        assert_eq!(problems(TABLE), Vec::<String>::new());
    }

    /// Rejects a check that only validates a cell when one is written: a live
    /// row reading `—`, or a row with no sixth cell at all, tracks nothing.
    /// Each live state is dashed in turn, which rejects a `live` test that
    /// knows only `not started`.
    #[test]
    fn a_live_row_without_an_issue_is_refused() {
        for (issue, line, number) in [("#61", 7, 17), ("#63", 8, 18), ("#64", 9, 19)] {
            let dashed = TABLE.replace(&format!("| {issue} |"), "| — |");

            let found = problems(&dashed);

            assert_eq!(found.len(), 1, "{issue}: {found:?}");
            assert!(
                found[0].starts_with(&format!("runbook/README.md:{line} — phase {number}")),
                "{issue}: {found:?}"
            );
        }

        let short = problems(&TABLE.replace(" | #63 |", " |"));

        assert_eq!(short.len(), 1, "{short:?}");
        assert!(short[0].contains("no Tracker cell"), "{short:?}");
    }

    /// Rejects a check that parses each cell in isolation: two phases pointing
    /// at one issue means one of them is tracked by nothing.
    #[test]
    fn two_rows_sharing_an_issue_are_refused_naming_both_lines() {
        let shared = TABLE.replace("| #63 |", "| #61 |");

        let found = problems(&shared);

        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].starts_with("runbook/README.md:8"), "{found:?}");
        assert!(
            found[0].contains("#61") && found[0].contains(":7"),
            "{found:?}"
        );
    }

    /// Rejects a check that asks only "does the cell start with `#`", and
    /// one that hands the digits to `str::parse`, which accepts `+5`.
    #[test]
    fn a_malformed_issue_number_is_refused() {
        for bad in ["#x", "#0", "#+5", "#061", "# 63", "63", "#63a"] {
            let table = TABLE.replace("| #63 |", &format!("| {bad} |"));

            let found = problems(&table);

            assert_eq!(found.len(), 1, "{bad}: {found:?}");
            assert!(found[0].contains("is not `#N`"), "{bad}: {found:?}");
        }
    }

    #[test]
    fn the_tracker_cell_parses_each_shape() {
        assert_eq!(Tracker::parse(None), Tracker::Absent);
        assert_eq!(Tracker::parse(Some("—")), Tracker::Untracked);
        assert_eq!(
            Tracker::parse(Some("**#61**")),
            Tracker::Issue(NonZeroU32::new(61).expect("61 is not zero"))
        );
        assert_eq!(
            Tracker::parse(Some("#x")),
            Tracker::Malformed("#x".to_owned())
        );
    }

    const PHASE: &str = "\
# Phase 99 — a fixture

**Goal.** Something.

**Work**

- [ ] #83 · **A tracked item**, wrapped
      onto a second line.
- [ ] #102, #103 · Two issues for one item.
- [x] A finished item needs no issue.
- [ ] ~~**A moved item.**~~ Moved to phase 17b.

**Proof artefact.** Something observable.

**Exit criteria**

- [ ] The specification is reconciled.

**Session log**
";

    fn scan(body: &str) -> WorkItemScan {
        scan_work_items("runbook/phases/99.md", body)
    }

    /// The real grammar passes: numbered, multi-numbered, ticked, struck and
    /// exit-criteria boxes. Rejects a parser that reads only the first `#N`
    /// and then demands the separator, which refuses `#102, #103 ·`.
    #[test]
    fn the_shipped_grammar_passes() {
        let found = scan(PHASE);

        assert_eq!(found.problems, Vec::<String>::new());
        assert_eq!(found.tracked, 2);
    }

    /// Rejects a check that only validates a number when one is written.
    #[test]
    fn an_unannotated_work_box_is_refused_at_its_line() {
        let body = PHASE.replace("- [ ] #83 · ", "- [ ] ");

        let found = scan(&body);

        assert_eq!(found.problems.len(), 1, "{:?}", found.problems);
        assert!(
            found.problems[0].starts_with("runbook/phases/99.md:7 — "),
            "{:?}",
            found.problems
        );
    }

    /// Rejects a check satisfied by any `#` at all.
    #[test]
    fn a_malformed_issue_list_is_refused() {
        for bad in ["#x · ", "#83 ", "#1,#2 · ", "#1, · ", "# 83 · ", "#083 · "] {
            let body = PHASE.replace("#83 · ", bad);

            let found = scan(&body);

            assert_eq!(found.problems.len(), 1, "{bad}: {:?}", found.problems);
        }
    }

    /// Rejects an exemption that never ends: once `**Exit criteria**` is
    /// seen, a scanner that does not leave it at the next heading exempts every
    /// box below it.
    #[test]
    fn the_exit_criteria_exemption_ends_at_the_next_heading() {
        let body = format!("{PHASE}\n## 99b — a sub-phase\n\n- [ ] An untracked item.\n");

        let found = scan(&body);

        assert_eq!(found.problems.len(), 1, "{:?}", found.problems);
        assert!(
            found.problems[0].starts_with("runbook/phases/99.md:23 —"),
            "{:?}",
            found.problems
        );
    }

    /// Phase 19 has no `**Work**` heading: its boxes sit under `## 19a` and
    /// `## 19b`. Rejects an allowlist keyed on a Work heading, which would
    /// hold none of phase 19's nine items.
    #[test]
    fn a_box_under_a_sub_phase_heading_is_held() {
        let body = "# Phase 19\n\n## 19a — the skeleton\n\n- [ ] Untracked.\n";

        let found = scan(body);

        assert_eq!(found.problems.len(), 1, "{:?}", found.problems);
    }

    /// A box inside a fence is an example, and a `#` comment inside a fence is
    /// not a heading that ends the exit-criteria exemption.
    #[test]
    fn fenced_lines_are_neither_boxes_nor_headings() {
        let body = "**Exit criteria**\n\n```text\n# not a heading\n- [ ] not a box\n```\n\n- [ ] An exit criterion.\n";

        let found = scan(body);

        assert_eq!(found.problems, Vec::<String>::new());
        assert_eq!(found.tracked, 0);
    }
}
