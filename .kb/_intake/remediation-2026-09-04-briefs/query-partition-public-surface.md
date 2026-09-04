# Once the query partition is two ceilings, does `MAX_QUERY_ARMS_PER_STATEMENT` stay public, and what does `planned_statement_count` count?

**Record id:** `sqlite-query-partition-surface`
**Would supersede:** nothing. ADR-0022 (`.kb/decisions/0022-append-condition-strategy.md`, `status: accepted`) owns the append-condition SQL strategy and is untouched by every option below: this is about two names on `happenstance-sqlite`'s public surface, not about what SQL either of them describes.

**This brief did not get the two-critic pass the original thirteen had.** It was written by the lane implementing audit entry `X-1`, in the same session as the change it describes, and nobody independent argued the other side. `README.md`'s discount applies. Each option carries its own strongest objection and an answer, which is the form, not a substitute.

---

## Why this is owed

`X-1` (`references/evaluation/review-pre-publication-2026-09-03.md:1036-1080`) routed exactly one of its two halves as a decision and named it: *"whether `MAX_QUERY_ARMS_PER_STATEMENT` stays public once it is no longer the partition"*, and *"whether `planned_statement_count` continues to mean 'arms' or is repointed at the real partition"*. Both are public-surface consequences of a fix that is otherwise mechanical, and both are free today and frozen at `0.2.0` — `happenstance-sqlite` is a release-bound crate that is **not yet on crates.io**.

The lane took the second decision under duress, because the honest fix required it. This brief says so loudly rather than presenting it as settled.

---

## What is true today, after the lane

Two `pub const`s on `SqliteEventStore`, and one `pub fn` reading both.

`crates/happenstance-sqlite/src/event_store.rs:281`:

```rust
    pub const MAX_QUERY_ARMS_PER_STATEMENT: usize = 400;
```

`crates/happenstance-sqlite/src/event_store.rs:308` — **new**:

```rust
    pub const MAX_QUERY_PARAMETERS_PER_STATEMENT: usize = PARAMETER_BUDGET;
```

`crates/happenstance-sqlite/src/event_store.rs:326-334` — **changed in what it counts, not in its signature**:

```rust
    pub fn planned_statement_count(query: &Query) -> usize {
        crate::query_sql::chunks(
            query,
            &Selectivity::default(),
            Self::MAX_QUERY_ARMS_PER_STATEMENT,
            Self::MAX_QUERY_PARAMETERS_PER_STATEMENT,
        )
        .len()
    }
```

The partition is greedy and order-preserving over both limits (`crates/happenstance-sqlite/src/query_sql.rs:249-269`), and the per-item cost is one bound parameter per distinct tag and one per type (`:287-289`, a second reading of `item_sql` at `:316`).

### The two axes are genuinely independent, which is the whole reason there are two numbers

400 items of one tag each is 400 arms and 400 parameters. The same 400 items at `MAX_TAGS_PER_EVENT` tags apiece is still 400 arms and **51,200** parameters, against `SQLITE_MAX_VARIABLE_NUMBER`'s 32,766. Before the lane, `planned_statement_count` reported that second query as a plan of `1`, and `prepare` refused the statement it counted. That is `experiments/shipped-append-condition-sql/results/selectivity.md:52-55`, measured, and `tests/selectivity_cost.rs:208` asserts it against the shipped public function.

### What the arm width still does

It is not decorative and it is not subsumed. At one tag per item — the shape `tests/wide_query.rs` builds, and the shape a consistency boundary usually has — 900 items is 900 parameters, nowhere near 30,000, and the arm limit is the only thing that splits the query. `tests/wide_tags.rs::the_arm_partition_still_binds_on_narrow_items` is the standing guard for exactly that: the regression it rejects is a partition that swapped one limit for the other rather than taking both.

So the premise inside `X-1`'s first question — *"once it is no longer the partition"* — is **half false as landed**, and this brief's recommendation turns on that. It is no longer the *whole* partition. It is still one of the two things the partition is.

---

## Question 1 — does `MAX_QUERY_ARMS_PER_STATEMENT` stay public?

