//! The pointer policy this repository's documentation is written under, and the
//! register of every pointer installed against it.
//!
//! Read this file and you can choose a pointer's shape without opening a planning
//! artefact. That is the whole reason it is here rather than in one: the
//! alternative was a markdown table in `_design.md`, which rots invisibly because
//! nothing compiles it. The register is data of the same shape as `SUMMARIES` at
//! `xtask/src/lint_constitution.rs:64` — a named list of surfaces a checker reads,
//! not a table a reviewer remembers.
//!
//! Nothing here is rendered to a reader. The register is build-time data, and a
//! page listing every pointer would be exactly the second navigation surface the
//! policy below exists to avoid: a navigation surface whose only reader is its
//! maintainer.
//!
//! # DT-10 — resolved: both, with one authoritative
//!
//! The question was whether the reference surface points outward once, repeatedly,
//! or contextually. DT-10 is resolved to **both, with one authoritative** — a front
//! door *and* pointers at individual items — and that resolution is worth nothing
//! on its own, because it reads as a permission slip. The binding half is the rule:
//! **one front door, and a pointer only at a stall.**
//!
//! ## 1. Exactly one authoritative text
//!
//! One sentence, authored once, mirrored **byte-identically** onto both front-door
//! surfaces — `crates/happenstance/src/lib.rs`'s crate root and
//! `crates/happenstance/README.md`. Two renderers, two readers, **one authority —
//! not two pointers.** A second sentence saying the same thing differently is the
//! defect, not the redundancy.
//!
//! ## 2. A secondary pointer is permitted at an item only where all four gates hold
//!
//! The register row records which of them admitted it, in its `gates` field.
//!
//! * **(i) evidenced stall.** A named, *recorded* reader failure at that item — a
//!   compiler diagnostic, a measured second question — and not a suspicion.
//! * **(ii) the front door provably cannot reach it.** The item is arrived at by
//!   deep link, by search, or from a diagnostic, so a reader can be standing on it
//!   with the crate root unseen.
//! * **(iii) subordinate and one line.** It follows the in-place fix, never
//!   precedes it, and it introduces no heading of its own. The moment a pointer
//!   stops being recessive, the page starts requiring a hop to unblock a reader.
//! * **(iv) a guard from the mechanism table.** Its form is one of the first three
//!   variants of [`PointerForm`]. The fourth — a bare URL, guarded by nothing — is
//!   forbidden to this project outright.
//!
//! Do not re-litigate the four. They were signed off with the design, and a project
//! adding a pointer applies them rather than re-deciding them.
//!
//! ## 3. The href ladder, applied per pointer and recorded in its row
//!
//! An intra-doc link **>** an in-tree markdown link inside the pinned documentation
//! tree **>** a named-but-unlinked cross-reference in the form already live at
//! `crates/happenstance-core/src/store.rs:77` **>** *nothing*. Take the highest rung
//! the destination allows and write which one down. **If only the fourth rung is
//! available the pointer is not installed and the project escalates** — the ladder
//! has no bottom rung that installs an unguarded link, and a pointer nothing catches
//! is a promise to a reader that nobody is keeping.
//!
//! ## 4. Aliases are not pointers and need no row
//!
//! `#[doc(alias = "…")]` is a **search key**, not a pointer: it moves a reader who
//! is already searching and does nothing for the reader who is reading. It is an
//! attribute on an item, so **deleting the item deletes the alias** — it cannot rot
//! independently, and it therefore **needs no register row**.
//!
//! An alias is permitted only where the string a reader searches is one that rustc,
//! the specification, or a recorded reader question actually emits, and is not the
//! item's own name or a substring of it. Two strings qualify today, both from the
//! diagnostic recorded as BR-15: `E0034`, which is what a reader pastes into a
//! search box, and `TraitVariantBlanketType`, which is what rustc's candidate notes
//! print. Rejected: synonym farming, aliases for concepts rather than strings, and
//! an alias used as a substitute for the pointer at the stall. **Residual risk,
//! named:** the alias *string* can go stale if rustc renames the internal type, and
//! nothing catches that.
//!
//! # The README's prose is not compiled, and no row may claim it is
//!
//! The project charter says the README's pointer is guarded because the README is
//! compiled as a doctest. That claim is over-broad for the placement the design
//! chose, and it must not be inherited. The mount at
//! `crates/happenstance/src/lib.rs:10` compiles the README's Rust fences and **not
//! its prose**, so a *prose* pointer there is unguarded by it.
//!
//! The row for `crates/happenstance/README.md` therefore carries the honest guard —
//! the mirror assertion, which does not exist yet — rather than the flattering one.
//! The concrete ask is on record and handed to the documentation-surface project: a
//! `check_summaries`-shaped step crossing a pinned sentence-fragment against the two
//! front-door paths, failing the build with the reason in the message. Until it
//! lands it is not counted as a guard.
//!
//! # Considered and declined, on record
//!
//! **A bespoke navigation widget was considered and is declined** — a breadcrumb, a
//! master-detail rail, a per-crate index page. rustdoc already renders a per-item
//! navigation bar and any narrative surface renders a sidebar table of contents, so
//! the evaluator's gap is evidence of a missing *link*, not a missing *widget*. No
//! navigation component is added by this project, and the stories that install
//! pointers inherit that as a constraint rather than re-deciding it.
//!
//! **A `prelude` module exporting `EventStore` and not `SendEventStore` is out of
//! this project's boundary and owed an ADR — it is not rejected.** It would make the
//! `E0034` collision unreachable by the default import path, and
//! `references/evaluation/review-dx-ergonomics.md:421-422` calls it "the single
//! highest-leverage doc fix in the crate". It is a public API addition carrying a
//! semver surface, and this project changes no public item. Its absence here is a
//! scoping decision and must never be read as a decision against it.
//!
//! # What this module cannot see, named rather than faked
//!
//! [`validate`] reads the register, not the diff. Nothing here can detect that a
//! story installed a pointer on a surface and **filed no row** — that obligation
//! lives in this text and in each installing story's own acceptance criteria, and a
//! check pretending to cover it would be worse than the honest absence. Nor can
//! `validate` tell a real mechanism from a plausible sentence: a row whose guard
//! names something that does not exist *yet* is permitted and must say so in those
//! words, because a named future guard is a claim somebody can check and an empty
//! cell is not.

