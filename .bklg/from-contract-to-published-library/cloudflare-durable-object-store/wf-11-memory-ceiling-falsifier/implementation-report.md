---
item: "HS-S0056"
stage: implement
created: "2026-08-19"
updated: "2026-08-19"
---

# Implementation Report — WF-11's falsifier tested where the memory ceiling is real

**One sentence for a reader in a hurry: the falsifier was fired at, on the target,
inside the gate, and it did not bite — because the runner the gate actually has
imposes no memory ceiling at all.** That is verdict **(c)**, and it is a verdict
rather than a shrug because it names the missing element and what would supply
it. Everything below is the evidence for that sentence and the guards that stop
it being a comfortable one.

## The verdict

*This is the section `adr-0023-and-atom-resolutions` (HS-S0058) cites when it
moves `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`
to resolved. It is written in the three-shape form AC-007 fixes, and the shape was
chosen by a three-variant enum in the source rather than by prose.*

**(c) The condition is not constructible on this runtime.**

- **The exact missing element.** A memory ceiling this isolate can be made to
  reach. The staircase asked the host for 2,047 pages and the host granted every
  one of them, taking linear memory to **2,169 pages = 142,147,584 bytes** — past
  Cloudflare's own documented 128 MiB (134,217,728 byte) per-isolate limit — and
  refused nothing. The walk stopped on the *probe's* page budget, not on a
  refusal, so no ceiling was measured: only a lower bound.

  The cause is upstream and already recorded. `every-rule-under-workerd`'s
  blocking finding is that **no `workerd`-class runner exists inside
  `cargo xtask ci`**; what does exist is `wasm-bindgen-test-runner` over Node,
  with a `node:sqlite`-backed `DurableObjectState`. A Node isolate has no
  per-isolate memory cap, so the one property WF-11's falsifier needs is exactly
  the property this runner does not model.

- **What would supply it.** An isolate that actually enforces the platform's
  documented ceiling — a real Workers isolate rather than this runner, or a runner
  flag capping the linear memory a test module may grow to. **At that
  134,217,728-byte ceiling the arithmetic puts the firing payload at 36,604,834
  bytes** (peak = payload × 11/3, the payload plus its ≈4/3 rendered `String` plus
  the ≈4/3 serialiser output buffer). That number is the useful half of this
  verdict: it is **thirty-five times** the 1 MiB `MAX_EVENT_DATA_LEN` this
  adapter's own fixture declares, so no payload *this* store would ever accept can
  fire the falsifier even on a real Workers isolate. The falsifier can only fire
  on a payload some **other** store accepted and this peer is asked to forward —
  which is precisely the *forwarding* condition WF-11 names, and it stays
  unmet here.

- **What was observed.** Every attempted encode succeeded. Six payload sizes were
  driven through `happenstance-core`'s real, non-streaming, human-readable payload
  encode and through the binary encode of the *same* bytes:

  | payload (bytes) | human-readable (bytes) | grew (pages / bytes) | binary (bytes) | grew (pages / bytes) |
  | ---: | ---: | ---: | ---: | ---: |
  | 65,536 | 87,453 | 5 / 327,680 | 65,558 | 2 / 131,072 |
  | 348,160 | 464,285 | 31 / 2,031,616 | 348,182 | 17 / 1,114,112 |
  | 696,320 | 928,497 | 59 / 3,866,624 | 696,342 | 33 / 2,162,688 |
  | 1,392,640 | 1,856,925 | 115 / 7,536,640 | 1,392,662 | 65 / 4,259,840 |
  | 2,785,280 | 3,713,777 | 228 / 14,942,208 | 2,785,303 | 129 / 8,454,144 |
  | 5,570,560 | 7,427,485 | 455 / 29,818,880 | 5,570,583 | 257 / 16,842,752 |

  Every payload round-tripped byte-identically back through the same module's
  `deserialize`. Every "grew" figure is a `memory_size` delta — a **high-water
  mark**, because `wasm32` linear memory never shrinks, so it cannot be flattered
  by an allocator that freed early.

