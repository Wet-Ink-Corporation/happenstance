---
item: HS-S0112
stage: spec
created: 2026-08-12T13:47:53.365Z
updated: 2026-08-12T13:47:53.365Z
template_sig: 87bbf1d0
rendered_sig: e134d9a0
---

# Spec — Every clause still rejects something that exists

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 12 (*the clause ledger is audited*), DoD 13 (*the gate is green on the assembled whole*), DoD 14 (*replication has an answer on disk*) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project (charter) | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` — AC-002 (this story's share: *no frozen clause edited*), AC-012 (*the corrections go through decision records, not edits*); DoD 7 |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/frozen-clause-repairs/spec.md` |
| Key briefs | `…/replication-identity-and-ingest/_decomposition.md` — *architecture: Tension 4* (the audit is a pre-commitment, not a hunt; the `(new)` markers are a repair and DoD 7 must not be read as forbidding them), *Tension 3* (the re-derived ledger this story's sibling consumes), **AC-A11** (every clause whose `Rejects:` names a symbol this project changed has a repair **in the same commit**); *testing: The test mix* (Static tier — `spec-trace` is AC-012's instrument) |
| Story map row | `…/replication-identity-and-ingest/_storymap.md`, *Slices*, `spec-repairs-and-clause-exit` row 1; *Backbone* A7 (*Leave the record true*); *Merge order* 7 — *"Last on purpose: a repair can only name the rules and symbols that exist"* |
| Signed-off design | `…/replication-identity-and-ingest/_design.md` — **no user-facing surface**, approved by the repository owner 2026-08-12, `design.capture` a declared skip. There is no surface id to render, and this story adds no Rust item at all |
| Discover stage (this story) | `…/frozen-clause-repairs/discover.md` — the signal ledger, the sweep already re-run at HEAD, the audit list **by id**, and *The wrong implementation*'s four named mutants |
| Slice-mate spec | `…/clause-arithmetic-and-deferral-renewals/spec.md` (HS-S0113 — the exit computation; it depends on this story and owns every maturity marker this one must not touch) |
| Playbooks that bind | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` (the mechanical test and the three-part form); `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| Roadmap pointer | `RUNBOOK.md:4516-4620` (phase 13); the record-keeping obligation is `RUNBOOK.md:334-336`'s standing lesson from phase 4 |

**One naming caveat, because it costs an implementer a search.** This story's item id is
**HS-S0112** (frontmatter above). Three sibling specs refer to it by that id
(`…/headline-rules-and-mutant-registry/spec.md`, `…/adr-0027-…/spec.md`,
`…/ingest-store-and-memory-peer-round-trip/spec.md`) and three refer to the same story, by slug, as
**HS-S0114** (`…/gate-mounts-for-the-sync-suite/spec.md`, `…/adr-0003-provisional-lift/spec.md`,
`…/byte-identical-round-trip-and-idempotent-replay/spec.md`). The slug `frozen-clause-repairs` is the
reliable key. Grep for **both** ids when collecting the findings the earlier slices handed on; a
handoff missed because it was filed under the other number is a repair that never happens.

## One-line PR slice

Re-run the `re-check|recheck|re-evaluat` sweep and record the finding either way, drop the `(new)`
markers from the `Rule:` lines whose rules now exist, and repair — in the playbook's three-part form,
MUST verbatim — every clause whose `Rejects:` names a symbol this project changed, each repair
authorised by ADR-0026 and justified by the mechanical test that the admitted implementation set is
unchanged.

## Executive summary

**This PR is where the specification stops describing a codebase that no longer exists.**

Six slices have run ahead of this one. They made `impl IngestStore for MemoryEventStore` truthful,
landed the three rules that SY-1, SY-2 and SY-6 have scheduled with `(new)` since phase 2, rewrote
the module prose that SY-6, SY-9 and SY-10 cite by line, and — if ADR-0027 went that way — may have
taken `EventGroup::guard` out of the public surface that SY-1 and SY-6 name as the reason
re-evaluation is reachable at all. Every one of those is a *win*, and every one of them silently
degrades a `[FROZEN]` clause into a sentence that forbids something nobody can do any more.

The delta this PR lands is small in bytes and is the whole of project AC-012. It is not a hunt: the
audit list is a pre-commitment written down at the discover stage and at `_decomposition.md`'s
Tension 4, and the ingest sweep has already been run once at HEAD with a **null** result, which this
story re-runs and records rather than assumes. What makes it hard is that the three easy ways to do
it are all wrong, and all three are already named in `…/frozen-clause-repairs/discover.md`, *The
wrong implementation*: do nothing and stay green; delete the `Rejects:` field on the grounds that the
obligation is met; or soften the MUST so the clause agrees with the code. The playbook's answer is
one shape — **keep the MUST verbatim, name the discharge as a discharge, cite the code and the test
that now assert it** (`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, *The safe
form for a discharged MUST*).

**Delta against `HEAD` that no brief carries, and that changes what this story must do.** Dropping
`(new)` from an `SY` clause's `Rule:` line does **nothing checkable** today, and would not after
`gate-mounts-for-the-sync-suite` either. `schedules_new` has exactly one consumer —
`xtask/src/spec_trace.rs:695` — and that line reads `if c.schedules_new || !has_suite(&c.id)`, while
`has_suite` accepts `ES-`, `VT-` and `WF-` and nothing else (`:1735-1737`). Every `SY` clause is
skipped by check 4 whether or not it carries a marker. HS-S0104 widens `RULE_FILES` and repairs check
4's *message*, and grepping its spec for `has_suite` returns nothing — so the predicate is
unclaimed, and a marker drop that lands without it is a repair that cannot fail, which is this
repository's most familiar failure mode wearing documentation clothes. Teaching `has_suite` about
`SY-` is therefore part of this story, and it is what converts every remaining `(new)` on an
unwritten rule from a comment into a ratchet.

## Context pack

The decisions this story must honour, stated as decisions. Everything deeper sits behind a signposted
anchor; nothing below needs another file to be actionable.

**1. The freeze protects the normative sentence and nothing else — and that is a licence with a
mechanical boundary, not a mood.** `.kb/decisions/README.md` states the test in the corpus's own
voice and the playbook restates it: *a correction to a `[FROZEN]` clause is a repair if the set of
implementations the clause admits is unchanged; otherwise it is a gap, and a gap is a decision's.*
The operational form is a yes/no question an implementer with no authority to decide anything can
answer honestly: **is there an implementation that was conformant before the edit and is not after,
or vice versa?** No → edit freely. Yes, **or you cannot tell** → you have found a gap, and the
correct output is a recorded finding and a re-plan, *not a smaller edit*. "Cannot tell" is on the gap
side deliberately.

**2. Every repair takes one of two shapes, and there is no third.** For an obligation that has been
met, the three-part form: the MUST stays **verbatim** (which is what makes it a repair), the
discharge is **named as a discharge** so a reader cannot mistake a satisfied obligation for a relaxed
one, and the **evidence is cited** — the code and the test — so the discharge is guarded rather than
a claim about a moment in time. *A discharge recorded without a named test is a discharge that can
silently regress.* For a stale referent — a citation at the wrong line, prose describing a superseded
implementation, a rule that now exists under a different name — the referent is rewritten and the
reasoning is untouched (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`: ask whether the
edit changes what the document **asserts**, not whether it changes the document).

**3. The authorisation exists already and must be cited, not re-derived.** ADR-0026 pre-authorises
exactly two families of repair and names this story as the performer:
the `(new)`-marker drop on SY-1 and SY-2, and the `Rejects:`-symbol repairs on SY-1, SY-6 and SY-12
(`…/adr-0026-peer-ingest-and-transport/spec.md`, *Integration contract → Clause(s)*, and its AC-011).
That is what makes project DoD 7 — *"`git diff` over `spec/SPECIFICATION.md` shows only additions a
decision record authorises"* — satisfiable by pointing at an authorisation rather than by winning an
argument after the fact. **If a needed repair falls outside what ADR-0026 authorised, it is not
smuggled in under the same heading**: it is either obviously within the same mechanical test and
recorded as such with its own justification, or it is a gap and the story stops.

**4. Dropping a `(new)` marker is a repair *and*, today, an unobservable one — so this story mounts
the check that makes it observable.** The marker mechanism is `xtask/src/spec_trace.rs:1626-1634`: a
`Rule:` line containing `(new)`, `†`, a leading `new `, `" new \`"`, or the words *unit test /
compile test / meta-test* marks the clause `schedules_new`, and check 4 skips it. Check 4 also skips
any clause `has_suite` does not recognise, and `has_suite` is a three-prefix `starts_with`
(`:1735-1737`) written when neither the projection nor the replication suite existed — its own
comment at `:684-688` says so. Consequences this story is bound by:

- After HS-S0104, the sync rules file is in `RULE_FILES` (`:85-89`) and therefore in `resolvable`,
  but no `SY` clause is ever consulted against it. **`has_suite` learns `SY-`, in this story**, and
  `PS-` deliberately does not — the projection suite is `projection-store-freeze`'s (HS-P0010) and
  teaching the predicate about a crate that does not exist reports every `PS` rule as missing, which
  is the exact noise the original comment refuses.
- With that in place the marker becomes a **ratchet in both directions**: dropping `(new)` from a
  clause whose rule exists lets the name be resolved, and dropping it from a clause whose rule does
  **not** exist is a hard `spec-trace` failure naming the clause and the rule. A misspelt rule name
  in a de-marked clause fails by name. That is the negative control this story owes, and without the
  predicate change there is no wrong version of this PR that the gate can reject.
- The drop is computed **from the tree**, never from a list in this spec: every `SY`/`WF` clause
  whose `Rule:` names rules that all now resolve loses its marker; every one that does not keeps it.
  Slices 3–6 landed rules this spec cannot enumerate in advance without going stale, which is why
  `_storymap.md` words the deliverable as *"the `Rule:` lines whose rules now exist"*.

**5. `WF` is already checked and `SY` is not, and the asymmetry changes what care each needs.**
`WF-` is in `has_suite` today, so a `WF` clause's `(new)` drop already bites, resolved against
`WIRE_TESTS` rather than the suite. `WF` marker drops therefore need no new mechanism and get no
grace: if a `WF` rule name does not resolve the gate is already red. `SY` marker drops need the
predicate change of §4 first. Do the predicate first, then the drops, so that every drop is made
against a check that is live.

