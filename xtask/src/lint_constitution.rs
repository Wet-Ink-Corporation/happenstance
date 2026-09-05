//! The Rust constitution's internal consistency check (`standards/rust/`).
//!
//! The constitution claims its examples compile and its citations resolve. The
//! first claim is discharged by `cargo test -p xtask --doc`, which compiles every
//! atom through [`crate::constitution`]. This module discharges the rest, and it
//! exists for the same reason `lints.rs` does: there is no type that says "this
//! rule names a wrong implementation", no compiler pass that reads a router, and
//! no lint that notices an atom nobody links to.
//!
//! # What this does not verify
//!
//! Stated first, because a check whose limits are undocumented is read as a
//! guarantee — `lints.rs` makes the same argument at greater length.
//!
//! * **It does not judge whether a rule is *right*.** [`MIN_REJECTS_CHARS`] is a
//!   length, exactly as `MIN_CHARS_PER_RULE` is, and for the same reason: the
//!   vocabulary test was tried in the neighbouring lint and produced nine false
//!   positives. A long and vacuous `**Rejects.**` passes here. The instrument for
//!   that is an adversarial reader, not a byte count.
//! * **It does not check that a citation's *claim* is true**, only that the
//!   cited file has that line and that the line carries the anchor text. Not
//!   *near* it: [`anchor_problem`] replaced a ten-line window that let a third
//!   of the corpus be green while pointing somewhere else.
//! * **It does not run the examples.** A fence tagged `rust` is asserted to
//!   exist; whether it compiles is the doctest step's answer, and the two are
//!   separate gate steps on purpose.
//! * **It does not lint fence bodies as Rust.** `cargo clippy` does not lint
//!   doctests at all, so the workspace lint set is unenforced inside every
//!   example in the corpus. [`FORBIDDEN_IN_FENCE`] is a deliberately narrow
//!   compensation for the two spellings that matter, not a substitute.
//!
//! # Why the citation parser hard-errors instead of skipping
//!
//! `spec_trace::citations` silently skips any backtick span it does not
//! recognise, which is safe there because `SPECIFICATION.md`'s spans are mostly
//! identifiers. Here the span *is* the check, so a citation this parser declines
//! to read is a citation nothing verifies — the failure mode the step exists to
//! prevent. So a span that looks like a citation and does not parse is a hard
//! error naming the file and line.
//!
//! The concrete case that forced this: a drafting pass produced
//! `` `path/to/file.rs:14 (`spawn_blocking` panics)` ``, with backticks *inside*
//! the span. Splitting on backticks mis-pairs every span after it on that line,
//! so a parser built like `spec_trace`'s would have read the anchor as a path
//! and skipped both. It renders wrongly too, which is the visible half of the
//! same defect.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::spec_trace::workspace_root;

/// The directory the atoms live in.
const ATOM_DIR: &str = "standards/rust";

/// The router, which is the only file in [`ATOM_DIR`] that is not an atom.
const ROUTER: &str = "standards/rust/README.md";

/// The harness that compiles the atoms' examples.
const HARNESS: &str = "xtask/src/constitution.rs";

/// The files whose style summaries must point at the router (C12).
const SUMMARIES: &[&str] = &["CLAUDE.md", "CONTRIBUTING.md"];

/// The five section markers a rule carries, in the order they must appear.
const SECTIONS: [&str; 5] = [
    "**Why.**",
    "**Do**",
    "**Not**",
    "**Rejects.**",
    "**Evidence.**",
];

/// The shortest `**Rejects.**` that has ever said anything.
///
/// Calibrated the way [`crate::lints::MIN_CHARS_PER_RULE`] was — against the
/// corpus, as a length rather than a vocabulary. A rule whose `**Rejects.**` is
/// "this would be confusing" is the wrong state; every real one in the corpus
/// clears this comfortably, because naming who is misled and when they find out
/// does not fit in less.
const MIN_REJECTS_CHARS: usize = 120;

/// The most rules one atom may carry.
///
/// The corpus exists so an agent loads one to three files. Six rules is the
/// point past which an atom stops being loadable and becomes a chapter.
const MAX_RULES_PER_ATOM: usize = 6;

/// The largest an atom may be, in bytes.
///
/// Bytes rather than lines, deliberately: a line ceiling is satisfied by writing
/// longer lines, which is worse for the token budget this cap exists to protect,
/// not better.
const MAX_ATOM_BYTES: usize = 16_384;

/// Spellings that must not appear inside a Rust fence outside the 60s band.
///
/// `unwrap_used` is `deny` workspace-wide and unenforceable here, so an atom can
/// violate the rule it states. The 60s band is exempt because test code
/// legitimately unwraps under a scoped allow, which is the thing those atoms are
/// about.
const FORBIDDEN_IN_FENCE: &[&str] = &[".unwrap()", ".expect("];

