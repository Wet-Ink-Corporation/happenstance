# The wire-format measurements

ADR-0016 cites "probe W1" through "probe W7" roughly forty times as the
source of nearly every number it states. No probe existed anywhere in the
repository when it was written — the probes were real programs, run once, in
throwaway crates under a session scratchpad, and then the scratchpad was
gone. "Measured, probe W5" was a label, not a citation. Six months from now
it would have been unreproducible in exactly the way
[`docs/experiments/position-visibility`](../position-visibility/README.md)'s
own methods note warns about: the form of reproducibility, not the thing
itself.

This directory is the real programs, kept. `cargo test -- --nocapture` from
here regenerates every measured number ADR-0016 cites, from source, with the
seed for the one probe that needed one written into the file rather than
described in prose.

It is **not** a crate the workspace knows about. It is not a member of
`D:/repos/happenstance/Cargo.toml`'s `members` list — the root manifest is
unchanged by this directory's existence — and it is not a `cargo xtask ci`
step. Both are load-bearing: CLAUDE.md's repository map calls
`docs/experiments/` "measurements. reproducible, and not in the gate," and
the isolation is what keeps it that way. `Cargo.toml` opens with a bare
`[workspace]` table, the same trick this repository already has no other
precedent for among its two prior experiments (neither is a Cargo crate at
all — see §6) but which is the standard way to stop cargo walking up and
adopting a nested manifest as a member. Running anything in here never
touches the root `Cargo.lock`.

---

## 1. What each probe answers, how to run it, and what it found

Everything below runs from this directory:

```console
$ cd docs/experiments/wire-format
$ cargo test -- --nocapture
```

Each test is self-contained and prints its own finding; run one at a time
with `cargo test --test <name> -- --nocapture` (integration tests) or
`cargo test --lib -- --nocapture` / `cargo test --doc` (the two probes in
`src/lib.rs`).

| Test | File | Question | ADR-0016 section |
|---|---|---|---|
| `w1_postcard_desynchronisation` | `tests/w1_postcard_desynchronisation.rs` | Does `skip_serializing_if` desynchronise a `postcard` decoder, and by how much? | §3 (WF-2, WF-7), the WF-2 amendment |
| `w2_option_query` | `tests/w2_option_query.rs` | Is `Query::All`, wrapped in `Option`, distinguishable from `None`? Does the externally tagged fix repair it? | §4 (WF-3), the WF-3/WF-4 amendments |
| `w3_store_id_json_size` | `tests/w3_store_id_json_size.rs` | What does a 16-byte `StoreId` cost as a JSON array of ints versus quoted hex? | §6 (WF-6), the WF-6 `Rejects:` correction |
| `w4_serde_default_is_inert` | `tests/w4_serde_default_is_inert.rs` | Does `#[serde(default)]` change anything observable, in either format? | §3 (WF-2), "Owed to the RUNBOOK" `:3356` |
| `w5_payload_encodings` | `tests/w5_payload_encodings.rs` | What does a 340 KiB payload cost under four candidate encodings? | §7 (WF-11) |
| `w6_envelope_varint` | `tests/w6_envelope_varint.rs` | Where does `format_version` land in `postcard`'s bytes, and does JSON care about field order? | §8 (WF-8) |
| `decorative_envelope_witness_count` | `tests/decorative_envelope_witness.rs` | Do either of WF-8's candidate rules reject a derived-`Deserialize`-plus-post-hoc-check envelope? | §8 (WF-8), the WF-8 amendment |
| `decorative_inverted_human_readable_branch` | `tests/decorative_inverted_branch.rs` | Does an inverted `is_human_readable` branch survive a round-trip rule, in either format? | §6, §7 (WF-6, WF-11) |
| `decorative_readoptions_const_assert` | `src/lib.rs` (`#[cfg(test)] mod tests`) | Does `Detect<T>` correctly identify `Serialize` where a `compile_fail` doctest cannot? | §10 (WF-12) |
| D1–D4 (four doctests) | `src/lib.rs` (doc comments) | Of four spellings of the same `compile_fail` doctest, how many pass for the wrong reason? | §10 (WF-12) |
| *(no `#[test]`; separate crate)* | `no-std-wasm-check/src/lib.rs` | Does a `no_std` + `alloc` `base64` build check clean on `wasm32-unknown-unknown`? | §7 (WF-11)'s falsifier, `no_std` half |
| *(research, no code)* | — | Does the DCB reference implementation publish a wire format? | §1 (WF-1), probe W7 |

