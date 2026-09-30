//! The text transforms that turn a `docs/` page into a site page. Pure, and so
//! tested here rather than through a Zola build.
//!
//! **What changes on the way, and what does not.** A page's prose and its code
//! reach the site as written. Four things are changed, each for a stated reason:
//! the `# Title` and the `> **Answers:**` line become front matter, because the
//! site renders them as the page head; links are re-aimed, because a relative
//! path that resolves inside the repository resolves nowhere on the site; a
//! Rust fence's info string is reduced to `rust`, because rustdoc's attributes
//! mean nothing to the highlighter; and rustdoc's hidden lines are dropped,
//! exactly as rustdoc drops them, because the reader of either surface should
//! see the same program.

use std::borrow::Cow;

use anyhow::{Context, Result, bail};

use crate::lint_narrative::TREE;
use crate::lint_pages::declarations;

/// Where a link that leaves the narrative tree is sent: the file on `main`.
///
/// `main` rather than a tag, because the site is rebuilt from `main` and a
/// link to the release tag would describe a tree the page no longer sits in.
pub(crate) const BLOB: &str = "https://github.com/Wet-Ink-Corporation/happenstance/blob/main";

/// The tree's index. The site does not render it — the guide section builds its
/// own index from the pages' declared needs — so a link to it lands there.
const INDEX: &str = "README.md";

/// Info-string tokens rustdoc reads as attributes of a Rust block.
///
/// A fence whose first token is one of these (or `rust`, or nothing) is Rust to
/// rustdoc, so it is Rust here: its hidden lines are dropped and its info
/// string is reduced to `rust` for the highlighter.
const RUSTDOC_ATTRIBUTES: &[&str] = &[
    "ignore",
    "no_run",
    "should_panic",
    "compile_fail",
    "test_harness",
    "standalone_crate",
    "edition2015",
    "edition2018",
    "edition2021",
    "edition2024",
];

/// A fenced block the scanner is inside, with the marker that closes it.
struct Open {
    marker: char,
    len: usize,
    rust: bool,
}

/// A fence marker line: its character, its length and the info string after it.
fn marker(line: &str) -> Option<(char, usize, &str)> {
    let trimmed = line.trim_start();
    let ch = trimmed.chars().next().filter(|c| *c == '`' || *c == '~')?;
    let after = trimmed.trim_start_matches(ch);
    let len = trimmed.len() - after.len();
    (len >= 3).then(|| (ch, len, after.trim()))
}

/// Whether an opening fence's info string makes it a Rust block to rustdoc.
///
/// rustdoc also reads a `{.class}` token as Rust-compatible; this does not,
/// because nothing under `docs/` writes one and the narrative lint refuses info
/// strings it does not recognise.
fn is_rust(info: &str) -> bool {
    match info.split(|c: char| c == ',' || c.is_whitespace()).next() {
        None | Some("" | "rust") => true,
        Some(first) => RUSTDOC_ATTRIBUTES.contains(&first) || first.starts_with("ignore-"),
    }
}

