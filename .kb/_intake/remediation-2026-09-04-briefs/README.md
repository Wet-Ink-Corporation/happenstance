# Decision briefs from the pre-publication remediation

Thirteen decision records, authored 2026-09-03/04 against
`references/evaluation/review-pre-publication-2026-09-03.md`, staged here rather than
written into `.kb/decisions/` because **an accepted decision atom is immutable and
authored by `/redkiln:kb-ingest`, never by hand** (`CLAUDE.md`). These are inputs to
that pass, not atoms.

## What they are, and what they are not

Each brief carries: the question as a question · why it is owed, with `path:line`
evidence · what is true today, quoted from the tree · at least two options with what
each costs a caller, an adapter author, and semver · a recommendation with the
strongest argument *against* it stated in its own words · cost of delay · what it does
not settle.

**No brief writes ADR prose, a status line or frontmatter.** None was implemented
except `adapter-driver-reexport-policy.md`, which the repository owner ratified
explicitly and which is now landed.

## How they were produced, and why that matters when reading them

Author (opus/xhigh) → two independent critics in parallel → revision folding the
criticism back in. The two critics had **different lenses**: one argued the strongest
rejected alternative; the other did nothing but hunt for a premise that a commit, an
accepted ADR, or the crates.io registry had **already falsified**.

That second lens exists because of finding `N-2`: ADR-0029 raised the MSRV on a
"nothing is published" premise that no longer holds, and rejected the per-crate
alternative on a `publish = false` that is also gone. Both were true when written.

**Twelve of the thirteen flipped their own recommendation under critique**, and the
flips ran overwhelmingly *toward cheaper and less irreversible* options. Four separate
briefs found that something the audit framed as "free now, permanent at `0.2.0`" either
had no deadline at all or rested on a premise the tree contradicts. Read the
`## Revision record` at the foot of each: it says what was removed and why, and
revision 2's falsified claims are kept inline with `[Falsified — revision 3]` markers
rather than deleted, so the corrected trail stays readable.

Confidence is stated per brief and is **medium** on several. That is not hedging; it is
the brief refusing to manufacture certainty it does not have. `transient-contention-tolerance.md`
goes further and **declines to recommend**, naming a fork and pre-committing to neither
shape, because the instrument that would decide it does not exist yet.

## The thirteen

| Brief | Question | Semver | Free until |
|---|---|---|---|
| `fixture-declension-policy.md` | May a testkit minor add a **required** item to a fixture trait? | additive | stable `0.2.0` |
| `event-metadata-floor.md` | Does `Event::metadata` get a floor, share `data`'s, or stay unbounded? | additive | stable `0.2.0` |
| `empty-decision-outcome.md` | Does an empty decision commit, refuse, or become a third outcome? | **breaking** | `0.2.0` |
| `tags-scope-agreement.md` | Is `tags ⊇ scope` checked, documented, or left to the author? | breaking (A) / none (B) | `0.2.0` for A |
| `sqlite-blocking-seam.md` | Where does the blocking seam sit, and does `Handle` get an escape hatch? | additive | `0.2.0` for the docs half |
| `append-condition-sql-shape.md` | Intersection chain or aggregate; does most-selective-first survive? | none | any version |
| `adapter-driver-reexport-policy.md` | Do adapters re-export their driver? | additive | **RATIFIED + LANDED** |
| `tuple-boundary-event-type.md` | Do tuple boundaries admit different `Event` types? | none | never expires |
| `domain-event-guard-and-decode.md` | What is `assert_domain_event` for? | mixed | `0.2.0` for B |
| `transient-contention-tolerance.md` | Fixture-declared tolerance, `CONTENDERS` moves, or both? | none | fork left open |
| `after-opt-scope.md` | Does `after_opt` keep blanket scope under a name that says so? | mixed | stable `0.2.0` |
| `append-batch-ownership.md` | Does `EventStore::append` take ownership? | none | `0.2.0` |
| `msrv-premise.md` | Does ADR-0029 survive re-derivation? | additive | stable `0.2.0` |

## Two more, added later and produced differently

| Brief | Question | Semver | Free until |
|---|---|---|---|
| `op-read-non-exhaustive.md` | Does `Op::Read` get variant-level `#[non_exhaustive]`, and must it land with the `to` field? | **breaking** | `0.2.0` |
| `model-family-case-floor.md` | Does `ops_agree_with_the_model` stop inheriting its case count from `PROPTEST_CASES`? | none | before adapters run the model family |

Both were written by the lane implementing `L1-1` and `L2-01` — the second after
that lane's adversarial review found the sweep — and **neither** went through the
author → two-critic → revision pass the section above describes. Each carries its
own strongest objection and answers it, which is the form, but nobody independent
argued the other side. Read them with that discount applied.

## And more from the fixture-declension lane, produced the same way

