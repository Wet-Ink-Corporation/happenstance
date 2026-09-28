## Phase 14 — Retention, deletion and completeness

> Carried from `RUNBOOK.md:5602-5650` at `3916f29`, verbatim below this note.
> **Edited since the split:** ADR-0028's *decision* moves to phase 17, because one
> of its two answers adds a method to `EventStore` and breaks every published
> adapter — cheap before 1.0 and expensive after. This phase builds what 17
> decides: the suffix store, the rules, and SY-32 across peers.


**Goal.** Decide what a store is permitted to forget and how it says so — or
refuse the whole area in writing, which is a legitimate answer that silence is not.

**Why here.** It is the last empty far end in the portfolio, and it is the one
where an accidental answer is most likely: four of the six scenarios reach the same
missing primitive from unrelated doors — a pruned slice, a purged cohort, a
compacted peer — and a store that has been deleted from is currently
indistinguishable from a young one at every value in §2.

**Decisions it settles.** ADR-0028. Discharges ES-39, CF-27, SY-32.

**Work**

- [ ] ADR-0028. Either a port surface by which a store reports the history it does
      not hold, or an explicit written refusal: deletion is out of scope for
      `EventStore`, and here is what a store that has been deleted from is permitted
      to look like. Both close E2E-46 and E2E-47; only one of them adds API.
- [ ] **The suffix store** (CF-27) — a testkit-adjacent store that deliberately
      holds only a suffix of its own log, so a runner or an ingest path written
      against it fails loudly rather than being accidentally correct. It is small,
      and it is the completeness axis's far end.
- [ ] `condition_over_removed_history_does_not_reject` (ES-40) and
      `positions_are_not_reused_after_removal` (ES-38): an append condition's
      meaning is scoped to the store that evaluates it, so a condition over
      destroyed history must refuse rather than pass.
- [ ] Retention coordinated across the peer set (SY-32): a retention gap is
      reported, not silent.
- [ ] Redaction (E2E-49): whether a tag can be redacted at all, given that
      `Tag` equality is byte equality and every index is keyed on it.

**Proof artefact.** The suffix store, committed, with a runner and an ingest path
that both fail against it — plus the rules that catch them. A store that lies about
its own completeness is the one wrong implementation nothing in the workspace can
currently detect.

**Exit criteria**

- [ ] ADR-0028 written, and it either adds a port surface or refuses the area in
      writing.
- [ ] The suffix store exists and at least two rules fail against it.
- [ ] ES-39, CF-27 and SY-32 are no longer `[DEFERRED]`.

**Cases this makes writable.** E2E-44, E2E-46, E2E-47, E2E-48, E2E-49.

**Estimate.** 5 days.

**Session log**
