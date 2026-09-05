//! What the facade mounts from the contract crate, and under whose gate.
//!
//! `happenstance` is a facade: the contract's paths are meant to still work
//! here. A **glob** — `pub use happenstance_core::*;` — delivers that in one
//! line and pays for it twice. It re-exports whatever the *compiled* contract
//! crate happens to expose, so an item `happenstance-core` gates on one of
//! **its own** features arrives at `happenstance::` whenever anything else in
//! the build graph turns that feature on; `happenstance-testkit` is a
//! dev-dependency of this crate and turns `unstable-projection` on
//! unconditionally, so under `cargo test` the whole unfrozen projection port
//! was reachable at `happenstance::` with this crate's `unstable-projection`
//! off — the opposite of what the manifest promises ("A reader has to type the
//! word `unstable` before any of them is in their build"). And it makes every
//! future addition to the contract crate an addition to this crate's surface
//! with nobody reviewing it.
//!
//! So the list is written out by hand, and this file is what keeps the hand
//! list and the fact in agreement (RS-81-5): the **fact** is derived from
//! `happenstance-core`'s own crate root, the **intention** is the list in
//! `happenstance`'s, and a failure here says which of the two moved.
//!
//! Read as source rather than resolved as paths on purpose. A leaked item is
//! *more* reachable, not less, so no `use` of it fails to compile; and the
//! configuration where the leak appears is the one where the feature is off,
//! which is the configuration `cargo test --all-features` never builds. There
//! is no type that says "this name must not resolve".

use std::collections::{BTreeMap, BTreeSet};

/// The contract crate's root — the fact.
const CORE_ROOT: &str = include_str!("../../happenstance-core/src/lib.rs");

/// This crate's root — the intention.
const ROOT: &str = include_str!("../src/lib.rs");

/// This crate's manifest, for the set of features it actually declares.
const MANIFEST: &str = include_str!("../Cargo.toml");

/// One `pub use` or `pub mod`: the names it publishes, and the features that
/// have to be on for it to exist.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Mounted {
    /// The `feature = "…"` names inside the item's `#[cfg(…)]`, sorted.
    ///
    /// A set rather than the attribute's text, because `all(a, b)` and
    /// `all(b, a)` are the same gate and a string comparison would call them
    /// different. It is deliberately blind to `any(…)` and `not(…)`: neither
    /// appears on a re-export in either crate root, and
    /// [`gate_of`] hard-errors rather than guessing if one ever does.
    gate: BTreeSet<String>,
    /// The line the item starts on, so a failure can be walked to.
    line: usize,
}

/// The `feature = "…"` names in one `#[cfg(…)]` attribute.
///
/// Hard-errors on anything else it meets (RS-81-2): a scanner that silently
/// returns "no features" for a gate it could not read reports the leak it was
/// written to find as compliance.
fn gate_of(attribute: &str, file: &str, line: usize) -> BTreeSet<String> {
    let inner = attribute
        .trim()
        .strip_prefix("#[cfg(")
        .and_then(|rest| rest.strip_suffix(")]"))
        .unwrap_or_else(|| panic!("{file}:{line}: cannot lex the gate `{attribute}`"));

    let inner = inner.strip_prefix("all(").map_or(inner, |rest| {
        rest.strip_suffix(')')
            .unwrap_or_else(|| panic!("{file}:{line}: unbalanced `all(` in `{attribute}`"))
    });

    let mut features = BTreeSet::new();
    for clause in inner.split(',') {
        let clause = clause.trim();
        if clause.is_empty() {
            continue;
        }
        let name = clause
            .strip_prefix("feature = \"")
            .and_then(|rest| rest.strip_suffix('"'))
            .unwrap_or_else(|| {
                panic!("{file}:{line}: `{clause}` is not `feature = \"…\"` — teach this scanner")
            });
        features.insert(name.to_owned());
    }
    assert!(
        !features.is_empty(),
        "{file}:{line}: a `#[cfg]` naming no feature: `{attribute}`"
    );
    features
}

