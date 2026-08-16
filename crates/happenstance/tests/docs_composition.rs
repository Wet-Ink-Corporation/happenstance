//! The rendered command-loop surface, read off the crate's own source.
//!
//! A doc comment that is technically accurate and unreadable — correct
//! signatures, no policy, no links, a bullet still marked *planned* — passes
//! every other test in this crate and fails these. No compiler runs here.

use std::path::{Path, PathBuf};

/// Code inside a doc fence, in columns.
const FENCE_COLUMNS: usize = 72;
/// The first sentence of any doc comment, in characters.
const FIRST_SENTENCE: usize = 80;
/// A public identifier, in characters.
const IDENTIFIER: usize = 24;

fn src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// One line ending, whatever the checkout used.
///
/// Every window below is cut with `\n`: `\n\n` for the attribute block that
/// belongs to *one* item, `\n}\n` for the end of an enum. This repository is
/// cloned with `core.autocrlf=true`, so on Windows those separators are `\r\n`
/// and neither cut lands. The loud half is a panic — `the enum is terminated`.
/// The quiet half is worse: `rsplit("\n\n")` finds no separator, yields the
/// **whole file above the item**, and every `contains` riding on it then passes
/// on an attribute written anywhere else in the module. Normalising here is
/// what keeps the windows narrow, on every platform.
fn normalise(source: &str) -> String {
    source.replace("\r\n", "\n")
}

fn read(name: &str) -> String {
    normalise(
        &std::fs::read_to_string(src().join(name)).expect("the crate's own source is readable"),
    )
}

/// The `//!` body of the crate root, line by line.
fn module_doc() -> Vec<String> {
    read("lib.rs")
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            trimmed
                .strip_prefix("//!")
                .map(|rest| rest.strip_prefix(' ').unwrap_or(rest).to_owned())
        })
        .collect()
}

/// The `///` block immediately above the line containing `declaration`.
fn doc_block_above(source: &str, declaration: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let at = lines
        .iter()
        .position(|line| line.trim_start().starts_with(declaration))
        .unwrap_or_else(|| panic!("`{declaration}` is not declared in that file"));

    let mut block = Vec::new();
    for line in lines[..at].iter().rev() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("///") {
            block.push(rest.strip_prefix(' ').unwrap_or(rest).to_owned());
            continue;
        }
        if trimmed.starts_with("#[") || trimmed.starts_with("//") {
            continue;
        }
        break;
    }
    block.reverse();
    block.join("\n")
}

// ---------------------------------------------------------------------------
// AC-009 — the vocabulary bullet became the function, in place
// ---------------------------------------------------------------------------

