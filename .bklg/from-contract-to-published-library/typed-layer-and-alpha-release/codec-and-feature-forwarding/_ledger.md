---
item: "HS-S0022"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Codec, JSON by default, CBOR and postcard behind forwarded features

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three things about this ledger are unusual and are unusual on purpose, so an implementer does not
"work around" them. **Four rows are not discharged by `cargo xtask ci --fast`.** AC-002 and AC-003
need `cargo hack check --feature-powerset`, AC-008 needs `cargo deny check licenses`, and AC-010
needs a nightly `--cfg docsrs` doc build — all three tools live in `xtask/src/main.rs`'s `OPTIONAL`
block, which `--fast` drops (`xtask/src/main.rs:535-601`). Evidence for those rows is the output of
the command run **by hand**; a green `--fast` is not evidence for any of them. **AC-008 has two
passing shapes**: `cbor` shipped after the licence check cleared, or `cbor` declined with the reason
recorded in the manifest — both flip the row, and widening `deny.toml` flips neither. And **AC-009 is
verified by a source-reading test, not by a compile**: an unstyled, uncomposed render — bare
`pub use`s with the roadmap bullet still standing — satisfies every other row in this ledger and
fails that one, which is the whole reason it exists.

```yaml
- id: AC-001
  criterion: >-
    GIVEN an application author who has just run `cargo add happenstance` and typed no feature flag,
    WHEN they encode a typed payload with the crate's default encoding and decode it back into the
    same Rust type, THEN the value round-trips equal, the call site names no codec at all, and
    `happenstance::Codec` and `happenstance::Json` both resolve from the crate root under default
    features — because `json` is in `default` and DT-2 was resolved so that a first program names a
    domain before it names an encoding.
  satisfied: true
  evidence: >-
    crates/happenstance/src/codec.rs:142 (`pub struct Json`) and :143-160 (its `Codec` impl);
    crates/happenstance/src/lib.rs:135-137 (`pub use codec::Json;`, beside the surviving glob at
    :144); crates/happenstance/Cargo.toml:47 (`default = ["std", "memory", "json"]`).
    `cargo test -p happenstance` at DEFAULT features:
    codec_round_trip.rs::json_round_trips_under_default_features ... ok (2 passed).
  mount_point: "crates/happenstance/src/lib.rs (crate root pub use, beside `pub use happenstance_core::*;`)"
  verifying_test: "crates/happenstance/tests/codec_round_trip.rs::json_round_trips_under_default_features (cargo test -p happenstance, default features)"

- id: AC-002
  criterion: >-
    GIVEN that same author months later, needing a compact wire format for an edge deployment, WHEN
    they add `features = ["postcard"]` (and `["cbor"]` where AC-008 permits it), THEN the new codec
    type appears at the crate root and round-trips, and every item that compiled before still
    compiles — a feature adds a type and takes nothing away (RS-51-1).
  satisfied: true
  evidence: >-
    crates/happenstance/src/codec.rs:168 (`Postcard`) and :194 (`Cbor`), each `pub use`d at
    crates/happenstance/src/lib.rs:138-140 and :132-134; crates/happenstance/Cargo.toml:62-69.
    `cargo test -p happenstance --all-features`: codec_round_trip.rs::postcard_round_trips ... ok
    and ::cbor_round_trips ... ok, each asserting JSON still round-trips in the same build.
    `cargo hack check --feature-powerset -p happenstance` — every combination compiles (run by
    hand; `--fast` drops it).
  mount_point: "crates/happenstance/Cargo.toml [features] + crates/happenstance/src/lib.rs (crate root pub use)"
  verifying_test: "crates/happenstance/tests/codec_round_trip.rs::postcard_round_trips + `cargo hack check --feature-powerset -p happenstance`"

- id: AC-003
  criterion: >-
    GIVEN an integrator who writes `happenstance = { version = "…", default-features = false }`
    because their build cannot afford `std`, WHEN they compare the switches they get against a direct
    dependency on `happenstance-core`, THEN the two agree item for item: every core feature stays
    forwarded, the feature named `serde` still means `happenstance-core/serde` and nothing else,
    every optional dependency is reached with `dep:` / `?/` rather than a bare `dep/feature`, and no
    feature this story adds subtracts anything (AC-A04, RS-51-1/RS-51-2).
  satisfied: true
  evidence: >-
    crates/happenstance/Cargo.toml:44-69 — `std`/`serde`/`memory` still forward the contract crate
    unchanged, `serde` is still exactly `["happenstance-core/serde"]` (:50-58 states in a comment
    that `serde` the crate and `serde` the feature are different things), and each codec feature is
    `dep:`-spelled. `cargo test -p happenstance`:
    manifest_contract.rs::features_forward_and_only_add ... ok.
    `cargo hack check --feature-powerset -p happenstance -p happenstance-core` — clean (run by
    hand); `cargo check -p happenstance --no-default-features` — clean.
  mount_point: "crates/happenstance/Cargo.toml [features]"
  verifying_test: "crates/happenstance/tests/manifest_contract.rs::features_forward_and_only_add + `cargo hack check --feature-powerset -p happenstance -p happenstance-core`"

- id: AC-004
  criterion: >-
    GIVEN an adapter author who must never learn a payload's shape, WHEN the typed layer encodes a
    domain value and hands it to `EventStore::append`, THEN what crosses the port is opaque
    `bytes::Bytes` reached through the existing `pub use bytes;` re-export, the encode call is a pure
    synchronous act that performs no I/O and mutates no store — so a caller can build, inspect and
    discard a payload with no consequence, the append staying the single irreversible act (AC-U12) —
    and nothing on the codec path requires `happenstance-core`'s `serde` feature to be on.
  satisfied: true
  evidence: >-
    crates/happenstance/src/codec.rs:39-77 — `Codec` is a pure synchronous trait returning
    `happenstance_core::bytes::Bytes` (imported at :12, the contract crate's own re-export; no
    second `bytes` dependency in crates/happenstance/Cargo.toml). `cargo test -p happenstance`:
    codec_round_trip.rs::payload_crosses_the_port_as_bytes ... ok — asserts the store is still
    empty after the encode and that the appended event's `data` is byte-identical to the codec's
    output. `cargo check -p happenstance --no-default-features --features std,json` — clean, so no
    codec path reaches `happenstance-core/serde`.
  mount_point: "crates/happenstance/src/lib.rs (the codec surface above the port; bytes reached through happenstance-core's re-export)"
  verifying_test: "crates/happenstance/tests/codec_round_trip.rs::payload_crosses_the_port_as_bytes + `cargo check -p happenstance --no-default-features --features std,json`"

- id: AC-005
  criterion: >-
    GIVEN an application author whose store already holds events written under one encoding and who
    now writes new events under another, WHEN they read that consistency boundary back through the
    typed layer, THEN each event decodes under the codec its own tag names — the tag written on
    encode and recovered on decode at the single seam ADR-0021 sited, never at both admissible homes
    — and both generations fold into one decision model without the caller branching on encoding
    anywhere.
  satisfied: true
  evidence: >-
    The seam is `Event::metadata`, per kb-decision-0021. Written at
    crates/happenstance/src/codec.rs:261 (`frame`), recovered at :284 (`recover`), resolved at :327
    (`decode_event`) and :342 (`decode_by_tag`), and wired into the read path at
    crates/happenstance/src/boundary.rs:121-146 (`Boundary::absorb`). One home only: nothing in
    this diff touches `Tags`. `cargo test -p happenstance --all-features`:
    codec_tag.rs::two_encodings_coexist_in_one_store ... ok (a JSON-tagged and a postcard-tagged
    event in one `MemoryEventStore`, folded with `&Json` alone),
    ::an_untagged_event_decodes_with_the_codec_in_hand ... ok,
    ::application_metadata_after_the_region_is_carried_through ... ok, and the write half at
    src/tests.rs::the_framing_region_names_the_codec_that_wrote_it /
    ::application_metadata_is_copied_through_untouched.
  mount_point: "crates/happenstance/src/lib.rs (the encode/decode seam) + the tag home ADR-0021 sited (crates/happenstance-core/src/event.rs:378-402 metadata, or crates/happenstance-core/src/tag.rs:304 Tags)"
  verifying_test: "crates/happenstance/tests/codec_tag.rs::two_encodings_coexist_in_one_store"

- id: AC-006
  criterion: >-
    GIVEN a reader that meets an event whose codec tag names an encoding this build does not carry —
    a postcard-tagged event in a `json`-only build — WHEN it attempts to decode, THEN it receives
    `CodecError::UnknownTag { tag }` whose `Display` renders the tag's actual value, a typed refusal
    that names what it could not do: never a panic, never a silent guess at JSON, and never a
    category message such as "unsupported encoding" (AC-U08, RS-30-4).
  satisfied: true
  evidence: >-
    crates/happenstance/src/codec.rs:86-93 (`UnknownTag { tag }`, `#[error("no codec is registered
    for tag `{tag}`")]` — the value, not the category) produced at :366 by `decode_by_tag` and at
    :311 for an unreadable region. No `unwrap`/`expect`/`panic!` on the path; every arm returns.
    `cargo test -p happenstance`: codec_tag.rs::unknown_tag_is_a_typed_refusal ... ok, matching the
    variant and asserting `to_string()` contains the offending tag.
  mount_point: "crates/happenstance/src/lib.rs (CodecError, pub use'd at the crate root)"
  verifying_test: "crates/happenstance/tests/codec_tag.rs::unknown_tag_is_a_typed_refusal"

