//! CF-25 and CF-26: a port freeze against §6.5's instrument portfolio (#125).
//!
//! CF-25 forbids declaring a port `[FROZEN]` while an axis it sits on has no
//! passing implementation at its far end, unless the freeze names the axis and
//! the record accepting the risk. CF-26 says a fixture at the far end does not
//! count. Both clauses name this checker, and until #125 nothing in it read the
//! portfolio at all.
//!
//! # The reading
//!
//! CF-25's `Rule:` line, taken literally: the check fails "when a `[FROZEN]`
//! port clause has an axis with no far-end row and no named risk acceptance".
//!
//! - **Which ports are walked.** Every port with at least one `[FROZEN]` clause
//!   under its §1.4 prefix, and every port §1.6 declares **Frozen**. §1.6's
//!   `Maturity` cell is read against a closed vocabulary, so a lead the check
//!   cannot classify is a failure rather than a silent "not frozen".
//! - **§6.5's portfolio.** Each axis's name, and the bold verdict that opens its
//!   `Far end exists?` cell. Only a verdict that says *adapter yes* puts an
//!   adapter at the far end (CF-26). Nothing else in a row is read.
//! - **§6.5's freeze ledger.** One row per axis a port sits on, saying how that
//!   port's freeze discharges it.
//!
//! # What it does not read
//!
//! The near end, whether a port really sits on no axis it has no ledger row
//! for, and what an accepting record says beyond naming the axis and the port.
//! [`summarise`] prints that on every run, so a pass is not read as more than
//! it is.

use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use super::Clause;
use breach::{Breach, Cause, Finding, RowProblem, Verdict};
use table::{Table, bold_lead, name_in, read_table};

mod breach;
mod table;

/// The heading the portfolio table sits under.
const PORTFOLIO: &str = "#### The portfolio";
/// The heading the freeze ledger sits under.
const LEDGER: &str = "#### The freeze ledger";
/// The heading prefix of §1.6, the port table.
const PORTS: &str = "### 1.6 ";
/// The heading prefix of §1.4, the clause-prefix table.
const PREFIXES: &str = "### 1.4 ";
/// The discharge a ledger row writes when nothing discharges it yet.
const UNDISCHARGED: &str = "undischarged";

/// What one accepting record says, as far as this check reads it.
pub(super) struct Record {
    /// Whether the decision atom's frontmatter reads `status: accepted`.
    pub(super) accepted: bool,
    /// The atom and, when there is one, its long-form record, concatenated.
    pub(super) text: String,
}

/// How a ledger row says a frozen port discharges an axis.
#[derive(Debug, PartialEq, Eq)]
enum Discharge {
    /// An adapter at the far end, per the portfolio's verdict.
    FarEnd,
    /// A record accepting the risk, by its four-digit number.
    Record(String),
    /// A clause of the port that is not yet frozen, by its ID.
    Carried(String),
    /// Nothing, said out loud: a gap recorded where a reader will meet it.
    Undischarged,
}

/// What §1.6's `Maturity` cell says, read by its bold lead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Maturity {
    Frozen,
    NotFrozen,
    Unreadable,
}

/// What a run walked, for the summary line.
#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Walk {
    /// Axes in the portfolio table.
    pub(super) axes: usize,
    /// Ports walked: a `[FROZEN]` clause, or a §1.6 declaration. §1.4 order.
    pub(super) frozen: Vec<String>,
    /// Frozen ports' rows discharged by an adapter at the far end.
    pub(super) far_end: usize,
    /// Frozen ports' rows discharged by an accepting record.
    pub(super) accepted: usize,
    /// Frozen ports' rows carried by an open clause.
    pub(super) carried: usize,
    /// Rows of ports with nothing frozen, which owe nothing yet.
    pub(super) unwalked: usize,
}

/// A port, merged from §1.4 and §1.6, and why it is walked, if it is.
struct Port {
    name: String,
    prefix: Option<String>,
    /// The line a missing ledger row is reported against.
    line: usize,
    cause: Option<Cause>,
}

