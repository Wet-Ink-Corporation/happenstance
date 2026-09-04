# Does the shipped multi-tag guard stay an intersection chain, and does most-selective-first survive its measured 38–44x cost?

**Record id:** `ADR-0022-s8-query-shape`
**Would supersede:** `kb-decision-0022` (`.kb/decisions/0022-append-condition-strategy.md`), **in part** — its items 1 and 2 of §8 only. That atom is `status: accepted` (`:5`), so it is immutable and this is a new superseding atom, never an edit. Nothing else in ADR-0022 is in scope: §4's `max(position)` strategy, §6's tag storage, §9's runtime seam and §11's pragmas are all untouched by every option below.

---

## Why this is owed

**1. ADR-0022 §8's own re-opening condition can never fire.** §16's falsifier for §8 is, verbatim (`references/adr/0022-append-condition-strategy.md:606-607`):

> **§8, the fast path.** Re-open it if a future SQLite pushes predicates through an aggregate, at which point the special case stops earning its branch.

There is no aggregate in the shipped multi-tag path to push a predicate through. A decision whose stated re-opening condition names a shape the code does not have has stopped being falsifiable by the mechanism it declared. That is the discriminator audit entry **I-1** names, and it is the reason this is a record rather than a documentation pass.

**2. Every number that prices the shipped shape was measured on a different shape.** §8's `1,093 µs → 556` and its `roughly 200x` are figures from `GROUP BY position HAVING COUNT(DISTINCT tag) = n` (`references/adr/0022-append-condition-strategy.md:355-372`). Those figures are quoted at four sites to justify the chain — three of them rendered:

| site | rendered? | what it quotes |
| --- | --- | --- |
| `crates/happenstance-sqlite/src/lib.rs:58-59` | yes (crate root) | "a `tag_cardinality` table supplying the per-value selectivity SQLite's `ANALYZE` cannot" |
| `crates/happenstance-sqlite/src/event_store.rs:91-96` | yes (`pub mod event_store`, `lib.rs:86`) | "must be probed most-selective-tag-first … roughly 200x a single-tag one (ADR-0022 §8)" |
| `crates/happenstance-sqlite/src/query_sql.rs:76-83` | **no** (`mod query_sql`, `lib.rs:89`) | "keeps the boundary pushable for the same reason … roughly 200x" |
| `spec/SPECIFICATION.md:3902-3905` | yes, and **`[FROZEN]`** (`:9104`) | "measured at roughly 200x … which is why ADR-0022 ships `tag_cardinality` and most-selective-tag-first probing as requirements" |

**3. The measurement that settles it now exists and is reproducible.** `experiments/shipped-append-condition-sql/` builds four guard shapes on one schema, clears the conformance suite on all four (`results/raw/conformance.txt`: `356 passed; 0 failed`, four arms × 89 rules), and checks its own transcription byte-for-byte against a `sqlite3_trace_v2` callback on a real `SqliteEventStore` (`results/raw/emitted-sql.txt:16`, `TRANSCRIPTION chain matches_adapter=true`). It is out of the gate by construction (`run.sh:12-16`).

---

## What is true today

### The SQL the adapter actually emits

Off the trace callback on a real store, `results/raw/emitted-sql.txt:9`:

```sql
SELECT max(position) FROM (SELECT position FROM event_tag WHERE tag = ? AND event_type IN (?) AND position IN (SELECT position FROM event_tag WHERE tag = ?))
```

The builder, `crates/happenstance-sqlite/src/query_sql.rs:340-354`:

```rust
    let mut sql = String::from("SELECT position FROM event_tag WHERE tag = ?");
    params.push(Value::Text(tags[0].clone()));
    if !types.is_empty() {
        for event_type in types {
            params.push(Value::Text(event_type.as_str().to_owned()));
        }
        sql.push_str(" AND event_type IN (");
        sql.push_str(&placeholders(types.len()));
        sql.push(')');
    }
    for tag in &tags[1..] {
        params.push(Value::Text(tag.clone()));
        sql.push_str(" AND position IN (SELECT position FROM event_tag WHERE tag = ?)");
    }
    sql
```

No aggregate anywhere, and no `position > ?`.

### The guard path that wraps it

`crates/happenstance-sqlite/src/event_store.rs:709-723`:

