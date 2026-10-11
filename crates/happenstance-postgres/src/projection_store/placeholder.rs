//! How many values a statement's text asks for: the highest `$n` it uses.
//!
//! ADR-0084 makes the adapter refuse a projection statement whose bound values
//! do not match its placeholders, in both directions and before anything is
//! sent. The server alone refuses only too few: a surplus value is accepted on a
//! connection preparing the text for the first time, and the prepared statement
//! it caches then refuses every later *correct* call of that text on that
//! connection.
//!
//! # Why a scan and not `describe`
//!
//! Asking the server costs a round trip per statement, and `sqlx`'s `describe`
//! answers from the same per-connection cache the surplus poisons, so it would
//! report a stale count rather than the server's. The scan is a few dozen lines
//! of `std` and runs before any `await`.
//!
//! # What it skips, and the one place it can disagree with the server
//!
//! It follows PostgreSQL's lexer (`scan.l`) with `standard_conforming_strings`
//! on: a `$n` inside a standard, escape (`E'…'`), bit, hex, national or Unicode
//! (`U&'…'`) string, a quoted identifier, a dollar-quoted body or a comment —
//! line or nested block — is not a placeholder, and neither is a `$` that
//! continues an identifier (`a$1`). A string continued across a newline
//! (`E'a'` newline `'b'`) keeps the quoting it began with, as `scan.l`'s
//! `quotecontinue` does. An unterminated construct runs to the end of the text,
//! which the server refuses on its own.
//!
//! With `standard_conforming_strings = off`, which has not been the default
//! since PostgreSQL 9.1, a backslash inside a plain `'…'` string escapes the
//! next byte, and the scan can then disagree with the server. ADR-0084 §10 names
//! that as a falsifier: a disagreement is a defect in this scan, fixed here and
//! pinned by a test.
//!
//! Every byte is read through `get`, so no input can make the scan panic, and a
//! placeholder number too large for `usize` saturates rather than overflowing.

use crate::error::PostgresProjectionStoreError;

/// The highest `$n` that `sql` uses as a parameter, or `0` when it uses none.
///
/// The highest, not the number of uses: `$1 OR k = $1` asks for one value, and
/// a text that uses only `$3` asks for three.
pub(crate) fn highest_placeholder(sql: &str) -> usize {
    let bytes = sql.as_bytes();
    let mut highest = 0_usize;
    let mut at = 0_usize;
    while let Some(&byte) = bytes.get(at) {
        let next = at.saturating_add(1);
        at = match byte {
            b'\'' => skip_quoted(bytes, next, b'\'', Backslash::Literal),
            b'"' => skip_quoted(bytes, next, b'"', Backslash::Literal),
            b'-' if bytes.get(next) == Some(&b'-') => skip_line_comment(bytes, next),
            b'/' if bytes.get(next) == Some(&b'*') => {
                skip_block_comment(bytes, next.saturating_add(1))
            }
            b'$' => match parameter(bytes, next) {
                Some((value, end)) => {
                    highest = highest.max(value);
                    end
                }
                None => dollar_quote_end(sql, at).unwrap_or(next),
            },
            _ if is_identifier_start(byte) => skip_identifier(bytes, at),
            _ => next,
        };
    }
    highest
}

/// Refuses a statement whose value count is not the highest `$n` its text uses.
///
/// `statement` is the zero-based position reported in the error.
pub(crate) fn check_parameter_count(
    statement: usize,
    sql: &str,
    supplied: usize,
) -> Result<(), PostgresProjectionStoreError> {
    let declared = highest_placeholder(sql);
    if declared == supplied {
        Ok(())
    } else {
        Err(PostgresProjectionStoreError::ParameterCount {
            statement,
            declared,
            supplied,
        })
    }
}

/// What a backslash means inside a quoted construct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Backslash {
    /// An ordinary byte: standard strings and quoted identifiers.
    Literal,
    /// It escapes the byte after it: `E'…'` strings.
    Escapes,
}