/// Checks every frozen port against §6.5's portfolio and freeze ledger.
///
/// `record` looks up an accepting record by its four-digit number, and is a
/// parameter so the tests can hand in records the tree does not hold.
///
/// # Errors
///
/// Only when `record` fails to read a record that exists.
pub(super) fn check<F>(spec: &str, clauses: &[Clause], record: F) -> Result<(Walk, Vec<Finding>)>
where
    F: Fn(&str) -> Result<Option<Record>>,
{
    let lines: Vec<&str> = spec.lines().collect();
    let mut found = Vec::new();
    let mut need = |heading: &str, columns: &[&str], what: &'static str| {
        let table = read_table(&lines, heading, columns);
        if table.is_none() {
            found.push(Finding {
                line: None,
                breach: Breach::MissingTable { what },
            });
        }
        table
    };
    let ports = need(
        PORTS,
        &["Port", "Maturity"],
        "§1.6 port table under `### 1.6` with `Port` and `Maturity` columns",
    );
    let prefixes = need(
        PREFIXES,
        &["Prefix", "Surface"],
        "§1.4 prefix table under `### 1.4` with `Prefix` and `Surface` columns",
    );
    let portfolio = need(
        PORTFOLIO,
        &["Axis", "Far end exists?"],
        "portfolio table under `#### The portfolio` with `Axis` and `Far end exists?` columns",
    );
    let ledger = need(
        LEDGER,
        &["Axis", "Port", "Discharged by"],
        "freeze ledger under `#### The freeze ledger` with `Axis`, `Port` and `Discharged by` \
         columns",
    );
    let (Some(ports), Some(prefixes), Some(portfolio), Some(ledger)) =
        (ports, prefixes, portfolio, ledger)
    else {
        return Ok((Walk::default(), found));
    };

    let known = known_ports(&prefixes, &ports, clauses, &mut found);
    let portfolio_axes = axes_of(&portfolio, &mut found);
    let mut walker = Walker {
        clauses,
        record: &record,
        axes: &portfolio_axes,
        walk: Walk {
            axes: portfolio_axes.len(),
            frozen: known
                .iter()
                .filter(|p| p.cause.is_some())
                .map(|p| p.name.clone())
                .collect(),
            ..Walk::default()
        },
        found,
    };

    walker.rows(&ledger, &known)?;
    Ok((walker.walk, walker.found))
}

/// §1.4's ports and §1.6's, merged by name in §1.4's order, each with why it
/// is walked. A §1.6 `Maturity` lead outside the vocabulary is reported here.
fn known_ports(
    prefixes: &Table,
    ports: &Table,
    clauses: &[Clause],
    found: &mut Vec<Finding>,
) -> Vec<Port> {
    let mut known: Vec<Port> = prefixes
        .rows
        .iter()
        .filter_map(|row| {
            let prefix = row.cell(0).trim().trim_matches('`').strip_suffix('n')?;
            let name = row
                .cell(1)
                .trim()
                .strip_prefix("the `")?
                .strip_suffix("` port")?;
            Some(Port {
                name: name.to_owned(),
                prefix: Some(prefix.to_owned()),
                line: row.line,
                cause: None,
            })
        })
        .collect();
    let mut declared: BTreeSet<String> = BTreeSet::new();
    for row in &ports.rows {
        let name = name_in(row.cell(0));
        match maturity(row.cell(1)) {
            Maturity::Frozen => {
                declared.insert(name.to_owned());
            }
            Maturity::NotFrozen => {}
            Maturity::Unreadable => found.push(Finding {
                line: Some(row.line),
                breach: Breach::UnreadableMaturity {
                    port: name.to_owned(),
                    maturity: lead_or_cell(row.cell(1)).to_owned(),
                },
            }),
        }
        match known.iter_mut().find(|p| p.name == name) {
            Some(port) => port.line = row.line,
            None => known.push(Port {
                name: name.to_owned(),
                prefix: None,
                line: row.line,
                cause: None,
            }),
        }
    }
    for port in &mut known {
        port.cause = cause_of(port, clauses, &declared);
    }
    known
}

/// Why a port is walked: its first `[FROZEN]` clause, else a §1.6 declaration.
fn cause_of(port: &mut Port, clauses: &[Clause], declared: &BTreeSet<String>) -> Option<Cause> {
    let frozen: Vec<&Clause> = port.prefix.as_deref().map_or_else(Vec::new, |prefix| {
        clauses
            .iter()
            .filter(|c| c.id.starts_with(prefix) && c.maturity.as_deref() == Some("FROZEN"))
            .collect()
    });
    if let [first, ..] = frozen.as_slice() {
        port.line = first.line;
        return Some(Cause::Clauses {
            first: first.id.clone(),
            count: frozen.len(),
        });
    }
    declared.contains(&port.name).then_some(Cause::Declared)
}