**6. The audit is by id, pre-committed, and each entry ends in one of three verdicts.** The list is
`…/frozen-clause-repairs/discover.md`, *Questions* 3, and it is closed rather than illustrative:
**SY-1** and **SY-6** (the public `guard: Option<AppendCondition>` field on `EventGroup`,
`spec/SPECIFICATION.md:5856-5867`, `:5998-6006`, cited to
`crates/happenstance-sync/src/peer.rs:236-243`); **SY-12** (the `impl IngestStore for
MemoryEventStore` target at `spec/SPECIFICATION.md:6173-6183`, cited to
`crates/happenstance-sync/src/ingest.rs:38-50`); **SY-1** and **SY-2**'s `(new)` markers plus every
other `SY`/`WF` clause whose scheduled rule this project landed; **SY-9** and **SY-10**'s citations
into the module documentation (`spec/SPECIFICATION.md:6120-6121` →
`crates/happenstance-sync/src/lib.rs:57-63`); **SY-15** and **SY-17**'s neighbourhood, where §5's own
preamble names the four test files by name (`spec/SPECIFICATION.md:5775`) including the
`real_peer_shapes.rs` stand-ins the architecture brief says are *superseded while their finding is
not*; and **SY-6**'s citation of `crates/happenstance-sync/src/lib.rs:100-104`. Each id gets a
recorded verdict — **keeps a live target** / **repaired** / **gap** — and *no id is silently
skipped*. A clause visited and found still true is a result, and it is the result this story expects
for most of the list.

**7. SY-12 is the sharpest entry, because it has already survived this once and says so in its own
text.** Its original exemplar was the sync crate's own withdrawn proposal; the clause was
**re-pointed rather than retired**, onto the fact that `impl IngestStore for MemoryEventStore` cannot
be written truthfully. `memory-store-ingest-seam` (HS-S0101) and
`ingest-store-and-memory-peer-round-trip` (HS-S0102) remove exactly that target. So SY-12 needs
either a new live wrong implementation or a recorded discharge in the three-part form — and it must
not get the *cheap* repair, a generic restatement like *"rejects a peer that carries identity
somewhere unreachable"*, which is CF-4's saboteur problem one level up: *"the mutant that earns its
place is the one someone would ship"* (`spec/SPECIFICATION.md:7208-7215`). Note that SY-12's `Rule:`
also carries `(new)` for `dedupe_reaches_identity_without_decoding`, which HS-S0102 named as a
future rule and may or may not have landed by slice 7 — the marker's disposition is §4's computation,
independent of the `Rejects:` verdict.

