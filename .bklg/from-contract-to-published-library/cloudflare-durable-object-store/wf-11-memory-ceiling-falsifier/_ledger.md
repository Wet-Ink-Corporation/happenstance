---
item: HS-S0056
stage: implement
created: 2026-08-12
updated: 2026-08-12
---

# Acceptance ledger — WF-11's falsifier tested where the memory ceiling is real

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: >-
    **GIVEN** an evaluator on *Decide in one sitting* who has read WF-11's `[PROVISIONAL]` bracket and wants to know whether anyone ever went and looked, **WHEN** they read the output of one `cargo xtask ci` — or the report that quotes it — **THEN** this isolate's memory ceiling is stated as a number **measured in-process**: a `core::arch::wasm32::memory_grow(0, delta)` staircase walked until the host refuses by returning `usize::MAX` rather than trapping, reported in 64 KiB pages *and* bytes, printed beside whatever the runner's own configuration claims so the two can be compared rather than conflated — never a figure copied from a documentation page
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — executed-target registry row (happenstance-cloudflare / wf11_memory_ceiling), driven by the named wasm32 execution Step in xtask/src/main.rs"
  verifying_test: "crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs::records_the_ceiling_and_the_encode_peaks (wasm32, executed by cargo xtask ci); plus rg -n \"unsafe\" crates/happenstance-cloudflare/tests/ returning nothing"
- id: AC-002
  criterion: >-
    **GIVEN** an adapter author on *Learn when you are finished* who suspects the number came from a local base64 helper rather than from the encoder a peer would actually call, **WHEN** they read the whole target and search it for a hand-rolled encoding, **THEN** the bytes go through `happenstance_core::Event`'s own `Serialize` — `mod payload`'s `serialize`, which calls `STANDARD.encode(value)` and hands the resulting `String` to `serializer.serialize_str` (`crates/happenstance-core/src/event.rs:626-641`) — reached only by `happenstance-core = { workspace = true, features = ["serde"] }` in the target-scoped **dev**-dependency block, with no local base64, no hand-built JSON and no re-implementation, and the encoded form **round-trips back to byte-identical input** through the same module's `deserialize`
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — executed-target registry row (happenstance-cloudflare / wf11_memory_ceiling), driven by the named wasm32 execution Step in xtask/src/main.rs"
  verifying_test: "crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs::records_the_ceiling_and_the_encode_peaks — its round-trip assertion through happenstance_core::Event's Deserialize; plus the structural rg over crates/happenstance-cloudflare/tests/"
