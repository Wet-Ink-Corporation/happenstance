# Pressure test — the evaluation and the revised runway, adjudicated

Six dimensions audited `ARCHITECTURAL-EVALUATION.md`, `revised-runway.md` and the
seven ADRs; every finding was then adversarially refuted. This is the verdict on
what survived, deduplicated across dimensions, with the disagreements settled by
reading and compiling the code rather than by preferring one auditor.

Where two dimensions disagreed I compiled the discriminating case myself. Those
adjudications are marked **[adjudicated by compilation]** and state the probe and
the exact diagnostic. Everything else cites `file:line` read in this tree at
`2a65d76` plus the uncommitted working-tree edits shown by `git diff`.

---

## 1. Verdict — what changed about our confidence

**The defect list is trustworthy. The section-3 decision list is not, and its
headline is false.**

Thirteen numbered defects (§4) were reproduced by execution against the real
crate. Not one is fabricated; two need re-costing and several need their spans
re-anchored, but the substance holds. That half of the evaluation can be executed
as written.

The other half — §1's verdict and §3's ranked decisions — rests on a claim I
compiled and refuted. `ARCHITECTURAL-EVALUATION.md:24-26` says
`#[trait_variant::make(SendEventStore: Send)]` "silently forbids `EventStore`
from ever gaining a defaulted method, which means `head()` and `count()` can
never be added". That is the stated reason for the whole ordering of §3, for
§3.4's "strictly blocked on §3.1", and for immediate action 4 at `:1017-1020`.

**[adjudicated by compilation]** Against `trait-variant 0.1.3` (the version in
`Cargo.lock:443-446`), a trait carrying `#[trait_variant::make(SendFoo: Send)]`
accepts a provided method written in exactly the hand-desugared form the
evaluation itself prescribes at `:131-133`:

```rust
fn head(&self) -> impl core::future::Future<Output = Result<u32, Self::Error>>
where Self: Sync
{ async move { self.append(0).await } }
```

It compiles. So does `fn spawnable<T: SendFoo + Sync>(t: &T) { assert_send(&t.head()); }`
— the derived flavour's provided future is `Send` in generic code, with the
`Sync` supplied at the point of use. Only the `async fn` spelling fails, with
`error[E0728]: await is only allowed inside async functions and blocks`, which I
also reproduced. The evaluation records the mechanism (fact 1, `:82-86`) and the
workaround (`:131-133`) and then draws a conclusion that contradicts both.

Two consequences. First, the highest-blast-radius decision in the workspace is
not blocking anything: `head`/`count` can ship without touching the attribute.
Second, and more corrosive, a document whose authority is `file:line` reached its
headline by argument rather than by compiling the two lines that decide it. Every
§3 verdict is now suspect until re-derived; §4, which was executed, is not.

The revised runway is the better of the two documents on this exact point — it
refuses to settle the flavour question by argument (`revised-runway.md:471-474`,
"**Do not settle this by argument.**") — but it is regressive in two others: it
re-uses an ADR number already accepted on disk, and its ledger drops four live
rows from `docs/RUNBOOK.md`, three of which own decisions its own phase 5 cannot
exit without.

Net: confidence in the *defects* is higher than when the dossier started;
confidence in the *plan* is lower. Neither document is adoptable as written.
Both are salvageable, and most of the salvage is mechanical.

---

## 2. What survives intact

Execute these without re-litigation.

**`ARCHITECTURAL-EVALUATION.md` §4, defects D1–D13.** All thirteen reproduce.
Eight were reproduced by execution against the crate, three by running the exact
command their fix proposes adding to the gate. The five `skip_serializing_if`
attributes are at `event.rs:342`, `:344`, `query.rs:281`, `:283`, `append.rs:121`
— read them there. `SequencePosition::next` is `saturating_add` under a doc
promising `None` (`event.rs:138-144`). `ReadOptions::limit` is
`NonZeroUsize::new(limit)` with the zero case documented as ignored
(`query.rs:260-266`). `Event::new`'s equality-constrained bound is at
`event.rs:197-200`. The condition-before-emptiness order is at `memory.rs:197-216`.
Corrections to individual entries are in §3 below; none of them unmakes a defect.

