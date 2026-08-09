# Phase 4 reconciliation

## What this is

The dossier ADR-0011 through ADR-0015 are written from. Phase 4 freezes
`EventStore`'s signatures, its semantic promises and the values they carry, and
its RUNBOOK body was written before `SPECIFICATION.md` existed in its current
form. Where the two disagree, [RUNBOOK rule 5](../RUNBOOK.md#L56) makes the
specification win and the phase body is fixed in the same commit — but the phase
body is where the *work* is enumerated, so a drafter reading only the clauses
will implement less than the phase owes, and a drafter reading only the phase
body will implement things three `[FROZEN]` clauses forbid. This document
reconciles them, records every compile that settled a question, and hands each
ADR a brief. It decides nothing: every resolution below is presented with its
cost and no recommendation, because the ADRs are where those are made.

Three inputs fed it — a clause-by-clause conflict audit, an evidence sweep and
nine compiled experiments — and each was adversarially re-checked against the
tree before anything here leaned on it. The corrections that pass produced are in
[their own section](#corrections-this-pass-made-to-its-own-inputs) rather than
absorbed silently, for the reason [RUNBOOK.md:73-75](../RUNBOOK.md#L73) gives.

**Nothing here amends `SPECIFICATION.md` or `RUNBOOK.md`.** Every spec amendment
this pass identified is recorded below as owed work and applied in the run that
writes the code.

---

## What the specification already decides

This is the reassuring half. Of the sixty-four clause IDs phase 4 claims to
discharge (`RUNBOOK.md:2755`, "Discharges ES-8 – ES-40, VT-1 – VT-31"),
forty-six are already `[FROZEN]`. For those, phase 4's job is transcription and
implementation, not design. The ones where a drafter might mistake execution for
a decision:

| Clause | Already decided | What phase 4 does |
|---|---|---|
| **ES-9** (`:2302`) | `from` is a range predicate, never a seek; an unoccupied position yields the higher neighbour | Write `read_from_a_gap_position`; the one-sided-saboteur asymmetry ADR-0010 §1 requires is already recorded at `:2335-2346` |
| **ES-16** (`:2648`) + **VT-29** (`:1529`) | `ReadOptions::to` is an **inclusive** upper bound and lands *before* the first adapter ships. Both name the same three rules | Add the field and the three rules. `ReadOptions` is `{ from, backwards, limit }` today (`query.rs:226-234`) |
| **ES-21** (`:2892`) | A condition is evaluated only against events the store already held | Write `batch_is_not_evaluated_against_its_own_condition`. The rejected implementation — per-row `INSERT … SELECT … WHERE NOT EXISTS` — is already named at `:2903-2910` |
| **ES-23** (`:2930`) | "May or may not have committed" *is* the contract, and the port MUST carry an explicit `# Cancellation` section | Write the section. The clause carries **`Rule:` none** by design (`:2949-2953`) |
| **ES-25** (`:3043`) | `conflicting_position` is **informational**; an adapter that cannot identify a culprit MUST be permitted to report `None` and callers MUST NOT depend on it (`:3088-3091`) | The docstring already says this (`error.rs:99-103`). What is missing is the `Display` impl and the Neon citation |
| **ES-30** (`:3275`) | `head()` is a **required** method — not provided, because the only provided form needs `where Self: Sync` and the `!Send` edge adapter cannot satisfy it (`:3284-3288`). **`count()` does not ship** (`:3326`) | Add `head()` to the trait and to every impl. `grep -rn "fn head" crates/` matches nothing today |
| **ES-33**, **ES-34** | Two handles share one consistency boundary; discharged at phase 3, rules registered at `registry.rs:106-107` | Nothing. The RUNBOOK item at `:2853` is stale |
| **ES-37** (`:3607`) | `EventStore` is closed over insertion; no delete, truncate, redact or compact at 0.1 | Transcribe. Carries **`Rule:` none** by design (`:3625-3626`) |
| **VT-13** (`:1004`) | `next()` MUST return `None` on overflow; `checkpoint.next()` is the documented resume idiom and is sound over gaps | One line: `event.rs:143` is `saturating_add`, which can never signal |
| **VT-15** (`:1108`) | Equality is byte equality and **the contract normalises nothing** (`:1110-1113`) | Transcribe into `Tag`'s docs. The cheapest exit criterion in the phase |
| **VT-17** (`:1167`) | `key:value` is unenforced convention, repeated keys are legal, and **`Tags` MUST NOT offer a `get(key)` accessor** (`:1169-1171`) | Write the missing E2E case; §7.5 (`:8066`) already describes it |
| **VT-18** (`:1194`) | `Event::new` MUST accept an already-built `EventType`; the three validation errors compose through `From`, and the `InvalidInput` collapse is rejected by name (`:1231-1234`) | Loosen the bound, add `impl From<Infallible> for InvalidEventType` and an `InvalidQuery::Tag` variant |
| **VT-26** (`:1421`) | `Query::Items` MUST NOT be constructible outside the crate, with **variant-level** `#[non_exhaustive]` and the reason for variant over enum (`:1437-1444`) | Today `Query::Items(Box::new([]))` compiles downstream and `Default` is derived (`query.rs:143-150`), which falsifies the doc claim at `query.rs:122-123` |
| **VT-27** (`:1446`) | `Query` carries no positions; per-fragment bounds mean one read per fragment | Transcribe. Its `Rule:` is "compile-level" (`:1451`), and its behavioural half depends on VT-30 landing |
| **VT-28** (`:1495`) | `limit(0)` returns nothing, and the ADR must state the divergence from the DCB reference implementation | `query.rs:263` is `NonZeroUsize::new(limit)` under a doc comment saying zero is ignored |
| **VT-4** (`:683`) / **VT-5** / **VT-7** / **VT-8** | `SequencedEvent` carries position, `EventId`, `RecordedAt` and the `Event`, all four public, struct stays `#[non_exhaustive]`. `EventId` is `(StoreId, SequencePosition)` and is a newtype, not a tuple alias. It is **not queryable** and MUST NOT be a `Tag`; membership gets a dedicated port operation (`:791-798`) | Build them. None of `EventId`, `StoreId` or `recorded_at` exists in `happenstance-core/src` today |

Two consequences of that table are easy to miss.

**ES-30 grows the port a required method, and VT-7 grows it a second one.**
VT-7's answer to "is `EventId` queryable" is a dedicated operation checked by
`contains_event_id_reports_membership` (`:797-798`). Phase 4's signatures list
never mentions it, and the exit criterion that says `append`'s signature is
unchanged by identity is true of `append` and not of the trait.

**Six `[FROZEN]` clauses are unimplemented and have no rule.** VT-4 (three
store-assigned facts), VT-13 (overflow), VT-25 (`AppendError::ExceedsStoreLimit`,
which `error.rs:150-166` does not have), VT-26, VT-28 and VT-29. A `[FROZEN]`
marker binds the design; it does not assert the code exists.

---

## Where the RUNBOOK and the specification disagree

Six substantive collisions. Each is stated as the clause sentence, the RUNBOOK
sentence, and what each way out costs. No recommendation — that is the ADRs'
work.

### 1. `read`'s query: by reference or by value (ES-13)

**Clause** (`SPECIFICATION.md:2500`, `[FROZEN]`): "`read` MUST continue to take
`query: &Query` by reference." Its `Rejects:` line (`:2530-2532`) names the
opposing item verbatim: *"the 'fix' that changes `read` to take `Query` by value
in order to make the E0716 go away. It compiles, it breaks every adapter
signature, and it puts an allocation on the hottest path in the port."*

**RUNBOOK** (`:2768-2774`): "**`read` takes its query by value.** … This must
land *before or with* the read-laziness rule, which cannot otherwise be written."

The premise is refuted by the clause's own body (`:2504-2509`): the laziness rule
is written by binding the query to a named local, which is what the suite's
`read_ok` helper already does. The real cases the RUNBOOK is reaching for are the
proof artefact's — returning a stream from a function and storing one in a struct
field — and those genuinely fail today.

**Options and costs.**

- *Keep ES-13, discharge the cases with precise capturing.* Compiled: on a plain
  RPITIT trait, `fn read<'a>(&'a self, query: &Query, …) -> impl Stream<…> +
  use<'a, Self>` works on 1.97.1, the borrowing adapter compiles with `use<'a>`,
  and the escape case compiles with `&Query` untouched. **The cost is that
  `#[trait_variant::make]` cannot emit it** — E0799, reproduced twice
  independently — so `store.rs` must hand-write `SendEventStore` and the blanket
  impl, roughly twenty-five lines, and that directly contradicts ADR-0001's and
  ADR-0008's stated mechanism. It also taxes every non-borrowing adapter with a
  `use<'a>` bound or a `refining_impl_trait` warning under `-D warnings`.
- *Keep ES-13, rewrite proof cases 1 and 2 to bind a named local.* Costs phase 4
  two of its four proof cases and leaves the artefact proving one thing.
- *Supersede ES-13 with a by-value clause.* Compiled: it works under real
  `trait_variant` with no hand-rolling, and both `memory.rs` tests pass. The
  measured price is 9 allocations / 212 bytes per read for a two-clause query, 0
  for `Query::all()` — so a benchmark written against `Query::all()` measures
  nothing — plus `error[E0308]` at every caller holding only a `&Query`, which
  includes `read_decision_model` (`store.rs:205-215`) itself. Under RUNBOOK rule
  3 this is a new ADR, not an edit.
- *Record the RUNBOOK edits as owed.* `:2768-2774` and phase 3's
  `:1690-1692` ("every rule is re-spelled when `read` takes its query by value")
  both argue from the rejected shape.

### 2. `append`'s batch: borrowed or owned (ES-17)

**Clause** (`:2695`, `[PROVISIONAL]`): "`append` MUST continue to take `events:
&[Event]`", falsifiable only by "the SQLite adapter's multi-row insert benchmark,
in the phase that builds it" — **phase 8**.

**RUNBOOK** (`:2759-2767`): "**`append` takes its events by value** (D2's family)
… Choose between `impl IntoIterator<Item = Event>` and an owned batch type on the
phase-2 evidence."

Three separate problems. The evidence the item asks to choose on cannot exist at
phase 4: `happenstance-sqlite` is `todo!()` bodies and the workspace has no
benchmark harness at all — ES-32's marker says so in terms (`:3358-3362`). The
owned-batch option is forbidden in terms by VT-24 (`:1372-1374`): "no `EventBatch`
type MUST be introduced". And `impl IntoIterator<Item = Event>` is **compiled to
be unerasable** by `dynosaur` — `error[E0191]`, the associated `IntoIter` must be
specified — which is the escape hatch ADR-0001:110-112 names for the missing `dyn
EventStore`. The RUNBOOK's "D2's family" label is also wrong: `SPECIFICATION.md:1164`
and `:1421` make D2 the infallible-constructor mistake on `Query::Items`, which
`RUNBOOK.md:2800` fixes as a separate item nine lines later.

**Options and costs.** Defer to phase 8 and leave ES-17 `[PROVISIONAL]` (honest,
but freezes a signature the phase claims to freeze while leaving its marker
live); or lift ES-17 to `[FROZEN]` on the three grounds the clause already states
— refcounted `Bytes`, a rejected append cloning nothing, and `ConditionViolated`
obliging the caller to keep its events anyway — and record that the phase-8
benchmark may reopen it as a contract change with its own ADR. Note that **no
rule in the suite would fail if `append` changed to by-value**: ES-17 cites
`append_preserves_event_payload`, a round-trip that passes identically either
way. The sentence has no falsifier of its own in either direction.

### 3. `head()` and `count()`: required, provided, or on an ext trait (ES-30)

**Clause** (`:3277-3288`, `[FROZEN]`): `head()` "MUST be a **required** method,
not a provided one", because the only provided form needs `where Self: Sync` and
the `!Send` edge adapter — the entire reason the bare flavour exists — is
`!Sync`. And (`:3326`) "**`count()` does not ship, and this is a decision rather
than an omission.**"

**RUNBOOK** (`:2779-2784`): "**Provided `head()` and `count(&Query)` on the port
itself**, hand-desugared per ADR-0008. These must *not* go on the blanket ext
trait: a blanket impl cannot be overridden."

The item's argument is an argument *against* the ext trait, not *for* a provided
body, so the two agree about where the methods do not go and disagree about
whether the body is defaulted. The non-overridability the item asserts was
compiled and holds twice over: `error[E0119]` inside the crate owning the trait,
`error[E0117]` (orphan) from anywhere else. **The compile consequence of the
frozen shape is that all six skeletons plus `MemoryEventStore`,
`LocalMemoryEventStore` and the testkit's mutants gain a body** — ES-30 already
costs this at seven conformant impls and three excluded ones (`:3288-3304`) — and
that is what makes exit criterion 4 unsatisfiable as written.

### 4. The umbrella validation error (VT-18)

**Clause** (`:1231-1234`, `[FROZEN]`): "Collapsing the three enums into one
`InvalidInput` was the alternative and it loses", with three reasons; what it
mandates instead is composition — `InvalidQuery: From<InvalidTag>` plus a new
`InvalidQuery::Tag` variant.

**RUNBOOK** (`:2923-2927`): "**One umbrella validation error** (E2E-51)."

The problem is real: `error.rs` gives `InvalidQuery` a `From<InvalidEventType>`
and a `From<Infallible>` and nothing for `InvalidTag`. The named fix is the
forbidden one. Two costs to weigh: reading "umbrella" as loose wording for
composition closes E2E-51 without touching a frozen clause; but VT-18 says
nothing about `AppendError<E>`, which is the fourth type the worked example must
unify, so composing the three validation errors still leaves a seam and the ADR
owes an explicit statement about whether it closes or stays open.

### 5. `Tags::value_of(key)` (VT-17)

**Clause** (`:1170-1171`, `[FROZEN]`): "`Tags` MUST NOT offer a `get(key)`
accessor", because (`:1188-1191`) "an accessor that returns one value where two
may exist would make the ambiguity invisible rather than resolving it".

**RUNBOOK** (`:2985-2988`): "`Tags::value_of(key)` as a `partition_point` on the
`"key:"` prefix — O(log n) rather than the O(n) scan the worked example writes by
hand twice … Decide whether repeated keys are legal (VT-17): they are today, and
an `Option` return would silently pick one."

The item notices the conflict and does not resolve it. Two ways out: decline the
accessor and record that the O(n) scan is the intended cost of an unenforced
convention; or ship a lookup returning **all** matches — a `value_range(key)` or
an iterator over the `"key:"` prefix — which preserves the visible ambiguity and
does not violate a MUST that names `get(key)` specifically. Repeated keys need no
decision: VT-17 settles them legal.

### 6. `recorded_at`'s type and its ordering rule (VT-9)

**Clause** (`:851-856`, `[PROVISIONAL]`): a **signed** 64-bit value, a newtype
over `i64` and not an alias (`:881`), and "**the contract MUST NOT state any
relationship between `RecordedAt` order and `SequencePosition` order.**"

**RUNBOOK** (`:2891-2894`): "a plain `u64` of milliseconds since the epoch …
**A rule that it is non-decreasing with position.**"

Three disagreements in four lines: unsigned against signed, alias against
newtype, and a rule the clause forbids the contract from stating. The clause's
coherence argument decides the newtype — `impl From<RecordedAt> for SystemTime`
is permitted only because `RecordedAt` is local — and its counter-evidence
against the ordering rule is two clocks 2.4 seconds apart and a tablet six
minutes fast after a factory reset (`:870-873`). `RUNBOOK.md:557`'s ledger
repeats the forbidden rule as VT-9's *falsifier*, so the ledger, the phase body
and the clause are three different answers.

### The ownership rot underneath all six

`RUNBOOK.md:447-451` and `:557-561` still assign VT-6, VT-9, VT-14 and VT-21–VT-24
to "phase 5", which is now the wire format (`:3069`). The specification names
phase 4 four times in terms — VT-14 at `:1085-1086`, VT-21 at `:1308-1310`,
VT-28 at `:1501`, CF-18 at `:6798` — and phase 4's own body claims them
(`:2741-2747`, `:3063`, "3 for the values that were phase 5"). Under rule 5 the
clauses win. Two cells in that block are correct and must not be swept up: `:558`
gives VT-10 to phase 13, and `:561` gives VT-30 to phase 4. `:563` is a third
kind of wrong — it gives ES-17 to "1 and 4", against ES-17's own phase-8
falsifier.

---

## What is still genuinely open

The provisional residue, by owning ADR. These are decisions, not transcriptions.

**ADR-0011.** ES-11 and ES-12 are `[PROVISIONAL]` on the **transport** axis
(`:2432`, `:2479`), whose far end is unbuilt and empty at *both* ends. Phase 4
can restate the falsifier and cannot lift it. The live defect is D7:
`store.rs:105-108` promises "nothing is executed until it is first polled" while
`MemoryEventStore` snapshots under the lock at call time (`memory.rs:155-181`),
and the third shape the decision must cover — a single response body that cannot
stream and is capped at 64 MiB — is documented at `docs/adapter-shapes.md:214-215`.
ES-12's rule is additionally blocked on a hostile fixture that can be paused
*between statements*, which nothing in the tree can do.

**ADR-0012.** ES-17's batch ownership, above. Everything else in its scope is
frozen; four of those frozen clauses have no rule yet — ES-19's
`batch_positions_follow_slice_order`, ES-21's, ES-22's
`dropped_append_future_leaves_no_partial_batch`, ES-24's
`reissued_conditional_batch_lands_once`. VT-30 (`:1562`) is `[PROVISIONAL]` with
two named instruments that are both unbuilt, while `RUNBOOK.md:561` gives it to
phase 4 with a falsifier phase 4 *can* meet ("E2E-04 and E2E-05 still unwritable
after phase 4"). The two falsifiers disagree and the ADR must pick one; VT-27
`[FROZEN]` depends on VT-30 landing to make its refusal sound.

**ADR-0013.** ES-10 lifts (see [the CF-25 section](#the-cf-25-exposure)). What
does not lift with it: arm C's `head()` is a **frontier**, not `max(position)`,
and ES-30's owed rule is named `head_is_the_highest_visible_position` — two
different predicates, unreconciled anywhere in the tree. Arm B-tag's
per-boundary-versus-global question is explicitly left open by the experiment
(`README.md:402-412`) and no draft position exists. ES-35 is `[PROVISIONAL]` on
durability with its falsifier owned by phase 8 (`RUNBOOK.md:572`), and ES-40 on
completeness with nothing planned before phase 14.

**ADR-0014.** VT-6 (`:751-760`) asks whether an adapter can detect a restore at
all, and nothing in the workspace can answer: `MemoryEventStore` has no
persistence, the five database skeletons are `todo!()`, and `DurableFixture`
reopens by instruction rather than by fault. VT-9 names a Durable Object's frozen
clock as its falsifier and there is no Durable Object before phase 9. VT-10 puts
foreign identity on `IngestStore` in `happenstance-sync`, whose testkit does not
exist. **This ADR has neither a fixture instrument nor an adapter one** — no
rule, mutant or racer anywhere in the workspace names `EventId`, `StoreId` or
`recorded_at`.

**ADR-0015.** VT-14 is `[PROVISIONAL]` and phase 3 wrote none of the seven-codepoint
bidi rejection it requires (zero hits for `202A`/`202E`/`2066`/`bidi` under
`crates/happenstance-core/src/`), and `spec-trace` structurally cannot notice
because VT-14 cites *unit tests*. VT-21–VT-24 are `[PROVISIONAL]`, each with a
falsifier phase 4 cannot reach, while `suite.rs:132-142` hardcodes 65,536 / 64 /
128 / 128 as suite-local private constants none of which exists in
`happenstance-core`. Const-constructibility and `ProjectionId`'s validation are
claimed by no clause at all.

---

## What the compiler said

Nine experiments. Four were re-run independently for this dossier (E1, E2, E4,
and the plain-trait half of E2); five are recorded on the scout's transcripts and
are marked as such. Every verdict below is a compile, not an argument.

### E1 — do proof cases 1 and 2 fail today? **Yes, and not with the cited error code.**

Re-run against unmodified `happenstance-core` on rustc 1.97.1. Returning a stream
from a generic function with the query bound to a **named local** gives:

```
error[E0597]: `query` does not live long enough
7 |     store.read(&query, ReadOptions::new())
  |     -----------^^^^^^---------------------
  |     argument requires that `query` is borrowed for `'1`
```

Storing one in a struct field gives the same E0597. `error[E0716]` is the
**inline-temporary** form only — `store.read(&Query::all(), …)` — and is what
`ES-13:2517` and `RUNBOOK.md:2771` both cite. An ADR naming only E0716 will be
corrected by the first reader who compiles the case the artefact actually
specifies.

**Which code you get depends on how the return type is spelled, and all three
arrangements were compiled a third time to settle a disagreement between this
dossier and ADR-0011's review, each of which had reported one code as though it
were the only one:**

| Arrangement | Diagnostic |
|---|---|
| returned, bare `-> impl Stream<Item = …>` | `error[E0597]` |
| returned, `-> impl Stream<…> + '_` or `+ 'a` | `error[E0515]` |
| returned inside a struct field, bare | `error[E0597]` |
| inline temporary, `store.read(&Query::all(), …)` | `error[E0716]` |

The distinction is not cosmetic and it is the Rust-specific point worth keeping:
**E0597 and E0515 are the same defect reported from two ends.** Without a
lifetime bound on the opaque type the compiler reasons from the *borrow* — the
`&query` argument must outlive the return, and `query` is dropped at the end of
the function. Add `+ 'a` and the opaque type is now required to live for `'a`,
so the compiler reasons from the *value* instead and reports that the returned
thing references a local. Either way the cause is that RPITIT's opaque type
captures the `&Query` lifetime. So the amendment ES-13 is owed must name the
family, not a code: a clause that pins one error code documents one spelling of
one call site.

### E2 — does precise capturing rescue ES-13? **On a plain trait yes; under `trait_variant` no.**

Re-verified on a fresh crate. A plain RPITIT trait with `fn read<'a>(&'a self,
query: &Query) -> impl Stream<…> + use<'a, Self>` compiles, a borrowing adapter
compiles with `use<'a>`, and the escape case compiles with `&Query` untouched.
Under `#[trait_variant::make]` (0.1.3, the newest release):

```
error[E0799]: `Self` can't be captured in `use<...>` precise captures list, since it is an alias
error: `impl Trait` must mention all type parameters in scope in `use<...>`
  = note: currently, all type parameters are required to be mentioned in the precise captures list
```

The cause is structural, not stylistic: `variant.rs`'s `blanket_impl_item` copies
the signature verbatim into `impl<TraitVariantBlanketType: SendEventStore>
EventStore for TraitVariantBlanketType`, where `Self` is an alias rather than a
generic argument. A hand-written blanket impl fails identically, so this is a
property of any blanket-impl derivation. Hand-rolling the expansion and naming
the blanket parameter explicitly works, and **both load-bearing `memory.rs` tests
pass verbatim against it** — `send_flavour_stream_is_send_in_generic_code`
(`:364-391`) and `spawns_from_generic` (`:393-453`) — so CLAUDE.md constraint 3
survives. Two further findings: `use<Self>` alone is the wrong spelling, because
it drops `&self` too and makes every borrowing adapter impossible (`E0700`); and
the shape makes possible a test that is impossible today,
`spawns_from_generic_with_inline_temporary`, holding the stream across an await
inside a real `tokio::spawn` with no hoisted `let query` — the exact line
`memory.rs:421-423` apologises for.

### E3 — what does by-value `read` cost? **It works under real `trait_variant`, and the price is at the call sites.** *(scout transcript)*

All four E1 shapes compile, both `memory.rs` tests pass, no hand-rolling. Measured
with a counting allocator: `size_of::<Query>()` is 16, one clone of a realistic
two-clause query is **9 allocations / 212 bytes**, and `Query::all().clone()` is
free — so any benchmark written against `Query::all()` measures nothing. The
decisive cost is not the clone but `error[E0308]` at every caller holding only a
`&Query`, which includes `read_decision_model` (`store.rs:205-215`),
`AppendCondition::new`, and every rule reading the same query twice. By-value does
not remove the clone; it moves it from zero-or-one sites to one per read, with no
caller opt-out.

### E4 — does proof case 4 compile today? **Yes — and it runs.**

Independently re-run against unmodified `happenstance-core`:

```rust
pub async fn case_four<S: EventStore>(store: &S) -> Result<(SequencePosition, usize), AppendError<S::Error>> {
    let events: Vec<Event> = vec![ /* two events */ ];
    let position = store.append(&events, None).await?;
    let mut total = 0usize;
    for event in events {
        let (ty, data, tags, metadata) = event.into_parts();
        total += /* … */;
    }
    Ok((position, total))
}
```

`test tests::runs ... ok`. The caller never gave the `Vec` away; `&events`
reborrows for the duration of the awaited temporary and the borrow ends at the
`?`. **Proof case 4 is not a falsifier of ES-17.** The unreachability ES-17
(`:2703-2708`) and VT-3 (`:670-677`) actually record is on the *adapter* side —
"unreachable from any trait impl" — which the artefact's own framing, "a
**generic** consumer, bound on `EventStore` and never on a concrete store"
(`RUNBOOK.md:2990-2992`), structurally excludes.

### E5 — can `dynosaur` erase the port? **Not today, and the blocker is `read`, not `append`.** *(scout transcript)*

```
error[E0277]: `dyn futures_core::Stream<Item = …>` cannot be unpinned
    = note: required for `Box<dyn Stream<…>>` to implement `Stream`
```

ADR-0001:110-112 and `E2E-CASES.md:1432` both assert `dynosaur` generates the
wrapper if erasure is needed. That assertion is false as the port stands, and
nothing in the workspace would have caught it. Adding `+ Unpin` to `read`'s
return fixes it and the erased store round-trips at runtime — but that is a
contract change and therefore ADR-0011's business, and it forbids a
self-referential generator-backed stream, which is a real axis to name. With the
fix, `&[Event]`, `Vec<Event>` and an owned `EventBatch` all erase;
`impl IntoIterator<Item = Event>` does not (`error[E0191]`), isolated against a
minimal trait with no `read` and a concrete error type.

### E6 — `Cow<'static, str>` with a const `from_static`? **Works, with one divergence nobody asked about.** *(scout transcript)*

`from_static("")` is `error[E0080]` at a **free const** site under `cargo check`,
`clippy` and `build`; at an **associated const** it passes check and clippy and
fails only at build — identical to `Capability::declined`'s documented hazard
(`contract.rs:325-330`). A plain runtime call compiles and panics. `Borrowed("A")`
and `Owned("A")` compare equal, hash equal and are one `HashMap` key; size goes
16 → 24 bytes. **The divergence:** `chars()` is not const, so const validation
must walk `as_bytes()`, which cannot see U+0080–U+009F — exactly the range
`char::is_control` (Unicode `Cc`) rejects today. Const and runtime constructors
would accept different sets unless ADR-0015 narrows the rule to ASCII controls,
which would also make the four "ASCII" docstrings true.

### E7 — does a third field break `SequencedEvent::new`? **Yes, `error[E0061]`, and `#[non_exhaustive]` is why.** *(scout transcript)*

`#[non_exhaustive]` gives `error[E0639]` on the struct literal, which makes `new`
the entire compatibility surface — the opposite of the protection it is usually
credited with. The shape that survives is `const fn new(<intrinsically required
only>)` plus `#[must_use] const fn with_*(mut self) -> Self`, which stays usable
in a `const` initialiser (a separate builder type would not) and matches
`ReadOptions` (`query.rs:246-266`) and `AppendCondition` (`append.rs:72-88`), so
the crate gains no third idiom. A fourth field was then added with **zero**
downstream edits including at a `const` call site. **The caveat that must travel
with it:** this survives only for fields with a meaningful default. A genuinely
required store-assigned field has no non-breaking constructor shape at all, and
phase 4's two additions are exactly that kind.

### E8 — does the looser bound fix D4? **Yes, and the impl it needs already exists one file over.** *(scout transcript)*

Today's `impl TryInto<EventType, Error = InvalidEventType>` handed an already-built
`EventType` gives `error[E0271]: expected InvalidEventType, found Infallible` at
`event.rs:198`. Replacing the equality constraint with `T: TryInto<EventType>,
InvalidEventType: From<T::Error>` plus `impl From<Infallible> for
InvalidEventType { fn from(never: Infallible) -> Self { match never {} } }`
accepts both a `&str` and a const `EventType`, and is a strict widening. This
composes with E6: the point of a const `from_static` is passing it to
`Event::new`, which is E0271 today, so the two must land together or the const
constructor is unusable at the one call site that wants it.

### E9 — does a blanket `EventStoreExt` collide with `trait_variant`'s? **No, and it cannot be overridden.** *(scout transcript)*

No `E0119`: the ext blanket impl implements a *local* trait while
`trait_variant`'s implements `EventStore` — different traits, nothing to overlap.
Two blanket impls stack and resolve in one call, proven at runtime against
`MemoryEventStore`. Override is refused twice: `error[E0119]` in the crate owning
the trait, `error[E0117]` (orphan) anywhere else. That is a feature for a
conformance-tested port and a hard ceiling for an adapter with a real
`SELECT max(position)` fast path, which must add an inherent method that then
**silently shadows** the ext method wherever the concrete type is in scope.

---

## The CF-25 exposure

CF-25 forbids declaring a port frozen while an axis of §6.5's portfolio has no
passing implementation at its far end, unless the freeze names the axis and an
ADR accepts the risk. The portfolio has **seven** axes
(`SPECIFICATION.md:7183-7189`, mirrored at `RUNBOOK.md:631-639`) and **the
adapter column is empty on every one of them**.

| Axis | Instrument today | Residual clause | Needs explicit acceptance |
|---|---|---|---|
| Position allocation | **Fixture** — `PreCommitPositionStore` fails `nothing_below_an_observed_position_appears_later` deterministically on one thread | ES-10, `[PROVISIONAL]` **until this ADR** | **Yes, once ES-10 lifts** |
| Transport | **Neither end.** `happenstance-neon` is a skeleton | ES-11, ES-12 | No — carried by their own markers |
| Async flavour | **Fixture** — `LocalMemoryEventStore` passes natively and on wasm32 (CF-28) | **none** | **Yes** |
| Batch shape (`ProjectionStore`) | Skeletons at both ends, a passing implementation at neither, and no projection suite | **none** | **Yes** — but see the scope caveat below |
| Handle multiplicity | **Fixture** — `Fixture::connect` + `SECOND_HANDLE` (CF-16), rule obligation CF-19; `CachedHeadFixture` fails it. All three fixtures hand out refcount clones of one in-process object, so no *connection* has ever been opened twice | **none** | **Yes** |
| Durability | **Fixture** — `REOPEN` (CF-17), `LosingFixture` fails the rule. Nothing yet loses a write to a *fault* rather than an instruction | ES-35 | No |
| Completeness | **Neither end**, nothing planned before phase 14 | ES-40 | No |

**The scope caveat.** Batch shape is a `ProjectionStore` axis and phase 4 freezes
`EventStore`. CF-25 is worded "before any port in this specification is declared
`[FROZEN]`", so a defensible reading assigns that acceptance to phase 6. ADR-0013
must state which reading it takes rather than leaving the axis claimed by neither
phase.

### The ES-10 verdict: lift it

The condition ES-10's marker states — *"one affordable answer lifts this clause
to `[FROZEN]` at phase 4"* — **is met**, and the exit criterion's if-and-only-if
at `RUNBOOK.md:3029-3038` therefore resolves in favour of lifting. **ES-25 and
ES-26 do not reopen.**

The evidence is `docs/experiments/position-visibility/`, run against a real
PostgreSQL with `fsync=on`. Arm C — `xid8` + `pg_snapshot_xmin` — is the only arm
that both passes the inversion detector on both writer pairs *and* leaves writers
unserialised. Throughput ratio to a bracketing baseline: **0.987 / 0.993 / 1.015
/ 1.026** at 1 / 8 / 32 / 64 clients, where the 1-client figure comes from
`results/ratios-c1long.csv` (the separate 90-second pass) and the other three from
`results/ratios.csv`. Both positive controls fired: the baseline reproduces the
inversion, and arm A at 64 clients collapses to 0.062× with p99 60× worse — so
the instrument is known to be capable of returning a negative.

Three caveats must travel with the lift, none of which the clause's current text
carries.

1. **Lifting ES-10 moves the position-allocation axis *into* ADR-0013's
   acceptance list, not out of it.** The `[PROVISIONAL]` marker was what carried
   that axis's CF-25 exposure in its own text; freezing removes it. The
   experiment says so itself (`README.md:364-366`): it measured four SQL
   strategies, not four implementations of `SendEventStore`. This is the opposite
   of what a casual reading of exit criterion 6 suggests.
2. **Arm C and ES-30 are unreconciled.** Under arm C, `head()` reports a frontier
   rather than `max(position)`, read-your-own-writes does not hold, and staleness
   is bounded by the longest open write transaction *anywhere in the cluster* —
   0.688 ms with no holder, 4010.719 ms behind a five-second write in an
   unrelated database (`results/staleness_pinned.txt`). ES-30 is `[FROZEN]` and
   its owed rule is `head_is_the_highest_visible_position`. "Highest visible
   position" and "frontier" are not the same predicate, and ES-11 names `head()`
   as the primitive that makes correct pagination possible, so this is
   load-bearing in two directions.
3. **Arm B-tag's question is open and cheap to close silently.** It is nearly
   free (0.935 at 64 clients) and buys a **per-boundary** invariant where ES-10
   states a **global** one; on disjoint tags it reproduces the baseline inversion
   byte-for-byte. DCB evaluates `AppendCondition` against a boundary, so ES-10
   may be strictly stronger than its consumers require. The experiment
   deliberately refused to settle it (`README.md:402-412`) and no draft position
   exists in the tree.

### The axis count is wrong in both documents

`RUNBOOK.md:3032` says "ADR-0013 accepts the remaining **six** axes"; and
`SPECIFICATION.md:260` says "The remaining six axes are accepted in the ADR that
lands this document." Because both documents agree, rule 5 does not arbitrate and
both need correcting.

The "six" has a traceable provenance: §1.3's first bullet takes position
allocation out as *measured rather than accepted*, and 7 − 1 = 6. But its second
bullet then names five `ES` clauses spanning **four** axes (position allocation,
transport, durability, completeness), and the exit criterion states that partition
as exclusive — "they are not part of this freeze and need no acceptance" — under
which the remainder is **three**: async flavour, batch shape, handle
multiplicity. Once ES-10 lifts, position allocation rejoins and it is **four**.

So there are two internally consistent numbers produced by two different
partitions in two documents. ADR-0013 owes a partition, not just a number, and
the spec amendment is owed at `:260`, `:362` and `:438`, which restate the same
split, plus §1.3's census at `:218-221` (193 clause IDs; 135 `[FROZEN]`, 46
`[PROVISIONAL]`) which moves by one when ES-10 lifts.

---

## What is proven by nothing

Stated plainly, because a freeze that does not name its exposure is a freeze
pretending to evidence it does not have.

- **That precise capturing works on the real `EventStore`.** The reproduction was
  a standalone crate; `crates/` was off-limits this run. Nobody has tried it on
  `store.rs`, nobody has checked whether the six skeletons' concrete stream types
  capture the query lifetime, and nobody has checked a newer `trait-variant`
  (0.1.3 is the newest release). What *is* compiled is that the derivation as it
  stands cannot express it.
- **That phase 4's proof artefact proves anything for two of its four cases.**
  `RUNBOOK.md:3013-3016` asserts "each fails against the current signatures. That
  is the whole point." Case 4 compiles and runs today (verified). Case 3 is
  already green in the gate as `spawns_from_generic` (`memory.rs:393-453`), which
  CLAUDE.md constraint 3 forbids deleting.
- **That every semantic sentence phase 4 writes has a rule that can fail.** Of
  the eight semantic items in the phase body, **three** have an existing rule with
  a registered failing store: position visibility (`PreCommitPositionStore`,
  `AwaitAcrossBorrowStore`), two handles (`CachedHeadFixture`), reopen
  (`LosingFixture`). The other five cite rules marked **(new)** that do not exist,
  and `conflicting_position` names no rule at all. Three `[FROZEN]`/`[PROVISIONAL]`
  clauses in scope declare `Rule:` none as their deliberate content — ES-23
  (`:2949-2953`), ES-32 (`:3383`), ES-37 (`:3625-3626`) — and VT-27 is
  compile-level. Exit criterion 2 as written requires amending them to be ticked.
- **That any CF-25 far end is filled.** Zero adapter instruments across all seven
  axes. "Fixture, not adapter" is the honest answer for four; "neither" for three
  — and transport is the axis ES-11, ES-12 and ES-13 all sit on, one of which is
  already `[FROZEN]`.
- **That a 128-bit `StoreId` crosses wasm32.** `docs/adapter-shapes.md:220`
  records that Workers SQL widens integers through a JS number, bounding positions
  at 2⁵³, and that `NonZeroU64` already permits positions that adapter cannot
  round-trip. A 128-bit `StoreId` cannot survive a JS number at all, and no
  capability row mentions it.
- **That ES-17 can be decided at phase 4.** Its falsifier is a benchmark on an
  adapter that is `todo!()` bodies, in a workspace with no benchmark harness.
- **That `Fixture` can bound a rule's strength.** `Fixture` declares three
  capability constants and none of them is a poll budget, so
  `nothing_below_an_observed_position_appears_later` — the single rule ADR-0013's
  entire ES-10 lift is checked by — varies silently in strength with the adapter.
  `suite.rs:2799-2800` states the consequence itself: "an adapter needing three
  polls is not let off." CF-33 forbids the clock that would otherwise bound it.
- **That the specification's own citations are checked outside it.**
  `check_citations` (`xtask/src/spec_trace.rs:276`) reads only `SPEC`, and it
  verifies file existence and a line range, not content. Any ADR citing scenario
  line numbers inherits that exposure.

---

## Corrections this pass made to its own inputs

Recorded rather than absorbed, per `RUNBOOK.md:73-75`.

1. **Arm C's one-client ratio has a different source file than the sweep
   claimed.** The sweep cited `results/ratios.csv` for the series "0.987 / 0.993 /
   1.015 / 1.026". `ratios.csv` gives **0.628** at one client against a bracketing
   baseline whose drift is 3.70×. The 0.987 is in `results/ratios-c1long.csv`, the
   separate 90-second single-client pass, and `README.md:249-256` explains the
   substitution and flags arm A's one-client number as the one to distrust. An ADR
   quoting the four-number series must cite two files or a reader chasing it in
   `ratios.csv` will conclude arm C is 37% slower at one writer.
2. **ES-32 is `[PROVISIONAL]`, not `[FROZEN]`.** The conflict audit's
   exit-criterion-2 row named it among "three `[FROZEN]` clauses … declare `Rule:`
   none". `SPECIFICATION.md:3357` carries a provisional marker falsified by the
   projection-runner benchmark. Its "`Rule:` none for the absence" (`:3383`) is
   real, so the finding survives — but on ES-23 and ES-37 alone, both `[FROZEN]`.
3. **The "six axes" number is not simply arithmetic nonsense.** The audit and the
   sweep both concluded that no partition of seven yields six. Seven minus the one
   *measured* axis does, and that is exactly what §1.3's first bullet performs.
   The defect is that §1.3's bullets 2 and 3 overlap while the exit criterion
   states the same split as exclusive, producing six and three from two documents.
   ADR-0013 owes a partition, not a corrected integer.
4. **VT-4's "three facts" and "four fields" are both verbatim and not in
   conflict.** The heading (`:683`) says three store-assigned facts; the body
   (`:685-687`) says four fields, the fourth being the `Event` itself. Exit
   criterion 9's "adding a *third* field" most plausibly means a third
   store-assigned addition — the fifth struct field. Either reading demands a
   non-widening constructor, so the substance is unchanged, but reporting the
   criterion as arithmetically wrong overstates it.
5. **Three counts in this run's briefing map are stale.** The testkit carries
   **55** event-store rules (counted from `registry.rs:105-177`), not 51 —
   `SPECIFICATION.md:362` already says 55 — plus a separate five-rule concurrency
   registry (`concurrency.rs:1050-1056`), for 60 portable rules. The mutant
   registry has **50** `Kind::Mutant` rows and **2** `Kind::ConformantVariant`,
   not 43 + 2, plus 6 racers in the separate `RACERS` table
   (`mutation_coverage.rs:1629`).
6. **The `sync/lib.rs:NN` shorthand appears seven times, not six.**
   `RUNBOOK.md:2971` says six. The tree has `E2E-CASES.md:895`, `:1009`, `:1197`,
   `:1249` and `scenarios/README.md:695`, `:1554`, `:1804`. The eighth grep hit is
   the RUNBOOK's own description of the shorthand.
7. **`saturating_add` is at `event.rs:143`.** The briefing map said `:141`, the
   sweep said `:142`. `VT-13:1016-1020` cites the method as `event.rs:138-144` and
   is right. `SequencedEvent::new` is `event.rs:286`; the stale MSRV comment is
   `append.rs:101-102`.
8. **The proof artefact's own error code is wrong in both documents, and this
   dossier got the correction half wrong on its first pass.** `ES-13:2517` and
   `RUNBOOK.md:2771` cite only `E0716`, which is the inline-temporary form and
   not the two cases the artefact enumerates. This dossier then reported `E0597`
   for both of those; ADR-0011's review reported `E0515` for both. A third
   compile of all four arrangements settled it — the code depends on whether the
   return type carries a lifetime bound, and the table in *What the compiler
   said* now records all four. Both earlier reports were right about one
   spelling and wrong to generalise. **The amendment owed at ES-13 is therefore
   to name the defect rather than a code**, which is the more durable clause in
   any case.
9. **Proof case 4 was executed, not merely type-checked.** Re-run this pass
   against unmodified `happenstance-core` with `MemoryEventStore`: `test
   tests::runs ... ok`. The sweep's prediction is confirmed by running.
10. **The ADR queue under-covers phase 4 by twenty-nine clause IDs — a finding
    neither input reported.** `RUNBOOK.md:2755` says phase 4 "Discharges ES-8 –
    ES-40, VT-1 – VT-31", which is 64 IDs. The queue at `RUNBOOK.md:284-288`
    scopes the five ADRs to 35: 0011 → ES-11–ES-13, 0012 → ES-17–ES-24, 0013 →
    VT-11–VT-13 + ES-10 + ES-38, 0014 → VT-4–VT-10, 0015 → VT-14–VT-25. The 29
    unclaimed include **every clause governing `head()`** (ES-30–ES-32), **all of
    `ReadOptions` and `Query`** (VT-26–VT-31, ES-8, ES-9, ES-14–ES-16), **all of
    the condition semantics** (ES-25–ES-29), and the multi-writer, durability and
    completeness sets (ES-33–ES-37, ES-39, ES-40). Every audit row classified
    "unclaused" sits in this gap. Each brief below names the extension its ADR
    must claim.
11. **The conflict audit filed rows by topic, not by the queue's stated scopes.**
    `EventStoreExt`, the prelude, `ReadOptions::to`, D2 and D5 were all assigned to
    ADR-0011, whose queue question is laziness and isolation. Recorded so the
    drafters extend a scope deliberately rather than inherit one by accident.
12. **CF-16 and CF-19 do different work on the handle-multiplicity axis.** The
    sweep cited CF-16 as carrying `two_handles_observe_each_others_appends`; §6.5's
    own portfolio row cites CF-19 (`:7187`) for the rule obligation and CF-16
    (`:6723`) for the fixture seam. Both name the rule; only CF-16 is the
    capability MUST.
13. **Five of six CF-16–CF-21 are already frozen.** `RUNBOOK.md:2866`'s "No phase
    owns freezing CF-16 – CF-21" is stale: CF-16 (`:6723`), CF-18 (`:6774`),
    CF-19 (`:6808`), CF-20 (`:6820`) and CF-21 (`:6833`) are all `[FROZEN]`, and
    CF-17 is `[PROVISIONAL]` and owned by phase 8 (`RUNBOOK.md:572`). The
    poll-count half of that item is live and unclaused and is verbatim true.
14. **Experiments E3, E5, E6, E7, E8 and E9 were not re-run this pass.** Their
    verdicts follow from the diagnostics quoted and are recorded as scout
    transcripts. E1, E2 and E4 were re-compiled independently and agree.

---

## What each ADR must answer

### ADR-0011 — `read`

**Queue question** (`RUNBOOK.md:284`): what does `read` promise about laziness and
isolation — when is the store's state sampled, and do the items of one `Query`
share one sample? (ES-11 – ES-13.)

Must answer, in this order: whether ES-13 stands (it is `[FROZEN]`, so departing
from it needs this ADR to supersede it explicitly, not to edit it); if it stands,
which of precise capturing or a rewritten proof artefact discharges cases 1 and 2,
and if precise capturing, that removing `#[trait_variant::make]` from `store.rs`
is a knowing contradiction of ADR-0001's mechanism costed against E2's compiled
result; the laziness answer against **three** store shapes, not two, the third
being the buffered 64 MiB single-response body at `adapter-shapes.md:214-215`;
whether `+ Unpin` joins `read`'s return type, since without it `dynosaur` cannot
erase the port and ADR-0001:110-112 asserts it can; and what happens to ES-11 and
ES-12, which cannot be lifted at phase 4 because their axis is empty at both ends.

It must also claim, or explicitly decline, the twenty-nine unclaimed IDs nearest
it: ES-8, ES-9, ES-14, ES-15, ES-16 and VT-26–VT-31 — which is where
`ReadOptions::to`, `limit(0)`, `Query::Items` and `after` actually live. The
`EventStoreExt` and prelude items have **no clause at all** and owe a new ES
clause naming a wrong implementation, or an explicit refusal.

Constraints it may not break: ES-13 `[FROZEN]`; ES-16 and VT-29 `[FROZEN]` on an
inclusive `to`; CLAUDE.md constraint 3 (both `memory.rs` tests); no
`#[async_trait]`; bind `EventStore`, not `SendEventStore`, in generic code.

### ADR-0012 — `append`

**Queue question** (`:285`): what shape does `append` take and what are its
preconditions — who owns the batch, what an empty batch is, whether a batch can
violate its own condition, and what a dropped future may have done? (ES-17 –
ES-24.)

Must answer: whether ES-17 lifts to `[FROZEN]` on `&[Event]` using the three
grounds it already states, or stays `[PROVISIONAL]` until phase 8 builds its named
measurement — and either way must record that the owned-batch alternative is
forbidden by VT-24 in terms, that `impl IntoIterator` is compiled to be
unerasable by `dynosaur` (E0191), and that no rule in the suite fails if the
ownership changes. It must state the self-conflict sentence (ES-21 has already
decided it) and the cancellation sentence (ES-23 has already decided it, and
carries `Rule:` none by design, which is what makes exit criterion 2
unsatisfiable). It must decide `conflicting_position`'s `Display` — including the
shape of the empty case, since adapters are permitted to leave it `None` — and
carry the `happenstance-neon` citation exit criterion 5 demands, which ES-25 does
not have today; `adapter-shapes.md:116-125` records that Neon's CTE **keeps** the
field, so Neon is the reason the question was asked and not the reason to demote
it.

Owed extensions: ES-25–ES-29 and VT-30 are in phase 4's declared scope and no
queued ADR claims them. VT-30 is `[PROVISIONAL]` with unbuilt instruments while
`RUNBOOK.md:561` gives it a phase-4-reachable falsifier; this ADR must pick one
falsifier, because VT-27's refusal is only sound if VT-30 lands. It must also
decide whether `Fixture::MID_BATCH_FAULT` earns a CF clause — CF-18:6793-6804
assigns the decision here in terms, and either exit moves or explicitly does not
move §1.3's hand-computed census. The stale 1.85 MSRV comment at
`append.rs:101-102` is prose-only and belongs here because ES-25 points readers
at that function.

### ADR-0013 — positions and the CF-25 acceptance

**Queue question** (`:286`): what does a store promise about position assignment
and visibility — gaps, reuse, and the invariant that makes `AppendCondition::after`
sound? (VT-11 – VT-13, ES-10, ES-38.)

Must answer: lift ES-10 to `[FROZEN]` on arm C's evidence — the condition its
marker names is met — while carrying all three caveats: that the lift moves
position allocation **into** the acceptance list rather than out of it; that
`head()` under arm C is a frontier and ES-30's owed rule says "highest visible
position", which is an unreconciled predicate mismatch load-bearing in two
directions; and that arm B-tag's per-boundary-versus-global question is left open
by the experiment and must not be closed silently by this lift. It must then name
the CF-25 axes it accepts, stating the **partition** it uses rather than
inheriting the number six from two documents that both carry it, and taking a
position on whether batch shape is phase 4's or phase 6's under CF-25's "any port"
wording. It must record the spec amendments owed at `:260`, `:362`, `:438` and
§1.3's census. VT-13's overflow fix belongs here. It must not restate the
visibility invariant in VT-12, which is `[NON-NORMATIVE]` precisely to stop that.

The poll-count limitation is this ADR's to settle and is unclaused: `Fixture`
cannot express a poll budget, so the single rule the ES-10 lift is checked by
varies in strength with the adapter, and CF-33 forbids the clock that would bound
it.

### ADR-0014 — identity and time

**Queue question** (`:287`): what does an event carry beyond type, data and tags —
identity, store incarnation, recorded time — and who assigns each? (VT-4 – VT-10.)

Must answer: `RecordedAt` as VT-9 requires — a newtype over **`i64`**, not a
`u64` and not an alias, with the `std`-gated `SystemTime` conversion the coherence
argument needs — and must **drop** the non-decreasing-with-position rule, which
VT-9's third MUST forbids the contract from stating; the RUNBOOK ledger cell that
names that forbidden rule as VT-9's falsifier is owed a correction. It must
restate VT-9's live falsifier (a target with no wall clock at append time; the
Workers skeleton is the instrument) and VT-6's (an adapter that can neither detect
a restore nor be given an out-of-band re-mint), since neither can be lifted here.
It must settle the constructor shape, and E7 supplies the compiled answer and its
caveat: a `new` + `with_*` shape survives a later field addition with zero
downstream edits, **but only for defaultable fields**, and this phase's two
additions are required store-assigned ones. Exit criterion 9 is satisfiable only
if `SequencedEvent::new` (`event.rs:286`) is superseded rather than widened.

