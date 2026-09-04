# Does `AppendCondition::after_opt` keep blanket scope under a name that says so, or does it change shape?

Record: **AE-3-after-opt**. Source entry: `references/evaluation/review-pre-publication-2026-09-03.md:742-780`.
Brief only — no atom, no ADR prose, no code.

---

## Why this is owed

`AppendCondition` is a chainable builder whose two mutators have different scopes
and the same return type, so the type system cannot tell the two call orders
apart.

`and_guard` **appends** one guard carrying its own boundary
(`crates/happenstance-core/src/append.rs:170-177`):

```rust
    #[must_use]
    pub fn and_guard(self, query: Query, after: Option<SequencePosition>) -> Self {
        let mut guards = Vec::from(self.guards);
        guards.push(Guard { query, after });
        Self {
            guards: guards.into_boxed_slice(),
        }
    }
```

`after_opt` **rewrites every guard already present**
(`crates/happenstance-core/src/append.rs:200-212`):

```rust
    #[must_use]
    pub fn after_opt(self, position: Option<SequencePosition>) -> Self {
        let guards = Vec::from(self.guards)
            .into_iter()
            .map(|guard| Guard {
                query: guard.query,
                after: position,
            })
            .collect::<Vec<_>>();
        Self {
            guards: guards.into_boxed_slice(),
        }
    }
```

The `map` reconstructs each `Guard` with `position` and never reads `guard.after`.
`after` is the same method (`append.rs:185-189`: `self.after_opt(Some(position))`).
So `AppendCondition::new(q1).and_guard(q2, Some(p2)).after(p1)` compiles, is
`#[must_use]`-clean, and discards `p2` with no diagnostic at any layer.

The consequence is asymmetric in the dangerous direction. A decision model's
busiest fragment usually carries the highest position, so a trailing `after`
*raises* the quiet guards' boundaries and widens what the condition tolerates —
an append that should have been refused is admitted. That is the lost update
VT-30's own `Rejects:` clause was written against
(`spec/SPECIFICATION.md:1877-1882`).

Neither doc block says it. `after_opt`'s doc explains the blanket scope and
points at `and_guard` for per-guard boundaries (`append.rs:191-199`) without
saying that calling it *after* `and_guard` erases what `and_guard` set;
`and_guard`'s doc (`append.rs:155-169`) does not say it either.

---

## What is true today

**The blanket scope is required, not incidental.** VT-30
(`spec/SPECIFICATION.md:1861-1867`):

> `AppendCondition::new(query)` MUST continue to produce a single unbounded
> guard, and `after`/`after_opt` MUST continue to apply the given boundary to
> every guard.

Its marker (`spec/SPECIFICATION.md:1869-1872`):

> `[PROVISIONAL — falsified if no adapter can push a multi-guard condition into a
> single statement without one self-join per guard, or if the `min()` collapse
> measures as immaterial on a Wattline-shaped workload; the Postgres adapter and
> the benchmark harness are the two instruments and both are unbuilt]`

A `[PROVISIONAL]` clause is **binding until the named thing happens**
(`spec/SPECIFICATION.md:198-200`). So today the behaviour is mandated; this record
is about the *spelling and shape*, not the semantics.

**The same MUST is restated in an accepted, immutable decision record.**
`references/adr/0012-append-shape-and-preconditions.md:604-608` carries the
sentence verbatim, in §9, alongside the `Box<[Guard]>` and `#[non_exhaustive]`
reasoning. Its atom `.kb/decisions/0012-append-shape-and-preconditions.md` is
`status: accepted`, `supersedes: null`, `superseded_by: null` (`:5, :17, :18`).

**Every call site in the tree writes the safe order.** Verified by
`grep -rn "and_guard|after_opt|\.after\("` across `crates/`, `examples/`,
`xtask/`, `docs/`, `README.md`:

