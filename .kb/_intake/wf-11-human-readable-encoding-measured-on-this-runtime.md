# WF-11 — the falsifier fired at, and the condition is not constructible on this runtime

**Staged for `/redkiln:kb-ingest`. Not an atom.** This resolves an **open
question**, in the shape `.kb/open-questions/README.md`'s *Resolving one* fixes:
new atom for the answer, the question's record **stays** with its body untouched,
`related` linked both ways, `status` moved to `withdrawn` or `superseded`, and the
`.kb/maps/open-questions-index.md` bullet **annotated in place, never removed**.

**Target question atom:**
`.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`
(`kb-open-question-human-readable-encoding-limits-001`). After the wave its
`git diff` against `main` must show **frontmatter hunks only**.

**No wire-format change belongs anywhere in this wave.** The evidence is gathered
in phase 9; the decision is `replication-identity-and-ingest`'s (HS-P0017) and
needs a decision record first.

---

## The verdict, in the shape the measurement produced it

The measurement is
`crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs`, executed on
`wasm32-unknown-unknown` inside `cargo xtask ci`, and it chose one of three
admissible shapes through a three-variant enum rather than through prose:

**(c) The condition is not constructible on this runtime.**

- **The missing element.** A memory ceiling this isolate can be made to reach. The
  staircase asked the host for 2,047 pages and the host **granted every one of
  them**, taking linear memory to 2,169 pages = 142,147,584 bytes — past
  Cloudflare's own documented 128 MiB per-isolate limit — and refused nothing. The
  walk stopped on the probe's own page budget, not on a refusal.
- **Why.** The gate's runner is `wasm-bindgen-test-runner` over Node with a
  `node:sqlite`-backed `DurableObjectState` shim, not `workerd`. A Node isolate has
  no per-isolate memory cap, so the one property this falsifier needs is exactly
  the property the runner does not model. This is a second, independent consequence
  of `every-rule-under-workerd`'s escalated blocking finding rather than a new
  problem.
- **What would supply it.** A real Workers isolate, or a runner flag capping the
  linear memory a test module may grow to.

## The number that changes the question

**At the platform's documented 128 MiB ceiling the arithmetic puts the firing
payload at 36,604,834 bytes** — peak = payload × 11/3, being the payload plus its
≈4/3 rendered string plus the ≈4/3 serialiser output buffer.

That is **thirty-five times** the 1 MiB `MAX_EVENT_DATA_LEN` this adapter's fixture
declares. So on this runtime — even a real one — **no payload this store would
accept can fire the falsifier.** It can only fire on a payload some *other* store
accepted and this peer is asked to **forward**, which is precisely the forwarding
condition WF-11 names, and it stays unmet here.

That sharpens the question's own **sub-question 1** considerably: the adapter *can*
forward a `MIN_SUPPORTED_EVENT_DATA_LEN`-sized payload through the human-readable
path — 65,536 bytes encoded and round-tripped at a cost of five 64 KiB pages — and
it is not forced to fail doing so at any size reachable from this gate.

## The category finding is untouched, and was re-confirmed by measurement

The atom's central claim — serde's data model has no streaming entry point for a
string, so **any** human-readable payload encoding materialises whole — is **not**
weakened by this verdict. It was re-observed at every one of six sizes, and the
published cost table reproduced exactly on `wasm32` from the published seed: at a
348,160-byte payload the human-readable path costs **464,218** bytes and the binary
path **348,163**, matching the figures the atom already carries. The memory claim
holds too: 31 pages against 17 at that size, on identical bytes, with the cheaper
path run first so the comparison is handicapped against the claim.

What changed is only the **peer condition**: it now has a measured answer on the
runtime that was supposed to supply it, and the answer is that this runtime cannot.

## Bearing on the atom's other two sub-questions

- **Sub-question 2** (streaming scheme, declared capability, or JSON scoped to
  diagnostics) — **not reached**, because the falsifier did not fire, and not this
  project's to choose. It belongs to HS-P0017 and needs a decision record.
- **Sub-question 3** (any bearing on ADR-0003's opaque-payload boundary) —
  **none**. The probe forwards bytes it never inspects, which is what makes
  *forward* the right verb; the boundary is observed here, not tested.

## What the resolution must say, and must not

**Must say:** that the condition is not constructible on the runtime the question
assigned it to, and what would construct it. *"We did not see it fire"* is not a
verdict; **(c) is a verdict because it names what is missing.**

**Must not say:** that WF-11 is safe, that its marker moves, that its `MUST`
re-scopes, or anything about what the wire format should become.