```rust
            let chunk: Option<i64> = connection.query_row(
                &format!("SELECT max(position) FROM ({matched})"),
                rusqlite::params_from_iter(params.iter()),
                |row| row.get(0),
            )?;
            highest = highest.max(chunk);
        }

        let boundary = guard.after.map_or(0, as_i64);
        if let Some(highest) = highest
            && highest > boundary
```

The boundary is compared in Rust, after the statement has returned. `chunks` takes no boundary parameter on either caller (`query_sql.rs:218-223`). This is **correct** — ES-26 fixes the semantics, not where they are evaluated — and it is what makes `query_sql.rs:77`'s "keeps the boundary pushable" a claim about a mechanism the shipped strategy never exercises.

### The ordering, and its two crate-private pieces

`crates/happenstance-sqlite/src/query_sql.rs:161-164`:

```rust
    fn most_selective_first(&self, mut tags: Vec<String>) -> Vec<String> {
        tags.sort_by_key(|tag| self.0.get(tag).copied().unwrap_or(0));
        tags
    }
```

`Selectivity` is `pub(crate)` (`:97`), `most_selective_first` is private (`:161`), and `query_sql` itself is a private module (`lib.rs:89`). **`:162`'s `unwrap_or(0)`** means a tag with no `tag_cardinality` row sorts as maximally selective and lands in the seed arm — which is the *cheap* side under the shipped ordering and, under any inverted policy, becomes the expensive side. The experiment never exercised it (both its tags are present), and its own `inverted()` doc (`experiments/shipped-append-condition-sql/src/chain.rs:212-217`) claims an absent tag "still sorts first" where the arithmetic it describes — present counts negated to negative, absent mapped to `0`, sorted ascending — puts it last. Worth checking before any option that keeps a sort.

The table behind the sort is maintained on every write, inside the transaction (`event_store.rs:866-876`):

```rust
        "INSERT INTO tag_cardinality (tag, events) VALUES (?, 1) \
         ON CONFLICT(tag) DO UPDATE SET events = events + 1",
```

one execution per tag per event. At the crate's declared ceilings — `MAX_TAGS_PER_EVENT = 128` (`:252`), `MAX_EVENTS_PER_BATCH = 256` (`:261`) — that is up to 32,768 upserts inside `BEGIN IMMEDIATE`. The crate already prices tags in exactly this unit: *"Every tag costs a row in `event_tag` and an upsert in `tag_cardinality`, both inside the write transaction, so the number is bounded by lock hold time rather than by storage"* (`:249-251`). **Nothing in the experiment measures that cost** — all four arms share one schema and all maintain the table identically.

### ADR-0022 §8, items 1 and 2, verbatim

`references/adr/0022-append-condition-strategy.md:375-381`, accepted and immutable:

> 1. **`tag_cardinality` is a requirement, not a convenience.** Multi-tag arms must
>    be probed **most-selective-tag-first**, and SQLite cannot supply per-value
>    cardinality on its own: `ANALYZE` stores an *average*, which is exactly wrong
>    for a tag set where one value matches a third of the log and another matches
>    one percent.
> 2. The 200x is measured **with `event_type` already covering**, so it is a floor
>    on the multi-tag path rather than evidence against §7's correction.

### The measurements, re-derived from the raw rather than the summary

**Chain against aggregate — 9 of 9, verified.** `guard_us_median` from `results/raw/guard-cost.txt`, `chain-as-shipped ÷ grouped-adr0022`:

| scenario | events | chain µs | grouped µs | ratio | `chain-bounded-all-arms` µs |
| --- | ---: | ---: | ---: | ---: | ---: |
| accepted-2tag-at-head | 50,000 | 29,845 | 19,178 | **1.56** | 124 |
| accepted-2tag-at-head | 500,000 | 296,229 | 185,224 | **1.60** | 143 |
| accepted-2tag-at-head | 1,000,000 | 559,591 | 362,385 | **1.54** | 179 |
| rejected-2tag-unbounded | 50,000 | 25,204 | 14,085 | **1.79** | 25,366 |
| rejected-2tag-unbounded | 500,000 | 373,858 | 231,501 | **1.61** | 374,662 |
| rejected-2tag-unbounded | 1,000,000 | 545,830 | 349,805 | **1.56** | 534,230 |
| rejected-2tag-midlog | 50,000 | 31,487 | 18,015 | **1.75** | 15,442 |
| rejected-2tag-midlog | 500,000 | 338,431 | 214,040 | **1.58** | 169,587 |
| rejected-2tag-midlog | 1,000,000 | 593,093 | 319,306 | **1.86** | 254,482 |