### Option 1A — both constants public (as landed)

- **Costs a caller:** two numbers to read instead of one, and a caller who wants "will my query fit in one statement" must do arithmetic over both. Nothing forces them to: `planned_statement_count` answers that question directly.
- **Costs an adapter author:** nothing. Neither constant is on any port.
- **Semver:** additive today, frozen at `0.2.0`. Two numbers frozen instead of one.
- **What it buys:** a test can compute the boundary rather than guess at it, which is the reason the arm width was made public in the first place (`event_store.rs:274-276`) and the reason this repository distrusts a merge nothing crosses. `tests/wide_tags.rs` computes its under/at/over cases from `MAX_QUERY_PARAMETERS_PER_STATEMENT` and its narrow-item case from `MAX_QUERY_ARMS_PER_STATEMENT`; with either private, one of those tests goes back to a literal.

### Option 1B — both private, `planned_statement_count` the only public seam

- **Costs a caller:** they can still ask how many statements a query will take, but not why, and not at what width. A caller sizing a decision model against the adapter — the workload `spec/SPECIFICATION.md`'s VT-23 exists for — gets a number with no gradient: it cannot compute how much room it has left.
- **Costs a test author:** the boundary tests must binary-search `planned_statement_count` or hard-code a literal. A literal is what `tests/wide_query.rs:42-46` explicitly refuses, in a comment, for the reason that changing the width would silently stop the file crossing the boundary.
- **Semver:** removing `MAX_QUERY_ARMS_PER_STATEMENT` is breaking *after* `0.2.0` and free before it.
- **What it buys:** one public number instead of three, and freedom to change the partition strategy later — for instance, if the parameter budget stops being a single flat number because an arm shape with a different per-item cost lands under `append-condition-sql-shape.md`.

### Option 1C — a single public accessor over both, the constants private

Something like `pub const fn query_ceilings() -> (usize, usize)`, or a small `QueryCeilings` struct on the model of `happenstance-cloudflare`'s `Ceilings` (`crates/happenstance-cloudflare/src/event_store.rs:228-236`).

- **Costs a caller:** an unfamiliar shape for two integers.
- **Costs the workspace:** it invents a third spelling of "declared ceiling" beside `StoreLimit` and `Ceilings`, in a crate that already has neither.
- **Semver:** the struct is the frozen thing rather than the numbers, so a third axis can be added later without a break — which is the one real argument for it.
- **What it buys:** room for a third axis. There is no third axis in evidence.

### Recommendation on question 1: **1A**, and the strongest argument against it

Keep both public. The arm limit did not stop being a real limit; it stopped being the only one, and a caller who cannot see it cannot tell a narrow-item split from a wide-item one.

**The strongest argument against, stated properly:** every public constant is a promise about an implementation detail, and this one is a promise about *SQLite's* compile-time configuration. `SQLITE_MAX_COMPOUND_SELECT` and `SQLITE_MAX_VARIABLE_NUMBER` are both build-time constants of the bundled library (`results/selectivity.md:88-92`), so both numbers are true of *this* build of `rusqlite` and not of SQLite in general. Freezing them at `0.2.0` freezes a fact about a dependency's build configuration onto this crate's public surface, and the honest consequence is that a future `rusqlite` with a raised limit cannot be exploited without a major. That argument applies with equal force to the constant that is *already* public, which is why it is an argument for 1B over the status quo ante rather than an argument created by this lane.

**Confidence: medium.** The case rests on tests being able to compute their boundary, which is a strong local argument and a weak general one.

---

## Question 2 — what does `planned_statement_count` count?

### Option 2A — the real partition (as landed)

The number is the count of statements that will actually be prepared and run.

- **Costs a caller:** any caller pinning the old number for a wide-tag query sees it change. That caller was pinning a plan the driver refuses; there is no such caller in the workspace, and no such caller anywhere, because the crate is unpublished.
- **Semver:** none as a signature. A behavioural change to a public function, free today.
- **Against it:** the function's own doc used to say it is *"the same call the read path makes"* — and it still is. Under 2A that sentence stays true, which is the property the doc was written to protect (`event_store.rs:310-317`): a second `ceil(arms / width)` beside it would agree by arithmetic rather than by construction, and would go on reporting a boundary the read path had stopped taking.

