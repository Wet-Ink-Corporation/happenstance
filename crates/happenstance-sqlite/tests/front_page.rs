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

/// This crate's own manifest — the artefact that decides whether cargo will
/// publish it, read as the fact rather than as one more opinion about it.
const MANIFEST: &str = include_str!("../Cargo.toml");

/// The gate's own intention, beside the fact, per RS-81-5. `include_str!` rather
/// than a copied list: the path is checked by the compiler, so moving the module
/// fails the build instead of silently emptying the derivation.
const PACKAGE_RS: &str = include_str!("../../../xtask/src/package.rs");

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

/// The front page never claims the crate still carries `publish = false`, in
/// any phrasing.
///
/// The check above is a phrase list, and a phrase list catches the sentence it
/// was written against and nothing else: this crate's front page went stale a
/// second time by spelling the identical false claim differently — `"is
/// `publish = false` until it passes"` became `"it still carries `publish =
/// false`"` — and `the_front_page_no_longer_describes_a_skeleton` stayed green
/// through the rewrite because neither string is a substring of the other.
///
/// So this test classifies every sentence that mentions the flag by what it
/// *says* rather than by matching one spelling of the claim: a sentence
/// mentioning `publish = false` must say the flag is gone, not that it is
/// carried, current, or still in force. That is the actual fact this crate's
/// own `Cargo.toml` and `xtask/src/package.rs`'s `PUBLISHABLE` have already
/// settled — no `publish = false` in the manifest, and the crate's name in the
/// list — so any sentence asserting the opposite is wrong regardless of which
/// words it uses to say so.
#[test]
fn the_front_page_never_claims_publish_false_is_still_carried() {
    const SAYS_GONE: [&str; 4] = ["is gone", "gone:", "no longer", "deliberately no"];

    for sentence in LIB.split('.') {
        if !sentence.contains("publish = false") {
            continue;
        }
        assert!(
            SAYS_GONE.iter().any(|marker| sentence.contains(marker)),
            "src/lib.rs has a sentence mentioning `publish = false` that does not \
             say the flag is gone: {sentence:?} — the manifest carries no \
             `publish = false` and `PUBLISHABLE` names this crate, so a sentence \
             claiming otherwise is stale no matter how it is phrased"
        );
    }
}

/// The crate this front page belongs to, spelled as `PUBLISHABLE` spells it.
const CRATE_NAME: &str = "happenstance-sqlite";

/// The manifest key, spelled as both the key and every sentence about it spell
/// it.
const FLAG: &str = "publish = false";

/// The phrase the front page is pinned to, **quoted from the manifest**.
///
/// Not wording this test invented. `Cargo.toml`'s own comment is what settled
/// the fact — the key is absent, and its absence is one half of a pair
/// `xtask/src/package.rs`'s `reconcile` fails on either half of — so holding the
/// front page to the manifest's words is what stops the pin drifting into a
/// preference. [`publication_problems`] checks the manifest still says it, in
/// the direction that names which side moved.
const PUBLICATION_ANCHOR: &str = "its absence is half of an atomic pair";

/// Text with its comment markers stripped and its runs of whitespace collapsed
/// to single spaces.
///
/// Applied to both sides of every anchor match, which is what lets the anchor be
/// written as one readable sentence while the prose carrying it is wrapped
/// across three source lines. Choosing an anchor that happens to fit one line
/// instead works today and breaks the day somebody re-wraps a paragraph, and the
/// repair a false positive teaches is deleting the pin.
fn collapsed(text: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        let line = line.trim_start();
        let line = line
            .strip_prefix("//!")
            .or_else(|| line.strip_prefix("///"))
            .or_else(|| line.strip_prefix("//"))
            .or_else(|| line.strip_prefix('#'))
            .unwrap_or(line);
        for word in line.split_whitespace() {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(word);
        }
    }
    out
}

/// Whether the manifest withholds this crate from a registry.
///
/// Read from the **keys**, never from the file's prose: `Cargo.toml`'s comment
/// carries the literal `publish = false` while explaining that the key is
/// absent, so a whole-file `contains` reports the exact opposite of the fact.
/// The key match is whole — `publish-lts = false` is a different key and does
/// not answer this question.
fn manifest_withholds_publication(manifest: &str) -> bool {
    manifest.lines().map(str::trim).any(|line| {
        line.strip_prefix("publish").is_some_and(|rest| {
            let rest = rest.trim_start();
            rest.starts_with('=') && rest.contains("false")
        })
    })
}

