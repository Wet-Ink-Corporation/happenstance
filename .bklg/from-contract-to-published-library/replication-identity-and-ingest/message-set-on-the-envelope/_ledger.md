---
item: HS-S0106
stage: implement
created: 2026-08-12T13:47:46.126Z
updated: 2026-08-12T13:47:46.126Z
---

# Acceptance ledger — The message set instantiates Envelope<T>

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story. **AC-002's evidence must include an ADR-0027 `file:line` for every
derive landed** — a derive with no citation is a wire-format decision taken here, which
`crates/happenstance-sync/src/lib.rs:86-90` forbids, and the ledger row is where that becomes
visible. And a green `cargo xtask ci --fast` is never sufficient evidence on its own: it proves the
gate passed, not which assertion ran (`_decomposition.md`, testing brief, **Merge-gate commands**),
so every row below names the specific test.

```yaml
- id: AC-001
  criterion: "GIVEN an adapter author reading `happenstance-sync`'s public surface to find out what a message *is*, WHEN they look at the message set this PR lands, THEN each message is its own `pub` type wrapped at the call site as `Envelope::new(message)`, `Envelope<T>` still carries exactly its two fields in declaration order, and no `enum Message { … }` of kinds exists anywhere in the crate — so adding a message kind later is not a shape change to the versioned type and cannot force a `FORMAT_VERSION` bump on a peer that has not been redeployed."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/wire.rs — the existing `wire` module beside `Envelope<T>` (:103-143), reachable through crates/happenstance-sync/src/lib.rs:147 (`pub mod wire;`) and the public re-export block at :154-160"
  verifying_test: "wire::envelope_shape_is_unchanged_by_the_message_set in crates/happenstance-sync/tests/wire.rs (cargo test -p happenstance-sync --all-features)"
- id: AC-002
  criterion: "GIVEN an adapter author who must serialise a batch to send it, WHEN they call `serde_json::to_string(&push_batch)` or `postcard::to_stdvec(&push_batch)`, THEN it compiles and round-trips — because `PushBatch`, `EventGroup` and `ReplicatedEvent` carry exactly the derives ADR-0027 names by name and no others, and any type the atom does not name is still underived and the message set is built without it."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/peer.rs (`PushBatch` :183-196, `EventGroup` :236-252) and crates/happenstance-sync/src/identity.rs (`ReplicatedEvent` :171-199), re-exported through crates/happenstance-sync/src/lib.rs:154-160"
  verifying_test: "wire::authorised_derives_round_trip_in_both_formats in crates/happenstance-sync/tests/wire.rs, plus an ADR-0027 atom file:line cited in evidence for EACH derive landed"
- id: AC-003
  criterion: "GIVEN an adapter author whose peer receives a message from a peer one release ahead, WHEN the envelope carries a version this build does not implement, THEN the message's own `Deserialize` is never called — proved over a *real* message-set type, in both `serde_json` and `postcard`, with the forbidden implementation compiled beside it and observed to call it once."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/wire.rs — the hand-written `Deserialize` on `Envelope<T>` (:234-238, :219-308), reached through crates/happenstance-sync/src/lib.rs:147"
  verifying_test: "wire::version_is_readable_before_the_message in crates/happenstance-sync/tests/wire.rs (existing WF-8 rule, extended from `String` to a message-set type; witness count zero through the real impl, one through the locally-declared DerivedEnvelopeDeserialize on the same bytes)"
- id: AC-004
  criterion: "GIVEN an operator triaging a peer that has started refusing traffic, WHEN the refusal reaches them, THEN *\"refused, park this\"* is distinguishable from *\"malformed, drop this\"*: the `serde_json` error carries `WireError::UNSUPPORTED_FORMAT_VERSION` verbatim and the postcard path answers through `check_format_version` on a `take_from_bytes::<u16>` peek — and `SyncError` gained nothing, because a version refusal is a decoding failure and folding it into the runner's enum would take a phase-13 design decision inside an encoding change."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/wire.rs — `WireError` and `check_format_version` (:145-217), reached through crates/happenstance-sync/src/lib.rs:147 and the re-export block at :154-160"
  verifying_test: "wire::rejects_an_unknown_format_version in crates/happenstance-sync/tests/wire.rs (existing WF-8 rule, extended to a message-set type in `T` position), with `SyncError`'s variants unchanged in crates/happenstance-sync/src/lib.rs:90-96"
- id: AC-005
  criterion: "GIVEN an application author who put encrypted bytes in `Event::data` and needs them to arrive as those bytes, WHEN a message carrying that event is encoded, THEN the payload is rendered by `happenstance-core`'s own codec and nothing else: `[de ad be ef]` appears as the JSON string `\"3q2+7w==\"` and as four raw bytes in postcard, with no `serde` attribute, helper or hand-written impl in this crate touching either field."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/identity.rs — `ReplicatedEvent` (:171-199) carrying the opaque `Event`, encoded through the message set mounted in crates/happenstance-sync/src/wire.rs and re-exported at crates/happenstance-sync/src/lib.rs:154-160"
  verifying_test: "wire::payload_bytes_render_through_the_core_codec in crates/happenstance-sync/tests/wire.rs (assertion on rendered bytes in both formats), with crates/happenstance-core/tests/wire.rs:1060 and :1114 still green"
