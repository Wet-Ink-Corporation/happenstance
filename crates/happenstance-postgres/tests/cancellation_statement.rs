//! **ES-23's adapter half, as a check rather than as a promise.**
//!
//! ES-23 (`spec/SPECIFICATION.md`) is `[FROZEN]` and obliges each adapter to
//! state, in an explicit `# Cancellation` section, which of two things a
//! dropped `append` future does. This crate carried no such section at all
//! until the `0.2.0` release pass — the strings "ES-23" and "# Cancellation"
//! returned zero hits across its `src/` — while two other adapters had
//! discharged it in prose AND with a guard test since phase 8.
//!
//! The wrong outcome is not a missing heading. A caller reads the port, is told
//! in terms that the adapter will say which it does, comes here, and finds
//! nothing — then picks a reading. The pessimistic one builds compensation this
//! store may not need; the convenient one assumes silence means safe, which is
//! the error the port documentation names by hand.
//!
//! Source-scanning only: no server, no container, no credentials. It runs in
//! the default gate, which is the point — a statement guarded by a test that
//! needs a live backend is guarded by nothing on most runs.

const EVENT_STORE: &str = include_str!("../src/event_store.rs");
const CRATE_ROOT: &str = include_str!("../src/lib.rs");

#[test]
fn the_crate_states_what_a_dropped_append_does() {
    let section = doc_section(EVENT_STORE, "# Cancellation").unwrap_or_else(|| {
        panic!(
            "this crate's store module carries no `# Cancellation` section. \
             ES-23 is [FROZEN] and obliges each adapter to state whether a dropped \
             `append` future may still have committed; a caller who reads the port \
             is told the adapter will answer, and arrives here to nothing"
        )
    });

    for token in ["ES-23", "cannot be cancelled", "MUST NOT"] {
        assert!(
            section.contains(token),
            "the `# Cancellation` section does not contain `{token}`, so it does \
             not do what ES-23 asks of it. The section must name the clause, say \
             which of the two outcomes this adapter has, and refuse to license a \
             caller to read a dropped future as evidence either way. Section:\n{section}"
        );
    }
}

#[test]
fn the_front_page_points_at_the_statement() {
    let root = CRATE_ROOT.replace('\r', "");
    assert!(
        root.contains("`# Cancellation` section"),
        "this crate's root documentation no longer points at the store module's \
         `# Cancellation` section, so ES-23's answer is a click away from a \
         reader who has no reason to guess it is there"
    );
}
/// The mechanism the statement rests on is still the one in the body.
///
/// `happenstance-sqlite`'s sibling asserts its `append` suspends NOWHERE, which
/// is what lets it say a dropped future commits nothing. This adapter answers the
/// opposite and rests on the opposite fact: `append` routes through `on_runtime`,
/// which is `Handle::spawn(work).await`, and tokio DETACHES a spawned task rather
/// than cancelling it. If that routing changed, the section would be describing an
/// adapter that no longer exists.
#[test]
fn append_still_routes_through_the_detaching_spawn() {
    let body = method_body(&code_only(EVENT_STORE), "async fn append(");
    assert!(
        body.contains("on_runtime"),
        "`append` no longer routes through `on_runtime`, so the `# Cancellation` \
         section's reason — a spawned task tokio detaches rather than cancels — \
         may no longer hold. Re-read the body, then the section. Body:\n{body}"
    );
}

/// The `# Cancellation` section of a `//!` documentation block, or `None`.
///
/// Bounded by the next `//! # ` heading rather than by the end of the block, so a
/// token appearing three sections later cannot satisfy an assertion about this
/// one.
///
/// **The section is returned with its `//!` prefixes stripped and its whitespace
/// collapsed**, and that is not tidiness. Doc comments are hard-wrapped, so a
/// two-word phrase can fall across a line break, and the check would then pass or
/// fail on where the author put a newline rather than on what the section says.
/// The first cut of this test did exactly that: two adapters carrying the same
/// sentence, one green and one red.
fn doc_section(source: &str, heading: &str) -> Option<String> {
    let source = source.replace('\r', "");
    let start = source.find(&format!("//! {heading}\n"))?;
    let rest = &source[start + 4 + heading.len()..];
    let end = rest.find("\n//! # ").unwrap_or(rest.len());
    Some(
        rest[..end]
            .lines()
            .map(|line| line.trim_start().trim_start_matches("//!"))
            .collect::<Vec<_>>()
            .join(" ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" "),
    )
}

/// The text of a method body, from its signature to the line that closes it at
/// method indentation.
///
/// # Panics
///
/// If the signature is absent, or if nothing closes it — either of which means
/// the file moved under the test and the scan must not quietly report success.
fn method_body(source: &str, signature: &str) -> String {
    let source = source.replace('\r', "");
    let start = source
        .find(signature)
        .unwrap_or_else(|| panic!("`{signature}` is no longer in this source"));
    let rest = &source[start..];
    let end = rest
        .find("\n    }\n")
        .unwrap_or_else(|| panic!("nothing closes `{signature}` at method indentation"));
    rest[..end].to_owned()
}

/// The parts of a source that are not `//` line comments.
///
/// Truncating at the first `//` on each line, which is the same rule
/// `xtask/src/proof.rs`'s own `code_only` uses and carries the same two limits: a
/// `//` inside a string literal cuts that line early, and block comments are not
/// read at all. Both can only make this scan *miss* an `.await`, never invent
/// one — and missing one is caught by the fact that the file it scans is this
/// crate's own and changes under review.
fn code_only(source: &str) -> String {
    source
        .lines()
        .map(|line| line.split("//").next().unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n")
}
