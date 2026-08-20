//! Checks that each publishable crate's `.crate` artifact carries its licences
//! and its README (D11).
//!
//! # Why the assertion is the deliverable
//!
//! Cargo packages only what lives inside the package directory. It will not
//! follow a path outside it, and it does not warn when `license = "MIT OR
//! Apache-2.0"` in the manifest is backed by no licence text in the artifact —
//! the metadata is what crates.io renders, so the omission is invisible until
//! someone unpacks the tarball. D11 was exactly that: three crates whose
//! metadata promised two licences neither of which shipped.
//!
//! The fix — copying both files into each crate directory — is a fact about the
//! working tree, and facts about the working tree decay. What makes it stay true
//! is this step. Running `cargo package --list` and discarding its output would
//! be the decorative-gate failure this repository names in CLAUDE.md: a check
//! that cannot fail. So the output is parsed and the three files are asserted,
//! by name, per crate.
//!
//! The wrong implementation this rejects is not hypothetical, and it is already
//! in the workspace: `cargo package -p happenstance-sqlite --list` exits 0,
//! lists seven files, and none of them is one of the three.
//!
//! # Why the set is derived and the hand list kept anyway
//!
//! That stub is not checked, because it is not published. The question is what
//! keeps *that* true. Cargo decides publishability from the manifests, and
//! promoting a stub is the one-line deletion of its `publish = false` — after
//! which the crate ships with neither licence nor README while a hand-written
//! list of names here stays green and silent. A list nothing reconciles is the
//! same decorative gate one level up.
//!
//! So [`publishable_members`] derives the set from `cargo metadata` and
//! [`reconcile`] fails when the list disagrees with it. Deriving alone would
//! keep the step honest, and it is the smaller code; the list is kept because
//! the two artefacts answer different questions. The derivation is the fact —
//! what Cargo will publish. The list is the intention — what we meant to
//! publish. A derived-only step reports "a crate is missing its licences" for
//! two unrelated bugs: a crate deliberately promoted whose files were never
//! copied, and a `publish = false` deleted by accident in a crate nobody meant
//! to ship. Holding both lets the failure say which one moved, and the second
//! bug is the one no other step in the gate would ever notice.
//!
//! # Why the JSON is scanned rather than deserialised
//!
//! `cargo metadata` speaks JSON and this workspace has no JSON parser. Three
//! options, and the trade-off is worth stating because it is the kind of call
//! Rust asks for constantly.
//!
//! Adding `serde_json` to `xtask` would reduce [`scan_publishable`] to about
//! five lines. It also puts a dependency tree into the tool that *is* the gate,
//! to read one field of one command's output — and `xtask` is the crate whose
//! whole value is that it needs nothing to run.
//!
//! Reading `crates/*/Cargo.toml` directly needs no dependency at all and is the
//! most obvious code of the three. It is also wrong for this particular check:
//! the workspace members are globs (`crates/*`, `examples/*`, `xtask`), so a
//! member added under a new path would be invisible to precisely the step that
//! exists to notice new publishable members. The derivation has to come from
//! Cargo, not from a re-implementation of Cargo's glob expansion.
//!
//! What is left is a narrow scan of `cargo metadata --format-version 1
//! --no-deps`, which is what [`scan_publishable`] is. `--format-version 1` pins
//! the schema, `--no-deps` restricts `packages` to workspace members, and the
//! scan reads exactly two keys at exactly one nesting depth. It costs one extra
//! process launch on a step that already launches three, and `--no-deps` skips
//! dependency resolution, so it is the cheapest thing here by a wide margin.
//!
//! # Why `--allow-dirty`
//!
//! The gate must run on an uncommitted tree — that is the only tree anyone runs
//! it against before pushing. Without the flag `cargo package` refuses outright
//! on any modification, so the step would be skippable in exactly the situation
//! it exists to cover.

use std::collections::BTreeSet;
use std::process::Command;

use anyhow::{Context, Result, bail};