The chain loses to the aggregate in all nine, one direction. **It also never beats `chain-bounded-all-arms`**: worse in six cells and inside band in the three `boundary=0` cells. The shipped shape is dominated by both alternatives, in every cell measured.

**The single-tag path is untouched by any of this.** `guard_us_median` for `accepted-1tag-at-head` and `rejected-1tag-unbounded` is 4–16 µs at every size on all four shapes (`guard-cost.txt:8-11,16-19,44-47,52-55,80-83,88-91`). The aggregate arm keeps §7/§8's single-tag fast path — `experiments/shipped-append-condition-sql/src/chain.rs:442-444` returns the bare seek for `tags.len() == 1` — so adopting it does not cost the correction §8 bought.

**Seed ordering — 38.1x to 44.0x, verified across two runs.** One 500,000-event store, one connection, **one statement text** with only the bound parameters swapped, alternating running order, every round asserting both orderings return the identical position (`results/raw/seed-ordering.txt:8-19`, `seed-ordering-replication.txt:7-18`):

| scenario | most-selective-first µs | least-selective-first µs | ratio | replication |
| --- | ---: | ---: | ---: | ---: |
| accepted-2tag-at-head | 511,054 | 11,839 | **43.2x** | 39.5x |
| rejected-2tag-unbounded | 553,328 | 13,340 | **41.5x** | 41.5x |
| rejected-2tag-midlog | 634,239 | 14,399 | **44.0x** | 38.1x |

The mechanism, from `results/raw/query-plans.txt:26-28`: the chained subquery is uncorrelated, so SQLite materialises it as a `LIST SUBQUERY 1`, and the shipped ordering puts the *larger* set there — `shard:cold` at 500,000 against `row:r7` at 5,155 (`seed-ordering.txt:7`), 97x of predicted work for 38–44x of measured time. **The policy's sign is inverted on the shape that ships.**

**Cold cost, the figure an application meets after a restart.** `query-plans.txt:29,54` at 10^6 events, boundary 500,000: chain `first_us=3,971,425` / `second_us=774,290`; aggregate `first_us=335,498` / `second_us=296,556`. **11.8x cold, 2.6x warm** in the same run. The plans say why — the chain's `LIST SUBQUERY 1` materialises the whole of the least-selective tag; the aggregate compiles to a `CO-ROUTINE` and materialises nothing into a list (`:27-28` against `:50-53`).

### What the measurements do not support

**The remedy audit entry I-2 proposed is falsified, and this brief does not resurrect it.** Correction **C-5** (`references/evaluation/review-pre-publication-2026-09-03.md:491-498`) records it: binding `position > ?` into the guard's *seed* arm costs **+2.0%** at 10^6 events on `accepted-2tag-at-head` (570,741 against 559,591, `guard-cost.txt:84-85`), with deltas of +8.8%, −4.5%, +2.0%, −3.2%, +2.6%, +1.6%, −7.2%, −2.9%, −7.4% across nine cells — no consistent sign, none outside its own spread. `EXPLAIN QUERY PLAN` shows the two plans identical line for line (`query-plans.txt:26-28` against `:34-36`): the seed is the **probe** side, and a predicate there lands on the arm already being seeked one row at a time rather than on the list it would have to shrink. No option below proposes it.

**The absolutes are warm-cache, one host, two tags.** The experiment says so itself (`README.md:217-260`): the 50,000-event calibration did not reproduce §1's 42,399 µs (14.1 ms here, 3.0x less, for page-cache reasons it sets out); the seeded log carries two tags per event and one event type; `row:r7` is spread uniformly and a clustered tag would change the page count; and one `APPEND` row (`rejected-2tag-midlog` at 500,000) is flagged bimodal and carrying no claim. **The ratios carry; the absolutes do not.**

### ES-27 is `[FROZEN]`, and its number is a clause edit

`spec/SPECIFICATION.md:3902-3905` is the `Rejects:` prose of ES-27, whose maturity row reads `| ES-27 | FROZEN |` at `:9104`:

> measured at roughly 200x a single-tag boundary's cost at 50,000 events, which is why ADR-0022 ships `tag_cardinality` and most-selective-tag-first probing as requirements

