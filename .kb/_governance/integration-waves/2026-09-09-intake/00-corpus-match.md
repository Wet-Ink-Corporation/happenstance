# Wave `2026-09-09-intake` — corpus match

What already exists in `.kb/`, what each incoming claim could have merged into, and the score
behind every disposition. Read this before `02-placement-and-adjudication.md`: the placements
there are only defensible because of what this file establishes about the corpus and about the
tree.

This is the corpus's **eleventh wave** and its shape is the exact inverse of the tenth. Wave 10
took fifty-seven staged files and its problem was volume. This wave takes **five**, and its
problem is that **four of them are two documents each**. Two ADRs arrived in the same batch as
the findings they settle, and each finding file already carries, in its own voice, a pointer at
the ADR that settled it. Reading the five as five would mint four atoms for two commitments —
two proposing and two deciding — and neither pair is the decision.

Its hard jobs are five:

1. **Collapsing two finding→decision pairs.** `ps-2-live-transaction-axis-is-forbidden-not-unbuilt`
   is the finding ADR-0060 rests on; `es-11s-falsifier-fired-on-the-adapter-it-named` is the
   finding ADR-0061 rests on, and the second says so in a callout added above its own first line.
   Each pair collapses to **one** decision atom. `02`, Adjudications 3 and 4.
2. **Refusing to supersede ADR-0036 when a wave's own source says not to.** ADR-0060 reaffirms
   ADR-0036's *decision* and replaces its *reason*. The temptation is to read a replaced reason as
   a superseded decision and flip `kb-decision-0036`'s frontmatter. It is not, and ADR-0060's own
   `## Why ADR-0036 is not superseded` section is the argument. `02`, Adjudication 5.
3. **Taking a reserved ADR number rather than the next free one.** `RUNBOOK.md`:392 has held
   `0025` open for the Ladybug phase since the plan was written. The staged brief already caught
   this and says so; the wave honours it, and `0026`–`0028` stay reserved. `02`, Adjudication 1.
4. **Finding the cluster three files share and none of them names.** PS-4's Rust-level falsifier
   (from the Ladybug brief) and PS-2's live-transaction end (from two others) are the same
   defect in two clauses: a `[PROVISIONAL]` marker whose stated falsifier the port's own
   signatures forbid from ever firing. One existing open question already owns that shape.
   `02`, Adjudication 7 — and it is the dedup the per-file digests missed.
5. **Minting nothing that asks for an edit that has already landed.** Two of the Ladybug brief's
   claims are corrections it requests; both were verified landed in this worktree before the plan
   was written. `02`, Adjudication 2.

## The corpus, as verified

```
ls .kb/decisions      →  58 decision atoms — 56 accepted, 2 superseded (0002, 0021)
                         numbered 0001–0024 and 0029–0059, plus SD-0001, SD-0002, standalone-svg
                         0025–0028 are NOT missing: RUNBOOK.md:391-395 reserves them by subject
ls .kb/open-questions →  73 atoms, authority_tier: note
ls .kb/reference      →  23 · playbooks 10 · concepts 3 · maps 3 · governance 2 · design 2 · product 0
ls references/adr     →  31 records — 0025, 0060 and 0061 all already written long-form
```

| Layer | Atoms | Bearing on this wave |
| --- | --- | --- |
| `decisions/` | **58** (56 accepted) | **Three are added** — ADR-0025 (a reserved row), ADR-0060, ADR-0061. **No accepted body is edited, no `status` is flipped, and nothing is superseded** — fifth wave running. `kb-decision-0036` is amended *from outside* on the ADR-0029-over-ADR-0004 precedent, and `kb-decision-0017`'s falsifier is *applied* rather than discharged |
| `open-questions/` | 73 | **Two are added** and **four existing atoms receive a merge.** None is resolved and none is superseded: every one of the four survives its own partial answer |
| `reference/` | 23 | **One is added** — the Ladybug driver probes. No existing reference atom is extended; the dating rule, seventh wave running |
| `playbooks/` | 10 | **Two are extended.** Neither is added — both incoming lessons have owners |
| `governance/` | 2 | **Both are extended**, which has not happened in one wave before. They take different halves of the same ADR pair |
| `concepts/`, `design/`, `product/` | 3, 2, 0 | No bearing. Nothing staged is a durable idea needing explanation, a persona, a journey, or an interaction pattern |
| `maps/` | 3 | All three inherit work: `decision-map` three rows, `domain-map` two sections, `open-questions-index` roughly six bullet changes |

For authority purposes:

- **Fifty-six accepted decision atoms exist and the intake names four of them.** ADR-0036,
  ADR-0017, ADR-0030 and ADR-0042. **Not one body is edited, and not one `status` is flipped.**
- **Zero decision supersessions.** Fifth wave running — and this is the first wave in which a
  staged file *proposes* a re-evaluation of an accepted decision by name and the wave still
  declines to supersede, because the file itself asks it to. See Adjudication 5.
