---
id: kb-open-question-gate-step-first-check-hides-001
title: One gate step bundles three unrelated checks and stops at the first failure
kind: open_question
status: accepted
authority_tier: note
summary: >-
  lints::stated_rule_counts, the body behind the REQUIRED step named "every stated rule count
  matches the suite," runs three unrelated checks in sequence with ? after each: a runbook-status
  check, then V-6's citation-range resolution, then the rule-count census the step is named for.
  Measured three times in one working session, a V-6 citation failure aborted the step before the
  census ran, so the step that reported the failure was named for a check that never executed —
  and the census is the one check that holds four documents (the testkit README, both lib.rs front
  pages, happenstance-core's feature comment) to the suite's real rule counts, so it is exactly
  the check that must not go dark behind an unrelated failure. The bundling itself was a
  deliberate, documented trade against obligating an unrelated file's export scan; the
  short-circuit ordering inside it was not chosen by anyone. What is not decided is whether the
  fix is to collect all three checks' problems in one pass or to split the step into three named
  ones, and both routes are argued with a real cost attached in the source brief without either
  being landed.
depends_on: []
related:
  - kb-playbook-anchoring-citations-001
  - kb-playbook-ratchet-gate-landing-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/a-gate-step-whose-first-check-hides-its-second.md
last_reviewed: 2026-09-07
---

# One gate step bundles three unrelated checks and stops at the first failure

## What is true today

`lints::stated_rule_counts` (`xtask/src/lints.rs:1084`) is the body behind the
`REQUIRED` gate step named "every stated rule count matches the suite," and it
runs three unrelated checks in sequence, each guarded by `?`:
`runbook_status_matches_the_registry()?`, then `citation_ranges_resolve()?`
(V-6), and only then the rule-count census the step's name describes. Each
`?` short-circuits the function on the first `Err`, so a failure in either of
the first two checks means the third — the one the step is named for — never
runs, and the failure that is reported carries the wrong step's name.

This was measured three times in one working session, not argued from the
code alone. Twice, a citation landing on a blank line after a ~400-line file
move made `cargo xtask lints` exit 1 with V-6's message while the rule-count
census — which had counted 116 rules across four files that day — never
printed. A third occurrence, against a different citation, is recorded in the
brief this question was raised from. The consequence is sharper than a
mislabelled failure: the census is the check that holds four separate
documents (the testkit crate's README, both `lib.rs` front pages, and
`happenstance-core`'s feature comment) to the suite's actual rule counts, and
its own module documentation records that those four documents once told
crates.io the projection suite had two rules of seventeen through fifteen
commits that made it seventeen. That is the exact check a stale citation
elsewhere in the repository can now silently switch off.

The bundling itself is not the defect and is not up for relitigation here:
each function's own comment explains that giving it a `Step` entry in
`main.rs`, or making it `pub(crate)`, would have obligated
`xtask/src/affected.rs`'s export scan — a file the change that added these
checks was not scoped to edit. That constraint was real and the bundling was
the correct response to it. What nobody chose is the short-circuit order
inside the bundle.

## What is not decided

Which of two repairs the step takes. **Collect**: run all three checks,
accumulate their problems instead of returning on the first, and report every
failure the step found in one pass — a signature change in three functions
and their call sites, contained in one file, with the benefit that no check
can hide another and a reader sees every failure a single run turned up.
**Split**: give each of the three its own `Step`, match arm, and
`lint_steps()` entry — correctly extending `affected.rs`'s export scan to
cover all three, the same mechanism this remediation pass already exercised
without difficulty for an unrelated new step — with the benefit that
`cargo xtask lints`' printed rows, which `print_help` uses to tell a reader
what the gate checks, gain three named rows instead of one mislabelled one.
The two are not mutually exclusive: if split is chosen, each of the three new
steps should still collect rather than `?` on its own first problem, since
`citation_ranges_resolve` already does exactly that internally, scanning 121
files and reporting every bad citation in one pass rather than the first.

A narrower, related finding sits underneath this one and is also unresolved:
V-6 fails on a citation landing on a blank or mid-construct line, but not on
one landing on a plausible line inside the *wrong* construct — three such
citations were found in the same session, all inside `BARE_NAME_MAP` roughly
a hundred lines from where they claimed to point, and all green.
`lint-constitution`'s anchor mechanism (a phrase required to still appear
within ten lines of the cited line) already solves this for a different
citation family; whether V-6 gains the same mechanism is a second, separable
question this brief also leaves open.

## What forces it

Recurrence: the ordering bug has already cost three debugging round-trips in
one day, one of them a green report standing in for what was actually a red
step. Nothing about it is scheduled to a phase.

## Ordered sub-questions

1. Collect, split, or both — and if split, does `stated_rule_counts` keep its
   name once the bundling it describes is unwound?
2. Does V-6 gain anchor derivation the way `spec_trace::subject_before`
   already has for `SPECIFICATION.md` citations, closing the wrong-construct
   gap alongside the ordering one?
