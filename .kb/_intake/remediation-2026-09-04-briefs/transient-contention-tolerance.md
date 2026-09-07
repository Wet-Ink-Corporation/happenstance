# Does `happenstance-testkit` grow a fixture-declared tolerance for transient contention, does `CONTENDERS` move, or both?

Decision record: **ADR-0022-s11-contention**. Brief only — no ADR prose, no atom, no code.

---

## Why this is owed

**ADR-0022 §11's falsifier is unconditional and it has fired.**
`references/adr/0022-append-condition-strategy.md:615-616`:

> **§11, the pragmas.** … Re-open the busy timeout if any run ever reports
> `busy > 0`, which would mean five seconds stopped being generous.

A run reported `busy > 0`. `experiments/busy-timeout-margin/results/raw/rate-n64-r2.txt`,
the two rows that carry it, verbatim and unabridged of the fields that matter:

```text
rule=exactly_one_of_n_contenders_commits          … race_us_max=5522382  committed=5  rejected=314  busy=1  failed=0  exhausted=0
rule=append_returns_the_callers_own_last_position … race_us_max=5502321  committed=5  rejected=313  busy=2  failed=0  exhausted=0
```

Both rows carry `busy_handler=sqlite-default`, so the `busy` bit is SQLite's own and
not an artefact of the instrument.

**The rate, counted from raw rather than from the summary.** Seven launches in the
corpus reproduce the gate's configuration at the shipped contender count —
`busy_handler=sqlite-default`, `contenders=64`, three racing rules concurrent:
`headroom-n64.txt` and `rate-n64-r1.txt` … `rate-n64-r6.txt`. **One of the seven**
(`rate-n64-r2`) reports `busy > 0`. Each launch is 3 rules × 5 rounds × 64 =
960 attempts, so the incident is **3 attempts in 6,720**, or 0.045%, all inside one
launch. Across the whole `results/raw/` directory there are 44 rows at
`contenders=64` and exactly those two carry a nonzero `busy`.

**Two precision notes on the raw, because the summary page and the audit each
over-read it in opposite directions.**

* The audit's own correction **C-7** is right that
  `results/busy-timeout-margin.md:199-200` — *"three contenders across two rules
  exhausted the full 5,000 ms"* — overstates what the counter says. `exhausted` reads
  `0` on both rows. Do not repeat the summary's sentence.
* But the audit's replacement inference — *"`exhausted=0` on both rows — no attempt
  ran the 5,000 ms budget out"* (`review:1335`) — is also more than the field can
  carry. `exhausted` is incremented **only inside the crate's own counting handler**
  (`experiments/busy-timeout-margin/src/busy.rs:262`), and `HS_HANDLER=default`
  installs SQLite's handler instead (`src/busy.rs:100-104`). On a
  `busy_handler=sqlite-default` row `exhausted` is **structurally zero**, along with
  `wait_ms_max`, `retries` and `lock_events` — all four read `0` on every one of those
  rows. The defensible sentence is the one the falsifier actually names: **`busy > 0`
  at the shipped `CONTENDERS = 64`**. The two rows' `race_us_max` of 5,522,382 µs and
  5,502,321 µs — both just past the 5,000 ms budget — corroborate exhaustion without
  proving it, and nothing in the decision below turns on which it was.

**The margin, verified from the counting-handler rows.** Worst budget wait per
`label`, from `results/raw/matrix-debug-c*.txt`: 1 core → 8 ms, 2 → 628, 4 → 2,628,
8 → **3,828**, 20 → **3,628**. The gate's own configuration (debug, all cores, three
rules concurrent) is `debug-c20` at 3,628 ms of 5,000 — **1.38x**; the worst cell
anywhere is 8 cores at 3,828 ms — **1.31x**. The `monotonic-guard` arm ADR-0022 §4
chose reached 3,728 ms (`matrix-debug-c20-guard.txt`) against the probe arm's 3,628 —
the margin is a property of the herd, not of the guard. Every `wait_ms` figure is a
**lower bound** (W-1, `review:1275`), so 1.31x–1.38x is the optimistic reading.

**What the gate does with that is the defect, and it is not "the suite should be more
forgiving".** The chain is stated in the experiment's own header
(`experiments/busy-timeout-margin/tests/busy_margin.rs:43-49`):

```text
SQLITE_BUSY → AppendError::Store → Attempt::Failed → the rule panics
```

— on a message that tells an adapter author their store is wrong when nothing about
their store is wrong, in the one place **CF-33 `[FROZEN]`**
(`spec/SPECIFICATION.md:8720-8721`) guarantees the suite cannot diagnose it:

> **CF-33.** No conformance rule may read a clock, measure elapsed time, or assert
> on an operation count. `[FROZEN]`

The adapter's busy timeout is therefore *the only liveness bound in the system*
(W-1, `review:1265`), and no rule may add another. A hung or slow store names no rule
by design; a busy-refused store names the wrong one.

**The two owners are already separated in the record, and one of them refuses this
question in terms.** `references/adr/0022-append-condition-strategy.md:618-619`:

> **§12, the contender count.** Not this record's to re-open: it supplies a number
> and `concurrency-family-and-contender-count` decides.

`.bklg/from-contract-to-published-library/sqlite-durable-store/concurrency-family-and-contender-count/`
is that story, and it is the one that set the constant to 64.

---

## What is true today

### The classification: three arms, and the third is a bucket

`crates/happenstance-testkit/src/concurrency.rs:246-265`:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
enum Attempt {
    /// The append landed, at this position.
    Committed(SequencePosition),
    /// The store reported `ConditionViolated`, which is the DCB retry signal.
    Rejected,
    /// Anything else: a store error, or an empty batch the rule did not send.
    Failed(String),
}

impl Attempt {
    fn of<E: core::fmt::Display>(result: Result<SequencePosition, AppendError<E>>) -> Self {
        match result {
            Ok(position) => Self::Committed(position),
            Err(AppendError::ConditionViolated(_)) => Self::Rejected,
            Err(other) => Self::Failed(format!("{other}")),
        }
    }
}
```

`Attempt` has no `pub` (`:247`), so a fourth arm is internal and costs nothing in
semver. `AppendError` is `#[non_exhaustive]` (`crates/happenstance-core/src/error.rs:213`)
with four variants and no `Busy`, so `Err(other)` is where a lock timeout, a disk
failure, a capacity refusal and a caller bug all arrive together.

**The store's own documentation already draws the distinction the testkit cannot
receive** (`crates/happenstance-sqlite/src/event_store.rs:1225-1227`):

