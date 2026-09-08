# Wave `2026-09-07-intake` — placement and adjudication

**Ninety-one operations**, ordered so that every atom a later operation links to exists before the
link is written. Fifty-seven staged files, roughly two hundred claims, and **twenty-one cross-file
clusters** — every one of which collapses to a single destination atom.
`00-corpus-match.md` establishes why.

**What this wave does to the accepted decision corpus: nothing destructive.** Twenty-three
decision atoms are added. **No accepted body is edited, no `status` is flipped on any decision,
and no decision is superseded** — fourth wave running. Three accepted decisions are amended *from
outside* on the precedent ADR-0029 set against ADR-0004: ADR-0001 and ADR-0035 by ADR-0038, and
ADR-0015 by ADR-0043. One ratification is **declined** and routed to an open question instead
(Adjudication 24). The wave's only supersession is of an *open question* whose mechanism was
deleted from the tree (Adjudication 45).

## Ordering

```
Ops  1– 8   reference atoms      — the evidence, before anything cites it
Ops  9–31   decision atoms       — the commitments, before the questions that link them
Ops 32–33   concept atoms
Ops 34–35   playbook atoms (new)
Ops 36–73   open-question atoms (new)
Op  74      the open-question supersession
Ops 75–77   playbook and governance merges
Ops 78–91   open-question merges into existing atoms
            ── the Maps phase runs after all ninety-one ──
```

References first because eleven decisions cite one. Decisions before open questions because
thirty-one of the thirty-eight new questions carry a `related` edge to a decision this wave signs.
Merges last because every merge writes a link to an atom created earlier in the same wave.

---

## The operation table

`adr` = `2026-09-06-adr-0038-…` · `pv` = `2026-09-07-adr-0024-…` · `poll` =
`2026-09-06-poll-count-…` · `rat` = `ratifications-2026-09-06-pre-publication.md` · `dis` =
`2026-09-07-ratifications-discharged-…` · `es42` = `es-42-marker-earned-off-at-0-2-0.md` ·
`f25` = `f2-5-holds-the-release-for-phase-10.md` · everything else is a brief basename under
`remediation-2026-09-04-briefs/`.

### Ops 1–8 · reference atoms

| # | Destination | Sources | Class |
| --- | --- | --- | --- |
| 1 | `reference/position-visibility-adapter-remeasurement-2026-09.md` | `pv` | extends |
| 2 | `reference/mutation-coverage-arm-two-measurement-2026-09.md` | `es-22-arm-two…`, `f25` | extends |
| 3 | `reference/one-connection-latency-2026-09.md` | `sqlite-blocking-seam`, `read-page-budget…` | extends |
| 4 | `reference/shipped-append-condition-sql-experiment-2026-09.md` | `append-condition-sql-shape` | extends |
| 5 | `reference/intake-citation-drift-census-2026-09.md` | `docs-citation-form-and-clause-content` | extends |
| 6 | `reference/model-family-case-count-cliff-2026-09.md` | `model-family-case-floor` | extends |
| 7 | `reference/spec-trace-unresolved-rule-declarations-2026-09.md` | `prose-guard-retired-and-what-it-owes` | extends |
| 8 | `reference/phase-7-macros-ceremony-second-example-2026-09.md` | `adr-0033-reopen-ground` | extends |

### Ops 9–31 · decision atoms

