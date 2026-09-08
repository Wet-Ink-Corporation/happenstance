# Wave `2026-09-07-intake` — corpus match

What already exists in `.kb/`, what each incoming claim could have merged into, and the score
behind every disposition. Read this before `02-placement-and-adjudication.md`: the placements
there are only defensible because of what this file establishes about the corpus, about the
tree, and about the fifty-seven staged files.

This is the corpus's **tenth wave**, and it is the largest by an order of magnitude — 57 staged
files against wave 9's three, and 50 of the 57 are one directory,
`.kb/_intake/remediation-2026-09-04-briefs/`, staged as a batch three days before the release
pass that ratified seventeen of them. That shape is what makes this wave different from every
previous one. **The dedup pressure is not between two files that happen to say the same thing.
It is between three documents that describe the same decision at three different moments in its
life**: the brief that argued it, the ratification that signed it, and the discharge that
recorded what execution actually produced. Collapsing those three into one atom per decision is
this wave's central job, and getting it wrong would mint three atoms for one commitment — one
proposing, one accepting, one correcting — none of which is the decision.

Its hard jobs are six:

1. **Collapsing the three-document ratification lineage.** Seventeen briefs carry seventeen
   `## Recommendation` sections; `ratifications-2026-09-06-pre-publication.md` carries one table
   ratifying them; `2026-09-07-ratifications-discharged-and-what-execution-changed.md` carries
   one table saying which six landed and which three landed *differently*. Every ratified brief
   therefore has two or three intake files behind it, and gets **one** decision atom.
   `02`, Adjudications 5–23.
2. **Refusing to sign a ratification the tree contradicts.** `msrv-premise` was ratified on its
   own recommendation, and its recommendation is *"Option B — one number, lowered to 1.95."*
   `Cargo.toml:26` still reads `rust-version = "1.97.1"`, `rust-toolchain.toml` still pins
   `1.97.1`, the discharge queue does not contain it, and `kb-decision-0037` — accepted,
   immutable — is titled *"…and the number does not move."* This wave does **not** supersede
   ADR-0037. `02`, Adjudication 24, and it is the wave's most contestable call.
3. **Two intake files that name the same open question in their own voices.** The poll-count
   file's *"a second limitation"* and the ADR-0024 file's *"whether the suite should be able to
   catch an off-poll adapter's visibility defect at all"* are the same question, and both
   digests flagged it as must-be-one-atom. `02`, Adjudication 3.
4. **Nine intake files converging on one playbook.** Citation drift, anchor ambiguity, repoint
   idempotence and the doctest-fence blind spot arrive from nine separate briefs plus the
   discharge document's first generalisation. `kb-playbook-anchoring-citations-001` takes all of
   them as one merge rather than nine. `02`, Adjudication 26.
5. **A decision whose ADR number was reserved four phases ago.** ADR-0024 is not the next free
   number — it is `RUNBOOK.md:391`'s reserved queue row for exactly this subject, and
   `references/adr/0024-position-visibility-mechanism.md` already exists as the long-form
   record. The atom takes the reserved number, not `0038`. `02`, Adjudication 1.
6. **Deciding what a resolved open question receives when its own mechanism has been deleted.**
   `kb-open-question-no-ps-rule-name-resolved-001` cites two line numbers inside a guard that no
   longer exists and cannot be repointed by anchor. The corpus's own resolution protocol says
   the answer is a new atom; here the *question* also has to be re-cut, so this is the wave's
   only open-question **supersession**. `02`, Adjudication 45.

## The corpus, as verified

```
ls .kb/decisions      →  37 files: 0001–0023, 0029–0037, SD-0001, SD-0002, standalone-svg, README
                         35 decision atoms — 33 accepted, 2 superseded (0002, 0021)
                         0024–0028 are NOT missing: RUNBOOK.md:391-395 reserves them by subject
ls .kb/open-questions →  34 atoms, all authority_tier: note
ls .kb/reference      →  15 · playbooks 8 · maps 3 · governance 2 · concepts 1 · design 2 · product 0
ls references/adr     →  28 records — including 0024 and 0038, both already written long-form
```

