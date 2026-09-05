# `MODEL_COVERAGE` is only true above about 192 generated cases. Should `ops_agree_with_the_model` stop inheriting its case count from the environment?

Short answer up front: **yes — take the case count as `max(environment, floor)` so
`PROPTEST_CASES` can only ever buy more coverage, never less.** The table this
repository publishes as a fact about the model family is a fact only at
`PROPTEST_CASES >= 192`; the default is 256, so the margin between "true" and
"silently false" is **1.33×**, and the knob that closes it is one environment
variable the rule's own documentation advertises.

This is **not a regression introduced by the `to` generator change**. The cliff
sits in the same place before and after it — measured below, both columns
reproduced in this working tree. What the `to` change did was swap *which* store
stands on the cliff edge, and it moved one store's margin by 3× in the wrong
direction. That is the second thing this brief is here to put on the record,
because the lane that made the change did not measure it and did not report it.

---

## Why this is owed

**Nobody has taken this measurement, and two things in the tree assert its
conclusion without it.**

`crates/happenstance-testkit/tests/mutation_coverage.rs` carries `MODEL_COVERAGE`,
a table of 88 rows claiming, per store, whether the model family rejects it. Its
own documentation says why it lists every store rather than only the caught ones:

> A table of the stores it catches would answer that and nothing else. A table of
> **every** store answers the question that actually matters — *what is this test
> blind to* — and it answers it in a form that goes red when the answer changes.

That is the right design and it is what caught the `LimitPerItemStore` regression
during the `to` change. But "goes red when the answer changes" is only true at a
case count nobody has pinned. Below ~192 the table goes red because the *budget*
changed, and the message it prints — *"the model got weaker and the table is the
only thing that noticed"* — names the wrong cause.

**The rule advertises the knob.** `crates/happenstance-testkit/src/model.rs`, in
`ops_agree_with_the_model`'s own rustdoc:

> Coverage is bought with `PROPTEST_CASES` — the case count is
> `Config::default()`'s, which reads that variable — rather than with a different
> sample each run.

and, three paragraphs above it, the reason the RNG is fixed at all:

> A conformance rule that fails one run in twenty is worse than no rule: an
> adapter author reruns CI until it is green, and the suite has taught them to.
> The seed is fixed, so a store either passes this rule or it does not, and a
> failure reproduces on the reviewer's machine.

Both sentences are true and they do not compose. The seed is fixed so the verdict
is reproducible; the *number of draws from that seed* is not fixed, so the verdict
is reproducible only per-environment. An adapter author who sets
`PROPTEST_CASES=100` in CI to keep a slow store's suite under a time limit gets
exactly the failure mode the first sentence forbids — except worse, because it
reproduces, and it looks like a defect in their store.

**And this ships.** `happenstance-testkit` is on the registry at `0.2.0-alpha.1`,
and `ops_agree_with_the_model` is one of the rules an adapter runs. The knob is in
the adapter author's environment, not ours.

---

## What is true today

### 1. The cliff, measured in this working tree

`cargo test -p happenstance-testkit --all-features --test mutation_coverage the_model_rule`,
with `PROPTEST_CASES` set. Both columns were run here rather than transcribed.

| `PROPTEST_CASES` | at `11f6f47` (before the `to` change) | at `a835e7c` (tip) |
|---|---|---|
| 64 | — | **FAILED** — `ForwardPagingBudgetStore` |
| 128 | — | **FAILED** — `PayloadDedupStore` |
| 160 | **FAILED** — `LimitPerItemStore` | **FAILED** — `PayloadDedupStore` |
| 176 | **FAILED** — `LimitPerItemStore` | **FAILED** — `PayloadDedupStore` |
| 192 | ok | ok |
| 256 (default) | — | ok |
| 512 | — | ok |

The failure is always the same shape: `MODEL_COVERAGE` claims `Rejected` and the
model answers `Agreed`, because the sequence that would have caught the store was
never drawn.

**The cliff is between 176 and 192 in both columns.** The `to` change did not move
it. It changed which store is standing on it.

### 2. The per-store depth table

Measured by the adversarial review of this lane, at the tip, and **not
re-derived here** — this brief reproduced only the two depths its own sweep
implies (`ForwardPagingBudgetStore` > 64 and ≤ 128; `PayloadDedupStore` > 176 and
≤ 192), which agree with it. The rest is the review's, and is labelled as such.

| Store | cases to first rejection, at tip | before the `to` change |
|---|---|---|
| `ToIsExclusiveStore` | 8 | *(not rejected at any count — the model could not reach `to`)* |
| `ToBoundIgnoredStore` | 64 | *(as above)* |
| `BackwardsToIsAnUpperBoundStore` | 64 | *(as above)* |
| `UnparenthesisedToPredicateStore` | **128** | *(did not exist)* |
| `LimitPerItemStore` | 128 | 192 — **improved** |
| `ForwardPagingBudgetStore` | 128 | 32 — **worse** |
| `PayloadDedupStore` | **192** | 64 — **worse, by 3×** |

