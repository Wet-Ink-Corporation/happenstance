//! The knowledge-base lint: an accepted decision's body is never edited.
//!
//! `.kb/decisions/README.md` states the rule — supersede, never edit — and until
//! 2026-09-28 `redkiln validate --kb` enforced it, pre-commit only. Redkiln is
//! retired in this repository until `redkiln-rs`, which left the rule as prose in
//! a README again, enforced by whoever remembered it. The owner's answer to
//! `wi-38373d` is this step: every atom under `.kb/decisions/` that reads
//! `status: accepted` **at the merge base** must still exist, and its body —
//! everything after the closing `---` — must be byte-identical, once line endings
//! are normalised, in the working tree.
//!
//! # Why the frontmatter is free
//!
//! Superseding an accepted decision is a *frontmatter* edit to it: `status:
//! superseded` and `superseded_by: <new id>`. That is the one edit an accepted
//! atom is allowed, so a whole-file diff would refuse the very operation the rule
//! exists to channel corrections into. [`tests::a_whole_file_diff_refuses_a_legitimate_supersession`]
//! holds that line.
//!
//! # Why the status is read at the base
//!
//! Reading it at `HEAD` would let one commit flip `accepted` to `draft` and edit
//! the body in the same breath, and the check would see a draft and look away.
//! What was accepted when this change began is what this change may not reword.
//!
//! # Where the base comes from, and why it never skips
//!
//! In order: `HS_KB_BASE` when it is set and not all zeros (CI sets it to the
//! pull request's base commit, or to the previous tip on a push); otherwise the
//! merge base of `HEAD` with `main`, then with `origin/main`. When that merge base
//! is `HEAD` itself — a run on `main` — the step compares against `HEAD^`, and it
//! prints the base it used either way. A base that cannot be resolved, or a
//! shallow clone with no base named, is an **error**: a lint that reports success
//! when it had nothing to compare against is the failure this repository names in
//! every other gate step. `ci.yml`'s `gate` job checks out at `fetch-depth: 0` for
//! this step (`wi-80cba0`).
//!
//! # What this does not verify
//!
//! **Frontmatter shape beyond what the check needs** (`wi-5f78c4`). The delimiters
//! must parse and a `status:` key must be present, or the step fails; the status
//! *vocabulary* and the key set are not checked, because
//! `adr-status-vocabulary-exceeds-the-schema` is an open question and a lint is no
//! place to settle it.
//!
//! **Atoms outside `.kb/decisions/`.** Open questions, concepts and playbooks
//! also carry `status: accepted`, and they are edited in place by design — an
//! open question is closed by editing it. Immutability belongs to decisions.
//!
//! **Repairs, bar one kind** (`wi-5fce24`). `kb-governance-referent-not-reasoning-001`
//! lets a drifted `path:line` citation inside an accepted decision be repointed in
//! place, and `citation_ranges_resolve` is what forces that repair, so refusing it
//! would set two gate steps against each other. That one repair is recognised
//! mechanically: a body whose every changed line differs from its base line only in
//! the numbers after a `:` passes. Every other change fails, including the rename
//! the same governance atom also permits — telling a meaning-preserving rename from
//! an amendment is a reading, not a diff, and a rename is rare enough to need its
//! own decision when it comes.
//!
//! **Committed history between the base and `HEAD`.** It compares two trees, not
//! every commit in between, so an edit made and reverted inside one branch
//! passes, which is the right answer.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::spec_trace::workspace_root;

/// The directory whose accepted atoms are immutable.
const DECISIONS: &str = ".kb/decisions";

/// The one file in that directory that is not an atom.
const LAYER_README: &str = "README.md";

/// The environment variable CI names the base through.
const BASE_VAR: &str = "HS_KB_BASE";

/// This step, as `REQUIRED` holds it. The whole row, for the reason
/// `lint_workflows::STEP` gives: a multi-line row inserted into `REQUIRED` moves
/// the constitution's anchored citations into `main.rs`, and one line does not.
pub(crate) const STEP: crate::Step = crate::Step {
    name: "no accepted decision's body has changed",
    program: "cargo",
    args: &["run", "--locked", "--quiet", "-p", "xtask", "--", "lint-kb"],
    env: &[],
    probe: None,
};

