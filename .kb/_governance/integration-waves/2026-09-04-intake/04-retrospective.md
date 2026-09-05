# Wave `2026-09-04-intake` — retrospective

## What merged vs. created

Two amendments, twelve creates. `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` is
this corpus's **first playbook merge** in eight waves — every prior second instance of a playbook's
*shape* got a `related` link rather than a merge, and this one qualifies because it is a second
instance of the exact defect the playbook exists to prevent, in the same repository, under the
same lifecycle rule (`02`, Adjudication 5). `.kb/open-questions/projection-store-in-adapter-
default-features.md` is this layer's third resolution-by-amendment, following `cf-40` and the
`deny.toml` question, and the third time no sibling atom was minted because none was available:
ADR-0036 already committed the gate, so recording that two adapters now honour it commits nothing
new (`02`, Adjudication 12). Full reasoning for every placement is in
`02-placement-and-adjudication.md`'s fourteen operations; `03-integration-summary.md` is the
roll-up.

**What this wave does to the accepted decision corpus: nothing.** One decision atom is added
(ADR-0037). No accepted body is edited, no `status` is flipped on any existing decision, and no
supersession is signed — third wave running. Five accepted decisions are named by incoming claims
(`kb-decision-0004`, `kb-decision-0029`, `kb-decision-0022`, `kb-decision-0015`,
`kb-decision-0036`) and every one of them is linked from the new atom's side only.

Two structural notes: `.kb/governance/` took its second atom ever, eight waves after being
scaffolded with one. `.kb/open-questions/` took six new atoms in a single wave — its largest
single-wave addition — reflecting that this staged review surfaced six live forks rather than
resolved conclusions.

## Unresolved claims

Three findings were surfaced and deliberately left unresolved, per the placement's own "five
things this wave deliberately does not do" (`02`, closing section) — not fixed here, not lost
either:

- **ADR-0015 is named as needing a superseding atom and does not get one.** The review states in
  terms that a floor for `Event::metadata` "needs a superseding atom" and supplies none of the
  three candidate answers. `kb-open-question-event-metadata-no-floor-001` (Op 8) records the gap
  and chooses nothing among them.
- **Two of ADR-0022's own re-open falsifiers have fired, and the firing is recorded outside it.**
  `kb-open-question-adr-0022-falsifiers-fired-001` (Op 13) is the record; ADR-0022 itself is
  untouched, on the reasoning that writing a fired falsifier into the decision that promised to
  re-open would make an immutable record silently informative about knowledge it didn't have when
  signed (`02`, Adjudication 4).
- **Whether a fixture contract needs a declared tolerance for transient contention** —
  `kb-open-question-testkit-contention-tolerance-001` (Op 10) — was scored at 55 against folding
  into the ADR-0022 falsifier question and refused as this wave's most contestable call: same
  root observation (`busy > 0` at 64 contenders), different owner, different answer shape,
  different lifetime (`02`, Adjudication 8).

Three known code/doc defects surfaced by the review are explicitly **not** repaired here and carry
no KB atom of their own, per `open-questions/README.md`'s exclusion of "work someone is expected to
do": `crates/happenstance/README.md:49`'s overstated MSRV claim, `xtask/src/proof.rs`'s
discovery-vs-execution vacuity hole, and the mis-anchored citation at
`references/evaluation/phase-7-contract-defects.md:10`. Each finding is recorded in the atom that
names it (Ops 7, 4, 5 respectively) without prescribing the fix.

## Follow-ups

- **`kb-decision-0037`** (Op 7) is forced at phase 12 — the MSRV becomes a promise the moment
  `happenstance-core`, `happenstance`, `happenstance-testkit` and `happenstance-sqlite` publish,
  and the `msrv` CI job stays vacuous (proving only that the workspace builds at 1.97.1, not that
  1.97.1 is the minimum) until the toolchain pin and the floor diverge.
- **`kb-open-question-event-metadata-no-floor-001`** (Op 8) is forced by phase 12, when the four
  `MIN_SUPPORTED_*` constants and the frozen `AppendError` variant set become a promise, and
  independently by the first adapter that has to refuse a metadata blob.
- **`kb-open-question-adr-0022-falsifiers-fired-001`** (Op 13) is forced by phase 12, after which
  the SQLite adapter's pragma set is a documented property of a published crate; it names no
  assignee, matching `kb-open-question-es-17-two-adapter-measurement-001`'s precedent.
- **`kb-open-question-testkit-contention-tolerance-001`** (Op 10) is forced by the first adapter
  whose conformance run fails on contention rather than on conformance, and by whoever next
  proposes changing `CONTENDERS`.
- **`kb-open-question-read-page-budget-001`** (Op 9) and
  **`kb-open-question-query-plan-parameter-chunking-001`** (Op 12) both carry a phase-12 deadline,
  after which the read surface and `MIN_SUPPORTED_QUERY_ITEMS` respectively become promises.
- **`kb-open-question-remint-precondition-trust-only-001`** (Op 11) is forced by phase 13's sync
  work, where a duplicated `StoreId` becomes a peer's problem rather than a local one.

## Doctor problems set aside as out of scope

`redkiln doctor --json` reported two problems. Both are `kind: unconsumed-foundation`, both are
under `.bklg/from-contract-to-published-library/`, and neither `file` falls under this wave's `.kb`
path set — this wave's writes are entirely under `.kb/decisions/`, `.kb/open-questions/`,
`.kb/playbooks/`, `.kb/reference/`, `.kb/governance/`, `.kb/maps/` and
`.kb/_governance/integration-waves/`, and it never opened `.bklg/` at all. Set aside rather than
repaired, because repairing a `.bklg/` foundation-story consumption gap here would smuggle an
unrelated backlog change into a KB-intake commit:

- `unconsumed-foundation: .bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0003-provisional-lift/story.md` (foundation story `HS-S0108`, consumed by no capability slice)
- `unconsumed-foundation: .bklg/from-contract-to-published-library/replication-identity-and-ingest/open-questions-resolved-and-indexed/story.md` (foundation story `HS-S0100`, consumed by no capability slice)

These are the identical two problems the prior wave (`2026-09-02-intake`) recorded as out of scope
— nothing in `.bklg/from-contract-to-published-library/` was touched by either wave.

`redkiln validate --kb` exited 0 with no problems of any kind. The combined, scoped verdict for
this wave is **pass**.