Two things are now wrong with it: the ratio, and the "which is why". **State this plainly to whoever executes: repairing a number inside a `[FROZEN]` clause is a clause edit, not a documentation pass.** CLAUDE.md's own rule — *"Changing a `[FROZEN]` clause requires a new ADR, not an edit"* — routes it through this record and not through a docs sweep. The clause's **normative** content ("a condition matches on tags, not only on types") is untouched by every option here; only the non-normative `Rejects:` prose moves.

And a caution on how it moves. On this host the like-for-like derived figure for the shipped chain is roughly **2,300x** (25,204 µs against 11 µs, `guard-cost.txt:16,20` — a ratio computed across two cells of one interleaved run and *not* asserted by the experiment), and for the aggregate roughly 1,400x. The 200x came from ADR-0022 §1's 42,399 µs against ~213 µs under different cache conditions. **The stable fact is the order-of-magnitude gap, not any particular multiple.** Replacing "200x" with "2,300x" re-freezes a warm-cache one-host derived number into a clause and buys the same rot again; the honest repair states the gap without pinning a figure, and cites the experiment for the figures.

---

## Options

### A — Keep the chain; correct only the prose

Repair the four sites to say what was measured on what, and leave the emitted SQL and the sort alone.

- **Costs a caller:** the 1.54x–1.86x warm and 11.8x cold penalty stands, inside `BEGIN IMMEDIATE` with every other writer queued behind it — 296 ms at 500,000 events, 560 ms at 10^6, 3.97 s on the first execution after a restart. Plus the 38–44x ordering penalty, which the prose would now have to describe rather than prescribe.
- **Costs an adapter author:** nothing new. But `event_store.rs:91-96` and `lib.rs:58-59` currently teach a policy that is inverted on the shipped shape, and a corrected version of them has to say "we ship a requirement we measured wrong and kept" — which is a worse sentence to render than either fix.
- **Semver:** none as API — `query_sql` is private and the statement text is not a signature. **The clause "`happenstance-sqlite` is unpublished" has been removed from this line as false:** the crate is live on crates.io at `0.0.0`, created and last updated 2026-08-18, 16 downloads. The `0.0.0` row is a name reservation carrying no schema, so nothing about the API argument changes — but the claim itself was untrue and is not repaired by rewording.
- **Forecloses:** nothing technically, but it leaves §16's falsifier for §8 permanently unreachable and hands the same question to the next reviewer with a bigger evidence pile and no new instrument.

### B — Adopt the aggregate for multi-tag items, keeping the single-tag fast path

Replace the chain with `GROUP BY position HAVING COUNT(DISTINCT tag) = n` — the form §8 measured — retaining the `tags.len() == 1` seek. This is `grouped-adr0022`, already built and conformance-cleared.

- **Costs a caller:** nothing on the single-tag path (4–16 µs, unchanged). On the multi-tag path it is 1.54x–1.86x cheaper warm and 11.8x cheaper cold than today. It is **not** cheap: 362 ms at 10^6 events with the write lock held.
- **Costs an adapter author:** ~~it changes one string in a private module~~ — **that claim is removed, because the code refutes it in two places.** `chunks` is not one string with one caller: it is reached from the guard path inside `BEGIN IMMEDIATE` (`event_store.rs:699`, with `Selectivity::read_for` at `:647`) *and* from the read path (`:1313`, `read_for` at `:1301`), so B moves both. And shipped `item_sql` (`query_sql.rs:316-354`) has **no `tags.len() == 1` branch at all** — the single-tag case is the chain's degenerate form, the seed arm with an empty `tags[1..]`. `grouped_sql` has to spell that branch explicitly (`chain.rs:441-443`), so B *adds* a branch rather than retaining one. `evaluate`'s Rust-side boundary comparison is untouched, so no correctness surface moves. `Selectivity`, `most_selective_first` and `tag_cardinality` lose their only consumer — `grouped_sql` takes no selectivity argument at all (`chain.rs:314,412-414`), because `tag IN (?,?)` is order-independent. That is a deletion, not a repair, and it takes the per-tag upsert out of the write transaction.
- **Semver:** none as API. **The schema half is not free forever** — dropping `tag_cardinality` from `MIGRATION_1` (`event_store.rs:221`) is a `SCHEMA_VERSION = 1` edit today and a migration 2 after `0.2.0`.
- **Forecloses:** predicate pushdown, by construction and by §8's own words — *"A `GROUP BY` is an optimisation barrier"*. The shape with the largest measured headroom (option D) becomes unreachable without reversing this.