/// `[A-Za-z_]`, or any byte of a multi-byte UTF-8 character.
const fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_' || byte >= 0x80
}

/// An identifier byte after the first. `$` is one, which is why `a$1` is a
/// single identifier rather than `a` followed by a placeholder.
const fn is_identifier_continue(byte: u8) -> bool {
    is_identifier_start(byte) || byte.is_ascii_digit() || byte == b'$'
}

/// A dollar-quote tag byte after the first: an identifier byte, but not `$`.
const fn is_tag_continue(byte: u8) -> bool {
    is_identifier_start(byte) || byte.is_ascii_digit()
}

/// The index just past the run of bytes from `from` that satisfy `keep`.
fn run_end(bytes: &[u8], from: usize, keep: impl Fn(u8) -> bool) -> usize {
    bytes
        .get(from..)
        .and_then(|rest| rest.iter().position(|&byte| !keep(byte)))
        .map_or(bytes.len(), |offset| from.saturating_add(offset))
}

/// Skips an identifier or keyword starting at `start`, and the string it
/// prefixes when it is exactly `E`/`e` (an escape string) or `U&`/`u&` (a
/// Unicode string or identifier).
///
/// Longest match decides: in `somee'…'` the identifier is `somee` and the
/// string after it is a standard one, so only a one-byte `e` prefixes.
fn skip_identifier(bytes: &[u8], start: usize) -> usize {
    let end = run_end(bytes, start.saturating_add(1), is_identifier_continue);
    let one_byte = end == start.saturating_add(1);
    let prefix = bytes
        .get(start)
        .copied()
        .map(|byte| byte.to_ascii_lowercase());
    let after = |offset: usize| bytes.get(end.saturating_add(offset)).copied();
    match (one_byte, prefix, after(0), after(1)) {
        (true, Some(b'e'), Some(b'\''), _) => {
            skip_quoted(bytes, end.saturating_add(1), b'\'', Backslash::Escapes)
        }
        (true, Some(b'u'), Some(b'&'), Some(quote @ (b'\'' | b'"'))) => {
            skip_quoted(bytes, end.saturating_add(2), quote, Backslash::Literal)
        }
        _ => end,
    }
}

/// The index just past the `quote` that closes a construct whose body starts at
/// `from`. A doubled `quote` is part of the body. The end of the text when the
/// construct is unterminated.
///
/// A string (`'`) continued across a newline is one string, in the quoting it
/// began with: `scan.l`'s `quotecontinue` returns to the state the first
/// segment was in, so after `E'…'` a continuation still honours backslashes.
/// Quoted identifiers have no continuation.
fn skip_quoted(bytes: &[u8], from: usize, quote: u8, backslash: Backslash) -> usize {
    let mut at = from;
    while let Some(&byte) = bytes.get(at) {
        let next = at.saturating_add(1);
        if byte == b'\\' && backslash == Backslash::Escapes {
            at = next.saturating_add(1);
        } else if byte == quote {
            if bytes.get(next) == Some(&quote) {
                at = next.saturating_add(1);
            } else if let Some(reopened) = continuation(bytes, next).filter(|_| quote == b'\'') {
                at = reopened.saturating_add(1);
            } else {
                return next;
            }
        } else {
            at = next;
        }
    }
    bytes.len()
}

/// `[ \t\f\v]`: `scan.l`'s `horiz_space`.
const fn is_horizontal_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | 0x0b | 0x0c)
}

/// `[\n\r]`: `scan.l`'s `newline`.
const fn is_newline(byte: u8) -> bool {
    matches!(byte, b'\n' | b'\r')
}

