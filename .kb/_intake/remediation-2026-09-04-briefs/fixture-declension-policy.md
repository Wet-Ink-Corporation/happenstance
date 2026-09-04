# May a `happenstance-testkit` minor add a REQUIRED associated const or method to a fixture trait, or must every future capability carry a testkit-written default?

Decision record: **C2-04-fixture-declension**. Brief only — no ADR prose, no atom, no code.

> **RATIFIED AND LANDED — Option A.** The repository owner ratified Option A and
> the `lane/fixture-declension` branch implemented it: `ProjectionFixture`'s three
> capability constants are now defaulted declensions, and the trait-level "the
> fixture writes the reason" rule is retracted in `contract.rs`, with its argument
> preserved as the reason to *override* rather than as a requirement. Every
> `contract.rs:NNN` citation below describes the tree **before** that change and is
> left as written, because this document is the input to the decision rather than a
> description of the result. `projection-declension-obligations.md` in this
> directory carries what the retraction left owed and did not take.

---

## Why this is owed

**The review filed it, and the entry is narrower than the problem.** C2-04
(`references/evaluation/review-pre-publication-2026-09-03.md:1699-1724`) reports that
`ProjectionFixture`'s stated declension policy makes every future capability
`error[E0046]` in every out-of-tree fixture, and that §6.6 — the one section that
governs what a testkit release may do to an adapter — names no such class. Its own
"Reshaping, recorded" paragraph (`:1720`) widens it: the base `Fixture` trait already
has the same shape today, not as a future risk.

**Three things force the question now rather than at phase 12.**

1. **The trait is published API.** `crates/happenstance-testkit/Cargo.toml:21` is
   `version = "0.2.0-alpha.1"`, and `CHANGELOG.md:306-308` records that release as
   *"The first published release, and it is a pre-release on purpose."* So
   `ProjectionFixture` and `Fixture` are on crates.io.
2. **The port is expected to move.** ADR-0036 says `ProjectionStore` is **not** frozen
   at `0.2.0` (`.kb/decisions/0036-the-projection-port-ships-gated.md:56-59`), so the
   next declinable capability the projection suite needs is not hypothetical — it is
   scheduled. Phase 6 is where the capability set next moves (`RUNBOOK.md:156`).
3. **The two fixture families are already following opposite policies, and the same
   review proposes both in the same document.** C2-04 records the projection family's
   rule that a capability is *required*; L3-01
   (`references/evaluation/review-pre-publication-2026-09-03.md:1644`) proposes the next
   event-store capability as *"a defaulted `const READ_FAULT: Capability` and a defaulted
   `fn arm_read_fault` on `Fixture` (defaulted, so no existing fixture breaks)"* and
   calls it **"Additive in every part"**. One trait family apart, two answers, neither
   written down where a release manager reads it.

**Nothing in the specification governs the class.** §6.6 is CF-29 through CF-32
(`spec/SPECIFICATION.md:8618-8709`): rule addition (CF-29), the pin recommendation
demoted to prose (CF-30), rule *meaning* change (CF-31), the independent version key
(CF-32). §6.3's fixture-contract clauses CF-15 through CF-18 and CF-39, CF-40 govern
what a fixture must *declare*, not what a release may *add*. The `CF` traceability
table runs CF-1 to CF-40 (`spec/SPECIFICATION.md:9204-9248`) and §6's own
"What this section does not settle" (`:8870-8922`) names DCB interop and the
projection suite's shape — not this.

**The house style already has a position on the mechanism, for a different trait.**
`standards/rust/40-public-surface-and-evolution.md:12-16`, RS-40-1:

> **RS-40-1. Grow a port with a new trait, never with a new required method.**
> **Why.** Every trait method needs a body somewhere, and `#[non_exhaustive]` does
> not apply to traits — so a new required method is `error[E0046]` in every
> adapter's crate, on a *minor* bump.

Read its words exactly: it says **port**, and `Fixture` is not a port. But that
atom's own trigger table names *"a fixture must decline an operation"* (`:5-6`), and
its rationale transfers verbatim — the compiler does not care which trait it is.

---

## What is true today

### The base `Fixture` trait: two required consts, three defaulted, one required method

`crates/happenstance-testkit/src/contract.rs:131-332`.

```rust
pub trait Fixture {
    type Store: EventStore;                        // :136  required
    const SECOND_HANDLE: Capability;               // :172  required
    const REOPEN: Capability;                      // :184  required
    const MID_BATCH_FAULT: Capability = …;         // :218  DEFAULTED
    const MAX_EVENT_DATA_LEN: Option<usize> = None;    // :264  DEFAULTED
    const MAX_TAGS_PER_EVENT: Option<usize> = None;    // :273  DEFAULTED
    const MAX_EVENTS_PER_BATCH: Option<usize> = None;  // :290  DEFAULTED
    fn arm_mid_batch_fault(&self, after: usize) -> …   // :308  defaulted (panicking body)
    fn connect(&self) -> impl Future<Output = Self::Store>;  // :332  required
    fn reopen(&self) -> …                              // :350  defaulted (panicking body)
}
```

The doc names the MUST and names it as singular (`contract.rs:146-152`):

> **This one is a MUST, and it is the only capability here that is.**
> SPECIFICATION.md CF-16 requires every fixture to be able to open a second
> handle; [`REOPEN`](Self::REOPEN) is a `SHOULD`, because a volatile store
> declining it is an honest answer.

