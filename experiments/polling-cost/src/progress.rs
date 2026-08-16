//! The harness's own console output: plain lines, and nothing else.
//!
//! Whole `\n`-terminated lines, no ANSI colour, no spinner, no percentage, no
//! carriage-return rewrite, nothing past eighty columns, and no assumption that
//! anything is attached to a terminal. A sweep that takes minutes is usually
//! piped to a file, and a progress animation is illegible there — it is also the
//! design's own rejection, which refused a progress indicator for the projection
//! runner on the grounds that polling renders nothing.
//!
//! The width is a budget rather than a wrap: a line that would exceed it is
//! **truncated with an ellipsis**, because a wrapped line in a piped log is two
//! records that look like one.

/// The line-length budget, in columns.
pub const COLUMNS: usize = 80;

/// One line of progress, fitted to the budget.
#[must_use]
pub fn line(text: &str) -> String {
    let text = text.replace(['\n', '\r', '\t'], " ");
    if text.chars().count() <= COLUMNS {
        return text;
    }
    let kept: String = text.chars().take(COLUMNS.saturating_sub(1)).collect();
    format!("{kept}…")
}

/// A cell's headline, as one line.
#[must_use]
pub fn cell_line(index: usize, total: usize, description: &str) -> String {
    line(&format!("[{index}/{total}] {description}"))
}

/// Writes one line to standard output.
pub fn say(text: &str) {
    // `println!` and nothing else: one line, one newline, no control bytes.
    println!("{}", line(text));
}