| # | ADR | Destination | Sources | Landed |
| --- | --- | --- | --- | --- |
| 9 | **0024** | `decisions/0024-position-visibility-mechanism.md` | `pv`, `poll` | yes |
| 10 | 0038 | `decisions/0038-async-trait-through-testcontainers.md` | `adr` | yes |
| 11 | 0039 | `decisions/0039-es-42-frozen-without-an-unpin-bound.md` | `es42` | yes |
| 12 | 0040 | `decisions/0040-f2-5-holds-the-release-for-phase-10.md` | `f25`, `es-22-arm-two…` | yes, discharged |
| 13 | 0041 | `decisions/0041-the-repository-is-public-and-security-has-an-email-channel.md` | `repository-url-…` | yes |
| 14 | 0042 | `decisions/0042-every-fixture-capability-lands-defaulted.md` | `fixture-declension-policy`, `projection-declension-obligations`, `read-fault-clause-…`, `stated-only-defects-…`, `rat` | yes |
| 15 | 0043 | `decisions/0043-event-metadata-gets-a-refusal-channel-with-no-floor.md` | `event-metadata-floor`, `rat` | no |
| 16 | 0044 | `decisions/0044-a-published-crate-re-exports-its-drivers.md` | `adapter-driver-reexport-policy` | yes |
| 17 | 0045 | `decisions/0045-a-citation-anchor-matches-exactly-or-the-lint-refuses.md` | `citation-anchor-slack`, `dis`, `sole-evidence-pins-…` | yes |
| 18 | 0046 | `decisions/0046-commit-reports-a-nothing-to-do-outcome.md` | `empty-decision-outcome`, `rat`, `dis` | `0ae10ab` |
| 19 | 0047 | `decisions/0047-tags-and-scope-must-agree-at-commit.md` | `tags-scope-agreement`, `rat`, `dis` | `0ae10ab` |
| 20 | 0048 | `decisions/0048-op-read-is-non-exhaustive.md` | `op-read-non-exhaustive`, `rat`, `dis` | `9000f35` |
| 21 | 0049 | `decisions/0049-a-codec-declares-the-tags-it-reads.md` | `codec-foreign-tag-resolution`, `rat`, `dis` | `2d37fc4` |
| 22 | 0050 | `decisions/0050-stringified-throw-is-crate-private.md` | `stringified-throw-visibility`, `rat`, `dis` | `9225c00` |
| 23 | 0051 | `decisions/0051-declension-by-inheritance-discharges-cf-18.md` | `cf-18-observable-skip-reporting`, `rat`, `dis` | `3df4b6e` |
| 24 | 0052 | `decisions/0052-the-query-partition-constants-stay-public.md` | `query-partition-public-surface`, `rat` | yes |
| 25 | 0053 | `decisions/0053-a-read-page-is-bounded-in-rows-and-in-bytes.md` | `read-page-budget-…`, `rat` | yes |
| 26 | 0054 | `decisions/0054-after-opt-applies-to-every-guard-and-says-so.md` | `after-opt-scope`, `rat` | no |
| 27 | 0055 | `decisions/0055-append-keeps-its-borrowed-batch-at-0-2-0.md` | `append-batch-ownership`, `rat` | partly |
| 28 | 0056 | `decisions/0056-wf-10s-rule-line-is-repaired-not-amended.md` | `wf-10-instruments-…`, `rat` | no |
| 29 | 0057 | `decisions/0057-the-testkit-version-key-is-dropped.md` | `testkit-dev-dependency-…`, `rat` | no |
| 30 | 0058 | `decisions/0058-the-sqlite-write-path-stays-inline-and-documents-it.md` | `sqlite-blocking-seam`, `rat` | W1 only |
| 31 | 0059 | `decisions/0059-the-domain-event-guard-checks-positional-agreement.md` | `domain-event-guard-and-decode`, `rat` | A+E only |

### Ops 32–35 · concepts and new playbooks

| # | Destination | Kind | Sources |
| --- | --- | --- | --- |
| 32 | `concepts/a-mutation-kind-needs-a-tethered-bar.md` | concept | `stated-only-defects-…` |
| 33 | `concepts/growing-a-sealed-trait-is-not-a-breaking-change.md` | concept | `tuple-boundary-event-type` |
| 34 | `playbooks/a-count-or-an-index-nobody-re-derives.md` | playbook | `dis`, `rat` |
| 35 | `playbooks/require-the-property-not-the-mechanism.md` | playbook | `typed-layer-promise-guard` |

### Ops 36–73 · new open questions

