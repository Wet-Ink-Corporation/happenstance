//! What a run says about itself, so a figure can never be quoted without the
//! conditions that produced it.
//!
//! # The rule this module exists to enforce
//!
//! `.kb/reference/README.md:22-28`:
//!
//! > A measurement without a commit sha or a date is not a weaker reference, it
//! > is a false one.
//!
//! So every artefact this crate writes — every raw log, every history entry —
//! opens with a [`Conditions`] block, and the block is built by reading the
//! environment rather than by a human typing it. A conditions table that is
//! maintained by hand is a conditions table that is wrong after the first
//! toolchain bump.
//!
//! # Why the commit sha is read from git and the dirty flag is kept
//!
//! A figure taken against uncommitted work is not reproducible by anybody, and
//! the honest way to say so is to record it. [`Conditions::commit`] carries a
//! `-dirty` suffix when the tree has modifications, and every consumer of these
//! records treats a dirty run as provisional.

use std::fmt;
use std::process::Command;

use serde::{Deserialize, Serialize};

/// Everything a reader needs in order to know what a figure is a figure *of*.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conditions {
    /// The commit the figures were taken at, with `-dirty` appended if the tree
    /// carried uncommitted changes.
    pub commit: String,
    /// The local date, `YYYY-MM-DD`.
    pub date: String,
    /// `rustc --version`, one line.
    pub toolchain: String,
    /// Target triple, from `cfg!` rather than from a command, so it describes
    /// the binary that is running rather than the compiler that is installed.
    pub target: String,
    /// The build profile the binary was compiled under.
    pub profile: String,
    /// Logical processors visible to the process.
    pub logical_cpus: usize,
    /// The SQLite the `bundled` driver linked.
    pub sqlite_version: String,
}

impl Conditions {
    /// Reads the conditions off the running process and its checkout.
    ///
    /// # Panics
    ///
    /// Panics if the SQLite version cannot be read from an in-memory
    /// connection, which would mean the driver itself is broken.
    pub fn read() -> Self {
        let connection = rusqlite::Connection::open_in_memory()
            .expect("a broken measurement environment, not a finding");
        let sqlite_version: String = connection
            .query_row("SELECT sqlite_version()", [], |row| row.get(0))
            .expect("a broken measurement environment, not a finding");

        Self {
            commit: commit(),
            date: date(),
            toolchain: git_free_command("rustc", &["--version"])
                .unwrap_or_else(|| "unknown".to_owned()),
            target: target_triple().to_owned(),
            profile: if cfg!(debug_assertions) {
                // Named rather than inferred, because it is the single most
                // common way a figure ends up an order of magnitude wrong: a
                // debug build reports a number nobody can reproduce under the
                // profile the documentation claims.
                "debug — NOT A MEASUREMENT PROFILE".to_owned()
            } else {
                "release".to_owned()
            },
            logical_cpus: std::thread::available_parallelism().map_or(0, std::num::NonZero::get),
            sqlite_version,
        }
    }
}

impl fmt::Display for Conditions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "commit:    {}", self.commit)?;
        writeln!(f, "date:      {}", self.date)?;
        writeln!(f, "toolchain: {}", self.toolchain)?;
        writeln!(f, "target:    {}", self.target)?;
        writeln!(f, "profile:   {}", self.profile)?;
        writeln!(f, "cpus:      {}", self.logical_cpus)?;
        write!(f, "sqlite:    {}", self.sqlite_version)
    }
}

/// The current commit, suffixed `-dirty` when the tree has modifications.
fn commit() -> String {
    let Some(sha) = git_free_command("git", &["rev-parse", "--short", "HEAD"]) else {
        return "unknown".to_owned();
    };
    let dirty = git_free_command("git", &["status", "--porcelain"])
        .is_some_and(|status| !status.trim().is_empty());
    if dirty { format!("{sha}-dirty") } else { sha }
}

/// Today's UTC date as `YYYY-MM-DD`.
///
/// # Why not the commit's date
///
/// That was the first version — `git log -1 --format=%cs` — and it is wrong in
/// a way that only shows up on the *second* run: it reports the commit's date
/// rather than the run's, so two runs a week apart at the same commit produce
/// the same `results/history/` filename and the second silently overwrites the
/// first. A history whose entries can overwrite each other is not a history.
/// The commit itself is still carried, in [`Conditions::commit`].
///
/// # Why not a date crate
///
/// RS-50-5: *"do not add a dependency to run four assertions"*. `chrono` or
/// `time` would put a graph behind one formatted string. The conversion below
/// is exact for every date in the proleptic Gregorian calendar and is ten
/// lines, checked by its own test.
///
/// UTC rather than local time, and that is a deliberate trade: a local date
/// makes two runs on the same afternoon in two timezones disagree about which
/// day they happened, and the whole point of the field is that two entries can
/// be ordered.
fn date() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| since.as_secs());
    let (year, month, day) = civil_from_days(i64::try_from(seconds / 86_400).unwrap_or(0));
    format!("{year:04}-{month:02}-{day:02}")
}

/// Days since the Unix epoch to a `(year, month, day)` triple.
///
/// Howard Hinnant's `civil_from_days`, transcribed. It shifts the epoch to
/// 0000-03-01 so the leap day lands at the end of a year and the day-of-year
/// arithmetic needs no month-length table.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let shifted = days + 719_468;
    let era = if shifted >= 0 {
        shifted
    } else {
        shifted - 146_096
    } / 146_097;
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = u32::try_from(day_of_year - (153 * shifted_month + 2) / 5 + 1).unwrap_or(1);
    let month = u32::try_from(if shifted_month < 10 {
        shifted_month + 3
    } else {
        shifted_month - 9
    })
    .unwrap_or(1);
    (if month <= 2 { year + 1 } else { year }, month, day)
}