/// The file extensions that make a backticked span a citation.
///
/// This list is the *whole* test — see [`Span::rfind_path_colon`]. It is also
/// what keeps ordinary prose out: `Note: something` and `error[E0433]: cannot
/// find` carry a colon and no extension, so they are not citations.
///
/// `.yml` is here on purpose, and it is the entry that was missing: the
/// constitution cites `.github/workflows/ci.yml` three times, and until this
/// list grew that entry those three were parsed, discarded and reported as
/// nothing. A workflow file is exactly where a gate step's real definition
/// lives, so a citation into one is the kind this corpus most needs held
/// honest. Anything absent from this list is skipped in silence — the failure
/// this constant exists to make impossible to reintroduce by accident — so add
/// the extension rather than leaving a citation unchecked.
const CITED_EXTENSIONS: &[&str] = &[".rs", ".toml", ".md", ".yml"];

/// Whether to check the router's generated region or rewrite it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
    /// Fail when the region disagrees with the atoms.
    Check,
    /// Rewrite the region from the atoms.
    Write,
}

/// One atom, parsed far enough to check it.
#[derive(Debug)]
struct Atom {
    /// File name, e.g. `21-send-is-not-inherited.md`.
    file: String,
    /// The two-digit band, e.g. `21`.
    band: String,
    /// The doctest module name the harness must use.
    module: String,
    /// The whole file.
    text: String,
    /// The `Load when:` triggers, already stripped of their marker.
    load_when: String,
    /// Every `## RS-…` heading in order.
    rules: Vec<Rule>,
    /// Fenced blocks, in order.
    fences: Vec<Fence>,
}

/// One `## RS-…` rule.
#[derive(Debug)]
struct Rule {
    /// The full identifier, e.g. `RS-21-1`.
    id: String,
    /// 1-based line of the heading.
    line: usize,
    /// Everything from the heading to the next heading or end of file.
    body: String,
}

/// One fenced block.
#[derive(Debug)]
struct Fence {
    /// The info string as written, e.g. `rust,compile_fail,E0277`.
    info: String,
    /// 1-based line of the opening fence.
    line: usize,
    /// The fence's contents.
    body: String,
    /// The line immediately above the fence, for the `<!-- ignore: … -->` rule.
    preceding: String,
}

/// Runs every check, reporting all problems rather than the first.
///
/// # Errors
///
/// Fails when any check finds a problem, or when the corpus cannot be read.
pub(crate) fn run(mode: Mode) -> Result<()> {
    let root = workspace_root()?;
    let atoms = atoms(&root)?;
    let mut problems = Vec::new();

    if atoms.is_empty() {
        bail!("{ATOM_DIR} holds no atoms, so every check below is vacuous");
    }

    check_router(&root, &atoms, mode, &mut problems)?;
    check_harness(&root, &atoms, &mut problems)?;
    check_summaries(&root, &mut problems)?;

    let mut seen: BTreeMap<String, String> = BTreeMap::new();
    for atom in &atoms {
        check_shape(atom, &mut problems);
        check_rules(atom, &mut seen, &mut problems);
        check_fences(atom, &mut problems);
        check_citations(&root, atom, &mut problems);
    }

    if problems.is_empty() {
        println!("  {} atoms, all consistent", atoms.len());
        return Ok(());
    }

    for problem in &problems {
        eprintln!("  {problem}");
    }
    bail!("{} problem(s) in {ATOM_DIR}", problems.len())
}

/// Reads and parses every atom.
///
/// # Errors
///
/// Fails when the directory cannot be read, or an atom's name does not carry the
/// `NN-slug.md` shape the module name is derived from.
fn atoms(root: &Path) -> Result<Vec<Atom>> {
    let dir = root.join(ATOM_DIR);
    let mut out = Vec::new();

    let entries = fs::read_dir(&dir).with_context(|| format!("reading {ATOM_DIR}"))?;
    let mut files: Vec<String> = Vec::new();
    for entry in entries {
        let entry = entry.with_context(|| format!("reading an entry of {ATOM_DIR}"))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if has_ext(&name, ".md") && name != "README.md" {
            files.push(name);
        }
    }
    files.sort();

    for file in files {
        let text = fs::read_to_string(dir.join(&file))
            .with_context(|| format!("reading {ATOM_DIR}/{file}"))?;
        let stem = file.strip_suffix(".md").unwrap_or(&file);
        let Some((band, slug)) = stem.split_once('-') else {
            bail!("{ATOM_DIR}/{file} — an atom is named `NN-slug.md`; this has no band");
        };
        if band.len() != 2 || !band.chars().all(|c| c.is_ascii_digit()) {
            bail!("{ATOM_DIR}/{file} — the band `{band}` is not two digits");
        }
        out.push(Atom {
            band: band.to_owned(),
            module: slug.replace('-', "_"),
            load_when: load_when(&text),
            rules: rules(&text),
            fences: fences(&text),
            file,
            text,
        });
    }
    Ok(out)
}

/// The `Load when:` triggers, or the empty string when the line is absent.
fn load_when(text: &str) -> String {
    for line in text.lines() {
        let trimmed = line.trim_start_matches(['>', ' ']);
        if let Some(rest) = trimmed.strip_prefix("**Load when:**") {
            return rest.trim().to_owned();
        }
    }
    String::new()
}

