---
item: "HS-S0022"
stage: implement
created: "2026-08-15"
updated: "2026-08-15"
---

# Implementation Report — Codec, JSON by default, CBOR and postcard behind forwarded features

**All ten ACs are satisfied, and `cbor` ships.** The licence verdict AC-008 makes a
precondition came back clean before the dependency landed, so the alpha carries three
codecs rather than two, and the manifest records why in the place a reader will look.

M2 had already landed the `Codec` trait and `CodecError` as declarations, so this story
**completed them in place** — the concrete codecs, the feature block, the docs.rs gate
badges and the codec-tag path — and introduced no second definition. The crate's
`[features]` block went from four entries to seven; `crates/happenstance/src/codec.rs`
went from 89 lines to 369.

## TDD Evidence

Rust cannot go red on a type that does not exist — the test target fails to compile,
which is an import-shaped failure. So the four new test files were split by what they
can observe, and two of them went red on **assertions** rather than on resolution:

| AC | Test | Red | Green |
| --- | --- | --- | --- |
| AC-009 | `doc_surface.rs::crate_root_renders_the_codec_surface` | `panicked … the \`Codec\` bullet still reads as a roadmap: * [**\`codec\`**](codec) — … the concrete codecs — json and friends — and the feature flags that gate them are what is still *planned*.` | ok |
| AC-010 | `doc_surface.rs::every_gated_item_carries_its_badge` | `panicked … no item in codec.rs is gated at all` | ok |
| AC-003 | `manifest_contract.rs::features_forward_and_only_add` | `panicked … \`default\` no longer contains "json": ["std", "memory"]` | ok |
| AC-008 | `manifest_contract.rs::cbor_is_shipped_or_declined_with_a_reason` | `panicked … no \`cbor\` feature and no recorded reason beside the feature block` | ok |
| AC-001, AC-002, AC-004 | `codec_round_trip.rs` | `error[E0432]: unresolved import \`happenstance::Json\`` — the item this story exists to add | ok (2 at default features, 4 at `--all-features`) |
| AC-005, AC-006 | `codec_tag.rs` | same unresolved import; then, once `Json` existed but the framing region did not agree with the test's hand-written bytes, `UnknownTag { tag: "hpst\u{1}\u{4}json" }` on all three tag tests | ok (4 tests) |

Two red steps are worth recording because they changed the implementation rather than
being repaired around it:

* **The length-prefixed framing lost to a terminator.** The first framing shape was
  `magic + version + u8 length + tag`. Writing that byte needs `tag.len() as u8`, which
  is a truncating cast the workspace's pedantic clippy rejects, and it caps a tag at 255
  for no gain. `0xFF` cannot appear in UTF-8 and `Codec::TAG` is a `&'static str`, so a
  `0xFF` terminator cannot collide with any tag. The fixture helper in `codec_tag.rs`
  moved with it; **no assertion changed**, and the format is still pinned from both sides
  (`src/tests.rs` drives the writer, `tests/codec_tag.rs` writes the bytes by hand).
* **`cargo doc` found the RS-70-2 violation the design's mock predicted.** `Codec`'s doc
  linked `[`Postcard`]` and `[`Cbor`]`, which is `error: unresolved link to \`Cbor\`` in
  any build without the gate. Fixed by naming the three codecs in plain backticks
  everywhere on an ungated page, and by moving the worked example onto `Json`'s own
  (gated) item page, where it can be written against a type that exists in that build.

## Commits

* `feat(typed-layer-and-alpha-release): Codec and feature forwarding` — see the SHA
  recorded in the slice digest; this story's whole diff plus these two reports.

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance/src/codec.rs` | Completed in place. `Json` / `Postcard` / `Cbor` unit structs and their `Codec` impls, each gated and each badged; the framing region (`MAGIC`, `FRAMING`, `TAG_END`, `TagIsWritable`, `frame`, `recover`, `unreadable`); the tag-directed read path (`decode_event`, `decode_by_tag`). `Codec`'s and `CodecError`'s docs gained the tag's home and the alternative that lost. |
| `crates/happenstance/src/boundary.rs` | Three lines: `Boundary::absorb` now routes through `codec::decode_event` instead of calling `E::decode` with the codec in hand, which is what makes two encodings fold into one model. |
| `crates/happenstance/src/lib.rs` | The mount. Three gated `pub use`s beside the surviving glob; region 4's `Codec` bullet rewritten in place; region 5 (`# Features`) added above the adapter-author pointer, which stays last. |
| `crates/happenstance/src/tests.rs` | Two crate-internal tests for the write seam, which is `pub(crate)` until the command loop mounts it. |
| `crates/happenstance/Cargo.toml` | Three optional dependencies; `json` / `postcard` / `cbor` features, each `dep:`-spelled; `json` added to `default`; a `tokio` dev-dependency for the async tests. The pre-existing `serde` comment was extended, not rewritten. |
| `Cargo.toml` | `ciborium` pinned with its licence verdict recorded; `postcard` moved **above** the `# --- dev / tooling only ---` header, following the `tokio` precedent and stating the reason in place. |
| `Cargo.lock` | Five new nodes: `ciborium`, `ciborium-io`, `ciborium-ll`, `crunchy`, `half`. |
| `crates/happenstance/tests/{codec_round_trip,codec_tag,manifest_contract,doc_surface}.rs` | New. Four files, thirteen tests. |