/// One line of a page, classified against the fences around it.
enum Line<'a> {
    /// Prose, outside every fence.
    Prose(&'a str),
    /// A fence opening: its marker character and length, which the closing line
    /// must match; its info string; and whether rustdoc reads it as Rust.
    Opens {
        line: &'a str,
        marker: char,
        len: usize,
        info: &'a str,
        rust: bool,
    },
    /// A line inside a fence.
    Inside { line: &'a str, rust: bool },
    /// The line that closes a fence.
    Closes(&'a str),
}

/// Classifies every line of `text`, in order, with its 0-based index.
fn classify(text: &str) -> Vec<(usize, Line<'_>)> {
    let mut open: Option<Open> = None;
    let mut out = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let classified = match (&open, marker(line)) {
            (None, Some((ch, len, info))) => {
                let rust = is_rust(info);
                open = Some(Open {
                    marker: ch,
                    len,
                    rust,
                });
                Line::Opens {
                    line,
                    marker: ch,
                    len,
                    info,
                    rust,
                }
            }
            (None, None) => Line::Prose(line),
            (Some(fence), Some((ch, len, info)))
                if ch == fence.marker && len >= fence.len && info.is_empty() =>
            {
                open = None;
                Line::Closes(line)
            }
            (Some(fence), _) => Line::Inside {
                line,
                rust: fence.rust,
            },
        };
        out.push((index, classified));
    }
    out
}

/// A line split into its leading whitespace and the rest.
///
/// A line that is whitespace throughout is all indentation, which is what the
/// fallback says.
fn split_indent(line: &str) -> (&str, &str) {
    line.find(|c: char| !c.is_whitespace())
        .and_then(|at| line.split_at_checked(at))
        .unwrap_or((line, ""))
}

/// A line of a Rust block as the reader sees it: `None` for a hidden line.
///
/// rustdoc hides a line whose content, after leading whitespace, is `#` alone or
/// starts `# `, and unescapes a leading `##` to `#`. Anything else — `#[attr]`,
/// `#!` — is code and stays.
fn strip_hidden(line: &str) -> Option<Cow<'_, str>> {
    let (indent, trimmed) = split_indent(line);
    if trimmed == "#" || trimmed.starts_with("# ") {
        return None;
    }
    match trimmed.strip_prefix("##") {
        Some(rest) => Some(Cow::Owned(format!("{indent}#{rest}"))),
        None => Some(Cow::Borrowed(line)),
    }
}

/// A page from `docs/`, as a Zola page: front matter, then the body.
///
/// `file` is the page's name inside the tree; `on_site` names every page the
/// site renders, so a link to one of them can become a checked `@/` link.
///
/// # Errors
///
/// When the page has no `# Title` line, when it does not carry exactly one
/// well-formed `> **Answers:**` line, or when a link on it resolves outside
/// the repository.
pub(crate) fn guide_page(
    file: &str,
    text: &str,
    weight: usize,
    on_site: &[&str],
) -> Result<String> {
    let source = format!("{TREE}/{file}");
    let declared: Vec<_> = declarations(text)
        .into_iter()
        .filter_map(|found| Some((found.line, found.token?, found.question?)))
        .collect();
    let [(declared_at, need, question)] = declared.as_slice() else {
        bail!(
            "{source}: expected exactly one well-formed `> **Answers:**` line, found {}",
            declared.len()
        );
    };
    let lines = classify(text);
    let (title_at, title) = lines
        .iter()
        .find_map(|(index, line)| match line {
            Line::Prose(prose) => prose.strip_prefix("# ").map(|title| (*index, title.trim())),
            _ => None,
        })
        .with_context(|| format!("{source}: no `# Title` line outside a fence"))?;

    let mut body = String::with_capacity(text.len());
    for (index, line) in &lines {
        if *index == title_at || *index + 1 == *declared_at {
            continue;
        }
        let rendered: Cow<'_, str> = match line {
            Line::Prose(prose) => Cow::Owned(
                rewrite_links(prose, on_site).with_context(|| format!("{source}:{}", index + 1))?,
            ),
            // Same marker, same length: the closing line still matches, and an
            // inner shorter fence stays inside the block.
            Line::Opens {
                rust: true,
                line,
                marker,
                len,
                ..
            } => Cow::Owned(format!(
                "{}{}rust",
                split_indent(line).0,
                marker.to_string().repeat(*len)
            )),
            Line::Inside { rust: true, line } => match strip_hidden(line) {
                Some(kept) => kept,
                None => continue,
            },
            Line::Opens { line, .. } | Line::Inside { line, .. } | Line::Closes(line) => {
                Cow::Borrowed(*line)
            }
        };
        body.push_str(&rendered);
        body.push('\n');
    }

    Ok(format!(
        "+++\ntitle = {}\ndescription = {}\nweight = {weight}\n\n[extra]\nneed = {}\nsource = {}\n+++\n{}",
        toml_string(title)?,
        toml_string(question)?,
        toml_string(need)?,
        toml_string(&source)?,
        body.trim_start_matches('\n')
    ))
}

