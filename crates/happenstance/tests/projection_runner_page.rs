//! What the runner's page owes an operator, and what it owed and did not say.
//!
//! Two gaps, one outcome. `run_projection` takes a `chunk: NonZeroUsize` whose
//! whole specification was its declaration — the page named it once, as the
//! unit of the buffer, and the doctest supplied `64` with no reason. And the
//! run is unobservable between chunk commits: no callback, no channel, no
//! `tracing`, and the only externally visible state is a durable checkpoint row
//! a caller would have to know to poll from a second handle.
//!
//! Put together, an operator rebuilding a projection over a large log — the
//! page's own documented use case — gets a process that is indistinguishable
//! from a hang for its entire duration, with the frequency of the only
//! observable moment set by the one parameter the crate gave no guidance on. A
//! caller who reasons *fewer commits is faster* and passes `1_000_000` buffers
//! a million applications into one write set, and the page's bounded-window
//! claim stays true the whole time — bounded by a number the caller invented.
//!
//! **Neither repair is taken here, and both are free.** `run_projection`,
//! `Progressed` and `ProjectionError` are behind `unstable-projection`, off by
//! default, and ADR-0036 records the semver exemption in terms —
//! *"`cargo-semver-checks` will not police the surface"* — so giving `chunk` a
//! named type, or adding an observed entry point, costs the same after `0.2.0`
//! as before it. Both belong to the projection-store freeze.
//!
//! What the exemption does **not** buy is silence. A page is not semver-gated
//! at all: it is read by whoever runs the thing today, and the operator above
//! is harmed now rather than at `0.2.0`. So what this file holds is the part
//! that is free *and* due: the page says what the number trades and what can be
//! seen while the run is going.

#![cfg(feature = "unstable-projection")]

use std::path::{Path, PathBuf};

fn src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// `runner.rs`, line endings normalised — the repository is developed on
/// Windows with `core.autocrlf = true`, so a fresh checkout is CRLF.
fn runner() -> String {
    std::fs::read_to_string(src().join("runner.rs"))
        .expect("runner.rs")
        .replace("\r\n", "\n")
}

/// The `///` lines of `run_projection`'s page, markers stripped, in order.
///
/// Read from the source rather than from the rendered HTML because the whole
/// subject is what the page *says*, and because a rendered page is not
/// available to a test that must fail in CI.
fn page(source: &str) -> Vec<&str> {
    let at = source
        .find("pub async fn run_projection<")
        .expect("`run_projection` is not in `runner.rs` any more");
    let mut block: Vec<&str> = Vec::new();
    for line in source[..at].lines().rev() {
        let trimmed = line.trim_start();
        if let Some(body) = trimmed.strip_prefix("/// ") {
            block.push(body);
        } else if trimmed == "///" {
            block.push("");
        } else if !trimmed.starts_with("//") && !trimmed.starts_with("#[") {
            break;
        }
    }
    assert!(
        block.len() > 40,
        "`run_projection`'s page is {} lines — the scanner has lost it, and \
         every assertion below would pass over nothing",
        block.len()
    );
    block.reverse();
    block
}

/// The index of the first line whose text starts with `heading`.
fn section(page: &[&str], heading: &str) -> usize {
    page.iter()
        .position(|line| line.trim() == heading)
        .unwrap_or_else(|| panic!("`run_projection`'s page has no `{heading}` section"))
}

/// The one parameter with no vocabulary gets some.
///
/// The wrong implementation this rejects is the page as it shipped: `chunk`
/// named once, as the unit of the buffer, with the doctest's `64` standing in
/// for a reason. It also rejects the likelier repair — a section that says
/// *"tune it"* without naming what goes wrong at either end, which is what a
/// reader already suspected and cannot act on.
#[test]
fn the_page_says_what_chunk_trades() {
    let source = runner();
    let page = page(&source);
    let at = section(&page, "# Choosing `chunk`");
    let body = page[at..].join(" ");

    for required in ["commit", "memory", "restart", "measure"] {
        assert!(
            body.contains(required),
            "the `# Choosing `chunk`` section never mentions `{required}`: a \
             reader still cannot tell which way to move the number, or what \
             they pay when they overshoot in either direction"
        );
    }
}

/// An operator is told what a running rebuild looks like from outside.
///
/// The wrong implementation this rejects is the page as it shipped, which
/// documents the rebuild-under-a-second-id workflow and never says that the
/// workflow is silent. A reader who is told to run a rebuild and not told it
/// looks like a hang finds out under time pressure.
#[test]
fn the_page_says_what_can_be_seen_while_it_runs() {
    let source = runner();
    let page = page(&source);
    let at = section(&page, "# What can be seen while it runs");
    let body = page[at..].join(" ");

    for required in ["checkpoint", "hang"] {
        assert!(
            body.contains(required),
            "the `# What can be seen while it runs` section never mentions \
             `{required}`, so it does not say what to poll or what silence \
             means"
        );
    }
}

/// Both new sections sit *below* the doctest, and that is load-bearing.
///
/// `spec/SPECIFICATION.md` cites `run_projection` by the line of the call
/// inside the fence, and `cargo xtask spec-trace` resolves a citation within a
/// twelve-line window. Prose inserted *above* the fence pushes that call out of
/// the window and fails the gate — with a message about an unresolved citation,
/// which is a long way from "you added a paragraph in the wrong place".
///
/// `runner.rs` carries that reasoning as a line comment beside the item. A
/// comment is advice; this is the part that fails.
#[test]
fn prose_added_to_this_page_stays_below_the_fence() {
    let source = runner();
    let page = page(&source);

    let fence = page
        .iter()
        .position(|line| line.trim_start().starts_with("```"))
        .expect("the page carries a doctest fence");

    for heading in ["# Choosing `chunk`", "# What can be seen while it runs"] {
        assert!(
            section(&page, heading) > fence,
            "`{heading}` was added above the doctest fence. Every line above \
             the fence pushes the `run_projection(` call inside it down, and \
             three sentences of `SPECIFICATION.md` cite this item at \
             `runner.rs:401` — a line inside that fence. `spec-trace` does not \
             anchor those three, so it will not tell you: this is where it is \
             said"
        );
    }
}
