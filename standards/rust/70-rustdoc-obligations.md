# 70 — Rustdoc obligations

> **Load when:** `error: missing documentation for …` · adding a public item ·
> `cargo doc` fails where `cargo build` passed · `unresolved link to` ·
> documenting an item that only exists under a feature · the gate fails at
> "documentation (no default features)" or at "docs.rs configuration (nightly)"
> **See also:** 51 (features) · 62 (doctests) · 80 (the gate) · 81 (what a grep
> can and cannot check)

---

## RS-70-1. Give a new member `[lints] workspace = true` before writing a public item in it.

**Why.** `[workspace.lints]` is inert until a member's own manifest opts in;
without the key the crate gets rustc's defaults, under which `missing_docs`,
`clippy::missing_errors_doc` and `missing_panics_doc` are all `allow`. The gate
runs clippy with `-D warnings`, which is why an undocumented item is a build
failure in the eleven members that carry the key — and no step of
`cargo xtask ci` reads a manifest for it, so a twelfth without it fails nothing.

**Do** — the manifest first; the documentation obligations below it only exist
because of it.

```toml
# Every member's Cargo.toml ends with these two lines, and nothing reads them
# except Cargo.
[lints]
workspace = true
```

```rust
/// A handle onto one store.
pub struct Store;

/// Why an append was refused.
#[derive(Debug)]
pub struct ConditionViolated;

impl Store {
    /// Appends `events`, returning the position assigned to the last one.
    ///
    /// # Errors
    ///
    /// Returns [`ConditionViolated`] when the store already holds a matching
    /// event. Routine under contention: rebuild the model and retry.
    ///
    /// # Panics
    ///
    /// Panics when `events` is empty. A batch is non-empty by construction, so
    /// an empty one is the caller's own bug and retrying cannot fix it.
    pub fn append(&self, events: &[u64]) -> Result<u64, ConditionViolated> {
        assert!(!events.is_empty(), "a batch is non-empty by construction");
        Ok(events.len() as u64)
    }
}

fn main() {
    assert!(matches!(Store.append(&[7]), Ok(1)));
}
```

**Not** — a member manifest with the section left off. Strip the docs out of the
fence above and clippy stops it at the gate; strip these two lines out and
nothing does, because the lints were never configured for this crate at all.

```toml
[package]
name = "happenstance-redis"
version = "0.1.0"
edition = "2024"

[dependencies]
happenstance-core = { path = "../happenstance-core" }
```

**Rejects.** A new adapter whose `Cargo.toml` was copied from a skeleton and
truncated at `[dependencies]`. Every gate step is green over it — there is no
policy in that crate to violate — so it reaches `cargo package` with an
undocumented `EventStore` impl and a fallible `append` whose error variants are
stated nowhere, in a workspace whose whole product is a contract other people
implement against. The adapter author reading the published documentation is the
first to notice, and by then the version is on crates.io, where a yank does not
remove it.

**Evidence.** `Cargo.toml:100 (Members opt in with)` · `Cargo.toml:103 (missing_docs)` ·
`Cargo.toml:116 (missing_errors_doc)` ·
`crates/happenstance-core/Cargo.toml:59 (workspace = true)` ·
`crates/happenstance-core/src/store.rs:203 (AppendError::NoEvents)` ·
[CONTRIBUTING §Style](../../CONTRIBUTING.md)

---

## RS-70-2. Never write an intra-doc link that resolves in only some feature configurations.

**Why.** `rustdoc::broken_intra_doc_links` is `deny`, so an unresolved link is a
hard error and not a warning — and rustdoc only resolves the configuration it is
told to build. `cargo doc --all-features` therefore cannot see a link that
breaks without `memory`, which is why the gate builds the documentation twice
and the second build is `-p happenstance-core --no-default-features`.

**Do** — name the type, do not link it, and say why in one sentence so the next
reader does not "fix" it.

```rust
/// The reference store, behind the `memory` feature.
pub struct MemoryEventStore;

/// A DCB-compliant event store.
///
/// For a runnable end-to-end example see `MemoryEventStore`, which the `memory`
/// feature provides. It is not linked because this item exists without that
/// feature and the link would not resolve.
pub trait EventStore {}

fn main() {}
```

The other spelling is a pair of `#[cfg_attr(feature = "…", doc = "…")]`
attributes with an ungated fallback arm; it costs two attributes per link and
buys a live link for the readers who have the feature on.

**Not** — compiles, renders, and passes `cargo doc --all-features`. It fails the
gate's "documentation (no default features)" step, and a `no_std` consumer would
have met it on their first build.

```rust
/// The reference store, behind the `memory` feature.
pub struct MemoryEventStore;

/// A DCB-compliant event store.
///
/// See [`MemoryEventStore`] for a walkthrough.
pub trait EventStore {}

fn main() {}
```

**Rejects.** D13 as it actually stood: three unconditional links to
`MemoryEventStore` in `lib.rs` and `store.rs` that had been broken in the
`no_std` configuration for as long as the gate had existed, because nothing ever
documented that configuration. The powerset step could not see them either — it
runs `cargo hack check`, not `cargo hack doc` — so the first person to find out
would have been a consumer building the crate the `no_std` support was written
for.

**Evidence.** `crates/happenstance-core/src/lib.rs:71 (The name is deliberately not a link here)` ·
`crates/happenstance-core/src/store.rs:77 (It is not linked because)` ·
`xtask/src/main.rs:502 (no default features)` · `Cargo.toml:134 (broken_intra_doc_links)`

---

## RS-70-3. Teach `doc-valid-idents` a proper noun; never allow `clippy::doc_markdown`.

