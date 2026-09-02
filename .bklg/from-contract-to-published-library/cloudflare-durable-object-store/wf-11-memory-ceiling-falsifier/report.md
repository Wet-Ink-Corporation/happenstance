---
item: "HS-S0056"
stage: report
created: "2026-08-19"
updated: "2026-08-19"
---

# Report — WF-11's falsifier tested where the memory ceiling is real

## Findings Ledger

**Outcome: ten of ten ACs satisfied, with a verdict of shape (c) — *the condition
is not constructible on this runtime*. No conformance rule added, no clause
amended, no `.kb/` write, no wire format changed, no `src/` edit anywhere.**

The story's deliverable is a number set and a verdict, and both exist: the probe
runs inside `cargo xtask ci`, on `wasm32-unknown-unknown`, driven by **one row** in
the executed-target registry. The falsifier did not fire, and the reason is not
that nobody looked — it is that the runner the gate has does not impose a memory
ceiling. That is written up as verdict (c) in `implementation-report.md`, *The
verdict*, which is the heading `adr-0023-and-atom-resolutions` (HS-S0058) cites.

### AC by AC

| AC | result | what proves it | where it is mounted |
| --- | --- | --- | --- |
| AC-001 — the ceiling is measured in-process | **satisfied** | `walk_to_the_ceiling` (`wf11_memory_ceiling.rs:337-382`), a doubling-then-bisect staircase over `core::arch::wasm32::memory_grow` reading `usize::MAX` as a refusal; reported in pages *and* bytes beside the documented platform limit and the architectural maximum, labelled as different kinds of thing. Transcript: `reached 2169 pages (142147584 bytes), refused at nothing … 11 of 64 grow requests spent`. No `unsafe` anywhere in `tests/`. | `xtask/src/proof.rs:730-749` |
| AC-002 — the real encoder, and a round trip | **satisfied** | `serde_json::to_vec` over a real `happenstance_core::Event`, reached only through the target-scoped dev edge with `features = ["std", "serde"]`; `serde_json::from_slice::<Event>` then `decoded == event` **and** byte-identical `data()`. `rg` for a local encoder in `crates/happenstance-cloudflare/tests/` returns nothing. | same row |
| AC-003 — anchored, then computed, announced before attempted | **satisfied** | both anchors asserted attempted (65,536 and 348,160); every size announced with its predicted peak *before* the attempt; the 11/3 arithmetic is a named `const` and two named functions, not a comment. | same row |
| AC-004 — deterministic bytes, comparable across sittings | **satisfied** | `SEED = 0x2545_F491_4F6C_DD1D` in the source, the xorshift64\* generator re-derived in eight lines, no dependency edge onto the out-of-gate measurement tree, no blob committed. The published host figures reproduce exactly. | same row |
| AC-005 — the control that stops the run being vacuous | **satisfied** | `the_two_encode_paths_produce_the_published_size_ratio`, order-independent by construction and observed running **first**; reproduces 464,218 and 348,163 exactly, and 13,333 / 10,000 basis points; `assert_ne!` makes an agreeing pair a failure rather than a verdict. | same row |
| AC-006 — the memory claim, not only the size claim | **satisfied** | **31 pages against 17** at the reference size — a 14-page separation, past the one-page resolution limit — with the binary path run *first* so the comparison is handicapped against the claim rather than for it. | same row |
| AC-007 — one of exactly three verdict shapes | **satisfied** | a three-variant `Verdict` enum with no catch-all (`:478-509`), chosen by `verdict` from the measurements (`:546-602`), with the emitted `WF-11 VERDICT: …` line asserted. The run chose (c). | same row |
| AC-008 — a row, never a second step | **satisfied** | one `WasmUnitTarget` row; `the_wf11_probe_is_a_row_and_not_a_second_step` additionally asserts that no `REQUIRED` step carries the runner variable in its `env` and none names the target in its `args`. `cargo xtask wasm` picks the row up by name with no further edit. | `xtask/src/proof.rs:730-749`, `:779-782` |
| AC-009 — an emptied or renamed probe fails before the run | **satisfied**, with its limit named | both negative controls performed and their output pasted verbatim; `Listed: 0 test(s)` on the `cfg`-away control is the `running 0 tests` shape a bare `cargo test` exits 0 on. The residual gap — a body emptied while both names survive — is stated in the report *and* in the registry constant's own doc comment. | same row |
| AC-010 — nothing reaches a consumer | **satisfied** | `base64` and `postcard` absent from `cargo tree -p happenstance-cloudflare -e normal` on host and wasm32; `serde_json` reaches it only through `worker`, as before. `[dependencies]` untouched, no invented feature, no `happenstance-sync` edge. Every standing detector green under `cargo xtask ci --fast`. | `crates/happenstance-cloudflare/Cargo.toml:88-108` |

### The two findings a reviewer should read

**1 — the instrument did not resolve anything until it was repaired, and the
repair is in the source rather than in the assertion.** The first executed run
reported 0 pages against 0 pages at the reference size: the runner starts the
module with a 4 MB heap, so both encodes fit inside free space the allocator
already held. `tighten` (`:237-249`) fills the free list before each measurement
and holds the ballast across it. The AC-006 assertion was **not** weakened, and no
size was raised until a difference appeared — which EC-005 explicitly forbids.

**2 — the verdict's useful number is not the one the story was expecting.** At the
platform's documented 128 MiB ceiling the arithmetic puts the firing payload at
**36,604,834 bytes**, which is thirty-five times the 1 MiB `MAX_EVENT_DATA_LEN`
this adapter's fixture declares. So even on a real Workers isolate, no payload
*this store would accept* can fire the falsifier; it can only fire on a payload
some other store accepted and this peer is asked to forward. That sharpens the
atom's sub-question 1 considerably and is what HS-S0058 carries into the
resolution.

### Deferred, and to whom

- **The wire-format decision** — whether WF-11's `MUST` re-scopes to formats rather
  than peers, and whether a streaming encode is owed — is **not** taken here and
  was never in scope. `replication-identity-and-ingest` (HS-P0017) owns it and it
  needs a decision record first.
- **The atom move** — `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`
  to *resolved rather than deleted* — is `adr-0023-and-atom-resolutions`'
  (HS-S0058), through `/redkiln:kb-ingest`. Nothing under `.kb/` is touched here.
- **A runner that enforces the platform's ceiling.** The verdict names it as the
  missing element. It is the same missing element `every-rule-under-workerd`
  escalated as a blocking finding — no `workerd`-class runner inside
  `cargo xtask ci` — and this story is a second, independent consequence of it
  rather than a new problem.

### What was not touched

`spec/SPECIFICATION.md` (WF-11's clause at `:2279-2313`, its `[PROVISIONAL]`
bracket, its `Rule:`/`Cases:` lines and its ledger row at `:8576`);
`crates/happenstance-core/` including `mod payload` and both WF-11 rules;
`crates/happenstance-sync/`; `crates/happenstance-cloudflare/src/`; `.kb/`;
`RUNBOOK.md`. `cargo xtask spec-trace` green.