/// The names one `pub use …;` or `pub mod …;` statement publishes.
///
/// `path` is everything between the keyword and the `;`, already joined across
/// lines. Aliases are read through their `as`, because the alias is the name a
/// caller types.
fn names_of(path: &str, file: &str, line: usize) -> Vec<String> {
    let leaf = |item: &str| -> String {
        let item = item.trim();
        let item = item.rsplit(" as ").next().unwrap_or(item).trim();
        item.rsplit("::").next().unwrap_or(item).trim().to_owned()
    };

    match path.split_once('{') {
        None => {
            assert!(
                !path.contains('*'),
                "{file}:{line}: a glob re-export — this file exists to forbid it"
            );
            vec![leaf(path)]
        }
        Some((_, rest)) => {
            let list = rest
                .strip_suffix('}')
                .unwrap_or_else(|| panic!("{file}:{line}: unbalanced `{{` in `{path}`"));
            list.split(',')
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(leaf)
                .collect()
        }
    }
}

/// Every name a crate root publishes with `pub use` or `pub mod`, mapped to its
/// gate.
///
/// `only_from` restricts to re-exports whose path starts with that prefix,
/// which is how this crate's own items — `Codec`, `commit`, `runner::…` — are
/// kept out of the comparison.
fn published(source: &str, file: &str, only_from: Option<&str>) -> BTreeMap<String, Mounted> {
    let mut out = BTreeMap::new();
    // The attribute is kept as text and lexed only when a `pub use`/`pub mod`
    // turns out to follow it. Lexing on sight would hard-error on
    // `#[cfg(doctest)] mod reexported_paths {}` — a private module this
    // comparison never looks at — and RS-81-2's hard error is worth keeping
    // pointed at the statements that are actually in scope.
    let mut gate: Option<(String, usize)> = None;
    let mut skipping_attribute = false;

    let mut lines = source.lines().enumerate().peekable();
    while let Some((index, line)) = lines.next() {
        let number = index + 1;
        let trimmed = line.trim();

        if skipping_attribute {
            skipping_attribute = !trimmed.ends_with(")]");
            continue;
        }
        // `#[cfg_attr(docsrs, doc(cfg(…)))]` is the rendered badge, not the
        // gate. It carries the same features and asserting on it as well would
        // be asserting the same fact twice; the badge is `docs_composition`'s.
        if trimmed.starts_with("#[cfg_attr(") {
            skipping_attribute = !trimmed.ends_with(")]");
            continue;
        }
        if trimmed.starts_with("#[cfg(") {
            gate = Some((trimmed.to_owned(), number));
            continue;
        }
        if trimmed.starts_with("//") || trimmed.starts_with("#!") || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.is_empty() {
            gate = None;
            continue;
        }

        let Some(rest) = trimmed
            .strip_prefix("pub use ")
            .or_else(|| trimmed.strip_prefix("pub mod "))
        else {
            gate = None;
            continue;
        };

        // A `pub use` may wrap. Join until the `;`, which is the only way this
        // scanner can see the whole item.
        let mut statement = rest.to_owned();
        while !statement.contains(';') {
            let (_, next) = lines
                .next()
                .unwrap_or_else(|| panic!("{file}:{number}: unterminated `pub use`"));
            statement.push_str(next.trim());
        }
        let statement = statement
            .split_once(';')
            .expect("the loop above stops on the `;`")
            .0
            .to_owned();

        let wanted = only_from.is_none_or(|prefix| statement.starts_with(prefix));
        if wanted {
            let features = gate
                .as_ref()
                .map_or_else(BTreeSet::new, |(text, at)| gate_of(text, file, *at));
            for name in names_of(&statement, file, number) {
                out.insert(
                    name,
                    Mounted {
                        gate: features.clone(),
                        line: number,
                    },
                );
            }
        }
        gate = None;
    }
    out
}

