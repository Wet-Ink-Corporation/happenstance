---
item: HS-S0120
stage: spec
created: 2026-08-12T13:48:01.031Z
updated: 2026-08-12T13:48:01.031Z
template_sig: 87bbf1d0
rendered_sig: c4e62384
---

# Spec — DT-7's signal shape and the redaction question, answered against the code

## Scope lock

| Axis | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DT-7 at `:426`, AC-14 at `:347-349`, DoD 15 at `:402-404` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — gate decision 4 (the no-published-surface-change constraint) at `:239-249` |
| Project | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` — AC-009 at `:228-230`, AC-010 at `:231-234`, the `[FROZEN]` exclusion at `:112-114`, the schedule-position risk at `:331-335` |
| This spec | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/dt-7-signal-shape-and-the-redaction-answer/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` — DA-7 (`:363-379`), DA-8 (`:381-397`), DA-9 (`:446-466`), the test-mix rows for AC-009/AC-010 (`:604-612`) |
| Signed-off design (mount point) | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` — the no-surface determination at `:20-36`, the sign-off at `:86-95` |
| Story map row | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md:74` (this story), `:96` (ADR-0028's row, which consumes it) |
| This story's discover | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/dt-7-signal-shape-and-the-redaction-answer/discover.md` |
| Roadmap pointer | `RUNBOOK.md:165` (phase 14's status row), `RUNBOOK.md:307` (ADR-0028's row in the ADR queue) |

## One-line PR slice

Resolve DT-7 in this project's `_design.md` — one undifferentiated incompleteness signal, or the
transient / benign-permanent / meaningful-permanent distinction — naming the option that lost and why,
decided against the evidence slices 1–3 produced rather than against intuition, and stated as a
**vocabulary rather than a reported signal** because no return shape in the port can carry one; and
answer the redaction question (E2E-49) against the code as an **explicit deferral naming a real
`experiments/` path** that exists on disk and says what it would measure.

## Executive summary

This PR lands two written resolutions and one new directory, and no Rust.

**Delta against the project.** `project.md:228-234` asks for two things this project has so far only
scheduled: DT-7 with its loser named (AC-009), and the redaction question answered or deferred against a
*named* experiment (AC-010). Slices 1–3 have by now produced the evidence both were waiting on — the
CF-27 pass list (`cf-27-experiment-and-recorded-pass-list`) and the two reader observations
(`decision-model-and-ingest-observed`) — so this story is where the project stops gathering and starts
answering.

**Delta against `_design.md`.** That artifact already exists, is already signed off, and already claims
DT-7 as its own: it says DT-7 *"is resolved as a vocabulary/documentation decision inside this project's
own prose and ADR-0028"* (`_design.md:20-36`). What it does not yet carry is the resolution. This PR
appends it, additively, leaving the no-surface determination and the approval block (`:86-95`) exactly
as signed.

**Delta against the redaction question.** DA-9 (`_decomposition.md:446-466`) has already read the code
and found the answer is bounded: `Tag` equality *is* string equality with no digest field to swap in,
and there is no store-side update path, so redaction needs either a new store operation (a surface
change, forbidden by gate decision 4) or a change to what `Tag` equality *means* (a silent re-keying of
every tag index). This PR converts that reading into the recorded answer AC-010 asks for, and makes the
named experiment real by creating it as a charter under `experiments/` rather than by writing a path
into prose and hoping.

**What this PR is not.** It does not write ADR-0028 — that is `adr-0028-and-the-open-question-wave`
(`_storymap.md:96`), which reads what this story writes and records AC-010 where the AC asks for it. It
adds no conformance rule, moves no spec marker, and touches no published signature.

## Context pack

The decisions below are binding on this story. They are stated here in full so an implementing agent can
start from this section alone; the deeper artifacts sit behind the anchors, linked and never pasted.

**1. DT-7 is decided here, and it is decided after the evidence — never before it.** The initiative
states the tension in one line: *"How a reader is told the log it is reading is incomplete, and why"* —
(a) one undifferentiated signal; (b) distinguish transient, benign-permanent and meaningful-permanent;
*"A shipped peer framework found one undifferentiated signal insufficient in production; distinguishing
costs the reader complexity"* (`initiative.md:426`). Both sides carry real weight and neither may be
waved away. The production evidence is Ecotone, which taxonomises *why* a position is missing and treats
the three cases as operationally different outcomes — out-of-order commits (transient, expected to
heal), rolled-back transactions (permanent, benign) and genuine deletions (permanent, and the one case
that is not benign)
(`.bklg/from-contract-to-published-library/_discovery/research/07-retention-deletion-and-incomplete-log-semantics.md:98-107`).
The cost is borne by every reader of this library, forever. The story's *new* input is the evidence
slices 1–3 produced: the pass list says how much the suite can distinguish at all, and the two reader
observations say what a reader actually receives. **Weigh against those, cited by path. A resolution
that reasons only from the charter's one-line framing has skipped the thing this project built.**

**2. Whichever option wins, what is landed is a *vocabulary*, not a reported signal — and the resolution
must say so in terms.** This is the decisive constraint, and it comes from the code rather than from
preference. There is no channel in the port for even *one* incompleteness signal, let alone three:
`read_decision_model` returns `(Vec<SequencedEvent>, Option<SequencePosition>)` and nothing else
(`crates/happenstance-core/src/store.rs:321-331`); `Guard::is_violated_by` returns `bool` from a
deliberate two-arm `match` with no third arm (`crates/happenstance-core/src/append.rs:239-253`); and
`contains_event_id` returns `Result<bool, _>` whose `false` already conflates *"I never had it"* with
*"I had it and forgot"* (`crates/happenstance-core/src/store.rs:268`, DA-8). So the resolution names
words that ADR-0028 and the rustdoc may use and that a future primitive would have ready — it does not
describe how a store *reports* which kind of hole a reader met. **Specifying a reporting shape is ES-39's
primitive under another name; that is an AC-012 escalation (DA-7, `_decomposition.md:363-379`), not a
decision this story may take.**

**3. The whole answer is constrained to what needs no published-surface change.** Gate decision 4
(`.bklg/from-contract-to-published-library/_decomposition.md:239-249`) binds both halves: ES-39 and
ES-40 are `EventStore` clauses, `0.2.0` is already published, and a decision that changes a published
port is a `0.3.0` this initiative's exit criteria do not contemplate. Inside that constraint DT-7 is a
vocabulary decision and redaction is a named deferral — both reachable, neither a compromise smuggled in
as one.

**4. Deciding *to* redact is not available to this story.** ES-37 is `[FROZEN]`: `EventStore` MUST NOT
grow delete / truncate / redact / compact / tombstone (`spec/SPECIFICATION.md:4274-4297`). Amending a
`[FROZEN]` clause takes a new decision atom and a re-plan (`project.md:112-114`), not an edit. The
reachable answer leaves ES-37's text exactly as it is. If the honest answer is concluded to *require*
amending ES-37, that is a finding to raise as an AC-012 escalation — never a change made here.

**5. The deferral must name an experiment, and the name must be real.** AC-010's own words: *"answered
in ADR-0028 or explicitly deferred against a **named** experiment. A deferral with no experiment is a
gate failure under CF-38"* (`project.md:231-234`). CF-38 is the traceability checker
(`spec/SPECIFICATION.md:8318`) and the standard it sets — no provisional claim with an empty falsifier —
is the standard this deferral is held to whether or not the checker's own globs reach a backlog
artifact. `experiments/` is the repository's home for measurements that are *reproducible and not in the
gate* (`CLAUDE.md`, repository map), it is not a workspace member (`Cargo.toml:3` lists
`crates/*`, `examples/*`, `xtask`), and `experiments/wire-format/README.md` records why that isolation
is load-bearing. **The experiment this story names exists on disk as a charter; it is not run here.**

**6. What the experiment must be measured against, and why exactly those two rules.** A tag-digest
scheme changes what `Tag` equality *means*, and `Tag`'s `PartialEq`/`Eq`/`Ord`/`Hash` are hand-written,
each delegating to `as_str()` (`crates/happenstance-core/src/tag.rs:79`, `:170-193`). The signature
would not move, so `cargo-semver-checks` would see nothing; what moves is every tag index in every
adapter. The two conformance rules whose observable behaviour such a change would move are
`query_item_tags_are_and` (`crates/happenstance-testkit/src/suite.rs:409`) and
`query_item_tags_match_supersets` (`:439`), both registered in `for_each_event_store_rule!`
(`crates/happenstance-testkit/src/registry.rs:112-113`). The charter names those two by name.

**7. The mount is `_design.md`, and the sign-off above it is not re-opened.** `_design.md` records **no
user-facing surface** and was approved at the `/redkiln:plan` design gate on 2026-08-12 (`:86-95`). That
approval covers the no-surface determination and the anti-patterns; it does not contain DT-7's content,
which the same artifact hands to itself (`:20-36`). This story adds a section; it does not edit the
approved text, does not re-decide the no-surface finding, and does not silently backdate its addition
into the signed block.

**8. The persona-journey slice this serves.** *Learn when you are finished* (the adapter author) and
*Choose a contract before a database* (the application author) — `initiative.md:242-249`. The humane
outcome is narrow and worth stating precisely: an adapter author who reads the retention vocabulary
learns the words **and** learns, in the same paragraph, that nothing in the port reports them — so the
first person to go looking for the reporting channel finds the sentence that says there is none, instead
of finding `Result<bool, _>` and drawing their own conclusion.

**9. The named wrong implementations this story exists to reject** (from `discover.md`, and each one is
green under every existing check because it is prose): **`ThreeKindsOfNothing`** — adopting the three-way
distinction and never saying nothing can carry it, which settles ES-39 by implication from an artifact no
ADR authorises. **`OneSignalBecauseItIsFriday`** — the one-signal answer taken because it is cheaper at
rank 5, recorded without naming the production evidence it overrules; `project.md:331-335` names this
schedule risk by name and DR-9 is its counterweight. **`DeferredToNothing`** — *"whether a `Tag` can be
redacted is deferred"*, full stop, which reads as diligence and is a gate failure by construction.
**`DigestTagsQuietly`** — proposing the digest swap as the *answer* rather than as the experiment, a
silent re-keying that no signature check can see.

## Integration contract

- **Archetype**: `foundation`. It lands no runtime substrate, and it is not a capability slice: what it
  produces is the written input `adr-0028-and-the-open-question-wave` consumes. Under this project's
  own framing that is real substrate — a decision the ADR records rather than invents.
- **Slice / milestone**: `the-retention-decision`. **Slice-mate**: `adr-0028-and-the-open-question-wave`
  (the two are implemented in one context; this one merges first — `_storymap.md` *Merge order* item 4).
- **Mount point**: `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` —
  a new top-level `## DT-7 — how a reader is told the log is incomplete` section, appended after the
  existing `## Sign-off` block and carrying its own dated attribution to this story (HS-S0120), so the
  approved text above it stays exactly as approved. This is the composition root the project's AC-009
  names by path; a resolution written anywhere else does not satisfy it.
- **Second mount**: `experiments/<named-experiment>/README.md` — the deferral's falsifier, made real. A
  `experiments/` path cited in prose but absent from the tree is `DeferredToNothing` with a longer
  sentence.
- **Wires into**: the evidence artifacts of the two upstream slices — the recorded pass list from
  `cf-27-experiment-and-recorded-pass-list` and the two reader observations from
  `decision-model-and-ingest-observed` (`.bklg/…/decision-model-and-ingest-observed/`), both cited by
  path in the resolution; and the code the redaction answer is read against —
  `crates/happenstance-core/src/tag.rs`, `crates/happenstance-core/src/event.rs`,
  `crates/happenstance-core/src/store.rs`, `crates/happenstance-core/src/append.rs`. It consumes them
  read-only: not one of those files is edited by this story.
- **Renders surfaces**: **none.** `_design.md` records no user-facing surface (`:38-40`) and this story
  does not create one. It renders no screen, adds no `pub` item, and changes no signature.
- **Public items**: none. `_design.md`'s `## Items` block is `N/A — no user-facing surface`, and this
  story does not add an entry to it.
- **Conformance rule(s)**: none added, and that is deliberate rather than an omission — this story's
  behaviour is not adapter-observable, because a vocabulary that nothing reports has nothing an adapter
  could get wrong. The nearest executable checks are the two rules the *named experiment* is specified
  to measure against (`suite.rs:409`, `:439`), which is why they are named in the charter rather than
  asserted here. Adding a rule for a signal the port cannot carry would be `ThreeKindsOfNothing` with a
  test attached.
- **Clause(s)**: none discharged, none amended. ES-37 (`spec/SPECIFICATION.md:4274-4297`, `[FROZEN]`) is
  the clause this story stands closest to and it is left untouched; ES-39 (`:4325-4349`) supplies the
  scattered-not-a-prefix argument the vocabulary must not contradict but keeps its `[DEFERRED]` marker,
  which `marker-moves-and-spec-trace-green` moves. **No edit to `spec/SPECIFICATION.md` belongs in this
  PR.**
- **Advances DoD scenario**: **DoD 15** (`initiative.md:402-404`) — *"Incomplete logs have an answer on
  disk … the reader either fails loudly or the refusal to define this is recorded as a decision."* This
  story supplies the decided content of that answer; `adr-0028-and-the-open-question-wave` turns it into
  the accepted atom DoD 15 reads. It also advances initiative AC-14 (`:347-349`).

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file changed outside
it.

```
.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md
.bklg/from-contract-to-published-library/retention-and-incomplete-logs/dt-7-signal-shape-and-the-redaction-answer/**
experiments/**
```

`experiments/**` is deliberately wider than the one README this story writes, because the directory's
name is decided in the writing; it is still narrow in the sense that matters — nothing under
`experiments/` is a workspace member (`Cargo.toml:3`) or a gate step (`CLAUDE.md`, repository map), so
the widened glob cannot reach anything the compiler or the suite sees.

**In this PR**

- The DT-7 resolution appended to `_design.md`: the option chosen, the option that lost, and the reason,
  weighed by name against the pass list and the two reader observations.
- The explicit statement, inside that section, that the chosen shape is a vocabulary and not a reported
  signal, with the three channel-less return shapes named.
- The redaction answer, recorded as an explicit deferral against a named experiment, with the code
  reading it rests on and the ES-37 constraint that bounds it.
- The experiment itself, as a charter README under `experiments/` — what it would measure, against which
  two conformance rules, and what result would settle the question either way.
- This story's own `_ledger.md` and stage artifacts.

**Explicitly not in this PR**

- **ADR-0028**, in any form — long-form under `references/adr/`, staged under `.kb/_intake/`, or an atom
  under `.kb/decisions/`. That is `adr-0028-and-the-open-question-wave` (`_storymap.md:96`), and
  hand-writing a `.kb/` atom is the move `CLAUDE.md` reverted at `0269720`.
- Any edit to `spec/SPECIFICATION.md` or `spec/E2E-CASES.md` — including moving ES-39's or CF-27's
  `[DEFERRED]` marker, which belongs to `marker-moves-and-spec-trace-green`.
- Any conformance rule, `REGISTRY` row, `Defect`, or fixture change.
- Any change to a published signature in `happenstance-core`, `happenstance` or `happenstance-testkit`;
  in particular, no change to `Tag`, its hand-written trait impls, or anything that alters what tag
  equality means. The digest scheme is the *experiment*, not the change.
- **Running** the experiment, or adding it to any gate step.
- Re-deciding `_design.md`'s no-surface determination, or editing the approved sign-off block.
- Settling `read_from_a_gap_position`'s ownership, which sits one clause away and is out of scope by
  `project.md`'s *Out of scope*.

**Merge DoD**: `_design.md` carries a DT-7 resolution naming the loser and stating the
vocabulary-not-signal limit; the redaction deferral names an experiment that exists on disk and says what
it measures; `git diff` touches nothing outside the boundary above; `cargo xtask affected --base main`
(`.redkiln/config.yaml:40`) is green — which for a prose story means the five file-reading lints and
`spec-trace` are green, the one gate a documentation-shaped change cannot bypass.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| DT-7 is resolved, in `_design.md`, with the loser named | A new `## DT-7 …` section states which of (a) one undifferentiated signal or (b) transient / benign-permanent / meaningful-permanent is adopted, **and** what the rejected option was and why it lost. A section stating a winner without naming what lost fails AC-009 by the brief's own test-mix row | `.bklg/…/retention-and-incomplete-logs/_design.md`; `project.md:228-230`; `_decomposition.md:604-612` (AC-009 row) |
| The resolution is weighed against evidence, not intuition | Both branches cite, by path: the recorded pass list from `cf-27-experiment-and-recorded-pass-list` (what the suite can distinguish at all) and the two reader observations from `decision-model-and-ingest-observed` (what a reader actually receives — the confidently wrong fold, and `holds()` answering `false` for an event this store minted). If (a) wins, the section names the Ecotone production evidence it overrules; if (b) wins, it names the reader complexity it buys | `.bklg/…/decision-model-and-ingest-observed/discover.md:39-51`; `.bklg/…/_discovery/research/07-retention-deletion-and-incomplete-log-semantics.md:98-107`, `:150-163` |
| The resolution declares itself a vocabulary, not a reported signal | The section states in terms that nothing in the port carries an incompleteness signal today, naming the three return shapes, and that adopting words now is preparation for a primitive a future ADR may add — never a claim that a store reports which kind of hole a reader met | `crates/happenstance-core/src/store.rs:321-331`, `:268`; `crates/happenstance-core/src/append.rs:239-253` |
| The vocabulary is not a boundary position in disguise | Incompleteness is not described as a floor ("history below position N is gone"). That is `earliest_position()` smuggled in as words, and ES-39 rejects it by name: a regulated purge is *scattered, not a prefix*, and a floor *"ships looking correct until a claim runs long"* | `spec/SPECIFICATION.md:4325-4349` (esp. `:4336-4341`) |
| No reporting shape is specified, and the escalation route is named instead | If the resolution's honest form needs a store to *report* the distinction, the section records that as the AC-012 escalation it is, pointing at DA-7's four-row option table (floor / retained ranges / third condition outcome / tri-state `contains_event_id`) and at `surface-diff-and-the-ac-012-escalation`, and makes no change | `_decomposition.md:363-379` (DA-7), `:381-397` (DA-8); `project.md:231-234` |
| The redaction question is answered against the code | The answer states: `Tag` is `Tag(Cow<'static, str>)` with hand-written `PartialEq`/`Eq`/`Ord`/`Hash` all delegating to `as_str()`, so tag equality **is** string equality with no digest field to swap in; `EventParts` separates opaque `data` from queryable `tags`, so shredding covers `data` and cannot cover `tags`; and `EventStore` has no store-side update path. E2E-49 is the concrete case the answer is owed to | `crates/happenstance-core/src/tag.rs:79`, `:170-193`; `crates/happenstance-core/src/event.rs`; `crates/happenstance-core/src/store.rs`; `spec/E2E-CASES.md:1288-1311`; `_decomposition.md:446-466` (DA-9) |
| The answer is an explicit deferral, and ES-37 is left as it is | Redaction needs either a new store operation (a published-surface change, forbidden by gate decision 4) or a change to what `Tag` equality means (a silent re-keying of every tag index — invisible to `cargo-semver-checks` because the signature does not move). Both are out of reach here, so the recorded answer is *not today, and here is the experiment that could change that*. ES-37's text is unchanged by this PR | `spec/SPECIFICATION.md:4274-4297`; `project.md:112-114`; `.bklg/from-contract-to-published-library/_decomposition.md:239-249` |
| The named experiment exists on disk | A charter README under `experiments/<name>/` states the hypothesis (a digest-keyed `Tag`), what it would measure, and that it has **not** been run. It is not a workspace member and not a gate step, matching the precedent `experiments/wire-format/README.md` sets for keeping a measurement reproducible and out of the gate | `experiments/wire-format/README.md`; `Cargo.toml:3`; `CLAUDE.md` (repository map) |
| The experiment names the two rules it is measured against | `query_item_tags_are_and` and `query_item_tags_match_supersets` — the two registered rules whose observable behaviour a change to tag equality would move — named in the charter by rule name and path, so the falsifier is executable rather than gestural | `crates/happenstance-testkit/src/suite.rs:409`, `:439`; `crates/happenstance-testkit/src/registry.rs:112-113` |
| The handoff to ADR-0028 is explicit | Both resolutions are written so `adr-0028-and-the-open-question-wave` can cite them by path and line: AC-010's answer is *recorded* in ADR-0028 where the AC asks for it, and this story writes no ADR, no `.kb/_intake/` file and no `.kb/` atom | `_storymap.md:74`, `:96`; `project.md:231-234`; `CLAUDE.md` (the `0269720` revert) |
| The sign-off is preserved, and the addition is honest about its date | The DT-7 section is appended after `## Sign-off` and carries its own attribution — added by HS-S0120 after the 2026-08-12 design gate, under the sign-off's own statement that DT-7's content lives here. The approved no-surface determination and the approval paragraph are unchanged | `.bklg/…/retention-and-incomplete-logs/_design.md:20-36`, `:86-95` |
| Nothing else moves | No published signature, no clause text, no conformance rule, no marker, no `.kb/` write. Verified by `git diff` against the PR boundary and by `cargo xtask affected --base main`, which runs the file-reading lints and `spec-trace` unconditionally — the check a story whose whole deliverable is prose would otherwise have none of | `.redkiln/config.yaml:40`; `_decomposition.md:604-612` (AC-011/AC-012 rows) |

## Data and migrations

**N/A — no schema, no persisted data, no migration.** This story writes two markdown artifacts and
creates one directory. It defines no type, touches no store, and introduces no serialised form; the only
"data" it produces is prose citing evidence recorded by earlier stories.

One near-miss is worth naming so it is not mistaken for a migration later: the deferred tag-digest
scheme *would* be a data migration of the largest kind — re-keying every tag index in every adapter,
with no signature change to announce it (`crates/happenstance-core/src/tag.rs:170-193`). That is exactly
why it is deferred to a measured experiment rather than proposed here, and why the charter states the
migration cost as part of what the experiment must weigh.

## Acceptance criteria

Eight criteria, each framed from the intent of a persona this initiative already names
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`, journeys
listed at `initiative.md:242-249`). AC-001 – AC-004 discharge project **AC-009**; AC-005 – AC-007
discharge project **AC-010**; AC-008 is the mount-fidelity criterion both depend on, because a
resolution that lands by overwriting a signed-off artifact satisfies neither.

There is no compiled test in this story and pretending otherwise would be worse than saying so: the
deliverable is two written resolutions and a directory. The verification column therefore names, for
each criterion, the **real artifact path the check reads** and the **command that gates the diff** —
`cargo xtask affected --base main` (`.redkiln/config.yaml:40`), which runs the five file-reading lints
and `spec-trace` unconditionally precisely so a prose-shaped story is still gated on something.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the application author on *Choose a contract before a database* needs this project's answer to "how is a reader told the log it is reading is incomplete" to be a decision rather than a scheduled intention, **WHEN** they open `.bklg/…/retention-and-incomplete-logs/_design.md`, **THEN** a top-level `## DT-7 — how a reader is told the log is incomplete` section states which of (a) one undifferentiated incompleteness signal or (b) the transient / benign-permanent / meaningful-permanent distinction is adopted, **names the option that lost**, and gives the reason it lost — a section naming a winner without naming a loser does not satisfy this, per the brief's own AC-009 row | static — content review of `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` § `DT-7 …`, read against `_decomposition.md:604-612` (AC-009 row) and `project.md:228-230`; diff gated by `cargo xtask affected --base main` |
| AC-002 | **GIVEN** the evaluator on *Decide in one sitting* must be able to see that the retention answer came from the evidence **this project built** and not from the charter's one-line framing, **WHEN** they read the DT-7 section, **THEN** both branches are weighed by explicit path citation against the recorded CF-27 pass list from `cf-27-experiment-and-recorded-pass-list` (what the suite can distinguish at all) **and** against the two reader observations from `decision-model-and-ingest-observed` (the confidently wrong `read_decision_model` fold; `IngestStore::holds` answering `false` for an event this store minted), **and** the losing option is refuted by name — the Ecotone production evidence if (a) wins, the reader complexity if (b) wins | static — content review of the DT-7 section's citations, each resolved against `.bklg/…/cf-27-experiment-and-recorded-pass-list/spec.md`, `.bklg/…/decision-model-and-ingest-observed/discover.md`, and `.bklg/from-contract-to-published-library/_discovery/research/07-retention-deletion-and-incomplete-log-semantics.md:98-107`; every cited path resolves in-tree |
| AC-003 | **GIVEN** the adapter author on *Learn when you are finished* would otherwise read a retention vocabulary and go looking for the channel that reports it, **WHEN** they reach the point in the DT-7 section where the adopted words are introduced, **THEN** the section has **already** told them no such channel exists — the statement precedes the vocabulary rather than following it — naming all three channel-less return shapes (`read_decision_model` → `(Vec<SequencedEvent>, Option<SequencePosition>)`, `crates/happenstance-core/src/store.rs:321-331`; `Guard::is_violated_by` → `bool` from a two-arm `match`, `crates/happenstance-core/src/append.rs:239-253`; `contains_event_id` → `Result<bool, _>` whose `false` conflates *"never had it"* with *"had it and forgot"*, `store.rs:268`) and stating the adopted shape is a **vocabulary a future primitive could carry**, never a signal a store reports today | static — content review for the ordering (statement before words) and for all three citations present; each line reference checked against the named source files. Rejects `ThreeKindsOfNothing` |
| AC-004 | **GIVEN** ES-39 rejects a floor because a regulated purge is *scattered, not a prefix* and *"ships looking correct until a claim runs long"*, and specifying a reporting shape would settle ES-39 by implication from an artifact no ADR authorises, **WHEN** the DT-7 section describes incompleteness, **THEN** it uses no boundary-position framing ("history below position N is gone") and specifies **no** reporting shape; and where the honest resolution would need one, that is recorded as the **AC-012 escalation** it is — pointing at DA-7's four-row option table (floor / retained ranges / third condition outcome / tri-state `contains_event_id`) and at `surface-diff-and-the-ac-012-escalation` — with no change made | static — content review against `spec/SPECIFICATION.md:4325-4349` (esp. `:4336-4341`) and `_decomposition.md:363-379`; plus `git diff --name-only` showing no change to `crates/happenstance-core/src/store.rs` or `crates/happenstance-core/src/append.rs`. Rejects `ThreeKindsOfNothing` and `OneSignalBecauseItIsFriday` |
| AC-005 | **GIVEN** the application author must know, *before* choosing this contract, whether an event's queryable tags can ever be erased for E2E-49's regulated-deletion case, **WHEN** they read the redaction answer in `_design.md`, **THEN** it answers **against the code**, each fact cited by path and line: `Tag` is `Tag(Cow<'static, str>)` whose hand-written `PartialEq`/`Eq`/`Ord`/`Hash` all delegate to `as_str()`, so tag equality **is** string equality with no digest field to swap in (`crates/happenstance-core/src/tag.rs:79`, `:170-193`); `EventParts` separates opaque `data` from queryable `tags`, so shredding reaches `data` and cannot reach `tags` (`crates/happenstance-core/src/event.rs`); and `EventStore` exposes no store-side update path (`crates/happenstance-core/src/store.rs`) | static — content review of the redaction section against `spec/E2E-CASES.md:1288-1311` (E2E-49) and `_decomposition.md:446-466` (DA-9); every cited `file:line` opened and confirmed to say what the section claims |
| AC-006 | **GIVEN** CF-38 makes a provisional claim with an empty falsifier a gate failure and AC-010 requires the deferral to name an experiment, **WHEN** the redaction answer is read and the `experiments/` path it names is opened, **THEN** the answer is an **explicit deferral** stating why — redaction needs either a new store operation (a published-surface change gate decision 4 forbids) or a change to what `Tag` equality *means* (a silent re-keying of every tag index, invisible to `cargo-semver-checks` because no signature moves) — **and** the named path resolves to a charter `README.md` that exists in the tree, **and** ES-37's `[FROZEN]` text is unchanged | static — `test -f` on the named `experiments/<name>/README.md`; content review of the deferral; `git diff` showing `spec/SPECIFICATION.md` untouched (`:4274-4297`). Rejects `DeferredToNothing` and `DigestTagsQuietly` |
| AC-007 | **GIVEN** a maintainer six months from now must be able to **run** the falsifier rather than re-derive it — the failure `experiments/wire-format/README.md` records, where *"measured, probe W5"* was a label and not a citation — **WHEN** they open the charter, **THEN** it states the hypothesis (a digest-keyed `Tag`), what it would measure including the tag-index migration cost, the **two registered rules** whose observable behaviour a tag-equality change would move — `query_item_tags_are_and` (`crates/happenstance-testkit/src/suite.rs:409`) and `query_item_tags_match_supersets` (`:439`), both in `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:112-113`) — what result would settle the question either way, and **in terms that it has not been run**; and the directory is neither a workspace member nor a gate step | static — content review of `experiments/<name>/README.md` against the two named rule sites (both resolve); `grep` of `Cargo.toml`'s `members` and `xtask/src/main.rs` showing the new directory in neither. Modelled on `experiments/position-visibility/README.md` and `experiments/wire-format/README.md` |
| AC-008 | **GIVEN** the repository owner approved `_design.md`'s no-surface determination on 2026-08-12 at the `/redkiln:plan` design gate and that approval must still mean exactly what it meant, **WHEN** this story's change is applied, **THEN** the DT-7 and redaction content is **appended** as new top-level sections **after** the existing `## Sign-off` block, carrying its own dated attribution to HS-S0120, with every line of the approved text (`:20-36`, `:86-95`) byte-identical and the CLI-owned frontmatter untouched; the added sections are composed in the artifact's own house shape — `##` sections with by-path citations, not a bare bullet dump — within the density budget (DT-7 ≤ 70 lines, redaction ≤ 50 lines, charter README ≤ 150 lines); and the diff touches nothing outside the PR boundary: no `.kb/` write, no `spec/` edit, no conformance rule, no published signature, leaving ADR-0028 to `adr-0028-and-the-open-question-wave` | static — `git diff .bklg/…/_design.md` showing only additions below `## Sign-off`; `git diff --name-only` against the PR boundary's fenced glob list; line counts of the two added sections and the charter; `cargo xtask affected --base main` green |

