//! The rendered codec surface, read off the crate's own source.
//!
//! An unstyled render — bare `pub use`s under a surviving roadmap bullet —
//! satisfies every compile assertion in this crate and fails every one below.
//! No compiler runs here; these are file reads, which is what makes them cheap
//! enough to be unconditional.

use std::path::{Path, PathBuf};

fn src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn read(name: &str) -> String {
    std::fs::read_to_string(src().join(name)).expect("the crate's own source is readable")
}

/// The `//!` body of the crate root, line by line.
fn module_doc() -> Vec<String> {
    read("lib.rs")
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            trimmed
                .strip_prefix("//!")
                .map(|rest| rest.strip_prefix(' ').unwrap_or(rest).to_owned())
        })
        .collect()
}

fn position_of(doc: &[String], needle: &str) -> usize {
    doc.iter()
        .position(|line| line.contains(needle))
        .unwrap_or_else(|| panic!("the crate root does not render `{needle}`"))
}

// ---------------------------------------------------------------------------
// AC-009 — the roadmap bullet became the real thing, in place
// ---------------------------------------------------------------------------

#[test]
fn crate_root_renders_the_codec_surface() {
    let doc = module_doc();

    // Region 4, and the `Codec` bullet is still its first entry: the vocabulary
    // is rewritten in place, not restructured around the new item.
    let vocabulary = position_of(&doc, "# The vocabulary");
    let bullets: Vec<(usize, &String)> = doc
        .iter()
        .enumerate()
        .filter(|(at, line)| *at > vocabulary && line.starts_with("* "))
        .collect();
    let names: Vec<&str> = bullets
        .iter()
        .filter_map(|(_, line)| {
            [
                "`Codec`",
                "`DomainEvent`",
                "`DecisionModel`",
                "The command loop",
                "The typed projection runner",
            ]
            .into_iter()
            .find(|name| line.contains(name))
        })
        .collect();
    assert_eq!(
        names,
        [
            "`Codec`",
            "`DomainEvent`",
            "`DecisionModel`",
            "The command loop",
            "The typed projection runner",
        ],
        "the vocabulary bullets moved; the rewrite was not in place"
    );

    // The bullet is a link, and the link is the emphasis.
    let codec_bullet = bullets
        .iter()
        .find(|(_, line)| line.contains("`Codec`"))
        .map(|(at, _)| *at)
        .expect("the vocabulary names `Codec`");
    assert!(
        doc[codec_bullet].contains("(Codec)"),
        "the `Codec` bullet is not an intra-doc link: {}",
        doc[codec_bullet]
    );

    // No roadmap survives for what this story landed. The bullet runs until the
    // next one, and nothing in it may still call the codecs planned.
    let next = bullets
        .iter()
        .map(|(at, _)| *at)
        .find(|at| *at > codec_bullet)
        .unwrap_or(doc.len());
    let region = doc[codec_bullet..next].join(" ").to_lowercase();
    assert!(
        !region.contains("planned"),
        "the `Codec` bullet still reads as a roadmap: {region}"
    );

    // Region 5 exists, is below the vocabulary, and is a table rather than
    // prose — recessive by form as well as by position.
    let features = position_of(&doc, "# Features");
    assert!(
        features > vocabulary,
        "the Features region was hoisted above the vocabulary"
    );
    let adapters = position_of(&doc, "Adapter authors should depend on");
    assert!(
        features < adapters,
        "the Features region displaced the adapter-author pointer from last"
    );

    let table = doc[features..adapters].join("\n");
    assert!(
        table.contains("| ---"),
        "the Features region is prose, not a table: {table}"
    );
    for shipped in shipped_features() {
        assert!(
            table.contains(&format!("`{shipped}`")),
            "the Features table says nothing about `{shipped}`"
        );
    }

    // The mount itself: every codec is reachable from the crate root, beside
    // the glob that must survive.
    let root = read("lib.rs");
    assert!(
        root.contains("pub use happenstance_core::*;"),
        "the contract crate's glob re-export must survive"
    );
    for (feature, item) in [("json", "Json"), ("postcard", "Postcard"), ("cbor", "Cbor")] {
        if !shipped_features().contains(&feature.to_owned()) {
            continue;
        }
        assert!(
            root.contains(&format!("pub use codec::{item};"))
                || root.contains(&format!("{item},"))
                || root.contains(&format!("{item}}}")),
            "`{item}` is not re-exported at the crate root"
        );
    }
}

