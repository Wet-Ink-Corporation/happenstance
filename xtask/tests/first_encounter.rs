//! Source-reading assertions over the opening encounter, and over the crate
//! root's half of it.
//!
//! No compiler runs here, and that is the point. The doctests under
//! `cargo test -p xtask --doc` prove the three programs *work*; nothing in that
//! run can see whether the page a reader opens is composed the way the
//! signed-off design fixed it — a page whose fences all pass and whose steps
//! open on a wall of prose, hide the boundary behind `#`, or quietly opt a
//! fence out of the compiler is green under every other check in this
//! repository. These assertions are the ones an unstyled render fails.
//!
//! The numbers are the signed-off design's
//! (`.bklg/docs-that-teach/application-author-path/_design.md`,
//! `## Density budget`), and the composition is its `## Composition`.

use std::path::{Path, PathBuf};

/// The opening encounter, repo-relative.
const PAGE: &str = "docs/first-encounter.md";
/// The narrative tree's index, which is what makes the page reachable.
const INDEX: &str = "docs/README.md";
/// The harness whose `include_str!` line is the page's mount.
const HARNESS: &str = "xtask/src/narrative.rs";
/// The crate root, the second surface this slice writes on.
const CRATE_ROOT: &str = "crates/happenstance/src/lib.rs";

/// A fence line, in columns. 72 is where `overflow-x` engages on the 696px
/// fence at 1024x768; 68 leaves four columns of headroom.
const FENCE_COLUMNS: usize = 68;
/// A step fence, in rendered lines.
const FENCE_LINES: usize = 24;
/// The crate root's landing fence, in rendered lines. The design exempts the
/// crate-root encounter from [`FENCE_LINES`] at this ceiling, because the
/// whole program has to fit on one page.
const LANDING_FENCE_LINES: usize = 32;
/// A paragraph, in characters — five rendered lines at 764px.
const PARAGRAPH: usize = 435;

/// The three step headings, in reading order, with their emitted anchors.
const STEPS: [(&str, &str); 3] = [
    ("## Append and read back", "#append-and-read-back"),
    ("## A condition that holds", "#a-condition-that-holds"),
    ("## A condition that refuses", "#a-condition-that-refuses"),
];

/// The words the prior model is named in. It is named on the bridge's page and
/// on the background pages a newcomer reads first, and never on this one.
const PRIOR_MODEL: [&str; 4] = [
    "aggregate",
    "your aggregates",
    "one stream per entity",
    "which stream",
];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask sits one level below the repository root")
        .to_path_buf()
}

/// Read a repo-relative file with its line endings normalised to `\n`.
///
/// This repository is developed on Windows with `core.autocrlf = true`, so a
/// freshly checked-out file is CRLF and every `starts_with`/`find` below would
/// answer a different question on a clean clone than in an editor's buffer.
fn read(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel))
        .unwrap_or_else(|_| panic!("{rel} is readable"))
        .replace("\r\n", "\n")
}

/// One fenced block: its info string, its 1-based opening line, and its body.
struct Fence {
    info: String,
    line: usize,
    body: Vec<String>,
}

/// Every fenced block on a page, in source order.
fn fences(text: &str) -> Vec<Fence> {
    let mut out: Vec<Fence> = Vec::new();
    let mut open: Option<Fence> = None;
    for (index, line) in text.lines().enumerate() {
        let Some(rest) = line.strip_prefix("```") else {
            if let Some(fence) = open.as_mut() {
                fence.body.push(line.to_owned());
            }
            continue;
        };
        match open.take() {
            Some(fence) => out.push(fence),
            None => {
                open = Some(Fence {
                    info: rest.trim().to_owned(),
                    line: index + 1,
                    body: Vec::new(),
                });
            }
        }
    }
    assert!(open.is_none(), "a fence is opened and never closed");
    out
}

/// The lines of one step, from its heading to the next `## ` or the page end.
fn step(text: &str, heading: &str) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    let at = lines
        .iter()
        .position(|line| line.trim_end() == heading)
        .unwrap_or_else(|| panic!("`{heading}` is not a heading on {PAGE}"));
    lines[at..]
        .iter()
        .enumerate()
        .take_while(|(offset, line)| *offset == 0 || !line.starts_with("## "))
        .map(|(_, line)| (*line).to_owned())
        .collect()
}

