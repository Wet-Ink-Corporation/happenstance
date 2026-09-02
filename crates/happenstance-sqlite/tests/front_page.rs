//! The crate's own published prose, read as an artefact rather than trusted.
//!
//! Nothing in this workspace reads documentation for truth: `cargo doc` under
//! `RUSTDOCFLAGS=-D warnings` proves that intra-doc links resolve and says
//! nothing about whether a sentence is still true, and none of `xtask`'s five
//! file-reading lints reads this crate at all. That gap is exactly how a
//! conformant adapter ships with a front page announcing it is a skeleton — the
//! defect survives every green run, because every green run is about the code.
//!
//! So this target is the small machine that closes the half of the gap a string
//! comparison can close: the markers that must be gone, the compiled results
//! that must survive the rewrite that removes them, and the density budget the
//! front page is held to. The other half — whether the replacement prose is
//! *true* — is a review obligation and is honestly recorded as one.
//!
//! It runs no query and opens no database. The sources are read with
//! [`include_str!`], so a marker reintroduced anywhere in `src/` fails a test
//! rather than waiting for a reader to notice.

/// Every module of the crate under test, by the path a reader would cite.
const SOURCES: [(&str, &str); 6] = [
    ("src/lib.rs", include_str!("../src/lib.rs")),
    ("src/connection.rs", include_str!("../src/connection.rs")),
    ("src/event_store.rs", include_str!("../src/event_store.rs")),
    (
        "src/projection_store.rs",
        include_str!("../src/projection_store.rs"),
    ),
    ("src/query_sql.rs", include_str!("../src/query_sql.rs")),
    ("src/row.rs", include_str!("../src/row.rs")),
];

const LIB: &str = include_str!("../src/lib.rs");
const EVENT_STORE: &str = include_str!("../src/event_store.rs");
const PROJECTION_STORE: &str = include_str!("../src/projection_store.rs");
const SHAPES: &str = include_str!("shapes.rs");

/// The lines of `source` that are code rather than comment.
///
/// A `todo!()` inside a doc comment is a sentence about history; a `todo!()` in
/// code is work not done. Only the second is a marker, and conflating them is
/// how a legitimate paragraph about what the crate used to be gets deleted to
/// satisfy a grep.
fn code_lines(source: &str) -> impl Iterator<Item = (usize, &str)> {
    source
        .lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line))
        .filter(|(_, line)| !line.trim_start().starts_with("//"))
}

/// The crate root carries no `#![allow(clippy::todo)]`.
///
/// `todo = "deny"` is workspace-wide and this crate opted out of it by name.
/// With the attribute gone the workspace lint covers the crate again with no
/// exception, which is the whole mechanism — `#![allow]` is silent in both
/// directions, so nothing but this assertion notices an attribute that has
/// outlived the bodies it was written for.
#[test]
fn the_scoped_allow_is_gone_from_the_crate_root() {
    assert!(
        !LIB.contains("allow(clippy::todo)"),
        "src/lib.rs still carries a scoped `#![allow(clippy::todo)]`; the \
         attribute and the last `todo!()` leave together"
    );
}

/// No `todo!()` invocation survives on any SQLite path.
#[test]
fn no_todo_macro_survives_in_the_crate() {
    for (path, source) in SOURCES {
        for (number, line) in code_lines(source) {
            assert!(
                !line.contains("todo!("),
                "{path}:{number} is still a `todo!()`: {}",
                line.trim()
            );
        }
    }
}

/// The name of the innermost function declared at or above `line`.
///
/// Keyed on the *declaration* rather than on a `{`-counting scan of the file: an
/// invocation's enclosing `fn` is the nearest `fn` above it in every shape this
/// crate writes, and a brace counter is a parser with the failure modes of one.
/// Returns `None` above the first function in the file, which is the honest
/// answer for a macro invocation at module scope and excuses nothing.
fn enclosing_fn(source: &str, line: usize) -> Option<&str> {
    let above: Vec<&str> = source.lines().take(line).collect();
    above.into_iter().rev().find_map(|candidate| {
        let mut rest = candidate.trim_start();
        for prefix in ["pub(crate) ", "pub ", "const ", "async ", "unsafe "] {
            rest = rest.strip_prefix(prefix).unwrap_or(rest);
        }
        let name = rest.strip_prefix("fn ")?;
        Some(
            name.split(|c: char| !(c.is_alphanumeric() || c == '_'))
                .next()
                .unwrap_or(name),
        )
    })
}

