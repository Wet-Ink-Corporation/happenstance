//! What `DomainEvent::tags` owes a reader whose identifiers are not literals.
//!
//! `fn tags(&self) -> Tags` is **total**, over a `Tags` that has no infallible
//! constructor from strings. `Tag::key_value` refuses an empty value, a value
//! past the length ceiling, and a set of control and bidirectional formatting
//! characters — so an identifier that arrived as a `String` cannot be turned
//! into a tag inside this method: there is no `Result` for a `?` to sit on, and
//! the only route left is an `expect` on the write path, inside `commit`, after
//! the decision has already been taken. Discovery is the first adversarial
//! input in production.
//!
//! The method carried one line of documentation. Its sibling on the sibling
//! trait, ninety-four lines further down, carries the full reasoning for the
//! *opposite* choice — `DecisionModel::scope` returns a reference to an
//! already-validated value precisely so the fallibility stays in the caller's
//! constructor "instead of inside an infallible signature, where it could only
//! become an `unwrap`". The same resolution applies one level down, to the
//! event, and the repository already uses it: both worked examples hold a
//! validated `Tag` on their identifier newtype and `.collect()` through the
//! infallible `FromIterator<Tag> for Tags`.
//!
//! The defect was that the idiom lived only in example *source*. A reader who
//! meets the trait on docs.rs met eight rendered `fn tags(&self) -> Tags {
//! Tags::empty() }` — the one case with no tags at all — and no statement that
//! the ordinary case has a price, let alone that it is payable.
//!
//! **The signature is not touched, and that is deliberate.** Whether `tags`
//! should be fallible is `d-1-the-validated-type-has-no-total-path.md`'s, an
//! accepted and open question. What this holds is the part that is repairable
//! without settling it.

use std::path::{Path, PathBuf};

fn src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// `domain.rs`, line endings normalised.
fn domain() -> String {
    std::fs::read_to_string(src().join("domain.rs"))
        .expect("domain.rs")
        .replace("\r\n", "\n")
}

/// The `///` block immediately above `item`, markers stripped, joined.
///
/// Hard-errors on a missing item and on an empty block: an empty block passes
/// every `contains` below without reading anything.
fn doc_block_above(source: &str, item: &str) -> String {
    let at = source
        .find(item)
        .unwrap_or_else(|| panic!("`{item}` is not in `domain.rs` any more"));
    let at = source[..at].rfind('\n').map_or(0, |newline| newline + 1);

    let mut block: Vec<&str> = Vec::new();
    for line in source[..at].lines().rev() {
        let trimmed = line.trim_start();
        if let Some(body) = trimmed.strip_prefix("/// ") {
            block.push(body);
        } else if trimmed == "///" {
            block.push("");
        } else if !trimmed.starts_with("#[") && !trimmed.starts_with("//") {
            break;
        }
    }
    assert!(!block.is_empty(), "`{item}` carries no doc block at all");
    block.reverse();
    block.join("\n")
}

/// The method says where the fallibility went, and where it is payable.
///
/// The wrong implementation this rejects is the page as it shipped: one line,
/// `The tags this event carries.`, on the one method in this trait whose
/// totality costs the caller something. It also rejects the repair that stops
/// at naming the problem — the page has to point at the resolution, because a
/// reader who is told the method is total and not told what to do instead
/// writes the `expect`.
#[test]
fn the_total_method_says_what_totality_costs() {
    let source = domain();
    let block = doc_block_above(&source, "fn tags(&self) -> Tags;");
    let flat = block.split_whitespace().collect::<Vec<_>>().join(" ");

    assert!(
        block.lines().count() > 1,
        "`DomainEvent::tags` still carries one line of documentation. Its \
         sibling `DecisionModel::scope` carries four for the opposite choice, \
         and it is this one that costs a caller something."
    );

    for required in ["total", "expect", "constructor", "DecisionModel::scope"] {
        assert!(
            flat.contains(required),
            "`DomainEvent::tags`'s page never mentions `{required}`: it does \
             not say where the fallibility went, or where it is payable"
        );
    }
}

/// One rendered example carries the ordinary case, and it is compiled.
///
/// Every rendered example of this method was `Tags::empty()` — the degenerate
/// case, and the only one where the totality is free. The wrong implementation
/// this rejects is a `# Examples` block that adds a ninth.
///
/// The fence is a doctest, so `cargo test --doc` compiles it and it cannot rot
/// into an illustration of an API that has moved.
#[test]
fn the_page_renders_the_case_where_the_tag_is_not_a_literal() {
    let source = domain();
    let block = doc_block_above(&source, "fn tags(&self) -> Tags;");

    assert!(
        block.contains("# Examples"),
        "`DomainEvent::tags` has no `# Examples` block, so the only rendered \
         examples of it remain the eight `Tags::empty()` ones"
    );

    let at = block.find("# Examples").expect("checked");
    let example = &block[at..];

    for required in ["Tag::key_value", "collect", "is_err"] {
        assert!(
            example.contains(required),
            "the `# Examples` block never uses `{required}`: it does not show \
             the identifier validated once at the edge, the infallible \
             `FromIterator<Tag>` that makes `tags` total honestly, or the \
             refusal that proves the constructor is the one doing the work"
        );
    }
    assert!(
        !example.contains("Tags::empty()"),
        "the `# Examples` block is a ninth `Tags::empty()`, which is the case \
         a reader already understood"
    );
}