/// Every `> **Answers:**` declaration line in a body of text, 1-based.
///
/// The crate root spells its doc lines with a `//! ` marker, so the marker is
/// stripped before the blockquote is looked for.
fn declarations(text: &str) -> Vec<(usize, String)> {
    text.lines()
        .enumerate()
        .map(|(index, line)| {
            let body = line
                .trim_start()
                .strip_prefix("//!")
                .map_or(line, |rest| rest.strip_prefix(' ').unwrap_or(rest));
            (index + 1, body.trim().to_owned())
        })
        .filter(|(_, body)| body.starts_with("> **Answers:**"))
        .collect()
}

// ---------------------------------------------------------------------------
// AC-007 — one answered-need, above everything, and above the first fence
// ---------------------------------------------------------------------------

#[test]
fn the_page_declares_exactly_one_need_above_its_first_fence() {
    let page = read(PAGE);
    let found = declarations(&page);
    assert_eq!(
        found.len(),
        1,
        "the page declares {} needs, not one: {found:?}",
        found.len()
    );

    let (line, body) = &found[0];
    // Immediately after the H1, with one blank line between and nothing else
    // interposed: RP-00-2's position is half of what the line means.
    let lines: Vec<&str> = page.lines().collect();
    assert!(
        lines[0].starts_with("# ") && lines[1].trim().is_empty(),
        "the page does not open on an H1 followed by a blank line"
    );
    assert_eq!(*line, 3, "the declaration is not the line after the H1");

    let token = body
        .strip_prefix("> **Answers:** `")
        .and_then(|rest| rest.split('`').next())
        .unwrap_or_default();
    assert!(
        ["orientation", "tutorial", "how-to", "explanation"].contains(&token),
        "`{token}` is not a member of the closed need set"
    );
    assert!(
        body.ends_with('?'),
        "the declaration's clause is not a question"
    );

    let first_fence = fences(&page)
        .first()
        .map(|fence| fence.line)
        .expect("the page carries a fence");
    assert!(
        *line < first_fence,
        "the answered-need line sits below the first fence"
    );
}

#[test]
fn the_crate_root_declares_its_one_need_above_its_first_fence() {
    let root = read(CRATE_ROOT);
    let found = declarations(&root);
    assert_eq!(
        found.len(),
        1,
        "the crate root declares {} needs, not one: {found:?}",
        found.len()
    );

    let doc: Vec<(usize, &str)> = root
        .lines()
        .enumerate()
        .filter(|(_, line)| line.trim_start().starts_with("//!"))
        .map(|(index, line)| (index + 1, line))
        .collect();
    let fence = doc
        .iter()
        .find(|(_, line)| line.contains("//! ```"))
        .map(|(at, _)| *at)
        .expect("the crate root carries a fence");
    assert!(
        found[0].0 < fence,
        "the crate root's answered-need line sits below its first fence"
    );
}

// ---------------------------------------------------------------------------
// AC-003 — a cold arrival lands on the header block, and on a whole program
// ---------------------------------------------------------------------------

#[test]
fn every_later_step_opens_on_its_two_line_header_block() {
    let page = read(PAGE);

    for (heading, _) in &STEPS[1..] {
        let body = step(&page, heading);
        let first = body
            .iter()
            .skip(1)
            .find(|line| !line.trim().is_empty())
            .unwrap_or_else(|| panic!("`{heading}` is empty"));
        assert!(
            first.starts_with("> "),
            "the first element under `{heading}` is not the header block: {first}"
        );

        // Two lines: the blockquote carries two paragraphs, so it renders as
        // two and not as one run-together sentence.
        let block: Vec<&String> = body
            .iter()
            .skip(1)
            .skip_while(|line| line.trim().is_empty())
            .take_while(|line| line.starts_with('>'))
            .collect();
        let breaks = block.iter().filter(|line| line.trim() == ">").count();
        assert_eq!(
            breaks, 1,
            "`{heading}`'s header block does not render as two lines: {block:?}"
        );

        // Line 2 is a one-hop link back to step one.
        let quoted = block
            .iter()
            .map(|line| line.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            quoted.contains(STEPS[0].1),
            "`{heading}`'s header block does not link step one: {quoted}"
        );
    }
}

#[test]
fn step_one_carries_the_answered_need_in_line_ones_place() {
    let page = read(PAGE);
    let body = step(&page, STEPS[0].0);
    let first = body
        .iter()
        .skip(1)
        .find(|line| !line.trim().is_empty())
        .expect("step one is not empty");
    assert!(
        !first.starts_with('>'),
        "step one carries a header block; the answered-need line above the page \
         is what stands in line 1's place, and line 2 is omitted"
    );
}