- **Bearing on the atom's three ordered sub-questions.**
  1. *Does phase 9's adapter need to forward a `MIN_SUPPORTED_EVENT_DATA_LEN`-sized
     or larger payload through JSON?* It **can**: 65,536 bytes encoded and
     round-tripped here at a cost of 5 pages. It is not *forced* to fail doing so
     on any runtime reachable from this gate.
  2. *If it fires, is the fix a streaming scheme, a declared capability, or JSON
     scoped to diagnostics?* Not reached — it did not fire, and this story does
     not choose. The decision belongs to `replication-identity-and-ingest`
     (HS-P0017) and needs a decision record first.
  3. *Any bearing on ADR-0003's opaque-payload boundary?* None. The probe forwards
     bytes it never inspects, which is what makes *forward* the right verb; the
     boundary is observed here, not tested.

- **What this verdict is not.** It is not "WF-11 is safe". The category finding
  the atom records — serde's data model has no streaming entry point for a string,
  so *any* human-readable payload encoding materialises whole — is untouched and
  was re-confirmed by measurement here at every one of six sizes. What changed is
  that the *peer condition* now has a measured answer on the one runtime that was
  supposed to supply it, and the answer is that this runtime cannot supply it.

## TDD Evidence

Red first, and both reds were real failures asserting the missing behaviour rather
than compile errors.

| AC | test | red | green |
| --- | --- | --- | --- |
| AC-008 | `xtask/src/proof.rs::the_wf11_probe_is_a_row_and_not_a_second_step` | written before the registry row existed; `cargo test -p xtask` → `panicked … the WF-11 memory-ceiling probe has no row in WASM_UNIT_TARGETS, so cargo xtask ci compiles it under the existing --tests wasm32 check and executes it nowhere` | passes once the `WasmUnitTarget` row lands at `xtask/src/proof.rs:730-749`; 70 `xtask` unit tests green |
| AC-009 | `xtask/src/proof.rs::the_wf11_probe_row_names_both_of_its_tests_unprefixed` | same run: `panicked … the WF-11 memory-ceiling probe has no row in WASM_UNIT_TARGETS` | passes against `WF11_MEMORY_CEILING_TESTS` at `:779-782` |
| AC-001 … AC-007 | `crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs` (both cases) | with the row registered and the target absent, the gate's own invocation failed before running anything: `error: no test target named 'wf11_memory_ceiling' in 'happenstance-cloudflare' package` | both cases pass under `wasm-bindgen-test-runner`; transcript below |
| AC-006 | the peak-separation assertion inside `records_the_ceiling_and_the_encode_peaks` | a **second, unplanned red**: the first executed run failed with `the human-readable encode grew linear memory by 0 pages and the binary encode by 0` | fixed in the *instrument*, not in the assertion — see below |

The AC-006 red is the one worth reading. It was not a wrong assertion; it was an
instrument that could not resolve anything. See *The instrument, and the one repair
it needed*.

## The instrument, and the one repair it needed

The runner starts the module with a **4 MB heap** (`linear memory at entry = 63
pages = 4128768 bytes`). A 340 KiB payload encoded through either path therefore
fits entirely inside free space the allocator already owns: `memory_size` never
moves, and both readings come out at zero pages. **Zero against zero is not a
measurement of two encoders — it is a measurement of the runner's start-up.**

The repair is `tighten` (`crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs:237-249`):
before each measurement the free list is filled in half-page blocks until the host
has to grant a page, and the ballast is held *across* the measurement. From there
the closure's own allocations have to be asked of the host, and the delta is about
the encode. It is stated as a lower bound rather than a peak: what is left of the
free list is under one half-page, and an allocator can still round a request up.

Two ordering decisions are load-bearing and are commented where they are made:

- **Anchored encodes first, ceiling second, computed size last.** Growing memory is
  one-way, so any other order silently zeroes every peak.
- **Within one attempt, the *binary* path runs first.** Whichever path runs first
  gets first refusal on whatever free space remains, so running the cheaper one
  first can only *understate* the difference this measurement is about — never
  manufacture it. The separation at the reference size survived that handicap: 31
  pages against 17.

## The two negative controls

Both were performed against the gate's own entry point,
`cargo run -p xtask -- wasm-conformance`, and reverted afterwards.

**1 — rename one case.** `records_the_ceiling_and_the_encode_peaks` →
`records_the_ceiling`:

```
xtask failed: `happenstance-cloudflare`'s --test wf11_memory_ceiling on wasm32-unknown-unknown is missing 1 of the tests the gate names: ["records_the_ceiling_and_the_encode_peaks"]

The target builds, so `cargo test` would have exited 0 with nothing to say about them. These are named in `xtask/src/proof.rs` rather than derived, so that a rename — or a `#[cfg]` that quietly stops compiling a module — has something to disagree with. Listed: 2 test(s).
```

**2 — `cfg` the whole target away.** `#![cfg(target_arch = "wasm32")]` →
`#![cfg(not(target_arch = "wasm32"))]`:

```
xtask failed: `happenstance-cloudflare`'s --test wf11_memory_ceiling on wasm32-unknown-unknown is missing 2 of the tests the gate names: ["records_the_ceiling_and_the_encode_peaks", "the_two_encode_paths_produce_the_published_size_ratio"]

The target builds, so `cargo test` would have exited 0 with nothing to say about them. These are named in `xtask/src/proof.rs` rather than derived, so that a rename — or a `#[cfg]` that quietly stops compiling a module — has something to disagree with. Listed: 0 test(s).
```

`Listed: 0 test(s)` is the important half of the second one: that is the exact
`running 0 tests` shape a bare `cargo test` exits 0 on.

## The residual gap this mechanism cannot close

**A body emptied while both names survive.** A name-based proof artefact reads
`cargo test -- --list`; libtest lists a function whose body is `{}` exactly as it
lists one that measures an isolate. So deletion, rename, `#[ignore]` and
`cfg`-away all fail the gate, and *gutting* does not.

It is worse here than it was for the conformance target one file over, and the
reason is structural rather than an oversight: that target's expectation is
**derived** from `for_each_event_store_rule!`, so a rule that stops running has an
enumeration to disagree with. There is no enumeration behind these two names — the
row is a citation. The compensating guards are the ones inside the target: the
round trip (a gutted encode cannot produce byte-identical input), the published
size ratios asserted exactly, and `assert_ne!` on the two costs, which makes a run
whose two paths agree a failure rather than a verdict. None of those survives
deleting the assertion itself. Stated here, and in the `WF11_MEMORY_CEILING_TESTS`
doc comment, rather than left for a reader to discover.

## The transcript, verbatim

From `cargo run --locked --quiet -p xtask -- wasm-conformance` (the gate's own
`wasm32 run of the conformance rules` step), whole step 25 s wall clock, this
target 1.22 s:

```
running 2 tests
Invoking test: the_two_encode_paths_produce_the_published_size_ratio
WF-11 control: at a payload of 348160 bytes the human-readable path costs 464218 bytes (13333 basis points of the payload) and the binary path 348163 bytes (10000 basis points). Published: 464218 and 348163.
test the_two_encode_paths_produce_the_published_size_ratio ... ok
Invoking test: records_the_ceiling_and_the_encode_peaks
=== WF-11 falsifier probe ===
  page = 65536 bytes; linear memory at entry = 63 pages = 4128768 bytes
  measured in-process, never read off a documentation page. Beside it, for comparison rather than conflation, what the configuration claims: the platform's documented per-isolate limit is 134217728 bytes, and the architectural maximum of a wasm32 linear memory is 65536 pages = 4294967296 bytes. The runner this executes under declares no maximum of its own.
WF-11 attempt: payload = 65536 bytes (floor anchor: MIN_SUPPORTED_EVENT_DATA_LEN, the smallest payload every conformant store must accept, so 'a store took it but a peer cannot forward it' is a real contradiction); predicted peak = 240295 bytes = 4 pages
  human-readable: 87453 bytes, grew 5 pages (327680 bytes) | binary: 65558 bytes, grew 2 pages (131072 bytes) | both peaks are memory_size deltas, high-water marks an allocator freeing early cannot flatter | round trip byte-identical
WF-11 attempt: payload = 348160 bytes (reference anchor: the 340 KiB payload the published measurement table is about); predicted peak = 1276583 bytes = 20 pages
  human-readable: 464285 bytes, grew 31 pages (2031616 bytes) | binary: 348182 bytes, grew 17 pages (1114112 bytes) | both peaks are memory_size deltas, high-water marks an allocator freeing early cannot flatter | round trip byte-identical
WF-11 ceiling walk: started at 122 pages, reached 2169 pages (142147584 bytes), refused at nothing — the host granted every request, 11 of 64 grow requests spent, stopped on the probe's own page budget
WF-11 arithmetic: peak = payload * 11/3. Against 134217728 bytes of documented-platform (no ceiling was measured) headroom the predicted firing payload is 36604834 bytes. This probe's own attempt budget is 8388608 bytes, which is eight times the 1048576 byte MAX_EVENT_DATA_LEN this adapter's fixture declares — deliberately over that store limit, because the falsifier is about forwarding a payload rather than appending one.
WF-11 attempt: payload = 696320 bytes (staircase toward the predicted firing size); predicted peak = 2553166 bytes = 39 pages
  human-readable: 928497 bytes, grew 59 pages (3866624 bytes) | binary: 696342 bytes, grew 33 pages (2162688 bytes) | both peaks are memory_size deltas, high-water marks an allocator freeing early cannot flatter | round trip byte-identical
WF-11 attempt: payload = 1392640 bytes (staircase toward the predicted firing size); predicted peak = 5106343 bytes = 78 pages
  human-readable: 1856925 bytes, grew 115 pages (7536640 bytes) | binary: 1392662 bytes, grew 65 pages (4259840 bytes) | both peaks are memory_size deltas, high-water marks an allocator freeing early cannot flatter | round trip byte-identical
WF-11 attempt: payload = 2785280 bytes (staircase toward the predicted firing size); predicted peak = 10212686 bytes = 156 pages
  human-readable: 3713777 bytes, grew 228 pages (14942208 bytes) | binary: 2785303 bytes, grew 129 pages (8454144 bytes) | both peaks are memory_size deltas, high-water marks an allocator freeing early cannot flatter | round trip byte-identical
WF-11 attempt: payload = 5570560 bytes (staircase toward the predicted firing size); predicted peak = 20425383 bytes = 312 pages
  human-readable: 7427485 bytes, grew 455 pages (29818880 bytes) | binary: 5570583 bytes, grew 257 pages (16842752 bytes) | both peaks are memory_size deltas, high-water marks an allocator freeing early cannot flatter | round trip byte-identical
WF-11 VERDICT: (c) NOT CONSTRUCTIBLE HERE — missing: a memory ceiling this isolate can be made to reach. The host granted every one of the 2047 pages this probe asked for, taking linear memory to 2169 pages = 142147584 bytes — past the platform's own documented per-isolate limit of 134217728 bytes — and refused nothing, so no ceiling was measured and no payload the arithmetic can name would exhaust one; what would supply it: an isolate that actually enforces the platform's documented ceiling — a real Workers isolate rather than this runner, or a runner flag capping the linear memory a test module may grow to. At the platform's 134217728-byte limit the arithmetic puts the firing payload at 36604834 bytes; what was observed: every attempted encode succeeded, the largest at a payload of 5570560 bytes — which the human-readable path rendered into 7427485 bytes, growing linear memory by 455 pages (29818880 bytes), against the binary path's 5570583 bytes and 257 pages (16842752 bytes) on identical bytes
test records_the_ceiling_and_the_encode_peaks ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 filtered out; finished in 1.22s
```

## Commits

One checkpoint on `initiative/from-contract-to-published-library`, carrying the
probe, its mount, the manifest rows, the changelog entry and this story's
artefacts together — the measurement and the thing that executes it are not
separable.

| SHA | subject |
| --- | --- |
| `PENDING` | `feat(cloudflare-durable-object-store): WF-11's falsifier tested where the memory ceiling is real` |

## Changes

| file | shape of the change |
| --- | --- |
| `crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs` | **new**, 869 lines. `#![cfg(target_arch = "wasm32")]`; the ceiling staircase, the re-derived xorshift64\* generator with the seed in the source, the two encode paths on identical bytes, the round trip, the three-variant `Verdict`, and `console_log!` reporting of every number. Two `#[wasm_bindgen_test]` cases, unprefixed. |
| `crates/happenstance-cloudflare/Cargo.toml` | three rows added to `[target.'cfg(target_arch = "wasm32")'.dev-dependencies]` — `happenstance-core` with `features = ["std", "serde"]`, `serde_json`, `postcard` — with the block's own comment stating why `[dependencies]` is untouched. Nothing else. |
| `xtask/src/proof.rs` | one `WasmUnitTarget` row; one `WF11_MEMORY_CEILING_TESTS` constant naming both cases; two `#[cfg(test)]` assertions; one stale doc sentence corrected (`WasmUnitTarget::gate` said "the two rows this registry carries" and there are now three). |
| `Cargo.lock` | two lines: `postcard` and `serde_json` recorded against `happenstance-cloudflare`. A consequence of `--locked`, not a graph change — both were already in the workspace lock. |
| `CHANGELOG.md` | one `[Unreleased] / Added` entry naming the measurement that now exists and the verdict it reached. |
| this story's folder | `_ledger.md` (ten rows flipped with cited evidence), this report, `report.md`. |