/// The index of the `'` that continues a string closed just before `from`, or
/// `None` when what follows is not a continuation.
///
/// `scan.l`'s `quotecontinue`: horizontal space or `--` comments, then a
/// newline, then any whitespace or `--` comments each ending in a newline, then
/// the quote. Without the newline, `'a' 'b'` is two tokens.
fn continuation(bytes: &[u8], from: usize) -> Option<usize> {
    let mut at = from;
    let mut crossed_newline = false;
    while let Some(&byte) = bytes.get(at) {
        let next = at.saturating_add(1);
        at = match byte {
            b'\'' if crossed_newline => return Some(at),
            _ if is_horizontal_space(byte) => next,
            _ if is_newline(byte) => {
                crossed_newline = true;
                next
            }
            b'-' if bytes.get(next) == Some(&b'-') => {
                let end = skip_line_comment(bytes, next);
                // A comment after the newline must itself end in one.
                if crossed_newline && !bytes.get(end).copied().is_some_and(is_newline) {
                    return None;
                }
                end
            }
            _ => return None,
        };
    }
    None
}

/// The index of the line break that ends a `--` comment, or the end of the
/// text.
fn skip_line_comment(bytes: &[u8], from: usize) -> usize {
    run_end(bytes, from, |byte| byte != b'\n' && byte != b'\r')
}

/// The index just past the `*/` that closes a block comment whose body starts
/// at `from`. PostgreSQL's block comments **nest**, so an inner `/*` needs its
/// own `*/`.
fn skip_block_comment(bytes: &[u8], from: usize) -> usize {
    let mut depth = 1_usize;
    let mut at = from;
    while let Some(&byte) = bytes.get(at) {
        let next = at.saturating_add(1);
        match (byte, bytes.get(next)) {
            (b'/', Some(b'*')) => {
                depth = depth.saturating_add(1);
                at = next.saturating_add(1);
            }
            (b'*', Some(b'/')) => {
                depth = depth.saturating_sub(1);
                at = next.saturating_add(1);
                if depth == 0 {
                    return at;
                }
            }
            _ => at = next,
        }
    }
    bytes.len()
}

/// The parameter whose digits start at `from`, with the index just past them,
/// or `None` when no digit follows the `$`.
///
/// Accumulated with saturating arithmetic, so `$01` is 1 and a number too large
/// for `usize` is `usize::MAX` — which no caller can supply that many values
/// for, so it is refused rather than wrapped into a plausible count.
fn parameter(bytes: &[u8], from: usize) -> Option<(usize, usize)> {
    let end = run_end(bytes, from, |byte| byte.is_ascii_digit());
    let digits = bytes.get(from..end).filter(|digits| !digits.is_empty())?;
    let value = digits.iter().fold(0_usize, |value, &digit| {
        value
            .saturating_mul(10)
            .saturating_add(usize::from(digit.saturating_sub(b'0')))
    });
    Some((value, end))
}

/// The index just past a dollar-quoted body whose opening `$` is at `start`, or
/// `None` when no tag opens there.
///
/// A tag is `$$`, or `$`, an identifier-start byte, tag bytes, and `$`. The
/// body runs to the next occurrence of the exact opening delimiter, or to the
/// end of the text.
fn dollar_quote_end(sql: &str, start: usize) -> Option<usize> {
    let bytes = sql.as_bytes();
    let tag_start = start.saturating_add(1);
    let tag_end = match bytes.get(tag_start) {
        Some(b'$') => tag_start,
        Some(&byte) if is_identifier_start(byte) => {
            run_end(bytes, tag_start.saturating_add(1), is_tag_continue)
        }
        _ => return None,
    };
    if bytes.get(tag_end) != Some(&b'$') {
        return None;
    }
    let body = tag_end.saturating_add(1);
    // Both ends of the delimiter are ASCII `$`, so both are char boundaries.
    let delimiter = sql.get(start..body)?;
    let rest = sql.get(body..)?;
    Some(rest.find(delimiter).map_or(sql.len(), |offset| {
        body.saturating_add(offset).saturating_add(delimiter.len())
    }))
}

