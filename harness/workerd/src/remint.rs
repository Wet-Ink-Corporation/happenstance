//! `CloudflareEventStore::remint_identity` across a real bookmark restore
//! (#298).
//!
//! The adapter's own tests prove the re-mint against a simulated restore, an
//! in-memory copy of the whole database made in the host shim. This module is
//! the half of the real one that does not need the platform: what the runner
//! carries across the restore, and the verdict the object gives after it.
//!
//! # The leg
//!
//! On one object, under its own [`TableNamespace`](happenstance_cloudflare::TableNamespace):
//!
//! 1. append [`KEPT`], then take a bookmark (`getCurrentBookmark`);
//! 2. append [`LOST`], and hand the runner a [`Ledger`] of all four events;
//! 3. `onNextSessionRestoreBookmark(bookmark)`, then `ctx.abort()`;
//! 4. on the next instance: check that the restore is visible, re-mint (or,
//!    for the [`Mode::Control`] cell, do not), append [`REISSUED`], and
//!    [`judge`] what the object now yields against the ledger.
//!
//! The bookmark calls are made by `worker.mjs`, because the `worker` crate
//! binds none of them. Everything else is the adapter as a consumer builds it.
//!
//! # Where it runs
//!
//! On a deployed object only. Local `workerd`, at the version
//! `package-lock.json` pins, answers `getCurrentBookmark` with an all-zero
//! bookmark and refuses `onNextSessionRestoreBookmark` with "does not
//! implement point-in-time recovery", and `test/remint.test.ts` pins that
//! refusal so a runtime bump that lifts it is noticed.

use core::fmt;
use core::str::FromStr;

use crate::incarnation::{Incarnation, InvalidIncarnation};

/// Written before the bookmark, so the restore keeps them.
pub const KEPT: [&str; 2] = ["A1", "A2"];

/// Written after the bookmark, so the restore rolls them back.
pub const LOST: [&str; 2] = ["B1", "B2"];

/// Written after the restore: more than were lost, so positions both inside
/// and beyond the rolled-back range are handed out.
pub const REISSUED: [&str; 3] = ["C1", "C2", "C3"];

/// Which cell runs: the re-mint, or the control that skips it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `remint_identity` runs between the restore and the next append.
    Remint,
    /// Nothing runs between them. The hazard the re-mint exists for should
    /// show here, which is what makes the re-mint cell's clean result mean
    /// something.
    Control,
}

impl Mode {
    /// The name the runner sends as `mode=`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Remint => "remint",
            Self::Control => "control",
        }
    }
}

impl FromStr for Mode {
    type Err = UnknownMode;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "remint" => Ok(Self::Remint),
            "control" => Ok(Self::Control),
            other => Err(UnknownMode(other.to_owned())),
        }
    }
}

/// A `mode=` that is neither `remint` nor `control`, handed back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownMode(pub String);

impl fmt::Display for UnknownMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no mode {:?} (remint or control)", self.0)
    }
}

impl core::error::Error for UnknownMode {}

/// One event as the store yielded it: its type, and its `EventId` as text.
///
/// The text is `EventId`'s `Display`, `store:position`. Identities are only
/// compared for equality, and the position is read back off the end of the
/// text, so nothing parses an `EventId` back into one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sighting {
    /// The event's type.
    pub event_type: String,
    /// `EventId`'s text.
    pub id: String,
}

impl Sighting {
    /// The position part of the identity.
    fn position(&self) -> Option<&str> {
        self.id.rsplit_once(':').map(|(_, position)| position)
    }
}

/// What the runner carries across the restore: the instance that wrote the
/// events, and every event it read back before the restore.
///
/// It travels through the runner, never through the storage under test, so
/// the restore cannot roll back the witness it is judged against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ledger {
    /// The instance the events were written in.
    pub incarnation: Incarnation,
    /// Every event, in the order the store yielded them.
    pub events: Vec<Sighting>,
}

impl fmt::Display for Ledger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "incarnation={}", self.incarnation)?;
        for event in &self.events {
            writeln!(f, "event={} {}", event.event_type, event.id)?;
        }
        Ok(())
    }
}

/// Why a ledger's text did not parse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidLedger {
    /// No `incarnation=` line, or more than one.
    Incarnation,
    /// An `incarnation=` line whose value is not one.
    BadIncarnation(InvalidIncarnation),
    /// A line that is neither form, handed back.
    Line(String),
}

impl fmt::Display for InvalidLedger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Incarnation => f.write_str("the ledger needs exactly one incarnation= line"),
            Self::BadIncarnation(err) => write!(f, "the ledger's incarnation: {err}"),
            Self::Line(line) => write!(f, "not a ledger line: {line:?}"),
        }
    }
}

impl core::error::Error for InvalidLedger {}

