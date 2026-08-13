---
item: HS-S0003
stage: discover
created: 2026-08-12T13:01:11.706Z
updated: 2026-08-12T13:01:11.706Z
template_sig: 86ce4036
rendered_sig: 3a67b9bb
---

# Discover — DT-3 and DT-8 resolved in the public-API design record

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: resolve DT-3 (one authoritative source for "this guarantee does not apply", citing CF-40's atom rather than minting a second declension policy) and DT-8 (whose adapter-author bar the suite holds) in `_design.md`, alongside the public projection API surface review the repurposed template asks for — signatures, visibility, what the shape costs a caller, and a doctest in place of a mock | `../_storymap.md:55` | Two decisions plus one API-surface review, all landing in one artefact. The review half is what `.redkiln/templates/_design.md` exists for in a library |
| **AC-006** — DT-3 resolved with **one** source named authoritative, citing `.kb/open-questions/cf-40-fixture-limits-ownership.md` rather than restating it | `../project.md:198-200` | "One authoritative" is the testable half; "cites rather than restates" is the half that stops a second policy being minted by paraphrase |
| **AC-007** — DT-8 resolved. If the bar is held for an outside author, a documented extension surface exists and AC-002 and AC-005 are demonstrated against a fixture written from the documentation alone | `../project.md:201-203` | The arm chosen here decides whether `documented-extension-surface` (HS-S0015) has a fixture to build or a scope statement to verify. This story is the fork; that story is the discharge |
| `dependsOn: projection-decision-atoms` — supplies the accepted ADR-0017/0018/0019 vocabulary the API surface review describes, and the `LiveHandleProjectionStore` disposition | `../_storymap.md:54-55,107` | The design record describes a shape three accepted atoms have already justified. Reversing the order would put the review upstream of its own rationale |
| **The `_design.md` that exists today records "No user-facing surface" and `N/A` in every section — Items, Signatures, Shape decision, What it costs a caller, The doctest — and carries no DT-3 or DT-8 resolution.** It is signed off, at the `/redkiln:plan` design gate, as the no-surface determination plus anti-patterns | `../_design.md:52-101` | **This is the gap this story closes.** The sign-off is real and correct on its own terms (there is no screen, and `design.capture` is undeclared), but AC-006, AC-007 and DoD 7 name content that artefact does not yet contain |
| DoD 7: "the public projection API surface has passed the design-stage review under the repurposed `.redkiln/templates/_design.md` — signatures, visibility, what the shape costs a caller, and a doctest in place of a mock" | `../project.md:252-254`; `CLAUDE.md`, *Where the work lives* | The template's whole point in this repo: "every other check in this repository is satisfied by an API that is correct and unusable". The `N/A` rows are the ones DoD 7 is about |
| UX brief **AC-U01** — DT-3's answer must dispose of **all three** reason-writers already in the tree: the fixture-written reason (`Capability::declined`), the testkit-written reason for a fact no adapter should paraphrase (`NO_CEILING_REASON`), and per-adapter prose | `../_decomposition.md:84-96`; `crates/happenstance-testkit/src/contract.rs:412,442,454` | "Names one authoritative source without disposing of the other two" is the named failure. `NO_CEILING_REASON`'s own doc calls itself "the one place the skip machinery here differs" |
| UX brief **AC-U02** — DT-8's answer must state its cost to an author this repository did not write: the extension surface is `projection_store_conformance!` + `ProjectionProbe`, and its cost must stay one feature flag on a dependency the adapter already has, with no new edge in the graph | `../_decomposition.md:97-108`; `spec/SPECIFICATION.md:5016-5025` | This is why `ProjectionProbe` is specified into `happenstance-core` rather than the testkit — the coherence argument is already written and must not be re-derived |
| UX brief **AC-U07** — `ResetError::Refused` carries no payload, so `_design.md` either gives the variant the store's stated reason or records the decision to leave it bare. "Silence here is the failure, not a passing default" | `../_decomposition.md:148-157`; `spec/SPECIFICATION.md:4676-4682` | A third thing this artefact owes beyond DT-3 and DT-8, and the one most likely to be skipped because no AC numbers it |
| Testing brief: AC-006 is **"None (design-stage artefact)"** and AC-007's narrower arm is **Static**. "Named here only so no story tries to invent a test for it" | `../_decomposition.md:776-777`; `:893-900` | There is no executable falsifier for this story. The instrument is the design-stage review, which makes the wrong implementation below the only guard there is |
| Project risk: "CF-40's ownership is claimed and disclaimed by one decision. DT-3's answer here should not silently mint a second, divergent declension policy for the projection fixture — one policy, or a stated reason for two" | `../project.md:322-325`; `.kb/open-questions/cf-40-fixture-limits-ownership.md` | The forbidden outcome is stated in the charter, in these words. The spec should quote it rather than paraphrase |
| Architecture brief Note 5: which capability constants the projection fixture declares, and whether any is a MUST in `SECOND_HANDLE`'s sense, is **`_design.md`'s call** | `../_decomposition.md:517-534`; `crates/happenstance-testkit/src/contract.rs:161,173,207` | A fourth deliverable, and the one `projection-capability-skips` (HS-S0008) consumes directly — its storymap row says the capability set is "fixed by DT-3's recorded resolution rather than invented here" (`../_storymap.md:60`) |
| UX brief, *What would make this brief wrong*: if the projection fixture ends with **no declinable capability at all**, AC-U08 – AC-U12 are decorative on this port — "that outcome is not a licence to quietly drop the reporting discipline: it is a finding, and it belongs in DT-3's resolution" | `../_decomposition.md:252-259` | A capability set with nothing declinable would make the initiative's AC-05 undemonstrable by the project that owns it. That must surface here, not at the gate run |

## Questions

Open questions to resolve before specifying.

1. **Is the existing signed-off `_design.md` amended, or does this story write a
   companion?** Deferred to `spec` — but the constraint is answered: AC-006 and
   AC-007 both say "in `_design.md`", so wherever the prose lands it must be
   reachable from that file, and the existing sign-off's scope ("what was
   approved is the no-surface determination itself", `../_design.md:96-101`)
   must not be read as approving content it does not contain.
2. **Which of the three reason-writers is authoritative for DT-3?** Deferred to
   `spec` by design — this story's discover stage fixes the *bar* (all three
   disposed of, CF-40's atom cited not re-litigated), not the answer. Choosing
   it here would settle it without the API surface review that is supposed to
   inform it.
3. **Which DT-8 arm?** Deferred to `spec`. The consequence is recorded now so
   the choice is made with its price visible: the outside-author arm obliges
   `documented-extension-surface` to build a fixture from documentation alone;
   the narrower arm obliges the recorded scope to appear where an outsider meets
   it — the testkit's own crate doc — and not only in `_design.md`
   (`../_decomposition.md:103-107`).
4. **Which capability constants does the projection fixture declare, and is any
   one a MUST?** Deferred to `spec`, and flagged as a *deliverable of this
   story* rather than of `projection-capability-skips`, because that story's
   own one-line defers to this one.
5. **Does `ResetError::Refused` carry the store's reason?** Deferred to `spec`,
   with the standing constraint that silence is not an allowed answer.
6. **Is `ProjectionId::new`'s infallibility revisited?** Answered: no
   (Architecture brief AC-A09, `../_decomposition.md:331-335`). AC-U03's
   illegal-states-unrepresentable argument is scoped to `Checkpoint` and the two
   error enums and is not a mandate to harden the identifier.

## Decision

Two questions decide what this port *says to a human*, and neither has an owner
until this slice: how a consumer learns that a guarantee does not apply to their
implementation (DT-3), and whose adapter-author bar the suite is held to (DT-8).
Both are unanswerable by a test — resolving them is choosing which documented
behaviour the suite commits to, and a test can check that the chosen behaviour
holds but not that the right behaviour was chosen. Alongside them sits the API
surface review DoD 7 owes and the project's `_design.md` currently records as
`N/A`: signatures, visibility, what the four new types cost a caller, and a
doctest in place of a mock. The spec will fix the artefact's shape and its bars:
DT-3's resolution names exactly one authoritative source **and** disposes of the
other two reason-writers already in the tree, citing
`.kb/open-questions/cf-40-fixture-limits-ownership.md` as authoritative rather
than re-litigating it; DT-8's resolution names its arm and states that arm's cost
to an author this repository did not write; the projection fixture's capability
set is enumerated with each constant's MUST/SHOULD status; `ResetError::Refused`
either gains the store's reason or records the decision to stay bare; and the
signature block covers `Checkpoint`'s three variants, the `CommitError`/`ResetError`
split, and the non-`async` infallible `begin`, each naming the alternative that
lost. No new ADR is required — the three from `projection-decision-atoms` are the
decision record; this is the surface review over them.

## The wrong implementation

**A `_design.md` that names the suite's output authoritative for DT-3 and stops
there.** It reads well, it satisfies AC-006's literal words ("one source named
authoritative"), it cites CF-40's atom, and it passes every check this repository
can run — because no check reads it. It is wrong because `NO_CEILING_REASON`
already exists as a *testkit-written* reason for a fact no adapter should
paraphrase, and `Capability::declined` already exists as a *fixture-written* one
(`crates/happenstance-testkit/src/contract.rs:412,454`). A resolution that does
not say which of those defers to which has minted the second divergent declension
policy the charter forbids in those words — "one policy, or a stated reason for
two" (`../project.md:322-325`) — and the projection fixture will inherit the
ambiguity as its default. UX brief AC-U01 is the bar, and the design-stage review
is the only instrument.

**The DT-8 version, and it is the more expensive one:** taking the outside-author
arm in `_design.md` and recording it nowhere an outsider looks. The suite's bar is
then internal-only in fact and unqualified in public, which is the exact failure
DT-8 names — "a suite's own permissiveness becomes publicly scrutinised the moment
outsiders make claims with it" (`../_decomposition.md:105-107`, quoting the
initiative charter's DT-8). Nothing fails; a reader of
`happenstance-testkit`'s crate doc is simply told nothing, and
`documented-extension-surface` later builds its fixture from documentation that
was never written. The guard is AC-U02's cost statement plus AC-007's conditional
fixture, which is HS-S0015's whole job — that story is the mutant for this one.

**The quiet one:** an API surface review that lists the four new types'
signatures and declares them fine, with the `N/A` rows filled in and no
alternative named. `cargo check` is green at every call site — it would be green
for `ProjectionError<E>` merged from `CommitError` and `ResetError` too, and for
`(Option<SequencePosition>, bool)` in place of `Checkpoint`, which can spell
`(None, true)` — "authoritative, never run — which means nothing"
(`spec/SPECIFICATION.md:4643-4658`). The compiler cannot tell a shape that costs
a caller nothing from one that forces them to match arms their call cannot
produce. Only the review can, and only if the record names the alternative that
lost at each decision (AC-U03 – AC-U05).

**The finding that must not be swallowed:** if the capability set this story
fixes ends with nothing declinable, then AC-U08 – AC-U12 are decorative on this
port and the initiative's AC-05 cannot be demonstrated by the project that owns
it. That is a recorded finding in DT-3's resolution
(`../_decomposition.md:252-259`), not a reason to drop the reporting discipline
and not something to discover at `whole-gate-run-and-proof-artefact`.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.

The two judgement boxes. **Literal positions**: this story adds no conformance
rule; its artefact is prose plus a doctest. Vacuously true. **`[FROZEN]`
clauses**: none is touched. DT-3 and DT-8 are initiative-level design tensions,
not specification clauses, and the capability set this story fixes is testkit
surface rather than `PS-*` text. The eighth box is vacuous for the same reason
the sixth is.