### C — Keep the chain, invert the sort

Put the *least* selective tag in the seed arm.

- **Costs a caller:** 38–44x on the two-tag case, in the good direction — 11,839 µs against 511,054 at 500,000 events. That is by far the largest single-change win on the table.
- **Costs an adapter author:** it keeps `tag_cardinality` and its write-path upsert, and inverts a documented requirement without a policy to replace it with.
- **Semver:** none.
- **Forecloses:** little, but **the evidence does not support it as a general policy and the experiment says so in terms** (`README.md:245-248`): with three or more tags the plan has more than one materialisation to choose between and the single-term arithmetic no longer holds. This is a two-tag answer to an n-tag policy. It also puts `query_sql.rs:162`'s `unwrap_or(0)` on the wrong side, un-analysed.

### D — `chain-bounded-all-arms`: bind the boundary into every arm

The chain with `position > ?` on the seed *and* inside each chained membership subquery.

- **Costs a caller:** the biggest win anywhere in the data, and only where the boundary discards work — 179 µs against the aggregate's 362,385 at 10^6 on `accepted-2tag-at-head`, **2,024x**, which is the commonest DCB path (read to head, append with `after = head`). It **loses** to the aggregate at `boundary = 0` (534,230 against 349,805) and wins 1.25x at mid-log. **Cold, it also beats the aggregate**: the same plan dump that supplies B's cold figure has D at `first_us=236,988` against B's `first_us=335,498` and B's *warm* `second_us=296,556` (`query-plans.txt:37-45,46-54`). B's cold advantage is priced against the chain only; against D it does not exist.
- **Costs an adapter author:** ~~the most of any option~~ — **the claim that D moves a correctness surface is removed, because the built arm refutes it.** `probe_store.rs:300-303` states the mechanism: "**The Rust-side comparison stays where it is for every shape.** A bounded shape returns only positions already above the boundary, so `highest > boundary` is then trivially true for any row it returns and false for the `NULL` it returns otherwise — the two spellings agree by construction, and no shape can win by deciding less." `evaluate` keeps `&& highest > boundary` verbatim at `:332`. The tagless and `Query::all` carry is not work to be done either: it is already written and is roughly eight lines — `chunks` at `chain.rs:258-262` for the `Query::all` arm and `chain_sql` at `:341-357` for the tagless ones. All four shapes then cleared the same bar, `conformance.txt`: **356 passed, 0 failed** — four shapes × 89 rules. What D does keep is `tag_cardinality` and its write-path upsert. **CN-1 is upside rather than risk:** `chain_sql` sorts with `most_selective_first` (`chain.rs:340`), so every D figure above was taken under the ordering measured 38–44x backwards — the pessimal one. Whether inverting it helps D further is unmeasured; whether the penalty is *hidden* in D's numbers is not in question, because it is included in them.
- **Semver:** none.
- **Forecloses:** nothing, and it is the only shape that makes `query_sql.rs:77-79`'s existing sentence true.
- **The evidence behind it is the thinnest of the four:** two tags, one event type, a uniformly-spread selective tag, warm cache — and the review's own remediation says so: *"adopting a 3,126x on that basis is precisely the move this section is about. Treat it as an arm to be built and measured against three-tag guards and a clustered tag, not as a patch."*

---

## Recommendation

**One thing is settled and one is not, and they should be decided as two questions rather than one.**

**Settled: the chain does not stay, and item 1 of §8 does not survive as a requirement.** The shipped shape is dominated by two different alternatives across the whole grid — nine of nine against the aggregate, six of nine plus three ties against the bounded chain — and its ordering policy is measured backwards on it at 38–44x across two runs. Option A is not defensible on the evidence; whatever else this record does, it should retire "most-selective-tag-first is a requirement" rather than restate it, and should not replace it with option C's inverted sort, which the experiment declines to generalise past two tags.

**Recommended shape: D, `chain-bounded-all-arms`. This recommendation was B in the first draft and flipped under review; what flipped it is recorded below and in the revision record.**

The B case rested on three legs and two of them were wrong.