/// One prose line with every link re-aimed for the site: inline links and
/// images (`](target)`) and reference definitions (`[label]: target`).
///
/// A link inside a code span is text, and is left as written. A code span is
/// delimited as `CommonMark` delimits it — by a backtick run of the same length
/// — but only within the line: a span that crosses a line break is not tracked,
/// and a `](` inside the second half of one would be re-aimed. Nothing under
/// `docs/` writes one.
///
/// # Errors
///
/// When a link's path climbs above the repository root.
pub(crate) fn rewrite_links(line: &str, on_site: &[&str]) -> Result<String> {
    match reference_definition(line, on_site)? {
        Some(defined) => Ok(defined),
        None => rewrite_inline(line, on_site),
    }
}

/// A backtick run that opened a code span not yet closed: its length, where the
/// output stood just after it, and the input that followed it.
struct OpenSpan<'a> {
    run: usize,
    written: usize,
    after: &'a str,
}

/// The inline links of one prose line, re-aimed.
///
/// A backtick run that nothing closes by the end of the line is literal text in
/// `CommonMark`, not an opener; so when the line ends with a span still open, the
/// output is cut back to just after that run and the rest is scanned again as
/// prose.
fn rewrite_inline(line: &str, on_site: &[&str]) -> Result<String> {
    let mut out = String::with_capacity(line.len());
    let mut open: Option<OpenSpan<'_>> = None;
    let mut rest = line;
    while let Some(at) = rest.find(['`', ']']) {
        let (before, from) = rest.split_at(at);
        out.push_str(before);
        let after_run = from.trim_start_matches('`');
        let run = from.len() - after_run.len();
        if run > 0 {
            out.push_str(&"`".repeat(run));
            open = match open {
                None => Some(OpenSpan {
                    run,
                    written: out.len(),
                    after: after_run,
                }),
                Some(span) if span.run == run => None,
                still => still,
            };
            rest = after_run;
            continue;
        }
        let target = open
            .is_none()
            .then(|| from.strip_prefix("]("))
            .flatten()
            .and_then(|after| after.find(')').map(|end| after.split_at(end)));
        if let Some((target, tail)) = target {
            out.push_str("](");
            out.push_str(&site_target(target, on_site)?);
            rest = tail;
            continue;
        }
        // Not a link: keep the bracket and move past it. `find` returned an
        // index into `rest`, so `from` holds at least the bracket; the `break`
        // is only the loop's own end should that ever stop being so.
        let mut chars = from.chars();
        let Some(bracket) = chars.next() else { break };
        out.push(bracket);
        rest = chars.as_str();
    }
    if let Some(span) = open {
        out.truncate(span.written);
        out.push_str(&rewrite_inline(span.after, on_site)?);
        return Ok(out);
    }
    out.push_str(rest);
    Ok(out)
}

/// A reference definition, `[label]: target …`, with its target re-aimed;
/// `None` when the line is not one. A footnote definition, `[^n]: …`, is not a
/// link and is left alone.
fn reference_definition(line: &str, on_site: &[&str]) -> Result<Option<String>> {
    let (indent, trimmed) = split_indent(line);
    let Some((label, after)) = trimmed
        .strip_prefix('[')
        .and_then(|inner| inner.split_once("]:"))
    else {
        return Ok(None);
    };
    if label.is_empty() || label.starts_with('^') {
        return Ok(None);
    }
    let (gap, spec) = split_indent(after);
    // No whitespace after the target means there is no title: all of it is
    // the target, which is what the fallback says.
    let (target, title) = spec
        .find(char::is_whitespace)
        .and_then(|at| spec.split_at_checked(at))
        .unwrap_or((spec, ""));
    if target.is_empty() {
        return Ok(None);
    }
    // `<target>` is the form that lets a target hold spaces; the brackets are
    // syntax, not path, so they come off for the re-aim and go back on after.
    let aimed = match target
        .strip_prefix('<')
        .and_then(|inner| inner.strip_suffix('>'))
    {
        Some(inner) => format!("<{}>", site_target(inner, on_site)?),
        None => site_target(target, on_site)?.into_owned(),
    };
    Ok(Some(format!("{indent}[{label}]:{gap}{aimed}{title}")))
}