/// The four permitted pointer forms, in the order the mechanism table states them.
///
/// Closed on purpose. A fifth form is a source edit *here*, visible in review,
/// rather than an improvisation at a call site — and adding a variant re-opens the
/// design's mechanism table rather than being a local decision. The one form that
/// nothing guards is present so that [`validate`] can reject it *by name*: a form
/// the type cannot express is a form the validator cannot report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerForm {
    /// An intra-doc link — ``[`Item`]`` or `[text](self::path)`.
    ///
    /// Guarded by `rustdoc::broken_intra_doc_links = "deny"` (`Cargo.toml:134`)
    /// across the gate's three rustdoc builds. It must resolve in **every** feature
    /// configuration, not only the default one: a link that resolves under
    /// `--all-features` and breaks under `--no-default-features` is the class of
    /// bug the third build exists to catch.
    IntraDoc,

    /// A link inside a Rust fence in `crates/happenstance/README.md`.
    ///
    /// Guarded by the README doctest mount at `crates/happenstance/src/lib.rs:10`,
    /// and only that far: the fence *compiles*, so its code is checked and its
    /// surrounding prose is not. A prose pointer on that surface takes a different
    /// guard or none.
    ReadmeRustFence,

    /// A markdown link on a page inside the pinned documentation tree.
    ///
    /// Guarded by that tree's registration check, which fails the build when a page
    /// moves out from under a link. The mechanism does not exist until the
    /// documentation-surface project lands it, so a row taking this form says so in
    /// its guard rather than describing a check that is not there yet.
    PinnedTreeMarkdown,

    /// A bare URL in prose.
    ///
    /// Guarded by **nothing**, and therefore **forbidden to this project
    /// outright** — the general mechanism table permits it for external targets
    /// with a row recording the absence, and this project does not, because a bare
    /// URL into a tree that moves is a pointer whose only guard is memory.
    /// [`validate`] rejects it unconditionally.
    BareUrl,
}

impl PointerForm {
    /// Every permitted form, so a consumer can enumerate them without matching.
    pub const ALL: [Self; 4] = [
        Self::IntraDoc,
        Self::ReadmeRustFence,
        Self::PinnedTreeMarkdown,
        Self::BareUrl,
    ];
}

/// One row of the register: one installed pointer, and everything a guard claim
/// about it needs to state.
///
/// Every field is required because every one of them is a fact a reviewer would
/// otherwise have to reconstruct from the diff. The two booleans exist *together*
/// so that a row cannot claim reachability it does not have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pointer {
    /// The row label — `P1`, `P2`, … — unique across the register.
    pub id: &'static str,

    /// The file the pointer sits on.
    pub surface: &'static str,

    /// What it points at, in the words a reader would use.
    pub destination: &'static str,

    /// Which of the four permitted forms it takes.
    pub form: PointerForm,

    /// The mechanism that fails the build when this pointer rots.
    ///
    /// Prose, naming the mechanism. A mechanism that does not exist yet is
    /// permitted and must say so in the same breath; an empty cell is not.
    pub guard: &'static str,

    /// The visible link text, which is all a screen reader or an extracted link
    /// list gives the reader.
    pub link_text: &'static str,

    /// Whether the answer is on the destination's first screen.
    pub answer_on_first_screen: bool,

    /// Whether the pointer targets a fragment rather than the top of a page.
    pub targets_fragment: bool,

    /// Which of gates (i)–(iv) admitted this pointer.
    ///
    /// Empty for the two front-door rows, which are rule 1 rather than rule 2.
    pub gates: &'static str,
}

/// The most rows the register may carry at this project's close.
///
/// Five are foreseen; eight is the bar with headroom. Enforced by [`validate`].
pub const MAX_ROWS_AT_PROJECT_CLOSE: usize = 8;