impl FromStr for Ledger {
    type Err = InvalidLedger;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut incarnation = None;
        let mut events = Vec::new();
        for line in text.lines().filter(|line| !line.is_empty()) {
            if let Some(value) = line.strip_prefix("incarnation=") {
                let parsed = value.parse().map_err(InvalidLedger::BadIncarnation)?;
                if incarnation.replace(parsed).is_some() {
                    return Err(InvalidLedger::Incarnation);
                }
            } else if let Some((event_type, id)) = line
                .strip_prefix("event=")
                .and_then(|rest| rest.split_once(' '))
            {
                events.push(Sighting {
                    event_type: event_type.to_owned(),
                    id: id.to_owned(),
                });
            } else {
                return Err(InvalidLedger::Line(line.to_owned()));
            }
        }
        Ok(Self {
            incarnation: incarnation.ok_or(InvalidLedger::Incarnation)?,
            events,
        })
    }
}

/// The events whose type is one of `types`, in order.
fn of_types<'a>(events: &'a [Sighting], types: &[&str]) -> Vec<&'a Sighting> {
    events
        .iter()
        .filter(|event| types.contains(&event.event_type.as_str()))
        .collect()
}

/// Whether the object, read after the restart, shows the restore: exactly the
/// [`KEPT`] events, under the identities they had, and nothing else.
///
/// Checked before anything is appended, so a restore that did not happen
/// (the bookmark ignored, or the rows kept) is refused rather than judged.
#[must_use]
pub fn restore_observed(before: &Ledger, now: &[Sighting]) -> bool {
    let kept = of_types(&before.events, &KEPT);
    kept.len() == KEPT.len() && now.iter().eq(kept)
}

/// One identity that named one event before the restore and another after.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reissue {
    /// The identity, as text.
    pub id: String,
    /// The event it named in the ledger.
    pub was: String,
    /// The event it names now.
    pub now: String,
}

/// What the object yielded after the restore, judged against the ledger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    /// The cell that ran.
    pub mode: Mode,
    /// Every identity issued again for a different event.
    pub reissued: Vec<Reissue>,
    /// How many [`REISSUED`] events landed at a position a [`LOST`] event
    /// had. The instrument: with none, the restore did not roll the position
    /// counter back, and neither cell could tell a re-mint from its absence.
    pub reused_positions: usize,
    /// How many [`REISSUED`] events the object yields.
    pub appended: usize,
    /// Whether the [`KEPT`] events still carry the identities they were
    /// written with.
    pub kept_intact: bool,
}

impl Verdict {
    /// Whether the cell showed what it exists to show.
    ///
    /// The re-mint cell passes when no identity is reissued, every
    /// [`REISSUED`] event landed, the restore handed a lost position out again
    /// and the kept events kept their identities. The control passes when an
    /// identity *is* reissued: the hazard, observed without the re-mint.
    #[must_use]
    pub fn passed(&self) -> bool {
        let landed = self.appended == REISSUED.len() && self.kept_intact;
        match self.mode {
            Mode::Remint => landed && self.reissued.is_empty() && self.reused_positions > 0,
            Mode::Control => landed && !self.reissued.is_empty(),
        }
    }
}

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "mode={} verdict={} reissued={} reused-positions={} appended={} kept={}",
            self.mode.name(),
            if self.passed() { "pass" } else { "fail" },
            self.reissued.len(),
            self.reused_positions,
            self.appended,
            if self.kept_intact {
                "intact"
            } else {
                "changed"
            },
        )?;
        for reissue in &self.reissued {
            write!(
                f,
                "\nreissued {} named {} before the restore and {} after",
                reissue.id, reissue.was, reissue.now
            )?;
        }
        Ok(())
    }
}

/// Judges what the object yields after the restore, `after`, against the
/// ledger taken before it, for the cell `mode`.
#[must_use]
pub fn judge(mode: Mode, before: &Ledger, after: &[Sighting]) -> Verdict {
    // Owned copies: the verdict outlives the reads it was judged from, and
    // is what the response carries.
    let reissued = after
        .iter()
        .filter_map(|now| {
            before
                .events
                .iter()
                .find(|was| was.id == now.id && was.event_type != now.event_type)
                .map(|was| Reissue {
                    id: now.id.clone(),
                    was: was.event_type.clone(),
                    now: now.event_type.clone(),
                })
        })
        .collect();
    let lost: Vec<&str> = of_types(&before.events, &LOST)
        .into_iter()
        .filter_map(Sighting::position)
        .collect();
    let appended = of_types(after, &REISSUED);
    let reused_positions = appended
        .iter()
        .filter(|event| event.position().is_some_and(|at| lost.contains(&at)))
        .count();
    Verdict {
        mode,
        reissued,
        reused_positions,
        appended: appended.len(),
        kept_intact: of_types(after, &KEPT) == of_types(&before.events, &KEPT),
    }
}

#[cfg(test)]
mod tests;
