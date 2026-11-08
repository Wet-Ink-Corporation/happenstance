//! CF-25 and CF-26: a port freeze against §6.5's instrument portfolio (#125).
//!
//! CF-25 forbids declaring a port `[FROZEN]` while an axis it sits on has no
//! passing implementation at its far end, unless the freeze names the axis and
//! the record accepting the risk. CF-26 says a fixture at the far end does not
//! count. Both clauses name this checker, and until #125 nothing in it read the
//! portfolio at all.
//!
//! # The reading, and why it is the narrow one
//!
//! Three tables carry the facts, and each is read for one thing:
//!
//! - **§1.6, the three ports.** A port is frozen when its `Maturity` cell opens
//!   `**Frozen`. Not when one of its clauses is: §1.3 says a port whose gating
//!   clause is unsatisfied is not frozen no matter how many of its clauses are.
//! - **§6.5's portfolio.** Each axis's name, and the bold verdict that opens its
//!   `Far end exists?` cell. Nothing else in a row is read.
//! - **§6.5's freeze ledger.** One row per axis a port sits on, saying how that
//!   port's freeze discharges it.
//!
//! §1.4's prefix table maps a port to its clause prefix, so that a `carried by`
//! row can be held to a clause of the right port.
//!
//! # What it does not read
//!
//! The near end, whether a port really sits on no axis it has no ledger row
//! for, and what an accepting record says beyond naming the axis. [`summarise`]
//! prints that on every green run, so a pass is not read as more than it is.

use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use super::Clause;
use breach::{Breach, Finding, Verdict};
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
}

/// What a run walked, for the summary line.
#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Walk {
    /// Axes in the portfolio table.
    pub(super) axes: usize,
    /// Ports §1.6 declares frozen, in table order.
    pub(super) frozen: Vec<String>,
    /// Frozen ports' rows discharged by an adapter at the far end.
    pub(super) far_end: usize,
    /// Frozen ports' rows discharged by an accepting record.
    pub(super) accepted: usize,
    /// Frozen ports' rows carried by an open clause.
    pub(super) carried: usize,
    /// Rows of ports not declared frozen, which owe nothing yet.
    pub(super) unwalked: usize,
}

/// Checks every port §1.6 declares frozen against §6.5's portfolio.
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

    let declared = declared_ports(&ports);
    let prefix_of = port_prefixes(&prefixes);
    let portfolio_axes = axes_of(&portfolio, &mut found);
    let mut walker = Walker {
        clauses,
        record: &record,
        prefix_of: &prefix_of,
        axes: &portfolio_axes,
        walk: Walk {
            axes: portfolio_axes.len(),
            frozen: declared
                .iter()
                .filter(|p| p.frozen)
                .map(|p| p.name.clone())
                .collect(),
            ..Walk::default()
        },
        found,
    };

    walker.rows(&ledger, &declared)?;
    Ok((walker.walk, walker.found))
}

/// The state one [`check`] threads through its ledger rows.
struct Walker<'a, F> {
    clauses: &'a [Clause],
    record: &'a F,
    prefix_of: &'a BTreeMap<String, String>,
    /// Axis name to its portfolio line and verdict.
    axes: &'a BTreeMap<String, (usize, Verdict)>,
    walk: Walk,
    found: Vec<Finding>,
}