- **Two accepted decisions are applied rather than amended.** `kb-decision-0017`'s PS-9/PS-11
  falsifier names *a second generic consumer*; the Ladybug adapter is evidence about that
  clause's cost and is **not** the data point it waits on. The staged brief states this in its
  own voice and the crate root has already been corrected to match.

## Provenance check

Every path the staged files lean on was tested against this worktree.

```
references/adr/0025-the-ladybug-projection-adapter.md            ✓ exists — the long form ADR-0025 cites
references/adr/0060-ps-2s-axis-re-evaluated.md                   ✓ exists
references/adr/0061-es-11s-sufficiency-condition-assumed-a-queue.md
                                                                 ✓ exists
RUNBOOK.md:392                                                   ✓ ADR-0025 reserved for phase 11, by subject
RUNBOOK.md:4917                                                  ✓ "Fills the write-vocabulary axis" — the correction has landed
crates/happenstance-ladybug/src/lib.rs:151                       ✓ "not one of PS-9's or PS-11's data points" — landed, past tense
standards/rust/90-skeletons-and-todo.md:82                       ✓ "the exemplar this rule used was itself refuted" — landed
crates/happenstance-core/tests/probe_live_transaction_shape.rs   ✓ exists — the compiled table the OQ already holds
experiments/live-handle-projection-batch/                        ✓ exists — named as option 2's measurement site
experiments/ladybug-driver-probes/{README.md,probes.rs}          ✓ both exist; results/ does NOT — see below
spec/SPECIFICATION.md:5148-5158                                  ✓ PS-4's MUST and its two-condition falsifier, verbatim
projection_store_conformance! invocations                        ✓ four storage adapters: sqlite, postgres, neon, ladybug
```

**One dangling reference, and it is load-bearing rather than incidental.**
`experiments/ladybug-driver-probes/README.md`:9 states that *"`results/probe-output.txt` is its
output, verbatim."* No `results/` directory exists in this worktree, and
`git check-ignore -v` names the cause: `.gitignore`:69's `*-output.txt`, a rule written at the
pre-publication sweep to stop machine-path-carrying transcripts being committed to a
repository about to go public. Every other experiment escapes it by accident of naming — 321
files under `experiments/**/results/` are tracked, none of them suffixed `-output.txt`.

This matters to this wave specifically, because ADR-0061 refuses to state a failure rate on the
stated ground that *"a measurement lives beside its raw output under `experiments/`"* — and
ADR-0025's own evidence is in exactly the position that rule forbids. The reference atom this
wave mints therefore cites the README's transcription and says the raw file is absent, rather
than citing a path that does not resolve. The general defect goes to an open question (Op 6).

## Scoring — the merge candidates that decided a disposition

Scores are lexical overlap against the live atom, and they decide `merge_existing` vs
`create_new` only where the *subject* also matches. A high score against an atom that answers a
different question is a `related` edge, not a merge.

| Cluster | Best live candidate | Score | Disposition | Why |
| --- | --- | --- | --- | --- |
| ADR-0060, the gate's new reason | `kb-decision-0036` | 92 | `create_new` | 0036 is **accepted and immutable**, and its decision *stands*. A replaced reason is a new atom or it is nothing — see Adjudication 5 |
| PS-2 forbidden, the mechanisms | `kb-open-question-probe-read-through-signature-001` | 85 | `merge_existing` | Direct owner. It already holds the compiled table for this seam and attributes the gap to **scarcity**; the finding replaces that attribution with a compiled cause |
| ADR-0061, ES-11's sufficiency condition | `kb-decision-0011` | 55 | `create_new` | 0011 is the read-laziness decision ES-11 rests on; it does not state the sufficiency condition and cannot absorb its narrowing |
| ES-11 falsifier fired | `kb-open-question-es-11-sqlite-ceiling-sample-cost-001` | 72 | `merge_existing` | Its **sub-question 3** asks verbatim whether an HTTP-transport adapter reporting should move ES-11's marker. This is that report |
| ADR-0025, the Ladybug bundle | `kb-decision-0017` | 78 | `create_new` | The high score is against the decision this one *implements*. 0017 is accepted; ADR-0025 applies its falsifier and discharges none of it |
| PS-4 Rust half + PS-2 end | `kb-open-question-provisional-falsifiers-001` | 68 | `merge_existing` | Same shape, sharper: its title is *"falsifiers can no longer falsify"* and these two **never could**. The cluster no digest named |
| Ladybug `RESET_REFUSAL` declined | `kb-open-question-reset-refusal-declension-001` | 62 | `merge_existing` | Its body says *"the population that could be lying is empty… it should close before the first one does."* The first one has arrived and it declines |
| "ADR-0036 is not superseded" | `kb-governance-referent-not-reasoning-001` | 72 | `merge_existing` | Direct owner of the immutability discrimination, and this is a third case its test does not yet name: the reason expires while the decision stands |
| Narrower claim vs retracted one | `kb-governance-what-may-refute-a-finding-001` | 60 | `merge_existing` | Its `## A fourth consequence` already runs the standard in both directions; this adds the case where the *predecessor* was the thing retracted |
| A narrowing is an amendment | `kb-playbook-repair-frozen-clause-001` | 62 | `merge_existing` | Its test is mechanical and this is the case that sharpens it — nothing gets harder for an adapter and the admitted set still moves |
| Four stale axis-naming passages | `kb-playbook-count-or-index-nobody-re-derives-001` | 55 | `merge_existing` | Same root — a written claim decays and nothing forces a re-read — third shape. See Adjudication 10 for the alternative that lost |
| ES-11's failure rate | `kb-reference-*` | — | **no operation** | There is no committed raw output, and both source files forbid stating a rate. A reference atom would be the false-reference the layer's dating rule names |
| PS-2's compiled refutations | `kb-reference-port-traits-compiled-findings-001` | 45 | **no operation** | The open question already holds the compiled table for this seam. A second home is the two-copies-one-stale failure `reference/README.md` warns about |
| Thirteen `[PROVISIONAL]` clauses, none moving | (none) | 20 | **no operation** | A statement that nothing changed. It lands in ADR-0060's body; the corpus does not mint an atom for an absence of consequence |
| RS-90-1's refuted exemplar | `standards/rust/90-…` | 95 | **no operation** | Outside `.kb`, and **already landed** at line 82. Evidence and provenance only |

