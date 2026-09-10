---
id: kb-governance-referent-not-reasoning-001
title: Rewrite the referent, never the reasoning
kind: governance
status: accepted
authority_tier: guideline
summary: >-
  How this corpus edits a decision record that is already signed, refined by a chain that had to
  apply it to itself three times in two days. The immutability rule says supersede rather than
  edit; this is the discrimination it needs in practice. A rename that preserves meaning may be
  rewritten in place - ADR-0001, ADR-0003, ADR-0004 and ADR-0007 had their crate names rewritten
  to happenstance-core at phase 0 rather than left as period spelling under a note, because
  renaming an identifier is not reversing a decision. Reasoning inside a decision that still
  stands is never touched. And a superseded decision's body is left factually intact even where
  it is now wrong about the world: ADR-0002 keeps every mention of eventum, because rewriting
  them would convert a true claim about a crates.io registration into a false one, and the value
  of a superseded record is that it says what was true when the choice was made. The test has the
  same shape as the repair-versus-amendment test one layer down: ask whether the edit changes
  what the document asserts, not whether it changes the document. A fourth instance, nine phases
  later, exercises a case none of the first three covers: the decision stands and its reason
  expires. ADR-0036 declined to freeze the projection port because exactly one adapter had run
  the projection suite, at one end of PS-2's batch-shape axis; by phase 11 four had, and phase
  11's pre-registered condition did not fire, so that sentence is now false about the tree while
  remaining exactly what ADR-0036 found. ADR-0060 reaffirms the decision - the port keeps its
  unstable-projection gate - and replaces the reason, because freezing begin, probe_write and
  probe_read_through would make a semver promise out of precisely the signatures that forbid the
  second batch shape. The two are recorded separately and ADR-0036 is neither edited nor
  superseded, on this atom's own ADR-0002 reasoning one layer up: rewriting "only one adapter
  has run the suite" into "four have" would convert a true claim about the state of the tree into
  a false claim about what a decision found, the same conversion rewriting eventum into
  happenstance would have made. What the case adds is that a reason can expire independently of
  the decision it supported, and that the honest record of an expired reason is a new atom rather
  than a repaired one - a decision whose stated reason has been quietly updated can no longer
  explain why it was taken.
depends_on: []
related:
  - kb-decision-0002
  - kb-decision-0005
  - kb-decision-0006
  - kb-decision-0007
  - kb-decision-0036
  - kb-decision-0060
  - kb-playbook-repair-frozen-clause-001
  - kb-playbook-one-decision-per-adr-title-001
  - kb-open-question-ps-32-adr-0007-correction-owed-001
  - kb-decision-0031
  - kb-decision-0023
  - kb-playbook-declared-page-need-001
  - kb-governance-what-may-refute-a-finding-001
  - kb-decision-0037
  - kb-open-question-dagger-convention-vs-maturity-markers-001
  - kb-open-question-references-adr-correction-policy-001
source_paths:
  - .kb/_intake/0005-rename-to-happenstance.md
  - .kb/_intake/0006-bare-name-to-the-typed-layer.md
  - .kb/_intake/0007-projection-runner-decodes.md
  - .kb/_intake/2026-09-08-adr-0060-ps-2s-axis-re-evaluated.md
  - references/adr/0002-crate-naming.md
  - references/adr/0006-bare-name-to-the-typed-layer.md
  - CONTRIBUTING.md
  - .kb/decisions/README.md
last_reviewed: 2026-09-09
---

# Rewrite the referent, never the reasoning

## The rule

`.kb/decisions/README.md` states the corpus's immutability rule: an accepted decision is never
edited; a correction becomes a new atom that supersedes it. That rule has one blind spot — it
does not say what to do when an edit does not correct the *decision* at all, only the *name*
something in it refers to. This atom is the discrimination the immutability rule needs in order
to be applied without either freezing every stale mention forever, or licensing rewrites that
change what a record asserts.

## The test

Ask whether the edit changes what the document **asserts**, not whether it changes the document.
A rename that preserves meaning may be rewritten in place. Reasoning inside a decision that
still stands is never touched, however dated its argument now reads. And a superseded decision's
body is left factually intact even where it is now wrong about the world, because the value of a
superseded record is that it says what was true when the choice was made. And a *reason* can
expire while the decision it supported still stands — which is a rewrite the test forbids just as
firmly, and for the same cause one layer up.