| Layer | Atoms | Bearing on this wave |
| --- | --- | --- |
| `decisions/` | **35** (33 accepted) | **Twenty-three are added** — ADR-0024 (a reserved row) and ADR-0038 … ADR-0059. **No accepted body is edited and no decision is superseded.** Three accepted atoms are amended *from outside* on the ADR-0029-over-ADR-0004 precedent: `kb-decision-0001` and `kb-decision-0035` (by ADR-0038), and `kb-decision-0015` (by ADR-0043) |
| `open-questions/` | 34 | **Thirty-nine are added** and **fifteen existing atoms receive an operation** — of those fifteen, **four are resolved** (poll-count, postgres-arm-c, event-metadata-floor, query-plan-chunking) and **one is superseded** (no-ps-rule-name), the wave's only supersession of any kind |
| `reference/` | 15 | **Eight are added.** No existing reference atom is extended — the dating rule, sixth wave running |
| `playbooks/` | 8 | **Two are added, two are extended.** `anchoring-citations` takes a nine-file merge |
| `concepts/` | **1** | **Two are added.** The layer has held one atom since it was scaffolded |
| `governance/` | 2 | **One is extended** — `what-may-refute-a-finding` gains the stale-plan-of-record instance |
| `maps/` | 3 | All three inherit work: `decision-map` twenty-three rows, `domain-map` several sections, `open-questions-index` roughly fifty bullet changes |
| `design/`, `product/` | 2, 0 | No bearing. Nothing staged is a persona, a journey, or an interaction pattern |

For authority purposes:

- **Thirty-three accepted decision atoms exist and the intake names eleven of them.** ADR-0001,
  ADR-0035, ADR-0029, ADR-0004, ADR-0037, ADR-0013, ADR-0015, ADR-0012, ADR-0020, ADR-0022 and
  ADR-0036. **Not one body is edited, and not one `status` is flipped.**
- **Zero decision supersessions.** Fourth wave running. Four staged files refuse the edit in
  their own voices — the ADR-0038 file's *"neither body changes, only the exemption set moves"*,
  the ADR-0024 file's *"no edit to kb-decision-0013 (accepted, immutable)"*, the
  event-metadata brief's own correction routing an **amendment** rather than a supersession, and
  the ratification file's opening line: *"an accepted decision atom is immutable and authored by
  that command."*
- **One ratification this wave declines to sign.** See job 2 above, and Adjudication 24.

## Provenance check

Every path the staged files lean on was tested against this worktree.

```
references/adr/0024-position-visibility-mechanism.md            ✓ exists — the long form ADR-0024 cites
references/adr/0038-async-trait-through-testcontainers.md       ✓ exists — the long form ADR-0038 cites
deny.toml:74-90                                                 ✓ the testcontainers/tonic wrappers, argument at the point of use
experiments/position-visibility/results/adapter-remeasurement.{txt,md}
                                                                ✓ both exist — ADR-0024's two-number evidence
experiments/shipped-append-condition-sql/                       ✓ exists — the four-guard-shape re-measurement
experiments/one-connection-latency/                             ✓ exists — cited by TWO briefs, one reference atom
crates/happenstance-postgres/tests/naive_arm_probe.rs           ✓ exists — the off-poll limitation's demonstration
crates/happenstance/src/codec.rs:185                            ✓ fn reads_tag — codec-foreign-tag/A, landed
Cargo.toml:24,26 · rust-toolchain.toml                          ✓ version 0.2.0; rust-version AND channel both still 1.97.1
.kb/_intake/remediation-2026-09-04-briefs/                      ✓ 49 briefs + README, as the ratification file states
```