/// The state one [`check`] threads through its ledger rows.
struct Walker<'a, F> {
    clauses: &'a [Clause],
    record: &'a F,
    /// Axis name to its portfolio line and verdict.
    axes: &'a BTreeMap<String, (usize, Verdict)>,
    walk: Walk,
    found: Vec<Finding>,
}

impl<F> Walker<'_, F>
where
    F: Fn(&str) -> Result<Option<Record>>,
{
    fn push(&mut self, line: usize, breach: Breach) {
        self.found.push(Finding {
            line: Some(line),
            breach,
        });
    }

    /// Walks every ledger row, then reports what the rows left uncovered.
    fn rows(&mut self, ledger: &Table, known: &[Port]) -> Result<()> {
        let mut attributed: BTreeSet<&str> = BTreeSet::new();
        let mut seen: BTreeSet<(&str, &str)> = BTreeSet::new();
        let mut walked_ports: BTreeSet<&str> = BTreeSet::new();
        for row in &ledger.rows {
            let (axis, name) = (name_in(row.cell(0)), name_in(row.cell(1)));
            let known_axis = self.axes.contains_key(axis);
            if known_axis {
                attributed.insert(axis);
            } else {
                let axis = axis.to_owned();
                self.push(row.line, Breach::UnknownAxis { axis });
            }
            let Some(port) = known.iter().find(|p| p.name == name) else {
                let port = name.to_owned();
                self.push(row.line, Breach::UnknownPort { port });
                continue;
            };
            if !seen.insert((axis, name)) {
                let (axis, port) = (axis.to_owned(), name.to_owned());
                self.push(row.line, Breach::DuplicateRow { axis, port });
                continue;
            }
            walked_ports.insert(name);
            if port.cause.is_none() {
                self.walk.unwalked += 1;
            } else if known_axis {
                self.discharge(row.line, axis, port, row.cell(2))?;
            }
        }
        self.uncovered(&attributed, known, &walked_ports);
        Ok(())
    }

    /// What the ledger left uncovered: an axis no port sits on, and a frozen
    /// port with no row at all.
    fn uncovered(
        &mut self,
        attributed: &BTreeSet<&str>,
        known: &[Port],
        walked_ports: &BTreeSet<&str>,
    ) {
        for (axis, (line, _)) in self.axes {
            if !attributed.contains(axis.as_str()) {
                let axis = axis.clone();
                self.push(*line, Breach::UnattributedAxis { axis });
            }
        }
        for port in known {
            if let Some(cause) = &port.cause
                && !walked_ports.contains(port.name.as_str())
            {
                let (port_name, cause) = (port.name.clone(), cause.clone());
                self.push(
                    port.line,
                    Breach::UnwalkedFreeze {
                        port: port_name,
                        cause,
                    },
                );
            }
        }
    }

    /// Holds one frozen port's row on one axis to the route it names, and
    /// reports it as one [`Breach::Row`] when the route does not hold. The
    /// axis and port names are allocated only on that failing path.
    fn discharge(&mut self, line: usize, axis: &str, port: &Port, cell: &str) -> Result<()> {
        let problem = match discharge_in(cell) {
            None => Some(RowProblem::Unrecognised {
                cell: cell.trim().trim_matches('`').trim().to_owned(),
            }),
            Some(Discharge::Undischarged) => Some(RowProblem::Undischarged),
            Some(Discharge::FarEnd) => self.far_end(axis),
            Some(Discharge::Record(adr)) => self.record_row(axis, &port.name, adr)?,
            Some(Discharge::Carried(clause)) => self.carried(port, clause),
        };
        if let Some(problem) = problem {
            let (axis, port) = (axis.to_owned(), port.name.clone());
            self.push(
                line,
                Breach::Row {
                    axis,
                    port,
                    problem,
                },
            );
        }
        Ok(())
    }

    /// A `far end` row holds when the axis's verdict names an adapter (CF-26).
    fn far_end(&mut self, axis: &str) -> Option<RowProblem> {
        // `rows` discharges only rows whose axis the portfolio carries, so a
        // missing axis cannot reach here; it reads as unreadable if it did.
        let verdict = self.axes.get(axis).map_or(Verdict::Unreadable, |(_, v)| *v);
        if verdict == Verdict::Adapter {
            self.walk.far_end += 1;
            return None;
        }
        Some(RowProblem::NoAdapterAtFarEnd { verdict })
    }

    /// An `ADR-NNNN` row holds when the record is accepted and names both the
    /// axis and the port.
    fn record_row(&mut self, axis: &str, port: &str, adr: String) -> Result<Option<RowProblem>> {
        Ok(match (self.record)(&adr)? {
            None => Some(RowProblem::RecordMissing { adr }),
            Some(r) if !r.accepted => Some(RowProblem::RecordNotAccepted { adr }),
            Some(r) if !r.text.to_lowercase().contains(&axis.to_lowercase()) => {
                Some(RowProblem::RecordSilentOnAxis { adr })
            }
            Some(r) if !r.text.contains(port) => Some(RowProblem::RecordSilentOnPort { adr }),
            Some(_) => {
                self.walk.accepted += 1;
                None
            }
        })
    }

    /// A `carried by` row holds when the clause is an open clause of the
    /// row's own port.
    fn carried(&mut self, port: &Port, clause: String) -> Option<RowProblem> {
        let Some(c) = self.clauses.iter().find(|c| c.id == clause) else {
            return Some(RowProblem::CarrierMissing { clause });
        };
        let own = port
            .prefix
            .as_deref()
            .is_some_and(|prefix| clause.starts_with(prefix));
        if !own {
            return Some(RowProblem::CarrierOnAnotherPort { clause });
        }
        match c.maturity.as_deref() {
            Some("PROVISIONAL" | "DEFERRED") => {
                self.walk.carried += 1;
                None
            }
            other => Some(RowProblem::CarrierNotOpen {
                clause,
                maturity: other.unwrap_or("unmarked").to_owned(),
            }),
        }
    }
}