512 cases produce the same 43 rejected rows as 256, so the table saturates at the
default and the extra budget buys nothing. That is the review's measurement.

### 3. Two honest consequences of the `to` weighting, neither previously reported

`crates/happenstance-testkit/src/model.rs`'s `any_to_anchor` emits `Anchor::Unset`
four times in five. That skew was itself a measured decision — sampling `to`
uniformly lost `LimitPerItemStore` outright — and it is recorded in the function's
rustdoc. What is **not** recorded, and belongs here:

- **It bought `to` coverage at a real cost elsewhere.** `PayloadDedupStore`'s
  margin fell from 64 cases to 192 — from a 4× margin under the default to
  1.33×. The lane that made the change reported the `LimitPerItemStore` near-miss
  and did not report this one, because `MODEL_COVERAGE` is a pass/fail table at
  one case count and cannot see a margin.
- **`to` coverage is materially shallower than the other four options'.** The
  model-only mutant, `UnparenthesisedToPredicateStore`, needs 128 of the 256
  available cases — half the budget — where `from`, `backwards` and `limit`
  defects are typically caught inside the first few dozen. That is the arithmetic
  cost of one option in five being sampled at one fifth the rate, and it means the
  newest read option is also the least redundantly covered.

### 4. The verdict table is not the only consumer

`the_model_rule_rejects_exactly_what_it_claims` drives the same rule, so this
repository's own meta-test inherits the same floor. A contributor who exports
`PROPTEST_CASES` for an unrelated reason gets a red gate naming a store they did
not touch.

---

## Options

### Option A — do nothing; document the floor in the rule's rustdoc

- **Costs a caller:** the same red build, arriving with a paragraph explaining it.
  Documentation does not run.
- **Costs an adapter author:** nothing new. They still meet it.
- **Semver:** none.

### Option B — pin `cases` in the rule; ignore the environment entirely

`TestRunner::new_with_rng(Config { cases: MODEL_CASES, ..Config::default() }, …)`.

- **Costs a caller:** the documented coverage knob stops working, in both
  directions. Someone who wants 10,000 cases to hunt a suspected defect cannot
  ask for them without editing the testkit.
- **Costs an adapter author:** a fixed, unavoidable runtime. For a store backed by
  a real database rather than a `Vec` behind an `Rc`, 256 generated sequences is
  not free and there is now no lever.
- **Semver:** none in the type system; a behaviour change to a published rule.

### Option C — floor it: `cases = max(Config::default().cases, MODEL_CASES)`

The environment can raise the count and cannot lower it.

- **Costs a caller:** `PROPTEST_CASES` below the floor is silently ignored, which
  is a documented lie unless the rustdoc says so — and it must.
- **Costs an adapter author:** the floor's runtime, unconditionally; the same cost
  as B, without B's loss of the upward lever.
- **Semver:** as B.

### Option D — assert the floor: fail the rule with a message naming the number

- **Costs a caller:** an explicit, well-worded failure instead of a mystifying one
  — but it is still a failure, so an adapter author who lowered the knob has a red
  suite either way. It converts a wrong diagnosis into a right one and fixes
  nothing else.
- **Costs an adapter author:** as A.
- **Semver:** none.

### Option E — skip with a declared reason below the floor

Return `RuleOutcome::Skipped` so a lowered budget reports honestly rather than
certifying.

- **Costs a caller:** nothing, and it is the most *honest* shape: it neither
  overrides the author's explicit instruction nor pretends the rule ran.
- **Costs an adapter author:** nothing.
- **Semver:** none.
- **But it breaks an existing invariant, and that is decisive.** A skip in this
  suite must carry a `(Capability, reason)` pair the **fixture** declined —
  `mutants_fail_exactly_their_declared_rules` asserts exactly that, and its
  documentation says why: *"a skip arriving from anywhere else means a rule
  stopped running and nothing noticed."* A skip caused by an environment variable
  has no fixture behind it and no capability to name. Option E would either need a
  new skip category with its own bookkeeping, or it would quietly become the first
  unaccounted skip in the binary — which is the thing that invariant exists to
  prevent.

---

## Recommendation

**Option C, with the floor at 256 — the current default — and the rustdoc rewritten
to say that the variable raises and does not lower.**

The reasoning is that the two sentences already in the rule's documentation only
compose under C. The seed is pinned so that a verdict is reproducible; pinning the
seed and leaving the *draw count* free reproduces a verdict per-environment, which
is the same defect one level up. C is the smallest change that makes
"a store either passes this rule or it does not" true as written, and it keeps the
only direction of the knob anyone actually needs — buying more coverage when
hunting something.

