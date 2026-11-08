//! What the portfolio check reports, and the words it reports it in.
//!
//! Typed, so the tests assert which breach a wrong input is rather than
//! comparing sentences; rendered once, by [`Finding`]'s `Display`, into the
//! `problems` lines `spec-trace` prints. Each message names the clause, the
//! port and the axis, and what would discharge it.

use std::fmt;

use super::super::SPEC;

/// What a portfolio row's `Far end exists?` verdict says sits at the far end.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::spec_trace) enum Verdict {
    /// A real adapter: implementability, CF-25's half a fixture cannot buy.
    Adapter,
    /// A fixture and no adapter: falsifiability only (CF-26).
    FixtureOnly,
    /// Nothing at either end.
    Nothing,
    /// Bold words this checker has no reading for.
    Unreadable,
}

impl Verdict {
    /// The verdict in the words of an error message.
    fn describe(self) -> &'static str {
        match self {
            Self::Adapter => "an adapter",
            Self::FixtureOnly => "a fixture and no adapter",
            Self::Nothing => "nothing at the far end",
            Self::Unreadable => "nothing this checker can read",
        }
    }
}

/// Why a port is walked: what froze it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::spec_trace) enum Cause {
    /// At least one `[FROZEN]` clause under the port's prefix: CF-25's trigger.
    Clauses { first: String, count: usize },
    /// No frozen clause, but §1.6 declares the port **Frozen**.
    Declared,
}

impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clauses { first, count } => {
                write!(f, "carries {count} [FROZEN] clause(s), the first {first},")
            }
            Self::Declared => write!(f, "is declared frozen in §1.6"),
        }
    }
}

/// One way a port freeze fails CF-25 or CF-26.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::spec_trace) enum Breach {
    /// A table the check reads is not where it is looked for.
    MissingTable { what: &'static str },
    /// A portfolio verdict outside the vocabulary [`super::verdict`] reads.
    UnreadableVerdict { axis: String, verdict: String },
    /// A portfolio axis named by two rows; the second would silently win.
    DuplicateAxis { axis: String },
    /// A §1.6 `Maturity` lead outside the vocabulary [`super::maturity`] reads.
    UnreadableMaturity { port: String, maturity: String },
    /// A portfolio axis with no ledger row for any port.
    UnattributedAxis { axis: String },
    /// A ledger row naming an axis the portfolio does not carry.
    UnknownAxis { axis: String },
    /// A ledger row naming a port neither §1.4 nor §1.6 lists.
    UnknownPort { port: String },
    /// Two ledger rows for one port and one axis.
    DuplicateRow { axis: String, port: String },
    /// A frozen port with no ledger row at all.
    UnwalkedFreeze { port: String, cause: Cause },
    /// A frozen port's row on one axis that its discharge does not discharge.
    Row {
        axis: String,
        port: String,
        problem: RowProblem,
    },
}

/// Why one frozen port's ledger row does not discharge its axis.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::spec_trace) enum RowProblem {
    /// The row says, out loud, nothing discharges it.
    Undischarged,
    /// The `Discharged by` cell is none of the forms.
    Unrecognised { cell: String },
    /// `far end`, over a verdict that names no adapter.
    NoAdapterAtFarEnd { verdict: Verdict },
    /// An `ADR-NNNN` with no decision atom of that number.
    RecordMissing { adr: String },
    /// An `ADR-NNNN` whose atom is no longer `status: accepted`.
    RecordNotAccepted { adr: String },
    /// An `ADR-NNNN` that never names the axis it is cited as accepting.
    RecordSilentOnAxis { adr: String },
    /// An `ADR-NNNN` that names the axis but never the port, so it accepts the
    /// risk for some other port's freeze (CF-25's *pro forma* sentence).
    RecordSilentOnPort { adr: String },
    /// `carried by` a clause the document does not declare.
    CarrierMissing { clause: String },
    /// `carried by` a clause of some other port.
    CarrierOnAnotherPort { clause: String },
    /// `carried by` a clause that is no longer open.
    CarrierNotOpen { clause: String, maturity: String },
}

/// A [`Breach`] and the specification line it is reported against.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::spec_trace) struct Finding {
    /// One-based; `None` when the breach is a table that is not there.
    pub(in crate::spec_trace) line: Option<usize>,
    pub(in crate::spec_trace) breach: Breach,
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.line {
            Some(line) => write!(f, "{SPEC}:{line} — {}", self.breach),
            None => write!(f, "{SPEC} — {}", self.breach),
        }
    }
}

/// The remedy every undischarged-row message ends with.
const REMEDY: &str = "Put an adapter at the far end, cite an accepted ADR naming the axis and \
                      the port, or carry it by an open clause of the port";