| Site | Order | Guards | Would a stricter shape change it? |
|---|---|---|---|
| `crates/happenstance-core/src/append.rs:92-94` (doc example) | `after_opt` → `and_guard` | 2 | No |
| `crates/happenstance-core/src/append.rs:372, :385` (unit tests) | `after` only | 1 | No |
| `crates/happenstance-core/tests/wire.rs:318-320` (proptest generator) | `after_opt` → `and_guard`* | 1–3 | No |
| `crates/happenstance-core/tests/wire.rs:894-896` | `after_opt(None)` → `and_guard` | 2 | No |
| `crates/happenstance/src/command.rs:309` (the typed command loop) | `after_opt` only | 1 | No |
| `crates/happenstance/tests/boundary_refusal.rs:76, :137` | `after_opt` only | 1 | No |
| `crates/happenstance/tests/command_loop.rs:314-315` | `after_opt` only | 1 | No |
| `crates/happenstance-testkit/src/fixtures.rs:166` (`condition_after`) | `after` only | 1 | No |
| `crates/happenstance-testkit/src/model.rs:434` | `after_opt` only | 1 | No |
| `crates/happenstance-testkit/src/suite.rs:5051-5052, :5071` | `condition_after` → `and_guard` | 2 | No |
| `crates/happenstance-testkit/src/suite.rs:5226-5229` | `condition_after` → `and_guard`× | 1–2 | No |
| `crates/happenstance-testkit/tests/properties.rs:280` | `after_opt` only | 1 | No |
| `crates/happenstance-sqlite/tests/append.rs:420, :433` | `after` only | 1 | No |
| `crates/happenstance-sqlite/tests/append.rs:474-476` | `new` → `and_guard` ×2, no boundary | 3 | No |
| `crates/happenstance-sqlite/tests/wide_query.rs:262, :281` | `after` only | 1 | No |
| `README.md:69`; `docs/first-encounter.md:70, :112`; `docs/carry-your-invariant.md:78, :121`; `crates/happenstance-cloudflare/README.md:66` | `after_opt` only | 1 | No |

\* The generator's own comment names the hazard and routes around it
(`crates/happenstance-core/tests/wire.rs:305-309`):

```rust
        /// One to three guards, each with its own optional boundary.
        ///
        /// `after_opt` is applied to the first guard before the others are added
        /// because it rewrites *every* guard it can see; adding the rest
        /// afterwards is what keeps their boundaries independent.
```

**Is the current behaviour pinned by a test, or merely observed?** The audit says
order-sensitivity is "already exercised" at `wire.rs:307-320` and `:895`. On the
evidence it is **observed and documented, not pinned.**

- `wire.rs:305-324` *avoids* the hazardous order to make the generator produce
  independent boundaries. It asserts nothing about what the hazardous order does.
- `wire.rs:891-936` (`condition_after_is_visible_to_an_ingest_policy`) builds
  `new(Query::all()).after_opt(None).and_guard(Query::all(), Some(boundary))` and
  asserts that `guards[0].after` is JSON `null` and `guards[1].after` is the
  boundary (`:914-928`). That pins the *safe* order — `and_guard` after
  `after_opt` keeps the new guard's boundary — and says nothing about the
  reverse.
- `append.rs:361-388` holds three unit tests, all single-guard.

The sharp form of that: **an edit changing `after_opt` from "overwrite every
guard" to "fill only the guards whose `after` is `None`" is green across the
entire workspace.** Walk the table above — every site either has one guard (whose
`after` is `None` at the point `after_opt` runs, so fill and overwrite coincide),
or calls `after_opt` before any `and_guard`, or passes `None`. Nothing anywhere
distinguishes the two implementations. That is the wrong implementation a pin
test would reject, and CLAUDE.md's rule ("a rule that no adapter can fail is
decorative — name a plausible wrong implementation it rejects") is satisfied by it.

