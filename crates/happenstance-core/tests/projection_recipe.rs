//! The published recipe for a projection adapter, held to compiling.
//!
//! `ProjectionProbe`'s page carries a `toml` fence telling an adapter author
//! what to put in their manifest. A `toml` fence is not a doctest: nothing
//! compiles it, nothing resolves it, and the first reader to find out it is
//! wrong is an author following it.
//!
//! It was wrong once. The fence wrote `happenstance-core = "…"` with no
//! features while every item the recipe asks the author to implement —
//! `ProjectionStore` and its value types — sat behind `unstable-projection`,
//! which was not in the default set; an adapter cannot make its own port impl
//! optional, so the recipe as printed did not compile. ADR-0063 lifted that
//! gate, and the fence is back to naming no feature on the dependency line —
//! which is right *now* and was wrong *then*, and the only way to tell the two
//! apart is to read `lib.rs`.
//!
//! So the requirement is **derived** rather than typed in here (RS-81-5): the
//! fact is which feature `lib.rs` gates each item behind, the intention is what
//! the fence says, and a failure names which of the two moved. A hand-written
//! `assert!(!fence.contains("unstable-projection"))` would pass just as happily
//! after somebody re-gated the port in `lib.rs` and not in the fence, which is
//! the mirror of the state this file was written to reject.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// A source file with its line endings normalised to `\n`.
fn read(name: &str) -> String {
    std::fs::read_to_string(src().join(name))
        .unwrap_or_else(|error| panic!("{name}: {error}"))
        .replace("\r\n", "\n")
}

/// The body of the one ```` ```toml ```` fence on `ProjectionProbe`'s page,
/// doc markers stripped.
///
/// Hard-errors rather than returning nothing when the fence is not found: an
/// empty fence satisfies nothing below and would report the recipe as absent
/// instead of as broken, which are different repairs.
fn recipe() -> String {
    let source = read("projection.rs");
    let at = source
        .find("pub trait ProjectionProbe")
        .expect("`ProjectionProbe` is not in `projection.rs` any more");

    let page = &source[..at];
    let open = page
        .rfind("/// ```toml")
        .expect("`ProjectionProbe`'s page carries no `toml` fence");
    let body = &page[open + "/// ```toml".len()..];
    let close = body
        .find("/// ```")
        .expect("the `toml` fence on `ProjectionProbe`'s page never closes");

    body[..close]
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            trimmed
                .strip_prefix("/// ")
                .or_else(|| (trimmed == "///").then_some(""))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The features `lib.rs` gates `item` behind, read off the crate root.
///
/// Walks the root once, carrying the `#[cfg(...)]` above each `pub use` onto
/// every name that statement publishes — which is what a reader does, and what
/// a `rfind` backwards from the name cannot, because the statement wraps and
/// the attribute is three lines further up than the name is.
///
/// Hard-errors on a gate it cannot lex rather than reporting one as absent
/// (RS-81-2), and the callers below refuse an empty answer (RS-81-4): a
/// scanner that has stopped finding gates passes every assertion in this file.
fn gated_behind(item: &str) -> BTreeSet<String> {
    let source = read("lib.rs");
    let mut gate: Option<String> = None;
    let mut skipping = false;

    let mut lines = source.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        if skipping {
            skipping = !trimmed.ends_with(")]");
            continue;
        }
        if trimmed.starts_with("#[cfg_attr(") {
            skipping = !trimmed.ends_with(")]");
            continue;
        }
        if trimmed.starts_with("#[cfg(") {
            gate = Some(trimmed.to_owned());
            continue;
        }
        if trimmed.starts_with("//") {
            continue;
        }
        let Some(rest) = trimmed.strip_prefix("pub use ") else {
            if trimmed.is_empty() {
                gate = None;
            }
            continue;
        };

        let mut statement = rest.to_owned();
        while !statement.contains(';') {
            let next = lines.next().expect("an unterminated `pub use` in lib.rs");
            statement.push_str(next.trim());
        }

        let publishes = statement
            .split(|c: char| !(c.is_alphanumeric() || c == '_'))
            .any(|name| name == item);
        if publishes {
            return match gate {
                None => BTreeSet::new(),
                Some(attribute) => features_of(&attribute),
            };
        }
        gate = None;
    }
    panic!("`{item}` is not re-exported from `happenstance-core`'s crate root any more")
}