/// The portfolio's axes, each with its line and verdict. An unreadable verdict
/// and a repeated axis are reported here, once (CF-26).
fn axes_of(portfolio: &Table, found: &mut Vec<Finding>) -> BTreeMap<String, (usize, Verdict)> {
    let mut by_name = BTreeMap::new();
    for row in &portfolio.rows {
        let axis = name_in(row.cell(0));
        if by_name.contains_key(axis) {
            found.push(Finding {
                line: Some(row.line),
                breach: Breach::DuplicateAxis {
                    axis: axis.to_owned(),
                },
            });
            continue;
        }
        let read = verdict(row.cell(1));
        if read == Verdict::Unreadable {
            found.push(Finding {
                line: Some(row.line),
                breach: Breach::UnreadableVerdict {
                    axis: axis.to_owned(),
                    verdict: lead_or_cell(row.cell(1)).to_owned(),
                },
            });
        }
        by_name.insert(axis.to_owned(), (row.line, read));
    }
    by_name
}

/// A cell's bold lead, or the whole cell when it opens with none.
fn lead_or_cell(cell: &str) -> &str {
    let cell = cell.trim();
    bold_lead(cell).unwrap_or(cell)
}

/// Reads a `Far end exists?` cell by the bold words that open it.
///
/// Only a lead that says *adapter yes* names an adapter (CF-26): a bare
/// "yes" could be a fixture, and the column exists to tell the two apart.
/// Anything outside the vocabulary is [`Verdict::Unreadable`], which
/// discharges nothing.
fn verdict(cell: &str) -> Verdict {
    let Some(lead) = bold_lead(cell.trim()) else {
        return Verdict::Unreadable;
    };
    let lead = lead.to_lowercase();
    if lead.contains("adapter yes") {
        Verdict::Adapter
    } else if lead.contains("fixture yes, adapter no") {
        Verdict::FixtureOnly
    } else if lead.starts_with("no") {
        Verdict::Nothing
    } else {
        Verdict::Unreadable
    }
}

/// Reads a §1.6 `Maturity` cell by its bold lead, against a closed vocabulary.
fn maturity(cell: &str) -> Maturity {
    let Some(lead) = bold_lead(cell.trim()) else {
        return Maturity::Unreadable;
    };
    let lead = lead.to_lowercase();
    if lead.starts_with("frozen") {
        Maturity::Frozen
    } else if ["shape specified", "provisional", "deferred"]
        .iter()
        .any(|form| lead.starts_with(form))
    {
        Maturity::NotFrozen
    } else {
        Maturity::Unreadable
    }
}