/// The crates this workspace *intends* to publish.
///
/// An intention, not a derivation: [`publishable_members`] reads the fact out of
/// the manifests and [`reconcile`] fails when the two disagree. Both exist so
/// that the failure can say which of them moved — see the module docs.
const PUBLISHABLE: &[&str] = &[
    "happenstance-core",
    "happenstance",
    "happenstance-testkit",
    // Phase 9. The first *adapter* in the list, and it arrived here in the same
    // change that deleted its `publish = false` — which is the only order that
    // is ever green. Either half alone is the drift [`reconcile`] exists to
    // report: a crate Cargo will publish that this step does not check, or a
    // name here Cargo will not publish.
    "happenstance-cloudflare",
];

/// The files every published artifact must contain.
///
/// Both licences, because `MIT OR Apache-2.0` is a choice the consumer makes and
/// a choice needs both texts to be made. The README, because it is what
/// crates.io renders as the crate's front page and `readme = "README.md"`
/// pointing at a file outside the artifact fails silently.
const REQUIRED_FILES: &[&str] = &["LICENSE-MIT", "LICENSE-APACHE", "README.md"];

/// Runs the packaging check over every publishable crate.
///
/// # Errors
///
/// Returns an error if [`PUBLISHABLE`] no longer matches what the manifests say
/// is publishable, if `cargo metadata` or `cargo package --list` cannot be run
/// or fails, or if any crate's artifact is missing one of [`REQUIRED_FILES`].
pub(crate) fn run() -> Result<()> {
    reconcile(&publishable_members()?)?;

    let mut missing: Vec<String> = Vec::new();

    // Iterating the hand list rather than the derived set only to get authored
    // order — `reconcile` has just proved the two hold the same names.
    for name in PUBLISHABLE {
        let output = Command::new("cargo")
            .args(["package", "-p", name, "--list", "--allow-dirty", "--locked"])
            .output()
            .with_context(|| format!("failed to launch `cargo package -p {name} --list`"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!(
                "`cargo package -p {name} --list` failed:\n{}",
                stderr.trim()
            );
        }

        let listing = String::from_utf8_lossy(&output.stdout).into_owned();
        let files: BTreeSet<&str> = listing
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .collect();

        let absent: Vec<&str> = REQUIRED_FILES
            .iter()
            .copied()
            .filter(|f| !files.contains(f))
            .collect();

        if absent.is_empty() {
            println!(
                "{name}: {} files packaged, including {}",
                files.len(),
                REQUIRED_FILES.join(", ")
            );
        } else {
            for f in absent {
                println!("{name}: `{f}` is NOT in the packaged artifact");
                missing.push(format!("{name} is missing {f}"));
            }
        }
    }

    if !missing.is_empty() {
        bail!(
            "{} file(s) promised by manifest metadata but absent from the artifact: {}. Copy the \
             file into the crate directory — Cargo will not follow a path outside it.",
            missing.len(),
            missing.join(", ")
        );
    }

    Ok(())
}

/// Fails when [`PUBLISHABLE`] and the manifests disagree, naming the direction.
///
/// The two directions are two different bugs with two different remedies, which
/// is the whole reason both sets exist; a message that merely said "they differ"
/// would throw away the only information the pair carries.
///
/// # Errors
///
/// Returns an error if either set contains a name the other does not.
fn reconcile(derived: &BTreeSet<String>) -> Result<()> {
    let declared: BTreeSet<&str> = PUBLISHABLE.iter().copied().collect();

    let promoted: Vec<&str> = derived
        .iter()
        .map(String::as_str)
        .filter(|name| !declared.contains(name))
        .collect();
    let withdrawn: Vec<&str> = declared
        .iter()
        .copied()
        .filter(|name| !derived.contains(*name))
        .collect();

    let mut faults: Vec<String> = Vec::new();

    if !promoted.is_empty() {
        faults.push(format!(
            "Cargo will publish {} but this step does not check it. If the crate was promoted on \
             purpose, copy {} into its directory and add its name to PUBLISHABLE in \
             xtask/src/package.rs; if not, its `publish = false` has gone missing.",
            promoted.join(", "),
            REQUIRED_FILES.join(", ")
        ));
    }

    if !withdrawn.is_empty() {
        faults.push(format!(
            "PUBLISHABLE names {} but Cargo will not publish it — the crate has gained a `publish \
             = false`, or the name here is stale.",
            withdrawn.join(", ")
        ));
    }

    if !faults.is_empty() {
        bail!(
            "the publishable set has drifted from the manifests. {}",
            faults.join(" ")
        );
    }

    println!(
        "publishable set agrees with the manifests: {}",
        PUBLISHABLE.join(", ")
    );
    Ok(())
}

/// The workspace members Cargo would publish, according to Cargo.
///
/// # Errors
///
/// Returns an error if `cargo metadata` cannot be run, fails, or emits something
/// other than the shape `--format-version 1` promises.
fn publishable_members() -> Result<BTreeSet<String>> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps", "--locked"])
        .output()
        .context("failed to launch `cargo metadata`")?;

    if !output.status.success() {
        bail!(
            "`cargo metadata --no-deps` failed:\n{}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    let json = String::from_utf8(output.stdout).context("`cargo metadata` emitted non-UTF-8")?;
    scan_publishable(&json)
}

/// Reads the name and `publish` field of every workspace member out of
/// `cargo metadata --format-version 1 --no-deps` output.
///
/// # Errors
///
/// Returns an error if the JSON is malformed, if a package carries no `name` or
/// no `publish`, or if `publish` holds a value this does not recognise. All
/// three are "the schema moved", and a gate that guessed at the new shape would
/// be worse than one that stops.
fn scan_publishable(json: &str) -> Result<BTreeSet<String>> {
    // Depth counts `{` and `[` alike, so the root object is 1, the `packages`
    // array is 2 and a package object is 3. Confining the scan to that one depth
    // inside that one array is what makes it safe: every dependency entry has a
    // `name` too, and `[workspace.metadata]` is arbitrary user JSON that may
    // contain any key at any depth.
    const PACKAGES_DEPTH: usize = 2;
    const FIELD_DEPTH: usize = PACKAGES_DEPTH + 1;

    let bytes = json.as_bytes();
    let mut publishable = BTreeSet::new();
    let mut depth = 0_usize;
    let mut in_packages = false;
    let mut name: Option<&str> = None;
    let mut publishes: Option<bool> = None;
    let mut i = 0_usize;

    while let Some(&byte) = bytes.get(i) {
        match byte {
            b'"' => {
                let (text, after) = read_string(json, i)?;
                i = after;
                if let Some(value) = value_index(bytes, after) {
                    if depth == 1 && text == "packages" {
                        in_packages = true;
                    } else if in_packages && depth == FIELD_DEPTH {
                        match text {
                            "name" => name = Some(read_string(json, value)?.0),
                            "publish" => publishes = Some(publishes_somewhere(bytes, value)?),
                            _ => {}
                        }
                    }
                }
            }
            b'{' => {
                depth += 1;
                if in_packages && depth == FIELD_DEPTH {
                    name = None;
                    publishes = None;
                }
                i += 1;
            }
            b'[' => {
                depth += 1;
                i += 1;
            }
            b'}' => {
                if in_packages && depth == FIELD_DEPTH {
                    let (Some(package), Some(ships)) = (name, publishes) else {
                        bail!(
                            "`cargo metadata` emitted a package with no `name` or no `publish` \
                             field; the --format-version 1 schema has changed"
                        );
                    };
                    if ships {
                        publishable.insert(package.to_owned());
                    }
                }
                depth = depth.saturating_sub(1);
                i += 1;
            }
            b']' => {
                if depth == PACKAGES_DEPTH {
                    in_packages = false;
                }
                depth = depth.saturating_sub(1);
                i += 1;
            }
            _ => i += 1,
        }
    }

    if publishable.is_empty() {
        bail!("`cargo metadata` reported no publishable workspace member, which cannot be true");
    }

    Ok(publishable)
}

/// Whether a `publish` value means the crate reaches a registry at all.
///
/// Cargo does not round-trip the manifest's boolean: `publish = false` arrives
/// as `[]`, an unrestricted crate as `null`, and `publish = ["some-registry"]`
/// as itself. An allow-list counts as published here — it ships to a named
/// registry rather than crates.io, and a consumer there unpacks the same tarball
/// and finds the same missing licence.
///
/// # Errors
///
/// Returns an error on any other value, rather than guessing.
fn publishes_somewhere(bytes: &[u8], value: usize) -> Result<bool> {
    match bytes.get(value) {
        Some(b'n') => Ok(true),
        Some(b'[') => Ok(bytes.get(skip_ws(bytes, value + 1)) != Some(&b']')),
        other => bail!(
            "`cargo metadata` gave a `publish` value starting with {:?}, which is neither `null` \
             nor an array",
            other.map(|b| char::from(*b))
        ),
    }
}

/// Reads the JSON string starting at `start`, which must index a `"`, returning
/// its *undecoded* contents and the index just past the closing quote.
///
/// Undecoded is deliberate. The only strings compared against anything are
/// object keys and Cargo package names, and neither can hold an escape — a
/// package name is `[A-Za-z0-9_-]+`. The escape handling exists only so that a
/// `\"` inside some other value is not mistaken for the terminator, which on
/// Windows matters immediately: manifest paths arrive as `D:\\repos\\…`.
///
/// # Errors
///
/// Returns an error if the string never terminates.
fn read_string(json: &str, start: usize) -> Result<(&str, usize)> {
    let bytes = json.as_bytes();
    let mut i = start + 1;

    while let Some(&byte) = bytes.get(i) {
        match byte {
            b'\\' => i += 2,
            b'"' => {
                let text = json
                    .get(start + 1..i)
                    .context("`cargo metadata` emitted a string split across a char boundary")?;
                return Ok((text, i + 1));
            }
            _ => i += 1,
        }
    }

    bail!("`cargo metadata` emitted an unterminated JSON string")
}

/// The index of the value belonging to a key that ended at `after_key`, or
/// `None` when the string was not a key at all.
///
/// The scanner cannot tell a key from a string value by looking at it, so it
/// looks at what follows: only a key is followed by a colon.
fn value_index(bytes: &[u8], after_key: usize) -> Option<usize> {
    let colon = skip_ws(bytes, after_key);
    if bytes.get(colon) == Some(&b':') {
        Some(skip_ws(bytes, colon + 1))
    } else {
        None
    }
}

/// The first index at or after `i` that is not JSON whitespace.
fn skip_ws(bytes: &[u8], mut i: usize) -> usize {
    while matches!(bytes.get(i), Some(b' ' | b'\t' | b'\n' | b'\r')) {
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Metadata shaped like Cargo's, carrying every trap the scanner has to
    /// survive: the three `publish` spellings, a dependency entry whose `name`
    /// sits deeper than a package's, a workspace `metadata` object holding a
    /// decoy `packages` array, a Windows path full of escaped backslashes, and a
    /// string value containing braces and an escaped quote.
    const METADATA: &str = r#"{"packages":[
        {"name":"unrestricted","dependencies":[{"name":"a-dependency","publish":null}],
         "manifest_path":"D:\\repos\\happenstance\\crates\\unrestricted\\Cargo.toml",
         "description":"braces {} and a \" quote","publish":null},
        {"name":"withheld","publish":[]},
        {"name":"allow-listed","publish":["internal-registry"]}
    ],"metadata":{"packages":[{"name":"decoy","publish":null}]},"version":1}"#;

    /// `publish = false` reaches the JSON as `[]`, not as `false`; a registry
    /// allow-list is still publication and still needs the licences.
    #[test]
    fn reads_every_publish_spelling() {
        let found = scan_publishable(METADATA).expect("the fixture is well-formed metadata");

        assert!(found.contains("unrestricted"), "`null` means publishable");
        assert!(found.contains("allow-listed"), "an allow-list publishes");
        assert!(!found.contains("withheld"), "`[]` is `publish = false`");
    }

    /// Only names at package depth inside the real `packages` array count.
    #[test]
    fn ignores_names_that_are_not_workspace_members() {
        let found = scan_publishable(METADATA).expect("the fixture is well-formed metadata");

        assert!(
            !found.contains("a-dependency"),
            "that is a dependency entry"
        );
        assert!(!found.contains("decoy"), "that is workspace metadata");
        assert_eq!(found.len(), 2);
    }

    /// A schema change must stop the gate rather than be guessed at.
    #[test]
    fn rejects_a_publish_value_it_does_not_understand() {
        let json = r#"{"packages":[{"name":"odd","publish":false}]}"#;

        let error = scan_publishable(json).expect_err("`false` is not a shape cargo emits");
        assert!(error.to_string().contains("publish"), "{error}");
    }
}