**Nothing about this reaches an adapter author.** An adapter receives the
assembled value and reads it through the accessor:
`crates/happenstance-sqlite/src/event_store.rs:693-738` is `for guard in
condition.guards() { … }`. Call order is spent before the port is entered, so no
conformance rule can see this and none should be minted for it.

**VT-30's two instruments, re-checked against the tree — the marker text is
partly stale.**

- *Postgres adapter*: unbuilt. `crates/happenstance-postgres/src/event_store.rs:143`
  is `todo!("postgres event store: append")`; six `todo!()` in that file, thirteen
  in the crate. Phase 10 owns it (`RUNBOOK.md:4544`).
- *Benchmark harness*: **built.** `crates/happenstance-testkit/src/bench.rs` exists
  at HEAD (landed in `2665883`), exports `event_store_benchmarks!` (`:825`) behind
  the off-by-default `bench` feature (`crates/happenstance-testkit/Cargo.toml:92`),
  with an emitter at `crates/happenstance-testkit/tests/memory_benchmarks.rs`. So
  the spec's "both are unbuilt" (`spec/SPECIFICATION.md:1871-1872`) is no longer
  true as written.

  What it *cannot* do is the part that matters: none of its three scenarios builds
  a multi-guard condition. `conditional_append_under_contention`
  (`bench.rs:574-636`) uses the single-guard helper `condition_after(...)` at
  `:597`, and `and_guard` appears nowhere in the file. The blocker is therefore
  not "no harness" but "no multi-guard workload in the harness" — a much smaller
  gap than the marker implies.