> * the driver failed — including `SQLITE_BUSY` after the configured busy
>   timeout has genuinely elapsed, which is **contention reported honestly
>   rather than a condition violation**;

### Three rules break on a busy refusal, and only one of them breaks the way the audit describes

The audit reports the classification arm. The tree has three sites, and two of them
fail on a *count* that no re-spelling of `Attempt` repairs.

**1. `exactly_one_of_n_contenders_commits` (`:379`)** — the classification assertion
the audit quotes, at `:405-414`:

```rust
let failures: Vec<&Attempt> = attempts
    .iter()
    .filter(|attempt| matches!(attempt, Attempt::Failed(_)))
    .collect();
assert!(
    failures.is_empty(),
    "a contender must either commit or be told `ConditionViolated`; \
     anything else is a store failure under contention rather than the \
     concurrency signal. Got {failures:?}"
);
```

Its second assertion needs `committed.len() == 1` (`:417-424`) and is unaffected as
long as the winner is not the contender that gets busied.

**2. `positions_are_unique_under_concurrent_appends` (`:598`)** — `:524-533` carries
the same `failures.is_empty()`, and then `:618-623` asserts:

```rust
assert_eq!(
    committed.len(),
    CONTENDERS,
    "an unconditional append has nothing to be rejected by, so every \
     contender must commit. Attempts: {attempts:?}"
);
```

A busied contender fails **this**, not the classification, and it fails it whatever
`Attempt::of` returns.

**3. `append_returns_the_callers_own_last_position` (`:669`)** — `:698-700`:

```rust
let Some(returned) = attempts[index].position() else {
    panic!("contender {index} did not commit an unconditional append: {attempts:?}");
};
```

Same shape: a per-contender `panic!` on any non-commit.

These are exactly the two rules that carried `busy=1` and `busy=2` in `rate-n64-r2`.
**A tolerance is therefore not a third `Attempt` arm. It is a change to what three
rules assert about how many contenders commit** — and for two of them the current
assertion is *all of them*.

### `CONTENDERS`, and its own documentation

`crates/happenstance-testkit/src/concurrency.rs:238`:

```rust
pub const CONTENDERS: usize = 64;
```

**That is the in-tree value, and it is not the published one.** `happenstance-testkit`
`0.2.0-alpha.1` was published on **2026-08-16** (`448e1ac`, `CHANGELOG.md:306`) carrying
`pub const CONTENDERS: usize = 8`. Commit **`995b987`** (2026-08-17, *"The concurrency
family, at the contender count the DoD asks for"*) raised it `8 → 64` **the day after
that publish**, touching no `Cargo.toml`, with no version bump and citing no CF-31 ADR.
So the constant has already moved once *after* publication, in the direction CF-31 would
have had the most to say about, and the repository did not treat CF-31 as a bar. Any
argument in this brief that reasons as though publication is still ahead of the constant
is wrong on its face; those arguments have been removed (see *Revision record*).

`:202-206`:

> **It is not a tuning knob.** The rules assert *set* properties — exactly one
> winner, k boundaries admit exactly k, all positions distinct — and every one
> of them holds at any size above one. Moving this number therefore changes how
> hard the operating system is asked to interleave, and nothing else.

Two facts follow that bear directly on the option:

* **No registered wrong implementation is known to need 64.** The six racers —
  `LockedStore`, `RacingProbeStore`, `GlobalVersionStore`, `RacingSequenceStore`,
  `GlobalHeadStore`, `RowAtATimeStore` (`crates/happenstance-testkit/tests/mutation_coverage.rs:2579-2680`)
  — are pinned to *which* assertion they trip, never to a contender count. So the
  number buys interleaving pressure, and the corpus holds no measurement of
  detection rate against it.
* **The 64 is also a stated proof artefact.** `RUNBOOK.md:159` — phase 8's is *"the
  concurrency macro green at 64 contenders"* — and `RUNBOOK.md:2700-2710` records the
  discrepancy that raising the constant closed, with its own instruction:

  > Whoever reaches phase 8 either raises the constant and says why, or amends
  > both proof artefacts. Leaving it is the third option and it is the one that rots.

* **A measurement at 8 exists, and it is the same measurement as the one at 64.**
  ADR-0022 §12 (`0022:497-503`, `experiments/append-condition/results/contention-64.md:60-66`)
  ran ten races per arm at **both** counts in one round-robin: *"exactly one winner per
  race at both counts, on every arm, with `busy = 0` and `failed = 0`."* Its conditions
  are `--release`, 20 logical cores, the target run alone (`experiments/append-condition/README.md:31-35`)
  — conditions W-1 established the gate does not reproduce — so it is not a measurement
  *of the gate* at 8. It is nevertheless a measurement below 64, taken on the same rig,
  in the same run, as the one that justified the raise. An earlier draft of this brief
  asserted that no measurement below 64 exists anywhere; that was false and has been
  removed.

### `BUSY_TIMEOUT_MS`, and the sentence the measurement caught

`crates/happenstance-sqlite/src/connection.rs:49-62`:

```rust
/// How long a connection waits for a contended write lock before giving up, in
/// milliseconds.
///
/// **Finite and generous**, and both halves are load-bearing. …
///
/// Five seconds was measured to absorb 64-way contention with zero `SQLITE_BUSY`
/// (ADR-0022 §11), which is what makes that zero mean something rather than
/// being the zero an unbounded handler would also have produced.
pub const BUSY_TIMEOUT_MS: u64 = 5_000;
```

The record's conditions for that zero were *"20 logical cores… `--release`"* with the
target run alone (`0022:140-142`); the gate's are `["test", "--locked", "--workspace",
"--all-features", …]` with no `--release` and no `--test-threads` cap
(`xtask/src/main.rs:181-188`). W-1's verdict is that the number is not what is
defective — the epistemics around it are (`review:1283`) — and its remediation is
**deliberately valueless**: *"it proposes no number"*, and *"raising a wall-clock
timeout is a bound on the hardware and not on the store"* (`review:1287`).

### The placement constraint, which is most of this decision

`crates/happenstance-testkit/src/concurrency.rs:186-193`:

```rust
pub trait ConcurrentFixture: Fixture<Store: Send> {}