- *"It changes one string in a private module and touches no correctness surface"* — half false. `chunks` is reached from the guard path under `BEGIN IMMEDIATE` (`event_store.rs:699`) and from the read path (`:1313`), and B **adds** a `tags.len() == 1` branch that shipped `item_sql` does not have (`query_sql.rs:316-354` against `chain.rs:441-443`).
- *"11.8x cheaper cold"* — true against the chain, and only against the chain. In the same dump D is cold at 236,988 µs against B's cold 335,498 and B's warm 296,556 (`query-plans.txt:37-45,46-54`). Cold cost was B's strongest argument and it belongs to D.
- The premise that made D expensive is falsified outright: `evaluate`'s `highest > boundary` does **not** change meaning under a bounded shape (`probe_store.rs:300-303`, `:332`), and the tagless / `Query::all` carry is ~8 lines already written (`chain.rs:258-262,341-357`). All four shapes cleared 89 conformance rules each, 356 passed, 0 failed.

What is left is the asymmetry this brief's own cost-of-delay section identified and then argued past. **B is the only irreversible option on the table**: it deletes `tag_cardinality` and, by §8's own "a `GROUP BY` is an optimisation barrier", forecloses D. **D forecloses nothing** — it keeps the table, keeps the ordering seam, and is the only shape that makes `query_sql.rs:77-79`'s existing sentence true. Buying a 1.5x warm improvement with the one one-way move available, when the alternative is free to reverse, is the wrong trade even before the numbers: B still leaves **362 ms of held `BEGIN IMMEDIATE` at 10^6 events**, which is three orders of magnitude above D's 179 µs on the same cell. And D's 155x / 1,295x / 2,024x wins over B on `accepted-2tag-at-head` were taken under the *pessimal* seed ordering, since `chain_sql` sorts most-selective-first (`chain.rs:340`) — so CN-1 is headroom in D's favour, not an unpriced risk.

**Unchanged by the flip:** the chain as shipped does not stay, item 1 of §8 does not survive as a requirement, and option C's inverted sort is not the replacement.

**The strongest argument against D, verbatim from this brief's own evidence section, which the flip does not dissolve:** *"The evidence behind it is the thinnest of the four: two tags, one event type, a uniformly-spread selective tag, warm cache — and the review's own remediation says so: 'adopting a 3,126x on that basis is precisely the move this section is about. Treat it as an arm to be built and measured against three-tag guards and a clustered tag, not as a patch.'"* That objection is real and it is not answered by anything above. What it does not do any more is favour B, because B's evidence base is the same grid and B is the option that cannot be walked back.

**So: D is the recommendation, and the extra run is still worth doing — but it is now a confirmation rather than a gate.** One more run of `experiments/shipped-append-condition-sql/run.sh` extended to three-tag guards, a clustered selective tag, a cold-cache column and `Shape::ChainBoundedAllArms` under both orderings costs one afternoon; the arms, the harness and the conformance clearance all exist. If the owner wants the shape settled before that run, take D, because it is the reversible half of the choice. If the run happens first and D fails to hold at n ≥ 3, nothing has been spent — the chain is still there and `tag_cardinality` with it.

---

## Cost of delay

**Three different clocks, and only one of them runs out.**

1. **The emitted SQL is free forever.** `query_sql` is private (`lib.rs:89`), the statement text is not API, and changing it breaks no signature at any version. I-1's own Semver line agrees: *"None."*
2. **The schema is free only until `0.2.0`.** If the shape stops needing `tag_cardinality`, the table is dead weight that still costs an upsert per tag per event inside `BEGIN IMMEDIATE` (`event_store.rs:866-876`) — up to 32,768 of them at declared ceilings. Removing it from `MIGRATION_1` while `SCHEMA_VERSION` is 1 and **no schema has reached a stranger** is an edit; removing it after `0.2.0` ships migration 1 to strangers is migration 2, a compatibility surface, and a `schema-migration-and-identity` story. **The clause "and nothing is published" has been removed from this item as false:** `happenstance`, `happenstance-core` and `happenstance-testkit` have been live at `0.2.0-alpha.1` since 2026-08-16, and `happenstance-sqlite` at `0.0.0` since 2026-08-18 — eighteen days at the time of writing. The clock survives the repair on the narrower fact the registry actually supports: the `0.0.0` row is a placeholder carrying no schema and `happenstance-sqlite` never cut an alpha, so `MIGRATION_1` has reached nobody. **This is the only genuinely one-way part of the decision, and it belongs to option B specifically** — which is now an argument for D rather than a residual risk of the recommendation.
3. **The prose is a clause edit at any time, and gets read at `0.2.0`.** ES-27's `Rejects:` line and the two rendered doc sites cost nothing to repair whenever, but `0.2.0` is when a stranger first reads them on docs.rs and on the specification.