| # | Destination (under `open-questions/`) | Sources |
| --- | --- | --- |
| 36 | `off-poll-adapter-visibility-defect-undetected.md` | `poll`, `pv` |
| 37 | `cf-5-conformant-control-per-rule-or-per-branch.md` | `es-22-arm-two…`, `f25` |
| 38 | `msrv-ratification-conflicts-with-the-accepted-floor.md` | `msrv-premise`, `rat` |
| 39 | `postgres-fixture-read-fault-declension-is-owed.md` | `dis`, `read-fault-clause-…` |
| 40 | `read-fault-rule-has-no-clause.md` | `read-fault-clause-…` |
| 41 | `cf-18-residuals-after-declension-by-inheritance.md` | `cf-18-…`, `dis` |
| 42 | `cf-23-emitter-names-mandatory-and-marked-unstable.md` | `emitter-surface-stability` |
| 43 | `cf-25-cf-26-portfolio-check-does-not-exist.md` | `portfolio-check-…`, `es42` |
| 44 | `gate-step-first-check-hides-its-second.md` | `a-gate-step-…` |
| 45 | `vt-30-provisional-marker-is-stale-and-unscheduled.md` | `after-opt-scope` |
| 46 | `adapter-version-lockstep-and-cf-32.md` | `adapter-driver-reexport-policy` |
| 47 | `happenstance-facade-does-not-match-adr-0006.md` | `adapter-driver-reexport-policy` |
| 48 | `stale-0-0-0-name-reservations.md` | `adapter-driver-reexport-policy`, `repository-url-…` |
| 49 | `event-type-positional-mapping-has-no-compiler-check.md` | `adr-0033-reopen-ground` |
| 50 | `references-adr-in-place-correction-policy.md` | `append-batch-ownership` |
| 51 | `cf-17-cf-14-maturity-markers-and-the-reopen-must.md` | `stated-only-defects-…` |
| 52 | `model-only-kind-memberless-dormant-or-withdrawn.md` | `model-only-kind-…` |
| 53 | `read-to-backwards-limit-composition-gap.md` | `model-only-kind-…` |
| 54 | `cf-33-cf-34-scope-outside-the-testkit.md` | `timed-assertions-…`, `benchmark-completion-…` |
| 55 | `es-23-frozen-doc-musts-adapter-half.md` | `frozen-doc-musts-…` |
| 56 | `cf-38-case-naming-no-clause-reading.md` | `which-reading-of-a-case-…` |
| 57 | `es-18-byte-identical-versus-conformance-reading.md` | `unswept-rows-…` |
| 58 | `probe-read-through-signature-and-live-transaction-seam.md` | `probe-read-through-…` |
| 59 | `projection-batch-sql-seam-statement-type.md` | `projection-batch-sql-seam` |
| 60 | `projection-runner-chunk-type-and-observation-seam.md` | `projection-runner-chunk-…` |
| 61 | `reset-refusal-declension-has-no-clause.md` | `projection-declension-obligations` |
| 62 | `testkit-projection-module-unstable-projection-exemption-scope.md` | `fixture-declension-policy` |
| 63 | `tuple-boundary-heterogeneous-event-type.md` | `tuple-boundary-event-type` |
| 64 | `trait-variant-caret-resolves-past-the-locked-gate.md` | `trait-variant-caret-…` |
| 65 | `sole-evidence-pin-requirement-generality.md` | `sole-evidence-pins-…` |
| 66 | `what-the-exact-anchor-rule-still-leaves-open.md` | `citation-anchor-slack`, `query-partition-…`, `sole-evidence-pins-…` |
| 67 | `docs-citation-anchor-form-and-clause-contradiction-check.md` | `docs-citation-form-…` |
| 68 | `es-11-ceiling-sample-cost-on-sqlite-read.md` | `sqlite-blocking-seam` |
| 69 | `then-empty-emission-idiom-and-the-nothing-to-do-channel.md` | `empty-decision-outcome` |
| 70 | `scope-coverage-helper-and-the-projection-port-gap.md` | `tags-scope-agreement` |
| 71 | `cloudflare-worker-feature-gate.md` | `adapter-driver-reexport-policy`, `stringified-throw-…` |
| 72 | `rustdoc-citations-relative-or-url-shaped.md` | `repository-url-…` |
| 73 | `should-codec-be-sealed.md` | `codec-foreign-tag-resolution`, `dis` |

### Ops 74–91 · mutations

| # | Op | Destination | Sources |
| --- | --- | --- | --- |
| 74 | **`supersede`** | `open-questions/is-the-dagger-convention-superseded-by-maturity-markers.md` ← `no-ps-rule-name-is-resolved.md` | `prose-guard-retired-…` |
| 75 | `merge_existing` | `playbooks/anchoring-citations-in-a-long-lived-document.md` | nine files |
| 76 | `merge_existing` | `playbooks/landing-a-stricter-gate-without-a-red-baseline.md` | `cf-36-thirteen-recorded-breaches` |
| 77 | `merge_existing` | `governance/what-may-refute-a-finding.md` | `f25`, `domain-event-guard-…`, `es-22-arm-two…` |
| 78 | `merge_existing` | `open-questions/poll-count-bounds-the-visibility-rule.md` → **resolved** | `poll` |
| 79 | `merge_existing` | `open-questions/postgres-arm-c-structural-cost.md` → **resolved** | `pv` |
| 80 | `merge_existing` | `open-questions/event-metadata-has-no-declared-floor.md` → **resolved** | `event-metadata-floor`, `rat` |
| 81 | `merge_existing` | `open-questions/query-plan-parameter-chunking-incomplete.md` → **resolved** | `query-partition-…`, `append-condition-sql-shape` |
| 82 | `merge_existing` | `open-questions/global-versus-per-boundary-visibility-invariant.md` | `pv` |
| 83 | `merge_existing` | `open-questions/adr-0022-falsifiers-have-fired.md` | `append-condition-sql-shape`, `transient-contention-…`, `sqlite-blocking-seam` |
| 84 | `merge_existing` | `open-questions/no-fixture-tolerance-for-transient-contention.md` | `transient-contention-…` |
| 85 | `merge_existing` | `open-questions/cf-36-names-a-cross-reference-nothing-performs.md` | `cf-36-…`, `stated-only-defects-…` |
| 86 | `merge_existing` | `open-questions/es-17-two-adapter-measurement-is-unscheduled.md` | `append-batch-ownership` |
| 87 | `merge_existing` | `open-questions/read-page-budget-is-unspecified.md` | `read-page-budget-…`, `sqlite-lane-spec-citation-repoints` |
| 88 | `merge_existing` | `open-questions/es-6-names-an-unwritable-rule.md` | `prose-guard-…`, `stated-only-defects-…`, `adapter-driver-reexport-policy` |
| 89 | `merge_existing` | `open-questions/disjoint-boundaries-have-no-clause.md` | `sole-evidence-pins-…` |
| 90 | `merge_existing` | `open-questions/model-family-rule-has-no-clause.md` | `op-read-non-exhaustive`, `model-family-case-floor` |
| 91 | `merge_existing` | `open-questions/no-workerd-class-runner-in-the-gate.md` | `query-partition-public-surface` |