#[test]
fn every_step_fence_is_a_complete_program() {
    let page = read(PAGE);
    for (heading, _) in STEPS {
        let body = step(&page, heading).join("\n");
        let owned = fences(&body);
        let rust: Vec<&Fence> = owned.iter().filter(|fence| fence.info == "rust").collect();
        assert_eq!(
            rust.len(),
            1,
            "`{heading}` does not carry exactly one fence"
        );

        let program = rust[0].body.join("\n");
        for required in ["use happenstance::", "#[tokio::main]", "async fn main()"] {
            assert!(
                program.contains(required),
                "`{heading}`'s fence is a fragment: it never writes `{required}`"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// AC-001 and AC-004 — the refusal is the program's own output, and both the
// tag join and the `after` are load-bearing
// ---------------------------------------------------------------------------

#[test]
fn step_three_prints_the_refusal_from_inside_the_matched_arm() {
    let page = read(PAGE);
    let body = step(&page, STEPS[2].0);
    let owned = fences(&body.join("\n"));
    let program = owned
        .iter()
        .find(|fence| fence.info == "rust")
        .map(|fence| fence.body.join("\n"))
        .expect("step three carries a fence");

    let arm = program
        .find("Err(AppendError::ConditionViolated(_))")
        .expect("step three never matches the refusal");
    let print = program.find("println!").expect("step three prints nothing");
    assert!(
        print > arm,
        "the print is not reached from inside the matched refusal arm, so the \
         line would appear whether or not the store refused"
    );

    // The output block sits directly beneath the fence and carries the token,
    // because `Display` on `ConditionViolated` never does.
    let output = owned
        .iter()
        .find(|fence| fence.info == "text")
        .map(|fence| fence.body.join("\n"))
        .expect("step three carries an output block");
    assert!(
        output.contains("ConditionViolated"),
        "step three's output block does not carry the token: {output}"
    );
}

#[test]
fn the_racing_append_carries_the_tag_the_guards_query_joins_on() {
    let page = read(PAGE);
    let body = step(&page, STEPS[2].0).join("\n");
    let owned = fences(&body);
    let program = owned
        .iter()
        .find(|fence| fence.info == "rust")
        .map(|fence| fence.body.join("\n"))
        .expect("step three carries a fence");

    // An untagged racing append never matches a tagged query item, so the
    // condition is never violated and the scenario refuses nothing while
    // appearing to. This is the measured trap, stated as a check.
    assert!(
        program.contains(".with_tags(held)"),
        "the racing append carries no tags, so the guard's query cannot match it"
    );
    assert!(
        !program.contains("Query::all()") && !program.contains("Tags::empty()"),
        "the guard is broadened, so the scenario would refuse for the wrong reason"
    );
}

/// Step 3's guard stands on a position step 3's own read observed.
///
/// **This assertion exists because the one it replaces could not fail.** It
/// asserted the source substring `after_opt(upto)`, over a program that read an
/// **empty** store: `upto` was `None`, and `AppendCondition::new(q)` already
/// carries `after: None` (`crates/happenstance-core/src/append.rs`, `Guard`),
/// so the call was inert — delete it and the step-3 doctest still passed.
///
/// And it cannot be fixed by asserting the criterion's own falsification
/// recipe, because that recipe is unsatisfiable by construction: `after: None`
/// checks the **whole log**, which is strictly *stronger* than any `after`, so
/// removing the call can only ever tighten the guard — never turn a refusal
/// into an acceptance. Recorded and routed as **BC-004** in
/// `boundary-refusal-encounter/_conditions.md`.
///
/// What is left is the property a reader must actually get right, and it is
/// falsifiable in both directions: the `after` is the position **this
/// program's own read observed**, taken before the race and carried into the
/// guard. Guard on a position read *after* the race and the store admits the
/// append — the lost update, verified red. Read an empty store and there is no
/// position to be wrong about, which is the program that shipped and which this
/// assertion fails.
#[test]
fn step_three_guards_on_a_position_its_own_read_observed() {
    let page = read(PAGE);
    let body = step(&page, STEPS[2].0).join("\n");
    let owned = fences(&body);
    let program = owned
        .iter()
        .find(|fence| fence.info == "rust")
        .map(|fence| fence.body.join("\n"))
        .expect("step three carries a fence");

    // The store holds a matching event before the decision reads it, so the
    // read observes a real position rather than `None`.
    let seeded = program
        .find(", None).await")
        .expect("step three never lands an event of its own");
    let read_back = program
        .find("read_decision_model(")
        .expect("step three never reads");
    assert!(
        seeded < read_back,
        "step three reads an empty store, so `upto` is None and `after_every_guard_opt(upto)` \
         is inert — the guard it builds is indistinguishable from the whole-log \
         guard `AppendCondition::new` already carries"
    );

    // The racing writer lands between the read and the guarded append, so the
    // event that violates the guard is strictly above the observed position.
    let racing = program[read_back..]
        .find(", None).await")
        .map(|at| read_back + at)
        .expect("nothing lands between the read and the guarded append");
    let guarded = program
        .find("Some(&condition)")
        .expect("step three never makes the guarded append");
    assert!(
        read_back < racing && racing < guarded,
        "the racing append does not sit between the read and the guarded append, \
         so nothing is above the position the guard was built from"
    );
    assert!(
        program.contains("after_every_guard_opt(upto)"),
        "the condition is not built from the position the read observed"
    );

    // And the reader can see that position: it is in the program's own output,
    // so an append refused above a real boundary is distinguishable on screen
    // from one refused because the guard was whole-log all along.
    let output = owned
        .iter()
        .find(|fence| fence.info == "text")
        .map(|fence| fence.body.join("\n"))
        .expect("step three carries an output block");
    assert!(
        output.contains("Some(SequencePosition("),
        "step three's output never shows the position it guarded on: {output}"
    );
}

// ---------------------------------------------------------------------------
// AC-005 — the boundary is rebuildable from what renders
// ---------------------------------------------------------------------------

#[test]
fn no_hidden_line_carries_any_part_of_the_boundary() {
    let page = read(PAGE);
    for fence in fences(&page) {
        if fence.info != "rust" {
            continue;
        }
        for (offset, line) in fence.body.iter().enumerate() {
            assert!(
                !line.trim_start().starts_with("# "),
                "{PAGE}:{}: a hidden line — rustdoc removes it from the DOM \
                 entirely, with no hover, focus or toggle that recovers it",
                fence.line + offset + 1
            );
        }
    }
}

// ---------------------------------------------------------------------------
// AC-006 — the page is mounted, and no fence opts out
// ---------------------------------------------------------------------------

#[test]
fn the_page_is_registered_in_the_harness_and_indexed() {
    let harness = read(HARNESS);
    assert!(
        harness.contains(r#"include_str!("../../docs/first-encounter.md")"#),
        "{HARNESS} does not include the page, so nothing compiles its fences"
    );
    assert!(
        harness.contains("mod first_encounter {"),
        "{HARNESS} does not register the page under its own module, so a failure \
         would not name the page"
    );

    let index = read(INDEX);
    assert!(
        index.contains("(first-encounter.md)"),
        "{INDEX} does not route to the page, so it is compiled but unreachable"
    );
}

#[test]
fn no_fence_on_the_page_opts_out_of_the_compiler() {
    let page = read(PAGE);
    for fence in fences(&page) {
        assert!(
            fence.info == "rust" || fence.info == "text",
            "{PAGE}:{}: the fence info string is `{}`; the tags this page uses \
             are `rust` and `text`, and nothing on it is exempt",
            fence.line,
            fence.info
        );
    }
    for opt_out in ["ignore", "no_run", "compile_fail"] {
        assert!(
            !page.contains(&format!("```{opt_out}")) && !page.contains(&format!("rust,{opt_out}")),
            "{PAGE} carries a `{opt_out}` fence"
        );
    }
}

// ---------------------------------------------------------------------------
// AC-007 — the density budget, the vocabulary, and zero affordances
// ---------------------------------------------------------------------------

#[test]
fn every_fence_holds_the_density_budget() {
    let page = read(PAGE);
    for fence in fences(&page) {
        if fence.info != "rust" {
            // The captured output block is quoted as it came out of the runner;
            // re-wrapping it would make the page lie about what a reader sees.
            continue;
        }
        assert!(
            fence.body.len() <= FENCE_LINES,
            "{PAGE}:{}: the fence is {} rendered lines, over {FENCE_LINES}",
            fence.line,
            fence.body.len()
        );
        for (offset, line) in fence.body.iter().enumerate() {
            let columns = line.chars().count();
            assert!(
                columns <= FENCE_COLUMNS,
                "{PAGE}:{}: a fence line is {columns} columns, over \
                 {FENCE_COLUMNS}, so it scrolls at 1024x768",
                fence.line + offset + 1
            );
        }
    }
}

/// The crate root's landing fence holds the same width budget as a step
/// fence and its own height ceiling, counted as rendered: a `# ` line is
/// hidden by rustdoc, so it costs nothing (#234).
#[test]
fn the_crate_roots_landing_fence_holds_its_budget() {
    let root = read(CRATE_ROOT);
    let doc: Vec<&str> = root
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("//!"))
        .map(|line| line.strip_prefix(' ').unwrap_or(line))
        .collect();
    let rendered: Vec<&str> = doc
        .iter()
        .skip_while(|line| !line.starts_with("```"))
        .skip(1)
        .take_while(|line| !line.starts_with("```"))
        .filter(|line| !line.starts_with("# ") && **line != "#")
        .copied()
        .collect();

    assert!(
        !rendered.is_empty(),
        "{CRATE_ROOT} carries no landing fence, so this test proves nothing"
    );
    assert!(
        rendered.len() <= LANDING_FENCE_LINES,
        "{CRATE_ROOT}: the landing fence renders {} lines, over {LANDING_FENCE_LINES}",
        rendered.len()
    );
    for line in rendered {
        let columns = line.chars().count();
        assert!(
            columns <= FENCE_COLUMNS,
            "{CRATE_ROOT}: a landing fence line is {columns} columns, over \
             {FENCE_COLUMNS}, so it scrolls at 1024x768: {line}"
        );
    }
}

#[test]
fn every_paragraph_holds_its_budget() {
    let page = read(PAGE);
    let mut fenced = false;
    let mut paragraph = String::new();
    for line in page.lines() {
        if line.starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        if line.trim().is_empty() {
            assert!(
                paragraph.chars().count() <= PARAGRAPH,
                "a paragraph is {} characters, over {PARAGRAPH}: {paragraph}",
                paragraph.chars().count()
            );
            paragraph.clear();
            continue;
        }
        paragraph.push_str(line.trim());
        paragraph.push(' ');
    }
}

#[test]
fn neither_surface_names_the_prior_model() {
    for surface in [PAGE, CRATE_ROOT] {
        let text = read(surface).to_lowercase();
        for word in PRIOR_MODEL {
            assert!(
                !text.contains(word),
                "{surface} names the prior model (`{word}`); it is named on \
                 the bridge's page and the background pages, never on this one"
            );
        }
    }
}

#[test]
fn the_page_introduces_no_affordance() {
    let page = read(PAGE);
    for control in ["<details", "<summary", "<script", "<style", "<img", "!["] {
        assert!(
            !page.to_lowercase().contains(control),
            "{PAGE} introduces `{control}`; the correct count of affordances \
             this project introduces is none"
        );
    }
}

#[test]
fn every_fence_imports_from_the_facade_and_binds_the_weaker_trait() {
    let page = read(PAGE);
    for fence in fences(&page) {
        if fence.info != "rust" {
            continue;
        }
        let program = fence.body.join("\n");
        assert!(
            !program.contains("happenstance_core"),
            "{PAGE}:{}: the fence imports the contract crate; the taught \
             vocabulary is `use happenstance::{{…}}`",
            fence.line
        );
        assert!(
            !program.contains("SendEventStore"),
            "{PAGE}:{}: the fence binds `SendEventStore`; generic code binds \
             `EventStore`, and the two names may not share a scope",
            fence.line
        );
    }
}

// ---------------------------------------------------------------------------
// AC-008 — normative weight is a citation, never a restatement
// ---------------------------------------------------------------------------

/// The clause citation is the last element of every step's **own** material.
///
/// "Own material" is the six budgeted elements, which stop at the first `###`:
/// the design composes the falsification drill as step 3's one declared
/// additional element, sitting *after* the citation
/// (`_design.md` `## Composition`, "Step 3 additionally carries, after item 6…").
/// Reading to the end of the step instead would make this assertion demand the
/// citation come after the drill, which is the opposite of what is signed off —
/// and `xtask/tests/falsification_drill.rs::the_drill_is_a_subsection_at_the_bottom_of_step_three`
/// is what holds the drill below the citation from the other side.
#[test]
fn every_step_closes_on_a_clause_citation() {
    let page = read(PAGE);
    for (heading, _) in STEPS {
        let body = step(&page, heading);
        let own: Vec<&String> = body
            .iter()
            .take_while(|line| !line.starts_with("### "))
            .collect();
        let last = own
            .iter()
            .rev()
            .find(|line| !line.trim().is_empty())
            .unwrap_or_else(|| panic!("`{heading}` is empty"));
        assert!(
            last.contains("](../spec/SPECIFICATION.md#"),
            "`{heading}` does not close on a clause citation: {last}"
        );
    }
}

/// Step 2 cites ES-26, so step 2's program must reach what ES-26 is about.
///
/// `every_step_closes_on_a_clause_citation` above only requires a *link*, and
/// `cargo xtask spec-trace` only requires the *id to resolve*. Neither can see
/// the defect this initiative was seeded by — a sentence claiming semantics
/// over a program that only demonstrates syntax — and this page is the one
/// written to close it. ES-26 is the **exclusivity** of `after`: an event at
/// exactly `after` does not reject. A program that guards an *empty* store
/// never reaches it, because `after_every_guard_opt(None)` means "no matching event at
/// all" (ES-25's other half) and no position is ever compared. That is the
/// wrong implementation this rule rejects, and it is the one the page shipped
/// with until this check existed.
#[test]
fn step_two_reaches_the_boundary_its_citation_claims() {
    let page = read(PAGE);
    let text = step(&page, STEPS[1].0).join("\n");
    assert!(
        text.contains("#es-26--"),
        "step two no longer cites ES-26; this assertion is what ties that \
         citation to the program, so it must move with the citation rather \
         than be left asserting a clause the step stopped claiming"
    );

    let owned = fences(&text);
    let program = owned
        .iter()
        .find(|fence| fence.info == "rust")
        .map(|fence| fence.body.join("\n"))
        .expect("step two carries a fence");

    // The store is seeded before it is read, so the read observes a matching
    // event and `upto` is `Some(...)`.
    let seeded = program
        .find(", None).await")
        .expect("step two never lands an event of its own");
    let read_back = program
        .find("read_decision_model(")
        .expect("step two never reads");
    assert!(
        seeded < read_back,
        "step two reads an empty store, so `after` is None and the \
         event-at-exactly-`after` boundary ES-26 states is never reached"
    );

    // The guard is built from that observed position, and the guarded append
    // is admitted rather than refused — both halves of what the prose claims.
    assert!(
        program.contains("after_every_guard_opt(upto)") && program.contains("Some(&condition)"),
        "step two does not guard on the position its own read observed"
    );

    // And the reader can see it: the output block names that position, so the
    // boundary is on screen rather than only in the sentence beneath it.
    let output = owned
        .iter()
        .find(|fence| fence.info == "text")
        .map(|fence| fence.body.join("\n"))
        .expect("step two carries an output block");
    assert!(
        output.contains("Some(SequencePosition("),
        "step two's output never shows the position it guarded on, so a reader \
         cannot tell an admitted append at a real boundary from one that had \
         no boundary to clear: {output}"
    );
}

#[test]
fn no_sentence_states_a_rule_in_the_pages_own_words() {
    let page = read(PAGE);
    for (index, line) in page.lines().enumerate() {
        if !line.contains("MUST") {
            continue;
        }
        assert!(
            line.contains("](../spec/SPECIFICATION.md#"),
            "{PAGE}:{}: a `MUST` sentence that is not a clause link: {line}",
            index + 1
        );
    }
}

// ---------------------------------------------------------------------------
// #236 — the crate root's fences, which no narrative step reads
// ---------------------------------------------------------------------------
//
// Appended rather than placed beside the page's attribute test above, because
// ADR-0054 cites this file by line, `first_encounter.rs:427` and a bare `:696`
// for the check now at `:739`; lint-kb repoints only the first, so insert below.

/// The crate root's doc comments as markdown, one entry per source line.
///
/// A line that is not a doc comment becomes empty rather than being dropped,
/// so a fence's reported line is the line in the file.
fn crate_root_docs() -> String {
    read(CRATE_ROOT)
        .lines()
        .map(|line| {
            let line = line.trim_start();
            line.strip_prefix("//!")
                .or_else(|| line.strip_prefix("///"))
                .map_or("", |doc| doc.strip_prefix(' ').unwrap_or(doc))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// The `CommonMark` fence rule lives in `xtask::fence`, shared with
// `carry_your_invariant.rs`, `lint-narrative` and the site renderer (#279,
// #294). It is imported here rather than at the top because ADR-0054 cites
// this file by line. `fences` above keeps its backtick-only, unindented
// reading, which the page's tests were written against.
use xtask::fence::{self, Opener};

/// Every fence opener in `docs`, as `(line, info)`, 1-based.
///
/// A fence closes only by [`fence::Delimiter::closes`], so a `~~~` line
/// inside a backtick fence is body rather than a new opener.
fn doc_fence_openers(docs: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut open: Option<Opener> = None;
    for (index, line) in docs.lines().enumerate() {
        let Some(found) = fence::delimiter(line) else {
            continue;
        };
        match open {
            Some(opener) if found.closes(opener) => {
                open = None;
            }
            Some(_) => {}
            None => {
                out.push((index + 1, found.info.to_owned()));
                open = Some(found.opener());
            }
        }
    }
    assert!(open.is_none(), "a doc fence is opened and never closed");
    out
}

/// Every fence in `docs` whose info string opts it out of running, as
/// `(line, info)`, 1-based.
fn opted_out(docs: &str) -> Vec<(usize, String)> {
    doc_fence_openers(docs)
        .into_iter()
        .filter(|(_, info)| !(info.is_empty() || info == "rust"))
        .collect()
}

/// The crate root is outside `cargo xtask narrative`'s tree, which is `docs/`
/// and nothing else (#236). Its landing fence is the first program a docs.rs
/// reader runs, so the attribute rule the page above is held to is held here
/// too: every fence compiles **and runs**. An `ignore`, `no_run` or
/// `compile_fail` tag on it would leave every other step green.
#[test]
fn no_fence_in_the_crate_roots_docs_opts_out_of_running() {
    let docs = crate_root_docs();
    assert!(
        !doc_fence_openers(&docs).is_empty(),
        "{CRATE_ROOT} carries no doc fence, so this test proves nothing"
    );
    let exempt = opted_out(&docs);
    assert!(
        exempt.is_empty(),
        "{CRATE_ROOT}: fences opted out as (line, info string) {exempt:?}; a \
         crate-root example is compiled and run, and nothing in it is exempt"
    );
}

/// rustdoc reads doc comments as `CommonMark`, which opens a fence with three
/// or more backticks **or tildes**, indented by up to three spaces, and
/// compiles what is inside either. `lint_narrative`'s
/// `a_fence_indented_up_to_three_spaces_is_still_a_fence` and
/// `a_tilde_fence_is_a_fence` record the same two ways past a walk that only
/// strips a leading `` ``` ``; this is the crate root's copy of both.
#[test]
fn a_tilde_or_indented_opt_out_in_the_crate_root_is_caught() {
    for (docs, info) in [
        ("~~~ignore\nfn f() {}\n~~~\n", "ignore"),
        ("~~~~rust,no_run\nfn f() {}\n~~~~\n", "rust,no_run"),
        ("   ```compile_fail\nfn f() {}\n   ```\n", "compile_fail"),
        ("  ~~~ignore\nfn f() {}\n  ~~~\n", "ignore"),
    ] {
        assert_eq!(
            opted_out(docs),
            [(1, info.to_owned())],
            "an opted-out fence was not seen in {docs:?}"
        );
    }
}

/// A doc line that opens with an inline code span is prose, not a fence
/// (#294). Read as an opener, it would pair with the `ignore` fence's opener
/// as its closer, and the opt-out would never be seen.
#[test]
fn a_code_span_at_the_start_of_a_line_hides_no_opt_out() {
    let docs = "```x``` opens the paragraph.\n\n```ignore\nfn f() {}\n```\n";
    assert_eq!(opted_out(docs), [(3, "ignore".to_owned())]);
}

// ---------------------------------------------------------------------------
// #234 — the crate root's headings, against rustdoc's sidebar
// ---------------------------------------------------------------------------
//
// Appended for the same reason as #236's section above: ADR-0054 cites this
// file by line.

/// A crate-root heading, in characters. rustdoc lists the crate root's
/// headings in its 200px sidebar with `white-space: nowrap` and an ellipsis,
/// so a longer heading is cut mid-phrase in the page's only navigation
/// (`_design.md`, `## Density budget`).
///
/// **The budget applies to rustdoc crate roots only.** A markdown page under
/// `docs/` has no rustdoc sidebar: the guide site lists page titles in its
/// sidebar, and its "On this page" list wraps a long heading rather than
/// cutting it. So `docs/` headings are not held to this number, and
/// `docs/first-encounter.md`'s 24-character `## A condition that refuses`
/// stands. That is the owner's decision "Split by surface", 2026-10-12, on
/// issue #234.
///
/// Only `happenstance`'s crate root is held to it, because that is the page
/// `_design.md` measured and the one a newcomer lands on. The other published
/// crate roots carry longer headings and are not checked here.
const CRATE_ROOT_HEADING: usize = 22;

/// Every ATX heading in `docs` outside a fence, as `(line, text)`, 1-based.
///
/// A fence is walked by [`fence::delimiter`]'s `CommonMark` reading,
/// because a `# ` line inside a doctest is a hidden line rather than a
/// heading. A heading is up to three spaces of indentation, one to six `#`,
/// then a space or the end of the line; `#hashtag` is a paragraph.
fn doc_headings(docs: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut open: Option<Opener> = None;
    for (index, line) in docs.lines().enumerate() {
        if let Some(found) = fence::delimiter(line) {
            open = match open {
                Some(opener) if found.closes(opener) => None,
                Some(still) => Some(still),
                None => Some(found.opener()),
            };
            continue;
        }
        if open.is_some() {
            continue;
        }
        let unindented = line.trim_start_matches(' ');
        if line.len() - unindented.len() > 3 {
            continue;
        }
        let text = unindented.trim_start_matches('#');
        let level = unindented.len() - text.len();
        if (1..=6).contains(&level) && (text.is_empty() || text.starts_with(' ')) {
            out.push((index + 1, text.trim().to_owned()));
        }
    }
    out
}

/// Every heading in `docs` over [`CRATE_ROOT_HEADING`], as
/// `(line, text, characters)`.
fn headings_over_budget(docs: &str) -> Vec<(usize, String, usize)> {
    doc_headings(docs)
        .into_iter()
        .map(|(line, text)| {
            let characters = text.chars().count();
            (line, text, characters)
        })
        .filter(|(_, _, characters)| *characters > CRATE_ROOT_HEADING)
        .collect()
}

/// Every heading on the crate root fits rustdoc's sidebar (#234).
#[test]
fn every_crate_root_heading_fits_the_sidebar() {
    let docs = crate_root_docs();
    assert!(
        !doc_headings(&docs).is_empty(),
        "{CRATE_ROOT} carries no heading, so this test proves nothing"
    );
    let over = headings_over_budget(&docs);
    assert!(
        over.is_empty(),
        "{CRATE_ROOT}: headings over {CRATE_ROOT_HEADING} characters, as \
         (line, text, characters), which rustdoc's sidebar truncates: {over:?}"
    );
}

/// The two headings #234 retitled are rejected, at every level and indent
/// `CommonMark` allows; a walk that read a doctest's hidden `# ` line as a
/// heading, or `#hashtag` as one, would flag the inputs that must pass.
#[test]
fn an_over_long_crate_root_heading_is_caught() {
    for (docs, text) in [
        (
            "# What arrives here, and what stays below\n",
            "What arrives here, and what stays below",
        ),
        (
            "## Testing without a database\n",
            "Testing without a database",
        ),
        (
            "   ###### Testing without a database\n",
            "Testing without a database",
        ),
        // One character over the budget: the boundary is `>`, at 22.
        ("## Twenty-three characters\n", "Twenty-three characters"),
    ] {
        assert_eq!(
            headings_over_budget(docs),
            [(1, text.to_owned(), text.chars().count())],
            "an over-long heading was not seen in {docs:?}"
        );
    }
    for docs in [
        "# Testing in memory\n",
        // Exactly the budget, and 23 bytes: characters are counted, not bytes.
        "## Twenty-two caractères!\n",
        "```\n# let a_hidden_doctest_line_over_the_budget = 1;\n```\n",
        "~~~\n# a hidden line in a tilde fence, over the budget\n~~~\n",
        "#hashtag-is-a-paragraph-and-over-the-budget\n",
        "    # an indented code block line, over the budget\n",
        "####### seven hashes is a paragraph, over the budget\n",
    ] {
        let over = headings_over_budget(docs);
        assert!(
            over.is_empty(),
            "a line that is not a heading was flagged in {docs:?}: {over:?}"
        );
    }
}