**One dangling reference, in two files.** `es-42-marker-earned-off-at-0-2-0.md` and
`ratifications-2026-09-06-pre-publication.md` both link `[[falsifier-ledger-reconciled-at-0-2-0]]`,
described as *"the repair that surfaced ES-42"*. No such atom exists in `.kb/`, no such file is
staged in `.kb/_intake/`, and no earlier wave ingested anything under that slug. It is carried to
`unresolved[]` rather than guessed at; the two atoms that would have linked it link each other
instead.

## Scoring — the merge candidates that decided a disposition

Scores are lexical overlap against the live atom, and they decide `merge_existing` vs
`create_new` only where the *subject* also matches. A high score against an atom that answers a
different question is a `related` edge, not a merge.

| Cluster | Best live candidate | Score | Disposition | Why |
| --- | --- | --- | --- | --- |
| ADR-0038, async-trait wrappers | `kb-decision-0035` | 75 | `create_new` | 0035 is **accepted and immutable**; a third wrapper set is a new commitment amending it from outside, the shape 0035 itself used on 0001 |
| ADR-0024, the mechanism | `kb-open-question-postgres-arm-c-cost-001` | 80 | `create_new` + resolve | The high score is against the question this decision *answers*, not a body to merge into |
| poll-count calibration | `kb-open-question-poll-count-rule-strength-001` | 97 | `merge_existing` → resolve | Direct owner; its own sub-questions 2, 3 and 4 are answered |
| off-poll limitation ×2 files | (none) | 45 max | `create_new` | Genuinely new; adjacent to poll-count but not the same question — a poll-based schedule cannot reach it *at all* |
| ES-42 freeze | `kb-decision-0011` | 85 | `create_new` | 0011's E11 wrapper is the *evidence*; the freeze is a new maturity commitment |
| `event-metadata-floor` | `kb-open-question-event-metadata-floor-001` | 92 | decision + resolve | The question is the owner and the ratification answers it; ADR-0015 is amended, never edited |
| `msrv-premise` | `kb-decision-0037` | 85 | **`defer_open_question`** | Conflicts with an accepted decision AND with the tree; see Adjudication 24 |
| citation drift ×9 files | `kb-playbook-anchoring-citations-001` | 90 max | `merge_existing` | Direct owner, already carrying a `## A second instance` section of exactly this shape |
| CF-36 breaches | `kb-open-question-cf-36-unperformed-cross-reference-001` | 95 | `merge_existing` | The check now exists; the atom describes its absence |
| ADR-0022 falsifiers ×3 files | `kb-open-question-adr-0022-falsifiers-fired-001` | 90 max | `merge_existing` | Three briefs fire three different falsifiers at one atom that already names all three |
| `read-page-budget` A | `kb-open-question-read-page-budget-001` | 95 | decision + annotate | **A** is ratified and **B** stays open, so the question survives its own partial answer |
| `no-ps-rule-name` | itself | 95 | **`supersede`** | Its mechanism was deleted; two of its citations cannot be repointed by anchor |
| `stringified-throw` dead code | `kb-decision-0023` | 15 | `create_new` | 0023 mentions the type incidentally; there is no owner |
| ES-22 arm two ×2 files | `kb-decision-0010` | 42 | `create_new` (reference) | A measurement, and `reference/README.md` forbids a decision from carrying its own evidence |
| one-connection-latency ×2 files | `kb-reference-busy-timeout-margin-001` | 45 | `create_new` | A different experiment directory, a different metric; the dating rule forbids extending a dated atom |
| `event-clone-allocations` restatement | `kb-reference-event-clone-allocations-001` | 94 | **no operation** | Same experiment, same numbers, two extra rows. Nothing new; the dating rule forbids the edit and the duplicate earns no second atom |

## Cross-file clusters, enumerated

Twenty-one clusters span more than one intake file. Each collapses to **one** destination atom.

