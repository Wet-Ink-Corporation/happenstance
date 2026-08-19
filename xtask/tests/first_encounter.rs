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
/// A paragraph, in characters — five rendered lines at 764px.
const PARAGRAPH: usize = 435;

/// The three step headings, in reading order, with their emitted anchors.
const STEPS: [(&str, &str); 3] = [
    ("## Append and read back", "#append-and-read-back"),
    ("## A condition that holds", "#a-condition-that-holds"),
    ("## A condition that refuses", "#a-condition-that-refuses"),
];

/// The words the prior model is named in. It is named on exactly one page in
/// the whole set — the bridge's — and that page is not this one.
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
fn the_racing_append_and_the_after_are_both_load_bearing() {
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
        program.contains("after_opt(upto)"),
        "the condition is not built from the position the read actually observed"
    );
    assert!(
        !program.contains("Query::all()") && !program.contains("Tags::empty()"),
        "the guard is broadened, so the scenario would refuse for the wrong reason"
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
                 exactly one page in the set, and this is not it"
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