impl fmt::Display for Breach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingTable { what } => write!(
                f,
                "CF-25: no {what}, so no port freeze can be checked against §6.5's portfolio. \
                 Restore it; a check that finds nothing to read must not pass"
            ),
            Self::UnreadableVerdict { axis, verdict } => write!(
                f,
                "CF-26: the portfolio's `Far end exists?` verdict for {axis} opens with \
                 `{verdict}`, which this check cannot classify. Open the cell with words that \
                 say **adapter yes**, **Fixture yes, adapter no**, or **No**"
            ),
            Self::DuplicateAxis { axis } => write!(
                f,
                "CF-25: the portfolio carries the axis {axis} twice. Merge the rows; the check \
                 will not guess which verdict holds"
            ),
            Self::UnreadableMaturity { port, maturity } => write!(
                f,
                "CF-25: §1.6's `Maturity` cell for `{port}` opens with `{maturity}`, which this \
                 check cannot classify. Open it with **Frozen**, **Provisional**, **Deferred** \
                 or **Shape specified**"
            ),
            Self::UnattributedAxis { axis } => write!(
                f,
                "CF-25: the portfolio axis {axis} has no row in §6.5's freeze ledger. Add one \
                 row per port that sits on it, saying how that port's freeze discharges it"
            ),
            Self::UnknownAxis { axis } => write!(
                f,
                "CF-25: the freeze ledger names the axis {axis}, which §6.5's portfolio does \
                 not carry. Use the portfolio's bold axis name"
            ),
            Self::UnknownPort { port } => write!(
                f,
                "CF-25: the freeze ledger names the port `{port}`, which neither §1.4 nor §1.6 \
                 lists"
            ),
            Self::DuplicateRow { axis, port } => write!(
                f,
                "CF-25: the freeze ledger has two rows for `{port}` on {axis}. Keep one: a \
                 port discharges an axis one way"
            ),
            Self::UnwalkedFreeze { port, cause } => write!(
                f,
                "CF-25: `{port}` {cause} and §6.5's freeze ledger has no row for it. Add a row \
                 for every portfolio axis it sits on, each discharged by `far end`, an \
                 accepting `ADR-NNNN`, or `carried by` an open clause of the port"
            ),
            Self::Row {
                axis,
                port,
                problem,
            } => {
                write!(
                    f,
                    "CF-25: `{port}` is frozen and its ledger row for {axis} "
                )?;
                problem.describe(f, axis, port)
            }
        }
    }
}

impl RowProblem {
    /// The rest of a [`Breach::Row`] message, after "its ledger row for X".
    fn describe(&self, f: &mut fmt::Formatter<'_>, axis: &str, port: &str) -> fmt::Result {
        match self {
            Self::Undischarged => write!(
                f,
                "is undischarged: no adapter at the far end, no accepting record, no open \
                 clause. {REMEDY}"
            ),
            Self::Unrecognised { cell } => write!(
                f,
                "is discharged by `{cell}`, which is none of `far end`, `ADR-NNNN`, `carried \
                 by <clause>` or `undischarged`"
            ),
            Self::NoAdapterAtFarEnd { verdict } => write!(
                f,
                "says `far end`, but the portfolio's verdict for that axis reads {}. A fixture \
                 proves a rule can fail and not that a real implementation can pass (CF-26). \
                 {REMEDY}",
                verdict.describe()
            ),
            Self::RecordMissing { adr } => {
                write!(f, "cites ADR-{adr}, and no .kb/decisions/{adr}-*.md exists")
            }
            Self::RecordNotAccepted { adr } => write!(
                f,
                "cites ADR-{adr}, which is no longer `status: accepted`. Cite the record that \
                 superseded it, if it still accepts the risk"
            ),
            Self::RecordSilentOnAxis { adr } => write!(
                f,
                "cites ADR-{adr}, which never names {axis}. CF-25's second route needs the \
                 record to name the axis it accepts"
            ),
            Self::RecordSilentOnPort { adr } => write!(
                f,
                "cites ADR-{adr}, which never names `{port}`. An acceptance for another port \
                 is pro forma for this one (CF-25)"
            ),
            Self::CarrierMissing { clause } => write!(
                f,
                "is carried by {clause}, which the specification does not declare"
            ),
            Self::CarrierOnAnotherPort { clause } => write!(
                f,
                "is carried by {clause}, which is not a `{port}` clause. Only the port's own \
                 open clause can carry its exposure"
            ),
            Self::CarrierNotOpen { clause, maturity } => write!(
                f,
                "is carried by {clause}, which is now [{maturity}]. Freezing it removed the \
                 disclosure. {REMEDY}"
            ),
        }
    }
}
