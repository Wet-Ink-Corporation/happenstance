# Playbooks layer — transferable practice

How to do a thing, written so that someone who was not there can do it: `playbook` atoms
carrying `authority_tier: guideline`. A playbook binds less than a decision and more than a
note — it is what this repository learned works, and the reasoning that makes it portable.

## What belongs here

A practice that generalises past the case that produced it:

- a **method** with a stated claim, the measurement or compilation that supports it, and the
  conditions under which it stops holding;
- a **procedure** whose steps have an order that matters, with the failure mode each step
  prevents;
- a **technique** that was arrived at by discarding cheaper alternatives, with those
  alternatives named and the reason each lost.

The last part is what separates a playbook from a summary. A playbook that lists only the
approach that won reads as arbitrary, and the next reader re-derives the rejected options at
full cost before trusting it.

Ground it. Cite the file, the command, the compiler error, the number — and put those paths in
`source_paths`. This repository's standard of evidence applies to its own practice: a claim
about how to work is settled against something that compiled or something that was measured,
not against how the work felt.

## What does not belong here

**A commitment.** If it says *must*, *shall*, or *must not* — if a future change would need a
decision to reverse it — it is a `decision` atom in [`../decisions/`](../decisions), and
filing it here strips it of the immutability that makes it enforceable.

**A one-off.** Something true of exactly one file, one clause, or one afternoon is either a
`reference` atom (if it records a state worth citing) or nothing at all. The test is whether
the claim survives being read by someone working on a different part of the system.

**An explanation.** A playbook tells you what to do. An atom that tells you how something
*works*, without prescribing anything, is a `concept`.

## Why this layer exists

The expensive part of a lesson is not noticing it — it is the pass that found it. A repository
that records only outcomes pays that cost again every time the same shape recurs in a new
place, and it recurs constantly: the check that verified an address instead of a referent, the
gate that could not be landed against a corpus that could not pass it, the frozen clause that
needed repair and not amendment. Each was found once, at real cost, and each generalises.