- id: AC-006
  criterion: "GIVEN the same author, WHEN somebody later \"improves readability\" by hexing the payload or swaps the `is_human_readable` branch, THEN a test fails by name — because the inverted / re-wrapping codec is compiled into the wire tests as a control and asserted against the rendered bytes, which is the only observation that separates it from the correct impl (it round-trips perfectly in both formats and every assertion on a decoded value is green)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/tests/wire.rs — the control is locally declared inside `mod wire`, in the same shape as the existing derived-envelope control (:18-24), against the message set mounted in crates/happenstance-sync/src/wire.rs"
  verifying_test: "wire::an_inverted_payload_codec_is_caught_on_the_rendered_bytes in crates/happenstance-sync/tests/wire.rs (control renders [222,173,190,239] in JSON and the ASCII of \"3q2+7w==\" in postcard, per spec/SPECIFICATION.md:2315-2322, and is rejected)"
- id: AC-007
  criterion: "GIVEN an adapter author whose origin store appended three independently guarded groups, WHEN the receiver decodes the push, THEN it is *told* three groups with each `guard` attached to its own events — it never re-infers a decomposition, because a receiver that regrouped would publish a state the origin never had."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/peer.rs — `PushBatch` (:183-196) and `EventGroup` (:236-252), encoded through the message set in crates/happenstance-sync/src/wire.rs and re-exported at crates/happenstance-sync/src/lib.rs:154-160"
  verifying_test: "wire::push_batch_preserves_group_boundaries_on_the_wire in crates/happenstance-sync/tests/wire.rs (n groups in, n groups out, per-group membership asserted, at least one `Some` guard and one `None`)"
- id: AC-008
  criterion: "GIVEN a future contributor who notices the nesting level costs bytes, WHEN they flatten `PushBatch` to a list of `ReplicatedEvent` \"because the receiver can regroup by guard\", THEN a test fails by name — because `FlattenedPushEncoding` is compiled as a control and the assertion is on the encoded *form* retaining the grouping, which no round-trip-on-values assertion can make."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/tests/wire.rs — the control is locally declared inside `mod wire`, against the `PushBatch` encoding mounted in crates/happenstance-sync/src/wire.rs"
  verifying_test: "wire::a_flattened_push_encoding_loses_the_group_boundary in crates/happenstance-sync/tests/wire.rs (control round-trips equal on decoded values and is smaller, and is rejected on structure)"
- id: AC-009
  criterion: "GIVEN a replication hub that must refuse a peer-supplied condition whose `after` is `Some(_)` (SY-6), WHEN a wire-carried `AppendCondition` reaches it, THEN `after` is there to be read — always present on the wire, never elided, with no `skip_serializing_if` on any wire struct in this crate — because a policy can only refuse what it can see, and eliding the field would make `wire_condition_with_after_is_refused` unfalsifiable by deleting the evidence it inspects."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/peer.rs — `EventGroup`'s `guard` (:236-252) carrying `happenstance-core`'s `AppendCondition`/`Guard` (crates/happenstance-core/src/append.rs:120-140), encoded through crates/happenstance-sync/src/wire.rs"
  verifying_test: "wire::guard_after_is_always_present_on_the_wire in crates/happenstance-sync/tests/wire.rs (key present in both renderings, for `after: None` and `after: Some(_)`, in both formats)"
- id: AC-010
  criterion: "GIVEN the local-first / edge developer who will wire the same adapter type as a hub on one edge and a spoke on another, WHEN they read the message set, THEN no type, field or variant names a role — no `hub`, `spoke`, `primary` or `replica` — because hub-ness is an edge property (SY-9 `[FROZEN]`) and a role in the wire format would make `hub-and-spoke-and-peer-to-peer-topologies` (HS-S0110) a format break rather than a wiring."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/wire.rs and crates/happenstance-sync/src/peer.rs — every declaration in the message set, re-exported through crates/happenstance-sync/src/lib.rs:154-160"
  verifying_test: "wire::messages_name_no_role in crates/happenstance-sync/tests/wire.rs (include_str! scan of ../src/wire.rs and ../src/peer.rs asserting the role vocabulary appears in no identifier)"
- id: AC-011
  criterion: "GIVEN the local-first / edge developer running two peers they cannot redeploy simultaneously, WHEN one of them raises a capacity bound, THEN nothing on the wire changes and the two stay mutually reachable — the over-capacity value decodes to `Ok` and is refused by the store that cannot hold it — and WHEN they ask what `FORMAT_VERSION = 1` is version 1 *of*, the constant's own rustdoc answers: whether the version is per-message or negotiated once per connection, whether landing the message set moved the constant, and the reasoning, executed from ADR-0027 rather than re-derived by the reader."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/wire.rs — `FORMAT_VERSION` and its rustdoc (:46-66), re-exported through crates/happenstance-sync/src/lib.rs:154-160"
  verifying_test: "wire::a_capacity_bound_does_not_bump_the_format_version in crates/happenstance-sync/tests/wire.rs, plus the compiled FORMAT_VERSION doctest in crates/happenstance-sync/src/wire.rs run by cargo test -p happenstance-sync --all-features --doc"
- id: AC-012
  criterion: "GIVEN an adapter author meeting this vocabulary for the first time, WHEN they open its docs, THEN every new public item carries rustdoc with a **compiled example** (and an `# Errors` section naming the conditions, not the error type, on anything fallible), nothing became `pub` that ADR-0027 did not name, every new wire test lists under `wire::` so a citation resolves, each earns a `CHANGELOG.md` entry naming the defect it detects, and no new test asserts a literal position value or reads a clock."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/wire.rs, src/peer.rs and src/identity.rs (rustdoc on every new public item, re-exported at crates/happenstance-sync/src/lib.rs:154-160); crates/happenstance-sync/tests/wire.rs (`mod wire` naming); CHANGELOG.md"
  verifying_test: "cargo test -p happenstance-sync --all-features --doc; cargo test -p happenstance-sync --all-features -- --list (every new name prints as wire::…); cargo xtask lints and cargo xtask spec-trace green with no new orphan; one CHANGELOG.md entry per new test"
```