impl<F> ConcurrentFixture for F
where
    F: Fixture,
    F::Store: Send,
{
}
```

with the property stated at `:180-182`: *"The blanket implementation means no adapter
ever writes it: any fixture whose handle can be moved to another thread already is
one."* The sibling brief measured the consequence on this toolchain: a required item
on `ConcurrentFixture` is `error[E0046]` **on the blanket impl itself**, and an
adapter cannot supply it either (`error[E0119]`)
(`briefs/fixture-declension-policy.md`, "The break, measured on the repository's own
toolchain"). **So a contention tolerance cannot be quarantined on the concurrency
trait. It lands on `Fixture`.**

`Fixture` is published API — `crates/happenstance-testkit/Cargo.toml:21` is
`version = "0.2.0-alpha.1"` and `CHANGELOG.md:306` records it as *"The first
published release"*. Its shape today (`crates/happenstance-testkit/src/contract.rs:131-332`):

```rust
pub trait Fixture {
    type Store: EventStore;                            // :136  required
    const SECOND_HANDLE: Capability;                   // :172  required
    const REOPEN: Capability;                          // :184  required
    const MID_BATCH_FAULT: Capability = …;             // :218  DEFAULTED
    const MAX_EVENT_DATA_LEN: Option<usize> = None;    // :264  DEFAULTED
    const MAX_TAGS_PER_EVENT: Option<usize> = None;    // :273  DEFAULTED
    const MAX_EVENTS_PER_BATCH: Option<usize> = None;  // :290  DEFAULTED
    fn arm_mid_batch_fault(&self, after: usize) -> …   // :308  defaulted (panicking body)
    fn connect(&self) -> impl Future<Output = Self::Store>;  // :332  required
}
```

Every capability added since the two founding consts is defaulted, five for five. The
already-authored sibling brief recommends exactly that as policy — *"every future
capability on either fixture trait lands defaulted, with the honesty obligation
carried by a CF-39-shaped clause-level MUST per capability"*. **Nothing in this
decision argues against it**; a defaulted `Capability` here is that policy's next
instance, and the in-tree blast radius of a required one would be roughly 27 `Fixture`
impls (`grep "impl Fixture for"` across `crates/` and `experiments/`) plus the racer
macro's expansion.

### `AppendError::Busy` in `happenstance-core`: two precedents, and the nearer one was the one dropped

`references/adr/0022-append-condition-strategy.md:446-456`:

> **Rejected: minting `index_arms()` in `happenstance-core`.** The addition would
> be purely additive and `EventStore` being `[FROZEN]` does not forbid it. What
> forbids it is CLAUDE.md's own rule: *a port is only as well-designed as the
> spread of what implements it*, and one implementor is not a spread. …
> **The re-open trigger is named rather than left to memory.** If
> `postgres-and-neon-stores` independently needs the same decomposition, that is
> two unlike storage shapes agreeing, and *that* is the evidence to mint
> `index_arms()` — as its own ADR then, not as a side effect of this one.

**But §10 is the *further* precedent, and this brief previously leaned on it while
naming and then dropping the nearer one.** `index_arms()` is an API convenience on a
query type — a decomposition one adapter wanted for its SQL generation. It is not an
error channel, and no caller has to tell two outcomes apart because of it.

**The nearer precedent is VT-25 / CF-40, and it is the identical harm.**
`crates/happenstance-core/src/error.rs:229-236`, on `ExceedsStoreLimit`:

> Distinct from [`Store`](Self::Store) on purpose, and the distinction is
> the whole reason the variant exists: a caller that cannot tell "this will
> never fit here, park it and tell a human" from "the disk is full, retry"
> has to guess…

That is exactly this question — a refusal arriving on the `Store(E)` channel that a
caller must tell from a real failure — and the repository repaired it **as a pair**:
a core variant (`AppendError::ExceedsStoreLimit`, `StoreLimit`), a *defaulted* fixture
fact (CF-40's three `Option<usize>` consts), one rule
(`append_reports_exceeded_store_limits`) and **four registered mutants in two pairs**
(`spec/SPECIFICATION.md:1628-1656`, `:8093-8139`). It was minted **at phase 4, with
zero adapters in the tree**, and CF-40's own named instruments are still unbuilt shapes
at phases 9 and 10.

**The discriminator this brief previously offered against `Busy` is false and has been
removed.** It read: *"`ExceedsStoreLimit` was minted for a property every store has; a
`Busy` variant today would be minted for one."* Both halves fail. CF-40 defaults all
three ceilings to **`None`** and says in terms that `None` *"means the store has no
ceiling for that limit"* (`:8093-8099`) — most stores have none. And the rule could not
run at all until the testkit built a fixture to make it run: `GappedPositionFixture`
states all three at VT-21 – VT-24's floors precisely because *"a portfolio in which no
conformant variant has a ceiling is a portfolio in which
`append_reports_exceeded_store_limits` never runs"*
(`crates/happenstance-testkit/tests/mutation_coverage/variants.rs:249-261`;
`spec/SPECIFICATION.md:8136-8139`). So the contract crate has already minted an
error-channel variant for a property **not** every store has, with **no** implementor
spread, against named-but-unbuilt shapes. On that precedent `Busy` is not refused; it
is live.

**What the contract offers contention today: nothing of its own.** ES-25 `[FROZEN]`
requires a rejection *"if and only if"* a guard is violated
(`spec/SPECIFICATION.md:3758-3763`), so reporting a transient refusal as
`ConditionViolated` is a defect, and the same clause makes `AppendError::Store(…)`
fully conformant for it. Contention therefore has one channel and it is the channel
that also carries a corrupt file.

**What still cuts the other way.** `happenstance-sqlite` is the workspace's only
event-store adapter that has run the suite; `happenstance-postgres` and
`happenstance-neon` are `todo!()` skeletons (CLAUDE.md, "🔩 skeleton"), so *one
implementor is not a spread* remains literally true, and §10's rejection of
*"adding public surface to the contract crate between the alpha and `0.2.0` for the
benefit of one adapter"* (`0022:446-452`) is a real sentence about a real interval that
this change would also sit in. The axis is named and both instruments are scheduled: a
`SERIALIZABLE` Postgres adapter's `40001 serialization_failure` and a one-shot-HTTP Neon
adapter's transient refusal are the same shape as `SQLITE_BUSY`, and the spec already
lists `40001` as an adapter shape in `GlobalVersionStore`'s provenance
(`mutation_coverage.rs:2626-2634`).

**The honest position is that the two precedents disagree and this brief does not
settle which governs.** That fork is now named in the recommendation rather than
resolved by citing only the precedent that pointed one way.

### What the specification says

| Clause | Where | Maturity | Bearing |
| --- | --- | --- | --- |
| **CF-33** | `spec/SPECIFICATION.md:8720-8721` | **`[FROZEN]`** | No conformance rule may read a clock. This is why the timeout is the only liveness bound and why no rule can diagnose a stall. It forecloses every watchdog-shaped repair. |
| **CF-34** | `:8747-8752` | `[PROVISIONAL]` | Keeps the measurement out of the gate. Its `Rejects:` — *"A threshold nobody can justify becomes a threshold everybody raises"* — is the argument against any tolerance spelled as a *fraction*. |
| **CF-18** | `:8029-8032` | **`[FROZEN]`** | A rule whose capability is unmet MUST still emit as a test reporting the skip. Any tolerance spelled as a `Capability` inherits this reporting obligation. |
| **CF-29** | `:8625-8627` | **`[FROZEN]`** | A rule may be added in a minor, and MUST land with its mutant (CF-1) and a changelog entry naming the defect. |
| **CF-31** | `:8670-8674` | **`[FROZEN]`** | Major means a rule's *meaning* changed; its `Rejects:` is *"tightening an existing rule in place"*. This brief previously read it as making a later re-raise of `CONTENDERS` a major event; that reading is withdrawn — the constant was already raised `8 → 64` after `0.2.0-alpha.1` shipped, unversioned and uncited, so whether CF-31 reaches a contention level is itself unsettled. It still governs the rule-behaviour half of Options 3 and 6, both of which are loosenings. |
| **CF-39** | `:8063-8078` | `[PROVISIONAL]` | The shape of a defaulted capability whose honesty is recovered as a clause-level MUST. The template for a defaulted tolerance. |
| **ES-25** | `:3757-3763` | **`[FROZEN]`** | *"A rejection MUST be reported as `AppendError::ConditionViolated`, never as `AppendError::Store`."* Governs *rejections*. A transient busy refusal is not a rejection, so **a store returning `AppendError::Store(SQLITE_BUSY)` is fully ES-25-conformant today.** |

**No clause governs the classification, and none governs `CONTENDERS`.** `busy` and
`contenders` do not appear in a normative clause anywhere in `spec/SPECIFICATION.md`.
That is the gap the audit names (`review:1295`), and it is real.

### Which atom this would supersede

**For the tolerance: none.** Nothing in `.kb/decisions/` settles `Attempt`'s
classification or the `Fixture` capability set. **ADR-0034**
(`.kb/decisions/0034-the-fixture-contract-has-no-single-owner.md`, `accepted`) says why
there is no standing owner — *"A `CF-` clause is minted by whichever decision first
needs the capability"* — so it is `related`, not superseded.

**For ADR-0022 §11: a superseding atom is owed regardless of what is decided here,
and it is owed even if nothing changes.** `.kb/decisions/0022-append-condition-strategy.md`
is `status: accepted` and immutable. Note carefully what is and is not falsified:

* The atom's body sentence at `:78` — *"a finite `busy_timeout` of 5,000 ms, which
  absorbed 64-way write contention **in the experiment** with zero `SQLITE_BUSY`
  errors"* — is scoped to `experiments/append-condition/` and survives as written.
* What has fired is §16's **unconditional** re-open trigger (`0022:615-616`), and what
  is falsified is the *inference* §11 and `connection.rs:59-61` draw from that zero:
  *"Five seconds was enough… so the timeout did real work and never ran out."* Under
  the gate's own configuration it ran out.

So the §11 re-open is a new atom superseding `kb-decision-0022`, and it lands whether
the answer to *this* question is "tolerance", "`CONTENDERS`", "both" or "neither". Its
content may legitimately be *"the number stays at 5,000 and here is the distribution
that says so"* — a fired falsifier owes a re-reading, not a change.

---

## Options

### Option 1 — Neither. Record the measurement; change no code.

- **Costs a caller:** nothing.
- **Costs an adapter author:** the standing one. One first `cargo test` in seven at
  this contention level, on a host at the plateau, is a red concurrency suite naming a
  defect the author does not have. Out-of-tree they have no ADR-0022 to tell them what
  they are looking at.
- **Semver class:** none.
- **Forecloses:** nothing — but the wrong message is **already published**, in
  `0.2.0-alpha.1` since 2026-08-16, and every author who meets it until a fix ships
  pays it. Note the size honestly: the published suite races **8** contenders, not 64,
  so an out-of-tree author on the registry crate meets the message at a contention level
  where the corpus has no `busy > 0` incident at all. The 3-in-6,720 rate is a fact
  about the in-tree constant.

### Option 2 — Fix the *message*, not the classification.

Rewrite the three sites' panic text so it names both candidates: a semantic violation
**and** a transient, contention-induced refusal by a store whose semantics are
perfect, pointing at the adapter's own busy/lock timeout. Land it with M-1's
`block_on` repair (`review:1379`), which narrows what a hung job can mean, and with a
revision of the instruction at `crates/happenstance-sqlite/tests/concurrency.rs:45-47`
that currently names one of two candidates as though it were the only one.

- **Costs a caller:** nothing.
- **Costs an adapter author:** the build is still red, one launch in seven. What
  changes is that the message no longer sends them to debug correct code.
- **Semver class:** **none.** `Attempt` is private, the messages are not API, and
  `block_on`'s waker discipline changes no signature (M-1's own Semver field,
  `review:1377`).
- **Forecloses:** nothing. It is strictly compatible with Options 3, 4 and 5.

### Option 3 — A fixture-declared tolerance, defaulted on `Fixture`.

A `const TRANSIENT_CONTENTION: Capability` (or equivalent) on `Fixture`, **defaulted
to declined**, plus a `fn is_transient(&self, error: &…) -> bool`-shaped hook with a
defaulted `false` body; a fourth private `Attempt` arm; and the three rules rewritten
so a declared-transient refusal is neither a commit nor a defect. The floors must be
**structural minima, not thresholds** — `exactly_one_of_n_contenders_commits` keeps
`committed.len() == 1` unchanged; `positions_are_unique_under_concurrent_appends`
needs at least two commits for uniqueness to be a claim at all;
`append_returns_the_callers_own_last_position` needs at least one. A *fraction* is
what CF-34's `Rejects:` forbids by name.

- **Costs a caller (the adapter's users):** the concurrency suite stops asserting that
  every unconditional append commits. A store that busies out 62 of 64 callers and
  commits 2 now passes `positions_are_unique_under_concurrent_appends`. That
  liveness claim is given up, and CF-33 forbids replacing it with a timed one.
- **Costs an adapter author:** nothing at compile time — a defaulted const keeps every
  one of the ~27 in-tree `Fixture` impls compiling and every out-of-tree one too. It
  costs one honest line for a store that wants the tolerance, and it inherits CF-18's
  reporting obligation: the declension is printed on every run.
- **Semver class:** **additive to add, breaking to remove.** `Attempt` is private
  (`concurrency.rs:247`); a defaulted associated const on a published trait is additive
  at `0.x` and after `1.0`. The rule-behaviour change is a *loosening*, which CF-31's
  `Rejects:` does not cover (it names tightening) and which cannot make a
  previously-passing adapter fail. CF-29 still applies: it must land with its mutant and
  its changelog entry. **But the surface is one-way in the other direction:** an
  out-of-tree fixture that writes `const TRANSIENT_CONTENTION = …` stops compiling the
  day the const is withdrawn, so *cheap to buy* is not *cheap to unbuy*. An earlier
  draft of this brief called the surface *"defaulted and therefore recoverable"*; that
  is false and has been removed.
- **Forecloses:** it does not foreclose `AppendError::Busy` later — the audit's
  remediation is right that the testkit-side shape *"keeps it clear of all six binding
  constraints"* (`review:1341`). What it does foreclose is the simple form of the
  liveness claim above; recovering it later would be tightening a rule in place, which
  CF-31 `[FROZEN]` rejects. And it is not *free* of Option 6 either: a fixture-level
  capability lets an adapter switch the liveness assertion off wholesale, for every
  error it can produce, where a per-error channel switches it off only for the errors
  the store actually labels transient.
- **What does not exist yet:** the instrument. No registered racer produces
  `Attempt::Failed` — check the six rows at
  `mutation_coverage.rs:2579-2680` — so the assertion the tolerance would soften has
  **no named wrong implementation today**, and the floors the tolerance introduces
  would be new assertions needing their own. The audit names the missing instrument:
  *"a decorator over `MemoryEventStore` whose `append` refuses the first m callers
  with a transient store error"* (`review:1341`). CLAUDE.md's bar — *"Before adding
  one, name a plausible wrong implementation it rejects, and write that
  implementation into the testkit's own `tests/`"* — applies to the floors.

### Option 4 — `CONTENDERS` moves down.

**Re-priced.** This option was previously carried as near-one-way, unmeasured and
CF-31-constrained. Three of those four supports have been withdrawn as resting on
falsified premises (below); what survives is W-1's core-axis refutation, which says a
lowering does not close *this* gap — a reason not to buy it **for this purpose**, not a
bar against buying it for wall time.

- **Costs a caller:** wall time returns — ADR-0022 §12 measured 8-vs-64 at 10.9x–20.0x
  per race (`0022:497-503`), so a lowering is a real CI saving.
- **Costs an adapter author:** less interleaving pressure per run. Unquantifiable in
  this corpus: no registered racer's detection is pinned to a count, and no
  measurement of detection rate against `n` exists.
- **Semver class:** the constant's *value*, not its signature — no API break. **And a
  lowering to 8 is a revert, not a move:** `0.2.0-alpha.1` on the registry carries
  `CONTENDERS = 8`, so lowering restores the published value and no consumer of the
  published crate sees any change at all. An earlier draft called a lowering *"close to
  one-way"* because CF-31 `[FROZEN]` would reject the re-raise as tightening in place;
  that argument has been **removed**, on two grounds. It assumed 64 was the published
  value, which it is not; and the repository has already re-raised this constant
  post-publication once (`995b987`, 2026-08-17) with no version bump and no CF-31
  citation, so CF-31 has not in practice been read as governing it. Whether it *should*
  be is a live question and belongs to `concurrency-family-and-contender-count`.
- **Forecloses:** the raise, if CF-31 is read as governing the constant — which is the
  question above and not a settled bar. And it re-opens the discrepancy `RUNBOOK.md:2700-2710`
  deliberately closed — phase 8's and phase 10's proof artefacts and the status table
  all read *64 contenders* (`RUNBOOK.md:159`), so a lowering must amend all three, in
  the same change, or recreate the rot the raise cured.
- **What the corpus does and does not support.** The `busy-timeout-margin` corpus is
  one-sided: every `contenders=` field in its `results/raw/` reads 64 (44 rows), 96, 128
  or 160, and the headroom sweep runs *upward* only. But ADR-0022 §12 **did** measure 8,
  in the same round-robin as 64, with *"exactly one winner per race at both counts… with
  `busy = 0` and `failed = 0`"* (`0022:497-503`) — under `--release` on 20 cores with the
  target run alone, which W-1 established the gate does not reproduce (`review:1273`).
  So the honest statement is *no measurement below 64 under the gate's own
  configuration*, in either direction — the same gap the 64 itself has. An earlier draft
  said *"there is no measurement below 64 anywhere"* and called a lowering *"the exact
  move §12 refused"*; both are **removed**. The first is contradicted by §12's own
  table. The second inverts §12, which refused to **apply** a number it had measured —
  *"This record supplies the number and does not apply it: `concurrency::CONTENDERS` is
  still 8"* (`0022:490-494`) — rather than refusing to supply one.
- **And it does not close the gap.** W-1's core-axis refutation (`review:1279`) shows
  the pathology is *contenders simultaneously runnable and retrying in lockstep* — a
  property of the host, not of `n` alone. A busy refusal remains reachable at any
  `n > 1` on a loaded runner. Lowering `CONTENDERS` moves a rate; it cannot make
  a busy store and a broken store distinguishable.

### Option 5 — Both: the tolerance lands *and* `CONTENDERS` moves.

- Inherits every cost of 3 and 4, and adds one of its own: **the two changes are not
  independently attributable.** With both landed, no later run can say whether the
  suite is green because the store is honest or because the pressure was reduced.
  That is the same confound the busy-timeout and lost-wakeup probes were deliberately
  run in one directory to avoid (`review:1375`: *"measuring one of the two would let
  either finding be answered with 'that was probably the other one'"*).
- **Semver class:** as Option 3. Option 4's half is a constant's value and, at 8, a
  revert to the published one; the *"one-way constant"* this line previously carried has
  been removed with the CF-31 argument it rested on.

### Option 6 — `AppendError::Busy` in `happenstance-core`. **Live, not refused.**

This option was previously headed *"refused on precedent"*. That refusal has been
withdrawn: it rested on §10's `index_arms()` — an API convenience on a query type — while
the nearer precedent, VT-25 / CF-40, is an error-channel repair of the *identical* harm
and was minted at phase 4 with zero adapters, on named-but-unbuilt shapes, as a pair of
core variant + defaulted fixture fact + one rule + four mutants. The discriminator that
kept the two apart in the earlier draft was false and is gone (see *two precedents*,
above).

`AppendError` is `#[non_exhaustive]` (`error.rs:213`), so the addition is purely additive
and `EventStore` being `[FROZEN]` does not forbid it. Weighed against Option 3 on the
merits rather than on precedent, it has three properties Option 3 does not: it adds **no
published `Fixture` surface**, so nothing has to be un-added later; it helps the
out-of-tree author who has declared nothing and does not know a capability exists,
because the store's own error tells the suite what the refusal was; and it is
**per-error**, so an adapter cannot switch the liveness assertion off wholesale for
every error it can produce.

