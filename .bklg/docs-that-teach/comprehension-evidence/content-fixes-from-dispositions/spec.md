---
item: HS-S0168
stage: spec
created: 2026-08-17T13:16:21.305Z
updated: 2026-08-17T13:16:21.305Z
template_sig: 87bbf1d0
rendered_sig: 1ffbc6bb
---

# Spec — Land the small content fixes a "fixed" disposition asserts

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — BR-05, BR-06, BR-14; the two-proof-artefacts framing this story sits inside |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) — `### Definition of Done` (row 6 is this story's target), `## Design tension ownership` (lines 150–159: the DT ids an escalation must carry), `### Merge order` |
| Project | [`.bklg/docs-that-teach/comprehension-evidence/project.md`](../project.md) — AC-006 is this story's traced criterion; AC-011 is its guard rail; DoD-7 is its integration bar |
| This spec | `.bklg/docs-that-teach/comprehension-evidence/content-fixes-from-dispositions/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — `## UX brief` (`#### The primitive layer to compose from — do not hand-roll`, IQ-1/IQ-3/IQ-4/IQ-7, **UX-AC-012** which is this story's tightest constraint) and `## Testing brief` (`**Content-fix tier, orthogonal to the table above**` — the four tiers a fix clears, named by command) |
| Grounding | [`../_grounding.md`](../_grounding.md) — `## Routing destinations for AC-007` (the destinations a *non*-fix disposition takes) and `## Existing code/precedent patterns worth citing, with a caution` (lines 184–198: do **not** cite `examples/outside-projection-adapter/`, it is unreachable from this worktree) |
| Signed-off design | [`../_design.md`](../_design.md) — signed off 2026-08-17, no conditions. `hasSurface: false`, `## Items` empty (`# no items — no public API surface, no rendered UI surface`). Binding on this story as a **prohibition**: this project adds no `pub` item and invents no format, and where `_design.md` fixes the friction log's path it wins verbatim over any path named below |
| Roadmap pointer | [`../_storymap.md`](../_storymap.md) — `## Merge order`, step 3 (`dispositions-and-routing`): `disposition-every-stumble` → then this story and `route-and-escalate`, independent of one another, in either order |

## One-line PR slice

Land the small content fixes whose disposition is "fixed", composing from the existing primitive
layer — doc-comment sections and intra-doc links, `docs/README.md`'s routing-table shape, compiled
examples — and prove each one through `cargo xtask affected` and `cargo xtask ci --fast`.

## Executive summary

This PR turns one arm of an already-written disposition into a fact about the tree. `disposition-
every-stumble` has, by the time this story starts, given every severity-marked stumble in
`_friction-log.md` exactly one arm; where that arm reads `fixed:`, the log currently asserts a change
that has not happened. This PR makes the assertion true — it edits the pages the log implicates, fills
each `fixed:` arm's `<ref>` so it resolves, and clears the gate.

The delta against the project charter is a narrowing, not an addition. `project.md`'s `## In scope`
admits "small content fixes arising from dispositions **where the fix does not reopen a settled design
tension**"; `_storymap.md`'s `## Coverage` narrows AC-006 further, giving this story "only the **fixed**
arm — landing the fix a 'fixed' disposition asserts, and clearing the gate for it". Everything this spec
adds is the shape of that arm: which page a fix may touch, which primitives it may compose from, what
happens when a fix turns out to be an escalation in disguise, and what "clearing the gate" costs.

Three consequences are worth stating up front because they are counter-intuitive:

- **This is the only story in HS-P0024 that touches code**, and therefore the only reason DoD-7's
  `cargo xtask ci --fast` bar bites here rather than being a formality (`../_storymap.md`,
  `### Why the slices fall here`).
- **Landing zero fixes is a legitimate, complete outcome.** If no stumble was dispositioned `fixed`,
  this PR changes no page. Manufacturing a fix to justify the story is the failure, not the empty diff.
- **This story does not gate the hand-off.** `handoff-note-to-closeout` has no `depends_on` edge to it,
  deliberately: *"a fix that lands late must not be able to hold up the evidence HS-P0025 needs"*
  (`../_storymap.md`, `### Dependency graph`, closing paragraph).

## Context pack

**Read this section and you can start. Everything below `## Anchors` is deferred depth, not optional
depth — open an anchor when the AC it is bound to says to.**

### The decision this story exists to make

**A `fixed:` disposition is a claim about the repository, and this story is the only thing that makes
it true.** `friction-log-skeleton` typed the disposition slot to exactly five arms — `not yet
dispositioned` / `fixed: <ref>` / `accepted: <reason>` / `routed: <id>` / `escalated: DT-<n>` — and
`disposition-every-stumble` chose one per stumble. Four of the five are complete the moment they are
written: an acceptance carries its reason, a route carries its id, an escalation carries its DT id.
`fixed:` is the one arm whose truth lives outside the log. Until the page changes and the `<ref>`
resolves, a `fixed:` arm is the most dangerous entry in the file — it reads as resolved to every
downstream actor and nothing anywhere fails.

### The persona-journey slice this realizes

Two people, at two different times, and the fix serves them differently:

- **U3, the downstream actor, reading the log to decide what is left** (`../_decomposition.md`,
  `## UX brief`, the three-user table). What must never happen to them is *"reaching an item whose
  destination is a description rather than an id, or whose disposition is implied by silence."* A
  `fixed:` arm with an unresolvable `<ref>` is that failure wearing the friendliest of the five labels.
- **The next reader who walks the same path U1 walked.** They never see the log. The fix is only real
  for them if it landed on the page that stopped U1 — which is why the wrong implementation this story
  rejects is stated as *"fixes applied to the pages the author found easiest to change rather than the
  ones the log implicates"* (`discover.md`, `## The wrong implementation`).

### The decisions this story must honor, stated as decisions

1. **The seam with `disposition-every-stumble` is: they choose the arm, this story discharges it.**
   `../_storymap.md`'s `## Coverage` splits AC-006 exactly there. This story does **not** re-decide
   whether a stumble should have been `fixed` rather than `routed`; if the fix turns out to be
   impossible or out of scope, the arm changes by revision (decision 7), it is not silently re-argued.

2. **Fix the page the log implicates, not the page that is easiest to change.** Every fix in this PR
   traces stumble id → implicated page → diff hunk. A fix with no stumble behind it is not in scope
   here at all: it is an unevidenced content change, and this initiative has four other projects whose
   job that is.

3. **Compose from the existing primitive layer; a fix that needs a new convention is not a fix.**
   UX-AC-012 is verbatim binding: a fix uses
   [`standards/rust/70-rustdoc-obligations.md`](../../../../standards/rust/70-rustdoc-obligations.md)'s
   doc-comment sections and intra-doc links, `docs/README.md`'s routing-table shape, or a compiled
   example per
   [`standards/rust/62-doctests-and-harnesses.md`](../../../../standards/rust/62-doctests-and-harnesses.md).
   *"It introduces no new navigation widget, no new page-structure convention, and no `ignore`-fenced
   snippet. Where the fix looks like it needs a new convention, that is a routed item for HS-P0021
   `page-need-discipline`, not an invention here."* Two of those forbidden moves have named
   alternatives already in the constitution: RS-62-4 gives the discarding-macro pattern for usage that
   must not compile, and RS-70-2 forbids an intra-doc link that resolves in only some feature
   configurations — which is exactly the link a hasty fix reaches for.

4. **A fix that would reopen a settled design tension is not a fix; it is an escalation carrying the
   DT id.** AC-011, and the ten DT ids are in `../../_decomposition.md:150-159`. The four content
   tensions were signed off in HS-P0022's design review, so re-deciding DT-1 (which prior mental model
   the teaching anchors against), DT-4, DT-5 or DT-6 here would *"put half a decision in each"*
   (`../project.md`, risk table, row 4). The same holds for DT-2/DT-3/DT-8 (HS-P0021), DT-7 (HS-P0020)
   and DT-10 (HS-P0023). The test is not "is this edit large"; it is **does the edit change what a
   sibling project decided**.

5. **Rewrite the referent, never the reasoning.** Binding at one specific seam:
   [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md)
   — *"ask whether the edit changes what the document asserts, not whether it changes the document."*
   Where a fix rewrites a `happenstance-core` doc comment that discharges a `SPECIFICATION.md` clause,
   the mechanical test from
   [`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`](../../../../.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md)
   applies: **a correction is a repair if the set of implementations it admits is unchanged, and
   otherwise it is a gap — and a gap is an ADR's, which this story is not authorised to write.** Record
   the gap and route it; do not decide it.