/// The codec features this build of the manifest actually ships.
fn shipped_features() -> Vec<String> {
    let manifest =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .expect("the crate's own manifest is readable");
    let mut inside = false;
    let mut out = Vec::new();
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            inside = trimmed == "[features]";
            continue;
        }
        if !inside || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((name, _)) = trimmed.split_once('=') {
            let name = name.trim();
            if ["json", "postcard", "cbor"].contains(&name) {
                out.push(name.to_owned());
            }
        }
    }
    assert!(!out.is_empty(), "no codec feature is declared at all");
    out
}

// ---------------------------------------------------------------------------
// AC-010 — a gated item renders its gate
// ---------------------------------------------------------------------------

#[test]
fn every_gated_item_carries_its_badge() {
    let source = read("codec.rs");
    let lines: Vec<&str> = source.lines().collect();

    let mut gated = 0;
    for (at, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("#[cfg(feature = \"") else {
            continue;
        };
        let Some(feature) = rest.split('"').next() else {
            continue;
        };

        // Only an *item* gate renders on a page. A `#[cfg]` on a statement
        // inside a function body has no page of its own to badge.
        let mut attributes = Vec::new();
        let mut cursor = at + 1;
        while let Some(next) = lines.get(cursor) {
            let next = next.trim();
            if next.starts_with("#[") || next.starts_with("///") || next.is_empty() {
                attributes.push(next);
                cursor += 1;
                continue;
            }
            break;
        }
        let declaration = lines
            .get(cursor)
            .map(|line| line.trim())
            .unwrap_or_default();
        if !declaration.starts_with("pub ") && !declaration.starts_with("impl ") {
            continue;
        }
        gated += 1;

        let badge = format!("#[cfg_attr(docsrs, doc(cfg(feature = \"{feature}\")))]");
        assert!(
            attributes.contains(&badge.as_str()),
            "codec.rs:{}: `{feature}` gates an item that renders no badge",
            at + 1
        );
    }
    assert!(gated > 0, "no item in codec.rs is gated at all");
}

#[test]
fn new_identifiers_fit_the_item_table() {
    for name in [
        "Codec",
        "CodecError",
        "Json",
        "Postcard",
        "Cbor",
        "Projection",
        "run_projection",
        "Progressed",
        "ProjectionError",
    ] {
        assert!(
            name.len() <= 24,
            "`{name}` is {} characters; name and summary cannot share a row",
            name.len()
        );
    }
}

// ---------------------------------------------------------------------------
// AC-009 — the fifth roadmap bullet became the real thing, in place
// ---------------------------------------------------------------------------