- id: AC-007
  criterion: >-
    GIVEN an adapter author implementing a store against the frozen contract, WHEN events the typed
    layer wrote are appended through their store and read back, THEN their adapter never reads,
    parses or branches on the codec tag: the conformance suite passes unchanged, no file under
    `crates/happenstance-core/src/**` is touched by this PR, and no store has to understand encoding
    to serve a read (VT-3, [FROZEN]).
  satisfied: true
  evidence: >-
    `git diff --stat -- crates/happenstance-core crates/happenstance-testkit deny.toml` is empty
    for this story's working tree — no contract-crate file and no suite file is touched.
    `cargo test -p happenstance-testkit --all-features` green with the suite unchanged (89, 268,
    91, 10, … passing). `cargo xtask affected --base main` green. The tag lives in
    `Event::metadata`, which VT-3 forbids every store and peer from parsing, so a conformant
    adapter cannot observe this story at all.
  mount_point: "crates/happenstance-testkit/src/suite.rs (unchanged — the observable form of 'no adapter can see this story')"
  verifying_test: "`cargo test -p happenstance-testkit` green with no suite edit + `cargo xtask affected --base main` + empty `git diff --stat main -- crates/happenstance-core/src`"

- id: AC-008
  criterion: >-
    GIVEN an evaluator with one bounded sitting who reads the feature table to decide whether this
    crate can pass their licence policy, WHEN they look for CBOR, THEN they find either a `cbor`
    feature whose dependency cleared `cargo deny check licenses` before it was added, or no `cbor`
    feature at all together with the recorded reason it was declined — and in neither case has
    `deny.toml`'s allowlist been widened to make a crate fit.
  satisfied: true
  evidence: >-
    `cbor` ships. The verdict was taken BEFORE the dependency landed: `cargo deny check licenses`
    was run at baseline (`licenses ok`), then again with `ciborium` in the graph (`licenses ok`) —
    `ciborium`/`ciborium-io`/`ciborium-ll` are Apache-2.0, `half`/`cfg-if`/`zerocopy` are
    `MIT OR Apache-2.0`, all already on the allowlist. `cargo deny check bans` adds no new
    duplicate major: the `syn 2` `zerocopy-derive` reaches is already in the graph through
    `sqlx` → `url` → `idna` → `icu` → `yoke-derive`. Recorded at Cargo.toml:41-49 and
    crates/happenstance/Cargo.toml:64-69. `git diff -- deny.toml` is empty. `cargo test -p
    happenstance`: manifest_contract.rs::cbor_is_shipped_or_declined_with_a_reason ... ok.
  mount_point: "crates/happenstance/Cargo.toml [features] (the cbor row, or its recorded absence) + Cargo.toml [workspace.dependencies]"
  verifying_test: "crates/happenstance/tests/manifest_contract.rs::cbor_is_shipped_or_declined_with_a_reason + `cargo deny check licenses` + empty `git diff -- deny.toml`"