---

## The adjudications that were not obvious

### Adjudication 1 — ADR-0024 takes a reserved number, not the next one

**Op 9** · `create_new` · `.kb/decisions/0024-position-visibility-mechanism.md`

Five staged digests proposed a next-free ADR number and four of them proposed `0038`. The corpus's
numbering is not a sequence with a gap in it: `RUNBOOK.md:391-395` reserves `0024`–`0028` by
subject, and `0024`'s row reads *"Postgres: how does the adapter buy the position-visibility
invariant when `nextval()` allocates outside the transaction — measured, not preferred?"* —
verbatim this decision. `references/adr/0024-position-visibility-mechanism.md` already exists as
the long form. Taking `0038` here would have left the reserved row permanently unfillable and put
two records for one decision under two numbers.

```yaml
id: kb-decision-0024
title: happenstance-postgres buys position visibility with xid8 and pg_snapshot_xmin
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0024
reversibility: medium
phase: 10
supersedes: null
superseded_by: null
```

**What it must not do.** Edit `kb-decision-0013`. ADR-0013 lifted ES-10 to `[FROZEN]` on the
strength of one affordable mechanism existing, and explicitly left *which* mechanism an adapter
buys to phase 10. This atom answers that and nothing else: no spec clause is added, ES-10's
`[FROZEN]` marker does not move, and ADR-0013 stays accepted and byte-identical.