#[test]
fn crate_root_renders_the_projection_surface() {
    let doc = module_doc();

    // The vocabulary is rewritten **in place**: same five bullets, same order,
    // and the projection runner is still the fifth.
    let vocabulary = position_of(&doc, "# The vocabulary");
    let bullets: Vec<(usize, &String)> = doc
        .iter()
        .enumerate()
        .filter(|(at, line)| *at > vocabulary && line.starts_with("* "))
        .collect();
    let runner = bullets
        .iter()
        .find(|(_, line)| line.contains("The typed projection runner"))
        .map(|(at, _)| *at)
        .expect("the vocabulary still names the projection runner");
    assert_eq!(
        bullets.last().map(|(at, _)| *at),
        Some(runner),
        "the projection bullet is no longer the fifth and last"
    );

    // The bullet is a link, and the link is the emphasis: the bolded lead-in
    // term is the link text, the way the command loop's already is.
    let next = doc.len();
    let region = doc[runner..next]
        .iter()
        .take_while(|line| line.starts_with("* ") || line.starts_with("  "))
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        region.contains("[**The typed projection runner**]"),
        "the projection bullet is not a link: {region}"
    );

    // Reference-style, with the target spelled **conditionally**, because a
    // plain `](run_projection)` is an unresolved link — a *hard* rustdoc error,
    // not a warning — in every build without `unstable-projection`, which is
    // the default one. This is RS-70-2, and it is the whole reason a gated item
    // cannot be linked inline from an ungated page.
    let root = read("lib.rs");
    assert!(
        root.contains(r#"doc = "[projection-runner]: run_projection""#),
        "the projection reference does not resolve to `run_projection` with the \
         feature on"
    );
    assert!(
        root.contains("feature = \"unstable-projection\"")
            && root.contains(r"[projection-runner]: https://docs.rs/happenstance"),
        "the projection reference resolves to nothing when the feature is off"
    );

    // No roadmap survives anywhere on the page: this was the last of the five.
    let page = doc.join("\n").to_lowercase();
    assert!(
        !page.contains("(planned)") && !page.contains("planned, and specified in"),
        "the crate-root page still carries a roadmap"
    );

    // Region 5 gains one recessive row, below the vocabulary, above the
    // adapter-author pointer, which stays last.
    let features = position_of(&doc, "# Features");
    let adapters = position_of(&doc, "Adapter authors should depend on");
    assert!(features > vocabulary && features < adapters);
    let table = doc[features..adapters].join("\n");
    assert!(
        table.contains("`unstable-projection`"),
        "the Features table says nothing about `unstable-projection`: {table}"
    );

    // The mount: every item is re-exported at the root beside the glob, and
    // the glob itself survives.
    assert!(root.contains("pub use happenstance_core::*;"));
    for item in [
        "Projection",
        "run_projection",
        "Progressed",
        "ProjectionError",
    ] {
        assert!(
            root.contains(&format!("{item},")) || root.contains(&format!("{item}}}")),
            "`{item}` is not re-exported at the crate root"
        );
    }
    assert!(
        root.contains("pub use runner::{"),
        "the four items are not re-exported from the crate's own module"
    );

    // The density budget, in the units the design fixed.
    assert!(
        doc.len() <= 130,
        "the crate-root module doc is {} lines; the budget is 130",
        doc.len()
    );
    let mut fenced = false;
    for line in &doc {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        // A line carrying a URL is exempt from the prose budget: a link target
        // cannot be wrapped, and this page already carries three.
        if !fenced && line.contains("http") {
            continue;
        }
        // The rendered column is the whole authored line, `//! ` included.
        let columns = line.chars().count() + if fenced { 0 } else { 4 };
        let budget = if fenced { 72 } else { 80 };
        assert!(
            columns <= budget,
            "a crate-root doc line is {columns} columns; the budget is {budget}: {line}"
        );
    }
}

// ---------------------------------------------------------------------------
// AC-002, AC-006, AC-011, AC-012 — four absences, each of them observable
// ---------------------------------------------------------------------------

/// Every `.rs` file under this crate's `src/` and `tests/`.
fn crate_sources() -> Vec<(PathBuf, String)> {
    fn walk(dir: &Path, out: &mut Vec<(PathBuf, String)>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                let body = std::fs::read_to_string(&path).expect("a source file is readable");
                out.push((path, body));
            }
        }
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut out = Vec::new();
    walk(&root.join("src"), &mut out);
    walk(&root.join("tests"), &mut out);
    assert!(!out.is_empty(), "this crate has sources");
    out
}

#[test]
fn no_second_decode_path() {
    let runner = read("runner.rs");
    assert!(
        runner.contains("decode_event"),
        "the runner does not go through the crate's one decode path"
    );
    for forbidden in ["serde_json::from_slice", "from_slice", "from_str"] {
        assert!(
            !runner.contains(forbidden),
            "the runner grew a second decode path: {forbidden}"
        );
    }
}

#[test]
fn runner_prints_nothing() {
    let runner = read("runner.rs");
    for forbidden in ["println!", "eprintln!", "print!", "eprint!", "\\r"] {
        assert!(
            !runner.contains(forbidden),
            "the runner renders while it works: {forbidden}. Polling renders nothing"
        );
    }
}

