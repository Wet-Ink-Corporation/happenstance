## Phase 14 — Retention, deletion and completeness

> Carried from `RUNBOOK.md:5602-5650` at `f89e184`, verbatim below this note.
> **Edited since the split:** ADR-0028's *decision* moves to phase 17, because one
> of its two answers adds a method to `EventStore` and breaks every published
> adapter — cheap before 1.0 and expensive after. This phase builds what 17
> decides: the suffix store, the rules, and SY-32 across peers.
> **Edited at phase 16:** the `freeze-by-14` clauses from [the 1.0
> dispositions](../ledgers.md) are listed, with an exit criterion; the suffix
> store may already exist in part, because phase 13 needs the same instrument for
> SY-27 (ADR-0066).
> **Edited at phase 17:** [ADR-0028](../../.kb/decisions/0028-what-a-store-may-forget.md)
> took the written refusal, with an additive reservation. The work items below are
> restated against it. What they say at the split is in the archive and in this
> file's history. The suffix store is now an instrument over an arbitrary retained set;
> "the suffix store" in the first note is left as written, as history. The proof
> artefact and exit criterion 2 are restated too: under the refusal the suite is
> expected to *pass* against the instrument, so nothing requires rules to fail there.
> The open question left for this phase is whether a reader needs a report, and a
> report can arrive only as a provided method defaulting to `Unknown`.


**Goal.** Decide what a store is permitted to forget and how it says so — or
refuse the whole area in writing, which is a legitimate answer that silence is not.

**Why here.** It is the last empty far end in the portfolio, and it is the one
where an accidental answer is most likely: four of the six scenarios reach the same
missing primitive from unrelated doors — a pruned slice, a purged cohort, a
compacted peer — and a store that has been deleted from is currently
indistinguishable from a young one at every value in §2.

**Decisions it settles.** ADR-0028. Discharges ES-39, CF-27, SY-32.

**Work**

- [x] ADR-0028. ~~Either a port surface by which a store reports the history it
      does not hold, or an explicit written refusal.~~ **Written at phase 17**: the
      refusal, with an additive reservation. Deletion is out of scope for
      `EventStore` through 1.x, ES-38 is what a deleted-from store may look like, and
      ES-40's vacuous pass is specified. Nothing here adds a required method.
- [ ] #144 · **The completeness instrument** (CF-27). A testkit decorator over any
      `EventStore` that holds an **arbitrary retained set** of its log, a suffix
      or a scattered subset, so that a runner or an ingest path can be run against
      a holed log and its behaviour recorded. It reports what it
      withholds through its **own inherent API**, never through an `EventStore`
      method. Run the full suite against it and record the pass list. Its rule,
      `instrument_report_is_accurate_and_the_suite_cannot_tell` (renamed at phase
      17 from `suffix_store_is_distinguishable_from_a_young_store`, whose name
      said the opposite of what the refusal specifies), asserts two things:
      the suite passes, so indistinguishability at the port is the specified
      outcome, and the instrument's report is accurate.
- [ ] #147 · **A removal capability on `Fixture`**, defaulted to declined on
      `MID_BATCH_FAULT`'s precedent. Through it, a fixture removes events outside
      the port with a raw `DELETE` on SQLite, Cloudflare, Postgres and Neon; the
      name is this phase's. Cloudflare's decline reason, or its support, must say
      whether a Durable Object wiped by `delete_all()` is the same store (VT-6).
- [ ] #151 · `positions_are_not_reused_after_removal` (ES-38), against the instrument
      and against every real adapter through that capability. Its named wrong
      implementation is a SQLite table declared `INTEGER PRIMARY KEY` without
      `AUTOINCREMENT`, which reuses the deleted tail's highest rowid.
- [ ] #154 · `condition_over_removed_history_does_not_reject` (ES-40), an asserted and
      documented outcome: the condition passes vacuously. ~~A condition over
      destroyed history must refuse rather than pass.~~ ADR-0028 rejected that
      third outcome for 1.x.
