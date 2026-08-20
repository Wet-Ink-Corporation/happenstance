# Reproduction — the `error[E0034]` transcript, and the toolchain it came from

AC-007's **second artefact**: the toolchain the transcript in
`crates/happenstance-core/src/store.rs`'s `## Import one flavour, not both` was
reproduced on, and the run it was pasted from. AC-001's provenance record: the
thing a reviewer diffs the rendered fence against.

Reproduced because the block is the deliverable. `references/evaluation/review-dx-ergonomics.md:404-414`
says what the block must **contain** and is explicitly not the text to copy — it
predates the MSRV raise recorded in `.kb/decisions/0029-msrv-raised-to-1-97-1.md`.
EC-010 settles the tie in advance: the compiler wins.

## Toolchain

```
$ rustc --version
rustc 1.97.1 (8bab26f4f 2026-07-14)

$ cargo --version
cargo 1.97.1 (c980f4866 2026-06-30)
```

`rust-toolchain.toml` pins `channel = "1.97.1"`, so this is the workspace's own
compiler and not a convenient one. Nothing here raises or moves it (NF-005).

## The scratch crate

Not committed — a scratch package outside the worktree, depending on
`happenstance-core` by path with the `memory` feature on, whose whole content is a
deliberate double-import. `src/lib.rs`, seven lines, the ambiguous call on line 6:

```rust
use happenstance_core::{EventStore, MemoryEventStore, Query, ReadOptions, SendEventStore};

pub fn ambiguous(store: &MemoryEventStore) {
    let query = Query::all();
    let options = ReadOptions::new();
    let _ = store.read(&query, options);
}
```

Two runs, because the first exposed something worth recording rather than hiding.

1. `cargo build` — rustc printed the location line as `src\lib.rs:6:19`. The
   backslash is Windows's, and it is a property of how cargo handed rustc the
   path, not of the diagnostic.
2. `rustc --crate-type lib --edition 2024 --extern happenstance_core=<rlib> -L
   dependency=<deps> src/lib.rs` — rustc echoes the path exactly as it was given,
   so the same diagnostic came out with `src/lib.rs:6:19`.

The second run is the one the fence was pasted from. **The two runs differ in one
character and it is not a compiler character** — it is the separator the invoker
supplied — so taking the platform-neutral spelling is not an edit to rustc's
output, and a maintainer on any platform who repeats run 2 gets this file back
byte for byte.

## The run, in full

Everything rustc printed, unedited:

```text
error[E0034]: multiple applicable items in scope
 --> src/lib.rs:6:19
  |
6 |     let _ = store.read(&query, options);
  |                   ^^^^ multiple `read` found
  |
  = note: candidate #1 is defined in an impl of the trait `SendEventStore` for the type `MemoryEventStore`
  = note: candidate #2 is defined in an impl of the trait `EventStore` for the type `TraitVariantBlanketType`
help: disambiguate the method for candidate #1
  |
6 -     let _ = store.read(&query, options);
6 +     let _ = SendEventStore::read(&store, &query, options);
  |
help: disambiguate the method for candidate #2
  |
6 -     let _ = store.read(&query, options);
6 +     let _ = EventStore::read(&store, &query, options);
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0034`.
```

## What the page carries, and the one thing it does not

The fence at `crates/happenstance-core/src/store.rs:36-50` is the block above with
**candidate #1's `help:` hunk removed** and nothing else changed. That cut is the
signed-off design's yield rule (3), exercised at design time (`_design.md`,
`## Density budget`, finding F3), and it is the only cut permitted: the hunk that
survives is the one whose fix the page then repeats in words. `error: aborting`
and the `--explain` line are rustc's per-*compilation* epilogue rather than part of
the `error[E0034]` block, and they are not in the fence.

Everything else is present, including the closing `  |` of the surviving hunk. The
interrupted session's draft ended the fence one line early, at the `+` line; that
truncates a suggestion box mid-glyph and it is repaired here.

Both `= note:` lines and `TraitVariantBlanketType` are present and never yield
(`_design.md`'s yield order; AC-001).

## What is pinned, and where

`crates/happenstance-core/src/store.rs`'s `module_doc::TRANSCRIPT` holds this
excerpt character for character, and `the_fence_is_rustcs_own_e0034_output`
compares the fence to it with `assert_eq!` rather than with a handful of
`contains` calls. That is the one place in this workspace where pinning a
paragraph is right: elsewhere a paragraph pin turns rewording into a build
failure, and here **rewording is the defect**.

## Divergence from the design's mock — none

EC-010 provides for the reproduction differing materially from what `_design.md`'s
mock reproduced. It does not. Both `= note:` candidate lines, both candidate item
names and `TraitVariantBlanketType` are exactly as the design predicted, and the
longest line measures **109 characters**, which is the number `_design.md` recorded
as finding F4. Nothing was re-decided.