/// Every `## RS-…` rule, with its body.
fn rules(text: &str) -> Vec<Rule> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out: Vec<Rule> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let Some(rest) = line.strip_prefix("## RS-") else {
            continue;
        };
        let id: String = rest
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect();
        out.push(Rule {
            id: format!("RS-{}", id.trim_end_matches('-')),
            line: index + 1,
            body: String::new(),
        });
        let start = index;
        let end = lines
            .iter()
            .skip(start + 1)
            .position(|l| l.starts_with("## "))
            .map_or(lines.len(), |offset| start + 1 + offset);
        if let Some(last) = out.last_mut() {
            last.body = lines[start..end].join("\n");
        }
    }
    out
}

/// Every fenced block, with the line above it.
fn fences(text: &str) -> Vec<Fence> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    let mut open: Option<(usize, String, Vec<String>)> = None;

    // Four backticks open a block whose *contents* are shown rather than run —
    // it is how the corpus quotes the atom template, inner ```rust fence and
    // all. Skipping only the delimiter lines would leave that inner fence
    // looking like an example, which rustdoc does not compile and this must not
    // count. So the whole block is stepped over.
    let mut quoted = false;

    for (index, line) in lines.iter().enumerate() {
        if line.starts_with("````") {
            quoted = !quoted;
            continue;
        }
        if quoted {
            continue;
        }
        let Some(rest) = line.strip_prefix("```") else {
            if let Some((_, _, body)) = open.as_mut() {
                body.push((*line).to_owned());
            }
            continue;
        };
        match open.take() {
            None => open = Some((index, rest.trim().to_owned(), Vec::new())),
            Some((start, info, body)) => out.push(Fence {
                info,
                line: start + 1,
                body: body.join("\n"),
                preceding: start
                    .checked_sub(1)
                    .and_then(|above| lines.get(above))
                    .unwrap_or(&"")
                    .trim()
                    .to_owned(),
            }),
        }
    }
    out
}

/// C1 and C2 — the router names every atom exactly once, and its generated
/// region agrees with what the atoms actually say.
fn check_router(root: &Path, atoms: &[Atom], mode: Mode, problems: &mut Vec<String>) -> Result<()> {
    let path = root.join(ROUTER);
    let router = fs::read_to_string(&path).with_context(|| format!("reading {ROUTER}"))?;

    // Every `.md` the router links must resolve. Deliberately a link check and
    // not a mention count: a markdown link spells its target twice, so counting
    // mentions reports every correctly-linked atom as a duplicate — which is
    // what the first version of this check did. The bijection over the atom set
    // is the region equality below, because the region is built from that set.
    for (line_no, line) in router.lines().enumerate() {
        for link in markdown_links(line) {
            let target = link.split('#').next().unwrap_or(&link);
            if !has_ext(target, ".md") {
                continue;
            }
            if !root.join(ATOM_DIR).join(target).exists() {
                problems.push(format!(
                    "{ROUTER}:{} — link `{link}` resolves to no file",
                    line_no + 1
                ));
            }
        }
    }

    let generated = generated_region(atoms);
    match region(&router) {
        None => problems.push(format!(
            "{ROUTER} — no `<!-- BEGIN GENERATED -->` / `<!-- END GENERATED -->` region"
        )),
        Some((start, end)) => {
            let current: Vec<&str> = router.lines().collect();
            let committed = current[start..end].join("\n");
            if committed.trim() != generated.trim() {
                if mode == Mode::Write {
                    let mut rebuilt: Vec<String> =
                        current[..start].iter().map(|l| (*l).to_owned()).collect();
                    rebuilt.push(generated);
                    rebuilt.extend(current[end..].iter().map(|l| (*l).to_owned()));
                    fs::write(&path, rebuilt.join("\n") + "\n")
                        .with_context(|| format!("writing {ROUTER}"))?;
                    println!("  rewrote {ROUTER}'s generated region");
                } else {
                    problems.push(format!(
                        "{ROUTER} — the generated region disagrees with the atoms; \
                         run `cargo xtask lint-constitution --write`"
                    ));
                }
            }
        }
    }
    Ok(())
}

/// The line range strictly between the region markers.
fn region(router: &str) -> Option<(usize, usize)> {
    let lines: Vec<&str> = router.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.trim() == "<!-- BEGIN GENERATED -->")?;
    let end = lines
        .iter()
        .position(|l| l.trim() == "<!-- END GENERATED -->")?;
    (start < end).then_some((start + 1, end))
}