- id: AC-003
  criterion: >-
    **GIVEN** the same evaluator, who has been burned by a measurement taken at a size chosen to succeed, **WHEN** they read which payload sizes were attempted and why, **THEN** the sizes are **anchored then computed**: 65,536 bytes (`MIN_SUPPORTED_EVENT_DATA_LEN`, `crates/happenstance-core/src/limits.rs:22` — the smallest payload every conformant store must accept), 348,160 bytes (the atom's own measurement subject), then a staircase toward the size the stated arithmetic predicts from the **measured** ceiling — peak ≈ 3.67 × payload, being the payload plus its ≈1.333× base64 `String` plus the ≈1.333× serialiser output buffer — with the predicted failure size printed *before* the attempt that tests it, so an attempt that kills the isolate still leaves its size in the transcript
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — executed-target registry row (happenstance-cloudflare / wf11_memory_ceiling), driven by the named wasm32 execution Step in xtask/src/main.rs"
  verifying_test: "crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs::records_the_ceiling_and_the_encode_peaks — announce-then-attempt logging and the assertion that both anchor sizes were attempted"
- id: AC-004
  criterion: >-
    **GIVEN** anyone who wants to re-run this next year on a different runner and compare, **WHEN** they read the target, **THEN** the payload bytes are produced by the xorshift64\* generator **re-derived in the target itself** with `SEED = 0x2545_F491_4F6C_DD1D` written in the source — the same seed `experiments/wire-format/tests/w5_payload_encodings.rs:26-42` publishes so these wasm32 numbers land beside comparable host numbers — with **no dependency edge onto `experiments/`**, which is deliberately outside the gate, and with no payload blob committed to git
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — executed-target registry row (happenstance-cloudflare / wf11_memory_ceiling), driven by the named wasm32 execution Step in xtask/src/main.rs"
  verifying_test: "crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs::the_two_encode_paths_produce_the_published_size_ratio — reproduces the published host sizes from the same seed; plus rg -n \"experiments\" crates/happenstance-cloudflare/ returning nothing"
- id: AC-005
  criterion: >-
    **GIVEN** the evaluator asking the one question that voids the whole exercise — *was the encoder exercised at all?* — **WHEN** they look for the control, **THEN** a second, deliberately **order-independent** test encodes the identical bytes through both paths and asserts the **published size ratio** holds on this runtime: base64-in-JSON ≈ 1.3333× the payload and `postcard::to_stdvec` ≈ 1.0000×, matching the atom's 464,218 and 348,163 bytes against a 348,160-byte payload — a size comparison rather than a memory comparison precisely so it cannot be flattered or flattened by whatever the first test did to linear memory
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — executed-target registry row (happenstance-cloudflare / wf11_memory_ceiling), driven by the named wasm32 execution Step in xtask/src/main.rs"
  verifying_test: "crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs::the_two_encode_paths_produce_the_published_size_ratio"
- id: AC-006
  criterion: >-
    **GIVEN** the same evaluator, who now wants the memory claim and not only the size claim, **WHEN** they read the peaks, **THEN** the human-readable encode's peak exceeds the binary encode's peak on the **same bytes** by more than one 64 KiB page at the 348,160-byte reference size — the resolution limit of the instrument, and the reason the 65,536-byte anchor is carried by AC-005's ratio and not by this row, since 0.333 × 65,536 ≈ 21 KiB is below a single page and would be unresolvable — with every peak stated as a **delta of `memory_size(0)`, a high-water mark that an allocator freeing early cannot flatter**, and said to be one in the report
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — executed-target registry row (happenstance-cloudflare / wf11_memory_ceiling), driven by the named wasm32 execution Step in xtask/src/main.rs"
  verifying_test: "crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs::records_the_ceiling_and_the_encode_peaks — the json_peak_pages > postcard_peak_pages assertion at the reference size"
- id: AC-007
  criterion: >-
    **GIVEN** the implementer of `adr-0023-and-atom-resolutions` (HS-S0058), who must move `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` to *resolved* citing this run and nothing else, **WHEN** they open this story's report, **THEN** they find a verdict in **exactly one of three admissible shapes** — *(a) it fires at payload size S*, with the ceiling, the peak and the failure mode; *(b) it does not fire*, with **H bytes of measured headroom** between peak and ceiling at the largest size this runtime could be made to attempt, H being a number; or *(c) the condition is not constructible here*, naming the exact missing element and what would supply it — expressed in the target as a **three-variant enum**, so *we did not see it fire* is not representable rather than merely discouraged
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — executed-target registry row (happenstance-cloudflare / wf11_memory_ceiling), driven by the named wasm32 execution Step in xtask/src/main.rs"
  verifying_test: "crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs::records_the_ceiling_and_the_encode_peaks — the three-variant Verdict enum and the assertion that one WF-11 VERDICT line was emitted; plus review of the report's verdict section against project.md AC-011"
- id: AC-008
  criterion: >-
    **GIVEN** the gate maintainer who built the wasm32 execution seam and committed in HS-S0048's **AC-006**, restated as HS-S0054's **AC-003**, that a further executed target arrives as a **row**, **WHEN** this story mounts the probe, **THEN** the whole `xtask` delta is **one row** in the declared executed-target registry in `xtask/src/proof.rs` naming `happenstance-cloudflare`, target `wf11_memory_ceiling` and its two unprefixed test names — no second `Step` in `const REQUIRED`, no second `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER` wiring, no second `--target` plumbing — and the probe is `#![cfg(target_arch = "wasm32")]`, so it **cannot** be satisfied by a host run where materialising 340 KiB is free
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — executed-target registry row (happenstance-cloudflare / wf11_memory_ceiling), driven by the named wasm32 execution Step in xtask/src/main.rs"
  verifying_test: "cargo test -p xtask — the new #[cfg(test)] assertions in xtask/src/proof.rs over the registry row and the resolved wasm step names; plus cargo xtask wasm executing the target"
- id: AC-009
  criterion: >-
    **GIVEN** a gate reader who knows `cargo test` exits 0 on `running 0 tests` (`xtask/src/proof.rs:9-23`), **WHEN** the probe is deleted, renamed, `#[ignore]`d, wrapped in `#[cfg(not(target_arch = "wasm32"))]`, or its target emptied, **THEN** `cargo xtask ci` **fails before the run** with a message naming exactly which expected test is missing; and the one mutation this mechanism provably cannot catch — a body emptied while both names survive — is **named in the report as a residual gap** with the reason (a name-based artefact cannot see an assertion that was deleted, and there is no rule enumeration here to derive from as HS-S0054 had) rather than left for a reader to discover
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/proof.rs — executed-target registry row (happenstance-cloudflare / wf11_memory_ceiling), driven by the named wasm32 execution Step in xtask/src/main.rs"
  verifying_test: "cargo xtask proof-artefact over the new row, plus the xtask #[cfg(test)] assertion that the row's expected-test list is non-empty, plus the two hand-run negative controls (rename, cfg-away) with their failure output recorded"
- id: AC-010
  criterion: >-
    **GIVEN** a library consumer on *Event-source at the edge without hand-rolling it* who will later `cargo add happenstance-cloudflare`, and the owners of every clause and atom this story reads, **WHEN** they resolve the crate and audit the diff after this merges, **THEN** nothing this story added reaches either of them: `serde_json`, `postcard` and the `serde`-featured `happenstance-core` live **only** in `[target.'cfg(target_arch = "wasm32")'.dev-dependencies]`, `[dependencies]` is untouched, no Cargo feature was invented to carry a test, no edge onto `happenstance-sync` exists, and `git diff --stat` shows no path under `.kb/`, `spec/`, `crates/happenstance-core/`, `crates/happenstance-sync/` or `crates/happenstance-cloudflare/src/` — with every standing detector still green
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/Cargo.toml — the [target.'cfg(target_arch = \"wasm32\")'.dev-dependencies] block landed by HS-S0054, plus the PR-boundary path list asserted over git diff --stat"
  verifying_test: "cargo tree -p happenstance-cloudflare -e normal identical before/after; cargo xtask spec-trace; cargo test -p happenstance-cloudflare (the four !Send probes incl. the_probe_is_not_vacuous); cargo test -p happenstance-core (both read-shape tests); cargo hack, cargo deny and package-check under cargo xtask ci"
```