## Interaction quality

**RFC §6.7/D6, applied honestly to a story that renders no surface.** This project's signed-off
`_design.md` records **N/A — no user-facing surface** (`:38-40`, `:78-80`), and the sign-off
(`:90-95`) approved that determination together with the framing that DT-7 *"is resolved as a
vocabulary/documentation decision inside this project's own prose and ADR-0028 … not as any rendered
UI"* (`:25-30`). So the STATE family has no screen to constrain — and the correct move is not to invent
one, which would contradict the design a human already approved.

What the design **does** bind is composition in the artifact this story writes into, and those
invariants are real, checkable, and each carried by an AC row in the table above. Every invariant below
names the AC that carries it; **none is stated only here**, because `redkiln verify` extracts ACs from
the table's leading `| AC-### |` cell and a prose bullet in this section would never be gated.

| Family | Invariant, as it applies here | Carried by | How it is verified |
| --- | --- | --- | --- |
| STATE — in-place vs context-jump | The resolution lands **at the mount** — inside `_design.md`, the artifact `project.md:228-230` names by path — not in a new companion file the reader must jump to. A DT-7 answer written into this story's own `discover.md` or a fresh `_dt7.md` does not satisfy AC-009 | AC-001 | content review confirms the section exists in `_design.md` itself |
| STATE — non-occlusion | The addition **must not occlude** what is already there. Appended after `## Sign-off`, never interleaved into `:20-36` and never rewritten over the approval paragraph, so the approved text stays legible as approved text | AC-008 | `git diff` on `_design.md` shows additions only, below the sign-off block |
| STATE — preserved state (the sign-off's analogue of focus/selection) | The 2026-08-12 approval and its "Conditions: none" keep their meaning; the addition carries its **own** date and attribution rather than being backdated into the signed block | AC-008 | the appended section's attribution line names HS-S0120 and its own date |
| STATE — reversibility | Nothing here is destructive: the whole change is additive markdown plus one new directory outside the workspace, revertible by `git revert` with no schema, no lockfile and no published artifact touched | AC-008 | PR boundary + `Cargo.toml` `members` unchanged |
| STATE — reachability (the keyboard-reachability analogue) | The resolution must be **reachable from the places that look for it**: cited by path from this story's report and consumable by `adr-0028-and-the-open-question-wave` at `file:line`. A decision no downstream story can cite is a decision nothing reads | AC-008 | the ADR story's spec cites `_design.md` § DT-7 by path; both mount and section heading are stable |
| COMPOSITION — presentation exists at all | The added content is **composed**, not dumped: `##` headings in the artifact's own shape, prose with by-path citations in the style `_design.md:10-36` already uses. A bullet list of conclusions with no citations is the prose equivalent of bare markup | AC-008 | content review against the artifact's existing style |
| COMPOSITION — placement | Exactly two top-level sections, after `## Sign-off`. Not nested under an `N/A` section, and not spliced into `## Surfaces`, which correctly says there is none | AC-008 | heading level and position in `git diff` |
| COMPOSITION — transience (persistent vs revealed vs on-demand) | The **vocabulary-not-a-signal** statement is persistent chrome, not disclosure: it sits **before** the adopted words, so a reader cannot acquire the vocabulary without having already met its limit. Deeper material — the pass list, the reader observations, DA-7's option table — stays on-demand behind path citations, linked and not pasted | AC-003 (ordering), AC-002 (citations not paste) | content review of section ordering; citation resolution |
| COMPOSITION — density budget | DT-7 section ≤ 70 lines; redaction section ≤ 50 lines; charter `README.md` ≤ 150 lines. `_design.md` is 95 lines today, and a 300-line appendix bolted under a 95-line artifact reads as a different document wearing its filename | AC-008 | line counts on the diff |
| COMPOSITION — hierarchy | The **decision** is the top of each section — the option adopted, the option that lost, the reason — with evidence beneath it. Evidence-first ordering buries the thing AC-009 is read for | AC-001 | content review of section structure |
| COMPOSITION — the design's named anti-patterns | `_design.md:25-30` forbids resolving DT-7 as anything rendered, and `:32-36` scopes any public-API addition to other ACs entirely. So: no surface is invented, no `pub` item is added, and `## Items` gains no entry | AC-004 (no reporting shape specified), AC-008 (no published signature in the diff) | `git diff --name-only`; no change under `crates/*/src/` |

## Error conditions

| id | condition | required response |
| --- | --- | --- |
| EC-001 | The upstream evidence is **absent or thinner than assumed** — `cf-27-experiment-and-recorded-pass-list` has not landed its recorded pass list, or `decision-model-and-ingest-observed` recorded a gap instead of the two observations | **Halt and report**, do not proceed on intuition. AC-002 requires the resolution to be weighed against evidence that exists; a DT-7 answer written against a pass list that was never produced is `OneSignalBecauseItIsFriday` with a citation to nothing. `_storymap.md:141-145` sequences this slice fourth precisely so the evidence is in hand |
| EC-002 | The honest resolution **needs a reporting shape** — the chosen distinction is only meaningful if a store can say which kind of hole a reader met | Record it as an **AC-012 escalation** naming DA-7's row (`_decomposition.md:363-379`) and hand it to `surface-diff-and-the-ac-012-escalation`; make no port change. Specifying the shape here settles ES-39 from an unauthorised artifact (AC-004) |
| EC-003 | The honest redaction answer is concluded to **require amending ES-37** (`spec/SPECIFICATION.md:4274-4297`, `[FROZEN]`) | **Stop.** Do not edit the clause. Raise it as a finding routed by `project.md:112-114` to a new decision atom and a re-plan, and re-open this story's discover gate box rather than defending it (`discover.md:68`) |
| EC-004 | The chosen experiment directory **collides** with an existing one — `experiments/position-visibility/`, `experiments/wire-format/`, `experiments/rustc-ice-gat-foreign-trait/` | Create a new directory with a distinct name. Never append this charter into an existing experiment's README: those record measurements that were *run*, and this one states in terms that it has not been (AC-007) |
| EC-005 | An edit is attempted against `_design.md`'s or this spec's **YAML frontmatter** | The `PreToolUse` hook denies it, and correctly (`CLAUDE.md`, *The CLI is the only writer of an item's system frontmatter*). Author only beneath the closing `---`; drive any state change through `redkiln`, never by hand |
| EC-006 | A cited `file:line` **does not say what the resolution claims** — line numbers drifted, or the claim was inherited from a brief rather than read | Re-read the source and correct the citation before merge. Every normative sentence in both added sections carries a path, and a citation that does not resolve is the defect this whole project exists to make visible in code |
| EC-007 | A reviewer reads the DT-7 section and **still asks where the signal is reported** | The section failed AC-003 regardless of what it says elsewhere. Fix the ordering and the wording, not the reviewer |

## Non-functional

| id | requirement | why, and how it is bounded |
| --- | --- | --- |
| NF-001 | **Zero build and gate cost.** This PR adds no workspace member, no test target and no gate step; `cargo xtask ci --fast` wall time is unchanged | `experiments/` is not in `Cargo.toml`'s `members` and not an `xtask` step (`CLAUDE.md`, repository map; `experiments/wire-format/README.md` on why that isolation is load-bearing) |
| NF-002 | **Citation density.** Every normative sentence in the two added sections and in the charter carries a resolvable path — `file:line` for code and clauses, `path` for backlog artifacts | The failure `experiments/wire-format/README.md` documents: a label that reads as a citation and reproduces nothing. This story's whole output is claims about code it does not change, so the citation *is* the evidence |
| NF-003 | **Reading cost.** An adapter author reaching the vocabulary meets its limit in the same screen, not three sections later; the density budget in *Interaction quality* is the mechanism | Journey *Learn when you are finished* (`initiative.md:246-248`). A limit that is technically stated but 60 lines away is `ThreeKindsOfNothing` with a footnote |
| NF-004 | **Reproducibility of the deferral.** The charter is self-contained: hypothesis, method, the two rules, and the settling criterion all inside `experiments/<name>/README.md`, with nothing depending on a session scratchpad or on this backlog item surviving | `experiments/position-visibility/README.md`'s methods note and `experiments/wire-format/README.md:1-14` both record the cost of the alternative |
| NF-005 | **Immutability discipline.** No `.kb/` atom is written or edited by this story, and no accepted decision body is touched | `CLAUDE.md`: accepted decision atoms are immutable and are authored by `/redkiln:kb-ingest`; the hand-written attempt was reverted at `0269720` |
| NF-006 | **Semver silence is not safety.** The charter must state that a digest-keyed `Tag` would pass `cargo-semver-checks` unchanged while re-keying every tag index, so no later reader mistakes tool silence for a small change | `crates/happenstance-core/src/tag.rs:170-193` — the trait impls are hand-written and delegate to `as_str()`, so the break is in meaning, not in signature |

## Implementation notes (non-prescriptive)

These are observations for whoever implements this, not instructions. The decision itself is
deliberately **not** pre-made here — this story exists to take it against evidence, and a spec that
pre-writes the answer would make AC-002 unfalsifiable.

- **Read in this order.** The two upstream stories' recorded outputs first (the pass list, then the two
  reader observations), *then* the research file's three-mechanism section, *then* the three return
  shapes in `store.rs` / `append.rs`. Reading the research first tends to produce a resolution that
  argues with Ecotone rather than with this repository's own port.
- **The strongest form of either answer names its own cost.** If one signal wins, the section should be
  able to say what an operator loses when a transient out-of-order gap and a regulatory deletion arrive
  at a reader wearing the same face. If three win, it should be able to say what a reader pays to carry
  three words that nothing reports. A resolution that can only articulate its own advantages has not
  weighed anything.
- **A useful shape for the DT-7 section**, offered and not mandated: the adopted answer in one
  sentence; the limit (vocabulary, not signal, with the three return shapes) immediately after; the
  option that lost and the evidence it lost against; then what ADR-0028 is expected to do with this.
  That ordering satisfies AC-003's *before, not after* requirement structurally rather than by care.
- **Naming the experiment.** The directory name is the deliverable's most durable artefact — it is what
  ADR-0028 and any later ADR will cite. Prefer something that names what is *measured* over what is
  *hoped for*; the two existing precedents (`position-visibility`, `wire-format`) both name the subject
  under measurement rather than the outcome.
- **The charter is a charter, not a stub.** `experiments/position-visibility/README.md` is the closer
  model of the two — it carries schema, method and results directories around a question. This one
  carries no results by design, and should say why in its own first paragraph rather than leaving a
  reader to wonder whether the run failed.
- **Where the escalation goes if EC-002 fires.** DA-7's option table already enumerates the four
  candidate surfaces; the escalation's job is to say **which row**, and what version consequence it
  carries — not to re-derive the table.

## Tests and CI (merge gate)

Grounded in this project's testing brief (`_decomposition.md`, *Test mix* rows for AC-009 and AC-010 at
`:604-612`, and *Merge-gate commands*), which classifies both project ACs this story serves as **static
(content review)** tiers. That is not a weaker bar stated politely — it is the tier the brief chose,
with the wrong implementation each row exists to reject written next to it.