Note the asymmetry that sentence creates and does not resolve: `REOPEN` is a
`SHOULD` **and is still a required item** (`:184`). The trait requires an answer to a
capability the specification does not require the fixture to have. That is deliberate
— `contract.rs:212-217` says so about the constant next door:

> A store with no way to fail one row of a batch declines, and the default
> below is that answer. It is defaulted rather than required — unlike
> [`SECOND_HANDLE`](Self::SECOND_HANDLE) and [`REOPEN`](Self::REOPEN),
> which every fixture must answer deliberately — because an in-memory store
> has no fault to inject and demanding an answer would buy one more line of
> boilerplate per fixture and no information.

So the event-store family's operative policy is: **the two founding consts are
required; everything added since has been defaulted.** CF-39 and CF-40 confirm it at
clause level. CF-40 is a MUST *to state* discharged by a default
(`spec/SPECIFICATION.md:8093-8096`): *"A fixture MUST state its store's capacity
ceilings as `Option<usize>` associated constants … each defaulting to `None`."*

### The `ProjectionFixture` trait: three required consts, no defaults

`crates/happenstance-testkit/src/contract.rs:536-733`. `SECOND_HANDLE` (`:575`),
`RESET_REFUSAL` (`:604`), `COMMIT_FAULT` (`:640`) are all required; `arm_commit_fault`
(`:657`), `protect_from_reset` (`:705`) are defaulted panicking bodies; `connect`
(`:733`) is required. The policy is stated on the constant and restated one constant
later (`contract.rs:588-596`):

> It is **required rather than defaulted**, unlike
> [`Fixture::MID_BATCH_FAULT`], and that is a deliberate difference of one
> line per fixture. A default would have to carry a *testkit-written*
> reason, and the projection family's declension policy is that the fixture
> writes the reason — a store's account of a trade only it can describe.
> The one standing exception to that policy on the event-store side
> ([`NO_CEILING_REASON`]) exists because "this store has no ceiling" is the
> same sentence for every store that says it; "this store refuses no reset"
> is not, because *why* it refuses none is the interesting half.

And at `:619-627`, for `COMMIT_FAULT`:

> *why a particular store cannot make a commit fail* is not — an in-memory map
> applies both halves under one lock, a one-shot HTTP backend has no interactive
> transaction to abort, and a pooled adapter usually can. The cost is one line per
> fixture and the return is that no fixture author is left un-asked.

