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
//!
//! # Three divergences from `lint_constitution`, stated rather than inherited
//!
//! This module copies `xtask/src/lint_constitution.rs`'s shape deliberately and
//! shares no code with it. Three of its rules are *different* here, and each
//! difference is a decision rather than an oversight.
//!
//! 1. **The fence rule inverts, and there is no `check_harness` equivalent.**
//!    `check_fences` rejects an *untagged* fence because `standards/rust/` **is**
//!    registered in the doctest harness, bidirectionally checked by
//!    `lint_constitution::check_harness` (`xtask/src/lint_constitution.rs:423-458`).
//!    This tree is deliberately **not** registered with any harness, so a
//!    `rust`-tagged fence here is a Rust claim nothing in the workspace compiles.
//!    Both `rust`-tagged and untagged fences are rejected; `text` and `markdown`
//!    are permitted. **Nothing here corresponds to `check_harness`**, and the
//!    absence is stated so the next reader does not see a checker that looks
//!    like `lint_constitution` with a check missing.
//! 2. **The generated region carries more weight here.** For `standards/rust/`
//!    the router's index is a convenience over a corpus the compiler also reads.
//!    Here it is the *only* mechanism preventing the router's index from
//!    disagreeing with the corpus it indexes, so a `--write` diff on this tree
//!    deserves more of a reviewer's attention than the same diff next door.
//! 3. **No shared abstraction with `lint_constitution`.** Neither
//!    `xtask/src/lint_constitution.rs` nor `xtask/src/constitution.rs` is
//!    refactored to share this shape (RS-81-3,
//!    `standards/rust/81-checks-that-cannot-be-types.md:209`). Copying the shape
//!    is cheap; one error message answering two trees' questions is not.
//!
//! One agreement check here is deliberately **not** `--write`-repairable:
//! `NEEDS` against `standards/pages/10-the-need-set.md`. That atom carries a
//! third column — *success for the reader* — which is prose no `const` holds, so
//! a generator would have to invent it.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::lint_narrative;
use crate::spec_trace::workspace_root;

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

/// The atom the need set is argued in, and the one [`NEEDS`] is checked against.
const NEED_SET_ATOM: &str = "10-the-need-set.md";

/// The gate step's name, which is also the claim it makes.
///
/// Named once, here, for the reason [`crate::lint_narrative::STEP`] is: `REQUIRED`
/// and `lint_steps()` both depend on it by value and `steps_named` panics on a
/// mismatch (`xtask/src/main.rs:816-826`). The *value* is pinned by the
/// signed-off design (`_design.md`, `## Surfaces`, sign-off condition 2) and
/// changing it is a design amendment, not an edit.
pub(crate) const STEP: &str = "every page declares one need";

/// The most rules one rule atom may carry.
///
/// Deliberately the same number as `lint_constitution::MAX_RULES_PER_ATOM`
/// (`xtask/src/lint_constitution.rs:88`) and deliberately a second copy: RS-81-3
/// scopes a scanner to the tree whose behaviour it constrains, and a shared
/// constant would make one failure message answer two trees' questions. Two
/// trees teaching two different numbers for the same idea would be its own
/// defect, which is why the *value* agrees.
const MAX_RULES_PER_ATOM: usize = 6;

/// The largest a rule atom may be, in bytes.
///
/// Bytes rather than lines, for `lint_constitution::MAX_ATOM_BYTES`'s reason
/// (`:95`): a line ceiling is satisfied by writing longer lines, which is worse
/// for the token budget the cap protects.
const MAX_ATOM_BYTES: usize = 16_384;

/// The five section markers a rule carries, in the order they must appear.
const SECTIONS: [&str; 5] = [
    "**Why.**",
    "**Do**",
    "**Not**",
    "**Rejects.**",
    "**Evidence.**",
];

/// Whether to check the router's generated region or rewrite it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
    /// Fail when the region disagrees with the rule atoms.
    Check,
    /// Rewrite the region from the rule atoms.
    Write,
}

/// One problem, before it is a line.
///
/// A small struct rather than a formatted `String` at the push site, so the
/// sort and the composition contract live in one place each. Formatting at the
/// push site spreads `_design.md`'s `## Composition` S4 across a dozen call
/// sites and makes the density budget unmeasurable.
///
/// The derived `Ord` **is** the sort the design specifies — path, then line,
/// then message — and `None` ordering before `Some(_)` is what puts a
/// whole-file problem above the same file's line problems. The message is the
/// tie-break so that two problems at one location never reshuffle between runs
/// while a reader is working down the list.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Problem {
    /// Repository-relative, `/`-separated.
    path: String,
    /// 1-based line, when there is an offending line.
    ///
    /// `Option`, not a faked line 0: a page with no declaration has no line to
    /// point at, and the design's own S4 sample prints that problem as
    /// `{path} — …`.
    line: Option<usize>,
    /// What is wrong, then why it matters or what to do.
    message: String,
}

impl Problem {
    /// A problem at one line of one file.
    fn at(path: &str, line: usize, message: String) -> Self {
        Self {
            path: path.to_owned(),
            line: Some(line),
            message,
        }
    }

    /// A problem about a whole file, or a whole directory.
    fn whole(path: &str, message: String) -> Self {
        Self {
            path: path.to_owned(),
            line: None,
            message,
        }
    }

    /// The one line this problem prints as.
    ///
    /// `{path}:{line} — {what is wrong}; {why it matters, or what to do}`, with
    /// the location first and the repair inside the line rather than in a
    /// footer (`_design.md`, `## Hierarchy` S4, `## Anti-patterns` 12).
    fn render(&self) -> String {
        match self.line {
            Some(line) => format!("{}:{line} — {}", self.path, self.message),
            None => format!("{} — {}", self.path, self.message),
        }
    }
}

/// One rule atom, parsed far enough to check it.
#[derive(Debug)]
struct Atom {
    /// File name, e.g. `10-the-need-set.md`.
    file: String,
    /// The two-digit band, e.g. `10`.
    band: String,
    /// The whole file.
    text: String,
    /// The first `Load when:` source line, marker stripped.
    load_when: String,
    /// Every `## RP-` rule, as `(id, body)`.
    rules: Vec<(String, String)>,
}

impl Atom {
    /// Repository-relative path, which is what a problem line prints.
    fn path(&self) -> String {
        format!("{RULE_DIR}/{}", self.file)
    }
}

/// One `> **Answers:**` line, as the parser met it.
///
/// A near miss is *not* degraded to "no declaration": an unbackticked token or a
/// clause that does not end in `?` is reported at its own line as a malformed
/// declaration, because sending the author to the wrong repair is worse than
/// sending them to none.
#[derive(Debug)]
struct Declaration {
    /// 1-based line of the `> **Answers:**` line itself.
    line: usize,
    /// The token between backticks, when the line matches the settled grammar.
    token: Option<String>,
    /// Which part of the grammar failed, when it does not.
    malformed: Option<String>,
}

/// One governed page under the pinned pages tree.
#[derive(Debug)]
struct Page {
    /// Repository-relative, `/`-separated: `docs/append-conditions.md`.
    path: String,
    /// The page's directory, repository-relative: `docs`, or `docs/guide`.
    dir: String,
    /// Every declaration on it, in source order.
    declarations: Vec<Declaration>,
}

/// Runs every check, reporting all problems rather than the first.
///
/// # Errors
///
/// Fails when either pinned tree cannot be read, when either is empty, or when
/// any check finds a problem.
pub(crate) fn run(mode: Mode) -> Result<()> {
    // C2-07, called from here rather than run as a check of its own: this
    // module's `run` is the only entry point `xtask/src/main.rs`'s dispatch
    // already wires to a bare subcommand (`lint-pages`), and C2-07's fix is
    // scoped to `crates/happenstance-testkit/*`, this file and
    // `xtask/src/lints.rs` — adding a new named step belongs to `main.rs`'s
    // `REQUIRED` table, which is out of scope here. See
    // `no_stale_publication_claims`'s own doc comment, and the doc comment on
    // `lints::TESTKIT_LIB`, for why the check itself lives *here* rather than
    // as a `lints::`-shaped export next to `testkit_version` and
    // `stated_rule_counts`, which is where it would otherwise belong.
    no_stale_publication_claims()?;
    // C2-07b, called from here for the same reason and reported after it: the
    // negative matcher above names the two historical sentences, which is the
    // text a reader repairing a page wants first; the positive pin below is
    // what a *paraphrase* of either has to get past.
    positive_publication_pin()?;

    let root = workspace_root()?;

    let atoms = rule_atoms(&root)?;
    guard_rules_not_vacuous(&atoms)?;
    let pages = governed_pages(&root)?;
    guard_pages_not_vacuous(&pages)?;

    let mut problems: Vec<Problem> = Vec::new();

    check_router(&root, &atoms, mode, &mut problems)?;
    check_need_set(&atoms, &mut problems);
    for atom in &atoms {
        check_atom_shape(atom, &mut problems);
        check_atom_fences(atom, &mut problems);
    }
    check_declarations(&pages, &mut problems);
    check_orientation_ceiling(&pages, &mut problems);
    check_docs_index_lists_every_page(&root, &pages, &mut problems);

    report(&pages, &atoms, problems)
}

/// C2-07: neither of `happenstance-testkit`'s two rendered surfaces may tell
/// its reader a fact the repository already contradicts.
///
/// # Why this lives here and not beside `lints::testkit_version`
///
/// It reuses `lints::stale_publication_claims` and `lints`'s testkit-fact
/// constants — it is a testkit rendered-surface check in every way that
/// matters, and belongs there by subject. But `xtask/src/affected.rs`'s
/// `exported_lints` scans `lints.rs` for the exact shape `pub(crate) fn
/// NAME() -> Result<()> {` and requires `affected::run`'s unconditional block
/// to call each one it finds by name (`xtask/src/affected.rs:1016-1071`) —
/// the invariant that catches a lint wired into `REQUIRED` and forgotten in
/// the story grain. This check already runs in the story grain: `run` above
/// is called unconditionally by `affected::run`
/// (`xtask/src/affected.rs:183`), and `run` calls this. Giving it that same
/// shape in `lints.rs` would trip the scanner over reachability it cannot see
/// through a name match, and `xtask/src/affected.rs` is not a path this
/// change owns. So the entry point stays here, where `run` already reaches
/// it, and only the reusable, unit-testable half —
/// `lints::stale_publication_claims`, which does not have this shape — lives
/// in `lints.rs`.
///
/// # What this does not verify
///
/// That the commit shas the rustdoc must name are the *right* two — a
/// rewritten history with different hashes at the same content would still
/// satisfy this. Nor does it call crates.io: whether `happenstance-testkit`
/// is actually published is a fact about the registry this gate step does
/// not control, which is exactly why the corrected sentence is a claim about
/// two commits instead (RS-81-1,
/// `standards/rust/81-checks-that-cannot-be-types.md:11`). And it reads
/// `lints::SQLITE_CONFORMANCE_TEST`'s *path*, not its content — a file
/// emptied to nothing but its own name would still satisfy the README half.
///
/// # Errors
///
/// Returns an error if either document cannot be read, or if
/// [`crate::lints::stale_publication_claims`] finds a problem in either.
fn no_stale_publication_claims() -> Result<()> {
    use crate::lints::{
        SQLITE_CONFORMANCE_TEST, TESTKIT_LIB, TESTKIT_README, stale_publication_claims,
    };

    let root = workspace_root()?;
    let lib = fs::read_to_string(root.join(TESTKIT_LIB))
        .with_context(|| format!("reading {TESTKIT_LIB}"))?;
    let readme = fs::read_to_string(root.join(TESTKIT_README))
        .with_context(|| format!("reading {TESTKIT_README}"))?;
    let sqlite_conformance_exists = root.join(SQLITE_CONFORMANCE_TEST).exists();

    let problems = stale_publication_claims(&lib, &readme, sqlite_conformance_exists);
    if !problems.is_empty() {
        for p in &problems {
            println!("  {p}");
        }
        bail!(
            "{} stale publication claim(s) in the surfaces a happenstance-testkit reader meets \
             (C2-07: crates/happenstance-testkit tells its reader something the repository \
             already contradicts).",
            problems.len()
        );
    }

    println!(
        "C2-07: {TESTKIT_LIB} and {TESTKIT_README} state no publication claim the repository \
         contradicts"
    );
    Ok(())
}

/// The adapter whose conformance target settles whether *any* adapter has run
/// the suite.
///
/// Named rather than globbed: the README's sentence names this crate, so the
/// pin has to be able to say *which* adapter stopped mounting the suite. A glob
/// over `crates/*/tests/` would answer a different question — whether anything
/// anywhere still mounts it — and would leave the README's own subject
/// unchecked.
const ADAPTER_CRATE: &str = "happenstance-sqlite";

/// The adapter's own front page. Its `# Status:` heading is the sentence
/// `crates/happenstance-testkit/README.md` is pinned to, quoted rather than
/// paraphrased.
const ADAPTER_LIB: &str = "crates/happenstance-sqlite/src/lib.rs";

/// The workspace changelog. Its **oldest** released heading is the version this
/// workspace first published at, and unlike the newest it never moves.
const CHANGELOG: &str = "CHANGELOG.md";

/// `happenstance-testkit`'s own manifest — the second artefact the publication
/// fact is reconciled against, per RS-81-5.
const TESTKIT_MANIFEST: &str = "crates/happenstance-testkit/Cargo.toml";

/// The invocation that makes `crate::lints::SQLITE_CONFORMANCE_TEST` a *mount*
/// of the suite rather than a file with a promising name.
///
/// Fully qualified on purpose. `event_store_model_conformance!` is a different
/// family and does not contain this string, but a bare `event_store_conformance`
/// would also be satisfied by the module documentation above the invocation,
/// which discusses the macro at length (RS-81-2: prose about a construct reads
/// exactly like the construct).
const MOUNT: &str = "happenstance_testkit::event_store_conformance!";