| Tier | Command / path | Proves |
| --- | --- | --- |
| Static — story grain, automatic | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The diff is gated on *something*. Runs the five file-reading lints and `spec-trace` unconditionally, which is the whole point for a story that maps to no workspace package. Rejects the assumption that a prose PR is ungated |
| Static — content review | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` § `DT-7 …` | AC-001 – AC-004. Read for: the option that lost, the reason, the evidence citations resolving, the vocabulary-not-signal statement present **and** ordered before the words, and no boundary-position framing. Rejects `ThreeKindsOfNothing` and `OneSignalBecauseItIsFriday` |
| Static — content review | `.bklg/…/retention-and-incomplete-logs/_design.md` § redaction | AC-005, AC-006. Read against `spec/E2E-CASES.md:1288-1311` and `_decomposition.md:446-466`, with every `file:line` opened. Rejects `DigestTagsQuietly` |
| Static — existence | `test -f experiments/<name>/README.md` | AC-006's falsifier is real rather than gestural. A named path absent from the tree is `DeferredToNothing` with a longer sentence — the failure CF-38 (`spec/SPECIFICATION.md:8318`) exists to make a gate failure |
| Static — citation resolution | `crates/happenstance-testkit/src/suite.rs:409`, `:439`; `crates/happenstance-testkit/src/registry.rs:112-113` | AC-007. The two rules the charter names exist, are registered in `for_each_event_store_rule!`, and are the ones a tag-equality change would move — so the deferral's falsifier is executable |
| Static — diff shape | `git diff --name-only` against the PR boundary's fenced glob; `git diff .bklg/…/_design.md` | AC-008. Additions only, below `## Sign-off`; nothing under `crates/`, `spec/` or `.kb/`; no frontmatter line changed |
| Static — density | line counts of the two added sections and the charter | AC-008's density budget (≤ 70 / ≤ 50 / ≤ 150). The composition invariant an unstyled prose dump would otherwise satisfy perfectly |
| Static — ledger | `redkiln verify --grain story` (`.redkiln/config.yaml`, the ledger clause) | Every AC-### in this spec has a row in `_ledger.md`, flipped only with cited evidence. The mechanism the runbook's *"a phase is done when its proof artefact exists"* needs |
| Integration grain — cheap tripwire | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | Nothing this story wrote broke a clause's cross-reference. Expected to be a no-op here, and expected to be **run**: an unexpected `spec-trace` failure means something touched `spec/` that should not have |
| Integration grain, non-terminal | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | The project-level bar `project.md`'s Definition of done names. Not this story's gate; run at the project's integration grain. The whole gate (`cargo xtask ci`) is HS-P0019's, per the brief |

