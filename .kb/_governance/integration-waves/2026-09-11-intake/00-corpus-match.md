# Wave `2026-09-11-intake` — corpus match

What already exists in `.kb/`, what each incoming claim could have merged into, and the score
behind every disposition. Read this before `02-placement-and-adjudication.md`: the placements
there are only defensible because of what this file establishes about the corpus — and, this
wave more than any before it, about **what the worktree does and does not carry**.

This is the corpus's **twelfth wave**. Five staged files, and they are three different kinds of
document wearing one directory. Two are ADR briefs (`adr-0062`, `adr-0063`) for decisions taken
on a lane this worktree cannot see. One is a decision brief with its evidence fully on disk
(`host`). One is an open question that has already been half-answered by its owner
(`citation-scan`), and one is a brief whose **first line asks not to be ingested** (`adr-0004`),
because the atom it proposes already exists.

Its hard jobs are five:

1. **Deciding what to do with two ADRs whose referents are not at `HEAD`.** `adr-0062` names
   `references/adr/0062-…` as its long form and `adr-0063` names `references/adr/0063-…`;
   **neither file exists in this worktree**, and the code both describe has not landed here:
   `ProjectionStore::begin` is still synchronous at `crates/happenstance-core/src/projection.rs:460`,
   PS-6's MUST still reads *"neither `async` nor fallible"* at `spec/SPECIFICATION.md:5183`,
   `conformance = ["unstable-projection"]` still stands at `crates/happenstance-core/Cargo.toml:110`,
   and §1.3's census still reads 138/46/12/5. `02`, Adjudication 1 — the wave's most contestable
   call, and the one carried to `unresolved[]`.
2. **Collapsing the deferral two ADRs make in the same words.** `adr-0062`'s *"What it
   deliberately does not decide"* and `adr-0063`'s *"The typed layer keeps a gate of the same name
   for a different reason"* are one question — `Projection::apply` is synchronous, and a live
   store cannot be driven through it — deferred twice with no owner. One open question, not two.
   `02`, Adjudication 4 — the dedup one digest routed at an accepted decision's body.
3. **Refusing to merge into an immutable atom a file that says it is already there.** Every
   claim in `adr-0004` scores 85–100 against `kb-decision-0037`, which is accepted. The file's own
   header says it is discharged by 0037 and asks to be held out. One sentence in it is not in the
   corpus, and it goes to the open question that owns registry state. `02`, Adjudication 8.
4. **Finding the owner of a question a file says is "worth an atom of its own".** The
   `citation-scan` file's second finding — `redkiln validate --kb`'s immutability check is a
   dirty-tree guard — is recorded in `CLAUDE.md` and in no `.kb` atom. Its first finding, by
   contrast, has an exact owner: `kb-open-question-docs-citation-anchor-contradiction-001`'s
   sub-question 2 asks the question the owner has now answered. `02`, Adjudications 6 and 7.
5. **Splitting the host brief the way it asks to be split, and once more.** The brief proposes
   two atoms — a decision and a reference. The wave finds a third: the SMT-sibling control that
   caught two wrong instruments is a method with a claim, a measurement and two rejected
   alternatives, which is `playbooks/README.md`'s definition verbatim. `02`, Adjudication 3.

## The corpus, as verified

```
ls .kb/decisions      →  61 decision atoms — 59 accepted, 2 superseded (0002, 0021)
                         numbered 0001–0025 and 0029–0061, plus SD-0001, SD-0002, standalone-svg
                         0026–0028 stay reserved by subject (RUNBOOK.md:391-395); 0062+ free on disk
ls .kb/open-questions →  75 atoms, authority_tier: note — 62 accepted, 13 superseded
ls .kb/reference      →  24 · playbooks 10 · concepts 3 · maps 3 · governance 2 · design 2 · product 0
ls references/adr     →  31 records — the highest is 0061; NO 0062, NO 0063
git log --oneline -1  →  86a410c, the same commit `main` and `lane/projection-probe-seam` sit at
```