/// Check every accepted decision atom against the base this run resolves.
///
/// # Errors
///
/// When the base cannot be resolved, when the clone is shallow and no base is
/// named, when git fails, when no accepted decision exists at the base, or when
/// any accepted body has changed or its atom has gone.
pub(crate) fn run() -> Result<()> {
    let root = workspace_root()?;
    let base = resolve_base(&root)?;
    check_against(&root, &base)
}

/// The same check, against a base the caller has already chosen.
///
/// `cargo xtask affected --base <ref>` calls this with the merge base it
/// computed, so the story grain and the gate compare against the same commit.
///
/// # Errors
///
/// As [`run`], less base resolution.
pub(crate) fn run_against(base: &str) -> Result<()> {
    let root = workspace_root()?;
    let commit = git(&root, &["merge-base", base, "HEAD"])
        .with_context(|| format!("lint-kb: `{base}` does not resolve to a commit"))?;
    check_against(&root, commit.trim())
}

fn check_against(root: &Path, base: &str) -> Result<()> {
    println!("  comparing {DECISIONS}/ against {base}");

    let listing = git(
        root,
        &["ls-tree", "-r", "--name-only", base, "--", DECISIONS],
    )?;
    let mut at_base = BTreeMap::new();
    for path in listing.lines().map(str::trim).filter(|p| !p.is_empty()) {
        let is_markdown = Path::new(path)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"));
        if !is_markdown || path.ends_with(&format!("/{LAYER_README}")) {
            continue;
        }
        let text = git(root, &["show", &format!("{base}:{path}")])?;
        at_base.insert(path.to_owned(), text);
    }

    let mut at_head = BTreeMap::new();
    for path in at_base.keys() {
        at_head.insert(
            path.clone(),
            std::fs::read(root.join(path))
                .ok()
                .map(|bytes| String::from_utf8_lossy(&bytes).into_owned()),
        );
    }

    let (accepted, problems) = changed_bodies(&at_base, &at_head);
    if accepted == 0 {
        bail!(
            "lint-kb: no atom under {DECISIONS}/ reads `status: accepted` at {base}. Either the \
             layer has moved and this check is pointed at nothing, or the base is wrong; both \
             are failures rather than a step with no work to do."
        );
    }
    if !problems.is_empty() {
        bail!(
            "lint-kb: {} accepted decision(s) changed against {base}:\n  {}\n\n\
             An accepted decision is superseded, never edited: write a new atom carrying \
             `supersedes:`, and flip only this one's frontmatter to `status: superseded` and \
             `superseded_by:`. See {DECISIONS}/{LAYER_README}.",
            problems.len(),
            problems.join("\n  ")
        );
    }
    println!("  {accepted} accepted decision(s), every body unchanged");
    Ok(())
}

/// Resolve the base commit, or fail saying why there is none.
fn resolve_base(root: &Path) -> Result<String> {
    let named = std::env::var(BASE_VAR).ok();
    let shallow = git(root, &["rev-parse", "--is-shallow-repository"])?.trim() == "true";
    let head = git(root, &["rev-parse", "HEAD"])?.trim().to_owned();
    let merge_base = ["main", "origin/main"]
        .iter()
        .find_map(|branch| git(root, &["merge-base", branch, "HEAD"]).ok())
        .map(|sha| sha.trim().to_owned());
    let parent = git(root, &["rev-parse", "--verify", "--quiet", "HEAD^"])
        .ok()
        .map(|sha| sha.trim().to_owned());

    let choice = pick_base(
        named.as_deref(),
        shallow,
        merge_base.as_deref(),
        &head,
        parent.as_deref(),
    )?;
    let commit = git(
        root,
        &["rev-parse", "--verify", &format!("{choice}^{{commit}}")],
    )
    .with_context(|| {
        format!(
            "lint-kb: the base `{choice}` is not a commit in this clone. In CI the `gate` job \
                 must check out at `fetch-depth: 0`; locally, fetch it"
        )
    })?;
    Ok(commit.trim().to_owned())
}

