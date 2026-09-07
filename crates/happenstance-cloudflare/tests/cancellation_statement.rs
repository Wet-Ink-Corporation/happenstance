//! **ES-23's adapter half, as a check rather than as a promise.**
//!
//! ES-23 (`spec/SPECIFICATION.md`) is `[FROZEN]` and carries two MUSTs:
//!
//! > An adapter MAY commit an append whose future was dropped. A caller MUST NOT
//! > treat a dropped future as evidence that the append did not commit. The port
//! > MUST document this in an explicit `# Cancellation` section, and **each
//! > adapter MUST state which of the two it does.**
//!
//! The port half is discharged at `crates/happenstance-core/src/store.rs`. The
//! adapter half was not, by either shipping adapter, from phase 8 until this
//! file. The statement now sits in this crate's store module, beside the
//! `append` it is about, with a pointer to it on the front page. The wrong outcome is not "a missing heading": a caller reads the port,
//! is told in terms that the adapter will say which it does, goes to the adapter
//! and finds nothing. The pessimistic reading builds compensation this store does
//! not need; the convenient reading is that silence means "safe", which is the
//! error the port doc names by hand.
//!
//! # Why this is a test and not a paragraph
//!
//! `xtask`'s `FROZEN_DOC_MUSTS` cannot reach this clause, by its own stated
//! derivation rule: a candidate is pinned only when the obligation falls on *the
//! contract's own documentation*. ES-23's adapter half falls on an adapter's, so
//! it is invisible to the gate and to the reader of that array alike. ADR-0012
//! §5 proposed the gate step that would have caught it, recorded that *"The step
//! does not exist and has not been written"*, and named the fallback — the
//! per-adapter review at phases 8–11. The step was never built and the review did
//! not produce the statement, so both the primary instrument and its recorded
//! fallback are undischarged. A prose fix with nothing checking it would be the
//! third.
//!
//! # What this can and cannot catch
//!
//! ADR-0012 states the ceiling on any such check in its own words: *"The gate
//! check proposed in §5 catches a missing `# Cancellation` section and cannot
//! catch a section that lies."* That is why there are two checks here rather than
//! one, with a control behind them. `the_crate_states_what_a_dropped_append_does`
//! catches the section being absent or gutted;
//! `the_append_body_suspends_nowhere` catches the section becoming **false**, by
//! asserting the fact it asserts — that `append` has no suspension point, so a
//! future polled once has already run to completion. An `.await` added to
//! `append` turns the second red, which is the half ADR-0012 said a
//! heading-check could not have; `the_scan_can_see_an_await_when_there_is_one` is
//! what stops that second check being a search that has never found anything.
//!
//! It still cannot catch every lie. A section could state the right fact and draw
//! the wrong conclusion from it, and no scan sees that. What is closed is the
//! failure that actually happened twice.

/// The store module: the page holding both the statement and the body it is
/// about.
///
/// Not the crate root, and that is a constraint rather than a preference.
/// `happenstance-sqlite`'s front page is held to a density budget by
/// `tests/front_page.rs` — five `#` headings and 54-94 `//!` lines — so a
/// sixth heading of thirty-odd lines is a section that page cannot carry.
/// Both adapters put it here for that reason and for the better one: a
/// statement about what `append` does belongs on the page `append` is on.
/// Each crate root carries a pointer to it.
const EVENT_STORE: &str = include_str!("../src/event_store.rs");

/// The crate root, read only to assert the pointer to the section is still
/// there — a reader who lands on the front page must not have to guess.
const CRATE_ROOT: &str = include_str!("../src/lib.rs");

/// The signature the scan is aimed at.
const APPEND: &str = "async fn append(";

/// A line only `append`'s body carries, so an extraction that missed fails loudly
/// rather than scanning an empty string and reporting no `.await`.
const APPEND_SENTINEL: &str = "AppendError::NoEvents";

/// The method that follows `append` in the same `impl`, so an extraction that ran
/// past its closing brace is caught rather than silently widened.
const NEXT_METHOD: &str = "async fn head(";

/// AC. The statement ES-23 obliges this adapter to make is present, and says the
/// four things the clause asks for.
///
/// Named tokens rather than a whole paragraph, because the paragraph is prose and
/// prose is rewritten. Each token is one of the clause's own obligations: the
/// heading it names, the clause a reader has to be able to find, the answer
/// (*which* of the two this adapter does), and the caller constraint ES-23
/// forbids an adapter from relaxing.
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

/// AC. The front page points at the statement.
///
/// The section is one module down, which is where it belongs and is not where a
/// reader lands. Without this the statement could be correct, checked, and
/// findable only by someone who already knew it was there.
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

/// AC. The fact the statement rests on is still true.
///
/// `append` evaluates its condition and writes its rows through the synchronous
/// `SqlStorage::exec`, with nothing awaited between them — which the body says of
/// itself in a comment, and which is exactly why it needs a check: a comment is
/// not an instrument, and this crate's own module documentation already rests
/// three separate claims on that sentence being true.
///
/// **The claim is load-bearing beyond ES-23 here.** The object cannot yield to
/// its event loop mid-batch, which is what makes the compensating discard's range
/// exact and what `MAX_EVENTS_PER_BATCH`'s derivation is written against. An
/// `.await` appearing in this body would falsify all three at once, and until
/// this test nothing would have said so.
#[test]
fn the_append_body_suspends_nowhere() {
    let body = method_body(EVENT_STORE, APPEND);

    assert!(
        body.contains(APPEND_SENTINEL),
        "the extracted `append` body does not contain `{APPEND_SENTINEL}`, so the \
         scan below would be reporting on the wrong text"
    );
    assert!(
        !body.contains(NEXT_METHOD),
        "the extraction ran past `append`'s closing brace and into `{NEXT_METHOD}`, \
         so a suspension point in a *neighbouring* method would be attributed here"
    );

    assert!(
        !code_only(&body).contains(".await"),
        "`append` now contains a suspension point, so the `# Cancellation` section \
         above it in this module is false as written — and so is the \
         module's account of why the compensating discard's range is exact, which \
         rests on the object not yielding to its event loop mid-batch"
    );
}

/// The control. Without it the scan above is a search that has never found
/// anything, which is indistinguishable from a search that cannot.
///
/// A fixture rather than a second method of this crate, and that is a fact about
/// the crate rather than a shortcut: nothing in
/// `crates/happenstance-cloudflare/src/` awaits at all outside its own tests.
/// `SqlStorage::exec` is synchronous inside the object and the object is
/// single-threaded, so there is nothing in this adapter for a real-source control
/// to find.
#[test]
fn the_scan_can_see_an_await_when_there_is_one() {
    let fixture = "    async fn append(&self) {\n        // .await in a comment\n        self.inner.append().await;\n    }\n";

    let body = method_body(fixture, APPEND);
    assert!(
        code_only(&body).contains(".await"),
        "the scan cannot see an `.await` that is there, so its silence over \
         `append` means nothing"
    );

    let commented = "    async fn append(&self) {\n        // a comment mentioning .await\n    }\n";
    assert!(
        !code_only(&method_body(commented, APPEND)).contains(".await"),
        "the scan counts an `.await` inside a comment, which is how this crate's \
         sibling adapter — whose `append` documents itself as containing none — \
         would fail a check about its code on the strength of its prose"
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
