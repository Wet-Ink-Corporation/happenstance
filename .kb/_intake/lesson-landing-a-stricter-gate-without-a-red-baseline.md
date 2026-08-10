# Landing a stricter gate check when the corpus cannot pass it yet

## Where this expects to land

**Layer: none of the three scaffolded ones.** A CI/gate practice is neither a persona, an
interaction pattern, nor an open question. **KB root**, or a `playbooks/` layer if one is
created. **Kind: `playbook`.**

`source_paths`: `xtask/src/spec_trace.rs`, `docs/architecture/SPECIFICATION.md`,
`docs/RUNBOOK.md`, `docs/evaluation/phase-4-5-reconciliation.md`, and this intake file.

## The situation this addresses

You tighten a gate check and the existing corpus fails it in ways you are not authorised to
fix. Here: widening check 6 of `cargo xtask spec-trace` from `suite.rs` alone to all three
entries of `RULE_FILES` (`xtask/src/spec_trace.rs:85-89`) surfaced six conformance rules that
no specification clause claimed and no clause retired. Four were attribution errors — the
clause already stated the proposition and already named the wrong implementation, and only
the rule's name was missing — so they were repaired in the same commit (`52105d2`).

**Two were not errors. They were holes in the specification,** and closing either would have
meant asserting that a `[FROZEN]` clause contains a proposition it does not contain. That is
an ADR's decision, not a checker's. So the check could not go green by repair, and the pass
was not permitted to make it go green by decision.

## The three options, and why two were refused

**Option A — do not land the check until the corpus is clean.** Refused because the clean-up
requires an ADR pass that has not been scheduled, and a check that waits for a decision is a
check that lands after the decision has already been made without it.

**Option B — land it report-only, promote to fatal later.** Refused. A warning in a green run
is invisible within a week; the promotion never gets scheduled, and the interval in which the
check is decorative is unbounded. Worse, report-only *removes the pressure that produces the
decision* — the exact pressure the check exists to create.

**Option C — a ratchet.** Land the check **fatal from day one**, with a named, evidenced,
self-invalidating exemption list holding exactly the entries that cannot be resolved without a
decision.

## The ratchet, and the four properties that make it one

`const UNCLAIMED_PENDING_ADR: [(&str, &str); 2]` (`xtask/src/spec_trace.rs:1968-1997`) is a
list of `(rule name, the decision it is waiting on)`. Its own doc comment states the design:

> It is not an allowlist in the usual sense, because it cannot be used to make a problem go
> away quietly: every entry is printed on every green run, and an entry whose rule *becomes*
> claimed is itself a failure, so the list can only shrink. Deleting the last entry deletes
> the mechanism.

**1. Fatal from day one.** The default arm is a hard failure naming the three ways out — a
clause claims the rule, a clause retires it with `Retires: <rule> — <reason>`, or it goes in
`UNCLAIMED_PENDING_ADR` with the decision it is waiting on
(`xtask/src/spec_trace.rs:782-787`). There is no warning tier, so nothing can accumulate
below the failure threshold.

**2. Every entry prints on every green run.** Not only on failure:

> Printed on a green run, on purpose. An open question that only shows up when something else
> is already broken is an open question nobody reads.
> — `xtask/src/spec_trace.rs:1000-1002`

The two entries are each several sentences long, and both print in full every time the gate
passes. That is deliberate friction: the list is *annoying in proportion to its length*, so
the incentive to shrink it is applied continuously to whoever runs the gate, rather than
episodically to whoever remembers it exists.

**3. An entry that becomes discharged is itself a failure.** If a rule is both claimed by a
clause and listed as pending, the check fails and says "The exemption is discharged — delete
its entry" (`xtask/src/spec_trace.rs:766-775`). The comment beside it names this as "the half
that makes the array a ratchet rather than an allowlist." **Without this half, a stale
exemption is free and the list only grows.** With it, the list can move in exactly one
direction.

**4. Each entry carries its evidence and its owed decision, not just a name.** Both entries
here run to a paragraph: what the rule enforces, which clause looks like it should claim it,
*why claiming it there would be false*, and what an ADR would have to do — "either widening
ES-25 or minting a clause". An exemption entry that is only a name is a mute suppression; one
that is an argument is a work item that has already had its analysis done.

## Why the list carries no count

The array is `[(&str, &str); 2]` and the number 2 appears in the type, but **the doc comment
does not state a count and the output does not hard-code one** — the printed figure is
`unclaimed_pending.len()`, computed at the call site. The comment says why:

> Per the drift allowlist that came before it (`3712c9b`), the count is computed and printed
> rather than written here, so this comment cannot come to disagree with the array beneath it.

That precedent is a commit in this branch's own history titled *"Stop the drift allowlist's
comment from counting its own list"*. **A self-referential number in a comment is a citation
to the thing it sits on, and it rots the same way every other citation rots** — which is the
same defect class the whole pass was repairing. The rule generalises: never write a count of a
list beside the list. Either compute it or omit it.

## Why a tool change and its document change sometimes must be one commit

Commit `52105d2` widened check 6 *and* repaired the four attribution errors in
`SPECIFICATION.md` in a single commit. This is normally bad practice — a tool change and a
content change have different review needs — and here it was forced:

- Widening the check without the repairs leaves the gate **red at that commit**, so the commit
  is not independently buildable and `git bisect` runs into a wall.
- Repairing the document without widening the check makes the repairs **unverifiable at that
  commit** — nothing yet looks at `model.rs` or `concurrency.rs`, so a reviewer has only the
  author's word that the four repairs are the right four and that they are complete.

The general rule: **when a check and the corpus it checks are changed together, they must land
together if and only if either half alone would leave the tree in a state that cannot be
verified.** A green gate at every commit is not a stylistic preference here — it is the only
evidence that the widening found six rules rather than five or seven.

The corollary is that the two exemptions had to be *part of* that same commit. A ratchet
introduced one commit after the check it exempts from is a ratchet with a red commit in front
of it, and the first thing anyone does with a red commit is weaken the check.

## Applicability, and where this is the wrong shape

A ratchet suits a **small, bounded, individually-argued** set of exceptions — single digits,
each owed to a specific pending decision. It does not suit a legacy corpus with hundreds of
violations: there, the per-entry argument is not written, printing every entry on every run is
noise rather than pressure, and the honest instrument is a decreasing threshold count with a
deadline, which is a weaker but survivable thing. The distinguishing question is whether you
can write, for each entry, the sentence *"this is not a defect; it is waiting on <named
decision>"* — and mean it. If you cannot, it is a defect and the list is an allowlist.