What still argues against it is unchanged and real: `happenstance-sqlite` is the only
event-store adapter that has run the suite, *one implementor is not a spread*, and
§10 objects in terms to adding contract-crate surface between the alpha and `0.2.0` for
one adapter's benefit. Its named re-open trigger — two unlike storage shapes agreeing —
would be supplied by Postgres's `40001` and Neon's one-shot HTTP.

**This brief does not decide between Option 3 and Option 6.** The fork is named in the
recommendation and left open.

### Option 7, refused on the record's own reasoning — raise `BUSY_TIMEOUT_MS`.

Free in semver — W-1's Semver field is **none** (`review:1285`); the constant joins
crates.io at 0.2.0. And W-1's remediation says why it is the wrong instrument anyway:
*"raising a wall-clock timeout is a bound on the hardware and not on the store"*
(`review:1287`). The `connection.rs:59-61` doc comment does need correcting either
way — the sentence it asserts is the one the measurement caught — but that is a
consequence of the §11 superseding atom, not a repair for this question.

---

## Recommendation

**Option 2 now, unconditionally. Record *neither* Option 3 nor Option 6: name the fork
between them and pre-commit to neither shape. `CONTENDERS` is re-opened and is not
settled here.**

Read as an answer to the question as posed: **neither the tolerance nor `CONTENDERS`
lands in this record. The message repair lands; the channel question is named and
handed on.**

