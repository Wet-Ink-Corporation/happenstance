# CF-25 and CF-26 name a portfolio check that does not exist. Is it built before `ProjectionStore` freezes, or does the freeze name the axis it accepts risk on?

Short answer up front: **build it — it is a tenth check over documents
`spec_trace.rs` already parses, and the alternative route CF-25 permits costs an
ADR whose content would be "the workspace decided not to machine-check its own
freeze bar".** But the decision is not this lane's: it belongs with
`projection-store-freeze` (HS-P0010), and the check cannot be written without one
reading that only the specification's owner can supply.

**This brief did not get the author → two-critic → revision pass the original
thirteen had.** Read it with that discount.

---

## Why this is owed

Two `[FROZEN]` clauses assert an instrument that is absent.

CF-25 gates every port freeze in the document and names its own checker:

> Rule: `cargo xtask spec-trace` (CF-38), which reads the portfolio table and the
> maturity markers and fails when a `[FROZEN]` port clause has an axis with no
> far-end row and no named risk acceptance.

CF-26 names the same mechanism against §6.5's `Far end exists?` column:

> Rule: the portfolio table's `Far end exists` column, checked by
> `cargo xtask spec-trace` (CF-38).

Measured over `xtask/src/spec_trace.rs` at `9b06836`, and again after this lane's
two new checks landed:

```console
$ grep -ci "portfolio" xtask/src/spec_trace.rs   -> 0
$ grep -ci "far end"   xtask/src/spec_trace.rs   -> 0
$ grep -ci "axis"      xtask/src/spec_trace.rs   -> 0
```

Nothing reads §6.5. The clause that gates the next port freeze asserts that
something does.

The deadline is real and it is not `0.2.0`. `ProjectionStore` is the port queued
for freeze; PS-2 gates it; PS-12, PS-23, PS-24 and PS-38 are `[PROVISIONAL]` on
axes whose far ends are unbuilt. Whoever runs that freeze gets a green
`spec-trace` and a clause telling them the portfolio bar was checked. It was not
checked; it was asserted.

---

## What is true today

§6.5's table has seven axes and the column CF-26 names. Read out of the document:

| Axis | `Far end exists?` |
|---|---|
| Position allocation | Fixture yes, adapter no |
| Transport | No |
| Async flavour | Fixture yes, adapter no |
| Batch shape (`ProjectionStore`) | Far end yes, **near end no** |
| Completeness | No, and nothing is planned |
| Handle multiplicity | Fixture yes, adapter yes |
| Durability | Fixture yes, adapter yes |

The prose beneath the table states its own arithmetic — *"Seven axes, and the
adapter column carries three ticks"*, *"Two axes are empty at both ends"*, *"Two
carry a fixture instrument and no adapter one"* — and nothing checks that
arithmetic against the column, which is the same defect §1.3's census had before
`check_stated_census` was written.

---

## The question

**Does the portfolio check get built before `ProjectionStore` freezes?**

- **Option A — build the whole of CF-25.** For each port, for each axis it sits
  on, if the port's gating clause is `[FROZEN]` then the axis must have a far end
  or the clause must name the ADR accepting the risk.

  Cost: it needs two mappings the document does not carry in machine-readable
  form — **which clauses are "port clauses"**, and **which axes each port sits
  on**. Both are readings of the specification. §6.5's `Batch shape` row is
  explicitly scoped `(ProjectionStore)`; the other six are not scoped to a port at
  all, and `Async flavour` plainly applies to both. Supplying that mapping is an
  edit to `spec/SPECIFICATION.md` (a machine-readable `Ports:` cell, say) or a
  table in `xtask` that duplicates a reading — and a duplicated reading is the
  shape this whole remediation exists to remove.

- **Option B — build CF-26's half only.** Parse the table; hold the `Far end
  exists?` column to §6.5's own prose census (seven axes, three adapter ticks, two
  empty at both ends, two fixture-only), exactly as `check_stated_census` holds
  §1.3. Then CF-25's half is a separate, later decision.

  Cost: it does not gate a freeze. It does stop the table's verdicts drifting
  away from the paragraph that summarises them, which is how the bar quietly
  improves on paper. Rejects: an axis's cell edited from `No` to `Fixture yes,
  adapter yes` with the summary paragraph left as it was.

- **Option C — the freeze names the axis it accepts risk on, and the ADR that
  accepts it.** CF-25's second permitted route, taken by hand.

  Cost: it discharges CF-25 for *this* freeze and leaves the next one in exactly
  the same position. It is also the only route that is free today.

**Recommendation: B now, and C or A at the freeze.** The strongest argument
against doing B first is that it is the *cheap* half and doing it may read as the
clause having been discharged — CF-26 would then be checked and CF-25 still not,
while `spec-trace`'s green looks the same. That argument is strong enough that B
should land with its summary line naming what it does **not** check, in the same
sentence, the way this lane's `UNRESOLVABLE_RULE_NAMES` and `CF36_UNDISCHARGED`
counts are printed on every green run.

---

## Why this lane did not take it

Option B is inside `xtask/` and would have been in scope. It was not taken
because the honest version of it holds the table to §6.5's *prose*, and the two
must agree at the moment the check lands — so if the prose and the column already
disagree, the fix is an edit to `spec/SPECIFICATION.md`, which another lane holds
for the duration of this remediation. Landing a check that cannot be made green
without a file you may not touch is how a gate step gets a skip list.

Option A additionally needs a reading nobody has taken.

---

## Cost of delay

Priced by CF-25 itself, at the `ProjectionStore` freeze. That is earlier than
first publish and earlier than every other deadline in this brief directory
except `append-condition-sql-shape.md`'s.

---

## What this does not settle

Which route the `ProjectionStore` freeze takes. Whether §6.5 acquires a
machine-readable port column. Whether the `Batch shape` row's one-sided tick —
far end green, near end holding nothing that has run anything — counts as an axis
with a far end for CF-25's purposes, which is the reading that decides whether
that freeze passes or names an acceptance.