- id: AC-009
  criterion: >-
    GIVEN an evaluator landing on the crate-root page after the alpha, WHEN they scan it for what
    this crate encodes with, THEN they meet the real surface and not a roadmap: the `Codec` bullet at
    `crates/happenstance/src/lib.rs:38-40` has become an intra-doc link to the real item in place —
    same position, same order, same discriminator prose — a `# Features` region gives each feature
    one clause, every new item is `pub use`d beside the surviving `pub use happenstance_core::*;` and
    shadows no contract name, and the density budget holds: each new item's first doc sentence ≤ 80
    characters, each identifier ≤ 24 characters, any code inside a doc fence ≤ 72 columns, and the
    module doc ≤ 130 lines total.
  satisfied: true
  evidence: >-
    crates/happenstance/src/lib.rs:80-83 — the `Codec` bullet is still the first entry of region 4,
    in its original position and order, now an intra-doc link with no "planned" left in it;
    :99-111 is the new `# Features` table, below the vocabulary and above the adapter-author
    pointer at :113-115, which stays last. `cargo test -p happenstance`:
    doc_surface.rs::crate_root_renders_the_codec_surface ... ok (bullet order, link, no roadmap,
    table form, region ordering, the surviving glob, every codec re-exported) and
    ::new_identifiers_fit_the_item_table ... ok; doc_budget.rs's seven budget tests still green
    (80-column prose, 72-column fences, ≤130 module-doc lines, first sentences, no shadowing).
    `cargo doc -p happenstance --no-deps` clean.
  mount_point: "crates/happenstance/src/lib.rs — module doc regions 4 and 5 of `crate-root-rustdoc`, plus the root pub use block"
  verifying_test: "crates/happenstance/tests/doc_surface.rs::crate_root_renders_the_codec_surface + `cargo doc -p happenstance --no-deps`"

