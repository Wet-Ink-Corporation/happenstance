# Is CF-18's reporting obligation discharged by something a stranger's default `cargo test` can observe?

**Lane:** suite self-reports (`L1-3`, mechanism half). **Written by the lane that
corrected the prose half, in the same session, and without the author → two-critic
→ revision pass the original thirteen briefs had.** Nobody independent argued the
other side. Read it with that discount applied.

**Confidence:** high on the diagnosis, medium on the recommendation.

**Semver:** the prose correction is none and is landed. The mechanism half is
**not** none: no passing libtest test can emit to a default run, so any mechanism
that puts the skip in front of a stranger changes what an adapter's suite *does*,
which is a change to the bar. `spec/SPECIFICATION.md` is explicit that *"the
contract's is a promise about types, the testkit's is a promise about the bar."*
`happenstance-testkit` is live at `0.2.0-alpha.1`; **`0.2.0` is the last cheap
moment for the mechanism half.**

## The question

CF-18 (`[FROZEN]`) requires a rule whose capability requirement is unmet to be
emitted as a test that **reports** the skip with the fixture's stated reason, and
its `Rejects:` paragraph turns on that landing *"in the adapter's CI log where a
reviewer and a user of the adapter can both see it."*

In a third-party repository neither half of that is present. Is the obligation
discharged by a mechanism a stranger's default `cargo test` can observe, or is
the clause narrowed to what libtest permits?

## Why it is owed — measured, not argued

A rule that skips is a test that **passes**, and libtest discards a passing
test's stdout. Against `happenstance-sqlite`'s fixture, which declines
`MID_BATCH_FAULT` and `READ_FAULT`:

```console
$ cargo test -p happenstance-sqlite --test conformance | grep -c SKIP
0
$ cargo test -p happenstance-sqlite --test conformance -- --show-output | grep -c SKIP
3
```

(The audit filed this as two; `READ_FAULT` landed the same day and made it
three.)

Exactly one CI in the world passes that flag — this repository's, at
`xtask/src/main.rs`, which says why. The machine-checked half of the obligation,
`mutation_coverage::capability_skips_are_reported`, asserts on `RuleOutcome`
*values* rather than on stdout and lives in this crate's own `tests/`, which
never runs in an adapter's CI. The file says so itself.

The named victim is not the adapter author. It is the person who chose an
adapter on the strength of a README saying it passes the happenstance conformance
suite, over a fixture that legally declined `REOPEN` and `MID_BATCH_FAULT`, and
who finds out when an acknowledged write is not there after a restart.

## What is landed, and what it deliberately is not

The three reader-facing surfaces — the crates.io front page, the docs.rs module
page, and `Capability::declined`'s rustdoc — now name `--show-output` beside the
promise, and `a_promised_skip_line_names_the_flag_it_needs` requires the caveat
wherever a surface puts printing within 120 characters of *stated reason*.

**That is a correction of four sentences and settles nothing.** It makes the
documentation agree with `RuleOutcome::report`'s own measurement, which was
already right. It does not put a skip in front of anybody.

## Option A — narrow the clause to what libtest permits

CF-18 keeps the emission requirement (a skipped rule is still a test, not a
`#[cfg]`-ed-out absence) and drops or qualifies the reporting half: the reason is
*reachable*, under a flag, rather than *reported*.

- **Costs an adapter author:** nothing changes.
- **Costs a user of an adapter:** the status quo, made honest. A green run is not
  evidence that every rule ran, and the clause would say so.
- **Costs the specification:** CF-18 is `[FROZEN]`, so this is a new ADR
  superseding whatever froze it. ES-35's `[PROVISIONAL]` marker depends on the
  same mechanism and would need re-reading.
- **The argument for it:** it is the only option that costs nothing and lies
  about nothing. The obligation as written was never met outside this repository
  and pretending otherwise for another release is worse than narrowing it.

## Option B — a mechanism a default run can observe

Candidates, each with a real cost:

1. **A skipped rule fails, unless the fixture opts in.** Turns a legal decline
   into a red build by default. Rejected on sight: it makes an honest volatile
   fixture unable to pass, which is the incentive inversion
   `reopen_over_claiming_is_undetectable_and_this_is_the_record` already measured
   running backwards.
2. **Emit skipped rules as `#[ignore]`d tests.** libtest prints `N ignored` in a
   default run and lists the names under `--list`. The count is visible with no
   flag; the *reason* still is not, and an ignored test is conventionally "not
   run yet" rather than "declined for a stated reason".
3. **One extra emitted test per suite that fails if any capability is declined
   without an accompanying declaration file** — an assertion over the fixture's
   own constants rather than over stdout, which is the shape
   `capability_skips_are_reported` already has and which would run in the
   adapter's CI if it were emitted by the macro rather than written in this
   crate's `tests/`. This is the candidate worth costing first.
4. **Print to stderr instead of stdout.** libtest captures both, so this buys
   nothing; recorded so nobody spends an afternoon on it.

- **Costs an adapter author:** for candidate 3, one more test in their binary and
  a red build the day their fixture declines something without saying why —
  which is the point.
- **Semver:** minor on `happenstance-testkit`, and breaking in practice, which
  this changelog already treats as the rule for that crate.

## Recommendation and the strongest argument against it

**Neither, until candidate 3 is costed.** The honest position today is that the
prose is corrected and the clause is known to overreach. Choosing A before
costing B3 would narrow a `[FROZEN]` clause on the strength of one measurement of
libtest, and B3 is the candidate that would let the clause stand as written.

**The strongest argument against waiting:** `0.2.0` is the last cheap moment for
the mechanism half, and a decision deferred past it is a decision made for A by
default — with the clause still saying something untrue in the meantime. If the
ADR pass cannot reach B3 before `0.2.0`, take A, say why, and leave B3 as the
open question rather than shipping the overreach for another cycle.

## Cost of delay

**Asymmetric, and the asymmetry runs the wrong way.** Delaying the prose fix cost
nothing and it is landed. Delaying the mechanism decision past `0.2.0` converts
an additive change into a breaking one on a crate whose minor bumps are already
breaking in practice.

## What this does not settle

- ES-35's `[PROVISIONAL]` marker, which rests on the same mechanism.
- Whether `capability_skips_are_reported`'s assertion could be macro-emitted at
  all without dragging the mutation-coverage harness into an adapter's build.
  That is the first thing candidate 3 has to answer and this brief does not.
- Whether an adapter's README claiming conformance should have to state its
  declined capabilities. That is a documentation-obligation question in VT's
  family and belongs with whoever owns publication.
