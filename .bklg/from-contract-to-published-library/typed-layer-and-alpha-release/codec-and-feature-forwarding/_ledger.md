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
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (crate root pub use, beside `pub use happenstance_core::*;`)"
  verifying_test: "crates/happenstance/tests/codec_round_trip.rs::json_round_trips_under_default_features (cargo test -p happenstance, default features)"

- id: AC-002
  criterion: >-
    GIVEN that same author months later, needing a compact wire format for an edge deployment, WHEN
    they add `features = ["postcard"]` (and `["cbor"]` where AC-008 permits it), THEN the new codec
    type appears at the crate root and round-trips, and every item that compiled before still
    compiles — a feature adds a type and takes nothing away (RS-51-1).
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (the encode/decode seam) + the tag home ADR-0021 sited (crates/happenstance-core/src/event.rs:378-402 metadata, or crates/happenstance-core/src/tag.rs:304 Tags)"
  verifying_test: "crates/happenstance/tests/codec_tag.rs::two_encodings_coexist_in_one_store"

- id: AC-006
  criterion: >-
    GIVEN a reader that meets an event whose codec tag names an encoding this build does not carry —
    a postcard-tagged event in a `json`-only build — WHEN it attempts to decode, THEN it receives
    `CodecError::UnknownTag { tag }` whose `Display` renders the tag's actual value, a typed refusal
    that names what it could not do: never a panic, never a silent guess at JSON, and never a
    category message such as "unsupported encoding" (AC-U08, RS-30-4).
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (CodecError, pub use'd at the crate root)"
  verifying_test: "crates/happenstance/tests/codec_tag.rs::unknown_tag_is_a_typed_refusal"

- id: AC-007
  criterion: >-
    GIVEN an adapter author implementing a store against the frozen contract, WHEN events the typed
    layer wrote are appended through their store and read back, THEN their adapter never reads,
    parses or branches on the codec tag: the conformance suite passes unchanged, no file under
    `crates/happenstance-core/src/**` is touched by this PR, and no store has to understand encoding
    to serve a read (VT-3, [FROZEN]).
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/suite.rs (unchanged — the observable form of 'no adapter can see this story')"
  verifying_test: "`cargo test -p happenstance-testkit` green with no suite edit + `cargo xtask affected --base main` + empty `git diff --stat main -- crates/happenstance-core/src`"

- id: AC-008
  criterion: >-
    GIVEN an evaluator with one bounded sitting who reads the feature table to decide whether this
    crate can pass their licence policy, WHEN they look for CBOR, THEN they find either a `cbor`
    feature whose dependency cleared `cargo deny check licenses` before it was added, or no `cbor`
    feature at all together with the recorded reason it was declined — and in neither case has
    `deny.toml`'s allowlist been widened to make a crate fit.
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/Cargo.toml [package.metadata.docs.rs] + crates/happenstance/src/lib.rs (#![cfg_attr(docsrs, feature(doc_cfg))])"
  verifying_test: "crates/happenstance/tests/manifest_contract.rs::docs_rs_metadata_is_declared + `cargo doc -p happenstance --no-deps` (default and --no-default-features) + `cargo +nightly doc -p happenstance --no-deps --all-features` under RUSTDOCFLAGS=\"--cfg docsrs -D warnings\""
```
