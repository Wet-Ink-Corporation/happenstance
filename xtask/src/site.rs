//! `cargo xtask site`: the documentation site, assembled and built.
//!
//! Four sources meet under `target/site/src`, and none of the result is
//! committed:
//!
//! - `site/` — templates, styles and the pages that only route (the landing
//!   page, the guide's index, the reference page). Presentation, not teaching.
//! - `docs/` — the narrative tree the gate compiles. Each page is rendered by
//!   [`render::guide_page`] into the guide section, placed by the need it
//!   declares, so the site cannot carry an example the gate has not compiled.
//! - `assets/brand/` — the artwork, copied rather than duplicated, and inlined
//!   by the templates so a wordmark takes the colour of the ground it lands on.
//! - `target/site-doc/doc` — a nightly rustdoc build of the workspace with
//!   scraped examples, served under `api/`.
//!
//! Then `zola build` (or `zola serve`). Zola refuses an internal link that does
//! not resolve, which is why a sibling-page link is rewritten to `@/`: the site
//! build is the check that the guide's cross-links still land.
//!
//! **Why the rustdoc build is nightly, and not in the gate.** Scraped examples
//! are `-Zrustdoc-scrape-examples`, which is unstable. The gate's own rustdoc
//! steps are the stable, `-D warnings` bar; this build is presentation, run by
//! `.github/workflows/pages.yml`, and a failure in it fails the site, not a
//! merge.

mod render;

use std::ffi::OsString;
use std::fs;
use std::io::{self, Write as _};
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, anyhow, bail};

use crate::lint_narrative::TREE;
use crate::spec_trace::workspace_root;

/// Every page the site renders from `docs/`, in reading order.
///
/// It opens on the four background pages, for a reader new to event sourcing:
/// events, event sourcing and CQRS, aggregates, then DCB, each ending on a link
/// to the next and the last on the opening encounter.
///
/// From there the order is the application author's path: run something, see the whole
/// program, set it against the aggregate you arrived with, carry it into your
/// own domain, then the reasoning behind the write, then how to retry that
/// write safely, then how to rebuild what was derived from it, then which store
/// to deploy it on, which is where the edge developer joins, then how to run
/// the store inside a Durable Object, then the adapter author's entry point and
/// how that author passes the conformance suite, and
/// last the upgrade guide, which only a reader already on an older release
/// needs. It is written here rather than derived because an order is a
/// decision, and [`check_coverage`] is what keeps the list from falling behind
/// the tree.
const READING_ORDER: &[&str] = &[
    "event-driven-systems.md",
    "event-sourcing-and-cqrs.md",
    "aggregates.md",
    "what-is-dcb.md",
    "first-encounter.md",
    "read-the-worked-example.md",
    "boundary-not-aggregate.md",
    "carry-your-invariant.md",
    "append-conditions.md",
    "retry-a-command.md",
    "rebuild-a-read-model.md",
    "choose-a-store.md",
    "run-on-a-durable-object.md",
    "adapter-reading-order.md",
    "pass-conformance-suite.md",
    "upgrading-to-0-4.md",
];

/// Pages under `docs/` the site deliberately does not render, each with why.
const OFF_SITE: &[(&str, &str)] = &[(
    "text-fences.md",
    "a fixture for the gate's fence walk, whose `text` fence is deliberately \
     false about the library — it teaches the gate, not a reader",
)];

/// The tree's index. The site builds its own from the pages' declared needs.
const INDEX: &str = "README.md";

/// Programs the landing page shows: the page, which Rust fence on it (0-based),
/// and the name the template loads it by. Each must be followed by a `text`
/// fence holding what it prints.
const SNIPPETS: &[(&str, usize, &str)] = &[("first-encounter.md", 2, "refusal")];

/// What `cargo xtask site` was asked for.
struct Options {
    /// `zola serve` on a local port instead of a one-shot build.
    serve: bool,
    /// Build the rustdoc API section. Off with `--no-api`, for iterating on
    /// templates without a nightly workspace doc build each time.
    api: bool,
}