- [ ] #156 · **The reader experiment** (ES-39). Write an ingest path and a projection
      runner against the instrument, and record whether either needs a port-level
      report. If one does, add it as a **provided** `EventStore` method whose
      default answers `Unknown`, never `Complete`, in ES-4's spelling (see
      `experiments/provided-method-spike/`), with the rule
      `a_store_reports_the_history_it_does_not_hold`. If neither does, freeze ES-39
      as the refusal.
- [ ] #158 · Retention across the peer set (SY-32): the scalar floor means the lowest
      resume point a peer can satisfy, not completeness. Build the 120-day-offline
      against 90-day-window case and show the gap is reported, not silent.
- [x] Redaction (E2E-49). **Answered by ADR-0028**: a tag cannot be redacted
      through the port, and a crypto-shred of `data` moves no position.
- [ ] #163 · **The clauses phase 16 gave this phase to freeze.** Each `freeze-by-14` row
      in [the 1.0 dispositions](../ledgers.md). All four are *decided* at phase 17,
      in ADR-0028, and *built and frozen* here against the suffix store:
      - **ES-39** — the rules of ADR-0028's written refusal (ES-38's and ES-40's),
        or `a_store_reports_the_history_it_does_not_hold` if the reader experiment
        adds the provided method. If this phase slips past 1.0, the row may become
        renew-past-1.0, because firing it is additive.
      - **ES-40** — frozen with its rule against the instrument. ADR-0028 kept the
        *MAY admit*, so nothing flipped in `0.4.0`.
      - **SY-32** — the 120-day-offline case, with the instrument standing in for
        the compacted peer, against the floor ADR-0028 narrowed to resumability.
        The marker's owner text was reconciled with the ledger at phase 17.
      - **CF-27** — the instrument itself, as a decorator over any `EventStore`
        run through the full suite, with the rule
        `instrument_report_is_accurate_and_the_suite_cannot_tell`. If phase 13 built
        the filtered-subset store for SY-27, this phase extends that one rather
        than writing a second.

**Proof artefact.** ~~The suffix store, committed, with a runner and an ingest path
that both fail against it — plus the rules that catch them. A store that lies about
its own completeness is the one wrong implementation nothing in the workspace can
currently detect.~~ **Restated at phase 17, against ADR-0028's refusal:** the
completeness instrument, committed, with the full suite's recorded pass list
against it, and the reader experiment's recorded outcome: whether the runner and
the ingest path written against it need a port-level report. Under the refusal no
port value reports completeness, so no store can lie about it at the port; the
wrong implementation left to catch is an instrument whose own report misstates
what it withholds.

**Exit criteria**

- [x] ADR-0028 written, and it either adds a port surface or refuses the area in
      writing. It refuses, at phase 17.
- [ ] ~~The suffix store exists and at least two rules fail against it.~~
      **Restated at phase 17:** the completeness instrument exists, the full
      suite's pass list against it is recorded, and its inherent report is
      accurate. ES-38's `positions_are_not_reused_after_removal` and ES-40's
      `condition_over_removed_history_does_not_reject` run against it and against
      every fixture that grants the removal capability, and ES-38's rule rejects
      its named wrong implementation.
- [ ] ES-39, CF-27 and SY-32 are no longer `[DEFERRED]`.
- [ ] Every `freeze-by-14` clause in [`ledgers.md`](../ledgers.md)'s 1.0
      dispositions — ES-39, ES-40, SY-32 and CF-27 — is `[FROZEN]`, or
      re-dispositioned by a record that says why.
- [ ] The specification is reconciled against this phase's changes (session
      protocol step 6), and `cargo xtask spec-trace` passes.

**Cases this makes writable.** E2E-44, E2E-46, E2E-47, E2E-48, E2E-49.

**Estimate.** 5 days.

**Session log**
