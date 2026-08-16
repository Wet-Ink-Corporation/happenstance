# Staged: the phase-7 contract defect log

**This is raw material, not an atom.** `redkiln validate --kb` skips
`_`-prefixed directories by design, so nothing here is held to `KbFrontmatter`
and nothing here pretends to be. The frontmatter below is **proposed** — a
starting point for `/redkiln:kb-ingest` to author from and adjudicate, not a
shape this document is asserting.

```yaml
# PROPOSED — for the ingest wave to adjudicate, not to copy verbatim.
proposed_kind: open-question        # or one decision atom per entry; the wave decides
proposed_title: What using the frozen contract revealed, and what each finding is owed
proposed_status: accepted
proposed_authority_tier: reference
source_paths:
  - references/evaluation/phase-7-contract-defects.md
  - .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/defect-log-and-macros-verdict/spec.md
related:
  - .kb/decisions/0009-error-carries-no-send-bound.md   # confirm slug at ingest
  - .kb/playbooks/repairing-a-frozen-clause-without-amending-it.md
pinned_to: 78a2170c1d06bad5eec34915b0b3682f524ec91f
date: 2026-08-16
```

## Why this is staged rather than written as an atom

Hand-writing a `.kb/` atom produces *"the directory layout of the process
without the process"*, which is why the first attempt at it was reverted at
`0269720` (CLAUDE.md, *Where the work lives*). The claims below are offered for
adjudication. The wave decides shape, supersession and links; this document
decides none of them.

**The long form is not a duplicate and must not be discarded.** A successful
ingest clears `_intake`, and a ~100-line atom cannot carry the call site, the
attempted code and the contract's actual response — the part that lets a later
reader judge whether a defect was real. That detail lives in
`references/evaluation/phase-7-contract-defects.md`, which is immutable and is
never cleared. Every atom this document feeds should cite it.

## The claims

Five findings, each already carrying a clause ID and a routing. Full text, with
the call sites and the proposed fixes, is in the long form.

### C1 — no infallible `QueryItem` constructor for pre-validated inputs

Bears on **VT-18** `[FROZEN]` (`spec/SPECIFICATION.md:1374-1386`). Every derived
query carries a `Result` that is unreachable for a well-formed model, because
`QueryItem::new` is fallible even when the caller holds already-validated
`EventType`s and `Tags` (`crates/happenstance-core/src/query.rs:48-62`).

**Stronger than first recorded:** `Boundary` is sealed, so the error arm this
creates is **untestable from outside the crate** — no downstream type can be a
failing `Boundary`.

Wanted from the wave: a decision record. The proposed fix (an infallible
constructor for pre-validated inputs) may well be right; the claim being staged
is that it is a decision with alternatives and a record, not a line edit.

### C2 — `DomainEvent::tags` is infallible over a fallible `Tags`

Bears on **VT-18** `[FROZEN]`, and is D-1's other face. `fn tags(&self) -> Tags`
is total; every route into `Tags` is fallible. An implementor whose tag values
are runtime strings therefore has **no total path** except to invent a newtype
holding the validated `Tag` — which the worked example does, at a cost of 81
lines (`examples/course-subscriptions/src/main.rs:102-182`, and its own doc says
so at `:104-109`).

Wanted from the wave: a decision record. Note the coupling to the macros verdict
below — if this is settled with an infallible `Tags` path, the identity newtypes
shrink and AC-013's measurement should be re-taken.

### C3 — CF-36 names a cross-reference nothing performs

Bears on **CF-36** `[FROZEN]` (`spec/SPECIFICATION.md:8611-8622`). Its `Rule:`
line claims `cargo xtask spec-trace` cross-references each case's level marker;
`grep -c "Level" xtask/src/spec_trace.rs` returns **0**. A green `spec-trace`
currently reads as evidence for CF-36 and is not.

Wanted from the wave: a decision record choosing between implementing the
cross-reference and superseding CF-36. Both are decisions; neither is a patch.

### C4 — no `PS` rule name is resolved; §7.2's `†` is redundant

Bears on **CF-38** `[FROZEN]`, with PS-27 and PS-30 as the visible symptom.
`spec_trace.rs`'s check 4 short-circuits on `!has_suite(&c.id)` (`:695-697`), so
no `PS` rule name is resolved at all. Separately, the `†` *must be written*
marker is redundant with the clause's own maturity marker and names no owner,
where the clause does.

Wanted from the wave: a decision record, and possibly a note that the `†`
convention is superseded by maturity markers carrying owners.

### C5 — `&mut P` makes N projections cost N reads, at the API level

Bears on the **projection port family**, `[PROVISIONAL]` — a port with no
conformance suite. Recorded as a **shape**, not a wrong answer: the runner's own
defence (an owned write set cannot be shared, and one failure policy for every
projection would be wrong) is sound for the alpha. It is staged so the eventual
tail-seam decision meets a measurement already on the record.

Wanted from the wave: evidence attached to whichever open question owns the
tail-seam decision. **Not** ES-32's disposition, which belongs elsewhere.

## Two findings that are deliberately *not* claims

Staged so the wave does not read their absence as an omission.

**N1 — `run_projection` cannot be spawned without a caller-side bound. Not a
defect.** ES-6 `[FROZEN]` leaves `Error` unbounded on purpose and ADR-0009
already assigns the obligation to the caller in terms, supplying the
marker-trait shape and explicitly declining to ship it in `happenstance-core`.
The promise was kept; what was missing was that nobody had written the
consequence down where a caller meets it, and that is now on `run_projection`'s
own page (`crates/happenstance/src/runner.rs:297-310`). **If the wave wants
anything here**, it is at most an amendment to ADR-0009's atom recording that
the marker has now been exercised by a real consumer — never a new decision.

**N2 — `read_through` is dead code in eight `wasm32` feature combinations. No
clause ID, therefore `support`.**
`crates/happenstance-core/src/projection_memory.rs:233`; CI's ambient
`RUSTFLAGS: -D warnings` makes it a failure there. Pre-dates phase 7. Classified
`support` at the moment of finding rather than promoted by inventing a clause
citation. The hand-off is owed as a `redkiln new` invocation; this PR does not
make it.

**A third, which is neither a defect nor support: a missing rule.** A private
module can shadow a glob-re-exported one — `mod projection;` in `happenstance`
silently shadowed `happenstance_core::projection` through `pub use
happenstance_core::*;`, producing only a `hidden_glob_reexports` warning.
`_design.md`'s anti-pattern 14 is written about *type* names; this arrived
through a *module* name. Proposed destination: a candidate rule in
`standards/rust/`, not `.kb/`.