impl Options {
    /// Reads the flags after `site`. They arrive as `OsString`, from
    /// `std::env::args_os`, because `std::env::args` panics on an argument that
    /// is not Unicode; here that is an error naming the argument.
    fn parse(args: impl IntoIterator<Item = OsString>) -> Result<Self> {
        let mut options = Self {
            serve: false,
            api: true,
        };
        for arg in args {
            let arg = arg
                .into_string()
                .map_err(|arg| anyhow!("an argument to site is not Unicode: {}", arg.display()))?;
            match arg.as_str() {
                "--serve" => options.serve = true,
                "--no-api" => options.api = false,
                other => bail!("unknown flag for site: {other}"),
            }
        }
        Ok(options)
    }
}

/// Assembles and builds the site, or serves it with `--serve`.
///
/// # Errors
///
/// When a `docs/` page is neither on the site nor excluded from it, when a page
/// cannot be rendered, when the nightly rustdoc build fails, or when `zola`
/// refuses the result — including a guide link that no longer lands.
pub(crate) fn run(args: impl IntoIterator<Item = OsString>) -> Result<()> {
    let options = Options::parse(args)?;
    let root = workspace_root()?;
    let out = root.join("target").join("site");
    let src = out.join("src");

    if src.exists() {
        fs::remove_dir_all(&src).with_context(|| format!("failed to clear {}", src.display()))?;
    }
    copy_tree(&root.join("site"), &src)?;
    copy_brand(
        &root.join("assets").join("brand"),
        &src.join("static").join("brand"),
    )?;
    write_guide(&root.join(TREE), &src.join("content").join("guide"))?;
    write_data(&root, &src.join("data"))?;
    if options.api {
        build_api(&root, &src.join("static").join("api"))?;
    }

    say(&format!("assembled {}", src.display()))?;
    let mut zola = Command::new("zola");
    zola.arg("--root").arg(&src);
    if options.serve {
        zola.arg("serve");
    } else {
        zola.arg("build")
            .arg("--output-dir")
            .arg(out.join("public"))
            .arg("--force");
    }
    let status = zola
        .status()
        .context("failed to launch `zola`; install it from https://www.getzola.org")?;
    if !status.success() {
        bail!("zola failed with {status}");
    }
    Ok(())
}

/// Every page name in `files` is in [`READING_ORDER`] or [`OFF_SITE`], exactly
/// once, and every name those lists hold is a file that exists.
///
/// # Errors
///
/// Names every page that is on neither list, on both, or listed but absent.
fn check_coverage(files: &[String]) -> Result<()> {
    let mut problems = Vec::new();
    for file in files.iter().filter(|file| *file != INDEX) {
        let on = READING_ORDER.contains(&file.as_str());
        let off = OFF_SITE.iter().any(|(name, _)| name == file);
        match (on, off) {
            (false, false) => problems.push(format!(
                "{TREE}/{file} is neither in the site's reading order nor excluded from it; \
                 add it to READING_ORDER or OFF_SITE in xtask/src/site.rs"
            )),
            (true, true) => problems.push(format!(
                "{TREE}/{file} is both in the reading order and excluded"
            )),
            _ => {}
        }
    }
    let listed = READING_ORDER
        .iter()
        .copied()
        .chain(OFF_SITE.iter().map(|(name, _)| *name));
    for name in listed {
        if !files.iter().any(|file| file == name) {
            problems.push(format!(
                "xtask/src/site.rs lists {TREE}/{name}, which does not exist"
            ));
        }
    }
    if problems.is_empty() {
        Ok(())
    } else {
        bail!("{}", problems.join("\n"))
    }
}

/// The Markdown file names directly under `dir`, sorted.
fn markdown_files(dir: &Path) -> Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let entry = entry.with_context(|| format!("failed to walk {}", dir.display()))?;
        let name = entry.file_name();
        let name = name
            .to_str()
            .with_context(|| format!("a file name under {} is not UTF-8", dir.display()))?;
        let markdown = Path::new(name)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"));
        let kind = entry
            .file_type()
            .with_context(|| format!("failed to stat {}", entry.path().display()))?;
        if kind.is_file() && markdown {
            files.push(name.to_owned());
        }
    }
    files.sort();
    Ok(files)
}