- A third fact bearing on the falsifier's first limb: the one adapter that *is*
  built deliberately does **not** push a multi-guard condition into a single
  statement. `crates/happenstance-sqlite/src/event_store.rs:661` — "**One `SELECT
  max(position)` per guard statement**, which is ADR-0022's decision and the one
  that measured fastest on the *rejection* path". That is a measured preference
  for statement-per-guard, not a failure to achieve pushdown, so it neither fires
  nor discharges the limb — but it is evidence whichever change next lifts the
  marker will have to weigh, and it is on disk today. (Revision 1 wrote "the
  marker-lifting pass" here as though one were scheduled. None is; see the
  Revision record.)

**Publication state, precisely.** `happenstance-core` is live at
`0.2.0-alpha.1` — "The first published release, and it is a pre-release on
purpose. The API is expected to move until the stable `0.2.0`; only one alpha
resolves at a time, and each is yanked when the next lands"
(`CHANGELOG.md:306-313`). The workspace manifest states the same intent:
"`-alpha.1` because the API is expected to keep moving until the stable `0.2.0`,
and a stable number would make 'we can't change that now' available as an
argument against the rest of the runbook" (`Cargo.toml:6-15`). The stable `0.2.0`
is phase 12 (`RUNBOOK.md:4681`), whose own text is explicit that publication does
not freeze the API — "The API was frozen in phases 4 – 6, on evidence, which is
what makes publishing safe" (`RUNBOOK.md:4690-4692`).

A break is not silent: CI runs `cargo-semver-checks` on every pull request against
`baseline-rev` (`.github/workflows/ci.yml:342-375`), so a rename is reported at
review time and the version bump is the acknowledgement.

**Governing records.** `.kb/decisions/0012-append-shape-and-preconditions.md`
(accepted, immutable) is the atom in scope; the blanket-scope MUST lives in its
long form at `references/adr/0012-append-shape-and-preconditions.md:604-608`, and
the atom's own body does not restate it (grep for `after_opt`/`and_guard`/
"every guard" in the atom returns nothing). Nothing in `.kb/open-questions/`
carries this question; the nearest neighbour,
`disjoint-boundaries-have-no-clause.md`, is a different gap.

**Which atom would this supersede?** *None.* The recommended half changes no
decision. If the breaking half is later taken, the precedent on disk is
**amendment without supersession**: `.kb/decisions/0029-msrv-raised-to-1-97-1.md`
amends ADR-0004 with `supersedes: null` (`:10`) because "that decision's body
stays verbatim, because its reasoning is what this one acted on" (`:14-16`). The
same applies here — ADR-0012 §9's reasoning is what a rename would be acting on,
not what it would be correcting.

---

## Options

### Option 1 — Pin the behaviour, and say it in both doc blocks. Change nothing else.

Two unit tests in `append.rs`'s existing `mod tests`: one asserting that
`new(q1).and_guard(q2, Some(p2)).after(p1)` leaves **both** guards at `p1`, one
asserting that the safe order leaves them independent. Plus one sentence on
`and_guard` and one on `after_opt` naming the erasure and the order that causes it.

- **Costs a caller:** nothing. No surface moves.
- **Costs an adapter author:** nothing. Call order never reaches the port.
- **Semver:** none. Tests and docs.
- **Forecloses:** nothing. Every other option remains open, and each subsumes this
  one — under Options 2 or 3 the pin is renamed mechanically; under Option 4 the
  hazardous half stops compiling and the pin is deleted having done its job for
  the alpha it covered.
- **What it does not buy:** a caller who writes the hazardous order still gets no
  diagnostic. This is a ratchet against silent *regression*, not a fix for the
  hazard.

### Option 2 — Rename to carry the scope, hard break (e.g. `after_every_guard` / `after_every_guard_opt`).

- **Costs a caller:** a compile error and a one-line edit per call site. Downstream
  is empty as far as this repository can see; in-tree it is ~15 call sites plus six
  documentation occurrences (`README.md:69`, `docs/first-encounter.md:70, :112`,
  `docs/carry-your-invariant.md:78, :121`,
  `crates/happenstance-cloudflare/README.md:66`), all single-guard and mechanical.
  One cost is easy to miss: `xtask/tests/first_encounter.rs:423` and `:696` assert
  the **source substring** `after_opt(upto)` in a generated program, so the rename
  breaks two doc-fidelity gate tests that match the method name as a literal string.
- **Costs an adapter author:** nothing.
- **Semver:** **breaking** on a published item — though see *Cost of delay*: on a
  yanked-on-next-alpha pre-release the practical price today is one alpha bump and
  a CHANGELOG line.
- **Also requires:** amending VT-30's sentence, which names `after`/`after_opt`
  explicitly (`spec/SPECIFICATION.md:1866`), and a new atom amending ADR-0012 §9's
  restatement.
- **Forecloses:** little, but it spends the break. Taking it now and then taking
  Option 4 later spends two breaks where one would have done — though see *Cost of
  delay*: while the version is a pre-release, both breaks cost one alpha bump each.
- **Weakness:** a better name warns; it does not reject. `after_every_guard(p1)`
  after `and_guard(q2, Some(p2))` still compiles, and a reader can plausibly read
  "every guard" as *tighten* every guard — `max(existing, p1)` — rather than
  *replace*. The hazard is call order, and a name cannot see call order.

### Option 3 — Additive rename: introduce the new name, `#[deprecated]` the old one.

- **Costs a caller:** a warning, not an error; they move at their own pace.
- **Costs an adapter author:** nothing.
- **Semver:** **additive (minor).** Available today *and* after 0.2.0, which
  materially weakens the framing that the rename is "free only until 0.2.0".
