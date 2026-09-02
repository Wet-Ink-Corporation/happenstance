//! Source-reading assertions over the falsification drill under step 3.
//!
//! The drill is the half of the opening encounter that no doctest can check on
//! its own: whether the check exists is a `cargo test` question, but whether a
//! reader can *perform* it — the exact edit, the exact failure, the exact
//! revert, in that order, on the page, not folded away — is a composition
//! question, and a page that mounted the drill as one undifferentiated
//! paragraph inside a `<details>` with the failure paraphrased would satisfy
//! every mechanical check the slice otherwise runs.
//!
//! The one assertion here that is not about composition is the byte comparison
//! between the failure the page quotes and the failure the recorded run
//! produced. It is the check that catches a drill written from imagination,
//! which the story's own risk table ranks first.

use std::path::{Path, PathBuf};

/// The page the drill lives on.
const PAGE: &str = "docs/first-encounter.md";
/// The step the drill sits at the bottom of.
const STEP: &str = "## A condition that refuses";
/// The drill's heading, supplied by `tension-resolutions/_resolutions.md`
/// § Anchor table, which read the emitted fragment id off a render.
const DRILL: &str = "### Try it wrong, then put it back";
/// The tier-4 transcript. The only record initiative DoD-4 will ever have.
const OBSERVATION: &str = ".bklg/docs-that-teach/application-author-path/\
                          boundary-falsification-drill/_drill-observation.md";
/// The second, already-real mount: the same scenario as a `#[tokio::test]`.
const TWIN: &str = "crates/happenstance/tests/boundary_refusal.rs";
/// The gate step whose `REQUIRED` array this story may not touch.
const GATE: &str = "xtask/src/main.rs";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask sits one level below the repository root")
        .to_path_buf()
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel))
        .unwrap_or_else(|_| panic!("{rel} is readable"))
        .replace("\r\n", "\n")
}

/// Step 3's lines, from its heading to the end of the page.
fn step_three(page: &str) -> Vec<&str> {
    let lines: Vec<&str> = page.lines().collect();
    let at = lines
        .iter()
        .position(|line| line.trim_end() == STEP)
        .unwrap_or_else(|| panic!("`{STEP}` is not a heading on {PAGE}"));
    lines[at..]
        .iter()
        .enumerate()
        .take_while(|(offset, line)| *offset == 0 || !line.starts_with("## "))
        .map(|(_, line)| *line)
        .collect()
}

/// The drill's own lines, from its `###` to the end of the step.
fn drill(page: &str) -> Vec<String> {
    let step = step_three(page);
    let at = step
        .iter()
        .position(|line| line.trim_end() == DRILL)
        .unwrap_or_else(|| panic!("`{DRILL}` is not under `{STEP}` on {PAGE}"));
    step[at..].iter().map(|line| (*line).to_owned()).collect()
}

// ---------------------------------------------------------------------------
// AC-001 — where the design put it, and three labelled parts in order
// ---------------------------------------------------------------------------

#[test]
fn the_drill_is_a_subsection_at_the_bottom_of_step_three() {
    let page = read(PAGE);
    let step = step_three(&page);

    let heading = step
        .iter()
        .position(|line| line.trim_end() == DRILL)
        .unwrap_or_else(|| panic!("`{DRILL}` is not under `{STEP}`"));

    // After the clause citation, which closes the step's own material.
    let citation = step
        .iter()
        .position(|line| line.contains("](../spec/SPECIFICATION.md#es-25"))
        .expect("step three closes on the ES-25 citation");
    assert!(
        heading > citation,
        "the drill precedes the clause citation; it is only performable by \
         someone who has just run step three"
    );

    // It is a `###` under a `##`, so no heading level is skipped, and it is
    // the only one — a second would make the drill a subject.
    let subsections = step.iter().filter(|line| line.starts_with("### ")).count();
    assert_eq!(
        subsections, 1,
        "step three carries {subsections} subsections, not one"
    );
}

#[test]
fn the_drill_states_the_edit_then_the_failure_then_the_revert() {
    let page = read(PAGE);
    let body = drill(&page);

    let labels = [
        "**The edit.**",
        "**What you should see.**",
        "**Putting it back.**",
    ];
    let mut at = 0;
    for label in labels {
        let found = body
            .iter()
            .skip(at)
            .position(|line| line.contains(label))
            .unwrap_or_else(|| {
                panic!("the drill never labels `{label}`, or labels it out of order")
            });
        at += found + 1;
    }

    // The edit is stated diff-shaped, in operable terms rather than described.
    let text = body.join("\n");
    assert!(
        text.contains("Some(&condition)") && text.contains("None"),
        "the drill does not name the expression to change and what to change it \
         to: {text}"
    );

    // And it names a command, so "run it" is not left to the reader.
    assert!(
        text.contains("cargo test -p xtask --doc"),
        "the drill names no command to run"
    );
}

