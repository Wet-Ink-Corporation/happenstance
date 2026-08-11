# Governance layer — rules about how this corpus is run

Rules about the KB's own operation: `governance` atoms, typically carrying
`authority_tier: guideline`. This is not where architecture commitments live — that is
[`../decisions/`](../decisions) — it is where the corpus's rules *about itself* live: how an
accepted record may be corrected without becoming a different record, who may accept or retire
an atom, and any rule that is itself mechanically checked rather than merely a norm someone has
to remember.

## What belongs here

- A rule about how an atom, once written, may later be edited — the discrimination between a
  repair and a reversal, applied consistently across kinds rather than reasoned about fresh each
  time.
- A rule about who may accept, supersede, or retire an atom, and under what authority.
- A rule that a check in this repository enforces (`redkiln validate --kb`, a gate step) —
  governance atoms are the natural place to document what a check enforces and why, so the check
  and its rationale do not drift apart.

## What does not belong here

**An architecture commitment.** *Must*, *shall*, *must not* about the system itself is a
`decision` atom in [`../decisions/`](../decisions). This layer governs how the record is
maintained, not what the system does.

**A transferable technique with no immutability angle.** A method for testing an interleaving,
or for citing evidence in a long-lived document, is a `playbook` atom in
[`../playbooks/`](../playbooks). The discriminator: would reversing this need a decision, or is
it a procedure someone could simply try differently next time? Governance atoms bind the
corpus's own editing discipline; playbooks bind nothing.

## Why this layer exists

The decisions layer's immutability rule — supersede, never edit — already lives in
`decisions/README.md`, stated once for that layer. But the corpus had to apply that rule to
itself three times within two days of ADRs, on three different kinds of edit (a rename, a
factual correction, a partial reversal), and the right answer differed each time for reasons
that generalise past decisions alone. A rule that is worked out in practice and never written
down gets rediscovered, at the same cost, the next time an editor meets the same fork.