impl<F> Walker<'_, F>
where
    F: Fn(&str) -> Result<Option<Record>>,
{
    fn push(&mut self, line: Option<usize>, breach: Breach) {
        self.found.push(Finding { line, breach });
    }

    /// Walks every ledger row, then reports what the rows left uncovered.
    fn rows(&mut self, ledger: &Table, declared: &[DeclaredPort]) -> Result<()> {
        let mut attributed: BTreeSet<&str> = BTreeSet::new();
        let mut seen: BTreeSet<(&str, &str)> = BTreeSet::new();
        let mut walked_ports: BTreeSet<&str> = BTreeSet::new();
        for row in &ledger.rows {
            let (axis, port) = (name_in(row.cell(0)), name_in(row.cell(1)));
            let at = Some(row.line);
            let known_axis = self.axes.contains_key(axis);
            if known_axis {
                attributed.insert(axis);
            } else {
                let axis = axis.to_owned();
                self.push(at, Breach::UnknownAxis { axis });
            }
            let Some(declared_port) = declared.iter().find(|p| p.name == port) else {
                let port = port.to_owned();
                self.push(at, Breach::UnknownPort { port });
                continue;
            };
            if !seen.insert((axis, port)) {
                let (axis, port) = (axis.to_owned(), port.to_owned());
                self.push(at, Breach::DuplicateRow { axis, port });
                continue;
            }
            walked_ports.insert(port);
            if !declared_port.frozen {
                self.walk.unwalked += 1;
            } else if known_axis {
                self.discharge(row.line, axis, port, row.cell(2))?;
            }
        }
        self.uncovered(&attributed, declared, &walked_ports);
        Ok(())
    }

    /// What the ledger left uncovered: an axis no port sits on, and a frozen
    /// port with no row at all.
    fn uncovered(
        &mut self,
        attributed: &BTreeSet<&str>,
        declared: &[DeclaredPort],
        walked_ports: &BTreeSet<&str>,
    ) {
        for (axis, (line, _)) in self.axes {
            if !attributed.contains(axis.as_str()) {
                self.push(Some(*line), Breach::UnattributedAxis { axis: axis.clone() });
            }
        }
        for port in declared.iter().filter(|p| p.frozen) {
            if !walked_ports.contains(port.name.as_str()) {
                self.push(
                    Some(port.line),
                    Breach::UnwalkedFreeze {
                        port: port.name.clone(),
                    },
                );
            }
        }
    }

    /// Holds one frozen port's row on one axis to the route it names.
    fn discharge(&mut self, line: usize, axis: &str, port: &str, cell: &str) -> Result<()> {
        let (axis_s, port_s) = (axis.to_owned(), port.to_owned());
        let at = Some(line);
        match discharge_in(cell) {
            None => self.push(
                at,
                Breach::UnrecognisedDischarge {
                    axis: axis_s,
                    port: port_s,
                    cell: cell.trim().trim_matches('`').trim().to_owned(),
                },
            ),
            Some(Discharge::FarEnd) => match self.axes.get(axis) {
                Some((_, Verdict::Adapter)) => self.walk.far_end += 1,
                Some((_, verdict)) => {
                    let verdict = *verdict;
                    self.push(
                        at,
                        Breach::NoAdapterAtFarEnd {
                            axis: axis_s,
                            port: port_s,
                            verdict,
                        },
                    );
                }
                // `check` walks only rows whose axis the portfolio carries.
                None => {}
            },
            Some(Discharge::Record(adr)) => match (self.record)(&adr)? {
                None => self.push(
                    at,
                    Breach::RecordMissing {
                        axis: axis_s,
                        port: port_s,
                        adr,
                    },
                ),
                Some(r) if !r.accepted => self.push(
                    at,
                    Breach::RecordNotAccepted {
                        axis: axis_s,
                        port: port_s,
                        adr,
                    },
                ),
                Some(r) if !r.text.to_lowercase().contains(&axis.to_lowercase()) => self.push(
                    at,
                    Breach::RecordSilentOnAxis {
                        axis: axis_s,
                        port: port_s,
                        adr,
                    },
                ),
                Some(_) => self.walk.accepted += 1,
            },
            Some(Discharge::Carried(clause)) => self.carried(line, axis_s, port_s, clause),
        }
        Ok(())
    }

    /// Holds a `carried by` row to an open clause of the row's own port.
    fn carried(&mut self, line: usize, axis: String, port: String, clause: String) {
        let at = Some(line);
        let Some(c) = self.clauses.iter().find(|c| c.id == clause) else {
            self.push(at, Breach::CarrierMissing { axis, port, clause });
            return;
        };
        let own = self
            .prefix_of
            .get(&port)
            .is_some_and(|prefix| clause.starts_with(prefix.as_str()));
        if !own {
            self.push(at, Breach::CarrierOnAnotherPort { axis, port, clause });
            return;
        }
        match c.maturity.as_deref() {
            Some("PROVISIONAL" | "DEFERRED") => self.walk.carried += 1,
            other => self.push(
                at,
                Breach::CarrierNotOpen {
                    axis,
                    port,
                    clause,
                    maturity: other.unwrap_or("unmarked").to_owned(),
                },
            ),
        }
    }
}