- id: AC-010
  criterion: >-
    GIVEN the same evaluator reading the published docs.rs page rather than the source, WHEN they
    open a feature-gated item, THEN it carries a `doc_cfg` badge naming the feature that turns it on
    — the manifest declaring `[package.metadata.docs.rs] all-features = true` and
    `rustdoc-args = ["--cfg", "docsrs"]`, and `lib.rs` carrying
    `#![cfg_attr(docsrs, feature(doc_cfg))]`, neither of which exists in this crate today — and the
    page renders complete and warning-free in all three configurations, with no intra-doc link that
    resolves in only some of them (RS-70-2/RS-70-4/RS-51-5).
  satisfied: true
  evidence: >-
    crates/happenstance/Cargo.toml:71-73 and crates/happenstance/src/lib.rs:118 were already in
    place from M2; what this story added is the badge on every gated item —
    crates/happenstance/src/codec.rs:140, :145, :166, :171, :192, :197 and lib.rs:132-140. Three configurations build clean: `cargo doc -p happenstance --no-deps`,
    the same `--no-default-features`, and `cargo +nightly doc -p happenstance --no-deps
    --all-features` under `RUSTDOCFLAGS="--cfg docsrs -D warnings"`. The badge is in the rendered
    HTML — `target/doc/happenstance/struct.Json.html` carries
    `stab portability">Available on <strong>crate feature <code>json`. No intra-doc link points at
    a gated item: `Json`/`Postcard`/`Cbor` are named in plain backticks in `Codec`'s and the crate
    root's prose, and the worked example moved onto `Json`'s own gated page (codec.rs:127-138).
    `cargo test -p happenstance`: manifest_contract.rs::docs_rs_metadata_is_declared ... ok and
    doc_surface.rs::every_gated_item_carries_its_badge ... ok.
  mount_point: "crates/happenstance/Cargo.toml [package.metadata.docs.rs] + crates/happenstance/src/lib.rs (#![cfg_attr(docsrs, feature(doc_cfg))])"
  verifying_test: "crates/happenstance/tests/manifest_contract.rs::docs_rs_metadata_is_declared + `cargo doc -p happenstance --no-deps` (default and --no-default-features) + `cargo +nightly doc -p happenstance --no-deps --all-features` under RUSTDOCFLAGS=\"--cfg docsrs -D warnings\""
```