/// Whether `xtask`'s `PUBLISHABLE` names `crate_name`.
///
/// Scoped to the constant's own body rather than to the file, because the module
/// documentation above it discusses the constant at length, and matched as a
/// whole quoted token, because `"happenstance"` is a prefix of
/// `"happenstance-sqlite"` and a substring test would let the shorter entry
/// discharge the longer one's question.
fn publishable_names(package_rs: &str, crate_name: &str) -> bool {
    let Some((_, after)) = package_rs.split_once("const PUBLISHABLE") else {
        return false;
    };
    let Some((body, _)) = after.split_once("];") else {
        return false;
    };
    let quoted = format!("\"{crate_name}\"");
    body.lines()
        .map(str::trim)
        .filter(|line| !line.starts_with("//"))
        .any(|line| line.starts_with(&quoted))
}

/// The crate-root `//!` block as paragraphs, markers stripped and whitespace
/// collapsed.
///
/// The paragraph is the unit because a claim about the flag lives in one, and a
/// *second* claim needs a second — which is the whole mechanism below.
fn front_page_paragraphs(lib: &str) -> Vec<String> {
    let mut paragraphs = Vec::new();
    let mut current = String::new();
    for line in lib.lines() {
        let Some(text) = line.strip_prefix("//!") else {
            continue;
        };
        if text.trim().is_empty() {
            if !current.is_empty() {
                paragraphs.push(std::mem::take(&mut current));
            }
            continue;
        }
        for word in text.split_whitespace() {
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
        }
    }
    if !current.is_empty() {
        paragraphs.push(current);
    }
    paragraphs
}

/// Everything wrong with what the front page says about its own publication,
/// each problem worded so that it names *which artefact moved*.
///
/// # What this verifies
///
/// Four things, and they close different doors:
///
/// * the derived fact and the gate's hand-written intention agree, so the pin is
///   never held against a state neither artefact is in;
/// * the manifest still uses the anchor's words, so the pin cannot quietly
///   become this test's own preference;
/// * the front page carries the anchor, so a rewrite that drops the true
///   statement fails even when it says nothing false;
/// * the front page mentions [`FLAG`] exactly once, in the paragraph carrying
///   the anchor — so a false claim *added* beside the true one fails too,
///   whatever words it is written in.
///
/// The fourth is the one that matters. The check this replaces was a negative
/// matcher over spellings of a falsehood, and a negative matcher over an
/// unbounded set teaches the next paraphrase: the previous fix was falsified by
/// a sentence that carried the false claim and the string `"is gone"` in one
/// breath. A positive, anchored, *counted* pin has no such set to enumerate.
///
/// # What this does not verify
///
/// A falsehood written without the literal `publish = false` — *"this crate is
/// withheld from the registry"*, say — passes every assertion here, because
/// every one of them is keyed to that token. That blind spot is asserted in
/// [`the_publication_pin_rejects_a_paraphrase_and_states_what_it_cannot_see`]
/// rather than promised, so nobody reads this pin as covering ground it does
/// not. Closing it needs a reader, not a longer list of phrases.
fn publication_problems(manifest: &str, package_rs: &str, lib: &str) -> Vec<String> {
    let mut problems = Vec::new();

    let withholds = manifest_withholds_publication(manifest);
    let listed = publishable_names(package_rs, CRATE_NAME);
    match (withholds, listed) {
        (false, true) => {}
        (true, true) => problems.push(format!(
            "Cargo.toml carries `{FLAG}` and PUBLISHABLE names {CRATE_NAME}: cargo will not \
             publish a crate the gate checks, and the front page cannot be right about both"
        )),
        (true, false) => problems.push(format!(
            "Cargo.toml carries `{FLAG}`, so this crate is withheld again; this pin holds the \
             front page to the words of a published one, and it is the pin that is now stale"
        )),
        (false, false) => problems.push(format!(
            "PUBLISHABLE no longer names {CRATE_NAME} while its manifest carries no `{FLAG}`: \
             cargo would publish a crate the gate does not check"
        )),
    }

    if !collapsed(manifest).contains(PUBLICATION_ANCHOR) {
        problems.push(format!(
            "Cargo.toml no longer says {PUBLICATION_ANCHOR:?}; the pin is quoting wording the \
             manifest does not use, so it has become this test's preference rather than the \
             manifest's decision"
        ));
    }

    let paragraphs = front_page_paragraphs(lib);
    if !paragraphs
        .iter()
        .any(|paragraph| paragraph.contains(PUBLICATION_ANCHOR))
    {
        problems.push(format!(
            "src/lib.rs's crate-root docs do not say {PUBLICATION_ANCHOR:?}; the front page is \
             the only artefact a consumer reads, and it no longer states what the manifest \
             settled"
        ));
    }

    let mentions: usize = paragraphs
        .iter()
        .map(|paragraph| paragraph.matches(FLAG).count())
        .sum();
    if mentions != 1 {
        problems.push(format!(
            "src/lib.rs's crate-root docs mention `{FLAG}` {mentions} times; exactly one \
             mention is allowed, and it is the anchored one — a second is a second claim, and \
             the manifest has already settled which of the two is false"
        ));
    }
    for paragraph in &paragraphs {
        if paragraph.contains(FLAG) && !paragraph.contains(PUBLICATION_ANCHOR) {
            problems.push(format!(
                "src/lib.rs's crate-root docs mention `{FLAG}` in a paragraph that does not \
                 carry {PUBLICATION_ANCHOR:?}: {paragraph:?}"
            ));
        }
    }

    problems
}