// ---------------------------------------------------------------------------
// AC-002 and AC-003 — executed at both mounts, and no gate step is added
// ---------------------------------------------------------------------------

#[test]
fn the_twin_asserts_refusal_with_the_condition_and_acceptance_without() {
    let twin = read(TWIN);
    assert!(
        twin.contains("#[tokio::test]"),
        "{TWIN} is not an executed test"
    );
    assert!(
        twin.contains("Err(AppendError::ConditionViolated(_))"),
        "{TWIN} does not assert the refusal"
    );
    assert!(
        twin.contains("Some(&condition)") && twin.contains("None"),
        "{TWIN} does not run the scenario both with and without the condition, \
         so it cannot show the boundary is what does the work"
    );
    assert!(
        !twin.contains("#[ignore]") && !twin.contains("todo!"),
        "{TWIN} opts itself out"
    );
}

#[test]
fn no_gate_step_was_added_for_the_drill() {
    let gate = read(GATE);
    // The falsifying check is the existing `"tests"` step. A story that adds
    // one has misread the substrate.
    assert!(
        gate.contains("name: \"tests\","),
        "the `\"tests\"` REQUIRED step is gone"
    );
    let allowances = read("xtask/src/lint_narrative.rs");
    assert!(
        allowances.contains("const IGNORE_ALLOWANCES: &[(&str, &str, &str)] = &[];"),
        "the allowance list gained an entry; this project ships zero opted-out \
         fences"
    );
}

// ---------------------------------------------------------------------------
// AC-005 and AC-006 — the transcript exists, and the page quotes it
// ---------------------------------------------------------------------------

#[test]
fn the_transcript_records_both_directions() {
    let record = read(OBSERVATION);
    for required in [
        "## The starting tree",
        "## Direction one — the boundary removed",
        "## Direction two — the boundary restored",
    ] {
        assert!(
            record.contains(required),
            "{OBSERVATION} is missing `{required}`; a transcript missing either \
             direction fails the criterion"
        );
    }
    assert!(
        record.contains("git status --porcelain"),
        "{OBSERVATION} does not record the closing clean-tree check"
    );
}

#[test]
fn the_pages_quoted_failure_is_byte_equal_to_the_recorded_one() {
    let page = read(PAGE);
    let record = read(OBSERVATION);
    let body = drill(&page);

    // Every line inside the drill's quoted blocks must appear, byte for byte,
    // in the transcript. This is the check that catches a failure written from
    // imagination and reconciled afterwards.
    let mut fenced = false;
    let mut quoted = 0;
    for line in &body {
        if line.starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if !fenced || line.trim().is_empty() {
            continue;
        }
        quoted += 1;
        assert!(
            record.contains(line.trim_end()),
            "the drill quotes a line the recorded run never produced: {line}"
        );
    }
    assert!(quoted > 0, "the drill quotes no output at all");
}

#[test]
fn the_drill_neither_uses_nor_suggests_emptying_the_query() {
    let page = read(PAGE);
    let body = drill(&page).join("\n");
    assert!(
        !body.contains("from_items([])"),
        "the drill suggests emptying the query, which returns \
         `Err(InvalidQuery::NoItems)` and goes red for a constructor reason the \
         reader cannot tell from a build error"
    );
}

#[test]
fn the_quoted_failure_is_copy_faithful() {
    let page = read(PAGE);
    let body = drill(&page);
    let mut fenced = false;
    for line in &body {
        if line.starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if !fenced {
            continue;
        }
        assert!(
            !line.contains('…') && !line.contains("..."),
            "the quoted output is elided: {line}"
        );
        assert!(
            !line.starts_with("$ ") || !line.contains("&&"),
            "a chained shell line is not what the reader runs: {line}"
        );
    }
}

// ---------------------------------------------------------------------------
// AC-007 — persistent, no second subject, no second answered-need
// ---------------------------------------------------------------------------

#[test]
fn the_drill_is_persistent_and_introduces_nothing() {
    let page = read(PAGE);
    let body = drill(&page).join("\n").to_lowercase();
    for control in ["<details", "<summary", "<script", "<style", "!["] {
        assert!(
            !body.contains(control),
            "the drill introduces `{control}`; it is classified Persistent and \
             the reader must not have to click before the edit is legible"
        );
    }
    for word in ["aggregate", "one stream per entity", "which stream"] {
        assert!(
            !body.contains(word),
            "the drill names the prior model: {word}"
        );
    }
    assert!(
        !drill(&page).join("\n").contains("MUST"),
        "the drill states a rule in its own words"
    );
    assert!(
        !body.contains("**answers:**"),
        "the drill adds a second answered-need; the page already carries one"
    );
    assert!(
        !body.contains("happenstance_core"),
        "the drill names the contract crate; the taught vocabulary is the facade"
    );
}
