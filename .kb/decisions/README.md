# Decisions layer — the record this design rests on

The architecture decision records: `decision` atoms carrying `authority_tier: decision`. This
is where the ADR corpus lives. Each atom carries `adr_id` (`ADR-0001` …), `reversibility`, and
the `phase` that owned it.

## The immutability rule

**An accepted decision is never edited.** `redkiln validate --kb` checks each `status: accepted`
decision atom against `HEAD` and fails the gate on a changed body. To correct one, write a new
atom carrying `supersedes: [<old id>]`, and flip the old atom's frontmatter to
`status: superseded` + `superseded_by: <new id>`. That metadata flip is the only edit an
accepted decision ever receives; its body is never reworded.

This is not a redkiln convention imposed on the corpus — it is the corpus's own rule, finally
enforced. ADR-0002 is fully superseded, ADR-0005 and ADR-0006 partly, and ADR-0004 is amended
by ADR-0029, and all four are kept verbatim precisely because the crate names and constraints
in older commits only make sense with them.

A **repair** and an **amendment** are different, and the distinction is mechanical: a
correction is a repair if the set of implementations the decision admits is unchanged.
Otherwise it is a gap, and a gap is a new decision's. Filing an amendment as a repair is how a
frozen commitment quietly moves.

## What belongs here

Anything that commits the project — *must*, *shall*, *must not*. A new architecture commitment
is a decision atom, never a `playbook` or a `concept`, because those two carry no immutability
and a commitment that can be edited is not a commitment.

State the alternatives that lost and why. A decision recorded without its rejected options is
indistinguishable from an accident, and the next person to meet the same fork has no way to
know it was a fork.

## What does not belong here

**The evidence.** The measurement or compilation a decision rests on is a `reference` atom that
this one cites. Separating them is what lets a decision be superseded without invalidating the
evidence underneath it.

**A decision not yet taken.** That is an `open_question` in
[`../open-questions/`](../open-questions/). Filing a live question here gives it an authority
nobody granted it.

**Current truth.** A decision record is *history* — why a choice was made, and when.
`docs/architecture/SPECIFICATION.md` says what is true **now**, and where the two disagree, the
specification wins. An ADR is never updated to match the code.

## Why this layer exists

The rule these records were always written under — supersede, never edit — was prose in a
README, and prose in a README is enforced by whoever remembers it. Here it is checked against
`HEAD` on every run of the gate.