/// What the router's generated region must contain.
fn generated_region(atoms: &[Atom]) -> String {
    let mut out = vec![
        "| Atom | Load when | Rules |".to_owned(),
        "|---|---|---|".to_owned(),
    ];
    for atom in atoms {
        let ids: Vec<&str> = atom.rules.iter().map(|r| r.id.as_str()).collect();
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

/// C3 — every atom is included by the harness, in a module of its own.
fn check_harness(root: &Path, atoms: &[Atom], problems: &mut Vec<String>) -> Result<()> {
    let harness =
        fs::read_to_string(root.join(HARNESS)).with_context(|| format!("reading {HARNESS}"))?;

    for atom in atoms {
        let include = format!("include_str!(\"../../{ATOM_DIR}/{}\")", atom.file);
        let declared = format!("mod {} {{", atom.module);
        if !harness.contains(&include) {
            problems.push(format!(
                "{HARNESS} — does not include {}; its examples are never compiled",
                atom.file
            ));
        }
        if !harness.contains(&declared) {
            problems.push(format!(
                "{HARNESS} — no `mod {}`; one module per atom is what keeps a \
                 doctest failure's line number relative to the atom",
                atom.module
            ));
        }
    }

    // The reverse: a module left behind by a renamed atom still compiles, because
    // `cfg(doctest)` hides it from every step but `cargo test`.
    for line in harness.lines() {
        let Some(rest) = line.trim().strip_prefix("mod ") else {
            continue;
        };
        let name = rest.trim_end_matches(" {");
        if !atoms.iter().any(|a| a.module == name) {
            problems.push(format!(
                "{HARNESS} — `mod {name}` names no atom in {ATOM_DIR}"
            ));
        }
    }
    Ok(())
}

/// C12 — the style summaries point at the router rather than restating it.
fn check_summaries(root: &Path, problems: &mut Vec<String>) -> Result<()> {
    for file in SUMMARIES {
        let text =
            fs::read_to_string(root.join(file)).with_context(|| format!("reading {file}"))?;
        if !text.contains(ATOM_DIR) {
            problems.push(format!(
                "{file} — does not link {ATOM_DIR}; a summary that does not point \
                 at the corpus is a second copy of it, and one of the two will be stale"
            ));
        }
    }
    Ok(())
}

/// C4 (static half), C10 — the atom's own shape.
fn check_shape(atom: &Atom, problems: &mut Vec<String>) {
    let file = &atom.file;

    if !atom.text.starts_with(&format!("# {} — ", atom.band)) {
        problems.push(format!(
            "{ATOM_DIR}/{file}:1 — the title must read `# {} — <title>`",
            atom.band
        ));
    }
    if atom.load_when.is_empty() {
        problems.push(format!(
            "{ATOM_DIR}/{file} — no `> **Load when:**` line; it is the router's source of truth"
        ));
    }
    if !atom.text.contains("**See also:**") {
        problems.push(format!("{ATOM_DIR}/{file} — no `> **See also:**` line"));
    }
    if atom.rules.is_empty() {
        problems.push(format!("{ATOM_DIR}/{file} — carries no `## RS-` rule"));
    }
    if atom.rules.len() > MAX_RULES_PER_ATOM {
        problems.push(format!(
            "{ATOM_DIR}/{file} — {} rules, and the ceiling is {MAX_RULES_PER_ATOM}; split the atom",
            atom.rules.len()
        ));
    }
    if atom.text.len() > MAX_ATOM_BYTES {
        problems.push(format!(
            "{ATOM_DIR}/{file} — {} bytes, and the ceiling is {MAX_ATOM_BYTES}; \
             an agent loading this pays for all of it",
            atom.text.len()
        ));
    }

    // C4's static half. `include_str!` proves the file is read; `cargo test`
    // exits 0 on "running 0 tests", so an atom whose fences are all ```text
    // would leave the doctest step green over an atom that compiles nothing.
    let compiling = atom.fences.iter().filter(|f| is_compiling(&f.info)).count();
    if compiling == 0 {
        problems.push(format!(
            "{ATOM_DIR}/{file} — no compiling `rust` fence; the doctest step would \
             pass over this atom without compiling anything"
        ));
    }

    // C6. A `compile_fail` with no compiling neighbour has no positive control,
    // and rustdoc 1.97.1 silently ignores an unmatched error code.
    if atom.fences.iter().any(|f| f.info.contains("compile_fail")) && compiling == 0 {
        problems.push(format!(
            "{ATOM_DIR}/{file} — a `compile_fail` fence with no compiling fence beside it"
        ));
    }
}

/// Whether a fence is compiled and counted by the doctest step.
fn is_compiling(info: &str) -> bool {
    info == "rust" || info == "rust,no_run" || info == "rust,should_panic"
}

/// Whether `path` ends in `ext`, case-insensitively.
///
/// Case-insensitively for the reason `spec_trace` gives: the repository is
/// developed on Windows, where `.RS` names the same file, and a citation the
/// checker silently declines to verify is the failure mode this step exists to
/// prevent.
fn has_ext(path: &str, ext: &str) -> bool {
    path.len() > ext.len() && path[path.len() - ext.len()..].eq_ignore_ascii_case(ext)
}

/// C7, C11, and the template's section order.
fn check_rules(atom: &Atom, seen: &mut BTreeMap<String, String>, problems: &mut Vec<String>) {
    let file = &atom.file;

    for rule in &atom.rules {
        let expected = format!("RS-{}-", atom.band);
        if !rule.id.starts_with(&expected) {
            problems.push(format!(
                "{ATOM_DIR}/{file}:{} — `{}` does not carry this atom's band `{expected}`",
                rule.line, rule.id
            ));
        }
        if let Some(first) = seen.insert(rule.id.clone(), file.clone()) {
            problems.push(format!(
                "{ATOM_DIR}/{file}:{} — `{}` is already used in {first}; a rule id \
                 names one rule or it names nothing",
                rule.line, rule.id
            ));
        }

        let mut at = 0usize;
        for section in SECTIONS {
            match rule.body[at..].find(section) {
                None => problems.push(format!(
                    "{ATOM_DIR}/{file}:{} — `{}` has no `{section}` section, or it is out of order",
                    rule.line, rule.id
                )),
                Some(offset) => at += offset + section.len(),
            }
        }

        let rejects = section_text(&rule.body, "**Rejects.**", "**Evidence.**");
        if rejects.chars().count() < MIN_REJECTS_CHARS {
            problems.push(format!(
                "{ATOM_DIR}/{file}:{} — `{}`'s `**Rejects.**` is {} characters and the \
                 floor is {MIN_REJECTS_CHARS}; name a wrong state that could ship, and who finds out when",
                rule.line,
                rule.id,
                rejects.chars().count()
            ));
        }
    }
}

/// The text between two markers, or the empty string.
fn section_text(body: &str, from: &str, to: &str) -> String {
    let Some(start) = body.find(from) else {
        return String::new();
    };
    let rest = &body[start + from.len()..];
    let end = rest.find(to).unwrap_or(rest.len());
    rest[..end].trim().to_owned()
}

/// C5 — fence discipline, and the narrow clippy compensation.
fn check_fences(atom: &Atom, problems: &mut Vec<String>) {
    let file = &atom.file;
    let is_test_band = atom.band.starts_with('6');

    for fence in &atom.fences {
        let info = fence.info.as_str();
        let at = format!("{ATOM_DIR}/{file}:{}", fence.line);

        let rust = info == "rust" || info.starts_with("rust,");
        if !rust {
            // A non-Rust tag is fine; an untagged fence is not, because rustdoc
            // treats it as Rust and would try to compile it.
            if info.is_empty() {
                problems.push(format!(
                    "{at} — an untagged fence is compiled as Rust; tag it `rust` or `text`"
                ));
            }
            continue;
        }

        let mut parts = info.split(',').skip(1);
        let mut ok = true;
        let mut ignored = false;
        let mut compile_fail = false;
        let mut code: Option<&str> = None;
        for part in &mut parts {
            match part {
                "no_run" | "should_panic" => {}
                "ignore" => ignored = true,
                "compile_fail" => compile_fail = true,
                other if other.starts_with('E') && other.len() == 5 => code = Some(other),
                _ => ok = false,
            }
        }
        if !ok {
            problems.push(format!("{at} — unrecognised fence info string `{info}`"));
        }
        if ignored && !fence.preceding.starts_with("<!-- ignore:") {
            problems.push(format!(
                "{at} — `ignore` needs an `<!-- ignore: <reason> -->` comment on the line above; \
                 without one it is indistinguishable from an example that stopped compiling"
            ));
        }
        if code.is_some() && !compile_fail {
            problems.push(format!(
                "{at} — an error code on a fence that is not `compile_fail`"
            ));
        }
        if compile_fail && let Some(code) = code {
            // rustdoc 1.97.1 accepts a `compile_fail` whose code never matches, so
            // the prose naming the code is the part a reader can check.
            if !atom.text.contains(code) {
                problems.push(format!(
                    "{at} — the fence claims `{code}` and the atom's prose never names it"
                ));
            }
        }

        if !is_test_band {
            for forbidden in FORBIDDEN_IN_FENCE {
                if fence.body.contains(forbidden) {
                    problems.push(format!(
                        "{at} — `{forbidden}` inside a fence. `unwrap_used` is `deny` \
                         workspace-wide and clippy does not lint doctests, so an example \
                         may violate the rule the corpus states"
                    ));
                }
            }
        }
    }
}

/// C8 and C9 — citations resolve to the right line, and links exist.
fn check_citations(root: &Path, atom: &Atom, problems: &mut Vec<String>) {
    let file = &atom.file;

    for (line_no, line) in atom.text.lines().enumerate() {
        let at = format!("{ATOM_DIR}/{file}:{}", line_no + 1);

        for span in spans(line) {
            let Some(colon) = span.rfind_path_colon() else {
                continue;
            };
            let (path, tail) = (&span.text[..colon], &span.text[colon + 1..]);
            let Some(citation) = parse_citation(tail) else {
                problems.push(format!(
                    "{at} — `{}` looks like a citation and does not parse as \
                     `path:line (anchor)`; a citation the checker cannot read is one nothing verifies",
                    span.text
                ));
                continue;
            };
            let target = root.join(path);
            let Ok(text) = fs::read_to_string(&target) else {
                problems.push(format!("{at} — `{path}` does not exist"));
                continue;
            };
            let lines: Vec<&str> = text.lines().collect();
            if citation.line == 0 || citation.line > lines.len() {
                problems.push(format!(
                    "{at} — `{path}:{}` is past the end of a {}-line file",
                    citation.line,
                    lines.len()
                ));
                continue;
            }
            if let Some(problem) = anchor_problem(path, &lines, &citation) {
                problems.push(format!("{at} — {problem}"));
            }
        }

        // C9's first half. A relative link into the repository must resolve.
        for link in markdown_links(line) {
            if link.starts_with("http") {
                continue;
            }
            let cleaned = link.split('#').next().unwrap_or(&link);
            if cleaned.is_empty() || !has_ext(cleaned, ".md") {
                continue;
            }
            if !root.join(ATOM_DIR).join(cleaned).exists() {
                problems.push(format!(
                    "{at} — link `{link}` resolves to no file (relative to {ATOM_DIR})"
                ));
            }
        }
    }

    // C9's second half, scoped to the rule rather than the file. Asking whether
    // the *atom* contains a stamp anywhere would pass an unstamped URL in every
    // rule but the first, which is the check being decorative rather than wrong.
    for rule in &atom.rules {
        let evidence = section_text(&rule.body, "**Evidence.**", "\n## ");
        if evidence.contains("http") && !evidence.contains("(checked ") {
            problems.push(format!(
                "{ATOM_DIR}/{file}:{} — `{}` cites an external URL with no \
                 `*(checked YYYY-MM-DD, rustc X.Y.Z)*` stamp; an unstamped external \
                 claim cannot be told from one that was never checked",
                rule.line, rule.id
            ));
        }
    }
}

/// What is wrong with a citation whose stated line does not carry its anchor,
/// and where the anchor actually is.
///
/// **Exact, with no window at all.** A `const ANCHOR_SLACK: usize = 10` stood
/// here, and its rationale was a prediction about a distribution — *ordinary
/// editing* stays inside ten lines, *a function moving* does not — that nobody
/// had ever measured. Measured on 2026-09-04 over the whole corpus: 88 of 323
/// anchored citations were green only on the tolerance, the offsets did not
/// decay with distance (a lobe of 22 at +7, ten at +9, two at exactly +10, one
/// inserted line from red), and three citations had already drifted onto the
/// *wrong* line while staying green — a closing brace cited for an attribute,
/// an `[advisories]` table cited for a licences rule, and the line above a
/// field. A green read as coverage and not coverage is worse than no check,
/// because it retires the reader's own vigilance, and
/// `standards/rust/README.md:118` promises every reader of the corpus exactly
/// the check this now performs.
///
/// The tolerance existed to spare a maintainer a renumbering, and that cost was
/// real. It is paid here instead: on a miss the whole file is searched and the
/// message **names the line the citation should carry**, so the repair is one
/// keystroke rather than a `grep`.
///
/// Where the anchor is not unique the message lists every candidate and
/// **refuses to choose**. That refusal is the point, not a limitation: a
/// checker that guesses at an ambiguous anchor has reintroduced, with more
/// confidence, the defect it exists to catch — which is what the window was
/// doing every time it silently accepted the *nearest* occurrence.
///
/// # Why `spec_trace` still has a window, and this does not
///
/// [`crate::spec_trace`] keeps `const ANCHOR_SLACK: usize = 12` over
/// `SPECIFICATION.md`. The two numbers differed silently for as long as both
/// existed; they differ *stated* now, and this is one half of the statement —
/// `spec_trace.rs:378 (Why this window survives)` is the other.
///
/// The difference is not a disagreement about strictness. **Here the anchor is
/// written and there it is derived.** An atom quotes its anchor beside the line
/// number, so "the cited line contains it" is exactly the claim the citation
/// makes. `spec_trace` has no quoted text: it takes the identifier the
/// specification's prose reached for and looks for it near a range that points
/// at the evidence — usually a doc comment, with the signature carrying the
/// identifier just outside it. Closing that window reddens 21 of its 80 anchored
/// citations, 20 of them inside `[FROZEN]` clause commentary, and the repair
/// would move each citation off the prose it is evidence for. Measured
/// 2026-09-04; the numbers and the worked cases are at that site.
///
/// # The wrong implementation
///
/// Returning the nearest occurrence when there are several. It is one line of
/// code and it is the whole `[advisories]`-cited-for-`[licenses]` failure,
/// automated.
fn anchor_problem(path: &str, lines: &[&str], citation: &Citation) -> Option<String> {
    if lines[citation.line - 1].contains(&citation.anchor) {
        return None;
    }
    let found: Vec<String> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains(&citation.anchor))
        .map(|(index, _)| (index + 1).to_string())
        .collect();
    let head = format!(
        "`{path}:{}` does not carry `{}`",
        citation.line, citation.anchor
    );
    Some(match found.as_slice() {
        [] => format!(
            "{head}, and the anchor is nowhere in the file. No renumbering finds \
             a line that is not there: read for the surviving statement of the \
             same claim, cite that, and say in the change that you chose it"
        ),
        [only] => format!("{head}; it is on line {only}"),
        candidates => format!(
            "{head}; the anchor is on lines {}. Choose one by reading — this \
             check will not choose for you, because an anchor that matches many \
             lines is a badly chosen anchor and sharpening it is the repair",
            candidates.join(", ")
        ),
    })
}

/// A backtick-delimited span, with the position it started at.
struct Span {
    /// The span's contents, without its delimiters.
    text: String,
}

impl Span {
    /// The colon separating a path from its line number, if the span has one.
    ///
    /// A **known file extension** is what makes a span a citation. The test this
    /// replaced asked for a slash before the colon, which kept prose out — but
    /// silently discarded every citation into a repository-root file, because
    /// `Cargo.toml:160` has no slash to find. That was one predicate here and a
    /// second, disagreeing one at the call site; there is one now, and
    /// [`CITED_EXTENSIONS`] is it.
    fn rfind_path_colon(&self) -> Option<usize> {
        let colon = self.text.find(':')?;
        let path = &self.text[..colon];
        CITED_EXTENSIONS
            .iter()
            .any(|ext| has_ext(path, ext))
            .then_some(colon)
    }
}

/// Every backtick-delimited span on a line.
///
/// A span containing a backtick cannot exist — the delimiter is the character —
/// so the naive pairing here is exact, and a nested backtick shows up as a span
/// that fails to parse rather than one silently skipped.
fn spans(line: &str) -> Vec<Span> {
    line.split('`')
        .skip(1)
        .step_by(2)
        .map(|text| Span {
            text: text.to_owned(),
        })
        .collect()
}

/// A parsed `line (anchor)` tail.
struct Citation {
    /// The cited line number.
    line: usize,
    /// The text that must sit near it.
    anchor: String,
}

/// Parses the part of a citation after the path's colon.
fn parse_citation(tail: &str) -> Option<Citation> {
    let digits: String = tail.chars().take_while(char::is_ascii_digit).collect();
    let line: usize = digits.parse().ok()?;
    let rest = tail[digits.len()..].trim_start();
    // A range cites its first line; the anchor is what pins it.
    let rest = rest
        .strip_prefix('-')
        .map_or(rest, |r| r.trim_start_matches(|c: char| c.is_ascii_digit()))
        .trim_start();
    let anchor = rest.strip_prefix('(')?.strip_suffix(')')?;
    (!anchor.trim().is_empty()).then(|| Citation {
        line,
        anchor: anchor.trim().to_owned(),
    })
}

/// Every `](target)` target on a line.
fn markdown_links(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find("](") {
        rest = &rest[start + 2..];
        if let Some(end) = rest.find(')') {
            out.push(rest[..end].to_owned());
            rest = &rest[end..];
        } else {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, reason = "test code, per the house style")]

    use super::*;

    #[test]
    fn a_citation_with_a_nested_backtick_does_not_parse() {
        // The real defect this parser exists to catch: splitting on backticks
        // mis-pairs every span after it, so a `spec_trace`-shaped parser reads the
        // anchor as a path and verifies neither.
        let spans = spans("`a/b.rs:14 (`spawn_blocking` panics)`");
        let first = spans.first().unwrap();
        assert!(parse_citation(&first.text[first.rfind_path_colon().unwrap() + 1..]).is_none());
    }

    #[test]
    fn a_citation_into_a_root_level_file_is_seen() {
        // The real defect this parser exists to catch: `Cargo.toml`, `deny.toml`
        // and `clippy.toml` sit at the repository root, so their citations carry
        // no slash. A path test that demands one drops them before the extension
        // filter downstream ever runs — not reported unparseable, not reported
        // external, just gone — and the workspace lint table's citations rot while
        // the checker prints "27 atoms, all consistent".
        let spans = spans("`Cargo.toml:160 (unsafe_code = \"forbid\")`");
        let first = spans.first().unwrap();
        let colon = first
            .rfind_path_colon()
            .expect("a root-level file is still a path");
        assert_eq!(&first.text[..colon], "Cargo.toml");
        let citation = parse_citation(&first.text[colon + 1..]).unwrap();
        assert_eq!(citation.line, 160);
        assert_eq!(citation.anchor, "unsafe_code = \"forbid\"");
    }

    #[test]
    fn prose_that_carries_a_colon_is_not_a_citation() {
        // What the discarded slash test was actually for. The corpus quotes
        // diagnostics and labels inside backticks, and reading one as a path
        // would turn `check_citations` into a machine for reporting that
        // `error[E0433]` does not exist — noise that trains a reader to skim
        // past the reports that are real.
        for prose in [
            "error[E0433]: failed to resolve",
            "Note: the bound is on the definition",
            "warning: unused import: `core::fmt`",
            "https://doc.rust-lang.org/nomicon",
            "Load when: adding a gate step",
        ] {
            let spans = spans(&format!("`{prose}`"));
            assert!(
                spans[0].rfind_path_colon().is_none(),
                "`{prose}` was read as a citation"
            );
        }
    }

    #[test]
    fn a_workflow_citation_is_seen() {
        // `.github/workflows/ci.yml` is cited three times and, until `.yml`
        // joined the extension list, checked never — the same silence as the
        // root-level files above, arriving by a different route.
        let spans = spans("`.github/workflows/ci.yml:309 (baseline-rev)`");
        let colon = spans[0].rfind_path_colon().unwrap();
        assert_eq!(&spans[0].text[..colon], ".github/workflows/ci.yml");
    }

    #[test]
    fn a_well_formed_citation_parses() {
        let citation = parse_citation("642 (fn spawns_from_generic)").unwrap();
        assert_eq!(citation.line, 642);
        assert_eq!(citation.anchor, "fn spawns_from_generic");
    }

    #[test]
    fn a_range_citation_uses_its_first_line() {
        let citation = parse_citation("272-282 (checked_add)").unwrap();
        assert_eq!(citation.line, 272);
    }

    #[test]
    fn an_anchorless_citation_is_refused() {
        assert!(parse_citation("642").is_none());
        assert!(parse_citation("642 ()").is_none());
    }

    #[test]
    fn four_backtick_fences_are_not_examples() {
        // The corpus shows the atom template inside a ````markdown fence. Read as
        // an example it would be compiled as Rust, and it is not Rust.
        let found = fences("````markdown\n```rust\nfn main() {}\n```\n````\n");
        assert!(found.iter().all(|f| f.info != "rust"));
    }

    #[test]
    fn rules_are_split_at_the_next_heading() {
        let found = rules("## RS-21-1. A.\nbody a\n\n## RS-21-2. B.\nbody b\n");
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].id, "RS-21-1");
        assert!(found[0].body.contains("body a"));
        assert!(!found[0].body.contains("body b"));
    }

    /// A fabricated workspace root under `std::env::temp_dir()`, never the
    /// workspace's own trees and never `tempfile` — mirroring
    /// `spec_trace::tests::fabricated_root`, which this module cannot reuse
    /// because it is a private helper of a sibling module. The nanosecond stamp
    /// keeps two parallel tests from sharing a directory.
    fn fabricated_root(label: &str) -> std::path::PathBuf {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|since| since.as_nanos())
            .unwrap_or_default();
        let path = std::env::temp_dir().join(format!("hs-lint-constitution-{label}-{stamp}"));
        fs::create_dir_all(&path).unwrap();
        path
    }

    /// An [`Atom`] carrying nothing but the text [`check_citations`] reads.
    fn atom_of(text: &str) -> Atom {
        Atom {
            file: "99-fixture.md".to_owned(),
            band: "99".to_owned(),
            module: "band_99".to_owned(),
            text: text.to_owned(),
            load_when: String::new(),
            rules: Vec::new(),
            fences: Vec::new(),
        }
    }

    /// Runs C8 over one fabricated atom against one fabricated target file.
    fn citations_of(label: &str, target: &str, body: &str, atom_text: &str) -> Vec<String> {
        let root = fabricated_root(label);
        let path = root.join(target);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, body).unwrap();
        let mut problems = Vec::new();
        check_citations(&root, &atom_of(atom_text), &mut problems);
        problems
    }

    #[test]
    fn a_citation_one_line_off_is_reported_and_told_where_to_go() {
        // The defect the window hid. `fn subject` is on line 3 and the citation
        // says 2, which is the smallest possible drift and the one an insertion
        // above a cited line produces every time. Under a tolerance of ten this
        // is green, and a reader who takes the step's "all consistent" for what
        // `standards/rust/README.md` says it means is misled: that page promises
        // the cited line *contains* its anchor, and a windowed check cannot
        // deliver it.
        let problems = citations_of(
            "one-line-off",
            "src/subject.rs",
            "// a\n// b\nfn subject() {}\n",
            "**Evidence.** `src/subject.rs:2 (fn subject)`\n",
        );
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(
            problems[0].contains("src/subject.rs:2"),
            "the report must name the citation: {}",
            problems[0]
        );
        assert!(
            problems[0].contains("line 3"),
            "the report must name the line the anchor is actually on: {}",
            problems[0]
        );
    }

    #[test]
    fn an_ambiguous_anchor_is_listed_and_not_chosen_for_you() {
        // The failure mode a repair message reintroduces if it guesses. Three
        // lines carry `impl Defect for` and nothing in the file says which one
        // the atom meant; a checker that picks the nearest is doing what the
        // slack did, one layer up and with more confidence. `mutants.rs` really
        // has seventy of these.
        let problems = citations_of(
            "ambiguous",
            "src/mutants.rs",
            "// a\n// b\nimpl Defect for A {}\n// d\n// e\nimpl Defect for B {}\n\
             // g\n// h\nimpl Defect for C {}\n// j\n",
            "**Evidence.** `src/mutants.rs:5 (impl Defect for)`\n",
        );
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(
            problems[0].contains("lines 3, 6, 9"),
            "the report must list every candidate: {}",
            problems[0]
        );
        assert!(
            problems[0].contains("choose"),
            "the report must leave the choice to a reader: {}",
            problems[0]
        );
    }

    #[test]
    fn an_anchor_deleted_from_the_file_asks_for_a_human() {
        // `52-wasm32-and-target-cfg.md`'s citation into `ci.yml`, whose anchor
        // text was removed outright at `8ea7bb7`. No renumbering can find a line
        // that is not there, and a message that says only "no longer has" sends
        // the reader looking for one.
        let problems = citations_of(
            "deleted",
            ".github/workflows/ci.yml",
            "name: ci\njobs:\n  gate:\n",
            "**Evidence.** `.github/workflows/ci.yml:2 (baseline-rev)`\n",
        );
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(
            problems[0].contains("nowhere in the file"),
            "the report must say the anchor is gone, not merely misplaced: {}",
            problems[0]
        );
    }
}
