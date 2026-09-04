//! The workflow lint: what CI is allowed to do, and whose code it runs.
//!
//! `.github/workflows/` is the only file in this repository that executes with a
//! credential. Everything else the gate reads is *source*, and a wrong answer
//! there is a red build; a wrong answer here is a third party's code running
//! against this repository's `GITHUB_TOKEN` on every push, with nothing in the
//! gate having looked at it. The pre-publication review found exactly that, and
//! found it twice — once on 2026-08-06 and again on 2026-09-03, unmoved in 652
//! commits: no `permissions:` key at any level, so every job took the
//! repository-default token scope, and six distinct actions on refs their owner
//! can move under us without changing a byte of this file. One of them,
//! `dtolnay/rust-toolchain@master`, was a *branch*.
//!
//! # The two claims
//!
//! **Every workflow states its token scope.** A top-level `permissions:` block
//! is what turns the repository default off. Without one the workflow says
//! nothing about what it may do, which is indistinguishable from having decided
//! the default was right — and the default is a repository setting, changed
//! elsewhere, by someone who is not reading this file. The block must also grant
//! no `write` at the top level: a job that genuinely needs to write says so at
//! the job, where a reviewer reads the grant beside the steps that need it.
//!
//! **Every `uses:` names an immutable ref.** A 40-character commit SHA is the
//! only ref GitHub will not let its owner re-point. A tag is not one — `v4` is
//! moved on every release, so a compromised maintainer account re-points it at
//! whatever it likes and every workflow in the world picks that up on its next
//! run. The version therefore has to be written where a human reads it, so the
//! trailing `# v4.4.0` comment is required as well: a bare SHA is an upgrade
//! nobody can review.
//!
//! # What this does not verify
//!
//! **That the SHA exists in the named repository, or that the trailing comment
//! names the version that SHA actually is.** Both need the network, and a gate
//! step that reaches the network fails on an aeroplane and goes green when a
//! registry is down. The comment is prose and this check treats it as prose: it
//! must be present and non-empty, and nothing here notices `# v4.4.0` written
//! beside the SHA of `v2.0.0`. What the pairing does buy is that a change of pin
//! is a diff a reviewer can read — [`the_version_comment_is_prose`] asserts that
//! limit rather than leaving it as a promise.
//!
//! **A job-level `permissions:` block.** Raising the scope for one job is a
//! decision with a reason, and this check is not where that reason is argued; it
//! constrains the workflow-level default, which is what every job that never
//! mentions permissions inherits. [`a_job_level_grant_is_not_adjudicated`] holds
//! that boundary in place.
//!
//! **Anything inside a block scalar.** A `run: |` body is shell, not YAML, so a
//! line reading `uses: actions/checkout@v4` inside one is a string and is
//! skipped — which is right, and is also the shape of the blind spot: a
//! block-scalar indicator this scanner fails to recognise would swallow every
//! `uses:` below it and the check would print success over an unpinned
//! workflow. Two things stand against that. The scanner hard-errors on a line it
//! cannot lex rather than passing it, and [`run`] fails when the whole corpus
//! yields no `uses:` at all — a check that has lost its subject is the quietest
//! failure available, so it is made loud here.

use std::fs;

use anyhow::{Context, Result, bail};

use crate::spec_trace::workspace_root;

/// The directory this check is scoped to.
///
/// Every workflow in it, rather than `ci.yml` by name. The defect this rejects
/// arrives as a *new* file — a release workflow, a docs deploy — and a check
/// naming one file is green over every workflow but that one.
const WORKFLOW_DIR: &str = ".github/workflows";

/// This step, as `REQUIRED` holds it.
///
/// The whole row rather than a name, which is a departure from the two `STEP`
/// constants beside it and is worth its sentence. `xtask/src/main.rs` carries
/// twenty anchored citations from `standards/rust/`, and `lint-constitution`
/// measures a citation's staleness in lines: a fifteen-line row inserted into
/// `REQUIRED` moves seven of them past the slack, and the repair would be an
/// edit to the constitution rather than to the gate. One line does not. The
/// argument for the step belongs in this module's own documentation in any
/// case — that is where a reader looking for what the check does not verify
/// will go.
pub(crate) const STEP: crate::Step = crate::Step {
    name: "every workflow pins its actions and its token scope",
    program: "cargo",
    args: &[
        "run",
        "--locked",
        "--quiet",
        "-p",
        "xtask",
        "--",
        "lint-workflows",
    ],
    env: &[],
    probe: None,
};