/// Where one link target, written relative to `docs/`, points on the site.
fn site_target<'a>(target: &'a str, on_site: &[&str]) -> Result<Cow<'a, str>> {
    if target.starts_with('#') || target.contains("://") || target.starts_with("mailto:") {
        return Ok(Cow::Borrowed(target));
    }
    let (path, anchor) = match target.split_once('#') {
        Some((path, anchor)) => (path, format!("#{anchor}")),
        None => (target, String::new()),
    };
    let mut resolved: Vec<&str> = vec![TREE];
    for segment in path.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                resolved
                    .pop()
                    .with_context(|| format!("the link `{target}` climbs out of the repository"))?;
            }
            name => resolved.push(name),
        }
    }
    let trailing = if path.ends_with('/') { "/" } else { "" };
    let joined = format!("{}{trailing}", resolved.join("/"));
    let in_tree = match resolved.as_slice() {
        [tree, file] if *tree == TREE => Some(*file),
        _ => None,
    };
    Ok(Cow::Owned(match in_tree {
        Some(INDEX) => format!("@/guide/_index.md{anchor}"),
        Some(file) if on_site.contains(&file) => format!("@/guide/{file}{anchor}"),
        _ => format!("{BLOB}/{joined}{anchor}"),
    }))
}

/// The `index`-th Rust block of `text`, as the reader sees it, and the `text`
/// block that follows it when nothing but blank lines stands between them.
///
/// The site's landing page shows a program from the tutorial this way, so the
/// program on the landing page is the one the gate compiles and runs.
///
/// # Errors
///
/// When the page has fewer than `index + 1` Rust blocks.
pub(crate) fn rust_fence(text: &str, index: usize) -> Result<(String, Option<String>)> {
    let lines = classify(text);
    let mut seen = 0_usize;
    let mut position = lines.iter();
    while let Some((_, line)) = position.next() {
        if !matches!(line, Line::Opens { rust: true, .. }) {
            continue;
        }
        if seen < index {
            seen += 1;
            continue;
        }
        let mut code = String::new();
        for (_, inner) in position.by_ref() {
            match inner {
                Line::Inside { line, .. } => {
                    if let Some(kept) = strip_hidden(line) {
                        code.push_str(&kept);
                        code.push('\n');
                    }
                }
                _ => break,
            }
        }
        let output = following_text(position.as_slice());
        return Ok((code, output));
    }
    bail!(
        "the page has no Rust fence number {}, only {seen}",
        index + 1
    )
}

/// The body of a `text` fence at the head of `rest`, past blank lines.
fn following_text(rest: &[(usize, Line<'_>)]) -> Option<String> {
    let mut lines = rest
        .iter()
        .skip_while(|(_, line)| matches!(line, Line::Prose(prose) if prose.trim().is_empty()));
    match lines.next() {
        Some((_, Line::Opens { info, .. })) if *info == "text" => {}
        _ => return None,
    }
    let mut body = String::new();
    for (_, line) in lines {
        match line {
            Line::Inside { line, .. } => {
                body.push_str(line);
                body.push('\n');
            }
            _ => break,
        }
    }
    Some(body)
}

/// `s` as a TOML basic string, quotes included.
///
/// # Errors
///
/// When `s` holds a control character other than a newline or a tab. Nothing
/// this module writes — a page title, a question, a version — has a reason to
/// carry one, so it is refused by code point rather than escaped into a value
/// nobody meant.
pub(crate) fn toml_string(s: &str) -> Result<String> {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => bail!("control character U+{:04X} in {s:?}", u32::from(c)),
            c => out.push(c),
        }
    }
    out.push('"');
    Ok(out)
}

