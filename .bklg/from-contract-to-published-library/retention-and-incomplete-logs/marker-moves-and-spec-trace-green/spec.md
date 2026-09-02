---
item: HS-S0123
stage: spec
created: 2026-08-12T14:09:40.221Z
updated: 2026-08-12T14:09:40.221Z
template_sig: 87bbf1d0
rendered_sig: 3635c1a5
---

# Spec — Three markers leave, and the census is recomputed by a human

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD **12**, the clause-ledger audit (`:393-395`), DoD **15** (`:402-404`), exit criterion 5 (`:561-582`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — gate decision 4, the no-surface-change constraint (`:239-249`) |
| Project | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` — **AC-014** (`:248-251`), and the Definition of done line it discharges (`:272-273`) |
| This spec | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/marker-moves-and-spec-trace-green/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` — architecture *Gate mechanics AC-014 must satisfy* (`:468-496`), composition root **7** (`:143-146`), the seam map's `spec/SPECIFICATION.md` row (`:64`), **AC-A12** (`:535-536`); testing brief's AC-014 row (`:616`) and *Merge-gate commands* (`:620-650`) |
| Signed-off design | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` — **surfaces: N/A, approved 2026-08-12** (`:38-48`, `:86-95`). This story renders none and adds no `pub` item; the design's binding content here is its no-surface determination |
| This story's discover | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/marker-moves-and-spec-trace-green/discover.md` — the signal ledger (`:16-30`), the two questions deferred to spec (`:35`, `:36`), the **ES-38 scope decision at the plan gate** (`:39`, `:72-74`), and the named wrong implementations (`:45-54`) |
| Upstream decision | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/adr-0028-and-the-open-question-wave/spec.md:492` — ADR-0028 is *"the decision ES-39, CF-27 and SY-32 are each `[DEFERRED]` on, without which no marker can honestly move"* |
| Story map row | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md:77` (this story), `:146-149` (merge order inside the slice) |
| Roadmap pointer | `RUNBOOK.md:4626-4674` (phase 14 in full), `RUNBOOK.md:165` (its status row — **not edited here**; the runbook is `closeout-and-durable-audience`'s) |

## One-line PR slice

Move three maturity markers in `spec/SPECIFICATION.md` — **ES-39** and **CF-27** out of `[DEFERRED]`,
**ES-40**'s `[PROVISIONAL]` discharged or renewed against a named experiment — with replacement text taken
from ADR-0028 and written so that *settled* is legibly distinct from *implemented*; take `(new)` off exactly
the `Rule:` lines whose rules now resolve and off no others; regenerate §7.1/§7.2 with
`cargo xtask spec-trace --write`; **hand**-reconcile §1.3's prose census in the same commit; and leave
`cargo xtask spec-trace` and `cargo xtask ci --fast` green on the result.

## Executive summary

This PR is the specification's side of everything the previous nine stories built. It touches one file.

Three clauses have been waiting on an instrument that did not exist and an experiment nobody had run. Both
now exist: HS-S0114 built the retained-set instrument, HS-S0115 ran CF-27's experiment and committed the pass
list, HS-S0116 and HS-S0117 wrote the two owed rules, HS-S0121 landed ADR-0028, and HS-S0122 settled CF-27's
own rule either way. Each of those stories deliberately left `spec/SPECIFICATION.md` byte-identical
(`positions-are-not-reused-after-removal/spec.md:437`,
`retained-set-instrument-and-conformance-mount/spec.md:392`) except for the one generated §7.2 cell a written
rule forces. **Every marker move in this project is this story's, in one commit, on purpose** — because a
marker move committed without its tables, or tables regenerated without §1.3, is green in both wrong cases
until the *next* author's unrelated marker move fails for a reason that is not theirs.

The delta is four edits to one file, and the two that a tool cannot do are the story:

1. **Three marker replacements**, whose *text* is ADR-0028's and whose *placement* is this story's — the
   checker reads a marker positionally and CF-27's is currently mid-sentence, which is a live hazard the
   moment the replacement prose mentions another clause's marker (`xtask/src/spec_trace.rs:1441-1472`).
2. **`(new)` removal on exactly one `Rule:` line** — ES-40's — because that is the only non-frozen clause
   whose named rule now resolves *and* whose prefix `spec-trace` check 4 actually reads
   (`xtask/src/spec_trace.rs:1735-1737`). ES-39's `(new)` **stays**, and removing it is a hard gate failure.
   ES-38 is not touched at all: reading (b), decided by the repository owner at the `/redkiln:plan` discover
   gate on 2026-08-12 (`discover.md:39`, `:72`).
3. **`cargo xtask spec-trace --write`**, which regenerates §7.1 and §7.2 between the committed markers at
   `spec/SPECIFICATION.md:8507` and `:8753`. The implementer accepts whatever delta it produces; hand-editing
   inside that region is how the equality check stops meaning anything.
4. **§1.3's prose census, hand-computed**, at `spec/SPECIFICATION.md:219-222`. This is the one number in the
   document a human is required to produce, and `xtask/src/spec_trace.rs:39-54` states why in its own words:
   §7.1 and §7.2 come from the same `parse_clauses` output, so a parser that quietly stops recognising a
   clause form shifts the census and the table *together* and the equality check stays green. §1.3 is the
   only count a person computed by reading the document, which makes its agreement with the checker the best
   available evidence that the parser reads the document the way a reader does.

What this PR does **not** do: add or retire a conformance rule, touch any crate, stage any `.kb/` atom, edit
ES-38 or any other `[FROZEN]` clause, move a version, or record a primitive as chosen. That last one is the
sharp mutant and it is entirely a prose failure — see the context pack.

## Context pack

Everything below is a decision this story must honour. Nothing here is a reading list; the deeper artifacts
sit behind the anchors table the second pass appends.

**Decision — ADR-0028 supplies the marker *text*; this story supplies the *mechanics* and the placement.**
ADR-0028 is the decision all three clauses are deferred *on*
(`adr-0028-and-the-open-question-wave/spec.md:492`). This story does not re-derive the answer, soften it, or
extend it. If ADR-0028 as merged does not say plainly enough what ES-39 is now, that is a blocker to raise
against HS-S0121, not a gap for the marker author to fill — an ambiguous ADR read by a marker author becomes
a specification clause nobody decided.

**Decision — the replacement text must distinguish *settled* from *implemented*, and this is the story's
central content obligation.** ES-39 asks whether a store can declare what it does not hold. Its question is
answered — in the negative, inside a stated constraint — while its *primitive* remains unchosen, because
every candidate is a port-surface change and gate decision 4 routes those to an AC-012 escalation rather than
to a diff (`_decomposition.md:363-379`, `../_decomposition.md:239-249`). So the marker that replaces
`[DEFERRED]` must record that no primitive was adopted and why, and must not read as though one had been.
Nothing in the toolchain can tell those two apart: `spec-trace` verifies that markers exist, that deferrals
and provisional clauses name what they are waiting on, that rules resolve and are claimed, and that the
tables agree — it cannot distinguish *"no primitive was adopted, and here is what a reader is therefore on
its own against"* from *"the retained-ranges primitive is adopted"*. The second is a documented promise no
code keeps: a breach of the no-surface-change constraint achieved entirely in prose, with a green gate. The
discover names it `SettledByAssertion` and calls it the sharper mutant (`discover.md:47`); it is the one this
spec is written against.

**Decision — ES-39's `(new)` stays, and removing it is a hard gate failure rather than a tidy-up.** Check 4
resolves every rule name a clause cites, and exempts a clause only while its `Rule:` line carries `(new)` or
`†` (`xtask/src/spec_trace.rs:680-711`, `:1626-1631`). ES-39 names
`a_store_reports_the_history_it_does_not_hold`, which is *"writable only once the primitive is chosen"*
(`spec/SPECIFICATION.md:4344-4345`) and no primitive was chosen, so the rule does not exist and will not
exist in this initiative. The exemption is independent of the maturity marker, so ES-39 can leave `[DEFERRED]`
while keeping `(new)` — that combination is not an inconsistency, it is the exact shape of *"the question is
answered, the rule is not writable"*, and the marker text should say so.

**Decision — ES-38 is not touched, at all.** ES-38 is `[FROZEN]` and carries
`positions_are_not_reused_after_removal` **(new)** on its `Rule:` line (`spec/SPECIFICATION.md:4308-4309`).
The storymap's *"the `(new)` markers come off the `Rule:` lines whose rules now exist"* (`_storymap.md:77`)
is honoured for every non-frozen clause and treated as a **preference** where the clause is frozen. Three
readings were on the table and the repository owner picked (b) at the `/redkiln:plan` discover gate on
2026-08-12: check 4 *permits* a `(new)` annotation rather than requiring its removal, so leaving ES-38 alone
is gate-safe and no ADR is owed (`discover.md:39`, `:72-74`). Readings (a) — the annotation is bookkeeping,
not clause content — and (c) — it is a frozen-clause edit needing ADR authorisation — are the alternatives
that lost, recorded here so the choice is not re-litigated in review. **The residual is stated rather than
hidden:** leaving `(new)` on ES-38 keeps that clause exempt from check 4 forever, so the checker never
verifies that `positions_are_not_reused_after_removal` resolves *via ES-38*. What does verify it is §7.2's
ES-38 cell losing its `†` (`spec/SPECIFICATION.md:8620`), which `rule_cell` daggers by **resolution** and not
by the clause's own wording (`xtask/src/spec_trace.rs:1147-1183`), plus check 6's ownership sweep
(`:725-727`).

**Decision — CF-27's replacement marker moves to declaration position, on its own line.** `maturity_of`
prefers a marker in *declaration position* — at the start or end of a line — and only falls back to
first-anywhere-in-the-body when a clause declares none (`xtask/src/spec_trace.rs:1441-1472`). CF-27's marker
today sits mid-sentence — line `spec/SPECIFICATION.md:8036` begins *"reports that it does.
`[DEFERRED — the experiment is building it as a decorator"* — so CF-27 is currently classified by the
fallback. That is harmless today because its body mentions no other bracketed marker. It stops being harmless
the moment the replacement prose says anything like *"ES-39 remains `[PROVISIONAL — …]`"*: the fallback takes
the **earliest** marker anywhere in the body and would silently classify CF-27 as whatever the sentence
quoted. Putting the replacement on its own line makes the `declared` branch win and makes later prose
inert. ES-39's and ES-40's markers are already in declaration position
(`spec/SPECIFICATION.md:4327`, `:4359`) and must stay there.

**Decision — a renewed `[PROVISIONAL]` names a real experiment that exists in the tree.** ES-40's marker
names its own falsifier: *"Falsified — or given its assertion — by the suffix store of CF-27, which is why
this clause and that instrument are one decision and not two"* (`spec/SPECIFICATION.md:4359`). HS-S0117 gave
it its assertion, so the expected branch is **discharge**. If it is renewed instead, check 2 requires a
falsifier of at least twelve characters (`xtask/src/spec_trace.rs:659-668`) and CF-38 makes an *unnamed*
one a gate failure by construction — but twelve characters of vagueness would pass, so the bar this spec
sets is higher and is the same bar project AC-010 is held to: a named path that **exists**, under
`experiments/`, which holds three real directories today (`experiments/position-visibility`,
`experiments/rustc-ice-gat-foreign-trait`, `experiments/wire-format`). `RenewalWithoutAnExperiment` is the
discover's named wrong implementation here (`discover.md:51`).

**Decision — the generated region is regenerated, never edited.** §7.1 and §7.2 live between
`<!-- BEGIN GENERATED: spec-trace §7.1–§7.2 -->` (`spec/SPECIFICATION.md:8507`) and `<!-- END GENERATED -->`
(`:8753`), and the specification states in its own §7 preamble that they are generated (`:8435`). Check 9
compares the committed region against the freshly computed one on every run (`xtask/src/spec_trace.rs:741-742`).
The implementer runs `cargo xtask spec-trace --write` (`xtask/src/main.rs:671-674`) and commits whatever it
produces. **Do not predict the delta and do not trim it.** Two of the sibling specs describe their own change
as *"exactly one §7.2 cell"*; CF-27's `Rule:` field wraps across five lines and names all three rules
(`spec/SPECIFICATION.md:8042-8047`), so its row at `:8738` loses daggers too, and the row count that actually
changes here is whatever the tool says.

**Decision — §1.3 is computed by a person and is the deliverable most likely to be skipped.** `--write` does
not touch it, deliberately (`xtask/src/spec_trace.rs:39-57`). Check 8 verifies four separate things about the
sentence at `spec/SPECIFICATION.md:219-222`: the stated total against the computed total, **each** of the
four maturity counts, and the sentence's own subtraction — total minus `[NON-NORMATIVE]` equals the "are
normative" figure (`xtask/src/spec_trace.rs:459-508`). The last is checked separately because a sentence can
disagree with itself while all three figures still match the checker. The anchor phrase *"As assembled, this
document carries"* is matched literally (`:168`), so the sentence's opening words are not free to be
reworded.

**The arithmetic, stated so it is checked rather than guessed.** Today: **200** clause IDs, **198**
normative, **139** `[FROZEN]`, **49** `[PROVISIONAL]`, **10** `[DEFERRED]`, **2** `[NON-NORMATIVE]`
(`spec/SPECIFICATION.md:219-222`, §7.1 at `:8513-8519`). This story adds and removes no clause and demotes
nothing to prose, so **200 and 198 and 2 are invariants of this PR** and only the FROZEN / PROVISIONAL /
DEFERRED triple re-balances — it must still sum to 198. `[DEFERRED]` goes **10 → 8** because ES-39 and CF-27
both leave it; where those two land and whether ES-40 leaves `[PROVISIONAL]` are ADR-0028's to say. Worked
example, not a prediction: if all three settle to `[FROZEN]`, the sentence reads 142 / 48 / 8 / 2. §7.1's per-
section rows move with them — the `ES` row's `[DEFERRED]` column goes 1 → 0 and the `CF` row's 2 → 1 — but
those are generated and are not hand-edited.

**Decision — no `.kb/` file, no crate, no version, no runbook row.** Everything this story writes is
specification prose and one regenerated region. ADR-0028 is cited **by id** and not restated
(`adr-0028-and-the-open-question-wave/spec.md:454`, `:564` — that story deliberately preferred id-plus-line
citations *because* this story edits the file underneath them). Phase 14's `RUNBOOK.md:165` status row and the
§6.5 portfolio table's completeness row are adjacent and out of scope: the CF-25/CF-26 residual exposure is
recorded open by ADR-0028 (`_decomposition.md:205-213`), and nothing in `spec-trace` reads the portfolio
table, so a green gate here is not evidence about it either way.

**Decision — every citation this story writes must resolve.** Check 7 resolves every `file:line` citation in
the document against a file that exists and is long enough (`xtask/src/spec_trace.rs:729-736`). Marker text
is prose, and prose in this document carries citations. Two live risks: a citation into a crate whose line
numbers moved when HS-S0117 added ES-40's rustdoc, and a citation into `spec/SPECIFICATION.md` itself, whose
line numbers this very PR shifts. Prefer clause **ids** over line numbers in new marker text; where both
serve, write both.

**The persona-journey slice.** The reader is the adapter author — and, at publish, the release auditor — who
reaches for the specification to find out what is settled. Before this PR the document says three of its own
questions are open and points at an instrument that does not exist. After it, the document is true on the day
it is read: the completeness instrument exists and CF-27 says so, the vacuous pass is asserted by a rule and
ES-40 says so, and ES-39 says the question is answered and the primitive is deliberately not shipped, naming
what a reader is therefore on its own against. That last sentence is the whole value: a clause that records
an honest "no" is worth more than a clause that has quietly been open for two phases, and it is the only
form of "no" this initiative's exit criteria accept.

## Integration contract

- **Archetype**: `capability` — a slice through the only stack this deliverable has: authored clause prose,
  the generated tables derived from it, the hand census cross-checking the parser, and the gate step that
  fails when any of the three disagree.
- **Slice / milestone**: `clause-exit-and-surface-record`. Slice-mates, implemented in the same context and
  mounted as one integrated surface: `cf-27-rule-or-recorded-refusal` (HS-S0122), which merges **first**
  because a marker can only move once the rule it names exists or is recorded as never-to-exist; and
  `surface-diff-and-the-ac-012-escalation` (HS-S0124), which merges **last** and reads this story's result as
  part of the assembled tree (`_storymap.md:146-149`).
- **Mount point**: **`spec/SPECIFICATION.md`** — composition root **7** of the architecture brief
  (`_decomposition.md:143-146`). Three named regions, and all three are load-bearing: the clause bodies
  **ES-39** (`:4325-4349`), **ES-40** (`:4351-4379`) and **CF-27** (`:8034-8060`); the generated region
  `<!-- BEGIN GENERATED: spec-trace §7.1–§7.2 -->` … `<!-- END GENERATED -->` (`:8507-8753`); and §1.3's
  census sentence (`:219-222`). This is a render path in the literal sense — `xtask/src/spec_trace.rs` reads
  the clause bodies and *renders* the middle region from them — which is why an edit to one region without
  the other two is the defect this story exists to avoid.
- **Wires into**: `xtask/src/spec_trace.rs` — the nine checks (`:650-742`), `maturity_of` and `falsifier_of`
  (`:1441-1487`), `rules_of`'s `schedules_new` (`:1626-1631`), `has_suite` (`:1735-1737`), `rule_cell`
  (`:1147-1183`) and `check_stated_census` (`:459-508`); `xtask/src/main.rs:671-674` for the two modes;
  `crates/happenstance-testkit/src/suite.rs` and `crates/happenstance-testkit/src/registry.rs` as the sources
  check 4 and check 6 resolve rule names against — **read, never written, by this story**; ADR-0028 under
  `.kb/decisions/`, read for the text and cited by id; and the three gate commands in
  `.redkiln/config.yaml:40`, `:48`, `:55`.
- **Renders surfaces**: **none.** `_design.md` records this project as having no user-facing surface, signed
  off 2026-08-12 (`_design.md:38-48`, `:86-95`), and this story adds no `pub` item of any kind. The
  `## Items` block is `N/A`, so there is no item id to claim.
- **Public items**: none. No crate is touched; no signature, rustdoc line or feature gate changes. This is
  the story in the project that most obviously cannot breach AC-011 by diff — and the one that most easily
  breaches it by prose, which is why the context pack's *settled versus implemented* decision is an
  acceptance criterion below and not a note.
- **Conformance rule(s)**: **none added, none retired, none renamed.** This story's behaviour is not
  adapter-observable and does not need to be: it changes the record an adapter author reads, not the contract
  an adapter meets. Its executable observers are `cargo xtask spec-trace`'s checks 1, 2, 4, 6, 7, 8 and 9,
  and `cargo xtask lints`' *"no retired rule is still live"* step (`xtask/src/main.rs:342`), which is here
  precisely so *"this project retires nothing"* is checked rather than assumed (`_decomposition.md:488-490`).
- **Clause(s)**: **ES-39** and **CF-27** — marker **replaced**, leaving `[DEFERRED]`; **ES-40** — marker
  discharged or renewed, and `(new)` removed from its `Rule:` line. All three are amended under the authority
  of **ADR-0028**, which is what a non-frozen marker move requires. **ES-38** (`[FROZEN]`) is **not** touched,
  per the discover gate decision, so no frozen clause is edited and no further ADR is owed. **CF-38** is
  *observed* — it is the clause that makes an empty falsifier a build failure — and not amended.
- **Advances DoD scenario**: initiative DoD **12** — *"A run over the specification reports every clause's
  maturity, and no clause is provisional with an empty falsifier; the count in the report matches
  `spec/SPECIFICATION.md`'s own stated figure"* (`.bklg/from-contract-to-published-library/initiative.md:393-395`).
  This story is the last thing standing between that audit and green, and it is the *only* story in the
  project that touches the stated figure. It also completes the record half of DoD **15** (`:402-404`): the
  answer exists on disk in `.kb/` after HS-S0121, and after this PR the specification agrees with it.

## PR boundary

```
spec/SPECIFICATION.md
.bklg/from-contract-to-published-library/retention-and-incomplete-logs/marker-moves-and-spec-trace-green/**
```

**In this PR**

- **ES-39's marker**, replaced in place at `spec/SPECIFICATION.md:4327-4333`, in declaration position, with
  ADR-0028's answer and an explicit statement that no primitive was adopted.
- **CF-27's marker**, replaced at `:8036-8041` and **moved onto its own line** so the checker classifies it
  from the declaration rather than the fallback.
- **ES-40's marker** at `:4359`, discharged — or renewed against a named `experiments/` path that exists —
  and the `(new)` removed from its `Rule:` line at `:4368-4371`.
- **CF-27's `Rule:` line** at `:8042-8047`: `(new;` disposed of according to which branch HS-S0122 took, and
  rewritten as a whole sentence rather than by deleting three characters.
- Any surrounding clause prose whose *truth* the moves change — for example ES-39's parting complaint that
  *"this is not a ledger row anywhere and needs one"* (`:4331-4333`), which this project's existence answers.
- **`cargo xtask spec-trace --write`**'s output inside `:8507-8753`, committed exactly as produced.
- **§1.3's census sentence** at `:219-222`, hand-computed, with the four figures and the subtraction that
  relates them all consistent.
- This story's own backlog folder — this spec, its `_ledger.md`, its implementation report.

**Explicitly not in this PR**

- **No edit to ES-38, or to any other `[FROZEN]` clause.** Reading (b), decided at the discover gate
  (`discover.md:39`). Its `(new)` stays and the residual is recorded, not removed.
- **No hand edit inside `<!-- BEGIN GENERATED -->` … `<!-- END GENERATED -->`.** If the region looks wrong,
  the clause body above it is wrong; fix that and re-run `--write`.
- **No generation of §1.3.** Moving that sentence inside the generated markers destroys the property it
  exists to prove (`xtask/src/spec_trace.rs:39-57`), and it will be the next contributor's first instinct.
- **No crate change of any kind** — no rule body, no registry line, no mutant, no `CHANGELOG.md` entry, no
  `Cargo.toml`, no version bump. A `CHANGELOG.md` entry here would be a second, drifting record of a rule
  another story already announced (CF-29's lint is satisfied by HS-S0116 and HS-S0117, not by this PR).
- **No `.kb/` file and no `.kb/_intake/` staging.** ADR-0028 is HS-S0121's and is cited by id.
- **No `RUNBOOK.md` edit**, including phase 14's status row at `:165` — the runbook is
  `closeout-and-durable-audience`'s.
- **No §6.5 portfolio-table edit and no CF-25/CF-26 marker move.** The completeness axis's residual exposure
  is recorded open by ADR-0028; `spec-trace` does not read that table, so a green gate says nothing about it.
- **No new clause, no retired clause, no demotion to `[NON-NORMATIVE]`** — each would move the total or the
  normative figure, which this story holds invariant.
- **No rule retired or renamed**, which is what makes `cargo xtask lints`' retired-rules step a real check
  here rather than a formality.

**Merge DoD (one line)** — `cargo xtask spec-trace` green on all nine checks with ES-39 and CF-27 out of
`[DEFERRED]` and §1.3 agreeing with the checker, `cargo xtask affected --base main`
(`.redkiln/config.yaml:40`) green at the story grain, `cargo xtask ci --fast` (`:55`) green at the integration
grain, and `git diff --name-only` listing nothing outside the two globs above.

## Behavior and interfaces

The *shape* is binding everywhere. Marker **text** is ADR-0028's and is quoted or paraphrased faithfully;
marker **placement**, the `(new)` dispositions, the regeneration and the census are binding as written here.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **ES-39 leaves `[DEFERRED]`** | The marker at `spec/SPECIFICATION.md:4327-4333` is replaced in declaration position with ADR-0028's answer. **Binding: it records that the question is settled and that no primitive was adopted**, naming what a reader is on its own against. It must not describe incompleteness as a boundary position — a floor is `earliest_position()`, and the clause's own argument is that a regulated purge is *scattered, not a prefix* (`:4336-4342`). If the replacement is `[PROVISIONAL]`, check 2 wants ≥12 characters of falsifier; if `[FROZEN]`, the existing `Rejects:` at `:4347-4349` already satisfies check 3 and is not edited | `spec/SPECIFICATION.md:4325-4349`; `xtask/src/spec_trace.rs:650-678`; ADR-0028 (`.kb/decisions/`, by id); `_decomposition.md:363-379` |
| **ES-39's `Rule:` line keeps `(new)`** | `a_store_reports_the_history_it_does_not_hold` is *"writable only once the primitive is chosen"* and no primitive was chosen. **Binding: do not remove it.** ES-39 is an `ES-` clause, so `has_suite` is true and check 4 applies; without the `(new)` exemption the checker resolves the name against `suite.rs` and fails hard. A marker leaving `[DEFERRED]` while its rule stays `(new)` is not an inconsistency — it is the shape of *answered but not implementable*, and the marker text should say so | `spec/SPECIFICATION.md:4344-4345`; `xtask/src/spec_trace.rs:680-711`, `:1626-1631`, `:1735-1737` |
| **CF-27 leaves `[DEFERRED]`, and its marker moves onto its own line** | The deferral's stated condition — build the decorator, run the full suite, record the list of rules that pass — was discharged by evidence in HS-S0114/HS-S0115. **Binding: the replacement occupies declaration position** (line-initial or line-final), because `maturity_of` falls back to first-marker-anywhere-in-the-body when a clause declares none, and any replacement prose quoting another clause's marker would then classify CF-27 by that quote | `spec/SPECIFICATION.md:8034-8041`; `xtask/src/spec_trace.rs:1441-1472`; `_storymap.md:69` (the pass list) |
| **CF-27's `Rule:` line, branched** | `Rule:` names `suffix_store_is_distinguishable_from_a_young_store` **(new;** at `:8042`. If HS-S0122 wrote the rule, the `(new;` clause is rewritten out as a whole sentence; if HS-S0122 recorded the refusal, `(new;` **stays** and the clause text records why the rule cannot exist. **Note the asymmetry: `has_suite` is false for `CF-` clauses, so check 4 never reads this line either way** — the honesty obligation here has no gate behind it, and the only mechanical signal is §7.2's CF-27 cell at `:8738` | `spec/SPECIFICATION.md:8042-8048`, `:8738`; `xtask/src/spec_trace.rs:1735-1737`, `:1147-1183`; `cf-27-rule-or-recorded-refusal/spec.md` |
| **ES-40's `[PROVISIONAL]` is discharged, or renewed against an experiment that exists** | The marker at `:4359` names CF-27's suffix store as its falsifier and HS-S0117 gave the clause its assertion, so **discharge is the expected branch**. A renewal must name a real path under `experiments/` — check 2's twelve-character floor is a floor, not this story's bar, and CF-38 is the clause that makes an unnamed falsifier a build failure | `spec/SPECIFICATION.md:4351-4379`; `xtask/src/spec_trace.rs:659-668`; `experiments/`; `discover.md:36`, `:51` |
| **ES-40's `(new)` comes off** | `condition_over_removed_history_does_not_reject` now resolves in `crates/happenstance-testkit/src/suite.rs` (HS-S0117). ES-40 is non-frozen and is an `ES-` clause, so this is **the one `Rule:` line this story reconciles**, and after the removal check 4 verifies the name for real instead of skipping the clause | `spec/SPECIFICATION.md:4368-4371`; `condition-over-removed-history-does-not-reject/spec.md:219`; `xtask/src/spec_trace.rs:680-711` |
| **ES-38 is byte-identical** | `[FROZEN]`, `(new)` retained, no edit. Reading (b) from the discover gate, 2026-08-12. The residual — check 4 never validating that rule name via ES-38 — is stated in the ledger rather than fixed, and §7.2's ES-38 cell losing its `†` at `:8620` is what actually reports the rule exists | `spec/SPECIFICATION.md:4299-4323`, `:8620`; `discover.md:39`, `:72-74`; `xtask/src/spec_trace.rs:1147-1183` |
| **§7.1/§7.2 regenerated, not written** | `cargo xtask spec-trace --write` rewrites everything between `:8507` and `:8753`; bare `cargo xtask spec-trace` checks it. **Binding: commit the output verbatim and do not predict the delta.** CF-27's `Rule:` field wraps across `:8042-8047` and names all three rules, so its row changes as well as ES-39's and ES-40's | `spec/SPECIFICATION.md:8435`, `:8507-8753`; `xtask/src/main.rs:671-674`; `xtask/src/spec_trace.rs:741-742` |
| **§1.3's census is hand-computed** | Four figures plus one subtraction at `:219-222`. **Invariants of this PR: total = 200, normative = 198, `[NON-NORMATIVE]` = 2.** `[DEFERRED]` goes 10 → 8; the FROZEN/PROVISIONAL/DEFERRED triple re-balances and must sum to 198. The opening words *"As assembled, this document carries"* are matched literally and are not free to be reworded | `spec/SPECIFICATION.md:219-222`; `xtask/src/spec_trace.rs:459-508`, `:158`, `:168`, `:39-57` |
| **One commit, in order** | Edit the three clause bodies → `cargo xtask spec-trace --write` → hand-reconcile §1.3 → `cargo xtask spec-trace`. **Binding: one commit.** Markers without tables, or tables without §1.3, are green *today* and leave the next author's unrelated marker move failing for a reason that is not theirs — the `MarkerFlip` mutant the discover names | `discover.md:47`; `_decomposition.md:616`; `xtask/src/spec_trace.rs:39-57` |
| **Citations resolve after the shift** | Check 7 resolves every `file:line` citation in the document. This PR moves line numbers inside `spec/SPECIFICATION.md` itself, and HS-S0117's rustdoc may have moved `append.rs`'s. **Prefer clause ids in new marker text; where both serve, write both** | `xtask/src/spec_trace.rs:729-736`; `adr-0028-and-the-open-question-wave/spec.md:390`, `:564` |
| **Nothing is retired** | No rule is removed from a clause. `cargo xtask lints`' retired-rules step is a real check here rather than a formality, and it is the reason the *"should retire nothing"* note is verified instead of assumed | `xtask/src/main.rs:342`; `_decomposition.md:488-490`; `discover.md:52` |
| **The gates** | Story grain: `cargo xtask affected --base main` — it runs the five file-reading lints and `spec-trace` unconditionally, which is the whole reason a story that maps to no package is still gated on something. Integration grain: `cargo xtask lints && cargo xtask spec-trace`, then `cargo xtask ci --fast`. `cargo xtask ci` (the whole gate) is HS-P0019's and is a local sanity check here at most | `.redkiln/config.yaml:36-40`, `:48`, `:55`, `:60`; `_decomposition.md:620-650` |

## Data and migrations

**N/A — no persisted data, no schema, no migration.** This story adds no type, no column, no serialised
form and no on-disk format; `happenstance-core`'s wire format and `ProjectionStore`'s checkpoint records are
untouched, and no crate is compiled differently after this PR than before it.

The one thing in scope that *looks* like data is the generated region at `spec/SPECIFICATION.md:8507-8753`,
and it is deliberately **derived**, not stored: `cargo xtask spec-trace --write` recomputes it in full from
`parse_clauses`' output on every run, and check 9 compares the committed bytes against a fresh computation
(`xtask/src/spec_trace.rs:741-742`). So the correct operation on it is regeneration, never migration — there
is no prior state to carry forward and no compatibility window to honour. §1.3's census at `:219-222` is the
mirror image: a hand-maintained figure that is deliberately *not* derived, for the reason recorded at
`xtask/src/spec_trace.rs:39-57`, and it is reconciled by a person in the same commit.

## Acceptance criteria

The persona throughout is the one the initiative names in *Learn when you are finished* and *Decide in one
sitting* (`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`): the
**adapter author** who opens `spec/SPECIFICATION.md` to find out what is settled before writing a line, and
the **evaluator** who reads the same document in a bounded sitting and decides adopt or decline. Both are
served by the same property — *the document is true on the day it is read* — and both are harmed in the same
way by a marker that says a question is open when it is answered, or says a primitive was chosen when none
was.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author reading ES-39 to learn whether a store may forget history and still be conformant, **WHEN** they reach the clause's maturity marker at `spec/SPECIFICATION.md:4327-4333`, **THEN** it no longer reads `[DEFERRED]`; the replacement carries ADR-0028's answer in declaration position, states in terms that **no primitive was adopted**, names what a reader is therefore on its own against, and describes incompleteness in a way that does **not** imply a boundary position — a floor is `earliest_position()` and the clause's own argument is that a regulated purge is scattered, not a prefix (`:4336-4342`) | `cargo xtask spec-trace` checks 1–3 (`xtask/src/spec_trace.rs:650-678`) for marker presence and well-formedness; the prose obligation by content review against the `SettledByAssertion` mutant named at `discover.md:47`, corroborated by `git diff --name-only` showing no change to `crates/happenstance-core/src/store.rs` or `crates/happenstance-core/src/append.rs` — a marker claiming a primitive with no port change behind it is the failure |
| AC-002 | **GIVEN** the same reader continuing to ES-39's `Rule:` line to find the rule that would check the clause, **WHEN** they read `a_store_reports_the_history_it_does_not_hold` at `spec/SPECIFICATION.md:4344-4345`, **THEN** its `(new)` annotation is still there and the marker text above explains why — the question is answered and the rule is not writable, because writing it needs a primitive nobody adopted; the reader is not left inferring that a missing rule means a forgotten one | `cargo xtask spec-trace` check 4 (`xtask/src/spec_trace.rs:680-711`) stays green **because** the `schedules_new` exemption holds (`:1626-1631`); falsified deliberately once — the implementer removes `(new)` locally, records that check 4 then fails naming the unresolvable rule, and reverts. Evidence for this row is that recorded failure, not the passing run |
| AC-003 | **GIVEN** an evaluator scanning §7.1's maturity census to judge how much of the contract is still moving, **WHEN** CF-27 is classified, **THEN** it is classified from a marker the clause *declares* — line-initial or line-final at `spec/SPECIFICATION.md:8036` — and no longer from `maturity_of`'s first-marker-anywhere fallback, so that replacement prose quoting another clause's marker cannot silently reclassify it | `cargo xtask spec-trace` check 1 plus `maturity_of`'s declared branch (`xtask/src/spec_trace.rs:1441-1472`); falsified deliberately once — the implementer adds a temporary sentence into CF-27's body quoting another clause's bracketed marker, confirms §7.1/§7.2 classification is **unchanged**, and reverts. Under the old placement that same sentence moves the row |
| AC-004 | **GIVEN** an adapter author asking whether a suffix store is distinguishable from a young one, **WHEN** they read CF-27's `Rule:` line at `spec/SPECIFICATION.md:8042-8047`, **THEN** it matches the branch HS-S0122 actually took — the `(new;` clause rewritten out as a whole sentence if the rule was written, or retained with the clause text recording in writing why the rule cannot exist if it was refused — and in neither case is it left as three deleted characters inside a sentence that no longer parses as English | `cargo xtask spec-trace` check 6's ownership sweep (`xtask/src/spec_trace.rs:725-727`) and §7.2's CF-27 row at `spec/SPECIFICATION.md:8738`, whose dagger is set by **resolution** rather than by the clause's wording (`rule_cell`, `:1147-1183`); plus a read-back against `cf-27-rule-or-recorded-refusal/spec.md`'s recorded branch. Note the asymmetry stated in the behaviour table: `has_suite` is false for `CF-` clauses (`:1735-1737`), so check 4 never reads this line — the honesty obligation here is carried by review |
| AC-005 | **GIVEN** an evaluator deciding in one sitting whether a `[PROVISIONAL]` clause is a real hedge or a parked one, **WHEN** they read ES-40 at `spec/SPECIFICATION.md:4351-4379`, **THEN** either the marker is **discharged** — the suffix store gave the clause its assertion in HS-S0117, which is the expected branch — or it is **renewed naming an experiment that exists on disk** under `experiments/`, in the shape of `experiments/position-visibility/README.md`; a renewal reading *pending further work* is a failure of this row even though it clears check 2's twelve-character floor | `cargo xtask spec-trace` check 2 (`xtask/src/spec_trace.rs:659-668`) for the floor; on the renewal branch, the named path is confirmed to exist before the commit and cited in the implementation report. `RenewalWithoutAnExperiment` (`discover.md:51`) is the wrong implementation this row rejects |
| AC-006 | **GIVEN** an adapter author who wants to run the rule ES-40 names, **WHEN** they read its `Rule:` line at `spec/SPECIFICATION.md:4368-4371`, **THEN** `condition_over_removed_history_does_not_reject` no longer carries `(new)`, because HS-S0117 wrote it — so the checker resolves the name for real instead of skipping the clause, and the author can run it by name | `cargo xtask spec-trace` check 4 (`xtask/src/spec_trace.rs:680-711`) now **reads** this clause and resolves the name against `crates/happenstance-testkit/src/suite.rs`; the implementer cites the rule's actual definition line in `suite.rs` as evidence, not the green run alone |
| AC-007 | **GIVEN** the repository owner reviewing this diff for the one thing `CLAUDE.md` forbids without an ADR, **WHEN** they read the hunks, **THEN** ES-38 (`spec/SPECIFICATION.md:4299-4323`) is byte-identical — `(new)` retained, reading (b) from the discover gate — no other `[FROZEN]` clause is touched, and no rule is retired or renamed anywhere in the tree | `git diff -U0 spec/SPECIFICATION.md` shows no hunk inside `:4299-4323`; `cargo xtask lints`' *no retired rule is still live* step (`xtask/src/main.rs:342`) green; §7.2's ES-38 cell at `spec/SPECIFICATION.md:8620` losing its `†` is the positive signal that `positions_are_not_reused_after_removal` resolves, since check 4 never reads ES-38's own line |
| AC-008 | **GIVEN** the next contributor moving an unrelated marker three months from now, **WHEN** they run `cargo xtask spec-trace` on a tree containing this commit, **THEN** it is green on the first run: §7.1/§7.2 between `spec/SPECIFICATION.md:8507` and `:8753` are exactly what `cargo xtask spec-trace --write` produced from this commit's clause bodies, committed verbatim with no hand edit inside the markers and no trimming of rows the author did not expect, and the marker moves, the regenerated region and the census reconciliation are **one commit** | bare `cargo xtask spec-trace` check 9 (`xtask/src/spec_trace.rs:741-742`) green; idempotence confirmed by re-running `cargo xtask spec-trace --write` (`xtask/src/main.rs:671-674`) after the commit and observing an empty `git diff`; `git log --oneline -1 --stat` showing the three regions in one commit. This is the row that rejects `MarkerFlip` (`discover.md:47`) |
| AC-009 | **GIVEN** the release auditor running the initiative's DoD 12 audit (`.bklg/from-contract-to-published-library/initiative.md:393-395`) — every clause's maturity reported, no provisional clause with an empty falsifier, and the report's count matching the document's own stated figure — **WHEN** they run the gate on this tree, **THEN** §1.3's sentence at `spec/SPECIFICATION.md:219-222` was recomputed **by a person** and agrees with the checker on all four maturity counts and on its own subtraction, holding total = 200, normative = 198 and `[NON-NORMATIVE]` = 2 invariant while `[DEFERRED]` goes 10 → 8 and the FROZEN/PROVISIONAL/DEFERRED triple still sums to 198; and `cargo xtask spec-trace`, `cargo xtask affected --base main` and `cargo xtask ci --fast` are all green on the result | `cargo xtask spec-trace` check 8 (`xtask/src/spec_trace.rs:459-508`), which verifies the stated total, each of the four counts and the sentence's subtraction separately; `.redkiln/config.yaml:40` at the story grain and `:55` at the integration grain; `git diff --name-only` listing nothing outside the two globs in the PR boundary. Checks 5 and 7 ride along in the same run — check 7 is what catches a citation broken by this PR's own line shift |

## Interaction quality

**Composition family — declared N/A, on the record.** The project's signed-off `_design.md` records
**Surfaces: N/A** and every composition section (`Items`, `Placement and re-export`, `The states the API must
express`, `Anti-patterns`, `The doctest`) as `N/A — no user-facing surface`, approved by the repository owner
on 2026-08-12 (`_design.md:38-48`, `:86-95`). This story renders no screen, adds no `pub` item and ships no
doctest, so there is no presentation, placement, transience, density or hierarchy invariant to carry. That is
a *declared* skip inherited from a human sign-off, not a silent one, and it is not a licence to skip the
state family below.

**State family — the surface is the document, and it has state.** `spec/SPECIFICATION.md` is read by a person
scrolling it and by `spec-trace` parsing it, and both are position-sensitive. Every invariant that applies is
carried by an AC row in the table above; none is left as a bullet here.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump** — each marker is replaced where the clause already lives; no clause is relocated, re-numbered or split, so a reader's mental index of the document and every inbound `file:line` citation survive | AC-001, AC-003 | The AC rows' own checks, plus check 7's citation sweep inside AC-009's gate run (`xtask/src/spec_trace.rs:729-736`) |
| **Non-occlusion** — the regenerated region stays strictly between `<!-- BEGIN GENERATED -->` and `<!-- END GENERATED -->` and swallows nothing; §1.3 in particular is never moved inside those markers, which would destroy the property it exists to prove | AC-008, AC-009 | Check 9's region comparison; a diff review confirming the hunk boundaries at `spec/SPECIFICATION.md:8507` and `:8753` are unchanged, and that `:219-222` is still outside them |
| **Preserved selection** — the things deliberately *not* selected stay untouched: ES-38's text, every other `[FROZEN]` clause, the §6.5 portfolio table, `RUNBOOK.md`, every crate | AC-007 | `git diff -U0` hunk inspection and the PR-boundary glob check in AC-009 |
| **Reversibility** — one `git revert` restores a self-consistent document, because the marker text, the generated tables and the census move together; a partial revert is impossible by construction since there is nothing to partially revert | AC-008 | The one-commit assertion in AC-008, checked by `git log --oneline -1 --stat` |
| **Legibility of state** — a reader can tell *settled* from *implemented* without opening ADR-0028, and can tell *answered but not implementable* from *forgotten* | AC-001, AC-002 | Content review against `discover.md:47`'s `SettledByAssertion`; this is the invariant with no mechanical check behind it, which is why it is stated first and reviewed by a person |
| **Keyboard reachability / focus / scroll** | — | **N/A.** There is no interactive surface; the only navigation is a text editor's, and it is unaffected |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | ADR-0028 as merged does not state ES-39's answer plainly enough to write a marker from | **Halt and raise against HS-S0121.** Do not fill the gap in marker prose. An ambiguous ADR read by a marker author becomes a specification clause nobody decided — the escalation is cheaper than the retraction |
| EC-002 | After removing ES-40's `(new)`, check 4 reports `condition_over_removed_history_does_not_reject` unresolvable | A dependency failure on HS-S0117, not a marker problem. **Do not restore `(new)` to go green** — that re-exempts the clause and hides the missing rule. Stop, confirm the rule's absence in `crates/happenstance-testkit/src/suite.rs`, and report it |
| EC-003 | Check 8 reports §1.3 disagreeing with the computed census | Recompute the census by hand and fix **§1.3**. Never edit §7.1/§7.2 to match the prose, and never move §1.3 inside the generated markers — both make the disagreement disappear by destroying the check (`xtask/src/spec_trace.rs:39-57`) |
| EC-004 | Check 9 reports the generated region stale after `--write` ran | The clause body above it is wrong, not the table. Fix the clause and re-run `cargo xtask spec-trace --write`. **No hand edit inside the markers**, ever, including to remove a row that looks surprising |
| EC-005 | Check 7 reports a citation that no longer resolves, caused by this PR's own line shift inside `spec/SPECIFICATION.md` | Rewrite the citation to a clause **id**, or to id-plus-line where both serve. Do not chase the moving line number, and do not delete the citation to silence the check |
| EC-006 | The `[DEFERRED]` count does not land on 8 | Something other than the two intended markers moved, or a clause was reworded into or out of recognition. **Do not adjust §1.3 to match a document that is wrong** — reconcile the clause bodies first, then recompute |
| EC-007 | §7.1 classifies CF-27 as a maturity the author did not write | The marker is not in declaration position and `maturity_of` took the fallback (`xtask/src/spec_trace.rs:1441-1472`). Move the marker onto its own line; do not reword the surrounding prose to make the fallback pick the right thing by luck |
| EC-008 | The renewal branch of ES-40 is taken and the named experiment path does not exist | Not a commit-and-fix-later condition. Either discharge the marker or create nothing here and escalate — a named-but-absent experiment passes check 2 and is exactly the `RenewalWithoutAnExperiment` mutant |

## Non-functional

| id | Requirement | Why |
| --- | --- | --- |
| NF-001 | **No crate compiles differently.** The diff touches no `.rs`, no `Cargo.toml`, no feature gate. `cargo xtask affected --base main` maps this diff to zero packages and is carried entirely by the five file-reading lints and `spec-trace` (`.redkiln/config.yaml:36-40`) | It is what makes `cargo xtask ci --fast` here a check of the *document*, and what makes a surprising package failure a signal that the boundary was breached |
| NF-002 | **`--write` is idempotent on the committed result.** Running `cargo xtask spec-trace --write` a second time produces an empty diff | A non-empty second diff means the region committed was hand-touched or the run was not the last thing before the commit |
| NF-003 | **The marker text stays a paragraph, not a page.** ES-39's replacement carries the answer, the non-adoption and the reader's exposure; the derivation belongs to ADR-0028 and is cited by id | A clause that restates its ADR drifts from it the first time the ADR is superseded — which is the failure mode `adr-0028-and-the-open-question-wave/spec.md:454` deliberately designed against |
| NF-004 | **The record survives supersession.** Every reference to the decision is by ADR **id**; no marker embeds a line number into `.kb/decisions/` or `references/adr/` | An accepted decision atom is immutable and is superseded rather than edited (`CLAUDE.md`), so an id survives what a line range does not |
| NF-005 | **§1.3 is computed by a method independent of `parse_clauses`.** A hand count, or a `rg` over the marker literals — not a reading of §7.1 | Copying §7.1 into §1.3 satisfies check 8 while destroying the only cross-check the document has on its own parser (`xtask/src/spec_trace.rs:39-54`) |

## Implementation notes (non-prescriptive)

- **Order is load-bearing, and it is one commit.** Edit the three clause bodies → `cargo xtask spec-trace
  --write` → hand-reconcile §1.3 → bare `cargo xtask spec-trace`. Anything else leaves a window where the
  document is internally inconsistent and green.
- **Read ADR-0028 first, and write the marker text from it before touching anything mechanical.** The
  mechanical parts of this story are twenty minutes; the ES-39 sentence that distinguishes *settled* from
  *implemented* is the deliverable, and writing it last invites the `SettledByAssertion` shortcut.
- **Compute §1.3 the independent way.** Counting the marker literals across the document with `rg` is a
  different program from `parse_clauses`, which is the entire point; if the two disagree, that disagreement is
  information about the parser and is worth a line in the implementation report even when the fix is obvious.
- **Do the two deliberate falsifications and record them** — removing ES-39's `(new)` until check 4 fails
  (AC-002), and adding a quoted marker into CF-27's body to confirm the declaration branch now wins (AC-003).
  Both are reverted before the commit; both turn a passing run into evidence that the check was live.
- **Expect more of the generated region to change than the sibling specs' *one cell* language suggests.**
  CF-27's `Rule:` field wraps across five lines and names three rules, so its row moves too. Accept the tool's
  output; the surprise is information, not an error to trim.
- **Prefer clause ids to line numbers in anything new you write into the document.** This PR is itself the
  thing that moves `spec/SPECIFICATION.md`'s line numbers.
- **If a check fails for a reason this spec does not name, stop and report it** rather than reshaping the
  document until the checker is quiet. Every one of the discover's named mutants is a green commit.

## Tests and CI (merge gate)

| Tier | Command / path | Proves |
| --- | --- | --- |
| Static — the document's own checker | `cargo xtask spec-trace` (`xtask/src/spec_trace.rs:650-742`) | Checks 1–3: every clause carries a marker, no `[PROVISIONAL]`/`[DEFERRED]` clause has an empty falsifier, every clause names what it rejects — AC-001, AC-003, AC-005 |
| Static — rule resolution | `cargo xtask spec-trace` check 4 (`xtask/src/spec_trace.rs:680-711`, `:1626-1631`, `:1735-1737`) | ES-40's name resolves against `crates/happenstance-testkit/src/suite.rs` once `(new)` is gone; ES-39 stays exempt while `(new)` remains — AC-002, AC-006 |
| Static — rule ownership | `cargo xtask spec-trace` check 6 (`xtask/src/spec_trace.rs:725-727`) | No rule is left unclaimed by any clause after CF-27's line is reconciled — AC-004 |
| Static — citations | `cargo xtask spec-trace` check 7 (`xtask/src/spec_trace.rs:729-736`) | Every `file:line` in the document still resolves after this PR shifts the document's own line numbers — AC-009 |
| Static — the hand census | `cargo xtask spec-trace` check 8 (`xtask/src/spec_trace.rs:459-508`) | §1.3's total, each of the four maturity counts and the sentence's own subtraction agree with the computed census — AC-009 |
| Static — the generated region | `cargo xtask spec-trace` check 9 (`xtask/src/spec_trace.rs:741-742`) versus `cargo xtask spec-trace --write` (`xtask/src/main.rs:671-674`) | §7.1/§7.2 are exactly what the clause bodies produce, and were regenerated rather than edited — AC-008 |
| Static — retired rules | `cargo xtask lints` (`xtask/src/main.rs:342`) | Nothing was retired while still live; the *this project retires nothing* note is checked rather than assumed (`_decomposition.md:488-490`) — AC-007 |
| Story grain (auto, `redkiln advance`) | `cargo xtask affected --base main` (`.redkiln/config.yaml:36-40`) | The five file-reading lints and `spec-trace` run unconditionally, so a story that maps to no package is still gated — AC-009 |
| Integration grain, cheap tripwire | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | The slice's document-level reachability check, no compiler involved — AC-007, AC-009 |
| Integration grain, non-terminal | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | The project's recorded bar: fmt, clippy `-D warnings`, tests, the four mandatory `wasm32` steps, docs and `spec-trace` on the assembled tree — AC-009. `cargo xtask ci` (`:60`) is HS-P0019's and is a local sanity check here at most (`_decomposition.md:495-496`) |
| Diff boundary | `git diff --name-only` against the PR-boundary globs; `git diff -U0 spec/SPECIFICATION.md` | Nothing outside `spec/SPECIFICATION.md` and this story's backlog folder; no hunk inside ES-38 at `:4299-4323` — AC-007, AC-009 |
| Review (no compiled test exists) | Content review against `discover.md:47`, `:51`, `:52` — `SettledByAssertion`, `RenewalWithoutAnExperiment`, `RetiredButLive` | The three failures every mechanical check above passes. This row is the reason the slice review is part of the gate and not a courtesy — AC-001, AC-004, AC-005 |

## Risks and coupling (PR-scoped)

- **Coupled to ADR-0028's wording, not just its existence.** This story consumes the ADR as text. If HS-S0121
  landed an answer that is decisive in the record but hedged in the summary, the marker author is the first
  person to discover it. Mitigation: EC-001 — escalate, do not paper over. Likelihood low, cost high, because
  a wrong marker is a published promise.
- **Coupled to which branch HS-S0122 took.** AC-004 has two shapes and the implementer must read
  `cf-27-rule-or-recorded-refusal/spec.md` and the merged result rather than assuming the decide branch. A
  `Rule:` line reconciled to the wrong branch is invisible to check 4 (`has_suite` is false for `CF-`) and
  survives to publish.
- **This PR moves `spec/SPECIFICATION.md`'s line numbers, and other artifacts cite them.** Check 7 catches
  citations *inside* the document; it does not police the backlog. Sibling specs and `RUNBOOK.md` cite spec
  lines, and `surface-diff-and-the-ac-012-escalation` reads this result immediately after. Mitigation: prefer
  clause ids in new text, and flag the shift in the implementation report so HS-S0124 re-resolves rather than
  trusts.
- **The stated residual on ES-38.** Leaving `(new)` keeps that clause exempt from check 4 permanently, so the
  checker never validates `positions_are_not_reused_after_removal` *via ES-38*. Accepted deliberately at the
  discover gate (`discover.md:39`, `:72-74`); the compensating signal is §7.2's ES-38 cell at
  `spec/SPECIFICATION.md:8620` and check 6's ownership sweep. Recorded here so a later reader finds the
  trade rather than re-discovering the hole.
- **The census is the step most likely to be skipped under time pressure**, and skipping it is green today.
  The whole cost lands on the *next* author. AC-008 and AC-009 exist as separate rows precisely so the ledger
  cannot be closed with the tables regenerated and the sentence untouched.
- **Out of scope and adjacent, so easy to drift into**: the §6.5 portfolio table's completeness row, the
  CF-25/CF-26 residual exposure (recorded open by ADR-0028, `_decomposition.md:205-213`), and `RUNBOOK.md:165`.
  `spec-trace` reads none of them, so a green gate is no evidence either way and a helpful edit here is an
  unreviewed change to another story's deliverable.

## Dependencies

**Blocks on** — all three must be merged before this story starts, because a marker can only move once the
rule it names exists or is recorded as never-to-exist (`_storymap.md:146-149`):

- `cf-27-rule-or-recorded-refusal` (HS-S0122) — settles CF-27's own rule either way, and transitively carries
  ADR-0028 into this slice. AC-003 and AC-004 are undefined without its branch.
- `positions-are-not-reused-after-removal` (HS-S0116) — makes `positions_are_not_reused_after_removal` resolve,
  which is what lets §7.2's ES-38 cell lose its `†` under AC-007.
- `condition-over-removed-history-does-not-reject` (HS-S0117) — makes
  `condition_over_removed_history_does_not_reject` resolve and gives ES-40 its assertion. AC-005's discharge
  branch and AC-006 both depend on it.

**Unlocks**

- `surface-diff-and-the-ac-012-escalation` (HS-S0124) — merges last in this slice and reads the assembled
  tree, including this story's document, as the exit computation over the project.
- Project **AC-014** in full (`project.md:248-251`), and with it the record half of initiative **DoD 12**
  (`.bklg/from-contract-to-published-library/initiative.md:393-395`) and **DoD 15** (`:402-404`).

## Anchors (progressive disclosure)

Open these at the moment named, not before. The context pack above is sufficient to start; these carry the
depth deliberately kept out of it.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` (ES-39 at `:4325-4349`) | The clause being amended, including the `Rejects:` line that already satisfies check 3 and the *scattered, not a prefix* argument the replacement text must not contradict | First, before writing a word of replacement marker text | AC-001, AC-002 |
| `spec/SPECIFICATION.md` (CF-27 at `:8034-8060`, its §7.2 row at `:8738`) | Shows the marker sitting mid-sentence and the five-line `Rule:` field naming three rules — the two facts that make CF-27 the awkward one | Before editing CF-27, and again when the regenerated region surprises you | AC-003, AC-004 |
| `spec/SPECIFICATION.md` (§1.3 at `:219-222`, §7.1 at `:8513-8519`) | The exact sentence to recompute, its literal opening words, and today's per-section figures | At the census step, after `--write` has run | AC-009 |
| `xtask/src/spec_trace.rs` | The nine checks (`:650-742`), `maturity_of`'s declaration-versus-fallback rule (`:1441-1472`), `schedules_new` (`:1626-1631`), `has_suite` (`:1735-1737`), `rule_cell`'s resolution-based dagger (`:1147-1183`), `check_stated_census` (`:459-508`), and the module comment stating in its own words why §1.3 must never be generated (`:39-57`) | Whenever a check fails, and before deciding any check is wrong | AC-002, AC-003, AC-006, AC-008, AC-009 |
| `xtask/src/main.rs` | The two `spec-trace` modes (`:671-674`) and the retired-rules lint step (`:342`) | At the regeneration step, and when confirming nothing was retired | AC-007, AC-008 |
| `crates/happenstance-testkit/src/suite.rs` | Where check 4 resolves rule names; the definition of `condition_over_removed_history_does_not_reject` that makes removing ES-40's `(new)` safe | Immediately before removing ES-40's `(new)` — cite the definition line as evidence | AC-006 |
| `crates/happenstance-testkit/src/registry.rs` | `for_each_event_store_rule!`, the registry check 6's ownership sweep reads | If check 6 reports an unclaimed rule after CF-27's line is reconciled | AC-004, AC-007 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/cf-27-rule-or-recorded-refusal/spec.md` | The branch HS-S0122 committed to — decide or refuse — which AC-004's whole shape depends on | Before touching CF-27's `Rule:` line | AC-004 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/condition-over-removed-history-does-not-reject/spec.md` | Records where ES-40's rule landed and what rustdoc discharged the clause's documentation obligation | When confirming AC-005's discharge branch is the right one | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/positions-are-not-reused-after-removal/spec.md` | Records that HS-S0116 left `spec/SPECIFICATION.md` otherwise byte-identical, which is why ES-38's `(new)` is still there to leave alone | When reviewing the ES-38 residual, or if §7.2's ES-38 cell does not lose its `†` | AC-007 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/adr-0028-and-the-open-question-wave/spec.md` | Where ADR-0028 was specified — its scope, its citation-by-id discipline (`:454`, `:564`), and the statement at `:492` that this is the decision all three clauses are deferred on | First, alongside the ADR itself, when drafting marker text | AC-001 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/marker-moves-and-spec-trace-green/discover.md` | The three named wrong implementations (`:47`, `:51`, `:52`) and the recorded ES-38 gate decision with the two readings that lost (`:39`, `:72-74`) | Before the self-review, and any time the ES-38 scope feels re-litigable | AC-001, AC-005, AC-007 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` | The architecture brief's *Gate mechanics AC-014 must satisfy* (`:468-496`), composition root 7 (`:143-146`), and the testing brief's AC-014 row (`:616`) and merge-gate commands (`:620-650`) | When wiring the gate commands, and if a reviewer disputes which gate is this project's bar | AC-008, AC-009 |
| `.bklg/from-contract-to-published-library/_decomposition.md` | Gate decision 4, the initiative-level no-published-surface constraint (`:239-249`) — the constraint a marker claiming an adopted primitive would breach in prose | When drafting ES-39's replacement, and at self-review against `SettledByAssertion` | AC-001 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` | The signed-off **Surfaces: N/A** determination (`:38-48`) and its sign-off (`:86-95`) — the reason the composition family of interaction quality is a declared skip here | Once, when confirming this story renders nothing | AC-001 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The adapter author's *Learn when you are finished* and the evaluator's *Decide in one sitting* — whose reading of the document every AC above is framed from | When judging whether a replacement marker is legible to its reader rather than merely well-formed | AC-001, AC-003 |
| `.redkiln/config.yaml` | The four `verify:` commands and the comment stating why `affected` runs the file-reading lints unconditionally (`:36-40`, `:48`, `:55`, `:60`) | When running the gates, and before claiming a grain is satisfied | AC-009 |
| `experiments/position-visibility/README.md` | The shape a named experiment takes in this repository — the bar ES-40's renewal branch must clear | Only on the renewal branch of AC-005 | AC-005 |
| `.bklg/from-contract-to-published-library/initiative.md` | DoD 12's exact wording (`:393-395`), DoD 15 (`:402-404`) and exit criterion 5 (`:561-582`) — what the marker moves are ultimately audited against | When writing the implementation report's DoD claims | AC-009 |

## Clarifications resolved during spec

1. **What replaces `[DEFERRED]` on ES-39 — deferred by discover (`:35`), resolved here as a *constraint on the
   text* rather than as a choice of word.** The marker word is ADR-0028's to imply; what this spec fixes is
   that the replacement must record the question as settled **and** the primitive as unadopted, in declaration
   position, naming the reader's residual exposure. AC-001 carries it. Any of `[FROZEN]`, `[PROVISIONAL]` or a
   demotion is admissible if ADR-0028 supports it — but a `[PROVISIONAL]` replacement inherits check 2's
   falsifier obligation and a demotion to `[NON-NORMATIVE]` would move the invariant totals in AC-009, so the
   two carry consequences the implementer must price before choosing.
2. **Discharge or renew ES-40 — deferred by discover (`:36`), resolved as a default with a bar.** Discharge is
   the expected branch because HS-S0117 gave the clause its assertion; renewal is admissible only against an
   `experiments/` path that exists on disk. AC-005 states both branches and EC-008 makes the absent-path case
   a halt rather than a fix-later.
3. **ES-38 is out of scope entirely**, per the repository owner's reading (b) at the `/redkiln:plan` discover
   gate on 2026-08-12 (`discover.md:39`, `:72-74`). This spec does not reopen it; AC-007 asserts the
   byte-identity and the *Risks* section records the residual the choice leaves.
4. **The AC set is exactly the nine ids the first pass enumerated** — AC-001 through AC-009 — with none added
   and none dropped. Two obligations named in the front half do not get their own row and are folded
   deliberately: **check 7's citation resolution** rides in AC-009's gate run (the whole nine-check pass is
   what that row demands) and is additionally carried as the *prefer clause ids* instruction in the behaviour
   table; and **nothing is retired** is folded into AC-007 alongside the frozen-clause byte-identity, because
   both are verified by the same act of reading the diff plus one lint step. Splitting either into a tenth row
   would have produced a ledger row whose evidence is a substring of another's.
5. **The census figures in this spec are today's, and are stated so they can be *checked* rather than
   trusted.** 200 / 198 / 139 / 49 / 10 / 2 is what the document says at `spec/SPECIFICATION.md:219-222`
   today; AC-009 binds the invariants (200, 198, 2) and the direction (`[DEFERRED]` 10 → 8) rather than the
   final triple, because where ES-39, ES-40 and CF-27 land is ADR-0028's to say and the worked 142 / 48 / 8 / 2
   example in the context pack is explicitly not a prediction.
6. **Interaction quality is not vacuous here despite the N/A design.** The composition family is a declared
   skip inherited from a signed-off `_design.md`; the state family is real, applies to a document rather than
   a screen, and is carried by AC rows rather than by prose bullets — which is why AC-008 asserts one commit
   and non-occlusion of the generated markers, and AC-007 asserts preserved selection.