**The load-bearing subtlety a summary would lose.** CF-13 still passes — and *the pass is not
evidence*. `crates/happenstance-postgres/tests/naive_arm_probe.rs` builds an arm with the frontier
predicate removed and it passes the same rule. The mechanism is evidenced by **direct staleness
observation** (Op 1's reference atom), not by a green conformance run. An atom that recorded
"conformant, therefore correct" would be recording the exact inference the probe was built to
refute.

**`mapsImpact`:** `decisionMap`, `domainMap`, `openQuestionIndex` — Ops 79 and 82 change two
bullets, and Op 36 adds one.

### Adjudication 2 — the phase-2 experiment atom is not extended

**Op 1** · `create_new` · `.kb/reference/position-visibility-adapter-remeasurement-2026-09.md`

`kb-reference-position-visibility-experiment-001` is high-scoring (68) and is the wrong home. It is
scoped to *"the phase-2 four-arm SQL-strategy experiment"* and pinned to it; this is a later,
adapter-level remeasurement with a different methodology (built adapter versus paired naive arm,
not four SQL strategies) and its own results files. The dating rule that has held for five waves
says a dated measurement is not repaired, it is joined by a newer one.

The digest offered a lower-bias alternative — fold the numbers into ADR-0024's body and skip the
atom. Declined on `reference/README.md`'s own separation rule: *"the measurement a decision rests
on is a `reference` atom that this one cites. Separating them is what lets a decision be
superseded without invalidating the evidence underneath it."* ADR-0024's staleness bound is exactly
the kind of number a later adapter change would move.

### Adjudication 3 — one off-poll question, not two

**Op 36** · `create_new` · `.kb/open-questions/off-poll-adapter-visibility-defect-undetected.md`

Both digests flagged this and both were right. `poll` supplies the mechanism narrative — an
adapter whose `append` hands work to a runtime advances on the runtime's schedule, so *no*
poll-based schedule can decide when to interleave — plus the demonstration
(`naive_arm_probe.rs`, a naive arm passing CF-13 even after the schedule fix). `pv` supplies the
framing that makes it decidable — the port exposes no suspension point between allocation and
commit, and the instrument that does catch it is adapter-specific. One question, two halves,
one atom.

**Why it is not a merge into `poll-count-bounds-the-visibility-rule`.** That atom's question is
*how to calibrate a poll count*; this one's is *what to do when poll counting cannot reach the
defect at all*. Calibrating the first does not touch the second — the schedule fix landed and
the naive arm still passes. Scoring the two at 45 was generous.

### Adjudication 4 — the poll-count question is resolved, and its answer is a code change

**Op 78** · `merge_existing` → `status: superseded`

`open-questions/README.md`: *"the answer is a new atom … move this atom's `status` to `withdrawn`
or `superseded`, and leave the body describing what was not known at the time."* Three of its
four ordered sub-questions are answered by `poll` — n comes from a real adapter (3, not a
synthetic worst case); only the schedule changes and ES-10 does not; and sub-question 2, the
`POLL_BUDGET` fixture capability, is **moot** because the new schedule counts no polls at all.
That last one matters beyond this atom: ADR-0013's CF-33 tension, which the atom's own 2026-08-20
amendment carried forward, never has to be resolved.

The atom is superseded rather than withdrawn because there is a successor question (Op 36) that
inherits the same rule's blind spot from the other side.

### Adjudication 5 — ADR-0038 amends two accepted decisions and edits neither

**Op 10** · `create_new` · `.kb/decisions/0038-async-trait-through-testcontainers.md`

`kb-decision-0035` scores 75 and is immutable. It is also the *precedent*: 0035 amended 0001's
exemption set from outside without touching 0001's body, and 0029 did the same to 0004. A third
wrapper pair is the same shape a third time.

The claim a summary would flatten, and must not: **three different arguments sit behind one
`wrappers` list.** `wasm-bindgen-test` is exempt because it ships in nothing.
`worker`/`worker-macros` ship, and are exempt on the narrower ground that they use the macro for
their *own* traits from which no happenstance port is derived. `testcontainers`/`tonic` are the
first argument again, and more cleanly than the crate it was written for — a dev-dependency
stripped from the published manifest, invisible to `cargo hack --no-dev-deps`. Collapsing that to
*"async-trait is allowed for test and platform crates"* would license the fourth route the atom
exists to keep failing.

**The cost, recorded rather than discovered.** `deny.toml`'s `wrappers` list matches only direct
parents, so naming `tonic` admits `async-trait` from anywhere in the graph, including a crate that
ships. That is the one real weakening and the atom says so in its own summary.

### Adjudications 6–23 — the ratified briefs, and why each is one atom

The pattern is identical across eighteen decisions and is stated once here rather than eighteen
times.

**Three documents, one commitment.** For each ratified brief there is (a) the brief, carrying the
options, the rejected alternatives and the cost of delay; (b)
`ratifications-2026-09-06-pre-publication.md`, carrying the sign-off and, for the eight reviewed
individually, the owner's own reason; and (c) for six of them,
`2026-09-07-ratifications-discharged-…`, carrying the commit and whether execution matched.
`decisions/README.md` requires the atom to *"state the alternatives that lost and why"*, which is
(a); `status: accepted` is (b); and the landing state is (c). One atom holds all three, and
`sourceFiles` lists every contributing file.

**Where execution changed the ratification, the atom records what landed — not what was signed.**
Three did:

- **ADR-0051 / `cf-18`** (Op 23). The brief's B3 asked for a test that fails *"if any capability
  is declined without an accompanying declaration"*. `Capability::declined` asserts a non-empty
  reason and `SUPPORTED` *is* the absence of one, so that state cannot be constructed and the rule
  would have passed on every fixture that will ever exist. What landed is **declension by
  inheritance**: five capabilities default to a declension carrying the testkit's own prose. The
  atom is titled for what landed. Its most valuable sentence is the one about the reference
  implementation — `MemoryFixture` failed the new rule three lines below its own comment arguing
  the principle. *A rule everyone agrees with and nothing checks holds until the second time
  somebody is busy.*
- **ADR-0049 / `codec-foreign-tag-resolution`** (Op 21). `Codec::reads_tag` landed as ratified and
  is *per codec* — the orphan rule makes that a hard boundary. It serves *"my codec also reads the
  tag I used to write"* and not the migration `Codec`'s own page had been using as its cautionary
  story. A registry would answer the second and costs global mutable state; sealing stays open
  (Op 73).
- **ADR-0050 / `stringified-throw-visibility`** (Op 22). `pub(crate)` as ratified, plus the
  consequence the brief did not name: `pub` methods on a `pub` type are a configuration in which
  `dead_code` can never fire, so narrowing made the compiler look and found two methods with no
  caller anywhere. The atom also records the rejected tidier fix — `#[cfg(test)]` on the whole
  type — because it would put ES-6's recorded alternative in the test profile and its counterpart
  in the shipped binary, making the two shapes the fork compares incomparable in one build.

**Where the ratification obliges code that is not written, the atom says so** and does not pretend
otherwise: ADR-0043, ADR-0054, ADR-0056, ADR-0057 and the `B` half of ADR-0059 all carry an
explicit obligation-outstanding sentence.

### Adjudication 15 — `Event::metadata` is amended, not superseded

**Op 15** · `create_new` · `.kb/decisions/0043-…` · **Op 80** · resolve the question

The brief supplies its own routing correction and it is right: ADR-0015's *"three variants, not
four"* reasoning is **correct on its own terms and survives intact**. What is missing is a
candidate it never enumerated, not a wrong argument to repair. That is the ADR-0029-over-ADR-0004
shape (`supersedes: null`, `depends_on: [kb-decision-0015]`), not the ADR-0032-over-ADR-0021 shape.
The previous wave declined to sign this supersession precisely because the review *named* the
question rather than answering it; the ratification answers it, so it is signable now — as an
amendment.

The decisive new evidence, absent from the existing question: `happenstance-sync` already treats
`data` and `metadata` as **one summed budget** in shipped code
(`ReplicatedEvent::payload_len`, `PeerLimits::admits`). That falsifies the question's implicit
premise that no port has a metadata term, and it is what flipped the brief from Option 1 to
Option 2. A separately-*floored* metadata would be incommensurable with `payload_len`.

The atom must be honest that Option 2 was preferred over Option 3 without being ranked on
evidence: it is the reversible minimum under acknowledged uncertainty, and that is the rationale,
stated as such.

### Adjudication 24 — the ratification this wave declines to sign

**Op 38** · `defer_open_question` ·
`.kb/open-questions/msrv-ratification-conflicts-with-the-accepted-floor.md`

This is the wave's hardest call and it is made against the ratification.

`msrv-premise` is one of the nine ratified *on their own recommendation*, and its
`## Recommendation` reads, verbatim: **"Option B — one number, lowered to 1.95."** Under the
ratification file's own rule — *"Each brief's `## Recommendation` is the decision"* — that is a
signed decision to lower the workspace MSRV.

Three facts sit against writing it as one:

1. **`kb-decision-0037` is accepted, immutable, and titled *"The MSRV becomes a promise at 0.2.0,
   and the number does not move."*** Writing ADR-00xx *"the number moves to 1.95"* three days
   later is a supersession, and a supersession needs the old atom's `status` flipped — an edit
   this wave would be making on the strength of a table row rather than a decision record.
2. **The tree disagrees.** `Cargo.toml:26` reads `rust-version = "1.97.1"` and
   `rust-toolchain.toml` pins `1.97.1`. `msrv-premise` is not in the discharge queue and nothing
   in the intake says it landed. A decision atom asserting a 1.95 floor would be asserting
   something false about the repository on the day it was written.
3. **The brief's own premise correction is a second, separate conflict.** It shows by bisection
   that `cfg_select!` stabilised in **1.95**, not 1.88 — an error inside the bodies of *two*
   accepted decisions (`kb-decision-0029` and `kb-decision-0037`), off by seven releases. That
   error cannot be edited out and must not be silently carried forward.

`open-questions/README.md`'s first case, verbatim: *"an **unresolved conflict** between two
sources, or between a source and the shipped code — `/redkiln:kb-ingest` routes these here as
`defer_open_question` rather than picking a winner."* Both conflicts are exactly that. The atom
states what is true today (1.97.1, in both files), what was ratified (1.95), what the bisection
found, and what forces the choice (the next MSRV-relevant dependency bump, or the next release).
It is grounded and it takes no side.

**What this deliberately costs.** If the owner meant the ratification to bind, the floor is now
recorded as unsettled rather than as lowered, and someone has to say so again. That is the cheaper
error: a decision atom is immutable, and one written on a misread of a table row would need its
own supersession to undo.

### Adjudication 26 — nine files, one playbook merge

**Op 75** · `merge_existing` · `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`

The contributing files: `dis` (21 citations moved; 14 repointed, 7 refused, 6 of those already
ambiguous — two suite macros with byte-identical comments), `citation-anchor-slack` (88 of 323
green only on the slack; the offset distribution's secondary lobes at +4 and +9),
`a-gate-step-…` (V-6 passes a citation landing inside the *wrong construct*, three times, ~100
lines off), `docs-citation-form-…` (a citation contradicting the clause it cites; a bounds-only
check; a bare `:122` shorthand outside every parser), `projection-runner-chunk-…` (three citations
into a doctest fence, invisible because anchor derivation covers 80 of 401),
`sqlite-lane-spec-citation-repoints` (ten repoints by anchor, not by offset, because two ranges
drift by different amounts at their two ends), `query-partition-public-surface` (the repoint
script is not idempotent — a content check passing on `    }`), `sole-evidence-pins-…` (two
checker-flagged and two silently-stale citations), and `es-42` indirectly.

**One playbook, not nine, because they are one technique with nine instances.** The atom already
carries a `## A second instance` section of exactly this shape, so the merge extends a structure
the atom invented rather than imposing one. The single sentence that generalises all nine is the
discharge document's: *a citation anchor that matches many lines is already broken, and only a
line number is hiding it.*

**What splits off and why.** The *unresolved* residue does not belong in a playbook, which
carries no authority to leave a question open: `spec_trace`'s `ANCHOR_SLACK = 12` was never
derived from anything, no rule requires an anchor to be unique, and a shipped `--repoint` would
inherit the idempotence hazard. Those three are Op 66, one atom, from three files.

### Adjudication 33 — the exact-anchor rule is a decision, not a playbook addendum

**Op 17** · `create_new` · `.kb/decisions/0045-…`

`citation-anchor-slack` was **ratified and landed** (GATE-02-anchor-slack, Option C): the
`ANCHOR_SLACK` constant is removed from `lint_constitution.rs` entirely, a citation must
exact-match its anchor, and on a miss the checker names the line, or lists every candidate and
**refuses to choose**, or says the anchor is nowhere and asks for a human. That is three
must-shaped behaviours on a gate step, with three named rejected alternatives (A, B, D). Authority
rule 2 puts it in a decision atom; the playbook (Op 75) describes the technique and links here.

The atom must carry the asymmetry that makes it defensible: `spec_trace.rs`'s `ANCHOR_SLACK = 12`
is **deliberately kept**, because its anchor is a derived identifier pointing at evidence prose
rather than an explicit quoted string, so exact match is not the same claim there — and closing it
would redden 21 of 80 citations, 20 inside `[FROZEN]` commentary. Each constant's doc comment now
cites the other instead of silently disagreeing, which closes the residual defect
`kb-reference-phase-4-5-spec-reconciliation-001` recorded.

### Adjudication 40 — the F2-5 hold, and the measurement it rested on

**Op 12** · `create_new` · `.kb/decisions/0040-…` · **Op 2** · the reference

`f2-5` and `es-22-arm-two-is-reached-the-finding-is-wrong` overlap near-verbatim, including a
shared block quote, and they are two different kinds of thing. The **measurement** — arm 2 of
`dropped_append_future_leaves_no_partial_batch` is reached by `PreCommitPositionStore` and
`AwaitAcrossBorrowStore` on every run in both feature configurations, since the rule's first
commit `d480446` — is a reference atom, because `decisions/README.md` forbids the decision from
carrying its own evidence. The **decision** — `0.2.0` waits for phase 10, and the hold is lifted
the same day when `lane/postgres-neon-stores` turns out to have already discharged the residual —
is the decision atom, and it cites the reference.

**Why the decision atom exists at all when the hold lasted a day.** Because the argument on both
sides was recorded in full, and because the episode is evidence for something durable: *the
decision was sound on the information available and the information was wrong.* A stale status row
in `RUNBOOK.md` — reading phase 10 as "not started" and eleven days away — deferred a release,
inside the same file whose census was being repaired at that moment. That lesson goes to Op 77,
the governance merge, as an instance; the anecdote stays here.

### Adjudication 45 — the wave's only supersession, and it is a question

**Op 74** · `supersede` · `open-questions/no-ps-rule-name-is-resolved.md` →
`open-questions/is-the-dagger-convention-superseded-by-maturity-markers.md`

`kb-open-question-no-ps-rule-name-resolved-001` describes a mechanism — `spec_trace.rs`'s
`c.schedules_new || !has_suite(&c.id)` guard and the `Rules::schedules_new` field it read — that
`prose-guard-retired-and-what-it-owes` deleted. Two of the atom's own citations point at code that
no longer exists and **cannot be repointed by anchor**, which is exactly the failure Op 17's
decision makes the lint refuse to paper over.

Three options were on the table and the brief recommended the middle one. Editing the accepted
atom's body to describe deleted code is what `governance/rewrite-the-referent-never-the-reasoning`
exists to forbid. Leaving it is a question that describes a mechanism nobody can find. Superseding
it with a narrower atom carrying only its still-open sub-question 3 — *is the dagger convention
superseded by maturity markers?* — keeps the record of what was not known and stops asserting a
mechanism that is gone.

`open-questions/README.md` licenses this directly: *"the answer is a new atom — and the record of
the question stays … leave the body describing what was not known at the time."* Sub-questions 1
and 2 are now **measured**, not merely reasoned: the bare dagger fires on zero clauses by its
literal forms, only the backticked term does PS's work (10 hits), and retiring it produces zero
newly-failing PS clauses. Those measurements go to Op 7's reference atom with the 45-declaration
census; the question that survives goes to the successor.

### Adjudication 47 — two concept atoms in a layer that has held one since it was scaffolded

**Ops 32–33**

Neither is a commitment and neither has a decision owner, which is what makes them concepts.

`a-mutation-kind-needs-a-tethered-bar` records **why `Kind::StatedOnlyDefect` was withdrawn**: its
bar was satisfiable by a defect-free subject whose two `fn() -> String` pointers were string
literals, and adding the positive control the review demanded does not close it. `Witness` escapes
this because it holds `<T as Defect>::select` — the store's *own* function — and there is no
analogue for a fixture-level defect because `Fixture` is not `dyn`-compatible (RPITIT). What
replaced the kind is a named test that *states* rather than falsely checks:
`reopen_over_claiming_is_undetectable_and_this_is_the_record`. The durable idea is the tether, and
it is why `Kind::ModelOnlyMutant` — whose bar survived attack — is a different case (Op 52).

`growing-a-sealed-trait-is-not-a-breaking-change` corrects a rule stated backwards in
`boundary.rs`'s own doc comment and complements RS-40-1 rather than contradicting it: adding a
required item to a **sealed** trait cannot produce `E0046` downstream, because no downstream crate
contains an impl for the item to be missing from — every implementor reaches the trait through
this crate's blanket impl, which supplies the new item with the rest. RS-40-1's `E0046` claim is
true, and true only of an **unsealed** port a stranger implements. Two residual costs survive and
the atom names them: a new method name can create resolution ambiguity at a downstream call site,
and a new associated type becomes nameable in signatures that must then keep it.

### Adjudication 51 — one atom per clause, and why that is not fragmentation

Eleven of the new open questions are *"clause X names an instrument that does not exist / is
narrower than its own MUST"*. A reviewer could reasonably ask why they are not one atom about
clause-versus-instrument drift.

They are separate because **the corpus already answers this question in its own layout**:
`es-6-names-an-unwritable-rule`, `ps-19-scope-narrower-than-its-rule`,
`ps-1-states-no-progress-obligation`, `cf-36-names-a-cross-reference-nothing-performs`,
`cf-40-fixture-limits-ownership`, `es-38-and-gap-read-rules-are-unowned`,
`disjoint-boundaries-have-no-clause` and `model-family-rule-has-no-clause` are eight live atoms of
exactly this shape, one per clause. Each has a different owner, a different forcing event, and a
different answer. A single atom about the *class* would be a generalisation nobody can act on, and
the class already has a home — `landing-a-stricter-gate-without-a-red-baseline` (Op 76) and
`verify-the-referent-and-report-coverage` hold the transferable technique.

What *was* merged, on the same test: two CF-33/CF-34 scope questions from two files (Op 54),
because they are one clause pair with one owner; and three anchor residuals from three files
(Op 66), because they are one rule's leftovers.

### Adjudication 52 — three claims that earn no atom, recorded so nobody re-derives them

- **The `Event` clone allocation table** arrives a second time in `sqlite-blocking-seam` with two
  extra rows and identical figures. `kb-reference-event-clone-allocations-001` holds the
  experiment; the dating rule forbids extending it; two rows do not earn a second dated atom.
- **Telemetry publication scope** was a live question when `repository-url-…` was written and is
  answered by the act — the repository is public and `.redkiln/telemetry/` shipped with it. It is
  one sentence in ADR-0041's body, not a question.
- **The `PartialBatch` rename and the `PreCommitPositionStore` doc comment** are one-line code
  edits routed to whichever lane next opens those files. `open-questions/README.md`: *"work that
  someone is expected to do is a backlog item in `.bklg/`, not a KB atom."*

---

## What the Maps phase inherits

| Map | Work |
| --- | --- |
| `kb-map-decision-001` | **23 new rows** — ADR-0024 in its reserved position, ADR-0038 … ADR-0059 appended. Three amendment edges to draw (0038 → 0001/0035, 0043 → 0015) and **no supersession edges at all** |
| `kb-map-domain-001` | Sections touched: contract ports (0043, 0048, 0054, 0055, 0056), the typed layer (0046, 0047, 0049, 0050, 0059), adapters (0024, 0052, 0053, 0058), the conformance suite (0042, 0051), the gate (0038, 0045, 0057), and publication (0039, 0040, 0041, 0044) |
| `kb-map-open-questions-index-001` | **39 bullets added**, **4 flipped to resolved** (poll-count, postgres-arm-c, event-metadata-floor, query-plan-chunking), **1 flipped to superseded** with its successor named (no-ps-rule-name), and **10 annotated** in the index's running-commentary style |
