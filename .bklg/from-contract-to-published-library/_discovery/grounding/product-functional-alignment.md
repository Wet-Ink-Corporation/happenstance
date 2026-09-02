---
title: Product / functional alignment — From Contract to Published Library
kind: grounding/summary
---

# Product / functional alignment

Scope: where this initiative sits against Redkiln's existing *functional* and
product knowledge — `.kb/maps/domain-map.md`'s domains, `.kb/concepts/`,
`.kb/product/`, `.kb/design/`, and the open questions already on file. No
architecture, tech choice or design is proposed here; that is out of scope for
this pass and for the brief it grounds
(`.bklg/from-contract-to-published-library/_intake-brief.md:84-87`).

## Headline finding: the product/design layers are structurally present and functionally empty

`.kb/product/README.md` and `.kb/design/README.md` each define the layer's
contract (personas + journeys; resolved interaction patterns) but neither
directory holds a single populated atom — both are template-only. There is no
existing persona, journey or design atom in this knowledge base for this
initiative to reinforce, extend or contradict. That is itself a fact worth
recording rather than a gap in this search: `.kb/product/README.md:26-31`
states the model directly — an initiative's own discovery keeps its personas
in `_discovery/distillation/personas-and-journeys.md` until `/redkiln:closeout`
promotes them, so "nothing here yet" is the expected state for the *first*
initiative to touch this territory, not a sign the search missed something.

