# Two residuals from the suite-self-reports lane: sole-evidence pins, and citations into files the lane moved

**Lane:** suite self-reports (`L3-04`, plus a cross-lane consequence of `L1-4`,
`L2-04` and `M-2`). **Written by the lane that implemented the changes it
describes, in the same session, and without the author → two-critic → revision
pass the original thirteen briefs had.** Nobody independent argued the other
side. Read it with that discount applied.

**Confidence:** high on part 2 (it is arithmetic); medium on part 1.

**Semver:** none. Part 1 is a `const` table private to a test target; part 2 is
four line numbers in two documents.

---

## Part 1 — does every singly-covered rule owe an `expect` pin, or only the shotgun mutant's?

### The question

`L3-04` found the caution over `REGISTRY` closing with *"no rule in the table is
covered by that mutant alone"* — false, because
`untagged_events_match_query_all` appears in exactly one `fails` list in the
whole event-store registry and it is `InnerJoinTagStore`'s. That sentence is what
a reviewer uses to stop checking, and it was aimed at precisely the audit it made
unnecessary.

The implemented fix is narrow: the paragraph now says what holds, the row carries
an `expect` pin naming the assertion, and
`the_shotgun_mutants_sole_coverage_is_pinned` requires a pin wherever **that**
mutant is a rule's only evidence. The shotgun is derived from the table as the
row with the strictly broadest `fails` list and then checked against the store
the paragraph names, so the two cannot come to be about different stores.

**The question this leaves open: should the requirement be general?** *Any* rule
whose only registered evidence is one mutant carries the same hazard — the day it
gains a second assertion, its evidence may become an anchor failure and CF-1's
obligation is discharged in name only.

### What is true today

Measured against `REGISTRY` at the time of writing: **16 rules** have exactly one
declaring mutant whose own `fails` list is longer than one, and by a crude parse
most of them carry no pin. The measurement is crude — a regex over the source,
which mis-reads at least one row that the file's prose says *is* pinned — so the
number is an order of magnitude rather than a count, and the general option below
should be costed by a Rust-side measurement rather than by this figure.

Names the crude parse produced, for whoever costs it: `LosingFixture` (two
rules), `DropsMetadataStore`, `ReadTimeClockStore`, `SingleGuardFastPathStore`,
`PreCommitPositionStore`, `SharedBatchPositionStore`, `TagsAreOrStore` (two),
`ExactTagMatchReadStore`, `UninternedTypeStore`, `NullHeadPagingStore`,
`PayloadDedupStore`, `Latin1IdentifierStore`, `InnerJoinTagStore`.

### Option A — leave it narrow (implemented)

The check follows the paragraph. The paragraph is about one store, so the check
is about one store.

- **Costs:** nothing today. The hazard remains live for fifteen other rules, and
  nothing says so anywhere but this brief.
- **The argument for it:** the finding was about a *sentence*, and a check that
  ranges wider than the sentence it makes falsifiable is a different change
  wearing this one's clothes.

### Option B — every singly-covered rule owes a pin

- **Costs a maintainer:** roughly fifteen pins, each of which must match the
  panic the rule actually raises (`mutants_fail_exactly_their_declared_rules`
  checks the substring against the real message, so a pin cannot be invented).
  Writing them is mechanical; getting them wrong is loud.
- **Costs an adapter author:** nothing. `REGISTRY` is private to a test target.
- **The argument against:** a pin is a claim about *which* assertion fires, and
  for a rule with one assertion it is redundant today and becomes load-bearing
  only if a second is added. Fifteen redundant claims is fifteen things to
  update, and the failure mode of a stale pin is a red build with a confusing
  message — which is the class of defect `L1-4` was about.

### Recommendation and the strongest argument against it

**B, but not in this lane.** The hazard is real and general; the narrow check is
the sentence's fix rather than the hazard's. Costing it needs a Rust-side census
rather than the regex above, and the natural home is the same pass that decides
whether `expect` should be *required* rather than optional.

**The strongest argument against B:** requiring a pin wherever coverage is
singular creates pressure to add a second mutant instead of a pin — which is the
better outcome and would satisfy the check by accident, but also the outcome
nobody chose. If the property that actually matters is *two independent
evidences per rule*, that is a different and larger proposition, and B would be
a proxy for it rather than a statement of it.

---

## Part 2 — four citations into files this lane moved

`spec/SPECIFICATION.md` and `spec/E2E-CASES.md` belong to another lane this wave,
so these are stated rather than applied. **`cargo xtask spec-trace` — and
therefore `cargo xtask lints` and `cargo xtask ci` — is red on
`lane/suite-self-reports` because of the first two, and cannot be made green
inside it.**

The two the checker flags:

| Citation site | Cites | Anchor | Should be |
|---|---|---|---|
| `spec/SPECIFICATION.md:1766` | `crates/happenstance-testkit/src/suite.rs:1512` | `read_limit_zero_yields_nothing` | `suite.rs:1534` |
| `spec/SPECIFICATION.md:8656` | `crates/happenstance-testkit/src/concurrency.rs:1015` | `connect_many` | `concurrency.rs:1083` |

Both moved because the lane changed those files: `suite.rs` gained
`query_matching_nothing_yields_empty`'s non-vacuity anchor and its rewritten
message (`L1-4`), and `concurrency.rs` gained the `Sightings` pair (`L2-04`) and
the emitter table (`M-2`).

Two more the checker does **not** flag, and one of them is worse for it:

| Citation site | Cites | Anchor | Note |
|---|---|---|---|
| `spec/E2E-CASES.md:71` | `suite.rs:1106-1162` | `positions_are_strictly_monotonic` | **Already wrong before this lane.** That rule is at `suite.rs:1929` and was at `:1907` at the wave's base; `:1106` was and is a different rule's doc comment. The range should be `1929-…`, and somebody should check what the range's second number was ever meant to bound. |
| `.kb/open-questions/disjoint-boundaries-have-no-clause.md:35` | `concurrency.rs:436` | `k_disjoint_boundaries_admit_exactly_k_commits` | **Also already wrong before this lane** — the rule was at `:468` at the base and is at `:477` now. This lane may not edit `.kb/open-questions/`. |

### The general point, which is not new

`citation-anchor-slack.md` is the brief that owns this. This lane is one more
data point for it, and a sharper one than most: **the `M-2` prose was written to
a line budget** so that `spec/SPECIFICATION.md:8656` would stay inside
`spec-trace`'s twelve-line tolerance, and it did — the citation was green and
pointing at the wrong line for one commit, which is exactly the failure mode the
tolerance creates. The later `L2-04` change pushed it past the tolerance and the
checker finally spoke.

Writing prose to a line budget to keep another file's citation green is not a
practice this repository should adopt, and it is recorded here so that nobody
mistakes it for one.

**What this does not settle:** whether the tolerance should exist at all, whether
citations should be anchor-only, and who owns a citation into a file a different
lane is changing. All three are `citation-anchor-slack.md`'s.