**Not free, and not on the `0.2.0` clock: the reputational one-shot.** The chain is what `0.2.0` publishes as the first behaviour of the adapter this specification exists to certify, and it is the arm that loses to everything it was measured against.

### Whether X-1 must wait on this record — the plain answer

**Half of X-1 is shape-invariant and need not wait. The other half is entirely contingent on this record, and X-1 should not be implemented before it lands.**

- **The `chunks` half does not wait.** `planned_statement_count` is `crate::query_sql::chunks(query, &Selectivity::default(), width).len()` (`event_store.rs:326-334`) — it counts the *partition*, not the arm SQL, and it makes the read path's own call by construction rather than agreeing with it by arithmetic. The partition arithmetic is identical under both shapes: one bound parameter per tag and one per type, whether the arm is a chain (`query_sql.rs:341,350`) or an aggregate (`chain.rs:430-431`). `MAX_QUERY_ARMS_PER_STATEMENT = 400` is likewise shape-invariant — its rationale is `SQLITE_MAX_COMPOUND_SELECT`'s 500 terms (`event_store.rs:266-267`), and a grouped arm is still one term of the compound. So the framing "X-1 partitions against a chain that is then abandoned, and `0.2.0` publishes a public description of an abandoned plan" **does not hold for this seam**: it describes the partition, and the partition survives the shape change.
- **The `Selectivity::read_for` half is contingent, and it is the half that fails first.** X-1's remediation is to *"give `Selectivity::read_for` a chunk width it currently has no concept of"* — and under option B there is no `read_for` to chunk. `grouped_sql` takes no `Selectivity` argument at all (`chain.rs:314,412-414`); the whole struct, its unchunked `WHERE tag IN (…)` lookup, and its two call sites (`event_store.rs:698` inside `BEGIN IMMEDIATE`, and `:1301` on the read path) are deleted rather than repaired. That is the site X-1 measured failing at **300 items × 128 tags = 38,400 parameters**, before `chunks` fails at 400 × 128 = 51,600 — the earlier of its two failures, and the one with the write lock held.

> **[Overtaken by events — 2026-09-04, the `X-1` lane.]** X-1 landed before this record, against its instruction above. Both halves are implemented: `chunks` now partitions on bound parameters as well as arms, and `Selectivity::read_for` takes a width. The quoted body of `planned_statement_count` is therefore no longer verbatim — it passes both ceilings now — though the sentence it supports is unchanged, because the partition is still what the seam counts. Under option B the `read_for` half is discarded with `read_for` itself, exactly as this section predicts; the `chunks` half survives any option. The note is here rather than in the body because this brief is another author's and had a two-critic pass this correction did not.


So: **X-1 must be sequenced after this record, not before.** Not because its partition is wrong — it is right under either shape — but because roughly half its remediation targets a function whose existence this record decides, and doing that work first is work that option B throws away. If this record chooses A, C or D, X-1 proceeds exactly as written.

---

## What this does not settle

- **Whether the boundary should reach SQL at all.** Option D is the only shape that makes it worthwhile and it is the one with the thinnest evidence. I-2's fact stands (no `position > ?` reaches SQLite); its proposed fix is falsified (C-5) and is not revived here.
- **The n ≥ 3 tag case, on any shape.** Nothing in the corpus measures a three-tag guard. Under the chain that is where the ordering policy stops having arithmetic; under the aggregate it is where `COUNT(DISTINCT tag) = n` starts costing more temp-B-tree work. Both are unmeasured.
- **A clustered selective tag, and cold cache as a first-class column.** `row:r7` is uniformly spread and every median here is warm. The one cold data point that exists (3.97 s against 0.34 s) is a single execution in a plan-dump, not a distribution.
- **The write-side cost of `tag_cardinality`.** All four arms share one schema and all maintain the table, so the experiment cannot price its removal. The arithmetic above (32,768 upserts at ceilings) is a bound, not a measurement.
- **`PAGE_SIZE` and the paged read.** Audit entry **I-3** — `Query::all` emitting `WHERE position IN (SELECT position FROM event)`, 60,092 µs against 71 µs for the identical page (`query-plans.txt:14,19-20`), and a 97.2 s replay of 10^6 events (`:7`) — is a different record. `event_store.rs:140` still says *"a placeholder until it is measured"*, and ADR-0022 §15 (`references/adr/0022-append-condition-strategy.md:585-587`) already records that as a deliberate non-verdict. It shares this record's file and none of its question.
- **Whether `MAX_QUERY_ARMS_PER_STATEMENT` stays public**, and **whether the testkit's VT-23 rule is widened to cross the tag axis** — both are X-1's, both have release-timing consequences of their own, and neither is decided by choosing a query shape.
- **Every other section of ADR-0022.** §4, §6, §7, §9, §10, §11 and §15 are unchanged, and this record supersedes only items 1 and 2 of §8.