/// The features `happenstance` declares, read off its own `[features]` table.
fn declared_features() -> BTreeSet<String> {
    let mut inside = false;
    let mut out = BTreeSet::new();
    for line in MANIFEST.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            inside = trimmed == "[features]";
            continue;
        }
        if !inside || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((name, _)) = trimmed.split_once('=') {
            out.insert(name.trim().to_owned());
        }
    }
    assert!(!out.is_empty(), "the manifest has no `[features]` table");
    out
}

/// The wrong implementation this file exists to reject, named as a literal.
///
/// Stated separately from [`names_of`]'s panic because the glob is a *whole
/// statement* rather than a name, and a reader who deletes the explicit list
/// and types this line back is the failure mode.
#[test]
fn the_contract_crate_is_not_glob_re_exported() {
    assert!(
        !ROOT.contains("pub use happenstance_core::*;"),
        "`crates/happenstance/src/lib.rs` glob re-exports the contract crate. \
         A glob mounts whatever the *compiled* contract crate exposes, which \
         is gated on **its** features and not on this crate's: with \
         `happenstance-testkit` in the dev-dependency graph that puts the \
         whole unfrozen projection port at `happenstance::` with \
         `unstable-projection` off. Write the list out."
    );
}

/// Every contract item this crate mounts arrives under the gate it was written
/// behind.
///
/// Three ways to fail, and the first is the one that shipped:
///
/// 1. an item `happenstance-core` gates is mounted here with no gate, or with
///    a weaker one — the leak;
/// 2. an item `happenstance-core` gates on a feature this crate does not even
///    declare (`conformance`, whose `ProjectionProbe` is a test double) is
///    mounted at all;
/// 3. an item `happenstance-core` publishes unconditionally is *missing* —
///    which is how "delete the glob" stops being a fix and starts being a
///    breaking change to the facade.
#[test]
fn every_contract_item_arrives_under_the_gate_it_was_written_behind() {
    let core = published(CORE_ROOT, "happenstance-core/src/lib.rs", None);
    let facade = published(ROOT, "happenstance/src/lib.rs", Some("happenstance_core::"));
    let features = declared_features();

    // The scanner must be finding something, or every assertion below is
    // vacuous (RS-81-4: an empty set passes every `for`).
    assert!(
        core.len() > 30,
        "the scanner found only {} items in the contract crate's root — it has \
         stopped reading it, and every check below is now decorative",
        core.len()
    );

    for (name, fact) in &core {
        // The contract crate re-exports itself to nobody; `bytes` and
        // `futures_core` are items, and they are in the comparison.
        let intent = facade.get(name);

        let undeclared: Vec<&String> = fact
            .gate
            .iter()
            .filter(|feature| !features.contains(*feature))
            .collect();
        if !undeclared.is_empty() {
            assert!(
                intent.is_none(),
                "`happenstance` mounts `{name}` (its lib.rs:{}), which \
                 `happenstance-core` gates on {undeclared:?} — features this \
                 crate does not declare, so no consumer of this crate can turn \
                 them on and no consumer can turn them off",
                intent.expect("checked").line
            );
            continue;
        }

        let Some(intent) = intent else {
            assert!(
                !fact.gate.is_empty(),
                "`happenstance-core` publishes `{name}` unconditionally \
                 (its lib.rs:{}) and `happenstance` does not re-export it. The \
                 facade's promise is that the contract's paths still work here.",
                fact.line
            );
            continue;
        };

        assert_eq!(
            intent.gate, fact.gate,
            "`{name}` is mounted at `happenstance::` behind {:?} \
             (lib.rs:{}) and written in `happenstance-core` behind {:?} \
             (its lib.rs:{}). The gates must be the same set: a narrower one \
             here hides an item a consumer asked for, and a wider one — or \
             none — hands out an item they did not.",
            intent.gate, intent.line, fact.gate, fact.line
        );
    }
}
