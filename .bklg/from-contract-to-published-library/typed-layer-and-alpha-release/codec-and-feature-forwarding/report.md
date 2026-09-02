---
item: "HS-S0022"
stage: report
created: "2026-08-15"
updated: "2026-08-15"
---

# Report — Codec, JSON by default, CBOR and postcard behind forwarded features

## Findings Ledger

**Outcome: ten of ten ACs satisfied. Nothing deferred, nothing blocked, and `cbor`
ships rather than being declined.**

The mount point is `crates/happenstance/src/lib.rs` — the crate root, which is the only
render path this library has. Every item this story adds is `pub use`d there beside the
surviving `pub use happenstance_core::*;` (`:144`), and the co-equal manifest mount is
`crates/happenstance/Cargo.toml`'s `[features]` block (`:44-69`).

| AC | Result | What proves it | Where it is mounted |
| --- | --- | --- | --- |
| **AC-001** | satisfied | `codec_round_trip.rs::json_round_trips_under_default_features`, run by `cargo test -p happenstance` at **default** features, not `--all-features` | `lib.rs:135-137`; `Cargo.toml:47` puts `json` in `default` |
| **AC-002** | satisfied | `codec_round_trip.rs::postcard_round_trips` and `::cbor_round_trips`, each asserting JSON still round-trips in the same build; `cargo hack check --feature-powerset -p happenstance` | `lib.rs:132-140`; `Cargo.toml:62-69` |
| **AC-003** | satisfied | `manifest_contract.rs::features_forward_and_only_add`; `cargo hack check --feature-powerset -p happenstance -p happenstance-core`; `cargo check --no-default-features` | `Cargo.toml:44-69` |
| **AC-004** | satisfied | `codec_round_trip.rs::payload_crosses_the_port_as_bytes` — the store is still empty after the encode, and the appended event's `data` is byte-identical to the codec's output; `cargo check --no-default-features --features std,json` | `codec.rs:39-77`, above the port; `bytes` reached through the contract crate's own re-export |
| **AC-005** | satisfied | `codec_tag.rs::two_encodings_coexist_in_one_store` — a JSON-tagged and a postcard-tagged event in one `MemoryEventStore`, folded with `&Json` alone; plus `::an_untagged_event_decodes_with_the_codec_in_hand` and `::application_metadata_after_the_region_is_carried_through`, and the write half in `src/tests.rs` | `codec.rs:261` (`frame`), `:284` (`recover`), `:327` (`decode_event`); wired into `boundary.rs:121-146` |
| **AC-006** | satisfied | `codec_tag.rs::unknown_tag_is_a_typed_refusal` — the variant matches and `to_string()` renders `protobuf`, the tag's actual value | `codec.rs:86-93`, `pub use`d at `lib.rs:141` |
| **AC-007** | satisfied | `cargo test -p happenstance-testkit --all-features` green with **no** suite edit; `git diff --stat -- crates/happenstance-core crates/happenstance-testkit deny.toml` empty; `cargo xtask affected --base main` green | nothing — the point is that no adapter can observe this story |
| **AC-008** | satisfied | `cargo deny check licenses` run at baseline and again with `ciborium` in the graph, **before** `Cbor` was written; `cargo deny check bans` shows no new duplicate major; `manifest_contract.rs::cbor_is_shipped_or_declined_with_a_reason`; `git diff -- deny.toml` empty | `Cargo.toml:41-49`; `crates/happenstance/Cargo.toml:64-69` |
| **AC-009** | satisfied | `doc_surface.rs::crate_root_renders_the_codec_surface` (bullet order, the link, no surviving roadmap, table form, region ordering, the glob, every codec re-exported) and the seven `doc_budget.rs` budget tests; `cargo doc -p happenstance --no-deps` | `lib.rs:80-83` (region 4, in place) and `:99-111` (region 5) |
| **AC-010** | satisfied | `manifest_contract.rs::docs_rs_metadata_is_declared`, `doc_surface.rs::every_gated_item_carries_its_badge`, three clean doc builds, and the rendered badge in `target/doc/happenstance/struct.Json.html` | `codec.rs:140, 145, 166, 171, 192, 197`; `lib.rs:118, 132-140` |

**Reviewable claims a reader should check rather than take.**

1. **The codec tag has exactly one home.** ADR-0021 sited it in `Event::metadata`; this
   diff touches no `Tags` construction anywhere. Two admissible constructions of one
   value is the defect the repository already names, and there is only one here.
2. **No adapter can see the tag.** `Boundary::absorb` is the only reader, and it is above
   the port. The framing bytes are inside `metadata`, which VT-3 (`[FROZEN]`) forbids
   every store and every peer from parsing.
3. **An untagged event decodes rather than refuses.** ADR-0021 requires this, because
   refusing would make every log written before the typed layer existed unreadable with
   no legal repair. `UnknownTag` therefore means exactly one thing: a tag *was* written
   and this build cannot honour it.
4. **The framing region's bytes are new, and are this milestone's to define.** The atom
   says so in terms. They are `b"hpst"` + a version byte + the tag + `0xFF`, then the
   application's own metadata, untouched. Magic-matches-but-version-does-not is a typed
   refusal rather than a silent misread — which is what leaves a later revision a legal
   move instead of a data-loss event.
5. **`cbor` shipping is a verdict, not an assumption.** The order was: check, then add.
   Had `cargo deny` refused, the alpha would have shipped `json` + `postcard` and the
   manifest would carry the refusal — which the AC treats as an equally passing outcome
   and which `cbor_is_shipped_or_declined_with_a_reason` still tests for.

**Carried into the slice-mate, and visible in the diff.** `codec.rs`'s `frame` carries an
`#[allow(dead_code)]` naming `command-loop` as its only library caller. It is the write
half of AC-005's seam, it is tested directly from `src/tests.rs`, and the allow is
removed by the story that mounts it. Nothing else in this story is provisional.

**Nothing deferred.** No AC is partially met, no test is skipped or `ignore`d, and no
defect candidate was added to AC-012's log — D-1 is inherited from M2 unchanged.