#[cfg(test)]
mod tests {
    use super::{check_parameter_count, highest_placeholder};
    use crate::error::PostgresProjectionStoreError;

    /// Asserts `highest_placeholder(sql) == expected`, naming the statement.
    fn declares(sql: &str, expected: usize) {
        assert_eq!(
            highest_placeholder(sql),
            expected,
            "the scan read the wrong highest placeholder from {sql:?}"
        );
    }

    #[test]
    fn a_statement_without_placeholders_declares_none() {
        declares("DELETE FROM t", 0);
        declares("", 0);
    }

    /// The highest `$n`, not the number of `$n` tokens: the server sizes its
    /// parameter list by the highest, and a repeated or out-of-order use binds
    /// the same value again.
    #[test]
    fn the_highest_placeholder_is_reported_not_the_number_of_uses() {
        declares("DELETE FROM t WHERE k = $1", 1);
        declares("INSERT INTO t VALUES ($1, $2)", 2);
        declares("DELETE FROM t WHERE k = $1 OR j = $1", 1);
        declares("INSERT INTO t VALUES ($2, $1)", 2);
        declares("SELECT $3", 3);
        declares("SELECT $12", 12);
        declares("SELECT $01", 1);
    }

    #[test]
    fn placeholders_inside_string_literals_are_not_counted() {
        declares("SELECT $1, '$2'", 1);
        declares("SELECT $1, 'it''s $2'", 1);
        declares("SELECT $1, B'$2'", 1);
        declares("SELECT $1, X'$2'", 1);
        declares("SELECT $1, N'$2'", 1);
        declares("SELECT $1, U&'$2'", 1);
        // A standard string does not treat a backslash as an escape, so the
        // second quote closes it and `$2` is a real placeholder.
        declares(r"SELECT '\', $2", 2);
    }

    /// The case that rejects a scan treating `e` as an escape prefix wherever it
    /// appears: `somee'\'` is the identifier `somee` and then a *standard*
    /// string, which ends at the second quote.
    #[test]
    fn an_escape_string_honours_backslash_escapes() {
        declares(r"SELECT E'\'$2', $1", 1);
        declares(r"SELECT e'\\', $2", 2);
        declares(r"SELECT e'it''s $3', $1", 1);
        declares(r"SELECT somee'\', $2", 2);
    }

    /// A string continued across a newline (`'a'` newline `'b'`) keeps the
    /// quoting of the string it continues: PostgreSQL's `quotecontinue` returns
    /// to the state the first segment was in. So after `E'…'` the next segment
    /// still honours backslashes. Both statements below were counted by
    /// PostgreSQL 17.10 (1 and 2), and a scan that restarts a continuation as a
    /// standard string reads 2 and 0.
    #[test]
    fn a_continued_escape_string_keeps_honouring_backslashes() {
        declares("SELECT E'x'\n'\\' $2', $1::text", 1);
        declares("SELECT E''\n'\\'', $2::text, $1::text", 2);
        declares("SELECT E'x' -- c\n  -- d\n'\\' $2', $1::text", 1);
        declares("SELECT e'x' \t\r\n\n '\\' $2', $1", 1);
        // Not a continuation: no newline before the next quote, so it is a
        // fresh standard string, which a backslash does not escape.
        declares("SELECT E'x', '\\', $2::text, $1::text", 2);
        // A continuation of a standard string is standard either way.
        declares("SELECT 'a'\n'\\', $1::text", 1);
        declares("SELECT U&'a'\n'$2', $1::text", 1);
    }