#[cfg(test)]
mod tests {
    use super::civil_from_days;

    /// CONTROL — the date arithmetic is exact at the epoch, at a leap day, and
    /// at the century rule.
    ///
    /// The three places this conversion can be wrong, and the reason it is
    /// worth ten lines rather than a dependency: all three are checkable in one
    /// test. A wrong date here does not fail anything — it quietly mislabels
    /// every history entry, which is the failure mode
    /// `.kb/reference/README.md:22-28` calls *"not a weaker reference, a false
    /// one"*.
    #[test]
    fn the_civil_calendar_conversion_is_exact() {
        assert_eq!(civil_from_days(0), (1970, 1, 1), "the Unix epoch");
        assert_eq!(
            civil_from_days(59),
            (1970, 3, 1),
            "1970 is not a leap year, so day 59 is 1 March"
        );
        assert_eq!(civil_from_days(789), (1972, 2, 29), "1972 is a leap year");
        assert_eq!(civil_from_days(790), (1972, 3, 1));
        // 2000 is a leap year because it divides by 400; 1900 was not, because
        // it divides by 100 and not 400. The second is what a naive rule gets
        // wrong, and the first is what a rule that only knows about 100 gets
        // wrong.
        assert_eq!(
            civil_from_days(11_016),
            (2000, 2, 29),
            "2000 is a leap year"
        );
        // 1900 was *not* a leap year — it divides by 100 and not by 400 — so
        // 28 February is followed directly by 1 March. This is the arm that
        // needs a negative day count, and therefore the arm that catches a
        // truncating division on the era.
        assert_eq!(civil_from_days(-25_509), (1900, 2, 28));
        assert_eq!(
            civil_from_days(-25_508),
            (1900, 3, 1),
            "1900 divides by 100 and not 400, so it has no 29 February"
        );
        assert_eq!(civil_from_days(20_701), (2026, 9, 5));
    }
}

/// Runs a command and returns its trimmed stdout, or `None` if it could not run
/// or failed.
fn git_free_command(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// The target triple this binary was built for.
const fn target_triple() -> &'static str {
    // `cfg!` rather than a build script: a build script would be a second place
    // this crate's compilation can go wrong, for one string.
    if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        "x86_64-pc-windows-msvc"
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        "x86_64-unknown-linux-gnu"
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        "aarch64-apple-darwin"
    } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
        "x86_64-apple-darwin"
    } else {
        "unrecognised — state it by hand in the results README"
    }
}

/// One row of a results table: a named figure, its units, and the shape it was
/// taken at.
///
/// Serialised into `results/history/` so a later run can diff against it, and
/// printed as CSV into `results/raw/` so the hand-written tables in
/// `results/*.md` have something to be written *from*.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    /// Which group the figure belongs to, e.g. `store_append`.
    pub group: String,
    /// Which arm produced it, e.g. `sqlite` or `raw/hand-rolled`.
    pub arm: String,
    /// The workload shape, e.g. `owned/1024B/3tags`.
    pub shape: String,
    /// What is being counted.
    pub metric: String,
    /// The figure.
    pub value: f64,
    /// The unit, e.g. `ns`, `events/s`, `heap_ops`, `bytes`.
    pub unit: String,
    /// Whether this row may stand as a headline number.
    ///
    /// `false` for anything taken in [`crate::corpus::Regime::Interned`], and
    /// for any absolute from a run whose drift was outside
    /// [`crate::paired::STABLE_DRIFT_MARGIN`]. A row that cannot be a headline
    /// is still committed — it is the control — but a reader meets it labelled.
    pub representative: bool,
}

impl Row {
    /// The CSV header these rows are written under.
    pub const CSV_HEADER: &'static str = "group,arm,shape,metric,value,unit,representative";

    /// This row as one CSV line, without a trailing newline.
    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{},{},{}",
            self.group,
            self.arm,
            self.shape,
            self.metric,
            self.value,
            self.unit,
            self.representative
        )
    }
}

/// One run's whole record: what it measured, and under what.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRecord {
    /// The conditions, read off the running process.
    pub conditions: Conditions,
    /// Every figure the run produced.
    pub rows: Vec<Row>,
}

impl RunRecord {
    /// An empty record with the conditions already read.
    pub fn new() -> Self {
        Self {
            conditions: Conditions::read(),
            rows: Vec::new(),
        }
    }

    /// Adds a figure.
    pub fn push(&mut self, row: Row) {
        self.rows.push(row);
    }

    /// The whole record as CSV, conditions in `#`-prefixed comment lines above
    /// the header.
    ///
    /// Comment lines rather than a separate file because the two drift apart
    /// the first time somebody copies one of them: a figure and the conditions
    /// it was taken under must be impossible to separate by accident.
    pub fn to_csv(&self) -> String {
        let mut out = String::new();
        for line in self.conditions.to_string().lines() {
            out.push_str("# ");
            out.push_str(line);
            out.push('\n');
        }
        out.push_str(Row::CSV_HEADER);
        out.push('\n');
        for row in &self.rows {
            out.push_str(&row.to_csv());
            out.push('\n');
        }
        out
    }
}

impl Default for RunRecord {
    fn default() -> Self {
        Self::new()
    }
}
