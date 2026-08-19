---
item: "HS-S0185"
stage: implement
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Conditions record — the crate-root half, measured against the merged tree

EC-008 is the sanctioned move when implementation contradicts a **binding** statement in
`_design.md`: record the contradiction, route it, never fold it in, and never edit a file a
human signed off (`spec.md` EC-008; `project.md:300-302`). This file is that record for one
contradiction with four acceptance consequences, plus three observations the same measurement
turned up.

`_design.md` is untouched. So is `_design.md`'s sign-off row. Nothing below re-decides DT-1,
DT-4, DT-5 or DT-6.

**Added on the 2026-08-19 fix pass:** § **BC-004**, at the bottom — a clause of AC-004 that no
implementation can satisfy, found by a slice review, measured, and routed the way BC-002 was.

## BC-002 — the crate root's one fence is already spoken for

**What the design says.** `## Composition` composes `crate-root-encounter` as: summary,
answered-need line, `## Status: a facade over happenstance_core`, then
**`## Watch a boundary refuse`** — "replaces `## Using it today`. Two sentences, then the
fence, then one sentence pointing at step one. The fence is the first code on the page and the
refusal is inside it" (`_design.md:415-440`). `tension-resolutions/_resolutions.md § Anchor
table` supplies its final title, **`A boundary refuses`** (18 chars, `#a-boundary-refuses`).
`merge-forward-preflight/_baseline.md § Composition baseline` routes the two exceeded density
rows here with the sentence *"the rewrite replaces the program"*.

**What the merged tree says.** The premise the design was written on is deleted by the merge.

| what the design assumed | what `crates/happenstance/src/lib.rs` now is | how it was re-derived |
| --- | --- | --- |
| `## Using it today` carrying a fence that constructs a store and asserts the log is empty, inside a `# async fn example()` wrapper that is never awaited | no such heading; the first code is a 37-line `#[tokio::main]` program that defines a `DomainEvent`, a `DecisionModel`, and calls `commit`, and it **executes** | `git grep -n "Using it today" -- crates/happenstance/src/lib.rs` → nothing; `crates/happenstance/src/lib.rs:26-64` |
| `## Status: a facade over happenstance_core` exists and is retitled | the heading does not exist; ADR-0006's reasoning moved into `# What arrives here, and what stays below` | `_baseline.md § Dispositions` row 5, re-confirmed at `crates/happenstance/src/lib.rs:76-81` |
| the planned-surface roadmap sits **above** the only code on the page | no roadmap survives anywhere on the page, and the fence is already first | `crates/happenstance/tests/doc_surface.rs` (`no roadmap survives`), and the fence precedes `# The vocabulary` |
| the page may take a second fence | it may not: `crates/happenstance/tests/doc_budget.rs:155` asserts **exactly one** fence, on the stated ground that "a second one would demote the first, which is the page's primary hierarchy signal" | `cargo test -p happenstance --test doc_budget` |
| the page may take ~35 more doc lines | it may not: `MODULE_DOC_LINES = 130` (`doc_budget.rs:16`, and again in `doc_surface.rs`). This story's own additions take it from 125 to **exactly 130** | `git grep -c "^//!" -- crates/happenstance/src/lib.rs` |

**The contradiction, stated once.** Authoring `## A boundary refuses` on the crate root requires
either a *second* fence — which a signed-off assertion in another project's test suite forbids
by name — or **replacing** the `commit` program, which is that project's landing copy, is the
one demonstration of the typed layer the crate exists for (ADR-0006), and is referred to by
name from two vocabulary bullets ("Its item page carries the program above over two variants")
and from the file's own opening comment ("The first program below calls `commit`…"). Either way
the 130-line budget is breached. `project.md`'s risk table is explicit that the seam with
HS-P0016 is "purpose, not paragraph. This project does not touch landing copy".

**Why it is recorded rather than resolved.** Replacing a signed-off landing program in a
documentation story is re-litigating another gate's decision in a commit message, which is the
shape EC-008 exists to refuse. It would also make the page *worse* on the axis the design cares
about: the crate whose whole identity is the typed layer would meet a reader with an untyped
`Event`/`Bytes` program, and the typed demonstration would leave the landing page entirely.