/// A `Discharged by` cell in one of its four forms.
fn discharge_in(cell: &str) -> Option<Discharge> {
    let text = cell.trim().trim_matches('`').trim();
    if text == "far end" {
        return Some(Discharge::FarEnd);
    }
    if text == UNDISCHARGED {
        return Some(Discharge::Undischarged);
    }
    if let Some(number) = text.strip_prefix("ADR-")
        && number.len() == 4
        && number.bytes().all(|b| b.is_ascii_digit())
    {
        return Some(Discharge::Record(number.to_owned()));
    }
    let clause = text.strip_prefix("carried by ")?.trim().trim_matches('`');
    let (prefix, digits) = clause.split_once('-')?;
    let well_formed = !prefix.is_empty()
        && prefix.bytes().all(|b| b.is_ascii_uppercase())
        && !digits.is_empty()
        && digits.bytes().all(|b| b.is_ascii_digit());
    well_formed.then(|| Discharge::Carried(clause.to_owned()))
}

/// The `status:` value in a decision atom's frontmatter, and only there: a
/// body line reading `status: accepted` is prose about some other atom.
fn status_of(atom: &str) -> Option<&str> {
    atom.strip_prefix("---")?
        .lines()
        .skip(1)
        .take_while(|l| l.trim() != "---")
        .find_map(|l| l.strip_prefix("status:"))
        .map(str::trim)
}

/// Looks records up under `root`: the atom at `.kb/decisions/NNNN-*.md` and,
/// when there is one, its long form at `references/adr/NNNN-*.md`.
pub(super) fn records_under(root: &Path) -> impl Fn(&str) -> Result<Option<Record>> + '_ {
    move |number| {
        let Some(atom) = numbered(&root.join(".kb/decisions"), number)? else {
            return Ok(None);
        };
        let mut text = std::fs::read_to_string(&atom)
            .with_context(|| format!("reading {}", atom.display()))?;
        let accepted = status_of(&text) == Some("accepted");
        if let Some(long) = numbered(&root.join("references/adr"), number)? {
            text.push_str(
                &std::fs::read_to_string(&long)
                    .with_context(|| format!("reading {}", long.display()))?,
            );
        }
        Ok(Some(Record { accepted, text }))
    }
}

/// The one `NNNN-*.md` file in `dir`, if there is one; two is an error, so
/// `read_dir`'s unspecified order never picks the record. A missing directory
/// holds none: a tree with no long-form records has no long form to add.
fn numbered(dir: &Path, number: &str) -> Result<Option<PathBuf>> {
    if !dir.is_dir() {
        return Ok(None);
    }
    let stem = format!("{number}-");
    let mut found: Option<PathBuf> = None;
    for entry in std::fs::read_dir(dir).with_context(|| format!("listing {}", dir.display()))? {
        let path = entry
            .with_context(|| format!("listing {}", dir.display()))?
            .path();
        let matches = path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with(&stem))
            && path
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("md"));
        if matches {
            if let Some(first) = &found {
                anyhow::bail!(
                    "two records numbered {number} in {}: {} and {}",
                    dir.display(),
                    first.display(),
                    path.display()
                );
            }
            found = Some(path);
        }
    }
    Ok(found)
}

/// The walk's one summary line, printed on every run, green or not.
///
/// # Errors
///
/// When `out` cannot be written.
pub(super) fn summarise(out: &mut impl io::Write, walk: &Walk) -> Result<()> {
    writeln!(
        out,
        "CF-25 portfolio: {} axes; {} frozen port(s) ({}), {} ledger row(s) discharged: {} \
         adapter at the far end, {} accepted by a record, {} carried by an open clause; {} \
         row(s) of ports with nothing frozen not walked. Not read: the near end, an axis a port \
         sits on with no row, and what a record says beyond naming the axis and the port",
        walk.axes,
        walk.frozen.len(),
        walk.frozen.join(", "),
        walk.far_end + walk.accepted + walk.carried,
        walk.far_end,
        walk.accepted,
        walk.carried,
        walk.unwalked,
    )
    .context("writing the CF-25 portfolio summary")
}

#[cfg(test)]
mod tests;
