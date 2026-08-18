//! # What this does not verify
//!
//! Stated first, because a check whose limits are undocumented is read as a
//! guarantee — RS-81-1, `standards/rust/81-checks-that-cannot-be-types.md:11`,
//! and `lint_constitution` makes the same argument at greater length.
//!
//! * **It checks that a need is *declared*, never that the page *answers* it.**
//!   A page may declare `explanation` and carry a how-to underneath it, and
//!   nothing here sees that. The instrument for the rest is DR-07's reviewer
//!   procedure — the non-author walk in `standards/pages/40-reviewing-a-page.md`
//!   — and never a byte count.
//! * **It does not judge whether the set is the right set.** Membership is
//!   decided against `NEEDS`; whether those are the needs a narrative tree owes
//!   its readers is an argument, and the argument lives in
//!   `standards/pages/10-the-need-set.md` where a reader can disagree with it.
//! * **It does not enforce the fold line beyond textual markers.** DT-8's rule
//!   is applied by a reader. Whether a hidden branch sits inside the checked
//!   surface at all is HS-P0020's demonstration to earn, not this module's to
//!   assert.
//! * **It does not resolve clause ids.** Whether a cited clause exists is
//!   `spec_trace`'s answer and HS-P0020's `clause_ids`; a second parser here is
//!   forbidden. A page can cite a real, resolving id and restate its content in
//!   the paragraph underneath, and nothing mechanical sees that either.
//! * **An empty rules tree passes every check below the vacuity guard.** The
//!   guard is what makes a green run a statement about a corpus rather than
//!   about an empty directory, and it lands with the checker.
//! * **Length is not quality.** Every ceiling here is a length, exactly as
//!   `MIN_REJECTS_CHARS` and `MAX_ATOM_BYTES` are and for the same reason: a
//!   long and vacuous rule passes, and the instrument for that is an
//!   adversarial reader.
//!
//! # What this module holds
//!
//! The closed set of needs a governed page may declare, enumerated once, plus
//! the two path pins the rules tree is addressed by. The checker that reads a
//! page and the router region that lists the rules both take their answer from
//! here rather than re-deriving it, so a need cannot half-land in one list and
//! not the other.
//!
//! # The hosting assumption this rests on
//!
//! The declaration form the rules tree fixes — a first-line blockquote,
//! immediately after the page's title — assumes only that the medium renders
//! `CommonMark` blockquotes as visible body text in document order. It assumes
//! nothing about front matter, about directory-derived navigation, or about a
//! renderer's own metadata layer. A renderer that strips or relocates leading
//! blockquotes, or one that requires front matter, invalidates this decision and
//! **re-opens DR-05** — so a hosting choice that violates the assumption is a
//! visible re-opening rather than a silent contradiction.

#![allow(
    dead_code,
    reason = "the membership accessor, `Need::job` and the two path pins are \
              consumed by page-need-checker-mounted-in-the-gate, which deletes \
              this line in the change that mounts the checker"
)]

/// One need a governed page may declare.
///
/// This is the single place the need set is enumerated: the membership test
/// below takes its answer from here, the router's generated region takes its
/// rows from here, and `standards/pages/10-the-need-set.md` carries the same
/// tokens as a table a human argues with. Three lists that must agree, kept as
/// one so that adding a need cannot half-land.
///
/// The design's table has a third column — *success for the reader* — and it is
/// deliberately not a field. Both fields here have a named machine consumer; a
/// third that nothing reads would be dead code with a doc comment on it, so that
/// column stays prose in band 10.
struct Need {
    /// The token exactly as a page writes it in its declaration, between
    /// backticks. This is what the membership test compares.
    token: &'static str,
    /// The page's job, in one clause. This is the trigger cell the router's
    /// generated row prints beside the token.
    job: &'static str,
}

/// The closed need set, in the order the signed-off design renders it.
///
/// Four tokens: `reference` is subtracted because rustdoc and
/// `spec/SPECIFICATION.md` are already this workspace's two reference surfaces,
/// and `orientation` is added because routing is a need this tree's readers have
/// and the source taxonomy is silent about it. Band 10 carries the argument.
const NEEDS: &[Need] = &[
    Need {
        token: "orientation",
        job: "route the reader to the page that answers their question",
    },
    Need {
        token: "tutorial",
        job: "carry a newcomer through one working thing, staged",
    },
    Need {
        token: "how-to",
        job: "get a reader who already has a goal to that goal",
    },
    Need {
        token: "explanation",
        job: "build the mental model behind a behaviour",
    },
];

/// The most members the set may ever hold.
///
/// A closed set that grows past six is bucket proliferation — the failure the
/// need set was closed to avoid. The ceiling is a `const` assertion rather than a
/// `#[test]` on purpose: a test can be deleted by whoever is adding the seventh
/// token, and a `const` assertion fails at `cargo check` before a test runs.
const MAX_NEEDS: usize = 6;

const _: () = assert!(
    NEEDS.len() <= MAX_NEEDS,
    "the need set is closed. A page that strains against every token is \
     answering more than one need: split the page, never add a token."
);

/// The directory the rule atoms live in.
///
/// Pinned by path rather than by convention, and depended on by value: renaming
/// it later is not a rename, it is three edits in `xtask/src/main.rs`, an `INERT`
/// entry in `xtask/src/affected.rs` and two `affected` tests.
const RULE_DIR: &str = "standards/pages";

/// The rules tree's router, and the tree's only composition root.
///
/// Taken as a forward pin by `need-vocabulary-and-declaration-form`, which
/// asserted the file's *absence* and named the story that would invert the
/// assertion rather than delete it quietly.
/// `router-precedence-and-announcement` created the file and performed that
/// inversion; `tests::router_is_created_by_the_router_story` is what the old
/// assertion became.
const ROUTER: &str = "standards/pages/README.md";