6. **`spec/SPECIFICATION.md` cites source by line range, and nothing in the gate resolves those
   ranges.** `spec/SPECIFICATION.md:371` cites `crates/happenstance-core/src/store.rs:93-268`. A
   doc-comment insertion above line 93 shifts that range silently: `cargo xtask spec-trace` checks
   clause↔rule cross-references and regenerates §7.1/§7.2 (`xtask/src/spec_trace.rs`, module docs), it
   does **not** resolve `file:line` citations. So a fix that adds lines to a file the specification
   cites carries a hand check that the gate cannot do for it.

7. **A changed disposition appends; it never overwrites.** IQ-4, and the same governance atom applied
   to dispositions: *"the referent may be rewritten, the reasoning may not be erased."* If a fix
   attempted here proves wrong, out of scope, or larger than "small", the entry's `Revisions:` slot
   takes a dated line carrying the earlier disposition and the reason — the original `fixed:` text
   stays legible. And IQ-3 binds absolutely: the entry's id and heading line, label included, are
   frozen. **This story edits the disposition and revisions slots of an entry and nothing else in the
   chronological record.**

8. **Proof is the ordinary tiers, run in the ordinary order — no new tier is warranted.**
   `cargo xtask affected --base main` at story grain (`.redkiln/config.yaml:40`), then
   `cargo xtask ci --fast` (`.redkiln/config.yaml:55`, and DoD-7 verbatim). The testing brief is blunt
   about why: *"A content fix that skips that gate is not 'documentation, so untested' — it is
   untested."* Doctests are this project's stand-in for a unit tier, because it has no functions.

9. **Zero fixes is a complete outcome.** The five arms are exclusive and one of them is `accepted`. If
   `disposition-every-stumble` produced no `fixed:` arm, this PR lands no content change and says so.
   The failure mode this guards against is the mirror of decision 2: inventing work to make a story
   look delivered.

10. **This project edits nothing under `.kb/`, and this story authors no atom and no ADR.** Both are
    named non-goals — hand-authoring atoms outside the ingest path was reverted once already
    (`0269720`), and ADR authorship belongs to the runbook's own pass, never to a fix. A fix that
    needs a decision is a gap, and decision 5 says what to do with it.

### What this story is explicitly not deciding

