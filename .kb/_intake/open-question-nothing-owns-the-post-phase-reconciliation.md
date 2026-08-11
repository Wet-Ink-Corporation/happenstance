# Open question — nothing owns the specification reconciliation at a phase's exit

## Where this expects to land

**Layer: `open-questions/`.** This is the one scaffolded layer this wave populates, and it fits
without forcing: a known gap the work uncovered and did not close, with what is actually true
today. **Kind: `open_question`. `authority_tier: note`** — it binds nothing.

`source_paths`: `RUNBOOK.md`, `spec/SPECIFICATION.md`,
`xtask/src/spec_trace.rs`, `references/evaluation/phase-4-5-reconciliation.md`, and this intake file.

Per the layer README, add a bullet for the resulting atom on whichever `map` atom indexes the
specification/governance area once one exists.

## What is true today

`spec/SPECIFICATION.md` holds 200 numbered clauses describing what is true of the
contract now. Phases 4 and 5 were, by the runbook's own account, "the two largest changes to
the contract in the plan." Neither carried an item obliging anyone to read the specification
back against the tree it had just changed.

The consequence was measured in August 2026 and is recorded at
`RUNBOOK.md:3777-3842`, under the heading **"Between 5 and 6 — the reconciliation nothing
owned"**, which opens:

> Not a phase. A pass that had to happen and that this plan had not scheduled, recorded here so
> the next one is scheduled rather than noticed.

Sixteen clauses stated things about the code that were false. Nine documentation MUSTs that
`[FROZEN]` clauses impose on `happenstance-core`'s own doc comments had never been discharged.
Five tests that clauses name did not exist. 254 of 338 citations had never been parsed by the
gate step that was supposed to check them.

**The rule that would have caught this was already written down, in the same file, and nothing
implements it.** From the end of the ADR queue, quoted verbatim
(`RUNBOOK.md:334-336`, restated at `:3284-3286` and again at `:3806-3807`):

> **The rule this leaves behind, for every later phase:** a phase's clause range and the union
> of its ADRs' clause ranges are two numbers, and nothing checks that they are equal. Compute
> both at the phase's exit.

The reconciliation pass responded by writing a standing exit criterion into the runbook
(`RUNBOOK.md:3810-3820`), applying to "Every phase from 6 onward, before its box is
ticked":

- [ ] Every clause the phase's ADRs discharge has been read against the code as it now stands,
      not as it stood when the clause was written. A clause whose supporting prose describes a
      superseded implementation is a defect even when its MUST is untouched.
- [ ] The phase's clause range and the union of its ADRs' clause ranges are computed and
      compared.
- [ ] `cargo xtask spec-trace`'s citation count has not fallen, and any clause the phase froze
      names a rule that exists or is marked `†`.

## What is not decided

**The criterion exists as prose in one section of the runbook and is attached to no phase and
to no tool.** Three things are open, and they are separable:

**(a) Does it become an item in each phase's own `Work` list?** Phase 6's section
(`RUNBOOK.md:3846` onward) enumerates its work as checkboxes and does *not* carry these
three. A standing rule stated once, several thousand lines above the phase that must obey it,
is a rule with the same enforcement profile as the one that already failed — which was also
written down, three times, and observed zero times.

**(c) Who computes the two numbers, and against what?** "A phase's clause range" is stated in
each phase's *Decisions it settles* line as prose (phase 6: "Discharges PS-1 – PS-37"). "The
union of its ADRs' clause ranges" is stated in each ADR. Neither is in a machine-readable
position today, so the comparison is a human diff of two prose statements — which is a
comparison that will be skipped precisely when the phase is large, which is when it matters.

**(b) Is any of this mechanisable in `cargo xtask spec-trace`?** The third bullet nearly is:
the tool already prints the citation count, so "has not fallen" needs a stored baseline rather
than new analysis, and `†` handling already exists (`xtask/src/spec_trace.rs:1104`,
`:1168`). The second bullet becomes checkable if phase and ADR clause ranges are written in a
parseable form. The first bullet is a reading task and probably cannot be mechanised at all —
which is an argument for mechanising the other two, so that the human effort lands where only
a human can spend it.

## What forces it

**Phase 6's exit.** Phase 6 freezes `ProjectionStore` and discharges PS-1 through PS-37 —
thirty-seven clauses, more than any prior phase, in the layer that already carries two
recorded MUST-versus-rule gaps (PS-1 and PS-19; see `gaps-owed-a-decision.md`). If the exit
criterion is not attached to something before phase 6 closes, the experiment runs once more
with the same setup, and the next reconciliation pass will be larger than this one.

A secondary forcing event is **first publish at phase 12**, when the specification stops being
an internal document and becomes a promise to downstream consumers. A clause that describes a
superseded implementation is a documentation defect today and a support burden then.

## Ordered sub-questions

1. **Does the standing criterion become a per-phase checkbox, or a gate step, or both?**
   Answer this first: it determines whether the remaining questions are about wording or about
   code.
2. **If a gate step: where does the baseline live?** "The citation count has not fallen" needs
   a committed number, and a committed number is a self-referential count — which
   `xtask/src/spec_trace.rs:1965-1967` records as its own known failure mode ("the count is
   computed and printed rather than written here, so this comment cannot come to disagree with
   the array beneath it"). Whatever form the baseline takes must not repeat that.
3. **Are phase clause ranges and ADR clause ranges made machine-readable?** Small, and it is
   what turns sub-question (c) from a diff of two paragraphs into an assertion.
4. **What is the obligation when the two numbers disagree?** The runbook says "computed and
   compared" and does not say what a disagreement obliges. A comparison with no consequence is
   a comparison that stops being run.
5. **Does this become an ADR, or is it runbook process only?** It changes no contract clause
   and admits no new implementation, so it is arguably process. But it is the *third* place
   the same rule is written down, and the previous two changed no behaviour — which is itself
   evidence that prose is the wrong instrument here.

## Why this is a question and not a task

There is a real decision inside it — mechanised gate step versus per-phase checklist versus
both — with a genuine tradeoff: a gate step cannot check the bullet that matters most (has a
human read the clauses against the code), and a checklist has already been demonstrated not to
survive contact with a large phase. Filing it as a backlog item would smuggle that decision
into whoever picks up the ticket.
