//! `CommonMark`'s fence rule, stated once.
//!
//! Three readers walk fenced blocks: the gate's `lint-narrative` step, the site
//! renderer, and the integration tests that read a page or a crate root as
//! markdown. Each used to carry its own copy of this rule, and the copies had
//! drifted: the renderer accepted any indentation, so a line indented four
//! spaces opened a fence on the site and nowhere else (#294). One page read
//! three ways is the failure this module exists to make impossible.
//!
//! It is `pub` in the lib, rather than `pub(crate)` in the bin, because the
//! bin, the lib and each integration test are separate crates: only the lib's
//! public surface reaches all three. `xtask` is `publish = false`, so `pub`
//! here promises nothing to anyone outside this repository.
//!
//! **Top-level blocks only.** The indentation limit is measured from column
//! zero, which is right for a fence that is a block of its own. A fence inside
//! a list item or a block quote is indented relative to its container, so it
//! can sit at four columns or more and still be a fence, and this rule reads
//! it as an indented code block. The pages these readers walk keep their
//! fences at the top level.

/// One fence delimiter line, as `CommonMark` reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Delimiter<'a> {
    /// `` ` `` or `~`.
    pub mark: char,
    /// How many of `mark` the run carries: three or more.
    pub run: usize,
    /// The rest of the line, trimmed: an opener's info string, and empty on
    /// a line that can close a fence.
    pub info: &'a str,
}

/// What a fence remembers of the delimiter that opened it: enough to know
/// which line closes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Opener {
    /// `` ` `` or `~`. A tilde fence is not closed by backticks.
    pub mark: char,
    /// How many of `mark` the opener carried. A closer needs at least as many.
    pub run: usize,
}

impl Delimiter<'_> {
    /// This delimiter as the opener of a fence.
    #[must_use]
    pub fn opener(&self) -> Opener {
        Opener {
            mark: self.mark,
            run: self.run,
        }
    }

    /// Whether this delimiter closes a fence that `opener` opened.
    ///
    /// A fence closes only on a delimiter of its own character, at least as
    /// long as its opener, with nothing after it. So a `~~~` line inside a
    /// backtick fence is body rather than a close, and a four-backtick block
    /// holds the three-backtick fences inside it as content.
    #[must_use]
    pub fn closes(&self, opener: Opener) -> bool {
        self.mark == opener.mark && self.run >= opener.run && self.info.is_empty()
    }
}

/// The fence delimiter `line` carries, by `CommonMark` 0.31 §4.5.
///
/// A delimiter is up to three spaces of indentation, then a run of three or
/// more of one of `` ` `` or `~`, then the info string. Two lines that look
/// like one are not:
///
/// - Four spaces of indentation, or a tab, makes an indented code block, not a
///   fence.
/// - After a backtick run, the info string may not contain a backtick. So a
///   prose line that opens with an inline code span, such as the line
///   ```` ```x``` and more ````, opens no fence. Read as one, it would
///   swallow the prose after it.
///
/// A tilde run carries no such restriction.
#[must_use]
pub fn delimiter(line: &str) -> Option<Delimiter<'_>> {
    let unindented = line.trim_start_matches(' ');
    if line.len() - unindented.len() > 3 {
        return None;
    }
    let mark = unindented
        .chars()
        .next()
        .filter(|character| matches!(character, '`' | '~'))?;
    let rest = unindented.trim_start_matches(mark);
    // Both marks are one byte, so the byte difference is the run's length.
    let run = unindented.len() - rest.len();
    if run < 3 || (mark == '`' && rest.contains('`')) {
        return None;
    }
    Some(Delimiter {
        mark,
        run,
        info: rest.trim(),
    })
}

#[cfg(test)]
mod tests {
    use super::{Delimiter, Opener, delimiter};

    fn found(mark: char, run: usize, info: &str) -> Delimiter<'_> {
        Delimiter { mark, run, info }
    }

    #[test]
    fn a_backtick_or_tilde_run_of_three_or_more_is_a_delimiter() {
        assert_eq!(delimiter("```"), Some(found('`', 3, "")));
        assert_eq!(delimiter("~~~"), Some(found('~', 3, "")));
        assert_eq!(
            delimiter("`````rust,no_run  "),
            Some(found('`', 5, "rust,no_run"))
        );
        assert_eq!(delimiter("~~~~ ignore"), Some(found('~', 4, "ignore")));
    }

    #[test]
    fn a_run_of_two_is_not_a_delimiter() {
        assert_eq!(delimiter("``"), None);
        assert_eq!(delimiter("~~ text"), None);
        assert_eq!(delimiter("prose with ``` inside"), None);
    }

    #[test]
    fn up_to_three_spaces_of_indentation_is_still_a_delimiter() {
        assert_eq!(delimiter(" ```"), Some(found('`', 3, "")));
        assert_eq!(delimiter("  ~~~ignore"), Some(found('~', 3, "ignore")));
        assert_eq!(
            delimiter("   ```compile_fail"),
            Some(found('`', 3, "compile_fail"))
        );
    }

    /// The renderer's old copy used `trim_start`, which accepted all three of
    /// these.
    #[test]
    fn four_spaces_or_a_tab_of_indentation_is_code_not_a_delimiter() {
        assert_eq!(delimiter("    ```"), None);
        assert_eq!(delimiter("        ~~~ignore"), None);
        assert_eq!(delimiter("\t```"), None);
    }

    /// Every copy of the rule accepted this before #294.
    #[test]
    fn a_backtick_info_string_with_a_backtick_is_not_a_delimiter() {
        assert_eq!(delimiter("```x``` opens an inline code span"), None);
        assert_eq!(delimiter("```rust `quoted`"), None);
        assert_eq!(delimiter("   ```` ` ````"), None);
    }

    #[test]
    fn an_opener_remembers_its_mark_and_run() {
        let found = delimiter("~~~~ text").map(|found| found.opener());
        assert_eq!(found, Some(Opener { mark: '~', run: 4 }));
    }

    #[test]
    fn a_tilde_info_string_may_carry_a_backtick() {
        assert_eq!(delimiter("~~~ `x`"), Some(found('~', 3, "`x`")));
    }

    #[test]
    fn a_fence_closes_on_its_own_mark_at_least_as_long_with_nothing_after() {
        let opener = Opener { mark: '`', run: 4 };
        let closes = |line| delimiter(line).is_some_and(|found| found.closes(opener));
        assert!(closes("````"));
        assert!(closes("  `````  "));
        assert!(!closes("```"), "shorter than the opener");
        assert!(!closes("~~~~"), "the other mark");
        assert!(!closes("```` text"), "something after the run");
    }
}