/// A port §1.6 lists, and whether it declares the port frozen.
struct DeclaredPort {
    name: String,
    line: usize,
    frozen: bool,
}

/// §1.6's rows. A port is frozen when its `Maturity` cell opens `**Frozen`.
fn declared_ports(ports: &Table) -> Vec<DeclaredPort> {
    ports
        .rows
        .iter()
        .map(|row| DeclaredPort {
            name: name_in(row.cell(0)).to_owned(),
            line: row.line,
            frozen: row.cell(1).trim_start().starts_with("**Frozen"),
        })
        .collect()
}

/// §1.4's rows that name a port: `` `ES-n` `` and "the `EventStore` port"
/// become `EventStore` → `ES-`.
fn port_prefixes(prefixes: &Table) -> BTreeMap<String, String> {
    prefixes
        .rows
        .iter()
        .filter_map(|row| {
            let prefix = row.cell(0).trim().trim_matches('`').strip_suffix('n')?;
            let port = row
                .cell(1)
                .trim()
                .strip_prefix("the `")?
                .strip_suffix("` port")?;
            Some((port.to_owned(), prefix.to_owned()))
        })
        .collect()
}

/// The portfolio's axes, each with its line and verdict. An unreadable
/// verdict is reported here, once, whichever ports sit on the axis (CF-26).
fn axes_of(portfolio: &Table, found: &mut Vec<Finding>) -> BTreeMap<String, (usize, Verdict)> {
    let mut by_name = BTreeMap::new();
    for row in &portfolio.rows {
        let axis = name_in(row.cell(0));
        let read = verdict(row.cell(1));
        if read == Verdict::Unreadable {
            found.push(Finding {
                line: Some(row.line),
                breach: Breach::UnreadableVerdict {
                    axis: axis.to_owned(),
                    verdict: bold_lead(row.cell(1).trim())
                        .unwrap_or(row.cell(1).trim())
                        .to_owned(),
                },
            });
        }
        by_name.insert(axis.to_owned(), (row.line, read));
    }
    by_name
}

/// Reads a `Far end exists?` cell by the bold words that open it.
///
/// Positive forms are tested first, so "fixture yes, adapter yes" is an
/// adapter however the rest of the sentence hedges. Anything outside the
/// vocabulary is [`Verdict::Unreadable`], which discharges nothing: a verdict
/// softened into words this cannot classify must not pass as an adapter.
fn verdict(cell: &str) -> Verdict {
    let Some(lead) = bold_lead(cell.trim()) else {
        return Verdict::Unreadable;
    };
    let lead = lead.to_lowercase();
    if lead.starts_with("yes") || lead.starts_with("far end yes") || lead.contains("adapter yes") {
        Verdict::Adapter
    } else if lead.contains("fixture yes, adapter no") {
        Verdict::FixtureOnly
    } else if lead.starts_with("no") || lead.starts_with("far end no") {
        Verdict::Nothing
    } else {
        Verdict::Unreadable
    }
}

/// A `Discharged by` cell in one of its three forms.
fn discharge_in(cell: &str) -> Option<Discharge> {
    let text = cell.trim().trim_matches('`').trim();
    if text == "far end" {
        return Some(Discharge::FarEnd);
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

/// The one `NNNN-*.md` file in `dir`, if there is one.
fn numbered(dir: &Path, number: &str) -> Result<Option<PathBuf>> {
    let stem = format!("{number}-");
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
            return Ok(Some(path));
        }
    }
    Ok(None)
}

/// The walk's one summary line, printed on every run, green or not.
///
/// # Errors
///
/// When `out` cannot be written.
pub(super) fn summarise(out: &mut impl io::Write, walk: &Walk) -> Result<()> {
    writeln!(
        out,
        "CF-25 portfolio: {} axes; {} port(s) declared frozen ({}), {} ledger row(s) walked: \
         {} adapter at the far end, {} accepted by a record, {} carried by an open clause; {} \
         row(s) of undeclared ports not walked. Not read: the near end, an axis a port sits on \
         with no row, and what a record says beyond naming the axis",
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