| Brief | Question | Semver | Free until |
|---|---|---|---|
| `projection-declension-obligations.md` | Which projection capability has anywhere for the honesty obligation to land, now that the trait no longer carries it? | additive + one rule | the first projection adapter |
| `stated-only-defects-and-the-reopen-must.md` | Is a registry *kind* the right home for a defect nothing can check, and who owns CF-17's marker now its text carries a MUST? **Revision 2: no, the kind was withdrawn.** | none | not blocking |
| `read-fault-clause-and-capability.md` | Does `EventStore::read`'s `Err` arm get a clause, now that it has a rule, a capability and a mutant? | none (the rule is landed) | phase 12 |

Written by the lane implementing the ratified Option A of
`fixture-declension-policy.md`, in the same session as the change it describes, and
**without** the author → two-critic → revision pass. Same discount.

## And one from the `to`-composition lane, produced the same way

| Brief | Question | Semver | Free until |
|---|---|---|---|
| `model-only-kind-has-no-members.md` | `Kind::ModelOnlyMutant` has no members now that a rule sees its only store. Withdraw the kind, keep it dormant, or make the emptiness the measurement? | none (private to a test target) | not blocking; review trigger is phase 12 |

Written by the lane that landed `read_to_composes_with_multi_item_query` and
`read_to_composes_with_limit`, in the same session as the change it describes, and
**without** the author → two-critic → revision pass. Same discount. It is worth
reading beside `stated-only-defects-and-the-reopen-must.md`, which answered a
question that looks identical and is not: that kind was withdrawn for an
**unsound bar**, this one is empty with a bar that has been attacked and held.

`op-read-non-exhaustive.md` is here rather than implemented because L2-01 splits
in two and only one half was that lane's: the generator hole is landed, and the
attribute is routed to whoever owns the testkit's public surface at first publish.
The two are coupled — the field addition already spent the break the attribute
would otherwise share — so the brief is about a window that is already closing
rather than one that has not opened.

`model-family-case-floor.md` is here rather than implemented because the number it
would pin is one nobody has measured against a real medium, and pinning it on the
strength of an in-memory mutant set would be the premise-audit failure this
directory exists to avoid. Its evidence — a cliff at ~192 cases against a default
of 256 — was measured in the tree, both before and after the `to` change, and is
**not** a regression of that change.

## And one from the query-ceilings lane, produced the same way

| Brief | Question | Semver | Free until |
|---|---|---|---|
| `query-partition-public-surface.md` | Once the partition is two ceilings, does `MAX_QUERY_ARMS_PER_STATEMENT` stay public, and does `planned_statement_count` count arms or statements? | additive (1A/2A) / breaking (1B, 2C) | `0.2.0` |

Written by the lane implementing `X-1`, in the same session as the change it
describes, and **without** the author -> two-critic -> revision pass. Same
discount. It differs from the others in one way worth knowing before reading:
**one of its two questions was answered by the lane rather than deferred to it.**
`planned_statement_count` was returning `1` for a plan of 51,200 bound parameters
that `prepare` refuses, and leaving that in place was not an option the fix could
carry, so option 2A landed. The brief says so at the top and states the argument
against it.

That lane also ran ahead of `append-condition-sql-shape.md`, which had asked to be
sequenced first. Both halves of `X-1` are implemented; the `Selectivity::read_for`
half is the one that brief's option B would discard, exactly as it predicted, and
a note recording that sits in its own sequencing section.

## And six from the checks-stronger-than-mechanism lane, produced the same way

| Brief | Question | Semver | Free until |
|---|---|---|---|
| `prose-guard-retired-and-what-it-owes.md` | `spec-trace`'s prose guard is deleted. What does `kb-open-question-no-ps-rule-name-resolved-001` owe, and what do the 45 declarations that replaced it say? | none | does not expire |
| `portfolio-check-and-the-port-freeze-bar.md` | CF-25/CF-26 name a portfolio check that does not exist. Built before `ProjectionStore` freezes, or does the freeze name its accepted axis? | none | **the `ProjectionStore` freeze** |
| `which-reading-of-a-case-naming-no-clause.md` | CF-38's fourth condition reads two ways and they differ by 54 cases. Which one? | none | not blocking; A is landed and green |
| `cf-36-thirteen-recorded-breaches.md` | CF-36 has been checked for the first time and thirteen clauses breach it. Which of the three repairs does each take? | none | first publish; two of the thirteen sooner |
| `a-gate-step-whose-first-check-hides-its-second.md` | Three unrelated checks share one gate step and short-circuit on the first `?`. Own steps, or does the step collect? | none | not blocking; it has bitten three times in one day |
| `trait-variant-caret-and-the-consumer-resolve.md` | Does `trait-variant`'s caret narrow, and does CI learn to resolve freshly? | none to the fix | whenever `0.1.4` publishes |
| `wf-10-instruments-narrower-than-the-clause.md` | WF-10 says *every value type*; the instruments cover a subset in two directions. Which instrument does each get? | none | not `0.2.0` |