### W7 has no code, and that is the point

Probe W7 answers a question a Rust program cannot answer: whether
`dcb.events`' TypeScript reference (`EventStore.ts` and its accompanying
specification prose) publishes a wire format at all. ADR-0016 §1 records the
finding — the reference's JSON snippets are captioned "a potential JSON
representation" beside an explicit disclaimer that implementations need not
match its field or method names, and `EventStore.ts` contains no
serialisation code — and names what it could **not** settle: whether the
specification has changed since the pass that read it (2026-08-05), what a
hypothetical reference *binary* encoding would look like (moot, since the
reference defines none), and whether other DCB implementations agree with
each other. None of that is re-runnable the way W1–W6 are; it is a reading,
with a date on it, and the date is the whole of its reproducibility
contract. Re-reading `dcb.events` today and diffing against ADR-0016 §1 is
the correct way to check it, not a test in this directory.

---

## 2. The seed, in full

W5's payload is 340 KiB generated by `xorshift64*`, seeded by a constant
written directly in `tests/w5_payload_encodings.rs`:

```rust
const SEED: u64 = 0x2545_F491_4F6C_DD1D;
```

That is the entire input. Re-running the test regenerates the same
348,160 bytes, byte for byte — verified by the checksum the test prints
alongside the first sixteen bytes:

```
first 16 raw bytes = [e7e3a8ea0b286c7fe0abf91c871971e4]
checksum (sum of bytes mod 2^32) = 44443899
```

Both match ADR-0016 §7's table exactly.

## 3. The nine neighbours, enumerated

ADR-0016's WF-2 amendment states a "3 wrong / 2 swallowed / 4 error" split
across nine values placed after the tagged `Event` in one `postcard` buffer.
`w1_postcard_desynchronisation` enumerates all nine and asserts the split as
a single tuple, so a change to any one outcome fails loudly rather than
silently changing what the total means:

| # | Neighbour | Outcome |
|---|---|---|
| 1 | a second `Event`, `event_type` `"A"`, 70-byte payload, no tags, no metadata | **WRONG** — decodes with a fabricated 63-byte `metadata` |
| 2 | a second `Event`, `event_type` `"B"`, 2-byte payload | ERROR |
| 3 | `bool` `false` | **SWALLOWED** — decodes to the correct value, having consumed the neighbour's one byte as a phantom `Option` tag |
| 4 | `bool` `true` | ERROR |
| 5 | `Option<Vec<u8>> = Some(vec![0xde, 0xad])` | **WRONG** — decodes with a fabricated 2-byte `metadata` |
| 6 | `Option<Vec<u8>> = None` | **SWALLOWED** |
| 7 | `u64 = 1` (stands in for `SequencePosition(1)`) | ERROR |
| 8 | a tuple of five `u8`: `(1, 2, 0xaa, 0xbb, 5)` | **WRONG** — decodes with a fabricated 2-byte `metadata`, one byte of the neighbour left over |
| 9 | `String` `"hello"` | ERROR |

Every ERROR is a genuine `Err`, not a crash; every WRONG is `Ok` with the
wrong value and no error at all; every SWALLOWED is `Ok` with the *original*
value, having silently eaten some or all of the neighbour's bytes as if they
belonged to the tagged `Event`. `w1_postcard_desynchronisation` asserts the
tuple `(3, 2, 4)` — the same nine, in the same order, that ADR-0016 counts.

---

## 4. Why some tests carry a local mirror instead of the real type

Two of ADR-0016's measured claims are about **HEAD's current shape**, and
phase 5 — the phase this ADR is written for — deletes that shape. An
experiment that calls straight into `happenstance_core::Event`'s `Serialize`
impl for the desync numbers, or into `happenstance_core::Query`'s for the
`Option`-ambiguity numbers, would stop demonstrating the bug the moment the
bug is fixed — it would silently start demonstrating the fix instead, under
a test name that still says "desynchronisation" or "ambiguity." That is
worse than the test disappearing, because nothing would flag the drift.

So:

- `w1_postcard_desynchronisation` defines `EventMirrorPreFix`, a permanent,
  local reproduction of `EventWire` as it stands in
  `crates/happenstance-core/src/event.rs:575-582` at HEAD (four fields,
  `tags` and `metadata` skipped at their default). It never changes
  underneath the assertions, regardless of what `happenstance-core` does.
- `w2_option_query` defines `QueryMirrorPreFix`, a permanent local
  reproduction of `Query`'s pre-fix `Serialize`/`Deserialize`
  (`query.rs:375-392` at HEAD: `All` via `serialize_none`, `Items` via
  `serialize_some`), plus `QueryWireProposed`, an externally tagged mirror of
  the *fix* — which carries no coupling risk either way, since it is a
  target shape rather than a measurement of HEAD.
- `w3_store_id_json_size` measures the "array of ints" cost against a bare
  `[u8; 16]`, not against `StoreId` directly, because `StoreId::serialize`
  (`identity.rs:194-198`) is exactly what WF-6 changes.

Each of those three files also runs the same scenario against the real
`happenstance_core` type once more, printed under a section marked
**"informational only… not asserted"**, so a reader can see at a glance
whether the phase has landed in whatever commit they're running this against
— without that observation being able to break the test.

`w4_serde_default_is_inert`, `w5_payload_encodings`, `w6_envelope_varint`,
and both `decorative_*` tests carry no such risk and needed no mirror:
`#[serde(default)]`'s own effect is isolated on fixture types this file
owns outright; `w5`'s four encodings are properties of `bytes::Bytes` (an
external crate happenstance cannot change the `Serialize` impl of) and of
base64/hex as primitives, not of any `happenstance-core` field; `w6`'s
`Envelope<T>` and both `decorative_*` fixtures measure a **proposed** shape
that does not exist in the crate yet, so there is no HEAD behaviour to drift
away from underneath them.

---

## 5. A defect the ADR states, reproduced exactly — and one place it disagrees with itself

Every number this experiment reproduces from the current text of
`docs/adr/0016-the-wire-format.md` agrees with it. Two things are worth
calling out on purpose rather than leaving to a diff:

**Confirmed, not just reproduced: postcard's truncated-buffer test.**
ADR-0016 §3 states that `#[serde(default)]`'s decode-side leniency "is
unreachable" in `postcard`, because running out of bytes there is a hard
parse error regardless of the attribute — unlike JSON, where a missing map
key genuinely benefits from `#[serde(default)]`. `w4_serde_default_is_inert`
measured this directly, by encoding a full struct and truncating it to just
past the `id` field: **both** a plain struct and a whole-struct-`#[serde(default)]`
struct reject the truncated buffer, with the identical error
(`DeserializeUnexpectedEnd`, "Hit the end of buffer"). This is worth flagging
because the first draft of this test asserted the opposite — that the
whole-struct-default type *would* accept the truncated buffer, on the belief
that serde's documented seq-mode defaulting would rescue it. It does not, in
`postcard`, and the test now asserts what was actually measured rather than
what seemed plausible beforehand.

**A live disagreement: the `no_std` half of WF-11's falsifier.** ADR-0016
§7 currently reads: *"**Not measured** — no `wasm32` or `no_std` build was
run in this pass."* `no-std-wasm-check/` in this directory is exactly that
build — a `#![no_std]` crate depending on `base64 = { default-features =
false, features = ["alloc"] }`, checked against
`wasm32-unknown-unknown`:

```console
$ cd no-std-wasm-check
$ cargo check --target wasm32-unknown-unknown
    Checking base64 v0.22.1
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.41s
```

Clean, no warnings. This mirrors the orchestrator's own M1 measurement
(`phase5-my-measurements.md`), taken **after** the lens reviews that produced
the current ADR text, which is why the ADR's prose has not caught up: M1
closes the `no_std` half of WF-11's falsifier by measurement, and ADR-0016
§7 as currently written still calls it unmeasured. The second half — that
`serde`'s `Serializer` offers no streaming entry point for a human-readable
string, so *any* human-readable payload encoding must materialise the whole
payload — is correctly marked in the ADR as read from serde's API rather
than measured, and this experiment does not attempt to measure it either: it
is a property of a trait's method signatures, not something a test
observes.

