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

## A fourteenth, added later and produced differently

`op-read-non-exhaustive.md` — *Should `Op::Read` carry variant-level
`#[non_exhaustive]`, and does it have to land in the same release as the `to`
field?* · **breaking** · free until `0.2.0`.

It was written by the lane implementing `L1-1` and `L2-01`, after the thirteen
above, and it did **not** go through the author → two-critic → revision pass the
section above describes: it carries its own strongest objection and answers it,
which is the form, but nobody independent argued the other side. Read it with that
discount applied.

It is here rather than implemented because L2-01 splits in two and only one half
was that lane's: the generator hole is landed, and the attribute is routed to
whoever owns the testkit's public surface at first publish. The two are coupled —
the field addition already spent the break the attribute would otherwise share —
so the brief is about a window that is already closing rather than one that has
not opened.

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
