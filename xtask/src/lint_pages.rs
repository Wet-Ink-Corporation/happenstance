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

/// The rules tree's router.
///
/// A forward pin: the file is created by `router-precedence-and-announcement`,
/// which inverts `tests::router_is_not_created_by_this_story` in the same change.
/// The constant is taken here so that both path pins are introduced together
/// rather than in two commits.
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
        for file in [BAND_00, BAND_10] {
            assert!(
                !atom(file).trim().is_empty(),
                "{RULE_DIR}/{file} resolves and is not empty"
            );
        }
    }

    #[test]
    fn router_is_not_created_by_this_story() {
        assert!(
            !root().join(ROUTER).exists(),
            "{ROUTER} is a forward pin: router-precedence-and-announcement \
             creates it and inverts this assertion in the same change"
        );
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
        for file in [BAND_00, BAND_10] {
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

    #[test]
    fn both_atoms_carry_the_atom_head_grammar() {
        for (file, band) in [(BAND_00, "00"), (BAND_10, "10")] {
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
        for file in [BAND_00, BAND_10] {
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
        for file in [BAND_00, BAND_10] {
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
        for file in [BAND_00, BAND_10] {
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
        for file in [BAND_00, BAND_10] {
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
        for file in [BAND_00, BAND_10] {
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
        for file in [BAND_00, BAND_10] {
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