It must record two things nobody has written down: that `EventId`'s 128-bit
`StoreId` has never been checked against the wasm32 shape, where
`adapter-shapes.md:220` already bounds positions at 2⁵³ because Workers SQL widens
through a JS number; and that VT-7 `[FROZEN]` grows the port a
`contains_event_id` membership operation the signatures section never lists, so
"the signature half is not reopened" is true of `append` and not of the trait.
This is the ADR with **neither a fixture nor an adapter instrument** anywhere, and
it must say so.

### ADR-0015 — validated identifiers and limits

**Queue question** (`:288`): how is a validated identifier constructed, and is
`Tag` equality byte equality? (VT-14 – VT-25.)

Must answer: D4's fix (VT-18's loose bound plus `impl From<Infallible> for
InvalidEventType`) and the error composition (`InvalidQuery: From<InvalidTag>`
plus an `InvalidQuery::Tag` variant), stating that "umbrella" meant composition
and that the `InvalidInput` collapse is rejected by name — and saying explicitly
whether `AppendError<E>`'s seam is closed or left to the caller, since VT-18 does
not mention it. It must decide const-constructibility, which **no clause
requires**, and if it ships `from_static` it owes a new clause and must resolve
E6's divergence: const validation walks bytes and cannot see U+0080–U+009F, so
either the rule narrows to ASCII controls (making four docstrings true) or
`from_static` is strictly weaker than `new` and must say so. Exit criterion 7 is
only checkable at a `const` call site, and satisfying it pulls phase 6's
`trybuild` dependency decision forward (`adapter-shapes.md:97-102`).

It must write VT-14's seven-codepoint bidi `matches!` and the four-site ASCII/`Cc`
prose fix together — VT-14 assigns both here in terms (`:1085-1086`) — and record
that both "positions" the RUNBOOK asks for are already taken: VT-14 forbids
blanket `Cf` rejection, VT-15 `[FROZEN]` normalises nothing. It must move the four
minima out of `suite.rs:132-142` into `happenstance-core` as public constants and
have the rules cite them, keeping VT-21–VT-24 `[PROVISIONAL]` until phases 8 and
10 test them, and add `AppendError::ExceedsStoreLimit`, which VT-25 `[FROZEN]`
requires and `error.rs:150-166` lacks. It must decline `Tags::value_of` or
replace it with a form that returns all matches, since VT-17 `[FROZEN]` forbids a
single-value accessor by name. `ProjectionId`'s asymmetry (`projection.rs:46` is
infallible while both siblings validate) has no clause; the docstring position can
land here and the validating half belongs with the projection port at phase 6.
VT-17's missing E2E case lands in `docs/scenarios/E2E-CASES.md`, not in
`SPECIFICATION.md`.