---

## Revision record

Two critiques were filed against the first draft. Neither was accepted as written and both changed the brief; one of them changed the recommendation.

**1. The recommendation flipped from B to D.** *(critique: `recommendation-should-flip`.)* The B case had three load-bearing legs, and the review falsified two of them and re-priced the third.

- **Removed as falsified:** option D's cost line claimed that binding the boundary into SQL makes `evaluate`'s `highest > boundary` "change meaning" — a correctness surface — and that "every tagless arm and the `Query::all` short-circuit must carry the boundary too". `probe_store.rs:300-303` documents the opposite mechanism ("the two spellings agree by construction, and no shape can win by deciding less"), `evaluate` keeps the comparison verbatim at `:332`, and the tagless carry is ~8 lines already written (`chain.rs:258-262,341-357`). All four shapes then cleared 89 conformance rules each (356 passed, 0 failed). The claim is struck rather than reworded, because it was the entire basis for calling D "the most [expensive] of any option".
- **Re-priced:** B's cold-cost advantage was stated against the chain only. The same plan dump has D cold at 236,988 µs against B cold 335,498 and B warm 296,556.
- **Re-signed:** CN-1 was carried as an open risk against D. Because `chain_sql` sorts most-selective-first (`chain.rs:340`), every D figure was measured under the ordering this brief shows is 38–44x backwards, so CN-1 is upside.
- **Followed through:** with those corrected, this brief's own cost-of-delay analysis points the other way. B is the sole irreversible option (it deletes `tag_cardinality` and a `GROUP BY` forecloses D); D forecloses nothing; and B buys 1.5x warm off 362 ms of held `BEGIN IMMEDIATE` at 10^6, three orders above D's 179 µs on the same cell.

The objection that did **not** move — D's evidence base is two tags, one event type, a uniform selective tag and a warm cache — is now recorded verbatim under Recommendation as the strongest argument against the recommended option.

**2. Two publication claims were false at the registry and have been removed.** *(critique: `premise-falsified`.)* Verified directly against crates.io on 2026-09-03.

- Option A's Semver line said *"All private; `happenstance-sqlite` is unpublished."* The crate is live at `0.0.0`, created 2026-08-18, 16 downloads. Removed and replaced with what the registry supports.
- Cost-of-delay item 2 said *"removing it from `MIGRATION_1` while `SCHEMA_VERSION` is 1 and nothing is published is an edit."* `happenstance`, `happenstance-core` and `happenstance-testkit` have been live at `0.2.0-alpha.1` since 2026-08-16. Removed. The clock itself survives on the narrower and true fact — the `0.0.0` row carries no schema and `happenstance-sqlite` never cut an alpha — but the false premise is struck rather than quietly rewritten, because correction **C-4** of the pre-publication review this brief cites had already corrected this same claim five days earlier, and §8's argument is precisely that a premise true when written and false when relied on is the failure mode. **Whoever executes should treat this as the repair that must land before, not with, the shape change.**
- Two smaller defects in the same critique, both understating B's cost, were folded into option B: `chunks` is shared by the guard path (`event_store.rs:699`) and the read path (`:1313`), so B is not "one string in a private module"; and shipped `item_sql` has no `tags.len() == 1` branch, so B adds one rather than keeping one.

**Unchanged by either critique:** the settled half of the recommendation (the chain does not stay; §8 item 1 does not survive as a requirement; option C is not the replacement), the nine-cell measurement table, the 38–44x seed-ordering finding, the falsification of I-2's remedy (C-5), the ES-27 `[FROZEN]` clause-edit analysis, and every entry under "What this does not settle". Every other cited `path:line` in the brief was checked by the second critique and resolves to what is claimed.