256 rather than 192 because 192 is the cliff and a floor at the cliff has no
margin; and rather than 512 because 512 rejects the same 43 rows, so the extra
budget is measured to buy nothing and would double an adapter's runtime for it.

**The strongest objection, in its own words.** *"A floor is a cost imposed on
exactly the person least able to refuse it. Every mutant in this repository is a
`Vec` behind an `Rc` and 256 sequences cost 1.7 seconds; against a real SQLite
file, a Durable Object, or Neon over one-shot HTTP, 256 sequences of up to `MAX_OPS`
appends and reads is not 1.7 seconds, and `PROPTEST_CASES` is the one lever that
adapter author has. Taking it away to protect a table that lives in **our** test
binary is making a stranger pay for our bookkeeping. Option D gives them a correct
diagnosis and leaves the lever in their hand, which is what a library should do."*

That objection is right about who pays and wrong about what is being protected. The
table is a symptom; the thing that degrades below the floor is the **rule's own
verdict**, which is the adapter author's result, not ours. Under D, an author who
lowers the knob gets a red suite and a message telling them to raise it — so the
lever they kept is one they cannot use, and D is C with a worse ergonomic and an
extra failure path. If the true finding is that 256 sequences is too expensive for
a real adapter, then the honest response is a *lower floor justified by
measurement against a real adapter* — which nothing in this workspace has taken,
because no adapter has run this family against a real medium yet. That measurement
is the thing that should move the number, and C is the shape that has a number to
move.

**What would flip this.** A measurement showing `ops_agree_with_the_model` at 256
cases is minutes rather than seconds against `happenstance-sqlite`. Then the
question stops being "floor or no floor" and becomes "what is the floor", and D
becomes defensible as an interim while that number is found. Nobody has run it;
this brief does not assume.

**One thing C does not fix, and should be said out loud.** A floor makes the table
*true*; it does not make it *robust*. `PayloadDedupStore` at 192 has a 1.33× margin
under a 256 floor, and any future change to the generator — a sixth option, a
re-weighting, a wider `any_event` — can push it over with no warning, exactly as
the `to` change pushed it from 64 to 192 and nobody noticed. The instrument that
would notice is a recorded *depth* per store rather than a pass/fail, and that is
named below as not settled here.

---

## Cost of delay

- **The knob is in the adapter author's hand at `0.2.0`.** `ops_agree_with_the_model`
  is a shipped rule and `PROPTEST_CASES` is a proptest convention many CI
  configurations already set globally, for unrelated crates. The first adapter to
  meet this will read it as a defect in their store.
- **Changing the count later changes verdicts.** Once adapters are running this
  family, moving the case count moves what passes. It is the same class of change
  as adding a rule, which this repository already treats as breaking in practice
  (CF-29), and it is cheapest before the population exists.
- **The margin is being spent without anyone watching.** `PayloadDedupStore` went
  from 4× to 1.33× in a single change whose author was not looking at margins,
  because nothing reports them. The next such change may cross the line, and the
  failure it produces will name a store that is not the cause — which is exactly
  what the 160-case runs above look like.
- **Nothing else blocks on this.** It is a two-line change plus a rustdoc
  paragraph, and it can land any time before publish.

---

## What this does not settle

- **Whether `MODEL_COVERAGE` should record depth rather than pass/fail.** A column
  of "cases to first rejection" would have made `PayloadDedupStore`'s 3× loss
  visible in the diff that caused it. It would also be a number that moves on every
  generator change, so it needs a tolerance, and a tolerance needs an owner. Named
  here; not proposed.
- **Whether `any_to_anchor`'s 4-in-5 skew is the right one.** It was chosen against
  one falsification (`LimitPerItemStore`) and its cost to `PayloadDedupStore` was
  not measured at the time. A different weighting, or a `to` sampled jointly with
  `from` rather than independently, may buy both. Not investigated.
- **What the floor should be against a real medium.** Named above as the one
  measurement that would change the recommendation, and unscheduled.
- **Anything about `PROPTEST_CASES` in the other three families.** Only the model
  family reads it; the enumerated rules do not.

---

## Provenance of this brief

Written by the lane implementing `L1-1` and `L2-01`, after the adversarial review
of that lane surfaced the sweep. Like `op-read-non-exhaustive.md` in this
directory, it did **not** go through the author → two-critic → revision pass the
first thirteen briefs describe: it carries its own strongest objection and answers
it, which is the form, but nobody independent argued the other side. Read it with
that discount applied.

The cliff table in *What is true today* §1 was measured in this working tree, both
columns. The per-store depth table in §2 is the review's and is labelled as such;
two of its seven rows are corroborated by the sweep here and the other five are
not independently checked.