## Gates

Run from the worktree root, all green:

```
cargo test -p happenstance                          # default features (AC-001's own bar)
cargo test -p happenstance --all-features           # 20 lib + 4 + 4 + 8 + 7 + 3 + 4 + 7 doctests
cargo test -p happenstance-testkit --all-features   # AC-007: the suite, unchanged
cargo hack check --feature-powerset -p happenstance -p happenstance-core
cargo deny check licenses                           # baseline, then again with ciborium
cargo deny check bans                               # no new duplicate major
cargo check -p happenstance --no-default-features
cargo check -p happenstance --no-default-features --features std,json
cargo check -p happenstance --target wasm32-unknown-unknown --no-default-features --features std,json
cargo check -p happenstance --target wasm32-unknown-unknown --no-default-features --features std,json,postcard,cbor
cargo doc -p happenstance --no-deps                 # and again --no-default-features
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc -p happenstance --no-deps --all-features
cargo fmt --all --check
cargo xtask affected --base main                    # the wired story gate
```

Four of those are **not** in `cargo xtask ci --fast` and were run by hand, which is what
the ledger's preamble asks for: both feature powersets, `cargo deny`, and the nightly
`--cfg docsrs` build. A green `--fast` is not evidence for AC-002, AC-003, AC-008 or
AC-010 and is not cited as such.

The MSRV is checked by running the compiler, per ADR-0029: `rust-toolchain.toml` pins
1.97.1 and `rustc --version` reports `1.97.1 (8bab26f4f 2026-07-14)`, so every command
above **is** the MSRV build. `ciborium` and its subtree compile on it.

## Notes

* **`cbor` ships, and the order of operations is the point.** `cargo deny check licenses`
  was run at baseline (`licenses ok`), then `ciborium` was added to the graph and it was
  run again (`licenses ok`) — before a single line of `Cbor` was written. `ciborium`,
  `ciborium-io` and `ciborium-ll` are Apache-2.0; `half`, `cfg-if` and `zerocopy` are
  `MIT OR Apache-2.0`. `cargo deny check bans` reports seventeen duplicate crates, all
  of them pre-existing `windows-*`/`rand`/`getrandom` families from `sqlx`; the `syn 2`
  that `zerocopy-derive` reaches is already in the graph through
  `sqlx` → `url` → `idna` → `icu` → `yoke-derive`. `deny.toml` was read and not touched.
* **The framing region's bytes are this story's, and ADR-0021 says so.** The atom sites
  the tag in `Event::metadata` inside "a versioned framing region the typed layer owns"
  and says "its exact bytes are a later milestone's". They are: `b"hpst"` + a version
  byte + the tag + `0xFF`, and everything after that is the application's own metadata,
  copied through and never parsed. Magic-matches-but-version-does-not is a refusal rather
  than a silent misread, which is what leaves a later revision a legal move.
* **`frame` carries an `#[allow(dead_code)]` and it is temporary by construction.** The
  write seam's only library caller is `commit_with`, which is the slice-mate
  `command-loop`'s. The allow names that story in a comment beside it and goes with it;
  the seam is not untested in the meantime — `src/tests.rs` drives it directly.
* **`serde_json` is declared twice on purpose**, once as an optional dependency behind
  `json` and once as a dev-dependency. The second entry is what lets this crate's own
  test targets link it in *every* feature configuration; without it a
  `--no-default-features` test build would fail to compile on a fixture, which is a test
  the powerset would quietly drop.
* **The `# Features` table has six rows, not the design's four.** `unstable-projection`
  is `projection-trait-and-runner`'s to add and does not exist in the manifest yet —
  advertising it here would be the changelog-claims-a-feature-the-manifest-lacks defect
  the design names as anti-pattern 15. The three forwarded features (`std`, `memory`,
  `serde`) were added instead, because AC-U14's bar is that a reader can tell what a
  feature turns on without opening `Cargo.toml`, and three of the crate's features are
  not codecs.
* **No conformance rule was added, and that is AC-007's content rather than an omission.**
  A testkit rule that could observe the codec tag would prove it visible below the port,
  which would falsify ADR-0021 rather than verify it.
* **Nothing under `crates/happenstance-core/src/**` moved**, and no new defect candidate
  was found. D-1 (`QueryItem::new` is fallible for pre-validated inputs) is inherited
  unchanged from M2 and is still routed to AC-012's log.
