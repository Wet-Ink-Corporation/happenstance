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

/// One way a port freeze fails CF-25 or CF-26.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::spec_trace) enum Breach {
    /// A table the check reads is not where it is looked for.
    MissingTable { what: &'static str },
    /// A portfolio verdict outside the vocabulary [`super::verdict`] reads.
    UnreadableVerdict { axis: String, verdict: String },
    /// A portfolio axis with no ledger row for any port.
    UnattributedAxis { axis: String },
    /// A ledger row naming an axis the portfolio does not carry.
    UnknownAxis { axis: String },
    /// A ledger row naming a port §1.6 does not list.
    UnknownPort { port: String },
    /// Two ledger rows for one port and one axis.
    DuplicateRow { axis: String, port: String },
    /// A port §1.6 declares frozen with no ledger row at all.
    UnwalkedFreeze { port: String },
    /// A frozen port's row whose `Discharged by` cell is none of the three forms.
    UnrecognisedDischarge {
        axis: String,
        port: String,
        cell: String,
    },
    /// `far end`, over a verdict that names no adapter.
    NoAdapterAtFarEnd {
        axis: String,
        port: String,
        verdict: Verdict,
    },
    /// An `ADR-NNNN` with no decision atom of that number.
    RecordMissing {
        axis: String,
        port: String,
        adr: String,
    },
    /// An `ADR-NNNN` whose atom is no longer `status: accepted`.
    RecordNotAccepted {
        axis: String,
        port: String,
        adr: String,
    },
    /// An `ADR-NNNN` that never names the axis it is cited as accepting.
    RecordSilentOnAxis {
        axis: String,
        port: String,
        adr: String,
    },
    /// `carried by` a clause the document does not declare.
    CarrierMissing {
        axis: String,
        port: String,
        clause: String,
    },
    /// `carried by` a clause of some other port.
    CarrierOnAnotherPort {
        axis: String,
        port: String,
        clause: String,
    },
    /// `carried by` a clause that is no longer open.
    CarrierNotOpen {
        axis: String,
        port: String,
        clause: String,
        maturity: String,
    },
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
                 `{verdict}`, which names neither an adapter, a fixture only, nor nothing. Open \
                 the cell with **Yes**, **Fixture yes, adapter yes/no** or **No**"
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
                "CF-25: the freeze ledger names the port `{port}`, which §1.6 does not list"
            ),
            Self::DuplicateRow { axis, port } => write!(
                f,
                "CF-25: the freeze ledger has two rows for `{port}` on {axis}. Keep one: a \
                 port discharges an axis one way"
            ),
            Self::UnwalkedFreeze { port } => write!(
                f,
                "CF-25: §1.6 declares `{port}` frozen and §6.5's freeze ledger has no row for \
                 it. Add a row for every portfolio axis it sits on, each discharged by `far \
                 end`, an accepting `ADR-NNNN`, or `carried by` an open clause of the port"
            ),
            Self::UnrecognisedDischarge { axis, port, cell } => write!(
                f,
                "CF-25: `{port}` on {axis} is discharged by `{cell}`, which is none of `far \
                 end`, `ADR-NNNN` or `carried by <clause>`"
            ),
            Self::NoAdapterAtFarEnd {
                axis,
                port,
                verdict,
            } => write!(
                f,
                "CF-25: `{port}` is frozen and its ledger row for {axis} says `far end`, but \
                 the portfolio's verdict for that axis reads {}. A fixture proves a rule can \
                 fail and not that a real implementation can pass (CF-26). Build an adapter \
                 at the far end, or cite the ADR accepting the risk on {axis}",
                verdict.describe()
            ),
            Self::RecordMissing { axis, port, adr } => write!(
                f,
                "CF-25: `{port}`'s ledger row for {axis} cites ADR-{adr}, and no \
                 .kb/decisions/{adr}-*.md exists"
            ),
            Self::RecordNotAccepted { axis, port, adr } => write!(
                f,
                "CF-25: `{port}`'s ledger row for {axis} cites ADR-{adr}, which is no longer \
                 `status: accepted`. Cite the record that superseded it, if it still accepts \
                 the risk"
            ),
            Self::RecordSilentOnAxis { axis, port, adr } => write!(
                f,
                "CF-25: `{port}`'s ledger row for {axis} cites ADR-{adr}, which never names \
                 {axis}. CF-25's second route needs the record to name the axis it accepts"
            ),
            Self::CarrierMissing { axis, port, clause } => write!(
                f,
                "CF-25: `{port}`'s ledger row for {axis} is carried by {clause}, which the \
                 specification does not declare"
            ),
            Self::CarrierOnAnotherPort { axis, port, clause } => write!(
                f,
                "CF-25: `{port}`'s ledger row for {axis} is carried by {clause}, which is not \
                 a `{port}` clause. Only the port's own open clause can carry its exposure"
            ),
            Self::CarrierNotOpen {
                axis,
                port,
                clause,
                maturity,
            } => write!(
                f,
                "CF-25: `{port}`'s ledger row for {axis} is carried by {clause}, which is now \
                 [{maturity}]. Freezing it removed the disclosure; put an adapter at the far \
                 end of {axis}, or cite the ADR accepting the risk"
            ),
        }
    }
}