Which arm any stumble got (`disposition-every-stumble`'s). Where a routed or escalated item goes, or
whether the log was submitted to an owner who can act (`route-and-escalate`'s, AC-007/AC-011). Whether
a second session runs (`second-session-decision`'s). What the hand-off note says
(`handoff-note-to-closeout`'s). And every content decision the four sibling *projects* own — the
one-need-per-page rule, the taxonomy, the opening encounter, the anchor model, the front door
(`../project.md`, `## Out of scope`).

### The ordering constraint that can invalidate this work

`disposition-every-stumble` must be committed first (`blocked_by: HS-S0166`). There is nothing for this
story to discharge until an arm says `fixed:`, and a fix landed ahead of its disposition inverts the
instrument: the log would then be recording what the author already changed rather than what the reader
stumbled on. If the friction log carries no `fixed:` arm when this story starts, that is decision 9's
outcome, not a licence to start choosing arms.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice: the next reader of the implicated page no longer meets the stumble, and U3 reading the log can see that it was discharged (`../_storymap.md`, slice table) |
| **Slice / milestone** | `dispositions-and-routing` |
| **Slice-mates** | `disposition-every-stumble` (lands **before**; writes the arm this story discharges) and `route-and-escalate` (independent of this story, either order). All three are implemented in one context as one integrated surface |
| **Mount point** | [`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md`](../_friction-log.md) → the `Disposition: fixed: <ref>` field of each fixed stumble in `## Chronological record`. That field is the render path by which U3 — a sibling-project owner, the `support` initiative, HS-P0025 — reaches this story's output at all. A page edited with no arm completed is constructed-but-unmounted; a `fixed:` arm whose `<ref>` resolves to nothing is worse, because it reads as mounted. **The path is `_design.md`'s to fix**: where `../_design.md`'s protocol section names a different one, that wins verbatim (`friction-log-skeleton/spec.md`, EC-001) |
| **Wires into** | The three document surfaces the UX brief enumerates as "the material the reader walks": [`docs/README.md`](../../../../docs/README.md) (its two-column routing table is both a primitive to compose from and a page a fix may land in), the rustdoc of [`crates/happenstance-core/src/`](../../../../crates/happenstance-core/src/store.rs) and [`crates/happenstance/src/lib.rs`](../../../../crates/happenstance/src/lib.rs), and [`examples/course-subscriptions/`](../../../../examples/course-subscriptions/). Plus the ledger contract — `.redkiln/templates/_ledger.md`'s `evidence: "file:line"` rows are what make a stable `FL-###` anchor worth citing |
| **Design-system primitives consumed** | RS-70-2/70-3/70-5's doc-comment sections and intra-doc links; RS-62's compiled-example patterns (RS-62-2's hidden runtime, RS-62-4's discarding macro in place of an `ignore` fence); `docs/README.md`'s two-column routing table; the one-line checkbox from `.redkiln/templates/gates/`. **No new format, no new heading vocabulary, no navigation widget** (`../_decomposition.md`, `#### The primitive layer to compose from — do not hand-roll`) |
| **Public items** | **None.** `../_design.md`'s `## Items` block is empty and every shape section reads `N/A — no user-facing surface`. This story adds no `pub` item, changes no signature and changes no visibility. An edit that alters a signature has stopped being a content fix |
| **Renders surfaces** | **No `_design.md` item id** — there are none to render. In the UX brief's own enumeration it *changes* surface **1** (the material the reader walks) and *writes into* surface **2** (the friction log), and it must not touch surface **3** (`_design.md`, the protocol — editing the protocol after the session is the retrofit AC-002's provenance check exists to catch) |
| **Conformance rule(s) / clause(s)** | **No new conformance rule and no clause change.** A content fix is not adapter-observable: it adds no rule to `suite.rs` and discharges no `SPECIFICATION.md` clause. The one specification-shaped obligation is decision 6's — a fix that shifts a cited line range repairs the citation, and a fix that would change what a `[FROZEN]` clause *asserts* is a gap for an ADR, not an edit (`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`) |
| **Advances DoD scenario** | **Initiative DoD scenario 6** — *"Every stumble in that log has a disposition"* (`../../_decomposition.md`, `### Definition of Done`, row 6; owner HS-P0024). `disposition-every-stumble` makes every arm present; this story is what makes the `fixed:` arms **true**. It must also leave scenarios 1 and 13 green — the gate stays green and nothing load-bearing becomes hidden from the check |

**Delivered mounted, not as an isolated component.** The acceptance bar for this PR includes that every
page this PR edits is reachable from a `fixed:` arm in the log, and every `fixed:` arm in the log is
reachable to a page in this PR — the correspondence runs both ways, or one side is lying.

## PR boundary

**In this PR**

- The content fixes themselves, one per `fixed:` stumble, in the pages that stumble implicates:
  doc comments under `crates/happenstance-core/src/` and `crates/happenstance/src/`, prose or the
  routing table in `docs/README.md`, and the worked example under `examples/course-subscriptions/`.
- The completed `<ref>` on each `fixed:` arm in `_friction-log.md`, and a `Revisions:` line wherever a
  disposition changed as a result of attempting the fix.
- A repair to a `spec/SPECIFICATION.md` citation whose line range this PR's own edits shifted —
  citation only, never a clause's normative sentence.
- This story's own `_ledger.md` and stage artifacts.

**Explicitly not in this PR**

- **Any content change with no stumble behind it.** Including one the author believes is an
  improvement. The four sibling projects own unevidenced content.
- **Any edit that reopens a settled design tension.** It leaves as an escalation carrying its DT id
  through `route-and-escalate`, not as a diff here (AC-011).
- **Any new page, new page-structure convention, or new navigation widget** — routed to HS-P0021
  `page-need-discipline` (UX-AC-012). A new page would also need pinning by path in `xtask/src/`,
  which is HS-P0020's, and that is the tell that the change has left this story.
- **Any `pub` item, signature, visibility or feature-gate change.** `../_design.md` declares no items;
  an API change is a different deliverable with a different sign-off.
- **Any edit to `../_design.md`**, to a stumble's id, heading line or narrative text, or to the
  chronological order (IQ-3). This story writes two slots, and only those two.
- **Any file under `.kb/`, and any ADR.** Atom authorship is closeout's, through the ingest path; a
  decision a fix needs is a recorded gap, not a record this story writes.
- **The library bug the reader may have found.** It routes to the `support` initiative
  (`.redkiln/config.yaml:5`) as a disposition — `../project.md`, `## Out of scope`, last bullet — and
  is `route-and-escalate`'s, not a code fix here.

**Merge DoD one-liner** — every `fixed:` arm in `_friction-log.md` resolves to a diff hunk in this PR
and every diff hunk in this PR resolves to a `fixed:` arm, with `cargo xtask affected --base main`
green at story grain, `cargo xtask ci --fast` green at integration grain, and `redkiln validate --kb &&
redkiln doctor` clean at exactly the six standing `template-drift` advisories `CLAUDE.md` documents.

**Paths this story may touch** — `redkiln verify --grain story` reads the first fenced block under this
heading and fails on any file changed outside it.

```
docs/**
crates/happenstance-core/src/**
crates/happenstance/src/**
examples/course-subscriptions/**
spec/SPECIFICATION.md
.bklg/docs-that-teach/comprehension-evidence/_friction-log.md
.bklg/docs-that-teach/comprehension-evidence/content-fixes-from-dispositions/**
```

The two `crates/**` globs are wider than this story's honest reach — git cannot express "doc comments
only" as a path — so decision 3 and the `## Behavior and interfaces` rows below carry that half of the
boundary, and the acceptance criteria check it. `xtask/**`, `standards/**`, `.kb/**` and
`.bklg/docs-that-teach/comprehension-evidence/_design.md` are deliberately absent: a change that needs
one of them has left this story, and the correct move is to route it, not to widen the block.

## Behavior and interfaces

The "interface" here is the correspondence between a log entry and a diff: what a fix is allowed to be,
what it must leave behind, and what makes it stop being a fix.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The input set is exactly the `fixed:` arms** | Enumerate the stumbles whose `Disposition:` arm reads `fixed:` in `_friction-log.md`'s `## Chronological record`. That set — no more, no less — is this PR's work list. An entry with any other arm is untouched by this story; an entry with `not yet dispositioned` means the dependency has not landed | `../_storymap.md`, `## Coverage`, AC-006 row (*"the second owns only the **fixed** arm"*); `friction-log-skeleton/spec.md`, the disposition-slot row |
| **Each fix names its stumble, and each stumble names its fix** | The correspondence is bidirectional and checkable in both directions: every diff hunk traces to an `FL-###`, and every `fixed:` arm's `<ref>` resolves to a real path (and, once committed, a commit) in this PR. `<ref>` is a path plus an anchor or line, in the `file:line` shape `.redkiln/templates/_ledger.md`'s `evidence` rows already use | `../_decomposition.md`, `## Testing brief` (the `evidence: "file:line"` ledger contract); IQ-7's *"destinations are ids, not descriptions"* applied to the `fixed` arm |
| **The implicated page, not the convenient one** | The page a fix lands in is the one the stumble's "What happened" field names — the file the reader opened, the link they followed, the search they typed. Where a stumble implicates a page that a sibling project owns, the fix is still landed here if it is small and reopens nothing; the ownership question is about *tensions*, not about files | `discover.md`, `## The wrong implementation`; `../project.md`, `## In scope`, small-content-fixes bullet, and `## Risks and coupling notes`, `**Coupling notes**` (*"a friction log is worthless if it cannot cause a change"*) |
| **Doc-comment fixes use the existing heading vocabulary** | `# Errors` naming the conditions rather than the error type, `# Panics`, and intra-doc links. No new heading is invented; RS-70-2 forbids a link that resolves in only some feature configurations, and RS-70-3 forbids reaching for `allow(clippy::doc_markdown)` instead of teaching `doc-valid-idents` a proper noun | [`standards/rust/70-rustdoc-obligations.md:93`](../../../../standards/rust/70-rustdoc-obligations.md) (RS-70-2), `:152` (RS-70-3), `:243` (RS-70-5); `../_decomposition.md`, UX-AC-012 |
| **An example a fix adds or repairs compiles** | A fix never lands an `ignore`-fenced snippet — that reproduces the anti-pattern the whole initiative exists to remove. RS-62-4's discarding macro is the pattern for usage that must not compile; RS-62-2's hidden runtime is the pattern for an `.await`ing example; RS-62-1 forbids trusting a lone `compile_fail` fence's error code | [`standards/rust/62-doctests-and-harnesses.md:12`](../../../../standards/rust/62-doctests-and-harnesses.md) (RS-62-1), `:75` (RS-62-2), `:189` (RS-62-4); `../_decomposition.md`, `#### The primitive layer…` |
| **A `docs/README.md` fix reuses its routing table** | The existing two-column "Looking for / It is at" table is the primitive; a fix adds or repairs a row rather than introducing a second navigation shape. The file's own note that two of its targets are read by the gate *by path* is the reason a fix never silently moves a tree | [`docs/README.md`](../../../../docs/README.md) — the table at lines 12–23 and the paragraph immediately below it |
| **The doc comment that discharges a clause: referent yes, reasoning no** | Apply the mechanical test before editing: does the edit change what the document *asserts*? For a `[FROZEN]` clause, does the set of implementations it admits change? If yes it is a gap — record it, route it, and leave the normative sentence verbatim. This is the seam `../project.md`'s risk table row 8 flags as Low likelihood / **High** impact | [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md), `## The test`; [`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`](../../../../.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md), `## The test: repair or gap` |
| **Line-range citations are re-checked by hand** | Adding lines to a file the specification cites by range shifts that range and no gate notices. Re-resolve every `SPECIFICATION.md` citation into a file this PR edited, and repair the range in the same commit as the edit that moved it | `spec/SPECIFICATION.md:371` (`crates/happenstance-core/src/store.rs:93-268`); `xtask/src/spec_trace.rs`, module docs (`# What it does not do`) |
| **An escalation is not a fix, and it leaves through the sibling** | If a fix would reopen a settled tension, stop. The stumble's arm changes from `fixed:` to `escalated: DT-<n>` **as a revision** (the original arm stays legible), and the escalation's substance is `route-and-escalate`'s. The DT id must exist in the ownership table | `../../_decomposition.md:150-159`; `../project.md`, AC-011; `../_decomposition.md`, IQ-4 |
| **A revised disposition appends beside the original** | Dated line in the entry's `Revisions:` slot carrying the earlier disposition and the reason for the change. Never an in-place edit of the `Disposition:` line's original text, and never an edit to the entry's id, heading label or narrative | `../_decomposition.md`, IQ-3 / IQ-4, UX-AC-007 / UX-AC-008; `friction-log-skeleton/spec.md`, the revisions-slot row |
| **The empty case is recorded, not filled** | No `fixed:` arm means no content change. Record that in the implementation report and the ledger with the count, and land nothing under `crates/`, `docs/`, `examples/` or `spec/` | `../project.md`, AC-006 (the five arms are exclusive); `discover.md`, `## The wrong implementation` |
| **Story grain, then integration grain** | `cargo xtask affected --base main` first — it maps the diff to workspace packages plus dependents and, for a diff that touches nothing under `crates/`, falls through to the five file-reading lints and `spec-trace` rather than passing vacuously on an empty package set. Then `cargo xtask ci --fast` | `.redkiln/config.yaml:40` (`affected_gate`), `:55` (`integration_scoped`); `../_decomposition.md`, `## Testing brief`, `### Notes`, first paragraph |
| **Doctests are the unit tier** | `cargo test --workspace --all-features` compiles every doc-comment example a fix touches. `cargo xtask ci --fast` runs it, but a fix that changes an example is worth running it against directly first | `../_decomposition.md`, `## Testing brief`, `**Content-fix tier**`, first bullet; `CLAUDE.md`, `## Commands` |
| **No automated "disposition check" is added** | The testing brief forbids it by name: a script confirming a `Disposition:` line exists would pass `"noted"` as readily as `"routed to HS-P0022, see FL-004"`, and rejects no wrong implementation. Any check this story writes asserts on **content** — a resolving `<ref>`, an existing DT id — or it stays reviewer-read | `../_decomposition.md`, `## Testing brief`, `### Notes`, *Do not automate AC-006 or AC-009…*; `CLAUDE.md`, *a rule that no adapter can fail is decorative* |

## Data and migrations

**N/A — no schema, no store, no migration.** This story defines no type, opens no connection and writes
no row; `../_design.md` records `hasSurface: false` and an empty items block, and every change in this
PR is prose inside an existing markdown file or an existing doc comment.

Two things behave enough like data contracts to be named, because breaking either is silent:

- **The stumble id `FL-###`** is a primary key that other documents foreign-key into. It is assigned in
  occurrence order, never reused, never reassigned, and its heading line — label included — is frozen
  at write time, because the anchor is a public identifier from the moment it is written
  (`../_decomposition.md`, IQ-3). The only "migration" this story performs on the log is filling two
  slots inside an existing entry. Renumbering or re-labelling is not a migration; it is a broken
  citation with no error message.
- **`SPECIFICATION.md`'s `file:line` citations** are references into files this story edits, and
  nothing in the gate resolves them (`xtask/src/spec_trace.rs`, `# What it does not do`). Adding lines
  above a cited range is the closest thing to a schema change available here, and the repair belongs in
  the same commit as the edit that caused it.

## Acceptance criteria

Seven criteria. Each is framed from a persona's intent crossing the whole stack — U3 the downstream
actor deciding what is left, the next reader who walks the path U1 walked, the sibling-project owner
whose signed-off tension a fix could quietly reopen, the repository owner running the gate. A
criterion framed as a bare capability ("the fix is landed") is satisfied by any diff at all, which is
exactly the wrong implementation `discover.md` names. The personas are `../_decomposition.md`'s
`## UX brief` three-user table; the journey they cut through is `../_storymap.md`'s backbone A4 → A5.

Throughout, **the log** means
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` (path superseded verbatim by
`../_design.md`'s protocol section if it fixes a different one), **a fixed arm** means a stumble whose
`Disposition:` field reads `fixed: <ref>`, and **the work list** means the set of fixed arms present
when this story starts.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** U3 — a sibling-project owner, the `support` initiative, or HS-P0025 — opens the log to decide what is still theirs, **WHEN** they read a stumble whose arm reads `fixed:`, **THEN** the `<ref>` resolves to a real path in this PR and to the hunk that discharged it, and conversely every content hunk in this PR traces back to exactly one `FL-###` whose arm reads `fixed:` — so the friendliest of the five labels is a fact about the tree rather than an assertion nothing can fail | Static, both directions, and both are required: for each fixed arm, the `<ref>`'s path passes `test -f` and its anchor or line is present; and `git diff --name-only main...HEAD` restricted to `crates/`, `docs/`, `examples/`, `spec/` maps onto the arm list with nothing left over on either side. Artifact-evidence: `_ledger.md` cites the `file:line` of each arm and its hunk. Rejects the two failures the integration contract names — a page edited with no arm completed (constructed but unmounted) and a `fixed:` arm whose `<ref>` resolves to nothing, which is worse because it reads as mounted |
| AC-002 | **GIVEN** the next reader who walks the same path U1 walked and who will never see the log, **WHEN** they reach the point where U1 stopped, **THEN** the page they open is the page the fix landed in — because every fix traces stumble id → the page named in that stumble's "What happened" field (the file opened, the link followed, the search typed) → the diff hunk, and no hunk lands in a page no stumble names | Artifact-evidence, ledger-cited as a three-column trace per fix (`FL-###` → implicated path → hunk), checked by a reviewer against the log's own narrative field. Rejects `discover.md`'s named wrong implementation verbatim — *"fixes applied to the pages the author found easiest to change rather than the ones the log implicates"* — and rejects the mirror failure decision 2 names: an unevidenced content improvement smuggled in beside a real fix, which the four sibling projects own |
| AC-003 | **GIVEN** a reviewer reading this PR as raw text with every rendering step stripped, **WHEN** they look at any one fix, **THEN** it is composed from one of exactly three permitted primitives — a `# Errors`/`# Panics` doc-comment section or intra-doc link per RS-70, a row in `docs/README.md`'s existing two-column routing table, or a compiled example per RS-62 — and it carries real composed presentation in that primitive's own shape (conditions named in the `# Errors` section, a routing row with both columns filled, an example that compiles) rather than a sentence appended below the existing prose; and the PR contains zero new heading vocabulary, zero navigation widgets, zero `ignore`-fenced snippets, zero `allow(clippy::doc_markdown)`, zero collapsed or `<details>` containers, and no intra-doc link that resolves in only some feature configurations | Static: `rg -n '```ignore' <changed files>` returns nothing; `rg -n "allow\(clippy::doc_markdown\)" <changed files>` returns nothing; `rg -n "<details>\|<summary>\|doc\(hidden\)" <changed files>` returns nothing; every new heading in a changed doc comment is one already used in `standards/rust/70-rustdoc-obligations.md`. Doctest tier: `cargo test --workspace --all-features` compiles every example touched. Artifact-evidence: a reviewer maps each fix to one of the three named primitives. Rejects UX-AC-012's named failure — a fix that needs a new convention, which is a routed item for HS-P0021 — and `interaction-patterns.md`'s anti-pattern, a bespoke widget over a medium that renders the equivalent for free |
| AC-004 | **GIVEN** an adapter author who will rely on a `happenstance-core` doc comment that discharges a `SPECIFICATION.md` clause, **WHEN** a fix rewrites that comment, **THEN** the set of implementations the clause admits is unchanged — the edit is a repair — or the edit is **not made**: the difference is recorded as a gap naming the clause id and routed through `route-and-escalate`, never decided here and never written as an ADR; and every `file:line` citation in `SPECIFICATION.md` whose range this PR's own edits shifted is re-resolved and repaired in the same commit as the edit that moved it | Artifact-evidence against the mechanical test in `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, `## The test: repair or gap`, ledger-cited per affected comment. Static: `rg -n "crates/happenstance-core/src/[a-z_]+\.rs:[0-9]+-[0-9]+" spec/SPECIFICATION.md` enumerates every range citation, each re-resolved by hand against the post-fix file (`spec/SPECIFICATION.md:371` cites `store.rs:93-268`); `cargo xtask spec-trace` green. Rejects the failure `../project.md`'s risk row 8 rates Low/**High**: a normative sentence rewritten because it read badly, and a silently shifted citation the gate cannot see (`xtask/src/spec_trace.rs`, `# What it does not do`) |
| AC-005 | **GIVEN** a sibling-project owner who signed off DT-1, DT-4, DT-5 or DT-6 in HS-P0022's design review, **WHEN** a fix attempted here would change what they decided, **THEN** the fix is not landed: the stumble's arm is revised to `escalated: DT-<n>` with a DT id that exists in the ownership table, the change appends a dated line in that entry's `Revisions:` slot carrying the earlier disposition and the reason with the original `fixed:` text still legible, and the entry's id, heading line (label included) and narrative text are byte-identical to what they were | Static: every `DT-<n>` written by this PR appears in `rg "^\| DT-" .bklg/docs-that-teach/_decomposition.md`; `git diff` over the log shows changes only inside `Disposition:` and `Revisions:` fields — no hunk touching a `### FL-` heading line, the entry order, or a "What happened" field. Artifact-evidence: the revision's reason is legible and the substance of the escalation is `route-and-escalate`'s, not this PR's. Rejects the two failures decision 7 and AC-011 name: an in-place overwrite that erases the earlier arm, and a tension re-decided here as "just a small fix", which would put half a decision in each project |
| AC-006 | **GIVEN** the repository owner merging this PR, **WHEN** they run the bars this repository already defines, **THEN** `cargo xtask affected --base main` is green at story grain, `cargo xtask ci --fast` is green at integration grain — including every doctest a fix touched and all four wasm32 steps — and `redkiln validate --kb && redkiln doctor` is clean at exactly the six standing `template-drift` advisories, with no gate step, no new tool, no new dependency and no automated "disposition check" added by this story | Story gate: `cargo xtask affected --base main` (`.redkiln/config.yaml:40`). Static tripwire: `cargo xtask lints && cargo xtask spec-trace` (`:48`). Integration: `cargo xtask ci --fast` (`:55`, DoD-7 verbatim). Doctest: `cargo test --workspace --all-features`. Hygiene: `redkiln validate --kb && redkiln doctor` (DoD-8). Rejects the testing brief's named failure — treating a content fix as "documentation, so untested" — and the forbidden structural checker that would pass `noted` as readily as a real routed id |
| AC-007 | **GIVEN** `disposition-every-stumble` gave every stumble an arm and none of them reads `fixed:`, **WHEN** this story runs, **THEN** it lands zero changes under `crates/`, `docs/`, `examples/` and `spec/`, and records the fixed-arm count — zero — in the implementation report and in this story's ledger, so an empty diff reads as a complete outcome rather than as work not done, and no fix is manufactured to make the story look delivered | Static: `rg -c "Disposition: fixed:" <the log>` yields the count the report states; when that count is zero, `git diff --name-only main...HEAD` contains no path under `crates/`, `docs/`, `examples/` or `spec/`. Artifact-evidence: the report states the count and the enumeration it came from. Rejects decision 9's inverse failure — an invented improvement dressed as a discharged disposition — and rejects a silent empty PR that never says why it is empty |

**Coverage of the traced project AC.** All seven serve `../project.md` AC-006, and specifically the
**fixed-arm half** of it that `../_storymap.md`'s `## Coverage` assigns here: *"landing the fix a
'fixed' disposition asserts, and clearing the gate for it."* The other half — *exactly one*
disposition per stumble — is `disposition-every-stumble`'s and is not claimed by any row above.
AC-005 touches project AC-011's territory only at the seam where a fix must **stop and become an
escalation**; AC-011 itself, and the escalation's substance, is `route-and-escalate`'s sole ownership
(`../_storymap.md`, `## Coverage`).

## Interaction quality

The blocking invariants, in two families. **Every one of them is carried by an `AC-###` row in the
table above** — this section says only which row carries which and how each is verified. Nothing here
is a free-floating bullet: `redkiln verify` extracts ACs from a leading `| AC-### |` table cell or an
`- AC-###:` bullet, so an invariant stated only as prose here would get no ledger row, would never be
gated, and would never be tested.

### State invariants

| Invariant | Carried by | How it is verified here |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1) — the discharge is readable at the stumble; the hop outward to the fixed page is optional and at most one | AC-001 | The `<ref>` sits in the entry's own `Disposition:` field, not in a separate index; a reviewer never leaves the entry to learn that the fix landed |
| **Non-occlusion** (IQ-2, and initiative DoD scenario 13) — a fix must not move load-bearing content behind a fold, a collapsed admonition or an `ignore` fence where the gate can no longer read it | AC-003 | `rg` over the changed files returns no `<details>`, no `<summary>`, no `doc(hidden)`, no `ignore` fence; every example a fix touches is compiled by `cargo test --workspace --all-features` |
| **Preserved position** (IQ-3) — every citation that resolved before this PR still resolves after it: stumble ids and heading lines, intra-doc links, `docs/README.md`'s targets, and `SPECIFICATION.md`'s `file:line` ranges | AC-004, AC-005 | `git diff` over the log touches no `### FL-` heading and no entry order; every range citation into a file this PR edited is re-resolved by hand and repaired in the same commit; `cargo xtask spec-trace` green |
| **Reversibility** (IQ-4) — a disposition that changes is recorded *as a change*, the earlier arm still legible with its reason | AC-005 | A dated line in the entry's `Revisions:` slot; never an in-place edit of the original `Disposition:` text. Authority: `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| **Keyboard reachability** — every fix is reachable and readable with the medium's own affordances (rustdoc search, browser find, intra-doc links); no fix introduces a pointer-only affordance, a widget or a script | AC-003 | Static text completeness over the changed files; the three permitted primitives are all keyboard-reachable by construction, and the fourth thing a fix might reach for is precisely the widget the anti-pattern list forbids |

### Composition invariants

`../_design.md` is signed off (2026-08-17, no conditions) with `hasSurface: false` and an empty items
block. It declares **no rendered surface and no public API item**, and states in its own words that
*"there is no CSS layer and no token file, and inventing one is out of scope"* — so there is no pixel
density budget and no component library. The composition constraints it **does** carry are the named
document and rustdoc primitives, and those bind this story exactly as a component library would. That
is the signed-off position, not a gap.

| Invariant | Real numbers / named source | Carried by |
| --- | --- | --- |
| **Presentation exists at all** — a fix carries real composed presentation in its primitive's own shape, not bare markup. A sentence appended below an existing paragraph is this medium's unstyled render: it satisfies every structural check and teaches the next reader nothing | An `# Errors` section that *names the conditions* rather than the error type (RS-70-2/70-5); a routing row with **both** columns filled; an example that **compiles** rather than an `ignore` fence (RS-62-4) | AC-003 |
| **Composition and placement** — each fix maps to one named existing primitive, none hand-rolled, and lands inside that primitive's existing position rather than at the end of the file | `standards/rust/70-rustdoc-obligations.md:93` (RS-70-2), `:152` (RS-70-3), `:243` (RS-70-5); `standards/rust/62-doctests-and-harnesses.md:12` (RS-62-1), `:75` (RS-62-2), `:189` (RS-62-4); `docs/README.md:12-23`'s two-column routing table | AC-002, AC-003 |
| **Transience** — what is persistent, what is revealed, what is opened on demand | Persistent and visible-by-default: every line a fix adds. Revealed: **nothing**. Opened on demand: **nothing** — no fold, no tab, no collapse, no `doc(hidden)`, no `ignore` fence, by rule. Initiative DoD scenario 13 is the standing check that nothing load-bearing becomes hidden | AC-003 |
| **Density budget, with its numbers** | **3** permitted primitive kinds and no fourth; **1** `<ref>` per fixed arm; **1** disposition arm per stumble (zero and two are both failures); **0** new headings, **0** navigation widgets, **0** `ignore` fences, **0** `pub` items, **0** signature changes, **0** files under `.kb/`, **0** ADRs; **1** dated line per revision. **Tripwire, not a hard fail**: a single fix spanning more than one file is checked against decision 4 before it lands, and the check is recorded in the ledger row — a fix that large is usually an escalation wearing a fix's label | AC-003, AC-005 |
| **Hierarchy** — the fix lands where the reader stumbled and in the reading order the page already has; a `# Errors` section goes in the doc comment's existing section order, a routing row goes in the existing table | The implicated page is the one the stumble's "What happened" field names; `docs/README.md`'s table is the existing shape a routing fix extends rather than replaces | AC-002, AC-003 |
| **Named anti-patterns** — a bespoke navigation widget over a medium that renders the equivalent for free; a load-bearing item behind a fold, an inactive tab or a collapsed admonition; an `ignore`-fenced snippet; `allow(clippy::doc_markdown)` in place of teaching `doc-valid-idents` a proper noun; an intra-doc link that resolves in only some feature configurations; a new page-structure convention invented here rather than routed to HS-P0021 | `../_decomposition.md`, `#### The primitive layer to compose from — do not hand-roll` and UX-AC-012; `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md`, `## Anti-patterns`; `standards/rust/70-rustdoc-obligations.md:93,152` | AC-003 |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | `../_design.md`'s protocol section fixes a different path or heading vocabulary for the friction log than this spec names | `_design.md` **wins verbatim**. Re-resolve the mount point against it before enumerating the work list, and note the discrepancy in the implementation report. The path and heading vocabulary are `_design.md`'s to fix, not this spec's (`../_storymap.md`, `### Why the slices fall here`) |
| EC-002 | `disposition-every-stumble` has not landed, or an entry still reads `not yet dispositioned` | **Stop; do not choose an arm.** The seam is that they choose and this story discharges (decision 1). A fix landed ahead of its disposition inverts the instrument: the log then records what the author already changed rather than what the reader stumbled on |
| EC-003 | A fix would change what a sibling project decided in a signed-off design tension | It is not a fix. Revise the arm to `escalated: DT-<n>` per AC-005, leave the code untouched, and hand the substance to `route-and-escalate`. The test is not "is this edit large" — it is *does the edit change what a sibling decided* (`../project.md`, AC-011 and risk row 4) |
| EC-004 | The fix appears to need a new page, a new page-structure convention or a new navigation widget | Route to HS-P0021 `page-need-discipline`; do not invent it here (UX-AC-012). A new page would also need pinning by path in `xtask/src/`, which is HS-P0020's — and that requirement is the tell that the change has left this story |
| EC-005 | The edit would change the set of implementations a `[FROZEN]` clause admits | It is a **gap**, not a repair. Record it with the clause id, route it, and leave the normative sentence verbatim. A gap is an ADR's, and this story is not authorised to write one — ADR authorship belongs to the runbook's own pass (`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`) |
| EC-006 | This PR's own edits shift a `file:line` range that `spec/SPECIFICATION.md` cites | Repair the citation **in the same commit as the edit that moved it**. Nothing in the gate resolves those ranges: `cargo xtask spec-trace` checks clause↔rule cross-references, not `file:line` citations (`xtask/src/spec_trace.rs`, `# What it does not do`) |
| EC-007 | The work list is empty — no arm reads `fixed:` | Not an error. Land no content change, record the count, and say so (AC-007). Manufacturing a fix is the failure; an empty diff is not |
| EC-008 | A change appears to need a path outside `## PR boundary`'s fenced block | The change has left this story. **Route it rather than widening the block** — `redkiln verify --grain story` reads that block and fails on any file changed outside it. `xtask/**`, `standards/**`, `.kb/**` and `../_design.md` are absent on purpose |
| EC-009 | The stumble implicates a page a sibling project owns, and the fix is larger than "small" | Small and tension-free lands here, citing the sibling's material; anything larger routes with the sibling's id through `route-and-escalate` (`../project.md`, `## Risks and coupling notes`, **Coupling notes**). File ownership is not the test — tension ownership is |
| EC-010 | The stumble is a bug in the library rather than in the documentation | It routes to the `support` initiative (`.redkiln/config.yaml:5`) as a disposition, and that is `route-and-escalate`'s. A library fix landed here is out of scope even when it is one line (`../project.md`, `## Out of scope`, last bullet) |
| EC-011 | `redkiln doctor` reports a seventh `template-drift` advisory, or fewer than six | A template was changed without a decision, or a customisation was reverted. Investigate; **never run `redkiln adopt --templates`**, which would overwrite all six customisations silently and then fail CI on the absence it created (`CLAUDE.md`) |
| EC-012 | A fix's doctest passes on the host but breaks one of the four wasm32 steps | The fix is not done. `cargo xtask ci --fast` keeps all four wasm32 steps, and the `happenstance-core` build there is the standing guard on ADR-0001. Re-shape the example (RS-62-2's hidden runtime is the existing pattern) rather than gating it away |

## Non-functional

| id | Requirement | Why it is here |
| --- | --- | --- |
| NF-001 | This story adds no gate step, no CI job, no tool, no dependency and no generated file | The proof tiers are the ordinary ones run in the ordinary order (decision 8). A new tier here would be a check nothing can fail, which `CLAUDE.md` calls decorative |
| NF-002 | Every fix, and every `<ref>`, is complete and legible as raw text in a `git diff` at ordinary terminal width, with no rendering step | `../_decomposition.md`'s accessibility floor: a screen reader and a `git diff` must both read the record. This is the reduced-motion floor stated for a medium with no motion |
| NF-003 | Each `<ref>` is a stable citation in the `file:line` shape `.redkiln/templates/_ledger.md`'s `evidence` rows already use — a path plus an anchor or a line, never a description | IQ-7 applied to the fixed arm: a destination that is a description is not a destination. It is also what makes the ledger's evidence and the arm's `<ref>` the same fact rather than two |
| NF-004 | One fix is one revertible unit: a commit (or a hunk boundary) that maps to exactly one `FL-###` | A fix that later proves wrong is revised, not unpicked from a mixed commit. It is also what makes AC-001's bidirectional check mechanical instead of a reading exercise |
| NF-005 | The three publishable crates still carry both licence files and a README after this PR | `cargo xtask ci --fast` asserts this via `cargo package --list`. A docs-shaped fix that moves or renames a file is the one plausible way this story breaks it |
| NF-006 | No MSRV move, no feature-gate change, no `pub` item, no signature change, no visibility change | `../_design.md` declares no items. An edit that alters a signature has stopped being a content fix and needs a different sign-off; an MSRV move is an ADR's (ADR-0029 amending ADR-0004), never a side effect |
| NF-007 | The diff is reviewable in one sitting, and its size is proportionate to the work list | The PR boundary is deliberately narrow. A large diff against a short work list is the signal that unevidenced content came in beside the evidenced fixes (decision 2) |

## Implementation notes (non-prescriptive)

Shape only — the implementer owns the wording and the fixes themselves.

- **Enumerate before you edit.** Read the log end to end and write the work list down first:
  `FL-###` → the page its "What happened" field names → what the fix would be. Opening a source file
  before that list exists is how the convenient page gets fixed instead of the implicated one.
- **Do the triage pass before any code.** For each candidate, ask decision 4's question (does this
  change what a sibling decided?) and decision 5's (does this change what the document asserts?).
  Both answers are cheaper before the edit than after it, and both have a defined exit that is not an
  edit here.
- **One commit per fix, keyed to the stumble id.** It makes NF-004 free, makes AC-001's correspondence
  check a `git log` read, and makes a wrong fix revertible without touching the right ones.
- **Fill the `<ref>` last, and re-check it after any rename.** The reference is the mount; a `<ref>`
  written against a path that later moved is exactly the "reads as mounted" failure AC-001 rejects.
- **Run the narrow tier first.** `cargo test -p <crate> --doc` on the crate a fix touched is seconds;
  `cargo xtask ci --fast` is not. Use the cheap one to iterate and the expensive one to finish.
- **Re-resolve the specification's citations deliberately, not hopefully.**
  `rg -n "src/[a-z_]+\.rs:[0-9]+-[0-9]+" spec/SPECIFICATION.md` lists them; only a human comparison
  against the post-fix file closes it, because no gate step does.
- **Write the empty case as prose, not as silence.** If the work list is empty, the report says so and
  states the count and where it came from. A PR that is empty and unexplained is indistinguishable
  from a PR that was never done.
- **Do not write a disposition checker.** Named as forbidden by the testing brief for a stated reason:
  a script confirming a `Disposition:` line exists passes `noted` as readily as `routed: HS-P0022, see
  FL-004`. Any check written here asserts on content — a resolving `<ref>`, an existing DT id.
- **When a fix starts to feel like an argument, stop.** That is decision 4 firing. The arm becomes an
  escalation with its DT id and the argument belongs to the project that owns the tension.

## Tests and CI (merge gate)

Grounded in `../_decomposition.md`'s `## Testing brief` — its `**Content-fix tier, orthogonal to the
table above**` block (doctest / static / integration, named by command), its Artifact-evidence
definition (ledger-cited `file:line`, not eyeballing), and its explicit instruction not to automate a
structural presence check into something nothing can fail.

| tier | command / path | proves |
| --- | --- | --- |
| Static — work list | `rg -n "Disposition: fixed:" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` | The input set, enumerated rather than remembered; its count is what AC-007 records when it is zero (AC-001, AC-007) |
| Static — `<ref>` resolution | `test -f` on each `<ref>`'s path, plus `rg` for its anchor or line | Every fixed arm points at something that exists — the difference between mounted and reading-as-mounted (AC-001) |
| Static — correspondence | `git diff --name-only main...HEAD` restricted to `crates/`, `docs/`, `examples/`, `spec/`, mapped against the work list in both directions | No orphan hunk and no undischarged arm; the correspondence runs both ways or one side is lying (AC-001, AC-002) |
| Static — forbidden constructs | `rg -n '```ignore'`, `rg -n "allow\(clippy::doc_markdown\)"`, `rg -n "<details>\|<summary>\|doc\(hidden\)"` over the changed files, each returning nothing | The composition invariants and the named anti-patterns, checkable rather than trusted (AC-003) |
| Static — DT id existence | `rg "^\| DT-" .bklg/docs-that-teach/_decomposition.md` | An escalation citing a DT id that does not exist is caught mechanically (AC-005) |
| Static — log diff shape | `git diff -- .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` shows hunks only inside `Disposition:` and `Revisions:` fields | Ids, heading lines and narrative are untouched; every citation a sibling has already made still resolves (AC-005) |
| Static — citation repair | `rg -n "src/[a-z_]+\.rs:[0-9]+-[0-9]+" spec/SPECIFICATION.md`, each range re-resolved by hand against the post-fix file | The one obligation the gate cannot discharge for itself (`xtask/src/spec_trace.rs`, `# What it does not do`) (AC-004) |
| Doctest — the unit tier | `cargo test --workspace --all-features` | Every doc-comment example a fix touched compiles and runs; this project has no functions, so doctests stand in for a unit tier (AC-003, AC-006) |
| Static tripwire | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | A doc-comment lint or a broken clause↔rule cross-reference is caught before the full gate runs (AC-004, AC-006) |
| Story gate | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The diff's packages plus dependents get fmt, clippy `-D warnings` and tests; a diff touching nothing under `crates/` still gets the five file-reading lints and `spec-trace` rather than a vacuous pass (AC-006) |
| Integration — this project's merge gate | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | DoD-7 verbatim: fmt, clippy, tests, all four wasm32 steps, docs, `spec-trace`, the `--no-default-features` doc build and the `cargo package --list` assertion (AC-006, NF-005, EC-012) |
| Artifact-evidence | `.bklg/docs-that-teach/comprehension-evidence/content-fixes-from-dispositions/_ledger.md` — one row per AC, each citing a real `file:line` | The claims a reviewer must read: the stumble→page→hunk trace, the primitive each fix composes from, the repair-or-gap judgement. `require_ledger: true` at `.redkiln/config.yaml:67` |
| Story verify | `redkiln verify --grain story` | The PR-boundary path block holds, and every ledger row is satisfied with non-placeholder evidence before `implement → report`. `require_commit_provenance: true` at `:73` also requires the work commit to be recorded |
| Backlog hygiene | `redkiln validate --kb && redkiln doctor` | Clean, at exactly the six standing `template-drift` advisories `CLAUDE.md` documents (`../project.md`, DoD-8) (AC-006) |

**Not run here, deliberately.** `cargo xtask ci` — the terminal `e2e` bar at `.redkiln/config.yaml:60`
— is HS-P0025's, and this project is `terminal: false`. And the friction-log *session* is never folded
into a CI step: a passing `cargo xtask ci --fast` says nothing whatever about comprehension, and the
two instruments falsify different things (`../_decomposition.md`, `## Testing brief`, the E2E tier
bullet).

## Risks and coupling (PR-scoped)

| Risk | Likelihood / Impact | Mitigation |
| --- | --- | --- |
| Unevidenced content improvements ride along beside the evidenced fixes, because the author is already in the file | **High** / Medium | AC-001's correspondence check runs in *both* directions and NF-007 makes a disproportionate diff a review signal. The four sibling projects own unevidenced content; a hunk with no `FL-###` behind it fails the criterion, not just the taste test |
| A fix quietly re-decides a settled tension because it looked like a wording change | Medium / **High** | AC-005 and EC-003 make it an escalation with a real DT id, checked mechanically against `.bklg/docs-that-teach/_decomposition.md:150-159`. The test is stated as *does the edit change what a sibling decided*, not *is the edit large* (`../project.md`, risk row 4) |
| A doc comment discharging a `[FROZEN]` clause is rewritten into something that admits a different set of implementations | Low / **High** | AC-004 runs the playbook's repair-or-gap test before the edit; a gap is recorded and routed, never decided. `../project.md`'s risk row 8 rates this exactly here |
| A doc-comment insertion shifts a `SPECIFICATION.md` line range and nothing notices | Medium / Medium | EC-006 puts the repair in the same commit and the Tests table makes the enumeration an `rg` rather than a memory. `spec-trace` explicitly does not resolve `file:line`, so this is a hand check by design, stated rather than assumed |
| The work list is empty and the story is padded to look delivered | Medium / **High** | AC-007 makes zero a first-class recorded outcome with a stated count; decision 9 says so in the context pack. `discover.md`'s wrong implementation is the mirror of it |
| The fix lands in the page that was easiest to change | Medium / Medium | AC-002 requires the three-column trace per fix, checked against the stumble's own "What happened" field. This is `discover.md`'s named wrong implementation, promoted to a criterion rather than left as advice |
| A fix reaches for a new convention — a widget, a new heading, an `ignore` fence — because the primitive layer feels short | Medium / Medium | AC-003's static checks return nothing or the criterion fails; UX-AC-012 routes the need to HS-P0021 rather than allowing an invention. RS-62-4 and RS-70-2 name the existing alternatives so the reach has somewhere to go |
| This story lands late and blocks the hand-off HS-P0025 needs | Low / Medium | It cannot: `handoff-note-to-closeout` has no `depends_on` edge to this story, deliberately (`../_storymap.md`, `### Dependency graph`). A fix landing after the hand-off is recorded as a revision, not a rewrite of the note (IQ-4) |
| Backwards coupling — this PR edits pages that sibling projects already merged and consider finished | Medium / Medium | That is the point (*"a friction log is worthless if it cannot cause a change"*), bounded by AC-005 and EC-009: small and tension-free lands here citing the sibling's material; anything else routes with the sibling's id |
| Wide `crates/**` globs in the PR boundary admit an edit that is not a doc comment | Medium / Medium | Git cannot express "doc comments only". Decision 3, the `## Behavior and interfaces` rows and NF-006 carry that half of the boundary, and AC-003 checks it — no `pub` item, no signature, no visibility change |

## Dependencies

**Blocks on**

- **`disposition-every-stumble`** (`blocked_by: HS-S0166`) — the only edge, and it is hard. It gives
  every severity-marked stumble exactly one arm; this story discharges the arms that read `fixed:`.
  There is literally nothing to do until it lands, and a fix landed ahead of its disposition inverts
  the instrument (EC-002). Both are in the `dispositions-and-routing` slice and are implemented in one
  context, sequentially, in that order (`../_storymap.md`, `## Merge order`, step 3).

**Slice-mate, not a dependency**

- **`route-and-escalate`** — independent of this story and may land in either order relative to it
  (`../_storymap.md`, `## Merge order`, step 3). The seam is one-directional and narrow: where a fix
  attempted here turns out to be an escalation (AC-005), the *substance* of that escalation —
  submission to a named owner, the destination id, AC-011's judgement — is `route-and-escalate`'s.
  This story writes the revised arm and stops.

**Unlocks**

- **Nothing, by design.** `handoff-note-to-closeout` deliberately carries no `depends_on` edge to this
  story: *"a fix that lands late must not be able to hold up the evidence HS-P0025 needs"*
  (`../_storymap.md`, `### Dependency graph`, closing paragraph). What this story does unlock is
  non-structural — the initiative's DoD scenario 6 becomes true rather than merely complete, because a
  `fixed:` arm is a claim about the repository until this story makes it a fact.

## Anchors (progressive disclosure)

Deferred depth, not optional depth. Open each at the moment its row names; do not preload the corpus.
Every path below was confirmed present in this worktree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/comprehension-evidence/_decomposition.md` | The UX brief holds **UX-AC-012** verbatim — this story's tightest constraint — plus `#### The primitive layer to compose from — do not hand-roll`, IQ-1/IQ-3/IQ-4/IQ-7 and the accessibility floor. The Testing brief holds `**Content-fix tier, orthogonal to the table above**`, which names the four tiers by command, and the do-not-over-automate rule | **First, before the triage pass** — UX-AC-012 decides what a fix is allowed to be, and reading it after the edit is reading it too late | AC-003, AC-005, AC-006 |
| `.bklg/docs-that-teach/comprehension-evidence/friction-log-skeleton/spec.md` | The log does not exist yet when this spec is read; this is where its shape is fixed — the five disposition arms, the six one-line entry fields, the `Revisions:` slot defaulting to `none` from first write, the `FL-###` id and label freeze | Before enumerating the work list, and again before writing any revision line | AC-001, AC-005 |
| `.bklg/docs-that-teach/comprehension-evidence/_design.md` | The signed-off design (2026-08-17, no conditions), `hasSurface: false` with an empty items block. Binding here as a **prohibition** — no `pub` item, no invented format — and as the authority on the log's path, which wins verbatim over any path this spec names | Before the first write, to confirm the mount path; again if a fix starts to look like an API change | AC-001, AC-003 |
| `standards/rust/70-rustdoc-obligations.md` | The doc-comment primitive layer: RS-70-2 at line 93 (never an intra-doc link that resolves in only some feature configurations — exactly the link a hasty fix reaches for), RS-70-3 at 152 (`doc-valid-idents`, never `allow(clippy::doc_markdown)`), RS-70-5 at 243 (name the alternative that lost, once) | Immediately before writing any doc-comment fix | AC-003 |
| `standards/rust/62-doctests-and-harnesses.md` | The compiled-example primitive: RS-62-1 at line 12 (never trust a lone `compile_fail` fence's error code), RS-62-2 at 75 (the hidden runtime for an `.await`ing example), RS-62-4 at 189 (the discarding macro in place of an `ignore` fence) | Before adding or repairing any example, and the moment an `ignore` fence feels tempting | AC-003 |
| `docs/README.md` | Lines 12–23 are the live two-column routing table a routing fix extends rather than replaces; the paragraph immediately below states that two of its targets are read by the gate **by path**, which is why a fix never silently moves a tree | Before any fix that touches `docs/README.md` | AC-002, AC-003 |
| `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | The mechanical test — *a correction is a repair if the set of implementations it admits is unchanged, and otherwise it is a gap*. It is the difference between an edit this story may make and one it must record and route | Before editing any doc comment in `crates/happenstance-core/src/` | AC-004 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The one KB atom binding on this work, at two seams: the doc-comment edit (the referent may be rewritten, the reasoning may not) and the revised disposition (append, never overwrite) | Beside the playbook above, and again before writing a `Revisions:` line | AC-004, AC-005 |
| `spec/SPECIFICATION.md` | Line 371 cites `crates/happenstance-core/src/store.rs:93-268` — the concrete instance of the hazard: a doc-comment insertion above line 93 shifts that range silently | After any edit to a file the specification cites, before committing | AC-004 |
| `xtask/src/spec_trace.rs` | Its module docs' `# What it does not do` section is the evidence that the gate does **not** resolve `file:line` citations. Without reading it, the hand check in AC-004 looks like belt-and-braces rather than the only thing standing there | Once, when deciding whether the citation repair can be delegated to the gate — it cannot | AC-004 |
| `.bklg/docs-that-teach/_decomposition.md` | `## Design tension ownership` (lines 150–159) is the DT-id vocabulary an escalation must cite; `### Definition of Done` row 6 is the initiative scenario this story makes true | Before writing any `escalated: DT-<n>` arm | AC-005 |
| `.bklg/docs-that-teach/comprehension-evidence/project.md` | AC-006 is the traced criterion; AC-011 is the guard rail; `## In scope`'s small-content-fixes bullet is the narrowing; risk rows 4 and 8 are the two high-impact failures this spec's ACs answer; DoD-7 and DoD-8 are the gate bars | Before the triage pass, and again when writing the ledger's `verifying_test` values | AC-004, AC-005, AC-006 |
| `.bklg/docs-that-teach/comprehension-evidence/_storymap.md` | `## Coverage` states the AC-006 split that bounds this story to the **fixed** arm; `## Merge order` step 3 fixes the ordering; `### Dependency graph`'s closing paragraph is why the hand-off is not gated on this work | Before starting, to confirm the boundary; again if the work starts to feel like it should choose an arm | AC-001, AC-007 |
| `.redkiln/config.yaml` | Line 5 `support_initiative: support` (the routed library-bug destination), line 40 `affected_gate`, line 48 `reachability_static`, line 55 `integration_scoped`, line 60 `e2e` (not this project's), line 67 `require_ledger`, line 73 `require_commit_provenance` | When running the gates and when authoring the ledger | AC-006 |
| `xtask/src/main.rs` | The single definition of what `cargo xtask ci --fast` actually runs — the four wasm32 steps, the `--no-default-features` doc build and the `cargo package --list` assertion. It is what NF-005 and EC-012 are about | If a gate step fails and the failure is not obviously the fix's | AC-006 |
| `.redkiln/templates/_ledger.md` | The `evidence: "file:line"` contract — the shape a `<ref>` takes and the reason a stable `FL-###` anchor is worth citing at all | When authoring this story's ledger, and when deciding the `<ref>` format | AC-001 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | `## Anti-patterns` names the bespoke navigation widget over a medium that renders the equivalent for free, and the load-bearing item behind a fold — the two failures a fix most plausibly commits | Before adding any navigational affordance; the answer is almost always a link, not a component | AC-003 |
| `.bklg/docs-that-teach/comprehension-evidence/_grounding.md` | `## Existing code/precedent patterns worth citing, with a caution` (lines 184–198) verifies `examples/course-subscriptions/` as the one real worked example in this worktree and **fails** `examples/outside-projection-adapter/` as unreachable — do not cite the latter in a fix | Before any fix touching `examples/`, and before citing precedent anywhere | AC-002 |
| `examples/course-subscriptions/src/main.rs` | The canonical DCB worked example and one of the three pages a fix may land in; it is also the artefact the application author's journey stalls on today | Only if a stumble implicates the worked example | AC-002, AC-003 |

## Clarifications resolved during spec

1. **The AC ids are exactly the seven the front half enumerated** — AC-001 … AC-007 — with none added
   and none dropped. The mapping is: AC-001 the bidirectional correspondence and the resolving
   `<ref>`; AC-002 the implicated page rather than the convenient one; AC-003 composition from the
   three permitted primitives and the anti-patterns; AC-004 referent-not-reasoning plus the line-range
   citation repair; AC-005 escalation-instead-of-fix, recorded as an appending revision; AC-006 the
   ordinary gates run in the ordinary order; AC-007 the empty work list as a complete outcome.

2. **Composition invariants are stated against document and rustdoc primitives, not a design system,
   and that is the signed-off position rather than a gap.** `../_design.md` records `hasSurface:
   false` with an empty items block and every shape section `N/A`, and states that there is no CSS
   layer and no token file and that inventing one is out of scope. The density budget is therefore in
   counts of primitives, arms, headings and forbidden constructs rather than in pixels — and it is
   still blocking, because zero `ignore` fences is as checkable as a contrast ratio.

3. **This story renders no `_design.md` item id, because `_design.md` declares none.** It *changes*
   surface 1 of the three document surfaces the UX brief enumerates (the material the reader walks)
   and *writes into* surface 2 (the friction log). It must not touch surface 3 (`_design.md` itself) —
   editing the protocol after the session is the retrofit the project's AC-002 provenance check exists
   to catch, which is why `../_design.md` is deliberately absent from the PR boundary's path block.

4. **The `<ref>` format is a `file:line`-shaped citation, matching the ledger's `evidence` contract**
   (`.redkiln/templates/_ledger.md`), and once the work commit exists it may additionally name the
   commit. It is fixed here so AC-001's resolution check is a `test -f` plus an `rg` rather than a
   judgement call; NF-003 forbids a prose destination, which is IQ-7 applied to the fixed arm. If
   `../_design.md`'s protocol section or `friction-log-skeleton`'s landed scaffold fixes a different
   spelling, that one wins verbatim (EC-001).

5. **No verifying test is a compiled test, and that is honest rather than a shortfall.** This story
   adds no `pub` item and no function, so its criteria are verified by the Static, Doctest and
   Artifact-evidence tiers the project's own testing brief defines, plus the two gate commands. The
   brief's warning is respected: every static check above names the wrong implementation it rejects,
   and the one check that could not reject a wrong implementation — "a `Disposition:` line exists" —
   is deliberately not written, in this story or anywhere else.

6. **AC-005 does not claim project AC-011.** It carries the *stop condition* — a fix that would reopen
   a settled tension is not landed, and the arm is revised to an escalation with a real DT id.
   AC-011's own judgement, the submission to a named owner and the escalation's substance are
   `route-and-escalate`'s sole ownership (`../_storymap.md`, `## Coverage`). Both stories are in the
   same slice, so the seam is a hand-off inside one context rather than across a merge.

7. **The density budget's one-file tripwire is a review gate, not a hard fail.** A fix spanning more
   than one file is not forbidden — a stumble may genuinely implicate a doc comment and the routing
   row that points at it — but it is the shape an escalation takes when it is wearing a fix's label,
   so decision 4's question is asked explicitly and the answer is recorded in the ledger row. A hard
   numeric limit would have been arbitrary; an explicit recorded check is not.

8. **`examples/outside-projection-adapter/` is not citable and is not a fix target.** `_grounding.md`
   verified directly that it does not exist in this worktree, and `../initiative.md` flags it as
   "asserted and unverifiable from this worktree". If a stumble appears to implicate it, the stumble
   is about a broken citation elsewhere, and that citation is the thing to fix.
