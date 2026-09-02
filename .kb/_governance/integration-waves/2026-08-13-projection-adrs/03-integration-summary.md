# Wave `2026-08-13-projection-adrs` — integration summary

Six operations executed exactly as adjudicated in `02-placement-and-adjudication.md`. Three atoms
created, three amended, zero superseded, zero deferred. This is the first wave to amend rather than
only create: the corpus the 2026-08-10 wave established had, by design, seven `open_question` atoms
waiting for exactly this kind of answer, and three of them got one.

## Atoms created

| # | destPath | id | kind | authority_tier | status |
| --- | --- | --- | --- | --- | --- |
| 1 | `.kb/decisions/0017-what-a-projection-batch-owns.md` | `kb-decision-0017` | decision | decision | accepted |
| 2 | `.kb/decisions/0018-returning-a-projection-to-never-run.md` | `kb-decision-0018` | decision | decision | accepted |
| 3 | `.kb/decisions/0019-what-happens-when-apply-fails.md` | `kb-decision-0019` | decision | decision | accepted |

**`.kb/decisions/` was empty before this wave.** The 2026-08-10 wave abstained from authoring a
single decision atom despite thirteen claims labelled `requires-new-decision`, on the argument that
a pass which both discovers a gap and decides it cannot be audited (`2026-08-10-intake/04`). This
wave is the other half of that abstention working as intended: the decisions arrive with `adr_id`s,
long-form records under `references/adr/`, recorded alternatives and a human sign-off behind them —
supplied by the ADR pass, not by an ingest.

All three carry `status: accepted` with provisional halves **and their falsifiers** folded into the
first clause of `summary`, per the convention the 2026-08-10 import established. `supersedes` and
`superseded_by` are reserved for full supersession and are `null` on all three.
`kb-open-question-adr-status-vocabulary-001` — which asks whether that workaround should become
schema — was deliberately **not** answered by acting on it, for the third wave running.

## Atoms amended

| # | destPath | id | change |
| --- | --- | --- | --- |
| 4 | `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` | `kb-open-question-projection-batch-no-apply-001` | `status: accepted` → **`superseded`**; answered by `kb-decision-0017` |
| 5 | `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md` | `kb-open-question-ps-19-scope-narrower-001` | sub-question 2 answered; status stays `accepted` |
| 6 | `.kb/open-questions/ps-1-states-no-progress-obligation.md` | `kb-open-question-ps-1-no-progress-obligation-001` | sub-question 3 answered; status stays `accepted` |

**No accepted decision body was edited, because none needed to be** — the three amendments are all
`open_question` atoms, whose bodies are also preserved byte-for-byte. In each case the answer landed
as one appended, dated section rather than a rewrite, per `.kb/open-questions/README.md`'s "do not
rewrite a question into its own answer". Each `summary` keeps its existing folded scalar verbatim
and appends one sentence; each `last_reviewed` moves `2026-08-10` → `2026-08-13`; no existing key
was stripped from any atom, including keys redkiln does not own.

**Op 4 resolved without deleting, and without inventing a key.** `status` went to `superseded`
rather than `withdrawn` because `kb-decision-0017` holds the answer and the reader must be sent to
it. `superseded_by` was **not** added: the `KbFrontmatter` authoring set reserves
`supersedes`/`superseded_by` for `decision` atoms, and an invented key on an `open_question`
validates silently under `.passthrough()` and is then read as corpus fact by every later wave. The
edge is carried by `related: [kb-decision-0017]` instead. Sub-question 3 — whether closing this
retroactively validates ADR-0006's encoding-versus-orchestration discriminator — is explicitly
recorded as **not** answered and left with the typed layer at HS-P0011.

## Map atoms updated

Three, all pre-existing:

- **`.kb/maps/decision-map.md`** — three rows for ADR-0017/0018/0019, status and phase 6,
  supersession column `—`. This is the mount point the intake documents named: an atom absent from
  the decision map is the `lib.rs` export block's missing `pub use`.
- **`.kb/maps/domain-map.md`** — the three decisions placed in their domain.
- **`.kb/maps/open-questions-index.md`** — Op 4's row moved to reflect `superseded`; Ops 5 and 6
  re-annotated against their partially-answered sub-questions.

## Links wired

**Outbound at creation time**, so the three parallel create passes could run without dangling
references — every link points at an atom that already existed or at an earlier op in the batch:

- `kb-decision-0017` → `kb-decision-0007`, `-0008`, `-0003`, `-0010`,
  `kb-reference-port-traits-compiled-findings-001`,
  `kb-open-question-projection-batch-no-apply-001`, `-ps-1-no-progress-obligation-001`,
  `kb-playbook-repair-frozen-clause-001`, `-one-decision-per-adr-title-001`,
  `kb-open-question-adr-status-vocabulary-001`
- `kb-decision-0018` → `kb-decision-0007`, `kb-decision-0017`, `kb-decision-0013`,
  `kb-open-question-ps-19-scope-narrower-001`, `-global-vs-boundary-visibility-001`,
  `kb-playbook-repair-frozen-clause-001`, `-one-decision-per-adr-title-001`,
  `kb-open-question-adr-status-vocabulary-001`
- `kb-decision-0019` → `kb-decision-0007`, `kb-decision-0017`,
  `kb-open-question-ps-1-no-progress-obligation-001`, `kb-playbook-repair-frozen-clause-001`,
  `kb-open-question-adr-status-vocabulary-001`

`kb-decision-0018` and `-0019` both `depends_on kb-decision-0017`, and both for a stated structural
reason rather than topical adjacency: 0018's `reset(batch, id)` uses the `ProjectionProbe::probe_delete_all`
0017 mints, and 0019's rollback-survival argument is stated as a constraint on 0017's own decision.

**Reciprocal backlinks**, wired by the Maps phase after the creates landed:

- `kb-reference-port-traits-compiled-findings-001` ← `kb-decision-0017`
- `kb-playbook-repair-frozen-clause-001` ← `kb-decision-0017`, `-0018`, `-0019`
- `kb-playbook-one-decision-per-adr-title-001` ← `kb-decision-0017`, `-0018`
- `kb-open-question-adr-status-vocabulary-001` ← `kb-decision-0017`, `-0018`, `-0019`
- `kb-open-question-global-vs-boundary-visibility-001` ← `kb-decision-0018`
- `kb-open-question-ps-1-no-progress-obligation-001` ← `kb-decision-0017`
  (`kb-decision-0019` was already reciprocal from the new-atom pass)

## Intake cleared

Four documents, all consumed:

- `.kb/_intake/2026-08-13-adr-0017-projection-batch.md`
- `.kb/_intake/2026-08-13-adr-0018-reset.md`
- `.kb/_intake/2026-08-13-adr-0019-apply-failure.md`
- `.kb/_intake/2026-08-13-ps-clause-pairing-sweep.md`

`.kb/_intake/README.md` was dropped from the ingest set at the approval gate and remains — a README
ingested as an atom is a corpus-shaped artefact with no decision in it.

## Verification

`redkiln validate --kb` — **passed, exit 0**. No duplicate ids, every `depends_on` / `related` /
`supersedes` / `superseded_by` resolves to a real atom, `KbFrontmatter` conformance holds on all six
touched atoms, and accepted-decision immutability holds against `HEAD`.

`redkiln doctor` — **exit 1, and not on this wave's account.** Nine errors, all of the form
"foundation story `HS-S####` is consumed by no capability slice in initiative
`from-contract-to-published-library`", entirely inside `.bklg/`. See `04-retrospective.md` for how
that was established and what it cost.