/// Which commit to compare against, given what the clone can tell us.
///
/// Pure, so that the refusal to skip is a unit-tested property rather than a
/// hope about a CI runner.
fn pick_base(
    named: Option<&str>,
    shallow: bool,
    merge_base: Option<&str>,
    head: &str,
    parent: Option<&str>,
) -> Result<String> {
    if let Some(named) = named
        .map(str::trim)
        .filter(|n| !n.is_empty() && !n.chars().all(|c| c == '0'))
    {
        return Ok(named.to_owned());
    }
    if shallow {
        bail!(
            "lint-kb: this clone is shallow and {BASE_VAR} names no base, so the merge base is not \
             here to compare against. Check out with `fetch-depth: 0`, or set {BASE_VAR}"
        );
    }
    let Some(merge_base) = merge_base else {
        bail!(
            "lint-kb: HEAD shares no merge base with `main` or `origin/main`, and {BASE_VAR} is \
             not set; there is nothing to compare accepted decisions against"
        );
    };
    if merge_base != head {
        return Ok(merge_base.to_owned());
    }
    parent.map(str::to_owned).with_context(|| {
        format!(
            "lint-kb: HEAD is its own merge base and has no parent, and {BASE_VAR} is not set; \
             there is nothing to compare accepted decisions against"
        )
    })
}

/// How many atoms were accepted at the base, and what changed about them.
///
/// `at_head` maps each base path to its working-tree text, or `None` when the
/// file is gone.
fn changed_bodies(
    at_base: &BTreeMap<String, String>,
    at_head: &BTreeMap<String, Option<String>>,
) -> (usize, Vec<String>) {
    let mut accepted = 0;
    let mut problems = Vec::new();

    for (path, base_text) in at_base {
        let (base_front, base_body) = match split(base_text) {
            Ok(parts) => parts,
            Err(why) => {
                problems.push(format!("{path} at the base: {why}"));
                continue;
            }
        };
        match status(&base_front) {
            Some("accepted") => accepted += 1,
            Some(_) => continue,
            None => {
                problems.push(format!(
                    "{path} at the base: its frontmatter has no `status:` key"
                ));
                continue;
            }
        }

        let Some(Some(head_text)) = at_head.get(path) else {
            problems.push(format!(
                "{path}: accepted at the base, and deleted or moved since"
            ));
            continue;
        };
        match split(head_text) {
            Ok((_, head_body)) if only_citations_moved(&base_body, &head_body) => {}
            Ok(_) => problems.push(format!(
                "{path}: the body of an accepted decision has changed"
            )),
            Err(why) => problems.push(format!("{path}: {why}")),
        }
    }

    (accepted, problems)
}

/// Split an atom into its frontmatter and its body, line endings normalised.
fn split(text: &str) -> Result<(String, String), String> {
    let text = text.replace("\r\n", "\n");
    let rest = text
        .strip_prefix("---\n")
        .ok_or("it does not open with a `---` frontmatter delimiter")?;
    let end = rest
        .find("\n---\n")
        .or_else(|| rest.ends_with("\n---").then(|| rest.len() - 4))
        .ok_or("its frontmatter is never closed by a `---` line")?;
    let front = rest[..end].to_owned();
    let body = rest[end + 4..]
        .strip_prefix('\n')
        .unwrap_or(&rest[end + 4..])
        .to_owned();
    Ok((front, body))
}

/// Whether two bodies are the same once every `:<digits>[-<digits>]` is erased.
///
/// Line for line, so a repair cannot add or remove a sentence under cover of a
/// citation. What it admits beyond citations is any other number after a colon —
/// a time of day, say — which is a narrow enough hole to name rather than close.
fn only_citations_moved(base: &str, head: &str) -> bool {
    base == head
        || (base.lines().count() == head.lines().count()
            && base
                .lines()
                .zip(head.lines())
                .all(|(b, h)| b == h || erase_line_numbers(b) == erase_line_numbers(h)))
}

/// A line with the digits after each `:`, and after a `-` that follows them, erased.
fn erase_line_numbers(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        out.push(c);
        if c != ':' || !chars.peek().is_some_and(char::is_ascii_digit) {
            continue;
        }
        out.push('#');
        while chars.peek().is_some_and(char::is_ascii_digit) {
            chars.next();
        }
        if chars.peek() == Some(&'-') {
            let mut ahead = chars.clone();
            ahead.next();
            if ahead.peek().is_some_and(char::is_ascii_digit) {
                chars.next();
                while chars.peek().is_some_and(char::is_ascii_digit) {
                    chars.next();
                }
            }
        }
    }
    out
}