- **Forecloses:** nothing. It carries a deprecated alias until something removes
  it, and the removal is itself a break — but **priced**, which the first revision
  of this brief did not do, and the omission is what let one clause ("deferred, not
  avoided") dismiss the option. The price is the same one *Cost of delay* puts on
  every break in this window: one alpha bump and a CHANGELOG line
  (`CHANGELOG.md:306-313`), on a version `cargo add` will not resolve by accident
  — "a pre-release only resolves for a requirement that asks for one, so `cargo add
  happenstance` does not pick this up by accident" (`Cargo.toml:12-14`). Add the
  alias now and delete it before the stable `0.2.0` and the whole episode costs one
  alpha bump: a courtesy period, not a carousel. Only an alias that *survives into*
  the stable `0.2.0` becomes permanent, and nothing forces it to.
- **Does not require** amending VT-30 today: `after`/`after_opt` continue to exist
  and continue to apply the boundary to every guard, so
  `spec/SPECIFICATION.md:1866-1867` stays literally true while the alias stands.
  The amendment, and the new atom against ADR-0012 §9's restatement, become owed at
  the moment the old names are deleted — inside the same alpha window, at the same
  price.
- **House friction, stated:** this workspace has previously declined a deprecated
  arm on exactly this reasoning — "There is no deprecated arm: nothing in this
  workspace is published yet, and this is the last release in which that is true"
  (`CHANGELOG.md:1697-1700`). That premise has now expired, so the precedent argues
  *for* considering Option 3, not against it. With `-D warnings` in the gate, all
  in-tree call sites must move in the same commit regardless.

### Option 4 — Change the shape so the overwrite cannot be written.

Make the blanket boundary settable only *before* any guard is added — `new` yields
a one-guard builder carrying `after`/`after_opt`; `and_guard` returns a type that
has neither. The safe order keeps compiling; the hazardous order becomes `E0599`.

- **Costs a caller:** the largest of the four. `AppendCondition::new(q)` stops
  being an `AppendCondition`, so every site that passes it straight to `append`
  needs a conversion — e.g. `crates/happenstance-sqlite/tests/append.rs:456, :498`,
  `crates/happenstance-testkit/src/fixtures.rs:155-157`.
- **Costs an adapter author:** nothing.
- **Semver:** **breaking**, and wider than Option 2.
- **Also requires:** amending VT-30's *first* compatibility sentence —
  "`AppendCondition::new(query)` MUST continue to produce a single unbounded
  guard" (`spec/SPECIFICATION.md:1865-1866`) — which a builder type violates
  literally.
- **Forecloses:** any future need to re-blanket an assembled condition (a
  replication ingest policy that wants to clamp every guard to a local watermark
  is the shape to check before choosing this; nothing in the tree does it today,
  but `happenstance-sync` is unwritten).
- **Strength:** it is the only option that converts the hazard into a compiler
  error rather than a hint.

### Option 5 — Change the semantics: `after_opt` fills only guards whose `after` is `None`. **Not supported by the evidence.**

Recorded so it is not re-proposed. It contradicts a binding `[PROVISIONAL]` MUST
(`spec/SPECIFICATION.md:1866-1867`) and its restatement in an accepted, immutable
record (`references/adr/0012:604-608`); and, as established above, it is a silent
behaviour change on a published item that **the entire existing test suite stays
green through**. It is precisely the edit Option 1's pin exists to stop.

---

## Recommendation

**Take Option 1 and Option 3 together, now: pin the behaviour and say it in both
doc blocks, *and* introduce the scope-carrying name with `#[deprecated]` on the
old one.**

**This recommendation flipped.** The first revision recommended Option 1 alone and
routed Options 2, 3 and 4 to "the pass that lifts VT-30's `[PROVISIONAL]` marker".
That routing claim has been **removed, not rewritten around**: the pass does not
exist. See *Why the deferral collapsed* below and the Revision record at the foot.

Option 1's half is unchanged and its evidence is unambiguous: the current
behaviour is mandated, undocumented at the point of danger, and unpinned — a flip
to fill-not-overwrite passes every test in the workspace. Pinning it costs one
test module addition, blocks nothing, and is subsumed by whichever of Options 2–4
is eventually taken.

**Why the deferral collapsed.** The entire case for holding Options 2–4 was that a
scheduled pass would decide them on better evidence. There is no such pass, and
the tree says so in four places:

- The provisional ledger's VT-30 row names owning phase **4** — shipped — with the
  falsifier "E2E-04 and E2E-05 still unwritable after phase 4" (`RUNBOOK.md:608`).
- ADR-0012 voids that falsifier in terms: "It is not a falsifier of the *sentence*
  … A delivery check wearing a falsifier's clothes satisfies CF-38's non-emptiness
  rule while defeating its purpose … the ledger cell is owed a correction"
  (`references/adr/0012-append-shape-and-preconditions.md:536-543`). Nobody made
  the correction.
- VT-30 appears in **no** runbook phase after 4 (`grep -n "VT-30" RUNBOOK.md`);
  "Wattline", the workload the spec marker names, appears nowhere in `RUNBOOK.md`
  at all; and phase 10's text (`RUNBOOK.md:4544` onward) never mentions multi-guard
  pushdown or the `min()` collapse.
- Phase 12's exit criterion audits provisional clauses **against that ledger**,
  explicitly "not against prose" (`RUNBOOK.md:4725-4730`). So VT-30 clears the
  audit on a row naming a phase already shipped, and no pass is ever convened.

"Route it to the marker-lifting pass" was routing to an address that does not
exist. Option 1 alone therefore concedes **zero hazard reduction over an interval
this brief cannot bound** — which is the objection's sharpest sentence, and it is
correct.

**Why Option 3 rides along, and not Option 2 or 4.** Option 3 survives every
branch the deferral was protecting against:

- *If VT-30 is withdrawn and the guard sequence with it* — the branch the first
  revision feared — the alias dies in the same edit that deletes `and_guard`
  (`references/adr/0016-the-wire-format.md:225-229`). No break is spent on a method
  that no longer has the problem, because a rename folds into a larger break.
- *It wants no Postgres evidence.* The complaint is not that the semantics may be
  wrong; it is that the name misstates the semantics VT-30 mandates **today**. That
  is decidable now and does not improve by waiting.
- *It does not spend Option 4.* The builder-shape diagnostic remains open on top of
  it, and taking it later would rename or delete both spellings in one edit.
- Option 2 buys nothing Option 3 does not, at the price of a compile error where a
  warning suffices — including two doc-fidelity gate tests that match the method
  name as a source substring (`xtask/tests/first_encounter.rs:423, :696`), which
  must move under Option 3 too but need not be forced on anyone downstream.

**The argument that flipped this, in the first revision's own words** (recorded
because it prevailed, not because it was answered):