/// The changelog's own words for its oldest release entry.
///
/// The pin quotes the artefact that settled the fact rather than wording this
/// module invented, so that a changelog which stops describing that entry as a
/// release fails here — naming the pin as the thing that moved — instead of the
/// pin quietly becoming `xtask`'s preference.
const FIRST_RELEASE_ANCHOR: &str = "The first published release";

/// The token every sentence in the testkit's rustdoc that speaks about this
/// crate's publication contains, in the true spelling and in the false one
/// alike.
///
/// It is a *counted* token, not a phrase to hunt: `nothing in this workspace is
/// published yet`, `this crate is not published yet` and `no published version
/// ever accepted it` all contain it, and only the last of those may stand.
const PUBLISH_TOKEN: &str = "publish";

/// The counted token for the README's half — present in `No adapter has run
/// this suite` and in `happenstance-sqlite has run this suite` alike.
const SUITE_TOKEN: &str = "run this suite";

/// The facts C2-07b's pin is held against, each read from the artefact that
/// settles it rather than stored here as a sentence.
///
/// # Why not `crates.io`
///
/// Whether `happenstance-testkit` is on the registry is a fact about the
/// registry, and a gate step that needs the network is a gate step that fails
/// on a train. The workspace's own release record answers the question the
/// rustdoc actually makes a claim about — *did this crate ship, and at what
/// version* — from two artefacts that disagree loudly when either moves.
#[derive(Debug)]
struct PublicationFacts {
    /// Version and date of the **oldest** released heading in [`CHANGELOG`], or
    /// `None` when it records no release at all.
    first_release: Option<(String, String)>,
    /// Whether [`CHANGELOG`] still describes that entry in the words
    /// [`FIRST_RELEASE_ANCHOR`] quotes.
    changelog_keeps_anchor: bool,
    /// Whether [`TESTKIT_MANIFEST`] withholds the crate from a registry.
    manifest_withholds: bool,
    /// The text after `# Status:` on [`ADAPTER_LIB`]'s front page, normalised.
    adapter_status: Option<String>,
    /// Whether `crate::lints::SQLITE_CONFORMANCE_TEST` still mounts [`MOUNT`].
    adapter_mounts_suite: bool,
}

/// The version and date of the oldest `## [x] — date` heading that is not
/// `[Unreleased]`.
///
/// The *oldest*, because the claim the rustdoc rests on is the **first**
/// publish. Keyed to the newest instead, the pin would demand a rewrite of a
/// sentence about history every time a release lands — and the repair that
/// teaches is deleting the pin.
fn first_release(changelog: &str) -> Option<(String, String)> {
    changelog
        .lines()
        .filter_map(|line| {
            let rest = line.trim().strip_prefix("## [")?;
            let (version, after) = rest.split_once(']')?;
            if version.eq_ignore_ascii_case("unreleased") {
                return None;
            }
            let date = after.trim().trim_start_matches(['—', '-']).trim();
            Some((version.to_owned(), date.to_owned()))
        })
        .next_back()
}

/// The text after the `# Status:` heading on a crate's front page.
///
/// Read from the heading rather than from the first paragraph under it: the
/// heading is the one line the adapter's author wrote to be quoted, and it is
/// the line `crates/happenstance-testkit/README.md` already cites by number.
fn status_line(front_page: &str) -> Option<String> {
    front_page.lines().find_map(|line| {
        let rest = line.trim().strip_prefix("//!")?;
        let status = collapsed(rest.trim().strip_prefix("# Status:")?);
        (!status.is_empty()).then_some(status)
    })
}

/// Whether `source` mounts [`MOUNT`] in code rather than mentioning it in prose.
///
/// The blind spot `crate::lints::stale_publication_claims` documents — *"it
/// reads `SQLITE_CONFORMANCE_TEST`'s path, not its content: a file emptied to
/// nothing but its own name would still satisfy the README half"* — is exactly
/// this function's job to close, so it reads the invocation and not the
/// filename.
fn mounts_suite(source: &str) -> bool {
    source
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with("//"))
        .any(|line| line.contains(MOUNT))
}

/// Whether a manifest withholds its crate from a registry.
///
/// Read from the **keys**, never from the file's prose: this manifest's comment
/// block discusses publication at length, so a whole-file `contains` would
/// report the opposite of the fact. Borrowed, shape and reasoning, from
/// `crates/happenstance-sqlite/tests/front_page.rs`'s
/// `manifest_withholds_publication`, which cannot be called from here — it is a
/// helper inside another crate's integration test.
fn manifest_withholds_publication(manifest: &str) -> bool {
    manifest.lines().map(str::trim).any(|line| {
        line.strip_prefix("publish").is_some_and(|rest| {
            let rest = rest.trim_start();
            rest.starts_with('=') && rest.contains("false")
        })
    })
}

/// Prose with its markers stripped, its emphasis removed and its whitespace
/// collapsed to single spaces.
///
/// Applied to both sides of every anchor match, which is what lets an anchor be
/// written as one readable sentence while the prose carrying it wraps across
/// three source lines and dresses a version in backticks. Choosing anchors that
/// happen to fit on one line and carry no formatting works today and breaks the
/// day somebody re-wraps a paragraph — and the repair a false positive teaches
/// is deleting the pin.
///
/// It is not a Markdown parser, for the same reason `crate::lints`'s `unwrapped`
/// states plainly that it is not one (RS-81-2,
/// `standards/rust/81-checks-that-cannot-be-types.md:95`): one marker per line,
/// prefix only, and emphasis dropped wherever it falls. That is enough to make a
/// substring search blind to wrapping and to backticks, and no more.
fn collapsed(text: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        let mut line = line.trim();
        for marker in ["//!", "///", "//", ">"] {
            if let Some(rest) = line.strip_prefix(marker) {
                line = rest.trim();
                break;
            }
        }
        for word in line.split_whitespace() {
            let word: String = word.chars().filter(|c| *c != '`' && *c != '*').collect();
            if word.is_empty() {
                continue;
            }
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(&word);
        }
    }
    out
}

/// The `///` and `//!` blocks of a Rust source file, as collapsed paragraphs.
///
/// The paragraph is the unit because a claim about publication lives in one, and
/// a *second* claim needs a second — which is the whole mechanism of the count
/// below. A blank doc line ends a paragraph, and so does any line that is not
/// documentation, so one item's docs never merge with the next item's.
fn doc_paragraphs(source: &str) -> Vec<String> {
    let mut paragraphs = Vec::new();
    let mut current = String::new();
    for line in source.lines() {
        let trimmed = line.trim();
        let text = trimmed
            .strip_prefix("//!")
            .or_else(|| trimmed.strip_prefix("///"));
        if let Some(text) = text
            && !text.trim().is_empty()
        {
            current.push_str(text);
            current.push('\n');
            continue;
        }
        if !current.is_empty() {
            paragraphs.push(collapsed(&std::mem::take(&mut current)));
        }
    }
    if !current.is_empty() {
        paragraphs.push(collapsed(&current));
    }
    paragraphs
}

/// A Markdown document as collapsed paragraphs, blockquote markers stripped.
///
/// A `>`-only line separates two paragraphs of a blockquote exactly as a blank
/// line separates two of body text, which matters because the README's status
/// claim is the *second* paragraph of a blockquote whose first paragraph is
/// about something else entirely.
fn markdown_paragraphs(text: &str) -> Vec<String> {
    let mut paragraphs = Vec::new();
    let mut current = String::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed
            .strip_prefix('>')
            .unwrap_or(trimmed)
            .trim()
            .is_empty()
        {
            if !current.is_empty() {
                paragraphs.push(collapsed(&std::mem::take(&mut current)));
            }
            continue;
        }
        current.push_str(trimmed);
        current.push('\n');
    }
    if !current.is_empty() {
        paragraphs.push(collapsed(&current));
    }
    paragraphs
}

/// The sentence the testkit's rustdoc must carry, composed from the release the
/// changelog records rather than from a version typed into this file.
fn publication_anchor(version: &str, date: &str) -> String {
    format!("first published at {version} on {date}")
}

/// Everything wrong with what `happenstance-testkit`'s two rendered surfaces say
/// about the two facts this tree has already settled, each problem worded so
/// that it names *which artefact moved*.
///
/// # Why this is a positive pin where `stale_publication_claims` is a negative one
///
/// That check hunts spellings of a falsehood, and a negative matcher over an
/// unbounded set of phrasings teaches the next paraphrase: every repair
/// enumerates one more sentence nobody is obliged to write. This one asserts the
/// *truth* instead — the crate is published at the version the changelog
/// records, and a named adapter has run the suite — and then counts the token a
/// contradicting sentence has to use. A paraphrase reinstating either falsehood
/// must delete a true statement to make room for it, or add a second paragraph
/// carrying the counted token; both fail, and neither route can be taken without
/// editing this function.
///
/// # What this does not verify
///
/// * **A falsehood written without the counted token.** *"Nothing here has
///   shipped yet"* carries no [`PUBLISH_TOKEN`]; *"no adapter has cleared the
///   bar"* carries no [`SUITE_TOKEN`]. Both pass every assertion here. That
///   blind spot is executed in
///   `tests::the_publication_pin_rejects_a_paraphrase_and_states_what_it_cannot_see`
///   rather than promised, per RS-81-1
///   (`standards/rust/81-checks-that-cannot-be-types.md:11`), so nobody reads
///   this pin as covering ground it does not. Closing it needs a reader.
/// * **The registry.** Whether `crates.io` serves the crate today is not asked.
///   The artefacts read here are what this workspace itself recorded about
///   shipping it, and a gate step that needs the network is one that fails on a
///   train.
/// * **That the anchored paragraph is *about* the anchor.** A paragraph carrying
///   the true sentence and then contradicting it in words containing no counted
///   token is invisible here, as above.
fn publication_pin_problems(facts: &PublicationFacts, lib: &str, readme: &str) -> Vec<String> {
    use crate::lints::{TESTKIT_LIB, TESTKIT_README};

    let mut problems = Vec::new();

    match (&facts.first_release, facts.manifest_withholds) {
        (Some((version, date)), false) => {
            let anchor = publication_anchor(version, date);
            let paragraphs = doc_paragraphs(lib);
            if !paragraphs.iter().any(|p| p.contains(&anchor)) {
                problems.push(format!(
                    "{TESTKIT_LIB} — does not say {anchor:?}. {CHANGELOG} records that release \
                     and {TESTKIT_MANIFEST} withholds nothing, so the rendered page is the only \
                     artefact still silent about it — and a page stating nothing cannot be \
                     paraphrased into stating the opposite (C2-07b)."
                ));
            }
            for paragraph in &paragraphs {
                if paragraph.contains(PUBLISH_TOKEN) && !paragraph.contains(&anchor) {
                    problems.push(format!(
                        "{TESTKIT_LIB} — mentions {PUBLISH_TOKEN:?} in a paragraph that does not \
                         carry {anchor:?}: {paragraph:?}. One anchored paragraph may speak about \
                         this crate's publication, because a second is a second claim and \
                         {CHANGELOG} has already settled which of the two is false (C2-07b)."
                    ));
                }
            }
        }
        (None, false) => problems.push(format!(
            "{CHANGELOG} — records no released version while {TESTKIT_MANIFEST} withholds \
             nothing: this pin holds a rendered page to the words of a published crate, and it \
             is the pin that is now stale rather than the page (C2-07b)."
        )),
        (Some(_), true) => problems.push(format!(
            "{TESTKIT_MANIFEST} — carries `publish = false` while {CHANGELOG} records a release: \
             the two artefacts disagree about whether this crate ships, and no sentence on the \
             page can be right about both (C2-07b)."
        )),
        (None, true) => problems.push(format!(
            "{TESTKIT_MANIFEST} — withholds the crate and {CHANGELOG} records no release: \
             nothing is published, so this pin must move before the page does (C2-07b)."
        )),
    }

    if !facts.changelog_keeps_anchor {
        problems.push(format!(
            "{CHANGELOG} — no longer says {FIRST_RELEASE_ANCHOR:?} of its oldest entry; the pin \
             quotes words the changelog does not use, so it has become this module's preference \
             rather than the changelog's decision (C2-07b)."
        ));
    }

    match (&facts.adapter_status, facts.adapter_mounts_suite) {
        (Some(status), true) => {
            let paragraphs = markdown_paragraphs(readme);
            if !paragraphs
                .iter()
                .any(|p| p.contains(status.as_str()) && p.contains(ADAPTER_CRATE))
            {
                problems.push(format!(
                    "{TESTKIT_README} — has no paragraph naming {ADAPTER_CRATE} that quotes its \
                     status, {status:?}, in the adapter's own words ({ADAPTER_LIB}). The suite \
                     is mounted; the README is the artefact still not saying so (C2-07b)."
                ));
            }
            for paragraph in &paragraphs {
                if paragraph.contains(SUITE_TOKEN) && !paragraph.contains(status.as_str()) {
                    problems.push(format!(
                        "{TESTKIT_README} — mentions {SUITE_TOKEN:?} in a paragraph that does not \
                         quote {status:?}: {paragraph:?}. One anchored paragraph may speak about \
                         whether an adapter has run the suite (C2-07b)."
                    ));
                }
            }
        }
        (_, false) => problems.push(format!(
            "{ADAPTER_CRATE} — its conformance target no longer mounts `{MOUNT}`, so no adapter \
             has run the suite and the README's positive claim is the stale one. Move the pin, \
             not the page (C2-07b)."
        )),
        (None, true) => problems.push(format!(
            "{ADAPTER_LIB} — carries no `# Status:` heading; the pin quotes that heading, so it \
             is now quoting words that do not exist (C2-07b)."
        )),
    }

    problems
}