The argument is good and the review says so (`:1716`: *"Not the policy, which is well
argued"*). What it does not say is what the policy costs on a version bump.

### The break, measured on the repository's own toolchain

`rust-toolchain.toml` pins `channel = "1.97.1"`; `rustc --version` reports
`rustc 1.97.1 (8bab26f4f 2026-07-14)`. Three probes, run in the scratchpad against
minimal models of the real shapes (the repository was not modified):

**1. A new required const is `E0046` in every existing impl.**

```
error[E0046]: not all trait items implemented, missing: `NEW_THING`
3 | impl ProjectionFixture for OutsideFixture {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `NEW_THING` in implementation
```

**2. A new required item on `ConcurrentFixture` is `E0046` on the blanket impl
itself** — there is nowhere to write the value.

```
error[E0046]: not all trait items implemented, missing: `NEW_THING`
4 | impl<F> ConcurrentFixture for F where F: Fixture {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `NEW_THING` in implementation
```

**3. And an adapter cannot supply it either** — the blanket impl already covers them:

```
error[E0119]: conflicting implementations of trait `ConcurrentFixture` for type `MyFixture`
```

The real blanket impl is `crates/happenstance-testkit/src/concurrency.rs:186-193`:

```rust
pub trait ConcurrentFixture: Fixture<Store: Send> {}

impl<F> ConcurrentFixture for F
where
    F: Fixture,
    F::Store: Send,
{
}
```

Its documentation states the property that makes it closed (`concurrency.rs:180-182`):
*"The blanket implementation means no adapter ever writes it: any fixture whose handle
can be moved to another thread already is one."* **Consequence: `ConcurrentFixture` is
permanently closed to required items while that impl stands.** Any capability the
concurrency rules need has to land on `Fixture` — defaulted or required — and cannot be
quarantined on the concurrency trait. That forecloses one of the options a reader
would otherwise reach for.

### The falsifier: two impls, and what `E0046` would mean for each

`examples/outside-projection-adapter` is the crate positioned so the orphan rule and
the dependency graph behave as they do for a stranger
(`examples/outside-projection-adapter/src/lib.rs:3-10`). It carries **exactly two**
`ProjectionFixture` impls, both in `tests/support/mod.rs`, and its module header states
why they are there rather than in `src/` (`mod.rs:3-7`): the testkit is a
dev-dependency, so `ProjectionFixture` does not exist for the library build at all.

- **`OutsideFixture` (`mod.rs:39-64`)** — the conformant subject. `SECOND_HANDLE =
  SUPPORTED`, `RESET_REFUSAL` declined by reading the store's own constant
  (`mod.rs:50-51`, resolving to `src/lib.rs:135-138`), `COMMIT_FAULT = SUPPORTED` with
  a real override. `E0046` here means: **the crate that exists to prove the extension
  surface is met by a stranger stops compiling**, and the one falsification instrument
  the projection port has goes dark until somebody edits it.
- **`CheckpointOnlyFixture` (`mod.rs:85-99`)** — the control, over the store that
  commits the checkpoint and drops the read-model write. `E0046` here means: **the
  negative control goes dark too**, so "the suite passed" and "the suite can fail"
  stop being separable in the same build.

The manifest is written long-hand on purpose to model an outsider's
(`examples/outside-projection-adapter/Cargo.toml:10-19`), and its dev-dependency is
`happenstance-testkit = { version = "0.2.0-alpha.1", … }` (`:36`).

### The version policy as it stands, and the contradiction inside it

`CHANGELOG.md:15-18`:

> **`happenstance-testkit` has its own version**, independent of the other
> crates. Adding a conformance rule is a semver-*minor* change that can turn a
> passing adapter's CI red, so treat a minor bump there as breaking and pin it
> exactly.

`crates/happenstance-testkit/README.md:207-211` repeats it. But
`crates/happenstance-testkit/src/lib.rs:205` — the onboarding recipe that *does* reach
docs.rs — tells an adapter author to write `happenstance-testkit = "0.2"`, and
`README.md:207-211`'s "pin it exactly" never reaches docs.rs. C2-06 (`:1925`) records
exactly that split.

> **Removed, and not rewritten around.** This paragraph previously said that
> `happenstance-testkit = "0.2"` is *"a caret that takes the next minor
> automatically."* That is false, and it misquoted the source it cited. The crate is
> pre-1.0 (`Cargo.toml:21`, `0.2.0-alpha.1`), so Cargo's compatibility unit is the
> **minor**: `^0.2` is `>=0.2.0, <0.3.0`. It takes patches and never resolves to
> `0.3.0`. C2-06 at `:1925` says exactly that — *"a caret that takes the **patch**
> automatically"*. Everything the brief built on the false reading has been removed
> below rather than restated in weaker terms; the removals are listed in the revision
> record.

**The arithmetic that replaces it, and it is load-bearing for the options.** At `0.x`
the minor *is* the incompatible unit, so **"minor bump" and "major bump" name the same
release event here: `0.2.x → 0.3.0`.** `cargo-semver-checks` applies the same `0.x`
rules, so a major-category change satisfied by a `0.2 → 0.3` bump is not a finding to
suppress. Two consequences run through everything below. First, Options B and C are
indistinguishable in mechanism until `1.0`; their difference is prose. Second, a rule
addition already consumes an incompatible bump under CF-29 — `Cargo.toml:10-13` records
`0.1 → 0.2` spent on exactly that, *"that change added thirty-four rules"* — so the
version number is already being spent on rule additions and has no separate signal left
for a trait item to dilute.

So the *stated* policy already treats a testkit minor as breaking. What it was written
about is a **CI break** — an adapter with a defect goes red. A required-item addition is
a **source break** — the adapter fails to compile whether or not it is conformant, in
its own crate, naming an item its author has never heard of. Those are not the same
event, and no document in the tree distinguishes them.

### The prior question nobody has answered

C2-04 flags it (`:1718`) and it is load-bearing for the semver class. ADR-0036's
exemption names `happenstance-core` and `happenstance`, forwarded as `projection-store`
on adapters. It does **not** name `happenstance-testkit`, whose projection module is
unconditional (`crates/happenstance-testkit/src/lib.rs:350-355`: *"Unconditional, unlike
its two nearest templates"*) and which turns `unstable-projection` on for
`happenstance-core` unconditionally in its own manifest
(`crates/happenstance-testkit/Cargo.toml:49-55`). On the documents as written,
`ProjectionFixture` is ordinary, un-exempt, published API.

### The governing decision record

**ADR-0034, `.kb/decisions/0034-the-fixture-contract-has-no-single-owner.md`**, status
`accepted`. Its durable half (`:86-96`):

> **Second finding, and the durable one: the fixture contract has no single owning
> document.** A `CF-` clause is minted by whichever decision first needs the
> capability, and it carries its reason there, beside the adapter that needed it,
> rather than in a central fixture-contract document.

It is about **who mints a clause**, not about **what a release may add**, so it does not
answer this question — but it shapes the answer twice. First, by construction there is
no owner standing by for the clause this brief points at; whoever needs the next
capability mints it. Second, ADR-0034 names its own falsification trigger (`:118-128`):
phase 10's `POLL_BUDGET`-shaped capability, owned by
`.kb/open-questions/poll-count-bounds-the-visibility-rule.md`. **`POLL_BUDGET` is the
first live instance of exactly this question**, and ADR-0034 says that if it cannot be
minted piecemeal, that collision is the evidence that would supersede ADR-0034.

**Which atom would a decision here supersede? None.** ADR-0034 is about clause
ownership and would be `related`, not superseded — a decision on declension policy does
not contradict "no single owning document", it operates under it. ADR-0036 sets the
gating exemption's scope and would also be `related`; if the owner decides the exemption
*does* reach `happenstance-testkit`, that is a **widening of an accepted atom's scope**
and would need its own superseding atom rather than a reading. ADR-0034 has **no long
form** in `references/adr/` — that directory holds `0001`–`0023`, `0029`, `0030` and
`0035` only, so 0034 and 0036 are atom-only — and the atom is therefore the whole record
for both.

---

## Options

### Option A — Everything future is defaulted. No fixture trait ever grows a required item.

Retracts the projection family's stated declension policy and replaces it with the
event-store family's practice.

- **Costs an adapter author:** nothing at compile time; a minor bump keeps compiling.
  What it costs is the property `contract.rs:625-627` claims — *"no fixture author is
  left un-asked."* A capability they never mention is declined on their behalf with a
  sentence the testkit wrote, printed in their CI log and read by their users as their
  store's account of itself.
- **The precedent this was originally priced against was the wrong one.** The brief
  cited CF-40 (`spec/SPECIFICATION.md:8103-8106`, *"would put a fiction in the CI
  log"*). CF-40 governs **facts** — `Option<usize>` ceilings, which its own text says
  *"MUST NOT be spelled as `Capability`"* because *"`None` here is not a trade."* A
  declension policy is entirely about trades, so CF-40 is off-point in both directions.
  **The on-point precedent is CF-39** (`spec/SPECIFICATION.md:8063-8078`), which governs
  `MID_BATCH_FAULT` — a genuine trade, **defaulted** on the trait, whose default carries
  a testkit-written reason drafted in the fixture's own voice (`contract.rs:218-222`:
  *"the injection has to come from the adapter and **this one has none to offer**"*) —
  and which recovers the honesty a default gives up as a clause-level MUST: *"A fixture
  whose store can absorb every fault it is able to arm MUST decline the capability with
  that as its stated reason"*, plus *"The fixture MUST state the mechanism."* That is
  Option A, already shipped and already spec-blessed, for the capability whose shape is
  closest to `COMMIT_FAULT`. So A's real cost is not "a fiction in the CI log"; it is
  that the testkit's default sentence must be true of every store that never overrides
  it, and a clause must carry the obligation the default cannot.
- **Costs a caller (the adapter's users):** the skip line stops being evidence about the
  store. `NO_CEILING_REASON` is legible because the fact is uniform; a default
  `COMMIT_FAULT` reason would have to say "this store may or may not be able to fail a
  commit; its author did not say", which is worse than silence.
- **Semver class:** additive/minor, genuinely. `cargo-semver-checks` reports nothing.
- **Forecloses:** the "the fixture writes the reason" policy as a *trait-level*
  requirement, for both families. Recovering the reason afterwards needs a CF-39-shaped
  clause-level MUST per capability.

  > **Removed, and not rewritten around.** This bullet previously priced that
  > recovery by citing L1-2 — *"prose with no compiler behind it … `NoopReopenFixture`
  > over-claims `REOPEN` and scores 86 passed, 3 skipped … A clause alone did not stop
  > it"* — and the premise is false. `REOPEN` is a **required** const
  > (`contract.rs:184`), not a defaulted one. In L1-2 the compiler *did* force an
  > answer and the answer was a lie, and the review states that no rule written against
  > today's `Fixture` surface can reject it (`:1615`). So L1-2 measures **requiredness
  > failing**, not a clause failing — and the review's own remedy for it is *"CF-39's
  > other half — a clause-level MUST on what declaring `REOPEN` commits a fixture to"*
  > (`:1615`). Cited honestly, L1-2 is evidence *against* the trait-level requirement
  > and *for* the clause, which is the opposite of the use it was put to. The claim is
  > removed; nothing replaces it, because the repository holds no measurement of a
  > CF-39-shaped clause's yield either way.

### Option B — A minor may add a required item; the crate declares that its minors are source-breaking.

Keeps the declension policy and extends the existing "treat a minor as breaking" convention to cover compilation.

- **Costs an adapter author:** *(removed — see the box below)*. Measured blast radius
  today, which survives the removal: **nine**
  `ProjectionFixture` impls (`crates/happenstance-sqlite/tests/projection.rs:134`;
  `contract.rs:445`, `:484`, `:519` — three of them doctests inside the trait's own
  documentation; `fixtures.rs:434`;
  `tests/mutation_coverage/variants.rs:633`; `tests/projection_mutation_coverage/buffering.rs:475`;
  and the falsifier's two) and roughly two dozen `Fixture` impls.

  > **Removed, and not rewritten around.** B's only stated cost to an adapter author
  > was *"a red build on `cargo update` inside a caret they were told to write
  > (`lib.rs:205`), with `error[E0046]` naming a const they have never heard of."* It
  > rested on the falsified caret premise and cannot occur. Under B the number still
  > moves `0.2.x → 0.3.0`, which `^0.2` will not resolve, so no `cargo update` reaches
  > the new item. The red build occurs only if the item ships in a **patch**, which
  > CF-29's `Rejects:` already forbids for a bare rule addition
  > (`spec/SPECIFICATION.md:8649-8653`) and which no option here proposes.
- **Costs a caller:** nothing directly.
- **Semver class:** **source-breaking, called minor — but at `0.x` that is the same
  release event as "major".** `0.2.x → 0.3.0` either way.

  > **Removed, and not rewritten around.** This bullet previously said
  > `cargo-semver-checks` *"will report `trait_missing_associated_constant` as major on
  > every such release; the project would suppress it each time"*, and the foreclosure
  > bullet built on it (`cargo-semver-checks` losing authority over the testkit). Both
  > rested on the falsified premise. `cargo-semver-checks` applies `0.x` rules, and a
  > `0.2 → 0.3` bump satisfies a major-category change — there is nothing to suppress
  > and no authority lost. The claim becomes true only after `1.0`, which is beyond
  > every phase this brief reasons about, and it is removed rather than deferred
  > because a cost that does not exist yet cannot decide between options today.
- **Forecloses:** nothing that survives the removal above, until `1.0`. C2-06's proposed
  rule-set baseline still would not see a trait item — it diffs *rule names* against a
  version key (`xtask/src/lints.rs:527-616`) — but that is equally true under C and is
  not a difference between them.

### Option C — A required item may be added, and the release that carries one is a MAJOR.

Keeps the declension policy intact; classifies the addition honestly.

- **Costs an adapter author:** nothing unplanned. `happenstance-testkit = "0.2"` does not
  resolve to `0.3.0`, so the upgrade is deliberate and the one-line addition is written
  with the changelog open. This is the outcome CF-30's retained advice was reaching for
  (`spec/SPECIFICATION.md:8654-8669`) without depending on the author having taken it.
  **This is true, and it is equally true of B** — the corrected arithmetic gives B the
  same `0.3.0` and the same non-resolving caret. It is not an advantage of C.
- **Costs the maintainer:** version-number churn. But the churn is cheaper here than
  almost anywhere: at `0.x` the compatibility unit is the minor, so a "major" is
  `0.3.0`, and CF-32 (`spec/SPECIFICATION.md:8684-8700`) exists precisely so the
  testkit's number can move without dragging `happenstance-core`. The manifest already
  moved *down* once for this reason (`CHANGELOG.md:310-313`).
- **Semver class:** major, correctly, with no override to justify and nothing for
  `cargo-semver-checks` to be suppressed about.
- **Forecloses:** *(removed — see the box.)*

  > **Removed, and not rewritten around.** This bullet claimed C forecloses *"part of
  > CF-31's current signal"* — that if majors also mean "the trait grew a line", a
  > reader can no longer tell from the number which happened. The same corrected
  > arithmetic voids it: at `0.x` a rule addition already consumes the incompatible
  > bump under CF-29, and `Cargo.toml:10-13` records `0.1 → 0.2` spent on precisely
  > that (*"thirty-four rules"*). The number is already doing rule-addition duty, so
  > **there is no undiluted CF-31 signal left for C to dilute.** Removed; the same
  > removal takes out the brief's former "strongest argument against", which was this
  > claim in longer form.

**Worth noticing about C: CF-31's existing text may already say this.** Its own words
(`spec/SPECIFICATION.md:8670-8674`):

> A major release of `happenstance-testkit` means a rule's **meaning**
> changed, or a rule was removed or renamed — anything that can make a
> previously-passing conformant adapter fail for a reason other than a
> newly-detected defect.

A required-item addition is a conformant adapter failing for a reason other than a
newly-detected defect. The enumerated causes do not name it; the generalising clause
covers it. So Option C may be a *reading* plus a sentence, where B and A are amendments.
The clause is `[FROZEN]`, and CLAUDE.md holds that changing a frozen clause needs a new
ADR — so if the owner reads CF-31 as already covering this, the cheapest instrument is a
new clause in §6.6 that *names the class and points at CF-31*, leaving CF-31's bytes
alone.

### Option D — The capability lands on a new opt-in trait with its own macro (RS-40-1's shape)

- **Costs an adapter author:** a second macro invocation to get the new rules, and
  nothing if they skip it.
- **Costs a caller:** **CF-18, by construction.** CF-18 is `[FROZEN]`
  (`spec/SPECIFICATION.md:8029-8032`): *"a rule whose capability requirement is unmet
  MUST still be emitted as a test that **reports** the skip … A rule MUST NOT be silently
  omitted."* A fixture that never invokes the second macro emits no test and no skip
  line — the exact `#[cfg]`-out shape CF-18's `Rejects:` forbids. It also breaks the
  one-line onboarding promise (`CLAUDE.md`, `lib.rs:239`).
- **Semver class:** additive/minor, genuinely.
- **Forecloses:** it does not work for `ConcurrentFixture` at all — measured above,
  `E0046` on the blanket impl and `E0119` for any adapter trying to supply it. Putting
  the required item on an extension trait that the *existing* macro's hoisted bound names
  (`lib.rs:600`, `concurrency.rs:1176`) is the same break as B wearing a different error
  code (`E0277` instead of `E0046`).

---

## Recommendation

> **This recommendation flipped.** The brief previously recommended **Option C** — a
> release adding a required item is a MAJOR. Two things flipped it, both verified
> against the tree, and both recorded in full in the revision record at the foot.
> **(1)** C's headline advantage over B was the corrected caret arithmetic's casualty:
> at `0.x`, B and C are the *same release event* (`0.2.x → 0.3.0`), so "B keeps the
> policy by asking the version number to lie" has no referent, and C's remaining
> advantage over B is prose until `1.0`. **(2)** With B and C collapsed, C's entire
> remaining value is protecting the projection family's declension policy — a policy
> this brief explicitly declined to defend. A brief cannot recommend paying a cost
> whose only return is a position it refuses to take. Priced against the right
> precedent (CF-39, not CF-40) the policy is the departure, not the baseline, so the
> recommendation now settles the policy first, which is what the critique asked for.

**Option A — every future capability on either fixture trait lands defaulted, with the
honesty obligation carried by a CF-39-shaped clause-level MUST per capability rather
than by the trait's requiredness — and the projection family's stated "the fixture
writes the reason" trait-level rule is retracted to match.**

Why it beats the others:

- **It is what the tree does, with the widest spread behind it.** Every capability
  added to `Fixture` since the two founding consts has been defaulted —
  `MID_BATCH_FAULT` (`contract.rs:218`) and CF-40's three (`:264`, `:273`, `:290`) —
  five for five, across the family that actually has adapters. L3-01's proposed
  `READ_FAULT` is defaulted too and called *"Additive in every part"*
  (`review-pre-publication-2026-09-03.md:1644`). The projection family's opposite rule
  is **one documentation paragraph** (`contract.rs:588-596`) with **zero adapter
  experience** behind it: no projection adapter has run the suite, and the port is not
  frozen (ADR-0036 `:56-59`).
- **The one capability of the same shape is already decided the other way, in the
  specification.** `COMMIT_FAULT`'s nearest relative is `MID_BATCH_FAULT`: a trade, not
  a fact, whose reason is store-specific for exactly the reason `contract.rs:619-627`
  gives about commits. It is defaulted, its default reason is written in the fixture's
  own voice, and CF-39 recovers the honesty as a clause-level MUST including *"The
  fixture MUST state the mechanism"* and a MUST-decline for a store that can absorb
  every fault. That is Option A, shipped and spec-blessed, for the case the projection
  family departed from without saying what the departure buys.
- **The policy's measured yield out-of-tree is poor.** Three declensions are written
  under it in the falsifier. `OutsideFixture`'s `RESET_REFUSAL` is rich, and reads its
  reason off its own store's const (`tests/support/mod.rs:50-51` → `src/lib.rs:135-138`).
  `CheckpointOnlyFixture`'s two are not: its `RESET_REFUSAL` reason is **another
  store's const** — `OutsideProjectionStore::NO_RESET_PROTECTION`, over a different
  store (`mod.rs:90-91`) — and its `COMMIT_FAULT` reason is content-free, *"this store
  cannot be made to report a failed commit"* (`mod.rs:93-94`), which says *that* it
  cannot and not *why*. Requiredness bought a filled-in line, not a store's account of
  itself. The caveat below still stands: that fixture is a deliberately-wrong store's
  control.
- **L1-2 is the direct measurement of what requiredness buys, and it is negative.**
  `REOPEN` is required (`contract.rs:184`). The compiler forced an answer;
  `NoopReopenFixture` gave a false one and scored **86 passed, 3 skipped** against an
  honest fixture's **83 passed, 6 skipped** (`:1598-1611`), and the review states no
  rule on today's `Fixture` surface can reject it (`:1615`). Its own remedy is a
  CF-39-shaped clause. Requiredness compels a sentence; it does not compel a true one.
- **It is additive, so it costs the rest of the decision nothing.** Giving an existing
  required const a default is a minor change and existing impls keep compiling; nothing
  in this workspace's nine `ProjectionFixture` impls has to move. Under A there is **no
  §6.6 clause to write, no CF-31 signal to dilute, and no superseding atom needed for
  ADR-0036** — the prior question below stops gating anything.

**What A does not do, and what it leaves standing.** It does not forbid a required item
forever by fiat; it says the next capability does not get one. If a future capability
genuinely cannot be defaulted, **Option C's mechanical half remains the right
classification for it** — the release is `0.3.0` and the changelog says why — and the
brief's finding stands that CF-31's generalising sentence
(`spec/SPECIFICATION.md:8670-8674`) already covers the class, so naming it costs a
pointing clause rather than an amendment to frozen text. That is C reduced to what it
actually is after the arithmetic: a contingency, not a policy.

**The prior question, demoted.** *Does ADR-0036's `unstable-projection` exemption reach
`happenstance-testkit`'s `ProjectionFixture`?* On the documents as written it does not
(`.kb/decisions/0036-…:57-62` names `happenstance-core` and `happenstance`;
`lib.rs:350-355` makes the testkit's projection module unconditional). Under C this had
to be settled first, because it changed C's cost. **Under A it does not block**, because
A produces no semver event whose scope the answer would change. It remains an unanswered
fact-shaped question about published API and is left in "What this does not settle".

### The strongest argument against, in its own words

*The projection family's policy is well argued, the review says so, and A discards it on
the strength of a family whose capabilities happen to have been uniform.*
`contract.rs:619-627` states the case in terms A cannot answer: *"why a particular store
cannot make a commit fail"* is not the same sentence twice — *"an in-memory map applies
both halves under one lock, a one-shot HTTP backend has no interactive transaction to
abort, and a pooled adapter usually can. The cost is one line per fixture and the return
is that no fixture author is left un-asked."* Under A that property is gone: a capability
an author never mentions is declined on their behalf, in a sentence the testkit wrote,
printed in their CI log and read by their users as their store's account of itself.
CF-39's clause-level MUST is the proposed replacement, and this repository has **no
measurement of a clause's yield** — L1-2 cannot be that measurement in either
direction, and the review filing L1-2 concedes its own remedy converts a hazard only
*"from undetectable to stated"* (`:1615`). A trades a compiler for prose and calls the
trade even without evidence.

I do not think it beats A, for two reasons the tree supplies. First, the trade is not
even: A keeps the clause **and** loses only the compiler, and L1-2 is a case where the
compiler was present and produced a lie that the clause was then asked to catch anyway
— so the compiler was never the thing holding the line. Second, the projection policy
is not being weighed against nothing; it is being weighed against CF-39, which decided
this exact shape the other way, with an adapter behind it, at clause level, in the
frozen specification. A policy stated in one doc paragraph and contradicted by the
specification's treatment of its nearest neighbour is the thing that owes an argument.
But it is the argument to answer in whatever gets written, and if the owner answers it
by keeping the policy, C's mechanical half above is the classification that follows.

---

## Cost of delay

**Free today. Not free after phase 12, and the cheapness is an artefact of the
pre-release, not of the decision being reversible.**

- **No downstream can currently reach the break.** The published version is
  `0.2.0-alpha.1`, and Cargo will not resolve a pre-release without an explicit
  pre-release requirement (`RUNBOOK.md:4220-4222`); each alpha is yanked when the next
  lands (`CHANGELOG.md:309-311`). The falsifier's own manifest records the arithmetic
  from having hit it (`examples/outside-projection-adapter/Cargo.toml:21-26`):
  *"a `"0.2.0"` requirement does not match a `0.2.0-alpha.1` candidate, so the whole
  workspace stops resolving."* So today the only impls that `E0046` can reach are the
  nine in this workspace, and fixing them is nine lines by the author who wrote the const.
- **Stable `0.2.0` is the hinge, though less sharply than this brief first said.** It is
  the first number a `^0.2` requirement can resolve to at all — and then only its
  patches, which is the shape C2-06 files (`:1925`) — and it is where
  `cargo-semver-checks` first gets a registry baseline (`RUNBOOK.md:163`, `:4723`).
  Before it, the class is theory. After it, a required item added in a `0.3.0` still
  reaches nobody automatically; what reaches a stranger's build is a required item added
  in a **patch**, which CF-29 already forbids. The corrected arithmetic makes the hinge
  about baselines and documents, not about an automatic red build.
- **The policy is cheap to change at any time; the trait is not.** Prose in §6.6 can be
  rewritten in any release. A required item, once shipped, cannot be softened to a
  default without deciding what the default's reason says — which is Option A arriving
  by accident, at a worse moment, with a testkit-written sentence chosen under time
  pressure.
- **One named event will force it whether or not it is decided.** `POLL_BUDGET`
  (`.kb/open-questions/poll-count-bounds-the-visibility-rule.md`, ADR-0034 `:118-128`)
  is the next capability-shaped addition on the roadmap and ADR-0034 names it as its own
  falsification trigger. Deciding this first means `POLL_BUDGET` lands under a policy;
  deciding it second means `POLL_BUDGET`'s implementer sets the policy by what they
  happened to type.

---

## What this does not settle

- **Whether ADR-0036's gating exemption reaches `happenstance-testkit`.** Named above as
  the prior question. It is a fact-shaped question about published API and it is not
  answered anywhere in the tree.
- **~~Whether the projection family's "the fixture writes the reason" policy should hold
  at all.~~ No longer unsettled here, and this is where the flip bites.** The brief
  previously recommended a semver classification that let the policy stand while
  declining to defend it; that position did not survive the collapse of B-vs-C, because
  once the classification buys nothing the policy is the only thing left being bought.
  The recommendation now **takes the position**: retract the trait-level rule, keep the
  reason as a CF-39-shaped clause. C2-04 (`:1722`) is still right that the trade *"is
  the owner's"* — this is a brief's recommendation on it, not a decision.
- **The falsifier's evidence on the policy, corrected and now load-bearing rather than
  offered in passing.** Three declensions are written out-of-tree under the policy.
  `OutsideFixture`'s `RESET_REFUSAL` is rich and reads its reason off its own store's
  const (`mod.rs:50-51` → `src/lib.rs:135-138`). `CheckpointOnlyFixture`'s
  `RESET_REFUSAL` reads `OutsideProjectionStore::NO_RESET_PROTECTION` — a **different
  store's** const (`mod.rs:90-91`) — and its `COMMIT_FAULT` is content-free,
  *"this store cannot be made to report a failed commit"* (`mod.rs:93-94`), which says
  *that* it cannot and not *why*, the exact bar the ladybug story sets
  (`.bklg/…/ladybug-fixture-and-conformance-run/spec.md:324`). The caveat that keeps
  this from being decisive is unchanged: that fixture is a deliberately-wrong store's
  control, so its author had the least incentive of anyone to write a rich reason. Two
  data points, stated, weighted lightly.
- **CF-18's status under any of these options is assumed, not re-derived.** Option D is
  rejected here on CF-18's frozen text; if the owner is willing to reopen CF-18, D
  becomes available and this brief has not evaluated it on that footing.
- **Whether the two fixture families should share one policy at all.** The brief now
  recommends one *declension* policy for both, which is a stronger claim than the
  version it replaced. The defensible position it argues against — `Fixture` defaults
  and `ProjectionFixture` requires, under one shared semver rule — is not refuted here;
  it is out-weighed by CF-39 having decided the nearest capability the other way, and
  an owner who reads the projection family's asymmetry as principled rather than
  accidental can keep it and take C's mechanical half instead.
- **Who owns the clause.** ADR-0034 (`:86-96`) is explicit that the fixture contract has
  no single owning document and that a `CF-` clause is minted by whichever decision first
  needs the capability. So the §6.6 clause C2-04 asks for has no standing owner by
  construction, and C2-06 (`:1929`) records the matching discipline: *"ADR and clause
  authorship belong to the runbook's own pass, not to a review side effect."*
- **The clause's own enforcement.** Nothing in `cargo xtask ci` would catch a required
  item shipping in a patch. C2-06's proposed rule-set baseline would not either — it
  diffs rule names (`xtask/src/lints.rs:527-616`). The only instrument that sees this
  class is `cargo-semver-checks` against a registry baseline, which does not exist until
  phase 12.
- **L1-2 and L3-01 both land under whatever this settles and neither is decided here.**
  L3-01's `READ_FAULT` is proposed as defaulted and would be additive under every option
  above; L1-2's `REOPEN` obligation adds no item at all. L1-2 is now cited **in support
  of** the recommendation — as the measurement of what requiredness fails to buy — and
  that is a use of its evidence, not a decision on its routing, which the review assigns
  to whoever owns CF-17's `[PROVISIONAL]` marker (`:1615`). Neither is authorised by
  this brief.

---

## Revision record

Two critiques were filed against the first draft and neither accepted its
recommendation. Both were checked against the repository at this commit before anything
here moved; both held. What changed, and why.

**1. A falsified premise about Cargo's caret at `0.x`, and everything resting on it.**

The draft asserted that `happenstance-testkit = "0.2"` (`lib.rs:205`) is *"a caret that
takes the next minor automatically."* False. The crate is pre-1.0
(`crates/happenstance-testkit/Cargo.toml:21`), so `^0.2` is `>=0.2.0, <0.3.0` — patches
only. The draft's own cited source says so at the exact line it cited: C2-06 at `:1925`
reads *"a caret that takes the **patch** automatically."* **Removed rather than
rewritten around**, in four places:

- the version-policy paragraph's claim itself, replaced by the corrected arithmetic;
- **Option B's only stated cost to an adapter author** — a red build on `cargo update`
  — which cannot occur, because B's release is `0.3.0` too and `^0.2` will not resolve
  it. It occurs only for an item shipped in a *patch*, which CF-29's `Rejects:` already
  forbids (`spec/SPECIFICATION.md:8649-8653`);
- **Option B's semver-suppression cost and its foreclosure** — `cargo-semver-checks`
  applies `0.x` rules, so a `0.2 → 0.3` bump satisfies a major-category change and there
  is nothing to suppress and no gate authority lost. True only after `1.0`;
- **Option C's "Forecloses: part of CF-31's current signal"**, and with it the draft's
  former "strongest argument against", which was that claim in longer form. At `0.x` a
  rule addition already consumes the incompatible bump under CF-29, and
  `Cargo.toml:10-13` records `0.1 → 0.2` spent on *"thirty-four rules"* — so there is no
  undiluted CF-31 signal left for C to dilute.

The "Cost of delay" hinge bullet was corrected to match: `0.2.0` matters for baselines
and documents, not for an automatic red build.

**2. A falsified premise inside Option A's foreclosure, which had L1-2 backwards.**

The draft used L1-2 to price a CF-39-shaped clause as *"prose with no compiler behind
it … A clause alone did not stop it."* `REOPEN` is a **required** const
(`contract.rs:184`). The compiler forced an answer and the answer was a lie, and the
review states no rule on today's `Fixture` surface can reject `NoopReopenFixture`
(`:1615`). L1-2 measures **requiredness** failing, not a clause failing, and the
review's own remedy for it is a CF-39-shaped clause. **Removed**, with nothing put in
its place: the repository holds no measurement of a clause's yield in either direction,
and saying so is more honest than a weaker version of the deleted claim.

**3. The recommendation flipped, from Option C to Option A.**

Two findings did it, in this order.

- *The B-vs-C collapse (from critique 2).* At `0.x`, B and C are the same release event.
  C's headline reason in the draft — *"B keeps the policy by asking the version number
  to lie"* — has no referent, and C's advantage over B is prose until `1.0`.
- *The unsupported residual (from critique 1).* With the mechanical difference gone,
  C's entire remaining cost is paid to protect the projection family's declension
  policy — which the draft explicitly declined to defend, in its own "What this does
  not settle". Recommending a cost whose only return is a position the brief refuses to
  take is not a defensible shape, so the policy had to be settled first.

Settling it moved the answer, because the draft had priced Option A against the wrong
clause. It cited **CF-40**, which governs *facts* (`Option<usize>` ceilings, which
CF-40's own text says *"MUST NOT be spelled as `Capability`"*). The on-point precedent
for a *trade* is **CF-39**: `MID_BATCH_FAULT` is defaulted, its default reason is
testkit-written in the fixture's own voice (`contract.rs:218-222`), and CF-39 recovers
the honesty as a clause-level MUST. That is Option A, already shipped and already in
the frozen specification, for the capability closest in shape to `COMMIT_FAULT`.
Against it, the projection family's rule is one documentation paragraph with no adapter
behind it, and its measured out-of-tree yield is two poor declensions out of three (one
borrowed from a *different* store's const, one content-free) — a fact the draft had
already recorded but weighted as a passing caveat rather than as evidence.

Option C was not deleted. Its mechanical half is retained inside the recommendation as
the classification that applies **if** a future capability genuinely cannot be
defaulted, together with the draft's surviving finding that CF-31's generalising
sentence already covers the class.

**4. Consequential edits, all downstream of the above and none independent of it.**

Option A's cost bullets were re-priced against CF-39; the prior question about
ADR-0036's exemption was demoted from blocking to open, because Option A produces no
semver event whose scope the answer would change; the "What this does not settle"
entries on the policy, on the falsifier's evidence, on family uniformity and on L1-2's
routing were updated to say what the flipped recommendation does and does not claim.
The strongest argument against is now the projection family's own case for the policy,
quoted from `contract.rs:619-627`, with the answer to it stated and the residual
conceded: this repository has no measurement of a clause's yield, and A trades a
compiler for prose on the strength of an argument rather than a number.

**What did not change.** The problem statement, the measured `E0046` / `E0119` probes,
the `ConcurrentFixture` closure finding, the blast-radius count of nine impls, Option D's
rejection on CF-18's frozen text, and the cost-of-delay conclusion that the class is
free to decide today and forced by `POLL_BUDGET` whether or not it is decided.