**What landed instead**, so the crate-root half is not empty:

- the answered-need line, in HS-P0021's notation, immediately under the summary and above
  everything (`crates/happenstance/src/lib.rs:21`) — `_baseline.md § Composition baseline`
  records it as *absent, as expected. This project authors it*;
- a one-sentence pointer from directly beneath the fence to the opening encounter
  (`crates/happenstance/src/lib.rs:66`), which is what makes `docs/first-encounter.md`
  reachable from the page a `cargo add happenstance` reader lands on;
- anti-pattern 2 discharged to **zero under both invocations** — see BC-003.

**Routed to:** the `_design.md` sign-off owner, as an amendment to `## Composition`'s
`crate-root-encounter` region list. `spec.md` EC-006 names that route in its own words: *"If the
merged API genuinely cannot fit, record it as a design gap and route it to the sign-off owner."*

**Acceptance consequence.** `_ledger.md` rows **AC-002**, **AC-007** and **AC-008** carry a
crate-root clause each, and each stays `satisfied: false` until this is answered. Their page
halves are met and are cited on the rows. Nothing is flipped on a partial.

- **AC-002** — the refusal fence, its printed output beneath it, and the `## A boundary refuses`
  heading. Met on this surface today: the fence executes (it is `#[tokio::main]`, not the
  never-awaited wrapper), the roadmap is below it because there is no roadmap, the
  adapter-author redirect is last, ADR-0006's reasoning survives, and the bracket count is zero.
- **AC-007** — three inherited numbers on the crate root: the fence is **35** rendered lines
  against a ceiling of 32, **70** columns against 68 (two lines), and two of four `##` headings
  are over 22 characters (39 and 26). All three are properties of the program and headings this
  story may not touch; all three were measured by `merge-forward-preflight` before this story
  started and routed here on the assumption the rewrite would replace the program.
