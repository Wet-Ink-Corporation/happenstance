---
item: HS-P0019
stage: briefs
---

# Briefs — The whole gate on the assembled library, and a durable audience

Companion to [`project.md`](project.md) and [`_grounding.md`](_grounding.md). This
project carries **one** warranted brief — `testing` — per `../_decomposition.md`
*Warranted briefs*: "it verifies and records; it designs nothing." No
`architecture`, `ux` or `deployment` section belongs here; forcing one would
produce exactly the stub the right-sizing rule exists to prevent.

## Testing brief

### Intent

The single child this brief scopes: **HS-P0019**, whose only deliverable is
verification and recording, never new production code. There is nothing here
for a story to implement in the ordinary sense — every AC below is proven by
*running an existing gate* on the assembled tree and *recording* what it
showed, not by writing a new test against new code. This brief therefore
defines the **test mix** (which tier proves which AC-###), the **merge-gate
commands** this project must exit zero on, the **fixtures/seams** it reuses
(it introduces none of its own), and — because AC-010/DR-10 assign it here —
**decides the evaluator-persona question** before any `.kb/product/` atom is
authored.

Traces to [`project.md`](project.md) AC-001…AC-014, DR-1…DR-13, DoD 1–7. It
does not restate the charter; see `project.md` for the full acceptance
criteria text and `../initiative.md` for BR-15, BR-16, AC-15, DoD 13, DoD 16.

### Acceptance Criteria

One row per project AC-###, each mapped to the test tier(s) that prove it and
the artefact the tier produces. "Static" below means a check that reads
source/config/frontmatter without executing the code under test; "process"
means a check over the commit graph or backlog state rather than over test
output — both are load-bearing tiers for a project whose job is auditing, not
building.

| AC | Tier | How it is proven |
| --- | --- | --- |
| **AC-001** — whole gate green, clean checkout | **E2E** | `cargo xtask ci` run to completion from a checkout with no untracked/ignored residue and no path-dependency override (DR-1). Record the exit code, the commit SHA it ran against, and — per the `xtask/src/main.rs` module doc's own accounting of which steps are tool-gated (`cargo hack`, `cargo deny`, the wasm32 powerset, the nightly `--cfg docsrs` build) — the reason for every step that printed `skipped`. |
| **AC-002** — four `wasm32` steps green in that run | **Integration** (inside the E2E run) | The same `cargo xtask ci` invocation's four wasm32 steps: the `wasm32-unknown-unknown` build of `happenstance-core` (the standing guard on ADR-0001 / binding constraint 1), the conformance-harness check, and the `happenstance-cloudflare` / `happenstance-neon` builds. These evidence BR-12 (DR-5) on the assembled tree rather than at each adapter's own merge. |
| **AC-003** — DoD 1–12, 14–15 re-observed as a set | **E2E / process** | One record (this project's closeout artefact, not a new automated test) listing all fourteen scenarios, each re-run or re-inspected on *this* tree with the command and artefact path cited. A sibling's `_ledger.md` (mandatory per `.redkiln/config.yaml:67`) may be the pointer to what to re-run; it is never accepted as the evidence itself (DR-3). |
| **AC-004** — DoD 13 delta stated | **Static / process** | `git log <published-tag>..HEAD` (or equivalent) over the closeout tree, filtered to the commits belonging to `replication-identity-and-ingest` (HS-P0017) and `retention-and-incomplete-logs` (HS-P0018); the record names the tag, the commits, the owning projects, and cites the gate decision (`../_decomposition.md` *Decisions taken at the gate*, item 4) that constrains why the published surface is unchanged across that delta. Not a test-framework check — a checked, cited artefact. |
| **AC-005** — audit table: atom per settled answer, alternatives named | **Static** | `redkiln validate --kb` (frontmatter/status conformance on every cited atom under `.kb/decisions/`) plus a manual cross-reference against the ADR queue at `RUNBOOK.md:262-284`. See *Notes → DR-6 audit procedure* below — the row shape is defined now; the rows cannot be populated until ADR-0017…0028 exist (grounding, "Accepted decision atoms" section). |
| **AC-006** — no open-question atom deleted | **Static / process** | `git diff --diff-filter=D <initiative-start-commit>..HEAD -- .kb/open-questions/` must be empty. Cross-checked against `.kb/maps/open-questions-index.md`'s annotation for each of the six atoms DR-7 names by filename (`cf-40-fixture-limits-ownership.md`, `projection-store-batch-has-no-apply-seam.md`, `es-38-and-gap-read-rules-are-unowned.md`, `global-versus-per-boundary-visibility-invariant.md`, `ps-1-states-no-progress-obligation.md`, `human-readable-payload-encoding-on-a-constrained-peer.md`), per the index's own "never removed, only annotated" convention. |
| **AC-007** — audience atoms exist under `.kb/product/` | **Static** | `redkiln validate --kb` against the new atoms; each is `concept` (persona) or `playbook` (journey) with `authority_tier: product`, per `.kb/product/README.md:15-24`. |
| **AC-008** — atoms arrived via the ingest path | **E2E (process)** | The commit range shows atoms entering through `.kb/_intake/` → `/redkiln:kb-ingest`, with intake staging consumed and cleared — not a hand-authored commit. Checked via `git log --diff-filter=A` on the `.kb/product/` paths plus `.redkiln/config.yaml:73`'s `require_commit_provenance`. This is the constraint the reverted commit `0269720` exists to remember (grounding, "Accepted decision atoms" section); DR-8. |
| **AC-009** — secondary-evidence qualification unmissable | **Static** | Content/schema check on each atom's `summary` and `source_paths` fields (`KbFrontmatter` shape, `src/schema/kb.ts`) confirming the qualifying sentence appears in `summary` itself, not only in the body, plus `redkiln validate --kb` passing. |
| **AC-010** — evaluator question decided before authoring | **This document** | Proven by existing: see *Notes → the evaluator-persona decision (DR-10)* below, authored before any `.kb/product/` atom for this initiative. |
| **AC-011** — backlog/KB clean, exactly six drift advisories | **Static** | `redkiln validate`, `redkiln validate --kb` exit zero; `redkiln doctor --json` parsed the same way `.github/workflows/ci.yml:142-187`'s `backlog` job does — zero `problems`, zero `process-drift`, and `template-drift` file list equal (not superset) to the six named there. Re-run locally on the closeout tree, not inferred from CI's last run. |
| **AC-012** — a seventh/missing-sixth advisory is dispositioned | **Static + process** | Same `doctor --json` run; if the six-file set differs, the record states the decision (revert, or a deliberate adopt with stated reason) and confirms `redkiln adopt --templates` was **not** run either way (CLAUDE.md; grounding). |
| **AC-013** — findings routed, not absorbed | **Process** | The closeout record's findings table shows a destination (`support`, a new item against the owning sibling, or a new decision atom + re-plan) for every defect the re-observation surfaces; zero commits inside this project's own stories touch code outside `.bklg/`/`.kb/` planning artefacts. |
| **AC-014** — initiative can close | **Process** | `redkiln status` shows no open child of `HS-I0006`; the initiative's own closeout links the promoted product atoms. |

### Notes

**Test mix, summarised by tier.**

- **Static** — the largest tier for this project, and the correct one: `cargo
  fmt --check`, `cargo clippy --workspace --all-targets --all-features -D
  warnings`, `cargo xtask spec-trace` (checks `spec/SPECIFICATION.md`'s own
  cross-references, so the 200/198/139/49/10/2 figures at
  `spec/SPECIFICATION.md:219-222` are verified rather than quoted), the
  `--no-default-features` doc build of `happenstance-core` (the standing
  check on binding constraint 2 / ADR-0003), the `cargo package --list`
  licence/README assertion, `cargo hack` (feature powerset),
  `cargo deny` (advisories/licences), the nightly `--cfg docsrs` build,
  `redkiln validate`, `redkiln validate --kb`, and `redkiln doctor --json`.
  All are read/lint/build-graph checks; none executes application logic.
- **Unit** — `cargo test --workspace --all-features` as invoked inside
  `cargo xtask ci`'s `tests` step. This project writes no new unit tests; it
  re-runs every crate's existing suite (including the `memory.rs` pair
  CLAUDE.md names — `send_flavour_stream_is_send_in_generic_code` and
  `spawns_from_generic`, the two tests binding constraint 3 says both matter)
  on the assembled tree rather than trusting each sibling's own run of them.
- **Integration** — the conformance-suite invocations already wired per
  adapter (`happenstance_testkit::event_store_conformance!`, CLAUDE.md *The
  rule that matters*) as exercised inside the `tests` step, plus the four
  `wasm32` steps (AC-002), which are build-and-check rather than
  build-only — that distinction is what makes them integration-tier rather
  than static.
- **E2E** — `cargo xtask ci` itself, run whole, from a clean checkout
  (AC-001); the DoD 1–12/14–15 re-observation record (AC-003), which is an
  E2E check over *process artefacts* rather than over a running binary; and
  the intake → `/redkiln:kb-ingest` → `.kb/product/` path exercised for real
  (AC-008), because the thing under test there is the promotion mechanism
  itself, not a fixed input/output pair.

**Merge-gate commands.** Exactly the terminal grain `.redkiln/config.yaml:56-60`
reserves for this project alone:

```console
cargo xtask ci                          # verify.e2e — the whole gate, not --fast
redkiln validate
redkiln validate --kb
redkiln doctor --json
```

`cargo xtask ci --fast` (`.redkiln/config.yaml:55`) is explicitly **not** this
project's bar — every sibling already met it; re-running only the fast subset
here would prove nothing this project exists to prove (DR-2, project.md risk
table row 1). All four commands run against the clean-checkout tree described
in DR-1, not against the working tree these planning artefacts are written in.

**Fixtures / seams.** This project introduces **no new fixtures** — it owns no
code (`project.md` *Out of scope*). It reuses, unmodified:

- `happenstance_testkit::fixtures::MemoryFixture` and every adapter fixture
  already wired into each sibling's own `event_store_conformance!` call
  (CLAUDE.md *The rule that matters*) — exercised here by re-running the
  `tests` step, not by writing a new invocation.
- The clean-checkout itself is the one seam this project must construct
  deliberately: a fresh `git worktree add` or clone distinct from any warm
  working tree, satisfying DR-1's "no untracked or ignored residue, no path
  dependency standing in for a published version." This is a process seam,
  not a code mock, and it is the thing AC-001 is actually checking for.
- Nothing is mocked. A project whose entire deliverable is re-observation
  would falsify its own purpose by substituting a double for anything it
  claims to have re-run.

**DR-6 audit procedure (rows not yet populatable).** Per grounding
("Accepted decision atoms that constrain this project" and "Tensions /
open items"), ADR numbers `0017`–`0028` are reserved but unwritten at
planning time — they belong to `projection-store-freeze`,
`typed-layer-and-alpha-release`, the three adapter projects and
`replication-identity-and-ingest`/`retention-and-incomplete-logs`, all of
which merge before this project (rank 6, `../_decomposition.md` *Merge
order*). The audit table AC-005 requires has this row shape, to be filled at
closeout, not now:

| Settled question | Atom | Status | Alternatives it records as rejected |
| --- | --- | --- | --- |
| *(one row per answer this initiative settled)* | `.kb/decisions/00NN-slug.md` | `accepted` | *(named in the atom body)* |

For any reserved number (`0017`–`0028`) that closeout finds unwritten, the row
states the reason instead of an atom path — `RUNBOOK.md:276-283` is the
precedent that a number staying empty can itself be correct, not a gap. The
audit also computes, rather than assumes, that the *union* of ranges the
written ADRs cover equals the range of questions this initiative actually
raised — `RUNBOOK.md:262-320`'s own observation that a phase's clause range
versus the union of its ADRs' ranges is "two numbers, and nothing checks that
they are equal" applies identically here.

**The evaluator-persona decision (DR-10, resolving AC-010).**

> **AMENDED 2026-08-12, by the repository owner, at the `/redkiln:plan` spec stage.
> This decision was overruled in part.** The promoted set is **three persona atoms**
> (application author, adapter author, local-first/edge developer) **plus the
> evaluation path as its own journey atom, linked to Persona 1** — not four persona
> atoms, and not a journey *stage* buried inside Persona 1's journey either.
>
> What survived from DR-10: **argument 1**, which `publication-and-positioning`'s
> `_design.md` never rebutted — the difference is in the *mechanism* of
> trust-building (one-shot public evidence versus revisable contact with the code
> over weeks), not a difference of degree within one journey. That is what earns the
> evaluation path a first-class atom of its own.
>
> What did not: the claim that this mechanism difference makes the evaluator a
> different **person**. Every distinguishing property DR-10 names — time-boxed,
> one-shot, cannot run the suite — is a property of a moment; the same human is an
> application author twenty minutes later. A `.kb/product/` **journey** atom holds a
> moment with its own mechanism, which is precisely what is wanted here.
>
> **Argument 3 was discounted outright.** It reasons that folding would
> retroactively make DT-1's option (c) incoherent. Option (c) *lost*, and
> `publication-and-positioning`'s stated reason for rejecting it was partly that it
> splits one person in half — so the argument is circular against the decision it is
> arguing with. Preserving a rejected option's premise is not a reason to shape the
> durable knowledge base.
>
> DR-10's own routing instruction was honoured: it said the inconsistency with
> HS-P0016 was "a finding to route, not something this brief re-litigates." It was
> routed, and this is the disposition. The reasoning below is preserved unedited as
> the argument that lost, because the reasons it lost only make sense beside it.
> Recorded on the other side too (`../publication-and-positioning/_design.md`, DT-1
> rider).

*Decision (superseded — see the amendment above):* **Promote Persona 4 ("the
evaluator, pre-adoption") as its own persona atom in `.kb/product/`, not folded
into Persona 1 (the application author) as an earlier journey stage.**

*Reasoning, cited:*

1. `../_discovery/distillation/personas-and-journeys.md`'s own *Cross-persona
   tensions* section names a structural difference the other three personas
   do not share: "Persona 4 is time-boxed and one-shot in a way the others
   are not. An application or adapter author can revise their understanding
   through contact with the code over weeks. An evaluator's decision window
   is whatever they spend reading a registry page and a README once." That
   is a difference in the *mechanism* of trust-building (one-shot public
   evidence vs. revisable in-code contact), not a difference in degree of the
   same journey — the test a fold-in would have to pass and does not.
2. The initiative's own proof artefact is written for exactly this reader.
   Grounding cites `_intake-brief.md:139-147`'s framing directly: "a suite
   green against `MemoryEventStore` alone proves nothing about the
   contract... only a *passing implementation* is a far end" — addressed to
   someone who "cannot run the suite themselves and has to trust what the
   project shows." That is Persona 4's journey specifically; Persona 1's
   journey (`personas-and-journeys.md` Persona 1, step 4) is about a facade
   *they have already built on*, a later and different moment.
3. `../initiative.md:420` records DT-1's option set as including "(c) two
   entry points, one per audience" for "which claim leads on first
   contact — contract-first proof, or the constrained-runtime audience."
   Option (c) already presumes an audience split at the positioning layer;
   folding Persona 4 into Persona 1 now would retroactively make that
   already-listed option incoherent by deleting the second audience it
   names. Keeping Persona 4 separate keeps DT-1's own option space accurate,
   regardless of which option `publication-and-positioning` (HS-P0016)
   actually chose. Per DR-12 / `project.md` risk table ("The evaluator
   decision has downstream reach"), whether HS-P0016's recorded DT-1
   resolution is consistent with this decision is a **finding to route**,
   not something this brief re-litigates — the disposition happens in the
   AC-013 findings table at closeout, not here.
4. The qualification carries forward, not a special case: `personas-and-journeys.md`
   *Risks* already flags Persona 4 as resting on inferred evidence "rather
   than from a named individual's stated experience," which is *thinner*
   than Persona 1's download-count evidence but not categorically different
   from Persona 3's single blog post + issue thread. DR-9/AC-009's
   secondary-evidence qualification applies uniformly to all four promoted
   atoms; Persona 4 does not need a stronger disclaimer than its siblings,
   only the same one, honestly worded.

*Consequence for promotion (governs AC-007) — as amended:* the promoted set is
**three** persona atoms (Application author, Adapter author, Local-first/edge
developer) plus the corresponding journey atoms, **and one further journey atom
for the evaluation path**, linked to the application-author persona and carrying
its own secondary-evidence qualification. Four atoms carry the evaluator's
concerns in total; exactly three of them are personas.

DR-9/AC-009's secondary-evidence qualification still applies uniformly, and
argument 4 above stands unchanged: the evaluation journey does not need a
stronger disclaimer than its siblings, only the same one, honestly worded.

*(The superseded consequence read: "the promoted set is **four** persona/concept
atoms … not three. Any story or ingest-staging step that assumes three personas
is wrong against this decision." A step that assumes three **personas** is now
correct; a step that assumes only three **atoms**, or that omits the evaluation
journey, is wrong.)*