/// Renders every page in [`READING_ORDER`] into the guide section.
fn write_guide(tree: &Path, guide: &Path) -> Result<()> {
    check_coverage(&markdown_files(tree)?)?;
    create_dir(guide)?;
    for (index, file) in READING_ORDER.iter().enumerate() {
        let text = read(&tree.join(file))?;
        let page = render::guide_page(file, &text, index + 1, READING_ORDER)?;
        write(&guide.join(file), &page)?;
    }
    Ok(())
}

/// Writes the facts and the landing page's programs the templates load.
///
/// `facts.json` is written by hand: its one value is a version string, and a
/// TOML basic string's escapes are JSON's for every character
/// [`render::toml_string`] emits.
fn write_data(root: &Path, data: &Path) -> Result<()> {
    let manifest = read(&root.join("Cargo.toml"))?;
    let version = render::workspace_version(&manifest)?;
    create_dir(&data.join("snippets"))?;
    write(
        &data.join("facts.json"),
        &format!("{{\"version\": {}}}\n", render::toml_string(version)?),
    )?;
    for (file, index, name) in SNIPPETS {
        let text = read(&root.join(TREE).join(file))?;
        let (code, output) = render::rust_fence(&text, *index)
            .with_context(|| format!("{TREE}/{file}: the landing page's `{name}` program"))?;
        let output = output.with_context(|| {
            format!("{TREE}/{file}: the `{name}` program has no `text` fence after it")
        })?;
        write(&data.join("snippets").join(format!("{name}.rs")), &code)?;
        write(&data.join("snippets").join(format!("{name}.out")), &output)?;
    }
    Ok(())
}

/// The nightly workspace rustdoc build with scraped examples, copied to `api`.
///
/// `happenstance-cloudflare` is left out because it builds only for `wasm32`;
/// the reference page links docs.rs for it. `xtask` is tooling, not API. A
/// separate target directory keeps this build's nightly artefacts out of the
/// stable gate's fingerprints. (`happenstance-ladybug` is retired and outside
/// the workspace, ADR-0078, so `--workspace` no longer reaches it.)
fn build_api(root: &Path, api: &Path) -> Result<()> {
    let target = root.join("target").join("site-doc");
    say("building the API with scraped examples (nightly)")?;
    let status = Command::new("cargo")
        .args([
            "+nightly",
            "doc",
            "-Zunstable-options",
            "-Zrustdoc-scrape-examples",
            "--locked",
            "--workspace",
            "--exclude",
            "happenstance-cloudflare",
            "--exclude",
            "xtask",
            "--all-features",
            "--no-deps",
            "--target-dir",
        ])
        .arg(&target)
        .env("RUSTDOCFLAGS", "--cfg docsrs")
        .current_dir(root)
        .status()
        .context("failed to launch `cargo +nightly doc`")?;
    if !status.success() {
        bail!("the nightly API build failed with {status}");
    }
    copy_tree(&target.join("doc"), api)
}

/// Copies the SVG artwork from `assets/brand/` — not the PNG, which is a raster
/// export the site has no use for.
fn copy_brand(from: &Path, to: &Path) -> Result<()> {
    create_dir(to)?;
    for entry in fs::read_dir(from).with_context(|| format!("failed to read {}", from.display()))? {
        let path = entry
            .with_context(|| format!("failed to walk {}", from.display()))?
            .path();
        if path.extension().is_some_and(|ext| ext == "svg") {
            let name = path.file_name().context("a directory entry has a name")?;
            fs::copy(&path, to.join(name))
                .with_context(|| format!("failed to copy {}", path.display()))?;
        }
    }
    Ok(())
}