/// The workspace's release version, read from `[workspace.package]`.
///
/// Line-based rather than a TOML parse, which would be a new dependency for
/// one key; the section and key are both fixed by the root manifest's shape.
///
/// # Errors
///
/// When the manifest has no `[workspace.package]` section or no `version` in it.
pub(crate) fn workspace_version(manifest: &str) -> Result<&str> {
    manifest
        .lines()
        .skip_while(|line| line.trim() != "[workspace.package]")
        .skip(1)
        .take_while(|line| !line.trim_start().starts_with('['))
        .find_map(|line| {
            let (key, value) = line.split_once('=')?;
            // A trailing `# comment` is TOML, not version; a version never
            // carries a `#`, so the first one ends the value.
            let value = value.split_once('#').map_or(value, |(value, _)| value);
            (key.trim() == "version").then(|| value.trim().trim_matches('"'))
        })
        .context("no `version` under `[workspace.package]` in the root manifest")
}

#[cfg(test)]
mod tests {
    use anyhow::{Context, Result};

    use super::{guide_page, rewrite_links, rust_fence, toml_string, workspace_version};

    const ON_SITE: &[&str] = &["first-encounter.md", "append-conditions.md"];

    const PAGE: &str = "# Appending under a condition\n\
        \n\
        > **Answers:** `explanation` — Why does a write re-read what it decided on?\n\
        \n\
        A decision and the write are two moments.\n";

    #[test]
    fn front_matter_carries_title_question_weight_need_and_source() -> Result<()> {
        let out = guide_page("append-conditions.md", PAGE, 3, ON_SITE)?;
        let (front, _) = out
            .strip_prefix("+++\n")
            .and_then(|rest| rest.split_once("+++\n"))
            .context("front matter is fenced by +++")?;
        assert!(
            front.contains("title = \"Appending under a condition\"\n"),
            "{front}"
        );
        assert!(
            front.contains("description = \"Why does a write re-read what it decided on?\"\n"),
            "{front}"
        );
        assert!(front.contains("weight = 3\n"), "{front}");
        assert!(front.contains("need = \"explanation\"\n"), "{front}");
        assert!(
            front.contains("source = \"docs/append-conditions.md\"\n"),
            "{front}"
        );
        Ok(())
    }

    #[test]
    fn the_title_and_the_declaration_leave_the_body() -> Result<()> {
        let out = guide_page("append-conditions.md", PAGE, 1, ON_SITE)?;
        let (_, body) = out.rsplit_once("+++\n").context("front matter closes")?;
        assert!(!body.contains("# Appending"), "{body}");
        assert!(!body.contains("**Answers:**"), "{body}");
        assert!(
            body.contains("A decision and the write are two moments."),
            "{body}"
        );
        Ok(())
    }

    #[test]
    fn a_page_with_no_declaration_is_refused_by_name() {
        let refused = guide_page("stray.md", "# Stray\n\nProse.\n", 1, ON_SITE);
        let message = refused
            .err()
            .map(|err| format!("{err:#}"))
            .unwrap_or_default();
        assert!(message.contains("docs/stray.md"), "{message}");
        assert!(message.contains("Answers"), "{message}");
    }

    #[test]
    fn a_page_with_no_title_is_refused_by_name() {
        let text = "> **Answers:** `how-to` — How?\n";
        let message = guide_page("bare.md", text, 1, ON_SITE)
            .err()
            .map(|err| format!("{err:#}"))
            .unwrap_or_default();
        assert!(message.contains("docs/bare.md"), "{message}");
    }

    #[test]
    fn links_are_rewritten_by_where_they_land() -> Result<()> {
        let cases = [
            (
                "[a](append-conditions.md)",
                "[a](@/guide/append-conditions.md)",
            ),
            (
                "[a](append-conditions.md#why)",
                "[a](@/guide/append-conditions.md#why)",
            ),
            ("[i](README.md)", "[i](@/guide/_index.md)"),
            (
                "[s](../spec/SPECIFICATION.md#es-8--ordering)",
                "[s](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/spec/SPECIFICATION.md#es-8--ordering)",
            ),
            (
                "[t](text-fences.md)",
                "[t](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/docs/text-fences.md)",
            ),
            (
                "[d](../standards/pages/)",
                "[d](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/standards/pages/)",
            ),
            ("[h](#step-two)", "[h](#step-two)"),
            ("[w](https://dcb.events/)", "[w](https://dcb.events/)"),
            (
                "see [a](append-conditions.md) and [b](first-encounter.md).",
                "see [a](@/guide/append-conditions.md) and [b](@/guide/first-encounter.md).",
            ),
        ];
        for (line, expected) in cases {
            assert_eq!(rewrite_links(line, ON_SITE)?, expected, "from {line}");
        }
        Ok(())
    }

