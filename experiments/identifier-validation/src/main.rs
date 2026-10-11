//! Prints the run `README.md` records: E11-R per position and constructor, the
//! refused set as ranges, and the corpus result. Exits non-zero unless the run
//! is empty by the pre-registered criteria.

use std::fmt::{self, Write as _};
use std::io::{self, Write};
use std::process::ExitCode;

use identifier_validation::{
    CORPUS, Constructor, NEGATIVE_INSERTS, PAIRS, Position, RTL_BLOCKS, SCALAR_VALUE_COUNT,
    SweepReport, Verdict, check_corpus, check_pairs, shape_problems, sweep,
};

fn main() -> io::Result<ExitCode> {
    let mut out = io::stdout().lock();

    let sweeps_empty = write_sweeps(&mut out)?;
    let corpus_empty = write_corpus(&mut out)?;

    let empty = sweeps_empty && corpus_empty;
    let result = if empty { "EMPTY" } else { "NOT EMPTY" };
    writeln!(out, "\n== Result: {result}")?;
    Ok(if empty {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

/// Criteria 1 and 2, and the coverage condition. Returns whether all held.
fn write_sweeps(out: &mut impl Write) -> io::Result<bool> {
    writeln!(out, "== E11-R: every scalar value, against the oracle")?;
    let mut empty = true;
    for position in Position::ALL {
        for constructor in Constructor::ALL {
            let report = sweep(position, |value| constructor.verdict(value));
            writeln!(
                out,
                "{position:?} / {constructor:?}: checked {}, disagreements {}, refused {}",
                report.checked, report.disagreements, report.refused_count
            )?;
            for d in &report.first_disagreements {
                writeln!(
                    out,
                    "  U+{:04X}: expected {:?}, got {:?}",
                    u32::from(d.value),
                    d.expected,
                    d.actual
                )?;
            }
            empty &= report.checked == SCALAR_VALUE_COUNT && report.disagreements == 0;
            if position == Position::Alone {
                empty &= write_refused(out, &report)?;
            }
        }
    }
    Ok(empty)
}

/// Criterion 3, the negative controls and the corpus shape. Returns whether
/// all held.
fn write_corpus(out: &mut impl Write) -> io::Result<bool> {
    writeln!(
        out,
        "\n== Corpus: {} entries, {} pairs",
        CORPUS.len(),
        PAIRS.len()
    )?;
    let mut empty = true;
    for constructor in Constructor::ALL {
        let report = check_corpus(|value| constructor.verdict(value));
        writeln!(
            out,
            "{constructor:?}: checked {}, refused {}; negative controls {} ({} inserts x 3 positions), escaped {}",
            report.entries_checked,
            report.refused.len(),
            report.controls_checked,
            NEGATIVE_INSERTS.len(),
            report.control_escape_count
        )?;
        for refusal in &report.refused {
            let text = escaped(refusal.text);
            writeln!(
                out,
                "  refused {:?} {:?}: {text}",
                refusal.script, refusal.verdict
            )?;
        }
        for escape in &report.control_escapes {
            let (inserted, entry) = (u32::from(escape.inserted), escaped(escape.entry));
            writeln!(
                out,
                "  escaped U+{inserted:04X} at {:?} of {entry}: {:?}",
                escape.at, escape.actual
            )?;
        }
        empty &= report.refused.is_empty() && report.control_escape_count == 0;
    }

    let pairs = check_pairs();
    writeln!(
        out,
        "Tag::key_value: checked {}, refused {}",
        PAIRS.len(),
        pairs.len()
    )?;
    for refusal in &pairs {
        let (key, value) = (escaped(refusal.pair.key), escaped(refusal.pair.value));
        writeln!(out, "  refused {key}:{value}: {:?}", refusal.verdict)?;
    }

    let shape = shape_problems();
    writeln!(out, "Corpus shape problems: {}", shape.len())?;
    for problem in &shape {
        writeln!(out, "  {problem:?}")?;
    }

    Ok(empty && pairs.is_empty() && shape.is_empty())
}

/// Writes the recorded refusals as ranges with their verdicts, says when that
/// list was capped, and names the RTL block of any recorded refusal. Returns
/// whether no refusal at all, recorded or not, lies in an RTL block.
fn write_refused(out: &mut impl Write, report: &SweepReport) -> io::Result<bool> {
    for (start, end, verdict) in ranges(&report.refused) {
        let (start, end) = (u32::from(start), u32::from(end));
        writeln!(out, "  refused U+{start:04X}..=U+{end:04X} {verdict:?}")?;
    }
    if report.refused_count > report.refused.len() {
        let (shown, total) = (report.refused.len(), report.refused_count);
        writeln!(out, "  (refused list truncated: {shown} of {total} shown)")?;
    }
    for &(c, _) in &report.refused {
        for block in RTL_BLOCKS.iter().filter(|block| block.range.contains(&c)) {
            let (c, name) = (u32::from(c), block.name);
            writeln!(out, "  U+{c:04X} is inside RTL block {name}")?;
        }
    }
    let inside = report.refused_in_rtl_block;
    writeln!(out, "  refused values inside an RTL block: {inside}")?;
    Ok(inside == 0)
}

/// Collapses consecutive codepoints with the same verdict into ranges.
fn ranges(refused: &[(char, Verdict)]) -> Vec<(char, char, Verdict)> {
    let mut out: Vec<(char, char, Verdict)> = Vec::new();
    for &(c, verdict) in refused {
        match out.last_mut() {
            Some((_, end, last))
                if *last == verdict && u32::from(*end).checked_add(1) == Some(u32::from(c)) =>
            {
                *end = c;
            }
            _ => out.push((c, c, verdict)),
        }
    }
    out
}

/// Displays `text` with every codepoint that is neither ASCII nor alphabetic
/// spelled as `<U+XXXX>`, so a terminal's bidi rendering cannot hide a mark or
/// a joiner.
struct Escaped<'a>(&'a str);

impl fmt::Display for Escaped<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.0.chars() {
            if c.is_ascii() || c.is_alphabetic() {
                f.write_char(c)?;
            } else {
                write!(f, "<U+{:04X}>", u32::from(c))?;
            }
        }
        Ok(())
    }
}

fn escaped(text: &str) -> Escaped<'_> {
    Escaped(text)
}