**This recommendation flipped, and what flipped it.** The earlier version read *"Option
2 now, unconditionally. Option 3 in shape, gated on its instrument. `CONTENDERS` does
not move."* Two things moved it. First, the refusal of Option 6 was built on §10's
`index_arms()` while the nearer precedent — VT-25 / CF-40, the same harm on the same
channel, minted at phase 4 with zero adapters — was named and dropped, and the
discriminator that kept them apart (*"minted for a property every store has"*) is false:
CF-40 defaults all three ceilings to `None` and the testkit had to build
`GappedPositionFixture` for the rule to run anywhere. With that precedent restored,
Option 6 is at least Option 3's equal on the merits and beats it on three axes — no
published `Fixture` surface, help for the author who declares nothing, and a per-error
rather than per-fixture switch — so pre-committing to Option 3's shape was picking the
weaker arm of a fork this brief had not actually argued. Second, *"defaulted and
therefore recoverable"* was false: adding a defaulted const is additive, **removing one
is breaking**, so Option 3's surface is cheap to buy and not cheap to unbuy. That
removes the asymmetry the old recommendation used to justify buying it early.

**Why `CONTENDERS` is re-opened rather than held.** The earlier version held it on four
supports and three have been withdrawn as falsified: there *is* a measurement below 64
(§12's own table, at 8, `busy = 0` and `failed = 0`); a lowering to 8 is a **revert to
the published value**, not a one-way move, because `0.2.0-alpha.1` carries 8 and the
raise to 64 happened *after* publication with no version bump and no CF-31 citation; and
§12 refused to **apply** a number it had measured, not to supply one, so "supplying a
number today is the move §12 refused" inverts the record. What survives is W-1's
core-axis refutation: the pathology is contenders simultaneously runnable and retrying
in lockstep, a property of the host, so lowering `CONTENDERS` moves a rate and cannot
make a busy store and a broken store distinguishable. That is a reason not to buy a
lowering **as a repair for this defect** — it is not a reason against a lowering for
wall time, which is `concurrency-family-and-contender-count`'s to weigh, now with the
in-tree/published divergence in front of it. The `RUNBOOK.md:159`, `:2700-2710` and
status-table amendments remain a real cost of any move, in either direction.

**Why Option 2 lands now regardless.** It costs no API, no clause, no version, and
forecloses nothing. It repairs the half of the defect the audit's own "Why a defect"
paragraph identifies as the harm — *"That sentence tells an adapter author their store
is wrong when nothing about their store is wrong"* (`review:1337`) — and it is the only
part of this decision that is free in every direction. It should not wait on Option 3.

**The gap is real, and naming the fork is not deferring the gap.** A transient,
contention-induced refusal by a store whose semantics are perfect is indistinguishable
*by construction* from a semantic violation, and CF-33 `[FROZEN]` guarantees no rule may
ever tell them apart with a clock. That is true of SQLite, of a `SERIALIZABLE` Postgres,
of a one-shot-HTTP Neon, and of any pooled adapter — a port-level fact, not a `rusqlite`
fact. What is *not* settled is which side of the port pays for it.

**The fork, stated so whoever takes it inherits the constraints rather than re-deriving
them.**

* **Option 3's placement is forced.** `ConcurrentFixture`'s blanket impl
  (`concurrency.rs:186-193`) is closed to required items, so a fixture-side declaration
  lands on `Fixture`, **defaulted** — the sibling brief's Option A, CF-39's shipped
  template, the fifth consecutive instance of the event-store family's practice. Its
  floors must be structural minima rather than a tolerated fraction, for CF-34's stated
  reason. Its surface is additive to add and **breaking to remove**.
* **Option 6's precedent is now the live one**, and its cost is contract-crate surface
  bought on one implementor, which §10 objects to in terms. Its re-open trigger —
  Postgres and Neon independently needing it — is the same evidence that would settle
  the fork.
* **Both are gated by the same missing instrument.** The assertion either would soften
  has **no registered wrong implementation today** — none of the six racers produces
  `Attempt::Failed` — and the floors either introduces are new assertions that would
  arrive with nothing able to fail them. CLAUDE.md's bar applies in both directions.
  **Write the audit's decorator first** (*"a decorator over `MemoryEventStore` whose
  `append` refuses the first m callers with a transient store error"*, `review:1341`);
  it is the instrument that discriminates the two arms as well as gating either, because
  a per-error channel and a per-fixture capability behave differently against it.

**The strongest argument against pre-committing to nothing.** Naming a fork is how a
question stops being answered. Option 3's shape is known, its placement is forced, and
the harm it addresses is already in a published crate; a brief that declines to pick
leaves the repair waiting on Postgres or Neon, neither of which is started, which is
Option 1 wearing a schedule. The counter — and the reason the recommendation lands where
it does — is that Option 2 removes the *harm* (a message that sends an author to debug
correct code) immediately and for free, leaving only the *classification* to the fork;
and that the instrument gating both arms does not exist yet, so no pre-commitment could
be honoured this week anyway.

**The strongest argument against Option 3 specifically, in its own words** — retained
because it now argues for the recommendation rather than against it, and because
whoever takes the fork must answer it:

> A defaulted `Capability` whose default is *declined* buys nothing for the very
> population the entry says is worst affected. The out-of-tree author on a plateau-class
> machine, meeting a red suite on their first `cargo test`, has not declared the
> capability and will not have — they do not yet know it exists. What they get from
> Option 3 is Option 2's better message and nothing else, and Option 2 is free. What
> Option 3 actually buys is the ability of an *informed* adapter author to turn off a
> liveness assertion, permanently, in exchange for a sentence in a CI log — and the
> only measured need for it is **three attempts in 6,720 on one Windows host**, all
> inside one launch, with `busy = 0` in the other six and in all 44 `n = 64` rows
> elsewhere in the corpus. On that evidence the honest reading is that this is a
> property of *one machine at one contention level*, that `busy = 0` in the other
> forty-two rows is weak evidence of a rare tail rather than of a real hazard, and
> that buying published trait surface for it before any adapter has asked is the move
> ADR-0022 §10 exists to stop — *one implementor is not a spread* applies to
> `happenstance-testkit`'s fixture contract exactly as it applies to
> `happenstance-core`'s ports.

The counter this brief previously offered — *"the surface is defaulted and therefore
recoverable"* — has been **removed as false**: a defaulted associated const is additive
to add and **breaking to remove**, so an out-of-tree fixture that declares it pins the
const in place. With that counter gone, the argument above stands unrebutted against
Option 3 and is part of why the recommendation no longer pre-commits to it. What remains
true is that the population cost of the current message is being paid now, in
`0.2.0-alpha.1`, and that Option 2 is what pays it down — which argues for Option 2 and
not for either arm of the fork.

---

## Cost of delay

**It is not free-now-and-permanent-at-0.2.0, and the audit's Semver field frames it
more sharply than the arithmetic supports.**

* **Adding the API half is equally cheap forever; withdrawing it is not.** The tolerance
  is a *defaulted* associated const on `Fixture`. A defaulted const is additive at `0.x`
  and remains additive after `1.0`; every existing impl keeps compiling whenever it
  lands. The sibling brief's corrected caret arithmetic reinforces this —
  `happenstance-testkit = "0.2"` is `>=0.2.0, <0.3.0` and never resolves to `0.3.0`, so
  no adapter is dragged into the change by a `cargo update` either. The audit's *"the one
  change here whose cost rises the day 0.2.0 ships"* (`review:1343`) is right about the
  class of hazard but not about this instance: what it describes — *"a rule or a
  classification added after 0.2.0 goes red on every adapter"* — is the hazard of a
  **tightening**, and this change is a loosening. **The asymmetry this bullet used to
  claim in the other direction is gone:** removing a defaulted const an out-of-tree
  fixture has written is a **breaking** change, so waiting costs nothing that landing it
  early would refund.
* **What is genuinely lost by delay is population cost, not optionality — and the meter
  is already running.** `happenstance-testkit 0.2.0-alpha.1` has been on the registry
  since **2026-08-16** carrying these three rules and this message, so the population is
  not waiting on phase 12's stable `0.2.0` to start paying. Every adapter author who
  meets the message pays it once, and out-of-tree they pay it without ADR-0022 to explain
  it. Size it honestly in the other direction too: the published crate races **8**
  contenders, and the corpus records no `busy > 0` at 8. That argues for **Option 2
  immediately** — which is free — and not for either arm of the fork.
* **The claim that `CONTENDERS` gets harder after publish has been removed.** It read:
  *"after `0.2.0` a raise is CF-31 `[FROZEN]` tightening in place."* It assumed the
  published value is 64. It is 8, and the raise to 64 already happened after publication
  (`995b987`, 2026-08-17) with no version bump and no CF-31 citation. Whether CF-31
  governs the constant is now itself an open question rather than a cost of delay.
* **The §11 superseding atom is owed on a schedule of its own.** The falsifier fired
  on 2026-09-03; `.kb/decisions/0022-append-condition-strategy.md` carries
  `last_reviewed: 2026-08-17`. It is a row in `RUNBOOK.md`'s ADR queue and it does not
  wait on this question.

---

## What this does not settle

- **Whether `BUSY_TIMEOUT_MS` stays at 5,000.** That is the §11 superseding atom's, and
  W-1 deliberately proposes no number. The `connection.rs:59-61` doc comment's claim
  needs correcting either way.
- **Which side of the port pays for the classification — Option 3 or Option 6.** This is
  the fork this brief names and does not close. It was previously closed in Option 3's
  favour; that pre-commitment has been withdrawn.
- **Whether the floors either arm introduces can be failed by anything.** The decorator
  the audit names does not exist, and this brief gates *both* arms on it rather than
  assuming the answer.
- **Whether `AppendError::Busy` lands in `happenstance-core`.** No longer refused here:
  the §10 refusal rested on a precedent about an API convenience while VT-25 / CF-40 —
  the same harm, the same channel, minted at phase 4 with zero adapters — points the
  other way. Its re-open trigger, Postgres and Neon independently needing it, would
  settle the fork; whether the fork can be settled *before* that is the open question.
- **Whether CF-31 `[FROZEN]` governs `CONTENDERS` at all.** The constant moved `8 → 64`
  after `0.2.0-alpha.1` shipped, with no version bump and no ADR. Either that was a
  CF-31 event nobody recorded, or CF-31 is about rule *meaning* and a contention level
  is not one. `concurrency-family-and-contender-count` owns the answer and this brief
  supplies only the fact.
- **The in-tree/published divergence in `CONTENDERS` itself** — 64 in the tree, 8 on the
  registry — which is a live discrepancy of exactly the kind `RUNBOOK.md:2700-2710` says
  rots when left, and is owned by the same story.
- **Whether `exactly_one_of_n_contenders_commits` is fully deterministic under a
  tolerance.** `committed.len() == 1` survives only while the *winner* is never the
  contender that gets busied. In `rate-n64-r2` `committed` stayed at 5 in every row,
  so the losers took it — but nothing in the store or the suite guarantees that, and
  the corpus contains one incident.
- **The exact spelling of the declaration** — `Capability` versus a predicate hook
  versus both. `Capability` inherits CF-18's reporting obligation and CF-39's
  clause-level MUST template; a bare predicate does not. That is a design question for
  whoever writes it.
- **Which clause home the tolerance gets.** ADR-0034 says the fixture contract has no
  single owning document and that whoever needs a capability mints its clause; this
  brief names no `CF-` number and should not.
- **M-1's `block_on` repair itself.** It is separable, needs no ADR (`review:1381`), and
  is recommended here only because it narrows what a hung job can mean — which is what
  makes the busy diagnosis actionable rather than a guess.
- **Whether `CONTENDERS` should move for reasons other than contention** — CI wall time
  at 10.9x–20.0x per race is a real cost and a legitimate separate question, owned by
  `concurrency-family-and-contender-count` and not answered here.

---

## Revision record

Two critiques declined the first version's recommendation. Both were checked against the
repository before anything here moved; both held. The recommendation flipped and five
claims were removed rather than rewritten around.

### The recommendation flipped

**Was:** *"Option 2 now, unconditionally. Option 3 in shape, gated on its instrument.
`CONTENDERS` does not move."*
**Now:** *"Option 2 now, unconditionally. Record neither Option 3 nor Option 6 — name the
fork and pre-commit to neither. `CONTENDERS` is re-opened and is not settled here."*

**What flipped it, verbatim:**

> Option 6 is the strongest option, and the brief refuses it on the wrong precedent. It
> leans on §10's `index_arms()` — an API convenience on a query type — and names then
> drops the nearer one: VT-25/CF-40, where the identical harm (a refusal arriving on the
> `Store(E)` channel that a caller must tell from a real failure) was repaired as a
> *pair* — core variant + defaulted fixture fact + one rule + four mutants — minted at
> phase 4 with zero adapters, on named-but-unbuilt shapes. A per-error channel beats a
> fixture capability: no published `Fixture` surface, it helps the out-of-tree author who
> declares nothing, and it cannot let an adapter switch liveness off wholesale for all
> its errors. Today the contract gives contention no correct channel — `ConditionViolated`
> is already a registered defect. So: Option 2 now, record *neither*, name the fork; do
> not pre-commit to Option 3's shape.

> This is the ADR-0029 defect verbatim. The brief cites 0.2.0-alpha.1 to prove Fixture is
> published API, then reasons about CONTENDERS as if publish were ahead ("once 0.2.0
> ships", "gets harder after publish"). The published artifact carries CONTENDERS=8, and
> the repo moved the constant the day after publishing without treating CF-31 as a bar.
> Three of four supports for "CONTENDERS does not move" collapse: lowering to 8 restores
> the currently published value, so it is a revert, not one-way; §12's table measures 8
> with "exactly one winner per race at both counts, busy=0 and failed=0 throughout"; and
> §12 refused to apply a number ("supplies the number and does not apply it: CONTENDERS
> is still 8"), not to supply one, inverting the brief's "the exact move §12 refused".
> Only W-1's core-axis argument survives. Option 4 is far cheaper than priced; Options
> 4/5 reopen.

### Claims removed as resting on falsified premises

1. **"`ExceedsStoreLimit` was minted for a property every store has; a `Busy` variant
   today would be minted for one."** Removed. CF-40 defaults all three ceilings to `None`
   and states that `None` means the store has no ceiling (`spec/SPECIFICATION.md:8093-8099`),
   and the testkit had to build `GappedPositionFixture` — which states all three at
   VT-21 – VT-24's floors — so that `append_reports_exceeded_store_limits` runs anywhere
   (`crates/happenstance-testkit/tests/mutation_coverage/variants.rs:249-261`). Most
   stores have no ceiling. The discriminator was the only thing holding Option 6's
   refusal apart from a precedent that supports it, so the refusal went with it.
2. **"The surface is defaulted and therefore recoverable."** Removed. Adding a defaulted
   associated const is additive; **removing** one that an out-of-tree fixture has written
   is breaking. Option 3's surface is cheap to buy and not cheap to unbuy, which deleted
   the asymmetry the old recommendation used to justify buying it early. The
   `Cost of delay` bullet built on it was corrected in the same way.
3. **"There is no measurement below 64 anywhere."** Removed. ADR-0022 §12 measured 8 and
   64 in one round-robin (`0022:497-503`,
   `experiments/append-condition/results/contention-64.md:60-66`), with `busy = 0` and
   `failed = 0` at both. What survives is the narrower and true statement: no measurement
   below 64 **under the gate's own configuration** — which is equally true of 64.
4. **"A lowering is close to one-way, because CF-31 `[FROZEN]` rejects the re-raise"**,
   and the matching *"`CONTENDERS` is the one thing that gets harder after publish."*
   Removed. `happenstance-testkit 0.2.0-alpha.1` (2026-08-16, `448e1ac`) publishes
   `CONTENDERS = 8`; `995b987` raised it to 64 on 2026-08-17 with no version bump and no
   CF-31 citation. A lowering to 8 is a revert to the published value, and CF-31 has not
   in practice been treated as governing the constant.
5. **"Choosing a lower number today … is the exact move §12 refused."** Removed as an
   inversion. §12 refused to **apply** a number it had measured — *"This record supplies
   the number and does not apply it"* — not to supply one.

### Corrections that follow

- The publication timeline is now stated once, in *What is true today*, and every
  future-tense *"once 0.2.0 ships"* framing about the constant is gone. The alpha is
  live; the population cost of the current message is being paid now, not from phase 12.
- Option 1 and *Cost of delay* now say that the **published** suite races 8 contenders,
  so the 3-in-6,720 `busy` rate is a fact about the in-tree constant and not about what
  an out-of-tree author meets on the registry crate.
- Option 4 is re-priced and Options 4 and 5 are re-opened. The only surviving argument
  against a lowering *as a repair for this defect* is W-1's core-axis result; wall-time
  arguments for one are untouched and belong to
  `concurrency-family-and-contender-count`.
- Option 6 is re-headed *live, not refused*, and the §10 versus VT-25/CF-40 precedent
  conflict is stated rather than resolved by citing one side.
- Three new items were added to *What this does not settle*: which arm of the fork wins,
  whether CF-31 governs `CONTENDERS`, and the in-tree/published divergence in the
  constant itself.

**Not changed by this revision:** the entry evidence (the fired §16 falsifier, the
3-in-6,720 rate and its two precision notes), the three-rule analysis in *Three rules
break on a busy refusal*, the placement constraint on `Fixture`, the clause table,
Option 2 in full, Option 7's refusal, and the standing obligation to write a superseding
atom for ADR-0022 §11 whatever is decided here.
