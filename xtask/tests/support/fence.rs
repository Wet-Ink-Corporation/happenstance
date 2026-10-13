//! `CommonMark`'s fence rule, shared by the integration tests that read a page
//! or a crate root as markdown.
//!
//! It lives here, under `tests/support/`, rather than in either test file,
//! because two of them walk fences and one shared copy cannot drift between
//! them (#279). The bin carries its own copies, `lint_narrative::fence_marker`
//! and `site::render::marker` (which accepts any indentation), so this is not
//! the only statement of the rule; merging the three is a separate change.
//! Cargo builds only `tests/*.rs` and `tests/*/main.rs` as test targets, so
//! this file is compiled once per test that declares it, and never on its own.

/// A fence delimiter line by `CommonMark`'s rule: up to three spaces of
/// indentation, then a run of three or more of one of `` ` `` or `~`. Returns
/// the character, the run's length and the trimmed rest of the line.
///
/// Four spaces of indentation is an indented code block, not a fence, so such
/// a line is `None`.
pub(crate) fn doc_fence_delimiter(line: &str) -> Option<(char, usize, &str)> {
    let unindented = line.trim_start_matches(' ');
    if line.len() - unindented.len() > 3 {
        return None;
    }
    let mark = unindented
        .chars()
        .next()
        .filter(|c| matches!(c, '`' | '~'))?;
    let rest = unindented.trim_start_matches(mark);
    let run = unindented.len() - rest.len();
    (run >= 3).then_some((mark, run, rest.trim()))
}

/// Whether a delimiter, as [`doc_fence_delimiter`] reads it, closes the fence
/// an opener of `(character, run)` opened.
///
/// A fence closes only on a delimiter of its own character, at least as long
/// as its opener, with nothing after it, so a `~~~` line inside a backtick
/// fence is body rather than a close.
pub(crate) fn closes(
    (opened, length): (char, usize),
    (mark, run, info): (char, usize, &str),
) -> bool {
    mark == opened && run >= length && info.is_empty()
}
