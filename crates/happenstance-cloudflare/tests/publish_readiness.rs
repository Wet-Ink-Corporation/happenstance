//! What this crate says about itself, read back and checked.
//!
//! # Why a test target rather than a gate step
//!
//! `cargo xtask package-check` asserts three *filenames* are inside the packaged
//! artifact and reads not one word of their contents
//! (`xtask/src/package.rs:110-161`). That leaves the whole of this crate's front
//! matter — the manifest `description` crates.io prints beside the name, the
//! README a registry page renders, the rustdoc header docs.rs renders above the
//! fold — checked by nothing at all.
//!
//! The wrong implementation that gap admits is named in this story's discover
//! note and it is not hypothetical: **a crate that is publishable and lies.**
//! `package-check` green, `cargo xtask ci` green, and the first two hundred
//! characters a stranger reads are a status header left over from a skeleton, or
//! a flat *"passes the DCB conformance suite"* on an adapter whose concurrency
//! family is never invoked and whose positions are bounded by 2^53 rather than
//! 2^64. Every mechanical check in the tree is satisfied by that crate.
//!
//! So the assertions here are about *content*, and they are deliberately tied to
//! artefacts that move on their own rather than to strings this file also
//! writes: the fixture's own [`Capability`](happenstance_testkit::Capability)
//! constants and its declared ceilings, the manifest keys Cargo reads, and the
//! repository's licence texts. A README that goes stale against any of them
//! fails here.
//!
//! # This target is empty on `wasm32`, and that is correct
//!
//! It reads files. `#![cfg(not(target_arch = "wasm32"))]` keeps it off a target
//! with no filesystem to read them from, which is the mirror of the condition
//! `durable_object_conformance.rs` carries for the opposite reason. Nothing here
//! needs a JavaScript heap, so it runs in an ordinary
//! `cargo test -p happenstance-cloudflare` with no wasm toolchain installed.
//!
//! # What it does not check
//!
//! It cannot read a true sentence that is narrower than it sounds, and it cannot
//! tell a well-composed paragraph from a list of keywords. Both are review
//! obligations, stated as such in this story's spec (*Tests and CI*, **Review
//! obligations**) rather than pretended away here.

#![cfg(not(target_arch = "wasm32"))]

use std::fs;
use std::path::PathBuf;

use happenstance_testkit::Fixture;

// The fixture is the *source* the conformance paragraph is checked against, so
// this target links it rather than restating its numbers. `dead_code` is allowed
// because an integration test is its own compilation unit: everything in
// `support` that this target does not name is unreachable *here* and reachable
// from `fixture_contract.rs`, which is the file that drives it.
#[allow(dead_code)]
mod support;

use support::CloudflareFixture;

/// The sentence the reserved `0.0.0` placeholder already promises.
///
/// Spelled once, from `xtask/src/reserve.rs:93`. The manifest and the
/// reservation describe one thing or the evaluator meets two.
const RESERVED_DESCRIPTION: &str =
    "Cloudflare Durable Object event store adapter for happenstance.";

/// The claim this criterion exists to reject, lower-cased for the search.
///
/// A README that *names* it in order to refuse it is the right implementation,
/// so occurrences are checked for a negation rather than merely counted.
const FLAT_CLAIM: &str = "passes the dcb conformance suite";

/// The claim the manifest's `cargo deny` paragraph may not make while
/// `deny.toml` exempts `worker`, spelled the way the manifest spelled it.
///
/// The wrong implementation this names is not hypothetical either: it is what
/// the paragraph said on 2026-09-03, three days after `deny.toml` carried the
/// ratified entry and `cargo deny check bans` began reporting `bans ok`.
const RED_BANS_CLAIM: &str = "`cargo deny check bans` is **red**";

/// The routing the same paragraph may not carry once the question is settled.
///
/// An escalation is a present-tense statement that something is *open*. Pointing
/// a reader at a decision that has already been taken elsewhere is how a
/// contributor re-litigates it.
const ESCALATION: &str = "escalated to";

/// The phrase the README uses to say the run declines nothing.
///
/// Held in both directions below: present while the fixture supports every
/// capability, and forbidden the moment one is declined.
const NO_SKIP_CLAIM: &str = "no `SKIP` line";

