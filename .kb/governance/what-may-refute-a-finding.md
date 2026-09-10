---
id: kb-governance-what-may-refute-a-finding-001
title: What may refute a finding, and how an accepted decision's currency is computed
kind: governance
status: accepted
authority_tier: guideline
summary: >-
  The precedence ladder this repository already declares - SPECIFICATION clause above accepted
  ADR above constitution atom above CLAUDE.md above references/evaluation/* - applied to a verb
  it was not written for. standards/rust/README.md:23-43 tells an author which document wins when
  two disagree; this atom tells an auditor what may overturn a finding, and three consequences
  follow that the ladder alone does not give. First, references/evaluation/* and RUNBOOK.md may
  annotate a finding and may never refute one: they are dated evidence rather than rules, and a
  review that lets them refute suppresses true findings using documents the repository itself
  declares non-binding - which is RS-01-3's rule read from the auditing end. Second, a refutation
  needs a quoted sentence answering the same question. A filename is not a refutation and a
  section number is not a refutation, and the absence of a citation in a corpus of roughly forty
  thousand lines is a search failure rather than a verdict. Third, an accepted decision's currency
  is computable rather than a matter of judgement: git log <adr-commit>..HEAD -- <the files the
  decision cites> returns empty when the decision was taken against exactly this code, and it
  refutes; when it returns commits, the decision refutes only if a line at HEAD still implements
  what it decided, and where that line has moved or vanished the correct verdict is that an
  accepted decision may have silently drifted - the highest-value class available and the one a
  naive "the ADR covers this" reading discards. Specification clauses are exempt from the
  computation because cargo xtask spec-trace is a gate step, so their citations resolve at HEAD by
  construction. A fourth follows from two 2026-09 episodes: a refutation is itself a finding and
  is held to the same standard of evidence, so an empirical counter-claim owes a measurement
  rather than a quotation, and the strongest label a brief can attach to a reason - decisive -
  carries the highest burden rather than exempting it. A fifth comes from a 2026-09 episode in
  which the thing needing checking was a retracted predecessor rather than the finding itself.
  HANDOVER.md records an ES-11 escalation made in error and retracted (2e0a0ae) on the claim
  that no asynchronous driver can conform, refuted by happenstance-postgres conforming through a
  runtime handoff at the first poll, and it records that a commit message asserting the falsifier
  had fired is wrong and is history rather than guidance. A later, narrower claim - that a driver
  with no shared ordering primitive between its operations cannot conform - had to be checked
  against that one before it could be accepted, and what separates them is a measurement rather
  than an argument: happenstance-neon already does the thing that refuted the old claim, spawning
  at call time and stating the clause's own sufficiency sentence back at it, and it records that
  the Postgres-shaped Handle::try_current() capture is what it shipped first and why that shape
  fails there. The remedy that killed the predecessor is applied and the failure survives it, so
  the claim is the residual after the old fix rather than the old claim restated
  (kb-decision-0061). Two things generalise. A retracted claim does not poison its own
  neighbourhood: the way to show a narrower successor is not the same claim is to apply the
  earlier refutation's remedy and demonstrate the failure survives, which is a measurement
  obligation and not a rhetorical one. And a claim in a neighbourhood where something was already
  retracted owes secondary checks a first claim does not - here, that the failure is always in one
  direction, and that the defect the clause is mostly about is structurally unreachable in this
  shape.
depends_on: []
related:
  - kb-governance-referent-not-reasoning-001
  - kb-playbook-anchoring-citations-001
  - kb-playbook-verify-referent-report-coverage-001
  - kb-playbook-count-or-index-nobody-re-derives-001
  - kb-decision-0022
  - kb-decision-0040
  - kb-decision-0059
  - kb-decision-0060
  - kb-decision-0061
  - kb-reference-mutation-coverage-arm-two-001
  - kb-open-question-cf-5-per-rule-or-branch-001
  - kb-open-question-adr-0022-falsifiers-fired-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - .kb/_intake/f2-5-holds-the-release-for-phase-10.md
  - .kb/_intake/remediation-2026-09-04-briefs/domain-event-guard-and-decode.md
  - .kb/_intake/remediation-2026-09-04-briefs/es-22-arm-two-is-reached-the-finding-is-wrong.md
  - .kb/_intake/2026-09-08-es-11s-falsifier-fired-on-the-adapter-it-named.md
  - .kb/_intake/2026-09-08-adr-0061-es-11s-sufficiency-condition-assumed-a-queue.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - standards/rust/README.md
  - standards/rust/01-standard-of-evidence.md
  - HANDOVER.md
last_reviewed: 2026-09-09
---

# What may refute a finding, and how an accepted decision's currency is computed

## What this adds to the existing ladder

`standards/rust/README.md:23-43` already states this repository's precedence ladder: a
SPECIFICATION clause outranks an accepted ADR, which outranks a constitution atom, which outranks
`CLAUDE.md`, which outranks `references/evaluation/*`. That ladder answers one question — which
document wins when two disagree about what is true. It was written for an *author* deciding what
to write next. This atom answers a different question the same ladder implies but never states:
what is allowed to **overturn a finding** someone has already made about the tree. The distinction
matters because a review that gets the second question wrong can suppress a true finding while
citing the very ladder that would, read correctly, have supported it.

## Three consequences, derived rather than assumed

**Dated evidence may annotate, and may never refute.** `references/evaluation/*` and
`RUNBOOK.md` sit at the bottom of the ladder by design — they are recorded observations about a
past state of the tree, not rules the tree is held to. An auditor who lets a citation from either
override a fresh finding has inverted the ladder: they are using the repository's own
lowest-authority documents to suppress something that may be true right now. Such documents may
still **annotate** a finding — noting that a prior review already looked at the same area, say —
but annotation is not refutation, and the difference is whether the citation changes what is true
or only adds context to it.

**A refutation needs a quoted sentence answering the same question.** The failure mode this
guards against is citing an artefact rather than its content: a filename, a section number, or "see
ADR-0022" with nothing quoted. None of those is a refutation, because none of them states what the
cited material actually says about the question at hand. In a corpus of roughly forty thousand
lines, failing to find a sentence that answers the question is evidence the search was incomplete,
not evidence the finding is wrong — the two are easy to conflate under time pressure, and only the
first is a defensible verdict.

**An accepted decision's currency is a computation, not a judgement call.** For a finding that
appears to contradict an accepted decision, run `git log <adr-commit>..HEAD -- <the files the
decision's rationale cites>`. An empty result means the decision was taken against exactly the code
that exists now, and it stands as a full refutation. A non-empty result does not settle anything by
itself — it means the cited files moved since the decision was written, and the decision refutes
the finding only if a line at `HEAD` still implements what the decision actually decided. Where
that line has moved or been removed, the correct verdict is **an accepted decision may have
silently drifted from the code it once described** — which is the class of finding the naive
reading ("the ADR already covers this, dismiss it") reliably discards, because that reading stops
at the citation resolving rather than checking what it resolves *to*.

## The first rule's sharpest instance, and it was not a review

The `0.2.0` release pass supplies the strongest case yet for holding `RUNBOOK.md` at the
bottom of the ladder, and what it cost was not a suppressed finding but a deferred release.
The decision to hold `0.2.0` for phase 10 rested on a report that phase 10 was `not started`
and eleven days away, read out of `RUNBOOK.md`'s status table. The row was stale —
`lane/postgres-neon-stores` already held a substantially built `happenstance-postgres`, and
the residual the hold was taken for was discharged the same day
([`kb-decision-0040`](../decisions/0040-f2-5-holds-the-release-for-phase-10.md)). The defect
occurred in the same file whose census that pass was at that moment repairing.

Two things generalise. **The rule is not confined to refutation.** Dated evidence read as
current fact drives a decision as readily as it suppresses a finding, by the same mechanism:
a document recording a past state of the tree gets consulted as though it recorded the
present one. Anything read out of `RUNBOOK.md` or `references/evaluation/*` as an *input* is
owed the re-derivation an auditor owes a citation. **And the judgement was sound.** The
decision was correct on the information available and the information was wrong; keeping
those apart is the value of recording it, because a correction that folds them together
teaches the next reader to distrust the reasoning rather than the referent.

## A fourth consequence: a refutation is itself a finding

Two 2026-09 episodes force the standard of evidence to run in both directions.

**An empirical counter-claim owes a measurement, not a quotation.** The brief on the domain
event guard rejected its Option B on three reasons and labelled the third *decisive*: the
option would fire spuriously against sixteen named-const impls this repository writes itself.
Nobody had measured it, and those sixteen have no call site at all — the guard is called from
three places, which the brief's own *"What this does not settle"* section had enumerated
throughout, and that is what made the error findable. With the counterfactual struck, nothing
remained that carried a rejection and the recommendation flipped
([`kb-decision-0059`](../decisions/0059-the-domain-event-guard-checks-positional-agreement.md)).
The *decisive* label raises the burden on a reason; it does not discharge it.

**Where the question is what the code does, run it.** F2-5's actionable half — that ES-22's
`landed == 0` arm had never executed — was refuted by mutating that arm so anything reaching
it must fail, then watching two registered stores fail it in both feature configurations
([`kb-reference-mutation-coverage-arm-two-001`](../reference/mutation-coverage-arm-two-measurement-2026-09.md)).
The finding looked true because the corpus was read *by axis*, and the store whose name
advertises the axis is not one of the two that reach the arm. Reading found the wrong store;
only running found the right ones.

**A partial refutation restates the residual rather than striking the finding.** Both
episodes overturned one half of a two-part claim and left the other standing, narrower and in
its own words: F2-5's untouched half became the hold above, and the leftover question of
whether a conformant control is owed per rule or per *branch* went to
[`kb-open-question-cf-5-per-rule-or-branch-001`](../open-questions/cf-5-conformant-control-per-rule-or-per-branch.md)
instead of dying with the half that was wrong.

## A fifth consequence: a retracted claim does not poison its own neighbourhood

The fourth consequence checks a finding against the evidence. The ES-11 pair supplies the case
where what needed checking was the **predecessor**: something in the same neighbourhood had
already been claimed, escalated and withdrawn, and the question was whether a new claim there was
the old one wearing different words.

`HANDOVER.md` records an ES-11 escalation *made in error and retracted* (`2e0a0ae`), on the claim
that **no asynchronous driver can conform**; `happenstance-postgres` conforming through a runtime
handoff at the first poll refuted it. It records something else worth naming separately: commit
`0341467`'s message, asserting the falsifier had fired, is **wrong and is history, not guidance**.
A commit message is dated evidence exactly as `references/evaluation/*` is, and the first
consequence above applies to it unchanged — with the sharper edge that nothing supersedes a commit
message, so the correction has to live somewhere a reader will reach first.

The later claim is narrower — **a driver with no shared ordering primitive between its operations
cannot conform** — and the discriminator was a measurement rather than an argument.
`happenstance-neon` already does the thing that refuted the old claim: its conformance transport
spawns at call time, states the clause's own sufficiency sentence back at it, and records that the
Postgres-shaped `Handle::try_current()` capture is what it shipped first and why that shape fails
there. **The remedy that killed the predecessor is applied and the failure survives it**, which is
what makes this the residual after the old fix rather than the old claim restated
([`kb-decision-0061`](../decisions/0061-es-11s-sufficiency-condition-assumed-a-queue.md)).

Two things generalise. **The way to show a narrower successor is not the same claim is to apply
the earlier refutation's remedy and demonstrate the failure survives it** — a measurement
obligation, not a rhetorical one, and the same demand the fourth consequence makes of a refutation,
now made of a claim standing where one was already withdrawn. **And such a claim owes secondary
checks a first claim does not.** Two were run here: the failure is always in one direction (the
read sees the *later* append, never an earlier state), and the paging defect ES-11 is mostly about
is structurally unreachable in this shape, because one read is one statement. Each is cheap, and
either coming out the other way would have meant a bug rather than a clause defect — which is the
shape the first escalation took.

The companion decision in the same pass shows the discipline from the other side:
[`kb-decision-0060`](../decisions/0060-ps-2s-axis-re-evaluated.md) reaffirms a gate whose *reason*
had expired, and refutes each driver PS-2 names by an independent mechanism rather than by one
argument covering both. A claim about an axis is checked against the tree at the axis, one
mechanism at a time.

## The one exemption, and why it needs none of this

Specification clauses do not need the git-log computation applied to them. `cargo xtask spec-trace`
is a standing gate step that re-verifies every clause's citations against `HEAD` on every run, so a
clause's references are current by construction rather than by an auditor's separate check — the
guarantee this atom asks an auditor to build by hand for a decision is already built into the gate
for a clause.