/// C2-07b: both of `happenstance-testkit`'s rendered surfaces state the two
/// facts this tree has already settled, in the words of the artefacts that
/// settled them.
///
/// Called from [`run`] beside [`no_stale_publication_claims`], and for the same
/// reason that function's doc comment gives: `xtask/src/affected.rs`'s
/// `exported_lints` scans `xtask/src/lints.rs` for the shape `pub(crate) fn
/// NAME() -> Result<()>` and would trip over an entry point declared there that
/// already runs transitively.
///
/// The two are not redundant, and neither subsumes the other. That one forbids
/// two known sentences and names them in its failure text, which is what a
/// reader repairing the page needs; this one requires two true ones and counts
/// the token any contradiction has to use, which is what survives a rewrite that
/// says the same false thing in other words.
///
/// # Errors
///
/// Returns an error if any of the six artefacts cannot be read, or if
/// [`publication_pin_problems`] finds a problem.
fn positive_publication_pin() -> Result<()> {
    use crate::lints::{SQLITE_CONFORMANCE_TEST, TESTKIT_LIB, TESTKIT_README};

    let root = workspace_root()?;
    let read =
        |rel: &str| fs::read_to_string(root.join(rel)).with_context(|| format!("reading {rel}"));

    let lib = read(TESTKIT_LIB)?;
    let readme = read(TESTKIT_README)?;
    let changelog = read(CHANGELOG)?;
    let manifest = read(TESTKIT_MANIFEST)?;
    let adapter_front_page = read(ADAPTER_LIB)?;
    let conformance = read(SQLITE_CONFORMANCE_TEST)?;

    let facts = PublicationFacts {
        first_release: first_release(&changelog),
        changelog_keeps_anchor: changelog.contains(FIRST_RELEASE_ANCHOR),
        manifest_withholds: manifest_withholds_publication(&manifest),
        adapter_status: status_line(&adapter_front_page),
        adapter_mounts_suite: mounts_suite(&conformance),
    };

    let problems = publication_pin_problems(&facts, &lib, &readme);
    if !problems.is_empty() {
        for problem in &problems {
            println!("  {problem}");
        }
        bail!(
            "{} unpinned publication fact(s) — `positive_publication_pin` (C2-07b): a rendered \
             surface of happenstance-testkit no longer states a fact this tree has settled, so \
             the falsehood it replaced can return in a rewrite that deletes nothing.",
            problems.len()
        );
    }

    println!(
        "C2-07b: {TESTKIT_LIB} and {TESTKIT_README} state the publication and conformance facts \
         {CHANGELOG} and {ADAPTER_LIB} settled, in those artefacts' own words"
    );
    Ok(())
}

/// Prints the run's outcome, and is the only place that decides how.
///
/// # Errors
///
/// When there is at least one problem.
fn report(pages: &[Page], atoms: &[Atom], mut problems: Vec<Problem>) -> Result<()> {
    if problems.is_empty() {
        let rules: usize = atoms.iter().map(|atom| atom.rules.len()).sum();
        println!("  {} pages, {rules} rules, all consistent", pages.len());
        return Ok(());
    }

    problems.sort();
    for problem in &problems {
        eprintln!("  {}", problem.render());
    }
    bail!(
        "{} problem(s) in {RULE_DIR} + {}",
        problems.len(),
        lint_narrative::TREE
    )
}

/// Reads and parses every rule atom under [`RULE_DIR`].
///
/// Top-level `.md` files only, `README.md` excluded — the router is not an atom,
/// and `examples/` holds the deliberately-broken worked example band 40 links,
/// which must stay permanently breakable without making a gate red.
///
/// # Errors
///
/// When [`RULE_DIR`] cannot be read, naming the path the gate expected rather
/// than reporting an empty tree; or when an atom is not named `NN-slug.md`.
fn rule_atoms(root: &Path) -> Result<Vec<Atom>> {
    let dir = root.join(RULE_DIR);
    let entries = fs::read_dir(&dir).with_context(|| format!("reading {RULE_DIR}"))?;

    let mut files: Vec<String> = Vec::new();
    for entry in entries {
        let entry = entry.with_context(|| format!("reading an entry of {RULE_DIR}"))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if is_markdown(&name) && !name.eq_ignore_ascii_case("README.md") {
            files.push(name);
        }
    }
    files.sort();

    let mut out = Vec::new();
    for file in files {
        let text = fs::read_to_string(dir.join(&file))
            .with_context(|| format!("reading {RULE_DIR}/{file}"))?;
        let stem = file.strip_suffix(".md").unwrap_or(&file);
        let Some((band, _)) = stem.split_once('-') else {
            bail!("{RULE_DIR}/{file} — a rule atom is named `NN-slug.md`; this has no band");
        };
        if band.len() != 2 || !band.chars().all(|c| c.is_ascii_digit()) {
            bail!("{RULE_DIR}/{file} — the band `{band}` is not two digits");
        }
        out.push(Atom {
            band: band.to_owned(),
            load_when: load_when(&text),
            rules: rules(&text),
            file,
            text,
        });
    }
    Ok(out)
}

/// Refuses a rules tree with no rule atoms in it.
///
/// # Errors
///
/// When [`RULE_DIR`] holds no rule atom.
fn guard_rules_not_vacuous(atoms: &[Atom]) -> Result<()> {
    if atoms.is_empty() {
        bail!("{RULE_DIR} holds no rule atoms, so every check below is vacuous");
    }
    Ok(())
}

/// Every governed page under the pinned pages tree, in path order.
///
/// The tree is [`crate::lint_narrative::TREE`] and **no second constant names
/// it**: moving the tree is one edit, and a `PAGE_DIR` declared here would be
/// the *three lists that must agree* defect `xtask/src/spec_trace.rs:122-160`
/// records, with the second copy drifting silently the day the tree moves.
///
/// The tree's index is not a page. That is not this module's judgement: the
/// pinned tree's own checker states it — `xtask/src/lint_narrative.rs:232-241`
/// ("the only file under `TREE` that is not a page") and `:486-491` ("a tree
/// holding only its own routing table holds nothing to check").
///
/// # Errors
///
/// When the tree, or any directory or page under it, cannot be read — naming
/// *that* path, because a file the checker cannot read is not a file with no
/// problems.
fn governed_pages(root: &Path) -> Result<Vec<Page>> {
    let mut out = Vec::new();
    collect_pages(root, "", &mut out)?;
    out.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(out)
}

/// Every markdown page at or below `rel_dir`, which is tree-relative.
fn collect_pages(root: &Path, rel_dir: &str, out: &mut Vec<Page>) -> Result<()> {
    let tree = lint_narrative::TREE;
    let here = if rel_dir.is_empty() {
        tree.to_owned()
    } else {
        format!("{tree}/{rel_dir}")
    };

    let entries = fs::read_dir(root.join(&here)).with_context(|| format!("reading {here}"))?;

    let mut found: Vec<(String, bool)> = Vec::new();
    for entry in entries {
        let entry = entry.with_context(|| format!("reading an entry of {here}"))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let is_dir = entry
            .file_type()
            .with_context(|| format!("reading the type of {here}/{name}"))?
            .is_dir();
        found.push((name, is_dir));
    }
    found.sort();

    for (name, is_dir) in found {
        let rel = if rel_dir.is_empty() {
            name.clone()
        } else {
            format!("{rel_dir}/{name}")
        };
        if is_dir {
            collect_pages(root, &rel, out)?;
        } else if is_markdown(&name) && !name.eq_ignore_ascii_case("README.md") {
            let path = format!("{tree}/{rel}");
            let text =
                fs::read_to_string(root.join(&path)).with_context(|| format!("reading {path}"))?;
            out.push(Page {
                dir: here.clone(),
                declarations: declarations(&text),
                path,
            });
        }
    }
    Ok(())
}

/// Refuses a pages tree with no governed page in it.
///
/// # Errors
///
/// When the pinned pages tree holds no page.
fn guard_pages_not_vacuous(pages: &[Page]) -> Result<()> {
    if pages.is_empty() {
        bail!(
            "{} holds no pages, so every check below is vacuous",
            lint_narrative::TREE
        );
    }
    Ok(())
}

/// Whether a file name is a markdown page, however it is cased.
fn is_markdown(name: &str) -> bool {
    name.len() > ".md".len()
        && name
            .get(name.len() - ".md".len()..)
            .is_some_and(|ext| ext.eq_ignore_ascii_case(".md"))
}

/// Every `> **Answers:**` line in one page's text, with its 1-based line.
///
/// The one parser. The counting check, the membership check, the `orientation`
/// ceiling and every test take their answer from here, because two parsers that
/// agree today drift tomorrow.
///
/// Prose that merely *mentions* a need word is not a declaration: only a
/// blockquote line whose content opens `**Answers:**` is one. Lines inside a
/// fenced block are skipped, so a page that *teaches* the declaration form —
/// band 00 does exactly this — is not read as declaring one. That is a stated
/// blind spot rather than a silent one: a second declaration hidden inside a
/// fence is invisible here, and the instrument for it is band 40's non-author
/// walk.
fn declarations(text: &str) -> Vec<Declaration> {
    let mut out = Vec::new();
    let mut fenced = false;
    for (index, line) in text.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        let Some(rest) = line.trim_start().strip_prefix('>') else {
            continue;
        };
        let Some(after) = rest.trim_start().strip_prefix("**Answers:**") else {
            continue;
        };
        out.push(declaration(index + 1, after));
    }
    out
}

/// One `> **Answers:**` line's remainder, read against the settled grammar.
///
/// A backticked token, ` — `, and a clause ending `?`. Each half that fails
/// names *itself*, because "malformed" without a part is the same non-answer as
/// silently reporting the line as absent.
fn declaration(line: usize, after: &str) -> Declaration {
    let malformed = |why: &str| Declaration {
        line,
        token: None,
        malformed: Some(why.to_owned()),
    };

    let Some(rest) = after.trim_start().strip_prefix('`') else {
        return malformed("the token is not in backticks");
    };
    let Some((token, rest)) = rest.split_once('`') else {
        return malformed("the token is not in backticks");
    };
    if token.is_empty() {
        return malformed("the token is empty");
    }
    let Some(question) = rest.strip_prefix(" — ") else {
        return malformed("no ` — ` between the token and the question");
    };
    if !question.trim_end().ends_with('?') {
        return malformed("the question does not end in `?`");
    }
    Declaration {
        line,
        token: Some(token.to_owned()),
        malformed: None,
    }
}

/// The declared tokens, in the shape a problem line names them.
fn listed(found: &[Declaration]) -> String {
    let names: Vec<String> = found
        .iter()
        .map(|declaration| {
            declaration.token.as_ref().map_or_else(
                || "a malformed line".to_owned(),
                |token| format!("`{token}`"),
            )
        })
        .collect();
    match names.split_last() {
        None => String::new(),
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
    }
}

/// The enumerated set, as a problem line spells it.
fn enumerated() -> String {
    NEEDS
        .iter()
        .map(|member| member.token)
        .collect::<Vec<_>>()
        .join(", ")
}

/// AC-004 — zero, two, malformed, and unenumerated declarations.
fn check_declarations(pages: &[Page], problems: &mut Vec<Problem>) {
    for page in pages {
        if page.declarations.is_empty() {
            problems.push(Problem::whole(
                &page.path,
                format!("no `> **Answers:**` line; see {RULE_DIR}/00-one-need.md"),
            ));
            continue;
        }

        // At the *second* declaration's line: the first one is not the mistake,
        // and a page that strains to be two things is split rather than granted
        // a fifth token.
        if let Some(second) = page.declarations.get(1) {
            problems.push(Problem::at(
                &page.path,
                second.line,
                format!(
                    "declares {}; a page answers one need",
                    listed(&page.declarations)
                ),
            ));
        }

        for found in &page.declarations {
            if let Some(why) = &found.malformed {
                problems.push(Problem::at(
                    &page.path,
                    found.line,
                    format!(
                        "malformed `> **Answers:**` line: {why}; see {RULE_DIR}/00-one-need.md"
                    ),
                ));
                continue;
            }
            if let Some(token) = &found.token
                && need(token).is_none()
            {
                problems.push(Problem::at(
                    &page.path,
                    found.line,
                    format!(
                        "`{token}` is not a need: {}; see {RULE_DIR}/{NEED_SET_ATOM}",
                        enumerated()
                    ),
                ));
            }
        }
    }
}

/// RP-10-3 — at most one `orientation` page per directory level.
///
/// The ceiling that stops the one need this set *added* from becoming the sink
/// the taxonomy it started from would have made it.
fn check_orientation_ceiling(pages: &[Page], problems: &mut Vec<Problem>) {
    let mut by_directory: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for page in pages {
        for found in &page.declarations {
            if found.token.as_deref() == Some("orientation") {
                by_directory
                    .entry(page.dir.as_str())
                    .or_default()
                    .push(format!("{}:{}", page.path, found.line));
            }
        }
    }

    for (directory, offenders) in by_directory {
        if offenders.len() > 1 {
            problems.push(Problem::whole(
                directory,
                format!(
                    "{} `orientation` pages at one directory level ({}); RP-10-3 allows one",
                    offenders.len(),
                    offenders.join(", ")
                ),
            ));
        }
    }
}

/// The narrative table `docs/README.md` routes readers through, quoted by its
/// header line so [`table_rows`] can find it among the second, unrelated table
/// the same page carries.
const DOCS_INDEX_TABLE_HEADER: &str = "| Page | Read it at |";