## Four worked instances — three from one week, and one nine phases later

**Rewritten in place.** ADR-0001, ADR-0003 and ADR-0004 all named the contract crate
`eventum-core` when written. When ADR-0005 renamed the project to `happenstance` and ADR-0006
moved the bare name to the typed layer, those crate names were rewritten to `happenstance-core`
— not left as period spelling under a note. Every sentence in them was always a statement about
*the crate that defines the ports*; the rename moved the string on the front of that crate and
left the referent exactly where it was. Rewriting the name preserves the meaning; leaving it
stale would have preserved the spelling and inverted the meaning, and ADR-0003 is the case that
proves it rather than merely illustrating it — left untouched, it would forbid `serde` to
whatever crate is called `happenstance`, which after ADR-0006 is the *typed* layer, the crate
whose entire job is `serde` encoding. Rewriting it is what keeps the constraint attached to the
crate it was always about.

**Left verbatim under a superseded banner.** ADR-0002 recorded that the bare name `eventum` was
taken on crates.io by an unrelated, dormant crate. When ADR-0005 superseded it, ADR-0002's body
kept every mention of `eventum` rather than being rewritten to `happenstance`. Rewriting it would
convert a true claim about a real crates.io registration into a false one about `happenstance`'s
availability — the opposite of what ADR-0005 actually found. `CONTRIBUTING.md` already requires
superseding an ADR rather than rewriting one; ADR-0005 applies that rule to ADR-0002, and ADR-0006
applies it again to ADR-0005 in turn.

**Reasoning stands, only a bundled sub-decision reverses.** ADR-0006 is the case where only part
of a body could later be touched, and the corpus chose a metadata flip over a rewrite even there:
ADR-0007 corrected ADR-0006's projection-runner allocation, and ADR-0006's frontmatter gained a
"partly superseded by" note while its body — including the naming argument that still stands —
stayed verbatim. Nothing about *why* the bare name went to the typed layer was rewritten; only
the record of what still binds changed, and it changed as metadata, not as prose.

**The decision stands and its reason expires.** [ADR-0036](kb-decision-0036) declined to freeze
the projection port at phase 6 because exactly one storage adapter had run
`projection_store_conformance!`, at one end of PS-2's batch-shape axis. By phase 11 four had —
a file, a pooled server, a one-shot HTTP proxy and an embedded graph database — and phase 11's
pre-registered condition, *"the batch needs a field the port cannot express"*, did not fire. The
sentence *"exactly one adapter has run the suite"* is therefore false about the tree, and is
still exactly what ADR-0036 found. [ADR-0060](kb-decision-0060) reaffirms the decision — the port
keeps its `unstable-projection` gate — and replaces the reason: freezing `begin`, `probe_write`
and `probe_read_through` would make a semver promise out of precisely the signatures that forbid
the second batch shape. ADR-0036 is neither edited nor superseded, and the two reasons are
recorded separately, on this atom's own ADR-0002 reasoning one layer up. Rewriting *"only one
adapter has run the suite"* into *"four have"* would convert a true claim about the state of the
tree into a false claim about what a decision found — the same conversion that rewriting `eventum`
into `happenstance` would have made, applied to the reason rather than the referent.

## Why the discrimination has to be explicit

Without it, an editor facing a stale crate name has two bad defaults: leave everything alone,
which lets true renames rot into confusing period spelling across a growing corpus, or rewrite
freely, which is how a superseded decision quietly loses its ability to explain why it was
reversed. The referent/reasoning line is what lets both instincts fire correctly on the same
file, depending on which sentence is in front of the editor. It is the same shape as the
repair-versus-amendment test one layer down, in
[Repairing a frozen clause without amending it](kb-playbook-repair-frozen-clause-001) — ask
whether the *set of things a document is true about* has changed, not whether characters on the
page have.

The fourth instance names the axis the first three left implicit. An expired reason is the most
tempting rewrite in the corpus, because the decision above it is still correct and nothing looks
broken after the edit. The honest record of one is a new atom, never a repaired one: a decision
whose stated reason has been quietly updated can no longer explain why it was taken.
