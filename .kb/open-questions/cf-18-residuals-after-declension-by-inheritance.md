---
id: kb-open-question-cf-18-residuals-after-declension-001
title: What declension by inheritance leaves open under CF-18
kind: open_question
status: accepted
authority_tier: note
summary: >-
  kb-decision-0051 discharged the costed half of CF-18's B3 brief — a rule that fails when a
  capability is declined without an accompanying declaration turned out to describe a state
  Capability::declined cannot construct, and what landed instead is declension by inheritance:
  five capabilities default to a declension carrying the testkit's own prose, so silence in a
  fixture prints as an account of that adapter's store. CF-18's MUST is unchanged and stays
  FROZEN. Three things the brief raised are still open. A stranger's default cargo test still
  prints no SKIP line for a declined capability — declension by inheritance guarantees a reason
  exists, not that libtest surfaces it without --show-output — so CF-18's own Rejects paragraph,
  turning on a reviewer seeing the reason "in the adapter's CI log," is still unmet outside this
  repository. ES-35's PROVISIONAL marker rests on that same unobservable mechanism and was not
  re-read. And whether an adapter's README claiming conformance must state its declined
  capabilities is a documentation obligation nobody has assigned an owner.
depends_on:
  - kb-decision-0051
related:
  - kb-decision-0042
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/cf-18-observable-skip-reporting.md
  - .kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md
last_reviewed: 2026-09-07
---

# What declension by inheritance leaves open under CF-18

## What is true today

CF-18 (`spec/SPECIFICATION.md`, `[FROZEN]`) requires a rule whose capability
requirement is unmet to be emitted as a test that reports the skip with the
fixture's stated reason, and its `Rejects:` paragraph turns on that reason
landing "in the adapter's CI log where a reviewer and a user of the adapter
can both see it." The brief that asked for a machine-checked enforcement of
that obligation (`cf-18-observable-skip-reporting.md`, candidate B3) described
a test that "fails if any capability is declined without an accompanying
declaration" — a state that cannot be constructed, because `Capability::declined`
already asserts a non-empty reason and `Capability::SUPPORTED` *is* the absence
of one. A rule written to those words would have passed on every fixture that
will ever exist: the decorative-rule failure `CLAUDE.md` warns about, arriving
from a direction nobody was watching.

`kb-decision-0051` records what was built instead: five capabilities now
default to a declension whose words are the testkit's own, so a fixture that
declines nothing about a capability still prints this crate's prose in an
adapter's CI log, as if it were that adapter's own account of its store. That
closes the gap CF-18's `Rejects:` paragraph named — a reviewer now sees
*something* — without touching CF-18's `MUST`, which is unchanged and stays
`[FROZEN]`, and without disturbing ES-35's `[PROVISIONAL]` marker, which rests
on the same mechanism this decision left alone.

## What is not decided

Three residuals, none of them discharged by `kb-decision-0051`.

1. **The observability gap CF-18 was originally written against is still
   open.** `cargo test` against a fixture that declines a capability, run with
   no flags, still prints zero `SKIP` lines — declension by inheritance
   guarantees a *reason exists* on the fixture's constants, not that libtest
   surfaces it to a default run. A stranger who clones an adapter and runs
   `cargo test` sees a green suite and nothing else; the reason is reachable
   only under `--show-output`, exactly as it was before this decision.
2. **ES-35's `[PROVISIONAL]` marker was never re-read against the new
   mechanism.** It names the same reporting gap this decision addresses one
   layer up (the *content* of a decline, not its *visibility*), and nobody has
   checked whether declension by inheritance changes what would falsify it.
3. **Whether an adapter's README claiming conformance must state its declined
   capabilities** is a documentation obligation the CF-18 brief raised and
   explicitly left for "whoever owns publication." No such owner has been
   named.

## What forces it

The next adapter author who reads a green `cargo test` from a third-party
`happenstance` adapter and treats it as evidence every rule ran — CF-18's
named victim is exactly that reader, and residual 1 means the victim still
exists. Publication of a fifth or sixth adapter (`happenstance-postgres`,
`happenstance-neon`) forces residual 3, since a README claiming conformance
without stating declines is the artifact the question is about.

## Ordered sub-questions

1. Is residual 1 worth a second mechanism (an `#[ignore]`d emission, or a
   documentation-only narrowing of CF-18 to "reachable under a flag"), or does
   declension by inheritance's CI-log improvement count as sufficient
   discharge on its own?
2. Does ES-35 lift, narrow, or stay `[PROVISIONAL]` once its falsifier is
   reread against declension by inheritance?
3. Who owns the README-disclosure question — the same lane that owns
   `standards/rust/` publication guidance, or a new clause in §6.6?
