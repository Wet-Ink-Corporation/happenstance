# CF-38's fourth condition reads two ways and they differ by 54 cases. Which one is it?

Short answer up front: **this lane implemented "a case no clause claims", which is
the reading §7.6 states and computes, and it is green. The other reading — "a case
body that names a clause" — is what CF-37's own words say, and 54 of 58 cases fail
it. Somebody has to say which of CF-37 and the checker is wrong, and it is not
the implementer.**

**This brief did not get the author → two-critic → revision pass the original
thirteen had.** Read it with that discount.

---

## Why this is owed

CF-38 `[FROZEN]` lists five conditions its checker must fail on. The fourth is:

> a case naming no clause

CF-37 `[FROZEN]` is the obligation behind it:

> Every E2E case MUST name the clause or clauses it exercises.

Read against `spec/E2E-CASES.md`, those two sentences describe a document that
does not exist. Measured in this working tree:

- **58** `### E2E-nn` headings.
- **0** `Clauses:` fields of any kind.
- **54 of 58** case bodies contain no clause identifier anywhere.

CF-37's own `Rejects:` paragraph says why, and says it about itself:

> E2E-CASES.md was written before this specification and therefore names no
> clauses; adding the back-reference is what makes "is every clause exercised by
> something concrete?" and "did this decision leave a case orphaned?" both
> answerable by a command.

So the clause knows the document does not satisfy it. What nobody wrote is the
check, in either reading.

---

## What is true today

This lane landed the **second** reading as `spec-trace`'s check 10: every case in
`E2E-CASES.md` must be claimed by at least one clause's `Cases:` line. It reports
**0 orphans**, and the count is printed on every green run:

```
201 clauses (...), 116 conformance rules, 58 e2e cases (0 claimed by no clause), ...
```

Before it, the checker read `E2E-CASES.md` in one direction only. Demonstrated in
a scratch worktree: deleting `E2E-14` from SY-6's `Cases:` line — SY-6 is the only
clause that claims it, and §7.6 names E2E-14 as one of four single-claim cases —
left `cargo xtask spec-trace` reporting *"traceability: no problems found"* and
exiting 0. It now fails, naming the case.

§7.6 is the reason that reading was chosen and not a convenience. It states the
result, states the method, and states the hazard about itself:

> Single-claim cases are the ones a later edit can orphan without anyone noticing

— E2E-14, E2E-31, E2E-50, E2E-51. The orphan set is empty **today**; it was
unguarded **tomorrow**.

---

## The question

**Which reading does CF-38's fourth condition carry?**

- **Option A — "a case no clause claims".** Landed. Zero failures. Guards the
  hazard §7.6 names. It does **not** discharge CF-37, whose sentence is about
  what a case *body* says.

- **Option B — "a case body that names no clause".** 54 of 58 fail. Discharging
  it means adding a `Clauses:` field to 54 cases — a real edit to a document
  whose whole point is that it predates the specification — or superseding CF-37.
  It is the reading CF-37's words support.

- **Option C — both, with B behind a declared list of the 54.** The shape this
  lane used for CF-36 and for `UNRESOLVABLE_RULE_NAMES`: the breach is recorded,
  counted on every green run, and a 55th fails. Cost: a 54-entry table whose only
  content is "this case predates the specification", which is one fact written 54
  times — the thing `CF36_UNDISCHARGED`'s own docs warn against.

**Recommendation: A is right and should stay; B is a separate obligation and
CF-37 should be repaired or superseded rather than checked.** The strongest
argument against that: CF-38's five conditions are a **floor** (`MUST fail on`),
and reading its fourth as A means the floor names a condition CF-37 does not,
which leaves CF-37 with no instrument at all — exactly the state this remediation
group exists to find. That argument is good, and the answer to it is that CF-37's
instrument cannot be written without deciding whether 54 cases acquire a field;
which is the decision, not the implementation.

The route for the repair is `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`
if the outcome is that CF-38's fourth condition means A, and a superseding clause
if the outcome is that CF-37 wants B.

---

## Cost of delay

Low. A is landed and green, so the live hazard is guarded. What delay costs is
that CF-37 stays a `[FROZEN]` clause with no instrument, and a reader of a green
`spec-trace` has no way to tell that from a discharged one — which is now
slightly worse than before, because the summary line reports `0 claimed by no
clause` and a hurried reader may take that for CF-37.

That last sentence is the strongest reason to settle it soon rather than the
weakest.

---

## What this does not settle

Whether `E2E-CASES.md` acquires a `Clauses:` field. Whether the four single-claim
cases §7.6 names should be given a second claimant instead, which would make the
hazard structural rather than checked.