    #[test]
    fn a_link_inside_inline_code_is_left_alone() -> Result<()> {
        let line = "write `[a](append-conditions.md)` to link";
        assert_eq!(rewrite_links(line, ON_SITE)?, line);
        Ok(())
    }

    #[test]
    fn a_link_that_climbs_out_of_the_repository_is_refused() {
        let message = rewrite_links("[x](../../elsewhere.md)", ON_SITE)
            .err()
            .map(|err| format!("{err:#}"))
            .unwrap_or_default();
        assert!(message.contains("../../elsewhere.md"), "{message}");
    }

    #[test]
    fn fences_are_not_rewritten_and_hidden_lines_are_dropped() -> Result<()> {
        let text = "# T\n\n> **Answers:** `tutorial` — How?\n\n\
            ```rust\n# use std::fmt;\nlet x = 1;\n## not hidden\n#\n```\n\n\
            ```text\n# kept\n[a](append-conditions.md)\n```\n";
        let out = guide_page("t.md", text, 1, ON_SITE)?;
        assert!(!out.contains("# use std::fmt;"), "{out}");
        assert!(out.contains("let x = 1;\n# not hidden\n```"), "{out}");
        assert!(out.contains("# kept\n[a](append-conditions.md)\n"), "{out}");
        Ok(())
    }

    #[test]
    fn rust_fence_returns_the_program_and_the_output_after_it() -> Result<()> {
        let text = "```rust\nfn a() {}\n```\n\nprose\n\n\
            ```rust\n# use x;\nfn b() {}\n```\n\n```text\nprinted\n```\n";
        let (code, output) = rust_fence(text, 1)?;
        assert_eq!(code, "fn b() {}\n");
        assert_eq!(output.as_deref(), Some("printed\n"));
        let (first, none) = rust_fence(text, 0)?;
        assert_eq!(first, "fn a() {}\n");
        assert_eq!(
            none, None,
            "prose sits between the first program and any text fence"
        );
        let message = rust_fence(text, 2)
            .err()
            .map(|err| format!("{err:#}"))
            .unwrap_or_default();
        assert!(
            message.contains("Rust fence number 3"),
            "there is no third program: {message}"
        );
        Ok(())
    }

