//! AC-009 — the console output is legible piped to a file.
//!
//! Whole lines, no ANSI, no carriage-return rewrite, no percentage, nothing past
//! eighty columns, and no assumption of a terminal. A sweep that takes minutes
//! is usually piped, and a progress animation in a log file is noise a reader
//! six months out has to scroll through.

use polling_cost::progress;

#[test]
fn a_progress_line_carries_no_control_bytes() {
    let line = progress::cell_line(3, 144, "amplification n=32 log=100000 arm=overlapping");
    assert!(!line.contains('\u{1b}'), "the line carries an ANSI escape");
    assert!(!line.contains('\r'), "the line rewrites itself in place");
    assert!(!line.contains('\n'), "one line is one line");
    assert!(!line.contains('%'), "the line carries a percentage");
}

#[test]
fn a_long_line_is_truncated_rather_than_wrapped() {
    let long = "x".repeat(400);
    let line = progress::line(&long);
    assert_eq!(line.chars().count(), progress::COLUMNS);
    assert!(
        line.ends_with('…'),
        "a line over budget was wrapped or cut silently; a wrapped line in a \
         piped log is two records that look like one"
    );
}

#[test]
fn every_line_the_sweep_would_emit_fits_the_budget() {
    let cells = polling_cost::sweep();
    let total = cells.len();
    for (index, cell) in cells.iter().enumerate() {
        let line = progress::cell_line(
            index + 1,
            total,
            &format!(
                "{} n={} log={} poll={}ms arm={}",
                cell.phase.as_str(),
                cell.fan_out,
                cell.log_size,
                cell.poll_interval_ms,
                cell.arm.as_str()
            ),
        );
        assert!(
            line.chars().count() <= progress::COLUMNS,
            "a sweep line is {} columns: {line}",
            line.chars().count()
        );
        assert!(!line.contains('\u{1b}') && !line.contains('\r'));
    }
}

#[test]
fn embedded_control_characters_are_flattened_rather_than_forwarded() {
    let line = progress::line("first\rsecond\nthird\ttab");
    assert_eq!(line, "first second third tab");
}