/// Copies a directory tree, creating `to`.
fn copy_tree(from: &Path, to: &Path) -> Result<()> {
    create_dir(to)?;
    for entry in fs::read_dir(from).with_context(|| format!("failed to read {}", from.display()))? {
        let entry = entry.with_context(|| format!("failed to walk {}", from.display()))?;
        let target = to.join(entry.file_name());
        let kind = entry
            .file_type()
            .with_context(|| format!("failed to stat {}", entry.path().display()))?;
        if kind.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target)
                .with_context(|| format!("failed to copy {}", entry.path().display()))?;
        }
    }
    Ok(())
}

fn create_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path).with_context(|| format!("failed to create {}", path.display()))
}

fn read(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))
}

fn write(path: &Path, text: &str) -> Result<()> {
    fs::write(path, text).with_context(|| format!("failed to write {}", path.display()))
}

/// One progress line, through a locked handle so a closed pipe is an error
/// rather than a panic.
fn say(line: &str) -> Result<()> {
    writeln!(io::stdout().lock(), "{line}").context("failed to write progress")
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use std::ffi::OsString;

    use super::{INDEX, OFF_SITE, Options, READING_ORDER, check_coverage, markdown_files};
    use crate::lint_narrative::TREE;
    use crate::spec_trace::workspace_root;

    fn names(files: &[&str]) -> Vec<String> {
        files.iter().map(|file| (*file).to_owned()).collect()
    }

    fn every_listed() -> Vec<&'static str> {
        READING_ORDER
            .iter()
            .copied()
            .chain(OFF_SITE.iter().map(|(name, _)| *name))
            .chain([INDEX])
            .collect()
    }

    #[test]
    fn a_page_on_neither_list_fails_by_name() {
        let mut files = every_listed();
        files.push("new-page.md");
        let message = check_coverage(&names(&files))
            .err()
            .map(|err| format!("{err:#}"))
            .unwrap_or_default();
        assert!(message.contains("docs/new-page.md is neither"), "{message}");
    }

    #[test]
    fn a_listed_page_that_is_gone_fails_by_name() {
        let files: Vec<&str> = every_listed()
            .into_iter()
            .filter(|file| *file != "append-conditions.md")
            .collect();
        let message = check_coverage(&names(&files))
            .err()
            .map(|err| format!("{err:#}"))
            .unwrap_or_default();
        assert!(
            message.contains("lists docs/append-conditions.md, which does not exist"),
            "{message}"
        );
    }

    #[test]
    fn flags_parse_and_an_unknown_one_is_named() -> Result<()> {
        let options = Options::parse([OsString::from("--serve"), OsString::from("--no-api")])?;
        assert!(options.serve && !options.api);
        let message = Options::parse([OsString::from("--publish")])
            .err()
            .map(|err| format!("{err:#}"))
            .unwrap_or_default();
        assert!(message.contains("--publish"), "{message}");
        Ok(())
    }

    /// `std::env::args` panics on an argument that is not Unicode; the flags
    /// are read from `args_os` so that is an error instead.
    #[test]
    fn an_argument_that_is_not_unicode_is_refused_not_panicked_on() {
        #[cfg(windows)]
        let bad = {
            use std::os::windows::ffi::OsStringExt as _;
            OsString::from_wide(&[0xD800])
        };
        #[cfg(unix)]
        let bad = {
            use std::os::unix::ffi::OsStringExt as _;
            OsString::from_vec(vec![0xFF])
        };
        let message = Options::parse([bad])
            .err()
            .map(|err| format!("{err:#}"))
            .unwrap_or_default();
        assert!(message.contains("not Unicode"), "{message}");
    }

    #[test]
    fn the_real_tree_is_covered() -> Result<()> {
        let files = markdown_files(&workspace_root()?.join(TREE))?;
        check_coverage(&files)
    }

    /// The host every absolute link into the documentation site is spelled with.
    const SITE_HOST: &str = "wet-ink-corporation.github.io";

    /// The surfaces outside the site that link into it: the front doors a reader
    /// arrives through from crates.io, docs.rs and GitHub.
    const SITE_LINKERS: &[(&str, &str)] = &[
        ("Cargo.toml", include_str!("../../Cargo.toml")),
        ("README.md", include_str!("../../README.md")),
        (
            "crates/happenstance/src/lib.rs",
            include_str!("../../crates/happenstance/src/lib.rs"),
        ),
        (
            "crates/happenstance/README.md",
            include_str!("../../crates/happenstance/README.md"),
        ),
        (
            "crates/happenstance-core/README.md",
            include_str!("../../crates/happenstance-core/README.md"),
        ),
        (
            "crates/happenstance-testkit/README.md",
            include_str!("../../crates/happenstance-testkit/README.md"),
        ),
        (
            "crates/happenstance-sqlite/README.md",
            include_str!("../../crates/happenstance-sqlite/README.md"),
        ),
        (
            "crates/happenstance-postgres/README.md",
            include_str!("../../crates/happenstance-postgres/README.md"),
        ),
        (
            "crates/happenstance-neon/README.md",
            include_str!("../../crates/happenstance-neon/README.md"),
        ),
        (
            "crates/happenstance-cloudflare/README.md",
            include_str!("../../crates/happenstance-cloudflare/README.md"),
        ),
    ];

    /// `base_url` as `site/config.toml` declares it.
    fn base_url() -> &'static str {
        include_str!("../../site/config.toml")
            .lines()
            .find_map(|line| line.strip_prefix("base_url = \""))
            .and_then(|rest| rest.strip_suffix('"'))
            .expect("site/config.toml declares base_url on one line")
    }

    /// Every link into the site on `text` that the site would not serve: one off
    /// `base`, or one naming a guide page `pages` does not render.
    fn site_link_problems(surface: &str, text: &str, base: &str, pages: &[&str]) -> Vec<String> {
        let mut found = Vec::new();
        for (at, _) in text.match_indices(SITE_HOST) {
            let start = text[..at].rfind("https://").unwrap_or(at);
            let url: String = text[start..]
                .chars()
                .take_while(|c| !c.is_whitespace() && !matches!(c, ')' | '"' | '>' | '<'))
                .collect();
            let Some(path) = url.strip_prefix(base) else {
                found.push(format!("{surface}: {url} is not under base_url {base}"));
                continue;
            };
            let served = match path.trim_matches('/') {
                "" | "guide" => true,
                rest => rest
                    .strip_prefix("guide/")
                    .is_some_and(|slug| pages.contains(&format!("{slug}.md").as_str())),
            };
            if !served {
                found.push(format!("{surface}: {url} names no page the site renders"));
            }
        }
        found
    }

    /// The guard the front door's absolute links rest on: each one is under the
    /// site's `base_url`, and each guide link names a page in `READING_ORDER`.
    #[test]
    fn every_link_into_the_site_names_a_page_it_renders() {
        let found: Vec<String> = SITE_LINKERS
            .iter()
            .flat_map(|(surface, text)| {
                site_link_problems(surface, text, base_url(), READING_ORDER)
            })
            .collect();
        assert!(found.is_empty(), "{found:?}");
        let linked = SITE_LINKERS
            .iter()
            .filter(|(_, text)| text.contains(SITE_HOST))
            .count();
        assert_eq!(
            linked,
            SITE_LINKERS.len(),
            "a front door stopped linking the site"
        );
    }

    #[test]
    fn a_site_link_off_the_base_or_to_an_unrendered_page_is_rejected() {
        let base = "https://wet-ink-corporation.github.io/happenstance";
        let pages = ["first-encounter.md"];
        let good = "[a](https://wet-ink-corporation.github.io/happenstance/guide/first-encounter/)";
        assert!(site_link_problems("good", good, base, &pages).is_empty());

        let off_base = "[a](https://wet-ink-corporation.github.io/redkiln/)";
        let gone = "[a](https://wet-ink-corporation.github.io/happenstance/guide/retired/)";
        for (wrong, reason) in [(off_base, "is not under base_url"), (gone, "names no page")] {
            let found = site_link_problems("wrong", wrong, base, &pages);
            assert_eq!(found.len(), 1, "{wrong}: {found:?}");
            assert!(found[0].contains(reason), "{wrong}: {found:?}");
        }
    }
}