/// V-4 — `docs/README.md` carries one row per page under the pinned narrative
/// tree, or a reader following the index never reaches the page that is missing
/// one.
///
/// # Why derived rather than counted
///
/// `pages` already comes from a directory listing ([`governed_pages`]), not
/// from a number typed into this file, so a **seventh** page added to `docs/`
/// without a row of its own fails this the same way the two omitted here did —
/// RS-81-5: the derived fact and the hand-written table are reconciled, and the
/// failure names which page the table is missing rather than a count that
/// silently drifted.
///
/// # What this does not verify
///
/// That the row's *prose* describes the page well, or that the row sits in the
/// table's `| Page | Read it at |` half rather than the second `| Looking for |
/// It is at |` table — a link to the page's file name anywhere in the whole
/// document satisfies this. The reviewer is the instrument for whether a row
/// reads like an invitation; this is the instrument for whether the row exists
/// at all. Whole-file rather than table-scoped is a deliberate looseness: the
/// two tables link disjoint targets in practice, and scoping to one region
/// would trade a false negative it does not currently have for a parser this
/// module would then have to maintain.
fn check_docs_index_lists_every_page(root: &Path, pages: &[Page], problems: &mut Vec<Problem>) {
    const DOCS_INDEX: &str = "docs/README.md";

    let text = match fs::read_to_string(root.join(DOCS_INDEX)) {
        Ok(text) => text,
        Err(err) => {
            problems.push(Problem::whole(
                DOCS_INDEX,
                format!("could not be read: {err}"),
            ));
            return;
        }
    };

    let mut linked_anywhere: Vec<String> = Vec::new();
    for line in text.lines() {
        linked_anywhere.extend(markdown_link_targets(line));
    }

    for page in pages {
        let Some(basename) = page.path.rsplit('/').next() else {
            continue;
        };
        if !linked_anywhere.iter().any(|target| target == basename) {
            problems.push(Problem::whole(
                DOCS_INDEX,
                format!(
                    "the `{DOCS_INDEX_TABLE_HEADER}` table carries no row linking `{basename}`; \
                     every page under the pinned narrative tree needs a route in from the index, \
                     and this one ({}) has none (V-4: check_docs_index_lists_every_page)",
                    page.path
                ),
            ));
        }
    }
}

/// The rule atom's own shape: head, sections, and the two ceilings.
fn check_atom_shape(atom: &Atom, problems: &mut Vec<Problem>) {
    let path = atom.path();

    if !atom.text.starts_with(&format!("# {} — ", atom.band)) {
        problems.push(Problem::at(
            &path,
            1,
            format!("the title must read `# {} — <title>`", atom.band),
        ));
    }
    if atom.load_when.is_empty() {
        problems.push(Problem::whole(
            &path,
            "no `> **Load when:**` line; it is the router's index source".to_owned(),
        ));
    }
    if !atom.text.contains("**See also:**") {
        problems.push(Problem::whole(
            &path,
            "no `> **See also:**` line; a rule nobody can leave is a rule nobody re-enters"
                .to_owned(),
        ));
    }
    if atom.rules.is_empty() {
        problems.push(Problem::whole(
            &path,
            "carries no `## RP-` rule; an atom with no rule states nothing".to_owned(),
        ));
    }
    if atom.rules.len() > MAX_RULES_PER_ATOM {
        problems.push(Problem::whole(
            &path,
            format!(
                "{} rules, and the ceiling is {MAX_RULES_PER_ATOM}; split the atom",
                atom.rules.len()
            ),
        ));
    }
    if atom.text.len() > MAX_ATOM_BYTES {
        problems.push(Problem::whole(
            &path,
            format!(
                "{} bytes, and the ceiling is {MAX_ATOM_BYTES}; an agent loading this pays \
                 for all of it",
                atom.text.len()
            ),
        ));
    }

    for (id, body) in &atom.rules {
        let line = rule_line(&atom.text, id);
        let mut previous = 0;
        for marker in SECTIONS {
            let Some(at) = body.lines().position(|line| line.starts_with(marker)) else {
                problems.push(Problem::at(
                    &path,
                    line,
                    format!("{id} has no `{marker}` section; a rule that names no wrong page is decorative"),
                ));
                continue;
            };
            if at < previous {
                problems.push(Problem::at(
                    &path,
                    line,
                    format!("{id} reaches `{marker}` out of order; the five sections run Why. · Do · Not · Rejects. · Evidence."),
                ));
            }
            previous = at;
        }
    }
}

/// Fences: `rust`-tagged and untagged are both rejected, with the reason.
fn check_atom_fences(atom: &Atom, problems: &mut Vec<Problem>) {
    let path = atom.path();
    for (line, why) in fence_problems(&atom.text) {
        problems.push(Problem::at(&path, line, why));
    }
}

/// The router's links resolve, and its `## Index` agrees with the atoms.
///
/// # Errors
///
/// When the router cannot be read, or cannot be rewritten under [`Mode::Write`].
fn check_router(
    root: &Path,
    atoms: &[Atom],
    mode: Mode,
    problems: &mut Vec<Problem>,
) -> Result<()> {
    let path = root.join(ROUTER);
    let router = fs::read_to_string(&path).with_context(|| format!("reading {ROUTER}"))?;

    // Deliberately a link check and not a mention count: a markdown link spells
    // its target twice, so counting mentions reports every correctly-linked atom
    // as a duplicate — the mistake the constitution's own check made first
    // (`xtask/src/lint_constitution.rs:339-343`).
    for (index, line) in router.lines().enumerate() {
        for target in markdown_link_targets(line) {
            if !is_markdown(&target) {
                continue;
            }
            if !root.join(RULE_DIR).join(&target).exists() {
                problems.push(Problem::at(
                    ROUTER,
                    index + 1,
                    format!("link `{target}` resolves to no file"),
                ));
            }
        }
    }

    let generated = generated_index(atoms);
    match region(&router) {
        None => problems.push(Problem::whole(
            ROUTER,
            "no `<!-- BEGIN GENERATED -->` / `<!-- END GENERATED -->` region; the `## Index` \
             is generated and cannot be checked without it"
                .to_owned(),
        )),
        Some((start, end)) => {
            let current: Vec<&str> = router.lines().collect();
            if current[start..end].join("\n").trim() != generated.trim() {
                if mode == Mode::Write {
                    let mut rebuilt: Vec<String> =
                        current[..start].iter().map(|l| (*l).to_owned()).collect();
                    rebuilt.push(generated);
                    rebuilt.extend(current[end..].iter().map(|l| (*l).to_owned()));
                    fs::write(&path, rebuilt.join("\n") + "\n")
                        .with_context(|| format!("writing {ROUTER}"))?;
                    println!("  rewrote {ROUTER}'s generated `## Index`");
                } else {
                    problems.push(Problem::whole(
                        ROUTER,
                        "the generated `## Index` disagrees with the rule atoms; run \
                         `cargo xtask lint-pages --write`"
                            .to_owned(),
                    ));
                }
            }
        }
    }
    Ok(())
}

/// What the router's `## Index` region must contain.
///
/// Byte for byte `lint_constitution::generated_region`'s format
/// (`xtask/src/lint_constitution.rs:400-420`) — the shape
/// `router-precedence-and-announcement` committed the region in, as a forward
/// contract on this generator: the first `--write` against that router must
/// produce no diff.
fn generated_index(atoms: &[Atom]) -> String {
    let mut out = vec![
        "| Atom | Load when | Rules |".to_owned(),
        "|---|---|---|".to_owned(),
    ];
    for atom in atoms {
        let ids: Vec<&str> = atom.rules.iter().map(|(id, _)| id.as_str()).collect();
        out.push(format!(
            "| [`{}`]({}) | {} | {} |",
            atom.file,
            atom.file,
            if atom.load_when.is_empty() {
                "—".to_owned()
            } else {
                atom.load_when.replace('|', "\\|")
            },
            ids.join(", ")
        ));
    }
    out.join("\n")
}

/// The line range strictly between the generated-region markers.
fn region(router: &str) -> Option<(usize, usize)> {
    let lines: Vec<&str> = router.lines().collect();
    let start = lines
        .iter()
        .position(|line| line.trim() == "<!-- BEGIN GENERATED -->")?;
    let end = lines
        .iter()
        .position(|line| line.trim() == "<!-- END GENERATED -->")?;
    (start < end).then_some((start + 1, end))
}

/// AC-008 — [`NEEDS`] and band 10's token table cannot disagree in silence.
///
/// Containment, exclusion, and the job cell — the two fields [`Need`] carries,
/// each with the machine that reads it that band 10 claims exists. The failure
/// names **which token moved** (RS-81-5,
/// `standards/rust/81-checks-that-cannot-be-types.md:335`) and cites both paths.
///
/// Deliberately **not** `--write`-repairable, and the message says so: the atom
/// carries a third column — *success for the reader* — that is prose no `const`
/// holds, so a generator would have to invent it.
///
/// The exclusion half is scoped to the token *table*, not to the whole atom.
/// Band 10 argues at length about `reference` in its prose, and an atom that may
/// not name the token it subtracted cannot state why it subtracted it.
fn check_need_set(atoms: &[Atom], problems: &mut Vec<Problem>) {
    let Some(atom) = atoms.iter().find(|atom| atom.file == NEED_SET_ATOM) else {
        problems.push(Problem::whole(
            RULE_DIR,
            format!("{NEED_SET_ATOM} is missing, so `NEEDS` is checked against nothing"),
        ));
        return;
    };

    let path = atom.path();
    let rows = need_table_rows(&atom.text);
    let two_part = "the const and the table move in one commit, and this one is not \
                    `--write`-repairable";

    for member in NEEDS {
        match rows.iter().find(|(token, _)| token == member.token) {
            None => problems.push(Problem::whole(
                &path,
                format!(
                    "`{}` is in `NEEDS` (xtask/src/lint_pages.rs) and not in this table; \
                     {two_part}",
                    member.token
                ),
            )),
            Some((_, job)) if job != member.job => problems.push(Problem::whole(
                &path,
                format!(
                    "`{}`'s job cell disagrees with `NEEDS` (xtask/src/lint_pages.rs); \
                     {two_part}",
                    member.token
                ),
            )),
            Some(_) => {}
        }
    }

    for (token, _) in &rows {
        if need(token).is_none() {
            problems.push(Problem::whole(
                &path,
                format!(
                    "`{token}` is in this table and not in `NEEDS` (xtask/src/lint_pages.rs); \
                     {two_part}"
                ),
            ));
        }
    }
}

/// The `Load when:` triggers of an atom, or the empty string.
///
/// Mirrors `lint_constitution::load_when` (`xtask/src/lint_constitution.rs:247-257`)
/// deliberately, including its limit: only the **first** source line of the
/// block is read, and a continuation on the next `>` line is silently dropped.
/// The router's `## The shape of a rule` states the one-source-line constraint
/// precisely because this is what builds the index cell.
fn load_when(text: &str) -> String {
    for line in text.lines() {
        let trimmed = line.trim_start_matches(['>', ' ']);
        if let Some(rest) = trimmed.strip_prefix("**Load when:**") {
            return rest.trim().to_owned();
        }
    }
    String::new()
}

/// Every `## RP-` rule in an atom, as `(id, body)`.
///
/// Split at the next line beginning `## `, which is how
/// `lint_constitution::rules` splits and why a `### ` sub-heading stays inside
/// the rule it belongs to.
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

/// The 1-based line of the heading that opens `id`, or 1.
fn rule_line(text: &str, id: &str) -> usize {
    text.lines()
        .position(|line| line.starts_with(&format!("## {id}.")))
        .map_or(1, |index| index + 1)
}