/// One line of a workflow, outside any block scalar.
///
/// The comment is carried rather than discarded because half of what this
/// module checks *is* the comment: a pinned SHA with no version beside it
/// satisfies the immutability claim and defeats the review claim.
#[derive(Debug)]
struct Line {
    /// 1-based, so a failure message is a line a reader can jump to.
    number: usize,
    /// Leading spaces. YAML forbids tabs in indentation, and so does the
    /// scanner: a tab is a construct it cannot measure, not one to guess at.
    indent: usize,
    /// The line with its comment removed, trailing whitespace trimmed.
    code: String,
    /// The comment text, trimmed, when the line carried one.
    comment: Option<String>,
}

/// Check every workflow's token scope and every action reference it makes.
///
/// # Errors
///
/// When `.github/workflows/` cannot be read, when it holds no workflow, when a
/// line cannot be lexed, when no `uses:` is found anywhere in the corpus, or
/// when any workflow fails either claim above.
pub(crate) fn run() -> Result<()> {
    let root = workspace_root()?;
    let dir = root.join(WORKFLOW_DIR);

    let mut files: Vec<_> = fs::read_dir(&dir)
        .with_context(|| format!("reading {WORKFLOW_DIR}"))?
        .collect::<std::io::Result<Vec<_>>>()
        .with_context(|| format!("reading {WORKFLOW_DIR}"))?
        .into_iter()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "yml" || extension == "yaml")
        })
        .collect();
    files.sort();

    // Anti-vacuity, first. A directory that has been renamed, or a glob that
    // stops matching, would otherwise make this step print success having read
    // nothing — and it would keep printing it, because nothing else in the gate
    // opens these files.
    if files.is_empty() {
        bail!(
            "lint-workflows: {WORKFLOW_DIR} holds no `.yml` or `.yaml` file. Either CI has \
             moved and this check is pointed at nothing, or it has been deleted; both are \
             failures rather than a step with no work to do."
        );
    }

    let mut problems = Vec::new();
    let mut uses_seen = 0usize;

    for path in &files {
        let name = path.file_name().map_or_else(
            || path.display().to_string(),
            |n| n.to_string_lossy().into(),
        );
        let at = format!("{WORKFLOW_DIR}/{name}");
        let text = fs::read_to_string(path).with_context(|| format!("reading {at}"))?;
        let lines = lex(&at, &text)?;

        uses_seen += lines
            .iter()
            .filter(|line| uses_value(line).is_some())
            .count();
        problems.extend(token_scope(&at, &lines));
        problems.extend(pinned_actions(&at, &lines));
    }

    // The second half of the anti-vacuity guard, and the half that matters: the
    // files were found and read, and the scanner reported no action reference in
    // any of them. That is not a workflow corpus this repository has ever had,
    // so it is the scanner that has stopped seeing rather than the corpus that
    // has stopped using actions.
    if uses_seen == 0 {
        bail!(
            "lint-workflows: read {} workflow(s) and found no `uses:` anywhere. This scanner \
             skips block scalars, so an indicator it mis-reads swallows every line below it — \
             a check that has lost its subject reports success, which is why this is a \
             failure instead.",
            files.len()
        );
    }

    if !problems.is_empty() {
        bail!(
            "lint-workflows: {} problem(s). CI is the one artefact here that runs with a \
             credential, and both claims below are about what a compromised third party \
             reaches:\n\n{}",
            problems.len(),
            problems.join("\n")
        );
    }

    println!(
        "lint-workflows: {} workflow(s), {uses_seen} action reference(s) — every one pinned to a \
         commit SHA with its version named, every workflow stating its token scope",
        files.len()
    );
    Ok(())
}

/// Split a line into its code and its comment, or fail naming what could not be
/// lexed.
///
/// A `#` inside a quoted scalar is not a comment, and a quote left open is the
/// construct that would make every line after it read as one long string. Both
/// halves of that are the reason this returns a `Result` rather than doing its
/// best: a scanner that silently stops reporting prints its success line and
/// exits 0.
fn split_comment<'a>(at: &str, number: usize, raw: &'a str) -> Result<(&'a str, Option<&'a str>)> {
    let bytes = raw.as_bytes();
    let mut quote: Option<u8> = None;
    let mut index = 0;

    while index < bytes.len() {
        let byte = bytes[index];
        match quote {
            Some(open) => {
                if open == b'"' && byte == b'\\' {
                    index += 2;
                    continue;
                }
                if byte == open {
                    // YAML escapes a single quote by doubling it, and a
                    // scanner that reads the pair as a close-then-open is a
                    // scanner whose idea of "inside a string" is inverted for
                    // the rest of the line.
                    if open == b'\'' && bytes.get(index + 1) == Some(&b'\'') {
                        index += 2;
                        continue;
                    }
                    quote = None;
                }
            }
            None => {
                if byte == b'"' || byte == b'\'' {
                    quote = Some(byte);
                } else if byte == b'#' && (index == 0 || bytes[index - 1].is_ascii_whitespace()) {
                    return Ok((&raw[..index], Some(raw[index + 1..].trim())));
                }
            }
        }
        index += 1;
    }

    if quote.is_some() {
        bail!(
            "{at}:{number} — a quoted scalar is still open at end of line. This check is not a \
             YAML parser, and a line it cannot lex is a hard error rather than a line it \
             quietly reports clean."
        );
    }

    Ok((raw, None))
}