/// The `feature = "…"` names inside one `#[cfg(...)]`.
fn features_of(attribute: &str) -> BTreeSet<String> {
    let inner = attribute
        .strip_prefix("#[cfg(")
        .and_then(|rest| rest.strip_suffix(")]"))
        .unwrap_or_else(|| panic!("cannot lex the gate `{attribute}`"));
    let inner = inner.strip_prefix("all(").map_or(inner, |rest| {
        rest.strip_suffix(')')
            .unwrap_or_else(|| panic!("unbalanced `all(` in `{attribute}`"))
    });

    inner
        .split(',')
        .map(str::trim)
        .filter(|clause| !clause.is_empty())
        .map(|clause| {
            clause
                .strip_prefix("feature = \"")
                .and_then(|rest| rest.strip_suffix('"'))
                .unwrap_or_else(|| panic!("`{clause}` is not `feature = \"…\"`"))
                .to_owned()
        })
        .collect()
}

/// `gated_behind`, refusing an empty answer.
///
/// `ProjectionProbe` is gated today, behind `conformance`. If it ever stops
/// being, this assertion is what should be deleted with it — deliberately,
/// rather than by a scanner quietly finding nothing.
fn must_be_gated(item: &str) -> BTreeSet<String> {
    let features = gated_behind(item);
    assert!(
        !features.is_empty(),
        "`{item}` is re-exported with no `#[cfg]` at all. Either it left          `conformance` — in which case this file's premise is gone and it          should be deleted — or the scanner has stopped reading `lib.rs`"
    );
    features
}

/// The port itself is unconditional, and the scanner can see that it is.
///
/// The positive control for [`gated_behind`]: a scanner that reports an empty
/// set for `ProjectionStore` because it found no gate is indistinguishable
/// from one that reports it because it stopped reading. `ProjectionProbe`
/// being found gated in the same run is what separates them.
#[test]
fn the_port_is_not_gated_and_the_probe_still_is() {
    assert!(
        gated_behind("ProjectionStore").is_empty(),
        "`ProjectionStore` has acquired a `#[cfg]` again; ADR-0063 lifted the          gate and a re-gated port is a decision record, not a line edit"
    );
    assert!(
        gated_behind("ProjectionProbe").contains("conformance"),
        "`ProjectionProbe` is no longer behind `conformance`, so the scanner's          empty answer for `ProjectionStore` proves nothing"
    );
}

/// Every feature the recipe's own items need is named in the recipe.
///
/// The wrong implementation this rejects is the fence as it shipped:
/// `happenstance-core = "…"`, no features, for a page whose whole subject is a
/// trait behind two of them. It also rejects the subtler state — a feature
/// renamed in `Cargo.toml` and `lib.rs` and not here — because the requirement
/// is read off `lib.rs` on every run rather than written down twice.
#[test]
fn the_recipe_names_every_feature_its_own_items_need() {
    let recipe = recipe();

    // `ProjectionProbe` is the trait the page documents and the one item on it
    // still behind a feature. `ProjectionStore`, its supertrait, is
    // unconditional since ADR-0063 — the test above holds that — so it needs
    // no feature named and would only fail this loop by acquiring a gate.
    for item in ["ProjectionProbe"] {
        for feature in must_be_gated(item) {
            assert!(
                recipe.contains(&feature),
                "`ProjectionProbe`'s manifest recipe never names `{feature}`, \
                 which is what `lib.rs` gates `{item}` behind. An adapter that \
                 copies this fence does not compile, and an adapter's port impl \
                 cannot be made optional to work around it:\n{recipe}"
            );
        }
    }
}

/// The dependency line carries exactly the features the port needs, which is
/// none.
///
/// The distinction this used to guard is still the point. `conformance` is
/// forwarded through the adapter's own feature, because the `impl
/// ProjectionProbe` is `cfg`-gated and a crate cannot `cfg` on a dependency's
/// feature. The `impl ProjectionStore` is unconditional in the adapter's
/// `src/`, so whatever the port needs the dependency line must carry — and
/// since ADR-0063 that is nothing. A line that still said `features =
/// ["unstable-projection"]` would compile, because the feature is retained as
/// an empty no-op, and would tell an author the port is gated when it is not.
#[test]
fn the_dependency_line_carries_what_the_port_needs_and_nothing_else() {
    let recipe = recipe();
    let line = recipe
        .lines()
        .find(|line| line.trim_start().starts_with("happenstance-core ="))
        .unwrap_or_else(|| panic!("the recipe has no `happenstance-core` dependency:\n{recipe}"));

    let needed = gated_behind("ProjectionStore");
    for feature in &needed {
        assert!(
            line.contains(feature),
            "the recipe's dependency line is `{line}` and does not enable              `{feature}`, which `lib.rs` gates `ProjectionStore` behind"
        );
    }
    if needed.is_empty() {
        assert!(
            !line.contains("features"),
            "the recipe's dependency line is `{line}`, naming a feature the              port does not need; an author copying it learns the port is gated"
        );
    }
}
