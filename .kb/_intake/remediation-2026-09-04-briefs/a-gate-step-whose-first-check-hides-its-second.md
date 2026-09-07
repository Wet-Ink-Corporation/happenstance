# Three unrelated checks share one gate step and short-circuit on the first `?`. Does each get its own step, or does the step collect?

Short answer up front: **yes, this belongs to the "checks stronger than
mechanism" family, and it is the same shape one level up — the step named
`every stated rule count matches the suite` can report a failure that is not a
rule count and never look at a rule count at all.** The cheapest correct fix is
to collect the three checks' problems rather than `?` on the first; the tidier
one is three steps.

This lane was asked to judge whether the ordering is a finding of its group. It
is, and it bit this lane twice in one session.

**This brief did not get the author → two-critic → revision pass the original
thirteen had.** Read it with that discount.

---

## Why this is owed

`lints::stated_rule_counts` (`xtask/src/lints.rs:1084`) is the body behind the
`REQUIRED` step named *"every stated rule count matches the suite"*, and it runs
three unrelated checks in sequence:

```rust
    runbook_status_matches_the_registry()?;
    citation_ranges_resolve()?;   // V-6
    // ... and only then the rule-count census
```

Each `?` short-circuits. So a V-6 citation failure — a `path:line` in `.kb/`,
`docs/` or `examples/` that lands on a blank or mid-construct line — aborts the
step before the rule-count census runs, and the step's *name* is the rule-count
one.

The comments on both bundled calls say honestly why they are there: adding a
`Step` to `main.rs` and a match arm beside it, or making the function
`pub(crate)`, would have obligated `xtask/src/affected.rs`'s export scan, and
neither file was that change's to edit. The bundling was the right call under
that constraint. The short-circuit is the part nobody chose.

---

## What is true today, measured in this session

**It bit twice.** Both times the first check failed and the second and third
never ran.

1. This lane's S-5 change moved `xtask/src/spec_trace.rs` by about four hundred
   lines. Four `.kb/` citations landed on blank lines. `cargo xtask lints` exited
   1 with V-6's message, and the rule-count census — `6 stated rule count(s)
   checked against 1 (model.rs), 5 (concurrency.rs), 17 (projection.rs), 93
   (suite.rs), 116 (all four rule files)` — did not print. The step that reported
   the failure is called *"every stated rule count matches the suite"*.

2. The same thing happened again after a follow-up commit moved two more.

The parent brief for this lane records a third occurrence earlier the same day,
against a citation into `lint_constitution.rs`.

**And the failure is worse than a wrong label.** The census is the check that
holds four *documents* — the testkit's README, both `lib.rs` front pages and
`happenstance-core`'s feature comment — to the suite's real rule counts, and its
own module documentation records that those four told crates.io the projection
suite was two rules of seventeen through the fifteen commits that made it
seventeen. That is the check that goes dark behind a stale line number.

---

## The question

**Does each bundled check get its own gate step, or does the step collect?**

- **Option A — collect.** Run all three, accumulate their problems, and fail once
  naming all of them. Cost: the three return `Result<()>` and print their own
  summaries, so collecting means each returns its problems instead — a signature
  change in three functions and their call sites, inside one file.
  Benefit: no check can hide another, and the step's message names every failure
  in one run rather than one per run.

- **Option B — three steps.** `lint-rule-counts`, `lint-runbook-status`,
  `lint-citations`, each with its own `Step`, match arm and `lint_steps()` entry.
  Cost: `affected.rs`'s export scan and its `unconditional_block` assertions must
  gain all three — which is *correct* and is that mechanism working, and is
  exactly what this lane did for `lint-enumerations` without difficulty. Benefit:
  each check is separately runnable and separately named, and `cargo xtask lints`
  prints three rows instead of one mislabelled one.

- **Option C — rename the step.** Cheapest, and wrong: it fixes the label and
  leaves the hiding.

**Recommendation: B.** The strongest argument against it is that it inflates
`REQUIRED` for three checks that always run together and always run fast, and
that Option A gets the load-bearing half — no check hides another — for a
smaller diff. That is a fair argument, and B wins on one thing A does not buy:
`cargo xtask lints`' printed rows are what `print_help` uses to tell a reader
what the gate checks, so a check with no row of its own is a check a reader
cannot find. This lane's `lint-enumerations` was added as a full step for that
reason and the `affected.rs` cost was three lines.

**A and B compose**; if B is taken, each new step should still collect rather
than `?` on its first problem, because `citation_ranges_resolve` itself scans 121
files and reports every bad citation in one pass — the discipline is already
there one level down.

---

## The second, larger finding underneath it

V-6 fails on a **blank** first line and on a **mid-construct** one. It does not
fail on a citation that lands on a plausible line in the wrong construct. This
lane found three such citations while repointing:
`subject_before` was at `spec_trace.rs:2248` and the playbook and the phase-8
census cited it at `:2104-2144`, `:2116-2118` and `:2158` — every one of them
inside `BARE_NAME_MAP`, a hundred lines short, and all three green.

`lint-constitution` has the stronger instrument for the same problem: an **anchor**
(a phrase that must still appear within 10 lines of the cited line). Extending
V-6 to derive an anchor the way `spec_trace::subject_before` already does for
`SPECIFICATION.md`'s citations would have caught all three. That is a separate
change with its own cost — `citation-anchor-slack.md` in this directory is
already about the 10-line window — and it is named here because the two are the
same defect at different strengths.

---

## Cost of delay

Low in the sense that nothing is unguarded; high in the sense that it has
happened three times in one day and each time cost a debugging round-trip and,
once, a green report of a step that was red.

---

## What this does not settle

Whether V-6 gains anchors. Whether `stated_rule_counts` keeps its name if the
bundling is unwound.