> A pin plus a doc sentence is exactly the remedy that reads as action and changes
> nothing a caller experiences. Documentation is the control that already failed
> here: `after_opt` carries a four-sentence doc block explaining its scope
> (`append.rs:191-199`) and it still does not say the one thing that matters, and
> the type's own example writes only the safe order (`append.rs:92-94`). A fifth
> sentence in a block that already failed is the same remedy at greater length. In
> the meantime `happenstance-core` is live, every phase between here and 0.2.0
> adds documentation and call sites in other people's code, and the one window in
> which the name can be fixed without a deprecation carousel is open now.

**The strongest argument against the *new* recommendation, in its own words:**

> A better name warns; it does not reject. `after_every_guard(p1)` after
> `and_guard(q2, Some(p2))` still compiles, and a reader can plausibly read "every
> guard" as *tighten* every guard — `max(existing, p1)` — rather than *replace*.
> The hazard is call order, and a name cannot see call order. Worse, the alias is a
> second public spelling of a method whose semantics a falsified VT-30 may withdraw
> entirely, and this workspace has declined a deprecated arm before on precisely
> this kind of reasoning (`CHANGELOG.md:1697-1700`). The premise behind that
> refusal expired; the taste behind it did not.

The reply is that the alias and its removal are both priced at one alpha bump
inside a window that *Cost of delay* shows is flat, and that Option 4 — the only
option that converts the hazard into a diagnostic — stays available on top. Whether
that diagnostic is worth its own break is still the human's call, and this brief
does not make it.

---

## Cost of delay

**Not "free now and rising". Flat, then a step.**