#[test]
fn roadmap_bullet_became_a_link() {
    let doc = module_doc();
    let vocabulary = doc
        .iter()
        .position(|line| line.contains("# The vocabulary"))
        .expect("region 4 exists");

    let bullets: Vec<&String> = doc
        .iter()
        .enumerate()
        .filter(|(at, line)| *at > vocabulary && line.starts_with("* "))
        .map(|(_, line)| line)
        .collect();

    let loop_bullet = bullets
        .iter()
        .find(|line| line.contains("command loop"))
        .expect("the vocabulary still names the command loop");

    // The link *is* the emphasis: the bolded lead-in term is the link text.
    assert!(
        loop_bullet.contains("[**The command loop**]"),
        "the command-loop bullet is not a link: {loop_bullet}"
    );

    // And it resolves to `commit` in the build that has one. A reference-style
    // link, because a plain `](commit)` is a hard rustdoc error in every build
    // without `json` — RS-70-2, and the failure the design's mock caught.
    let root = read("lib.rs");
    assert!(
        root.contains(r#"doc = "[command-loop]: commit""#),
        "the command-loop reference does not resolve to `commit`"
    );
    assert!(
        root.contains(r#"doc = "[command-loop]: commit_with""#),
        "the command-loop reference resolves to nothing when `json` is off"
    );

    // In place: it is still the fourth bullet, in the order M2 left it.
    let order: Vec<&str> = bullets
        .iter()
        .filter_map(|line| {
            [
                "`Codec`",
                "`DomainEvent`",
                "`DecisionModel`",
                "command loop",
                "projection runner",
            ]
            .into_iter()
            .find(|name| line.contains(name))
        })
        .collect();
    assert_eq!(
        order,
        [
            "`Codec`",
            "`DomainEvent`",
            "`DecisionModel`",
            "command loop",
            "projection runner"
        ],
        "the vocabulary was restructured rather than rewritten in place"
    );

    // Region 2 still leads, and region 7 is still last.
    let fence = doc
        .iter()
        .position(|line| line.starts_with("```"))
        .expect("region 2 exists");
    let adapters = doc
        .iter()
        .position(|line| line.contains("Adapter authors should depend on"))
        .expect("region 7 exists");
    assert!(fence < vocabulary, "the first program was displaced");
    assert!(adapters > vocabulary, "the adapter pointer was displaced");
}

#[test]
fn no_planned_heading_survives() {
    let doc = module_doc();
    assert!(
        !doc.iter()
            .any(|line| line.starts_with('#') && line.to_lowercase().contains("planned")),
        "the page carries a `Planned` heading"
    );

    let loop_bullet = doc
        .iter()
        .find(|line| line.contains("command loop"))
        .expect("the vocabulary still names the command loop");
    assert!(
        !loop_bullet.to_lowercase().contains("planned"),
        "the command loop is still advertised as planned: {loop_bullet}"
    );
}

#[test]
fn landing_page_names_only_the_persistent_four() {
    let doc = module_doc();
    let bullets: Vec<&String> = doc.iter().filter(|line| line.starts_with("* ")).collect();

    for revealed in ["Committed", "CommandError", "Boundary"] {
        assert!(
            !bullets.iter().any(|line| line.contains(revealed)),
            "`{revealed}` was hoisted onto the landing page; it is revealed from \
             `commit`'s signature, not chrome"
        );
    }
}

#[test]
fn no_item_shadows_a_core_name() {
    let root = read("lib.rs");
    assert!(
        root.contains("pub use happenstance_core::*;"),
        "the contract crate's glob re-export must survive"
    );

    for shadowed in ["Query", "EventStore", "Tags", "Event", "AppendError"] {
        for declaration in [
            "pub trait ",
            "pub enum ",
            "pub struct ",
            "pub fn ",
            "pub async fn ",
        ] {
            let banned = format!("{declaration}{shadowed}");
            let source = read("command.rs");
            assert!(
                !source.contains(&banned),
                "command.rs declares `{banned}`, which shadows a contract name"
            );
        }
    }
}

#[test]
fn retry_policy_is_on_commit_itself() {
    let source = read("command.rs");
    let block = doc_block_above(&source, "pub async fn commit<");

    for required in ["re-read", "verbatim", "lost update", "# Errors"] {
        assert!(
            block.contains(required),
            "`commit`'s own page does not state `{required}`"
        );
    }
    // RS-70-5: the alternative that lost, named once, where the reader is.
    assert!(
        block.contains("The alternative that lost"),
        "`commit` does not name the alternative that lost"
    );
    for deferral in ["see the specification", "see `spec/", "see SPECIFICATION"] {
        assert!(
            !block.contains(deferral),
            "`commit` defers its policy to another document: `{deferral}`"
        );
    }
}

// ---------------------------------------------------------------------------
// AC-003, AC-007 — the value and the vocabulary, read off the source
// ---------------------------------------------------------------------------

#[test]
fn committed_is_non_exhaustive_and_must_use() {
    let source = read("command.rs");
    let at = source
        .find("pub struct Committed")
        .expect("`Committed` is declared");
    let attributes = &source[..at];

    assert!(
        attributes.ends_with("#[non_exhaustive]\n")
            || attributes.contains("#[non_exhaustive]\npub struct Committed")
            || attributes
                .rsplit("\n\n")
                .next()
                .is_some_and(|near| near.contains("#[non_exhaustive]")),
        "`Committed` is not `#[non_exhaustive]`, so a later field is a breaking change"
    );
    assert!(
        attributes
            .rsplit("\n\n")
            .next()
            .is_some_and(|near| near.contains("#[must_use")),
        "`Committed` is not `#[must_use]`, so a dropped outcome compiles quietly"
    );
}

#[test]
fn no_string_payloads_in_command_error() {
    let source = read("command.rs");
    let at = source
        .find("pub enum CommandError")
        .expect("`CommandError` is declared");
    let body = source[at..]
        .split_once("\n}\n")
        .map(|(body, _)| body)
        .expect("the enum is terminated");

    for banned in [": String", "(String)", ": Box<str>", "(Box<str>)"] {
        assert!(
            !body.contains(banned),
            "`CommandError` carries a `{banned}` payload; a string loses the chain"
        );
    }
    assert!(
        body.contains("#[non_exhaustive]") || source[..at].contains("#[non_exhaustive]"),
        "`CommandError` is not `#[non_exhaustive]`"
    );
    // Every carried error is a typed source a caller can walk to.
    assert!(
        body.matches("#[source]").count() >= 5,
        "not every `CommandError` payload is a typed `#[source]`"
    );
}

#[test]
fn retry_has_no_default() {
    let source = read("command.rs");
    assert!(
        !source.contains("impl Default for Retry"),
        "`Retry` has a `Default`, so the bound is a hidden 3 again"
    );
    let at = source
        .find("pub struct Retry")
        .expect("`Retry` is declared");
    let attributes = source[..at]
        .rsplit("\n\n")
        .next()
        .unwrap_or_default()
        .to_owned();
    assert!(
        !attributes.contains("Default"),
        "`Retry` derives `Default`: {attributes}"
    );
}

// ---------------------------------------------------------------------------
// AC-010 — the density budget, and the gate a reader can see
// ---------------------------------------------------------------------------

#[test]
fn density_budget_holds() {
    let source = read("command.rs");
    let mut in_fence = false;
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        let Some(body) = trimmed
            .strip_prefix("///")
            .or_else(|| trimmed.strip_prefix("//!"))
        else {
            continue;
        };
        let body = body.strip_prefix(' ').unwrap_or(body);
        if body.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence && !body.starts_with("# ") {
            assert!(
                body.chars().count() <= FENCE_COLUMNS,
                "command.rs:{}: a fence line is {} columns, over {FENCE_COLUMNS}",
                index + 1,
                body.chars().count()
            );
        }
    }
    assert!(!in_fence, "command.rs: an unterminated doc fence");

    for name in [
        "commit",
        "commit_with",
        "Retry",
        "Committed",
        "CommandError",
    ] {
        assert!(
            name.len() <= IDENTIFIER,
            "`{name}` is {} characters, over {IDENTIFIER}",
            name.len()
        );
    }

    // Every item this story adds leads with one complete claim that fits the
    // item table's column.
    for declaration in [
        "pub async fn commit<",
        "pub async fn commit_with<",
        "pub struct Retry",
        "pub struct Committed",
        "pub enum CommandError",
    ] {
        let block = doc_block_above(&source, declaration);
        let first = block.lines().take_while(|line| !line.is_empty()).fold(
            String::new(),
            |mut acc, line| {
                if !acc.is_empty() {
                    acc.push(' ');
                }
                acc.push_str(line.trim());
                acc
            },
        );
        let end = first
            .find(". ")
            .or_else(|| first.ends_with('.').then(|| first.len() - 1))
            .unwrap_or_else(|| panic!("`{declaration}` has no complete first sentence"));
        assert!(
            end < FIRST_SENTENCE,
            "`{declaration}`'s first sentence is {} characters, over {FIRST_SENTENCE}",
            end + 1
        );
    }
}

#[test]
fn docsrs_metadata_is_present() {
    let manifest = normalise(
        &std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .expect("the crate's own manifest is readable"),
    );
    assert!(manifest.contains("[package.metadata.docs.rs]"));
    assert!(manifest.contains("all-features = true"));
    assert!(manifest.contains(r#"rustdoc-args = ["--cfg", "docsrs"]"#));
    assert!(read("lib.rs").contains("#![cfg_attr(docsrs, feature(doc_cfg))]"));

    // `commit` is the JSON convenience, so its page must say which feature
    // turns it on. `commit_with` is ungated and must not be gated.
    let source = read("command.rs");
    let at = source
        .find("pub async fn commit<")
        .expect("`commit` is declared");
    let attributes = source[..at]
        .rsplit("\n\n")
        .next()
        .unwrap_or_default()
        .to_owned();
    assert!(
        attributes.contains("#[cfg(feature = \"json\")]"),
        "`commit` is not gated on `json`"
    );
    assert!(
        attributes.contains("#[cfg_attr(docsrs, doc(cfg(feature = \"json\")))]"),
        "`commit` renders no gate badge"
    );

    let with = source
        .find("pub async fn commit_with<")
        .expect("`commit_with` is declared");
    let with_attributes = source[..with]
        .rsplit("\n\n")
        .next()
        .unwrap_or_default()
        .to_owned();
    assert!(
        !with_attributes.contains("#[cfg(feature"),
        "`commit_with` is feature-gated; it is the ungated door"
    );
}