### Option 2B — keep it meaning "arms", add a second function

`planned_statement_count` keeps counting the arm partition; something like `planned_parameter_count` or `planned_statement_count_exact` reports the truth.

- **Costs a caller:** two functions, one of which is documented as not answering the question its name asks. The number `planned_statement_count` would return is not the number of statements.
- **Semver:** additive.
- **Against it:** it preserves a name that describes an abandoned plan, which is `X-1`'s own defect one level up. This repository has a recorded instance of exactly that shape — `query_sql.rs`'s module doc claimed both callers went through one entry point while the append path did not (`crates/happenstance-sqlite/src/query_sql.rs:6-10`) — and the note there exists so that nobody does it again.

### Option 2C — remove the function

- **Costs a test author:** the boundary becomes unobservable from an integration test, which is the situation `tests/wide_query.rs:15-18` was designed to escape: it is an integration target *on purpose*, so it can only reach public API, and the boundary observation *"had to be designed rather than smuggled in behind `#[doc(hidden)]` or `cfg(test)`"*.
- **Semver:** breaking after `0.2.0`, free before.
- **Against it:** it removes the only seam by which a merge that never executes can be caught, in a crate whose own tests say that is the failure mode the project exists to retire.

### Recommendation on question 2: **2A**, which is what landed

The name says "statements"; make it count statements. The lane implemented this rather than briefing it because the alternative was to leave a public function returning a number known to be wrong in the exact case the finding is about — and the changelog entry says so in those words.

**The strongest argument against, and it is not weak:** the decision was taken by the lane rather than by the owner of the public surface, and it is the one thing in `X-1` that could not be deferred without leaving the defect half-fixed. If the owner prefers 2B, the change is small and free until `0.2.0`. What cannot be recovered cheaply is 2A landing *after* publication.

**Confidence: high** on the direction, **medium** on the authority.

---

## Cost of delay

`0.2.0`, for all of it. Both constants and the function's meaning become promises at first publish. Before then every option above is a small edit; afterwards 1B and 2C are majors and 2B is a permanent second name.

There is no earlier deadline. Nothing in the gate depends on either answer, and `tests/wide_tags.rs` continues to pass under 1A/2A without modification.

---

## What this does not settle

- **The testkit's VT-23 rule.** `store_evaluates_a_query_at_the_guaranteed_minimum_item_count` (`crates/happenstance-testkit/src/suite.rs:3899-3903`) builds 128 items carrying **one tag each** — 128 parameters — while its own assertion message speaks of *"an adapter that sends only its first chunk of bound parameters"*. The rule names the unit it is not measuring in. Widening it to cross the tag axis is a testkit change, which per CF-29 is a minor every adapter takes involuntarily, so it is the same release-timing question as the fixture wave; it was deliberately out of the `X-1` lane's writable surface and is not proposed here.
- **Whether the two shipping adapters must agree on a declared query ceiling.** That is `X-2`'s, and it belongs with VT-23's own `[PROVISIONAL]` marker.
- **The per-item parameter cost's coupling to the arm shape.** `item_parameters` (`crates/happenstance-sqlite/src/query_sql.rs:287-289`) is a second reading of `item_sql`, and it is correct only for the intersection-chain arm that ships today. If `append-condition-sql-shape.md` chooses an aggregate arm, whoever lands it owns re-deriving that function — the count is *"one per tag and one per type"* under both shapes measured so far, but that is a fact about two shapes, not a theorem.
- **`Selectivity::read_for`'s existence.** `append-condition-sql-shape.md`'s option B deletes it, and with it the chunk width the `X-1` lane gave it. That brief asked for `X-1` to be sequenced after it and `X-1` ran first; the note recording that is in its §"Sequencing".
- **The quadratic in `read_for`'s accumulation.** Audit entry `I-5` owns it, the lane left `wanted.contains` exactly as it found it, and chunking the statement does not make the accumulation cheaper. The two changes touch the same function and will conflict if they land in either order without a merge.