**§2, the protected-decisions table.** Every row I checked holds, and the
reasoning is the kind that stops a later "simplification". One internal
contradiction (the `AppendError<E>` row versus §3.1's verdict) is listed in §3.

**§3.1's two mechanical facts.** `trait_variant` does not rewrite `async fn`
default bodies (E0728, reproduced), and an impl in the documented
`async fn append(&self, …)` style already requires `Self: Sync` for its future to
be `Send`. Both belong verbatim in the ADR. The *conclusion* drawn from them does
not — see §4.

**§3.2's problem statement.** `Event::into_parts` (`event.rs:243-246`) is
unreachable from any trait impl, because `append` takes `&[Event]`
(`store.rs:143`) and `memory.rs:219-221` therefore clones. Its doc comment —
"avoiding a clone in adapter write paths" — is false as written. The empty-batch
precedence divergence is real and the suite only ever tests `append(&[], None)`
(`suite.rs:406`).

**§3.5's sharpest claim, on a corrected cause.** `commit` accepts a batch begun
on a different store of the same type. That is real. The stated cause and the
implied remedy are not — see §3.

**The revised runway's method.** Sequence by blast radius; settle a port against
something that compiles; never accept "the gate is green" as a phase's proof
(`revised-runway.md:34-39`); a port frozen against one storage shape is shaped
like that shape (`:1769-1776`). This is a genuine improvement on `RUNBOOK.md`'s
artefact-first ordering, and it is the part worth keeping when the phase bodies
are rewritten.

**The runway's Postgres/Neon argument (`:243-263`).** Position assignment is the
right axis for `EventStore`, and `nextval()` allocating outside the transaction
is the only thing on the roadmap that can violate the invariant — `memory.rs:195-220`
assigns under the same lock it appends under, and so does every other planned
adapter. The instruments-before-adapters framing is correct.

**The runway's refusal to settle the flavour question by argument (`:471-478`).**
Given §1 above, this is the position the evaluation should have taken.

**`research-dcb-spec.md`'s clause matrix, on the letter of the specification.**
Every MUST is implemented. Two rows need caveats (§3); no row is wrong about
conformance.

**ADR-0001's design, ADR-0003 and ADR-0004.** The two-flavour scheme's coherence
and composition hold. `serde` is genuinely absent from every default path
(`Cargo.toml:26-33`, with `serde?/std` as a weak forward from `std`), and the
`memory` feature is fully dead-code-eliminated. The MSRV of 1.85 is coherent
against every non-dev dependency in `Cargo.lock`.

**The gap-permission rule.** Honoured throughout `suite.rs` — every position
assertion anchors on a value the store assigned. This is the suite's best
property and nothing in either document threatens it.

---

## 3. What must be corrected

### 3.1 The evaluation contradicts itself three ways on `Error: Send + Sync`

`§2`'s `AppendError<E>` row defends keeping `E` generic because it "avoids
forcing a `Send + Sync` bound the wasm flavour may not want". `§3.1`'s verdict
(b) imposes exactly that bound on both flavours. `§9`'s immediate action 4
(`:1017-1020`) tells the reader to make the change now. `revised-runway.md:207`
records the same question as **open — highest blast radius in the workspace**.

Adjudication: the runway is right and §2 is right; §3.1's verdict and §9's action
must be withdrawn to "open, decided in ADR-0008 against a compiled `!Send` error
shape". The wasm half of this bound is the one claim in (b) with no in-tree
evidence — `MemoryStoreError` is uninhabited (`memory.rs:143-145`) and
`SqliteEventStoreError` is a unit variant (`happenstance-sqlite/src/event_store.rs:49-56`),
so both "confirmations" of (b) are free by construction.

### 3.2 Option (b)'s cost is understated, and the option list is not exhaustive

**[adjudicated by compilation]** `trait_variant` appends the attribute's whole
bound list to every returned future *and* to `read`'s stream. With
`make(SendFoo: Send)`, a stream whose hidden type is `struct NotSyncStream(Cell<u32>)`
— `Send`, not `Sync` — is accepted. Change the attribute to `Send + Sync` and the
same impl fails:

```
error[E0277]: `Cell<u32>` cannot be shared between threads safely
note: required because it appears within the type `NotSyncStream`
```

So (b) is not "one line each" and not "nearly free": it demands `Sync` futures
and a `Sync` stream from every adapter, and nothing in the workspace has been
checked against that. It also is not "one line" at the impl sites, which spell
the bound by hand — `memory.rs:154` and `happenstance-sqlite/src/event_store.rs:65`
both write `+ Send` today and would both need `+ Send + Sync`.

Two options the table never lists, both of which I compiled: put `where Self: Sync`
on the provided method (works, and leaves the stream alone), or make `head`/`count`
required now — there are exactly two impls in the workspace and one is `todo!()`.

### 3.3 §3.5's foreign-batch remedy does not work

**[adjudicated by compilation]** The hazard is real: `commit`'s elided batch
lifetime (`projection.rs:109-114`) is a fresh method-level parameter, so
`b.commit(a.begin().await?, …)` type-checks. But the evaluation names the wrong
cause, and the obvious fix fails. A port declared

```rust
fn begin<'a>(&'a self) -> Self::Batch<'a>;
fn commit<'a>(&'a self, batch: Self::Batch<'a>);
```

— the batch tied to the receiver's lifetime — still accepts
`let batch = a.begin(); b.commit(batch);`. It compiles. A lifetime names a
region, not an instance, and two `&Store` references unify to a common region.

Only a generative brand (an invariant lifetime the caller cannot unify) rejects
the call. The owned-`Batch` shape the evaluation proposes does not reject it
either; `:361-364` is candid that it works because the adapter never hands out a
live transaction, which is the `Vec<PendingWrite>` *discipline*, not the type.
`revised-runway.md:1090-1093` then endorses `sqlx::Transaction<'static, Postgres>`
as the model owned batch — a live transaction — and asserts unrepresentability
eleven lines later at `:1101-1104`. That sentence is wrong in the runway and must
be deleted; the evaluation's version is defensible and its wording should be
carried over.

### 3.4 ADR-0007 overstates what cannot be written; the suite is what cannot

**[adjudicated by compilation]** `ADR-0007:38-40` says "The runner ADR-0006
relocated therefore cannot be written against the port as it stands — in either
crate", and `RUNBOOK.md:205-208` turns that into phase-2 work. I compiled the
ADR's own indicative pump, with a real body, against `projection.rs` unchanged:

```rust
pub async fn pump<P, F>(store: &P, id: &ProjectionId, events: &[SequencedEvent], mut apply: F)
    -> Result<(), P::Error>
where P: ProjectionStore, F: FnMut(&mut P::Batch<'_>, &SequencedEvent)
```

`checkpoint` → `begin` → per-event `apply` → `commit`/`rollback`. It builds. The
callback's caller knows the concrete `Batch`, so the port needs no `apply` seam
to carry a runner.

What genuinely cannot be written is the **conformance suite**. Generic suite code
holding a `P::Batch<'_>` can only pass it to `commit` or `rollback`, so three of
the six rules phase 5 specifies (`revised-runway.md:1121-1126`) — rollback leaves
both unchanged, a dropped batch leaves both unchanged, a failed commit leaves the
store unchanged — cannot observe the read model at all. The port exists to defend
read-model write and checkpoint write in one transaction (`projection.rs:13-30`),
and the suite that freezes it can test only the second conjunct. By CLAUDE.md's
own corollary that is a decorative suite: it cannot reject an adapter that
commits the checkpoint and silently drops the read-model write.

Restate the decision as: the port needs a write seam **because the suite needs
one**, and the suite additionally needs a fixture trait supplying
`write_probe(&mut Batch)` and an out-of-band `read_probe(&Store)`. Correct
`ADR-0007`'s Context to "a runner that itself writes into the batch cannot be
written; a callback-driven one can", and reopen `RUNBOOK.md:68`.

### 3.5 The `ProjectionStore` port cannot be implemented without an undocumented spelling

**[adjudicated by compilation]** Writing the impl with the concrete batch type —
`async fn commit(&self, _batch: MyBatch<'_>, …)` — fails:

```
error[E0195]: lifetime parameters or bounds on method `commit` do not match the trait declaration
 --> crates/happenstance-core/src/projection.rs:109:5
```

Only the literal `Self::Batch<'_>` spelling compiles. `grep -rn "ProjectionStore for" --include=*.rs .`
returns nothing across the whole workspace, so no in-tree impl and no doctest
shields an adapter author from it. This is the concrete reason nothing has been
built against the port, and it is a stronger argument for reopening the port's
ledger row than the apply-seam claim the ADR gives. It appears in neither
document's defect list.

### 3.6 The ADR queue collides with an accepted ADR — in both documents

`docs/adr/0007-projection-runner-decodes.md` exists, `Status: accepted`, dated
2026-08-06, committed at HEAD as `2a65d76`. `ARCHITECTURAL-EVALUATION.md:115`,
`:812`, `:927` and `:1019` allocate the number 0007 to the async-port decision;
`revised-runway.md:162` and nine other places do the same, and the queue runs to
0023.

Renumbering the runway alone leaves the document that regenerates the collision
untouched. Both must be fixed. The consequences beyond arithmetic:

- `revised-runway.md:1127` schedules "Move the projection runner into
  `happenstance-core` per ADR-0006" — ADR-0007 splits it, pump in the contract
  crate, `Projection` and the application-facing runner in the typed layer
  (`0007:47-50`). The item is half right and needs rewriting, not deleting.
- `revised-runway.md:1172-1173` justifies phase 6's dependency graph on a claim
  ADR-0007 undid.
- Once `:1127` is corrected, the half of the runner an application actually calls
  has **no owning phase**: phase 6's work list (`:1199-1247`) never mentions
  `Projection` or a runner.

### 3.7 The runway's ledger drops four live rows from the RUNBOOK's

I compared the two ledgers row by row. `RUNBOOK.md:61-80` carries four rows that
`revised-runway.md:205-239` does not:

| Dropped row | RUNBOOK | Why it cannot be dropped |
|---|---|---|
| The `apply` seam (**decided** — the port grows it) | `:68` | Phase 5 freezes the port; ADR-0015 is scoped to ownership and drop only (`:160-178`), so the write seam is missing from the ledger *and* the queue |
| Read-model lifecycle / reset for a rebuild (open) | `:69` | Adding `reset` after the freeze is breaking, and rebuild is the commonest read-model operation |
| Where the runner lives (**decided**, cites the real ADR-0007) | `:72` | The only row in the workspace citing ADR-0007, and the only owner of its falsifier at `0007:119-121` |
| Projection failure policy (open) | `:73` | A poisoned projection has no defined behaviour and `PumpError` has no variant for a decode failure |

The runway also drops `RUNBOOK.md:80`'s warning that deferring the sync port
"leaks `EventId` and a tail seam back into `EventStore`" — the precise coupling
the runway's two largest deferrals (sync to phase 11, tail seam to phase 8)
depend on being absent. Claims of completeness at `revised-runway.md:1793-1794`
cannot stand while five rows are missing.

### 3.8 Individual defect entries needing re-costing or re-anchoring

- **D1.** The JSON cost is understated ~2.5x: a bare `Event` goes from 35 to 61
  bytes, not "≈ 10". The five attributes live on three wire structs, not the five
  types `revised-runway.md:992-994` names. D1's fix path and D13's fix command
  both say `happenstance-core`, a package that does not exist
  (`crates/happenstance-core/Cargo.toml:2`) — they presuppose the §3.6 rename.
- **D1 + D6 must land together.** `AppendCondition::Wire.fail_if_events_match`
  (`append.rs:119-122`) carries no `#[serde(default)]`; `from_str::<AppendCondition>("{}")`
  succeeds only because `Query`'s `Deserialize` is `Option`-shaped. Once D6 makes
  `Query` explicitly tagged, `{}` becomes a hard error unless a default is added
  deliberately. Neither entry says so.
- **D6 reverses a documented decision.** `query.rs:306` states the intent —
  "`None` is the match-all query; `Some(items)` is a filtered one". The ADR that
  changes it must say it is reversing a choice, not fixing an oversight.
- **D7 is adjudicated in the evaluation and reopened in the runway.**
  `ARCHITECTURAL-EVALUATION.md` picks an answer; `revised-runway.md:733-749` runs
  it as an open ADR with three live options and a third adapter shape to decide
  against. The runway's version is better; §4 of the evaluation should defer.
- **D9's fix is not "one extra check, no dependency".** `core` exposes no
  general-category predicate, and blanket `Cf` rejection bans U+200C/U+200D,
  which Persian, Hindi and emoji sequences require. The runway
  (`revised-runway.md:960-973`) already states the honest version; the evaluation
  must be brought into line with it. The doc-wording half (four sites say
  "ASCII"; `char::is_control` is Unicode Cc) is a genuine two-word fix.
- **D10 and D11 interact.** `include_str!("../README.md")` from
  `crates/happenstance-core/src/lib.rs` resolves to the *per-crate* README that D11
  tells you to create — not the workspace README whose Quick start is broken. The
  plan must name which file CI doctests and say whether the other is duplicated
  or left unverified. Phase 0's exit criterion at `revised-runway.md:413` is not
  achieved by the tasks above it. The span is `README.md:89-102`, not `87-100`,
  and the wrong span is copied into `revised-runway.md:364`.
- **Citation drift.** `store.rs:103-108` not `:103-110`; `RUNBOOK.md:70` for the
  identity row, not `:67`; `RUNBOOK.md:433` for the release hazard, not `:390`;
  `RUNBOOK.md:457-464` for the bias diagnosis the runway cites as `414-421`;
  `tag.rs:47`/`event.rs:37` for the "ASCII" wording. The drift is confined to
  prose spans — the three claims I checked hardest (`cargo package --list`, the
  three `cargo doc` errors, the `cargo tree` feature edge) are exact. For a plan
  meant to be executed against exact spans, re-anchor everything before adoption.

### 3.9 Conformance-suite gaps the plan does not schedule

Verified by reading `suite.rs`. Every append-condition call site builds its
condition from `query_of_types` — `:390`, `:447`, `:466`, `:483`, `:501`, `:519`,
`:537`, `:561`. The one tagged condition, `:603-605`, runs against a store
holding one untagged `CourseDefined` (`:600`) and one tagged `StudentSubscribed`
(`:607`), so a type-only probe returns identical verdicts at `:612` and `:621`.
**No rule's verdict depends on the append-condition path matching tags.** An
adapter that drops the tag join from its condition probe — the natural first cut,
since the join is the expensive half and `happenstance-sqlite/src/event_store.rs:21-27`
puts tags in a separate table — passes the whole suite while rejecting every
command that touches any course. That is a total-availability failure certified
as conformant, and it is the canonical DCB uniqueness shape.

The planned condition-shape rules (`revised-runway.md:818-820`) catch the
fail-open direction by accident. The over-reject direction needs its own rule:
two events sharing a type and differing in tags, a condition tagged for one of
them, asserting rejection — and its mirror asserting acceptance for a tag no
event carries.

Three smaller ones in the same family: `Query::all()` is never read on a store
holding a multi-tagged event, so a join without `DISTINCT` passes; no rule
evaluates a condition against an *empty* store, and
`ARCHITECTURAL-EVALUATION.md`'s C7 rule for that case is the one value-edge rule
`revised-runway.md:857-860` drops; and `positions_are_unique` /
`positions_are_strictly_monotonic` (`suite.rs:336-365`) both read back through
`read`, which every adapter returns in position order, so both are vacuous for
any sorted read path. The two rules that actually pin cross-batch ordering are
append-condition rules.

### 3.10 Smaller corrections

- **`§3.4` closes a live ledger row in half a sentence.** "Do **not** add … a
  subscription API — all measured as already optimal or premature", and
  `:903` says the tail seam is "**absent, and not even in the ledger**".
  `RUNBOOK.md:66` is that row. Fix `:903` and `:1038` before anyone acts on "add
  the missing ledger rows".
- **`§3.3`'s first argument attacks a framing the RUNBOOK already retracted** at
  `:450-455`. Its second argument (the constructor-arity funnel) is its own and
  is load-bearing. Add the cross-reference; keep the second.
- **`§3.7`'s `from_static` cost/benefit is inert without D4.** A
  `const COURSE_DEFINED: EventType` does not compile at `Event::new`
  (`E0271`, pointing at `event.rs:198`). D4 fixes it; §3.7 never cross-references
  D4. Ship them together.
- **F1's cost argument is refuted by its own citation.** The DCB reference
  probes `backwards, limit 1`, so `after: None` costs the same as `after: Some(h)`.
  F1's *title* bakes the wrong reason in ("forcing whole-log condition checks");
  rewrite the title to the expressiveness claim, which survives untouched.
- **`limit(0)` matches the reference implementation.** `InMemoryDcbEventStore`
  treats `limit: 0` as unlimited through JavaScript falsiness. D5 is still worth
  fixing, but it is a deliberate divergence from precedent, not a correction of
  one, and the ADR should say so.
- **`revised-runway.md` propagates the vacuous test it was told to fix.**
  `:1756` restates constraint 3 as "A unit test asserts this. (ADR-0001)" while
  `ARCHITECTURAL-EVALUATION.md:135-137` shows `memory.rs:328-340` passes by
  auto-trait leakage on a concrete type. `CLAUDE.md` constraint 3 currently
  protects a test that cannot fail.
- **CLAUDE.md and `happenstance-runtime/src/lib.rs` contradict two accepted
  ADRs.** ADR-0006 decided `happenstance-runtime` ceases to exist; CLAUDE.md
  still lists it as "named seam" and still carries "whether `happenstance-runtime`
  is the right name" as an open question. An accepted-but-unexecuted rename is
  already producing wrong instructions to the two documents an agent loads first.
- **`crates/happenstance-sync/src/lib.rs:94-96` claims a skeleton that does not
  exist** ("which is why a skeleton for it exists before the trait does").
  `ls crates/` returns six directories; there is no `happenstance-neon`. That
  text is an uncommitted edit from the evaluation pass itself.
- **CLAUDE.md's tooling note is false.** `cargo hack 0.6.45` and
  `cargo deny 0.20.2` both resolve on this machine, so the two steps run rather
  than printing `skipped`.
- **Estimates.** Serialising phase 4 and re-estimating phases 0 and 3 gives
  ~10.4 weeks, not ~8 — enough to erase the claimed advantage over the old
  ordering. The "11–12 weeks" baseline is not derivable: `RUNBOOK.md` contains no
  estimate anywhere in its 529 lines. And the two 0.1s are not like-for-like —
  the old one shipped the projection port frozen against a real adapter. State
  the honest figure or drop the comparison and keep the ordering argument, which
  stands on its own.

---

## 4. What must be discarded

**`ARCHITECTURAL-EVALUATION.md:22-32` — the verdict's central claim.** "…because
`#[trait_variant::make(SendEventStore: Send)]` at `store.rs:91` silently forbids
`EventStore` from ever gaining a defaulted method, which means `head()` and
`count()` can never be added". Compiled and refuted (§1). Delete the clause and
re-derive the verdict. What survives of §1 is the sequencing thesis and the
observation that nothing is published, so every fix is free today.

**`§3.1`'s "Consequence of leaving both" (`:90-94`) and its verdict (`:104-117`).**
The first half of the consequence is false. The second half — the `Error` bound —
is real but is contradicted by §2 and by the runway, and must revert to open. The
three-option table is not exhaustive; two cheaper options exist and both compile.
Keep the two mechanical facts, the CONTRIBUTING note, and the falsification test.

**`§3.4`'s "strictly blocked on §3.1".** False. `head`/`count` are shippable
without touching the attribute.

**`§9` immediate action 4 (`:1017-1020`), "Change one line and one bound".**
Withdraw; it executes a verdict that no longer holds and imposes a bound the
runway explicitly refuses to settle by argument.

**`revised-runway.md:1101-1104`'s "This also makes the foreign-batch hazard
unrepresentable".** Compiled and refuted (§3.3). The evaluation's wording at
`:361-365` is the correct one.

**`revised-runway.md:1127` as written** ("Move the projection runner into
`happenstance-core` per ADR-0006"). Contradicts an accepted ADR.

**`revised-runway.md:1172-1173`'s dependency rationale.** Obsolete: ADR-0007
puts the application-facing runner in the typed layer, so phase 6 genuinely does
need `ProjectionStore` frozen. The table is right; the paragraph explaining it
argues for removing the dependency the table keeps.

**`revised-runway.md:588-593`'s Neon claim** — "the only adapter that satisfies
both flavours from one body of code". `store.rs:17-18` documents the implication
table: implementing the bare `EventStore` satisfies exactly one flavour, and
satisfying both needs `impl SendEventStore`, which cannot compile on
`wasm32-unknown-unknown`. Correct it to "compiles for both targets, implementing
the bare flavour on each", which is a weaker and different data point.

**`revised-runway.md:625-633`'s phase-2 exit criterion.** "A table with no
rejections means the skeletons were not ambitious enough: go back and try … until
they fail." An exit criterion that mandates its own outcome, in a plan whose own
rule (`:34-39`) is that a proof must be an artefact that would not exist if the
design were wrong. The document half-notices at `:297-299`. Rewrite as: for each
skeleton, record the most ambitious signature attempted and its outcome —
rejection with the compiler error, or agreement.

**`ARCHITECTURAL-EVALUATION.md:903`'s "not even in the ledger"** and the
instruction it generates at `:1038`. `RUNBOOK.md:66` is the row. Three of the
four "missing ledger rows" exist; only `recorded_at` is genuinely absent.

**The A16 promotion argument.** The deserialisation-bomb framing at `:817` does
not survive testing — bincode 1 does not honour a length prefix as a reserve, and
serde_json's allocation is proportional to input already sent. What survives is
the meta-point: §4's inclusion criterion is unstated, which is why a
2^64-positions overflow with "blast radius: nil in practice" is a numbered defect
and a validation-ordering issue on the replication path is a table row. State the
criterion; do not promote A16 on a severity claim whose premise is unverified.

**The "8 of 10 injected bugs pass" headline.** A pass rate over an
author-chosen bug set carries no information the 4-of-6 figure did not. The four
new stores are real and belong in the harness; the *rate* is a selection artefact
and must not be quoted.

---

## 5. Ranked open issues, by blast radius on the architectural specification

**1. The trait-derivation decision — `EventStore` and `ProjectionStore` both.**
Shapes every signature in the workspace. Now genuinely open, on evidence rather
than on the false premise that closed it. Four surfaces reduce to the same
requirement — the provided-method body, the extension trait, `tokio::spawn`, and
generic helpers all need `Self: Sync`, and all three can take it at the point of
use. The specification must state one rule: *any provided or extension body that
holds `&self` across an await requires `Self: Sync` at the point of use.* It must
also decide `Error: Send + Sync` separately from the trait bound, and it must
cover `ProjectionStore`, which carries the identical construction at
`projection.rs:70` and appears in no ADR at all. And it owes an obligation nobody
has written down: `variant.rs` clones a provided body into the variant, so one
body must type-check under both flavours' bounds simultaneously.

**2. Event identity and the ingest path — one decision, currently two.**
`revised-runway.md:213` and `RUNBOOK.md:70` both fix identity as store-assigned
on `SequencedEvent`. `Event` carries no identity (`event.rs:183-188`) and
`append(&self, events: &[Event], …)` (`store.rs:141-145`) has no slot for a
foreign one, so an ingesting peer's store assigns its own and
`happenstance-sync/src/lib.rs:73-77`'s dedup requirement has nothing to work
with. §3.2 freezes `append` in phase 3 and §3.3 chooses identity in phase 4, so
phase 4 reopens phase 3. There is a non-breaking third option neither document
considers — an `IngestStore` trait defined in `happenstance-sync` itself, which
is the port-in-the-sync-crate posture the runway already adopts at `:1610-1617` —
but nothing records it, and the ledger row says "store-assigned", which is
exactly the property an ingesting peer must override. The claimed three-week
saving (`revised-runway.md:44-46`) rests on this being a non-issue.

**3. The projection port. Four coupled defects, one freeze.** The write seam the
suite needs (§3.4); the fixture shape that would let the suite observe a read
model; the foreign-batch hole and its non-remedy (§3.3); and the E0195 spelling
trap (§3.5). Add to those the undecided questions neither document carries:
whether `commit` may name a position no applied event occupies, whether a
checkpoint may move backwards (rebuild), whether a `Batch` is read-your-writes
within a chunk, and whether two views in one store can advance in one
transaction. Phase 5 as scheduled freezes this port against `MemoryProjectionStore`
and an in-process rusqlite transaction — one shape, the exact monoculture the
plan's own portfolio table convicts at `:252`.

**4. The wire format.** D1 (non-self-describing formats broken), D6 (`Query::All`
→ `null`, `{}` → match-everything), D12 (`serde/alloc` arriving only via
`bytes`), plus two things neither document states: the encoding diverges from the
DCB reference's published shape (`{items: […]}` versus a bare sequence, and
happenstance cannot parse the reference's match-all `[]` at all), and every new
bound the plan proposes — `MAX_EVENT_DATA_LEN`, `MAX_TAGS`, `MAX_QUERY_ITEMS` —
is a wire-compatibility break with no quarantine path, because `AppendError` has
no variant meaning "refused, park this". The format has zero conformance rules
and zero implementers, and `happenstance-sync/Cargo.toml:17` already depends on
it non-optionally. Phase 4 must decide whether the wire is happenstance-private
or DCB-interoperable; today it is neither.

**5. The conformance suite's real coverage.** §3.9: the append-condition path
never requires tags, both position rules are vacuous, no condition is evaluated
against an empty store, and no rule survives a store reopen. The suite is the
project's differentiator and the thing every adapter is measured against; a gap
here propagates into every adapter written afterwards.

**6. ADR numbering and ledger reconciliation.** Mechanical, but it gates
everything else, because the deliverable of the next pass is ADR bodies. Renumber
0007→0008 in both documents, restore the four dropped RUNBOOK rows plus the
"leaks `EventId` and a tail seam" warning, add an ADR for the projection write
seam, give the typed runner an owning phase, and add the D-number to each runway
work item so "is every confirmed defect scheduled?" is answerable mechanically.
(It is, today: D10–D13 in phase 0, D5/D2/D3 in 3a, D7/D8 in 3b, D4/D9/D1/D6 in 4
— reconstructed by prose search, which is the problem.)

**7. The remaining defects.** D2–D5, D8–D13, and the checkpoint-resume off-by-one
neither document carries: `projection.rs:84-93` tells the reader to "advance past"
the checkpoint by hand because `ReadOptions::from` is inclusive
(`query.rs:227`, `store.rs:116`) while `AppendCondition::after` is exclusive
(`append.rs:103-106`),
and the only API for advancing is `SequencePosition::next()` — the exact method
D3 says returns the wrong answer where it can fail. `revised-runway.md:711-716`
schedules the fix; the defect list does not carry it.

**8. Scheduling defects.** Phase 1 cannot exit (its ADR evidence is required to
come from phase 2, which must not precede it — amend phase 1's work list, which
already contains a throwaway provided-method probe, to carry two throwaway
error-shape probes). Phase 8 depends on 2 and 3 but its schema needs phase 4's
columns. Phases 8 and 9 have no exit criteria. Phase 3 splits cleanly into a
behaviour half with no dependency on phase 2 and a signature half that has one.
The tail-seam row names phase 8 as owner and phase 8's work list has no item for
it. Phase 0 says seven crates.io names and lists ten.

---

## 6. Scenario-relevant findings — the input to e2e design

Grouped by the scenario that would exercise them, not by finding.

**S1 — Wire round-trip across three formats and two peers.** D1 (five
`skip_serializing_if` attributes; sparse shapes fail in postcard and bincode),
D6 (`Query::All` → `null`; `Some(Query::All)` indistinguishable from `None`;
`from_str::<AppendCondition>("{}")` → match-everything), D12, the missing
`#[serde(default)]` on `fail_if_events_match`, the divergence from the DCB
reference's `{items: […]}` shape, and the fact that new bounds are a wire break
with no quarantine variant. The scenario: encode every envelope shape in
JSON, postcard and bincode; decode on a peer built at a different bound; assert
byte-identical payload and structurally-equal envelope.

**S2 — Idempotent ingest between two stores.** Event identity is store-assigned
but must survive ingest; `append(&[Event])` has no slot for a foreign `EventId`;
re-delivery must be harmless; there is no transactional home for a per-peer
watermark; canonical `Tags` re-sorting means a foreign peer's hash over its own
tag order will not agree. The scenario: peer A appends, peer B ingests twice,
assert one event and a stable identity. This scenario is **currently unwritable
against either specification**, which is the finding.

**S3 — Conditional append under contention, tags load-bearing.** Two events
sharing a type and differing in tags; a condition tagged for one; assert
rejection, then the mirror asserting acceptance for a tag no event carries. Plus:
a condition evaluated against an empty store; `Query::all()` used as a condition;
`after` pointing beyond the head; the empty-batch/condition precedence
(`memory.rs:197-216` returns `ConditionViolated` where `NoEvents` is the caller's
actual bug); the retry-safety property, which holds only for a *verbatim*
resubmission and must not be collapsed with the `ConditionViolated` re-decide
path.

**S4 — Read laziness and isolation.** D7: `store.rs:103-108` promises the stream
is lazy; `memory.rs:155-181` snapshots under the lock at call time; the observable
difference is one event versus two. Note that writing the discriminating rule
requires holding the query in a named local, because `read`'s opaque return
captures the `&Query` lifetime (E0716) — a signature change that must land before
or with the rule.

**S5 — Projection through a real transaction.** The suite cannot write a read
model, so the one-transaction invariant is untested; `commit` accepts a foreign
batch; `Batch` carries no `Send` bound on the `Send` flavour even though `commit`'s
future does; two events touching the same row inside one chunk both read the
pre-batch value; two views in one store cannot advance in one transaction; a
decode failure has no variant in `PumpError`; a rebuild cannot be expressed.
The scenario: a counter projection over two events in one chunk, plus a rebuild,
plus a poisoned event.

**S6 — Durability across a process boundary.** `conformance_test!`
(`testkit/src/lib.rs:83-91`) re-evaluates `$factory` per test, so no rule can
hold two handles: durability, reopen and genuine multi-connection rules are all
foreclosed by the fixture shape. Nothing in the workspace can fail a store that
returns `Ok` from `append` and loses the write on reopen.

**S7 — `limit` and `from` composed with filtering.** `limit` is the only read
option tested against a filtering query, and it is the one that is
cardinality-sensitive. `from` commutes with filtering *semantically* but not in
generated SQL: `WHERE a OR b AND position > ?` without parentheses is a textbook
precedence bug, so a `from` × multi-item-query rule can fail and must be written.

**S8 — Batch and query size.** The number of events per append is unbounded, and
`EventBatch`'s constructor is the only place a `MAX_EVENTS_PER_BATCH` can ever be
enforced — a public constant that freezes with everything else.
`SQLITE_MAX_VARIABLE_NUMBER = 32766` applies to a multi-row INSERT exactly as it
applies to tags, so an over-large batch fails at write time after the caller has
decided.

**S9 — `Event::new` with a held `EventType`.** D4 and §3.7 together: the typed
layer interns one `EventType` per `DomainEvent`, and today that value cannot be
passed to `Event::new` (E0271 at `event.rs:198`). This must land before phase 6
writes any code against `Event::new`.

**S10 — AC3's boundary.** `after` is exclusive in happenstance
(`append.rs:103-106`) and in the reference; the spec's verbatim text says "ignore
the Events **before** the specified position", which is a lower bound on what
must be ignored rather than a contradiction. Underspecified, not divergent — but
an off-by-one here is a silent lost update once ingest re-checks conditions, so
whichever ADR settles ingest must name the reading explicitly.

---

## 7. What only an experiment can settle

These are the reason the e2e scenarios exist. No amount of document review moves
any of them.

**Does any real adapter produce a `!Sync` future or a `!Sync` stream?** Option
(b) demands both. `MemoryEventStore`'s `Snapshot` is `Sync`, which is exactly why
the evidence base missed it. rusqlite, tokio-postgres and a Workers HTTP client
have never been checked. One skeleton per shape, declaring its real stream and
future types with `todo!()` bodies, answers it in an afternoon — and until it is
answered, ADR-0008 has no evidence.

**Does stringifying a `JsValue` lose information the caller needs?** This is the
whole cost of `Error: Send + Sync`, and `worker::Error::JsError(String)` suggests
the answer is no. The Cloudflare skeleton is the only instrument.

**Which Postgres mechanism buys the position-visibility invariant, and at what
cost?** `xid8` + `pg_snapshot_xmin`, transaction-scoped advisory locks, and a
serialised sequence table each cost something real. This is owed a measurement,
and it is the one place where the roadmap already says so.

**Does the checkpoint pump acquire an independent caller?** `ADR-0007:119-121`
sets the falsifier itself — "if it has acquired no independent caller by the time
the typed layer's phase exits, collapse it upward and supersede this ADR". No
phase-6 exit criterion evaluates it. Add one.

**Can a `Vec<PendingWrite>` batch carry a real adapter, or does something need a
live transaction?** The owned-`Batch` verdict rests on the buffering discipline.
Ladybug's graph writes and a Postgres transaction are the two shapes that could
refuse it, and neither has been attempted.

**Do two peers converge if ingest re-checks append conditions — and does the
consistency boundary survive if it does not?** An `AppendCondition` is defined
against one store's positions, so two peers each holding no matching event both
accept the same conditional append legitimately. Re-checking can then only reject
an event already durable at its origin, and the logs diverge permanently with no
error variant to report it. Not re-checking breaks the invariant with nothing
able to detect it. Only a two-peer scenario that asserts on convergence *and* on
the invariant discriminates, and the answer probably renames the question to
"which writes are permitted on which peer".

**Is replication whole-log or scoped?** A spoke holding a filtered subset cannot
distinguish "not yet received" from "filtered out", so a position-based resume
watermark against a hub is unsound. If it is scoped, the peer port carries a
`Query` — a value that bounds nothing and lands in a write path evaluated under
the write lock. This decides whether the sync suite's round-trip rule can assert
log equality at all.

**Does a projection reading its own read model inside a chunk lose a write?**
Two events touching the same row in one `begin`/`commit` cycle. The three
candidate answers — the `Batch` is read-your-writes, the runner guarantees one
event per batch, or a projection may not read what it writes — are all design
decisions, and the scenario is what forces one.

**Does any store lose an acknowledged write across a reopen?** Nothing in the
workspace can currently express the question, because the fixture takes one
handle. This is the axis with nothing at either end.

**Is `cargo test --workspace` green at the 1.85 MSRV?** `proptest 1.11.0` and
`getrandom 0.4.3` both declare `rust-version = 1.85`, and CI's
`--no-dev-deps` (`ci.yml:62`) cannot see them. Zero headroom, verified nowhere.
One CI job settles it.