/// The ceiling, ever — documented rather than enforced, because enforcing it as
/// well would be unreachable code behind [`MAX_ROWS_AT_PROJECT_CLOSE`].
///
/// It is on the constant rather than in a planning file because the person about
/// to raise the cap is reading *here*. **A twenty-first row means the pointer
/// policy is wrong, not that the register needs a scrollbar** — the number is the
/// DT-10 alarm, and raising it silently is the failure the cap exists to catch.
pub const MAX_ROWS_EVER: usize = 20;

/// Every pointer installed under the policy above.
///
/// It landed **empty**, and that was the design rather than an omission: each
/// story that installs a pointer files its own row in the same change that
/// installs it, so a pre-filled register would be a set of claims nobody had
/// earned. The rules in [`validate`] are therefore exercised against
/// deliberately-wrong registers rather than against this one — a rule no register
/// can fail is decorative.
///
/// **P3 is the first row, and it is the one place the four forms could not name
/// the rung.** The href ladder's third rung — a named-but-unlinked
/// cross-reference — is not a [`PointerForm`] variant, because the enum
/// enumerates the architecture brief's four *mechanism-table* rows and the ladder
/// is a different list. A fifth variant would re-open the signed-off mechanism
/// table for a rung the design's own P3 row already groups with
/// [`PointerForm::PinnedTreeMarkdown`] ("in-tree markdown link, **or** the
/// named-unlinked form live at `store.rs:77`"), and both share one guard. So the
/// row is filed under that variant and the rung it actually took is stated in its
/// `guard`, where a reviewer reads it.
pub const POINTER_REGISTER: &[Pointer] = &[Pointer {
    id: "P3",
    surface: "crates/happenstance-core/src/store.rs",
    destination: "the adapter reading order, docs/adapter-reading-order.md",
    form: PointerForm::PinnedTreeMarkdown,
    guard: "two mechanisms, and between them they cover both ends. The narrative tree's \
            bidirectional registration check — `cargo xtask narrative`, a mandatory step — \
            fails the build when the destination stops being registered in \
            `xtask/src/narrative.rs`, whether it was deleted or renamed. And \
            `row_p3s_pointer_is_installed_on_the_surface_it_claims`, in this file, fails \
            when the prose on the surface stops carrying this row's destination path or its \
            link text, or when that path stops being the one the tree registers — the gap \
            this row previously recorded as unguarded, closed by the story that filed it. \
            Rung actually taken: the named-but-unlinked cross-reference, rung 3, because no \
            link form from a rustdoc page resolves into the narrative tree in all three of \
            the gate's rustdoc builds",
    link_text: "the adapter reading order",
    answer_on_first_screen: true,
    targets_fragment: false,
    gates: "(i) BR-15, a recorded reader failure at this item; (ii) the reader arrives from a \
            diagnostic or a search box and `pub mod store` renders its own page; (iii) one \
            sentence, last in the section, no heading of its own; (iv) rung 3 of the ladder, \
            guarded by the registration check",
}];

/// The fewest words a self-describing link text can be built from.
const LINK_TEXT_MINIMUM_WORDS: usize = 3;

/// Link texts that name no destination, and are therefore rejected.
///
/// A row is rejected when its link text *is* one of these or *begins with* one,
/// compared case-insensitively and with trailing punctuation trimmed — "See this
/// page." is the same defect wearing a full stop.
const NON_DESCRIPTIVE_LINK_TEXT: &[&str] = &[
    "here",
    "this",
    "see this page",
    "docs",
    "read more",
    "http://",
    "https://",
];

/// Check a register against the policy in this module's documentation.
///
/// A pure function over a slice rather than an assertion over
/// [`POINTER_REGISTER`], so that the rules can be exercised against registers
/// nobody has filed — which is what stops them being vacuous while the real
/// register is empty. It never panics.
///
/// # Errors
///
/// Returns **one message per defect**, never the first and stop: a story filing a
/// bad row should learn everything wrong with it in one run. The order is stable —
/// register order, then rule order within a row (guard, form, link text, fragment,
/// duplicate id), then the cap — so a failing build produces a reviewable diff.
/// Every message names the offending row and carries the reason its rule exists.
pub fn validate(rows: &[Pointer]) -> Result<(), Vec<String>> {
    let mut problems = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        check_guard(row, &mut problems);
        check_form(row, &mut problems);
        check_link_text(row, &mut problems);
        check_fragment(row, &mut problems);
        check_unique_id(row, &rows[..index], &mut problems);
    }
    check_cap(rows, &mut problems);

    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems)
    }
}

/// No pointer is installed whose only guard is memory.
fn check_guard(row: &Pointer, problems: &mut Vec<String>) {
    if !row.guard.trim().is_empty() {
        return;
    }
    let (id, surface) = (row.id, row.surface);
    problems.push(format!(
        "{id} ({surface}) — the guard cell is empty. No pointer is installed whose \
         only guard is memory: name the mechanism that fails the build when this \
         pointer rots, and if that mechanism does not exist yet, say so in the same \
         sentence rather than leaving the cell blank"
    ));
}