/// The block-scalar indicator a line opens, if it opens one.
///
/// `run: |`, `if: >-`, `script: |2` — the value is the indicator and nothing
/// else. Everything more-indented below such a line is data: shell, a jq
/// program, a Markdown body. Reading it as YAML is how a lint fires on the
/// sentence that explains it.
fn block_scalar_indicator(code: &str) -> Option<&str> {
    let (_, value) = code.split_once(':')?;
    let value = value.trim();
    let mut chars = value.chars();
    if !matches!(chars.next()?, '|' | '>') {
        return None;
    }
    chars
        .all(|c| matches!(c, '-' | '+' | '0'..='9'))
        .then_some(value)
}

/// Every line of a workflow that is YAML rather than the contents of a block
/// scalar.
fn lex(at: &str, text: &str) -> Result<Vec<Line>> {
    let mut lines = Vec::new();
    let mut block: Option<usize> = None;

    for (index, raw) in text.lines().enumerate() {
        let number = index + 1;
        let trimmed = raw.trim_start();
        let indent = raw.len() - trimmed.len();

        if let Some(opened_at) = block {
            if trimmed.is_empty() || indent > opened_at {
                continue;
            }
            block = None;
        }
        if trimmed.is_empty() {
            continue;
        }
        if raw[..indent].contains('\t') {
            bail!(
                "{at}:{number} — a tab in the indentation. YAML forbids it and this scanner \
                 cannot measure it, so it is a hard error rather than a guess at a width."
            );
        }

        let (code, comment) = split_comment(at, number, raw)?;
        let code = code.trim_end();

        if block_scalar_indicator(code).is_some() {
            block = Some(indent);
        }

        lines.push(Line {
            number,
            indent,
            code: code.to_owned(),
            comment: comment.map(str::to_owned),
        });
    }

    Ok(lines)
}

/// The value of a `uses:` key, if the line is one.
///
/// Both spellings, because a step is a sequence item and a reusable workflow
/// call is not: `- uses: …` and `uses: …`. Surrounding quotes are stripped, so
/// `uses: "actions/checkout@…"` is the same reference as the bare one.
fn uses_value(line: &Line) -> Option<&str> {
    let code = line.code.trim_start();
    let code = code.strip_prefix("- ").unwrap_or(code).trim_start();
    let value = code.strip_prefix("uses:")?.trim();
    let value = value
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .or_else(|| {
            value
                .strip_prefix('\'')
                .and_then(|rest| rest.strip_suffix('\''))
        })
        .unwrap_or(value);
    Some(value)
}

/// Whether a ref is a 40-character lowercase commit SHA.
///
/// Lowercase deliberately: GitHub renders them lowercase everywhere, and a
/// mixed-case pin is a hand-typed one — which is the pin most likely to be a
/// transcription error nothing else in this repository would catch.
fn is_commit_sha(reference: &str) -> bool {
    reference.len() == 40
        && reference
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}

/// Claim one: the workflow states its token scope, and states it read-only.
fn token_scope(at: &str, lines: &[Line]) -> Vec<String> {
    let mut problems = Vec::new();

    let Some(position) = lines
        .iter()
        .position(|line| line.indent == 0 && line.code.starts_with("permissions:"))
    else {
        problems.push(format!(
            "{at} — no top-level `permissions:` block, so every job runs with whatever the \
             repository default is. That default is a setting changed elsewhere by someone \
             who is not reading this file; `permissions: contents: read` here says what CI \
             may do, in CI."
        ));
        return problems;
    };

    // The block is the key's own inline value plus every more-indented line
    // under it — `permissions: read-all` and a nested mapping are the same
    // claim written two ways, and only reading both catches the first.
    let mut scope = lines[position].code.clone();
    for line in &lines[position + 1..] {
        if line.indent == 0 {
            break;
        }
        scope.push(' ');
        scope.push_str(line.code.trim());
    }

    if scope.contains("write") {
        problems.push(format!(
            "{at}:{} — the top-level `permissions:` block grants write (`{}`). A workflow-wide \
             write scope is inherited by every job, including the ones that only read; raise it \
             on the job that needs it, where a reviewer reads the grant beside the steps that \
             use it.",
            lines[position].number,
            scope.trim()
        ));
    }

    problems
}