/// Every `](target)` on one line, target only.
///
/// A markdown link spells its target twice, which is why this reads the
/// parenthesised half rather than counting mentions — the mistake the
/// constitution's own link check made first
/// (`xtask/src/lint_constitution.rs:339-343`).
fn markdown_link_targets(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(at) = rest.find("](") {
        rest = &rest[at + 2..];
        if let Some(close) = rest.find(')') {
            out.push(rest[..close].split('#').next().unwrap_or("").to_owned());
            rest = &rest[close + 1..];
        } else {
            break;
        }
    }
    out
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

/// Band 10's need table, as `(token, the page's job)` — its first two columns.
///
/// The third column, *success for the reader*, is read by nothing on purpose:
/// it is the prose that makes this agreement check un-generatable, which is
/// exactly why it is not a field of [`Need`].
fn need_table_rows(text: &str) -> Vec<(String, String)> {
    table_rows(text, "| Token |")
        .iter()
        .filter_map(|row| {
            let token = cell(row, 0)?;
            let token = token.strip_prefix('`')?.strip_suffix('`')?.to_owned();
            Some((token, cell(row, 1).unwrap_or_default()))
        })
        .collect()
}

/// One cell of a markdown table row, zero-indexed, trimmed.
fn cell(row: &str, index: usize) -> Option<String> {
    row.trim_start_matches('|')
        .split('|')
        .nth(index)
        .map(|cell| cell.trim().to_owned())
}

/// One fenced block, as a walk over the file meets it.
struct Fence {
    /// The 1-based line of the **opening** marker, so a failure message pastes
    /// into an editor.
    line: usize,
    /// The opener's info string, trimmed. Empty means an untagged fence.
    info: String,
    /// Whether a closing marker was found before the end of the file.
    closed: bool,
}

/// Every fence in `text`, judged by a state walk rather than line by line.
///
/// The walk is the whole point, and it is not defensive engineering. In
/// `CommonMark` a fence's **closing** line is spelled exactly like an
/// **untagged opener** — three backticks and nothing else — so no per-line
/// predicate can tell them apart, and every shape that tries gets one of the
/// two cases wrong:
///
/// * skip the empty info string, and an untagged opener is waved through along
///   with the closers it is hiding among;
/// * do not skip it, and the closer of a perfectly good `text` fence is judged
///   as an opener and fails, blaming an untagged fence;
/// * count the bare lines instead and check the count is even, and nothing can
///   ever fail, because a well-formed untagged fence contributes exactly two of
///   them.
///
/// This tree shipped the first two at once — the second in the router's copy —
/// and then substituted the third for the untagged half. Toggling on each
/// marker and reporting only the opening side is what makes the two spellings
/// distinguishable, and is why there is one function rather than a copy per
/// call site.
///
/// A fence marker is recognised by prefix rather than by exact match, so a
/// longer run (` ```` `) or a trailing space still toggles the state; that is
/// `CommonMark`'s own rule and it keeps the walk in phase.
fn fences(text: &str) -> Vec<Fence> {
    let mut out: Vec<Fence> = Vec::new();
    let mut open = false;
    for (index, line) in text.lines().enumerate() {
        let Some(info) = line.strip_prefix("```") else {
            continue;
        };
        if open {
            open = false;
            if let Some(last) = out.last_mut() {
                last.closed = true;
            }
            continue;
        }
        open = true;
        out.push(Fence {
            line: index + 1,
            info: info.trim().to_owned(),
            closed: false,
        });
    }
    out
}

/// Every wrongly tagged fence in one file, as `(line, why)`.
///
/// The single implementation of the fence rule, called by the checker over
/// every rule atom **and** by the tests over the router. Two copies of this
/// rule is how the tree came to hold two different wrong implementations of it.
fn fence_problems(text: &str) -> Vec<(usize, String)> {
    let mut wrong = Vec::new();
    for fence in fences(text) {
        let Fence { line, info, closed } = fence;
        if !closed {
            wrong.push((
                line,
                "a fence is opened here and never closed; every fence after it is read \
                 inside-out, so this is named before its tag is judged"
                    .to_owned(),
            ));
            continue;
        }
        if info.is_empty() {
            wrong.push((
                line,
                "an untagged fence opener; a bare fence is as wrong as a `rust`-tagged \
                 one, so that a future decision to register this tree with the doctest \
                 harness cannot be undermined retroactively. Tag it `text` or `markdown`"
                    .to_owned(),
            ));
        } else if info != "text" && info != "markdown" {
            wrong.push((
                line,
                format!(
                    "a fence tagged `{info}`; nothing in the workspace compiles this \
                     tree, so `text` or `markdown` is the honest tag"
                ),
            ));
        }
    }
    wrong
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

    /// Band 30 — a page cites a clause id and never restates the clause.
    const BAND_30: &str = "30-citing-the-specification.md";

    /// Band 40 — the non-author verdict walk, and the paraphrase spot check.
    const BAND_40: &str = "40-reviewing-a-page.md";

    /// The worked example band 40's walk is calibrated against.
    ///
    /// Deliberately inert: it sits under `examples/`, so a corpus reader that
    /// takes top-level `.md` files only never sees it, and it can stay
    /// permanently broken without ever making a gate red.
    const FIXTURE: &str = "examples/two-needs.md";

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
    const TREE: &[&str] = &[BAND_00, BAND_10, BAND_20, BAND_30, BAND_40];

    /// The precedence chain, copied from `standards/rust/README.md:25-26` —
    /// the chain's own words, without the closing period of the sentence that
    /// carries them.
    ///
    /// A literal here rather than a read of that file. A test in *this* tree
    /// that opens the constitution's router turns an edit to the constitution
    /// into a red pages-tree test with a pages-tree message, which is the one
    /// error message answering two trees' questions that RS-81-3 forbids
    /// (`standards/rust/81-checks-that-cannot-be-types.md:209`, architecture
    /// brief Note 6; Note 8 lists `standards/rust/` under what must not move).
    /// That the chain itself is unedited is a ledger-side fact with its own
    /// command: `git diff main -- standards/rust/README.md`.
    const PRECEDENCE_CHAIN: &str = "SPECIFICATION clause > ADR > constitution atom > \
                                    `CLAUDE.md` / `CONTRIBUTING.md` summary > \
                                    `references/evaluation/*`";

    /// The generated region's header and separator, copied from
    /// `standards/rust/README.md:68-69`.
    ///
    /// Inlined for the same reason as [`PRECEDENCE_CHAIN`], and it is what makes
    /// the checker story's `generated_region` analogue a copy of the precedent
    /// rather than a variant of it.
    const GENERATED_HEADER: [&str; 2] = ["| Atom | Load when | Rules |", "|---|---|---|"];

    fn root() -> PathBuf {
        workspace_root().unwrap()
    }

    /// [`fence_problems`], rendered as the `file:line — why` lines these tests
    /// read.
    ///
    /// A test-side spelling of the checker's own formatter, so the specimens
    /// that prove the fence rule can fail are ordinary assertions on ordinary
    /// values — a `#[should_panic]` proves a panic happened somewhere and not
    /// that it happened at the right line for the right reason.
    fn fence_tag_problems(file: &str, text: &str) -> Vec<String> {
        fence_problems(text)
            .into_iter()
            .map(|(line, why)| format!("{file}:{line} — {why}"))
            .collect()
    }

    /// The tokens in the first column of band 10's need table.
    fn need_table_tokens(text: &str) -> Vec<String> {
        need_table_rows(text)
            .into_iter()
            .map(|(token, _)| token)
            .collect()
    }

    /// One named atom under [`RULE_DIR`], read whole.
    ///
    /// Named paths only, never `read_dir`: the directory-reading corpus reader
    /// belongs to `page-need-checker-mounted-in-the-gate`, and building a second
    /// one here is the duplication this slice's ordering exists to prevent.
    fn atom(file: &str) -> String {
        let path = root().join(RULE_DIR).join(file);
        fs::read_to_string(&path).unwrap_or_else(|err| panic!("reading {}: {err}", path.display()))
    }

    /// The rules tree's router, read whole.
    fn router() -> String {
        let path = root().join(ROUTER);
        fs::read_to_string(&path).unwrap_or_else(|err| panic!("reading {}: {err}", path.display()))
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

    /// This module's `//!` block, marker stripped, in source order.
    fn module_docs() -> String {
        THIS_FILE
            .lines()
            .take_while(|line| line.starts_with("//!"))
            .map(|line| line.trim_start_matches("//!").trim_start())
            .collect::<Vec<_>>()
            .join("\n")
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

        // The chain is quoted here, and the quote is compared with an inlined
        // copy rather than with `standards/rust/README.md` itself — see
        // `PRECEDENCE_CHAIN` for why this tree's tests do not open that file,
        // and for the ledger-side command that proves it unedited.
        //
        // Blockquote markers are stripped and the whole file flattened first:
        // the router wraps the chain across two `>` lines, and on this Windows
        // checkout it wraps with CRLF besides.
        assert!(
            unquoted(&text).contains(PRECEDENCE_CHAIN),
            "the router quotes the chain verbatim rather than re-deriving it: \
             it must carry {PRECEDENCE_CHAIN}"
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
        // Compared against `GENERATED_HEADER`, an inlined copy, rather than
        // against the constitution's live region: a pages-tree test may only
        // fail for pages-tree reasons (RS-81-3).
        assert_eq!(
            region.first().map(String::as_str),
            Some(GENERATED_HEADER[0]),
            "the region's header row is the constitution's verbatim"
        );
        assert_eq!(
            region.get(1).map(String::as_str),
            Some(GENERATED_HEADER[1]),
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
        let prose = flat(&text);
        assert!(
            text.contains("## What checks this tree, and what does not"),
            "a check whose limits are undocumented is read as a guarantee \
             (RS-81-1); so is a tree whose reader assumes the gate is watching it"
        );
        assert!(
            prose.contains(&format!("`{STEP}` step reads this tree")),
            "the router names the step by the name `REQUIRED` carries, so a reader \
             can find it in the gate's own output"
        );
        assert!(
            prose.contains("walking this directory"),
            "and says *how* it reads the tree: the corpus reader is what makes a \
             green run a statement about the directory rather than about a list"
        );
        assert!(
            !prose.contains("No **dedicated** gate step reads this tree yet"),
            "the dedicated step landed with `page-need-checker-mounted-in-the-gate`; \
             a router that still says it has not is a false statement in the one \
             place a reader meets the question — RS-81-1 inverted"
        );

        // This module's own tests read every atom named in `TREE` on every
        // `cargo xtask ci`. A router claiming otherwise ships a false statement
        // about what checks it, in the one place a reader meets the question —
        // RS-81-1 inverted, and never-fold class 5 broken by the tree that
        // wrote the class.
        assert!(
            !prose.contains("no gate step reads `standards/pages/`"),
            "the tree *is* read by the gate; the router may not say it is not"
        );
    }

    #[test]
    fn the_repository_index_reaches_the_rules_tree_in_one_hop() {
        // Flattened to one whitespace-separated line before matching: the
        // sentences below are prose that wraps, and on this Windows checkout
        // they wrap with CRLF. A phrase that spans a line break is still the
        // same phrase to a reader, so the assertion is about the words rather
        // than about where the author happened to break them.
        let raw = fs::read_to_string(root().join("docs/README.md")).unwrap();
        let index = raw.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(
            index.contains("(../standards/pages/README.md)"),
            "docs/README.md's `Looking for / It is at` table links the router, \
             so the tree is reachable by someone who does not know it exists"
        );
        assert!(
            index.contains("Four trees are read by the gate rather than only by people"),
            "this tree is one of them: the gate's mandatory `tests` step runs \
             this module, which reads every atom by name"
        );
        assert!(
            !index.contains("no gate step reads it yet"),
            "the repository index may not tell a reader the pages tree is \
             unread when `cargo xtask ci` reads it on every run"
        );
        // The inversion `router-precedence-and-announcement` scheduled. That
        // story wrote the marker as a dated claim with a named retirement
        // rather than a TODO, and this is the story it named: the bracket is
        // removed in the same commit that makes the sentence true, so no
        // provisional claim outlives the change that discharges it.
        assert!(
            !index.contains("PROVISIONAL"),
            "the marker is retired in the commit that makes the sentence \
             unconditionally true, not in a later tidy-up"
        );
        assert!(
            index.contains("the mandatory `cargo xtask lint-pages` step walks that directory"),
            "and what replaces it is the dedicated step, named, so a reader \
             learns which command to run rather than which module to open"
        );
        assert!(
            index.contains("fails by file and line"),
            "the fourth reader reads this directory too, and the index says so \
             where a reader of `docs/` will meet it"
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

        // The same rule the rules tree is held to, through the same function:
        // the router is a page in this tree and nothing here compiles either.
        let wrong = fence_tag_problems(ROUTER, &text);
        assert!(wrong.is_empty(), "{}", wrong.join("\n"));
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

    /// [`flat`], with each line's leading blockquote markers removed first.
    ///
    /// A quotation that wraps is still the same quotation, and in `CommonMark`
    /// every continuation line of a blockquote carries its own `>`. Stripping
    /// them is what lets a quoted sentence be compared with the literal it was
    /// copied from.
    fn unquoted(text: &str) -> String {
        let stripped: Vec<&str> = text
            .lines()
            .map(|line| line.trim_start().trim_start_matches('>'))
            .collect();
        flat(&stripped.join(" "))
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
            "Nothing counts folds",
            "the markers are textual and the verdict is not",
            "checks the rule's shape and never the pages it governs",
            "DT-7",
            "band 40",
            "not a `const`",
        ] {
            assert!(
                text.contains(limit),
                "the closing statement names all four limits; `{limit}` is missing"
            );
        }

        // The first limit is scoped to what it can honestly claim. The atom is
        // read on every `cargo xtask ci` — by this module's own tests — so a
        // bullet saying no gate step reads *the rule* is false, while "nothing
        // counts folds on a governed page" is exactly true.
        assert!(
            !text.contains("No gate step reads this rule"),
            "the fold *line* is unchecked; the atom stating it is not"
        );

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
    fn band_thirty_gives_the_clause_or_page_test() {
        let text = flat(&atom(BAND_30));
        assert!(
            text.contains(
                "could a conformant adapter written in another language violate this sentence"
            ),
            "band 30 hands the author a falsifiable test, not a preference"
        );
        assert!(
            text.to_lowercase().contains("never restate"),
            "the rule is cite-never-restate; without the second half a page \
             becomes a second specification"
        );
        assert!(
            text.contains("visible link text"),
            "the citation is the stable clause id as visible link text, so a \
             later reader can see they are being handed to the specification"
        );
        assert!(
            text.contains("never renumbered") && text.contains("spec/SPECIFICATION.md:280"),
            "clause ids are stable names, and the atom cites where that is said"
        );
    }

    #[test]
    fn band_thirty_states_its_blind_spots_before_its_first_rule() {
        let text = atom(BAND_30);
        let blind = text
            .find("clause_ids")
            .unwrap_or_else(|| panic!("{BAND_30} does not name HS-P0020's `clause_ids`"));
        let first_rule = text
            .find("## RP-30-1")
            .unwrap_or_else(|| panic!("{BAND_30} carries no `## RP-30-1` rule"));
        assert!(
            blind < first_rule,
            "the blind spots come first: a reader must not reach a rule before \
             learning that resolution is checked elsewhere and paraphrase by \
             nothing (RS-81-1)"
        );
        assert!(
            flat(&text).contains("checked by nothing mechanical"),
            "the sharper half — a page can cite a real id and restate it \
             underneath, and nothing sees that"
        );
    }

    #[test]
    fn band_forty_names_who_walks_and_what_they_may_consult() {
        let text = atom(BAND_40);
        let walk = text
            .lines()
            .position(|line| line.trim() == "### The walk")
            .unwrap_or_else(|| panic!("{BAND_40} carries no `### The walk` heading"));
        let one = text
            .lines()
            .position(|line| line.starts_with("## RP-40-1"))
            .unwrap_or_else(|| panic!("{BAND_40} carries no `## RP-40-1` rule"));
        let two = text
            .lines()
            .position(|line| line.starts_with("## RP-40-2"))
            .unwrap_or_else(|| panic!("{BAND_40} carries no `## RP-40-2` rule"));
        assert!(
            one < walk && walk < two,
            "`### The walk` is the anchor the router points at, and it sits \
             inside RP-40-1's body"
        );
        assert_eq!(
            text.matches("### The walk").count(),
            1,
            "one anchor, so the fragment resolves to one heading"
        );

        let flattened = flat(&text).to_lowercase();
        assert!(
            flattened.contains("not the author"),
            "the walk is performed by someone who did not write the page — the \
             whole point of DoD-8"
        );
        assert!(
            flattened.contains("git history"),
            "and it forbids repository archaeology, so the verdict does not \
             depend on what the author remembers"
        );
    }

    #[test]
    fn the_walk_is_answerable_steps_only() {
        let text = atom(BAND_40);
        let lines: Vec<&str> = text.lines().collect();
        let start = lines
            .iter()
            .position(|line| line.trim() == "### The walk")
            .unwrap_or_else(|| panic!("{BAND_40} carries no `### The walk` heading"));
        let end = lines
            .iter()
            .skip(start)
            .position(|line| line.starts_with("## RP-40-2"))
            .map_or(lines.len(), |offset| start + offset);
        let steps: Vec<&&str> = lines[start..end]
            .iter()
            .filter(|line| line.starts_with(|c: char| c.is_ascii_digit()) && line.contains(". "))
            .collect();
        assert!(
            steps.len() >= 3,
            "the walk is an ordered list a stranger executes; it holds {} steps",
            steps.len()
        );
        for step in steps {
            assert!(
                step.trim_end().ends_with('?'),
                "every step is a question with a yes/no answer; this one is not: {step}"
            );
        }
    }

    #[test]
    fn the_walk_closes_in_a_four_row_verdict_table() {
        let text = atom(BAND_40);
        let rows = table_rows(&text, "| Verdict");
        assert_eq!(
            rows.len(),
            4,
            "four verdicts, so a reviewer never has a soft pass available; the \
             table holds {rows:?}"
        );
        for verdict in [
            "`pass`",
            "`fail — two needs`",
            "`fail — need not answered`",
            "`indeterminate`",
        ] {
            assert!(
                rows.iter().any(|row| row.contains(verdict)),
                "the verdict table carries {verdict}"
            );
        }
        assert!(
            flat(&text).contains("defect in the page"),
            "`indeterminate` is recorded as a defect in the page, never in the \
             procedure"
        );
    }

    #[test]
    fn the_walk_records_an_empty_corpus_as_vacuous() {
        let text = flat(&atom(BAND_40)).to_lowercase();
        assert!(
            text.contains("vacuous"),
            "an empty governed set is recorded as vacuous, in those words, and \
             never as a pass — the decorative-green failure one medium over"
        );
        assert!(
            text.contains("never as a pass"),
            "the instruction says what the reviewer must not write down, not \
             merely what they may"
        );
    }

    #[test]
    fn the_walk_is_calibrated_against_a_two_need_fixture() {
        let fixture = atom(FIXTURE);
        let declarations = fixture
            .lines()
            .filter(|line| line.starts_with("> **Answers:**"))
            .count();
        assert_eq!(
            declarations, 2,
            "a procedure that has never returned `fail` is decorative; the \
             fixture carries two declarations so the walk can reach one"
        );
        let body = rule_body(&atom(BAND_40), "RP-40-1");
        assert!(
            body.contains(&format!("]({FIXTURE})")),
            "the worked example is linked from RP-40-1, not inlined: it is long, \
             it is genuinely an aside, and a link is the one opened-on-demand \
             mechanism this repository does not have to verify"
        );
        assert!(
            !TREE.contains(&FIXTURE),
            "the fixture is inert — outside the atom namespace, so it can be \
             permanently broken without ever making a gate red"
        );
    }

    #[test]
    fn band_forty_carries_the_paraphrase_spot_check() {
        let body = flat(&rule_body(&atom(BAND_40), "RP-40-2"));
        assert!(
            body.contains("sentence"),
            "the spot check is per normative sentence, not per page"
        );
        assert!(
            body.contains("restate") || body.contains("restatement"),
            "it asks whether a sentence's authority is a citation or a \
             restatement — the thing no parser reaches"
        );
        assert!(
            body.contains("standards/pages"),
            "the corpus it is run over is named, so the run is repeatable"
        );
    }

    #[test]
    fn every_markdown_link_in_the_tree_resolves() {
        let mut checked = 0_usize;
        for file in TREE.iter().copied().chain([ROUTER]) {
            // The router is addressed from the workspace root and the atoms
            // from `RULE_DIR`; both resolve against `RULE_DIR` because that is
            // the directory every link in this tree is relative to.
            let text = if file == ROUTER { router() } else { atom(file) };
            let mut fenced = false;
            for line in text.lines() {
                if line.starts_with("```") {
                    fenced = !fenced;
                    continue;
                }
                if fenced {
                    // A fence shows a *specimen* page; its links are examples
                    // and are not this tree's to resolve.
                    continue;
                }
                for target in markdown_link_targets(line) {
                    let is_page = Path::new(&target)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"));
                    if !is_page {
                        continue;
                    }
                    checked += 1;
                    assert!(
                        root().join(RULE_DIR).join(&target).exists(),
                        "{file} links `{target}`, which resolves to no file — a \
                         dangling link is the one state this tree may not ship in"
                    );
                }
            }
        }
        assert!(checked > 0, "the link check found no link to check");
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
            let wrong = fence_tag_problems(file, &atom(file));
            assert!(wrong.is_empty(), "{}", wrong.join("\n"));
        }
    }

    /// The three wrong fences, held as specimens rather than as tree edits.
    ///
    /// This is CLAUDE.md's decorative-rule corollary paid in the currency it
    /// asks for: *name a plausible wrong implementation the rule rejects, and
    /// write it down where the suite can run it.* Without these three strings
    /// the untagged half of AC-014 is verified only by a corpus that happens to
    /// be clean, and the day the corpus stops being clean is the only day the
    /// check is exercised.
    ///
    /// Each specimen also names the implementation it forbids. The bare fence
    /// is what a per-line scan that `continue`s on an empty info string waves
    /// through, and what a parity count over bare lines can never see, because
    /// a well-formed untagged fence contributes exactly two of them and
    /// `2 % 2 == 0`. The unterminated opener is what any per-line scan misses
    /// entirely.
    #[test]
    fn the_fence_check_rejects_the_three_wrong_fences() {
        let untagged = fence_tag_problems("specimen.md", "# T\n\n```\nbare\n```\n");
        assert_eq!(
            untagged.len(),
            1,
            "one untagged fence is one problem, reported once: {untagged:?}"
        );
        assert!(
            untagged[0].starts_with("specimen.md:3 —") && untagged[0].contains("untagged"),
            "the message must open on the opener's own `file:line` and say \
             which case failed: {untagged:?}"
        );

        let rust = fence_tag_problems("specimen.md", "# T\n\n```rust\nfn main() {}\n```\n");
        assert_eq!(rust.len(), 1, "one `rust` fence is one problem: {rust:?}");
        assert!(
            rust[0].starts_with("specimen.md:3 —") && rust[0].contains("`rust`"),
            "the message must name the tag it rejected: {rust:?}"
        );

        let unclosed = fence_tag_problems("specimen.md", "# T\n\n```text\nno closer\n");
        assert_eq!(
            unclosed.len(),
            1,
            "an opener with no closer is one problem: {unclosed:?}"
        );
        assert!(
            unclosed[0].starts_with("specimen.md:3 —") && unclosed[0].contains("never closed"),
            "an unterminated fence makes every fence after it read inside-out, \
             so it is named rather than silently re-phased: {unclosed:?}"
        );
    }

    /// The right fence, and the reason the walk is a walk.
    ///
    /// A closing fence marker is spelled exactly like an untagged opener, so
    /// any predicate applied line by line judges closers as openers. The router
    /// half of this rule did precisely that until this change: it asserted
    /// `info == "text" || info == "markdown"` on every line starting with a
    /// fence marker, and passed only because `standards/pages/README.md` had no
    /// fence at all — the first legitimately tagged fence added to the router
    /// would have failed on its own closing line, blaming an untagged fence.
    /// That is the specimen below, and it must stay silent.
    #[test]
    fn the_fence_check_never_judges_a_closing_marker() {
        for (info, text) in [
            ("text", "# T\n\n```text\nbody\n```\n"),
            (
                "markdown",
                "# T\n\n```markdown\n> **Answers:** `how-to` — ?\n```\n",
            ),
        ] {
            let clean = fence_tag_problems("specimen.md", text);
            assert!(
                clean.is_empty(),
                "a `{info}` fence and its closer are both well-formed: {clean:?}"
            );
        }

        let two = fence_tag_problems("specimen.md", "```text\na\n```\n\n```\nb\n```\n");
        assert_eq!(
            two.len(),
            1,
            "the second block's opener is the only problem; the first block's \
             closer is not an opener and the second block's closer is not \
             either: {two:?}"
        );
        assert!(
            two[0].starts_with("specimen.md:5 —"),
            "and it is reported at the untagged opener's line, not at a closer's: {two:?}"
        );
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

    // ======================================================================
    // page-need-checker-mounted-in-the-gate
    // ======================================================================

    /// A fabricated workspace root under `std::env::temp_dir()`.
    ///
    /// Never the workspace's own trees, and never `tempfile`: `xtask/Cargo.toml`
    /// carries only `anyhow` and this story adds no dependency. The name carries
    /// the caller's label and a nanosecond stamp, so the default parallel
    /// harness cannot make two tests share a directory.
    fn fabricated_root(label: &str) -> PathBuf {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|since| since.as_nanos())
            .unwrap_or_default();
        let path = std::env::temp_dir().join(format!("hs-lint-pages-{label}-{stamp}"));
        fs::create_dir_all(&path).unwrap();
        path
    }

    /// Writes one file under a fabricated root, creating its parents.
    fn write_at(root: &Path, rel: &str, text: &str) {
        let path = root.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    /// One synthetic page, parsed by the one parser.
    fn synthetic_page(path: &str, text: &str) -> Page {
        let dir = path
            .rsplit_once('/')
            .map_or(String::new(), |(dir, _)| dir.to_owned());
        Page {
            path: path.to_owned(),
            dir,
            declarations: declarations(text),
        }
    }

    /// One synthetic rule atom, parsed exactly as [`rule_atoms`] parses a real one.
    fn synthetic_atom(file: &str, text: &str) -> Atom {
        Atom {
            band: file[..2].to_owned(),
            load_when: load_when(text),
            rules: rules(text),
            file: file.to_owned(),
            text: text.to_owned(),
        }
    }

    /// The problems, rendered as the lines a reader would meet.
    fn rendered(problems: &[Problem]) -> Vec<String> {
        problems.iter().map(Problem::render).collect()
    }

    /// A well-formed one-rule atom, as a base for the shape tests to perturb.
    fn well_formed_atom(band: &str) -> String {
        format!(
            "# {band} — Title\n\n> **Load when:** doing a thing\n\n> **See also:** 00 (a band)\n\
             \n---\n\nProse.\n\n## RP-{band}-1. Do the thing.\n\n**Why.** Because.\n\n\
             **Do**\n\n```text\nright\n```\n\n**Not**\n\n```text\nwrong\n```\n\n\
             **Rejects.** A page that could ship and should not.\n\n\
             **Evidence.** `xtask/src/lint_pages.rs:1`\n"
        )
    }

    // ---------- AC-002: both trees, both guards ----------

    #[test]
    fn a_missing_rules_tree_fails_and_names_the_path_it_expected() {
        let root = fabricated_root("rules-missing");
        let err = rule_atoms(&root).unwrap_err();
        let chain = format!("{err:#}");
        assert!(
            chain.contains(RULE_DIR),
            "the error names the tree the gate expected, in `lint_constitution.rs:212`'s \
             shape; it reads {chain}"
        );
        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn an_empty_rules_tree_is_vacuous_rather_than_green() {
        let root = fabricated_root("rules-empty");
        fs::create_dir_all(root.join(RULE_DIR)).unwrap();
        let atoms = rule_atoms(&root).unwrap();
        let err = guard_rules_not_vacuous(&atoms).unwrap_err();
        let message = format!("{err}");
        assert!(
            message.contains(RULE_DIR) && message.contains("vacuous"),
            "an emptied tree bails in `lint_constitution.rs:176`'s spelling and never \
             prints `0 rules, all consistent`; it reads {message}"
        );
        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_missing_pages_tree_fails_and_names_the_path_it_expected() {
        let root = fabricated_root("pages-missing");
        let err = governed_pages(&root).unwrap_err();
        let chain = format!("{err:#}");
        assert!(
            chain.contains(lint_narrative::TREE),
            "guarding its own tree and trusting the pages tree is the named wrong \
             implementation; the error reads {chain}"
        );
        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn an_empty_pages_tree_is_vacuous_rather_than_green() {
        let root = fabricated_root("pages-empty");
        fs::create_dir_all(root.join(lint_narrative::TREE)).unwrap();
        let pages = governed_pages(&root).unwrap();
        let err = guard_pages_not_vacuous(&pages).unwrap_err();
        let message = format!("{err}");
        assert!(
            message.contains(lint_narrative::TREE) && message.contains("vacuous"),
            "the same guard on the other tree; it reads {message}"
        );
        fs::remove_dir_all(&root).ok();
    }

    // ---------- AC-003: one constant for the pages tree ----------

    #[test]
    fn the_pages_root_resolves_from_the_narrative_trees_own_constant() {
        let root = fabricated_root("pages-root");
        write_at(
            &root,
            &format!("{}/only.md", lint_narrative::TREE),
            "# T\n\n> **Answers:** `how-to` — How do I do the thing?\n\nProse.\n",
        );
        let pages = governed_pages(&root).unwrap();
        assert_eq!(
            pages
                .iter()
                .map(|page| page.path.clone())
                .collect::<Vec<_>>(),
            [format!("{}/only.md", lint_narrative::TREE)],
            "the checker addresses the pages tree through `lint_narrative::TREE` and \
             declares no `PAGE_DIR` of its own"
        );
        assert!(
            !THIS_FILE.contains(&format!("const PAGE_{}", "DIR")),
            "a second constant naming the pages tree is the `three lists that must \
             agree` defect, foreclosed rather than documented"
        );
        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn the_index_is_not_a_governed_page() {
        let root = fabricated_root("pages-index");
        write_at(
            &root,
            &format!("{}/README.md", lint_narrative::TREE),
            "# Index\n\nRouting.\n",
        );
        let pages = governed_pages(&root).unwrap();
        assert!(
            pages.is_empty(),
            "the pinned tree's own checker says the index is not a page \
             (`xtask/src/lint_narrative.rs:232-241`, `:486-491`); it found {pages:?}"
        );
        fs::remove_dir_all(&root).ok();
    }

    // ---------- AC-004: the declaration parser ----------

    #[test]
    fn one_declaration_parses_to_its_token_and_its_line() {
        let found = declarations(
            "# Append conditions\n\n> **Answers:** `explanation` — Why re-read?\n\nProse.\n",
        );
        assert_eq!(found.len(), 1, "one declaration");
        assert_eq!(
            found[0].line, 3,
            "the 1-based line of the declaration itself"
        );
        assert_eq!(found[0].token.as_deref(), Some("explanation"));
        assert!(found[0].malformed.is_none());
    }

    #[test]
    fn a_page_with_no_declaration_is_one_problem_naming_band_zero() {
        let page = synthetic_page(
            "docs/getting-started.md",
            "# Getting started\n\nProse about explanation and how-to.\n",
        );
        let mut problems = Vec::new();
        check_declarations(&[page], &mut problems);
        let lines = rendered(&problems);
        assert_eq!(lines.len(), 1, "one problem per page: {lines:?}");
        assert_eq!(
            lines[0],
            "docs/getting-started.md — no `> **Answers:**` line; see \
             standards/pages/00-one-need.md",
            "the file-level form, with no faked line 0, and the atom to read"
        );
    }

    #[test]
    fn a_second_declaration_is_reported_at_the_second_ones_line() {
        let page = synthetic_page(
            "docs/two.md",
            "# Two\n\n> **Answers:** `explanation` — Why?\n> **Answers:** `how-to` — How?\n",
        );
        let mut problems = Vec::new();
        check_declarations(&[page], &mut problems);
        let lines = rendered(&problems);
        assert_eq!(
            lines,
            ["docs/two.md:4 — declares `explanation` and `how-to`; a page answers one need"],
            "at the line of the offending second declaration, naming both tokens"
        );
    }

    #[test]
    fn prose_that_mentions_a_need_word_is_never_a_declaration() {
        let page = synthetic_page(
            "docs/one.md",
            "# One\n\n> **Answers:** `explanation` — Why?\n\nThis page is an explanation, \
             not a how-to, and it is certainly not a `tutorial`.\n\n```text\n\
             > **Answers:** `how-to` — How do I write one?\n```\n",
        );
        let mut problems = Vec::new();
        check_declarations(&[page], &mut problems);
        assert!(
            problems.is_empty(),
            "only a blockquote line opening `**Answers:**` outside a fence is a \
             declaration: {:?}",
            rendered(&problems)
        );
    }

    #[test]
    fn a_malformed_declaration_is_its_own_problem_not_a_missing_one() {
        for (text, expect) in [
            (
                "# T\n\n> **Answers:** how-to — How?\n",
                "the token is not in backticks",
            ),
            (
                "# T\n\n> **Answers:** `how-to` How?\n",
                "no ` — ` between the token and the question",
            ),
            (
                "# T\n\n> **Answers:** `how-to` — How.\n",
                "the question does not end in `?`",
            ),
        ] {
            let page = synthetic_page("docs/near-miss.md", text);
            let mut problems = Vec::new();
            check_declarations(&[page], &mut problems);
            let lines = rendered(&problems);
            assert_eq!(lines.len(), 1, "one problem, not two: {lines:?}");
            assert!(
                lines[0].starts_with("docs/near-miss.md:3 — ") && lines[0].contains(expect),
                "a near miss is reported at its own line, naming the part of the \
                 grammar that failed, never degraded to `no declaration`: {lines:?}"
            );
        }
    }

    /// The three wrong pages, held as `&str` specimens rather than committed files.
    ///
    /// `CLAUDE.md`'s decorative-rule corollary paid in the currency it asks for:
    /// name a plausible wrong implementation the rule rejects and write it down
    /// where the suite can run it. None of the three is a file, which is what
    /// makes the rejection permanent rather than a one-off observation.
    #[test]
    fn the_three_wrong_pages_are_each_rejected_at_the_right_line() {
        let two = synthetic_page(
            "docs/append-conditions.md",
            "# Appending under a condition\n\n> **Answers:** `explanation` — Why re-read?\n\
             > **Answers:** `how-to` — How do I append under a condition?\n",
        );
        let none = synthetic_page("docs/getting-started.md", "# Getting started\n\nProse.\n");
        let unenumerated = synthetic_page(
            "docs/store-api.md",
            "# The store API\n\n> **Answers:** `reference` — What are the methods?\n",
        );

        let mut problems = Vec::new();
        check_declarations(&[two, none, unenumerated], &mut problems);
        problems.sort();
        let lines = rendered(&problems);
        assert_eq!(
            lines.len(),
            3,
            "three wrong pages, three problems: {lines:?}"
        );
        assert!(
            lines[0]
                .starts_with("docs/append-conditions.md:4 — declares `explanation` and `how-to`"),
            "{lines:?}"
        );
        assert!(
            lines[1].starts_with("docs/getting-started.md — no `> **Answers:**` line"),
            "{lines:?}"
        );
        assert!(
            lines[2].starts_with("docs/store-api.md:3 — `reference` is not a need")
                && lines[2].contains("explanation")
                && lines[2].contains("10-the-need-set.md"),
            "the offending token, the enumerated set, and where the set is argued: {lines:?}"
        );
    }

    // ---------- AC-006: the orientation ceiling ----------

    #[test]
    fn two_orientation_pages_at_one_level_are_one_problem_naming_both() {
        let pages = [
            synthetic_page(
                "docs/index.md",
                "# Index\n\n> **Answers:** `orientation` — Where do I go?\n",
            ),
            synthetic_page(
                "docs/start.md",
                "# Start\n\n> **Answers:** `orientation` — Where do I start?\n",
            ),
        ];
        let mut problems = Vec::new();
        check_orientation_ceiling(&pages, &mut problems);
        let lines = rendered(&problems);
        assert_eq!(lines.len(), 1, "one problem for the directory: {lines:?}");
        assert!(
            lines[0].starts_with("docs — ")
                && lines[0].contains("docs/index.md:3")
                && lines[0].contains("docs/start.md:3")
                && lines[0].contains("RP-10-3"),
            "the directory, every offending `path:line`, and the rule: {lines:?}"
        );
    }

    #[test]
    fn one_orientation_page_per_directory_passes() {
        let pages = [
            synthetic_page(
                "docs/index.md",
                "# Index\n\n> **Answers:** `orientation` — Where do I go?\n",
            ),
            synthetic_page(
                "docs/guide/index.md",
                "# Guide\n\n> **Answers:** `orientation` — Where in the guide?\n",
            ),
        ];
        let mut problems = Vec::new();
        check_orientation_ceiling(&pages, &mut problems);
        assert!(
            problems.is_empty(),
            "the ceiling is per directory level, not per tree: {:?}",
            rendered(&problems)
        );
    }

    // ---------- AC-007: the router's generated index ----------

    /// A fabricated rules tree with a router whose region is `region_body`.
    fn router_root(label: &str, region_body: &str) -> PathBuf {
        let root = fabricated_root(label);
        write_at(
            &root,
            &format!("{RULE_DIR}/00-one-need.md"),
            &well_formed_atom("00"),
        );
        write_at(
            &root,
            &format!("{RULE_DIR}/10-the-need-set.md"),
            &well_formed_atom("10"),
        );
        write_at(
            &root,
            ROUTER,
            &format!(
                "# Page standards\n\n## Index\n\n<!-- BEGIN GENERATED -->\n{region_body}\n\
                 <!-- END GENERATED -->\n"
            ),
        );
        root
    }

    #[test]
    fn a_router_index_disagreeing_by_one_atom_names_the_repair_in_the_line() {
        let root = router_root(
            "router-stale",
            "| Atom | Load when | Rules |\n|---|---|---|\n\
             | [`00-one-need.md`](00-one-need.md) | doing a thing | RP-00-1 |",
        );
        let atoms = rule_atoms(&root).unwrap();
        let mut problems = Vec::new();
        check_router(&root, &atoms, Mode::Check, &mut problems).unwrap();
        let lines = rendered(&problems);
        assert_eq!(lines.len(), 1, "{lines:?}");
        assert!(
            lines[0].starts_with(&format!("{ROUTER} — "))
                && lines[0].contains("cargo xtask lint-pages --write"),
            "the repair sits inside the problem line, never in a footer: {lines:?}"
        );
        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn write_mode_rewrites_only_the_region_and_reaches_equality() {
        let root = router_root(
            "router-write",
            "| Atom | Load when | Rules |\n|---|---|---|",
        );
        let atoms = rule_atoms(&root).unwrap();
        let mut problems = Vec::new();
        check_router(&root, &atoms, Mode::Write, &mut problems).unwrap();
        assert!(problems.is_empty(), "{:?}", rendered(&problems));

        let rewritten = fs::read_to_string(root.join(ROUTER)).unwrap();
        assert!(
            rewritten.starts_with("# Page standards\n\n## Index\n"),
            "everything outside the markers is untouched: {rewritten}"
        );
        let (start, end) = region(&rewritten).unwrap();
        let lines: Vec<&str> = rewritten.lines().collect();
        assert_eq!(lines[start..end].join("\n"), generated_index(&atoms));

        let mut again = Vec::new();
        check_router(&root, &atoms, Mode::Check, &mut again).unwrap();
        assert!(
            again.is_empty(),
            "a second `--write` is idempotent: {:?}",
            rendered(&again)
        );
        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_dangling_router_link_is_a_problem_at_its_own_line() {
        let root = router_root(
            "router-dangling",
            "| Atom | Load when | Rules |\n|---|---|---|",
        );
        let atoms = rule_atoms(&root).unwrap();
        let router = fs::read_to_string(root.join(ROUTER)).unwrap();
        write_at(
            &root,
            ROUTER,
            &format!("{router}\nSee [`99-gone.md`](99-gone.md).\n"),
        );
        let mut problems = Vec::new();
        check_router(&root, &atoms, Mode::Check, &mut problems).unwrap();
        let lines = rendered(&problems);
        assert!(
            lines
                .iter()
                .any(|line| line.contains("99-gone.md") && line.contains("resolves to no file")),
            "a router may not ship a dangling link: {lines:?}"
        );
        fs::remove_dir_all(&root).ok();
    }

    // ---------- AC-008: NEEDS against band 10 ----------

    /// Band 10, as a synthetic atom whose token table is `rows`.
    fn band_ten_with(rows: &str) -> Atom {
        synthetic_atom(
            NEED_SET_ATOM,
            &format!(
                "# 10 — The need set\n\n> **Load when:** choosing\n\n> **See also:** 00\n\n\
                 ---\n\n| Token | The page's job | Success for the reader |\n\
                 | --- | --- | --- |\n{rows}\n"
            ),
        )
    }

    /// Every member's real row, so a test perturbs exactly one thing.
    fn band_ten_rows() -> String {
        NEEDS
            .iter()
            .map(|member| format!("| `{}` | {} | prose |", member.token, member.job))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn a_needs_member_missing_from_band_ten_names_which_token_moved() {
        let rows = band_ten_rows()
            .lines()
            .filter(|row| !row.contains("`how-to`"))
            .collect::<Vec<_>>()
            .join("\n");
        let mut problems = Vec::new();
        check_need_set(&[band_ten_with(&rows)], &mut problems);
        let lines = rendered(&problems);
        assert_eq!(lines.len(), 1, "{lines:?}");
        assert!(
            lines[0].contains("`how-to`")
                && lines[0].contains("xtask/src/lint_pages.rs")
                && lines[0].starts_with(&format!("{RULE_DIR}/{NEED_SET_ATOM}")),
            "the failure says which one moved and cites both paths: {lines:?}"
        );
    }

    #[test]
    fn a_need_shaped_token_in_band_ten_that_is_not_a_member_is_named() {
        let rows = format!(
            "{}\n| `reference` | be a second spec | prose |",
            band_ten_rows()
        );
        let mut problems = Vec::new();
        check_need_set(&[band_ten_with(&rows)], &mut problems);
        let lines = rendered(&problems);
        assert_eq!(lines.len(), 1, "{lines:?}");
        assert!(
            lines[0].contains("`reference`") && lines[0].contains("not `--write`-repairable"),
            "the exclusion half, and the sentence saying why no generator repairs it: \
             {lines:?}"
        );
    }

    #[test]
    fn band_tens_job_column_cannot_drift_from_the_const() {
        let rows = band_ten_rows().replace(
            "carry a newcomer through one working thing, staged",
            "something else entirely",
        );
        let mut problems = Vec::new();
        check_need_set(&[band_ten_with(&rows)], &mut problems);
        let lines = rendered(&problems);
        assert!(
            lines.iter().any(|line| line.contains("`tutorial`")),
            "`Need::job` has a machine that reads it, which is what band 10 claims \
             about both of its fields: {lines:?}"
        );
    }

    // ---------- AC-009: the rule atom's own shape ----------

    #[test]
    fn a_well_formed_atom_reports_nothing() {
        let mut problems = Vec::new();
        let atom = synthetic_atom("00-one-need.md", &well_formed_atom("00"));
        check_atom_shape(&atom, &mut problems);
        check_atom_fences(&atom, &mut problems);
        assert!(problems.is_empty(), "{:?}", rendered(&problems));
    }

    #[test]
    fn an_atom_missing_a_section_is_one_problem_at_the_rules_line() {
        let text = well_formed_atom("00").replace(
            "**Rejects.** A page that could ship and should not.\n\n",
            "",
        );
        let mut problems = Vec::new();
        check_atom_shape(&synthetic_atom("00-one-need.md", &text), &mut problems);
        let lines = rendered(&problems);
        assert!(
            lines
                .iter()
                .any(|line| line.contains("**Rejects.**") && line.contains("RP-00-1")),
            "a rule that names no wrong page is decorative: {lines:?}"
        );
    }

    #[test]
    fn an_atom_with_no_rule_at_all_is_a_problem() {
        let mut problems = Vec::new();
        let text = "# 00 — Title\n\n> **Load when:** x\n\n> **See also:** 10\n\n---\n\nProse.\n";
        check_atom_shape(&synthetic_atom("00-one-need.md", text), &mut problems);
        let lines = rendered(&problems);
        assert!(
            lines
                .iter()
                .any(|line| line.contains("carries no `## RP-` rule")),
            "{lines:?}"
        );
    }

    #[test]
    fn an_atom_over_the_rule_ceiling_is_a_problem_naming_the_ceiling() {
        let extra: Vec<String> = (2..=(MAX_RULES_PER_ATOM + 1))
            .map(|index| {
                format!(
                    "\n## RP-00-{index}. Another.\n\n**Why.** Because.\n\n**Do**\n\n\
                     ```text\na\n```\n\n**Not**\n\n```text\nb\n```\n\n\
                     **Rejects.** A page.\n\n**Evidence.** `x:1`\n"
                )
            })
            .collect();
        let text = well_formed_atom("00") + &extra.join("");
        let mut problems = Vec::new();
        check_atom_shape(&synthetic_atom("00-one-need.md", &text), &mut problems);
        let lines = rendered(&problems);
        assert!(
            lines.iter().any(
                |line| line.contains(&format!("{} rules", MAX_RULES_PER_ATOM + 1))
                    && line.contains(&MAX_RULES_PER_ATOM.to_string())
            ),
            "{lines:?}"
        );
    }

    #[test]
    fn an_atom_over_the_byte_ceiling_carries_the_measured_count() {
        let mut text = well_formed_atom("00");
        text.push_str(&"\nfiller filler filler\n".repeat(1_200));
        let bytes = text.len();
        assert!(
            bytes > MAX_ATOM_BYTES,
            "the specimen must exceed the ceiling"
        );
        let mut problems = Vec::new();
        check_atom_shape(&synthetic_atom("00-one-need.md", &text), &mut problems);
        let lines = rendered(&problems);
        assert!(
            lines.iter().any(|line| line.contains(&bytes.to_string())
                && line.contains(&MAX_ATOM_BYTES.to_string())),
            "the measured byte count is in the message, not just the ceiling: {lines:?}"
        );
    }

    #[test]
    fn text_and_markdown_fences_pass_while_rust_and_untagged_fail() {
        for (info, wrong) in [("text", false), ("markdown", false), ("rust", true)] {
            let text = format!(
                "# 00 — T\n\n> **Load when:** x\n\n> **See also:** 10\n\n---\n\n\
                 ## RP-00-1. Do.\n\n**Why.** B.\n\n**Do**\n\n```{info}\na\n```\n\n\
                 **Not**\n\n```text\nb\n```\n\n**Rejects.** A page.\n\n**Evidence.** `x:1`\n"
            );
            let mut problems = Vec::new();
            check_atom_fences(&synthetic_atom("00-one-need.md", &text), &mut problems);
            assert_eq!(
                !problems.is_empty(),
                wrong,
                "`{info}` fence: {:?}",
                rendered(&problems)
            );
            if wrong {
                assert!(
                    rendered(&problems)[0].contains("nothing in the workspace compiles"),
                    "the message says *why*: {:?}",
                    rendered(&problems)
                );
            }
        }

        let untagged = "# 00 — T\n\n> **Load when:** x\n\n> **See also:** 10\n\n---\n\n\
             ## RP-00-1. Do.\n\n**Why.** B.\n\n**Do**\n\n```\na\n```\n\n\
             **Not**\n\n```text\nb\n```\n\n**Rejects.** A page.\n\n**Evidence.** `x:1`\n";
        let mut problems = Vec::new();
        check_atom_fences(&synthetic_atom("00-one-need.md", untagged), &mut problems);
        assert!(
            rendered(&problems)
                .iter()
                .any(|line| line.contains("untagged")),
            "untagged is rejected too, so a future decision to register this tree \
             cannot be undermined retroactively: {:?}",
            rendered(&problems)
        );
    }

    // ---------- AC-010: the terminal surface ----------

    #[test]
    fn a_problem_line_is_path_line_dash_message_in_that_order() {
        assert_eq!(
            Problem::at("docs/a.md", 7, "what is wrong; what to do".to_owned()).render(),
            "docs/a.md:7 — what is wrong; what to do",
            "location primary, what-is-wrong secondary, the repair third and never removed"
        );
        assert_eq!(
            Problem::whole("docs/a.md", "what is wrong; what to do".to_owned()).render(),
            "docs/a.md — what is wrong; what to do",
            "the whole-file form carries no faked line 0"
        );
    }

    #[test]
    fn problems_sort_by_path_then_line_whatever_order_they_arrive_in() {
        let ordered = [
            Problem::whole("docs/a.md", "file-level".to_owned()),
            Problem::at("docs/a.md", 2, "second".to_owned()),
            Problem::at("docs/a.md", 10, "aaa".to_owned()),
            Problem::at("docs/a.md", 10, "bbb".to_owned()),
            Problem::at("docs/b.md", 1, "first".to_owned()),
        ];
        let expected = rendered(&ordered);

        for rotation in 0..expected.len() {
            let mut shuffled: Vec<Problem> = vec![
                Problem::at("docs/b.md", 1, "first".to_owned()),
                Problem::at("docs/a.md", 10, "bbb".to_owned()),
                Problem::whole("docs/a.md", "file-level".to_owned()),
                Problem::at("docs/a.md", 10, "aaa".to_owned()),
                Problem::at("docs/a.md", 2, "second".to_owned()),
            ];
            shuffled.rotate_left(rotation);
            shuffled.sort();
            assert_eq!(
                rendered(&shuffled),
                expected,
                "`read_dir` order must not reach the output, and a tie on path and \
                 line breaks on the message so a re-run never reshuffles the list"
            );
        }
    }

    /// The two facts this tree has settled, in the shape
    /// [`positive_publication_pin`] derives them in.
    fn settled_facts() -> PublicationFacts {
        PublicationFacts {
            first_release: Some(("0.2.0-alpha.1".to_owned(), "2026-08-16".to_owned())),
            changelog_keeps_anchor: true,
            manifest_withholds: false,
            adapter_status: Some("an adapter, and it has run the suite".to_owned()),
            adapter_mounts_suite: true,
        }
    }

    /// A rustdoc page that states the publication fact, wrapped and backticked
    /// the way a real one is.
    const TRUE_LIB: &str = "\
/// # Migrating from `factory =`\n\
///\n\
/// The keyword was `factory =` and took a store expression. There is no\n\
/// deprecated arm: `factory =` was introduced at `23fd446` and removed at\n\
/// `1c1a6b7`, before this crate was first published at `0.2.0-alpha.1` on\n\
/// 2026-08-16, so no published version ever accepted it.\n";

    /// A README that quotes the adapter's own status, in a blockquote whose
    /// first paragraph is about something else.
    const TRUE_README: &str = "\
> **Status: early, and the reason has moved.** Deliberately about something\n\
> else, so that the paragraph split is exercised rather than assumed.\n\
>\n\
> **`happenstance-sqlite` has run this suite** — its own front page says so in\n\
> those words, *an adapter, and it has run the suite*.\n";

    #[test]
    fn the_first_release_is_the_oldest_heading_not_the_newest() {
        const CHANGELOG_TEXT: &str = "\
## [Unreleased]\n\
\n\
## [0.3.0] — 2026-09-01\n\
\n\
## [0.2.0-alpha.1] — 2026-08-16\n";

        assert_eq!(
            first_release(CHANGELOG_TEXT),
            Some(("0.2.0-alpha.1".to_owned(), "2026-08-16".to_owned())),
            "keyed to the newest release, the pin would demand a rewrite of a \
             sentence about history at every release"
        );
        assert_eq!(
            first_release("## [Unreleased]\n"),
            None,
            "`[Unreleased]` is not a publication"
        );
    }

    #[test]
    fn mounting_the_suite_is_read_from_the_invocation_not_from_the_prose() {
        assert!(mounts_suite(&format!("{MOUNT}(SqliteFixture::new());\n")));
        assert!(
            !mounts_suite(&format!(
                "//! `{MOUNT}` is a list of examples somebody wrote.\n"
            )),
            "the blind spot `stale_publication_claims` documents is this \
             function's job to close: a target emptied to its own documentation \
             mounts nothing"
        );
    }

    #[test]
    fn the_publication_pin_passes_a_page_that_states_what_the_tree_settled() {
        assert_eq!(
            publication_pin_problems(&settled_facts(), TRUE_LIB, TRUE_README),
            Vec::<String>::new(),
            "the anchors are written as one sentence and matched through \
             wrapping, backticks and bold"
        );
    }

    /// The pin rejects a restatement of each falsehood by either route, and the
    /// blind spot it keeps is executed rather than promised (RS-81-1).
    ///
    /// The mutations are not the sentences
    /// [`crate::lints::stale_publication_claims`] holds: each says the same
    /// false thing in words that matcher does not carry, which is the attack
    /// that reopened C2-07. Both placements are asserted — *replacing* the true
    /// sentence and *standing beside* it — because the second is the cheaper
    /// edit and is the one a negative matcher cannot see at all.
    #[test]
    fn the_publication_pin_rejects_a_paraphrase_and_states_what_it_cannot_see() {
        const LIB_PARAPHRASE: &str = "\
/// # Migrating from `factory =`\n\
///\n\
/// There is no deprecated arm, because nothing here is published yet and this\n\
/// is the last release in which that is true.\n";
        const README_PARAPHRASE: &str = "\
> **Status: early.**\n\
>\n\
> Not one adapter outside this crate's own tests has run this suite.\n";

        let facts = settled_facts();

        assert!(
            !publication_pin_problems(&facts, LIB_PARAPHRASE, TRUE_README).is_empty(),
            "the falsehood replaced the true sentence and the pin stayed green"
        );
        assert!(
            !publication_pin_problems(&facts, TRUE_LIB, README_PARAPHRASE).is_empty(),
            "the falsehood replaced the true sentence and the pin stayed green"
        );

        let lib_beside = format!("{TRUE_LIB}\n{LIB_PARAPHRASE}");
        let readme_beside = format!("{TRUE_README}\n{README_PARAPHRASE}");
        assert!(
            !publication_pin_problems(&facts, &lib_beside, TRUE_README).is_empty(),
            "the falsehood was added beside the true statement and the pin \
             stayed green"
        );
        assert!(
            !publication_pin_problems(&facts, TRUE_LIB, &readme_beside).is_empty(),
            "the falsehood was added beside the true statement and the pin \
             stayed green"
        );

        // The documented blind spot, executed. Neither sentence carries the
        // token it is counted by, and both are false.
        let unseen_lib = format!(
            "{TRUE_LIB}\n/// There is no deprecated arm, because this crate has never reached a \
             registry.\n"
        );
        let unseen_readme =
            format!("{TRUE_README}\n> No adapter anywhere has yet cleared this bar.\n");
        assert_eq!(
            publication_pin_problems(&facts, &unseen_lib, &unseen_readme),
            Vec::<String>::new(),
            "a falsehood written without the counted token passes, and that \
             limit is documented on `publication_pin_problems` rather than \
             left for a reader to discover"
        );
    }

    /// When the tree moves under the pin, the failure names the artefact that
    /// moved rather than blaming the page (RS-81-5).
    #[test]
    fn the_pin_names_which_artefact_moved() {
        let unpublished = PublicationFacts {
            first_release: None,
            ..settled_facts()
        };
        let problems = publication_pin_problems(&unpublished, TRUE_LIB, TRUE_README);
        assert!(
            problems
                .iter()
                .any(|p| p.contains("the pin that is now stale")),
            "with nothing published the page's claim would be the true one; \
             the pin is what must move: {problems:?}"
        );

        let withheld = PublicationFacts {
            manifest_withholds: true,
            ..settled_facts()
        };
        let problems = publication_pin_problems(&withheld, TRUE_LIB, TRUE_README);
        assert!(
            problems.iter().any(|p| p.contains(TESTKIT_MANIFEST)),
            "a release recorded and a `publish = false` in the manifest are two \
             artefacts disagreeing, and the message says so: {problems:?}"
        );

        let unmounted = PublicationFacts {
            adapter_mounts_suite: false,
            ..settled_facts()
        };
        let problems = publication_pin_problems(&unmounted, TRUE_LIB, TRUE_README);
        assert!(
            problems
                .iter()
                .any(|p| p.contains("Move the pin, not the page")),
            "an adapter that stopped mounting the suite makes the README's \
             positive claim the false one: {problems:?}"
        );
    }
}