/// The first word of the frontmatter's top-level `status:` value, unquoted.
fn status(front: &str) -> Option<&str> {
    front.lines().find_map(|line| {
        let value = line.strip_prefix("status:")?;
        value
            .split_whitespace()
            .next()
            .map(|word| word.trim_matches(|c| c == '"' || c == '\''))
    })
}

/// Run a git command in `root` and return its stdout.
fn git(root: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .with_context(|| format!("failed to launch `git {}`", args.join(" ")))?;
    if !out.status.success() {
        bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    String::from_utf8(out.stdout).context("git printed non-UTF-8")
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = ".kb/decisions/0099-a-decision.md";

    const ACCEPTED: &str = "---\r\nid: kb-decision-0099\r\nkind: decision\r\nstatus: accepted\r\n\
                            superseded_by: null\r\n---\r\n\r\n# A decision\r\n\r\nWe chose A.\r\n";

    fn base() -> BTreeMap<String, String> {
        BTreeMap::from([(PATH.to_owned(), ACCEPTED.to_owned())])
    }

    fn head(text: Option<&str>) -> BTreeMap<String, Option<String>> {
        BTreeMap::from([(PATH.to_owned(), text.map(str::to_owned))])
    }

    fn problems(text: Option<&str>) -> Vec<String> {
        changed_bodies(&base(), &head(text)).1
    }

    // The wrong implementations this step exists to reject, written down first.
    // Each is a plausible way to build this check, and each is refuted by the
    // test that names it.

    /// Wrong: diff the whole file. Refuses the supersession the rule channels
    /// corrections into.
    fn whole_file_diff(base: &str, head: &str) -> bool {
        base == head
    }

    /// Wrong: read the status at `HEAD`. A body edit made together with a flip
    /// to `draft` is waved through.
    fn accepted_at_head_only(base: &str, head: &str) -> bool {
        let (front, body) = split(head).expect("the fixture parses");
        status(&front) != Some("accepted") || body == split(base).expect("the fixture parses").1
    }

    /// Wrong: when there is no base, pass. The step then reports green on every
    /// shallow CI clone, which is where it runs.
    fn skip_without_base(merge_base: Option<&str>) -> Option<String> {
        merge_base.map(str::to_owned)
    }

    /// Wrong: walk the atoms that exist at `HEAD`. A deleted accepted decision is
    /// never visited.
    fn head_only_walk(head: &BTreeMap<String, Option<String>>) -> usize {
        head.values().filter(|text| text.is_some()).count()
    }

    #[test]
    fn a_whole_file_diff_refuses_a_legitimate_supersession() {
        let superseded = ACCEPTED
            .replace("status: accepted", "status: superseded")
            .replace("superseded_by: null", "superseded_by: kb-decision-0100");
        assert!(
            !whole_file_diff(ACCEPTED, &superseded),
            "the wrong implementation refuses it"
        );
        assert!(
            problems(Some(&superseded)).is_empty(),
            "{:?}",
            problems(Some(&superseded))
        );
    }

    #[test]
    fn a_body_edit_hidden_behind_a_status_flip_is_refused() {
        let hidden = ACCEPTED
            .replace("status: accepted", "status: draft")
            .replace("We chose A.", "We chose B.");
        assert!(
            accepted_at_head_only(ACCEPTED, &hidden),
            "the wrong implementation passes it"
        );
        assert_eq!(
            problems(Some(&hidden)),
            vec![format!(
                "{PATH}: the body of an accepted decision has changed"
            )]
        );
    }

    #[test]
    fn a_missing_base_is_an_error_not_a_pass() {
        assert_eq!(
            skip_without_base(None),
            None,
            "the wrong implementation skips"
        );
        let err = pick_base(None, false, None, "h", Some("p"))
            .expect_err("refused")
            .to_string();
        assert!(err.contains("no merge base"), "{err}");
        let err = pick_base(None, true, Some("m"), "h", Some("p"))
            .expect_err("refused")
            .to_string();
        assert!(err.contains("shallow"), "{err}");
        let err = pick_base(Some("0000000000"), true, None, "h", None)
            .expect_err("refused")
            .to_string();
        assert!(
            err.contains("shallow"),
            "an all-zero push `before` is not a base: {err}"
        );
    }

    #[test]
    fn a_deleted_accepted_decision_is_refused() {
        assert_eq!(
            head_only_walk(&head(None)),
            0,
            "the wrong implementation visits nothing"
        );
        assert_eq!(
            problems(None),
            vec![format!(
                "{PATH}: accepted at the base, and deleted or moved since"
            )]
        );
    }

    #[test]
    fn a_repointed_citation_is_the_one_repair_that_passes() {
        let cited = ACCEPTED.replace("We chose A.", "We chose A (`src/lib.rs:12-30`).");
        let at_base = BTreeMap::from([(PATH.to_owned(), cited.clone())]);
        let repointed = cited.replace("lib.rs:12-30", "lib.rs:14");
        assert_eq!(
            changed_bodies(&at_base, &head(Some(&repointed))),
            (1, vec![])
        );

        let reworded = repointed.replace("We chose A", "We chose B");
        assert_eq!(changed_bodies(&at_base, &head(Some(&reworded))).1.len(), 1);
        let renamed = cited.replace("src/lib.rs", "src/main.rs");
        assert_eq!(changed_bodies(&at_base, &head(Some(&renamed))).1.len(), 1);
        let grown = format!(
            "{repointed}A new sentence.
"
        );
        assert_eq!(changed_bodies(&at_base, &head(Some(&grown))).1.len(), 1);
    }

    #[test]
    fn a_plain_body_edit_is_refused() {
        let edited = ACCEPTED.replace("We chose A.", "We chose A, mostly.");
        assert_eq!(problems(Some(&edited)).len(), 1);
    }

    #[test]
    fn line_endings_alone_are_not_an_edit() {
        let lf = ACCEPTED.replace("\r\n", "\n");
        assert!(problems(Some(&lf)).is_empty(), "{:?}", problems(Some(&lf)));
    }

    #[test]
    fn an_unchanged_tree_counts_its_accepted_atoms() {
        assert_eq!(changed_bodies(&base(), &head(Some(ACCEPTED))), (1, vec![]));
    }

    #[test]
    fn an_atom_not_accepted_at_the_base_is_free() {
        let draft = ACCEPTED.replace("status: accepted", "status: proposed");
        let at_base = BTreeMap::from([(PATH.to_owned(), draft)]);
        let edited = ACCEPTED.replace("We chose A.", "We chose B.");
        assert_eq!(changed_bodies(&at_base, &head(Some(&edited))), (0, vec![]));
    }

    #[test]
    fn a_malformed_atom_fails_loudly() {
        let unclosed = "---\nstatus: accepted\n# no closing delimiter\n";
        let at_base = BTreeMap::from([(PATH.to_owned(), unclosed.to_owned())]);
        let (_, found) = changed_bodies(&at_base, &head(Some(unclosed)));
        assert_eq!(found.len(), 1, "{found:?}");

        let no_status = ACCEPTED.replace("status: accepted\r\n", "");
        let at_base = BTreeMap::from([(PATH.to_owned(), no_status.clone())]);
        let (_, found) = changed_bodies(&at_base, &head(Some(&no_status)));
        assert_eq!(found.len(), 1, "{found:?}");
    }

    #[test]
    fn the_base_is_chosen_in_the_documented_order() {
        assert_eq!(
            pick_base(Some(" abc "), true, None, "h", None).expect("resolves"),
            "abc"
        );
        assert_eq!(
            pick_base(None, false, Some("m"), "h", Some("p")).expect("resolves"),
            "m"
        );
        assert_eq!(
            pick_base(None, false, Some("h"), "h", Some("p")).expect("resolves"),
            "p"
        );
        assert!(pick_base(None, false, Some("h"), "h", None).is_err());
    }

    #[test]
    fn a_status_value_is_read_past_quotes_and_comments() {
        assert_eq!(status("status: \"accepted\""), Some("accepted"));
        assert_eq!(status("status: accepted  # UNCHANGED"), Some("accepted"));
        assert_eq!(status("id: x\nkind: decision"), None);
    }
}