/// Claim two: every action reference is immutable, and says which version it is.
fn pinned_actions(at: &str, lines: &[Line]) -> Vec<String> {
    let mut problems = Vec::new();

    for line in lines {
        let Some(value) = uses_value(line) else {
            continue;
        };
        let here = format!("{at}:{}", line.number);

        // A path into this repository is already frozen: it is checked out at
        // the same commit as the workflow that calls it.
        if value.starts_with("./") || value.starts_with(".\\") {
            continue;
        }

        if let Some(image) = value.strip_prefix("docker://") {
            if !image.contains("@sha256:") {
                problems.push(format!(
                    "{here} — `{value}` names a container by tag. A registry tag moves exactly \
                     the way a git tag does; pin the `@sha256:` digest."
                ));
            }
            continue;
        }

        let Some((action, reference)) = value.rsplit_once('@') else {
            problems.push(format!(
                "{here} — `uses: {value}` names no ref at all, so it resolves to the default \
                 branch and this workflow runs whatever was pushed there last."
            ));
            continue;
        };

        if !is_commit_sha(reference) {
            problems.push(format!(
                "{here} — `uses: {action}@{reference}` is a mutable ref. A tag and a branch are \
                 both re-pointed by whoever owns the repository, so this line runs code nobody \
                 here has read; pin the 40-character commit SHA and name the version in a \
                 trailing comment."
            ));
            continue;
        }

        if line.comment.as_deref().is_none_or(str::is_empty) {
            problems.push(format!(
                "{here} — `{action}` is pinned to a commit SHA with no version beside it. The \
                 pin is what makes the reference immutable and the comment is what makes \
                 raising it reviewable: write `# v4.4.0` after the ref."
            ));
        }
    }

    problems
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    /// A workflow that satisfies both claims, for the cases below to mutate.
    const PINNED: &str = "\
name: CI
permissions:
  contents: read
jobs:
  gate:
    steps:
      - uses: actions/checkout@11d5960a326750d5838078e36cf38b85af677262 # v4.4.0
";

    fn problems(text: &str) -> Vec<String> {
        let lines = lex("w.yml", text).unwrap();
        let mut found = token_scope("w.yml", &lines);
        found.extend(pinned_actions("w.yml", &lines));
        found
    }

    #[test]
    fn a_pinned_workflow_that_states_its_scope_is_clean() {
        assert!(problems(PINNED).is_empty(), "{:?}", problems(PINNED));
    }

    /// The defect as it stood at `56ef6c5`, both halves of it.
    #[test]
    fn the_workflow_this_check_was_written_against_fails_both_claims() {
        let found = problems(
            "\
name: CI
jobs:
  gate:
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
",
        );

        assert_eq!(found.len(), 3, "{found:#?}");
        assert!(
            found[0].contains("no top-level `permissions:` block"),
            "{}",
            found[0]
        );
        assert!(found[1].contains("actions/checkout@v4"), "{}", found[1]);
        assert!(
            found[2].contains("dtolnay/rust-toolchain@master"),
            "{}",
            found[2]
        );
    }

    #[test]
    fn a_sha_with_no_version_comment_is_not_a_reviewable_pin() {
        let found = problems(&PINNED.replace(" # v4.4.0", ""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("no version beside it"), "{}", found[0]);
    }

    #[test]
    fn a_ref_that_is_not_a_sha_is_rejected_however_it_is_spelled() {
        // Quoted, and a branch rather than a tag. Both are the same defect and
        // a check that only knows the bare-tag spelling is green over both.
        let found = problems(
            "\
permissions: {}
jobs:
  j:
    steps:
      - uses: \"owner/action@main\" # main
",
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("mutable ref"), "{}", found[0]);
    }

    #[test]
    fn a_uses_with_no_ref_at_all_is_rejected() {
        let found = problems(
            "\
permissions: {}
jobs:
  j:
    steps:
      - uses: owner/action
",
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("names no ref at all"), "{}", found[0]);
    }

    /// A workflow-wide write scope, in both spellings.
    #[test]
    fn a_top_level_write_scope_is_rejected() {
        let inline =
            problems(&PINNED.replace("permissions:\n  contents: read", "permissions: write-all"));
        assert_eq!(inline.len(), 1, "{inline:#?}");
        assert!(inline[0].contains("grants write"), "{}", inline[0]);

        let nested = problems(&PINNED.replace("contents: read", "contents: write"));
        assert_eq!(nested.len(), 1, "{nested:#?}");
        assert!(nested[0].contains("grants write"), "{}", nested[0]);
    }

    /// The documented limit, asserted rather than promised: a job may raise its
    /// own scope and this check does not adjudicate it.
    #[test]
    fn a_job_level_grant_is_not_adjudicated() {
        let text = PINNED.replace(
            "  gate:\n",
            "  gate:\n    permissions:\n      id-token: write\n",
        );
        assert!(problems(&text).is_empty(), "{:?}", problems(&text));
    }

    /// The other documented limit: the comment is prose and is never compared
    /// to the SHA beside it.
    #[test]
    fn the_version_comment_is_prose() {
        let text = PINNED.replace("# v4.4.0", "# v99.99.99, which this SHA is not");
        assert!(problems(&text).is_empty(), "{:?}", problems(&text));
    }

    /// A `uses:` inside a `run: |` body is shell, and a `#` inside a quoted
    /// scalar is not a comment. Both directions, because the scanner that gets
    /// the first right by blanking everything gets the second wrong.
    #[test]
    fn a_block_scalar_body_is_data_and_a_quoted_hash_is_not_a_comment() {
        let text = "\
permissions: {}
jobs:
  j:
    steps:
      - name: 'a # inside a scalar'
        run: |
          uses: actions/checkout@v4
          echo '# not a comment either'
      - uses: owner/action@11d5960a326750d5838078e36cf38b85af677262 # v1.2.3
";
        assert!(problems(text).is_empty(), "{:?}", problems(text));

        // And the block ends where the indentation does: the real `uses:`
        // below it was seen.
        let lines = lex("w.yml", text).unwrap();
        assert_eq!(
            lines.iter().filter(|l| uses_value(l).is_some()).count(),
            1,
            "the scanner must see the step's `uses:` and not the shell's"
        );
    }

    /// A folded scalar opened by `if: >-`, which `ci.yml` actually uses.
    #[test]
    fn a_folded_scalar_is_a_block_scalar_too() {
        assert_eq!(block_scalar_indicator("if: >-"), Some(">-"));
        assert_eq!(block_scalar_indicator("run: |"), Some("|"));
        assert_eq!(block_scalar_indicator("script: |2"), Some("|2"));
        assert_eq!(block_scalar_indicator("uses: owner/action@v1"), None);
        assert_eq!(block_scalar_indicator("group: ${{ github.ref }}"), None);
    }

    /// The construct that would invert the scanner's idea of "inside a string"
    /// for every line after it. It is a hard error, not a best effort.
    #[test]
    fn a_line_that_cannot_be_lexed_is_a_hard_error() {
        let err = lex("w.yml", "run: 'unterminated\n")
            .unwrap_err()
            .to_string();
        assert!(err.contains("still open at end of line"), "{err}");

        let tabbed = lex("w.yml", "jobs:\n\tj:\n").unwrap_err().to_string();
        assert!(tabbed.contains("tab in the indentation"), "{tabbed}");
    }

    /// A doubled single quote closes nothing, which is the case a naive toggle
    /// gets backwards.
    #[test]
    fn a_doubled_single_quote_is_an_escape() {
        let (code, comment) = split_comment("w.yml", 1, "name: 'it''s fine' # v1").unwrap();
        assert_eq!(code.trim_end(), "name: 'it''s fine'");
        assert_eq!(comment, Some("v1"));
    }

    #[test]
    fn a_sha_is_forty_lowercase_hex_and_nothing_else() {
        assert!(is_commit_sha("11d5960a326750d5838078e36cf38b85af677262"));
        assert!(!is_commit_sha("11D5960A326750D5838078E36CF38B85AF677262"));
        assert!(!is_commit_sha("11d5960a326750d5838078e36cf38b85af6772"));
        assert!(!is_commit_sha("v4.4.0"));
        assert!(!is_commit_sha("11d5960a326750d5838078e36cf38b85af67726g"));
    }

    /// A local action is checked out at this commit, so it is already frozen.
    #[test]
    fn a_local_action_needs_no_pin() {
        let text = "\
permissions: {}
jobs:
  j:
    steps:
      - uses: ./.github/actions/setup
";
        assert!(problems(text).is_empty(), "{:?}", problems(text));
    }

    /// A container image by tag is the same defect one registry over.
    #[test]
    fn a_container_action_needs_its_digest() {
        let found = problems(
            "\
permissions: {}
jobs:
  j:
    steps:
      - uses: docker://alpine:3.20
",
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("@sha256:"), "{}", found[0]);
    }
}