- **AC-008** — the crate root has no section whose last sentence can carry the ES-25 citation,
  because the section does not exist. Step 3 carries it (`docs/first-encounter.md:130`, `:129`
  before the BC-004 fix pass added a line inside step 3's fence).

## BC-003 — the literal bracket pairs were invocation-dependent, and are now zero

**What `_baseline.md` handed over.** Anti-pattern 2's count is a property of the *invocation*,
not of the page: 0 under `cargo doc -p happenstance --no-deps`, **2** under the gate's
`documentation` step, which exited 0 with both unresolved (`_baseline.md § Composition
baseline`, row 1; disposition row 9 routes the page half here and the gate half to `support`).

**What was done.** Both `[`happenstance_core`]` shortcut references are now explicit inline
intra-doc links, `[`happenstance_core`](happenstance_core)`
(`crates/happenstance/src/lib.rs:70`, `:144`). A shortcut reference that fails to resolve is
left as literal text and warns about nothing, which is why `-D warnings` exited 0 over it; an
explicit one resolves under both invocations. Re-measured in this session, both builds run one
after the other:

```console
$ cargo doc -p happenstance --no-deps
$ # count of "[<code>" in target/doc/happenstance/index.html -> 0
$ RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --all-features \
      --no-deps --document-private-items
$ echo $?                                        # -> 0
$ # count of "[<code>" in target/doc/happenstance/index.html -> 0
```

The invocation this story's zero is measured under is **both**, which is the statement
disposition row 9 asked for. The gate-level half — that the `documentation` step can render an
unresolved shortcut reference and exit 0 — is unchanged and stays routed to the `support`
initiative (`.redkiln/config.yaml:5`); this change removes the two instances, not the hole.

The rewrite is a link spelling, not a claim: `.kb/governance/rewrite-the-referent-never-the-reasoning.md`'s
test is whether the edit changes what the document *asserts*, and both sentences assert exactly
what they asserted before.

## Observations, recorded rather than absorbed

| # | the observation | disposition |
| --- | --- | --- |
| O1 | The crate-root fence's two hidden lines are `crates/happenstance/src/lib.rs:31` (`# #[tokio::main] async fn main() -> …`) and `:63` (`# Ok::<(), Box<dyn Error>>(()) }`). `_baseline.md` recorded that the `#[tokio::main]` wrapper is on neither `_design.md:524`'s permitted nor its forbidden list, so this story decides it deliberately: **permitted**. It constructs no `Query`, `Tags`, `Guard` or `AppendCondition`, is neither the append nor the read call, and is not an assertion — the `commit` call and the `assert_eq!` are both visible. AC-005 holds on both surfaces, and `docs/first-encounter.md` carries **zero** hidden lines, so the question does not arise on the surface this story authored. | decided here, and stated so a later reviewer does not read the silence as an oversight |
| O2 | `Tags::empty()` appears twice on the crate root (`:40`, `:57`) — the initiative's own headline evidence, and `_baseline.md § Merge` tracks it. AC-004's prose clause is *"nowhere does prose claim a real consistency boundary over code constructing an empty one"*, and no sentence adjacent to that fence makes a boundary claim: the lead-in is "one enum of events, one struct that folds them, and one call that reads, decides, appends and retries", which is what the program does. So the criterion holds — but the empty scope is still the thing this initiative was seeded by, and it sits on the page a `cargo add` reader meets first. | routed with BC-002, to the same sign-off owner and in the same amendment: whatever program the crate root carries is where this is answered |
| O3 | The `###` falsification-drill slot at the bottom of step 3 is left empty by this story and filled by `boundary-falsification-drill` in the same slice, under the heading `_resolutions.md § Anchor table` supplies (`Try it wrong, then put it back`). Nothing above it moves. | slice-mate's, by design |
| O4 | This story's acceptance criteria are encoded as a source-reading test at `xtask/tests/first_encounter.rs`, which is one path outside the five-entry PR-boundary fence in `spec.md`. It is the repository's own idiom for a composition check no compiler can make (`crates/happenstance/tests/doc_budget.rs`, `docs_composition.rs`, `doc_surface.rs` are the precedent), and `xtask` is the crate that owns the narrative tree and is `publish = false`. Recorded rather than hidden, per EC-010's spirit: a fence widened to quiet a tool stops being a statement about scope. | **Answered 2026-08-19, and the original disposition was wrong.** Recording is not one of the two responses EC-010 sanctions — *revert it, or route it as its own story* — so "recorded rather than hidden" left the violation standing under a description of itself. The fence is amended to admit `xtask/tests/**`, tests only, with the reason inline and on its own commit (`4232b34`), in the form `a41a1a5` used; `xtask/src/**` stays outside, so the widening admits no production path, gate step or `REQUIRED` entry. The reasoning below still stands as the *reason* for the amendment: the alternative — no mechanical check at all for AC-003, AC-005, AC-006 and AC-007 — is worse |

## Re-deriving this record

```console
$ git grep -c "^//!" -- crates/happenstance/src/lib.rs
$ cargo test -p happenstance --test doc_budget --test doc_surface --test docs_composition
$ cargo doc -p happenstance --no-deps
$ RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --all-features --no-deps \
      --document-private-items
$ cargo test -p xtask --doc -- first_encounter
$ cargo test -p xtask --test first_encounter
$ cargo run -p xtask -- lints && cargo run -p xtask -- spec-trace
$ cargo run -p xtask -- affected --base main
```

## BC-002 — RESOLVED 2026-08-19 by the `_design.md` sign-off owner

**Decision: amend the design; do not amend the merged tree.** The crate root does not carry the
refusal. `_design.md` § Composition region 4 is struck by an amendment note under
`crate-root-encounter`, its `## Hierarchy` primary-element bullet is struck by a second note, and
`spec.md` gains an `## Amendment — BC-002` section immediately above the acceptance table naming
exactly which clause of AC-002, AC-007 and AC-008 falls and what stands in its place. All original
text is left standing; nothing is edited away.

The two rejected routes, and why:

- **Raise the crate-root budgets** (allow a second fence, lift `MODULE_DOC_LINES`). Rejected: it
  overrides another project's signed-off assertion and the stated reason behind it — *"a second
  one would demote the first, which is the page's primary hierarchy signal"* — from inside a
  documentation story. Editing the check that says no is not the same as answering it.
- **Replace HS-P0016's landing program.** Rejected on this record's own argument: outside the
  stated seam, and it would meet a reader with an untyped `Event`/`Bytes` program on the crate
  whose identity is the typed layer.

The three inherited AC-007 numbers are **recorded as owed by HS-P0016**, not waived — they are
real overages on a real page, and this project is simply not the one that may fix them.

The cost is recorded rather than absorbed: the reader who lands on docs.rs meets a `commit`
program and reaches the refusal one hop later. That is a weaker outcome than the signed-off design
intended, and it is accepted as such.

## Acceptance reconciliation, 2026-08-19 — the ledger catches up to the amendment

BC-002 was resolved above at `faa8834`, which touched `_design.md`, `spec.md` and this file and
**not** `_ledger.md`. The ledger's last write was the implementation checkpoint `9493276`, so
until now the story was presented for review with AC-002, AC-007 and AC-008 all reading
`satisfied: false` while `report.md`, `implementation-report.md` and the amendment that
supersedes those rows described the slice as complete. A story whose ledger and whose governing
amendment disagree about what is done has no reviewable state; that is what this section closes.

| row | now | on what authority |
| --- | --- | --- |
| **AC-002** | `satisfied: true` | `spec.md` § Amendment — BC-002 strikes the heading, the fence and the printed refusal. Every clause standing in their place is met and cited on the row: the fence executes (`crates/happenstance/src/lib.rs:26-64`), no roadmap survives above it, the adapter-author redirect is last (`:145-147`), ADR-0006's reasoning survives verbatim (`:77-82`), the literal `[happenstance_core]` bracket count is zero under **both** doc invocations, the answered-need line is above everything (`:21`), and the pointer at `:66` reaches the refusal in one hop |
| **AC-008** | `satisfied: true` | The same amendment confines the criterion to `docs/first-encounter.md`: with no crate-root section there is no crate-root last sentence. ES-25 is carried in last position at `docs/first-encounter.md:130`, ES-8 and ES-26 at `:43` and `:84`, `spec-trace` is green, and neither surface contains `MUST` |
| **AC-007** | `satisfied: false`, deliberately | See the next section. The amendment strikes three numbers and records them *owed*; the item that owes them does not exist on this branch |

**The descope is a decision, not a ledger edit, and one half of recording it is still owed.**
This file's own preamble and `_ledger.md`'s say it: *"Scope changes are a human decision recorded
through `redkiln advance`, not a quiet ledger edit."* The decision itself is human — the
`_design.md` sign-off owner's, at `faa8834` — and it is recorded in three places that are not
commit prose: the amendment note in `_design.md` § Composition, `spec.md` § Amendment — BC-002,
and § BC-002 RESOLVED above. What is **not** yet done is the CLI half, because the `redkiln` CLI
is the single writer of an item's system fields and an implementer may not drive a stage
transition. It is handed back to the orchestrating command, and it is owed on this story:

```console
$ redkiln advance HS-S0185 --note "descope: spec.md Amendment - BC-002 strikes the crate-root \
    clause of AC-002, AC-007 and AC-008; AC-007 carried open"
```

## The three crate-root overages, and where they go

`spec.md` § Amendment — BC-002 records the three inherited AC-007 numbers as **owed by HS-P0016,
not waived**. That is true and it is not a route. `HS-P0016 publication-and-positioning` lives on
`initiative/from-contract-to-published-library`, an unmerged sibling branch; naming it in prose
routes nothing `redkiln` can see, and project DoD item 9 asks for a destination among four —
substrate to HS-P0020, pointer and reach to HS-P0023, comprehension to HS-P0024, incidental bugs
to the `support` initiative (`.redkiln/config.yaml:5`). Restated here with their measurement, so
that whatever item receives them starts from numbers rather than from a sentence:

| # | measured | budget | how to re-derive |
| --- | --- | --- | --- |
| F-1 | the crate-root fence is **35 rendered lines** | 32, the crate-root exemption in `_design.md` § Density budget | count the non-hidden lines of the fence in the module doc of `crates/happenstance/src/lib.rs:26-64` |
| F-2 | **70 columns** on two lines | 68; 72 is where `overflow-x` engages on the 696px fence at 1024×768 | widest rendered fence line in `target/doc/happenstance/index.html` |
| F-3 | two of four `##` headings exceed **22 characters**, at 39 and 26 | 22; the 200px sidebar TOC clips with an ellipsis and never wraps | `git grep -n "^//! # " -- crates/happenstance/src/lib.rs`, then count each title |

All three are properties of HS-P0016's landing program and its headings. They were measured by
`merge-forward-preflight/_baseline.md` *before* this story began and routed here on the assumption
that the rewrite would replace the program; the amendment is the decision that it will not.

**Destination.** The `support` initiative, `HS-I0005` (`.bklg/support/initiative.md`), which is
project DoD item 9's named home for an incidental defect on a surface this project may not touch.
Opening the item is a `redkiln new` call, which the single-writer rule reserves to the
orchestrating command or a human:

```console
$ redkiln new story --initiative support \
    --title "The crate-root fence and headings exceed the documentation density budget"
```

Its body should carry F-1, F-2 and F-3 verbatim, cite this section and `spec.md` § Amendment —
BC-002 as the origin, and state that the fix belongs with whoever owns the landing program in
`crates/happenstance/src/lib.rs` once `initiative/from-contract-to-published-library` merges.

**Until that item exists, `_ledger.md` AC-007 stays `satisfied: false`.** Flipping it against a
destination that carries no rows would reproduce, one directory over, exactly the defect this
section was written to close. The shortfall is therefore an **open** item for the project-level
review to carry to closeout, not a routed-and-closed one. `HS-S0189
fence-inventory-and-clause-audit` is the item that will meet these three numbers next by
construction — it is `blocked_by` this story, its AC-001 inventories `crate-root-encounter`, and
its AC-006 enumerates the same 68 / 24 / 32 / 22 budget — but it inventories and routes rather
than fixes, so it is not a substitute for the `support` item above.

## BC-004 — one half of AC-004's falsification cannot be had, and the seam it hid

**Found by the slice review, 2026-08-19**, and this section is the measurement rather than the
rebuttal. AC-004 asks for two falsifications of step 3's guard: *remove **either** the tag join
from the guard's `Query` **or** the `after_opt(upto)` from the `AppendCondition`, THEN the
scenario stops refusing — both are load-bearing*. The first is real and is demonstrated below.
**The second is unsatisfiable by construction, for every program anyone could write.**

**Why, from the type.** `AppendCondition::new(query)` already constructs its one `Guard` with
`after: None` (`crates/happenstance-core/src/append.rs`, `impl AppendCondition`), and `None` is
not "unguarded" — `Guard::after`'s own documentation says it: *"`None` checks the entire log."*
So `after` is a **relaxation** of a whole-log guard, and deleting `.after_opt(upto)` moves the
guard from "no matching event above the position I read" to "no matching event anywhere", which
is strictly *stronger*. It can turn an acceptance into a refusal. It can never turn a refusal
into an acceptance, which is what the criterion asks a reader to observe.

**Measured, on the corrected page, both directions:**

```console
$ # `.after_opt(upto)` deleted from docs/first-encounter.md step 3
$ cargo test -p xtask --doc -- first_encounter
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 168 filtered out

$ # `.with_tags(held)` dropped from the event instead
$ cargo test -p xtask --doc -- first_encounter
thread 'main' panicked at …doctest_bundle_2024.rs:84:15:
the boundary did not hold: Ok(SequencePosition(3))
test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 168 filtered out
```

**What this concealed, which is the part that matters.** Because the criterion's recipe cannot
be run, the implementation satisfied it with a *source substring* —
`xtask/tests/first_encounter.rs::the_racing_append_and_the_after_are_both_load_bearing` asserted
`program.contains("after_opt(upto)")` — over a step 3 that read an **empty** store. `upto` was
`None`, `AppendCondition::new` already carries `after: None`, and the call was therefore inert:
the reviewer deleted it and the step-3 doctest stayed green. A rule no wrong implementation can
fail is decorative (`CLAUDE.md`, *A rule that no adapter can fail is decorative*), and this one
named the property in its own function name while checking a substring. It is the same defect
shape `5af116b` found in step 2, one step further on.

**What replaced it — real behaviour, in three places.**

| # | what now stands where the inert call stood | where |
| --- | --- | --- |
| 1 | Step 3 lands one matching seat **before** the decision reads, so `upto` is `Some(SequencePosition(1))`: the guard carries a position this program observed, the racing writer lands strictly above it, and the reader sees the position in the program's own output (`guarded above Some(SequencePosition(1)): ConditionViolated`) | `docs/first-encounter.md:96-125` |
| 2 | `::step_three_guards_on_a_position_its_own_read_observed` — the seed precedes the read, the race sits between the read and the guarded append, and the output block shows the position. **Verified red against the program that shipped**: *"step three reads an empty store, so `upto` is None and `after_opt(upto)` is inert"* | `xtask/tests/first_encounter.rs` |
| 3 | `::the_after_is_load_bearing_in_its_value_not_in_its_presence` — an executed `#[tokio::test]` carrying both halves of this finding: the whole-log guard a reader is left with after deleting `after_opt` still **refuses**, and a guard built from a read taken *after* the race is **accepted**, which is the lost update | `crates/happenstance/tests/boundary_refusal.rs` |

Row 3 is the one that makes this record falsifiable rather than an argument: if some later change
made `after: None` weaker than an `after`, that test goes red and this section is wrong in a way
the gate reports.

**Acceptance consequence, and why it differs from AC-007's.** `_ledger.md` AC-004 stays
`satisfied: true`, and the row's evidence is corrected to say exactly this rather than to repeat
the sentence *"both halves of the guard are therefore load-bearing"*, which was false as written.
The reasoning, stated so a reviewer can reject it rather than guess at it:

- The criterion's **obligation** — no empty consistency boundary where prose claims a real one,
  the guard tagged to the invariant, the racing append carrying the same tags, and the `after`
  built from the position the read observed — is met on the page and pinned by tests that fail
  the wrong implementation in *both* directions.
- What is unsatisfiable is one **falsification recipe** for it, and the reason it is
  unsatisfiable is that the library is *safer* than the recipe assumed. There is no reader-facing
  shortfall to owe: nothing a reader meets is weaker than the criterion intended.
- AC-007 is the opposite case and is why the two are treated differently: three measured overages
  a reader actually meets on a page this project may not touch. That is a debt, it was carried
  `satisfied: false` until **HS-B0001** existed to own it, and it was flipped against that item's
  id and not against prose. Nothing here asks for the same treatment, because nothing here is
  owed to a reader.

**Routed to:** the sign-off owner of `spec.md`'s acceptance table, as an amendment to AC-004's
wording, on the same route BC-002 took (`spec.md` EC-006: *"record it as a design gap and route
it to the sign-off owner"*). The clause to amend is *"remove **either** the tag join … **or** the
`after_opt(upto)` … THEN the scenario stops refusing"*; the wording that is true of the code, and
that the tests above already check, is:

> remove the tag join from the guard's `Query`, **or** build the `after` from a read taken after
> the race rather than the one the decision was made on, and the scenario stops refusing. The
> `after` is load-bearing in its **value**: deleting it does not weaken the guard, it widens it
> to the whole log.

`spec.md:155` and the Behavior table at `:410` carry the same sentence and fall with it. Nothing
is edited here: the criterion stands unedited in `_ledger.md` and in `spec.md`, this section is
what supersedes the one clause, and `xtask/tests/first_encounter.rs` no longer carries a test
whose *name* claims the property that does not hold.

**Re-deriving this record:**

```console
$ cargo test -p xtask --test first_encounter -- step_three_guards_on_a_position_its_own_read_observed
$ cargo test -p happenstance --test boundary_refusal
$ cargo test -p xtask --doc -- first_encounter
```