/// The front page states the publication status its own manifest settled.
///
/// See [`publication_problems`] for the mechanism and for what it cannot see.
#[test]
fn the_front_page_states_the_publication_status_its_manifest_settled() {
    let problems = publication_problems(MANIFEST, PACKAGE_RS, LIB);
    assert!(
        problems.is_empty(),
        "the front page and the manifest disagree about publication:\n- {}",
        problems.join("\n- ")
    );
}

/// The pin rejects the paraphrase that falsified its predecessor, and the blind
/// spot it keeps is executed rather than promised.
///
/// The mutation is verbatim the one a refuter used against the negative matcher:
/// it restores the false claim in wording no phrase list held, and it carries
/// `"is gone"` — about a *different* subject — so the sentence classifier waved
/// it through. Both places it can be put are asserted, because putting it inside
/// the true paragraph rather than beside it is the cheaper edit of the two.
#[test]
fn the_publication_pin_rejects_a_paraphrase_and_states_what_it_cannot_see() {
    const TRUE_PAGE: &str = "\
//! Whether the crate is *published* is a different question.\n\
//! `publish = false` is gone from its manifest, and its absence is half of an\n\
//! atomic pair.\n";
    const MUTATION: &str = "The scoped allow is gone, but the manifest still carries \
                            `publish = false`, and lifting that is the publication pass's \
                            decision rather than this crate's.";

    assert!(
        publication_problems(MANIFEST, PACKAGE_RS, TRUE_PAGE).is_empty(),
        "the pin fires on a front page that states exactly what the manifest settled"
    );

    let beside = format!("{TRUE_PAGE}//!\n//! {MUTATION}\n");
    assert!(
        !publication_problems(MANIFEST, PACKAGE_RS, &beside).is_empty(),
        "the falsehood was restored in its own paragraph and the pin stayed green"
    );

    let within = format!("{TRUE_PAGE}//! {MUTATION}\n");
    assert!(
        !publication_problems(MANIFEST, PACKAGE_RS, &within).is_empty(),
        "the falsehood was appended to the anchored paragraph and the pin stayed green"
    );

    let dropped = TRUE_PAGE.replace("its absence is half of an", "it is");
    assert!(
        !publication_problems(MANIFEST, PACKAGE_RS, &dropped).is_empty(),
        "the anchor was paraphrased away and the pin stayed green"
    );

    // The documented blind spot, asserted so that no reader takes this pin for
    // ground it does not cover: the token is what every rule above is keyed to,
    // and a falsehood avoiding it is a review's to catch, not this test's.
    let unkeyed = format!("{TRUE_PAGE}//!\n//! This crate is withheld from the registry.\n");
    assert!(
        publication_problems(MANIFEST, PACKAGE_RS, &unkeyed).is_empty(),
        "the blind spot has closed; say so in `publication_problems`' documentation"
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