- **Option 1 (recommended):** free today and free permanently. No deadline. What
  decays is exposure, not price: until it lands, a semantics flip is green.
- **Options 2 and 4 (breaking):** the price is *flat* between now and the stable
  `0.2.0`, not rising. The published `0.2.0-alpha.1` is a pre-release that is
  yanked when the next alpha lands (`CHANGELOG.md:308-310`), chosen precisely so
  that "we can't change that now" is unavailable as an argument
  (`Cargo.toml:8-11`). So the cost today is one alpha bump plus a CHANGELOG entry,
  and it is the same cost at phase 10. The step is at phase 12
  (`RUNBOOK.md:4681`): after the stable `0.2.0`, the same edit costs a `0.3.0` or a
  permanent alias.
- **Option 3 (additive):** available at any time, before or after `0.2.0`. It has
  no deadline at all, which is the fact that most weakens the urgency framing.

**Consequence — corrected.** The first revision concluded from the flat price that
"there is no cost to letting the marker-lifting pass decide, and there is a real
option-value cost to deciding now". That conclusion is **withdrawn with its
premise**: there is no marker-lifting pass (see *Why the deferral collapsed*), so
"letting it decide" resolves to never deciding, and the option value was being
bought from a counterparty that does not exist. What the flat price actually
supports is the opposite reading: a break now and a break later cost the same, so
waiting saves nothing, and Option 3's alias can be both added and retired inside
that flat interval for one alpha bump.

The one thing that is genuinely time-boxed is **phase 12's exit criterion**: every
`[PROVISIONAL]` clause published at the stable release must have its falsifier
scheduled in a later phase or sit behind an unstable feature
(`RUNBOOK.md:4725-4730`). VT-30 is on the published surface and behind no feature.
But that criterion is audited **against the provisional ledger, not against the
clause**, and the ledger row already names an owning phase (`RUNBOOK.md:608`,
phase 4) that has shipped — so VT-30 passes the audit unexamined. The deadline is
real and the mechanism that was supposed to enforce it does not bind here.

---

## What this does not settle

1. **VT-30's `[PROVISIONAL]` marker.** Untouched by anything here. Its first limb
   still needs the Postgres adapter (unbuilt, `RUNBOOK.md:4544`); its second needs
   a multi-guard `min()`-collapse workload that the built harness does not contain.
   And — established during this revision — **no phase after 4 names VT-30**, so
   nothing on today's runbook schedules the lifting at all. That is itself a gap
   worth someone's attention, and it is larger than this record.
2. **Whether Option 4's diagnostic is worth its own break.** Option 3 is now
   recommended and Option 2 is subsumed by it; Option 4 stays open on top of both.
   Choosing it still wants the Postgres evidence about whether multi-guard
   conditions survive a store that does not serialise its writers, and nothing
   currently schedules that evidence either.
3. **The stale marker text.** `spec/SPECIFICATION.md:1871-1872` says both
   instruments "are unbuilt"; `crates/happenstance-testkit/src/bench.rs` exists.
   Correcting that sentence — and restating the blocker as *the harness has no
   multi-guard scenario* — is owed to whichever change next edits VT-30. On today's
   runbook nothing does, which is part of why the first revision's deferral was
   unsafe. It is body prose, not the MUST, but `spec-trace` will not catch it.
4. **Whether the harness gains a multi-guard scenario, and who pays for it.** It is
   the cheaper of VT-30's two instruments and nothing currently schedules it.
5. **The ADR number.** The audit's routing note stands: ADR-0012 discharged VT-30
   and is accepted and immutable, and the queue at `RUNBOOK.md:262-275` allocates
   no number for this. `.kb/decisions/` runs to 0036 on disk.
6. **Whether the pin should ever become a conformance rule.** It should not — an
   adapter only ever sees the assembled condition
   (`crates/happenstance-sqlite/src/event_store.rs:693-738`) — but nothing written
   down says so, and the next person to read AE-3 may reach for `suite.rs`.