/// The fourth form is forbidden to this project outright.
fn check_form(row: &Pointer, problems: &mut Vec<String>) {
    if row.form != PointerForm::BareUrl {
        return;
    }
    let (id, surface) = (row.id, row.surface);
    problems.push(format!(
        "{id} ({surface}) — a bare URL is guarded by nothing, and this project \
         forbids it outright. The href ladder's last rung is not a worse link: it \
         is do not install, and escalate. Take an intra-doc link, an in-tree \
         markdown link, or a named-but-unlinked cross-reference — or leave the \
         pointer uninstalled and say why"
    ));
}

/// The link text is all a screen reader gives the reader.
fn check_link_text(row: &Pointer, problems: &mut Vec<String>) {
    let lowered = row.link_text.trim().to_lowercase();
    let normalised = lowered
        .trim_end_matches(|character: char| character.is_ascii_punctuation())
        .trim();

    let too_short = normalised.split_whitespace().count() < LINK_TEXT_MINIMUM_WORDS;
    let vague = NON_DESCRIPTIVE_LINK_TEXT
        .iter()
        .any(|denied| normalised.starts_with(denied));
    if !too_short && !vague {
        return;
    }

    let (id, surface, text) = (row.id, row.surface, row.link_text);
    problems.push(format!(
        "{id} ({surface}) — the link text {text:?} does not name its destination. A \
         reader navigating by an extracted link list or a screen reader is given the \
         link text and nothing else, so it must be at least {LINK_TEXT_MINIMUM_WORDS} \
         words and a noun phrase naming what it points at. Repair the text, never the \
         rule"
    ));
}

/// Land the reader on the answer, not the top of the page.
fn check_fragment(row: &Pointer, problems: &mut Vec<String>) {
    if row.answer_on_first_screen || row.targets_fragment {
        return;
    }
    let (id, surface, destination) = (row.id, row.surface, row.destination);
    problems.push(format!(
        "{id} ({surface}) — the row says the answer is not on the first screen of \
         {destination} and that the pointer does not target a fragment. Land the \
         reader on the answer, not the top of the page: give the link the answering \
         passage's heading anchor, or correct whichever of the two claims is wrong"
    ));
}

/// Two rows sharing an id makes every other message ambiguous.
fn check_unique_id(row: &Pointer, earlier: &[Pointer], problems: &mut Vec<String>) {
    let Some(first) = earlier.iter().find(|other| other.id == row.id) else {
        return;
    };
    let (id, surface, other) = (row.id, row.surface, first.surface);
    problems.push(format!(
        "{id} ({surface}) — the row id {id} is already filed by the row for {other}. \
         Two rows sharing an id makes every message about them ambiguous, and lets \
         one story overwrite a sibling's row while believing it filed its own"
    ));
}