/// No `todo!()` synonym stands in for work not done.
///
/// `clippy::unimplemented` is **not** in the workspace lint table, so
/// `-D warnings` has nothing to fire on and a one-character rename defeats every
/// grep in the project. The one invocation this crate is allowed is named here
/// with its reason, so that a second one is a test failure rather than a
/// judgement call: `SqliteProjectionStore::probe_read_through` panics because
/// `READS_THROUGH_BATCH` is `false` and answering from committed state is what
/// PS-12 forbids — a contract-mandated panic on a path the suite never takes,
/// not a body someone has not written yet.
///
/// The excuse is keyed to that **function**, and the first spelling of it was
/// keyed to the file and to a substring of the file — `*permitted_path == path
/// && source.contains(reason)`. That excused every synonym anywhere in
/// `projection_store.rs`, because the reason string stays present in the file
/// wherever a second one is added, so the doc comment above claimed a guard the
/// code did not have. One invocation site is excused; a second anywhere else in
/// the same file fails.
#[test]
fn no_todo_synonym_stands_in_for_work_not_done() {
    const PERMITTED: [(&str, &str); 1] = [("src/projection_store.rs", "probe_read_through")];

    for (path, source) in SOURCES {
        for (number, line) in code_lines(source) {
            for marker in ["unimplemented!(", "unreachable!(", "not implemented"] {
                if !line.contains(marker) {
                    continue;
                }
                let excused = PERMITTED.iter().any(|(permitted_path, permitted_fn)| {
                    *permitted_path == path && enclosing_fn(source, number) == Some(*permitted_fn)
                });
                assert!(
                    excused,
                    "{path}:{number} stands in for work not done with `{marker}`: {}",
                    line.trim()
                );
            }
        }
    }
}

/// The front page describes the adapter that exists, not the skeleton it was.
///
/// Four statements shipped on docs.rs while every one of them was false. Each is
/// asserted absent by the phrase a reader would recognise it by, because that is
/// what a stale sentence is — recognisable, and unnoticed.
#[test]
fn the_front_page_no_longer_describes_a_skeleton() {
    for stale in [
        "an instrument, not yet an adapter",
        "Every operation that touches SQL is",
        "is `publish = false` until it passes",
        "These are settled in the pass that implements this crate",
        "Which one wins depends on how tag matching is",
        "A join table against a canonical serialised blob",
    ] {
        assert!(
            !LIB.contains(stale),
            "src/lib.rs still says {stale:?}, which stopped being true when the \
             suite went green against a real file"
        );
    }

    assert!(
        LIB.contains("ADR-0022"),
        "src/lib.rs closes its open decisions against nothing; ADR-0022 settled \
         the append-condition strategy and tag storage, and the page should say \
         which decision closed them"
    );
}

/// Removing the markers does not remove what sat behind them.
///
/// Three passages on the crate root carry *compiled results* rather than status,
/// and each is cited from outside this crate. Deleting them alongside the status
/// prose is the tidy-up that loses the evidence, and it is the likeliest way for
/// this story to do damage while satisfying every other assertion here.
#[test]
fn the_front_page_keeps_its_compiled_results() {
    for kept in [
        "# The shape this crate represents",
        "SQLite wearing four",
        "# What the type checker has already decided",
        "error[E0195]",
        "# Not the Cloudflare adapter",
        "its futures are `!Send`",
    ] {
        assert!(
            LIB.contains(kept),
            "src/lib.rs no longer carries {kept:?} — a compiled result, not a \
             status marker, and not this rewrite's to remove"
        );
    }
}

/// The front page stays a front page.
///
/// 76 `//!` lines across five `#` headings is what it was; the budget is five
/// headings and roughly twenty lines either side of that. A front page that
/// grows into an essay stops being one, and a front page that shrinks to a
/// sentence has thrown away the compiled results above.
#[test]
fn the_front_page_stays_within_its_density_budget() {
    let doc_lines = LIB.lines().filter(|line| line.starts_with("//!")).count();
    let headings = LIB
        .lines()
        .filter(|line| line.starts_with("//! # "))
        .count();

    assert!(
        (54..=94).contains(&doc_lines),
        "the crate root's module doc is {doc_lines} lines; the budget is 54–94, \
         measured from the 76 it carried as an instrument"
    );
    assert!(
        headings <= 5,
        "the crate root's module doc carries {headings} `#` headings; the budget \
         is five"
    );
}

/// No status marker one level down contradicts the front page.
///
/// A marker in prose is still a marker, and the two most easily missed are not
/// in `src/` at all: a test target's module doc explaining that it runs nothing
/// because every body is unimplemented is wrong about its own crate the moment
/// the bodies are real.
#[test]
fn no_module_or_test_target_still_calls_this_crate_a_skeleton() {
    for (path, source) in [
        ("src/event_store.rs", EVENT_STORE),
        ("src/projection_store.rs", PROJECTION_STORE),
        ("tests/shapes.rs", SHAPES),
    ] {
        for stale in [
            "# Status: bodies unimplemented, types real",
            "every body is `todo!()` in the crate under test",
            "the whole point of the skeleton is",
        ] {
            assert!(
                !source.contains(stale),
                "{path} still says {stale:?}, which contradicts the crate's own \
                 front page"
            );
        }
    }
}