7. **Whether `happenstance-sync`'s ingest policy will want to re-blanket an
   assembled condition.** That is the use case Option 4 forecloses, and the crate
   that would answer it is unwritten (`crates/happenstance-sync/`, `publish = false`).
8. **Documentation for the multi-guard form.** No shipped page teaches it —
   `docs/carry-your-invariant.md:90-93` says the multi-guard shape "is described but
   is not settled … so nothing above leans on it". Correct today, and it is why a
   rename's documentation blast radius is currently six single-guard lines.

---

## Revision record

**Revision 2 — 2026-09-03.** Two critiques were sustained against revision 1. Both
were checked against the tree before being acted on; neither was accepted on
assertion.

**1. A premise was falsified, and the claim resting on it is removed.** Revision 1
recommended Option 1 alone and routed Options 2, 3 and 4 to "the pass that lifts
VT-30's `[PROVISIONAL]` marker". No such pass is scheduled. Verified:
`grep -n "VT-30" RUNBOOK.md` returns no phase after 4; the provisional ledger row
(`RUNBOOK.md:608`) names owning phase 4 — shipped — with a falsifier that ADR-0012
itself voids as "a delivery check wearing a falsifier's clothes … the ledger cell
is owed a correction"
(`references/adr/0012-append-shape-and-preconditions.md:536-543`), a correction
nobody made; "Wattline" does not occur in `RUNBOOK.md` at all; phase 10
(`RUNBOOK.md:4544` onward) never mentions multi-guard pushdown; and phase 12 audits
against the ledger, "not against prose" (`RUNBOOK.md:4725-4730`), so VT-30 clears
that gate unexamined.

  Removed rather than rewritten around: the routing instruction in the
  Recommendation; the two-paragraph rationale that "the pass that lifts VT-30's
  marker may withdraw the guard sequence altogether" and that choosing between the
  options "wants the Postgres evidence … which is exactly what the marker-lifting
  pass is convened to look at"; the *Cost of delay* conclusion that "there is no
  cost to letting the marker-lifting pass decide, and there is a real option-value
  cost to deciding now"; the phrase "which is the same pass this record routes to"
  under phase 12's exit criterion; and "at the marker-lifting pass" in Option 2's
  *Forecloses* bullet. Each site now says what was removed and why.

**2. The recommendation flipped, from Option 1 alone to Options 1 + 3.** What
flipped it: with the routing target gone, Option 1 alone concedes zero hazard
reduction over an interval this brief cannot bound, and revision 1's own
strongest-argument-against — quoted intact under *Recommendation* — no longer had
a reply. The critique also showed that revision 1 dismissed Option 3 in one
unpriced clause ("the removal is itself the breaking act — deferred, not avoided")
while its own *Cost of delay* priced every break in this window at one alpha bump
plus a CHANGELOG line, on a pre-release `cargo add` will not resolve
(`Cargo.toml:12-14`). Pricing the alias's removal at that same figure turns it
into a courtesy period rather than a carousel, and Option 3 survives both branches
the deferral was protecting against — a withdrawn guard sequence folds the alias
into a larger break (`references/adr/0016-the-wire-format.md:225-229`), and the
naming complaint needs no Postgres evidence because it is about today's mandated
semantics.

  Consequently changed: the Recommendation section (flip, the collapse of the
  deferral, why Option 3 and not 2 or 4, and a new strongest-argument-against);
  Option 3's *Forecloses* bullet, now priced, plus a new bullet stating that VT-30
  needs no amendment while the alias stands; the *Cost of delay* consequence; and
  items 1–3 of *What this does not settle*.

**Unchanged:** everything above *Options* — the mechanism, the call-site table, the
"observed not pinned" finding and the publication-state facts drew no objection and
were not touched. Option 1, 2, 4 and 5's own analyses stand as written except where
listed above. Option 5 remains unsupported by the evidence.