Seven, not six — `wf-10-instruments-narrower-than-the-clause.md` and
`trait-variant-caret-and-the-consumer-resolve.md` are the two whose remediations
lie entirely outside that lane's writable surface (`crates/` and
`.github/workflows/`), so they are records with measurements rather than
proposals the lane could have taken.

Written by the lane implementing `S-5`, `Q-03`/`Q-04` and `M-4`, in the same
session as the changes they describe, and **without** the author → two-critic →
revision pass the first section describes. Same discount. Three of them —
`prose-guard-retired`, `cf-36-thirteen-recorded-breaches` and
`which-reading-of-a-case-naming-no-clause` — describe a change that has **landed**
and ask for a decision about the record it left, rather than proposing one.

## Three that block other work

- **`fixture-declension-policy.md`** is the keystone for the whole testkit wave: it
  decides whether `READ_FAULT`, `MAX_METADATA_LEN` and the contention tolerance land
  defaulted or required. Nothing in `L1-2`, `L3-01`, `C2-06` or `AE-4`'s rule should
  start before it is settled.
- **`append-condition-sql-shape.md`** blocks `X-1`. `X-1`'s public seam
  `planned_statement_count` *describes* whatever plan shape this record picks, so
  partitioning against a chain that is then abandoned publishes a description of an
  abandoned plan — `X-1`'s own defect, freshly committed.
- **`append-batch-ownership.md`** cannot be settled at all without a measurement nobody
  has scheduled, and it says so. It also corrects the subject: the two-build measurement
  belongs against **`happenstance-cloudflare`**, not SQLite, because `write_rows` copies
  into owned `SqlValue`s — the worker layer marshals into JS and cannot bind a borrow.
  ADR-0012 named the wrong crate for its own falsifier.

## One correction to a claim made earlier in the session

An earlier report to the repository owner said `fixture-declension-policy.md` and
`transient-contention-tolerance.md` **contradict** each other on whether "defaulted"
implies "recoverable". They do not. `C2-04`'s case rests on *adding* a defaulted const
being additive; `§11`'s correction is that *removing* one is breaking, and it applies
that against its own Option 3. Together they read: **default freely, but only add what
you are prepared to keep.** The contradiction was overstated and is withdrawn.

## And two from the frozen-clause lane, produced the same way

| Brief | Question | Semver | Free until |
|---|---|---|---|
| `unswept-rows-and-byte-identity.md` | ES-18 says *byte-identical* and no counter-assigning store can be. Does the second sentence get amended, or does the conformance reading become the clause? | none | phase 12 |
| `frozen-doc-musts-and-the-adapter-half.md` | ES-23 has two MUSTs on two owners and `FROZEN_DOC_MUSTS` records one. Third disposition, `Excluded`, or a paragraph? | none (`xtask` is `publish = false`) | phase 12 |

Written by the lane implementing `Q-01` and `Q-02`, in the same session as the
changes they describe, and **without** the author → two-critic → revision pass the
first section describes. Same discount.

Both are residuals rather than blockers: `Q-01`'s adapter fix and `Q-02`'s
statement are landed and checked, and each brief is about the *instrument* the
finding got past rather than about the finding. The second recommends an `xtask`
change the lane was forbidden to make, so it is routed rather than implemented.

## And one from the Cloudflare error-and-manifest lane, produced the same way

| Brief | Question | Semver | Free until |
|---|---|---|---|
| `stringified-throw-visibility.md` | `StringifiedThrow`'s field is sealed. Should the type be `pub` at all, when both its in-crate roles are satisfied by `pub(crate)`? | breaking (B, C) / none (A) | the release that first ships `happenstance-cloudflare`, deferred past `0.2.0` |

Written by the lane implementing `G-2` and `N-1`, in the same session as the
changes it describes, and **without** the author → two-critic → revision pass.
Same discount.

Two things to know before reading it. **It records a decision the lane took
rather than deferred**, and says why: the audit routed the choice between
RS-13-1 (private field) and RS-13-3 (`#[non_exhaustive]` with a public field) to
a decision record, and RS-13-3 does not close the hazard `G-2` is about —
an attribute blocks the struct literal and does nothing to an assignment on a
value already held. Where one of two written rules does not do the job, there is
no fork. And **it declines to recommend** on the question it *is* about, for a
different reason from `transient-contention-tolerance.md`: not that the deciding
instrument is missing, but that the deciding evidence is a fact about consumers
of a crate that has none yet.

`N-1` produced no brief. It was a manifest comment contradicting `deny.toml` at
the same commit; `cargo deny check bans` reports `bans ok` and the comment said
red, so the tool settled it and there was nothing to decide.