Also **not reproduced here, on purpose**: M2 and M3 from
`phase5-my-measurements.md` (that `base64` is already reachable in the
*workspace's* dependency graph via `sqlx-postgres`, at zero new crates, and
that `0.22` rather than `0.23` keeps it that way) are facts about the root
workspace's `Cargo.lock`, which this experiment is built specifically not to
touch. They were verified read-only, once, from the repository root:

```console
$ cargo tree --workspace -i base64 --depth 1
base64 v0.22.1
├── sqlx-core v0.8.6
└── sqlx-postgres v0.8.6
```

with `git status --porcelain Cargo.lock` empty before and after. A reader
who wants to re-check this runs that one command themselves, from the
workspace root, outside this directory — it needs the real `Cargo.lock`,
which is exactly the thing this experiment is not allowed to depend on or
disturb.

**One rounding note, immaterial.** ADR-0016 §7 states base64-in-JSON is
"2.678× smaller" than the array-of-integers encoding. `w5_payload_encodings`
computes the same ratio (`1,243,464 / 464,218`) as **2.679×** at three
decimal places — `2.6786…` rounds up at the fourth digit where the ADR's
figure reads as a truncation. Both describe the same measured byte counts;
neither is wrong.

---

## 6. What this does NOT measure

- **Compiled size.** Whether `base64`, `postcard`, or the wire types
  themselves move `happenstance-core`'s binary size on any target is not
  measured here — matching `docs/experiments/position-visibility`'s own
  disclaimer that its numbers are not portable across environments, this
  experiment does not even attempt a size measurement, portable or not.
- **Build time on target**, beyond the one-line `Finished ... in 1.41s`
  `cargo check` prints for `no-std-wasm-check` — that is a debug-build
  timestamp from one machine on one run, not a benchmark.
- **The workspace's own dependency graph** (§5, above) — read-only checked
  once against the real `Cargo.lock`, not re-verified by anything in this
  directory's own `cargo test`.
- **Whether `dcb.events` still says what probe W7 read on 2026-08-05** — see
  §1's note on W7 above. That is a reading with a date on it, not a test.
- **Runtime performance of the encode/decode paths themselves.** Every
  number here is a byte count or a pass/fail outcome, never a timing.
  `docs/experiments/position-visibility` is what a throughput measurement in
  this repository looks like when one is actually taken; nothing here
  attempts that.

This experiment is deliberately outside `cargo xtask ci`. `wire-format-probes`
and `no-std-wasm-check` are each isolated by an empty `[workspace]` table in
their own `Cargo.toml`, so cargo never treats either as a member of the root
workspace, and the root `members` list is untouched by their existence.
Running `cargo test` here produces `Cargo.lock` files scoped to this
directory and to `no-std-wasm-check/` alone; the root workspace's
`Cargo.lock` is never touched.

---

## 7. Layout

```
Cargo.toml                       wire-format-probes: empty [workspace], path dep on
                                  happenstance-core (std, serde, memory), dev-deps
                                  postcard, serde_json, base64, serde
src/lib.rs                       Detect<T> mechanism + its test; the four D1-D4
                                  compile_fail doctests (D1 captured, not live — see
                                  its own module doc for why)
tests/
  w1_postcard_desynchronisation.rs   W1 — the skip_serializing_if desync, all nine
                                      neighbours enumerated, local pre-fix mirror
  w2_option_query.rs                 W2 — Option<Query> ambiguity, local pre-fix
                                      mirror plus the externally-tagged fix
  w3_store_id_json_size.rs           W3 — StoreId as array-of-ints vs quoted hex
  w4_serde_default_is_inert.rs       W4 — #[serde(default)]'s isolated effect
  w5_payload_encodings.rs            W5 — 340 KiB payload, four encodings, seeded
  w6_envelope_varint.rs              W6 — postcard varint layout, JSON key order
  decorative_envelope_witness.rs     WF-8's derived-vs-hand-written witness count
  decorative_inverted_branch.rs      WF-6/WF-11's inverted is_human_readable branch
no-std-wasm-check/
  Cargo.toml                       empty [workspace]; base64 no_std + alloc only
  src/lib.rs                       M1 — the no_std/wasm32 build itself; no #[test]
README.md                          this file
```
