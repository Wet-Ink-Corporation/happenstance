# CF-36 has been checked for the first time and thirteen clauses are in breach. Which of the three repairs does each take?

Short answer up front: **nine of the thirteen are one fact and take one repair;
two are a `Cases:` line that cites the wrong case; two are open until the
projection runner lands.** All thirteen are recorded in `CF36_UNDISCHARGED`,
counted on every green run and reconciled so the list can only shrink — but every
repair is an edit to `spec/SPECIFICATION.md`, which is not the implementing lane's
to make.

**This brief did not get the author → two-critic → revision pass the original
thirteen had.** Read it with that discount.

---

## Why this is owed

CF-36 `[FROZEN]`:

> A clause backed only by **integration**- or **scenario**-level cases MUST NOT
> name a conformance rule.
> Rule: `cargo xtask spec-trace` (CF-38), cross-referencing each case's level
> marker (E2E-CASES.md:19-28).

Measured at `9b06836`: `grep -c "Level" xtask/src/spec_trace.rs` returns **0**,
against 58 `- **Level:**` markers in the document the clause says this file
cross-references. `.kb/open-questions/cf-36-names-a-cross-reference-nothing-performs.md`
(`kb-open-question-cf-36-unperformed-cross-reference-001`) is dated `2026-08-17`
and names first publish as its second forcing event.

The check now exists. Its first run found thirteen clauses in breach:

```
13 clause(s) name a conformance rule and cite no contract-level case (CF-36),
each recorded in `CF36_UNDISCHARGED`: VT-21, WF-9, PS-29, PS-30, SY-2, SY-4,
SY-5, SY-7, SY-9, SY-10, SY-24, SY-25, SY-34
```

The document's 58 cases carry three level values: 41 `contract`, 9
`integration`, 8 `contract (sync)`. There are no `scenario`-level cases.

---

## The thirteen, in three groups

### Group 1 — the replication family (nine): SY-2, SY-4, SY-5, SY-7, SY-9, SY-10, SY-24, SY-25, SY-34

Each names a rule that would live in `happenstance-sync-testkit`, and cites only
integration-level cases (E2E-39, E2E-42, E2E-44, E2E-45). CF-36's own `Rejects`
paragraph names that crate as the right home:

> it belongs in the e2e crate, or — for a contract-level case that cannot be
> expressed against a single store handle — in `happenstance-sync-testkit`, named
> at `crates/happenstance-sync/src/lib.rs:23-24` and not yet existing.

So these nine may be **exactly what CF-36 describes as correct** — rules that
belong in a crate that does not exist — while breaching the sentence, which
forbids the clause from *naming* them at all. That is one question asked nine
times, and it should be answered once.

### Group 2 — a `Cases:` line that cites the wrong case (two): VT-21, WF-9

These are the two worth looking at first, because they name **live rules in
`suite.rs`**:

- VT-21 names `store_accepts_the_guaranteed_minimum_payload` and
  `append_reports_exceeded_store_limits`, against **E2E-42 alone** — transitive
  convergence across a peer mesh.
- WF-9 is the same shape and shares one of the two names.

Both rules are single-store and run against one handle; the case is not. The
mismatch reads like a `Cases:` line, not like a misplaced rule.

### Group 3 — open until the projection runner lands (two): PS-29, PS-30

Both cite E2E-28 (integration) and both name rules that do not exist —
`one_poisoned_projection_does_not_stall_the_others` and
`panicking_apply_rolls_back`, declared `Scheduled` in
`UNRESOLVABLE_RULE_NAMES`. Whether either can be expressed against a single
projection-store handle is a question the runner answers.

---

## The question

**For each group, which of CF-36's three repairs applies?** The repairs are: give
the clause a contract-level case; move the rule name out of the clause; or
supersede the clause.

- **Group 1** — one decision for nine clauses. The candidate answer is that
  naming a rule for a suite that does not exist yet is what §7.2's `†` convention
  exists for and CF-36 did not contemplate, in which case CF-36 wants a
  qualification (a rule named for a *named, unbuilt* suite is not a rule it
  forbids) and the repair is a superseding clause rather than nine edits. The
  argument against: that qualification is exactly the escape hatch that lets any
  clause name any rule by promising a crate.
- **Group 2** — most likely a `Cases:` line repair, which is an edit to two
  clauses and does not touch a `[FROZEN]` sentence.
- **Group 3** — leave recorded until the projection runner lands; the entries name
  that as their trigger.

---

## Why the record is a table and not a skip list

`CF36_UNDISCHARGED` is reconciled in both directions on every run: a clause that
stops breaching CF-36 fails the gate as a stale entry, a recorded clause the
document stops declaring fails the same way, and a **fourteenth** breach fails
because it is not recorded. So the list can only shrink and cannot be used to
make a new problem quiet. Its contents are printed on every green run beside
`UNCLAIMED_PENDING_ADR`'s, for the reason that comment gives: a cost that only
appears when something is already broken is a cost nobody prices.

The alternative was to leave CF-36 unimplemented for another phase, on the
grounds that landing it with thirteen recorded breaches is landing a check with a
skip list. That reading is wrong in one specific way: unimplemented, the
fourteenth breach is as invisible as these thirteen were, and nobody would have
had this list at all.

---

## Cost of delay

Priced by CF-36's own open question at first publish. Group 2 is cheaper than
that and should not wait for it: two `Cases:` lines citing a peer-mesh case for
single-store rules is a factual error in a `[FROZEN]` clause, visible on
docs.rs's rendering of §7.2.

---

## What this does not settle

Whether a rule named for a named-but-unbuilt suite is a rule CF-36 forbids.
Whether the eight `contract (sync)` cases are contract-level for CF-36's purposes
— this lane reads the level as the **first word**, so they are, and that reading
is what the check runs on. If they are not, group 1 grows.