**No `src/` edit anywhere.** No path under `.kb/`, `spec/`,
`crates/happenstance-core/`, `crates/happenstance-sync/` or
`crates/happenstance-cloudflare/src/` is touched. No public API, no conformance
rule, no clause, no wire format.

## Gates

| command | result |
| --- | --- |
| `cargo test -p xtask` | green — 70 unit tests, 165 integration, 62 doctests; includes the two new registry assertions |
| `cargo run --locked --quiet -p xtask -- wasm-conformance` | green — four conformance rows plus three unit rows, 25 s wall clock; the transcript above is its output |
| `cargo clippy -p happenstance-cloudflare --target wasm32-unknown-unknown --all-targets -- -D warnings` | green — no `#![allow]` was introduced to make the probe fit; one real finding (`too_many_lines`) was fixed by extracting `report_entry`/`report_ceiling`/`report_arithmetic` rather than allowed |
| `cargo fmt --all --check` | green |
| `cargo xtask affected --base main` | **`affected gate passed`** |
| `cargo xtask ci --fast` | green (the project grain for this non-terminal project) |
| `cargo tree -p happenstance-cloudflare -e normal` | `base64` and `postcard` absent on host and on wasm32; `serde_json` reaches the graph only through `worker`, as before |
| structural | `rg -n "base64\|BASE64\|b64" crates/happenstance-cloudflare/tests/` → nothing; `rg -n "unsafe" crates/happenstance-cloudflare/tests/` → nothing |

## Notes

**Three deviations from the plan, each recorded rather than absorbed.**

1. **The probe carries an explicit page budget and the ceiling was never
   measured — only bounded from below.** EC-002 anticipated this and fixed the
   response: *a ceiling reported as at least X, the probe's budget, is honest*.
   The budget is one page past the platform's documented 128 MiB, chosen so that a
   runner honouring something near the platform limit would be **detected** and one
   that does not is **bounded**. Walking a 4 GiB ceiling in 64 KiB steps would have
   turned the gate into a hang, which NF-007 forbids. Eleven of sixty-four grow
   requests were spent; the whole step costs 25 s.

2. **The seed's provenance is cited by file name, not by path.** AC-004's own
   verification asks that `rg -n "experiments" crates/happenstance-cloudflare/`
   return nothing, and the implementation notes ask for a comment citing that
   tree's path. Those conflict, and the AC wins: the target names
   `w5_payload_encodings.rs:26-42` and describes it as the workspace's out-of-gate
   wire-format measurement, without the directory word. Worth flagging that the
   crate-scope `rg` was **already** non-empty on `main` — `src/event_store.rs`,
   `src/host.rs`, `src/lib.rs` and `tests/support/mod.rs` all cite that tree in
   prose — so the property the AC is really about is *no dependency edge*, which is
   checked against the manifest instead. The new target itself is clean of the
   token.

3. **`Cargo.lock` is in the diff and is not in the spec's path list.** Adding two
   target-scoped dev-dependencies to a manifest changes the lock, and every gate
   step in this repository runs `--locked`. It is two lines and it adds nothing to
   the runtime graph.

**One thing that did not happen and is worth saying so.** EC-003's escalation path
— a fired falsifier turning the gate red, with the reflex to shrink the probe —
was never reached. The falsifier did not fire, so nothing had to be escalated to
the slice, and no size was raised or lowered after seeing a result. The staircase's
sizes are a doubling from the reference anchor, capped by a budget fixed before the
first run.

**The one number a later reader should not misread.** The probe deliberately
attempts payloads up to 8 MiB, which is eight times the 1 MiB `MAX_EVENT_DATA_LEN`
this adapter's fixture declares. That is not a violated store limit: that constant
governs what a store will **accept** on `append`, and WF-11's falsifier is about
what a peer must **forward**. The transcript says so on the line that announces the
budget.