| Layer | Atoms | Bearing on this wave |
| --- | --- | --- |
| `decisions/` | **61** (59 accepted) | **Three are added** — ADR-0062, ADR-0063, ADR-0064. **No accepted body is edited, no `status` is flipped on any decision, and nothing is superseded** — sixth wave running. ADR-0063 *discharges* `kb-decision-0036` and `kb-decision-0060` on the ADR-0037-over-ADR-0004 shape: a conditional decision whose condition has been met is not a wrong decision |
| `open-questions/` | 75 | **Two are added**, **one is resolved** (`probe-read-through-signature`, flipped to `superseded` in the corpus's own convention for an answered question), and **five existing atoms receive a merge** |
| `reference/` | 24 | **One is added** — the host clocksource. No existing reference atom is extended; the dating rule, eighth wave running |
| `playbooks/` | 10 | **One is added** and **one is extended** |
| `governance/` | 2 | **One is extended** — a fifth worked instance for `rewrite-the-referent`, and the first in which the rule collided with the check that enforces it |
| `concepts/`, `design/`, `product/` | 3, 2, 0 | No bearing |
| `maps/` | 3 | All three inherit work: `decision-map` three rows and two annotations, `domain-map` a section that does not yet exist (measurement), `open-questions-index` two bullets added, one flipped, five annotated |

For authority purposes:

- **Fifty-nine accepted decision atoms exist and the intake names eight of them.** ADR-0004,
  ADR-0029, ADR-0037 (by `adr-0004`); ADR-0017, ADR-0036, ADR-0060 (by both ADR briefs);
  ADR-0022 (by `host`); ADR-0058 (by `citation-scan`). **Not one body is edited, and not one
  `status` is flipped.**
- **Zero decision supersessions.** Sixth wave running. Two briefs ask in their own voices not to
  supersede: `adr-0063` — *"Neither is superseded in its reasoning, which was right when
  written"* — and `adr-0004` — *"an amendment lineage, not a supersession"*.
- **One accepted decision was repaired in place before this wave, and the wave records the
  repair rather than repeating it.** `kb-decision-0058`'s three ranges were repointed at
  `4e13ee2` under the referent rule; `citation-scan` is the file that explains why, and the
  governance atom is where the explanation goes.

## Provenance check

Every path the staged files lean on was tested against this worktree at `86a410c`.

```
── host ──
ops/host/{00-system.sh,cpu-tuning.sh,preflight.sh,host.env,README.md}     ✓ all exist
ops/host/probes/{tsc-coherence.c,tsc-pairwise.c,tsc-migrate.c}            ✓ all three instruments, in order
ops/host/README.md:149,160,180                                            ✓ the TSC warp line, tsc=reliable, the 62,493 / 0 table
benchmarks/src/paired.rs:71-73                                            ✓ "CF-34 forbids a benchmark result gating a merge" — exact
benchmarks/src/cpu.rs:58                                                  ✓ RESOLVABLE_FLOOR = 156 ms, doc'd as 10× the ~15.6 ms tick
benchmarks/tests/controls_fire.rs                                         ✓ exists
benchmarks/README.md:337                                                  ✓ "## What none of this shows"
xtask/src/affected.rs:448, :999                                           ✓ "ops/" on INERT; the_host_provisioning_tree_selects_no_package
references/adr/0022-append-condition-strategy.md:120-144                  ✓ "disagreed by up to 45%… varied by 4x" — the range holds
spec/SPECIFICATION.md:8747  (cited as CF-34)                              ✗ DRIFTED — CF-34 is at :9021; :8747 is inside CF-38's Rule
xtask/src/lints.rs:2007     (cited as CITATION_SCAN_EXCLUDE)              ✗ DRIFTED — the constant is at :2488

── citation-scan ──
xtask/src/lints.rs:2003     (cited as CITATION_SCAN_EXCLUDE)              ✗ DRIFTED — :2488, same constant, same text
.kb/decisions/0058-…:33                                                   ✓ the line; its ranges now read :1396-1431, repointed at 4e13ee2
a4616ca, 025f300, 6acdf24, a0a925b, 4e13ee2                               ✓ all five commits resolve; messages match the file's account
REMEDIATION-HANDOVER.md:143                                               ✓ "~1,200" under .kb/_intake, beside ~2,700 and ~25,700
.github/workflows/ci.yml:243                                              ✓ `if: false` on the backlog job
CLAUDE.md:216                                                             ✓ the "edited in place" transcript the finding rests on

── adr-0004 ──
.kb/decisions/0037-msrv-becomes-a-promise-at-0-2-0.md                     ✓ accepted, phase 12, adr_id ADR-0037 — as the header says
.kb/decisions/0004-*, 0029-*                                              ✓ both accepted, superseded_by: null, byte-identical to HEAD

── adr-0062 / adr-0063 ──
references/adr/0062-the-probe-seam-moves-and-the-far-end-is-built.md      ✗ ABSENT
references/adr/0063-the-projection-port-is-frozen.md                      ✗ ABSENT
crates/happenstance-core/src/projection.rs:460  fn begin(&self) -> Self::Batch
                                                                          ✗ NOT MOVED — synchronous, infallible, as ADR-0060 found it
spec/SPECIFICATION.md:5183  "PS-6 — begin MUST be neither async nor fallible"
                                                                          ✗ NOT REWRITTEN
crates/happenstance-core/Cargo.toml:110  conformance = ["unstable-projection"]
                                                                          ✗ NOT RETIRED — "conformance implies nothing again" is not yet true here
crates/happenstance/Cargo.toml:142  unstable-projection = ["happenstance-core/unstable-projection", …]
                                                                          ✗ STILL FORWARDS
crates/happenstance-postgres/src/  LivePostgresProjectionStore             ✗ ABSENT — no such type in the crate
spec/SPECIFICATION.md:223, :9288  census 138 / 46 / 12 / 5                 ✗ NOT 141 / 41 / 12 / 7
git branch -a                                                             lane/projection-probe-seam exists and is AT 86a410c —
                                                                          zero commits ahead of main; whatever carries the work is uncommitted
```

**The two ADR briefs fail provenance wholesale, and it is a fact about where the work is rather
than about whether it happened.** The main checkout is on `lane/projection-probe-seam`, at the
same commit as this worktree, with no commit ahead of `main`. Under this wave's hard constraint
the other checkout is not read for authority, so from here the decisions are *described* and
not *evidenced*: every number in them — 20 of 20, eight probe impls, 117 call sites, one runner
error arm — is the brief's, and the atoms must attribute it that way. What the wave can verify is
exactly what ADR-0060 verified last wave, and that is the state the two new atoms record as
*before*. Adjudication 1 says what follows.

**Three citations drifted while staged, in the two files that talk about citation drift.** The
`citation-scan` file cites `lints.rs:2003`, the `host` file cites `lints.rs:2007`, and the
constant is at `:2488`; the `host` file (and `ops/host/README.md`, which it inherits from) cite
CF-34 at `:8747`, which is now a line inside CF-38's `Rule`. None of the three would redden the
gate — `check_citation` passes an in-range line in a non-Rust target and only checks a Rust
target's first line for blankness or mid-construct shape — which is the milder half of what
`kb-open-question-docs-citation-anchor-contradiction-001` already records. Every atom this wave
writes carries the re-anchored line, exactly as the `host` file's *"Note to the ingest"* asks.

## Scoring — the merge candidates that decided a disposition

Scores are lexical overlap against the live atom, and they decide `merge_existing` vs
`create_new` only where the *subject* also matches. A high score against an atom that answers a
different question is a `related` edge, not a merge.

| Cluster | Best live candidate | Score | Disposition | Why |
| --- | --- | --- | --- | --- |
| ADR-0062, the seam moves | `kb-open-question-probe-read-through-signature-001` | 88 | `create_new` decision + **resolve** the OQ | The OQ is the question; ADR-0062 is the answer. A decision does not merge into the question it closes — see Adjudication 2 |
| ADR-0062 vs ADR-0060 | `kb-decision-0060` | 75 | `create_new` | 0060 is accepted, named this change and *declined* it. The atom that makes it is new, and `depends_on` 0060 |
| ADR-0063, the port is frozen | `kb-decision-0060`, `kb-decision-0036` | 78, 72 | `create_new` | Both accepted and both said *gate the port*. Neither is wrong: PS-3's SHOULD was conditional and its condition is met. Discharged, not superseded — Adjudication 5 |
| `apply` is synchronous — deferred by 0062 and 0063 | `kb-open-question-projection-batch-no-apply-001` (superseded) | 50 | `create_new` open question | The superseded atom's sub-question 3 is about ADR-0006's discriminator, not about `apply`'s shape. The 0062 digest aimed this at `kb-decision-0017`'s body, which is accepted — Adjudication 4 |
| PS-2's forbidden end, unforbidden | `kb-open-question-provisional-falsifiers-001` | 70 | `merge_existing` | Last wave added *"PS-2's live end is forbidden by `begin`'s signature … thirteen clauses wait on PS-2 alone"*. Both halves moved: the signature was changed rather than the marker, and the thirteen was a ledger simplification |
| PS-6's MUST named a mechanism | `kb-playbook-require-the-property-001` | 62 | `merge_existing` | Same shape from the specification side: a signature stood proxy for *no round trip at `begin`*; when the signature had to move, two tests took the property over |
| Testkit exemption scope | `kb-open-question-projection-module-exemption-scope-001` | 66 | `merge_existing` (annotate) | The exemption the question was scoped to no longer exists on the port. Not resolved: the testkit manifest after the lane cannot be read here |
| The host decision | `kb-decision-0022` | 45 | `create_new` decision | 0022 carries one of the three instability instances as *evidence for a strategy*; it decides nothing about the machine |
| Clocksource finding | `kb-reference-busy-timeout-margin-001` | 20 | `create_new` reference | Structural sibling only: a dated, one-host measurement with a Conditions section |
| The SMT-sibling control | `kb-playbook-verify-referent-report-coverage-001` | 25 | `create_new` playbook | No owner. `kb-decision-0010` states the principle for the conformance suite; this is the method one layer down, with two instruments it rejected |
| preflight and CF-34 | `kb-open-question-cf-33-cf-34-scope-001` | 55 | `merge_existing` | Direct owner of *clock-adjacent assertions outside the testkit*. This is a third instance, and the first that is deliberately unreachable from the gate |
| Re-anchor at promotion (decided) | `kb-open-question-docs-citation-anchor-contradiction-001` | 82 | `merge_existing` | Sub-question 2 asks *"scanning intake after all, a pre-ratification repointing step, or something narrower?"* The owner chose the third and declined the first, by name |
| The hole fired (0058:33) | `kb-reference-intake-citation-drift-census-001` | 78 | **no operation** on the reference | The census is dated and stays dated. The firing goes to the OQ above, which the census already feeds |
| Dirty-tree immutability guard | `kb-governance-referent-not-reasoning-001` | 60 | `create_new` open question + `merge_existing` governance | The governance atom gets the fifth instance and the collision; the *question* (stronger check or carve-out) has no owner and is a `defer_open_question` |
| Supersede-to-fix-a-citation rejected | `kb-governance-referent-not-reasoning-001` | 58 | `merge_existing` (same op) | A rejected alternative for the same repair — one sentence, in the same section |
| `adr-0004`, all five claims | `kb-decision-0037` | 85–100 | **no operation** on 0037 | Accepted and immutable; and the file says it is already there. The digests' `merge_existing` into 0037 is not an available operation |
| Registry re-read 2026-09-08 | `kb-open-question-stale-0-0-0-name-reservations-001` | 64 | `merge_existing` | Its *"What is true today"* has five crates at `0.0.0`; the re-read has seven. The one fact in the file the corpus does not hold |
| `adr-0004` §"amendment, not supersession" | `kb-governance-referent-not-reasoning-001` | 75 | **no operation** | Already the governance atom's ADR-0029/ADR-0004 shape, and 0037 is already in its `related` |

## Cross-file clusters, enumerated

Three clusters span more than one intake file. Each collapses to **one** destination atom.

| Cluster | Files | Destination |
| --- | --- | --- |
| `Projection::apply` is synchronous, and a live store cannot be driven through it | `adr-0062`, `adr-0063` | `open-questions/projection-apply-is-synchronous-against-a-live-store.md` (new) |
| PS-2's bar, met by moving the signature; the "thirteen on PS-2 alone" ledger row narrowed | `adr-0062`, `adr-0063` | `open-questions/es-7-and-vt-9-provisional-markers.md` (merge) |
| `.kb/_intake` is outside the citation scan; re-anchor at promotion | `citation-scan`, `host` (its "Note to the ingest") | `open-questions/docs-citation-anchor-form-and-clause-contradiction-check.md` (merge) |

Two further pairs are *not* clusters and are worth naming so the next wave does not merge them.
`adr-0062` and `adr-0063` are consecutive decisions about the same port and the second depends on
the first, but they answer different questions — *does the seam move* and *does the gate come
off* — and 0063's own alternatives list includes *"freezing PS-6 the day its MUST was
rewritten"*, which is a refusal to collapse them. And `adr-0004` and `citation-scan` both invoke
accepted-atom immutability, from opposite directions: one to say a marker cannot be lifted by
edit, the other to say a line number can be repaired by one. The governance atom already holds
that discrimination; neither file changes it.

## Corrections to a staged framing

**Correction 1 — the digests' ADR numbering is right and was derived rather than trusted.**
`adr-0062` and `adr-0063` carry their numbers in their own titles; under wave 5's rule *an ADR
number is allocated by the artefact that already carries it*, and the briefs are the only
artefacts on this disk that do. `0064` for the host decision is highest-taken + 1. `0026`–`0028`
stay reserved and unclaimed. The `host` digest's own guess — *"0064 … number provisional"* — was
correct for the right reason.

**Correction 2 — one digest aimed a merge at an accepted decision's body.** The `adr-0062`
digest routed the `apply` deferral to *"`.kb/decisions/0017-…` (annotate sub-question 3, no new
atom warranted)"*. `kb-decision-0017` is accepted and immutable, and its *"sub-question 3"* is a
reference to `kb-open-question-projection-batch-no-apply-001`'s third sub-question, which asks
about ADR-0006's encoding-versus-orchestration discriminator and not about `apply`'s synchrony.
The `adr-0063` digest routed the same claim to a new open question, and that is the disposition.

**Correction 3 — five digests proposed `merge_existing` into `kb-decision-0037`, and none of
those is an available operation.** The `adr-0004` digest's own verification found what the
file's header says: 0037 exists, is accepted, and already carries every sentence the file wants
carried. A merge into an accepted decision's body is the one edit the corpus forbids. The file
mints no decision-layer operation, and the wave says so in `01` §D.

**Correction 4 — the `citation-scan` file arrived with no digest.** The extraction phase
produced digests for four of the five files. Its claims were extracted in this phase, from the
file, against the corpus — `01` §B — and are the least pre-digested content in the wave.

**Correction 5 — the digests treated ADR-0062's and ADR-0063's evidence as verified, and it is
not verifiable from here.** *"20/20 against live PostgreSQL"*, *"117 call sites"*, *"a third
test holds that no in-tree crate still forwards the retired feature"* — all true or false about a
tree this wave cannot see. The atoms carry them as the brief's statements, in the brief's voice,
with the tree's state at `86a410c` stated beside them. Adjudication 1 has the full argument and
`unresolved[]` has the residue.