    #[test]
    fn placeholders_inside_quoted_identifiers_are_not_counted() {
        declares(r#"SELECT $1 AS "col$2""#, 1);
        declares(r#"SELECT $1 AS "a""$2""#, 1);
        declares(r#"SELECT $1 AS U&"$2""#, 1);
    }

    #[test]
    fn placeholders_inside_dollar_quoted_bodies_are_not_counted() {
        declares("SELECT $$ $2 $$, $1", 1);
        declares("SELECT $tag$ $2 $other$ $tag$, $1", 1);
        declares("SELECT $_t9$ $2 $_t9$, $1", 1);
        declares("SELECT $1, $$ unterminated $2", 1);
        // A parameter ends at its last digit, so a tag may follow it at once.
        declares("SELECT $1$$ $2 $$", 1);
        declares("SELECT $1::text||$a1$ $2 $a1$", 1);
    }

    /// Nesting is the case that rejects a scan which closes a block comment at
    /// the first `*/`: PostgreSQL's comments nest, so `$2` below is still inside.
    #[test]
    fn placeholders_inside_comments_are_not_counted() {
        declares("SELECT $1 -- $2\n", 1);
        declares("SELECT $1 -- $3\r, $2", 2);
        declares("SELECT $1 /* $2 */", 1);
        declares("SELECT /* /* */ $2 */ $1", 1);
        declares("SELECT $1 /* unterminated $2", 1);
    }

    #[test]
    fn a_dollar_inside_an_identifier_is_not_a_placeholder() {
        declares("SELECT a$1", 0);
        declares("SELECT $1 AS a$2", 1);
        declares("SELECT _x$3, $1", 1);
        // A multi-byte identifier byte continues an identifier too.
        declares("SELECT $1::text AS é$2", 1);
        declares("SELECT café$1", 0);
    }

    /// An operator byte ends the token before it, so a placeholder directly
    /// after `=` or `-` is still one — and `--` is a comment, not two minuses.
    #[test]
    fn a_placeholder_directly_after_an_operator_is_counted() {
        declares("SELECT 1 WHERE k=$1", 1);
        declares("SELECT -$1::int", 1);
        declares("SELECT 1--$1", 0);
    }

    /// Every string of one to four bytes over the bytes the scan dispatches on
    /// returns rather than panicking — 18 + 18² + 18³ + 18⁴ statements,
    /// exhaustively, which is the totality property a property test would
    /// sample.
    #[test]
    fn the_scan_is_total() {
        const ALPHABET: [char; 18] = [
            '$', '1', '9', '\'', '"', '-', '/', '*', 'e', 'E', 'u', 'U', '_', '\\', '\n', '\r',
            ' ', '&',
        ];
        let mut scanned = 0_usize;
        let mut texts: Vec<String> = vec![String::new()];
        for _ in 0..4 {
            texts = texts
                .iter()
                .flat_map(|prefix| {
                    ALPHABET
                        .iter()
                        .map(move |symbol| format!("{prefix}{symbol}"))
                })
                .collect();
            for text in &texts {
                let highest = highest_placeholder(text);
                assert!(highest <= 999, "{text:?} declared {highest}");
                scanned += 1;
            }
        }
        assert_eq!(scanned, 18 + 18 * 18 + 18 * 18 * 18 + 18 * 18 * 18 * 18);

        let absurd = format!("SELECT ${}", "9".repeat(25));
        assert_eq!(
            highest_placeholder(&absurd),
            usize::MAX,
            "{absurd} saturates"
        );
    }

    #[test]
    fn check_parameter_count_refuses_both_directions() {
        assert!(matches!(
            check_parameter_count(0, "DELETE FROM t WHERE k = $1 OR k = $2", 1),
            Err(PostgresProjectionStoreError::ParameterCount {
                statement: 0,
                declared: 2,
                supplied: 1,
            })
        ));
        assert!(matches!(
            check_parameter_count(3, "DELETE FROM t WHERE k = $1", 2),
            Err(PostgresProjectionStoreError::ParameterCount {
                statement: 3,
                declared: 1,
                supplied: 2,
            })
        ));
        assert!(matches!(
            check_parameter_count(0, "DELETE FROM t WHERE k = $1", 1),
            Ok(())
        ));
    }
}