    #[test]
    fn toml_strings_escape_quotes_and_backslashes() -> Result<()> {
        assert_eq!(toml_string(r#"a "b" \c"#)?, r#""a \"b\" \\c""#);
        Ok(())
    }

    #[test]
    fn a_control_character_is_refused_rather_than_escaped() {
        let message = toml_string("bell\u{7}")
            .err()
            .map(|err| format!("{err:#}"))
            .unwrap_or_default();
        assert!(message.contains("U+0007"), "{message}");
    }

    #[test]
    fn the_version_comes_from_workspace_package() -> Result<()> {
        let manifest = "[workspace]\nmembers = []\n\n[workspace.package]\n\
            edition = \"2024\"\nversion = \"0.3.2\"\n\n[workspace.dependencies]\n\
            version = \"9\"\n";
        assert_eq!(workspace_version(manifest)?, "0.3.2");
        let message = workspace_version("[package]\nversion = \"1\"\n")
            .err()
            .map(|err| format!("{err:#}"))
            .unwrap_or_default();
        assert!(message.contains("[workspace.package]"), "{message}");
        let commented = "[workspace.package]\nversion = \"0.4.0\" # the breaking window\n";
        assert_eq!(workspace_version(commented)?, "0.4.0");
        Ok(())
    }

    /// The body of a rendered page: everything after the front matter.
    fn body(file: &str, text: &str) -> Result<String> {
        let out = guide_page(file, text, 1, ON_SITE)?;
        let (_, body) = out.rsplit_once("+++\n").context("front matter closes")?;
        Ok(body.to_owned())
    }

    const HEAD: &str = "# T\n\n> **Answers:** `tutorial` — How?\n\n";

    #[test]
    fn a_rust_fence_keeps_its_marker_and_length() -> Result<()> {
        let tilde = body("t.md", &format!("{HEAD}~~~rust\nlet a = 1;\n~~~\n"))?;
        assert!(tilde.contains("~~~rust\nlet a = 1;\n~~~\n"), "{tilde}");
        let four = body(
            "t.md",
            &format!("{HEAD}````rust\n```\ninner\n```\n````\nafter\n"),
        )?;
        assert!(four.contains("````rust\n```\ninner\n```\n````\n"), "{four}");
        Ok(())
    }

    #[test]
    fn a_fence_rustdoc_reads_as_rust_drops_its_hidden_lines() -> Result<()> {
        for opener in [
            "```rust,no_run",
            "```ignore",
            "```",
            "```ignore-wasm32",
            "```standalone_crate",
        ] {
            let out = body("t.md", &format!("{HEAD}{opener}\n# hidden\nshown\n```\n"))?;
            assert!(!out.contains("# hidden"), "{opener}: {out}");
            assert!(out.contains("```rust\nshown\n```"), "{opener}: {out}");
        }
        let prose = body("t.md", &format!("{HEAD}```toml\n# kept\n```\n"))?;
        assert!(prose.contains("```toml\n# kept\n```"), "{prose}");
        Ok(())
    }

    #[test]
    fn code_spans_are_delimited_by_runs_of_equal_length() -> Result<()> {
        let double = "``[a](append-conditions.md) ` still code`` then [b](first-encounter.md)";
        assert_eq!(
            rewrite_links(double, ON_SITE)?,
            "``[a](append-conditions.md) ` still code`` then [b](@/guide/first-encounter.md)"
        );
        let code_text = "[`AppendCondition`](append-conditions.md)";
        assert_eq!(
            rewrite_links(code_text, ON_SITE)?,
            "[`AppendCondition`](@/guide/append-conditions.md)"
        );
        Ok(())
    }

    #[test]
    fn a_reference_definition_is_re_aimed() -> Result<()> {
        assert_eq!(
            rewrite_links("[why]: append-conditions.md#why", ON_SITE)?,
            "[why]: @/guide/append-conditions.md#why"
        );
        assert_eq!(
            rewrite_links("  [spec]: ../spec/SPECIFICATION.md", ON_SITE)?,
            format!("  [spec]: {}/spec/SPECIFICATION.md", super::BLOB)
        );
        Ok(())
    }

    /// An unclosed backtick run is literal in `CommonMark`, so a link after it
    /// on the same line is still a link.
    #[test]
    fn an_unclosed_backtick_run_does_not_hide_the_links_after_it() -> Result<()> {
        assert_eq!(
            rewrite_links("a ` stray, then [b](first-encounter.md)", ON_SITE)?,
            "a ` stray, then [b](@/guide/first-encounter.md)"
        );
        Ok(())
    }

    #[test]
    fn an_angle_bracket_reference_target_keeps_its_brackets() -> Result<()> {
        assert_eq!(
            rewrite_links("[why]: <append-conditions.md> \"Why\"", ON_SITE)?,
            "[why]: <@/guide/append-conditions.md> \"Why\""
        );
        Ok(())
    }

    #[test]
    fn only_a_text_fence_is_taken_as_output() -> Result<()> {
        let text = "```rust\nfn a() {}\n```\n\n```plaintext\nnot output\n```\n";
        let (_, output) = rust_fence(text, 0)?;
        assert_eq!(output, None);
        Ok(())
    }
}