The closest thing to evidenced personas anywhere in the repository is the
seed's own "Who it is for" section
(`references/seeds/remaining-runway.md:79-91`), carried into the approved
intake brief verbatim in substance: **application authors** doing DCB event
sourcing in Rust with no storage-agnostic option today, **adapter authors**
(including this project's own) who need an executable definition of "correct"
rather than prose, and **the local-first / edge case specifically**, named as
load-bearing rather than a nice-to-have because the `!Send` port flavour is
carried at real cost to keep a store runnable inside a Cloudflare Durable
Object on `wasm32` (`.bklg/from-contract-to-published-library/_intake-brief.md:69-71`).
None of these three is yet a promoted `.kb/product/` atom. This initiative is
a plausible first candidate to promote them — but per the product layer's own
rule, that is a closeout decision, not something this grounding pass should
pre-empt.

The design layer is correctly out of scope for the whole initiative, not just
absent by coincidence: the intake brief states `userFacing` is **false** — "a
library with no screen" — and deliberately withholds the standing
`interaction-pattern-prior-art` research angle
(`.bklg/from-contract-to-published-library/_intake-brief.md:201-203`), which
matches `CLAUDE.md`'s own note that `design.capture` is absent from
`.redkiln/config.yaml` for exactly this repository (the perceptual review is a
skip, not a silent pass, because "there is no app to screenshot").

## Functional domains touched, per `.kb/maps/domain-map.md`

Two domains exist today (`.kb/maps/domain-map.md:37-142`), and this initiative
sits inside both rather than opening a third:

- **"Specification governance & conformance"** — whether the specification's
  prose, its assigned conformance rules, and the code agree, and what a pass
  does when it finds they do not (`.kb/maps/domain-map.md:39-43`). This
  initiative's desired outcome #3 — "every provisional clause audited against
  the ledger rather than against prose" at publish
  (`.bklg/from-contract-to-published-library/_intake-brief.md:44-45`) — is a
  direct, functional continuation of this domain's own concern, extended past
  the specification document itself to a published artefact a real consumer
  can rely on.
- **"Contract ports, conformance, and the ADR corpus"** — `happenstance-core`'s
  port design, the conformance suite, and the seventeen-ADR record
  (`.kb/maps/domain-map.md:82-90`). This initiative's outcomes #1 and #2 — the
  contract "has been used" by a typed layer and worked example, and adapters
  have passed the suite "across shapes that genuinely disagree"
  (`.bklg/from-contract-to-published-library/_intake-brief.md:36-42`) — sit
  squarely inside this domain's stated concern (`the conformance suite that
  proves an adapter against it`) but push it from *specified* to *proven by
  contact*, which is exactly the gap the seed names: "a `todo!()` body
  type-checks against any signature at all... every skeleton-based proof to
  date has been decorative" (`.bklg/from-contract-to-published-library/_intake-brief.md:145-146`).

No new functional domain is named in the map for "publication," "adoption," or
"what a consumer relies on" — this initiative's desired outcomes 1, 3 and 4 sit
at the edge of what the two existing domains cover (they are about a contract
being *used* and *promised*, not just specified and tested in isolation). That
is worth flagging for planning/distillation rather than resolving here: whether
this initiative's closeout opens a new domain section on `domain-map.md` (a
domain a map atom, not this document, would create) is a decomposition
question the brief explicitly defers (`_intake-brief.md:84-87`).

## Concepts reinforced

`.kb/concepts/torn-reads-and-the-append-condition-boundary.md`
(`kb-concept-torn-read-append-boundary-001`) is the one populated concept atom
in the corpus, and it is functional rather than purely mechanical in what it
describes: a consumer that reads a torn slice of the log gets a **silently
accepted** append instead of a rejected one — "the failure mode is silent data
loss on commit... the more dangerous of the two directions, because nothing
about the append's success signals that anything was wrong"
(`.kb/concepts/torn-reads-and-the-append-condition-boundary.md:56-59`). That is
precisely the class of defect this initiative's desired outcome #1 is built to
surface — "the defects that use discovered are recorded," with `trybuild`
compile-fail coverage named as one shape of proof
(`.bklg/from-contract-to-published-library/_intake-brief.md:36-38`) — and
outcome #2's insistence on stores that disagree on who assigns positions and
when (`happenstance-postgres`, outside the transaction) is a direct functional
test of whether the guarantee this concept describes survives contact with a
store where the read/append boundary the concept relies on is genuinely harder
to hold. This initiative **reinforces and stress-tests** the concept; it does
not contradict it.

## Decisions reinforced

`.kb/decisions/0006-bare-name-to-the-typed-layer.md` (`kb-decision-0006`) is
the one accepted decision framed around who a crate is *for*, functionally,
rather than how it is built: `happenstance-core` is "what an adapter author
pins," `happenstance` is "what an application `cargo add`s... gives an
application what it actually wants"
(`.kb/decisions/0006-bare-name-to-the-typed-layer.md:64-66,83`). Today that
promise is aspirational — the crate an application actually installs is a
"five-line facade," a glob re-export
(`crates/happenstance/src/lib.rs:75`, cited at
`.bklg/from-contract-to-published-library/_intake-brief.md:24-25`). This
initiative's outcome #1 is the functional fulfillment of ADR-0006's premise:
it is what makes "an application `cargo add`s `happenstance` and gets what it
actually wants" true by contact rather than by naming decision. No tension
here — this is the decision the initiative is, in product terms, cashing in.

## Open questions this initiative's clauses touch, read functionally

The brief's own clause ledger (`_intake-brief.md:159-185`) cites several
`.kb/open-questions/` atoms already on file. Read for their *functional* stake
— what an application author, adapter author, or edge deployment actually
experiences — rather than their mechanism:

- `kb-open-question-ps-1-no-progress-obligation-001` — PS-1's `MUST` currently
  permits a projection `commit` to return `Ok` while advancing neither the
  read-model row nor the checkpoint: "atomic and useless"
  (`.kb/open-questions/ps-1-states-no-progress-obligation.md:44-46`).
  Functionally, this is whether an application author's projection can report
  success while silently making zero progress forever — not an edge case, a
  correctness promise an application depends on without knowing it is unmade.
- `kb-open-question-global-vs-boundary-visibility-001` — whether ES-10's
  visibility invariant needs to be global; if the projection checkpoint turns
  out to be boundary-scoped instead, a projection runner can permanently skip
  an event "with no error anywhere"
  (`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md:55-56`).
  Functionally: whether an application author's read model can go quietly and
  permanently wrong, and whether they'd ever find out.
- `kb-open-question-human-readable-encoding-limits-001` — WF-11's
  human-readable (JSON) payload encoding requires materialising the whole
  payload in memory; the open question is whether a memory-constrained edge
  peer can ever forward one at all
  (`.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md:65-71`).
  This is the clause most directly tied to the brief's named "local-first and
  edge" audience — it is the first place a memory ceiling and a real payload
  meet in this workspace's own plan.
- `kb-open-question-cf-40-ownership-001` — whether a fixture (and by extension
  a store adapter) can even *declare* the numeric ceilings
  (`MAX_EVENT_DATA_LEN`, etc.) it enforces
  (`.kb/open-questions/cf-40-fixture-limits-ownership.md:33-37`). Functionally:
  whether an application author who exceeds a real store's limit gets a named
  error or an adapter that cannot describe its own ceiling in the first place.
- `kb-open-question-sync-message-set-undesigned-001` and the five `[DEFERRED]`
  `SY-*` clauses — replication is explicitly **not** this initiative's to
  settle by contact; the brief carries it as an open question owed "a decision
  or an explicit, reasoned refusal," not silence
  (`_intake-brief.md:46-47,95-97`).

These are read here for their functional stake only; which (if any) this
initiative's projects actually discharge is a decomposition question for
`/redkiln:plan`, per the brief's own non-goals.

## Tensions and boundary risks

- **No adjacent initiative to check for functional overlap.** `.bklg/` holds
  exactly one sibling item, `support` (`.bklg/support/initiative.md`), still
  at `stage: intake` with its Summary/Outcomes sections unfilled — there is no
  populated initiative to compare against. `.bklg/_archive/` does not exist in
  this worktree, so there is no legacy initiative corpus to reconcile either.
  This is a clean-search result, not an omission.
- **The KB's existing framing is implementer-language even where the stake is
  functional.** Every populated `.kb/open-questions/` atom is written in
  clause-id and mechanism terms (PS-1, ES-10, WF-11) rather than in
  who-is-affected terms. The brief's own "Who it is for" section works to
  restate that stake in audience terms; later distillation should keep doing
  that translation deliberately rather than let planning reason straight from
  clause IDs and lose the audience the brief names.
- **Promotion timing for personas.** The three audiences named in the brief
  (application author, adapter author, local-first/edge) are real and
  evidenced by the seed and the ADR corpus, but none is a `.kb/product/` atom
  yet. Whether *this* initiative is the one whose closeout promotes them is a
  closeout-stage call (`.kb/product/README.md:19-21`), not a grounding-stage
  one — noted here so distillation does not quietly invent competing personas
  from scratch instead of reusing what the seed already evidenced.

## Vocabulary worth carrying forward

- **Adapter author** vs. **application author** — the two audiences ADR-0006's
  crate split is built around (`.kb/decisions/0006-bare-name-to-the-typed-layer.md:64-66`);
  distinct from "consumer," which the seed uses more loosely for whichever of
  the two discovers a contract defect by using it.
- **Conformance suite** as *the executable definition of correct* — the seed's
  own phrase for why adapter authors are named as an audience at all
  (`references/seeds/remaining-runway.md:85-86`): a bar to build against
  instead of a prose specification to interpret.
- **Declined-capability path** — the mechanism (`Capability` associated
  constants on a `Fixture`) by which an adapter tells a consumer a guarantee
  does not apply to it, rather than silently failing to meet it; functionally
  the same shape CF-40 wants extended to numeric limits
  (`.kb/open-questions/cf-40-fixture-limits-ownership.md:31-37`).
- **Provisional / frozen** — the clause maturity vocabulary
  (`spec/SPECIFICATION.md`, cited throughout `.kb/decisions/`) that is, in
  functional terms, the strength of promise a consumer can currently rely on;
  this initiative's outcome #3 is what converts a provisional clause into one
  audited and either frozen or demoted at the moment of publish.
- **Torn read** — a read that observes an inconsistent slice of the log
  (`.kb/concepts/torn-reads-and-the-append-condition-boundary.md`); the
  functional failure mode is silent data loss on commit, not a retry.