/// This crate's own directory — the package boundary every check here is about.
fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// A file that travels inside the package.
///
/// Panics rather than returning an error, and the message is the assertion: a
/// missing file here is the defect, not a test-harness problem.
fn packaged(relative: &str) -> String {
    let path = crate_dir().join(relative);
    fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "`{relative}` must sit inside `crates/happenstance-cloudflare/`, because Cargo \
             packages only what lives inside the package directory and will not follow a path \
             outside it: {err}"
        )
    })
}

/// A file that belongs to the repository rather than to the package.
///
/// `None` once this crate has been unpacked from its `.crate` artifact, which is
/// the whole distinction these checks are about — so the comparisons that use it
/// are strongest exactly where they run, in the tree the gate runs in.
fn repository(relative: &str) -> Option<String> {
    let path = crate_dir().join("..").join("..").join(relative);
    if path.is_file() {
        Some(
            fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("reading {}: {err}", path.display())),
        )
    } else {
        None
    }
}

/// The manifest, with comment lines discarded.
///
/// The comments are dropped because this crate's manifest carries several
/// paragraphs of prose that quote the very keys being looked for — a scan that
/// read them would answer questions about the commentary.
fn manifest_code() -> String {
    packaged("Cargo.toml")
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The manifest's prose, split into paragraphs.
///
/// The inverse of [`manifest_code`], and it exists because this crate's manifest
/// records *decisions* in comments — several paragraphs of them — and a record
/// that has gone stale against the artefact it describes is exactly as
/// misleading as the stale README the checks above reject. A run of `#` lines is
/// a block; a bare `#` separates two paragraphs inside it, which is the
/// convention the manifest already writes.
fn manifest_comment_paragraphs() -> Vec<String> {
    let mut paragraphs = Vec::new();
    let mut current: Vec<String> = Vec::new();

    for line in packaged("Cargo.toml").lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') && trimmed != "#" {
            current.push(trimmed.to_owned());
        } else if !current.is_empty() {
            paragraphs.push(current.join("\n"));
            current.clear();
        }
    }
    if !current.is_empty() {
        paragraphs.push(current.join("\n"));
    }

    paragraphs
}

/// `deny.toml`'s `deny` entry for `async-trait`, from the crate name to the
/// brace that closes it.
///
/// Text rather than parsed TOML on purpose: this target links no TOML parser,
/// and the entry is read for two facts a substring answers — which wrappers it
/// names, and which decisions its `reason` cites.
fn async_trait_deny_entry(deny: &str) -> Option<&str> {
    let at = deny.find(r#"crate = "async-trait""#)?;
    let rest = &deny[at..];
    let end = rest.find('}')?;
    Some(&rest[..=end])
}

/// Every `ADR-<four digits>` in `text`, in first-appearance order, deduplicated.
fn adr_ids(text: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    let mut rest = text;

    while let Some(at) = rest.find("ADR-") {
        let after = &rest[at + "ADR-".len()..];
        let digits: String = after.chars().take_while(char::is_ascii_digit).collect();
        if digits.len() == 4 {
            let id = format!("ADR-{digits}");
            if !found.contains(&id) {
                found.push(id);
            }
        }
        rest = after;
    }

    found
}

/// The `[package]` table's value for `key`, unquoted.
fn package_key(key: &str) -> Option<String> {
    let mut in_package = false;
    for line in manifest_code().lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_package = line == "[package]";
            continue;
        }
        if !in_package {
            continue;
        }
        if let Some(value) = line.strip_prefix(key)
            && let Some(value) = value.trim_start().strip_prefix('=')
        {
            return Some(value.trim().trim_matches('"').to_owned());
        }
    }
    None
}

/// Every fenced block in `markdown` whose info string names Rust.
///
/// An empty info string counts: rustdoc treats an unlabelled block as Rust and
/// compiles it, so a scan that skipped one would miss the block most likely to
/// have been written without thinking about it.
fn rust_blocks(markdown: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut current: Option<String> = None;

    for line in markdown.lines() {
        if let Some(info) = line.trim_start().strip_prefix("```") {
            if let Some(block) = current.take() {
                blocks.push(block);
            } else {
                let info = info.trim();
                let rustish = info.is_empty()
                    || info.starts_with("rust")
                    || info.starts_with("no_run")
                    || info.starts_with("ignore")
                    || info.starts_with("compile_fail");
                if rustish {
                    current = Some(String::new());
                }
            }
            continue;
        }
        if let Some(block) = current.as_mut() {
            block.push_str(line);
            block.push('\n');
        }
    }

    blocks
}