/// The cap is the DT-10 alarm, wired to a number.
fn check_cap(rows: &[Pointer], problems: &mut Vec<String>) {
    let Some(row) = rows.get(MAX_ROWS_AT_PROJECT_CLOSE) else {
        return;
    };
    let (id, surface, count) = (row.id, row.surface, rows.len());
    problems.push(format!(
        "{id} ({surface}) — the register carries {count} rows and the cap at this \
         project's close is {MAX_ROWS_AT_PROJECT_CLOSE}, with a ceiling of \
         {MAX_ROWS_EVER} ever. A twenty-first row means the pointer policy is wrong, \
         not that the register needs a scrollbar: the repair is the policy, and \
         raising the cap re-opens the density budget that set it"
    ));
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, reason = "test code, per the house style")]

    use super::*;

    /// This module's own source. The policy is a deliverable, so it is asserted
    /// rather than trusted — a module doc nothing checks is exactly the rot this
    /// module exists to stop.
    const SOURCE: &str = include_str!("pointers.rs");

    /// The nine row ids the cap test needs, one past the cap.
    const IDS: [&str; 9] = ["P1", "P2", "P3", "P4", "P5", "P6", "P7", "P8", "P9"];

    /// The shortest rejection message that has ever said anything.
    ///
    /// Calibrated the way `lint_constitution`'s `MIN_REJECTS_CHARS` was: a length,
    /// not a vocabulary. It is the "presentation exists at all" invariant in this
    /// medium — a `validate` returning a bare `false` satisfies every structural
    /// assertion perfectly and teaches nobody anything.
    const MIN_MESSAGE_CHARS: usize = 120;

    /// The source above the test module, so an assertion about the policy cannot
    /// be satisfied by the test's own pinned strings.
    fn production_source() -> &'static str {
        SOURCE.split("#[cfg(test)]").next().unwrap()
    }

    /// The module doc, whitespace-normalised, so a pinned subject string is not
    /// broken by where a line happens to wrap.
    fn policy_prose() -> String {
        SOURCE
            .lines()
            .take_while(|line| line.starts_with("//!"))
            .map(|line| line.trim_start_matches("//!"))
            .collect::<Vec<_>>()
            .join(" ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Pin short subject strings, never paragraphs: a test that pins a paragraph
    /// turns ordinary rewording into a build failure and gets deleted rather than
    /// fixed.
    fn assert_states(prose: &str, subjects: &[&str]) {
        let missing: Vec<&&str> = subjects.iter().filter(|s| !prose.contains(**s)).collect();
        assert!(
            missing.is_empty(),
            "the policy no longer states {missing:?}; a later story would have to \
             read `_design.md` to choose a pointer shape"
        );
    }

    /// The `pub enum PointerForm` declaration, doc comments included.
    fn enum_block() -> &'static str {
        let source = production_source();
        let start = source.find("pub enum PointerForm {").unwrap();
        let rest = &source[start..];
        let end = rest.find("\n}").unwrap();
        &rest[..end]
    }

    /// The doc comment attached to one variant of that enum, whitespace-normalised
    /// so a pinned subject string is not broken by where a line happens to wrap.
    fn variant_doc(variant: &str) -> String {
        let block = enum_block();
        let end = block.find(&format!("\n    {variant},")).unwrap();
        let start = block[..end].rfind(",\n\n").map_or(0, |index| index + 2);
        block[start..end]
            .lines()
            .map(|line| line.trim().trim_start_matches("///"))
            .collect::<Vec<_>>()
            .join(" ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// A row that passes every rule, so each wrong register below differs from a
    /// good one in exactly the field its rule is about.
    fn row(id: &'static str) -> Pointer {
        Pointer {
            id,
            surface: "crates/happenstance-core/src/store.rs",
            destination: "the adapter reasoning account",
            form: PointerForm::PinnedTreeMarkdown,
            guard: "the pinned tree's registration check — named, and it does not exist yet",
            link_text: "the adapter reasoning account",
            answer_on_first_screen: true,
            targets_fragment: false,
            gates: "(i) BR-15, (ii) reached from a diagnostic, (iii) recessive, (iv) form 3",
        }
    }

    /// The problems `validate` reports, for a register that must be rejected.
    fn problems(rows: &[Pointer]) -> Vec<String> {
        validate(rows).expect_err("this register must be rejected")
    }

    // ---- AC-001 ------------------------------------------------------------

    #[test]
    fn policy_states_the_dt10_resolution_its_four_gates_and_the_href_ladder() {
        assert_states(
            &policy_prose(),
            &[
                "DT-10",
                "both, with one authoritative",
                "one front door, and a pointer only at a stall",
                "byte-identically",
                "evidenced stall",
                "provably cannot reach",
                "subordinate and one line",
                "guard from the mechanism table",
                "the pointer is not installed and the project escalates",
            ],
        );
    }

    // ---- AC-002 ------------------------------------------------------------

    #[test]
    fn permitted_forms_are_closed_and_each_documents_its_guard() {
        // Exhaustive, with no wildcard arm: a fifth form is a source edit here,
        // and this match is what makes that edit visible rather than silent.
        for form in PointerForm::ALL {
            let name = match form {
                PointerForm::IntraDoc => "IntraDoc",
                PointerForm::ReadmeRustFence => "ReadmeRustFence",
                PointerForm::PinnedTreeMarkdown => "PinnedTreeMarkdown",
                PointerForm::BareUrl => "BareUrl",
            };
            assert!(
                enum_block().contains(&format!("\n    {name},")),
                "{name} is not declared in the enum this test reads"
            );
        }
        assert_eq!(
            PointerForm::ALL.len(),
            4,
            "the mechanism table has four rows; a fifth re-opens the design's table"
        );

        for (variant, guard) in [
            ("IntraDoc", "broken_intra_doc_links"),
            ("ReadmeRustFence", "crates/happenstance/src/lib.rs:10"),
            ("PinnedTreeMarkdown", "registration check"),
            ("BareUrl", "forbidden to this project outright"),
        ] {
            let doc = variant_doc(variant);
            assert!(
                doc.contains(guard),
                "{variant} does not document what catches it rotting ({guard})"
            );
        }
        assert!(
            variant_doc("BareUrl").contains("nothing"),
            "the one unguarded form must say so in the word the table uses"
        );
    }

    #[test]
    fn policy_records_the_readme_prose_gap_and_the_alias_rule() {
        assert_states(
            &policy_prose(),
            &[
                "compiles the README's Rust fences",
                "not its prose",
                "doc(alias",
                "search key",
                "deleting the item deletes the alias",
                "needs no register row",
            ],
        );
    }

    // ---- AC-003 ------------------------------------------------------------

    /// What `the_register_lands_empty_and_valid` became once a story filed a row.
    ///
    /// `pointer-policy-and-inventory` landed [`POINTER_REGISTER`] empty and
    /// asserted that emptiness as a forward pin. `store-error-site-rewrite`
    /// installed the first pointer, so the assertion is **inverted here rather
    /// than deleted quietly** — the shape [`crate::lint_pages`]'s `ROUTER` doc
    /// records one tree over, where a story asserted a file's absence and named
    /// the story that would invert it. What survives the inversion is the half
    /// that never expires: the validator is wired to the *live* register, and the
    /// register holds only rows a story actually installed.
    #[test]
    fn the_register_carries_only_rows_a_story_installed() {
        assert!(
            validate(POINTER_REGISTER).is_ok(),
            "the validator must be wired to the real register, not only to synthetic ones"
        );
        let filed: Vec<&str> = POINTER_REGISTER.iter().map(|row| row.id).collect();
        assert_eq!(
            filed,
            ["P3"],
            "the register grows one row per installed pointer: a row here that no \
             story installed is a claim nobody earned, and a missing row is a \
             pointer whose only guard is memory"
        );
    }

    /// Row **P3** — the `store.rs` module doc's cross-reference to the adapter
    /// reading order — is filed with a guard that names a mechanism which exists.
    ///
    /// The rung assertion is the load-bearing one. [`PointerForm`] enumerates the
    /// architecture brief's four *mechanism-table* rows and has no variant for the
    /// href ladder's third rung, so a row taking that rung is filed under
    /// [`PointerForm::PinnedTreeMarkdown`] — the row whose guard it shares — and
    /// has to say so in words. A guard cell that quietly dropped the rung would
    /// leave the register claiming a link that is not there.
    #[test]
    fn row_p3_records_the_store_module_pointer_and_the_rung_it_took() {
        let p3 = POINTER_REGISTER
            .iter()
            .find(|row| row.id == "P3")
            .expect("P3 is store.rs's pointer to the adapter reading order");

        assert_eq!(p3.surface, "crates/happenstance-core/src/store.rs");
        assert!(
            p3.destination.contains("docs/adapter-reading-order.md"),
            "the destination must name the page the pointer resolves to: {}",
            p3.destination
        );
        assert_eq!(p3.form, PointerForm::PinnedTreeMarkdown);
        assert!(
            p3.guard.contains("cargo xtask narrative") && p3.guard.contains("registration check"),
            "the guard must name the mechanism that fails the build: {}",
            p3.guard
        );
        assert!(
            p3.guard.contains("named-but-unlinked"),
            "the guard must record the rung actually taken, which is not the one \
             the form's name suggests: {}",
            p3.guard
        );
        assert!(
            p3.gates.contains("(i)") && p3.gates.contains("(iv)"),
            "rule 2 admits a pointer at an item only through its four gates: {}",
            p3.gates
        );
        assert!(
            p3.answer_on_first_screen,
            "the reading order is the destination's first screen, which is why the \
             row needs no fragment"
        );
    }

    /// Row **P3** describes a pointer that is really installed, on the surface it
    /// names, pointing where it says.
    ///
    /// [`validate`] reads the register and not the diff, and this module's own
    /// documentation says plainly that nothing here can catch a story installing a
    /// pointer and filing no row. **The inverse direction is catchable, and this is
    /// it**: a row is a claim about two files, so the claim is read back out of
    /// them. Three ways for the register to start lying are closed —
    ///
    /// * the row is filed and the pointer was never installed, or was later
    ///   deleted from the surface;
    /// * the link text drifted on the surface, so an extracted link list and the
    ///   register disagree about what the reader is offered;
    /// * the path spelled in the prose stopped being the path the narrative tree
    ///   registers, which is the gap P3's guard used to record as unguarded.
    ///
    /// The two files it reads are read with `include_str!`, so a *surface* or a
    /// *registration table* that is deleted or moved fails to compile rather than
    /// to assert. Reaching across the workspace is safe here and would not be from
    /// a published crate: `xtask` is `publish = false`, so none of the packaging
    /// hazard applies.
    ///
    /// **What this does not catch, said plainly rather than left to be assumed:**
    /// the destination page going missing. Nothing here opens
    /// `docs/adapter-reading-order.md` — it checks that the surface and the
    /// registration table agree on its *path*. The page's own existence is the
    /// other half of P3's guard and belongs to the narrative tree:
    /// `xtask/src/narrative.rs` registers it under `#[cfg(doctest)]`, so
    /// `cargo test --doc` fails to compile without it, and `cargo xtask narrative`
    /// reports that `mod adapter_reading_order` names no page in `docs`. Both were
    /// run against a working tree with the page renamed away, and both fired.
    #[test]
    fn row_p3s_pointer_is_installed_on_the_surface_it_claims() {
        const SURFACE: &str = include_str!("../../crates/happenstance-core/src/store.rs");
        const REGISTRATIONS: &str = include_str!("narrative.rs");
        const DESTINATION: &str = "docs/adapter-reading-order.md";

        let p3 = POINTER_REGISTER
            .iter()
            .find(|row| row.id == "P3")
            .expect("P3 is store.rs's pointer to the adapter reading order");

        // The module `//!` comment and nothing below it. The surface is the page a
        // reader sees, so the file's own test module — which quotes both strings
        // while asserting the same thing from the other side — must not be able to
        // satisfy this from inside.
        let page: Vec<&str> = SURFACE
            .lines()
            .take_while(|line| line.starts_with("//!"))
            .collect();

        assert!(
            p3.destination.contains(DESTINATION),
            "this test reads the surface for {DESTINATION}, so the row has to be \
             about that page: {}",
            p3.destination
        );
        let installed = page
            .iter()
            .filter(|line| line.contains(DESTINATION))
            .count();
        assert_eq!(
            installed, 1,
            "the surface {} carries the destination path {installed} times in its \
             documentation. A row filed against a surface that does not carry the \
             pointer is the register claiming a reach nobody installed; two of them \
             is the recessive single hop turning into a navigation surface",
            p3.surface
        );
        assert!(
            page.iter().any(|line| line.contains(p3.link_text)),
            "the link text {:?} is not on the surface. The register and the page have \
             drifted, and the register is what a maintainer reads instead of every \
             file",
            p3.link_text
        );
        assert!(
            REGISTRATIONS.contains(DESTINATION),
            "the path spelled on the surface is no longer the path \
             `xtask/src/narrative.rs` registers, so `cargo xtask narrative` is \
             guarding a different page from the one the reader is sent to"
        );
    }

    #[test]
    fn the_mount_declares_the_module_and_displaces_nothing() {
        let root = include_str!("lib.rs");
        assert!(
            root.contains("pub mod pointers;"),
            "the register is not declared in the lib crate root; a module nothing declares is \
             the foundation-as-fixme this story forbids"
        );
        assert!(
            root.contains("mod constitution;") && root.contains("mod narrative;"),
            "the mount displaced an existing harness"
        );
        assert!(
            root.lines()
                .nth(20)
                .unwrap()
                .starts_with("#![cfg_attr(doctest,"),
            "the repository README's doctest mount at `xtask/src/lib.rs:21` moved"
        );
        assert!(
            !root.starts_with("//! Nothing but a home for the repository README's doctests."),
            "the crate doc still opens with a sentence this module made false"
        );
    }

    // ---- AC-004 ------------------------------------------------------------

    #[test]
    fn a_row_records_every_fact_a_guard_claim_needs() {
        let filed = row("P3");
        assert_eq!(filed.id, "P3");
        assert_eq!(filed.surface, "crates/happenstance-core/src/store.rs");
        assert_eq!(filed.destination, "the adapter reasoning account");
        assert_eq!(filed.form, PointerForm::PinnedTreeMarkdown);
        assert!(filed.guard.contains("registration check"));
        assert_eq!(filed.link_text, "the adapter reasoning account");
        assert!(filed.answer_on_first_screen);
        assert!(!filed.targets_fragment);
        assert!(filed.gates.contains("(iv)"));

        let rendered = format!("{filed:?}");
        assert!(
            rendered.contains("P3") && rendered.contains("PinnedTreeMarkdown"),
            "a rejected row must print itself, not an index: {rendered}"
        );
    }

    // ---- AC-005 ------------------------------------------------------------

    #[test]
    fn rejects_a_row_whose_guard_is_empty() {
        let mut bad = row("P3");
        bad.guard = "";
        let reported = problems(&[bad]);
        assert_eq!(reported.len(), 1, "{reported:?}");
        assert!(reported[0].contains("P3"), "{}", reported[0]);
        assert!(
            reported[0].contains("only guard is memory"),
            "the message must enforce the project's own sentence: {}",
            reported[0]
        );
    }

    #[test]
    fn rejects_a_row_whose_guard_is_only_whitespace() {
        let mut bad = row("P3");
        bad.guard = "   \t ";
        let reported = problems(&[bad]);
        assert_eq!(reported.len(), 1, "{reported:?}");
        assert!(reported[0].contains("P3"), "{}", reported[0]);
    }

    // ---- AC-006 ------------------------------------------------------------

    #[test]
    fn rejects_a_bare_url_outright_and_states_the_escalation() {
        let mut bad = row("P4");
        bad.form = PointerForm::BareUrl;
        bad.guard = "none — the tree moves and the URL will not";
        let reported = problems(&[bad]);
        assert_eq!(reported.len(), 1, "{reported:?}");
        assert!(reported[0].contains("P4"), "{}", reported[0]);
        assert!(
            reported[0].contains("do not install"),
            "the message must state the ladder's escalation, not offer a workaround: {}",
            reported[0]
        );
        assert!(
            !reported[0].contains("guard: none"),
            "the message must not offer a way to record the absence of a guard: {}",
            reported[0]
        );
    }

    // ---- AC-007 ------------------------------------------------------------

    #[test]
    fn rejects_link_text_that_does_not_name_its_destination() {
        let mut checked = 0_usize;
        for phrase in NON_DESCRIPTIVE_LINK_TEXT {
            for text in [
                (*phrase).to_owned(),
                phrase.to_uppercase(),
                format!("{phrase}."),
                format!("{phrase} for the adapter reasoning account"),
            ] {
                let mut bad = row("P5");
                bad.link_text = Box::leak(text.clone().into_boxed_str());
                let reported = problems(&[bad]);
                assert_eq!(reported.len(), 1, "{text:?} → {reported:?}");
                assert!(reported[0].contains("P5"), "{text:?} → {}", reported[0]);
                checked += 1;
            }
        }
        assert_eq!(checked, 28, "every denied phrase, cased and punctuated");

        // "See this page." is the same defect wearing a full stop.
        let mut punctuated = row("P5");
        punctuated.link_text = "See this page.";
        assert_eq!(problems(&[punctuated]).len(), 1);

        // Fewer than three words, and on no denied list at all.
        let mut short = row("P5");
        short.link_text = "the guide";
        assert_eq!(problems(&[short]).len(), 1);
    }

    #[test]
    fn accepts_a_three_word_noun_phrase() {
        let mut good = row("P5");
        good.link_text = "adapter reasoning account";
        assert!(validate(&[good]).is_ok());
    }

    // ---- AC-008 ------------------------------------------------------------

    #[test]
    fn rejects_a_row_that_admits_a_deep_answer_and_targets_a_page() {
        let mut bad = row("P4");
        bad.answer_on_first_screen = false;
        bad.targets_fragment = false;
        let reported = problems(&[bad]);
        assert_eq!(reported.len(), 1, "{reported:?}");
        assert!(reported[0].contains("P4"), "{}", reported[0]);
        assert!(
            reported[0].contains("not the top of the page"),
            "the message must state the invariant it enforces: {}",
            reported[0]
        );
    }

    #[test]
    fn accepts_a_deep_answer_that_targets_a_fragment() {
        let mut good = row("P4");
        good.answer_on_first_screen = false;
        good.targets_fragment = true;
        assert!(validate(&[good]).is_ok());
    }

    // ---- AC-009 ------------------------------------------------------------

    #[test]
    fn rejects_duplicate_row_ids() {
        let first = row("P1");
        let mut second = row("P1");
        second.surface = "crates/happenstance/README.md";
        let reported = problems(&[first, second]);
        assert_eq!(reported.len(), 1, "{reported:?}");
        assert!(
            reported[0].contains("crates/happenstance/README.md")
                && reported[0].contains("crates/happenstance-core/src/store.rs"),
            "both surfaces must be named, or the reader cannot tell which row to fix: {}",
            reported[0]
        );
    }

    #[test]
    fn reports_every_problem_not_only_the_first() {
        let mut first = row("P1");
        first.guard = "";
        let mut second = row("P2");
        second.form = PointerForm::BareUrl;
        let reported = problems(&[first, second]);
        assert_eq!(reported.len(), 2, "{reported:?}");
        assert!(reported[0].contains("P1"), "{}", reported[0]);
        assert!(reported[1].contains("P2"), "{}", reported[1]);
    }

    #[test]
    fn every_message_names_its_row_and_states_its_reason() {
        let mut empty_guard = row("P1");
        empty_guard.guard = "";
        let mut bare_url = row("P2");
        bare_url.form = PointerForm::BareUrl;
        let mut vague_text = row("P3");
        vague_text.link_text = "here";
        let mut deep_answer = row("P4");
        deep_answer.answer_on_first_screen = false;
        deep_answer.targets_fragment = false;
        let duplicate = row("P4");

        let register = [empty_guard, bare_url, vague_text, deep_answer, duplicate];
        let reported = problems(&register);
        assert_eq!(reported.len(), 5, "{reported:?}");

        for (index, message) in reported.iter().enumerate() {
            let expected = ["P1", "P2", "P3", "P4", "P4"][index];
            assert!(
                message.starts_with(expected),
                "message {index} does not open with its row id: {message}"
            );
            assert!(
                message.chars().count() >= MIN_MESSAGE_CHARS,
                "message {index} carries no reason, only a verdict: {message}"
            );
        }
    }

    #[test]
    fn rejects_a_ninth_row_as_the_dt10_alarm() {
        let register: Vec<Pointer> = IDS.iter().map(|id| row(id)).collect();
        assert_eq!(register.len(), MAX_ROWS_AT_PROJECT_CLOSE + 1);
        let reported = problems(&register);
        assert_eq!(reported.len(), 1, "{reported:?}");
        assert!(reported[0].contains("P9"), "{}", reported[0]);
        assert!(
            reported[0].contains("the pointer policy is wrong"),
            "the cap is an alarm with a stated meaning, not an arithmetic complaint: {}",
            reported[0]
        );

        // Eight is the cap, not the alarm.
        assert!(validate(&register[..MAX_ROWS_AT_PROJECT_CLOSE]).is_ok());

        // The ceiling that is documented rather than enforced, and the sentence
        // the person raising the cap has to read.
        assert_eq!(MAX_ROWS_EVER, 20);
        let source = production_source();
        let declaration = source.find("pub const MAX_ROWS_EVER").unwrap();
        let doc = &source[declaration.saturating_sub(900)..declaration];
        assert!(
            doc.contains("scrollbar") && doc.contains("policy is wrong"),
            "the ceiling's own doc must carry the alarm's meaning"
        );
    }

    // ---- AC-010 ------------------------------------------------------------

    #[test]
    fn policy_records_the_declined_widget_and_the_out_of_boundary_prelude() {
        assert_states(
            &policy_prose(),
            &[
                "breadcrumb",
                "master-detail rail",
                "per-crate index page",
                "missing *link*, not a missing *widget*",
                "prelude",
                "out of this project's boundary",
                "owed an ADR",
                "not rejected",
            ],
        );
    }
}