**Not run here, and deliberately**: the named experiment itself. AC-007 requires the charter to state
it has not been run; running it would produce a measurement this story is not scoped to interpret and
would turn a bounded deferral into an unbounded one.

## Risks and coupling (PR-scoped)

| Risk | Mechanism | Mitigation in this PR |
| --- | --- | --- |
| **The schedule-position risk, by name.** This project is rank 5 and this slice is fourth within it; the cheaper answer is available and would look like a decision | `project.md:331-335` names it explicitly, and DR-9 is its counterweight: the refusal branch is not cheaper, because the instrument and the illustration are kept either way | AC-002 makes "weighed against the evidence, by path" a criterion rather than a hope; `OneSignalBecauseItIsFriday` is named in the Context pack as a wrong implementation the review looks for |
| **Prose passes every existing check.** No compiler, no conformance rule and no `spec-trace` assertion can read a `_design.md` section. Every named wrong implementation in this story is green under `cargo xtask ci` | `discover.md:46` — this is the defect class the project's whole subject is about, arriving in the project's own artifacts | The verification column names a human content review at a real path for each AC, and `_ledger.md` forces evidence to be cited per row rather than asserted in a summary |
| **Coupling to `adr-0028-and-the-open-question-wave` (slice-mate).** The two are implemented in one context; the temptation is to write the ADR's reasoning here, or this resolution's content only there | `_storymap.md:96`, `:141-145` — the ADR story lists this one in its `depends_on` and reads what it writes | The PR boundary excludes `references/adr/`, `.kb/_intake/` and `.kb/decisions/` entirely. If a sentence belongs in ADR-0028, it does not go in this PR |
| **Backward coupling to slices 1–3.** If the pass list or the reader observations are missing or contradict the assumption in DA-8, the resolution's evidence base moves | EC-001 | Halt and report, never substitute intuition. The slice ordering exists so this is discovered with slack remaining |
| **`experiments/**` is a wide glob in the PR boundary** | The directory name is decided in the writing, so it cannot be narrowed in advance | Bounded by what the glob can reach: nothing under `experiments/` is a workspace member (`Cargo.toml`) or a gate step, so the width cannot touch the compiler or the suite. EC-004 covers collision with the three existing directories |
| **Silent re-keying if the digest idea is mistaken for the answer** | `Tag`'s hand-written trait impls delegate to `as_str()` (`crates/happenstance-core/src/tag.rs:170-193`); changing equality's *meaning* moves no signature and `cargo-semver-checks` sees nothing | NF-006 requires the charter to state this in terms; the PR boundary forbids any change to `crates/happenstance-core/src/tag.rs`; `DigestTagsQuietly` is named as a wrong implementation |
| **The sign-off silently widening.** An appended section under an approved artifact can read, later, as having been approved | The approval's text covers the no-surface determination and the anti-patterns (`_design.md:90-95`), not DT-7's content | AC-008 requires the addition to sit **after** `## Sign-off` with its own dated attribution to HS-S0120 |