**8. The ingest sweep is a *recorded* null finding, not an assumed one.** `re-check|recheck|
re-evaluat` over `spec/SPECIFICATION.md` returned ten hits at HEAD on 2026-08-12, five of them about
ingest — `:4373` (ES-38's `Rejects:`, which *presupposes* unconditional ingest and calls the vacuous
pass *"the **normal** path"*), `:5825` (the Cold Chain paragraph distinguishing *"a domain decision
about facts already accepted, running after the ingest, not a gate in front of it"*), `:5862`,
`:5998` and `:6014` (SY-1's and SY-6's own `Rejects:`) — and all five read as consistent with SY-1
and SY-6 as frozen. The charter's phrase *"the clauses framing ingest as re-checking conditions"*
resolves, on present evidence, to **no clause**. Re-run it at implementation time and write the
result down either way, including the hit list and the line numbers, because *"a null finding
recorded is worth more than a null finding assumed"* (`_decomposition.md`, Tension 4) and because
six slices of code have landed since the sweep was last run.

**9. Maturity markers are out of bounds, and so is arithmetic.** `[FROZEN]` / `[PROVISIONAL]` /
`[DEFERRED]` movement, deferral renewals against named experiments (CF-38,
`spec/SPECIFICATION.md:213-217`) and the clause-range union are **HS-S0113**'s
(`clause-arithmetic-and-deferral-renewals`), which declares this story as its predecessor. This story
touches `Rule:` fields, `Rejects:` fields and citations. A repair that would require a marker to move
is, by construction, one where the set of admitted implementations changed — a gap, and the stop
condition of §1.

**10. Re-anchoring a citation without reading the new referent is the one defect no tool can see.**
SY-6 cites `crates/happenstance-sync/src/lib.rs:100-104` for *"a position is meaningful only inside
the store that assigned it"*, and this project rewrites that module documentation wholesale.
Re-pointing at whatever now occupies those lines yields a clause citing a real file at a real line
that says something else — `spec-trace` resolves the anchor and reports green. Every re-anchored
citation is **opened and read**, and the repair records what the new referent says, per
`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`. Where the sentence the clause needs
has moved into an ADR rather than staying in the code, cite the ADR — the architecture brief's own
closing note is that this project's implementation *deletes* the prose that holds phase 2's findings,
and that the findings must land in the atom that consumed them first.

**11. `spec-trace` will be red immediately after the edits, and that is expected — regenerate, never
hand-edit.** §7.1 and §7.2 are held **equal** to what the tool computes and §7.2's rule cell is the
`Rule:` text truncated to 79 characters (`xtask/src/spec_trace.rs:735-744`;
`spec/SPECIFICATION.md:8717`). Every `Rule:`-line edit changes a rendered cell. Run
`cargo xtask spec-trace --write` and commit the regenerated regions; hand-editing a region held equal
to a computation is how the equality stops meaning anything.

**12. Two non-clause findings were handed forward to this story and both must be closed out
explicitly.** `headline-rules-and-mutant-registry` (HS-S0105) recorded that `SyncPeer::push`'s
rustdoc — *"Transport- or authorisation-level refusals only. A peer may not refuse a push because it
disagrees with the events in it"* (`crates/happenstance-sync/src/peer.rs:140-148`) — reads as though
SY-6's mandatory refusal of an `after`-carrying guard might itself be forbidden, when refusing a
structurally unreplicable **condition** is not disagreement with an **event**. And
`ingest-store-and-memory-peer-round-trip` (HS-S0102) owes this story a recorded list of every
`spec/SPECIFICATION.md` and test citation its diff invalidated. Each is either discharged here or
recorded as no-longer-applicable **with the evidence that it stopped applying** — a handoff quietly
dropped is indistinguishable from a handoff never made.

**13. The persona-journey slice.** The reader is the **evaluator** on *Decide in one sitting* — *"the
evaluator's bounded look at public evidence, ending in adopt or decline for a stated reason"*
(`.bklg/from-contract-to-published-library/initiative.md:249-250`, in the journey list at `:243-250`). `spec/SPECIFICATION.md` is the
public evidence, and its 200 clauses are its entire claim to being an instrument rather than a
brochure. An evaluator who opens SY-12, reads a `Rejects:` field describing a compile error that no
longer occurs, and checks it against the tree learns something worse than "the clause is stale": they
learn the document is not maintained against the code, and every other clause loses credibility with
it. Secondary reader: the **adapter author** on *Learn when you are finished*, who follows a `Rule:`
field to a rule and, before this PR, finds three that ship and still render as scheduled.

## Integration contract

- **Archetype**: `capability` — the observable slice is a reader's: an evaluator opens any clause in
  the audit list and finds it naming something that exists, and a maintainer who de-marks a clause
  whose rule does not exist is stopped by the gate.
- **Slice / milestone**: `spec-repairs-and-clause-exit`. Slice-mate, implemented in the same context
  and merged after this story: `clause-arithmetic-and-deferral-renewals` (HS-S0113), which owns every
  maturity marker and the exit arithmetic and takes this story as its hard predecessor
  (`…/_storymap.md`, *Merge order* 7). Declared `depends_on`:
  `headline-rules-and-mutant-registry` (HS-S0105 — a marker may only come off once its rule exists)
  and `adr-0026-peer-ingest-and-transport` (HS-S0098 — the authorisation). In practice this story
  runs **last of the seven slices**, and its audit input is the union of findings recorded by slices
  2–6.
- **Mount point**: **`xtask/src/spec_trace.rs`** — specifically `has_suite` (`:1735-1737`) and check
  4's loop (`:694-711`). This is the file where a specification repair stops being prose and becomes
  something the gate can fail. `spec/SPECIFICATION.md` is the *document* this story edits, but a
  clause edit that no check consults is the documentation form of a component nobody imported: today
  every `SY` clause is skipped by check 4 regardless of its marker, so removing `(new)` from SY-1
  changes one rendered table cell and nothing else. Adding `SY-` to `has_suite` is the mount — after
  it, each `Rule:` name in a de-marked `SY` clause is resolved against `RULE_FILES` (`:85-89`, which
  `gate-mounts-for-the-sync-suite` widened to the sync suite) and a name that does not resolve fails
  `cargo xtask spec-trace` by clause id and rule name.
- **Wires into**:
  - `spec/SPECIFICATION.md` — the `Rule:` and `Rejects:` fields and citations of the audit-list
    clauses (§6), and the generated §7.1/§7.2 regions regenerated by `--write` (`:8717`);
  - `xtask/src/spec_trace.rs` — `has_suite` (`:1735-1737`), check 4 (`:694-711`), `rules_of`'s marker
    logic (`:1626-1634`), `RULE_FILES` (`:85-89`) and the `--write` equality (`:735-744`), all of
    them read-only here except `has_suite`;
  - `crates/happenstance-sync-testkit/src/rules.rs` and the rest of `RULE_FILES` — **read**, as the
    ground truth for which rules exist; no rule is added, renamed or moved by this story;
  - `crates/happenstance-sync/src/peer.rs` — the `SyncPeer::push` rustdoc of Context pack §12,
    **doc comments only**, no item, signature or body;
  - `.kb/decisions/0026-*.md` — the authorisation each repair cites, read and never edited;
  - the implementation reports of HS-S0101, HS-S0102, HS-S0105 and the rest of slices 2–6 — the
    recorded citation-invalidation findings this story consumes (grep both **HS-S0112** and
    **HS-S0114**, per *Scope lock*).
- **Renders surfaces**: **none.** `…/replication-identity-and-ingest/_design.md` records a
  no-surface determination approved 2026-08-12 with `design.capture` a declared skip. This story adds
  no public Rust item, so even the API-surface obligation that file redirects to the briefs is
  vacuous here; the one `.rs` change it makes is a doc comment on an existing item, which
  `standards/rust/70-rustdoc-obligations.md` still governs.
- **Public items**: **none.** `has_suite` is a private `fn` in `xtask`; the `push` doc comment
  changes documentation on an item that already exists.
- **Conformance rule(s)**: **none added, and this story is not adapter-observable** — no port, no
  value type, no wire shape changes, so there is nothing an adapter could newly pass or fail. It is
  nevertheless **gate-observable**, which is the substitute obligation it owes: the negative control
  of Context pack §4 (a de-marked clause naming a rule that does not exist must fail `spec-trace` by
  name) is the check that this story's own deliverable can be got wrong. The rules it *cites* as
  discharges are the ones HS-S0105 landed — `ingest_never_rejects`,
  `compensation_is_atomic_with_the_losing_event`, `wire_condition_with_after_is_refused` — plus
  whichever of SY-5/SY-11/SY-12/SY-30's named rules slices 3–6 actually landed.
- **Clause(s)**: **repairs, amends none, edits no normative sentence.** In the `Rule:` field: SY-1,
  SY-2, SY-6 and every other `SY`/`WF` clause whose scheduled rules now resolve (computed, §4). In
  the `Rejects:` field and citations: SY-1, SY-6, SY-12, and SY-9/SY-10/SY-15/SY-17's citations into
  `crates/happenstance-sync/`'s module and test prose (§6). Every MUST stays **verbatim** and every
  maturity marker is untouched — marker movement is HS-S0113's. **Changing a `[FROZEN]` clause takes
  a new ADR, and this story writes none**: where the mechanical test says *gap*, it stops and records.
- **Advances DoD scenario**: project **DoD 7** — *"No `[FROZEN]` clause was edited, and `git diff`
  over `spec/SPECIFICATION.md` shows only additions a decision record authorises"* — which this story
  is the only one able to move, because it is the only one that edits the file. Through it, initiative
  **DoD 12** (*"the clause ledger is audited… and the count in the report matches
  `spec/SPECIFICATION.md`'s own stated figure"*, `initiative.md:393-395`): this story makes the
  clauses true and HS-S0113 makes the count true, in that order. Initiative **DoD 13** (*"the gate is
  green on the assembled whole"*) is the not-regressed obligation — `cargo xtask spec-trace` must be
  green after `--write`, and it is the step most likely to go red on a `Rule:` edit committed without
  regeneration.

## PR boundary

```
spec/SPECIFICATION.md
xtask/src/spec_trace.rs
crates/happenstance-sync/src/peer.rs
.bklg/from-contract-to-published-library/replication-identity-and-ingest/frozen-clause-repairs/**
```

**In this PR**

- The `re-check|recheck|re-evaluat` sweep re-run at the tree this story starts from, with the hit
  list, the line numbers and the verdict per hit recorded in the implementation report — including a
  null finding stated as a null finding.
- `has_suite` in `xtask/src/spec_trace.rs` extended to `SY-`, with the reason written in the
  function's own doc comment (the sync suite now exists in `RULE_FILES`; `PS-` deliberately still
  does not) and a unit test in `xtask`'s module-local style asserting that a de-marked clause naming
  a rule that does not resolve is reported by clause id and rule name.
- The `(new)` / `†` / scheduled-marker drops on exactly those `SY` and `WF` clauses whose `Rule:`
  names rules that all resolve against `RULE_FILES` — computed from the tree, enumerated in the
  ledger with the resolving file for each.
- The `Rejects:`-field and citation repairs for the audit list of Context pack §6, each in the
  playbook's three-part form, each naming its ADR-0026 authorisation, each recording the mechanical
  test's answer, and each re-anchored citation verified by reading the new referent.
- A recorded verdict for **every** id in the audit list, including *keeps a live target* entries
  where nothing changed.
- The `SyncPeer::push` doc-comment clarification of Context pack §12, or the recorded evidence that a
  slice-2–5 rewrite already discharged it.
- §7.1/§7.2 regenerated with `cargo xtask spec-trace --write`.
- This story's own `_ledger.md` and implementation report.

**Explicitly not in this PR**

- **Any maturity marker, any deferral renewal, any clause arithmetic.** `[FROZEN]`, `[PROVISIONAL]`
  and `[DEFERRED]` movement, CF-38's named-experiment sweep and the ADR-0026 ∪ ADR-0027 range union
  are `clause-arithmetic-and-deferral-renewals`' (HS-S0113), which merges after this story.
- **Any normative sentence.** No MUST, MUST NOT, SHOULD or MAY changes by a character. Softening
  SY-6's *"MUST be refused"* to *"SHOULD"*, or narrowing SY-1's *"any reason that is a function of the
  receiving store's state"* to *"any `AppendCondition`"*, are the amendments-in-repair-clothing that
  `discover.md`'s third mutant names; the tell is that the edit makes the clause agree with the
  **code** rather than making the clause's *description* agree with the code.
- **Any new ADR, and any `.kb/` write.** Where the mechanical test says *gap*, the output is a
  recorded finding and a raised blocker — the atom and the re-plan are outside this story, and atoms
  arrive only through `/redkiln:kb-ingest`.
- **Any conformance rule**, added, renamed, retired or moved. This story reads `RULE_FILES`; a
  clause that needs a rule which does not exist is a finding for the story that owns that rule, never
  a rule written here to make a citation resolve.
- **Any `crates/happenstance-sync/` change beyond doc comments**, and no other crate at all. If a
  repair appears to need a code change to be true, that is a gap.
- **`spec/E2E-CASES.md` and the `Cases:` fields.** Case ids are unchanged; a case that no longer
  holds is a finding, not an edit here.
- **`cargo xtask ci`** — the whole gate is the terminal project's; `cargo xtask ci --fast` is this
  project's ceiling (`project.md`, DoD 1).

The implementer **may** additionally touch the wiring file named in the Integration contract where
mounting requires it: `xtask/src/spec_trace.rs` is inside the boundary precisely because a clause
repair no check consults is not mounted, and teaching `has_suite` about `SY-` is that mount rather
than scope drift. Widening the boundary to any other `xtask` constant is drift — those are
HS-S0104's and are already in the tree when this story starts.

**Merge DoD**: `cargo xtask affected --base main` green; `cargo xtask lints && cargo xtask spec-trace`
green after `--write` regeneration, with the negative control demonstrated and reverted;
`cargo test -p xtask` green including the new `has_suite` test; `cargo xtask ci --fast` green on the
tree; `git diff main -- spec/SPECIFICATION.md` reviewed against project DoD 7 and showing only
`Rule:` fields, `Rejects:` fields, citations and generated regions; `_ledger.md` cites evidence per
AC-###.

## Behavior and interfaces

The **binding** parts below are the ones carrying a citation — the mechanical test and its stop
condition, the three-part form, the marker computation being read from the tree, the predicate
change, and the audit list being closed. Wording of individual repairs is the implementer's, and a
deviation is legitimate when it is recorded with its reason.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The sweep is re-run and its result recorded either way** | `re-check\|recheck\|re-evaluat` over `spec/SPECIFICATION.md` at the tree this story starts from. Record every hit with its line number and a one-line verdict. The HEAD run on 2026-08-12 gave ten hits, five about ingest, all consistent with SY-1/SY-6 as frozen — a **null finding**, which is a result and is recorded as one. Six slices of code have landed since, so the run is repeated rather than quoted | `_decomposition.md`, *Tension 4*; `…/frozen-clause-repairs/discover.md`, *Signal Ledger* row 5 and *Questions* 1; `spec/SPECIFICATION.md:4373`, `:5825`, `:5862`, `:5998`, `:6014` |
| **The repair/gap decision is mechanical and answered before any edit** | For each candidate: *is there an implementation that was conformant before and is not after, or vice versa?* No → repair. Yes **or cannot tell** → gap. The answer is written down per clause, not per pass | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, *The test: repair or gap*; `.kb/decisions/README.md` |
| **A gap stops the story rather than shrinking the edit** | Output is a recorded finding plus a raised blocker naming what an ADR would have to decide; the finding sits **inside the clause it is about**, and nothing normative changes. A finding filed elsewhere is one the next reader of the clause will not see | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, *The form for a gap*; `project.md`, *Out of scope* |
| **A discharged obligation keeps its MUST and gains a named discharge** | Three components, all required: MUST **verbatim**; the discharge **named as a discharge**; the code **and the test** cited. Deleting a met obligation is the tempting edit and is almost always wrong — *"the obligation's bare absence reads as an oversight"* | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, *The safe form for a discharged MUST* |
| **A discharge cites a test that itself honoured CF-6** | Where a repair points at a rule's assertion as the discharge, that assertion must not be a literal-position assertion — pointing at one would launder a defect into the specification as evidence | `…/frozen-clause-repairs/discover.md`, *Gate: Discover* box 6; `spec/SPECIFICATION.md:7233-7249` |
| **`has_suite` learns `SY-`, and only `SY-`** | `xtask/src/spec_trace.rs:1735-1737` is `starts_with("ES-") \|\| starts_with("VT-") \|\| starts_with("WF-")`. Add `SY-`; leave `PS-` out, because the projection suite does not exist and including it reports every `PS` rule as missing — the noise the original comment at `:684-688` refuses. Record the reason in the function's doc comment | `xtask/src/spec_trace.rs:684-711`, `:1735-1737`, `:85-89`; `…/gate-mounts-for-the-sync-suite/spec.md` (grep `has_suite`: unclaimed) |
| **A de-marked clause naming a non-existent rule fails the gate by name** | The negative control: transiently misspell a rule name in a de-marked `SY` clause and assert `cargo xtask spec-trace` reports it as *"names rule `x`, which is not in … and the clause does not declare it new"*, with the clause id. Demonstrate, record the transcript, revert. A `has_suite` change asserted only by a green gate proves nothing | `xtask/src/spec_trace.rs:694-711`; `…/gate-mounts-for-the-sync-suite/spec.md`, AC-002's transient-violation pattern |
| **The marker drops are computed from the tree, not from a list** | For each `SY`/`WF` clause carrying a scheduled marker, resolve every name in its `Rule:` field against the rules defined in `RULE_FILES`. All resolve → drop the marker. Any unresolved → keep it, and record which name is missing. The ledger lists each drop with the file the rule resolved in | `_storymap.md`, *Slices*, `spec-repairs-and-clause-exit` row 1; `xtask/src/spec_trace.rs:85-89`, `:1626-1634`, `:1746-1755` |
| **Marker drop ≠ marker invention** | The same mechanism recognises `†`, a leading `new `, `" new \`"` and the words *unit test / compile test / meta-test*. Removing the phrase *"compile test"* from a clause to make it resolvable would be an amendment of what the clause says its rule is — WF-12's comment at `:1608-1624` records exactly that trap. Only the `(new)`/`†` scheduling markers come off | `xtask/src/spec_trace.rs:1608-1634` |
| **The audit list is closed and every id gets a verdict** | SY-1, SY-2, SY-6, SY-9, SY-10, SY-12, SY-15, SY-17 and every clause whose scheduled rule landed. Verdict ∈ {keeps a live target, repaired, gap}. *Keeps a live target* is the expected majority result and is recorded, not omitted | `…/frozen-clause-repairs/discover.md`, *Questions* 3; `_decomposition.md`, AC-A11 |
| **SY-12 gets a live target or a named discharge, never a generic restatement** | Its previous exemplar was withdrawn once and the clause was re-pointed rather than retired; slices 2's work removes the replacement. A restatement like *"a peer that carries identity somewhere unreachable"* names nothing anyone would ship — CF-4's saboteur bar, one level up | `spec/SPECIFICATION.md:6173-6183`, `:7208-7215`; `crates/happenstance-sync/src/ingest.rs:38-50`; `…/frozen-clause-repairs/discover.md`, *The wrong implementation* |
| **SY-1 and SY-6 are repaired only if `guard` actually changed** | Both name the **public** `guard: Option<AppendCondition>` field on `EventGroup` as the reason re-evaluation is reachable. If ADR-0027 left it public, both keep a live target and the verdict is *keeps a live target*. If it was made private, renamed or removed, both lose their named wrong implementation in this project's diff and owe a repair in the same commit | `spec/SPECIFICATION.md:5856-5867`, `:5998-6006`; `crates/happenstance-sync/src/peer.rs:236-243`; `_decomposition.md`, *Tension 4* and AC-A11 |
| **Every re-anchored citation is read before it is re-pointed** | A citation moved to a line that resolves but says something else is undetectable by every tool in this repository. Record what the new referent says. Where the sentence has moved out of the code and into ADR-0026, cite the atom | `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md`; `spec/SPECIFICATION.md:6120-6121` → `crates/happenstance-sync/src/lib.rs:57-63` |
| **The two handed-forward findings are closed out explicitly** | (a) `SyncPeer::push`'s rustdoc reading as though SY-6's refusal were forbidden — one clarifying sentence distinguishing refusal of a structurally unreplicable *condition* from disagreement with an *event*, or evidence a slice-2–5 rewrite already says it. (b) HS-S0102's recorded citation-invalidation list, consumed entry by entry. Each ends *discharged* or *no longer applicable, because …* | `crates/happenstance-sync/src/peer.rs:140-148`; `…/headline-rules-and-mutant-registry/spec.md`, *Context pack* §6; `…/ingest-store-and-memory-peer-round-trip/spec.md`, *Context pack* §11 |
| **The generated regions are regenerated, never hand-written** | Every `Rule:` edit changes §7.2's 79-character rule cell, so `spec-trace` is red in `Mode::Check` until `--write` runs. Regenerate and commit; a hand-edited region held equal to a computation destroys the equality | `xtask/src/spec_trace.rs:735-744`; `spec/SPECIFICATION.md:8717` |
| **The diff is reviewable against DoD 7 by inspection** | `git diff main -- spec/SPECIFICATION.md` contains only `Rule:` fields, `Rejects:` fields, citations and generated regions. Any hunk touching a normative sentence or a maturity marker is a boundary breach, visible in `git diff` rather than argued afterwards | `project.md`, DoD 7; `.redkiln/config.yaml:62-67` (`require_ledger`) |

## Data and migrations

**N/A — and the reason is worth stating rather than asserting, because this story looks like a
migration and is not.**

No schema, no persisted state, no wire format and no on-disk artifact changes. `spec/SPECIFICATION.md`
is a text file under version control; `xtask/src/spec_trace.rs` is a build-time checker that reads it
and writes only its own generated regions back; `crates/happenstance-sync/src/peer.rs` changes by a
doc comment. Nothing this story touches is read by a running program, serialised, or persisted
anywhere a previous version could have written.

Two things that would otherwise be migration-shaped are explicitly held elsewhere. **The generated
§7.1/§7.2 regions** are derived data with a one-way regeneration (`cargo xtask spec-trace --write`)
and no reverse migration — the source of truth is the clause bodies, and the regions are recomputed
rather than edited, which is why hand-editing them is forbidden above. **`FORMAT_VERSION` and the
wire message set** are `message-set-on-the-envelope`'s (HS-S0106) and ADR-0027's; nothing in this
story can move a version number, and if a repair appears to need one, that is a gap under Context
pack §1 rather than a migration.

The nearest thing to a migration here is a **record** rather than a transformation: the audit list's
per-id verdicts and the sweep's hit list are what a future reader uses to tell a clause that was
checked and found true from a clause nobody looked at. That record lives in this story's
`_ledger.md` and implementation report, not in the specification.

## Acceptance criteria

The persona throughout is the **evaluator (pre-adoption)** on *Decide in one sitting* —
*"a Rust developer who has **not yet** decided to adopt anything… decide, in a bounded amount of
research time, whether `happenstance`'s claims are real, without being able to run the project's own
internal test suite themselves or take the maintainers' word for it"*
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:249-266`;
the journey line at `.bklg/from-contract-to-published-library/initiative.md:249-250`). For that
persona `spec/SPECIFICATION.md` **is** the product: their own field has *"nothing external to check
against… 'DCB-compliant' is industry-wide an unverified, self-asserted label"*
(`personas-and-journeys.md:271-277`), so a clause whose `Rejects:` names a thing that no longer
exists does not merely mislead — it converts the one instrument the project offers back into the
self-assertion the evaluator came to escape. The secondary persona is the **adapter author** on
*Learn when you are finished* (`personas-and-journeys.md:114-167`), who follows a `Rule:` field to a
rule and today finds three that ship and still render as scheduled. The tertiary reader is the
**maintainer** — the person who, six months from now, drops a `(new)` marker and needs the gate to
tell them they were wrong.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an evaluator who wants to know whether the specification still frames ingest as re-checking a writer's conditions — the charter's own named worry — **WHEN** the implementer re-runs `re-check\|recheck\|re-evaluat` over `spec/SPECIFICATION.md` at the tree this story starts from, **THEN** the implementation report carries **every** hit with its line number and a one-line verdict against SY-1/SY-6-as-frozen, and states the outcome as a finding in its own words — including *"null finding: no clause describes ingest as re-checking conditions"* if that is what the run says — never as a quotation of the HEAD run recorded in `discover.md` | The recorded hit list in the implementation report, re-derivable by re-running the sweep on the merge commit; reviewed against `…/frozen-clause-repairs/discover.md`, *Signal Ledger* row 5 (ten hits, five about ingest, at HEAD 2026-08-12) — a report whose hit list is byte-identical to that row **and** carries no re-run transcript fails review |
| AC-002 | **GIVEN** a maintainer who removes a `(new)` marker from an `SY` clause, **WHEN** `cargo xtask spec-trace` runs, **THEN** that clause's `Rule:` names are actually resolved — because `has_suite` (`xtask/src/spec_trace.rs:1735-1737`) admits `SY-` alongside `ES-`, `VT-` and `WF-` (three prefixes to four), still refuses `PS-`, and its rustdoc states why the asymmetry is deliberate: the replication suite is now in `RULE_FILES` and the projection suite is not, so admitting `PS-` would report every `PS` rule as missing — *"noise indistinguishable from a real typo"* (`:684-688`) | `xtask/src/spec_trace.rs::tests::has_suite_admits_sy_and_still_refuses_ps` (a `#[cfg(test)] mod tests` in the module, the shape `xtask/src/package.rs:408` already uses), green under `cargo test -p xtask`; plus review that the rustdoc names `RULE_FILES` as the reason rather than restating the code |
| AC-003 | **GIVEN** the same maintainer, but they have de-marked a clause whose named rule does **not** exist — a typo, or a rule the slice never landed — **WHEN** `cargo xtask spec-trace` runs, **THEN** the gate is **red**, naming the clause id and the missing rule in the existing check-4 message form (`… — SY-n names rule \`x\`, which is not in … and the clause does not declare it new`, `xtask/src/spec_trace.rs:694-711`), and the implementation report carries the transcript of that failure demonstrated against the real tree and then reverted | `xtask/src/spec_trace.rs::tests::de_marked_sy_clause_reports_its_missing_rule_by_name` green under `cargo test -p xtask`; **and** the recorded transient-violation transcript (misspell one rule name in a de-marked `SY` clause, run `cargo xtask spec-trace`, capture, revert), the pattern `…/gate-mounts-for-the-sync-suite/spec.md` AC-002 establishes. A green gate alone does not verify this AC |
| AC-004 | **GIVEN** an adapter author who follows SY-1's `Rule:` field expecting to find `ingest_never_rejects`, **WHEN** they read `spec/SPECIFICATION.md` after this PR, **THEN** every `SY`/`WF` clause whose `Rule:` names rules that **all** now resolve against `RULE_FILES` has lost its `(new)`/`†` scheduling marker, every clause with an unresolved name has **kept** its marker with the unresolved name recorded, and the set was **computed from the tree** — the ledger names each drop together with the file the rule resolved in, and the enumeration exists nowhere in this spec because slices 3–6 landed rules a plan written earlier cannot list without going stale | `cargo xtask spec-trace` green (the AC-002 predicate makes each drop load-bearing); the per-drop table in the implementation report, each row cross-checkable against `xtask/src/spec_trace.rs::collect_rules` over `RULE_FILES` (`:85-89`); review that no drop was copied from a list rather than derived |
| AC-005 | **GIVEN** a maintainer tempted to make a clause resolvable by tidying its prose, **WHEN** the diff is reviewed, **THEN** only `(new)` and `†` scheduling markers were removed: the words *unit test*, *compile test* and *meta-test* — the `elsewhere` family at `xtask/src/spec_trace.rs:1626-1631` — are untouched in every clause, WF-12 in particular still reading *"a const-evaluation assertion … **not** a compile test"* (`:1617-1624`), because removing those two words would change what the clause says its rule **is** and turn WF-12 into a `†` no test can ever clear | `git diff main -- spec/SPECIFICATION.md` reviewed for any removal of `unit test`/`compile test`/`meta-test`; `cargo xtask spec-trace` green (a WF-12 tidied into resolvability would go red on the missing `read_options_is_not_serialisable`, which is a `const _` and not a `#[test]`) |
| AC-006 | **GIVEN** an evaluator auditing whether the specification was checked or merely edited, **WHEN** they open the implementation report, **THEN** **every** id in the pre-committed audit list — SY-1, SY-2, SY-6, SY-9, SY-10, SY-12, SY-15, SY-17, plus every clause whose scheduled rule this project landed (`…/frozen-clause-repairs/discover.md`, *Questions* 3) — carries exactly one verdict from {**keeps a live target**, **repaired**, **gap**} with its evidence, no id is absent, and where the verdict is **gap** the recorded finding sits **inside the clause it is about** with nothing normative changed and a blocker raised naming what an ADR would have to decide | The per-id verdict table in the implementation report, its id set compared against `discover.md`, *Questions* 3 by review; for any `gap`, the in-clause finding visible in `git diff main -- spec/SPECIFICATION.md` and the blocker on the story item. *Keeps a live target* rows are expected to be the majority and a report with none is a report that stopped looking |
| AC-007 | **GIVEN** an evaluator who opens SY-12 — the clause that has already survived one withdrawal and records it in its own text (`spec/SPECIFICATION.md:6173-6183`) — after `memory-store-ingest-seam` and `ingest-store-and-memory-peer-round-trip` made `impl IngestStore for MemoryEventStore` truthful, **THEN** its `Rejects:` either names a **new wrong implementation someone would actually ship** or records a **discharge in the three-part form** — the MUST **verbatim**, the discharge named as a discharge, and the code **and** the test that now assert it cited — and in neither case is the field deleted or replaced by a generic restatement such as *"a peer that carries identity somewhere unreachable"* | Review of the SY-12 hunk against `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, *The safe form for a discharged MUST*, and against CF-4's saboteur bar — *"the mutant that earns its place is the one someone would ship"* (`spec/SPECIFICATION.md:7208-7215`); `cargo xtask spec-trace` resolves the cited rule and citation; the cited test is confirmed to exist and to assert what the discharge claims |
| AC-008 | **GIVEN** an evaluator reading SY-1 and SY-6, whose `Rejects:` fields both name the **public** `guard: Option<AppendCondition>` field on `EventGroup` as the reason re-evaluation is reachable at all (`spec/SPECIFICATION.md:5856-5867`, `:5998-6006` → `crates/happenstance-sync/src/peer.rs:236-243`), **THEN** the disposition of that field at the merge tree decides the verdict and the verdict is recorded: field still public → **keeps a live target**, verdict recorded and the field left alone; field made private, renamed or removed by ADR-0027's slice → **repaired in this same commit**, in the three-part form, with the mechanical test's answer written down | Read `crates/happenstance-sync/src/peer.rs` at the merge tree and record the field's visibility; the recorded verdict in the ledger, cross-checked against `_decomposition.md`, **AC-A11** (*"a repair in the same commit"*). A `repaired` verdict on an unchanged field, or a `keeps a live target` verdict on a field that went private, both fail review |
| AC-009 | **GIVEN** an evaluator who follows a clause's `file:line` citation into `crates/happenstance-sync/` — module prose this project rewrites wholesale — **THEN** every citation this story re-anchored was **opened and read first**, the implementation report records what the new referent actually says in the implementer's own words, and where the sentence the clause needs has moved out of the code and into a decision record the citation points at the record rather than at whatever now occupies the old lines. Specifically covered: SY-9/SY-10 at `spec/SPECIFICATION.md:6120-6121` → `crates/happenstance-sync/src/lib.rs:57-63`; SY-6's citation of `crates/happenstance-sync/src/lib.rs:100-104`; SY-15/SY-17's neighbourhood at `spec/SPECIFICATION.md:5775` naming the `tests/real_peer_shapes.rs` stand-ins | The per-citation "what it now says" record in the implementation report, spot-checked by review against the referent; `cargo xtask spec-trace`'s citation check (`xtask/src/spec_trace.rs:731-733`) proves only that the anchor **resolves** — it cannot see this defect, which is why the record is the verification |
| AC-010 | **GIVEN** two findings handed forward to this story by earlier slices, **WHEN** the implementation report is read, **THEN** each is closed **explicitly**: (a) `SyncPeer::push`'s rustdoc — *"Transport- or authorisation-level refusals only. A peer may not refuse a push because it disagrees with the events in it"* (`crates/happenstance-sync/src/peer.rs:140-148`) — either gains one clarifying sentence distinguishing refusal of a structurally unreplicable **condition** from disagreement with an **event**, or carries the evidence that a slice-2–5 rewrite already says it; (b) `ingest-store-and-memory-peer-round-trip`'s recorded list of `spec/SPECIFICATION.md` and test citations its diff invalidated is consumed entry by entry, each ending *discharged* or *no longer applicable, because …* | Review of `crates/happenstance-sync/src/peer.rs`'s doc comment against `standards/rust/70-rustdoc-obligations.md`; the entry-by-entry closeout table in the implementation report, its input list located by grepping the earlier implementation reports for **both** `HS-S0112` **and** `HS-S0114` (the id split named in *Scope lock*); `cargo test -p happenstance-sync --doc` green |
| AC-011 | **GIVEN** the repository owner reviewing this PR against project DoD 7 — *"No `[FROZEN]` clause was edited, and `git diff` over `spec/SPECIFICATION.md` shows only additions a decision record authorises"* — **WHEN** they read `git diff main -- spec/SPECIFICATION.md`, **THEN** every hunk is a `Rule:` field, a `Rejects:` field, a citation, an in-clause recorded finding, or a `--write`-regenerated §7.1/§7.2 region: **no MUST, MUST NOT, SHOULD or MAY changes by a character**, **no `[FROZEN]`/`[PROVISIONAL]`/`[DEFERRED]` marker moves**, no clause id changes, no `Cases:` field changes, and each repair hunk is traceable to the ADR-0026 authorisation that permits it | `git diff main -- spec/SPECIFICATION.md` reviewed hunk by hunk (the DoD-7 pass); `cargo xtask spec-trace` green **after** `cargo xtask spec-trace --write` regeneration — the 79-character §7.2 rule cell (`xtask/src/spec_trace.rs:735-744`, `spec/SPECIFICATION.md:8717`) changes on every `Rule:` edit, so a green check-mode run is itself evidence the regeneration happened; `cargo xtask ci --fast` green |

**Traceability to the project's ACs.** AC-006, AC-007, AC-008 and AC-009 discharge project **AC-012**
(*"the specification passages whose named wrong implementation is `happenstance-sync`'s own superseded
proposal are audited **by id**… each clause either keeps a wrong implementation that still exists, or
is corrected by decision record"*) — the audit's closure is AC-006, its two named hard cases are
AC-007 and AC-008, and its citation half is AC-009. AC-001, AC-005 and AC-011 discharge project
**AC-002**'s share for this story (*"no frozen clause is edited in this project's diff"*, and the
central question reconciled rather than assumed): AC-001 is the reconciliation re-run, AC-005 and
AC-011 are the two ways an edit could stop being a repair. AC-002, AC-003 and AC-004 are the mount
that makes the whole of AC-012 mechanical rather than asserted — the testing brief names
`spec-trace`'s `Rejects:`/rule resolution as AC-012's Static-tier instrument
(`_decomposition.md`, *The test mix* → **AC-012**), and today that instrument does not look at `SY`
clauses at all. AC-010 carries the two non-clause handoffs that would otherwise die between stories.

## Interaction quality

RFC §6.7/D6. **This story renders no surface** — `…/replication-identity-and-ingest/_design.md`
records a no-surface determination approved by the repository owner on 2026-08-12, with
`design.capture` a declared skip and every composition section reading *"N/A — no user-facing
surface"*. What it does have is two *read* surfaces that a human interacts with under exactly the
pressures D6 is written about: `spec/SPECIFICATION.md`, which the evaluator reads clause by clause,
and `cargo xtask spec-trace`'s output, which the maintainer reads when the gate stops them. The
invariants below therefore bind to those, and **every one is carried by an AC row in the table above**
— nothing here is a free-floating bullet, because a bullet in this section gets no ledger row and is
never gated.

**State invariants.**

| invariant | carried by | how it is verified |
| --- | --- | --- |
| **In place, not a context jump** — a finding lands inside the clause it is about; a reader who opens SY-12 sees its disposition without navigating to a report | AC-006, AC-007 | `git diff main -- spec/SPECIFICATION.md` shows the finding in the clause body; a `gap` recorded only in the implementation report fails AC-006 |
| **Non-occlusion** — nothing this story writes hides a stale referent behind a resolving anchor; the citation mutant (`discover.md`, *The wrong implementation*, fourth paragraph) is exactly an occlusion defect | AC-009 | The "what the new referent now says" record; the tool's citation check cannot see this, so the record is the instrument |
| **Preserved selection and focus** — the clause's identity survives the edit: id, MUST text, maturity marker and `Cases:` field are byte-identical, so a reader's existing citation of the clause still means what it meant | AC-011, AC-005 | Hunk-by-hunk DoD-7 review; `cargo xtask spec-trace` green after `--write` |
| **Reversibility** — the negative control is demonstrated and reverted, and a `gap` verdict reverses the story rather than shrinking the edit (recorded finding + blocker, nothing normative moved) | AC-003, AC-006 | The captured-then-reverted transcript; `git diff` clean of the transient misspelling at merge |
| **Reachability** — the maintainer's keyboard-equivalent: a repair must be reachable by the gate at all. Before this story, `spec-trace` never consults an `SY` clause's rules, so a de-marked `SY-` clause is unreachable and no wrong version of this PR can be rejected | AC-002, AC-003 | The two `xtask/src/spec_trace.rs` unit tests plus the transient-violation transcript |

**Composition invariants.** The signed-off design declares no rendered surface, so its composition
sections are vacuous by its own text; the composition discipline that *does* bind here is the
clause's own composed form, which `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`
fixes and which this story is the first to be measured against.

| invariant | carried by | how it is verified |
| --- | --- | --- |
| **Presentation exists at all** — a repair is a *composed* three-part clause, never bare markup. Deleting a met obligation, or leaving a `Rejects:` field that names nothing, is the "unstyled render" of a clause: it satisfies every mechanical check and communicates nothing | AC-007 | Review against *The safe form for a discharged MUST*; a deleted `Rejects:` field or a generic restatement fails |
| **Hierarchy** — the MUST stays first and verbatim, the discharge is named as a discharge beneath it, the evidence is cited last. A reader must not be able to mistake a satisfied obligation for a relaxed one | AC-007, AC-011 | Review of each repair hunk; any reordering that puts the discharge above the obligation fails |
| **Density budget, with its real numbers** — the three-part form has **exactly three** components and a repair with two is incomplete; §7.2's rule cell is the `Rule:` text truncated to **79 characters** (`xtask/src/spec_trace.rs:735-744`); `has_suite` goes from **three** prefixes to **four**, not five; `RULE_FILES` stays at the **three** files it holds after HS-S0104 (`:85-89`) because this story adds no rule; the audit list is **eight** named ids plus a computed set | AC-002, AC-004, AC-007, AC-011 | `cargo xtask spec-trace` after `--write`; the ledger's per-drop table; review of each repair for all three components |
| **Placement and transience** — a null finding is **persistent chrome**, not a transient one: it is written into the report as a result. A blocker is the transient element and must not be the only record of a gap | AC-001, AC-006, AC-010 | The recorded sweep result; the per-id verdict table; the entry-by-entry handoff closeout |
| **The named anti-patterns**, all four from `discover.md`, *The wrong implementation* — (1) do nothing and stay green; (2) delete the `Rejects:` field because the obligation is met; (3) the repair that is quietly an amendment (softening SY-6's *"MUST be refused"*, narrowing SY-1's *"any reason that is a function of the receiving store's state"*); (4) re-anchor without reading the new referent | AC-006 (1), AC-007 (2), AC-011 and AC-005 (3), AC-009 (4) | Each named anti-pattern maps to a listed AC; adversarial review checks the mapping rather than the absence of the words |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | The mechanical test returns **yes** — an implementation was conformant before the edit and is not after, or vice versa — **or the implementer cannot tell** | **Stop.** This is a gap, and "cannot tell" is on the gap side deliberately (`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`, *The test: repair or gap*). Record the finding **inside the clause**, raise a blocker naming what an ADR would have to decide, change nothing normative, and do **not** make a smaller edit instead. This is `project.md`'s *Out of scope* stop condition and this story may not absorb it |
| EC-002 | A repair needs a rule that does not exist, or a `Rule:` name that resolves only under a different spelling | **Never write the rule here, and never widen the resolution set to make the name fit.** Rules are the owning story's; record the finding, keep the clause's `(new)` marker, and name the story that owes the rule. A rule added in this PR to make a citation resolve is the decorative-rule failure mode with documentation on top |
| EC-003 | A needed repair falls outside what ADR-0026 authorised | Two admissible outcomes only: it is **obviously within the same mechanical test** and lands with its own written justification naming the test, or it is a **gap** under EC-001. It is never filed under ADR-0026's heading because the heading is nearby (*Context pack* §3) |
| EC-004 | Adding `SY-` to `has_suite` turns `spec-trace` red on clauses **this story does not own** — an `SY` clause with no marker whose rule genuinely never landed | That is a **true finding, not noise**, and it is the predicate doing its job. Do **not** silence it by narrowing the predicate, by inventing a marker on a clause that never scheduled one, or by adding the rule. Record it, route it to the story that owns the rule, and raise it if it blocks. The `PS-` exclusion is the only asymmetry this story is authorised to keep, and only because the projection suite does not exist |
| EC-005 | `cargo xtask spec-trace` is red on §7.1/§7.2 staleness after the clause edits | Expected, not a defect: every `Rule:` edit changes a rendered 79-character cell. Run `cargo xtask spec-trace --write` and commit the regenerated regions. **Hand-editing a region held equal to a computation destroys the equality** and is a boundary breach even when the resulting text is correct |
| EC-006 | A re-anchored citation resolves but its referent says something else | Undetectable by every tool in this repository — the citation check proves existence and length, never meaning. The procedure is the control: open and read before re-pointing, record what the referent now says (AC-009). Where the sentence has moved into a decision record, cite the record; where it was deleted outright, that is a candidate gap under EC-001, not a citation to be moved somewhere plausible |
| EC-007 | A handed-forward finding cannot be located in the earlier slices' implementation reports | Grep for **both** `HS-S0112` and `HS-S0114` — six sibling specs are split across the two ids for the same story and the slug `frozen-clause-repairs` is the reliable key (*Scope lock*). If it is still absent after both, record it as **not handed forward** with the search performed, rather than silently closing AC-010 on an empty input |
| EC-008 | A repair wants to cite a rule's assertion as the discharge, and that assertion is a literal-position assertion | Refuse the citation. Pointing at a CF-6-violating assertion would launder a defect into the specification as evidence (`discover.md`, *Gate: Discover* box 6; `spec/SPECIFICATION.md:7233-7249`). Cite a different assertion, or record the discharge as unavailable and treat the clause under EC-001 |
| EC-009 | The `guard` field's disposition at the merge tree is ambiguous — it exists but behind a feature, or was renamed with no recorded ADR-0027 decision | Do not guess the verdict. Read `crates/happenstance-sync/src/peer.rs` at the merge commit, record the exact visibility and spelling found, and if the change is not traceable to a decision record, that ambiguity is itself a finding for ADR-0027's story before SY-1 and SY-6 can be given a verdict |
| EC-010 | `cargo xtask ci --fast` goes red on something this story did not touch | Not this story's to fix in this PR. Confirm the failure reproduces on `main` at the same tree; if it does, route it to the `support` initiative per `.redkiln/config.yaml` and record the routing. If it does **not** reproduce, it is this PR's and the story stops until it is understood |

## Non-functional

| id | requirement | how it is met |
| --- | --- | --- |
| NF-001 | **Gate cost is unchanged.** Adding one prefix to `has_suite` must not add I/O or measurably change `cargo xtask spec-trace`'s runtime | `has_suite` is a `starts_with` chain over an already-parsed clause id (`xtask/src/spec_trace.rs:1735-1737`); the extra work is check 4's inner loop over `SY` clauses against the already-built resolution set. No file is opened that `RULE_FILES` did not already open |
| NF-002 | **Regeneration is idempotent.** `cargo xtask spec-trace --write` run twice leaves the second run with an empty diff, and check mode is green immediately after | Run `--write`, commit, then run `cargo xtask spec-trace`; a non-empty second diff means the clause bodies and the generated regions disagree about something other than this story's edits |
| NF-003 | **The diff is reviewable in one sitting.** Project DoD 7 is a *human* review, so the diff must be inspectable hunk by hunk without cross-referencing a second document to classify a hunk | Each repair hunk carries its ADR-0026 authorisation and the mechanical test's answer in the clause body itself; a hunk whose legitimacy can only be established from the implementation report fails NF-003 and AC-011 together |
| NF-004 | **Nothing published, nothing compiled differently.** `xtask` is not a published crate; `crates/happenstance-sync` is `publish = false`; the only `.rs` change outside `xtask` is a doc comment | The `cargo package --list` assertions inside `cargo xtask ci --fast` are unchanged; no MSRV, feature, `wasm32` or dependency movement — a `Cargo.toml` in this PR's diff is a boundary breach |
| NF-005 | **Immutability of the knowledge base is preserved.** No `.kb/` atom is edited, superseded or added by this story | `git diff --name-only main -- .kb/` is empty; `redkiln validate --kb && redkiln doctor` green. Atoms arrive only through `/redkiln:kb-ingest` (`CLAUDE.md`, *Where the work lives*; the hand-authoring reverted at `0269720`) |
| NF-006 | **The record outlives the story.** A future reader can tell a clause that was checked and found true from a clause nobody looked at | The per-id verdict table (AC-006) and the sweep's hit list (AC-001) live in `_ledger.md` and the implementation report, both committed; *keeps a live target* is recorded rather than omitted |
| NF-007 | **The repair's register matches the corpus it joins.** A clause repaired here must be indistinguishable in voice from one written at phase 2 — the specification's authority rests partly on reading as one document | Review against neighbouring clauses in `spec/SPECIFICATION.md` §5; the discharge sentence names the discharge in the same idiom the playbook uses, not in commit-message English |

## Implementation notes (non-prescriptive)

Shape suggestions, not instructions. A deviation is legitimate when it is recorded with its reason.

**Do the predicate before the drops.** `has_suite` first, with its unit tests and the transient
negative control; only then compute and apply the marker drops. Every drop then lands against a check
that is live, and a mistaken drop fails immediately rather than at the end of a long edit. Reversing
the order gives a long green run that proves nothing (*Context pack* §5).

**Compute the drop set with the tool, not by eye.** `xtask/src/spec_trace.rs::collect_rules` over
`RULE_FILES` (`:85-89`) already produces the set of rules that exist, and the clause parser already
produces each clause's `Rule:` names and its `schedules_new`/`elsewhere` flags (`:1626-1634`). A
scratch run of the existing report will list every scheduled clause whose names all resolve.
Enumerating by reading is how a marker gets dropped from a clause whose *second* rule name is still
missing.

**Order the audit by cost, not by id.** The `keeps a live target` verdicts are cheap and are most of
the list; settle them first so the remaining budget goes to SY-12 (AC-007) and to whatever the
`guard` disposition turns out to be (AC-008). Record each verdict as it is reached rather than at the
end — a verdict table assembled from memory at the close is the mechanism by which an id gets
silently skipped.

**Write the repair before deciding it is a repair.** Draft the three parts, then apply the mechanical
test to the *drafted* text: is there an implementation conformant before and not after? Drafting
second tends to produce the smaller edit the playbook forbids, because the answer is already known.

**For the `has_suite` rustdoc, amend rather than replace.** The existing doc comment explains why
`PS-` and `SY-` were both excluded, and that explanation is still half true. Rewriting the referent —
*"the replication port's suite now exists in `RULE_FILES`; the projection store's does not"* — while
leaving the reasoning is exactly `.kb/governance/rewrite-the-referent-never-the-reasoning.md`'s
shape, applied to a doc comment rather than to a clause.

**Prefer citing the decision record over re-pointing into deleted prose.** The architecture brief's
closing note is that this project's implementation *deletes* most of `crates/happenstance-sync/`'s
self-documentation because it stops being true, and that *"the findings are not the same as the
`todo!()`s"* (`_decomposition.md`, *Notes*). Where a clause cites a paragraph this project removed,
the honest anchor is usually the atom that consumed the finding, not the nearest surviving line.

**Two `xtask` tests, not one.** `has_suite_admits_sy_and_still_refuses_ps` pins the predicate
including the deliberate `PS-` exclusion, so a later "tidy" that adds `PS-` fails by name.
`de_marked_sy_clause_reports_its_missing_rule_by_name` pins the *message*, so a refactor that keeps
the check but loses the clause id or the rule name from the output fails too. Neither substitutes for
the transient-violation transcript against the real specification, which is what proves both are
wired to the same code path the gate runs.

**Say what the null finding is a null finding about.** *"No hits"* and *"ten hits, none of which
describes ingest as re-checking conditions"* are different claims, and only the second is what
AC-001 asks for.

## Tests and CI (merge gate)

Grounded in the project testing brief's **Static** tier — *"`cargo xtask spec-trace` … This is the
check for AC-002 … and AC-012 (a `Rejects:` symbol that no longer exists is a repair, and
`spec-trace` is what would otherwise let it rot unnoticed)"* — and its *Merge-gate commands* block,
in the order a story actually runs them (`_decomposition.md`, *Testing brief*).

| tier | command / path | proves |
| --- | --- | --- |
| Static (story grain) | `cargo xtask affected --base main` | Nothing this diff touches regressed elsewhere; the story-grain command `.redkiln/config.yaml`'s `verify:` block wires whether or not anyone types it |
| Static (unconditional) | `cargo xtask lints && cargo xtask spec-trace` — `reachability_static` (`.redkiln/config.yaml:48`) | AC-004, AC-005, AC-011. Every de-marked clause's rule names resolve; no rule is orphaned; §7.1/§7.2 equal what the run computed. Red before `--write` is expected (EC-005) and green after is the evidence that regeneration happened |
| Static (write pass) | `cargo xtask spec-trace --write`, then `cargo xtask spec-trace` | NF-002, AC-011. The generated regions are recomputed rather than hand-edited; the second run's empty diff is the idempotency proof |
| Unit | `cargo test -p xtask` → `xtask/src/spec_trace.rs::tests::has_suite_admits_sy_and_still_refuses_ps` | AC-002. `SY-` admitted, `PS-` still refused, and a later widening to `PS-` fails by name rather than by a flood of missing-rule reports |
| Unit | `cargo test -p xtask` → `xtask/src/spec_trace.rs::tests::de_marked_sy_clause_reports_its_missing_rule_by_name` | AC-003. Check 4's message still carries the clause id and the rule name; a refactor that keeps the check and loses the diagnostic fails |
| Negative control (recorded, not committed) | Misspell one rule name in a de-marked `SY` clause → `cargo xtask spec-trace` → capture transcript → revert. The pattern is `…/gate-mounts-for-the-sync-suite/spec.md` AC-002's | AC-003. That a wrong version of **this PR's own deliverable** exists and the gate rejects it — the substitute obligation this story owes for adding no conformance rule |
| Unit (doc) | `cargo test -p happenstance-sync --doc` | AC-010(a). The `SyncPeer::push` doc-comment clarification compiles and its examples still run (`standards/rust/70-rustdoc-obligations.md`) |
| Review (DoD 7 pass) | `git diff main -- spec/SPECIFICATION.md`, read hunk by hunk | AC-005, AC-006, AC-007, AC-008, AC-011, NF-003. No normative sentence, no maturity marker, no clause id, no `Cases:` field; each repair hunk traceable to its ADR-0026 authorisation and carrying the mechanical test's answer |
| Review (record) | The implementation report's sweep hit list, per-id verdict table, per-drop table and citation "what it now says" record | AC-001, AC-004, AC-006, AC-009, AC-010, NF-006, NF-007. The things no command can check, each with the artefact that makes it re-derivable |
| Integration-scoped (ceiling) | `cargo xtask ci --fast` — `integration_scoped` (`.redkiln/config.yaml:55`); `project.md` DoD 1 | Initiative DoD 13's not-regressed obligation on this project's tree, and NF-004's `cargo package --list` assertions. The whole `cargo xtask ci` gate is the terminal project's, not this one's |
| KB gate | `redkiln validate --kb && redkiln doctor` | NF-005. No `.kb/` atom edited or added; accepted-decision immutability intact. Expected to be a no-op diff, and a non-empty one is a boundary breach |
| Ledger | `_ledger.md` with cited evidence per AC-### (`.redkiln/config.yaml:62-67`, `require_ledger: true`) | Project DoD 3 — *"a green suite proves something works, never that the criteria this project was written to satisfy are the things that work"* |

## Risks and coupling (PR-scoped)

| risk | coupling | mitigation in this PR |
| --- | --- | --- |
| **The marker drop lands without the predicate and proves nothing.** `schedules_new`'s only consumer is `xtask/src/spec_trace.rs:695`, guarded by `has_suite`, which admits no `SY` clause today. A PR that drops markers and leaves `has_suite` alone is green, looks complete, and has changed one rendered table cell | AC-002 → AC-004; the whole of the mount | Order fixed in the implementation notes (predicate first), and AC-003's transient-violation transcript is an artefact that cannot be produced without the predicate |
| **`has_suite` is unclaimed, so nobody else will do it.** HS-S0104 widens `RULE_FILES` and repairs check 4's *message*; grepping `…/gate-mounts-for-the-sync-suite/spec.md` for `has_suite` returns nothing | Silent overlap with HS-S0104 | Named as this story's mount in the *Integration contract*; if HS-S0104 landed it anyway, this story records that and verifies the `PS-` exclusion survived, rather than editing it twice |
| **The `PS-` temptation.** Adding `SY-` invites "and `PS-` while we are here". Doing so reports every `PS` rule as missing — the exact noise the original comment refuses (`xtask/src/spec_trace.rs:684-688`) — and the projection suite is `projection-store-freeze`'s (HS-P0010) | Cross-project | Pinned by `has_suite_admits_sy_and_still_refuses_ps`, which fails by name; and stated in the function's own rustdoc (AC-002) |
| **The audit's inputs arrive from six other slices and are filed under two story ids.** Three sibling specs call this story HS-S0112 and three call it HS-S0114 | AC-006, AC-010; every predecessor slice | EC-007: grep both ids, key on the slug, and record a *not handed forward* result rather than closing on an empty input |
| **SY-1/SY-6's verdict depends on a decision another story makes.** If ADR-0027 makes `EventGroup::guard` private, both clauses need a repair **in the same commit** (`_decomposition.md`, AC-A11); if it does not, editing them is unauthorised work on a `[FROZEN]` clause | AC-008; `adr-0027-merge-compensation-and-message-set` | AC-008 makes the verdict conditional on the field read at the merge tree, and EC-009 forbids guessing when the disposition is ambiguous |
| **The repair that is quietly an amendment.** The three easy ways to close this story — do nothing, delete the field, soften the MUST — are all green and all wrong | AC-005, AC-007, AC-011; project DoD 7 | The mechanical test answered per clause and written down (EC-001), and the anti-pattern-to-AC mapping in *Interaction quality* that adversarial review checks |
| **Regeneration forgotten.** Every `Rule:` edit changes a 79-character §7.2 cell; a `Rule:` edit committed without `--write` leaves `spec-trace` red and tempts a hand-edit of the generated region | AC-011, EC-005, NF-002; initiative DoD 13 | The write pass is its own tier row in the merge gate, and the second check-mode run is the evidence |
| **This story is last, so its budget is what the six ahead of it left.** The audit is the initiative's only proof that the specification is maintained against the code, and it is the easiest thing to close by asserting | AC-006; project AC-012, DoD 7 | The audit list is a **closed pre-commitment** written at discover, and AC-006 requires a verdict per id including the *keeps a live target* majority. A report with no such rows is a report that stopped looking |
| **`clause-arithmetic-and-deferral-renewals` merges immediately after and reads this story's output.** HS-S0113 owns every maturity marker and the exit arithmetic and takes this story as its hard predecessor | Slice-mate, same context | The marker boundary is stated three times (*Context pack* §9, *PR boundary*, AC-011); a marker moved here is a boundary breach visible in `git diff`, and the per-id verdict table is HS-S0113's input |
| **The predicate change can surface pre-existing red that this story neither caused nor owns** | EC-004; whichever story owes the missing rule | The finding is recorded and routed, never silenced; the `PS-` exclusion is the only permitted narrowing and it is pinned by a test |

## Dependencies

**Blocks on** — both declared in the story item's `depends_on` and in `…/_storymap.md`, *Slices*:

- **`headline-rules-and-mutant-registry`** (HS-S0105) — a `(new)` marker may come off only once the
  rule it schedules exists. This story consumes `ingest_never_rejects`,
  `compensation_is_atomic_with_the_losing_event` and `wire_condition_with_after_is_refused` as
  *existing*, and consumes two recorded findings from it: the `SyncPeer::push` rustdoc ambiguity and
  the `(new)`-marker handoff on SY-1/SY-2/SY-6 (`…/headline-rules-and-mutant-registry/spec.md`,
  AC-013).
- **`adr-0026-peer-ingest-and-transport`** (HS-S0098) — the authorisation. ADR-0026 pre-authorises
  the two families of repair by name, which is what makes project DoD 7 satisfiable by pointing at an
  authorisation rather than by arguing after the fact
  (`…/adr-0026-peer-ingest-and-transport/spec.md`, *Integration contract* → *Clause(s)*, and its
  AC-011).

**In practice also downstream of** — not declared edges, because the story map's merge order already
places this story seventh of seven and each of these lands before it; recorded so an implementer
knows where the audit's inputs come from: `memory-store-ingest-seam` and
`ingest-store-and-memory-peer-round-trip` (which remove SY-12's target and owe the
citation-invalidation list), `sync-testkit-crate-and-rule-registry`,
`gate-mounts-for-the-sync-suite` (which widens `RULE_FILES` and repairs check 4's message),
`adr-0027-merge-compensation-and-message-set` (which decides `EventGroup::guard`'s fate) and
`message-set-on-the-envelope`.

**Unlocks**

- **`clause-arithmetic-and-deferral-renewals`** (HS-S0113) — the slice-mate, merged immediately
  after. It owns every maturity marker, CF-38's named-experiment sweep and the ADR-0026 ∪ ADR-0027
  clause-range union, and it names this story as its hard predecessor (`…/_storymap.md`, *Merge
  order* 7): the arithmetic can only be right over clauses that are already true. Its input is this
  story's per-id verdict table.
- **Project DoD 7 and, through it, initiative DoD 12** — this story is the only one in the project
  that edits `spec/SPECIFICATION.md`, so it is the only one that can move DoD 7. It makes the clauses
  true and HS-S0113 makes the count true, in that order.

## Anchors (progressive disclosure)

Open these when the row says to, and never in bulk. Everything needed to *start* is above; these are
the load-bearing depth this spec deliberately did not paste.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | The mechanical test in its own words, the repair/gap taxonomy, and *The safe form for a discharged MUST* — the three-part form every repair in this story must take. Nothing else in the repository states it | Before writing the **first** repair, and again before answering the mechanical test for each clause | AC-006 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/frozen-clause-repairs/discover.md` | The pre-committed audit list *by id* (*Questions* 3), the HEAD sweep's ten hits with line numbers (*Signal Ledger* row 5), and the four named mutants this story must not become (*The wrong implementation*) | First, before touching `spec/SPECIFICATION.md` at all — it is the closed input set AC-006 is measured against | AC-001 |
| `xtask/src/spec_trace.rs` | The mount. `has_suite` at `:1735-1737`, check 4 at `:694-711` with the comment at `:684-688` that explains the exclusion being lifted, the marker logic at `:1608-1634` including the WF-12 trap, `RULE_FILES` at `:85-89`, and the `--write` equality at `:735-744` | Before the predicate change, and again before computing the drop set | AC-002 |
| `spec/SPECIFICATION.md` | The document under repair. SY-1 at `:5841-5867`, SY-6 at `:5973-6029`, SY-12 at `:6153-6207`, SY-9/SY-10's citations at `:6120-6121`, §5's test-file preamble at `:5775`, CF-4's saboteur bar at `:7208-7215`, CF-6 at `:7233-7249`, and the generated-region marker at `:8717` | Per clause, as each id's verdict is reached — never read end to end for this story | AC-006 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` | *Tension 4* (the audit as pre-commitment, and why the `(new)` drops are a repair DoD 7 must not be read as forbidding), **AC-A11** (a repair in the same commit), and the *Testing brief*'s Static tier naming `spec-trace` as AC-012's instrument | When a verdict feels discretionary, and when assembling the merge-gate evidence | AC-004 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` | AC-002 and AC-012 in the words this story traces to, DoD 7 verbatim, and the *Out of scope* stop condition — *"amending any `[FROZEN]` clause… is a new decision atom and a re-plan, not a line edit"* | Before recording any `gap` verdict, and at the DoD-7 review pass | AC-011 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0026-peer-ingest-and-transport/spec.md` | The authorisation this story cites rather than re-derives: its *Integration contract → Clause(s)* names the two repair families and this story as the performer, and its AC-011 is the obligation being discharged. The accepted atom `.kb/decisions/0026-*.md` **does not exist in the tree yet** — it arrives through `/redkiln:kb-ingest` when HS-S0098 merges, so this spec anchors the story artefact, which is real today | Before writing the first repair, to check the repair falls inside what was authorised (EC-003); and again at merge, to cite the landed atom's real path | AC-007 |
| `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` | The discipline for re-pointing a `file:line` into prose that moved — the one defect class no tool in this repository can see | Immediately before re-anchoring any citation, not after | AC-009 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The governing question — does the edit change what the document **asserts**, or only what it points at? — which separates every legal edit here from the amendment the project forbids. Applies to the `has_suite` rustdoc as much as to a clause | When an edit's legitimacy is in doubt, and when amending `has_suite`'s doc comment | AC-005 |
| `.kb/decisions/README.md` | The corpus's own statement of what an accepted decision protects and what it does not — the authority the playbook's mechanical test derives from, and the immutability rule NF-005 rests on | Once, before the first mechanical-test answer, if the playbook's framing is not already loaded | AC-006 |
| `crates/happenstance-sync/src/peer.rs` | The live target of SY-1 and SY-6: `EventGroup::guard`'s visibility at `:236-243`, whose disposition at the merge tree decides both verdicts. Also `SyncPeer::push`'s rustdoc at `:140-148`, the first handed-forward finding | At the start of AC-008 (read the field, record what is there), and again at AC-010(a) | AC-008 |
| `crates/happenstance-sync/src/ingest.rs` | SY-12's current target — *why `impl IngestStore for MemoryEventStore` cannot be written truthfully* at `:38-50` — the sentence earlier slices made false | When settling SY-12, to see exactly what stopped being true before choosing between a new target and a discharge | AC-007 |
| `crates/happenstance-sync/src/lib.rs` | The module prose SY-6 cites at `:100-104` and SY-9/SY-10 at `:57-63`, rewritten wholesale by this project. The referent that must be read, not assumed | While re-anchoring each of those three citations | AC-009 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/headline-rules-and-mutant-registry/spec.md` | The predecessor that landed the three rules and recorded two findings for this story: the `SyncPeer::push` rustdoc ambiguity and the marker handoff (its AC-013). It is the reason the `(new)` drops are possible at all | When collecting the handed-forward findings, and when the drop computation reports a rule as resolved | AC-010 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/ingest-store-and-memory-peer-round-trip/spec.md` | Owes this story the recorded list of `spec/SPECIFICATION.md` and test citations its diff invalidated (*Context pack* §11), and is half of what removes SY-12's target | At the start of AC-010(b), to obtain the input list before consuming it entry by entry | AC-010 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/gate-mounts-for-the-sync-suite/spec.md` | Widened `RULE_FILES` and repaired check 4's *message*, and — verified by grep — claims **nothing** about `has_suite`. Confirms the predicate is unclaimed and this story's to mount; also the source of AC-003's transient-violation pattern | Before editing `has_suite`, to re-verify the overlap has not changed since planning | AC-003 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/clause-arithmetic-and-deferral-renewals/spec.md` | The slice-mate that merges next and consumes this story's per-id verdict table; it owns every maturity marker and the clause arithmetic this story must not touch | When a repair looks as though it needs a marker to move — that is the boundary, and this is where the work goes | AC-011 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_storymap.md` | The slice row, the `depends_on` edges, and *Merge order* 7's reason this story is last: *"a repair can only name the rules and symbols that exist"* | If the merge order or an edge is ever in question | AC-004 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` | The signed-off no-surface determination (approved 2026-08-12, `design.capture` a declared skip) and the recorded reason the API-surface obligation is discharged in the briefs rather than there | Once, to confirm no composition obligation was skipped silently — the *Interaction quality* section rests on it | AC-011 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | Persona 4, the evaluator, at `:249-300` — including *"no external body to check 'DCB-compliant' against"*, which is why a stale clause costs more here than a stale comment costs anywhere else | When wording a repair, to keep the reader in view rather than the checker | AC-001 |
| `.bklg/from-contract-to-published-library/initiative.md` | DoD 12 (the clause ledger audited), DoD 13 (the gate green on the assembled whole) and the journey line at `:249-250` this story's persona comes from | Once, if the story's contribution to the initiative's own DoD is in question | AC-011 |
| `.redkiln/config.yaml` | `reachability_static` at `:48`, `integration_scoped` at `:55`, `require_ledger` at `:62-67` — the commands this story is measured by whether or not anyone types them, and the `support`-initiative routing EC-010 uses | When assembling the merge-gate evidence and filling `_ledger.md` | AC-011 |
| `RUNBOOK.md` | Phase 13 in full at `:4516-4620`, and the standing record-keeping lesson from phase 4 at `:334-336` — compute the arithmetic, do not assert it — which is why the verdict table exists at all | For orientation if the story's place in the plan is unclear; not needed to start | AC-006 |
| `standards/rust/70-rustdoc-obligations.md` | Governs the one `.rs` prose change this story makes — the `SyncPeer::push` clarification — and the `has_suite` rustdoc amendment | Before writing either doc comment | AC-010 |
| `crates/happenstance-testkit/src/registry.rs` | The reference shape for how rules are enumerated in one place, which is what makes "the rule exists" a computable claim rather than a reading | Only if the drop computation's notion of *a rule exists* needs checking against the event-store precedent | AC-004 |

## Clarifications resolved during spec

1. **The AC set is exactly the eleven the first pass enumerated** — AC-001 … AC-011, unchanged, none
   added and none dropped. `_ledger.md` carries the same eleven ids.
2. **`has_suite` is inside this story's boundary, and that is a widening the first pass justified
   rather than an inherited fact.** The *Executive summary*'s delta stands and was re-verified while
   writing this half: `schedules_new`'s only consumer is `xtask/src/spec_trace.rs:695`, guarded by
   `has_suite`, which is three `starts_with` calls at `:1735-1737` admitting `ES-`, `VT-` and `WF-`
   only. Without the predicate change, AC-004 is a repair no check can reject.
3. **`PS-` is excluded on purpose and pinned by a test, not by a comment.** The reason is the
   original comment's own (`:684-688`): the projection suite does not exist, so admitting `PS-`
   reports every `PS` rule as missing. `has_suite_admits_sy_and_still_refuses_ps` makes the exclusion
   fail by name if a later tidy adds it, which is the difference between a decision and a note.
4. **The `.kb/decisions/0026-*.md` atom does not exist in the tree today** — verified by listing
   `.kb/decisions/`, whose highest-numbered atom is `0029-msrv-raised-to-1-97-1.md` with no `0026`
   present. It arrives through `/redkiln:kb-ingest` when `adr-0026-peer-ingest-and-transport` merges,
   which is this story's declared `depends_on`. The *Anchors* table therefore points at that story's
   `spec.md`, which exists, and says so explicitly. No anchor in this spec names a file that is not
   on disk.
5. **The design's composition sections are vacuous, so *Interaction quality*'s composition family
   binds to the clause form instead.** `_design.md` records *"N/A — no user-facing surface"* under
   every composition heading, with the no-surface determination itself approved and `design.capture`
   a declared skip. Rather than skip the family, this spec binds it to the composition discipline
   that genuinely applies — the three-part clause form, its hierarchy, its density numbers and the
   four named anti-patterns — and maps each to an existing AC row. Nothing was invented and no
   invariant was silently dropped.
6. **The negative control is a *recorded transcript*, not a committed fixture.** A permanently
   misspelt rule name in the specification would fail the gate forever; the transient-violation
   pattern (`…/gate-mounts-for-the-sync-suite/spec.md`, AC-002) is demonstrate → capture → revert,
   with the two `xtask` unit tests carrying the permanent half. Both halves are required: the unit
   tests pin the predicate and the message, the transcript proves they are wired to the path the gate
   runs.
7. **The audit list's *keeps a live target* verdicts are deliverables, not omissions.** AC-006 states
   it explicitly because the cheapest way to close this story is a report that only mentions what
   changed — which is indistinguishable from a report by someone who only looked at what changed.
8. **`Cases:` fields and `spec/E2E-CASES.md` stay out**, per the *PR boundary*, and no AC covers
   them. A case that no longer holds is a finding routed to whoever owns the case, not an edit made
   here — the same rule EC-002 applies to rules.
9. **EC-010 was added for a condition no earlier section named**: the project's ceiling command
   (`cargo xtask ci --fast`) going red on something outside this diff. Because this story merges
   seventh of seven, it is the most likely place for an unrelated regression to surface, and the
   default of "fix it here" would silently widen a PR boundary the whole story is built around.