| Cluster | Files | Destination |
| --- | --- | --- |
| Off-poll visibility limitation | poll-count, adr-0024 | `open-questions/off-poll-adapter-visibility-defect-undetected.md` |
| F2-5 hold and its discharge | f2-5, es-22-arm-two | `decisions/0040-…` + `reference/mutation-coverage-arm-two-…` |
| CF-5 per rule or per branch | es-22-arm-two, f2-5 | `open-questions/cf-5-conformant-control-per-rule-or-per-branch.md` |
| `cf-18` / B3 | cf-18 brief, ratifications, discharge | `decisions/0051-…` |
| `codec-foreign-tag-resolution` / A | codec brief, ratifications, discharge | `decisions/0049-…` |
| Sealing `Codec` | codec brief, discharge | `open-questions/should-codec-be-sealed.md` |
| `stringified-throw-visibility` / B | brief, ratifications, discharge | `decisions/0050-…` |
| `empty-decision-outcome` / 3 | brief, ratifications, discharge | `decisions/0046-…` |
| `tags-scope-agreement` / A+B | brief, ratifications, discharge | `decisions/0047-…` |
| `op-read-non-exhaustive` / A | brief, ratifications, discharge | `decisions/0048-…` |
| Citation anchoring and drift | 9 files (see Adjudication 26) | `playbooks/anchoring-citations-…` (merge) |
| What the exact-anchor rule leaves open | citation-anchor-slack, query-partition, sole-evidence-pins | `open-questions/what-the-exact-anchor-rule-still-leaves-open.md` |
| Counts and indices that drift | discharge, ratifications | `playbooks/a-count-or-an-index-nobody-re-derives.md` |
| ADR-0022 falsifiers | append-condition-sql-shape, transient-contention, sqlite-blocking-seam | `open-questions/adr-0022-falsifiers-have-fired.md` (merge) |
| `one-connection-latency` | sqlite-blocking-seam, read-page-budget | `reference/one-connection-latency-2026-09.md` |
| Query-plan two-axis chunking | query-partition, append-condition-sql-shape | `open-questions/query-plan-parameter-chunking-incomplete.md` (resolve) |
| Stale `0.0.0` name reservations | adapter-driver-reexport, repository-url | `open-questions/stale-0-0-0-name-reservations.md` |
| Cloudflare feature table | adapter-driver-reexport, stringified-throw | `open-questions/cloudflare-worker-feature-gate.md` |
| CF-33 / CF-34 scope outside the testkit | benchmark-completion, timed-assertions | `open-questions/cf-33-cf-34-scope-outside-the-testkit.md` |
| Fixture declension policy | fixture-declension, projection-declension, read-fault | `decisions/0042-…` (+ two residual questions) |
| Model-family rule composition | op-read, model-family-case-floor | `open-questions/model-family-rule-has-no-clause.md` (merge) |

## Corrections to a staged framing

**Correction 1 — the ADR numbering is not sequential and must not be made so.** Four staged
digests proposed `0038` for four different subjects and two proposed `00XX`.
`RUNBOOK.md:391-395` reserves `0024`–`0028` by subject (Postgres visibility, Ladybug, sync peer,
merge rule, forgetting), and `references/adr/0024-…` and `references/adr/0038-…` already exist.
ADR-0024 takes its reserved row; everything else in this wave starts at `0038` and runs to
`0059`. `0025`–`0028` stay reserved and unclaimed.

**Correction 2 — `0.2.0` is not "about to publish".** Several briefs price their cost of delay
as *"free until `0.2.0`"* and were written on 2026-09-04 against `0.2.0-alpha.1`.
`Cargo.toml:24` now reads `version = "0.2.0"`. Every atom this wave writes states the deadline in
the past tense where the deadline has passed, and says so rather than inheriting the brief's
tense. Two questions change character because of it: sealing `Codec` and the emitter-name
promise both lose the free window their briefs assumed.

**Correction 3 — a brief's provenance discount travels with its claims.** Roughly half the fifty
briefs state in their own voice that they did **not** get the author → two-critic → revision pass
the original thirteen had. That is not a reason to drop a claim, and it is not a reason to hide
it: each atom sourced from a single-pass brief carries the discount in its own summary, in the
brief's own words. The ratification file's `## Method` section is what makes this legible, and it
is recorded once — here, and in each atom's summary — rather than argued fifty times.