/// `line` with every `[<digits>]` index collapsed, so an index is not read as a
/// literal position.
///
/// `events[0].position` is a binding read, not a literal assumption; `[1, 2, 3]`
/// is the thing this file exists to reject.
fn without_indexes(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;

    while let Some(open) = rest.find('[') {
        out.push_str(&rest[..=open]);
        let after = &rest[open + 1..];
        let digits: String = after.chars().take_while(char::is_ascii_digit).collect();
        if !digits.is_empty() && after[digits.len()..].starts_with(']') {
            rest = &after[digits.len()..];
        } else {
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// `n`, written the way a document writes it: `1048576` → `1,048,576`.
fn grouped(n: usize) -> String {
    let digits = n.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// AC-001. Both licence texts are inside the crate directory, and they are the
/// repository's own.
///
/// `license = "MIT OR Apache-2.0"` is a choice the consumer makes, and a choice
/// needs both texts to be makeable. Cargo does not warn when the metadata
/// promises a licence the artifact does not carry — D11 shipped exactly that —
/// so the check is that the *bytes* are here, not that a pointer to them is.
#[test]
fn both_licence_texts_travel_inside_the_crate_directory() {
    let mit = packaged("LICENSE-MIT");
    let apache = packaged("LICENSE-APACHE");

    // A "see the repository root" pointer file satisfies every filename check
    // there is. These do not let it.
    assert!(
        mit.contains("Permission is hereby granted, free of charge")
            && mit.contains("WITHOUT WARRANTY OF ANY KIND"),
        "LICENSE-MIT must be the licence text itself, not a pointer to it"
    );
    assert!(
        apache.contains("Apache License")
            && apache.contains("Version 2.0, January 2004")
            && apache.contains("APPENDIX: How to apply the Apache License"),
        "LICENSE-APACHE must be the licence text itself, not a pointer to it"
    );

    for (name, packaged_text) in [("LICENSE-MIT", &mit), ("LICENSE-APACHE", &apache)] {
        if let Some(root) = repository(name) {
            assert_eq!(
                packaged_text.as_str(),
                root.as_str(),
                "{name} inside the crate directory has drifted from the repository root's copy; \
                 the two are copies on purpose (NF-001) and a divergence means one of them was \
                 edited"
            );
        }
    }
}

/// AC-002. The README belongs to this crate, sits beside the manifest, and is
/// named by it.
///
/// Cargo would auto-discover it. The key is stated anyway, for the reason
/// `crates/happenstance-core/Cargo.toml:12-15` gives: what crates.io renders is
/// the first impression the crate gets, and a silent default is a poor thing to
/// rely on for it.
#[test]
fn the_readme_is_beside_the_manifest_and_named_by_it() {
    let readme = packaged("README.md");
    assert!(
        readme.starts_with("# happenstance-cloudflare"),
        "the README must be this crate's own, not the repository's — the repository README would \
         not resolve once the crate is unpacked from its artifact"
    );
    assert_eq!(
        package_key("readme").as_deref(),
        Some("README.md"),
        "`readme = \"README.md\"` must be stated rather than left to auto-discovery"
    );
}

/// AC-003. The manifest no longer declines publication.
///
/// The other half of this criterion — `\"happenstance-cloudflare\"` in
/// `PUBLISHABLE` — is not restated here, because `xtask/src/package.rs`'s
/// `reconcile` already fails the gate in both directions and naming it twice
/// would be the decorative test this repository forbids. What a test *can* see
/// is that this side of the pair moved.
#[test]
fn the_manifest_no_longer_declines_publication() {
    assert!(
        !manifest_code().contains("publish"),
        "`publish = false` must be gone from the manifest; the paired half is the `PUBLISHABLE` \
         entry in xtask/src/package.rs, which `reconcile` holds to this one"
    );
}

/// AC-005. The description is the sentence the reservation already promises.
///
/// Two artefacts describe this crate to a stranger — the placeholder holding the
/// name and the crate itself — and they describe one thing or the evaluator
/// meets a contradiction before they meet the code.
#[test]
fn the_description_is_the_sentence_the_reservation_promises() {
    let description = package_key("description").expect("the manifest states a description");

    assert!(
        !description.contains("Not yet implemented"),
        "the description crates.io prints beside the name still says the crate is unimplemented"
    );
    assert_eq!(
        description, RESERVED_DESCRIPTION,
        "the manifest description must match the `RESERVABLE` row at xtask/src/reserve.rs:93"
    );

    if let Some(reserve) = repository("xtask/src/reserve.rs") {
        assert!(
            reserve.contains(RESERVED_DESCRIPTION),
            "xtask/src/reserve.rs no longer carries this description; the reservation and the \
             crate have drifted apart"
        );
    }
}

/// AC-006. The front page opens on what the crate is, and declares how docs.rs
/// should build it.
///
/// `# Status: not implemented` occupied the first two hundred characters docs.rs
/// renders. Nothing false may sit in front of the thing the reader came for, and
/// a page that fails to build renders an error where the front page should be.
#[test]
fn the_front_page_says_what_the_crate_is_and_declares_its_docs_rs_build() {
    let lib = packaged("src/lib.rs");

    assert!(
        !lib.contains("Status: not implemented"),
        "the crate's front page still opens with the skeleton's status header"
    );

    // `skip_while` first: the crate root opens on ordinary `//` comments — the
    // `cfg(doctest)` README include and its reasoning — and the *documentation*
    // starts at the first `//!`. That is the line docs.rs renders above the fold.
    let opening: String = lib
        .lines()
        .skip_while(|line| !line.starts_with("//!"))
        .take_while(|line| line.starts_with("//!"))
        .take(2)
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        opening.contains("Cloudflare Durable Object") && opening.contains("happenstance"),
        "the first thing the front page says must be what the crate is: {opening}"
    );

    let manifest = manifest_code();
    assert!(
        manifest.contains("[package.metadata.docs.rs]"),
        "declare `[package.metadata.docs.rs]` so the registry page is built the way this crate \
         is built, following crates/happenstance-core/Cargo.toml"
    );
}

/// AC-007. The README's example is compiled, and it is compiled from inside the
/// package.
///
/// A README example that does not compile is worse than no example: it is the
/// first thing a reader tries. `include_str!` resolves against the *source*
/// directory, so a path outside the package resolves in this tree and vanishes
/// once the crate is unpacked — which is why the path is asserted, not merely
/// the attribute.
#[test]
fn the_readme_example_is_compiled_from_inside_the_package() {
    let lib = packaged("src/lib.rs");

    assert!(
        lib.contains(r#"#![cfg_attr(doctest, doc = include_str!("../README.md"))]"#),
        "the crate root must carry the `cfg(doctest)` README include, which is what makes \
         `cargo test -p happenstance-cloudflare --doc` compile every block in it"
    );
    assert!(
        !lib.contains(r#"include_str!("../../README.md")"#),
        "the repository README is outside the package and would not resolve once published"
    );
    assert!(
        !rust_blocks(&packaged("README.md")).is_empty(),
        "the README must carry at least one Rust block for that attribute to compile"
    );
}

/// AC-008. No example in the README asserts on a literal position value.
///
/// The specification permits gaps and a conformant adapter may leave them, so a
/// literal `[1, 2, 3]` on the most-read page this crate has would teach every
/// adapter author who copies it the one thing this repository forbids.
///
/// **Its limit, stated rather than implied.** It rejects an integer array
/// literal and a bare integer on a line that mentions a position; it cannot see
/// a wrong assumption written some third way. The remainder is the review
/// obligation this story's spec names.
#[test]
fn no_example_in_the_readme_asserts_on_a_literal_position() {
    for block in rust_blocks(&packaged("README.md")) {
        for line in block.lines() {
            let code = line.split("//").next().unwrap_or(line);

            let array_of_integers = code.split('[').skip(1).any(|after| {
                after.split(']').next().is_some_and(|inner| {
                    inner.contains(',')
                        && !inner.trim().is_empty()
                        && inner
                            .split(',')
                            .all(|item| item.trim().parse::<u64>().is_ok())
                })
            });
            assert!(
                !array_of_integers,
                "a literal array of positions in the README example: {line}"
            );

            let mentions_position = code.to_lowercase().contains("position");
            let carries_a_literal = without_indexes(code).chars().any(|c| c.is_ascii_digit());
            assert!(
                !(mentions_position && carries_a_literal),
                "a literal position value in the README example — compare against what the store \
                 assigned instead: {line}"
            );
        }
    }
}

/// AC-009. The conformance claim carries the qualifiers the run itself produces.
///
/// Not *"passes the DCB conformance suite"* flat. Every qualifier below is
/// checked against something that moves on its own — the fixture's `Capability`
/// constants and its declared ceilings — so a README that goes stale against the
/// adapter fails here rather than being believed.
#[test]
fn the_conformance_claim_carries_every_qualifier_the_run_produces() {
    let readme = packaged("README.md");

    // The flat claim is the wrong implementation this criterion exists to
    // reject — but a README that *names* it in order to refuse it is the right
    // one, and a bare `contains` could not tell the two apart. So every
    // occurrence must be negated by the words just before it.
    let lower = readme.to_lowercase();
    let mut searched = 0;
    while let Some(offset) = lower[searched..].find(FLAT_CLAIM) {
        let at = searched + offset;
        let preceding = &lower[at.saturating_sub(40)..at];
        assert!(
            preceding.contains("not"),
            "the README makes the flat claim: name what ran, on what, and what was not asked"
        );
        searched = at + FLAT_CLAIM.len();
    }

    for needle in [
        // What ran, and the macro that ran it.
        "event_store_conformance!",
        // Where it ran. Not `workerd`, and the README says so in the run's own
        // terms rather than leaving the reader to assume the platform.
        "wasm32-unknown-unknown",
        "workerd",
        // The documented non-invocation, with the reason that forces it.
        "event_store_concurrency_conformance!",
        "EventStore + Send",
        // The declared store limit a caller can actually hit.
        "2^53",
    ] {
        assert!(
            readme.contains(needle),
            "the conformance section must state `{needle}` — every qualifier here is one the run \
             or the fixture produces, not prose invented for the README"
        );
    }

    let capabilities = [
        (
            "SECOND_HANDLE",
            <CloudflareFixture as Fixture>::SECOND_HANDLE,
        ),
        ("REOPEN", <CloudflareFixture as Fixture>::REOPEN),
        (
            "MID_BATCH_FAULT",
            <CloudflareFixture as Fixture>::MID_BATCH_FAULT,
        ),
    ];

    for (name, capability) in capabilities {
        match capability.reason() {
            // A declined capability is reported in the fixture's own words, so
            // the README's sentence can be diffed against the run's `SKIP` line.
            Some(reason) => assert!(
                readme.contains(reason),
                "`{name}` is declined and the README does not carry the fixture's stated reason: \
                 {reason}"
            ),
            None => assert!(
                readme.contains(name),
                "`{name}` is supported and the README does not say so; a reader cannot tell a \
                 rule that ran from one that was skipped"
            ),
        }
    }

    let nothing_declined = capabilities
        .iter()
        .all(|(_, capability)| capability.is_supported());
    assert_eq!(
        readme.contains(NO_SKIP_CLAIM),
        nothing_declined,
        "the README's claim about skipped rules and the fixture's capabilities disagree: the \
         fixture declines {} capability(ies)",
        capabilities
            .iter()
            .filter(|(_, capability)| !capability.is_supported())
            .count()
    );

    for (name, ceiling) in [
        (
            "MAX_EVENT_DATA_LEN",
            <CloudflareFixture as Fixture>::MAX_EVENT_DATA_LEN,
        ),
        (
            "MAX_TAGS_PER_EVENT",
            <CloudflareFixture as Fixture>::MAX_TAGS_PER_EVENT,
        ),
        (
            "MAX_EVENTS_PER_BATCH",
            <CloudflareFixture as Fixture>::MAX_EVENTS_PER_BATCH,
        ),
    ] {
        let value = ceiling.unwrap_or_else(|| {
            panic!(
                "this adapter declares every store limit and the README states all three; \
                 `{name}` is now `None`"
            )
        });
        assert!(
            readme.contains(&grouped(value)) || readme.contains(&value.to_string()),
            "the README does not state the ceiling the fixture declares for `{name}`: {value}"
        );
    }
}

/// AC-011. No unimplemented body and no scoped allow survives.
///
/// This story **verifies** rather than causes it: the scoped
/// `#![allow(clippy::todo)]` was documented to leave with the last unimplemented
/// body rather than outlive it, and a survivor means an upstream story did not
/// finish. The needles are split so this file does not match itself.
#[test]
fn no_unimplemented_body_or_scoped_allow_survives() {
    let unimplemented = concat!("todo", "!()");
    let scoped_allow = concat!("allow(clippy::", "todo)");

    let mut sources = Vec::new();
    collect_rust_sources(&crate_dir().join("src"), &mut sources);

    for path in sources {
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("reading {}: {err}", path.display()));
        for (number, line) in source.lines().enumerate() {
            // Documentation *about* the lint is not the lint, and this crate's
            // front page discusses both by name.
            if line.trim_start().starts_with("//") {
                continue;
            }
            assert!(
                !line.contains(unimplemented),
                "an unimplemented body survives at {}:{} — halt rather than deleting it, it means \
                 an upstream story did not finish",
                path.display(),
                number + 1
            );
            assert!(
                !line.contains(scoped_allow),
                "the scoped allow survives at {}:{}, and it was documented to leave with the last \
                 unimplemented body",
                path.display(),
                number + 1
            );
        }
    }
}

/// N-1. The manifest's `cargo deny check bans` paragraph says what `deny.toml`
/// says.
///
/// This crate's manifest is where the price of taking the real `worker` crate is
/// recorded, and the record is good practice — *"a silent one is how a guard
/// dies"* is the paragraph's own sentence. What nothing checked is whether the
/// record survives the question being **answered**. It did not: `deny.toml`'s
/// single `async-trait` entry now names `worker` and `worker-macros` as wrappers
/// and cites ADR-0035 for it, `cargo deny check bans` reports `bans ok`, and the
/// manifest went on declaring the step red and routing the reader to an
/// escalation for a decision already taken.
///
/// The wrong implementation, and it is the one that shipped: **three artefacts
/// at one commit disagreeing, with the manifest the only one that is wrong.** A
/// contributor reads a present-tense unratified finding and either re-opens a
/// settled question or stops believing the green gate the rest of the evidence
/// rests on. They find the contradiction only by opening `deny.toml` themselves.
///
/// Every side of the comparison is an artefact that moves on its own — the
/// wrappers list, and the ADR ids inside the entry's own `reason` string — so
/// this cannot be satisfied by editing the sentence into agreement with a string
/// this file also writes.
///
/// **Its limits, stated rather than implied.** It reads the *shape* of the
/// claim, not its truth: it cannot run `cargo deny`, and a paragraph that goes
/// wrong some third way passes here. And it is vacuous once this crate is
/// unpacked from its `.crate` artifact, where `deny.toml` no longer resolves —
/// the same limit [`repository`] carries for the licence comparison.
#[test]
fn the_manifest_reports_the_async_trait_ban_the_way_deny_toml_settled_it() {
    let Some(deny) = repository("deny.toml") else {
        return;
    };
    let entry = async_trait_deny_entry(&deny).expect(
        "deny.toml must carry a `deny` entry for `async-trait`; ADR-0001 is what puts it there",
    );

    let paragraphs: Vec<String> = manifest_comment_paragraphs()
        .into_iter()
        .filter(|paragraph| paragraph.contains("cargo deny check bans"))
        .collect();
    assert!(
        !paragraphs.is_empty(),
        "the manifest must keep saying what taking `worker` costs the ban — this crate is why the \
         exemption exists, and an unrecorded price is the failure mode the paragraph itself names"
    );

    // `worker` absent from the wrappers list would mean the ban really is red,
    // and the paragraph would be right to say so. The assertions below are about
    // the state `deny.toml` is actually in.
    if !entry.contains(r#""worker""#) {
        return;
    }

    for paragraph in &paragraphs {
        assert!(
            !paragraph.contains(RED_BANS_CLAIM),
            "`deny.toml` exempts `worker` and the check passes, but the manifest still declares it \
             red:\n{paragraph}"
        );
        assert!(
            !paragraph.contains(ESCALATION),
            "the question this paragraph escalates was answered — `deny.toml`'s entry is the \
             answer — so the paragraph routes a reader to a decision already taken:\n{paragraph}"
        );
        for adr in adr_ids(entry) {
            assert!(
                paragraph.contains(&adr),
                "`deny.toml`'s `async-trait` entry cites {adr} and the manifest paragraph does \
                 not; the two describe one exemption or a reader meets two:\n{paragraph}"
            );
        }
    }
}

/// Every `.rs` file under `dir`, recursively.
fn collect_rust_sources(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
    let entries =
        fs::read_dir(dir).unwrap_or_else(|err| panic!("reading {}: {err}", dir.display()));
    for entry in entries {
        let path = entry
            .unwrap_or_else(|err| panic!("reading an entry of {}: {err}", dir.display()))
            .path();
        if path.is_dir() {
            collect_rust_sources(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}