#[test]
fn no_contract_name_is_shadowed() {
    for name in [
        "ProjectionId",
        "ProjectionStore",
        "Query",
        "ReadOptions",
        "SequencePosition",
        "Checkpoint",
        "Authority",
    ] {
        for (path, body) in crate_sources() {
            for shape in ["pub struct", "pub enum", "pub trait", "pub type"] {
                assert!(
                    !body.contains(&format!("{shape} {name}")),
                    "{}: `{name}` shadows a contract name that arrives through the glob",
                    path.display()
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// HS-S0031 AC-007 — the flavour is stated where the reader meets it
// ---------------------------------------------------------------------------

/// The `///` block immediately above an item declaration, line by line.
///
/// Returns the doc lines in source order with the `/// ` prefix removed, so
/// both the first sentence and the rendered column can be measured.
fn item_doc(source: &str, declaration: &str) -> Vec<String> {
    let lines: Vec<&str> = source.lines().collect();
    let at = lines
        .iter()
        .position(|line| line.trim_start().starts_with(declaration))
        .unwrap_or_else(|| panic!("no item declared `{declaration}`"));

    let mut doc: Vec<String> = Vec::new();
    for line in lines[..at].iter().rev() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("///") {
            doc.push(rest.strip_prefix(' ').unwrap_or(rest).to_owned());
            continue;
        }
        // Attributes and ordinary comments sit between the doc block and the
        // declaration; anything else ends the block.
        if trimmed.starts_with("#[") || trimmed.starts_with("//") || trimmed.is_empty() {
            continue;
        }
        break;
    }
    doc.reverse();
    assert!(!doc.is_empty(), "`{declaration}` carries no documentation");
    doc
}

/// The first sentence of a doc block: up to the first `. ` or trailing `.`.
fn first_sentence(doc: &[String]) -> String {
    let joined = doc.join(" ");
    match joined.find(". ") {
        Some(at) => joined[..=at].trim().to_owned(),
        None => joined.trim_end().to_owned(),
    }
}

#[test]
fn entry_points_state_their_flavour() {
    // Every generic entry point this crate exposes over a caller's store. The
    // DSL is deliberately absent: `given` builds its own `MemoryEventStore` and
    // takes no store parameter, so it has no flavour to bind.
    let mut entry_points: Vec<(&str, &str)> = vec![
        ("command.rs", "pub async fn commit<"),
        ("command.rs", "pub async fn commit_with<"),
    ];
    if cfg!(feature = "unstable-projection") {
        entry_points.push(("runner.rs", "pub async fn run_projection<"));
    }

    for (file, declaration) in entry_points {
        let source = read(file);
        let doc = item_doc(&source, declaration);
        let page = doc.join(" ");

        // Stated on the item, not in a separate "edge notes" section a reader
        // has to jump to.
        assert!(
            page.contains("EventStore") && !page.contains("SendEventStore"),
            "{file}: `{declaration}` does not say on its own page that it binds \
             `EventStore`, the weaker flavour that accepts both"
        );
        assert!(
            page.contains("wasm32") || page.contains("!Send") || page.contains("edge"),
            "{file}: `{declaration}` names the bound without naming what it \
             buys — a store that is not `Send` at all"
        );

        // Density budget, in `_design.md`'s own units.
        let opening = first_sentence(&doc);
        assert!(
            opening.chars().count() <= 80,
            "{file}: `{declaration}`'s first sentence is {} characters; the \
             budget is 80 and the item table truncates: {opening}",
            opening.chars().count()
        );
        let mut fenced = false;
        for line in &doc {
            if line.trim_start().starts_with("```") {
                fenced = !fenced;
                continue;
            }
            if fenced || line.contains("http") {
                continue;
            }
            // The rendered column is the whole authored line, `/// ` included.
            let columns = line.chars().count() + 4;
            assert!(
                columns <= 80,
                "{file}: a doc line on `{declaration}` is {columns} columns; \
                 the budget is 80: {line}"
            );
        }
    }
}

/// AC-007's second half, stated as an absence: nothing here promises an
/// *executed* edge test. The gate step is a `cargo check`, and claiming more
/// than it checks is the prose-guarantee failure this repository keeps finding.
#[test]
fn no_page_claims_an_executed_edge_test() {
    for (path, body) in crate_sources() {
        for line in body.lines() {
            let trimmed = line.trim_start();
            if !trimmed.starts_with("//") {
                continue;
            }
            let lowered = trimmed.to_lowercase();
            let claims_execution = (lowered.contains("wasm32") || lowered.contains("workers"))
                && (lowered.contains("tests run") || lowered.contains("tested on"));
            assert!(
                !claims_execution,
                "{}: a page claims the typed layer's tests execute on an edge \
                 runtime; the gate compiles, and executing them is HS-P0013's: \
                 {trimmed}",
                path.display()
            );
        }
    }
}

#[test]
fn no_local_projection_store() {
    // Assembled rather than written out, so this assertion does not find
    // itself: the needle it looks for must not be a literal in this file.
    let bare = format!("impl {} for", "ProjectionStore");
    let send = format!("impl Send{} for", "ProjectionStore");
    for (path, body) in crate_sources() {
        assert!(
            !body.contains(&bare) && !body.contains(&send),
            "{}: a local projection store freezes a fixture shape this project \
             does not own (HS-P0010's AC-012)",
            path.display()
        );
    }
}

// ---------------------------------------------------------------------------
// The caller's obligation, stated where the caller stands
// ---------------------------------------------------------------------------

/// The `#`-headed sections of a doc block, each keyed by its heading text.
///
/// Every topic on `run_projection`'s page is a section, and a claim is only
/// worth reading where its qualifications are: "exactly one runner" in one
/// section and "the caller's" three screens away is two half-sentences a
/// reader is asked to join up. Splitting here is what lets an assertion say
/// *together*.
fn doc_sections(doc: &[String]) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for line in doc {
        match line.strip_prefix("# ") {
            Some(heading) => out.push((heading.trim().to_owned(), String::new())),
            None => {
                if let Some((_, body)) = out.last_mut() {
                    body.push_str(line);
                    body.push(' ');
                }
            }
        }
    }
    out
}

/// Two runners on one `(store, ProjectionId)` is the rolling-redeploy shape
/// PS-22's `Rejects:` paragraph already names, and PS-22 guards only the
/// **backwards** half — the stale runner dragging the checkpoint down. Two
/// runners both advancing forwards never trip `CheckpointRegression`; they
/// interleave, and the monotonic checkpoint is the evidence that nothing went
/// wrong. Nothing in this crate guards that, so the page a caller reads has to
/// say whose obligation it is, and say it in one place.
///
/// A source read, deliberately: the *mechanism* — a lease, a lock, an
/// ownership token — is a real design question and belongs with the
/// projection-store freeze. The sentence does not.
#[test]
fn run_projection_states_the_single_writer_obligation() {
    if !cfg!(feature = "unstable-projection") {
        return;
    }

    let source = read("runner.rs");
    let doc = item_doc(&source, "pub async fn run_projection<");
    let sections = doc_sections(&doc);

    // One section carries the whole claim. The page already says "the caller's"
    // about spawning and already names `ProjectionId` under rebuilding, so an
    // assertion over the joined page would be satisfied by what is there now.
    let (heading, body) = sections
        .iter()
        .find(|(_, body)| body.to_lowercase().contains("exactly one"))
        .unwrap_or_else(|| {
            panic!(
                "no section of `run_projection`'s page says that exactly one \
                 runner may drive a projection at a time; the sections are {:?}",
                sections.iter().map(|(h, _)| h).collect::<Vec<_>>()
            )
        });
    let lowered = body.to_lowercase();

    assert!(
        lowered.contains("runner"),
        "`{heading}` says \"exactly one\" of something other than a runner: {body}"
    );
    assert!(
        body.contains("ProjectionId") && lowered.contains("caller"),
        "`{heading}` does not name the obligation as the caller's, per \
         `(store, ProjectionId)`: {body}"
    );
    // The load-bearing half. `CheckpointRegression` is the guard a reader
    // already knows about, and it catches only the stale runner going
    // backwards; two runners both advancing forwards never trip it. A page
    // that states the obligation without naming that gap leaves the reader
    // believing the guard they already have covers this.
    assert!(
        body.contains("CheckpointRegression") && lowered.contains("forward"),
        "`{heading}` states the obligation without saying that \
         `CheckpointRegression` guards only the backwards half: {body}"
    );
}
