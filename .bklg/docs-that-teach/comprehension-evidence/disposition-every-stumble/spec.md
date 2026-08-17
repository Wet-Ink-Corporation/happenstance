---
item: HS-S0166
stage: spec
created: 2026-08-17T13:16:20.065Z
updated: 2026-08-17T13:16:20.065Z
template_sig: 87bbf1d0
rendered_sig: 51d2e824
---

# Spec — Exactly one disposition per stumble, readable at the stumble

## Scope lock

| What | Where |
| --- | --- |
| Initiative (gold source) | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — BR-06, DoD scenario 6 |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) — `### Definition of Done` row 6 (owner HS-P0024); `## Design tension ownership` (the DT vocabulary an escalation must draw from) |
| Project | [`.bklg/docs-that-teach/comprehension-evidence/project.md`](../project.md) — AC-006 (this story's traced criterion), derived requirement 7, DoD item 2 |
| This spec | `.bklg/docs-that-teach/comprehension-evidence/disposition-every-stumble/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — `## UX brief` (U3's row; the states list; IQ-1, IQ-2, IQ-4, IQ-7; UX-AC-005/006/008/011) and `## Testing brief` (the AC-006 row, and the standing prohibition on automating it into a presence check) |
| Signed-off design | [`../_design.md`](../_design.md) — approved 2026-08-17, no conditions; `hasSurface: false`, empty `## Items` block. Binding as a *constraint*: no new format, no widget, no invented primitive |
| Story map / roadmap pointer | [`../_storymap.md`](../_storymap.md) — slice `dispositions-and-routing`; `## Coverage`, AC-006 row (the seam with `content-fixes-from-dispositions`); `## Merge order` §3 |
| The artifact this story writes into | `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` — landed by `friction-log-skeleton`, filled by `session-run-against-pinned-tree`. Does not exist yet at spec time; see `## Integration contract` |

## One-line PR slice

Give every severity-marked item exactly one disposition — fixed, deliberately accepted with its
reason, or routed — readable at the stumble itself, with any later change recorded as a revision
beside the original rather than overwriting it.

That sentence is `../_storymap.md`'s own, unparaphrased and deliberately so: a human approved that
row at the story-map gate on 2026-08-17, and a second description of the same slice is a description
that can drift from the approved one.

## Executive summary

**What this PR lands.** The disposition pass over the friction log: one arm chosen and written at
every severity-marked stumble, a typed payload attached to it, and the derived index regenerated
from the record rather than substituted for it. After this PR the log answers, at every entry and
without a context jump, the question "what happened to this?"

**Pointer, not restatement.** The *why* is `../project.md`'s AC-006 and derived requirement 7, and
the research basis is `../_grounding.md:69-71` — a log is evidence only once routed to someone who
can act; a log with no destination is opinion by another name. Neither is re-argued here.

**The delta this story adds on top of what already exists.** `friction-log-skeleton` landed the
*slot* — a `Disposition:` field on every entry with five expressible arms and a `Revisions:` field
defaulting to `none` (`../friction-log-skeleton/spec.md:257-258`). `session-run-against-pinned-tree`
filled the record and deliberately left every slot reading `not yet dispositioned`
(`../session-run-against-pinned-tree/spec.md:416`). Neither of them decided anything. This story is
the decision pass: it converts a complete record of what stopped a reader into a complete record of
what is being done about it, and it is the first point in the project where the count "zero
undispositioned, zero double-dispositioned" can be true at all.

**What it deliberately stops short of.** It does not submit the log to an owner, does not verify
that a destination id is the *correct* owner, and does not land a single content fix. Those are
`route-and-escalate`'s and `content-fixes-from-dispositions`'s, and the seams are stated below
rather than left to be discovered at review.

## Context pack

Everything in this section is load-bearing and stated as a decision. The deeper artifacts stay
behind the signposted anchors in the second half of this spec.

### The persona-journey slice this realizes

**U3, the downstream actor** — a sibling project owner, the `support` initiative, or HS-P0025 —
whose intent the UX brief states as: *open the log, find the items that are theirs, and act, without
reading the whole thing and without asking the logger what an entry meant*; and whose named failure
is *reaching an item whose destination is a description rather than an id, or whose disposition is
implied by silence* (`../_decomposition.md`, `## UX brief`, the three-user table).

Both halves of that failure are this story's to prevent. "Implied by silence" is an undispositioned
entry; "a description rather than an id" is a disposition whose payload is prose. The backbone
activity is A4, *Give every stumble a destination*, whose stated end condition is that **each
severity-marked item resolves to a fix, a recorded acceptance, a routed id, or an escalation
carrying a DT id** (`../_storymap.md`, `## Backbone`).

U2, the facilitator, is present only as the author of the record this pass must not disturb. U1, the
recruited reader, is gone by the time this story runs and is never consulted to reinterpret an entry
— if an entry cannot be understood without asking the logger what it meant, that is a finding about
the record, dispositioned like any other, not a licence to rewrite it.

### Decision 1 — dispositioning is a pass *over* the record, and the record is not editable by it

The chronological section is append-only and its ids, heading labels, severity marks and
"what happened" text are frozen at write time (`../_decomposition.md`, IQ-3 / UX-AC-007;
`../friction-log-skeleton/spec.md:254`). This story writes into two fields on each entry —
`Disposition:` and `Revisions:` — and regenerates one derived section. It changes nothing else in the
file.

The wrong implementation this rejects is the tidy-up pass: renumbering to close gaps, re-wording a
heading label so it reads better next to its disposition, reordering entries by severity, or
"correcting" a severity mark now that the fix is understood. Each of those breaks a citation that a
sibling project may already have made into a stumble id, silently and with no error message
(`../session-run-against-pinned-tree/spec.md:448-458`). Gaps in the id space are permitted and
expected; closing them is the forbidden operation.

### Decision 2 — exactly one arm, and both zero and two are failures

`../project.md`'s AC-006 is literal: *"Zero undispositioned items; zero items with two
dispositions."* The shape already expresses five arms — `not yet dispositioned` | `fixed: <ref>` |
`accepted: <reason>` | `routed: <id>` | `escalated: DT-<n>` — and `not yet dispositioned` is legal
*during* the session and illegal at hand-off (`../_decomposition.md`, `## UX brief`, states list;
`../friction-log-skeleton/spec.md:257`).

Two arms is the failure mode nobody guards: "fixed, and also routed to HS-P0022 in case it comes
back" reads as diligence and is a stumble with two owners and no accountability. Where a stumble
genuinely has two halves, the correct move is one arm on the entry plus a **new** appended entry for
the second half, not two arms on one entry — the shape is the constraint, and this story does not
widen it.

### Decision 3 — the disposition is readable at the stumble; the index is derived and deletable

IQ-1 is *in place, not a context jump*: a reviewer must never leave the entry to learn whether it
was resolved. IQ-2 is *non-occlusion*: a roll-up is additive, and the check is stated as a deletion
— *if the collapsed or filtered view were deleted, would the record still carry every stumble and
its disposition?* (`../_decomposition.md`, IQ-1, IQ-2; UX-AC-005, UX-AC-006).

So `## Dispositions index` is regenerated in this PR **from** the entries, states in its own first
line that it is derived and that the chronological record is authoritative, and can be deleted
without loss. A disposition that lives only in the index — the obvious labour-saving move once there
are twenty of them — converts every future review into a context jump and fails the deletion check
by construction.

### Decision 4 — a changed disposition appends beside the original; nothing is overwritten

IQ-4 is reversibility, and its authority is
[`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md)
(accepted, `authority_tier: guideline`), applied one level out from doc comments: the test is
*whether the edit changes what the document asserts*. Changing a disposition changes what the log
asserts about that stumble, so it is an **append with its reason**, dated, with the earlier
disposition still legible — never an in-place edit of the `Disposition:` line.

The `Revisions:` slot exists from first write, defaulting to `none`, precisely so that the first
revision has somewhere to go; retrofitting it at the moment of first change is how the original gets
overwritten (`../friction-log-skeleton/spec.md:258`). This is also what makes an escalation safe to
make: one that later proves unnecessary can be withdrawn without the trail vanishing.

### Decision 5 — the payload is content, not presence, and this is why the check is not automated

`../_decomposition.md`'s `## Testing brief` forbids by name the obvious automation: *"A script that
merely confirms a `Disposition:` line exists under every stumble heading would pass a disposition
that says nothing ('noted') as readily as one that says 'routed to HS-P0022, see stumble #4' — it
does not reject the wrong implementation research 04 is worried about."* This is `CLAUDE.md`'s
"a rule that no adapter can fail is decorative" one level up.

The consequence is a split this story adopts rather than argues with: the **arm and its payload
shape** are mechanically checkable (an arm token is present exactly once; a `routed:` payload matches
an id pattern; a `fixed:` payload names a referent), and the **substance** — is this reason a reason?
is this the right destination? — is reviewer-read and ledger-cited as `file:line`
(`.redkiln/templates/_ledger.md`; `require_ledger: true` at `.redkiln/config.yaml:67`). Writing an
automated check that asserts only presence would be worse than writing none, because it would
report green over exactly the failure it was built to catch.

### Decision 6 — destinations are ids, and all three destination kinds are real but not identical

IQ-7: *"Route to the docs team" is not a destination.* The three destination kinds AC-007 names, with
what each actually is in this repository:

- **The `support` initiative** — `.redkiln/config.yaml:5` (`support_initiative: support`), and
  `.bklg/support/initiative.md` exists as `HS-I0005`. It is **real but structurally empty**: an
  unfilled template at `stage: intake` with zero projects (`../_decomposition.md`, `### Notes`).
  Routing an incidental library bug there is correct; writing the disposition as though it were slotting
  into an active backlog is not.
- **A sibling project id** — `HS-P0020` … `HS-P0023`, each resolving to a real `project.md` in this
  initiative. Two of them (`HS-P0022`, `HS-P0023`) already name HS-P0024 reciprocally as *their*
  disposition source, so the edge is bidirectional and pre-agreed (`../_grounding.md:109-137`).
- **A deferral** — which is **staged material, not a file this project writes**. `.kb/` atoms are
  authored only through the ingest path or closeout, and the first hand-authoring attempt was
  reverted (`0269720`). A deferral disposition therefore records the id of the hand-off that stages
  the question, and this story edits nothing under `.kb/` (`../_decomposition.md`, `### Notes`).

### Decision 7 — escalation is a distinct arm, and choosing it is this story's; perfecting it is not

`../project.md`'s AC-011 makes an escalation *a disposition that would change a resolved design
tension owned by a sibling project, recorded with the tension's DT id, rather than absorbed as a fix
here*. The **recognition** — noticing that this stumble is DT-shaped and therefore not ours to fix —
is inseparable from choosing the arm, so it happens in this story. The **DT id's correctness, the
submission to a named owner, and the verification that a destination is the right owner** are
`route-and-escalate`'s (`../_storymap.md`, `## Coverage`, AC-007 and AC-011 rows).

The failure this seam exists to prevent is absorption: a stumble that says the staged encounter's
anchor model does not land, dispositioned `fixed:` and quietly repaired here, puts half of DT-1 in
each of two projects. The DT vocabulary is the ten-row table at
`.bklg/docs-that-teach/_decomposition.md:150-159`; the shape sibling projects already declared they
would accept is quoted in `../_decomposition.md`, `### Notes`.

### Decision 8 — the `fixed:` arm is a claim made here and discharged next door

`../_storymap.md`'s `## Coverage` splits AC-006 explicitly: this story owns *exactly one disposition
per stumble*; `content-fixes-from-dispositions` owns **only the "fixed" arm** — "landing the fix a
'fixed' disposition asserts, and clearing the gate for it".

So a `fixed:` disposition written here names *what will change and where* in enough detail that the
next story can act on it without re-deriving the finding, and this PR lands no edit under `crates/`,
`docs/`, `examples/`, `spec/` or `standards/`. Choosing `fixed:` for something that in fact needs a
new page-structure convention is the mis-arm UX-AC-012 already names: that is a routed item for
HS-P0021 `page-need-discipline`, not an invention.

### Decision 9 — what requires a disposition, and the forbidden way to reduce the count

The obligation attaches to **severity-marked** entries. The skeleton's entry shape has four `Kind`
values — `observation` | `reaction` | `intervention` | `abandonment` — and a `Severity:` field that
takes `n/a` for non-stumble kinds (`../friction-log-skeleton/spec.md:255-256`). A `Kind:
intervention` entry records that the facilitator acted; it is a methodological fact, not a stumble,
and carries `n/a` on both severity and disposition.

Two moves are forbidden, and they are the same move wearing two hats: **erasing or downgrading a
severity mark during this pass to shrink the work**, and **dispositioning an entry `accepted:` with
no reason because it looked minor**. The first also violates Decision 1 (the record is not editable
here); the second is the "noted" failure of Decision 5. An abandonment entry is a finding, not a
void, and if the session ended there it carries a severity and therefore a disposition
(`../_decomposition.md`, IQ-6).

### Decision 10 — this story ships no code, and that does not lower the bar

No `pub` item, no crate under `crates/`, no `SPECIFICATION.md` clause, no conformance rule. The
bar is the one `../_decomposition.md`'s `## Testing brief` sets for artifact work:
`cargo xtask affected --base main` (`.redkiln/config.yaml:40`) falls through to the five
file-reading lints and `spec-trace` unconditionally rather than passing vacuously on an empty
package set, `redkiln validate && redkiln doctor` stay clean at exactly the six standing
`template-drift` advisories `CLAUDE.md` documents, and the `_ledger.md` carries a real `file:line`
per AC into the log itself — because the log *is* the proof artifact here, which is the shape rigor
takes when the deliverable is a record rather than a function.

`cargo xtask ci --fast` (`.redkiln/config.yaml:55`) is the project's integration bar and bites for
`content-fixes-from-dispositions`, not for this PR, which compiles nothing.

### What this story is explicitly not deciding

- **The log's path, section set, entry template, arm vocabulary or revision format.**
  `friction-log-skeleton`'s, downstream of `../_design.md`. This story fills a shape it did not
  invent, and if the shape proves wrong the defect is logged and routed, not patched mid-pass.
- **The protocol** — persona, scenario, narration mode, severity scale, disqualifying criteria.
  `dt9-and-fixed-protocol`'s, and `../_design.md` is not edited here; project AC-002's provenance
  check depends on it staying untouched.
- **Whether a destination is the right owner, and the submission itself.** `route-and-escalate`'s.
- **Any content fix.** `content-fixes-from-dispositions`'s.
- **The scope sentence, the second-session verdict, the hand-off note.** `scope-the-claim`,
  `second-session-decision`, `handoff-note-to-closeout`.
- **Anything under `.kb/`**, including staging a persona or an open question as a file.

### The one ordering constraint that can invalidate this work

This story is `blocked_by: HS-S0165` (`session-run-against-pinned-tree`) and there is nothing to
disposition until the log has stumbles in it. Two consequences worth stating rather than
discovering:

1. **A short log is still the whole set.** If the session was abandoned early (IQ-6), the stumble
   set may be three entries. Three dispositioned entries satisfy AC-006 exactly as thirty would; a
   thin log is an input to `second-session-decision`, never a reason to hold this pass open.
2. **Dispositioning must not begin before the session ends.** A disposition written beside a stumble
   *during* the session is the double-disposition failure starting early, and
   `../session-run-against-pinned-tree/spec.md:358-361` already forbids it from the other side.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice. The observable is U3 opening the log and finding what happened to an item (`../_storymap.md`, slice table) |
| **Slice / milestone** | `dispositions-and-routing` |
| **Slice-mates** | `route-and-escalate` and `content-fixes-from-dispositions` — both `depends_on` this story, and both are implemented in the same context and mounted as one integrated surface. Merge order §3 of `../_storymap.md`: this story first, then those two in either order |
| **Mount point** | `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` — specifically the `Disposition:` and `Revisions:` fields of each `### FL-###` entry in `## Chronological record`, and the `## Dispositions index` section regenerated from them. That file is the project's real render path for this capability: it is reachable in one hop from `../project.md`'s `## Companions` list, which is how a human arriving from `redkiln board` finds it. A disposition recorded in this story's own folder, in a scratch table, or in an implementation report **is not a delivery** |
| **Mount point caveat** | The log does not exist at spec time (`test -f` fails today). Its path is `../_design.md`'s to bind and `friction-log-skeleton` follows it; if a different home is bound — `references/evaluation/` being the plausible alternative — this story follows it too and `## PR boundary` is widened here, deliberately, before the pass begins. See `EC-001` in the second half |
| **Wires into** | The **entry shape** (`../friction-log-skeleton/spec.md:255-258`) — six one-line fields, four `Kind` values, five disposition arms, `Revisions:` defaulting to `none`; the **filled chronological record** left by `session-run-against-pinned-tree`, consumed read-only except for those two fields; the **`## Dispositions index`** two-column routing table in `docs/README.md`'s existing shape; `.redkiln/config.yaml:5` (`support_initiative: support`) as the `routed:` arm's destination vocabulary; `.bklg/docs-that-teach/_decomposition.md:150-159` as the `escalated:` arm's DT vocabulary; `.redkiln/templates/_ledger.md`'s `evidence: "file:line"` contract, which is what makes a stable stumble id worth having |
| **Design-system primitives consumed** | The document primitives `../_decomposition.md`'s `#### The primitive layer to compose from — do not hand-roll` fixes: `docs/README.md`'s two-column routing table (the index), the one-line checkbox from `.redkiln/templates/gates/`, and the existing entry shape. **No new format, no navigation widget, no fold, no colour or emoji as a sole carrier of meaning** |
| **Renders surfaces** | **None by id** — `../_design.md`'s `## Items` block is an empty fence (`# no items — no public API surface, no rendered UI surface`) and every shape section reads `N/A — no user-facing surface`; the design review recorded `hasSurface: false` explicitly rather than skipping the project. What this story changes is the UX brief's **surface 2, the friction log**, which `_design.md` names in prose rather than as an item. It touches neither surface 1 (the material walked) nor surface 3 (`_design.md`) |
| **Conformance rule(s)** | **None, and not adapter-observable.** No rule in `crates/happenstance-testkit/src/suite.rs` can observe a friction log; no port, signature or bound changes. Stated rather than left blank because `CLAUDE.md` requires a story that changes a port to name a rule — this story's answer is that it changes no port. It adds no rule and asserts no literal position value anywhere |
| **Clause(s)** | **Discharges and amends none.** No `SPECIFICATION.md` clause is edited, no `[FROZEN]` clause changes, no line-anchored citation moves, so no ADR is owed and `cargo xtask spec-trace` has nothing to re-anchor. A *finding* that implies a clause is wrong is a stumble with a disposition — routed by `route-and-escalate`, never a clause edit inside this PR |
| **Advances DoD scenario** | Initiative **DoD scenario 6** — *"Every stumble in that log has a disposition"* (`.bklg/docs-that-teach/_decomposition.md:226`, owner HS-P0024), and project DoD item 2. This story is what makes scenario 6 checkable end to end; `route-and-escalate` turns the "routed to someone who can act" half green on top of it |

**Delivered mounted, not as an isolated component.** The acceptance bar for this PR includes that a
reviewer arriving cold at `redkiln board` reaches the project card, follows one `## Companions` link,
opens any entry in the log, and sees its disposition there — with no second document, no filter and
no question to the logger.

## PR boundary

**In this PR**

- One disposition arm, with its payload, written into the `Disposition:` field of every
  severity-marked entry in `## Chronological record`.
- `n/a` recorded on the entries the shape exempts (`Kind: intervention`, and any entry whose
  `Severity:` is `n/a`), so exemption is stated rather than inferred from a blank.
- Any revision made during this pass appended to the entry's `Revisions:` field, dated, with the
  earlier disposition and the reason both legible.
- `## Dispositions index` regenerated from the entries, carrying its derived-and-not-authoritative
  first line, in `docs/README.md`'s two-column shape.
- This story's own backlog folder: `_ledger.md` with a real `file:line` per AC, and the stage
  artifacts.

**Explicitly not in this PR**

- **Any edit to an entry's id, heading label, `Time`, `Kind`, `Severity` or "what happened" text.**
  The record is read-only to this pass (Decision 1).
- **Any content fix** under `crates/`, `docs/`, `examples/`, `spec/` or `standards/` — including the
  one-line doc-comment fix that would take less time than writing the disposition.
  `content-fixes-from-dispositions`'s, and its own gate bar is what proves it.
- **Submission of the log to a named owner, and the recording of that submission**;
  **verification that a destination id is the correct owner**; **the escalation's DT-id
  correctness**. `route-and-escalate`'s.
- **Any edit to `../_design.md`** — the protocol, the severity scale, the persona. Project AC-002's
  provenance check depends on that file not moving after the session.
- **Anything under `.kb/`**, including staging a persona or an open question as a file. A deferral
  disposition records the id of the hand-off; it does not write the atom.
- **The scope sentence, the second-session verdict and the hand-off note.**
- **Any automation that "checks" a disposition exists.** Forbidden by name in
  `../_decomposition.md`'s `## Testing brief`: a presence check passes `"noted"` as readily as
  `"routed: HS-P0022"` and rejects no wrong implementation.
- **A second session, or any re-run of the walk.**

**The composition-root exception, stated so it is not read as scope drift.** The implementer MAY
touch the wiring files named in `## Integration contract` — the `## Companions` row in
`../project.md` (body prose only; the CLI owns the frontmatter and a `PreToolUse` hook denies the
edit) — if and only if the mount is not already in place from `friction-log-skeleton`. Mounting this
slice is delivery, not drift.

**Merge DoD one-liner** — every severity-marked entry in the log carries exactly one disposition arm
with a typed payload, readable at the entry; the index is derived and deletable; no id, label or
severity moved; `cargo xtask affected --base main` green; `redkiln validate && redkiln doctor` clean
at exactly the six standing `template-drift` advisories; `_ledger.md` carries a real `file:line` per
AC.

**Paths this story may touch** — `redkiln verify --grain story` reads the first fenced block under
this heading and fails on any file changed outside it.

```
.bklg/docs-that-teach/comprehension-evidence/_friction-log.md
.bklg/docs-that-teach/comprehension-evidence/project.md
.bklg/docs-that-teach/comprehension-evidence/disposition-every-stumble/**
```

Narrow on purpose: one artifact, one mount, one story folder. No crate, no `docs/`, no `.kb/`, no
`_design.md`. If a change here appears to need a fourth path, it is almost certainly a content fix
that belongs to the next story — write the `fixed:` disposition and stop.

## Behavior and interfaces

"Evidence path" is where the claim is grounded today — the thing to read, not the thing to copy.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **every severity-marked entry carries exactly one arm** | Sweep the whole `## Chronological record`, entry by entry, in id order. At the end of the pass, zero entries read `not yet dispositioned` and zero carry two arms. Both are failures of equal weight | `../project.md` AC-006 ("Zero undispositioned items; zero items with two dispositions") · `../_decomposition.md` `## UX brief`, states list |
| **the arm vocabulary is the shape's, unextended** | `fixed: <ref>` \| `accepted: <reason>` \| `routed: <id>` \| `escalated: DT-<n>`, with `not yet dispositioned` legal only before this pass. No sixth arm is invented, and "partially fixed", "monitoring" or "won't fix" are spellings of an existing arm, not new ones | `../friction-log-skeleton/spec.md:257` · `../project.md` AC-006, AC-007, AC-011 |
| **the disposition is legible at the entry** | The `Disposition:` field on the entry carries the arm and its payload. A reviewer reading one entry learns its outcome without navigating; at most one optional hop outward, by id | `../_decomposition.md` IQ-1 / UX-AC-005 |
| **`accepted:` carries a reason a stranger can evaluate** | The reason states what was accepted and why it is acceptable — not a mood, not "noted", not "minor". A reason that only makes sense to someone who was in the room fails, because U3's stated constraint is acting *without asking the logger what an entry meant* | `../_decomposition.md` `## Testing brief`, `**Do not automate AC-006…**` · `## UX brief`, U3's row |
| **`fixed:` names its referent, and lands nothing** | The payload names what will change and where, specifically enough that `content-fixes-from-dispositions` can act without re-deriving the finding. No file under `crates/`, `docs/`, `examples/`, `spec/` or `standards/` is edited in this PR | `../_storymap.md` `## Coverage`, AC-006 row (the "fixed" arm is the next story's) · `../_decomposition.md` UX-AC-012 |
| **`routed:` and `escalated:` carry an id token, never prose** | `HS-P0020` … `HS-P0023`, the `support` initiative per `.redkiln/config.yaml:5`, a named staged deferral, or `DT-<n>` from the ownership table. "Route to the docs team" is IQ-7's own named failure and fails AC-006 as surely as an undispositioned item | `../_decomposition.md` IQ-7 / UX-AC-011 · `.bklg/docs-that-teach/_decomposition.md:150-159` · `../_grounding.md:96-152` |
| **escalation is recognised here, perfected next door** | A stumble whose fix would change a resolved sibling tension takes the `escalated:` arm rather than `fixed:`. Getting the DT id right, submitting the log, and confirming the destination owner are `route-and-escalate`'s | `../project.md` AC-011 · `../_storymap.md` `## Coverage`, AC-011 row · `../_decomposition.md` `### Notes` (the shape siblings declared they would accept) |
| **the exempt entries say so** | `Kind: intervention`, and any entry whose `Severity:` is `n/a`, record `n/a` on the disposition rather than being left blank. A blank is indistinguishable from an oversight; a stated `n/a` is an assertion | `../friction-log-skeleton/spec.md:255-256` · `../_decomposition.md` IQ-5 / UX-AC-009 |
| **severity is never adjusted to reduce the work** | No severity mark is erased, downgraded or re-scaled during this pass. If a mark now looks wrong, that is a finding about the protocol, appended and dispositioned — not a silent correction | `../_decomposition.md` UX-AC-003 · `../session-run-against-pinned-tree/spec.md:409` (severity applied inline, never retrofitted) |
| **the record itself is not rewritten** | Ids, heading labels, `Time`, `Kind` and "what happened" text are unchanged by this PR. `git diff` on the log shows changes confined to `Disposition:` / `Revisions:` lines and the `## Dispositions index` block | `../_decomposition.md` IQ-3 / UX-AC-007 · `../session-run-against-pinned-tree/spec.md:448-458` (the id space has no forwarding mechanism) |
| **a changed disposition appends, dated, with its reason** | Where a disposition written earlier in this pass is revised, the `Revisions:` field gains a dated line carrying the earlier disposition and why it changed; the original `Disposition:` text is not edited in place | `.kb/governance/rewrite-the-referent-never-the-reasoning.md` · `../_decomposition.md` IQ-4 / UX-AC-008 |
| **the index is derived, additive and deletable** | `## Dispositions index` is regenerated from the entries, states in its first line that it is derived and that the chronological record is authoritative, and uses `docs/README.md`'s two-column shape. Delete the whole section and no stumble and no disposition is lost | `../_decomposition.md` IQ-2 / UX-AC-006 · `docs/README.md` (the primitive) |
| **complete as static text, with nothing behind a fold** | No `<details>`, `<summary>` or script is introduced; no severity or disposition carries meaning by colour or emoji alone; any checkbox stays on one line. The log stays readable in `git diff` and by a screen reader | `../_decomposition.md` `#### The accessibility floor` · `CLAUDE.md` (the line-by-line gate parser) |
| **the pass runs after the session, over the whole set** | Dispositioning begins only once the session has ended and the record is closed; every severity-marked entry in the log is covered, including the abandonment entry if the session ended in one | `../session-run-against-pinned-tree/spec.md:358-361, 413` · `../_decomposition.md` IQ-6 |
| **checks assert on payload shape; substance is ledger-cited** | Mechanical checks confirm one arm token per entry and an id-shaped `routed:`/`escalated:` payload. Whether a reason is a reason, and whether a destination is the right one, stay reviewer-read with `file:line` evidence in `_ledger.md` | `../_decomposition.md` `## Testing brief` · `.redkiln/templates/_ledger.md` · `.redkiln/config.yaml:67` |
| **nothing else moves** | No `.kb/` file, no persona promotion, no `_design.md` amendment, no doc comment, no crate, no `SPECIFICATION.md` clause, no conformance rule, no submission, no second session | `../project.md` `## Out of scope` · `CLAUDE.md` binding constraints |

### The acceptance criteria this story enumerates

Nine, in the order the behaviour above lands them, framed from the intent of the person each serves —
U3 the downstream actor, and the reviewer six months later who has no one left to ask:

**AC-001** (every severity-marked stumble carries exactly one arm at hand-off; zero and two both
fail) · **AC-002** (the disposition is readable at the stumble, one optional hop maximum) ·
**AC-003** (the `accepted:` arm carries a reason a stranger can evaluate, not a presence token) ·
**AC-004** (the `fixed:` arm names its referent and lands no code in this PR) · **AC-005** (the
`routed:` and `escalated:` arms carry id tokens, never prose, and a DT-shaped stumble is not absorbed
as a fix) · **AC-006** (a changed disposition appends a dated revision beside the original, which
stays legible) · **AC-007** (which entries owe a disposition is stated and applied — exemptions
recorded as `n/a`, and no severity adjusted to shrink the set) · **AC-008** (the chronological record
is not rewritten: ids, labels, severities and narrative text unchanged; the diff is confined to the
two fields and the index) · **AC-009** (the index is derived and deletable, the log stays complete as
static text, and the disposition is reachable in one hop from the project card).

## Data and migrations

**N/A for schema, store and runtime state.** This story adds no `pub` item, defines no type, compiles
nothing, opens no connection and writes no row. It edits two fields per entry in one markdown file
and regenerates one section of it. `../_design.md` records `hasSurface: false` and an empty items
block for exactly this reason, and `happenstance-core`'s binding constraints (no `#[async_trait]`, no
`serde` in default features) are not approached from any direction.

**One identifier space is nonetheless data-shaped, and its migration policy is "there is none".**
The stumble id `FL-###` behaves like a primary key that other documents foreign-key into: the moment
`route-and-escalate` hands `FL-004` to HS-P0022, that id is an external reference with no redirect,
no alias table and no forwarding mechanism (`../session-run-against-pinned-tree/spec.md:448-458`).
This story is the first pass that has a *reason* to want to renumber — dispositions cluster, and a
severity-ordered log reads better — and it is the pass that must not. Gaps are permitted and
expected; closing them is a broken citation with no error message, in the same spirit as this
repository's rule that a conformance test never asserts literal position values because the
specification permits gaps (`CLAUDE.md`, `## The rule that matters`).

The one thing that grows here is the `Revisions:` field, and it grows **append-only**: a revision is
a new dated line, never an edit of an existing one. That is the whole of this story's write model,
and it is the same discipline the append-only chronological record already runs under.

## Acceptance criteria

Nine, framed from the intent of the person each criterion serves — **U3, the downstream
actor** (a sibling project owner, the `support` initiative, HS-P0025), and the reviewer six
months out who has nobody left to ask (`../_decomposition.md`, `## UX brief`, the three-user
table). Each crosses the whole slice: a decision taken, written into the record at the
stumble, and reachable from the project card without a second document.

`<log>` throughout is `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` — the
path `friction-log-skeleton`'s spec binds and mounts at `../project.md`'s `## Companions`.
Where `../_design.md` fixes a different path, that path substitutes verbatim in every row
below and in `## PR boundary` (EC-001).

"Verification" follows the project testing brief's tier split (`../_decomposition.md`,
`## Testing brief`): **Static** where the claim is an `rg` / `git` / `test -f` fact, and
**Artifact-evidence** where it is genuinely reviewer-read and carried by the `_ledger.md`
`file:line` discipline `require_ledger: true` (`.redkiln/config.yaml:67`) already enforces.
Per that brief's standing prohibition, **no static check below asserts mere presence** — a
check that would pass `"noted"` as readily as `"routed: HS-P0022"` rejects no wrong
implementation and would report green over exactly the failure it was built to catch.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** U3 receives the log at hand-off and must be able to trust that nothing was quietly left undecided, **WHEN** they read `## Chronological record` end to end, **THEN** every entry carrying a severity token from the `## Severity scale` legend has **exactly one** disposition arm with its payload — `fixed: <ref>` \| `accepted: <reason>` \| `routed: <id>` \| `escalated: DT-<n>` — zero entries still read `not yet dispositioned`, and no entry carries two arms; and **NOT** a log where the "obvious" items were left blank, nor one where an item is both fixed *and* routed "in case it comes back", which is a stumble with two owners and no accountability. | **Static.** `rg -n "^Disposition:" <log>` returns one line per `### FL-` block (counts equal); for every severity-marked entry the line matches exactly one arm token and zero match `not yet dispositioned`; no line contains two arm tokens. **Artifact-evidence:** the reviewer counts severity-marked entries against dispositioned entries and the ledger cites both counts by `file:line`. Traces project **AC-006** ("Zero undispositioned items; zero items with two dispositions"). |
| AC-002 | **GIVEN** U3's stated constraint is acting *without asking the logger what an entry meant*, **WHEN** they open any single `### FL-###` entry, **THEN** the arm and its payload are legible on the `Disposition:` line **inside that entry's own block**, so the outcome is known without leaving the entry — any outward move is by id, optional, and at most one hop; and **NOT** a `Disposition:` field reading "see the index below", nor a disposition that exists only in `## Dispositions index`. | **Static.** Every `### FL-` block contains its own `Disposition:` line carrying an arm and payload; a scan of those lines for the cross-reference words `see` / `below` / `above` / `index` / `as noted` returns nothing. **Artifact-evidence:** ledger cites two entries by `file:line` — one `routed:`, one non-`routed:` — and the reviewer confirms each is self-contained. IQ-1 / UX-AC-005. |
| AC-003 | **GIVEN** a reviewer six months out is deciding whether an accepted stumble should now be reopened, **WHEN** they read an entry dispositioned `accepted:`, **THEN** the payload states **what is being accepted and why that is acceptable** — the cost borne, and the reason bearing it is right — in terms a stranger to the session can evaluate; and **NOT** `"noted"`, `"minor"`, `"by design"`, `"wontfix"`, a mood, or a reason that only makes sense to someone who was in the room. | **Artifact-evidence (reviewer-read, ledger-cited)** — the brief forbids automating this into a presence check. **Static support only:** every `accepted:` payload is a full clause, not a bare token, and matches none of the deny-list `noted` / `minor` / `ok` / `n/a` / `by design` / `wontfix` alone. The ledger cites each `accepted:` entry by `file:line`. Traces project **AC-006**; `../_decomposition.md`, `## Testing brief`, *Do not automate AC-006…*. |
| AC-004 | **GIVEN** `content-fixes-from-dispositions` must land the fix this arm asserts without re-deriving the finding, **WHEN** an entry is dispositioned `fixed:`, **THEN** the payload names **what will change and where** — a real path that resolves, and the section, item or doc comment within it — and **NOT** a past-tense claim about an edit made in this PR: the diff contains no file under `crates/`, `docs/`, `examples/`, `spec/` or `standards/`, including the one-line doc-comment fix that would take less time than writing the disposition. | **Static.** `git diff --name-only main...HEAD` intersected with `crates/ docs/ examples/ spec/ standards/` is empty; every `fixed:` payload contains a path that `test -f` or `test -d` resolves from this worktree. **Artifact-evidence:** the reviewer confirms the payload is specific enough to act on cold. `../_storymap.md`, `## Coverage`, AC-006 row (the "fixed" arm is the next story's). |
| AC-005 | **GIVEN** U3 opens the log to *find the items that are theirs*, **WHEN** they read a `routed:` or `escalated:` entry, **THEN** the payload is an **id token** — `HS-P0020` … `HS-P0023`, the `support` initiative (`.redkiln/config.yaml:5`), or a named staged deferral hand-off — respectively `DT-<n>` drawn from the ten-row ownership table; and a stumble whose fix would change a design tension a sibling already resolved takes the `escalated:` arm rather than being absorbed as `fixed:` here; and **NOT** "route to the docs team", "the docs owner", or any prose destination, which fails as surely as an undispositioned item. | **Static (id existence).** Every `routed:` payload matches `HS-[PI][0-9]{4}` or names `support`, and resolves: `test -f .bklg/support/initiative.md`, `test -f .bklg/docs-that-teach/<sibling>/project.md`. Every `escalated:` payload matches `DT-([1-9]\|10)` and appears in `rg -n "^\| DT-" .bklg/docs-that-teach/_decomposition.md`. **Artifact-evidence:** the reviewer confirms no DT-shaped finding took the `fixed:` arm. Traces project **AC-006**; guards project AC-011, whose *correctness* and submission are `route-and-escalate`'s. IQ-7 / UX-AC-011. |
| AC-006 | **GIVEN** an escalation or acceptance must be safe to make because it can be withdrawn without the trail vanishing, **WHEN** a disposition written earlier in this pass is changed, **THEN** the entry's `Revisions:` field gains a **dated line carrying the earlier arm and payload verbatim plus the reason for the change**, the `Disposition:` line shows the current arm, and the earlier one stays legible; and **NOT** an in-place edit that leaves no trace, nor a `Revisions:` slot retrofitted at the moment of first change — it exists from first write, defaulting to `none`, precisely so the first revision has somewhere to go. | **Static.** `git log -p --follow <log>` over this story's commits: no hunk rewrites a `Disposition:` line without a `Revisions:` addition in the same commit; every entry carries a `Revisions:` line (`none` where unrevised). **Artifact-evidence:** ledger cites each revised entry by `file:line`, or asserts positively that none was revised — a blank is not an assertion. IQ-4; `.kb/governance/rewrite-the-referent-never-the-reasoning.md`. |
| AC-007 | **GIVEN** "zero undispositioned" means nothing unless the denominator is honest, **WHEN** the pass sweeps the record, **THEN** every entry of a stumble kind carrying a legend severity token owes an arm; entries the shape exempts (`Kind: intervention`, and any entry whose `Severity:` is `n/a`) record `n/a` on `Disposition:` **explicitly**, so exemption is an assertion rather than an inference from a blank; and no `Severity:` value is erased, downgraded or re-scaled during this pass; and **NOT** a shrunken denominator produced by demoting a mark now that the fix looks hard, nor by leaving exempt entries blank so that a blank has to be read as an exemption. An abandonment entry is a finding, not a void: if the session ended there it carries a severity and therefore an arm. | **Static.** `git diff main...HEAD -- <log>` shows **zero** changed `Severity:` lines; every `### FL-` block carries a non-empty `Disposition:` value, `n/a` exactly where `Kind: intervention` or `Severity: n/a`. **Artifact-evidence:** the reviewer confirms each `n/a` matches an exempt kind and no stumble kind carries one. UX-AC-003; IQ-5 / IQ-6. |
| AC-008 | **GIVEN** `route-and-escalate` will hand `FL-004` to HS-P0022 and there is no redirect, alias table or forwarding mechanism behind that id, **WHEN** this PR's diff on the log is read, **THEN** it is confined to `Disposition:` lines, `Revisions:` lines and the `## Dispositions index` block — no id renumbered, no gap closed, no heading label re-worded, no entry reordered by severity in place of chronology, no `Time`, `Kind`, `Severity` or `What happened` text changed, and no section added, renamed or reordered; and **NOT** a tidy-up pass that makes the record read better next to its dispositions, which breaks a citation silently and with no error message. | **Static.** `git diff -U0 main...HEAD -- <log>`: every changed line is a `Disposition:` line, a `Revisions:` line, or inside `## Dispositions index`. `rg -n "^### FL-" <log>` yields the identical id sequence before and after; `rg -n "^## " <log>` yields the same eight headings in the same order. **Artifact-evidence:** ledger cites the first and last entry headings as unchanged. IQ-3 / UX-AC-007. |
| AC-009 | **GIVEN** a reviewer arriving cold at `redkiln board`, **WHEN** they open the project card, follow **one** `## Companions` link and open any entry, **THEN** they see its disposition with no second document, no filter, no script and no rendering step: `## Dispositions index` is regenerated from the entries in `docs/README.md`'s two-column shape, states in its own first line that it is **derived** and that `## Chronological record` is authoritative, and deleting the whole section loses no stumble and no disposition; no `<details>`, fold, widget or colour/emoji-only carrier is introduced and any checkbox stays on one line; and **NOT** a disposition reachable only through the index, which is the labour-saving move that converts every future review into a context jump. | **Static (deletion check).** In a scratch copy, delete `## Dispositions index` and every fold: the `### FL-` count and the `Disposition:` line count are unchanged. `rg -n "<details>\|<summary>\|<script>" <log>` returns nothing; the index's first line asserts derivation. `rg -n "_friction-log" .bklg/docs-that-teach/comprehension-evidence/project.md` returns the `## Companions` row (the one hop). **Artifact-evidence:** ledger cites the index's first line and the Companions row. IQ-2 / UX-AC-006; accessibility floor. |

**Coverage of the traced project AC.** This story traces exactly one: project **AC-006** —
*"each severity-marked item resolves to a fix, a recorded deliberate acceptance with its
reason, or a routed item with a destination id. Zero undispositioned items; zero items with
two dispositions."* It is carried by **AC-001** (the count, both directions), **AC-003**,
**AC-004** and **AC-005** (each arm's payload is a payload rather than a token), and
**AC-007** (the denominator the count is taken over). `../_storymap.md`'s split is honoured:
the `fixed:` arm's *discharge* is `content-fixes-from-dispositions`'s, and AC-004 asserts
this PR lands none of it.

**Where the other four rows come from.** AC-002, AC-006, AC-008 and AC-009 are the UX
brief's interaction-quality invariants for this story's own surface — IQ-1, IQ-4, IQ-3 and
IQ-2 respectively — promoted to table rows rather than left as prose, because a bullet in
`## Interaction quality` carries no ledger row and is gated by nothing. AC-005 additionally
*guards* project AC-011 without owning it: recognising a DT-shaped stumble happens here,
while the DT id's correctness and the submission are `route-and-escalate`'s.

## Interaction quality

RFC §6.7/D6. Every invariant below is already an `AC-###` row above — this section says
**which** row carries it and how that row is checked, and introduces nothing new. The
placement is deliberate: `redkiln verify` extracts ACs by matching a leading `| AC-001 |`
table cell or an `- AC-001:` bullet, so an invariant stated only as prose here would carry
no ledger row, be gated by nothing, and be tested by nobody.

**Where the composition family comes from, given `hasSurface: false`.** `../_design.md` is
signed off (Ryan Britton, 2026-08-17, no conditions) and declares no surface id — its
`## Items` fence is `# no items — no public API surface, no rendered UI surface` and every
shape section reads `N/A`. That is not an exemption from composition; it is a *constraint*
on it. What `_design.md` binds in its `## Items` prose is the **primitive layer**: *"there
is no CSS layer and no token file, and inventing one is out of scope"*, and everything these
artifacts need is drawn from named repository document primitives that already exist. The
composition invariants below are therefore taken from that clause, from the UX brief's
`#### The primitive layer to compose from — do not hand-roll` and `#### The accessibility
floor`, and from the entry shape `friction-log-skeleton` fixes. None of them is re-decided
here.

### State invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump (IQ-1)** — the disposition is readable at the stumble; an index may exist *in addition*; one hop maximum and the hop is optional | **AC-002**, with **AC-009** for the one-hop reach from the project card | The `Disposition:` line lives inside the `### FL-` block and carries the arm and payload, never a cross-reference; the `## Companions` row is the single hop from `../project.md` |
| **Non-occlusion (IQ-2)** — a roll-up, filter or per-owner extract is additive; nothing exists only in a derived or collapsed view | **AC-009** | The deletion check, stated by the UX brief as the method and applied verbatim: delete `## Dispositions index` and every fold from a scratch copy; entry count and `Disposition:` count are unchanged |
| **Preserved position (IQ-3)** — ids, order, heading labels and the section set are stable; the record is not editable by this pass | **AC-008**, with **AC-007** for the severity marks specifically | `git diff -U0` on the log touches only `Disposition:` / `Revisions:` lines and the index block; the `### FL-` id sequence and the eight `## ` headings are byte-identical before and after |
| **Reversibility (IQ-4)** — a changed disposition is recorded *as a change*, dated, with the earlier one legible and the reason attached; nothing is overwritten | **AC-006** | `git log -p --follow`: no `Disposition:` rewrite without a same-commit `Revisions:` append. Every entry carries a `Revisions:` line, `none` where unrevised — the slot exists from first write, not from first revision |
| **Preserved focus / selection, in this medium's terms** — a reviewer who had cited `FL-004` and returns to that anchor finds the same item, and a reader mid-file is not relocated by a re-sort | **AC-008** | The id sequence and chronological order are unchanged; no severity-ordered re-sort is present in the diff |
| **Keyboard reachability** — the log stays navigable by browser find and stable heading anchors alone; no widget, script or rendering tool is required to read a disposition | **AC-009** | `rg` for `<details>` / `<summary>` / `<script>` returns nothing; the index is a plain markdown table |
| **Exemption is stated, not inferred (IQ-5 / IQ-6)** — an intervention entry and a non-stumble entry say `n/a` rather than being left blank, and an abandonment entry is dispositioned like any other finding | **AC-007** | Every entry carries a non-empty `Disposition:` value; `n/a` appears exactly on exempt kinds |

### Composition invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — a disposition is a *composed* field value with an arm token and a typed payload, never a bare sentence dropped after a colon | **AC-001** (one arm token), **AC-003**, **AC-004**, **AC-005** (the payload is typed per arm) | Each `Disposition:` value parses as `<arm>: <payload>`; the payload is a clause, a resolving path, or an id matching its arm's pattern |
| **Composition and placement** — the disposition lives in the entry's existing `Disposition:` slot and the revision in its existing `Revisions:` slot; the index lands in the existing `## Dispositions index` section. No section is added, renamed or reordered | **AC-008**, **AC-009** | `rg -n "^## " <log>` returns the skeleton's eight headings in order; the diff adds no heading |
| **Transience — persistent chrome vs revealed vs opened-on-demand** | **AC-002**, **AC-009** | *Persistent chrome*: every `Disposition:` and `Revisions:` line, and the `## Dispositions index` derivation notice, are visible by default at all times. *Revealed*: nothing. *Opened on demand*: nothing — no fold, no collapsed admonition, no inactive tab, no external recording, and no tool needed to read any arm |
| **Density budget, with its real numbers** | **AC-001**, **AC-003**, **AC-005**, **AC-006**, **AC-009** | Exactly **one** arm token per `Disposition:` line and the whole field stays on **one line** (the skeleton's shape is six one-line labelled fields). A `routed:` / `escalated:` payload is **exactly one** id token — not a list, not an id plus a hedge. A `Revisions:` entry is **one dated line per revision**, appended. `## Dispositions index` keeps `docs/README.md`'s **two** columns, one row per dispositioned entry, and no third column is added. Any checkbox stays on **one line** — the gate parser matches line by line and a wrapped box can never match |
| **Hierarchy** — `## Chronological record` is authoritative and `## Dispositions index` is derived, and the index says so in its own first line | **AC-009** | The first line of the index asserts derivation and names the record as authoritative; the deletion check proves the direction of dependence |
| **Anti-pattern — a load-bearing item behind a fold, an inactive tab or a collapsed admonition** with no visible-by-default counterpart (`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md`, `## Anti-patterns`) | **AC-009** | The deletion check plus the `<details>` / `<summary>` scan |
| **Anti-pattern — a bespoke navigation widget layered on a medium that renders the equivalent for free**, and inventing a CSS layer, a token file, a new format or a new heading vocabulary | **AC-009** | The diff introduces no script, style file, widget or new section heading; the index reuses `docs/README.md`'s existing table shape |
| **Anti-pattern — colour or emoji as the sole carrier** of an arm or a severity | **AC-005**, **AC-007** | Plain-text read of `git show HEAD:<log>`: every arm is a word token and every severity a legend token; no value is emoji-only |
| **Anti-pattern — the disposition that lives only in the index**, written there because it is quicker once there are twenty of them | **AC-002**, **AC-009** | Every entry carries its own `Disposition:` line; the deletion check leaves the disposition count unchanged |
| **Anti-pattern — the prose destination** ("route to the docs team"), and its sibling, the arm with two ids | **AC-005** | Id-pattern match plus existence check on each destination; exactly one id token per routed or escalated payload |
| **Anti-pattern — the tidy-up pass**: renumbering to close gaps, re-wording a heading beside its new disposition, re-sorting by severity, "correcting" a mark now that the fix is understood | **AC-007**, **AC-008** | Zero changed `Severity:` lines; identical id sequence and heading set; `git diff -U0` confined to the two fields and the index block |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `friction-log-skeleton` landed the log somewhere other than `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` — `references/evaluation/` being the plausible alternative | This story **follows the skeleton and `../_design.md`**, and the `## PR boundary` fenced block is widened **here, deliberately, before the pass begins** — never at gate time, and never by editing the boundary after a file has already been written outside it. Every `<log>` reference in this spec substitutes verbatim |
| **EC-002** | An entry cannot be understood well enough to disposition it — the "what happened" text is ambiguous and U1 is long gone | **The ambiguity is itself a finding about the record**, appended as a new entry with a severity and dispositioned like any other (most plausibly `accepted:` with the reason, or `routed:`). It is **not** a licence to reinterpret, rewrite or "clarify" the original entry, and the logger is not consulted to reconstruct meaning after the fact — a reconstructed entry is a different instrument (`../_decomposition.md`, `## UX brief`, U2's row) |
| **EC-003** | A stumble genuinely has two halves — a doc fix *and* a library bug, say — and both arms feel correct | **One arm on the entry, plus a new appended entry for the second half**, each with its own arm. Two arms on one entry is the double-disposition failure project AC-006 names, and it is the one that reads as diligence. The shape is the constraint and this story does not widen it |
| **EC-004** | The right destination for a routed item is not obvious, or two sibling projects could plausibly own it | Route to the **best-supported** owner and say so in the payload; **whether it is the correct owner is `route-and-escalate`'s to verify** (`../_storymap.md`, `## Coverage`, AC-007 row). Do not park the item on `not yet dispositioned` waiting for certainty — an undispositioned item at hand-off fails AC-001, and a revised route is cheap (AC-006) |
| **EC-005** | A finding implicates a design tension a sibling project already resolved — DT-1's anchor model, DT-6's wrong-model contrast | The `escalated:` arm with the `DT-<n>` id from `.bklg/docs-that-teach/_decomposition.md:150-159`. **Absorbing it as `fixed:` is the forbidden move**: it puts half of a signed-off decision in each of two projects. The escalation's substance and its submission are `route-and-escalate`'s (project AC-011) |
| **EC-006** | A finding implicates a `[FROZEN]` clause in `spec/SPECIFICATION.md`, or a `.kb/` decision atom | Disposition it — `routed:` or `escalated:` — and change nothing. A clause edit requires a new ADR, not an edit (`CLAUDE.md`), and `.kb/` atoms are authored only through the ingest path or closeout; the first hand-authoring attempt was reverted (`0269720`). A deferral disposition records the id of the hand-off that stages the question |
| **EC-007** | The reader hit a genuine library bug rather than a documentation problem | `routed:` to the `support` initiative (`.redkiln/config.yaml:5`; `.bklg/support/initiative.md` is `HS-I0005`). It is **real but structurally empty** — an unfilled template at `stage: intake` with zero projects — so the payload records the destination without implying a story slot exists to receive it |
| **EC-008** | The fix looks like it needs a new page-structure convention or a new navigation affordance | **Not `fixed:`.** That is a routed item for HS-P0021 `page-need-discipline` (UX-AC-012). Choosing `fixed:` here would commit `content-fixes-from-dispositions` to inventing a convention inside a content-fix story |
| **EC-009** | A disposition written earlier in this pass now looks wrong | **Append, do not overwrite** (AC-006): a dated `Revisions:` line carrying the earlier arm verbatim and the reason. This applies within the pass, not only after hand-off — the temptation to "just correct it, nobody has read it yet" is exactly what the slot exists to absorb |
| **EC-010** | The log is thin — the session was abandoned at the third stumble (IQ-6) | Three dispositioned entries satisfy AC-001 exactly as thirty would. A thin log is an **input to `second-session-decision`**, never a reason to hold this pass open, and never a reason to invent entries to pad it |
| **EC-011** | A severity mark now looks plainly wrong — too high, or applied to something that was not a stumble | **Do not touch it** (AC-007). Append a new entry recording the observation about the protocol, with its own severity and disposition. Re-scaling during the disposition pass is the tidy-up failure and it also destroys the inline-application property project AC-004 rests on |
| **EC-012** | The pass is tempted to begin while the session is still running, because a stumble's fix is obvious in the moment | Forbidden from both sides: `../session-run-against-pinned-tree/spec.md:358-361` asserts the `not yet dispositioned` arm exclusively during the session, and this story's own AC-001 counts arms only over a closed record. A disposition written live is the double-disposition failure starting early |
| **EC-013** | `git diff` shows a change outside `Disposition:` / `Revisions:` / the index — most plausibly a whitespace normalisation or an editor's trailing-newline fix | Revert it. AC-008 is checked mechanically on the diff, and the point is not fussiness: a reformatting hunk that touches a `### FL-` heading line makes it impossible to prove by inspection that no label moved |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| **NF-001** | The disposition is legible to a **stranger six months out**: a reviewer who never met the logger and never opened `_design.md` can read any entry and act on it | The UX brief's stated bar for U3, and the reason AC-003 rejects `"noted"`. This is the standing property AC-002 and AC-003 each check one facet of |
| **NF-002** | The log remains readable with **no tool, script, rendering step or network access** — plain markdown, browser find, stable heading anchors, and a `git diff` view with no colour at all | `../_decomposition.md`, `#### The accessibility floor`. AC-009 states it as a criterion; NF-002 states it as a property every later reader inherits |
| **NF-003** | The pass introduces **no new dependency, tool, format, convention or heading vocabulary** — no CSS, no token file, no widget, no sixth arm | `../_design.md`, `## Items`: inventing a primitive layer is out of scope. The arms are the skeleton's five; "partially fixed", "monitoring" and "won't fix" are spellings of existing arms, not new ones |
| **NF-004** | The record's **append-only** discipline is preserved through the pass: the only growth is a `Revisions:` line, and the only mutation is a `Disposition:` field moving off `not yet dispositioned` | The same discipline the chronological record already runs under. It is what makes AC-008's diff check a cheap, total proof rather than a sampling |
| **NF-005** | Repository hygiene is unchanged: `redkiln validate --kb && redkiln doctor` clean at **exactly** the six standing `template-drift` advisories, no more and no fewer | `CLAUDE.md`, `## Where the work lives` — a seventh is an unintended template change, a missing one a reverted customisation. This story changes no template and must move neither number |
| **NF-006** | No credential, access token, private path or third-party personal detail enters a disposition payload | The log is committed to a repository that will be public. A `routed:` id is an id; it is not a person's contact details |
| **NF-007** | The pass is **complete before hand-off, in one sitting per severity band if possible** — dispositioning that trails across days invites the record to be re-read and quietly re-tuned | The only "performance" requirement here. It is a consequence of NF-004 rather than a schedule: every hour the record stays open is an hour in which the tidy-up temptation compounds |

## Implementation notes (non-prescriptive)

Not instructions — the reasoning behind the shape, so the implementer can make the hundred
small calls the criteria do not reach.

**Do the sweep in id order, once, and resist re-entering.** The failure mode is not
laziness; it is the second pass. A first sweep that dispositions everything and a second
that "improves" the wording is how a `Disposition:` line gets edited in place instead of
appended to (AC-006), and how a heading label gets tidied on the way past (AC-008). One
sweep, id order, and any later change goes through `Revisions:`.

**Write the payload before choosing the arm, not after.** If the reason will not write, the
arm is probably wrong: an `accepted:` whose reason keeps coming out as "minor" is usually a
`routed:` nobody wants to route, and a `fixed:` whose referent will not resolve to a path is
usually an `escalated:` — a convention that does not exist yet, rather than a line that is
wrong. This is the cheapest available check against every mis-arm in `## Error conditions`.

**Regenerate the index last, from the file, not from your notes.** The index is derived
(AC-009), and the only way it stays derived is if it is produced by reading the record after
every entry is dispositioned. Building it as you go makes it an authoring surface, and an
authoring surface is one where a disposition can exist that the record does not have.

**Grep before you assert the count.** AC-001's claim is a count over a denominator AC-007
defines; both are one `rg` away and neither is safe to eyeball at twenty entries. The number
that matters is *severity-marked entries* versus *entries carrying an arm* — count them
separately and reconcile, rather than counting `Disposition:` lines and calling it done.

**Prefer `routed:` to a strained `fixed:`.** Routing is cheap and reversible (AC-006);
absorbing a sibling's decision is neither. The risk table's named failure is dispositions
becoming a sink where everything is routed and nothing is fixed — that is real, and
`route-and-escalate` plus `content-fixes-from-dispositions` are the two stories that make it
falsifiable. The reciprocal failure, absorbing everything as `fixed:` because routing feels
like passing the buck, has no downstream check at all.

**Say `n/a` out loud on the exempt entries.** It costs a token and it converts "there is no
disposition here" from an inference into an assertion (AC-007). The reviewer's alternative
is to reconstruct which kinds were exempt, from the shape, six months later.

**When a disposition and a boundary disagree, the boundary wins.** If writing the
disposition makes you want to open a file under `crates/` or `docs/`, that is AC-004 working
as designed. Write what will change and where, and stop — the next story has a gate for it
and this one does not.

## Tests and CI (merge gate)

Grounded in `../_decomposition.md`, `## Testing brief` — its AC/tier table, its
content-fix tier, and its standing prohibition — and in `.redkiln/config.yaml`'s wired
grains. Every static row below asserts on **content**, never on structural presence: the
brief's own example is that a check confirming a `Disposition:` line exists under every
heading would pass `"noted"` as readily as `"routed: HS-P0022, see FL-004"`, and would
report green over exactly the failure it was built to catch.

| tier | command / path | proves |
| --- | --- | --- |
| **Count (static, `rg`)** | `rg -c "^### FL-" <log>` against `rg -c "^Disposition:" <log>`, and `rg -n "not yet dispositioned" <log>` returning nothing on a severity-marked entry | **AC-001** — every entry carries a line, and zero remain on the pre-pass arm. The denominator is AC-007's |
| **Arm shape (static, `rg`)** | Each `Disposition:` value parsed for exactly one of `fixed:` / `accepted:` / `routed:` / `escalated:` / `n/a`; a line carrying two arm tokens fails | **AC-001** — the double-disposition failure, which is the one nobody guards |
| **Payload typing (static, `rg` + `test -f`)** | `routed:` payloads match `HS-[PI][0-9]{4}` or `support` and resolve — `test -f .bklg/support/initiative.md`, `test -f .bklg/docs-that-teach/<sibling>/project.md`; `escalated:` payloads match `DT-([1-9]\|10)` and appear in `rg -n "^\| DT-" .bklg/docs-that-teach/_decomposition.md`; `fixed:` payloads contain a path that resolves | **AC-004**, **AC-005** — a destination is an id that lands, not a description. This is the testing brief's own AC-007 and AC-011 mechanism, run one story early as a guard rather than re-owned |
| **Self-containment (static, `rg`)** | `Disposition:` values scanned for `see \|below\|above\|index\|as noted` — nothing matches | **AC-002** / IQ-1 — the disposition is at the stumble, not a pointer into the index |
| **Immutability of the record (static, `git`)** | `git diff -U0 main...HEAD -- <log>`: every changed line is `Disposition:`, `Revisions:`, or inside `## Dispositions index`. `git diff main...HEAD -- <log>` shows zero changed `Severity:` lines. `rg -n "^### FL-" <log>` and `rg -n "^## " <log>` compared before and after | **AC-007**, **AC-008** / IQ-3 — no renumbering, no re-labelling, no re-scaling, no re-sort, no section moved |
| **Append-only revisions (static, `git`)** | `git log -p --follow <log>` across this story's commits: no `Disposition:` rewrite without a same-commit `Revisions:` append; every entry carries a `Revisions:` line | **AC-006** / IQ-4 — the earlier disposition stays legible, and the slot is used rather than retrofitted |
| **Deletion check (static)** | In a scratch copy, delete `## Dispositions index` and every fold; compare `rg -c "^### FL-" ` and `rg -c "^Disposition:" ` before and after | **AC-009** / IQ-2 — no stumble and no disposition exists only inside a derived view. The UX brief's own stated method, applied verbatim |
| **Plain-text legibility (static)** | `git show HEAD:<log>` read with all styling stripped; `rg -n "<details>\|<summary>\|<script>" <log>` returns nothing | **AC-009**, NF-002 — a screen reader and `git diff` both read every arm; nothing needs a renderer |
| **Mount reachability (static, `rg`)** | `rg -n "_friction-log" .bklg/docs-that-teach/comprehension-evidence/project.md` returns the `## Companions` row | **AC-009** — the one hop from the project card a cold reviewer takes. A disposition recorded anywhere else is not a delivery |
| **Boundary (static, `git`)** | `git diff --name-only main...HEAD` intersected with `crates/ docs/ examples/ spec/ standards/ .kb/` is empty, and every changed path matches the `## PR boundary` fenced block | **AC-004** and the PR boundary — no content fix leaked, no `.kb/` atom authored, no `_design.md` amendment |
| **Artifact-evidence (ledger)** | `.bklg/docs-that-teach/comprehension-evidence/disposition-every-stumble/_ledger.md`, enforced by `redkiln verify --grain story` with `require_ledger: true` (`.redkiln/config.yaml:67`) | Every AC row carries a real `file:line` into the log. This is the tier that carries **AC-003** — whether a reason is a reason — which the brief forbids over-automating, and the substance half of **AC-005** |
| **Story grain** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The repository gate for this diff. Touching no crate, it falls through to the five file-reading lints and `spec-trace` unconditionally rather than passing vacuously on an empty package set |
| **Integration grain** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | DoD-7's non-terminal bar. Expected green and **unchanged** — this story compiles nothing, so a failure here is a signal the PR boundary leaked. The bar bites for real in `content-fixes-from-dispositions`, not here |
| **Backlog hygiene** | `redkiln validate --kb && redkiln doctor` | NF-005 — clean at exactly the six standing `template-drift` advisories, and no `.kb/` atom authored |
| **Explicitly not run** | `cargo xtask ci` (`.redkiln/config.yaml:60`, `e2e`) | The terminal bar is HS-P0025's (`../project.md`, DoD-7). And per the brief: a passing gate says nothing about comprehension — folding the session's evidence into the merge gate confuses two instruments that falsify different things |

## Risks and coupling (PR-scoped)

| Risk | Likelihood / Impact | Mitigation, inside this PR |
| --- | --- | --- |
| The tidy-up pass: entries renumbered, re-labelled or re-sorted by severity because the log now reads badly next to its dispositions | **High** / High | This is the single most likely breach, because this is the first pass with a *reason* to want it. AC-008 is checked on the diff itself (`git diff -U0` confined to two field names and one section), and AC-007 separately pins every `Severity:` line at zero changes. Neither is a sampling check |
| A double disposition written as diligence — "fixed, and also routed in case it comes back" | Medium / High | AC-001's arm-token check fails a line carrying two arms; EC-003 gives the legitimate move (one arm plus a **new appended entry**) so the correct path is available rather than merely the wrong one forbidden |
| `accepted:` used as a bin for everything hard, with reasons that say nothing | Medium / High | AC-003 is Artifact-evidence by design and the brief forbids automating it; the deny-list check catches the crudest cases only. The structural mitigation is EC-004's: routing is cheap and reversible, so there is a legal destination for an item nobody wants to accept |
| Dispositions become a sink — everything routed, nothing fixed (`../project.md`, risk table, row 3) | Medium / Medium | Not repairable inside this PR alone, and stated so. AC-005 forces every route to name a resolving id, `route-and-escalate` verifies the owner, and `content-fixes-from-dispositions` has to land what `fixed:` asserted — the three together make the sink falsifiable. The reciprocal risk, absorbing a DT-shaped finding as `fixed:`, is AC-005's escalation clause and EC-005 |
| A one-line content fix lands here because it is faster than describing it | Medium / High | AC-004's boundary check is a `git diff --name-only` intersection, and the `## PR boundary` fenced block is read by `redkiln verify --grain story`. The implementation note states the tell: wanting to open a file under `crates/` is the criterion working, not failing |
| A disposition is edited in place because "nobody has read it yet" | Medium / Medium | AC-006 checks the *commit history*, not the end state, so an in-place edit is visible even when the final text looks correct. EC-009 states that the rule applies within the pass, which is exactly where the excuse lives |
| The index becomes the authoring surface and entries are dispositioned into it | Low / High | AC-009's deletion check fails immediately, and the implementation note fixes the order: regenerate the index **last, from the file**. This is the failure IQ-2 was written for |
| The log is thin and the pass feels premature | Medium / Low | EC-010: a short log is the whole set. `second-session-decision` owns the "was this enough" verdict (project AC-010) and this story neither pre-empts nor waits on it |
| `../_design.md` or the skeleton's shape looks wrong mid-pass | Low / High | `_design.md` is signed off and its commit date is what makes project AC-002's provenance check meaningful — amending it retroactively destroys that check. The shape defect is a **finding**, appended and dispositioned (EC-002), and the arm vocabulary is not widened (NF-003) |
| The log's home differs from `_friction-log.md` and this spec's paths go stale | Low / Low | `friction-log-skeleton`'s spec binds `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` and mounts it at `../project.md`'s `## Companions`, so the two agree today; EC-001 covers divergence and the `## PR boundary` glob already covers the whole project folder |

**Coupling, stated once.** Upstream, this story cannot start until
`session-run-against-pinned-tree` has closed the record — there is nothing to disposition,
and dispositioning early is the double-disposition failure beginning during the session
(EC-012). Downstream, both slice-mates take their input from here and neither can repair a
defect in it: `route-and-escalate` cannot verify an owner for an item with no destination,
and `content-fixes-from-dispositions` cannot land a fix that no `fixed:` payload described.
`second-session-decision` also depends on this story, and takes from it the shape of the
findings rather than their count.

## Dependencies

**Blocks on** — matching `depends_on` in `../_storymap.md` exactly.

| Story slug | What it must have landed before this pass begins |
| --- | --- |
| `session-run-against-pinned-tree` | The **closed** chronological record: every `FL-###` entry written, severity applied inline from the named legend, interventions timestamped, the end state recorded (completion or `Kind: abandonment`), and every `Disposition:` field sitting on the `not yet dispositioned` arm with its `Revisions:` slot present and defaulting to `none` (`../session-run-against-pinned-tree/spec.md:416`, `:488`). Without it there is nothing to disposition; with it half-done, the count in AC-001 is taken over a moving denominator |

Transitively, through that story: `friction-log-skeleton` (the eight sections, the `FL-###`
id convention, the six one-line entry fields, the five disposition arms and the `Revisions:`
slot — `../friction-log-skeleton/spec.md:248-259`), `non-insider-recruitment`, and
`dt9-and-fixed-protocol` (the severity legend this pass reads but never edits).

**Unlocks** — directly, and both are slice-mates implemented in the same context:

| Story slug | What it takes from this story |
| --- | --- |
| `route-and-escalate` | The `routed:` and `escalated:` arms with their id payloads — the set of items that have a destination at all. It verifies the destination is the *correct* owner, submits the log to a named owner and records that submission (project AC-007, AC-011); none of that is decided here |
| `content-fixes-from-dispositions` | Every `fixed:` payload, each naming what will change and where. It lands the fix and clears `cargo xtask ci --fast` for it — the only story in this project where that bar bites |
| `second-session-decision` | The dispositioned findings, as the input to "were these dominated by a single blocking defect?" (project AC-010). It takes the shape of the set, not this story's verdict on it |

And transitively, through `route-and-escalate`: `handoff-note-to-closeout`, the sole input
HS-P0025 `durable-audience-closeout` cannot manufacture.

## Anchors (progressive disclosure)

Load-bearing depth, deferred but not optional. The `## Context pack` above is self-sufficient
to begin the pass; open these at the stated moment. **Link, never paste in bulk.** Every path
below was confirmed present in this worktree.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/comprehension-evidence/friction-log-skeleton/spec.md` | Fixes the exact shape this pass writes into: the eight sections in order, the `### FL-###` frozen heading line, the six one-line entry fields, the **five disposition arms** and their payload types, the `Revisions:` slot defaulting to `none`, and the `## Dispositions index` two-column derived shape. Its `## Behavior and interfaces` table (lines 248–264) is the field-by-field contract this story is forbidden to widen | **Before writing the first disposition**, and again before regenerating the index | AC-001, AC-005, AC-006, AC-009 |
| `.bklg/docs-that-teach/comprehension-evidence/session-run-against-pinned-tree/spec.md` | The other side of the seam: it asserts every arm sits on `not yet dispositioned` at hand-off, that the id space has no forwarding mechanism (lines 448–458), and that no disposition was pre-empted during the session. Reading it is how you know the denominator AC-001 counts over is closed and honest | **At pre-flight**, before the sweep starts — to confirm the record is closed and no arm was written live | AC-001, AC-007, AC-008 |
| `.bklg/docs-that-teach/comprehension-evidence/_decomposition.md` | The UX brief's seven interaction-quality invariants (IQ-1…IQ-7), the accessibility floor in this medium's own terms, the three-user table that frames every criterion here, and the testing brief's AC/tier table with its **named prohibition** on automating AC-006 into a presence check | Open the UX brief when judging whether a disposition is legible at the stumble; open the testing brief **before writing any check**, because it names by example the check you must not write | AC-002, AC-003, AC-007, AC-009 |
| `.bklg/docs-that-teach/comprehension-evidence/project.md` | AC-006's literal wording (the criterion this story traces), derived requirement 7, the eleven-AC set, DoD item 2, and the risk-table row on dispositions becoming a sink. Also the `## Companions` list that is the mount's one hop | When writing the ledger's `criterion` values, and whenever an arm choice feels like it might belong to a different AC | AC-001, AC-004, AC-005 |
| `.bklg/docs-that-teach/comprehension-evidence/_storymap.md` | The `## Coverage` split of AC-006 between this story and `content-fixes-from-dispositions`, the AC-007 / AC-011 rows that belong to `route-and-escalate`, and `## Merge order` §3 which puts this story first in the slice | The moment a disposition starts to feel like it should also *do* something — the split is stated there rather than negotiable | AC-004, AC-005 |
| `.bklg/docs-that-teach/comprehension-evidence/_design.md` | The signed-off design (2026-08-17, no conditions). It declares `hasSurface: false` with an **empty `## Items` fence**, and its prose binds the primitive layer every composition invariant here derives from — *"there is no CSS layer and no token file, and inventing one is out of scope"*. **Read-only: amending it destroys project AC-002's provenance check retroactively** | Before adding any format, column, widget or arm that is not already in the shape | AC-009 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The one Accepted `.kb` atom that binds this story. Its test — *whether the edit changes what the document asserts* — is the authority behind append-a-revision rather than edit-in-place, applied one level out from doc comments | The first time a disposition already written looks wrong (EC-009) | AC-006 |
| `.bklg/docs-that-teach/_decomposition.md` | The ten-row `## Design tension ownership` table at lines 150–159 — the **only** legal vocabulary for an `escalated:` payload — plus the initiative DoD row 6 this story makes checkable | When a stumble looks like it would reopen a sibling's resolved decision, before choosing between `fixed:` and `escalated:` | AC-005 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | The anti-pattern list the composition invariants cite by name: a load-bearing item behind a fold or collapsed admonition with no visible-by-default counterpart, and a bespoke navigation widget over a medium that renders the equivalent for free | When the index starts to feel like it wants a filter, a fold or a third column because there are twenty entries | AC-009 |
| `.bklg/docs-that-teach/_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md` | The primary evidence behind Decision 5 and behind the whole disposition pass: a log with no destination is a diary, and a check that cannot fail is decoration. It is the argument that answers the pull to write a presence check and call AC-001 automated | When tempted to lighten AC-003's reviewer burden with a script | AC-003 |
| `.bklg/docs-that-teach/comprehension-evidence/_grounding.md` | The verified grounding for this project's method claims, including `:69-71` (routing is what makes a log evidence) and `:96-152` (the destination vocabulary and the reciprocal edges two sibling projects already declared) | When a routing decision needs its source, or when confirming a sibling actually agreed to receive findings in this shape | AC-005 |
| `.bklg/support/initiative.md` | The `support` initiative destination itself — `HS-I0005`, real, at `stage: intake`, with zero projects. Reading it is how you write a `routed: support` payload that is honest about what receiving looks like | Before routing an incidental library bug (EC-007) | AC-005 |
| `docs/README.md` | The two-column routing-table primitive the `## Dispositions index` reuses rather than replaces. **Read for its shape; never edited by this story** | When regenerating the index, last | AC-009 |
| `.redkiln/config.yaml` | The wired gate grains this PR is actually checked by: `support_initiative` (line 5, the routed destination), `affected_gate` (line 40), `integration_scoped` (line 55), `e2e` (line 60 — explicitly not this project's), `require_ledger` (line 67) | When running the merge gate, and when writing a `routed:` payload that names the support initiative | AC-004, AC-005 |
| `.redkiln/templates/_ledger.md` | The ledger shape and its rules: planning authors every row `satisfied: false`; the implementer may only flip a row with real `file:line` evidence, and may never re-word a criterion or flip a satisfied row back | When filling `_ledger.md` after the sweep | AC-001, AC-003 |

## Clarifications resolved during spec

**The AC set is exactly the nine the front half enumerated.** None added, none dropped.
AC-001 … AC-009 above carry the same ids, in the same order, with the same subjects as the
closing paragraph of `## Behavior and interfaces`, promoted to full GIVEN/WHEN/THEN criteria.
The `_ledger.md` carries exactly these nine rows and no others.

**One traced project AC, and it is carried by five rows rather than one.** `../_storymap.md`
traces this story to project AC-006 alone. Rather than mirror it as a single criterion, this
spec splits it along the axes that can independently fail: the count in both directions
(AC-001), each arm's payload being a payload (AC-003, AC-004, AC-005), and the denominator
the count is taken over (AC-007). A single "every stumble has one disposition" row would be
satisfied by twenty entries reading `accepted: noted`.

**Project AC-011 is guarded here and owned next door, deliberately.** Recognising that a
stumble is DT-shaped is inseparable from choosing its arm, so AC-005 requires the
`escalated:` arm rather than an absorbed `fixed:`. The DT id's *correctness*, the submission
to a named owner, and the verification that a destination is the right owner are
`route-and-escalate`'s per `../_storymap.md`'s `## Coverage`. The seam is stated rather than
discovered at review because the failure it prevents — half of a signed-off decision in each
of two projects — is invisible in a passing gate.

**Where the composition invariants come from, given `hasSurface: false`.** `../_design.md`
declares no surface id and an empty `## Items` fence, so there is no signed-off surface to
bind to in the usual sense. That is treated as a *constraint*, not an exemption: the
composition family is taken from what `_design.md` does bind in its `## Items` prose — the
named document primitive layer and the explicit ruling that inventing a CSS or token layer is
out of scope — joined to the UX brief's accessibility floor and the entry shape
`friction-log-skeleton` fixes. Nothing in `## Interaction quality` is a new decision, and
every invariant there is carried by a row in the table above.

**"Verifying test" means a real check, not a test file.** This project has no functions and
no test binary, and the testing brief says so plainly. Each AC's verification is a real
`rg` / `git` / `test -f` / deletion check against a real path, plus the ledger's `file:line`
discipline — and per the brief's own warning, each static check asserts on **content** rather
than structural presence, so none of them is a check that nothing can fail.

**The mechanical/substantive split is adopted, not invented.** `../_decomposition.md`'s
testing brief names the exact wrong implementation ("a script that merely confirms a
`Disposition:` line exists") and states the repair: assert on content — a destination id
pattern, a resolving path — or leave it in the Artifact-evidence tier. AC-003 is the one row
that stays wholly reviewer-read, and that is the brief's ruling rather than this spec's
concession.

**No decision atom is cited for this story's core subject because none exists.** All
seventeen Accepted atoms under `.kb/decisions/` concern port flavours, crate naming, opaque
payloads, MSRV, error traits, append conditions, position assignment, event identity,
validated identifiers and the wire format. The one atom that binds is
`.kb/governance/rewrite-the-referent-never-the-reasoning.md`, at the revisions seam (AC-006).
Stating the absence is the honest form; manufacturing an ADR number would not be.

**The log does not exist at spec time, and that is not a blocker.** `test -f` on
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` fails today; the file is
`friction-log-skeleton`'s to land and `session-run-against-pinned-tree`'s to fill, and both
precede this story in the merge order. EC-001 covers the case where `../_design.md` fixes a
different home, and requires the `## PR boundary` block to be widened deliberately before the
pass begins rather than at gate time.
