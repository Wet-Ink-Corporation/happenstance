# F2-5's residual holds `0.2.0` for phase 10, against the review's own verdict on its class

Record: **F2-5-HOLD**. Staged for `/redkiln:kb-ingest`. Decided by the repository
owner at the `0.2.0` release pass, 2026-09-06.

**The decision runs against the recommendation it was given, and against the
pre-publication review's own verdict. That is recorded here rather than smoothed
over, because a decision whose counter-argument is not written down is
indistinguishable later from one that never had one.**

---

## What F2-5 was, and what is left of it

The audit filed F2-5 as two claims about
`dropped_append_future_leaves_no_partial_batch`:

1. one of its two arms has never executed, and
2. the rule has been certified against nothing but a strawman.

**The first is false.** Arm 2 — the byte-identical
`snapshot_of(&after) == snapshot_of(&before)` comparison — is reached by two
registered stores on every run of `tests/mutation_coverage`, in both feature
configurations, and has been since the rule's first commit. The lane sent to build
"a store that suspends before its first write" found two already in the tree and
stopped rather than adding a third. The measurement is in
`es-22-arm-two-is-reached-the-finding-is-wrong.md` and is mechanical.

**What survives is one sentence:**

> `dropped_append_future_leaves_no_partial_batch` has never been answered by a
> store with a **real medium** under it. Both arms execute and both are decisive
> against the instruments in `tests/mutation_coverage`, but every store that has
> ever reached either arm is an `Rc<RefCell<…>>` in the testkit's own test target,
> where *"the future was dropped"* means a local went out of scope rather than a
> connection was severed.

Plus a smaller, separable one: arm 2 is exercised by two mutants and by no
`Kind::ConformantVariant`, so nothing in the portfolio would catch arm 2
**over**-specifying. Whether CF-5's conformant-control obligation runs per rule or
per *branch* is undecided, and ES-22 is the first place the difference is visible.

## The decision

**`0.2.0` waits for phase 10.** The store that answers the residual is
`happenstance-postgres` — the first adapter in this workspace with a connection
that can be severed rather than a `RefCell` that can be dropped.

## The argument against it, in the strongest form available

Three sources, none of them sympathetic to the decision:

1. **The review that raised F2-5 says its whole class costs the same later.**
   *"The asymmetry is one-shot rather than semver: no fix here is a breaking
   change, and every one of them is available at the same price after `0.2.0`.
   What cannot be re-run is the release."*
   (`references/evaluation/review-pre-publication-2026-09-03.md:863`.)

2. **`RUNBOOK.md`'s own sequencing put phase 10 off the trunk deliberately.** Its
   design contribution was spent in phases 2, 4 and 6; it was a branch that never
   rejoins, and this decision makes it rejoin.

3. **The cost is concrete.** Phase 10 is `not started` and estimated at eleven
   days for two adapters. Four finished crates and an entire release pass wait
   behind it, and the repository stays private for that period because the owner
   also chose to hold publication of the repository until the crates ship.

## The argument for it

One, and it is not weak: the testkit is the artefact this project exists to be
trusted. A conformance suite whose atomicity rule has only ever been proved
against an `Rc<RefCell<…>>` is asking adapter authors to trust a guarantee about
severed connections on the evidence of a dropped local. CLAUDE.md's own standing
rule is that a port is only as well-designed as the *spread* of what implements
it, and names this exact failure — four adapters, one storage shape. F2-5 is that
rule applied to a conformance rule rather than to a port.

Publishing is what makes the suite's verdict cost something to be wrong about.

## What this does not settle

- **The CF-5 sub-question** — per rule or per branch — is not decided here. It
  rides with the residual and is owed before ES-22's control set is called
  complete.
- **Whether phase 10 actually discharges it.** `happenstance-postgres` must
  *reach* both arms with a severed connection; building the adapter is necessary
  and may not be sufficient. If it turns out a real medium cannot reach arm 2
  either, that is a finding about the rule rather than about the adapter, and the
  hold will have bought exactly the knowledge it was taken for.
- **The fallback.** Accepting the residual in writing and publishing remains
  available at any point. Taking it later would be a decision, not a drift.

## Related

- [[ratifications-2026-09-06-pre-publication]] — the seventeen briefs decided in
  the same pass, whose window this decision widens rather than closes.
- [[es-42-marker-earned-off-at-0-2-0]] — the clause-level gate, which was cleared.
- `.kb/open-questions/es-17-two-adapter-measurement-is-unscheduled.md` — the other
  place a two-adapter measurement is owed and unscheduled.