## Dependencies

**Blocks on** (`depends_on`, matching this story's item and `_storymap.md:74`):

- **`decision-model-and-ingest-observed`** (HS-S0118) — supplies the two reader observations AC-002
  weighs against: `read_decision_model` folding a confidently wrong answer over a retained suffix, and
  `IngestStore::holds` answering `false` for an event this store minted and acknowledged. Without them
  the DT-7 resolution has nothing to say about what a reader *actually receives*.
- Transitively through that story: **`retained-set-instrument-and-conformance-mount`** (the instrument)
  and **`cf-27-experiment-and-recorded-pass-list`** (the recorded pass list AC-002 also cites). Both
  precede slice 3 in the merge order (`_storymap.md:130-140`), so both are landed by the time this
  story opens; neither is listed as a direct edge, and neither is re-derived here.

**Unlocks**:

- **`adr-0028-and-the-open-question-wave`** (HS-S0121, same slice) — lists this story in its own
  `depends_on` (`_storymap.md:96`) and cites what this PR writes: the DT-7 resolution as decided
  content, and AC-010's answer, which the ADR *records* where the project AC asks for it. This story
  writes no ADR, no `.kb/_intake/` file and no `.kb/` atom.
- Indirectly, **`surface-diff-and-the-ac-012-escalation`** — if EC-002 fires, the escalation raised here
  is one of that story's inputs, naming the DA-7 row rather than re-deriving the option table.

## Anchors (progressive disclosure)

Load-bearing depth, deferred and not optional. The Context pack above is self-sufficient for starting;
open these at the moment named. Every path was confirmed present in the worktree before it was cited.

| Anchor (real path) | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` | The mount. Carries the no-surface determination (`:20-36`), the API-surface note (`:32-36`) and the signed approval (`:86-95`) this story appends beneath without editing | First, before writing a single line — it defines where the section goes and what must not move | AC-001, AC-008 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/cf-27-experiment-and-recorded-pass-list/spec.md` | The upstream that produced the recorded pass list — what the suite can distinguish between a pruned store and a young one. AC-002's first evidence leg | Before writing the DT-7 resolution's reasoning, and before EC-001 can be ruled out | AC-002 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/decision-model-and-ingest-observed/discover.md` | The direct `depends_on`. Records what a reader **actually receives** across a hole — the confidently wrong fold and the lying `holds()`. AC-002's second evidence leg, and the reason DT-7 is decided after the evidence rather than before it | Immediately after the pass list, before choosing between (a) and (b) | AC-002 |
| `.bklg/from-contract-to-published-library/_discovery/research/07-retention-deletion-and-incomplete-log-semantics.md` | The production evidence for three kinds (Ecotone's taxonomy of *why* a position is missing, `:98-107`) — the thing option (a) must overrule by name if it wins | When drafting the losing option's refutation, third and not first | AC-002 |
| `crates/happenstance-core/src/store.rs` | The channel-less return shapes: `read_decision_model` at `:321-331`, `contains_event_id` at `:268`. AC-003's three citations must be read, not inherited from this spec | While writing the vocabulary-not-signal statement — open the file and confirm the lines | AC-003 |
| `crates/happenstance-core/src/append.rs` | `Guard::is_violated_by` at `:239-253` — the two-arm `match` with no third arm, the third channel-less shape | Same moment as `store.rs`, and again if EC-002 is being considered | AC-003, AC-004 |
| `spec/SPECIFICATION.md` | ES-39 at `:4325-4349` (esp. `:4336-4341`) supplies the scattered-not-a-prefix argument the vocabulary must not contradict; ES-37 at `:4274-4297` is the `[FROZEN]` clause the redaction answer must leave untouched; CF-38 at `:8318` is the empty-falsifier prohibition | ES-39 before writing the DT-7 section's framing; ES-37 before writing the redaction deferral. Read-only in both cases — no edit belongs in this PR | AC-004, AC-006 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md` | DA-7's four-row escalation option table (`:363-379`), DA-8's tri-state reading (`:381-397`), DA-9's redaction reading (`:446-466`), and the AC-009/AC-010 test-mix rows (`:604-612`) | DA-9 before the redaction section; DA-7 only if EC-002 fires; the test-mix rows when writing the report's evidence | AC-004, AC-005, AC-007 |
| `crates/happenstance-core/src/tag.rs` | `Tag(Cow<'static, str>)` at `:79`, the hand-written `PartialEq`/`Eq`/`Ord`/`Hash` at `:170-193`. The whole redaction answer and the whole semver-silence warning rest on these lines saying what the spec claims | Before writing the redaction answer — this is the file the answer is *against* | AC-005, AC-006 |
| `crates/happenstance-core/src/event.rs` | `EventParts`' separation of opaque `data` from queryable `tags` — why shredding reaches one and not the other | Same reading pass as `tag.rs` | AC-005 |
| `spec/E2E-CASES.md` | E2E-49 at `:1288-1311` — the concrete regulated-deletion case the redaction answer is owed to. An answer that never names its case is an abstraction | When drafting the redaction section's opening, so the answer is anchored to a scenario | AC-005 |
| `experiments/wire-format/README.md` | The precedent for an `experiments/` directory: why isolation from `Cargo.toml`'s `members` and from the gate is load-bearing, and what an unreproducible "measured" label cost (`:1-14`) | Before creating the new directory — copy the isolation discipline, not the results structure | AC-006, AC-007 |
| `experiments/position-visibility/README.md` | The closer structural model: a question with schema, method and a settling criterion around it. This charter is that shape with the results section deliberately empty | While writing the charter's structure | AC-007 |
| `crates/happenstance-testkit/src/suite.rs` and `crates/happenstance-testkit/src/registry.rs` | `query_item_tags_are_and` (`suite.rs:409`) and `query_item_tags_match_supersets` (`:439`), both registered at `registry.rs:112-113`. These two names make the deferral's falsifier executable rather than gestural | When naming the falsifier in the charter — open both and confirm the rules exist and are registered | AC-007 |
| `.bklg/from-contract-to-published-library/_decomposition.md` | Gate decision 4 at `:239-249` — the no-published-surface-change constraint that bounds both halves of this story | If any answer starts to look like it needs a port change; re-read before writing an escalation | AC-004, AC-006 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md` | AC-009 (`:228-230`) and AC-010 (`:231-234`) verbatim — the two project criteria this story discharges — plus the `[FROZEN]` routing rule at `:112-114` and the schedule risk at `:331-335` | At the start, to read the ACs in their own words, and at the end, to check the resolution against them | AC-001, AC-006 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The personas and journeys the acceptance criteria above are framed from — *Learn when you are finished*, *Choose a contract before a database*, *Decide in one sitting* | When judging whether the prose serves a reader or only a reviewer | AC-003, AC-005 |
| `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md` | Row `:74` (this story), row `:96` (the ADR story that consumes it), merge order `:141-145` (this slice cannot open until slices 1–3 land) | When confirming the handoff boundary and that the upstream evidence is in | AC-002, AC-008 |
| `.redkiln/config.yaml` | The `verify:` block — `affected_gate` at `:40`, `reachability_static` at `:48`, `integration_scoped` at `:55`, and the ledger clause. These are the commands that fire automatically, not a parallel checklist | Before claiming any AC in the ledger | AC-008 |

## Clarifications resolved during spec

1. **The eight AC ids the front half decided are enumerated unchanged.** AC-001 – AC-008, none added,
   none dropped. The mapping is AC-001 – AC-004 → project AC-009, AC-005 – AC-007 → project AC-010,
   AC-008 → both (a resolution that overwrites a signed-off artifact discharges neither).
2. **The composition invariants live on AC-008 rather than on a ninth AC.** *Interaction quality*
   requires every applicable invariant to be a table row, and the presentation, placement and density
   invariants are all one thing — the mount is faithful — so they were folded into AC-008's criterion
   rather than split into a new id the front half did not declare. The transience and hierarchy
   invariants sit on AC-003 and AC-001 respectively, where they were already load-bearing.
3. **The STATE family is answered, not skipped.** `_design.md` records no user-facing surface and that
   determination is signed off, so there is no screen to constrain — but the *analogues* (in-place,
   non-occlusion, preserved state, reversibility, reachability) all apply to the artifact this story
   writes into, and each is bound to an AC in the table. Writing "N/A — no surface" and stopping would
   have left the sign-off unprotected, which is the one thing this mount can actually get wrong.
4. **The DT-7 answer itself is deliberately not pre-decided in this spec.** The story exists to take the
   decision against slice 1–3's evidence; naming a winner here would make AC-002 unfalsifiable and would
   be `OneSignalBecauseItIsFriday` committed one stage earlier. What the spec fixes is the *shape* of an
   acceptable answer — loser named, evidence cited, limit stated first.
5. **The experiment's directory name is not fixed here either**, and that is why the PR boundary carries
   `experiments/**` rather than a single path. AC-006 and AC-007 constrain what the charter must contain
   and that the path must resolve; EC-004 constrains collision with the three directories that already
   exist. Implementation notes offer a naming heuristic, not a name.
6. **The verification column names content review at real paths, not invented test ids.** Both project
   ACs are classified `static (content review)` by this project's own testing brief (`:604-612`); the
   honest verifying instrument is the artifact path plus the diff-shape and existence checks, gated by
   `cargo xtask affected --base main`. A fabricated `#[test]` name would have made the ledger look
   stronger and been unresolvable at implement time.
7. **AC-007's "has not been run" is a positive requirement, not an omission.** The charter must say so
   in terms. A README that simply has no results section reads as a run that failed or was abandoned,
   which is the ambiguity `experiments/wire-format/README.md:1-14` records the cost of.