/// The one place a token is judged a member of [`NEEDS`].
///
/// Exact and case-sensitive: no trimming, no case folding, no aliasing and no
/// plural form. The checker calls this rather than re-deriving the set, because
/// two parsers that agree today drift tomorrow.
fn need(token: &str) -> Option<&'static Need> {
    NEEDS.iter().find(|candidate| candidate.token == token)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, reason = "test code, per the house style")]

    use std::path::PathBuf;

    use super::*;

    /// This module's own source, read at compile time.
    ///
    /// `include_str!` resolves relative to *this file*, so it needs no path
    /// constant and no dependency, and it turns "the module docs say X, and say
    /// it first" into a compiled assertion. There is no in-repo precedent for a
    /// module reading itself, which is why it is spelled out here rather than
    /// left to be inferred.
    const THIS_FILE: &str = include_str!("lint_pages.rs");

    /// Band 00 — one need per page, and the declaration's grammar.
    const BAND_00: &str = "00-one-need.md";

    /// Band 10 — the closed need set and the two `orientation` ceilings.
    const BAND_10: &str = "10-the-need-set.md";

    /// Band 20 — the fold line: the deletion test, the closed never-fold list,
    /// and the permission gate that ships empty.
    const BAND_20: &str = "20-the-fold-line.md";

    /// Words a rule may never use, because each hands the reader back the
    /// judgement the rule exists to replace.
    ///
    /// `_design.md` anti-pattern 15. Matched case-insensitively over whole
    /// atoms: a walk step that says "consider" yields an impression, and two
    /// strangers reading it reach two verdicts.
    const HEDGES: [&str; 5] = [
        "consider",
        "use judgement",
        "use judgment",
        "as appropriate",
        "if it seems",
    ];

    /// The atoms filed into the tree, in filename order.
    ///
    /// A deliberate list rather than a `read_dir`: the directory-reading corpus
    /// reader belongs to `page-need-checker-mounted-in-the-gate`, and a second
    /// one here is the duplication this slice's ordering exists to prevent.
    /// Each story in the slice appends its own band as that band lands, which
    /// is what keeps the router's generated region a *derived* region rather
    /// than a hand-maintained one.
    const TREE: &[&str] = &[BAND_00, BAND_10, BAND_20];

    /// The five section markers a rule carries, in the order they must appear.
    ///
    /// The same list, in the same order, as `lint_constitution::SECTIONS`
    /// (`xtask/src/lint_constitution.rs:67-81`). Deliberately a second copy and
    /// not a shared import: RS-81-3 scopes a scanner to the directory whose
    /// behaviour it constrains, and a shared constant would make one failure
    /// message answer two trees' questions.
    const SECTIONS: [&str; 5] = [
        "**Why.**",
        "**Do**",
        "**Not**",
        "**Rejects.**",
        "**Evidence.**",
    ];

    fn root() -> PathBuf {
        crate::spec_trace::workspace_root().unwrap()
    }

    /// One named atom under [`RULE_DIR`], read whole.
    ///
    /// Named paths only, never `read_dir`: the directory-reading corpus reader
    /// belongs to `page-need-checker-mounted-in-the-gate`, and building a second
    /// one here is the duplication this slice's ordering exists to prevent.
    fn atom(file: &str) -> String {
        let path = root().join(RULE_DIR).join(file);
        std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("reading {}: {err}", path.display()))
    }

    /// The rules tree's router, read whole.
    fn router() -> String {
        let path = root().join(ROUTER);
        std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("reading {}: {err}", path.display()))
    }

    /// The `Load when:` triggers of an atom, or the empty string.
    ///
    /// Mirrors `lint_constitution::load_when` (`xtask/src/lint_constitution.rs:245-257`)
    /// deliberately, including its limit: only the **first** source line of the
    /// block is read, and a continuation on the next `>` line is silently
    /// dropped. The router's `## The shape of a rule` states the one-source-line
    /// constraint precisely because this is what builds the index cell.
    fn load_when(text: &str) -> String {
        for line in text.lines() {
            let trimmed = line.trim_start_matches(['>', ' ']);
            if let Some(rest) = trimmed.strip_prefix("**Load when:**") {
                return rest.trim().to_owned();
            }
        }
        String::new()
    }

    /// The row the generator would emit for one atom.
    ///
    /// Byte-for-byte `lint_constitution::generated_region`'s per-atom format
    /// (`xtask/src/lint_constitution.rs:400-420`): a markdown link to the file,
    /// the first `Load when` source line with every interior `|` escaped (or an
    /// em dash when absent), then the comma-separated rule ids. Derived from the
    /// real atom rather than written down, so the checker's first `--write`
    /// against this router can only produce no diff.
    fn expected_index_row(file: &str) -> String {
        let text = atom(file);
        let trigger = load_when(&text);
        let ids: Vec<String> = rules(&text).into_iter().map(|(id, _)| id).collect();
        format!(
            "| [`{file}`]({file}) | {} | {} |",
            if trigger.is_empty() {
                "—".to_owned()
            } else {
                trigger.replace('|', "\\|")
            },
            ids.join(", ")
        )
    }

    /// The lines strictly between the generated-region markers of a file.
    fn generated_region(text: &str) -> Vec<String> {
        let lines: Vec<&str> = text.lines().collect();
        let start = lines
            .iter()
            .position(|line| line.trim() == "<!-- BEGIN GENERATED -->")
            .unwrap_or_else(|| panic!("no `<!-- BEGIN GENERATED -->` marker"));
        let end = lines
            .iter()
            .position(|line| line.trim() == "<!-- END GENERATED -->")
            .unwrap_or_else(|| panic!("no `<!-- END GENERATED -->` marker"));
        assert!(start < end, "the region's markers are out of order");
        lines[start + 1..end]
            .iter()
            .map(|line| (*line).to_owned())
            .collect()
    }

    /// The rows of the first markdown table whose header line is `header`.
    fn table_rows(text: &str, header: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut inside = false;
        for line in text.lines() {
            if line.starts_with(header) {
                inside = true;
                continue;
            }
            if !inside {
                continue;
            }
            if !line.starts_with('|') {
                break;
            }
            if line.starts_with("| ---") || line.starts_with("|---") {
                continue;
            }
            out.push(line.to_owned());
        }
        out
    }

    /// This module's `//!` block, marker stripped, in source order.
    fn module_docs() -> String {
        THIS_FILE
            .lines()
            .take_while(|line| line.starts_with("//!"))
            .map(|line| line.trim_start_matches("//!").trim_start())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Every `## RP-` rule in an atom, as `(id, body)`.
    ///
    /// Split at the next line beginning `## `, which is how
    /// `lint_constitution::rules` splits and why a `### ` sub-heading stays
    /// inside the rule it belongs to.
    fn rules(text: &str) -> Vec<(String, String)> {
        let lines: Vec<&str> = text.lines().collect();
        let mut out: Vec<(String, String)> = Vec::new();
        for (index, line) in lines.iter().enumerate() {
            let Some(rest) = line.strip_prefix("## RP-") else {
                continue;
            };
            let id: String = rest
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '-')
                .collect();
            let end = lines
                .iter()
                .skip(index + 1)
                .position(|l| l.starts_with("## "))
                .map_or(lines.len(), |offset| index + 1 + offset);
            out.push((
                format!("RP-{}", id.trim_end_matches('-')),
                lines[index..end].join("\n"),
            ));
        }
        out
    }

    /// The paragraph a bold run-in marker opens, marker included.
    fn section<'a>(body: &'a str, marker: &str) -> Option<&'a str> {
        let start = body.lines().position(|line| line.starts_with(marker))?;
        let lines: Vec<&str> = body.lines().collect();
        let end = lines
            .iter()
            .skip(start + 1)
            .position(|line| line.trim().is_empty())
            .map_or(lines.len(), |offset| start + 1 + offset);
        let from = body.find(lines[start])?;
        let text = &body[from..];
        let taken: usize = lines[start..end].iter().map(|l| l.len() + 1).sum();
        Some(&text[..taken.min(text.len())])
    }

    /// The tokens in the first column of band 10's need table.
    fn need_table_tokens(text: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut inside = false;
        for line in text.lines() {
            if line.starts_with("| Token |") {
                inside = true;
                continue;
            }
            if !inside {
                continue;
            }
            if !line.starts_with('|') {
                break;
            }
            if line.starts_with("| ---") {
                continue;
            }
            let cell = line
                .trim_start_matches('|')
                .split('|')
                .next()
                .unwrap_or_default()
                .trim();
            if let Some(token) = cell.strip_prefix('`').and_then(|c| c.strip_suffix('`')) {
                out.push(token.to_owned());
            }
        }
        out
    }

    #[test]
    fn needs_holds_the_four_tokens_in_design_order() {
        let tokens: Vec<&str> = NEEDS.iter().map(|need| need.token).collect();
        assert_eq!(
            tokens,
            ["orientation", "tutorial", "how-to", "explanation"],
            "the set and its order are the signed-off design's; a token that \
             moved here moved in one of the three lists that must agree"
        );
        assert!(
            NEEDS.iter().all(|need| !need.job.is_empty()),
            "every member carries the job the router's generated row prints"
        );
    }

    #[test]
    fn reference_is_not_a_member() {
        assert!(
            need("reference").is_none(),
            "rustdoc and spec/SPECIFICATION.md are already this workspace's two \
             reference surfaces; a third bucket is either empty or a second \
             specification"
        );
    }

    #[test]
    fn band_ten_states_why_reference_was_subtracted() {
        let text = atom(BAND_10);
        assert!(
            text.contains("spec/SPECIFICATION.md"),
            "band 10 names the surfaces that already own reference"
        );
        assert!(
            text.contains("second specification"),
            "band 10 states the consequence, not merely the absence"
        );
    }

    #[test]
    fn the_accessor_accepts_each_of_the_four_tokens() {
        assert_eq!(NEEDS.len(), 4, "four members today");
        for member in NEEDS {
            assert_eq!(
                need(member.token).map(|found| found.token),
                Some(member.token),
                "`{}` is a member and the accessor must say so",
                member.token
            );
        }
    }

    #[test]
    fn the_accessor_is_exact_and_case_sensitive() {
        for wrong in [
            "reference",
            "guide",
            "Explanation",
            "explanation ",
            "how_to",
        ] {
            assert!(
                need(wrong).is_none(),
                "`{wrong}` is not a member: the accessor does not trim, fold \
                 case, or alias"
            );
        }
    }

    #[test]
    fn rule_dir_holds_this_storys_two_atoms() {
        // Deliberately these two and not `TREE`: this assertion belongs to
        // `need-vocabulary-and-declaration-form`, and widening it would quietly
        // re-point that story's ledger evidence at a later story's files.
        for file in [BAND_00, BAND_10] {
            assert!(
                !atom(file).trim().is_empty(),
                "{RULE_DIR}/{file} resolves and is not empty"
            );
        }
    }

    /// The inversion `need-vocabulary-and-declaration-form` scheduled.
    ///
    /// That story took `ROUTER` as a forward pin and asserted the file's
    /// *absence*, naming `router-precedence-and-announcement` as the story that
    /// would invert the assertion rather than delete it quietly. This is that
    /// inversion, in the change that creates the file.
    #[test]
    fn router_is_created_by_the_router_story() {
        assert!(
            root().join(ROUTER).exists(),
            "{ROUTER} is the tree's composition root; without it every atom is \
             reachable only by `ls`"
        );
    }

    #[test]
    fn router_opens_with_the_scope_paragraph_and_the_band_table() {
        let text = router();
        assert!(
            text.lines().next() == Some("# Page standards"),
            "the router opens `# Page standards`; it opens {:?}",
            text.lines().next()
        );
        assert!(
            text.contains("load one rule, never the tree"),
            "the scope paragraph carries the load instruction"
        );
        let bands: Vec<String> = table_rows(&text, "| Band |");
        let names: Vec<&str> = bands
            .iter()
            .map(|row| {
                row.trim_start_matches('|')
                    .split('|')
                    .next()
                    .unwrap_or_default()
                    .trim()
            })
            .collect();
        assert_eq!(
            names,
            ["`00`", "`10`", "`20`", "`30`", "`40`"],
            "the band table is the tree's numeric namespace, five rows, in order"
        );
    }

    #[test]
    fn router_states_its_rank_without_editing_the_chain() {
        let text = router();
        assert!(
            text.contains("constitution-atom tier"),
            "the reader learns the rank on the page they landed on (UX-005)"
        );
        assert!(
            text.contains("SPECIFICATION clause"),
            "the rank is stated relative to the five-tier chain, not in the abstract"
        );
        assert!(
            !text.contains("sixth tier"),
            "the discipline sits inside the chain and adds no tier to it"
        );

        // The only way anything inside this diff can speak to a *different*
        // file being untouched is to read that file and find its own words
        // still there. `git diff main -- standards/rust/README.md` is the
        // ledger's form of the same check.
        //
        // Line endings are normalised first: this is a Windows checkout with
        // `core.autocrlf=true`, so a tracked file arrives CRLF and a literal
        // written with `\n` would fail on the checkout rather than on the edit.
        let chain = std::fs::read_to_string(root().join("standards/rust/README.md"))
            .unwrap()
            .replace("\r\n", "\n");
        assert!(
            chain.contains(
                "> **SPECIFICATION clause > ADR > constitution atom > `CLAUDE.md` /\n\
                 > `CONTRIBUTING.md` summary > `references/evaluation/*`.**"
            ),
            "standards/rust/README.md's precedence block is cited, never edited"
        );
    }

    #[test]
    fn router_indexes_every_atom_in_the_tree() {
        let text = router();
        let region = generated_region(&text);
        assert!(
            !text.contains("show all") && !text.contains("<details"),
            "a filter may not hide what it filters: the index is on the same \
             page, complete, whether or not `## Start here` matched"
        );
        let rows: Vec<&String> = region.iter().skip(2).collect();
        assert_eq!(
            rows.len(),
            TREE.len(),
            "the index carries one row per atom; it has {} rows and the tree \
             has {} atoms",
            rows.len(),
            TREE.len()
        );
        for (file, row) in TREE.iter().zip(rows) {
            assert!(
                row.contains(&format!("]({file})")),
                "the index row for {file} links to it; the row reads {row}"
            );
            assert!(
                root().join(RULE_DIR).join(file).exists(),
                "{RULE_DIR}/{file} resolves; a router may not ship a dangling link"
            );
        }
    }

    #[test]
    fn router_index_rows_are_byte_identical_to_the_generator() {
        let text = router();
        let region = generated_region(&text);

        // The header and separator are the precedent's verbatim, so the checker
        // story's `generated_region` analogue is a copy rather than a variant.
        let chain = std::fs::read_to_string(root().join("standards/rust/README.md")).unwrap();
        let precedent = generated_region(&chain);
        assert_eq!(
            region.first().map(String::as_str),
            precedent.first().map(String::as_str),
            "the region's header row is the constitution's verbatim"
        );
        assert_eq!(
            region.get(1).map(String::as_str),
            precedent.get(1).map(String::as_str),
            "the region's separator row is the constitution's verbatim"
        );

        for (index, file) in TREE.iter().enumerate() {
            let want = expected_index_row(file);
            let found = region.get(index + 2).cloned().unwrap_or_default();
            assert_eq!(
                found, want,
                "row {index} disagrees with what the generator would emit for \
                 {file}; the checker's first `--write` must produce no diff"
            );
        }
        assert!(
            !region.iter().any(|line| line.trim().is_empty()),
            "no blank line inside the markers — the region is compared whole"
        );
    }

    #[test]
    fn router_states_the_one_source_line_rule_for_load_when() {
        let text = router();
        assert!(
            text.contains("## The shape of a rule"),
            "the router carries the authoring grammar for the reader writing a rule"
        );
        assert!(
            text.contains("one source line"),
            "the generator reads only the first line of a `Load when:` block, so \
             the constraint is stated where the atom author will meet it"
        );
        for file in TREE {
            let atom_text = atom(file);
            let lines: Vec<&str> = atom_text.lines().collect();
            let at = lines
                .iter()
                .position(|line| line.starts_with("> **Load when:**"))
                .unwrap_or_else(|| panic!("{file} carries no `> **Load when:**` line"));
            assert!(
                !lines[at + 1].starts_with('>'),
                "{file}'s trigger phrase would be truncated mid-phrase in the \
                 generated index, which is the defect the precedent tolerates"
            );
        }
    }

    #[test]
    fn router_states_what_checks_this_tree_and_what_does_not() {
        let text = router();
        assert!(
            text.contains("## What checks this tree, and what does not"),
            "a check whose limits are undocumented is read as a guarantee \
             (RS-81-1); so is a tree whose reader assumes the gate is watching it"
        );
        assert!(
            text.contains("page-need-checker-mounted-in-the-gate"),
            "the section names the story that adds the first gate step"
        );
        assert!(
            text.contains("no gate step reads"),
            "at this merge nothing reads the tree, and the router says so"
        );
    }

    #[test]
    fn the_repository_index_reaches_the_rules_tree_in_one_hop() {
        // Flattened to one whitespace-separated line before matching: the
        // sentences below are prose that wraps, and on this Windows checkout
        // they wrap with CRLF. A phrase that spans a line break is still the
        // same phrase to a reader, so the assertion is about the words rather
        // than about where the author happened to break them.
        let raw = std::fs::read_to_string(root().join("docs/README.md")).unwrap();
        let index = raw.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(
            index.contains("(../standards/pages/README.md)"),
            "docs/README.md's `Looking for / It is at` table links the router, \
             so the tree is reachable by someone who does not know it exists"
        );
        assert!(
            index.contains("[PROVISIONAL — settles at `page-need-checker-mounted-in-the-gate`]"),
            "the gate-read paragraph names the third pinned tree with a marker \
             that names its own removal, rather than promising a check that does \
             not exist"
        );
        assert!(
            index.contains("which is the point of pinning them by path rather than by convention"),
            "the referent is rewritten and the reasoning is not \
             (.kb/governance/rewrite-the-referent-never-the-reasoning.md)"
        );
    }

    #[test]
    fn router_regions_are_in_the_binding_order() {
        let text = router();
        let headings: Vec<&str> = text
            .lines()
            .filter(|line| line.starts_with("# ") || line.starts_with("## "))
            .collect();
        assert_eq!(
            headings,
            [
                "# Page standards",
                "## Precedence",
                "## Start here",
                "## Index",
                "## The shape of a rule",
                "## What checks this tree, and what does not",
            ],
            "the filter sits above the thing it filters, and the order is the \
             only positional language a text medium has"
        );
        for marker in [
            "<details",
            "<summary",
            "role=\"tab\"",
            "{{#tab",
            "<small>",
            "<sub>",
            "<sup>",
            "<nav>",
            "<img",
        ] {
            assert!(
                !text.contains(marker),
                "{ROUTER} carries `{marker}`: no region is occluded, and no \
                 meaning is carried by size, an icon or a widget"
            );
        }
    }

    #[test]
    fn router_is_inside_its_budgets() {
        let text = router();
        let bytes = text.len();
        assert!(
            bytes <= 8_192,
            "{ROUTER} is {bytes} bytes and the ceiling is 8192 — half an atom, \
             because this is the one file every page author loads"
        );
        let over: Vec<String> = text
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.starts_with('|'))
            .filter(|(_, line)| line.chars().count() > 96)
            .map(|(index, line)| {
                format!("{ROUTER}:{}: {} columns", index + 1, line.chars().count())
            })
            .collect();
        assert!(over.is_empty(), "prose wraps at 96 columns: {over:?}");

        let filter = table_rows(&text, "| You are");
        assert!(
            filter.len() <= 12,
            "`## Start here` carries {} rows and the ceiling is 12; past a dozen \
             a filter is a second index and the reader reads both",
            filter.len()
        );
        assert!(
            !filter.is_empty(),
            "`## Start here` is the filter; an empty one routes nobody"
        );

        for (index, line) in text.lines().enumerate() {
            let Some(info) = line.strip_prefix("```") else {
                continue;
            };
            let info = info.trim();
            assert!(
                info == "text" || info == "markdown",
                "{ROUTER}:{} — a fence tagged `{info}`; nothing in the workspace \
                 compiles this tree, so `text` or `markdown` is the honest tag",
                index + 1
            );
        }
    }

    #[test]
    fn module_docs_open_with_what_this_does_not_verify() {
        let docs = module_docs();
        let headings: Vec<&str> = docs.lines().filter(|line| line.starts_with("# ")).collect();
        assert_eq!(
            headings.first().copied(),
            Some("# What this does not verify"),
            "a check whose limits are undocumented is read as a guarantee \
             (RS-81-1); headings were {headings:?}"
        );

        let limits = [
            "It checks that a need is *declared*, never that the page *answers* it.",
            "It does not judge whether the set is the right set.",
            "It does not enforce the fold line beyond textual markers.",
            "It does not resolve clause ids.",
            "An empty rules tree passes every check below the vacuity guard.",
            "Length is not quality.",
        ];
        let mut previous = 0;
        for (index, limit) in limits.iter().enumerate() {
            let at = docs
                .find(limit)
                .unwrap_or_else(|| panic!("limit {} is missing: {limit}", index + 1));
            assert!(
                at >= previous,
                "limit {} is out of order; item 1 is first and the rest follow",
                index + 1
            );
            previous = at;
        }
        assert!(
            docs.contains("DR-07"),
            "limit 1 names the reviewer procedure as the instrument for the rest"
        );
    }

    #[test]
    fn module_docs_carry_the_hosting_assumption() {
        // Code spans are stripped before matching. RS-70-3 forbids allowing
        // `clippy::doc_markdown`, so `CommonMark` wears backticks in the prose
        // while the design's phrase reads across them.
        let docs = module_docs().replace('`', "");
        for phrase in ["CommonMark blockquotes", "document order", "re-opens DR-05"] {
            assert!(
                docs.contains(phrase),
                "the hosting assumption travels with the decision, in the \
                 design's own words: {phrase}"
            );
        }
    }

    #[test]
    fn band_zero_fixes_the_declaration_grammar() {
        let text = atom(BAND_00);
        assert!(
            text.contains("> **Answers:** `token` — <question>?"),
            "band 00 shows the declaration's literal template line"
        );
        assert!(
            text.contains("immediately after the page's `# Title`"),
            "the declaration's position is its meaning"
        );
        assert!(
            text.contains("interposed"),
            "nothing may sit between the H1 and the declaration"
        );
        assert!(
            text.contains("badge row"),
            "the interposition prohibition names what authors actually reach for"
        );
        assert!(
            text.contains("96 characters"),
            "the declaration's hard budget is stated on the page that fixes it"
        );
    }

    #[test]
    fn band_zero_states_the_overflow_diagnosis() {
        let text = atom(BAND_00);
        assert!(
            text.contains("the overflow is the diagnosis"),
            "when the question will not fit, the page is answering more than \
             one need — the yield order is stated, not left to taste"
        );
    }

    #[test]
    fn band_zero_forbids_occlusion_and_defers_the_mechanism_list() {
        let text = atom(BAND_00);
        assert!(
            text.contains("never sit behind a fold"),
            "the declaration is persistent chrome in every state"
        );
        assert!(
            text.contains("> **See also:**") && text.contains(" 20 "),
            "band 00 points at band 20 by number for the mechanism list"
        );
        assert!(
            !text.contains("PERMITTED_FOLD_MECHANISMS"),
            "band 00 must not settle DT-8 Part 3 in passing; the mechanism list \
             is band 20's"
        );
    }

    #[test]
    fn the_rules_tree_contains_no_disclosure_markup() {
        for &file in TREE {
            let text = atom(file);
            for marker in ["<details", "<summary", "role=\"tab\"", "{{#tab"] {
                assert!(
                    !text.contains(marker),
                    "{RULE_DIR}/{file} carries `{marker}`: a tree that folds \
                     cannot write the rule against folding"
                );
            }
        }
    }

    #[test]
    fn band_ten_names_every_needs_token_and_no_other() {
        let table = need_table_tokens(&atom(BAND_10));
        let expected: Vec<&str> = NEEDS.iter().map(|need| need.token).collect();
        for (row, (found, want)) in table.iter().zip(expected.iter()).enumerate() {
            assert_eq!(
                found, want,
                "row {row} of band 10's token table says `{found}` where NEEDS \
                 says `{want}` — the const and the atom move in one commit, \
                 never in two"
            );
        }
        assert_eq!(
            table.len(),
            expected.len(),
            "band 10's table has {} rows and NEEDS has {} members; the table \
             reads {table:?} and the const reads {expected:?}",
            table.len(),
            expected.len()
        );
    }

    #[test]
    fn band_ten_names_the_three_rejected_options() {
        let text = atom(BAND_10);
        for loser in ["Diátaxis", "open set", "persona"] {
            assert!(
                text.contains(loser),
                "band 10 names the option that lost and why: {loser}"
            );
        }
    }

    #[test]
    fn band_ten_states_the_two_part_amendment_rule() {
        let text = atom(BAND_10);
        assert!(
            text.contains("two-part commit"),
            "changing the set moves the const and this atom together"
        );
        assert!(
            text.contains("one-part"),
            "the atom names the wrong shape as well as the right one"
        );
    }

    #[test]
    fn band_ten_carries_both_orientation_ceilings() {
        let text = atom(BAND_10);
        let ids: Vec<String> = rules(&text).into_iter().map(|(id, _)| id).collect();
        for id in ["RP-10-2", "RP-10-3"] {
            assert!(
                ids.iter().any(|found| found == id),
                "band 10 carries {id}; it holds {ids:?}"
            );
        }
        let body = rules(&text)
            .into_iter()
            .find(|(id, _)| id == "RP-10-2")
            .map(|(_, body)| body)
            .unwrap();
        assert!(
            body.contains("teaches nothing"),
            "RP-10-2 states the ceiling that stops orientation becoming a sink"
        );
        assert!(
            body.contains("split"),
            "RP-10-2 states the remedy, which is splitting the page"
        );
        let body = rules(&text)
            .into_iter()
            .find(|(id, _)| id == "RP-10-3")
            .map(|(_, body)| body)
            .unwrap();
        assert!(
            body.contains("directory level"),
            "RP-10-3 caps orientation pages at one per directory level"
        );
    }

    /// One whitespace-separated line, for matching prose that wraps.
    ///
    /// A sentence is the same sentence to a reader whether or not the author
    /// broke it at column 80, and on this Windows checkout it wraps with CRLF
    /// besides. Flattening first keeps these assertions about the words rather
    /// than about where the line ended.
    fn flat(text: &str) -> String {
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// One `## RP-` rule's body, by id.
    fn rule_body(text: &str, id: &str) -> String {
        let found = rules(text).into_iter().find(|(found, _)| found == id);
        let Some((_, body)) = found else {
            panic!("no {id} rule");
        };
        body
    }

    #[test]
    fn band_twenty_hands_the_reviewer_the_deletion_test() {
        let text = atom(BAND_20);
        assert!(
            flat(&text).contains("would the page still teach the constraint correctly"),
            "band 20 carries the deletion test verbatim, as the rule's spirit"
        );
        let body = flat(&rule_body(&text, "RP-20-1"));
        assert!(
            body.contains("may not be collapsed"),
            "the test is a question with a yes/no answer and a stated \
             consequence, not an invitation to weigh"
        );
    }

    #[test]
    fn no_rule_in_the_tree_hands_back_the_judgement_it_replaces() {
        for &file in TREE {
            let text = atom(file).to_lowercase();
            for hedge in HEDGES {
                assert!(
                    !text.contains(hedge),
                    "{RULE_DIR}/{file} says `{hedge}`: two strangers applying \
                     that reach two impressions, and the rule exists because \
                     'use good judgment' is the non-answer that let an \
                     invariant drift here once already"
                );
            }
        }
    }

    #[test]
    fn band_twenty_closes_the_never_fold_list_at_five() {
        let body = rule_body(&atom(BAND_20), "RP-20-2");
        let classes = body
            .lines()
            .filter(|line| line.starts_with(|c: char| c.is_ascii_digit()) && line.contains(". "))
            .count();
        assert_eq!(
            classes, 5,
            "the never-fold list is five enumerated classes; RP-20-2 lists {classes}"
        );
        assert!(
            body.contains("no reviewer may grant an exception"),
            "the letter of the rule admits no exception — that is what makes it \
             a letter rather than a preference"
        );
        assert!(
            body.contains("closed") && body.contains("sign-off condition 3"),
            "the list is closed, and growing it is a disagreement with the \
             signed-off design rather than an authoring choice"
        );
    }

    #[test]
    fn band_twenty_pays_band_zeros_deferral() {
        let twenty = atom(BAND_20);
        let see = twenty
            .lines()
            .find(|line| line.starts_with("> **See also:**"))
            .unwrap_or_else(|| panic!("{BAND_20} carries no `> **See also:**` line"));
        assert!(
            see.contains(" 00 ") || see.contains(" 00("),
            "band 20's `See also` names band 00 by number, so the pairing is \
             findable from either side; it reads {see}"
        );
        assert!(
            atom(BAND_00).contains(" 20 "),
            "band 00 already points forward at band 20; this story does not \
             edit band 00 to say so"
        );
        assert!(
            rule_body(&twenty, "RP-20-2").contains("**Answers:**"),
            "class 1 of the never-fold list *is* the declaration — the deferral \
             band 00 wrote is paid here, not restated there"
        );
    }

    #[test]
    fn band_twenty_ships_an_empty_permitted_mechanism_table() {
        let text = atom(BAND_20);
        let rows = table_rows(&text, "| Mechanism |");
        assert!(
            rows.is_empty(),
            "the permitted-mechanism table ships with zero data rows, which is \
             what makes folding forbidden in practice; it holds {rows:?}"
        );
        assert!(
            text.contains("| Mechanism |"),
            "the table exists, with its header and separator: an absent table \
             reads as an oversight, an empty one reads as a decision"
        );
        assert!(
            text.contains("forbidden in practice"),
            "the atom says what an empty list means, in those words"
        );
        assert!(
            text.contains("DT-7"),
            "and it names what would lift it — HS-P0020's demonstration"
        );
    }

    #[test]
    fn band_twenty_holds_a_mechanism_to_four_recorded_observations() {
        let body = rule_body(&atom(BAND_20), "RP-20-3");
        for observation in ["accessibility tree", "keyboard", "Ctrl-F", "print"] {
            assert!(
                body.contains(observation),
                "the entry procedure names all four observations; `{observation}` \
                 is missing"
            );
        }
        assert!(
            body.contains("unverified"),
            "an unverified property counts as unmet (UX-011)"
        );
        assert!(
            body.contains("upstream"),
            "upstream documentation is not an observation — the sentence that \
             stops a mechanism being admitted on someone else's assurance"
        );
    }

    #[test]
    fn band_twenty_answers_the_renderer_supplied_wrapper_in_both_halves() {
        let text = flat(&atom(BAND_20));
        assert!(
            text.contains("toggle top-doc") || text.contains("toggle-all-docs"),
            "the wrapper the mock actually found is named, not gestured at"
        );
        assert!(
            text.contains("the author's own markup"),
            "half one: RP-20-2 binds what the author wrote, so a renderer's own \
             open-by-default wrapper puts no page in breach"
        );
        assert!(
            text.contains("unmet property"),
            "half two: nobody here has observed what survives closing it, so it \
             is recorded as an unmet property owed by the hosting decision. \
             Saying only half one turns 'we did not check' into 'it is fine'"
        );
    }

    #[test]
    fn band_twenty_states_what_this_rule_does_not_do() {
        let text = flat(&atom(BAND_20));
        assert!(
            text.contains("What this rule does not do"),
            "the atom states its own limits, unfolded, because that statement is \
             never-fold class 5 applied to the atom that wrote the class"
        );
        for limit in [
            "No gate step reads this rule",
            "DT-7",
            "band 40",
            "not a `const`",
        ] {
            assert!(
                text.contains(limit),
                "the closing statement names all four limits; `{limit}` is missing"
            );
        }

        // Assembled at run time from two halves, and anchored on `const `.
        // `THIS_FILE` is this module's own source: a whole literal here would
        // match itself — the one shape of self-reading test that is always
        // red — and a bare name would match band 00's guard, which asserts the
        // identifier is *absent* from band 00 and so must spell it.
        let forbidden = format!("const {}{}", "PERMITTED_", "FOLD_MECHANISMS");
        assert!(
            !THIS_FILE.contains(&forbidden),
            "the permitted-mechanism list is a table in the rules tree and not a \
             `const` here: nothing enforces it, and an unenforced const beside an \
             enforced one reads as a check that exists"
        );
    }

    #[test]
    fn every_atom_is_reachable_from_both_router_regions() {
        let router = router();
        let filter: Vec<String> = table_rows(&router, "| You are");
        let indexed: Vec<String> = generated_region(&router);
        for &file in TREE {
            let band = &file[..2];
            assert!(
                filter.iter().any(|row| row.contains(&format!("`{band}`"))),
                "no `## Start here` row routes to band {band}; an atom reachable \
                 only by reading the index is an atom the filter failed"
            );
            assert!(
                indexed
                    .iter()
                    .any(|row| row.contains(&format!("]({file})"))),
                "{file} has no row in the generated index; an atom absent from \
                 the index is occluded by omission"
            );
        }
    }

    #[test]
    fn both_atoms_carry_the_atom_head_grammar() {
        for &file in TREE {
            // The band is the filename's own first two characters, so an atom
            // whose head disagrees with its address fails here rather than
            // being tolerated by a hand-written pair.
            let band = &file[..2];
            let text = atom(file);
            let lines: Vec<&str> = text.lines().collect();
            assert!(
                lines[0].starts_with(&format!("# {band} — ")),
                "{file} opens `# NN — Title`; it opens {:?}",
                lines[0]
            );

            let load = lines
                .iter()
                .position(|line| line.starts_with("> **Load when:**"))
                .unwrap_or_else(|| panic!("{file} carries no `> **Load when:**` line"));
            assert!(
                !lines[load + 1].starts_with('>'),
                "{file}'s `Load when:` is one source line — the generator reads \
                 only the first and silently drops continuations"
            );

            let see = lines
                .iter()
                .position(|line| line.starts_with("> **See also:**"))
                .unwrap_or_else(|| panic!("{file} carries no `> **See also:**` line"));
            assert!(
                !lines[see].contains("]("),
                "{file} names sibling bands by number, not by link, so nothing \
                 dangles before those bands land"
            );
            assert!(
                lines.iter().any(|line| line.trim() == "---"),
                "{file} closes its head with a rule"
            );
        }
    }

    #[test]
    fn both_atoms_carry_the_five_sections_in_order() {
        for &file in TREE {
            let text = atom(file);
            let found = rules(&text);
            assert!(
                !found.is_empty(),
                "{file} carries at least one `## RP-` rule"
            );
            for (id, body) in found {
                let mut previous = 0;
                for marker in SECTIONS {
                    let at = body
                        .lines()
                        .position(|line| line.starts_with(marker))
                        .unwrap_or_else(|| panic!("{file} {id} has no {marker} section"));
                    assert!(
                        at > previous || previous == 0,
                        "{file} {id} orders its sections Why./Do/Not/Rejects./Evidence."
                    );
                    previous = at;
                }
            }
        }
    }

    #[test]
    fn both_atoms_are_inside_the_rule_and_byte_ceilings() {
        for &file in TREE {
            let text = atom(file);
            let count = rules(&text).len();
            assert!(
                count <= 6,
                "{file} carries {count} rules and the ceiling is 6"
            );
            let bytes = text.len();
            assert!(
                bytes <= 16_384,
                "{file} is {bytes} bytes and the ceiling is 16384 — an agent \
                 loading this pays for all of it"
            );
        }
    }

    #[test]
    fn prose_lines_stay_within_ninety_six_columns() {
        for &file in TREE {
            let text = atom(file);
            let over: Vec<String> = text
                .lines()
                .enumerate()
                .filter(|(_, line)| !line.starts_with('|'))
                .filter(|(_, line)| line.chars().count() > 96)
                .map(|(index, line)| {
                    format!("{file}:{}: {} columns", index + 1, line.chars().count())
                })
                .collect();
            assert!(over.is_empty(), "prose wraps at 96 columns: {over:?}");
        }
    }

    #[test]
    fn every_rejects_section_names_a_wrong_page() {
        for &file in TREE {
            let text = atom(file);
            for (id, body) in rules(&text) {
                let rejects = section(&body, "**Rejects.**")
                    .unwrap_or_else(|| panic!("{file} {id} has no **Rejects.** section"));
                let chars = rejects.chars().count();
                assert!(
                    chars >= 120,
                    "{file} {id}'s **Rejects.** is {chars} characters; naming \
                     who is misled and when they find out does not fit in less"
                );
            }
        }
    }

    #[test]
    fn no_rust_tagged_and_no_untagged_fence_in_the_rules_tree() {
        for &file in TREE {
            let text = atom(file);
            for (index, line) in text.lines().enumerate() {
                let Some(info) = line.strip_prefix("```") else {
                    continue;
                };
                let info = info.trim();
                if info.is_empty() {
                    continue;
                }
                assert!(
                    info == "text" || info == "markdown",
                    "{file}:{} — a fence here is tagged `{info}`; nothing in the \
                     workspace compiles this tree, so `text` or `markdown` is \
                     the honest tag",
                    index + 1
                );
            }
            let openers = text
                .lines()
                .filter(|line| line.starts_with("```"))
                .filter(|line| line.trim() == "```")
                .count();
            assert_eq!(
                openers % 2,
                0,
                "{file} has an untagged fence opener; a future decision to \
                 register this tree must not be undermined retroactively"
            );
        }
    }

    #[test]
    fn evidence_sections_cite_the_repository_first() {
        for &file in TREE {
            let text = atom(file);
            for (id, body) in rules(&text) {
                let evidence = section(&body, "**Evidence.**")
                    .unwrap_or_else(|| panic!("{file} {id} has no **Evidence.** section"));
                let first = evidence
                    .split('`')
                    .nth(1)
                    .unwrap_or_else(|| panic!("{file} {id}'s **Evidence.** cites nothing"));
                assert!(
                    first
                        .split(':')
                        .nth(1)
                        .is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_digit())),
                    "{file} {id} cites `{first}` first; repository `path:line` \
                     comes before the dossier and before any URL"
                );
            }
        }
    }
}