**Why.** `doc_markdown` fires on any CamelCase-looking word in prose, and this
workspace's prose is full of product names that are not identifiers. `clippy.toml`
carries a `doc-valid-idents` list precisely so the lint keeps its value on the
words that *are* identifiers; an `allow` disables it for every doc comment in
whatever scope it is written at.

**Do**

```rust
/// Reads a batch out of SQLite.
///
/// SQLite and Postgres are in `doc-valid-idents`, so they are prose here.
/// `SequencePosition` is an identifier and keeps its backticks.
pub fn read_batch() {}

fn main() {}
```

**Not** — the one-line repair an agent reaches for after the first false
positive. Nothing later in the crate will be flagged, including the identifiers
the lint was worth having for.

```rust
#![allow(clippy::doc_markdown)]

/// Reads a batch out of SQLite into a SequencePosition.
pub fn read_batch() {}

fn main() {}
```

**Rejects.** A crate-root `allow` added because `Cloudflare` tripped the lint,
after which every bare `SequencePosition`, `AppendCondition` and `ReadOptions`
written in prose renders as plain text and is reported by nothing. The cost lands
on readers of the published documentation, months later, and the fix is
indistinguishable from a cosmetic pass so nobody makes it.

**Evidence.** `clippy.toml:3 (clippy::doc_markdown)` · `clippy.toml:7 (doc-valid-idents)`

---

## RS-70-4. Gate docs.rs rendering on `docsrs` with `feature(doc_cfg)` — `doc_auto_cfg` no longer exists.

**Why.** `#[doc(cfg(…))]` is still nightly-only under the `doc_cfg` gate, and the
`doc_auto_cfg` gate was *removed* in Rust 1.92.0 and folded into `doc_cfg`, so a
crate still naming it fails to compile on any nightly that could render it.
Cargo emits `--check-cfg 'cfg(docsrs,test)'` unconditionally, so the cfg needs no
`unexpected_cfgs` declaration; docs.rs has passed `--cfg docsrs` to every rustdoc
invocation since 2024, so the manifest's `rustdoc-args` entry is belt and braces
rather than load-bearing.

**Do**

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]

/// The reference store.
#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]
pub struct MemoryEventStore;

fn main() {}
```

**Not** — compiles here and in every stable build, because `docsrs` is off
everywhere except docs.rs and the gate's *probed* nightly step. Where it does not
compile is the one build a publisher cannot re-run.

```rust
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

/// The reference store.
pub struct MemoryEventStore;

fn main() {}
```

**Rejects.** A published version whose docs.rs build fails while every local gate
was green, because the nightly step is behind a probe and skips on a machine with
one toolchain. crates.io versions can be yanked and never removed, so the broken
rendering is permanent for that number, and the author learns about it from the
docs.rs build log rather than from anything they ran.

**Evidence.** `crates/happenstance-core/src/lib.rs:85 (feature(doc_cfg))` ·
`xtask/src/main.rs:45 (nightly rustdoc build with)` ·
[rustc removed features](https://raw.githubusercontent.com/rust-lang/rust/master/compiler/rustc_feature/src/removed.rs) *(checked 2026-08-09, rustc 1.97.1)* ·
[docs.rs metadata](https://docs.rs/about/metadata) *(checked 2026-08-09, rustc 1.97.1)*

---

## RS-70-5. Name the alternative that lost, in the doc comment, once.

**Why.** Nothing in the gate reads prose (→ 81), so the doc comment is the only
instrument that stops a settled decision being re-proposed. The house form is one
comment naming the constraint and the compiler error the alternative produces —
not three narrating the code, which the next reader deletes as duplication of the
line above.

**Do**

```rust
use core::future::Future;

/// The head of a store's sequence.
pub trait Head {
    /// How this adapter fails.
    type Error;

    /// The highest position currently visible, or `None` when the store is
    /// empty.
    ///
    /// # Why this is required rather than provided
    ///
    /// The only provided body holds `&self` across an `await`, which needs
    /// `where Self: Sync` — and the edge adapter the bare flavour exists for is
    /// not `Sync`, so the convenience body would fail to compile for exactly
    /// the adapter it was meant to spare.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the head cannot be determined.
    fn head(&self) -> impl Future<Output = Result<Option<u64>, Self::Error>>;
}

fn main() {}
```

**Not** — every word restates the signature. It satisfies `missing_docs` and
`missing_errors_doc`, so the whole gate is green over it; the catch is a reader.

```rust
use core::future::Future;

/// The head of a store's sequence.
pub trait Head {
    /// How this adapter fails.
    type Error;

    /// Returns the head position.
    ///
    /// # Errors
    ///
    /// Returns an error on failure.
    fn head(&self) -> impl Future<Output = Result<Option<u64>, Self::Error>>;
}

fn main() {}
```

**Rejects.** A pull request turning `head` back into a provided method with a
`read(&Query::all(), backwards().limit(1))` body. It compiles against
`MemoryEventStore` and every native adapter and passes the whole gate; ES-30 says
why it is wrong, and this doc comment is the only place a reviewer reading the
trait would have met ES-30. Without it the trade is re-argued by whoever
remembers it, and in the meantime a blanket body no adapter can override has
displaced SQLite's `SELECT max(position)` fast path.

**Evidence.** `crates/happenstance-core/src/store.rs:222 (Why this is required rather than provided)` ·
`crates/happenstance-core/src/store.rs:233 (where Self: Sync)` ·
[SPECIFICATION ES-30](../../spec/SPECIFICATION.md) ·
[CONTRIBUTING §Style](../../CONTRIBUTING.md)