## Cross-file clusters, enumerated

Four clusters span more than one intake file. Each collapses to **one** destination atom.

| Cluster | Files | Destination |
| --- | --- | --- |
| PS-2's axis: finding + decision | `ps-2-forbidden`, `adr-0060` | `decisions/0060-ps-2s-axis-re-evaluated.md` |
| ES-11's falsifier: finding + decision | `es-11-fired`, `adr-0061` | `decisions/0061-es-11s-sufficiency-condition-assumed-a-queue.md` |
| A falsifier the port forbids from firing | `adr-0025` (PS-4), `ps-2-forbidden`, `adr-0060` (PS-2) | `open-questions/es-7-and-vt-9-provisional-markers.md` (merge) |
| The probe seam, widened past one method | `ps-2-forbidden`, `adr-0060` | `open-questions/probe-read-through-signature-and-live-transaction-seam.md` (merge) |

Two further pairs are *not* clusters and are worth naming so the next wave does not merge them.
ADR-0025 and ADR-0060 both assert that PS-2's live-transaction end is forbidden rather than
unbuilt — the **mechanism** is ADR-0060's alone and ADR-0025 cites it, because two atoms
carrying one compiled argument is one atom carrying it and one drifting. And ADR-0060 and
ADR-0061 are both re-evaluations of a clause against `happenstance-neon`; they touch different
clauses on different ports and share nothing but the adapter.

## Corrections to a staged framing

**Correction 1 — the digests' ADR numbering was right for the wrong reason, once.** The
per-file digest for the Ladybug brief proposed `0025` and it is correct, but not because it is
free: `0025` is *reserved by subject* at `RUNBOOK.md`:392 for exactly this phase. `0060` and
`0061` are the next two free numbers after `0059`, and `0026`–`0028` stay reserved and
unclaimed. All three long-form records already exist under `references/adr/` at these numbers.

**Correction 2 — one digest proposed superseding a decision, and no decision is superseded.**
The `ps-2-forbidden` digest labelled its remedy-menu claim `supersede`, pointing at a sibling
*intake file* as the `destPath`. An intake file cannot be superseded, and what the menu is
superseded by is a decision that chooses among its options — which is a `create_new` attributed
to the other file in the pair, not a supersession of anything in `.kb/`.

**Correction 3 — two claims ask for edits that have already landed, and the digests said so.**
`crates/happenstance-ladybug/src/lib.rs`:151 and `RUNBOOK.md`:4917 both carry their corrections
in this worktree, in the past tense, naming what they used to say. Every atom this wave writes
states them as history. **Nothing is owed**, and an atom minted asking for either would be a
task rather than knowledge — which `open-questions/README.md` puts in `.bklg/`, not here.

**Correction 4 — the file-lock measurement is one host, and the atom must not generalise it.**
`experiments/ladybug-driver-probes/README.md`:3 states the run: `x86_64-pc-windows-msvc`, one
machine, one date, `lbug` 0.20.3. The refusal it records is `Error: 33`. `Arc<Database>` is
forced by that measurement on that host; whether the lock is the engine's or the platform's is
a distinction the single run cannot make, and the reference atom says so rather than promoting
one observation into a property of the driver. Carried to `unresolved[]`.
